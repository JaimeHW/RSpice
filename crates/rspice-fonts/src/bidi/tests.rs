use unicode_bidi::{BidiClass, bidi_class};

use super::*;

/// The pieces of one-line `text`, as (logical text, right to left, shaped).
fn display(role: Role, text: &str) -> Vec<(&str, bool, String)> {
    let bidi = BidiText::new(text).expect("the text needs reordering");
    bidi.line(role, 0..text.len())
        .into_iter()
        .map(|piece| {
            (
                &text[piece.logical],
                piece.right_to_left,
                piece.shaped.into_owned(),
            )
        })
        .collect()
}

fn logical_order(pieces: &[(&str, bool, String)]) -> Vec<String> {
    pieces.iter().map(|(text, ..)| (*text).to_owned()).collect()
}

#[test]
fn text_without_right_to_left_characters_is_left_as_it_is() {
    for text in [
        "",
        "R1 = 4.7 kΩ",
        "ΔV ≥ 3 µA, ⌀10 mm",
        "电阻 10 kΩ",
        "ภาษาไทย",
        "Grüße, Ünïcode",
        "\u{202A}LRE\u{202C} \u{200E}",
    ] {
        assert!(is_logical_order(text), "{text:?}");
        assert!(BidiText::new(text).is_none(), "{text:?}");
    }
}

#[test]
fn the_scan_catches_every_character_that_can_open_a_right_to_left_run() {
    let opening = [
        BidiClass::R,
        BidiClass::AL,
        BidiClass::AN,
        BidiClass::RLE,
        BidiClass::RLO,
        BidiClass::RLI,
    ];
    let missed: Vec<char> = (0..=u32::from(char::MAX))
        .filter_map(char::from_u32)
        .filter(|character| opening.contains(&bidi_class(*character)))
        .filter(|character| is_logical_order(&character.to_string()))
        .collect();
    assert!(missed.is_empty(), "{missed:?}");
}

#[test]
fn hebrew_words_display_from_right_to_left() {
    let pieces = display(Role::Sans, "שלום עולם");
    assert_eq!(logical_order(&pieces), ["עולם", " ", "שלום"]);
    assert!(pieces.iter().all(|(_, right_to_left, _)| *right_to_left));
    // The shaper reads Hebrew right to left by itself and is given it as typed.
    assert_eq!(pieces[0].2, "עולם");
}

#[test]
fn numbers_inside_arabic_keep_their_digit_order() {
    let pieces = display(Role::Sans, "عدد 123 مرحبا");
    assert_eq!(logical_order(&pieces), ["مرحبا", " ", "123", " ", "عدد"]);
    assert_eq!(pieces[2], ("123", false, "123".to_owned()));
}

#[test]
fn a_value_and_its_unit_read_left_to_right_inside_hebrew() {
    let pieces = display(Role::Sans, "נגד R1 = 4.7 kΩ");
    assert_eq!(logical_order(&pieces), ["R1 = 4.7 kΩ", " ", "נגד"]);
    assert!(!pieces[0].1);
    assert_eq!(pieces[0].2, "R1 = 4.7 kΩ");
}

#[test]
fn a_bracketed_signal_name_keeps_its_brackets_inside_arabic() {
    let pieces = display(Role::Mono, "الجهد V(out)");
    assert_eq!(logical_order(&pieces), ["V(out)", " ", "الجهد"]);
    assert!(!pieces[0].1);
}

#[test]
fn brackets_lettered_by_another_face_are_mirrored_for_the_shaper() {
    // The brackets fall to Plex Sans, so the shaper sees a bracket on its
    // own and would draw it facing the wrong way.
    let pieces = display(Role::Sans, "(שלום)");
    assert_eq!(logical_order(&pieces), [")", "שלום", "("]);
    let shaped: Vec<&str> = pieces.iter().map(|(.., shaped)| shaped.as_str()).collect();
    assert_eq!(shaped, ["(", "שלום", ")"]);
}

#[test]
fn arabic_indic_digits_are_handed_to_the_shaper_reversed() {
    // The shaper takes the digits' script, Arabic, to read right to left,
    // and would reverse them; reversed once more, they read as written.
    let pieces = display(Role::Sans, "عدد ١٢٣");
    assert_eq!(logical_order(&pieces), ["١٢٣", " ", "عدد"]);
    assert_eq!(pieces[0], ("١٢٣", false, "٣٢١".to_owned()));
}

#[test]
fn a_letter_and_its_marks_stay_in_one_piece() {
    let pieces = display(Role::Sans, "שָׁלוֹם");
    assert_eq!(pieces.len(), 1);
    assert_eq!(pieces[0].2, "שָׁלוֹם");
}

#[test]
fn each_paragraph_takes_its_direction_from_its_first_strong_character() {
    let text = "abc\nשלום";
    let bidi = BidiText::new(text).expect("the text needs reordering");
    assert!(!bidi.paragraph_is_right_to_left(0));
    assert!(bidi.paragraph_is_right_to_left(4));
    let pieces = bidi.line(Role::Sans, 4..text.len());
    assert_eq!(pieces.len(), 1);
    assert!(pieces[0].right_to_left);
}

#[test]
fn a_line_broken_after_a_space_ends_on_the_paragraph_side() {
    let text = "שלום עולם";
    let bidi = BidiText::new(text).expect("the text needs reordering");
    let first_line = 0..text.find('ע').expect("a second word");
    let pieces = bidi.line(Role::Sans, first_line);
    assert_eq!(&text[pieces[0].logical.clone()], " ");
    assert_eq!(&text[pieces[1].logical.clone()], "שלום");
}

#[test]
fn the_pieces_of_a_line_cover_it_once() {
    for text in [
        "עדכון R12 → 4.7 kΩ (±1%)",
        "مرحبا World 2026",
        "a אבג b",
        "ﷺ ﷲ لا",
        "\u{202E}abc\u{202C} def",
    ] {
        let bidi = BidiText::new(text).expect("the text needs reordering");
        let mut covered: Vec<Range<usize>> = bidi
            .line(Role::Sans, 0..text.len())
            .into_iter()
            .map(|piece| piece.logical)
            .collect();
        covered.sort_by_key(|range| range.start);
        let mut next = 0;
        for range in covered {
            assert_eq!(range.start, next, "{text:?}");
            next = range.end;
        }
        assert_eq!(next, text.len(), "{text:?}");
    }
}
