use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::fmt::Write;

use super::{
    CanonicalMetadata, CanonicalValueType, CompilerPhase, HirArray, HirAssignment, HirBranch,
    HirContribution, HirContributionKind, HirExprKind, HirExprRef, HirExpression, HirInternalNode,
    HirLaplaceKind, HirLoop, HirModel, HirParamRange, HirParameter, HirPort, HirStatement,
    HirVariable, HirZiKind, InvalidationClass, IrDiagnostic, MirAnalysisDomain, MirBranch,
    MirBranchRef, MirEquation, MirEquationKind, MirModel, MirNode, MirParameterSlot, MirStateSlot,
    OptModel, OptOp, OptSchedule, OptValue, OptValueType, SourceSpanRef, StableDigest,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalIrArtifact {
    pub metadata: CanonicalMetadata,
    pub hir_digest: SmolStr,
    pub mir_digest: SmolStr,
    pub opt_digest: SmolStr,
    pub hir: HirModel,
    pub mir: MirModel,
    pub opt: OptModel,
}

impl CanonicalIrArtifact {
    pub fn from_parts(
        metadata: CanonicalMetadata,
        hir: HirModel,
        mir: MirModel,
        opt: OptModel,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        hir.validate()?;
        mir.validate()?;
        opt.validate()?;

        let diagnostics = artifact_diagnostics(&metadata, &hir, &mir, &opt);
        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        let hir_digest = digest_text(&hir_summary(&hir));
        let mir_digest = digest_text(&mir_summary(&mir));
        let opt_digest = digest_text(&opt_summary(&opt));

        Ok(Self {
            metadata,
            hir_digest,
            mir_digest,
            opt_digest,
            hir,
            mir,
            opt,
        })
    }

    pub fn dump_text(&self) -> String {
        let mut out = String::new();
        writeln!(out, "canonical-veriloga-ir").expect("write to string");
        writeln!(out, "schema_version={}", self.metadata.schema_version).expect("write to string");
        writeln!(out, "source_package={}", self.metadata.source_package).expect("write to string");
        writeln!(out, "source_digest={}", self.metadata.source_digest).expect("write to string");
        writeln!(out, "compiler_version={}", self.metadata.compiler_version)
            .expect("write to string");
        writeln!(out, "hir_digest={}", self.hir_digest).expect("write to string");
        writeln!(out, "mir_digest={}", self.mir_digest).expect("write to string");
        writeln!(out, "opt_digest={}", self.opt_digest).expect("write to string");
        writeln!(
            out,
            "hir module={} ports={} parameters={} contributions={}",
            self.hir.module_name,
            self.hir.ports.len(),
            self.hir.parameters.len(),
            self.hir.contributions.len()
        )
        .expect("write to string");
        writeln!(
            out,
            "mir nodes={} equations={}",
            self.mir.nodes.len(),
            self.mir.equations.len()
        )
        .expect("write to string");
        writeln!(
            out,
            "opt schedules={} values={} equation_count={}",
            self.opt.schedules.len(),
            self.opt.values.len(),
            self.opt.equation_count
        )
        .expect("write to string");
        out
    }
}

fn artifact_diagnostics(
    metadata: &CanonicalMetadata,
    hir: &HirModel,
    mir: &MirModel,
    opt: &OptModel,
) -> Vec<IrDiagnostic> {
    let mut diagnostics = Vec::new();

    if metadata.schema_version != hir.schema_version {
        diagnostics.push(artifact_error(format!(
            "metadata schema_version {} must match HIR schema_version {}",
            metadata.schema_version, hir.schema_version
        )));
    }

    if metadata.source_package != hir.source_package {
        diagnostics.push(artifact_error(format!(
            "metadata source_package '{}' must match HIR source_package '{}'",
            metadata.source_package, hir.source_package
        )));
    }

    if metadata.source_digest != hir.source_digest {
        diagnostics.push(artifact_error(format!(
            "metadata source_digest '{}' must match HIR source_digest '{}'",
            metadata.source_digest, hir.source_digest
        )));
    }

    if metadata.compiler_version != hir.compiler_version {
        diagnostics.push(artifact_error(format!(
            "metadata compiler_version '{}' must match HIR compiler_version '{}'",
            metadata.compiler_version, hir.compiler_version
        )));
    }

    if hir.module_name != mir.module_name || hir.module_name != opt.module_name {
        diagnostics.push(artifact_error(format!(
            "HIR/MIR/OptIR module names must match: hir='{}' mir='{}' opt='{}'",
            hir.module_name, mir.module_name, opt.module_name
        )));
    }

    let mir_equation_count =
        u32::try_from(mir.equations.len()).expect("MIR equation count exceeds u32::MAX");
    if opt.equation_count != mir_equation_count {
        diagnostics.push(artifact_error(format!(
            "OptIR equation count {} must match MIR equation count {}",
            opt.equation_count, mir_equation_count
        )));
    }

    diagnostics
}

fn artifact_error(message: impl Into<String>) -> IrDiagnostic {
    IrDiagnostic::global_error(CompilerPhase::Artifact, message)
}

fn digest_text(text: &str) -> SmolStr {
    StableDigest::from_text(text).as_hex().into()
}

fn hir_summary(hir: &HirModel) -> String {
    let mut out = String::new();
    writeln!(out, "hir").expect("write to string");
    writeln!(out, "module_id={}", hir.module_id.index()).expect("write to string");
    writeln!(out, "module_name={}", hir.module_name).expect("write to string");
    writeln!(out, "schema_version={}", hir.schema_version).expect("write to string");
    writeln!(out, "source_package={}", hir.source_package).expect("write to string");
    writeln!(out, "source_digest={}", hir.source_digest).expect("write to string");
    writeln!(out, "compiler_version={}", hir.compiler_version).expect("write to string");
    writeln!(out, "feature_flags={}", join_smol(&hir.feature_flags)).expect("write to string");

    for port in &hir.ports {
        write_hir_port(&mut out, port);
    }
    for parameter in &hir.parameters {
        write_hir_parameter(&mut out, parameter);
    }
    for variable in &hir.variables {
        write_hir_variable(&mut out, variable);
    }
    for array in &hir.arrays {
        write_hir_array(&mut out, array);
    }
    for branch in &hir.branches {
        write_hir_branch(&mut out, branch);
    }
    for node in &hir.internal_nodes {
        write_hir_internal_node(&mut out, node);
    }
    writeln!(out, "ground_nodes={}", join_smol(&hir.ground_nodes)).expect("write to string");
    for expression in &hir.expressions {
        write_hir_expression(&mut out, expression);
    }
    for contribution in &hir.contributions {
        write_hir_contribution(&mut out, contribution);
    }
    write_hir_statements(&mut out, "statement", &hir.statements);
    out
}

fn mir_summary(mir: &MirModel) -> String {
    let mut out = String::new();
    writeln!(out, "mir").expect("write to string");
    writeln!(out, "module_name={}", mir.module_name).expect("write to string");
    for node in &mir.nodes {
        write_mir_node(&mut out, node);
    }
    for parameter in &mir.parameters {
        write_mir_parameter(&mut out, parameter);
    }
    for branch in &mir.branches {
        write_mir_branch(&mut out, branch);
    }
    for state_slot in &mir.state_slots {
        write_mir_state_slot(&mut out, state_slot);
    }
    writeln!(out, "ground_nodes={}", join_smol(&mir.ground_nodes)).expect("write to string");
    for expression in &mir.expressions {
        write_hir_expression(&mut out, expression);
    }
    for equation in &mir.equations {
        write_mir_equation(&mut out, equation);
    }
    out
}

fn opt_summary(opt: &OptModel) -> String {
    let mut out = String::new();
    writeln!(out, "opt").expect("write to string");
    writeln!(out, "module_name={}", opt.module_name).expect("write to string");
    writeln!(out, "equation_count={}", opt.equation_count).expect("write to string");
    for value in &opt.values {
        write_opt_value(&mut out, value);
    }
    for schedule in &opt.schedules {
        write_opt_schedule(&mut out, schedule);
    }
    out
}

fn write_hir_port(out: &mut String, port: &HirPort) {
    writeln!(
        out,
        "port id={} name={} direction={} discipline={} nature_potential={} nature_flow={}",
        port.id.index(),
        port.name,
        port.direction,
        port.discipline,
        option_smol(port.nature_potential.as_ref()),
        option_smol(port.nature_flow.as_ref())
    )
    .expect("write to string");
}

fn write_hir_parameter(out: &mut String, parameter: &HirParameter) {
    writeln!(
        out,
        "parameter id={} name={} type={} default={} default_expr={} range={} aliases={}",
        parameter.id.index(),
        parameter.name,
        value_type_label(parameter.value_type),
        option_f64(parameter.default),
        expr_ref_label(parameter.default_expr.as_ref()),
        range_label(parameter.range.as_ref()),
        join_smol(&parameter.aliases)
    )
    .expect("write to string");
}

fn write_hir_variable(out: &mut String, variable: &HirVariable) {
    writeln!(
        out,
        "variable id={} name={} type={} is_state={}",
        variable.id.index(),
        variable.name,
        value_type_label(variable.value_type),
        variable.is_state
    )
    .expect("write to string");
}

fn write_hir_array(out: &mut String, array: &HirArray) {
    writeln!(
        out,
        "array id={} name={} base={} lower={} len={}",
        array.id.index(),
        array.name,
        array.base.index(),
        array.lower,
        array.len
    )
    .expect("write to string");
}

fn write_hir_branch(out: &mut String, branch: &HirBranch) {
    writeln!(
        out,
        "branch id={} name={} pos={} neg={} discipline={}",
        branch.id.index(),
        branch.name,
        branch.pos_node,
        branch.neg_node,
        branch.discipline
    )
    .expect("write to string");
}

fn write_hir_internal_node(out: &mut String, node: &HirInternalNode) {
    writeln!(
        out,
        "internal_node id={} name={} discipline={} index={}",
        node.id.index(),
        node.name,
        node.discipline,
        node.index
    )
    .expect("write to string");
}

fn write_hir_contribution(out: &mut String, contribution: &HirContribution) {
    writeln!(
        out,
        "contribution id={} branch={} kind={} expression={} expr_type={} span={}",
        contribution.id.index(),
        contribution.branch,
        contribution_kind_label(contribution.kind),
        expr_ref_label(Some(&contribution.expression)),
        value_type_label(contribution.expr_type),
        span_label(contribution.span)
    )
    .expect("write to string");
}

fn write_hir_expression(out: &mut String, expression: &HirExpression) {
    writeln!(
        out,
        "expression id={} kind={} span={}",
        expression.id.index(),
        hir_expr_kind_label(&expression.kind),
        span_label(expression.span)
    )
    .expect("write to string");
}

fn write_hir_statements(out: &mut String, prefix: &str, statements: &[HirStatement]) {
    for (index, statement) in statements.iter().enumerate() {
        match statement {
            HirStatement::Assignment(assignment) => {
                write_hir_assignment(out, &format!("{prefix}[{index}]"), assignment);
            }
            HirStatement::Loop(loop_statement) => {
                write_hir_loop(out, &format!("{prefix}[{index}]"), loop_statement);
            }
        }
    }
}

fn write_hir_assignment(out: &mut String, label: &str, assignment: &HirAssignment) {
    writeln!(
        out,
        "{}=assignment target={} target_name={} index={} expr={} expr_type={} span={}",
        label,
        assignment.target.index(),
        assignment.target_name,
        expr_ref_label(assignment.index.as_ref()),
        expr_ref_label(Some(&assignment.expr)),
        value_type_label(assignment.expr_type),
        span_label(assignment.span)
    )
    .expect("write to string");
}

fn write_hir_loop(out: &mut String, label: &str, loop_statement: &HirLoop) {
    writeln!(
        out,
        "{}=loop condition={} span={} body_len={}",
        label,
        expr_ref_label(Some(&loop_statement.condition)),
        span_label(loop_statement.span),
        loop_statement.body.len()
    )
    .expect("write to string");
    write_hir_statements(out, &format!("{label}.body"), &loop_statement.body);
}

fn write_mir_node(out: &mut String, node: &MirNode) {
    writeln!(
        out,
        "node id={} name={} is_external={}",
        node.id.index(),
        node.name,
        node.is_external
    )
    .expect("write to string");
}

fn write_mir_parameter(out: &mut String, parameter: &MirParameterSlot) {
    writeln!(
        out,
        "parameter id={} name={} type={} default={} default_expr={} range={} aliases={}",
        parameter.id.index(),
        parameter.name,
        value_type_label(parameter.value_type),
        option_f64(parameter.default),
        expr_ref_label(parameter.default_expr.as_ref()),
        range_label(parameter.range.as_ref()),
        join_smol(&parameter.aliases)
    )
    .expect("write to string");
}

fn write_mir_branch(out: &mut String, branch: &MirBranch) {
    writeln!(
        out,
        "branch id={} name={} pos={} neg={} discipline={}",
        branch.id.index(),
        branch.name,
        option_id(branch.pos_node.map(|id| id.index())),
        option_id(branch.neg_node.map(|id| id.index())),
        branch.discipline
    )
    .expect("write to string");
}

fn write_mir_state_slot(out: &mut String, state_slot: &MirStateSlot) {
    writeln!(
        out,
        "state_slot id={} name={} owner={}",
        state_slot.id.index(),
        state_slot.name,
        state_slot.owner.index()
    )
    .expect("write to string");
}

fn write_mir_equation(out: &mut String, equation: &MirEquation) {
    writeln!(
        out,
        "equation id={} contribution={} branch={} kind={} expression={} domains={} span={}",
        equation.id.index(),
        equation.contribution.index(),
        branch_ref_label(&equation.branch),
        equation_kind_label(equation.kind),
        expr_ref_label(Some(&equation.expression)),
        join_domains(&equation.active_domains),
        span_label(equation.span)
    )
    .expect("write to string");
}

fn write_opt_value(out: &mut String, value: &OptValue) {
    writeln!(
        out,
        "value id={} type={}",
        value.id.index(),
        opt_value_type_label(value.value_type)
    )
    .expect("write to string");
}

fn write_opt_schedule(out: &mut String, schedule: &OptSchedule) {
    writeln!(
        out,
        "schedule id={} invalidation={} ops={}",
        schedule.id.index(),
        invalidation_label(schedule.invalidation),
        schedule
            .ops
            .iter()
            .map(opt_op_label)
            .collect::<Vec<_>>()
            .join(",")
    )
    .expect("write to string");
}

fn branch_ref_label(branch: &MirBranchRef) -> String {
    format!(
        "label:{} pos:{} neg:{}",
        branch.label,
        option_id(branch.pos_node.map(|id| id.index())),
        option_id(branch.neg_node.map(|id| id.index()))
    )
}

fn expr_ref_label(expr: Option<&HirExprRef>) -> String {
    expr.map(|expr| {
        format!(
            "id:{} kind:{} span:{}",
            expr.id.index(),
            expr.kind,
            span_label(expr.span)
        )
    })
    .unwrap_or_else(|| "-".to_string())
}

fn hir_expr_kind_label(kind: &HirExprKind) -> String {
    match kind {
        HirExprKind::Number { value, raw } => {
            format!("number value:{} raw:{}", f64_label(*value), raw)
        }
        HirExprKind::StringLiteral { value } => format!("string value:{}", value),
        HirExprKind::Identifier { name } => format!("identifier name:{}", name),
        HirExprKind::SystemFunction { name, args } => {
            format!("system_function name:{} args:{}", name, join_expr_ids(args))
        }
        HirExprKind::Binary { op, left, right } => {
            format!(
                "binary op:{} left:{} right:{}",
                op,
                left.index(),
                right.index()
            )
        }
        HirExprKind::Unary { op, operand } => {
            format!("unary op:{} operand:{}", op, operand.index())
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => format!(
            "conditional condition:{} then:{} else:{}",
            condition.index(),
            then_expr.index(),
            else_expr.index()
        ),
        HirExprKind::Call { name, args } => {
            format!("call name:{} args:{}", name, join_expr_ids(args))
        }
        HirExprKind::BranchAccess { access, pos, neg } => {
            format!(
                "branch_access access:{} pos:{} neg:{}",
                access,
                pos,
                option_smol(neg.as_ref())
            )
        }
        HirExprKind::NamedBranchAccess { access, name } => {
            format!("named_branch_access access:{} name:{}", access, name)
        }
        HirExprKind::ArrayAccess { array, index } => {
            format!("array_access array:{} index:{}", array, index.index())
        }
        HirExprKind::ArrayLiteral { elements } => {
            format!("array_literal elements:{}", join_expr_ids(elements))
        }
        HirExprKind::AnalogOperator { operator, operands } => {
            format!(
                "analog_operator operator:{} operands:{}",
                operator,
                join_expr_ids(operands)
            )
        }
        HirExprKind::Laplace { expr, kind } => {
            format!("laplace expr:{} {}", expr.index(), laplace_kind_label(kind))
        }
        HirExprKind::Zi { expr, kind } => {
            format!("zi expr:{} {}", expr.index(), zi_kind_label(kind))
        }
        HirExprKind::NoiseSource {
            source,
            operands,
            name,
        } => format!(
            "noise_source source:{} operands:{} name:{}",
            source,
            join_expr_ids(operands),
            option_smol(name.as_ref())
        ),
    }
}

fn laplace_kind_label(kind: &HirLaplaceKind) -> String {
    match kind {
        HirLaplaceKind::ZeroPole { zeros, poles } => {
            format!(
                "zero_pole zeros:{} poles:{}",
                join_expr_ids(zeros),
                join_expr_ids(poles)
            )
        }
        HirLaplaceKind::ZeroDenominator { zeros, denominator } => format!(
            "zero_denominator zeros:{} denominator:{}",
            join_expr_ids(zeros),
            join_expr_ids(denominator)
        ),
        HirLaplaceKind::NumeratorPole { numerator, poles } => format!(
            "numerator_pole numerator:{} poles:{}",
            join_expr_ids(numerator),
            join_expr_ids(poles)
        ),
        HirLaplaceKind::NumeratorDenominator {
            numerator,
            denominator,
        } => format!(
            "numerator_denominator numerator:{} denominator:{}",
            join_expr_ids(numerator),
            join_expr_ids(denominator)
        ),
    }
}

fn zi_kind_label(kind: &HirZiKind) -> String {
    match kind {
        HirZiKind::ZeroPole { zeros, poles } => {
            format!(
                "zero_pole zeros:{} poles:{}",
                join_expr_ids(zeros),
                join_expr_ids(poles)
            )
        }
        HirZiKind::ZeroDenominator { zeros, denominator } => format!(
            "zero_denominator zeros:{} denominator:{}",
            join_expr_ids(zeros),
            join_expr_ids(denominator)
        ),
        HirZiKind::NumeratorPole { numerator, poles } => format!(
            "numerator_pole numerator:{} poles:{}",
            join_expr_ids(numerator),
            join_expr_ids(poles)
        ),
        HirZiKind::NumeratorDenominator {
            numerator,
            denominator,
        } => format!(
            "numerator_denominator numerator:{} denominator:{}",
            join_expr_ids(numerator),
            join_expr_ids(denominator)
        ),
    }
}

fn range_label(range: Option<&HirParamRange>) -> String {
    range
        .map(|range| {
            format!(
                "min:{} max:{} min_exclusive:{} max_exclusive:{} exclude:{}",
                option_f64(range.min),
                option_f64(range.max),
                range.min_exclusive,
                range.max_exclusive,
                range
                    .exclude
                    .iter()
                    .map(|value| f64_label(*value))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .unwrap_or_else(|| "-".to_string())
}

fn span_label(span: SourceSpanRef) -> String {
    format!("{}:{}-{}", span.source_file_id, span.start, span.end)
}

fn join_smol(values: &[SmolStr]) -> String {
    values
        .iter()
        .map(|value| value.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn join_expr_ids(values: &[super::ExprId]) -> String {
    values
        .iter()
        .map(|value| value.index().to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn join_domains(values: &[MirAnalysisDomain]) -> String {
    values
        .iter()
        .map(|value| analysis_domain_label(*value))
        .collect::<Vec<_>>()
        .join(",")
}

fn option_smol(value: Option<&SmolStr>) -> &str {
    value.map(|value| value.as_str()).unwrap_or("-")
}

fn option_id(value: Option<u32>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn option_f64(value: Option<f64>) -> String {
    value.map(f64_label).unwrap_or_else(|| "-".to_string())
}

fn f64_label(value: f64) -> String {
    if value.is_nan() {
        "NaN".to_string()
    } else if value == f64::INFINITY {
        "inf".to_string()
    } else if value == f64::NEG_INFINITY {
        "-inf".to_string()
    } else {
        value.to_string()
    }
}

fn value_type_label(value_type: CanonicalValueType) -> &'static str {
    match value_type {
        CanonicalValueType::Real => "real",
        CanonicalValueType::Integer => "integer",
        CanonicalValueType::String => "string",
        CanonicalValueType::Boolean => "boolean",
        CanonicalValueType::NatureAccess => "nature_access",
        CanonicalValueType::Void => "void",
        CanonicalValueType::Unknown => "unknown",
        CanonicalValueType::Error => "error",
    }
}

fn contribution_kind_label(kind: HirContributionKind) -> &'static str {
    match kind {
        HirContributionKind::Current => "current",
        HirContributionKind::Potential => "potential",
        HirContributionKind::Indirect => "indirect",
    }
}

fn analysis_domain_label(domain: MirAnalysisDomain) -> &'static str {
    match domain {
        MirAnalysisDomain::Dc => "dc",
        MirAnalysisDomain::Ac => "ac",
        MirAnalysisDomain::Transient => "transient",
        MirAnalysisDomain::Noise => "noise",
        MirAnalysisDomain::OperatingPoint => "operating_point",
    }
}

fn equation_kind_label(kind: MirEquationKind) -> &'static str {
    match kind {
        MirEquationKind::Current => "current",
        MirEquationKind::Potential => "potential",
        MirEquationKind::Indirect => "indirect",
    }
}

fn opt_value_type_label(value_type: OptValueType) -> &'static str {
    match value_type {
        OptValueType::Real => "real",
        OptValueType::Boolean => "boolean",
    }
}

fn invalidation_label(invalidation: InvalidationClass) -> &'static str {
    match invalidation {
        InvalidationClass::InstanceStatic => "instance_static",
        InvalidationClass::TemperatureStatic => "temperature_static",
        InvalidationClass::TimestepStatic => "timestep_static",
        InvalidationClass::OperatingPointStatic => "operating_point_static",
        InvalidationClass::NewtonIteration => "newton_iteration",
        InvalidationClass::AcFrequency => "ac_frequency",
        InvalidationClass::NoiseFrequency => "noise_frequency",
        InvalidationClass::OperatingPointReport => "operating_point_report",
    }
}

fn opt_op_label(op: &OptOp) -> String {
    match op {
        OptOp::EvaluateEquation { equation } => {
            format!("evaluate_equation:{}", equation.index())
        }
    }
}
