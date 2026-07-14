use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use super::{
    BranchUnknownId, CompilerPhase, EquationId, ExprId, HirAnalogOperator, HirExprKind, HirExprRef,
    HirExpression, HirModel, IrDiagnostic, MirEquation, MirEquationKind, MirModel, NodeId,
    NoiseSourceId, SourceSpanRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalNoiseSourceKind {
    White,
    Flicker,
    Table,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalNoiseEndpoint {
    pub node: Option<NodeId>,
    pub name: SmolStr,
    pub is_internal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalNoiseTable {
    pub operands: Vec<HirExprRef>,
    pub log_interp: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalNoiseSource {
    pub id: NoiseSourceId,
    pub equation: EquationId,
    pub kind: CanonicalNoiseSourceKind,
    pub label: Option<SmolStr>,
    pub mechanism: SmolStr,
    pub is_current: bool,
    pub branch_ordinal: Option<BranchUnknownId>,
    pub pos: CanonicalNoiseEndpoint,
    pub neg: CanonicalNoiseEndpoint,
    pub psd: HirExprRef,
    pub exponent: Option<HirExprRef>,
    pub table: Option<CanonicalNoiseTable>,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CanonicalNoiseSourcePlan {
    pub sources: Vec<CanonicalNoiseSource>,
}

impl CanonicalNoiseSourcePlan {
    pub fn from_hir_and_mir(
        hir: &mut HirModel,
        mir: &mut MirModel,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        let mut sources = Vec::new();
        let mut diagnostics = Vec::new();

        for equation in mir.equations.clone() {
            if let Err(diagnostic) = extract_equation(hir, mir, &equation, &mut sources) {
                diagnostics.push(diagnostic);
            }
        }

        for (index, source) in sources.iter_mut().enumerate() {
            source.id = NoiseSourceId::from(index);
        }

        if diagnostics.is_empty() {
            mir.expressions.clone_from(&hir.expressions);
            Ok(Self { sources })
        } else {
            Err(diagnostics)
        }
    }

    pub fn diagnostics(&self, hir: &HirModel, mir: &MirModel) -> Vec<IrDiagnostic> {
        let mut diagnostics = Vec::new();
        for (index, source) in self.sources.iter().enumerate() {
            if usize::from(source.id) != index {
                diagnostics.push(plan_error(format!(
                    "canonical noise source id {} is not dense at index {index}",
                    source.id
                )));
            }
            let Some(equation) = mir.equations.get(usize::from(source.equation)) else {
                diagnostics.push(plan_error(format!(
                    "canonical noise source {} references missing equation {}",
                    source.id, source.equation
                )));
                continue;
            };
            if source.pos.node != equation.branch.pos_node
                || source.neg.node != equation.branch.neg_node
            {
                diagnostics.push(plan_error(format!(
                    "canonical noise source {} endpoints do not match equation {}",
                    source.id, source.equation
                )));
            }
            if source.is_current != matches!(equation.kind, MirEquationKind::Current) {
                diagnostics.push(plan_error(format!(
                    "canonical noise source {} contribution kind does not match equation {}",
                    source.id, source.equation
                )));
            }
            let expected_mechanism = canonical_mechanism(
                source.kind,
                &source.pos,
                &source.neg,
                source.label.as_deref(),
            );
            if source.mechanism != expected_mechanism {
                diagnostics.push(plan_error(format!(
                    "canonical noise source {} mechanism '{}' is stale; expected '{}'",
                    source.id, source.mechanism, expected_mechanism
                )));
            }
            let expected_ordinal = mir
                .branch_unknowns
                .iter()
                .find(|unknown| unknown.equation == source.equation)
                .map(|unknown| unknown.id);
            if source.branch_ordinal != expected_ordinal {
                diagnostics.push(plan_error(format!(
                    "canonical noise source {} branch ordinal does not match equation {}",
                    source.id, source.equation
                )));
            }
            match source.kind {
                CanonicalNoiseSourceKind::White
                    if source.exponent.is_some() || source.table.is_some() =>
                {
                    diagnostics.push(plan_error(format!(
                        "canonical white-noise source {} cannot carry exponent/table metadata",
                        source.id
                    )));
                }
                CanonicalNoiseSourceKind::Flicker
                    if source.exponent.is_none() || source.table.is_some() =>
                {
                    diagnostics.push(plan_error(format!(
                        "canonical flicker-noise source {} requires only exponent metadata",
                        source.id
                    )));
                }
                CanonicalNoiseSourceKind::Table
                    if source.exponent.is_some() || source.table.is_none() =>
                {
                    diagnostics.push(plan_error(format!(
                        "canonical table-noise source {} requires only table metadata",
                        source.id
                    )));
                }
                _ => {}
            }
            let mut refs = vec![&source.psd];
            refs.extend(source.exponent.iter());
            if let Some(table) = &source.table {
                refs.extend(&table.operands);
            }
            for expression in refs {
                if usize::from(expression.id) >= hir.expressions.len() {
                    diagnostics.push(plan_error(format!(
                        "canonical noise source {} references missing expression {}",
                        source.id, expression.id
                    )));
                }
            }
        }
        diagnostics
    }
}

fn plan_error(message: impl Into<String>) -> IrDiagnostic {
    IrDiagnostic::global_error(CompilerPhase::Artifact, message)
}

fn extract_equation(
    hir: &mut HirModel,
    mir: &MirModel,
    equation: &MirEquation,
    sources: &mut Vec<CanonicalNoiseSource>,
) -> Result<(), IrDiagnostic> {
    let branch_ordinal = mir
        .branch_unknowns
        .iter()
        .find(|unknown| unknown.equation == equation.id)
        .map(|unknown| unknown.id);
    let is_current = matches!(equation.kind, MirEquationKind::Current);
    let pos = endpoint(mir, equation.branch.pos_node);
    let neg = endpoint(mir, equation.branch.neg_node);
    if !contains_noise(hir, equation.expression.id) {
        return Ok(());
    }
    let amplitude = append_number(hir, 1.0, equation.expression.span);
    extract_expression(
        hir,
        equation.expression.id,
        amplitude,
        equation,
        is_current,
        branch_ordinal,
        &pos,
        &neg,
        sources,
    )
}

fn endpoint(mir: &MirModel, node: Option<NodeId>) -> CanonicalNoiseEndpoint {
    let Some(node) = node else {
        return CanonicalNoiseEndpoint {
            node: None,
            name: "0".into(),
            is_internal: false,
        };
    };
    let metadata = &mir.nodes[usize::from(node)];
    CanonicalNoiseEndpoint {
        node: Some(node),
        name: metadata.name.clone(),
        is_internal: !metadata.is_external,
    }
}

#[allow(clippy::too_many_arguments)]
fn extract_expression(
    hir: &mut HirModel,
    expr: ExprId,
    amplitude: HirExprRef,
    equation: &MirEquation,
    is_current: bool,
    branch_ordinal: Option<BranchUnknownId>,
    pos: &CanonicalNoiseEndpoint,
    neg: &CanonicalNoiseEndpoint,
    sources: &mut Vec<CanonicalNoiseSource>,
) -> Result<(), IrDiagnostic> {
    if !contains_noise(hir, expr) {
        return Ok(());
    }
    let expression = hir.expressions[usize::from(expr)].clone();
    let unsupported = |placement: &str| {
        IrDiagnostic::error(
            CompilerPhase::HirLowering,
            format!(
                "noise function in a {placement} (noise terms must enter contributions additively, optionally scaled)"
            ),
            expression.span,
        )
    };
    let mut push = |kind, label: Option<SmolStr>, psd, exponent, table| {
        let mechanism = canonical_mechanism(kind, pos, neg, label.as_deref());
        sources.push(CanonicalNoiseSource {
            id: NoiseSourceId::new(0),
            equation: equation.id,
            kind,
            label,
            mechanism,
            is_current,
            branch_ordinal,
            pos: pos.clone(),
            neg: neg.clone(),
            psd,
            exponent,
            table,
            span: expression.span,
        });
    };

    match &expression.kind {
        HirExprKind::Call { name, args } | HirExprKind::SystemFunction { name, args }
            if is_noise_call(name) =>
        {
            let mechanism = args.last().and_then(|id| string_literal(hir, *id));
            match name.trim_start_matches('$') {
                "white_noise" if !args.is_empty() => {
                    let squared = multiply(hir, amplitude.id, amplitude.id, expression.span);
                    let psd = multiply(hir, squared.id, args[0], expression.span);
                    push(CanonicalNoiseSourceKind::White, mechanism, psd, None, None);
                    Ok(())
                }
                "flicker_noise" if args.len() >= 2 => {
                    let squared = multiply(hir, amplitude.id, amplitude.id, expression.span);
                    let psd = multiply(hir, squared.id, args[0], expression.span);
                    push(
                        CanonicalNoiseSourceKind::Flicker,
                        mechanism,
                        psd,
                        Some(expr_ref(hir, args[1])),
                        None,
                    );
                    Ok(())
                }
                "noise_table" | "noise_table_log" if !args.is_empty() => {
                    let operands = match &hir.expressions[usize::from(args[0])].kind {
                        HirExprKind::ArrayLiteral { elements } => {
                            elements.iter().map(|id| expr_ref(hir, *id)).collect()
                        }
                        _ => vec![expr_ref(hir, args[0])],
                    };
                    let psd = multiply(hir, amplitude.id, amplitude.id, expression.span);
                    push(
                        CanonicalNoiseSourceKind::Table,
                        mechanism,
                        psd,
                        None,
                        Some(CanonicalNoiseTable {
                            operands,
                            log_interp: name.trim_start_matches('$') == "noise_table_log",
                        }),
                    );
                    Ok(())
                }
                _ => Err(IrDiagnostic::error(
                    CompilerPhase::HirLowering,
                    format!(
                        "canonical noise source '{}' has invalid operand count {}",
                        name,
                        args.len()
                    ),
                    expression.span,
                )),
            }
        }
        HirExprKind::NoiseSource {
            source,
            operands,
            name,
        } => match source.as_str() {
            "White" if operands.len() == 1 => {
                push(
                    CanonicalNoiseSourceKind::White,
                    name.clone(),
                    multiply(hir, amplitude.id, amplitude.id, expression.span),
                    None,
                    None,
                );
                let last = sources.last_mut().expect("source was just pushed");
                last.psd = multiply(hir, last.psd.id, operands[0], expression.span);
                Ok(())
            }
            "Flicker" if operands.len() == 2 => {
                push(
                    CanonicalNoiseSourceKind::Flicker,
                    name.clone(),
                    multiply(hir, amplitude.id, amplitude.id, expression.span),
                    Some(expr_ref(hir, operands[1])),
                    None,
                );
                let last = sources.last_mut().expect("source was just pushed");
                last.psd = multiply(hir, last.psd.id, operands[0], expression.span);
                Ok(())
            }
            "Table" if !operands.is_empty() => {
                push(
                    CanonicalNoiseSourceKind::Table,
                    name.clone(),
                    multiply(hir, amplitude.id, amplitude.id, expression.span),
                    None,
                    Some(CanonicalNoiseTable {
                        operands: operands.iter().map(|id| expr_ref(hir, *id)).collect(),
                        log_interp: false,
                    }),
                );
                Ok(())
            }
            _ => Err(IrDiagnostic::error(
                CompilerPhase::HirLowering,
                format!(
                    "canonical noise source '{}' has invalid operand count {}",
                    source,
                    operands.len()
                ),
                expression.span,
            )),
        },
        HirExprKind::Binary { op, left, right } if matches!(op.as_str(), "Add" | "Sub") => {
            extract_expression(
                hir,
                *left,
                amplitude.clone(),
                equation,
                is_current,
                branch_ordinal,
                pos,
                neg,
                sources,
            )?;
            extract_expression(
                hir,
                *right,
                amplitude,
                equation,
                is_current,
                branch_ordinal,
                pos,
                neg,
                sources,
            )
        }
        HirExprKind::Binary { op, left, right } if op == "Mul" => {
            match (contains_noise(hir, *left), contains_noise(hir, *right)) {
                (true, true) => Err(unsupported("product of noise terms")),
                (true, false) => {
                    let scaled = multiply(hir, amplitude.id, *right, expression.span);
                    extract_expression(
                        hir,
                        *left,
                        scaled,
                        equation,
                        is_current,
                        branch_ordinal,
                        pos,
                        neg,
                        sources,
                    )
                }
                (false, true) => {
                    let scaled = multiply(hir, amplitude.id, *left, expression.span);
                    extract_expression(
                        hir,
                        *right,
                        scaled,
                        equation,
                        is_current,
                        branch_ordinal,
                        pos,
                        neg,
                        sources,
                    )
                }
                (false, false) => Ok(()),
            }
        }
        HirExprKind::Binary { op, left, right } if op == "Div" => {
            if contains_noise(hir, *right) {
                return Err(unsupported("divisor"));
            }
            let scaled = binary(hir, "Div", amplitude.id, *right, expression.span);
            extract_expression(
                hir,
                *left,
                scaled,
                equation,
                is_current,
                branch_ordinal,
                pos,
                neg,
                sources,
            )
        }
        HirExprKind::Unary { op, operand } if matches!(op.as_str(), "Neg" | "Pos") => {
            extract_expression(
                hir,
                *operand,
                amplitude,
                equation,
                is_current,
                branch_ordinal,
                pos,
                neg,
                sources,
            )
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            if contains_noise(hir, *condition) {
                return Err(unsupported("condition"));
            }
            if contains_noise(hir, *then_expr) {
                let one = append_number(hir, 1.0, expression.span);
                let zero = append_number(hir, 0.0, expression.span);
                let gate = conditional(hir, *condition, one.id, zero.id, expression.span);
                let scaled = multiply(hir, amplitude.id, gate.id, expression.span);
                extract_expression(
                    hir,
                    *then_expr,
                    scaled,
                    equation,
                    is_current,
                    branch_ordinal,
                    pos,
                    neg,
                    sources,
                )?;
            }
            if contains_noise(hir, *else_expr) {
                let zero = append_number(hir, 0.0, expression.span);
                let one = append_number(hir, 1.0, expression.span);
                let gate = conditional(hir, *condition, zero.id, one.id, expression.span);
                let scaled = multiply(hir, amplitude.id, gate.id, expression.span);
                extract_expression(
                    hir,
                    *else_expr,
                    scaled,
                    equation,
                    is_current,
                    branch_ordinal,
                    pos,
                    neg,
                    sources,
                )?;
            }
            Ok(())
        }
        _ => Err(unsupported("nonlinear or dynamic position")),
    }
}

fn multiply(hir: &mut HirModel, left: ExprId, right: ExprId, span: SourceSpanRef) -> HirExprRef {
    binary(hir, "Mul", left, right, span)
}

fn binary(
    hir: &mut HirModel,
    op: &str,
    left: ExprId,
    right: ExprId,
    span: SourceSpanRef,
) -> HirExprRef {
    append_expression(
        hir,
        HirExprKind::Binary {
            op: op.into(),
            left,
            right,
        },
        "binary",
        span,
    )
}

fn conditional(
    hir: &mut HirModel,
    condition: ExprId,
    then_expr: ExprId,
    else_expr: ExprId,
    span: SourceSpanRef,
) -> HirExprRef {
    append_expression(
        hir,
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        },
        "conditional",
        span,
    )
}

fn append_number(hir: &mut HirModel, value: f64, span: SourceSpanRef) -> HirExprRef {
    append_expression(
        hir,
        HirExprKind::Number {
            value,
            raw: value.to_string().into(),
        },
        "number",
        span,
    )
}

fn append_expression(
    hir: &mut HirModel,
    kind: HirExprKind,
    label: &str,
    span: SourceSpanRef,
) -> HirExprRef {
    let id = ExprId::from(hir.expressions.len());
    hir.expressions.push(HirExpression { id, kind, span });
    HirExprRef {
        id,
        kind: label.into(),
        span,
    }
}

fn expr_ref(hir: &HirModel, id: ExprId) -> HirExprRef {
    let expression = &hir.expressions[usize::from(id)];
    HirExprRef {
        id,
        kind: expression_kind_label(&expression.kind).into(),
        span: expression.span,
    }
}

fn expression_kind_label(kind: &HirExprKind) -> &'static str {
    match kind {
        HirExprKind::Number { .. } => "number",
        HirExprKind::StringLiteral { .. } => "string",
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

fn contains_noise(hir: &HirModel, root: ExprId) -> bool {
    let mut stack = vec![root];
    let mut visited = std::collections::HashSet::new();
    while let Some(expr) = stack.pop() {
        if !visited.insert(expr) {
            continue;
        }
        match &hir.expressions[usize::from(expr)].kind {
            HirExprKind::NoiseSource { .. } => return true,
            HirExprKind::Call { name, .. } | HirExprKind::SystemFunction { name, .. }
                if is_noise_call(name) =>
            {
                return true;
            }
            HirExprKind::Binary { left, right, .. } => stack.extend([*left, *right]),
            HirExprKind::Unary { operand, .. } => stack.push(*operand),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => stack.extend([*condition, *then_expr, *else_expr]),
            HirExprKind::Call { args, .. }
            | HirExprKind::SystemFunction { args, .. }
            | HirExprKind::ArrayLiteral { elements: args } => stack.extend(args.iter().copied()),
            HirExprKind::ArrayAccess { index, .. } => stack.push(*index),
            HirExprKind::AnalogOperator { op } => push_analog_children(op, &mut stack),
            HirExprKind::Laplace { expr, .. } | HirExprKind::Zi { expr, .. } => stack.push(*expr),
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::Identifier { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
        }
    }
    false
}

fn is_noise_call(name: &str) -> bool {
    matches!(
        name.trim_start_matches('$'),
        "white_noise" | "flicker_noise" | "noise_table" | "noise_table_log"
    )
}

fn canonical_mechanism(
    kind: CanonicalNoiseSourceKind,
    pos: &CanonicalNoiseEndpoint,
    neg: &CanonicalNoiseEndpoint,
    label: Option<&str>,
) -> SmolStr {
    let kind = match kind {
        CanonicalNoiseSourceKind::White => "WHITE",
        CanonicalNoiseSourceKind::Flicker => "FLICKER",
        CanonicalNoiseSourceKind::Table => "TABLE",
    };
    let mut mechanism = format!(
        "{kind}_{}_{}",
        canonical_identifier(&pos.name, pos.node.is_none()),
        canonical_identifier(&neg.name, neg.node.is_none())
    );
    if let Some(label) = label {
        let label = canonical_identifier(label, false);
        if !label.is_empty() {
            mechanism.push('_');
            mechanism.push_str(&label);
        }
    }
    mechanism.into()
}

fn canonical_identifier(value: &str, ground: bool) -> String {
    if ground {
        return "GND".to_string();
    }
    let mut out = String::with_capacity(value.len());
    let mut separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() || matches!(character, '_' | '#') {
            if separator && !out.is_empty() && !out.ends_with('_') {
                out.push('_');
            }
            separator = false;
            out.push(character.to_ascii_uppercase());
        } else {
            separator = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}

fn string_literal(hir: &HirModel, id: ExprId) -> Option<SmolStr> {
    match &hir.expressions[usize::from(id)].kind {
        HirExprKind::StringLiteral { value } => Some(value.clone()),
        _ => None,
    }
}

fn push_analog_children(op: &HirAnalogOperator, stack: &mut Vec<ExprId>) {
    match op {
        HirAnalogOperator::Ddt { expr, abstol } => {
            stack.push(*expr);
            stack.extend(abstol.iter().copied());
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => {
            stack.push(*expr);
            stack.extend(ic.iter().chain(assert).chain(abstol).copied());
        }
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => {
            stack.push(*expr);
            stack.extend(
                ic.iter()
                    .chain(modulus)
                    .chain(offset)
                    .chain(abstol)
                    .copied(),
            );
        }
        HirAnalogOperator::Ddx { expr, probe } => stack.extend([*expr, *probe]),
        HirAnalogOperator::Limexp { expr } | HirAnalogOperator::LastCrossing { expr, .. } => {
            stack.push(*expr);
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => {
            stack.extend([*expr, *delay]);
            stack.extend(max_delay.iter().copied());
        }
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => {
            stack.push(*expr);
            stack.extend(
                delay
                    .iter()
                    .chain(rise)
                    .chain(fall)
                    .chain(tolerance)
                    .copied(),
            );
        }
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => {
            stack.push(*expr);
            stack.extend(max_rise.iter().chain(max_fall).copied());
        }
    }
}
