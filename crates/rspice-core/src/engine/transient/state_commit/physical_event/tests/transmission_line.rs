use super::*;
use crate::device::TransmissionLineTimeSide;

#[test]
fn line_event_acceptance_is_atomic_and_retains_physical_slopes() {
    let (engine, mut circuit, mut matrix, incoming, mut history) = fixture(
        "physical line\nV1 s 0 PWL(0 0 1 0 1 2 2 4)\nR1 s near 50\nT1 near 0 far 0 Z0=50 TD=4\nR2 far 0 50\n.end\n",
    );
    Engine::initialize_tline_history(&mut circuit, &incoming, 0.0);
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 0.001,
                phase_events: PhysicalEventOrders::Declared(&[]),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    assert!(
        circuit.tlines[0]
            .checkpoint_state()
            .unwrap()
            .events
            .is_empty()
    );
    close(point.lines[0].sample.outgoing[0], 1.0, 1e-14);
    close(point.lines[0].sample.outgoing[1], 0.02, 1e-14);
    close(point.lines[0].sample.outgoing_wave_slopes[0], 2.0, 1e-14);
    assert_eq!(point.lines[0].sample.incoming_wave_slopes, [0.0; 2]);
    let saved = circuit.tlines[0].checkpoint_state().unwrap();
    // Even restoring identical numeric history invalidates stale preparation.
    circuit.tlines[0].restore_checkpoint_state(&saved).unwrap();
    let mut solution = point.state().solution.clone();
    assert!(
        accept(
            &engine,
            &mut circuit,
            &mut matrix,
            &mut history,
            &point,
            &mut solution
        )
        .err()
        .unwrap()
        .to_string()
        .contains("history changed")
    );
    assert_eq!(circuit.tlines[0].checkpoint_state().unwrap(), saved);
    let point = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                incoming: &incoming,
                time: 1.0,
                dt: 0.001,
                phase_events: PhysicalEventOrders::Declared(&[]),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    accept(
        &engine,
        &mut circuit,
        &mut matrix,
        &mut history,
        &point,
        &mut solution,
    )
    .unwrap();
    let line = &circuit.tlines[0];
    assert_eq!(
        line.next_history_event_arrival_after(1.0).unwrap(),
        Some(5.0)
    );
    close(
        line.lossless_wave_on_side(5.0, true, TransmissionLineTimeSide::Incoming),
        0.0,
        1e-14,
    );
    close(
        line.lossless_wave_on_side(5.0, true, TransmissionLineTimeSide::Outgoing),
        2.0,
        1e-14,
    );
    close(
        line.lossless_wave_slope_on_side(5.0, true, TransmissionLineTimeSide::Outgoing)
            .unwrap(),
        2.0,
        1e-14,
    );
}

#[test]
fn line_event_startup_keeps_selected_dc_prehistory() {
    let (engine, mut circuit, _, mut solution, mut history) = fixture(
        "line startup\nV1 s 0 DC 0 PWL(0 2 1 4)\nR1 s near 50\nT1 near 0 far 0 Z0=50 TD=4\nR2 far 0 50\n.end\n",
    );
    Engine::initialize_tline_history(&mut circuit, &solution, 0.0);
    engine
        .transition_physical_startup(
            &mut circuit,
            &mut solution,
            &mut history,
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
    let line = &circuit.tlines[0];
    let saved = line.checkpoint_state().unwrap();
    assert_eq!(saved.state_history.len(), 1);
    assert_eq!(saved.events.len(), 1);
    close(
        line.lossless_wave_on_side(4.0, true, TransmissionLineTimeSide::Incoming),
        0.0,
        1e-14,
    );
    close(
        line.lossless_wave_on_side(4.0, true, TransmissionLineTimeSide::Outgoing),
        2.0,
        1e-14,
    );
    close(
        line.lossless_wave_slope_on_side(4.0, true, TransmissionLineTimeSide::Outgoing)
            .unwrap(),
        2.0,
        1e-14,
    );
}
