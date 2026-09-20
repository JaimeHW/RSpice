//! Bounded metadata and one numeric buffer for worker reliability evidence.

use super::*;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityTransferMetadata {
    request: ReliabilityRunRequest,
    phases: Vec<PhaseLayout>,
    aged: Vec<AgedLayout>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PhaseLayout {
    samples: usize,
    fresh: PointLayout,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PointLayout {
    voltages: Vec<String>,
    currents: Vec<String>,
    observables: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgedLayout {
    parameters: Vec<ParameterLayout>,
    point: PointLayout,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ParameterLayout {
    device: String,
    model: String,
    parameter: String,
    update: AgingParameterUpdate,
}

impl PointLayout {
    fn from_point(point: &ReliabilityOperatingPoint) -> Self {
        Self {
            voltages: point.voltages.keys().cloned().collect(),
            currents: point.branch_currents.keys().cloned().collect(),
            observables: point.device_observables.keys().cloned().collect(),
        }
    }
    fn count(&self) -> Result<usize, SimulationError> {
        let mut count = 0usize;
        for keys in [&self.voltages, &self.currents, &self.observables] {
            let mut seen = BTreeSet::new();
            for key in keys {
                if key.is_empty()
                    || key.len() > 8192
                    || key.chars().any(char::is_control)
                    || !seen.insert(key.to_ascii_uppercase())
                {
                    return Err(invalid(
                        "Reliability transfer contains ambiguous observation names",
                    ));
                }
            }
            count = count.saturating_add(keys.len());
        }
        Ok(count)
    }
    fn read(
        self,
        values: &mut std::slice::Iter<'_, f64>,
        abort: &dyn AbortSignal,
    ) -> Result<ReliabilityOperatingPoint, SimulationError> {
        fn read_map(
            keys: Vec<String>,
            values: &mut std::slice::Iter<'_, f64>,
            abort: &dyn AbortSignal,
        ) -> Result<BTreeMap<String, f64>, SimulationError> {
            keys.into_iter()
                .map(|key| {
                    check_abort(abort)?;
                    Ok((key, *values.next().expect("validated buffer length")))
                })
                .collect()
        }
        Ok(ReliabilityOperatingPoint {
            voltages: read_map(self.voltages, values, abort)?,
            branch_currents: read_map(self.currents, values, abort)?,
            device_observables: read_map(self.observables, values, abort)?,
        })
    }
}

impl ReliabilityTransferMetadata {
    /// Checked before copying numeric arrays out of an untrusted worker frame.
    pub fn validate_transfer_layout_with_abort(
        &self,
        values: usize,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        check_abort(abort)?;
        self.request.validate().map_err(invalid)?;
        let study = &self.request.study;
        ResourceLimitError::ensure(
            ResourceKind::BatchRuns,
            study.mission.len().saturating_mul(
                self.request
                    .target_years
                    .len()
                    .saturating_add(1)
                    .saturating_add(usize::from(study.transient_stress.is_some())),
            ),
            limits.max_batch_runs,
        )?;
        ResourceLimitError::ensure(ResourceKind::ResultValues, values, limits.max_result_values)?;
        if self.phases.len() != study.mission.len()
            || self.aged.len()
                != study
                    .mission
                    .len()
                    .saturating_mul(self.request.target_years.len())
        {
            return Err(invalid(
                "Reliability transfer phase/checkpoint dimensions are inconsistent",
            ));
        }
        let mut count = 0usize;
        for phase in &self.phases {
            check_abort(abort)?;
            if phase.samples < 2 {
                return Err(invalid("Reliability transfer stress window is empty"));
            }
            ResourceLimitError::ensure(
                ResourceKind::AnalysisPoints,
                phase.samples,
                limits.max_analysis_points,
            )?;
            count = count
                .saturating_add(phase.samples.saturating_mul(1 + 4 * study.bindings.len()))
                .saturating_add(phase.fresh.count()?);
        }
        for point in &self.aged {
            check_abort(abort)?;
            count = count
                .saturating_add(point.parameters.len().saturating_mul(3))
                .saturating_add(point.point.count()?);
        }
        if count != values {
            return Err(invalid(
                "Reliability transfer numeric buffer length does not match its layout",
            ));
        }
        Ok(())
    }
}

impl ReliabilityRunResult {
    pub fn into_transfer_parts_with_abort(
        self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(ReliabilityTransferMetadata, Vec<f64>), SimulationError> {
        self.validate_retained_payload_with_abort(limits, abort)?;
        let mut values = Vec::new();
        let mut phases = Vec::new();
        fn write_point(point: ReliabilityOperatingPoint, values: &mut Vec<f64>) {
            values.extend(point.voltages.into_values());
            values.extend(point.branch_currents.into_values());
            values.extend(point.device_observables.into_values());
        }
        for phase in self.stress.phases {
            check_abort(abort)?;
            let fresh = phase
                .fresh_operating_point
                .expect("validated fresh operating point");
            phases.push(PhaseLayout {
                samples: phase.time_s.len(),
                fresh: PointLayout::from_point(&fresh),
            });
            values.extend(phase.time_s);
            for device in phase.devices {
                for stress in device.samples {
                    check_abort(abort)?;
                    values.extend([
                        stress.gate_source_v,
                        stress.drain_source_v,
                        stress.temperature_k,
                        stress.current_density_a_per_m2,
                    ]);
                }
            }
            write_point(fresh, &mut values);
        }
        let mut aged = Vec::new();
        for point in self.aged {
            check_abort(abort)?;
            let mut parameters = Vec::new();
            for p in point.parameters {
                parameters.push(ParameterLayout {
                    device: p.device,
                    model: p.compact_model,
                    parameter: p.parameter,
                    update: p.update,
                });
                values.extend([p.shift, p.fresh_value, p.aged_value]);
            }
            aged.push(AgedLayout {
                parameters,
                point: PointLayout::from_point(&point.operating_point),
            });
            write_point(point.operating_point, &mut values);
        }
        let metadata = ReliabilityTransferMetadata {
            request: self.stress.request,
            phases,
            aged,
        };
        metadata.validate_transfer_layout_with_abort(values.len(), limits, abort)?;
        Ok((metadata, values))
    }

    pub fn from_transfer_parts_with_abort(
        metadata: ReliabilityTransferMetadata,
        values: Vec<f64>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        metadata.validate_transfer_layout_with_abort(values.len(), limits, abort)?;
        for value in &values {
            check_abort(abort)?;
            if !value.is_finite() {
                return Err(invalid("Reliability transfer contains a non-finite value"));
            }
        }
        let retained = values.len();
        let mut values = values.iter();
        let request = metadata.request;
        let mut phases = Vec::new();
        for (phase_index, phase) in metadata.phases.into_iter().enumerate() {
            check_abort(abort)?;
            let mut time_s = Vec::new();
            for _ in 0..phase.samples {
                check_abort(abort)?;
                time_s.push(*values.next().unwrap());
            }
            let mut devices = Vec::new();
            for binding in &request.study.bindings {
                let mut samples = Vec::new();
                for _ in 0..phase.samples {
                    check_abort(abort)?;
                    samples.push(AgingStress {
                        gate_source_v: *values.next().unwrap(),
                        drain_source_v: *values.next().unwrap(),
                        temperature_k: *values.next().unwrap(),
                        current_density_a_per_m2: *values.next().unwrap(),
                    });
                }
                devices.push(ReliabilityDeviceStress {
                    device: binding.device.clone(),
                    compact_model: binding.compact_model.clone(),
                    samples,
                });
            }
            phases.push(ReliabilityPhaseStress {
                phase_index,
                time_s,
                devices,
                fresh_operating_point: Some(phase.fresh.read(&mut values, abort)?),
            });
        }
        // Derived clocks need not be copied as a second potentially contradictory
        // numerical history. Recompute from primary samples under the exact fit.
        let checkpoints = mission::integrate(&request, &phases, *limits, retained, abort)?;
        let mut aged = Vec::new();
        for (index, point) in metadata.aged.into_iter().enumerate() {
            check_abort(abort)?;
            let mut parameters = Vec::new();
            for p in point.parameters {
                check_abort(abort)?;
                parameters.push(ReliabilityAppliedParameter {
                    device: p.device,
                    compact_model: p.model,
                    parameter: p.parameter,
                    update: p.update,
                    shift: *values.next().unwrap(),
                    fresh_value: *values.next().unwrap(),
                    aged_value: *values.next().unwrap(),
                });
            }
            aged.push(ReliabilityAgedPoint {
                phase_index: index / request.target_years.len(),
                years: request.target_years[index % request.target_years.len()],
                parameters,
                operating_point: point.point.read(&mut values, abort)?,
            });
        }
        let result = Self {
            stress: ReliabilityStressResult {
                request,
                phases,
                checkpoints,
            },
            aged,
        };
        result.validate_retained_payload_with_abort(limits, abort)?;
        Ok(result)
    }
}
