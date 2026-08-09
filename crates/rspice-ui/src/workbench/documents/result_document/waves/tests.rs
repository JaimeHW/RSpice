//! Tests for trace pairing, complex display, and cursor copy.
//!
//! Overlays must pair results by exact source instance rather than by name,
//! complex values must display under their declared component or phase
//! policy, and a copied cursor value must carry its explicit numeric policy.

use super::*;

#[test]
fn the_readout_strip_stands_down_until_a_cursor_is_placed() {
    let mut state = AppState::default();
    assert!(state.ui.results.cursor_tool.is_armed(), "armed by default");
    assert!(!state.ui.results.cursor_readout_active());
    assert_eq!(readout_strip_height(&state), 0.0);

    state.ui.results.cursors.place(1.0e-3);
    assert!(state.ui.results.cursor_readout_active());
    // No visible traces on an empty run: the header alone, never a band
    // of blank rows.
    assert_eq!(readout_strip_height(&state), READOUT_HEADER_H);
}

#[test]
fn disarming_the_cursor_tool_clears_the_pair_and_hides_the_strip() {
    let mut state = AppState::default();
    state.ui.results.cursors.place(1.0);
    state.ui.results.cursors.place(2.0);
    state.ui.results.cursor_strip = Some(0);

    state.ui.results.toggle_cursor_tool();

    assert!(!state.ui.results.cursor_tool.is_armed());
    assert!(!state.ui.results.cursors.any());
    assert_eq!(state.ui.results.cursor_strip, None);
    assert_eq!(readout_strip_height(&state), 0.0);

    state.ui.results.toggle_cursor_tool();
    assert!(state.ui.results.cursor_tool.is_armed());
    assert!(
        !state.ui.results.cursors.any(),
        "re-arming must not resurrect cleared cursors"
    );
}

#[test]
fn deep_readouts_scroll_without_exceeding_the_212_px_body_cap() {
    let state = deep_readout_fixture(16);

    assert_eq!(readout_trace_count(&state), 16);
    assert_eq!(READOUT_BODY_MAX_H, 212.0);
    assert_eq!(READOUT_MAX_H, READOUT_HEADER_H + READOUT_BODY_MAX_H);
    assert_eq!(readout_strip_height(&state), READOUT_MAX_H);
    assert!(readout_strip_height(&state) <= 238.0);
}
use crate::product::{AnalysisInstanceId, ContentDigest, DatasetId, ObjectRevision};
use crate::results::visualization_document::{
    FamilyAggregationMethod, FamilyAggregationPolicy, FamilyComparisonOperator, FamilyDimension,
    FamilyEncodingMap, FamilyFilterExpression, FamilyPredicate, FamilyPresentationPolicy,
    FamilyXDimension, FamilyXOrdering, MissingPointPolicy, TypedValue, ValueType,
};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultProvenance, SimulationRun,
    WaveformData,
};
use crate::workbench::ChoicePreference;
use crate::workbench::documents::visualization_family::FamilyManifest;

fn assert_editor_spans_disjoint(left: EditorSpan, right: EditorSpan) {
    assert!(
        left.end() <= right.start + f32::EPSILON,
        "editor spans overlap: {left:?} and {right:?}"
    );
}

#[test]
fn compact_expression_editor_reserves_add_and_stacks_error() {
    let layout = expr_editor_layout(350.0, 28.0, 42.0, Some(420.0));

    assert!(layout.stack_error);
    assert_eq!(layout.add.end(), 350.0);
    assert_eq!(layout.error.width, 0.0);
    assert!(layout.input.width > EXPR_EDITOR_MIN_INLINE_INPUT);
    assert_editor_spans_disjoint(layout.label, layout.input);
    assert_editor_spans_disjoint(layout.input, layout.add);
}

#[test]
fn wide_expression_editor_bounds_inline_error_without_starving_input() {
    let layout = expr_editor_layout(900.0, 28.0, 42.0, Some(640.0));

    assert!(!layout.stack_error);
    assert!(layout.error.width > 0.0);
    assert!(layout.error.width <= 900.0 * 0.28);
    assert!(layout.input.width >= EXPR_EDITOR_MIN_INLINE_INPUT);
    assert_editor_spans_disjoint(layout.label, layout.input);
    assert_editor_spans_disjoint(layout.input, layout.error);
    assert_editor_spans_disjoint(layout.error, layout.add);
    assert_eq!(layout.add.end(), 900.0);
}

#[test]
fn expression_editor_geometry_stays_inside_pathological_widths() {
    for width in [0.0, 20.0, 64.0, 180.0, 560.0] {
        let layout = expr_editor_layout(width, 28.0, 42.0, None);
        for span in [layout.label, layout.input, layout.error, layout.add] {
            assert!(span.start >= 0.0);
            assert!(span.end() <= width + f32::EPSILON);
        }
        assert_editor_spans_disjoint(layout.label, layout.input);
        assert_editor_spans_disjoint(layout.input, layout.add);
    }
}

fn family_policy() -> FamilyPresentationPolicy {
    let process = FamilyDimension::new("process", ValueType::Text).unwrap();
    FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: FamilyDimension::new("RGAIN", ValueType::Real).unwrap(),
            ordering: FamilyXOrdering::Source,
        },
        family_dimensions: vec![process.clone()],
        facet_layout: None,
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::None,
            over_dimensions: Vec::new(),
        },
        filter: None,
        missing_points: MissingPointPolicy::ExcludeWithOmissionRecord,
        encodings: vec![
            FamilyEncodingMap::Color {
                dimension: process.clone(),
                palette: AccessibleColorPalette::OkabeItoCategorical,
            },
            FamilyEncodingMap::Dash {
                dimension: process.clone(),
            },
            FamilyEncodingMap::Marker { dimension: process },
        ],
    }
}

fn family_analysis(values: Vec<f64>) -> AnalysisResult {
    AnalysisResult::new(41, AnalysisType::Corner, "PVT")
        .with_waveforms(vec![WaveformData::new(
            "V(out)",
            vec![101.0, 102.0, 103.0, 104.0, 105.0, 106.0],
            values,
            "#fff",
        )])
        .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
            x_values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            x_label: "RGAIN".to_owned(),
            x_unit: "kOhm".to_owned(),
            temperatures_c: vec![27.0; 6],
            corner_labels: vec![
                "SS".to_owned(),
                "SS".to_owned(),
                "TT".to_owned(),
                "TT".to_owned(),
                "FF".to_owned(),
                "FF".to_owned(),
            ],
            failed_corners: 0,
        })
}

/// A one-analysis transient run with a single ramp on `V(out)`.
fn marker_fixture() -> AppState {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 5.0], "#fff"),
        ]),
    );
    state
}

fn deep_readout_fixture(trace_count: usize) -> AppState {
    let waveforms = (0..trace_count)
        .map(|index| {
            WaveformData::new(
                format!("V(out{index})"),
                vec![0.0, 1.0],
                vec![index as f64, index as f64 + 1.0],
                "#fff",
            )
        })
        .collect();
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(waveforms),
    );
    state.ui.results.cursors.place(0.5);
    state.ui.results.cursor_strip = Some(0);
    state
}

#[test]
fn shared_x_navigator_preserves_view_width_and_clamps_to_the_full_domain() {
    let linear = panned_shared_x_view(XScale::Linear, (0.0, 10.0), (2.0, 6.0), 0.7)
        .expect("a zoomed linear view can pan");
    assert!((linear.0 - 6.0).abs() < 1.0e-9);
    assert!((linear.1 - 10.0).abs() < 1.0e-9);

    let logarithmic = panned_shared_x_view(XScale::Log10, (1.0, 1.0e6), (10.0, 1.0e3), 1.0 / 6.0)
        .expect("a zoomed logarithmic view can pan");
    assert!((logarithmic.0 / 100.0 - 1.0).abs() < 1.0e-9);
    assert!((logarithmic.1 / 1.0e4 - 1.0).abs() < 1.0e-9);

    assert!(panned_shared_x_view(XScale::Linear, (0.0, 10.0), (0.0, 10.0), 0.1).is_none());
}

#[test]
fn shared_x_zoom_stays_pointer_anchored_and_inside_the_retained_domain() {
    let zoomed = zoomed_shared_x_view(XScale::Linear, (0.0, 100.0), (20.0, 60.0), 0.4, 0.5)
        .expect("a finite zoomed view");
    assert!((zoomed.0 - 30.0).abs() < 1.0e-9);
    assert!((zoomed.1 - 50.0).abs() < 1.0e-9);

    let full = zoomed_shared_x_view(XScale::Log10, (1.0, 1.0e6), (10.0, 1.0e3), 1.0, 100.0)
        .expect("zoom-out clamps to the retained range");
    assert!((full.0 - 1.0).abs() < 1.0e-9);
    assert!((full.1 / 1.0e6 - 1.0).abs() < 1.0e-9);
}

#[test]
fn shared_x_click_recenters_without_changing_window_width() {
    let view = recentered_shared_x_view(XScale::Linear, (0.0, 10.0), (1.0, 5.0), 0.8)
        .expect("a zoomed view can recenter");
    assert!((view.0 - 6.0).abs() < 1.0e-9);
    assert!((view.1 - 10.0).abs() < 1.0e-9);
    assert!(recentered_shared_x_view(XScale::Linear, (0.0, 10.0), (0.0, 10.0), 0.5).is_none());
}

#[test]
fn multi_pane_geometry_never_grows_past_its_strip_budget() {
    for available in [0.0, 18.0, 60.0, 140.0, 320.0] {
        for pane_count in 1..=8 {
            let geometry = wave_stack_geometry(available, pane_count);
            let seams = geometry.seam_height * pane_count.saturating_sub(1) as f32;
            let used = geometry.pane_height * pane_count as f32 + geometry.shared_x_height + seams;
            assert!(geometry.pane_height >= 0.0);
            assert!((0.0..=WAVE_SHARED_X_HEIGHT).contains(&geometry.shared_x_height));
            assert!(used <= available.max(0.0) + f32::EPSILON * 16.0);
        }
    }
}

#[test]
fn hidden_unit_signals_keep_a_pane_available_for_reactivation() {
    let mut state = marker_fixture();
    state.simulation.runs[0].analyses[0].waveforms[0].visible = false;
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let panes = models[0].unit_panes();
    assert_eq!(panes.len(), 1);
    assert_eq!(panes[0].unit, "V");
    assert!(panes[0].traces.is_empty());
}

#[test]
fn shared_x_frame_exposes_current_and_full_ranges_to_accessibility() {
    let mut state = marker_fixture();
    state.ui.results.cursor_strip = Some(0);
    state.ui.results.cursors.a = Some(0.25);
    state.ui.results.cursors.b = Some(0.75);
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let model = &models[0];
    set_shared_x_view(
        &mut state.ui.results,
        model.analysis_key,
        1,
        Some((0.25, 0.75)),
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            show_shared_x_axis(ui, &mut state, model, (0.0, 1.0), 1, 50.0, None);
        });
    });
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::GraphicsDocument
            && node.label().is_some_and(|label| {
                label.contains("Shared ")
                    && label.contains("Current range")
                    && label.contains("Full retained range")
                    && label.contains("Cursor A")
                    && label.contains("cursor B")
            })
    }));
}

#[test]
fn each_cursor_trace_reports_its_own_linear_slope() {
    let mut state = marker_fixture();
    let presentation = state.ui.preferences.result_presentation_policy();
    let complex_display = presentation.complex_number_display();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        complex_display,
        &Tokens::default(),
    );

    let slopes = slope_values(&models[0], 0.0, 1.0, presentation);

    assert_eq!(slopes.len(), 1);
    assert!(
        slopes[0].contains("5") && slopes[0].contains("V/s"),
        "unexpected slope readout: {}",
        slopes[0]
    );
}

fn marker_identity(state: &AppState) -> (AnalysisPresentationKey, WaveformPresentationKey) {
    let run = state.simulation.active_run().expect("active marker run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let waveform = WaveformPresentationKey {
        analysis,
        trace: TracePresentationKey {
            source_name: "V(out)".to_owned(),
            kind: TraceKind::Value as u8,
            family_group: 0,
        },
    };
    (analysis, waveform)
}

#[test]
fn a_marker_never_migrates_to_a_different_dataset_after_a_re_run() {
    let waveforms = || {
        vec![WaveformData::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 5.0],
            "#fff",
        )]
    };
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(waveforms()),
    );
    let mut derived = DerivedSeries::default();
    let first = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );
    let anchor = anchor_key(&first[0], &first[0].traces[0]);

    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(waveforms()),
    );
    let mut derived = DerivedSeries::default();
    let second = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );
    let migrated = second[0]
        .traces
        .iter()
        .find(|trace| !trace.overlay && anchor_key(&second[0], trace) == anchor);
    assert!(
        migrated.is_none(),
        "a marker owned by the prior immutable dataset must not relabel itself as the new run"
    );
}

#[test]
fn markers_alone_keep_a_compact_readout_strip_on_screen() {
    let mut state = marker_fixture();
    let (analysis, waveform) = marker_identity(&state);
    assert_eq!(readout_strip_height(&state), 0.0);

    state
        .ui
        .results
        .add_marker(analysis, waveform, "V(out)".to_owned(), 0.5);
    assert_eq!(
        readout_strip_height(&state),
        READOUT_HEADER_H + MARKER_ROW_H,
        "markers-only dock: one shared header and its one row"
    );

    // A closed strip takes its markers off screen with it.
    state.ui.results.hidden_strips.insert(analysis);
    assert_eq!(readout_strip_height(&state), 0.0);
}

#[test]
fn the_strip_carries_cursors_and_markers_together() {
    let mut state = marker_fixture();
    let (analysis, waveform) = marker_identity(&state);
    state.ui.results.cursors.place(0.5);
    state.ui.results.cursor_strip = Some(0);
    let cursors_only = readout_strip_height(&state);
    assert!(cursors_only > 0.0);

    state
        .ui
        .results
        .add_marker(analysis, waveform, "V(out)".to_owned(), 0.5);
    assert_eq!(
        readout_strip_height(&state),
        cursors_only,
        "desktop columns share one body height instead of stacking sections"
    );
}

#[test]
fn collapse_keeps_one_header_and_no_content_still_removes_the_strip() {
    let mut state = marker_fixture();
    let (analysis, waveform) = marker_identity(&state);
    state
        .ui
        .results
        .add_marker(analysis, waveform, "V(out)".to_owned(), 0.5);
    state.ui.results.readout_collapsed = true;

    assert_eq!(readout_strip_height(&state), READOUT_HEADER_H);

    state.ui.results.hidden_strips.insert(analysis);
    assert_eq!(readout_strip_height(&state), 0.0);
}

#[test]
fn every_visible_marker_remains_in_the_scroll_owned_body() {
    let mut state = marker_fixture();
    let (analysis, waveform) = marker_identity(&state);
    for index in 0..12 {
        state.ui.results.add_marker(
            analysis,
            waveform.clone(),
            "V(out)".to_owned(),
            index as f64 / 12.0,
        );
    }

    assert_eq!(visible_markers(&state).len(), 12);
    assert_eq!(marker_body_height(&state), 12.0 * MARKER_ROW_H);
    assert_eq!(readout_strip_height(&state), READOUT_MAX_H);
}

#[test]
fn cursor_and_marker_columns_split_at_normal_desktop_widths() {
    assert!(!readout_columns_side_by_side(679.0, true, true));
    assert!(readout_columns_side_by_side(680.0, true, true));
    assert!(!readout_columns_side_by_side(900.0, true, false));
    assert!(!readout_columns_side_by_side(900.0, false, true));
}

#[test]
fn markers_outlive_the_tool_that_placed_them() {
    let mut state = marker_fixture();
    let (analysis, waveform) = marker_identity(&state);
    assert!(
        !state.ui.results.marker_tool.is_armed(),
        "annotating is deliberate — the tool is off until asked for"
    );
    state.ui.results.toggle_marker_tool();
    let id = state
        .ui
        .results
        .add_marker(analysis, waveform, "V(out)".to_owned(), 0.5);
    state.ui.results.toggle_marker_tool();

    assert!(!state.ui.results.marker_tool.is_armed());
    assert_eq!(state.ui.results.markers.len(), 1);

    // Cursors are a readout and clear; markers are content and do not.
    state.ui.results.clear_cursors();
    assert_eq!(state.ui.results.markers.len(), 1);
    assert_eq!(state.ui.results.markers[0].id, id);
}

#[test]
fn removing_a_marker_takes_its_open_note_editor_with_it() {
    let mut state = marker_fixture();
    let (analysis, waveform) = marker_identity(&state);
    let first = state
        .ui
        .results
        .add_marker(analysis, waveform.clone(), "V(out)".to_owned(), 0.5);
    state.ui.results.editing_marker = Some(first);

    state.ui.results.remove_marker(first);

    assert!(state.ui.results.markers.is_empty());
    assert_eq!(state.ui.results.editing_marker, None);

    // Ids are not recycled: M1 must not come back meaning something else.
    let second = state
        .ui
        .results
        .add_marker(analysis, waveform, "V(out)".to_owned(), 0.9);
    assert_ne!(first, second);
}

#[test]
fn only_a_spec_marker_declines_to_report_a_trace_value() {
    assert!(MarkerKind::Note.rides_a_trace());
    assert!(MarkerKind::Peak.rides_a_trace());
    assert!(
        !MarkerKind::Spec.rides_a_trace(),
        "a spec constrains the axis position, not one curve"
    );

    let mut kind = MarkerKind::Note;
    for _ in 0..MarkerKind::ALL.len() {
        kind = kind.next();
    }
    assert_eq!(kind, MarkerKind::Note, "the kind control cycles");
}

#[test]
fn a_marker_tag_names_the_note_only_when_there_is_one() {
    let analysis_result = AnalysisResult::new(1, AnalysisType::Transient, "marker analysis");
    let analysis = AnalysisPresentationKey::new(DatasetId::new(), &analysis_result);
    let mut marker = ResultMarker {
        id: 3,
        analysis,
        anchor: WaveformPresentationKey {
            analysis,
            trace: TracePresentationKey {
                source_name: "V(out)".to_owned(),
                kind: TraceKind::Value as u8,
                family_group: 0,
            },
        },
        trace_name: "V(out)".to_owned(),
        x: 0.0,
        kind: MarkerKind::Note,
        note: String::new(),
    };
    assert_eq!(marker_label(&marker), "M3");

    marker.note = "  settling  ".to_owned();
    assert_eq!(marker_label(&marker), "M3 · settling");
}

#[test]
fn a_signal_owns_its_unit_rather_than_inheriting_the_analysis_default() {
    // The accessor in the name is authoritative where there is one.
    assert_eq!(signal_unit("V(out)", TraceKind::Value, "V"), "V");
    assert_eq!(signal_unit("I(R1)", TraceKind::Value, "V"), "A");
    assert_eq!(signal_unit("i(vsense)", TraceKind::Value, "V"), "A");
    assert_eq!(signal_unit("P(M1)", TraceKind::Value, "V"), "W");

    // Derived projections keep the underlying signal's unit.
    assert_eq!(signal_unit("re(V(out))", TraceKind::Real, ""), "V");
    assert_eq!(signal_unit("im(I(R1))", TraceKind::Imaginary, ""), "A");

    // The analysis default applies only where the name carries nothing
    // to read a unit from.
    assert_eq!(signal_unit("onoise", TraceKind::Value, "V^2/Hz"), "V^2/Hz");

    // Derived kinds have their own units regardless of the source.
    assert_eq!(signal_unit("V(out)", TraceKind::MagnitudeDb, "V"), "dB");
    assert_eq!(signal_unit("V(out)", TraceKind::PhaseDeg, "V"), "°");
}

#[test]
fn mixed_units_on_one_analysis_become_separate_panes() {
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 5.0], "#fff"),
            WaveformData::new("I(R1)", vec![0.0, 1.0], vec![0.0, 1.0e-3], "#fff"),
            WaveformData::new("V(in)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
        ]),
    );
    let mut derived = DerivedSeries::default();
    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );

    let panes = models[0].unit_panes();
    assert_eq!(panes.len(), 2, "volts and amps cannot share an axis");
    assert_eq!(panes[0].unit, "V");
    assert_eq!(
        panes[0].traces.len(),
        2,
        "both voltages belong to the volt pane"
    );
    assert_eq!(panes[1].unit, "A");
    assert_eq!(panes[1].traces.len(), 1);
}

#[test]
fn one_unit_stays_one_pane() {
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 5.0], "#fff"),
            WaveformData::new("V(in)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
        ]),
    );
    let mut derived = DerivedSeries::default();
    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );

    let panes = models[0].unit_panes();
    assert_eq!(panes.len(), 1, "a strip does not split without a reason to");
    assert_eq!(panes[0].unit, "V");
}

#[test]
fn a_hidden_trace_keeps_its_unit_pane_available_for_reactivation() {
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 5.0], "#fff"),
            WaveformData::new("I(R1)", vec![0.0, 1.0], vec![0.0, 1.0e-3], "#fff"),
        ]),
    );
    if let Some(run) = simulation.active_run_mut()
        && let Some(analysis) = run.analyses.get_mut(0)
        && let Some(current) = analysis.waveforms.get_mut(1)
    {
        current.visible = false;
    }
    let mut derived = DerivedSeries::default();
    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );

    let panes = models[0].unit_panes();
    assert_eq!(panes.len(), 2);
    assert_eq!(panes[0].unit, "V");
    assert_eq!(panes[1].unit, "A");
    assert!(
        panes[1].traces.is_empty(),
        "the amp pane remains as the owner of its hidden signal"
    );
}

#[test]
fn a_bode_pair_stacks_magnitude_over_phase() {
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new("|V(out)|", vec![1.0, 10.0], vec![1.0, 0.5], "#fff"),
            WaveformData::new("phase(V(out))", vec![1.0, 10.0], vec![0.0, -45.0], "#fff"),
        ]),
    );
    let mut derived = DerivedSeries::default();
    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );

    let panes = models[0].unit_panes();
    assert_eq!(
        panes.len(),
        2,
        "the mockup Bode instrument is two stacked panes over one X domain"
    );
    assert_eq!(panes[0].unit, "dB", "magnitude keeps the primary slot");
    assert_eq!(panes[1].unit, "°");
    assert_eq!(panes[1].traces.len(), 1);
}

#[test]
fn noise_analyses_project_psd_into_an_nv_sqrt_hz_pane() {
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Noise, "NOISE").with_waveforms(vec![
            WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 4.0e-18], "#fff"),
        ]),
    );
    let mut derived = DerivedSeries::default();
    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );

    let panes = models[0].unit_panes();
    assert_eq!(panes.len(), 1);
    assert_eq!(panes[0].unit, "nV/√Hz");
    // 1e-18 V²/Hz → 1 nV/√Hz; 4e-18 → 2 nV/√Hz. The projection is the
    // exact amplitude-density conversion, not a relabeled PSD.
    let trace = &models[0].traces[0];
    assert!((trace.y[0] - 1.0).abs() < 1e-9);
    assert!((trace.y[1] - 2.0).abs() < 1e-9);
}

#[test]
fn phase_panes_order_after_quantity_panes_regardless_of_waveform_order() {
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new("phase(V(out))", vec![1.0, 10.0], vec![0.0, -45.0], "#fff"),
            WaveformData::new("|V(out)|", vec![1.0, 10.0], vec![1.0, 0.5], "#fff"),
        ]),
    );
    let mut derived = DerivedSeries::default();
    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );

    let panes = models[0].unit_panes();
    assert_eq!(panes.len(), 2);
    assert_eq!(
        panes[0].unit, "dB",
        "magnitude is the primary pane even when the dataset lists phase first"
    );
    assert_eq!(panes[1].unit, "°");
}

#[test]
fn fitting_a_strip_fits_every_pane_of_it() {
    let mut state = AppState::default();
    let viewer = super::super::ResultViewer::Waves;
    state.ui.results.plot_view_pane_mut(viewer, 0, 0).y = Some((0.0, 1.0));
    state.ui.results.plot_view_pane_mut(viewer, 0, 1).y = Some((0.0, 2.0));
    state.ui.results.plot_view_pane_mut(viewer, 1, 0).y = Some((0.0, 3.0));
    assert!(state.ui.results.strip_is_zoomed(viewer, 0));

    state.ui.results.reset_plot_view(viewer, 0);

    assert!(
        !state.ui.results.strip_is_zoomed(viewer, 0),
        "leaving one pane zoomed would make the strip's panes disagree"
    );
    assert!(
        state.ui.results.strip_is_zoomed(viewer, 1),
        "fitting one strip does not reach into another"
    );
}

#[test]
fn each_pane_keeps_its_own_y_viewport() {
    let mut state = AppState::default();
    let viewer = super::super::ResultViewer::Waves;
    state.ui.results.plot_view_pane_mut(viewer, 0, 0).y = Some((-5.0, 5.0));
    state.ui.results.plot_view_pane_mut(viewer, 0, 1).y = Some((0.0, 1.0e-3));

    // One zoom factor across volts and amps would mean nothing, so the
    // panes never share a Y override.
    assert_eq!(
        state.ui.results.plot_view_pane(viewer, 0, 0).y,
        Some((-5.0, 5.0))
    );
    assert_eq!(
        state.ui.results.plot_view_pane(viewer, 0, 1).y,
        Some((0.0, 1.0e-3))
    );
}

#[test]
fn noise_strip_uses_spectral_density_unit_without_db_conversion() {
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Noise, "Noise").with_waveforms(vec![
            WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 4.0e-18], "#fff"),
        ]),
    );
    let mut derived = DerivedSeries::default();

    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );

    // Retained PSDs project to amplitude density (nV/√Hz), never a dB
    // relabeling of V²/Hz.
    assert_eq!(models.len(), 1);
    assert!(matches!(models[0].traces[0].kind, TraceKind::NoiseDensity));
    assert_eq!(models[0].traces[0].y.as_slice(), &[1.0, 2.0]);
}

#[test]
fn family_selection_projects_exact_source_rows_without_mutating_the_run() {
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(41, AnalysisType::Corner, "PVT").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![1.0, 2.0, 3.0, 4.0],
                vec![10.0, 20.0, 30.0, 40.0],
                "#fff",
            ),
        ]),
    );
    let run = simulation.active_run().expect("active run");
    let selection = SourceSampleSelection::new(run.dataset_id, 41, vec![1, 3])
        .expect("ordered exact selection");
    let mut derived = DerivedSeries::default();

    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        Some(&selection),
        &HashSet::new(),
    );

    assert_eq!(models[0].traces[0].x.as_slice(), &[2.0, 4.0]);
    assert_eq!(models[0].traces[0].y.as_slice(), &[20.0, 40.0]);
    let original = &simulation.active_run().unwrap().analyses[0].waveforms[0];
    assert_eq!(original.x.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(original.y.as_slice(), &[10.0, 20.0, 30.0, 40.0]);
}

#[test]
fn family_policy_expands_stable_styles_and_preserves_overlay_sources() {
    let mut active = SimulationRun::new(2);
    active.add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
    let active_dataset = active.dataset_id;
    let manifest = FamilyManifest::from_analysis(&active.analyses[0])
        .unwrap()
        .unwrap();
    let selection = SourceSampleSelection::new(active_dataset, 41, vec![0, 1, 2, 3, 4, 5])
        .unwrap()
        .with_family_presentation(&manifest, &family_policy())
        .unwrap();

    let mut overlay = SimulationRun::new(1);
    overlay.add_analysis(family_analysis(vec![11.0, 21.0, 31.0, 41.0, 51.0, 61.0]));
    let overlay_dataset = overlay.dataset_id;
    let simulation = SimulationState {
        runs: vec![active, overlay],
        active_run_idx: Some(0),
        active_analysis_idx: Some(0),
        overlay_dataset_ids: vec![overlay_dataset],
        ..SimulationState::default()
    };
    let mut derived = DerivedSeries::default();

    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        Some(&selection),
        &HashSet::new(),
    );

    let model = &models[0];
    assert_eq!(model.signal_trace_count, 3);
    assert_eq!(
        model.traces.len(),
        6,
        "overlay signal must project through every exact family group"
    );
    assert_eq!(model.x_label, "RGAIN");
    assert_eq!(model.x_unit, "kOhm");
    assert_eq!(model.x_scale, XScale::Linear);
    let ss = model
        .traces
        .iter()
        .find(|trace| !trace.overlay && trace.name.contains("SS"))
        .unwrap();
    let tt = model
        .traces
        .iter()
        .find(|trace| !trace.overlay && trace.name.contains("TT"))
        .unwrap();
    let tt_visibility_key = tt.family_visibility_key.unwrap();
    assert_eq!(ss.x.as_slice(), &[1.0, 2.0]);
    assert_eq!(ss.y.as_slice(), &[10.0, 20.0]);
    assert_eq!(tt.x.as_slice(), &[3.0, 4.0]);
    assert_eq!(tt.y.as_slice(), &[30.0, 40.0]);
    assert_ne!(ss.color, tt.color);
    assert_ne!(
        ss.family_style.unwrap().marker_ordinal,
        tt.family_style.unwrap().marker_ordinal
    );
    let styled = apply_family_trace_style(Trace::new(&ss.x, &ss.y, ss.color), ss.family_style);
    assert_eq!(styled.dash_style, ss.family_style.unwrap().dash_ordinal);
    assert_eq!(styled.marker_style, ss.family_style.unwrap().marker_ordinal);
    assert!(styled.show_single_point);
    assert!(model.traces.last().unwrap().overlay);
    assert_eq!(model.traces.last().unwrap().x.as_slice(), &[3.0, 4.0]);

    let source = &simulation.runs[0].analyses[0].waveforms[0];
    assert_eq!(
        source.x.as_slice(),
        &[101.0, 102.0, 103.0, 104.0, 105.0, 106.0]
    );
    assert_eq!(source.y.as_slice(), &[10.0, 20.0, 30.0, 40.0, 50.0, 60.0]);

    let mut copied = String::new();
    append_copied_cursor(
        &mut copied,
        "A",
        3.0,
        model,
        SampleInterpolation::Linear,
        crate::quantity::QuantityPresentationPolicy::default(),
    );
    assert!(copied.contains("A RGAIN ="));
    assert!(copied.contains("kOhm"));
    let domain = model.cursor_domain();
    let mut incompatible_unit = domain.clone();
    incompatible_unit.x_unit = "Ohm".to_owned();
    assert_ne!(domain, incompatible_unit);
    let mut incompatible_label = domain.clone();
    incompatible_label.x_label = "Resistance".to_owned();
    assert_ne!(domain, incompatible_label);

    let mut results = ResultsState::default();
    results.set_sample_selection(Some(selection.clone()));
    results.toggle_family_trace_visibility(tt_visibility_key);
    let toggled = build_models(
        &simulation,
        &mut results.derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        Some(&selection),
        &results.hidden_family_traces,
    );
    let toggled_model = &toggled[0];
    assert!(
        toggled_model
            .traces
            .iter()
            .find(|trace| !trace.overlay && trace.name.contains("SS"))
            .unwrap()
            .visible
    );
    assert!(
        !toggled_model
            .traces
            .iter()
            .find(|trace| !trace.overlay && trace.name.contains("TT"))
            .unwrap()
            .visible
    );
    for overlay in toggled_model.traces.iter().filter(|trace| trace.overlay) {
        let active = toggled_model
            .traces
            .iter()
            .find(|trace| {
                !trace.overlay
                    && trace.presentation_key == overlay.presentation_key
                    && trace.kind == overlay.kind
            })
            .unwrap();
        assert_eq!(overlay.visible, active.visible);
    }
    assert!(
        toggled_model
            .traces
            .iter()
            .any(|trace| trace.overlay && trace.visible)
    );
    assert!(simulation.runs[0].analyses[0].waveforms[0].visible);
    results.set_sample_selection(None);
    assert!(results.hidden_family_traces.is_empty());
}

#[test]
fn incompatible_family_overlay_is_visibly_rejected_without_drawing_native_x() {
    let mut active = SimulationRun::new(2);
    active.add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
    let manifest = FamilyManifest::from_analysis(&active.analyses[0])
        .unwrap()
        .unwrap();
    let selection = SourceSampleSelection::new(active.dataset_id, 41, vec![0, 1, 2, 3, 4, 5])
        .unwrap()
        .with_family_presentation(&manifest, &family_policy())
        .unwrap();

    let mut incompatible = family_analysis(vec![11.0, 21.0, 31.0, 41.0, 51.0, 61.0]);
    let Some(AnalysisResultFamilyMetadata::Corner { x_unit, .. }) =
        incompatible.family_metadata.as_mut()
    else {
        panic!("corner metadata");
    };
    *x_unit = "Ohm".to_owned();
    let mut overlay = SimulationRun::new(1);
    overlay.add_analysis(incompatible);
    let overlay_dataset = overlay.dataset_id;
    let simulation = SimulationState {
        runs: vec![active, overlay],
        active_run_idx: Some(0),
        overlay_dataset_ids: vec![overlay_dataset],
        ..SimulationState::default()
    };

    let models = build_models(
        &simulation,
        &mut DerivedSeries::default(),
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        Some(&selection),
        &HashSet::new(),
    );
    assert_eq!(models[0].traces.len(), models[0].signal_trace_count);
    assert!(
        models[0]
            .subtitle
            .contains("incompatible family overlay hidden")
    );
    assert!(models[0].traces.iter().all(|trace| !trace.overlay));
}

#[test]
fn filtered_overlay_uses_typed_ast_and_ignores_excluded_duplicate_x_rows() {
    let mut active = SimulationRun::new(2);
    active.add_analysis(family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]));
    let manifest = FamilyManifest::from_analysis(&active.analyses[0])
        .unwrap()
        .unwrap();
    let process = FamilyDimension::new("process", ValueType::Text).unwrap();
    let mut policy = family_policy();
    policy.filter = Some(FamilyFilterExpression {
        // Deliberately contradictory UI source: the typed AST is the
        // persisted execution contract.
        source: "process = SS".to_owned(),
        predicate: FamilyPredicate::Compare {
            dimension: process,
            operator: FamilyComparisonOperator::Equal,
            value: TypedValue::Text("TT".to_owned()),
        },
    });
    let indices = manifest
        .matching_source_indices_for_filter(policy.filter.as_ref())
        .unwrap();
    assert_eq!(indices, [2, 3]);
    let selection = SourceSampleSelection::new(active.dataset_id, 41, indices)
        .unwrap()
        .with_family_presentation(&manifest, &policy)
        .unwrap();

    let mut overlay_analysis = family_analysis(vec![11.0, 21.0, 31.0, 41.0, 51.0, 61.0]);
    let Some(AnalysisResultFamilyMetadata::Corner { x_values, .. }) =
        overlay_analysis.family_metadata.as_mut()
    else {
        panic!("corner metadata");
    };
    // Excluded SS rows are non-monotonic. Re-evaluating all overlay rows
    // would reject this otherwise compatible filtered TT projection.
    x_values[0] = 1.0;
    x_values[1] = 1.0;
    let mut overlay = SimulationRun::new(1);
    overlay.add_analysis(overlay_analysis);
    let overlay_dataset = overlay.dataset_id;
    let simulation = SimulationState {
        runs: vec![active, overlay],
        active_run_idx: Some(0),
        overlay_dataset_ids: vec![overlay_dataset],
        ..SimulationState::default()
    };

    let models = build_models(
        &simulation,
        &mut DerivedSeries::default(),
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        Some(&selection),
        &HashSet::new(),
    );
    assert_eq!(models[0].signal_trace_count, 1);
    assert_eq!(models[0].traces.len(), 2);
    let overlay = models[0].traces.iter().find(|trace| trace.overlay).unwrap();
    assert_eq!(overlay.x.as_slice(), &[3.0, 4.0]);
    assert_eq!(overlay.y.as_slice(), &[31.0, 41.0]);
}

#[test]
fn derived_expression_rows_are_split_by_the_exact_family_plan() {
    let analysis = family_analysis(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]);
    let manifest = FamilyManifest::from_analysis(&analysis).unwrap().unwrap();
    let selection = SourceSampleSelection::new(DatasetId::new(), 41, vec![0, 2, 4])
        .unwrap()
        .with_family_presentation(&manifest, &family_policy())
        .unwrap();
    // Expression evaluation has already selected exact rows 0, 2, 4.
    let x = Arc::new(vec![101.0, 103.0, 105.0]);
    let y = Arc::new(vec![100.0, 300.0, 500.0]);

    let projections = projected_selected_family_series(&x, &y, Some(&selection)).unwrap();

    assert_eq!(projections.len(), 3);
    assert!(projections.iter().all(|projection| projection.x.len() == 1));
    let tt = projections
        .iter()
        .find(|projection| projection.group.unwrap().label.contains("TT"))
        .unwrap();
    assert_eq!(tt.x.as_slice(), &[3.0]);
    assert_eq!(tt.y.as_slice(), &[300.0]);
    let styled = apply_family_trace_style(
        Trace::new(&tt.x, &tt.y, egui::Color32::WHITE),
        Some(tt.group.unwrap().style),
    );
    assert!(styled.show_single_point);
}

fn ac_result(source_id: AnalysisInstanceId, values: [f64; 2], snapshot_byte: u8) -> AnalysisResult {
    AnalysisResult::new(1, AnalysisType::Ac, "AC")
        .with_waveforms(vec![WaveformData::new(
            "V(out)",
            vec![1.0, 10.0],
            values.to_vec(),
            "#fff",
        )])
        .with_provenance(
            AnalysisResultProvenance::new(
                source_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([snapshot_byte; 32]),
                Vec::new(),
            )
            .expect("valid AC provenance"),
        )
}

#[test]
fn overlays_pair_two_same_kind_results_by_exact_source_instance() {
    let first_id = AnalysisInstanceId::new();
    let second_id = AnalysisInstanceId::new();

    let mut active = SimulationRun::new(2);
    active.add_analysis(ac_result(first_id, [1.0, 2.0], 0x11));
    active.add_analysis(ac_result(second_id, [3.0, 4.0], 0x11));

    let mut overlay = SimulationRun::new(1);
    // Reverse the same-kind result order: kind/label matching would alias
    // the first overlay result onto both active strips.
    overlay.add_analysis(ac_result(second_id, [201.0, 202.0], 0x22));
    overlay.add_analysis(ac_result(first_id, [101.0, 102.0], 0x22));
    let overlay_dataset_id = overlay.dataset_id;

    let mut simulation = SimulationState {
        runs: vec![active, overlay],
        active_run_idx: Some(0),
        overlay_dataset_ids: vec![overlay_dataset_id],
        ..SimulationState::default()
    };
    assert!(simulation.select_analysis(0));
    let mut derived = DerivedSeries::default();

    let models = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        None,
        &HashSet::new(),
    );

    assert_eq!(models.len(), 2);
    assert_eq!(models[0].signal_trace_count, 1);
    assert_eq!(models[0].traces.len(), 2);
    assert!(models[0].traces[1].overlay);
    assert_eq!(models[0].traces[1].y.as_slice(), &[101.0, 102.0]);
    assert_eq!(models[1].signal_trace_count, 1);
    assert_eq!(models[1].traces.len(), 2);
    assert!(models[1].traces[1].overlay);
    assert_eq!(models[1].traces[1].y.as_slice(), &[201.0, 202.0]);
}

#[test]
fn complex_display_policy_uses_original_components_or_radian_phase() {
    let magnitude = WaveformData::new("|V(out)|", vec![1.0, 10.0], vec![1.0, 10.0], "#fff")
        .with_complex_components("V(out)", vec![0.8, 6.0], vec![0.6, 8.0]);
    let phase = WaveformData::new("phase(V(out))", vec![1.0, 10.0], vec![180.0, 90.0], "#aaa");
    let mut simulation = SimulationState::default();
    simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![magnitude, phase]),
    );

    let mut derived = DerivedSeries::default();
    let cartesian = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::RealImaginary,
        None,
        &HashSet::new(),
    );
    assert_eq!(cartesian[0].signal_trace_count, 2);
    assert_eq!(cartesian[0].y_unit, "");
    assert_eq!(cartesian[0].traces[0].name, "re(V(out))");
    assert_eq!(cartesian[0].traces[0].y.as_slice(), &[0.8, 6.0]);
    assert_eq!(cartesian[0].traces[1].name, "im(V(out))");
    assert_eq!(cartesian[0].traces[1].y.as_slice(), &[0.6, 8.0]);

    let radians = build_models(
        &simulation,
        &mut derived,
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseRadians,
        None,
        &HashSet::new(),
    );
    assert!(matches!(radians[0].traces[0].kind, TraceKind::MagnitudeDb));
    assert_eq!(radians[0].traces[0].y.as_slice(), &[0.0, 20.0]);
    assert!(matches!(radians[0].traces[1].kind, TraceKind::PhaseRad));
    assert!((radians[0].traces[1].y[0] - std::f64::consts::PI).abs() < 1e-12);
    assert!((radians[0].traces[1].y[1] - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
}

#[test]
fn cursor_copy_uses_explicit_scientific_si_policy() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Noise, "Noise").with_waveforms(vec![
            WaveformData::new("onoise", vec![1.0, 10.0], vec![2.0e-18, 4.0e-18], "#fff"),
        ]),
    );
    state.ui.results.cursor_strip = Some(0);
    state.ui.results.cursors.a = Some(10.0);
    state
        .ui
        .preferences
        .set_choice(ChoicePreference::CopiedValueFormat, 1)
        .unwrap();

    let copied = copy_cursor_text(&mut state).expect("active cursor has copy data");

    assert!(copied.contains("A f = 1.00000000000000000e1 Hz"));
    // 4e-18 V²/Hz at the cursor → 2 nV/√Hz through the amplitude-density
    // projection the pane displays.
    assert!(copied.contains("onoise = 2."));
    assert!(copied.contains("e0 nV/√Hz"));
}

#[test]
fn browser_classification_reads_the_accessor_through_derived_wrappers() {
    for (name, unit, current) in [
        ("V(out)", "V", false),
        ("I(C1)", "A", true),
        ("|V(OUT)|", "V", false),
        ("|I(VIN)|", "A", true),
        ("phase(V(OUT))", "°", false),
        ("phase(I(VIN))", "°", true),
        ("re(I(R1))", "A", true),
        ("onoise", "V^2/Hz", false),
    ] {
        assert_eq!(browser_signal_unit(name, "V^2/Hz"), unit, "{name}");
        assert_eq!(browser_signal_is_current(name), current, "{name}");
    }
}

#[test]
fn favorite_toggle_is_a_strict_membership_flip() {
    let mut state = AppState::default();
    let results = &mut state.ui.results;
    assert!(!results.is_favorite_signal("V(out)"));
    results.toggle_favorite_signal("V(out)");
    assert!(results.is_favorite_signal("V(out)"));
    results.toggle_favorite_signal("V(out)");
    assert!(!results.is_favorite_signal("V(out)"));
}

#[test]
fn recent_signals_front_insert_deduplicate_and_age_out() {
    let mut state = AppState::default();
    let results = &mut state.ui.results;
    for index in 0..30 {
        results.note_recent_signal(&format!("V(n{index})"));
    }
    // Re-noting an older name moves it to the front without duplicating it.
    results.note_recent_signal("V(n20)");
    assert_eq!(results.recent_signal_rank("V(n20)"), Some(0));
    assert_eq!(results.recent_signal_rank("V(n29)"), Some(1));
    // The shortlist is bounded: the oldest names have aged out entirely.
    assert!(results.recent_signal_rank("V(n6)").is_some());
    assert_eq!(results.recent_signal_rank("V(n5)"), None);
    assert_eq!(results.recent_signal_rank("V(n0)"), None);
}
