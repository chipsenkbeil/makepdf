mod font;

pub use font::{RuntimeDocFont, RuntimeDocFonts};

use anyhow::Context;
use printpdf::{Mm, PdfDocument, PdfDocumentReference, PdfLayerReference, PdfPageReference};
use std::fs::File;
use std::io::BufWriter;

pub struct RuntimeDoc {
    doc: PdfDocumentReference,
}

impl RuntimeDoc {
    /// Creates a new, empty document named `title`.
    pub fn new(title: &str) -> Self {
        Self {
            doc: PdfDocument::empty(title),
        }
    }

    /// Adds a new, empty page named `title` of `width` x `height` to the document.
    ///
    /// This will be the next page in sequence!
    pub fn add_empty_page(
        &self,
        width: Mm,
        height: Mm,
        name: &str,
    ) -> (PdfPageReference, PdfLayerReference) {
        let (page_index, layer_index) = self.doc.add_page(width, height, name);
        let page = self.doc.get_page(page_index);
        let layer = page.get_layer(layer_index);
        (page, layer)
    }

    /// Loads a font into the document, returning a wrapper around the font.
    pub fn load_font(&self, path: Option<&str>) -> anyhow::Result<RuntimeDocFont> {
        let mut font = match path {
            Some(path) => RuntimeDocFont::load(path)?,
            None => RuntimeDocFont::system()?,
        };

        font.font = Some(
            self.doc
                .add_external_font(font.as_slice())
                .context("Failed to add external font")?,
        );

        Ok(font)
    }

    /// Saves the doc to the specified `filename`.
    pub fn save(self, filename: impl Into<String>) -> anyhow::Result<()> {
        let filename = filename.into();
        let f = File::create(&filename).with_context(|| format!("Failed to create {filename}"))?;
        self.doc
            .save(&mut BufWriter::new(f))
            .with_context(|| format!("Failed to save {filename}"))
    }
}
