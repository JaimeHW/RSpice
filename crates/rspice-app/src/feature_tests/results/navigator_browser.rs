//! The Results navigator's data browser, drawn in the whole navigator: what it
//! lists, discloses, filters, selects, plots and copies, and how dense its rows
//! are on desktop and touch.

use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisType, RetainedTrace, SimulationRun,
};
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::AnalysisPresentationKey;
use crate::workbench::state::Workspace;
use rspice_app_results::workbench::docks::navigator::results_actions::{
    select_result_analysis, select_result_dataset, select_result_signal,
};
use rspice_app_results::workbench::docks::navigator::results_rows::{
    EXPRESSION_HEADER_HEIGHT, SIGNAL_ROW_HEIGHT, TOUCH_TARGET_HEIGHT,
    responsive_result_control_height,
};
use rspice_app_results::workbench::docks::navigator::{results, results_actions, results_rows};
use rspice_app_workbench::workbench::docks::navigator;
use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, SourceWaveformPresentationKey,
};
use rspice_results_ui::selection::ResultBrowserSelectionKey;

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
            navigator::show(ui, &mut app);
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
            && node.label() == Some("Find signal…")
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
    // A 48 px target chosen in Accessibility is the height, not the floor.
    assert_eq!(
        responsive_result_control_height(SIGNAL_ROW_HEIGHT, 48.0),
        48.0
    );
}

fn result_navigator_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Results;
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
    ]);
    let ac =
        AnalysisResult::new(2, AnalysisType::Ac, "AC").with_waveforms(vec![RetainedTrace::new(
            "V(in)",
            vec![1.0, 10.0],
            vec![1.0, 0.5],
            "#55aaff",
        )]);
    let mut run = SimulationRun::new(1);
    run.add_analysis(transient);
    run.add_analysis(ac);
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.model.simulation.view.active_run_idx = None;
    app.state.model.simulation.view.active_analysis_idx = None;
    app
}

/// The scope band states a count only while a filter is holding rows back,
/// and the selection band belongs to a selection.
///
/// Unfiltered, "2 / 2" repeated the tab band's own count under it, and a
/// permanent 29 px strip carried one rarely used command across a browser
/// whose every row has a checkbox.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_data_browser_counts_only_what_a_filter_holds_back() {
    fn painted(app: RSpiceApp) -> Vec<String> {
        navigator_painted_lines(256.0, app)
            .into_iter()
            .map(|(text, _)| text)
            .collect()
    }

    let shown = painted(result_navigator_app());
    assert!(
        !shown.iter().any(|line| line.contains(" / ")),
        "nothing is held back, so no fraction: {shown:?}"
    );
    assert!(
        !shown.iter().any(|line| line.starts_with("Select visible")),
        "nothing is checked, so no selection band: {shown:?}"
    );

    let mut app = result_navigator_app();
    app.state.workbench.set_navigator_filter("V(out)");
    let filtered = painted(app);
    assert!(
        filtered.iter().any(|line| line == "1 / 2"),
        "one of two rows survives the query, and the band says so: {filtered:?}"
    );
}

#[test]
fn result_navigator_selection_preserves_dataset_and_visibility_invariants() {
    let mut app = result_navigator_app();
    let dataset_id = app.state.model.simulation.retained.runs[0].dataset_id;

    assert!(select_result_dataset(&mut app.state.results_parts(), 0));
    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(0));
    assert_eq!(app.state.model.simulation.view.active_analysis_idx, Some(0));
    assert_eq!(
        app.state.workbench.documents.active(Workspace::Results),
        Some(&crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id))
    );

    let was_visible = app.state.model.simulation.retained.runs[0].analyses[0].waveforms[0].visible;
    assert!(select_result_signal(
        &mut app.state.results_parts(),
        0,
        0,
        0
    ));
    let selected = app
        .state
        .results
        .results
        .valid_selected_trace(&app.state.model.simulation)
        .expect("signal selection resolves against the active dataset");
    assert_eq!(selected.source_name(), "V(out)");
    crate::workbench::documents::result_document::prepare_viewer_state(
        &mut app.state.results_parts(),
    );
    assert_eq!(
        app.state
            .results
            .results
            .valid_selected_trace(&app.state.model.simulation)
            .map(|selected| selected.source_name()),
        Some("V(out)"),
        "the first Results render after activation must retain the navigator selection"
    );
    assert_eq!(
        app.state.model.simulation.retained.runs[0].analyses[0].waveforms[0].visible,
        was_visible
    );

    assert!(select_result_analysis(&mut app.state.results_parts(), 0, 1));
    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(0));
    assert_eq!(app.state.model.simulation.view.active_analysis_idx, Some(1));
    assert!(
        app.state
            .results
            .results
            .valid_selected_trace(&app.state.model.simulation)
            .is_none(),
        "changing analysis clears the now-invalid signal selection"
    );
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
            results_rows::result_quantity_kind_label(name, unit),
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
    let run = result_navigator_app()
        .state
        .model
        .simulation
        .retained
        .runs
        .remove(0);
    let first = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let second = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[1]);
    assert!(results::result_browser_group_expanded(&ctx, first, true));
    assert!(!results::result_browser_group_expanded(&ctx, second, false));

    results::set_result_browser_group_expanded(&ctx, second, true);
    results::set_result_browser_group_expanded(&ctx, first, false);
    assert!(results::result_browser_group_expanded(&ctx, second, false));
    assert!(
        !results::result_browser_group_expanded(&ctx, first, true),
        "closing the active analysis must outrank the default"
    );
}

/// A batch plot action is the single-row act repeated: it must reach the
/// same visibility path, skip rows already in the requested state, and
/// leave the checked set intact so a second action can follow.
#[test]
fn batch_plot_action_moves_only_the_checked_quantities() {
    let mut app = result_navigator_app();
    assert!(select_result_dataset(&mut app.state.results_parts(), 0));
    let run = &app.state.model.simulation.retained.runs[0];
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

    results_actions::set_checked_signal_visibility(&mut app.state.results_parts(), &checked, false);
    assert!(
        !app.state
            .results
            .results
            .session
            .waveform_visibility(&checked_key, source_default),
        "the checked quantity leaves its pane"
    );
    assert_eq!(
        app.state
            .results
            .results
            .session
            .waveform_visibility(&untouched_key, untouched_default),
        untouched_default,
        "an unchecked quantity in another analysis is not disturbed"
    );
    assert_eq!(
        app.state.model.simulation.retained.runs[0].analyses[0].waveforms[0].visible,
        source_default,
        "browser presentation never mutates immutable retained evidence"
    );

    results_actions::set_checked_signal_visibility(&mut app.state.results_parts(), &checked, true);
    assert!(
        app.state
            .results
            .results
            .session
            .waveform_visibility(&checked_key, source_default),
        "the same act returns it"
    );
    // Idempotent: repeating the request cannot toggle it back off.
    results_actions::set_checked_signal_visibility(&mut app.state.results_parts(), &checked, true);
    assert!(
        app.state
            .results
            .results
            .session
            .waveform_visibility(&checked_key, source_default)
    );
}

#[test]
fn sample_counts_group_thousands_for_reading_as_magnitudes() {
    assert_eq!(navigator::grouped_count(0), "0");
    assert_eq!(navigator::grouped_count(151), "151");
    assert_eq!(navigator::grouped_count(15_001), "15,001");
    assert_eq!(navigator::grouped_count(100_008), "100,008");
    assert_eq!(navigator::grouped_count(1_000_000), "1,000,000");
}

#[test]
fn browser_rows_keep_the_mockup_two_register_metrics() {
    // Touch stages still promote both to the full target height.
    assert_eq!(
        results_rows::responsive_result_control_height(
            results_rows::RESULT_QUANTITY_ROW_HEIGHT,
            results_rows::TOUCH_TARGET_HEIGHT
        ),
        results_rows::TOUCH_TARGET_HEIGHT
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
            !results::result_browser_facets_stack(
                dock_width - navigator::PANEL_SEARCH_MARGIN_X * 2.0
            ),
            "the pair must stay side by side at a {dock_width} px dock"
        );
    }
    assert!(results::result_browser_facets_stack(
        results::RESULT_BROWSER_STACKED_FACET_MAX_WIDTH - 1.0
    ));
    assert!(!results::result_browser_facets_stack(
        results::RESULT_BROWSER_STACKED_FACET_MAX_WIDTH
    ));
    assert!(!results::result_browser_facets_stack(420.0));
}

#[test]
fn exact_browser_copy_uses_round_trip_values_and_source_provenance() {
    let app = result_navigator_app();
    let run = &app.state.model.simulation.retained.runs[0];
    let key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        "V(out)",
    );
    let tsv = crate::workbench::documents::result_document::exact_result_signal_tsv(
        &key,
        &app.state.model.simulation.retained.runs,
    )
    .expect("exact source waveform resolves");
    assert!(tsv.contains(&format!("# dataset\t{}", run.dataset_id)));
    assert!(tsv.contains("# analysis\tTRAN"));
    assert!(tsv.contains("# quantity\tV(out)"));
    assert!(tsv.contains("sample\tx\ty\n"));
    assert!(tsv.contains("1\t1.00000000000000000e0\t1.00000000000000000e0"));

    // "Copy value" states the x unit and follows Copied values, at full
    // precision either way.
    use rspice_vocabulary::quantity::{CopiedValueFormat, QuantityPresentationPolicy};
    let last = |copied_value_format| {
        let policy = QuantityPresentationPolicy {
            copied_value_format,
            ..QuantityPresentationPolicy::default()
        };
        crate::workbench::documents::result_document::exact_result_signal_last_sample(
            &key,
            &app.state.model.simulation.retained.runs,
            policy,
        )
        .expect("last retained sample exists")
    };
    assert_eq!(
        last(CopiedValueFormat::EngineeringNotationWithUnit),
        "V(out)\tx=1.00000000000000000 s\ty=1.00000000000000000 V"
    );
    assert_eq!(
        last(CopiedValueFormat::ScientificNotationWithSiUnit),
        "V(out)\tx=1.00000000000000000e0 s\ty=1.00000000000000000e0 V"
    );
}

#[test]
fn exact_browser_copy_fails_closed_on_incoherent_vectors() {
    let mut app = result_navigator_app();
    let run = &mut app.state.model.simulation.retained.runs[0];
    run.analyses[0].waveforms[0].y = std::sync::Arc::new(vec![0.0]);
    let key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        "V(out)",
    );
    let error = crate::workbench::documents::result_document::exact_result_signal_tsv(
        &key,
        &app.state.model.simulation.retained.runs,
    )
    .expect_err("mismatched exact vectors must not be truncated");
    assert!(
        error.contains("retained waveform 'V(out)' has 2 coordinates but 1 values"),
        "{error}"
    );
}

#[test]
fn browser_range_selection_uses_stable_filtered_order() {
    let mut app = result_navigator_app();
    let run = &app.state.model.simulation.retained.runs[0];
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
        .results
        .results
        .session
        .set_browser_range_anchor(first_selection);
    app.state
        .results
        .results
        .session
        .select_checked_result_range(&second_selection, &visible);
    assert!(app.state.results.results.session.is_checked_signal(&first));
    assert!(app.state.results.results.session.is_checked_signal(&second));
}

/// The browser offers one facet per independent question. Kind carries the
/// typed shape classes and closes with a single negation over the base
/// electrical dimensions, which is the whole of what a separate unit control
/// used to ask.
#[test]
fn browser_kind_facet_admits_only_its_declared_inventory_classes() {
    use results::{ResultArtifactKind, ResultsBrowserKind};

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
    use crate::workbench::documents::result_document::operational_state::ResultCurrentness;
    use results::result_analysis_data_state;
    use results_rows::ResultCompletenessClass;

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

    assert!(results::result_browser_dataset_expanded(&ctx, first, true));
    assert!(!results::result_browser_dataset_expanded(
        &ctx, second, false
    ));
    results::set_result_browser_dataset_expanded(&ctx, first, false);
    results::set_result_browser_dataset_expanded(&ctx, second, true);
    assert!(!results::result_browser_dataset_expanded(&ctx, first, true));
    assert!(results::result_browser_dataset_expanded(
        &ctx, second, false
    ));
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
    let artifacts = results_rows::retained_result_artifacts(&run.analyses[0], presentation);
    let artifact = artifacts
        .iter()
        .find(|artifact| artifact.identity.canonical_name() == "payload/scalar-measurements")
        .expect("scalar payload is inventoried");
    assert_eq!(artifact.kind, results::ResultArtifactKind::Scalar);

    let key = ResultArtifactPresentationKey::new(presentation, "payload/scalar-measurements");
    let exact = crate::workbench::documents::result_document::exact_result_artifact_text(
        &key,
        std::slice::from_ref(&run),
    )
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

/// A periodic payload's row names the data and states the physics: how many
/// multipliers or modes, the orbit, and the stability verdict. How the
/// spectrum was qualified is the engine's own bookkeeping and used to be
/// spelled out twice on the row ("0 retained multipliers; completeness
/// unavailable · legacy evidence unknown").
#[test]
fn periodic_payload_artifacts_report_their_counts_and_verdicts() {
    for (analysis_type, canonical, expected_name, noun) in [
        (
            AnalysisType::Pss,
            "payload/pss-floquet",
            "Floquet multipliers",
            "multipliers",
        ),
        (
            AnalysisType::Pstb,
            "payload/pstb-floquet",
            "Floquet modes",
            "modes",
        ),
    ] {
        let analysis = AnalysisResult::new(41, analysis_type, expected_name).with_result_payload(
            AnalysisResultPayload::legacy_periodic_marker(analysis_type).unwrap(),
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let presentation = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
        let artifacts = results_rows::retained_result_artifacts(&run.analyses[0], presentation);
        let artifact = artifacts
            .iter()
            .find(|artifact| artifact.identity.canonical_name() == canonical)
            .expect("periodic payload is inventoried");

        assert_eq!(artifact.name, expected_name);
        assert_eq!(artifact.kind, results::ResultArtifactKind::Array);
        assert_eq!(artifact.meta, "0 values");
        assert_eq!(artifact.viewer, crate::workbench::ResultViewer::Table);
        let value = artifact.value.as_deref().unwrap();
        assert!(value.starts_with(&format!("0 {noun}")), "{value}");
        assert!(value.contains("indeterminate"), "{value}");
        for internal in ["retained", "completeness", "legacy", "qualified"] {
            assert!(!value.contains(internal), "{internal} in {value}");
        }
    }
}

/// A quantity's "Open run log" opens the log of the run that produced it, and
/// is not offered once the session no longer holds that run.
#[test]
fn a_quantitys_run_log_is_the_log_of_the_run_that_produced_it() {
    use crate::workbench::commands::result_navigation::{open_run_log, producing_run};
    use crate::workbench::state::{ConsoleChannel, ConsolePage};

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.console_visible = false;
    app.state.workbench.console_page = ConsolePage::Problems;
    let run = SimulationRun::new(7);
    let dataset = run.dataset_id;
    app.state.model.simulation.retained.runs.push(run);

    let producer = producing_run(&app.state.model.simulation.retained.runs, dataset)
        .expect("the session holds the producing run");
    open_run_log(&app.state.model, &mut app.state.workbench, producer);
    assert!(app.state.workbench.console_visible);
    assert_eq!(app.state.workbench.console_page, ConsolePage::Console);
    assert_eq!(
        app.state.workbench.console_view.channel,
        ConsoleChannel::Run(7)
    );

    app.state.model.simulation.retained.runs.clear();
    assert_eq!(
        producing_run(&app.state.model.simulation.retained.runs, dataset),
        None
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
    app.state.model.simulation.retained.runs[0].analyses[0].result_payload =
        Some(AnalysisResultPayload::ScalarMeasurements { values });
    assert!(select_result_dataset(&mut app.state.results_parts(), 0));

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
                        navigator::show(ui, &mut app);
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
        .find(|(run, _)| run == "Copy name")
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
                navigator::show(ui, &mut app);
            });
    });
    let mut lines = Vec::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut lines);
    }
    lines
}
