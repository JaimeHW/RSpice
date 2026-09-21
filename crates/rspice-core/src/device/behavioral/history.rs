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
