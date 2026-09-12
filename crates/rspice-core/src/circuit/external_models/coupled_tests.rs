use crate::xspice::event_scheduler::{EventTarget, SchedulerLimits, TimeResolution};
use crate::xspice::verilog::host::{
    DigitalActiveExchange, DigitalActiveParticipant, DigitalHost, DigitalRunError,
};
use crate::xspice::verilog::store::DigitalBitConnection;
use crate::xspice::verilog::store::{DigitalBitChange, ExternalBitDriverId};
use crate::xspice::{DigitalState, DigitalStrength, DigitalValue, PortConnection, XspiceInstance};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use std::collections::VecDeque;
use std::sync::Arc;

fn host(source: &str, nets: &[&str]) -> DigitalHost {
    let artifact =
        rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default())
            .compile_canonical_ir_module(source, None)
            .unwrap();
    let resolution = TimeResolution::new(artifact.digital.timing.precision_exponent).unwrap();
    let mut host = DigitalHost::from_plan(
        Arc::new(artifact.digital),
        resolution,
        SchedulerLimits::default(),
    );
    let groups = nets
        .iter()
        .map(|name| {
            vec![DigitalBitConnection {
                signal: host.signal(name).unwrap(),
                bit: 0,
            }]
        })
        .collect::<Vec<_>>();
    host.connect_bits(&groups).unwrap();
    host
}
fn target(net: usize, name: &str) -> EventTarget {
    EventTarget {
        node_id: net,
        instance: name.into(),
        port_name: "out".into(),
        driver_index: 0,
    }
}
fn bit(host: &DigitalHost, name: &str) -> String {
    host.read(host.signal(name).unwrap()).unwrap().spelling()
}

/// A real XSPICE inverter, with an input observation view and its original
/// output contribution. This fixture has one gate and no internal feedback;
/// it uses the same single-wave entry point as the production XSPICE loop.
#[derive(Clone)]
struct Inverter {
    circuit: crate::CircuitData,
    wave: Option<super::XspiceActiveWave>,
    driver: ExternalBitDriverId,
    pending: VecDeque<DigitalValue>,
    last: Option<DigitalValue>,
    transitions: Vec<DigitalValue>,
    fail_after_drive: bool,
}
impl Inverter {
    fn new(driver: ExternalBitDriverId) -> Self {
        let mut circuit = crate::CircuitData::new();
        circuit.get_or_create_node("in");
        circuit.get_or_create_node("out");
        let mut instance = XspiceInstance::new(
            "Ainv",
            Arc::new(crate::xspice::models::DigitalInverter),
            vec![PortConnection::Digital(1), PortConnection::Digital(2)],
            &[("rise_delay".into(), 0.0), ("fall_delay".into(), 0.0)],
            &[],
            &[],
            &[],
        )
        .unwrap();
        instance.init().unwrap();
        circuit.add_xspice_instance(instance);
        Self {
            circuit,
            wave: None,
            driver,
            pending: VecDeque::new(),
            last: None,
            transitions: Vec::new(),
            fail_after_drive: false,
        }
    }
}
impl DigitalActiveParticipant for Inverter {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        for change in exchange.take_changes() {
            if change.net == 0 {
                self.pending.push_back(change.value);
            }
        }
        if self.wave.is_none() {
            self.wave=Some(self.circuit.begin_xspice_active_wave(
                exchange.physical_seconds(),0.0,crate::xspice::AnalysisType::Transient,
                crate::xspice::EvaluationPhase::DirectEvaluation,
                super::XspiceCompanionPolicy {
                    coefficients:&crate::numerics::integration::CompanionCoefficients::backward_euler(),
                    xyce_one_step_order2:false,
                }).map_err(|error|DigitalRunError::ExternalExecution {detail:error.to_string()})?);
        }
        let wave = self.wave.as_mut().unwrap();
        if let Some(input) = self.pending.pop_front() {
            self.circuit
                .observe_xspice_shared_digital_inputs(wave, &[(1, input)]);
        }
        let more = self
            .circuit
            .step_xspice_active_wave(wave, &[], None)
            .map_err(|error| DigitalRunError::ExternalExecution {
                detail: error.to_string(),
            })?;
        let value = self
            .circuit
            .xspice_event_values
            .digital_drivers
            .get(&2)
            .and_then(|drivers| drivers.get(&("Ainv".into(), "out".into(), 0)))
            .copied()
            .expect("inverter scheduled its original output contribution");
        if self.last != Some(value) {
            exchange.drive_many(&[(self.driver, value)])?;
            self.last = Some(value);
            self.transitions.push(value);
            if self.fail_after_drive {
                return Err(DigitalRunError::ExternalExecution {
                    detail: "injected participant failure".into(),
                });
            }
        }
        Ok(more || !self.pending.is_empty())
    }
}

#[test]
fn coupled_active_xspice_returns_before_inactive_and_nba_then_replays() {
    let mut digital = host(
        r#"
module regions;
 reg a,sampled,observed; wire command,response,trigger; assign command=a;
 initial begin a=0; sampled=1; observed=0; end
 always @(posedge trigger) begin a=1; #0 sampled=response; a<=0; end
 always @(negedge response) if(a) observed<=a;
endmodule
"#,
        &["command", "response"],
    );
    let ids = digital
        .attach_external_bits(&[0, 1], &[(1, target(1, "Ainv"))])
        .unwrap();
    let mut inverter = Inverter::new(ids[0]);
    digital.prepare_start().unwrap();
    digital.advance_to_with(0, &mut inverter).unwrap();
    assert_eq!(bit(&digital, "response"), "1");
    let accepted = (digital.clone(), inverter.clone());
    // The ngspice-compatible gate enforces at least 1 ps propagation after
    // startup. At physical time zero its initialization response is zero-delay,
    // so this checks actual gate behavior without bypassing that delay policy.
    let trigger = [(
        digital.signal("trigger").unwrap(),
        FourStateValue::splat(1, rspice_veriloga::four_state::FourStateBit::One),
    )];
    inverter.fail_after_drive = true;
    assert!(
        digital
            .force_many_from_analog_with(&trigger, 0, 0.0, &mut inverter)
            .unwrap_err()
            .to_string()
            .contains("injected participant failure")
    );
    (digital, inverter) = accepted.clone();
    digital
        .force_many_from_analog_with(&trigger, 0, 0.0, &mut inverter)
        .unwrap();
    assert_eq!(
        bit(&digital, "sampled"),
        "0",
        "inactive code must see the XSPICE response"
    );
    assert_eq!(
        bit(&digital, "observed"),
        "1",
        "the response edge captures a before its NBA reset"
    );
    assert_eq!(bit(&digital, "a"), "0");
    assert_eq!(bit(&digital, "response"), "1");
    assert_eq!(
        inverter
            .transitions
            .iter()
            .map(|v| v.state)
            .collect::<Vec<_>>(),
        vec![DigitalState::One, DigitalState::Zero, DigitalState::One]
    );
    let (mut independent, mut participant) = accepted;
    independent
        .force_many_from_analog_with(&trigger, 0, 0.0, &mut participant)
        .unwrap();
    assert_eq!(participant.transitions, inverter.transitions);
    assert_eq!(bit(&independent, "observed"), bit(&digital, "observed"));
}

#[derive(Default)]
struct Bank {
    next: Vec<(ExternalBitDriverId, DigitalValue)>,
    changes: Vec<DigitalBitChange>,
    clocks: Vec<(u64, f64)>,
}
impl DigitalActiveParticipant for Bank {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        self.changes.extend(exchange.take_changes());
        self.clocks
            .push((exchange.tick(), exchange.physical_seconds()));
        let drives = std::mem::take(&mut self.next);
        exchange.drive_many(&drives)?;
        Ok(!drives.is_empty())
    }
}

#[test]
fn coupled_active_driver_strengths_release_and_fresh_state() {
    let mut digital = host(
        r#"
module drivers; reg enabled; wire bus; assign bus=enabled ? 1'b0 : 1'bz;
 initial begin enabled=1; #1 enabled=0; end endmodule
"#,
        &["bus"],
    );
    assert!(
        digital
            .attach_external_bits(&[9], &[(0, target(0, "Apull"))])
            .is_err()
    );
    let ids = digital
        .attach_external_bits(&[0], &[(0, target(0, "Apull")), (0, target(0, "Adrive"))])
        .unwrap();
    let mut bank = Bank {
        next: vec![(
            ids[0],
            DigitalValue::new(DigitalState::One, DigitalStrength::Resistive),
        )],
        ..Default::default()
    };
    digital.prepare_start().unwrap();
    digital.advance_to_with(0, &mut bank).unwrap();
    assert_eq!(
        bit(&digital, "bus"),
        "0",
        "strong HDL low overrides an external weak pull-up"
    );
    bank.next = vec![(ids[1], DigitalValue::one())];
    digital.settle_with(0, &mut bank).unwrap();
    assert_eq!(
        bit(&digital, "bus"),
        "x",
        "independent strong contributors contend"
    );
    digital.advance_to_with(1, &mut bank).unwrap();
    assert_eq!(bit(&digital, "bus"), "1");
    bank.next = vec![(ids[1], DigitalValue::high_z())];
    digital.settle_with(1, &mut bank).unwrap();
    assert_eq!(bit(&digital, "bus"), "1");
    assert_eq!(
        bank.changes.last().unwrap().value.strength,
        DigitalStrength::Resistive
    );
    assert!(
        bank.changes
            .iter()
            .any(|change| change.previous.strength == DigitalStrength::Strong
                && change.value.strength == DigitalStrength::Resistive),
        "strength-only changes remain visible to event participants"
    );
    bank.next = vec![(ids[0], DigitalValue::high_z())];
    digital.settle_with(1, &mut bank).unwrap();
    assert_eq!(bit(&digital, "bus"), "z");
    let mut fresh = digital.fresh();
    assert_eq!(bit(&fresh, "bus"), "z");
    let mut fresh_bank = Bank {
        next: vec![(ids[1], DigitalValue::one())],
        ..Default::default()
    };
    let enabled = fresh.signal("enabled").unwrap();
    assert!(
        fresh
            .force(
                enabled,
                FourStateValue::splat(1, rspice_veriloga::four_state::FourStateBit::One),
                0
            )
            .is_err()
    );
    assert_eq!(
        bit(&fresh, "enabled"),
        "x",
        "a missing participant is refused before any input write"
    );
    fresh.prepare_start().unwrap();
    fresh.advance_to_with(0, &mut fresh_bank).unwrap();
    assert_eq!(bit(&fresh, "bus"), "x");
}

#[test]
fn coupled_active_off_grid_work_does_not_consume_a_future_timer() {
    let mut digital = host(
        r#"
`timescale 1ns/1ps
module causal; wire edge_in; reg immediate,delayed,unrelated_timer;
 initial begin immediate=0; delayed=0; unrelated_timer=0; #0.101 unrelated_timer=1; end
 always @(posedge edge_in) begin immediate=1; delayed<=#0.025 1; end
endmodule
"#,
        &["edge_in"],
    );
    let ids = digital
        .attach_external_bits(&[0], &[(0, target(0, "Aedge"))])
        .unwrap();
    let mut bank = Bank {
        next: vec![(ids[0], DigitalValue::zero())],
        ..Default::default()
    };
    digital.prepare_start().unwrap();
    digital.advance_to_with(0, &mut bank).unwrap();
    bank.next = vec![(ids[0], DigitalValue::one())];
    let physical = 100.6e-12;
    digital
        .force_many_from_analog_with(&[], 101, physical, &mut bank)
        .unwrap();
    assert_eq!(bit(&digital, "immediate"), "1");
    assert_eq!(bit(&digital, "unrelated_timer"), "0");
    assert_eq!(bit(&digital, "delayed"), "0");
    assert!(bank.clocks.contains(&(101, physical)));
    digital.advance_to_with(101, &mut bank).unwrap();
    assert_eq!(bit(&digital, "unrelated_timer"), "1");
    assert_eq!(bit(&digital, "delayed"), "0");
    digital.advance_to_with(126, &mut bank).unwrap();
    assert_eq!(bit(&digital, "delayed"), "1");
}

#[test]
fn coupled_active_driver_bank_is_atomic_for_expression_waits_and_observers() {
    let mut digital = host(
        r#"
module bank; wire a,b; reg [31:0] glitches; initial glitches=0;
 always @(posedge(a^b)) glitches=glitches+1; endmodule
"#,
        &["a", "b"],
    );
    let ids = digital
        .attach_external_bits(&[0, 1], &[(0, target(0, "Aword")), (1, target(1, "Aword"))])
        .unwrap();
    let mut bank = Bank {
        next: vec![
            (ids[0], DigitalValue::zero()),
            (ids[1], DigitalValue::zero()),
        ],
        ..Default::default()
    };
    digital.prepare_start().unwrap();
    digital.advance_to_with(0, &mut bank).unwrap();
    bank.changes.clear();
    bank.next = vec![(ids[0], DigitalValue::one()), (ids[1], DigitalValue::one())];
    digital.settle_with(0, &mut bank).unwrap();
    assert!(bit(&digital, "glitches").chars().all(|bit| bit == '0'));
    assert_eq!(bank.changes.len(), 2);
    assert!(bank.changes[0].starts_publication);
    assert!(!bank.changes[1].starts_publication);
    assert!(
        digital
            .attach_external_bits(&[0], &[])
            .unwrap_err()
            .to_string()
            .contains("after digital execution")
    );
}

fn routed_inverters() -> crate::CircuitData {
    let mut circuit = crate::CircuitData::new();
    for name in ["command", "bus", "response"] {
        circuit.get_or_create_node(name);
    }
    for (name, input, output) in [("Ainv", 1, 2), ("Aobserver", 2, 3)] {
        let mut instance = XspiceInstance::new(
            name,
            Arc::new(crate::xspice::models::DigitalInverter),
            vec![
                PortConnection::Digital(input),
                PortConnection::Digital(output),
            ],
            &[("rise_delay".into(), 0.0), ("fall_delay".into(), 0.0)],
            &[],
            &[],
            &[],
        )
        .unwrap();
        instance.init().unwrap();
        circuit.add_xspice_instance(instance);
    }
    circuit
}

fn routed_participant<'a>(
    circuit: &'a mut crate::CircuitData,
    bindings: &'a super::XspiceDigitalBindings,
    time: f64,
    timestep: f64,
) -> super::XspiceDigitalParticipant<'a> {
    super::XspiceDigitalParticipant::new(
        circuit,
        bindings,
        &[],
        time,
        timestep,
        crate::xspice::AnalysisType::Transient,
        crate::xspice::EvaluationPhase::DirectEvaluation,
        super::XspiceCompanionPolicy {
            coefficients: &crate::numerics::integration::CompanionCoefficients::backward_euler(),
            xyce_one_step_order2: false,
        },
        None,
    )
}

#[test]
fn coupled_routed_xspice_fanout_reads_combined_contention_and_release() {
    let mut digital = host(
        r#"
module routed;
 reg own,sampled; wire command,bus,response,trigger;
 assign command=0; assign bus=own ? 1'b0 : 1'bz;
 initial begin own=1; sampled=1; end
 always @(posedge trigger) begin own=0; #0 sampled=response; end
endmodule
"#,
        &["command", "bus", "response"],
    );
    let mut circuit = routed_inverters();
    let bindings =
        super::XspiceDigitalBindings::enroll(&circuit, &mut digital, &[(1, 0), (2, 1), (3, 2)])
            .unwrap()
            .unwrap();
    digital.prepare_start().unwrap();
    digital
        .advance_to_with(
            0,
            &mut routed_participant(&mut circuit, &bindings, 0.0, 0.0),
        )
        .unwrap();
    assert_eq!(bit(&digital, "bus"), "x");
    assert_eq!(
        bit(&digital, "response"),
        "x",
        "native XSPICE fanout must see HDL contention"
    );
    assert_eq!(
        circuit.xspice_event_values.digital_values[&2].state,
        DigitalState::Unknown
    );
    let original = &circuit.xspice_event_values.digital_drivers[&2];
    assert_eq!(
        original.len(),
        1,
        "a resolved HDL observation must not become an XSPICE driver"
    );
    assert_eq!(
        original[&("Ainv".into(), "out".into(), 0)].state,
        DigitalState::One
    );
    let accepted = (digital.clone(), circuit.clone());
    let trigger = [(
        digital.signal("trigger").unwrap(),
        FourStateValue::splat(1, rspice_veriloga::four_state::FourStateBit::One),
    )];
    digital
        .force_many_from_analog_with(
            &trigger,
            0,
            0.0,
            &mut routed_participant(&mut circuit, &bindings, 0.0, 0.0),
        )
        .unwrap();
    assert_eq!(bit(&digital, "bus"), "1");
    assert_eq!(bit(&digital, "response"), "0");
    assert_eq!(
        bit(&digital, "sampled"),
        "0",
        "XSPICE fanout settles before inactive reads"
    );
    assert_eq!(
        circuit.xspice_event_values.digital_values[&2],
        DigitalValue::one()
    );
    let (mut retry, mut retried_circuit) = accepted;
    retry
        .force_many_from_analog_with(
            &trigger,
            0,
            0.0,
            &mut routed_participant(&mut retried_circuit, &bindings, 0.0, 0.0),
        )
        .unwrap();
    assert_eq!(bit(&retry, "sampled"), bit(&digital, "sampled"));
    assert_eq!(
        retried_circuit.xspice_event_values.digital_values,
        circuit.xspice_event_values.digital_values
    );
}

#[test]
fn coupled_routed_delayed_xspice_event_keeps_physical_time_and_future_hdl_timer() {
    let mut digital = host(
        r#"
`timescale 1ns/1ps
module delayed;
 reg a,unrelated,future,captured; wire command,bus,response,trigger;
 assign command=a;
 initial begin a=0; unrelated=0; future=0; captured=1; #0.101 unrelated=1; #0.001 future=1; end
 always @(posedge trigger) a=1;
 always @(negedge bus) captured=future;
endmodule
"#,
        &["command", "bus", "response"],
    );
    let mut circuit = routed_inverters();
    let bindings =
        super::XspiceDigitalBindings::enroll(&circuit, &mut digital, &[(1, 0), (2, 1), (3, 2)])
            .unwrap()
            .unwrap();
    digital.prepare_start().unwrap();
    digital
        .advance_to_with(
            0,
            &mut routed_participant(&mut circuit, &bindings, 0.0, 0.0),
        )
        .unwrap();
    assert_eq!(bit(&digital, "bus"), "1");
    let physical = 100.6e-12;
    let trigger = [(
        digital.signal("trigger").unwrap(),
        FourStateValue::splat(1, rspice_veriloga::four_state::FourStateBit::One),
    )];
    digital
        .force_many_from_analog_with(
            &trigger,
            101,
            physical,
            &mut routed_participant(&mut circuit, &bindings, physical, physical),
        )
        .unwrap();
    let due = circuit.xspice_event_queue.next_event_time().unwrap();
    assert!((due - (physical + 1e-12)).abs() < 1e-25);
    assert_eq!(bit(&digital, "unrelated"), "0");
    let grid_time = TimeResolution::new(-12)
        .unwrap()
        .ticks_to_seconds(101)
        .unwrap();
    digital
        .advance_to_with(
            101,
            &mut routed_participant(&mut circuit, &bindings, grid_time, grid_time - physical),
        )
        .unwrap();
    assert_eq!(bit(&digital, "unrelated"), "1");
    assert_eq!(bit(&digital, "bus"), "1");
    let accepted = (digital.clone(), circuit.clone());
    let (mut missed, mut missed_circuit) = accepted.clone();
    let late = due + 0.1e-12;
    let error = missed
        .force_many_from_analog_with(
            &[],
            102,
            late,
            &mut routed_participant(&mut missed_circuit, &bindings, late, late - grid_time),
        )
        .unwrap_err();
    assert!(error.to_string().contains("missed XSPICE breakpoint"));
    digital
        .force_many_from_analog_with(
            &[],
            102,
            due,
            &mut routed_participant(&mut circuit, &bindings, due, due - grid_time),
        )
        .unwrap();
    assert_eq!(bit(&digital, "bus"), "0");
    assert_eq!(bit(&digital, "captured"), "0");
    assert_eq!(
        bit(&digital, "future"),
        "0",
        "external event must not consume the 102 ps timer"
    );
    assert_eq!(circuit.xspice_event_values.digital_event_times[&2], due);
    let (mut retry, mut retried_circuit) = accepted;
    retry
        .force_many_from_analog_with(
            &[],
            102,
            due,
            &mut routed_participant(&mut retried_circuit, &bindings, due, due - grid_time),
        )
        .unwrap();
    assert_eq!(bit(&retry, "captured"), "0");
    assert_eq!(
        retried_circuit.xspice_event_queue.next_event_time(),
        circuit.xspice_event_queue.next_event_time()
    );
}

/// A coupled circuit with one code-model event queued a picosecond after the
/// accepted analog time, and nothing else pending, as
/// `(host, circuit, bindings, accepted, due)`.
///
/// The interval is the gate's arithmetic rather than a choice:
/// `rise_delay`/`fall_delay` are clamped to ngspice's 1 ps minimum, so it is
/// the finest one this path can produce, and a floor either side of it is what
/// puts the same queued event on either side of the reachability line.
fn a_pending_coupled_event() -> (
    DigitalHost,
    crate::CircuitData,
    super::XspiceDigitalBindings,
    f64,
    f64,
) {
    let mut digital = host(
        r#"
`timescale 1ns/1ps
module armed;
 reg a, captured; wire command, bus, response, trigger;
 assign command = a;
 initial begin a = 0; captured = 1; end
 always @(posedge trigger) a = 1;
 always @(negedge bus) captured = 0;
endmodule
"#,
        &["command", "bus", "response"],
    );
    let mut circuit = routed_inverters();
    let bindings =
        super::XspiceDigitalBindings::enroll(&circuit, &mut digital, &[(1, 0), (2, 1), (3, 2)])
            .unwrap()
            .unwrap();
    digital.prepare_start().unwrap();
    digital
        .advance_to_with(
            0,
            &mut routed_participant(&mut circuit, &bindings, 0.0, 0.0),
        )
        .unwrap();
    let accepted = 1.0e-9;
    let trigger = [(
        digital.signal("trigger").unwrap(),
        FourStateValue::splat(1, rspice_veriloga::four_state::FourStateBit::One),
    )];
    digital
        .force_many_from_analog_with(
            &trigger,
            1000,
            accepted,
            &mut routed_participant(&mut circuit, &bindings, accepted, accepted),
        )
        .unwrap();
    let due = circuit
        .xspice_event_queue
        .next_event_time()
        .expect("the gate queued its clamped 1 ps output");
    (digital, circuit, bindings, accepted, due)
}

/// The missed-breakpoint refusal is kept for a code-model event the stepper
/// had a legal interval to, and only for that one.
///
/// Both sides of the line, from the participant's own interface, because only
/// one of them is reachable from a deck: a coupled code model's events are
/// landing targets the stepper owns, and the finest interval one can ask for —
/// a 1 ps gate delay — is a tenth of the minimum step a deck that gets
/// anywhere near this guard leaves the solver. See
/// `XspiceDigitalParticipant::event_was_reachable`. The mixed half states the
/// same rule against the same floor, and pins it the same way, in
/// `xspice::verilog::mixed`'s
/// `only_an_activation_with_an_interval_to_it_is_a_missed_breakpoint`.
#[test]
fn only_a_code_model_event_with_an_interval_to_it_is_a_missed_breakpoint() {
    let (mut digital, mut circuit, bindings, accepted, due) = a_pending_coupled_event();
    let interval = due - accepted;
    assert!(
        interval > 0.0,
        "the queued event must be ahead of the accepted point, {due:e}s against {accepted:e}s"
    );
    let past = due + 0.1e-12;

    circuit.set_mixed_analog_step_floor(interval * 0.5);
    let error = digital
        .force_many_from_analog_with(
            &[],
            1002,
            past,
            &mut routed_participant(&mut circuit, &bindings, past, past - accepted),
        )
        .expect_err("Active work past a reachable code-model event is a lost breakpoint");
    assert!(
        error.to_string().contains("missed XSPICE breakpoint"),
        "the refusal must name the breakpoint that was stepped over, got {error}"
    );

    let (mut digital, mut circuit, bindings, accepted, due) = a_pending_coupled_event();
    let past = due + 0.1e-12;
    circuit.set_mixed_analog_step_floor((due - accepted) * 2.0);
    digital
        .force_many_from_analog_with(
            &[],
            1002,
            past,
            &mut routed_participant(&mut circuit, &bindings, past, past - accepted),
        )
        .expect("an event no analog step can reach is delivered here, not refused");
    assert_eq!(
        circuit.xspice_event_values.digital_event_times[&2], past,
        "the coalesced event is dated at the timepoint the stepper landed on — the shared \
         net's resolved value is observed inside this wave — rather than at the sub-floor \
         instant the gate asked for, which no analog step reached"
    );
    assert_eq!(
        bit(&digital, "bus"),
        "0",
        "and it is delivered rather than left in the queue: the HDL half reads what the code \
         model published"
    );
}

struct RoutedVector {
    ports: Vec<crate::xspice::PortSpec>,
}
impl crate::xspice::CodeModel for RoutedVector {
    fn name(&self) -> &str {
        "routed_vector"
    }
    fn ports(&self) -> &[crate::xspice::PortSpec] {
        &self.ports
    }
    fn parameters(&self) -> &[crate::xspice::ParamSpec] {
        &[]
    }
    fn init(&self, _: &mut crate::xspice::CmContext) -> crate::xspice::CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, ctx: &mut crate::xspice::CmContext) -> crate::xspice::CmResult<()> {
        let release = ctx
            .input_digital("in")
            .expect("declared digital input")
            .state
            == DigitalState::One;
        let bit = if release {
            DigitalValue::one()
        } else {
            DigitalValue::zero()
        };
        ctx.set_output_digital_vector_from_slice(
            "out",
            &[
                if release {
                    DigitalValue::high_z()
                } else {
                    DigitalValue::one()
                },
                DigitalValue::one(),
                bit,
                bit,
            ],
            0.0,
        );
        Ok(())
    }
}

#[test]
fn coupled_routed_vector_inversion_driver_release_and_atomic_publication() {
    use crate::xspice::{DigitalPortConnection, PortSpec, PortType};
    let mut digital = host(
        r#"
module vectors;
 reg release_driver; reg [31:0] glitches; wire command,bus,left,right,trigger;
 assign command=release_driver;
 initial begin release_driver=0; glitches=0; end
 always @(posedge trigger) release_driver=1;
 always @(posedge (left^right)) glitches=glitches+1;
endmodule
"#,
        &["command", "bus", "left", "right"],
    );
    let mut circuit = crate::CircuitData::new();
    for name in ["command", "bus", "left", "right"] {
        circuit.get_or_create_node(name);
    }
    let mut instance = XspiceInstance::new(
        "Avec",
        Arc::new(RoutedVector {
            ports: vec![
                PortSpec::input("in", PortType::Digital),
                PortSpec::vector_output("out", PortType::Digital),
            ],
        }),
        vec![
            PortConnection::Digital(1),
            PortConnection::DigitalVectorMapped(vec![
                DigitalPortConnection::new(2, false),
                DigitalPortConnection::new(2, true),
                DigitalPortConnection::new(3, false),
                DigitalPortConnection::new(4, false),
            ]),
        ],
        &[],
        &[],
        &[],
        &[],
    )
    .unwrap();
    instance.init().unwrap();
    circuit.add_xspice_instance(instance);
    let bindings = super::XspiceDigitalBindings::enroll(
        &circuit,
        &mut digital,
        &[(1, 0), (2, 1), (3, 2), (4, 3)],
    )
    .unwrap()
    .unwrap();
    digital.prepare_start().unwrap();
    digital
        .advance_to_with(
            0,
            &mut routed_participant(&mut circuit, &bindings, 0.0, 0.0),
        )
        .unwrap();
    assert_eq!(bit(&digital, "bus"), "x");
    assert_eq!(circuit.xspice_event_values.digital_drivers[&2].len(), 2);
    let trigger = [(
        digital.signal("trigger").unwrap(),
        FourStateValue::splat(1, rspice_veriloga::four_state::FourStateBit::One),
    )];
    digital
        .force_many_from_analog_with(
            &trigger,
            0,
            0.0,
            &mut routed_participant(&mut circuit, &bindings, 0.0, 0.0),
        )
        .unwrap();
    assert_eq!(bit(&digital, "bus"), "0");
    assert_eq!(bit(&digital, "left"), "1");
    assert_eq!(bit(&digital, "right"), "1");
    assert_eq!(
        bit(&digital, "glitches"),
        "00000000000000000000000000000000"
    );
    let drivers = &circuit.xspice_event_values.digital_drivers[&2];
    assert_eq!(
        drivers[&("Avec".into(), "out".into(), 0)],
        DigitalValue::high_z()
    );
    assert_eq!(
        drivers[&("Avec".into(), "out".into(), 1)],
        DigitalValue::zero()
    );
}
