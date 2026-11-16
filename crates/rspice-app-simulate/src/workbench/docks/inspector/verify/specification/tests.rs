//! The Specifications view's inspector: each property of the selected
//! specification commits alone, as one plan change, and with nothing selected
//! it states the plan's acceptance settings and the run being judged.

use rspice_results::specification::SpecificationComparison;

use super::*;
use crate::product::ObjectRevision;
use crate::state::{SpecEntry, VerificationPage};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::test_host::SimulateTestHost;
use crate::workbench::state::Workspace;

type Harness = FeatureHarness<SimulateFeature>;

fn revision(harness: &Harness) -> ObjectRevision {
    harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the harness plan is stable")
        .revision()
}

fn harness_with_gain() -> Harness {
    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the harness plan is stable")
        .id();
    harness.model.workspace.content.replace_active_specs(
        plan_id,
        vec![SpecEntry {
            measurement: "gain".to_owned(),
            expression: String::new(),
            min: Some(40.0),
            max: None,
            unit: "dB".to_owned(),
            scope: SpecPointScope::AllPoints,
        }],
    );
    harness.wb.activate(Workspace::Verify);
    harness.state.verification_page = VerificationPage::Specifications;
    harness
}

/// Every word one pass of the inspector painted.
fn inspector_words(harness: &mut Harness) -> Vec<String> {
    fn walk(shape: &egui::Shape, words: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => words.push(text.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, words)),
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 1200.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                super::super::verify(ui, &mut harness.cx());
            });
        },
    );
    let mut words = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut words);
    }
    words
}

/// Change the selected specification's draft as `edit` does and commit it,
/// as the inspector does when a field lets go of the keyboard; then commit
/// the same draft again, which changes nothing.
fn commit_edit(harness: &mut Harness, property: &str, edit: impl FnOnce(&mut SpecDraft)) {
    let measurement = harness
        .state
        .selected_specification
        .clone()
        .expect("a selected specification");
    let definition = plan_definition(&harness.model, &measurement).expect("its definition");
    let mut draft = SpecDraft::from_definition(&definition);
    edit(&mut draft);
    let before = revision(harness);
    harness.state.verification.specification_draft = Some(SpecificationInspectorDraft {
        specification: Some(definition.id),
        revision: Some(before),
        draft,
    });
    commit_draft(&mut harness.cx(), &definition, property);
    assert_eq!(
        harness.state.verification.action_problem, None,
        "{property} commits"
    );
    assert_eq!(
        revision(harness),
        before.next().expect("a next revision"),
        "{property} is one plan change"
    );
    let measurement = harness
        .state
        .selected_specification
        .clone()
        .expect("the specification stays selected");
    let committed = plan_definition(&harness.model, &measurement).expect("its definition");
    let after = revision(harness);
    commit_draft(&mut harness.cx(), &committed, property);
    assert_eq!(revision(harness), after, "{property} committed once");
}

fn selected(harness: &Harness) -> SpecificationDefinition {
    let measurement = harness
        .state
        .selected_specification
        .as_deref()
        .expect("a selected specification");
    plan_definition(&harness.model, measurement).expect("its definition")
}

#[test]
fn every_property_commits_alone_as_one_plan_change() {
    let mut harness = harness_with_gain();
    harness.state.selected_specification = Some("gain".to_owned());
    let words = inspector_words(&mut harness);
    assert!(
        words.iter().any(|word| word == "Specification"),
        "{words:?}"
    );
    assert!(words.iter().any(|word| word == "Verdict"), "{words:?}");

    commit_edit(&mut harness, "name", |draft| {
        draft.requirement_name = "Open-loop gain".to_owned();
    });
    assert_eq!(selected(&harness).requirement_name, "Open-loop gain");

    commit_edit(&mut harness, "key", |draft| {
        draft.requirement_key = "AMP-1".to_owned();
    });
    assert_eq!(selected(&harness).requirement_key, "AMP-1");

    commit_edit(&mut harness, "comparison", |draft| {
        draft.comparison = ComparisonDraftKind::Range;
        draft.primary_limit = "40".to_owned();
        draft.secondary_limit = "60".to_owned();
    });
    assert_eq!(
        selected(&harness).comparison,
        SpecificationComparison::Range {
            minimum: 40.0,
            maximum: 60.0
        }
    );

    commit_edit(&mut harness, "guard band", |draft| {
        draft.guard_band = "1".to_owned();
    });
    assert_eq!(selected(&harness).guard_band, Some(1.0));

    commit_edit(&mut harness, "unit", |draft| {
        draft.unit = "V/V".to_owned();
    });
    assert_eq!(selected(&harness).unit, "V/V");

    commit_edit(&mut harness, "role", |draft| {
        draft.role = SpecificationRole::Review;
    });
    assert_eq!(selected(&harness).role, SpecificationRole::Review);

    commit_edit(&mut harness, "points it applies to", |draft| {
        draft.scope = SpecPointScope::Nominal;
    });
    assert_eq!(selected(&harness).scope, SpecPointScope::Nominal);

    let producer = analysis_options(&harness.model)
        .first()
        .map(|(id, _)| *id)
        .expect("the plan holds an analysis");
    commit_edit(&mut harness, "producing analysis", |draft| {
        draft.producing_analysis = Some(producer);
    });
    assert_eq!(selected(&harness).producing_analysis, Some(producer));

    commit_edit(&mut harness, "expression", |draft| {
        draft.expression = "max(vdb(out))".to_owned();
    });
    assert_eq!(selected(&harness).expression, "max(vdb(out))");

    commit_edit(&mut harness, "measurement", |draft| {
        draft.measurement = "gain_db".to_owned();
    });
    assert_eq!(
        harness.state.selected_specification.as_deref(),
        Some("gain_db"),
        "the selection follows the specification to its new measurement"
    );
    assert_eq!(selected(&harness).requirement_name, "Open-loop gain");
}

/// A draft that does not make a valid definition commits nothing: the plan
/// keeps what it had and the reason is on the inspector.
#[test]
fn a_draft_that_is_not_a_definition_commits_nothing() {
    let mut harness = harness_with_gain();
    harness.state.selected_specification = Some("gain".to_owned());
    let definition = selected(&harness);
    let mut draft = SpecDraft::from_definition(&definition);
    draft.primary_limit = "forty".to_owned();
    let before = revision(&harness);
    harness.state.verification.specification_draft = Some(SpecificationInspectorDraft {
        specification: Some(definition.id),
        revision: Some(before),
        draft,
    });
    commit_draft(&mut harness.cx(), &definition, "limit");
    assert_eq!(revision(&harness), before);
    let words = inspector_words(&mut harness);
    assert!(
        words
            .iter()
            .any(|word| word.contains("minimum limit is invalid")),
        "{words:?}"
    );
}

#[test]
fn nothing_selected_states_acceptance_and_evidence() {
    let mut harness = harness_with_gain();
    harness.state.selected_specification = None;
    let words = inspector_words(&mut harness);
    for word in [
        "Acceptance",
        "Nominal failure",
        "Missing measurement",
        "Monte Carlo gate",
        "Regression check",
        "Evidence",
        "No run is selected.",
    ] {
        assert!(
            words.iter().any(|painted| painted == word),
            "{word}: {words:?}"
        );
    }
}

/// The points a specification may apply to are the plan's: every point, the
/// nominal one, and each process corner the run set declares. A scope the
/// run set no longer declares stays listed, so the choice shows what the
/// specification holds.
#[test]
fn the_scope_choices_are_the_corners_the_run_set_declares() {
    use crate::product::ProcessCorner;
    use rspice_simulation_contract::corner_config::{CornerBaseAnalysis, CornerConfig};

    let mut harness = harness_with_gain();
    harness.model.sim_setup.run_set =
        crate::simulation::controller::test_execution::pvt::from_corner_config(&CornerConfig {
            process_corners: vec![ProcessCorner::TT, ProcessCorner::SS],
            voltages: vec![1.0],
            supply_source_names: Vec::new(),
            temperatures: vec![-40.0, 27.0, 125.0],
            full_matrix: true,
            points: Vec::new(),
            base_analysis: CornerBaseAnalysis::Op,
        });
    let labels = |current: &SpecPointScope| {
        scope_options(&harness.model, current)
            .into_iter()
            .map(|(label, _)| label)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        labels(&SpecPointScope::AllPoints),
        ["Every point", "The nominal point", "Corner TT", "Corner SS"]
    );
    assert_eq!(
        labels(&SpecPointScope::SelectedCorners {
            corners: vec!["FF".to_owned()],
        }),
        [
            "Every point",
            "The nominal point",
            "Corner TT",
            "Corner SS",
            "Corners FF"
        ]
    );
}

/// The selection names a specification by its measurement, so removing one
/// never hands the selection to the specification that took its place.
#[test]
fn a_removed_specification_hands_its_selection_to_no_other() {
    let mut harness = harness_with_gain();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the harness plan is stable")
        .id();
    harness.model.workspace.content.replace_active_specs(
        plan_id,
        vec![SpecEntry {
            measurement: "bandwidth".to_owned(),
            expression: String::new(),
            min: Some(1.0e6),
            max: None,
            unit: "Hz".to_owned(),
            scope: SpecPointScope::AllPoints,
        }],
    );
    harness.state.selected_specification = Some("gain".to_owned());
    let words = inspector_words(&mut harness);
    assert!(
        words.iter().any(|word| word == "Acceptance"),
        "nothing resolves, so the inspector edits the plan's acceptance: {words:?}"
    );
    assert!(
        !words.iter().any(|word| word.contains("bandwidth")),
        "{words:?}"
    );
}

/// A draft edits the properties its rows show and keeps the rest of the
/// definition it was taken from: its identity, the requirement table it was
/// imported from, its waiver, its producer, its comparison and its scope.
#[test]
fn a_draft_keeps_what_its_rows_do_not_show() {
    use rspice_results::specification::{SpecificationSource, SpecificationWaiver};

    let producer = AnalysisInstanceId::new();
    let entry = SpecEntry {
        measurement: "offset".to_owned(),
        expression: "avg V(out)".to_owned(),
        min: Some(-0.1),
        max: Some(0.1),
        unit: "V".to_owned(),
        scope: SpecPointScope::Nominal,
    };
    let mut definition =
        SpecificationDefinition::from_legacy(crate::product::SimulationPlanId::new(), 0, &entry);
    definition.requirement_key = "REQ-OFFSET-1".to_owned();
    definition.comparison = SpecificationComparison::EqualWithin {
        target: 0.0,
        tolerance: 0.1,
    };
    definition.guard_band = Some(0.01);
    definition.role = SpecificationRole::Review;
    definition.producing_analysis = Some(producer);
    definition.source = Some(SpecificationSource {
        logical_path: "requirements/analog.csv".to_owned(),
        row: 12,
        imported_revision: "req-19".to_owned(),
        source_digest: crate::product::ContentDigest::from_bytes([0x71; 32]),
    });
    definition.waiver = Some(SpecificationWaiver {
        reference: "WVR-7".to_owned(),
        owner: "Analog lead".to_owned(),
        rationale: "Characterization disposition".to_owned(),
    });

    let mut draft = SpecDraft::from_definition(&definition);
    draft.requirement_name = "Input-referred offset".to_owned();
    let retained = draft
        .parse()
        .expect("the draft is a definition")
        .expect("a named measurement is kept");

    assert_eq!(retained.id, definition.id);
    assert_eq!(retained.requirement_name, "Input-referred offset");
    assert_eq!(retained.comparison, definition.comparison);
    assert_eq!(retained.source, definition.source);
    assert_eq!(retained.waiver, definition.waiver);
    assert_eq!(retained.producing_analysis, Some(producer));
    assert_eq!(retained.scope, SpecPointScope::Nominal);
}
