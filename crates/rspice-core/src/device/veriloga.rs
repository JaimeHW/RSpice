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
    error::CompileError,

    // VM context for advanced usage
    vm::VmContext,
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

    /// Find device by name
    pub fn find_by_name(&self, name: &str) -> Option<&VerilogADevice> {
        self.devices.iter().find(|d| d.name.as_str() == name)
    }

    /// Iterate over devices
    pub fn iter(&self) -> impl Iterator<Item = &VerilogADevice> {
        self.devices.iter()
    }

    /// Iterate mutably over devices
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut VerilogADevice> {
        self.devices.iter_mut()
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

    /// Stamp all devices into matrix and RHS
    pub fn stamp_all<M, R>(&mut self, circuit_voltages: &[Value], mut matrix_add: M, mut rhs_add: R)
    where
        M: FnMut(usize, usize, Value),
        R: FnMut(usize, Value),
    {
        if let Err(err) = self.try_stamp_all(circuit_voltages, &mut matrix_add, &mut rhs_add) {
            panic!("{err}");
        }
    }

    /// Checked variant of [`Self::stamp_all`] for simulator paths that can
    /// return model diagnostics instead of unwinding.
    pub fn try_stamp_all<M, R>(
        &mut self,
        circuit_voltages: &[Value],
        mut matrix_add: M,
        mut rhs_add: R,
    ) -> Result<(), String>
    where
        M: FnMut(usize, usize, Value),
        R: FnMut(usize, Value),
    {
        for device in &mut self.devices {
            device.try_stamp_into_matrix(circuit_voltages, &mut matrix_add, &mut rhs_add)?;
        }
        Ok(())
    }

    /// Get total number of internal nodes across all devices
    pub fn total_internal_nodes(&self) -> usize {
        self.devices.iter().map(|d| d.num_internal_nodes()).sum()
    }

    /// Remap all terminal and internal circuit node IDs after topology changes.
    pub fn remap_circuit_nodes(&mut self, mut remap: impl FnMut(usize) -> usize) {
        for device in &mut self.devices {
            device.remap_circuit_nodes(&mut remap);
        }
    }
}
