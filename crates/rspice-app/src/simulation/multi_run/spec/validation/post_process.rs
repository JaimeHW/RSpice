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
            num_periods,
            output_node,
            output_ref,
            additional_outputs,
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
            if *num_periods == 0 {
                return Err("Fourier num_periods must be > 0".to_owned());
            }
            let duration = *num_periods as f64 / *fundamental_freq;
            if !duration.is_finite()
                || stop_time - start_time + 16.0 * f64::EPSILON * stop_time.abs().max(duration)
                    < duration
            {
                return Err("Fourier window must contain the requested complete periods".to_owned());
            }
            if output_node.trim().is_empty() {
                return Err("Fourier output_node is required".to_string());
            }
            rspice_simulation_contract::fourier_output::validate_fourier_output_accessor(
                output_node,
                Some(output_ref),
            )?;
            // Every further output reads the same trajectory through the same
            // accessor grammar, so it is refused on the same terms.
            for (index, output) in additional_outputs.iter().enumerate() {
                let (node, reference) =
                    rspice_simulation_contract::fourier_output::split_fourier_output(output)
                        .map_err(|error| format!("Fourier output {}: {error}", index + 2))?;
                rspice_simulation_contract::fourier_output::validate_fourier_output_accessor(
                    &node,
                    Some(&reference),
                )
                .map_err(|error| format!("Fourier output {}: {error}", index + 2))?;
            }
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
