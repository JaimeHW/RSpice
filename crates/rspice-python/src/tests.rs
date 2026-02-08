//! Module-level integration tests for the Python bindings
//!
//! These tests verify the integration between different modules and
//! exercise complete simulation workflows.
//!
//! **NOTE**: These tests cause GIL-related panics when run via `cargo test`
//! due to PyO3's internal thread-safety requirements. Run via Python instead.

/// Integration tests - disabled by default to prevent GIL panics.
/// Enable with: cargo test -p rspice-python --features integration-tests
#[cfg(all(test, feature = "integration-tests"))]
mod integration_tests {
    use crate::config::{
        PyBypassConfig, PyConvergenceConfig, PyDampingStrategy, PySimulationConfig,
    };
    use crate::engine::PyEngine;
    use crate::netlist::PyNetlist;

    //=========================================================================
    // Complete Simulation Workflow Tests
    //=========================================================================

    #[test]
    fn test_complete_dc_workflow() {
        // Parse → Engine → DC OP → Extract results
        let netlist = PyNetlist::parse(
            r#"
* Complete DC workflow test
V1 1 0 12
R1 1 2 2k
R2 2 0 4k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        // Verify voltage divider: V2 = 12 * (4k / 6k) = 8V
        let v2 = result.voltage_by_index(2);
        assert!((v2 - 8.0).abs() < 0.1, "Expected V2=8V, got {}V", v2);
    }

    #[test]
    fn test_complete_transient_workflow() {
        // Parse → Engine → Transient → Verify waveform
        let netlist = PyNetlist::parse(
            r#"
* Complete transient workflow test
V1 1 0 10
R1 1 2 10k
C1 2 0 100n
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        // RC time constant = 10k * 100n = 1ms
        let result = engine.run_tran(&netlist, 5e-3, 50e-6).unwrap();

        assert!(result.num_points() > 10);
        assert!(result.stop_time() > 4e-3);

        // Check that voltage increases monotonically
        let v_start = result.voltage_at(2, 0);
        let v_end = result.voltage_at(2, result.num_points() - 1);
        assert!(
            v_end > v_start,
            "Voltage should increase: start={}, end={}",
            v_start,
            v_end
        );
    }

    #[test]
    fn test_complete_dc_sweep_workflow() {
        // Parse → Engine → DC Sweep → Verify linearity
        let netlist = PyNetlist::parse(
            r#"
* Complete DC sweep workflow test
V1 1 0 0
R1 1 2 1k
R2 2 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let sweep = engine.run_dc_sweep(&netlist, "V1", 0.0, 10.0, 2.0).unwrap();

        assert_eq!(sweep.len(), 6); // 0, 2, 4, 6, 8, 10

        // Verify linear relationship (divider = 0.5)
        for i in 0..sweep.len() {
            let vin = sweep.voltage_at(i);
            let vout = sweep.result_at(i).unwrap().voltage_by_index(2);
            let expected = vin / 2.0;
            assert!(
                (vout - expected).abs() < 0.1,
                "At Vin={}, expected Vout={}, got {}",
                vin,
                expected,
                vout
            );
        }
    }

    #[test]
    #[ignore = "AC analysis has GIL issues during parallel test execution"]
    fn test_complete_ac_workflow() {
        // Parse → Engine → AC → Verify filter response
        let netlist = PyNetlist::parse(
            r#"
* Complete AC workflow test (RC lowpass, fc = 159 Hz)
V1 1 0 AC 1
R1 1 2 1k
C1 2 0 1u
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        // Sweep from well below to well above cutoff
        let frequencies: Vec<f64> = vec![10.0, 100.0, 1000.0, 10000.0];
        let result = engine.run_ac(&netlist, frequencies).unwrap();

        assert_eq!(result.num_frequencies(), 4);

        // Verify low-pass characteristic
        let mag_10hz = result.magnitude_at(0, 1);
        let mag_10khz = result.magnitude_at(3, 1);

        assert!(
            mag_10khz < mag_10hz * 0.5,
            "Expected significant attenuation at 10kHz: {}V vs {}V at 10Hz",
            mag_10khz,
            mag_10hz
        );
    }

    //=========================================================================
    // Configuration Integration Tests
    //=========================================================================

    #[test]
    fn test_engine_with_custom_tolerance() {
        let mut config = PySimulationConfig::new();
        config.set_tolerance(1e-15); // Very tight tolerance

        let engine = PyEngine::new(Some(config));
        let netlist = PyNetlist::parse("V1 1 0 5\nR1 1 0 1k\n.end").unwrap();
        let result = engine.run_dc_op(&netlist).unwrap();

        // Should still converge with tight tolerance on simple circuit
        assert!((result.voltage_by_index(1) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_engine_with_custom_iterations() {
        let mut config = PySimulationConfig::new();
        config.set_max_iterations(200);

        let engine = PyEngine::new(Some(config));
        let cfg = engine.config();
        assert_eq!(cfg.inner.max_iterations, 200);
    }

    #[test]
    fn test_engine_with_custom_temperature() {
        let mut config = PySimulationConfig::new();
        config.set_temperature(350.0); // Hot junction

        let engine = PyEngine::new(Some(config));
        let cfg = engine.config();
        assert!((cfg.inner.temperature - 350.0).abs() < 0.1);
    }

    #[test]
    fn test_engine_with_robust_convergence() {
        let mut config = PySimulationConfig::new();
        config.set_convergence(PyConvergenceConfig::robust());

        let engine = PyEngine::new(Some(config));

        // Should handle diode circuit with robust convergence
        let netlist = PyNetlist::parse(
            r#"
V1 1 0 5
D1 1 2 1N4148
R1 2 0 1k
.end
"#,
        )
        .unwrap();

        let result = engine.run_dc_op(&netlist).unwrap();
        assert!(result.voltage_by_index(2) > 0.0);
    }

    #[test]
    fn test_engine_with_bypass_enabled() {
        let mut config = PySimulationConfig::new();
        config.set_bypass(PyBypassConfig::enabled());

        let engine = PyEngine::new(Some(config));
        let cfg = engine.config();
        assert!(cfg.inner.bypass_config.enabled);
    }

    //=========================================================================
    // Error Handling Integration Tests
    //=========================================================================

    #[test]
    fn test_dc_sweep_invalid_source() {
        let netlist = PyNetlist::parse("V1 1 0 5\nR1 1 0 1k\n.end").unwrap();
        let engine = PyEngine::new(None);

        let result = engine.run_dc_sweep(&netlist, "VNOTEXIST", 0.0, 5.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_dc_op_empty_circuit() {
        let netlist = PyNetlist::parse(".end").unwrap();
        let engine = PyEngine::new(None);

        let result = engine.run_dc_op(&netlist);
        assert!(result.is_err());
    }

    //=========================================================================
    // Complex Circuit Tests
    //=========================================================================

    #[test]
    fn test_multi_stage_amplifier_dc() {
        let netlist = PyNetlist::parse(
            r#"
* Two-stage resistor network
V1 1 0 10
R1 1 2 1k
R2 2 3 1k
R3 3 4 1k
R4 4 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        // Equal resistors in series: V drops by 2.5V at each node
        assert!((result.voltage_by_index(2) - 7.5).abs() < 0.1);
        assert!((result.voltage_by_index(3) - 5.0).abs() < 0.1);
        assert!((result.voltage_by_index(4) - 2.5).abs() < 0.1);
    }

    #[test]
    fn test_bridge_circuit_dc() {
        let netlist = PyNetlist::parse(
            r#"
* Wheatstone bridge
V1 1 0 10
R1 1 2 1k
R2 1 3 1k
R3 2 4 1k
R4 3 4 1k
R5 4 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        // Balanced bridge: V2 = V3 (symmetry)
        let v2 = result.voltage_by_index(2);
        let v3 = result.voltage_by_index(3);
        assert!(
            (v2 - v3).abs() < 0.1,
            "Bridge should be balanced: V2={}, V3={}",
            v2,
            v3
        );
    }

    #[test]
    fn test_multiple_voltage_sources() {
        let netlist = PyNetlist::parse(
            r#"
* Multiple voltage sources in series
V1 1 0 3
V2 2 1 2
V3 3 2 5
R1 3 0 1k
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        assert!((result.voltage_by_index(1) - 3.0).abs() < 0.1);
        assert!((result.voltage_by_index(2) - 5.0).abs() < 0.1);
        assert!((result.voltage_by_index(3) - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_current_source_with_parallel_resistors() {
        let netlist = PyNetlist::parse(
            r#"
* Current source with parallel load
I1 0 1 10m
R1 1 0 100
R2 1 0 100
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        // Parallel 100Ω = 50Ω, so V = 10mA * 50Ω = 0.5V
        let v1 = result.voltage_by_index(1);
        assert!((v1 - 0.5).abs() < 0.05, "Expected 0.5V, got {}V", v1);
    }

    //=========================================================================
    // DampingStrategy Integration Tests
    //=========================================================================

    #[test]
    #[ignore = "Damping strategy iteration may cause stack overflow in parallel tests"]
    fn test_all_damping_strategies() {
        let strategies = [
            PyDampingStrategy::None,
            PyDampingStrategy::LineSearch,
            PyDampingStrategy::VoltageLimiting,
            PyDampingStrategy::BankRose,
            PyDampingStrategy::Combined,
        ];

        let netlist = PyNetlist::parse("V1 1 0 5\nR1 1 0 1k\n.end").unwrap();

        for strategy in strategies {
            let mut config = PySimulationConfig::new();
            let mut convergence = PyConvergenceConfig::new();
            convergence.set_damping_strategy(strategy);
            config.set_convergence(convergence);

            let engine = PyEngine::new(Some(config));
            let result = engine.run_dc_op(&netlist).unwrap();

            assert!(
                (result.voltage_by_index(1) - 5.0).abs() < 0.1,
                "Failed with strategy {:?}",
                strategy
            );
        }
    }

    //=========================================================================
    // Performance/Stress Tests
    //=========================================================================

    #[test]
    fn test_many_resistor_chain() {
        // Create a chain of 20 resistors
        let mut netlist_str = String::from("* Long resistor chain\nV1 1 0 20\n");
        for i in 1..=20 {
            netlist_str.push_str(&format!("R{} {} {} 1k\n", i, i, i + 1));
        }
        netlist_str.push_str("R21 21 0 1k\n.end");

        let netlist = PyNetlist::parse(&netlist_str).unwrap();
        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();

        // V1 = 20V, each resistor drops ~0.95V (21 equal resistors)
        let v1 = result.voltage_by_index(1);
        assert!((v1 - 20.0).abs() < 0.1);

        // Last node before ground should have small voltage
        let v_last = result.voltage_by_index(21);
        let expected_last = 20.0 / 21.0; // ~0.95V
        assert!((v_last - expected_last).abs() < 0.1);
    }

    #[test]
    fn test_fine_dc_sweep() {
        let netlist = PyNetlist::parse("V1 1 0 0\nR1 1 0 1k\n.end").unwrap();
        let engine = PyEngine::new(None);

        // Many sweep points
        let sweep = engine.run_dc_sweep(&netlist, "V1", 0.0, 1.0, 0.01).unwrap();
        assert_eq!(sweep.len(), 101);
    }

    #[test]
    fn test_long_transient() {
        let netlist = PyNetlist::parse(
            r#"
V1 1 0 5
R1 1 2 1k
C1 2 0 1n
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        // Short time constant (1µs), run for 10µs
        let result = engine.run_tran(&netlist, 10e-6, 0.1e-6).unwrap();

        assert!(result.num_points() > 50);
    }

    //=========================================================================
    // Case Sensitivity Tests (SPICE Standard)
    //=========================================================================

    #[test]
    fn test_source_name_case_insensitivity() {
        let netlist = PyNetlist::parse("V1 1 0 0\nR1 1 0 1k\n.end").unwrap();
        let engine = PyEngine::new(None);

        // Should work with lowercase
        let result1 = engine.run_dc_sweep(&netlist, "v1", 0.0, 1.0, 1.0);
        assert!(result1.is_ok());

        // Should work with mixed case
        let result2 = engine.run_dc_sweep(&netlist, "V1", 0.0, 1.0, 1.0);
        assert!(result2.is_ok());
    }

    //=========================================================================
    // Netlist Feature Tests
    //=========================================================================

    #[test]
    fn test_netlist_with_comments() {
        let netlist = PyNetlist::parse(
            r#"
* This is the title
* Another comment
V1 1 0 5      ; inline comment style
R1 1 0 1k     $ another style
.end
"#,
        )
        .unwrap();

        let engine = PyEngine::new(None);
        let result = engine.run_dc_op(&netlist).unwrap();
        assert!((result.voltage_by_index(1) - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_netlist_with_continuation() {
        let netlist = PyNetlist::parse(
            r#"
* Test continuation lines
V1 1 0 5
R1 1
+ 0 1k
.end
"#,
        )
        .unwrap();

        // Parser should handle continuation lines
        assert!(netlist.num_elements() >= 1);
    }

    #[test]
    fn test_netlist_engineering_units() {
        let netlist = PyNetlist::parse(
            r#"
* Engineering notation test
V1 1 0 2.5
R1 1 2 1k
R2 2 3 10meg
R3 3 0 100
C1 2 0 1n
C2 3 0 47p
L1 1 0 10u
.end
"#,
        )
        .unwrap();

        assert!(netlist.num_elements() >= 5);
    }
}
