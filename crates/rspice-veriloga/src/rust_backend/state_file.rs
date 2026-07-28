//! The parts of the generated device that are not the body.
//!
//! `state.rs` and `mod.rs` for a generated device, and the `ddt`/`idt` slot
//! census the state file is sized from. None of it is emitter-specific: it
//! describes the shape a device presents to `rspice-core`, which is the same
//! whichever backend wrote the stamp.
//!
//! Extracted from the 56,506-line `device.rs` when Phase 6 deleted the legacy
//! tiers. The canonical emitter needed exactly this much of that file and
//! nothing else.

use std::collections::HashMap;

use smol_str::SmolStr;

use crate::canonical_ir::{
    CanonicalIrArtifact, CanonicalValueType, ExprId, HirExprKind, MirParameterSlot,
};

use super::expr::comparison_operator;

use super::{GeneratedRustDevice, RustBackendError, RustTranspileOptions};

/// How many packed scratch workspaces a generated device keeps pooled.
pub(super) const MAX_CACHED_SCRATCH_WORKSPACES: usize = 2;

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}


pub(super) fn generate_mod_file() -> String {
    [
        "#[cfg(feature = \"veriloga-builtins-noise\")]",
        "pub mod noise;",
        "pub mod state;",
        "mod stamp;",
        "",
        "#[cfg(feature = \"veriloga-builtins-noise\")]",
        "pub use noise::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor, NOISE_SOURCES};",
        "pub use state::{Instance, Parameters};",
        "",
    ]
    .join("\n")
}

#[derive(Debug, Clone)]
pub(super) struct StateFileExtensions {
    pub params_visibility: &'static str,
    pub support_types: String,
    pub instance_fields: String,
    pub clone_fields: String,
    pub new_initializers: String,
    pub after_new: String,
    pub set_parameter_hook: String,
    pub impl_methods: String,
    pub limiter_converged_expr: String,
    pub rollback_value_count: usize,
    pub rollback_flag_count: usize,
    pub rollback_capture_values: String,
    pub rollback_capture_flags: String,
    pub rollback_restore_fields: String,
    pub checkpoint_capture_fields: String,
    pub checkpoint_shape_checks: String,
    pub checkpoint_restore_fields: String,
}

impl Default for StateFileExtensions {
    fn default() -> Self {
        Self {
            params_visibility: "pub",
            support_types: String::new(),
            instance_fields: String::new(),
            clone_fields: String::new(),
            new_initializers: String::new(),
            after_new: String::new(),
            set_parameter_hook: String::new(),
            impl_methods: String::new(),
            limiter_converged_expr: "true".to_string(),
            rollback_value_count: 0,
            rollback_flag_count: 0,
            rollback_capture_values: String::new(),
            rollback_capture_flags: String::new(),
            rollback_restore_fields: String::new(),
            checkpoint_capture_fields:
                "            limiter_anchor: Vec::new(),\n            limiter_initialized: Vec::new(),\n"
                    .to_string(),
            checkpoint_shape_checks: String::new(),
            checkpoint_restore_fields: String::new(),
        }
    }
}

pub(super) fn generate_state_file_with_extensions(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
    parameter_fields: &HashMap<String, String>,
    ddt_state_count: usize,
    idt_state_count: usize,
    branch_count: usize,
    extensions: &StateFileExtensions,
) -> Result<String, RustBackendError> {
    let checkpoint_model_identity = CHECKPOINT_IDENTITY_PLACEHOLDER;
    let mut out = String::new();
    out.push_str("#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]\n\n");
    out.push_str(&format!(
        "use {}::{{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState}};\n",
        options.runtime_path
    ));
    out.push('\n');
    out.push_str("#[repr(C)]\n");
    out.push_str("#[derive(Copy, Clone)]\n");
    out.push_str("pub struct Parameters {\n");
    out.push_str(&format!(
        "    pub values: [f64; {}],\n",
        artifact.mir.parameters.len()
    ));
    out.push_str("}\n\n");

    out.push_str("impl std::ops::Index<usize> for Parameters {\n");
    out.push_str("    type Output = f64;\n");
    out.push_str("    #[inline]\n");
    out.push_str("    fn index(&self, index: usize) -> &Self::Output { &self.values[index] }\n");
    out.push_str("}\n\n");
    out.push_str("impl std::ops::IndexMut<usize> for Parameters {\n");
    out.push_str("    #[inline]\n");
    out.push_str(
        "    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.values[index] }\n",
    );
    out.push_str("}\n\n");

    out.push_str("impl Parameters {\n");
    out.push_str("    fn new_box() -> Box<Self> {\n");
    if artifact.mir.parameters.is_empty() {
        out.push_str("        Box::new(Self { values: [] })\n");
    } else {
        out.push_str("        // SAFETY: every parameter slot is f64, so zero bytes are valid 0.0 values; numeric default chunks are copied into the values array.\n");
        out.push_str("        let mut boxed = Box::<Self>::new_uninit();\n");
        out.push_str("        unsafe {\n");
        out.push_str("            let ptr = boxed.as_mut_ptr();\n");
        out.push_str("            std::ptr::write_bytes(ptr, 0, 1);\n");
        emit_parameter_defaults(artifact, parameter_fields, &mut out)?;
        out.push_str("            let params = &*ptr;\n");
        out.push_str("            for index in 0..PARAMETER_DISPLAY_NAMES.len() {\n");
        out.push_str("                let value = read_parameter_slot(params, index);\n");
        out.push_str("                validate_parameter_metadata(params, index, value).expect(\"generated Verilog-A parameter defaults must satisfy declared ranges\");\n");
        out.push_str("            }\n");
        out.push_str("            boxed.assume_init()\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("impl Default for Parameters {\n");
    out.push_str("    fn default() -> Self {\n");
    out.push_str("        *Self::new_box()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    if !artifact.mir.parameters.is_empty() {
        out.push_str(&generate_shared_parameter_validator());
        out.push('\n');
        emit_parameter_metadata(artifact, parameter_fields, &mut out)?;
    }

    out.push_str("fn boxed_zero_f64_array<const N: usize>() -> Box<[f64; N]> {\n");
    out.push_str("    let mut boxed = Box::<[f64; N]>::new_uninit();\n");
    out.push_str("    unsafe {\n");
    out.push_str("        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n");
    out.push_str("        boxed.assume_init()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("fn boxed_zero_bool_array<const N: usize>() -> Box<[bool; N]> {\n");
    out.push_str("    let mut boxed = Box::<[bool; N]>::new_uninit();\n");
    out.push_str("    unsafe {\n");
    out.push_str("        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n");
    out.push_str("        boxed.assume_init()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("#[derive(Clone)]\n");
    out.push_str("pub(crate) struct StampState<const DDT: usize, const IDT: usize> {\n");
    out.push_str("    pub(crate) ddt_current: [f64; DDT],\n");
    out.push_str("    pub(crate) ddt_previous: [f64; DDT],\n");
    out.push_str("    pub(crate) ddt_older: [f64; DDT],\n");
    out.push_str("    pub(crate) ddt_derivative_current: [f64; DDT],\n");
    out.push_str("    pub(crate) ddt_derivative_previous: [f64; DDT],\n");
    out.push_str("    pub(crate) idt_current: [f64; IDT],\n");
    out.push_str("    pub(crate) idt_previous: [f64; IDT],\n");
    out.push_str("    pub(crate) ddt_initialized: [bool; DDT],\n");
    out.push_str("    pub(crate) idt_initialized: [bool; IDT],\n");
    out.push_str("}\n\n");
    out.push_str("impl<const DDT: usize, const IDT: usize> StampState<DDT, IDT> {\n");
    out.push_str("    fn new_box() -> Box<Self> {\n");
    out.push_str("        let mut boxed = Box::<Self>::new_uninit();\n");
    out.push_str("        unsafe {\n");
    out.push_str("            // SAFETY: every field is an array of f64 or bool; all-zero bytes are valid values for both.\n");
    out.push_str("            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n");
    out.push_str("            boxed.assume_init()\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str(&extensions.support_types);

    let node_count = artifact.mir.nodes.len();
    let terminal_count = artifact
        .mir
        .nodes
        .iter()
        .filter(|node| node.is_external)
        .count();
    let internal_node_names: Vec<_> = artifact
        .mir
        .nodes
        .iter()
        .filter(|node| !node.is_external)
        .map(|node| node.name.as_str())
        .collect();
    let internal_node_count = internal_node_names.len();
    let parameter_count = artifact.mir.parameters.len();
    let variable_count = artifact.hir.variables.len();
    out.push_str("pub struct Instance {\n");
    out.push_str(&format!("    pub nodes: [usize; {node_count}],\n"));
    out.push_str(&format!("    pub branches: [usize; {branch_count}],\n"));
    out.push_str(&format!(
        "    {} params: Box<Parameters>,\n",
        extensions.params_visibility
    ));
    out.push_str(&format!(
        "    pub(crate) param_given: Box<[bool; {parameter_count}]>,\n"
    ));
    out.push_str("    pub(crate) multiplicity: f64,\n");
    out.push_str(&format!(
        "    pub(crate) stamp_state: Box<StampState<{ddt_state_count}, {idt_state_count}>>,\n"
    ));
    out.push_str("    pub(crate) time: f64,\n");
    out.push_str("    pub(crate) timestep: f64,\n");
    out.push_str("    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,\n");
    out.push_str(&extensions.instance_fields);
    out.push_str("}\n\n");
    out.push_str("impl Clone for Instance {\n");
    out.push_str("    #[inline]\n");
    out.push_str("    fn clone(&self) -> Self {\n");
    out.push_str("        Self {\n");
    out.push_str("            nodes: self.nodes,\n");
    out.push_str("            branches: self.branches,\n");
    out.push_str("            params: self.params.clone(),\n");
    out.push_str("            param_given: self.param_given.clone(),\n");
    out.push_str("            multiplicity: self.multiplicity,\n");
    out.push_str("            stamp_state: self.stamp_state.clone(),\n");
    out.push_str("            time: self.time,\n");
    out.push_str("            timestep: self.timestep,\n");
    out.push_str("            ddt_coefficients: self.ddt_coefficients,\n");
    out.push_str(&extensions.clone_fields);
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("impl Instance {\n");
    out.push_str(&format!(
        "    pub const TERMINAL_COUNT: usize = {terminal_count};\n"
    ));
    out.push_str(&format!(
        "    pub const INTERNAL_NODE_COUNT: usize = {internal_node_count};\n"
    ));
    out.push_str(&format!(
        "    pub const NODE_COUNT: usize = {node_count};\n"
    ));
    out.push_str(&format!(
        "    pub const INTERNAL_NODE_NAMES: [&str; {internal_node_count}] = [{}];\n\n",
        internal_node_names
            .iter()
            .map(|name| format!("{name:?}"))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "    pub const BRANCH_COUNT: usize = {branch_count};\n"
    ));
    out.push_str(&format!(
        "    pub const PARAMETER_COUNT: usize = {parameter_count};\n"
    ));
    out.push_str(&format!(
        "    pub const VARIABLE_COUNT: usize = {};\n",
        variable_count
    ));
    out.push_str(&format!(
        "    pub const DDT_STATE_COUNT: usize = {ddt_state_count};\n"
    ));
    out.push_str(&format!(
        "    pub const IDT_STATE_COUNT: usize = {idt_state_count};\n"
    ));
    out.push_str(&format!(
        "    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = {checkpoint_model_identity:?};\n"
    ));
    out.push_str("    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;\n");
    out.push_str("    pub const DDT_EPSILON: f64 = 1.0e-20;\n\n");
    out.push_str("    pub fn new(nodes: &[usize]) -> Self {\n");
    out.push_str("        assert_eq!(nodes.len(), Self::NODE_COUNT, \"generated Verilog-A node count mismatch\");\n");
    out.push_str("        let mut mapped = [0usize; Self::NODE_COUNT];\n");
    out.push_str("        mapped.copy_from_slice(nodes);\n");
    if extensions.after_new.is_empty() {
        out.push_str("        Self {\n");
    } else {
        out.push_str("        let mut instance = Self {\n");
    }
    out.push_str("            nodes: mapped,\n");
    out.push_str("            branches: [0usize; Self::BRANCH_COUNT],\n");
    out.push_str("            params: Parameters::new_box(),\n");
    out.push_str(
        "            param_given: boxed_zero_bool_array::<{ Self::PARAMETER_COUNT }>(),\n",
    );
    out.push_str("            multiplicity: 1.0,\n");
    out.push_str("            stamp_state: StampState::new_box(),\n");
    out.push_str("            time: 0.0,\n");
    out.push_str("            timestep: 0.0,\n");
    out.push_str("            ddt_coefficients: GeneratedDdtCoefficients::inactive(),\n");
    out.push_str(&extensions.new_initializers);
    if extensions.after_new.is_empty() {
        out.push_str("        }\n");
    } else {
        out.push_str("        };\n");
        out.push_str(&extensions.after_new);
        out.push_str("        instance\n");
    }
    out.push_str("    }\n\n");
    let rollback_value_count = ddt_state_count
        .saturating_mul(5)
        .saturating_add(idt_state_count.saturating_mul(2))
        .saturating_add(extensions.rollback_value_count);
    let rollback_flag_count = ddt_state_count
        .saturating_add(idt_state_count)
        .saturating_add(extensions.rollback_flag_count);
    out.push_str(
        "    pub(crate) fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {\n",
    );
    out.push_str(&format!(
        "        let mut values = Vec::with_capacity({rollback_value_count});\n"
    ));
    out.push_str("        values.extend_from_slice(&self.stamp_state.ddt_current);\n");
    out.push_str("        values.extend_from_slice(&self.stamp_state.ddt_previous);\n");
    out.push_str("        values.extend_from_slice(&self.stamp_state.ddt_older);\n");
    out.push_str("        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);\n");
    out.push_str("        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);\n");
    out.push_str("        values.extend_from_slice(&self.stamp_state.idt_current);\n");
    out.push_str("        values.extend_from_slice(&self.stamp_state.idt_previous);\n");
    out.push_str(&extensions.rollback_capture_values);
    out.push_str(&format!(
        "        let mut flags = Vec::with_capacity({rollback_flag_count});\n"
    ));
    out.push_str("        flags.extend_from_slice(&self.stamp_state.ddt_initialized);\n");
    out.push_str("        flags.extend_from_slice(&self.stamp_state.idt_initialized);\n");
    out.push_str(&extensions.rollback_capture_flags);
    out.push_str("        GeneratedVerilogARollbackState { values, flags }\n");
    out.push_str("    }\n\n");

    out.push_str(
        "    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {\n",
    );
    out.push_str(&format!(
        "        debug_assert_eq!(state.values.len(), {rollback_value_count});\n"
    ));
    out.push_str(&format!(
        "        debug_assert_eq!(state.flags.len(), {rollback_flag_count});\n"
    ));
    out.push_str("        let mut rollback_values = state.values.as_slice();\n");
    for field in [
        "ddt_current",
        "ddt_previous",
        "ddt_older",
        "ddt_derivative_current",
        "ddt_derivative_previous",
    ] {
        out.push_str(&format!(
            "        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);\n        self.stamp_state.{field}.copy_from_slice(field);\n        rollback_values = remaining;\n"
        ));
    }
    for field in ["idt_current", "idt_previous"] {
        out.push_str(&format!(
            "        let (field, remaining) = rollback_values.split_at(Self::IDT_STATE_COUNT);\n        self.stamp_state.{field}.copy_from_slice(field);\n        rollback_values = remaining;\n"
        ));
    }
    out.push_str("        let mut rollback_flags = state.flags.as_slice();\n");
    out.push_str(
        "        let (field, remaining) = rollback_flags.split_at(Self::DDT_STATE_COUNT);\n",
    );
    out.push_str("        self.stamp_state.ddt_initialized.copy_from_slice(field);\n");
    out.push_str("        rollback_flags = remaining;\n");
    out.push_str(
        "        let (field, remaining) = rollback_flags.split_at(Self::IDT_STATE_COUNT);\n",
    );
    out.push_str("        self.stamp_state.idt_initialized.copy_from_slice(field);\n");
    out.push_str("        rollback_flags = remaining;\n");
    out.push_str(&extensions.rollback_restore_fields);
    out.push_str("        debug_assert!(rollback_values.is_empty());\n");
    out.push_str("        debug_assert!(rollback_flags.is_empty());\n");
    out.push_str("    }\n\n");

    out.push_str(
        "    pub(crate) fn capture_persistent_state(&self) -> GeneratedVerilogAPersistentState {\n",
    );
    out.push_str("        GeneratedVerilogAPersistentState {\n");
    out.push_str("            ddt_previous: self.stamp_state.ddt_previous.to_vec(),\n");
    out.push_str("            ddt_older: self.stamp_state.ddt_older.to_vec(),\n");
    out.push_str(
        "            ddt_derivative_previous: self.stamp_state.ddt_derivative_previous.to_vec(),\n",
    );
    out.push_str("            ddt_initialized: self.stamp_state.ddt_initialized.to_vec(),\n");
    out.push_str("            idt_previous: self.stamp_state.idt_previous.to_vec(),\n");
    out.push_str("            idt_initialized: self.stamp_state.idt_initialized.to_vec(),\n");
    out.push_str(&extensions.checkpoint_capture_fields);
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str("    pub(crate) fn validate_persistent_state_shape(&self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {\n");
    out.push_str("        if state.ddt_previous.len() != Self::DDT_STATE_COUNT || state.ddt_older.len() != Self::DDT_STATE_COUNT || state.ddt_derivative_previous.len() != Self::DDT_STATE_COUNT || state.ddt_initialized.len() != Self::DDT_STATE_COUNT {\n");
    out.push_str("            return Err(format!(\"generated ddt checkpoint shape mismatch: expected {}, found {} / {} / {} / {}\", Self::DDT_STATE_COUNT, state.ddt_previous.len(), state.ddt_older.len(), state.ddt_derivative_previous.len(), state.ddt_initialized.len()));\n");
    out.push_str("        }\n");
    out.push_str("        if state.idt_previous.len() != Self::IDT_STATE_COUNT || state.idt_initialized.len() != Self::IDT_STATE_COUNT {\n");
    out.push_str("            return Err(format!(\"generated idt checkpoint shape mismatch: expected {}, found {} / {}\", Self::IDT_STATE_COUNT, state.idt_previous.len(), state.idt_initialized.len()));\n");
    out.push_str("        }\n");
    out.push_str("        if state.ddt_previous.iter().chain(&state.ddt_older).chain(&state.ddt_derivative_previous).chain(&state.idt_previous).chain(&state.limiter_anchor).any(|value| !value.is_finite()) {\n");
    out.push_str("            return Err(\"generated Verilog-A checkpoint contains non-finite persistent state\".to_string());\n");
    out.push_str("        }\n");
    out.push_str(&extensions.checkpoint_shape_checks);
    out.push_str("        Ok(())\n");
    out.push_str("    }\n\n");
    out.push_str("    pub(crate) fn restore_persistent_state(&mut self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {\n");
    out.push_str("        self.validate_persistent_state_shape(state)?;\n");
    out.push_str("        self.stamp_state.ddt_previous.copy_from_slice(&state.ddt_previous);\n");
    out.push_str("        self.stamp_state.ddt_current.copy_from_slice(&state.ddt_previous);\n");
    out.push_str("        self.stamp_state.ddt_older.copy_from_slice(&state.ddt_older);\n");
    out.push_str(
        "        self.stamp_state.ddt_derivative_previous.copy_from_slice(&state.ddt_derivative_previous);\n",
    );
    out.push_str(
        "        self.stamp_state.ddt_derivative_current.copy_from_slice(&state.ddt_derivative_previous);\n",
    );
    out.push_str(
        "        self.stamp_state.ddt_initialized.copy_from_slice(&state.ddt_initialized);\n",
    );
    out.push_str("        self.stamp_state.idt_previous.copy_from_slice(&state.idt_previous);\n");
    out.push_str("        self.stamp_state.idt_current.copy_from_slice(&state.idt_previous);\n");
    out.push_str(
        "        self.stamp_state.idt_initialized.copy_from_slice(&state.idt_initialized);\n",
    );
    out.push_str(&extensions.checkpoint_restore_fields);
    out.push_str("        Ok(())\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn set_branch_indices(&mut self, branches: &[usize]) {\n");
    out.push_str("        assert_eq!(branches.len(), Self::BRANCH_COUNT, \"generated Verilog-A branch count mismatch\");\n");
    out.push_str("        self.branches.copy_from_slice(branches);\n");
    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {\n",
    );
    if artifact.mir.parameters.is_empty() {
        out.push_str("        let _ = value;\n");
        out.push_str(&format!(
            "        Err(format!(\"unknown parameter '{{}}' for generated Verilog-A model '{}'\", name))\n",
            artifact.mir.module_name
        ));
    } else {
        out.push_str("        let lower = name.to_ascii_lowercase();\n");
        out.push_str("        let Some(index) = parameter_index_for_name(lower.as_str()) else {\n");
        out.push_str(&format!(
            "            return Err(format!(\"unknown parameter '{{}}' for generated Verilog-A model '{}'\", name));\n",
            artifact.mir.module_name
        ));
        out.push_str("        };\n");
        out.push_str("        validate_parameter_scalar_metadata(index, value)?;\n");
        out.push_str("        let was_given = self.param_given[index];\n");
        out.push_str("        let value_changed = self.write_parameter_slot(index, value);\n");
        out.push_str("        self.finish_set_parameter(index, value_changed || !was_given);\n");
        out.push_str("        Ok(())\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    /// Validate the complete parameter vector after applying all instance overrides.\n",
    );
    out.push_str("    pub fn validate_parameters(&self) -> Result<(), String> {\n");
    if artifact.mir.parameters.is_empty() {
        out.push_str("        Ok(())\n");
    } else {
        out.push_str("        for index in 0..Self::PARAMETER_COUNT {\n");
        out.push_str("            let value = read_parameter_slot(self.params.as_ref(), index);\n");
        out.push_str(
            "            validate_parameter_metadata(self.params.as_ref(), index, value)?;\n",
        );
        out.push_str("        }\n");
        out.push_str("        Ok(())\n");
    }
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    fn write_parameter_slot(&mut self, index: usize, value: f64) -> bool {\n");
    out.push_str("        debug_assert!(index < Self::PARAMETER_COUNT, \"generated parameter index out of range\");\n");
    out.push_str("        let slot = &mut self.params.values[index];\n");
    out.push_str("        let changed = slot.to_bits() != value.to_bits();\n");
    out.push_str("        *slot = value;\n");
    out.push_str("        changed\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str(
        "    fn finish_set_parameter(&mut self, index: usize, invalidates_caches: bool) {\n",
    );
    out.push_str("        self.mark_param_given(index);\n");
    if !extensions.set_parameter_hook.is_empty() {
        out.push_str("        if invalidates_caches {\n");
        for line in extensions.set_parameter_hook.lines() {
            if !line.trim().is_empty() {
                out.push_str("            ");
                out.push_str(line.trim_end());
                out.push('\n');
            }
        }
        out.push_str("        }\n");
    } else {
        out.push_str("        let _ = invalidates_caches;\n");
    }
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    fn mark_param_given(&mut self, index: usize) {\n");
    out.push_str("        debug_assert!(index < Self::PARAMETER_COUNT, \"generated parameter index out of range\");\n");
    out.push_str("        self.param_given[index] = true;\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str(
        "    pub fn set_multiplicity(&mut self, multiplicity: f64) -> Result<(), String> {\n",
    );
    out.push_str("        if multiplicity.is_finite() && multiplicity > 0.0 {\n");
    out.push_str("            self.multiplicity = multiplicity;\n");
    out.push_str("            Ok(())\n");
    out.push_str("        } else {\n");
    out.push_str(
        "            Err(format!(\"instance multiplicity 'm' must be finite and > 0.0, got {}\", multiplicity))\n",
    );
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str(
        "    pub fn set_timepoint(&mut self, time: f64, timestep: f64, ddt_coefficients: GeneratedDdtCoefficients) {\n",
    );
    out.push_str("        self.time = time;\n");
    out.push_str("        self.timestep = timestep;\n");
    out.push_str("        self.ddt_coefficients = ddt_coefficients;\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn accept_timestep(&mut self) {\n");
    out.push_str("        let mut index = 0usize;\n");
    out.push_str("        while index < Self::DDT_STATE_COUNT {\n");
    out.push_str(
        "            self.stamp_state.ddt_older[index] = self.stamp_state.ddt_previous[index];\n",
    );
    out.push_str(
        "            self.stamp_state.ddt_previous[index] = self.stamp_state.ddt_current[index];\n",
    );
    out.push_str(
        "            self.stamp_state.ddt_derivative_previous[index] = self.stamp_state.ddt_derivative_current[index];\n",
    );
    out.push_str("            self.stamp_state.ddt_initialized[index] = true;\n");
    out.push_str("            index += 1;\n");
    out.push_str("        }\n");
    out.push_str("        let mut index = 0usize;\n");
    out.push_str("        while index < Self::IDT_STATE_COUNT {\n");
    out.push_str(
        "            self.stamp_state.idt_previous[index] = self.stamp_state.idt_current[index];\n",
    );
    out.push_str("            self.stamp_state.idt_initialized[index] = true;\n");
    out.push_str("            index += 1;\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    if ddt_state_count > 0 {
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {\n");
        out.push_str("        debug_assert!(slot < Self::DDT_STATE_COUNT, \"generated ddt state slot out of range\");\n");
        out.push_str("        let previous = if self.stamp_state.ddt_initialized[slot] {\n");
        out.push_str("            self.stamp_state.ddt_previous[slot]\n");
        out.push_str("        } else {\n");
        out.push_str("            value\n");
        out.push_str("        };\n");
        out.push_str("        let older = if self.stamp_state.ddt_initialized[slot] {\n");
        out.push_str("            self.stamp_state.ddt_older[slot]\n");
        out.push_str("        } else {\n");
        out.push_str("            value\n");
        out.push_str("        };\n");
        out.push_str("        self.stamp_state.ddt_current[slot] = value;\n");
        out.push_str("        if self.ddt_coefficients.active {\n");
        out.push_str("            let result = value * self.ddt_coefficients.derivative_scale\n");
        out.push_str("                - previous * self.ddt_coefficients.previous_value_scale\n");
        out.push_str("                - older * self.ddt_coefficients.older_value_scale\n");
        out.push_str("                - self.stamp_state.ddt_derivative_previous[slot] * self.ddt_coefficients.previous_derivative_scale;\n");
        out.push_str("            self.stamp_state.ddt_derivative_current[slot] = result;\n");
        out.push_str("            result\n");
        out.push_str("        } else {\n");
        out.push_str("            self.stamp_state.ddt_current[slot] = value;\n");
        out.push_str("            self.stamp_state.ddt_previous[slot] = value;\n");
        out.push_str("            self.stamp_state.ddt_older[slot] = value;\n");
        out.push_str("            self.stamp_state.ddt_derivative_current[slot] = 0.0;\n");
        out.push_str("            self.stamp_state.ddt_derivative_previous[slot] = 0.0;\n");
        out.push_str("            self.stamp_state.ddt_initialized[slot] = true;\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n\n");
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {\n");
        out.push_str("        if self.ddt_coefficients.active {\n");
        out.push_str("            derivative * self.ddt_coefficients.derivative_scale\n");
        out.push_str("        } else {\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
    }
    if idt_state_count > 0 {
        out.push_str("    #[inline]\n");
        out.push_str(
            "    pub(crate) fn eval_idt(&mut self, slot: usize, value: f64, ic: f64) -> f64 {\n",
        );
        out.push_str("        debug_assert!(slot < Self::IDT_STATE_COUNT, \"generated idt state slot out of range\");\n");
        out.push_str("        let previous = if self.stamp_state.idt_initialized[slot] {\n");
        out.push_str("            self.stamp_state.idt_previous[slot]\n");
        out.push_str("        } else {\n");
        out.push_str("            ic\n");
        out.push_str("        };\n");
        out.push_str("        let current = if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            previous + value * self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            ic\n");
        out.push_str("        };\n");
        out.push_str("        self.stamp_state.idt_current[slot] = current;\n");
        out.push_str("        if self.timestep.abs() <= Self::DDT_EPSILON {\n");
        out.push_str("            self.stamp_state.idt_previous[slot] = current;\n");
        out.push_str("            self.stamp_state.idt_initialized[slot] = true;\n");
        out.push_str("        }\n");
        out.push_str("        current\n");
        out.push_str("    }\n\n");
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn idt_jacobian(&self, derivative: f64) -> f64 {\n");
        out.push_str("        if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            derivative * self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
    }
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn limiter_converged(&self) -> bool {\n");
    out.push_str("        ");
    out.push_str(&extensions.limiter_converged_expr);
    out.push_str("\n    }\n");
    out.push_str(&extensions.impl_methods);
    out.push_str("}\n");
    Ok(out)
}

const CHECKPOINT_IDENTITY_PLACEHOLDER: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

fn hash_identity_field(hasher: &mut blake3::Hasher, field: &[u8]) {
    hasher.update(&(field.len() as u64).to_le_bytes());
    hasher.update(field);
}

pub(super) fn finalize_checkpoint_identity(
    device: &mut GeneratedRustDevice,
) -> Result<(), RustBackendError> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"rspice-generated-persistent-state-v2\0");
    hash_identity_field(
        &mut hasher,
        env!("RSPICE_VERILOGA_GENERATOR_SOURCE_DIGEST").as_bytes(),
    );
    hash_identity_field(&mut hasher, device.module_name.as_bytes());
    hash_identity_field(&mut hasher, device.public_model_name.as_bytes());
    hash_identity_field(&mut hasher, device.folder_name.as_bytes());
    hash_identity_field(&mut hasher, device.source_digest.as_bytes());

    let mut files = device.files.iter().collect::<Vec<_>>();
    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    for file in files {
        hash_identity_field(&mut hasher, file.relative_path.as_bytes());
        hash_identity_field(&mut hasher, file.contents.as_bytes());
    }
    let identity = hasher.finalize().to_hex().to_string();
    let declaration = format!(
        "pub const CHECKPOINT_MODEL_IDENTITY: &'static str = {CHECKPOINT_IDENTITY_PLACEHOLDER:?};"
    );
    let replacement = format!("pub const CHECKPOINT_MODEL_IDENTITY: &'static str = {identity:?};");
    let mut replacements = 0usize;
    for file in &mut device.files {
        if file.contents.contains(&declaration) {
            file.contents = file.contents.replacen(&declaration, &replacement, 1);
            replacements += 1;
        }
    }
    if replacements != 1 {
        return Err(RustBackendError::internal(
            device.source_digest.clone(),
            device.module_name.clone(),
            format!(
                "generated checkpoint identity placeholder count is {replacements}, expected 1"
            ),
        ));
    }
    Ok(())
}


fn emit_parameter_defaults(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let mut index = 0usize;
    let mut chunk_index = 0usize;
    while index < artifact.mir.parameters.len() {
        if artifact.mir.parameters[index].default.is_some() {
            let start = index;
            while index < artifact.mir.parameters.len()
                && artifact.mir.parameters[index].default.is_some()
            {
                index += 1;
            }
            emit_numeric_parameter_default_chunk(
                artifact,
                parameter_fields,
                start,
                index,
                chunk_index,
                out,
            )?;
            chunk_index += 1;
            continue;
        }

        let parameter = &artifact.mir.parameters[index];
        let field = &parameter_fields[parameter.name.as_str()];
        let default = parameter_default_rust_expr(artifact, parameter, parameter_fields)?;
        out.push_str("            {\n");
        out.push_str("                let params = &mut *ptr;\n");
        out.push_str(&format!("                params[{field}] = {default};\n"));
        emit_parameter_default_validation(parameter, field, "                ", out)?;
        out.push_str("            }\n");
        index += 1;
    }
    Ok(())
}

fn emit_numeric_parameter_default_chunk(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    start: usize,
    end: usize,
    chunk_index: usize,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let defaults = artifact.mir.parameters[start..end]
        .iter()
        .map(|parameter| parameter_default_rust_expr(artifact, parameter, parameter_fields))
        .collect::<Result<Vec<_>, _>>()?;
    let name = format!("DEFAULTS_{chunk_index}");
    emit_f64_const_array(&name, &defaults, out);
    out.push_str(&format!(
        "            std::ptr::copy_nonoverlapping({name}.as_ptr(), (*ptr).values.as_mut_ptr().add({start}), {});\n",
        defaults.len()
    ));
    Ok(())
}

fn emit_f64_const_array(name: &str, values: &[String], out: &mut String) {
    const VALUES_PER_LINE: usize = 8;
    out.push_str(&format!(
        "            const {name}: [f64; {}] = [\n",
        values.len()
    ));
    for chunk in values.chunks(VALUES_PER_LINE) {
        out.push_str("                ");
        out.push_str(&chunk.join(", "));
        out.push_str(",\n");
    }
    out.push_str("            ];\n");
}

fn emit_parameter_default_validation(
    parameter: &MirParameterSlot,
    field: &str,
    indent: &str,
    out: &mut String,
) -> Result<(), RustBackendError> {
    if parameter_default_requires_runtime_validation(parameter) {
        let validation = parameter_validation_call(
            parameter.name.as_str(),
            &format!("params[{field}]"),
            parameter.value_type,
            parameter.range.as_ref(),
        )?;
        out.push_str(&format!(
            "{indent}{validation}.expect(\"generated Verilog-A parameter default must satisfy declared range\");\n"
        ));
    }
    Ok(())
}

fn emit_parameter_metadata(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let parameter_count = artifact.mir.parameters.len();
    let mut parameters_by_index = vec![None; parameter_count];
    for parameter in &artifact.mir.parameters {
        let index = usize::from(parameter.id);
        if index >= parameter_count {
            return Err(unsupported(
                artifact,
                format!(
                    "parameter '{}' has out-of-range generated id",
                    parameter.name
                ),
            ));
        }
        parameters_by_index[index] = Some(parameter);
    }
    let mut parameter_indices = HashMap::with_capacity(parameter_count);
    for parameter in &artifact.mir.parameters {
        if parameter_indices
            .insert(parameter.name.as_str(), usize::from(parameter.id))
            .is_some()
        {
            return Err(unsupported(
                artifact,
                format!("duplicate canonical parameter name '{}'", parameter.name),
            ));
        }
    }

    let lookup_count = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| 1 + parameter.aliases.len())
        .sum::<usize>();
    out.push_str(&format!(
        "const PARAMETER_NAME_LOOKUP: [(&str, usize); {lookup_count}] = [\n"
    ));
    emit_parameter_name_lookup_entries(artifact, out);
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_MIN_REFERENCES: [Option<usize>; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        16,
        |parameter| {
            parameter_reference_option_literal(
                parameter.name.as_str(),
                parameter
                    .range
                    .as_ref()
                    .and_then(|range| range.min_parameter.as_deref()),
                &parameter_indices,
            )
        },
        out,
    )?;
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_MAX_REFERENCES: [Option<usize>; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        16,
        |parameter| {
            parameter_reference_option_literal(
                parameter.name.as_str(),
                parameter
                    .range
                    .as_ref()
                    .and_then(|range| range.max_parameter.as_deref()),
                &parameter_indices,
            )
        },
        out,
    )?;
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_DISPLAY_NAMES: [&str; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        16,
        |parameter| Ok(format!("{:?}", parameter.name.as_str())),
        out,
    )?;
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        8,
        |parameter| {
            parameter_excluded_references_literal(
                parameter.name.as_str(),
                parameter
                    .range
                    .as_ref()
                    .map(|range| range.exclude_parameters.as_slice())
                    .unwrap_or_default(),
                &parameter_indices,
            )
        },
        out,
    )?;
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_INTEGER_FLAGS: [bool; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        32,
        |parameter| Ok((parameter.value_type == CanonicalValueType::Integer).to_string()),
        out,
    )?;
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        8,
        |parameter| {
            Ok(parameter_bound_option_literal(
                parameter.range.as_ref().and_then(|range| range.min),
            ))
        },
        out,
    )?;
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        8,
        |parameter| {
            Ok(parameter_bound_option_literal(
                parameter.range.as_ref().and_then(|range| range.max),
            ))
        },
        out,
    )?;
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_RANGE_FLAGS: [u8; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        32,
        |parameter| Ok(parameter_range_flags_literal(parameter.range.as_ref())),
        out,
    )?;
    out.push_str("];\n\n");

    out.push_str(&format!(
        "const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; {parameter_count}] = [\n"
    ));
    emit_chunked_parameter_metadata_array(
        &parameters_by_index,
        8,
        |parameter| {
            parameter_excluded_bounds_literal(parameter.name.as_str(), parameter.range.as_ref())
        },
        out,
    )?;
    out.push_str("];\n\n");

    emit_computed_parameter_bound_function(
        artifact,
        parameter_fields,
        true,
        "parameter_computed_min_bound",
        out,
    )?;
    emit_computed_parameter_bound_function(
        artifact,
        parameter_fields,
        false,
        "parameter_computed_max_bound",
        out,
    )?;
    emit_computed_parameter_exclusion_function(artifact, parameter_fields, out)?;

    out.push_str("fn parameter_index_for_name(name: &str) -> Option<usize> {\n");
    out.push_str("    PARAMETER_NAME_LOOKUP\n");
    out.push_str("        .iter()\n");
    out.push_str(
        "        .find_map(|(candidate, index)| (*candidate == name).then_some(*index))\n",
    );
    out.push_str("}\n\n");

    Ok(())
}


fn generate_shared_parameter_validator() -> String {
    r###"
#[derive(Copy, Clone)]
struct ParameterBound {
    value: f64,
    label: &'static str,
}

const PARAMETER_MIN_EXCLUSIVE_FLAG: u8 = 1;
const PARAMETER_MAX_EXCLUSIVE_FLAG: u8 = 2;

#[inline]
fn read_parameter_slot(parameters: &Parameters, index: usize) -> f64 {
    debug_assert!(index < PARAMETER_DISPLAY_NAMES.len(), "generated parameter index out of range");
    parameters.values[index]
}

fn validate_parameter_scalar_metadata(index: usize, value: f64) -> Result<(), String> {
    let Some(&name) = PARAMETER_DISPLAY_NAMES.get(index) else {
        return Err(format!("generated parameter index {} is out of range", index));
    };
    let flags = PARAMETER_RANGE_FLAGS[index];
    validate_finite_parameter(name, value)?;
    if PARAMETER_INTEGER_FLAGS[index] && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if PARAMETER_INTEGER_FLAGS[index] && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    validate_parameter_bounds(
        name,
        value,
        flags,
        PARAMETER_MIN_BOUNDS[index],
        PARAMETER_MAX_BOUNDS[index],
        PARAMETER_EXCLUDED_BOUNDS[index],
    )
}

fn validate_parameter_metadata(
    parameters: &Parameters,
    index: usize,
    value: f64,
) -> Result<(), String> {
    validate_parameter_scalar_metadata(index, value)?;
    let name = PARAMETER_DISPLAY_NAMES[index];
    let flags = PARAMETER_RANGE_FLAGS[index];
    let computed_min = parameter_computed_min_bound(parameters, index)?;
    let lower_source_count = usize::from(PARAMETER_MIN_BOUNDS[index].is_some())
        + usize::from(PARAMETER_MIN_REFERENCES[index].is_some())
        + usize::from(computed_min.is_some());
    if lower_source_count > 1 {
        return Err(format!("parameter '{}' has conflicting lower-bound sources", name));
    }
    let min = match PARAMETER_MIN_REFERENCES[index] {
        Some(reference) => Some(parameter_bound_from_reference(parameters, reference)?),
        None => computed_min.or(PARAMETER_MIN_BOUNDS[index]),
    };
    let computed_max = parameter_computed_max_bound(parameters, index)?;
    let upper_source_count = usize::from(PARAMETER_MAX_BOUNDS[index].is_some())
        + usize::from(PARAMETER_MAX_REFERENCES[index].is_some())
        + usize::from(computed_max.is_some());
    if upper_source_count > 1 {
        return Err(format!("parameter '{}' has conflicting upper-bound sources", name));
    }
    let max = match PARAMETER_MAX_REFERENCES[index] {
        Some(reference) => Some(parameter_bound_from_reference(parameters, reference)?),
        None => computed_max.or(PARAMETER_MAX_BOUNDS[index]),
    };
    if let (Some(min), Some(max)) = (min, max) {
        let empty = min.value > max.value
            || (min.value == max.value
                && flags & (PARAMETER_MIN_EXCLUSIVE_FLAG | PARAMETER_MAX_EXCLUSIVE_FLAG) != 0);
        if empty {
            return Err(format!(
                "parameter '{}' has an empty range: lower bound {}={} exceeds upper bound {}={}",
                name, min.label, min.value, max.label, max.value
            ));
        }
    }
    validate_parameter_bounds(name, value, flags, min, max, PARAMETER_EXCLUDED_BOUNDS[index])?;
    for &reference in PARAMETER_EXCLUDED_REFERENCES[index] {
        let excluded = parameter_bound_from_reference(parameters, reference)?;
        if value == excluded.value {
            return Err(format!(
                "parameter '{}' must not equal {}={}, got {}",
                name, excluded.label, excluded.value, value
            ));
        }
    }
    validate_parameter_computed_exclusions(parameters, index, value)?;
    Ok(())
}

fn parameter_bound_from_reference(
    parameters: &Parameters,
    index: usize,
) -> Result<ParameterBound, String> {
    let Some(&name) = PARAMETER_DISPLAY_NAMES.get(index) else {
        return Err(format!("generated parameter range reference {} is out of range", index));
    };
    let value = read_parameter_slot(parameters, index);
    validate_finite_parameter(name, value)?;
    Ok(ParameterBound { value, label: name })
}

fn validate_parameter_bounds(
    name: &str,
    value: f64,
    flags: u8,
    min: Option<ParameterBound>,
    max: Option<ParameterBound>,
    excluded: &[ParameterBound],
) -> Result<(), String> {
    if let Some(min) = min {
        if flags & PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!("parameter '{}' must be > {}, got {}", name, min.label, value));
            }
        } else if value < min.value {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, min.label, value));
        }
    }
    if let Some(max) = max {
        if flags & PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!("parameter '{}' must be < {}, got {}", name, max.label, value));
            }
        } else if value > max.value {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, max.label, value));
        }
    }
    for excluded in excluded {
        if value == excluded.value {
            return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
        }
    }
    Ok(())
}

fn validate_finite_parameter(name: &str, value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter '{}' must be finite, got {}", name, value));
    }
    Ok(())
}

fn validate_parameter(
    name: &str,
    value: f64,
    integer: bool,
    min: Option<(f64, &str)>,
    min_exclusive: bool,
    max: Option<(f64, &str)>,
    max_exclusive: bool,
    excluded: &[(f64, &str)],
) -> Result<(), String> {
    validate_finite_parameter(name, value)?;
    if integer && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if integer && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    if let Some((min, label)) = min {
        if min_exclusive {
            if value <= min {
                return Err(format!("parameter '{}' must be > {}, got {}", name, label, value));
            }
        } else if value < min {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, label, value));
        }
    }
    if let Some((max, label)) = max {
        if max_exclusive {
            if value >= max {
                return Err(format!("parameter '{}' must be < {}, got {}", name, label, value));
            }
        } else if value > max {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, label, value));
        }
    }
    for (excluded, label) in excluded {
        if value == *excluded {
            return Err(format!("parameter '{}' must not equal {}, got {}", name, label, value));
        }
    }
    Ok(())
}
"###
        .trim_start()
        .to_string()
}

fn emit_parameter_name_lookup_entries(artifact: &CanonicalIrArtifact, out: &mut String) {
    const ENTRIES_PER_LINE: usize = 16;
    let mut entries = Vec::new();
    for parameter in &artifact.mir.parameters {
        entries.push(parameter_name_lookup_entry(
            parameter.name.as_str(),
            usize::from(parameter.id),
        ));
        for alias in &parameter.aliases {
            entries.push(parameter_name_lookup_entry(
                alias.as_str(),
                usize::from(parameter.id),
            ));
        }
    }

    for chunk in entries.chunks(ENTRIES_PER_LINE) {
        out.push_str("    ");
        out.push_str(&chunk.join(", "));
        out.push_str(",\n");
    }
}

fn emit_computed_parameter_bound_function(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    lower: bool,
    function_name: &str,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let label = if lower {
        "computed lower-bound expression"
    } else {
        "computed upper-bound expression"
    };
    out.push_str(&format!(
        "fn {function_name}(parameters: &Parameters, index: usize) -> Result<Option<ParameterBound>, String> {{\n"
    ));
    out.push_str("    let params = parameters;\n");
    out.push_str("    let bound: Option<ParameterBound> = match index {\n");
    for parameter in &artifact.mir.parameters {
        let expression = parameter.range.as_ref().and_then(|range| {
            if lower {
                range.min_expression.as_ref()
            } else {
                range.max_expression.as_ref()
            }
        });
        let Some(expression) = expression else {
            continue;
        };
        let value = lower_parameter_default_expr(artifact, expression.id, parameter_fields)?;
        out.push_str(&format!(
            "        {} => Some(ParameterBound {{ value: {value}, label: {label:?} }}),\n",
            usize::from(parameter.id)
        ));
    }
    out.push_str("        _ => None,\n");
    out.push_str("    };\n");
    out.push_str("    if let Some(bound) = bound {\n");
    out.push_str("        validate_finite_parameter(bound.label, bound.value)?;\n");
    out.push_str("    }\n");
    out.push_str("    Ok(bound)\n");
    out.push_str("}\n\n");
    Ok(())
}

fn emit_computed_parameter_exclusion_function(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    out.push_str("fn validate_parameter_computed_exclusions(\n");
    out.push_str("    parameters: &Parameters,\n");
    out.push_str("    index: usize,\n");
    out.push_str("    value: f64,\n");
    out.push_str(") -> Result<(), String> {\n");
    out.push_str("    let params = parameters;\n");
    out.push_str("    match index {\n");
    for parameter in &artifact.mir.parameters {
        let expressions = parameter
            .range
            .as_ref()
            .map(|range| range.exclude_expressions.as_slice())
            .unwrap_or_default();
        if expressions.is_empty() {
            continue;
        }
        out.push_str(&format!("        {} => {{\n", usize::from(parameter.id)));
        for (expression_index, expression) in expressions.iter().enumerate() {
            let excluded = lower_parameter_default_expr(artifact, expression.id, parameter_fields)?;
            let label = format!("computed exclusion expression {expression_index}");
            out.push_str(&format!("            let excluded = {excluded};\n"));
            out.push_str(&format!(
                "            validate_finite_parameter({label:?}, excluded)?;\n"
            ));
            out.push_str("            if value == excluded {\n");
            out.push_str(&format!(
                "                return Err(format!(\"parameter '{{}}' must not equal {label}={{}}, got {{}}\", PARAMETER_DISPLAY_NAMES[index], excluded, value));\n"
            ));
            out.push_str("            }\n");
        }
        out.push_str("        }\n");
    }
    out.push_str("        _ => {}\n");
    out.push_str("    }\n");
    out.push_str("    Ok(())\n");
    out.push_str("}\n\n");
    Ok(())
}

fn parameter_name_lookup_entry(name: &str, index: usize) -> String {
    format!("({:?}, {index})", name.to_ascii_lowercase())
}

fn emit_chunked_parameter_metadata_array<F>(
    parameters_by_index: &[Option<&MirParameterSlot>],
    entries_per_line: usize,
    mut emit_entry: F,
    out: &mut String,
) -> Result<(), RustBackendError>
where
    F: FnMut(&MirParameterSlot) -> Result<String, RustBackendError>,
{
    for chunk in parameters_by_index.chunks(entries_per_line) {
        let mut entries = Vec::with_capacity(chunk.len());
        for parameter in chunk {
            let Some(parameter) = parameter else {
                return Err(RustBackendError::unsupported(
                    "<generated>",
                    "<parameter metadata>",
                    "missing generated parameter id",
                ));
            };
            entries.push(emit_entry(parameter)?);
        }
        out.push_str("    ");
        out.push_str(&entries.join(", "));
        out.push_str(",\n");
    }
    Ok(())
}

fn parameter_bound_option_literal(value: Option<f64>) -> String {
    value
        .filter(|value| value.is_finite())
        .map(|value| format!("Some({})", parameter_bound_literal(value)))
        .unwrap_or_else(|| "None".to_string())
}

fn parameter_reference_option_literal(
    parameter_name: &str,
    reference: Option<&str>,
    parameter_indices: &HashMap<&str, usize>,
) -> Result<String, RustBackendError> {
    let Some(reference) = reference else {
        return Ok("None".to_string());
    };
    let index = parameter_indices.get(reference).copied().ok_or_else(|| {
        RustBackendError::unsupported(
            "<generated>",
            parameter_name,
            format!("range references unknown parameter '{reference}'"),
        )
    })?;
    Ok(format!("Some({index})"))
}

fn parameter_excluded_references_literal(
    parameter_name: &str,
    references: &[SmolStr],
    parameter_indices: &HashMap<&str, usize>,
) -> Result<String, RustBackendError> {
    let indices = references
        .iter()
        .map(|reference| {
            parameter_indices
                .get(reference.as_str())
                .copied()
                .ok_or_else(|| {
                    RustBackendError::unsupported(
                        "<generated>",
                        parameter_name,
                        format!("range excludes unknown parameter '{reference}'"),
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if indices.is_empty() {
        Ok("&[]".to_string())
    } else {
        Ok(format!(
            "&[{}]",
            indices
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

fn parameter_excluded_bounds_literal(
    parameter_name: &str,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<String, RustBackendError> {
    let Some(range) = range else {
        return Ok("&[]".to_string());
    };
    if range.exclude.is_empty() {
        return Ok("&[]".to_string());
    }

    let mut excluded = Vec::with_capacity(range.exclude.len());
    for value in &range.exclude {
        if value.is_finite() {
            excluded.push(parameter_bound_literal(*value));
        } else {
            return Err(RustBackendError::unsupported(
                "<generated>",
                parameter_name,
                "non-finite parameter exclude constraint",
            ));
        }
    }
    Ok(format!("&[{}]", excluded.join(", ")))
}

fn parameter_range_flags_literal(range: Option<&crate::canonical_ir::HirParamRange>) -> String {
    let mut flags = 0u8;
    if range.is_some_and(|range| range.min_exclusive) {
        flags |= 1;
    }
    if range.is_some_and(|range| range.max_exclusive) {
        flags |= 2;
    }
    flags.to_string()
}

fn parameter_bound_literal(value: f64) -> String {
    let label = format_f64(value);
    format!("ParameterBound {{ value: {label}, label: {label:?} }}")
}

fn parameter_default_requires_runtime_validation(parameter: &MirParameterSlot) -> bool {
    parameter.default.is_none()
}

fn parameter_default_rust_expr(
    artifact: &CanonicalIrArtifact,
    parameter: &MirParameterSlot,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    if let Some(default) = parameter.default {
        validate_parameter_value_for_codegen(
            artifact,
            parameter.name.as_str(),
            default,
            parameter.value_type,
            parameter.range.as_ref(),
        )?;
        return Ok(format_f64(default));
    }

    if let Some(default_expr) = &parameter.default_expr {
        return lower_parameter_default_expr(artifact, default_expr.id, parameter_fields);
    }

    Err(unsupported(
        artifact,
        format!(
            "parameter '{}' default that does not fold to a constant",
            parameter.name
        ),
    ))
}


fn parameter_validation_call(
    parameter_name: &str,
    value_expr: &str,
    value_type: CanonicalValueType,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<String, RustBackendError> {
    let integer = value_type == CanonicalValueType::Integer;
    if !integer && !parameter_range_has_runtime_constraints(parameter_name, range)? {
        return Ok(format!(
            "validate_finite_parameter({parameter_name:?}, {value_expr})"
        ));
    }

    let min = range
        .and_then(|range| range.min.filter(|value| value.is_finite()))
        .map(range_bound_arg);
    let max = range
        .and_then(|range| range.max.filter(|value| value.is_finite()))
        .map(range_bound_arg);
    let min_exclusive = range.is_some_and(|range| range.min_exclusive);
    let max_exclusive = range.is_some_and(|range| range.max_exclusive);
    let exclude = if let Some(range) = range {
        let mut excluded = Vec::with_capacity(range.exclude.len());
        for value in &range.exclude {
            if !value.is_finite() {
                return Err(RustBackendError::unsupported(
                    "<generated>",
                    parameter_name,
                    "non-finite parameter exclude constraint",
                ));
            }
            excluded.push(range_excluded_arg(*value));
        }
        format!("&[{}]", excluded.join(", "))
    } else {
        "&[]".to_string()
    };

    Ok(format!(
        "validate_parameter({parameter_name:?}, {value_expr}, {integer}, {}, {min_exclusive}, {}, {max_exclusive}, {exclude})",
        min.unwrap_or_else(|| "None".to_string()),
        max.unwrap_or_else(|| "None".to_string())
    ))
}


fn lower_parameter_default_expr(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(expr))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("parameter default expression {expr} is outside MIR arena"),
            )
        })?;

    match &expression.kind {
        HirExprKind::Number { value, .. } => Ok(format_f64(*value)),
        HirExprKind::Identifier { name } => parameter_fields
            .get(name.as_str())
            .map(|field| format!("params[{field}]"))
            .ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("parameter default references non-parameter identifier '{name}'"),
                )
            }),
        HirExprKind::Unary { op, operand } => {
            let operand = lower_parameter_default_expr(artifact, *operand, parameter_fields)?;
            match op.as_str() {
                "Neg" => Ok(format!("(-{operand})")),
                "Pos" => Ok(format!("({operand})")),
                "Not" => Ok(format!(
                    "if {} {{ 1.0 }} else {{ 0.0 }}",
                    negate_condition(&format!("({operand} != 0.0)"))
                )),
                _ => Err(unsupported(
                    artifact,
                    format!("parameter default unary operator {op}"),
                )),
            }
        }
        HirExprKind::Binary { op, left, right } => {
            if let Some(operator) = comparison_operator(op.as_str()) {
                let left = lower_parameter_default_expr(artifact, *left, parameter_fields)?;
                let right = lower_parameter_default_expr(artifact, *right, parameter_fields)?;
                return Ok(format!(
                    "if ({left} {operator} {right}) {{ 1.0 }} else {{ 0.0 }}"
                ));
            }
            if op.as_str() == "And" || op.as_str() == "Or" {
                let left_condition =
                    lower_parameter_default_condition(artifact, *left, parameter_fields)?;
                let right_condition =
                    lower_parameter_default_condition(artifact, *right, parameter_fields)?;
                let operator = if op.as_str() == "And" { "&&" } else { "||" };
                return Ok(format!(
                    "if ({left_condition} {operator} {right_condition}) {{ 1.0 }} else {{ 0.0 }}"
                ));
            }
            let left = lower_parameter_default_expr(artifact, *left, parameter_fields)?;
            let right = lower_parameter_default_expr(artifact, *right, parameter_fields)?;
            let operator = match op.as_str() {
                "Add" => "+",
                "Sub" => "-",
                "Mul" => "*",
                "Div" => "/",
                "Mod" => "%",
                _ => {
                    return Err(unsupported(
                        artifact,
                        format!("parameter default binary operator {op}"),
                    ));
                }
            };
            Ok(format!("({left} {operator} {right})"))
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            let condition =
                lower_parameter_default_condition(artifact, *condition, parameter_fields)?;
            let then_expr = lower_parameter_default_expr(artifact, *then_expr, parameter_fields)?;
            let else_expr = lower_parameter_default_expr(artifact, *else_expr, parameter_fields)?;
            Ok(format!(
                "if {condition} {{ {then_expr} }} else {{ {else_expr} }}"
            ))
        }
        HirExprKind::SystemFunction { name, args }
            if name.as_str().eq_ignore_ascii_case("$simparam")
                || name.as_str().eq_ignore_ascii_case("simparam") =>
        {
            let Some(fallback) = args.get(1) else {
                return Err(unsupported(
                    artifact,
                    "parameter default $simparam without explicit fallback",
                ));
            };
            lower_parameter_default_expr(artifact, *fallback, parameter_fields)
        }
        other => Err(unsupported(
            artifact,
            format!("parameter default expression kind {other:?}"),
        )),
    }
}


fn validate_parameter_value_for_codegen(
    artifact: &CanonicalIrArtifact,
    parameter_name: &str,
    value: f64,
    value_type: CanonicalValueType,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<(), RustBackendError> {
    if !value.is_finite() {
        return Err(unsupported(
            artifact,
            format!("non-finite default for parameter '{parameter_name}'"),
        ));
    }
    if value_type == CanonicalValueType::Integer && value.fract() != 0.0 {
        return Err(unsupported(
            artifact,
            format!("default for integer parameter '{parameter_name}' is fractional"),
        ));
    }
    if value_type == CanonicalValueType::Integer
        && (value < i32::MIN as f64 || value > i32::MAX as f64)
    {
        return Err(unsupported(
            artifact,
            format!("default for integer parameter '{parameter_name}' is outside i32 range"),
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


fn parameter_range_has_runtime_constraints(
    parameter_name: &str,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<bool, RustBackendError> {
    let Some(range) = range else {
        return Ok(false);
    };
    if range.exclude.iter().any(|value| !value.is_finite()) {
        return Err(RustBackendError::unsupported(
            "<generated>",
            parameter_name,
            "non-finite parameter exclude constraint",
        ));
    }
    Ok(range.min.is_some_and(|value| value.is_finite())
        || range.max.is_some_and(|value| value.is_finite())
        || range.min_parameter.is_some()
        || range.max_parameter.is_some()
        || range.min_expression.is_some()
        || range.max_expression.is_some()
        || !range.exclude.is_empty()
        || !range.exclude_parameters.is_empty()
        || !range.exclude_expressions.is_empty())
}

fn range_bound_arg(value: f64) -> String {
    let label = format_f64(value);
    format!("Some(({}, {:?}))", label, label)
}

fn range_excluded_arg(value: f64) -> String {
    let label = format_f64(value);
    format!("({}, {:?})", label, label)
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


fn lower_parameter_default_condition(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(expr))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("parameter default condition {expr} is outside MIR arena"),
            )
        })?;

    match &expression.kind {
        HirExprKind::Binary { op, left, right } if comparison_operator(op.as_str()).is_some() => {
            let operator = comparison_operator(op.as_str()).expect("checked above");
            let left = lower_parameter_default_expr(artifact, *left, parameter_fields)?;
            let right = lower_parameter_default_expr(artifact, *right, parameter_fields)?;
            Ok(format!("({left} {operator} {right})"))
        }
        HirExprKind::Binary { op, left, right } if op.as_str() == "And" || op.as_str() == "Or" => {
            let left = lower_parameter_default_condition(artifact, *left, parameter_fields)?;
            let right = lower_parameter_default_condition(artifact, *right, parameter_fields)?;
            let operator = if op.as_str() == "And" { "&&" } else { "||" };
            Ok(format!("({left} {operator} {right})"))
        }
        HirExprKind::Unary { op, operand } if op.as_str() == "Not" => {
            let operand = lower_parameter_default_condition(artifact, *operand, parameter_fields)?;
            Ok(negate_condition(&operand))
        }
        _ => {
            let value = lower_parameter_default_expr(artifact, expr, parameter_fields)?;
            Ok(format!("({value} != 0.0)"))
        }
    }
}


fn negate_condition(condition: &str) -> String {
    let condition = condition.trim();
    if let Some(inner) = condition
        .strip_prefix("(!")
        .and_then(|inner| inner.strip_suffix(')'))
    {
        return inner.to_string();
    }
    if let Some(inner) = condition
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
    {
        if let Some(value) = inner.strip_suffix(" != 0.0") {
            return format!("({value} == 0.0)");
        }
        if let Some(value) = inner.strip_suffix(" == 0.0") {
            return format!("({value} != 0.0)");
        }
    }
    format!("(!{condition})")
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

