//! Publishing the spectrum a bound transient already computed.
//!
//! This is the whole of a recorded FFT's execution: take the trajectory
//! artifact the transient produced, select the spectrum whose request is this
//! analysis's own card, and hand it back. No deck is parsed, no solve is
//! started, and the netlist never reaches this module — which is what makes
//! "one solve" checkable rather than asserted.

use rspice_core::ResourceLimits;
use rspice_core::abort_signal::AbortSignal;

use crate::error::SimulationError;
use crate::execution_artifact::ResolvedExecutionDependencies;
use crate::results::SimulationResult;
use rspice_simulation_contract::config::FftRequest;

pub(super) fn run(
    request: &FftRequest,
    dependencies: &ResolvedExecutionDependencies,
    context: crate::engine_services::ServiceContext<'_>,
) -> Result<SimulationResult, SimulationError> {
    let abort = context.abort;
    super::ensure_not_aborted(abort)?;
    let trajectory = dependencies.transient_trajectory().map_err(|error| {
        SimulationError::InvalidConfig(format!(
            "recorded FFT dependency artifact is unavailable: {error}"
        ))
    })?;
    run_from_trajectory_with_resource_limits(request, trajectory, context.limits, abort)
}

pub(super) fn run_from_trajectory(
    request: &FftRequest,
    trajectory: &crate::execution_artifact::TransientTrajectoryArtifact,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    run_from_trajectory_with_resource_limits(request, trajectory, ResourceLimits::default(), abort)
}

fn run_from_trajectory_with_resource_limits(
    request: &FftRequest,
    trajectory: &crate::execution_artifact::TransientTrajectoryArtifact,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    let key = request
        .engine_key()
        .map_err(SimulationError::InvalidConfig)?;
    let spectrum = trajectory.spectrum(&key).ok_or_else(|| {
        SimulationError::InvalidConfig(format!(
            "the bound transient produced no spectrum for {key}"
        ))
    })?;
    // The producer already performed the transform. Admit the selected result
    // under the consumer's policy without allocating or recomputing its bins.
    let points = spectrum.frequency.len();
    if points > limits.max_analysis_points {
        return Err(SimulationError::ResourceLimit {
            resource: "analysis_points".into(),
            requested: points,
            limit: limits.max_analysis_points,
        });
    }
    let values = spectrum.numeric_value_count().saturating_add(
        trajectory
            .convergence()
            .map_or(0, |evidence| evidence.transfer_value_count()),
    );
    if values > limits.max_result_values {
        return Err(SimulationError::ResourceLimit {
            resource: "result_values".into(),
            requested: values,
            limit: limits.max_result_values,
        });
    }
    Ok(SimulationResult::Fft {
        spectrum: std::sync::Arc::clone(spectrum),
        convergence: trajectory.convergence().cloned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine_bridge::EngineBridge;
    use crate::execution_artifact::TransientTrajectoryArtifact;
    use rspice_core::NoAbort;
    use rspice_simulation_contract::analysis_spec::AnalysisSpec;

    #[test]
    fn retained_fft_obeys_consumer_limits_without_repeating_the_solve() {
        let request = FftRequest {
            output: "V(out)".into(),
            points: 16,
            start: Some(0.0),
            stop: Some(0.001),
            ..Default::default()
        };
        let deck = format!(
            "Recorded FFT policy\nV1 in 0 SIN(0 1 1k)\nR1 in out 1k\nR2 out 0 1k\n{}\n.end\n",
            request.engine_key().unwrap(),
        );
        let result = super::super::run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::Transient {
                stop_time: 0.001,
                step_time: 1e-5,
                start_time: 0.0,
                max_timestep: Some(1e-5),
                uic: false,
            },
            Default::default(),
            &deck,
            None,
            &ResolvedExecutionDependencies::default(),
            &NoAbort,
        )
        .unwrap();
        let trajectory = TransientTrajectoryArtifact::from_result(&result, &[], true)
            .unwrap()
            .unwrap();
        let recorded = trajectory.spectrum(&request.engine_key().unwrap()).unwrap();
        assert_eq!(recorded.frequency.len(), 9);
        let mut limits = ResourceLimits::default();
        // This policy cannot solve the producer circuit, but the consumer only
        // publishes the existing nine bins and their three numeric columns.
        limits.max_matrix_unknowns = 1;
        limits.max_analysis_points = 9;
        limits.max_result_values = 27;
        let run = |limits| {
            run_from_trajectory_with_resource_limits(&request, &trajectory, limits, &NoAbort)
        };
        let SimulationResult::Fft { spectrum, .. } = run(limits).unwrap() else {
            panic!("FFT result");
        };
        assert!(std::sync::Arc::ptr_eq(recorded, &spectrum));
        let mut fewer_bins = limits;
        fewer_bins.max_analysis_points = 8;
        assert!(
            matches!(run(fewer_bins), Err(SimulationError::ResourceLimit {
            resource, requested: 9, limit: 8,
        }) if resource == "analysis_points")
        );
        let mut fewer_values = limits;
        fewer_values.max_result_values = 26;
        assert!(
            matches!(run(fewer_values), Err(SimulationError::ResourceLimit {
            resource, requested: 27, limit: 26,
        }) if resource == "result_values")
        );
        assert!(matches!(
            run_from_trajectory_with_resource_limits(
                &request,
                &trajectory,
                limits,
                &rspice_core::abort_signal::ImmediateAbort,
            ),
            Err(SimulationError::Aborted)
        ));
    }
}
