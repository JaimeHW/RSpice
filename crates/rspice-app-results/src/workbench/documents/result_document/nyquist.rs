//! The Nyquist sheet: the loop gain of the stability analysis shown, read off
//! the run's retained traces whenever the sheet is shown.
//!
//! The locus is not held anywhere but in the run. Any retained stability run
//! draws, in memory or stored, straight after it finished or after the project
//! was reopened, and each run in the history draws its own.

#[cfg(test)]
mod agreement;

use std::sync::Arc;

use crate::product::DatasetId;
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use egui::Ui;
use rspice_results::nyquist::{LOOP_GAIN_LOCUS_LABEL, loop_gain_traces};
use rspice_results::waveform::Fetched;
use rspice_results_ui::nyquist::NyquistData;
use rspice_results_ui::nyquist::view::{self, NyquistFrame, NyquistPlot, NyquistStability};
use rspice_results_ui::presentation::{PlotView, well_hint};

/// Where the shown run keeps the loop gain the sheet draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LoopGainSource {
    dataset: DatasetId,
    analysis: usize,
    real: usize,
    imaginary: usize,
}

/// The stability analysis the sheet draws: the selected analysis when it
/// measured a loop gain, else the shown run's first that did. A solve that
/// failed retained whatever the engine emitted before it gave up, which is no
/// loop gain to read a criterion off.
pub(crate) fn loop_gain_source(state: ResultsReads<'_>) -> Option<LoopGainSource> {
    let simulation = &state.model.simulation;
    let run = simulation.active_run()?;
    let source = |index: usize| {
        let analysis = run.analyses.get(index)?;
        if !analysis.success {
            return None;
        }
        let (real, imaginary) = loop_gain_traces(analysis.analysis_type, &analysis.waveforms)?;
        Some(LoopGainSource {
            dataset: run.dataset_id,
            analysis: index,
            real,
            imaginary,
        })
    };
    simulation
        .view
        .active_analysis_idx
        .and_then(source)
        .or_else(|| (0..run.analyses.len()).find_map(source))
}

/// One stability analysis's locus and the numbers read off it, kept until
/// the run history's samples change.
#[derive(Debug, Clone)]
pub struct NyquistDerived {
    version: u64,
    locus: Arc<NyquistData>,
    /// The locus as drawn, on the interpolant the margins are read from; the
    /// columns below are its coordinates.
    drawn: Arc<NyquistData>,
    real: Arc<[f64]>,
    imaginary: Arc<[f64]>,
    /// The negated imaginary column: the branch for negative frequencies.
    mirror: Arc<[f64]>,
    stability: NyquistStability,
}

/// What the sheet has to draw for the run shown.
enum Locus {
    Ready(NyquistDerived),
    /// A stored run's loop gain is being read; a later frame has it.
    Reading,
    /// The loop gain cannot be read, and why.
    Refused(String),
    /// The run shown measured no loop gain.
    Absent,
}

const NEEDS_LOOP_GAIN: &str =
    "No loop gain — run a stability (.STB) analysis to draw its Nyquist locus";

/// The shown run's locus, read through the reader service: whole, since the
/// encirclement count and the margins are searched over the whole sweep.
fn locus(state: &mut ResultsParts<'_>) -> Locus {
    let Some(source) = loop_gain_source(state.results_reads()) else {
        return Locus::Absent;
    };
    let version = state.model.simulation.view.data_version;
    let locus = read_locus(state.results_reads(), source);
    if let Locus::Ready(derived) = &locus
        && !state
            .results
            .nyquist
            .get(&source)
            .is_some_and(|held| held.version == version)
    {
        let nyquist = &mut state.results.nyquist;
        nyquist.retain(|_, held| held.version == version);
        nyquist.insert(source, derived.clone());
    }
    locus
}

/// The locus of `source` and the numbers read off it: the memo's while it
/// holds this generation's, else read afresh.
fn read_locus(state: ResultsReads<'_>, source: LoopGainSource) -> Locus {
    let version = state.model.simulation.view.data_version;
    if let Some(cached) = state.results.nyquist.get(&source)
        && cached.version == version
    {
        return Locus::Ready(cached.clone());
    }
    let Some(analysis) = state
        .model
        .simulation
        .active_run()
        .and_then(|run| run.analyses.get(source.analysis))
    else {
        return Locus::Absent;
    };
    let (real, imaginary) = (
        &analysis.waveforms[source.real],
        &analysis.waveforms[source.imaginary],
    );
    let (frequency, real, imaginary) = match (
        real.x.fetch_whole(),
        real.y.fetch_whole(),
        imaginary.y.fetch_whole(),
    ) {
        (Fetched::Ready(frequency), Fetched::Ready(real), Fetched::Ready(imaginary)) => {
            (frequency, real, imaginary)
        }
        (Fetched::Refused(reason), _, _)
        | (_, Fetched::Refused(reason), _)
        | (_, _, Fetched::Refused(reason)) => return Locus::Refused(reason),
        _ => return Locus::Reading,
    };
    super::frame_work::note(super::frame_work::DatasetWalk::LoopGainLocus);
    let locus = NyquistData::from_arrays(LOOP_GAIN_LOCUS_LABEL, &frequency, &real, &imaginary);
    let drawn = locus.drawn_locus();
    let derived = NyquistDerived {
        version,
        stability: NyquistStability {
            encirclements: locus.count_encirclements(),
            min_distance: locus.min_distance_from_critical(),
            gain_margin: locus.gain_margin(),
            phase_margin: locus.phase_margin(),
        },
        real: drawn.points.iter().map(|point| point.real).collect(),
        imaginary: drawn.points.iter().map(|point| point.imag).collect(),
        mirror: drawn.points.iter().map(|point| -point.imag).collect(),
        drawn: Arc::new(drawn),
        locus: Arc::new(locus),
    };
    Locus::Ready(derived)
}

/// The reader's significant digits, which the stability figures read at.
fn significant_digits(state: ResultsReads<'_>) -> usize {
    usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    )
}

/// Draw the shown run's loop gain and apply the viewer's local outcomes.
pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let significant_digits = significant_digits(state.results_reads());
    let derived = match locus(state) {
        Locus::Ready(derived) => derived,
        Locus::Reading => {
            well_hint(ui, rspice_results::waveform::BEING_READ);
            return;
        }
        Locus::Refused(reason) => {
            well_hint(ui, &reason);
            return;
        }
        Locus::Absent => {
            well_hint(ui, NEEDS_LOOP_GAIN);
            return;
        }
    };
    let source = NyquistPlot {
        curve: &derived.locus,
        drawn: &derived.drawn,
        real: &derived.real,
        imaginary: &derived.imaginary,
        mirror: &derived.mirror,
        stability: Some(derived.stability),
    };
    let results = &mut state.results;
    let viewer = super::ResultViewer::Nyquist;
    let mut view = results.session.plot_view(viewer, 0);
    let mut pin = results.session.rf_pin.get(&viewer).copied();
    if let Some(response) = view::show(
        ui,
        Some(source),
        &mut view,
        &mut pin,
        &mut results.session.cache,
        &quantity_policy,
        significant_digits,
    ) {
        super::record_drawn_axes(results, viewer, &response);
        if response.view.any() {
            *results.session.plot_view_mut(viewer, 0) = view;
        }
        if response.response.clicked() {
            match pin {
                Some(hit) => {
                    results.session.rf_pin.insert(viewer, hit);
                }
                None => {
                    results.session.rf_pin.remove(&viewer);
                }
            }
        }
    }
}

/// The sheet's bar: frame the critical point, as the sheet opens, or the
/// whole locus.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let Some(frame) = view::domain_bar(ui) else {
        return true;
    };
    let view = match frame {
        NyquistFrame::CriticalPoint => PlotView::default(),
        NyquistFrame::WholeLocus => {
            let Locus::Ready(derived) = locus(state) else {
                return true;
            };
            let (x, y) = view::locus_frame(&derived.real, &derived.imaginary);
            PlotView {
                x: Some(x),
                y: Some(y),
            }
        }
    };
    *state
        .results
        .session
        .plot_view_mut(super::ResultViewer::Nyquist, 0) = view;
    true
}

/// The loop's margins and crossovers as the card shows them, for a printed
/// Nyquist page; none while the run shows no loop gain.
pub(super) fn printed_figures(state: ResultsReads<'_>) -> rspice_results_ui::table::SheetPrint {
    use rspice_results_ui::table::SheetPrint;
    let Some(source) = loop_gain_source(state) else {
        return SheetPrint::default();
    };
    let Locus::Ready(derived) = read_locus(state, source) else {
        return SheetPrint::default();
    };
    SheetPrint::figures(
        state
            .model
            .simulation
            .active_run()
            .and_then(|run| run.analyses.get(source.analysis))
            .map(|analysis| analysis.id),
        view::card_figures(
            &derived.stability,
            &state.model.preferences.quantity_presentation_policy(),
            significant_digits(state),
        ),
    )
}

/// The stability numbers of the shown run's loop gain, in the inspector.
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let significant_digits = significant_digits(state.results_reads());
    let stability = match locus(state) {
        Locus::Ready(derived) => Some(derived.stability),
        _ => None,
    };
    view::right_panel(ui, stability, &quantity_policy, significant_digits);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
    use rspice_results::nyquist::{LOOP_GAIN_IMAGINARY_TRACE, LOOP_GAIN_REAL_TRACE};

    /// A stability analysis retaining `L = gain / (1 + jf/1 kHz)³` at ten
    /// points a decade from 1 Hz to 1 MHz, as its Re and Im traces.
    fn stability_analysis(gain: f64) -> AnalysisResult {
        let frequency: Vec<f64> = (0..=60).map(|i| 10f64.powf(f64::from(i) / 10.0)).collect();
        let (real, imaginary): (Vec<f64>, Vec<f64>) = frequency
            .iter()
            .map(|&f| {
                let l = num_complex::Complex64::new(gain, 0.0)
                    / num_complex::Complex64::new(1.0, f / 1.0e3).powi(3);
                (l.re, l.im)
            })
            .unzip();
        AnalysisResult::new(1, AnalysisType::Stb, "STB").with_waveforms(vec![
            RetainedTrace::new(LOOP_GAIN_REAL_TRACE, frequency.clone(), real, "#fff"),
            RetainedTrace::new(LOOP_GAIN_IMAGINARY_TRACE, frequency, imaginary, "#fff"),
        ])
    }

    fn run_of(analysis: AnalysisResult) -> SimulationRun {
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        run
    }

    fn gain_margin_db(state: &mut ResultsTestHost) -> Option<f64> {
        match locus(&mut state.results_parts()) {
            Locus::Ready(derived) => derived
                .stability
                .gain_margin
                .map(|margin| 20.0 * margin.value.log10()),
            _ => None,
        }
    }

    /// The locus is read off the shown run's retained loop gain, so a run in
    /// the history that nothing loaded draws, and each run draws its own.
    #[test]
    fn each_retained_stability_run_draws_its_own_locus() {
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![
            run_of(stability_analysis(4.0)),
            run_of(stability_analysis(2.0)),
        ]
        .into();
        assert!(state.model.simulation.select_run(0));
        let first = gain_margin_db(&mut state).expect("the first run's locus");
        assert!((first - 20.0 * 2f64.log10()).abs() < 1.0e-3, "{first}");

        assert!(state.model.simulation.select_run(1));
        let second = gain_margin_db(&mut state).expect("the second run's locus");
        assert!((second - 20.0 * 4f64.log10()).abs() < 1.0e-3, "{second}");
    }

    /// A stored stability run draws its locus from the results store: each
    /// column is read once, and the next frame reads nothing.
    #[test]
    fn a_stored_stability_run_reads_its_loop_gain_once() {
        let in_memory = stability_analysis(4.0);
        let column = |index: usize| {
            in_memory.waveforms[index]
                .y
                .in_memory()
                .expect("the fixture holds its samples")
                .to_vec()
        };
        let frequency = in_memory.waveforms[0]
            .x
            .in_memory()
            .expect("the fixture holds its frequencies")
            .to_vec();
        let (mut stored, columns) = super::super::test_fixtures::stored_transient(
            frequency,
            vec![
                (LOOP_GAIN_REAL_TRACE.to_owned(), column(0)),
                (LOOP_GAIN_IMAGINARY_TRACE.to_owned(), column(1)),
            ],
            None,
        );
        stored.analysis_type = AnalysisType::Stb;
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run_of(stored)].into();
        assert!(state.model.simulation.select_run(0));

        let margin = gain_margin_db(&mut state).expect("the stored run's locus");
        assert!((margin - 20.0 * 2f64.log10()).abs() < 1.0e-3, "{margin}");
        let reads: Vec<u64> = columns.iter().map(|column| column.whole_reads()).collect();
        assert_eq!(reads, [1, 1, 1], "each column is read whole once");
        assert!(gain_margin_db(&mut state).is_some());
        let again: Vec<u64> = columns.iter().map(|column| column.whole_reads()).collect();
        assert_eq!(again, reads, "the next frame reads nothing");
    }

    /// An AC response is a complex curve, not a loop gain: no locus.
    #[test]
    fn an_ac_response_draws_no_locus() {
        let mut analysis = stability_analysis(4.0);
        analysis.analysis_type = AnalysisType::Ac;
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run_of(analysis)].into();
        assert!(state.model.simulation.select_run(0));

        assert!(loop_gain_source(state.results_reads()).is_none());
        assert!(matches!(locus(&mut state.results_parts()), Locus::Absent));
    }
}
