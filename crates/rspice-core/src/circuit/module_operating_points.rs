//! The operating point of each runtime-compiled Verilog-A instance, named by
//! its module.
//!
//! An instance reports the current entering each of its ports and every
//! module variable its source describes as an output (`(* desc="…",
//! units="…" *) real gm;`), all from one evaluation of the module at the
//! solution, without limiting.

use super::*;
use crate::device::veriloga::{VerilogADevice, VerilogAEvaluationMode};
use rspice_veriloga::canonical_ir::{CanonicalIrArtifact, HirOutputScaling};

impl CircuitData {
    /// Append an entry for every runtime Verilog-A instance to `report`.
    pub(super) fn add_runtime_veriloga_operating_points(
        &self,
        report: &mut DeviceOpReport,
        solution: &[Value],
    ) -> Result<(), String> {
        for device in self.veriloga_devices.iter() {
            let artifact = self
                .veriloga_instance_modules
                .get(&device.name.to_ascii_lowercase())
                .map(std::sync::Arc::as_ref);
            let module = module_operating_point(device, artifact, solution).map_err(|error| {
                format!(
                    "the operating point of the Verilog-A instance '{}' could not be read: \
                     {error}",
                    device.name
                )
            })?;
            report
                .entries
                .push(DeviceOpEntry::module(device.name.to_string(), module));
        }
        Ok(())
    }
}

/// The units a terminal current is reported with, unless the port's
/// discipline gives its flow other units (`W` on a thermal port).
const TERMINAL_CURRENT_UNITS: &str = "A";

/// Evaluate `device` once at `solution` and read its terminal currents and
/// outputs.
///
/// The evaluation runs on a copy whose terminals each sit on a row of their
/// own past the circuit's, carrying their net's value, so the sum along a
/// terminal's row is the current entering that terminal.
fn module_operating_point(
    device: &VerilogADevice,
    artifact: Option<&CanonicalIrArtifact>,
    solution: &[Value],
) -> Result<ModuleOpReport, String> {
    let rows = solution.len();
    let terminals = device.num_terminals();
    let net_value = |node: usize| {
        node.checked_sub(1)
            .map_or(Some(0.0), |row| solution.get(row).copied())
    };
    let mut at = solution.to_vec();
    for terminal in 0..terminals {
        at.push(
            net_value(device.node_for_terminal(terminal))
                .ok_or("a terminal's net is outside the solution")?,
        );
    }
    let nodes: Vec<usize> = (0..terminals).map(|terminal| rows + 1 + terminal).collect();
    let mut probe = device
        .try_with_terminal_nodes(&nodes)
        .map_err(|error| error.to_string())?;
    // A row's conductances times the evaluation point, less its source.
    let mut conducted = vec![0.0; terminals];
    let mut sourced = vec![0.0; terminals];
    probe
        .try_stamp_with_mode(
            &at,
            |row, column, value| {
                if let Some(current) = row.checked_sub(rows).and_then(|t| conducted.get_mut(t)) {
                    *current += value * at.get(column).copied().unwrap_or(0.0);
                }
            },
            |row, value| {
                if let Some(source) = row.checked_sub(rows).and_then(|t| sourced.get_mut(t)) {
                    *source += value;
                }
            },
            VerilogAEvaluationMode::StaticProbe,
        )
        .map_err(|error| error.to_string())?;
    let currents = conducted
        .into_iter()
        .zip(sourced)
        .map(|(conducted, sourced)| conducted - sourced);

    let mut outputs = Vec::new();
    if let Some(artifact) = artifact {
        probe
            .observe_variables(artifact)
            .map_err(|error| error.to_string())?;
        let multiplicity = device.multiplicity();
        for variable in &artifact.hir.variables {
            let Some(output) = &variable.output else {
                continue;
            };
            let Some(value) = probe.variable(&variable.name) else {
                continue;
            };
            let value = match output.scaling {
                HirOutputScaling::Unscaled => value,
                HirOutputScaling::Multiply => value * multiplicity,
                HirOutputScaling::Divide => value / multiplicity,
            };
            outputs.push(ModuleOpQuantity {
                name: variable.name.to_string(),
                value,
                units: output.units.to_string(),
                description: output.description.to_string(),
            });
        }
    }

    // Ports the card connects, not the hidden terminals that bind a module's
    // own nets to deck nodes. A module output of the same name as a port's
    // current label is the module's, and keeps the name.
    let declared =
        terminals.saturating_sub(artifact.map_or(0, |artifact| artifact.hir.node_aliases.len()));
    let flow_units = |terminal: usize| {
        artifact
            .and_then(|artifact| artifact.hir.ports.get(terminal))
            .and_then(|port| port.flow_units.as_deref())
    };
    let mut quantities: Vec<ModuleOpQuantity> = device
        .terminal_names()
        .iter()
        .take(declared)
        .zip(currents)
        .enumerate()
        .map(|(terminal, (port, value))| {
            let (units, description) = match flow_units(terminal) {
                Some(units) => (units.to_owned(), format!("Flow into terminal {port}")),
                None => (
                    TERMINAL_CURRENT_UNITS.to_owned(),
                    format!("Current into terminal {port}"),
                ),
            };
            ModuleOpQuantity {
                name: terminal_current_label(port),
                value,
                units,
                description,
            }
        })
        .filter(|current| {
            !outputs
                .iter()
                .any(|output| output.name.eq_ignore_ascii_case(&current.name))
        })
        .collect();
    quantities.extend(outputs);
    Ok(ModuleOpReport {
        module: device.model_name().to_owned(),
        quantities,
    })
}

/// The name a port's current is read by: `i` and the port name, lower case,
/// as `@x1[iout]` spells it.
pub(crate) fn terminal_current_label(port: &str) -> String {
    format!("i{}", port.to_ascii_lowercase())
}
