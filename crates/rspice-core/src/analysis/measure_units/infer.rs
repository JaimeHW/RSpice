use std::collections::HashMap;

use super::{MeasurementUnit, MeasurementUnits};
use crate::netlist::expr::{BinOpKind, Expr, UnaryOpKind, parse_expression};
use crate::netlist::measure::{ExtremaOutput, MeasureStatement, MeasureType};

/// Infer from actual statement syntax and producer-supplied signal/axis units.
/// Parameters without declared physical units and cyclic references stay unknown.
pub fn measurement_units(
    statements: &[&MeasureStatement],
    axis: Option<&MeasurementUnit>,
    signals: &HashMap<String, MeasurementUnit>,
) -> Vec<MeasurementUnits> {
    let mut inference = Inference {
        statements,
        axis,
        signals: signals
            .iter()
            .map(|(name, unit)| (name.to_ascii_uppercase(), unit.clone()))
            .collect(),
        names: statements
            .iter()
            .enumerate()
            .map(|(i, statement)| (statement.name.to_ascii_uppercase(), i))
            .collect(),
        units: vec![None; statements.len()],
        visiting: vec![false; statements.len()],
    };
    (0..statements.len())
        .map(|index| inference.statement(index, 0))
        .collect()
}

struct Inference<'a> {
    statements: &'a [&'a MeasureStatement],
    axis: Option<&'a MeasurementUnit>,
    signals: HashMap<String, MeasurementUnit>,
    names: HashMap<String, usize>,
    units: Vec<Option<MeasurementUnits>>,
    visiting: Vec<bool>,
}

fn known(symbol: &str) -> MeasurementUnit {
    MeasurementUnit::Known(symbol.to_owned())
}
fn unknown() -> MeasurementUnits {
    MeasurementUnits {
        value: MeasurementUnit::Unknown,
        raw_value: MeasurementUnit::Unknown,
        axis: MeasurementUnit::Unknown,
    }
}

impl Inference<'_> {
    fn statement(&mut self, index: usize, depth: usize) -> MeasurementUnits {
        if let Some(unit) = &self.units[index] {
            return unit.clone();
        }
        if depth > 128 || self.visiting[index] {
            return unknown();
        }
        self.visiting[index] = true;
        let statement = self.statements[index];
        let axis = self.axis.cloned().unwrap_or_else(|| {
            match statement.analysis.to_ascii_uppercase().as_str() {
                "TRAN" | "TRAN_CONT" => known("s"),
                "AC" | "AC_CONT" | "NOISE" | "NOISE_CONT" => known("Hz"),
                _ => MeasurementUnit::Unknown,
            }
        });
        let raw_value = match &statement.measure_type {
            MeasureType::When { .. } => axis.clone(),
            MeasureType::Delay { .. }
            | MeasureType::RiseTime { .. }
            | MeasureType::FallTime { .. } => axis.interval(),
            MeasureType::Min { signal, .. }
            | MeasureType::Max { signal, .. }
            | MeasureType::Avg { signal, .. }
            | MeasureType::Rms { signal, .. }
            | MeasureType::Find { signal, .. } => self.signal(signal, depth + 1, &axis),
            MeasureType::PeakToPeak { signal, .. } | MeasureType::FileError { signal, .. } => {
                self.signal(signal, depth + 1, &axis).interval()
            }
            MeasureType::Integ { signal, .. } => self
                .signal(signal, depth + 1, &axis)
                .product(&axis.interval(), false),
            MeasureType::Derivative { signal, .. } => self
                .signal(signal, depth + 1, &axis)
                .interval()
                .product(&axis.interval(), true),
            MeasureType::Param { expression } | MeasureType::Equation { expression, .. } => {
                self.signal(&expression.text, depth + 1, &axis)
            }
            MeasureType::ErrorFunction { .. } => known("1"),
        };
        let value = match statement.measure_type {
            MeasureType::Min {
                output: ExtremaOutput::IndependentAxis,
                ..
            }
            | MeasureType::Max {
                output: ExtremaOutput::IndependentAxis,
                ..
            } => axis.clone(),
            _ => raw_value.clone(),
        };
        let units = MeasurementUnits {
            value,
            raw_value,
            axis,
        };
        self.visiting[index] = false;
        self.units[index] = Some(units.clone());
        units
    }

    fn signal(&mut self, text: &str, depth: usize, axis: &MeasurementUnit) -> MeasurementUnit {
        if depth > 128 {
            return MeasurementUnit::Unknown;
        }
        let text = text.trim();
        let folded = text.to_ascii_uppercase();
        if let Some(unit) = self.signals.get(&folded) {
            return unit.clone();
        }
        if let Some(index) = self.names.get(&folded) {
            return self.statement(*index, depth + 1).value;
        }
        if let Some(body) = text.strip_prefix('@')
            && let Some((_, quantity)) = body
                .strip_suffix(']')
                .and_then(|body| body.rsplit_once('['))
        {
            return match quantity.to_ascii_lowercase().as_str() {
                "i" | "id" | "ig" | "is" | "ib" | "ic" | "ie" | "isub" => known("A"),
                "gm" | "gds" | "gmb" | "gmbs" | "go" => known("S"),
                "vgs" | "vds" | "vbs" | "vbe" | "vce" | "vbc" => known("V"),
                "power" | "p" => known("W"),
                _ => MeasurementUnit::Unknown,
            };
        }
        let text = text
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .or_else(|| text.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
            .unwrap_or(text);
        match parse_expression(text) {
            Ok(expression) => self.expression(&expression, depth + 1, axis),
            Err(_) => MeasurementUnit::Unknown,
        }
    }

    fn expression(
        &mut self,
        expression: &Expr,
        depth: usize,
        axis: &MeasurementUnit,
    ) -> MeasurementUnit {
        if depth > 128 {
            return MeasurementUnit::Unknown;
        }
        match expression {
            Expr::Number(_) | Expr::ComplexNumber(_) => known("1"),
            Expr::StringLiteral(_) => MeasurementUnit::Unknown,
            Expr::Param(name) => {
                let folded = name.to_ascii_uppercase();
                if let Some(unit) = self.signals.get(&folded) {
                    return unit.clone();
                }
                if let Some(index) = self.names.get(&folded) {
                    return self.statement(*index, depth + 1).value;
                }
                match folded.as_str() {
                    "TIME" => axis.clone(),
                    "FREQ" | "FREQUENCY" => known("Hz"),
                    "TEMP" | "TEMPER" => known("degC"),
                    "PI" | "E" => known("1"),
                    _ => MeasurementUnit::Unknown,
                }
            }
            Expr::UnaryOp {
                op: UnaryOpKind::Not,
                ..
            } => known("1"),
            Expr::UnaryOp { operand, .. } => self.expression(operand, depth + 1, axis),
            Expr::BinOp { op, left, right } => {
                let a = self.expression(left, depth + 1, axis);
                let b = self.expression(right, depth + 1, axis);
                match op {
                    BinOpKind::Mul => a.product(&b, false),
                    BinOpKind::Div => a.product(&b, true),
                    BinOpKind::Pow => {
                        numeric(right).map_or(MeasurementUnit::Unknown, |power| a.power(power))
                    }
                    BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mod => {
                        if numeric(left) == Some(0.0) {
                            b
                        } else if numeric(right) == Some(0.0) || same_scale(&a, &b) {
                            a
                        } else {
                            MeasurementUnit::Unknown
                        }
                    }
                    BinOpKind::Gt
                    | BinOpKind::Lt
                    | BinOpKind::Ge
                    | BinOpKind::Le
                    | BinOpKind::Eq
                    | BinOpKind::Ne
                    | BinOpKind::And
                    | BinOpKind::Or => known("1"),
                }
            }
            Expr::FnCall { name, args } => {
                let name = name.to_ascii_uppercase();
                // A producer can type device-noise probes and other quantities
                // whose physical dimensions are not defined by probe syntax.
                if let Some(arguments) = args
                    .iter()
                    .map(|arg| match arg {
                        Expr::Param(name) | Expr::StringLiteral(name) => {
                            Some(name.to_ascii_uppercase())
                        }
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>()
                    && let Some(unit) = self
                        .signals
                        .get(&format!("{name}({})", arguments.join(",")))
                {
                    return unit.clone();
                }
                if let Some(unit) = probe_unit(&name, args.len()) {
                    return unit;
                }
                let units = args
                    .iter()
                    .map(|arg| self.expression(arg, depth + 1, axis))
                    .collect::<Vec<_>>();
                let first = units.first().cloned().unwrap_or(MeasurementUnit::Unknown);
                match name.as_str() {
                    "ABS" | "MAG" | "MAGNITUDE" | "REAL" | "RE" | "IMAG" | "IM" | "CEIL"
                    | "FLOOR" | "ROUND" | "NINT" | "URAMP" => first,
                    "PH" | "PHASE" => known("deg"),
                    "DB" => known("dB"),
                    "ASIN" | "ACOS" | "ATAN" | "ATAN2" | "ARCSIN" | "ARCCOS" | "ARCTAN" => {
                        known("rad")
                    }
                    "SIN" | "COS" | "TAN" | "SINH" | "COSH" | "TANH" | "EXP" | "LN" | "LOG"
                    | "LOG10" | "SGN" | "SIGN" | "U" => known("1"),
                    "SQRT" => first.power(0.5),
                    "POW" | "PWR" => args
                        .get(1)
                        .and_then(numeric)
                        .map_or(MeasurementUnit::Unknown, |power| first.power(power)),
                    "MIN" | "MAX" if units.iter().all(|unit| same_scale(unit, &first)) => first,
                    "IF" if units.len() == 3 && same_scale(&units[1], &units[2]) => {
                        units[1].clone()
                    }
                    _ => MeasurementUnit::Unknown,
                }
            }
        }
    }
}

fn numeric(expression: &Expr) -> Option<f64> {
    match expression {
        Expr::Number(value) => Some(*value),
        Expr::UnaryOp {
            op: UnaryOpKind::Neg,
            operand,
        } => numeric(operand).map(|value| -value),
        Expr::UnaryOp {
            op: UnaryOpKind::Pos,
            operand,
        } => numeric(operand),
        _ => None,
    }
}

fn same_scale(a: &MeasurementUnit, b: &MeasurementUnit) -> bool {
    a.parsed().zip(b.parsed()).is_some_and(|(a, b)| a == b)
}

fn probe_unit(name: &str, arguments: usize) -> Option<MeasurementUnit> {
    if matches!(name, "V" | "VM" | "VR" | "VI") && matches!(arguments, 1 | 2) {
        return Some(known("V"));
    }
    if name == "VP" && matches!(arguments, 1 | 2) {
        return Some(known("deg"));
    }
    if name == "VDB" && matches!(arguments, 1 | 2) {
        return Some(known("dB"));
    }
    if crate::netlist::is_current_output_accessor(name) && arguments == 1 {
        return Some(known(match name {
            "IP" => "deg",
            "IDB" => "dB",
            _ => "A",
        }));
    }
    if matches!(name, "P" | "W") && arguments == 1 {
        return Some(known("W"));
    }
    if arguments == 2 {
        let (family, suffix) = name.split_at_checked(1)?;
        if matches!(family, "S" | "Y" | "Z") {
            return Some(known(match suffix {
                "P" => "deg",
                "DB" => "dB",
                "" | "R" | "I" | "M" => match family {
                    "S" => "1",
                    "Y" => "S",
                    _ => "ohm",
                },
                _ => return None,
            }));
        }
    }
    None
}
