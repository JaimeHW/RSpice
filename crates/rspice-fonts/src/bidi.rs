//! Right-to-left text in the order it is displayed.
//!
//! Text is stored in logical order: the order it is typed and read. Arabic
//! and Hebrew read right to left, and a paragraph that mixes them with
//! Latin, numbers or units is displayed by the Unicode Bidirectional
//! Algorithm (UAX #9): each line is cut from the logical text, then its runs
//! are reordered for display. A shaper lays out one run at a time, in the
//! direction it guesses from the run's own script, and knows nothing of the
//! paragraph around it, so the reordering is done here.
//!
//! A line becomes [`Piece`]s, in display order from left to right. Each is
//! lettered in one face and read in one direction, as a shaper given that
//! piece alone will lay it out. Where the shaper would guess the wrong
//! direction (a bracket or a space between Hebrew words is not a Hebrew
//! character, and Arabic-Indic digits read left to right), the piece hands
//! it the text already reversed and mirrored.
//!
//! Text with no right-to-left character answers [`is_logical_order`] from a
//! scan of its characters, and Latin text from its bytes, without allocating.

use std::borrow::Cow;
use std::ops::Range;

use unicode_bidi::BidiInfo;
use unicode_segmentation::UnicodeSegmentation;

use crate::{Face, Role, Sink};

/// Whether `text` is displayed in its logical order: it has no character of
/// a right-to-left script, no Arabic number, and no control that opens a
/// right-to-left run. The bidirectional algorithm then leaves every line
/// as it is.
#[must_use]
pub fn is_logical_order(text: &str) -> bool {
    text.is_ascii() || !text.chars().any(opens_right_to_left)
}

/// Every character whose bidirectional class is right-to-left, Arabic
/// letter or Arabic number, and the right-to-left mark, embedding, override
/// and isolate. The blocks below hold all of them, as the tests check
/// against the class table.
fn opens_right_to_left(character: char) -> bool {
    matches!(
        character,
        '\u{0590}'..='\u{08FF}'
            | '\u{200F}'
            | '\u{202B}'
            | '\u{202E}'
            | '\u{2067}'
            | '\u{FB1D}'..='\u{FDFF}'
            | '\u{FE70}'..='\u{FEFF}'
            | '\u{10800}'..='\u{10FFF}'
            | '\u{1E800}'..='\u{1EFFF}'
    )
}

/// A stretch of one line lettered in one face and read in one direction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece<'a> {
    /// Where the piece is in the logical text, in bytes.
    pub logical: Range<usize>,
    /// The face the role's chain letters the piece in; `None` for characters
    /// past the embedded faces.
    pub face: Option<Face>,
    /// Whether the piece reads right to left. Its glyphs then stand in the
    /// reverse order of its grapheme clusters.
    pub right_to_left: bool,
    /// What to give the shaper: the logical text, or its grapheme clusters
    /// reversed and mirrored where the shaper would lay it out the other way.
    pub shaped: Cow<'a, str>,
}

/// A text's bidirectional levels, from which its lines are laid out.
pub struct BidiText<'a> {
    text: &'a str,
    info: BidiInfo<'a>,
}

impl<'a> BidiText<'a> {
    /// The levels of `text`, whose paragraphs end at its line breaks and take
    /// their direction from their first strong character. `None` when the
    /// text is displayed in logical order, which costs a scan.
    #[must_use]
    pub fn new(text: &'a str) -> Option<Self> {
        if is_logical_order(text) {
            return None;
        }
        Some(Self {
            text,
            info: BidiInfo::new(text, None),
        })
    }

    /// Whether the paragraph holding byte `at` reads right to left.
    #[must_use]
    pub fn paragraph_is_right_to_left(&self, at: usize) -> bool {
        self.paragraph(at)
            .is_some_and(|paragraph| paragraph.level.is_rtl())
    }

    fn paragraph(&self, at: usize) -> Option<&unicode_bidi::ParagraphInfo> {
        self.info
            .paragraphs
            .iter()
            .find(|paragraph| paragraph.range.contains(&at))
    }

    /// The pieces of one line, from left to right as displayed.
    ///
    /// `line` is a byte range within one paragraph, without the line break
    /// that ends it. Whitespace at the end of a line takes the paragraph's
    /// direction, so a wrapped right-to-left line does not start with the
    /// space it was broken at.
    #[must_use]
    pub fn line(&self, role: Role, line: Range<usize>) -> Vec<Piece<'a>> {
        let mut pieces = Vec::new();
        if line.is_empty() {
            return pieces;
        }
        let Some(paragraph) = self.paragraph(line.start) else {
            return pieces;
        };
        let (levels, runs) = self.info.visual_runs(paragraph, line);
        for run in runs {
            let right_to_left = levels[run.start].is_rtl();
            let mut faces = face_runs(self.text, run, role);
            if right_to_left {
                faces.reverse();
            }
            for (logical, face) in faces {
                let text = &self.text[logical.clone()];
                let shaped = if shaper_reads_right_to_left(text) == right_to_left {
                    Cow::Borrowed(text)
                } else {
                    Cow::Owned(reversed_and_mirrored(text))
                };
                pieces.push(Piece {
                    logical,
                    face,
                    right_to_left,
                    shaped,
                });
            }
        }
        pieces
    }
}

/// `run` split where the face changes, in logical order. A grapheme
/// cluster takes the face of its first character, as the screen's layout
/// resolves it.
fn face_runs(text: &str, run: Range<usize>, role: Role) -> Vec<(Range<usize>, Option<Face>)> {
    let mut faces: Vec<(Range<usize>, Option<Face>)> = Vec::new();
    for (offset, cluster) in text[run.clone()].grapheme_indices(true) {
        let start = run.start + offset;
        let end = start + cluster.len();
        let face = cluster
            .chars()
            .next()
            .and_then(|first| role.face_for(Sink::Screen, first));
        match faces.last_mut() {
            Some((range, previous)) if *previous == face => range.end = end,
            _ => faces.push((start..end, face)),
        }
    }
    faces
}

/// Whether the shaper lays `text` out right to left. It takes the direction
/// of the first character that belongs to a script, and left to right when
/// none does; asking it gives the answer the screen's layout will act on.
fn shaper_reads_right_to_left(text: &str) -> bool {
    let mut buffer = harfrust::UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.guess_segment_properties();
    buffer.direction() == harfrust::Direction::RightToLeft
}

/// `text`'s grapheme clusters in reverse order, with each character that
/// has a mirror image replaced by it, so a bracket still opens towards what
/// it encloses.
fn reversed_and_mirrored(text: &str) -> String {
    text.graphemes(true)
        .rev()
        .flat_map(str::chars)
        .map(|character| unicode_bidi_mirroring::get_mirrored(character).unwrap_or(character))
        .collect()
}

#[cfg(test)]
mod tests;
