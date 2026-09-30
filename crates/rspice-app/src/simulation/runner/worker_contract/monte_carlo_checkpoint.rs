//! Checkpoint input bytes never expand into worker request JSON.
use super::*;
use rspice_simulation::monte_carlo_checkpoint::MonteCarloCheckpointInput;

fn input_mut(request: &mut WorkerRequest) -> Option<&mut MonteCarloCheckpointInput> {
    match &mut request.request {
        WorkerSimulationRequest::Spec { options, .. } => {
            options.mc_checkpoint.as_mut()?.resume.as_mut()
        }
        _ => None,
    }
}

pub(crate) fn take_worker_request_checkpoint(
    request: &mut WorkerRequest,
) -> Result<Vec<Vec<u8>>, String> {
    input_mut(request)
        .map(|input| {
            input
                .take_bytes()
                .map(|bytes| vec![bytes])
                .map_err(|error| error.to_string())
        })
        .unwrap_or_else(|| Ok(vec![]))
}

pub(super) fn restore_worker_request_checkpoint(
    request: &mut WorkerRequest,
    mut buffers: Vec<Vec<u8>>,
) -> Result<(), String> {
    let input = input_mut(request);
    if buffers.len() != usize::from(input.is_some()) {
        return Err(
            "Worker Monte Carlo checkpoint byte buffers disagree with the declared input".into(),
        );
    }
    if let Some(input) = input {
        input
            .restore_bytes(buffers.pop().expect("one checkpoint buffer"))
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}
