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

#[cfg(not(target_arch = "wasm32"))]
use rspice_fonts::system::print::SystemFaceId;
use rspice_fonts::{PrintFace, Role};

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

/// The faces print letters in, parsed for their metrics.
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
                PrintFace::BUNDLED
                    .into_iter()
                    .map(|face| ttf_parser::Face::parse(face.bytes(), 0).map_err(|_| face.name()))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|faces| Self { faces })
            })
            .as_ref()
            .map_err(|name| HardcopyRenderError::InvalidEmbeddedFont(name))
    }

    /// `face`, parsed. An installed face is parsed the first time it is
    /// asked for; one this parser cannot read is measured as RSpice Symbols.
    pub(crate) fn face(&self, face: PrintFace) -> &ttf_parser::Face<'static> {
        match face {
            PrintFace::Bundled(face) => &self.faces[face as usize],
            #[cfg(not(target_arch = "wasm32"))]
            PrintFace::System(id) => {
                use std::collections::HashMap;
                use std::sync::{Mutex, PoisonError};
                type Parsed = HashMap<SystemFaceId, Option<&'static ttf_parser::Face<'static>>>;
                static PARSED: Mutex<Option<Parsed>> = Mutex::new(None);
                let parsed = *PARSED
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .get_or_insert_with(HashMap::new)
                    .entry(id)
                    .or_insert_with(|| {
                        let face = id.face();
                        ttf_parser::Face::parse(face.bytes(), face.index())
                            .ok()
                            .map(|parsed| &*Box::leak(Box::new(parsed)))
                    });
                parsed.unwrap_or(&self.faces[Face::Symbols as usize])
            }
        }
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
}

/// The receipt's entries for `faces`, in the receipt's own order: each face's
/// family, weight and version, and the SHA-256 of the bytes the binary
/// embeds, worked out once per process. An installed face is recorded with
/// the file it was read from and the SHA-256 of that file as it was read.
pub(crate) fn receipt_faces(
    faces: &std::collections::BTreeSet<PrintFace>,
) -> Result<Vec<HardcopyFace>, HardcopyRenderError> {
    use rspice_hardcopy_contract::HardcopyFaceSource;

    static ENTRIES: OnceLock<Result<Vec<HardcopyFace>, &'static str>> = OnceLock::new();
    let entries = ENTRIES
        .get_or_init(|| {
            PrintFace::BUNDLED
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
                    let embedded = face.bundled().ok_or(face.name())?;
                    HardcopyFace::try_new(
                        embedded.family(),
                        embedded.weight(),
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
        .map(|&face| match face {
            PrintFace::Bundled(face) => Ok(entries[face as usize].clone()),
            #[cfg(not(target_arch = "wasm32"))]
            PrintFace::System(id) => system_receipt_face(id),
        })
        .collect::<Result<Vec<_>, _>>()?;
    recorded.sort();
    Ok(recorded)
}

/// The receipt's entry for the installed face `id`, worked out once per
/// process.
#[cfg(not(target_arch = "wasm32"))]
fn system_receipt_face(id: SystemFaceId) -> Result<HardcopyFace, HardcopyRenderError> {
    use std::collections::HashMap;
    use std::sync::{Mutex, PoisonError};

    use rspice_hardcopy_contract::HardcopyFaceSource;

    static ENTRIES: Mutex<Option<HashMap<SystemFaceId, Result<HardcopyFace, &'static str>>>> =
        Mutex::new(None);
    ENTRIES
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get_or_insert_with(HashMap::new)
        .entry(id)
        .or_insert_with(|| {
            let face = id.face();
            let parsed =
                ttf_parser::Face::parse(face.bytes(), face.index()).map_err(|_| face.family())?;
            let version = parsed
                .names()
                .into_iter()
                .filter(|name| name.name_id == ttf_parser::name_id::VERSION)
                .find_map(|name| name.to_string())
                .unwrap_or_default();
            HardcopyFace::try_new(
                face.family(),
                parsed.weight().to_number(),
                version.trim(),
                ContentDigest::from_bytes(Sha256::digest(face.bytes()).into()),
                HardcopyFaceSource::System {
                    path: receipt_path(face.path(), home_directory().as_deref()),
                },
            )
            .map_err(|_| face.family())
        })
        .clone()
        .map_err(HardcopyRenderError::InvalidEmbeddedFont)
}

/// The user's home directory, which a receipt never names.
#[cfg(not(target_arch = "wasm32"))]
fn home_directory() -> Option<std::path::PathBuf> {
    let variable = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    std::env::var_os(variable)
        .filter(|home| !home.is_empty())
        .map(std::path::PathBuf::from)
}

/// `path` as a receipt records it: a file under `home`, such as a face one
/// user installed, starts `~` in place of the home directory, so a receipt
/// shared with others does not carry the user's name; any other file, such
/// as a face of the operating system's, is recorded as it is.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn receipt_path(path: &std::path::Path, home: Option<&std::path::Path>) -> String {
    match home.and_then(|home| path.strip_prefix(home).ok()) {
        Some(within) => format!("~{}{}", std::path::MAIN_SEPARATOR, within.display()),
        None => path.display().to_string(),
    }
}
