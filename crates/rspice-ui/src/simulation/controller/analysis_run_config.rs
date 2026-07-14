use super::*;

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

        let mut pss_state = state.sim_setup.pss.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings required for PAC: {}", e))?;

        let sweep = match pac_cfg.sweep_type {
            crate::simulation::dialog::pac::PacSweepType::Decade => PacFrequencySweep::Decade,
            crate::simulation::dialog::pac::PacSweepType::Octave => PacFrequencySweep::Octave,
            crate::simulation::dialog::pac::PacSweepType::Linear => PacFrequencySweep::Linear,
        };

        let output_ref =
            (!pac_cfg.output_ref.trim().is_empty()).then(|| pac_cfg.output_ref.clone());
        let (reltol, abstol) = Self::periodic_solver_tolerances(state);

        Ok(PacRunConfig {
            pss_fundamental_freq: pss_cfg.fund_freq,
            pss_num_harmonics: pss_cfg.num_harmonics as usize,
            pss_tolerance: pss_cfg.stab_tol,
            start_freq: pac_cfg.start_freq,
            stop_freq: pac_cfg.stop_freq,
            points_per_unit: pac_cfg.num_points as usize,
            sweep,
            max_sideband: pac_cfg.max_sideband,
            input_source: pac_cfg.input_source,
            output_node: pac_cfg.output_node,
            output_ref,
            pac_magnitude: pac_cfg.pac_magnitude,
            include_dc: pac_cfg.include_dc,
            reltol,
            abstol,
        })
    }

    pub(super) fn tf_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::TfRunConfig, String> {
        use crate::services::simulation_runner::{TfFrequencySweep, TfRunConfig};

        let mut xf_state = state.sim_setup.xf.clone();
        xf_state.ensure_initialized();
        let xf_cfg = xf_state
            .to_config()
            .map_err(|e| format!("invalid transfer-function settings: {}", e))?;

        let sweep = match xf_cfg.sweep_type {
            crate::simulation::dialog::xf::XfSweepType::Decade => TfFrequencySweep::Decade,
            crate::simulation::dialog::xf::XfSweepType::Octave => TfFrequencySweep::Octave,
            crate::simulation::dialog::xf::XfSweepType::Linear => TfFrequencySweep::Linear,
        };

        let output_ref = (!xf_cfg.output_ref.trim().is_empty()).then(|| xf_cfg.output_ref.clone());

        Ok(TfRunConfig {
            start_freq: xf_cfg.start_freq,
            stop_freq: xf_cfg.stop_freq,
            points_per_unit: xf_cfg.num_points as usize,
            sweep,
            input_source: xf_cfg.input_source,
            output_node: xf_cfg.output_node,
            output_ref,
            group_delay: xf_cfg.group_delay,
            input_impedance: xf_cfg.input_impedance,
            output_impedance: xf_cfg.output_impedance,
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

        let mut pss_state = state.sim_setup.pss.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings required for PNOISE: {}", e))?;

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
            pss_fundamental_freq: pss_cfg.fund_freq,
            pss_num_harmonics: pss_cfg.num_harmonics as usize,
            pss_tolerance: pss_cfg.stab_tol,
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

        let mut pss_state = state.sim_setup.pss.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings required for PXF: {}", e))?;

        let sweep = match pxf_cfg.sweep_type {
            crate::simulation::dialog::pxf::PxfSweepType::Decade => PxfFrequencySweep::Decade,
            crate::simulation::dialog::pxf::PxfSweepType::Octave => PxfFrequencySweep::Octave,
            crate::simulation::dialog::pxf::PxfSweepType::Linear => PxfFrequencySweep::Linear,
        };

        let output_ref =
            (!pxf_cfg.output_ref.trim().is_empty()).then(|| pxf_cfg.output_ref.clone());
        let (reltol, abstol) = Self::periodic_solver_tolerances(state);

        Ok(PxfRunConfig {
            pss_fundamental_freq: pss_cfg.fund_freq,
            pss_num_harmonics: pss_cfg.num_harmonics as usize,
            pss_tolerance: pss_cfg.stab_tol,
            start_freq: pxf_cfg.start_freq,
            stop_freq: pxf_cfg.stop_freq,
            points_per_unit: pxf_cfg.num_points as usize,
            sweep,
            input_source: pxf_cfg.input_source,
            input_sideband: 1,
            output_node: pxf_cfg.output_node,
            output_ref,
            output_sideband: pxf_cfg.output_sideband,
            max_sideband: pxf_cfg.max_sideband,
            reltol,
            abstol,
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
            pss_num_harmonics: pss_cfg.num_harmonics as usize,
            pss_tolerance: pss_cfg.stab_tol,
            probe_instance: pstb_cfg.probe,
            max_harmonics: pstb_cfg.max_harmonics as usize,
            num_multipliers: pstb_cfg.num_multipliers as usize,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
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
                let source_name = state.sim_setup.dc.source.trim();
                if source_name.is_empty() {
                    return Err(
                        "temperature sweep DC base analysis requires a non-empty sweep source"
                            .to_string(),
                    );
                }
                CornerBaseMode::DcSweep {
                    source_name: source_name.to_string(),
                    start: parse_spice_value_checked(&state.sim_setup.dc.start)
                        .map_err(|e| format!("invalid temperature DC start value: {}", e))?,
                    stop: parse_spice_value_checked(&state.sim_setup.dc.stop)
                        .map_err(|e| format!("invalid temperature DC stop value: {}", e))?,
                    step: parse_spice_value_checked(&state.sim_setup.dc.step)
                        .map_err(|e| format!("invalid temperature DC step value: {}", e))?,
                }
            }
            TempBaseAnalysis::Transient => CornerBaseMode::Transient {
                stop_time: parse_spice_value_checked(&state.sim_setup.tran.stop)
                    .map_err(|e| format!("invalid temperature transient stop time: {}", e))?,
                step_time: parse_spice_value_checked(&state.sim_setup.tran.step)
                    .map_err(|e| format!("invalid temperature transient step time: {}", e))?,
            },
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
            CornerBaseMode, CornerFrequencySweep, CornerProcess, CornerRunConfig,
        };
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, ProcessCorner};

        let process_corners: Vec<CornerProcess> = corner_cfg
            .process_corners
            .iter()
            .map(|corner| match corner {
                ProcessCorner::TT => CornerProcess::TT,
                ProcessCorner::SS => CornerProcess::SS,
                ProcessCorner::FF => CornerProcess::FF,
                ProcessCorner::SF => CornerProcess::SF,
                ProcessCorner::FS => CornerProcess::FS,
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
                let source_name = state.sim_setup.dc.source.trim();
                if source_name.is_empty() {
                    return Err(
                        "corner DC base analysis requires a non-empty sweep source".to_string()
                    );
                }
                CornerBaseMode::DcSweep {
                    source_name: source_name.to_string(),
                    start: parse_spice_value_checked(&state.sim_setup.dc.start)
                        .map_err(|e| format!("invalid corner DC start value: {}", e))?,
                    stop: parse_spice_value_checked(&state.sim_setup.dc.stop)
                        .map_err(|e| format!("invalid corner DC stop value: {}", e))?,
                    step: parse_spice_value_checked(&state.sim_setup.dc.step)
                        .map_err(|e| format!("invalid corner DC step value: {}", e))?,
                }
            }
            CornerBaseAnalysis::Transient => CornerBaseMode::Transient {
                stop_time: parse_spice_value_checked(&state.sim_setup.tran.stop)
                    .map_err(|e| format!("invalid corner transient stop time: {}", e))?,
                step_time: parse_spice_value_checked(&state.sim_setup.tran.step)
                    .map_err(|e| format!("invalid corner transient step time: {}", e))?,
            },
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
            temperatures_c: corner_cfg.temperatures.clone(),
            full_matrix: corner_cfg.full_matrix,
            nominal_voltage,
            base_mode,
            model_bindings,
        })
    }

    pub(super) fn periodic_solver_tolerances(state: &AppState) -> (f64, f64) {
        let opts = &state.sim_setup.options;
        (opts.reltol, opts.abstol)
    }
}
