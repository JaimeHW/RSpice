//! Validation for the analyses that walk time.
//!
//! The operating point is here rather than beside the AC and DC sweeps
//! because what it refuses is a *startup* policy — an initial guess and a
//! node initialization that have to be honourable together — which is a
//! question about the first time point, not about a sweep axis.

use crate::simulation::config::TransientAnalysisConfig;
use crate::simulation::dialog::OpConfig;
use crate::simulation::multi_run::AnalysisSpec;

/// Validate one time domain specification.
pub(super) fn validate(spec: &AnalysisSpec) -> Result<(), String> {
    match spec {
        AnalysisSpec::DcOp {
            temperature_mode,
            temperature_celsius,
            initial_guess,
            node_initialization,
            homotopy,
            annotation,
            device_detail,
            save_device_op,
            accuracy,
            selected_devices,
            previous_state,
            violation_devices,
            violation_source_content_digest,
            run_point,
        } => OpConfig {
            temperature_mode: *temperature_mode,
            temperature_celsius: *temperature_celsius,
            initial_guess: *initial_guess,
            node_initialization: *node_initialization,
            homotopy: *homotopy,
            annotation: *annotation,
            device_detail: *device_detail,
            save_device_op: *save_device_op,
            accuracy: *accuracy,
            selected_devices: selected_devices.clone(),
            previous_state: previous_state.clone(),
            violation_devices: violation_devices.clone(),
            violation_source_content_digest: *violation_source_content_digest,
            run_point: run_point.clone(),
        }
        .validate(),
        AnalysisSpec::Transient {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            uic,
        } => TransientAnalysisConfig {
            stop_time: *stop_time,
            step_time: *step_time,
            start_time: *start_time,
            max_timestep: *max_timestep,
            uic: *uic,
        }
        .validate()
        .map_err(|errors| errors.join("; ")),
        AnalysisSpec::TransientNoise {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            noise_fmax,
            noise_fmin,
            scale,
            ..
        } => {
            if !stop_time.is_finite() || *stop_time <= 0.0 {
                return Err("TNOISE stop_time must be finite and > 0".to_owned());
            }
            if !step_time.is_finite() || *step_time <= 0.0 || step_time > stop_time {
                return Err("TNOISE step_time must be finite, > 0, and <= stop_time".to_owned());
            }
            if !start_time.is_finite() || *start_time < 0.0 || start_time >= stop_time {
                return Err("TNOISE start_time must be finite, >= 0, and < stop_time".to_owned());
            }
            if !max_timestep.is_finite() || *max_timestep <= 0.0 || max_timestep > stop_time {
                return Err("TNOISE max_timestep must be finite, > 0, and <= stop_time".to_owned());
            }
            if !noise_fmax.is_finite() || *noise_fmax <= 0.0 || !scale.is_finite() || *scale < 0.0 {
                return Err(
                    "TNOISE requires positive finite fmax and nonnegative finite scale".to_owned(),
                );
            }
            // An absent floor is the engine's `1/tstop` derivation and refuses
            // nothing. An authored one has to be a frequency inside the band
            // the ceiling opens, in the same words the neighbours above use.
            if let Some(noise_fmin) = noise_fmin
                && (!noise_fmin.is_finite() || *noise_fmin <= 0.0 || noise_fmin >= noise_fmax)
            {
                return Err("TNOISE noise_fmin must be finite, > 0, and < noise_fmax".to_owned());
            }
            Ok(())
        }
        other => Err(super::misrouted_specification("time domain", other)),
    }
}
