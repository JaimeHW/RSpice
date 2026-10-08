//! A one-instance adapter to the circuit's shared event coordinator.
//! The coordinator owns every process and observer; the host remains its analog
//! participant and signal view. No observer or scheduling semantics live here.
use super::*;
use super::super::host::{DigitalActiveExchange, DigitalActiveParticipant};
use crate::abort_signal::AbortSignal;
use std::collections::VecDeque;

#[derive(Clone)]
pub(super) struct StandaloneExecution {
    coordinator: shared::MixedDigitalCoordinator,
    cursor: Option<shared::SharedTrialCursor>,
    inputs: StandaloneInputs,
}

#[derive(Clone, Default)]
struct StandaloneInputs {
    time: f64,
    banks: VecDeque<Vec<(DigitalSignalId, FourStateValue)>>,
}

impl DigitalActiveParticipant for StandaloneInputs {
    fn next_event_time(&self) -> Option<f64> {
        (!self.banks.is_empty()).then_some(self.time)
    }

    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        if exchange.physical_seconds() < self.time {
            return Ok(false);
        }
        let Some(bank) = self.banks.front() else {
            return Ok(false);
        };
        exchange.force_signals(bank)?;
        self.banks.pop_front();
        Ok(true)
    }
}

impl StandaloneExecution {
    pub(super) fn new(coordinator: shared::MixedDigitalCoordinator) -> Self {
        Self {
            coordinator,
            cursor: None,
            inputs: StandaloneInputs::default(),
        }
    }

    pub(super) fn reset(&mut self) {
        self.coordinator = self.coordinator.fresh();
        self.cursor = None;
        self.inputs = StandaloneInputs::default();
    }

    pub(super) fn start(&mut self) -> Result<(), MixedSignalError> {
        self.coordinator.start()
    }

    pub(super) fn set_interval_event_limit(&mut self, limit: usize) {
        self.coordinator.set_interval_event_limit(limit);
    }

    pub(super) fn retain_policy_from(&mut self, current: &Self) {
        self.coordinator
            .set_interval_event_limit(current.coordinator.interval_event_limit());
    }

    pub(super) fn next_event_time(&self) -> Result<Option<f64>, MixedSignalError> {
        Ok(self
            .coordinator
            .next_event_time()?
            .map(|(_, time)| time)
            .into_iter()
            .chain(self.inputs.next_event_time())
            .min_by(f64::total_cmp))
    }

    pub(super) fn begin(&mut self, host: &mut MixedSignalHost) -> Result<(), MixedSignalError> {
        let trial = host.trial.as_ref().expect("opened host trial");
        self.coordinator
            .set_analog_step_floor(host.analog_step_floor());
        let cursor = self
            .coordinator
            .open_trial(trial.time_seconds, trial.probe)?;
        self.inputs.time = trial.time_seconds;
        self.inputs.banks.clear();
        if cursor.opened_on_scheduled_activation() {
            host.note_scheduled_activation();
        }
        self.cursor = Some(cursor);
        Ok(())
    }

    pub(super) fn queue_forces(
        &mut self,
        drives: &[(DigitalSignalId, FourStateValue)],
    ) -> Result<(), MixedSignalError> {
        let bank = self.coordinator.standalone_input_bank(drives)?;
        if !bank.is_empty() {
            self.inputs.banks.push_back(bank);
        }
        Ok(())
    }

    pub(super) fn advance(
        &mut self,
        host: &mut MixedSignalHost,
        solution: &[f64],
    ) -> Result<(), MixedSignalError> {
        let cursor = self
            .cursor
            .as_mut()
            .ok_or_else(|| MixedSignalError::TrialProtocol {
                detail: "standalone coordinator has no active trial".into(),
            })?;
        let external = (!self.inputs.banks.is_empty())
            .then_some(&mut self.inputs as &mut dyn DigitalActiveParticipant);
        self.coordinator
            .advance_with(cursor, std::slice::from_mut(host), solution, external)?;
        self.coordinator.synchronize(std::slice::from_mut(host))?;
        Ok(())
    }

    pub(super) fn settle(
        &mut self,
        host: &mut MixedSignalHost,
        solution: &[f64],
        abort: &dyn AbortSignal,
    ) -> Result<bool, MixedSignalError> {
        self.advance(host, solution)?;
        let mut changed = host.settle_analog_bridges_with_abort(solution, abort)?;
        let cursor = self.cursor.as_mut().expect("advance validated trial");
        let external = (!self.inputs.banks.is_empty())
            .then_some(&mut self.inputs as &mut dyn DigitalActiveParticipant);
        changed |= self.coordinator.publish_adc_with(
            abort,
            cursor,
            std::slice::from_mut(host),
            solution,
            external,
        )?;
        changed |= self.coordinator.synchronize(std::slice::from_mut(host))?;
        if changed {
            host.trial
                .as_mut()
                .expect("active host trial")
                .bridges_quiet = false;
        }
        Ok(changed)
    }

    pub(super) fn commit(&mut self) {
        self.coordinator
            .commit_trial(self.cursor.take().expect("validated standalone trial"));
        self.inputs.banks.clear();
    }

    pub(super) fn reject(&mut self) {
        if let Some(cursor) = self.cursor.take() {
            self.coordinator.rollback_trial(cursor);
        }
        self.inputs.banks.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absdelta_standalone_analysis_reset_restarts_both_domains() {
        let source = "`timescale 1ps/1ps\nmodule observer(a); input a; electrical a; integer count=0; always @(absdelta(V(a),0.25)) count=count+1; endmodule";
        let mut host =
            MixedSignalHost::compile(source, None, "observer", &[1], SchedulerLimits::default())
                .unwrap();
        for run in 0..2 {
            if run != 0 {
                host.begin_analog_analysis(2).unwrap();
                host.start_digital_execution().unwrap();
            }
            for (time, dt, voltage, expected) in [(0.0, 0.0, 0.0, 1), (1e-9, 1e-9, 1.0, 5)] {
                host.begin_trial(
                    time,
                    dt,
                    IntegrationCoefficients::inactive(),
                    time == 0.0,
                    false,
                )
                .unwrap();
                let mut quiet = false;
                for _ in 0..8 {
                    host.stamp(&[voltage], |_, _, _| {}, |_, _| {}).unwrap();
                    if !host.settle_analog_bridges(&[voltage]).unwrap() {
                        quiet = true;
                        break;
                    }
                }
                assert!(quiet);
                host.accept_trial().unwrap();
                assert_eq!(
                    host.read_digital("count").unwrap(),
                    format!("{expected:032b}")
                );
            }
        }
    }
}
