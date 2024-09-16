use crate::constants::DEFAULT_FONT;
use anyhow::Context;
use owned_ttf_parser::{AsFaceRef, Face, OwnedFace};
use printpdf::{IndirectFontRef, PdfDocumentReference};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Unique id associated with a loaded font that can be used to
/// retrieve a font face or a document's indirect font reference.
pub type RuntimeFontId = u32;

// TODO:
//
// 1. Fonts holds three things: faces, font indirect references, and default id
// 2. Generate random id for each face loaded
// 3. Support doc ref pass to function to get indirect font
// 4. Ctx holds fonts ref
// 5. Font field is id not str
// 6. Fonts canonicalizes path and stores in map with font id to cache
// 7. Store in app data to support `pdf.font.load()` returning id and adding to list
//    and retrieving for text bounds

/// Contains fonts used by the runtime.
#[derive(Debug, Default)]
pub struct RuntimeFonts {
    paths: HashMap<PathBuf, RuntimeFontId>,
    faces: HashMap<RuntimeFontId, OwnedFace>,
    refs: HashMap<RuntimeFontId, IndirectFontRef>,
    builtin_font_id: Option<RuntimeFontId>,
    fallback_font_id: Option<RuntimeFontId>,
}

impl RuntimeFonts {
    /// Creates a new, empty collection of fonts.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the font id for the specified `path` is one has been loaded from that path.
    pub fn font_for_path(&self, path: impl AsRef<Path>) -> anyhow::Result<Option<RuntimeFontId>> {
        // Canonicalize the font's path so we have a consistent path to use
        // when looking up a font by path
        let path = path
            .as_ref()
            .canonicalize()
            .context("Failed to canonicalize font path")?;
        Ok(self.paths.get(&path).copied())
    }

    /// Return the path for the specified font `id` if it was loaded externally.
    pub fn path_for_font(&self, id: RuntimeFontId) -> Option<PathBuf> {
        self.paths
            .iter()
            .find_map(|(font_path, font_id)| {
                if *font_id == id {
                    Some(font_path)
                } else {
                    None
                }
            })
            .cloned()
    }

    /// Loads the font face from `path` into memory, returning an id to access the font
    /// information.
    ///
    /// This will cache the `path` provided (after canonicalizing it) such that subsequent calls to
    /// add a font that resolve to the same path will instead return the same font id.
    pub fn add_from_path(&mut self, path: impl AsRef<Path>) -> anyhow::Result<RuntimeFontId> {
        // Canonicalize the font's path so we have a consistent path to use
        // when looking up a font by path
        let path = path
            .as_ref()
            .canonicalize()
            .context("Failed to canonicalize font path")?;

        // Check if we have already loaded the font at the specified path, and if so
        // return its id without reloading it
        if let Some(id) = self.paths.get(&path).copied() {
            return Ok(id);
        }

        // Otherwise, this is considered a new font and we will read it into memory and add the
        // bytes as a new owned font face
        let bytes =
            std::fs::read(path.as_path()).with_context(|| "Failed to read font file: {path}")?;
        let id = self.add_from_bytes(bytes)?;

        // Cache the path so we don't reload the same font in the future
        self.paths.insert(path, id);

        Ok(id)
    }

    /// Loads the font face from `bytes` into memory, returning an id to access the font
    /// information.
    ///
    /// NOTE: This does not prevent adding the same font more than once! Caching only happens when
    ///       loading a font from a path where the path is cached; so, avoid invoking this directly
    ///       when loading fonts from disk.
    pub fn add_from_bytes(&mut self, bytes: Vec<u8>) -> anyhow::Result<RuntimeFontId> {
        let face = OwnedFace::from_vec(bytes, 0).context("Failed to build font into face")?;
        let id = Self::random_font_id();
        self.faces.insert(id, face);
        Ok(id)
    }

    /// Adds the builtin font to the collection.
    ///
    /// This will cache the font such that subsequent calls to add the builtin font will instead
    /// return the same font id.
    #[inline]
    pub fn add_builtin_font(&mut self) -> anyhow::Result<RuntimeFontId> {
        // If we have already loaded the builtin font, do nothing
        if let Some(id) = self.builtin_font_id {
            return Ok(id);
        }

        // Otherwise, load the builtin font from its bytes
        let id = self.add_from_bytes(DEFAULT_FONT.to_vec())?;
        self.builtin_font_id = Some(id);
        Ok(id)
    }

    /// Adds the font specified by `id` as the fallback font associated with the set.
    ///
    /// Returns an option of a font id in case there was an existing fallback font.
    #[inline]
    pub fn add_font_as_fallback(&mut self, id: RuntimeFontId) -> Option<RuntimeFontId> {
        self.fallback_font_id.replace(id)
    }

    /// Returns the id of the fallback font, if one has been configured.
    #[inline]
    pub fn fallback_font_id(&self) -> Option<RuntimeFontId> {
        self.fallback_font_id
    }

    /// Returns a distinct collection of font ids.
    ///
    /// These may or may not have been added to the PDF document.
    pub fn to_ids(&self) -> Vec<RuntimeFontId> {
        let mut ids: Vec<_> = self
            .faces
            .keys()
            .chain(self.refs.keys())
            .chain(self.fallback_font_id.iter())
            .chain(self.builtin_font_id.iter())
            .copied()
            .collect();

        // Remove any duplicate ids
        ids.sort_unstable();
        ids.dedup();

        ids
    }

    /// Adds the font specified by `id` to the provided `doc`.
    ///
    /// Returns true if the font exists and was added to the doc, or false if the font does not
    /// exist. Any other error will be captured and returned as an error.
    ///
    /// NOTE: Because the font is cached, this means that you cannot add the font to more than one
    ///       PDF document. A font collection should really only be used with a singular PDF
    ///       document, so it is considered out of scope to handle multiple documents with the same
    ///       collection.
    pub fn add_font_to_doc(
        &mut self,
        id: RuntimeFontId,
        doc: &PdfDocumentReference,
    ) -> anyhow::Result<bool> {
        // Check if we have already added the font to the document, and if so do nothing
        if self.refs.contains_key(&id) {
            return Ok(true);
        }

        match self.get_font_slice(id) {
            Some(slice) => {
                self.refs.insert(
                    id,
                    doc.add_external_font(slice)
                        .context("Failed to add external font to PDF document")?,
                );

                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Returns a reference to the face of the font with the specified `id`.
    pub fn get_font_face(&self, id: RuntimeFontId) -> Option<&Face> {
        self.faces.get(&id).map(|face| face.as_face_ref())
    }

    /// Returns a slice to the data of the font with the specified `id`.
    pub fn get_font_slice(&self, id: RuntimeFontId) -> Option<&[u8]> {
        self.faces.get(&id).map(|face| face.as_slice())
    }

    /// Returns a reference to the document's font for the font with the specified `id`.
    pub fn get_font_doc_ref(&self, id: RuntimeFontId) -> Option<&IndirectFontRef> {
        self.refs.get(&id)
    }

    #[inline]
    fn random_font_id() -> RuntimeFontId {
        rand::random()
    }
}
