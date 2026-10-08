use super::*;
use rspice_veriloga_runtime::transport_delay::{
    DelayCheckpoint, DelayConfiguration, MAX_DELAY_HISTORY_SAMPLES,
};

#[test]
fn physical_event_record_ceiling_keeps_instance_counts_and_accepted_state() {
    let (engine, circuit, _, incoming, mut history) = fixture(
        "physical event ceiling\nVc c 0 2\nVb b 0 .6\nQ1 c b 0 qm\nQ2 c b 0 qm\nC1 b 0 1p\n.model qm NPN(IS=1e-16 TF=1n PTF=30 CJE=1p CJC=.2p)\n.end\n",
    );
    let delay = circuit.bjts.devices[1].legacy_excess_phase_delay();
    let time = delay / 2.0;
    let value = history.phase[1]
        .as_ref()
        .unwrap()
        .accepted_samples()
        .next_back()
        .unwrap()
        .1;
    let original = history.phase[1].take();
    history.phase[1] = Some(
        DelayBuffer::from_checkpoint(DelayCheckpoint {
            configuration: Some(DelayConfiguration::Fixed { delay }),
            samples: (0..MAX_DELAY_HISTORY_SAMPLES)
                .map(|index| {
                    (
                        time * index as f64 / MAX_DELAY_HISTORY_SAMPLES as f64,
                        value,
                    )
                })
                .collect(),
            left_limits: Vec::new(),
            event_orders: Vec::new(),
        })
        .unwrap(),
    );
    let before = history.clone();
    let state_before = format!(
        "{:?}{:?}{:?}",
        circuit.bjts, circuit.capacitors, circuit.inductors
    );
    for (order, records) in [
        (DelayEventOrder::Unknown, 2),
        (DelayEventOrder::AtLeast(0), 3),
        (DelayEventOrder::AtLeast(2), 3),
    ] {
        let result = engine.prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                diode_history: &EMPTY_DIODES,
                integration_coefficients: None,
                incoming: &incoming,
                time,
                dt: time,
                phase_events: PhysicalEventOrders::Declared(&[Some(order); 2]),
            },
            &options(),
            1e-20,
            &NoAbort,
        );
        let Err(error) = result else {
            panic!("over-budget physical event was prepared");
        };
        assert!(
            matches!(error, SimulationError::DeviceResourceLimit { ref instance, source }
            if instance == &circuit.bjts.devices[1].name
            && source.resource == crate::ResourceKind::TransportHistoryRecords
            && source.requested == MAX_DELAY_HISTORY_SAMPLES + records
            && source.limit == MAX_DELAY_HISTORY_SAMPLES),
            "{error}"
        );
        assert_eq!(history, before);
        assert_eq!(
            format!(
                "{:?}{:?}{:?}",
                circuit.bjts, circuit.capacitors, circuit.inductors
            ),
            state_before
        );
    }
    history.phase[1] = original;
    let _ = engine
        .prepare_physical_event(
            &circuit,
            &history,
            PhysicalEventStep {
                diode_history: &EMPTY_DIODES,
                integration_coefficients: None,
                incoming: &incoming,
                time,
                dt: time,
                phase_events: PhysicalEventOrders::Declared(
                    &[Some(DelayEventOrder::AtLeast(2)); 2],
                ),
            },
            &options(),
            1e-20,
            &NoAbort,
        )
        .unwrap();
}
