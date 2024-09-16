use crate::constants::DEFAULT_FONT;
use anyhow::Context;
use owned_ttf_parser::{AsFaceRef, Face, OwnedFace};
use printpdf::IndirectFontRef;

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
#[derive(Debug)]
pub struct RuntimeDocFonts {
    faces: Vec<OwnedFace>,
}

impl RuntimeDocFonts {
    pub fn new() -> Self {
        Self { faces: Vec::new() }
    }

    /// Loads the font face from `path` into memory.
    pub fn add_from_path(&mut self, path: impl AsRef<str>) -> anyhow::Result<()> {
        let bytes = std::fs::read(path.as_ref()).context("Failed to read font")?;
        self.add_from_bytes(bytes)
    }

    /// Loads the font face from `bytes` into memory.
    fn add_from_bytes(&mut self, bytes: Vec<u8>) -> anyhow::Result<()> {
        let face = OwnedFace::from_vec(bytes, 0).context("Failed to build font into face")?;
        self.faces.push(face);
        Ok(())
    }
}

/// Represents the font used by the runtime.
#[derive(Debug)]
pub struct RuntimeDocFont {
    pub face: OwnedFace,
    pub font: IndirectFontRef,
}

impl RuntimeDocFont {
    /// Loads the font from `path` into memory.
    pub fn load(path: impl AsRef<str>) -> anyhow::Result<Self> {
        let bytes = std::fs::read(path.as_ref()).context("Failed to read font")?;
        Self::from_bytes(bytes)
    }

    /// Loads the system font into memory.
    pub fn system() -> anyhow::Result<Self> {
        Self::from_bytes(DEFAULT_FONT.to_vec())
    }

    fn from_bytes(bytes: Vec<u8>) -> anyhow::Result<Self> {
        let face = OwnedFace::from_vec(bytes, 0).context("Failed to build font into face")?;
        Ok(Self { face, font: None })
    }

    /// Returns a reference to the font face.
    pub fn as_face(&self) -> &Face {
        self.face.as_face_ref()
    }

    /// Returns a slice of data passed to the font face.
    pub fn as_slice(&self) -> &[u8] {
        self.face.as_slice()
    }

    /// Returns a reference to the underlying font.
    ///
    /// This is only available once added to the PDF document.
    pub fn as_font_ref(&self) -> Option<&IndirectFontRef> {
        self.font.as_ref()
    }
}
