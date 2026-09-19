//! Validation for the analyses that derive their answer from a retained
//! transient rather than from a solve of their own.
//!
//! What a post-processing kind refuses on is the window it reads and the
//! accessor it reads through, both of which are questions about the run
//! it is derived from.

use crate::simulation::multi_run::AnalysisSpec;

/// Validate one post-processing specification.
pub(super) fn validate(spec: &AnalysisSpec) -> Result<(), String> {
    match spec {
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            output_node,
            output_ref,
            start_time,
            stop_time,
            ..
        } => {
            if !fundamental_freq.is_finite() || *fundamental_freq <= 0.0 {
                return Err("Fourier fundamental_freq must be finite and > 0".to_string());
            }
            if *num_harmonics == 0 {
                return Err("Fourier num_harmonics must be > 0".to_string());
            }
            if output_node.trim().is_empty() {
                return Err("Fourier output_node is required".to_string());
            }
            crate::services::simulation_runner::validate_fourier_output_accessor(
                output_node,
                Some(output_ref),
            )?;
            if !start_time.is_finite() || *start_time < 0.0 {
                return Err("Fourier start_time must be finite and >= 0".to_string());
            }
            if !stop_time.is_finite() || *stop_time <= *start_time {
                return Err(
                    "Fourier stop_time must be finite and greater than start_time".to_string(),
                );
            }
            Ok(())
        }
        // The card is the request, and its refusals are the engine's own.
        AnalysisSpec::Fft { request } => request.validate(),
        other => Err(super::misrouted_specification("post-processing", other)),
    }
}
