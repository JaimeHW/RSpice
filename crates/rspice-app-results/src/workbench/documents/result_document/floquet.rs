//! Periodic stability: the Floquet spectrum of a PSTB run, or of a PSS run
//! that computed its stability, read from its validated payload for the
//! Floquet sheet.

use super::{ResultViewer, ResultsParts, ResultsReads, ViewerAvailability};
use crate::state::{AnalysisResult, AnalysisResultPayload};
use egui::Ui;
use rspice_results_ui::floquet as view;

/// The Floquet spectrum `analysis` holds, as the sheet reads it: a payload
/// that validates and has at least one mode.
fn data_of(analysis: &AnalysisResult) -> Option<view::FloquetData> {
    if !analysis.success {
        return None;
    }
    let payload = analysis.result_payload.as_ref()?;
    match payload {
        AnalysisResultPayload::Pstb { modes, .. } if modes.is_empty() => return None,
        AnalysisResultPayload::PssFloquet { multipliers, .. } if multipliers.is_empty() => {
            return None;
        }
        AnalysisResultPayload::Pstb { .. } | AnalysisResultPayload::PssFloquet { .. } => {}
        _ => return None,
    }
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    Some(match payload {
        AnalysisResultPayload::Pstb {
            period_s,
            stability_threshold,
            probe_instance,
            modes,
            orbit_kind,
            stability_verdict,
            stability_classification,
            ..
        } => view::FloquetData {
            label: analysis.label.clone(),
            kind: "PSTB",
            modes: modes
                .iter()
                .map(|mode| view::FloquetMode {
                    multiplier: (mode.multiplier.real, mode.multiplier.imaginary),
                    exponent: (mode.exponent.real, mode.exponent.imaginary),
                    participation: Some(mode.probe_participation),
                    trivial: mode.is_trivial,
                    subharmonic: mode.subharmonic_order,
                })
                .collect(),
            // The run's boundary is a magnitude; past it a mode is unstable.
            band: stability_threshold - 1.0,
            verdict: *stability_verdict,
            classification: Some(*stability_classification),
            orbit: *orbit_kind,
            period_s: *period_s,
            probe: Some(probe_instance.clone()),
        },
        AnalysisResultPayload::PssFloquet {
            period_s,
            multipliers,
            orbit_kind,
            trivial_multiplier_index,
            stability_verdict,
            ..
        } => view::FloquetData {
            label: analysis.label.clone(),
            kind: "PSS",
            modes: multipliers
                .iter()
                .enumerate()
                .map(|(index, mode)| {
                    let multiplier = (mode.multiplier.real, mode.multiplier.imaginary);
                    view::FloquetMode {
                        multiplier,
                        exponent: view::exponent_of(multiplier, *period_s),
                        participation: None,
                        trivial: *trivial_multiplier_index == Some(index as u64),
                        subharmonic: None,
                    }
                })
                .collect(),
            // A steady state is classified about the unit circle itself,
            // within the solver's numerical band.
            band: rspice_core::analysis::FLOQUET_UNIT_CIRCLE_BAND,
            verdict: *stability_verdict,
            classification: None,
            orbit: *orbit_kind,
            period_s: *period_s,
            probe: None,
        },
        _ => return None,
    })
}

/// Whether `analysis` holds a spectrum the sheet can draw.
pub(super) fn analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    data_of(analysis).is_some()
}

/// The analysis the sheet reads: the one shown when it holds a spectrum,
/// else the run's first that does.
fn data(state: ResultsReads<'_>) -> Option<view::FloquetData> {
    let run = state.model.simulation.active_run()?;
    state
        .model
        .simulation
        .active_analysis()
        .and_then(data_of)
        .or_else(|| run.analyses.iter().find_map(data_of))
}

pub(super) fn availability(state: ResultsReads<'_>) -> ViewerAvailability {
    if data(state).is_some() {
        ViewerAvailability::available("Floquet multipliers are available on the unit circle")
    } else {
        ViewerAvailability::unavailable(
            "Requires a PSTB analysis, or a PSS analysis with its stability computed",
        )
    }
}

fn digits(state: ResultsReads<'_>) -> usize {
    usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    )
}

pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let data = data(state.results_reads());
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let digits = digits(state.results_reads());
    let viewer = ResultViewer::Floquet;
    let results = &mut state.results;
    let mut plot_view = results.session.plot_view(viewer, 0);
    let mut pin = results.session.rf_pin.get(&viewer).copied();
    let Some(response) = view::show(
        ui,
        data.as_ref(),
        &mut plot_view,
        &mut pin,
        &mut results.session.cache,
        &quantity_policy,
        digits,
    ) else {
        return;
    };
    if response.fit_clicked {
        results.session.reset_plot_view(viewer, 0);
    }
    super::record_drawn_axes(results, viewer, &response.plot);
    if response.plot.view.any() {
        *results.session.plot_view_mut(viewer, 0) = plot_view;
    }
    if response.pin_changed {
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

pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let reads = state.results_reads();
    view::right_panel(ui, data(reads).as_ref(), digits(reads));
}

#[cfg(test)]
mod tests {
    use super::super::test_fixtures::pstb_analysis;
    use super::super::test_host::ResultsTestHost;
    use super::*;
    use crate::state::{
        FloquetStabilityVerdictEvidence, PstbStabilityClassificationEvidence, SimulationRun,
        SimulationRunLifecycle,
    };

    fn host_with(analysis: AnalysisResult) -> ResultsTestHost {
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        run.lifecycle = SimulationRunLifecycle::Completed;
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        state
    }

    /// A stability run opens on its multipliers, and the sheet's verdict,
    /// unstable count and margin are the ones the run's payload holds.
    #[test]
    fn a_stability_run_opens_on_the_floquet_sheet_with_the_verdict_its_payload_holds() {
        let analysis = pstb_analysis();
        let Some(AnalysisResultPayload::Pstb {
            stability_verdict,
            stability_classification,
            min_stability_margin_db,
            num_unstable,
            modes,
            ..
        }) = analysis.result_payload.clone()
        else {
            unreachable!("the fixture is a stability run");
        };
        assert_eq!(stability_verdict, FloquetStabilityVerdictEvidence::Unstable);
        assert_eq!(
            stability_classification,
            PstbStabilityClassificationEvidence::UnstableReal
        );
        let mut state = host_with(analysis);
        state.quick_frame();

        assert_eq!(
            super::super::opening_sheet(state.results_reads()),
            Some(ResultViewer::Floquet)
        );
        let data = data(state.results_reads()).expect("the run's spectrum");
        assert_eq!(data.verdict, stability_verdict);
        assert_eq!(data.classification, Some(stability_classification));
        assert_eq!(view::verdict_text(&data), "Unstable · real multiplier");
        assert_eq!(data.unstable_count() as u64, num_unstable);
        assert_eq!(data.min_margin_db(), min_stability_margin_db);
        assert_eq!(data.modes.len(), modes.len());
        // The phase of the outside multiplier is the half-period flip.
        let flip = data.modes[0];
        assert_eq!(flip.magnitude(), 1.25);
        assert_eq!(flip.angle_rad(), std::f64::consts::PI);

        state.results.results.session.viewer = ResultViewer::Floquet;
        state.quick_frame();
    }

    /// A run whose spectrum contradicts its verdict draws nothing: the sheet
    /// shows only what validates.
    #[test]
    fn a_spectrum_that_contradicts_its_verdict_is_not_drawn() {
        let mut analysis = pstb_analysis();
        if let Some(AnalysisResultPayload::Pstb {
            stability_verdict, ..
        }) = analysis.result_payload.as_mut()
        {
            *stability_verdict = FloquetStabilityVerdictEvidence::Stable;
        }
        let state = host_with(analysis);
        assert!(data(state.results_reads()).is_none());
        assert!(!availability(state.results_reads()).available);
    }
}
