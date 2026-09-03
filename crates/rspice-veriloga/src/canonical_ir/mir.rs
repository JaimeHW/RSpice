//! MIR: the second canonical level, flattened into solver equations.
//!
//! [`MirModel::from_hir`] turns the source-shaped HIR into the rows the MNA
//! solver actually assembles. Each [`MirEquation`] is a current, potential, or
//! indirect contribution; branch unknowns are materialized for the potential
//! and indirect kinds that need an extra column; and parameters and operator
//! state get numbered slots. Equations also carry the [`MirAnalysisDomain`]s
//! they participate in, which is what lets a backend skip work an analysis
//! cannot observe.

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

use crate::semantic::ParameterScope;

use super::hir::{
    CanonicalValueType, HirAnalogOperator, HirContributionKind, HirExprKind, HirExprRef,
    HirExpression, HirModel, HirParamRange, HirParameterDimension,
};
use super::{
    BranchId, BranchUnknownId, CompilerPhase, ContributionId, EquationId, ExprId, IrDiagnostic,
    IrValidationResult, NodeId, ParamId, SourceSpanRef, StateId,
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
    #[serde(default = "default_true")]
    pub is_public: bool,
    pub scope: ParameterScope,
    #[serde(default)]
    pub also_model: bool,
    pub value_type: CanonicalValueType,
    #[serde(default)]
    pub dimensions: Vec<HirParameterDimension>,
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
    pub declared_name: Option<SmolStr>,
    pub pos_node: Option<NodeId>,
    pub neg_node: Option<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirBranch {
    pub id: BranchId,
    pub name: SmolStr,
    pub pos_node: Option<NodeId>,
    pub neg_node: Option<NodeId>,
    pub discipline: SmolStr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirBranchUnknown {
    pub id: BranchUnknownId,
    pub equation: EquationId,
    pub declared_name: Option<SmolStr>,
    pub pos_node: Option<NodeId>,
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
    pub branches: Vec<MirBranch>,
    pub branch_unknowns: Vec<MirBranchUnknown>,
    pub state_slots: Vec<MirStateSlot>,
    pub equations: Vec<MirEquation>,
    pub expressions: Vec<HirExpression>,
    #[serde(default = "canonical_default_transition")]
    pub default_transition: f64,
    pub value_symbols: Vec<SmolStr>,
    pub ground_nodes: Vec<SmolStr>,
}

impl MirModel {
    pub fn from_hir(hir: &HirModel) -> Result<Self, Vec<IrDiagnostic>> {
        let span = crate::metrics::FineSpan::new("mir.hir_validate");
        hir.validate()?;
        span.finish(&format!("expressions={}", hir.expressions.len()));

        let span = crate::metrics::FineSpan::new("mir.build");
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
                is_public: parameter.is_public,
                scope: parameter.scope,
                also_model: parameter.also_model,
                value_type: parameter.value_type,
                dimensions: parameter.dimensions.clone(),
                default: parameter.default,
                default_expr: parameter.default_expr.clone(),
                range: parameter.range.clone(),
                aliases: parameter.aliases.clone(),
            })
            .collect();

        let node_ids_by_name = node_ids_by_name(&nodes);
        let branches = hir
            .branches
            .iter()
            .map(|branch| {
                let pos_node = resolve_optional_endpoint(&branch.pos_node, hir, &node_ids_by_name);
                let neg_node = if branch.neg_node.is_empty() {
                    None
                } else {
                    resolve_optional_endpoint(&branch.neg_node, hir, &node_ids_by_name)
                };

                MirBranch {
                    id: branch.id,
                    name: branch.name.clone(),
                    pos_node,
                    neg_node,
                    discipline: branch.discipline.clone(),
                }
            })
            .collect();
        let equations: Vec<MirEquation> = hir
            .contributions
            .iter()
            .enumerate()
            .map(|(index, contribution)| MirEquation {
                id: EquationId::from(index),
                contribution: contribution.id,
                branch: resolve_branch_ref(
                    &contribution.branch,
                    contribution.declared_branch.as_ref(),
                    hir,
                    &node_ids_by_name,
                ),
                kind: MirEquationKind::from(contribution.kind),
                expression: contribution.expression.clone(),
                active_domains: default_active_domains(),
                span: contribution.span,
            })
            .collect();
        let branch_unknowns = collect_branch_unknowns(&equations);
        span.finish(&format!("equations={}", equations.len()));

        let span = crate::metrics::FineSpan::new("mir.arena_clone");
        let expressions = hir.expressions.clone();
        span.finish(&format!("expressions={}", expressions.len()));

        let mir = Self {
            module_name: hir.module_name.clone(),
            nodes,
            parameters,
            branches,
            branch_unknowns,
            state_slots: Vec::new(),
            equations,
            expressions,
            default_transition: hir.default_transition,
            value_symbols: sorted_value_symbols(hir),
            ground_nodes: hir.ground_nodes.clone(),
        };

        let span = crate::metrics::FineSpan::new("mir.validate");
        let validated = mir.validate();
        span.finish(&format!("expressions={}", mir.expressions.len()));
        validated.map(|()| mir)
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
        if !self.default_transition.is_finite() || self.default_transition < 0.0 {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR default_transition must be finite and non-negative, got {}",
                    self.default_transition
                ),
            ));
        }

        validate_dense_node_ids(&mut diagnostics, &self.nodes);
        validate_dense_parameter_ids(&mut diagnostics, &self.parameters);
        validate_dense_branch_ids(&mut diagnostics, &self.branches);
        validate_dense_branch_unknown_ids(&mut diagnostics, &self.branch_unknowns);
        validate_dense_state_slot_ids(&mut diagnostics, &self.state_slots);
        validate_dense_equation_ids(&mut diagnostics, &self.equations);
        validate_dense_expression_ids(&mut diagnostics, &self.expressions);
        validate_node_names(&mut diagnostics, &self.nodes);
        validate_branches(&mut diagnostics, &self.branches, &self.nodes);
        validate_branch_unknowns(
            &mut diagnostics,
            &self.branch_unknowns,
            &self.equations,
            &self.nodes,
        );
        validate_expressions(
            &mut diagnostics,
            &self.expressions,
            &self.nodes,
            &self.branches,
            &self.value_symbols,
            &self.ground_nodes,
        );
        validate_value_symbols(&mut diagnostics, &self.value_symbols);
        validate_parameter_names_and_aliases(&mut diagnostics, &self.parameters);
        validate_parameter_default_exprs(&mut diagnostics, &self.parameters, &self.expressions);
        diagnostics.extend(super::parameter_array::validate_parameter_array_contract(
            CompilerPhase::MirValidation,
            &self.parameters,
            &self.expressions,
        ));
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

const fn canonical_default_transition() -> f64 {
    1.0e-9
}

const fn default_true() -> bool {
    true
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

fn collect_branch_unknowns(equations: &[MirEquation]) -> Vec<MirBranchUnknown> {
    equations
        .iter()
        .filter(|equation| {
            matches!(
                equation.kind,
                MirEquationKind::Potential | MirEquationKind::Indirect
            )
        })
        .enumerate()
        .map(|(index, equation)| MirBranchUnknown {
            id: BranchUnknownId::from(index),
            equation: equation.id,
            declared_name: equation.branch.declared_name.clone(),
            pos_node: equation.branch.pos_node,
            neg_node: equation.branch.neg_node,
        })
        .collect()
}

fn node_ids_by_name(nodes: &[MirNode]) -> HashMap<SmolStr, NodeId> {
    nodes
        .iter()
        .map(|node| (node.name.clone(), node.id))
        .collect()
}

fn sorted_value_symbols(hir: &HirModel) -> Vec<SmolStr> {
    let mut symbols: Vec<_> = hir.known_value_symbol_names().into_iter().collect();
    symbols.sort();
    symbols
}

fn resolve_node_id(name: &SmolStr, node_ids_by_name: &HashMap<SmolStr, NodeId>) -> Option<NodeId> {
    node_ids_by_name.get(name).copied()
}

fn resolve_branch_ref(
    branch_label: &SmolStr,
    declared_branch: Option<&SmolStr>,
    hir: &HirModel,
    node_ids_by_name: &HashMap<SmolStr, NodeId>,
) -> MirBranchRef {
    let mut declared_name = declared_branch.cloned();
    let (pos_name, neg_name) = hir
        .branches
        .iter()
        .find(|branch| {
            declared_branch.is_some_and(|declared| branch.name.as_str() == declared.as_str())
        })
        .or_else(|| {
            hir.branches
                .iter()
                .find(|branch| branch.name == *branch_label)
        })
        .map(|branch| {
            declared_name = Some(branch.name.clone());
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

    let pos_node = resolve_optional_endpoint(&pos_name, hir, node_ids_by_name);
    let neg_node = neg_name
        .as_ref()
        .and_then(|name| resolve_optional_endpoint(name, hir, node_ids_by_name));
    let label = canonical_branch_label(pos_node, neg_node, node_ids_by_name);

    MirBranchRef {
        label,
        declared_name,
        pos_node,
        neg_node,
    }
}

fn resolve_optional_endpoint(
    name: &SmolStr,
    hir: &HirModel,
    node_ids_by_name: &HashMap<SmolStr, NodeId>,
) -> Option<NodeId> {
    if is_ground_name(name, hir) {
        None
    } else {
        Some(
            resolve_node_id(name, node_ids_by_name)
                .expect("validated HIR branch endpoint must resolve to MIR node or ground"),
        )
    }
}

fn canonical_branch_label(
    pos_node: Option<NodeId>,
    neg_node: Option<NodeId>,
    node_ids_by_name: &HashMap<SmolStr, NodeId>,
) -> SmolStr {
    let name_by_id: HashMap<_, _> = node_ids_by_name
        .iter()
        .map(|(name, id)| (*id, name.as_str()))
        .collect();
    let endpoint_name = |node: Option<NodeId>| match node {
        Some(node) => *name_by_id
            .get(&node)
            .expect("canonical branch endpoint must resolve to MIR node name"),
        None => "0",
    };

    format!("{},{}", endpoint_name(pos_node), endpoint_name(neg_node)).into()
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

fn validate_dense_branch_ids(diagnostics: &mut Vec<IrDiagnostic>, branches: &[MirBranch]) {
    for (expected, branch) in branches.iter().enumerate() {
        let expected = u32::try_from(expected).expect("MIR branch count exceeds u32::MAX");
        if branch.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch IDs must be dense: expected BranchId({}) at index {}, found {}",
                    expected, expected, branch.id
                ),
            ));
        }
    }
}

fn validate_dense_branch_unknown_ids(
    diagnostics: &mut Vec<IrDiagnostic>,
    branch_unknowns: &[MirBranchUnknown],
) {
    for (expected, unknown) in branch_unknowns.iter().enumerate() {
        let expected = u32::try_from(expected).expect("MIR branch unknown count exceeds u32::MAX");
        if unknown.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch unknown IDs must be dense: expected BranchUnknownId({}) at index {}, found {}",
                    expected, expected, unknown.id
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

fn validate_branches(
    diagnostics: &mut Vec<IrDiagnostic>,
    branches: &[MirBranch],
    nodes: &[MirNode],
) {
    let mut names = HashSet::new();

    for branch in branches {
        if branch.name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                "MIR branch name must not be empty",
            ));
        } else if !names.insert(branch.name.clone()) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!("MIR duplicate branch name '{}'", branch.name),
            ));
        }

        if branch.pos_node.is_none() && branch.neg_node.is_none() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch '{}' must have at least one concrete endpoint",
                    branch.name
                ),
            ));
        }

        if let Some(pos_node) = branch.pos_node {
            if usize::from(pos_node) >= nodes.len() {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR branch '{}' pos_node {} is out of range for {} nodes",
                        branch.name,
                        pos_node,
                        nodes.len()
                    ),
                ));
            }
        }

        if let Some(neg_node) = branch.neg_node {
            if usize::from(neg_node) >= nodes.len() {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR branch '{}' neg_node {} is out of range for {} nodes",
                        branch.name,
                        neg_node,
                        nodes.len()
                    ),
                ));
            }
        }
    }
}

fn validate_branch_unknowns(
    diagnostics: &mut Vec<IrDiagnostic>,
    branch_unknowns: &[MirBranchUnknown],
    equations: &[MirEquation],
    nodes: &[MirNode],
) {
    let mut equations_seen = HashSet::new();
    for unknown in branch_unknowns {
        if !equations_seen.insert(unknown.equation) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR duplicate branch unknown for equation {}",
                    unknown.equation
                ),
            ));
        }

        let Some(equation) = equations.get(usize::from(unknown.equation)) else {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch unknown {} equation {} is out of range for {} equations",
                    unknown.id,
                    unknown.equation,
                    equations.len()
                ),
            ));
            continue;
        };
        if equation.id != unknown.equation {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch unknown {} equation {} does not match equation table entry {}",
                    unknown.id, unknown.equation, equation.id
                ),
            ));
        }
        if !matches!(
            equation.kind,
            MirEquationKind::Potential | MirEquationKind::Indirect
        ) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch unknown {} must reference a potential or indirect equation, found {:?}",
                    unknown.id, equation.kind
                ),
            ));
        }
        if unknown.pos_node != equation.branch.pos_node
            || unknown.neg_node != equation.branch.neg_node
        {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch unknown {} endpoints do not match equation {} branch endpoints",
                    unknown.id, unknown.equation
                ),
            ));
        }
        if unknown.declared_name != equation.branch.declared_name {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch unknown {} declared name does not match equation {} branch",
                    unknown.id, unknown.equation
                ),
            ));
        }
        if unknown.pos_node.is_none() && unknown.neg_node.is_none() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR branch unknown {} must have a concrete endpoint",
                    unknown.id
                ),
            ));
        }
        if let Some(pos_node) = unknown.pos_node {
            if usize::from(pos_node) >= nodes.len() {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR branch unknown {} pos_node {} is out of range for {} nodes",
                        unknown.id,
                        pos_node,
                        nodes.len()
                    ),
                ));
            }
        }
        if let Some(neg_node) = unknown.neg_node {
            if usize::from(neg_node) >= nodes.len() {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR branch unknown {} neg_node {} is out of range for {} nodes",
                        unknown.id,
                        neg_node,
                        nodes.len()
                    ),
                ));
            }
        }
    }
}

fn validate_endpoint_node<'a>(
    diagnostics: &mut Vec<IrDiagnostic>,
    label: &str,
    endpoint: Option<NodeId>,
    nodes: &'a [MirNode],
    span: SourceSpanRef,
) -> Option<Option<&'a str>> {
    match endpoint {
        Some(node) => {
            let Some(name) = node_name(nodes, node) else {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::MirValidation,
                    format!(
                        "MIR {} {} is out of range for {} nodes",
                        label,
                        node,
                        nodes.len()
                    ),
                    span,
                ));
                return None;
            };
            Some(Some(name))
        }
        None => Some(None),
    }
}

fn canonical_label_from_endpoint_names(pos_name: Option<&str>, neg_name: Option<&str>) -> String {
    format!("{},{}", pos_name.unwrap_or("0"), neg_name.unwrap_or("0"))
}

fn validate_branch_ref(
    diagnostics: &mut Vec<IrDiagnostic>,
    equation: &MirEquation,
    nodes: &[MirNode],
) {
    if equation.branch.pos_node.is_none() && equation.branch.neg_node.is_none() {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            format!(
                "MIR equation {} branch must have at least one concrete endpoint",
                equation.id
            ),
            equation.span,
        ));
        return;
    }

    let Some(pos_name) = validate_endpoint_node(
        diagnostics,
        &format!("equation {} branch pos_node", equation.id),
        equation.branch.pos_node,
        nodes,
        equation.span,
    ) else {
        return;
    };
    let Some(neg_name) = validate_endpoint_node(
        diagnostics,
        &format!("equation {} branch neg_node", equation.id),
        equation.branch.neg_node,
        nodes,
        equation.span,
    ) else {
        return;
    };

    let canonical_label = canonical_label_from_endpoint_names(pos_name, neg_name);

    if equation.branch.label.as_str() != canonical_label {
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
        HirExprKind::NullArgument => "null_argument",
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
        HirExprKind::AnalogOperator { .. } => "analog_operator",
        HirExprKind::NoiseSource { .. } => "noise_source",
    }
}

fn validate_expressions(
    diagnostics: &mut Vec<IrDiagnostic>,
    expressions: &[HirExpression],
    nodes: &[MirNode],
    branches: &[MirBranch],
    value_symbols: &[SmolStr],
    ground_nodes: &[SmolStr],
) {
    let node_names: HashSet<_> = nodes.iter().map(|node| node.name.clone()).collect();
    let branch_names: HashSet<_> = branches.iter().map(|branch| branch.name.clone()).collect();
    let value_symbols: HashSet<_> = value_symbols.iter().cloned().collect();

    for expression in expressions {
        match &expression.kind {
            HirExprKind::NullArgument
            | HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. } => {}
            HirExprKind::Identifier { name } => {
                validate_identifier(diagnostics, expression, name, &value_symbols);
            }
            HirExprKind::BranchAccess { pos, neg, .. } => {
                validate_branch_access_node(
                    diagnostics,
                    expression,
                    pos,
                    &node_names,
                    ground_nodes,
                );
                if let Some(neg) = neg {
                    validate_branch_access_node(
                        diagnostics,
                        expression,
                        neg,
                        &node_names,
                        ground_nodes,
                    );
                }
            }
            HirExprKind::NamedBranchAccess { name, .. } => {
                if !branch_names.contains(name) {
                    diagnostics.push(IrDiagnostic::error(
                        CompilerPhase::MirValidation,
                        format!("MIR unknown named branch access '{}'", name),
                        expression.span,
                    ));
                }
            }
            HirExprKind::SystemFunction { args, .. } => {
                validate_expression_child_list(diagnostics, expressions, expression, "arg", args);
            }
            HirExprKind::Call { name, args } => {
                validate_expression_child_list(diagnostics, expressions, expression, "arg", args);
                validate_zi_call_budget(diagnostics, expressions, expression, name, args);
            }
            HirExprKind::Binary { left, right, .. } => {
                validate_expression_child(diagnostics, expressions, expression, "left", *left);
                validate_expression_child(diagnostics, expressions, expression, "right", *right);
            }
            HirExprKind::Unary { operand, .. } => {
                validate_expression_child(
                    diagnostics,
                    expressions,
                    expression,
                    "operand",
                    *operand,
                );
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                validate_expression_child(
                    diagnostics,
                    expressions,
                    expression,
                    "condition",
                    *condition,
                );
                validate_expression_child(
                    diagnostics,
                    expressions,
                    expression,
                    "then_expr",
                    *then_expr,
                );
                validate_expression_child(
                    diagnostics,
                    expressions,
                    expression,
                    "else_expr",
                    *else_expr,
                );
            }
            HirExprKind::ArrayAccess { index, .. } => {
                validate_expression_child(diagnostics, expressions, expression, "index", *index);
            }
            HirExprKind::ArrayLiteral { elements, .. } => {
                validate_expression_child_list(
                    diagnostics,
                    expressions,
                    expression,
                    "element",
                    elements,
                );
            }
            HirExprKind::AnalogOperator { op } => {
                validate_analog_operator_children(diagnostics, expressions, expression, op);
            }
            HirExprKind::NoiseSource { operands, .. } => {
                validate_expression_child_list(
                    diagnostics,
                    expressions,
                    expression,
                    "operand",
                    operands,
                );
            }
        }
    }
}

fn validate_zi_call_budget(
    diagnostics: &mut Vec<IrDiagnostic>,
    expressions: &[HirExpression],
    expression: &HirExpression,
    name: &SmolStr,
    args: &[ExprId],
) {
    if !matches!(
        name.to_ascii_lowercase().as_str(),
        "zi_zp" | "zi_zd" | "zi_np" | "zi_nd"
    ) || args.len() < 3
    {
        return;
    }
    let scalar_count = |id: ExprId| {
        expressions
            .get(usize::from(id))
            .map(|child| match &child.kind {
                HirExprKind::NullArgument => 0,
                HirExprKind::ArrayLiteral { elements, .. } => elements.len(),
                _ => 1,
            })
    };
    let (Some(numerator), Some(denominator)) = (scalar_count(args[1]), scalar_count(args[2]))
    else {
        return;
    };
    if let Err(error) =
        crate::zfilter::validate_zi_runtime_operand_budget(name, numerator, denominator)
    {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            error.to_string(),
            expression.span,
        ));
    }
}

fn validate_identifier(
    diagnostics: &mut Vec<IrDiagnostic>,
    expression: &HirExpression,
    name: &SmolStr,
    value_symbols: &HashSet<SmolStr>,
) {
    if !value_symbols.contains(name) {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            format!("MIR unknown identifier '{}'", name),
            expression.span,
        ));
    }
}

fn validate_analog_operator_children(
    diagnostics: &mut Vec<IrDiagnostic>,
    expressions: &[HirExpression],
    expression: &HirExpression,
    op: &HirAnalogOperator,
) {
    match op {
        HirAnalogOperator::Limit {
            proposed,
            candidate,
            type_metadata,
            ..
        } => {
            validate_expression_child(diagnostics, expressions, expression, "proposed", *proposed);
            validate_expression_child(
                diagnostics,
                expressions,
                expression,
                "candidate",
                *candidate,
            );
            validate_optional_expression_child(
                diagnostics,
                expressions,
                expression,
                "type_metadata",
                *type_metadata,
            );
        }
        HirAnalogOperator::LimiterArgument { .. } => {}
    }
}

fn validate_branch_access_node(
    diagnostics: &mut Vec<IrDiagnostic>,
    expression: &HirExpression,
    node: &SmolStr,
    node_names: &HashSet<SmolStr>,
    ground_nodes: &[SmolStr],
) {
    if node_names.contains(node) || node.as_str() == "0" || ground_nodes.contains(node) {
        return;
    }

    diagnostics.push(IrDiagnostic::error(
        CompilerPhase::MirValidation,
        format!("MIR unknown branch access node '{}'", node),
        expression.span,
    ));
}

fn validate_optional_expression_child(
    diagnostics: &mut Vec<IrDiagnostic>,
    expressions: &[HirExpression],
    expression: &HirExpression,
    label: &str,
    child: Option<ExprId>,
) {
    if let Some(child) = child {
        validate_expression_child(diagnostics, expressions, expression, label, child);
    }
}

fn validate_expression_child_list(
    diagnostics: &mut Vec<IrDiagnostic>,
    expressions: &[HirExpression],
    expression: &HirExpression,
    label: &str,
    children: &[ExprId],
) {
    for (index, child) in children.iter().copied().enumerate() {
        validate_expression_child(
            diagnostics,
            expressions,
            expression,
            &format!("{label}[{index}]"),
            child,
        );
    }
}

fn validate_expression_child(
    diagnostics: &mut Vec<IrDiagnostic>,
    expressions: &[HirExpression],
    expression: &HirExpression,
    label: &str,
    child: ExprId,
) {
    if usize::from(child) >= expressions.len() {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            format!(
                "MIR expression {} child {} {} is outside expression arena length {}",
                expression.id,
                label,
                child,
                expressions.len()
            ),
            expression.span,
        ));
        return;
    }

    if child.index() >= expression.id.index() {
        diagnostics.push(IrDiagnostic::error(
            CompilerPhase::MirValidation,
            format!(
                "MIR expression {} child {} {} violates expression postorder",
                expression.id, label, child
            ),
            expression.span,
        ));
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

fn validate_value_symbols(diagnostics: &mut Vec<IrDiagnostic>, value_symbols: &[SmolStr]) {
    let mut names = HashSet::new();
    let mut previous: Option<&SmolStr> = None;

    for symbol in value_symbols {
        if symbol.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                "MIR value symbol names must not be empty",
            ));
        } else if !names.insert(symbol.clone()) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!("MIR duplicate value symbol '{}'", symbol),
            ));
        }

        if let Some(previous) = previous
            && previous > symbol
        {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                "MIR value symbols must be sorted",
            ));
        }

        previous = Some(symbol);
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
        for (dimension_index, dimension) in parameter.dimensions.iter().enumerate() {
            validate_expr_ref(
                diagnostics,
                &format!(
                    "parameter '{}' dimension {} left bound",
                    parameter.name, dimension_index
                ),
                &dimension.left,
                expressions,
            );
            validate_expr_ref(
                diagnostics,
                &format!(
                    "parameter '{}' dimension {} right bound",
                    parameter.name, dimension_index
                ),
                &dimension.right,
                expressions,
            );
        }
        if let Some(default_expr) = &parameter.default_expr {
            validate_expr_ref(
                diagnostics,
                &format!("parameter '{}' default", parameter.name),
                default_expr,
                expressions,
            );
        }
        if let Some(range) = &parameter.range {
            if let Some(expression) = &range.min_expression {
                validate_expr_ref(
                    diagnostics,
                    &format!("parameter '{}' lower range bound", parameter.name),
                    expression,
                    expressions,
                );
            }
            if let Some(expression) = &range.max_expression {
                validate_expr_ref(
                    diagnostics,
                    &format!("parameter '{}' upper range bound", parameter.name),
                    expression,
                    expressions,
                );
            }
            for expression in &range.exclude_expressions {
                validate_expr_ref(
                    diagnostics,
                    &format!("parameter '{}' excluded range value", parameter.name),
                    expression,
                    expressions,
                );
            }
        }
    }
}

fn validate_state_slot_owners(
    diagnostics: &mut Vec<IrDiagnostic>,
    state_slots: &[MirStateSlot],
    equation_count: usize,
) {
    let mut names = HashSet::new();

    for state_slot in state_slots {
        if state_slot.name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!("MIR state slot {} name must not be empty", state_slot.id),
            ));
        } else if !names.insert(state_slot.name.clone()) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::MirValidation,
                format!("MIR duplicate state slot name '{}'", state_slot.name),
            ));
        }

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
