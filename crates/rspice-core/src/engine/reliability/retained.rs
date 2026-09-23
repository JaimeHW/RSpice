//! Checked retained reliability evidence, shared by native and worker consumers.

use super::*;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use std::collections::{BTreeMap, BTreeSet};

impl ReliabilityRunResult {
    /// Streaming digest of the complete typed evidence, including calibrated
    /// model provenance and observations. No large JSON byte buffer is built.
    pub fn retained_identity_with_abort(
        &self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<String, SimulationError> {
        self.validate_retained_payload_with_abort(limits, abort)?;
        struct Sink<'a> {
            hash: blake3::Hasher,
            abort: &'a dyn AbortSignal,
        }
        impl std::io::Write for Sink<'_> {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if self.abort.is_aborted() {
                    return Err(std::io::Error::other("cancelled"));
                }
                self.hash.update(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut sink = Sink {
            hash: blake3::Hasher::new(),
            abort,
        };
        sink.hash.update(b"rspice-reliability-result-v1");
        let encoded = serde_json::to_writer(&mut sink, self);
        check_abort(abort)?;
        encoded.map_err(|e| invalid(e.to_string()))?;
        Ok(sink.hash.finalize().to_hex().to_string())
    }

    /// Validate dimensions and numeric domains, then recompute aging clocks
    /// and recoverable populations from retained stress samples. Electrical solutions
    /// are retained observations; this does not re-simulate or certify a fit.
    pub fn validate_retained_payload_with_abort(
        &self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        check_abort(abort)?;
        let request = &self.stress.request;
        request.validate().map_err(invalid)?;
        let study = &request.study;
        ResourceLimitError::ensure(
            ResourceKind::BatchRuns,
            study.mission.len().saturating_mul(
                request
                    .target_years
                    .len()
                    .saturating_add(1)
                    .saturating_add(usize::from(study.transient_stress.is_some())),
            ),
            limits.max_batch_runs,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            request.target_years.len(),
            limits.max_analysis_points,
        )?;
        if self.stress.phases.len() != study.mission.len()
            || self.stress.checkpoints.len() != request.target_years.len()
            || self.aged.len()
                != study
                    .mission
                    .len()
                    .saturating_mul(request.target_years.len())
        {
            return Err(invalid(
                "Reliability retained phase/checkpoint counts do not match the request",
            ));
        }
        let mut retained = 0usize;
        for (index, phase) in self.stress.phases.iter().enumerate() {
            check_abort(abort)?;
            ResourceLimitError::ensure(
                ResourceKind::AnalysisPoints,
                phase.time_s.len(),
                limits.max_analysis_points,
            )?;
            let span = study
                .transient_stress
                .as_ref()
                .map_or(study.mission[index].duration_s, |w| w.stop_s - w.start_s);
            if phase.phase_index != index
                || phase.devices.len() != study.bindings.len()
                || phase.time_s.len() < 2
                || phase.time_s[0] != 0.0
                || phase.time_s.last() != Some(&span)
                || phase.time_s.iter().any(|v| !v.is_finite())
                || phase.time_s.windows(2).any(|w| w[0] >= w[1])
                || (study.transient_stress.is_none() && phase.time_s.len() != 2)
            {
                return Err(invalid(
                    "Reliability retained stress window is inconsistent",
                ));
            }
            retained = retained.saturating_add(
                phase
                    .time_s
                    .len()
                    .saturating_mul(1 + 4 * phase.devices.len()),
            );
            for (binding, device) in study.bindings.iter().zip(&phase.devices) {
                check_abort(abort)?;
                if device.device != binding.device
                    || device.compact_model != binding.compact_model
                    || device.samples.len() != phase.time_s.len()
                    || (study.transient_stress.is_none() && device.samples[0] != device.samples[1])
                {
                    return Err(invalid(
                        "Reliability retained stress device does not match its binding",
                    ));
                }
                // Every observation is checked, including phases beyond the last
                // checkpoint and intervals whose gate polarity suspends aging.
                for id in &binding.aging_models {
                    let model = study
                        .model_pack
                        .models
                        .iter()
                        .find(|m| &m.id == id)
                        .expect("validated binding");
                    let mut clock = AgingClock::new(model).map_err(|e| invalid(e.to_string()))?;
                    for &sample in &device.samples {
                        clock.advance(0.0, sample, abort).map_err(|e| match e {
                            AgingError::Aborted => SimulationError::Aborted,
                            other => invalid(other.to_string()),
                        })?;
                    }
                }
            }
            let point = phase
                .fresh_operating_point
                .as_ref()
                .ok_or_else(|| invalid("Reliability is missing its fresh operating point"))?;
            retained = retained.saturating_add(validate_point(point, abort)?);
            ResourceLimitError::ensure(
                ResourceKind::ResultValues,
                retained,
                limits.max_result_values,
            )?;
        }
        let recomputed =
            mission::integrate(request, &self.stress.phases, *limits, retained, abort)?;
        if self.stress.checkpoints != recomputed {
            return Err(invalid(
                "Reliability aging checkpoints contradict the retained stress and calibration",
            ));
        }
        for checkpoint in &self.stress.checkpoints {
            for device in &checkpoint.devices {
                for contribution in &device.contributions {
                    retained = retained.saturating_add(
                        3 + contribution.parameters.len() + contribution.trap_occupancies.len(),
                    );
                }
            }
        }
        for (index, point) in self.aged.iter().enumerate() {
            check_abort(abort)?;
            let phase_index = index / request.target_years.len();
            let checkpoint_index = index % request.target_years.len();
            if point.phase_index != phase_index
                || point.years != request.target_years[checkpoint_index]
            {
                return Err(invalid(
                    "Reliability aged operating-point coordinates contradict the request",
                ));
            }
            let fresh = self.stress.phases[phase_index]
                .fresh_operating_point
                .as_ref()
                .unwrap();
            if !same_keys(&fresh.voltages, &point.operating_point.voltages)
                || !same_keys(
                    &fresh.branch_currents,
                    &point.operating_point.branch_currents,
                )
                || !same_keys(
                    &fresh.device_observables,
                    &point.operating_point.device_observables,
                )
            {
                return Err(invalid(
                    "Reliability aged operating-point observations differ from the fresh circuit",
                ));
            }
            retained = retained.saturating_add(validate_point(&point.operating_point, abort)?);
            let mut expected = BTreeMap::new();
            for device in &self.stress.checkpoints[checkpoint_index].devices {
                for contribution in &device.contributions {
                    for parameter in &contribution.parameters {
                        let key = (
                            device.device.as_str(),
                            parameter.parameter.to_ascii_uppercase(),
                        );
                        let entry = expected.entry(key).or_insert((parameter.update, 0.0));
                        entry.1 += parameter.shift;
                    }
                }
            }
            if expected.len() != point.parameters.len() {
                return Err(invalid(
                    "Reliability applied parameter count differs from the calibrated shifts",
                ));
            }
            if checkpoint_index > 0 {
                let first = &self.aged[phase_index * request.target_years.len()];
                if !point
                    .parameters
                    .iter()
                    .zip(&first.parameters)
                    .all(|(a, b)| {
                        a.device == b.device
                            && a.compact_model == b.compact_model
                            && a.parameter == b.parameter
                            && a.update == b.update
                            && a.fresh_value == b.fresh_value
                    })
                {
                    return Err(invalid(
                        "Reliability applied parameter identities or fresh baselines vary across lifetime checkpoints",
                    ));
                }
            }
            for parameter in &point.parameters {
                check_abort(abort)?;
                valid_name(&parameter.compact_model)?;
                let expected = expected
                    .remove(&(parameter.device.as_str(), parameter.parameter.clone()))
                    .ok_or_else(|| {
                        invalid("Reliability applied parameter is unknown or duplicated")
                    })?;
                let calculated = match parameter.update {
                    AgingParameterUpdate::Additive => parameter.fresh_value + parameter.shift,
                    AgingParameterUpdate::Relative => {
                        parameter.fresh_value * (1.0 + parameter.shift)
                    }
                };
                if parameter.update != expected.0
                    || parameter.shift != expected.1
                    || !parameter.fresh_value.is_finite()
                    || !calculated.is_finite()
                    || parameter.aged_value != calculated
                {
                    return Err(invalid(
                        "Reliability applied model value contradicts its aging shift",
                    ));
                }
                retained = retained.saturating_add(3);
            }
            ResourceLimitError::ensure(
                ResourceKind::ResultValues,
                retained,
                limits.max_result_values,
            )?;
        }
        check_abort(abort)
    }
}

fn same_keys(a: &BTreeMap<String, f64>, b: &BTreeMap<String, f64>) -> bool {
    a.keys().eq(b.keys())
}

fn valid_name(name: &str) -> Result<(), SimulationError> {
    if name.is_empty() || name.len() > 8192 || name.chars().any(char::is_control) {
        Err(invalid("Reliability observation identity is invalid"))
    } else {
        Ok(())
    }
}

fn validate_point(
    point: &ReliabilityOperatingPoint,
    abort: &dyn AbortSignal,
) -> Result<usize, SimulationError> {
    if point.voltages.is_empty() {
        return Err(invalid(
            "Reliability operating point has no voltage observations",
        ));
    }
    for values in [
        &point.voltages,
        &point.branch_currents,
        &point.device_observables,
    ] {
        let mut names = BTreeSet::new();
        for (name, value) in values {
            check_abort(abort)?;
            valid_name(name)?;
            if !value.is_finite() || !names.insert(name.to_ascii_uppercase()) {
                return Err(invalid(
                    "Reliability operating-point observations are non-finite or ambiguous",
                ));
            }
        }
    }
    Ok(point.value_count())
}
