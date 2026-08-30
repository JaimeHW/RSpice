use std::collections::{HashMap, HashSet};

use smol_str::SmolStr;

use crate::integer_runtime::{IntegerBinaryOperation, integer_binary};
use crate::numeric_literal::{exact_integer_as_f64, parse_integer_literal};
use crate::semantic::{MAX_PARAMETER_ARRAY_ELEMENTS, MAX_PARAMETER_ARRAY_RANK};

use super::{
    CanonicalValueType, CompilerPhase, ExprId, HirExprKind, HirExprRef, HirExpression,
    HirParamRange, HirParameter, HirParameterDimension, IrDiagnostic, MirParameterSlot,
};

const MAX_PARAMETER_ARRAY_DIAGNOSTICS: usize = 64;
const MAX_CONSTANT_EXPRESSION_DEPTH: usize = 128;
const MAX_CONSTANT_EVALUATION_WORK: usize = 4_194_304;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConstantValue {
    Integer(i64),
    Real(f64),
}

impl ConstantValue {
    fn as_f64(self) -> f64 {
        match self {
            Self::Integer(value) => value as f64,
            Self::Real(value) => value,
        }
    }

    fn is_truthy(self) -> bool {
        self.as_f64() != 0.0
    }
}

pub(super) trait CanonicalParameterMetadata {
    fn name(&self) -> &SmolStr;
    fn value_type(&self) -> CanonicalValueType;
    fn dimensions(&self) -> &[HirParameterDimension];
    fn default(&self) -> Option<f64>;
    fn default_expr(&self) -> Option<&HirExprRef>;
    fn range(&self) -> Option<&HirParamRange>;
}

impl CanonicalParameterMetadata for HirParameter {
    fn name(&self) -> &SmolStr {
        &self.name
    }

    fn value_type(&self) -> CanonicalValueType {
        self.value_type
    }

    fn dimensions(&self) -> &[HirParameterDimension] {
        &self.dimensions
    }

    fn default(&self) -> Option<f64> {
        self.default
    }

    fn default_expr(&self) -> Option<&HirExprRef> {
        self.default_expr.as_ref()
    }

    fn range(&self) -> Option<&HirParamRange> {
        self.range.as_ref()
    }
}

impl CanonicalParameterMetadata for MirParameterSlot {
    fn name(&self) -> &SmolStr {
        &self.name
    }

    fn value_type(&self) -> CanonicalValueType {
        self.value_type
    }

    fn dimensions(&self) -> &[HirParameterDimension] {
        &self.dimensions
    }

    fn default(&self) -> Option<f64> {
        self.default
    }

    fn default_expr(&self) -> Option<&HirExprRef> {
        self.default_expr.as_ref()
    }

    fn range(&self) -> Option<&HirParamRange> {
        self.range.as_ref()
    }
}

pub(super) fn validate_parameter_array_contract<P: CanonicalParameterMetadata>(
    phase: CompilerPhase,
    parameters: &[P],
    expressions: &[HirExpression],
) -> Vec<IrDiagnostic> {
    // Scalar defaults are evaluated here only to authenticate the constants
    // consumed by parameter-array bounds and initializers.  Scalar-only
    // artifacts retain their independent backend parameter-contract checks;
    // applying the array dependency validator to them would collapse those
    // boundaries and reject an artifact before a backend can authenticate it
    // against the executable model supplied alongside it.
    if !parameters
        .iter()
        .any(|parameter| !parameter.dimensions().is_empty())
    {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    let mut declared_scalars = HashMap::<SmolStr, ConstantValue>::new();
    let mut remaining_work = expressions
        .len()
        .saturating_mul(4)
        .clamp(1_024, MAX_CONSTANT_EVALUATION_WORK);

    for parameter in parameters {
        if diagnostics.len() >= MAX_PARAMETER_ARRAY_DIAGNOSTICS {
            break;
        }
        if parameter.dimensions().is_empty() {
            if matches!(
                parameter.value_type(),
                CanonicalValueType::Real | CanonicalValueType::Integer
            ) {
                let expression_value = parameter.default_expr().map(|expression| {
                    ConstantEvaluator::new(expressions, &declared_scalars, &mut remaining_work)
                        .evaluate(expression.id)
                });
                let value = match (parameter.default(), expression_value) {
                    (Some(folded), Some(Ok(expression)))
                        if folded.to_bits() != expression.as_f64().to_bits() =>
                    {
                        let reference = parameter.default_expr().expect("expression is present");
                        diagnostics.push(IrDiagnostic::error(
                            phase,
                            format!(
                                "scalar parameter '{}' folded default {folded} contradicts its default expression value {}",
                                parameter.name(),
                                expression.as_f64(),
                            ),
                            reference.span,
                        ));
                        None
                    }
                    (Some(folded), Some(Ok(_))) | (Some(folded), None) => Some(folded),
                    (None, Some(Ok(expression))) => Some(expression.as_f64()),
                    // An unevaluable expression is intentionally not replaced
                    // by potentially contradictory folded metadata. Any array
                    // that depends on it will then fail closed at its bound or
                    // initializer reference.
                    (_, Some(Err(_))) | (None, None) => None,
                };
                if let Some(value) = value.filter(|value| value.is_finite()) {
                    let value = if parameter.value_type() == CanonicalValueType::Integer {
                        exact_i64(value).map(ConstantValue::Integer)
                    } else {
                        Some(ConstantValue::Real(value))
                    };
                    if let Some(value) = value {
                        declared_scalars.insert(parameter.name().clone(), value);
                    }
                }
            }
            continue;
        }

        let context = format!("parameter array '{}'", parameter.name());
        if parameter.dimensions().len() > MAX_PARAMETER_ARRAY_RANK {
            diagnostics.push(IrDiagnostic::global_error(
                phase,
                format!(
                    "{context} has rank {}; the supported safety limit is {MAX_PARAMETER_ARRAY_RANK}",
                    parameter.dimensions().len()
                ),
            ));
            continue;
        }
        if !matches!(
            parameter.value_type(),
            CanonicalValueType::Real | CanonicalValueType::Integer
        ) {
            diagnostics.push(IrDiagnostic::global_error(
                phase,
                format!("{context} must have real or integer elements"),
            ));
        }
        if parameter.default().is_some() {
            diagnostics.push(IrDiagnostic::global_error(
                phase,
                format!("{context} must not carry a folded scalar default"),
            ));
        }
        if parameter.range().is_some() {
            diagnostics.push(IrDiagnostic::global_error(
                phase,
                format!("{context} may not carry scalar range metadata"),
            ));
        }

        let Some(default_expr) = parameter.default_expr() else {
            diagnostics.push(IrDiagnostic::global_error(
                phase,
                format!("{context} requires a constant assignment-pattern default"),
            ));
            continue;
        };
        let Some(default_node) = expressions.get(usize::from(default_expr.id)) else {
            continue;
        };
        if !matches!(
            default_node.kind,
            HirExprKind::ArrayLiteral {
                assignment_pattern: true,
                ..
            }
        ) {
            diagnostics.push(IrDiagnostic::error(
                phase,
                format!("{context} default must be an assignment pattern"),
                default_expr.span,
            ));
            continue;
        }

        let mut evaluator =
            ConstantEvaluator::new(expressions, &declared_scalars, &mut remaining_work);
        let mut shape = Vec::with_capacity(parameter.dimensions().len());
        let mut total_elements = 1_u64;
        let mut shape_valid = true;
        for (index, dimension) in parameter.dimensions().iter().enumerate() {
            let left = eval_integer_bound(
                &context,
                index,
                "left",
                &dimension.left,
                &mut evaluator,
                phase,
                &mut diagnostics,
            );
            let right = eval_integer_bound(
                &context,
                index,
                "right",
                &dimension.right,
                &mut evaluator,
                phase,
                &mut diagnostics,
            );
            let (Some(left), Some(right)) = (left, right) else {
                shape_valid = false;
                continue;
            };
            let Some(extent) = left.abs_diff(right).checked_add(1) else {
                diagnostics.push(IrDiagnostic::error(
                    phase,
                    format!(
                        "{context} dimension {} extent is unrepresentable",
                        index + 1
                    ),
                    dimension.span,
                ));
                shape_valid = false;
                continue;
            };
            let Some(next_total) = total_elements.checked_mul(extent) else {
                diagnostics.push(IrDiagnostic::error(
                    phase,
                    format!("{context} element count overflows canonical shape storage"),
                    dimension.span,
                ));
                shape_valid = false;
                continue;
            };
            if next_total > MAX_PARAMETER_ARRAY_ELEMENTS {
                diagnostics.push(IrDiagnostic::error(
                    phase,
                    format!(
                        "{context} declares {next_total} elements; the supported safety limit is {MAX_PARAMETER_ARRAY_ELEMENTS}"
                    ),
                    dimension.span,
                ));
                shape_valid = false;
                continue;
            }
            total_elements = next_total;
            shape.push(extent);
        }

        if shape_valid {
            validate_initializer_shape(
                &context,
                default_expr.id,
                &shape,
                0,
                parameter.value_type(),
                &mut evaluator,
                phase,
                &mut diagnostics,
            );
        }
    }

    diagnostics
}

fn eval_integer_bound(
    context: &str,
    dimension: usize,
    side: &str,
    expression: &HirExprRef,
    evaluator: &mut ConstantEvaluator<'_, '_>,
    phase: CompilerPhase,
    diagnostics: &mut Vec<IrDiagnostic>,
) -> Option<i64> {
    let owner = format!("{context} dimension {} {side} bound", dimension + 1);
    let value = match evaluator.evaluate(expression.id) {
        Ok(value) => value,
        Err(detail) => {
            diagnostics.push(IrDiagnostic::error(
                phase,
                format!("{owner} is not a valid earlier-scalar constant expression: {detail}"),
                expression.span,
            ));
            return None;
        }
    };
    let Some(value) = exact_i64(value.as_f64()) else {
        diagnostics.push(IrDiagnostic::error(
            phase,
            format!("{owner} does not resolve to a finite signed 64-bit integer"),
            expression.span,
        ));
        return None;
    };
    Some(value)
}

#[allow(clippy::too_many_arguments)]
fn validate_initializer_shape(
    context: &str,
    expression: ExprId,
    shape: &[u64],
    dimension: usize,
    value_type: CanonicalValueType,
    evaluator: &mut ConstantEvaluator<'_, '_>,
    phase: CompilerPhase,
    diagnostics: &mut Vec<IrDiagnostic>,
) {
    if diagnostics.len() >= MAX_PARAMETER_ARRAY_DIAGNOSTICS {
        return;
    }
    let Some(node) = evaluator.expressions.get(usize::from(expression)) else {
        return;
    };
    let HirExprKind::ArrayLiteral {
        elements,
        assignment_pattern,
    } = &node.kind
    else {
        diagnostics.push(IrDiagnostic::error(
            phase,
            format!(
                "{context} dimension {} contains a scalar before the final dimension",
                dimension + 1
            ),
            node.span,
        ));
        return;
    };
    if !assignment_pattern {
        diagnostics.push(IrDiagnostic::error(
            phase,
            format!(
                "{context} dimension {} contains a concatenation instead of an assignment pattern",
                dimension + 1
            ),
            node.span,
        ));
        return;
    }
    let Some(&expected_len) = shape.get(dimension) else {
        diagnostics.push(IrDiagnostic::error(
            phase,
            format!(
                "{context} initializer exceeds declared rank {}",
                shape.len()
            ),
            node.span,
        ));
        return;
    };
    if u64::try_from(elements.len()).ok() != Some(expected_len) {
        diagnostics.push(IrDiagnostic::error(
            phase,
            format!(
                "{context} dimension {} has {} elements; expected {expected_len}",
                dimension + 1,
                elements.len()
            ),
            node.span,
        ));
        return;
    }

    let is_leaf = dimension + 1 == shape.len();
    for element in elements {
        if diagnostics.len() >= MAX_PARAMETER_ARRAY_DIAGNOSTICS {
            return;
        }
        if is_leaf {
            let Some(element_node) = evaluator.expressions.get(usize::from(*element)) else {
                continue;
            };
            if matches!(element_node.kind, HirExprKind::ArrayLiteral { .. }) {
                diagnostics.push(IrDiagnostic::error(
                    phase,
                    format!(
                        "{context} dimension {} contains an unexpected nested assignment pattern",
                        dimension + 1
                    ),
                    element_node.span,
                ));
                continue;
            }
            let element_span = element_node.span;
            match evaluator.evaluate(*element) {
                Ok(value)
                    if value.as_f64().is_finite()
                        && (value_type != CanonicalValueType::Integer
                            || (value.as_f64().fract() == 0.0
                                && value.as_f64() >= f64::from(i32::MIN)
                                && value.as_f64() <= f64::from(i32::MAX))) => {}
                Ok(value) => diagnostics.push(IrDiagnostic::error(
                    phase,
                    format!(
                        "{context} has invalid {value_type:?} element value {}",
                        value.as_f64()
                    ),
                    element_span,
                )),
                Err(detail) => diagnostics.push(IrDiagnostic::error(
                    phase,
                    format!("{context} element is not constant: {detail}"),
                    element_span,
                )),
            }
        } else {
            validate_initializer_shape(
                context,
                *element,
                shape,
                dimension + 1,
                value_type,
                evaluator,
                phase,
                diagnostics,
            );
        }
    }
}

struct ConstantEvaluator<'expressions, 'budget> {
    expressions: &'expressions [HirExpression],
    env: &'expressions HashMap<SmolStr, ConstantValue>,
    visiting: HashSet<ExprId>,
    cache: HashMap<ExprId, Result<ConstantValue, String>>,
    remaining_work: &'budget mut usize,
}

impl<'expressions, 'budget> ConstantEvaluator<'expressions, 'budget> {
    fn new(
        expressions: &'expressions [HirExpression],
        env: &'expressions HashMap<SmolStr, ConstantValue>,
        remaining_work: &'budget mut usize,
    ) -> Self {
        Self {
            expressions,
            env,
            visiting: HashSet::new(),
            cache: HashMap::new(),
            remaining_work,
        }
    }

    fn evaluate(&mut self, expression: ExprId) -> Result<ConstantValue, String> {
        self.evaluate_at_depth(expression, 0)
    }

    fn evaluate_at_depth(
        &mut self,
        expression: ExprId,
        depth: usize,
    ) -> Result<ConstantValue, String> {
        if let Some(cached) = self.cache.get(&expression) {
            return cached.clone();
        }
        if depth > MAX_CONSTANT_EXPRESSION_DEPTH {
            return Err(format!(
                "constant expression depth exceeds the safety limit of {MAX_CONSTANT_EXPRESSION_DEPTH}"
            ));
        }
        if *self.remaining_work == 0 {
            return Err(format!(
                "constant-expression validation exceeds the work limit of {MAX_CONSTANT_EVALUATION_WORK} nodes"
            ));
        }
        *self.remaining_work -= 1;
        if !self.visiting.insert(expression) {
            return Err(format!("cyclic expression reference {expression}"));
        }

        let result = self.evaluate_uncached(expression, depth);
        self.visiting.remove(&expression);
        self.cache.insert(expression, result.clone());
        result
    }

    fn evaluate_uncached(
        &mut self,
        expression: ExprId,
        depth: usize,
    ) -> Result<ConstantValue, String> {
        let kind = self
            .expressions
            .get(usize::from(expression))
            .ok_or_else(|| format!("expression {expression} is out of range"))?
            .kind
            .clone();
        let next_depth = depth + 1;
        match kind {
            HirExprKind::Number { value, raw } => match integer_literal_value(&raw)? {
                Some(integer)
                    if exact_integer_as_f64(integer)
                        .is_some_and(|exact| value.to_bits() == exact.to_bits()) =>
                {
                    Ok(ConstantValue::Integer(integer))
                }
                Some(integer) => Err(format!(
                    "integer literal raw text '{raw}' resolves to {integer}, which is not represented exactly by stored value {value}"
                )),
                None => Ok(ConstantValue::Real(value)),
            },
            HirExprKind::Identifier { name } => {
                self.env.get(&name).copied().ok_or_else(|| {
                    format!("identifier '{name}' is not an earlier scalar parameter")
                })
            }
            HirExprKind::Unary { op, operand } => {
                let value = self.evaluate_at_depth(operand, next_depth)?;
                match op.as_str() {
                    "Neg" => match value {
                        ConstantValue::Integer(value) => value
                            .checked_neg()
                            .map(ConstantValue::Integer)
                            .ok_or_else(|| "integer negation overflows".to_string()),
                        ConstantValue::Real(value) => Ok(ConstantValue::Real(-value)),
                    },
                    "Pos" => Ok(value),
                    "Not" => Ok(ConstantValue::Integer(i64::from(!value.is_truthy()))),
                    "BitNot" => {
                        let value = constant_integer(value)
                            .ok_or_else(|| "bitwise operand is not an integer".to_string())?;
                        let value = i32::try_from(value).map_err(|_| {
                            "bitwise operand is outside the signed 32-bit integer range".to_string()
                        })?;
                        Ok(ConstantValue::Integer(i64::from(!value)))
                    }
                    _ => Err(format!("unsupported unary operator '{op}'")),
                }
            }
            HirExprKind::Binary { op, left, right } => {
                let left = self.evaluate_at_depth(left, next_depth)?;
                let right = self.evaluate_at_depth(right, next_depth)?;
                match op.as_str() {
                    "Add" => constant_add(left, right),
                    "Sub" => constant_sub(left, right),
                    "Mul" => constant_mul(left, right),
                    "Div" => constant_div(left, right),
                    "Mod" => constant_mod(left, right),
                    "Pow" => constant_pow(left, right),
                    "Eq" => Ok(ConstantValue::Integer(i64::from(
                        left.as_f64() == right.as_f64(),
                    ))),
                    "Ne" => Ok(ConstantValue::Integer(i64::from(
                        left.as_f64() != right.as_f64(),
                    ))),
                    "Lt" => Ok(ConstantValue::Integer(i64::from(
                        left.as_f64() < right.as_f64(),
                    ))),
                    "Le" => Ok(ConstantValue::Integer(i64::from(
                        left.as_f64() <= right.as_f64(),
                    ))),
                    "Gt" => Ok(ConstantValue::Integer(i64::from(
                        left.as_f64() > right.as_f64(),
                    ))),
                    "Ge" => Ok(ConstantValue::Integer(i64::from(
                        left.as_f64() >= right.as_f64(),
                    ))),
                    "And" => Ok(ConstantValue::Integer(i64::from(
                        left.is_truthy() && right.is_truthy(),
                    ))),
                    "Or" => Ok(ConstantValue::Integer(i64::from(
                        left.is_truthy() || right.is_truthy(),
                    ))),
                    "Shl" | "Shr" => {
                        let value = constant_integer(left)
                            .ok_or_else(|| "shift value is not an integer".to_string())?;
                        let shift = constant_integer(right)
                            .ok_or_else(|| "shift count is not an integer".to_string())?;
                        let operation = if op == "Shl" {
                            IntegerBinaryOperation::Shl
                        } else {
                            IntegerBinaryOperation::Shr
                        };
                        integer_binary(operation, value as f64, shift as f64)
                            .map(|value| ConstantValue::Integer(value as i64))
                            .map_err(|error| error.to_string())
                    }
                    "BitAnd" | "BitOr" | "BitXor" => {
                        let left = constant_integer(left)
                            .ok_or_else(|| "bitwise operand is not an integer".to_string())?;
                        let right = constant_integer(right)
                            .ok_or_else(|| "bitwise operand is not an integer".to_string())?;
                        let operation = match op.as_str() {
                            "BitAnd" => IntegerBinaryOperation::BitAnd,
                            "BitOr" => IntegerBinaryOperation::BitOr,
                            _ => IntegerBinaryOperation::BitXor,
                        };
                        integer_binary(operation, left as f64, right as f64)
                            .map(|value| ConstantValue::Integer(value as i64))
                            .map_err(|error| error.to_string())
                    }
                    _ => Err(format!("unsupported binary operator '{op}'")),
                }
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let condition = self.evaluate_at_depth(condition, next_depth)?;
                if condition.is_truthy() {
                    self.evaluate_at_depth(then_expr, next_depth)
                } else {
                    self.evaluate_at_depth(else_expr, next_depth)
                }
            }
            HirExprKind::Call { name, args } => match (name.as_str(), args.as_slice()) {
                ("abs", [value]) => match self.evaluate_at_depth(*value, next_depth)? {
                    ConstantValue::Integer(value) => value
                        .checked_abs()
                        .map(ConstantValue::Integer)
                        .ok_or_else(|| "integer absolute value overflows".to_string()),
                    ConstantValue::Real(value) => Ok(ConstantValue::Real(value.abs())),
                },
                ("sqrt", [value]) => Ok(ConstantValue::Real(
                    self.evaluate_at_depth(*value, next_depth)?.as_f64().sqrt(),
                )),
                ("exp", [value]) => Ok(ConstantValue::Real(
                    self.evaluate_at_depth(*value, next_depth)?.as_f64().exp(),
                )),
                ("ln" | "log", [value]) => Ok(ConstantValue::Real(
                    self.evaluate_at_depth(*value, next_depth)?.as_f64().ln(),
                )),
                ("log10", [value]) => Ok(ConstantValue::Real(
                    self.evaluate_at_depth(*value, next_depth)?.as_f64().log10(),
                )),
                ("floor", [value]) => Ok(ConstantValue::Real(
                    self.evaluate_at_depth(*value, next_depth)?.as_f64().floor(),
                )),
                ("ceil", [value]) => Ok(ConstantValue::Real(
                    self.evaluate_at_depth(*value, next_depth)?.as_f64().ceil(),
                )),
                ("min", [left, right]) => Ok(constant_min(
                    self.evaluate_at_depth(*left, next_depth)?,
                    self.evaluate_at_depth(*right, next_depth)?,
                )),
                ("max", [left, right]) => Ok(constant_max(
                    self.evaluate_at_depth(*left, next_depth)?,
                    self.evaluate_at_depth(*right, next_depth)?,
                )),
                ("pow", [left, right]) => Ok(ConstantValue::Real(
                    self.evaluate_at_depth(*left, next_depth)?
                        .as_f64()
                        .powf(self.evaluate_at_depth(*right, next_depth)?.as_f64()),
                )),
                _ => Err(format!(
                    "call '{name}' is not a supported constant function"
                )),
            },
            other => Err(format!(
                "expression kind '{}' is not a parameter constant",
                expression_kind_name(&other)
            )),
        }
    }
}

fn integer_literal_value(raw: &str) -> Result<Option<i64>, String> {
    // Built-in constants are lowered to numeric HIR nodes with their symbolic
    // spelling retained as `raw`. Only source spellings that can begin a
    // numeric literal are parsed here.
    if !raw
        .trim_start()
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit() || character == '\'')
    {
        return Ok(None);
    }
    parse_integer_literal(raw)
}

fn constant_integer(value: ConstantValue) -> Option<i64> {
    match value {
        ConstantValue::Integer(value) => Some(value),
        ConstantValue::Real(_) => None,
    }
}

fn constant_add(left: ConstantValue, right: ConstantValue) -> Result<ConstantValue, String> {
    match (left, right) {
        (ConstantValue::Integer(left), ConstantValue::Integer(right)) => left
            .checked_add(right)
            .map(ConstantValue::Integer)
            .ok_or_else(|| "integer addition overflows".to_string()),
        _ => Ok(ConstantValue::Real(left.as_f64() + right.as_f64())),
    }
}

fn constant_sub(left: ConstantValue, right: ConstantValue) -> Result<ConstantValue, String> {
    match (left, right) {
        (ConstantValue::Integer(left), ConstantValue::Integer(right)) => left
            .checked_sub(right)
            .map(ConstantValue::Integer)
            .ok_or_else(|| "integer subtraction overflows".to_string()),
        _ => Ok(ConstantValue::Real(left.as_f64() - right.as_f64())),
    }
}

fn constant_mul(left: ConstantValue, right: ConstantValue) -> Result<ConstantValue, String> {
    match (left, right) {
        (ConstantValue::Integer(left), ConstantValue::Integer(right)) => left
            .checked_mul(right)
            .map(ConstantValue::Integer)
            .ok_or_else(|| "integer multiplication overflows".to_string()),
        _ => Ok(ConstantValue::Real(left.as_f64() * right.as_f64())),
    }
}

fn constant_div(left: ConstantValue, right: ConstantValue) -> Result<ConstantValue, String> {
    match (left, right) {
        (ConstantValue::Integer(left), ConstantValue::Integer(right)) => left
            .checked_div(right)
            .map(ConstantValue::Integer)
            .ok_or_else(|| "integer division by zero or overflow".to_string()),
        _ => Ok(ConstantValue::Real(left.as_f64() / right.as_f64())),
    }
}

fn constant_mod(left: ConstantValue, right: ConstantValue) -> Result<ConstantValue, String> {
    match (left, right) {
        (ConstantValue::Integer(left), ConstantValue::Integer(right)) => left
            .checked_rem(right)
            .map(ConstantValue::Integer)
            .ok_or_else(|| "integer modulus by zero or overflow".to_string()),
        _ => Ok(ConstantValue::Real(left.as_f64() % right.as_f64())),
    }
}

fn constant_pow(left: ConstantValue, right: ConstantValue) -> Result<ConstantValue, String> {
    match (left, right) {
        (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
            let exponent = u32::try_from(right)
                .map_err(|_| "integer exponent must be nonnegative and fit u32".to_string())?;
            left.checked_pow(exponent)
                .map(ConstantValue::Integer)
                .ok_or_else(|| "integer exponentiation overflows".to_string())
        }
        _ => Ok(ConstantValue::Real(left.as_f64().powf(right.as_f64()))),
    }
}

fn constant_min(left: ConstantValue, right: ConstantValue) -> ConstantValue {
    match (left, right) {
        (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
            ConstantValue::Integer(left.min(right))
        }
        _ => ConstantValue::Real(left.as_f64().min(right.as_f64())),
    }
}

fn constant_max(left: ConstantValue, right: ConstantValue) -> ConstantValue {
    match (left, right) {
        (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
            ConstantValue::Integer(left.max(right))
        }
        _ => ConstantValue::Real(left.as_f64().max(right.as_f64())),
    }
}

fn exact_i64(value: f64) -> Option<i64> {
    (value.is_finite()
        && value.fract() == 0.0
        && value >= i64::MIN as f64
        && value < 9_223_372_036_854_775_808.0)
        .then_some(value as i64)
}

fn expression_kind_name(kind: &HirExprKind) -> &'static str {
    match kind {
        HirExprKind::NullArgument => "null_argument",
        HirExprKind::Number { .. } => "number",
        HirExprKind::StringLiteral { .. } => "string_literal",
        HirExprKind::Identifier { .. } => "identifier",
        HirExprKind::SystemFunction { .. } => "system_function",
        HirExprKind::Binary { .. } => "binary",
        HirExprKind::Unary { .. } => "unary",
        HirExprKind::Conditional { .. } => "conditional",
        HirExprKind::Call { .. } => "call",
        HirExprKind::BranchAccess { .. } => "branch_access",
        HirExprKind::NamedBranchAccess { .. } => "named_branch_access",
        HirExprKind::ArrayAccess { .. } => "array_access",
        HirExprKind::ArrayLiteral { .. } => "array_literal",
        HirExprKind::AnalogOperator { .. } => "analog_operator",
        HirExprKind::Laplace { .. } => "laplace",
        HirExprKind::Zi { .. } => "zi",
        HirExprKind::NoiseSource { .. } => "noise_source",
    }
}
