use std::collections::{HashMap, HashSet};

use crate::canonical_ir::{
    CanonicalIrArtifact, CanonicalValueType, ExprId, HirAnalogOperator, HirExprKind, HirStatement,
    MirEquationKind,
};

use super::expr::{
    DdtSlots, LoweredVariable, lower_equation_expr_with_variables,
    lower_reactive_expr_with_variables, parameter_field_names, unique_identifiers,
};
use super::{
    GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames, RustTranspileOptions,
};

pub fn generate_device(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
) -> Result<GeneratedRustDevice, RustBackendError> {
    reject_unsupported_model_shape(artifact)?;

    let source_file_name = artifact.metadata.source_package.as_str();
    let names = RustDeviceNames::new(
        source_file_name,
        artifact.mir.module_name.as_str(),
        artifact.metadata.source_digest.as_str(),
    );
    let parameter_fields = parameter_field_names(artifact);
    let variable_fields = variable_local_names(artifact);
    let ddt_slots = collect_ddt_slots(artifact)?;

    Ok(GeneratedRustDevice {
        module_name: artifact.mir.module_name.to_string(),
        public_model_name: names.public_model_name,
        folder_name: names.folder,
        source_digest: artifact.metadata.source_digest.to_string(),
        files: vec![
            GeneratedRustFile {
                relative_path: "mod.rs".to_string(),
                contents: generate_mod_file(),
            },
            GeneratedRustFile {
                relative_path: "state.rs".to_string(),
                contents: generate_state_file(artifact, &parameter_fields, ddt_slots.len())?,
            },
            GeneratedRustFile {
                relative_path: "stamp.rs".to_string(),
                contents: generate_stamp_file(
                    artifact,
                    options,
                    &parameter_fields,
                    &variable_fields,
                    &ddt_slots,
                )?,
            },
        ],
    })
}

fn reject_unsupported_model_shape(artifact: &CanonicalIrArtifact) -> Result<(), RustBackendError> {
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
        if !is_supported_scalar_value_type(variable.value_type) {
            return Err(unsupported(
                artifact,
                format!(
                    "non-numeric scalar variable '{}' with type {:?}",
                    variable.name, variable.value_type
                ),
            ));
        }
    }
    if !artifact.mir.state_slots.is_empty() {
        return Err(unsupported(artifact, "state slots"));
    }
    reject_unsupported_statements(artifact, &artifact.hir.statements)?;

    for equation in &artifact.mir.equations {
        if equation.kind != MirEquationKind::Current {
            return Err(unsupported(artifact, "potential or indirect contributions"));
        }
    }
    for expression in &artifact.mir.expressions {
        match &expression.kind {
            HirExprKind::AnalogOperator { op } => match op {
                HirAnalogOperator::Ddt { abstol, .. } => {
                    if abstol.is_some() {
                        return Err(unsupported(artifact, "ddt abstol argument"));
                    }
                }
                HirAnalogOperator::Limexp { .. } => {
                    return Err(unsupported(artifact, "convergence-limited limexp operator"));
                }
                _ => {
                    return Err(unsupported(
                        artifact,
                        format!("stateful or effectful analog operator {op:?}"),
                    ));
                }
            },
            HirExprKind::Laplace { .. } | HirExprKind::Zi { .. } => {
                return Err(unsupported(
                    artifact,
                    format!(
                        "stateful or effectful expression kind {:?}",
                        expression.kind
                    ),
                ));
            }
            HirExprKind::NoiseSource { .. } => {}
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                if args.len() != 1 {
                    return Err(unsupported(
                        artifact,
                        format!("ddt expects one operand, found {}", args.len()),
                    ));
                }
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {}
            HirExprKind::Call { name, .. } if is_stateful_or_effectful_call(name.as_str()) => {
                return Err(unsupported(
                    artifact,
                    format!("stateful or effectful analog operator call {name}"),
                ));
            }
            _ => {}
        }
    }

    Ok(())
}

fn is_supported_scalar_value_type(value_type: CanonicalValueType) -> bool {
    matches!(
        value_type,
        CanonicalValueType::Real | CanonicalValueType::Integer | CanonicalValueType::Boolean
    )
}

fn reject_unsupported_statements(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
) -> Result<(), RustBackendError> {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                if assignment.index.is_some() {
                    return Err(unsupported(
                        artifact,
                        format!("indexed assignment to '{}'", assignment.target_name),
                    ));
                }
                if usize::from(assignment.target) >= artifact.hir.variables.len() {
                    return Err(RustBackendError::internal(
                        artifact.metadata.source_package.as_str(),
                        artifact.mir.module_name.as_str(),
                        format!(
                            "assignment target {} is outside HIR variable arena",
                            assignment.target
                        ),
                    ));
                }
            }
            HirStatement::Loop(_) => {
                return Err(unsupported(
                    artifact,
                    "runtime analog loops in Rust backend assignment lowering",
                ));
            }
        }
    }
    Ok(())
}

fn collect_ddt_slots(artifact: &CanonicalIrArtifact) -> Result<DdtSlots, RustBackendError> {
    let mut collector = DdtSlotCollector {
        artifact,
        slots: HashMap::new(),
        visited: HashSet::new(),
    };

    for statement in &artifact.hir.statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                collector.collect(assignment.expr.id)?;
                if let Some(index) = &assignment.index {
                    collector.collect(index.id)?;
                }
            }
            HirStatement::Loop(_) => {}
        }
    }
    for equation in &artifact.mir.equations {
        collector.collect(equation.expression.id)?;
    }

    Ok(DdtSlots::new(collector.slots))
}

struct DdtSlotCollector<'a> {
    artifact: &'a CanonicalIrArtifact,
    slots: HashMap<ExprId, usize>,
    visited: HashSet<ExprId>,
}

impl DdtSlotCollector<'_> {
    fn collect(&mut self, id: ExprId) -> Result<(), RustBackendError> {
        if !self.visited.insert(id) {
            return Ok(());
        }

        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| {
                RustBackendError::internal(
                    self.artifact.metadata.source_package.as_str(),
                    self.artifact.mir.module_name.as_str(),
                    format!("expression {id} is outside MIR arena"),
                )
            })?;

        match &expression.kind {
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::Identifier { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
            HirExprKind::SystemFunction { args, .. } | HirExprKind::Call { args, .. } => {
                if let HirExprKind::Call { name, args } = &expression.kind
                    && is_ddt_name(name.as_str())
                {
                    if args.len() != 1 {
                        return Err(unsupported(
                            self.artifact,
                            format!("ddt expects one operand, found {}", args.len()),
                        ));
                    }
                    let next_slot = self.slots.len();
                    self.slots.entry(id).or_insert(next_slot);
                }
                for arg in args {
                    self.collect(*arg)?;
                }
            }
            HirExprKind::Binary { left, right, .. } => {
                self.collect(*left)?;
                self.collect(*right)?;
            }
            HirExprKind::Unary { operand, .. } => {
                self.collect(*operand)?;
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.collect(*condition)?;
                self.collect(*then_expr)?;
                self.collect(*else_expr)?;
            }
            HirExprKind::ArrayAccess { index, .. } => {
                self.collect(*index)?;
            }
            HirExprKind::ArrayLiteral { elements } => {
                for element in elements {
                    self.collect(*element)?;
                }
            }
            HirExprKind::AnalogOperator { op } => {
                self.collect_analog_operator(id, op)?;
            }
            HirExprKind::Laplace { expr, kind } => {
                self.collect(*expr)?;
                match kind {
                    crate::canonical_ir::HirLaplaceKind::ZeroPole { zeros, poles }
                    | crate::canonical_ir::HirLaplaceKind::ZeroDenominator {
                        zeros,
                        denominator: poles,
                    }
                    | crate::canonical_ir::HirLaplaceKind::NumeratorPole {
                        numerator: zeros,
                        poles,
                    }
                    | crate::canonical_ir::HirLaplaceKind::NumeratorDenominator {
                        numerator: zeros,
                        denominator: poles,
                    } => {
                        for child in zeros.iter().chain(poles) {
                            self.collect(*child)?;
                        }
                    }
                }
            }
            HirExprKind::Zi { expr, kind } => {
                self.collect(*expr)?;
                match kind {
                    crate::canonical_ir::HirZiKind::ZeroPole { zeros, poles }
                    | crate::canonical_ir::HirZiKind::ZeroDenominator {
                        zeros,
                        denominator: poles,
                    }
                    | crate::canonical_ir::HirZiKind::NumeratorPole {
                        numerator: zeros,
                        poles,
                    }
                    | crate::canonical_ir::HirZiKind::NumeratorDenominator {
                        numerator: zeros,
                        denominator: poles,
                    } => {
                        for child in zeros.iter().chain(poles) {
                            self.collect(*child)?;
                        }
                    }
                }
            }
            HirExprKind::NoiseSource { operands, .. } => {
                for operand in operands {
                    self.collect(*operand)?;
                }
            }
        }

        Ok(())
    }

    fn collect_analog_operator(
        &mut self,
        id: ExprId,
        op: &HirAnalogOperator,
    ) -> Result<(), RustBackendError> {
        match op {
            HirAnalogOperator::Ddt { expr, abstol } => {
                if abstol.is_some() {
                    return Err(unsupported(self.artifact, "ddt abstol argument"));
                }
                let next_slot = self.slots.len();
                self.slots.entry(id).or_insert(next_slot);
                self.collect(*expr)?;
            }
            HirAnalogOperator::Idt {
                expr,
                ic,
                assert,
                abstol,
            } => {
                self.collect(*expr)?;
                for child in [*ic, *assert, *abstol].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
            } => {
                self.collect(*expr)?;
                for child in [*ic, *modulus, *offset, *abstol].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::Ddx { expr, probe } => {
                self.collect(*expr)?;
                self.collect(*probe)?;
            }
            HirAnalogOperator::Limexp { expr } => {
                self.collect(*expr)?;
            }
            HirAnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
            } => {
                self.collect(*expr)?;
                self.collect(*delay)?;
                if let Some(max_delay) = max_delay {
                    self.collect(*max_delay)?;
                }
            }
            HirAnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
            } => {
                self.collect(*expr)?;
                for child in [*delay, *rise, *fall, *tolerance].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
            } => {
                self.collect(*expr)?;
                for child in [*max_rise, *max_fall].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::LastCrossing { expr, .. } => {
                self.collect(*expr)?;
            }
        }
        Ok(())
    }
}

fn is_stateful_or_effectful_call(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "idt"
            | "idtmod"
            | "ddx"
            | "absdelay"
            | "transition"
            | "slew"
            | "last_crossing"
            | "laplace_zp"
            | "laplace_zd"
            | "laplace_np"
            | "laplace_nd"
            | "zi_zp"
            | "zi_zd"
            | "zi_np"
            | "zi_nd"
            | "noise_table"
            | "noise_table_log"
    )
}

fn is_ddt_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("ddt")
}

fn is_noise_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "white_noise" | "$white_noise" | "flicker_noise" | "$flicker_noise"
    )
}

fn generate_mod_file() -> String {
    [
        "pub mod state;",
        "mod stamp;",
        "",
        "pub use state::{Instance, Parameters};",
        "",
    ]
    .join("\n")
}

fn generate_state_file(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    ddt_state_count: usize,
) -> Result<String, RustBackendError> {
    let mut out = String::new();
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub struct Parameters {\n");
    for parameter in &artifact.mir.parameters {
        let field = &parameter_fields[parameter.name.as_str()];
        out.push_str(&format!("    pub {field}: f64,\n"));
    }
    out.push_str("}\n\n");

    out.push_str("impl Default for Parameters {\n");
    out.push_str("    fn default() -> Self {\n");
    out.push_str("        Self {\n");
    for parameter in &artifact.mir.parameters {
        let field = &parameter_fields[parameter.name.as_str()];
        let default = validated_default(
            artifact,
            parameter.name.as_str(),
            parameter.default,
            parameter.range.as_ref(),
        )?;
        out.push_str(&format!("            {field}: {},\n", format_f64(default)));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    for parameter in &artifact.mir.parameters {
        let field = &parameter_fields[parameter.name.as_str()];
        out.push_str(&generate_parameter_validator(
            parameter.name.as_str(),
            field,
            parameter.range.as_ref(),
        )?);
        out.push('\n');
    }

    let node_count = artifact.mir.nodes.len();
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub struct Instance {\n");
    out.push_str(&format!("    pub nodes: [usize; {node_count}],\n"));
    out.push_str("    pub params: Parameters,\n");
    out.push_str(&format!(
        "    pub(crate) ddt_state_current: [f64; {ddt_state_count}],\n"
    ));
    out.push_str(&format!(
        "    pub(crate) ddt_state_previous: [f64; {ddt_state_count}],\n"
    ));
    out.push_str(&format!(
        "    pub(crate) ddt_state_initialized: [bool; {ddt_state_count}],\n"
    ));
    out.push_str("    pub(crate) time: f64,\n");
    out.push_str("    pub(crate) timestep: f64,\n");
    out.push_str("}\n\n");

    out.push_str("impl Instance {\n");
    out.push_str(&format!(
        "    pub const NODE_COUNT: usize = {node_count};\n\n"
    ));
    out.push_str(&format!(
        "    pub const DDT_STATE_COUNT: usize = {ddt_state_count};\n"
    ));
    out.push_str("    pub const DDT_EPSILON: f64 = 1.0e-20;\n\n");
    out.push_str("    pub fn new(nodes: &[usize]) -> Self {\n");
    out.push_str("        assert_eq!(nodes.len(), Self::NODE_COUNT, \"generated Verilog-A node count mismatch\");\n");
    out.push_str("        let mut mapped = [0usize; Self::NODE_COUNT];\n");
    out.push_str("        mapped.copy_from_slice(nodes);\n");
    out.push_str("        Self {\n");
    out.push_str("            nodes: mapped,\n");
    out.push_str("            params: Parameters::default(),\n");
    out.push_str("            ddt_state_current: [0.0; Self::DDT_STATE_COUNT],\n");
    out.push_str("            ddt_state_previous: [0.0; Self::DDT_STATE_COUNT],\n");
    out.push_str("            ddt_state_initialized: [false; Self::DDT_STATE_COUNT],\n");
    out.push_str("            time: 0.0,\n");
    out.push_str("            timestep: 0.0,\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {\n",
    );
    out.push_str("        match name.to_ascii_lowercase().as_str() {\n");
    for parameter in &artifact.mir.parameters {
        let field = &parameter_fields[parameter.name.as_str()];
        out.push_str(&format!(
            "            \"{}\" => {{ validate_parameter_{field}(value)?; self.params.{field} = value; Ok(()) }}\n",
            parameter.name.to_ascii_lowercase()
        ));
        for alias in &parameter.aliases {
            out.push_str(&format!(
                "            \"{}\" => {{ validate_parameter_{field}(value)?; self.params.{field} = value; Ok(()) }}\n",
                alias.to_ascii_lowercase()
            ));
        }
    }
    out.push_str(&format!(
        "            _ => Err(format!(\"unknown parameter '{{}}' for generated Verilog-A model '{}'\", name)),\n",
        artifact.mir.module_name
    ));
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push('\n');
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn set_timepoint(&mut self, time: f64, timestep: f64) {\n");
    out.push_str("        self.time = time;\n");
    out.push_str("        self.timestep = timestep;\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn accept_timestep(&mut self) {\n");
    out.push_str("        let mut index = 0usize;\n");
    out.push_str("        while index < Self::DDT_STATE_COUNT {\n");
    out.push_str("            self.ddt_state_previous[index] = self.ddt_state_current[index];\n");
    out.push_str("            self.ddt_state_initialized[index] = true;\n");
    out.push_str("            index += 1;\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    if ddt_state_count > 0 {
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {\n");
        out.push_str("        debug_assert!(slot < Self::DDT_STATE_COUNT, \"generated ddt state slot out of range\");\n");
        out.push_str("        let previous = if self.ddt_state_initialized[slot] {\n");
        out.push_str("            self.ddt_state_previous[slot]\n");
        out.push_str("        } else {\n");
        out.push_str("            value\n");
        out.push_str("        };\n");
        out.push_str("        self.ddt_state_current[slot] = value;\n");
        out.push_str("        if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            (value - previous) / self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            self.ddt_state_previous[slot] = value;\n");
        out.push_str("            self.ddt_state_initialized[slot] = true;\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n\n");
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {\n");
        out.push_str("        if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            derivative / self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
    }
    out.push_str("}\n");
    Ok(out)
}

fn validated_default(
    artifact: &CanonicalIrArtifact,
    parameter_name: &str,
    default: Option<f64>,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<f64, RustBackendError> {
    let Some(default) = default else {
        return Err(unsupported(
            artifact,
            format!("parameter '{parameter_name}' default that does not fold to a constant"),
        ));
    };
    validate_parameter_value_for_codegen(artifact, parameter_name, default, range)?;
    Ok(default)
}

fn validate_parameter_value_for_codegen(
    artifact: &CanonicalIrArtifact,
    parameter_name: &str,
    value: f64,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<(), RustBackendError> {
    if !value.is_finite() {
        return Err(unsupported(
            artifact,
            format!("non-finite default for parameter '{parameter_name}'"),
        ));
    }
    if let Some(range) = range {
        if !range_contains(range, value) {
            return Err(unsupported(
                artifact,
                format!("default for parameter '{parameter_name}' violates declared range"),
            ));
        }
        if range.exclude.iter().any(|excluded| !excluded.is_finite()) {
            return Err(unsupported(
                artifact,
                format!("non-finite exclude constraint for parameter '{parameter_name}'"),
            ));
        }
    }
    Ok(())
}

fn range_contains(range: &crate::canonical_ir::HirParamRange, value: f64) -> bool {
    if let Some(min) = range.min {
        if range.min_exclusive {
            if value <= min {
                return false;
            }
        } else if value < min {
            return false;
        }
    }
    if let Some(max) = range.max {
        if range.max_exclusive {
            if value >= max {
                return false;
            }
        } else if value > max {
            return false;
        }
    }
    !range.exclude.contains(&value)
}

fn generate_parameter_validator(
    parameter_name: &str,
    field_name: &str,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<String, RustBackendError> {
    let mut out = String::new();
    out.push_str(&format!(
        "fn validate_parameter_{field_name}(value: f64) -> Result<(), String> {{\n"
    ));
    out.push_str("    if !value.is_finite() {\n");
    out.push_str(&format!(
        "        return Err(format!(\"parameter '{}' must be finite, got {{}}\", value));\n",
        parameter_name
    ));
    out.push_str("    }\n");

    if let Some(range) = range {
        if let Some(min) = range.min.filter(|value| value.is_finite()) {
            let op = if range.min_exclusive { ">" } else { ">=" };
            let condition = if range.min_exclusive { "<=" } else { "<" };
            out.push_str(&format!(
                "    if value {condition} {} {{\n",
                format_f64(min)
            ));
            out.push_str(&format!(
                "        return Err(format!(\"parameter '{}' must be {op} {}, got {{}}\", value));\n",
                parameter_name,
                format_f64(min)
            ));
            out.push_str("    }\n");
        }
        if let Some(max) = range.max.filter(|value| value.is_finite()) {
            let op = if range.max_exclusive { "<" } else { "<=" };
            let condition = if range.max_exclusive { ">=" } else { ">" };
            out.push_str(&format!(
                "    if value {condition} {} {{\n",
                format_f64(max)
            ));
            out.push_str(&format!(
                "        return Err(format!(\"parameter '{}' must be {op} {}, got {{}}\", value));\n",
                parameter_name,
                format_f64(max)
            ));
            out.push_str("    }\n");
        }
        for excluded in &range.exclude {
            if !excluded.is_finite() {
                return Err(RustBackendError::unsupported(
                    "<generated>",
                    parameter_name,
                    "non-finite parameter exclude constraint",
                ));
            }
            out.push_str(&format!("    if value == {} {{\n", format_f64(*excluded)));
            out.push_str(&format!(
                "        return Err(format!(\"parameter '{}' must not equal {}, got {{}}\", value));\n",
                parameter_name,
                format_f64(*excluded)
            ));
            out.push_str("    }\n");
        }
    }

    out.push_str("    Ok(())\n");
    out.push_str("}\n");
    Ok(out)
}

fn generate_stamp_file(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    ddt_slots: &DdtSlots,
) -> Result<String, RustBackendError> {
    let mut out = String::new();
    out.push_str("#![allow(unused_assignments, unused_parens, unused_variables)]\n\n");
    out.push_str("use super::state::Instance;\n");
    out.push_str(&format!(
        "use {}::{{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};\n\n",
        options.runtime_path
    ));
    out.push_str("impl Instance {\n");
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
    );
    emit_stamp_body(
        artifact,
        parameter_fields,
        variable_fields,
        ddt_slots,
        false,
        &mut out,
    )?;
    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
    );
    emit_stamp_body(
        artifact,
        parameter_fields,
        variable_fields,
        ddt_slots,
        true,
        &mut out,
    )?;
    out.push_str("    }\n");
    out.push_str("}\n");
    Ok(out)
}

fn emit_stamp_body(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    ddt_slots: &DdtSlots,
    reactive: bool,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let mut variables = emit_variable_initializers(artifact, variable_fields, out);
    emit_assignment_statements(
        artifact,
        parameter_fields,
        variable_fields,
        &mut variables,
        out,
        ddt_slots,
        reactive,
    )?;

    for (index, equation) in artifact.mir.equations.iter().enumerate() {
        let prefix = format!("eq{index}");
        let lowered = if reactive {
            lower_reactive_expr_with_variables(
                artifact,
                equation.expression.id,
                &prefix,
                parameter_fields,
                &variables,
                ddt_slots,
            )?
        } else {
            lower_equation_expr_with_variables(
                artifact,
                equation.expression.id,
                &prefix,
                parameter_fields,
                &variables,
                ddt_slots,
            )?
        };
        if reactive {
            if lowered.has_reactive {
                for line in lowered.lines {
                    out.push_str("        ");
                    out.push_str(&line);
                    out.push('\n');
                }
                out.push_str("        stamper.stamp_current_reactive(\n");
                out.push_str(&format!(
                    "            {},\n",
                    optional_node_expr(equation.branch.pos_node)
                ));
                out.push_str(&format!(
                    "            {},\n",
                    optional_node_expr(equation.branch.neg_node)
                ));
                out.push_str("            &[\n");
                for node_index in 0..artifact.mir.nodes.len() {
                    out.push_str(&format!(
                        "                (self.nodes[{node_index}], {}),\n",
                        lowered.reactive_derivatives[node_index]
                    ));
                }
                out.push_str("            ],\n");
                out.push_str("        );\n");
            }
            continue;
        }

        for line in lowered.lines {
            out.push_str("        ");
            out.push_str(&line);
            out.push('\n');
        }

        let value = format!("{prefix}_value");
        out.push_str(&format!("        let {value}: f64 = {};\n", lowered.value));
        for (node_index, derivative) in lowered.derivatives.iter().enumerate() {
            out.push_str(&format!(
                "        let {prefix}_d_n{node_index}: f64 = {derivative};\n"
            ));
        }
        out.push_str("        stamper.stamp_current(\n");
        out.push_str(&format!(
            "            {},\n",
            optional_node_expr(equation.branch.pos_node)
        ));
        out.push_str(&format!(
            "            {},\n",
            optional_node_expr(equation.branch.neg_node)
        ));
        out.push_str(&format!("            {value},\n"));
        out.push_str("            &[\n");
        for node_index in 0..artifact.mir.nodes.len() {
            out.push_str(&format!(
                "                (self.nodes[{node_index}], {prefix}_d_n{node_index}),\n"
            ));
        }
        out.push_str("            ],\n");
        out.push_str("        );\n");
    }
    Ok(())
}

fn variable_local_names(artifact: &CanonicalIrArtifact) -> HashMap<String, String> {
    let names = artifact
        .hir
        .variables
        .iter()
        .map(|variable| variable.name.to_string())
        .collect::<Vec<_>>();
    unique_identifiers(&names)
}

fn emit_variable_initializers(
    artifact: &CanonicalIrArtifact,
    variable_fields: &HashMap<String, String>,
    out: &mut String,
) -> HashMap<String, LoweredVariable> {
    let mut variables = HashMap::new();
    for variable in &artifact.hir.variables {
        let local = variable_fields[variable.name.as_str()].clone();
        out.push_str(&format!("        let mut {local}: f64 = 0.0;\n"));
        let mut derivatives = Vec::with_capacity(artifact.mir.nodes.len());
        let reactive_value = format!("{local}_q");
        out.push_str(&format!("        let mut {reactive_value}: f64 = 0.0;\n"));
        let mut reactive_derivatives = Vec::with_capacity(artifact.mir.nodes.len());
        for node_index in 0..artifact.mir.nodes.len() {
            let derivative = format!("{local}_d_n{node_index}");
            out.push_str(&format!("        let mut {derivative}: f64 = 0.0;\n"));
            derivatives.push(derivative);
            let reactive_derivative = format!("{local}_q_d_n{node_index}");
            out.push_str(&format!(
                "        let mut {reactive_derivative}: f64 = 0.0;\n"
            ));
            reactive_derivatives.push(reactive_derivative);
        }
        variables.insert(
            variable.name.to_string(),
            LoweredVariable {
                value: local,
                derivatives,
                has_reactive: false,
                reactive_value,
                reactive_derivatives,
            },
        );
    }
    if !artifact.hir.variables.is_empty() {
        out.push('\n');
    }
    variables
}

fn emit_assignment_statements(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    reactive: bool,
) -> Result<(), RustBackendError> {
    for (index, statement) in artifact.hir.statements.iter().enumerate() {
        let HirStatement::Assignment(assignment) = statement else {
            return Err(unsupported(
                artifact,
                "runtime analog loops in Rust backend assignment lowering",
            ));
        };

        let prefix = format!("assign{index}");
        let lowered = if reactive {
            lower_reactive_expr_with_variables(
                artifact,
                assignment.expr.id,
                &prefix,
                parameter_fields,
                variables,
                ddt_slots,
            )?
        } else {
            lower_equation_expr_with_variables(
                artifact,
                assignment.expr.id,
                &prefix,
                parameter_fields,
                variables,
                ddt_slots,
            )?
        };
        for line in lowered.lines {
            out.push_str("        ");
            out.push_str(&line);
            out.push('\n');
        }

        let target = artifact
            .hir
            .variables
            .get(usize::from(assignment.target))
            .ok_or_else(|| {
                RustBackendError::internal(
                    artifact.metadata.source_package.as_str(),
                    artifact.mir.module_name.as_str(),
                    format!(
                        "assignment target {} is outside HIR variable arena",
                        assignment.target
                    ),
                )
            })?;
        let target_local = variable_fields[target.name.as_str()].clone();
        out.push_str(&format!("        {target_local} = {};\n", lowered.value));
        for (node_index, derivative) in lowered.derivatives.iter().enumerate() {
            out.push_str(&format!(
                "        {target_local}_d_n{node_index} = {derivative};\n"
            ));
        }
        let target_reactive = format!("{target_local}_q");
        out.push_str(&format!(
            "        {target_reactive} = {};\n",
            lowered.reactive_value
        ));
        for (node_index, derivative) in lowered.reactive_derivatives.iter().enumerate() {
            out.push_str(&format!(
                "        {target_reactive}_d_n{node_index} = {derivative};\n"
            ));
        }

        let derivative_locals = (0..artifact.mir.nodes.len())
            .map(|node_index| format!("{target_local}_d_n{node_index}"))
            .collect();
        let reactive_derivative_locals = (0..artifact.mir.nodes.len())
            .map(|node_index| format!("{target_reactive}_d_n{node_index}"))
            .collect();
        variables.insert(
            target.name.to_string(),
            LoweredVariable {
                value: target_local,
                derivatives: derivative_locals,
                has_reactive: lowered.has_reactive,
                reactive_value: target_reactive,
                reactive_derivatives: reactive_derivative_locals,
            },
        );
    }
    if !artifact.hir.statements.is_empty() {
        out.push('\n');
    }
    Ok(())
}

fn optional_node_expr(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some(self.nodes[{}])", node.index()))
        .unwrap_or_else(|| "None".to_string())
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
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
