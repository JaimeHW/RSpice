//! The readout a plot sheet pinned, kept per authored analysis and sheet.
//!
//! The session holds the pins the sheets draw; the run's working view keeps
//! them, one per pane and sheet. Before the quick view draws, each pane's
//! pins are taken into the session, and a pin the reader placed, moved or
//! took away is written back through the view's one writer afterwards. A
//! pin names the authored analysis, not the one solve of it, so the next
//! run of the plan reads its own values at the same X.

use std::collections::HashMap;

use rspice_results::result_presentation::AnalysisPresentationSource;

use super::{AnalysisPresentationKey, ResultViewer, ResultsParts, ResultsReads};
use crate::results::visualization_document::{DocumentEdit, PaneId, VisualizationDocument};

/// Every pin, by the authored analysis and the sheet it was placed on.
pub(super) type Pins = HashMap<(AnalysisPresentationSource, ResultViewer), f64>;

/// The authored analysis the plot sheets draw: the shown run's active one.
fn shown(state: ResultsReads<'_>) -> Option<AnalysisPresentationSource> {
    let simulation = &state.model.simulation;
    let run = simulation.active_run()?;
    let analysis = simulation.active_analysis()?;
    Some(AnalysisPresentationKey::new(run.dataset_id, analysis).authored())
}

/// The X `sheet`'s readout is pinned at for the analysis shown.
pub(super) fn pinned(state: ResultsReads<'_>, sheet: ResultViewer) -> Option<f64> {
    let source = shown(state)?;
    state
        .results
        .session
        .readout_pins
        .get(&(source, sheet))
        .copied()
}

/// Keep the pin the reader placed on `sheet`, or with `None` take it away.
pub(super) fn keep(state: &mut ResultsParts<'_>, sheet: ResultViewer, pin: Option<f64>) {
    let Some(source) = shown(state.results_reads()) else {
        return;
    };
    let pins = &mut state.results.session.readout_pins;
    match pin {
        Some(x) => {
            pins.insert((source, sheet), x);
        }
        None => {
            pins.remove(&(source, sheet));
        }
    }
}

/// Take `pane_id`'s pins in `document` into the session for `key`.
pub(super) fn project(
    pins: &mut Pins,
    document: &VisualizationDocument,
    key: AnalysisPresentationKey,
    pane_id: PaneId,
) {
    let source = key.authored();
    pins.retain(|(held, _), _| *held != source);
    pins.extend(
        document
            .readout_pins()
            .iter()
            .filter(|pin| pin.pane_id == pane_id)
            .map(|pin| ((source, pin.sheet), pin.x)),
    );
}

/// The session's pins for the analyses `keys` name.
pub(super) fn scratch(pins: &Pins, keys: &[(AnalysisPresentationKey, PaneId)]) -> Pins {
    pins.iter()
        .filter(|((source, _), _)| keys.iter().any(|(key, _)| key.authored() == *source))
        .map(|(held, x)| (*held, *x))
        .collect()
}

/// The edits that make `pane_id` in `document` hold the session's pins for
/// `key`.
pub(super) fn edits_for(
    pins: &Pins,
    document: &VisualizationDocument,
    key: AnalysisPresentationKey,
    pane_id: PaneId,
) -> Vec<DocumentEdit> {
    let source = key.authored();
    let held: HashMap<ResultViewer, f64> = document
        .readout_pins()
        .iter()
        .filter(|pin| pin.pane_id == pane_id)
        .map(|pin| (pin.sheet, pin.x))
        .collect();
    let wanted: HashMap<ResultViewer, f64> = pins
        .iter()
        .filter(|((held, _), _)| *held == source)
        .map(|((_, sheet), x)| (*sheet, *x))
        .collect();
    let placed = wanted
        .iter()
        .filter(|(sheet, x)| held.get(*sheet) != Some(*x))
        .map(|(sheet, x)| DocumentEdit::SetReadoutPin {
            pane_id,
            sheet: *sheet,
            x: Some(*x),
        });
    let taken = held
        .keys()
        .filter(|sheet| !wanted.contains_key(*sheet))
        .map(|sheet| DocumentEdit::SetReadoutPin {
            pane_id,
            sheet: *sheet,
            x: None,
        });
    placed.chain(taken).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResult, AnalysisResultProvenance, AnalysisType, RetainedTrace, SimulationRun,
    };
    use crate::workbench::documents::result_document::test_fixtures::{
        completed_prepared_run, digest,
    };
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;

    /// A finished run of `plan` solving the transient `analysis`.
    fn plan_run(plan: SimulationPlanId, analysis: AnalysisInstanceId) -> SimulationRun {
        let mut run = completed_prepared_run(plan, ObjectRevision::INITIAL, digest(0x50));
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![RetainedTrace::new(
                    "V(out)",
                    vec![0.0, 1.0],
                    vec![0.0, 1.0],
                    "#ffbd2e",
                )])
                .with_provenance(
                    AnalysisResultProvenance::new(
                        analysis,
                        ObjectRevision::INITIAL,
                        digest(0x51),
                        Vec::new(),
                    )
                    .expect("analysis provenance"),
                ),
        );
        run
    }

    fn view_pins(state: &ResultsTestHost) -> Vec<(ResultViewer, f64)> {
        state
            .working_view()
            .expect("the run's view")
            .readout_pins()
            .iter()
            .map(|pin| (pin.sheet, pin.x))
            .collect()
    }

    /// A readout pinned on a sheet is kept by the plan's working view, and
    /// the next run of the plan reads at the same X even when nothing but
    /// the view remembers it.
    #[test]
    fn a_pinned_readout_survives_a_rerun() {
        let (plan, analysis) = (SimulationPlanId::new(), AnalysisInstanceId::new());
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![plan_run(plan, analysis)].into();
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        keep(&mut state.results_parts(), ResultViewer::Fft, Some(1.0e3));
        state.quick_frame();
        assert_eq!(view_pins(&state), [(ResultViewer::Fft, 1.0e3)]);

        // The quick view lets go of what it drew and the session forgets
        // the pin, as a restart does; then the plan runs again.
        super::super::working_view::release(&mut state.results_parts());
        state.results.results.session.readout_pins.clear();
        state
            .model
            .simulation
            .retained
            .runs
            .insert(0, plan_run(plan, analysis));
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        assert_eq!(
            pinned(state.results_reads(), ResultViewer::Fft),
            Some(1.0e3),
            "the re-run reads at the pinned X"
        );
        assert_eq!(view_pins(&state), [(ResultViewer::Fft, 1.0e3)]);

        keep(&mut state.results_parts(), ResultViewer::Fft, None);
        state.quick_frame();
        assert!(view_pins(&state).is_empty(), "unpinning leaves the view");
    }
}
