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
    BehavioralPreparationError, BinOpKind, ComplexValue, Expr, ExprError,
    ExpressionEvaluationError, ParseExpressionWithAbortError, PreparedExpression, UnaryOpKind,
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
    let protected = params
        .all_params()
        .into_iter()
        .map(|(name, _)| name.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let expanded = match crate::netlist::expr::expand_output_user_functions_with_abort(
        body, params, &protected, abort,
    ) {
        Ok(expanded) => expanded,
        Err(BehavioralPreparationError::Aborted) => return Err(SimulationError::Aborted),
        Err(BehavioralPreparationError::Semantic(_)) => return Ok(None),
    };
    let expression = match crate::netlist::expr::parse_xyce_expression_with_abort(&expanded, abort)
    {
        Ok(expression) => expression,
        Err(ParseExpressionWithAbortError::Aborted) => return Err(SimulationError::Aborted),
        Err(ParseExpressionWithAbortError::Parse(_)) => return Ok(None),
    };

    let mut phasors: Vec<Vec<ComplexValue>> = Vec::new();
    let mut keys: HashMap<String, usize> = HashMap::new();
    let mut unavailable = None;
    let compiled = PreparedExpression::compile_with_bindings(
        &expression,
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
    let columns = if is_complex(&expression) {
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
/// carry their operands' type.
fn is_complex(expression: &Expr) -> bool {
    let mut pending = vec![expression];
    while let Some(node) = pending.pop() {
        match node {
            Expr::Number(_) | Expr::StringLiteral(_) | Expr::Param(_) => {}
            Expr::ComplexNumber(value) => {
                if value.im != 0.0 {
                    return true;
                }
            }
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
                "V" | "I" => return true,
                lead if is_lead_accessor(lead) => return true,
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
    false
}

#[cfg(test)]
mod tests {
    use super::is_complex;

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
            let expression = crate::netlist::expr::parse_expression(body).expect("parses");
            assert_eq!(is_complex(&expression), complex, "{body}");
        }
    }
}
