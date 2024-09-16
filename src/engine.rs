mod doc;
mod hooks;
mod pages;
mod script;

pub use doc::{EngineDoc, EngineDocFont};
pub use hooks::EngineHooks;
use pages::*;
use script::Script;

use crate::pdf::{Pdf, PdfConfig, PdfContext, PdfLink};
use anyhow::Context;
use std::collections::HashMap;

/// PDF generation engine, using `T` as a state machine to progress through a series of steps
/// towards generating and saving a PDF.
pub struct Engine<T>(T);

impl Engine<()> {
    /// Creates a new engine for the provided `config`.
    pub fn new(config: PdfConfig) -> Engine<PdfConfig> {
        Engine(config)
    }
}

impl Engine<PdfConfig> {
    /// Runs the configured Lua script to setup the final configuration and register hooks to
    /// process pages of the PDF among other things.
    pub fn setup(self) -> anyhow::Result<Engine<(PdfConfig, Script, EngineHooks)>> {
        let config = self.0;

        // Initialize a script and relevant application data
        let mut script =
            Script::load_from_script(&config.script).context("Failed to load script")?;

        // Hooks need to be configured as available before running our script as the
        // script can access and register new hooks into the system
        script.set_app_data(EngineHooks::new());

        // Store a fresh copy of the PDF global into our Lua runtime to be accessible
        script
            .set_global("pdf", Pdf::new(config))
            .context("Failed to initialize PDF script global")?;

        // Do the actual execution of the script
        script.exec()?;

        // Retrieve the post-script PDF information
        let pdf: Pdf = script
            .get_global("pdf")
            .context("Failed to retrieve PDF information post-script execution")?;

        // Retrieve the hooks to process
        let hooks: EngineHooks = script
            .remove_app_data()
            .context("Missing hooks post-script execution")?;

        Ok(Engine((pdf.config, script, hooks)))
    }
}

impl Engine<(PdfConfig, Script, EngineHooks)> {
    /// Runs the hooks that configure the pages to populate the PDF document.
    pub fn run_hooks(self) -> anyhow::Result<Engine<(PdfConfig, EnginePages)>> {
        let (config, script, hooks) = self.0;

        // Create a set of pages configured for the planner. These are not
        // actually created within the doc yet, but are available for access
        // by hooks in advance of us constructing the document.
        let mut pages = EnginePages::for_planner(&config.planner)?;
        let keys = pages.keys().collect::<Vec<_>>();

        for key in keys {
            // Get access to the current page to process with hooks
            if let Some(page) = pages.get_page(key) {
                // Pages need to be configured as available before running our hooks as the
                // hooks can access and manipulate pages. The hooks will potentially modify
                // pages, so we will retrieve from our app data the pages once hooks are done.
                script.set_app_data(pages);

                match page.kind {
                    EnginePageKind::Daily => {
                        hooks.on_daily_page(page)?;
                    }
                    EnginePageKind::Monthly => {
                        hooks.on_monthly_page(page)?;
                    }
                    EnginePageKind::Weekly => {
                        hooks.on_weekly_page(page)?;
                    }
                }

                // Pull back out the pages from our global app data
                // so we can use it to retrieve the next page
                pages = script.remove_app_data().unwrap();
            }
        }

        Ok(Engine((config, pages)))
    }
}

impl Engine<(PdfConfig, EnginePages)> {
    /// Builds the document representing the PDF.
    pub fn build(self) -> anyhow::Result<Engine<EngineDoc>> {
        let (config, pages) = self.0;
        let (width, height) = (config.page.width, config.page.height);

        // Create our actual PDF document (empty)
        let doc = EngineDoc::new(&config.title);

        // Load up our default font to pass into the draw context
        let font = doc.load_font(config.page.font.as_deref())?;

        // Create the month, week, and daily page instances (in order) based on our internal pages
        let mut refs = HashMap::new();
        for page in pages.iter_monthly_pages() {
            // e.g. January 2024
            let name = page.date.format("%B %Y").to_string();
            refs.insert(page.id, doc.add_empty_page(width, height, &name));
        }
        for page in pages.iter_weekly_pages() {
            // e.g. Week 1 2024
            let name = page.date.format("Week %V %Y").to_string();
            refs.insert(page.id, doc.add_empty_page(width, height, &name));
        }
        for page in pages.iter_daily_pages() {
            // e.g. 2024-09-15 (Sunday)
            let name = page.date.format("%v (%A)").to_string();
            refs.insert(page.id, doc.add_empty_page(width, height, &name));
        }

        // Draw all pages, which can be done in any order, by looking up the PDF references
        // based on the page's id
        for page in pages {
            if let Some((_, layer)) = refs.get(&page.id) {
                let ctx = PdfContext {
                    config: &config,
                    font: &font,
                    layer,
                };

                page.draw(ctx);

                // Get annotations, sorted by depth, that we will add to our layer
                let mut annotations = page.link_annotations(ctx);
                annotations.sort_unstable_by(|a, b| a.depth.cmp(&b.depth));

                for annotation in annotations {
                    use printpdf::{Actions, Destination, LinkAnnotation};

                    // Map our link to an action, which can be none if it's an invalid action
                    // such as linking to a page that does not exist
                    let action = match annotation.link {
                        PdfLink::GoTo { page } => refs.get(&page).map(|x| x.0.page).map(|page| {
                            Actions::go_to(Destination::XYZ {
                                page,
                                left: None,
                                top: None,
                                zoom: None,
                            })
                        }),
                        PdfLink::Uri { uri } => Some(Actions::uri(uri)),
                    };

                    // If we have an action, add an annotation for it
                    if let Some(action) = action {
                        layer.add_link_annotation(LinkAnnotation::new(
                            annotation.bounds.into(),
                            None,
                            None,
                            action,
                            None,
                        ));
                    }
                }
            }
        }

        Ok(Engine(doc))
    }
}

impl Engine<EngineDoc> {
    /// Saves the PDF to the specified `filename`.
    pub fn save(self, filename: impl Into<String>) -> anyhow::Result<()> {
        self.0.save(filename)
    }
}
