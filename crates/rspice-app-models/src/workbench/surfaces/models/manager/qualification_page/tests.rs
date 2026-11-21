//! What the qualification matrix is allowed to claim, and where it stops.
//!
//! The page's whole subject is coverage, so the assertions here are about the
//! difference between an absence and a zero: a family that never wrote a
//! transient vector and a family whose transient vectors all fail are two
//! different findings, and a matrix that spelled them the same way would be
//! worse than the badge it replaced.

use super::*;

use crate::state::ModelsPage;
use crate::state::model_library::{ModelLibraryManager, ProjectModelDefinition};
use crate::workbench::documents::model_editor::{
    QualificationAuthoringAnalysis, QualificationAuthoringProbe, QualificationAuthoringSample,
};

/// The document column in the shell at a 1680 px window, and a viewport tall
/// enough to show the composition the page is measured against.
const PAGE_SIZE: [f32; 2] = [1_180.0, 900.0];

/// A project owning one model with retained DC, AC/charge and transient
/// suites — and deliberately no noise suite.
///
/// Authored through the model editor's own transaction rather than by writing
/// a `ModelQualificationState` by hand: a suite is only counted when its
/// vectors bind the *exact* resolved source revision, and a fixture that
/// assembled that binding itself would be testing the fixture.
fn source_owned_app() -> FeatureHarness<ModelsFeature> {
    let mut app = harness();
    app.model.model_library_manager = ModelLibraryManager::new();
    let definition = ProjectModelDefinition {
        name: "nch_owned".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Project-owned qualification fixture".to_owned(),
        numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0), ("vth0".to_owned(), 0.48)]),
        string_parameters: BTreeMap::new(),
    };
    app.model
        .model_library_manager
        .create_project_model("owned-models", &definition)
        .expect("create project model");

    let summary = qualification_summaries(&app.model, &app.state.model_editor)
        .into_iter()
        .find(|summary| summary.model == "nch_owned")
        .expect("the project-owned family is summarized");
    app.model
        .model_library_manager
        .select_library(&summary.library)
        .expect("the fixture library is loaded");
    app.state.selected_model = Some(summary.model.clone());
    execute_qualification_action(&mut app.cx(), QualificationPageAction::ReviewVectors);

    let editor = &mut app.state.model_editor;

    editor.begin_qualification_suite();
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "dc-op".to_owned();
        fields.suite_name = "DC operating point".to_owned();
        fields.vector_id = "nominal".to_owned();
        fields.vector_name = "Nominal bias".to_owned();
        fields.executable_input =
            "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.op\n.end\n".to_owned();
        fields.quantity = "v(out)".to_owned();
        fields.probe_target = "out".to_owned();
        fields.expected = "1".to_owned();
        fields.absolute_tolerance = "1e-9".to_owned();
        fields.relative_tolerance = "1e-9".to_owned();
    }
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    editor.begin_qualification_suite();
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "ac-cv".to_owned();
        fields.suite_name = "AC and capacitance".to_owned();
        fields.vector_id = "ac-nominal".to_owned();
        fields.vector_name = "Nominal small-signal".to_owned();
        fields.analysis = QualificationAuthoringAnalysis::AcSweep;
        fields.frequencies = "1e3, 1e4".to_owned();
        fields.probe = QualificationAuthoringProbe::AcEffectiveCapacitance;
        fields.probe_target = "V1".to_owned();
        fields.excitation_magnitude = "1".to_owned();
        fields.sample = QualificationAuthoringSample::FirstFrequencyPoint;
        fields.executable_input =
            "V1 out 0 DC 0 AC 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.end\n".to_owned();
        fields.quantity = "ceff(V1)".to_owned();
        fields.expected = "0".to_owned();
        fields.absolute_tolerance = "1e-15".to_owned();
        fields.relative_tolerance = "0".to_owned();
    }
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    editor.begin_qualification_suite();
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "transient".to_owned();
        fields.suite_name = "Transient".to_owned();
        fields.vector_id = "transient-nominal".to_owned();
        fields.vector_name = "Nominal waveform".to_owned();
        fields.executable_input = "V1 out 0 DC 1\nMbind out out 0 0 nch_owned\n.end\n".to_owned();
        fields.analysis = QualificationAuthoringAnalysis::Transient;
        fields.transient_stop_time = "1e-6".to_owned();
        fields.transient_max_step = "1e-7".to_owned();
        fields.quantity = "v(out)".to_owned();
        fields.probe = QualificationAuthoringProbe::TransientNodeVoltage;
        fields.probe_target = "out".to_owned();
        fields.sample = QualificationAuthoringSample::LastTimePoint;
        fields.expected = "1".to_owned();
        fields.absolute_tolerance = "1e-8".to_owned();
        fields.relative_tolerance = "1e-8".to_owned();
    }
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    app.state.models_page = ModelsPage::Qualification;
    app
}

fn owned_summary(app: &FeatureHarness<ModelsFeature>) -> QualificationModelSummary {
    qualification_summaries(&app.model, &app.state.model_editor)
        .into_iter()
        .find(|summary| summary.model == "nch_owned")
        .expect("the project-owned family is summarized")
}

fn themed_context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// The closure ledger counts only the population the gate governs, and states
/// no verdict at all.
///
/// A fresh project holding nothing but compiled-in cards once read
/// "0 / 16 qualified" — a failure claim assembled entirely out of models the
/// gate has nothing to say about — so the denominator is still pinned here.
/// The tones are gone outright: the band summarises the matrix below it, and
/// the matrix's own domain and gate columns are where a shortfall is read.
#[test]
fn the_closure_inputs_count_gate_subjects_and_not_the_exempt_population() {
    let app = harness();
    let summaries = qualification_summaries(&app.model, &app.state.model_editor);
    let exempt = summaries
        .iter()
        .filter(|summary| !summary.gate.is_gate_subject())
        .count();
    assert!(
        exempt > 0 && exempt == summaries.len(),
        "a fresh project holds the engine-owned families and nothing else"
    );
    for input in &closure_inputs(&summaries) {
        assert_eq!(
            input.value, "0/0",
            "`{}` totalled the exempt population into its denominator",
            input.label
        );
    }
    assert_eq!(
        closure_inputs(&summaries)[2].detail,
        format!("{exempt} engine-owned"),
        "the exempt population is what the qualified segment has to account for"
    );

    let app = source_owned_app();
    let summaries = qualification_summaries(&app.model, &app.state.model_editor);
    let inputs = closure_inputs(&summaries);
    assert_eq!(
        inputs
            .iter()
            .map(|input| (input.label, input.value.as_str()))
            .collect::<Vec<_>>(),
        vec![
            ("vectors", "0/3"),
            ("references", "0/3"),
            ("qualified", "0/1"),
            ("parity", "0/1"),
        ],
        "the ledger read the whole corpus rather than the one family the gate governs"
    );
    assert_eq!(inputs[0].detail, "no open dispositions");
    assert_eq!(inputs[1].detail, "vectors with a retained reference");
    assert_eq!(inputs[3].detail, "desktop · WebAssembly");
}

/// The segments are painted glyphs, so the node each hover region publishes is
/// the only thing a screen reader has — and it has to carry the fact the band
/// itself has room for only on hover.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_closure_segment_announces_its_ratio_and_the_fact_behind_it() {
    let ctx = themed_context();
    ctx.enable_accesskit();
    let mut app = source_owned_app();
    let size = egui::vec2(PAGE_SIZE[0], PAGE_SIZE[1]);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::super::show(ui, &mut app.cx()));
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("qualification accessibility tree")
        .nodes;
    // egui gives `WidgetType::Label` AccessKit's Label role, whose text is the
    // node's *value*; `label()` is empty on every painted-glyph segment.
    let announcements = nodes
        .iter()
        .filter_map(|(_, node)| node.value())
        .collect::<Vec<_>>();
    for announced in [
        "vectors 0/3, no open dispositions",
        "references 0/3, vectors with a retained reference",
        "parity 0/1, desktop · WebAssembly",
    ] {
        assert!(
            announcements.contains(&announced),
            "no closure segment announced {announced:?}"
        );
    }
    // This corpus holds no engine-owned cards, so the qualified segment
    // announces the population it does govern; the exempt phrasing is pinned
    // by `the_closure_inputs_count_gate_subjects_...` on the fresh project.
    assert!(
        announcements.contains(&"qualified 0/1, source-owned release gates"),
        "the qualified segment does not state the population the gate governs"
    );
}

/// A domain the family never wrote a vector against reads as absent, and one
/// it did reads as its own counts.
///
/// This is the whole reason the page is a matrix. "0 / 0" against an
/// undeclared domain is a coverage claim nobody made.
#[test]
fn an_undeclared_domain_reads_absent_and_a_declared_one_reads_its_own_counts() {
    let app = source_owned_app();
    let summary = owned_summary(&app);
    assert_eq!(summary.suites, 3, "the fixture retains three suites");
    let ctx = themed_context();
    let t = Tokens::get(&ctx);

    for domain in [
        QualificationDomain::Dc,
        QualificationDomain::Ac,
        QualificationDomain::Transient,
    ] {
        let (cell, color) = domain_cell(&summary, domain, &t);
        assert_eq!(
            cell, "0 / 1",
            "{domain:?} retains one vector and no passing evidence"
        );
        assert_eq!(
            color, t.color.warn,
            "{domain:?} declares a vector nothing has passed and is not toned as a pass"
        );
    }

    let (noise, color) = domain_cell(&summary, QualificationDomain::Noise, &t);
    assert_eq!(
        noise, "—",
        "a domain the family never declared must not read as a count"
    );
    assert_eq!(color, t.color.text_faint);
    assert!(
        summary.domain(QualificationDomain::Noise).is_none(),
        "the projection invented a noise domain the suites do not hold"
    );
}

/// Every cell a sighted reader can see is announced, because none of them is a
/// widget: the row is painted, and the row's node is the only thing a screen
/// reader has.
#[test]
fn a_matrix_row_announces_every_domain_it_paints() {
    let app = source_owned_app();
    let announced = row_announcement(&owned_summary(&app));
    for domain in QualificationDomain::ALL {
        assert!(
            announced.contains(domain.label()),
            "the row says nothing about {domain:?}: {announced}"
        );
    }
    assert!(
        announced.contains("not declared"),
        "the undeclared noise domain is announced as a count: {announced}"
    );
    assert!(announced.contains("nch_owned") && announced.contains("owned-models"));
}

/// Both per-family workflows stay reachable from the matrix's selection strip.
///
/// Not a wiring test — [`qualification_action_block_reason`] has those — but a
/// guard that the composition change did not strand a route: the strip is the
/// only place either action is authored now.
#[test]
fn the_selection_strip_keeps_both_family_workflows_reachable() {
    let mut app = source_owned_app();
    let summary = owned_summary(&app);
    assert_eq!(
        qualification_action_block_reason(
            &app.cx(),
            Some(&summary),
            QualificationPageAction::ReviewVectors
        ),
        None,
        "a family with retained vectors can be reviewed"
    );
    let engine_owned = qualification_summaries(&app.model, &app.state.model_editor)
        .into_iter()
        .find(|summary| !summary.gate.is_gate_subject());
    if let Some(engine_owned) = engine_owned {
        assert!(
            qualification_action_block_reason(
                &app.cx(),
                Some(&engine_owned),
                QualificationPageAction::ReviewVectors
            )
            .is_some_and(|reason| reason.contains("exempt")),
            "an exempt family's blocked action must say why rather than vanish"
        );
    }
}
