//! A face cut down to what one page letters, for an SVG to carry.
//!
//! A face is mostly outlines, and a page letters few of them. Every other
//! table travels as it is: the glyph ids, the character map and the
//! substitution and positioning rules stay whole, so a viewer joins, marks
//! and kerns the page's text exactly as print shaped it. The outlines kept
//! are the glyphs print shaped, each lettered character's own glyph, the
//! parts a composite glyph is built from, and the missing-glyph box; every
//! other outline is emptied, as a space's is, and the glyph names go.
//!
//! A subset made for PDF renumbers the glyphs and drops the character map
//! and the layout rules, since a PDF addresses glyphs itself. A viewer of an
//! SVG letters from the text, so such a subset would leave it nothing to
//! letter with.

use std::collections::{BTreeMap, BTreeSet};

/// What a page letters in one face: the glyphs print shaped, and the
/// characters they letter.
#[derive(Debug, Default, Clone)]
pub(in crate::render) struct Lettered {
    pub(in crate::render) glyphs: BTreeSet<u32>,
    pub(in crate::render) characters: BTreeSet<char>,
}

fn u16_at(data: &[u8], at: usize) -> Option<u16> {
    data.get(at..at.checked_add(2)?)
        .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn u32_at(data: &[u8], at: usize) -> Option<u32> {
    data.get(at..at.checked_add(4)?)
        .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// A TrueType-outlined face's tables by tag; none for a collection or a
/// face with CFF outlines.
fn tables(data: &[u8]) -> Option<BTreeMap<[u8; 4], &[u8]>> {
    if u32_at(data, 0)? != 0x0001_0000 && data.get(..4)? != b"true" {
        return None;
    }
    (0..usize::from(u16_at(data, 4)?))
        .map(|index| {
            let record = 12 + 16 * index;
            let tag: [u8; 4] = data.get(record..record + 4)?.try_into().ok()?;
            let offset = usize::try_from(u32_at(data, record + 8)?).ok()?;
            let length = usize::try_from(u32_at(data, record + 12)?).ok()?;
            Some((tag, data.get(offset..offset.checked_add(length)?)?))
        })
        .collect()
}

/// The glyphs a composite glyph's outline is built from; none for a simple
/// or empty one.
fn components(outline: &[u8]) -> Vec<usize> {
    const ARGUMENTS_ARE_WORDS: u16 = 0x0001;
    const SCALE: u16 = 0x0008;
    const MORE: u16 = 0x0020;
    const X_AND_Y_SCALE: u16 = 0x0040;
    const TWO_BY_TWO: u16 = 0x0080;
    let mut parts = Vec::new();
    if u16_at(outline, 0).is_none_or(|contours| contours.cast_signed() >= 0) {
        return parts;
    }
    let mut at = 10;
    while let (Some(flags), Some(part)) = (u16_at(outline, at), u16_at(outline, at + 2)) {
        parts.push(usize::from(part));
        at += if flags & ARGUMENTS_ARE_WORDS != 0 {
            8
        } else {
            6
        };
        at += if flags & SCALE != 0 {
            2
        } else if flags & X_AND_Y_SCALE != 0 {
            4
        } else if flags & TWO_BY_TWO != 0 {
            8
        } else {
            0
        };
        if flags & MORE == 0 {
            break;
        }
    }
    parts
}

/// The sum of `data` as big-endian words, the last padded with zeros.
fn checksum(data: &[u8]) -> u32 {
    data.chunks(4).fold(0_u32, |sum, chunk| {
        let mut word = [0; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

/// `face` with only the outlines `lettered` reaches; `None` where the face
/// is not one this cuts, or cutting it saves nothing.
pub(in crate::render) fn subset(face: &[u8], lettered: &Lettered) -> Option<Vec<u8>> {
    let tables = tables(face)?;
    let head = *tables.get(b"head")?;
    let long = u16_at(head, 50)? != 0;
    let count = usize::from(u16_at(tables.get(b"maxp")?, 4)?);
    let (loca, glyf) = (*tables.get(b"loca")?, *tables.get(b"glyf")?);
    let offsets = (0..=count)
        .map(|index| {
            if long {
                u32_at(loca, 4 * index).and_then(|offset| usize::try_from(offset).ok())
            } else {
                u16_at(loca, 2 * index).map(|offset| usize::from(offset) * 2)
            }
        })
        .collect::<Option<Vec<usize>>>()?;
    let outline = |glyph: usize| glyf.get(offsets[glyph]..offsets[glyph + 1]);
    let parsed = ttf_parser::Face::parse(face, 0).ok()?;
    let mut wanted = std::iter::once(0)
        .chain(
            lettered
                .glyphs
                .iter()
                .filter_map(|glyph| usize::try_from(*glyph).ok()),
        )
        .chain(
            lettered
                .characters
                .iter()
                .filter_map(|character| parsed.glyph_index(*character))
                .map(|glyph| usize::from(glyph.0)),
        )
        .collect::<Vec<_>>();
    let mut kept = vec![false; count];
    while let Some(glyph) = wanted.pop() {
        if glyph >= count || kept[glyph] {
            continue;
        }
        kept[glyph] = true;
        wanted.extend(components(outline(glyph)?));
    }
    // A short location table counts in words, so each outline starts on one.
    let align = if long { 4 } else { 2 };
    let mut cut_glyf = Vec::new();
    // Each outline starts where the one before it ends; the last entry
    // marks where the last one ends.
    let mut starts = Vec::with_capacity(count + 1);
    for (glyph, keep) in kept.iter().enumerate() {
        starts.push(cut_glyf.len());
        if *keep {
            cut_glyf.extend_from_slice(outline(glyph)?);
            cut_glyf.resize(cut_glyf.len().next_multiple_of(align), 0);
        }
    }
    starts.push(cut_glyf.len());
    let mut cut_loca = Vec::with_capacity(loca.len());
    for start in starts {
        if long {
            cut_loca.extend(u32::try_from(start).ok()?.to_be_bytes());
        } else {
            cut_loca.extend(u16::try_from(start / 2).ok()?.to_be_bytes());
        }
    }
    let mut cut = tables
        .iter()
        // A signature no longer signs what it signed.
        .filter(|(tag, _)| **tag != *b"DSIG")
        .map(|(tag, data)| (*tag, data.to_vec()))
        .collect::<BTreeMap<[u8; 4], Vec<u8>>>();
    cut.insert(*b"glyf", cut_glyf);
    cut.insert(*b"loca", cut_loca);
    if let Some(post) = cut.get_mut(b"post")
        && post.len() >= 32
    {
        // Version 3 names no glyph.
        post.truncate(32);
        post[..4].copy_from_slice(&0x0003_0000_u32.to_be_bytes());
    }
    let bytes = assemble(u32_at(face, 0)?, cut)?;
    (bytes.len() < face.len()).then_some(bytes)
}

/// An sfnt of `tables`, its directory in tag order, each table on a word
/// boundary and every checksum, the whole face's among them, set.
fn assemble(version: u32, mut tables: BTreeMap<[u8; 4], Vec<u8>>) -> Option<Vec<u8>> {
    // The face's own checksum is set once the face is whole.
    tables.get_mut(b"head")?.get_mut(8..12)?.fill(0);
    let count = tables.len();
    let power = 1_usize << count.checked_ilog2()?;
    let mut font = Vec::new();
    font.extend(version.to_be_bytes());
    font.extend(u16::try_from(count).ok()?.to_be_bytes());
    font.extend(u16::try_from(power * 16).ok()?.to_be_bytes());
    font.extend(u16::try_from(power.ilog2()).ok()?.to_be_bytes());
    font.extend(u16::try_from((count - power) * 16).ok()?.to_be_bytes());
    let mut offset = 12 + 16 * count;
    let mut head_at = None;
    for (tag, data) in &tables {
        if tag == b"head" {
            head_at = Some(offset);
        }
        font.extend(tag);
        font.extend(checksum(data).to_be_bytes());
        font.extend(u32::try_from(offset).ok()?.to_be_bytes());
        font.extend(u32::try_from(data.len()).ok()?.to_be_bytes());
        offset += data.len().next_multiple_of(4);
    }
    for data in tables.values() {
        font.extend(data);
        font.resize(font.len().next_multiple_of(4), 0);
    }
    let adjustment = 0xB1B0_AFBA_u32.wrapping_sub(checksum(&font));
    font.get_mut(head_at? + 8..head_at? + 12)?
        .copy_from_slice(&adjustment.to_be_bytes());
    Some(font)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The characters of `text`, and the glyphs print shapes them into in
    /// `face`.
    fn lettered(face: rspice_fonts::Face, text: &str) -> Lettered {
        let mut lettered = Lettered::default();
        for piece in rspice_fonts::print::line(rspice_fonts::Role::Sans, text) {
            if piece.face.bundled() == Some(face) {
                lettered
                    .glyphs
                    .extend(piece.glyphs.iter().map(|glyph| glyph.id));
            }
        }
        lettered.characters.extend(text.chars());
        lettered
    }

    /// A cut face keeps every glyph id, its character map and its kerning,
    /// and the outlines of what the page letters, and none it does not.
    #[test]
    fn a_cut_face_keeps_what_its_page_letters_and_its_layout_rules() {
        let face = rspice_fonts::Face::SansRegular;
        let whole = face.bytes();
        let cut = subset(whole, &lettered(face, "AVATAR To Wa 1.2 mA")).unwrap();
        assert!(
            cut.len() * 2 < whole.len(),
            "{} of {} bytes",
            cut.len(),
            whole.len()
        );
        let (whole, cut) = (
            ttf_parser::Face::parse(whole, 0).unwrap(),
            ttf_parser::Face::parse(&cut, 0).unwrap(),
        );
        assert_eq!(cut.number_of_glyphs(), whole.number_of_glyphs());
        assert!(cut.tables().gpos.is_some() && cut.tables().gsub.is_some());
        for character in ['A', 'V', 'T', 'o', 'W', 'a', '1', '.', 'm'] {
            let glyph = cut.glyph_index(character).unwrap();
            assert_eq!(Some(glyph), whole.glyph_index(character));
            assert_eq!(
                cut.glyph_bounding_box(glyph),
                whole.glyph_bounding_box(glyph),
                "{character}"
            );
            assert_eq!(cut.glyph_hor_advance(glyph), whole.glyph_hor_advance(glyph));
        }
        let unused = cut.glyph_index('z').unwrap();
        assert!(whole.glyph_bounding_box(unused).is_some());
        assert_eq!(cut.glyph_bounding_box(unused), None, "z is not lettered");
    }

    /// The face's checksums hold: each table's, and the whole face's.
    #[test]
    fn a_cut_face_is_checksummed_as_a_font_is() {
        let cut = subset(
            rspice_fonts::Face::MonoRegular.bytes(),
            &lettered(rspice_fonts::Face::MonoRegular, "R1 10 k"),
        )
        .unwrap();
        assert_eq!(checksum(&cut), 0xB1B0_AFBA);
        let tables = tables(&cut).unwrap();
        for (index, (tag, data)) in tables.iter().enumerate() {
            let recorded = u32_at(&cut, 12 + 16 * index + 4).unwrap();
            if tag != b"head" {
                assert_eq!(recorded, checksum(data), "{}", String::from_utf8_lossy(tag));
            }
        }
    }
}
