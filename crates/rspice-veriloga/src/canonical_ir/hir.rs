use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashSet;

use crate::ast::{Expression, PortDirection};
use crate::semantic::{AnalyzedModule, AnalyzedStatement};
use crate::types::{ParameterRange, ValueType};

use super::{
    ArrayId, BranchId, CanonicalMetadata, CompilerPhase, ContributionId, IrDiagnostic,
    IrValidationResult, ModuleId, NodeId, ParamId, PortId, SourceSpanRef, VariableId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalValueType {
    Real,
    Integer,
    String,
    Boolean,
    NatureAccess,
    Void,
    Unknown,
    Error,
}

impl From<ValueType> for CanonicalValueType {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Real => Self::Real,
            ValueType::Integer => Self::Integer,
            ValueType::String => Self::String,
            ValueType::Boolean => Self::Boolean,
            ValueType::NatureAccess => Self::NatureAccess,
            ValueType::Void => Self::Void,
            ValueType::Unknown => Self::Unknown,
            ValueType::Error => Self::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParamRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_exclusive: bool,
    pub max_exclusive: bool,
    pub exclude: Vec<f64>,
}

impl HirParamRange {
    pub fn from_range(range: &ParameterRange) -> Self {
        Self {
            min: range.min,
            max: range.max,
            min_exclusive: range.min_exclusive,
            max_exclusive: range.max_exclusive,
            exclude: range.exclude.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirExprRef {
    pub kind: SmolStr,
    pub span: SourceSpanRef,
}

impl HirExprRef {
    pub fn from_expr(expr: &Expression) -> Self {
        Self {
            kind: expression_kind(expr),
            span: SourceSpanRef::from(expr.span()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirPort {
    pub id: PortId,
    pub name: SmolStr,
    pub direction: SmolStr,
    pub discipline: SmolStr,
    pub nature_potential: Option<SmolStr>,
    pub nature_flow: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParameter {
    pub id: ParamId,
    pub name: SmolStr,
    pub value_type: CanonicalValueType,
    pub default: Option<f64>,
    pub default_expr: Option<HirExprRef>,
    pub range: Option<HirParamRange>,
    pub aliases: Vec<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirVariable {
    pub id: VariableId,
    pub name: SmolStr,
    pub value_type: CanonicalValueType,
    pub is_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirArray {
    pub id: ArrayId,
    pub name: SmolStr,
    pub base: VariableId,
    pub lower: i64,
    pub len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirBranch {
    pub id: BranchId,
    pub name: SmolStr,
    pub pos_node: SmolStr,
    pub neg_node: SmolStr,
    pub discipline: SmolStr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirInternalNode {
    pub id: NodeId,
    pub name: SmolStr,
    pub discipline: SmolStr,
    pub index: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirContributionKind {
    Current,
    Potential,
    Indirect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirContribution {
    pub id: ContributionId,
    pub branch: SmolStr,
    pub kind: HirContributionKind,
    pub expression: HirExprRef,
    pub expr_type: CanonicalValueType,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirAssignment {
    pub target: VariableId,
    pub target_name: SmolStr,
    pub index: Option<HirExprRef>,
    pub expr: HirExprRef,
    pub expr_type: CanonicalValueType,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirLoop {
    pub condition: HirExprRef,
    pub body: Vec<HirStatement>,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirStatement {
    Assignment(HirAssignment),
    Loop(HirLoop),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirModel {
    pub module_id: ModuleId,
    pub module_name: SmolStr,
    pub schema_version: u32,
    pub source_package: SmolStr,
    pub source_digest: SmolStr,
    pub compiler_version: SmolStr,
    pub feature_flags: Vec<SmolStr>,
    pub ports: Vec<HirPort>,
    pub parameters: Vec<HirParameter>,
    pub variables: Vec<HirVariable>,
    pub arrays: Vec<HirArray>,
    pub branches: Vec<HirBranch>,
    pub contributions: Vec<HirContribution>,
    pub statements: Vec<HirStatement>,
    pub internal_nodes: Vec<HirInternalNode>,
    pub ground_nodes: Vec<SmolStr>,
}

impl HirModel {
    pub fn from_analyzed_module(metadata: &CanonicalMetadata, module: &AnalyzedModule) -> Self {
        let mut parameters: Vec<_> = module
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| HirParameter {
                id: ParamId::from(index),
                name: parameter.name.clone(),
                value_type: CanonicalValueType::from(parameter.value_type),
                default: parameter.default,
                default_expr: parameter.default_expr.as_ref().map(HirExprRef::from_expr),
                range: parameter.range.as_ref().map(HirParamRange::from_range),
                aliases: Vec::new(),
            })
            .collect();

        for alias in &module.param_aliases {
            if let Some(parameter) = parameters.get_mut(alias.target) {
                parameter.aliases.push(alias.alias.clone());
            }
        }

        let mut arrays: Vec<_> = module.arrays.iter().collect();
        arrays.sort_by(|(left_name, _), (right_name, _)| left_name.cmp(right_name));
        let arrays = arrays
            .into_iter()
            .enumerate()
            .map(|(index, (name, array))| HirArray {
                id: ArrayId::from(index),
                name: name.clone(),
                base: VariableId::from(array.base),
                lower: array.lower,
                len: array.len,
            })
            .collect();

        Self {
            module_id: ModuleId::new(0),
            module_name: module.name.clone(),
            schema_version: metadata.schema_version,
            source_package: metadata.source_package.clone(),
            source_digest: metadata.source_digest.clone(),
            compiler_version: metadata.compiler_version.clone(),
            feature_flags: metadata.feature_flags.clone(),
            ports: module
                .ports
                .iter()
                .enumerate()
                .map(|(index, port)| HirPort {
                    id: PortId::from(index),
                    name: port.name.clone(),
                    direction: port_direction_label(port.direction),
                    discipline: port.discipline.clone(),
                    nature_potential: port.nature_potential.clone(),
                    nature_flow: port.nature_flow.clone(),
                })
                .collect(),
            parameters,
            variables: module
                .variables
                .iter()
                .enumerate()
                .map(|(index, variable)| HirVariable {
                    id: VariableId::from(index),
                    name: variable.name.clone(),
                    value_type: CanonicalValueType::from(variable.value_type),
                    is_state: variable.is_state,
                })
                .collect(),
            arrays,
            branches: module
                .branches
                .iter()
                .enumerate()
                .map(|(index, branch)| HirBranch {
                    id: BranchId::from(index),
                    name: branch.name.clone(),
                    pos_node: branch.pos_node.clone(),
                    neg_node: branch.neg_node.clone(),
                    discipline: branch.discipline.clone(),
                })
                .collect(),
            contributions: module
                .contributions
                .iter()
                .enumerate()
                .map(|(index, contribution)| HirContribution {
                    id: ContributionId::from(index),
                    branch: contribution.branch.clone(),
                    kind: contribution_kind(contribution.indirect, contribution.is_current),
                    expression: HirExprRef::from_expr(&contribution.expression),
                    expr_type: CanonicalValueType::from(contribution.expr_type),
                    span: SourceSpanRef::from(contribution.span),
                })
                .collect(),
            statements: module.statements.iter().map(lower_statement).collect(),
            internal_nodes: module
                .internal_nodes
                .iter()
                .map(|node| HirInternalNode {
                    id: NodeId::from(node.index),
                    name: node.name.clone(),
                    discipline: node.discipline.clone(),
                    index: u32::try_from(node.index).expect("internal node index exceeds u32::MAX"),
                })
                .collect(),
            ground_nodes: module.ground_nodes.clone(),
        }
    }

    pub fn validate(&self) -> IrValidationResult {
        let mut diagnostics = Vec::new();

        if self.module_name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                "HIR module name must not be empty",
            ));
        }

        if self.ports.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                "HIR module must have at least one port",
            ));
        }

        validate_dense_port_ids(&mut diagnostics, &self.ports);
        validate_dense_parameter_ids(&mut diagnostics, &self.parameters);
        validate_dense_variable_ids(&mut diagnostics, &self.variables);
        validate_dense_array_ids(&mut diagnostics, &self.arrays);
        self.validate_arrays(&mut diagnostics);
        self.validate_parameter_aliases(&mut diagnostics);
        self.validate_contributions(&mut diagnostics);
        self.validate_statements(&mut diagnostics, &self.statements);

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }

    fn validate_arrays(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let variable_count = self.variables.len();

        for array in &self.arrays {
            let base = usize::from(array.base);
            let Some(end) = base.checked_add(array.len) else {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR array '{}' base overflows variable index space",
                        array.name
                    ),
                ));
                continue;
            };

            if base >= variable_count || end > variable_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR array '{}' base {} with len {} exceeds variable count {}",
                        array.name, array.base, array.len, variable_count
                    ),
                ));
            }
        }
    }

    fn validate_parameter_aliases(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let mut aliases = HashSet::new();

        for parameter in &self.parameters {
            for alias in &parameter.aliases {
                if !aliases.insert(alias.clone()) {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::HirValidation,
                        format!("HIR duplicate parameter alias '{}'", alias),
                    ));
                }
            }
        }
    }

    fn validate_contributions(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        for contribution in &self.contributions {
            if contribution.branch.is_empty() {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    "HIR contribution branch name must not be empty",
                    contribution.span,
                ));
            }
        }
    }

    fn validate_statements(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        statements: &[HirStatement],
    ) {
        for statement in statements {
            match statement {
                HirStatement::Assignment(assignment) => {
                    if usize::from(assignment.target) >= self.variables.len() {
                        diagnostics.push(IrDiagnostic::error(
                            CompilerPhase::HirValidation,
                            format!(
                                "HIR assignment target {} is outside variable count {}",
                                assignment.target,
                                self.variables.len()
                            ),
                            assignment.span,
                        ));
                    }
                }
                HirStatement::Loop(loop_statement) => {
                    self.validate_statements(diagnostics, &loop_statement.body);
                }
            }
        }
    }
}

fn validate_dense_port_ids(diagnostics: &mut Vec<IrDiagnostic>, ports: &[HirPort]) {
    for (expected, port) in ports.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR port count exceeds u32::MAX");
        if port.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR port IDs must be dense: expected PortId({}) at index {}, found {}",
                    expected, expected, port.id
                ),
            ));
        }
    }
}

fn validate_dense_parameter_ids(diagnostics: &mut Vec<IrDiagnostic>, parameters: &[HirParameter]) {
    for (expected, parameter) in parameters.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR parameter count exceeds u32::MAX");
        if parameter.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR parameter IDs must be dense: expected ParamId({}) at index {}, found {}",
                    expected, expected, parameter.id
                ),
            ));
        }
    }
}

fn validate_dense_variable_ids(diagnostics: &mut Vec<IrDiagnostic>, variables: &[HirVariable]) {
    for (expected, variable) in variables.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR variable count exceeds u32::MAX");
        if variable.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR variable IDs must be dense: expected VariableId({}) at index {}, found {}",
                    expected, expected, variable.id
                ),
            ));
        }
    }
}

fn validate_dense_array_ids(diagnostics: &mut Vec<IrDiagnostic>, arrays: &[HirArray]) {
    for (expected, array) in arrays.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR array count exceeds u32::MAX");
        if array.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR array IDs must be dense: expected ArrayId({}) at index {}, found {}",
                    expected, expected, array.id
                ),
            ));
        }
    }
}

fn lower_statement(statement: &AnalyzedStatement) -> HirStatement {
    match statement {
        AnalyzedStatement::Assignment(assignment) => HirStatement::Assignment(HirAssignment {
            target: VariableId::from(assignment.var_index),
            target_name: assignment.target.clone(),
            index: assignment.index.as_ref().map(HirExprRef::from_expr),
            expr: HirExprRef::from_expr(&assignment.expression),
            expr_type: CanonicalValueType::from(assignment.expr_type),
            span: SourceSpanRef::from(assignment.span),
        }),
        AnalyzedStatement::Loop(loop_statement) => HirStatement::Loop(HirLoop {
            condition: HirExprRef::from_expr(&loop_statement.condition),
            body: loop_statement.body.iter().map(lower_statement).collect(),
            span: SourceSpanRef::from(loop_statement.span),
        }),
    }
}

fn contribution_kind(indirect: bool, is_current: bool) -> HirContributionKind {
    if indirect {
        HirContributionKind::Indirect
    } else if is_current {
        HirContributionKind::Current
    } else {
        HirContributionKind::Potential
    }
}

fn expression_kind(expr: &Expression) -> SmolStr {
    match expr {
        Expression::Number(_) => "number",
        Expression::StringLit(_) => "string",
        Expression::Identifier(_) => "identifier",
        Expression::SystemFunction(_) => "system_function",
        Expression::Binary(_) => "binary",
        Expression::Unary(_) => "unary",
        Expression::Conditional(_) => "conditional",
        Expression::Call(_) => "call",
        Expression::BranchAccess(_) => "branch_access",
        Expression::ArrayAccess(_) => "array_access",
        Expression::ArrayLiteral(_) => "array_literal",
        Expression::AnalogOperator(_) => "analog_operator",
        Expression::NoiseSource(_) => "noise_source",
    }
    .into()
}

fn port_direction_label(direction: PortDirection) -> SmolStr {
    match direction {
        PortDirection::Input => "input",
        PortDirection::Output => "output",
        PortDirection::Inout => "inout",
    }
    .into()
}
