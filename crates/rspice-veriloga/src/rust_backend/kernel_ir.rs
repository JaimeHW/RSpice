use std::collections::HashSet;

use crate::canonical_ir::{
    CanonicalIrArtifact, ExprId, HirAnalogOperator, HirExprKind, HirStatement, OptOp,
};

use super::{RustBackendError, RustDerivativeStorage, RustKernelPlan, RustKernelTier};

// Below these limits direct scalar Rust is both compact and faster than
// setting up a derivative kernel. Above them, LLVM compile cost grows with the
// scalarized graph rather than with the original structured model.
const MAX_DIRECT_SCALAR_VALUES: usize = 12_000;
const MAX_DIRECT_SCALAR_OPTIMIZER_NODES: usize = 32_000;
const MIN_STRUCTURED_EXPANSION_RATIO: usize = 3;

fn scalar_expansion_ratio(scalar_optimizer_nodes: usize, structured_operations: usize) -> usize {
    scalar_optimizer_nodes / structured_operations.max(1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreferredKernelTier {
    DirectScalar,
    SparseLocal,
    Structured,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DerivativeStorageStrategy {
    Scalar,
    Sparse,
    Dense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum KernelRegionKind {
    Assignment,
    Loop,
    Equation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KernelRegionPlan {
    pub kind: KernelRegionKind,
    pub root: ExprId,
    pub operation_count: usize,
    pub children: Vec<KernelRegionPlan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct KernelMetrics {
    pub scalar_values: usize,
    pub scalar_derivative_entries: usize,
    pub scalar_optimizer_nodes: usize,
    pub structured_expressions: usize,
    pub structured_operations: usize,
    pub structured_control_regions: usize,
    pub runtime_loop_operations: usize,
    pub derivative_lanes: usize,
    pub maximum_value_derivative_lanes: usize,
    pub scalar_expansion_ratio: usize,
}

impl KernelMetrics {
    fn preferred_tier(self) -> PreferredKernelTier {
        let scalar_graph_is_large = self.scalar_values > MAX_DIRECT_SCALAR_VALUES
            || self.scalar_optimizer_nodes > MAX_DIRECT_SCALAR_OPTIMIZER_NODES;
        let structured_model_is_materially_smaller =
            self.scalar_expansion_ratio >= MIN_STRUCTURED_EXPANSION_RATIO;
        if !scalar_graph_is_large {
            PreferredKernelTier::DirectScalar
        } else if structured_model_is_materially_smaller {
            PreferredKernelTier::Structured
        } else {
            PreferredKernelTier::SparseLocal
        }
    }

    fn derivative_storage(self) -> DerivativeStorageStrategy {
        if self.scalar_derivative_entries == 0 || self.derivative_lanes == 0 {
            return DerivativeStorageStrategy::Scalar;
        }
        let active_values = self.scalar_values.max(1);
        let possible = active_values.saturating_mul(self.derivative_lanes);
        if self.scalar_derivative_entries.saturating_mul(3) < possible {
            DerivativeStorageStrategy::Sparse
        } else {
            DerivativeStorageStrategy::Dense
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KernelPlan {
    metrics: KernelMetrics,
    statement_regions: Vec<KernelRegionPlan>,
    equation_regions: Vec<KernelRegionPlan>,
}

impl KernelPlan {
    pub(super) fn analyze(artifact: &CanonicalIrArtifact) -> Result<Self, RustBackendError> {
        artifact.validate().map_err(|diagnostics| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("cannot plan invalid canonical IR: {diagnostics:?}"),
            )
        })?;

        let mut statement_regions = Vec::new();
        lower_statement_regions(artifact, &artifact.hir.statements, &mut statement_regions)?;
        let equation_regions = artifact
            .mir
            .equations
            .iter()
            .map(|equation| {
                expression_region(artifact, KernelRegionKind::Equation, equation.expression.id)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let scalar_values = artifact.opt.values.len();
        let scalar_derivative_entries = artifact
            .opt
            .values
            .iter()
            .map(|value| value.derivatives.len())
            .sum::<usize>();
        let scalar_optimizer_nodes = scalar_values.saturating_add(scalar_derivative_entries);
        let structured_expressions = artifact.mir.expressions.len();
        let structured_operations = statement_regions
            .iter()
            .chain(&equation_regions)
            .map(region_operation_count)
            .sum::<usize>()
            .max(1);
        let structured_control_regions = count_control_regions(&statement_regions)
            + artifact
                .mir
                .expressions
                .iter()
                .filter(|expression| matches!(expression.kind, HirExprKind::Conditional { .. }))
                .count();
        let runtime_loop_operations = artifact
            .opt
            .runtime_loops
            .iter()
            .map(|runtime_loop| runtime_loop.assignments.len().saturating_add(1))
            .sum();
        let derivative_lanes = artifact
            .mir
            .nodes
            .len()
            .saturating_add(artifact.mir.branch_unknowns.len());
        let maximum_value_derivative_lanes = artifact
            .opt
            .values
            .iter()
            .map(|value| value.derivatives.len())
            .max()
            .unwrap_or(0);
        // This is a threshold metric: a model is "at least N times larger"
        // only when the complete integer ratio reaches N.  Rounding up here
        // would classify every expansion above 2x as 3x and can route sparse
        // models through the substantially heavier structured backend.
        let scalar_expansion_ratio =
            scalar_expansion_ratio(scalar_optimizer_nodes, structured_operations);

        let plan = Self {
            metrics: KernelMetrics {
                scalar_values,
                scalar_derivative_entries,
                scalar_optimizer_nodes,
                structured_expressions,
                structured_operations,
                structured_control_regions,
                runtime_loop_operations,
                derivative_lanes,
                maximum_value_derivative_lanes,
                scalar_expansion_ratio,
            },
            statement_regions,
            equation_regions,
        };
        plan.validate(artifact)?;
        Ok(plan)
    }

    pub(super) fn preferred_tier(&self) -> PreferredKernelTier {
        self.metrics.preferred_tier()
    }

    pub(super) fn derivative_storage(&self) -> DerivativeStorageStrategy {
        self.metrics.derivative_storage()
    }

    pub(super) fn summary(&self, artifact: &CanonicalIrArtifact) -> RustKernelPlan {
        RustKernelPlan {
            preferred_tier: match self.preferred_tier() {
                PreferredKernelTier::DirectScalar => RustKernelTier::DirectScalar,
                PreferredKernelTier::SparseLocal => RustKernelTier::SparseLocal,
                PreferredKernelTier::Structured => RustKernelTier::Structured,
            },
            derivative_storage: match self.derivative_storage() {
                DerivativeStorageStrategy::Scalar => RustDerivativeStorage::Scalar,
                DerivativeStorageStrategy::Sparse => RustDerivativeStorage::Sparse,
                DerivativeStorageStrategy::Dense => RustDerivativeStorage::Dense,
            },
            scalar_values: self.metrics.scalar_values,
            scalar_derivative_entries: self.metrics.scalar_derivative_entries,
            scalar_optimizer_nodes: self.metrics.scalar_optimizer_nodes,
            structured_expressions: self.metrics.structured_expressions,
            structured_operations: self.metrics.structured_operations,
            structured_control_regions: self.metrics.structured_control_regions,
            runtime_loop_operations: self.metrics.runtime_loop_operations,
            scheduled_opt_operations: scheduled_opt_operations(artifact),
            derivative_lanes: self.metrics.derivative_lanes,
            maximum_value_derivative_lanes: self.metrics.maximum_value_derivative_lanes,
            scalar_expansion_ratio: self.metrics.scalar_expansion_ratio,
            statement_regions: count_regions(&self.statement_regions),
            equation_regions: count_regions(&self.equation_regions),
        }
    }

    fn validate(&self, artifact: &CanonicalIrArtifact) -> Result<(), RustBackendError> {
        for region in self.statement_regions.iter().chain(&self.equation_regions) {
            validate_region(artifact, region)?;
        }
        Ok(())
    }
}

fn lower_statement_regions(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    out: &mut Vec<KernelRegionPlan>,
) -> Result<(), RustBackendError> {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => out.push(expression_region(
                artifact,
                KernelRegionKind::Assignment,
                assignment.expr.id,
            )?),
            HirStatement::Loop(loop_statement) => {
                let mut children = Vec::new();
                lower_statement_regions(artifact, &loop_statement.body, &mut children)?;
                let mut region = expression_region(
                    artifact,
                    KernelRegionKind::Loop,
                    loop_statement.condition.id,
                )?;
                region.children = children;
                out.push(region);
            }
        }
    }
    Ok(())
}

fn expression_region(
    artifact: &CanonicalIrArtifact,
    kind: KernelRegionKind,
    root: ExprId,
) -> Result<KernelRegionPlan, RustBackendError> {
    let mut visited = HashSet::new();
    let operation_count = visit_expression_operations(artifact, root, &mut visited)?.max(1);
    Ok(KernelRegionPlan {
        kind,
        root,
        operation_count,
        children: Vec::new(),
    })
}

fn visit_expression_operations(
    artifact: &CanonicalIrArtifact,
    id: ExprId,
    visited: &mut HashSet<ExprId>,
) -> Result<usize, RustBackendError> {
    if !visited.insert(id) {
        return Ok(0);
    }
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(id))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("kernel expression {id} is outside the MIR arena"),
            )
        })?;
    let mut operations = usize::from(!matches!(
        expression.kind,
        HirExprKind::Number { .. } | HirExprKind::StringLiteral { .. }
    ));
    for child in expression_children(&expression.kind) {
        operations =
            operations.saturating_add(visit_expression_operations(artifact, child, visited)?);
    }
    Ok(operations)
}

fn expression_children(kind: &HirExprKind) -> Vec<ExprId> {
    match kind {
        HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. } => Vec::new(),
        HirExprKind::SystemFunction { args, .. } | HirExprKind::Call { args, .. } => args.clone(),
        HirExprKind::Binary { left, right, .. } => vec![*left, *right],
        HirExprKind::Unary { operand, .. } => vec![*operand],
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => vec![*condition, *then_expr, *else_expr],
        HirExprKind::ArrayAccess { index, .. } => vec![*index],
        HirExprKind::ArrayLiteral { elements } => elements.clone(),
        HirExprKind::AnalogOperator { op } => analog_operator_children(op),
        HirExprKind::Laplace { expr, kind } => {
            let mut children = vec![*expr];
            match kind {
                crate::canonical_ir::HirLaplaceKind::ZeroPole { zeros, poles } => {
                    children.extend(zeros.iter().copied());
                    children.extend(poles.iter().copied());
                }
                crate::canonical_ir::HirLaplaceKind::ZeroDenominator { zeros, denominator } => {
                    children.extend(zeros.iter().copied());
                    children.extend(denominator.iter().copied());
                }
                crate::canonical_ir::HirLaplaceKind::NumeratorPole { numerator, poles } => {
                    children.extend(numerator.iter().copied());
                    children.extend(poles.iter().copied());
                }
                crate::canonical_ir::HirLaplaceKind::NumeratorDenominator {
                    numerator,
                    denominator,
                } => {
                    children.extend(numerator.iter().copied());
                    children.extend(denominator.iter().copied());
                }
            }
            children
        }
        HirExprKind::Zi { expr, kind } => {
            let mut children = vec![*expr];
            match kind {
                crate::canonical_ir::HirZiKind::ZeroPole { zeros, poles } => {
                    children.extend(zeros.iter().copied());
                    children.extend(poles.iter().copied());
                }
                crate::canonical_ir::HirZiKind::ZeroDenominator { zeros, denominator } => {
                    children.extend(zeros.iter().copied());
                    children.extend(denominator.iter().copied());
                }
                crate::canonical_ir::HirZiKind::NumeratorPole { numerator, poles } => {
                    children.extend(numerator.iter().copied());
                    children.extend(poles.iter().copied());
                }
                crate::canonical_ir::HirZiKind::NumeratorDenominator {
                    numerator,
                    denominator,
                } => {
                    children.extend(numerator.iter().copied());
                    children.extend(denominator.iter().copied());
                }
            }
            children
        }
        HirExprKind::NoiseSource { operands, .. } => operands.clone(),
    }
}

fn analog_operator_children(op: &HirAnalogOperator) -> Vec<ExprId> {
    match op {
        HirAnalogOperator::Limit {
            proposed,
            candidate,
            type_metadata,
            ..
        } => optional_children(*proposed, [Some(*candidate), *type_metadata]),
        HirAnalogOperator::LimiterArgument { .. } => Vec::new(),
        HirAnalogOperator::Ddt { expr, abstol } => optional_children(*expr, [*abstol]),
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => optional_children(*expr, [*ic, *assert, *abstol]),
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => optional_children(*expr, [*ic, *modulus, *offset, *abstol]),
        HirAnalogOperator::Ddx { expr, probe } => vec![*expr, *probe],
        HirAnalogOperator::Limexp { expr } | HirAnalogOperator::LastCrossing { expr, .. } => {
            vec![*expr]
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => optional_children(*expr, [Some(*delay), *max_delay]),
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => optional_children(*expr, [*delay, *rise, *fall, *tolerance]),
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => optional_children(*expr, [*max_rise, *max_fall]),
    }
}

fn optional_children<const N: usize>(root: ExprId, rest: [Option<ExprId>; N]) -> Vec<ExprId> {
    let mut children = Vec::with_capacity(N + 1);
    children.push(root);
    children.extend(rest.into_iter().flatten());
    children
}

fn region_operation_count(region: &KernelRegionPlan) -> usize {
    region.operation_count.saturating_add(
        region
            .children
            .iter()
            .map(region_operation_count)
            .sum::<usize>(),
    )
}

fn count_control_regions(regions: &[KernelRegionPlan]) -> usize {
    regions
        .iter()
        .map(|region| {
            usize::from(region.kind == KernelRegionKind::Loop)
                + count_control_regions(&region.children)
        })
        .sum()
}

fn count_regions(regions: &[KernelRegionPlan]) -> usize {
    regions
        .iter()
        .map(|region| 1usize.saturating_add(count_regions(&region.children)))
        .sum()
}

fn validate_region(
    artifact: &CanonicalIrArtifact,
    region: &KernelRegionPlan,
) -> Result<(), RustBackendError> {
    if region.operation_count == 0 || usize::from(region.root) >= artifact.mir.expressions.len() {
        return Err(RustBackendError::internal(
            artifact.metadata.source_package.as_str(),
            artifact.mir.module_name.as_str(),
            format!(
                "invalid {:?} kernel region rooted at {} with {} operations",
                region.kind, region.root, region.operation_count
            ),
        ));
    }
    for child in &region.children {
        validate_region(artifact, child)?;
    }
    Ok(())
}

pub(super) fn scheduled_opt_operations(artifact: &CanonicalIrArtifact) -> usize {
    artifact
        .opt
        .schedules
        .iter()
        .map(|schedule| {
            schedule
                .ops
                .iter()
                .map(|op| match op {
                    OptOp::ComputeValue { .. } | OptOp::EvaluateEquation { .. } => 1usize,
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_models_prefer_direct_scalar_kernels() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
            )
            .expect("canonical IR");

        let plan = KernelPlan::analyze(&artifact).expect("kernel plan");

        assert_eq!(plan.preferred_tier(), PreferredKernelTier::DirectScalar);
        assert!(!plan.equation_regions.is_empty());
        assert_eq!(plan.metrics.derivative_lanes, 2);
        assert!(scheduled_opt_operations(&artifact) > 0);
    }

    #[test]
    fn structured_regions_preserve_runtime_loops() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module loop_model(p, n);
    inout p, n;
    electrical p, n;
    integer i;
    real sum;
    analog begin
        i = 0;
        sum = 0.0;
        while (i < 100) begin
            sum = sum + V(p, n) * 0.01;
            i = i + 1;
        end
        I(p, n) <+ sum;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let plan = KernelPlan::analyze(&artifact).expect("kernel plan");

        assert!(
            plan.statement_regions
                .iter()
                .any(|region| region.kind == KernelRegionKind::Loop && !region.children.is_empty())
        );
        assert!(plan.metrics.structured_control_regions > 0);
        assert!(plan.summary(&artifact).statement_regions >= 3);
    }

    #[test]
    fn cost_model_moves_large_scalar_expansions_to_structured_kernels() {
        let metrics = KernelMetrics {
            scalar_values: MAX_DIRECT_SCALAR_VALUES + 1,
            scalar_derivative_entries: MAX_DIRECT_SCALAR_OPTIMIZER_NODES,
            scalar_optimizer_nodes: MAX_DIRECT_SCALAR_OPTIMIZER_NODES + 1,
            structured_expressions: 1_000,
            structured_operations: 2_000,
            structured_control_regions: 10,
            runtime_loop_operations: 0,
            derivative_lanes: 32,
            maximum_value_derivative_lanes: 32,
            scalar_expansion_ratio: MIN_STRUCTURED_EXPANSION_RATIO,
        };

        assert_eq!(metrics.preferred_tier(), PreferredKernelTier::Structured);
        assert_eq!(
            metrics.derivative_storage(),
            DerivativeStorageStrategy::Sparse
        );
    }

    #[test]
    fn cost_model_keeps_large_low_expansion_graphs_in_sparse_local_kernels() {
        let metrics = KernelMetrics {
            scalar_values: MAX_DIRECT_SCALAR_VALUES + 1,
            scalar_derivative_entries: 512,
            scalar_optimizer_nodes: MAX_DIRECT_SCALAR_OPTIMIZER_NODES + 1,
            structured_expressions: 16_000,
            structured_operations: 16_000,
            structured_control_regions: 10,
            runtime_loop_operations: 0,
            derivative_lanes: 32,
            maximum_value_derivative_lanes: 8,
            scalar_expansion_ratio: MIN_STRUCTURED_EXPANSION_RATIO - 1,
        };

        assert_eq!(metrics.preferred_tier(), PreferredKernelTier::SparseLocal);
        assert_eq!(
            metrics.derivative_storage(),
            DerivativeStorageStrategy::Sparse
        );
    }

    #[test]
    fn cost_model_does_not_round_subthreshold_expansion_up_to_structured() {
        let structured_operations = 75_626;
        let scalar_optimizer_nodes = 222_664;
        let metrics = KernelMetrics {
            scalar_values: scalar_optimizer_nodes - 518,
            scalar_derivative_entries: 518,
            scalar_optimizer_nodes,
            structured_expressions: 83_853,
            structured_operations,
            structured_control_regions: 4_732,
            runtime_loop_operations: 4,
            derivative_lanes: 19,
            maximum_value_derivative_lanes: 19,
            scalar_expansion_ratio: scalar_expansion_ratio(
                scalar_optimizer_nodes,
                structured_operations,
            ),
        };

        assert_eq!(metrics.scalar_expansion_ratio, 2);
        assert_eq!(metrics.preferred_tier(), PreferredKernelTier::SparseLocal);
    }

    #[test]
    fn expansion_ratio_observes_exact_threshold_boundaries() {
        assert_eq!(scalar_expansion_ratio(299, 100), 2);
        assert_eq!(scalar_expansion_ratio(300, 100), 3);
        assert_eq!(scalar_expansion_ratio(3, 1), 3);
        assert_eq!(scalar_expansion_ratio(3, 0), 3);
    }
}
