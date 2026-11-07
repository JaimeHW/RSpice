//! Time-domain operators inside behavioral expressions: `ddt`, `idt` and
//! `delay` (with `absdelay` as an alias).
//!
//! Each call is realized at parse time by a private node and existing,
//! fully validated elements, and the call is replaced by that node's voltage.
//! Every analysis then sees an ordinary circuit:
//!
//! - `ddt(x)` drives the current `x` through a 1 H inductor, whose voltage is
//!   `dx/dt`: zero at DC, `jω·x` in AC, and step-controlled by the inductor's
//!   truncation error in transient.
//! - `idt(x[,ic[,reset]])` integrates the current `x` on a 1 F capacitor whose
//!   zero initial condition is held through the operating point, so the
//!   integral is `ic` at DC, `x/(jω)` in AC, and starts from `ic` in transient.
//!   While `reset` exceeds 0.5 during a transient the capacitor is held at zero.
//! - `delay(x,td[,tdmax])` launches `x` into a matched lossless line of delay
//!   `td`. Its far end is `x(t-td)` with the operating point as the history
//!   before time zero, `x·e^(-jω·td)` in AC; the line keeps the waveform
//!   history and schedules the delayed breakpoints. A delay that reads the
//!   circuit or time is a variable-delay source instead: it keeps the history
//!   of `x`, back as far as `tdmax` when that is given, and reads it at
//!   `t-td(t)`; a delay outside `[0, tdmax]` stops the simulation. In AC it is
//!   `x·e^(-jω·td)` at the operating point's delay.
//!
//! `rand(x)`, `random(x)` and `white(x)` are pure functions of `x` and a seed.
//! A call without its own seed is given the deck's `.options seed` here, so
//! every behavioral source draws the same samples the parameter evaluator does.

use super::{Element, ElementKind, ParseError, SourceMultiplicity};
use crate::netlist::expr::DEFAULT_RANDOM_SEED;
use crate::netlist::lexer::parse_spice_value_complete;
use crate::netlist::{ElementProvenance, SynthesizedTransferForm};

/// Impedance of the delay line and its termination. A lossless line falls
/// back to a milliohm series resistance at DC, so a high impedance keeps that
/// fallback a part-per-billion error in the operating point and its history.
const DELAY_LINE_IMPEDANCE: f64 = 1e6;

/// One operator call located in an expression.
struct Call {
    operator: Operator,
    /// Byte range of the whole call, name through closing parenthesis.
    start: usize,
    end: usize,
    args: Vec<String>,
}

#[derive(Clone, Copy)]
enum Operator {
    /// A time-domain operator realized by elements.
    Dynamic(SynthesizedTransferForm),
    /// A random function, by its canonical name.
    Random(&'static str),
}

fn operator(name: &str) -> Option<Operator> {
    match name.to_ascii_uppercase().as_str() {
        "DDT" => Some(Operator::Dynamic(SynthesizedTransferForm::Derivative)),
        "IDT" => Some(Operator::Dynamic(SynthesizedTransferForm::Integral)),
        "DELAY" | "ABSDELAY" => Some(Operator::Dynamic(SynthesizedTransferForm::Delay)),
        "RAND" => Some(Operator::Random("rand")),
        "RANDOM" => Some(Operator::Random("random")),
        "WHITE" => Some(Operator::Random("white")),
        _ => None,
    }
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'$' | b'#')
}

/// Every identifier in `text` outside quoted strings, with the byte offset of
/// the first non-blank character after it.
fn identifiers(text: &str) -> Vec<(usize, &str, usize)> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'"' {
            index += 1;
            while index < bytes.len() && bytes[index] != b'"' {
                index += 1;
            }
            index += 1;
        } else if is_identifier_start(byte) {
            let start = index;
            while index < bytes.len() && is_identifier_byte(bytes[index]) {
                index += 1;
            }
            let mut next = index;
            while next < bytes.len() && bytes[next].is_ascii_whitespace() {
                next += 1;
            }
            found.push((start, &text[start..index], next));
        } else if byte.is_ascii_digit() {
            // A number with an exponent or scale suffix is not an identifier.
            while index < bytes.len() && is_identifier_byte(bytes[index]) {
                index += 1;
            }
        } else {
            index += 1;
        }
    }
    found
}

/// Split the parenthesized argument list opening at `open` into top-level
/// arguments, returning them with the offset just past the closing parenthesis.
fn call_arguments(text: &str, open: usize) -> Option<(Vec<String>, usize)> {
    let bytes = text.as_bytes();
    let mut depth = 0usize;
    let mut args = Vec::new();
    let mut argument_start = open + 1;
    let mut index = open;
    while index < bytes.len() {
        match bytes[index] {
            b'"' => {
                index += 1;
                while index < bytes.len() && bytes[index] != b'"' {
                    index += 1;
                }
            }
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    args.push(text[argument_start..index].trim().to_string());
                    return Some((args, index + 1));
                }
            }
            b',' if depth == 1 => {
                args.push(text[argument_start..index].trim().to_string());
                argument_start = index + 1;
            }
            _ => {}
        }
        index += 1;
    }
    None
}

/// The first operator call in `text` at or after byte `from`.
fn first_call(text: &str, from: usize, line: usize) -> Result<Option<Call>, ParseError> {
    for (start, name, next) in identifiers(text) {
        let Some(operator) = operator(name).filter(|_| start >= from) else {
            continue;
        };
        if text.as_bytes().get(next) != Some(&b'(') {
            continue;
        }
        let (args, end) = call_arguments(text, next).ok_or_else(|| ParseError::Syntax {
            line,
            message: format!("unbalanced parentheses in {name}() call"),
        })?;
        return Ok(Some(Call {
            operator,
            start,
            end,
            args,
        }));
    }
    Ok(None)
}

/// Whether an expression reads the circuit solution or simulation time, which
/// a line's fixed delay cannot follow.
fn varies_during_simulation(text: &str) -> bool {
    identifiers(text).iter().any(|(_, name, next)| {
        let upper = name.to_ascii_uppercase();
        upper == "TIME"
            || (text.as_bytes().get(*next) == Some(&b'(')
                && matches!(
                    upper.as_str(),
                    "V" | "I"
                        | "VM"
                        | "VP"
                        | "VR"
                        | "VI"
                        | "VDB"
                        | "IM"
                        | "IP"
                        | "IR"
                        | "II"
                        | "IDB"
                ))
    })
}

/// Lower every `ddt`, `idt` and `delay` call in a behavioral expression owned
/// by `owner`, appending the elements that realize them to `elements`, and seed
/// every random function that names no seed with the deck's `seed`. Returns
/// the rewritten expression.
pub(super) fn lower_dynamic_operators(
    owner: &str,
    expression: String,
    line: usize,
    seed: u64,
    elements: &mut Vec<Element>,
) -> Result<String, ParseError> {
    let mut lowering = Lowering {
        owner,
        line,
        seed,
        ordinal: 0,
        elements,
    };
    lowering.lower(&expression)
}

struct Lowering<'a> {
    owner: &'a str,
    line: usize,
    seed: u64,
    ordinal: usize,
    elements: &'a mut Vec<Element>,
}

impl Lowering<'_> {
    /// Replace every call in `text`, realizing the operators inside an
    /// argument before the call that reads them.
    fn lower(&mut self, text: &str) -> Result<String, ParseError> {
        let mut text = text.to_string();
        let mut cursor = 0;
        while let Some(mut call) = first_call(&text, cursor, self.line)? {
            for arg in &mut call.args {
                *arg = self.lower(arg)?;
            }
            let replacement = match call.operator {
                Operator::Random(name) => {
                    if call.args.len() == 1 && self.seed != DEFAULT_RANDOM_SEED {
                        call.args.push(self.seed.to_string());
                    }
                    format!("{name}({})", call.args.join(","))
                }
                Operator::Dynamic(form) => {
                    self.ordinal += 1;
                    realize(
                        self.owner,
                        self.ordinal,
                        form,
                        &call.args,
                        self.line,
                        self.elements,
                    )?
                }
            };
            text.replace_range(call.start..call.end, &replacement);
            cursor = call.start + replacement.len();
        }
        Ok(text)
    }
}

fn realize(
    owner: &str,
    ordinal: usize,
    form: SynthesizedTransferForm,
    args: &[String],
    line: usize,
    elements: &mut Vec<Element>,
) -> Result<String, ParseError> {
    let label = form.label();
    let fail = |message: String| ParseError::Syntax {
        line,
        message: format!("behavioral source '{owner}': {message}"),
    };
    let (min_args, max_args) = match form {
        SynthesizedTransferForm::Derivative => (1, 1),
        SynthesizedTransferForm::Integral => (1, 3),
        _ => (2, 3),
    };
    if !(min_args..=max_args).contains(&args.len()) || args.iter().any(String::is_empty) {
        return Err(fail(format!(
            "{label}() takes {min_args} to {max_args} arguments, got {}",
            args.len()
        )));
    }
    let input = &args[0];
    let node = |role: &str| format!("{owner}.__{role}{ordinal}");
    let internal = |node: &str| ElementProvenance::GeneratedDynamicInternalNode {
        owner: owner.to_string(),
        form,
        node: node.to_string(),
    };
    let driven = ElementProvenance::GeneratedDynamicStateDerivative {
        owner: owner.to_string(),
        form,
        dc_determined: true,
    };
    let current_into = |name: String, node: &str, expression: &str| Element {
        name,
        kind: ElementKind::BehavioralCurrent {
            expression: expression.to_string(),
            tc1: 0.0,
            tc2: 0.0,
            multiplicity: SourceMultiplicity::default(),
        },
        nodes: vec!["0".to_string(), node.to_string()],
        provenance: driven.clone(),
    };

    match form {
        SynthesizedTransferForm::Derivative => {
            let state = node("DDT");
            elements.push(current_into(node("GDDT"), &state, input));
            elements.push(Element {
                name: node("LDDT"),
                kind: ElementKind::Inductor {
                    value: 1.0,
                    value_expr: None,
                    initial_current: None,
                    model: None,
                    instance_params: Vec::new(),
                    deferred_params: Vec::new(),
                },
                nodes: vec![state.clone(), "0".to_string()],
                provenance: internal(&state),
            });
            Ok(format!("V({state})"))
        }
        SynthesizedTransferForm::Integral => {
            let state = node("IDT");
            elements.push(current_into(node("GIDT"), &state, input));
            elements.push(Element {
                name: node("CIDT"),
                kind: ElementKind::Capacitor {
                    value: 1.0,
                    value_expr: None,
                    initial_voltage: Some(0.0),
                    model: None,
                    instance_params: Vec::new(),
                    deferred_params: Vec::new(),
                },
                nodes: vec![state.clone(), "0".to_string()],
                provenance: internal(&state),
            });
            if let Some(reset) = args.get(2) {
                // An ideal switch across the capacitor: while reset is asserted
                // the branch pins the state to zero; otherwise its equation
                // reduces to a zero branch current.
                let name = node("SIDT");
                elements.push(Element {
                    kind: ElementKind::BehavioralVoltage {
                        expression: format!("if(time>0&&({reset})>0.5,0,V({state})-I({name}))"),
                        tc1: 0.0,
                        tc2: 0.0,
                        multiplicity: SourceMultiplicity::default(),
                    },
                    name,
                    nodes: vec![state.clone(), "0".to_string()],
                    provenance: internal(&state),
                });
            }
            Ok(match args.get(1) {
                Some(initial) => format!("(({initial})+V({state}))"),
                None => format!("V({state})"),
            })
        }
        _ => {
            let delay = &args[1];
            let maximum = args.get(2);
            if let Some(value) = maximum.and_then(|text| parse_spice_value_complete(text).ok())
                && !(value.is_finite() && value > 0.0)
            {
                return Err(fail(format!(
                    "{label}() maximum delay must be finite and positive, got {value}"
                )));
            }
            if varies_during_simulation(delay) {
                return Ok(variable_delay(
                    owner, ordinal, form, input, delay, maximum, elements,
                ));
            }
            let fixed = parse_spice_value_complete(delay).ok();
            if let Some(value) = fixed {
                if !(value.is_finite() && value >= 0.0) {
                    return Err(fail(format!(
                        "{label}() delay must be finite and non-negative, got {value}"
                    )));
                }
                if value == 0.0 {
                    return Ok(format!("({input})"));
                }
                if let Some(maximum) = args
                    .get(2)
                    .and_then(|text| parse_spice_value_complete(text).ok())
                    && value > maximum
                {
                    return Err(fail(format!(
                        "{label}() delay {value} exceeds its maximum {maximum}"
                    )));
                }
            }
            let delay = match fixed {
                Some(value) => DelayTime::Fixed(value),
                None => DelayTime::Deferred(delay.clone()),
            };
            Ok(delay_line(owner, ordinal, form, input, delay, elements))
        }
    }
}

/// Realize a delay that reads the circuit or time: private nodes carry the
/// input, the delay and its bound, and a variable-delay source reads them.
/// Returns the expression of its output.
fn variable_delay(
    owner: &str,
    ordinal: usize,
    form: SynthesizedTransferForm,
    input: &str,
    delay: &str,
    maximum: Option<&String>,
    elements: &mut Vec<Element>,
) -> String {
    let node = |role: &str| format!("{owner}.__{role}{ordinal}");
    let source = |name: String, node: &str, expression: &str| Element {
        name,
        kind: ElementKind::BehavioralVoltage {
            expression: expression.to_string(),
            tc1: 0.0,
            tc2: 0.0,
            multiplicity: SourceMultiplicity::default(),
        },
        nodes: vec![node.to_string(), "0".to_string()],
        provenance: ElementProvenance::GeneratedDynamicInternalNode {
            owner: owner.to_string(),
            form,
            node: node.to_string(),
        },
    };
    let (input_node, delay_node, output) = (node("DLYI"), node("DLYT"), node("DLYO"));
    elements.push(source(node("BDLY"), &input_node, input));
    elements.push(source(node("BDLT"), &delay_node, delay));
    let maximum = maximum.map(|bound| {
        let bound_node = node("DLYM");
        elements.push(source(node("BDLM"), &bound_node, bound));
        bound_node
    });
    elements.push(Element {
        name: node("VDLY"),
        kind: ElementKind::BehavioralVoltage {
            expression: format!("V({input_node})"),
            tc1: 0.0,
            tc2: 0.0,
            multiplicity: SourceMultiplicity::default(),
        },
        nodes: vec![output.clone(), "0".to_string()],
        provenance: ElementProvenance::GeneratedVariableDelay {
            owner: owner.to_string(),
            node: output.clone(),
            input: input_node,
            delay: delay_node,
            maximum,
        },
    });
    format!("V({output})")
}

/// How long a generated delay line delays: a number, or an expression the
/// flattener resolves in the instance's parameter scope.
pub(super) enum DelayTime {
    Fixed(f64),
    Deferred(String),
}

/// Launch `input` into a matched lossless line of `delay` owned by `owner`,
/// appending its elements; returns the expression of its delayed far end.
pub(super) fn delay_line(
    owner: &str,
    ordinal: usize,
    form: SynthesizedTransferForm,
    input: &str,
    delay: DelayTime,
    elements: &mut Vec<Element>,
) -> String {
    let node = |role: &str| format!("{owner}.__{role}{ordinal}");
    let internal = |node: &str| ElementProvenance::GeneratedDynamicInternalNode {
        owner: owner.to_string(),
        form,
        node: node.to_string(),
    };
    let (td, deferred_params) = match delay {
        DelayTime::Fixed(value) => (Some(value), Vec::new()),
        DelayTime::Deferred(text) => (None, vec![("TD".to_string(), text)]),
    };
    let launch = node("DLYI");
    let far = node("DLYO");
    elements.push(Element {
        name: node("BDLY"),
        kind: ElementKind::BehavioralVoltage {
            expression: input.to_string(),
            tc1: 0.0,
            tc2: 0.0,
            multiplicity: SourceMultiplicity::default(),
        },
        nodes: vec![launch.clone(), "0".to_string()],
        provenance: internal(&launch),
    });
    elements.push(Element {
        name: node("TDLY"),
        kind: ElementKind::TransmissionLine {
            z0: Some(DELAY_LINE_IMPEDANCE),
            td,
            freq: None,
            nl: None,
            model: None,
            deferred_params,
        },
        nodes: vec![launch, "0".to_string(), far.clone(), "0".to_string()],
        provenance: internal(&far),
    });
    elements.push(Element {
        name: node("RDLY"),
        kind: ElementKind::Resistor {
            value: DELAY_LINE_IMPEDANCE,
            value_expr: None,
            model: None,
            instance_params: Vec::new(),
            deferred_params: Vec::new(),
        },
        nodes: vec![far.clone(), "0".to_string()],
        provenance: internal(&far),
    });
    format!("V({far})")
}
