//! Find and replace drawn and applied through a whole application.

use crate::state::Point;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::find_replace::tests::{ask, part, replace_all};
use crate::workbench::app::dialogs::find_replace::*;
use crate::workbench::commands::vocabulary::Command;
use egui::Context;

/// Each option row is one line, its checks in the columns the scope's
/// segments stand in, and every row of the form is one pitch from the
/// next.
#[test]
fn the_options_stand_on_one_grid() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.design.find_replace.open = true;
    let mut render = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            },
            |ui| app.render_frame_dialogs(ui.ctx()),
        )
    };
    let _ = render();
    let nodes = render()
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;
    let rect = |label: &str| {
        let bounds = nodes
            .iter()
            .map(|(_, node)| node)
            .find(|node| {
                node.label() == Some(label)
                    && matches!(
                        node.role(),
                        egui::accesskit::Role::TextInput
                            | egui::accesskit::Role::CheckBox
                            | egui::accesskit::Role::RadioButton
                    )
            })
            .and_then(|node| node.bounds())
            .unwrap_or_else(|| panic!("{label} is not published with its bounds"));
        egui::Rect::from_min_max(
            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
        )
    };
    let rows = [
        vec!["Find"],
        vec!["Replace"],
        vec!["Instance names", "Net labels", "Values"],
        SCOPES.to_vec(),
        vec!["Whole word", "Match case", "Regular expression"],
    ];
    let centres: Vec<f32> = rows
        .iter()
        .map(|row| {
            let centre = rect(row[0]).center().y;
            for item in row {
                assert!(
                    (rect(item).center().y - centre).abs() <= 0.5,
                    "{item} is not on the line {} is on",
                    row[0]
                );
            }
            centre
        })
        .collect();
    let pitch = centres[1] - centres[0];
    for pair in centres.windows(2) {
        assert!(
            (pair[1] - pair[0] - pitch).abs() <= 0.5,
            "the rows step {centres:?}, not one pitch"
        );
    }
    for column in 0..3 {
        let left = rect(SCOPES[column]).left();
        for row in [&rows[2], &rows[4]] {
            assert!(
                (rect(row[column]).left() - left).abs() <= 0.5,
                "{} does not start where {} does",
                row[column],
                SCOPES[column]
            );
        }
    }
    let (find, scope) = (rect("Find"), rect(SCOPES[2]));
    assert!(
        (scope.right() - find.right()).abs() <= 0.5,
        "the scope does not span the field column"
    );
}

/// An instance renamed by Replace all ends as the Rename form leaves
/// it: the probe and the saved output that name it follow it, and one
/// Undo takes the whole change back.
#[test]
fn replace_all_renames_an_instance_as_the_rename_form_does() {
    fn fixture() -> (RSpiceApp, crate::product::SimulationPlanId) {
        let mut app = RSpiceApp::test_instance();
        let document = app.state.model.schematic.document_mut_for_test();
        document.components = vec![part(1, "R1", "1k"), part(2, "R2", "1k")];
        document.wires.clear();
        document.probes = vec![
            crate::state::SchematicProbe::new(
                30,
                Point::new(100, 40),
                "I(R1)",
                Some("I(R1)".to_owned()),
            )
            .expect("a current probe"),
        ];
        app.state.model.schematic.init_undo_history();
        let plan_id = crate::product::SimulationPlanId::new();
        app.state
            .model
            .workspace
            .content
            .simulation_plan_payloads
            .push(crate::state::SimulationPlanPayloadRecord {
                plan_id,
                payload: crate::state::SimulationPlanPayload {
                    saved_outputs: vec![
                        crate::state::SavedOutput::new(
                            crate::state::SavedOutputKind::RawVoltageOrCurrent,
                            "load current",
                            "I(R1)",
                            crate::state::SavedOutputCompatibility::OpTranAc,
                            crate::state::SavedOutputPolicy::EveryAcceptedPoint,
                            crate::state::SavedOutputPrecision::FullSourcePrecision,
                            crate::state::SavedOutputStreaming::StoreOnly,
                        )
                        .expect("a raw current is a saved output"),
                    ],
                    ..Default::default()
                },
            });
        (app, plan_id)
    }
    fn outcome(
        app: &RSpiceApp,
        plan_id: crate::product::SimulationPlanId,
    ) -> (Vec<String>, Vec<Option<String>>, String) {
        let document = app.state.model.schematic.document();
        (
            document
                .components
                .iter()
                .map(|component| component.name.clone())
                .collect(),
            document
                .probes
                .iter()
                .map(|probe| probe.source_expression.clone())
                .collect(),
            app.state
                .model
                .workspace
                .content
                .plan_data(plan_id)
                .expect("the plan this test owns")
                .saved_outputs[0]
                .source_expression
                .clone(),
        )
    }

    let (mut by_form, form_plan) = fixture();
    let expected = by_form.state.model.schematic.document().components[0].clone();
    assert_eq!(
        by_form
            .state
            .model
            .rename_component_transaction(&expected, "R10".to_owned()),
        Ok(true)
    );
    let (mut by_replace, replace_plan) = fixture();
    ask(&mut by_replace.state, "R1", "R10", FindScope::CellView);
    assert_eq!(replace_all(&mut by_replace.state), Ok((1, 1)));

    let renamed = (
        vec!["R10".to_owned(), "R2".to_owned()],
        vec![Some("I(R10)".to_owned())],
        "I(R10)".to_owned(),
    );
    assert_eq!(outcome(&by_form, form_plan), renamed, "the Rename form");
    assert_eq!(outcome(&by_replace, replace_plan), renamed, "Replace all");
    by_replace.execute_command_key(Command::Undo.key());
    assert_eq!(
        outcome(&by_replace, replace_plan),
        (
            vec!["R1".to_owned(), "R2".to_owned()],
            vec![Some("I(R1)".to_owned())],
            "I(R1)".to_owned(),
        ),
        "one Undo takes the rename and its references back"
    );
}
