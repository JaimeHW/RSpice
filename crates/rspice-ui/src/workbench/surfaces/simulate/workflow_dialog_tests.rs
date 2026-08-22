//! Tests for the simulation studio's workflow dialogs.
//!
//! A workflow dialog is a plan edit with a control that commits it, so what
//! these pin is when that control is live: an edit the plan would refuse, or
//! one that would change nothing, must not be offered as if it would.

use super::*;

/// The rename dialog opens on the name the analysis already has, and refuses
/// to commit it.
///
/// Applying that no-op advances the plan revision and stales every pinned
/// preflight artifact -- which is exactly what the dialog's own Effect row
/// promises -- for no change at all. The control that would do it stays
/// disabled until the field says something different.
///
/// Read from the accessibility tree, because "disabled" is what the control
/// publishes to a reader who is not looking at its fill.
#[test]
fn the_rename_dialog_refuses_a_rename_that_renames_nothing() {
    fn primary_is_disabled(name: &str) -> bool {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        let instance = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the test instance owns a stable plan")
            .instances()
            .first()
            .expect("a plan holds at least one instance")
            .id();
        let shown_as = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("plan")
            .instance(instance)
            .expect("the instance the plan just named")
            .display_name()
            .to_owned();
        let mut draft =
            crate::workbench::state::RenameAnalysisDraft::for_instance(instance, "", &shown_as);
        draft.name = name.to_owned();
        app.state.workbench.simulation_workflow =
            Some(SimulationWorkflowDialog::RenameAnalysis(draft));

        let mut run = || {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_200.0, 800.0))),
                    ..egui::RawInput::default()
                },
                |ctx| super::show_workflow_dialogs(ctx, &mut app),
            )
        };
        // A content-height surface lays out against its previous measurement.
        let _ = run();
        let output = run();

        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("AccessKit tree update")
            .nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button
                    && node.label().is_some_and(|label| label.contains("Apply"))
            })
            .map(|(_, node)| node.is_disabled())
            .expect("the dialog publishes its primary control")
    }

    let shown_as = RSpiceApp::test_instance()
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("plan")
        .instances()
        .first()
        .expect("a plan holds at least one instance")
        .display_name()
        .to_owned();

    assert!(
        primary_is_disabled(&shown_as),
        "committing the name the analysis already has must be refused"
    );
    // Trailing space and all: the field is trimmed before it is compared, so
    // whitespace is not a change either.
    assert!(
        primary_is_disabled(&format!("  {shown_as}  ")),
        "whitespace around the same name is still not a rename"
    );
    assert!(
        !primary_is_disabled("Startup transient"),
        "a name the analysis does not have must be committable"
    );
}
