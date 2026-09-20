//! The same calibrated request is validated at preparation and dispatch.
use super::AnalysisSpec;
use rspice_core::analysis::reliability::ReliabilityRunRequest;

impl AnalysisSpec {
    pub(crate) fn reliability_request(&self) -> Result<ReliabilityRunRequest, String> {
        let Self::Reliability {
            study,
            target_years,
            enable_hci,
            enable_nbti,
            enable_em,
            min_stress_voltage,
        } = self
        else {
            return Err("Expected a Reliability specification".into());
        };
        let study = study.clone().ok_or_else(||
            "Reliability execution is unavailable without an imported calibrated model pack and explicit mission bindings; request rejected before dispatch".to_owned())?;
        let request = ReliabilityRunRequest {
            study,
            target_years: target_years.clone(),
            enable_hci: *enable_hci,
            enable_nbti: *enable_nbti,
            enable_em: *enable_em,
            min_stress_voltage: *min_stress_voltage,
        };
        request.validate()?;
        Ok(request)
    }

    /// Reliability is a typed Studio study, not a standard SPICE directive.
    /// Its full request is retained in the prepared task and authenticated by
    /// the task digest. This comment identifies that request in the circuit
    /// deck without inventing a base-analysis card or implying that a bare
    /// exported deck can run a study whose calibration it does not contain.
    pub(crate) fn reliability_plan_statement(&self) -> Result<String, String> {
        let request = self.reliability_request()?;
        let encoded = serde_json::to_vec(&request).map_err(|error| error.to_string())?;
        let digest = crate::simulation::execution::content_digest(
            "rspice.reliability-plan-statement/v1",
            &encoded,
        );
        Ok(format!(
            "* Reliability mission: {} phases, {} devices, years={:?}; run from saved Studio plan (request {})",
            request.study.mission.len(),
            request.study.bindings.len(),
            request.target_years,
            digest,
        ))
    }
}
