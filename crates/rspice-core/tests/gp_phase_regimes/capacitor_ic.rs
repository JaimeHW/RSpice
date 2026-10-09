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
