//! The faces RSpice embeds and the one table of fallback chains.
//!
//! Every crate that letters text takes its faces from here: the egui screen,
//! the PDF and SVG hardcopy, the shortcut reference, the published page and
//! its viewer. Each face is therefore embedded once, and screen and print fall
//! back through the same chain.
//!
//! A [`Role`] is what a run of text asks for: sans or mono, at a weight. Its
//! [`Role::chain`] lists the faces a character is looked for in, in order.
//! Every chain holds RSpice Symbols, which carries the engineering symbols
//! no Plex face has (⌀ ∅ ⏚, the GD&T characteristics, the interface marks)
//! and [`REPLACEMENT`], the square lettered for a character no face covers.
//! A desktop build goes on to IBM Plex Sans Arabic, Hebrew and Thai; a
//! browser does not embed them.
//!
//! Screen and print letter from every face in a chain. Print names its faces
//! with [`PrintFace`], and [`print`] sets its text: shaped, and in the order
//! it is read.
//!
//! The faces are distributed under the SIL Open Font License 1.1; each
//! face's [`Face::licence`] is the text that must travel with it.

#![deny(clippy::print_stdout, clippy::print_stderr)]

use std::sync::OnceLock;

use skrifa::{FontRef, MetadataProvider, charmap::Charmap};
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "bidi")]
pub mod bidi;
#[cfg(feature = "egui")]
mod egui_families;
mod han;
#[cfg(feature = "bidi")]
pub mod print;
#[cfg(all(feature = "system", not(target_arch = "wasm32")))]
pub mod system;
#[cfg(feature = "egui")]
pub use egui_families::egui_font_definitions;
pub use han::{HanPreference, HanVariant, han_order};

/// What is lettered in place of a character no face in a chain covers:
/// U+25FB WHITE MEDIUM SQUARE, from RSpice Symbols. egui paints the same
/// square for a character a family lacks, because every family carries it.
pub const REPLACEMENT: char = '\u{25FB}';
const REPLACEMENT_TEXT: &str = "\u{25FB}";

macro_rules! font_asset {
    ($file:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/fonts/",
            $file
        ))
    };
}

/// Embeds a face as an array static, so the binary holds its bytes once and
/// every reference to them, from any crate or `const`, is to that one
/// symbol. Bytes written inline in a `const fn`, or reached through a
/// static that holds a reference to them, are copied into each crate and
/// codegen unit that uses them.
macro_rules! face_static {
    ($name:ident, $file:literal) => {
        static $name: [u8; font_asset!($file).len()] = *font_asset!($file);
    };
}

face_static!(SANS_REGULAR, "IBMPlexSans-Regular.ttf");
face_static!(SANS_MEDIUM, "IBMPlexSans-Medium.ttf");
face_static!(SANS_SEMIBOLD, "IBMPlexSans-SemiBold.ttf");
face_static!(MONO_REGULAR, "IBMPlexMono-Regular.ttf");
face_static!(MONO_MEDIUM, "IBMPlexMono-Medium.ttf");
face_static!(SYMBOLS, "RSpiceSymbols-Regular.ttf");
#[cfg(not(target_arch = "wasm32"))]
face_static!(SANS_ARABIC_REGULAR, "IBMPlexSansArabic-Regular.ttf");
#[cfg(not(target_arch = "wasm32"))]
face_static!(SANS_ARABIC_SEMIBOLD, "IBMPlexSansArabic-SemiBold.ttf");
#[cfg(not(target_arch = "wasm32"))]
face_static!(SANS_HEBREW_REGULAR, "IBMPlexSansHebrew-Regular.ttf");
#[cfg(not(target_arch = "wasm32"))]
face_static!(SANS_HEBREW_SEMIBOLD, "IBMPlexSansHebrew-SemiBold.ttf");
#[cfg(not(target_arch = "wasm32"))]
face_static!(SANS_THAI_REGULAR, "IBMPlexSansThai-Regular.ttf");
#[cfg(not(target_arch = "wasm32"))]
face_static!(SANS_THAI_SEMIBOLD, "IBMPlexSansThai-SemiBold.ttf");

const IBM_PLEX_LICENCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/fonts/OFL-IBMPlex.txt"
));
const RSPICE_SYMBOLS_LICENCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/fonts/OFL-RSpiceSymbols.txt"
));

/// One embedded face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Face {
    SansRegular,
    SansMedium,
    SansSemibold,
    MonoRegular,
    MonoMedium,
    Symbols,
    #[cfg(not(target_arch = "wasm32"))]
    SansArabicRegular,
    #[cfg(not(target_arch = "wasm32"))]
    SansArabicSemibold,
    #[cfg(not(target_arch = "wasm32"))]
    SansHebrewRegular,
    #[cfg(not(target_arch = "wasm32"))]
    SansHebrewSemibold,
    #[cfg(not(target_arch = "wasm32"))]
    SansThaiRegular,
    #[cfg(not(target_arch = "wasm32"))]
    SansThaiSemibold,
}

impl Face {
    #[cfg(target_arch = "wasm32")]
    pub const ALL: [Self; 6] = [
        Self::SansRegular,
        Self::SansMedium,
        Self::SansSemibold,
        Self::MonoRegular,
        Self::MonoMedium,
        Self::Symbols,
    ];
    #[cfg(not(target_arch = "wasm32"))]
    pub const ALL: [Self; 12] = [
        Self::SansRegular,
        Self::SansMedium,
        Self::SansSemibold,
        Self::MonoRegular,
        Self::MonoMedium,
        Self::Symbols,
        Self::SansArabicRegular,
        Self::SansArabicSemibold,
        Self::SansHebrewRegular,
        Self::SansHebrewSemibold,
        Self::SansThaiRegular,
        Self::SansThaiSemibold,
    ];

    /// The face's TrueType bytes, as embedded in the binary.
    #[must_use]
    pub const fn bytes(self) -> &'static [u8] {
        match self {
            Self::SansRegular => &SANS_REGULAR,
            Self::SansMedium => &SANS_MEDIUM,
            Self::SansSemibold => &SANS_SEMIBOLD,
            Self::MonoRegular => &MONO_REGULAR,
            Self::MonoMedium => &MONO_MEDIUM,
            Self::Symbols => &SYMBOLS,
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicRegular => &SANS_ARABIC_REGULAR,
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicSemibold => &SANS_ARABIC_SEMIBOLD,
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansHebrewRegular => &SANS_HEBREW_REGULAR,
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansHebrewSemibold => &SANS_HEBREW_SEMIBOLD,
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansThaiRegular => &SANS_THAI_REGULAR,
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansThaiSemibold => &SANS_THAI_SEMIBOLD,
        }
    }

    /// The face's family, as a stylesheet or a PDF names it.
    #[must_use]
    pub const fn family(self) -> &'static str {
        match self {
            Self::SansRegular | Self::SansMedium | Self::SansSemibold => "IBM Plex Sans",
            Self::MonoRegular | Self::MonoMedium => "IBM Plex Mono",
            Self::Symbols => "RSpice Symbols",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicRegular | Self::SansArabicSemibold => "IBM Plex Sans Arabic",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansHebrewRegular | Self::SansHebrewSemibold => "IBM Plex Sans Hebrew",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansThaiRegular | Self::SansThaiSemibold => "IBM Plex Sans Thai",
        }
    }

    /// The face's family and weight, as a reader names it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::SansRegular => "IBM Plex Sans Regular",
            Self::SansMedium => "IBM Plex Sans Medium",
            Self::SansSemibold => "IBM Plex Sans SemiBold",
            Self::MonoRegular => "IBM Plex Mono Regular",
            Self::MonoMedium => "IBM Plex Mono Medium",
            Self::Symbols => "RSpice Symbols Regular",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicRegular => "IBM Plex Sans Arabic Regular",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicSemibold => "IBM Plex Sans Arabic SemiBold",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansHebrewRegular => "IBM Plex Sans Hebrew Regular",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansHebrewSemibold => "IBM Plex Sans Hebrew SemiBold",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansThaiRegular => "IBM Plex Sans Thai Regular",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansThaiSemibold => "IBM Plex Sans Thai SemiBold",
        }
    }

    /// The face's weight class, as its `OS/2` table records it.
    #[must_use]
    pub const fn weight(self) -> u16 {
        match self {
            Self::SansRegular | Self::MonoRegular | Self::Symbols => 400,
            Self::SansMedium | Self::MonoMedium => 500,
            Self::SansSemibold => 600,
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicRegular | Self::SansHebrewRegular | Self::SansThaiRegular => 400,
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicSemibold | Self::SansHebrewSemibold | Self::SansThaiSemibold => 600,
        }
    }

    /// A stable lowercase identifier for the face: egui's font-data key.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::SansRegular => "plex-sans",
            Self::SansMedium => "plex-sans-medium",
            Self::SansSemibold => "plex-sans-semibold",
            Self::MonoRegular => "plex-mono",
            Self::MonoMedium => "plex-mono-medium",
            Self::Symbols => "rspice-symbols",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicRegular => "plex-sans-arabic",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansArabicSemibold => "plex-sans-arabic-semibold",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansHebrewRegular => "plex-sans-hebrew",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansHebrewSemibold => "plex-sans-hebrew-semibold",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansThaiRegular => "plex-sans-thai",
            #[cfg(not(target_arch = "wasm32"))]
            Self::SansThaiSemibold => "plex-sans-thai-semibold",
        }
    }

    /// The licence the face is distributed under, with its copyright lines.
    #[must_use]
    pub const fn licence(self) -> &'static str {
        match self {
            Self::Symbols => RSPICE_SYMBOLS_LICENCE,
            _ => IBM_PLEX_LICENCE,
        }
    }

    /// Whether the face has a glyph for `character`.
    #[must_use]
    pub fn covers(self, character: char) -> bool {
        charmap(self).map(character).is_some()
    }
}

/// The face's character map, read once per process. A face that does not
/// parse maps nothing, so its characters fall through to the next face in
/// the chain; the tests hold every embedded face to parsing.
fn charmap(face: Face) -> &'static Charmap<'static> {
    static CHARMAPS: [OnceLock<Charmap<'static>>; Face::ALL.len()] =
        [const { OnceLock::new() }; Face::ALL.len()];
    CHARMAPS[face as usize].get_or_init(|| {
        FontRef::new(face.bytes())
            .map(|font| font.charmap())
            .unwrap_or_default()
    })
}

/// What a run of text asks to be lettered in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Role {
    Sans,
    SansMedium,
    SansSemibold,
    Mono,
    MonoMedium,
}

impl Role {
    pub const ALL: [Self; 5] = [
        Self::Sans,
        Self::SansMedium,
        Self::SansSemibold,
        Self::Mono,
        Self::MonoMedium,
    ];

    /// The faces a character is looked for in, in order; the one table of
    /// chains for screen and print.
    ///
    /// A weighted role borrows from its own weight first, so a borrowed glyph
    /// keeps its stroke where a face of that weight has it. Plex Mono has no
    /// Greek, so the mono roles borrow Δ, Ω and μ from Plex Sans. Every chain
    /// holds RSpice Symbols after the Plex Latin faces. On a desktop the
    /// Arabic, Hebrew and Thai faces follow it: SemiBold for the SemiBold
    /// role, Regular for the others, which have no cut of their weight. Their
    /// Latin is also in Plex Sans, which comes first, so Latin never reaches
    /// them.
    #[must_use]
    pub const fn chain(self) -> &'static [Face] {
        match self {
            Self::Sans => &[
                Face::SansRegular,
                Face::Symbols,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansArabicRegular,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansHebrewRegular,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansThaiRegular,
            ],
            Self::SansMedium => &[
                Face::SansMedium,
                Face::SansRegular,
                Face::Symbols,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansArabicRegular,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansHebrewRegular,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansThaiRegular,
            ],
            Self::SansSemibold => &[
                Face::SansSemibold,
                Face::SansRegular,
                Face::Symbols,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansArabicSemibold,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansHebrewSemibold,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansThaiSemibold,
            ],
            Self::Mono => &[
                Face::MonoRegular,
                Face::SansRegular,
                Face::Symbols,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansArabicRegular,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansHebrewRegular,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansThaiRegular,
            ],
            Self::MonoMedium => &[
                Face::MonoMedium,
                Face::MonoRegular,
                Face::SansMedium,
                Face::SansRegular,
                Face::Symbols,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansArabicRegular,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansHebrewRegular,
                #[cfg(not(target_arch = "wasm32"))]
                Face::SansThaiRegular,
            ],
        }
    }

    /// The face the role's own text is lettered in, whose metrics a layout
    /// measures against.
    #[must_use]
    pub const fn primary(self) -> Face {
        self.chain()[0]
    }

    /// The first face in the chain that has `character`.
    #[must_use]
    pub fn face_for(self, character: char) -> Option<Face> {
        self.chain()
            .iter()
            .copied()
            .find(|face| face.covers(character))
    }

    /// Whether some face in the chain has `character`.
    #[must_use]
    pub fn covers(self, character: char) -> bool {
        self.face_for(character).is_some()
    }

    /// The face that letters `character`, and the character it letters: the
    /// character itself from the first face that has it, or [`REPLACEMENT`]
    /// from RSpice Symbols.
    #[must_use]
    pub fn resolve(self, character: char) -> (Face, char) {
        self.face_for(character)
            .map_or((Face::Symbols, REPLACEMENT), |face| (face, character))
    }

    /// The face print letters the role's own text in: [`Self::primary`].
    #[must_use]
    pub const fn print_primary(self) -> PrintFace {
        PrintFace::Bundled(self.primary())
    }

    /// `text` as print letters it in the role, split where the face changes.
    ///
    /// A grapheme cluster is never split between faces: it is lettered by the
    /// first face that has every character of it that draws, or else by the
    /// face of its first character, so a base and its marks stay together.
    /// The role's chain is looked in first, then, on a desktop, the installed
    /// faces print has been told of (`system::print::print_from`). A
    /// cluster whose first character no face has letters as one
    /// [`REPLACEMENT`].
    #[must_use]
    pub fn runs(self, text: &str) -> Vec<Run> {
        let mut runs: Vec<Run> = Vec::new();
        for cluster in text.graphemes(true) {
            let (face, lettered) = self.letter_cluster(cluster);
            match runs.last_mut() {
                Some(run) if run.face == face => run.text.push_str(lettered),
                _ => runs.push(Run {
                    face,
                    text: lettered.to_owned(),
                }),
            }
        }
        runs
    }

    /// The faces of the chain, as print names them, in order.
    fn print_faces(self) -> impl Iterator<Item = PrintFace> {
        self.chain().iter().map(|face| face.printed())
    }

    fn letter_cluster(self, cluster: &str) -> (PrintFace, &str) {
        match self.cluster_face(cluster) {
            Some(face) => (face, cluster),
            None => (PrintFace::Bundled(Face::Symbols), REPLACEMENT_TEXT),
        }
    }

    /// The face print letters a grapheme cluster in, or `None` when it is
    /// lettered as [`REPLACEMENT`]; see [`Self::runs`].
    fn cluster_face(self, cluster: &str) -> Option<PrintFace> {
        let drawn: Vec<char> = cluster
            .chars()
            .filter(|character| draws(*character))
            .collect();
        let whole = self
            .print_faces()
            .find(|face| drawn.iter().all(|&character| face.covers(character)));
        if whole.is_some() {
            return whole;
        }
        #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
        let installed = system::print::face_for(cluster, &drawn);
        #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
        if let Some((face, true)) = installed {
            return Some(PrintFace::System(face));
        }
        let first = cluster.chars().next()?;
        #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
        let installed = installed.map(|(face, _)| PrintFace::System(face));
        #[cfg(not(all(feature = "system", not(target_arch = "wasm32"))))]
        let installed = None;
        self.print_faces()
            .find(|face| face.covers(first))
            .or(installed)
    }
}

/// A face print letters from: an embedded face, or on a desktop a face
/// installed there that print has been told of.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrintFace {
    Bundled(Face),
    #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
    System(system::print::SystemFaceId),
}

impl PrintFace {
    /// Every embedded face, as print names it.
    pub const BUNDLED: [Self; Face::ALL.len()] = {
        let mut all = [Self::Bundled(Face::SansRegular); Face::ALL.len()];
        let mut at = 0;
        while at < all.len() {
            all[at] = Self::Bundled(Face::ALL[at]);
            at += 1;
        }
        all
    };

    /// The embedded face, or `None` for an installed one.
    #[must_use]
    pub const fn bundled(self) -> Option<Face> {
        match self {
            Self::Bundled(face) => Some(face),
            #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
            Self::System(_) => None,
        }
    }

    /// The bytes of the face's file: the TrueType face embedded, or the
    /// installed file as it was read when the face joined print.
    #[must_use]
    pub fn bytes(self) -> &'static [u8] {
        match self {
            Self::Bundled(face) => face.bytes(),
            #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
            Self::System(face) => face.face().bytes(),
        }
    }

    /// The face's index in [`Self::bytes`], which a collection holds
    /// several of.
    #[must_use]
    pub fn index(self) -> u32 {
        match self {
            Self::Bundled(_) => 0,
            #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
            Self::System(face) => face.face().index(),
        }
    }

    /// The face's family and weight, or an installed face's family.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Bundled(face) => face.name(),
            #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
            Self::System(face) => face.face().family(),
        }
    }

    /// Whether the face has a glyph for `character`.
    #[must_use]
    pub fn covers(self, character: char) -> bool {
        match self {
            Self::Bundled(face) => face.covers(character),
            #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
            Self::System(face) => face.face().covers(character),
        }
    }
}

impl Face {
    /// The face as print names it.
    #[must_use]
    pub const fn printed(self) -> PrintFace {
        PrintFace::Bundled(self)
    }
}

/// Whether a character needs a glyph of its own. Joiners and variation
/// selectors steer how their neighbours are drawn and are not drawn
/// themselves, so a face need not have them to letter their cluster.
fn draws(character: char) -> bool {
    !matches!(
        character,
        '\u{200C}' | '\u{200D}' | '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}'
    )
}

/// A stretch of text print letters in one face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub face: PrintFace,
    pub text: String,
}

#[cfg(test)]
mod tests;
