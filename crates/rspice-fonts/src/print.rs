//! Text as print sets it: shaped, and in the order it is read.
//!
//! Print draws glyphs, not characters. A line is cut into the runs of the
//! Unicode Bidirectional Algorithm (UAX #9), in display order. Each run is
//! split where the face changes, by the rule [`Role::runs`] letters a
//! grapheme cluster with, and each piece is shaped in the run's own
//! direction, which joins Arabic, stacks Thai marks, mirrors a bracket in a
//! right-to-left run and hands the glyphs back from left to right.
//!
//! Every glyph keeps the logical bytes of the cluster it letters, so a PDF
//! maps it back to the text it stands for and copied text comes out in the
//! order it was typed, whatever order the glyphs stand in.
//!
//! A cluster no face has is set as [`REPLACEMENT`] from RSpice Symbols, and
//! so is a cluster its face shapes to the missing glyph, as when a face has
//! a base but not the mark on it: no sink ever draws a face's notdef box.

use std::ops::Range;
use std::sync::OnceLock;

use harfrust::{BufferFlags, Direction, FontRef, ShaperData, UnicodeBuffer};
use unicode_bidi::{BidiClass, BidiInfo, bidi_class};
use unicode_segmentation::UnicodeSegmentation;

use crate::bidi::is_logical_order;
use crate::{PrintFace, REPLACEMENT_TEXT, Role};

/// A glyph as the shaper set it, in its face's font units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Glyph {
    /// The glyph's index in its face.
    pub id: u32,
    /// The bytes of the line's text the glyph's cluster letters.
    pub text: Range<usize>,
    pub advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
}

/// A stretch of a printed line in one face and one direction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub face: PrintFace,
    pub right_to_left: bool,
    /// The glyphs from left to right.
    pub glyphs: Vec<Glyph>,
}

impl Piece {
    /// How far the piece advances, in its face's font units.
    #[must_use]
    pub fn advance(&self) -> i64 {
        self.glyphs
            .iter()
            .map(|glyph| i64::from(glyph.advance))
            .sum()
    }
}

/// `text` as print sets it in `role`: its pieces from left to right.
///
/// The text is one line. A paragraph separator in it ends a paragraph,
/// whose direction its first strong character sets, and is not drawn.
#[must_use]
pub fn line(role: Role, text: &str) -> Vec<Piece> {
    let mut pieces = Vec::new();
    for (run, right_to_left) in level_runs(text) {
        let mut stretches = stretches(role, text, run);
        if right_to_left {
            stretches.reverse();
        }
        for stretch in stretches {
            set(text, &stretch, right_to_left, &mut pieces);
        }
    }
    pieces
}

/// The runs of one level, from left to right, each with whether it reads
/// right to left.
fn level_runs(text: &str) -> Vec<(Range<usize>, bool)> {
    if text.is_empty() {
        return Vec::new();
    }
    if is_logical_order(text) {
        return vec![(0..text.len(), false)];
    }
    let info = BidiInfo::new(text, None);
    let mut runs = Vec::new();
    for paragraph in &info.paragraphs {
        let separator: usize = text[paragraph.range.clone()]
            .chars()
            .rev()
            .take_while(|&character| bidi_class(character) == BidiClass::B)
            .map(char::len_utf8)
            .sum();
        let line = paragraph.range.start..paragraph.range.end - separator;
        if line.is_empty() {
            continue;
        }
        let (levels, visual) = info.visual_runs(paragraph, line);
        runs.extend(visual.into_iter().map(|run| {
            let right_to_left = levels[run.start].is_rtl();
            (run, right_to_left)
        }));
    }
    runs
}

/// Consecutive clusters lettered alike: in one face, or as the replacement.
struct Stretch {
    text: Range<usize>,
    face: Option<PrintFace>,
}

/// `run` split where the lettering changes, in logical order.
fn stretches(role: Role, text: &str, run: Range<usize>) -> Vec<Stretch> {
    let mut stretches: Vec<Stretch> = Vec::new();
    for (offset, cluster) in text[run.clone()].grapheme_indices(true) {
        let start = run.start + offset;
        let end = start + cluster.len();
        let face = role.cluster_face(cluster);
        match stretches.last_mut() {
            Some(stretch) if stretch.face == face => stretch.text.end = end,
            _ => stretches.push(Stretch {
                text: start..end,
                face,
            }),
        }
    }
    stretches
}

/// Sets `stretch` in its face, adding its pieces to `pieces`. A cluster the
/// face shapes to the missing glyph is set as the replacement instead, as
/// a piece of its own.
fn set(text: &str, stretch: &Stretch, right_to_left: bool, pieces: &mut Vec<Piece>) {
    let Some(face) = stretch.face else {
        push_replacements(text, stretch.text.clone(), right_to_left, pieces);
        return;
    };
    let shaped = shape(face, text, stretch.text.clone(), right_to_left);
    let missing: Vec<Range<usize>> = shaped
        .iter()
        .filter(|glyph| glyph.id == 0)
        .map(|glyph| glyph.text.clone())
        .collect();
    let mut kept = Vec::new();
    let mut replaced: Vec<Range<usize>> = Vec::new();
    for glyph in shaped {
        if missing.contains(&glyph.text) {
            if !replaced.contains(&glyph.text) {
                flush(face, right_to_left, &mut kept, pieces);
                replaced.push(glyph.text.clone());
                push_replacements(text, glyph.text, right_to_left, pieces);
            }
        } else {
            kept.push(glyph);
        }
    }
    flush(face, right_to_left, &mut kept, pieces);
}

fn flush(face: PrintFace, right_to_left: bool, glyphs: &mut Vec<Glyph>, pieces: &mut Vec<Piece>) {
    if glyphs.is_empty() {
        return;
    }
    pieces.push(Piece {
        face,
        right_to_left,
        glyphs: std::mem::take(glyphs),
    });
}

/// Sets each grapheme cluster of `range` as one replacement square that
/// stands for it, joining the last piece when that is replacements too.
fn push_replacements(
    text: &str,
    range: Range<usize>,
    right_to_left: bool,
    pieces: &mut Vec<Piece>,
) {
    let square = replacement_glyph();
    let mut glyphs: Vec<Glyph> = text[range.clone()]
        .grapheme_indices(true)
        .map(|(offset, cluster)| Glyph {
            text: range.start + offset..range.start + offset + cluster.len(),
            ..square.clone()
        })
        .collect();
    if right_to_left {
        glyphs.reverse();
    }
    match pieces.last_mut() {
        Some(piece)
            if piece.face == PrintFace::Symbols
                && piece.right_to_left == right_to_left
                && piece.glyphs.iter().all(|glyph| glyph.id == square.id) =>
        {
            piece.glyphs.extend(glyphs);
        }
        _ => pieces.push(Piece {
            face: PrintFace::Symbols,
            right_to_left,
            glyphs,
        }),
    }
}

/// The replacement square as RSpice Symbols sets it.
fn replacement_glyph() -> &'static Glyph {
    static SQUARE: OnceLock<Glyph> = OnceLock::new();
    SQUARE.get_or_init(|| {
        shape(
            PrintFace::Symbols,
            REPLACEMENT_TEXT,
            0..REPLACEMENT_TEXT.len(),
            false,
        )
        .into_iter()
        .next()
        .unwrap_or(Glyph {
            id: 0,
            text: 0..0,
            advance: 0,
            x_offset: 0,
            y_offset: 0,
        })
    })
}

/// A face, parsed, with what the shaper caches for it.
struct Shaping {
    font: FontRef<'static>,
    data: ShaperData,
}

fn shaping(face: PrintFace) -> Option<&'static Shaping> {
    static SHAPING: [OnceLock<Option<Shaping>>; PrintFace::ALL.len()] =
        [const { OnceLock::new() }; PrintFace::ALL.len()];
    SHAPING[face as usize]
        .get_or_init(|| {
            let font = FontRef::new(face.bytes()).ok()?;
            let data = ShaperData::new(&font);
            Some(Shaping { font, data })
        })
        .as_ref()
}

/// `text[range]` shaped in `face`, from left to right. Each glyph's text is
/// its cluster's bytes in `text`. A face that does not parse sets nothing;
/// the tests hold every embedded face to parsing.
fn shape(face: PrintFace, text: &str, range: Range<usize>, right_to_left: bool) -> Vec<Glyph> {
    let Some(shaping) = shaping(face) else {
        return Vec::new();
    };
    let mut buffer = UnicodeBuffer::new();
    let mut flags = BufferFlags::empty();
    if range.start == 0 {
        flags |= BufferFlags::BEGINNING_OF_TEXT;
    }
    if range.end == text.len() {
        flags |= BufferFlags::END_OF_TEXT;
    }
    buffer.set_flags(flags);
    buffer.push_str(&text[range.clone()]);
    buffer.set_direction(if right_to_left {
        Direction::RightToLeft
    } else {
        Direction::LeftToRight
    });
    buffer.guess_segment_properties();
    let shaper = shaping.data.shaper(&shaping.font).build();
    let output = shaper.shape(buffer, &[]);
    let infos = output.glyph_infos();
    let mut starts: Vec<usize> = infos.iter().map(|info| info.cluster as usize).collect();
    starts.sort_unstable();
    starts.dedup();
    let length = range.len();
    infos
        .iter()
        .zip(output.glyph_positions())
        .map(|(info, position)| {
            let start = info.cluster as usize;
            let end = starts
                .iter()
                .copied()
                .find(|&next| next > start)
                .unwrap_or(length);
            Glyph {
                id: info.glyph_id,
                text: range.start + start..range.start + end,
                advance: position.x_advance,
                x_offset: position.x_offset,
                y_offset: position.y_offset,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
