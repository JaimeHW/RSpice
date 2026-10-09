//! Portable accepted state of an enrolled analog/bridge participant. The
//! coordinator owns the HDL values; views are rebuilt from it during restore.
use super::*;
use crate::xspice::verilog::store::checkpoint::fingerprint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub(crate) struct ParticipantCheckpointLimits {
    pub max_analog_words: usize,
    pub max_items: usize,
}
impl Default for ParticipantCheckpointLimits {
    fn default() -> Self {
        Self {
            max_analog_words: 16_777_216,
            max_items: 1_048_576,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ParticipantCheckpoint {
    version: u32,
    topology: [u8; 32],
    // Instance, model, source digest and resolved parameter/state shape.
    analog_identity: [String; 4],
    analog: Vec<u64>,
    inputs: SolverInputsImage,
    time: u64,
    tick: u64,
    adc_voltages: Vec<u64>,
    adc_decisions: Vec<Option<u8>>,
    adc_transitions: Vec<Option<u64>>,
    probes: Vec<Option<u64>>,
    adc_history: Vec<HistoryImage>,
    dac_history: Vec<HistoryImage>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SolverInputsImage {
    analysis: u8,
    phase: u8,
    initial: bool,
    final_step: bool,
    time: u64,
    timestep: u64,
    integration: [u64; 5],
    state_integration: [u64; 5],
}

fn coefficients_image(value: IntegrationCoefficients) -> [u64; 5] {
    [
        u64::from(value.active),
        value.derivative_scale.to_bits(),
        value.previous_value_scale.to_bits(),
        value.older_value_scale.to_bits(),
        value.previous_derivative_scale.to_bits(),
    ]
}
fn coefficients(words: [u64; 5]) -> Result<IntegrationCoefficients, String> {
    if words[0] > 1 {
        return Err("invalid checkpoint integration activity flag".into());
    }
    let value = IntegrationCoefficients {
        active: words[0] == 1,
        derivative_scale: f64::from_bits(words[1]),
        previous_value_scale: f64::from_bits(words[2]),
        older_value_scale: f64::from_bits(words[3]),
        previous_derivative_scale: f64::from_bits(words[4]),
    };
    value.validate().map_err(|e| e.to_string())?;
    Ok(value)
}

impl SolverInputsImage {
    fn capture(value: AnalogSolverInputs) -> Self {
        Self {
            analysis: value.analysis,
            phase: value.phase as u8,
            initial: value.initial_step,
            final_step: value.final_step,
            time: value.time_seconds.to_bits(),
            timestep: value.timestep_seconds.to_bits(),
            integration: coefficients_image(value.integration),
            state_integration: coefficients_image(value.state_integration),
        }
    }
    fn restore(&self, time: f64) -> Result<AnalogSolverInputs, String> {
        // Other physical analyses have separate initialization/linearization
        // contracts. This image represents an accepted transient timepoint.
        if self.analysis != 2 || self.phase != 0 || f64::from_bits(self.time) != time {
            return Err("participant checkpoint is not an accepted transient point".into());
        }
        let dt = f64::from_bits(self.timestep);
        if !dt.is_finite() || dt < 0.0 {
            return Err("invalid participant checkpoint timestep".into());
        }
        let integration = coefficients(self.integration)?;
        let state_integration = coefficients(self.state_integration)?;
        if integration.active != state_integration.active {
            return Err("checkpoint integration rules disagree on activity".into());
        }
        Ok(AnalogSolverInputs {
            analysis: 2,
            phase: rspice_veriloga_runtime::AnalogAnalysisPhase::Point,
            initial_step: self.initial,
            final_step: self.final_step,
            time_seconds: f64::from_bits(self.time),
            timestep_seconds: dt,
            integration,
            state_integration,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HistoryImage {
    run: u32,
    recent: u16,
    filled: u32,
}
impl HistoryImage {
    fn capture(value: &BoundaryNetHistory) -> Self {
        Self {
            run: value.run,
            recent: value.recent,
            filled: value.filled,
        }
    }
    fn restore(self) -> Result<BoundaryNetHistory, String> {
        if self.filled == 0
            || self.filled > BOUNDARY_VALUE_HISTORY
            || self.run > MAX_CONSECUTIVE_BOUNDARY_FLIPS
            || (self.filled < BOUNDARY_VALUE_HISTORY
                && u32::from(self.recent) >> (self.filled * 2) != 0)
        {
            return Err("invalid checkpoint boundary history".into());
        }
        Ok(BoundaryNetHistory {
            run: self.run,
            recent: self.recent,
            filled: self.filled,
        })
    }
}

fn decision(value: u8) -> Result<FourStateBit, String> {
    match value {
        0 => Ok(FourStateBit::Zero),
        1 => Ok(FourStateBit::One),
        2 => Ok(FourStateBit::Unknown),
        _ => Err("invalid checkpoint ADC decision".into()),
    }
}

impl MixedSignalHost {
    pub(crate) fn validate_participant_checkpoint_boundary(
        &self,
        time: f64,
    ) -> Result<(), String> {
        self.require_idle("capture a circuit participant")
            .map_err(|e| e.to_string())?;
        if !self.state.started || self.state.accepted_time != time {
            return Err("participant and circuit accepted times differ".into());
        }
        Ok(())
    }

    fn participant_topology(&self) -> Result<[u8; 32], String> {
        let adc: Vec<_> = self
            .state
            .bridges
            .adc
            .iter()
            .map(|b| {
                (
                    b.signal,
                    b.bit,
                    &b.signal_name,
                    (b.positive, b.negative),
                    (b.low.to_bits(), b.high.to_bits()),
                    b.root_only,
                    match b.threshold_behavior {
                        crate::xspice::AnalogThresholdBehavior::Hysteresis => 0u8,
                        crate::xspice::AnalogThresholdBehavior::UnknownBand => 1,
                    },
                    &b.converter_input,
                )
            })
            .collect();
        let dac: Vec<_> = self
            .state
            .bridges
            .dac
            .iter()
            .map(|b| {
                (
                    b.signal,
                    b.bit,
                    &b.signal_name,
                    (b.positive, b.negative),
                    [b.low.to_bits(), b.high.to_bits(), b.resistance.to_bits()],
                )
            })
            .collect();
        let probes: Vec<_> = self
            .analog_probes
            .iter()
            .map(|probe| match probe {
                AnalogProbeWiring::Solution {
                    positive,
                    negative,
                    scale,
                } => (0u8, (*positive, *negative), scale.to_bits(), None),
                AnalogProbeWiring::Variable { name, retained } => {
                    (1u8, (0, 0), 0, Some((name, *retained)))
                }
            })
            .collect();
        let inputs: Vec<_> = self
            .discrete_inputs
            .iter()
            .map(|i| {
                (
                    i.signal,
                    i.variable,
                    i.validity,
                    i.selection,
                    i.signed,
                    i.real,
                    &i.name,
                )
            })
            .collect();
        let buses: Vec<_> = self
            .boundary_buses
            .iter()
            .map(|b| (&b.name, b.msb, b.lsb, &b.members))
            .collect();
        let ports: Vec<_> = self
            .event_ports
            .iter()
            .map(|p| {
                (
            p.signal, p.bit, p.node, match p.direction {
                rspice_veriloga::canonical_ir::digital_link::DigitalLinkDirection::Input => 0u8,
                rspice_veriloga::canonical_ir::digital_link::DigitalLinkDirection::Output => 1,
                rspice_veriloga::canonical_ir::digital_link::DigitalLinkDirection::Inout => 2,
            }, p.trace_node,
        )
            })
            .collect();
        let terminals: Vec<_> = (0..self.analog.num_terminals())
            .map(|i| self.analog.node_for_terminal(i))
            .collect();
        fingerprint(&(
            1u32,
            &self.instance,
            &self.source_digest,
            self.state.digital.plan().content_identity,
            self.resolution.seconds_per_tick().to_bits(),
            self.analog_step_floor.to_bits(),
            (adc, dac, probes, inputs),
            (buses, ports, &self.event_nodes, terminals),
        ))
    }

    fn participant_budget(
        &self,
        image: &ParticipantCheckpoint,
        limits: ParticipantCheckpointLimits,
    ) -> Result<(), String> {
        if image.analog.len() > limits.max_analog_words {
            return Err("analog checkpoint word limit exceeded".into());
        }
        let sizes = [
            image.adc_voltages.len(),
            image.adc_decisions.len(),
            image.adc_transitions.len(),
            image.probes.len(),
            image.adc_history.len(),
            image.dac_history.len(),
        ];
        let adc = self.state.bridges.adc.len();
        if sizes
            != [
                adc,
                adc,
                adc,
                self.analog_probes.len(),
                adc,
                self.state.bridges.dac.len(),
            ]
        {
            return Err("participant checkpoint boundary dimensions differ".into());
        }
        sizes
            .into_iter()
            .try_fold(0usize, |n, size| {
                n.checked_add(size).filter(|n| *n <= limits.max_items)
            })
            .ok_or("participant checkpoint item limit exceeded")?;
        Ok(())
    }

    pub(crate) fn participant_checkpoint(
        &self,
        limits: ParticipantCheckpointLimits,
    ) -> Result<ParticipantCheckpoint, String> {
        self.require_idle("capture a circuit participant")
            .map_err(|e| e.to_string())?;
        if !self.state.digital.is_view()
            || self.standalone.is_some()
            || !self.digital_started
            || !self.state.started
        {
            return Err("participant capture requires an accepted circuit-owned module".into());
        }
        let analog = self.analog.checkpoint_state().map_err(|e| e.to_string())?;
        let image = ParticipantCheckpoint {
            version: 1,
            topology: self.participant_topology()?,
            analog_identity: [
                analog.instance_name.to_string(),
                analog.model_name.to_string(),
                analog.source_digest.to_string(),
                analog.shape_identity.to_string(),
            ],
            analog: analog.to_words(),
            inputs: SolverInputsImage::capture(self.analog_inputs),
            time: self.state.accepted_time.to_bits(),
            tick: self.state.accepted_tick,
            adc_voltages: self
                .state
                .accepted_adc_voltages
                .iter()
                .map(|v| v.to_bits())
                .collect(),
            adc_decisions: self
                .state
                .accepted_adc_decisions
                .iter()
                .map(|v| v.map(|v| BoundaryNetHistory::code(v) as u8))
                .collect(),
            adc_transitions: self
                .state
                .accepted_adc_transition_times
                .iter()
                .map(|v| v.map(f64::to_bits))
                .collect(),
            probes: self
                .state
                .accepted_probe_values
                .iter()
                .map(|v| v.map(f64::to_bits))
                .collect(),
            adc_history: self
                .state
                .adc_history
                .iter()
                .map(HistoryImage::capture)
                .collect(),
            dac_history: self
                .state
                .dac_history
                .iter()
                .map(HistoryImage::capture)
                .collect(),
        };
        self.participant_budget(&image, limits)?;
        image.inputs.restore(self.state.accepted_time)?;
        Ok(image)
    }

    /// Validate and construct without modifying either the receiving participant
    /// or the coordinator. The outer circuit must also authenticate its complete
    /// solver topology, temperature/configuration and source include identities.
    pub(crate) fn restored_participant_checkpoint(
        &self,
        image: &ParticipantCheckpoint,
        coordinator: &MixedDigitalCoordinator,
        limits: ParticipantCheckpointLimits,
    ) -> Result<Self, String> {
        self.require_idle("restore a circuit participant")
            .map_err(|e| e.to_string())?;
        if !self.state.digital.is_view() || self.standalone.is_some() {
            return Err("participant restore requires a circuit-owned module".into());
        }
        if image.version != 1 || image.topology != self.participant_topology()? {
            return Err("participant checkpoint source, topology or time policy differs".into());
        }
        self.participant_budget(image, limits)?;
        let time = f64::from_bits(image.time);
        let tick =
            hdl_tick(time, |at| at.floor_tick(self.resolution)).map_err(|e| e.to_string())?;
        if tick != image.tick {
            return Err("participant checkpoint accepted tick differs".into());
        }
        let inputs = image.inputs.restore(time)?;
        let [instance, model, source, shape] = &image.analog_identity;
        let analog = VerilogADeviceCheckpoint::from_words(
            instance.as_str().into(),
            model.as_str().into(),
            source.as_str().into(),
            shape.as_str().into(),
            &image.analog,
        )?;
        self.analog
            .validate_checkpoint_state(&analog)
            .map_err(|e| e.to_string())?;
        if analog.accepted.time != time {
            return Err("analog and participant accepted times differ".into());
        }
        let mut restored = self.clone();
        restored.state.digital = MixedCell::new(coordinator.checkpoint_view(
            &self.instance,
            Arc::clone(self.state.digital.plan()),
            time,
        )?);
        restored.state.accepted_adc_voltages = image
            .adc_voltages
            .iter()
            .map(|v| f64::from_bits(*v))
            .collect();
        if restored
            .state
            .accepted_adc_voltages
            .iter()
            .any(|v| !v.is_finite())
        {
            return Err("non-finite accepted ADC voltage".into());
        }
        restored.state.accepted_adc_decisions = image
            .adc_decisions
            .iter()
            .map(|v| v.map(decision).transpose())
            .collect::<Result<_, _>>()?;
        restored.state.accepted_adc_transition_times = image
            .adc_transitions
            .iter()
            .map(|v| v.map(f64::from_bits))
            .collect();
        if restored
            .state
            .accepted_adc_transition_times
            .iter()
            .flatten()
            .any(|v| !v.is_finite() || *v < 0.0 || *v > time)
        {
            return Err("invalid accepted ADC transition time".into());
        }
        restored.state.accepted_probe_values =
            image.probes.iter().map(|v| v.map(f64::from_bits)).collect();
        // Variable probes may be unavailable until demanded by a process. A
        // present sample must still be a finite accepted analog quantity.
        if restored
            .state
            .accepted_probe_values
            .iter()
            .flatten()
            .any(|v| !v.is_finite())
        {
            return Err("non-finite accepted analog probe".into());
        }
        restored.state.adc_history = image
            .adc_history
            .iter()
            .map(|v| v.restore())
            .collect::<Result<_, _>>()?;
        restored.state.dac_history = image
            .dac_history
            .iter()
            .map(|v| v.restore())
            .collect::<Result<_, _>>()?;
        for (index, (bridge, history)) in restored
            .state
            .bridges
            .adc
            .iter()
            .zip(&restored.state.adc_history)
            .enumerate()
        {
            let bit = if bridge.root_only {
                restored.state.accepted_adc_decisions[index].unwrap_or(FourStateBit::Unknown)
            } else {
                restored
                    .state
                    .digital
                    .read(bridge.driven_signal())
                    .ok_or("missing ADC signal")?
                    .bit(bridge.bit)
            };
            if history.recent & 3 != BoundaryNetHistory::code(bit) {
                return Err(
                    "ADC checkpoint history differs from restored decision or signal".into(),
                );
            }
        }
        for (bridge, history) in restored
            .state
            .bridges
            .dac
            .iter()
            .zip(&restored.state.dac_history)
        {
            let bit = restored
                .state
                .digital
                .read(bridge.signal)
                .ok_or("missing DAC signal")?
                .bit(bridge.bit);
            if history.recent & 3 != BoundaryNetHistory::code(bit) {
                return Err("DAC checkpoint history differs from restored signal".into());
            }
        }
        restored
            .analog
            .make_mut()
            .apply_validated_checkpoint_state(&analog);
        restored
            .apply_analog_inputs(inputs)
            .map_err(|e| e.to_string())?;
        restored.analog_inputs = inputs;
        restored.state.accepted_time = time;
        restored.state.accepted_tick = tick;
        restored.state.started = true;
        restored.state.initial_digital = None;
        restored.digital_started = true;
        restored.scratch = TrialScratch::default();
        Ok(restored)
    }
}

#[cfg(test)]
mod tests;
