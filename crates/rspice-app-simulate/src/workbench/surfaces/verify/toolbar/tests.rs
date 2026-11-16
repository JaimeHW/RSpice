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

/// Every part of the row, by name, left to right.
fn parts(geometry: &PageGeometry) -> Vec<(String, Rect)> {
    let mut parts = [
        "toolbar.title",
        "toolbar.run",
        "toolbar.summary",
        "toolbar.more",
    ]
    .into_iter()
    .flat_map(|key| {
        geometry
            .all(key)
            .into_iter()
            .map(move |rect| (key.to_owned(), rect))
    })
    .chain(
        geometry
            .all("toolbar.action")
            .into_iter()
            .map(|rect| ("toolbar.action".to_owned(), rect)),
    )
    .collect::<Vec<_>>();
    parts.sort_by(|left, right| left.1.left().total_cmp(&right.1.left()));
    parts
}

#[test]
fn nothing_on_the_row_overlaps_at_desktop_tablet_or_phone_width() {
    let ctx = context();
    let mut standing = Vec::new();
    for width in [1100.0, 820.0, 560.0] {
        let _ = bar(&ctx, width, Vec::new());
        let (_, geometry) = bar(&ctx, width, Vec::new());
        let parts = parts(&geometry);
        let title = geometry.one("toolbar.title").expect("a title");
        assert_eq!(title.left(), LEFT + TABLE_EDGE_INSET, "at {width}");
        for (index, (name, rect)) in parts.iter().enumerate() {
            assert!(
                rect.left() >= LEFT + TABLE_EDGE_INSET - 0.5
                    && rect.right() <= LEFT + width - TABLE_EDGE_INSET + 0.5,
                "{name} {rect:?} leaves the {width}-point row"
            );
            for (other, beside) in &parts[index + 1..] {
                assert!(
                    !rect.intersects(*beside),
                    "{name} {rect:?} overlaps {other} {beside:?} at {width}"
                );
            }
        }
        let actions = geometry.all("toolbar.action").len();
        let more = geometry.one("toolbar.more");
        if let Some(more) = more {
            assert_eq!(
                parts.last().map(|(name, _)| name.as_str()),
                Some("toolbar.more"),
                "More ends the row at {width}"
            );
            assert!(more.right() <= LEFT + width - TABLE_EDGE_INSET + 0.5);
        }
        assert_eq!(
            more.is_some(),
            actions < 5,
            "More shows only over folded actions"
        );
        standing.push(actions);
    }
    assert_eq!(standing[0], 5, "every action stands at the desktop width");
    assert!(standing[1] < 5, "a tablet folds some: {standing:?}");
    assert!(
        standing[2] <= standing[1],
        "a phone folds as many: {standing:?}"
    );
}

#[test]
fn the_desktop_row_holds_the_summary_on_one_line() {
    let ctx = context();
    let _ = bar(&ctx, 1100.0, Vec::new());
    let (_, geometry) = bar(&ctx, 1100.0, Vec::new());
    let toolbar = geometry.one("toolbar").expect("toolbar");
    assert_eq!(toolbar.height(), TOOLBAR_H);
    let summary = geometry.one("toolbar.summary").expect("summary");
    assert!(summary.top() >= toolbar.top() && summary.bottom() <= toolbar.bottom());
    let run = geometry.one("toolbar.run").expect("a run selector");
    assert!(summary.left() >= run.right() + GROUP_GAP - 0.5);
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

fn wants<'w>(row: f32, title: f32, summary: Option<f32>, actions: &'w [f32]) -> Wants<'w> {
    Wants {
        row,
        title,
        selector: false,
        summary,
        actions,
        more: 60.0,
    }
}

#[test]
fn the_row_folds_trailing_actions_before_it_drops_the_summary_or_elides_the_title() {
    let actions = [110.0, 100.0, 120.0, 90.0];
    let all = GROUP_GAP + 420.0 + 3.0 * ACTION_GAP;
    let roomy = plan(&wants(1100.0, 70.0, Some(400.0), &actions));
    assert_eq!(
        roomy,
        Plan {
            title: 70.0,
            shown: 4,
            actions: all,
            summary_inline: true,
        }
    );
    // 540 points of room: the title, 156 for the summary, two actions and
    // More.
    let tablet = plan(&wants(560.0, 70.0, Some(400.0), &actions));
    assert_eq!(tablet.shown, 2);
    assert_eq!(tablet.actions, GROUP_GAP + 210.0 + 60.0 + 2.0 * ACTION_GAP);
    assert!(tablet.summary_inline);
    // A short summary keeps only its own width, which leaves room for a
    // third action.
    assert_eq!(plan(&wants(580.0, 70.0, Some(400.0), &actions)).shown, 2);
    assert_eq!(plan(&wants(580.0, 70.0, Some(40.0), &actions)).shown, 3);

    // A long title leaves no room for the summary beside More: the summary
    // takes its own line and the actions refit.
    let crowded = plan(&wants(400.0, 200.0, Some(400.0), &actions));
    assert_eq!((crowded.shown, crowded.summary_inline), (0, false));
    assert_eq!(crowded.title, 200.0);

    // Not even More fits beside the title: the title elides.
    let narrow = plan(&wants(200.0, 200.0, Some(400.0), &actions));
    assert_eq!(narrow.title, 180.0 - (GROUP_GAP + 60.0));
    assert_eq!(narrow.shown, 0);

    let mut with_selector = wants(1100.0, 70.0, Some(400.0), &actions);
    with_selector.selector = true;
    assert_eq!(plan(&with_selector).shown, 4);
    with_selector.row = 820.0;
    assert!(plan(&with_selector).shown < 4);

    let bare = plan(&wants(300.0, 70.0, None, &[]));
    assert_eq!(
        bare,
        Plan {
            title: 70.0,
            shown: 0,
            actions: 0.0,
            summary_inline: true,
        }
    );
}

#[test]
fn one_folded_action_narrower_than_more_stands_instead() {
    // Showing the last action costs no more than More would.
    let actions = [110.0, 40.0];
    let tight = GROUP_GAP + 150.0 + ACTION_GAP;
    let room = 2.0 * TABLE_EDGE_INSET + 70.0 + tight;
    assert_eq!(plan(&wants(room, 70.0, None, &actions)).shown, 2);
}
