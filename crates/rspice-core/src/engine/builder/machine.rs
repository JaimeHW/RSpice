//! Elaboration of LTspice `.machine` state machines.
//!
//! A machine runs as a code model whose inputs are the node voltages and
//! branch currents its expressions read. It advances at accepted time points: at each one the
//! `*` rules and then the current state's rules are tried in order, and the
//! first whose condition reaches 0.5 moves the machine, one rule per point.
//! The outputs are then evaluated at that point and drive their nodes as
//! constant currents, one milliampere per unit, until the next accepted
//! point. In the operating point the machine is in its first state and its
//! outputs are zero; it first advances on the operating point, at time zero.
//!
//! A rule trips where its condition crosses, located to within `tripdt`
//! (by default a part in 10⁹ of the stop time): at each accepted point the
//! machine extrapolates its inputs along their last step and asks the
//! stepper to land just past the first instant the next rule would fire.
//! A candidate point a rule would fire at, reached on inputs that turned
//! inside the step, is rejected, and the stepper lands just past the first
//! instant the inputs, read as straight lines across the step, fire it.
//! After every change of state it asks for a step of that length, so the
//! outputs switch promptly and a rule the new state enables fires next.

use super::*;
use crate::config::ExpressionDialect;
use crate::expr::{CompiledExpr, Context, Vm, compile, parse_expression_strict};
use crate::netlist::StateMachine;
use crate::xspice::{
    CmContext, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortSpec, PortType,
    XspiceCheckpointSupport, XspiceInstance, XspiceSmallSignalDescriptor,
};
use std::sync::Arc;

/// The current one unit of an `.output` expression drives.
const AMPERES_PER_UNIT: Value = 1e-3;

/// The trip resolution without a `tripdt`, as a fraction of the stop time.
const DEFAULT_RESOLUTION: Value = 1e-9;

/// How far past the last step an accepted point extrapolates its inputs,
/// in steps.
const HORIZON_STEPS: Value = 4.0;

/// Samples of the horizon taken before the first trip found is bisected.
const HORIZON_SAMPLES: usize = 8;

/// Accepted-state slots: the state's index, then each output's value, then
/// each input's value.
mod slot {
    pub(super) const STATE: usize = 0;
    pub(super) const OUTPUTS: usize = 1;
}

/// What one voltage slot of a compiled expression reads.
#[derive(Debug, Clone, Copy)]
enum Operand {
    Input(usize),
    State,
    Ground,
}

/// A compiled expression, what each of its voltage slots reads, and the
/// input each of its branch-current slots reads.
struct Program {
    code: CompiledExpr,
    operands: Vec<Operand>,
    currents: Vec<usize>,
}

/// What one input of a machine reads.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Input {
    Node(String),
    Branch(String),
}

struct Rule {
    from: Option<usize>,
    to: usize,
    condition: Program,
}

struct MachineModel {
    ports: Vec<PortSpec>,
    input_ports: Vec<String>,
    output_ports: Vec<String>,
    state_values: Vec<Value>,
    rules: Vec<Rule>,
    outputs: Vec<Program>,
    tripdt: Option<Value>,
    dialect: ExpressionDialect,
    temperature: Value,
}

/// The expression machinery one evaluation shares.
struct Evaluator<'a> {
    model: &'a MachineModel,
    vm: Vm,
    voltages: Vec<Value>,
    currents: Vec<Value>,
}

impl Evaluator<'_> {
    fn value(&mut self, program: &Program, inputs: &[Value], state: usize, time: Value) -> Value {
        let state = self.model.state_values.get(state).copied().unwrap_or(0.0);
        self.voltages.clear();
        self.voltages
            .extend(program.operands.iter().map(|operand| match operand {
                Operand::Input(index) => inputs[*index],
                Operand::State => state,
                Operand::Ground => 0.0,
            }));
        self.currents.clear();
        self.currents
            .extend(program.currents.iter().map(|index| inputs[*index]));
        let context = Context::transient(&self.voltages, &self.currents, time)
            .with_temperature(self.model.temperature)
            .with_expression_dialect(self.model.dialect);
        self.vm.execute(&program.code, &context)
    }

    /// The state the first rule that fires from `state` enters, or `state`
    /// when none fires.
    fn next(&mut self, state: usize, inputs: &[Value], time: Value) -> usize {
        let model = self.model;
        let leaving = model
            .rules
            .iter()
            .filter(|rule| rule.from.is_none())
            .chain(model.rules.iter().filter(|rule| rule.from == Some(state)));
        for rule in leaving {
            if self.value(&rule.condition, inputs, state, time) >= 0.5 {
                return rule.to;
            }
        }
        state
    }
}

impl MachineModel {
    fn evaluator(&self) -> Evaluator<'_> {
        Evaluator {
            model: self,
            vm: Vm::new(),
            voltages: Vec::new(),
            currents: Vec::new(),
        }
    }

    fn input_slot(&self, index: usize) -> usize {
        slot::OUTPUTS + self.output_ports.len() + index
    }

    /// Advance from `previous` at the accepted point (`inputs`, `time`) and
    /// record the state, the outputs it drives next and the inputs.
    fn advance(
        &self,
        ctx: &mut CmContext,
        evaluator: &mut Evaluator<'_>,
        previous: usize,
        inputs: &[Value],
        time: Value,
        operating_point: bool,
    ) -> usize {
        let state = evaluator.next(previous, inputs, time);
        let record = |ctx: &mut CmContext, index: usize, value: Value| {
            if operating_point {
                ctx.set_initial_state(index, value);
            } else {
                ctx.set_state(index, value);
            }
        };
        record(ctx, slot::STATE, state as Value);
        for (index, output) in self.outputs.iter().enumerate() {
            let value = evaluator.value(output, inputs, state, time);
            record(ctx, slot::OUTPUTS + index, value);
        }
        for (index, input) in inputs.iter().enumerate() {
            record(ctx, self.input_slot(index), *input);
        }
        state
    }

    /// How closely a trip is located.
    fn resolution(&self, ctx: &CmContext) -> Value {
        self.tripdt.unwrap_or_else(|| {
            DEFAULT_RESOLUTION * ctx.transient_stop_time().unwrap_or(ctx.time).max(ctx.time)
        })
    }

    /// Ask the stepper for the next point the machine needs: a short step
    /// after a change of state or before a rule that fires next, else just
    /// past the first trip its extrapolated inputs reach.
    fn schedule(
        &self,
        ctx: &mut CmContext,
        evaluator: &mut Evaluator<'_>,
        (previous, state): (usize, usize),
        inputs: &[Value],
    ) {
        let (time, step) = (ctx.time, ctx.time - ctx.time_prev);
        let resolution = self.resolution(ctx);
        let request = |ctx: &mut CmContext, at: Value| {
            if ctx.is_ahead(at) {
                ctx.request_breakpoint(at);
            }
        };
        if state != previous || evaluator.next(state, inputs, time) != state {
            request(ctx, time + resolution);
            return;
        }
        if step.is_nan() || step <= 0.0 {
            return;
        }
        let slopes: Vec<Value> = inputs
            .iter()
            .enumerate()
            .map(|(index, input)| (input - ctx.state_prev(self.input_slot(index))) / step)
            .collect();
        let mut horizon = HORIZON_STEPS * step;
        if let Some(stop) = ctx.transient_stop_time() {
            horizon = horizon.min(stop - time);
        }
        let mut extrapolated = vec![0.0; inputs.len()];
        let mut trips = |at: Value| {
            for ((value, input), slope) in extrapolated.iter_mut().zip(inputs).zip(&slopes) {
                *value = input + slope * (at - time);
            }
            evaluator.next(state, &extrapolated, at) != state
        };
        let Some(sample) = (1..=HORIZON_SAMPLES)
            .find(|&sample| trips(time + horizon * sample as Value / HORIZON_SAMPLES as Value))
        else {
            return;
        };
        let (mut before, mut after) = (
            time + horizon * (sample - 1) as Value / HORIZON_SAMPLES as Value,
            time + horizon * sample as Value / HORIZON_SAMPLES as Value,
        );
        while after - before > 0.5 * resolution {
            let middle = 0.5 * (before + after);
            if middle <= before || middle >= after {
                break;
            }
            if trips(middle) {
                after = middle;
            } else {
                before = middle;
            }
        }
        request(ctx, after + 0.5 * resolution);
    }
}

impl MachineModel {
    /// Just past the first instant a rule fires from the accepted state on
    /// the inputs read as straight lines from the last accepted point to this
    /// candidate, when the candidate fires one and that instant falls more
    /// than the trip resolution before it.
    fn candidate_trip(&self, ctx: &CmContext) -> Option<Value> {
        let (time, before) = (ctx.time, ctx.time_prev);
        let resolution = self.resolution(ctx);
        if time - before <= resolution {
            return None;
        }
        let state = ctx.state_prev(slot::STATE) as usize;
        let candidate: Vec<Value> = self
            .input_ports
            .iter()
            .map(|port| ctx.input(port))
            .collect();
        let accepted: Vec<Value> = (0..candidate.len())
            .map(|index| ctx.state_prev(self.input_slot(index)))
            .collect();
        let mut evaluator = self.evaluator();
        let mut between = vec![0.0; candidate.len()];
        let mut trips = |at: Value| {
            let weight = (at - before) / (time - before);
            for ((value, start), end) in between.iter_mut().zip(&accepted).zip(&candidate) {
                *value = start + (end - start) * weight;
            }
            evaluator.next(state, &between, at) != state
        };
        // A rule that fires on the accepted inputs already has its short step.
        if !trips(time) || trips(before) {
            return None;
        }
        let (mut early, mut late) = (before, time);
        while late - early > 0.5 * resolution {
            let middle = 0.5 * (early + late);
            if middle <= early || middle >= late {
                break;
            }
            if trips(middle) {
                late = middle;
            } else {
                early = middle;
            }
        }
        let landing = late + 0.5 * resolution;
        (landing < time - 0.5 * resolution).then_some(landing)
    }
}

impl CodeModel for MachineModel {
    fn name(&self) -> &str {
        "ltspice_machine"
    }

    fn description(&self) -> &str {
        "LTspice state machine"
    }

    fn ports(&self) -> &[PortSpec] {
        &self.ports
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        // The outputs are currents held between accepted points: nothing
        // small-signal flows through them.
        Ok(XspiceSmallSignalDescriptor::AffineAc)
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn probe_writes_only_outputs(&self) -> bool {
        // A probe drives the outputs the last accepted point decided and
        // nothing else.
        true
    }

    fn candidate_event_time(&self, ctx: &CmContext) -> Option<Value> {
        self.candidate_trip(ctx)
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(self.input_slot(self.input_ports.len()));
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let inputs: Vec<Value> = self
            .input_ports
            .iter()
            .map(|port| ctx.input(port))
            .collect();
        let mut evaluator = self.evaluator();
        if ctx.is_dc_operating_point() || !ctx.is_transient() {
            self.advance(ctx, &mut evaluator, 0, &inputs, 0.0, true);
            for port in &self.output_ports {
                ctx.set_output(port, 0.0);
            }
            return Ok(());
        }
        // The outputs drive what the last accepted point decided; this
        // point's decision drives the next step.
        for (index, port) in self.output_ports.iter().enumerate() {
            let current = AMPERES_PER_UNIT * ctx.state_prev(slot::OUTPUTS + index);
            ctx.set_output(port, -current);
        }
        if ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe {
            // A trial only drives the outputs: the machine advances at
            // accepted points.
            return Ok(());
        }
        let previous = ctx.state_prev(slot::STATE) as usize;
        let time = ctx.time;
        let state = self.advance(ctx, &mut evaluator, previous, &inputs, time, false);
        self.schedule(ctx, &mut evaluator, (previous, state), &inputs);
        Ok(())
    }
}

/// Characters an expression identifier may continue with, as the expression
/// lexer reads them.
fn continues_identifier(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '.' | '#' | ':' | '$')
}

/// The index just past the parenthesis that closes the one at `open`.
fn closing(chars: &[char], open: usize) -> usize {
    let mut depth = 0usize;
    for (index, c) in chars.iter().enumerate().skip(open) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return index + 1;
                }
            }
            _ => {}
        }
    }
    chars.len()
}

/// Rewrite `expression` with each state name replaced by its value and
/// `state` by a voltage probe of `state_node`, leaving probe arguments,
/// function names and numbers as written. `probes` collects the argument
/// text of every `V(...)` probe.
fn bind_states(
    expression: &str,
    states: &[(String, Value)],
    state_node: &str,
    probes: &mut Vec<String>,
) -> String {
    let chars: Vec<char> = expression.chars().collect();
    let mut bound = String::with_capacity(expression.len());
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index];
        let number_start = c.is_ascii_digit()
            || (c == '.' && chars.get(index + 1).is_some_and(char::is_ascii_digit));
        if number_start {
            let start = index;
            while index < chars.len() && (chars[index].is_ascii_digit() || chars[index] == '.') {
                index += 1;
            }
            if index < chars.len() && matches!(chars[index], 'e' | 'E') {
                let digits_at = match chars.get(index + 1) {
                    Some('+' | '-') => index + 2,
                    _ => index + 1,
                };
                if chars.get(digits_at).is_some_and(char::is_ascii_digit) {
                    index = digits_at;
                    while index < chars.len() && chars[index].is_ascii_digit() {
                        index += 1;
                    }
                }
            }
            while index < chars.len() && chars[index].is_ascii_alphabetic() {
                index += 1;
            }
            bound.extend(&chars[start..index]);
            continue;
        }
        if !(c.is_alphabetic() || c == '_' || c == '$') {
            bound.push(c);
            index += 1;
            continue;
        }
        let start = index;
        while index < chars.len() && continues_identifier(chars[index]) {
            index += 1;
        }
        let name: String = chars[start..index].iter().collect();
        let mut next = index;
        while next < chars.len() && chars[next].is_whitespace() {
            next += 1;
        }
        let calls = chars.get(next) == Some(&'(');
        if calls && (name.eq_ignore_ascii_case("v") || name.eq_ignore_ascii_case("i")) {
            let end = closing(&chars, next);
            if name.eq_ignore_ascii_case("v") {
                let inner: String = chars[next + 1..end.saturating_sub(1).max(next + 1)]
                    .iter()
                    .collect();
                probes.extend(inner.split(',').map(|node| node.trim().to_string()));
            }
            bound.extend(&chars[start..end]);
            index = end;
        } else if calls {
            bound.push_str(&name);
        } else if name.eq_ignore_ascii_case("state") {
            bound.push_str(&format!("V({state_node})"));
        } else if let Some((_, value)) = states
            .iter()
            .find(|(state, _)| state.eq_ignore_ascii_case(&name))
        {
            bound.push_str(&format!("({value:e})"));
        } else {
            bound.push_str(&name);
        }
    }
    bound
}

/// Build the state machines into `circuit`, once every other element is
/// in place.
pub(super) fn build_all(
    circuit: &mut CircuitData,
    machines: &[(&Element, &StateMachine)],
    params: &crate::netlist::ParamContext,
    temperature: Value,
) -> Result<(), SimulationError> {
    // A machine may read another's output.
    for (element, _) in machines {
        for node in &element.nodes {
            circuit.get_or_create_node(node);
        }
    }
    for (element, machine) in machines {
        build(circuit, element, machine, params, temperature)?;
    }
    Ok(())
}

/// Build one state machine into `circuit`.
fn build(
    circuit: &mut CircuitData,
    element: &Element,
    machine: &StateMachine,
    params: &crate::netlist::ParamContext,
    temperature: Value,
) -> Result<(), SimulationError> {
    let fail = |message: String| SimulationError::Circuit(format!("{}: {message}", element.name));
    let compile_text = |what: &str, text: &str| -> Result<CompiledExpr, SimulationError> {
        let prepared = prepare_behavioral_expression(text, params)
            .map_err(|error| fail(format!("{what} '{text}': {error}")))?;
        let parsed = parse_expression_strict(&prepared)
            .map_err(|error| fail(format!("{what} '{text}': {error}")))?;
        Ok(compile(&parsed))
    };
    let constant = |what: &str, text: &str| -> Result<Value, SimulationError> {
        let code = compile_text(what, text)?;
        if !code.node_map.is_empty() || !code.branch_map.is_empty() {
            return Err(fail(format!("{what} '{text}' must be a constant")));
        }
        let value = Vm::new().execute(&code, &Context::dc(&[], &[]));
        if !value.is_finite() {
            return Err(fail(format!("{what} '{text}' is not a finite number")));
        }
        Ok(value)
    };

    let mut states = Vec::with_capacity(machine.states.len());
    for (name, text) in &machine.states {
        states.push((name.clone(), constant(&format!("state {name}"), text)?));
    }
    let tripdt = match &machine.tripdt {
        Some(text) => {
            let tripdt = constant("tripdt", text)?;
            if tripdt <= 0.0 {
                return Err(fail(format!("tripdt must be positive, found {tripdt}")));
            }
            Some(tripdt)
        }
        None => None,
    };

    // `state` reads as a probe of a node no expression names.
    let sources: Vec<&str> = machine
        .rules
        .iter()
        .map(|rule| rule.condition.as_str())
        .chain(machine.outputs.iter().map(String::as_str))
        .collect();
    let mut probes = Vec::new();
    for source in &sources {
        bind_states(source, &states, "state", &mut probes);
    }
    let mut state_node = String::from("state");
    while probes
        .iter()
        .any(|probe| probe.eq_ignore_ascii_case(&state_node))
    {
        state_node.push('#');
    }

    let mut inputs: Vec<Input> = Vec::new();
    let mut input_index = |input: Input| match inputs.iter().position(|known| *known == input) {
        Some(index) => index,
        None => {
            inputs.push(input);
            inputs.len() - 1
        }
    };
    let mut program = |what: &str, text: &str| -> Result<Program, SimulationError> {
        let bound = bind_states(text, &states, &state_node, &mut Vec::new());
        let code = compile_text(what, &bound)?;
        let mut operands = vec![Operand::Ground; code.node_map.len()];
        for (node, local) in &code.node_map {
            // Node names fold to upper case, as every node table matches them.
            let node = node.to_ascii_uppercase();
            operands[*local] = if node == state_node.to_ascii_uppercase() {
                Operand::State
            } else if crate::naming::is_spice_ground_name(&node) {
                Operand::Ground
            } else if circuit.get_node_by_name(&node).is_none() {
                return Err(fail(format!(
                    "{what} '{text}' reads node {node}, which no element connects"
                )));
            } else {
                Operand::Input(input_index(Input::Node(node)))
            };
        }
        let mut currents = vec![0; code.branch_map.len()];
        for (branch, local) in &code.branch_map {
            currents[*local] = input_index(Input::Branch(branch.to_ascii_uppercase()));
        }
        Ok(Program {
            code,
            operands,
            currents,
        })
    };
    let mut rules = Vec::with_capacity(machine.rules.len());
    for rule in &machine.rules {
        rules.push(Rule {
            from: rule.from,
            to: rule.to,
            condition: program("rule condition", &rule.condition)?,
        });
    }
    let mut outputs = Vec::with_capacity(machine.outputs.len());
    for output in &machine.outputs {
        outputs.push(program("output", output)?);
    }

    if element.nodes.len() != 2 * outputs.len() {
        return Err(fail(format!(
            "{} outputs need {} nodes, found {}",
            outputs.len(),
            2 * outputs.len(),
            element.nodes.len()
        )));
    }
    let input_ports: Vec<String> = (0..inputs.len())
        .map(|index| format!("in{index}"))
        .collect();
    let output_ports: Vec<String> = (0..outputs.len())
        .map(|index| format!("out{index}"))
        .collect();
    let ports = input_ports
        .iter()
        .zip(&inputs)
        .map(|(port, input)| {
            let port_type = match input {
                Input::Node(_) => PortType::DifferentialVoltage,
                Input::Branch(_) => PortType::VoltageName,
            };
            PortSpec::input(port.as_str(), port_type)
        })
        .chain(
            output_ports
                .iter()
                .map(|port| PortSpec::output(port.as_str(), PortType::DifferentialCurrent)),
        )
        .collect();
    let connections: Vec<XspicePort> = inputs
        .iter()
        .map(|input| match input {
            Input::Node(node) => XspicePort::DifferentialVoltage {
                pos: node.clone(),
                neg: "0".to_string(),
            },
            Input::Branch(branch) => XspicePort::VoltageName(branch.clone()),
        })
        .chain(
            element
                .nodes
                .chunks(2)
                .map(|pair| XspicePort::DifferentialCurrent {
                    pos: pair[0].clone(),
                    neg: pair[1].clone(),
                }),
        )
        .collect();
    let model: Arc<dyn CodeModel> = Arc::new(MachineModel {
        ports,
        input_ports,
        output_ports,
        state_values: states.into_iter().map(|(_, value)| value).collect(),
        rules,
        outputs,
        tripdt,
        dialect: params.expression_dialect(),
        temperature: crate::constants::kelvin_to_celsius(temperature),
    });

    let connections = coerce_xspice_connections(
        circuit,
        model.ports(),
        &connections,
        &element.name,
        model.name(),
    )?;
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bind(expression: &str) -> (String, Vec<String>) {
        let states = [("S0a".to_string(), 0.0), ("hi".to_string(), 2.5)];
        let mut probes = Vec::new();
        let bound = bind_states(expression, &states, "state#", &mut probes);
        (bound, probes)
    }

    #[test]
    fn state_names_bind_to_their_values_and_state_to_its_probe() {
        assert_eq!(
            bind("state==S0A & V(hi) > hi*2"),
            (
                "V(state#)==(0e0) & V(hi) > (2.5e0)*2".to_string(),
                vec!["hi".to_string()]
            )
        );
    }

    #[test]
    fn numbers_functions_and_differential_probes_stay_as_written() {
        assert_eq!(
            bind("hi(1e-3, 2meg) + 1hi + V(a, state)"),
            (
                "hi(1e-3, 2meg) + 1hi + V(a, state)".to_string(),
                vec!["a".to_string(), "state".to_string()]
            )
        );
    }
}
