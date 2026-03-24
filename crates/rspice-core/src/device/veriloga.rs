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
        for device in &mut self.devices {
            device.stamp(circuit_voltages, &mut matrix_add, &mut rhs_add);
        }
    }

    /// Get total number of internal nodes across all devices
    pub fn total_internal_nodes(&self) -> usize {
        self.devices.iter().map(|d| d.num_internal_nodes()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_model() -> CompiledModel {
        use rspice_veriloga::codegen::{BytecodeProgram, Instruction, StampProgram};

        let value_program = BytecodeProgram {
            instructions: vec![
                Instruction::PushParam(0),      // G
                Instruction::PushVoltage(0, 1), // V(p,n)
                Instruction::Mul,
            ],
        };

        CompiledModel {
            name: "test_resistor".into(),
            num_terminals: 2,
            terminal_names: vec!["p".into(), "n".into()],
            parameters: vec![CompiledParameter {
                name: "g".into(),
                default: 0.001,
                min: Some(0.0),
                max: None,
            }],
            num_variables: 0,
            assignment_programs: Vec::new(),
            stamp_programs: vec![StampProgram {
                stamp_locations: vec![
                    StampLocation {
                        row: StampIndex::Terminal(0),
                        col: StampIndex::Ground,
                        sign: 1.0,
                    },
                    StampLocation {
                        row: StampIndex::Terminal(1),
                        col: StampIndex::Ground,
                        sign: -1.0,
                    },
                ],
                value_program,
                jacobian_programs: vec![],
            }],
            lookup_tables: Vec::new(),
            internal_nodes: 0,
            branch_currents: 0,
            laplace_filters: Vec::new(),
        }
    }

    #[test]
    fn test_veriloga_devices_collection() {
        let mut devices = VerilogADevices::new();
        assert!(devices.is_empty());

        let model = create_test_model();
        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.add(VerilogADevice::new("R2", model, &[2, 1]));

        assert_eq!(devices.len(), 2);
        assert!(!devices.is_empty());
    }

    #[test]
    fn test_find_device_by_name() {
        let mut devices = VerilogADevices::new();
        let model = create_test_model();

        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.add(VerilogADevice::new("R2", model, &[2, 1]));

        assert!(devices.find_by_name("R1").is_some());
        assert!(devices.find_by_name("R2").is_some());
        assert!(devices.find_by_name("R3").is_none());
    }

    #[test]
    fn test_set_temperature_all() {
        let mut devices = VerilogADevices::new();
        let model = create_test_model();

        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.add(VerilogADevice::new("R2", model, &[2, 1]));

        devices.set_temperature(350.0);
        // Verifies doesn't panic and applies to all devices
    }

    #[test]
    fn test_stamp_all() {
        let mut devices = VerilogADevices::new();
        let model = create_test_model();

        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.get_mut(0).unwrap().set_parameter("g", 0.01);

        let voltages = vec![5.0, 3.0];
        devices.update_all_voltages(&voltages);

        let mut matrix_entries = Vec::new();
        let mut rhs_entries = Vec::new();

        devices.stamp_all(
            &voltages,
            |row, col, val| matrix_entries.push((row, col, val)),
            |node, val| rhs_entries.push((node, val)),
        );

        // Should have RHS contributions from the resistor
        assert!(!rhs_entries.is_empty());
    }

    #[test]
    fn test_device_ext_trait() {
        let model = create_test_model();
        let device = VerilogADevice::new("R1", model, &[1, 0]);

        assert_eq!(device.total_nodes(), 2); // 2 terminals, 0 internal
    }

    #[test]
    fn test_total_internal_nodes() {
        let devices = VerilogADevices::new();
        assert_eq!(devices.total_internal_nodes(), 0);
    }

    #[test]
    fn test_compile_verilog_a_source() {
        // End-to-end: compile Verilog-A source through wrapper
        // Note: We define disciplines inline since include isn't processed
        let source = r#"
            nature electrical;
                units = "V";
                access = V;
                abstol = 1e-12;
            endnature
            
            nature current;
                units = "A";
                access = I;
                abstol = 1e-12;
            endnature
            
            discipline electrical;
                potential electrical;
                flow current;
            enddiscipline
            
            module test_resistor(p, n);
                inout p, n;
                electrical p, n;
                parameter real g = 0.001 from (0:inf);
                analog I(p, n) <+ g * V(p, n);
            endmodule
        "#;

        let compiler = Compiler::default();
        let result = compiler.compile(source);

        assert!(
            result.is_ok(),
            "Compilation should succeed: {:?}",
            result.err()
        );
        let model = result.unwrap();
        assert_eq!(model.num_terminals, 2);
        assert_eq!(model.name.as_str(), "test_resistor");
    }

    #[test]
    fn test_get_device_by_index() {
        let mut devices = VerilogADevices::new();
        let model = create_test_model();

        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.add(VerilogADevice::new("R2", model, &[2, 1]));

        assert!(devices.get(0).is_some());
        assert!(devices.get(1).is_some());
        assert!(devices.get(2).is_none());

        // Mutable access
        let dev = devices.get_mut(0).unwrap();
        dev.set_parameter("g", 0.02);
    }

    #[test]
    fn test_device_iterator() {
        let mut devices = VerilogADevices::new();
        let model = create_test_model();

        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.add(VerilogADevice::new("R2", model, &[2, 1]));

        let names: Vec<_> = devices.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, vec!["R1", "R2"]);
    }

    #[test]
    fn test_set_time_all() {
        let mut devices = VerilogADevices::new();
        let model = create_test_model();

        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.add(VerilogADevice::new("R2", model, &[2, 1]));

        // Set time for transient analysis
        devices.set_time(1e-6);
        // Verifies method doesn't panic
    }
}
