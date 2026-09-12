//! Verilog-A Device Integration
//!
//! This module provides integration between the rspice-veriloga compiler
//! and the rspice-core simulation engine. It re-exports the key types
//! and provides adapter methods for circuit simulation.
//!
//! # Feature Gating
//!
//! This module is only available when the `veriloga` feature is enabled.
//!
//! # Usage
//!
//! ```ignore
//! use rspice_core::device::veriloga::{VerilogADevice, CompiledModel, Compiler};
//!
//! // Compile a Verilog-A model
//! let compiler = Compiler::new();
//! let model = compiler.compile(source_code)?;
//!
//! // Create device instance
//! let device = VerilogADevice::new("D1", model, &[1, 0]);
//! ```

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

/// Adapter trait for integrating VerilogADevice with the simulation engine
pub trait VerilogADeviceExt {
    /// Stamp the device into the circuit matrix
    ///
    /// This adapts the Verilog-A device stamping to the rspice-core matrix format.
    fn stamp_into_matrix(
        &mut self,
        circuit_voltages: &[Value],
        matrix_add: impl FnMut(usize, usize, Value),
        rhs_add: impl FnMut(usize, Value),
    );

    /// Checked stamping variant for solver paths that can return diagnostics.
    fn try_stamp_into_matrix(
        &mut self,
        circuit_voltages: &[Value],
        matrix_add: impl FnMut(usize, usize, Value),
        rhs_add: impl FnMut(usize, Value),
    ) -> Result<(), StampError>;

    /// Checked stamping with an explicit named-limiter evaluation policy.
    fn try_stamp_into_matrix_with_mode(
        &mut self,
        circuit_voltages: &[Value],
        matrix_add: impl FnMut(usize, usize, Value),
        rhs_add: impl FnMut(usize, Value),
        mode: VerilogAEvaluationMode,
    ) -> Result<(), StampError>;

    /// Get the total number of nodes (terminals + internal)
    fn total_nodes(&self) -> usize;
}

impl VerilogADeviceExt for VerilogADevice {
    fn stamp_into_matrix(
        &mut self,
        circuit_voltages: &[Value],
        matrix_add: impl FnMut(usize, usize, Value),
        rhs_add: impl FnMut(usize, Value),
    ) {
        self.stamp(circuit_voltages, matrix_add, rhs_add);
    }

    fn try_stamp_into_matrix(
        &mut self,
        circuit_voltages: &[Value],
        matrix_add: impl FnMut(usize, usize, Value),
        rhs_add: impl FnMut(usize, Value),
    ) -> Result<(), StampError> {
        match self.try_stamp(circuit_voltages, matrix_add, rhs_add) {
            Ok(()) => Ok(()),
            Err(error) => Err(stamp_failure(self, circuit_voltages, &error)),
        }
    }

    fn try_stamp_into_matrix_with_mode(
        &mut self,
        circuit_voltages: &[Value],
        matrix_add: impl FnMut(usize, usize, Value),
        rhs_add: impl FnMut(usize, Value),
        mode: VerilogAEvaluationMode,
    ) -> Result<(), StampError> {
        match self.try_stamp_with_mode(circuit_voltages, matrix_add, rhs_add, mode) {
            Ok(()) => Ok(()),
            Err(error) => Err(stamp_failure(self, circuit_voltages, &error)),
        }
    }

    fn total_nodes(&self) -> usize {
        self.num_terminals() + self.num_internal_nodes()
    }
}

/// Collection of Verilog-A devices in a circuit
///
/// This provides efficient storage and stamping for multiple Verilog-A devices.
#[derive(Debug, Clone, Default)]
pub struct VerilogADevices {
    /// Device instances
    devices: Vec<VerilogADevice>,
}

impl VerilogADevices {
    /// Create an empty collection
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
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
        for device in &mut self.devices {
            device.try_stamp_into_matrix_with_mode(
                circuit_voltages,
                &mut matrix_add,
                &mut rhs_add,
                mode,
            )?;
        }
        Ok(())
    }

    /// Whether every device's named limiters accepted their proposal during
    /// the latest limited Newton stamp.
    #[inline]
    pub fn all_converged(&self) -> bool {
        self.devices.iter().all(VerilogADevice::limiter_converged)
    }

    /// Remap all terminal and internal circuit node IDs after topology changes.
    pub fn remap_circuit_nodes(&mut self, mut remap: impl FnMut(usize) -> usize) {
        for device in &mut self.devices {
            device.remap_circuit_nodes(&mut remap);
        }
    }
}

#[cfg(all(test, feature = "veriloga", not(feature = "veriloga-native")))]
mod checkpoint_tests {
    use super::{Compiler, VerilogADevice, VerilogADeviceExt, VerilogADevices};

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
        let stamp_error = device
            .try_stamp_into_matrix(&[-1.0], |_, _, _| {}, |_, _| {})
            .expect_err("ln() leaves its domain at V(p,n) = -1 V");
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
