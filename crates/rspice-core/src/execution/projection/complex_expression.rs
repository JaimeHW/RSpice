//! A Xyce `.PRINT AC` or `.PRINT HB` expression, evaluated in complex
//! arithmetic over the sweep or the spectrum's bins. Xyce compiles an AC or
//! HB frequency-domain output expression over complex values and
//! prints one column when the expression's type is real, and its real and
//! imaginary parts as `Re(...)` and `Im(...)` when it is complex
//! (IOInterfacePKG/N_IO_OutputterLocal.C:368-377). The type is static, one
//! rule per node (UtilityPKG/ExpressionSrc/ast.h, astfuncs.h, astbinary.h):
//! a `V()` or `I()` phasor is complex; a literal, a parameter and a rendered
//! accessor (`VR`, `VM`, `VP`, ...) are real; arithmetic, `IF` and the
//! analytic functions carry their operands' type; `ABS`, the rounding and
//! comparison operators, `MAX`, `MIN` and their kin are real.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use super::{
    ProjectedSignal, ProjectionSource, ProjectionValues, SignalKind, SignalValueType,
    is_lead_accessor, resolve_complex_operand, signal_descriptor,
};
use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::analysis::measure_signals::PhaseUnit;
use crate::engine::SimulationError;
use crate::netlist::expr::{
    BinOpKind, ComplexValue, Expr, ExprError, ExpressionEvaluationError, OutputExpression,
    OutputExpressionError, PreparedExpression, UnaryOpKind,
};
use crate::netlist::{OutputOperandKind, ParamContext, SaveSignal, canonical_symbol};

/// The columns one Xyce `.PRINT AC` expression prints: its value, or `Re()`
/// and `Im()` of it when the expression is complex. `None` when the body
/// does not parse or compile here, which leaves it to the real resolver.
pub(super) fn xyce_ac_expression_columns(
    source: &ProjectionSource<'_>,
    params: &ParamContext,
    authored: &str,
    body: &str,
    phase_unit: PhaseUnit,
    abort: &dyn AbortSignal,
) -> Result<Option<Vec<ProjectedSignal>>, SimulationError> {
    let output = match OutputExpression::parse(body, params, abort) {
        Ok(output) => output,
        Err(OutputExpressionError::Aborted) => return Err(SimulationError::Aborted),
        Err(OutputExpressionError::Parse(_) | OutputExpressionError::Semantic(_)) => {
            return Ok(None);
        }
    };

    let mut phasors: Vec<Vec<ComplexValue>> = Vec::new();
    let mut keys: HashMap<String, usize> = HashMap::new();
    let mut unavailable = None;
    let compiled = PreparedExpression::compile_output(
        output.root(),
        &output,
        params,
        &HashSet::new(),
        &mut |node| {
            let Expr::FnCall { name, args } = node else {
                return Ok(None);
            };
            if params.has_function(name) || !is_phasor_accessor(name) {
                return Ok(None);
            }
            let arguments = args
                .iter()
                .map(probe_argument)
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    ExprError::InvalidArgument(format!("{name}() has an invalid argument"))
                })?;
            let text = format!("{}({})", name.to_ascii_uppercase(), arguments.join(","));
            let kind = match arguments.as_slice() {
                [positive, negative] if name.eq_ignore_ascii_case("V") => OutputOperandKind::Probe(
                    SaveSignal::VoltageDiff(positive.clone(), negative.clone()),
                ),
                _ => OutputOperandKind::Expression { body: text.clone() },
            };
            let column = match resolve_complex_operand(source, &text, &kind, phase_unit) {
                Ok(column) => column,
                Err(error) => {
                    unavailable = Some(error);
                    return Err(ExprError::InvalidArgument(text));
                }
            };
            let values = match &column.values {
                ProjectionValues::Complex { real, imag } => real
                    .iter()
                    .zip(imag.iter())
                    .map(|(real, imag)| ComplexValue::new(*real, *imag))
                    .collect(),
                ProjectionValues::Real(values) => values
                    .iter()
                    .map(|value| ComplexValue::new(*value, 0.0))
                    .collect(),
            };
            // Parsed identifiers cannot contain NUL, so the key names no
            // parameter the deck could author.
            let key = format!("\0RSPICE_AC_PHASOR_{}", phasors.len());
            keys.insert(key.clone(), phasors.len());
            phasors.push(values);
            Ok(Some(key))
        },
    );
    if let Some(error) = unavailable {
        return Err(error);
    }
    let Ok(mut compiled) = compiled else {
        return Ok(None);
    };

    let lookup = source
        .lookup
        .iter()
        .map(|(name, values)| (canonical_symbol(name), *values))
        .collect::<Vec<_>>();
    let axis = source.axis.as_ref();
    let mut values = Vec::with_capacity(axis.len());
    for (row, &frequency) in axis.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let value = compiled.evaluate_with_abort(
            params,
            &mut |name| {
                if let Some(&index) = keys.get(name) {
                    return Ok(phasors
                        .get(index)
                        .and_then(|phasor| phasor.get(row))
                        .copied());
                }
                let canonical = canonical_symbol(name);
                if let Some((_, series)) = lookup.iter().find(|(key, _)| *key == canonical) {
                    return Ok(series.get(row).map(|value| ComplexValue::new(*value, 0.0)));
                }
                if matches!(canonical.as_str(), "FREQ" | "FREQUENCY" | "HERTZ") {
                    return Ok(Some(ComplexValue::new(frequency, 0.0)));
                }
                Ok(None)
            },
            abort,
        );
        match value {
            Ok(value) => values.push(value),
            Err(ExpressionEvaluationError::Aborted) => return Err(SimulationError::Aborted),
            Err(ExpressionEvaluationError::Expression(error)) => {
                return Err(SimulationError::Netlist(format!(
                    "output expression '{authored}' at frequency point {row}: {error}"
                )));
            }
        }
    }

    let name = authored.trim();
    let columns = if output_is_complex(&output) {
        vec![
            real_column(format!("Re({name})"), values.iter().map(|value| value.re))?,
            real_column(format!("Im({name})"), values.iter().map(|value| value.im))?,
        ]
    } else {
        vec![real_column(
            name.to_string(),
            values.iter().map(|value| value.re),
        )?]
    };
    Ok(Some(columns))
}

fn real_column(
    name: String,
    values: impl Iterator<Item = Value>,
) -> Result<ProjectedSignal, SimulationError> {
    let values = values.collect::<Vec<_>>();
    let descriptor = signal_descriptor(&name, &name, SignalKind::Scalar, SignalValueType::Real)
        .map_err(|error| {
            SimulationError::Netlist(format!("output '{name}' cannot be described: {error}"))
        })?;
    Ok(ProjectedSignal {
        descriptor,
        validity: vec![true; values.len()],
        values: ProjectionValues::Real(Cow::Owned(values)),
    })
}

/// An accessor reading the AC result: a phasor, a lead current or one of
/// their renderings.
fn is_phasor_accessor(name: &str) -> bool {
    is_lead_accessor(name)
        || matches!(
            name.to_ascii_uppercase().as_str(),
            "V" | "VR" | "VI" | "VM" | "VP" | "VDB" | "I" | "IR" | "II" | "IM" | "IP" | "IDB"
        )
}

/// A node or branch name as an accessor's argument parses: a name, or a
/// node number.
fn probe_argument(argument: &Expr) -> Option<String> {
    match argument {
        Expr::Param(name) => Some(name.clone()),
        Expr::Number(value) if value.is_finite() && value.fract() == 0.0 => {
            Some(format!("{value:.0}"))
        }
        _ => None,
    }
}

/// Whether Xyce types the expression complex: some `V()` or `I()` phasor (or
/// a literal with an imaginary part) reaches the root through nodes that
/// carry their operands' type, through the bodies of the functions it calls
/// as through its own nodes.
fn output_is_complex(output: &OutputExpression) -> bool {
    let mut summaries = HashMap::new();
    complex_reach(output.root(), &[], output, &mut summaries).complex
}

/// How a tree's type reaches its root: complex in itself, and which of the
/// enclosing function's formals carry their argument's type to it.
#[derive(Clone, Default)]
struct TypeReach {
    complex: bool,
    formals: Vec<usize>,
}

/// The type reach of `tree`, read in a body with `formals`. A call carries
/// the type its instance's body does, so each instance is typed once.
fn complex_reach(
    tree: &Expr,
    formals: &[String],
    output: &OutputExpression,
    summaries: &mut HashMap<String, TypeReach>,
) -> TypeReach {
    let mut reach = TypeReach::default();
    let mut pending = vec![tree];
    while let Some(node) = pending.pop() {
        if let Expr::FnCall { name, args } = node
            && let Some(instance) = output.instance(name)
        {
            let callee = match summaries.get(instance.key()) {
                Some(callee) => callee.clone(),
                None => {
                    let callee =
                        complex_reach(instance.body(), instance.formals(), output, summaries);
                    summaries.insert(instance.key().to_owned(), callee.clone());
                    callee
                }
            };
            reach.complex |= callee.complex;
            pending.extend(callee.formals.iter().filter_map(|&formal| args.get(formal)));
            continue;
        }
        match node {
            Expr::Param(name) => {
                if let Some(formal) = formals.iter().position(|formal| formal == name)
                    && !reach.formals.contains(&formal)
                {
                    reach.formals.push(formal);
                }
            }
            Expr::Number(_) | Expr::StringLiteral(_) => {}
            Expr::ComplexNumber(value) => reach.complex |= value.im != 0.0,
            Expr::UnaryOp {
                op: UnaryOpKind::Neg,
                operand,
            } => pending.push(operand),
            Expr::UnaryOp { .. } => {}
            Expr::BinOp {
                op:
                    BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div | BinOpKind::Pow,
                left,
                right,
            } => pending.extend([left.as_ref(), right.as_ref()]),
            // `%` is Xyce's real fmod; comparisons and logic are real.
            Expr::BinOp { .. } => {}
            Expr::FnCall { name, args } => match name.to_ascii_uppercase().as_str() {
                "V" | "I" => reach.complex = true,
                lead if is_lead_accessor(lead) => reach.complex = true,
                "IF" => pending.extend(args.iter().skip(1)),
                "VR" | "VI" | "VM" | "VP" | "VDB" | "IR" | "II" | "IM" | "IP" | "IDB" | "ABS"
                | "REAL" | "RE" | "IMAG" | "IMG" | "PH" | "PHASE" | "MAG" | "M" | "DB" | "MAX"
                | "MIN" | "SGN" | "SIGN" | "FMOD" | "ROUND" | "NINT" | "CEIL" | "FLOOR" | "INT"
                | "LIMIT" | "STP" | "URAMP" | "TABLE" | "TBL" | "SCHEDULE" | "SDT" | "DDT"
                | "P" | "W" | "N" => {}
                _ => pending.extend(args),
            },
        }
    }
    reach
}

#[cfg(test)]
mod tests {
    use super::output_is_complex;
    use crate::netlist::ParamContext;
    use crate::netlist::expr::OutputExpression;

    fn is_complex(body: &str, params: &ParamContext) -> bool {
        let output =
            OutputExpression::parse(body, params, &crate::abort_signal::NoAbort).expect("parses");
        output_is_complex(&output)
    }

    #[test]
    fn xyce_types_an_ac_expression_by_its_phasors() {
        for (body, complex) in [
            ("v(2)*2", true),
            ("-v(2)", true),
            ("sqrt(v(1)-v(2))", true),
            ("if(1, v(2), 0)", true),
            ("vr(2)*k", false),
            ("(vm(2)-vr(2))/k", false),
            ("abs(v(2))", false),
            ("vp(2)", false),
            ("max(v(1), v(2))", false),
            ("v(2) > 0", false),
        ] {
            assert_eq!(is_complex(body, &ParamContext::new()), complex, "{body}");
        }
    }

    #[test]
    fn a_call_carries_the_type_its_body_gives_its_arguments() {
        let mut params = ParamContext::new();
        params.define_function("twice", vec!["x".into()], "2*x");
        params.define_function("magnitude", vec!["x".into()], "abs(x)");
        params.define_function("probe", vec!["x".into()], "v(2)+x");
        params.define_function(
            "outer",
            vec!["a".into(), "b".into()],
            "twice(a)*magnitude(b)",
        );
        for (body, complex) in [
            ("twice(v(2))", true),
            ("twice(vr(2))", false),
            ("magnitude(v(2))", false),
            ("probe(1)", true),
            ("outer(v(1), 1)", true),
            ("outer(1, v(1))", false),
        ] {
            assert_eq!(is_complex(body, &params), complex, "{body}");
        }
    }
}
