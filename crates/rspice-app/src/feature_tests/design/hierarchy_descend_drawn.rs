//! Descending into a hierarchy block, drawn through a whole application.

use crate::feature_tests::design::hierarchy_descend::hierarchy_state;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hierarchy::descend::*;
use egui::Context;

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

/// The form states each fact once, in its one label style, with every
/// value in the kit's 13 point face: mono for the instance and its cell
/// view, the body face for a choice named in words. Each row was a bold
/// title over a helper sentence that restated it, beside a mono select
/// and a small-bold value.
#[test]
fn the_form_has_one_label_style_and_values_in_the_kit_face() {
    let (mut state, _, _) = hierarchy_state();
    assert!(open_descend_hierarchy_dialog(
        &mut state.design.descend_hierarchy,
        &state.model,
        &mut state.workbench,
    ));
    state.design.descend_hierarchy.parent_dirty = true;
    let mut app = RSpiceApp::test_instance();
    app.state = state;
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
    let painted = shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some((
                text.galley.text().to_owned(),
                text.galley.job.sections[0].format.font_id.clone(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let font = |wanted: &str| {
        painted
            .iter()
            .find(|(text, _)| text == wanted)
            .map(|(_, font)| font.clone())
            .unwrap_or_else(|| panic!("{wanted:?} is not painted: {painted:?}"))
    };
    let label = font("Instance");
    for other in ["Cell view", "Edit mode", "Unsaved parent changes"] {
        assert_eq!(font(other), label, "{other} is not in the label style");
    }
    assert_eq!(font("/XAFE"), rspice_app_workbench::ui::theme::field_mono());
    assert_eq!(
        font("Edit in place"),
        rspice_app_workbench::ui::theme::field_sans()
    );
    assert_eq!(
        font("Kept as a working checkpoint"),
        rspice_app_workbench::ui::theme::field_sans()
    );
    for (text, _) in &painted {
        for gone in [
            "Selected hierarchical",
            "Controls who owns",
            "transactional",
        ] {
            assert!(!text.contains(gone), "{text:?} is painted");
        }
    }
}
