//! Local viewport and browser-selection regressions.
use super::*;
use rspice_results::{
    analysis_result::AnalysisResult, analysis_type::AnalysisType, waveform::RetainedWaveform,
};

#[test]
fn fitting_a_strip_fits_every_pane_of_it() {
    let mut results = ResultViewerState::default();
    let viewer = ResultViewer::Waves;
    results.plot_view_pane_mut(viewer, 0, 0).y = Some((0.0, 1.0));
    results.plot_view_pane_mut(viewer, 0, 1).y = Some((0.0, 2.0));
    results.plot_view_pane_mut(viewer, 1, 0).y = Some((0.0, 3.0));
    assert!(results.strip_is_zoomed(viewer, 0));

    results.reset_plot_view(viewer, 0);

    assert!(
        !results.strip_is_zoomed(viewer, 0),
        "leaving one pane zoomed would make the strip's panes disagree"
    );
    assert!(
        results.strip_is_zoomed(viewer, 1),
        "fitting one strip does not reach into another"
    );
}

#[test]
fn each_pane_keeps_its_own_y_viewport() {
    let mut results = ResultViewerState::default();
    let viewer = ResultViewer::Waves;
    results.plot_view_pane_mut(viewer, 0, 0).y = Some((-5.0, 5.0));
    results.plot_view_pane_mut(viewer, 0, 1).y = Some((0.0, 1.0e-3));

    // One zoom factor across volts and amps would mean nothing, so the
    // panes never share a Y override.
    assert_eq!(results.plot_view_pane(viewer, 0, 0).y, Some((-5.0, 5.0)));
    assert_eq!(results.plot_view_pane(viewer, 0, 1).y, Some((0.0, 1.0e-3)));
}

#[test]
fn recent_signals_front_insert_deduplicate_and_age_out() {
    let mut results = ResultViewerState::default();
    let analysis = AnalysisResult::<RetainedWaveform>::new(1, AnalysisType::Transient, "TRAN", 0.0);
    let analysis_key =
        AnalysisPresentationKey::new(rspice_vocabulary::product::DatasetId::new(), &analysis);
    let key = |index| SourceWaveformPresentationKey::new(analysis_key, format!("V(n{index})"));

    for index in 0..30 {
        results.note_recent_signal(key(index));
    }
    // Re-noting an older name moves it to the front without duplicating it.
    results.note_recent_signal(key(20));
    assert_eq!(results.recent_signal_rank(&key(20)), Some(0));
    assert_eq!(results.recent_signal_rank(&key(29)), Some(1));
    // The shortlist is bounded: the oldest names have aged out entirely.
    assert!(results.recent_signal_rank(&key(6)).is_some());
    assert_eq!(results.recent_signal_rank(&key(5)), None);
    assert_eq!(results.recent_signal_rank(&key(0)), None);
}

#[test]
fn browser_signal_marks_do_not_alias_equal_names_from_different_datasets() {
    let analysis = AnalysisResult::<RetainedWaveform>::new(1, AnalysisType::Transient, "TRAN", 0.0);
    let first = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(rspice_vocabulary::product::DatasetId::new(), &analysis),
        "V(out)",
    );
    let second = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(rspice_vocabulary::product::DatasetId::new(), &analysis),
        "V(out)",
    );
    let mut results = ResultViewerState::default();

    results.note_recent_signal(first.clone());

    assert_eq!(results.recent_signal_rank(&first), Some(0));
    assert_eq!(results.recent_signal_rank(&second), None);
}

/// A signal plotted takes the lowest colour place free and holds it until it
/// comes off, so plotting another never recolours one on the plot; a signal
/// drawn without a place (a restored session) adopts the one it is drawn
/// at, and one plotted alone starts again at the first.
#[test]
fn a_plotted_signal_holds_its_colour_place_until_it_comes_off() {
    let analysis = AnalysisResult::<RetainedWaveform>::new(1, AnalysisType::Transient, "TRAN", 0.0);
    let analysis_key =
        AnalysisPresentationKey::new(rspice_vocabulary::product::DatasetId::new(), &analysis);
    let key = |name: &str| SourceWaveformPresentationKey::new(analysis_key, name);
    let mut ticks = VisibilityOverrides::default();

    ticks.set(key("V(b)"), Some(true));
    ticks.set(key("V(a)"), Some(true));
    assert_eq!(ticks.plot_place(&key("V(b)")), Some(0));
    assert_eq!(ticks.plot_place(&key("V(a)")), Some(1), "not by column");

    ticks.set(key("V(b)"), Some(false));
    assert_eq!(
        ticks.plot_place(&key("V(b)")),
        Some(0),
        "hidden, still held"
    );
    ticks.set(key("V(b)"), None);
    ticks.set(key("V(c)"), Some(true));
    assert_eq!(ticks.plot_place(&key("V(c)")), Some(0), "the place freed");

    ticks.project(&key("V(d)"), Some(true));
    assert_eq!(ticks.plot_place(&key("V(d)")), None);
    ticks.adopt_plot_place(&key("V(d)"), 3);
    ticks.adopt_plot_place(&key("V(a)"), 0);
    assert_eq!(ticks.plot_place(&key("V(d)")), Some(3));
    assert_eq!(
        ticks.plot_place(&key("V(a)")),
        Some(1),
        "a held place stays"
    );
    ticks.set(key("V(e)"), Some(true));
    assert_eq!(
        ticks.plot_place(&key("V(e)")),
        Some(2),
        "below the adopted one"
    );

    for name in ["V(c)", "V(d)", "V(e)"] {
        ticks.set(key(name), None);
    }
    ticks.replot_first(&key("V(a)"));
    assert_eq!(ticks.plot_place(&key("V(a)")), Some(0));
}

/// Clicking H a second time puts the horizontal-cursor tool down, clears
/// every pane's horizontal cursors, and gives the plot back its ordinary
/// tool, as disarming A│B clears the pair.
#[test]
fn putting_the_horizontal_cursor_tool_down_clears_its_cursors() {
    let mut results = ResultViewerState::default();
    let analysis = AnalysisResult::<RetainedWaveform>::new(1, AnalysisType::Transient, "TRAN", 0.0);
    let pane = WavePanePresentationKey {
        analysis: AnalysisPresentationKey::new(
            rspice_vocabulary::product::DatasetId::new(),
            &analysis,
        ),
        unit: "V".to_owned(),
    };

    results.toggle_horizontal_cursor_tool();
    assert!(results.horizontal_cursor_placement_enabled());
    results.set_horizontal_cursor(pane.clone(), 0, 11.2);
    results.set_horizontal_cursor(pane.clone(), 1, 1.12);

    results.toggle_horizontal_cursor_tool();
    assert_eq!(results.plot_tool, ResultPlotTool::Cursor);
    assert_eq!(results.pane_horizontal_cursors(&pane), [None, None]);
}

/// "Remove cursors" on the plot's menu, and Escape, take A, B and every
/// horizontal cursor off the plot, and leave the tools as they were.
#[test]
fn removing_cursors_takes_the_pair_and_the_horizontal_ones() {
    let mut results = ResultViewerState::default();
    let analysis = AnalysisResult::<RetainedWaveform>::new(1, AnalysisType::Transient, "TRAN", 0.0);
    let pane = WavePanePresentationKey {
        analysis: AnalysisPresentationKey::new(
            rspice_vocabulary::product::DatasetId::new(),
            &analysis,
        ),
        unit: "V".to_owned(),
    };
    assert!(!results.any_cursor_placed());
    results.set_horizontal_cursor(pane.clone(), 0, 1.0);
    assert!(
        results.any_cursor_placed(),
        "a horizontal cursor alone counts"
    );
    results.cursors.place(2.0e-3);

    results.remove_all_cursors();

    assert!(!results.any_cursor_placed());
    assert_eq!(results.pane_horizontal_cursors(&pane), [None, None]);
}
