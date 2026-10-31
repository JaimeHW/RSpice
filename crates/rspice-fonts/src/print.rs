//! Text as print sets it: shaped, and in the order it is read.
//!
//! Print draws glyphs, not characters. A line is cut into the runs of the
//! Unicode Bidirectional Algorithm (UAX #9), in display order. Each run is
//! split where the face changes, by the rule [`Role::runs`] letters a
//! grapheme cluster with, and each piece is shaped in the run's own
//! direction, which joins Arabic, stacks Thai marks, mirrors a bracket in a
//! right-to-left run and hands the glyphs back from left to right.
//!
//! Every glyph keeps the logical bytes of the characters it letters: its own
//! character, every character of a ligature, or, where the face composes a
//! mark with its base, the whole composed cluster, which each of its glyphs
//! shares. The text of a right-to-left piece's glyphs therefore falls from
//! left to right, and a PDF can map each glyph back to what it stands for,
//! giving copied text in the order it was typed whatever order the glyphs
//! stand in.
//!
//! A grapheme cluster no face has is set as [`crate::REPLACEMENT`] from
//! RSpice Symbols, and so is one its face shapes to the missing glyph, as
//! when a face has a base but not the mark on it: print never draws a face's
//! notdef box.

use std::collections::HashMap;
use std::ops::Range;
use std::sync::{Mutex, OnceLock, PoisonError};

use harfrust::{BufferClusterLevel, BufferFlags, Direction, FontRef, ShaperData, UnicodeBuffer};
use skrifa::MetadataProvider;
use skrifa::instance::{LocationRef, Size};
use unicode_bidi::{BidiClass, BidiInfo, bidi_class};
use unicode_segmentation::UnicodeSegmentation;

use crate::bidi::is_logical_order;
use crate::{Face, PrintFace, REPLACEMENT_TEXT, Role};

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
    /// Whether the piece is replacement squares, each standing for a
    /// grapheme cluster no face letters.
    pub replacement: bool,
    /// The font units to the em of the piece's face.
    pub units_per_em: u16,
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

    /// How far the piece advances lettered `size` units to the em.
    #[must_use]
    pub fn width(&self, size: f64) -> f64 {
        self.advance() as f64 * size / f64::from(self.units_per_em.max(1))
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

/// How wide print sets `text` in `role`, in ems: what its pieces advance.
///
/// The canvas and print both cut a title-block value by this width, so the
/// two cut it at the same place. A width is measured once and kept while the
/// installed faces print looks in stand as they are, so a frame that letters
/// the same text again shapes nothing.
#[must_use]
pub fn width_em(role: Role, text: &str) -> f64 {
    let changes = installed_changes();
    if let Some(widths) = WIDTHS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .as_ref()
        .filter(|widths| widths.changes == changes)
        && let Some(&em) = widths.ems.get(&role).and_then(|ems| ems.get(text))
    {
        return em;
    }
    let em = line(role, text).iter().map(|piece| piece.width(1.0)).sum();
    let mut widths = WIDTHS.lock().unwrap_or_else(PoisonError::into_inner);
    let widths = widths.get_or_insert_with(Widths::default);
    if widths.changes != changes || widths.count >= MAX_WIDTHS {
        *widths = Widths {
            changes,
            ..Widths::default()
        };
    }
    if widths
        .ems
        .entry(role)
        .or_default()
        .insert(text.into(), em)
        .is_none()
    {
        widths.count += 1;
    }
    em
}

/// The most widths [`width_em`] keeps; past it, it starts again.
const MAX_WIDTHS: usize = 4_096;

/// The widths [`width_em`] has measured.
static WIDTHS: Mutex<Option<Widths>> = Mutex::new(None);

#[derive(Default)]
struct Widths {
    /// What [`installed_changes`] said when they were measured.
    changes: u64,
    ems: HashMap<Role, HashMap<Box<str>, f64>>,
    count: usize,
}

/// A count that moves whenever print may letter a cluster in another
/// installed face than before; a build without installed faces never moves
/// it.
fn installed_changes() -> u64 {
    #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
    {
        crate::system::print::changes()
    }
    #[cfg(not(all(feature = "system", not(target_arch = "wasm32"))))]
    {
        0
    }
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

/// Sets `stretch` in its face, adding its pieces to `pieces`. A grapheme
/// cluster the face shapes to the missing glyph is set as the replacement
/// instead, as a piece of its own.
fn set(text: &str, stretch: &Stretch, right_to_left: bool, pieces: &mut Vec<Piece>) {
    let Some(face) = stretch.face else {
        push_replacements(text, stretch.text.clone(), right_to_left, pieces);
        return;
    };
    let shaped = shape(face, text, stretch.text.clone(), right_to_left);
    let clusters: Vec<Range<usize>> = text[stretch.text.clone()]
        .grapheme_indices(true)
        .map(|(offset, cluster)| {
            let start = stretch.text.start + offset;
            start..start + cluster.len()
        })
        .collect();
    let cluster_of = |glyph: &Glyph| {
        clusters
            .iter()
            .find(|cluster| cluster.contains(&glyph.text.start))
            .cloned()
            .unwrap_or_else(|| glyph.text.clone())
    };
    let missing: Vec<Range<usize>> = shaped
        .iter()
        .filter(|glyph| glyph.id == 0)
        .map(cluster_of)
        .collect();
    let mut kept = Vec::new();
    let mut replaced: Vec<Range<usize>> = Vec::new();
    for glyph in shaped {
        let cluster = cluster_of(&glyph);
        if missing.contains(&cluster) {
            if !replaced.contains(&cluster) {
                flush(face, right_to_left, &mut kept, pieces);
                replaced.push(cluster.clone());
                push_replacements(text, cluster, right_to_left, pieces);
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
        replacement: false,
        units_per_em: units_per_em(face),
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
        Some(piece) if piece.replacement && piece.right_to_left == right_to_left => {
            piece.glyphs.extend(glyphs);
        }
        _ => pieces.push(Piece {
            face: PrintFace::Bundled(Face::Symbols),
            right_to_left,
            replacement: true,
            units_per_em: units_per_em(PrintFace::Bundled(Face::Symbols)),
            glyphs,
        }),
    }
}

/// The replacement square as RSpice Symbols sets it.
fn replacement_glyph() -> &'static Glyph {
    static SQUARE: OnceLock<Glyph> = OnceLock::new();
    SQUARE.get_or_init(|| {
        shape(
            PrintFace::Bundled(Face::Symbols),
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
    units_per_em: u16,
}

/// The font units to the em of `face`.
fn units_per_em(face: PrintFace) -> u16 {
    shaping(face).map_or(1_000, |shaping| shaping.units_per_em)
}

fn shaping(face: PrintFace) -> Option<&'static Shaping> {
    match face {
        PrintFace::Bundled(face) => {
            static SHAPING: [OnceLock<Option<Shaping>>; Face::ALL.len()] =
                [const { OnceLock::new() }; Face::ALL.len()];
            SHAPING[face as usize]
                .get_or_init(|| Shaping::new(face.bytes(), 0))
                .as_ref()
        }
        #[cfg(all(feature = "system", not(target_arch = "wasm32")))]
        PrintFace::System(id) => {
            use std::collections::HashMap;
            use std::sync::{Mutex, PoisonError};
            // An installed face is parsed once, the first time print sets
            // text in it, and kept as long as the face itself.
            static SHAPING: Mutex<
                Option<HashMap<crate::system::print::SystemFaceId, Option<&'static Shaping>>>,
            > = Mutex::new(None);
            *SHAPING
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .get_or_insert_with(HashMap::new)
                .entry(id)
                .or_insert_with(|| {
                    let face = id.face();
                    Shaping::new(face.bytes(), face.index())
                        .map(|shaping| &*Box::leak(Box::new(shaping)))
                })
        }
    }
}

impl Shaping {
    fn new(bytes: &'static [u8], index: u32) -> Option<Self> {
        let font = FontRef::from_index(bytes, index).ok()?;
        let data = ShaperData::new(&font);
        let units_per_em = font
            .metrics(Size::unscaled(), LocationRef::default())
            .units_per_em;
        Some(Self {
            font,
            data,
            units_per_em,
        })
    }
}

/// `text[range]` shaped in `face`, from left to right. Each character is a
/// cluster of its own unless the face ligates it with others, so a glyph's
/// text is its characters' bytes in `text`, and the clusters stand in order
/// of the run's direction. A face that does not parse sets nothing; the
/// tests hold every embedded face to parsing.
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
    buffer.set_cluster_level(BufferClusterLevel::MonotoneCharacters);
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
