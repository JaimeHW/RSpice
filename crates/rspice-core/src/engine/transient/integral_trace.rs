//! Internal accepted-state capture for transient-assisted periodic startup.

use super::*;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub(in crate::engine) struct TransientIntegralTrace {
    pub names: Vec<String>,
    pub values: Vec<Vec<Value>>,
}

impl TransientIntegralTrace {
    pub(super) fn retained_value_count(&self) -> usize {
        self.values
            .iter()
            .fold(0usize, |count, row| count.saturating_add(row.len()))
    }

    pub(super) fn initialize(&mut self, circuit: &crate::circuit::CircuitData) {
        self.names = circuit
            .behavioral_sources
            .integral_names()
            .chain(circuit.capacitors.integral_names())
            .collect();
        self.values = self.names.iter().map(|_| Vec::new()).collect();
    }

    pub(super) fn record(
        &mut self,
        circuit: &crate::circuit::CircuitData,
    ) -> Result<usize, SimulationError> {
        if circuit.behavioral_sources.integral_count() + circuit.capacitors.integral_count()
            != self.values.len()
        {
            return Err(SimulationError::Circuit(
                "transient integral-state capture basis changed".into(),
            ));
        }
        for (row, value) in self.values.iter_mut().zip(
            circuit
                .behavioral_sources
                .accepted_integrals()
                .chain(circuit.capacitors.accepted_integrals()),
        ) {
            if !value.is_finite() {
                return Err(SimulationError::Circuit(
                    "transient integral-state capture contains a non-finite accepted value".into(),
                ));
            }
            row.try_reserve(1).map_err(|_| {
                SimulationError::Circuit(
                    "transient integral-state capture allocation failed".into(),
                )
            })?;
            row.push(value);
        }
        Ok(self.values.len())
    }
}

impl Engine {
    /// The periodic initializer owns the complete accepted physical and memory
    /// trajectory. Output selectors must not discard its state coordinates.
    pub(in crate::engine) fn run_tran_for_periodic_seed(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        dc_seed: Option<&super::super::PeriodicDcOperatingPointSeed>,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientIntegralTrace), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        self.reset_convergence_quality();
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_transient_request_floor(tstop, max_step)?;
        let mut seed_netlist = std::borrow::Cow::Borrowed(netlist);
        if !netlist.saves.keeps_everything() {
            seed_netlist.to_mut().saves.signals.push(SaveSignal::All);
        }
        let trace = RefCell::new(TransientIntegralTrace::default());
        let (result, _, _) = engine.run_tran_resolved_with_resume(
            &seed_netlist,
            netlist,
            TransientRunWindow {
                tstop,
                max_step,
                startup_mode: TransientStartupMode::OperatingPoint,
                dc_seed,
                integral_trace: Some(&trace),
            },
            abort,
            TransientResumePlan {
                resume: None,
                resume_validation: ResumeValidation::ExactNetlist,
                final_checkpoint_retention: FinalCheckpointRetention::Discarded,
                scheduled_checkpoint_times: &[],
            },
        )?;
        let trace = trace.into_inner();
        if trace
            .values
            .iter()
            .any(|values| values.len() != result.time.len())
        {
            return Err(SimulationError::Circuit(
                "transient integral-state capture does not cover every accepted point".into(),
            ));
        }
        Ok((result, trace))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn periodic_seed_captures_actual_integrals_despite_output_selection() {
        for rate in [1.0e3, 1.0e9] {
            let engine = Engine::default();
            let netlist = Netlist::parse(&format!(
                "Transient integral capture\nvin in 0 1\nrin in 0 1k\n\
                 bfirst out 0 v={rate}*sdt(v(in))\nrout out 0 1k\n\
                 bnested nested 0 v={rate}*{rate}*sdt(sdt(-1000*i(vin)))\nrnested nested 0 1k\n\
                 bcur sink 0 i={rate}*.001*sdt(v(in))\nrsink sink 0 1k\n.save v(out)\n.end\n"
            ))
            .unwrap();
            let period = 1.0 / rate;
            let (result, trace) = engine
                .run_tran_for_periodic_seed(&netlist, period, period / 64.0, None, &NoAbort)
                .unwrap();
            assert_eq!(trace.names.len(), 4);
            assert!(
                result
                    .voltages
                    .iter()
                    .chain(&result.branch_currents)
                    .all(|values| values.len() == result.time.len())
            );
            assert!(
                !result
                    .branch_names
                    .iter()
                    .any(|name| name.contains(":sdt:"))
            );
            assert_eq!(
                netlist.saves.signals.len(),
                1,
                "internal retention must not change the user's output selection"
            );
            for (sample, &time) in result.time.iter().enumerate() {
                for row in [0, 1, 3] {
                    assert!((trace.values[row][sample] - time).abs() * rate < 1.0e-12);
                }
                assert!(
                    (trace.values[2][sample] - 0.5 * time * time).abs() * rate * rate < 1.0e-12
                );
            }
            // The independently captured checkpoint is the accepted VM state,
            // not a waveform reconstruction or output-derived approximation.
            let (_, checkpoint) = engine
                .run_tran_checkpointed(&netlist, period, period / 64.0)
                .unwrap();
            let mut circuit = engine.build_circuit(&netlist).unwrap();
            checkpoint.inject(&mut circuit).unwrap();
            for (row, actual) in trace
                .values
                .iter()
                .zip(circuit.behavioral_sources.accepted_integrals())
            {
                assert_eq!(row.last().unwrap().to_bits(), actual.to_bits());
            }
            let mut limited = engine.config.clone();
            limited.resource_limits.max_result_values =
                Engine::transient_result_value_count(&result);
            assert!(matches!(
                Engine::new(limited).run_tran_for_periodic_seed(
                    &netlist,
                    period,
                    period / 64.0,
                    None,
                    &NoAbort
                ),
                Err(SimulationError::ResourceLimit(_))
            ));
        }
    }
}
