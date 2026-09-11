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
    assert!(bank.clocks.iter().any(|clock| *clock == (101, physical)));
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
