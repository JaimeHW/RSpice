use super::causal::roots;
use super::*;
use rspice_veriloga_runtime::transport_delay::DelayEvent;

#[test]
fn causal_event_orders_current_ramp_gains_continuous_rate_only_through_storage() {
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        for storage in [false, true] {
            let capacitor = if storage { "C1 c 0 1u" } else { "" };
            let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(&format!(
                "current ramp regularity\nVs s 0 {}\nRc s c 1k\nVb b 0 {}\nI1 c 0 PWL(0 0 1 0 2 {})\n{capacitor}\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-16 VAF=20 TF=1n PTF=30 CJE=1p CJC=0)\n.end\n",
                2.0 * polarity,
                0.6 * polarity,
                1e-6 * polarity,
            ));
            let sources = roots(&mut circuit, 2.0);
            let before = history.clone();
            let point = engine
                .prepare_physical_event(
                    &circuit,
                    &history,
                    PhysicalEventStep {
                        incoming: &incoming,
                        time: 1.0,
                        dt: 1.0,
                        phase_events: PhysicalEventOrders::FromCauses {
                            sources: &sources,
                            accepted_time: 0.0,
                        },
                    },
                    &options(),
                    1e-20,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(history, before);
            let sample = point.bjt.values[0].phase_sample.unwrap();
            let order = DelayEventOrder::AtLeast(if storage { 2 } else { 1 });
            assert_eq!(sample.event.unwrap().order, order);
            let collector = circuit.get_node_by_name("c").unwrap() - 1;
            let collector_rate = point.state.coordinate_rates[collector].unwrap();
            if storage {
                // At the start of a current ramp the forcing value is still
                // zero. C*dV/dt therefore stays zero; d2V/dt2 changes instead.
                close(collector_rate, 0.0, 1e-7);
                close(sample.outgoing_slope.unwrap(), 0.0, 1e-14);
            } else {
                // With algebraic collector KCL, dV/dt responds immediately
                // to the ramp slope. The finite Early voltage couples this
                // rate into the GP forward current as well.
                assert!(collector_rate.abs() > 1e-5);
                assert!(sample.outgoing_slope.unwrap().abs() > 1e-12);
            }
            let mut solution = point.state.solution.clone();
            accept(
                &engine,
                &mut circuit,
                &mut matrix,
                &mut history,
                &point,
                &mut solution,
            )
            .unwrap();
            let checkpoint = history.phase[0].as_ref().unwrap().checkpoint();
            let restored = DelayBuffer::from_checkpoint(checkpoint).unwrap();
            assert_eq!(
                restored.next_event_after(1.0).unwrap().unwrap().order,
                order
            );
        }
    }
}

#[test]
fn causal_event_orders_delayed_corners_retain_incidence_and_constitutive_limits() {
    for (storage, c1_only) in [(false, false), (true, false), (true, true)] {
        for declared in [
            DelayEventOrder::Unknown,
            DelayEventOrder::AtLeast(1),
            DelayEventOrder::AtLeast(2),
            DelayEventOrder::AtLeast(5),
        ] {
            let capacitor = if storage { "C1 c 0 1u" } else { "" };
            let join = if c1_only { "VJE=1.2 FC=.5" } else { "" };
            let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(&format!(
                "delayed corner regularity\nVs s 0 2\nRc s c 1k\nVb b 0 .6\n{capacitor}\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 VAF=20 TF=1n PTF=30 CJE=1p CJC=0 {join})\n.end\n",
            ));
            let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
            let sources = roots(&mut circuit, 2.0 * delay);
            let phase = history.phase[0].as_mut().unwrap();
            let current = phase.accepted_samples().next_back().unwrap().1;
            let mut replacement = DelayBuffer::new(0);
            replacement
                .accept_event(
                    0.0,
                    DelayEvent {
                        left: current,
                        right: current,
                        order: declared,
                    },
                    delay,
                    None,
                )
                .unwrap();
            replacement
                .accept_sample(0.5 * delay, current, delay, None)
                .unwrap();
            let accepted_time = 0.75 * delay;
            replacement
                .accept_sample(accepted_time, current, delay, None)
                .unwrap();
            *phase = replacement;
            let before = history.clone();
            let point = engine
                .prepare_physical_event(
                    &circuit,
                    &history,
                    PhysicalEventStep {
                        incoming: &incoming,
                        time: delay,
                        dt: delay - accepted_time,
                        phase_events: PhysicalEventOrders::FromCauses {
                            sources: &sources,
                            accepted_time,
                        },
                    },
                    &options(),
                    1e-20,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(history, before);
            let expected = match (storage, declared) {
                (false, DelayEventOrder::Unknown) => DelayEventOrder::Unknown,
                (true, DelayEventOrder::Unknown) | (false, DelayEventOrder::AtLeast(1)) => {
                    DelayEventOrder::AtLeast(1)
                }
                (true, DelayEventOrder::AtLeast(1)) | (false, DelayEventOrder::AtLeast(2)) => {
                    DelayEventOrder::AtLeast(2)
                }
                _ => DelayEventOrder::AtLeast(3),
            };
            let expected = if c1_only {
                expected.merge(DelayEventOrder::AtLeast(2))
            } else {
                expected
            };
            assert_eq!(
                point.bjt.values[0]
                    .phase_sample
                    .unwrap()
                    .event
                    .unwrap()
                    .order,
                expected
            );
            if declared == DelayEventOrder::AtLeast(2) {
                let mut solution = point.state.solution.clone();
                accept(
                    &engine,
                    &mut circuit,
                    &mut matrix,
                    &mut history,
                    &point,
                    &mut solution,
                )
                .unwrap();
                let checkpoint = history.phase[0].as_ref().unwrap().checkpoint();
                let restored = DelayBuffer::from_checkpoint(checkpoint).unwrap();
                assert_eq!(
                    restored.next_event_after(delay).unwrap().unwrap().order,
                    expected
                );
            }
        }
    }
}

#[test]
fn causal_event_orders_simultaneous_voltage_corner_limits_current_smoothing() {
    let (engine, mut circuit, _, incoming, history) = fixture(
        "simultaneous corner regularity\nVc c 0 2\nVb b 0 PWL(0 .6 1 .6 2 .7)\nI1 c 0 PWL(0 0 1 0 2 1u)\nQ1 c b 0 qm\n.model qm NPN(IS=1e-16 VAF=20 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let sources = roots(&mut circuit, 2.0);
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 1.0,
                phase_events: PhysicalEventOrders::FromCauses {
                    sources: &sources,
                    accepted_time: 0.0,
                },
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let sample = point.bjt.values[0].phase_sample.unwrap();
    assert_eq!(sample.event.unwrap().order, DelayEventOrder::AtLeast(1));
    assert!(sample.outgoing_slope.unwrap().abs() > 1e-6);
}
