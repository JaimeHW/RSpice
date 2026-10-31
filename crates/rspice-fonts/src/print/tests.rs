use skrifa::instance::{LocationRef, Size};
use skrifa::{GlyphId, MetadataProvider};

use super::*;
use crate::REPLACEMENT;

/// The glyph `face` maps `character` to.
fn glyph_of(face: PrintFace, character: char) -> u32 {
    skrifa::FontRef::new(face.bytes())
        .expect("every embedded face parses")
        .charmap()
        .map(character)
        .map_or(0, GlyphId::to_u32)
}

/// `face`'s own advance for `character`, without any shaping.
fn unshaped_advance(face: PrintFace, character: char) -> i64 {
    let font = skrifa::FontRef::new(face.bytes()).expect("every embedded face parses");
    let glyph = font.charmap().map(character).expect("the face has it");
    font.glyph_metrics(Size::unscaled(), LocationRef::default())
        .advance_width(glyph)
        .map_or(0, |advance| advance.round() as i64)
}

/// Each glyph's text, from left to right.
fn texts<'a>(text: &'a str, pieces: &[Piece]) -> Vec<&'a str> {
    pieces
        .iter()
        .flat_map(|piece| &piece.glyphs)
        .map(|glyph| &text[glyph.text.clone()])
        .collect()
}

const LINES: [&str; 8] = [
    "R1 = 4.7 kΩ",
    "ΔV ≥ 3 µA, ⌀10 mm ⏚",
    "AVATAR To Wa",
    "Grüße, Ünïcode",
    "a\u{0363}b",
    "\u{E000} PUA",
    "אב (גד) R1 = 10 kΩ",
    "مرحبا 123",
];

#[test]
fn every_cluster_of_a_line_is_set_once_and_never_as_a_missing_glyph() {
    for role in Role::ALL {
        for text in LINES {
            let pieces = line(role, text);
            let mut covered: Vec<Range<usize>> = pieces
                .iter()
                .flat_map(|piece| piece.glyphs.iter().map(|glyph| glyph.text.clone()))
                .collect();
            covered.sort_by_key(|range| range.start);
            covered.dedup();
            let mut at = 0;
            for range in &covered {
                assert_eq!(range.start, at, "{role:?} {text:?}: {covered:?}");
                at = range.end;
            }
            assert_eq!(at, text.len(), "{role:?} {text:?}");
            for piece in &pieces {
                assert!(!piece.glyphs.is_empty(), "{role:?} {text:?}");
                assert!(
                    piece.glyphs.iter().all(|glyph| glyph.id != 0),
                    "{role:?} {text:?}: {piece:?}"
                );
            }
        }
    }
}

#[test]
fn latin_is_set_as_its_face_spaces_it_and_reads_left_to_right() {
    let text = "AVATAR To Wa";
    let pieces = line(Role::Sans, text);
    assert_eq!(pieces.len(), 1);
    let piece = &pieces[0];
    assert_eq!(piece.face, PrintFace::Bundled(Face::SansRegular));
    assert!(!piece.right_to_left);
    assert_eq!(
        texts(text, &pieces),
        text.chars().map(String::from).collect::<Vec<_>>()
    );
    // Kerning closes the pairs a face's own advances leave open, so a line
    // placed by summing those advances runs long.
    let unshaped: i64 = text
        .chars()
        .map(|character| unshaped_advance(PrintFace::Bundled(Face::SansRegular), character))
        .sum();
    assert!(
        piece.advance() < unshaped,
        "{} vs {unshaped}",
        piece.advance()
    );
}

#[test]
fn a_borrowed_character_is_set_in_the_face_that_has_it() {
    let text = "ΔV ⌀10";
    let pieces = line(Role::Mono, text);
    let faces: Vec<(PrintFace, Vec<&str>)> = pieces
        .iter()
        .map(|piece| {
            (
                piece.face,
                piece
                    .glyphs
                    .iter()
                    .map(|glyph| &text[glyph.text.clone()])
                    .collect(),
            )
        })
        .collect();
    assert_eq!(
        faces,
        [
            (PrintFace::Bundled(Face::SansRegular), vec!["Δ"]),
            (PrintFace::Bundled(Face::MonoRegular), vec!["V", " "]),
            (PrintFace::Bundled(Face::Symbols), vec!["⌀"]),
            (PrintFace::Bundled(Face::MonoRegular), vec!["1", "0"]),
        ]
    );
}

#[test]
fn a_character_no_face_has_is_set_as_the_square_and_keeps_its_text() {
    let text = "x\u{E000}\u{E001}y";
    let pieces = line(Role::Sans, text);
    let square = glyph_of(PrintFace::Bundled(Face::Symbols), REPLACEMENT);
    assert_eq!(pieces.len(), 3, "{pieces:?}");
    assert_eq!(pieces[1].face, PrintFace::Bundled(Face::Symbols));
    assert_eq!(
        pieces[1]
            .glyphs
            .iter()
            .map(|glyph| (glyph.id, &text[glyph.text.clone()]))
            .collect::<Vec<_>>(),
        [(square, "\u{E000}"), (square, "\u{E001}")]
    );
}

#[test]
fn a_mark_its_face_lacks_sets_its_cluster_as_the_square() {
    // No face has U+0363, so its cluster is lettered by the face of its
    // base, which shapes the mark to its missing glyph.
    let text = "a\u{0363}b";
    let pieces = line(Role::Sans, text);
    let square = glyph_of(PrintFace::Bundled(Face::Symbols), REPLACEMENT);
    assert_eq!(
        pieces
            .iter()
            .map(|piece| (piece.face, texts(text, std::slice::from_ref(piece))))
            .collect::<Vec<_>>(),
        [
            (PrintFace::Bundled(Face::Symbols), vec!["a\u{0363}"]),
            (PrintFace::Bundled(Face::SansRegular), vec!["b"]),
        ]
    );
    assert_eq!(pieces[0].glyphs[0].id, square);
}

#[test]
fn a_right_to_left_run_is_set_from_its_right_end() {
    let text = "R1 אבג";
    let pieces = line(Role::Sans, text);
    let first = &pieces[0];
    assert!(!first.right_to_left);
    assert_eq!(&text[first.glyphs[0].text.clone()], "R");
    let last = pieces.last().expect("pieces");
    assert!(last.right_to_left);
    let starts: Vec<usize> = last.glyphs.iter().map(|glyph| glyph.text.start).collect();
    assert!(
        starts.windows(2).all(|pair| pair[0] > pair[1]),
        "the glyphs of a right-to-left run stand in reverse: {starts:?}"
    );
}

#[test]
fn a_number_inside_a_right_to_left_line_keeps_its_digit_order() {
    let text = "אב 123 גד";
    let pieces = line(Role::Sans, text);
    let digits: Vec<&str> = texts(text, &pieces)
        .into_iter()
        .filter(|text| text.chars().all(|character| character.is_ascii_digit()))
        .collect();
    assert_eq!(digits, ["1", "2", "3"]);
    // The paragraph reads right to left, so its last word stands leftmost.
    assert_eq!(texts(text, &pieces[..1]).concat(), "דג");
}

#[test]
fn a_bracket_in_a_right_to_left_run_is_set_as_its_mirror_image() {
    let text = "אב (גד)";
    let pieces = line(Role::Sans, text);
    let glyph = |wanted: &str| {
        pieces
            .iter()
            .flat_map(|piece| piece.glyphs.iter().map(move |glyph| (piece.face, glyph)))
            .find(|(_, glyph)| &text[glyph.text.clone()] == wanted)
            .map(|(face, glyph)| (face, glyph.id))
            .expect("the bracket is set")
    };
    let (face, opening) = glyph("(");
    assert_eq!(opening, glyph_of(face, ')'));
    let (face, closing) = glyph(")");
    assert_eq!(closing, glyph_of(face, '('));
}

#[test]
fn a_paragraph_separator_is_not_set() {
    let text = "אב\nR1";
    let pieces = line(Role::Sans, text);
    assert!(!texts(text, &pieces).contains(&"\n"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn arabic_is_joined_and_lam_alef_is_one_glyph_for_both_letters() {
    let text = "ببب لا";
    let pieces = line(Role::Sans, text);
    let arabic: Vec<&Piece> = pieces
        .iter()
        .filter(|piece| piece.face == PrintFace::Bundled(Face::SansArabicRegular))
        .collect();
    assert_eq!(arabic.len(), 2, "{pieces:?}");
    assert!(arabic.iter().all(|piece| piece.right_to_left));
    // A joined beh is not the isolated form the character map names.
    let isolated = glyph_of(PrintFace::Bundled(Face::SansArabicRegular), 'ب');
    let beh = arabic
        .iter()
        .find(|piece| piece.glyphs.len() == 3)
        .expect("the three behs are one piece");
    assert!(
        beh.glyphs.iter().all(|glyph| glyph.id != isolated),
        "{beh:?}"
    );
    let lam_alef = arabic
        .iter()
        .find(|piece| piece.glyphs.len() == 1)
        .expect("lam-alef is one glyph");
    assert_eq!(&text[lam_alef.glyphs[0].text.clone()], "لا");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_mark_in_a_right_to_left_run_stands_for_itself() {
    // A mark is a glyph of its own text unless the face composes it with its
    // base, as Plex Sans Hebrew does a shin and its dot; the glyphs of one
    // composed cluster share its text. So a right-to-left piece's text falls
    // from left to right, and a reader that reverses its clusters reads the
    // word as typed.
    for text in ["שָׁלוֹם", "مَرْحَبًا", "בְּרֵאשִׁית"] {
        let pieces = line(Role::Sans, text);
        assert_eq!(pieces.len(), 1, "{text}: {pieces:?}");
        let glyphs = &pieces[0].glyphs;
        assert!(
            glyphs
                .windows(2)
                .all(|pair| pair[0].text == pair[1].text || pair[0].text.start >= pair[1].text.end),
            "{text}: {glyphs:?}"
        );
        let mut clusters: Vec<Range<usize>> = glyphs
            .iter()
            .rev()
            .map(|glyph| glyph.text.clone())
            .collect();
        clusters.dedup();
        let read: String = clusters.into_iter().map(|range| &text[range]).collect();
        assert_eq!(read, text);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn thai_is_set_in_its_face_with_every_mark_drawn() {
    let text = "ภาษาไทย น้ำ";
    let pieces = line(Role::Sans, text);
    assert!(pieces.iter().all(|piece| !piece.right_to_left));
    assert!(
        pieces
            .iter()
            .filter(|piece| piece.face != PrintFace::Bundled(Face::SansRegular))
            .all(|piece| piece.face == PrintFace::Bundled(Face::SansThaiRegular)),
        "{pieces:?}"
    );
    assert!(
        pieces
            .iter()
            .flat_map(|piece| &piece.glyphs)
            .all(|glyph| glyph.id != 0)
    );
}

#[test]
fn a_line_is_as_wide_as_its_pieces_advance() {
    // IBM Plex Mono sets every Latin character 0.6 em wide.
    assert!((width_em(Role::Mono, "R1 = 10k") - 4.8).abs() < 1e-9);
    #[cfg(not(target_arch = "wasm32"))]
    for (role, text) in [
        (Role::Mono, "\u{5de}\u{5d2}\u{5d1}\u{5e8} (R1)"),
        (
            Role::MonoMedium,
            "\u{645}\u{64e}\u{631}\u{652}\u{62d}\u{64e}\u{628}\u{64b}\u{627} 12",
        ),
        (Role::Sans, "\u{e2a}\u{e27}\u{e31}\u{e2a}\u{e14}\u{e35}"),
    ] {
        let pieces: f64 = line(role, text).iter().map(|piece| piece.width(1.0)).sum();
        assert!(pieces > 0.0);
        assert_eq!(width_em(role, text), pieces, "{text}");
        // Measured again, the width is the one kept.
        assert_eq!(width_em(role, text), pieces, "{text}");
    }
}
