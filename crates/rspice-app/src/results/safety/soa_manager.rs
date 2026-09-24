//! Sampling and duration finalization against the portable SOA contract.
use super::{
    SoADefinition, SoAEvaluation, SoAParameter, SoARuleVerdict, SoAViolation, SoaDeratingSamples,
    SoaPowerDeratingEvidence, ViolationSeverity, compare_soa_stress,
};
#[cfg(test)]
use super::{SoALimit, SoaPowerDerating};
use std::collections::HashMap;

// =============================================================================
// SOA Manager
// =============================================================================

/// Manager for tracking and checking Safe Operating Area limits
pub struct SoAManager {
    thresholds: super::SoaThresholds,
    /// Device instance ID -> SOA Definition
    device_defs: HashMap<String, SoADefinition>,
    /// Accumulated violations from most recent check
    violations: Vec<SoAViolation>,
    /// Complete evaluated-rule coverage, keyed by stable device/parameter identity.
    evaluations: HashMap<(String, SoAParameter), SoAEvaluation>,
    /// Every sampled stress magnitude per rule, in `check_point` order.
    ///
    /// Kept beside `evaluations` and written in the same step, so a rule's
    /// history can never disagree with the worst point derived from it.
    stress_history: HashMap<(String, SoAParameter), Vec<f64>>,
    derating_history: HashMap<(String, SoAParameter), SoaDeratingSamples>,
    envelope_history: HashMap<(String, SoAParameter), super::SoaEnvelopeSamples>,
}

impl Default for SoAManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SoAManager {
    pub fn new() -> Self {
        Self {
            thresholds: Default::default(),
            device_defs: HashMap::new(),
            violations: Vec::new(),
            evaluations: HashMap::new(),
            stress_history: HashMap::new(),
            derating_history: HashMap::new(),
            envelope_history: HashMap::new(),
        }
    }

    pub fn with_thresholds(thresholds: super::SoaThresholds) -> Result<Self, String> {
        thresholds.validate()?;
        Ok(Self {
            thresholds,
            ..Self::new()
        })
    }

    /// Register SOA limits for a device
    pub fn register_device(
        &mut self,
        device_id: impl Into<String>,
        def: SoADefinition,
    ) -> Result<(), String> {
        let device_id = device_id.into();
        if device_id.trim().is_empty() {
            return Err("SOA device identity is empty".to_owned());
        }
        if def.limits.is_empty() {
            return Err(format!("SOA device '{device_id}' has no enabled rules"));
        }
        let mut parameters = std::collections::HashSet::new();
        for limit in &def.limits {
            if !parameters.insert(limit.parameter) {
                return Err(format!(
                    "SOA device '{device_id}' has duplicate rules for {:?}",
                    limit.parameter
                ));
            }
            if !limit.max_value.is_finite()
                || limit.max_value < 0.0
                || (limit.max_value == 0.0 && limit.parameter.polarity().is_none())
            {
                return Err(format!(
                    "SOA device '{device_id}' has an invalid {:?} limit",
                    limit.parameter
                ));
            }
            limit.duration_mode.validate(limit.minimum_duration_s)?;
            if let Some(curve) = &limit.current_envelope {
                curve.validate()?;
                if limit.max_value <= 0.0 {
                    return Err(
                        "SOA current/voltage curves require a positive maximum-current cap".into(),
                    );
                }
                if super::SoaCurrentEnvelope::voltage_parameter(limit.parameter).is_none() {
                    return Err("SOA current/voltage curves require Id, Ic or Ia".into());
                }
            }
            if let Some(curve) = limit.power_derating {
                curve.validate()?;
                if limit.parameter != SoAParameter::Pdiss {
                    return Err("SOA temperature derating applies only to conductive power".into());
                }
            }
            if limit.unit.trim().is_empty() || limit.description.trim().is_empty() {
                return Err(format!(
                    "SOA device '{device_id}' has incomplete {:?} rule metadata",
                    limit.parameter
                ));
            }
        }
        if self.device_defs.contains_key(&device_id) {
            return Err(format!("SOA device '{device_id}' was registered twice"));
        }
        self.device_defs.insert(device_id, def);
        Ok(())
    }

    /// Clear all violations
    #[cfg(test)]
    pub fn clear_violations(&mut self) {
        self.violations.clear();
        self.evaluations.clear();
        self.stress_history.clear();
        self.derating_history.clear();
        self.envelope_history.clear();
    }

    /// Check a single measurement point for all registered devices
    #[cfg(test)]
    pub fn check_point(
        &mut self,
        time: f64,
        values: &HashMap<String, HashMap<SoAParameter, f64>>,
    ) -> Result<(), String> {
        self.check_point_with_curve_voltages(time, values, &HashMap::new())
    }

    pub fn check_point_with_curve_voltages(
        &mut self,
        time: f64,
        values: &HashMap<String, HashMap<SoAParameter, f64>>,
        curve_voltages: &HashMap<(String, SoAParameter), f64>,
    ) -> Result<(), String> {
        if !time.is_finite() || time < 0.0 {
            return Err("SOA sample time must be finite and nonnegative".to_owned());
        }
        for (device_id, device_values) in values {
            if let Some(def) = self.device_defs.get(device_id) {
                for limit in &def.limits {
                    if let Some(&actual) = device_values.get(&limit.parameter) {
                        if !actual.is_finite() || actual < 0.0 {
                            return Err(format!(
                                "SOA device '{device_id}' has an invalid {:?} sample",
                                limit.parameter
                            ));
                        }
                        let temperature = if limit.power_derating.is_some() {
                            let temperature = device_values.get(&SoAParameter::Temp).copied().ok_or_else(|| format!("SOA derating for '{device_id}' requires accepted device temperature"))?;
                            if !temperature.is_finite() || temperature <= 0.0 {
                                return Err(format!(
                                    "SOA derating for '{device_id}' requires temperature above absolute zero"
                                ));
                            }
                            Some(temperature)
                        } else {
                            None
                        };
                        let mut maximum = limit
                            .power_derating
                            .zip(temperature)
                            .map_or(limit.max_value, |(curve, temperature)| {
                                curve.limit(limit.max_value, temperature)
                            });
                        let key = (device_id.clone(), limit.parameter);
                        if let Some(curve) = &limit.current_envelope {
                            let voltage = *curve_voltages
                                .get(&key)
                                .ok_or("SOA current curve is missing terminal voltage")?;
                            maximum = maximum.min(curve.limit(voltage)?);
                            let history = self.envelope_history.entry(key.clone()).or_default();
                            history.voltages_v.push(voltage);
                            history.limits_a.push(maximum);
                        }
                        let verdict = self.thresholds.verdict(actual, maximum);
                        if let Some(temperature) = temperature {
                            let history = self.derating_history.entry(key.clone()).or_default();
                            history.temperatures_kelvin.push(temperature);
                            history.limits_w.push(maximum);
                        }
                        self.stress_history
                            .entry(key.clone())
                            .or_default()
                            .push(actual);
                        let evaluation =
                            self.evaluations
                                .entry(key)
                                .or_insert_with(|| SoAEvaluation {
                                    duration: None,
                                    thresholds: self.thresholds,
                                    envelope: limit.current_envelope.clone().map(|curve| {
                                        super::SoaCurrentEnvelopeEvidence {
                                            maximum_current_a: limit.max_value,
                                            curve,
                                        }
                                    }),
                                    derating: limit.power_derating.map(|curve| {
                                        SoaPowerDeratingEvidence {
                                            rated_power_w: limit.max_value,
                                            curve,
                                        }
                                    }),
                                    device_id: device_id.clone(),
                                    parameter: limit.parameter,
                                    limit_value: maximum,
                                    worst_actual_value: actual,
                                    worst_time: time,
                                    sample_count: 0,
                                    unit: limit.unit.clone(),
                                    description: limit.description.clone(),
                                    verdict,
                                });
                        evaluation.sample_count =
                            evaluation.sample_count.checked_add(1).ok_or_else(|| {
                                format!("SOA sample count overflow for device '{device_id}'")
                            })?;
                        if compare_soa_stress(
                            actual,
                            maximum,
                            evaluation.worst_actual_value,
                            evaluation.limit_value,
                        )
                        .is_gt()
                        {
                            evaluation.limit_value = maximum;
                            evaluation.worst_actual_value = actual;
                            evaluation.worst_time = time;
                            evaluation.verdict = verdict;
                        }
                        let severity = match verdict {
                            SoARuleVerdict::Pass => None,
                            SoARuleVerdict::Warning => Some(ViolationSeverity::Warning),
                            SoARuleVerdict::Violation => Some(ViolationSeverity::Violation),
                            SoARuleVerdict::Critical => Some(ViolationSeverity::Critical),
                        };
                        if let Some(severity) = severity {
                            self.violations.push(SoAViolation {
                                device_id: device_id.clone(),
                                parameter: limit.parameter,
                                limit_value: maximum,
                                actual_value: actual,
                                time,
                                severity,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Reclassify complete excursions once the checked observation window is known.
    pub fn finalize_durations(
        &mut self,
        time: &[f64],
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<bool, rspice_core::SimulationError> {
        use super::{SoaLimitTrace, qualify_soa_duration_with_mode, soa_duration_verdict};
        use rspice_core::SimulationError;
        let policies: Vec<_> = self
            .device_defs
            .iter()
            .flat_map(|(device, definition)| {
                definition.limits.iter().filter_map(move |limit| {
                    limit.minimum_duration_s.map(|minimum| {
                        (
                            device.clone(),
                            limit.parameter,
                            limit.max_value,
                            minimum,
                            limit.duration_mode,
                        )
                    })
                })
            })
            .collect();
        if policies.is_empty() {
            return Ok(false);
        }
        let mut keys: HashMap<&str, std::collections::HashSet<SoAParameter>> = HashMap::new();
        for (device, parameter, _, _, _) in &policies {
            keys.entry(device.as_str()).or_default().insert(*parameter);
        }
        let mut retained = Vec::new();
        for (index, event) in self.violations.drain(..).enumerate() {
            if index % 256 == 0 && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if !keys
                .get(event.device_id.as_str())
                .is_some_and(|parameters| parameters.contains(&event.parameter))
            {
                retained.push(event);
            }
        }
        self.violations = retained;
        for (device, parameter, maximum, minimum, mode) in policies {
            let key = (device.clone(), parameter);
            let stress = self.stress_history.get(&key).ok_or_else(|| {
                SimulationError::Circuit("SOA duration is missing its stress history".into())
            })?;
            let limits = self
                .derating_history
                .get(&key)
                .map_or(SoaLimitTrace::Constant(maximum), |history| {
                    SoaLimitTrace::Samples(&history.limits_w)
                });
            let limits = self
                .envelope_history
                .get(&key)
                .map_or(limits, |history| SoaLimitTrace::Samples(&history.limits_a));
            let scan = qualify_soa_duration_with_mode(time, stress, limits, minimum, mode, abort)?;
            let mut worst = 0;
            let mut verdict = soa_duration_verdict(
                self.thresholds,
                stress[0],
                limits.at(0),
                scan.qualified_samples[0],
            );
            for i in 0..time.len() {
                if i % 256 == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let sample_verdict = soa_duration_verdict(
                    self.thresholds,
                    stress[i],
                    limits.at(i),
                    scan.qualified_samples[i],
                );
                if sample_verdict
                    .cmp(&verdict)
                    .then_with(|| {
                        compare_soa_stress(stress[i], limits.at(i), stress[worst], limits.at(worst))
                    })
                    .is_gt()
                {
                    worst = i;
                    verdict = sample_verdict;
                }
                let severity = match sample_verdict {
                    SoARuleVerdict::Pass => None,
                    SoARuleVerdict::Warning => Some(ViolationSeverity::Warning),
                    SoARuleVerdict::Violation => Some(ViolationSeverity::Violation),
                    SoARuleVerdict::Critical => Some(ViolationSeverity::Critical),
                };
                if let Some(severity) = severity {
                    self.violations.push(SoAViolation {
                        device_id: device.clone(),
                        parameter,
                        limit_value: limits.at(i),
                        actual_value: stress[i],
                        time: time[i],
                        severity,
                    });
                }
            }
            let evaluation = self.evaluations.get_mut(&key).ok_or_else(|| {
                SimulationError::Circuit("SOA duration is missing its evaluation".into())
            })?;
            evaluation.duration = Some(scan.evidence);
            evaluation.verdict = verdict;
            evaluation.worst_time = time[worst];
            evaluation.worst_actual_value = stress[worst];
            evaluation.limit_value = limits.at(worst);
        }
        Ok(true)
    }

    /// Retained external voltage and allowed current for a curve rule.
    pub fn envelope_history(
        &self,
        device: &str,
        parameter: SoAParameter,
    ) -> Option<&super::SoaEnvelopeSamples> {
        self.envelope_history.get(&(device.into(), parameter))
    }

    /// Get all detected violations
    pub fn violations(&self) -> &[SoAViolation] {
        &self.violations
    }

    /// Iterate the complete evaluated-rule set. Callers that persist or
    /// transport these records must impose canonical ordering.
    pub fn evaluations(&self) -> impl Iterator<Item = &SoAEvaluation> {
        self.evaluations.values()
    }

    pub fn derating_history(
        &self,
        device_id: &str,
        parameter: SoAParameter,
    ) -> Option<&SoaDeratingSamples> {
        self.derating_history
            .get(&(device_id.to_owned(), parameter))
    }

    /// The sampled stress history for one evaluated rule, in sample order.
    ///
    /// Its length always equals that rule's `sample_count`; a rule the run
    /// never sampled has no entry at all.
    pub fn stress_history(&self, device_id: &str, parameter: SoAParameter) -> Option<&[f64]> {
        self.stress_history
            .get(&(device_id.to_owned(), parameter))
            .map(Vec::as_slice)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_retains_complete_rule_coverage_and_exact_worst_point() {
        let mut manager = SoAManager::new();
        manager
            .register_device(
                "M1",
                SoADefinition {
                    limits: vec![SoALimit {
                        duration_mode: Default::default(),
                        minimum_duration_s: None,
                        current_envelope: None,
                        power_derating: None,
                        voltage_basis: Default::default(),
                        parameter: SoAParameter::Vds,
                        max_value: 10.0,
                        unit: "V".to_owned(),
                        description: "Maximum drain-source voltage".to_owned(),
                    }],
                },
            )
            .expect("valid SOA rule registers");
        assert!(
            manager
                .register_device(
                    "M1",
                    SoADefinition {
                        limits: vec![SoALimit {
                            duration_mode: Default::default(),
                            minimum_duration_s: None,
                            current_envelope: None,
                            power_derating: None,
                            voltage_basis: Default::default(),
                            parameter: SoAParameter::Vds,
                            max_value: 1.0,
                            unit: "V".to_owned(),
                            description: "Conflicting duplicate".to_owned(),
                        }],
                    },
                )
                .is_err()
        );

        for (time, actual) in [(0.0, 5.0), (1.0, 9.5), (2.0, 12.5)] {
            manager
                .check_point(
                    time,
                    &HashMap::from([(
                        "M1".to_owned(),
                        HashMap::from([(SoAParameter::Vds, actual)]),
                    )]),
                )
                .expect("finite SOA sample evaluates");
        }

        let evaluations = manager.evaluations().collect::<Vec<_>>();
        assert_eq!(evaluations.len(), 1);
        assert_eq!(evaluations[0].sample_count, 3);
        assert_eq!(evaluations[0].limit_value, 10.0);
        assert_eq!(evaluations[0].worst_actual_value, 12.5);
        assert_eq!(evaluations[0].worst_time, 2.0);
        assert_eq!(evaluations[0].verdict, SoARuleVerdict::Critical);
        assert_eq!(manager.violations().len(), 2);
        assert_eq!(manager.violations()[0].severity, ViolationSeverity::Warning);
        assert_eq!(
            manager.violations()[1].severity,
            ViolationSeverity::Critical
        );

        manager.clear_violations();
        assert!(manager.violations().is_empty());
        assert_eq!(manager.evaluations().count(), 0);
    }
}

#[cfg(test)]
#[test]
fn soa_derating_selects_highest_utilization_and_retains_zero_limit_events() {
    let curve = SoaPowerDerating {
        reference_temperature_kelvin: 300.0,
        watts_per_kelvin: 0.01,
    };
    let mut manager = SoAManager::with_thresholds(super::SoaThresholds {
        warning_fraction: Some(0.95),
        critical_fraction: Some(1.5),
    })
    .unwrap();
    manager
        .register_device(
            "Q1",
            SoADefinition {
                limits: vec![SoALimit {
                    duration_mode: Default::default(),
                    minimum_duration_s: None,
                    current_envelope: None,
                    power_derating: Some(curve),
                    voltage_basis: Default::default(),
                    parameter: SoAParameter::Pdiss,
                    max_value: 1.0,
                    unit: "W".into(),
                    description: "Rated power".into(),
                }],
            },
        )
        .unwrap();
    let values = |power, temp| {
        HashMap::from([(
            "Q1".into(),
            HashMap::from([(SoAParameter::Pdiss, power), (SoAParameter::Temp, temp)]),
        )])
    };
    assert!(
        manager
            .check_point(
                0.0,
                &HashMap::from([("Q1".into(), HashMap::from([(SoAParameter::Pdiss, 0.8)]))])
            )
            .is_err()
    );
    for (time, power, temp) in [(0.0, 0.8, 290.0), (1.0, 0.5, 350.0), (2.0, 0.2, 400.0)] {
        manager.check_point(time, &values(power, temp)).unwrap();
    }
    let result = manager.evaluations().next().unwrap();
    assert_eq!(result.worst_actual_value, 0.2);
    assert_eq!(result.worst_time, 2.0);
    assert_eq!(result.limit_value, 0.0);
    assert_eq!(result.verdict, SoARuleVerdict::Critical);
    assert!(compare_soa_stress(f64::MAX, 1e-308, 1.0, 0.0).is_lt());
    assert!(compare_soa_stress(f64::MAX, 1e-308, f64::MAX / 2.0, 1e-310).is_lt());
    assert!(compare_soa_stress(1e-200, 1e308, 1e-300, 1e100).is_lt());
    assert_eq!(
        manager
            .derating_history("Q1", SoAParameter::Pdiss)
            .unwrap()
            .limits_w,
        vec![1.0, 0.5, 0.0]
    );
    assert_eq!(manager.violations().len(), 2);
    manager.clear_violations();
    assert!(
        manager
            .derating_history("Q1", SoAParameter::Pdiss)
            .is_none()
    );
    assert!(manager.stress_history("Q1", SoAParameter::Pdiss).is_none());
}
