use std::collections::HashMap;

use super::EngineBridge;
use crate::simulation::config::DcSweepConfig;
use crate::simulation::results::{DcOpResult, SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Run DC operating point analysis.
    pub(super) fn run_dc_op(
        &self,
        netlist: &rspice_core::Netlist,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let (core_result, device_report) = engine
            .run_dc_op_with_report(netlist)
            .map_err(|e| self.translate_error(e))?;

        let mut result = convert_dc_result(&core_result);
        if !device_report.is_empty() {
            result.device_report = Some(device_report);
        }
        Ok(SimulationResult::DcOp(result))
    }

    /// Run DC sweep analysis.
    pub(super) fn run_dc_sweep(
        &self,
        netlist: &rspice_core::Netlist,
        config: &DcSweepConfig,
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let nested_cfg = nested_dc_sweep_config(config)?;
        let mut sweep_values = Vec::new();
        let mut waveforms = HashMap::new();

        if let Some((source2, start2, stop2, step2)) = nested_cfg {
            let sweep2 =
                rspice_core::analysis::DcSweep::new(source2.to_string(), start2, stop2, step2);
            let sweep2_values = sweep2.points();
            if sweep2_values.is_empty() {
                return Err(SimulationError::InvalidConfig(
                    "Nested DC secondary sweep produced no points".to_string(),
                ));
            }

            for &sweep2_value in &sweep2_values {
                let mut nested_netlist = netlist.clone();
                set_dc_source_value(&mut nested_netlist, source2, sweep2_value)?;

                let sweep_results = engine
                    .run_dc_sweep(
                        &nested_netlist,
                        &config.source,
                        config.start,
                        config.stop,
                        config.step,
                    )
                    .map_err(|e| self.translate_error(e))?;

                if sweep_results.is_empty() {
                    continue;
                }

                if sweep_values.is_empty() {
                    sweep_values = sweep_results.iter().map(|(v, _)| *v).collect();
                }

                if let Some((_, first_result)) = sweep_results.first() {
                    for (node_idx, node_name) in first_result.node_names.iter().enumerate() {
                        if node_idx == 0 {
                            continue;
                        }
                        let voltages: Vec<f64> = sweep_results
                            .iter()
                            .map(|(_, result)| {
                                result.node_voltages.get(node_idx).copied().unwrap_or(0.0)
                            })
                            .collect();
                        let trace_name = format!("{} [{}={:.6}]", node_name, source2, sweep2_value);
                        waveforms.insert(
                            trace_name.clone(),
                            WaveformData::new_time_domain(
                                trace_name,
                                sweep_values.clone(),
                                voltages,
                            ),
                        );
                    }
                }
            }
        } else {
            let sweep_results = engine
                .run_dc_sweep(
                    netlist,
                    &config.source,
                    config.start,
                    config.stop,
                    config.step,
                )
                .map_err(|e| self.translate_error(e))?;

            sweep_values = sweep_results.iter().map(|(v, _)| *v).collect();

            if let Some((_, first_result)) = sweep_results.first() {
                for (i, name) in first_result.node_names.iter().enumerate() {
                    if i == 0 {
                        continue;
                    }
                    let voltages: Vec<f64> = sweep_results
                        .iter()
                        .map(|(_, result)| result.node_voltages.get(i).copied().unwrap_or(0.0))
                        .collect();

                    waveforms.insert(
                        name.clone(),
                        WaveformData::new_time_domain(name, sweep_values.clone(), voltages),
                    );
                }
            }
        }

        let measurements =
            super::measure::evaluate_measurements(netlist, "DC", &sweep_values, &waveforms);
        Ok(SimulationResult::DcSweep {
            sweep_var: config.source.clone(),
            sweep_values,
            waveforms,
            measurements,
        })
    }
}

fn convert_dc_result(core_result: &rspice_core::SimulationResult) -> DcOpResult {
    let mut result = DcOpResult::default();

    for (i, &voltage) in core_result.node_voltages.iter().enumerate() {
        if i > 0 {
            let name = core_result
                .node_names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("{}", i));
            result.node_voltages.insert(name, voltage);
        }
    }

    for (i, &current) in core_result.branch_currents.iter().enumerate() {
        let name = dc_branch_waveform_name(core_result, i);
        result.branch_currents.insert(name, current);
    }

    result
}

fn dc_branch_waveform_name(result: &rspice_core::SimulationResult, branch_idx: usize) -> String {
    result
        .branch_names
        .get(branch_idx)
        .filter(|name| !name.is_empty())
        .map(|name| format!("I({})", name))
        .unwrap_or_else(|| format!("I({})", branch_idx + 1))
}

fn nested_dc_sweep_config(
    config: &DcSweepConfig,
) -> Result<Option<(&str, f64, f64, f64)>, SimulationError> {
    match (&config.source2, config.start2, config.stop2, config.step2) {
        (None, None, None, None) => Ok(None),
        (Some(source2), Some(start2), Some(stop2), Some(step2)) => {
            Ok(Some((source2.as_str(), start2, stop2, step2)))
        }
        _ => Err(SimulationError::InvalidConfig(
            "Nested DC sweep requires source2/start2/stop2/step2".to_string(),
        )),
    }
}

fn set_dc_source_value(
    netlist: &mut rspice_core::Netlist,
    source_name: &str,
    value: f64,
) -> Result<(), SimulationError> {
    if source_name.trim().is_empty() {
        return Err(SimulationError::InvalidConfig(
            "DC sweep source name cannot be empty".to_string(),
        ));
    }

    for element in &mut netlist.elements {
        if !element.name.eq_ignore_ascii_case(source_name) {
            continue;
        }
        if let rspice_core::netlist::ElementKind::VoltageSource(spec) = &mut element.kind {
            if set_source_spec_dc(spec, value) {
                return Ok(());
            }
            return Err(SimulationError::InvalidConfig(format!(
                "Source '{}' is not a DC or DC/AC voltage source",
                source_name
            )));
        }
    }

    Err(SimulationError::InvalidConfig(format!(
        "Source '{}' not found in netlist",
        source_name
    )))
}

fn set_source_spec_dc(spec: &mut rspice_core::netlist::SourceSpec, value: f64) -> bool {
    match spec {
        rspice_core::netlist::SourceSpec::Dc(v) => {
            *v = value;
            true
        }
        rspice_core::netlist::SourceSpec::DcAc { dc_value, .. } => {
            *dc_value = value;
            true
        }
        _ => false,
    }
}
