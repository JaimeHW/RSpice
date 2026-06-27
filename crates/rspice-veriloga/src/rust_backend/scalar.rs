use std::collections::{HashMap, HashSet};

use crate::canonical_ir::{
    CanonicalIrArtifact, CanonicalValueType, DerivativeLaneKind, EquationId, HirStatement,
    InvalidationClass, MirEquation, MirEquationKind, OptBinaryOp, OptOp, OptUnaryOp, OptValue,
    OptValueKind, OptValueType, ValueId,
};

use super::expr::parameter_field_names;
use super::{GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames};
use super::{RustTranspileOptions, device};

pub(super) struct ScalarStaticCache {
    values: Vec<ValueId>,
    set: HashSet<ValueId>,
}

impl ScalarStaticCache {
    fn from_artifact(artifact: &CanonicalIrArtifact) -> Self {
        let values: Vec<_> = artifact
            .opt
            .schedules
            .iter()
            .filter(|schedule| schedule.invalidation == InvalidationClass::InstanceStatic)
            .flat_map(|schedule| schedule.ops.iter())
            .filter_map(|op| match op {
                OptOp::ComputeValue { value } => Some(*value),
                OptOp::EvaluateEquation { .. } => None,
            })
            .collect();
        let set = values.iter().copied().collect();
        Self { values, set }
    }

    pub(super) fn from_roots(
        artifact: &CanonicalIrArtifact,
        roots: &HashMap<EquationId, ValueId>,
    ) -> Result<Self, RustBackendError> {
        let empty_cache = Self {
            values: Vec::new(),
            set: HashSet::new(),
        };
        let live = collect_stamp_live_values(artifact, roots, &empty_cache)?;
        let values: Vec<_> = artifact
            .opt
            .schedules
            .iter()
            .filter(|schedule| schedule.invalidation == InvalidationClass::InstanceStatic)
            .flat_map(|schedule| schedule.ops.iter())
            .filter_map(|op| match op {
                OptOp::ComputeValue { value } if live.contains(value) => Some(*value),
                OptOp::ComputeValue { .. } | OptOp::EvaluateEquation { .. } => None,
            })
            .collect();
        let set = values.iter().copied().collect();
        Ok(Self { values, set })
    }

    fn contains(&self, value: ValueId) -> bool {
        self.set.contains(&value)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

struct ValueEmitContext<'a> {
    cached_values: &'a HashSet<ValueId>,
    use_cached_fields: bool,
    inline_uncached_constants: bool,
}

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
    let static_cache = ScalarStaticCache::from_artifact(artifact);
    let stamp = generate_stamp_file(artifact, options, &parameter_fields, &static_cache)?;
    let state = if static_cache.is_empty() {
        device::generate_state_file(artifact, options, &parameter_fields, 0, 0, 0)?
    } else {
        let extensions = scalar_state_extensions(artifact, &parameter_fields, &static_cache)?;
        device::generate_state_file_with_extensions(
            artifact,
            options,
            &parameter_fields,
            0,
            0,
            0,
            &extensions,
        )?
    };
    let files = vec![
        GeneratedRustFile {
            relative_path: "mod.rs".to_string(),
            contents: device::generate_mod_file(),
        },
        GeneratedRustFile {
            relative_path: "state.rs".to_string(),
            contents: state,
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
    static_cache: &ScalarStaticCache,
) -> Result<String, RustBackendError> {
    let roots = scalar_equation_roots(artifact)?;
    let stamp_live = collect_stamp_live_values(artifact, &roots, static_cache)?;
    let stamp_needs_params = artifact.opt.values.iter().any(|value| {
        stamp_live.contains(&value.id) && matches!(value.kind, OptValueKind::Parameter { .. })
    });
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
    if stamp_needs_params {
        out.push_str("        let p = &(*self.params);\n");
    }
    out.push_str("        let multiplicity = self.multiplicity;\n");

    let stamp_context = ValueEmitContext {
        cached_values: &static_cache.set,
        use_cached_fields: true,
        inline_uncached_constants: false,
    };
    for value in &artifact.opt.values {
        if !stamp_live.contains(&value.id) {
            continue;
        }
        let expr = emit_value_expr(artifact, parameter_fields, value, &stamp_context)?;
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

    emit_static_current_stamps(artifact, &roots, static_cache, &mut out)?;

    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn stamp_reactive(&mut self, _ctx: &GeneratedEvalContext<'_>, _stamper: &mut GeneratedReactiveStamper<'_>) {\n",
    );
    out.push_str("    }\n");
    out.push_str("}\n");
    Ok(out)
}

pub(super) fn scalar_state_extensions(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    static_cache: &ScalarStaticCache,
) -> Result<device::StateFileExtensions, RustBackendError> {
    let mut extensions = device::StateFileExtensions {
        params_visibility: "pub(crate)",
        ..device::StateFileExtensions::default()
    };
    let recompute_context = ValueEmitContext {
        cached_values: &static_cache.set,
        use_cached_fields: false,
        inline_uncached_constants: true,
    };
    let mut recompute = String::new();
    recompute.push_str("\n    #[inline]\n");
    recompute.push_str("    fn recompute_instance_static(&mut self) {\n");
    recompute.push_str("        let p = &(*self.params);\n");

    for value_id in &static_cache.values {
        let value = artifact
            .opt
            .values
            .get(usize::from(*value_id))
            .ok_or_else(|| {
                unsupported(artifact, format!("missing static scalar value {value_id}"))
            })?;
        let field = cache_field_name(*value_id);
        let ty = rust_type(value.value_type);
        let default = default_value(value.value_type);
        extensions
            .instance_fields
            .push_str(&format!("    pub(crate) {field}: {ty},\n"));
        extensions
            .clone_fields
            .push_str(&format!("            {field}: self.{field},\n"));
        extensions
            .new_initializers
            .push_str(&format!("            {field}: {default},\n"));
        extensions
            .restore_destructure_fields
            .push_str(&format!("            {field},\n"));
        extensions
            .restore_initializers
            .push_str(&format!("            {field},\n"));

        let local = value_name(*value_id);
        let expr = emit_value_expr(artifact, parameter_fields, value, &recompute_context)?;
        recompute.push_str(&format!("        let {local}: {ty} = {expr};\n"));
        recompute.push_str(&format!("        self.{field} = {local};\n"));
    }

    recompute.push_str("    }\n");
    extensions.after_new = "        instance.recompute_instance_static();\n".to_string();
    extensions.set_parameter_hook = "self.recompute_instance_static(); ".to_string();
    extensions.impl_methods = recompute;
    Ok(extensions)
}

pub(super) fn scalarizable_current_equation_roots(
    artifact: &CanonicalIrArtifact,
) -> Result<HashMap<EquationId, ValueId>, RustBackendError> {
    let roots = available_scalar_equation_roots(artifact);
    let mut selected = HashMap::new();
    for equation in &artifact.mir.equations {
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        match validate_scalar_current_equation(artifact, equation, root) {
            Ok(()) => {
                selected.insert(equation.id, root);
            }
            Err(error) if error.is_unsupported() => {}
            Err(error) => return Err(error),
        }
    }
    Ok(selected)
}

pub(super) fn emit_static_current_values(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let stamp_live = collect_stamp_live_values(artifact, roots, static_cache)?;
    let stamp_context = ValueEmitContext {
        cached_values: &static_cache.set,
        use_cached_fields: true,
        inline_uncached_constants: false,
    };
    for value in &artifact.opt.values {
        if !stamp_live.contains(&value.id) {
            continue;
        }
        let expr = emit_value_expr(artifact, parameter_fields, value, &stamp_context)?;
        out.push_str(&format!(
            "        let {}: {} = {};\n",
            value_name(value.id),
            rust_type(value.value_type),
            expr
        ));
    }
    if !stamp_live.is_empty() {
        out.push('\n');
    }
    Ok(())
}

pub(super) fn emit_static_current_stamps(
    artifact: &CanonicalIrArtifact,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    out: &mut String,
) -> Result<(), RustBackendError> {
    for equation in &artifact.mir.equations {
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        emit_current_stamp(artifact, equation, root, static_cache, out)?;
    }
    Ok(())
}

fn emit_current_stamp(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
    static_cache: &ScalarStaticCache,
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
            cached_or_local_value_name(*value, static_cache)
        ));
    }

    let pos = optional_node_local_expr(equation.branch.pos_node);
    let neg = optional_node_local_expr(equation.branch.neg_node);
    let root_name = cached_or_local_value_name(root, static_cache);
    let root_expr = current_root_expr(root_value.value_type, &root_name);
    match derivatives.as_slice() {
        [] => {
            out.push_str("        stamper.stamp_current_const_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str("        );\n");
        }
        [(node0, _)] => {
            out.push_str("        stamper.stamp_current_node1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
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
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
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
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
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

fn current_root_expr(value_type: OptValueType, root_name: &str) -> String {
    match value_type {
        OptValueType::Real => root_name.to_string(),
        OptValueType::Boolean => format!("if {root_name} {{ 1.0 }} else {{ 0.0 }}"),
    }
}

fn emit_value_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    value: &OptValue,
    context: &ValueEmitContext<'_>,
) -> Result<String, RustBackendError> {
    let expr = match &value.kind {
        OptValueKind::RealConstant(value) => format_f64(*value),
        OptValueKind::BooleanConstant(value) => value.to_string(),
        OptValueKind::Parameter { parameter } => {
            emit_parameter_expr(artifact, parameter_fields, *parameter)?
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
        OptValueKind::Unary { op, input } => {
            emit_unary_expr(*op, value_ref(artifact, parameter_fields, *input, context)?)
        }
        OptValueKind::Binary { op, left, right } => emit_binary_expr(
            *op,
            value_ref(artifact, parameter_fields, *left, context)?,
            value_ref(artifact, parameter_fields, *right, context)?,
        ),
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => format!(
            "(if {} {{ {} }} else {{ {} }})",
            value_ref(artifact, parameter_fields, *condition, context)?,
            value_ref(artifact, parameter_fields, *then_value, context)?,
            value_ref(artifact, parameter_fields, *else_value, context)?
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

fn emit_parameter_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    parameter: crate::canonical_ir::ParamId,
) -> Result<String, RustBackendError> {
    let parameter_slot = artifact
        .mir
        .parameters
        .get(usize::from(parameter))
        .ok_or_else(|| unsupported(artifact, format!("missing parameter {parameter}")))?;
    let field = parameter_fields
        .get(parameter_slot.name.as_str())
        .ok_or_else(|| {
            unsupported(
                artifact,
                format!("missing parameter field '{}'", parameter_slot.name),
            )
        })?;
    Ok(format!("p.{field}"))
}

fn value_ref(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    value: ValueId,
    context: &ValueEmitContext<'_>,
) -> Result<String, RustBackendError> {
    if context.use_cached_fields && context.cached_values.contains(&value) {
        return Ok(format!("self.{}", cache_field_name(value)));
    }

    if context.inline_uncached_constants {
        let value_slot = artifact
            .opt
            .values
            .get(usize::from(value))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
        match value_slot.kind {
            OptValueKind::RealConstant(value) => return Ok(format_f64(value)),
            OptValueKind::BooleanConstant(value) => return Ok(value.to_string()),
            OptValueKind::Parameter { parameter } => {
                return emit_parameter_expr(artifact, parameter_fields, parameter);
            }
            _ => {}
        }
    }

    Ok(value_name(value))
}

fn emit_unary_expr(op: OptUnaryOp, input: String) -> String {
    match op {
        OptUnaryOp::Pos => input,
        OptUnaryOp::Neg => format!("(-{input})"),
        OptUnaryOp::Not => format!("(!{input})"),
        OptUnaryOp::Exp => format!("{input}.exp()"),
        OptUnaryOp::Ln => format!("{input}.ln()"),
        OptUnaryOp::Sqrt => format!("{input}.sqrt()"),
        OptUnaryOp::Abs => format!("{input}.abs()"),
        OptUnaryOp::Sin => format!("{input}.sin()"),
        OptUnaryOp::Cos => format!("{input}.cos()"),
        OptUnaryOp::Tan => format!("{input}.tan()"),
        OptUnaryOp::Sinh => format!("{input}.sinh()"),
        OptUnaryOp::Cosh => format!("{input}.cosh()"),
        OptUnaryOp::Tanh => format!("{input}.tanh()"),
        OptUnaryOp::Atan => format!("{input}.atan()"),
        OptUnaryOp::Asinh => format!("{input}.asinh()"),
    }
}

fn emit_binary_expr(op: OptBinaryOp, left: String, right: String) -> String {
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
    let roots = available_scalar_equation_roots(artifact);
    for equation in &artifact.mir.equations {
        let root = roots.get(&equation.id).copied().ok_or_else(|| {
            unsupported(
                artifact,
                format!("missing scalar root for equation {}", equation.id),
            )
        })?;
        validate_scalar_current_equation(artifact, equation, root)?;
    }

    Ok(roots)
}

fn available_scalar_equation_roots(artifact: &CanonicalIrArtifact) -> HashMap<EquationId, ValueId> {
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
                    if let Some(value) = pending_value.take() {
                        roots.insert(equation, value);
                    }
                }
            }
        }
    }
    roots
}

fn validate_scalar_current_equation(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
) -> Result<(), RustBackendError> {
    if equation.kind != MirEquationKind::Current {
        return Err(unsupported(artifact, "non-current equations"));
    }
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    if root_value.derivatives.len() > 3 {
        return Err(unsupported(
            artifact,
            format!(
                "scalar current equation {} with {} node derivative lanes",
                equation.id,
                root_value.derivatives.len()
            ),
        ));
    }
    for derivative in &root_value.derivatives {
        if derivative.lane.kind != DerivativeLaneKind::Node {
            return Err(unsupported(
                artifact,
                format!("branch derivative lane on scalar equation {}", equation.id),
            ));
        }
    }
    let roots = HashMap::from([(equation.id, root)]);
    let empty_cache = ScalarStaticCache {
        values: Vec::new(),
        set: HashSet::new(),
    };
    let live = collect_stamp_live_values(artifact, &roots, &empty_cache)?;
    for value in live {
        let value_slot = artifact
            .opt
            .values
            .get(usize::from(value))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
        if matches!(
            value_slot.kind,
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

fn collect_stamp_live_values(
    artifact: &CanonicalIrArtifact,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
) -> Result<HashSet<ValueId>, RustBackendError> {
    let mut live = HashSet::new();
    for root in roots.values().copied() {
        mark_stamp_live_value(artifact, root, static_cache, &mut live)?;
        let root_value =
            artifact.opt.values.get(usize::from(root)).ok_or_else(|| {
                unsupported(artifact, format!("missing root scalar value {root}"))
            })?;
        for derivative in &root_value.derivatives {
            mark_stamp_live_value(artifact, derivative.value, static_cache, &mut live)?;
        }
    }
    Ok(live)
}

fn mark_stamp_live_value(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    static_cache: &ScalarStaticCache,
    live: &mut HashSet<ValueId>,
) -> Result<(), RustBackendError> {
    if static_cache.contains(value) || !live.insert(value) {
        return Ok(());
    }

    let value_slot = artifact
        .opt
        .values
        .get(usize::from(value))
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
    match value_slot.kind {
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::EquationValue { .. } => {}
        OptValueKind::Unary { input, .. } => {
            mark_stamp_live_value(artifact, input, static_cache, live)?;
        }
        OptValueKind::Binary { left, right, .. } => {
            mark_stamp_live_value(artifact, left, static_cache, live)?;
            mark_stamp_live_value(artifact, right, static_cache, live)?;
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            mark_stamp_live_value(artifact, condition, static_cache, live)?;
            mark_stamp_live_value(artifact, then_value, static_cache, live)?;
            mark_stamp_live_value(artifact, else_value, static_cache, live)?;
        }
    }
    Ok(())
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

fn default_value(value_type: OptValueType) -> &'static str {
    match value_type {
        OptValueType::Real => "0.0",
        OptValueType::Boolean => "false",
    }
}

fn value_name(value: ValueId) -> String {
    format!("v{}", value.index())
}

fn cache_field_name(value: ValueId) -> String {
    format!("scalar_v{}", value.index())
}

fn cached_or_local_value_name(value: ValueId, static_cache: &ScalarStaticCache) -> String {
    if static_cache.contains(value) {
        format!("self.{}", cache_field_name(value))
    } else {
        value_name(value)
    }
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
