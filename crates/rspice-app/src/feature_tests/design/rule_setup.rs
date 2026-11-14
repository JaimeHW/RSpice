//! The rule setup dialog drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::rule_setup::*;
use crate::workbench::app_state::AppState;
use egui::Context;

/// Draw the dialog as the frame's step does, over the whole state.
fn render_rule_setup_dialog(ctx: &Context, state: &mut AppState) -> bool {
    let review_markers = state.design.schematic_visibility.review_markers;
    crate::workbench::app::dialogs::rule_setup::render_rule_setup_dialog(
        &mut state.design.rule_setup,
        review_markers,
        &mut state.model,
        &mut state.runs,
        ctx,
    )
}

/// The table is inside the dialog its header and footer bound: every
/// cell ends left of the footer's last button. On the manager size the
/// rows ran about 180 points past the header and the footer, which
/// stopped at the size's width while the surface grew under the table.
/// Apply, unavailable on an unchanged deck, says why in the footer.
#[test]
fn the_rule_table_stays_inside_its_dialog() {
    let mut app = RSpiceApp::test_instance();
    open_rule_setup(
        &mut app.state.design.rule_setup,
        &app.state.model,
        &mut app.state.workbench,
        &app.state.platform,
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut shapes = Vec::new();
    for _ in 0..4 {
        shapes = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    let _ = render_rule_setup_dialog(ui.ctx(), &mut app.state);
                },
            )
            .shapes;
    }
    let texts = shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some((
                text.galley.text().to_owned(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let at = |wanted: &str| {
        texts
            .iter()
            .find(|(text, _)| text == wanted)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("{wanted:?} was not painted"))
    };
    let footer_end = at(PRIMARY).right();
    for wanted in ["Lowest allowed", "DRC-OFF-SHEET-CONNECTOR-WITHOUT-PARTNER"] {
        assert!(
            at(wanted).right() < footer_end,
            "{wanted:?} ends at {}, past the footer's {footer_end}",
            at(wanted).right()
        );
    }
    assert!(texts.iter().all(|(text, _)| text != "Floor"));
    let _ = at(UNCHANGED);
    // Every project select shows its whole choice, and the inheriting
    // one does not restate the Default column beside it.
    let _ = at(INHERIT_DEFAULT);
    for (text, _) in &texts {
        assert!(
            !text.contains('\u{2026}') && !text.starts_with("Default ("),
            "a severity reads {text:?}"
        );
    }
}
