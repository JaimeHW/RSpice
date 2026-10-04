//! Smith chart source qualification, selection and cache coordination.

use egui::Ui;
use rspice_results::network_matrix::trace_identity;

use crate::workbench::AppState;
use crate::workbench::app_state::{
    ActiveViewer, SpecializedViewerAnalysisIdentity, SpecializedViewerCacheProvenance,
};

use super::frame_work::{self, DatasetWalk};
use rspice_results_ui::smith_chart::view::{self, SmithPlotTrace};

fn trace_is_well_formed(waveform: &crate::state::WaveformData) -> bool {
    let Some(complex) = waveform.complex.as_ref() else {
        return false;
    };
    if waveform.x.is_empty()
        || waveform.x.len() != complex.real.len()
        || waveform.x.len() != complex.imag.len()
    {
        return false;
    }
    frame_work::note(DatasetWalk::SParameterTraceScan);
    waveform
        .x
        .iter()
        .all(|frequency| frequency.is_finite() && *frequency > 0.0)
        && waveform.x.windows(2).all(|pair| pair[0] < pair[1])
        && complex
            .real
            .iter()
            .chain(complex.imag.iter())
            .all(|value| value.is_finite())
}

pub(super) fn analysis_is_renderable(analysis: &crate::state::AnalysisResult) -> bool {
    frame_work::note(DatasetWalk::EvidenceValidation);
    analysis.validate_retained_evidence().is_ok() && structure_is_renderable(analysis)
}

/// What the tab strip says about this sheet, in the sheet's own words.
///
/// The verdict comes from the workspace memo rather than a fresh walk, which
/// is why it is spelled through the structural gate and not through
/// [`analysis_is_renderable`].
pub(super) fn availability(state: &AppState) -> super::ViewerAvailability {
    if state.simulation.active_run().is_some_and(|run| {
        state.simulation.active_analysis().is_some_and(|analysis| {
            super::analysis_answers_structural_gate(
                state,
                run.dataset_id,
                analysis,
                super::StructuralGate::SParameterStructure,
            ) && super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
        })
    }) {
        super::ViewerAvailability::available(
            "Retained S-parameter coefficients and per-port reference impedances are available",
        )
    } else {
        super::ViewerAvailability::unavailable(
            "Requires SP, PSP, or HBSP with exact complex traces and retained port impedances",
        )
    }
}

/// The same question with the retained-evidence verdict left to the caller.
///
/// Split out because the per-frame callers — the tab strip's availability
/// gate and the cache synchronizer — take that verdict from the workspace
/// memo instead of walking every retained complex sample again.
pub(super) fn structure_is_renderable(analysis: &crate::state::AnalysisResult) -> bool {
    if !analysis.success
        || !matches!(
            analysis.analysis_type,
            crate::state::AnalysisType::SParameter
                | crate::state::AnalysisType::Psp
                | crate::state::AnalysisType::Hbsp
        )
    {
        return false;
    }
    let Some(crate::state::AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
        ..
    }) = analysis.family_metadata.as_ref()
    else {
        return false;
    };
    analysis.waveforms.iter().any(|waveform| {
        let Some(identity) = trace_identity(&waveform.name) else {
            return false;
        };
        let port_count = if identity.physical_ports {
            reference_impedances_ohm.len()
        } else if reference_impedances_ohm.len().is_multiple_of(2) {
            reference_impedances_ohm.len() / 2
        } else {
            return false;
        };
        identity.output_port <= port_count
            && identity.input_port <= port_count
            && trace_is_well_formed(waveform)
    })
}

/// Rebuild the mutable drawing cache solely from the selected immutable
/// result. Cache provenance makes repeated frames a no-op and prevents a
/// same-shape result from inheriting another run's RF traces.
pub(super) fn synchronize_active_analysis(state: &mut AppState) -> bool {
    if state.viewer_capability(ActiveViewer::SmithChart).available {
        return true;
    }
    let Some(run) = state.simulation.active_run() else {
        state.analysis.smith_chart_state.clear_traces();
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        return false;
    };
    let Some(analysis) = state.simulation.active_analysis() else {
        state.analysis.smith_chart_state.clear_traces();
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        return false;
    };
    if !structure_is_renderable(analysis)
        || !super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
    {
        state.analysis.smith_chart_state.clear_traces();
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        return false;
    }
    let provenance = SpecializedViewerCacheProvenance::for_analysis(run.dataset_id, analysis);
    let crate::state::AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
        ..
    } = analysis
        .family_metadata
        .as_ref()
        .expect("renderable S-parameter metadata")
    else {
        unreachable!("renderability checked the S-parameter metadata variant");
    };
    let mut waveforms = analysis.waveforms.clone();
    let reference_impedances_ohm = reference_impedances_ohm.clone();
    waveforms.sort_by(|left, right| left.name.cmp(&right.name));

    state.analysis.smith_chart_state.clear_traces();
    for waveform in waveforms {
        let Some(identity) = trace_identity(&waveform.name) else {
            continue;
        };
        let port_count = if identity.physical_ports {
            reference_impedances_ohm.len()
        } else {
            reference_impedances_ohm.len() / 2
        };
        if identity.output_port > port_count || identity.input_port > port_count {
            continue;
        }
        let reference_impedance_ohm = (identity.physical_ports
            && identity.output_port == identity.input_port)
            .then(|| reference_impedances_ohm[identity.output_port - 1]);
        let Some(complex) = waveform.complex.as_ref() else {
            continue;
        };
        if state
            .analysis
            .smith_chart_state
            .load_sparam_data(
                &waveform.name,
                &waveform.x,
                &complex.real,
                &complex.imag,
                reference_impedance_ohm,
            )
            .is_err()
        {
            state.analysis.smith_chart_state.clear_traces();
            state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
            return false;
        }
    }
    if state.analysis.smith_chart_state.traces.is_empty() {
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        return false;
    }
    state.bind_specialized_viewer_cache(ActiveViewer::SmithChart, provenance);
    true
}

/// Present the qualified traces and apply local viewer outcomes.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    // Γ-plane component arrays per trace, cached per data version.
    //
    // The cache and the traces live in disjoint halves of the session, so the
    // arrays are built by reference. Copying the loci out first — to release
    // the borrow — meant every frame paid for a full copy of every visible
    // trace's points before finding out the cache already held them.
    //
    // The key carries the cache's owning result as well as the trace ordinal:
    // selecting another analysis of the same run reloads the loci without
    // moving the data version, and a bare ordinal would hand the new
    // selection the previous one's coefficients.
    let owner = state
        .active_specialized_viewer_cache_provenance()
        .map_or(0, |provenance| {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            provenance.dataset_id.hash(&mut hasher);
            match provenance.analysis_identity {
                SpecializedViewerAnalysisIdentity::Prepared(id) => {
                    id.hash(&mut hasher);
                }
                SpecializedViewerAnalysisIdentity::LegacyResultId(id) => id.hash(&mut hasher),
            }
            hasher.finish()
        });
    let smith = &state.analysis.smith_chart_state;
    let mut traces = Vec::new();
    for (index, trace) in smith
        .traces
        .iter()
        .enumerate()
        .filter(|(_, trace)| trace.visible && !trace.points.is_empty())
    {
        let points = &trace.points;
        let key = owner ^ ((index as u64) << 8);
        let derived = &mut state.ui.results.session.derived;
        let real = derived.get_or(key ^ 0x501_0000, || {
            std::sync::Arc::new(points.iter().map(|p| p.s.re).collect::<Vec<_>>())
        });
        let imaginary = derived.get_or(key ^ 0x501_0001, || {
            std::sync::Arc::new(points.iter().map(|p| p.s.im).collect::<Vec<_>>())
        });
        traces.push(SmithPlotTrace {
            index,
            trace,
            real,
            imaginary,
        });
    }
    let results = &mut state.ui.results;
    let viewer = super::ResultViewer::Smith;
    let mut view = results.session.plot_view(viewer, 0);
    let mut pin = results.session.rf_pin.get(&viewer).copied();
    if let Some(response) = view::show(
        ui,
        &traces,
        &mut view,
        &mut pin,
        &mut results.session.cache,
        &quantity_policy,
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

/// Present the selected source's trace summary and marker.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    view::right_panel(
        ui,
        &state.analysis.smith_chart_state,
        state
            .ui
            .results
            .session
            .rf_pin
            .get(&super::ResultViewer::Smith)
            .copied(),
        &state.ui.preferences.quantity_presentation_policy(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A retained S-parameter analysis whose S11 locus is `values`.
    fn sparam_analysis(
        id: u64,
        label: &str,
        values: &[(f64, f64)],
    ) -> crate::state::AnalysisResult {
        let x: Vec<f64> = (0..values.len())
            .map(|index| 1.0e9 * (index as f64 + 1.0))
            .collect();
        let waveform = crate::state::WaveformData::new(
            "S11",
            x.clone(),
            values.iter().map(|(re, _)| *re).collect::<Vec<_>>(),
            "#0af",
        )
        .with_complex_components(
            "S11",
            values.iter().map(|(re, _)| *re).collect::<Vec<_>>(),
            values.iter().map(|(_, im)| *im).collect::<Vec<_>>(),
        );
        crate::state::AnalysisResult::new(id, crate::state::AnalysisType::SParameter, label)
            .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
                noise_reference_temperature_kelvin: None,
                reference_impedances_ohm: vec![50.0, 50.0],
            })
            .with_waveforms(vec![waveform])
    }

    fn draw(state: &mut AppState) {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 700.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, state));
            },
        );
    }

    /// The Γ-component arrays are cached, and the cache belongs to the result
    /// that filled it. Selecting another analysis of the same run reloads the
    /// loci without moving the data version, so a cache keyed only by trace
    /// ordinal would draw the previous analysis' coefficients under the new
    /// one's name.
    #[test]
    fn the_gamma_arrays_belong_to_the_analysis_that_filled_them() {
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(sparam_analysis(1, "SP low", &[(0.1, 0.0), (0.2, 0.1)]));
        run.add_analysis(sparam_analysis(2, "SP high", &[(-0.6, 0.3), (-0.5, 0.4)]));
        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));

        assert!(synchronize_active_analysis(&mut state));
        draw(&mut state);
        let first = state.analysis.smith_chart_state.traces[0].points[0].s.re;
        assert!((first - 0.1).abs() < 1.0e-12, "{first}");

        assert!(state.simulation.select_analysis(1));
        assert!(synchronize_active_analysis(&mut state));
        draw(&mut state);
        let second = state.analysis.smith_chart_state.traces[0].points[0].s.re;
        assert!((second + 0.6).abs() < 1.0e-12, "{second}");

        // The cached component array has to have moved with the selection.
        let owner = state
            .active_specialized_viewer_cache_provenance()
            .expect("the reloaded cache names its owner");
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        {
            use std::hash::Hash as _;
            owner.dataset_id.hash(&mut hasher);
            match owner.analysis_identity {
                SpecializedViewerAnalysisIdentity::Prepared(id) => {
                    id.hash(&mut hasher);
                }
                SpecializedViewerAnalysisIdentity::LegacyResultId(id) => id.hash(&mut hasher),
            }
        }
        let key = {
            use std::hash::Hasher as _;
            hasher.finish()
        };
        let cached = state
            .ui
            .results
            .session
            .derived
            .get_or(key ^ 0x501_0000, || std::sync::Arc::new(Vec::new()));
        assert_eq!(
            cached.first().copied(),
            Some(-0.6),
            "the sheet drew the previous analysis' coefficients"
        );
    }
}
