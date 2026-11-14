//! Descending into a hierarchy block, drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hierarchy::descend::tests::hierarchy_state;
use crate::workbench::app::dialogs::hierarchy::descend::*;
use egui::Context;

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
    crate::ui::Theme::default().apply(&ctx);
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
    assert_eq!(font("/XAFE"), crate::ui::theme::field_mono());
    assert_eq!(font("Edit in place"), crate::ui::theme::field_sans());
    assert_eq!(
        font("Kept as a working checkpoint"),
        crate::ui::theme::field_sans()
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
