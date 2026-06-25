use std::collections::HashMap;

use crate::canonical_ir::{CanonicalIrArtifact, HirExprKind, MirEquationKind};

use super::expr::{lower_equation_expr, parameter_field_names};
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
                contents: generate_state_file(artifact, &parameter_fields),
            },
            GeneratedRustFile {
                relative_path: "stamp.rs".to_string(),
                contents: generate_stamp_file(artifact, options, &parameter_fields)?,
            },
        ],
    })
}

fn reject_unsupported_model_shape(artifact: &CanonicalIrArtifact) -> Result<(), RustBackendError> {
    if !artifact.hir.statements.is_empty() {
        return Err(unsupported(
            artifact,
            "analog assignment/control statements",
        ));
    }
    if !artifact.hir.arrays.is_empty() {
        return Err(unsupported(artifact, "arrays"));
    }
    if !artifact.hir.variables.is_empty() {
        return Err(unsupported(artifact, "analog variables"));
    }
    if !artifact.mir.state_slots.is_empty() {
        return Err(unsupported(artifact, "state slots"));
    }

    for equation in &artifact.mir.equations {
        if equation.kind != MirEquationKind::Current {
            return Err(unsupported(artifact, "potential or indirect contributions"));
        }
    }
    for expression in &artifact.mir.expressions {
        match &expression.kind {
            HirExprKind::AnalogOperator { op } => match op {
                crate::canonical_ir::HirAnalogOperator::Limexp { .. } => {}
                _ => {
                    return Err(unsupported(
                        artifact,
                        format!("stateful or effectful analog operator {op:?}"),
                    ));
                }
            },
            HirExprKind::Laplace { .. }
            | HirExprKind::Zi { .. }
            | HirExprKind::NoiseSource { .. } => {
                return Err(unsupported(
                    artifact,
                    format!(
                        "stateful or effectful expression kind {:?}",
                        expression.kind
                    ),
                ));
            }
            _ => {}
        }
    }

    Ok(())
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
) -> String {
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
        let default = parameter.default.unwrap_or(0.0);
        out.push_str(&format!("            {field}: {},\n", format_f64(default)));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    let node_count = artifact.mir.nodes.len();
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub struct Instance {\n");
    out.push_str(&format!("    pub nodes: [usize; {node_count}],\n"));
    out.push_str("    pub params: Parameters,\n");
    out.push_str("}\n\n");

    out.push_str("impl Instance {\n");
    out.push_str(&format!(
        "    pub const NODE_COUNT: usize = {node_count};\n\n"
    ));
    out.push_str("    pub fn new(nodes: &[usize]) -> Self {\n");
    out.push_str("        assert_eq!(nodes.len(), Self::NODE_COUNT, \"generated Verilog-A node count mismatch\");\n");
    out.push_str("        let mut mapped = [0usize; Self::NODE_COUNT];\n");
    out.push_str("        mapped.copy_from_slice(nodes);\n");
    out.push_str("        Self { nodes: mapped, params: Parameters::default() }\n");
    out.push_str("    }\n\n");
    out.push_str("    pub fn set_parameter(&mut self, name: &str, value: f64) -> bool {\n");
    out.push_str("        match name.to_ascii_lowercase().as_str() {\n");
    for parameter in &artifact.mir.parameters {
        let field = &parameter_fields[parameter.name.as_str()];
        out.push_str(&format!(
            "            \"{}\" => {{ self.params.{field} = value; true }}\n",
            parameter.name.to_ascii_lowercase()
        ));
        for alias in &parameter.aliases {
            out.push_str(&format!(
                "            \"{}\" => {{ self.params.{field} = value; true }}\n",
                alias.to_ascii_lowercase()
            ));
        }
    }
    out.push_str("            _ => false,\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

fn generate_stamp_file(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    let mut out = String::new();
    out.push_str("use super::state::Instance;\n");
    out.push_str(&format!(
        "use {}::{{GeneratedEvalContext, GeneratedStamper}};\n\n",
        options.runtime_path
    ));
    out.push_str("impl Instance {\n");
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
    );

    for (index, equation) in artifact.mir.equations.iter().enumerate() {
        let prefix = format!("eq{index}");
        let lowered =
            lower_equation_expr(artifact, equation.expression.id, &prefix, parameter_fields)?;
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

    out.push_str("    }\n");
    out.push_str("}\n");
    Ok(out)
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
