//! The visibility options dialog drawn through a whole application.

use crate::state::SchematicParameterLabelVisibility;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::visibility_options::tests::render;
use crate::workbench::app::dialogs::visibility_options::*;
use crate::workbench::app_state::AppState;
use egui::Context;

fn apply_display_options(state: &mut AppState) {
    state
        .design
        .apply_display_options(&mut state.model, &mut state.workbench);
}

/// The visibility and the drawing-sheet-layers commands open one
/// dialog, which applies both together and offers no wire routing: that
/// is set in Grid, snap and routing alone.
#[test]
fn one_display_options_dialog_owns_visibility_and_sheet_layers() {
    let mut app = RSpiceApp::test_instance();
    assert!(open_display_options(
        &mut app.state.design.schematic_visibility_dialog,
        app.state.design.schematic_visibility,
        app.state.design.drawing_sheet_layers
    ));
    assert!(
        !open_schematic_visibility_options(
            &mut app.state.design.schematic_visibility_dialog,
            app.state.design.schematic_visibility,
            app.state.design.drawing_sheet_layers
        ),
        "both commands open the same dialog"
    );

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    // A later pass publishes only the nodes that changed, so the tree is
    // what every pass published.
    let mut labels = String::new();
    for _ in 0..2 {
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            render(&mut app, ui.ctx());
        });
        if let Some(update) = output.platform_output.accesskit_update {
            for text in update
                .nodes
                .iter()
                .flat_map(|(_, node)| [node.label(), node.value()])
                .flatten()
            {
                labels.push_str(text);
                labels.push('\n');
            }
        }
    }
    for row in [
        "Annotations",
        "Net highlighting",
        "Zone references",
        "Title block",
    ] {
        assert!(labels.contains(row), "{row} is missing:\n{labels}");
    }
    assert!(
        !labels.contains("Wire routing"),
        "wire routing is not a display option:\n{labels}"
    );

    app.state
        .design
        .schematic_visibility_dialog
        .layers_draft
        .zones = false;
    app.state
        .design
        .schematic_visibility_dialog
        .draft
        .parameter_labels = SchematicParameterLabelVisibility::Hidden;
    apply_display_options(&mut app.state);
    assert!(!app.state.design.drawing_sheet_layers.zones);
    assert_eq!(
        app.state.design.schematic_visibility.parameter_labels,
        SchematicParameterLabelVisibility::Hidden
    );
    assert!(!app.state.design.schematic_visibility_dialog.open);
}
