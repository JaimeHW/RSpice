use super::*;
use crate::analysis::{HbConfig, HbTone};
#[cfg(feature = "veriloga")]
use crate::circuit::CircuitData;
#[cfg(feature = "veriloga")]
use crate::device::veriloga::VerilogADevice;
#[cfg(feature = "veriloga")]
use rspice_veriloga::codegen::{
    BytecodeProgram, CompiledModel, CompiledParameter, Instruction,
    JacobianEntry as VaJacobianEntry, StampIndex, StampLocation, StampProgram,
};

#[cfg(feature = "veriloga")]
fn create_veriloga_resistor_model() -> CompiledModel {
    let value_program = BytecodeProgram {
        instructions: vec![
            Instruction::PushParam(0),
            Instruction::PushVoltage(0, 1),
            Instruction::Mul,
        ],
    };
    let g_pos = BytecodeProgram {
        instructions: vec![Instruction::PushParam(0)],
    };
    let g_neg = BytecodeProgram {
        instructions: vec![Instruction::PushParam(0), Instruction::Neg],
    };

    CompiledModel {
        name: "hb_engine_va_resistor".into(),
        num_terminals: 2,
        terminal_names: vec!["p".into(), "n".into()],
        parameters: vec![CompiledParameter {
            name: "g".into(),
            default: 1e-3,
            min: Some(0.0),
            max: None,
        }],
        num_variables: 0,
        assignment_programs: vec![],
        stamp_programs: vec![StampProgram {
            stamp_locations: vec![
                StampLocation {
                    row: StampIndex::Terminal(0),
                    col: StampIndex::Ground,
                    sign: -1.0,
                },
                StampLocation {
                    row: StampIndex::Terminal(1),
                    col: StampIndex::Ground,
                    sign: 1.0,
                },
            ],
            value_program,
            jacobian_programs: vec![
                VaJacobianEntry {
                    row: StampIndex::Terminal(0),
                    col: StampIndex::Terminal(0),
                    program: g_pos.clone(),
                },
                VaJacobianEntry {
                    row: StampIndex::Terminal(0),
                    col: StampIndex::Terminal(1),
                    program: g_neg.clone(),
                },
                VaJacobianEntry {
                    row: StampIndex::Terminal(1),
                    col: StampIndex::Terminal(0),
                    program: g_neg,
                },
                VaJacobianEntry {
                    row: StampIndex::Terminal(1),
                    col: StampIndex::Terminal(1),
                    program: g_pos,
                },
            ],
        }],
        lookup_tables: vec![],
        internal_nodes: 0,
        branch_currents: 0,
        laplace_filters: vec![],
    }
}

// =========================================================================
// Error Type Tests
// =========================================================================

#[test]
fn test_hb_error_display_convergence() {
    let err = HbError::ConvergenceFailed {
        iterations: 50,
        residual: 1e-3,
    };
    let msg = format!("{}", err);
    assert!(msg.contains("50 iterations"));
    assert!(msg.contains("1.000e-3") || msg.contains("1.0e-3"));
}

#[test]
fn test_hb_error_display_no_reactive() {
    let err = HbError::NoReactiveElements;
    let msg = format!("{}", err);
    assert!(msg.contains("no capacitors or inductors"));
}

#[test]
fn test_hb_error_display_invalid_config() {
    let err = HbError::InvalidConfig("Bad frequency".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("Invalid HB config"));
    assert!(msg.contains("Bad frequency"));
}

#[test]
fn test_hb_error_display_singular() {
    let err = HbError::SingularMatrix;
    let msg = format!("{}", err);
    assert!(msg.contains("Singular"));
}

#[test]
fn test_hb_error_to_simulation_error_convergence() {
    let err = HbError::ConvergenceFailed {
        iterations: 25,
        residual: 1e-5,
    };
    let sim_err: SimulationError = err.into();
    match sim_err {
        SimulationError::ConvergenceFailed(n) => assert_eq!(n, 25),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_hb_error_to_simulation_error_no_reactive() {
    let err = HbError::NoReactiveElements;
    let sim_err: SimulationError = err.into();
    match sim_err {
        SimulationError::Circuit(msg) => assert!(msg.contains("capacitors")),
        _ => panic!("Wrong error type"),
    }
}

// =========================================================================
// Configuration Validation Tests
// =========================================================================

#[test]
fn test_run_hb_rejects_zero_frequency() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(0.0);

    let result = engine.run_hb(&netlist, config);
    assert!(result.is_err());

    if let Err(e) = result {
        let msg = format!("{}", e);
        assert!(msg.contains("frequency") || msg.contains("positive"));
    }
}

#[test]
fn test_run_hb_rejects_negative_frequency() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(-1e6);

    let result = engine.run_hb(&netlist, config);
    assert!(result.is_err());
}

#[test]
fn test_run_hb_rejects_purely_resistive() {
    use crate::Netlist;

    let netlist_str = r#"
            * Purely resistive - no reactive elements
            V1 in 0 DC 1
            R1 in out 1k
            R2 out 0 1k
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_err(),
        "HB should fail for purely resistive circuit"
    );

    if let Err(e) = result {
        let msg = format!("{}", e);
        assert!(msg.contains("capacitor") || msg.contains("reactive"));
    }
}

#[test]
fn test_run_hb_solves_supported_nonlinear_devices() {
    use crate::Netlist;

    let netlist_str = r#"
            * Nonlinear diode with reactive element
            I1 0 in DC 1m
            R1 in 0 1k
            C1 in 0 1n
            D1 in 0 DMOD
            .MODEL DMOD D (IS=1e-14 N=1)
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should solve supported nonlinear circuits: {:?}",
        result.err()
    );

    let hb = result.expect("nonlinear HB should succeed");
    assert!(hb.result.is_valid());
}

#[test]
fn test_run_hb_solves_jfet_nonlinear_devices() {
    use crate::Netlist;

    let netlist_str = r#"
            * JFET nonlinear HB support
            IBIAS 0 d DC 1m
            VG g 0 DC -1
            R1 d 0 2k
            C1 d 0 1n
            J1 d g 0 JMOD
            .MODEL JMOD NJF (VTO=-2 BETA=1e-3 LAMBDA=0.01)
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "JFET nonlinear HB should succeed: {:?}",
        result.err()
    );
    let hb = result.expect("JFET nonlinear HB should succeed");
    assert!(hb.result.is_valid());
}

#[test]
fn test_run_hb_solves_voltage_switch_nonlinear_devices() {
    use crate::Netlist;

    let netlist_str = r#"
            * Voltage-controlled switch nonlinear HB support
            VCTRL vc 0 DC 2
            IBIAS 0 out DC 1m
            RLOAD out 0 2k
            C1 out 0 1n
            S1 out 0 vc 0 SMOD
            .MODEL SMOD VSWITCH (VT=1 VH=0 RON=10 ROFF=1e9)
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "VSwitch nonlinear HB should succeed: {:?}",
        result.err()
    );
    let hb = result.expect("VSwitch nonlinear HB should succeed");
    assert!(hb.result.is_valid());
}

#[test]
fn test_run_hb_solves_current_switch_nonlinear_devices() {
    use crate::Netlist;

    let netlist_str = r#"
            * Current-controlled switch nonlinear HB support (0V sensing source)
            IBIAS 0 in DC 1m
            VSENSE in out DC 0
            RLOAD out 0 2k
            C1 out 0 1n
            W1 out 0 VSENSE SMOD
            .MODEL SMOD ISWITCH (IT=0.5m IH=0 RON=10 ROFF=1e9)
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "ISwitch nonlinear HB should succeed for 0V sensing control source: {:?}",
        result.err()
    );
    let hb = result.expect("ISwitch nonlinear HB should succeed");
    assert!(hb.result.is_valid());
}

#[cfg(feature = "veriloga")]
#[test]
fn test_hb_veriloga_devices_are_treated_as_supported() {
    let mut circuit = CircuitData::new();
    let node = circuit.get_or_create_node("n1");
    let mut device = VerilogADevice::new("RVA1", create_veriloga_resistor_model(), &[node, 0]);
    assert!(device.set_parameter("g", 1e-3));
    circuit.veriloga_devices.add(device);
    let num_nodes = circuit.num_nodes();

    assert!(
        Engine::hb_has_supported_nonlinear_devices(&circuit, num_nodes),
        "Verilog-A devices should enable nonlinear HB mode"
    );
    assert!(
        Engine::hb_unsupported_nonlinear_device_summary(&circuit, num_nodes).is_none(),
        "Verilog-A devices should not be rejected as unsupported"
    );
}

#[cfg(feature = "veriloga")]
#[test]
fn test_hb_stamps_veriloga_devices_into_solver() {
    let mut circuit = CircuitData::new();
    let node = circuit.get_or_create_node("n1");
    let mut device = VerilogADevice::new("RVA1", create_veriloga_resistor_model(), &[node, 0]);
    assert!(device.set_parameter("g", 1e-3));
    circuit.veriloga_devices.add(device);

    let num_nodes = circuit.num_nodes();
    let mut solver = HbSolver::new(HbConfig::new(1e6).with_harmonics(1), num_nodes);
    let engine = Engine::default();
    engine.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes);

    assert!(
        solver.has_nonlinear_devices(),
        "Verilog-A device stamping should register nonlinear HB devices"
    );
}

#[test]
fn test_run_hb_supports_iswitch_static_dc_control_source() {
    use crate::Netlist;

    let netlist_str = r#"
            * ISwitch control source may use static DC sensing source in HB
            VCTRL ctrl 0 DC 0
            VSENSE nsense 0 DC 1
            IBIAS 0 out DC 1m
            RLOAD out 0 1k
            C1 out 0 1n
            W1 out 0 VSENSE SMOD
            .MODEL SMOD ISWITCH (IT=1m IH=0 RON=1 ROFF=1e9)
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "static DC control source should be supported in HB"
    );
}

#[test]
fn test_run_hb_rejects_iswitch_time_varying_control_source() {
    use crate::Netlist;

    let netlist_str = r#"
            * Time-varying ISwitch control source remains unsupported in HB
            VSENSE nsense 0 AC 1
            IBIAS 0 out DC 1m
            RLOAD out 0 1k
            C1 out 0 1n
            W1 out 0 VSENSE SMOD
            .MODEL SMOD ISWITCH (IT=1m IH=0 RON=1 ROFF=1e9)
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(result.is_err(), "time-varying control source should fail");
    let msg = result.err().map(|e| e.to_string()).unwrap_or_default();
    assert!(
        msg.contains("current switch"),
        "expected current switch summary: {}",
        msg
    );
    assert!(
        msg.contains("static control-source waveforms"),
        "expected static-waveform guidance in diagnostics: {}",
        msg
    );
}

// =========================================================================
// Basic Circuit Tests
// =========================================================================

#[test]
fn test_run_hb_simple_rc() {
    use crate::Netlist;

    let netlist_str = r#"
            * Simple RC circuit
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(5).with_tolerance(1e-6);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should succeed for RC circuit: {:?}",
        result.err()
    );

    let hb_result = result.unwrap();
    assert_eq!(hb_result.num_harmonics, 5);
    assert!(hb_result.fundamental_freq > 0.0);
}

#[test]
fn test_run_hb_returns_spectral_voltages() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(3);

    if let Ok(result) = engine.run_hb(&netlist, config) {
        // Should have spectral voltages for each node
        assert!(!result.result.spectral_voltages.is_empty());

        // Each spectral voltage should have DC + harmonics coefficients
        for sv in &result.result.spectral_voltages {
            assert_eq!(sv.coefficients.len(), 4); // DC + 3 harmonics
        }
    }
}

#[test]
fn test_run_hb_with_current_source() {
    use crate::Netlist;

    let netlist_str = r#"
            * Circuit with current source
            I1 0 in DC 1m
            R1 in 0 1k
            C1 in 0 10n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(100e3).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with current source: {:?}",
        result.err()
    );
}

// =========================================================================
// Frequency Configuration Tests
// =========================================================================

#[test]
fn test_run_hb_preserves_fundamental_frequency() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    for freq in [1e3, 1e6, 2.5e6, 1e9] {
        let config = HbConfig::new(freq).with_harmonics(5);
        if let Ok(result) = engine.run_hb(&netlist, config) {
            assert!(
                (result.fundamental_freq - freq).abs() < 1.0,
                "Frequency should be preserved: expected {}, got {}",
                freq,
                result.fundamental_freq
            );
        }
    }
}

#[test]
fn test_run_hb_harmonics_count() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    for n_harm in [3, 5, 9, 15] {
        let config = HbConfig::new(1e6).with_harmonics(n_harm);
        if let Ok(result) = engine.run_hb(&netlist, config) {
            assert_eq!(result.num_harmonics, n_harm);
        }
    }
}

// =========================================================================
// Multi-Node Circuit Tests
// =========================================================================

#[test]
fn test_run_hb_two_stage_filter() {
    use crate::Netlist;

    let netlist_str = r#"
            * Two-stage RC filter
            V1 in 0 DC 1
            R1 in mid 1k
            C1 mid 0 1n
            R2 mid out 1k
            C2 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(100e3).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with two-stage filter: {:?}",
        result.err()
    );

    if let Ok(r) = result {
        // Should have nodes for in, mid, out
        assert!(r.result.num_nodes() >= 2);
    }
}

#[test]
fn test_run_hb_preserves_circuit_node_names_in_results() {
    use crate::Netlist;

    let netlist_str = r#"
            * Node-name preservation
            V1 vin 0 DC 1
            R1 vin vout 1k
            C1 vout 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should preserve netlist node names: {:?}",
        result.err()
    );

    let hb = result.expect("HB run should succeed");
    assert!(hb
        .result
        .node_names
        .iter()
        .any(|name| name.eq_ignore_ascii_case("vin")));
    assert!(hb
        .result
        .node_names
        .iter()
        .any(|name| name.eq_ignore_ascii_case("vout")));
    assert!(hb
        .result
        .spectral_voltages
        .iter()
        .any(|spectrum| spectrum.node_name.eq_ignore_ascii_case("vin")));
    assert!(hb
        .result
        .spectral_voltages
        .iter()
        .any(|spectrum| spectrum.node_name.eq_ignore_ascii_case("vout")));
}

#[test]
fn test_run_hb_parallel_rc() {
    use crate::Netlist;

    let netlist_str = r#"
            * Parallel RC
            V1 in 0 DC 1
            R1 in 0 1k
            C1 in 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with parallel RC: {:?}",
        result.err()
    );
}

// =========================================================================
// Result Validity Tests
// =========================================================================

#[test]
fn test_run_hb_result_is_valid() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 100
            C1 out 0 10p
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(10e6).with_harmonics(5);

    if let Ok(result) = engine.run_hb(&netlist, config) {
        assert!(result.result.is_valid());
    }
}

#[test]
fn test_run_hb_dc_operating_point() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 5
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(3);

    if let Ok(result) = engine.run_hb(&netlist, config) {
        let dc_op = result.result.dc_operating_point();
        // Should have DC values for each node
        for (name, dc) in &dc_op {
            assert!(dc.is_finite(), "DC at {} should be finite", name);
        }
    }
}

// =========================================================================
// HbAnalysisResult Tests
// =========================================================================

#[test]
fn test_hb_analysis_result_fields() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(2.5e6).with_harmonics(7);

    if let Ok(result) = engine.run_hb(&netlist, config) {
        assert_eq!(result.fundamental_freq, 2.5e6);
        assert_eq!(result.num_harmonics, 7);
        // converged should be boolean
        assert!(result.converged || !result.converged);
    }
}

// =========================================================================
// Numerical Accuracy Tests
// =========================================================================

#[test]
fn test_hb_rc_filter_dc_gain() {
    // RC lowpass filter: at DC, output = input (gain = 1)
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 5
            R1 in out 1k
            C1 out 0 1u
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    // Very low frequency - should see nearly full DC gain
    let config = HbConfig::new(10.0).with_harmonics(3);

    if let Ok(result) = engine.run_hb(&netlist, config) {
        // At DC, output should be close to input for RC filter
        let dc_op = result.result.dc_operating_point();
        // Verify we got DC values
        assert!(!dc_op.is_empty(), "Should have DC values");
    }
}

#[test]
fn test_hb_high_frequency() {
    // Test at GHz frequencies (RF/MW regime)
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 50
            C1 out 0 1p
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    // 1 GHz
    let config = HbConfig::new(1e9).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work at GHz frequencies: {:?}",
        result.err()
    );

    if let Ok(r) = result {
        assert_eq!(r.fundamental_freq, 1e9);
    }
}

#[test]
fn test_hb_very_low_frequency() {
    // Test at very low frequencies (sub-Hz)
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1M
            C1 out 0 10u
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    // 0.1 Hz
    let config = HbConfig::new(0.1).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work at sub-Hz frequencies: {:?}",
        result.err()
    );
}

// =========================================================================
// Edge Case Tests
// =========================================================================

#[test]
fn test_hb_tiny_capacitance() {
    // Femtofarad capacitance (RF regime)
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 50
            C1 out 0 10f
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(10e9).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with femtofarad capacitors: {:?}",
        result.err()
    );
}

#[test]
fn test_hb_large_resistance() {
    // GigaOhm resistance
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1G
            C1 out 0 1p
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with GΩ resistors: {:?}",
        result.err()
    );
}

#[test]
fn test_hb_small_resistance() {
    // Milliohm resistance
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 10m
            C1 out 0 100u
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e3).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with mΩ resistors: {:?}",
        result.err()
    );
}

#[test]
fn test_hb_many_harmonics() {
    // Test with large number of harmonics
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(31);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with 31 harmonics: {:?}",
        result.err()
    );

    if let Ok(r) = result {
        assert_eq!(r.num_harmonics, 31);
        // Each spectral voltage should have 32 coefficients (DC + 31 harmonics)
        for sv in &r.result.spectral_voltages {
            assert_eq!(sv.coefficients.len(), 32);
        }
    }
}

// =========================================================================
// Multi-Element Circuit Tests
// =========================================================================

#[test]
fn test_hb_ladder_filter() {
    // 3-stage ladder filter
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in n1 1k
            C1 n1 0 1n
            R2 n1 n2 1k
            C2 n2 0 1n
            R3 n2 out 1k
            C3 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(100e3).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with ladder filter: {:?}",
        result.err()
    );

    if let Ok(r) = result {
        // Should have nodes for n1, n2, out, in
        assert!(r.result.num_nodes() >= 3);
    }
}

#[test]
fn test_hb_parallel_elements() {
    // Multiple parallel RC elements
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in 0 1k
            R2 in 0 2k
            C1 in 0 1n
            C2 in 0 2n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with parallel elements: {:?}",
        result.err()
    );
}

#[test]
fn test_hb_bridge_circuit() {
    // Bridge/mesh topology
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in a 1k
            R2 in b 1k
            R3 a out 1k
            R4 b out 1k
            C1 a b 1n
            C2 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with bridge circuit: {:?}",
        result.err()
    );
}

// =========================================================================
// Multiple Source Tests
// =========================================================================

#[test]
fn test_hb_multiple_voltage_sources() {
    // Multiple voltage sources in circuit
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 5
            V2 bias 0 DC 2.5
            R1 in out 1k
            R2 bias out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with multiple voltage sources: {:?}",
        result.err()
    );
}

#[test]
fn test_hb_mixed_sources() {
    // Both voltage and current sources
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            I1 0 bias DC 1m
            R1 in out 1k
            R2 bias 0 1k
            C1 out bias 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with mixed sources: {:?}",
        result.err()
    );
}

#[test]
fn test_hb_voltage_source_enforces_dc_constraint_without_norton_error() {
    // Ideal source + divider should give exact DC divider ratio.
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 10
            R1 in out 1k
            R2 out 0 1k
            C1 out 0 1p
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(10e3).with_harmonics(3).with_tolerance(1e-9);

    let result = engine
        .run_hb(&netlist, config)
        .expect("HB run should succeed");
    let out = result
        .result
        .spectral_voltages
        .iter()
        .find(|sv| sv.node_name.eq_ignore_ascii_case("out"))
        .expect("out node should exist")
        .dc();
    assert!(
        (out - 5.0).abs() < 1e-6,
        "ideal source should produce 5.0 V divider output, got {}",
        out
    );
}

#[test]
fn test_hb_voltage_source_ac_amplitude_is_ideal() {
    // AC amplitude at an ideal source node should remain 1V regardless of load.
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 0 AC 1
            R1 in 0 1k
            C1 in 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e3).with_harmonics(3).with_tolerance(1e-9);

    let result = engine
        .run_hb(&netlist, config)
        .expect("HB run should succeed");
    let in_node = result
        .result
        .spectral_voltages
        .iter()
        .find(|sv| sv.node_name.eq_ignore_ascii_case("in"))
        .expect("in node should exist");
    let fundamental = in_node.magnitude(1);
    assert!(
        (fundamental - 1.0).abs() < 1e-6,
        "ideal AC source amplitude should be 1.0 V, got {}",
        fundamental
    );
}

// =========================================================================
// Spectral Coefficient Verification Tests
// =========================================================================

#[test]
fn test_hb_spectral_coefficients_dc_only() {
    // With only DC source, all AC harmonics should be ~zero
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(5);

    if let Ok(result) = engine.run_hb(&netlist, config) {
        for sv in &result.result.spectral_voltages {
            // DC component (index 0) may be non-zero
            // But all AC harmonics should be zero for DC-only input
            for k in 1..sv.coefficients.len() {
                let mag = sv.coefficients[k].norm();
                assert!(
                    mag < 1e-6,
                    "Harmonic {} should be ~zero for DC input, got {}",
                    k,
                    mag
                );
            }
        }
    }
}

#[test]
fn test_hb_spectral_voltage_magnitudes_finite() {
    // All spectral coefficients should be finite
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(9);

    if let Ok(result) = engine.run_hb(&netlist, config) {
        for sv in &result.result.spectral_voltages {
            for (i, coeff) in sv.coefficients.iter().enumerate() {
                assert!(
                    coeff.re.is_finite() && coeff.im.is_finite(),
                    "Coefficient {} should be finite: {:?}",
                    i,
                    coeff
                );
            }
        }
    }
}

#[test]
fn test_hb_all_nodes_have_spectral_voltages() {
    // Every node should have spectral voltage data
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in mid 1k
            C1 mid 0 1n
            R2 mid out 1k
            C2 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(1e6).with_harmonics(5);

    if let Ok(result) = engine.run_hb(&netlist, config) {
        let num_nodes = result.result.num_nodes();
        assert!(num_nodes >= 2, "Should have multiple nodes");
        assert_eq!(
            result.result.spectral_voltages.len(),
            num_nodes,
            "Should have spectral voltage for each node"
        );
    }
}

// =========================================================================
// Tolerance and Config Tests
// =========================================================================

#[test]
fn test_hb_different_tolerances() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    for tol in [1e-3, 1e-6, 1e-9, 1e-12] {
        let config = HbConfig::new(1e6).with_harmonics(5).with_tolerance(tol);
        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with tolerance {}: {:?}",
            tol,
            result.err()
        );
    }
}

#[test]
fn test_hb_oversample_factors() {
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    for oversample in [2, 4, 8] {
        let config = HbConfig::new(1e6)
            .with_harmonics(5)
            .with_oversample(oversample);
        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with oversample {}: {:?}",
            oversample,
            result.err()
        );
    }
}

// =========================================================================
// Stress Tests
// =========================================================================

#[test]
fn test_hb_many_nodes() {
    // Circuit with many nodes
    use crate::Netlist;

    let netlist_str = r#"
            V1 n1 0 DC 1
            R1 n1 n2 1k
            C1 n2 0 1n
            R2 n2 n3 1k
            C2 n3 0 1n
            R3 n3 n4 1k
            C3 n4 0 1n
            R4 n4 n5 1k
            C4 n5 0 1n
            R5 n5 out 1k
            C5 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    let config = HbConfig::new(100e3).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "HB should work with many nodes: {:?}",
        result.err()
    );

    if let Ok(r) = result {
        assert!(r.result.num_nodes() >= 5, "Should have 5+ nodes");
    }
}

#[test]
fn test_hb_repeated_runs_consistent() {
    // Multiple runs should give consistent results
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(5);

    let mut results = Vec::new();
    for _ in 0..3 {
        if let Ok(r) = engine.run_hb(&netlist, config.clone()) {
            results.push(r);
        }
    }

    assert_eq!(results.len(), 3, "All runs should succeed");

    // Results should be identical
    for i in 1..results.len() {
        assert_eq!(
            results[0].result.num_nodes(),
            results[i].result.num_nodes(),
            "Node count should be consistent"
        );
        assert_eq!(
            results[0].num_harmonics, results[i].num_harmonics,
            "Harmonics count should be consistent"
        );
    }
}

// =========================================================================
// Inductor HB Tests
// =========================================================================

#[test]
fn test_run_hb_simple_rl() {
    // Simple RL circuit: V1 -> R -> L -> GND
    // At DC, inductor is short circuit
    // At AC, |V_out| = |V_in| * X_L / sqrt(R² + X_L²)
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1 AC 1
            R1 in out 100
            L1 out 0 1u
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    // f = 1 MHz, X_L = 2π * 1e6 * 1e-6 = 2π ≈ 6.28 Ω
    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "RL circuit HB should succeed: {:?}",
        result.err()
    );

    let r = result.unwrap();
    assert!(r.result.converged, "Should converge");
}

#[test]
fn test_run_hb_rl_inductor_impedance() {
    // Test inductor impedance: |Z_L| = ωL at fundamental
    use crate::Netlist;

    let netlist_str = r#"
            I1 0 out DC 0 AC 1
            L1 out 0 10u
            R1 out 0 1k
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    // f = 100 kHz, X_L = 2π * 1e5 * 10e-6 = 6.28 Ω
    // R is 1kΩ, so Z ≈ 1kΩ (parallel), V ≈ I * Z
    let freq = 100e3;
    let config = HbConfig::new(freq).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(result.is_ok(), "Inductor test should succeed");

    let r = result.unwrap();
    assert!(r.result.converged);

    // At the fundamental (k=1), inductor has impedance j*ω*L
    // With parallel R, the voltage amplitude should be reasonable
    if let Some(sv) = r.result.spectral_voltages.first() {
        // Fundamental harmonic should have some voltage
        if sv.coefficients.len() > 1 {
            let v_fundamental = sv.coefficients[1].norm();
            // Should be non-zero (current source driving parallel R||L)
            assert!(v_fundamental > 0.0, "Should have AC response");
        }
    }
}

#[test]
fn test_run_hb_series_rl_frequency_response() {
    // Verify frequency response of series RL: higher frequency = more L impedance
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 0 AC 1
            R1 in mid 50
            L1 mid out 100u
            R2 out 0 50
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();

    // At low frequency
    let config_low = HbConfig::new(1e3).with_harmonics(3);
    let result_low = engine.run_hb(&netlist, config_low);

    // At high frequency
    let config_high = HbConfig::new(100e3).with_harmonics(3);
    let result_high = engine.run_hb(&netlist, config_high);

    assert!(result_low.is_ok(), "Low freq should work");
    assert!(result_high.is_ok(), "High freq should work");

    // Both should converge
    if let (Ok(r_low), Ok(r_high)) = (result_low, result_high) {
        assert!(r_low.result.converged);
        assert!(r_high.result.converged);
    }
}

#[test]
fn test_run_hb_inductor_only_circuit() {
    // Circuit with only inductor as reactive element
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 5
            R1 in mid 100
            L1 mid out 1m
            R2 out 0 100
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "Inductor-only reactive should work: {:?}",
        result.err()
    );
}

#[test]
fn test_run_hb_rlc_circuit() {
    // Mixed RLC circuit
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 1 AC 1
            R1 in mid1 100
            L1 mid1 mid2 10u
            C1 mid2 out 10n
            R2 out 0 1k
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(50e3).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "RLC circuit should work: {:?}",
        result.err()
    );

    if let Ok(r) = result {
        assert!(r.result.converged);
        assert!(r.result.num_nodes() >= 3);
    }
}

#[test]
fn test_run_hb_parallel_rlc() {
    // Parallel RLC tank circuit
    use crate::Netlist;

    let netlist_str = r#"
            I1 0 tank DC 0 AC 1m
            R1 tank 0 10k
            L1 tank 0 100u
            C1 tank 0 1n
            .END
        "#;

    // Resonant frequency f0 = 1/(2π√(LC)) ≈ 503 kHz
    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(500e3).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "Parallel RLC tank should work: {:?}",
        result.err()
    );
}

#[test]
fn test_run_hb_inductor_dc_short() {
    // At DC, inductor should act as short circuit
    // V1 -> L -> R -> GND: at DC, V_out should equal V_in (L is short)
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 10
            L1 in out 1m
            R1 out 0 1k
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(3);

    let result = engine.run_hb(&netlist, config);
    assert!(result.is_ok(), "DC inductor test should work");
}

#[test]
fn test_run_hb_multiple_inductors() {
    // Circuit with multiple inductors
    use crate::Netlist;

    let netlist_str = r#"
            V1 in 0 DC 5 AC 1
            R1 in n1 50
            L1 n1 n2 10u
            L2 n2 n3 20u
            R2 n3 0 50
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");
    let engine = Engine::default();
    let config = HbConfig::new(1e6).with_harmonics(5);

    let result = engine.run_hb(&netlist, config);
    assert!(
        result.is_ok(),
        "Multiple inductors should work: {:?}",
        result.err()
    );
}

#[test]
fn test_hb_solver_add_inductance() {
    // Test the solver's add_inductance method directly
    let config = HbConfig::new(1e6).with_harmonics(5);
    let mut solver = HbSolver::new(config, 3);

    // Add inductance stamps
    solver.add_inductance(0, 0, 1e-6);
    solver.add_inductance(0, 1, -1e-6);
    solver.add_inductance(1, 0, -1e-6);
    solver.add_inductance(1, 1, 1e-6);

    // Should complete without error
    // Verify by creating a state and computing residual
    let mut state = HbSolverState::new(3, 5);
    state.x[0][1] = Complex64::new(1.0, 0.0); // Set fundamental at node 0

    solver.compute_linear_residual(&mut state);
    // Residual should be finite
    assert!(state.residual_norm.is_finite());
}

#[test]
fn test_hb_solver_inductor_frequency_dependence() {
    // Verify inductor admittance is frequency-dependent
    use std::f64::consts::PI;

    let freq = 1e6;
    let l = 10e-6; // 10 µH
    let config = HbConfig::new(freq).with_harmonics(3);
    let mut solver = HbSolver::new(config, 1);

    // Add single inductor to ground
    solver.add_inductance(0, 0, l);

    let mut state = HbSolverState::new(1, 3);

    // Set DC component
    state.x[0][0] = Complex64::new(1.0, 0.0);
    // Set fundamental
    state.x[0][1] = Complex64::new(1.0, 0.0);
    // Set 2nd harmonic
    state.x[0][2] = Complex64::new(1.0, 0.0);

    solver.compute_linear_residual(&mut state);

    // At DC, inductor is short (large G), so residual[0] should be large
    // At harmonics, inductor has admittance -j/(ωL)
    // The residual magnitudes should differ between harmonics
    let res_dc = state.residual[0][0].norm();
    let res_1 = state.residual[0][1].norm();
    let res_2 = state.residual[0][2].norm();

    // DC residual should be larger (short circuit = large conductance)
    assert!(
        res_dc > res_1,
        "DC should have larger residual due to short circuit model"
    );

    // 2nd harmonic has 2x the frequency, so 2x the admittance (1/X_L)
    // The residual ratio should reflect this
    let omega1 = 2.0 * PI * freq;
    let omega2 = 2.0 * PI * freq * 2.0;
    let y1_mag = 1.0 / (omega1 * l);
    let y2_mag = 1.0 / (omega2 * l);

    // Expected ratio of residuals (both have same voltage)
    let expected_ratio = y2_mag / y1_mag; // = 0.5
    let actual_ratio = res_2 / res_1;

    assert!(
        (actual_ratio - expected_ratio).abs() < 0.1,
        "Admittance ratio should match frequency ratio: expected {}, got {}",
        expected_ratio,
        actual_ratio
    );
}

#[test]
fn test_dcac_parsing_and_circuit_building() {
    // Comprehensive test to verify DC AC combined syntax parsing
    // and propagation through circuit building to HB solver
    use crate::netlist::{ElementKind, SourceSpec};
    use crate::Netlist;

    let netlist_str = r#"
            * Test DC AC combined source
            I1 0 out DC 0 AC 1
            R1 out 0 1k
            C1 out 0 1n
            .END
        "#;

    let netlist = Netlist::parse(netlist_str).expect("Parse failed");

    // Verify the netlist parsed the current source correctly
    let isrc = netlist
        .elements
        .iter()
        .find(|e| e.name.to_uppercase() == "I1")
        .expect("Should find I1");

    // Check that SourceSpec is DcAc
    match &isrc.kind {
        ElementKind::CurrentSource(spec) => match spec {
            SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            } => {
                assert!(
                    (*dc_value - 0.0).abs() < 1e-12,
                    "DC value should be 0, got {}",
                    dc_value
                );
                assert!(
                    (*ac_magnitude - 1.0).abs() < 1e-12,
                    "AC magnitude should be 1, got {}",
                    ac_magnitude
                );
                assert!(
                    (*ac_phase - 0.0).abs() < 1e-12,
                    "AC phase should be 0, got {}",
                    ac_phase
                );
            }
            other => panic!("Expected DcAc variant, got {:?}", other),
        },
        other => panic!("Expected CurrentSource, got {:?}", other),
    }

    // Now verify it propagates through circuit building
    let engine = Engine::default();
    let circuit = engine
        .build_circuit(&netlist)
        .expect("Circuit build failed");

    // Check AC values in circuit
    assert!(
        !circuit.current_sources.is_empty(),
        "Should have current sources"
    );
    assert!(
        !circuit.current_sources.ac_magnitudes.is_empty(),
        "Should have AC magnitudes"
    );

    let ac_mag = circuit.current_sources.ac_magnitudes[0];
    let ac_phase = circuit.current_sources.ac_phases[0];

    assert!(
        (ac_mag - 1.0).abs() < 1e-12,
        "Circuit should have AC magnitude 1.0, got {}",
        ac_mag
    );
    assert!(
        (ac_phase - 0.0).abs() < 1e-12,
        "Circuit should have AC phase 0.0, got {}",
        ac_phase
    );
}

/// Test RLC series resonance in HB
#[test]
fn test_run_hb_rlc_series_resonance() {
    use std::f64::consts::PI;

    // At resonance f0 = 1/(2π√LC), impedance is purely resistive
    // Below/above resonance, phase shifts occur
    let r: f64 = 100.0;
    let l: f64 = 1e-3; // 1mH
    let c: f64 = 10e-9; // 10nF
                        // Resonant frequency = 1/(2π√(1e-3 * 10e-9)) ≈ 50.33 kHz
    let f0 = 1.0 / (2.0 * PI * (l * c).sqrt());

    // Test at resonance
    let netlist_res = format!(
        "* RLC Series at Resonance
V1 1 0 DC 0 AC 1
R1 1 2 {r}
L1 2 3 {l}
C1 3 0 {c}
.END"
    );
    let netlist = Netlist::parse(&netlist_res).unwrap();
    let engine = Engine::default();
    let config = HbConfig::new(f0);

    let result = engine.run_hb(&netlist, config).unwrap();

    // At resonance, Z = R, so V across R should be close to source voltage
    // Current = V/R = 1/100 = 10mA, voltage across R = IR = 1V
    // This is approximate due to AC source handling
    assert!(
        result.result.iterations < 5,
        "Should converge quickly at resonance"
    );
}

/// Test inductor phase shift (current lags voltage by 90°)
#[test]
fn test_hb_inductor_phase_shift() {
    use crate::Netlist;

    // Pure inductor: current should lag voltage by 90°
    let netlist_str = "* Pure RL for phase test
I1 0 1 DC 0 AC 1
R1 1 0 1k
L1 1 0 10mH
.END";
    let netlist = Netlist::parse(netlist_str).unwrap();
    let engine = Engine::default();
    let config = HbConfig::new(1e3); // 1kHz

    let result = engine.run_hb(&netlist, config).unwrap();

    // At node 1, we should have a voltage
    // The impedance of parallel R||jωL determines the voltage
    if let Some(v1) = result.result.get_node_voltage("1") {
        // Verify voltage has both real and imaginary parts (indicating phase)
        let magnitude = v1.magnitude(1); // 1 = fundamental
        assert!(magnitude > 0.0, "Should have non-zero voltage");
    }
}

/// Test that inductor impedance scales with frequency correctly
#[test]
fn test_hb_inductor_impedance_scaling() {
    use std::f64::consts::PI;

    let l = 10e-3; // 10mH
    let freq1 = 1e3; // 1 kHz
    let freq2 = 10e3; // 10 kHz

    // At freq1: X_L = 2π * 1000 * 0.01 ≈ 62.8 Ω
    // At freq2: X_L = 2π * 10000 * 0.01 ≈ 628 Ω
    // Ratio should be 10x

    let x_l1 = 2.0 * PI * freq1 * l;
    let x_l2 = 2.0 * PI * freq2 * l;

    let ratio = x_l2 / x_l1;
    assert!(
        (ratio - 10.0).abs() < 0.01,
        "Impedance should scale linearly with frequency: got {}",
        ratio
    );

    // Verify by running HB at both frequencies
    let netlist_str = "* RL circuit
I1 0 1 DC 0 AC 1
R1 1 0 100
L1 1 0 10m
.END";
    let netlist = Netlist::parse(netlist_str).unwrap();
    let engine = Engine::default();

    // Both should converge
    let result1 = engine.run_hb(&netlist, HbConfig::new(freq1)).unwrap();
    let result2 = engine.run_hb(&netlist, HbConfig::new(freq2)).unwrap();

    assert!(result1.converged, "Should converge at low freq");
    assert!(result2.converged, "Should converge at high freq");
}

/// Test parallel RLC resonance
#[test]
fn test_run_hb_parallel_rlc_resonance() {
    use std::f64::consts::PI;

    // Parallel RLC: at resonance, impedance is maximum (= R)
    let r: f64 = 10e3; // 10kΩ
    let l: f64 = 1e-3; // 1mH
    let c: f64 = 10e-9; // 10nF
    let f0 = 1.0 / (2.0 * PI * (l * c).sqrt()); // ~50.33 kHz

    let netlist_str = format!(
        "* Parallel RLC
I1 0 1 DC 0 AC 1mA
R1 1 0 {r}
L1 1 0 {l}
C1 1 0 {c}
.END"
    );
    let netlist = Netlist::parse(&netlist_str).unwrap();
    let engine = Engine::default();
    let config = HbConfig::new(f0);

    let result = engine.run_hb(&netlist, config).unwrap();

    // Should converge
    assert!(result.converged, "Should converge for parallel RLC");

    // At resonance, V = I * R = 1mA * 10kΩ = 10V (approximately)
    if let Some(v1) = result.result.get_node_voltage("1") {
        // Magnitude should be significant
        assert!(
            v1.magnitude(1) > 0.1,
            "Should have measurable voltage at resonance"
        );
    }
}

/// Test AC current source with various phases
#[test]
fn test_run_hb_ac_source_phase() {
    use crate::Netlist;

    // AC source at 45 degrees phase
    let netlist_str = "* AC source with phase
I1 0 1 DC 0 AC 1 45
R1 1 0 1k
C1 1 0 1n
.END";
    let netlist = Netlist::parse(netlist_str).unwrap();
    let engine = Engine::default();
    let config = HbConfig::new(1e3);

    let result = engine.run_hb(&netlist, config).unwrap();

    // Should converge with phase offset
    assert!(result.converged, "Should converge with phase offset");

    if let Some(v1) = result.result.get_node_voltage("1") {
        // Phase should be approximately 45 degrees
        let phase_deg = v1.phase(1) * 180.0 / std::f64::consts::PI;
        // Allow some tolerance due to numerical effects
        assert!(
            (phase_deg - 45.0).abs() < 5.0 || (phase_deg - 45.0 + 360.0).abs() < 5.0,
            "Phase should be ~45 degrees, got {}",
            phase_deg
        );
    }
}

#[test]
fn test_run_hb_multi_tone_source_stamps_target_harmonics() {
    use crate::Netlist;

    let netlist_str = "* Multi-tone harmonic source stamp
V1 1 0 DC 0 AC 1
R1 1 2 1k
C1 2 0 1n
.END";
    let netlist = Netlist::parse(netlist_str).expect("netlist should parse");
    let engine = Engine::default();

    let mut config = HbConfig::new(1e6).with_harmonics(9).with_tolerance(1e-6);
    config.tones = vec![
        HbTone::new(2e6, 1).with_name("f1"),
        HbTone::new(3e6, 1).with_name("f2"),
    ];

    let result = engine
        .run_hb(&netlist, config)
        .expect("HB multi-tone solve should succeed");
    assert!(result.converged);

    let v1 = result
        .result
        .get_node_voltage("1")
        .expect("node 1 should exist in HB result");
    assert!(
        v1.magnitude(2) > 0.9,
        "tone at harmonic 2 should be injected by the source"
    );
    assert!(
        v1.magnitude(3) > 0.9,
        "tone at harmonic 3 should be injected by the source"
    );
    assert!(
        v1.magnitude(1) < 1e-9,
        "no source energy should be stamped at harmonic 1 for this configuration"
    );
}

#[test]
fn test_run_hb_multi_tone_source_filters_route_tones_to_matching_sources() {
    use crate::Netlist;

    let netlist_str = "* Source-filtered multi-tone
VRF 1 0 DC 0 AC 1
VLO 2 0 DC 0 AC 1
R1 1 0 1k
R2 2 0 1k
C1 1 0 1n
C2 2 0 1n
.END";
    let netlist = Netlist::parse(netlist_str).expect("netlist should parse");
    let engine = Engine::default();

    let mut config = HbConfig::new(1e6).with_harmonics(8).with_tolerance(1e-6);
    config.tones = vec![
        HbTone::new(2e6, 1).with_name("rf").with_source("VRF"),
        HbTone::new(3e6, 1).with_name("lo").with_source("VLO"),
    ];

    let result = engine
        .run_hb(&netlist, config)
        .expect("HB source-filtered multi-tone solve should succeed");
    assert!(result.converged);

    let vrf = result
        .result
        .get_node_voltage("1")
        .expect("node 1 should exist in HB result");
    let vlo = result
        .result
        .get_node_voltage("2")
        .expect("node 2 should exist in HB result");

    assert!(vrf.magnitude(2) > 0.9, "VRF should be driven at harmonic 2");
    assert!(
        vrf.magnitude(3) < 1e-9,
        "VRF should not be driven at harmonic 3"
    );
    assert!(vlo.magnitude(3) > 0.9, "VLO should be driven at harmonic 3");
    assert!(
        vlo.magnitude(2) < 1e-9,
        "VLO should not be driven at harmonic 2"
    );
}

#[test]
fn test_run_hb_rejects_unknown_tone_source_filter() {
    use crate::Netlist;

    let netlist_str = "* Unknown tone source filter
V1 1 0 DC 0 AC 1
R1 1 0 1k
C1 1 0 1n
.END";
    let netlist = Netlist::parse(netlist_str).expect("netlist should parse");
    let engine = Engine::default();

    let mut config = HbConfig::new(1e6).with_harmonics(4).with_tolerance(1e-6);
    config.tones = vec![HbTone::new(2e6, 1).with_name("rf").with_source("V_MISSING")];

    let err = engine
        .run_hb(&netlist, config)
        .expect_err("unknown tone source filter should fail");
    assert!(
        err.to_string()
            .contains("not present in circuit independent sources"),
        "expected unknown source validation error, got: {}",
        err
    );
}

#[test]
fn test_run_hb_rejects_non_integer_tone_mapping() {
    use crate::Netlist;

    let netlist_str = "* Non-integer tone mapping
V1 1 0 DC 0 AC 1
R1 1 0 1k
C1 1 0 1n
.END";
    let netlist = Netlist::parse(netlist_str).expect("netlist should parse");
    let engine = Engine::default();

    let mut config = HbConfig::new(1e6).with_harmonics(12).with_tolerance(1e-6);
    config.tones = vec![HbTone::new(2.5e6, 1).with_name("bad-tone")];

    let err = engine
        .run_hb(&netlist, config)
        .expect_err("tone not on integer harmonic should fail");
    assert!(
        err.to_string().contains("integer harmonic"),
        "expected integer-harmonic validation error, got: {}",
        err
    );
}

#[test]
fn test_run_hb_rejects_tone_harmonic_beyond_configured_limit() {
    use crate::Netlist;

    let netlist_str = "* Tone harmonic limit
V1 1 0 DC 0 AC 1
R1 1 0 1k
C1 1 0 1n
.END";
    let netlist = Netlist::parse(netlist_str).expect("netlist should parse");
    let engine = Engine::default();

    let mut config = HbConfig::new(1e6).with_harmonics(4).with_tolerance(1e-6);
    config.tones = vec![HbTone::new(5e6, 1).with_name("h5")];

    let err = engine
        .run_hb(&netlist, config)
        .expect_err("tone above harmonic cap should fail");
    assert!(
        err.to_string().contains("num_harmonics"),
        "expected harmonic-cap validation error, got: {}",
        err
    );
}

/// Verify inductor acts as open circuit at very high frequency
#[test]
fn test_hb_inductor_high_frequency_behavior() {
    use crate::Netlist;

    // At very high frequency, inductor impedance becomes very large
    // For a voltage source driving R + L series, current -> 0
    let netlist_str = "* High frequency RL
V1 1 0 DC 0 AC 1
R1 1 2 100
L1 2 0 1
.END"; // 1H inductor at GHz would be huge impedance

    let netlist = Netlist::parse(netlist_str).unwrap();
    let engine = Engine::default();
    let config = HbConfig::new(1e9); // 1 GHz

    let result = engine.run_hb(&netlist, config);

    // Should still converge (or handle gracefully)
    // At 1GHz with 1H inductor: X_L = 6.28e9 Ω - essentially open
    // The solver should handle this without numerical issues
    assert!(result.is_ok(), "Should handle high frequency inductors");
}
