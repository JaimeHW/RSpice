use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashSet;

use crate::ast::{
    AnalogOperator, BranchAccess, Expression, LaplaceKind, NoiseSource, PortDirection, ZiKind,
};
use crate::semantic::{AnalyzedModule, AnalyzedStatement};
use crate::types::{ParameterRange, ValueType};

use super::{
    ArrayId, BranchId, CanonicalMetadata, CompilerPhase, ContributionId, ExprId, IrDiagnostic,
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
    pub id: ExprId,
    pub kind: SmolStr,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirExpression {
    pub id: ExprId,
    pub kind: HirExprKind,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirLaplaceKind {
    ZeroPole {
        zeros: Vec<ExprId>,
        poles: Vec<ExprId>,
    },
    ZeroDenominator {
        zeros: Vec<ExprId>,
        denominator: Vec<ExprId>,
    },
    NumeratorPole {
        numerator: Vec<ExprId>,
        poles: Vec<ExprId>,
    },
    NumeratorDenominator {
        numerator: Vec<ExprId>,
        denominator: Vec<ExprId>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirZiKind {
    ZeroPole {
        zeros: Vec<ExprId>,
        poles: Vec<ExprId>,
    },
    ZeroDenominator {
        zeros: Vec<ExprId>,
        denominator: Vec<ExprId>,
    },
    NumeratorPole {
        numerator: Vec<ExprId>,
        poles: Vec<ExprId>,
    },
    NumeratorDenominator {
        numerator: Vec<ExprId>,
        denominator: Vec<ExprId>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirExprKind {
    Number {
        value: f64,
        raw: SmolStr,
    },
    StringLiteral {
        value: SmolStr,
    },
    Identifier {
        name: SmolStr,
    },
    SystemFunction {
        name: SmolStr,
        args: Vec<ExprId>,
    },
    Binary {
        op: SmolStr,
        left: ExprId,
        right: ExprId,
    },
    Unary {
        op: SmolStr,
        operand: ExprId,
    },
    Conditional {
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    },
    Call {
        name: SmolStr,
        args: Vec<ExprId>,
    },
    BranchAccess {
        access: SmolStr,
        pos: SmolStr,
        neg: Option<SmolStr>,
    },
    NamedBranchAccess {
        access: SmolStr,
        name: SmolStr,
    },
    ArrayAccess {
        array: SmolStr,
        index: ExprId,
    },
    ArrayLiteral {
        elements: Vec<ExprId>,
    },
    AnalogOperator {
        operator: SmolStr,
        operands: Vec<ExprId>,
    },
    Laplace {
        expr: ExprId,
        kind: HirLaplaceKind,
    },
    Zi {
        expr: ExprId,
        kind: HirZiKind,
    },
    NoiseSource {
        source: SmolStr,
        operands: Vec<ExprId>,
        name: Option<SmolStr>,
    },
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
    pub len: u32,
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
    pub expressions: Vec<HirExpression>,
    pub internal_nodes: Vec<HirInternalNode>,
    pub ground_nodes: Vec<SmolStr>,
}

impl HirModel {
    pub fn from_analyzed_module(metadata: &CanonicalMetadata, module: &AnalyzedModule) -> Self {
        let mut lowerer = HirLowerer::new();
        let mut parameters: Vec<_> = module
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| HirParameter {
                id: ParamId::from(index),
                name: parameter.name.clone(),
                value_type: CanonicalValueType::from(parameter.value_type),
                default: parameter.default,
                default_expr: parameter
                    .default_expr
                    .as_ref()
                    .map(|expr| lowerer.lower_expr(expr)),
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
                len: u32::try_from(array.len).expect("array length exceeds u32::MAX"),
            })
            .collect();

        let contributions = module
            .contributions
            .iter()
            .enumerate()
            .map(|(index, contribution)| HirContribution {
                id: ContributionId::from(index),
                branch: contribution.branch.clone(),
                kind: contribution_kind(contribution.indirect, contribution.is_current),
                expression: lowerer.lower_expr(&contribution.expression),
                expr_type: CanonicalValueType::from(contribution.expr_type),
                span: SourceSpanRef::from(contribution.span),
            })
            .collect();

        let statements = module
            .statements
            .iter()
            .map(|statement| lower_statement(&mut lowerer, statement))
            .collect();

        let expressions = lowerer.expressions;

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
            contributions,
            statements,
            expressions,
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
        validate_dense_branch_ids(&mut diagnostics, &self.branches);
        validate_dense_contribution_ids(&mut diagnostics, &self.contributions);
        validate_dense_internal_node_ids(&mut diagnostics, &self.internal_nodes);
        self.validate_expressions(&mut diagnostics);
        self.validate_arrays(&mut diagnostics);
        self.validate_parameter_aliases(&mut diagnostics);
        self.validate_parameter_expression_refs(&mut diagnostics);
        self.validate_branches(&mut diagnostics);
        self.validate_contributions(&mut diagnostics);
        self.validate_statements(&mut diagnostics, &self.statements);

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }

    fn validate_expressions(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        for (expected, expression) in self.expressions.iter().enumerate() {
            let expected = u32::try_from(expected).expect("HIR expression count exceeds u32::MAX");
            if expression.id.index() != expected {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR expression IDs must be dense: expected ExprId({}) at index {}, found {}",
                        expected, expected, expression.id
                    ),
                ));
            }

            self.validate_expression_children(diagnostics, expression);
        }
    }

    fn validate_expression_children(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
    ) {
        match &expression.kind {
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::Identifier { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
            HirExprKind::SystemFunction { args, .. } | HirExprKind::Call { args, .. } => {
                self.validate_expression_child_list(diagnostics, expression, "arg", args);
            }
            HirExprKind::Binary { left, right, .. } => {
                self.validate_expression_child(diagnostics, expression, "left", *left);
                self.validate_expression_child(diagnostics, expression, "right", *right);
            }
            HirExprKind::Unary { operand, .. } => {
                self.validate_expression_child(diagnostics, expression, "operand", *operand);
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.validate_expression_child(diagnostics, expression, "condition", *condition);
                self.validate_expression_child(diagnostics, expression, "then_expr", *then_expr);
                self.validate_expression_child(diagnostics, expression, "else_expr", *else_expr);
            }
            HirExprKind::ArrayAccess { index, .. } => {
                self.validate_expression_child(diagnostics, expression, "index", *index);
            }
            HirExprKind::ArrayLiteral { elements } => {
                self.validate_expression_child_list(diagnostics, expression, "element", elements);
            }
            HirExprKind::AnalogOperator { operands, .. } => {
                self.validate_expression_child_list(diagnostics, expression, "operand", operands);
            }
            HirExprKind::Laplace { expr, kind } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_laplace_children(diagnostics, expression, kind);
            }
            HirExprKind::Zi { expr, kind } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_zi_children(diagnostics, expression, kind);
            }
            HirExprKind::NoiseSource { operands, .. } => {
                self.validate_expression_child_list(diagnostics, expression, "operand", operands);
            }
        }
    }

    fn validate_laplace_children(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        kind: &HirLaplaceKind,
    ) {
        match kind {
            HirLaplaceKind::ZeroPole { zeros, poles } => {
                self.validate_expression_child_list(diagnostics, expression, "zeros", zeros);
                self.validate_expression_child_list(diagnostics, expression, "poles", poles);
            }
            HirLaplaceKind::ZeroDenominator { zeros, denominator } => {
                self.validate_expression_child_list(diagnostics, expression, "zeros", zeros);
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "denominator",
                    denominator,
                );
            }
            HirLaplaceKind::NumeratorPole { numerator, poles } => {
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "numerator",
                    numerator,
                );
                self.validate_expression_child_list(diagnostics, expression, "poles", poles);
            }
            HirLaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => {
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "numerator",
                    numerator,
                );
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "denominator",
                    denominator,
                );
            }
        }
    }

    fn validate_zi_children(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        kind: &HirZiKind,
    ) {
        match kind {
            HirZiKind::ZeroPole { zeros, poles } => {
                self.validate_expression_child_list(diagnostics, expression, "zeros", zeros);
                self.validate_expression_child_list(diagnostics, expression, "poles", poles);
            }
            HirZiKind::ZeroDenominator { zeros, denominator } => {
                self.validate_expression_child_list(diagnostics, expression, "zeros", zeros);
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "denominator",
                    denominator,
                );
            }
            HirZiKind::NumeratorPole { numerator, poles } => {
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "numerator",
                    numerator,
                );
                self.validate_expression_child_list(diagnostics, expression, "poles", poles);
            }
            HirZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => {
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "numerator",
                    numerator,
                );
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "denominator",
                    denominator,
                );
            }
        }
    }

    fn validate_expression_child_list(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        label: &str,
        children: &[ExprId],
    ) {
        for (index, child) in children.iter().copied().enumerate() {
            self.validate_expression_child(
                diagnostics,
                expression,
                &format!("{label}[{index}]"),
                child,
            );
        }
    }

    fn validate_expression_child(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        label: &str,
        child: ExprId,
    ) {
        if usize::from(child) >= self.expressions.len() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR expression {} child {} {} is outside expression arena length {}",
                    expression.id,
                    label,
                    child,
                    self.expressions.len()
                ),
                expression.span,
            ));
            return;
        }

        if child.index() >= expression.id.index() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR expression {} child {} {} violates expression postorder",
                    expression.id, label, child
                ),
                expression.span,
            ));
        }
    }

    fn validate_arrays(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let variable_count = self.variables.len();

        for array in &self.arrays {
            let base = usize::from(array.base);
            let len = usize::try_from(array.len).expect("HIR array len exceeds usize::MAX");
            let Some(end) = base.checked_add(len) else {
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
                        array.name, array.base, len, variable_count
                    ),
                ));
            }
        }
    }

    fn validate_parameter_aliases(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let parameter_names: HashSet<_> = self
            .parameters
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect();
        let mut observed_parameter_names = HashSet::new();
        let mut aliases = HashSet::new();

        for parameter in &self.parameters {
            if !observed_parameter_names.insert(parameter.name.clone()) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!("HIR duplicate parameter name '{}'", parameter.name),
                ));
            }

            for alias in &parameter.aliases {
                if alias.is_empty() {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR parameter alias for '{}' must not be empty",
                            parameter.name
                        ),
                    ));
                }

                if parameter_names.contains(alias) {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR parameter alias '{}' collides with parameter name",
                            alias
                        ),
                    ));
                }

                if !aliases.insert(alias.clone()) {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::HirValidation,
                        format!("HIR duplicate parameter alias '{}'", alias),
                    ));
                }
            }
        }
    }

    fn validate_parameter_expression_refs(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        for parameter in &self.parameters {
            if let Some(default_expr) = &parameter.default_expr {
                self.validate_expr_ref(
                    diagnostics,
                    &format!("parameter '{}' default", parameter.name),
                    default_expr,
                );
            }
        }
    }

    fn validate_branches(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let known_nodes = self.known_node_names();
        let mut branch_names = HashSet::new();

        for branch in &self.branches {
            if branch.name.is_empty() {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    "HIR branch name must not be empty",
                ));
            } else if !branch_names.insert(branch.name.clone()) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!("HIR duplicate branch name '{}'", branch.name),
                ));
            }

            if !known_nodes.contains(&branch.pos_node) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR branch '{}' pos_node '{}' is unknown",
                        branch.name, branch.pos_node
                    ),
                ));
            }

            if !branch.neg_node.is_empty() && !known_nodes.contains(&branch.neg_node) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR branch '{}' neg_node '{}' is unknown",
                        branch.name, branch.neg_node
                    ),
                ));
            }
        }
    }

    fn validate_contributions(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let known_nodes = self.known_node_names();
        let declared_branches: HashSet<_> = self
            .branches
            .iter()
            .map(|branch| branch.name.clone())
            .collect();

        for contribution in &self.contributions {
            self.validate_expr_ref(
                diagnostics,
                &format!("contribution {} expression", contribution.id.index()),
                &contribution.expression,
            );

            if contribution.branch.is_empty() {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    "HIR contribution branch name must not be empty",
                    contribution.span,
                ));
            } else if !is_valid_contribution_branch(
                contribution.branch.as_str(),
                &known_nodes,
                &declared_branches,
            ) {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    format!("HIR unknown contribution branch '{}'", contribution.branch),
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
                    self.validate_expr_ref(
                        diagnostics,
                        &format!("assignment '{}' expression", assignment.target_name),
                        &assignment.expr,
                    );
                    if let Some(index) = &assignment.index {
                        self.validate_expr_ref(
                            diagnostics,
                            &format!("assignment '{}' index", assignment.target_name),
                            index,
                        );
                    }

                    let target_index = usize::from(assignment.target);
                    if target_index >= self.variables.len() {
                        diagnostics.push(IrDiagnostic::error(
                            CompilerPhase::HirValidation,
                            format!(
                                "HIR assignment target {} is outside variable count {}",
                                assignment.target,
                                self.variables.len()
                            ),
                            assignment.span,
                        ));
                        continue;
                    }

                    self.validate_assignment_shape(diagnostics, assignment);
                }
                HirStatement::Loop(loop_statement) => {
                    self.validate_expr_ref(
                        diagnostics,
                        "loop condition",
                        &loop_statement.condition,
                    );
                    self.validate_statements(diagnostics, &loop_statement.body);
                }
            }
        }
    }

    fn validate_expr_ref(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        owner: &str,
        expr_ref: &HirExprRef,
    ) {
        let index = usize::from(expr_ref.id);
        let Some(expression) = self.expressions.get(index) else {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR expression ref {} id {} is outside expression arena length {}",
                    owner,
                    expr_ref.id,
                    self.expressions.len()
                ),
                expr_ref.span,
            ));
            return;
        };

        let actual_kind = hir_expr_kind_label(&expression.kind);
        if expr_ref.kind.as_str() != actual_kind {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR expression ref {} kind '{}' does not match '{}'",
                    owner, expr_ref.kind, actual_kind
                ),
                expr_ref.span,
            ));
        }
    }

    fn validate_assignment_shape(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        assignment: &HirAssignment,
    ) {
        let target_variable = &self.variables[usize::from(assignment.target)];
        let array_by_name = self
            .arrays
            .iter()
            .find(|array| array.name == assignment.target_name);
        let array_by_base = self
            .arrays
            .iter()
            .find(|array| array.base == assignment.target);

        if assignment.index.is_some() {
            if let Some(array) = array_by_name {
                if assignment.target != array.base {
                    diagnostics.push(IrDiagnostic::error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR indexed assignment '{}' target {} must match array base {}",
                            assignment.target_name, assignment.target, array.base
                        ),
                        assignment.span,
                    ));
                }
            } else if let Some(array) = array_by_base {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR indexed assignment target name '{}' does not match array '{}'",
                        assignment.target_name, array.name
                    ),
                    assignment.span,
                ));
            } else {
                if assignment.target_name != target_variable.name {
                    diagnostics.push(IrDiagnostic::error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR assignment target name '{}' does not match variable '{}'",
                            assignment.target_name, target_variable.name
                        ),
                        assignment.span,
                    ));
                }

                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR scalar assignment '{}' must not have an index",
                        assignment.target_name
                    ),
                    assignment.span,
                ));
            }
        } else if let Some(array) = array_by_name {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR array assignment '{}' must include an index",
                    array.name
                ),
                assignment.span,
            ));
        } else if assignment.target_name != target_variable.name {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR assignment target name '{}' does not match variable '{}'",
                    assignment.target_name, target_variable.name
                ),
                assignment.span,
            ));
        }
    }

    fn known_node_names(&self) -> HashSet<SmolStr> {
        let mut known = HashSet::new();
        known.insert("0".into());
        known.extend(self.ports.iter().map(|port| port.name.clone()));
        known.extend(self.internal_nodes.iter().map(|node| node.name.clone()));
        known.extend(self.ground_nodes.iter().cloned());
        known
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

fn validate_dense_branch_ids(diagnostics: &mut Vec<IrDiagnostic>, branches: &[HirBranch]) {
    for (expected, branch) in branches.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR branch count exceeds u32::MAX");
        if branch.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR branch IDs must be dense: expected BranchId({}) at index {}, found {}",
                    expected, expected, branch.id
                ),
            ));
        }
    }
}

fn validate_dense_contribution_ids(
    diagnostics: &mut Vec<IrDiagnostic>,
    contributions: &[HirContribution],
) {
    for (expected, contribution) in contributions.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR contribution count exceeds u32::MAX");
        if contribution.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR contribution IDs must be dense: expected ContributionId({}) at index {}, found {}",
                    expected, expected, contribution.id
                ),
            ));
        }
    }
}

fn validate_dense_internal_node_ids(
    diagnostics: &mut Vec<IrDiagnostic>,
    internal_nodes: &[HirInternalNode],
) {
    for (expected, internal_node) in internal_nodes.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR internal node count exceeds u32::MAX");
        if internal_node.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR internal node IDs must be dense: expected NodeId({}) at index {}, found {}",
                    expected, expected, internal_node.id
                ),
            ));
        }

        if internal_node.index != internal_node.id.index() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR internal node index {} does not match id {}",
                    internal_node.index, internal_node.id
                ),
            ));
        }
    }
}

fn is_valid_contribution_branch(
    branch: &str,
    known_nodes: &HashSet<SmolStr>,
    declared_branches: &HashSet<SmolStr>,
) -> bool {
    if declared_branches.contains(branch) {
        return true;
    }

    if let Some((pos, neg)) = branch.split_once(',') {
        return known_nodes.contains(pos) && known_nodes.contains(neg);
    }

    known_nodes.contains(branch)
}

fn lower_statement(lowerer: &mut HirLowerer, statement: &AnalyzedStatement) -> HirStatement {
    match statement {
        AnalyzedStatement::Assignment(assignment) => HirStatement::Assignment(HirAssignment {
            target: VariableId::from(assignment.var_index),
            target_name: assignment.target.clone(),
            index: assignment
                .index
                .as_ref()
                .map(|expr| lowerer.lower_expr(expr)),
            expr: lowerer.lower_expr(&assignment.expression),
            expr_type: CanonicalValueType::from(assignment.expr_type),
            span: SourceSpanRef::from(assignment.span),
        }),
        AnalyzedStatement::Loop(loop_statement) => HirStatement::Loop(HirLoop {
            condition: lowerer.lower_expr(&loop_statement.condition),
            body: loop_statement
                .body
                .iter()
                .map(|statement| lower_statement(lowerer, statement))
                .collect(),
            span: SourceSpanRef::from(loop_statement.span),
        }),
    }
}

#[derive(Debug, Default)]
struct HirLowerer {
    expressions: Vec<HirExpression>,
}

impl HirLowerer {
    fn new() -> Self {
        Self::default()
    }

    fn lower_expr(&mut self, expr: &Expression) -> HirExprRef {
        let kind_label = expression_kind(expr);
        let span = SourceSpanRef::from(expr.span());
        let kind = self.lower_expr_kind(expr);
        let id = ExprId::from(self.expressions.len());

        self.expressions.push(HirExpression { id, kind, span });

        HirExprRef {
            id,
            kind: kind_label,
            span,
        }
    }

    fn lower_expr_kind(&mut self, expr: &Expression) -> HirExprKind {
        match expr {
            Expression::Number(number) => HirExprKind::Number {
                value: number.value,
                raw: number.raw.clone(),
            },
            Expression::StringLit(string) => HirExprKind::StringLiteral {
                value: string.value.clone(),
            },
            Expression::Identifier(identifier) => HirExprKind::Identifier {
                name: identifier.name.clone(),
            },
            Expression::SystemFunction(function) => HirExprKind::SystemFunction {
                name: function.name.clone(),
                args: self.lower_expr_ids(&function.args),
            },
            Expression::Binary(binary) => HirExprKind::Binary {
                op: format!("{:?}", binary.op).into(),
                left: self.lower_expr(&binary.left).id,
                right: self.lower_expr(&binary.right).id,
            },
            Expression::Unary(unary) => HirExprKind::Unary {
                op: format!("{:?}", unary.op).into(),
                operand: self.lower_expr(&unary.operand).id,
            },
            Expression::Conditional(conditional) => HirExprKind::Conditional {
                condition: self.lower_expr(&conditional.condition).id,
                then_expr: self.lower_expr(&conditional.then_expr).id,
                else_expr: self.lower_expr(&conditional.else_expr).id,
            },
            Expression::Call(call) => HirExprKind::Call {
                name: call.name.clone(),
                args: self.lower_expr_ids(&call.args),
            },
            Expression::BranchAccess(access) => self.lower_branch_access_kind(access),
            Expression::ArrayAccess(array) => HirExprKind::ArrayAccess {
                array: array.array.clone(),
                index: self.lower_expr(&array.index).id,
            },
            Expression::ArrayLiteral(array) => HirExprKind::ArrayLiteral {
                elements: self.lower_expr_ids(&array.elements),
            },
            Expression::AnalogOperator(operator) => self.lower_analog_operator(operator),
            Expression::NoiseSource(source) => self.lower_noise_source(source),
        }
    }

    fn lower_expr_ids(&mut self, expressions: &[Expression]) -> Vec<ExprId> {
        expressions
            .iter()
            .map(|expr| self.lower_expr(expr).id)
            .collect()
    }

    fn lower_branch_access_expr(&mut self, access: &BranchAccess) -> ExprId {
        let id = ExprId::from(self.expressions.len());
        let span = SourceSpanRef::from(access.span());
        let kind = self.lower_branch_access_kind(access);

        self.expressions.push(HirExpression { id, kind, span });
        id
    }

    fn lower_branch_access_kind(&self, access: &BranchAccess) -> HirExprKind {
        match access {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => HirExprKind::BranchAccess {
                access: access.clone(),
                pos: pos.clone(),
                neg: neg.clone(),
            },
            BranchAccess::Branch { access, name, .. } => HirExprKind::NamedBranchAccess {
                access: access.clone(),
                name: name.clone(),
            },
        }
    }

    fn lower_optional_expr(&mut self, expression: &Option<Box<Expression>>) -> Vec<ExprId> {
        expression
            .iter()
            .map(|expr| self.lower_expr(expr).id)
            .collect()
    }

    fn lower_analog_operator(&mut self, operator: &AnalogOperator) -> HirExprKind {
        let (operator_name, operands) = match operator {
            AnalogOperator::Ddt { expr, abstol, .. } => {
                let mut operands = vec![self.lower_expr(expr).id];
                operands.extend(self.lower_optional_expr(abstol));
                ("Ddt", operands)
            }
            AnalogOperator::Idt {
                expr,
                ic,
                assert_val,
                abstol,
                ..
            } => {
                let mut operands = vec![self.lower_expr(expr).id];
                operands.extend(self.lower_optional_expr(ic));
                operands.extend(self.lower_optional_expr(assert_val));
                operands.extend(self.lower_optional_expr(abstol));
                ("Idt", operands)
            }
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
                ..
            } => {
                let mut operands = vec![self.lower_expr(expr).id];
                operands.extend(self.lower_optional_expr(ic));
                operands.extend(self.lower_optional_expr(modulus));
                operands.extend(self.lower_optional_expr(offset));
                operands.extend(self.lower_optional_expr(abstol));
                ("IdtMod", operands)
            }
            AnalogOperator::Ddx { expr, probe, .. } => {
                let operands = vec![
                    self.lower_expr(expr).id,
                    self.lower_branch_access_expr(probe),
                ];
                ("Ddx", operands)
            }
            AnalogOperator::Limexp { expr, .. } => ("Limexp", vec![self.lower_expr(expr).id]),
            AnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
                ..
            } => {
                let mut operands = vec![self.lower_expr(expr).id, self.lower_expr(delay).id];
                operands.extend(self.lower_optional_expr(max_delay));
                ("Absdelay", operands)
            }
            AnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                ..
            } => {
                let mut operands = vec![self.lower_expr(expr).id];
                operands.extend(self.lower_optional_expr(delay));
                operands.extend(self.lower_optional_expr(rise));
                operands.extend(self.lower_optional_expr(fall));
                operands.extend(self.lower_optional_expr(tolerance));
                ("Transition", operands)
            }
            AnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
                ..
            } => {
                let mut operands = vec![self.lower_expr(expr).id];
                operands.extend(self.lower_optional_expr(max_rise));
                operands.extend(self.lower_optional_expr(max_fall));
                ("Slew", operands)
            }
            AnalogOperator::LastCrossing { expr, edge, .. } => {
                let operator_name = match edge {
                    Some(direction) => SmolStr::from(format!("LastCrossing::{direction:?}")),
                    None => "LastCrossing".into(),
                };
                return HirExprKind::AnalogOperator {
                    operator: operator_name,
                    operands: vec![self.lower_expr(expr).id],
                };
            }
            AnalogOperator::Laplace { kind, expr, .. } => {
                return HirExprKind::Laplace {
                    expr: self.lower_expr(expr).id,
                    kind: self.lower_laplace_kind(kind),
                };
            }
            AnalogOperator::Zi { kind, expr, .. } => {
                return HirExprKind::Zi {
                    expr: self.lower_expr(expr).id,
                    kind: self.lower_zi_kind(kind),
                };
            }
        };

        HirExprKind::AnalogOperator {
            operator: operator_name.into(),
            operands,
        }
    }

    fn lower_laplace_kind(&mut self, kind: &LaplaceKind) -> HirLaplaceKind {
        match kind {
            LaplaceKind::ZeroPole { zeros, poles } => HirLaplaceKind::ZeroPole {
                zeros: self.lower_expr_ids(zeros),
                poles: self.lower_expr_ids(poles),
            },
            LaplaceKind::ZeroDenominator { zeros, denominator } => {
                HirLaplaceKind::ZeroDenominator {
                    zeros: self.lower_expr_ids(zeros),
                    denominator: self.lower_expr_ids(denominator),
                }
            }
            LaplaceKind::NumeratorPole { numerator, poles } => HirLaplaceKind::NumeratorPole {
                numerator: self.lower_expr_ids(numerator),
                poles: self.lower_expr_ids(poles),
            },
            LaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => HirLaplaceKind::NumeratorDenominator {
                numerator: self.lower_expr_ids(numerator),
                denominator: self.lower_expr_ids(denominator),
            },
        }
    }

    fn lower_zi_kind(&mut self, kind: &ZiKind) -> HirZiKind {
        match kind {
            ZiKind::ZeroPole { zeros, poles } => HirZiKind::ZeroPole {
                zeros: self.lower_expr_ids(zeros),
                poles: self.lower_expr_ids(poles),
            },
            ZiKind::ZeroDenominator { zeros, denominator } => HirZiKind::ZeroDenominator {
                zeros: self.lower_expr_ids(zeros),
                denominator: self.lower_expr_ids(denominator),
            },
            ZiKind::NumeratorPole { numerator, poles } => HirZiKind::NumeratorPole {
                numerator: self.lower_expr_ids(numerator),
                poles: self.lower_expr_ids(poles),
            },
            ZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => HirZiKind::NumeratorDenominator {
                numerator: self.lower_expr_ids(numerator),
                denominator: self.lower_expr_ids(denominator),
            },
        }
    }

    fn lower_noise_source(&mut self, source: &NoiseSource) -> HirExprKind {
        match source {
            NoiseSource::White { power, name, .. } => HirExprKind::NoiseSource {
                source: "White".into(),
                operands: vec![self.lower_expr(power).id],
                name: name.clone(),
            },
            NoiseSource::Flicker {
                power,
                exponent,
                name,
                ..
            } => HirExprKind::NoiseSource {
                source: "Flicker".into(),
                operands: vec![self.lower_expr(power).id, self.lower_expr(exponent).id],
                name: name.clone(),
            },
            NoiseSource::Table { data, name, .. } => HirExprKind::NoiseSource {
                source: "Table".into(),
                operands: self.lower_expr_ids(data),
                name: name.clone(),
            },
        }
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
