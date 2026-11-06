//! Elaboration of the LTspice special-function devices.
//!
//! Each instance runs as an analog behavioral model on the code-model host.
//! Inputs are differential voltage ports against the device common. Each
//! output is a current port on (output, common) whose Norton linearization
//! reads its own terminal voltage through a sensing port, so clamps and
//! output impedances stamp as conductances in every analysis. Output
//! capacitance is an ordinary capacitor to the common.

use super::*;
use crate::device::special_function::{
    Gate, GateFunction, Ota, OutputStage, Schmitt, SpecialFunctionError,
};
use crate::netlist::SpecialFunction;
use crate::xspice::{
    CmContext, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortNoiseCurrent, PortSpec,
    PortType, XspiceCheckpointSupport, XspiceInstance, XspiceSmallSignalDescriptor,
};
use std::sync::Arc;

/// The terminals of one instance, with every terminal tied to the common
/// reported as unused.
struct Terminals<'a> {
    nodes: &'a [String],
}

impl<'a> Terminals<'a> {
    fn new(element: &'a Element) -> Result<Self, SimulationError> {
        if element.nodes.len() != 8 {
            return Err(SimulationError::Circuit(format!(
                "special function {} needs eight terminals, found {}",
                element.name,
                element.nodes.len()
            )));
        }
        Ok(Self {
            nodes: &element.nodes,
        })
    }

    /// Terminal `pin` (1-based).
    fn node(&self, pin: usize) -> &'a str {
        &self.nodes[pin - 1]
    }

    fn common(&self) -> &'a str {
        self.node(8)
    }

    fn used(&self, pin: usize) -> bool {
        !self.node(pin).eq_ignore_ascii_case(self.common())
    }

    /// The voltage port of terminal `pin` against the common.
    fn sensed(&self, pin: usize) -> XspicePort {
        XspicePort::DifferentialVoltage {
            pos: self.node(pin).to_string(),
            neg: self.common().to_string(),
        }
    }

    fn driven(&self, pin: usize) -> XspicePort {
        XspicePort::DifferentialCurrent {
            pos: self.node(pin).to_string(),
            neg: self.common().to_string(),
        }
    }
}

fn device_error(element: &Element, error: SpecialFunctionError) -> SimulationError {
    SimulationError::Circuit(format!("{}: {error}", element.name))
}

/// A device's code model, the ports it connects, and the capacitance it puts
/// from each output terminal to the common.
struct Hosted {
    model: Arc<dyn CodeModel>,
    ports: Vec<XspicePort>,
    capacitances: Vec<(usize, Value)>,
}

/// Build one special-function instance into `circuit`.
pub(super) fn build(
    circuit: &mut CircuitData,
    element: &Element,
    function: SpecialFunction,
    params: &[(String, Value)],
    temperature: Value,
) -> Result<(), SimulationError> {
    let terminals = Terminals::new(element)?;
    let Hosted {
        model,
        ports,
        capacitances,
    } = match function {
        SpecialFunction::Ota => {
            let ota = Ota::from_params(params).map_err(|error| device_error(element, error))?;
            if ota.noise.incm > 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "{}: OTA common-mode input current noise (incm) is not supported",
                    element.name
                )));
            }
            for pin in [5, 6] {
                if terminals.used(pin) {
                    return Err(SimulationError::Circuit(format!(
                        "{}: OTA terminal {pin} must be tied to the common terminal",
                        element.name
                    )));
                }
            }
            let multiplier = terminals.used(3) || terminals.used(4);
            let ports = vec![
                XspicePort::DifferentialVoltage {
                    pos: terminals.node(2).to_string(),
                    neg: terminals.node(1).to_string(),
                },
                if multiplier {
                    XspicePort::DifferentialVoltage {
                        pos: terminals.node(3).to_string(),
                        neg: terminals.node(4).to_string(),
                    }
                } else {
                    XspicePort::Null
                },
                terminals.sensed(7),
                terminals.driven(7),
            ];
            Hosted {
                capacitances: vec![(7, ota.cout)],
                model: Arc::new(OtaModel { ota, multiplier }),
                ports,
            }
        }
        SpecialFunction::Buf
        | SpecialFunction::And
        | SpecialFunction::Or
        | SpecialFunction::Xor
        | SpecialFunction::Schmitt => {
            let (model, ports) = logic_device(element, &terminals, function, params)?;
            let cout = model.stage.cout;
            Hosted {
                model: Arc::new(model),
                ports,
                capacitances: vec![(6, cout), (7, cout)],
            }
        }
        other => {
            return Err(SimulationError::Circuit(format!(
                "{}: LTspice special function {} is not supported",
                element.name,
                other.ltspice_name()
            )));
        }
    };

    let connections =
        coerce_xspice_connections(circuit, model.ports(), &ports, &element.name, model.name())?;
    let mut instance =
        XspiceInstance::new(element.name.clone(), model, connections, &[], &[], &[], &[]).map_err(
            |error| xspice_creation_error(format!("Failed to create {}", element.name), error),
        )?;
    instance.set_temperature(temperature);
    assign_xspice_output_branches(circuit, &mut instance, &element.name)?;
    instance.init().map_err(|error| {
        SimulationError::Circuit(format!("Failed to initialize {}: {error}", element.name))
    })?;
    circuit.add_xspice_instance(instance);

    let common = node_id(circuit, terminals.common());
    for (pin, capacitance) in capacitances {
        if capacitance > 0.0 && terminals.used(pin) {
            let node = node_id(circuit, terminals.node(pin));
            circuit.capacitors.add_internal(
                format!("{}#cout{pin}", element.name),
                node,
                common,
                capacitance,
            );
        }
    }
    Ok(())
}

fn node_id(circuit: &mut CircuitData, name: &str) -> crate::NodeId {
    if name == "0" {
        0
    } else {
        circuit.get_or_create_node(name)
    }
}

//=============================================================================
// OTA
//=============================================================================

struct OtaModel {
    ota: Ota,
    multiplier: bool,
}

impl OtaModel {
    fn multiplier(&self, ctx: &CmContext) -> Value {
        if self.multiplier {
            ctx.input("mult")
        } else {
            1.0
        }
    }

    /// The current into the output terminal and its partials with respect
    /// to the input, multiplier and output-voltage ports.
    fn output(&self, ctx: &CmContext) -> (Value, [Value; 3]) {
        let drive = self.ota.drive(ctx.input("in"), self.multiplier(ctx));
        let v = ctx.input("vout");
        let (clamp, clamp_conductance) = self.ota.clamp(v);
        let load = self.ota.rout.map_or(0.0, |rout| 1.0 / rout);
        (
            clamp + load * v - drive.current,
            [
                -drive.d_input,
                -drive.d_multiplier,
                clamp_conductance + load,
            ],
        )
    }
}

impl CodeModel for OtaModel {
    fn name(&self) -> &str {
        "OTA"
    }

    fn description(&self) -> &str {
        "LTspice transconductance amplifier"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::DifferentialVoltage),
                PortSpec::input("mult", PortType::DifferentialVoltage).nullable(),
                PortSpec::input("vout", PortType::DifferentialVoltage),
                PortSpec::output("out", PortType::DifferentialCurrent),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn probe_writes_only_outputs(&self) -> bool {
        true
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        Ok(XspiceSmallSignalDescriptor::AffineAc)
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Stateless
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let (current, _) = self.output(ctx);
        ctx.set_output("out", current);
        Ok(())
    }

    fn port_noise_currents(&self, ctx: &CmContext) -> Vec<PortNoiseCurrent> {
        let noise = &self.ota.noise;
        let drive = self.ota.drive(ctx.input("in"), self.multiplier(ctx));
        // The input voltage noise reaches the output through the
        // transconductance at the bias; the normal-mode current noise flows
        // between the inputs.
        vec![
            PortNoiseCurrent {
                mechanism: "en",
                port: "out",
                white: (drive.d_input * noise.en).powi(2),
                corner: noise.enk,
            },
            PortNoiseCurrent {
                mechanism: "in",
                port: "in",
                white: noise.in_.powi(2),
                corner: noise.ink,
            },
        ]
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let (_, [d_input, d_multiplier, d_output]) = self.output(ctx);
        let mut partials = vec![("in".to_string(), d_input), ("vout".to_string(), d_output)];
        if self.multiplier {
            partials.push(("mult".to_string(), d_multiplier));
        }
        partials
    }
}

//=============================================================================
// Gates and Schmitt triggers
//=============================================================================

/// What a logic device decides from its inputs.
enum Decision {
    /// A gate over the used inputs among terminals 1 to 5.
    Gate { gate: Gate, used: [bool; 5] },
    /// A Schmitt trigger on `V1 − V2`.
    Schmitt(Schmitt),
}

/// Accepted-state slots of a logic device.
mod slot {
    /// The logic function's latest value (0 or 1).
    pub(super) const DESIRED: usize = 0;
    /// When that value took hold.
    pub(super) const SINCE: usize = 1;
    /// The level the outputs are driven toward (0 or 1).
    pub(super) const LEVEL: usize = 2;
    /// The `Q` and `_Q` source positions above the common.
    pub(super) const Q: usize = 3;
    pub(super) const QB: usize = 4;
    /// Each input's value, for locating threshold crossings.
    pub(super) const INPUTS: usize = 5;
    pub(super) const COUNT: usize = INPUTS + 5;
}

const INPUT_PORTS: [&str; 5] = ["in1", "in2", "in3", "in4", "in5"];

struct LogicModel {
    name: &'static str,
    decision: Decision,
    stage: OutputStage,
    /// Whether `_Q` (terminal 6) and `Q` (terminal 7) are used.
    outputs: [bool; 2],
}

impl LogicModel {
    /// The input values, and the value of the logic function given its
    /// accepted value `desired`.
    fn decide(&self, ctx: &CmContext, desired: bool) -> ([Value; 5], bool) {
        let mut inputs = [0.0; 5];
        match &self.decision {
            Decision::Gate { gate, used } => {
                let mut levels = Vec::with_capacity(5);
                for (index, port) in INPUT_PORTS.iter().enumerate() {
                    if used[index] {
                        inputs[index] = ctx.input(port);
                        levels.push(inputs[index] >= gate.reference);
                    }
                }
                (inputs, gate.function.evaluate(levels))
            }
            Decision::Schmitt(schmitt) => {
                inputs[0] = ctx.input("in1");
                (inputs, schmitt.next(desired, inputs[0]))
            }
        }
    }

    /// The level each input crosses to change the function from `desired`.
    fn thresholds(&self, desired: bool) -> [Value; 5] {
        match &self.decision {
            Decision::Gate { gate, .. } => [gate.reference; 5],
            Decision::Schmitt(schmitt) => [schmitt.trip(desired); 5],
        }
    }

    /// When the function left `desired` within the step ending at
    /// `ctx.time`: the latest crossing among the inputs that changed side.
    fn crossing(&self, ctx: &CmContext, inputs: &[Value; 5], desired: bool) -> Value {
        let (start, end) = (ctx.time_prev, ctx.time);
        let mut latest = start;
        let thresholds = self.thresholds(desired);
        for (index, input) in inputs.iter().enumerate() {
            let before = ctx.state_prev(slot::INPUTS + index) - thresholds[index];
            let now = input - thresholds[index];
            if (before >= 0.0) != (now >= 0.0) && before != now {
                let fraction = (before / (before - now)).clamp(0.0, 1.0);
                latest = latest.max(start + fraction * (end - start));
            }
        }
        latest
    }

    /// The used outputs as (sense port, drive port, is `Q`).
    fn output_ports(&self) -> impl Iterator<Item = (&'static str, &'static str, bool)> + '_ {
        [("vqb", "qb", false), ("vq", "q", true)]
            .into_iter()
            .zip(self.outputs)
            .filter(|(_, used)| *used)
            .map(|(ports, _)| ports)
    }
}

fn flag(value: bool) -> Value {
    Value::from(u8::from(value))
}

impl CodeModel for LogicModel {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        "LTspice behavioral logic"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            let mut ports: Vec<PortSpec> = INPUT_PORTS
                .iter()
                .map(|name| PortSpec::input(*name, PortType::DifferentialVoltage).nullable())
                .collect();
            for (sense, drive) in [("vqb", "qb"), ("vq", "q")] {
                ports.push(PortSpec::input(sense, PortType::DifferentialVoltage).nullable());
                ports.push(PortSpec::output(drive, PortType::DifferentialCurrent).nullable());
            }
            ports
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        Ok(XspiceSmallSignalDescriptor::AffineAc)
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(slot::COUNT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        // Every evaluation rewrites the candidate state from the accepted one,
        // so a probe may write it too: the output partials read it.
        let stage = &self.stage;
        let (level, q, qb) = if ctx.is_dc() || !ctx.is_transient() {
            let (inputs, desired) = self.decide(ctx, ctx.state_prev(slot::DESIRED) != 0.0);
            let (q, qb) = (stage.level(desired), stage.level(!desired));
            ctx.set_initial_state(slot::DESIRED, flag(desired));
            ctx.set_initial_state(slot::LEVEL, flag(desired));
            ctx.set_initial_state(slot::Q, q);
            ctx.set_initial_state(slot::QB, qb);
            ctx.set_initial_state(slot::SINCE, Value::MIN);
            for (index, input) in inputs.into_iter().enumerate() {
                ctx.set_initial_state(slot::INPUTS + index, input);
            }
            (desired, q, qb)
        } else {
            let (start, now) = (ctx.time_prev, ctx.time);
            let was_desired = ctx.state_prev(slot::DESIRED) != 0.0;
            let was_level = ctx.state_prev(slot::LEVEL) != 0.0;
            let (inputs, desired) = self.decide(ctx, was_desired);
            let since = if desired == was_desired {
                ctx.state_prev(slot::SINCE)
            } else {
                self.crossing(ctx, &inputs, was_desired)
            };
            let edge = since + stage.delay;
            let switches = desired != was_level && edge <= now;
            let level = if switches { desired } else { was_level };
            let advance = |x: Value, high: bool| {
                if switches {
                    let at_edge = stage.slew(x, !high, edge - start);
                    stage.slew(at_edge, high, now - edge)
                } else {
                    stage.slew(x, high, now - start)
                }
            };
            let q = advance(ctx.state_prev(slot::Q), level);
            let qb = advance(ctx.state_prev(slot::QB), !level);
            ctx.set_state(slot::DESIRED, flag(desired));
            ctx.set_state(slot::SINCE, since);
            ctx.set_state(slot::LEVEL, flag(level));
            ctx.set_state(slot::Q, q);
            ctx.set_state(slot::QB, qb);
            for (index, input) in inputs.into_iter().enumerate() {
                ctx.set_state(slot::INPUTS + index, input);
            }
            if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
                if desired != level {
                    ctx.request_breakpoint(edge);
                }
                if switches && let Some(tripdt) = stage.tripdt {
                    ctx.request_breakpoint(edge + tripdt);
                }
                for (x, high) in [(q, level), (qb, !level)] {
                    if let Some(ramp) = stage.ramp_duration(x, high).filter(|ramp| *ramp > 0.0) {
                        ctx.request_breakpoint(now + ramp);
                    }
                }
            }
            (level, q, qb)
        };
        for (sense, drive, true_output) in self.output_ports() {
            let high = level == true_output;
            let source = if true_output { q } else { qb };
            let current = (ctx.input(sense) - source) / stage.resistance(high);
            ctx.set_output(drive, current);
        }
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let level = ctx.state(slot::LEVEL) != 0.0;
        self.output_ports()
            .find(|(_, drive, _)| output_port.eq_ignore_ascii_case(drive))
            .map(|(sense, _, true_output)| {
                let high = level == true_output;
                vec![(sense.to_string(), 1.0 / self.stage.resistance(high))]
            })
            .unwrap_or_default()
    }
}

fn logic_device(
    element: &Element,
    terminals: &Terminals<'_>,
    function: SpecialFunction,
    params: &[(String, Value)],
) -> Result<(LogicModel, Vec<XspicePort>), SimulationError> {
    let gate_function = match function {
        SpecialFunction::Buf => Some(("BUF", GateFunction::Or)),
        SpecialFunction::Or => Some(("OR", GateFunction::Or)),
        SpecialFunction::And => Some(("AND", GateFunction::And)),
        SpecialFunction::Xor => Some(("XOR", GateFunction::Xor)),
        _ => None,
    };
    let (name, decision, stage) = if let Some((name, gate_function)) = gate_function {
        let gate = Gate::from_params(name, gate_function, params)
            .map_err(|error| device_error(element, error))?;
        let used = [1, 2, 3, 4, 5].map(|pin| terminals.used(pin));
        if !used.contains(&true) {
            return Err(SimulationError::Circuit(format!(
                "{}: {name} has no input; every input terminal is tied to the common",
                element.name
            )));
        }
        let stage = gate.stage;
        (name, Decision::Gate { gate, used }, stage)
    } else {
        let schmitt = Schmitt::from_params(params).map_err(|error| device_error(element, error))?;
        for pin in [3, 4, 5] {
            if terminals.used(pin) {
                return Err(SimulationError::Circuit(format!(
                    "{}: SCHMITT terminal {pin} must be tied to the common terminal",
                    element.name
                )));
            }
        }
        let stage = schmitt.stage;
        ("SCHMITT", Decision::Schmitt(schmitt), stage)
    };
    let outputs = [terminals.used(6), terminals.used(7)];
    let mut ports = Vec::with_capacity(9);
    match &decision {
        Decision::Gate { used, .. } => {
            for (index, used) in used.iter().enumerate() {
                ports.push(if *used {
                    terminals.sensed(index + 1)
                } else {
                    XspicePort::Null
                });
            }
        }
        Decision::Schmitt(_) => {
            ports.push(XspicePort::DifferentialVoltage {
                pos: terminals.node(1).to_string(),
                neg: terminals.node(2).to_string(),
            });
            ports.extend(std::iter::repeat_n(XspicePort::Null, 4));
        }
    }
    for (pin, used) in [(6, outputs[0]), (7, outputs[1])] {
        if used {
            ports.push(terminals.sensed(pin));
            ports.push(terminals.driven(pin));
        } else {
            ports.push(XspicePort::Null);
            ports.push(XspicePort::Null);
        }
    }
    Ok((
        LogicModel {
            name,
            decision,
            stage,
            outputs,
        },
        ports,
    ))
}
