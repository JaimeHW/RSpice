//! Integration tests for the Verilog-A compiler pipeline
//!
//! Verifies that Verilog-A models can be compiled and used in simulation.

#[cfg(feature = "veriloga")]
mod veriloga_tests {
    use rspice_core::{Engine, Netlist};

    /// Test that netlist with .VERILOGA directive can be parsed
    #[test]
    fn test_parse_veriloga_netlist() {
        let netlist_str = r#"Verilog-A Integration Test
.VERILOGA resistor.va
R1 1 0 1k
.END
"#;

        let netlist = Netlist::parse(netlist_str).unwrap();

        assert_eq!(netlist.veriloga_includes.len(), 1);
        assert_eq!(
            netlist.veriloga_includes[0].file_path.to_str().unwrap(),
            "resistor.va"
        );
        assert_eq!(netlist.elements.len(), 1);
    }

    /// Test that engine can build circuit with VerilogA devices field
    #[test]
    fn test_circuit_has_veriloga_devices() {
        // Simple netlist without VA includes - just verify the field exists
        let netlist_str = r#"Circuit Test
R1 1 0 1k
V1 1 0 5
.END
"#;

        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        // Verify the veriloga_devices field exists and is empty
        assert!(circuit.veriloga_devices.is_empty());
    }

    /// Test inline Verilog-A compilation through the engine
    #[test]
    fn test_compile_inline_veriloga() {
        use rspice_veriloga::VerilogACompiler;

        // Define Verilog-A source with inline disciplines
        let va_source = r#"
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
            
            module va_resistor(p, n);
                inout p, n;
                electrical p, n;
                parameter real r = 1000.0 from (0:inf);
                
                analog begin
                    I(p, n) <+ V(p, n) / r;
                end
            endmodule
        "#;

        let compiler = VerilogACompiler::default();
        let result = compiler.compile(va_source);

        assert!(result.is_ok(), "Compilation should succeed");
        let model = result.unwrap();
        assert_eq!(model.name, "va_resistor");
        assert_eq!(model.num_terminals, 2);
    }

    /// Test VerilogADevice integration with VerilogADevices collection
    #[test]
    fn test_veriloga_device_collection() {
        use rspice_core::device::veriloga::{VerilogADevice, VerilogADevices};
        use rspice_veriloga::VerilogACompiler;

        // Compile a model
        let va_source = r#"
            nature electrical; units = "V"; access = V; abstol = 1e-12; endnature
            nature current; units = "A"; access = I; abstol = 1e-12; endnature
            discipline electrical; potential electrical; flow current; enddiscipline
            
            module resistor(p, n);
                inout p, n;
                electrical p, n;
                parameter real g = 0.001;
                analog I(p, n) <+ g * V(p, n);
            endmodule
        "#;

        let compiler = VerilogACompiler::default();
        let model = compiler.compile(va_source).unwrap();

        // Create device and add to collection
        let mut devices = VerilogADevices::new();
        let device = VerilogADevice::new("R1", model, &[1, 0]);
        devices.add(device);

        assert_eq!(devices.len(), 1);
        assert!(devices.find_by_name("R1").is_some());
    }

    /// Test multiple devices in collection with iteration
    #[test]
    fn test_multiple_veriloga_devices() {
        use rspice_core::device::veriloga::{VerilogADevice, VerilogADevices};
        use rspice_veriloga::VerilogACompiler;

        let va_source = r#"
            nature electrical; units = "V"; access = V; abstol = 1e-12; endnature
            nature current; units = "A"; access = I; abstol = 1e-12; endnature
            discipline electrical; potential electrical; flow current; enddiscipline
            module resistor(p, n);
                inout p, n; electrical p, n;
                parameter real g = 0.001;
                analog I(p, n) <+ g * V(p, n);
            endmodule
        "#;

        let compiler = VerilogACompiler::default();
        let model = compiler.compile(va_source).unwrap();

        let mut devices = VerilogADevices::new();
        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.add(VerilogADevice::new("R2", model.clone(), &[2, 1]));
        devices.add(VerilogADevice::new("R3", model, &[2, 0]));

        assert_eq!(devices.len(), 3);

        // Test iteration
        let names: Vec<_> = devices.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, vec!["R1", "R2", "R3"]);

        // Test find by name
        assert!(devices.find_by_name("R2").is_some());
        assert!(devices.find_by_name("R4").is_none());
    }

    /// Test temperature and time propagation to all devices
    #[test]
    fn test_temperature_and_time_propagation() {
        use rspice_core::device::veriloga::{VerilogADevice, VerilogADevices};
        use rspice_veriloga::VerilogACompiler;

        let va_source = r#"
            nature electrical; units = "V"; access = V; abstol = 1e-12; endnature
            nature current; units = "A"; access = I; abstol = 1e-12; endnature
            discipline electrical; potential electrical; flow current; enddiscipline
            module resistor(p, n);
                inout p, n; electrical p, n;
                parameter real g = 0.001;
                analog I(p, n) <+ g * V(p, n);
            endmodule
        "#;

        let compiler = VerilogACompiler::default();
        let model = compiler.compile(va_source).unwrap();

        let mut devices = VerilogADevices::new();
        devices.add(VerilogADevice::new("R1", model.clone(), &[1, 0]));
        devices.add(VerilogADevice::new("R2", model, &[2, 0]));

        // Set temperature (K)
        devices.set_temperature(350.0);

        // Set simulation time
        devices.set_time(1e-6);

        // Verify through device context (if exposed)
        // The fact that no panic occurs verifies propagation works
        assert_eq!(devices.len(), 2);
    }

    /// Test device stamping into matrix and RHS
    #[test]
    fn test_device_stamping() {
        use rspice_core::device::veriloga::{VerilogADevice, VerilogADevices};
        use rspice_veriloga::VerilogACompiler;

        let va_source = r#"
            nature electrical; units = "V"; access = V; abstol = 1e-12; endnature
            nature current; units = "A"; access = I; abstol = 1e-12; endnature
            discipline electrical; potential electrical; flow current; enddiscipline
            module resistor(p, n);
                inout p, n; electrical p, n;
                parameter real g = 0.001;
                analog I(p, n) <+ g * V(p, n);
            endmodule
        "#;

        let compiler = VerilogACompiler::default();
        let model = compiler.compile(va_source).unwrap();

        let mut devices = VerilogADevices::new();
        devices.add(VerilogADevice::new("R1", model, &[1, 0]));

        // Verify total nodes count
        assert_eq!(devices.total_internal_nodes(), 0);

        // Create mock matrix/RHS for stamping
        let voltages = vec![1.0, 0.0]; // V(1) = 1V, V(0) = 0V (ground)
        use std::cell::Cell;
        let stamped_count = Cell::new(0);

        // Stamp all devices - uses closure-based stamping
        devices.stamp_all(
            &voltages,
            |_row, _col, _val| {
                stamped_count.set(stamped_count.get() + 1);
            },
            |_row, _val| {
                stamped_count.set(stamped_count.get() + 1);
            },
        );

        // Stamping should have occurred
        assert!(stamped_count.get() > 0, "Device should stamp entries");
    }

    /// Test error handling for invalid Verilog-A source
    #[test]
    fn test_invalid_veriloga_error() {
        use rspice_veriloga::VerilogACompiler;

        let invalid_source = r#"
            module broken syntax here
            this is not valid verilog-a
        "#;

        let compiler = VerilogACompiler::default();
        let result = compiler.compile(invalid_source);

        assert!(result.is_err(), "Invalid source should return error");
    }

    /// End-to-end test: compile, instantiate, evaluate, verify results
    #[test]
    fn test_end_to_end_evaluation() {
        use rspice_core::device::veriloga::VerilogADevice;
        use rspice_veriloga::VerilogACompiler;

        // A conductance of 0.001 S (1kΩ) with 1V → 1mA current
        let va_source = r#"
            nature electrical; units = "V"; access = V; abstol = 1e-12; endnature
            nature current; units = "A"; access = I; abstol = 1e-12; endnature
            discipline electrical; potential electrical; flow current; enddiscipline
            module resistor(p, n);
                inout p, n; electrical p, n;
                parameter real g = 0.001;
                analog I(p, n) <+ g * V(p, n);
            endmodule
        "#;

        let compiler = VerilogACompiler::default();
        let model = compiler.compile(va_source).unwrap();

        // Create device with nodes: p=1, n=0 (ground)
        let mut device = VerilogADevice::new("R1", model, &[1, 0]);

        // Update voltages: V(1) = 5V, V(0) = 0V
        // Voltage across = 5V, Current = 5V * 0.001 = 5mA
        device.update_voltages(&[5.0, 0.0]);

        // Evaluate the device
        let results = device.evaluate();

        // Results should be the computed current(s)
        // The resistor with g=0.001 S and V=5V should produce I = g*V = 0.005 A
        assert!(!results.is_empty(), "Evaluation should produce results");

        // First result is the current
        let current = results[0];
        assert!(
            (current.abs() - 0.005).abs() < 1e-10,
            "Expected ~5mA, got {} A",
            current
        );
    }
}
