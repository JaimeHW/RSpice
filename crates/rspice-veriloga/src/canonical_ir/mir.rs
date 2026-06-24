use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashSet;

use super::hir::{HirContributionKind, HirModel};
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
    pub default: Option<f64>,
    pub aliases: Vec<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirStateSlot {
    pub id: StateId,
    pub name: SmolStr,
    pub owner: ContributionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirEquation {
    pub id: EquationId,
    pub contribution: ContributionId,
    pub branch: SmolStr,
    pub kind: MirEquationKind,
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
                default: parameter.default,
                aliases: parameter.aliases.clone(),
            })
            .collect();

        let equations = hir
            .contributions
            .iter()
            .enumerate()
            .map(|(index, contribution)| MirEquation {
                id: EquationId::from(index),
                contribution: contribution.id,
                branch: contribution.branch.clone(),
                kind: MirEquationKind::from(contribution.kind),
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
        };

        mir.validate().map(|()| mir)
    }

    pub fn validate(&self) -> IrValidationResult {
        let mut diagnostics = Vec::new();

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
        validate_node_names(&mut diagnostics, &self.nodes);
        validate_parameter_names_and_aliases(&mut diagnostics, &self.parameters);
        validate_state_slot_owners(&mut diagnostics, &self.state_slots, self.equations.len());
        validate_equations(&mut diagnostics, &self.equations);

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

fn validate_node_names(diagnostics: &mut Vec<IrDiagnostic>, nodes: &[MirNode]) {
    let mut names = HashSet::new();

    for node in nodes {
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

fn validate_equations(diagnostics: &mut Vec<IrDiagnostic>, equations: &[MirEquation]) {
    for equation in equations {
        if usize::from(equation.contribution) >= equations.len() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::MirValidation,
                format!(
                    "MIR equation {} contribution {} is out of range for {} equations",
                    equation.id,
                    equation.contribution,
                    equations.len()
                ),
                equation.span,
            ));
        }

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
    }
}
