//! The worker's transport as bytes, for a host that drives the browser
//! worker from outside a browser.
//!
//! The browser application posts each request to its module worker as a
//! structured-clone message (`wasm_worker::worker_message`) and reads the
//! answer back (`worker_response_from_value`). The mixed corpus runner drives
//! the same worker image and script under Node, so it needs both encodings
//! as bytes: a request's metadata as JSON beside its numeric and byte
//! buffers, and an answer read back from its JSON metadata and numeric
//! buffers. Both go through the transport the browser uses, so a request is
//! checked and split, and an answer validated, exactly as there.

use super::{
    WorkerRequest, WorkerRequestTransport, WorkerResponseTransport,
    WorkerResponseTransportMetadata, validate_worker_response_id,
};
use std::sync::Arc;

use crate::error::SimulationError;
use crate::execution::ResolvedTaskDispatch;
use crate::results::SimulationResult;

/// One request as the browser host transfers it to the worker.
#[derive(Debug, Clone)]
pub struct WorkerWireRequest {
    /// The transport revision the worker checks first (`protocolVersion`).
    pub protocol: u8,
    /// The request's metadata (`request`), as JSON.
    pub metadata: String,
    /// The numeric buffers (`buffers`), each sent as a `Float64Array`.
    pub buffers: Vec<Vec<f64>>,
    /// The byte buffers (`byteBuffers`), each sent as a `Uint8Array`: the
    /// checkpoint a run resumes from.
    pub byte_buffers: Vec<Vec<u8>>,
    /// The request's resource limits, which bound every transient state the
    /// worker streams back.
    limits: rspice_core::ResourceLimits,
}

impl WorkerWireRequest {
    /// A transient state the worker streamed while running this request,
    /// checked as the browser host checks it before keeping it.
    pub fn transient_checkpoint(&self, bytes: Vec<u8>) -> Result<Arc<[u8]>, SimulationError> {
        crate::transient_checkpoint::validate_bytes_size(bytes.len(), self.limits)?;
        crate::transient_checkpoint::decode_bytes(&bytes, self.limits, &rspice_core::NoAbort)?;
        Ok(Arc::from(bytes))
    }
}

/// The request the browser host posts for `dispatch` under identity `id`.
pub fn worker_wire_request(
    id: u64,
    dispatch: ResolvedTaskDispatch,
) -> Result<WorkerWireRequest, SimulationError> {
    let (request, input) = crate::runner::prepared_request(dispatch, false);
    let limits = input.execution_limits;
    let request = WorkerRequest::from_runner_parts(id, &request, &input)?;
    let transport =
        WorkerRequestTransport::from_request(request).map_err(SimulationError::InvalidConfig)?;
    let metadata = serde_json::to_string(&transport.request).map_err(|error| {
        SimulationError::InvalidConfig(format!("worker request metadata: {error}"))
    })?;
    Ok(WorkerWireRequest {
        protocol: transport.protocol,
        metadata,
        buffers: transport.buffers,
        byte_buffers: transport.byte_buffers,
        limits,
    })
}

/// The result the browser host reads from the worker's answer to request
/// `id`: its transport revision, its metadata (`response`) as JSON and its
/// numeric buffers. A failure the worker answered with is this result's
/// error, as the host reports it.
pub fn worker_wire_result(
    id: u64,
    protocol: u8,
    metadata: &str,
    buffers: Vec<Vec<f64>>,
) -> Result<SimulationResult, SimulationError> {
    let response: WorkerResponseTransportMetadata =
        serde_json::from_str(metadata).map_err(|error| {
            SimulationError::InvalidConfig(format!("worker response metadata: {error}"))
        })?;
    let response = WorkerResponseTransport {
        protocol,
        response,
        buffers,
    }
    .into_response()
    .map_err(SimulationError::InvalidConfig)?;
    validate_worker_response_id(id, &response)?;
    response.into_result()
}
