use super::*;
use rspice_core::engine::TransientStartupMode;
use rspice_core::{CurrentImpulseOwner, CurrentImpulseTrace};

fn trace<'a>(result: &'a TransientResult, name: &str) -> &'a CurrentImpulseTrace {
    result.current_impulses.as_ref().unwrap().iter().find(|trace|
        matches!(&trace.owner, CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case(name))).unwrap()
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
