//! Descending into a hierarchy block, drawn through a whole application.

use crate::feature_tests::design::hierarchy_descend::hierarchy_state;
use crate::workbench::app::RSpiceApp;
use egui::Context;
use rspice_app_design::workbench::app::dialogs::hierarchy::descend::*;

/// Every text the descent form paints over `app`, after three frames.
fn painted_texts(app: &mut RSpiceApp) -> Vec<String> {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut shapes = Vec::new();
    for _ in 0..3 {
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
                    let state = &mut app.state;
                    render_descend_hierarchy_dialog(
                        &mut state.design.descend_hierarchy,
                        &state.model,
                        &mut state.workbench,
                        ui.ctx(),
                    );
                },
            )
            .shapes;
    }
    shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect()
}

/// The Cell view row names the view the chosen mode opens. Edited in place,
/// that is the instance's symbol, not the schematic a descent would open,
/// and where the cell has no symbol to edit the row says so before the form
/// is committed.
#[test]
fn the_cell_view_row_names_the_symbol_the_in_place_mode_edits() {
    let (mut state, _, _) = hierarchy_state();
    assert!(open_descend_hierarchy_dialog(
        &mut state.design.descend_hierarchy,
        &state.model,
        &mut state.workbench,
    ));
    let mut app = RSpiceApp::test_instance();
    app.state = state;
    assert!(painted_texts(&mut app).contains(&"afe_core \u{00b7} schematic".to_owned()));

    app.state.design.descend_hierarchy.edit_mode = HierarchyDescendEditMode::EditSymbolInPlace;
    let refused = painted_texts(&mut app);
    assert!(
        refused.contains(&"The cell has no symbol view".to_owned()),
        "{refused:?}"
    );
    assert!(!refused.contains(&"afe_core \u{00b7} schematic".to_owned()));

    app.state
        .model
        .library_manager
        .get_library_mut("hierarchy_test")
        .and_then(|library| library.get_cell_mut("afe_core"))
        .expect("the instance's cell")
        .add_view(rspice_app_state::state::View::new(
            "symbol",
            rspice_app_state::state::ViewType::Symbol,
        ));
    let named = painted_texts(&mut app);
    assert!(
        named.contains(&"afe_core \u{00b7} symbol".to_owned()),
        "{named:?}"
    );
}
