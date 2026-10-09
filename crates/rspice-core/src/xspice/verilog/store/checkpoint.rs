//! Persistent store state at a drained event boundary. The receiving store is
//! an elaborated template: topology, observers and analog dependency bindings
//! must agree before any restored values can be installed by the host.
use super::*;
use rspice_veriloga::canonical_ir::digital_eval::{DigitalScalar, checkpoint::*};
use serde::{Deserialize, Serialize};

pub(super) fn fingerprint(value: &impl Serialize) -> Result<[u8; 32], String> {
    struct Writer(blake3::Hasher);
    impl std::io::Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.update(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Writer(blake3::Hasher::new());
    serde_json::to_writer(&mut writer, value).map_err(|error| error.to_string())?;
    Ok(*writer.0.finalize().as_bytes())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StoreCheckpoint {
    version: u32,
    plan: [u8; 32],
    analog_bindings: [u8; 32],
    trace_bindings: [u8; 32],
    sequence: u64,
    values: Vec<DigitalScalarCheckpoint>,
    contributions: Vec<Option<DigitalScalarCheckpoint>>,
    analog_samples: Vec<Option<u64>>,
    clock: Option<(u64, u64)>,
    expressions: Vec<ExpressionCheckpoint>,
    bits: Option<bindings::BitCheckpoint>,
    reals: Option<real_bindings::RealCheckpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExpressionCheckpoint {
    token: u64,
    wait: DigitalExpressionCheckpoint,
    remaining: DigitalEventCountCheckpoint,
}

impl DigitalSignalStore {
    fn analog_bindings_identity(&self) -> Result<[u8; 32], String> {
        // HashMap iteration is randomized; sort identities, preserving each
        // producer's ordered dependency list from circuit elaboration.
        let inputs = self
            .analog_variable_inputs
            .as_ref()
            .map(|inputs| inputs.iter().collect::<BTreeMap<_, _>>());
        fingerprint(&inputs)
    }

    fn check_checkpoint_boundary(&self) -> Result<(), String> {
        if self.sequence.is_none() || self.expression_error.is_some() {
            return Err("failed digital store cannot be checkpointed".into());
        }
        if !self.external_changes.is_empty()
            || self.external_batch.is_some()
            || !self.expression_captures.is_empty()
            || !self.deferred.is_empty()
            || !self.event_captures.is_empty()
            || !self.delayed.is_empty()
            || !self.transitions.is_empty()
            || self.has_traces()
        {
            return Err("digital store publications and captured updates must be drained".into());
        }
        Ok(())
    }

    pub(crate) fn checkpoint(&self) -> Result<StoreCheckpoint, String> {
        self.check_checkpoint_boundary()?;
        let values = self
            .plan
            .signals
            .iter()
            .enumerate()
            .map(|(index, signal)| {
                DigitalScalarCheckpoint::capture(&if signal.kind.is_real() {
                    DigitalScalar::Real(self.reals[index])
                } else {
                    DigitalScalar::FourState(self.values[index].clone())
                })
            })
            .collect();
        let contributions = self
            .contributions
            .iter()
            .map(|contribution| {
                let value = match &contribution.value {
                    ContributionValue::FourState(value) => value
                        .as_ref()
                        .map(|value| DigitalScalar::FourState(value.clone())),
                    ContributionValue::Real(value) => value.map(DigitalScalar::Real),
                };
                value.as_ref().map(DigitalScalarCheckpoint::capture)
            })
            .collect();
        Ok(StoreCheckpoint {
            version: 1,
            plan: self.plan.content_identity,
            analog_bindings: self.analog_bindings_identity()?,
            trace_bindings: self.traces.checkpoint_identity()?,
            sequence: self.sequence.unwrap(),
            values,
            contributions,
            analog_samples: self
                .analog_samples
                .iter()
                .map(|value| value.map(f64::to_bits))
                .collect(),
            clock: self
                .activation_clock
                .map(|clock| (clock.tick, clock.absolute_seconds.to_bits())),
            expressions: self
                .expression_waits
                .iter()
                .map(|(token, subscription)| ExpressionCheckpoint {
                    token: *token,
                    wait: DigitalExpressionCheckpoint::capture(&subscription.wait),
                    remaining: DigitalEventCountCheckpoint::capture(&subscription.remaining),
                })
                .collect(),
            bits: self
                .connected
                .as_ref()
                .map(ConnectedBits::checkpoint)
                .transpose()?,
            reals: self
                .external_reals
                .as_ref()
                .map(ExternalReals::checkpoint)
                .transpose()?,
        })
    }

    /// Construct a replacement without touching this store. The enclosing host
    /// authenticates subscription ownership and queue relationships before swap.
    pub(crate) fn restored_checkpoint(
        &self,
        image: &StoreCheckpoint,
        reader: &mut DigitalCheckpointReader<'_>,
    ) -> Result<Self, String> {
        if image.version != 1 || image.plan != self.plan.content_identity {
            return Err("digital store schema or compiled plan differs".into());
        }
        if image.analog_bindings != self.analog_bindings_identity()?
            || image.trace_bindings != self.traces.checkpoint_identity()?
        {
            return Err("digital store analog or trace bindings differ".into());
        }
        if image.values.len() != self.plan.signals.len()
            || image.contributions.len() != self.contributions.len()
            || image.analog_samples.len() != self.analog_samples.len()
        {
            return Err("digital store checkpoint dimensions differ".into());
        }
        let mut restored = Self::from_plan(Arc::clone(&self.plan));
        restored.inherit_bit_connections(self);
        restored.inherit_analog_variable_inputs(self);
        restored.traces = self.traces.clone(); // The journal was checked empty above.
        restored.connected = match (&self.connected, &image.bits) {
            (Some(bits), Some(image)) => Some(bits.restore_checkpoint(image)?),
            (None, None) => None,
            _ => return Err("digital store bit connections differ".into()),
        };
        restored.external_reals = match (&self.external_reals, &image.reals) {
            (Some(reals), Some(image)) => Some(reals.restore_checkpoint(image)?),
            (None, None) => None,
            _ => return Err("digital store real connections differ".into()),
        };
        for (index, (signal, image)) in self.plan.signals.iter().zip(&image.values).enumerate() {
            let value = reader
                .restore_scalar(image)
                .map_err(|error| error.to_string())?;
            match value {
                DigitalScalar::FourState(value)
                    if !signal.kind.is_real() && value.width() == signal.width =>
                {
                    restored.values[index] = value
                }
                DigitalScalar::Real(value) if signal.kind.is_real() => {
                    restored.reals[index] = value
                }
                _ => return Err("digital store value domain or width differs".into()),
            }
        }
        // The contribution table is grouped by declared net and driver order.
        // Rebuild locations from that same plan, never from serialized offsets.
        let mut slot = 0;
        for signal in &self.plan.signals {
            for driver in self.plan.drivers_of(signal.id) {
                if let Some(image) = &image.contributions[slot] {
                    let value = reader
                        .restore_scalar(image)
                        .map_err(|error| error.to_string())?;
                    restored.contributions[slot].value = match value {
                        DigitalScalar::Real(value) if signal.kind.is_real() => {
                            ContributionValue::Real(Some(value))
                        }
                        DigitalScalar::FourState(value)
                            if !signal.kind.is_real()
                                && value.width()
                                    == driver_width(signal, &driver.target.select)? =>
                        {
                            ContributionValue::FourState(Some(value))
                        }
                        _ => {
                            return Err(
                                "digital driver contribution domain or width differs".into()
                            );
                        }
                    };
                }
                slot += 1;
            }
        }
        restored.analog_samples = image
            .analog_samples
            .iter()
            .map(|value| value.map(f64::from_bits))
            .collect();
        restored.activation_clock = image.clock.map(|(tick, bits)| DigitalClock {
            tick,
            absolute_seconds: f64::from_bits(bits),
        });
        if restored.activation_clock.is_some_and(|clock| {
            !clock.absolute_seconds.is_finite()
                || clock.absolute_seconds < 0.0
                || clock.tick > crate::xspice::event_scheduler::TimeResolution::MAX_EXACT_TICKS
        }) {
            return Err("invalid digital activation clock".into());
        }
        restored.sequence = Some(image.sequence);
        let mut last = None;
        for subscription in &image.expressions {
            if subscription.token == 0
                || subscription.token > image.sequence
                || last.is_some_and(|previous| previous >= subscription.token)
            {
                return Err("invalid or unordered expression subscription identity".into());
            }
            let wait = reader
                .restore_expression(&subscription.wait)
                .map_err(|error| error.to_string())?;
            let remaining = reader
                .restore_count(&subscription.remaining)
                .map_err(|error| error.to_string())?;
            if remaining.is_zero() {
                return Err("completed expression subscription is still registered".into());
            }
            restored.insert_expression_wait(subscription.token, wait, remaining);
            last = Some(subscription.token);
        }
        restored.validate_checkpoint_resolution()?;
        Ok(restored)
    }

    fn validate_checkpoint_resolution(&self) -> Result<(), String> {
        let connected = self.validate_connected_checkpoint()?;
        for signal in &self.plan.signals {
            let index = usize::from(signal.id);
            if signal.procedurally_assignable {
                continue;
            }
            if signal.kind.is_real() {
                if self.real_members(&signal.id).iter().any(|member| {
                    self.reals[usize::from(*member)].to_bits() != self.reals[index].to_bits()
                }) {
                    return Err("aliased real net values differ".into());
                }
                let driven = self.real_members(&signal.id).iter().any(|member| {
                    self.spans[usize::from(*member)].count != 0
                        || self.external_real_values(*member).next().is_some()
                });
                if driven && self.resolve_real(signal.id).to_bits() != self.reals[index].to_bits() {
                    return Err("real net value differs from its driver resolution".into());
                }
            } else if self.spans[index].count != 0 {
                let expected = self.resolve(signal.id);
                for bit in 0..signal.width {
                    if !connected.contains(&DigitalBitConnection {
                        signal: signal.id,
                        bit,
                    }) && expected.bit(bit) != self.values[index].bit(bit)
                    {
                        return Err("digital net value differs from its driver resolution".into());
                    }
                }
            }
        }
        Ok(())
    }
}

fn driver_width(
    signal: &DigitalSignal,
    select: &rspice_veriloga::canonical_ir::digital::DigitalWriteSelect,
) -> Result<u32, String> {
    use rspice_veriloga::canonical_ir::digital::DigitalWriteSelect::*;
    match select {
        Whole => Ok(signal.width),
        Bit(_) => Ok(1),
        Part { msb, lsb } => msb
            .abs_diff(*lsb)
            .checked_add(1)
            .and_then(|width| u32::try_from(width).ok())
            .ok_or_else(|| "invalid compiled driver width".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::digital::{DigitalState, DigitalStrength, DigitalValue};
    use crate::xspice::event_scheduler::EventTarget;
    use rspice_veriloga::canonical_ir::digital::DigitalProcessKind;
    use rspice_veriloga::canonical_ir::digital_eval::{DigitalProcessOutcome, start};

    fn store(source: &str) -> DigitalSignalStore {
        let plan = rspice_veriloga::VerilogACompiler::default()
            .compile_canonical_ir(source)
            .unwrap()
            .digital;
        DigitalSignalStore::from_plan(Arc::new(plan))
    }
    fn signal(store: &DigitalSignalStore, name: &str) -> DigitalSignalId {
        store
            .plan
            .signals
            .iter()
            .find(|signal| signal.name == name)
            .unwrap()
            .id
    }
    fn transport(image: &StoreCheckpoint) -> StoreCheckpoint {
        serde_json::from_slice(&serde_json::to_vec(image).unwrap()).unwrap()
    }
    fn restore(
        store: &DigitalSignalStore,
        image: &StoreCheckpoint,
    ) -> Result<DigitalSignalStore, String> {
        let mut reader =
            DigitalCheckpointReader::new(&store.plan, DigitalCheckpointLimits::default()).unwrap();
        store.restored_checkpoint(image, &mut reader)
    }
    fn run_drivers(store: &mut DigitalSignalStore) {
        let plan = Arc::clone(&store.plan);
        for process in &plan.processes {
            if process.kind == DigitalProcessKind::ContinuousAssign {
                start(&plan, process, store).unwrap();
            }
        }
    }
    fn drain(store: &mut DigitalSignalStore) {
        store.take_transitions();
        store.external_changes.clear();
        store.drain_traces(&mut Vec::new());
    }

    #[test]
    fn store_checkpoint_preserves_drivers_values_and_expression_baselines() {
        let mut original = store(
            r#"
module stored;
reg [3:0] a,b; wire [3:0] bus; real left,right,state; wrealsum total;
assign bus=a; assign bus=b; assign total=left; assign total=right;
initial begin a=4'bz01x; b=4'b1z1z; left=2; right=3; state=0; @(a ^ b) a=0; end
endmodule
"#,
        );
        let plan = Arc::clone(&original.plan);
        let process = plan
            .processes
            .iter()
            .find(|p| p.kind == DigitalProcessKind::Initial)
            .unwrap();
        let DigitalProcessOutcome::Suspended(wait) = start(&plan, process, &mut original).unwrap()
        else {
            panic!("wait");
        };
        let (DigitalWaitRequest::Expressions(wait), _) = wait.into_parts() else {
            panic!("expression");
        };
        let token = original.register_expression_wait(wait, DigitalEventCount::one());
        run_drivers(&mut original);
        let state = signal(&original, "state");
        let nan = 0xfff8_1234_5678_9abcu64;
        original.write_real_signal(state, f64::from_bits(nan));
        assert!(original.checkpoint().is_err(), "undrained transitions");
        drain(&mut original);
        let image = transport(&original.checkpoint().unwrap());
        let mut resumed = restore(&original, &image).unwrap();
        assert_eq!(resumed.real_value(state).unwrap().to_bits(), nan);
        assert_eq!(resumed.checkpoint().unwrap(), image);
        let a = signal(&original, "a");
        for store in [&mut original, &mut resumed] {
            store.write_signal(a, FourStateValue::from_u64(4, 0));
            let transitions = store.take_transitions();
            assert!(
                transitions
                    .iter()
                    .any(|transition| transition.expressions.contains(&token))
            );
            assert!(!store.expression_waits.contains_key(&token));
            run_drivers(store);
            drain(store);
        }
        assert_eq!(
            resumed.checkpoint().unwrap(),
            original.checkpoint().unwrap()
        );
    }

    #[test]
    fn store_checkpoint_preserves_external_strengths_real_contributions_and_observers() {
        let mut original = store(
            "module connected; wire bus,other; wrealsum total; assign bus=1'b1; assign other=1'bz; assign total=2.0; endmodule",
        );
        let bus = signal(&original, "bus");
        let other = signal(&original, "other");
        let total = signal(&original, "total");
        original
            .connect_bits(&[vec![
                DigitalBitConnection {
                    signal: bus,
                    bit: 0,
                },
                DigitalBitConnection {
                    signal: other,
                    bit: 0,
                },
            ]])
            .unwrap();
        let target = |instance: &str| EventTarget {
            node_id: usize::MAX,
            port_name: "out".into(),
            instance: instance.into(),
            driver_index: 0,
        };
        let bits = original
            .attach_external_bits(&[0], &[(0, target("bit"))])
            .unwrap();
        original.observe_other_drivers(&bits).unwrap();
        let reals = original
            .attach_external_reals(&[total], &[(total, target("real"))])
            .unwrap();
        original.configure_traces(
            &[
                (1, TraceSource::ResolvedBit(0)),
                (2, TraceSource::Real(total)),
            ],
            Arc::from([true, true]),
        );
        original.set_activation_clock(DigitalClock {
            tick: 3,
            absolute_seconds: 2.75e-9,
        });
        run_drivers(&mut original);
        original.publish_external_bank(
            &[(
                bits[0],
                DigitalValue::new(DigitalState::Zero, DigitalStrength::Resistive),
            )],
            &[(reals[0], 3.0)],
        );
        assert!(original.checkpoint().is_err());
        drain(&mut original);
        let image = transport(&original.checkpoint().unwrap());
        let mut resumed = restore(&original, &image).unwrap();
        assert_eq!(resumed.checkpoint().unwrap(), image);
        assert_eq!(resumed.connected_value(0), Some(DigitalValue::one()));
        assert_eq!(resumed.real_value(total), Some(5.0));
        for store in [&mut original, &mut resumed] {
            store.publish_external_bank(&[(bits[0], DigitalValue::zero())], &[(reals[0], 7.0)]);
            assert_eq!(store.connected_value(0), Some(DigitalValue::unknown()));
            assert_eq!(store.real_value(total), Some(9.0));
            assert_eq!(store.other_driver_value(bits[0]), Some(DigitalValue::one()));
            assert!(store.has_traces());
            drain(store);
        }
        assert_eq!(
            resumed.checkpoint().unwrap(),
            original.checkpoint().unwrap()
        );
        let mut malformed = serde_json::to_value(image).unwrap();
        malformed["bits"]["resolved"][0] = serde_json::to_value(DigitalValue::zero()).unwrap();
        let malformed = serde_json::from_value(malformed).unwrap();
        assert!(restore(&original, &malformed).is_err());
    }

    #[test]
    fn store_checkpoint_rejects_inconsistent_images_without_changing_the_receiver() {
        let mut original = store(
            "module checked; reg a,q; wire y; assign y=a; initial begin a=1; @(a | q) q=1; end endmodule",
        );
        let plan = Arc::clone(&original.plan);
        let initial = plan
            .processes
            .iter()
            .find(|p| p.kind == DigitalProcessKind::Initial)
            .unwrap();
        let DigitalProcessOutcome::Suspended(wait) = start(&plan, initial, &mut original).unwrap()
        else {
            panic!("wait");
        };
        let (DigitalWaitRequest::Expressions(wait), _) = wait.into_parts() else {
            panic!("expression");
        };
        original.register_expression_wait(wait, DigitalEventCount::one());
        run_drivers(&mut original);
        drain(&mut original);
        let image = original.checkpoint().unwrap();
        for case in 0..7 {
            let mut bad = image.clone();
            match case {
                0 => bad.values.pop().map(|_| ()).unwrap(),
                1 => {
                    bad.contributions[0] = Some(DigitalScalarCheckpoint::capture(
                        &DigitalScalar::FourState(FourStateValue::from_u64(1, 0)),
                    ))
                }
                2 => bad.expressions[0].token = bad.sequence + 1,
                3 => bad.expressions.push(bad.expressions[0].clone()),
                4 => bad.trace_bindings[0] ^= 1,
                5 => bad.analog_bindings[0] ^= 1,
                _ => bad.clock = Some((0, f64::NAN.to_bits())),
            }
            assert!(restore(&original, &bad).is_err(), "case {case}");
            assert_eq!(original.checkpoint().unwrap(), image);
        }
        original.external_batch = Some(Vec::new());
        assert!(original.checkpoint().is_err());
        original.external_batch = None;
        original.sequence = None;
        assert!(original.checkpoint().is_err());
    }
}
