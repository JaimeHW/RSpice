//! The Models page's binding actions and qualification gate.
//!
//! A binding action commits exactly the order, corner or removal it names,
//! and a refused one leaves the plan and its revision as they were. The gate
//! never reads an open disposition, an unreadable gate, an empty closure or
//! an engine-owned one as clean; it orders findings worst first, and its
//! totals are the sum over the workspace summaries.

use crate::workbench::RSpiceApp;
use crate::workbench::framework::feature::ModelsPort;
use crate::workbench::framework::ports::QualificationGate;
use crate::workbench::surfaces::simulate::page_kit::Tone;
use crate::workbench::surfaces::simulate::page_models::{
    ModelBindingAction, ModelGate, apply_model_binding_action, gate_reading, gate_status,
};

fn active_plan_revision(app: &RSpiceApp) -> crate::product::ObjectRevision {
    app.state
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .expect("test instance has an active plan")
        .revision()
}

fn add_sectioned_library(app: &mut RSpiceApp, source_name: &str, model: &str) -> String {
    app.state
        .model
        .model_library_manager
        .load_library_bytes(
            source_name,
            format!(
                ".lib TT\n.model {model} NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n\
                 .lib FF\n.model {model} NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n"
            )
            .into_bytes(),
            None,
        )
        .expect("sectioned test library is executable")
}

#[test]
fn model_binding_actions_commit_exact_order_corner_and_removal() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let first = add_sectioned_library(&mut app, "models-first.lib", "first_nch");
    let second = add_sectioned_library(&mut app, "models-second.lib", "second_nch");
    let initial_revision = active_plan_revision(&app);
    app.state.runs.preflight.report = Some(crate::state::PreflightReport::retained_for_test(
        app.state.model.workspace.content.project.revision().get(),
    ));

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Attach(first.clone()),
    );
    assert_eq!(
        app.state.model.sim_setup.model_bindings[0].library_name,
        first
    );
    assert_ne!(active_plan_revision(&app), initial_revision);
    assert!(app.state.runs.preflight.report.is_none());

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Attach(second.clone()),
    );
    assert_eq!(
        app.state
            .model
            .sim_setup
            .model_bindings
            .iter()
            .map(|binding| binding.library_name.as_str())
            .collect::<Vec<_>>(),
        [first.as_str(), second.as_str()]
    );

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::MoveUp(1),
    );
    assert_eq!(
        app.state
            .model
            .sim_setup
            .model_bindings
            .iter()
            .map(|binding| binding.library_name.as_str())
            .collect::<Vec<_>>(),
        [second.as_str(), first.as_str()]
    );

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::SetCorner {
            index: 0,
            corner: Some("FF".to_owned()),
        },
    );
    assert_eq!(
        app.state.model.sim_setup.model_bindings[0]
            .selected_corner
            .as_deref(),
        Some("FF")
    );

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Remove(1),
    );
    assert_eq!(app.state.model.sim_setup.model_bindings.len(), 1);
    assert_eq!(
        app.state.model.sim_setup.model_bindings[0].library_name,
        second
    );
    assert!(!app.state.model.analysis_lifecycle_status.is_refusal());
}

#[test]
fn refused_model_binding_actions_preserve_plan_state_and_revision() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let library = add_sectioned_library(&mut app, "models-valid.lib", "valid_nch");
    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Attach(library),
    );
    let before_bindings = app.state.model.sim_setup.model_bindings.clone();
    let before_revision = active_plan_revision(&app);

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::SetCorner {
            index: 0,
            corner: Some("NONEXISTENT".to_owned()),
        },
    );
    assert_eq!(app.state.model.sim_setup.model_bindings, before_bindings);
    assert_eq!(active_plan_revision(&app), before_revision);
    assert!(app.state.model.analysis_lifecycle_status.is_refusal());

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Attach("missing-library".to_owned()),
    );
    assert_eq!(app.state.model.sim_setup.model_bindings, before_bindings);
    assert_eq!(active_plan_revision(&app), before_revision);
    assert!(app.state.model.analysis_lifecycle_status.is_refusal());
}

const fn model_gate<'a>(
    model: &'a str,
    vectors: usize,
    evidenced_vectors: usize,
    passing_vectors: usize,
    open_dispositions: usize,
    unreadable: bool,
    gate: QualificationGate,
) -> ModelGate<'a> {
    ModelGate {
        library: "pdk",
        model,
        vectors,
        evidenced_vectors,
        passing_vectors,
        open_dispositions,
        unreadable,
        gate,
    }
}

#[test]
fn open_dispositions_are_reported_and_never_read_as_clean() {
    let reading = gate_reading([model_gate(
        "nch",
        12,
        12,
        12,
        3,
        false,
        QualificationGate::Review,
    )]);

    assert_eq!(reading.open_dispositions, 3);
    let (status, tone) = gate_status(&reading).expect("open dispositions are stated");
    assert_eq!(status, "3 dispositions open");
    assert_eq!(tone, Tone::Warn);
    let finding = reading
        .findings
        .first()
        .expect("a model with open dispositions has a finding");
    assert_eq!(finding.model, "pdk/nch");
    assert_eq!(finding.finding, "3 dispositions open");
    assert_eq!(finding.state, "review");
    assert_eq!(finding.tone, Tone::Warn);
}

#[test]
fn an_unreadable_gate_is_reported_rather_than_counted_as_passing() {
    // The unreadable model carries outcomes that say every one of its
    // vectors passed. They are recorded against a source identity that no
    // longer validates, so the card must not spend them.
    let reading = gate_reading([
        model_gate("readable", 4, 4, 4, 0, false, QualificationGate::Qualified),
        model_gate("stale", 6, 6, 6, 0, true, QualificationGate::Blocked),
    ]);

    assert_eq!(reading.passing, 4);
    assert_eq!(reading.vectors, 10);
    assert_eq!(reading.unreadable, 1);
    let (status, tone) = gate_status(&reading).expect("an unreadable gate is stated");
    assert_eq!(status, "0 dispositions open · 1 gate unreadable");
    assert_eq!(tone, Tone::Error);
    let finding = reading
        .findings
        .first()
        .expect("an unreadable gate is a finding");
    assert_eq!(finding.model, "pdk/stale");
    assert_eq!(finding.finding, "gate could not be read");
}

#[test]
fn a_qualified_closure_reports_no_findings() {
    let reading = gate_reading([model_gate(
        "nch",
        8,
        8,
        8,
        0,
        false,
        QualificationGate::Qualified,
    )]);

    assert!(reading.findings.is_empty());
    assert_eq!(
        gate_status(&reading),
        None,
        "a clear gate is the normal state and states nothing"
    );
}

/// An empty closure has nothing to gate. Reporting it as clean would be the
/// card's one chance to sign off a release on no evidence at all.
#[test]
fn an_empty_closure_never_reads_as_a_clean_gate() {
    assert_eq!(
        gate_status(&gate_reading([])),
        Some(("no models to gate".to_owned(), Tone::Neutral))
    );
}

/// An engine-owned card is not a gate subject, so it is neither a finding
/// nor a pass. Both mistakes are available here: reporting the exemption as
/// "held for review" puts the simulator's own library in a table of defects,
/// and reading a closure of nothing but exempt models as qualified signs off
/// a release against a gate with no subjects in it.
#[test]
fn an_engine_owned_closure_is_neither_a_finding_nor_a_clean_gate() {
    let reading = gate_reading([
        model_gate("res", 0, 0, 0, 0, false, QualificationGate::EngineOwned),
        model_gate("cap", 0, 0, 0, 0, false, QualificationGate::EngineOwned),
    ]);

    assert!(reading.findings.is_empty());
    assert_eq!(reading.engine_owned, 2);
    assert_eq!(
        gate_status(&reading),
        Some((
            "2 engine-owned models · nothing under the release gate".to_owned(),
            Tone::Neutral
        ))
    );

    // One source-owned model in the closure and the gate has a subject
    // again, so the card goes back to reporting its verdict: clear, which
    // states nothing.
    let mixed = gate_reading([
        model_gate("res", 0, 0, 0, 0, false, QualificationGate::EngineOwned),
        model_gate("nch", 8, 8, 8, 0, false, QualificationGate::Qualified),
    ]);
    assert!(mixed.findings.is_empty());
    assert_eq!(gate_status(&mixed), None);
}

/// The card shows the first [`GATE_FINDING_ROWS`] findings, so they have to
/// be the worst ones.
#[test]
fn findings_are_ordered_worst_first() {
    let reading = gate_reading([
        model_gate("review", 2, 2, 2, 0, false, QualificationGate::Review),
        model_gate("nosuite", 0, 0, 0, 0, false, QualificationGate::Unqualified),
        model_gate("noevid", 5, 1, 1, 0, false, QualificationGate::Review),
        model_gate("disp", 3, 3, 3, 1, false, QualificationGate::Review),
        model_gate("fail", 4, 4, 3, 0, false, QualificationGate::Review),
        model_gate("unread", 9, 9, 9, 0, true, QualificationGate::Blocked),
    ]);

    let order: Vec<&str> = reading
        .findings
        .iter()
        .map(|finding| finding.model.as_str())
        .collect();
    assert_eq!(
        order,
        [
            "pdk/unread",
            "pdk/fail",
            "pdk/disp",
            "pdk/noevid",
            "pdk/nosuite",
            "pdk/review",
        ]
    );
    assert_eq!(reading.findings[1].finding, "1 evidenced vector failing");
    assert_eq!(reading.findings[2].finding, "1 disposition open");
    assert_eq!(reading.findings[3].finding, "4 vectors without results");
    assert_eq!(reading.findings[4].finding, "no qualification suite");
    assert_eq!(reading.findings[4].state, "unqualified");
}

/// The card and the Models workspace must not be able to disagree, so the
/// totals it shows are asserted against the summaries themselves.
#[test]
fn gate_totals_equal_the_sum_over_the_workspace_summaries() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
    let definition = crate::state::model_library::ProjectModelDefinition {
        name: "nch_gated".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Plan-page qualification gate fixture".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("vth0".to_owned(), 0.48),
        ]),
        string_parameters: std::collections::BTreeMap::new(),
    };
    app.state
        .model
        .model_library_manager
        .create_project_model("owned-models", &definition)
        .expect("create project model");
    crate::workbench::documents::model_editor::open_project_model(
        &mut app.state.model_editor_parts(),
        "owned-models",
        "nch_gated",
    )
    .expect("open the project model for qualification authoring");
    let editor = &mut app.state.models.model_editor;
    editor.begin_qualification_suite();
    let authoring = &mut editor.qualification_authoring;
    authoring.suite_id = "dc-op".to_owned();
    authoring.suite_name = "DC operating point".to_owned();
    authoring.vector_id = "nominal".to_owned();
    authoring.vector_name = "Nominal bias".to_owned();
    authoring.executable_input =
        "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_gated\n.op\n.end\n".to_owned();
    authoring.quantity = "v(out)".to_owned();
    authoring.probe_target = "out".to_owned();
    authoring.expected = "1".to_owned();
    authoring.absolute_tolerance = "1e-9".to_owned();
    authoring.relative_tolerance = "1e-9".to_owned();
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    let summaries = app.state.models.model_gate_facts(&app.state.model);
    let reading = gate_reading(summaries.iter().map(ModelGate::from));

    assert!(
        reading.vectors > 0,
        "the fixture must declare at least one release-gating vector"
    );
    assert_eq!(reading.models, summaries.len());
    assert_eq!(
        reading.vectors,
        summaries
            .iter()
            .map(|summary| summary.vectors)
            .sum::<usize>()
    );
    assert_eq!(
        reading.passing,
        summaries
            .iter()
            .filter(|summary| !summary.unreadable)
            .map(|summary| summary.passing_vectors)
            .sum::<usize>()
    );
    assert_eq!(
        reading.open_dispositions,
        summaries
            .iter()
            .map(|summary| summary.open_dispositions)
            .sum::<usize>()
    );
    assert_eq!(
        reading.unreadable,
        summaries
            .iter()
            .filter(|summary| summary.unreadable)
            .count()
    );
}
