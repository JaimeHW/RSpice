//! Whole worker response packets validate before encoding and after reconstruction.
use super::*;

impl WorkerResponseTransport {
    #[cfg(test)]
    pub(in crate::runner::worker_contract) fn from_response(
        response: WorkerResponse,
    ) -> Result<Self, String> {
        Self::from_response_with_backends(response, None)
    }

    /// The response and which code ran it.
    #[cfg(any(feature = "browser-worker", test))]
    pub(in crate::runner::worker_contract) fn from_response_with_backends(
        response: WorkerResponse,
        backends: Option<rspice_core::backends::ExecutionBackends>,
    ) -> Result<Self, String> {
        validate_worker_response_before_transport(&response)?;
        let mut buffers = Vec::new();
        let response = WorkerResponseTransportMetadata {
            id: response.id,
            outcome: WorkerOutcomeTransport::from_outcome(response.outcome, &mut buffers)?,
            backends,
        };
        Ok(Self {
            protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
            response,
            buffers,
        })
    }

    #[cfg(test)]
    pub(in crate::runner::worker_contract) fn into_response(
        self,
    ) -> Result<WorkerResponse, String> {
        self.into_response_and_backends()
            .map(|(response, _)| response)
    }

    /// The response and which code ran it.
    pub(in crate::runner::worker_contract) fn into_response_and_backends(
        self,
    ) -> Result<
        (
            WorkerResponse,
            Option<rspice_core::backends::ExecutionBackends>,
        ),
        String,
    > {
        if self.protocol != WORKER_RESPONSE_TRANSPORT_PROTOCOL {
            return Err(format!(
                "unsupported worker response transport protocol {}",
                self.protocol
            ));
        }
        let backends = self.response.backends;
        let response = WorkerResponse {
            id: self.response.id,
            outcome: self.response.outcome.into_outcome(&self.buffers)?,
        };
        validate_worker_response_before_transport(&response)?;
        Ok((response, backends))
    }
}
