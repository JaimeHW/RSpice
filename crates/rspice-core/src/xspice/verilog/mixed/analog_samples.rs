//! Normal analog evaluation shared by process reads and matrix assembly.
use super::super::host::{DigitalActiveExchange, DigitalActiveParticipant};
use super::*;

#[derive(Default)]
pub(super) struct PreparedAnalogStamp {
    valid: bool,
    solution: Vec<(usize, u64)>,
    inputs: Vec<u64>,
    matrix: Vec<(usize, usize, f64)>,
    rhs: Vec<(usize, f64)>,
    #[cfg(test)]
    pub(super) evaluations: u64,
}

impl Clone for PreparedAnalogStamp {
    fn clone(&self) -> Self {
        if !self.valid {
            // An accepted or rejected host has no reusable candidate. Its clone
            // need not copy scratch matrix buffers retained for the next trial.
            return Self::default();
        }
        Self {
            valid: true,
            solution: self.solution.clone(),
            inputs: self.inputs.clone(),
            matrix: self.matrix.clone(),
            rhs: self.rhs.clone(),
            #[cfg(test)]
            evaluations: self.evaluations,
        }
    }
}

impl PreparedAnalogStamp {
    pub(super) fn invalidate(&mut self) {
        self.valid = false;
    }

    pub(super) fn prepare(
        &mut self,
        analog: &mut VerilogADevice,
        inputs: &[DiscreteAnalogInput],
        solution: &[f64],
    ) -> Result<(), MixedSignalError> {
        if self.valid
            && self.inputs.len() == inputs.len()
            && self
                .solution
                .iter()
                .all(|(node, bits)| node_voltage(solution, *node).to_bits() == *bits)
            && inputs.iter().zip(&self.inputs).all(|(input, bits)| {
                analog
                    .discrete_state_value(input.variable)
                    .map(f64::to_bits)
                    == Some(*bits)
            })
        {
            return Ok(());
        }
        self.valid = false;
        self.matrix.clear();
        self.rhs.clear();
        analog
            .try_stamp(
                solution,
                |row, column, value| self.matrix.push((row, column, value)),
                |row, value| self.rhs.push((row, value)),
            )
            .map_err(analog_error)?;
        self.solution.clear();
        self.solution.extend(
            analog_solver_nodes(analog).map(|node| (node, node_voltage(solution, node).to_bits())),
        );
        self.inputs.clear();
        for input in inputs {
            let value = analog.discrete_state_value(input.variable).ok_or_else(|| {
                MixedSignalError::InvalidBridge {
                    detail: format!("analog input `{}` lost its state slot", input.name),
                }
            })?;
            self.inputs.push(value.to_bits());
        }
        self.valid = true;
        #[cfg(test)]
        {
            self.evaluations = self.evaluations.saturating_add(1);
        }
        Ok(())
    }

    pub(super) fn stamp(
        &self,
        mut matrix_add: impl FnMut(usize, usize, f64),
        mut rhs_add: impl FnMut(usize, f64),
    ) {
        debug_assert!(self.valid, "only a completed evaluation can be stamped");
        for &(row, column, value) in &self.matrix {
            matrix_add(row, column, value);
        }
        for &(row, value) in &self.rhs {
            rhs_add(row, value);
        }
    }
}

/// The same producer implementation serves local and circuit-linked processes.
pub(super) struct AnalogModelParticipant<'a> {
    pub(super) instance: &'a str,
    pub(super) analog: &'a mut MixedCell<VerilogADevice>,
    pub(super) inputs: &'a [DiscreteAnalogInput],
    pub(super) probes: &'a [AnalogProbeWiring],
    pub(super) prepared: &'a mut PreparedAnalogStamp,
    pub(super) solution: &'a [f64],
    pub(super) signals: Option<&'a [DigitalSignalId]>,
    pub(super) probe_ids: Option<&'a [DigitalAnalogProbeId]>,
}

impl AnalogModelParticipant<'_> {
    pub(super) fn sample_into(
        &mut self,
        exchange: &DigitalActiveExchange<'_>,
        requested: &[DigitalAnalogProbeId],
        samples: &mut Vec<(DigitalAnalogProbeId, f64)>,
    ) -> Result<(), DigitalRunError> {
        let probe_id = |index: usize| {
            self.probe_ids
                .map_or_else(|| DigitalAnalogProbeId::from(index), |map| map[index])
        };
        if !self.probes.iter().enumerate().any(|(index, probe)| {
            matches!(probe, AnalogProbeWiring::Variable { .. })
                && requested.contains(&probe_id(index))
        }) {
            return Ok(());
        }

        let result = (|| -> Result<(), MixedSignalError> {
            let analog = self.analog.make_mut();
            for input in self.inputs {
                let signal = self
                    .signals
                    .map_or(input.signal, |map| map[usize::from(input.signal)]);
                let value = if input.real {
                    exchange.read_real_signal(signal)
                } else {
                    exchange
                        .read_signal(signal)
                        .and_then(|value| value.to_integer(input.signed))
                        .map(|value| value as f64)
                }
                .filter(|value| value.is_finite())
                .ok_or_else(|| MixedSignalError::InvalidBridge {
                    detail: format!(
                        "analog read of discrete signal `{}` has an X, Z, or non-finite value",
                        input.name
                    ),
                })?;
                if analog
                    .discrete_state_value(input.variable)
                    .map(f64::to_bits)
                    != Some(value.to_bits())
                {
                    analog
                        .sample_discrete_state(input.variable, value)
                        .map_err(analog_error)?;
                }
            }
            self.prepared.prepare(analog, self.inputs, self.solution)?;
            // The retained assignment roots are published by this evaluation.
            // Do not call observe_variables: it may replay the model body.
            for (index, probe) in self.probes.iter().enumerate() {
                if let AnalogProbeWiring::Variable { name } = probe {
                    let value =
                        analog
                            .variable(name)
                            .ok_or_else(|| MixedSignalError::InvalidBridge {
                                detail: format!(
                                    "analog variable `{name}` has no retained evaluation slot"
                                ),
                            })?;
                    samples.push((probe_id(index), value));
                }
            }
            Ok(())
        })();
        result.map_err(|error| DigitalRunError::ExternalExecution {
            detail: format!("analog producer `{}`: {error}", self.instance),
        })
    }
}

impl DigitalActiveParticipant for AnalogModelParticipant<'_> {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        exchange.require_standalone_execution()?;
        Ok(false)
    }
    fn sample_analog(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<(), DigitalRunError> {
        let requested = exchange.analog_sample_requests();
        let mut samples = Vec::new();
        self.sample_into(exchange, &requested, &mut samples)?;
        exchange.publish_analog_variables(&samples)
    }
}

impl MixedSignalHost {
    pub(super) fn advance_digital_at_candidate(
        &mut self,
        solution: &[f64],
        probes: &[Option<f64>],
        tick: u64,
        start: bool,
    ) -> Result<(), MixedSignalError> {
        if !self.state.digital.is_view() {
            self.with_analog_participant(solution, |digital, producer| {
                digital.sample_analog_probes(probes);
                if start {
                    digital.prepare_start()?;
                }
                digital.advance_to_with(tick, producer)
            })?;
        }
        if start {
            self.trial.as_mut().unwrap().start_digital = false;
        }
        Ok(())
    }

    pub(super) fn with_analog_participant<T>(
        &mut self,
        solution: &[f64],
        run: impl FnOnce(
            &mut DigitalHost,
            &mut AnalogModelParticipant<'_>,
        ) -> Result<T, DigitalRunError>,
    ) -> Result<T, MixedSignalError> {
        let mut producer = AnalogModelParticipant {
            instance: &self.instance,
            analog: &mut self.analog,
            inputs: &self.discrete_inputs,
            probes: &self.analog_probes,
            prepared: &mut self.prepared_analog,
            solution,
            signals: None,
            probe_ids: None,
        };
        let MixedDigital::Owned(digital) = self.state.digital.make_mut() else {
            return Err(MixedSignalError::TrialProtocol {
                detail: "circuit-owned processes require the circuit analog participant".into(),
            });
        };
        run(digital, &mut producer).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn begin(host: &mut MixedSignalHost, time: f64) {
        host.begin_trial(
            time,
            time,
            if time == 0.0 {
                IntegrationCoefficients::inactive()
            } else {
                IntegrationCoefficients::backward_euler(time).unwrap()
            },
            time == 0.0,
            false,
        )
        .unwrap();
    }

    fn sample(host: &mut MixedSignalHost, voltage: f64) -> f64 {
        while host.settle_analog_bridges(&[voltage]).unwrap() {}
        let before_stamp = host.prepared_analog.evaluations;
        let mut conductance = 0.0;
        let mut rhs = 0.0;
        host.stamp(
            &[voltage],
            |row, column, value| {
                assert_eq!((row, column), (0, 0));
                conductance += value;
            },
            |row, value| {
                assert_eq!(row, 0);
                rhs += value;
            },
        )
        .unwrap();
        let transient = host.analog_inputs.time_seconds > 0.0;
        let expected_g = if transient { 0.002 } else { 0.001 };
        let expected_rhs = if transient { 0.00225 } else { 0.0 };
        assert!((conductance - expected_g).abs() < 1e-14, "G={conductance}");
        assert!((rhs - expected_rhs).abs() < 1e-14, "RHS={rhs}");
        assert_eq!(
            host.prepared_analog.evaluations, before_stamp,
            "matrix assembly must reuse the normal producer evaluation"
        );
        let captured = host.state.digital.signal("captured").unwrap();
        host.state.digital.read_real(captured).unwrap()
    }

    #[test]
    fn analog_variable_producer_reuses_stamps_and_discards_rejected_candidates() {
        let source = r#"
`timescale 1ns/1ns
module sampled(p); inout p; electrical p;
reg [7:0] gain; real measured,captured;
analog begin measured=gain*V(p); I(p)<+V(p)/1000+ddt(1e-12*V(p)); end
initial begin gain=2; captured=measured; #1 gain=3; captured=measured; end
endmodule
"#;
        let mut host =
            MixedSignalHost::compile(source, None, "sampler", &[1], SchedulerLimits::default())
                .unwrap();
        begin(&mut host, 0.0);
        assert_eq!(sample(&mut host, 1.5), 3.0);
        assert_eq!(host.prepared_analog.evaluations, 1);
        host.stamp(&[1.5], |_, _, _| {}, |_, _| {}).unwrap();
        assert_eq!(host.prepared_analog.evaluations, 1);
        host.reject_trial().unwrap();
        assert!(!host.prepared_analog.valid);

        begin(&mut host, 0.0);
        assert_eq!(sample(&mut host, 2.25), 4.5);
        assert_eq!(host.prepared_analog.evaluations, 2);
        host.accept_trial().unwrap();
        let accepted = host.checkpoint().unwrap();
        begin(&mut host, 1e-9);
        assert_eq!(sample(&mut host, 2.25), 6.75);
        host.reject_trial().unwrap();
        assert!(!host.prepared_analog.valid);
        assert_eq!(
            host.analog.checkpoint_state().unwrap(),
            accepted.analog_checkpoint
        );

        begin(&mut host, 1e-9);
        assert_eq!(sample(&mut host, 4.0), 12.0);
        host.accept_trial().unwrap();
        host.restore(&accepted).unwrap();
        begin(&mut host, 1e-9);
        assert_eq!(sample(&mut host, 2.0), 6.0);
        host.accept_trial().unwrap();

        let accepted_transient = host.checkpoint().unwrap();
        for voltage in [8.0, 3.0] {
            // Unchanged companion coefficients must not leave a rejected ddt
            // candidate live merely because solver-input restoration is a no-op.
            host.begin_trial(
                2e-9,
                1e-9,
                IntegrationCoefficients::backward_euler(1e-9).unwrap(),
                false,
                false,
            )
            .unwrap();
            let mut rhs = 0.0;
            host.stamp(&[voltage], |_, _, _| {}, |_, value| rhs += value)
                .unwrap();
            assert!((rhs - 0.002).abs() < 1e-14);
            host.reject_trial().unwrap();
            assert_eq!(
                host.analog.checkpoint_state().unwrap(),
                accepted_transient.analog_checkpoint
            );
        }
    }
}
