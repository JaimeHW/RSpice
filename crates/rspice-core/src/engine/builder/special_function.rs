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
    Gate, GateFunction, Ota, OutputStage, Schmitt, Sequential, SequentialFunction,
    SpecialFunctionError,
};
use crate::expr::{
    CompiledExpr, Expr, compile, function_uses_implicit_time, parse_expression_strict,
};
use crate::netlist::SpecialFunction;
use crate::xspice::{
    CmContext, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortNoiseCurrent, PortSpec,
    PortType, XspiceCheckpointSupport, XspiceInstance, XspiceSmallSignalDescriptor,
};
use std::sync::Arc;

mod mixed;

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

/// Build one special-function instance into `circuit`. `params` are its
/// numeric parameters and the noise densities the flattener kept as
/// expressions of frequency and the operating point.
pub(super) fn build(
    circuit: &mut CircuitData,
    element: &Element,
    function: SpecialFunction,
    params: &[(String, Value)],
    densities: &[(String, String)],
    temperature: Value,
) -> Result<(), SimulationError> {
    let terminals = Terminals::new(element)?;
    if function != SpecialFunction::Ota
        && let Some((name, _)) = densities.first()
    {
        return Err(SimulationError::Circuit(format!(
            "{}: parameter {name} must be a constant",
            element.name
        )));
    }
    // The node an OTA's common-mode input current noise is injected into.
    let mut common_mode_node = None;
    let Hosted {
        model,
        ports,
        capacitances,
    } = match function {
        SpecialFunction::Ota => {
            let ota = Ota::from_params(params).map_err(|error| device_error(element, error))?;
            let mut en = None;
            let mut in_ = None;
            let mut incm = None;
            for (name, expression) in densities {
                let slot = match name.to_ascii_uppercase().as_str() {
                    "EN" => &mut en,
                    "IN" => &mut in_,
                    "INCM" => &mut incm,
                    _ => {
                        return Err(SimulationError::Circuit(format!(
                            "{}: parameter {name} must be a constant",
                            element.name
                        )));
                    }
                };
                *slot = Some(noise_density(element, name, expression)?);
            }
            if incm.is_some() || ota.noise.incm > 0.0 {
                common_mode_node = Some(format!("{}#ICM", element.name));
            }
            for pin in [5, 6] {
                if terminals.used(pin) {
                    // LTspice gives these terminals no documented use, and no
                    // run settles what terminal 6 does beyond drawing a fixed
                    // current.
                    return Err(SimulationError::Circuit(format!(
                        "{}: OTA terminal {pin} is connected to node {}; RSpice requires \
                         OTA terminals 5 and 6 on the common node (terminal 8, {})",
                        element.name,
                        terminals.node(pin),
                        terminals.common()
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
                match &common_mode_node {
                    Some(node) => XspicePort::DifferentialCurrent {
                        pos: node.clone(),
                        neg: terminals.common().to_string(),
                    },
                    None => XspicePort::Null,
                },
            ];
            Hosted {
                capacitances: vec![(7, ota.cout)],
                model: Arc::new(OtaModel {
                    ota,
                    multiplier,
                    en,
                    in_,
                    incm,
                    common_mode: common_mode_node.is_some(),
                }),
                ports,
            }
        }
        SpecialFunction::Buf
        | SpecialFunction::And
        | SpecialFunction::Or
        | SpecialFunction::Xor
        | SpecialFunction::Schmitt
        | SpecialFunction::DFlop
        | SpecialFunction::SrFlop
        | SpecialFunction::Counter => {
            let (model, ports) = logic_device(element, &terminals, function, params)?;
            let cout = model.stage.cout;
            Hosted {
                model: Arc::new(model),
                ports,
                capacitances: vec![(6, cout), (7, cout)],
            }
        }
        SpecialFunction::PhaseDet => mixed::phase_detector(element, &terminals, params)?,
        SpecialFunction::SampleHold => mixed::sample_hold(element, &terminals, params)?,
        SpecialFunction::Modulator => mixed::modulator(element, &terminals, params)?,
        SpecialFunction::Varistor => mixed::varistor(element, &terminals, params)?,
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
    if let Some(node) = common_mode_node {
        // LTspice's incm is one noise current driven into both inputs from
        // the common. It is injected into a private node held by a quiet
        // 1 ohm resistor, whose voltage drives a 1 S current into each input.
        let node = node_id(circuit, &node);
        circuit.resistors.add_device_series(
            &element.name,
            format!("{}#ICM", element.name),
            node,
            common,
            1.0,
        );
        circuit.resistors.set_last_noisy(false);
        for pin in [1, 2] {
            let input = node_id(circuit, terminals.node(pin));
            circuit.vccs.add(
                format!("{}#ICM{pin}", element.name),
                VoltageControlledNodes {
                    node_pos: common,
                    node_neg: input,
                    ctrl_pos: node,
                    ctrl_neg: common,
                },
                1.0,
            );
        }
    }
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

/// A noise density written as an expression: compiled, and held to
/// frequency, temperature and node voltages, which the noise analysis reads
/// at the operating point.
fn noise_density(
    element: &Element,
    name: &str,
    expression: &str,
) -> Result<CompiledExpr, SimulationError> {
    let fail = |message: String| {
        SimulationError::Circuit(format!(
            "{}: noise density {name} '{expression}' {message}",
            element.name
        ))
    };
    let parsed = parse_expression_strict(expression).map_err(|error| fail(error.to_string()))?;
    if reads_time(&parsed) {
        return Err(fail("reads the time".to_string()));
    }
    let compiled = compile(&parsed);
    if !compiled.branch_map.is_empty() {
        return Err(fail("reads a branch current".to_string()));
    }
    Ok(compiled)
}

fn reads_time(expression: &Expr) -> bool {
    match expression {
        Expr::Time => true,
        Expr::Unary { operand, .. } => reads_time(operand),
        Expr::Binary { left, right, .. } => reads_time(left) || reads_time(right),
        Expr::Function { func, args } => {
            function_uses_implicit_time(*func) || args.iter().any(reads_time)
        }
        Expr::LookupTable { input, .. } => reads_time(input),
        _ => false,
    }
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
    /// Input voltage, normal-mode and common-mode current noise written as
    /// expressions, which replace the constant densities.
    en: Option<CompiledExpr>,
    in_: Option<CompiledExpr>,
    incm: Option<CompiledExpr>,
    /// Whether the `icm` port reaches the common-mode noise node.
    common_mode: bool,
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
                PortSpec::output("icm", PortType::DifferentialCurrent).nullable(),
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
        if self.common_mode {
            // The port only carries the common-mode noise current.
            ctx.set_output("icm", 0.0);
        }
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
                white: match self.en {
                    Some(_) => drive.d_input.powi(2),
                    None => (drive.d_input * noise.en).powi(2),
                },
                corner: noise.enk,
                spectrum: self.en.clone(),
            },
            PortNoiseCurrent {
                mechanism: "in",
                port: "in",
                white: match self.in_ {
                    Some(_) => 1.0,
                    None => noise.in_.powi(2),
                },
                corner: noise.ink,
                spectrum: self.in_.clone(),
            },
            PortNoiseCurrent {
                mechanism: "icm",
                port: "icm",
                white: match self.incm {
                    Some(_) => 1.0,
                    None => noise.incm.powi(2),
                },
                corner: noise.incmk,
                spectrum: self.incm.clone(),
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
// Gates, Schmitt triggers, flip-flops and the counter
//=============================================================================

/// What a logic device decides from its inputs.
enum Decision {
    /// A gate over the used inputs among terminals 1 to 5.
    Gate { gate: Gate, used: [bool; 5] },
    /// A Schmitt trigger on `V1 − V2`.
    Schmitt(Schmitt),
    /// A flip-flop or counter over the used inputs among terminals 1 to 5.
    Sequential { device: Sequential, used: [bool; 5] },
}

/// Accepted-state slots of a logic device. An output word holds the level
/// of `Q` in bit 0 and that of `_Q` in bit 1.
mod slot {
    /// The output word the device's inputs call for.
    pub(super) const DESIRED: usize = 0;
    /// When that word took hold.
    pub(super) const SINCE: usize = 1;
    /// The output word the outputs are driven toward.
    pub(super) const LEVEL: usize = 2;
    /// The `Q` and `_Q` source positions above the common.
    pub(super) const Q: usize = 3;
    pub(super) const QB: usize = 4;
    /// A flip-flop's Q or a counter's count.
    pub(super) const STATE: usize = 5;
    /// Each input's value, for edges and threshold crossings.
    pub(super) const INPUTS: usize = 6;
    pub(super) const COUNT: usize = INPUTS + 5;
}

const INPUT_PORTS: [&str; 5] = ["in1", "in2", "in3", "in4", "in5"];

fn word(q: bool, qb: bool) -> Value {
    Value::from(u8::from(q) | (u8::from(qb) << 1))
}

/// The levels of `Q` and `_Q` in an output word.
fn bits(word: Value) -> (bool, bool) {
    (word % 2.0 >= 1.0, word >= 2.0)
}

struct LogicModel {
    name: &'static str,
    decision: Decision,
    stage: OutputStage,
    /// Whether `_Q` (terminal 6) and `Q` (terminal 7) are used.
    outputs: [bool; 2],
}

impl LogicModel {
    /// The input values, the output word and the sequential state, given the
    /// accepted word and state. A flip-flop or counter keeps its initial
    /// state through the operating point.
    fn decide(
        &self,
        ctx: &CmContext,
        desired: Value,
        state: Value,
        operating_point: bool,
    ) -> ([Value; 5], Value, Value) {
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
                let high = gate.function.evaluate(levels);
                (inputs, word(high, !high), 0.0)
            }
            Decision::Schmitt(schmitt) => {
                inputs[0] = ctx.input("in1");
                let high = schmitt.next(bits(desired).0, inputs[0]);
                (inputs, word(high, !high), 0.0)
            }
            Decision::Sequential { device, used } => {
                let (mut before, mut now) = ([None; 5], [None; 5]);
                for (index, port) in INPUT_PORTS.iter().enumerate() {
                    if used[index] {
                        inputs[index] = ctx.input(port);
                        let previous = ctx.state_prev(slot::INPUTS + index);
                        before[index] = Some(previous >= device.reference);
                        now[index] = Some(inputs[index] >= device.reference);
                    }
                }
                let state = if operating_point {
                    u32::from(self.stage.initial == Some(true))
                } else {
                    device.next(state as u32, before, now)
                };
                let (q, qb) = device.outputs(state);
                (inputs, word(q, qb), Value::from(state))
            }
        }
    }

    /// The input values the first time step reads as the operating point's.
    /// A counter's clock reads low there, so a clock already high counts.
    fn operating_point_inputs(&self, mut inputs: [Value; 5]) -> [Value; 5] {
        if let Decision::Sequential { device, used } = &self.decision
            && matches!(device.function, SequentialFunction::Counter { .. })
            && used[0]
        {
            inputs[0] = Value::MIN;
        }
        inputs
    }

    /// The level each input crosses to change the output word from `desired`.
    fn thresholds(&self, desired: Value) -> [Value; 5] {
        match &self.decision {
            Decision::Gate { gate, .. } => [gate.reference; 5],
            Decision::Schmitt(schmitt) => [schmitt.trip(bits(desired).0); 5],
            Decision::Sequential { device, .. } => [device.reference; 5],
        }
    }

    /// When the output word left `desired` within the step ending at
    /// `ctx.time`: the latest crossing among the inputs that changed side.
    fn crossing(&self, ctx: &CmContext, inputs: &[Value; 5], desired: Value) -> Value {
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
        let (level, q, qb) = if ctx.is_dc_operating_point() || !ctx.is_transient() {
            let (inputs, desired, state) = self.decide(
                ctx,
                ctx.state_prev(slot::DESIRED),
                ctx.state_prev(slot::STATE),
                true,
            );
            // A gate or Schmitt trigger given `IC` drives it here, and its
            // function takes over `Td` after time zero.
            let held = match (&self.decision, stage.initial) {
                (Decision::Sequential { .. }, _) | (_, None) => None,
                (_, Some(high)) => Some(word(high, !high)),
            };
            let level = held.unwrap_or(desired);
            let (high, high_b) = bits(level);
            let (q, qb) = (stage.level(high), stage.level(high_b));
            ctx.set_initial_state(slot::DESIRED, desired);
            ctx.set_initial_state(slot::LEVEL, level);
            ctx.set_initial_state(slot::Q, q);
            ctx.set_initial_state(slot::QB, qb);
            ctx.set_initial_state(slot::STATE, state);
            let since = if held.is_some() { 0.0 } else { Value::MIN };
            ctx.set_initial_state(slot::SINCE, since);
            for (index, input) in self.operating_point_inputs(inputs).into_iter().enumerate() {
                ctx.set_initial_state(slot::INPUTS + index, input);
            }
            (level, q, qb)
        } else {
            let (start, now) = (ctx.time_prev, ctx.time);
            let was_desired = ctx.state_prev(slot::DESIRED);
            let was_level = ctx.state_prev(slot::LEVEL);
            let (inputs, desired, state) =
                self.decide(ctx, was_desired, ctx.state_prev(slot::STATE), false);
            let since = if desired == was_desired {
                ctx.state_prev(slot::SINCE)
            } else {
                self.crossing(ctx, &inputs, was_desired)
            };
            let edge = since + stage.delay;
            let switches = desired != was_level && edge <= now;
            let level = if switches { desired } else { was_level };
            let advance = |x: Value, from: bool, to: bool| {
                if switches {
                    stage.slew(stage.slew(x, from, edge - start), to, now - edge)
                } else {
                    stage.slew(x, from, now - start)
                }
            };
            let ((from, from_b), (to, to_b)) = (bits(was_level), bits(level));
            let q = advance(ctx.state_prev(slot::Q), from, to);
            let qb = advance(ctx.state_prev(slot::QB), from_b, to_b);
            ctx.set_state(slot::DESIRED, desired);
            ctx.set_state(slot::SINCE, since);
            ctx.set_state(slot::LEVEL, level);
            ctx.set_state(slot::Q, q);
            ctx.set_state(slot::QB, qb);
            ctx.set_state(slot::STATE, state);
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
                for (x, high) in [(q, to), (qb, to_b)] {
                    if let Some(ramp) = stage.ramp_duration(x, high).filter(|ramp| *ramp > 0.0) {
                        ctx.request_breakpoint(now + ramp);
                    }
                }
            }
            (level, q, qb)
        };
        let (high, high_b) = bits(level);
        for (sense, drive, true_output) in self.output_ports() {
            let (high, source) = if true_output { (high, q) } else { (high_b, qb) };
            let current = (ctx.input(sense) - source) / stage.resistance(high);
            ctx.set_output(drive, current);
        }
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let (high, high_b) = bits(ctx.state(slot::LEVEL));
        self.output_ports()
            .find(|(_, drive, _)| output_port.eq_ignore_ascii_case(drive))
            .map(|(sense, _, true_output)| {
                let high = if true_output { high } else { high_b };
                vec![(sense.to_string(), 1.0 / self.stage.resistance(high))]
            })
            .unwrap_or_default()
    }
}

/// The terminals among 1 to 5 a sequential device reads; it ignores the
/// others.
fn sequential_inputs(function: SpecialFunction) -> &'static [usize] {
    match function {
        SpecialFunction::DFlop => &[1, 3, 4, 5],
        _ => &[1, 2],
    }
}

fn logic_device(
    element: &Element,
    terminals: &Terminals<'_>,
    function: SpecialFunction,
    params: &[(String, Value)],
) -> Result<(LogicModel, Vec<XspicePort>), SimulationError> {
    let name = function.ltspice_name();
    let error = |error| device_error(element, error);
    let gate_function = match function {
        SpecialFunction::Buf | SpecialFunction::Or => Some(GateFunction::Or),
        SpecialFunction::And => Some(GateFunction::And),
        SpecialFunction::Xor => Some(GateFunction::Xor),
        _ => None,
    };
    let (decision, stage) = if let Some(gate_function) = gate_function {
        let gate = Gate::from_params(name, gate_function, params).map_err(error)?;
        let used = [1, 2, 3, 4, 5].map(|pin| terminals.used(pin));
        if !used.contains(&true) {
            return Err(SimulationError::Circuit(format!(
                "{}: {name} has no input; every input terminal is tied to the common",
                element.name
            )));
        }
        let stage = gate.stage;
        (Decision::Gate { gate, used }, stage)
    } else if function == SpecialFunction::Schmitt {
        let schmitt = Schmitt::from_params(params).map_err(error)?;
        for pin in [3, 4, 5] {
            if terminals.used(pin) {
                return Err(SimulationError::Circuit(format!(
                    "{}: SCHMITT terminal {pin} must be tied to the common terminal",
                    element.name
                )));
            }
        }
        let stage = schmitt.stage;
        (Decision::Schmitt(schmitt), stage)
    } else {
        let device = Sequential::from_params(name, params).map_err(error)?;
        let read = sequential_inputs(function);
        let used = [1, 2, 3, 4, 5].map(|pin| read.contains(&pin) && terminals.used(pin));
        let stage = device.stage;
        (Decision::Sequential { device, used }, stage)
    };
    let outputs = [terminals.used(6), terminals.used(7)];
    let mut ports = Vec::with_capacity(9);
    match &decision {
        Decision::Gate { used, .. } | Decision::Sequential { used, .. } => {
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
