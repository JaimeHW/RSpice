use super::*;
use crate::device::TransmissionLineHistoryEvent;

#[test]
fn line_event_circuit_solves_both_wave_limits_and_finite_rates() {
    let mut circuit =
        build("line event\nR1 near 0 50\nR2 far 0 50\nT1 near 0 far 0 Z0=50 TD=1\n.end\n");
    let near = circuit.get_node_by_name("near").unwrap() - 1;
    let far = circuit.get_node_by_name("far").unwrap() - 1;
    let line = &mut circuit.tlines[0];
    line.update_history(0.0, 0.0, 0.0, 0.0, 0.0);
    line.accept_history_event(TransmissionLineHistoryEvent {
        time: 0.25,
        incoming: [0.5, 0.0, -1.0, 0.0],
        outgoing: [1.5, 0.0, -2.0, 0.0],
        incoming_wave_slopes: [2.0, -4.0],
        outgoing_wave_slopes: [6.0, -8.0],
    })
    .unwrap();
    let retained = line.checkpoint_state().unwrap();
    let options = options();
    let mut sampler = PreparedEventCircuit::new(&circuit, 1e-20, &options, &NoAbort).unwrap();
    let zeros = vec![0.0; circuit.matrix_size()];
    for (side, near_v, far_v, near_rate, far_rate) in [
        (SourceTimeSide::LeftLimit, -0.5, 0.25, -2.0, 1.0),
        (SourceTimeSide::RightLimit, -1.0, 0.75, -4.0, 3.0),
    ] {
        let topology = sampler.topology(1.25, side, &options, &NoAbort).unwrap();
        let state = topology
            .solve(&zeros, &zeros, &options, &NoAbort, |state, abort| {
                sampler.sample(1.25, side, state, &[], &options, abort)
            })
            .unwrap();
        close(state.solution[near], near_v, 1e-14);
        close(state.solution[far], far_v, 1e-14);
        close(state.coordinate_rates[near].unwrap(), near_rate, 1e-14);
        close(state.coordinate_rates[far].unwrap(), far_rate, 1e-14);
        assert!(state.source_impulses.is_empty());
        assert_eq!(circuit.tlines[0].checkpoint_state().unwrap(), retained);
    }
    assert!(
        sampler
            .sample(
                1.5,
                SourceTimeSide::RightLimit,
                &zeros,
                &[],
                &options,
                &NoAbort
            )
            .err()
            .unwrap()
            .to_string()
            .contains("accepted delay history")
    );
}
