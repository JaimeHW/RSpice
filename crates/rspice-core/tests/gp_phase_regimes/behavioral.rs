use super::*;

// Independent clamped-terminal GP laws. The derivative term belongs to
// instantaneous charge; only transport sees the delay and constant prehistory.
pub(super) fn check(
    result: &TransientResult,
    dialect: SpiceDialect,
    polarity: f64,
    current: Option<&str>,
    base_name: &str,
    uic: bool,
) {
    let voltage = result.try_voltage_waveform_named("b").unwrap();
    let collector = result.try_branch_current_waveform_named("vc").unwrap();
    let base = result.try_branch_current_waveform_named(base_name).unwrap();
    let extra = current.map(|name| result.try_branch_current_waveform_named(name).unwrap());
    let omega = std::f64::consts::TAU * 1e9;
    let vt = thermal_voltage(dialect);
    for (index, &time) in result.time.iter().enumerate() {
        let vb = 0.7 + 1e-6 * (omega * time).sin();
        let rate = 1e-6 * omega * (omega * time).cos();
        let delayed = 0.7 + 1e-6 * (omega * (time - DELAY).max(0.0)).sin();
        let (forward, conductance) = diode(vb, vt, dialect);
        let reverse = diode(vb - 2.0, vt, dialect).0;
        let forcing = if current.is_some() {
            1e-6 * (omega * time).cos()
        } else {
            0.0
        };
        // UIC has zero transport prehistory. The exact arrival publishes its
        // outgoing side; allow ULP-sized decimal PTF conversion in locating
        // that clock, while retaining the same current accuracy gate.
        let arrival = (time - DELAY).abs() <= 8.0 * f64::EPSILON * DELAY;
        let expected_collector = if uic && time < DELAY && !arrival {
            -2.0 * reverse
        } else {
            diode(delayed, vt, dialect).0 - 2.0 * reverse
        };
        let expected_base = forward / 100.0 + reverse + TF * conductance * rate + forcing;
        assert!(
            (polarity * voltage[index] - vb).abs() < 1e-10,
            "voltage at {time:e}"
        );
        assert!(
            (-polarity * collector[index] - expected_collector).abs() < 2e-11,
            "{dialect:?}/{polarity}: collector at {time:e}: {} vs {expected_collector:e}",
            -polarity * collector[index]
        );
        assert!(
            (-polarity * base[index] - expected_base).abs() < 3e-10,
            "{dialect:?}/{polarity}: base at {time:e}: {} vs {expected_base:e}",
            -polarity * base[index]
        );
        if let Some(extra) = extra {
            assert!((polarity * extra[index] - forcing).abs() < 1e-16);
        }
    }
    assert_eq!(result.time.last(), Some(&2.5e-9));
    assert!(
        result
            .current_impulses
            .as_ref()
            .unwrap()
            .iter()
            .all(|trace| trace.complete)
    );
}

fn deck(polarity: f64, current: bool) -> Netlist {
    let mut text = include_str!("../testdata/qualification/gp-behavioral-event-gap.cir")
        .replace("VC c 0 2", &format!("VC c 0 {}", 2.0 * polarity))
        .replace(
            "V={.7+1u*sin(2*pi*1e9*time)}",
            &format!("V={{{polarity}*(.7+1u*sin(2*pi*1e9*time))}}"),
        );
    if polarity < 0.0 {
        text = text.replace("qm NPN", "qm PNP");
    }
    let extra = if current {
        format!("BI b 0 I={{{polarity}*1u*cos(2*pi*1e9*time)}}\n.save v(b) i(vc) i(bb) i(bi)\n.end")
    } else {
        ".save v(b) i(vc) i(bb)\n.end".into()
    };
    Netlist::parse(&text.replace(".end", &extra)).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_behavioral_prescribed_forcing_has_physical_event_equations() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let engine = Engine::new(SimulationConfig {
            gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
            ..SimulationConfig::default().with_spice_dialect(dialect)
        });
        for polarity in [1.0, -1.0] {
            for current in [false, true] {
                let source = deck(polarity, current);
                let result = engine.run_tran(&source, 2.5e-9, 4e-12).unwrap();
                check(
                    &result,
                    dialect,
                    polarity,
                    current.then_some("bi"),
                    "bb",
                    false,
                );
                let (_, checkpoint) = engine
                    .run_tran_checkpointed(&source, 1.2e-9, 4e-12)
                    .unwrap();
                let (resumed, _) = engine
                    .run_tran_resume(&source, &checkpoint, 2.5e-9, 4e-12)
                    .unwrap();
                check(
                    &resumed,
                    dialect,
                    polarity,
                    current.then_some("bi"),
                    "bb",
                    false,
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_behavioral_uic_preserves_charge_impulses_and_zero_transport_prehistory() {
    use rspice_core::CurrentImpulseOwner;
    use rspice_core::engine::TransientStartupMode;
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let engine = Engine::new(SimulationConfig {
            gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
            ..SimulationConfig::default().with_spice_dialect(dialect)
        });
        for polarity in [1.0, -1.0] {
            let result = engine
                .run_tran_with_startup_mode(
                    &deck(polarity, true),
                    2.5e-9,
                    4e-12,
                    TransientStartupMode::Uic,
                )
                .unwrap();
            check(&result, dialect, polarity, Some("bi"), "bb", true);
            let impulses = result.current_impulses.as_ref().unwrap();
            let base = impulses.iter().find(|trace| matches!(
                &trace.owner, CurrentImpulseOwner::Branch { branch_name } if branch_name.eq_ignore_ascii_case("bb")
            )).unwrap();
            let initial = base.points.iter().find(|point| point.time == 0.0).unwrap();
            let expected = -polarity * TF * diode(0.7, thermal_voltage(dialect), dialect).0;
            assert!((initial.charge_coulombs - expected).abs() < 1e-25 + expected.abs() * 1e-10);
            let current = impulses.iter().find(|trace| matches!(
                &trace.owner, CurrentImpulseOwner::Branch { branch_name } if branch_name.eq_ignore_ascii_case("bi")
            )).unwrap();
            assert!(
                current
                    .points
                    .iter()
                    .all(|point| point.charge_coulombs == 0.0)
            );
            assert!(
                result
                    .time
                    .iter()
                    .any(|&time| (time - DELAY).abs() <= 8.0 * f64::EPSILON * DELAY)
            );
        }
    }
}

/// A capacitor charged through a CCCS impulse discharges through a nonlinear,
/// explicitly time-dependent behavioral current. Its closed-form charge law
/// does not use any core interpolation, derivative or device helper.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_nodal_behavioral_current_preserves_nonlinear_decay_and_packed_restart() {
    use rspice_core::engine::{
        TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode,
    };
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
            for polarity in [1.0, -1.0] {
                let deck = Netlist::parse(&format!(
                    "nonlinear behavioral decay\nVSTEP drive 0 PWL(0 0 .5n 0 .5n {polarity} 2n {polarity})\nCIN drive 0 1p\nFCHARGE out 0 VSTEP 2\nCOUT out 0 2p\nBDIS out 0 I={{.001*(1+time/1n)*v(out)^3}}\nVC c 0 2\nVB b 0 .6\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=100 TF=.2n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-6 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(out) i(bdis) i(cout)\n.end\n"
                )).unwrap();
                let mut config = SimulationConfig {
                    gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
                    integration_method: method,
                    ..SimulationConfig::default().with_spice_dialect(dialect)
                };
                config.convergence_config.gmin_target = 0.0;
                let engine = Engine::new(config);
                let (result, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &deck,
                        2e-9,
                        1e-12,
                        TransientStartupMode::OperatingPoint,
                        &[0.5e-9, 1.1e-9],
                    )
                    .unwrap_or_else(|error| panic!("{dialect:?}/{method:?}/{polarity}: {error}"));
                let voltage = result.try_voltage_waveform_named("out").unwrap();
                let current = result.try_branch_current_waveform_named("bdis").unwrap();
                for (index, &time) in result.time.iter().enumerate() {
                    let expected = if time < 0.5e-9 {
                        0.0
                    } else {
                        let elapsed = time - 0.5e-9;
                        let integral = elapsed * (1.0 + (time + 0.5e-9) / 2e-9);
                        polarity / (1.0 + 1e9 * integral).sqrt()
                    };
                    assert!(
                        (voltage[index] - expected).abs() < 3e-4,
                        "{dialect:?}/{method:?}/{polarity}, t={time:e}: {} vs {expected}",
                        voltage[index]
                    );
                    let physical = 0.001 * (1.0 + time / 1e-9) * voltage[index].powi(3);
                    assert!((current[index] - physical).abs() < 1e-15);
                }
                let impulse = result.current_impulses.as_ref().unwrap().iter().find(|trace|
                    matches!(&trace.owner, rspice_core::CurrentImpulseOwner::Branch { branch_name }
                        if branch_name.eq_ignore_ascii_case("bdis"))).unwrap();
                assert!(impulse.complete);
                assert!(
                    impulse
                        .points
                        .iter()
                        .all(|point| point.charge_coulombs == 0.0)
                );
                assert!(impulse.derivatives.is_empty());
                for checkpoint in checkpoints {
                    let checkpoint = TransientCheckpoint::from_bytes(
                        &checkpoint
                            .checkpoint
                            .to_bytes(TransientCheckpointEncoding::Packed)
                            .unwrap(),
                    )
                    .unwrap();
                    let (resumed, _) = engine
                        .run_tran_resume(&deck, &checkpoint, 2e-9, 1e-12)
                        .unwrap();
                    let seam = result
                        .time
                        .iter()
                        .position(|time| *time == resumed.time[0])
                        .unwrap();
                    assert_eq!(resumed.time, result.time[seam..]);
                    for (actual, expected) in resumed
                        .voltages
                        .iter()
                        .chain(&resumed.branch_currents)
                        .zip(result.voltages.iter().chain(&result.branch_currents))
                    {
                        if expected.is_empty() {
                            assert!(actual.is_empty());
                        } else {
                            assert_eq!(actual, &expected[seam..]);
                        }
                    }
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_nodal_behavioral_current_couples_to_gp_charge_at_startup() {
    use rspice_core::engine::TransientStartupMode;
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for polarity in [1.0, -1.0] {
            let kind = if polarity > 0.0 { "NPN" } else { "PNP" };
            let deck = Netlist::parse(&format!(
                "GP nodal current\nVC c 0 {}\nVB b 0 DC {} SIN({} {} 1G)\nBI b 0 I={{{polarity}*1u*(v(b)/.7)^2*cos(2*pi*1e9*time)}}\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) i(vc) i(vb) i(bi)\n.end\n",
                2.0*polarity, 0.7*polarity, 0.7*polarity, 1e-6*polarity,
            )).unwrap();
            let mut config = SimulationConfig {
                gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
                ..SimulationConfig::default().with_spice_dialect(dialect)
            };
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            for startup in [
                TransientStartupMode::OperatingPoint,
                TransientStartupMode::Uic,
            ] {
                let result = engine
                    .run_tran_with_startup_mode(&deck, 2.5e-9, 4e-12, startup)
                    .unwrap_or_else(|e| panic!("{dialect:?}/{polarity}/{startup:?}: {e}"));
                let base = result.try_branch_current_waveform_named("VB").unwrap();
                let current = result.try_branch_current_waveform_named("BI").unwrap();
                let voltage = result.try_voltage_waveform_named("b").unwrap();
                let omega = std::f64::consts::TAU * 1e9;
                for (index, &time) in result.time.iter().enumerate() {
                    let vb = 0.7 + 1e-6 * (omega * time).sin();
                    let rate = 1e-6 * omega * (omega * time).cos();
                    let (forward, conductance) = diode(vb, thermal_voltage(dialect), dialect);
                    let reverse = diode(vb - 2.0, thermal_voltage(dialect), dialect).0;
                    let forcing = 1e-6 * (vb / 0.7).powi(2) * (omega * time).cos();
                    assert!((polarity * voltage[index] - vb).abs() < 1e-10);
                    assert!((polarity * current[index] - forcing).abs() < 1e-16);
                    let physical = forward / 100.0 + reverse + TF * conductance * rate + forcing;
                    assert!(
                        (-polarity * base[index] - physical).abs() < 3e-10,
                        "{dialect:?}/{polarity}/{startup:?}, t={time:e}: {} vs {physical:e}",
                        -polarity * base[index]
                    );
                }
                let impulse = result.current_impulses.as_ref().unwrap().iter().find(|trace|
                    matches!(&trace.owner, rspice_core::CurrentImpulseOwner::Branch { branch_name }
                        if branch_name.eq_ignore_ascii_case("bi"))).unwrap();
                assert!(impulse.complete && impulse.derivatives.is_empty());
                assert!(
                    impulse
                        .points
                        .iter()
                        .all(|point| point.charge_coulombs == 0.0)
                );
            }
        }
    }
}
