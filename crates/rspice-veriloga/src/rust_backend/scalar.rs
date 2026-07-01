use std::collections::{HashMap, HashSet};

use crate::canonical_ir::opt::LIMEXP_MAX;
use crate::canonical_ir::{
    BranchId, CanonicalIrArtifact, CanonicalValueType, DerivativeLane, DerivativeLaneKind,
    EquationId, ExprId, HirAnalogOperator, HirExprKind, HirStatement, InvalidationClass,
    MirEquation, MirEquationKind, NodeId, OptBinaryOp, OptOp, OptUnaryOp, OptValue, OptValueKind,
    OptValueType, ValueId,
};

use super::expr::{DdtSlots, LoweredVariable, analysis_predicate_expr, parameter_field_names};
use super::{GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames};
use super::{RustTranspileOptions, device};

const SPARSE_STAMP_DERIVATIVE_THRESHOLD: usize = 10;

pub(super) struct ScalarStaticCache {
    instance_values: Vec<ValueId>,
    temperature_values: Vec<ValueId>,
    set: HashSet<ValueId>,
}

impl ScalarStaticCache {
    fn from_artifact(artifact: &CanonicalIrArtifact) -> Self {
        let instance_values = scheduled_values(artifact, InvalidationClass::InstanceStatic, None);
        let temperature_values =
            scheduled_values(artifact, InvalidationClass::TemperatureStatic, None);
        let set = instance_values
            .iter()
            .chain(temperature_values.iter())
            .copied()
            .collect();
        Self {
            instance_values,
            temperature_values,
            set,
        }
    }

    pub(super) fn from_roots(
        artifact: &CanonicalIrArtifact,
        roots: &HashMap<EquationId, ValueId>,
    ) -> Result<Self, RustBackendError> {
        let empty_cache = Self {
            instance_values: Vec::new(),
            temperature_values: Vec::new(),
            set: HashSet::new(),
        };
        let live = collect_stamp_live_values(artifact, roots, &empty_cache)?;
        let instance_values =
            scheduled_values(artifact, InvalidationClass::InstanceStatic, Some(&live));
        let temperature_values =
            scheduled_values(artifact, InvalidationClass::TemperatureStatic, Some(&live));
        let set = instance_values
            .iter()
            .chain(temperature_values.iter())
            .copied()
            .collect();
        Ok(Self {
            instance_values,
            temperature_values,
            set,
        })
    }

    fn contains(&self, value: ValueId) -> bool {
        self.set.contains(&value)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.instance_values.is_empty() && self.temperature_values.is_empty()
    }

    pub(super) fn has_temperature_values(&self) -> bool {
        !self.temperature_values.is_empty()
    }
}

fn scheduled_values(
    artifact: &CanonicalIrArtifact,
    invalidation: InvalidationClass,
    live: Option<&HashSet<ValueId>>,
) -> Vec<ValueId> {
    artifact
        .opt
        .schedules
        .iter()
        .filter(|schedule| schedule.invalidation == invalidation)
        .flat_map(|schedule| schedule.ops.iter())
        .filter_map(|op| match op {
            OptOp::ComputeValue { value } if live.map_or(true, |live| live.contains(value)) => {
                Some(*value)
            }
            OptOp::ComputeValue { .. } | OptOp::EvaluateEquation { .. } => None,
        })
        .collect()
}

struct ValueEmitContext<'a> {
    cached_values: &'a HashSet<ValueId>,
    use_cached_fields: bool,
    inline_uncached_constants: bool,
    limexp_max_expr: String,
    temperature_expr: String,
    thermal_voltage_expr: String,
}

struct ScalarDerivatives {
    nodes: Vec<(u32, ValueId)>,
    branches: Vec<(u32, ValueId)>,
}

#[derive(Debug, Clone, Copy)]
enum ScalarTransientOperator {
    Ddt {
        operator: ExprId,
    },
    Idt {
        operator: ExprId,
        ic: Option<ExprId>,
    },
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
    let ddt_slots = device::collect_ddt_slots(artifact)?;
    let potential_branch_count = artifact.mir.branch_unknowns.len();
    let stamp = generate_stamp_file(
        artifact,
        options,
        &parameter_fields,
        &static_cache,
        &ddt_slots,
    )?;
    let state = if static_cache.is_empty() {
        device::generate_state_file(
            artifact,
            options,
            &parameter_fields,
            ddt_slots.len(),
            ddt_slots.idt_len(),
            potential_branch_count,
            device::StateScratchUsage::default(),
        )?
    } else {
        let extensions = scalar_state_extensions(artifact, &parameter_fields, &static_cache)?;
        device::generate_state_file_with_extensions(
            artifact,
            options,
            &parameter_fields,
            ddt_slots.len(),
            ddt_slots.idt_len(),
            potential_branch_count,
            device::StateScratchUsage::default(),
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
    ddt_slots: &DdtSlots,
) -> Result<String, RustBackendError> {
    let roots = scalar_equation_roots(artifact)?;
    let stamp_live = collect_stamp_live_values(artifact, &roots, static_cache)?;
    let has_idt_slots = ddt_slots.idt_len() > 0;
    let stamp_needs_params = has_idt_slots
        || artifact.opt.values.iter().any(|value| {
            stamp_live.contains(&value.id) && matches!(value.kind, OptValueKind::Parameter { .. })
        });
    let stamp_needs_param_given = has_idt_slots
        || artifact.opt.values.iter().any(|value| {
            stamp_live.contains(&value.id) && matches!(value.kind, OptValueKind::ParamGiven { .. })
        });
    let stamp_needs_branches = artifact.opt.values.iter().any(|value| {
        stamp_live.contains(&value.id) && matches!(value.kind, OptValueKind::BranchFlow { .. })
    });
    let mut out = String::new();
    out.push_str("#![allow(dead_code, unused_imports, unused_parens, unused_variables)]\n\n");
    out.push_str("use super::state::Instance;\n");
    out.push_str(&format!(
        "use {}::{{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};\n\n",
        options.runtime_path
    ));
    if scalar_model_uses_limexp(artifact) {
        out.push_str(&format!(
            "const LIMEXP_MAX: f64 = {};\n\n",
            format_f64(LIMEXP_MAX)
        ));
    }
    if ddt_slots.len() > 0 || has_idt_slots {
        emit_transient_state_helpers(ddt_slots, &mut out);
    }
    out.push_str("impl Instance {\n");
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
    );
    out.push_str("        let nodes = self.nodes;\n");
    if stamp_needs_branches {
        out.push_str("        let branches = self.branches;\n");
    }
    if static_cache.has_temperature_values() {
        out.push_str(
            "        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());\n",
        );
    }
    if stamp_needs_params {
        out.push_str("        let p = &(*self.params);\n");
    }
    if stamp_needs_param_given {
        out.push_str("        let param_given = self.param_given.as_ref();\n");
    }
    out.push_str("        let multiplicity = self.multiplicity;\n");
    if ddt_slots.len() > 0 || has_idt_slots {
        out.push_str("        let timestep = self.timestep;\n");
    }
    if ddt_slots.len() > 0 {
        out.push_str("        let ddt_state_current = self.ddt_state_current.as_mut();\n");
        out.push_str("        let ddt_state_previous = self.ddt_state_previous.as_mut();\n");
        out.push_str("        let ddt_state_older = self.ddt_state_older.as_mut();\n");
        out.push_str("        let ddt_state_initialized = self.ddt_state_initialized.as_mut();\n");
        out.push_str(
            "        let ddt_derivative_current = self.ddt_derivative_current.as_mut();\n",
        );
        out.push_str(
            "        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();\n",
        );
    }
    if has_idt_slots {
        out.push_str("        let idt_state_current = self.idt_state_current.as_mut();\n");
        out.push_str("        let idt_state_previous = self.idt_state_previous.as_mut();\n");
        out.push_str("        let idt_state_initialized = self.idt_state_initialized.as_mut();\n");
    }
    if ddt_slots.len() > 0 || has_idt_slots {
        out.push_str("        let ddt_active = self.ddt_coefficients.active;\n");
    }
    if ddt_slots.len() > 0 {
        out.push_str("        let ddt_scale = self.ddt_coefficients.derivative_scale;\n");
        out.push_str(
            "        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;\n",
        );
        out.push_str(
            "        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;\n",
        );
        out.push_str(
            "        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;\n",
        );
    }
    if has_idt_slots {
        out.push_str("        let idt_scale = if ddt_active { timestep } else { 0.0 };\n");
    }

    let stamp_context = ValueEmitContext {
        cached_values: &static_cache.set,
        use_cached_fields: true,
        inline_uncached_constants: false,
        limexp_max_expr: "LIMEXP_MAX".to_string(),
        temperature_expr: "ctx.temperature()".to_string(),
        thermal_voltage_expr: "ctx.thermal_voltage()".to_string(),
    };
    emit_live_values(
        artifact,
        parameter_fields,
        &stamp_live,
        static_cache,
        &stamp_context,
        &mut out,
    )?;

    emit_current_stamps(
        artifact,
        parameter_fields,
        &roots,
        static_cache,
        Some(ddt_slots),
        &mut out,
    )?;

    out.push_str("    }\n\n");
    let reactive_roots = ddt_equation_roots(artifact, &roots);
    if reactive_roots.is_empty() {
        out.push_str(
            "    pub fn stamp_reactive(&mut self, _ctx: &GeneratedEvalContext<'_>, _stamper: &mut GeneratedReactiveStamper<'_>) {\n",
        );
        out.push_str("    }\n");
    } else {
        let reactive_live = collect_stamp_live_values(artifact, &reactive_roots, static_cache)?;
        out.push_str(
            "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
        );
        out.push_str("        let nodes = self.nodes;\n");
        out.push_str("        let branches = self.branches;\n");
        if static_cache.has_temperature_values() {
            out.push_str(
                "        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());\n",
            );
        }
        out.push_str("        let p = &(*self.params);\n");
        out.push_str("        let multiplicity = self.multiplicity;\n");
        emit_live_values(
            artifact,
            parameter_fields,
            &reactive_live,
            static_cache,
            &stamp_context,
            &mut out,
        )?;
        emit_current_reactive_stamps(artifact, &reactive_roots, static_cache, &mut out)?;
        out.push_str("    }\n");
    }
    out.push_str("}\n");
    Ok(out)
}

fn emit_transient_state_helpers(ddt_slots: &DdtSlots, out: &mut String) {
    if ddt_slots.len() > 0 {
        out.push_str("#[inline]\n");
        out.push_str("fn eval_ddt<const STATE_COUNT: usize>(\n");
        out.push_str("    current: &mut [f64; STATE_COUNT],\n");
        out.push_str("    previous: &mut [f64; STATE_COUNT],\n");
        out.push_str("    older: &mut [f64; STATE_COUNT],\n");
        out.push_str("    initialized: &mut [bool; STATE_COUNT],\n");
        out.push_str("    derivative_current: &mut [f64; STATE_COUNT],\n");
        out.push_str("    derivative_previous: &mut [f64; STATE_COUNT],\n");
        out.push_str("    ddt_active: bool,\n");
        out.push_str("    ddt_scale: f64,\n");
        out.push_str("    ddt_previous_value_scale: f64,\n");
        out.push_str("    ddt_older_value_scale: f64,\n");
        out.push_str("    ddt_previous_derivative_scale: f64,\n");
        out.push_str("    slot: usize,\n");
        out.push_str("    value: f64,\n");
        out.push_str(") -> f64 {\n");
        out.push_str(
            "    debug_assert!(slot < STATE_COUNT, \"generated ddt state slot out of range\");\n",
        );
        out.push_str(
            "    let previous_value = if initialized[slot] { previous[slot] } else { value };\n",
        );
        out.push_str(
            "    let older_value = if initialized[slot] { older[slot] } else { value };\n",
        );
        out.push_str("    current[slot] = value;\n");
        out.push_str("    if ddt_active {\n");
        out.push_str("        let result = value * ddt_scale\n");
        out.push_str("            - previous_value * ddt_previous_value_scale\n");
        out.push_str("            - older_value * ddt_older_value_scale\n");
        out.push_str("            - derivative_previous[slot] * ddt_previous_derivative_scale;\n");
        out.push_str("        derivative_current[slot] = result;\n");
        out.push_str("        result\n");
        out.push_str("    } else {\n");
        out.push_str("        current[slot] = value;\n");
        out.push_str("        previous[slot] = value;\n");
        out.push_str("        older[slot] = value;\n");
        out.push_str("        derivative_current[slot] = 0.0;\n");
        out.push_str("        derivative_previous[slot] = 0.0;\n");
        out.push_str("        initialized[slot] = true;\n");
        out.push_str("        0.0\n");
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }
    if ddt_slots.idt_len() > 0 {
        out.push_str("#[inline]\n");
        out.push_str("fn eval_idt<const STATE_COUNT: usize>(\n");
        out.push_str("    current: &mut [f64; STATE_COUNT],\n");
        out.push_str("    previous: &mut [f64; STATE_COUNT],\n");
        out.push_str("    initialized: &mut [bool; STATE_COUNT],\n");
        out.push_str("    ddt_active: bool,\n");
        out.push_str("    idt_scale: f64,\n");
        out.push_str("    slot: usize,\n");
        out.push_str("    value: f64,\n");
        out.push_str("    ic: f64,\n");
        out.push_str(") -> f64 {\n");
        out.push_str(
            "    debug_assert!(slot < STATE_COUNT, \"generated idt state slot out of range\");\n",
        );
        out.push_str(
            "    let previous_value = if initialized[slot] { previous[slot] } else { ic };\n",
        );
        out.push_str("    let current_value = if ddt_active {\n");
        out.push_str("        previous_value + value * idt_scale\n");
        out.push_str("    } else {\n");
        out.push_str("        ic\n");
        out.push_str("    };\n");
        out.push_str("    current[slot] = current_value;\n");
        out.push_str("    if !ddt_active {\n");
        out.push_str("        previous[slot] = current_value;\n");
        out.push_str("        initialized[slot] = true;\n");
        out.push_str("    }\n");
        out.push_str("    current_value\n");
        out.push_str("}\n\n");
    }
}

fn scalar_model_uses_limexp(artifact: &CanonicalIrArtifact) -> bool {
    artifact.opt.values.iter().any(|value| {
        matches!(
            &value.kind,
            OptValueKind::Unary {
                op: OptUnaryOp::LimExp
                    | OptUnaryOp::LimExpDerivative
                    | OptUnaryOp::LimitedExp
                    | OptUnaryOp::LimitedExpDerivative,
                ..
            }
        )
    })
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
    let instance_context = ValueEmitContext {
        cached_values: &static_cache.set,
        use_cached_fields: false,
        inline_uncached_constants: true,
        limexp_max_expr: format_f64(LIMEXP_MAX),
        temperature_expr: "temperature".to_string(),
        thermal_voltage_expr: "thermal_voltage".to_string(),
    };
    let temperature_context = ValueEmitContext {
        cached_values: &static_cache.set,
        use_cached_fields: true,
        inline_uncached_constants: true,
        limexp_max_expr: format_f64(LIMEXP_MAX),
        temperature_expr: "temperature".to_string(),
        thermal_voltage_expr: "thermal_voltage".to_string(),
    };
    let mut methods = String::new();

    for value_id in static_cache
        .instance_values
        .iter()
        .chain(static_cache.temperature_values.iter())
    {
        let value = artifact
            .opt
            .values
            .get(usize::from(*value_id))
            .ok_or_else(|| {
                unsupported(artifact, format!("missing static scalar value {value_id}"))
            })?;
        push_cached_value_state_fields(&mut extensions, *value_id, value.value_type);
    }

    if !static_cache.instance_values.is_empty() {
        methods.push_str("\n    #[inline]\n");
        methods.push_str("    fn recompute_instance_static(&mut self) {\n");
        methods.push_str("        let p = &(*self.params);\n");
        if cached_values_need_param_given(artifact, &static_cache.instance_values) {
            methods.push_str("        let param_given = self.param_given.as_ref();\n");
        }
        for value_id in &static_cache.instance_values {
            let value = artifact
                .opt
                .values
                .get(usize::from(*value_id))
                .ok_or_else(|| {
                    unsupported(artifact, format!("missing static scalar value {value_id}"))
                })?;
            let local = value_name(*value_id);
            let ty = rust_type(value.value_type);
            let expr = emit_value_expr(artifact, parameter_fields, value, &instance_context)?;
            methods.push_str(&format!("        let {local}: {ty} = {expr};\n"));
            methods.push_str(&format!(
                "        self.{} = {local};\n",
                cache_field_name(*value_id)
            ));
        }
        methods.push_str("    }\n");
        extensions
            .after_new
            .push_str("        instance.recompute_instance_static();\n");
        extensions
            .set_parameter_hook
            .push_str("self.recompute_instance_static(); ");
    }

    if static_cache.has_temperature_values() {
        push_temperature_cache_state_fields(&mut extensions);
        extensions
            .set_parameter_hook
            .push_str("self.invalidate_temperature_static(); ");
        methods.push_str(
            "\n    #[inline]\n    fn invalidate_temperature_static(&mut self) {\n        self.scalar_temperature_static_valid = false;\n    }\n",
        );
        methods.push_str(
            "\n    #[inline]\n    pub(super) fn ensure_temperature_static(&mut self, temperature: f64, thermal_voltage: f64) {\n        if !self.scalar_temperature_static_valid\n            || self.scalar_temperature_static_temperature.to_bits() != temperature.to_bits()\n            || self.scalar_temperature_static_thermal_voltage.to_bits() != thermal_voltage.to_bits()\n        {\n            self.recompute_temperature_static(temperature, thermal_voltage);\n        }\n    }\n",
        );
        methods.push_str(
            "\n    #[inline]\n    fn recompute_temperature_static(&mut self, temperature: f64, thermal_voltage: f64) {\n        let p = &(*self.params);\n",
        );
        if cached_values_need_param_given(artifact, &static_cache.temperature_values) {
            methods.push_str("        let param_given = self.param_given.as_ref();\n");
        }
        for value_id in &static_cache.temperature_values {
            let value = artifact
                .opt
                .values
                .get(usize::from(*value_id))
                .ok_or_else(|| {
                    unsupported(
                        artifact,
                        format!("missing temperature-static scalar value {value_id}"),
                    )
                })?;
            let local = value_name(*value_id);
            let ty = rust_type(value.value_type);
            let expr = emit_value_expr(artifact, parameter_fields, value, &temperature_context)?;
            methods.push_str(&format!("        let {local}: {ty} = {expr};\n"));
            methods.push_str(&format!(
                "        self.{} = {local};\n",
                cache_field_name(*value_id)
            ));
        }
        methods.push_str("        self.scalar_temperature_static_temperature = temperature;\n");
        methods.push_str(
            "        self.scalar_temperature_static_thermal_voltage = thermal_voltage;\n",
        );
        methods.push_str("        self.scalar_temperature_static_valid = true;\n");
        methods.push_str("    }\n");
    }

    extensions.impl_methods = methods;
    Ok(extensions)
}

fn cached_values_need_param_given(artifact: &CanonicalIrArtifact, values: &[ValueId]) -> bool {
    values.iter().any(|value_id| {
        artifact
            .opt
            .values
            .get(usize::from(*value_id))
            .is_some_and(|value| matches!(value.kind, OptValueKind::ParamGiven { .. }))
    })
}

fn push_cached_value_state_fields(
    extensions: &mut device::StateFileExtensions,
    value_id: ValueId,
    value_type: OptValueType,
) {
    let field = cache_field_name(value_id);
    let ty = rust_type(value_type);
    let default = default_value(value_type);
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
}

fn push_temperature_cache_state_fields(extensions: &mut device::StateFileExtensions) {
    extensions.instance_fields.push_str(
        "    pub(crate) scalar_temperature_static_valid: bool,\n    pub(crate) scalar_temperature_static_temperature: f64,\n    pub(crate) scalar_temperature_static_thermal_voltage: f64,\n",
    );
    extensions.clone_fields.push_str(
        "            scalar_temperature_static_valid: self.scalar_temperature_static_valid,\n            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,\n            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,\n",
    );
    extensions.new_initializers.push_str(
        "            scalar_temperature_static_valid: false,\n            scalar_temperature_static_temperature: 0.0,\n            scalar_temperature_static_thermal_voltage: 0.0,\n",
    );
    extensions.restore_destructure_fields.push_str(
        "            scalar_temperature_static_valid,\n            scalar_temperature_static_temperature,\n            scalar_temperature_static_thermal_voltage,\n",
    );
    extensions.restore_initializers.push_str(
        "            scalar_temperature_static_valid,\n            scalar_temperature_static_temperature,\n            scalar_temperature_static_thermal_voltage,\n",
    );
}

pub(super) fn scalarizable_current_equation_roots(
    artifact: &CanonicalIrArtifact,
) -> Result<HashMap<EquationId, ValueId>, RustBackendError> {
    let roots = available_scalar_equation_roots(artifact);
    let mut selected = HashMap::new();
    for equation in &artifact.mir.equations {
        if equation_transient_operator(artifact, equation)?.is_some() {
            continue;
        }
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

pub(super) fn scalarizable_potential_equation_roots(
    artifact: &CanonicalIrArtifact,
) -> Result<HashMap<EquationId, ValueId>, RustBackendError> {
    let roots = available_scalar_equation_roots(artifact);
    let mut selected = HashMap::new();
    for equation in &artifact.mir.equations {
        if equation.kind != MirEquationKind::Potential {
            continue;
        }
        if equation_transient_operator(artifact, equation)?.is_some() {
            continue;
        }
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        match validate_scalar_potential_equation(artifact, equation, root) {
            Ok(()) => {
                selected.insert(equation.id, root);
            }
            Err(error) if error.is_unsupported() => {}
            Err(error) => return Err(error),
        }
    }
    Ok(selected)
}

pub(super) fn scalarizable_ddt_current_equation_roots(
    artifact: &CanonicalIrArtifact,
) -> Result<HashMap<EquationId, ValueId>, RustBackendError> {
    let roots = available_scalar_equation_roots(artifact);
    let mut selected = HashMap::new();
    for equation in &artifact.mir.equations {
        if equation_ddt_expr(artifact, equation)?.is_none() {
            continue;
        }
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        match validate_scalar_current_equation(artifact, equation, root)
            .and_then(|()| scalar_derivatives(artifact, equation, root).map(|_| ()))
        {
            Ok(()) => {
                selected.insert(equation.id, root);
            }
            Err(error) if error.is_unsupported() => {}
            Err(error) => return Err(error),
        }
    }
    Ok(selected)
}

pub(super) fn scalarizable_idt_equation_roots(
    artifact: &CanonicalIrArtifact,
) -> Result<HashMap<EquationId, ValueId>, RustBackendError> {
    let roots = available_scalar_equation_roots(artifact);
    let mut selected = HashMap::new();
    for equation in &artifact.mir.equations {
        if !matches!(
            equation_transient_operator(artifact, equation)?,
            Some(ScalarTransientOperator::Idt { .. })
        ) {
            continue;
        }
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        let validated = match equation.kind {
            MirEquationKind::Current => validate_scalar_current_equation(artifact, equation, root),
            MirEquationKind::Potential => {
                validate_scalar_potential_equation(artifact, equation, root)
            }
            MirEquationKind::Indirect => Err(unsupported(artifact, "indirect contributions")),
        };
        match validated {
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
        limexp_max_expr: "LIMEXP_MAX".to_string(),
        temperature_expr: "ctx.temperature()".to_string(),
        thermal_voltage_expr: "ctx.thermal_voltage()".to_string(),
    };
    emit_live_values(
        artifact,
        parameter_fields,
        &stamp_live,
        static_cache,
        &stamp_context,
        out,
    )?;
    Ok(())
}

fn emit_live_values(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    live: &HashSet<ValueId>,
    static_cache: &ScalarStaticCache,
    context: &ValueEmitContext<'_>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let values = ordered_live_values(artifact, live, static_cache)?;
    for value_id in values {
        let value = artifact
            .opt
            .values
            .get(usize::from(value_id))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value_id}")))?;
        let expr = emit_value_expr(artifact, parameter_fields, value, context)?;
        out.push_str(&format!(
            "        let {}: {} = {};\n",
            value_name(value.id),
            rust_type(value.value_type),
            expr
        ));
    }
    if !live.is_empty() {
        out.push('\n');
    }
    Ok(())
}

pub(super) fn emit_static_current_stamps(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    out: &mut String,
) -> Result<(), RustBackendError> {
    emit_current_stamps(artifact, parameter_fields, roots, static_cache, None, out)
}

pub(super) fn emit_ddt_current_stamps(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    ddt_slots: &DdtSlots,
    out: &mut String,
) -> Result<(), RustBackendError> {
    emit_current_stamps(
        artifact,
        parameter_fields,
        roots,
        static_cache,
        Some(ddt_slots),
        out,
    )
}

fn emit_current_stamps(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    ddt_slots: Option<&DdtSlots>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    for equation in &artifact.mir.equations {
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        match equation.kind {
            MirEquationKind::Current => {
                emit_current_stamp(
                    artifact,
                    parameter_fields,
                    equation,
                    root,
                    static_cache,
                    ddt_slots,
                    out,
                )?;
            }
            MirEquationKind::Potential => {
                emit_potential_stamp(
                    artifact,
                    parameter_fields,
                    equation,
                    root,
                    static_cache,
                    ddt_slots,
                    out,
                )?;
            }
            MirEquationKind::Indirect => {
                return Err(unsupported(artifact, "indirect contributions"));
            }
        }
    }
    Ok(())
}

pub(super) fn emit_current_reactive_stamps(
    artifact: &CanonicalIrArtifact,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    out: &mut String,
) -> Result<(), RustBackendError> {
    for equation in &artifact.mir.equations {
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        emit_current_reactive_stamp(artifact, equation, root, static_cache, out)?;
    }
    Ok(())
}

pub(super) fn scalar_transient_current_lowered_variable(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
    static_cache: &ScalarStaticCache,
    branch_axis_count: usize,
    ddt_slots: Option<&DdtSlots>,
) -> Result<LoweredVariable, RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    let mut node_derivatives = vec!["0.0".to_string(); artifact.mir.nodes.len()];
    for (node, _) in &derivatives.nodes {
        node_derivatives[*node as usize] = derivative_name(root, *node);
    }
    let mut branch_derivatives = vec!["0.0".to_string(); branch_axis_count];
    for (branch, _) in &derivatives.branches {
        let index = *branch as usize;
        if index >= branch_axis_count {
            return Err(internal(
                artifact,
                format!("branch derivative lane {branch} exceeds axis count {branch_axis_count}"),
            ));
        }
        branch_derivatives[index] = branch_derivative_name(root, *branch);
    }

    let root_name = cached_or_local_value_name(root, static_cache);
    let mut value = current_root_expr(root_value.value_type, &root_name);
    let mut derivative_scale = "1.0";
    match equation_transient_operator(artifact, equation)? {
        Some(ScalarTransientOperator::Ddt { operator }) => {
            let slots =
                ddt_slots.ok_or_else(|| unsupported(artifact, "ddt scalar cache context"))?;
            if slots.slot_for(operator).is_none() {
                return Err(internal(
                    artifact,
                    format!("ddt expression {operator} has no generated state slot"),
                ));
            }
            value = format!("{}_ddt", value_name(root));
            derivative_scale = "ddt_scale";
        }
        Some(ScalarTransientOperator::Idt { operator, .. }) => {
            let slots =
                ddt_slots.ok_or_else(|| unsupported(artifact, "idt scalar cache context"))?;
            if slots.idt_slot_for(operator).is_none() {
                return Err(internal(
                    artifact,
                    format!("idt expression {operator} has no generated state slot"),
                ));
            }
            value = format!("{}_idt", value_name(root));
            derivative_scale = "idt_scale";
        }
        None => {}
    }
    if derivative_scale != "1.0" {
        for derivative in &mut node_derivatives {
            if derivative != "0.0" {
                *derivative = scaled_derivative_expr(derivative.clone(), derivative_scale);
            }
        }
        for derivative in &mut branch_derivatives {
            if derivative != "0.0" {
                *derivative = scaled_derivative_expr(derivative.clone(), derivative_scale);
            }
        }
    }

    Ok(LoweredVariable {
        value,
        condition: None,
        derivatives: node_derivatives,
        branch_derivatives,
        has_reactive: false,
        reactive_value: "0.0".to_string(),
        reactive_derivatives: vec!["0.0".to_string(); artifact.mir.nodes.len()],
        reactive_branch_derivatives: vec!["0.0".to_string(); branch_axis_count],
    })
}

pub(super) fn scalar_reactive_current_lowered_variable(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
    static_cache: &ScalarStaticCache,
    branch_axis_count: usize,
) -> Result<LoweredVariable, RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    let mut node_derivatives = vec!["0.0".to_string(); artifact.mir.nodes.len()];
    for (node, _) in &derivatives.nodes {
        node_derivatives[*node as usize] = derivative_name(root, *node);
    }
    let mut branch_derivatives = vec!["0.0".to_string(); branch_axis_count];
    for (branch, _) in &derivatives.branches {
        let index = *branch as usize;
        if index >= branch_axis_count {
            return Err(internal(
                artifact,
                format!("branch derivative lane {branch} exceeds axis count {branch_axis_count}"),
            ));
        }
        branch_derivatives[index] = branch_derivative_name(root, *branch);
    }
    let root_name = cached_or_local_value_name(root, static_cache);
    let value = current_root_expr(root_value.value_type, &root_name);

    Ok(LoweredVariable {
        value: value.clone(),
        condition: None,
        derivatives: node_derivatives.clone(),
        branch_derivatives: branch_derivatives.clone(),
        has_reactive: true,
        reactive_value: value,
        reactive_derivatives: node_derivatives,
        reactive_branch_derivatives: branch_derivatives,
    })
}

fn scalar_derivatives(
    artifact: &CanonicalIrArtifact,
    _equation: &MirEquation,
    root: ValueId,
) -> Result<ScalarDerivatives, RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let mut nodes = Vec::new();
    let mut branches = Vec::new();
    for derivative in &root_value.derivatives {
        match derivative.lane.kind {
            DerivativeLaneKind::Node => nodes.push((derivative.lane.index, derivative.value)),
            DerivativeLaneKind::BranchUnknown => {
                branches.push((derivative.lane.index, derivative.value));
            }
        }
    }
    Ok(ScalarDerivatives { nodes, branches })
}

fn emit_transient_operator_root(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    equation: &MirEquation,
    root: ValueId,
    root_expr: String,
    ddt_slots: Option<&DdtSlots>,
    out: &mut String,
) -> Result<(String, String), RustBackendError> {
    match equation_transient_operator(artifact, equation)? {
        Some(ScalarTransientOperator::Ddt { operator }) => {
            let slots =
                ddt_slots.ok_or_else(|| unsupported(artifact, "ddt scalar stamp context"))?;
            let slot = slots.slot_for(operator).ok_or_else(|| {
                internal(
                    artifact,
                    format!("ddt expression {operator} has no generated state slot"),
                )
            })?;
            let ddt_value = format!("{}_ddt", value_name(root));
            out.push_str(&format!(
                "        let {ddt_value}: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, {slot}, {root_expr});\n"
            ));
            Ok((ddt_value, "ddt_scale".to_string()))
        }
        Some(ScalarTransientOperator::Idt { operator, ic }) => {
            let slots =
                ddt_slots.ok_or_else(|| unsupported(artifact, "idt scalar stamp context"))?;
            let slot = slots.idt_slot_for(operator).ok_or_else(|| {
                internal(
                    artifact,
                    format!("idt expression {operator} has no generated state slot"),
                )
            })?;
            let ic_expr = emit_idt_ic_expr(artifact, parameter_fields, ic)?;
            let idt_value = format!("{}_idt", value_name(root));
            out.push_str(&format!(
                "        let {idt_value}: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, {slot}, {root_expr}, {ic_expr});\n"
            ));
            Ok((idt_value, "idt_scale".to_string()))
        }
        None => Ok((root_expr, "1.0".to_string())),
    }
}

fn emit_current_stamp(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    equation: &MirEquation,
    root: ValueId,
    static_cache: &ScalarStaticCache,
    ddt_slots: Option<&DdtSlots>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    for (node, value) in &derivatives.nodes {
        out.push_str(&format!(
            "        let {}: f64 = {};\n",
            derivative_name(root, *node),
            cached_or_local_value_name(*value, static_cache)
        ));
    }
    for (branch, value) in &derivatives.branches {
        out.push_str(&format!(
            "        let {}: f64 = {};\n",
            branch_derivative_name(root, *branch),
            cached_or_local_value_name(*value, static_cache)
        ));
    }

    let pos = optional_node_local_expr(equation.branch.pos_node);
    let neg = optional_node_local_expr(equation.branch.neg_node);
    let root_name = cached_or_local_value_name(root, static_cache);
    let root_expr = current_root_expr(root_value.value_type, &root_name);
    let (root_expr, derivative_scale) = emit_transient_operator_root(
        artifact,
        parameter_fields,
        equation,
        root,
        root_expr,
        ddt_slots,
        out,
    )?;
    match (
        derivatives.nodes.as_slice(),
        derivatives.branches.as_slice(),
    ) {
        ([], []) => {
            out.push_str("        stamper.stamp_current_const_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str("        );\n");
        }
        ([(node0, _)], []) => {
            out.push_str("        stamper.stamp_current_node1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str("        );\n");
        }
        ([(node0, _), (node1, _)], []) => {
            out.push_str("        stamper.stamp_current_node2_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node1), derivative_scale.as_str())
            ));
            out.push_str("        );\n");
        }
        ([(node0, _), (node1, _), (node2, _)], []) => {
            out.push_str("        stamper.stamp_current_node3_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node1), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {node2},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node2), derivative_scale.as_str())
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_current_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch0),
                    derivative_scale.as_str()
                )
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, _), (branch1, _)]) => {
            out.push_str("        stamper.stamp_current_branch2_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch0),
                    derivative_scale.as_str()
                )
            ));
            out.push_str(&format!("            {branch1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch1),
                    derivative_scale.as_str()
                )
            ));
            out.push_str("        );\n");
        }
        ([(node0, _)], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_current_node1_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch0),
                    derivative_scale.as_str()
                )
            ));
            out.push_str("        );\n");
        }
        ([(node0, _), (node1, _)], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_current_node2_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(derivative_name(root, *node1), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch0),
                    derivative_scale.as_str()
                )
            ));
            out.push_str("        );\n");
        }
        _ => {
            emit_wide_current_stamp(
                artifact,
                root,
                &derivatives.nodes,
                &derivatives.branches,
                &pos,
                &neg,
                &root_expr,
                derivative_scale.as_str(),
                out,
            );
        }
    }
    Ok(())
}

fn emit_potential_stamp(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    equation: &MirEquation,
    root: ValueId,
    static_cache: &ScalarStaticCache,
    ddt_slots: Option<&DdtSlots>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    for (node, value) in &derivatives.nodes {
        out.push_str(&format!(
            "        let {}: f64 = {};\n",
            derivative_name(root, *node),
            cached_or_local_value_name(*value, static_cache)
        ));
    }
    for (branch, value) in &derivatives.branches {
        out.push_str(&format!(
            "        let {}: f64 = {};\n",
            branch_derivative_name(root, *branch),
            cached_or_local_value_name(*value, static_cache)
        ));
    }

    let branch_slot = potential_branch_slot(artifact, equation)?;
    let pos = optional_node_local_expr(equation.branch.pos_node);
    let neg = optional_node_local_expr(equation.branch.neg_node);
    let root_name = cached_or_local_value_name(root, static_cache);
    let root_expr = current_root_expr(root_value.value_type, &root_name);
    let (root_expr, derivative_scale) = emit_transient_operator_root(
        artifact,
        parameter_fields,
        equation,
        root,
        root_expr,
        ddt_slots,
        out,
    )?;
    out.push_str("        stamper.stamp_potential_branch_local(\n");
    out.push_str(&format!("            {pos},\n"));
    out.push_str(&format!("            {neg},\n"));
    out.push_str(&format!("            {branch_slot},\n"));
    out.push_str("            multiplicity,\n");
    out.push_str("        );\n");

    match (
        derivatives.nodes.as_slice(),
        derivatives.branches.as_slice(),
    ) {
        ([], []) => {
            out.push_str("        stamper.stamp_potential_const_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str("        );\n");
        }
        ([(node0, _)], []) => {
            out.push_str("        stamper.stamp_potential_node1_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str("        );\n");
        }
        ([(node0, _), (node1, _)], []) => {
            out.push_str("        stamper.stamp_potential_node2_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_name(root, *node1), derivative_scale.as_str())
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_potential_branch1_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch0),
                    derivative_scale.as_str()
                )
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, _), (branch1, _)]) => {
            out.push_str("        stamper.stamp_potential_branch2_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch0),
                    derivative_scale.as_str()
                )
            ));
            out.push_str(&format!("            {branch1},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch1),
                    derivative_scale.as_str()
                )
            ));
            out.push_str("        );\n");
        }
        ([(node0, _)], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_potential_node1_branch1_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch0),
                    derivative_scale.as_str()
                )
            ));
            out.push_str("        );\n");
        }
        ([(node0, _), (node1, _)], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_potential_node2_branch1_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_name(root, *node0), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_name(root, *node1), derivative_scale.as_str())
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(
                    branch_derivative_name(root, *branch0),
                    derivative_scale.as_str()
                )
            ));
            out.push_str("        );\n");
        }
        _ => {
            emit_wide_potential_stamp(
                artifact,
                root,
                &derivatives.nodes,
                &derivatives.branches,
                branch_slot,
                &root_expr,
                derivative_scale.as_str(),
                out,
            );
        }
    }
    Ok(())
}

fn emit_current_reactive_stamp(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
    static_cache: &ScalarStaticCache,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    for (node, value) in &derivatives.nodes {
        out.push_str(&format!(
            "        let {}: f64 = {};\n",
            derivative_name(root, *node),
            cached_or_local_value_name(*value, static_cache)
        ));
    }
    for (branch, value) in &derivatives.branches {
        out.push_str(&format!(
            "        let {}: f64 = {};\n",
            branch_derivative_name(root, *branch),
            cached_or_local_value_name(*value, static_cache)
        ));
    }

    let pos = optional_node_global_expr(equation.branch.pos_node);
    let neg = optional_node_global_expr(equation.branch.neg_node);
    match (
        derivatives.nodes.as_slice(),
        derivatives.branches.as_slice(),
    ) {
        ([], []) => {}
        ([(node0, _)], []) => {
            out.push_str("        stamper.stamp_current_reactive_node1(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            nodes[{node0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node0)
            ));
            out.push_str("        );\n");
        }
        ([(node0, _), (node1, _)], []) => {
            out.push_str("        stamper.stamp_current_reactive_node2(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            nodes[{node0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node0)
            ));
            out.push_str(&format!("            nodes[{node1}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node1)
            ));
            out.push_str("        );\n");
        }
        ([(node0, _), (node1, _), (node2, _)], []) => {
            out.push_str("        stamper.stamp_current_reactive_node3(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            nodes[{node0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node0)
            ));
            out.push_str(&format!("            nodes[{node1}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node1)
            ));
            out.push_str(&format!("            nodes[{node2}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node2)
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_current_reactive_branch1(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            branches[{branch0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                branch_derivative_name(root, *branch0)
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, _), (branch1, _)]) => {
            out.push_str("        stamper.stamp_current_reactive_branch2(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            branches[{branch0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                branch_derivative_name(root, *branch0)
            ));
            out.push_str(&format!("            branches[{branch1}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                branch_derivative_name(root, *branch1)
            ));
            out.push_str("        );\n");
        }
        ([(node0, _)], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_current_reactive_node1_branch1(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            nodes[{node0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node0)
            ));
            out.push_str(&format!("            branches[{branch0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                branch_derivative_name(root, *branch0)
            ));
            out.push_str("        );\n");
        }
        ([(node0, _), (node1, _)], [(branch0, _)]) => {
            out.push_str("        stamper.stamp_current_reactive_node2_branch1(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            nodes[{node0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node0)
            ));
            out.push_str(&format!("            nodes[{node1}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                derivative_name(root, *node1)
            ));
            out.push_str(&format!("            branches[{branch0}],\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                branch_derivative_name(root, *branch0)
            ));
            out.push_str("        );\n");
        }
        _ => {
            emit_wide_current_reactive_stamp(
                artifact,
                root,
                &derivatives.nodes,
                &derivatives.branches,
                &pos,
                &neg,
                out,
            );
        }
    }
    Ok(())
}

fn emit_wide_current_reactive_stamp(
    artifact: &CanonicalIrArtifact,
    root: ValueId,
    node_derivatives: &[(u32, ValueId)],
    branch_derivatives: &[(u32, ValueId)],
    pos: &str,
    neg: &str,
    out: &mut String,
) {
    if branch_derivatives.is_empty() {
        emit_wide_node_current_reactive_stamp(artifact, root, node_derivatives, pos, neg, out);
        return;
    }

    if node_derivatives.len() == artifact.mir.nodes.len()
        && branch_derivatives.len() == artifact.mir.branch_unknowns.len()
    {
        let mut node_values = vec!["0.0".to_string(); artifact.mir.nodes.len()];
        for (node, _) in node_derivatives {
            node_values[*node as usize] = derivative_name(root, *node);
        }
        let mut branch_values = vec!["0.0".to_string(); artifact.mir.branch_unknowns.len()];
        for (branch, _) in branch_derivatives {
            branch_values[*branch as usize] = branch_derivative_name(root, *branch);
        }
        out.push_str(&format!(
            "        let {}_reactive_node_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            node_values.len(),
            node_values.join(", ")
        ));
        out.push_str(&format!(
            "        let {}_reactive_branch_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            branch_values.len(),
            branch_values.join(", ")
        ));
        out.push_str("        stamper.stamp_current_reactive_dense(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str("            &nodes,\n");
        out.push_str(&format!(
            "            &{}_reactive_node_derivatives,\n",
            value_name(root)
        ));
        out.push_str("            &branches,\n");
        out.push_str(&format!(
            "            &{}_reactive_branch_derivatives,\n",
            value_name(root)
        ));
        out.push_str("            multiplicity,\n");
        out.push_str("        );\n");
    } else {
        let node_indices = node_derivatives
            .iter()
            .map(|(node, _)| format!("nodes[{node}]"))
            .collect::<Vec<_>>()
            .join(", ");
        let node_values = node_derivatives
            .iter()
            .map(|(node, _)| derivative_name(root, *node))
            .collect::<Vec<_>>()
            .join(", ");
        let branch_indices = branch_derivatives
            .iter()
            .map(|(branch, _)| format!("branches[{branch}]"))
            .collect::<Vec<_>>()
            .join(", ");
        let branch_values = branch_derivatives
            .iter()
            .map(|(branch, _)| branch_derivative_name(root, *branch))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "        let {}_reactive_nodes: [usize; {}] = [{}];\n",
            value_name(root),
            node_derivatives.len(),
            node_indices
        ));
        out.push_str(&format!(
            "        let {}_reactive_node_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            node_derivatives.len(),
            node_values
        ));
        out.push_str(&format!(
            "        let {}_reactive_branches: [usize; {}] = [{}];\n",
            value_name(root),
            branch_derivatives.len(),
            branch_indices
        ));
        out.push_str(&format!(
            "        let {}_reactive_branch_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            branch_derivatives.len(),
            branch_values
        ));
        out.push_str("        stamper.stamp_current_reactive_dense(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str(&format!(
            "            &{}_reactive_nodes,\n",
            value_name(root)
        ));
        out.push_str(&format!(
            "            &{}_reactive_node_derivatives,\n",
            value_name(root)
        ));
        out.push_str(&format!(
            "            &{}_reactive_branches,\n",
            value_name(root)
        ));
        out.push_str(&format!(
            "            &{}_reactive_branch_derivatives,\n",
            value_name(root)
        ));
        out.push_str("            multiplicity,\n");
        out.push_str("        );\n");
    }
}

fn emit_wide_node_current_reactive_stamp(
    artifact: &CanonicalIrArtifact,
    root: ValueId,
    derivatives: &[(u32, ValueId)],
    pos: &str,
    neg: &str,
    out: &mut String,
) {
    if derivatives.len() == artifact.mir.nodes.len() {
        let mut node_derivatives = vec!["0.0".to_string(); artifact.mir.nodes.len()];
        for (node, _) in derivatives {
            node_derivatives[*node as usize] = derivative_name(root, *node);
        }
        out.push_str(&format!(
            "        let {}_reactive_node_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            node_derivatives.len(),
            node_derivatives.join(", ")
        ));
        out.push_str("        stamper.stamp_current_reactive_dense(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str("            &nodes,\n");
        out.push_str(&format!(
            "            &{}_reactive_node_derivatives,\n",
            value_name(root)
        ));
        out.push_str("            &[],\n");
        out.push_str("            &[],\n");
        out.push_str("            multiplicity,\n");
        out.push_str("        );\n");
    } else {
        let node_indices = derivatives
            .iter()
            .map(|(node, _)| format!("nodes[{node}]"))
            .collect::<Vec<_>>()
            .join(", ");
        let node_derivatives = derivatives
            .iter()
            .map(|(node, _)| derivative_name(root, *node))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "        let {}_reactive_nodes: [usize; {}] = [{}];\n",
            value_name(root),
            derivatives.len(),
            node_indices
        ));
        out.push_str(&format!(
            "        let {}_reactive_node_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            derivatives.len(),
            node_derivatives
        ));
        out.push_str("        stamper.stamp_current_reactive_dense(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str(&format!(
            "            &{}_reactive_nodes,\n",
            value_name(root)
        ));
        out.push_str(&format!(
            "            &{}_reactive_node_derivatives,\n",
            value_name(root)
        ));
        out.push_str("            &[],\n");
        out.push_str("            &[],\n");
        out.push_str("            multiplicity,\n");
        out.push_str("        );\n");
    }
}

fn emit_wide_potential_stamp(
    artifact: &CanonicalIrArtifact,
    root: ValueId,
    node_derivatives: &[(u32, ValueId)],
    branch_derivatives: &[(u32, ValueId)],
    branch_slot: usize,
    root_expr: &str,
    derivative_scale: &str,
    out: &mut String,
) {
    if node_derivatives.len() == artifact.mir.nodes.len()
        && branch_derivatives.len() == artifact.mir.branch_unknowns.len()
    {
        let mut node_values = vec!["0.0".to_string(); artifact.mir.nodes.len()];
        for (node, _) in node_derivatives {
            node_values[*node as usize] =
                scaled_derivative_expr(derivative_name(root, *node), derivative_scale);
        }
        let mut branch_values = vec!["0.0".to_string(); artifact.mir.branch_unknowns.len()];
        for (branch, _) in branch_derivatives {
            branch_values[*branch as usize] =
                scaled_derivative_expr(branch_derivative_name(root, *branch), derivative_scale);
        }
        out.push_str(&format!(
            "        let {}_node_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            node_values.len(),
            node_values.join(", ")
        ));
        out.push_str(&format!(
            "        let {}_branch_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            branch_values.len(),
            branch_values.join(", ")
        ));
        out.push_str("        stamper.stamp_potential_dense_local(\n");
        out.push_str(&format!("            {branch_slot},\n"));
        out.push_str(&format!("            {root_expr},\n"));
        out.push_str(&format!(
            "            &{}_node_derivatives,\n",
            value_name(root)
        ));
        out.push_str(&format!(
            "            &{}_branch_derivatives,\n",
            value_name(root)
        ));
        out.push_str("        );\n");
    } else {
        let node_indices = node_derivatives
            .iter()
            .map(|(node, _)| node.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let node_values = node_derivatives
            .iter()
            .map(|(node, _)| scaled_derivative_expr(derivative_name(root, *node), derivative_scale))
            .collect::<Vec<_>>()
            .join(", ");
        let branch_indices = branch_derivatives
            .iter()
            .map(|(branch, _)| branch.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let branch_values = branch_derivatives
            .iter()
            .map(|(branch, _)| {
                scaled_derivative_expr(branch_derivative_name(root, *branch), derivative_scale)
            })
            .collect::<Vec<_>>()
            .join(", ");
        if node_derivatives.len() + branch_derivatives.len() <= SPARSE_STAMP_DERIVATIVE_THRESHOLD {
            out.push_str(&format!(
                "        stamper.stamp_potential_sparse_local::<{}, {}>(\n",
                node_derivatives.len(),
                branch_derivatives.len()
            ));
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            [{node_indices}],\n"));
            out.push_str(&format!("            [{node_values}],\n"));
            out.push_str(&format!("            [{branch_indices}],\n"));
            out.push_str(&format!("            [{branch_values}],\n"));
            out.push_str("        );\n");
        } else {
            out.push_str(&format!(
                "        let {}_node_derivative_indices: [usize; {}] = [{}];\n",
                value_name(root),
                node_derivatives.len(),
                node_indices
            ));
            out.push_str(&format!(
                "        let {}_node_derivatives: [f64; {}] = [{}];\n",
                value_name(root),
                node_derivatives.len(),
                node_values
            ));
            out.push_str(&format!(
                "        let {}_branch_derivative_indices: [usize; {}] = [{}];\n",
                value_name(root),
                branch_derivatives.len(),
                branch_indices
            ));
            out.push_str(&format!(
                "        let {}_branch_derivatives: [f64; {}] = [{}];\n",
                value_name(root),
                branch_derivatives.len(),
                branch_values
            ));
            out.push_str("        stamper.stamp_potential_indexed_dense_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!(
                "            &{}_node_derivative_indices,\n",
                value_name(root)
            ));
            out.push_str(&format!(
                "            &{}_node_derivatives,\n",
                value_name(root)
            ));
            out.push_str(&format!(
                "            &{}_branch_derivative_indices,\n",
                value_name(root)
            ));
            out.push_str(&format!(
                "            &{}_branch_derivatives,\n",
                value_name(root)
            ));
            out.push_str("        );\n");
        }
    }
}

fn emit_wide_current_stamp(
    artifact: &CanonicalIrArtifact,
    root: ValueId,
    node_derivatives: &[(u32, ValueId)],
    branch_derivatives: &[(u32, ValueId)],
    pos: &str,
    neg: &str,
    root_expr: &str,
    derivative_scale: &str,
    out: &mut String,
) {
    if node_derivatives.len() == artifact.mir.nodes.len()
        && branch_derivatives.len() == artifact.mir.branch_unknowns.len()
    {
        let mut node_values = vec!["0.0".to_string(); artifact.mir.nodes.len()];
        for (node, _) in node_derivatives {
            node_values[*node as usize] =
                scaled_derivative_expr(derivative_name(root, *node), derivative_scale);
        }
        let mut branch_values = vec!["0.0".to_string(); artifact.mir.branch_unknowns.len()];
        for (branch, _) in branch_derivatives {
            branch_values[*branch as usize] =
                scaled_derivative_expr(branch_derivative_name(root, *branch), derivative_scale);
        }
        out.push_str(&format!(
            "        let {}_node_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            node_values.len(),
            node_values.join(", ")
        ));
        out.push_str(&format!(
            "        let {}_branch_derivatives: [f64; {}] = [{}];\n",
            value_name(root),
            branch_values.len(),
            branch_values.join(", ")
        ));
        out.push_str("        stamper.stamp_current_dense_local(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
        out.push_str(&format!(
            "            &{}_node_derivatives,\n",
            value_name(root)
        ));
        out.push_str(&format!(
            "            &{}_branch_derivatives,\n",
            value_name(root)
        ));
        out.push_str("            multiplicity,\n");
        out.push_str("        );\n");
    } else {
        let node_indices = node_derivatives
            .iter()
            .map(|(node, _)| node.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let node_values = node_derivatives
            .iter()
            .map(|(node, _)| scaled_derivative_expr(derivative_name(root, *node), derivative_scale))
            .collect::<Vec<_>>()
            .join(", ");
        let branch_indices = branch_derivatives
            .iter()
            .map(|(branch, _)| branch.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let branch_values = branch_derivatives
            .iter()
            .map(|(branch, _)| {
                scaled_derivative_expr(branch_derivative_name(root, *branch), derivative_scale)
            })
            .collect::<Vec<_>>()
            .join(", ");
        if node_derivatives.len() + branch_derivatives.len() <= SPARSE_STAMP_DERIVATIVE_THRESHOLD {
            out.push_str(&format!(
                "        stamper.stamp_current_sparse_local::<{}, {}>(\n",
                node_derivatives.len(),
                branch_derivatives.len()
            ));
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            [{node_indices}],\n"));
            out.push_str(&format!("            [{node_values}],\n"));
            out.push_str(&format!("            [{branch_indices}],\n"));
            out.push_str(&format!("            [{branch_values}],\n"));
            out.push_str("            multiplicity,\n");
            out.push_str("        );\n");
        } else {
            out.push_str(&format!(
                "        let {}_node_derivative_indices: [usize; {}] = [{}];\n",
                value_name(root),
                node_derivatives.len(),
                node_indices
            ));
            out.push_str(&format!(
                "        let {}_node_derivatives: [f64; {}] = [{}];\n",
                value_name(root),
                node_derivatives.len(),
                node_values
            ));
            out.push_str(&format!(
                "        let {}_branch_derivative_indices: [usize; {}] = [{}];\n",
                value_name(root),
                branch_derivatives.len(),
                branch_indices
            ));
            out.push_str(&format!(
                "        let {}_branch_derivatives: [f64; {}] = [{}];\n",
                value_name(root),
                branch_derivatives.len(),
                branch_values
            ));
            out.push_str("        stamper.stamp_current_indexed_dense_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!(
                "            &{}_node_derivative_indices,\n",
                value_name(root)
            ));
            out.push_str(&format!(
                "            &{}_node_derivatives,\n",
                value_name(root)
            ));
            out.push_str(&format!(
                "            &{}_branch_derivative_indices,\n",
                value_name(root)
            ));
            out.push_str(&format!(
                "            &{}_branch_derivatives,\n",
                value_name(root)
            ));
            out.push_str("            multiplicity,\n");
            out.push_str("        );\n");
        }
    }
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
        OptValueKind::ParamGiven { parameter } => {
            let index = usize::from(*parameter);
            format!("if param_given[{index}] {{ 1.0 }} else {{ 0.0 }}")
        }
        OptValueKind::Temperature => context.temperature_expr.clone(),
        OptValueKind::ThermalVoltage => context.thermal_voltage_expr.clone(),
        OptValueKind::Multiplicity => "multiplicity".to_string(),
        OptValueKind::Time => "self.time".to_string(),
        OptValueKind::Analysis { query } => {
            format!(
                "if {} {{ 1.0 }} else {{ 0.0 }}",
                analysis_predicate_expr(query.as_str())
            )
        }
        OptValueKind::Ddx {
            value,
            pos_node,
            neg_node,
        } => emit_ddx_projection_expr(
            artifact,
            parameter_fields,
            *value,
            *pos_node,
            *neg_node,
            context,
        )?,
        OptValueKind::NodePotential { node } => {
            format!("ctx.node_voltage(nodes[{}])", node.index())
        }
        OptValueKind::BranchFlow { branch } => {
            let slot = branch_flow_slot(artifact, *branch)?;
            format!("ctx.branch_current(branches[{slot}])")
        }
        OptValueKind::Unary { op, input } => emit_unary_expr(
            *op,
            value_ref(artifact, parameter_fields, *input, context)?,
            value_type(artifact, *input)?,
            &context.limexp_max_expr,
        ),
        OptValueKind::Binary { op, left, right } => {
            let left_type = value_type(artifact, *left)?;
            let right_type = value_type(artifact, *right)?;
            emit_binary_expr(
                *op,
                value_ref(artifact, parameter_fields, *left, context)?,
                left_type,
                value_ref(artifact, parameter_fields, *right, context)?,
                right_type,
            )
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            let condition_type = value_type(artifact, *condition)?;
            format!(
                "(if {} {{ {} }} else {{ {} }})",
                truth_expr(
                    value_ref(artifact, parameter_fields, *condition, context)?,
                    condition_type,
                ),
                value_ref(artifact, parameter_fields, *then_value, context)?,
                value_ref(artifact, parameter_fields, *else_value, context)?
            )
        }
        OptValueKind::EquationValue { .. } => {
            return Err(unsupported(
                artifact,
                "legacy equation value in scalar backend",
            ));
        }
    };
    Ok(expr)
}

fn emit_ddx_projection_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    value: ValueId,
    pos_node: Option<NodeId>,
    neg_node: Option<NodeId>,
    context: &ValueEmitContext<'_>,
) -> Result<String, RustBackendError> {
    let pos = ddx_derivative_expr(artifact, parameter_fields, value, pos_node, context)?;
    if let Some(neg_node) = neg_node {
        let neg = ddx_derivative_expr(artifact, parameter_fields, value, Some(neg_node), context)?;
        Ok(format!("(0.5 * ({pos} - {neg}))"))
    } else {
        Ok(pos)
    }
}

fn ddx_derivative_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    value: ValueId,
    node: Option<NodeId>,
    context: &ValueEmitContext<'_>,
) -> Result<String, RustBackendError> {
    let Some(node) = node else {
        return Ok("0.0".to_string());
    };
    let Some(derivative) = derivative_value_for_lane(artifact, value, DerivativeLane::node(node))?
    else {
        return Ok("0.0".to_string());
    };
    value_ref(artifact, parameter_fields, derivative, context)
}

fn emit_idt_ic_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    ic: Option<ExprId>,
) -> Result<String, RustBackendError> {
    match ic {
        Some(ic) => emit_value_only_expr(artifact, parameter_fields, ic),
        None => Ok("0.0".to_string()),
    }
}

fn emit_value_only_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    expr: ExprId,
) -> Result<String, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(expr))
        .ok_or_else(|| unsupported(artifact, format!("missing expression {expr}")))?;
    match &expression.kind {
        HirExprKind::Number { value, .. } => Ok(format_f64(*value)),
        HirExprKind::Identifier { name } => {
            let parameter = artifact
                .mir
                .parameters
                .iter()
                .find(|parameter| parameter.name.as_str() == name.as_str())
                .ok_or_else(|| {
                    unsupported(
                        artifact,
                        format!("idt initial condition identifier '{name}'"),
                    )
                })?;
            emit_parameter_expr(artifact, parameter_fields, parameter.id)
        }
        HirExprKind::Unary { op, operand } => {
            let operand = emit_value_only_expr(artifact, parameter_fields, *operand)?;
            match op.as_str() {
                "Pos" => Ok(operand),
                "Neg" => Ok(format!("(-{operand})")),
                "Not" => Ok(format!(
                    "if {} {{ 0.0 }} else {{ 1.0 }}",
                    value_truth_expr(&operand)
                )),
                _ => Err(unsupported(
                    artifact,
                    format!("idt initial condition unary operator {op}"),
                )),
            }
        }
        HirExprKind::Binary { op, left, right } => {
            let left = emit_value_only_expr(artifact, parameter_fields, *left)?;
            let right = emit_value_only_expr(artifact, parameter_fields, *right)?;
            let value = match op.as_str() {
                "Add" => format!("({left} + {right})"),
                "Sub" => format!("({left} - {right})"),
                "Mul" => format!("({left} * {right})"),
                "Div" => format!("({left} / {right})"),
                "Mod" => format!("({left} % {right})"),
                "Pow" => format!("({left}).powf({right})"),
                "Eq" => format!("if {left} == {right} {{ 1.0 }} else {{ 0.0 }}"),
                "Ne" => format!("if {left} != {right} {{ 1.0 }} else {{ 0.0 }}"),
                "Lt" => format!("if {left} < {right} {{ 1.0 }} else {{ 0.0 }}"),
                "Le" => format!("if {left} <= {right} {{ 1.0 }} else {{ 0.0 }}"),
                "Gt" => format!("if {left} > {right} {{ 1.0 }} else {{ 0.0 }}"),
                "Ge" => format!("if {left} >= {right} {{ 1.0 }} else {{ 0.0 }}"),
                "And" => format!(
                    "if {} && {} {{ 1.0 }} else {{ 0.0 }}",
                    value_truth_expr(&left),
                    value_truth_expr(&right)
                ),
                "Or" => format!(
                    "if {} || {} {{ 1.0 }} else {{ 0.0 }}",
                    value_truth_expr(&left),
                    value_truth_expr(&right)
                ),
                _ => {
                    return Err(unsupported(
                        artifact,
                        format!("idt initial condition binary operator {op}"),
                    ));
                }
            };
            Ok(value)
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            let condition = emit_value_only_expr(artifact, parameter_fields, *condition)?;
            let then_expr = emit_value_only_expr(artifact, parameter_fields, *then_expr)?;
            let else_expr = emit_value_only_expr(artifact, parameter_fields, *else_expr)?;
            Ok(format!(
                "if {} {{ {then_expr} }} else {{ {else_expr} }}",
                value_truth_expr(&condition)
            ))
        }
        HirExprKind::SystemFunction { name, args } => {
            emit_value_only_system_function(artifact, parameter_fields, name.as_str(), args)
        }
        HirExprKind::Call { name, args } => {
            emit_value_only_call(artifact, parameter_fields, name.as_str(), args)
        }
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Limexp { expr },
        } => {
            let expr = emit_value_only_expr(artifact, parameter_fields, *expr)?;
            Ok(format!(
                "{{ let limexp_arg = {expr}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ {} * (1.0 + (limexp_arg - 80.0)) }} }}",
                format_f64(LIMEXP_MAX)
            ))
        }
        _ => Err(unsupported(
            artifact,
            format!(
                "idt initial condition expression kind {:?}",
                &expression.kind
            ),
        )),
    }
}

fn emit_value_only_system_function(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    name: &str,
    args: &[ExprId],
) -> Result<String, RustBackendError> {
    let normalized = name.to_ascii_lowercase();
    match normalized.as_str() {
        "$temperature" if args.is_empty() => Ok("ctx.temperature()".to_string()),
        "$abstime" | "$realtime" if args.is_empty() => Ok("self.time".to_string()),
        "$mfactor" if args.is_empty() => Ok("multiplicity".to_string()),
        "$vt" | "$thermal_vt" if args.is_empty() => Ok("ctx.thermal_voltage()".to_string()),
        "$vt" | "$thermal_vt" if args.len() == 1 => {
            let temperature = emit_value_only_expr(artifact, parameter_fields, args[0])?;
            Ok(format!(
                "(({temperature}) * {})",
                format_f64(crate::canonical_ir::opt::THERMAL_VOLTAGE_PER_K)
            ))
        }
        "$param_given" if args.len() == 1 => {
            let parameter = idt_ic_parameter_arg(artifact, args[0]).ok_or_else(|| {
                unsupported(artifact, "$param_given idt initial condition argument")
            })?;
            Ok(format!(
                "if param_given[{}] {{ 1.0 }} else {{ 0.0 }}",
                usize::from(parameter)
            ))
        }
        "$port_connected" if args.len() == 1 => Ok("1.0".to_string()),
        _ => Err(unsupported(
            artifact,
            format!("idt initial condition system function '{name}'"),
        )),
    }
}

fn emit_value_only_call(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    name: &str,
    args: &[ExprId],
) -> Result<String, RustBackendError> {
    if name.eq_ignore_ascii_case("analysis") {
        return Ok("0.0".to_string());
    }
    let normalized = name.to_ascii_lowercase();
    let mut lowered = Vec::with_capacity(args.len());
    for arg in args {
        lowered.push(emit_value_only_expr(artifact, parameter_fields, *arg)?);
    }
    let value = match (normalized.as_str(), lowered.as_slice()) {
        ("abs" | "fabs", [arg]) => format!("{}.abs()", f64_method_receiver(arg)),
        ("sqrt", [arg]) => format!("{}.sqrt()", f64_method_receiver(arg)),
        ("exp", [arg]) => format!("{}.exp()", f64_method_receiver(arg)),
        ("limexp", [arg]) => format!(
            "{{ let limexp_arg = {arg}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ {} * (1.0 + (limexp_arg - 80.0)) }} }}",
            format_f64(LIMEXP_MAX)
        ),
        ("ln" | "log", [arg]) => format!("{}.ln()", f64_method_receiver(arg)),
        ("log10", [arg]) => format!("{}.log10()", f64_method_receiver(arg)),
        ("sin", [arg]) => format!("{}.sin()", f64_method_receiver(arg)),
        ("cos", [arg]) => format!("{}.cos()", f64_method_receiver(arg)),
        ("tan", [arg]) => format!("{}.tan()", f64_method_receiver(arg)),
        ("atan", [arg]) => format!("{}.atan()", f64_method_receiver(arg)),
        ("sinh", [arg]) => format!("{}.sinh()", f64_method_receiver(arg)),
        ("cosh", [arg]) => format!("{}.cosh()", f64_method_receiver(arg)),
        ("tanh", [arg]) => format!("{}.tanh()", f64_method_receiver(arg)),
        ("asinh", [arg]) => format!("{}.asinh()", f64_method_receiver(arg)),
        ("acosh", [arg]) => format!("{}.acosh()", f64_method_receiver(arg)),
        ("atanh", [arg]) => format!("{}.atanh()", f64_method_receiver(arg)),
        ("floor", [arg]) => format!("{}.floor()", f64_method_receiver(arg)),
        ("ceil", [arg]) => format!("{}.ceil()", f64_method_receiver(arg)),
        ("pow", [left, right]) => format!("{}.powf({right})", f64_method_receiver(left)),
        ("min", [left, right]) => format!("{}.min({right})", f64_method_receiver(left)),
        ("max", [left, right]) => format!("{}.max({right})", f64_method_receiver(left)),
        ("hypot", [left, right]) => format!("{}.hypot({right})", f64_method_receiver(left)),
        ("atan2", [left, right]) => format!("{}.atan2({right})", f64_method_receiver(left)),
        _ => {
            return Err(unsupported(
                artifact,
                format!("idt initial condition function '{name}'"),
            ));
        }
    };
    Ok(value)
}

fn idt_ic_parameter_arg(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
) -> Option<crate::canonical_ir::ParamId> {
    let expression = artifact.mir.expressions.get(usize::from(expr))?;
    let HirExprKind::Identifier { name } = &expression.kind else {
        return None;
    };
    artifact
        .mir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == name.as_str())
        .map(|parameter| parameter.id)
}

fn value_truth_expr(value: &str) -> String {
    format!("(({value}) != 0.0)")
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
            OptValueKind::ParamGiven { parameter } => {
                return Ok(format!(
                    "if param_given[{}] {{ 1.0 }} else {{ 0.0 }}",
                    usize::from(parameter)
                ));
            }
            OptValueKind::Temperature => return Ok(context.temperature_expr.clone()),
            OptValueKind::ThermalVoltage => return Ok(context.thermal_voltage_expr.clone()),
            OptValueKind::Multiplicity => return Ok("multiplicity".to_string()),
            OptValueKind::Time => return Ok("self.time".to_string()),
            OptValueKind::Analysis { ref query } => {
                return Ok(format!(
                    "if {} {{ 1.0 }} else {{ 0.0 }}",
                    analysis_predicate_expr(query.as_str())
                ));
            }
            _ => {}
        }
    }

    Ok(value_name(value))
}

fn value_type(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
) -> Result<OptValueType, RustBackendError> {
    artifact
        .opt
        .values
        .get(usize::from(value))
        .map(|value| value.value_type)
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))
}

fn emit_unary_expr(
    op: OptUnaryOp,
    input: String,
    input_type: OptValueType,
    limexp_max: &str,
) -> String {
    match op {
        OptUnaryOp::Pos => input,
        OptUnaryOp::Neg => format!("(-{input})"),
        OptUnaryOp::Not => format!("(!{})", truth_expr(input, input_type)),
        OptUnaryOp::Exp => format!("{}.exp()", f64_method_receiver(&input)),
        OptUnaryOp::LimExp => format!(
            "{{ let limexp_arg = {input}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ {limexp_max} * (1.0 + (limexp_arg - 80.0)) }} }}"
        ),
        OptUnaryOp::LimExpDerivative => format!(
            "{{ let limexp_arg = {input}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ {limexp_max} }} }}"
        ),
        OptUnaryOp::LimitedExp => format!(
            "{{ let limited_exp_arg = {input}; if limited_exp_arg > 80.0 {{ {limexp_max} * (1.0 + limited_exp_arg - 80.0) }} else if limited_exp_arg < -80.0 {{ 1.804851387e-35 }} else {{ limited_exp_arg.exp() }} }}"
        ),
        OptUnaryOp::LimitedExpDerivative => format!(
            "{{ let limited_exp_arg = {input}; if limited_exp_arg > 80.0 {{ {limexp_max} }} else if limited_exp_arg < -80.0 {{ 0.0 }} else {{ limited_exp_arg.exp() }} }}"
        ),
        OptUnaryOp::Ln => format!("{}.ln()", f64_method_receiver(&input)),
        OptUnaryOp::Sqrt => format!("{}.sqrt()", f64_method_receiver(&input)),
        OptUnaryOp::Abs => format!("{}.abs()", f64_method_receiver(&input)),
        OptUnaryOp::Sin => format!("{}.sin()", f64_method_receiver(&input)),
        OptUnaryOp::Cos => format!("{}.cos()", f64_method_receiver(&input)),
        OptUnaryOp::Tan => format!("{}.tan()", f64_method_receiver(&input)),
        OptUnaryOp::Sinh => format!("{}.sinh()", f64_method_receiver(&input)),
        OptUnaryOp::Cosh => format!("{}.cosh()", f64_method_receiver(&input)),
        OptUnaryOp::Tanh => format!("{}.tanh()", f64_method_receiver(&input)),
        OptUnaryOp::Atan => format!("{}.atan()", f64_method_receiver(&input)),
        OptUnaryOp::Asinh => format!("{}.asinh()", f64_method_receiver(&input)),
        OptUnaryOp::Floor => format!("{}.floor()", f64_method_receiver(&input)),
        OptUnaryOp::Ceil => format!("{}.ceil()", f64_method_receiver(&input)),
    }
}

fn emit_binary_expr(
    op: OptBinaryOp,
    left: String,
    left_type: OptValueType,
    right: String,
    right_type: OptValueType,
) -> String {
    match op {
        OptBinaryOp::Add => format!("({left} + {right})"),
        OptBinaryOp::Sub => format!("({left} - {right})"),
        OptBinaryOp::Mul => format!("({left} * {right})"),
        OptBinaryOp::Div => format!("({left} / {right})"),
        OptBinaryOp::Pow => format!("f64::powf({left}, {right})"),
        OptBinaryOp::Eq => format!("({left} == {right})"),
        OptBinaryOp::Ne => format!("({left} != {right})"),
        OptBinaryOp::Lt => format!("({left} < {right})"),
        OptBinaryOp::Le => format!("({left} <= {right})"),
        OptBinaryOp::Gt => format!("({left} > {right})"),
        OptBinaryOp::Ge => format!("({left} >= {right})"),
        OptBinaryOp::And => format!(
            "({} && {})",
            truth_expr(left, left_type),
            truth_expr(right, right_type)
        ),
        OptBinaryOp::Or => format!(
            "({} || {})",
            truth_expr(left, left_type),
            truth_expr(right, right_type)
        ),
    }
}

fn truth_expr(expr: String, value_type: OptValueType) -> String {
    match value_type {
        OptValueType::Boolean => expr,
        OptValueType::Real => format!("({expr} != 0.0)"),
    }
}

fn ddt_equation_roots(
    artifact: &CanonicalIrArtifact,
    roots: &HashMap<EquationId, ValueId>,
) -> HashMap<EquationId, ValueId> {
    artifact
        .mir
        .equations
        .iter()
        .filter(|equation| equation_ddt_expr(artifact, equation).is_ok_and(|expr| expr.is_some()))
        .filter_map(|equation| {
            roots
                .get(&equation.id)
                .copied()
                .map(|root| (equation.id, root))
        })
        .collect()
}

fn equation_transient_operator(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
) -> Result<Option<ScalarTransientOperator>, RustBackendError> {
    if let Some(expr) = equation_ddt_expr(artifact, equation)? {
        return Ok(Some(ScalarTransientOperator::Ddt { operator: expr }));
    }
    equation_idt_expr(artifact, equation)
}

fn equation_ddt_expr(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
) -> Result<Option<ExprId>, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(equation.expression.id))
        .ok_or_else(|| {
            unsupported(
                artifact,
                format!("missing equation expression {}", equation.expression.id),
            )
        })?;
    match &expression.kind {
        HirExprKind::AnalogOperator {
            op:
                HirAnalogOperator::Ddt {
                    expr: _,
                    abstol: Some(_),
                },
        } => Err(unsupported(artifact, "ddt abstol argument")),
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Ddt { .. },
        } => Ok(Some(equation.expression.id)),
        HirExprKind::Call { name, args } if name.eq_ignore_ascii_case("ddt") => {
            if args.len() == 1 {
                Ok(Some(equation.expression.id))
            } else {
                Err(unsupported(
                    artifact,
                    format!("ddt expects one operand, found {}", args.len()),
                ))
            }
        }
        _ => Ok(None),
    }
}

fn equation_idt_expr(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
) -> Result<Option<ScalarTransientOperator>, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(equation.expression.id))
        .ok_or_else(|| {
            unsupported(
                artifact,
                format!("missing equation expression {}", equation.expression.id),
            )
        })?;
    match &expression.kind {
        HirExprKind::AnalogOperator {
            op:
                HirAnalogOperator::Idt {
                    ic: _,
                    assert: Some(_),
                    ..
                },
        } => Err(unsupported(artifact, "idt assert argument")),
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Idt {
                abstol: Some(_), ..
            },
        } => Err(unsupported(artifact, "idt abstol argument")),
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Idt { expr: _, ic, .. },
        } => Ok(Some(ScalarTransientOperator::Idt {
            operator: equation.expression.id,
            ic: *ic,
        })),
        HirExprKind::Call { name, args } if name.eq_ignore_ascii_case("idt") => {
            match args.as_slice() {
                [_expr] => Ok(Some(ScalarTransientOperator::Idt {
                    operator: equation.expression.id,
                    ic: None,
                })),
                [_expr, ic] => Ok(Some(ScalarTransientOperator::Idt {
                    operator: equation.expression.id,
                    ic: Some(*ic),
                })),
                _ => Err(unsupported(
                    artifact,
                    format!("idt expects one or two operands, found {}", args.len()),
                )),
            }
        }
        _ => Ok(None),
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
        validate_scalar_equation(artifact, equation, root)?;
    }

    Ok(roots)
}

fn validate_scalar_equation(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
) -> Result<(), RustBackendError> {
    match equation.kind {
        MirEquationKind::Current => validate_scalar_current_equation(artifact, equation, root),
        MirEquationKind::Potential => validate_scalar_potential_equation(artifact, equation, root),
        MirEquationKind::Indirect => Err(unsupported(artifact, "indirect contributions")),
    }
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
    validate_scalar_value_graph(artifact, equation, root)
}

fn validate_scalar_potential_equation(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
) -> Result<(), RustBackendError> {
    if equation.kind != MirEquationKind::Potential {
        return Err(unsupported(artifact, "non-potential equations"));
    }
    potential_branch_slot(artifact, equation)?;
    validate_scalar_value_graph(artifact, equation, root)
}

fn validate_scalar_value_graph(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    root: ValueId,
) -> Result<(), RustBackendError> {
    let roots = HashMap::from([(equation.id, root)]);
    let empty_cache = ScalarStaticCache {
        instance_values: Vec::new(),
        temperature_values: Vec::new(),
        set: HashSet::new(),
    };
    let live = collect_stamp_live_values(artifact, &roots, &empty_cache)?;
    for value in live {
        let value_slot = artifact
            .opt
            .values
            .get(usize::from(value))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
        match value_slot.kind {
            OptValueKind::BranchFlow { branch } => {
                branch_flow_slot(artifact, branch)?;
            }
            OptValueKind::EquationValue { .. } => {
                return Err(unsupported(
                    artifact,
                    "legacy equation values in scalar OptIR",
                ));
            }
            _ => {}
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
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::EquationValue { .. } => {}
        OptValueKind::Ddx {
            value: input,
            pos_node,
            neg_node,
        } => {
            mark_stamp_live_value(artifact, input, static_cache, live)?;
            for derivative in projected_ddx_derivative_values(artifact, input, pos_node, neg_node)?
            {
                mark_stamp_live_value(artifact, derivative, static_cache, live)?;
            }
        }
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum ValueVisitState {
    Unvisited,
    Visiting,
    Done,
}

fn ordered_live_values(
    artifact: &CanonicalIrArtifact,
    live: &HashSet<ValueId>,
    static_cache: &ScalarStaticCache,
) -> Result<Vec<ValueId>, RustBackendError> {
    let mut state = vec![ValueVisitState::Unvisited; artifact.opt.values.len()];
    let mut ordered = Vec::with_capacity(live.len());

    for value in &artifact.opt.values {
        if live.contains(&value.id) && !static_cache.contains(value.id) {
            visit_live_value(
                artifact,
                value.id,
                live,
                static_cache,
                &mut state,
                &mut ordered,
            )?;
        }
    }

    Ok(ordered)
}

fn visit_live_value(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    live: &HashSet<ValueId>,
    static_cache: &ScalarStaticCache,
    state: &mut [ValueVisitState],
    ordered: &mut Vec<ValueId>,
) -> Result<(), RustBackendError> {
    if static_cache.contains(value) || !live.contains(&value) {
        return Ok(());
    }

    let index = usize::from(value);
    match state
        .get(index)
        .copied()
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?
    {
        ValueVisitState::Done => return Ok(()),
        ValueVisitState::Visiting => {
            return Err(unsupported(
                artifact,
                format!("cyclic scalar value dependency at {value}"),
            ));
        }
        ValueVisitState::Unvisited => {}
    }

    state[index] = ValueVisitState::Visiting;
    let value_slot = artifact
        .opt
        .values
        .get(index)
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
    match value_slot.kind {
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::EquationValue { .. } => {}
        OptValueKind::Ddx {
            value: input,
            pos_node,
            neg_node,
        } => {
            visit_live_value(artifact, input, live, static_cache, state, ordered)?;
            for derivative in projected_ddx_derivative_values(artifact, input, pos_node, neg_node)?
            {
                visit_live_value(artifact, derivative, live, static_cache, state, ordered)?;
            }
        }
        OptValueKind::Unary { input, .. } => {
            visit_live_value(artifact, input, live, static_cache, state, ordered)?;
        }
        OptValueKind::Binary { left, right, .. } => {
            visit_live_value(artifact, left, live, static_cache, state, ordered)?;
            visit_live_value(artifact, right, live, static_cache, state, ordered)?;
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            visit_live_value(artifact, condition, live, static_cache, state, ordered)?;
            visit_live_value(artifact, then_value, live, static_cache, state, ordered)?;
            visit_live_value(artifact, else_value, live, static_cache, state, ordered)?;
        }
    }
    state[index] = ValueVisitState::Done;
    ordered.push(value);
    Ok(())
}

fn projected_ddx_derivative_values(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    pos_node: Option<NodeId>,
    neg_node: Option<NodeId>,
) -> Result<Vec<ValueId>, RustBackendError> {
    let mut derivatives = Vec::new();
    for node in [pos_node, neg_node].into_iter().flatten() {
        if let Some(derivative) =
            derivative_value_for_lane(artifact, value, DerivativeLane::node(node))?
        {
            derivatives.push(derivative);
        }
    }
    Ok(derivatives)
}

fn derivative_value_for_lane(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    lane: DerivativeLane,
) -> Result<Option<ValueId>, RustBackendError> {
    let value_slot = artifact
        .opt
        .values
        .get(usize::from(value))
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
    Ok(value_slot
        .derivatives
        .iter()
        .find(|derivative| derivative.lane == lane)
        .map(|derivative| derivative.value))
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
        reject_unsupported_scalar_statement_shape(artifact, statement)?;
    }
    if !artifact.mir.state_slots.is_empty() {
        return Err(unsupported(artifact, "state slots"));
    }
    for equation in &artifact.mir.equations {
        match equation.kind {
            MirEquationKind::Current => {}
            MirEquationKind::Potential => {
                potential_branch_slot(artifact, equation)?;
            }
            MirEquationKind::Indirect => {
                return Err(unsupported(artifact, "indirect contributions"));
            }
        }
    }
    for value in &artifact.opt.values {
        match value.kind {
            OptValueKind::BranchFlow { branch } => {
                branch_flow_slot(artifact, branch)?;
            }
            OptValueKind::ParamGiven { .. }
            | OptValueKind::Temperature
            | OptValueKind::ThermalVoltage
            | OptValueKind::Multiplicity
            | OptValueKind::Time
            | OptValueKind::Analysis { .. }
            | OptValueKind::Ddx { .. } => {}
            OptValueKind::EquationValue { .. } => {
                return Err(unsupported(
                    artifact,
                    "legacy equation values in scalar OptIR",
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn reject_unsupported_scalar_statement_shape(
    artifact: &CanonicalIrArtifact,
    statement: &HirStatement,
) -> Result<(), RustBackendError> {
    match statement {
        HirStatement::Assignment(assignment)
            if assignment.index.is_none() && supported_scalar_value_type(assignment.expr_type) =>
        {
            Ok(())
        }
        HirStatement::Assignment(assignment) if assignment.index.is_some() => {
            Err(unsupported(artifact, "indexed assignments"))
        }
        HirStatement::Assignment(assignment) => Err(unsupported(
            artifact,
            format!(
                "assignment '{}' with type {:?}",
                assignment.target_name, assignment.expr_type
            ),
        )),
        HirStatement::Loop(loop_statement) => {
            for statement in &loop_statement.body {
                reject_unsupported_scalar_statement_shape(artifact, statement)?;
            }
            Ok(())
        }
    }
}

fn supported_scalar_value_type(value_type: CanonicalValueType) -> bool {
    matches!(
        value_type,
        CanonicalValueType::Real
            | CanonicalValueType::Integer
            | CanonicalValueType::Boolean
            | CanonicalValueType::NatureAccess
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

fn branch_derivative_name(root: ValueId, branch: u32) -> String {
    format!("d{}_db{branch}", root.index())
}

fn scaled_derivative_expr(derivative: String, scale: &str) -> String {
    if scale == "1.0" {
        derivative
    } else {
        format!("(({derivative}) * {scale})")
    }
}

fn optional_node_local_expr(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some({})", node.index()))
        .unwrap_or_else(|| "None".to_string())
}

fn optional_node_global_expr(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some(nodes[{}])", node.index()))
        .unwrap_or_else(|| "None".to_string())
}

fn potential_branch_slot(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
) -> Result<usize, RustBackendError> {
    artifact
        .mir
        .branch_unknowns
        .iter()
        .find(|unknown| unknown.equation == equation.id)
        .map(|unknown| usize::from(unknown.id))
        .ok_or_else(|| {
            unsupported(
                artifact,
                format!("potential equation {} has no branch unknown", equation.id),
            )
        })
}

fn branch_flow_slot(
    artifact: &CanonicalIrArtifact,
    branch_id: BranchId,
) -> Result<usize, RustBackendError> {
    let branch = artifact
        .mir
        .branches
        .get(usize::from(branch_id))
        .ok_or_else(|| unsupported(artifact, format!("missing branch {branch_id}")))?;

    if let Some(unknown) = artifact.mir.branch_unknowns.iter().find(|unknown| {
        unknown
            .declared_name
            .as_deref()
            .is_some_and(|name| name == branch.name.as_str())
    }) {
        return Ok(usize::from(unknown.id));
    }

    let mut matches = artifact.mir.branch_unknowns.iter().filter(|unknown| {
        unknown.pos_node == branch.pos_node && unknown.neg_node == branch.neg_node
    });
    let Some(first) = matches.next() else {
        return Err(unsupported(
            artifact,
            format!(
                "branch current probe '{}' has no branch unknown",
                branch.name
            ),
        ));
    };
    if matches.next().is_some() {
        return Err(unsupported(
            artifact,
            format!(
                "branch current probe '{}' matches multiple branch unknowns",
                branch.name
            ),
        ));
    }
    Ok(usize::from(first.id))
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

fn f64_method_receiver(value: &str) -> String {
    format!("(({value}) as f64)")
}
