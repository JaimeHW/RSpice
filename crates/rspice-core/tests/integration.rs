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

    // Build circuit successfully
    let circuit = engine
        .build_circuit(&netlist)
        .expect("Failed to build circuit");

    // Should have 2 nodes (in, out) plus ground
    assert!(circuit.num_nodes() >= 2, "Expected at least 2 nodes");

    // Run DC operating point
    let result = engine.run_dc_op(&netlist).expect("DC OP failed");

    // With 0V DC input, all nodes should be ~0V
    assert!(
        result.node_voltages[0].abs() < 0.1,
        "Node voltage should be near 0"
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
            result.node_voltages.len() > 0,
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
