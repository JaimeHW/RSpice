//! Nonlinear voltage constraints coupled to physical charge and GP memory.
use super::*;
use rspice_core::CurrentImpulseOwner;
use rspice_core::engine::TransientStartupMode;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_nodal_voltage_preserves_transport_charge_startup_and_packed_restart() {
    let wave = |time: f64| {
        let omega = std::f64::consts::TAU * 1e9;
        let sine = (omega * time).sin();
        let base = 0.7 + 1e-6 * (sine + sine.powi(3) + 0.25 * (time / 1e-9).powi(2));
        let rate =
            1e-6 * (omega * (omega * time).cos() * (1.0 + 3.0 * sine * sine) + 0.5 * time / 1e-18);
        (base, rate)
    };
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let mut config = SimulationConfig {
            gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
            ..SimulationConfig::default().with_spice_dialect(dialect)
        };
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        let vt = thermal_voltage(dialect);
        for polarity in [1.0, -1.0] {
            let kind = if polarity > 0.0 { "NPN" } else { "PNP" };
            let deck=Netlist::parse(&format!(
                "GP nonlinear voltage\nVC c 0 {}\nVX x 0 SIN(0 1 1G)\nBB b 0 V={{{polarity}*(.7+1u*(v(x)+v(x)^3+.25*(time/1n)^2))}}\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) i(vc) i(bb) i(vx)\n.end\n",2.0*polarity
            )).unwrap();
            for startup in [
                TransientStartupMode::OperatingPoint,
                TransientStartupMode::Uic,
            ] {
                let (result, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &deck,
                        2.5e-9,
                        4e-12,
                        startup,
                        &[1.2e-9],
                    )
                    .unwrap_or_else(|error| panic!("{dialect:?}/{polarity}/{startup:?}: {error}"));
                let base = result.try_voltage_waveform_named("b").unwrap();
                let ib = result.try_branch_current_waveform_named("bb").unwrap();
                let ic = result.try_branch_current_waveform_named("vc").unwrap();
                let ix = result.try_branch_current_waveform_named("vx").unwrap();
                for (i, &time) in result.time.iter().enumerate() {
                    let (voltage, rate) = wave(time);
                    let (forward, conductance) = diode(voltage, vt, dialect);
                    let reverse = diode(voltage - 2.0, vt, dialect).0;
                    let arrived =
                        time >= DELAY || (time - DELAY).abs() <= 8.0 * f64::EPSILON * DELAY;
                    let delayed = if startup == TransientStartupMode::Uic && !arrived {
                        0.0
                    } else {
                        diode(wave((time - DELAY).max(0.0)).0, vt, dialect).0
                    };
                    let expected_base = forward / 100.0 + reverse + TF * conductance * rate;
                    let expected_collector = delayed - 2.0 * reverse;
                    assert!(
                        (polarity * base[i] - voltage).abs() < 1e-10,
                        "{dialect:?}/{startup:?} voltage at {time:e}"
                    );
                    assert!(
                        (-polarity * ib[i] - expected_base).abs() < 3e-10,
                        "{dialect:?}/{startup:?} base at {time:e}: {} vs {expected_base:e}",
                        -polarity * ib[i]
                    );
                    assert!(
                        (-polarity * ic[i] - expected_collector).abs() < 2e-11,
                        "{dialect:?}/{startup:?} collector at {time:e}"
                    );
                    assert!(ix[i].abs() < 1e-16);
                }
                let impulses = result.current_impulses.as_ref().unwrap();
                assert!(impulses.iter().all(|trace| trace.complete));
                if startup == TransientStartupMode::Uic {
                    let trace=impulses.iter().find(|trace| matches!(&trace.owner,CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case("bb"))).unwrap();
                    let initial = trace.points.iter().find(|point| point.time == 0.0).unwrap();
                    let expected = -polarity * TF * diode(0.7, vt, dialect).0;
                    assert!(
                        (initial.charge_coulombs - expected).abs() < 1e-25 + expected.abs() * 1e-10
                    );
                }
                for checkpoint in checkpoints {
                    exact_restart(
                        &engine,
                        &deck,
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_nodal_voltage_feedback_preserves_nonlinear_charge_jump_and_current_fanout() {
    // The small root of y-.25*y^2=.19+.1*x^2 gives .2 before the
    // step. Its outgoing value and depletion charge are closed-form laws.
    let before = 0.2;
    let after = 2.0 - (4.0_f64 - 4.0 * 0.29).sqrt();
    let charge = |voltage: f64| 2e-12 * voltage + 2e-12 * (1.0 - (1.0 - voltage).sqrt());
    let impulse = -(charge(after) - charge(before));
    let initial_w = -2.0 * impulse / 3e-12;
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
            let mut config = SimulationConfig {
                gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
                integration_method: method,
                ..SimulationConfig::default().with_spice_dialect(dialect)
            };
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            let deck=Netlist::parse(
                "nonlinear voltage feedback step\nVX x 0 PWL(0 0 .5n 0 .5n 1 2n 1)\nBV out 0 V={.19+.1*v(x)^2+.25*v(out)^2}\nCY out 0 2p\nQD 0 out 0 qload\n.model qload NPN(IS=1e-16 BF=100 BR=1 CJE=1p VJE=1 MJE=.5 FC=.5 CJC=0 TF=0 TR=0)\nF1 w 0 BV 2\nCW w 0 3p\nRW w 0 1k\nVC c 0 2\nVB b 0 .6\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=100 TF=.2n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-7 VNTOL=1e-10 ABSTOL=1e-16 CHGTOL=1e-26\n.save v(out) v(w) i(bv) i(f1)\n.end\n"
            ).unwrap();
            let (result, checkpoints) = engine
                .run_tran_checkpoint_schedule_with_startup_mode(
                    &deck,
                    2e-9,
                    1e-12,
                    TransientStartupMode::OperatingPoint,
                    &[0.5e-9, 1.1e-9],
                )
                .unwrap_or_else(|error| panic!("{dialect:?}/{method:?}: {error}"));
            let out = result.try_voltage_waveform_named("out").unwrap();
            let w = result.try_voltage_waveform_named("w").unwrap();
            let ib = result.try_branch_current_waveform_named("bv").unwrap();
            let fanout = result.try_branch_current_waveform_named("f1").unwrap();
            let vt = thermal_voltage(dialect);
            // Both base junctions of the load transistor are forward biased;
            // its base current is I_BE/BF + I_BC/BR. Only CJE stores charge.
            let leakage = |v: f64| 1.01 * diode(v, vt, dialect).0;
            let steady_before = 2000.0 * leakage(before);
            let steady_after = 2000.0 * leakage(after);
            for (i, &time) in result.time.iter().enumerate() {
                let arrived =
                    time >= 0.5e-9 || (time - 0.5e-9).abs() <= 8.0 * f64::EPSILON * 0.5e-9;
                let voltage = if arrived { after } else { before };
                let expected = if arrived {
                    steady_after
                        + (steady_before + initial_w - steady_after)
                            * (-(time - 0.5e-9) / 3e-9).exp()
                } else {
                    steady_before
                };
                assert!(
                    (out[i] - voltage).abs() < 1e-10,
                    "{dialect:?}/{method:?} voltage at {time:e}"
                );
                assert!(
                    (w[i] - expected).abs() < 2e-5,
                    "{dialect:?}/{method:?} fanout at {time:e}: {} vs {expected}",
                    w[i]
                );
                assert!((fanout[i] - 2.0 * ib[i]).abs() < 1e-15);
            }
            let traces = result.current_impulses.as_ref().unwrap();
            assert!(traces.iter().all(|trace| trace.complete));
            for (name, expected) in [("bv", impulse), ("f1", 2.0 * impulse)] {
                let trace=traces.iter().find(|trace| matches!(&trace.owner,CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case(name))).unwrap();
                let observed = trace
                    .points
                    .iter()
                    .find(|point| point.time == 0.5e-9)
                    .unwrap()
                    .charge_coulombs;
                assert!(
                    (observed - expected).abs() < 1e-25 + expected.abs() * 1e-8,
                    "{dialect:?}/{method:?}/{name}: {observed:e} vs {expected:e}"
                );
            }
            for checkpoint in checkpoints {
                exact_restart(&engine, &deck, &result, &checkpoint.checkpoint, 2e-9, 1e-12);
            }
        }
    }
}
