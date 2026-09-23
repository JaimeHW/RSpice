//! Fresh and aged DC evidence using instance-isolated compact-model copies.

use super::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityOperatingPoint {
    pub voltages: BTreeMap<String, f64>,
    pub branch_currents: BTreeMap<String, f64>,
    pub device_observables: BTreeMap<String, f64>,
}

impl ReliabilityOperatingPoint {
    pub(super) fn from_result(result: &crate::solver::SimulationResult) -> Self {
        Self {
            voltages: result
                .node_names
                .iter()
                .filter_map(|name| {
                    result
                        .try_voltage_named(name)
                        .map(|value| (name.clone(), value))
                })
                .collect(),
            branch_currents: result
                .branch_names
                .iter()
                .cloned()
                .zip(result.branch_currents.iter().copied())
                .collect(),
            device_observables: result.dc_observables.iter().cloned().collect(),
        }
    }

    pub(super) fn value_count(&self) -> usize {
        self.voltages
            .len()
            .saturating_add(self.branch_currents.len())
            .saturating_add(self.device_observables.len())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityAppliedParameter {
    pub device: String,
    pub compact_model: String,
    pub parameter: String,
    pub update: AgingParameterUpdate,
    /// Sum of contributions from independently characterized mechanisms.
    pub shift: f64,
    pub fresh_value: f64,
    pub aged_value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityAgedPoint {
    pub years: f64,
    pub phase_index: usize,
    pub parameters: Vec<ReliabilityAppliedParameter>,
    pub operating_point: ReliabilityOperatingPoint,
}

/// Stress is held at its fresh-circuit value while aging and recovery advance.
/// Every checkpoint is applied to the original model, never the prior aged one.
/// EM consumes fitted lifetime only and does not invent a resistance change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityRunResult {
    pub stress: ReliabilityStressResult,
    pub aged: Vec<ReliabilityAgedPoint>,
}

impl Engine {
    pub fn run_reliability_with_abort(
        &self,
        netlist: &Netlist,
        request: &ReliabilityRunRequest,
        abort: &dyn AbortSignal,
    ) -> Result<ReliabilityRunResult, SimulationError> {
        check_abort(abort)?;
        request.validate().map_err(invalid)?;
        if !netlist.ast_overlay.instance_models.is_empty() {
            return Err(invalid(
                "Reliability must start from a fresh circuit without model aging overlays",
            ));
        }
        let phase_count = request.study.mission.len();
        let runs = phase_count.saturating_mul(
            1usize
                .saturating_add(request.target_years.len())
                .saturating_add(usize::from(request.study.transient_stress.is_some())),
        );
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::BatchRuns,
            runs,
            self.config.resource_limits.max_batch_runs,
        )?;
        let mut stress = self.run_reliability_stress_with_abort(netlist, request, abort)?;
        let mut retained = stress.phases.iter().fold(0usize, |count, phase| {
            count
                .saturating_add(
                    phase
                        .time_s
                        .len()
                        .saturating_mul(1 + 4 * phase.devices.len()),
                )
                .saturating_add(
                    phase
                        .fresh_operating_point
                        .as_ref()
                        .map_or(0, ReliabilityOperatingPoint::value_count),
                )
        });
        for checkpoint in &stress.checkpoints {
            for device in &checkpoint.devices {
                for contribution in &device.contributions {
                    retained = retained.saturating_add(3 + contribution.parameters.len());
                }
            }
        }
        let mut aged = Vec::new();
        for (phase_index, phase) in request.study.mission.iter().enumerate() {
            check_abort(abort)?;
            let fresh = self.reliability_phase_netlist(netlist, phase, abort)?;
            let mut config = self.resolved_for_netlist(&fresh).config.clone();
            config.temperature = crate::constants::celsius_to_kelvin(phase.temperature_c);
            let engine = self
                .try_resolved_with_config(config)
                .map_err(SimulationError::Configuration)?;
            if stress.phases[phase_index].fresh_operating_point.is_none() {
                let point = ReliabilityOperatingPoint::from_result(
                    &engine.run_dc_op_with_abort(&fresh, abort)?,
                );
                retained = retained.saturating_add(point.value_count());
                self.ensure_result_values(retained)?;
                stress.phases[phase_index].fresh_operating_point = Some(point);
            }
            for checkpoint in &stress.checkpoints {
                check_abort(abort)?;
                let (aged_netlist, parameters) =
                    engine.reliability_aged_netlist(&fresh, &request.study, checkpoint, abort)?;
                let point = ReliabilityOperatingPoint::from_result(
                    &engine.run_dc_op_with_abort(&aged_netlist, abort)?,
                );
                retained = retained
                    .saturating_add(point.value_count())
                    .saturating_add(parameters.len().saturating_mul(3));
                self.ensure_result_values(retained)?;
                aged.push(ReliabilityAgedPoint {
                    years: checkpoint.years,
                    phase_index,
                    parameters,
                    operating_point: point,
                });
            }
        }
        Ok(ReliabilityRunResult { stress, aged })
    }

    pub(super) fn reliability_aged_netlist(
        &self,
        fresh: &Netlist,
        study: &ReliabilityStudy,
        checkpoint: &ReliabilityAgingCheckpoint,
        abort: &dyn AbortSignal,
    ) -> Result<(Netlist, Vec<ReliabilityAppliedParameter>), SimulationError> {
        let flattened = crate::netlist::flatten_netlist_with_models_config_with_abort(
            fresh,
            crate::netlist::FlattenerConfig {
                max_depth: self.config.resource_limits.max_hierarchy_depth,
                max_elements: self.config.resource_limits.max_flattened_elements,
                ..Default::default()
            },
            abort,
        )
        .map_err(parse_error)?;
        let mut model_context = fresh.clone();
        model_context.models.extend(flattened.scoped_models);
        let mut aged = fresh.clone();
        let mut evidence = Vec::new();
        for (binding, device) in study.bindings.iter().zip(&checkpoint.devices) {
            check_abort(abort)?;
            let mut changes: BTreeMap<String, (AgingParameterUpdate, f64)> = BTreeMap::new();
            for contribution in &device.contributions {
                for parameter in &contribution.parameters {
                    let entry = changes
                        .entry(parameter.parameter.to_ascii_uppercase())
                        .or_insert((parameter.update, 0.0));
                    if entry.0 != parameter.update {
                        return Err(invalid("Conflicting aging parameter update modes"));
                    }
                    entry.1 += parameter.shift;
                }
            }
            if changes.is_empty() {
                continue;
            }
            let element = flattened
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(&binding.device))
                .ok_or_else(|| invalid(format!("Aged device '{}' disappeared", binding.device)))?;
            let mut overlay = super::super::builder::reliability_model_parameters(
                &model_context,
                element,
                &binding.compact_model,
                changes.keys().cloned(),
                self.config.temperature,
            )?;
            for (parameter, (update, shift)) in changes {
                let fresh_value = overlay.parameters[&parameter];
                let aged_value = match update {
                    AgingParameterUpdate::Additive => fresh_value + shift,
                    AgingParameterUpdate::Relative => fresh_value * (1.0 + shift),
                };
                if !aged_value.is_finite() || !shift.is_finite() {
                    return Err(invalid(format!(
                        "Aged parameter '{}:{parameter}' is not representable",
                        binding.device
                    )));
                }
                overlay.parameters.insert(parameter.clone(), aged_value);
                evidence.push(ReliabilityAppliedParameter {
                    device: binding.device.clone(),
                    compact_model: overlay.selected_model.clone(),
                    parameter,
                    update,
                    shift,
                    fresh_value,
                    aged_value,
                });
            }
            aged.ast_overlay
                .instance_models
                .insert(binding.device.to_ascii_uppercase(), overlay);
        }
        Ok((aged, evidence))
    }
}
