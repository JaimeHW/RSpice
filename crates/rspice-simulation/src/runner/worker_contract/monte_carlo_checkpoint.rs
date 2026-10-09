//! Checkpoint input bytes never expand into worker request JSON.
use super::*;
use crate::monte_carlo_checkpoint::MonteCarloCheckpointInput;
use crate::transient_checkpoint::TransientCheckpointInput;

enum Input<'a> {
    MonteCarlo(&'a mut MonteCarloCheckpointInput),
    Transient(&'a mut TransientCheckpointInput),
}
fn input_mut(request: &mut WorkerRequest) -> Result<Option<Input<'_>>, String> {
    let WorkerSimulationRequest::Spec { options, .. } = &mut request.request else {
        return Ok(None);
    };
    if options.mc_checkpoint.is_some() && options.tran_checkpoint.is_some() {
        return Err("A worker request cannot carry two checkpoint kinds".into());
    }
    Ok(if let Some(request) = &mut options.mc_checkpoint {
        request.resume.as_mut().map(Input::MonteCarlo)
    } else {
        options
            .tran_checkpoint
            .as_mut()
            .and_then(|request| request.resume.as_mut())
            .map(Input::Transient)
    })
}
pub(crate) fn take_worker_request_checkpoint(
    request: &mut WorkerRequest,
) -> Result<Vec<Vec<u8>>, String> {
    let limits = request.execution_limits;
    match input_mut(request)? {
        Some(Input::MonteCarlo(input)) => input.take_bytes(),
        Some(Input::Transient(input)) => input.take_bytes(limits),
        None => return Ok(vec![]),
    }
    .map(|bytes| vec![bytes])
    .map_err(|error| error.to_string())
}
#[cfg(any(feature = "browser-worker", test))]
pub(super) fn restore_worker_request_checkpoint(
    request: &mut WorkerRequest,
    mut buffers: Vec<Vec<u8>>,
) -> Result<(), String> {
    let limits = request.execution_limits;
    let input = input_mut(request)?;
    if buffers.len() != usize::from(input.is_some()) {
        return Err("Worker checkpoint byte buffers disagree with the declared input".into());
    }
    match input {
        Some(Input::MonteCarlo(input)) => {
            input.restore_bytes(buffers.pop().expect("one checkpoint buffer"))
        }
        Some(Input::Transient(input)) => {
            input.restore_bytes(buffers.pop().expect("one checkpoint buffer"), limits)
        }
        None => Ok(()),
    }
    .map_err(|error| error.to_string())
}

/// Check declared kind/count and the receiving run budget before copying buffers.
#[cfg(any(feature = "browser-worker", test, target_arch = "wasm32"))]
pub(crate) fn validate_checkpoint_request_lengths(
    request: &WorkerRequest,
    numeric_values: usize,
    lengths: &[usize],
) -> Result<(), String> {
    let expected = if let WorkerSimulationRequest::Spec { options, .. } = &request.request {
        if options.mc_checkpoint.is_some() && options.tran_checkpoint.is_some() {
            return Err("A worker request cannot carry two checkpoint kinds".into());
        }
        usize::from(
            options
                .mc_checkpoint
                .as_ref()
                .is_some_and(|p| p.resume.is_some())
                || options
                    .tran_checkpoint
                    .as_ref()
                    .is_some_and(|p| p.resume.is_some()),
        )
    } else {
        0
    };
    if lengths.len() != expected {
        return Err("Worker checkpoint buffers differ from the declared input".into());
    }
    // Other dependency-only requests keep their existing transport policy.
    // A resume payload shares the receiving byte budget with its numeric buffers.
    if expected == 0 {
        return Ok(());
    }
    let bytes = lengths
        .iter()
        .fold(numeric_values.saturating_mul(8), |total, n| {
            total.saturating_add(*n)
        });
    crate::transient_checkpoint::check_limit(
        rspice_core::ResourceKind::ExternalDataBytes,
        bytes,
        request.execution_limits.max_external_data_bytes,
    )
    .map_err(|error| error.to_string())
}
