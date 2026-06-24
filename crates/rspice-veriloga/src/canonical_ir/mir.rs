use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

use super::hir::{
    CanonicalValueType, HirContributionKind, HirExprKind, HirExprRef, HirExpression, HirModel,
    HirParamRange,
};
use super::{
    CompilerPhase, ContributionId, EquationId, IrDiagnostic, IrValidationResult, NodeId, ParamId,
    SourceSpanRef, StateId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MirAnalysisDomain {
    Dc,
    Ac,
    Transient,
    Noise,
    OperatingPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MirEquationKind {
    Current,
    Potential,
    Indirect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirNode {
    pub id: NodeId,
    pub name: SmolStr,
    pub is_external: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirParameterSlot {
    pub id: ParamId,
    pub name: SmolStr,
    pub value_type: CanonicalValueType,
    pub default: Option<f64>,
    pub default_expr: Option<HirExprRef>,
    pub range: Option<HirParamRange>,
    pub aliases: Vec<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirStateSlot {
    pub id: StateId,
    pub name: SmolStr,
    pub owner: EquationId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirBranchRef {
    pub label: SmolStr,
    pub pos_node: NodeId,
    pub neg_node: Option<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirEquation {
    pub id: EquationId,
    pub contribution: ContributionId,
    pub branch: MirBranchRef,
    pub kind: MirEquationKind,
    pub expression: HirExprRef,
    pub active_domains: Vec<MirAnalysisDomain>,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirModel {
    pub module_name: SmolStr,
    pub nodes: Vec<MirNode>,
    pub parameters: Vec<MirParameterSlot>,
    pub state_slots: Vec<MirStateSlot>,
    pub equations: Vec<MirEquation>,
    pub expressions: Vec<HirExpression>,
}

impl MirModel {
    pub fn from_hir(hir: &HirModel) -> Result<Self, Vec<IrDiagnostic>> {
        hir.validate()?;

        let mut nodes: Vec<_> = hir
            .ports
            .iter()
            .enumerate()
            .map(|(index, port)| MirNode {
                id: NodeId::from(index),
                name: port.name.clone(),
                is_external: true,
            })
            .collect();

        let external_node_count = nodes.len();
        nodes.extend(
            hir.internal_nodes
                .iter()
                .enumerate()
                .map(|(index, node)| MirNode {
                    id: NodeId::from(external_node_count + index),
                    name: node.name.clone(),
                    is_external: false,
                }),
        );

        let parameters = hir
            .parameters
            .iter()
            .map(|parameter| MirParameterSlot {
                id: parameter.id,
                name: parameter.name.clone(),
                value_type: parameter.value_type,
                default: parameter.default,
                default_expr: parameter.default_expr.clone(),
                range: parameter.range.clone(),
                aliases: parameter.aliases.clone(),
            })
            .collect();

        let node_ids_by_name = node_ids_by_name(&nodes);
        let equations = hir
            .contributions
            .iter()
            .enumerate()
            .map(|(index, contribution)| MirEquation {
                id: EquationId::from(index),
                contribution: contribution.id,
                branch: resolve_branch_ref(&contribution.branch, hir, &node_ids_by_name),
                kind: MirEquationKind::from(contribution.kind),
                expression: contribution.expression.clone(),
                active_domains: default_active_domains(),
                span: contribution.span,
            })
            .collect();

        let mir = Self {
            module_name: hir.module_name.clone(),
            nodes,
            parameters,
            state_slots: Vec::new(),
            equations,
            expressions: hir.expressions.clone(),
        };

        mir.validate().map(|()| mir)
    }

    pub fn validate(&self) -> IrValidationResult {
        let mut diagnostics = Vec::new();

        if self.module_name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                "MIR module name must not be empty",
            ));
        }

        if self.nodes.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                "MIR model must have at least one node",
            ));
        }

        validate_dense_node_ids(&mut diagnostics, &self.nodes);
        validate_dense_parameter_ids(&mut diagnostics, &self.parameters);
        validate_dense_state_slot_ids(&mut diagnostics, &self.state_slots);
        validate_dense_equation_ids(&mut diagnostics, &self.equations);
        validate_dense_expression_ids(&mut diagnostics, &self.expressions);
        validate_node_names(&mut diagnostics, &self.nodes);
        validate_parameter_names_and_aliases(&mut diagnostics, &self.parameters);
        validate_parameter_default_exprs(&mut diagnostics, &self.parameters, &self.expressions);
        validate_state_slot_owners(&mut diagnostics, &self.state_slots, self.equations.len());
        validate_equations(
            &mut diagnostics,
            &self.equations,
            &self.expressions,
            &self.nodes,
        );

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}

impl From<HirContributionKind> for MirEquationKind {
    fn from(value: HirContributionKind) -> Self {
        match value {
            HirContributionKind::Current => Self::Current,
            HirContributionKind::Potential => Self::Potential,
            HirContributionKind::Indirect => Self::Indirect,
        }
    }
}

fn default_active_domains() -> Vec<MirAnalysisDomain> {
    vec![
        MirAnalysisDomain::Dc,
        MirAnalysisDomain::Ac,
        MirAnalysisDomain::Transient,
        MirAnalysisDomain::OperatingPoint,
    ]
}

fn node_ids_by_name(nodes: &[MirNode]) -> HashMap<SmolStr, NodeId> {
    nodes
        .iter()
        .map(|node| (node.name.clone(), node.id))
        .collect()
}

fn resolve_branch_ref(
    branch_label: &SmolStr,
    hir: &HirModel,
    node_ids_by_name: &HashMap<SmolStr, NodeId>,
) -> MirBranchRef {
    let (pos_name, neg_name) = hir
        .branches
        .iter()
        .find(|branch| branch.name == *branch_label)
        .map(|branch| {
            (
                branch.pos_node.clone(),
                if branch.neg_node.is_empty() {
                    None
                } else {
                    Some(branch.neg_node.clone())
                },
            )
        })
        .unwrap_or_else(|| {
            if let Some((pos, neg)) = branch_label.split_once(',') {
                (pos.into(), Some(neg.into()))
            } else {
                (branch_label.clone(), None)
            }
        });

    let pos_node = *node_ids_by_name
        .get(&pos_name)
        .expect("validated HIR branch pos node must resolve to MIR node");
    let neg_node = neg_name
        .as_ref()
        .filter(|name| !is_ground_name(name, hir))
        .map(|name| {
            *node_ids_by_name
                .get(name)
                .expect("validated HIR branch neg node must resolve to MIR node")
        });
    let label = if hir
        .branches
        .iter()
        .any(|branch| branch.name == *branch_label)
    {
        match neg_name {
            Some(neg_name) => format!("{pos_name},{neg_name}").into(),
            None => pos_name.clone(),
        }
    } else {
        branch_label.clone()
    };

    MirBranchRef {
        label,
        pos_node,
        neg_node,
    }
}

fn is_ground_name(name: &str, hir: &HirModel) -> bool {
    name == "0"
        || hir
            .ground_nodes
            .iter()
            .any(|ground| ground.as_str() == name)
}

fn validate_dense_node_ids(diagnostics: &mut Vec<IrDiagnostic>, nodes: &[MirNode]) {
    for (expected, node) in nodes.iter().enumerate() {
        let expected = u32::try_from(expected).expect("MIR node count exceeds u32::MAX");
        if node.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR node IDs must be dense: expected NodeId({}) at index {}, found {}",
                    expected, expected, node.id
                ),
            ));
        }
    }
}

fn validate_dense_parameter_ids(
    diagnostics: &mut Vec<IrDiagnostic>,
    parameters: &[MirParameterSlot],
) {
    for (expected, parameter) in parameters.iter().enumerate() {
        let expected = u32::try_from(expected).expect("MIR parameter count exceeds u32::MAX");
        if parameter.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR parameter IDs must be dense: expected ParamId({}) at index {}, found {}",
                    expected, expected, parameter.id
                ),
            ));
        }
    }
}

fn validate_dense_state_slot_ids(
    diagnostics: &mut Vec<IrDiagnostic>,
    state_slots: &[MirStateSlot],
) {
    for (expected, state_slot) in state_slots.iter().enumerate() {
        let expected = u32::try_from(expected).expect("MIR state slot count exceeds u32::MAX");
        if state_slot.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR state slot IDs must be dense: expected StateId({}) at index {}, found {}",
                    expected, expected, state_slot.id
                ),
            ));
        }
    }
}

fn validate_dense_equation_ids(diagnostics: &mut Vec<IrDiagnostic>, equations: &[MirEquation]) {
    for (expected, equation) in equations.iter().enumerate() {
        let expected = u32::try_from(expected).expect("MIR equation count exceeds u32::MAX");
        if equation.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR equation IDs must be dense: expected EquationId({}) at index {}, found {}",
                    expected, expected, equation.id
                ),
            ));
        }
    }
}

fn validate_dense_expression_ids(
    diagnostics: &mut Vec<IrDiagnostic>,
    expressions: &[HirExpression],
) {
    for (expected, expression) in expressions.iter().enumerate() {
        let expected = u32::try_from(expected).expect("MIR expression count exceeds u32::MAX");
        if expression.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR expression IDs must be dense: expected ExprId({}) at index {}, found {}",
                    expected, expected, expression.id
                ),
            ));
        }
    }
}

fn validate_node_names(diagnostics: &mut Vec<IrDiagnostic>, nodes: &[MirNode]) {
    let mut names = HashSet::new();
    let mut saw_internal = false;

    for node in nodes {
        if node.is_external {
            if saw_internal {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    "MIR external nodes must precede internal nodes",
                ));
            }
        } else {
            saw_internal = true;
        }

        if node.name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!("MIR node {} name must not be empty", node.id),
            ));
        } else if !names.insert(node.name.clone()) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!("MIR duplicate node name '{}'", node.name),
            ));
        }
    }
}

fn validate_parameter_names_and_aliases(
    diagnostics: &mut Vec<IrDiagnostic>,
    parameters: &[MirParameterSlot],
) {
    let mut names = HashSet::new();
    for parameter in parameters {
        if parameter.name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!("MIR parameter {} name must not be empty", parameter.id),
            ));
        } else if !names.insert(parameter.name.clone()) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!("MIR duplicate parameter name '{}'", parameter.name),
            ));
        }
    }

    let mut identifiers = HashSet::new();
    for parameter in parameters {
        if !parameter.name.is_empty() {
            identifiers.insert(parameter.name.clone());
        }

        let mut local_aliases = HashSet::new();
        for alias in &parameter.aliases {
            if alias.is_empty() {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR parameter alias for '{}' must not be empty",
                        parameter.name
                    ),
                ));
                continue;
            }

            if names.contains(alias) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR parameter alias '{}' collides with parameter name",
                        alias
                    ),
                ));
            }

            if !local_aliases.insert(alias.clone()) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR duplicate parameter alias '{}' on parameter '{}'",
                        alias, parameter.name
                    ),
                ));
            }

            if !identifiers.insert(alias.clone()) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    format!("MIR duplicate parameter alias '{}'", alias),
                ));
            }
        }
    }
}

fn validate_parameter_default_exprs(
    diagnostics: &mut Vec<IrDiagnostic>,
    parameters: &[MirParameterSlot],
    expressions: &[HirExpression],
) {
    for parameter in parameters {
        if let Some(default_expr) = &parameter.default_expr {
            validate_expr_ref(
                diagnostics,
                &format!("parameter '{}' default", parameter.name),
                default_expr,
                expressions,
            );
        }
    }
}

fn validate_state_slot_owners(
    diagnostics: &mut Vec<IrDiagnostic>,
    state_slots: &[MirStateSlot],
    equation_count: usize,
) {
    for state_slot in state_slots {
        if usize::from(state_slot.owner) >= equation_count {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR state slot {} owner {} is out of range for {} equations",
                    state_slot.id, state_slot.owner, equation_count
                ),
            ));
        }
    }
}

fn validate_equations(
    diagnostics: &mut Vec<IrDiagnostic>,
    equations: &[MirEquation],
    expressions: &[HirExpression],
    nodes: &[MirNode],
) {
    for equation in equations {
        if equation.contribution.index() != equation.id.index() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR equation {} contribution {} must match equation id {}",
                    equation.id, equation.contribution, equation.id
                ),
                equation.span,
            ));
        }

        validate_expr_ref(
            diagnostics,
            &format!("equation {} expression", equation.id.index()),
            &equation.expression,
            expressions,
        );
        validate_branch_ref(diagnostics, equation, nodes);

        if equation.active_domains.is_empty() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR equation {} must have at least one active domain",
                    equation.id
                ),
                equation.span,
            ));
        }

        let mut domains = HashSet::new();
        for domain in &equation.active_domains {
            if !domains.insert(*domain) {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR equation {} has duplicate active domain {:?}",
                        equation.id, domain
                    ),
                    equation.span,
                ));
            }
        }
    }
}

fn validate_expr_ref(
    diagnostics: &mut Vec<IrDiagnostic>,
    label: &str,
    expr_ref: &HirExprRef,
    expressions: &[HirExpression],
) {
    let index = usize::from(expr_ref.id);
    let Some(expression) = expressions.get(index) else {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            format!(
                "MIR expression ref {} id {} is outside expression arena length {}",
                label,
                expr_ref.id,
                expressions.len()
            ),
            expr_ref.span,
        ));
        return;
    };

    let actual_kind = hir_expr_kind_label(&expression.kind);
    if expr_ref.kind.as_str() != actual_kind {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            format!(
                "MIR expression ref {} kind '{}' does not match '{}'",
                label, expr_ref.kind, actual_kind
            ),
            expr_ref.span,
        ));
    }
}

fn validate_branch_ref(
    diagnostics: &mut Vec<IrDiagnostic>,
    equation: &MirEquation,
    nodes: &[MirNode],
) {
    let Some(pos_name) = node_name(nodes, equation.branch.pos_node) else {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            format!(
                "MIR equation {} branch pos_node {} is out of range for {} nodes",
                equation.id,
                equation.branch.pos_node,
                nodes.len()
            ),
            equation.span,
        ));
        return;
    };

    let neg_name = match equation.branch.neg_node {
        Some(neg_node) => {
            let Some(name) = node_name(nodes, neg_node) else {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR equation {} branch neg_node {} is out of range for {} nodes",
                        equation.id,
                        neg_node,
                        nodes.len()
                    ),
                    equation.span,
                ));
                return;
            };
            Some(name)
        }
        None => None,
    };

    let canonical_label = match neg_name {
        Some(neg_name) => format!("{pos_name},{neg_name}"),
        None => pos_name.to_string(),
    };
    let label_matches = if equation.branch.neg_node.is_none() {
        let zero_label = format!("{pos_name},0");
        let gnd_label = format!("{pos_name},gnd");
        equation.branch.label.as_str() == canonical_label
            || equation.branch.label.as_str() == zero_label
            || equation.branch.label.as_str() == gnd_label
    } else {
        equation.branch.label.as_str() == canonical_label
    };

    if !label_matches {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            format!(
                "MIR equation {} branch label '{}' does not match endpoints {}",
                equation.id, equation.branch.label, canonical_label
            ),
            equation.span,
        ));
    }
}

fn node_name(nodes: &[MirNode], id: NodeId) -> Option<&str> {
    nodes.get(usize::from(id)).map(|node| node.name.as_str())
}

fn hir_expr_kind_label(kind: &HirExprKind) -> &'static str {
    match kind {
        HirExprKind::Number { .. } => "number",
        HirExprKind::StringLiteral { .. } => "string",
        HirExprKind::Identifier { .. } => "identifier",
        HirExprKind::SystemFunction { .. } => "system_function",
        HirExprKind::Binary { .. } => "binary",
        HirExprKind::Unary { .. } => "unary",
        HirExprKind::Conditional { .. } => "conditional",
        HirExprKind::Call { .. } => "call",
        HirExprKind::BranchAccess { .. } | HirExprKind::NamedBranchAccess { .. } => "branch_access",
        HirExprKind::ArrayAccess { .. } => "array_access",
        HirExprKind::ArrayLiteral { .. } => "array_literal",
        HirExprKind::AnalogOperator { .. }
        | HirExprKind::Laplace { .. }
        | HirExprKind::Zi { .. } => "analog_operator",
        HirExprKind::NoiseSource { .. } => "noise_source",
    }
}
