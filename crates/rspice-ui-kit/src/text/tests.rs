use std::cell::Cell;

use egui::{Context, RawInput, pos2, vec2};

use super::*;
use crate::raster::{Canvas, render};

const SIZE: f32 = 40.0;

fn sans() -> FontId {
    FontId::proportional(SIZE)
}

/// Runs `read` with the application's fonts, inside a pass so they are
/// built.
fn with_fonts<R>(read: impl FnOnce(&mut FontsView<'_>) -> R) -> R {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    let mut read = Some(read);
    let mut out = None;
    let _ = ctx.run_ui(RawInput::default(), |ui| {
        if let Some(read) = read.take() {
            out = Some(ui.fonts_mut(read));
        }
    });
    out.expect("the pass ran")
}

fn displayed(text: &str) -> Arc<Galley> {
    with_fonts(|fonts| layout_no_wrap(fonts, text, sans(), Color32::WHITE))
}

/// The left edge of the first glyph lettering `character`.
fn x_of(galley: &Galley, character: char) -> f32 {
    galley
        .rows
        .iter()
        .flat_map(|row| {
            row.row
                .glyphs
                .iter()
                .map(move |glyph| (row.pos.x + glyph.pos.x, glyph.chr))
        })
        .find(|(_, chr)| *chr == character)
        .map(|(x, _)| x)
        .unwrap_or_else(|| panic!("{character:?} is lettered in {:?}", galley.job.text))
}

fn glyph_count(galley: &Galley) -> usize {
    galley
        .rows
        .iter()
        .map(|row| row.row.glyphs.len() + usize::from(row.ends_with_newline))
        .sum()
}

#[test]
fn text_without_right_to_left_characters_is_laid_out_as_egui_lays_it_out() {
    for text in ["R1 4.7 kΩ ±1 %", "ภาษาไทย 25 °C", "电阻 10 kΩ"] {
        let (ours, egui) = with_fonts(|fonts| {
            (
                layout_no_wrap(fonts, text, sans(), Color32::WHITE),
                fonts.layout_no_wrap(text.to_owned(), sans(), Color32::WHITE),
            )
        });
        assert_eq!(ours.job, egui.job, "{text:?}");
    }
}

#[test]
#[ignore = "needs the emilk/egui#8576 fix, which no egui release has yet"]
fn every_glyph_stands_for_one_character() {
    // egui 0.35.0 lays a right-to-left run out with 2n - 1 glyphs, which
    // throws the text cursor past the end of the text. Plex Sans Arabic and
    // Hebrew shape no character to several glyphs, so the count is exact.
    for text in [
        "بب",
        "שלום 42 עולם",
        "عدد 123 مرحبا",
        "بِسْمِ ٱللَّٰهِ",
        "שָׁלוֹם",
        "لا ﷲ ﷺ",
    ] {
        let galley = displayed(text);
        assert_eq!(glyph_count(&galley), text.chars().count(), "{text:?}");
        assert_eq!(galley.end().index.0, text.chars().count(), "{text:?}");
    }
}

#[test]
fn hebrew_words_read_from_right_to_left() {
    let galley = displayed("שלום עולם");
    assert!(x_of(&galley, 'ש') > x_of(&galley, 'ע'));
}

#[test]
fn a_number_inside_arabic_reads_left_to_right_where_it_stands() {
    let galley = displayed("عدد 123 مرحبا");
    let (one, two, three) = (x_of(&galley, '1'), x_of(&galley, '2'), x_of(&galley, '3'));
    assert!(one < two && two < three);
    assert!(x_of(&galley, 'م') < one, "the last word is on the left");
    assert!(three < x_of(&galley, 'ع'), "the first word is on the right");
}

#[test]
fn a_value_and_its_unit_keep_their_order_inside_hebrew() {
    let galley = displayed("נגד R12 = 4.7 kΩ");
    let order =
        ['R', '1', '2', '=', '4', '7', 'k', 'Ω', 'נ'].map(|character| x_of(&galley, character));
    assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
}

#[test]
fn a_signal_name_keeps_its_brackets_inside_arabic() {
    let galley = displayed("الجهد V(out)");
    let order = ['V', '(', 'o', 't', ')', 'ا'].map(|character| x_of(&galley, character));
    assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
}

#[test]
fn wrapped_right_to_left_rows_are_each_read_from_the_right() {
    let text = "שלום עולם טוב מאוד";
    let galley = with_fonts(|fonts| {
        let width = fonts
            .layout_no_wrap("שלום עולם".to_owned(), sans(), Color32::WHITE)
            .size()
            .x;
        layout(
            fonts,
            text,
            TextFormat::simple(sans(), Color32::WHITE),
            TextWrapping::wrap_at_width(width + 1.0),
        )
    });
    assert!(galley.rows.len() > 1, "{:?}", galley.job.text);
    // Each word starts with a letter no other word has. Every row holds
    // whole words in their logical order, read from the right.
    let initials = ['ש', 'ע', 'ט', 'מ'];
    let mut next_word = 0;
    let mut a_row_holds_two_words = false;
    for row in &galley.rows {
        let starts: Vec<(usize, f32)> = initials
            .iter()
            .enumerate()
            .filter_map(|(word, initial)| {
                row.row
                    .glyphs
                    .iter()
                    .find(|glyph| glyph.chr == *initial)
                    .map(|glyph| (word, glyph.pos.x))
            })
            .collect();
        let words: Vec<usize> = starts.iter().map(|(word, _)| *word).collect();
        assert_eq!(
            words,
            (next_word..next_word + words.len()).collect::<Vec<_>>()
        );
        next_word += words.len();
        a_row_holds_two_words |= words.len() > 1;
        assert!(
            starts.windows(2).all(|pair| pair[0].1 > pair[1].1),
            "{starts:?} in {:?}",
            galley.job.text
        );
    }
    assert_eq!(next_word, initials.len());
    assert!(a_row_holds_two_words, "{:?}", galley.job.text);
}

#[test]
fn a_right_to_left_line_cut_short_ends_on_the_left() {
    let text = "שלום עולם טוב מאוד";
    let galley = with_fonts(|fonts| {
        let width = fonts
            .layout_no_wrap("שלום עולם".to_owned(), sans(), Color32::WHITE)
            .size()
            .x;
        layout(
            fonts,
            text,
            TextFormat::simple(sans(), Color32::WHITE),
            TextWrapping::truncate_at_width(width),
        )
    });
    assert_eq!(galley.rows.len(), 1);
    let leftmost = galley.rows[0]
        .row
        .glyphs
        .iter()
        .min_by(|a, b| a.pos.x.total_cmp(&b.pos.x))
        .expect("a glyph");
    assert_eq!(leftmost.chr, '…', "{:?}", galley.job.text);
    assert!(x_of(&galley, 'ש') > x_of(&galley, '…'));
}

/// Paints `text` once and returns the canvas and where the text landed.
fn painted(text: &str) -> (Canvas, Rect) {
    let landed = Cell::new(Rect::NOTHING);
    let canvas = render(vec2(420.0, 90.0), |ui, _| {
        landed.set(paint_text(
            ui.painter(),
            pos2(10.0, 10.0),
            Align2::LEFT_TOP,
            text,
            sans(),
            Color32::WHITE,
        ));
    });
    (canvas, landed.get())
}

fn inked(canvas: &Canvas, x: usize, y: usize) -> bool {
    canvas.pixel(x, y).is_some_and(|pixel| {
        let (pixel, background) = (pixel.to_array(), canvas.background().to_array());
        (0..3).any(|channel| pixel[channel].abs_diff(background[channel]) > 96)
    })
}

/// The stretches of columns in `rect` that carry ink, left to right.
fn ink_columns(canvas: &Canvas, rect: Rect) -> Vec<Range<usize>> {
    let rows = rect.min.y.floor() as usize..rect.max.y.ceil() as usize;
    let mut stretches: Vec<Range<usize>> = Vec::new();
    for x in rect.min.x.floor() as usize..rect.max.x.ceil() as usize {
        if !rows.clone().any(|y| inked(canvas, x, y)) {
            continue;
        }
        match stretches.last_mut() {
            Some(stretch) if stretch.end == x => stretch.end = x + 1,
            _ => stretches.push(x..x + 1),
        }
    }
    stretches
}

/// The first row in `rect` that carries ink.
fn top_ink(canvas: &Canvas, rect: Rect) -> usize {
    let columns = rect.min.x.floor() as usize..rect.max.x.ceil() as usize;
    (rect.min.y.floor() as usize..rect.max.y.ceil() as usize)
        .find(|y| columns.clone().any(|x| inked(canvas, x, *y)))
        .expect("the text has ink")
}

#[test]
fn arabic_letters_join_into_one_stroke() {
    // The same four letters, held apart by zero-width non-joiners.
    let (joined, joined_at) = painted("سلام");
    let (apart, apart_at) = painted("س\u{200C}ل\u{200C}ا\u{200C}م");
    let joined = ink_columns(&joined, joined_at);
    let apart = ink_columns(&apart, apart_at);
    assert!(!joined.is_empty());
    assert!(
        joined.len() < apart.len(),
        "joined {joined:?} against apart {apart:?}"
    );
    // Seen and lam join, and lam-alef is one ligature: one stroke, then meem.
    assert_eq!(joined.len(), 2, "{joined:?}");
}

#[test]
fn a_thai_tone_mark_climbs_over_a_vowel_above() {
    // The tone mark sits on the consonant alone, and above the vowel when
    // one is there: placed by the face's mark positioning, not stacked on
    // one height.
    let (bare, bare_at) = painted("ก่");
    let (stacked, stacked_at) = painted("กิ่");
    let bare_top = top_ink(&bare, bare_at);
    let stacked_top = top_ink(&stacked, stacked_at);
    assert!(
        stacked_top + 3 < bare_top,
        "top ink at {stacked_top} with the vowel, {bare_top} without"
    );
    // Both marks stand over the consonant rather than after it.
    let (base, base_at) = painted("ก");
    let width = |canvas: &Canvas, rect: Rect| {
        let columns = ink_columns(canvas, rect);
        columns.last().map_or(0, |last| last.end) - columns.first().map_or(0, |first| first.start)
    };
    assert!(width(&stacked, stacked_at) <= width(&base, base_at) + 2);
}

#[test]
fn a_bracket_in_hebrew_is_drawn_facing_its_text() {
    // Displayed from the left, "(שלום)" starts with the closing bracket,
    // which must be drawn as an opening one.
    let galley = displayed("(שלום)");
    let leftmost = galley.rows[0]
        .row
        .glyphs
        .iter()
        .min_by(|a, b| a.pos.x.total_cmp(&b.pos.x))
        .expect("a glyph");
    assert_eq!(leftmost.chr, '(');

    let (canvas, at) = painted("(שלום)");
    let bracket = ink_columns(&canvas, at)[0].clone();
    let rows = at.min.y as usize..at.max.y as usize;
    let inked_rows: Vec<usize> = rows
        .filter(|y| bracket.clone().any(|x| inked(&canvas, x, *y)))
        .collect();
    let (top, bottom) = (inked_rows[0], inked_rows[inked_rows.len() - 1]);
    let leftmost_ink = |y: usize| bracket.clone().find(|x| inked(&canvas, *x, y));
    let middle = leftmost_ink((top + bottom) / 2).expect("ink mid-height");
    let tip = leftmost_ink(top + 1).expect("ink at the top");
    assert!(
        middle < tip,
        "an opening bracket bows left: middle {middle}, tip {tip}"
    );
}

/// The canvas sets each inked glyph of a line where print does, so a sheet
/// exported as PDF or SVG, or rasterised from the SVG, reads as the canvas
/// shows it: the same faces, the same shaping and the same order. Print
/// and the SVG agree glyph for glyph in the hardcopy tests; this holds the
/// canvas to print.
#[test]
fn the_canvas_sets_every_glyph_where_print_does() {
    const SIZE: f32 = 100.0;
    let mut lines = vec![
        (egui::FontFamily::Proportional, "AVATAR To Wa"),
        (crate::fonts::mono(), "R1 10 kΩ α ⌀6 ⏚"),
        (crate::fonts::sans_semibold(), "DWG Δ 4.7 µF"),
    ];
    #[cfg(not(target_arch = "wasm32"))]
    lines.extend([
        (egui::FontFamily::Proportional, "מגבר R1 = 10 kΩ (V+)"),
        (egui::FontFamily::Proportional, "לא (גד) 12 שלום"),
        (egui::FontFamily::Proportional, "لا إله قيمة 123"),
        (crate::fonts::sans_semibold(), "مَرْحَبًا R2"),
        (egui::FontFamily::Proportional, "שָׁלוֹם R3"),
        (egui::FontFamily::Proportional, "ภาษาไทย น้ำ"),
    ]);
    for (family, text) in lines {
        let galley = with_fonts(|fonts| {
            layout_no_wrap(
                fonts,
                text,
                FontId::new(SIZE, family.clone()),
                Color32::WHITE,
            )
        });
        let mut canvas: Vec<f32> = galley
            .rows
            .iter()
            .flat_map(|row| {
                row.row
                    .glyphs
                    .iter()
                    .filter(|glyph| !glyph.chr.is_whitespace() && !glyph.uv_rect.is_nothing())
                    .map(move |glyph| row.pos.x + glyph.pos.x)
            })
            .collect();
        let mut printed = Vec::new();
        let mut left = 0.0;
        for piece in rspice_fonts::print::line(role_of(&family), text) {
            let scale = SIZE / f32::from(piece.units_per_em);
            for glyph in &piece.glyphs {
                if !text[glyph.text.clone()].trim().is_empty() {
                    printed.push(left + glyph.x_offset as f32 * scale);
                }
                left += glyph.advance as f32 * scale;
            }
        }
        assert_eq!(
            canvas.len(),
            printed.len(),
            "{text:?}: {canvas:?} {printed:?}"
        );
        for glyphs in [&mut canvas, &mut printed] {
            glyphs.sort_by(f32::total_cmp);
        }
        let (from_canvas, from_print) = (canvas[0], printed[0]);
        for (on_canvas, in_print) in canvas.iter().zip(&printed) {
            assert!(
                ((on_canvas - from_canvas) - (in_print - from_print)).abs() < SIZE * 0.01,
                "{text:?}: a glyph at {} on the canvas, where print sets one at {}",
                on_canvas - from_canvas,
                in_print - from_print
            );
        }
    }
}
