use std::collections::HashMap;

use crate::canonical_ir::{
    CanonicalIrArtifact, CanonicalValueType, DerivativeLaneKind, EquationId, HirStatement,
    InvalidationClass, MirEquation, MirEquationKind, OptBinaryOp, OptOp, OptUnaryOp, OptValue,
    OptValueKind, OptValueType, ValueId,
};

use super::expr::parameter_field_names;
use super::{GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames};
use super::{RustTranspileOptions, device};

pub fn generate_device(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
) -> Result<GeneratedRustDevice, RustBackendError> {
    reject_unsupported_scalar_shape(artifact)?;
    artifact.opt.validate().map_err(|diagnostics| {
        internal(artifact, format!("invalid scalar OptIR: {diagnostics:?}"))
    })?;

    let source_file_name = artifact.metadata.source_package.as_str();
    let names = RustDeviceNames::new(
        source_file_name,
        artifact.mir.module_name.as_str(),
        artifact.metadata.source_digest.as_str(),
    );
    let parameter_fields = parameter_field_names(artifact);
    let stamp = generate_stamp_file(artifact, options, &parameter_fields)?;
    let files = vec![
        GeneratedRustFile {
            relative_path: "mod.rs".to_string(),
            contents: device::generate_mod_file(),
        },
        GeneratedRustFile {
            relative_path: "state.rs".to_string(),
            contents: device::generate_state_file(artifact, options, &parameter_fields, 0, 0, 0)?,
        },
        GeneratedRustFile {
            relative_path: "stamp.rs".to_string(),
            contents: stamp,
        },
    ];

    Ok(GeneratedRustDevice {
        module_name: artifact.mir.module_name.to_string(),
        public_model_name: names.public_model_name,
        folder_name: names.folder,
        source_digest: artifact.metadata.source_digest.to_string(),
        files,
    })
}

fn generate_stamp_file(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    let roots = scalar_equation_roots(artifact)?;
    let mut out = String::new();
    out.push_str("#![allow(dead_code, unused_imports, unused_parens, unused_variables)]\n\n");
    out.push_str("use super::state::Instance;\n");
    out.push_str(&format!(
        "use {}::{{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};\n\n",
        options.runtime_path
    ));
    out.push_str("impl Instance {\n");
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
    );
    out.push_str("        let nodes = self.nodes;\n");
    out.push_str("        let p = &(*self.params);\n");
    out.push_str("        let multiplicity = self.multiplicity;\n");

    for value in &artifact.opt.values {
        let expr = emit_value_expr(artifact, parameter_fields, value)?;
        out.push_str(&format!(
            "        let {}: {} = {};\n",
            value_name(value.id),
            rust_type(value.value_type),
            expr
        ));
    }
    if !artifact.opt.values.is_empty() {
        out.push('\n');
    }

    for equation in &artifact.mir.equations {
        let root = roots.get(&equation.id).copied().ok_or_else(|| {
            unsupported(
                artifact,
                format!("missing scalar value for {}", equation.id),
            )
        })?;
        emit_current_stamp(artifact, equation, root, &mut out)?;
    }

    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn stamp_reactive(&mut self, _ctx: &GeneratedEvalContext<'_>, _stamper: &mut GeneratedReactiveStamper<'_>) {\n",
    );
    out.push_str("    }\n");
    out.push_str("}\n");
    Ok(out)
}

fn emit_current_stamp(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = root_value
        .derivatives
        .iter()
        .map(|derivative| {
            if derivative.lane.kind != DerivativeLaneKind::Node {
                return Err(unsupported(
                    artifact,
                    format!("branch derivative lane on scalar equation {}", equation.id),
                ));
            }
            Ok((derivative.lane.index, derivative.value))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for (node, value) in &derivatives {
        out.push_str(&format!(
            "        let {}: f64 = {};\n",
            derivative_name(root, *node),
            value_name(*value)
        ));
    }

    let pos = optional_node_local_expr(equation.branch.pos_node);
    let neg = optional_node_local_expr(equation.branch.neg_node);
    let root_name = value_name(root);
    match derivatives.as_slice() {
        [] => {
            out.push_str("        stamper.stamp_current_const_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_name}),\n"));
            out.push_str("        );\n");
        }
        [(node0, _)] => {
            out.push_str("        stamper.stamp_current_node1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_name}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node0)
            ));
            out.push_str("        );\n");
        }
        [(node0, _), (node1, _)] => {
            out.push_str("        stamper.stamp_current_node2_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_name}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node0)
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node1)
            ));
            out.push_str("        );\n");
        }
        [(node0, _), (node1, _), (node2, _)] => {
            out.push_str("        stamper.stamp_current_node3_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_name}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node0)
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node1)
            ));
            out.push_str(&format!("            {node2},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node2)
            ));
            out.push_str("        );\n");
        }
        _ => {
            return Err(unsupported(
                artifact,
                format!(
                    "scalar current equation {} with {} node derivative lanes",
                    equation.id,
                    derivatives.len()
                ),
            ));
        }
    }
    Ok(())
}

fn emit_value_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    value: &OptValue,
) -> Result<String, RustBackendError> {
    let expr = match &value.kind {
        OptValueKind::RealConstant(value) => format_f64(*value),
        OptValueKind::BooleanConstant(value) => value.to_string(),
        OptValueKind::Parameter { parameter } => {
            let parameter = artifact
                .mir
                .parameters
                .get(usize::from(*parameter))
                .ok_or_else(|| unsupported(artifact, format!("missing parameter {parameter}")))?;
            let field = parameter_fields
                .get(parameter.name.as_str())
                .ok_or_else(|| {
                    unsupported(
                        artifact,
                        format!("missing parameter field '{}'", parameter.name),
                    )
                })?;
            format!("p.{field}")
        }
        OptValueKind::NodePotential { node } => {
            format!("ctx.node_voltage(nodes[{}])", node.index())
        }
        OptValueKind::BranchFlow { .. } => {
            return Err(unsupported(
                artifact,
                "branch current probes in scalar backend",
            ));
        }
        OptValueKind::Unary { op, input } => emit_unary_expr(*op, *input),
        OptValueKind::Binary { op, left, right } => emit_binary_expr(*op, *left, *right),
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => format!(
            "(if {} {{ {} }} else {{ {} }})",
            value_name(*condition),
            value_name(*then_value),
            value_name(*else_value)
        ),
        OptValueKind::EquationValue { .. } => {
            return Err(unsupported(
                artifact,
                "legacy equation value in scalar backend",
            ));
        }
    };
    Ok(expr)
}

fn emit_unary_expr(op: OptUnaryOp, input: ValueId) -> String {
    let input = value_name(input);
    match op {
        OptUnaryOp::Pos => input,
        OptUnaryOp::Neg => format!("(-{input})"),
        OptUnaryOp::Not => format!("(!{input})"),
        OptUnaryOp::Exp => format!("{input}.exp()"),
        OptUnaryOp::Ln => format!("{input}.ln()"),
        OptUnaryOp::Sqrt => format!("{input}.sqrt()"),
        OptUnaryOp::Abs => format!("{input}.abs()"),
    }
}

fn emit_binary_expr(op: OptBinaryOp, left: ValueId, right: ValueId) -> String {
    let left = value_name(left);
    let right = value_name(right);
    match op {
        OptBinaryOp::Add => format!("({left} + {right})"),
        OptBinaryOp::Sub => format!("({left} - {right})"),
        OptBinaryOp::Mul => format!("({left} * {right})"),
        OptBinaryOp::Div => format!("({left} / {right})"),
        OptBinaryOp::Pow => format!("{left}.powf({right})"),
        OptBinaryOp::Eq => format!("({left} == {right})"),
        OptBinaryOp::Ne => format!("({left} != {right})"),
        OptBinaryOp::Lt => format!("({left} < {right})"),
        OptBinaryOp::Le => format!("({left} <= {right})"),
        OptBinaryOp::Gt => format!("({left} > {right})"),
        OptBinaryOp::Ge => format!("({left} >= {right})"),
        OptBinaryOp::And => format!("({left} && {right})"),
        OptBinaryOp::Or => format!("({left} || {right})"),
    }
}

fn scalar_equation_roots(
    artifact: &CanonicalIrArtifact,
) -> Result<HashMap<EquationId, ValueId>, RustBackendError> {
    let mut roots = HashMap::new();
    for schedule in &artifact.opt.schedules {
        if schedule.invalidation != InvalidationClass::NewtonIteration {
            continue;
        }

        let mut pending_value = None;
        for op in &schedule.ops {
            match *op {
                OptOp::ComputeValue { value } => pending_value = Some(value),
                OptOp::EvaluateEquation { equation } => {
                    let value = pending_value.take().ok_or_else(|| {
                        unsupported(artifact, format!("non-scalar equation {}", equation))
                    })?;
                    roots.insert(equation, value);
                }
            }
        }
    }

    for equation in &artifact.mir.equations {
        if !roots.contains_key(&equation.id) {
            return Err(unsupported(
                artifact,
                format!("missing scalar root for equation {}", equation.id),
            ));
        }
    }

    Ok(roots)
}

fn reject_unsupported_scalar_shape(artifact: &CanonicalIrArtifact) -> Result<(), RustBackendError> {
    if !artifact.hir.arrays.is_empty() {
        return Err(unsupported(artifact, "arrays"));
    }
    for variable in &artifact.hir.variables {
        if variable.is_state {
            return Err(unsupported(
                artifact,
                format!("state variable '{}'", variable.name),
            ));
        }
        if !supported_scalar_value_type(variable.value_type) {
            return Err(unsupported(
                artifact,
                format!(
                    "non-numeric scalar variable '{}' with type {:?}",
                    variable.name, variable.value_type
                ),
            ));
        }
    }
    for statement in &artifact.hir.statements {
        match statement {
            HirStatement::Assignment(assignment)
                if assignment.index.is_none()
                    && supported_scalar_value_type(assignment.expr_type) => {}
            HirStatement::Assignment(assignment) if assignment.index.is_some() => {
                return Err(unsupported(artifact, "indexed assignments"));
            }
            HirStatement::Assignment(assignment) => {
                return Err(unsupported(
                    artifact,
                    format!(
                        "assignment '{}' with type {:?}",
                        assignment.target_name, assignment.expr_type
                    ),
                ));
            }
            HirStatement::Loop(_) => return Err(unsupported(artifact, "analog loops")),
        }
    }
    if !artifact.mir.state_slots.is_empty() {
        return Err(unsupported(artifact, "state slots"));
    }
    if !artifact.mir.branches.is_empty() {
        return Err(unsupported(artifact, "declared branches"));
    }
    if !artifact.mir.branch_unknowns.is_empty() {
        return Err(unsupported(artifact, "branch unknowns"));
    }
    for equation in &artifact.mir.equations {
        if equation.kind != MirEquationKind::Current {
            return Err(unsupported(artifact, "non-current equations"));
        }
    }
    for value in &artifact.opt.values {
        if matches!(
            value.kind,
            OptValueKind::BranchFlow { .. } | OptValueKind::EquationValue { .. }
        ) {
            return Err(unsupported(
                artifact,
                "branch flows or legacy equation values in scalar OptIR",
            ));
        }
    }
    Ok(())
}

fn supported_scalar_value_type(value_type: CanonicalValueType) -> bool {
    matches!(
        value_type,
        CanonicalValueType::Real | CanonicalValueType::Integer | CanonicalValueType::Boolean
    )
}

fn rust_type(value_type: OptValueType) -> &'static str {
    match value_type {
        OptValueType::Real => "f64",
        OptValueType::Boolean => "bool",
    }
}

fn value_name(value: ValueId) -> String {
    format!("v{}", value.index())
}

fn derivative_name(root: ValueId, node: u32) -> String {
    format!("d{}_dn{node}", root.index())
}

fn optional_node_local_expr(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some({})", node.index()))
        .unwrap_or_else(|| "None".to_string())
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}

fn internal(artifact: &CanonicalIrArtifact, message: impl Into<String>) -> RustBackendError {
    RustBackendError::internal(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        message,
    )
}

fn format_f64(value: f64) -> String {
    if value.is_nan() {
        "f64::NAN".to_string()
    } else if value == f64::INFINITY {
        "f64::INFINITY".to_string()
    } else if value == f64::NEG_INFINITY {
        "f64::NEG_INFINITY".to_string()
    } else {
        format!("{value:?}")
    }
}
