//! What the navigator is allowed to claim about a run it did not execute.
//!
//! An empty specification set is neutral, not healthy; a finite goal miss is
//! executed but not passed; a mapping without an active dataset claims no
//! execution at all. The remaining cases hold the mock's grid and spacing, and
//! the density switch between desktop and touch.

use super::{
    CAPABILITY_BANNER_GAP, CAPABILITY_BANNER_ICON_SIZE, CAPABILITY_BANNER_MARGIN,
    EMPTY_HINT_PADDING_X, EMPTY_HINT_PADDING_Y, EXPRESSION_HEADER_HEIGHT, FLOW_DETAIL_TOP,
    FLOW_LABEL_TOP, FLOW_ROW_HEIGHT, FLOW_STATUS_TOP, FLOW_TEXT_LEFT, NAV_PROPERTY_COLUMN_GAP,
    NAV_PROPERTY_PADDING_X, PANEL_SEARCH_MARGIN_X, SIGNAL_ROW_HEIGHT, TOUCH_TARGET_HEIGHT,
    active_mc_sample_trail, flow_row_geometry, header, panel_search, panel_search_field_width,
    responsive_result_control_height, select_result_analysis, select_result_dataset,
    select_result_signal, simulate_nav_meta, verification_coverage, verification_flow_label,
    verification_navigator_requires_scroll,
};
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::services::yield_manager::{
    DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
};
use crate::simulation::run_set::RunSetDimensionKind;
use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisType, SimulationRun, SimulationState,
    WaveformData,
};
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultArtifactPresentationKey, ResultBrowserSelectionKey,
    SourceWaveformPresentationKey,
};
use crate::workbench::state::{SimulationPage, VerificationPage, Workspace};

fn result(trail: Vec<bool>) -> YieldResult {
    let pass_count = trail.iter().filter(|passes| **passes).count();
    YieldResult {
        spec: YieldSpec::lower("gain", 0.0, ""),
        total_runs: trail.len(),
        pass_count,
        fail_count: trail.len() - pass_count,
        yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
        stats: DistributionStats::default(),
        samples: vec![1.0; trail.len()],
        trail,
    }
}

/// Every painted run of text in the navigator, with where it landed.
fn navigator_painted_lines(width: f32, mut app: RSpiceApp) -> Vec<(String, egui::Rect)> {
    fn collect(shape: &egui::epaint::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width, 1400.0),
        )),
        ..Default::default()
    };
    let output = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_width(width);
                super::show(ui, &mut app);
            });
    });
    let mut lines = Vec::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut lines);
    }
    lines
}

/// No Run set row paints its value on top of its label.
///
/// The card is a two-column form grid in the mockup and the dock is 228 px
/// wide below 1261 px, which is where the single-row layout failed: the value
/// was floated right and grew leftwards with nothing to stop it, so
/// "Supply voltage" ran straight into "3 values · axis off" and "Variation"
/// into "no Monte Carlo instance" — no gutter, no ellipsis, no way to read
/// where one ended. Measured columns cannot do that, and this is the width
/// that proves it.
#[test]
fn run_set_card_rows_keep_their_value_off_their_label() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Simulate);
    let lines = navigator_painted_lines(228.0, app);

    let find = |text: &str| -> egui::Rect {
        lines
            .iter()
            .find(|(painted, _)| painted == text)
            .unwrap_or_else(|| {
                panic!(
                    "the Run set card paints {text:?}; it painted {:?}",
                    lines.iter().map(|(text, _)| text).collect::<Vec<_>>()
                )
            })
            .1
    };
    for (label, value) in [
        ("Supply voltage", "3 values \u{b7} axis off"),
        ("Variation", "no Monte Carlo instance"),
    ] {
        let label_rect = find(label);
        let value_rect = find(value);
        assert!(
            value_rect.left() - label_rect.right() >= NAV_PROPERTY_COLUMN_GAP - 0.5,
            "{label:?} ends at {} and {value:?} starts at {}, closer than the \
             {NAV_PROPERTY_COLUMN_GAP} px column gap",
            label_rect.right(),
            value_rect.left()
        );
        // A value pushed off the card's trailing inset would satisfy the gap
        // above and still be unreadable, so the row has to stay inside the
        // dock as well as clear of its own label.
        assert!(
            value_rect.right() <= 228.0 - NAV_PROPERTY_PADDING_X + 0.5,
            "{value:?} ends at {}, past the card's {NAV_PROPERTY_PADDING_X} px \
             trailing inset in a 228 px dock",
            value_rect.right()
        );
    }
}

/// A quantity axis states its unit; a reference axis has none to state.
///
/// The run set stores temperature as bare lexical numbers, so the rail read
/// "-40 25 125" and the unit lived only in the reader's head. The axis knows
/// its own unit — this is that unit reaching the row.
#[test]
fn run_set_card_states_the_unit_of_a_quantity_axis() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Simulate);
    // A disabled axis states its size, not its values, so the unit has nothing
    // to attach to. Enable the one this asks about.
    let temperature = app
        .state
        .sim_setup
        .run_set
        .dimensions
        .iter_mut()
        .find(|dimension| dimension.kind == RunSetDimensionKind::Temperature)
        .expect("the fixture run set declares a temperature axis");
    temperature.enabled = true;
    assert_eq!(temperature.unit(), Some("\u{b0}C"));
    let expected = format!(
        "{} \u{b0}C",
        temperature
            .values
            .iter()
            .map(|value| value.lexical.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let painted = navigator_painted_lines(228.0, app)
        .into_iter()
        .map(|(text, _)| text)
        .collect::<Vec<_>>();
    assert!(
        painted.contains(&expected),
        "the temperature row states its unit ({expected:?}); it painted {painted:?}"
    );
}

#[test]
fn navigator_search_and_empty_hint_match_mockup_spacing() {
    assert_eq!(PANEL_SEARCH_MARGIN_X, 8.0);
    assert_eq!(panel_search_field_width(260.0), 244.0);
    assert_eq!(NAV_PROPERTY_PADDING_X, 10.0);
    assert_eq!(EMPTY_HINT_PADDING_X, 12);
    assert_eq!(EMPTY_HINT_PADDING_Y, 20);
}

#[test]
fn simulation_models_meta_counts_only_active_plan_bindings() {
    let mut app = RSpiceApp::test_instance();

    assert_eq!(
        simulate_nav_meta(&app, SimulationPage::Models, "unused", None),
        None,
        "globally available libraries are not owned by the active simulation plan",
    );

    app.state.sim_setup.model_bindings.push(
        crate::state::model_library::SimulationPlanModelBinding {
            library_name: "project-process-models".to_owned(),
            source_digest: ContentDigest::from_bytes([0x6d; 32]),
            selected_corner: Some("tt".to_owned()),
        },
    );

    assert_eq!(
        simulate_nav_meta(&app, SimulationPage::Models, "unused", None),
        Some("1".to_owned()),
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn navigator_heading_and_search_expose_explicit_accesskit_names() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let mut query = String::new();
    let mut focus_pending = false;

    let nodes = ctx
        .run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(260.0);
                header(ui, &mut app);
                panel_search(
                    ui,
                    &mut query,
                    "navigator-accessibility-test",
                    "Find instance, net or port",
                    &mut focus_pending,
                );
            });
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Heading
            && node.label() == Some("Design navigator")
            && node.level() == Some(2)
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput
            && node.label() == Some("Find instance, net or port")
    }));
}

/// The mockup's data browser reads top to bottom as: tab band, then the
/// query and facets that filter the tab it selected, then the scope and
/// count band. A search above the tabs would read as filtering all three
/// tabs at once, which is what this pins against.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_data_browser_puts_its_query_under_the_tab_it_filters() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Results);

    let mut search_rect: Option<egui::Rect> = None;
    let mut producer_rect: Option<egui::Rect> = None;
    let mut facet_rects: Vec<egui::Rect> = Vec::new();
    // The narrowest the dock can be dragged to. `set_width` alone only raises
    // the minimum, so the surface would otherwise lay itself out against the
    // default screen and the row geometry below would prove nothing about the
    // width the toolbar actually has to survive.
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(220.0, 900.0),
        )),
        ..Default::default()
    };
    let output = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(220.0);
            super::show(ui, &mut app);
        });
    });
    for (_, node) in output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AccessKit tree update")
        .nodes
        .iter()
    {
        if node.role() == egui::accesskit::Role::TextInput
            && node.label() == Some("Find canonical name, path, unit, or type…")
            && let Some(bounds) = node.bounds()
        {
            search_rect = Some(egui::Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            ));
        }
        if node.role() == egui::accesskit::Role::ComboBox
            && matches!(node.label(), Some("Quantity kind" | "Quantity sort"))
            && let Some(bounds) = node.bounds()
        {
            facet_rects.push(egui::Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            ));
        }
        if node.role() == egui::accesskit::Role::ComboBox
            && node.label() == Some("Producing analysis")
            && let Some(bounds) = node.bounds()
        {
            producer_rect = Some(egui::Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            ));
        }
    }

    let search = search_rect.expect("the browser owns a query field with its own placeholder");
    // The tab band is a painted strip 30 px tall directly under the panel
    // header, so anything below it starts lower than that.
    assert!(
        search.top() > 30.0,
        "the query must sit below the tab band, not above it ({})",
        search.top()
    );
    // The producing analysis takes a whole row between the query and the typed
    // pair: its option strings are the longest in the panel, and it is the one
    // control that changes what the tree contains rather than how it reads.
    let producer = producer_rect.expect("the browser scopes itself to a producing analysis");
    assert!(
        producer.top() > search.bottom(),
        "the analysis facet sits under the query ({} vs {})",
        producer.top(),
        search.bottom()
    );
    assert!(
        (producer.width() - search.width()).abs() <= 1.5,
        "the analysis facet takes the whole row ({} vs {})",
        producer.width(),
        search.width()
    );
    assert_eq!(facet_rects.len(), 2, "kind and sort facets");
    facet_rects.sort_by(|a, b| a.left().total_cmp(&b.left()));
    let [kind, sort] = [facet_rects[0], facet_rects[1]];
    // The pair splits unevenly: kind carries the long option names, sort three
    // short ones, so neither list clips at the dock's own width.
    let pair = kind.width() + sort.width();
    assert!(
        (kind.width() / pair - 0.554).abs() <= 0.01,
        "kind takes the wider share of the pair ({} vs {})",
        kind.width(),
        sort.width()
    );
    assert!(
        (kind.top() - sort.top()).abs() <= 1.0,
        "the facets share one row"
    );
    assert!(
        kind.top() > producer.bottom(),
        "the typed pair sits under the analysis facet, as one toolbar block"
    );
    // The whole toolbar shares one inset, so the pair reads as a control
    // row belonging to the field above it at any dock width.
    assert!(
        (kind.left() - search.left()).abs() <= 1.0,
        "the facet row starts on the query's inset ({} vs {})",
        kind.left(),
        search.left()
    );
    assert!(
        (sort.right() - search.right()).abs() <= 1.5,
        "the facet row ends on the query's inset ({} vs {})",
        sort.right(),
        search.right()
    );
}

#[test]
fn verification_flow_rows_match_the_mock_grid_without_text_collisions() {
    assert_eq!(FLOW_ROW_HEIGHT, 63.0);
    assert_eq!(FLOW_TEXT_LEFT, 35.0);
    assert_eq!(FLOW_LABEL_TOP, 7.0);
    assert_eq!(FLOW_DETAIL_TOP, 23.0);
    assert_eq!(FLOW_STATUS_TOP, 43.0);
    assert!(FLOW_DETAIL_TOP - (FLOW_LABEL_TOP + 11.0) >= 4.0);
    assert!(FLOW_STATUS_TOP - (FLOW_DETAIL_TOP + 11.0) >= 4.0);
    assert!(FLOW_ROW_HEIGHT - (FLOW_STATUS_TOP + 11.0) >= 7.0);
    assert_eq!(flow_row_geometry(1), (63.0, 43.0));
    assert_eq!(flow_row_geometry(2), (78.0, 58.0));
    assert_eq!(flow_row_geometry(3), (93.0, 73.0));
}

#[test]
fn verification_navigation_exposes_the_operational_tuning_route() {
    assert_eq!(VerificationPage::NAVIGATION.len(), 6);
    let labels = VerificationPage::NAVIGATION.map(verification_flow_label);
    assert_eq!(
        labels,
        [
            "PVT & Monte Carlo",
            "Process corners",
            "Parameter tuning sandbox",
            "Optimization",
            "Electrical reliability & SOA",
            "Regression · main",
        ]
    );
    assert!(!VerificationPage::NAVIGATION.contains(&VerificationPage::Drc));
    assert!(!VerificationPage::Drc.is_operational());
}

#[test]
fn specification_mapping_does_not_claim_execution_without_an_active_dataset() {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.specs.push(crate::state::SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(1.0),
        max: None,
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    });
    app.state.simulation.active_run_idx = None;

    let coverage = verification_coverage(&app);

    assert_eq!(coverage.mapped, 1);
    assert_eq!(coverage.executed, 0);
    assert_eq!(coverage.passed, 0);
    assert_eq!(coverage.gaps, 1);
    assert_ne!(coverage.status, "Coverage current for active dataset");
}

#[test]
fn empty_specification_set_is_neutral_not_healthy() {
    let app = RSpiceApp::test_instance();
    let coverage = verification_coverage(&app);

    assert_eq!(coverage.total, 0);
    assert_eq!(coverage.gaps, 0);
    assert_eq!(coverage.status, "No project specifications configured");
    assert_ne!(coverage.status, "Coverage current for active dataset");
}

#[test]
fn finite_goal_miss_counts_as_executed_but_not_passed() {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.specs.push(crate::state::SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(41.0),
        max: None,
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    });
    let mut measurement = rspice_core::MeasureResult::success("gain", 40.0);
    measurement.passed = false;
    measurement.error = Some("value misses GOAL".to_owned());
    let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC")
        .with_measurements(vec![measurement])
        .with_provenance(
            crate::state::AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x7b; 32]),
                Vec::new(),
            )
            .expect("test provenance is valid"),
        );
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    app.state.simulation.runs = vec![run];
    app.state.simulation.active_run_idx = Some(0);

    let coverage = verification_coverage(&app);
    assert_eq!(coverage.executed, 1);
    assert_eq!(coverage.passed, 0);
    assert_eq!(coverage.gaps, 0);
}

#[test]
fn verification_navigator_scrolls_when_flows_exceed_compact_height() {
    assert!(verification_navigator_requires_scroll(503.0));
    assert!(!verification_navigator_requires_scroll(504.0));
    assert!(verification_navigator_requires_scroll(390.0));
    assert!(!verification_navigator_requires_scroll(560.0));
    assert!(!verification_navigator_requires_scroll(700.0));
}

#[test]
fn capability_policy_banner_matches_mock_spacing() {
    assert_eq!(CAPABILITY_BANNER_MARGIN, 8);
    assert_eq!(CAPABILITY_BANNER_ICON_SIZE, 15.0);
    assert_eq!(CAPABILITY_BANNER_GAP, 7.0);
}

#[test]
fn mc_sample_trail_is_visible_only_for_its_active_provenance_dataset() {
    let source = SimulationRun::new(1);
    let source_run_id = source.run_id;
    let source_dataset_id = source.dataset_id;
    let other = SimulationRun::new(2);
    let mut simulation = SimulationState {
        runs: vec![source, other],
        active_run_idx: Some(0),
        ..SimulationState::default()
    };
    simulation.replace_yield_evidence(
        vec![result(vec![true, false, true])],
        Some(YieldAnalysisProvenance {
            source_run_id,
            source_dataset_id,
            seed: 7,
            runs_requested: 3,
            runs_completed: 3,
            sampling_mode: MonteCarloSamplingMode::PseudoRandom,
        }),
    );

    assert_eq!(active_mc_sample_trail(&simulation), 3);
    simulation.active_run_idx = Some(1);
    assert_eq!(active_mc_sample_trail(&simulation), 0);
    simulation.active_run_idx = None;
    assert_eq!(active_mc_sample_trail(&simulation), 0);
}

#[test]
fn result_navigator_preserves_desktop_density_and_expands_touch_controls() {
    assert_eq!(
        responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, 32.0),
        EXPRESSION_HEADER_HEIGHT
    );
    assert_eq!(
        responsive_result_control_height(SIGNAL_ROW_HEIGHT, 32.0),
        SIGNAL_ROW_HEIGHT
    );
    assert_eq!(
        responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, TOUCH_TARGET_HEIGHT),
        TOUCH_TARGET_HEIGHT
    );
    assert_eq!(
        responsive_result_control_height(SIGNAL_ROW_HEIGHT, TOUCH_TARGET_HEIGHT),
        TOUCH_TARGET_HEIGHT
    );
}

fn result_navigator_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Results;
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
    ]);
    let ac =
        AnalysisResult::new(2, AnalysisType::Ac, "AC").with_waveforms(vec![WaveformData::new(
            "V(in)",
            vec![1.0, 10.0],
            vec![1.0, 0.5],
            "#55aaff",
        )]);
    let mut run = SimulationRun::new(1);
    run.add_analysis(transient);
    run.add_analysis(ac);
    app.state.simulation.runs = vec![run];
    app.state.simulation.active_run_idx = None;
    app.state.simulation.active_analysis_idx = None;
    app
}

#[test]
fn result_navigator_selection_preserves_dataset_and_visibility_invariants() {
    let mut app = result_navigator_app();
    let dataset_id = app.state.simulation.runs[0].dataset_id;

    assert!(select_result_dataset(&mut app, 0));
    assert_eq!(app.state.simulation.active_run_idx, Some(0));
    assert_eq!(app.state.simulation.active_analysis_idx, Some(0));
    assert_eq!(
        app.state.workbench.documents.active(Workspace::Results),
        Some(&crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id))
    );

    let was_visible = app.state.simulation.runs[0].analyses[0].waveforms[0].visible;
    assert!(select_result_signal(&mut app, 0, 0, 0));
    let selected = app
        .state
        .ui
        .results
        .valid_selected_trace(&app.state.simulation)
        .expect("signal selection resolves against the active dataset");
    assert_eq!(selected.source_name(), "V(out)");
    crate::workbench::documents::result_document::prepare_viewer_state(&mut app);
    assert_eq!(
        app.state
            .ui
            .results
            .valid_selected_trace(&app.state.simulation)
            .map(|selected| selected.source_name()),
        Some("V(out)"),
        "the first Results render after activation must retain the navigator selection"
    );
    assert_eq!(
        app.state.simulation.runs[0].analyses[0].waveforms[0].visible,
        was_visible
    );

    assert!(select_result_analysis(&mut app, 0, 1));
    assert_eq!(app.state.simulation.active_run_idx, Some(0));
    assert_eq!(app.state.simulation.active_analysis_idx, Some(1));
    assert!(
        app.state
            .ui
            .results
            .valid_selected_trace(&app.state.simulation)
            .is_none(),
        "changing analysis clears the now-invalid signal selection"
    );
}

#[test]
fn schematic_navigator_uses_the_upgraded_compact_tree_metrics() {
    assert_eq!(super::SCHEMATIC_NAV_ROW_HEIGHT, 24.0);
    assert_eq!(super::SCHEMATIC_NAV_LABEL_SIZE, 12.0);
    assert_eq!(super::SCHEMATIC_NAV_META_SIZE, 10.0);
}

/// The metadata register names the kind through the same accessor
/// authority the unit column uses, so a derived projection never reports
/// the kind of the wrapper instead of the quantity underneath it.
#[test]
fn quantity_metadata_names_the_kind_under_derived_projections() {
    for (name, unit, kind) in [
        ("V(out)", "V", "Voltage"),
        ("I(C1)", "A", "Current"),
        ("|I(VIN)|", "A", "Current"),
        ("phase(V(OUT))", "°", "Phase"),
        ("phase(I(VIN))", "°", "Current phase"),
        ("re(I(R1))", "A", "Current"),
        ("onoise", "V^2/Hz", "Noise PSD"),
    ] {
        assert_eq!(
            super::result_quantity_kind_label(name, unit),
            kind,
            "{name}"
        );
    }
}

/// A run's quantities open as an index of its analyses: the active
/// analysis is disclosed, the rest stay closed until asked for, and an
/// explicit toggle survives as the session's answer either way.
#[test]
fn analysis_groups_default_to_disclosing_only_the_active_analysis() {
    let ctx = egui::Context::default();
    let run = result_navigator_app().state.simulation.runs.remove(0);
    let first = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let second = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[1]);
    assert!(super::result_browser_group_expanded(&ctx, first, true));
    assert!(!super::result_browser_group_expanded(&ctx, second, false));

    super::set_result_browser_group_expanded(&ctx, second, true);
    super::set_result_browser_group_expanded(&ctx, first, false);
    assert!(super::result_browser_group_expanded(&ctx, second, false));
    assert!(
        !super::result_browser_group_expanded(&ctx, first, true),
        "closing the active analysis must outrank the default"
    );
}

/// A batch plot action is the single-row act repeated: it must reach the
/// same visibility path, skip rows already in the requested state, and
/// leave the checked set intact so a second action can follow.
#[test]
fn batch_plot_action_moves_only_the_checked_quantities() {
    let mut app = result_navigator_app();
    assert!(select_result_dataset(&mut app, 0));
    let run = &app.state.simulation.runs[0];
    let checked_key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        "V(out)",
    );
    let untouched_key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[1]),
        "V(in)",
    );
    let checked = [ResultBrowserSelectionKey::Waveform(checked_key.clone())]
        .into_iter()
        .collect();
    let source_default = run.analyses[0].waveforms[0].visible;
    let untouched_default = run.analyses[1].waveforms[0].visible;

    super::set_checked_signal_visibility(&mut app, &checked, false);
    assert!(
        !app.state
            .ui
            .results
            .waveform_visibility(&checked_key, source_default),
        "the checked quantity leaves its pane"
    );
    assert_eq!(
        app.state
            .ui
            .results
            .waveform_visibility(&untouched_key, untouched_default),
        untouched_default,
        "an unchecked quantity in another analysis is not disturbed"
    );
    assert_eq!(
        app.state.simulation.runs[0].analyses[0].waveforms[0].visible, source_default,
        "browser presentation never mutates immutable retained evidence"
    );

    super::set_checked_signal_visibility(&mut app, &checked, true);
    assert!(
        app.state
            .ui
            .results
            .waveform_visibility(&checked_key, source_default),
        "the same act returns it"
    );
    // Idempotent: repeating the request cannot toggle it back off.
    super::set_checked_signal_visibility(&mut app, &checked, true);
    assert!(
        app.state
            .ui
            .results
            .waveform_visibility(&checked_key, source_default)
    );
}

#[test]
fn sample_counts_group_thousands_for_reading_as_magnitudes() {
    assert_eq!(super::grouped_count(0), "0");
    assert_eq!(super::grouped_count(151), "151");
    assert_eq!(super::grouped_count(15_001), "15,001");
    assert_eq!(super::grouped_count(100_008), "100,008");
    assert_eq!(super::grouped_count(1_000_000), "1,000,000");
}

#[test]
fn browser_rows_keep_the_mockup_two_register_metrics() {
    assert_eq!(super::RESULT_QUANTITY_ROW_HEIGHT, 36.0);
    assert_eq!(super::RESULT_ANALYSIS_HEAD_HEIGHT, 31.0);
    assert_eq!(super::RESULT_BROWSER_VIRTUALIZATION_THRESHOLD, 250);
    assert_eq!(super::RESULT_BROWSER_RENDER_WINDOW_ROWS, 120);
    assert_eq!(super::RESULT_BROWSER_OVERSCAN_ROWS, 20);
    // Touch stages still promote both to the full target height.
    assert_eq!(
        super::responsive_result_control_height(
            super::RESULT_QUANTITY_ROW_HEIGHT,
            super::TOUCH_TARGET_HEIGHT
        ),
        super::TOUCH_TARGET_HEIGHT
    );
}

/// The typed pair stays side by side at every width the navigator dock can be
/// dragged to, and stacks only in the narrower touch drawers. Stacking inside
/// the dock's own range turns the toolbar into a column of selects at ordinary
/// widths, which is the layout the mockup exists to replace.
#[test]
fn browser_facets_stay_paired_across_the_whole_dock_range() {
    // The dock clamps to 220..=440 px; the toolbar insets take 16 of those.
    for dock_width in [220.0_f32, 255.0, 300.0, 440.0] {
        assert!(
            !super::result_browser_facets_stack(dock_width - super::PANEL_SEARCH_MARGIN_X * 2.0),
            "the pair must stay side by side at a {dock_width} px dock"
        );
    }
    assert!(super::result_browser_facets_stack(
        super::RESULT_BROWSER_STACKED_FACET_MAX_WIDTH - 1.0
    ));
    assert!(!super::result_browser_facets_stack(
        super::RESULT_BROWSER_STACKED_FACET_MAX_WIDTH
    ));
    assert!(!super::result_browser_facets_stack(420.0));
}

#[test]
fn exact_browser_copy_uses_round_trip_values_and_source_provenance() {
    let app = result_navigator_app();
    let run = &app.state.simulation.runs[0];
    let key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        "V(out)",
    );
    let tsv = super::exact_result_signal_tsv(&key, &app.state.simulation.runs)
        .expect("exact source waveform resolves");
    assert!(tsv.contains(&format!("# dataset\t{}", run.dataset_id)));
    assert!(tsv.contains("# analysis\tTRAN"));
    assert!(tsv.contains("# quantity\tV(out)"));
    assert!(tsv.contains("sample\tx\ty\n"));
    assert!(tsv.contains("1\t1.00000000000000000e0\t1.00000000000000000e0"));

    let last = super::exact_result_signal_last_sample(&key, &app.state.simulation.runs)
        .expect("last retained sample exists");
    assert_eq!(
        last,
        "V(out)\tx=1.00000000000000000e0\ty=1.00000000000000000e0"
    );
}

#[test]
fn exact_browser_copy_fails_closed_on_incoherent_vectors() {
    let mut app = result_navigator_app();
    let run = &mut app.state.simulation.runs[0];
    run.analyses[0].waveforms[0].y = std::sync::Arc::new(vec![0.0]);
    let key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        "V(out)",
    );
    let error = super::exact_result_signal_tsv(&key, &app.state.simulation.runs)
        .expect_err("mismatched exact vectors must not be truncated");
    assert!(
        error.contains("retained waveform 'V(out)' has 2 coordinates but 1 values"),
        "{error}"
    );
}

#[test]
fn browser_range_selection_uses_stable_filtered_order() {
    let mut app = result_navigator_app();
    let run = &app.state.simulation.runs[0];
    let first = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        "V(out)",
    );
    let second = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[1]),
        "V(in)",
    );
    let first_selection = ResultBrowserSelectionKey::Waveform(first.clone());
    let second_selection = ResultBrowserSelectionKey::Waveform(second.clone());
    let visible = vec![first_selection.clone(), second_selection.clone()];
    app.state
        .ui
        .results
        .set_browser_range_anchor(first_selection);
    app.state
        .ui
        .results
        .select_checked_result_range(&second_selection, &visible);
    assert!(app.state.ui.results.is_checked_signal(&first));
    assert!(app.state.ui.results.is_checked_signal(&second));
}

/// The browser offers one facet per independent question. Kind carries the
/// typed shape classes and closes with a single negation over the base
/// electrical dimensions, which is the whole of what a separate unit control
/// used to ask.
#[test]
fn browser_kind_facet_admits_only_its_declared_inventory_classes() {
    use super::{ResultArtifactKind, ResultsBrowserKind};

    assert!(ResultsBrowserKind::Voltage.admits("V"));
    assert!(!ResultsBrowserKind::Voltage.admits("V^2/Hz"));
    assert!(ResultsBrowserKind::NoiseDensity.admits("nV/√Hz"));
    assert!(ResultsBrowserKind::Scalar.admits_artifact(ResultArtifactKind::Scalar, "V"));
    assert!(ResultsBrowserKind::Array.admits_artifact(ResultArtifactKind::Array, "V"));
    assert!(ResultsBrowserKind::EventStream.admits_artifact(ResultArtifactKind::EventStream, ""));
    assert!(
        ResultsBrowserKind::Contribution.admits_artifact(ResultArtifactKind::Contribution, "%")
    );
    assert!(!ResultsBrowserKind::Voltage.admits_artifact(ResultArtifactKind::Scalar, "V"));
    for unit in ResultsBrowserKind::BASE_UNITS {
        assert!(
            !ResultsBrowserKind::OtherUnits.admits(unit),
            "{unit} is a base electrical dimension, not an other unit"
        );
        assert!(!ResultsBrowserKind::OtherUnits.admits_artifact(ResultArtifactKind::Scalar, unit));
    }
    assert!(ResultsBrowserKind::OtherUnits.admits("Ω"));
    assert!(ResultsBrowserKind::OtherUnits.admits(""));
    assert!(ResultsBrowserKind::OtherUnits.admits_artifact(ResultArtifactKind::Scalar, "°"));
}

/// Data state is one merged reading, severity first, so a caller can never
/// read integrity and completeness as two verdicts on the same evidence check.
#[test]
fn analysis_data_state_merges_one_reading_worst_first() {
    use super::{ResultCompletenessClass, result_analysis_data_state};
    use crate::workbench::documents::result_document::operational_state::ResultCurrentness;

    let word =
        |currentness, completeness| result_analysis_data_state(currentness, completeness).word;

    // An unreadable payload outranks every completeness reading.
    for completeness in [
        ResultCompletenessClass::Complete,
        ResultCompletenessClass::Partial,
        ResultCompletenessClass::Loading,
        ResultCompletenessClass::Failed,
        ResultCompletenessClass::Cancelled,
    ] {
        assert_eq!(
            word(ResultCurrentness::Corrupted, completeness),
            "corrupted"
        );
    }
    // A short or interrupted scope outranks any claim about the source revision.
    assert_eq!(
        word(ResultCurrentness::Current, ResultCompletenessClass::Failed),
        "failed"
    );
    assert_eq!(
        word(ResultCurrentness::Stale, ResultCompletenessClass::Cancelled),
        "cancelled"
    );
    assert_eq!(
        word(ResultCurrentness::Current, ResultCompletenessClass::Loading),
        "loading"
    );
    assert_eq!(
        word(ResultCurrentness::Current, ResultCompletenessClass::Partial),
        "partial"
    );
    // Only a complete, readable analysis reports its currentness.
    assert_eq!(
        word(ResultCurrentness::Stale, ResultCompletenessClass::Complete),
        "stale"
    );
    assert_eq!(
        word(
            ResultCurrentness::Superseded,
            ResultCompletenessClass::Complete
        ),
        "superseded"
    );
    assert_eq!(
        word(
            ResultCurrentness::Unresolved,
            ResultCompletenessClass::Complete
        ),
        "unresolved"
    );
    assert_eq!(
        word(
            ResultCurrentness::Recovered,
            ResultCompletenessClass::Complete
        ),
        "recovered"
    );
    assert_eq!(
        word(
            ResultCurrentness::Current,
            ResultCompletenessClass::Complete
        ),
        "current"
    );
    // Every reading carries its own consequence, so no caller has to look one
    // up from the other's display text.
    for currentness in ResultCurrentness::ALL {
        let state = result_analysis_data_state(currentness, ResultCompletenessClass::Complete);
        assert!(
            !state.note.is_empty(),
            "{} states no consequence",
            state.word
        );
    }
}

#[test]
fn dataset_disclosure_is_bound_to_immutable_dataset_identity() {
    let ctx = egui::Context::default();
    let first = crate::product::DatasetId::new();
    let second = crate::product::DatasetId::new();

    assert!(super::result_browser_dataset_expanded(&ctx, first, true));
    assert!(!super::result_browser_dataset_expanded(&ctx, second, false));
    super::set_result_browser_dataset_expanded(&ctx, first, false);
    super::set_result_browser_dataset_expanded(&ctx, second, true);
    assert!(!super::result_browser_dataset_expanded(&ctx, first, true));
    assert!(super::result_browser_dataset_expanded(&ctx, second, false));
}

#[test]
fn typed_scalar_payload_is_a_stable_exact_browser_artifact() {
    let mut values = std::collections::BTreeMap::new();
    values.insert("settling_time".to_owned(), 1.0 / 3.0);
    let mut analysis = AnalysisResult::new(41, AnalysisType::Transient, "TRAN typed");
    analysis.result_payload = Some(AnalysisResultPayload::ScalarMeasurements { values });
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    let presentation = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let artifacts = super::retained_result_artifacts(&run.analyses[0], presentation);
    let artifact = artifacts
        .iter()
        .find(|artifact| artifact.identity.canonical_name() == "payload/scalar-measurements")
        .expect("scalar payload is inventoried");
    assert_eq!(artifact.kind, super::ResultArtifactKind::Scalar);

    let key = ResultArtifactPresentationKey::new(presentation, "payload/scalar-measurements");
    let exact = super::exact_result_artifact_text(&key, std::slice::from_ref(&run))
        .expect("typed scalar exact adapter resolves");
    assert!(exact.contains("# rspice-result-artifact-v1"));
    assert!(exact.contains("analysis-result-payload-v1"));
    assert!(exact.contains("settling_time"));
    assert!(exact.contains("0.3333333333333333"));

    let selected = ResultBrowserSelectionKey::Artifact(key);
    let bundle =
        crate::workbench::documents::result_document::exact_result_browser_selection_bundle(
            &[selected],
            &[run],
        )
        .expect("the batch adapter resolves the same typed evidence");
    assert!(bundle.contains("# item-count\t1"));
    assert!(bundle.contains("payload/scalar-measurements"));
    assert!(bundle.contains("settling_time"));
}

/// "Reveal producer log" has to reveal something.
///
/// It used to push one info line naming the producer and open the console
/// unfiltered, which left the reader to find that producer's entries in the
/// whole session's log by eye.
#[test]
fn revealing_a_producer_log_narrows_the_console_to_that_producer() {
    use crate::diagnostics::{LogSeverity, LogSource};

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.console_visible = false;
    app.state.workbench.console_page = crate::workbench::state::ConsolePage::Problems;
    app.state.log_buffer.clear();
    app.state.log_buffer.log(
        LogSeverity::Info,
        LogSource::Simulation,
        "  gain = 1.234000e1",
        None,
    );
    app.state.log_buffer.log(
        LogSeverity::Info,
        LogSource::Simulation,
        "Transient: 512 points, 3 waveforms",
        None,
    );

    crate::workbench::commands::result_navigation::reveal_producer_log(
        &mut app,
        "dataset/7/analysis/3/artifact/gain".to_owned(),
        "gain",
    );

    assert!(app.state.workbench.console_visible);
    assert_eq!(
        app.state.workbench.console_page,
        crate::workbench::state::ConsolePage::Console
    );
    let filter = app
        .state
        .workbench
        .console_producer_filter
        .as_ref()
        .expect("the row installs a producer filter");
    assert_eq!(filter.producer, "dataset/7/analysis/3/artifact/gain");
    assert_eq!(filter.label(), "gain");
    assert!(
        filter.scroll_to_newest,
        "the reader asked to be shown the newest entry"
    );
    let matched: Vec<&str> = app
        .state
        .log_buffer
        .entries()
        .filter(|entry| filter.matches(entry))
        .map(|entry| entry.message.as_str())
        .collect();
    assert_eq!(matched, vec!["  gain = 1.234000e1"]);
}

/// An entry tagged with the producer's own identity matches regardless of
/// what it says. Nothing emits that tag today, which is exactly why the rule
/// has to exist before an emission site can be worth adding.
#[test]
fn a_producer_tagged_entry_matches_without_naming_its_quantity() {
    use crate::diagnostics::{LogSeverity, LogSource};

    let mut app = RSpiceApp::test_instance();
    app.state.log_buffer.clear();
    app.state.log_buffer.log(
        LogSeverity::Info,
        LogSource::Engine,
        "solved in 4 iterations",
        Some("dataset/7/analysis/3/quantity/V(out)".to_owned()),
    );
    let filter = crate::workbench::state::ConsoleProducerFilter::new(
        "dataset/7/analysis/3/quantity/V(out)",
        "V(out)",
    );

    assert_eq!(
        app.state
            .log_buffer
            .entries()
            .filter(|entry| filter.matches(entry))
            .count(),
        1
    );
}

/// The simulation rail derives what it states once per frame.
///
/// The Run set row states the resolved point count and the Run set card below
/// states it again, and each asked the run set for it — two validations of one
/// declaration to print one number twice. The Excitations row states how many
/// sources the sheet places and resolved the whole placed-source list to read
/// its length, which walks the design's nets, parses every source's parameters
/// and sorts the result.
///
/// Neither is visible in anything the dock paints, which is why it is counted.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_simulation_rail_derives_what_it_states_once_a_frame() {
    use crate::simulation::cost_probe::{Derivation, count, reset};

    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.activate(Workspace::Simulate);

    reset();
    let painted = navigator_painted_lines(228.0, app);
    assert!(
        painted.iter().any(|(text, _)| text == "RUN SET"),
        "the frame drew the simulation rail"
    );

    assert_eq!(
        count(Derivation::RunSetValidation),
        1,
        "the rail validates its declaration once and reads both numbers off it"
    );
    assert_eq!(
        count(Derivation::PlacedSources),
        0,
        "the rail counts placed sources without resolving them"
    );
}

/// The simulation rail's filter reaches every line it paints.
///
/// It used to reach the nine route rows and stop there. Typing a corner name
/// hid all nine and left the Run set axes, the Variation row and the standing
/// capability notice sitting under a header for a section whose rows had all
/// gone — a rail that answered a search with everything the search was not
/// about.
///
/// Section headers are claims about what is under them, so one that retains no
/// row goes with its rows. The creating action at the foot of the tree is not
/// a datum and stays: it is how an empty plan is filled, including the plan a
/// filter has emptied on screen.
#[test]
fn the_simulate_rail_filter_reaches_past_its_page_rows() {
    fn rail(query: &str) -> Vec<String> {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Simulate);
        app.state.workbench.set_navigator_filter(query);
        navigator_painted_lines(228.0, app)
            .into_iter()
            .map(|(text, _)| text)
            .collect()
    }

    let unfiltered = rail("");
    for line in [
        "LAB CHARACTERIZATION",
        "Analyses",
        "RUN SET",
        "Supply voltage",
        "Variation",
        "CAPABILITY POLICY",
    ] {
        assert!(
            unfiltered.iter().any(|painted| painted.as_str() == line),
            "an unfiltered rail still paints {line:?}; it painted {unfiltered:?}"
        );
    }

    // Matches one axis by name and nothing else the rail holds.
    let filtered = rail("supply");
    assert!(
        filtered
            .iter()
            .any(|painted| painted.as_str() == "Supply voltage"),
        "the axis whose name matches survives; the rail painted {filtered:?}"
    );
    assert!(
        filtered.iter().any(|painted| painted.as_str() == "RUN SET"),
        "and the section that kept a row keeps its header; it painted {filtered:?}"
    );
    for dropped in [
        "Variation",
        "Analyses",
        "LAB CHARACTERIZATION",
        "CAPABILITY POLICY",
    ] {
        assert!(
            !filtered.iter().any(|painted| painted.as_str() == dropped),
            "{dropped:?} matches nothing the reader typed; the rail painted {filtered:?}"
        );
    }
    assert!(
        filtered
            .iter()
            .any(|painted| painted.starts_with("Add analysis")),
        "the panel's own creating action is not a datum the filter can hide; \
         the rail painted {filtered:?}"
    );
}

/// A result artifact's menu is reachable without a pointer: the browser's rows
/// are in the tab ring and Shift+F10 opens the focused row's menu. A keyboard
/// open has no pointer position for the menu to sit at, so the row anchors it —
/// on every frame it stays open, not only the one that opened it, or the menu
/// would paint once and vanish before it could be read.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_artifact_menu_opens_from_the_keyboard_on_the_focused_row() {
    /// The row the keyboard is on, as the accessibility tree reports it.
    fn focused_label(output: &egui::FullOutput) -> Option<String> {
        let update = output.platform_output.accesskit_update.as_ref()?;
        update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .and_then(|(_, node)| node.label().map(str::to_owned))
    }

    /// Every painted run of text in the frame, with where it landed.
    fn painted_runs(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
        fn walk(shape: &egui::epaint::Shape, into: &mut Vec<(String, egui::Rect)>) {
            match shape {
                egui::epaint::Shape::Text(painted) => into.push((
                    painted.galley.job.text.clone(),
                    egui::Rect::from_min_size(painted.pos, painted.galley.size()),
                )),
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        walk(shape, into);
                    }
                }
                _ => {}
            }
        }
        let mut runs = Vec::new();
        for clipped in &output.shapes {
            walk(&clipped.shape, &mut runs);
        }
        runs
    }

    let mut app = result_navigator_app();
    let mut values = std::collections::BTreeMap::new();
    values.insert("settling_time".to_owned(), 0.5);
    app.state.simulation.runs[0].analyses[0].result_payload =
        Some(AnalysisResultPayload::ScalarMeasurements { values });
    assert!(select_result_dataset(&mut app, 0));

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    // A closure over the app rather than a helper handed it, so the frame
    // harness costs the whole-application-access ratchet nothing.
    let mut browser_output = |events: Vec<egui::Event>| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(260.0, 1600.0),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        ui.set_width(260.0);
                        super::show(ui, &mut app);
                    });
            },
        )
    };
    let _ = browser_output(Vec::new());
    let _ = browser_output(Vec::new());

    let key = |key, modifiers| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    };
    // Walk the keyboard onto the artifact row the way a reader without a
    // pointer reaches it. The check cell announces "Select …", so only the
    // selection half of the row carries the artifact's own name.
    let mut landed = false;
    for _ in 0..80 {
        let output = browser_output(vec![key(egui::Key::Tab, egui::Modifiers::NONE)]);
        if focused_label(&output)
            .as_deref()
            .is_some_and(|label| label.starts_with("Scalar result values"))
        {
            landed = true;
            break;
        }
    }
    assert!(
        landed,
        "an artifact row must be reachable from the keyboard"
    );

    let _ = browser_output(vec![key(egui::Key::F10, egui::Modifiers::SHIFT)]);
    // The frame after the opening one is the load-bearing frame: a menu whose
    // anchor only lives on the opening frame is gone by now.
    let output = browser_output(Vec::new());
    let runs = painted_runs(&output);
    let copy = runs
        .iter()
        .find(|(run, _)| run == "Copy canonical name")
        .map(|(_, rect)| *rect)
        .unwrap_or_else(|| {
            panic!(
                "Shift+F10 keeps the focused row's menu open; the frame painted {:?}",
                runs.iter().map(|(run, _)| run.as_str()).collect::<Vec<_>>()
            )
        });

    let click = |at: egui::Pos2| {
        vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ]
    };
    let output = browser_output(click(copy.center()));
    assert!(
        output.platform_output.commands.iter().any(|command| matches!(
            command,
            egui::OutputCommand::CopyText(text) if text.as_str() == "payload/scalar-measurements"
        )),
        "and the menu acts on the row the keyboard was on"
    );
}
