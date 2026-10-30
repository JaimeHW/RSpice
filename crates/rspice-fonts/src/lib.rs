//! The faces RSpice embeds and the one table of fallback chains.
//!
//! Every crate that letters text takes its faces from here: the egui screen,
//! the PDF and SVG hardcopy, the shortcut reference, the published page and
//! its viewer. Each face is therefore embedded once, and screen and print fall
//! back through the same chain.
//!
//! A [`Role`] is what a run of text asks for: sans or mono, at a weight. Its
//! [`Role::chain`] lists the faces a character is looked for in, in order.
//! Every chain ends in RSpice Symbols, which carries the engineering symbols
//! no Plex face has (⌀ ∅ ⏚, the GD&T characteristics, the interface marks)
//! and [`REPLACEMENT`], the square lettered for a character no face covers.
//!
//! The faces are distributed under the SIL Open Font License 1.1; each
//! face's [`Face::licence`] is the text that must travel with it.

#![deny(clippy::print_stdout, clippy::print_stderr)]

use std::sync::OnceLock;

use skrifa::{FontRef, MetadataProvider, charmap::Charmap};
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "egui")]
mod egui_families;
#[cfg(feature = "egui")]
pub use egui_families::egui_font_definitions;

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
}

impl Face {
    pub const ALL: [Self; 6] = [
        Self::SansRegular,
        Self::SansMedium,
        Self::SansSemibold,
        Self::MonoRegular,
        Self::MonoMedium,
        Self::Symbols,
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
        }
    }

    /// The face's family, as a stylesheet or a PDF names it.
    #[must_use]
    pub const fn family(self) -> &'static str {
        match self {
            Self::SansRegular | Self::SansMedium | Self::SansSemibold => "IBM Plex Sans",
            Self::MonoRegular | Self::MonoMedium => "IBM Plex Mono",
            Self::Symbols => "RSpice Symbols",
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
        }
    }

    /// The face's weight class, as its `OS/2` table records it.
    #[must_use]
    pub const fn weight(self) -> u16 {
        match self {
            Self::SansRegular | Self::MonoRegular | Self::Symbols => 400,
            Self::SansMedium | Self::MonoMedium => 500,
            Self::SansSemibold => 600,
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
    /// ends in RSpice Symbols.
    #[must_use]
    pub const fn chain(self) -> &'static [Face] {
        match self {
            Self::Sans => &[Face::SansRegular, Face::Symbols],
            Self::SansMedium => &[Face::SansMedium, Face::SansRegular, Face::Symbols],
            Self::SansSemibold => &[Face::SansSemibold, Face::SansRegular, Face::Symbols],
            Self::Mono => &[Face::MonoRegular, Face::SansRegular, Face::Symbols],
            Self::MonoMedium => &[
                Face::MonoMedium,
                Face::MonoRegular,
                Face::SansMedium,
                Face::SansRegular,
                Face::Symbols,
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

    /// `text` as the role letters it, split where the face changes.
    ///
    /// A grapheme cluster is never split between faces: it is lettered by the
    /// first face that has every character of it that draws, or else by the
    /// face of its first character, so a base and its marks stay together. A
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

    fn letter_cluster(self, cluster: &str) -> (Face, &str) {
        let whole = self.chain().iter().copied().find(|face| {
            cluster
                .chars()
                .filter(|character| draws(*character))
                .all(|character| face.covers(character))
        });
        if let Some(face) = whole {
            return (face, cluster);
        }
        match cluster
            .chars()
            .next()
            .and_then(|first| self.face_for(first))
        {
            Some(face) => (face, cluster),
            None => (Face::Symbols, REPLACEMENT_TEXT),
        }
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

/// A stretch of text lettered in one face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub face: Face,
    pub text: String,
}

#[cfg(test)]
mod tests;
