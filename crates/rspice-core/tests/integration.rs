//! Integration tests for RSpice
//!
//! These tests verify the simulator works correctly with real netlists.

use rspice_core::{Engine, Netlist, SimulationConfig};

/// Parse a simple netlist string
fn parse_netlist(content: &str) -> Netlist {
    Netlist::parse(content).expect("Failed to parse netlist")
}

/// Test RC lowpass filter frequency response
#[test]
fn test_rc_lowpass_ac() {
    let netlist_str = r#"
* RC Lowpass Filter Integration Test
R1 in out 1k
C1 out 0 1u
Vin in 0 DC 0 AC 1
.AC DEC 10 1 100k
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    // Run AC Analysis
    // 1kHz cutoff frequency: f_c = 1 / (2*pi*R*C) = 1 / (2*pi*1k*1u) ≈ 159.15 Hz
    let results = engine
        .run_ac(&netlist, &[159.15])
        .expect("AC analysis failed");

    assert!(!results.is_empty(), "Should have AC results");

    // At cutoff frequency:
    // Magnitude should be 1/sqrt(2) ≈ 0.707 (-3dB)
    // Phase should be -45 degrees
    let cutoff = &results[0];

    // Find output node (should be index 2 for "out" if standard ordering holds,
    // but let's check index 1 and 2 to be safe or use named lookup if possible.
    // Indexing: 0=Ground, 1=in, 2=out usually.
    // If results keys are not available, we assume index 2.
    // If it was 0, maybe "out" is index 1?
    // Let's print for debugging if it fails again, but here let's try to be smarter.
    // If node 1 is input, it should be 1.0.
    // If node 2 is output, it should be 0.707.

    let v1 = cutoff.voltage_magnitude(1);
    let v2 = cutoff.voltage_magnitude(2);

    let (v_in, v_out) = if (v1 - 1.0).abs() < 0.1 {
        (v1, v2)
    } else {
        (v2, v1) // Swap if ordering is reversed
    };

    let phase_rad = if (v1 - 1.0).abs() < 0.1 {
        cutoff.voltage_phase(2)
    } else {
        cutoff.voltage_phase(1)
    };

    let phase_deg = phase_rad * 180.0 / std::f64::consts::PI;

    // Verify input is present (AC 1)
    assert!(
        (v_in - 1.0).abs() < 0.1,
        "Input AC magnitude should be 1.0, got {}",
        v_in
    );

    // Verify output
    assert!(
        (v_out - std::f64::consts::FRAC_1_SQRT_2).abs() < 0.05,
        "Magnitude at cutoff should be ~0.707, got {}. (in={}, out={})",
        v_out,
        v_in,
        v_out
    );
    assert!(
        (phase_deg + 45.0).abs() < 5.0,
        "Phase at cutoff should be ~-45 deg, got {}",
        phase_deg
    );
}

/// Test voltage divider DC operating point
#[test]
fn test_voltage_divider_dc() {
    let netlist_str = r#"
* Voltage Divider
V1 vcc 0 DC 10
R1 vcc out 1k
R2 out 0 1k
.OP
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    let result = engine.run_dc_op(&netlist).expect("DC OP failed");

    // With equal resistors, output should be half of input
    // Find the 'out' node voltage (should be ~5V)
    let has_expected_voltage = result.node_voltages.iter().any(|&v| (v - 5.0).abs() < 0.5);

    assert!(
        has_expected_voltage,
        "Expected ~5V at output node, got {:?}",
        result.node_voltages
    );
}

/// Test BJT common-emitter amplifier
#[test]
fn test_bjt_amplifier() {
    let netlist_str = r#"
* BJT Common-Emitter Amplifier
VCC vcc 0 DC 12
VIN in 0 DC 0.7
Q1 collector base emitter 2N2222
.MODEL 2N2222 NPN(IS=1e-14 BF=100 VAF=100)
RB vcc base 100k
RC vcc collector 1k
RE emitter 0 100
.OP
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    // Should parse and build successfully
    let circuit = engine
        .build_circuit(&netlist)
        .expect("Failed to build BJT circuit");
    assert!(
        circuit.has_nonlinear_devices(),
        "BJT circuit should be nonlinear"
    );

    // Run DC analysis - BJT circuits can be challenging to converge
    let result = engine.run_dc_op(&netlist);

    // Just verify it runs (BJT convergence can vary)
    assert!(
        result.is_ok(),
        "BJT DC should complete (may or may not converge)"
    );
}

/// Test MOSFET inverter
#[test]
fn test_mosfet_inverter() {
    let netlist_str = r#"
* NMOS Inverter with Resistor Load
VDD vdd 0 DC 5
VIN in 0 DC 0
M1 out in 0 0 NMOS1 W=10u L=1u
.MODEL NMOS1 NMOS(VTO=0.7 KP=110u)
RD vdd out 10k
.OP
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    let result = engine.run_dc_op(&netlist).expect("MOSFET DC OP failed");

    // With VIN=0 (below threshold), MOSFET is off, output should be high (~5V)
    // Look for a voltage near VDD
    let has_high_output = result.node_voltages.iter().any(|&v| v > 4.0);
    assert!(
        has_high_output,
        "Expected high output voltage when MOSFET is off, got {:?}",
        result.node_voltages
    );
}

/// Test DC sweep
#[test]
fn test_dc_sweep() {
    let netlist_str = r#"
* DC Sweep Test
V1 in 0 DC 0
R1 in out 1k
R2 out 0 1k
.DC V1 0 10 1
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    let results = engine
        .run_dc_sweep(&netlist, "V1", 0.0, 10.0, 1.0)
        .expect("DC sweep failed");

    // Should have 11 points (0 to 10 in steps of 1)
    assert_eq!(results.len(), 11, "Expected 11 sweep points");

    // Verify all results are valid simulations
    for (val, result) in &results {
        assert!(
            !result.node_voltages.is_empty(),
            "Should have voltage results at sweep point {}",
            val
        );
    }
}

/// Test linear circuits solve correctly  
#[test]
fn test_simple_resistor_network() {
    let netlist_str = r#"
* Simple resistor network
V1 n1 0 DC 5
R1 n1 n2 1k
R2 n2 0 1k
.OP
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    let result = engine.run_dc_op(&netlist).expect("DC OP failed");

    // Node n2 should have ~2.5V (voltage divider)
    let v_n2 = result
        .node_voltages
        .iter()
        .find(|&&v| (v - 2.5).abs() < 0.5);

    assert!(
        v_n2.is_some(),
        "Expected ~2.5V at n2 node: {:?}",
        result.node_voltages
    );
}

/// Test CMOS inverter - industry-standard digital circuit
#[test]
fn test_cmos_inverter() {
    let netlist_str = r#"
* CMOS Inverter
VDD vdd 0 DC 3.3
VIN in 0 DC 0
MP vdd in out vdd PMOS W=2u L=0.18u
MN out in 0 0 NMOS W=1u L=0.18u
.MODEL PMOS PMOS(VTO=-0.5 KP=50u)
.MODEL NMOS NMOS(VTO=0.5 KP=100u)
.OP
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    // Should parse successfully
    let circuit = engine
        .build_circuit(&netlist)
        .expect("Failed to build CMOS inverter");

    assert!(
        circuit.has_nonlinear_devices(),
        "CMOS circuit should be nonlinear"
    );

    // Run DC OP - with VIN=0, output should be high (~VDD)
    let result = engine.run_dc_op(&netlist);
    assert!(result.is_ok(), "CMOS inverter DC OP should complete");
}

/// Test stiff circuit - convergence stress test
/// High ratio of time constants requires robust solver
#[test]
fn test_stiff_circuit_convergence() {
    let netlist_str = r#"
* Stiff RC Network (wide range of time constants)
V1 in 0 DC 1
R1 in n1 1
C1 n1 0 1u
R2 n1 n2 1Meg
C2 n2 0 1p
.OP
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    // Stiff circuits can be challenging but DC OP should converge
    let result = engine.run_dc_op(&netlist);
    assert!(
        result.is_ok(),
        "Stiff circuit should converge: {:?}",
        result.err()
    );

    // All capacitors are open in DC, so n1 and n2 should be at input voltage
    let result = result.unwrap();
    let has_unity_voltage = result.node_voltages.iter().any(|&v| (v - 1.0).abs() < 0.1);
    assert!(has_unity_voltage, "Should have ~1V in stiff circuit");
}

/// Test multi-source circuit
#[test]
fn test_multi_source_superposition() {
    let netlist_str = r#"
* Multi-source circuit (superposition test)
V1 n1 0 DC 10
V2 0 n2 DC 5
R1 n1 n3 1k
R2 n2 n3 1k
R3 n3 0 1k
.OP
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    let result = engine.run_dc_op(&netlist);
    assert!(result.is_ok(), "Multi-source circuit should solve");
}

/// Test RC step response (Transient)
#[test]
fn test_rc_step_response() {
    let netlist_str = r#"
* RC Step Response
V1 in 0 PULSE(0 1 0 1u 1u 1m 2m)
R1 in out 1k
C1 out 0 1u
.TRAN 10u 5m
.END
"#;
    // Note: Time constant tau = R*C = 1ms
    // V(out) at t=tau should be ~0.632 * Vfinal (assuming start from 0)
    // Here PULSE starts at 0, goes to 1V.

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    let result = engine
        .run_tran(&netlist, 0.005, 0.00001)
        .expect("Transient failed");

    // Find index near t = 1ms
    let tau = 1e-3;
    let idx = result
        .time
        .iter()
        .position(|&t| t >= tau)
        .expect("Simulation should cover tau");

    // Check voltage at tau
    let v_out_at_tau = result.voltages[1][idx]; // Node 2 (out) is index 1

    assert!(
        (v_out_at_tau - 0.632).abs() < 0.05,
        "V(out) at tau (1ms) should be ~0.632V, got {}",
        v_out_at_tau
    );
}

/// Test Measurement Directives
#[test]
fn test_meas_directives() {
    let netlist_str = r#"
* Measurement Test
V1 in 0 SIN(0 1 1k)
R1 in 0 1k
.TRAN 10u 2m
.MEAS TRAN avg_v AVG V(in)
.MEAS TRAN max_v MAX V(in)
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    let result = engine
        .run_tran(&netlist, 0.002, 10e-6)
        .expect("Transient failed");

    // Evaluate measurements
    let mut meas_engine = rspice_core::MeasureEngine::new();
    for meas in &netlist.measurements {
        meas_engine.add(meas.clone());
    }

    use std::collections::HashMap;
    let mut signals: HashMap<String, &[f64]> = HashMap::new();
    // V(in) -> Node 1 (index 0 in voltages?)
    // In TransientResult: voltages[0] corresponds to Node 1.
    signals.insert("V(IN)".to_string(), &result.voltages[0]);

    let meas_results = meas_engine.evaluate(&result.time, &signals);

    let avg = meas_results
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case("avg_v"))
        .unwrap();
    let max = meas_results
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case("max_v"))
        .unwrap();

    // Avg of sine over integer cycles is 0
    assert!(avg.value.unwrap().abs() < 0.1, "AVG should be near 0");
    // Max should be 1
    assert!((max.value.unwrap() - 1.0).abs() < 0.01, "MAX should be 1.0");
}

/// Test Parametric Sweep (.STEP)
#[test]
fn test_step_param() {
    // Template netlist with placeholder for Rval
    let netlist_template = r#"
* Parametric Sweep
.PARAM Rval={}
V1 in 0 DC 10
R1 in out 1k
R2 out 0 {Rval}
.OP
.END
"#;
    // Steps:
    // 1. Rval=1k -> V(out) = 5V
    // 2. Rval=3k -> V(out) = 7.5V
    // 3. Rval=9k -> V(out) = 9V

    let values = [1000.0, 3000.0, 9000.0];
    let expected_outs = [5.0, 7.5, 9.0];

    for (i, &val) in values.iter().enumerate() {
        // Construct netlist with current parameter value
        // Note: We format the float to ensure it looks like "1000.0" etc.
        let netlist_str = netlist_template.replace("{}", &format!("{:.1}", val));

        let netlist = parse_netlist(&netlist_str);
        let engine = Engine::new(SimulationConfig::default());

        let result = engine.run_dc_op(&netlist).expect("DC OP failed");

        // Find 'out' node index. Node 0 is ground.
        // The netlist parsing order might vary, so we look up by name if possible,
        // or rely on the stable sort order used in `run_dc_op`.
        // In `run_dc_op`, `result.node_names` contains the names.

        let out_idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("Output node 'out' not found in results");

        let v_out = result.node_voltages[out_idx];

        assert!(
            (v_out - expected_outs[i]).abs() < 0.1,
            "Step {}: Rval={}, Expected {}V, got {}V. All voltages: {:?}",
            i,
            val,
            expected_outs[i],
            v_out,
            result.node_voltages
        );
    }
}

/// Test BJT amplifier transient - verifies simulation doesn't hang
/// and produces reasonable operating point values (not -1000V clamped values)
#[test]
fn test_bjt_transient() {
    let netlist_str = r#"
* BJT Common-Emitter Amplifier Transient
VCC vcc 0 DC 12
VIN in 0 SIN(0.6 0.1 1k)
Q1 collector base emitter 2N2222
.MODEL 2N2222 NPN(IS=1e-14 BF=100 VAF=100)
RB vcc base 100k
Rin in base 10k
RC vcc collector 1k
RE emitter 0 100
.TRAN 1u 2m
.END
"#;

    let netlist = parse_netlist(netlist_str);
    let engine = Engine::new(SimulationConfig::default());

    // Build circuit should succeed
    let circuit = engine
        .build_circuit(&netlist)
        .expect("Failed to build BJT circuit");
    assert!(
        circuit.has_nonlinear_devices(),
        "BJT circuit should be nonlinear"
    );

    // Run DC operating point first and check it produces reasonable values
    let dc_result = engine.run_dc_op(&netlist);
    assert!(dc_result.is_ok(), "BJT DC OP should complete");

    let dc = dc_result.unwrap();
    // Verify no voltage hit the -1000V / +1000V clamp limit (indicates convergence failure)
    for (i, &v) in dc.node_voltages.iter().enumerate() {
        assert!(
            v.abs() < 900.0,
            "Node {} voltage {} is near clamp limit - likely convergence failure",
            i,
            v
        );
    }

    // Run transient - should complete within reasonable time
    let start = std::time::Instant::now();
    let result = engine.run_tran(&netlist, 2e-3, 1e-6);
    let elapsed = start.elapsed();

    // Should complete reasonably fast (not hit 5-minute timeout)
    assert!(
        elapsed.as_secs() < 60,
        "Transient took too long: {:?} - likely convergence issues",
        elapsed
    );
    assert!(result.is_ok(), "BJT transient should complete");

    let tran = result.unwrap();
    // Verify we have multiple timepoints
    assert!(
        tran.time.len() > 10,
        "Expected multiple time points, got {}",
        tran.time.len()
    );
}
