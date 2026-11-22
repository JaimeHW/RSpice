//! Phase-noise source qualification, retained trace binding, the sheet's
//! kept settings and viewport ownership.

use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
#[cfg(test)]
use crate::state::{AnalysisResultFamilyMetadata, PeriodicNoiseOutputQuantity};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use egui::Ui;
#[cfg(test)]
use rspice_results::phase_noise::retained_measurement;
use rspice_results::phase_noise::{retained_device_noise_shares, retained_phase_noise_carrier};
use rspice_results::visualization_document::SheetSettingValue;
use rspice_results::waveform::HeldColumn;
use rspice_results_ui::phase_noise as view;

/// One phase-noise waveform of the active immutable run.
struct PhaseNoiseSource {
    analysis_index: usize,
    waveform_index: usize,
    label: String,
    carrier_frequency_hz: f64,
    offset_hz: HeldColumn,
    level_dbc_per_hz: HeldColumn,
}

/// Every phase-noise trace the sheet draws: the selected analysis' first,
/// then those of the run's other phase-noise analyses, in run order.
struct PhaseNoiseModel {
    analysis_index: usize,
    sources: Vec<PhaseNoiseSource>,
}

impl PhaseNoiseModel {
    fn traces(&self) -> Vec<view::PhaseNoiseTrace<'_>> {
        self.sources
            .iter()
            .map(|source| view::PhaseNoiseTrace {
                label: &source.label,
                carrier_frequency_hz: source.carrier_frequency_hz,
                offset_hz: &source.offset_hz,
                level_dbc_per_hz: &source.level_dbc_per_hz,
                cache_key: 0x504E_0000_u64
                    | ((source.analysis_index as u64) << 16)
                    | source.waveform_index as u64,
            })
            .collect()
    }

    /// The offsets the traces span together.
    fn sweep(&self) -> Option<(f64, f64)> {
        self.sources
            .iter()
            .filter_map(|source| Some((*source.offset_hz.first()?, *source.offset_hz.last()?)))
            .reduce(|(a0, a1), (b0, b1)| (a0.min(b0), a1.max(b1)))
    }

    fn level_range(&self) -> Option<(f64, f64)> {
        self.sources
            .iter()
            .filter_map(|source| finite_range(&source.level_dbc_per_hz))
            .reduce(|(a0, a1), (b0, b1)| (a0.min(b0), a1.max(b1)))
    }
}

pub(super) fn phase_noise_waveform_is_renderable(waveform: &RetainedTrace) -> bool {
    rspice_results::phase_noise::phase_noise_waveform_is_renderable(waveform.as_ref(), || {
        super::frame_work::note(super::frame_work::DatasetWalk::PhaseNoiseSpectrumScan);
    })
}

pub(super) fn phase_noise_is_renderable(analysis: &AnalysisResult) -> bool {
    rspice_results::phase_noise::phase_noise_is_renderable(
        analysis.success,
        analysis.analysis_type,
        analysis.family_metadata.as_ref(),
        &analysis.waveforms,
        || super::frame_work::note(super::frame_work::DatasetWalk::PhaseNoiseSpectrumScan),
    )
}

/// Every analysis of the active run the sheet reads, the selected one first.
fn phase_noise_analysis_indices(state: ResultsReads<'_>) -> Vec<usize> {
    let Some(run) = state.model.simulation.active_run() else {
        return Vec::new();
    };
    // Through the workspace memo: the predicate walks every retained offset
    // and level, and this resolves on every frame the sheet is open.
    let renderable = |analysis: &AnalysisResult| {
        super::analysis_answers_structural_gate(
            state,
            run.dataset_id,
            analysis,
            super::StructuralGate::PhaseNoiseSpectrum,
        )
    };
    let mut indices: Vec<usize> = run
        .analyses
        .iter()
        .enumerate()
        .filter(|(_, analysis)| renderable(analysis))
        .map(|(index, _)| index)
        .collect();
    if let Some(selected) = state.model.simulation.view.active_analysis_idx
        && let Some(at) = indices.iter().position(|&index| index == selected)
    {
        indices[..=at].rotate_right(1);
    }
    indices
}

/// The renderable phase-noise traces of one analysis, each read whole once
/// it has arrived; `several` names each by its analysis as well.
fn sources_of(
    analysis_index: usize,
    analysis: &AnalysisResult,
    several: bool,
) -> Vec<PhaseNoiseSource> {
    if !phase_noise_is_renderable(analysis) {
        return Vec::new();
    }
    let Some(carrier_frequency_hz) =
        retained_phase_noise_carrier(analysis.analysis_type, analysis.family_metadata.as_ref())
    else {
        return Vec::new();
    };
    analysis
        .waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| phase_noise_waveform_is_renderable(waveform))
        .filter_map(|(waveform_index, waveform)| {
            Some(PhaseNoiseSource {
                analysis_index,
                waveform_index,
                label: if several {
                    format!("{} · {}", analysis.label, waveform.name)
                } else {
                    waveform.name.clone()
                },
                carrier_frequency_hz,
                // The sheet reads each spectrum whole: a stored one is read off
                // the frame, and the trace joins once it has arrived.
                offset_hz: waveform.x.fetch_whole().ready()?,
                level_dbc_per_hz: waveform.y.fetch_whole().ready()?,
            })
        })
        .collect()
}

fn model_from_analyses(analyses: &[AnalysisResult], indices: &[usize]) -> Option<PhaseNoiseModel> {
    let several = indices.len() > 1;
    let sources: Vec<PhaseNoiseSource> = indices
        .iter()
        .filter_map(|&index| Some((index, analyses.get(index)?)))
        .flat_map(|(index, analysis)| sources_of(index, analysis, several))
        .collect();
    let analysis_index = *indices.first()?;
    (!sources.is_empty()).then_some(PhaseNoiseModel {
        analysis_index,
        sources,
    })
}

fn build_model(state: ResultsReads<'_>) -> Option<PhaseNoiseModel> {
    let run = state.model.simulation.active_run()?;
    model_from_analyses(&run.analyses, &phase_noise_analysis_indices(state))
}

fn active_periodic_noise_without_phase_trace(state: ResultsReads<'_>) -> bool {
    let Some(run) = state.model.simulation.active_run() else {
        return false;
    };
    let Some(index) = state.model.simulation.view.active_analysis_idx else {
        return false;
    };
    let Some(analysis) = run.analyses.get(index) else {
        return false;
    };
    matches!(
        analysis.analysis_type,
        AnalysisType::Pnoise | AnalysisType::Qpnoise
    ) && !phase_noise_is_renderable(analysis)
}

fn finite_range(values: &[f64]) -> Option<(f64, f64)> {
    rspice_results::measurements::finite_extremes(values)
}

/// What the reader set the sheet to read, as the view keeps it, or what a
/// sweep opens with.
fn settings(state: ResultsReads<'_>, sweep: Option<(f64, f64)>) -> view::PhaseNoiseSettings {
    match super::sheet_settings::kept(state, super::ResultViewer::PhaseNoise) {
        Some(SheetSettingValue::PhaseNoise {
            spots,
            band,
            slopes,
        }) => view::PhaseNoiseSettings {
            spots,
            band: band.map(|[low, high]| (low, high)),
            slopes,
        },
        _ => sweep.map_or(
            view::PhaseNoiseSettings {
                spots: Vec::new(),
                band: None,
                slopes: false,
            },
            view::PhaseNoiseSettings::for_sweep,
        ),
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

/// Keep what the reader set the sheet to read in the view, where undo steps
/// it back and a re-run reads it again.
fn keep_settings(state: &mut ResultsParts<'_>, settings: view::PhaseNoiseSettings) {
    super::sheet_settings::keep(
        state,
        super::ResultViewer::PhaseNoise,
        Some(SheetSettingValue::PhaseNoise {
            spots: settings.spots,
            band: settings.band.map(|(low, high)| [low, high]),
            slopes: settings.slopes,
        }),
    );
}

/// The sheet's bar: spots, band and slope guides.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let sweep = build_model(state.results_reads()).and_then(|model| model.sweep());
    let current = settings(state.results_reads(), sweep);
    let digits = digits(state.results_reads());
    if let Some(changed) = view::domain_bar(ui, &current, sweep, digits) {
        keep_settings(state, changed);
    }
    true
}

/// Resolve the displayed sources and apply requests to the phase-noise viewport.
pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let quantities = state.model.preferences.quantity_presentation_policy();
    let presentation = state.results_reads().presentation_policy().readout();
    let Some(model) = build_model(state.results_reads()) else {
        view::show_absent(
            ui,
            active_periodic_noise_without_phase_trace(state.results_reads()),
        );
        return;
    };
    let settings = settings(state.results_reads(), model.sweep());
    let viewport = state
        .results
        .session
        .plot_view(super::ResultViewer::PhaseNoise, 0);
    let sheet = super::ResultViewer::PhaseNoise;
    let mut pin = super::readout_pins::pinned(state.results_reads(), sheet);
    let pinned = pin;
    let traces = model.traces();
    let response = view::show(
        ui,
        &traces,
        &settings,
        viewport,
        model.level_range(),
        &quantities,
        presentation,
        &mut state.results.session.cache,
        &mut pin,
    );
    if pin != pinned {
        super::readout_pins::keep(state, sheet, pin);
    }
    if response.fit {
        state
            .results
            .session
            .reset_plot_view(super::ResultViewer::PhaseNoise, 0);
    }
    if let Some(response) = response.plot {
        super::record_drawn_axes(state.results, super::ResultViewer::PhaseNoise, &response);
        if response.view.any() {
            state
                .results
                .session
                .plot_view_mut(super::ResultViewer::PhaseNoise, 0)
                .apply(&response.view);
        }
    }
}

/// The inspector: the carrier, the spots, the band's figures and the
/// selected analysis' device shares.
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let quantities = state.model.preferences.quantity_presentation_policy();
    let digits = digits(state.results_reads());
    let model = build_model(state.results_reads());
    let settings = settings(
        state.results_reads(),
        model.as_ref().and_then(PhaseNoiseModel::sweep),
    );
    let traces = model.as_ref().map(PhaseNoiseModel::traces);
    let input = match (&model, &traces) {
        (Some(model), Some(traces)) => {
            let analysis = state
                .model
                .simulation
                .active_run()
                .and_then(|run| run.analyses.get(model.analysis_index));
            Some(view::PhaseNoiseInspector {
                traces,
                settings: &settings,
                device_shares: analysis.map_or_else(Vec::new, |analysis| {
                    retained_device_noise_shares(&analysis.measurements).collect()
                }),
            })
        }
        _ => None,
    };
    view::right_panel(ui, input, &quantities, digits);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pnoise_inspector_uses_retained_jitter_and_never_a_failed_measurement() {
        let mut analysis = phase_result(1, AnalysisType::Pnoise, "PNOISE");
        analysis.measurements = vec![
            rspice_core::MeasureResult::success("phase_error_rms_rad", 0.02),
            rspice_core::MeasureResult::success("timing_jitter_rms_s", 3e-12),
        ];
        assert_eq!(
            retained_measurement(&analysis.data, "phase_error_rms_rad"),
            Some(0.02)
        );
        assert_eq!(
            retained_measurement(&analysis.data, "timing_jitter_rms_s"),
            Some(3e-12)
        );
        analysis.measurements[1].passed = false;
        assert_eq!(
            retained_measurement(&analysis.data, "timing_jitter_rms_s"),
            None
        );
        analysis
            .data
            .measurements
            .push(analysis.data.measurements[0].clone());
        assert_eq!(
            retained_measurement(&analysis.data, "phase_error_rms_rad"),
            None
        );
    }

    fn waveform(name: &str) -> RetainedTrace {
        RetainedTrace::new(
            name,
            vec![1.0, 1.0e3, 1.0e6],
            vec![-72.0, -103.0, -132.0],
            "#f5b700",
        )
    }

    fn phase_result(id: u64, analysis_type: AnalysisType, label: &str) -> AnalysisResult {
        AnalysisResult::new(id, analysis_type, label)
            .with_waveforms(vec![waveform("phase_noise")])
            .with_family_metadata(AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(2.4e9),
            })
    }

    #[test]
    fn phase_noise_model_requires_explicit_trace_evidence() {
        let ordinary = AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE")
            .with_waveforms(vec![waveform("onoise")]);
        assert!(!phase_noise_is_renderable(&ordinary));
        assert!(model_from_analyses(std::slice::from_ref(&ordinary), &[0]).is_none());

        let phase = phase_result(2, AnalysisType::Pnoise, "PNOISE phase");
        let analyses = [ordinary.clone(), phase.clone()];
        let model = model_from_analyses(&analyses, &[1]).expect("explicit phase trace renders");
        assert_eq!(model.analysis_index, 1);
        assert_eq!(model.sources[0].label, "phase_noise");
        assert!(phase_noise_is_renderable(&phase));
    }

    #[test]
    fn qpnoise_and_l_of_f_are_accepted_but_other_analysis_kinds_are_not() {
        let mut qpnoise = phase_result(1, AnalysisType::Qpnoise, "QPNOISE");
        qpnoise.waveforms = vec![waveform("L(f)")];
        assert!(phase_noise_is_renderable(&qpnoise));

        let mut noise = AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
            .with_waveforms(vec![waveform("phase noise")]);
        noise.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(2.4e9),
        });
        assert!(!phase_noise_is_renderable(&noise));
    }

    #[test]
    fn failed_phase_noise_analysis_never_becomes_renderable_evidence() {
        let mut failed = phase_result(1, AnalysisType::Pnoise, "PNOISE failed");
        failed.success = false;

        assert!(!phase_noise_is_renderable(&failed));
        assert!(model_from_analyses(std::slice::from_ref(&failed), &[0]).is_none());
    }

    #[test]
    fn phase_noise_requires_typed_quantity_and_retained_carrier() {
        let output_psd = AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE output")
            .with_waveforms(vec![waveform("phase_noise")])
            .with_family_metadata(AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
                carrier_frequency_hz: Some(2.4e9),
            });
        assert!(!phase_noise_is_renderable(&output_psd));

        let mut missing_carrier = AnalysisResult::new(2, AnalysisType::Pnoise, "PNOISE phase")
            .with_waveforms(vec![waveform("phase_noise")]);
        missing_carrier.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: None,
        });
        assert!(!phase_noise_is_renderable(&missing_carrier));
    }

    /// A stored PNOISE run of `plan` whose L(f) falls 20 dB/decade from
    /// −60 dBc/Hz at 1 kHz, sampled each decade to 10 MHz.
    fn stored_phase_noise_run(
        plan: crate::product::SimulationPlanId,
        analysis: crate::product::AnalysisInstanceId,
    ) -> crate::state::SimulationRun {
        use super::super::test_fixtures::{completed_prepared_run, digest, stored_transient};
        let offsets = vec![1.0e3, 1.0e4, 1.0e5, 1.0e6, 1.0e7];
        let levels = offsets
            .iter()
            .map(|offset: &f64| -60.0 - 20.0 * (offset / 1.0e3).log10())
            .collect();
        let (mut stored, _) = stored_transient(
            offsets,
            vec![("phase_noise".to_owned(), levels)],
            Some([0x3d; 32]),
        );
        stored.analysis_type = AnalysisType::Pnoise;
        stored.label = "PNOISE".to_owned();
        stored.family_metadata = Some(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(1.0e9),
        });
        let mut run =
            completed_prepared_run(plan, crate::product::ObjectRevision::INITIAL, digest(0x3d));
        run.add_analysis(
            stored.with_provenance(
                crate::state::AnalysisResultProvenance::new(
                    analysis,
                    crate::product::ObjectRevision::INITIAL,
                    digest(0x3e),
                    Vec::new(),
                )
                .expect("analysis provenance"),
            ),
        );
        run
    }

    /// A stored run opens with a spot at each decade inside its sweep; the
    /// spots, band and slope guides the reader sets are the view's, so the
    /// next run of the plan reads L(f) at the same spots, between samples,
    /// and integrates the same band, and undo steps a change back.
    #[test]
    fn the_spots_and_band_the_reader_sets_are_kept_by_the_view_and_read_between_samples() {
        use super::super::test_host::ResultsTestHost;
        use super::super::view_edits;
        let (plan, analysis) = (
            crate::product::SimulationPlanId::new(),
            crate::product::AnalysisInstanceId::new(),
        );
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![stored_phase_noise_run(plan, analysis)].into();
        assert!(state.model.simulation.select_run(0));
        state.results.results.session.viewer = super::super::ResultViewer::PhaseNoise;
        state.quick_frame();

        let model = build_model(state.results_reads()).expect("the stored trace is read");
        assert_eq!(model.sweep(), Some((1.0e3, 1.0e7)));
        let opened = settings(state.results_reads(), model.sweep());
        assert_eq!(opened.spots, [1.0e4, 1.0e5, 1.0e6]);
        assert_eq!(opened.band, None);

        let chosen = view::PhaseNoiseSettings {
            spots: vec![3.0e4, 2.0e5],
            band: Some((1.0e4, 1.0e6)),
            slopes: true,
        };
        keep_settings(&mut state.results_parts(), chosen.clone());
        state.quick_frame();

        // The session forgets them, as a restart does; then the plan runs
        // again.
        super::super::working_view::release(&mut state.results_parts());
        state.results.results.session.sheet_settings.clear();
        state
            .model
            .simulation
            .retained
            .runs
            .insert(0, stored_phase_noise_run(plan, analysis));
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        let model = build_model(state.results_reads()).expect("the re-run's trace");
        let kept = settings(state.results_reads(), model.sweep());
        assert_eq!(kept, chosen);

        // 30 kHz lies between the 10 kHz and 100 kHz samples.
        let traces = model.traces();
        let spot = rspice_results::phase_noise::spot_level_at(
            traces[0].offset_hz,
            traces[0].level_dbc_per_hz,
            3.0e4,
        )
        .expect("a spot inside the sweep");
        assert!((spot - (-60.0 - 20.0 * 30.0_f64.log10())).abs() < 1e-9);
        // ∫ L df over 10 kHz…1 MHz of L = 10⁻⁶ (1 kHz / f)²
        // is 10⁻⁶ · 10⁶ · (10⁻⁴ − 10⁻⁶) = 9.9·10⁻⁵.
        let figures = view::band_figures(&traces[0], &kept).expect("the band's figures");
        let power = 9.9e-5_f64;
        assert!((figures.integrated.single_sideband_power / power - 1.0).abs() < 1e-9);
        assert!(
            (figures.jitter_s / ((2.0 * power).sqrt() / (std::f64::consts::TAU * 1.0e9)) - 1.0)
                .abs()
                < 1e-9
        );

        keep_settings(
            &mut state.results_parts(),
            view::PhaseNoiseSettings {
                band: None,
                ..chosen.clone()
            },
        );
        state.quick_frame();
        let view = state.working_view().expect("the run's view").id();
        view_edits::step(&mut state.results_parts(), view, view_edits::Step::Undo)
            .expect("the change of band is undone");
        state.quick_frame();
        assert_eq!(settings(state.results_reads(), model.sweep()), chosen);
    }
}
