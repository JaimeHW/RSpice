use super::*;
use rspice_core::engine::TransientStartupMode;
use rspice_core::{CurrentImpulseOwner, CurrentImpulseTrace};

fn trace<'a>(result: &'a TransientResult, name: &str) -> &'a CurrentImpulseTrace {
    result.current_impulses.as_ref().unwrap().iter().find(|trace|
        matches!(&trace.owner, CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case(name))).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_constant_bias_current_fanout_preserves_resistive_output_and_restart() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.gp_transient_phase_model = GpTransientPhaseModel::ExactDelay;
        config.integration_method = IntegrationMethod::BackwardEuler;
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        // Refine the first-order Xyce trajectory to the same 10-pA oracle
        // gate. Its coarse-grid RL error is unchanged by adding this fanout.
        let max_step = if dialect == SpiceDialect::Xyce {
            25e-15
        } else {
            2e-12
        };
        for polarity in [1.0, -1.0] {
            for threshold in [2.0, 1000.0] {
                let source = Netlist::parse(&format!(
                    "constant bias GP current fanout\nVC c 0 {}\nVB b 0 {}\nVD drive 0 DC 0 SIN(0 {} 1G)\nR1 drive coil 1k\nL1 coil 0 1u\nF1 b 0 L1 1\nF2 copy 0 VB 1\nR2 copy 0 1k\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\n.options device zeroresistancetol={threshold}\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) v(copy) i(vb) i(f2) i(l1)\n.end\n",
                    2.0 * polarity, 0.7 * polarity, 1e-3 * polarity,
                    if polarity > 0.0 { "NPN" } else { "PNP" }
                )).unwrap();
                let (result, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &source,
                        50e-12,
                        max_step,
                        TransientStartupMode::OperatingPoint,
                        &[25e-12],
                    )
                    .unwrap_or_else(|error| panic!("{dialect:?}/{polarity}/{threshold}: {error}"));
                let voltage = result.try_voltage_waveform_named("copy").unwrap();
                let base = result.try_branch_current_waveform_named("vb").unwrap();
                let copy = result.try_branch_current_waveform_named("f2").unwrap();
                let winding = result.try_branch_current_waveform_named("l1").unwrap();
                let vt = thermal_voltage(dialect);
                let bias = diode(0.7, vt, dialect).0 / 100.0 + diode(-1.3, vt, dialect).0;
                let omega = std::f64::consts::TAU * 1e9;
                let reactance = omega * 1e-6;
                for (index, &time) in result.time.iter().enumerate() {
                    let current = 1e-3
                        * (1000.0 * (omega * time).sin() - reactance * (omega * time).cos()
                            + reactance * (-time / 1e-9).exp())
                        / (1e6 + reactance * reactance);
                    assert!(
                        (polarity * winding[index] - current).abs() < 1e-11,
                        "{dialect:?}/{polarity}/{threshold}: RL at {time:e}: actual={:e}, expected={current:e}, points={}",
                        polarity * winding[index],
                        result.time.len()
                    );
                    assert!(
                        (-polarity * base[index] - bias - current).abs() < 2e-11,
                        "{dialect:?}/{polarity}/{threshold}: base at {time:e}"
                    );
                    assert!((copy[index] - base[index]).abs() < 1e-16);
                    assert!((voltage[index] + 1000.0 * copy[index]).abs() < 1e-10);
                }
                assert_eq!(result.time.last(), Some(&50e-12));
                assert!(trace(&result, "f2").complete);
                assert!(
                    trace(&result, "f2")
                        .points
                        .iter()
                        .all(|point| point.charge_coulombs.abs() < 1e-25)
                );
                for checkpoint in checkpoints {
                    exact_restart(
                        &engine,
                        &source,
                        &result,
                        &checkpoint.checkpoint,
                        50e-12,
                        max_step,
                    );
                }
                assert_eq!(engine.convergence_quality().force_accepted_points, 0);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_finite_behavioral_current_controls_preserve_rl_law_and_packed_restart() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.gp_transient_phase_model = GpTransientPhaseModel::ExactDelay;
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        for polarity in [1.0, -1.0] {
            for control in ["L1", "R1"] {
                for square in [false, true] {
                    let expression = if square {
                        format!("{polarity}*1e6*I({control})^2")
                    } else {
                        format!("I({control})^1")
                    };
                    let source = Netlist::parse(&format!(
                        "finite behavioral current\nVC c 0 {}\nVB b 0 {}\nVD drive 0 DC 0 SIN(0 {} 1G)\nR1 drive coil 1k\nL1 coil 0 1u\nB1 b 0 I={{{expression}}}\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\n.options device zeroresistancetol=1k\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) i(vb) i(vc) i(b1) i(l1) i(r1)\n.end\n",
                        2.0*polarity, 0.7*polarity, 1e-3*polarity, if polarity > 0.0 { "NPN" } else { "PNP" }
                    )).unwrap();
                    for startup in [
                        TransientStartupMode::OperatingPoint,
                        TransientStartupMode::Uic,
                    ] {
                        let (result, checkpoints) = engine
                            .run_tran_checkpoint_schedule_with_startup_mode(
                                &source,
                                2.5e-9,
                                1e-12,
                                startup,
                                &[1.2e-9],
                            )
                            .unwrap_or_else(|error| {
                                panic!(
                                    "{dialect:?}/{polarity}/{control}/{square}/{startup:?}: {error}"
                                )
                            });
                        let voltage = result.try_voltage_waveform_named("b").unwrap();
                        let output = result.try_branch_current_waveform_named("b1").unwrap();
                        let winding = result.try_branch_current_waveform_named("l1").unwrap();
                        let resistor = result.try_branch_current_waveform_named("r1").unwrap();
                        let base = result.try_branch_current_waveform_named("vb").unwrap();
                        let collector = result.try_branch_current_waveform_named("vc").unwrap();
                        let vt = thermal_voltage(dialect);
                        let forward = diode(0.7, vt, dialect).0;
                        let reverse = diode(-1.3, vt, dialect).0;
                        let omega = std::f64::consts::TAU * 1e9;
                        let reactance = omega * 1e-6;
                        for (index, &time) in result.time.iter().enumerate() {
                            // Independent solution of L*dI/dt + R*I = A*sin(w*t), I(0)=0.
                            let current = 1e-3
                                * (1000.0 * (omega * time).sin()
                                    - reactance * (omega * time).cos()
                                    + reactance * (-time / 1e-9).exp())
                                / (1e6 + reactance * reactance);
                            let forcing = if square {
                                1e6 * current * current
                            } else {
                                current
                            };
                            assert!((polarity * voltage[index] - 0.7).abs() < 1e-10);
                            assert!(
                                (polarity * winding[index] - current).abs() < 1e-11,
                                "{dialect:?}/{polarity}/{control}/{square}/{startup:?}: RL current at {time:e}: actual={}, expected={current:e}",
                                winding[index]
                            );
                            assert!((winding[index] - resistor[index]).abs() < 1e-15);
                            assert!(
                                (polarity * output[index] - forcing).abs() < 1e-11,
                                "behavioral current at {time:e}"
                            );
                            assert!(
                                (-polarity * base[index] - (forward / 100.0 + reverse + forcing))
                                    .abs()
                                    < 2e-11,
                                "{dialect:?}/{polarity}/{control}/{square}/{startup:?}: base at {time:e}: actual={:e}, expected={:e}",
                                -polarity * base[index],
                                forward / 100.0 + reverse + forcing
                            );
                            let arrived =
                                time >= DELAY || (time - DELAY).abs() <= 8.0 * f64::EPSILON * DELAY;
                            let transport = if startup == TransientStartupMode::Uic && !arrived {
                                0.0
                            } else {
                                forward
                            };
                            assert!(
                                (-polarity * collector[index] - (transport - 2.0 * reverse)).abs()
                                    < 2e-11
                            );
                        }
                        assert_eq!(result.time.last(), Some(&2.5e-9));
                        assert!(trace(&result, "b1").complete);
                        assert!(
                            trace(&result, "b1")
                                .points
                                .iter()
                                .all(|point| point.charge_coulombs == 0.0)
                        );
                        if startup == TransientStartupMode::Uic {
                            let impulse = trace(&result, "vb")
                                .points
                                .iter()
                                .find(|point| point.time == 0.0)
                                .unwrap()
                                .charge_coulombs;
                            assert!(
                                (impulse + polarity * TF * forward).abs()
                                    < 1e-25 + 1e-10 * TF * forward
                            );
                        }
                        for checkpoint in checkpoints {
                            exact_restart(
                                &engine,
                                &source,
                                &result,
                                &checkpoint.checkpoint,
                                2.5e-9,
                                1e-12,
                            );
                        }
                        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
                    }
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_current_controlled_sources_preserve_finite_currents_impulses_and_restart() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.gp_transient_phase_model = GpTransientPhaseModel::ExactDelay;
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        for polarity in [1.0, -1.0] {
            let source = Netlist::parse(&format!(
                "GP current control\nVC c 0 {}\nVB b 0 DC {} SIN({} {} 1G)\nVS sense 0 0\nIS 0 sense DC {} SIN(0 {} 1G 0 0 90)\nF1 b 0 VS 1\nFCOPY copy 0 VB -2\nRC copy 0 1k\nCC copy 0 1p\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) v(copy) i(vc) i(vb) i(vs) i(f1) i(fcopy)\n.end\n",
                2.0*polarity,0.7*polarity,0.7*polarity,1e-6*polarity,1e-6*polarity,1e-6*polarity,
                if polarity>0.0 {"NPN"} else {"PNP"},
            )).unwrap();
            let check = |result: &TransientResult, uic: bool| {
                behavioral::check(result, dialect, polarity, Some("f1"), "vb", uic);
                let base = result.try_branch_current_waveform_named("vb").unwrap();
                let copy = result.try_branch_current_waveform_named("fcopy").unwrap();
                for (&base, &copy) in base.iter().zip(copy) {
                    assert!((copy + 2.0 * base).abs() < 1e-16);
                }
                assert!(
                    trace(result, "f1")
                        .points
                        .iter()
                        .all(|point| point.charge_coulombs == 0.0)
                );
                if uic {
                    let charge =
                        2.0 * polarity * TF * diode(0.7, thermal_voltage(dialect), dialect).0;
                    let initial = trace(result, "fcopy")
                        .points
                        .iter()
                        .find(|point| point.time == 0.0)
                        .unwrap();
                    assert!(
                        (initial.charge_coulombs - charge).abs() < 1e-25 + charge.abs() * 1e-10
                    );
                    let voltage = result.try_voltage_waveform_named("copy").unwrap()[0];
                    assert!((voltage + charge / 1e-12).abs() < 1e-10);
                }
            };
            for startup in [
                TransientStartupMode::OperatingPoint,
                TransientStartupMode::Uic,
            ] {
                let result = engine
                    .run_tran_with_startup_mode(&source, 2.5e-9, 4e-12, startup)
                    .unwrap_or_else(|error| {
                        panic!("{dialect:?}/{polarity}/{startup:?}: {error:?}")
                    });
                check(&result, startup == TransientStartupMode::Uic);
            }
            let (_, checkpoint) = engine
                .run_tran_checkpointed(&source, 1.2e-9, 4e-12)
                .unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&source, &checkpoint, 2.5e-9, 4e-12)
                .unwrap();
            check(&resumed, false);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_current_controlled_impulse_drives_the_independent_rc_response_after_restart() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.gp_transient_phase_model = GpTransientPhaseModel::ExactDelay;
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        for polarity in [1.0, -1.0] {
            for gain in [-2.0, 0.5] {
                let source = Netlist::parse(&format!(
                    "controlled impulse\nVC c 0 {}\nVB b 0 {}\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\nVSTEP control 0 PWL(0 0 .5n 0 .5n {polarity} 2n {polarity})\nC1 control 0 1p\nF1 out 0 VSTEP {gain}\nC2 out 0 2p\nR2 out 0 1k\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(out) i(vstep) i(f1)\n.end\n",
                    2.0*polarity,0.7*polarity,if polarity>0.0 {"NPN"} else {"PNP"},
                )).unwrap();
                let exact = |time: f64| {
                    let after =
                        time >= 0.5e-9 || (time - 0.5e-9).abs() < 8.0 * f64::EPSILON * 0.5e-9;
                    if after {
                        0.5 * gain * polarity * (-(time - 0.5e-9) / 2e-9).exp()
                    } else {
                        0.0
                    }
                };
                let error = |result: &TransientResult| {
                    result
                        .time
                        .iter()
                        .zip(result.try_voltage_waveform_named("out").unwrap())
                        .map(|(&time, &value)| (value - exact(time)).abs())
                        .fold(0.0_f64, f64::max)
                };
                let coarse = engine.run_tran(&source, 2e-9, 4e-12).unwrap();
                let full = engine.run_tran(&source, 2e-9, 1e-12).unwrap();
                let (coarse_error, fine_error) = (error(&coarse), error(&full));
                eprintln!(
                    "CCCS RC {dialect:?}/{polarity}/{gain}: coarse={coarse_error:e}, fine={fine_error:e}"
                );
                assert!(
                    fine_error < 0.7 * coarse_error,
                    "RC refinement did not reduce the waveform error"
                );
                let (_, checkpoint) = engine
                    .run_tran_checkpointed(&source, 0.4e-9, 1e-12)
                    .unwrap();
                let (resumed, _) = engine
                    .run_tran_resume(&source, &checkpoint, 2e-9, 1e-12)
                    .unwrap();
                for result in [&full, &resumed] {
                    let voltage = result.try_voltage_waveform_named("out").unwrap();
                    for (&time, &actual) in result.time.iter().zip(voltage) {
                        let expected = exact(time);
                        assert!(
                            (actual - expected).abs() < 2e-6,
                            "{dialect:?}/{polarity}/{gain} at {time:e}: {actual:e} vs {expected:e}"
                        );
                    }
                    for name in ["vstep", "f1"] {
                        assert!(
                            result
                                .try_branch_current_waveform_named(name)
                                .unwrap()
                                .iter()
                                .all(|current| current.abs() < 1e-12)
                        );
                        let expected = -polarity * 1e-12 * if name == "f1" { gain } else { 1.0 };
                        let impulse = trace(result, name)
                            .points
                            .iter()
                            .find(|point| point.time == 0.5e-9)
                            .unwrap();
                        assert!(
                            (impulse.charge_coulombs - expected).abs()
                                < 1e-24 + expected.abs() * 1e-10
                        );
                    }
                    assert!(
                        result
                            .current_impulses
                            .as_ref()
                            .unwrap()
                            .iter()
                            .all(|trace| trace.complete)
                    );
                    assert_eq!(result.time.last(), Some(&2e-9));
                }
                assert_eq!(engine.convergence_quality().force_accepted_points, 0);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_resistive_ccvs_preserves_transport_charge_currents_and_restart() {
    check_gp_finite_ccvs(false);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_probe_ccvs_preserves_transport_charge_currents_and_restart() {
    check_gp_finite_ccvs(true);
}

fn check_gp_finite_ccvs(probe: bool) {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.gp_transient_phase_model = GpTransientPhaseModel::ExactDelay;
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        for polarity in [1.0, -1.0] {
            for (resistance, sign) in if probe {
                [(1000.0, 1.0), (1000.0, -1.0)]
            } else {
                [(1.0, 1.0), (-2.0, 1.0)]
            } {
                // A floating sensing resistor keeps the sign and reference-node
                // mapping observable. Its MNA current remains a solved output.
                let (drive_node, control_name, probe_deck) = if probe {
                    (
                        "drive",
                        "vsense",
                        if sign > 0.0 {
                            "VSENSE drive ctrl 0\n"
                        } else {
                            "VSENSE ctrl drive 0\n"
                        },
                    )
                } else {
                    ("ctrl", "rc", "")
                };
                let source = Netlist::parse(&format!(
                    "finite CCVS GP\nVC c 0 {}\nVREF ref 0 1.25\nVB {drive_node} ref DC {} SIN({} {} 1G)\n{probe_deck}RC ctrl ref {resistance}\nH1 b 0 {control_name} {}\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\n.options device zeroresistancetol=2\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) i(vc) i(h1) i(vb) i({control_name})\n.end\n",
                    2.0*polarity,0.35*polarity,0.35*polarity,0.5e-6*polarity,
                    2.0*resistance*sign, if polarity>0.0 {"NPN"} else {"PNP"},
                )).unwrap();
                let check = |result: &TransientResult, uic: bool| {
                    behavioral::check(result, dialect, polarity, None, "h1", uic);
                    let drive = result.try_branch_current_waveform_named("vb").unwrap();
                    let control = result
                        .try_branch_current_waveform_named(control_name)
                        .unwrap();
                    for ((&time, &drive), &control) in result.time.iter().zip(drive).zip(control) {
                        let expected = sign
                            * polarity
                            * (0.35 + 0.5e-6 * (std::f64::consts::TAU * 1e9 * time).sin())
                            / resistance;
                        assert!(
                            (control - expected).abs() < 1e-12,
                            "{dialect:?}/{polarity}/{resistance}: control at {time:e}"
                        );
                        assert!((control + sign * drive).abs() < 1e-12);
                    }
                    for name in ["vb", control_name] {
                        assert!(trace(result, name).complete);
                        assert!(
                            trace(result, name)
                                .points
                                .iter()
                                .all(|point| point.charge_coulombs == 0.0)
                        );
                    }
                    if uic {
                        let expected =
                            -polarity * TF * diode(0.7, thermal_voltage(dialect), dialect).0;
                        let initial = trace(result, "h1")
                            .points
                            .iter()
                            .find(|point| point.time == 0.0)
                            .unwrap();
                        assert!(
                            (initial.charge_coulombs - expected).abs()
                                < 1e-25 + expected.abs() * 1e-10
                        );
                    }
                    assert_eq!(engine.convergence_quality().force_accepted_points, 0);
                };
                for startup in [
                    TransientStartupMode::OperatingPoint,
                    TransientStartupMode::Uic,
                ] {
                    let (result, checkpoints) = engine
                        .run_tran_checkpoint_schedule_with_startup_mode(
                            &source,
                            2.5e-9,
                            4e-12,
                            startup,
                            &[1.2e-9],
                        )
                        .unwrap_or_else(|error| {
                            panic!("{dialect:?}/{polarity}/{resistance}/{startup:?}: {error}")
                        });
                    check(&result, startup == TransientStartupMode::Uic);
                    for checkpoint in checkpoints {
                        exact_restart(
                            &engine,
                            &source,
                            &result,
                            &checkpoint.checkpoint,
                            2.5e-9,
                            4e-12,
                        );
                    }
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_resistive_ccvs_step_preserves_charge_fanout_and_packed_restart() {
    check_gp_finite_ccvs_step(false);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_probe_ccvs_step_preserves_charge_fanout_and_packed_restart() {
    check_gp_finite_ccvs_step(true);
}

fn check_gp_finite_ccvs_step(probe: bool) {
    let jump = 0.5e-9;
    let after =
        |time: f64, event: f64| time >= event || (time - event).abs() <= 8.0 * f64::EPSILON * event;
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
        ] {
            let mut config = SimulationConfig::default().with_spice_dialect(dialect);
            config.gp_transient_phase_model = GpTransientPhaseModel::ExactDelay;
            config.integration_method = method;
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            for polarity in [1.0, -1.0] {
                let sign = if probe { polarity } else { 1.0 };
                let resistance = if probe { 1000.0 } else { 1.0 };
                let (drive_node, control_name, control_deck, drive_charge) = if probe {
                    (
                        "drive",
                        "vsense",
                        if sign > 0.0 {
                            "VSENSE drive ctrl 0\n"
                        } else {
                            "VSENSE ctrl drive 0\n"
                        },
                        0.0,
                    )
                } else {
                    ("ctrl", "rc", "CC ctrl 0 1p\n", -polarity * 0.0005 * 1e-12)
                };
                let gain = 2.0 * resistance * sign;
                let source=Netlist::parse(&format!(
                    "CCVS charge jump\nVC c 0 {}\nVB {drive_node} 0 PWL(0 {} .5n {} .5n {} 2n {})\nRC ctrl 0 {resistance}\n{control_deck}H1 b 0 {control_name} {gain}\nCB b 0 2p\nF1 copy 0 H1 2\nRF copy 0 1k\nCF copy 0 3p\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\n.options device zeroresistancetol=2\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) v(copy) i(vc) i(h1) i(vb) i({control_name}) i(f1)\n.end\n",
                    2.0*polarity,0.35*polarity,0.35*polarity,0.3505*polarity,0.3505*polarity,if polarity>0.0 {"NPN"} else {"PNP"},
                )).unwrap();
                let (result, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &source,
                        2e-9,
                        2e-12,
                        TransientStartupMode::OperatingPoint,
                        &[jump, 1.1e-9],
                    )
                    .unwrap_or_else(|error| panic!("{dialect:?}/{method:?}/{polarity}: {error}"));
                let vt = thermal_voltage(dialect);
                let base_current =
                    |v: f64| diode(v, vt, dialect).0 / 100.0 + diode(v - 2.0, vt, dialect).0;
                let charge =
                    TF * (diode(0.701, vt, dialect).0 - diode(0.7, vt, dialect).0) + 2e-12 * 0.001;
                let b = result.try_voltage_waveform_named("b").unwrap();
                let copy = result.try_voltage_waveform_named("copy").unwrap();
                let h = result.try_branch_current_waveform_named("h1").unwrap();
                let rc = result
                    .try_branch_current_waveform_named(control_name)
                    .unwrap();
                let ic = result.try_branch_current_waveform_named("vc").unwrap();
                let fanout = result.try_branch_current_waveform_named("f1").unwrap();
                for (i, &time) in result.time.iter().enumerate() {
                    let v = if after(time, jump) { 0.701 } else { 0.7 };
                    let delayed = if after(time, jump + DELAY) {
                        0.701
                    } else {
                        0.7
                    };
                    let expected_copy = if after(time, jump) {
                        2000.0 * base_current(0.701)
                            + (2000.0 * (base_current(0.7) - base_current(0.701))
                                + 2.0 * charge / 3e-12)
                                * (-(time - jump) / 3e-9).exp()
                    } else {
                        2000.0 * base_current(0.7)
                    };
                    assert!((polarity * b[i] - v).abs() < 1e-10);
                    assert!((polarity * rc[i] - v / gain).abs() < 1e-12);
                    assert!(
                        (-polarity * h[i] - base_current(v)).abs() < 2e-11,
                        "{dialect:?}/{method:?}: finite CCVS current at {time:e}"
                    );
                    assert!(
                        (-polarity * ic[i]
                            - (diode(delayed, vt, dialect).0
                                - 2.0 * diode(v - 2.0, vt, dialect).0))
                            .abs()
                            < 2e-11
                    );
                    assert!((fanout[i] - 2.0 * h[i]).abs() < 1e-15);
                    // BE global integration error, at dt/tau <= 1/1500,
                    // bounds this millivolt RC transient to two microvolts.
                    assert!(
                        (polarity * copy[i] - expected_copy).abs() < 2e-6,
                        "{dialect:?}/{method:?}: copy at {time:e}"
                    );
                }
                for clock in [jump, jump + DELAY] {
                    assert!(
                        result
                            .time
                            .iter()
                            .any(|&time| (time - clock).abs() <= 8.0 * f64::EPSILON * clock)
                    );
                }
                for (name, expected) in [
                    ("h1", -polarity * charge),
                    ("f1", -2.0 * polarity * charge),
                    ("vb", drive_charge),
                ] {
                    let observation = trace(&result, name);
                    assert!(observation.complete && observation.derivatives.is_empty());
                    if expected == 0.0 {
                        assert!(
                            observation
                                .points
                                .iter()
                                .all(|point| point.charge_coulombs == 0.0)
                        );
                        continue;
                    }
                    let observed = observation
                        .points
                        .iter()
                        .find(|point| point.time == jump)
                        .unwrap()
                        .charge_coulombs;
                    assert!(
                        (observed - expected).abs() < 1e-25 + expected.abs() * 1e-9,
                        "{dialect:?}/{method:?}/{name}: {observed:e} != {expected:e}"
                    );
                }
                assert!(
                    trace(&result, control_name)
                        .points
                        .iter()
                        .all(|point| point.charge_coulombs == 0.0)
                );
                for checkpoint in checkpoints {
                    exact_restart(
                        &engine,
                        &source,
                        &result,
                        &checkpoint.checkpoint,
                        2e-9,
                        2e-12,
                    );
                }
                assert_eq!(engine.convergence_quality().force_accepted_points, 0);
            }
        }
    }
}
