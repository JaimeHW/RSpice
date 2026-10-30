//! Per-glyph face fallback, shared by every hardcopy backend.
//!
//! A scene font letters through a chain of the bundled faces. For each
//! character, the first face in the chain that carries it draws it. The
//! chains follow the ones the screen's egui families fall back through
//! (`rspice_ui_kit::fonts::font_definitions`), so a character the canvas
//! borrows from Plex Sans prints from Plex Sans too. A code point no face in
//! the chain carries prints as [`REPLACEMENT_CHARACTER`] from the chain's
//! first face, which is the replacement the canvas paints in its place:
//! uncovered text never refuses a print.
//!
//! [`SceneFont::fallback_chain`] is the one table of chains.

use std::sync::OnceLock;

use super::*;

/// What prints in place of a code point no face in the chain carries. egui
/// paints `◻` when a face in the family has it, and `?` otherwise. No
/// bundled Plex face carries `◻`, so the canvas paints `?`.
pub const REPLACEMENT_CHARACTER: char = '?';

/// One bundled face a hardcopy can letter in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrintFace {
    SansRegular,
    SansSemibold,
    MonoRegular,
    MonoMedium,
}

impl PrintFace {
    pub const ALL: [Self; 4] = [
        Self::SansRegular,
        Self::SansSemibold,
        Self::MonoRegular,
        Self::MonoMedium,
    ];

    /// The face's TrueType bytes, as embedded in the binary.
    #[must_use]
    pub const fn bytes(self) -> &'static [u8] {
        match self {
            Self::SansRegular => IBM_PLEX_SANS_REGULAR,
            Self::SansSemibold => IBM_PLEX_SANS_SEMIBOLD,
            Self::MonoRegular => IBM_PLEX_MONO_REGULAR,
            Self::MonoMedium => IBM_PLEX_MONO_MEDIUM,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::SansRegular => "IBM Plex Sans Regular",
            Self::SansSemibold => "IBM Plex Sans Semibold",
            Self::MonoRegular => "IBM Plex Mono Regular",
            Self::MonoMedium => "IBM Plex Mono Medium",
        }
    }

    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

impl SceneFont {
    /// The faces this font letters through, in the order a character is
    /// looked for. A weighted run borrows from its own weight first, so a
    /// borrowed glyph keeps its stroke where a bundled face has it. The
    /// canvas's medium mono borrows Plex Sans Medium, which hardcopy does not
    /// bundle; the print borrows the same glyphs from Plex Sans Regular,
    /// one weight lighter.
    #[must_use]
    pub const fn fallback_chain(self) -> &'static [PrintFace] {
        match self {
            Self::Sans => &[PrintFace::SansRegular],
            Self::SansSemibold => &[PrintFace::SansSemibold, PrintFace::SansRegular],
            Self::Monospace => &[PrintFace::MonoRegular, PrintFace::SansRegular],
            Self::MonospaceMedium => &[
                PrintFace::MonoMedium,
                PrintFace::MonoRegular,
                PrintFace::SansRegular,
            ],
        }
    }
}

/// A stretch of text lettered in one face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlyphRun {
    pub face: PrintFace,
    pub text: String,
}

/// The parsed bundled faces.
pub(crate) struct PrintFaces {
    faces: Vec<ttf_parser::Face<'static>>,
}

impl PrintFaces {
    /// The bundled faces, parsed once per process. They are constants of the
    /// binary, so a face that does not parse is a build defect, reported
    /// rather than panicked on.
    pub(crate) fn get() -> Result<&'static Self, HardcopyRenderError> {
        static FACES: OnceLock<Result<PrintFaces, &'static str>> = OnceLock::new();
        FACES
            .get_or_init(|| {
                PrintFace::ALL
                    .into_iter()
                    .map(|face| ttf_parser::Face::parse(face.bytes(), 0).map_err(|_| face.name()))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|faces| Self { faces })
            })
            .as_ref()
            .map_err(|name| HardcopyRenderError::InvalidEmbeddedFont(name))
    }

    pub(crate) fn face(&self, face: PrintFace) -> &ttf_parser::Face<'static> {
        &self.faces[face.index()]
    }

    /// The face that letters `character` in `font`, and the character it
    /// letters: the character itself from the first face carrying it, or
    /// the replacement from the chain's first face.
    pub(crate) fn resolve(&self, font: SceneFont, character: char) -> (PrintFace, char) {
        let chain = font.fallback_chain();
        chain
            .iter()
            .find(|face| self.face(**face).glyph_index(character).is_some())
            .map_or((chain[0], REPLACEMENT_CHARACTER), |face| (*face, character))
    }

    /// `text` as `font` letters it, split where the face changes.
    pub(crate) fn runs(&self, font: SceneFont, text: &str) -> Vec<GlyphRun> {
        let mut runs: Vec<GlyphRun> = Vec::new();
        for character in text.chars() {
            let (face, lettered) = self.resolve(font, character);
            match runs.last_mut() {
                Some(run) if run.face == face => run.text.push(lettered),
                _ => runs.push(GlyphRun {
                    face,
                    text: lettered.to_string(),
                }),
            }
        }
        runs
    }

    /// Advance of one run, in units of `font`'s first face's em.
    pub(crate) fn run_advance_units(&self, font: SceneFont, run: &GlyphRun) -> u64 {
        let face = self.face(run.face);
        let advance = run
            .text
            .chars()
            .filter_map(|character| face.glyph_index(character))
            .filter_map(|glyph| face.glyph_hor_advance(glyph))
            .map(u64::from)
            .sum();
        self.to_primary_units(font, run.face, advance)
    }

    /// Inked width of one grapheme lettered in `font`, from the leftmost ink
    /// or the origin to the rightmost ink or the advance, in units of `font`'s
    /// first face's em. Overhangs count as well as advances, so a grapheme
    /// kept whole at a wrap never inks past its measured width.
    pub(crate) fn grapheme_width(&self, font: SceneFont, grapheme: &str) -> u64 {
        let primary = i64::from(self.face(font.fallback_chain()[0]).units_per_em());
        let mut advance = 0_i64;
        let mut left = 0_i64;
        let mut right = 0_i64;
        for character in grapheme.chars() {
            let (lettered_in, lettered) = self.resolve(font, character);
            let face = self.face(lettered_in);
            let scale = |value: i64| value * primary / i64::from(face.units_per_em());
            let Some(glyph) = face.glyph_index(lettered) else {
                continue;
            };
            if let Some(bounds) = face.glyph_bounding_box(glyph) {
                left = left.min(advance + scale(i64::from(bounds.x_min)));
                right = right.max(advance + scale(i64::from(bounds.x_max)));
            }
            advance += scale(i64::from(face.glyph_hor_advance(glyph).unwrap_or(0)));
            right = right.max(advance);
        }
        (right - left) as u64
    }

    /// The em of `font`'s first face, in its own units.
    pub(crate) fn units_per_em(&self, font: SceneFont) -> u64 {
        u64::from(self.face(font.fallback_chain()[0]).units_per_em())
    }

    fn to_primary_units(&self, font: SceneFont, face: PrintFace, value: u64) -> u64 {
        let primary = self.units_per_em(font);
        value * primary / u64::from(self.face(face).units_per_em())
    }
}
