mod font;
mod hooks;
mod pages;
mod planner;
mod script;

use anyhow::Context;
use font::Font;
pub use hooks::EngineHooks;
use pages::*;
use planner::EnginePlanner;
use script::Script;
use std::{fs::File, io::BufWriter};

use crate::pdf::{Pdf, PdfConfig};

/// PDF generation engine.
pub struct Engine {
    planner: EnginePlanner,
}

impl Engine {
    /// Initialize and build the PDF using a Lua script engine, returning a reference to
    /// the engine that can be used to save the PDF externally.
    pub fn build(config: PdfConfig) -> anyhow::Result<Self> {
        // Initialize a script and relevant application data
        let mut script =
            Script::load_from_script(&config.script).context("Failed to load script")?;
        script.set_app_data(EngineHooks::new());

        // Execute a script to populate the information we need to generate a PDF
        script
            .set_global("pdf", Pdf::new(config))
            .context("Failed to initialize PDF script global")?;
        script.exec()?;

        // Retrieve the post-script PDF information
        let pdf: Pdf = script
            .get_global("pdf")
            .context("Failed to retrieve PDF information post-script")?;

        // Retrieve the hooks to use with the planner
        let hooks: EngineHooks = script
            .remove_app_data()
            .context("Missing hooks post-script")?;

        // Initialize the planner
        let planner = EnginePlanner::new(pdf.config)?;

        // Run the hooks
        planner.run_hooks(hooks)?;

        Ok(Self { planner })
    }

    /// Saves the planner to the specified `filename`.
    pub fn save(self, filename: impl Into<String>) -> anyhow::Result<()> {
        let filename = filename.into();
        let f = File::create(&filename).with_context(|| format!("Failed to create {filename}"))?;
        self.planner
            .into_doc()
            .save(&mut BufWriter::new(f))
            .with_context(|| format!("Failed to save {filename}"))
    }
}
