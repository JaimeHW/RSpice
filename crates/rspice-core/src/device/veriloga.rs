//! Verilog-A Device Integration
//!
//! This module provides integration between the rspice-veriloga compiler
//! and the rspice-core simulation engine. It re-exports the key types
//! and provides adapter methods for circuit simulation.
//!
//! # Feature Gating
//!
//! This module is only available when the `veriloga` feature is enabled.

// Re-export core types from rspice-veriloga
pub use rspice_veriloga::{
    CompileResult,
    CompilerOptions,
    // Compiler infrastructure
    VerilogACompiler as Compiler,
    // Compiled model
    codegen::CompiledModel,
    codegen::CompiledParameter,
    codegen::StampIndex,
    codegen::StampLocation,

    device::DeviceBuilder,
    device::JacobianEntry,
    // One `ddt` operand's charge and accepted history, as the transient step
    // controller's charge-truncation walk consumes them.
    device::RuntimeDynamicCharge,

    // Device types
    device::VerilogADevice,
    device::VerilogADeviceCheckpoint,
    error::CompileError,

    // VM context for advanced usage
    vm::{VerilogAEvaluationMode, VmContext},
};

use crate::Value;
use crate::device::StampError;

/// One instance's terminal voltages at the iterate the solver handed it.
///
/// The iterate itself is finite — it is the module's own arithmetic that left
/// its domain — so these are the numbers a user needs to see to understand why
/// the evaluation failed.
fn terminal_iterate_summary(device: &VerilogADevice, circuit_voltages: &[Value]) -> String {
    let terminals = device
        .terminal_names()
        .iter()
        .enumerate()
        .map(|(terminal, name)| {
            // Device node ids are 1-based over the MNA solution; 0 is ground.
            let node = device.node_for_terminal(terminal);
            let voltage = if node == 0 {
                Some(0.0)
            } else {
                circuit_voltages.get(node - 1).copied()
            };
            match voltage {
                Some(voltage) => format!("{name}={voltage:.6e}"),
                None => format!("{name}=<unmapped>"),
            }
        })
        .collect::<Vec<_>>();
    format!("[{}]", terminals.join(", "))
}

/// Classify one stamping failure: a non-finite evaluation is a rejectable
/// trial, so the Newton loops can cut dt or step the sources instead of ending
/// the run. Every other fault is structural and keeps its wording.
///
/// `InvalidNumericResult` is the compiler's non-finite-arithmetic variant, and
/// only that. The refusals that share its shape without depending on the
/// iterate — a Zi layout the runtime will not execute, an index that rounds
/// outside the representable range — carry `InvalidRuntimeOperation` and fall
/// into the structural arm with everything else.
fn stamp_failure(
    device: &VerilogADevice,
    circuit_voltages: &[Value],
    error: &rspice_veriloga::vm::VmError,
) -> StampError {
    match error {
        rspice_veriloga::vm::VmError::InvalidNumericResult(detail) => StampError::nonfinite_trial(
            device.name.clone(),
            format!(
                "Verilog-A device '{}' produced a non-finite value at a trial iterate {}: {detail}",
                device.name,
                terminal_iterate_summary(device, circuit_voltages),
            ),
        ),
        other => StampError::Structural(format!(
            "Verilog-A device '{}' stamping failed: {other}",
            device.name
        )),
    }
}

/// Collection of Verilog-A devices in a circuit
///
/// This provides efficient storage and stamping for multiple Verilog-A devices.
#[derive(Debug, Clone, Default)]
pub struct VerilogADevices {
    /// Device instances
    devices: Vec<VerilogADevice>,
    /// The linear model each device's latest limited Newton stamp put on its
    /// terminal rows, one per device; empty after any other kind of stamp.
    newton_models: Vec<TerminalCurrentModel>,
    /// Whether the operating-point iterate solved from those stamps moves some
    /// terminal current by more than the tolerance allows.
    newton_currents_unsettled: bool,
}

/// One device's limited Newton stamp as it bears on its terminal currents.
///
/// A terminal row's stamp is the device's own linearization: its conductances
/// times the stamp point, less the equivalent source, is the current the
/// device carried there, whatever the conductances are. The current the next
/// iterate predicts differs from it by the conductances times the step. The
/// iterate has settled the device when that predicted change is within
/// tolerance of the current, the test SPICE devices apply to their branch
/// currents. It is what keeps a solution off a Jacobian that disagrees with
/// the currents: Newton then steps to wherever that Jacobian says KCL holds,
/// which predicts a change as large as the current it fails to balance.
#[derive(Debug, Clone, Default)]
struct TerminalCurrentModel {
    /// Each terminal row with the current the device carried on it.
    rows: Vec<(usize, Value)>,
    /// Each conductance on a terminal row: its row's slot in `rows`, its
    /// column, its value and the column's value at the stamp point.
    conductances: Vec<(usize, usize, Value, Value)>,
    /// The stamp's raw matrix entries and sources, empty between stamps.
    entries: Vec<(usize, usize, Value)>,
    sources: Vec<(usize, Value)>,
}

impl TerminalCurrentModel {
    /// Reduce the recorded stamp to the device's terminal rows at the point
    /// it was stamped at. `terminal_rows` are matrix rows (ground excluded).
    fn reduce(&mut self, terminal_rows: impl IntoIterator<Item = usize>, at: &[Value]) {
        self.rows.clear();
        self.conductances.clear();
        for row in terminal_rows {
            if !self.rows.iter().any(|&(known, _)| known == row) {
                self.rows.push((row, 0.0));
            }
        }
        for &(row, column, value) in &self.entries {
            let Some(slot) = self.rows.iter().position(|&(known, _)| known == row) else {
                continue;
            };
            let column_value = at.get(column).copied().unwrap_or(0.0);
            self.rows[slot].1 += value * column_value;
            self.conductances.push((slot, column, value, column_value));
        }
        for &(row, value) in &self.sources {
            if let Some(slot) = self.rows.iter().position(|&(known, _)| known == row) {
                self.rows[slot].1 -= value;
            }
        }
        self.entries.clear();
        self.sources.clear();
    }

    /// Whether moving to `solution` changes every terminal current by no more
    /// than the device tolerance allows.
    fn settled(
        &self,
        solution: &[Value],
        criteria: crate::device::NonlinearConvergenceCriteria,
    ) -> bool {
        let mut change = vec![0.0; self.rows.len()];
        for &(slot, column, value, column_value) in &self.conductances {
            let Some(&next) = solution.get(column) else {
                return false;
            };
            change[slot] += value * (next - column_value);
        }
        self.rows
            .iter()
            .zip(&change)
            .all(|(&(_, current), &change)| {
                let tolerance = criteria.current_tolerance()
                    + criteria.relative_tolerance() * current.abs().max((current + change).abs());
                change.abs() <= tolerance
            })
    }
}

impl VerilogADevices {
    /// Create an empty collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a device to the collection
    pub fn add(&mut self, device: VerilogADevice) {
        self.devices.push(device);
    }

    /// Get the number of devices
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Get a device by index
    pub fn get(&self, index: usize) -> Option<&VerilogADevice> {
        self.devices.get(index)
    }

    /// Get a mutable device by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut VerilogADevice> {
        self.devices.get_mut(index)
    }

    /// Iterate over devices
    pub fn iter(&self) -> impl Iterator<Item = &VerilogADevice> {
        self.devices.iter()
    }

    /// Iterate mutably over devices
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut VerilogADevice> {
        self.devices.iter_mut()
    }

    pub(crate) fn checkpoint_states(&self) -> Result<Vec<VerilogADeviceCheckpoint>, String> {
        self.devices
            .iter()
            .map(|device| {
                device.checkpoint_state().map_err(|error| {
                    format!(
                        "Verilog-A device '{}' checkpoint capture failed: {error}",
                        device.name
                    )
                })
            })
            .collect()
    }

    pub(crate) fn validate_checkpoint_states(
        &self,
        states: &[VerilogADeviceCheckpoint],
    ) -> Result<(), String> {
        if states.len() != self.devices.len() {
            return Err(format!(
                "runtime Verilog-A checkpoint shape mismatch: captured {}, circuit has {}",
                states.len(),
                self.devices.len()
            ));
        }
        for (index, (device, state)) in self.devices.iter().zip(states).enumerate() {
            device.validate_checkpoint_state(state).map_err(|error| {
                format!(
                    "runtime Verilog-A checkpoint instance {index} ('{}') is invalid: {error}",
                    device.name
                )
            })?;
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint_states(
        &mut self,
        states: &[VerilogADeviceCheckpoint],
    ) -> Result<(), String> {
        self.validate_checkpoint_states(states)?;
        for (device, state) in self.devices.iter_mut().zip(states) {
            device.apply_validated_checkpoint_state(state);
        }
        Ok(())
    }

    pub(crate) fn validate_timestep_acceptance(&self) -> Result<(), String> {
        for device in &self.devices {
            device.validate_advance_state().map_err(|error| {
                format!(
                    "Verilog-A device '{}' timestep acceptance failed: {error}",
                    device.name
                )
            })?;
        }
        Ok(())
    }

    /// Publish devices whose continuation state and static activation were
    /// prepared without mutating this live collection.
    pub(crate) fn install_prepared_analysis_continuation(&mut self, devices: Vec<VerilogADevice>) {
        debug_assert_eq!(devices.len(), self.devices.len());
        self.devices = devices;
    }

    pub(crate) fn apply_validated_timestep_acceptance(&mut self) {
        for device in &mut self.devices {
            device.apply_validated_advance_state();
        }
    }

    /// Set temperature for all devices
    pub fn set_temperature(&mut self, temp_k: Value) {
        for device in &mut self.devices {
            device.set_temperature(temp_k);
        }
    }

    /// Set time for all devices (transient analysis)
    pub fn set_time(&mut self, time: Value) {
        for device in &mut self.devices {
            device.set_time(time);
        }
    }

    /// Update voltages for all devices
    pub fn update_all_voltages(&mut self, circuit_voltages: &[Value]) {
        for device in &mut self.devices {
            device.update_all_voltages(circuit_voltages);
        }
    }

    /// Stamp every device with an explicit named-limiter evaluation policy.
    pub fn try_stamp_all_with_mode<M, R>(
        &mut self,
        circuit_voltages: &[Value],
        mut matrix_add: M,
        mut rhs_add: R,
        mode: VerilogAEvaluationMode,
    ) -> Result<(), StampError>
    where
        M: FnMut(usize, usize, Value),
        R: FnMut(usize, Value),
    {
        self.newton_currents_unsettled = false;
        if mode != VerilogAEvaluationMode::NewtonLimited {
            self.newton_models.clear();
            for device in &mut self.devices {
                device
                    .try_stamp_with_mode(circuit_voltages, &mut matrix_add, &mut rhs_add, mode)
                    .map_err(|error| stamp_failure(device, circuit_voltages, &error))?;
            }
            return Ok(());
        }
        self.newton_models
            .resize_with(self.devices.len(), TerminalCurrentModel::default);
        for (device, model) in self.devices.iter_mut().zip(&mut self.newton_models) {
            let TerminalCurrentModel {
                entries, sources, ..
            } = model;
            device
                .try_stamp_with_mode(
                    circuit_voltages,
                    |row, column, value| {
                        entries.push((row, column, value));
                        matrix_add(row, column, value);
                    },
                    |row, value| {
                        sources.push((row, value));
                        rhs_add(row, value);
                    },
                    mode,
                )
                .map_err(|error| stamp_failure(device, circuit_voltages, &error))?;
            let terminal_rows = (0..device.num_terminals())
                .filter_map(|terminal| device.node_for_terminal(terminal).checked_sub(1));
            model.reduce(terminal_rows, circuit_voltages);
        }
        Ok(())
    }

    /// Judge the operating-point iterate `solution` solved from the latest
    /// limited Newton stamps by the terminal currents those stamps predict
    /// for it; [`Self::all_converged`] reports the verdict until the next
    /// stamp.
    pub(crate) fn settle_newton_currents(
        &mut self,
        solution: &[Value],
        criteria: crate::device::NonlinearConvergenceCriteria,
    ) {
        self.newton_currents_unsettled = !self
            .newton_models
            .iter()
            .all(|model| model.settled(solution, criteria));
    }

    /// Whether every device's named limiters accepted their proposal during
    /// the latest limited Newton stamp, and the operating-point iterate
    /// solved from it settled every terminal current.
    #[inline]
    pub fn all_converged(&self) -> bool {
        !self.newton_currents_unsettled
            && self.devices.iter().all(VerilogADevice::limiter_converged)
    }
}

#[cfg(all(test, feature = "veriloga", not(feature = "veriloga-native")))]
mod checkpoint_tests {
    use super::{Compiler, VerilogADevice, VerilogADevices};

    /// The classification the Newton loops read has to survive the trip
    /// through `SimulationError::Circuit`, and it must carry the iterate: a
    /// bare "invalid numeric result" tells nobody which instance overshot.
    #[test]
    fn a_non_finite_contribution_reports_a_rejectable_trial_with_its_iterate() {
        let source = r#"
module domain_edge(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 1.0e-3 * ln(V(p, n) + 0.1);
endmodule
"#;
        let model = Compiler::default()
            .compile(source)
            .expect("compile domain-edge fixture");
        let mut device = VerilogADevice::try_new("x1", model, &[1, 0]).unwrap();
        device.try_set_analysis_type(2).unwrap();
        device.try_set_time(0.0).unwrap();
        device.try_set_timestep(0.0).unwrap();
        let vm_error = device
            .try_stamp(&[-1.0], |_, _, _| {}, |_, _| {})
            .expect_err("ln() leaves its domain at V(p,n) = -1 V");
        let stamp_error = super::stamp_failure(&device, &[-1.0], &vm_error);
        // The classification is a variant now, and it survives the widening to
        // `SimulationError` without anything reading the sentence.
        assert!(matches!(
            stamp_error,
            crate::device::StampError::NonFiniteTrial(_)
        ));
        let error = crate::SimulationError::from(stamp_error);
        let detail = error
            .nonfinite_trial_detail()
            .expect("a non-finite contribution is a rejectable trial, not a fatal fault");
        assert!(detail.contains("x1"), "{detail}");
        assert!(detail.contains("p=-1.000000e0"), "{detail}");
        assert!(detail.contains("n=0.000000e0"), "{detail}");
        assert!(detail.contains("contribution"), "{detail}");
        assert_eq!(
            error.nonfinite_trial().map(|trial| trial.instance.as_str()),
            Some("x1")
        );
    }

    /// A structural fault is not a rejectable trial: retrying it forever would
    /// replace a precise refusal with a convergence failure.
    #[test]
    fn a_structural_stamp_failure_is_not_a_rejectable_trial() {
        let error = crate::SimulationError::from(crate::device::StampError::Structural(
            "Verilog-A device 'x1' stamping failed: invalid compiled model: no such branch".into(),
        ));
        assert!(matches!(error, crate::SimulationError::Circuit(_)));
        assert!(error.nonfinite_trial_detail().is_none());
    }

    /// A runtime refusal that does not depend on the iterate keeps the
    /// structural arm even though it comes out of the same evaluation call as
    /// a domain edge does.
    #[test]
    fn a_structural_runtime_refusal_is_not_a_rejectable_trial() {
        let source = r#"
module domain_edge_structural(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 1.0e-3 * V(p, n);
endmodule
"#;
        let model = Compiler::default()
            .compile(source)
            .expect("compile structural-refusal fixture");
        let device = VerilogADevice::try_new("x1", model, &[1, 0]).unwrap();
        let error = super::stamp_failure(
            &device,
            &[-1.0],
            &rspice_veriloga::vm::VmError::InvalidRuntimeOperation(
                "runtime array index 1e30 rounds outside the signed 64-bit index range".into(),
            ),
        );
        assert!(
            matches!(error, crate::device::StampError::Structural(_)),
            "{error}"
        );
        assert!(error.to_string().contains("stamping failed"), "{error}");
        assert!(error.to_string().contains("array index"), "{error}");
    }

    #[test]
    fn multi_device_acceptance_validates_all_before_mutating_any_instance() {
        let source = r#"
`include "disciplines.vams"
module atomic_zi(p, n);
    inout p, n;
    electrical p, n;
    real sampled;
    analog begin
        sampled = zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6, 0.0);
        I(p, n) <+ sampled;
    end
endmodule
"#;
        let model = Compiler::default()
            .compile(source)
            .expect("compile atomic Zi fixture");
        let mut devices = VerilogADevices::new();
        devices.add(VerilogADevice::try_new("x1", model.clone(), &[1, 0]).unwrap());
        devices.add(VerilogADevice::try_new("x2", model, &[1, 0]).unwrap());
        for device in devices.iter_mut() {
            device.try_set_analysis_type(2).unwrap();
            device.try_set_time(0.0).unwrap();
            device.try_set_timestep(0.0).unwrap();
            device
                .try_stamp(&[0.0, 0.0], |_, _, _| {}, |_, _| {})
                .unwrap();
        }
        devices.validate_timestep_acceptance().unwrap();
        devices.apply_validated_timestep_acceptance();

        for device in devices.iter_mut() {
            device.try_set_time(0.5e-6).unwrap();
            device.try_set_timestep(0.5e-6).unwrap();
        }
        devices
            .get_mut(0)
            .unwrap()
            .try_stamp(&[0.0, 0.0], |_, _, _| {}, |_, _| {})
            .unwrap();
        assert!(
            devices.validate_timestep_acceptance().is_err(),
            "the second active Zi site was not evaluated at the candidate time"
        );

        let first = devices.get_mut(0).unwrap();
        first.try_set_time(0.25e-6).unwrap();
        first.try_set_timestep(0.25e-6).unwrap();
        first
            .try_stamp(&[0.0, 0.0], |_, _, _| {}, |_, _| {})
            .expect("failed validation must leave the first device accepted at t=0");
    }
}

#[cfg(all(test, feature = "veriloga", not(feature = "veriloga-native")))]
mod newton_current_tests {
    use super::{Compiler, VerilogADevice, VerilogADevices, VerilogAEvaluationMode};
    use crate::device::NonlinearConvergenceCriteria;

    const VT: f64 = 0.025;

    fn junction() -> VerilogADevices {
        let source = r#"
module junction(a, c);
    inout a, c;
    electrical a, c;
    analog I(a, c) <+ 1.0e-14 * (limexp(V(a, c) / 0.025) - 1.0);
endmodule
"#;
        let model = Compiler::default()
            .compile(source)
            .expect("compile junction fixture");
        let mut device = VerilogADevice::try_new("x1", model, &[1, 0]).unwrap();
        device.try_set_analysis_type(0).unwrap();
        let mut devices = VerilogADevices::new();
        devices.add(device);
        devices
    }

    /// One Newton step of a 1 mA source driving the junction from `at`, and
    /// whether the collection then calls the operating point converged.
    fn newton_step(devices: &mut VerilogADevices, at: f64) -> (f64, bool) {
        let (mut conductance, mut rhs) = (0.0, 1.0e-3);
        devices
            .try_stamp_all_with_mode(
                &[at],
                |row, column, value| {
                    assert_eq!((row, column), (0, 0));
                    conductance += value;
                },
                |row, value| {
                    assert_eq!(row, 0);
                    rhs += value;
                },
                VerilogAEvaluationMode::NewtonLimited,
            )
            .expect("stamp the junction");
        let next = rhs / conductance;
        devices.settle_newton_currents(&[next], NonlinearConvergenceCriteria::default());
        (next, devices.all_converged())
    }

    /// Far past its knee the junction carries some 1e28 A against the 1 mA
    /// that drives it. No limiter acts there, so only the terminal current
    /// the step predicts can say the iterate is not a solution.
    #[test]
    fn an_iterate_whose_terminal_current_does_not_settle_is_not_converged() {
        let mut devices = junction();
        let (_, converged) = newton_step(&mut devices, 1.0e6);
        assert!(!converged, "a 1e28 A imbalance passed as converged");

        let solution = VT * (1.0e-3_f64 / 1.0e-14 + 1.0).ln();
        let (next, converged) = newton_step(&mut devices, solution);
        assert!(
            (next - solution).abs() <= 1.0e-12,
            "Newton left the solution: {next} vs {solution}"
        );
        assert!(converged, "the solution itself did not settle");
    }
}
