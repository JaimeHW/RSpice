use super::*;

#[test]
fn gp_right_trial_holds_incoming_transport_through_nonlinear_bias_probes() {
    for polarity in [1.0, -1.0] {
        for private in [false, true] {
            for scale in [1.0, 6.0] {
                let bjt = transistor(polarity, private, scale);
                let delay = bjt.legacy_excess_phase_delay();
                let incoming = bjt.charge_snapshot(polarity * 2.0, polarity * 0.67, 0.0, 0.0);
                let left = forward_reference(&bjt, polarity, &incoming.reduction.internal_voltages);
                let anchor = 0.25 * left;
                let mut history = DelayBuffer::new(0);
                history.accept_sample(0.0, anchor, delay, None).unwrap();
                let accepted = history.clone();
                // The query is either prehistory, 3/5 of the incoming interval,
                // or 15/16 of it. Its endpoint is the independently supplied
                // left state, regardless of the subsequent right-side bias.
                for (factor, weight) in [(0.5, 0.0), (2.5, 0.6), (16.0, 0.9375)] {
                    let delayed = (1.0 - weight) * anchor + weight * left;
                    let trial = BjtPhaseTrial {
                        history: &history,
                        time: factor * delay,
                        left_limit: Some(left),
                    };
                    for base in [0.61, 0.69, 0.75] {
                        let right = bjt.charge_snapshot(polarity * 2.0, polarity * base, 0.0, 0.0);
                        let internal = right.reduction.internal_voltages;
                        let forward = forward_reference(&bjt, polarity, &internal);
                        let correction = trial.correction(&bjt, &internal).unwrap();
                        assert!(
                            (correction.current - (delayed - forward)).abs()
                                <= 5e-14 * (delayed.abs() + forward.abs()),
                            "polarity={polarity} private={private} scale={scale} factor={factor}"
                        );
                        for index in [
                            BJT_VBI_STATE_INDEX,
                            BJT_VCI_STATE_INDEX,
                            BJT_VEI_STATE_INDEX,
                        ] {
                            let h = 1e-7;
                            let mut plus = internal;
                            let mut minus = internal;
                            plus[index] += h;
                            minus[index] -= h;
                            // The delayed incoming value cancels in this
                            // difference; only the present physical current
                            // responds to the right-side Newton perturbation.
                            let expected = -(forward_reference(&bjt, polarity, &plus)
                                - forward_reference(&bjt, polarity, &minus))
                                / (2.0 * h);
                            assert!(
                                (correction.d_internal[index] - expected).abs()
                                    <= 2e-7 * expected.abs(),
                                "polarity={polarity} factor={factor} index={index}: {} != {expected}",
                                correction.d_internal[index]
                            );
                        }
                    }
                }
                assert_eq!(history, accepted, "right trials changed accepted history");
            }
        }
    }
}

#[test]
fn gp_right_trial_refuses_invalid_left_state_without_changing_history() {
    let bjt = transistor(1.0, true, 1.0);
    let snapshot = bjt.charge_snapshot(2.0, 0.68, 0.0, 0.0);
    let delay = bjt.legacy_excess_phase_delay();
    let history = delay_history(&bjt, 1e-5);
    let before = history.clone();
    for left in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let trial = BjtPhaseTrial {
            history: &history,
            time: 3.0 * delay,
            left_limit: Some(left),
        };
        assert!(
            trial
                .correction(&bjt, &snapshot.reduction.internal_voltages)
                .unwrap_err()
                .contains("left limit must be finite")
        );
    }
    let trial = BjtPhaseTrial {
        history: &history,
        time: 2.0 * delay,
        left_limit: Some(2e-5),
    };
    assert!(
        trial
            .correction(&bjt, &snapshot.reduction.internal_voltages)
            .unwrap_err()
            .contains("differs from its accepted knot")
    );
    assert_eq!(history, before);
}
