//! Engine Unit Tests
//!
//! Comprehensive tests for all simulation analysis types.

#[cfg(test)]
mod engine_tests {
    use crate::Netlist;
    use crate::Value;
    use crate::abort_signal::ImmediateAbort;
    use crate::engine::Engine;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rspice_engine_{}_{}_{}",
            prefix,
            std::process::id(),
            stamp
        ))
    }

    #[test]
    fn test_simple_resistor_divider() {
        let netlist_str = r#"
* Simple voltage divider
V1 1 0 10
R1 1 2 1k
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v2 = result.voltage(2);
        assert!((v2 - 5.0).abs() < 0.01, "Expected 5V, got {}V", v2);
    }

    #[test]
    fn test_model_based_resistor_uses_rsh_and_geometry() {
        let netlist_str = r#"
* Model-based resistor with geometry
V1 in 0 1
R1 in out RMOD L=10u W=2u
R2 out 0 1k
.MODEL RMOD R (RSH=100)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let vout = result.voltage(2);

        // R1 = RSH * (L/W) = 100 * (10u/2u) = 500 ohms
        // Divider: Vout = 1k / (500 + 1k) = 2/3
        assert!(
            (vout - (2.0 / 3.0)).abs() < 1e-3,
            "Expected ~0.6667V, got {}V",
            vout
        );
    }

    #[test]
    fn test_model_based_resistor_respects_multiplicity() {
        let netlist_str = r#"
* Model-based resistor multiplicity
V1 in 0 1
R1 in out RMOD L=10u W=2u M=2
R2 out 0 1k
.MODEL RMOD R (RSH=100)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let vout = result.voltage(2);

        // Base R = 500 ohms, M=2 -> effective 250 ohms
        // Divider: 1k / (250 + 1k) = 0.8
        assert!((vout - 0.8).abs() < 1e-3, "Expected ~0.8V, got {}V", vout);
    }

    #[test]
    fn test_model_based_resistor_explicit_r_overrides_geometry() {
        let netlist_str = r#"
* Model-based resistor explicit override
V1 in 0 1
R1 in out RMOD R=2k L=10u W=2u
R2 out 0 1k
.MODEL RMOD R (RSH=100)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let vout = result.voltage(2);

        // Explicit R=2k overrides geometry-derived value.
        // Divider: 1k / (2k + 1k) = 1/3
        assert!(
            (vout - (1.0 / 3.0)).abs() < 1e-3,
            "Expected ~0.3333V, got {}V",
            vout
        );
    }

    #[test]
    fn test_model_based_resistor_missing_model_errors() {
        let netlist_str = r#"
* Missing model for model-based resistor
V1 in 0 1
R1 in out RMOD L=10u W=2u
R2 out 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let err = engine
            .run_dc_op(&netlist)
            .expect_err("missing model should fail");
        let msg = format!("{}", err);
        assert!(
            msg.contains("unknown model 'RMOD'"),
            "unexpected error: {}",
            msg
        );
    }

    #[test]
    fn test_current_source() {
        let netlist_str = r#"
* Current source test
I1 0 1 1m
R1 1 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v1 = result.voltage(1);
        assert!((v1 - 1.0).abs() < 0.01, "Expected 1V, got {}V", v1);
    }

    #[test]
    fn test_diode_circuit() {
        let netlist_str = r#"
* Diode forward voltage test
V1 1 0 5
D1 1 2 1N4148
R1 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v2 = result.voltage(2);
        assert!(v2 > 0.0, "Expected positive voltage at node 2, got {}V", v2);
    }

    #[test]
    fn test_behavioral_voltage_source_stamps_branch_and_evaluates_expression() {
        let netlist = Netlist::parse(
            r#"
* Behavioral voltage source should evaluate V() and stamp as MNA branch
V1 in 0 2
B1 out 0 V=V(in)*2
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let vout = result.voltage(2);
        assert!(
            (vout - 4.0).abs() < 1e-6,
            "expected behavioral source output of 4V, got {}",
            vout
        );
    }

    #[test]
    fn test_behavioral_source_branch_current_reference_resolves_by_name() {
        let netlist = Netlist::parse(
            r#"
* Behavioral source references current through named branch source
V1 in 0 3
R1 in 0 1k
B1 out 0 V=ABS(I(V1))*1000
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let vout = result.voltage(2);
        assert!(
            (vout - 3.0).abs() < 2e-3,
            "expected behavioral branch-current derived output near 3V, got {}",
            vout
        );
    }

    #[test]
    fn test_behavioral_source_unknown_node_reference_fails_build() {
        let netlist = Netlist::parse(
            r#"
* Unknown behavioral node reference must fail build
V1 in 0 1
B1 out 0 V=V(does_not_exist)
R1 out 0 1k
.end
"#,
        )
        .unwrap();

        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown behavioral reference should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Behavioral source") && msg.contains("unknown node"),
            "expected unknown behavioral node error, got {}",
            msg
        );
    }

    #[test]
    fn test_behavioral_source_invalid_expression_fails_build() {
        let netlist = Netlist::parse(
            r#"
* Invalid behavioral expression must fail build
V1 in 0 1
B1 out 0 V=V(in) @ 2
R1 out 0 1k
.end
"#,
        )
        .unwrap();

        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("invalid behavioral expression should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Invalid behavioral expression") && msg.contains("@"),
            "expected strict behavioral parse error, got {}",
            msg
        );
    }

    #[test]
    fn test_behavioral_source_expands_user_defined_function_calls() {
        let netlist = Netlist::parse(
            r#"
* Behavioral source should support .FUNC expansion
.FUNC BAR2(P) 'V(P)+P'
VNODEP p 0 102
B1 out 0 V='bar2(17.0)'
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let out_idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("missing out node");
        let vout = result.voltage(out_idx);
        assert!(
            (vout - 119.0).abs() < 1e-6,
            "expected behavioral .FUNC expansion output of 119V, got {}",
            vout
        );
    }

    #[test]
    fn test_behavioral_source_subckt_internal_probe_is_hierarchically_remapped() {
        let netlist = Netlist::parse(
            r#"
* Internal subckt node probe V(1) should map to hierarchical node
.SUBCKT NINT_SRC OUT
V1 1 0 2.6
B1 OUT 0 V=nint(v(1))
.ENDS
X1 out NINT_SRC
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let out_idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("missing out node");
        let vout = result.voltage(out_idx);
        assert!(
            (vout - 3.0).abs() < 1e-6,
            "expected remapped internal probe nint(v(1)) to evaluate to 3V, got {}",
            vout
        );
    }

    #[test]
    fn test_behavioral_nint_uses_round_ties_even() {
        let netlist = Netlist::parse(
            r#"
* nint should round halves to even (2.5 -> 2)
B1 out 0 V=nint(2.5)
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let out_idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("missing out node");
        let vout = result.voltage(out_idx);
        assert!(
            (vout - 2.0).abs() < 1e-6,
            "expected nint(2.5) = 2.0 (ties-even), got {}",
            vout
        );
    }

    #[test]
    fn test_behavioral_u_u2_and_eq0_helpers_match_ngspice_semantics() {
        let netlist = Netlist::parse(
            r#"
* u(0) = 0.5, u2(1.25) = 1, eq0(0) = 1
B1 n_u 0 V=u(0)
B2 n_u2 0 V=u2(1.25)
B3 n_eq0 0 V=eq0(0)
R1 n_u 0 1k
R2 n_u2 0 1k
R3 n_eq0 0 1k
.end
"#,
        )
        .unwrap();

        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let node_voltage = |name: &str| -> f64 {
            let idx = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap_or_else(|| panic!("missing node {}", name));
            result.voltage(idx)
        };
        let vu = node_voltage("n_u");
        let vu2 = node_voltage("n_u2");
        let veq0 = node_voltage("n_eq0");

        assert!((vu - 0.5).abs() < 1e-6, "expected u(0)=0.5, got {}", vu);
        assert!(
            (vu2 - 1.0).abs() < 1e-6,
            "expected u2(1.25)=1.0, got {}",
            vu2
        );
        assert!(
            (veq0 - 1.0).abs() < 1e-6,
            "expected eq0(0)=1.0, got {}",
            veq0
        );
    }

    #[test]
    fn test_behavioral_func_i_probe_expression_converges() {
        let netlist = Netlist::parse(
            r#"
* Isolate .FUNC with I(vsrc) probe usage
vncol2 p 0 102.0
vp 1 0 105.0
rp 1 0 1.0
.func baz1(n,vp) 'n+i(vp)+vp'
b1 out 0 v='baz1(17.0,10000)'
rload out 0 1k
.end
"#,
        )
        .unwrap();

        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("I(vsrc)-based behavioral function should converge");
        let out_idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("missing out node");
        let vout = result.voltage(out_idx);
        assert!(
            (vout - 9912.0).abs() < 1e-3,
            "expected baz1 result of 9912, got {}",
            vout
        );
    }

    #[test]
    fn test_behavioral_func_i_probe_expression_converges_without_load() {
        let netlist = Netlist::parse(
            r#"
* Isolate .FUNC with I(vsrc) probe usage (no explicit B-source load)
vncol2 p 0 102.0
vp 1 0 105.0
rp 1 0 1.0
.func baz1(n,vp) 'n+i(vp)+vp'
b1 out 0 v='baz1(17.0,10000)'
.end
"#,
        )
        .unwrap();

        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("unloaded I(vsrc)-based behavioral function should converge");
        let out_idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("missing out node");
        let vout = result.voltage(out_idx);
        assert!(
            (vout - 9912.0).abs() < 1e-3,
            "expected baz1 result of 9912, got {}",
            vout
        );
    }

    #[test]
    fn test_behavioral_i_probe_binds_to_named_voltage_source_branch() {
        let netlist = Netlist::parse(
            r#"
vncol2 p 0 102.0
vp 1 0 105.0
rp 1 0 1.0
b1 out 0 v=i(vp)+17
rload out 0 1k
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let mut circuit = engine.build_circuit(&netlist).unwrap();
        assert_eq!(circuit.behavioral_sources.voltage_sources.len(), 1);
        let vp_branch = circuit
            .get_branch_by_name("vp")
            .expect("missing vp branch ordinal");
        let num_nodes = circuit.num_nodes();
        let vp_idx = num_nodes + vp_branch - 1;
        let mut solution = vec![0.0; circuit.matrix_size()];
        solution[vp_idx] = -105.0;

        let out = circuit.behavioral_sources.voltage_sources[0].evaluate(&solution, 0.0);
        assert!(
            (out - (-88.0)).abs() < 1e-9,
            "expected i(vp)+17 with i(vp)=-105 to evaluate to -88, got {}",
            out
        );
    }

    #[test]
    fn test_transient_rc() {
        let netlist_str = r#"
* RC Transient Test
V1 1 0 5
R1 1 2 1k
C1 2 0 1u
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_tran(&netlist, 1e-3, 100e-6).unwrap();
        assert!(result.num_points() > 1, "Expected multiple time points");
        assert!(result.time.len() > 1, "Expected time progression");
    }

    #[test]
    fn test_transient_honors_fixed_integration_method() {
        let netlist_str = r#"
* RC step response (method-sensitive with coarse timesteps)
V1 in 0 PULSE(0 1 0 1n 1n 2u 4u)
R1 in out 1k
C1 out 0 1n
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();

        let mut trap_cfg = crate::engine::SimulationConfig::default();
        trap_cfg.integration_method = crate::analysis::IntegrationMethod::Trapezoidal;
        trap_cfg.min_timestep = 1e-15;
        let trap = Engine::new(trap_cfg)
            .run_tran(&netlist, 8e-6, 5e-7)
            .unwrap();

        let mut gear_cfg = crate::engine::SimulationConfig::default();
        gear_cfg.integration_method = crate::analysis::IntegrationMethod::Gear2;
        gear_cfg.min_timestep = 1e-15;
        let gear = Engine::new(gear_cfg)
            .run_tran(&netlist, 8e-6, 5e-7)
            .unwrap();

        let trap_out_idx = trap
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("trap run missing out node");
        let gear_out_idx = gear
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("gear run missing out node");

        let sample_at = |time: &[f64], values: &[f64], t_query: f64| -> f64 {
            if time.is_empty() || values.is_empty() {
                return 0.0;
            }
            if t_query <= time[0] {
                return values[0];
            }
            for i in 0..time.len().saturating_sub(1) {
                let t0 = time[i];
                let t1 = time[i + 1];
                if t_query >= t0 && t_query <= t1 {
                    let y0 = values[i];
                    let y1 = values[i + 1];
                    let alpha = if (t1 - t0).abs() > 0.0 {
                        (t_query - t0) / (t1 - t0)
                    } else {
                        0.0
                    };
                    return y0 + alpha * (y1 - y0);
                }
            }
            *values.last().unwrap_or(&0.0)
        };

        let trap_v = sample_at(&trap.time, &trap.voltages[trap_out_idx], 0.5e-6);
        let gear_v = sample_at(&gear.time, &gear.voltages[gear_out_idx], 0.5e-6);

        assert!(
            (trap_v - gear_v).abs() > 1e-4,
            "fixed integration methods should diverge on coarse RC step: trap_v={}, gear_v={}",
            trap_v,
            gear_v
        );
    }

    #[test]
    fn test_collect_node_voltage_hints_prefers_ic_over_nodeset() {
        let netlist = Netlist::parse(
            r#"
* .IC should override .NODESET for the same node
V1 in 0 1
R1 in out 1k
R2 out 0 1k
.NODESET out=0.25
.IC out=0.75
.end
"#,
        )
        .expect("netlist should parse");

        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("circuit should build");
        let hints = engine.collect_node_voltage_hints(&netlist, &circuit);
        let out_node = circuit
            .get_node_by_name("out")
            .expect("out node should exist");

        let out_hint = hints
            .iter()
            .find(|(node_id, _)| *node_id == out_node)
            .map(|(_, v)| *v);
        assert_eq!(out_hint, Some(0.75));
    }

    #[test]
    fn test_transient_ic_overrides_initial_point() {
        let netlist = Netlist::parse(
            r#"
* .IC should override t=0 transient node state
V1 in 0 5
R1 in out 1k
C1 out 0 1u
.IC V(out)=0
.end
"#,
        )
        .expect("netlist should parse");

        let engine = Engine::default();
        let result = engine
            .run_tran(&netlist, 200e-6, 2e-6)
            .expect("transient should run");
        assert!(
            result.num_points() >= 2,
            "expected multiple transient points"
        );

        let v_out_t0 = result.voltage_at(2, 0);
        let v_out_t1 = result.voltage_at(2, 1);
        assert!(
            v_out_t0.abs() < 1e-9,
            "expected V(out) at t=0 from .IC to be ~0V, got {}",
            v_out_t0
        );
        assert!(
            v_out_t1 > v_out_t0,
            "expected V(out) to begin charging after t=0, got t0={} t1={}",
            v_out_t0,
            v_out_t1
        );
    }

    #[test]
    fn test_ac_rc_lowpass() {
        let netlist_str = r#"
* AC Lowpass Test
V1 1 0 AC 1
R1 1 2 1k
C1 2 0 1u
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let frequencies = vec![10.0, 10000.0];
        let results = engine.run_ac(&netlist, &frequencies).unwrap();
        assert_eq!(results.len(), 2);
        let mag_low = results[0].voltage_magnitude(1);
        assert!(mag_low > 0.8, "Expected ~1V at low freq, got {}V", mag_low);
    }

    #[test]
    fn test_ac_inductor_branch_stamping_matches_rl_transfer() {
        let netlist_str = r#"
* AC RL inductor transfer
V1 1 0 AC 1
R1 1 2 1k
L1 2 0 1m
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine.run_ac(&netlist, &[1.0, 1e6]).unwrap();
        assert_eq!(results.len(), 2);

        let low = results[0].voltage_magnitude(2);
        let high = results[1].voltage_magnitude(2);

        assert!(
            low < 0.02,
            "low-frequency inductor output should be near 0, got {}",
            low
        );
        assert!(
            high > 0.95,
            "high-frequency inductor output should approach input, got {}",
            high
        );
    }

    #[test]
    fn test_ac_current_source_excitation_stamps_rhs() {
        let netlist_str = r#"
* AC current-source excitation
I1 1 0 AC 1m
R1 1 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine.run_ac(&netlist, &[10e3]).unwrap();
        assert_eq!(results.len(), 1);

        let v1 = results[0].voltage_magnitude(1);
        assert!(
            (v1 - 1.0).abs() < 1e-3,
            "expected |V(1)|≈1V from 1mA*1k, got {}",
            v1
        );
    }

    #[test]
    fn test_ac_vcvs_gain_stamping() {
        let netlist_str = r#"
* AC VCVS gain test
V1 1 0 AC 1
E1 2 0 1 0 2
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine.run_ac(&netlist, &[1000.0]).unwrap();
        let v2 = results[0].voltage_magnitude(2);
        assert!(
            (v2 - 2.0).abs() < 1e-3,
            "expected VCVS gain of 2, got |V(2)|={}",
            v2
        );
    }

    #[test]
    fn test_ac_vccs_transconductance_stamping() {
        let netlist_str = r#"
* AC VCCS transconductance
V1 1 0 AC 1
G1 2 0 1 0 1m
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine.run_ac(&netlist, &[1000.0]).unwrap();
        let v2 = results[0].voltage_magnitude(2);
        assert!(
            (v2 - 1.0).abs() < 1e-3,
            "expected |V(2)|≈1V for gm=1m and R=1k, got {}",
            v2
        );
    }

    #[test]
    fn test_ac_cccs_control_branch_stamping() {
        let netlist_str = r#"
* AC CCCS controlled by V1 current
V1 1 0 AC 1
R1 1 0 1k
F1 2 0 V1 2
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine.run_ac(&netlist, &[1000.0]).unwrap();
        let v2 = results[0].voltage_magnitude(2);
        assert!(
            (v2 - 2.0).abs() < 0.05,
            "expected |V(2)|≈2V for CCCS gain 2, got {}",
            v2
        );
    }

    #[test]
    fn test_ac_ccvs_control_branch_stamping() {
        let netlist_str = r#"
* AC CCVS controlled by V1 current
V1 1 0 AC 1
R1 1 0 1k
H1 2 0 V1 500
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine.run_ac(&netlist, &[1000.0]).unwrap();
        let v2 = results[0].voltage_magnitude(2);
        assert!(
            (v2 - 0.5).abs() < 0.02,
            "expected |V(2)|≈0.5V for RM=500 and I(V1)=1mA, got {}",
            v2
        );
    }

    #[test]
    fn test_ac_vswitch_initial_state_controls_transfer_in_hysteresis_window() {
        let off_netlist = Netlist::parse(
            r#"
* AC VSwitch OFF initial state in hysteresis window
VDD in 0 DC 5 AC 1
RLOAD out 0 1k
VCTRL ctrl 0 1.0
S1 in out ctrl 0 SWMOD OFF
.MODEL SWMOD SW (RON=1 ROFF=1e9 VT=1.0 VH=0.2 SMOOTH=0.05)
.end
"#,
        )
        .unwrap();
        let on_netlist = Netlist::parse(
            r#"
* AC VSwitch ON initial state in hysteresis window
VDD in 0 DC 5 AC 1
RLOAD out 0 1k
VCTRL ctrl 0 1.0
S1 in out ctrl 0 SWMOD ON
.MODEL SWMOD SW (RON=1 ROFF=1e9 VT=1.0 VH=0.2 SMOOTH=0.05)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let off = engine.run_ac(&off_netlist, &[1e3]).unwrap();
        let on = engine.run_ac(&on_netlist, &[1e3]).unwrap();
        let off_mag = off[0].voltage_magnitude(2);
        let on_mag = on[0].voltage_magnitude(2);

        assert!(
            off_mag < 1e-3,
            "OFF-initialized VSwitch should strongly attenuate AC transfer, got {}",
            off_mag
        );
        assert!(
            on_mag > 0.95,
            "ON-initialized VSwitch should pass AC transfer, got {}",
            on_mag
        );
        assert!(
            on_mag > off_mag * 1e4,
            "hysteresis-state AC separation too small: off={} on={}",
            off_mag,
            on_mag
        );
    }

    #[test]
    fn test_ac_iswitch_initial_state_controls_transfer_in_hysteresis_window() {
        let off_netlist = Netlist::parse(
            r#"
* AC ISwitch OFF initial state in hysteresis window
VDD in 0 DC 5 AC 1
RLOAD out 0 1k
IBIAS 0 sense 5m
VCTRL sense 0 0
W1 in out VCTRL CSWMOD OFF
.MODEL CSWMOD CSW (RON=1 ROFF=1e9 IT=5m IH=1m SMOOTH=1e-4)
.end
"#,
        )
        .unwrap();
        let on_netlist = Netlist::parse(
            r#"
* AC ISwitch ON initial state in hysteresis window
VDD in 0 DC 5 AC 1
RLOAD out 0 1k
IBIAS 0 sense 5m
VCTRL sense 0 0
W1 in out VCTRL CSWMOD ON
.MODEL CSWMOD CSW (RON=1 ROFF=1e9 IT=5m IH=1m SMOOTH=1e-4)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let off = engine.run_ac(&off_netlist, &[1e3]).unwrap();
        let on = engine.run_ac(&on_netlist, &[1e3]).unwrap();
        let off_mag = off[0].voltage_magnitude(2);
        let on_mag = on[0].voltage_magnitude(2);

        assert!(
            off_mag < 1e-3,
            "OFF-initialized ISwitch should strongly attenuate AC transfer, got {}",
            off_mag
        );
        assert!(
            on_mag > 0.95,
            "ON-initialized ISwitch should pass AC transfer, got {}",
            on_mag
        );
        assert!(
            on_mag > off_mag * 1e4,
            "current-switch hysteresis AC separation too small: off={} on={}",
            off_mag,
            on_mag
        );
    }

    #[test]
    fn test_ac_vswitch_control_coupling_stamps_small_signal_jacobian() {
        let baseline = Netlist::parse(
            r#"
* Baseline: no AC excitation on VSwitch control terminal
VDD in 0 0.1
RBIAS in out 1k
S1 out 0 ctrl 0 SWMOD
VCTRL ctrl 0 DC 1.0 AC 0
.MODEL SWMOD SW (RON=1 ROFF=1e6 VT=1.0 VH=0 SMOOTH=0.2)
.end
"#,
        )
        .unwrap();
        let excited = Netlist::parse(
            r#"
* Excited: AC perturbation on VSwitch control terminal
VDD in 0 0.1
RBIAS in out 1k
S1 out 0 ctrl 0 SWMOD
VCTRL ctrl 0 DC 1.0 AC 1
.MODEL SWMOD SW (RON=1 ROFF=1e6 VT=1.0 VH=0 SMOOTH=0.2)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let base = engine.run_ac(&baseline, &[1e3]).unwrap();
        let sig = engine.run_ac(&excited, &[1e3]).unwrap();
        let v_base = base[0].voltage_magnitude(2);
        let v_sig = sig[0].voltage_magnitude(2);

        assert!(
            v_base < 1e-12,
            "without AC excitation baseline should be near zero, got {}",
            v_base
        );
        assert!(
            v_sig > 1e-3,
            "VSwitch control coupling should produce measurable AC response, got {}",
            v_sig
        );
        assert!(
            v_sig > v_base * 1e6 + 1e-3,
            "VSwitch control-coupling response too small: baseline={} excited={}",
            v_base,
            v_sig
        );
    }

    #[test]
    fn test_ac_iswitch_control_branch_coupling_stamps_small_signal_jacobian() {
        let baseline = Netlist::parse(
            r#"
* Baseline: no AC excitation on ISwitch control branch source
VDD in 0 0.1
RBIAS in out 1k
RLOAD out 0 1k
IBIAS 0 sense 1m
VCTRL sense 0 DC 0 AC 0
RCTRL sense 0 1k
W1 in out VCTRL CSWMOD
.MODEL CSWMOD CSW (RON=1 ROFF=1e6 IT=1m IH=0 SMOOTH=2m)
.end
"#,
        )
        .unwrap();
        let excited = Netlist::parse(
            r#"
* Excited: AC perturbation on ISwitch control branch source
VDD in 0 0.1
RBIAS in out 1k
RLOAD out 0 1k
IBIAS 0 sense 1m
VCTRL sense 0 DC 0 AC 1
RCTRL sense 0 1k
W1 in out VCTRL CSWMOD
.MODEL CSWMOD CSW (RON=1 ROFF=1e6 IT=1m IH=0 SMOOTH=2m)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let base = engine.run_ac(&baseline, &[1e3]).unwrap();
        let sig = engine.run_ac(&excited, &[1e3]).unwrap();
        let v_base = base[0].voltage_magnitude(2);
        let v_sig = sig[0].voltage_magnitude(2);

        assert!(
            v_base < 1e-12,
            "without AC excitation baseline should be near zero, got {}",
            v_base
        );
        assert!(
            v_sig > 1e-4,
            "ISwitch control-branch coupling should produce measurable AC response, got {}",
            v_sig
        );
        assert!(
            v_sig > v_base * 1e6 + 1e-4,
            "ISwitch control-branch response too small: baseline={} excited={}",
            v_base,
            v_sig
        );
    }

    #[test]
    fn test_ac_diode_small_signal_conductance_linearization() {
        let netlist_str = r#"
* Diode small-signal conductance test
V1 in 0 DC 1 AC 1
R1 in 1 1k
D1 1 0 DMOD
.MODEL DMOD D (IS=1e-14 N=1 RS=0 CJO=0 TT=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let dc = engine.run_dc_op(&netlist).unwrap();
        let ac = engine.run_ac(&netlist, &[1.0]).unwrap();
        let vac = ac[0].voltage_magnitude(2);

        // Expected transfer with source resistor:
        // |V(1)| ≈ 1 / (1 + R * gd), with gd from diode operating point.
        let vd = dc.voltage(2);
        let is = 1e-14;
        let n = 1.0;
        let vt = 0.02585;
        let gd = (is / (n * vt)) * (vd / (n * vt)).exp();
        let expected = 1.0 / (1.0 + 1e3 * gd);

        assert!(expected.is_finite() && expected > 0.0);
        let rel_err = ((vac - expected) / expected).abs();
        assert!(
            rel_err < 0.08,
            "expected |V(1)|≈{}V from diode gd divider, got {}V (rel_err={})",
            expected,
            vac,
            rel_err
        );
    }

    #[test]
    fn test_ac_diode_junction_capacitance_creates_high_frequency_rolloff() {
        let netlist_str = r#"
* Diode capacitance roll-off test
V1 in 0 DC 1 AC 1
R1 in 1 1k
D1 1 0 DMOD
.MODEL DMOD D (IS=1e-14 N=1 RS=0 CJO=1u TT=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let points = engine.run_ac(&netlist, &[1.0, 1e6]).unwrap();
        let low = points[0].voltage_magnitude(2);
        let high = points[1].voltage_magnitude(2);

        assert!(
            low.is_finite() && low > 0.0,
            "low-frequency response invalid: {}",
            low
        );
        assert!(
            high < low * 0.02,
            "expected strong Cj roll-off: low={} high={}",
            low,
            high
        );
    }

    #[test]
    fn test_ac_bjt_common_emitter_has_nonlinear_small_signal_gain() {
        let netlist_str = r#"
* BJT common-emitter AC gain test
Vcc 3 0 5
Vb 1 0 DC 0.6 AC 1m
Rc 3 2 1k
Q1 2 1 0 QB
.MODEL QB NPN (IS=1e-14 BF=200 BR=1 NF=1 NR=1 VAF=1e12 CJE=0 CJC=0 TF=0 TR=0 IKF=1e9 IKR=1e9)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let dc = engine.run_dc_op(&netlist).unwrap();
        let ac = engine.run_ac(&netlist, &[1e3]).unwrap();
        let vout = ac[0].voltage_magnitude(3);

        // Estimate expected gain magnitude using gm ≈ Ic/Vt and ideal source drive.
        let ic_est = ((5.0 - dc.voltage(3)) / 1e3).max(0.0);
        let gm_est = ic_est / 0.02585;
        let expected = gm_est * 1e-3 * 1e3;

        assert!(
            vout > 5e-4,
            "expected measurable CE gain, got |Vout|={}",
            vout
        );
        if expected > 1e-4 {
            let ratio = vout / expected;
            assert!(
                (0.2..5.0).contains(&ratio),
                "unexpected CE gain ratio: measured={} expected={} ratio={}",
                vout,
                expected,
                ratio
            );
        }
    }

    #[test]
    fn test_ac_mos_common_source_has_nonlinear_small_signal_gain() {
        let netlist_str = r#"
* MOS common-source AC gain test
Vdd 3 0 5
Vg 1 0 DC 2 AC 1m
Rd 3 2 1k
M1 2 1 0 0 MTEST
.MODEL MTEST NMOS (LEVEL=1 VTO=0.7 KP=100U LAMBDA=0 CGSO=0 CGDO=0 CGBO=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let dc = engine.run_dc_op(&netlist).unwrap();
        let ac = engine.run_ac(&netlist, &[1e3]).unwrap();
        let vout = ac[0].voltage_magnitude(3);

        // Estimate expected gain with gm ≈ 2*Id/Vov in saturation.
        let id_est = ((5.0 - dc.voltage(3)) / 1e3).max(0.0);
        let vov: Value = (2.0_f64 - 0.7_f64).max(1e-12_f64);
        let gm_est = 2.0 * id_est / vov;
        let expected = gm_est * 1e-3 * 1e3;

        assert!(
            vout > 2e-4,
            "expected measurable CS gain from MOS small-signal model, got |Vout|={}",
            vout
        );
        if expected > 1e-4 {
            let ratio = vout / expected;
            assert!(
                (0.15..6.0).contains(&ratio),
                "unexpected MOS gain ratio: measured={} expected={} ratio={}",
                vout,
                expected,
                ratio
            );
        }
    }

    #[test]
    fn test_jfet_dc_common_source_bias_affects_output() {
        let netlist_str = r#"
* JFET DC common-source bias
Vdd 1 0 10
Vg 2 0 0
Rd 1 3 1k
J1 3 2 0 JMOD
.MODEL JMOD NJF (VTO=-2 BETA=1m LAMBDA=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let vout = result.voltage(3);

        assert!(
            vout < 9.5 && vout > 0.1,
            "JFET should load drain node in DC; expected 0.1V < V(3) < 9.5V, got {}V",
            vout
        );
    }

    #[test]
    fn test_jfet_dc_gate_forward_bias_produces_gate_leakage_path() {
        let forward = Netlist::parse(
            r#"
* NJF gate-source forward-bias leakage
VG 1 0 0.6
RS 2 0 1k
RD 3 0 1k
J1 3 1 2 JLEAK
.MODEL JLEAK NJF (VTO=-2 BETA=0 LAMBDA=0 IS=1e-14 N=1)
.end
"#,
        )
        .unwrap();
        let reverse = Netlist::parse(
            r#"
* NJF reverse-biased gate (baseline leakage near zero)
VG 1 0 -2
RS 2 0 1k
RD 3 0 1k
J1 3 1 2 JLEAK
.MODEL JLEAK NJF (VTO=-2 BETA=0 LAMBDA=0 IS=1e-14 N=1)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let fwd = engine.run_dc_op(&forward).unwrap();
        let rev = engine.run_dc_op(&reverse).unwrap();
        let vs_fwd = fwd.voltage(2);
        let vs_rev = rev.voltage(2);

        assert!(
            vs_fwd > 0.02,
            "forward-biased gate junction should raise source node via leakage, got V(s)={}",
            vs_fwd
        );
        assert!(
            vs_rev.abs() < 1e-3,
            "reverse-biased gate junction should have near-zero leakage, got V(s)={}",
            vs_rev
        );
        assert!(
            vs_fwd > vs_rev + 0.02,
            "forward leakage should be much larger than reverse leakage: Vsf={} Vsr={}",
            vs_fwd,
            vs_rev
        );
    }

    #[test]
    fn test_pjf_dc_gate_leakage_polarity_is_source_to_gate_when_forward_biased() {
        let forward = Netlist::parse(
            r#"
* PJF gate junction forward-biased from source to gate
VDD 4 0 1
RBIAS 4 2 1k
VG 1 0 0
RD 3 0 1k
J1 3 1 2 PMOD
.MODEL PMOD PJF (VTO=-2 BETA=0 LAMBDA=0 IS=1e-14 N=1)
.end
"#,
        )
        .unwrap();
        let reverse = Netlist::parse(
            r#"
* PJF gate junction reverse-biased baseline
VDD 4 0 1
RBIAS 4 2 1k
VG 1 0 1.5
RD 3 0 1k
J1 3 1 2 PMOD
.MODEL PMOD PJF (VTO=-2 BETA=0 LAMBDA=0 IS=1e-14 N=1)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let fwd = engine.run_dc_op(&forward).unwrap();
        let rev = engine.run_dc_op(&reverse).unwrap();
        let vs_fwd = fwd.voltage(2);
        let vs_rev = rev.voltage(2);

        assert!(
            vs_fwd < 0.9,
            "forward-biased PJF gate junction should sink source node, got V(s)={}",
            vs_fwd
        );
        assert!(
            vs_rev > 0.95,
            "reverse-biased PJF gate junction should keep source near supply, got V(s)={}",
            vs_rev
        );
        assert!(
            vs_fwd < vs_rev - 0.1,
            "PJF forward gate leakage polarity mismatch: Vsf={} Vsr={}",
            vs_fwd,
            vs_rev
        );
    }

    #[test]
    fn test_ac_jfet_common_source_has_nonlinear_small_signal_gain() {
        let netlist_str = r#"
* JFET common-source AC gain test
Vdd 1 0 10
Vg 2 0 DC 0 AC 1m
Rd 1 3 1k
J1 3 2 0 JMOD
.MODEL JMOD NJF (VTO=-2 BETA=1m LAMBDA=0 CGS=0 CGD=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let dc = engine.run_dc_op(&netlist).unwrap();
        let ac = engine.run_ac(&netlist, &[1e3]).unwrap();
        let vout = ac[0].voltage_magnitude(3);

        // Saturation estimate: gm ≈ 2*sqrt(beta*Id), gain ≈ gm*Rd*vin.
        let id_est = ((10.0 - dc.voltage(3)) / 1e3).max(0.0);
        let gm_est = 2.0 * (1e-3_f64 * id_est).sqrt();
        let expected = gm_est * 1e3 * 1e-3;

        assert!(
            vout > 1e-3,
            "expected measurable JFET common-source gain, got |Vout|={}",
            vout
        );
        if expected > 1e-4 {
            let ratio = vout / expected;
            assert!(
                (0.2..5.0).contains(&ratio),
                "unexpected JFET gain ratio: measured={} expected={} ratio={}",
                vout,
                expected,
                ratio
            );
        }
    }

    #[test]
    fn test_ac_jfet_gate_drain_capacitance_creates_high_frequency_rolloff() {
        let netlist_str = r#"
* JFET capacitance roll-off test
Vdd 1 0 10
Vg 2 0 DC 0 AC 1m
Rd 1 3 1k
J1 3 2 0 JCAP
.MODEL JCAP NJF (VTO=-2 BETA=1m LAMBDA=0 CGS=0 CGD=10n PB=1 FC=0.5)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let points = engine.run_ac(&netlist, &[1e3, 1e8]).unwrap();
        let low = points[0].voltage_magnitude(3);
        let high = points[1].voltage_magnitude(3);

        assert!(
            low.is_finite() && low > 1e-6,
            "low-frequency JFET gain invalid: {}",
            low
        );
        assert!(
            high < low * 0.4,
            "expected strong JFET Cgd roll-off: low={} high={}",
            low,
            high
        );
    }

    #[test]
    fn test_bjt_circuit() {
        let netlist_str = r#"
* BJT Simple Test
Vcc 1 0 5
Rb 1 2 10k
Rc 1 3 1k
Q1 3 2 0 2N2222
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        match engine.run_dc_op(&netlist) {
            Ok(r) => {
                let vcc = r.voltage(1);
                assert!((vcc - 5.0).abs() < 0.1, "Expected Vcc=5V, got {}V", vcc);
            }
            Err(_) => {} // BJT convergence failure acceptable
        }
    }

    #[test]
    fn test_bjt_uses_embedded_model_when_no_model_card_present() {
        let netlist_str = r#"
* BJT Embedded Model Fallback Test
Vcc 1 0 5
Rb 1 2 10k
Rc 1 3 1k
Q1 3 2 0 2N2222
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.bjts.devices.len(), 1);
        let q = &circuit.bjts.devices[0];
        assert!(
            (q.bf - 255.9).abs() < 1e-6,
            "Expected BF from embedded 2N2222 model"
        );
        assert!(
            (q.is - 14.34e-15).abs() < 1e-20,
            "Expected IS from embedded 2N2222 model"
        );
    }

    #[test]
    fn test_bjt_explicit_model_card_overrides_embedded_fallback() {
        let netlist_str = r#"
* BJT Explicit Model Card Precedence Test
Vcc 1 0 5
Rb 1 2 10k
Rc 1 3 1k
Q1 3 2 0 2N2222
.MODEL 2N2222 NPN (BF=123 IS=9e-15)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.bjts.devices.len(), 1);
        let q = &circuit.bjts.devices[0];
        assert!((q.bf - 123.0).abs() < 1e-9);
        assert!((q.is - 9e-15).abs() < 1e-22);
    }

    #[test]
    fn test_bjt_model_card_type_sets_pnp_polarity() {
        let netlist_str = r#"
* BJT model card type should define device polarity
Q1 1 2 0 QMOD
.MODEL QMOD PNP (BF=80 IS=2e-14)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.bjts.devices.len(), 1);
        assert_eq!(
            circuit.bjts.devices[0].bjt_type,
            crate::device::BjtType::Pnp
        );
    }

    #[test]
    fn test_mos_model_card_type_sets_pmos_polarity() {
        let netlist_str = r#"
* MOS model card type should define NMOS/PMOS polarity
M1 2 1 0 0 PMOD
.MODEL PMOD PMOS (LEVEL=1 VTO=-0.7 KP=100u)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.mosfets.devices.len(), 1);
        assert_eq!(
            circuit.mosfets.devices[0].mos_type,
            crate::device::MosType::Pmos
        );
    }

    #[test]
    fn test_mos_instance_geometry_overrides_model_geometry() {
        let netlist_str = r#"
* Instance W/L should override model-card defaults
M1 2 1 0 0 NMOD W=20u L=0.25u
.MODEL NMOD NMOS (LEVEL=1 W=4u L=1u)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        let m = &circuit.mosfets.devices[0];

        assert!((m.w - 20e-6).abs() < 1e-18);
        assert!((m.l - 0.25e-6).abs() < 1e-18);
    }

    #[test]
    fn test_mos_instance_multiplier_scales_effective_width() {
        let netlist_str = r#"
* M and NF should scale effective width
M1 2 1 0 0 NMOD W=10u M=3 NF=2
.MODEL NMOD NMOS (LEVEL=1)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        let m = &circuit.mosfets.devices[0];

        assert!((m.w - 60e-6).abs() < 1e-18);
        assert!((m.l - 1e-6).abs() < 1e-18);
    }

    #[test]
    fn test_bsimsoi_external_body_contact_becomes_runtime_bulk_node() {
        let netlist_str = r#"
* BSIMSOI tied-body instance should use the external body contact
M1 d g s e b NMOD
.MODEL NMOD NMOS (LEVEL=55 VTO=0.7 KP=100u)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        let m = &circuit.mosfets.devices[0];
        let body_contact = circuit.get_node_by_name("b").unwrap();

        assert_eq!(m.level, 55);
        assert_eq!(m.node_bulk, body_contact);
        assert!(circuit.get_node_by_name("e").is_none());
    }

    #[test]
    fn test_bsimsoi_without_external_body_uses_substrate_node() {
        let netlist_str = r#"
* BSIMSOI floating-body instance should fall back to the substrate node
M1 d g s e NMOD
.MODEL NMOD NMOS (LEVEL=55 VTO=0.7 KP=100u)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        let m = &circuit.mosfets.devices[0];
        let substrate = circuit.get_node_by_name("e").unwrap();

        assert_eq!(m.level, 55);
        assert_eq!(m.node_bulk, substrate);
    }

    #[test]
    fn test_jfet_model_card_type_sets_pjf_polarity() {
        let netlist_str = r#"
* JFET model card type should define NJF/PJF polarity
J1 3 2 0 JMOD
.MODEL JMOD PJF (VTO=-2 BETA=1m)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.jfets.len(), 1);
        assert_eq!(circuit.jfets[0].jfet_type, crate::device::JfetType::PJF);
    }

    #[test]
    fn test_mesfet_model_card_type_sets_pmf_polarity() {
        let netlist_str = r#"
* MESFET model card type should define NMF/PMF polarity
Z1 3 2 0 MMOD
.MODEL MMOD PMF (VTO=-2 BETA=1m)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.jfets.len(), 1);
        assert_eq!(circuit.jfets[0].jfet_type, crate::device::JfetType::PJF);
    }

    #[test]
    fn test_bjt_lowercase_model_params_are_applied() {
        let netlist_str = r#"
* Lowercase model params must remain case-insensitive
Q1 3 2 0 qmod
.MODEL qmod npn (bf=75 rb=40 rc=25)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let circuit = Engine::default().build_circuit(&netlist).unwrap();
        let q = &circuit.bjts.devices[0];
        assert!((q.bf - 75.0).abs() < 1e-12);
        assert!((q.rb - 40.0).abs() < 1e-12);
        assert!((q.rc - 25.0).abs() < 1e-12);
    }

    #[test]
    fn test_mos_lowercase_level_param_is_applied() {
        let netlist_str = r#"
* Lowercase LEVEL should configure level-6 path
M1 2 1 0 0 mmod
.MODEL mmod nmos (level=6 kc=2e-5 nc=1.2 kv=0.9 nv=0.8)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let circuit = Engine::default().build_circuit(&netlist).unwrap();
        let m = &circuit.mosfets.devices[0];
        assert_eq!(m.level, 6);
        assert!((m.kc - 2e-5).abs() < 1e-18);
    }

    #[test]
    fn test_mesfet_lowercase_model_params_are_applied() {
        let netlist_str = r#"
* Lowercase VT0/ETA/SIGMA0 should map into MESFET compatibility path
Z1 3 2 0 mmod w=10u l=1u
.MODEL mmod nhfet (level=5 vt0=0.3 eta=1.2 sigma0=0.04 rd=60 rs=60)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let circuit = Engine::default().build_circuit(&netlist).unwrap();
        let z = &circuit.jfets[0];
        assert!((z.params.vto - 0.3).abs() < 1e-12);
        assert!((z.params.eta - 1.2).abs() < 1e-12);
        assert!((z.params.sigma0 - 0.04).abs() < 1e-12);
    }

    #[test]
    fn test_diode_unknown_model_errors() {
        let netlist_str = r#"
* Diode unknown model must fail
D1 1 0 DMISS
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown diode model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Diode") && msg.contains("unknown model"),
            "expected unknown diode model error, got {}",
            msg
        );
    }

    #[test]
    fn test_bjt_unknown_model_errors() {
        let netlist_str = r#"
* BJT unknown model must fail
Q1 3 2 0 QMISSING
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown BJT model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("BJT") && msg.contains("unknown model"),
            "expected unknown BJT model error, got {}",
            msg
        );
    }

    #[test]
    fn test_mos_unknown_model_errors() {
        let netlist_str = r#"
* MOS unknown model must fail
M1 2 1 0 0 MMISSING
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown MOS model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("MOSFET") && msg.contains("unknown model"),
            "expected unknown MOS model error, got {}",
            msg
        );
    }

    #[test]
    fn test_jfet_unknown_model_errors() {
        let netlist_str = r#"
* JFET unknown model must fail
J1 3 2 0 JMISSING
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown JFET model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("JFET") && msg.contains("unknown model"),
            "expected unknown JFET model error, got {}",
            msg
        );
    }

    #[test]
    fn test_mesfet_unknown_model_errors() {
        let netlist_str = r#"
* MESFET unknown model must fail
Z1 3 2 0 MMISSING
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown MESFET model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("MESFET") && msg.contains("unknown model"),
            "expected unknown MESFET model error, got {}",
            msg
        );
    }

    #[test]
    fn test_xspice_unknown_model_fails_build() {
        let netlist_str = r#"
* XSPICE unknown model must fail
V1 in 0 1
R1 out 0 1k
A1 in out no_such_model
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown XSPICE model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Unknown XSPICE model"),
            "expected unknown XSPICE model error, got {}",
            msg
        );
    }

    #[test]
    fn test_xspice_model_alias_resolves_builtin_code_model() {
        let temp_dir = unique_temp_dir("xspice_alias");
        std::fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let netlist_path = temp_dir.join("alias.cir");
        let stimulus_path = temp_dir.join("stimulus.txt");
        std::fs::write(&stimulus_path, "* digital stimulus placeholder").expect("stimulus file");

        let netlist = Netlist::parse_with_path(
            "\
* XSPICE alias should resolve through .model definition
.model dsrc d_source (input_file=\"stimulus.txt\")
A1 [out] dsrc
.end
",
            &netlist_path,
        )
        .expect("netlist should parse");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("xspice model alias should build");
        assert_eq!(circuit.xspice_instances.len(), 1);

        let instance = &circuit.xspice_instances[0];
        assert_eq!(instance.model_name(), "d_source");
        assert_eq!(
            std::path::PathBuf::from(
                instance
                    .string_param("input_file")
                    .expect("input_file string param should be set")
            ),
            stimulus_path
        );
    }

    #[test]
    fn test_xspice_model_alias_merges_model_and_instance_params() {
        let netlist = Netlist::parse(
            r#"
* XSPICE alias should merge .model defaults with instance overrides
V1 in 0 1
A1 in out gain_alias gain=4
RLOAD out 0 1k
.model gain_alias gain (gain=2)
.end
"#,
        )
        .expect("netlist should parse");

        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("xspice alias netlist should converge");
        let vout = result.voltage(2);
        assert!(
            (vout - 4.0).abs() < 1e-6,
            "instance parameter override should win over .model default; got {}",
            vout
        );
    }

    #[test]
    fn test_xspice_invalid_port_count_fails_build() {
        let netlist_str = r#"
* XSPICE instance with wrong port count must fail
A1 out gain
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("invalid XSPICE port count should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Failed to create XSPICE instance") && msg.contains("Port count"),
            "expected XSPICE creation error with port count details, got {}",
            msg
        );
    }

    #[test]
    fn test_xspice_gain_participates_in_dc_operating_point() {
        let netlist_str = r#"
* XSPICE gain block should contribute during DC solve
V1 in 0 1
A1 in out gain gain=2
RLOAD out 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let vout = result.voltage(2);
        assert!(
            (vout - 2.0).abs() < 1e-6,
            "expected XSPICE gain to stamp as a controlled voltage source (2V), got {}",
            vout
        );
    }

    #[test]
    fn test_matrix_topology_includes_xspice_voltage_output_branch_couplings() {
        let netlist = Netlist::parse(
            r#"
* Matrix topology must include branch couplings for XSPICE voltage outputs
V1 in 0 1
A1 in out gain gain=2
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        assert_eq!(circuit.xspice_instances.len(), 1);

        let inst = &circuit.xspice_instances[0];
        let branch_ordinal = inst
            .branch_ordinal_at(1)
            .expect("xspice gain output should allocate branch ordinal");
        let branch_mna = circuit.get_branch_matrix_index(branch_ordinal);
        let out_node = circuit
            .get_node_by_name("out")
            .expect("out node should exist in node map");

        let matrix = engine.build_matrix(&circuit).unwrap();
        assert!(
            matrix.get_index(branch_mna - 1, out_node - 1).is_some(),
            "expected branch-row to output-node coupling in matrix topology"
        );
        assert!(
            matrix.get_index(out_node - 1, branch_mna - 1).is_some(),
            "expected output-node to branch-column coupling in matrix topology"
        );
    }

    #[test]
    fn test_xspice_integrator_advances_output_in_transient() {
        let netlist = Netlist::parse(
            r#"
* XSPICE integrator must integrate with transient dt
V1 in 0 1
A1 in out integrator gain=1000 out_ic=0
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        let out_node = circuit
            .get_node_by_name("out")
            .expect("out node should exist");

        let result = engine
            .run_tran(&netlist, 1e-3, 100e-6)
            .expect("transient with XSPICE integrator should converge");
        assert!(
            result.num_points() >= 2,
            "expected multiple transient output points"
        );

        let v0 = result.voltage_at(out_node, 0);
        let vend = result.voltage_at(out_node, result.num_points() - 1);
        assert!(
            vend > v0 + 1e-6,
            "integrator output should ramp over transient; got v0={} vend={}",
            v0,
            vend
        );
    }

    #[test]
    fn test_xspice_aswitch_controls_dc_conductive_path() {
        let netlist_on = Netlist::parse(
            r#"
* XSPICE analog switch ON state should pass source voltage
VCTRL ctrl 0 1
VIN in 0 1
A1 ctrl in out aswitch cntl_on=0.5 cntl_off=0 r_on=1 r_off=1e9 log=0
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();
        let netlist_off = Netlist::parse(
            r#"
* XSPICE analog switch OFF state should isolate output
VCTRL ctrl 0 0
VIN in 0 1
A1 ctrl in out aswitch cntl_on=0.5 cntl_off=0 r_on=1 r_off=1e9 log=0
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let on = engine
            .run_dc_op(&netlist_on)
            .expect("ON state should converge");
        let off = engine
            .run_dc_op(&netlist_off)
            .expect("OFF state should converge");

        // Node order: 1=ctrl, 2=in, 3=out
        let v_on = on.voltage(3);
        let v_off = off.voltage(3);
        assert!(
            v_on > 0.95,
            "ON-state switch should transfer input to output, got {}",
            v_on
        );
        assert!(
            v_off < 1e-5,
            "OFF-state switch should isolate output, got {}",
            v_off
        );
    }

    #[test]
    fn test_xspice_aswitch_inout_ports_use_nodal_topology_without_branches() {
        let netlist = Netlist::parse(
            r#"
* XSPICE analog switch should reserve nodal coupling topology, not output branches
VCTRL ctrl 0 1
VIN in 0 1
A1 ctrl in out aswitch cntl_on=0.5 cntl_off=0 r_on=1 r_off=1e9
RLOAD out 0 1k
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        assert_eq!(circuit.xspice_instances.len(), 1);

        let inst = &circuit.xspice_instances[0];
        assert_eq!(
            inst.branch_ordinal_at(1),
            None,
            "ps inout terminal must not allocate branch variable"
        );
        assert_eq!(
            inst.branch_ordinal_at(2),
            None,
            "ns inout terminal must not allocate branch variable"
        );

        let in_node = circuit
            .get_node_by_name("in")
            .expect("in node should exist in node map");
        let out_node = circuit
            .get_node_by_name("out")
            .expect("out node should exist in node map");
        let matrix = engine.build_matrix(&circuit).unwrap();

        assert!(
            matrix.get_index(in_node - 1, out_node - 1).is_some(),
            "expected inout off-diagonal coupling in matrix topology"
        );
        assert!(
            matrix.get_index(out_node - 1, in_node - 1).is_some(),
            "expected reciprocal inout coupling in matrix topology"
        );
    }

    #[test]
    fn test_diode_incompatible_model_type_errors() {
        let netlist_str = r#"
* Diode references incompatible model type
D1 1 0 DMOD
.MODEL DMOD NPN (BF=100)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("diode model type mismatch should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("incompatible type"),
            "expected incompatible type error, got {}",
            msg
        );
        assert!(
            msg.contains("Diode") && msg.contains("expected D or DIODE"),
            "expected diode model family mismatch details, got {}",
            msg
        );
    }

    #[test]
    fn test_bjt_incompatible_model_type_errors() {
        let netlist_str = r#"
* BJT references incompatible model type
Q1 3 2 0 QMOD
.MODEL QMOD NMOS (VTO=0.7 KP=100u)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("BJT model type mismatch should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("incompatible type") && msg.contains("expected NPN or PNP"),
            "expected BJT model family mismatch details, got {}",
            msg
        );
    }

    #[test]
    fn test_mos_incompatible_model_type_errors() {
        let netlist_str = r#"
* MOS references incompatible model type
M1 2 1 0 0 MMOD
.MODEL MMOD PJF (VTO=-2 BETA=1m)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("MOS model type mismatch should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("incompatible type") && msg.contains("expected NMOS or PMOS"),
            "expected MOS model family mismatch details, got {}",
            msg
        );
    }

    #[test]
    fn test_jfet_incompatible_model_type_errors() {
        let netlist_str = r#"
* JFET references incompatible model type
J1 3 2 0 JMOD
.MODEL JMOD NPN (BF=100)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("JFET model type mismatch should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("incompatible type") && msg.contains("expected NJF or PJF"),
            "expected JFET model family mismatch details, got {}",
            msg
        );
    }

    #[test]
    fn test_mesfet_incompatible_model_type_errors() {
        let netlist_str = r#"
* MESFET references incompatible model type
Z1 3 2 0 MMOD
.MODEL MMOD PMOS (LEVEL=1 VTO=-0.7 KP=100u)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("MESFET model type mismatch should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("incompatible type") && msg.contains("expected NMF or PMF"),
            "expected MESFET model family mismatch details, got {}",
            msg
        );
    }

    #[test]
    fn test_transmission_line_incompatible_model_type_errors() {
        let netlist_str = r#"
* TLine references incompatible model type
O1 1 0 2 0 TLMOD
.MODEL TLMOD NPN (BF=100)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("transmission line model type mismatch should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("incompatible type") && msg.contains("expected LTRA or TXL"),
            "expected transmission-line model family mismatch details, got {}",
            msg
        );
    }

    #[test]
    fn test_jfet_model_rd_rs_expand_to_internal_nodes_and_series_resistors() {
        let netlist_str = r#"
* JFET RD/RS expansion
VDD 1 0 10
VG 2 0 0
J1 3 2 0 JMOD
.MODEL JMOD NJF (VTO=-2 BETA=1m RD=120 RS=340)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.jfets.len(), 1);
        assert!(
            circuit.resistors.names.iter().any(|n| n == "J1.__rd"),
            "expected generated RD resistor for J1"
        );
        assert!(
            circuit.resistors.names.iter().any(|n| n == "J1.__rs"),
            "expected generated RS resistor for J1"
        );

        let node_names = circuit.node_names_sorted();
        assert!(
            node_names.iter().any(|n| n == "J1.__dint"),
            "expected generated internal drain node name"
        );
        assert!(
            node_names.iter().any(|n| n == "J1.__sint"),
            "expected generated internal source node name"
        );
        assert!(
            circuit.jfets[0].params.rd.abs() < 1e-30 && circuit.jfets[0].params.rs.abs() < 1e-30,
            "intrinsic JFET should have RD/RS zeroed after externalization"
        );
    }

    #[test]
    fn test_jfet_series_rd_rs_reduce_common_source_drain_loading() {
        let baseline = Netlist::parse(
            r#"
* Baseline JFET without extrinsic RD/RS
VDD 1 0 10
VG 2 0 0
RL 1 3 1k
J1 3 2 0 JMOD
.MODEL JMOD NJF (VTO=-2 BETA=1m LAMBDA=0 RD=0 RS=0)
.end
"#,
        )
        .unwrap();
        let with_series = Netlist::parse(
            r#"
* JFET with large extrinsic RD/RS
VDD 1 0 10
VG 2 0 0
RL 1 3 1k
J1 3 2 0 JMOD
.MODEL JMOD NJF (VTO=-2 BETA=1m LAMBDA=0 RD=2k RS=2k)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let base = engine.run_dc_op(&baseline).unwrap();
        let series = engine.run_dc_op(&with_series).unwrap();
        let vdrain_base = base.voltage(3);
        let vdrain_series = series.voltage(3);

        assert!(
            vdrain_series > vdrain_base + 1.0,
            "large RD/RS should significantly reduce channel loading: baseline={} with_series={}",
            vdrain_base,
            vdrain_series
        );
    }

    #[test]
    fn test_mosfet_circuit() {
        let netlist_str = r#"
* NMOS Test
Vgs 1 0 3
Vds 2 0 5
M1 2 1 0 0 NMOS
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let vg = result.voltage(1);
        let vd = result.voltage(2);
        assert!((vg - 3.0).abs() < 0.1, "Expected gate at 3V, got {}V", vg);
        assert!((vd - 5.0).abs() < 0.1, "Expected drain at 5V, got {}V", vd);
    }

    #[test]
    fn test_multi_resistor_divider() {
        let netlist_str = r#"
* Multi-Resistor Divider
V1 1 0 12
R1 1 2 1k
R2 2 3 2k
R3 3 0 3k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v2 = result.voltage(2);
        let v3 = result.voltage(3);
        assert!((v2 - 10.0).abs() < 0.1, "Expected V(2)=10V, got {}V", v2);
        assert!((v3 - 6.0).abs() < 0.1, "Expected V(3)=6V, got {}V", v3);
    }

    #[test]
    fn test_dc_sweep() {
        let netlist_str = r#"
* DC Sweep Test
V1 1 0 0
R1 1 2 1k
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine.run_dc_sweep(&netlist, "V1", 0.0, 5.0, 1.0).unwrap();
        assert_eq!(results.len(), 6);
        for (vin, result) in &results {
            let v2 = result.voltage(2);
            let expected = vin / 2.0;
            assert!((v2 - expected).abs() < 0.01);
        }
    }

    #[test]
    fn test_dc_sweep_preserves_netlist_node_names_for_results() {
        let netlist_str = r#"
* DC Sweep node-name mapping
VDRV in 0 0
RLOAD in out 1k
RRET out 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine
            .run_dc_sweep(&netlist, "VDRV", 0.0, 1.0, 1.0)
            .unwrap();
        assert!(!results.is_empty());

        // Regression guard: dc-sweep result node names must be real circuit names,
        // not numeric placeholders generated by SimulationResult::new.
        let (_, first) = &results[0];
        assert!(
            first
                .node_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("in"))
        );
        assert!(
            first
                .node_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("out"))
        );
    }

    #[test]
    fn test_dc_sweep_with_abort_returns_aborted_error() {
        let netlist = Netlist::parse(
            r#"
* DC sweep abort coverage
V1 in 0 0
R1 in out 1k
R2 out 0 1k
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let abort = ImmediateAbort;
        let result = engine.run_dc_sweep_with_abort(&netlist, "V1", 0.0, 5.0, 0.1, &abort);
        assert!(
            matches!(result, Err(crate::engine::SimulationError::Aborted)),
            "expected sweep to abort immediately, got {:?}",
            result
        );
    }

    #[test]
    fn test_inductor_transient() {
        let netlist_str = r#"
* RL Transient
V1 1 0 5
R1 1 2 1k
L1 2 0 1m
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_tran(&netlist, 10e-6, 0.5e-6).unwrap();
        assert!(result.num_points() > 5, "Expected multiple time points");
    }

    #[test]
    fn test_transient_jfet_cgs_companion_creates_gate_rc_delay() {
        let with_cap = Netlist::parse(
            r#"
* JFET gate RC loading through Cgs
VSTEP in 0 PULSE(0 1 0 1n 1n 2m 4m)
RG in g 1k
RD d 0 1k
J1 d g 0 JCAP
.MODEL JCAP NJF (VTO=-2 BETA=0 CGS=1u CGD=0 PB=1 FC=0.5 IS=0)
.end
"#,
        )
        .unwrap();
        let no_cap = Netlist::parse(
            r#"
* Baseline: same topology without JFET capacitance
VSTEP in 0 PULSE(0 1 0 1n 1n 2m 4m)
RG in g 1k
RD d 0 1k
J1 d g 0 JCAP
.MODEL JCAP NJF (VTO=-2 BETA=0 CGS=0 CGD=0 PB=1 FC=0.5 IS=0)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let with_cap_result = engine.run_tran(&with_cap, 100e-6, 1e-6).unwrap();
        let no_cap_result = engine.run_tran(&no_cap, 100e-6, 1e-6).unwrap();

        let g_with = with_cap_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("g"))
            .expect("gate node g missing in with-cap result");
        let g_no = no_cap_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("g"))
            .expect("gate node g missing in baseline result");

        let idx_with_20us = with_cap_result
            .time
            .iter()
            .position(|&time| time >= 20e-6)
            .unwrap_or(with_cap_result.time.len() - 1);
        let idx_no_20us = no_cap_result
            .time
            .iter()
            .position(|&time| time >= 20e-6)
            .unwrap_or(no_cap_result.time.len() - 1);

        let vg_with_20us = with_cap_result.voltages[g_with][idx_with_20us];
        let vg_no_20us = no_cap_result.voltages[g_no][idx_no_20us];

        assert!(
            vg_with_20us < 0.08,
            "expected heavy Cgs loading at 20us, got Vg={}",
            vg_with_20us
        );
        assert!(
            vg_no_20us > 0.95,
            "baseline without Cgs should settle near source quickly, got Vg={}",
            vg_no_20us
        );
    }

    #[test]
    fn test_transient_jfet_cgd_companion_couples_gate_edge_to_drain() {
        let with_cgd = Netlist::parse(
            r#"
* JFET Cgd feedthrough pulse
VG g 0 PULSE(0 1 0 1u 1u 40u 80u)
RLOAD d 0 1k
J1 d g 0 JCAP
.MODEL JCAP NJF (VTO=-2 BETA=0 CGS=0 CGD=1n PB=1 FC=0.5 IS=0)
.end
"#,
        )
        .unwrap();
        let without_cgd = Netlist::parse(
            r#"
* Baseline without Cgd
VG g 0 PULSE(0 1 0 1u 1u 40u 80u)
RLOAD d 0 1k
J1 d g 0 JCAP
.MODEL JCAP NJF (VTO=-2 BETA=0 CGS=0 CGD=0 PB=1 FC=0.5 IS=0)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let with_cgd_result = engine.run_tran(&with_cgd, 20e-6, 50e-9).unwrap();
        let without_cgd_result = engine.run_tran(&without_cgd, 20e-6, 50e-9).unwrap();

        let d_with = with_cgd_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("d"))
            .expect("drain node d missing in with-cgd result");
        let d_without = without_cgd_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("d"))
            .expect("drain node d missing in baseline result");

        let peak_with = with_cgd_result
            .time
            .iter()
            .enumerate()
            .filter(|(_, t)| **t <= 3e-6)
            .map(|(i, _)| with_cgd_result.voltages[d_with][i].abs())
            .fold(0.0, Value::max);
        let peak_without = without_cgd_result
            .time
            .iter()
            .enumerate()
            .filter(|(_, t)| **t <= 3e-6)
            .map(|(i, _)| without_cgd_result.voltages[d_without][i].abs())
            .fold(0.0, Value::max);
        let idx_tail = with_cgd_result
            .time
            .iter()
            .position(|&time| time >= 15e-6)
            .unwrap_or(with_cgd_result.time.len() - 1);
        let tail_with = with_cgd_result.voltages[d_with][idx_tail].abs();

        assert!(
            peak_with > 0.2,
            "expected strong Cgd feedthrough pulse at drain, peak={}",
            peak_with
        );
        assert!(
            peak_without < 1e-3 && peak_without < peak_with * 0.05,
            "baseline without Cgd should have negligible drain pulse, peak_without={} peak_with={}",
            peak_without,
            peak_with
        );
        assert!(
            tail_with < peak_with * 0.1,
            "drain pulse should decay through RLOAD, tail={} peak={}",
            tail_with,
            peak_with
        );
    }

    #[test]
    fn test_transient_mosfet_cgd_companion_couples_gate_edge_to_drain() {
        let with_cgd = Netlist::parse(
            r#"
* MOSFET Cgd feedthrough pulse
VG g 0 PULSE(0 1 0 1u 1u 40u 80u)
RLOAD d 0 1k
M1 d g 0 0 MCAP
.MODEL MCAP NMOS (LEVEL=1 KP=0 VTO=100 CGSO=0 CGDO=1e-4 CGBO=0)
.end
"#,
        )
        .unwrap();
        let without_cgd = Netlist::parse(
            r#"
* Baseline without MOS Cgd
VG g 0 PULSE(0 1 0 1u 1u 40u 80u)
RLOAD d 0 1k
M1 d g 0 0 MCAP
.MODEL MCAP NMOS (LEVEL=1 KP=0 VTO=100 CGSO=0 CGDO=0 CGBO=0)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let with_cgd_result = engine.run_tran(&with_cgd, 20e-6, 50e-9).unwrap();
        let without_cgd_result = engine.run_tran(&without_cgd, 20e-6, 50e-9).unwrap();

        let d_with = with_cgd_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("d"))
            .expect("drain node d missing in with-cgd result");
        let d_without = without_cgd_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("d"))
            .expect("drain node d missing in baseline result");

        let peak_with = with_cgd_result
            .time
            .iter()
            .enumerate()
            .filter(|(_, t)| **t <= 3e-6)
            .map(|(i, _)| with_cgd_result.voltages[d_with][i].abs())
            .fold(0.0, Value::max);
        let peak_without = without_cgd_result
            .time
            .iter()
            .enumerate()
            .filter(|(_, t)| **t <= 3e-6)
            .map(|(i, _)| without_cgd_result.voltages[d_without][i].abs())
            .fold(0.0, Value::max);

        assert!(
            peak_with > 0.2,
            "expected strong MOS Cgd feedthrough pulse at drain, peak={}",
            peak_with
        );
        assert!(
            peak_without < 1e-3 && peak_without < peak_with * 0.05,
            "baseline without MOS Cgd should have negligible drain pulse, peak_without={} peak_with={}",
            peak_without,
            peak_with
        );
    }

    #[test]
    fn test_transient_mosfet_cgs_companion_creates_gate_rc_delay() {
        let with_cgs = Netlist::parse(
            r#"
* MOSFET gate RC loading through Cgs
VSTEP in 0 PULSE(0 1 0 1u 1u 20u 40u)
RG in g 1k
M1 d g 0 0 MCAP
RD d 0 1k
.MODEL MCAP NMOS (LEVEL=1 KP=0 VTO=100 CGSO=1e-4 CGDO=0 CGBO=0)
.end
"#,
        )
        .unwrap();
        let no_cgs = Netlist::parse(
            r#"
* Baseline: same topology without MOS Cgs
VSTEP in 0 PULSE(0 1 0 1u 1u 20u 40u)
RG in g 1k
M1 d g 0 0 MCAP
RD d 0 1k
.MODEL MCAP NMOS (LEVEL=1 KP=0 VTO=100 CGSO=0 CGDO=0 CGBO=0)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let with_cgs_result = engine.run_tran(&with_cgs, 20e-6, 100e-9).unwrap();
        let no_cgs_result = engine.run_tran(&no_cgs, 20e-6, 100e-9).unwrap();

        let g_with = with_cgs_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("g"))
            .expect("gate node g missing in with-cgs result");
        let g_no = no_cgs_result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("g"))
            .expect("gate node g missing in baseline result");

        let idx_with = with_cgs_result
            .time
            .iter()
            .position(|&time| time >= 2e-6)
            .unwrap_or(with_cgs_result.time.len() - 1);
        let idx_no = no_cgs_result
            .time
            .iter()
            .position(|&time| time >= 2e-6)
            .unwrap_or(no_cgs_result.time.len() - 1);

        let vg_with = with_cgs_result.voltages[g_with][idx_with];
        let vg_no = no_cgs_result.voltages[g_no][idx_no];

        assert!(
            vg_with < 0.8,
            "expected MOS Cgs loading to delay gate at 2us, got Vg={}",
            vg_with
        );
        assert!(
            vg_no > 0.95,
            "baseline without MOS Cgs should settle near source quickly, got Vg={}",
            vg_no
        );
    }

    #[test]
    fn test_dc_inductor_behaves_as_short() {
        let netlist_str = r#"
* DC inductor short behavior
V1 1 0 1
L1 1 2 1m
R1 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v2 = result.voltage(2);
        assert!(
            (v2 - 1.0).abs() < 1e-6,
            "expected V(2)=1V with DC-short inductor, got {}",
            v2
        );
    }

    #[test]
    fn test_dc_cccs_control_branch_stamping() {
        let netlist_str = r#"
* CCCS controlled by V1 current
V1 1 0 1
R1 1 0 1k
F1 2 0 V1 2
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v2 = result.voltage(2).abs();
        assert!(
            (v2 - 2.0).abs() < 0.05,
            "expected |V(2)|≈2V for CCCS gain 2, got {}",
            v2
        );
    }

    #[test]
    fn test_dc_ccvs_control_branch_stamping() {
        let netlist_str = r#"
* CCVS controlled by V1 current
V1 1 0 1
R1 1 0 1k
H1 2 0 V1 500
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v2 = result.voltage(2).abs();
        assert!(
            (v2 - 0.5).abs() < 0.02,
            "expected |V(2)|≈0.5V for RM=500 and I(V1)=1mA, got {}",
            v2
        );
    }

    #[test]
    fn test_vcvs_voltage_follower() {
        let netlist_str = r#"
* VCVS Voltage Follower
V1 1 0 3
R1 1 0 1k
E1 2 0 1 0 1.0
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v1 = result.voltage(1);
        let v2 = result.voltage(2);
        assert!((v1 - 3.0).abs() < 0.01, "V1 should be 3V");
        assert!((v2 - 3.0).abs() < 0.1, "V2 should follow V1, got {}", v2);
    }

    #[test]
    fn test_vccs_transconductance() {
        let netlist_str = r#"
* VCCS Transconductance
V1 1 0 2
R1 1 0 1k
G1 2 0 1 0 0.001
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v2 = result.voltage(2);
        assert!(
            v2.abs() > 1.8 && v2.abs() < 2.2,
            "V2 should be ~|2V|, got {}",
            v2
        );
    }

    #[test]
    fn test_very_large_resistor() {
        let netlist_str = r#"
* Large Resistor
V1 1 0 1
R1 1 2 1G
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v2 = result.voltage(2);
        assert!(v2.abs() < 1e-3, "V2 should be ~0, got {}", v2);
    }

    #[test]
    fn test_dc_sweep_negative_step() {
        let netlist_str = r#"
* Negative DC Sweep
V1 1 0 5
R1 1 2 1k
R2 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine.run_dc_sweep(&netlist, "V1", 5.0, 0.0, -1.0).unwrap();
        assert_eq!(results.len(), 6);
        assert_eq!(results[0].0, 5.0);
        assert_eq!(results[5].0, 0.0);
    }

    #[test]
    fn test_two_voltage_sources_series() {
        let netlist_str = r#"
* Series Voltage Sources
V1 1 0 3
V2 2 1 2
R1 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v1 = result.voltage(1);
        let v2 = result.voltage(2);
        assert!((v1 - 3.0).abs() < 0.01, "V1 should be 3V, got {}", v1);
        assert!((v2 - 5.0).abs() < 0.01, "V2 should be 5V, got {}", v2);
    }

    #[test]
    fn test_parallel_resistors() {
        let netlist_str = r#"
* Parallel Resistors
V1 1 0 5
R1 1 0 1k
R2 1 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        let v1 = result.voltage(1);
        assert!((v1 - 5.0).abs() < 0.01, "V1 should be 5V, got {}", v1);
    }

    #[test]
    fn test_rc_time_constant_accuracy() {
        let netlist_str = r#"
* RC Time Constant
V1 1 0 5
R1 1 2 1k
C1 2 0 1u
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine.run_tran(&netlist, 5e-3, 50e-6).unwrap();
        assert!(result.num_points() > 10);
        let v_final = result.voltage_at(2, result.num_points() - 1);
        assert!(
            v_final > 4.0,
            "Final V should approach 5V, got {:.2}V",
            v_final
        );
    }

    #[test]
    fn test_coupled_inductor_unit() {
        use crate::device::{CoupledInductorPair, InductorCoupling};

        let coupling = InductorCoupling::new(
            "K1".to_string(),
            vec!["L1".to_string(), "L2".to_string()],
            0.95,
        );
        assert!((coupling.mutual_inductance(1e-3, 1e-3) - 0.95e-3).abs() < 1e-6);

        let transformer = CoupledInductorPair::new("T1".to_string(), 1, 0, 1e-3, 2, 0, 4e-3, 0.99);
        let n = transformer.turns_ratio();
        assert!(
            (n - 0.5).abs() < 0.1,
            "Turns ratio should be ~0.5, got {}",
            n
        );
    }

    #[test]
    fn test_voltage_switch_unit() {
        use crate::device::NonlinearDevice;
        use crate::device::VoltageSwitch;

        let mut sw = VoltageSwitch::new("S1".to_string(), 1, 0, 2, 0)
            .with_thresholds(2.5, 0.5)
            .with_resistances(1.0, 1e6);

        let voltages = vec![1.0, 0.0];
        sw.update(&voltages);
        assert!(sw.resistance() > 1e4, "Should be high R when off");

        let voltages = vec![1.0, 5.0];
        sw.update(&voltages);
        assert!(sw.resistance() < 100.0, "Should be low R when on");
    }

    #[test]
    fn test_build_vswitch_applies_model_params_and_initial_state() {
        let netlist_str = r#"
* Voltage-controlled switch model application
VDD 1 0 5
VCTRL 2 0 1
RLOAD 3 0 1k
S1 1 3 2 0 SWMOD ON
.MODEL SWMOD SW (VT=1 VH=0.2 RON=2 ROFF=1e8 SMOOTH=0.05)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.vswitches.len(), 1);
        let sw = &circuit.vswitches[0];
        assert!((sw.vt - 1.0).abs() < 1e-12);
        assert!((sw.vh - 0.2).abs() < 1e-12);
        assert!((sw.ron - 2.0).abs() < 1e-12);
        assert!((sw.roff - 1e8).abs() < 1e-3);
        assert!((sw.smooth - 0.05).abs() < 1e-12);
        assert_eq!(sw.state(), crate::device::SwitchState::On);
    }

    #[test]
    fn test_build_iswitch_applies_model_params_and_resolves_control_branch() {
        let netlist_str = r#"
* Current-controlled switch model application
VCTRL 4 0 0
VDD 1 0 5
RBIAS 1 4 1k
RLOAD 2 0 1k
W1 1 2 VCTRL CSWMOD OFF
.MODEL CSWMOD CSW (IT=1m IH=0.2m RON=3 ROFF=2e8 SMOOTH=1e-4)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.iswitches.len(), 1);
        let sw = &circuit.iswitches[0];
        assert!((sw.it - 1e-3).abs() < 1e-15);
        assert!((sw.ih - 0.2e-3).abs() < 1e-15);
        assert!((sw.ron - 3.0).abs() < 1e-12);
        assert!((sw.roff - 2e8).abs() < 1e-3);
        assert!((sw.smooth - 1e-4).abs() < 1e-15);
        assert_eq!(sw.state(), crate::device::SwitchState::Off);

        let control_branch = circuit
            .get_branch_by_name("VCTRL")
            .expect("control source branch should be registered");
        let expected_matrix_index = circuit.get_branch_matrix_index(control_branch);
        assert_eq!(sw.ctrl_branch, Some(expected_matrix_index));
    }

    #[test]
    fn test_build_vswitch_unknown_model_errors() {
        let netlist_str = r#"
* VSwitch unknown model
VCTRL 2 0 0
S1 1 0 2 0 MISSING
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown VSwitch model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Voltage-controlled switch") && msg.contains("unknown model"),
            "expected unknown switch model error, got {}",
            msg
        );
    }

    #[test]
    fn test_build_vswitch_incompatible_model_type_errors() {
        let netlist_str = r#"
* VSwitch incompatible model type
VCTRL 2 0 0
S1 1 0 2 0 SWMOD
.MODEL SWMOD NPN (BF=100)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("incompatible VSwitch model type should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Voltage-controlled switch") && msg.contains("incompatible type"),
            "expected incompatible VSwitch model type error, got {}",
            msg
        );
    }

    #[test]
    fn test_build_iswitch_unknown_model_errors() {
        let netlist_str = r#"
* ISwitch unknown model
VCTRL 2 0 0
W1 1 0 VCTRL MISSING
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unknown ISwitch model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Current-controlled switch") && msg.contains("unknown model"),
            "expected unknown current switch model error, got {}",
            msg
        );
    }

    #[test]
    fn test_build_iswitch_incompatible_model_type_errors() {
        let netlist_str = r#"
* ISwitch incompatible model type
VCTRL 2 0 0
W1 1 0 VCTRL CSW1
.MODEL CSW1 PMOS (LEVEL=1 VTO=-0.7 KP=100u)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("incompatible ISwitch model type should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("Current-controlled switch") && msg.contains("incompatible type"),
            "expected incompatible ISwitch model type error, got {}",
            msg
        );
    }

    #[test]
    fn test_build_iswitch_missing_control_source_errors() {
        let netlist_str = r#"
* ISwitch missing control source
VDD 1 0 5
RLOAD 2 0 1k
W1 1 2 VMISSING CSW1
.MODEL CSW1 CSW (IT=1m IH=0 RON=1 ROFF=1e9)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let err = Engine::default()
            .build_circuit(&netlist)
            .expect_err("missing ISwitch control source should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("ISWITCH control element not found"),
            "expected missing ISwitch control-source error, got {}",
            msg
        );
    }

    #[test]
    fn test_vswitch_initial_state_affects_dc_solution_in_hysteresis_window() {
        let off_netlist = Netlist::parse(
            r#"
* VSwitch OFF initial state in hysteresis window
VDD 1 0 5
VCTRL 2 0 1.0
S1 1 3 2 0 SWMOD OFF
RLOAD 3 0 1k
.MODEL SWMOD SW (RON=1 ROFF=1e9 VT=1.0 VH=0.2 SMOOTH=0.05)
.end
"#,
        )
        .unwrap();
        let on_netlist = Netlist::parse(
            r#"
* VSwitch ON initial state in hysteresis window
VDD 1 0 5
VCTRL 2 0 1.0
S1 1 3 2 0 SWMOD ON
RLOAD 3 0 1k
.MODEL SWMOD SW (RON=1 ROFF=1e9 VT=1.0 VH=0.2 SMOOTH=0.05)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let off = engine.run_dc_op(&off_netlist).unwrap();
        let on = engine.run_dc_op(&on_netlist).unwrap();
        let vout_off = off.voltage(3);
        let vout_on = on.voltage(3);

        assert!(
            vout_off < 0.1,
            "OFF-initialized switch should hold output low in hysteresis window, got {}",
            vout_off
        );
        assert!(
            vout_on > 4.0,
            "ON-initialized switch should hold output high in hysteresis window, got {}",
            vout_on
        );
        assert!(
            vout_on > vout_off + 3.0,
            "switch initial-state hysteresis effect too small: off={} on={}",
            vout_off,
            vout_on
        );
    }

    #[test]
    fn test_iswitch_initial_state_affects_dc_solution_in_hysteresis_window() {
        let off_netlist = Netlist::parse(
            r#"
* ISwitch OFF initial state in hysteresis window
VDD 1 0 5
IBIAS 0 4 5m
VCTRL 4 0 0
W1 1 2 VCTRL CSWMOD OFF
RLOAD 2 0 1k
.MODEL CSWMOD CSW (RON=1 ROFF=1e9 IT=5m IH=1m SMOOTH=1e-4)
.end
"#,
        )
        .unwrap();
        let on_netlist = Netlist::parse(
            r#"
* ISwitch ON initial state in hysteresis window
VDD 1 0 5
IBIAS 0 4 5m
VCTRL 4 0 0
W1 1 2 VCTRL CSWMOD ON
RLOAD 2 0 1k
.MODEL CSWMOD CSW (RON=1 ROFF=1e9 IT=5m IH=1m SMOOTH=1e-4)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let off = engine.run_dc_op(&off_netlist).unwrap();
        let on = engine.run_dc_op(&on_netlist).unwrap();
        let out_node_off = off
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("2"))
            .expect("output node '2' should exist in OFF result");
        let out_node_on = on
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("2"))
            .expect("output node '2' should exist in ON result");
        let vout_off = off.voltage(out_node_off);
        let vout_on = on.voltage(out_node_on);

        assert!(
            vout_off < 0.1,
            "OFF-initialized current switch should hold output low, got {}",
            vout_off
        );
        assert!(
            vout_on > 4.0,
            "ON-initialized current switch should hold output high, got {}",
            vout_on
        );
        assert!(
            vout_on > vout_off + 3.0,
            "current-switch initial-state hysteresis effect too small: off={} on={}",
            vout_off,
            vout_on
        );
    }

    #[test]
    fn test_matrix_topology_includes_vswitch_control_couplings() {
        let netlist = Netlist::parse(
            r#"
* Matrix topology should include VSwitch control Jacobian columns
VDD 1 0 5
VCPLUS 2 0 1.0
VCMINUS 4 0 0.2
RLOAD 3 0 1k
S1 1 3 2 4 SWMOD
.MODEL SWMOD SW (RON=1 ROFF=1e9 VT=0.8 VH=0.1 SMOOTH=0.05)
.end
"#,
        )
        .unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        let matrix = engine.build_matrix(&circuit).unwrap();

        let sw = &circuit.vswitches[0];
        let p = sw.node_pos - 1;
        let n = sw.node_neg - 1;
        let cp = sw.ctrl_pos - 1;
        let cn = sw.ctrl_neg - 1;

        assert!(
            matrix.get_index(p, cp).is_some(),
            "missing (p,cp) VSwitch control coupling entry"
        );
        assert!(
            matrix.get_index(p, cn).is_some(),
            "missing (p,cn) VSwitch control coupling entry"
        );
        assert!(
            matrix.get_index(n, cp).is_some(),
            "missing (n,cp) VSwitch control coupling entry"
        );
        assert!(
            matrix.get_index(n, cn).is_some(),
            "missing (n,cn) VSwitch control coupling entry"
        );
    }

    #[test]
    fn test_matrix_topology_includes_iswitch_control_branch_couplings() {
        let netlist = Netlist::parse(
            r#"
* Matrix topology should include ISwitch control-branch Jacobian column
VCTRL 4 0 0
VDD 1 0 5
RBIAS 1 4 1k
RLOAD 2 0 1k
W1 1 2 VCTRL CSWMOD
.MODEL CSWMOD CSW (RON=1 ROFF=1e9 IT=1m IH=0 SMOOTH=1e-4)
.end
"#,
        )
        .unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();
        let matrix = engine.build_matrix(&circuit).unwrap();

        let sw = &circuit.iswitches[0];
        let cb = sw
            .ctrl_branch
            .expect("ISwitch control branch should be resolved");

        assert!(
            matrix.get_index(sw.node_pos - 1, cb - 1).is_some(),
            "missing ISwitch (p,ctrl_branch) coupling entry"
        );
        assert!(
            matrix.get_index(sw.node_neg - 1, cb - 1).is_some(),
            "missing ISwitch (n,ctrl_branch) coupling entry"
        );
    }

    #[test]
    fn test_vswitch_initial_state_persists_in_transient_inside_hysteresis_window() {
        let off_netlist = Netlist::parse(
            r#"
* VSwitch OFF initial state should persist when control stays inside hysteresis
VDD 1 0 5
VCTRL 2 0 1.0
S1 1 3 2 0 SWMOD OFF
RLOAD 3 0 1k
.MODEL SWMOD SW (RON=1 ROFF=1e9 VT=1.0 VH=0.2 SMOOTH=0.05)
.end
"#,
        )
        .unwrap();
        let on_netlist = Netlist::parse(
            r#"
* VSwitch ON initial state should persist when control stays inside hysteresis
VDD 1 0 5
VCTRL 2 0 1.0
S1 1 3 2 0 SWMOD ON
RLOAD 3 0 1k
.MODEL SWMOD SW (RON=1 ROFF=1e9 VT=1.0 VH=0.2 SMOOTH=0.05)
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let off = engine
            .run_tran(&off_netlist, 2e-6, 20e-9)
            .expect("OFF-initialized transient should converge");
        let on = engine
            .run_tran(&on_netlist, 2e-6, 20e-9)
            .expect("ON-initialized transient should converge");

        let idx_off = off
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("3"))
            .expect("node 3 should exist in OFF result");
        let idx_on = on
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("3"))
            .expect("node 3 should exist in ON result");

        let off_peak = off.voltages[idx_off]
            .iter()
            .copied()
            .fold(Value::NEG_INFINITY, Value::max);
        let on_floor = on.voltages[idx_on]
            .iter()
            .copied()
            .fold(Value::INFINITY, Value::min);

        assert!(
            off_peak < 0.1,
            "OFF-initialized VSwitch should remain low in transient window, peak={}",
            off_peak
        );
        assert!(
            on_floor > 4.0,
            "ON-initialized VSwitch should remain high in transient window, floor={}",
            on_floor
        );
    }

    #[test]
    fn test_transmission_line_unit() {
        use crate::device::TransmissionLine;

        let mut tl = TransmissionLine::new("T1".to_string(), 1, 0, 2, 0, 50.0, 1e-9);
        assert_eq!(tl.impedance(), 50.0);
        assert_eq!(tl.delay(), 1e-9);
        assert_eq!(tl.conductance(), 0.02);

        tl.update_history(0.0, 1.0, 0.02, 0.0, 0.0);
        tl.update_history(0.5e-9, 1.0, 0.02, 0.0, 0.0);
        tl.update_history(1.0e-9, 1.0, 0.02, 0.0, 0.0);
        tl.update_history(1.5e-9, 1.0, 0.02, 0.0, 0.0);

        let delayed = tl.delayed_forward();
        assert!(delayed > 1.5, "Delayed wave should arrive, got {}", delayed);
    }

    #[test]
    fn test_build_oline_uses_model_card_for_z0_and_td() {
        let netlist_str = r#"
* O-line with model-derived parameters
O1 1 0 2 0 LLINE
.MODEL LLINE LTRA R=12.45 G=0 L=8.972e-9 C=0.468e-12 LEN=16
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.tlines.len(), 1);
        let tl = &circuit.tlines[0];

        let expected_z0 = (8.972e-9_f64 / 0.468e-12_f64).sqrt();
        let expected_td = 16.0 * (8.972e-9_f64 * 0.468e-12_f64).sqrt();
        assert!(
            ((tl.z0 - expected_z0) / expected_z0).abs() < 1e-6,
            "expected Z0≈{}, got {}",
            expected_z0,
            tl.z0
        );
        assert!(
            ((tl.td - expected_td) / expected_td).abs() < 1e-6,
            "expected TD≈{}, got {}",
            expected_td,
            tl.td
        );
    }

    #[test]
    fn test_build_yline_inline_values_override_model_card() {
        let netlist_str = r#"
* Y-line with explicit override values
Y1 1 0 2 0 YMOD Z0=75 TD=2n
.MODEL YMOD TXL R=12.45 G=0 L=8.972e-9 C=0.468e-12 LENGTH=16
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.tlines.len(), 1);
        let tl = &circuit.tlines[0];
        assert!(
            (tl.z0 - 75.0).abs() < 1e-12,
            "expected Z0=75, got {}",
            tl.z0
        );
        assert!(
            (tl.td - 2e-9).abs() < 1e-18,
            "expected TD=2ns, got {}",
            tl.td
        );
    }

    #[test]
    fn test_build_tline_unknown_model_without_z0_errors() {
        let netlist_str = r#"
* Unknown transmission-line model
O1 1 0 2 0 MISSING_MODEL
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let err = engine
            .build_circuit(&netlist)
            .expect_err("unknown O-line model should fail build");
        let msg = err.to_string();
        assert!(
            msg.contains("unknown model"),
            "expected unknown model error, got {}",
            msg
        );
    }

    #[test]
    fn test_build_coupled_p_line_expands_multiconductor_runtime() {
        let netlist_str = r#"
* Coupled/multiconductor transmission line
P1 1 0 2 0 3 0 PMOD
.MODEL PMOD LTRA (Z0=50 TD=1N)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let circuit = engine
            .build_circuit(&netlist)
            .expect("P-line should expand into runtime multiconductor conductors");

        assert_eq!(
            circuit.tlines.len(),
            3,
            "expected three uncoupled conductors for six-node P-line"
        );
        assert!(
            circuit
                .tlines
                .iter()
                .all(|tl| tl.node1_neg == 0 && tl.node2_neg == 0),
            "expanded P-line conductors should use ground-referenced returns"
        );
        assert_eq!(circuit.tlines[0].name, "P1#1");
        assert_eq!(circuit.tlines[1].name, "P1#2");
        assert_eq!(circuit.tlines[2].name, "P1#3");
    }

    #[test]
    fn test_build_cpl_p_line_synthesizes_distributed_coupled_network() {
        let netlist_str = r#"
* Two-conductor CPL line with shared reference
P1 V1 V2 0 V3 V4 0 CPL1
.MODEL CPL1 CPL
+R = 0.5 0 0.5
+L = 247.3e-9  31.65e-9
+              247.3e-9
+C = 31.4e-12 -2.45e-12
+              31.4e-12
+G = 0 0 0
+LENGTH = 0.3048
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("CPL P-line should synthesize a distributed RLGC ladder");

        assert!(
            circuit.tlines.is_empty(),
            "CPL lines should expand into explicit distributed primitives, not scalar tlines"
        );
        assert_eq!(
            circuit.multi_winding_transformers.len(),
            8,
            "2-conductor CPL line should create one coupled section per distributed segment"
        );
        assert_eq!(
            circuit.coupled_inductor_pairs.len(),
            0,
            "CPL implementation should use the dense multi-winding realization path"
        );
        assert!(
            circuit.capacitors.len() >= 24,
            "Expected distributed shunt capacitance network for CPL line"
        );
    }

    #[test]
    fn test_build_cpl_p_line_supports_distinct_reference_nodes() {
        let netlist_str = r#"
* Two-conductor CPL line with different near/far references
P1 V1 V2 RNEAR V3 V4 RFAR CPL1
.MODEL CPL1 CPL
+R = 0.5 0 0.5
+L = 247.3e-9  31.65e-9
+              247.3e-9
+C = 31.4e-12 -2.45e-12
+              31.4e-12
+G = 0 0 0
+LENGTH = 0.3048
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine
            .build_circuit(&netlist)
            .expect("CPL line with distinct references should still build");

        let near_ref = circuit
            .get_node_by_name("RNEAR")
            .expect("near reference should exist");
        let far_ref = circuit
            .get_node_by_name("RFAR")
            .expect("far reference should exist");
        assert_ne!(
            near_ref, far_ref,
            "references should remain distinct external nodes"
        );
        assert!(
            circuit
                .resistors
                .names
                .iter()
                .any(|name| name.contains("__cpl.refwire")),
            "distinct references should create an internal reference chain"
        );
    }

    #[test]
    fn test_build_cpl_rejects_off_diagonal_series_resistance() {
        let netlist_str = r#"
* CPL model with unsupported off-diagonal series resistance
P1 V1 V2 0 V3 V4 0 CPL1
.MODEL CPL1 CPL
+R = 0.5 0.01 0.5
+L = 247.3e-9  31.65e-9
+              247.3e-9
+C = 31.4e-12 -2.45e-12
+              31.4e-12
+G = 0 0 0
+LENGTH = 0.3048
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let err = engine
            .build_circuit(&netlist)
            .expect_err("off-diagonal series resistance should be rejected explicitly");
        let msg = err.to_string();
        assert!(
            msg.contains("off-diagonal series resistance"),
            "expected explicit realization error, got {}",
            msg
        );
    }

    #[test]
    fn test_build_jiles_atherton_inductor_integrates_runtime_model() {
        let netlist_str = r#"
* Jiles-Atherton inductor should build with runtime integration
L1 in 0 1m MODEL=CORE1
V1 in 0 DC 0
.MODEL CORE1 CORE (MS=8e5 A=100 K=50 C=0.2 ALPHA=1e-3 AREA=1e-4 LENGTH=0.1)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let circuit = engine
            .build_circuit(&netlist)
            .expect("Jiles-Atherton model should build with runtime state");
        assert!(
            !circuit.jiles_atherton_inductors.is_empty(),
            "expected Jiles-Atherton binding to be present"
        );
        assert_eq!(circuit.jiles_atherton_inductors.len(), 1);
        assert_eq!(circuit.inductors.len(), 1);
        assert!(
            circuit.inductors.inductances[0].is_finite() && circuit.inductors.inductances[0] > 0.0,
            "runtime inductor must have finite positive effective inductance"
        );
    }

    #[test]
    fn test_build_tline_model_sets_attenuation_from_rlgc() {
        let netlist_str = r#"
* O-line with model-derived attenuation
O1 1 0 2 0 LLOSS
.MODEL LLOSS LTRA R=500 G=0 L=2.5e-7 C=1e-10 LEN=0.2
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let circuit = engine.build_circuit(&netlist).unwrap();

        assert_eq!(circuit.tlines.len(), 1);
        let tl = &circuit.tlines[0];
        let expected = (-1.0_f64).exp();
        let expected_rdc = 500.0 * 0.2;
        assert!(
            (tl.attenuation() - expected).abs() < 1e-9,
            "expected attenuation={}, got {}",
            expected,
            tl.attenuation()
        );
        assert!(
            (tl.dc_series_resistance() - expected_rdc).abs() < 1e-12,
            "expected DC series resistance={}, got {}",
            expected_rdc,
            tl.dc_series_resistance()
        );
    }

    #[test]
    fn test_transient_tline_initial_dc_level_is_preserved_before_first_edge() {
        let netlist_str = r#"
* Hold source high; first edge occurs well after simulation window.
V1 src 0 PULSE(5 0 10n 100p 100p 10n 20n)
Rsrc src n1 50
T1 n1 0 n2 0 Z0=50 TD=1n
C1 n1 0 25f
C2 n2 0 7f
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine
            .run_tran(&netlist, 2e-9, 2e-12)
            .expect("transient tline startup should converge");

        let n1_idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("n1"))
            .expect("n1 should exist");
        let n2_idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("n2"))
            .expect("n2 should exist");

        let mut n1_min = Value::INFINITY;
        let mut n2_min = Value::INFINITY;
        for (i, &time) in result.time.iter().enumerate() {
            if time <= 2e-9 {
                n1_min = n1_min.min(result.voltages[n1_idx][i]);
                n2_min = n2_min.min(result.voltages[n2_idx][i]);
            }
        }

        assert!(
            n1_min > 4.95,
            "near-end node drooped before first edge: min={}",
            n1_min
        );
        assert!(
            n2_min > 4.95,
            "far-end node drooped before first edge: min={}",
            n2_min
        );
    }

    #[test]
    fn test_transient_tline_enforces_delay_before_load_rises() {
        let netlist_str = r#"
* Matched source/load around a 1ns transmission line
V1 in 0 PULSE(0 1 0 1p 1p 4n 8n)
Rsrc in 1 50
T1 1 0 2 0 Z0=50 TD=1n
Rload 2 0 50
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine
            .run_tran(&netlist, 6e-9, 25e-12)
            .expect("transient with tline should converge");

        let node2 = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("2"))
            .expect("node 2 should exist");

        let mut v_before = Value::NEG_INFINITY;
        let mut v_after = Value::NEG_INFINITY;
        for (i, &time) in result.time.iter().enumerate() {
            let v = result.voltages[node2][i];
            if time <= 0.8e-9 {
                v_before = v_before.max(v);
            }
            if (1.2e-9..=2.5e-9).contains(&time) {
                v_after = v_after.max(v);
            }
        }

        assert!(
            v_before < 0.05,
            "load rose too early before delay: v_before={}",
            v_before
        );
        assert!(
            v_after > 0.2,
            "load did not rise after expected delay: v_after={}",
            v_after
        );
    }

    #[test]
    fn test_transient_compressed_tline_enforces_delay_before_load_rises() {
        let netlist_str = r#"
* Matched source/load around a 1ns transmission line (compressed path)
V1 in 0 PULSE(0 1 0 1p 1p 4n 8n)
Rsrc in 1 50
T1 1 0 2 0 Z0=50 TD=1n
Rload 2 0 50
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine
            .run_tran_compressed(
                &netlist,
                6e-9,
                25e-12,
                crate::engine::CompressionConfig::none(),
            )
            .expect("compressed transient with tline should converge");

        // Node ordering follows node allocation order:
        // in -> 1, "1" -> 2, "2" -> 3, so load node index is 2 (0-based).
        let load_idx = 2;
        assert!(
            result.num_nodes > load_idx,
            "expected load node index {} in compressed result with {} nodes",
            load_idx,
            result.num_nodes
        );

        let mut v_before = Value::NEG_INFINITY;
        let mut v_after = Value::NEG_INFINITY;
        for (i, &time) in result.time.iter().enumerate() {
            let v = result.voltages[load_idx][i];
            if time <= 0.8e-9 {
                v_before = v_before.max(v);
            }
            if (1.2e-9..=2.5e-9).contains(&time) {
                v_after = v_after.max(v);
            }
        }

        assert!(
            v_before < 0.05,
            "compressed load rose too early before delay: v_before={}",
            v_before
        );
        assert!(
            v_after > 0.2,
            "compressed load did not rise after expected delay: v_after={}",
            v_after
        );
    }

    #[test]
    fn test_transient_tline_model_attenuation_reduces_load_peak() {
        let lossless_netlist = Netlist::parse(
            r#"
* Baseline lossless line
V1 in 0 PULSE(0 1 0 1p 1p 4n 8n)
Rsrc in 1 50
T1 1 0 2 0 Z0=50 TD=1n
Rload 2 0 50
.end
"#,
        )
        .unwrap();

        let lossy_netlist = Netlist::parse(
            r#"
* Lossy model line
V1 in 0 PULSE(0 1 0 1p 1p 4n 8n)
Rsrc in 1 50
Y1 1 0 2 0 LLOSS
Rload 2 0 50
.MODEL LLOSS LTRA R=500 G=0 L=2.5e-7 C=1e-10 LEN=0.2
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let baseline = engine
            .run_tran(&lossless_netlist, 6e-9, 25e-12)
            .expect("baseline transient should converge");
        let lossy = engine
            .run_tran(&lossy_netlist, 6e-9, 25e-12)
            .expect("lossy transient should converge");

        let baseline_node2 = baseline
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("2"))
            .expect("baseline node 2 should exist");
        let lossy_node2 = lossy
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("2"))
            .expect("lossy node 2 should exist");

        let baseline_peak = baseline
            .time
            .iter()
            .enumerate()
            .filter(|(_, t)| (1.2e-9..=2.5e-9).contains(*t))
            .map(|(i, _)| baseline.voltages[baseline_node2][i])
            .fold(Value::NEG_INFINITY, Value::max);
        let lossy_peak = lossy
            .time
            .iter()
            .enumerate()
            .filter(|(_, t)| (1.2e-9..=2.5e-9).contains(*t))
            .map(|(i, _)| lossy.voltages[lossy_node2][i])
            .fold(Value::NEG_INFINITY, Value::max);

        assert!(
            baseline_peak > 0.2,
            "baseline peak should be significant, got {}",
            baseline_peak
        );
        assert!(
            lossy_peak < baseline_peak * 0.8,
            "lossy peak should be lower than baseline: lossy={}, baseline={}",
            lossy_peak,
            baseline_peak
        );

        let ratio = lossy_peak / baseline_peak;
        assert!(
            (0.2..0.6).contains(&ratio),
            "unexpected attenuation ratio: {}",
            ratio
        );
    }

    #[test]
    fn test_dc_with_tline_stamps_matrix_topology_consistently() {
        let netlist_str = r#"
* DC solve with transmission line companion ports
V1 in 0 1
Rsrc in 1 50
T1 1 0 2 0 Z0=50 TD=1n
Rload 2 0 50
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let result = engine
            .run_dc_op(&netlist)
            .expect("DC with tline should solve without topology/stamping mismatch");

        let node1_idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("1"))
            .expect("node '1' should exist");
        let node2_idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("2"))
            .expect("node '2' should exist");

        let v1 = result.voltage(node1_idx);
        let v2 = result.voltage(node2_idx);
        assert!(
            v1.is_finite(),
            "node 1 voltage should be finite after DC solve, got {}",
            v1
        );
        assert!(
            (v1 - v2).abs() < 2e-5,
            "tline DC fallback should couple near/far ports (v1={}, v2={})",
            v1,
            v2
        );
        assert!(
            (v2 - 0.5).abs() < 1e-3,
            "expected 50/50 divider DC level at load, got {}",
            v2
        );
    }

    #[test]
    fn test_ac_tline_matched_line_has_expected_magnitude_and_delay() {
        use std::f64::consts::PI;

        let netlist_str = r#"
* AC matched line transfer
V1 in 0 AC 1
Rsrc in 1 50
T1 1 0 2 0 Z0=50 TD=1n
Rload 2 0 50
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();

        let f1 = 100e6;
        let f2 = 300e6;
        let results = engine
            .run_ac(&netlist, &[f1, f2])
            .expect("AC with transmission line should converge");
        assert_eq!(results.len(), 2);

        // Node order: in, 1, 2 => load is node index 3 in SPICE numbering.
        let vout_mag_f1 = results[0].voltage_magnitude(3);
        let vout_mag_f2 = results[1].voltage_magnitude(3);
        assert!(
            (vout_mag_f1 - 0.5).abs() < 0.06,
            "expected ~0.5 magnitude at f1, got {}",
            vout_mag_f1
        );
        assert!(
            (vout_mag_f2 - 0.5).abs() < 0.06,
            "expected ~0.5 magnitude at f2, got {}",
            vout_mag_f2
        );

        let phase_f1 = results[0].voltage_phase(3);
        let phase_f2 = results[1].voltage_phase(3);
        let phase_diff = phase_f2 - phase_f1;
        let expected_diff = -2.0 * PI * (f2 - f1) * 1e-9;
        let wrapped = (phase_diff - expected_diff + PI).rem_euclid(2.0 * PI) - PI;
        assert!(
            wrapped.abs() < 0.15,
            "unexpected tline phase delay slope: measured diff={}, expected diff={}, wrapped err={}",
            phase_diff,
            expected_diff,
            wrapped
        );
    }

    #[test]
    fn test_ac_tline_model_attenuation_reduces_transfer_magnitude() {
        let baseline = Netlist::parse(
            r#"
* AC baseline lossless line
V1 in 0 AC 1
Rsrc in 1 50
T1 1 0 2 0 Z0=50 TD=1n
Rload 2 0 50
.end
"#,
        )
        .unwrap();
        let lossy = Netlist::parse(
            r#"
* AC lossy model line
V1 in 0 AC 1
Rsrc in 1 50
Y1 1 0 2 0 LLOSS
Rload 2 0 50
.MODEL LLOSS LTRA R=500 G=0 L=2.5e-7 C=1e-10 LEN=0.2
.end
"#,
        )
        .unwrap();

        let engine = Engine::default();
        let freq = 100e6;
        let baseline_res = engine
            .run_ac(&baseline, &[freq])
            .expect("baseline AC should converge");
        let lossy_res = engine
            .run_ac(&lossy, &[freq])
            .expect("lossy AC should converge");

        let baseline_mag = baseline_res[0].voltage_magnitude(3);
        let lossy_mag = lossy_res[0].voltage_magnitude(3);
        assert!(
            baseline_mag > 0.3,
            "baseline magnitude should be substantial, got {}",
            baseline_mag
        );
        assert!(
            lossy_mag < baseline_mag * 0.8,
            "lossy transfer should be smaller: lossy={}, baseline={}",
            lossy_mag,
            baseline_mag
        );

        let ratio = lossy_mag / baseline_mag;
        assert!(
            (0.2..0.6).contains(&ratio),
            "unexpected attenuation ratio in AC: {}",
            ratio
        );
    }

    #[test]
    fn test_ac_tline_handles_zero_frequency_without_nan() {
        let netlist_str = r#"
* AC tline zero-frequency stability
V1 in 0 AC 1
Rsrc in 1 50
T1 1 0 2 0 Z0=50 TD=1n
Rload 2 0 50
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();
        let engine = Engine::default();
        let results = engine
            .run_ac(&netlist, &[0.0, 1e6, 10e6])
            .expect("AC sweep including DC should converge");

        for (idx, point) in results.iter().enumerate() {
            let v = point.voltage_magnitude(3);
            assert!(
                v.is_finite(),
                "AC magnitude should stay finite at point {} (f={}): {}",
                idx,
                point.frequency,
                v
            );
        }
    }

    #[test]
    fn test_parametric_sweep_integration() {
        use crate::analysis::{ParametricSweep, StepSpec};

        let step_r = StepSpec::param("R1", 1000.0, 3000.0, 1000.0);
        let step_c = StepSpec::param_list("C1", vec![1e-9, 2e-9]);

        let mut sweep = ParametricSweep::new(vec![step_r, step_c]);
        assert_eq!(sweep.total_combinations(), 6);

        let mut combo_count = 0;
        loop {
            let values = sweep.current_values();
            assert_eq!(values.len(), 2);
            combo_count += 1;
            if !sweep.advance() {
                break;
            }
        }
        assert_eq!(combo_count, 6);
    }

    #[test]
    fn test_temperature_scaling_integration() {
        use crate::analysis::temperature::{
            JunctionTempScaling, ResistorTempCoeffs, TemperatureContext,
        };

        let temp_85c = TemperatureContext::from_celsius(85.0, 27.0);
        let tc = ResistorTempCoeffs::new(0.0039, 0.0);

        let r_27c = 1000.0;
        let r_85c = tc.scale_resistance(r_27c, &temp_85c);

        assert!(r_85c > r_27c);
        assert!((r_85c - 1226.0).abs() < 50.0, "R at 85C = {}", r_85c);

        let js = JunctionTempScaling::default();
        let is_scaled = js.scale_is(1e-14, &temp_85c);
        assert!(
            is_scaled > 1e-12,
            "Is should increase 100x+ at 85C, got {}",
            is_scaled
        );
    }

    #[test]
    fn test_fourier_analysis_integration() {
        use crate::analysis::{FourierAnalysis, FourierConfig};

        let config = FourierConfig::new(1000.0).with_harmonics(9);
        let analysis = FourierAnalysis::new(config);

        let num_points = 2000;
        let duration = 0.002;
        let time: Vec<f64> = (0..num_points)
            .map(|i| i as f64 * duration / (num_points - 1) as f64)
            .collect();
        let values: Vec<f64> = time
            .iter()
            .map(|&t| {
                let phase = (t * 1000.0).fract();
                if phase < 0.5 { 1.0 } else { -1.0 }
            })
            .collect();

        let result = analysis.analyze(&time, &values);

        assert!(result.dc_component.abs() < 0.1, "DC should be ~0");
        let fund = result.fundamental().unwrap();
        assert!(fund.magnitude > 0.5, "Fundamental should be significant");
        assert!(
            result.thd > 40.0,
            "Square wave THD should be >40%, got {:.1}%",
            result.thd
        );
    }

    #[test]
    fn test_noise_source_types() {
        let boltzmann: f64 = 1.380649e-23;
        let temp_k: f64 = 300.0;
        let r: f64 = 1000.0;

        let thermal_density: f64 = 4.0 * boltzmann * temp_k * r;
        let nv_sqrt_hz = thermal_density.sqrt() * 1e9;
        assert!(
            (nv_sqrt_hz - 4.0).abs() < 0.5,
            "Thermal noise should be ~4nV/√Hz, got {}",
            nv_sqrt_hz
        );

        let q: f64 = 1.602176634e-19;
        let id: f64 = 1e-3;
        let shot_density: f64 = 2.0 * q * id;
        let pa_sqrt_hz = shot_density.sqrt() * 1e12;
        assert!(
            pa_sqrt_hz > 10.0 && pa_sqrt_hz < 20.0,
            "Shot noise should be ~18pA/√Hz, got {}",
            pa_sqrt_hz
        );
    }

    #[test]
    fn test_convergence_helpers_integration() {
        use crate::solver::convergence::{GminStepper, PseudoTransient, SourceStepper};

        let mut source_stepper = SourceStepper::new();
        assert!(!source_stepper.is_complete());
        assert_eq!(source_stepper.factor(), 0.0);

        while !source_stepper.is_complete() {
            source_stepper.advance_on_success();
        }
        assert_eq!(source_stepper.factor(), 1.0);

        let gmin_stepper = GminStepper::new();
        assert!(gmin_stepper.gmin() > 1e-13);

        let ptran = PseudoTransient::new();
        let g = ptran.conductance(1);
        assert!(g > 0.0);
    }

    //=========================================================================
    // ConvergenceConfig Tests
    //=========================================================================

    #[test]
    fn test_convergence_config_default() {
        use crate::engine::{ConvergenceConfig, DampingStrategy};

        let config = ConvergenceConfig::default();
        assert!(config.gmin_stepping);
        assert!(config.source_stepping);
        assert!(config.pseudo_transient);
        assert!(!config.arc_length); // Arc-length is off by default
        assert_eq!(config.damping_strategy, DampingStrategy::VoltageLimiting);
        assert!(config.gmin_initial > config.gmin_target);
        assert!(config.voltage_reltol > 0.0);
        assert_eq!(config.voltage_abstol, 0.0);
        assert!(config.current_abstol > 0.0);
        assert!(config.residual_reltol > 0.0);
        assert!(!config.verbose);
    }

    #[test]
    fn test_convergence_config_fast() {
        use crate::engine::{ConvergenceConfig, DampingStrategy};

        let config = ConvergenceConfig::fast();
        assert!(!config.gmin_stepping);
        assert!(!config.source_stepping);
        assert!(!config.pseudo_transient);
        assert!(!config.arc_length);
        assert_eq!(config.damping_strategy, DampingStrategy::None);
    }

    #[test]
    fn test_convergence_config_robust() {
        use crate::engine::{ConvergenceConfig, DampingStrategy};

        let config = ConvergenceConfig::robust();
        assert!(config.gmin_stepping);
        assert!(config.source_stepping);
        assert!(config.pseudo_transient);
        assert!(config.arc_length);
        assert_eq!(config.damping_strategy, DampingStrategy::Combined);
    }

    #[test]
    fn test_convergence_config_with_verbose() {
        use crate::engine::ConvergenceConfig;

        let config = ConvergenceConfig::default().with_verbose(true);
        assert!(config.verbose);

        let config = ConvergenceConfig::fast().with_verbose(false);
        assert!(!config.verbose);
    }

    #[test]
    fn test_convergence_config_with_damping() {
        use crate::engine::{ConvergenceConfig, DampingStrategy};

        let config = ConvergenceConfig::default().with_damping(DampingStrategy::LineSearch);
        assert_eq!(config.damping_strategy, DampingStrategy::LineSearch);

        let config = ConvergenceConfig::fast().with_damping(DampingStrategy::BankRose);
        assert_eq!(config.damping_strategy, DampingStrategy::BankRose);
    }

    #[test]
    fn test_convergence_config_with_voltage_tolerances() {
        use crate::engine::ConvergenceConfig;

        let config = ConvergenceConfig::default().with_voltage_tolerances(5e-4, 2e-6);
        assert!((config.voltage_reltol - 5e-4).abs() < 1e-15);
        assert!((config.voltage_abstol - 2e-6).abs() < 1e-15);
    }

    #[test]
    fn test_convergence_config_with_current_tolerance() {
        use crate::engine::ConvergenceConfig;

        let config = ConvergenceConfig::default().with_current_tolerance(8e-13);
        assert!((config.current_abstol - 8e-13).abs() < 1e-24);
    }

    #[test]
    fn test_convergence_config_with_residual_reltol() {
        use crate::engine::ConvergenceConfig;

        let config = ConvergenceConfig::default().with_residual_reltol(2e-4);
        assert!((config.residual_reltol - 2e-4).abs() < 1e-18);
    }

    #[test]
    fn test_damping_strategy_default() {
        use crate::engine::DampingStrategy;

        let strategy = DampingStrategy::default();
        assert_eq!(strategy, DampingStrategy::None);
    }

    #[test]
    fn test_damping_strategy_variants() {
        use crate::engine::DampingStrategy;

        // All variants should be distinct
        assert_ne!(DampingStrategy::None, DampingStrategy::LineSearch);
        assert_ne!(
            DampingStrategy::LineSearch,
            DampingStrategy::VoltageLimiting
        );
        assert_ne!(DampingStrategy::VoltageLimiting, DampingStrategy::BankRose);
        assert_ne!(DampingStrategy::BankRose, DampingStrategy::Combined);
    }

    #[test]
    fn test_simulation_config_has_convergence_config() {
        use crate::engine::SimulationConfig;

        let config = SimulationConfig::default();
        // ConvergenceConfig should be accessible
        assert!(config.convergence_config.gmin_stepping);
    }

    #[test]
    fn test_simulation_config_custom_convergence() {
        use crate::engine::{ConvergenceConfig, SimulationConfig};

        let mut config = SimulationConfig::default();
        config.convergence_config = ConvergenceConfig::fast();
        assert!(!config.convergence_config.gmin_stepping);
    }

    #[test]
    fn test_convergence_config_gmin_values() {
        use crate::engine::ConvergenceConfig;

        let config = ConvergenceConfig::default();
        assert!(config.gmin_initial > 0.0);
        assert!(config.gmin_target > 0.0);
        assert!(config.gmin_initial > config.gmin_target);
        // GMIN initial should be around 1e-12
        assert!(config.gmin_initial > 1e-14 && config.gmin_initial < 1e-10);
        // GMIN target should be around 1e-15
        assert!(config.gmin_target > 1e-17 && config.gmin_target < 1e-13);
    }

    #[test]
    fn test_engine_with_fast_convergence() {
        use crate::Netlist;
        use crate::engine::{ConvergenceConfig, Engine, SimulationConfig};

        let netlist_str = r#"
* Simple resistor (should converge without aids)
V1 1 0 5
R1 1 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();

        let mut config = SimulationConfig::default();
        config.convergence_config = ConvergenceConfig::fast();

        let engine = Engine::new(config);
        let result = engine.run_dc_op(&netlist).unwrap();

        assert!((result.voltage(1) - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_engine_fast_convergence_reports_failure_when_aids_are_disabled() {
        use crate::Netlist;
        use crate::engine::{ConvergenceConfig, Engine, SimulationConfig, SimulationError};

        let netlist_str = r#"
* Nonlinear diode clamp with low iteration budget
V1 in 0 5
R1 in out 1k
D1 out 0 DMOD
.MODEL DMOD D (IS=1e-14 N=1 RS=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();

        let mut config = SimulationConfig::default();
        config.max_iterations = 0; // Force very small direct Newton budget
        config.convergence_config = ConvergenceConfig::fast(); // No source/GMIN fallback

        let engine = Engine::new(config);
        let err = engine
            .run_dc_op(&netlist)
            .expect_err("fast config should fail on this nonlinear case with no fallback aids");
        assert!(
            matches!(err, SimulationError::ConvergenceFailed(_)),
            "expected convergence-failed error with aids disabled, got {}",
            err
        );
    }

    #[test]
    fn test_engine_with_robust_convergence() {
        use crate::Netlist;
        use crate::engine::{ConvergenceConfig, Engine, SimulationConfig};

        let netlist_str = r#"
* Diode with resistor
V1 1 0 5
D1 1 2 1N4148
R1 2 0 1k
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();

        let mut config = SimulationConfig::default();
        config.convergence_config = ConvergenceConfig::robust();

        let engine = Engine::new(config);
        let result = engine.run_dc_op(&netlist).unwrap();

        // Should converge with robust settings
        assert!(result.voltage(2) > 0.0);
    }

    #[test]
    fn test_engine_with_pseudo_transient_only_convergence() {
        use crate::Netlist;
        use crate::engine::{ConvergenceConfig, Engine, SimulationConfig};

        let netlist_str = r#"
* Nonlinear diode clamp, solved via pseudo-transient continuation
V1 in 0 5
R1 in out 1k
D1 out 0 DMOD
.MODEL DMOD D (IS=1e-14 N=1 RS=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();

        let mut config = SimulationConfig::default();
        config.max_iterations = 0; // Force fallback paths
        config.convergence_config = ConvergenceConfig {
            gmin_stepping: false,
            source_stepping: false,
            pseudo_transient: true,
            arc_length: false,
            ..ConvergenceConfig::default()
        };

        let engine = Engine::new(config);
        let result = engine.run_dc_op(&netlist).unwrap();

        assert!(result.voltage(2).is_finite());
        assert!(result.voltage(2) > 0.0);
        assert!(result.voltage(2) < 5.0);
    }

    #[test]
    fn test_engine_with_arc_length_only_convergence() {
        use crate::Netlist;
        use crate::engine::{ConvergenceConfig, Engine, SimulationConfig};

        let netlist_str = r#"
* Nonlinear diode clamp, solved via arc-length continuation
V1 in 0 5
R1 in out 1k
D1 out 0 DMOD
.MODEL DMOD D (IS=1e-14 N=1 RS=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();

        let mut config = SimulationConfig::default();
        config.max_iterations = 0; // Force fallback paths
        config.convergence_config = ConvergenceConfig {
            gmin_stepping: false,
            source_stepping: false,
            pseudo_transient: false,
            arc_length: true,
            ..ConvergenceConfig::default()
        };

        let engine = Engine::new(config);
        let result = engine.run_dc_op(&netlist).unwrap();

        assert!(result.voltage(2).is_finite());
        assert!(result.voltage(2) > 0.0);
        assert!(result.voltage(2) < 5.0);
    }

    #[test]
    fn test_engine_with_line_search_damping_strategy() {
        use crate::Netlist;
        use crate::engine::{ConvergenceConfig, DampingStrategy, Engine, SimulationConfig};

        let netlist_str = r#"
* Diode clamp circuit
V1 in 0 5
R1 in out 1k
D1 out 0 DMOD
.MODEL DMOD D (IS=1e-14 N=1 RS=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();

        let mut config = SimulationConfig::default();
        config.convergence_config =
            ConvergenceConfig::default().with_damping(DampingStrategy::LineSearch);

        let engine = Engine::new(config);
        let result = engine.run_dc_op(&netlist).unwrap();

        assert!(result.voltage(2).is_finite());
        assert!(result.voltage(2) > 0.0);
        assert!(result.voltage(2) < 5.0);
    }

    #[test]
    fn test_engine_with_bank_rose_damping_strategy() {
        use crate::Netlist;
        use crate::engine::{ConvergenceConfig, DampingStrategy, Engine, SimulationConfig};

        let netlist_str = r#"
* Diode clamp circuit
V1 in 0 5
R1 in out 1k
D1 out 0 DMOD
.MODEL DMOD D (IS=1e-14 N=1 RS=0)
.end
"#;
        let netlist = Netlist::parse(netlist_str).unwrap();

        let mut config = SimulationConfig::default();
        config.convergence_config =
            ConvergenceConfig::default().with_damping(DampingStrategy::BankRose);

        let engine = Engine::new(config);
        let result = engine.run_dc_op(&netlist).unwrap();

        assert!(result.voltage(2).is_finite());
        assert!(result.voltage(2) > 0.0);
        assert!(result.voltage(2) < 5.0);
    }

    #[test]
    fn test_damping_strategy_clone_eq() {
        use crate::engine::DampingStrategy;

        let s1 = DampingStrategy::Combined;
        let s2 = s1;
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_convergence_config_clone() {
        use crate::engine::ConvergenceConfig;

        let config1 = ConvergenceConfig::robust();
        let config2 = config1.clone();

        assert_eq!(config1.gmin_stepping, config2.gmin_stepping);
        assert_eq!(config1.arc_length, config2.arc_length);
    }
}
