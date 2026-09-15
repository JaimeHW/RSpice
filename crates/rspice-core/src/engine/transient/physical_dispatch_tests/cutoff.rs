use super::*;

/// Step response of u' + alpha*u + beta*u(t-delay) = forcing, with zero
/// prehistory. Expand the Laplace resolvent in delayed powers; each term is
/// the integral of exp(-alpha*t)*t^n/n!. This oracle does not use device,
/// interpolation, event-classification or integration implementation code.
fn delayed_step(time: Value, delay: Value, alpha: Value, beta: Value, forcing: Value) -> Value {
    if time <= 0.0 {
        return 0.0;
    }
    let mut response = 0.0;
    let mut weight = forcing / alpha;
    for n in 0..=(time / delay).floor() as usize {
        let x = alpha * (time - n as Value * delay);
        let mut term = 1.0;
        let mut sum = term;
        for k in 1..=n {
            term *= x / k as Value;
            sum += term;
        }
        response += weight * (1.0 - (-x).exp() * sum);
        weight *= -beta / alpha;
    }
    response
}

fn check_delayed_feedback(dialect: SpiceDialect, method: IntegrationMethod) {
    let text = "Delayed GP collector feedback\nVs s 0 3\nRc s c 1k\nVb b 0 .6\nC1 c 0 1n\nIstep 0 c PWL(0 0 .37u 0 .37u 100u 8u 100u)\nQ1 c b 0 qm\n.model qm NPN(IS=1e-14 BF=100 BR=1 VAF=1 TF=1u PTF=57.29577951308232 TNOM=27)\n.options gmin=0 temp=27 reltol=1e-6 abstol=1e-13 vntol=1e-9\n.tran .07u 8u\n.end\n";
    let delay = 1e-6 * (57.295_779_513_082_32 * std::f64::consts::PI / 180.0);
    let config = SimulationConfig {
        integration_method: method,
        ..SimulationConfig::default().with_spice_dialect(dialect)
    };
    // The collector time constant is 1 us. Resolve the first-order BE
    // global error at h/tau=1e-3; second-order methods use h/tau=1e-2.
    // These bounds also keep the independent linear-history error small.
    let max_step = if method == IntegrationMethod::BackwardEuler {
        1e-9
    } else if dialect == SpiceDialect::Xyce {
        // Xyce takes two first-order intervals after each physical event.
        // Halve h to resolve their O(h^2) local restart errors in this budget.
        5e-9
    } else {
        1e-8
    };
    let scheduled: &[Value] = if method == IntegrationMethod::Gear2 || dialect == SpiceDialect::Xyce
    {
        &[0.37e-6, 3.25e-6]
    } else {
        &[]
    };
    let (result, checkpoints) =
        run_with_configuration(text, 8e-6, max_step, None, scheduled, config.clone());
    let vt: Value = match dialect {
        SpiceDialect::Xyce => 1.380_622_6e-23 * 300.15 / 1.602_191_8e-19,
        _ => 1.380_649e-23 * 300.15 / 1.602_176_634e-19,
    };
    let forward = 1e-14 * (0.6 / vt).exp_m1();
    let alpha = 1.0 / (1000.0 * 1e-9);
    let beta = forward / 1e-9; // VAF=1, so dIF/dVC=forward / VAF.
    let bias = (3.0 / 1000.0 - forward * (1.0 - 0.6)) / (1.0 / 1000.0 + forward);
    let voltage = result.try_voltage_waveform_named("c").unwrap();
    assert!((voltage[0] - bias).abs() < 1e-9);
    let mut maximum_error = 0.0_f64;
    let mut maximum_error_time = 0.0;
    let mut omitted_delay_error = 0.0_f64;
    for (&time, &actual) in result.time.iter().zip(voltage) {
        let elapsed = time - 0.37e-6;
        let expected = bias + delayed_step(elapsed, delay, alpha, beta, 1e5);
        if (actual - expected).abs() > maximum_error {
            maximum_error = (actual - expected).abs();
            maximum_error_time = time;
        }
        let instantaneous =
            bias + 1e5 / (alpha + beta) * (1.0 - (-(alpha + beta) * elapsed.max(0.0)).exp());
        omitted_delay_error = omitted_delay_error.max((actual - instantaneous).abs());
    }
    // Reverse cubic leakage is below 4e-14 A throughout this bias range,
    // hence <4e-11 V at the collector. The oracle retains the full delayed
    // Early feedback; it does not reduce the transistor to zero phase.
    let budget = if method == IntegrationMethod::BackwardEuler {
        3e-5
    } else {
        3e-6
    };
    eprintln!(
        "GP cutoff {dialect:?}/{method:?}: points={}, max_error={maximum_error:e} at {maximum_error_time:e}, omitted_delay_error={omitted_delay_error:e}",
        result.time.len()
    );
    assert!(
        maximum_error < budget,
        "{dialect:?}/{method:?}: {maximum_error:e}"
    );
    assert!(omitted_delay_error > 1e-3);
    for captured in &checkpoints {
        let checkpoint = &captured.checkpoint;
        let history = &checkpoint.accepted_junction_transient_history().bjt_history;
        if checkpoint.time == 0.37e-6 && method == IntegrationMethod::Gear2 {
            assert!(history.accepted_dt_prev > 0.0);
            assert_eq!(history.accepted_dt_prev_prev, 0.0);
        }
        if checkpoint.time == 3.25e-6 {
            let phase = history.phase[0].as_ref().unwrap();
            let smooth = phase.next_event_after(checkpoint.time).unwrap().unwrap();
            assert_eq!(
                smooth.order,
                rspice_veriloga_runtime::transport_delay::DelayEventOrder::AtLeast(3)
            );
            assert!(
                !result.time.contains(&smooth.time),
                "smooth arrival still forced the grid"
            );
        }
        let restored = TransientCheckpoint::from_bytes(
            &checkpoint
                .to_bytes(TransientCheckpointEncoding::Packed)
                .unwrap(),
        )
        .unwrap();
        let (resumed, _) = run_with_configuration(
            text,
            8e-6,
            max_step,
            Some(&restored),
            scheduled
                .iter()
                .copied()
                .filter(|&time| time > checkpoint.time)
                .collect::<Vec<_>>()
                .as_slice(),
            config.clone(),
        );
        let seam = result
            .time
            .iter()
            .position(|&time| time == checkpoint.time)
            .unwrap();
        assert!(
            resumed.time == result.time[seam..],
            "{dialect:?}/{method:?}: time grid changed after resume at {}",
            checkpoint.time
        );
        for (resumed, baseline) in resumed.voltages.iter().zip(&result.voltages) {
            assert_eq!(resumed, &baseline[seam..]);
        }
    }
}

#[test]
fn physical_dispatch_cutoff_analytic_backward_euler() {
    check_delayed_feedback(SpiceDialect::Ngspice, IntegrationMethod::BackwardEuler);
}

#[test]
fn physical_dispatch_cutoff_analytic_trapezoidal() {
    check_delayed_feedback(SpiceDialect::Ngspice, IntegrationMethod::Trapezoidal);
}

#[test]
fn physical_dispatch_cutoff_analytic_gear2_and_restart() {
    check_delayed_feedback(SpiceDialect::Ngspice, IntegrationMethod::Gear2);
}

#[test]
fn physical_dispatch_cutoff_analytic_xyce_trapgear() {
    check_delayed_feedback(SpiceDialect::Xyce, IntegrationMethod::TrapGear);
}
