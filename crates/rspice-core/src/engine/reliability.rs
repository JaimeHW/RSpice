//! Circuit stress extraction and lifetime integration for explicit aging fits.

use super::{Engine, SimulationError};
use crate::abort_signal::AbortSignal;
use crate::analysis::reliability::{
    AgingClock, AgingError, AgingEvaluation, AgingMechanism, AgingParameterUpdate, AgingStress,
    ReliabilityBinding, ReliabilityMissionPhase, ReliabilityRunRequest, ReliabilityStudy,
    SECONDS_PER_AGING_YEAR,
};
use crate::netlist::{Element, ElementKind, Netlist, SaveSignal};
use serde::{Deserialize, Serialize};

mod aged;
mod mission;
mod retained;
mod transfer;
pub use aged::{
    ReliabilityAgedPoint, ReliabilityAppliedParameter, ReliabilityOperatingPoint,
    ReliabilityRunResult,
};
pub use transfer::ReliabilityTransferMetadata;
#[cfg(test)]
mod tests;

/// Signed accepted stresses for one explicitly bound device.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityDeviceStress {
    pub device: String,
    pub compact_model: String,
    pub samples: Vec<AgingStress>,
}

/// One representative phase window. Time starts at zero after warmup removal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityPhaseStress {
    pub phase_index: usize,
    pub fresh_operating_point: Option<ReliabilityOperatingPoint>,
    pub time_s: Vec<f64>,
    pub devices: Vec<ReliabilityDeviceStress>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityDeviceAging {
    pub device: String,
    pub contributions: Vec<AgingEvaluation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityAgingCheckpoint {
    pub years: f64,
    pub devices: Vec<ReliabilityDeviceAging>,
}

/// Fully retained stress evidence and calculated parameter changes. Aged
/// circuit re-simulation is a separate stage, not inferred from these shifts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityStressResult {
    pub request: ReliabilityRunRequest,
    pub phases: Vec<ReliabilityPhaseStress>,
    pub checkpoints: Vec<ReliabilityAgingCheckpoint>,
}

impl Engine {
    /// Solve every mission phase, then integrate the chosen fits at each age.
    /// Stress is extracted from the fresh circuit and remains fixed throughout
    /// this stage. Irreversible exposure uses trapezoidal acceleration. Trapping
    /// uses chronological endpoint half steps of the kinetic equation.
    pub fn run_reliability_stress_with_abort(
        &self,
        netlist: &Netlist,
        request: &ReliabilityRunRequest,
        abort: &dyn AbortSignal,
    ) -> Result<ReliabilityStressResult, SimulationError> {
        check_abort(abort)?;
        request.validate().map_err(invalid)?;
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::BatchRuns,
            request.study.mission.len(),
            self.config.resource_limits.max_batch_runs,
        )?;
        let mut phases = Vec::new();
        let mut retained = 0usize;
        for (phase_index, phase) in request.study.mission.iter().enumerate() {
            check_abort(abort)?;
            let materialized = self.reliability_phase_netlist(netlist, phase, abort)?;
            let mut config = self.resolved_for_netlist(&materialized).config.clone();
            config.temperature = crate::constants::celsius_to_kelvin(phase.temperature_c);
            let engine = self
                .try_resolved_with_config(config)
                .map_err(SimulationError::Configuration)?;
            let result = engine.reliability_phase_stress(
                &materialized,
                &request.study,
                phase_index,
                abort,
            )?;
            retained = retained.saturating_add(
                result
                    .time_s
                    .len()
                    .saturating_mul(1 + 4 * result.devices.len()),
            );
            self.ensure_result_values(retained)?;
            retained = retained.saturating_add(
                result
                    .fresh_operating_point
                    .as_ref()
                    .map_or(0, ReliabilityOperatingPoint::value_count),
            );
            self.ensure_result_values(retained)?;
            phases.push(result);
        }
        let checkpoints = mission::integrate(
            request,
            &phases,
            self.config.resource_limits,
            retained,
            abort,
        )?;
        Ok(ReliabilityStressResult {
            request: request.clone(),
            phases,
            checkpoints,
        })
    }

    pub(super) fn reliability_phase_netlist(
        &self,
        netlist: &Netlist,
        phase: &ReliabilityMissionPhase,
        abort: &dyn AbortSignal,
    ) -> Result<Netlist, SimulationError> {
        check_abort(abort)?;
        let mut overrides = Vec::new();
        for (name, value) in &phase.parameters {
            check_abort(abort)?;
            if !netlist.params.has_any_parameter_binding(name) {
                return Err(invalid(format!(
                    "Mission parameter '{name}' is not defined in the circuit"
                )));
            }
            overrides.push((name.clone(), *value));
        }
        let mut materialized = if overrides.is_empty() {
            netlist.clone()
        } else {
            let (materialized, applied) =
                Self::create_perturbed_netlist_multi_with_limits_and_abort(
                    netlist,
                    &overrides,
                    self.config.resource_limits,
                    abort,
                )?;
            if applied != overrides.len() {
                return Err(invalid("Mission parameter overrides were not all applied"));
            }
            materialized
        };
        materialized.options.temp = Some(phase.temperature_c);
        Ok(materialized)
    }

    fn reliability_phase_stress(
        &self,
        netlist: &Netlist,
        study: &ReliabilityStudy,
        phase_index: usize,
        abort: &dyn AbortSignal,
    ) -> Result<ReliabilityPhaseStress, SimulationError> {
        let flattened = crate::netlist::flatten_netlist_with_models_config_with_abort(
            netlist,
            crate::netlist::FlattenerConfig {
                max_depth: self.config.resource_limits.max_hierarchy_depth,
                max_elements: self.config.resource_limits.max_flattened_elements,
                ..Default::default()
            },
            abort,
        )
        .map_err(parse_error)?;
        let mut model_context = netlist.clone();
        model_context.models.extend(flattened.scoped_models.clone());
        let mut targets = Vec::new();
        for binding in &study.bindings {
            check_abort(abort)?;
            let element = flattened
                .elements
                .iter()
                .find(|e| e.name.eq_ignore_ascii_case(&binding.device))
                .ok_or_else(|| {
                    invalid(format!(
                        "Bound reliability device '{}' was not found",
                        binding.device
                    ))
                })?;
            let circuit_temperature = super::builder::reliability_mos_uses_circuit_temperature(
                &model_context,
                element,
                &binding.compact_model,
                self.config.temperature,
                self.config.spice_dialect,
            )?;
            let target = StressTarget::new(
                element,
                binding,
                self.config.temperature,
                circuit_temperature,
            )?;
            if !target.fet
                && binding.aging_models.iter().any(|id| {
                    study.model_pack.models.iter().any(|model| {
                        &model.id == id && model.mechanism != AgingMechanism::Electromigration
                    })
                })
            {
                return Err(invalid(format!(
                    "Device '{}' cannot supply transistor gate stress for HCI/NBTI",
                    binding.device
                )));
            }
            targets.push(target);
        }
        let mut output = ReliabilityPhaseStress {
            phase_index,
            fresh_operating_point: None,
            time_s: Vec::new(),
            devices: Vec::new(),
        };
        if let Some(window) = &study.transient_stress {
            let mut observed = netlist.clone();
            // Retain the actual accepted states, independently of visible .SAVE selection.
            for target in &targets {
                for node in &target.nodes {
                    observed
                        .saves
                        .signals
                        .push(SaveSignal::Voltage(node.clone()));
                }
                if target.binding.conductor_area_m2.is_some() {
                    observed.saves.signals.push(if target.fet {
                        SaveSignal::DeviceParam {
                            device: target.binding.device.clone(),
                            param: target.current_parameter.into(),
                        }
                    } else {
                        SaveSignal::Current(target.binding.device.clone())
                    });
                }
            }
            let result = self.run_tran_with_startup_mode_and_abort(
                &observed,
                window.stop_s,
                crate::analysis::transient::resolve_transient_maximum_step(
                    window.step_s,
                    window.stop_s,
                    Some(window.start_s),
                    window.max_step_s,
                )
                .map_err(|error| invalid(error.to_string()))?,
                super::TransientStartupMode::from_uic(window.use_initial_conditions),
                abort,
            )?;
            let span = window.stop_s - window.start_s;
            let mut times = vec![window.start_s];
            for &time in &result.time {
                check_abort(abort)?;
                if time > window.start_s && time < window.stop_s {
                    times.push(time);
                }
            }
            times.push(window.stop_s);
            if result.time.first().is_none_or(|t| *t > window.start_s)
                || result.time.last().is_none_or(|t| *t < window.stop_s)
            {
                return Err(invalid(
                    "Transient reliability stress did not cover the requested window",
                ));
            }
            self.ensure_result_shape(times.len(), 1 + 4 * targets.len())?;
            output.time_s = times.iter().map(|t| t - window.start_s).collect();
            *output.time_s.last_mut().unwrap() = span;
            for target in targets {
                check_abort(abort)?;
                let node_indices: Vec<_> = target
                    .nodes
                    .iter()
                    .map(|node| {
                        if netlist.ground_policy().canonical_node(node) == "0" {
                            Ok(None)
                        } else {
                            result
                                .node_names
                                .iter()
                                .position(|name| name.eq_ignore_ascii_case(node))
                                .map(Some)
                                .ok_or_else(|| {
                                    invalid(format!("No transient voltage for '{node}'"))
                                })
                        }
                    })
                    .collect::<Result<_, _>>()?;
                let mut samples = Vec::new();
                for &time in &times {
                    check_abort(abort)?;
                    let volts: Vec<_> = node_indices
                        .iter()
                        .map(|index| match index {
                            None => Ok(0.0),
                            Some(index) => {
                                interpolate(&result.time, &result.voltages[*index], time)
                                    .ok_or_else(|| {
                                        invalid("Missing accepted transient terminal voltage")
                                    })
                            }
                        })
                        .collect::<Result<_, _>>()?;
                    let current = if target.binding.conductor_area_m2.is_some() {
                        (if target.fet {
                            result.try_device_op_waveform_named(
                                &target.binding.device,
                                target.current_parameter,
                            )
                        } else {
                            result.try_branch_current_waveform_named(&target.binding.device)
                        })
                        .and_then(|values| interpolate(&result.time, values, time))
                        .ok_or_else(|| {
                            invalid(format!(
                                "No accepted {} current for '{}'",
                                target.current_parameter, target.binding.device
                            ))
                        })?
                    } else {
                        0.0
                    };
                    samples.push(target.stress(&volts, current)?);
                }
                output.devices.push(target.finish(samples));
            }
        } else {
            let result = self.run_dc_op_with_abort(netlist, abort)?;
            output.fresh_operating_point = Some(ReliabilityOperatingPoint::from_result(&result));
            output.time_s = vec![0.0, study.mission[phase_index].duration_s];
            self.ensure_result_shape(2, 1 + 4 * targets.len())?;
            for target in targets {
                check_abort(abort)?;
                let volts: Vec<_> = target
                    .nodes
                    .iter()
                    .map(|node| {
                        result
                            .try_voltage_named(netlist.ground_policy().canonical_node(node))
                            .ok_or_else(|| invalid(format!("No DC voltage for '{node}'")))
                    })
                    .collect::<Result<_, _>>()?;
                let current = if target.binding.conductor_area_m2.is_some() {
                    result
                        .try_dc_observable_named(&format!(
                            "{}:{}",
                            target.binding.device, target.current_parameter
                        ))
                        .or_else(|| {
                            result.try_dc_observable_named(&format!("I({})", target.binding.device))
                        })
                        .ok_or_else(|| {
                            invalid(format!(
                                "No DC terminal current for '{}'",
                                target.binding.device
                            ))
                        })?
                } else {
                    0.0
                };
                let stress = target.stress(&volts, current)?;
                output.devices.push(target.finish(vec![stress, stress]));
            }
        }
        Ok(output)
    }
}

struct StressTarget<'a> {
    binding: &'a ReliabilityBinding,
    nodes: Vec<String>,
    fet: bool,
    temperature_k: f64,
    current_parameter: &'static str,
}

impl<'a> StressTarget<'a> {
    fn new(
        element: &Element,
        binding: &'a ReliabilityBinding,
        ambient_k: f64,
        circuit_temperature: bool,
    ) -> Result<Self, SimulationError> {
        let (model, instance_params, fet, current_parameter) = match &element.kind {
            ElementKind::Mosfet {
                model,
                instance_params,
                ..
            }
            | ElementKind::Jfet {
                model,
                instance_params,
                ..
            }
            | ElementKind::Mesfet {
                model,
                instance_params,
                ..
            } => (model.as_str(), instance_params, true, "id"),
            ElementKind::Resistor {
                model: Some(model),
                instance_params,
                ..
            } => (model.as_str(), instance_params, false, "i"),
            _ => {
                return Err(invalid(format!(
                    "Reliability needs explicit FET terminal stress or a modeled conductor for '{}'",
                    element.name
                )));
            }
        };
        if !model.eq_ignore_ascii_case(&binding.compact_model) {
            return Err(invalid(format!(
                "Device '{}' uses compact model '{model}', not '{}'",
                element.name, binding.compact_model
            )));
        }
        if element.nodes.len() < if fet { 3 } else { 2 }
            || element.nodes.len() > if fet { 4 } else { 2 }
        {
            return Err(invalid(format!(
                "Device '{}' needs an explicit thermal/terminal aging adapter",
                element.name
            )));
        }
        let instance = |key: &str| {
            instance_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(key))
                .map(|(_, value)| *value)
        };
        let temperature_k = if circuit_temperature {
            ambient_k
        } else {
            instance("TEMP")
                .map(crate::constants::celsius_to_kelvin)
                .unwrap_or(ambient_k + instance("DTEMP").unwrap_or(0.0))
        };
        if !temperature_k.is_finite() || temperature_k <= 0.0 {
            return Err(invalid("Device stress temperature is invalid"));
        }
        Ok(Self {
            binding,
            nodes: element.nodes.clone(),
            fet,
            temperature_k,
            current_parameter,
        })
    }

    fn stress(&self, volts: &[f64], current: f64) -> Result<AgingStress, SimulationError> {
        let stress = AgingStress {
            gate_source_v: if self.fet { volts[1] - volts[2] } else { 0.0 },
            drain_source_v: volts[0] - volts[if self.fet { 2 } else { 1 }],
            temperature_k: self.temperature_k,
            current_density_a_per_m2: self
                .binding
                .conductor_area_m2
                .map_or(0.0, |area| current.abs() / area),
        };
        if [
            stress.gate_source_v,
            stress.drain_source_v,
            stress.temperature_k,
            stress.current_density_a_per_m2,
        ]
        .iter()
        .any(|v| !v.is_finite())
        {
            return Err(invalid(format!(
                "Non-finite circuit stress for '{}'",
                self.binding.device
            )));
        }
        Ok(stress)
    }

    fn finish(self, samples: Vec<AgingStress>) -> ReliabilityDeviceStress {
        ReliabilityDeviceStress {
            device: self.binding.device.clone(),
            compact_model: self.binding.compact_model.clone(),
            samples,
        }
    }
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

fn invalid(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(message.into())
}

fn interpolate(times: &[f64], values: &[f64], target: f64) -> Option<f64> {
    if values.len() != times.len() || times.is_empty() {
        return None;
    }
    match times.binary_search_by(|time| time.total_cmp(&target)) {
        Ok(index) => values[index].is_finite().then_some(values[index]),
        Err(index) if index > 0 && index < times.len() => {
            let weight = (target - times[index - 1]) / (times[index] - times[index - 1]);
            let value = (1.0 - weight) * values[index - 1] + weight * values[index];
            value.is_finite().then_some(value)
        }
        _ => None,
    }
}

fn parse_error(error: crate::netlist::ParseWithAbortError) -> SimulationError {
    match error {
        crate::netlist::ParseWithAbortError::Aborted => SimulationError::Aborted,
        crate::netlist::ParseWithAbortError::Parse(crate::netlist::ParseError::ResourceLimit(
            e,
        )) => SimulationError::ResourceLimit(e),
        crate::netlist::ParseWithAbortError::Parse(e) => SimulationError::Netlist(e.to_string()),
    }
}
