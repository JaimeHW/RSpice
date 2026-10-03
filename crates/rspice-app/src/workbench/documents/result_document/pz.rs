//! Active pole-zero source qualification and shared plot-view action ownership.

use crate::{
    state::{AnalysisResultPayload, AnalysisType},
    workbench::AppState,
};
use egui::Ui;
use rspice_results::pole_zero::data::{ComplexRoot, PoleZeroData};
use rspice_results_ui::pole_zero as view;

fn active_data(state: &AppState) -> Option<PoleZeroData> {
    let analysis = state.simulation.active_analysis()?;
    if !analysis.success || analysis.analysis_type != AnalysisType::PoleZero {
        return None;
    }
    let payload = analysis.result_payload.as_ref()?;
    let AnalysisResultPayload::PoleZero {
        poles,
        zeros,
        pole_evidence,
        zero_evidence,
        gain,
    } = payload
    else {
        return None;
    };
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }

    let mut data = PoleZeroData::new(&analysis.label);
    data.gain = *gain;
    data.pole_evidence = pole_evidence.clone();
    data.zero_evidence = zero_evidence.clone();
    data.roots.extend(
        poles
            .iter()
            .map(|root| ComplexRoot::pole(root.real, root.imaginary)),
    );
    data.roots.extend(
        zeros
            .iter()
            .map(|root| ComplexRoot::zero(root.real, root.imaginary)),
    );
    Some(data)
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let data = active_data(state);
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let results = &mut state.ui.results;
    let viewer = super::ResultViewer::PoleZero;
    let mut plot_view = results.session.plot_view(viewer, 0);
    let mut pin = results.session.rf_pin.get(&viewer).copied();
    if let Some(response) = view::show(
        ui,
        data.as_ref(),
        &mut plot_view,
        &mut pin,
        &mut results.session.cache,
        &quantity_policy,
    ) {
        if response.fit_clicked {
            results.session.reset_plot_view(viewer, 0);
        }
        super::record_drawn_axes(results, viewer, &response.plot);
        if response.plot.view.any() {
            *results.session.plot_view_mut(viewer, 0) = plot_view;
        }
        if response.plot.response.clicked() {
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

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let data = active_data(state);
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    view::right_panel(ui, data.as_ref(), &quantity_policy);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultPayload, AnalysisType, ComplexResultValue, SimulationRun,
    };

    fn qualified_evidence(root_count: u64) -> crate::state::PoleZeroRootSetEvidence {
        let certificate = crate::state::PoleZeroSpectrumCertificate {
            problem_order: root_count,
            infinite_count: 0,
            max_backward_error: 1.0e-14,
            qualification_tolerance:
                crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(
                    root_count,
                )
                .unwrap(),
        };
        if root_count == 0 {
            crate::state::PoleZeroRootSetEvidence::QualifiedEmpty { certificate }
        } else {
            crate::state::PoleZeroRootSetEvidence::Qualified { certificate }
        }
    }

    #[test]
    fn retained_payload_is_the_only_pole_zero_viewer_authority() {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(7, AnalysisType::PoleZero, "PZ 7"));
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(active_data(&state).is_none());

        state.simulation.runs[0].analyses[0] =
            AnalysisResult::new(7, AnalysisType::PoleZero, "PZ 7").with_result_payload(
                AnalysisResultPayload::PoleZero {
                    poles: vec![
                        ComplexResultValue {
                            real: -10.0,
                            imaginary: 20.0,
                        },
                        ComplexResultValue {
                            real: -10.0,
                            imaginary: -20.0,
                        },
                    ],
                    zeros: vec![ComplexResultValue {
                        real: -3.0,
                        imaginary: 0.0,
                    }],
                    pole_evidence: qualified_evidence(2),
                    zero_evidence: qualified_evidence(1),
                    gain: Some(4.25),
                },
            );

        let data = active_data(&state).expect("retained PZ payload");
        assert_eq!(data.name, "PZ 7");
        assert_eq!(data.gain, Some(4.25));
        assert_eq!(data.roots.len(), 3);
        assert!(data.roots[0].is_pole());
        assert_eq!((data.roots[0].real, data.roots[0].imag), (-10.0, 20.0));
        assert!(data.roots[1].is_pole());
        assert_eq!((data.roots[1].real, data.roots[1].imag), (-10.0, -20.0));
        assert_eq!(
            data.roots[2].root_type,
            rspice_results::pole_zero::data::RootType::Zero
        );
        assert_eq!((data.roots[2].real, data.roots[2].imag), (-3.0, 0.0));
    }
}
