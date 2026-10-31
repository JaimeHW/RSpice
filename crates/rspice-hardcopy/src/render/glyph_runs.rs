//! Per-glyph face fallback, shared by every hardcopy backend.
//!
//! A scene font letters through its role's chain in `rspice-fonts`, the one
//! table the screen's egui families are built from too. For each character,
//! the first face in the chain that carries it draws it, so a character the
//! canvas borrows from another face prints from that face. A character no
//! face carries prints as [`rspice_fonts::REPLACEMENT`], the white medium
//! square the canvas paints in its place: uncovered text never refuses a
//! print.

use std::sync::OnceLock;

use rspice_fonts::{PrintFace, Role, Run};

use super::*;

impl SceneFont {
    /// The role the font letters in, whose chain a character is looked for
    /// in. The screen letters the same role through the same chain.
    #[must_use]
    pub const fn role(self) -> Role {
        match self {
            Self::Sans => Role::Sans,
            Self::SansSemibold => Role::SansSemibold,
            Self::Monospace => Role::Mono,
            Self::MonospaceMedium => Role::MonoMedium,
        }
    }
}

/// The embedded faces, parsed for their metrics.
pub(crate) struct PrintFaces {
    faces: Vec<ttf_parser::Face<'static>>,
}

impl PrintFaces {
    /// The embedded faces, parsed once per process. They are constants of
    /// the binary, so a face that does not parse is a build defect, reported
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
        &self.faces[face as usize]
    }

    /// Advance of one run, in units of `font`'s first face's em.
    pub(crate) fn run_advance_units(&self, font: SceneFont, run: &Run) -> u64 {
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
        let primary = i64::from(self.face(font.role().print_primary()).units_per_em());
        let mut advance = 0_i64;
        let mut left = 0_i64;
        let mut right = 0_i64;
        for run in font.role().runs(grapheme) {
            let face = self.face(run.face);
            let scale = |value: i64| value * primary / i64::from(face.units_per_em());
            for character in run.text.chars() {
                let Some(glyph) = face.glyph_index(character) else {
                    continue;
                };
                if let Some(bounds) = face.glyph_bounding_box(glyph) {
                    left = left.min(advance + scale(i64::from(bounds.x_min)));
                    right = right.max(advance + scale(i64::from(bounds.x_max)));
                }
                advance += scale(i64::from(face.glyph_hor_advance(glyph).unwrap_or(0)));
                right = right.max(advance);
            }
        }
        (right - left) as u64
    }

    /// The em of `font`'s first face, in its own units.
    pub(crate) fn units_per_em(&self, font: SceneFont) -> u64 {
        u64::from(self.face(font.role().print_primary()).units_per_em())
    }

    fn to_primary_units(&self, font: SceneFont, face: PrintFace, value: u64) -> u64 {
        let primary = self.units_per_em(font);
        value * primary / u64::from(self.face(face).units_per_em())
    }
}

/// The receipt's entries for `faces`, in the receipt's own order: each face's
/// family, weight and version, and the SHA-256 of the bytes the binary
/// embeds, worked out once per process.
pub(crate) fn receipt_faces(
    faces: &std::collections::BTreeSet<PrintFace>,
) -> Result<Vec<HardcopyFace>, HardcopyRenderError> {
    use rspice_hardcopy_contract::HardcopyFaceSource;

    static ENTRIES: OnceLock<Result<Vec<HardcopyFace>, &'static str>> = OnceLock::new();
    let entries = ENTRIES
        .get_or_init(|| {
            PrintFace::ALL
                .into_iter()
                .map(|face| {
                    let parsed =
                        ttf_parser::Face::parse(face.bytes(), 0).map_err(|_| face.name())?;
                    let version = parsed
                        .names()
                        .into_iter()
                        .filter(|name| name.name_id == ttf_parser::name_id::VERSION)
                        .find_map(|name| name.to_string())
                        .ok_or(face.name())?;
                    HardcopyFace::try_new(
                        face.face().family(),
                        face.face().weight(),
                        version.trim(),
                        ContentDigest::from_bytes(Sha256::digest(face.bytes()).into()),
                        HardcopyFaceSource::Bundled,
                    )
                    .map_err(|_| face.name())
                })
                .collect()
        })
        .as_ref()
        .map_err(|name| HardcopyRenderError::InvalidEmbeddedFont(name))?;
    let mut recorded = faces
        .iter()
        .map(|&face| entries[face as usize].clone())
        .collect::<Vec<_>>();
    recorded.sort();
    Ok(recorded)
}
