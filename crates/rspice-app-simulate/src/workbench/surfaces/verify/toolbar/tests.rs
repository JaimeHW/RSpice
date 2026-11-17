//! The Verify toolbar at desktop, tablet and phone widths: nothing on its
//! row overlaps, the summary elides between the title and the actions, and
//! the actions that do not fit fold behind More in order.

use egui::{Event, Modifiers, PointerButton, Pos2, Rect, pos2, vec2};

use super::*;
use crate::workbench::surfaces::verify::raster::{PageGeometry, page_pass};

/// The toolbar sits right of a navigator, so its own left edge is not the
/// screen's.
const LEFT: f32 = 307.0;

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

fn click(pos: Pos2) -> Vec<Event> {
    let edge = |pressed| Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    vec![Event::PointerMoved(pos), edge(true), edge(false)]
}

fn runs() -> RunChoice {
    RunChoice {
        candidates: vec![(0, "Run 7".to_owned()), (1, "Run 8 (reference)".to_owned())],
        selected: Some(0),
        role: RunRole::Read,
    }
}

fn actions() -> Vec<ToolbarAction<'static>> {
    vec![
        ToolbarAction::new("Add corner"),
        ToolbarAction::new("Export results"),
        ToolbarAction::new("Open in Results"),
        ToolbarAction::new("Set as reference")
            .enabled(false)
            .disabled_reason("This run is the reference"),
        ToolbarAction::new("Show passing").toggled(true),
    ]
}

const SUMMARY: &str = "3 of 12 corners fail · worst margin −42 mV at SS, 1.62 V, 125 °C";

/// One frame of the Corners toolbar, `width` wide, over `events`.
fn bar(ctx: &egui::Context, width: f32, events: Vec<Event>) -> (ToolbarOutput, PageGeometry) {
    let runs = runs();
    let actions = actions();
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(LEFT + width, 600.0))),
        events,
        ..Default::default()
    };
    let mut output = ToolbarOutput::default();
    let (_, geometry) = page_pass(ctx, input, |ui| {
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(Rect::from_min_size(pos2(LEFT, 0.0), vec2(width, 600.0)))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        output = PageToolbar {
            title: "Corners",
            runs: &runs,
            summary: Some(Summary::failing(SUMMARY)),
            actions: &actions,
        }
        .show(&mut child);
    });
    (output, geometry)
}

/// Open the More menu at `width` and read where its rows went.
fn open_more(ctx: &egui::Context, width: f32) -> PageGeometry {
    let _ = bar(ctx, width, Vec::new());
    let (_, geometry) = bar(ctx, width, Vec::new());
    let more = geometry.one("toolbar.more").expect("More is on the row");
    let _ = bar(ctx, width, click(more.center()));
    bar(ctx, width, Vec::new()).1
}

#[test]
fn the_more_menu_lists_the_folded_actions_in_order_and_checks_the_latched_one() {
    for width in [820.0, 560.0] {
        let ctx = context();
        let geometry = open_more(&ctx, width);
        let standing = geometry.all("toolbar.action").len();
        let rows = (0..5)
            .map(|index| geometry.one(&format!("toolbar.overflow.{index}")))
            .collect::<Vec<_>>();
        for (index, row) in rows.iter().enumerate() {
            assert_eq!(
                row.is_some(),
                index >= standing,
                "action {index} is listed only when folded at {width}"
            );
        }
        let tops = rows
            .iter()
            .flatten()
            .map(|row| row.top())
            .collect::<Vec<_>>();
        assert!(
            tops.windows(2).all(|pair| pair[0] < pair[1]),
            "the menu keeps the row's order at {width}: {tops:?}"
        );
        let check = geometry
            .one("toolbar.overflow.check")
            .expect("a check mark");
        let latched = rows[4].expect("Show passing is folded");
        assert!(
            latched.contains_rect(check),
            "the check is on Show passing's row"
        );
        assert_eq!(geometry.all("toolbar.overflow.check").len(), 1);
    }
}

#[test]
fn a_folded_action_is_taken_from_the_menu_unless_it_is_disabled() {
    let ctx = context();
    let geometry = open_more(&ctx, 560.0);
    let disabled = geometry.one("toolbar.overflow.3").expect("row");
    let (output, _) = bar(&ctx, 560.0, click(disabled.center()));
    assert_eq!(output.action, None, "a disabled action is not taken");

    let geometry = bar(&ctx, 560.0, Vec::new()).1;
    let latched = match geometry.one("toolbar.overflow.4") {
        Some(row) => row,
        None => open_more(&ctx, 560.0)
            .one("toolbar.overflow.4")
            .expect("row"),
    };
    let (output, _) = bar(&ctx, 560.0, click(latched.center()));
    assert_eq!(output.action, Some(4));
    let (_, geometry) = bar(&ctx, 560.0, Vec::new());
    assert!(
        geometry.one("toolbar.overflow.4").is_none(),
        "taking an action closes the menu"
    );
}
