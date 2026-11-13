//! How an object row of a whole-design rail shares its width between the
//! instance it names and the reading beside it.
//!
//! The reference is the row's identity, so it is painted whole; the reading is
//! the row's detail, so it takes the room that is left and is marked when cut.

use super::*;

/// An object row is named by its reference, so the reference reads whole and a
/// reading too long for the rail elides beside it. In the rail the Verilog-A
/// flow draws, 255 px wide, the 1 V source's row read `V V · DC · 1V ·
/// transient drive`: the reading took the room and clipped `V1` to its first
/// letter, with no mark that anything was missing.
#[test]
fn an_object_rows_reference_reads_whole_beside_a_reading_too_long_for_the_rail() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let reading = "V \u{00b7} DC \u{00b7} 1V \u{00b7} transient drive";
    // A child of exactly the rail's width. A central panel's `Ui` already
    // spans the whole panel when its contents run, and `set_width` cannot
    // shrink a `Ui` below what it spans, so a row drawn there is as wide as
    // the screen and nothing in it is ever cut.
    let output = ctx.run_ui(Default::default(), |ui| {
        ui.allocate_ui(egui::vec2(255.0, SCHEMATIC_NAV_ROW_HEIGHT), |ui| {
            occurrence_object_row(
                ui,
                OccurrenceObjectRow {
                    icon: WorkbenchIcon::ArrowRight,
                    occurrence: None,
                    reference: "V1",
                    meta: reading,
                    alert: false,
                    selected: false,
                },
            )
        });
    });
    let runs = output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::epaint::Shape::Text(text) => Some((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
                clipped.clip_rect,
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let (_, name, clip) = runs
        .iter()
        .find(|(text, ..)| text == "V1")
        .expect("the reference is painted");
    assert!(
        clip.contains_rect(*name),
        "the reference at {name:?} is painted whole, not clipped to {clip:?}"
    );
    let (shown, meta, _) = runs
        .iter()
        .find(|(text, ..)| text.starts_with("V \u{00b7}"))
        .expect("the reading is painted");
    assert!(
        meta.left() > name.right(),
        "the reading {shown:?} at {meta:?} starts clear of the reference at {name:?}"
    );
    assert!(
        shown == reading || shown.ends_with('\u{2026}'),
        "the reading is whole or marked as cut: {shown:?}"
    );
}
