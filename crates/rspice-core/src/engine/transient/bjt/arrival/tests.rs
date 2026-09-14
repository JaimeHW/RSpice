use super::*;
use crate::abort_signal::CountingAbort;
use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayConfiguration};

fn fixture() -> (crate::CircuitData, BjtTransientHistory) {
    let deck = Netlist::parse(
        "phase arrival ownership\n\
         Qslow c b 0 slow\nQcorner c b 0 fast\nQjump c b 0 fast\nQordinary c b 0 ordinary\n\
         Rc c 0 1k\nRb b 0 1k\n\
         .model slow NPN(TF=2 PTF=180)\n.model fast NPN(TF=1 PTF=180)\n\
         .model ordinary NPN\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let circuit = engine.build_circuit(&deck).unwrap();
    let solution = vec![0.0; circuit.matrix_size()];
    let mut history =
        Engine::initialize_bjt_history(&circuit, &solution, ReactiveHistorySeed::SolvedBias);
    Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
    for (index, phase) in history.phase.iter_mut().enumerate() {
        if let Some(phase) = phase {
            let delay = circuit.bjts.devices[index].legacy_excess_phase_delay();
            *phase = DelayBuffer::new(0);
            let left = if index == 1 { 2.0 } else { 1.0 };
            phase
                .accept_discontinuity(0.0, left, 2.0, delay, None)
                .unwrap();
        }
    }
    advance(&circuit, &mut history, 0.5);
    (circuit, history)
}

fn advance(circuit: &crate::CircuitData, history: &mut BjtTransientHistory, time: Value) {
    for (bjt, phase) in circuit.bjts.devices.iter().zip(&mut history.phase) {
        if let Some(phase) = phase {
            phase
                .accept_sample(time, 2.0, bjt.legacy_excess_phase_delay(), None)
                .unwrap();
        }
    }
}

#[test]
fn gp_phase_arrival_merges_event_orders_across_devices_and_checkpoint_restore() {
    use rspice_veriloga_runtime::transport_delay::DelayEvent;
    let (circuit, mut history) = fixture();
    for (index, order) in [(1, 3), (2, 1)] {
        let delay = circuit.bjts.devices[index].legacy_excess_phase_delay();
        let mut phase = DelayBuffer::new(0);
        phase
            .accept_event(
                0.0,
                DelayEvent {
                    left: 2.0,
                    right: 2.0,
                    order: DelayEventOrder::AtLeast(order),
                },
                delay,
                None,
            )
            .unwrap();
        phase.accept_sample(0.5, 2.0, delay, None).unwrap();
        history.phase[index] = Some(phase);
    }
    let before = history.clone();
    let event = next(&circuit, &history, 0.5, 20.0, &NoAbort)
        .unwrap()
        .unwrap();
    assert_eq!(
        event.time,
        circuit.bjts.devices[1].legacy_excess_phase_delay()
    );
    assert_eq!(event.device_index, 1);
    assert_eq!(event.order, DelayEventOrder::AtLeast(1));
    assert_eq!(history, before);
    for phase in history.phase.iter_mut().flatten() {
        *phase = DelayBuffer::from_checkpoint(phase.checkpoint()).unwrap();
    }
    assert_eq!(
        next(&circuit, &history, 0.5, 20.0, &NoAbort).unwrap(),
        Some(event)
    );
    let mut legacy = history.phase[2].as_ref().unwrap().checkpoint();
    legacy.event_orders.clear();
    history.phase[2] = Some(DelayBuffer::from_checkpoint(legacy).unwrap());
    assert_eq!(
        next(&circuit, &history, 0.5, 20.0, &NoAbort)
            .unwrap()
            .unwrap()
            .order,
        DelayEventOrder::Unknown
    );
}

#[test]
fn gp_phase_arrival_owns_earliest_corner_jump_ties_and_checkpoint_replay() {
    let (circuit, mut history) = fixture();
    let fast = circuit.bjts.devices[1].legacy_excess_phase_delay();
    let slow = circuit.bjts.devices[0].legacy_excess_phase_delay();
    let expected = Some(PhaseArrival {
        order: DelayEventOrder::Unknown,
        time: fast,
        device_index: 1,
    });
    let before = history.clone();
    for _rejected_trial in 0..3 {
        assert_eq!(
            next(&circuit, &history, 0.5, 20.0, &NoAbort).unwrap(),
            expected
        );
        assert_eq!(
            next(&circuit, &history, 0.5, fast.next_down(), &NoAbort).unwrap(),
            None
        );
        assert_eq!(
            next(&circuit, &history, 0.5, fast, &NoAbort).unwrap(),
            expected
        );
    }
    assert_eq!(history, before);
    for phase in history.phase.iter_mut().flatten() {
        *phase = DelayBuffer::from_checkpoint(phase.checkpoint()).unwrap();
    }
    assert_eq!(
        next(&circuit, &history, 0.5, 20.0, &NoAbort).unwrap(),
        expected
    );
    advance(&circuit, &mut history, fast);
    assert_eq!(
        next(&circuit, &history, fast, 20.0, &NoAbort).unwrap(),
        Some(PhaseArrival {
            order: DelayEventOrder::Unknown,
            time: slow,
            device_index: 0
        })
    );
    advance(&circuit, &mut history, slow);
    assert_eq!(
        next(&circuit, &history, slow, 20.0, &NoAbort).unwrap(),
        None
    );
    // Smooth accepted knots, even many delay widths apart, create no event.
    advance(&circuit, &mut history, 100.0);
    assert_eq!(
        next(&circuit, &history, 100.0, 1000.0, &NoAbort).unwrap(),
        None
    );
}

#[test]
fn gp_phase_arrival_refuses_misaligned_missing_and_mismatched_memory() {
    let (circuit, history) = fixture();
    for (time, stop) in [
        (Value::NAN, 1.0),
        (-1.0, 1.0),
        (0.5, Value::INFINITY),
        (0.5, 0.4),
    ] {
        assert!(next(&circuit, &history, time, stop, &NoAbort).is_err());
    }
    assert!(
        next(&circuit, &history, 0.5_f64.next_up(), 20.0, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains(&circuit.bjts.devices[0].name)
    );
    let mut bad = history.clone();
    bad.phase.pop();
    assert!(
        next(&circuit, &bad, 0.5, 20.0, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("population")
    );
    let mut bad = history.clone();
    bad.phase[0] = None;
    assert!(
        next(&circuit, &bad, 0.5, 20.0, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("missing")
    );
    let mut bad = history.clone();
    let mut checkpoint = bad.phase[0].as_ref().unwrap().checkpoint();
    checkpoint.configuration = Some(DelayConfiguration::Fixed {
        delay: circuit.bjts.devices[0]
            .legacy_excess_phase_delay()
            .next_up(),
    });
    bad.phase[0] = Some(DelayBuffer::from_checkpoint(checkpoint).unwrap());
    assert!(
        next(&circuit, &bad, 0.5, 20.0, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("different nominal")
    );
    let mut bad = history.clone();
    bad.phase[3] = bad.phase[0].clone();
    assert!(
        next(&circuit, &bad, 0.5, 20.0, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains(&circuit.bjts.devices[3].name)
    );
    let mut bad = history.clone();
    let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
    bad.phase[0]
        .as_mut()
        .unwrap()
        .eval_with_coefficients(1.0, 3.0, delay, None)
        .unwrap();
    assert!(
        next(&circuit, &bad, 0.5, 20.0, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("in-flight")
    );
    assert_eq!(history, fixture().1);
}

#[test]
fn gp_phase_arrival_cancellation_is_bounded_and_does_not_consume_events() {
    let (mut circuit, mut history) = fixture();
    let device = circuit.bjts.devices[0].clone();
    let phase = history.phase[0].clone();
    circuit.bjts.devices = vec![device; 512];
    history.phase = vec![phase; 512];
    let before = history.clone();
    for threshold in [0, 1, 2] {
        let abort = CountingAbort::new(threshold);
        assert!(matches!(
            next(&circuit, &history, 0.5, 20.0, &abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(abort.count(), threshold + 1);
        assert_eq!(abort.polls_after_abort(), 0);
    }
    assert_eq!(history, before);
}

#[test]
fn gp_phase_arrival_survives_bias_replay_and_refuses_floor_coalescence() {
    let (circuit, history) = fixture();
    let arrival = next(&circuit, &history, 0.5, 20.0, &NoAbort)
        .unwrap()
        .unwrap();
    let gap = arrival.time - 0.5;
    // Short source-activity cuts remain short; a widened replay or fitted
    // interval lands the physical event instead of leaping over it.
    assert_eq!(
        arrival.limit_step(&circuit, 0.5, 0.1, 0.6, 0.01).unwrap(),
        None
    );
    for width in [gap, 2.0 * gap, 10.0 * gap] {
        assert_eq!(
            arrival
                .limit_step(&circuit, 0.5, width, 0.5 + width, 0.01)
                .unwrap(),
            Some(gap)
        );
    }
    arrival.ensure_reachable(&circuit, 0.5, gap).unwrap();
    let error = arrival
        .ensure_reachable(&circuit, 0.5, gap.next_up())
        .unwrap_err()
        .to_string();
    assert!(error.contains(&circuit.bjts.devices[1].name) && error.contains("coalescing"));
    for minimum in [0.0, -1.0, Value::NAN, Value::INFINITY] {
        assert!(arrival.ensure_reachable(&circuit, 0.5, minimum).is_err());
    }
    for (width, clock) in [
        (Value::NAN, arrival.time),
        (gap, Value::NAN),
        (gap, 0.5),
        (0.0, arrival.time),
    ] {
        assert!(
            arrival
                .limit_step(&circuit, 0.5, width, clock, 0.01)
                .is_err()
        );
    }
    assert!(
        arrival
            .limit_step(&circuit, 0.5, 0.001, arrival.time, 0.01)
            .is_err()
    );
}

#[test]
fn gp_phase_arrival_preserves_sub_ulp_delays_and_exact_equation_clocks() {
    let deck = Netlist::parse(
        "tiny phase\nQ1 c b 0 m\nRc c 0 1k\nRb b 0 1k\n.model m NPN(TF=1e-20 PTF=180)\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let circuit = engine.build_circuit(&deck).unwrap();
    let mut history = Engine::initialize_bjt_history(
        &circuit,
        &vec![0.0; circuit.matrix_size()],
        ReactiveHistorySeed::SolvedBias,
    );
    Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
    let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
    assert!(delay > 0.0 && delay < Value::EPSILON);
    history.phase[0]
        .as_mut()
        .unwrap()
        .accept_discontinuity(1.0, 1.0, 2.0, delay, None)
        .unwrap();
    let arrival = next(&circuit, &history, 1.0, 2.0, &NoAbort)
        .unwrap()
        .unwrap();
    assert_eq!(arrival.time.to_bits(), 1.0_f64.next_up().to_bits());
    let width = arrival
        .limit_step(&circuit, 1.0, 0.1, 1.1, Value::EPSILON)
        .unwrap()
        .unwrap();
    assert_eq!(width, Value::EPSILON);
    assert!(
        arrival
            .ensure_reachable(&circuit, 1.0, Value::EPSILON.next_up())
            .is_err()
    );
    // At a large absolute clock, addition can reach the event even though
    // the integration width is less than its subtracted clock gap.
    let arrival = PhaseArrival {
        order: DelayEventOrder::Unknown,
        time: 1e12_f64.next_up(),
        device_index: 0,
    };
    let width = (arrival.time - 1e12) * 0.75;
    assert_eq!(1e12 + width, arrival.time);
    assert_eq!(
        arrival
            .limit_step(&circuit, 1e12, width, 1e12 + width, width)
            .unwrap(),
        Some(width)
    );
    let clock =
        canonical_transient_step_time_with_device_event(1e12, width, 2e12, Some(arrival.time));
    validate_clock(true, clock, arrival.time).unwrap();
    assert!(validate_clock(true, clock, clock.next_up()).is_err());
    validate_clock(false, clock, clock.next_up()).unwrap();
}

#[test]
fn gp_phase_arrival_preserves_an_exact_target_when_addition_misses_it() {
    let (circuit, _) = fixture();
    // The fixed binary64 oracle misses the endpoint by one ULP without
    // retaining the absolute target independently from its integration width.
    let accepted = Value::from_bits(0x3e02_db21_7f74_4098);
    let target = Value::from_bits(0x3e42_ffcc_ca47_13bf);
    assert_ne!(accepted + (target - accepted), target);
    let arrival = PhaseArrival {
        order: DelayEventOrder::Unknown,
        time: target,
        device_index: 0,
    };
    let width = arrival
        .limit_step(&circuit, accepted, target - accepted, target, 1e-10)
        .unwrap()
        .unwrap();
    let clock = canonical_transient_step_time_with_device_event(accepted, width, 2.0, Some(target));
    assert_eq!(clock.to_bits(), target.to_bits());
    validate_clock(true, clock, target).unwrap();
}

#[test]
fn gp_phase_arrival_interval_fit_leaves_a_reachable_final_gap() {
    let (circuit, history) = fixture();
    let arrival = next(&circuit, &history, 0.5, 20.0, &NoAbort)
        .unwrap()
        .unwrap();
    let minimum = 0.1;
    let proposed = arrival.time - 0.5 - 0.001;
    assert!(arrival.time - (0.5 + proposed) < minimum);
    let fitted =
        breakpoints::fit_model_interval(0.5, arrival.time, proposed, minimum, 10.0, 10.0, false)
            .unwrap();
    assert!(fitted < proposed);
    arrival
        .ensure_reachable(&circuit, 0.5 + fitted, minimum)
        .unwrap();
    assert_eq!(
        arrival
            .limit_step(&circuit, 0.5, fitted, 0.5 + fitted, minimum)
            .unwrap(),
        None
    );
    let remaining = arrival.time - (0.5 + fitted);
    assert_eq!(
        arrival
            .limit_step(&circuit, 0.5 + fitted, remaining, arrival.time, minimum)
            .unwrap(),
        Some(remaining)
    );
}

#[test]
fn gp_phase_arrival_groups_only_one_representable_clock_interval() {
    let deck = Netlist::parse(
        "grouped phase\nQ1 c b 0 m\nRc c 0 1k\nRb b 0 1k\n.model m NPN(TF=1e12 PTF=180)\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let circuit = engine.build_circuit(&deck).unwrap();
    let mut history = Engine::initialize_bjt_history(
        &circuit,
        &vec![0.0; circuit.matrix_size()],
        ReactiveHistorySeed::SolvedBias,
    );
    Engine::initialize_bjt_phase_history(&circuit, &mut history).unwrap();
    let delay = circuit.bjts.devices[0].legacy_excess_phase_delay();
    for time in [1.0_f64.next_up(), 1.0_f64.next_up().next_up()] {
        history.phase[0]
            .as_mut()
            .unwrap()
            .accept_discontinuity(time, 1.0, 2.0, delay, None)
            .unwrap();
    }
    advance(&circuit, &mut history, 2.0);
    let arrival = next(&circuit, &history, 2.0, 2.0 * delay, &NoAbort)
        .unwrap()
        .unwrap();
    assert_eq!(arrival.time, (delay + 1.0).next_up());
    advance(&circuit, &mut history, arrival.time);
    assert_eq!(
        history.phase[0]
            .as_ref()
            .unwrap()
            .accepted_left_limits()
            .len(),
        2
    );
    assert_eq!(
        next(&circuit, &history, arrival.time, 2.0 * delay, &NoAbort).unwrap(),
        None
    );
}
