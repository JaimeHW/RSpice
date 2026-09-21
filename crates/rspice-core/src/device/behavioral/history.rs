//! Accepted operator history, bound to each source and compiled SDT occurrence.
use super::*;
use crate::expr::AcceptedSdtState;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BehavioralAcceptedState {
    pub voltage: bool,
    pub name: String,
    pub integrals: Vec<AcceptedSdtState>,
}

impl BehavioralSources {
    fn stateful_vms(&self) -> impl Iterator<Item = (bool, &str, usize, &Vm)> {
        self.voltage_sources
            .iter()
            .map(|s| (true, s.name.as_str(), s.program.sdt_count, &s.vm))
            .chain(
                self.current_sources
                    .iter()
                    .map(|s| (false, s.name.as_str(), s.program.sdt_count, &s.vm)),
            )
            .filter(|(_, _, count, _)| *count != 0)
    }

    pub(crate) fn integral_count(&self) -> usize {
        self.stateful_vms().map(|(_, _, count, _)| count).sum()
    }

    pub(crate) fn integral_names(&self) -> impl Iterator<Item = String> + '_ {
        self.stateful_vms().flat_map(|(_, name, count, _)| {
            (0..count).map(move |index| format!("B:{name}:sdt:{index}"))
        })
    }

    pub(crate) fn accepted_integrals(&self) -> impl Iterator<Item = Value> + '_ {
        self.stateful_vms()
            .flat_map(|(_, _, count, vm)| vm.accepted_integrals(count))
    }

    /// A shooting state supplies integration constants, not prior trial frames.
    /// The origin consistency solve will establish the accepted input values.
    pub(crate) fn reset_integrals(&mut self, values: &[Value]) -> Result<(), String> {
        if values.len() != self.integral_count() || values.iter().any(|value| !value.is_finite()) {
            return Err(
                "behavioral SDT shooting state has invalid dimensions or non-finite values".into(),
            );
        }
        let mut values = values.iter();
        for source in &mut self.voltage_sources {
            let history = (0..source.program.sdt_count)
                .map(|_| AcceptedSdtState {
                    integral: *values.next().expect("validated integral count"),
                    ..Default::default()
                })
                .collect::<Vec<_>>();
            if !history.is_empty() {
                source.vm.restore_sdt_history(&history);
                source.invalidate_cached_exact_constraint();
            }
        }
        for source in &mut self.current_sources {
            let history = (0..source.program.sdt_count)
                .map(|_| AcceptedSdtState {
                    integral: *values.next().expect("validated integral count"),
                    ..Default::default()
                })
                .collect::<Vec<_>>();
            if !history.is_empty() {
                source.vm.restore_sdt_history(&history);
            }
        }
        Ok(())
    }

    /// Rebase an already accepted periodic endpoint without resetting its
    /// integral values or inputs. No integration is performed during this move.
    pub(crate) fn rebase_accepted_history(&mut self, time: Value) -> Result<(), String> {
        if !time.is_finite() || time < 0.0 {
            return Err("behavioral SDT history origin must be finite and non-negative".into());
        }
        let mut history = self.accepted_history();
        for state in &mut history {
            for integral in &mut state.integrals {
                integral.time = time;
            }
        }
        self.restore_accepted_history(Some(&history), time)
    }

    pub(crate) fn accepted_history(&self) -> Vec<BehavioralAcceptedState> {
        self.stateful_vms()
            .map(|(voltage, name, count, vm)| BehavioralAcceptedState {
                voltage,
                name: name.to_owned(),
                integrals: vm.accepted_sdt_history(count),
            })
            .collect()
    }

    pub(crate) fn validate_accepted_history(
        &self,
        history: Option<&[BehavioralAcceptedState]>,
        time: Value,
    ) -> Result<(), String> {
        let Some(history) = history else {
            return if self.stateful_vms().next().is_some() {
                Err("legacy transient checkpoint does not contain behavioral SDT history; re-run the transient from t=0".into())
            } else {
                Ok(())
            };
        };
        if self.stateful_vms().count() != history.len() {
            return Err("behavioral SDT checkpoint source count does not match the circuit".into());
        }
        for ((voltage, name, count, _), state) in self.stateful_vms().zip(history) {
            if state.voltage != voltage || state.name != name || state.integrals.len() != count {
                return Err(format!(
                    "behavioral SDT checkpoint identity or operator count does not match source '{name}'"
                ));
            }
            if state
                .integrals
                .iter()
                .any(|s| s.time != time || !s.input.is_finite() || !s.integral.is_finite())
            {
                return Err(format!(
                    "behavioral SDT checkpoint for '{name}' has invalid accepted history"
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn restore_accepted_history(
        &mut self,
        history: Option<&[BehavioralAcceptedState]>,
        time: Value,
    ) -> Result<(), String> {
        self.validate_accepted_history(history, time)?;
        let Some(history) = history else {
            return Ok(());
        };
        let mut states = history.iter();
        for source in &mut self.voltage_sources {
            if source.program.sdt_count != 0 {
                source
                    .vm
                    .restore_sdt_history(&states.next().expect("validated history").integrals);
                source.invalidate_cached_exact_constraint();
            }
        }
        for source in &mut self.current_sources {
            if source.program.sdt_count != 0 {
                source
                    .vm
                    .restore_sdt_history(&states.next().expect("validated history").integrals);
            }
        }
        Ok(())
    }
}
