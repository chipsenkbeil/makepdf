mod doc;
mod fonts;
mod pages;
mod script;

pub use doc::RuntimeDoc;
pub use fonts::{RuntimeFontId, RuntimeFonts};
pub(crate) use pages::*;
use script::RuntimeScript;

use crate::constants::GLOBAL_PDF_VAR_NAME;
use crate::pdf::{Pdf, PdfConfig, PdfContext, PdfLink};
use anyhow::Context;
use log::*;
use std::collections::HashMap;

/// PDF generation runtime, using `T` as a state machine to progress through a series of steps
/// towards generating and saving a PDF.
pub struct Runtime<T>(T);

impl Runtime<()> {
    /// Creates a new runtime for the provided `config`.
    pub fn new(config: PdfConfig) -> Runtime<PdfConfig> {
        Runtime(config)
    }
}

impl Runtime<PdfConfig> {
    /// Runs the configured Lua script to setup the final configuration and register hooks to
    /// process pages of the PDF among other things.
    pub fn setup(self) -> anyhow::Result<Runtime<(PdfConfig, RuntimePages, RuntimeFonts)>> {
        let config = self.0;

        // Initialize a script and relevant application data
        //
        // 1. Fonts need to be configured as available before running our script as the script can
        //    access and load new fonts into the system
        // 2. Pages need to be configured as available before running our script as the script can
        //    access and add new pages into the system
        info!("Loading {}", config.script);
        let mut script =
            RuntimeScript::load_from_script(&config.script).context("Failed to load script")?;
        script.set_app_data(RuntimePages::new());

        // Initialize our fonts with the pre-configured font used as the fallback for now
        info!("Initializing fonts");
        script.set_app_data({
            let mut fonts = RuntimeFonts::new();

            // At the beginning, load the configured font as the fallback PRIOR to running our
            // scripts, knowing that this may change when we are done running scripts and we
            // will reload and reset the fallback then
            let fallback_font_id = match config.page.font.as_deref() {
                Some(path_str) => fonts
                    .add_from_path(path_str)
                    .with_context(|| format!("Failed to load default font from {path_str}"))?,
                None => fonts
                    .add_builtin_font()
                    .context("Failed to load builtin font")?,
            };

            // Mark the fallback font
            fonts.add_font_as_fallback(fallback_font_id);

            fonts
        });

        // Store a fresh copy of the PDF global into our Lua runtime to be accessible
        script
            .set_global(GLOBAL_PDF_VAR_NAME, Pdf::new(config))
            .context("Failed to initialize PDF script global")?;

        // Do the actual execution of the script
        info!("Executing script");
        script.exec()?;

        // Retrieve the post-script PDF information
        let pdf: Pdf = script
            .get_global(GLOBAL_PDF_VAR_NAME)
            .context("Failed to retrieve PDF information post-script execution")?;

        // Retrieve the pages to process
        let pages: RuntimePages = script
            .remove_app_data()
            .context("Missing pages post-script execution")?;

        // Retrieve the fonts to process
        let fonts: RuntimeFonts = script
            .remove_app_data()
            .context("Missing fonts post-script execution")?;

        Ok(Runtime((pdf.config, pages, fonts)))
    }
}

impl Runtime<(PdfConfig, RuntimePages, RuntimeFonts)> {
    /// Builds the document representing the PDF.
    pub fn build(self) -> anyhow::Result<Runtime<RuntimeDoc>> {
        let (config, pages, mut fonts) = self.0;
        let (width, height) = (config.page.width, config.page.height);

        // Create our actual PDF document (empty)
        debug!("Initializing PDF document");
        let doc = RuntimeDoc::new(&config.title);

        // Load up our default font to pass into the draw context. We have already done this once,
        // but it may have changed since we ran our script; so, attempt to reload everything.
        // Because of caching, this should not be an issue if we have already loaded the external
        // or builtin font before.
        let fallback_font_id = match config.page.font.as_deref() {
            Some(path_str) => fonts
                .add_from_path(path_str)
                .with_context(|| format!("Failed to load default font from {path_str}"))?,
            None => fonts
                .add_builtin_font()
                .context("Failed to load builtin font")?,
        };

        // Mark the fallback font, which may be the same as before, to ensure that it is used
        // everywhere like we expect when adding the objects on the PDF
        debug!("Adding fallback font: {fallback_font_id}");
        fonts.add_font_as_fallback(fallback_font_id);

        // Attempt to add all the fonts to our document
        for id in fonts.to_ids() {
            debug!("Adding external font: {id}");
            if !fonts.add_font_to_doc(id, doc.as_ref())? {
                anyhow::bail!("Failed to add font {id} to PDF document");
            }
        }

        // Create pages in order that they were added to ensure that they show up in the right
        // order within the PDF itself
        let mut refs = HashMap::new();
        for id in pages.ids() {
            if let Some(page) = pages.get_page(id) {
                refs.insert(
                    page.id,
                    doc.add_empty_page(
                        page.width.unwrap_or(width),
                        page.height.unwrap_or(height),
                        &page.title,
                    ),
                );
            }
        }

        // Draw all pages, which can be done in any order, by looking up the PDF references
        // based on the page's id
        let page_cnt = pages.len();
        info!("Building {} PDF pages", page_cnt);
        for (i, page) in pages.into_iter().enumerate() {
            debug!("Building page {} ({} / {})", page.id, i, page_cnt);
            match refs.get(&page.id) {
                None => warn!("Missing refs for page {}", page.id),
                Some((_, layer)) => {
                    let ctx = PdfContext {
                        config: &config,
                        layer,
                        fonts: &fonts,
                        fallback_font_id,
                    };

                    trace!("Drawing page {}", page.id);
                    page.draw(ctx);

                    // Get annotations, sorted by depth, that we will add to our layer
                    let mut annotations = page.link_annotations(ctx);
                    annotations.sort_unstable_by(|a, b| a.depth.cmp(&b.depth));

                    trace!(
                        "Processing {} annotations for page {}",
                        annotations.len(),
                        page.id
                    );
                    for annotation in annotations {
                        use printpdf::{Actions, Destination, LinkAnnotation};

                        // Map our link to an action, which can be none if it's an invalid action
                        // such as linking to a page that does not exist
                        let action = match annotation.link {
                            PdfLink::GoTo { page } => {
                                refs.get(&page).map(|x| x.0.page).map(|page| {
                                    Actions::go_to(Destination::XYZ {
                                        page,
                                        left: None,
                                        top: None,
                                        zoom: None,
                                    })
                                })
                            }
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
        }

        Ok(Runtime(doc))
    }
}

impl Runtime<RuntimeDoc> {
    /// Saves the PDF to the specified `filename`.
    pub fn save(self, filename: impl Into<String>) -> anyhow::Result<()> {
        let filename = filename.into();

        info!("Saving PDF to {}", &filename);
        self.0.save(filename)
    }
}
