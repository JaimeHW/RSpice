//! Building a runner configuration from a dialog.
//!
//! The bridge between what the analysis options sheet holds and the typed
//! run configuration each service runner takes. Kept apart from the dialogs
//! so a run configuration can be built without an egui context.

use super::*;

/// The large-signal basis a periodic small-signal request states, read from
/// the carrier family that request names.
///
/// The three numbers are the producer's, not the consumer's: the engine
/// replaces a `.PAC`-family request's authored fundamental with the carrier's
/// own before it solves anything. They travel on the request all the same,
/// because a shooting carrier's retained state is authenticated against them
/// (`PeriodicStateArtifact::validate_consumer_basis`) and because a standalone
/// run has no carrier to read.
///
/// A harmonic-balance carrier reads the HB form's primary tone, its harmonic
/// count and its relative tolerance. Reading the PSS form instead — which is
/// what a single builder did — asked an operator who never opened that form to
/// fill it in before a run that never touches it.
struct PeriodicCarrierBasis {
    fundamental_freq: f64,
    num_harmonics: usize,
    tolerance: f64,
}

impl PeriodicCarrierBasis {
    fn read(
        state: &AppState,
        carrier: crate::services::simulation_runner::PeriodicCarrier,
        consumer: &str,
    ) -> Result<Self, String> {
        use crate::services::simulation_runner::PeriodicCarrier;

        if matches!(carrier, PeriodicCarrier::Hb) {
            let mut hb_state = state.sim_setup.hb.clone();
            hb_state.ensure_initialized();
            let hb_cfg = hb_state
                .to_config()
                .map_err(|error| format!("invalid HB settings required for {consumer}: {error}"))?;
            return Ok(Self {
                fundamental_freq: hb_cfg.fundamental_freq,
                num_harmonics: hb_cfg.num_harmonics as usize,
                tolerance: hb_cfg.reltol,
            });
        }
        let mut pss_state = state.sim_setup.pss.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|error| format!("invalid PSS settings required for {consumer}: {error}"))?;
        Ok(Self {
            fundamental_freq: pss_cfg.fund_freq,
            num_harmonics: pss_cfg.num_harmonics,
            tolerance: pss_cfg.tolerance,
        })
    }
}

impl SimulationController {
    pub(super) fn pac_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::PacRunConfig, String> {
        use crate::services::simulation_runner::{PacFrequencySweep, PacRunConfig};

        let mut pac_state = state.sim_setup.pac.clone();
        pac_state.ensure_initialized();
        let pac_cfg = pac_state
            .to_config()
            .map_err(|e| format!("invalid PAC settings: {}", e))?;

        let basis = PeriodicCarrierBasis::read(state, pac_cfg.carrier, "PAC")?;

        let sweep = match pac_cfg.sweep_type {
            crate::simulation::dialog::pac::PacSweepType::Decade => PacFrequencySweep::Decade,
            crate::simulation::dialog::pac::PacSweepType::Octave => PacFrequencySweep::Octave,
            crate::simulation::dialog::pac::PacSweepType::Linear => PacFrequencySweep::Linear,
        };

        let output_ref =
            (!pac_cfg.output_ref.trim().is_empty()).then(|| pac_cfg.output_ref.clone());
        let (reltol, abstol) =
            Self::authored_or_plan_tolerances(state, pac_cfg.reltol, pac_cfg.abstol);
        let (sideband_min, sideband_max) = pac_cfg.resolved_sidebands();

        Ok(PacRunConfig {
            pss_fundamental_freq: basis.fundamental_freq,
            pss_num_harmonics: basis.num_harmonics,
            pss_tolerance: basis.tolerance,
            start_freq: pac_cfg.start_freq,
            stop_freq: pac_cfg.stop_freq,
            points_per_unit: pac_cfg.num_points as usize,
            sweep,
            sideband_min,
            sideband_max,
            input_source: pac_cfg.input_source,
            output_node: pac_cfg.output_node,
            output_ref,
            pac_magnitude: pac_cfg.pac_magnitude,
            include_dc: pac_cfg.include_dc,
            reltol,
            abstol,
            carrier: pac_cfg.carrier,
        })
    }

    pub(super) fn pnoise_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::PnoiseRunConfig, String> {
        use crate::services::simulation_runner::{
            PnoiseFrequencySweep, PnoiseReference, PnoiseRunConfig,
        };

        let mut pnoise_state = state.sim_setup.pnoise.clone();
        pnoise_state.ensure_initialized();
        let pnoise_cfg = pnoise_state
            .to_config()
            .map_err(|e| format!("invalid PNOISE settings: {}", e))?;

        let basis = PeriodicCarrierBasis::read(state, pnoise_cfg.carrier, "PNOISE")?;

        let sweep = match pnoise_cfg.sweep_type {
            crate::simulation::dialog::pnoise::PnoiseSweepType::Decade => {
                PnoiseFrequencySweep::Decade
            }
            crate::simulation::dialog::pnoise::PnoiseSweepType::Octave => {
                PnoiseFrequencySweep::Octave
            }
            crate::simulation::dialog::pnoise::PnoiseSweepType::Linear => {
                PnoiseFrequencySweep::Linear
            }
        };

        let noise_ref = match pnoise_cfg.noise_ref {
            crate::simulation::dialog::pnoise::NoiseReferenceType::Output => {
                PnoiseReference::Output
            }
            crate::simulation::dialog::pnoise::NoiseReferenceType::Input => PnoiseReference::Input,
            crate::simulation::dialog::pnoise::NoiseReferenceType::Phase => PnoiseReference::Phase,
        };

        let output_ref =
            (!pnoise_cfg.output_ref.trim().is_empty()).then(|| pnoise_cfg.output_ref.clone());
        let (reltol, abstol) = Self::periodic_solver_tolerances(state);

        Ok(PnoiseRunConfig {
            sampling: pnoise_cfg.sampling,
            input_sideband: pnoise_cfg.input_sideband,
            output_sideband: pnoise_cfg.output_sideband,
            pss_fundamental_freq: basis.fundamental_freq,
            pss_num_harmonics: basis.num_harmonics,
            pss_tolerance: basis.tolerance,
            start_freq: pnoise_cfg.start_freq,
            stop_freq: pnoise_cfg.stop_freq,
            points_per_unit: pnoise_cfg.num_points as usize,
            sweep,
            max_sideband: pnoise_cfg.max_sideband,
            output_node: pnoise_cfg.output_node,
            output_ref,
            input_source: pnoise_cfg.input_source,
            noise_ref,
            integrated_noise: pnoise_cfg.integrated_noise,
            noise_summary: pnoise_cfg.noise_summary,
            reltol,
            abstol,
            carrier: pnoise_cfg.carrier,
        })
    }

    pub(super) fn pxf_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::PxfRunConfig, String> {
        use crate::services::simulation_runner::{PxfFrequencySweep, PxfRunConfig};

        let mut pxf_state = state.sim_setup.pxf.clone();
        pxf_state.ensure_initialized();
        let pxf_cfg = pxf_state
            .to_config()
            .map_err(|e| format!("invalid PXF settings: {}", e))?;

        let basis = PeriodicCarrierBasis::read(state, pxf_cfg.carrier, "PXF")?;

        let sweep = match pxf_cfg.sweep_type {
            crate::simulation::dialog::pxf::PxfSweepType::Decade => PxfFrequencySweep::Decade,
            crate::simulation::dialog::pxf::PxfSweepType::Octave => PxfFrequencySweep::Octave,
            crate::simulation::dialog::pxf::PxfSweepType::Linear => PxfFrequencySweep::Linear,
        };

        let output_ref =
            (!pxf_cfg.output_ref.trim().is_empty()).then(|| pxf_cfg.output_ref.clone());
        let (reltol, abstol) =
            Self::authored_or_plan_tolerances(state, pxf_cfg.reltol, pxf_cfg.abstol);

        Ok(PxfRunConfig {
            pss_fundamental_freq: basis.fundamental_freq,
            pss_num_harmonics: basis.num_harmonics,
            pss_tolerance: basis.tolerance,
            start_freq: pxf_cfg.start_freq,
            stop_freq: pxf_cfg.stop_freq,
            points_per_unit: pxf_cfg.num_points as usize,
            sweep,
            input_source: pxf_cfg.input_source,
            input_sideband: pxf_cfg.input_sideband,
            output_node: pxf_cfg.output_node,
            output_ref,
            output_sideband: pxf_cfg.output_sideband,
            max_sideband: pxf_cfg.max_sideband,
            reltol,
            abstol,
            carrier: pxf_cfg.carrier,
        })
    }

    pub(super) fn pstb_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::PstbRunConfig, String> {
        use crate::services::simulation_runner::PstbRunConfig;

        let mut pstb_state = state.sim_setup.pstb.clone();
        pstb_state.ensure_initialized();
        let pstb_cfg = pstb_state
            .to_config()
            .map_err(|e| format!("invalid PSTB settings: {}", e))?;

        let mut pss_state = state.sim_setup.pss.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings required for PSTB: {}", e))?;

        Ok(PstbRunConfig {
            pss_fundamental_freq: pss_cfg.fund_freq,
            pss_num_harmonics: pss_cfg.num_harmonics,
            pss_tolerance: pss_cfg.tolerance,
            probe_instance: pstb_cfg.probe,
            max_harmonics: pstb_cfg.max_harmonics as usize,
            num_multipliers: pstb_cfg.num_multipliers as usize,
            stability_threshold: pstb_cfg.stability_threshold,
            detect_subharmonics: pstb_cfg.detect_subharmonics,
            eigenvalue_tolerance: pstb_cfg.eigenvalue_tolerance,
        })
    }

    pub(super) fn temp_run_config_from_dialog(
        state: &AppState,
        temp_cfg: &crate::simulation::dialog::temp::TempConfig,
    ) -> Result<crate::services::simulation_runner::TempRunConfig, String> {
        use crate::services::simulation_runner::{
            CornerBaseMode, CornerFrequencySweep, TempRunConfig,
        };
        use crate::simulation::dialog::temp::TempBaseAnalysis;

        let temperatures_c = if !temp_cfg.specific_temps.is_empty() {
            temp_cfg.specific_temps.clone()
        } else {
            Self::expand_temperature_points(
                temp_cfg.temp_start,
                temp_cfg.temp_stop,
                temp_cfg.temp_step,
            )?
        };

        let base_mode = match temp_cfg.base_analysis {
            TempBaseAnalysis::Op => CornerBaseMode::Op,
            TempBaseAnalysis::Dc => {
                CornerBaseMode::from_dc_config(&state.sim_setup.dc.to_config()?)
            }
            TempBaseAnalysis::Transient => Self::transient_study_base_mode(state)?,
            TempBaseAnalysis::Ac => {
                let sweep = match Self::map_frequency_sweep(state.sim_setup.ac.sweep) {
                    FrequencySweep::Decade => CornerFrequencySweep::Decade,
                    FrequencySweep::Octave => CornerFrequencySweep::Octave,
                    FrequencySweep::Linear => CornerFrequencySweep::Linear,
                };
                CornerBaseMode::Ac {
                    start_freq: parse_spice_value_checked(&state.sim_setup.ac.fstart)
                        .map_err(|e| format!("invalid temperature AC start frequency: {}", e))?,
                    stop_freq: parse_spice_value_checked(&state.sim_setup.ac.fstop)
                        .map_err(|e| format!("invalid temperature AC stop frequency: {}", e))?,
                    points_per_unit: Self::parse_positive_points(
                        &state.sim_setup.ac.points,
                        "ac_points",
                    )
                    .map_err(|e| format!("invalid temperature AC points: {}", e))?,
                    sweep,
                }
            }
        };

        Ok(TempRunConfig {
            temperatures_c,
            base_mode,
        })
    }

    pub(super) fn corner_run_config_from_dialog(
        state: &AppState,
        corner_cfg: &crate::simulation::dialog::corner::CornerConfig,
        sealed_model_sources: &crate::state::model_library::SealedModelExecutionSources,
    ) -> Result<crate::services::simulation_runner::CornerRunConfig, String> {
        use crate::services::simulation_runner::{
            CornerBaseMode, CornerFrequencySweep, CornerPoint, CornerRunConfig,
        };
        use crate::simulation::dialog::corner::CornerBaseAnalysis;

        let process_corners = corner_cfg.process_corners.clone();
        let points: Vec<CornerPoint> = corner_cfg
            .points
            .iter()
            .map(|point| CornerPoint {
                process: point.process,
                voltage: point.voltage,
                temperature_c: point.temperature_celsius,
            })
            .collect();
        let model_bindings = sealed_model_sources.corner_model_bindings(&process_corners)?;

        let nominal_voltage = match corner_cfg.voltages.len() {
            0 => None,
            1 => Some(corner_cfg.voltages[0]),
            n => Some(corner_cfg.voltages[n / 2]),
        };

        let base_mode = match corner_cfg.base_analysis {
            CornerBaseAnalysis::Op => CornerBaseMode::Op,
            CornerBaseAnalysis::Dc => {
                CornerBaseMode::from_dc_config(&state.sim_setup.dc.to_config()?)
            }
            CornerBaseAnalysis::Transient => Self::transient_study_base_mode(state)?,
            CornerBaseAnalysis::Ac => {
                let sweep = match Self::map_frequency_sweep(state.sim_setup.ac.sweep) {
                    FrequencySweep::Decade => CornerFrequencySweep::Decade,
                    FrequencySweep::Octave => CornerFrequencySweep::Octave,
                    FrequencySweep::Linear => CornerFrequencySweep::Linear,
                };
                CornerBaseMode::Ac {
                    start_freq: parse_spice_value_checked(&state.sim_setup.ac.fstart)
                        .map_err(|e| format!("invalid corner AC start frequency: {}", e))?,
                    stop_freq: parse_spice_value_checked(&state.sim_setup.ac.fstop)
                        .map_err(|e| format!("invalid corner AC stop frequency: {}", e))?,
                    points_per_unit: Self::parse_positive_points(
                        &state.sim_setup.ac.points,
                        "ac_points",
                    )
                    .map_err(|e| format!("invalid corner AC points: {}", e))?,
                    sweep,
                }
            }
        };

        Ok(CornerRunConfig {
            process_corners,
            voltages: corner_cfg.voltages.clone(),
            supply_source_names: corner_cfg.supply_source_names.clone(),
            temperatures_c: corner_cfg.temperatures.clone(),
            full_matrix: corner_cfg.full_matrix,
            nominal_voltage,
            base_mode,
            model_bindings,
            points,
        })
    }

    fn transient_study_base_mode(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::CornerBaseMode, String> {
        use crate::services::simulation_runner::CornerBaseMode;
        let draft = &state.sim_setup.tran;
        let config = crate::simulation::config::TransientAnalysisConfig {
            stop_time: parse_spice_value_checked(&draft.stop)
                .map_err(|e| format!("invalid study transient stop time: {e}"))?,
            step_time: parse_spice_value_checked(&draft.step)
                .map_err(|e| format!("invalid study transient step time: {e}"))?,
            start_time: parse_spice_value_checked(&draft.start)
                .map_err(|e| format!("invalid study transient start time: {e}"))?,
            max_timestep: Self::parse_optional_spice_value(&draft.max_step)
                .map_err(|e| format!("invalid study transient max step: {e}"))?,
            uic: draft.uic,
        };
        config.validate().map_err(|errors| errors.join("; "))?;
        // Keep the historical request identity when no window/IC control is
        // authored, while retaining every configured field when one is.
        Ok(
            if config.start_time == 0.0 && config.max_timestep.is_none() && !config.uic {
                CornerBaseMode::Transient {
                    stop_time: config.stop_time,
                    step_time: config.step_time,
                }
            } else {
                CornerBaseMode::TransientWindow {
                    stop_time: config.stop_time,
                    step_time: config.step_time,
                    start_time: config.start_time,
                    max_timestep: config.max_timestep,
                    uic: config.uic,
                }
            },
        )
    }

    pub(super) fn periodic_solver_tolerances(state: &AppState) -> (f64, f64) {
        let opts = &state.sim_setup.options;
        (opts.reltol, opts.abstol)
    }

    /// The tolerances a periodic small-signal run solves at.
    ///
    /// The plan's Solver options channel states the deck-wide policy and every
    /// periodic dependent used to be handed it unconditionally, so an analysis
    /// that needed a tighter periodic solve than the rest of the deck had no
    /// way to ask. An authored field is this analysis's own and wins; an empty
    /// one is the policy, which is what every run written before the two
    /// fields existed asked for — so an untouched form reaches the engine with
    /// exactly the numbers it always did.
    ///
    /// Only `.PAC` and `.PXF` take this. `.PNOISE` and `.PSTB` have no
    /// `RELTOL=`/`ABSTOL=` arm on their cards
    /// (`rspice-core/src/netlist/parser/periodic_cards.rs`), so a field for
    /// them would be a number no deck could carry and no round trip could
    /// preserve.
    pub(super) fn authored_or_plan_tolerances(
        state: &AppState,
        reltol: Option<f64>,
        abstol: Option<f64>,
    ) -> (f64, f64) {
        let (plan_reltol, plan_abstol) = Self::periodic_solver_tolerances(state);
        (reltol.unwrap_or(plan_reltol), abstol.unwrap_or(plan_abstol))
    }
}

#[cfg(test)]
mod pvt_base_tests {
    use super::*;
    use crate::services::simulation_runner::CornerBaseMode;
    use crate::simulation::runner::worker_contract::WorkerCornerBaseMode;

    #[test]
    fn pvt_base_transient_window_survives_configuration_and_worker_transport() {
        let mut state = AppState::default();
        state.sim_setup.tran.stop = "1m".into();
        state.sim_setup.tran.step = "10u".into();
        let temperature = crate::simulation::dialog::temp::TempConfig {
            base_analysis: crate::simulation::dialog::temp::TempBaseAnalysis::Transient,
            ..Default::default()
        };
        let corner = crate::simulation::dialog::corner::CornerConfig::default();
        let sealed = state
            .model_library_manager
            .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
            .unwrap();
        assert!(matches!(
            SimulationController::temp_run_config_from_dialog(&state, &temperature)
                .unwrap()
                .base_mode,
            CornerBaseMode::Transient { .. }
        ));

        state.sim_setup.tran.start = "200u".into();
        state.sim_setup.tran.max_step = "2u".into();
        state.sim_setup.tran.uic = true;
        for mode in [
            SimulationController::temp_run_config_from_dialog(&state, &temperature)
                .unwrap()
                .base_mode,
            SimulationController::corner_run_config_from_dialog(&state, &corner, &sealed)
                .unwrap()
                .base_mode,
        ] {
            let packet = WorkerCornerBaseMode::from(&mode);
            let json = serde_json::to_string(&packet).unwrap();
            let restored: WorkerCornerBaseMode = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, packet);
            let CornerBaseMode::TransientWindow {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } = CornerBaseMode::from(restored)
            else {
                panic!("all transient settings must be retained")
            };
            for (actual, expected) in [(stop_time, 1e-3), (step_time, 1e-5), (start_time, 2e-4)] {
                assert!((actual / expected - 1.0).abs() < 1e-14);
            }
            assert_eq!(max_timestep, Some(2e-6));
            assert!(uic);
        }

        // Invalid inherited fields must fail on the study form as they do on Transient.
        state.sim_setup.tran.max_step = "-2u".into();
        assert!(SimulationController::temp_run_config_from_dialog(&state, &temperature).is_err());
        assert!(
            SimulationController::corner_run_config_from_dialog(&state, &corner, &sealed).is_err()
        );
    }
}
