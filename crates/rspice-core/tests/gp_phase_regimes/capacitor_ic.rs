use super::*;
use rspice_core::engine::TransientStartupMode;

fn xyce_engine(method: IntegrationMethod, max_step: f64) -> Engine {
    let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
    config.gp_transient_phase_model = GpTransientPhaseModel::ExactDelay;
    config.integration_method = method;
    config.max_timestep = max_step;
    config.min_timestep = max_step * 1e-3;
    config.convergence_config.gmin_target = 0.0;
    Engine::new(config)
}

fn actions<'a>(result: &'a TransientResult, name: &str) -> &'a rspice_core::CurrentImpulseTrace {
    result.current_impulses.as_ref().unwrap().iter().find(|trace| matches!(&trace.owner,
        rspice_core::CurrentImpulseOwner::Branch { branch_name } if branch_name.eq_ignore_ascii_case(name))).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_capacitor_ic_preserves_rc_decay_and_outgoing_current() {
    let deck = Netlist::parse("RC initial charge with exact GP\nVb b 0 .65\nVc c 0 2\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=80 TF=1n PTF=30 CJE=2p CJC=.2p)\nV1 in 0 1\nR1 in out 100\nC1 out 0 10p IC=.25\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n").unwrap();
    for method in [
        IntegrationMethod::Trapezoidal,
        IntegrationMethod::Gear2,
        IntegrationMethod::TrapGear,
    ] {
        for startup in [
            TransientStartupMode::OperatingPoint,
            TransientStartupMode::Uic,
        ] {
            let result = xyce_engine(method, 1e-12)
                .run_tran_with_startup_mode(&deck, 3e-9, 1e-12, startup)
                .unwrap();
            let voltage = result.try_voltage_waveform_named("out").unwrap();
            let current = result.try_branch_current_waveform_named("c1").unwrap();
            assert!((voltage[0] - 0.25).abs() < 1e-12);
            assert!(
                (current[0] - 0.0075).abs() < 1e-12,
                "{startup:?}/{method:?}: {}",
                current[0]
            );
            // tau=100 ohms*10 pF=1 ns; the prescribed initial charge
            // releases into the resistor with its physical outgoing current.
            for (i, &time) in result.time.iter().enumerate() {
                let decay = (-time / 1e-9).exp();
                let expected = 1.0 - 0.75 * decay;
                assert!(
                    (voltage[i] - expected).abs() < 3e-6,
                    "{startup:?}/{method:?} at {time:e}: {} != {expected}",
                    voltage[i]
                );
                assert!((current[i] - 0.0075 * decay).abs() < 3e-8);
                assert!((current[i] - (1.0 - voltage[i]) / 100.0).abs() < 1e-12);
            }
            let trace = actions(&result, "c1");
            assert!(
                trace.complete && trace.points.is_empty() && trace.derivatives.is_empty(),
                "{trace:?}"
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_capacitor_ic_charge_fanout_survives_packed_restart() {
    for (terminals, initial, sign) in [("a reference", ".25", 1.0), ("reference a", "-.25", -1.0)] {
        let deck = Netlist::parse(&format!("Capacitor IC current fanout with exact GP\nVb b 0 .65\nVc c 0 2\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 BF=80 TF=1n PTF=30 CJE=2p CJC=.2p)\nVref reference 0 .125\nV1 a reference PWL(0 .25 1n .25 1n .75 4n .75)\nC1 {terminals} 2p IC={initial}\nF1 out 0 V1 -2\nR1 out 0 100\nC2 out 0 4p IC=0\nCzero out 0 0 IC=0\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n")).unwrap();
        for method in [
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
        ] {
            let engine = xyce_engine(method, 0.5e-12);
            let (result, checkpoints) = engine
                .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                    &deck,
                    4e-9,
                    0.5e-12,
                    TransientStartupMode::Uic,
                    &[1e-9, 1.4e-9],
                    &rspice_core::NoAbort,
                )
                .unwrap();
            // C1 transfers 2 pF*.5 V=1 pC. F1 removes twice that charge
            // from C2, so V(out) jumps by -2 pC/4 pF=-.5 V and decays
            // with tau=100 ohms*4 pF=.4 ns. Czero carries neither part.
            for (name, charge) in [
                ("c1", sign * 1e-12),
                ("v1", -1e-12),
                ("f1", 2e-12),
                ("c2", -2e-12),
            ] {
                let trace = actions(&result, name);
                assert!(trace.complete && trace.derivatives.is_empty(), "{trace:?}");
                assert_eq!(trace.points.len(), 1, "{trace:?}");
                assert_eq!(trace.points[0].time, 1e-9);
                assert!(
                    (trace.points[0].charge_coulombs - charge).abs() < 1e-23,
                    "{trace:?}"
                );
            }
            let zero = actions(&result, "czero");
            assert!(zero.complete && zero.points.is_empty() && zero.derivatives.is_empty());
            let output = result.try_voltage_waveform_named("out").unwrap();
            let capacitor = result.try_branch_current_waveform_named("c2").unwrap();
            let zero = result.try_branch_current_waveform_named("czero").unwrap();
            for (i, &time) in result.time.iter().enumerate() {
                let expected = if time < 1e-9 {
                    0.0
                } else {
                    -0.5 * (-(time - 1e-9) / 0.4e-9).exp()
                };
                assert!(
                    (output[i] - expected).abs() < 4e-6,
                    "{method:?} at {time:e}: {} != {expected}",
                    output[i]
                );
                assert!((capacitor[i] + output[i] / 100.0).abs() < 1e-10);
                assert_eq!(zero[i], 0.0);
            }
            for saved in checkpoints {
                let checkpoint = TransientCheckpoint::from_bytes(
                    &saved
                        .checkpoint
                        .to_bytes(TransientCheckpointEncoding::Packed)
                        .unwrap(),
                )
                .unwrap();
                let (resumed, _) = engine
                    .run_tran_resume(&deck, &checkpoint, 4e-9, 0.5e-12)
                    .unwrap();
                let offset = result
                    .time
                    .iter()
                    .position(|&time| time == checkpoint.time)
                    .unwrap();
                assert_eq!(resumed.time, result.time[offset..]);
                for (actual, full) in resumed
                    .voltages
                    .iter()
                    .zip(&result.voltages)
                    .chain(resumed.branch_currents.iter().zip(&result.branch_currents))
                {
                    assert_eq!(
                        actual,
                        &full[offset..],
                        "{method:?} at checkpoint {}",
                        checkpoint.time
                    );
                }
                let mut expected = result.current_impulses.clone().unwrap();
                for trace in &mut expected {
                    trace.points.retain(|point| point.time > checkpoint.time);
                    trace
                        .derivatives
                        .retain(|point| point.time > checkpoint.time);
                }
                assert_eq!(resumed.current_impulses, Some(expected));
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_capacitor_ic_shares_nonlinear_junction_charge_without_double_counting() {
    let vt = thermal_voltage(SpiceDialect::Xyce);
    let forward = |base: f64| 1e-16 * (base / vt).exp_m1();
    let intrinsic_jump = 1e-9 * (forward(0.65) - forward(0.6));
    let capacitor_jump = 2e-12 * (0.65 - 0.6);
    for polarity in [1.0, -1.0] {
        let kind = if polarity > 0.0 { "NPN" } else { "PNP" };
        let deck = Netlist::parse(&format!(
            "Shared intrinsic and capacitor IC charge\nVC c 0 {}\nVB b 0 PWL(0 {} 1n {} 1n {} 2.5n {})\nQ1 c b 0 qm IC=0,0\n.model qm {kind}(IS=1e-16 BF=80 BR=1 TF=1n PTF=57.29577951308232)\nC1 b 0 2p IC={}\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n",
            2.0 * polarity, 0.6 * polarity, 0.6 * polarity,
            0.65 * polarity, 0.65 * polarity, 0.6 * polarity,
        )).unwrap();
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
        ] {
            // The clamping voltage source and capacitor IC would duplicate
            // the OP constraint. UIC releases the capacitor into its DAE row.
            let startup = TransientStartupMode::Uic;
            let result = xyce_engine(method, 5e-12)
                .run_tran_with_startup_mode(&deck, 2.5e-9, 5e-12, startup)
                .unwrap_or_else(|error| panic!("{kind}/{method:?}/{startup:?}: {error}"));
            let capacitor = actions(&result, "c1");
            assert!(capacitor.complete && capacitor.derivatives.is_empty());
            assert_eq!(capacitor.points.len(), 1, "{capacitor:?}");
            assert_eq!(capacitor.points[0].time, 1e-9);
            assert!(
                (capacitor.points[0].charge_coulombs - polarity * capacitor_jump).abs() < 2e-24
            );
            let source = actions(&result, "vb");
            assert!(source.complete && source.derivatives.is_empty());
            assert_eq!(source.points.len(), 2, "{source:?}");
            for point in &source.points {
                // Authored C initial charge is already present in UIC;
                // only the GP junction draws additional startup charge.
                let expected = -polarity
                    * if point.time == 0.0 {
                        1e-9 * forward(0.6)
                    } else {
                        assert_eq!(point.time, 1e-9);
                        intrinsic_jump + capacitor_jump
                    };
                assert!(
                    (point.charge_coulombs - expected).abs() < 2e-24,
                    "{point:?}, expected {expected:e}"
                );
            }
            let icap = result.try_branch_current_waveform_named("c1").unwrap();
            let ib = result.try_branch_current_waveform_named("vb").unwrap();
            let ic = result.try_branch_current_waveform_named("vc").unwrap();
            for (i, &time) in result.time.iter().enumerate() {
                let base = if time < 1e-9 { 0.6 } else { 0.65 };
                let reverse = 1e-16 * ((base - 2.0) / vt).exp_m1();
                let arrived = time >= 1e-9 || (time - 1e-9).abs() < 8.0 * f64::EPSILON * 1e-9;
                let changed = time >= 2e-9 || (time - 2e-9).abs() < 8.0 * f64::EPSILON * 2e-9;
                let delayed = if !arrived {
                    0.0
                } else {
                    forward(if changed { 0.65 } else { 0.6 })
                };
                assert!(
                    icap[i].abs() < 1e-14,
                    "{kind}/{method:?}/{startup:?} at {time:e}: I_C={}",
                    icap[i]
                );
                assert!((-polarity * ib[i] - forward(base) / 80.0 - reverse).abs() < 1e-12);
                assert!(
                    (-polarity * ic[i] - delayed + 2.0 * reverse).abs() < 1e-12,
                    "{kind}/{method:?}/{startup:?} at {time:e}: I_C={} delayed={delayed:e}",
                    ic[i]
                );
            }
        }
    }
}
