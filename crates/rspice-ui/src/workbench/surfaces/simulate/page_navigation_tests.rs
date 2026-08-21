//! The studio's return edges, as the setup pages offer them.
//!
//! The pages state a great deal about datasets a run already produced —
//! "prior Run 3 immutable", "Producing analysis", "worst of 12 retained
//! measurements". Each of those sentences is only worth printing if the thing
//! it names can be reached, so these cases check that the control exists, that
//! it is disabled with its own reason when the object is not there, and that
//! the resolution behind it names the exact retained record.

use egui::{Rect, vec2};

use super::pages;
use crate::product::ContentDigest;
use crate::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisResultSourceDomain, AnalysisType,
    PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SavedOutput,
    SavedOutputMaterializationStatus, SavedOutputReceipt, SpecEntry, SpecPointScope, WaveformData,
};
use crate::workbench::state::SimulationPage;
use crate::workbench::{AppState, RSpiceApp};

const RENDER_VIEWPORT_HEIGHT: f32 = 2600.0;

/// Render one setup page over a seeded app and collect the text it painted.
fn render_with(page: SimulationPage, width: f32, seed: impl FnOnce(&mut AppState)) -> String {
    fn collect(shape: &egui::epaint::Shape, rendered: &mut String) {
        match shape {
            egui::epaint::Shape::Text(text) => {
                rendered.push_str(&text.galley.job.text);
                rendered.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, rendered);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.simulation_page = page;
    seed(&mut app.state);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                vec2(width, RENDER_VIEWPORT_HEIGHT),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    // The Analyses route is drawn by the surface itself and
                    // never reaches `pages::show`, so the whole surface is
                    // what a test of the plan heading has to run.
                    if page == SimulationPage::Analyses {
                        super::show(ui, &mut app);
                    } else {
                        egui::ScrollArea::vertical().show(ui, |ui| pages::show(ui, &mut app, page));
                    }
                });
        },
    );
    let mut rendered = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut rendered);
    }
    rendered
}

/// One plan-owned saved output over a plain node voltage.
fn saved_output() -> SavedOutput {
    SavedOutput::new(
        crate::state::SavedOutputKind::RawVoltageOrCurrent,
        "V(out)",
        "V(out)",
        crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
        crate::state::SavedOutputPolicy::EveryAcceptedPoint,
        crate::state::SavedOutputPrecision::FullSourcePrecision,
        crate::state::SavedOutputStreaming::StoreOnly,
    )
    .expect("a valid saved output")
}

/// Seal one retained run against the app's own active plan, produced by its
/// first instance, holding `analysis`.
fn retain_run_for_active_plan(state: &mut AppState, analysis: AnalysisResult) {
    state.project_lifecycle.project_open = true;
    let plan = state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance owns a stable analysis plan");
    let plan_id = plan.id();
    let analysis_id = plan
        .instances()
        .first()
        .expect("the default plan holds one instance")
        .id();
    let project_revision = state.workspace.project.revision();
    let receipt = PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        project_revision,
        ContentDigest::from_bytes([0x11; 32]),
        ContentDigest::from_bytes([0x22; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x33; 32])),
        vec![
            PreparedRunTaskReceipt::new(
                analysis_id,
                project_revision,
                Vec::new(),
                1,
                ContentDigest::from_bytes([0x44; 32]),
            )
            .expect("valid prepared task"),
        ],
    )
    .expect("valid prepared run receipt");
    let analysis = analysis.with_provenance(
        AnalysisResultProvenance::new_with_source_domain(
            AnalysisResultSourceDomain::SimulationPlan,
            analysis_id,
            project_revision,
            ContentDigest::from_bytes([0x44; 32]),
            Vec::new(),
        )
        .expect("valid prepared analysis provenance"),
    );
    state
        .simulation
        .start_prepared_run(receipt)
        .add_analysis(analysis);
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
}

/// The plan heading says whether a prior dataset exists; both controls beside
/// it are the ways of acting on that sentence, and both are drawn whether or
/// not there is one so the reader is never left guessing that a route exists.
#[test]
fn the_plan_heading_offers_its_prior_dataset_and_the_split_stage() {
    let empty = render_with(SimulationPage::Analyses, 1400.0, |_| {});
    assert!(
        empty.contains("no prior dataset"),
        "the heading states the absence:\n{empty}"
    );
    assert!(
        empty.contains("Open prior run"),
        "and the route to one is still drawn, disabled:\n{empty}"
    );
    assert!(
        empty.contains("Split with results"),
        "the split stage has a visible affordance beside the plan actions:\n{empty}"
    );

    let retained = render_with(SimulationPage::Analyses, 1400.0, |app| {
        retain_run_for_active_plan(
            app,
            AnalysisResult::new(1, AnalysisType::Transient, "retained TRAN"),
        );
    });
    assert!(
        retained.contains("prior Run 1 immutable"),
        "the heading names the run it found:\n{retained}"
    );
    assert!(
        retained.contains("Open Run 1"),
        "and the control names the same one, so the two cannot disagree:\n{retained}"
    );
}

/// A limit's detail card names the analysis that produces it and the evidence
/// that answers it. Both are routes, and both refuse in their own words.
#[test]
fn the_selected_limit_offers_its_evidence_and_its_producer() {
    let without = render_with(SimulationPage::Specifications, 1400.0, |state| {
        let plan_id = state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        state.workspace.replace_active_specs(
            plan_id,
            vec![SpecEntry {
                measurement: "gain_dc".to_owned(),
                expression: "db20(V(out))".to_owned(),
                min: Some(40.0),
                max: None,
                unit: "dB".to_owned(),
                scope: SpecPointScope::AllPoints,
            }],
        );
        state.workbench.selected_specification = Some("gain_dc".to_owned());
    });
    assert!(
        without.contains("Open evidence in Results"),
        "the evidence route is drawn even with no dataset, disabled:\n{without}"
    );
    assert!(
        without.contains("Producing analysis"),
        "and the producer is still identified:\n{without}"
    );
}

/// A saved output's card offers the trace the dataset actually stored, and
/// says which of the several reasons there is none when there is none.
#[test]
fn a_saved_output_offers_the_trace_it_stored() {
    let rendered = render_with(SimulationPage::Outputs, 1400.0, |state| {
        let plan_id = state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let output = saved_output();
        let name = output.name.clone();
        state
            .workspace
            .add_saved_output(plan_id, output)
            .expect("the plan accepts one saved output");
        state.workbench.selected_saved_output = Some(name);
    });
    assert!(
        rendered.contains("View trace"),
        "the trace route is drawn on the record's own card:\n{rendered}"
    );
}

/// The point table's family column is present whether or not a family exists,
/// so the refusal is on screen rather than implied by an absent control.
#[test]
fn the_resolved_point_table_offers_each_point_its_family_member() {
    let rendered = render_with(SimulationPage::RunSet, 1400.0, |_| {});
    assert!(
        rendered.contains("FAMILY"),
        "the point table carries a column for the route out:\n{rendered}"
    );
    assert!(
        rendered.contains("Open"),
        "and each row carries the control itself:\n{rendered}"
    );
}

/// The refusals are the product: a run from another plan, a dataset with no
/// family, and no run at all are three different answers.
#[test]
fn a_point_without_a_retained_family_says_which_kind_of_nothing_it_has() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    assert_eq!(
        super::page_runset::family_target(&app),
        Err("No run has been retained, so there is no family to open a point in.")
    );

    retain_run_for_active_plan(
        &mut app.state,
        AnalysisResult::new(1, AnalysisType::Transient, "retained TRAN"),
    );
    assert_eq!(
        super::page_runset::family_target(&app),
        Err(
            "The retained dataset holds no analysis with a family, so there are no members to slice."
        ),
        "a dataset without family metadata is not a dataset with an empty family"
    );
}

/// The family view's initial selection is its typed filter, so a point is
/// carried into it by compiling its coordinates into one — over the axes the
/// retained family actually declares, and nothing else.
#[test]
fn a_point_is_carried_into_the_family_view_as_a_filter_over_its_own_axes() {
    use crate::state::AnalysisResultFamilyMetadata;

    let mut app = RSpiceApp::test_instance();
    let analysis = AnalysisResult::new(1, AnalysisType::Corner, "corners").with_family_metadata(
        AnalysisResultFamilyMetadata::Corner {
            x_values: vec![1.8, 1.8],
            x_label: "supply".to_owned(),
            x_unit: "V".to_owned(),
            temperatures_c: vec![27.0, 125.0],
            corner_labels: vec!["TT".to_owned(), "TT".to_owned()],
            failed_corners: 0,
        },
    );
    retain_run_for_active_plan(&mut app.state, analysis);
    assert_eq!(super::page_runset::family_target(&app), Ok(0));

    // An axis the family does not declare contributes no clause, and one it
    // does is matched by the dimension's own identity.
    let coordinates = vec![
        (
            "temperature".to_owned(),
            "Temperature".to_owned(),
            "125".to_owned(),
        ),
        (
            "moon-phase".to_owned(),
            "Moon phase".to_owned(),
            "waxing".to_owned(),
        ),
    ];
    assert_eq!(
        super::page_runset::family_query_for_point(&mut app.state, 0, &coordinates),
        Some("temperature = 125".to_owned()),
        "only the axes the retained family declares reach the filter"
    );

    super::page_runset::open_point_in_family(&mut app, 0, &coordinates);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Results,
        "the hop lands where the family view is drawn"
    );
    assert_eq!(
        app.state.workbench.visualization_studio.family_query, "temperature = 125",
        "and it arrives narrowed to the member the point names"
    );

    // A point the family holds no member for refuses rather than landing on
    // the whole family and calling that the point.
    let absent = vec![(
        "temperature".to_owned(),
        "Temperature".to_owned(),
        "-40".to_owned(),
    )];
    let before = app.state.log_buffer.revision();
    assert_eq!(
        super::page_runset::family_query_for_point(&mut app.state, 0, &absent),
        None
    );
    assert!(app.state.log_buffer.revision() > before);
}

/// The receipt is the authority on what a saved output stored, so each
/// materialization status refuses in its own words rather than collapsing to
/// "no trace".
#[test]
fn a_saved_output_trace_is_resolved_from_the_receipt_not_the_waveform_names() {
    let mut app = RSpiceApp::test_instance();
    let output = saved_output();
    assert_eq!(
        super::page_outputs::materialized_trace(&app, &output),
        Err("No run has been retained, so this output has no stored trace.".to_owned())
    );

    // A waveform of the same name, with no receipt claiming it, is not
    // evidence that this contract produced it.
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "tran").with_waveforms(vec![
        WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
    ]);
    retain_run_for_active_plan(&mut app.state, analysis);
    assert_eq!(
        super::page_outputs::materialized_trace(&app, &output),
        Err(
            "The retained dataset holds no receipt for this output, so it was never stored."
                .to_owned()
        )
    );

    // A deferred contract is retained but unevaluated, and says so.
    app.state.simulation.runs[0].analyses[0]
        .saved_output_receipts
        .push(SavedOutputReceipt {
            output_id: output.id,
            output_revision: output.revision,
            analysis_id: crate::product::AnalysisInstanceId::new(),
            contract_digest: ContentDigest::from_bytes([0x55; 32]),
            name: output.name.clone(),
            source_expression: output.source_expression.clone(),
            output_kind: output.kind,
            save_policy: output.save_policy,
            stored_precision: output.stored_precision,
            streaming: output.streaming,
            display_intent: output.display_intent,
            status: SavedOutputMaterializationStatus::Deferred,
        });
    assert!(
        super::page_outputs::materialized_trace(&app, &output)
            .expect_err("a deferred output has no trace")
            .contains("deferred in the retained dataset"),
        "the refusal names the status rather than the absence"
    );

    // Materialized, and the waveform is there: the hop resolves to it.
    app.state.simulation.runs[0].analyses[0].saved_output_receipts[0].status =
        SavedOutputMaterializationStatus::Materialized {
            waveform_name: "V(out)".to_owned(),
            sample_count: 2,
        };
    assert_eq!(
        super::page_outputs::materialized_trace(&app, &output),
        Ok((0, 0)),
        "the analysis and waveform the receipt names are what the hop carries"
    );

    super::page_outputs::open_materialized_trace(&mut app, 0, 0);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Results
    );
    assert_eq!(
        app.state
            .ui
            .results
            .selected_trace
            .as_ref()
            .map(crate::workbench::documents::result_document::SelectedResultTrace::source_name),
        Some("V(out)"),
        "and the trace itself is selected, not merely the workspace opened"
    );
}
