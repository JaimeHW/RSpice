//! Native diode physical charge, finite currents and accepted restart state.
use super::*;
use rspice_core::CurrentImpulseOwner;
use rspice_core::engine::TransientStartupMode;

fn engine(dialect: SpiceDialect, method: IntegrationMethod) -> Engine {
    let mut config = SimulationConfig {
        gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
        integration_method: method,
        ..SimulationConfig::default().with_spice_dialect(dialect)
    };
    config.convergence_config.gmin_target = 0.0;
    Engine::new(config)
}

fn branch_impulses<'a>(
    result: &'a TransientResult,
    name: &str,
) -> &'a rspice_core::CurrentImpulseTrace {
    result.current_impulses.as_ref().unwrap().iter().find(|trace|
        matches!(&trace.owner, CurrentImpulseOwner::Branch { branch_name } if branch_name.eq_ignore_ascii_case(name))
    ).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_diode_charge_jumps_preserve_finite_currents_startup_and_packed_restart() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        // Native diodes retain the ngspice constant pair outside Xyce mode.
        let vt = if dialect == SpiceDialect::Xyce {
            1.380_622_6e-23 / 1.602_191_8e-19 * 300.15
        } else {
            1.380_648_52e-23 / 1.602_176_620_8e-19 * 300.15
        };
        let current = |v: f64| 1e-16 * (v / vt).exp_m1();
        let charge = |v: f64| {
            let cj = 2e-12;
            let phi = 0.8;
            let m = 0.4;
            let boundary = 0.4;
            let q_boundary = cj * phi * (1.0 - 0.5_f64.powf(1.0 - m)) / (1.0 - m);
            let c_boundary = cj * 0.5_f64.powf(-m);
            let depletion = if v <= boundary {
                cj * phi * (1.0 - (1.0 - v / phi).powf(1.0 - m)) / (1.0 - m)
            } else {
                let delta = v - boundary;
                q_boundary + c_boundary * delta + 0.5 * c_boundary * m / (phi * 0.5) * delta * delta
            };
            depletion + 0.3e-9 * current(v)
        };
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
        ] {
            let engine = engine(dialect, method);
            for startup in [
                TransientStartupMode::OperatingPoint,
                TransientStartupMode::Uic,
            ] {
                let deck = Netlist::parse("native diode charge event\nVD n 0 DC .2 PWL(0 .2 1n .2 1n .6 3n .6)\nD1 n 0 dm IC=.1\n.model dm D(IS=1e-16 N=1 CJO=2p VJ=.8 M=.4 FC=.5 TT=.3n)\nVC c 0 2\nVB b 0 .6\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=100 TF=.1n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(n) i(vd) i(d1)\n.end\n").unwrap();
                let (result, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &deck,
                        3e-9,
                        5e-12,
                        startup,
                        &[1e-9, 1.7e-9],
                    )
                    .unwrap_or_else(|error| panic!("{dialect:?}/{method:?}/{startup:?}: {error}"));
                let voltage = result.try_voltage_waveform_named("n").unwrap();
                let diode = result.try_branch_current_waveform_named("d1").unwrap();
                let source = result.try_branch_current_waveform_named("vd").unwrap();
                for (index, &time) in result.time.iter().enumerate() {
                    let expected = if time >= 1e-9 { 0.6 } else { 0.2 };
                    assert!((voltage[index] - expected).abs() < 1e-10);
                    assert!(
                        (diode[index] - current(expected)).abs() < 1e-15,
                        "{dialect:?}/{method:?}/{startup:?} I(D1) at {time:e}: {}",
                        diode[index]
                    );
                    // Ordinary affine companion assembly subtracts C*V/dt
                    // terms. Bound its roundoff explicitly; exact event rows
                    // have no timestep companion and retain the 1 fA gate.
                    let dt = result.step_sizes[index];
                    let rounding = if time == 0.0 || time == 1e-9 {
                        0.0
                    } else {
                        8.0 * f64::EPSILON * 3.3e-12 * expected / dt
                    };
                    assert!(
                        (diode[index] + source[index]).abs() < 1e-15 + rounding,
                        "{dialect:?}/{method:?}/{startup:?} KCL at {time:e}: diode={:e}, source={:e}",
                        diode[index],
                        source[index]
                    );
                }
                let trace = branch_impulses(&result, "d1");
                let source = branch_impulses(&result, "vd");
                assert!(trace.complete && source.complete);
                for (time, delta) in [
                    (
                        0.0,
                        if startup == TransientStartupMode::Uic {
                            charge(0.2) - charge(0.1)
                        } else {
                            0.0
                        },
                    ),
                    (1e-9, charge(0.6) - charge(0.2)),
                ] {
                    let observed = |trace: &rspice_core::CurrentImpulseTrace| {
                        trace
                            .points
                            .iter()
                            .find(|point| point.time == time)
                            .map_or(0.0, |point| point.charge_coulombs)
                    };
                    assert!(
                        (observed(trace) - delta).abs() < 1e-25 + 1e-10 * delta.abs(),
                        "{dialect:?}/{startup:?} diode impulse at {time:e}"
                    );
                    assert!((observed(source) + delta).abs() < 1e-25 + 1e-10 * delta.abs());
                }
                for checkpoint in checkpoints {
                    exact_restart(&engine, &deck, &result, &checkpoint.checkpoint, 3e-9, 5e-12);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_diode_series_resistance_retains_junction_charge_and_finite_event_current() {
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
            let engine = engine(dialect, method);
            // Negligible Shockley conduction isolates the model's constant CJO
            // behind its real, builder-externalized RS. tau=2 ns, independent
            // of the device evaluator and event/integration implementation. The
            // 0.1-tau window keeps BE global error below 20 uV at a 2 ps step.
            let deck=Netlist::parse("native diode series event\nVD s 0 DC .2 PWL(0 .2 1n .2 1n .6 1.2n .6)\nD1 s 0 dm\n.model dm D(IS=1e-30 N=1 RS=2k CJO=1p VJ=1 M=0 TT=0)\nVC c 0 2\nVB b 0 .6\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=100 TF=.1n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(s) i(vd) i(d1)\n.end\n").unwrap();
            let (result, checkpoints) = engine
                .run_tran_checkpoint_schedule_with_startup_mode(
                    &deck,
                    1.2e-9,
                    2e-12,
                    TransientStartupMode::OperatingPoint,
                    &[1e-9, 1.1e-9],
                )
                .unwrap_or_else(|error| panic!("{dialect:?}/{method:?}: {error}"));
            let voltage = result.try_voltage_waveform_named("s").unwrap();
            let current = result.try_branch_current_waveform_named("d1").unwrap();
            let source = result.try_branch_current_waveform_named("vd").unwrap();
            for (i, &time) in result.time.iter().enumerate() {
                let arrived = time >= 1e-9;
                let decay = if arrived {
                    (-(time - 1e-9) / 2e-9).exp()
                } else {
                    1.0
                };
                let expected = 0.6 - 0.4 * decay;
                let expected_current = if arrived { 0.4 / 2000.0 * decay } else { 0.0 };
                assert!(
                    (voltage[i] - 2000.0 * current[i] - expected).abs() < 2e-5,
                    "{dialect:?}/{method:?} junction voltage at {time:e}: {}, expected {expected:e}",
                    voltage[i] - 2000.0 * current[i]
                );
                assert!(
                    (current[i] - expected_current).abs() < 1e-8,
                    "{dialect:?}/{method:?} current at {time:e}: {}",
                    current[i]
                );
                assert!((current[i] + source[i]).abs() < 1e-12);
            }
            for name in ["vd", "d1"] {
                assert!(
                    branch_impulses(&result, name)
                        .points
                        .iter()
                        .all(|point| point.charge_coulombs.abs() < 1e-24)
                );
            }
            for checkpoint in checkpoints {
                exact_restart(
                    &engine,
                    &deck,
                    &result,
                    &checkpoint.checkpoint,
                    1.2e-9,
                    2e-12,
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_diode_smooth_model_terms_do_not_create_endless_delay_events() {
    // BV/IKF/ISR used to reset every GP arrival to order zero, even in an
    // analytic diode region. False arrivals accumulated before the real 1 ns
    // source jump until their rounded clocks made the transient abort.
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
            let engine = engine(dialect, method);
            for (bias, extra) in [
                (0.2, ""),
                (0.2, "BV=100"),
                (0.2, "IKF=1m"),
                (0.2, "ISR=1e-15"),
                (-0.2, "BV=100 IKR=1m ISR=1e-15"),
                (
                    0.2,
                    "BV=3 IKF=1m IKR=1m ISR=1e-15 JSW=1e-16 NS=1.2 IKP=1m JTUN=1e-18 JTUNSW=1e-18 NTUN=2 CJP=.2p TT=.1n",
                ),
                (
                    -3.05,
                    "BV=3 IKF=1m IKR=1m ISR=1e-15 JSW=1e-16 NS=1.2 IKP=1m TT=.1n",
                ),
            ] {
                let deck = Netlist::parse(&format!("diode event continuity\nVD d 0 {bias}\nD1 d 0 dm PJ=1\n.model dm D(IS=1e-16 CJO=1p {extra})\nVC c 0 2\nVB b 0 DC .6 PWL(0 .6 1n .6 1n .61 20n .61)\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=100 BR=1 TF=.1n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-6 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) v(d) i(vc) i(d1) i(vd)\n.end\n")).unwrap();
                let (result, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &deck,
                        20e-9,
                        1e-9,
                        TransientStartupMode::OperatingPoint,
                        &[1e-9, 2e-9],
                    )
                    .unwrap_or_else(|error| {
                        panic!("{dialect:?}/{method:?}/{bias}/{extra}: {error}")
                    });
                // The delay is .1 ns but ordinary adaptation must span the
                // settled tail. This count bounds event growth without a
                // machine-dependent wall-clock threshold.
                assert!(
                    result.time.len() < 80,
                    "{dialect:?}/{method:?}/{bias}/{extra}: {} samples",
                    result.time.len()
                );
                let base = result.try_voltage_waveform_named("b").unwrap();
                let collector = result.try_branch_current_waveform_named("vc").unwrap();
                let junction = result.try_voltage_waveform_named("d").unwrap();
                let diode_current = result.try_branch_current_waveform_named("d1").unwrap();
                let supply = result.try_branch_current_waveform_named("vd").unwrap();
                let at_or_after = |t: f64, event: f64| {
                    t >= event || (t - event).abs() <= 8.0 * f64::EPSILON * event
                };
                let vt = thermal_voltage(dialect);
                for (index, &time) in result.time.iter().enumerate() {
                    let voltage = if at_or_after(time, 1e-9) { 0.61 } else { 0.6 };
                    let delayed = if at_or_after(time, 1e-9 + 0.1e-9) {
                        0.61
                    } else {
                        0.6
                    };
                    let expected =
                        diode(delayed, vt, dialect).0 - 2.0 * diode(voltage - 2.0, vt, dialect).0;
                    assert!((base[index] - voltage).abs() < 1e-10);
                    assert!((junction[index] - bias).abs() < 1e-10);
                    assert!(
                        (collector[index] + expected).abs() < 2e-11,
                        "{dialect:?}/{method:?}/{extra}: collector at {time:e}"
                    );
                    assert!(
                        (diode_current[index] - diode_current[0]).abs()
                            < 1e-15 + 1e-10 * diode_current[0].abs()
                    );
                    // Physical event KCL has no timestep companion or
                    // C*V/dt cancellation allowance.
                    if time == 0.0
                        || time == 1e-9
                        || (time - (1e-9 + 0.1e-9)).abs() <= 8.0 * f64::EPSILON * time
                    {
                        assert!(
                            (diode_current[index] + supply[index]).abs()
                                < 1e-15 + 1e-10 * diode_current[index].abs()
                        );
                    }
                }
                for event in [1e-9, 1e-9 + 0.1e-9] {
                    assert!(
                        result
                            .time
                            .iter()
                            .any(|t| (*t - event).abs() <= 8.0 * f64::EPSILON * event)
                    );
                }
                let trace = branch_impulses(&result, "d1");
                assert!(
                    trace.complete
                        && trace
                            .points
                            .iter()
                            .all(|point| point.charge_coulombs == 0.0)
                );
                for checkpoint in checkpoints {
                    exact_restart(&engine, &deck, &result, &checkpoint.checkpoint, 20e-9, 1e-9);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_xyce_diode_injection_preserves_charge_impulses_and_finite_currents() {
    for (temperature, coefficient) in [(27.0, 0.0), (127.0, 0.01), (-23.0, 0.01), (127.0, -0.01)] {
        let vt: f64 = (temperature + 273.15) * 1.380_622_6e-23 / 1.602_191_8e-19;
        let ratio: f64 = (temperature + 273.15) / 300.15;
        let saturation = 1e-14 * ((ratio - 1.0) * 1.11 / vt + 3.0 * ratio.ln()).exp();
        let knee = 1e-3 * (1.0 + coefficient * (temperature - 27.0));
        let v0 = vt * (1.0 + 1e-3 / saturation).ln();
        let v1 = vt * (1.0 + 5e-3 / saturation).ln();
        for gmin in [0.0, 1e-3] {
            let current = |v: f64| {
                let normal = saturation * (v / vt).exp_m1() + gmin * v;
                if knee > 0.0 {
                    normal / (1.0 + normal / knee).sqrt()
                } else {
                    normal
                }
            };
            for method in [
                IntegrationMethod::BackwardEuler,
                IntegrationMethod::Trapezoidal,
                IntegrationMethod::Gear2,
            ] {
                let engine = engine(SpiceDialect::Xyce, method);
                let deck = Netlist::parse(&format!("Xyce injection event\nVD d 0 DC {v0:.17e} PWL(0 {v0:.17e} 1n {v0:.17e} 1n {v1:.17e} 3n {v1:.17e})\nD1 d 0 dm TEMP={temperature}\n.model dm D(IS=1e-14 N=1 EG=1.11 XTI=3 TNOM=27 IKF=1m TIKF={coefficient} CJO=0 TT=2n)\nVC c 0 2\nVB b 0 .6\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=100 TF=.1n PTF=57.29577951308232)\n.options GMIN={gmin:e} RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(d) i(d1) i(vd)\n.end\n")).unwrap();
                let (result, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &deck,
                        3e-9,
                        5e-12,
                        TransientStartupMode::OperatingPoint,
                        &[1e-9, 1.7e-9],
                    )
                    .unwrap();
                let diode = result.try_branch_current_waveform_named("d1").unwrap();
                let source = result.try_branch_current_waveform_named("vd").unwrap();
                for (index, &time) in result.time.iter().enumerate() {
                    let expected = current(if time >= 1e-9 { v1 } else { v0 });
                    assert!(
                        (diode[index] - expected).abs() < 1e-13 + expected.abs() * 1e-9,
                        "TEMP={temperature}/{coefficient}/{method:?}/{gmin}: I(D1) at {time:e}: {} vs {expected:e}",
                        diode[index]
                    );
                    if time == 0.0 || time == 1e-9 {
                        assert!(
                            (diode[index] + source[index]).abs() < 1e-13 + expected.abs() * 1e-9
                        );
                    }
                }
                let expected = 2e-9 * (current(v1) - current(v0));
                for (name, sign) in [("d1", 1.0), ("vd", -1.0)] {
                    let trace = branch_impulses(&result, name);
                    assert!(trace.complete);
                    let jump = trace.points.iter().find(|p| p.time == 1e-9).unwrap();
                    assert!(
                        (jump.charge_coulombs - sign * expected).abs()
                            < 1e-25 + expected.abs() * 1e-10
                    );
                }
                for checkpoint in checkpoints {
                    exact_restart(&engine, &deck, &result, &checkpoint.checkpoint, 3e-9, 5e-12);
                }
            }
        }
    }
}
