//! Atomic persistence of an initialized, settled HDL host. The enclosing mixed
//! circuit must also authenticate physical state, participants and source inputs.
use super::super::store::checkpoint::StoreCheckpoint;
use super::*;
use crate::xspice::event_scheduler::{SchedulerCheckpoint, SchedulerCheckpointLimits};
use rspice_veriloga::canonical_ir::digital_eval::checkpoint::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub(crate) struct HostCheckpointLimits {
    pub scheduler: SchedulerCheckpointLimits,
    pub digital: DigitalCheckpointLimits,
    pub max_processes: usize,
    pub max_updates: usize,
}
impl Default for HostCheckpointLimits {
    fn default() -> Self {
        Self {
            scheduler: Default::default(),
            digital: Default::default(),
            max_processes: 1_048_576,
            max_updates: 4_194_304,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct HostCheckpoint {
    version: u32,
    plan: [u8; 32],
    tick_seconds: u64,
    scheduler: SchedulerCheckpoint,
    store: StoreCheckpoint,
    slots: Vec<SlotImage>,
    delayed: Vec<(u64, Vec<DigitalDeferredCheckpoint>)>,
    events: Vec<EventImage>,
    expressions: Vec<(u64, DigitalDeferredCheckpoint)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SlotImage {
    status: StatusImage,
    resume: Option<DigitalResumeCheckpoint>,
    after: u64,
    remaining: DigitalEventCountCheckpoint,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum StatusImage {
    Queued,
    Event(DigitalWaitCheckpoint),
    Expression(u64),
    Finished,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EventImage {
    sequence: u64,
    remaining: DigitalEventCountCheckpoint,
    wait: DigitalWaitCheckpoint,
    update: DigitalDeferredCheckpoint,
}

fn check_size(
    processes: usize,
    mut updates: impl Iterator<Item = usize>,
    limits: HostCheckpointLimits,
) -> Result<(), String> {
    if processes > limits.max_processes {
        return Err("host process count exceeds checkpoint limit".into());
    }
    updates
        .try_fold(0usize, |total, count| {
            total
                .checked_add(count)
                .filter(|total| *total <= limits.max_updates)
        })
        .ok_or_else(|| "host update count exceeds checkpoint limit".to_string())?;
    Ok(())
}

impl DigitalHost {
    fn checkpoint_boundary(&self) -> Result<(), String> {
        if !self.elaboration_closed || self.checkpoint_failed {
            return Err("host must be initialized and successfully settled before capture".into());
        }
        if !self.inactive.is_empty()
            || !self.analog_waiters.is_empty()
            || self.analog_ready.is_some()
            || self.analog_activation_seconds.is_some()
            || self.analog_exchange_seconds.is_some()
            || self.slots.iter().any(|slot| {
                matches!(
                    slot.status,
                    ProcessStatus::Inactive | ProcessStatus::AwaitingAnalog(_)
                )
            })
        {
            return Err("host has unfinished same-time execution".into());
        }
        Ok(())
    }

    pub(crate) fn checkpoint(
        &self,
        limits: HostCheckpointLimits,
    ) -> Result<HostCheckpoint, String> {
        self.checkpoint_boundary()?;
        check_size(
            self.slots.len(),
            self.delayed_updates
                .values()
                .map(Vec::len)
                .chain([self.event_updates.len(), self.expression_updates.len()]),
            limits,
        )?;
        let scheduler = self
            .scheduler
            .checkpoint(limits.scheduler)
            .map_err(|e| e.to_string())?;
        let store = self.store.checkpoint()?;
        let reader =
            DigitalCheckpointReader::new(&self.plan, limits.digital).map_err(|e| e.to_string())?;
        self.validate_checkpoint_links(&scheduler, &reader)?;
        let slots = self
            .slots
            .iter()
            .map(|slot| SlotImage {
                status: match &slot.status {
                    ProcessStatus::Queued => StatusImage::Queued,
                    ProcessStatus::AwaitingEvent(terms) => {
                        StatusImage::Event(DigitalWaitCheckpoint::capture(
                            &self.plan,
                            &DigitalWaitRequest::Event(terms.clone()),
                        ))
                    }
                    ProcessStatus::AwaitingExpression(token) => StatusImage::Expression(*token),
                    ProcessStatus::Finished => StatusImage::Finished,
                    _ => unreachable!("checked drained boundary"),
                },
                resume: slot.resume.as_ref().map(DigitalResumeCheckpoint::capture),
                after: slot.wait_after_sequence,
                remaining: DigitalEventCountCheckpoint::capture(&slot.remaining_events),
            })
            .collect();
        Ok(HostCheckpoint {
            version: 1,
            plan: self.plan.content_identity,
            tick_seconds: self.resolution.seconds_per_tick().to_bits(),
            scheduler,
            store,
            slots,
            delayed: self
                .delayed_updates
                .iter()
                .map(|(tick, updates)| {
                    (
                        *tick,
                        updates
                            .iter()
                            .map(|u| DigitalDeferredCheckpoint::capture(&self.plan, u))
                            .collect(),
                    )
                })
                .collect(),
            events: self
                .event_updates
                .values()
                .map(|event| EventImage {
                    sequence: event.sequence,
                    remaining: DigitalEventCountCheckpoint::capture(&event.remaining),
                    wait: DigitalWaitCheckpoint::capture(
                        &self.plan,
                        &DigitalWaitRequest::Event(event.terms.clone()),
                    ),
                    update: DigitalDeferredCheckpoint::capture(&self.plan, &event.update),
                })
                .collect(),
            expressions: self
                .expression_updates
                .iter()
                .map(|(token, update)| {
                    (
                        *token,
                        DigitalDeferredCheckpoint::capture(&self.plan, update),
                    )
                })
                .collect(),
        })
    }

    /// All decoding, ownership and scheduling checks precede the only live swap.
    /// Input bytes must be bounded by the enclosing circuit file reader.
    pub(crate) fn restore_checkpoint(
        &mut self,
        image: &HostCheckpoint,
        limits: HostCheckpointLimits,
    ) -> Result<(), String> {
        let restored = self.restored_checkpoint(image, limits)?;
        *self = restored;
        Ok(())
    }

    pub(crate) fn restored_checkpoint(
        &self,
        image: &HostCheckpoint,
        limits: HostCheckpointLimits,
    ) -> Result<Self, String> {
        if image.version != 1
            || image.plan != self.plan.content_identity
            || image.tick_seconds != self.resolution.seconds_per_tick().to_bits()
        {
            return Err("host checkpoint schema, plan or timing differs".into());
        }
        check_size(
            image.slots.len(),
            image
                .delayed
                .iter()
                .map(|(_, updates)| updates.len())
                .chain([image.events.len(), image.expressions.len()]),
            limits,
        )?;
        if image.slots.len() != self.plan.processes.len() {
            return Err("host process dimensions differ".into());
        }
        let mut reader =
            DigitalCheckpointReader::new(&self.plan, limits.digital).map_err(|e| e.to_string())?;
        let mut restored = self.fresh();
        // Preserve the receiving registration order, including interleaved real
        // and bit participants. No old queued event or accounting is copied.
        restored.scheduler = self.scheduler.checkpoint_template();
        restored.targets.clone_from(&self.targets);
        restored.nba_target = self.nba_target;
        restored.external_targets.clone_from(&self.external_targets);
        restored
            .external_real_targets
            .clone_from(&self.external_real_targets);
        restored
            .process_of_target
            .clone_from(&self.process_of_target);
        restored
            .scheduler
            .restore_checkpoint(&image.scheduler, limits.scheduler)
            .map_err(|e| e.to_string())?;
        restored.store = self.store.restored_checkpoint(&image.store, &mut reader)?;
        restored.elaboration_closed = true;
        for (index, image) in image.slots.iter().enumerate() {
            let resume = image
                .resume
                .as_ref()
                .map(|image| reader.restore_resume(image))
                .transpose()
                .map_err(|e| e.to_string())?;
            let remaining_events = reader
                .restore_count(&image.remaining)
                .map_err(|e| e.to_string())?;
            let status = match &image.status {
                StatusImage::Queued => ProcessStatus::Queued,
                StatusImage::Finished => ProcessStatus::Finished,
                StatusImage::Event(image) => {
                    let DigitalWaitRequest::Event(terms) =
                        reader.restore_wait(image).map_err(|e| e.to_string())?
                    else {
                        return Err("process event subscription has the wrong domain".into());
                    };
                    restored.subscribe(index, &terms);
                    ProcessStatus::AwaitingEvent(terms)
                }
                StatusImage::Expression(token) => {
                    if restored
                        .expression_processes
                        .insert(*token, index)
                        .is_some()
                    {
                        return Err("expression subscription has multiple processes".into());
                    }
                    ProcessStatus::AwaitingExpression(*token)
                }
            };
            restored.slots[index] = ProcessSlot {
                status,
                resume,
                remaining_events,
                wait_after_sequence: image.after,
            };
        }
        let mut previous = None;
        for (tick, images) in &image.delayed {
            if previous.is_some_and(|previous| previous >= *tick) || images.is_empty() {
                return Err("unordered or empty delayed update bucket".into());
            }
            restored.instant_of(*tick).map_err(|e| e.to_string())?;
            let mut updates = Vec::with_capacity(images.len());
            for image in images {
                updates.push(reader.restore_deferred(image).map_err(|e| e.to_string())?);
            }
            restored.delayed_updates.insert(*tick, updates);
            previous = Some(*tick);
        }
        let mut previous = None;
        for image in &image.events {
            if previous.is_some_and(|previous| previous >= image.sequence) {
                return Err("unordered event capture identities".into());
            }
            let DigitalWaitRequest::Event(terms) = reader
                .restore_wait(&image.wait)
                .map_err(|e| e.to_string())?
            else {
                return Err("event capture has the wrong subscription domain".into());
            };
            let remaining = reader
                .restore_count(&image.remaining)
                .map_err(|e| e.to_string())?;
            let update = reader
                .restore_deferred(&image.update)
                .map_err(|e| e.to_string())?;
            for term in &terms {
                restored.event_waiters[usize::from(term.signal)].insert(image.sequence);
            }
            restored.event_updates.insert(
                image.sequence,
                EventCapture {
                    sequence: image.sequence,
                    remaining,
                    terms,
                    update,
                },
            );
            previous = Some(image.sequence);
        }
        let mut previous = None;
        for (token, image) in &image.expressions {
            if previous.is_some_and(|previous| previous >= *token) {
                return Err("unordered expression update identities".into());
            }
            let update = reader.restore_deferred(image).map_err(|e| e.to_string())?;
            restored.expression_updates.insert(*token, update);
            previous = Some(*token);
        }
        restored.validate_checkpoint_links(&image.scheduler, &reader)?;
        Ok(restored)
    }

    fn validate_checkpoint_links(
        &self,
        scheduler: &SchedulerCheckpoint,
        reader: &DigitalCheckpointReader<'_>,
    ) -> Result<(), String> {
        let sequence = self
            .store
            .current_sequence()
            .ok_or_else(|| "exhausted store sequence".to_string())?;
        let mut expression_owners = BTreeSet::new();
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.wait_after_sequence > sequence
                || slot
                    .resume
                    .as_ref()
                    .is_some_and(|resume| usize::from(resume.process()) != index)
            {
                return Err("process frame or event barrier belongs to another execution".into());
            }
            let require_frame = || {
                slot.resume
                    .as_ref()
                    .ok_or_else(|| "suspended process has no continuation".to_string())
            };
            match &slot.status {
                ProcessStatus::Queued => reader
                    .validate_timed_resume(require_frame()?)
                    .map_err(|e| e.to_string())?,
                ProcessStatus::AwaitingEvent(terms) => {
                    let repeated = reader
                        .validate_event_resume(
                            require_frame()?,
                            &DigitalWaitRequest::Event(terms.clone()),
                        )
                        .map_err(|e| e.to_string())?;
                    check_remaining(&slot.remaining_events, repeated)?;
                    if self.plan.processes[index].kind == DigitalProcessKind::ContinuousAssign
                        && slot.wait_after_sequence != 0
                    {
                        return Err("continuous driver has a procedural event barrier".into());
                    }
                }
                ProcessStatus::AwaitingExpression(token) => {
                    let (wait, remaining) = self
                        .store
                        .checkpoint_expression(*token)
                        .ok_or_else(|| "process subscription is absent from store".to_string())?;
                    if self.expression_processes.get(token) != Some(&index)
                        || !expression_owners.insert(*token)
                    {
                        return Err("process expression ownership differs".into());
                    }
                    let repeated = reader
                        .validate_event_resume(
                            require_frame()?,
                            &DigitalWaitRequest::Expressions(wait.clone()),
                        )
                        .map_err(|e| e.to_string())?;
                    check_remaining(remaining, repeated)?;
                }
                ProcessStatus::Finished => {
                    if self.plan.processes[index].kind == DigitalProcessKind::Always {
                        return Err("always process is marked finished".into());
                    }
                }
                _ => return Err("host has unfinished same-time execution".into()),
            }
        }
        if expression_owners.len() != self.expression_processes.len() {
            return Err("orphan process expression ownership".into());
        }
        for (token, update) in &self.expression_updates {
            if self.store.checkpoint_expression(*token).is_none()
                || !expression_owners.insert(*token)
                || update.wait.is_some()
            {
                return Err("expression update ownership or capture differs".into());
            }
        }
        if !self
            .store
            .checkpoint_expression_tokens()
            .eq(expression_owners.iter().copied())
        {
            return Err("orphan store expression subscription".into());
        }
        for (token, event) in &self.event_updates {
            if *token == 0
                || *token != event.sequence
                || *token > sequence
                || expression_owners.contains(token)
                || event.remaining.is_zero()
                || event.update.wait.is_some()
            {
                return Err("event capture identity, count or ownership differs".into());
            }
        }
        for updates in self.delayed_updates.values() {
            if updates.is_empty() || updates.iter().any(|update| !matches!(update.wait, Some(DigitalWaitRequest::Delay(delay)) if delay > 0)) {
                return Err("delayed bucket contains an invalid timed capture".into());
            }
        }

        let processes: BTreeMap<_, _> = self
            .targets
            .iter()
            .copied()
            .enumerate()
            .map(|(index, target)| (target, index))
            .collect();
        let known: BTreeSet<_> = self
            .targets
            .iter()
            .chain([&self.nba_target])
            .chain(&self.external_targets)
            .chain(&self.external_real_targets)
            .copied()
            .collect();
        if known.len() != scheduler.target_count()
            || processes.len() != self.slots.len()
            || processes.contains_key(&self.nba_target)
        {
            return Err("scheduler target registration differs from the host".into());
        }
        let mut activations = vec![0usize; self.slots.len()];
        let mut buckets = BTreeSet::new();
        for (at, region, target, value) in scheduler.live_events() {
            if region != SchedulerRegion::Active
                || value != EventValue::Digital(DigitalValue::default())
            {
                return Err("host wakeup region or payload differs".into());
            }
            let tick = self.tick_of(at).map_err(|e| e.to_string())?;
            if self.instant_of(tick).map_err(|e| e.to_string())? != at {
                return Err("host wakeup is not on its design grid".into());
            }
            if target == self.nba_target {
                if !self.delayed_updates.contains_key(&tick) || !buckets.insert(tick) {
                    return Err("NBA wakeup has no unique delayed bucket".into());
                }
            } else if let Some(index) = processes.get(&target) {
                activations[*index] += 1;
                if activations[*index] != 1 {
                    return Err("process has multiple pending wakeups".into());
                }
            } else {
                return Err("host scheduler contains an unowned wakeup".into());
            }
        }
        if buckets.len() != self.delayed_updates.len() {
            return Err("delayed update bucket has no wakeup".into());
        }
        for (slot, count) in self.slots.iter().zip(activations) {
            if count != usize::from(matches!(slot.status, ProcessStatus::Queued)) {
                return Err("queued process and scheduled wakeup disagree".into());
            }
        }
        Ok(())
    }
}

fn check_remaining(count: &DigitalEventCount, repeated: bool) -> Result<(), String> {
    if count.is_zero() || (!repeated && count != &DigitalEventCount::one()) {
        return Err("process event count differs from its source wait".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    const SOURCE: &str = r#"
`timescale 1ns/1ns
module resumed;
reg clk,a,b,expr_done; reg [7:0] data,timed,event_q,expr_q,out,prefix;
wire [7:0] bus; real rv,rout;
assign bus=out;
initial begin
  clk=0; a=0; b=1; data=8'h42; prefix=1; out=0; rv=-0.0;
  timed<=#5 data;
  event_q<=repeat(3) @(posedge clk) data;
  expr_q<=@(posedge (a & b)) data;
  out = #7 data;
  #1; rout=rv;
end
initial begin expr_done=0; @(a ^ b) expr_done=1; end
endmodule
"#;
    fn host(source: &str) -> DigitalHost {
        let plan = rspice_veriloga::VerilogACompiler::default()
            .compile_canonical_ir(source)
            .unwrap()
            .digital;
        DigitalHost::new(
            &plan,
            TimeResolution::new(-9).unwrap(),
            SchedulerLimits::default(),
        )
    }
    fn signal(host: &DigitalHost, name: &str) -> DigitalSignalId {
        host.signal(name).unwrap()
    }
    fn force(host: &mut DigitalHost, name: &str, width: u32, value: u64, tick: u64) {
        host.force(
            signal(host, name),
            FourStateValue::from_u64(width, value),
            tick,
        )
        .unwrap();
    }
    fn image(host: &DigitalHost) -> HostCheckpoint {
        host.checkpoint(HostCheckpointLimits::default()).unwrap()
    }
    fn transport(image: &HostCheckpoint) -> HostCheckpoint {
        serde_json::from_slice(&serde_json::to_vec(image).unwrap()).unwrap()
    }
    fn prepared() -> DigitalHost {
        let mut host = host(SOURCE);
        host.start().unwrap();
        force(&mut host, "clk", 1, 1, 1);
        force(&mut host, "clk", 1, 0, 1);
        force(&mut host, "data", 8, 0x99, 1);
        force(&mut host, "prefix", 8, 100, 1);
        host
    }
    fn finish(host: &mut DigitalHost) {
        force(host, "clk", 1, 1, 2);
        force(host, "clk", 1, 0, 2);
        force(host, "a", 1, 1, 3);
        force(host, "clk", 1, 1, 4);
        host.advance_to(8).unwrap();
        for name in ["timed", "event_q", "expr_q", "out", "bus"] {
            assert_eq!(
                host.read(signal(host, name)).unwrap().to_u64(),
                Some(0x42),
                "{name}"
            );
        }
        assert_eq!(
            host.read(signal(host, "prefix")).unwrap().to_u64(),
            Some(100)
        );
        assert_eq!(
            host.read(signal(host, "expr_done")).unwrap().to_u64(),
            Some(1)
        );
        assert_eq!(
            host.read_real(signal(host, "rout")).unwrap().to_bits(),
            (-0.0f64).to_bits()
        );
    }

    #[test]
    fn host_checkpoint_resumes_timers_captured_updates_and_repeated_expression_events() {
        let mut original = prepared();
        let saved = transport(&image(&original));
        let mut restored = host(SOURCE);
        restored
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&restored), saved);
        finish(&mut original);
        finish(&mut restored);
        assert_eq!(image(&restored), image(&original));
    }

    #[test]
    fn host_checkpoint_preserves_an_analog_clock_with_an_undelivered_same_tick_timer() {
        let source = "`timescale 1ns/1ns\nmodule causal; reg a,q; real observed; initial begin a=0; q=0; #1; q=1; end always @(a) observed=$abstime; endmodule";
        let mut original = host(source);
        original.start().unwrap();
        original
            .force_many_from_analog_ordered(
                &[(signal(&original, "a"), FourStateValue::from_u64(1, 1))],
                1,
                0.4e-9,
                0.6e-9,
            )
            .unwrap();
        assert_eq!(
            original.read(signal(&original, "q")).unwrap().to_u64(),
            Some(0)
        );
        assert_eq!(
            original.read_real(signal(&original, "observed")),
            Some(0.4e-9)
        );
        let saved = transport(&image(&original));
        let mut restored = host(source);
        restored
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&restored), saved);
        original.advance_to(1).unwrap();
        restored.advance_to(1).unwrap();
        assert_eq!(
            restored.read(signal(&restored, "q")).unwrap().to_u64(),
            Some(1)
        );
        assert_eq!(image(&restored), image(&original));
    }

    #[test]
    fn host_checkpoint_rejects_orphaned_frames_wakeups_and_subscriptions_atomically() {
        let mut receiver = prepared();
        let before = image(&receiver);
        let base = serde_json::to_value(&before).unwrap();
        let queued = receiver
            .slots
            .iter()
            .position(|slot| matches!(slot.status, ProcessStatus::Queued))
            .unwrap();
        let event = receiver
            .slots
            .iter()
            .position(|slot| matches!(slot.status, ProcessStatus::AwaitingEvent(_)))
            .unwrap();
        let expression = receiver
            .slots
            .iter()
            .position(|slot| matches!(slot.status, ProcessStatus::AwaitingExpression(_)))
            .unwrap();
        let mut bad: Vec<Value> = Vec::new();
        let mut value = base.clone();
        value["scheduler"]["events"].as_array_mut().unwrap().pop();
        bad.push(value);
        let mut value = base.clone();
        value["slots"][queued]["status"] = json!("Finished");
        bad.push(value);
        let mut value = base.clone();
        value["slots"][queued]["resume"] = Value::Null;
        bad.push(value);
        let mut value = base.clone();
        value["slots"][event]["status"]["Event"]["base"]["Event"][0]["signal"] =
            json!(signal(&receiver, "data").index());
        bad.push(value);
        let mut value = base.clone();
        value["slots"][queued]["status"] = value["slots"][expression]["status"].clone();
        bad.push(value);
        let mut value = base.clone();
        value["store"]["expressions"].as_array_mut().unwrap().pop();
        bad.push(value);
        let mut value = base.clone();
        value["delayed"] = json!([]);
        bad.push(value);
        let mut value = base.clone();
        value["slots"][event]["after"] = json!(u64::MAX);
        bad.push(value);
        let mut value = base.clone();
        value["tick_seconds"] = json!(0);
        bad.push(value);
        let mut value = base.clone();
        value["events"][0]["remaining"]["aval"] = json!([0]);
        bad.push(value);
        for (case, value) in bad.into_iter().enumerate() {
            let bad: HostCheckpoint = serde_json::from_value(value).unwrap();
            assert!(
                receiver
                    .restore_checkpoint(&bad, HostCheckpointLimits::default())
                    .is_err(),
                "case {case}"
            );
            assert_eq!(image(&receiver), before, "case {case} changed receiver");
        }
        assert!(
            receiver
                .restore_checkpoint(
                    &before,
                    HostCheckpointLimits {
                        max_updates: 0,
                        ..Default::default()
                    }
                )
                .is_err()
        );
        assert_eq!(image(&receiver), before);
    }

    #[test]
    fn host_checkpoint_refuses_failed_settlement_and_recovers_only_through_valid_restore() {
        let mut receiver = host("module done; reg q; initial q=1; endmodule");
        assert!(
            receiver
                .checkpoint(HostCheckpointLimits::default())
                .is_err()
        );
        receiver.start().unwrap();
        let saved = image(&receiver);
        struct Fail;
        impl DigitalActiveParticipant for Fail {
            fn settle_active(
                &mut self,
                _: &mut DigitalActiveExchange<'_>,
            ) -> Result<bool, DigitalRunError> {
                Err(DigitalRunError::ExternalExecution {
                    detail: "fixture participant failure".into(),
                })
            }
        }
        assert!(receiver.settle_with(1, &mut Fail).is_err());
        assert!(
            receiver
                .checkpoint(HostCheckpointLimits::default())
                .is_err()
        );
        receiver
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&receiver), saved);
    }

    #[test]
    fn host_checkpoint_retains_interleaved_external_registration_and_driver_accounting() {
        use crate::xspice::verilog::store::{
            DigitalBitConnection, ExternalBitDriverId, ExternalRealDriverId,
        };
        fn enrolled() -> (DigitalHost, ExternalBitDriverId, ExternalRealDriverId) {
            let mut host = host(
                "module enrolled; wire d; wreal r; reg q; real captured; initial @(posedge d) q=1; always @(r) captured=r; endmodule",
            );
            let d = signal(&host, "d");
            let r = signal(&host, "r");
            host.connect_bits(&[vec![DigitalBitConnection { signal: d, bit: 0 }]])
                .unwrap();
            let target = |name: &str| EventTarget {
                node_id: usize::MAX,
                port_name: "out".into(),
                driver_index: 0,
                instance: name.into(),
            };
            // Deliberately opposite to fresh()'s registration order.
            let real = host
                .attach_external_reals(&[r], &[(r, target("real"))])
                .unwrap()[0];
            let bit = host
                .attach_external_bits(&[0], &[(0, target("bit"))])
                .unwrap()[0];
            (host, bit, real)
        }
        struct Driver {
            bits: Vec<(ExternalBitDriverId, DigitalValue)>,
            reals: Vec<(ExternalRealDriverId, f64)>,
        }
        impl DigitalActiveParticipant for Driver {
            fn settle_active(
                &mut self,
                exchange: &mut DigitalActiveExchange<'_>,
            ) -> Result<bool, DigitalRunError> {
                let mut changed = !exchange.take_event_changes().is_empty();
                if !self.bits.is_empty() || !self.reals.is_empty() {
                    exchange.drive_bank(&self.bits, &self.reals)?;
                    self.bits.clear();
                    self.reals.clear();
                    changed = true;
                }
                Ok(changed)
            }
        }
        let (mut original, bit, real) = enrolled();
        original.prepare_start().unwrap();
        original
            .settle_with(
                0,
                &mut Driver {
                    bits: vec![(bit, DigitalValue::zero())],
                    reals: vec![(real, 1.25)],
                },
            )
            .unwrap();
        let saved = transport(&image(&original));
        let (mut restored, restored_bit, _) = enrolled();
        restored
            .restore_checkpoint(&saved, HostCheckpointLimits::default())
            .unwrap();
        assert_eq!(image(&restored), saved);
        original
            .settle_with(
                2,
                &mut Driver {
                    bits: vec![(bit, DigitalValue::one())],
                    reals: vec![],
                },
            )
            .unwrap();
        restored
            .settle_with(
                2,
                &mut Driver {
                    bits: vec![(restored_bit, DigitalValue::one())],
                    reals: vec![],
                },
            )
            .unwrap();
        assert_eq!(
            restored.read(signal(&restored, "q")).unwrap().to_u64(),
            Some(1)
        );
        assert_eq!(
            restored.read_real(signal(&restored, "captured")),
            Some(1.25)
        );
        assert_eq!(image(&restored), image(&original));
    }
}
