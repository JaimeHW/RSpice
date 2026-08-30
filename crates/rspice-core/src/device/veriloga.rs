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

    // Device types
    device::VerilogADevice,
    device::VerilogADeviceCheckpoint,
    error::CompileError,

    // VM context for advanced usage
    vm::{VerilogAEvaluationMode, VmContext},
};

use crate::Value;

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
    ) -> Result<(), String>;

    /// Checked stamping with an explicit named-limiter evaluation policy.
    fn try_stamp_into_matrix_with_mode(
        &mut self,
        circuit_voltages: &[Value],
        matrix_add: impl FnMut(usize, usize, Value),
        rhs_add: impl FnMut(usize, Value),
        mode: VerilogAEvaluationMode,
    ) -> Result<(), String>;

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
    ) -> Result<(), String> {
        self.try_stamp(circuit_voltages, matrix_add, rhs_add)
            .map_err(|err| format!("Verilog-A device '{}' stamping failed: {err}", self.name))
    }

    fn try_stamp_into_matrix_with_mode(
        &mut self,
        circuit_voltages: &[Value],
        matrix_add: impl FnMut(usize, usize, Value),
        rhs_add: impl FnMut(usize, Value),
        mode: VerilogAEvaluationMode,
    ) -> Result<(), String> {
        self.try_stamp_with_mode(circuit_voltages, matrix_add, rhs_add, mode)
            .map_err(|err| format!("Verilog-A device '{}' stamping failed: {err}", self.name))
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
    ) -> Result<(), String>
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
    use super::{Compiler, VerilogADevice, VerilogADevices};

    #[test]
    fn multi_device_acceptance_validates_all_before_mutating_any_instance() {
        let source = r#"
`include "disciplines.vams"
module atomic_zi(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ zi_nd(V(p, n), {1.0}, {1.0}, 1.0e-6, 0.0);
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
