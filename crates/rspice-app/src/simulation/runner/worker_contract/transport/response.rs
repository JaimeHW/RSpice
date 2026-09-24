//! Whole worker response packets validate before encoding and after reconstruction.
use super::*;

impl WorkerResponseTransport {
    pub(in crate::simulation::runner::worker_contract) fn from_response(
        response: WorkerResponse,
    ) -> Result<Self, String> {
        validate_worker_response_before_transport(&response)?;
        let mut buffers = Vec::new();
        let response = WorkerResponseTransportMetadata {
            id: response.id,
            outcome: WorkerOutcomeTransport::from_outcome(response.outcome, &mut buffers)?,
        };
        validate_worker_transfer_buffers(&buffers)?;
        Ok(Self {
            protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
            response,
            buffers,
        })
    }

    pub(in crate::simulation::runner::worker_contract) fn into_response(
        self,
    ) -> Result<WorkerResponse, String> {
        if self.protocol != WORKER_RESPONSE_TRANSPORT_PROTOCOL {
            return Err(format!(
                "unsupported worker response transport protocol {}",
                self.protocol
            ));
        }
        validate_worker_transfer_buffers(&self.buffers)?;

        let response = WorkerResponse {
            id: self.response.id,
            outcome: self.response.outcome.into_outcome(&self.buffers)?,
        };
        validate_worker_response_before_transport(&response)?;
        Ok(response)
    }
}
