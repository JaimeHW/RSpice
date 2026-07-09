use std::collections::{HashMap, HashSet};

use crate::canonical_ir::opt::LIMEXP_MAX;
use crate::canonical_ir::{
    BranchId, BranchUnknownId, CanonicalIrArtifact, CanonicalValueType, DerivativeLane,
    DerivativeLaneKind, EquationId, ExprId, HirAnalogOperator, HirExprKind, InvalidationClass,
    MirEquation, MirEquationKind, NodeId, OptBinaryOp, OptOp, OptUnaryOp, OptValue, OptValueKind,
    OptValueType, ValueId,
};

use super::expr::{DdtSlots, LoweredVariable, analysis_predicate_expr, parameter_field_names};
use super::{GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames};
use super::{RustTranspileOptions, device};

const SPARSE_STAMP_DERIVATIVE_THRESHOLD: usize = 10;
const MAX_SCALAR_STAMP_LIVE_VALUES: usize = 1_000_000;
const MAX_SCALAR_STAMP_EMITTED_VALUES: usize = 1_000_000;
const MAX_SCALAR_STAMP_SOURCE_BYTES: usize = 16 * 1024 * 1024;
const MAX_SCALAR_STAMP_SOURCE_LINES: usize = 120_000;
const SCALAR_STAMP_SOURCE_LINE_OVERHEAD_RESERVE: usize = 1024;
const MIN_COMPACT_SCALAR_VALUE_BINDINGS_PER_LINE: usize = 8;
const MAX_SCALAR_RUNTIME_LOOP_ASSIGNMENTS: usize = 8_192;
const MAX_SCALAR_RUNTIME_LOOP_VARIABLES: usize = 1_024;
const MAX_SCALAR_INLINE_EXPR_COST: usize = 128;
const MAX_COMPACT_LET_LINE_BYTES: usize = 4096;
const MIN_SHARED_STAMP_LIVE_VALUES: usize = 1024;
const MIN_ALIASED_SCALAR_STATIC_CACHE_VALUES: usize = 64;
const MIN_LOCAL_SHARED_STAMP_VALUES: usize = 64;

pub(super) struct ScalarStaticCache {
    instance_values: Vec<ValueId>,
    temperature_values: Vec<ValueId>,
    set: HashSet<ValueId>,
    refs: HashMap<ValueId, String>,
    f64_count: usize,
    bool_count: usize,
}

impl ScalarStaticCache {
    fn from_artifact(artifact: &CanonicalIrArtifact) -> Result<Self, RustBackendError> {
        let instance_values = scheduled_values(artifact, InvalidationClass::InstanceStatic, None);
        let temperature_values =
            scheduled_values(artifact, InvalidationClass::TemperatureStatic, None);
        Self::with_values(artifact, instance_values, temperature_values)
    }

    pub(super) fn from_roots(
        artifact: &CanonicalIrArtifact,
        roots: &HashMap<EquationId, ValueId>,
    ) -> Result<Self, RustBackendError> {
        let empty_cache = Self::empty();
        let live = collect_stamp_live_values(artifact, roots, &empty_cache)?;
        let instance_values =
            scheduled_values(artifact, InvalidationClass::InstanceStatic, Some(&live));
        let temperature_values =
            scheduled_values(artifact, InvalidationClass::TemperatureStatic, Some(&live));
        Self::with_values(artifact, instance_values, temperature_values)
    }

    fn empty() -> Self {
        Self {
            instance_values: Vec::new(),
            temperature_values: Vec::new(),
            set: HashSet::new(),
            refs: HashMap::new(),
            f64_count: 0,
            bool_count: 0,
        }
    }

    fn with_values(
        artifact: &CanonicalIrArtifact,
        instance_values: Vec<ValueId>,
        temperature_values: Vec<ValueId>,
    ) -> Result<Self, RustBackendError> {
        let mut set = HashSet::new();
        let mut refs = HashMap::new();
        let mut f64_count = 0;
        let mut bool_count = 0;
        for value_id in instance_values.iter().chain(temperature_values.iter()) {
            if !set.insert(*value_id) {
                continue;
            }
            let value = artifact
                .opt
                .values
                .get(usize::from(*value_id))
                .ok_or_else(|| {
                    unsupported(artifact, format!("missing static scalar value {value_id}"))
                })?;
            let reference = match value.value_type {
                OptValueType::Real => {
                    let slot = f64_count;
                    f64_count += 1;
                    format!("self.scalar_static_f64[{slot}]")
                }
                OptValueType::Boolean => {
                    let slot = bool_count;
                    bool_count += 1;
                    format!("self.scalar_static_bool[{slot}]")
                }
            };
            refs.insert(*value_id, reference);
        }
        Ok(Self {
            instance_values,
            temperature_values,
            set,
            refs,
            f64_count,
            bool_count,
        })
    }

    fn contains(&self, value: ValueId) -> bool {
        self.set.contains(&value)
    }

    fn cache_ref(&self, value: ValueId) -> Option<&str> {
        self.refs.get(&value).map(String::as_str)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.instance_values.is_empty() && self.temperature_values.is_empty()
    }

    pub(super) fn has_temperature_values(&self) -> bool {
        !self.temperature_values.is_empty()
    }

    fn has_f64_values(&self) -> bool {
        self.f64_count > 0
    }

    fn has_bool_values(&self) -> bool {
        self.bool_count > 0
    }
}

fn use_scalar_static_cache_aliases(static_cache: &ScalarStaticCache) -> bool {
    static_cache
        .f64_count
        .saturating_add(static_cache.bool_count)
        >= MIN_ALIASED_SCALAR_STATIC_CACHE_VALUES
}

fn scalar_static_cache_refs_for_stamp(
    static_cache: &ScalarStaticCache,
) -> HashMap<ValueId, String> {
    if !use_scalar_static_cache_aliases(static_cache) {
        return static_cache.refs.clone();
    }

    static_cache
        .refs
        .iter()
        .map(|(value, reference)| {
            let reference = if let Some(suffix) = reference.strip_prefix("self.scalar_static_f64") {
                format!("sf{suffix}")
            } else if let Some(suffix) = reference.strip_prefix("self.scalar_static_bool") {
                format!("sb{suffix}")
            } else {
                reference.clone()
            };
            (*value, reference)
        })
        .collect()
}

fn emit_scalar_static_cache_aliases(static_cache: &ScalarStaticCache, out: &mut String) {
    if !use_scalar_static_cache_aliases(static_cache) {
        return;
    }
    if static_cache.has_f64_values() {
        out.push_str("        let sf=&self.scalar_static_f64;\n");
    }
    if static_cache.has_bool_values() {
        out.push_str("        let sb=&self.scalar_static_bool;\n");
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
    cached_value_refs: &'a HashMap<ValueId, String>,
    inline_values: &'a HashSet<ValueId>,
    use_cached_fields: bool,
    inline_uncached_constants: bool,
    use_exp_helpers: bool,
    limexp_max_expr: String,
    temperature_expr: String,
    thermal_voltage_expr: String,
    ddt_slots: Option<&'a DdtSlots>,
    ddt_mode: DdtEmitMode,
    node_array_expr: &'static str,
    branch_array_expr: &'static str,
    param_given_expr: &'static str,
    multiplicity_expr: &'static str,
    loop_index_exprs: HashMap<u32, String>,
    runtime_loop_values: HashMap<(u32, u32), String>,
    runtime_loop_derivatives: HashMap<(u32, u32, DerivativeLane), String>,
    external_value_refs: HashMap<ValueId, String>,
}

struct CompactLetEmitter {
    indent: &'static str,
    line: String,
}

impl CompactLetEmitter {
    fn new(indent: &'static str) -> Self {
        Self {
            indent,
            line: String::new(),
        }
    }

    fn push(&mut self, out: &mut String, name: &str, expr: &str) {
        let statement = format!("let {name}={expr};");
        self.push_statement(out, &statement);
    }

    fn push_mut_typed(&mut self, out: &mut String, name: &str, ty: &str, expr: &str) {
        let statement = format!("let mut {name}:{ty}={expr};");
        self.push_statement(out, &statement);
    }

    fn push_statement(&mut self, out: &mut String, statement: &str) {
        if self.line.is_empty() {
            self.line.push_str(self.indent);
            self.line.push_str(&statement);
            return;
        }
        if self.line.len().saturating_add(statement.len()) > MAX_COMPACT_LET_LINE_BYTES {
            self.flush(out);
            self.line.push_str(self.indent);
            self.line.push_str(&statement);
        } else {
            self.line.push_str(&statement);
        }
    }

    fn flush(&mut self, out: &mut String) {
        if self.line.is_empty() {
            return;
        }
        out.push_str(&self.line);
        out.push('\n');
        self.line.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DdtEmitMode {
    Transient,
    ReactiveLinearized,
}

fn local_stamp_context<'a>(
    static_cache: &'a ScalarStaticCache,
    inline_values: &'a HashSet<ValueId>,
    ddt_slots: Option<&'a DdtSlots>,
    ddt_mode: DdtEmitMode,
) -> ValueEmitContext<'a> {
    ValueEmitContext {
        cached_values: &static_cache.set,
        cached_value_refs: &static_cache.refs,
        inline_values,
        use_cached_fields: true,
        inline_uncached_constants: false,
        use_exp_helpers: true,
        limexp_max_expr: "LIMEXP_MAX".to_string(),
        temperature_expr: "ctx.temperature()".to_string(),
        thermal_voltage_expr: "ctx.thermal_voltage()".to_string(),
        ddt_slots,
        ddt_mode,
        node_array_expr: "nodes",
        branch_array_expr: "branches",
        param_given_expr: "param_given",
        multiplicity_expr: "multiplicity",
        loop_index_exprs: HashMap::new(),
        runtime_loop_values: HashMap::new(),
        runtime_loop_derivatives: HashMap::new(),
        external_value_refs: HashMap::new(),
    }
}

pub(super) struct SharedStampValuesPlan {
    pub(super) live: HashSet<ValueId>,
    boundary: Vec<ValueId>,
    refs: HashMap<ValueId, String>,
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
    let static_cache = ScalarStaticCache::from_artifact(artifact)?;
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
    if stamp_live.len() > MAX_SCALAR_STAMP_LIVE_VALUES {
        return Err(unsupported(
            artifact,
            format!(
                "scalar OptIR stamp graph has {} live values; current scalar emitter limit is {}",
                stamp_live.len(),
                MAX_SCALAR_STAMP_LIVE_VALUES
            ),
        ));
    }
    let reactive_roots = reactive_equation_roots(artifact, &roots)?;
    let reactive_live = if reactive_roots.is_empty() {
        HashSet::new()
    } else {
        collect_stamp_live_values(artifact, &reactive_roots, static_cache)?
    };
    let shared_plan = if reactive_roots.is_empty() {
        None
    } else {
        shared_stamp_values_plan(
            artifact,
            &stamp_live,
            &reactive_live,
            static_cache,
            &roots,
            &reactive_roots,
        )?
    };
    let stamp_emit_live = shared_plan
        .as_ref()
        .map(|plan| tail_live_values(&stamp_live, &plan.live))
        .unwrap_or_else(|| stamp_live.clone());
    let reactive_emit_live = shared_plan
        .as_ref()
        .map(|plan| tail_live_values(&reactive_live, &plan.live))
        .unwrap_or_else(|| reactive_live.clone());
    let stamp_inline_values =
        scalar_stamp_inline_values(artifact, &stamp_emit_live, static_cache, &roots)?;
    let reactive_inline_values = if reactive_roots.is_empty() {
        HashSet::new()
    } else {
        scalar_stamp_inline_values(artifact, &reactive_emit_live, static_cache, &reactive_roots)?
    };
    let common_inline_values = shared_plan
        .as_ref()
        .map(|plan| shared_stamp_inline_values(artifact, static_cache, plan))
        .transpose()?;
    reject_oversized_scalar_stamp_value_emit(
        artifact,
        static_cache,
        shared_plan.as_ref(),
        &stamp_emit_live,
        &stamp_inline_values,
        &reactive_emit_live,
        &reactive_inline_values,
        common_inline_values.as_ref(),
    )?;
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
        stamp_live.contains(&value.id)
            && matches!(
                value.kind,
                OptValueKind::BranchFlow { .. } | OptValueKind::BranchUnknownFlow { .. }
            )
    });
    let mut out = String::new();
    out.push_str("#![allow(dead_code, unused_imports, unused_parens, unused_variables)]\n\n");
    out.push_str("use super::state::Instance;\n");
    out.push_str(&format!(
        "use {}::{{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};\n\n",
        options.runtime_path
    ));
    let exp_helpers = scalar_exp_helper_usage(artifact);
    if !exp_helpers.is_empty() {
        out.push_str(&format!(
            "const LIMEXP_MAX: f64 = {};\n\n",
            format_f64(LIMEXP_MAX)
        ));
        emit_scalar_exp_helpers(&mut out, exp_helpers);
    }
    if ddt_slots.len() > 0 || has_idt_slots {
        emit_transient_state_helpers(ddt_slots, &mut out);
    }
    if let Some(plan) = &shared_plan {
        emit_shared_stamp_values_struct(artifact, plan, &mut out)?;
    }
    out.push_str("impl Instance {\n");
    if let Some(plan) = &shared_plan {
        emit_shared_stamp_values_method(
            artifact,
            parameter_fields,
            static_cache,
            ddt_slots,
            plan,
            true,
            &mut out,
        )?;
    }
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
    );
    out.push_str("        let n=self.nodes;\n");
    out.push_str("        let nodes=n;\n");
    if stamp_needs_branches {
        out.push_str("        let br=self.branches;\n");
        out.push_str("        let branches=br;\n");
    }
    if static_cache.has_temperature_values() {
        out.push_str(
            "        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());\n",
        );
    }
    let stamp_shared_refs = shared_plan
        .as_ref()
        .map(shared_stamp_value_refs)
        .unwrap_or_default();
    if let Some(plan) = &shared_plan {
        emit_shared_stamp_values_binding(plan, &mut out);
    }
    if stamp_needs_params {
        out.push_str("        let p=&(*self.params);\n");
    }
    if stamp_needs_param_given {
        out.push_str("        let pg=self.param_given.as_ref();\n");
        out.push_str("        let param_given=pg;\n");
    }
    out.push_str("        let m=self.multiplicity;\n");
    out.push_str("        let multiplicity=m;\n");
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
    let stamp_static_refs = scalar_static_cache_refs_for_stamp(static_cache);
    emit_scalar_static_cache_aliases(static_cache, &mut out);

    let stamp_context = ValueEmitContext {
        cached_values: &static_cache.set,
        cached_value_refs: &stamp_static_refs,
        inline_values: &stamp_inline_values,
        use_cached_fields: true,
        inline_uncached_constants: false,
        use_exp_helpers: true,
        limexp_max_expr: "LIMEXP_MAX".to_string(),
        temperature_expr: "ctx.temperature()".to_string(),
        thermal_voltage_expr: "ctx.thermal_voltage()".to_string(),
        ddt_slots: Some(ddt_slots),
        ddt_mode: DdtEmitMode::Transient,
        node_array_expr: "n",
        branch_array_expr: "br",
        param_given_expr: "pg",
        multiplicity_expr: "m",
        loop_index_exprs: HashMap::new(),
        runtime_loop_values: HashMap::new(),
        runtime_loop_derivatives: HashMap::new(),
        external_value_refs: stamp_shared_refs,
    };
    emit_live_values(
        artifact,
        parameter_fields,
        &stamp_emit_live,
        static_cache,
        &stamp_context,
        true,
        &mut out,
    )?;

    emit_current_stamps(
        artifact,
        parameter_fields,
        &roots,
        &stamp_context,
        Some(ddt_slots),
        true,
        &mut out,
    )?;
    reject_oversized_scalar_stamp_source_bytes_so_far(artifact, &out)?;

    out.push_str("    }\n\n");
    if reactive_roots.is_empty() {
        out.push_str(
            "    pub fn stamp_reactive(&mut self, _ctx: &GeneratedEvalContext<'_>, _stamper: &mut GeneratedReactiveStamper<'_>) {\n",
        );
        out.push_str("    }\n");
    } else {
        let reactive_needs_param_given = artifact.opt.values.iter().any(|value| {
            reactive_live.contains(&value.id)
                && matches!(value.kind, OptValueKind::ParamGiven { .. })
        });
        out.push_str(
            "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
        );
        out.push_str("        let n=self.nodes;\n");
        out.push_str("        let nodes=n;\n");
        out.push_str("        let br=self.branches;\n");
        out.push_str("        let branches=br;\n");
        if static_cache.has_temperature_values() {
            out.push_str(
                "        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());\n",
            );
        }
        let reactive_shared_refs = shared_plan
            .as_ref()
            .map(shared_stamp_value_refs)
            .unwrap_or_default();
        if let Some(plan) = &shared_plan {
            emit_shared_stamp_values_binding(plan, &mut out);
        }
        out.push_str("        let p=&(*self.params);\n");
        if reactive_needs_param_given {
            out.push_str("        let pg=self.param_given.as_ref();\n");
            out.push_str("        let param_given=pg;\n");
        }
        out.push_str("        let m=self.multiplicity;\n");
        out.push_str("        let multiplicity=m;\n");
        let reactive_static_refs = scalar_static_cache_refs_for_stamp(static_cache);
        emit_scalar_static_cache_aliases(static_cache, &mut out);
        let reactive_context = ValueEmitContext {
            cached_values: &static_cache.set,
            cached_value_refs: &reactive_static_refs,
            inline_values: &reactive_inline_values,
            use_cached_fields: true,
            inline_uncached_constants: false,
            use_exp_helpers: true,
            limexp_max_expr: "LIMEXP_MAX".to_string(),
            temperature_expr: "ctx.temperature()".to_string(),
            thermal_voltage_expr: "ctx.thermal_voltage()".to_string(),
            ddt_slots: Some(ddt_slots),
            ddt_mode: DdtEmitMode::ReactiveLinearized,
            node_array_expr: "n",
            branch_array_expr: "br",
            param_given_expr: "pg",
            multiplicity_expr: "m",
            loop_index_exprs: HashMap::new(),
            runtime_loop_values: HashMap::new(),
            runtime_loop_derivatives: HashMap::new(),
            external_value_refs: reactive_shared_refs,
        };
        emit_live_values(
            artifact,
            parameter_fields,
            &reactive_emit_live,
            static_cache,
            &reactive_context,
            true,
            &mut out,
        )?;
        emit_current_reactive_stamps_with_context(
            artifact,
            parameter_fields,
            &reactive_roots,
            &reactive_context,
            true,
            &mut out,
        )?;
        reject_oversized_scalar_stamp_source_bytes_so_far(artifact, &out)?;
        out.push_str("    }\n");
    }
    out.push_str("}\n");
    trace_scalar_stamp_source_size(artifact, &out);
    reject_oversized_scalar_stamp_source(artifact, &out)?;
    Ok(out)
}

fn trace_scalar_stamp_source_size(artifact: &CanonicalIrArtifact, source: &str) {
    if std::env::var_os("RSPICE_VERILOGA_SCALAR_STAMP_SOURCE_SIZE_TRACE").is_none() {
        return;
    }
    eprintln!(
        "scalar stamp source size {}: bytes={} lines={}",
        artifact.mir.module_name,
        source.len(),
        source.lines().count()
    );
}

fn reject_oversized_scalar_stamp_source(
    artifact: &CanonicalIrArtifact,
    source: &str,
) -> Result<(), RustBackendError> {
    let source_bytes = source.len();
    let source_lines = source.lines().count();
    if source_bytes <= MAX_SCALAR_STAMP_SOURCE_BYTES
        && source_lines <= MAX_SCALAR_STAMP_SOURCE_LINES
    {
        return Ok(());
    }

    Err(unsupported(
        artifact,
        format!(
            "pure scalar stamp source has {source_bytes} bytes and {source_lines} lines; current scalar source budget is {MAX_SCALAR_STAMP_SOURCE_BYTES} bytes and {MAX_SCALAR_STAMP_SOURCE_LINES} lines"
        ),
    ))
}

fn reject_oversized_scalar_stamp_source_bytes_so_far(
    artifact: &CanonicalIrArtifact,
    source: &str,
) -> Result<(), RustBackendError> {
    let source_bytes = source.len();
    if source_bytes <= MAX_SCALAR_STAMP_SOURCE_BYTES {
        return Ok(());
    }

    let source_lines = source.lines().count();
    Err(unsupported(
        artifact,
        format!(
            "pure scalar stamp source exceeded byte budget after {source_bytes} bytes and {source_lines} lines; current scalar source budget is {MAX_SCALAR_STAMP_SOURCE_BYTES} bytes and {MAX_SCALAR_STAMP_SOURCE_LINES} lines"
        ),
    ))
}

fn reject_oversized_scalar_stamp_value_emit(
    artifact: &CanonicalIrArtifact,
    static_cache: &ScalarStaticCache,
    shared_plan: Option<&SharedStampValuesPlan>,
    stamp_emit_live: &HashSet<ValueId>,
    stamp_inline_values: &HashSet<ValueId>,
    reactive_emit_live: &HashSet<ValueId>,
    reactive_inline_values: &HashSet<ValueId>,
    common_inline_values: Option<&HashSet<ValueId>>,
) -> Result<(), RustBackendError> {
    let emitted_values = scalar_stamp_value_emit_estimate(
        artifact,
        static_cache,
        shared_plan,
        stamp_emit_live,
        stamp_inline_values,
        reactive_emit_live,
        reactive_inline_values,
        common_inline_values,
    )?;
    if scalar_stamp_source_line_estimate_exceeds_budget(emitted_values) {
        let binding_lines = scalar_stamp_packed_binding_line_estimate(emitted_values);
        return Err(unsupported(
            artifact,
            format!(
                "pure scalar stamp would emit approximately {emitted_values} value bindings, estimated as at least {binding_lines} packed binding lines before stamp calls; current scalar source line budget is {MAX_SCALAR_STAMP_SOURCE_LINES}"
            ),
        ));
    }
    if !scalar_stamp_emitted_values_exceeds_budget(emitted_values) {
        return Ok(());
    }

    Err(unsupported(
        artifact,
        format!(
            "pure scalar stamp would emit approximately {emitted_values} value bindings; current scalar emission budget is {MAX_SCALAR_STAMP_EMITTED_VALUES}"
        ),
    ))
}

fn scalar_stamp_value_emit_estimate(
    artifact: &CanonicalIrArtifact,
    static_cache: &ScalarStaticCache,
    shared_plan: Option<&SharedStampValuesPlan>,
    stamp_emit_live: &HashSet<ValueId>,
    stamp_inline_values: &HashSet<ValueId>,
    reactive_emit_live: &HashSet<ValueId>,
    reactive_inline_values: &HashSet<ValueId>,
    common_inline_values: Option<&HashSet<ValueId>>,
) -> Result<usize, RustBackendError> {
    let empty = HashSet::new();
    let common_values = if let Some(plan) = shared_plan {
        scalar_live_value_emit_count(
            artifact,
            &plan.live,
            static_cache,
            common_inline_values.unwrap_or(&empty),
            &empty,
        )?
    } else {
        0
    };
    let shared_external_values = shared_plan
        .map(|plan| plan.boundary.iter().copied().collect::<HashSet<_>>())
        .unwrap_or_default();
    Ok(common_values
        .saturating_add(scalar_live_value_emit_count(
            artifact,
            stamp_emit_live,
            static_cache,
            stamp_inline_values,
            &shared_external_values,
        )?)
        .saturating_add(scalar_live_value_emit_count(
            artifact,
            reactive_emit_live,
            static_cache,
            reactive_inline_values,
            &shared_external_values,
        )?))
}

fn scalar_live_value_emit_count(
    artifact: &CanonicalIrArtifact,
    live: &HashSet<ValueId>,
    static_cache: &ScalarStaticCache,
    inline_values: &HashSet<ValueId>,
    external_values: &HashSet<ValueId>,
) -> Result<usize, RustBackendError> {
    let values = ordered_live_values(artifact, live, static_cache)?;
    let mut emitted_runtime_loops = HashSet::new();
    let mut count = 0usize;
    for value_id in values {
        if external_values.contains(&value_id) || inline_values.contains(&value_id) {
            continue;
        }
        let value = artifact
            .opt
            .values
            .get(usize::from(value_id))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value_id}")))?;
        if let Some(loop_id) = runtime_loop_result_id(&value.kind) {
            if emitted_runtime_loops.insert(loop_id) {
                count = count.saturating_add(runtime_loop_source_binding_estimate(
                    artifact,
                    loop_id,
                    live,
                    &static_cache.set,
                )?);
            }
            continue;
        }
        if matches!(
            value.kind,
            OptValueKind::RuntimeLoopVariable { .. }
                | OptValueKind::RuntimeLoopVariableDerivative { .. }
        ) {
            continue;
        }
        count = count.saturating_add(1);
    }
    Ok(count)
}

pub(super) fn scalar_stamp_emitted_values_exceeds_budget(emitted_values: usize) -> bool {
    emitted_values > MAX_SCALAR_STAMP_EMITTED_VALUES
}

fn scalar_stamp_source_line_estimate_exceeds_budget(emitted_values: usize) -> bool {
    scalar_stamp_packed_binding_line_estimate(emitted_values)
        .saturating_add(SCALAR_STAMP_SOURCE_LINE_OVERHEAD_RESERVE)
        > MAX_SCALAR_STAMP_SOURCE_LINES
}

fn scalar_stamp_packed_binding_line_estimate(emitted_values: usize) -> usize {
    emitted_values.saturating_add(MIN_COMPACT_SCALAR_VALUE_BINDINGS_PER_LINE - 1)
        / MIN_COMPACT_SCALAR_VALUE_BINDINGS_PER_LINE
}

fn shared_stamp_values_plan(
    artifact: &CanonicalIrArtifact,
    stamp_live: &HashSet<ValueId>,
    reactive_live: &HashSet<ValueId>,
    static_cache: &ScalarStaticCache,
    roots: &HashMap<EquationId, ValueId>,
    reactive_roots: &HashMap<EquationId, ValueId>,
) -> Result<Option<SharedStampValuesPlan>, RustBackendError> {
    let live = shareable_common_stamp_values(artifact, stamp_live, reactive_live, static_cache)?;
    if live.len() < MIN_SHARED_STAMP_LIVE_VALUES {
        return Ok(None);
    }

    let mut boundary = HashSet::new();
    for value in stamp_live.iter().chain(reactive_live.iter()).copied() {
        if live.contains(&value) || static_cache.contains(value) {
            continue;
        }
        for dependency in scalar_value_dependencies(artifact, value)? {
            if live.contains(&dependency) {
                boundary.insert(dependency);
            }
        }
    }
    for value in stamp_external_values(artifact, roots)?
        .into_iter()
        .chain(stamp_external_values(artifact, reactive_roots)?)
    {
        if live.contains(&value) {
            boundary.insert(value);
        }
    }

    if boundary.is_empty() {
        return Ok(None);
    }

    let mut boundary = boundary.into_iter().collect::<Vec<_>>();
    boundary.sort_by_key(|value| value.index());
    let refs = boundary
        .iter()
        .map(|value| (*value, format!("common.{}", value_name(*value))))
        .collect();
    Ok(Some(SharedStampValuesPlan {
        live,
        boundary,
        refs,
    }))
}

pub(super) fn shared_stamp_values_plan_for_roots(
    artifact: &CanonicalIrArtifact,
    static_cache: &ScalarStaticCache,
    roots: &HashMap<EquationId, ValueId>,
    reactive_roots: &HashMap<EquationId, ValueId>,
) -> Result<Option<SharedStampValuesPlan>, RustBackendError> {
    if reactive_roots.is_empty() {
        return Ok(None);
    }
    let stamp_live = collect_stamp_live_values(artifact, roots, static_cache)?;
    let reactive_live = collect_stamp_live_values(artifact, reactive_roots, static_cache)?;
    shared_stamp_values_plan(
        artifact,
        &stamp_live,
        &reactive_live,
        static_cache,
        roots,
        reactive_roots,
    )
}

pub(super) fn scalar_stamp_emitted_value_estimate_for_roots(
    artifact: &CanonicalIrArtifact,
    static_cache: &ScalarStaticCache,
    roots: &HashMap<EquationId, ValueId>,
    reactive_roots: &HashMap<EquationId, ValueId>,
) -> Result<usize, RustBackendError> {
    let stamp_live = collect_stamp_live_values(artifact, roots, static_cache)?;
    let reactive_live = if reactive_roots.is_empty() {
        HashSet::new()
    } else {
        collect_stamp_live_values(artifact, reactive_roots, static_cache)?
    };
    let shared_plan = if reactive_roots.is_empty() {
        None
    } else {
        shared_stamp_values_plan(
            artifact,
            &stamp_live,
            &reactive_live,
            static_cache,
            roots,
            reactive_roots,
        )?
    };
    let stamp_emit_live = shared_plan
        .as_ref()
        .map(|plan| tail_live_values(&stamp_live, &plan.live))
        .unwrap_or_else(|| stamp_live.clone());
    let reactive_emit_live = shared_plan
        .as_ref()
        .map(|plan| tail_live_values(&reactive_live, &plan.live))
        .unwrap_or(reactive_live);
    let stamp_inline_values =
        scalar_stamp_inline_values(artifact, &stamp_emit_live, static_cache, roots)?;
    let reactive_inline_values = if reactive_roots.is_empty() {
        HashSet::new()
    } else {
        scalar_stamp_inline_values(artifact, &reactive_emit_live, static_cache, reactive_roots)?
    };
    let common_inline_values = shared_plan
        .as_ref()
        .map(|plan| shared_stamp_inline_values(artifact, static_cache, plan))
        .transpose()?;
    scalar_stamp_value_emit_estimate(
        artifact,
        static_cache,
        shared_plan.as_ref(),
        &stamp_emit_live,
        &stamp_inline_values,
        &reactive_emit_live,
        &reactive_inline_values,
        common_inline_values.as_ref(),
    )
}

fn shareable_common_stamp_values(
    artifact: &CanonicalIrArtifact,
    stamp_live: &HashSet<ValueId>,
    reactive_live: &HashSet<ValueId>,
    static_cache: &ScalarStaticCache,
) -> Result<HashSet<ValueId>, RustBackendError> {
    let common = stamp_live
        .intersection(reactive_live)
        .copied()
        .filter(|value| !static_cache.contains(*value))
        .collect::<HashSet<_>>();
    let mut shared = HashSet::new();
    for value_id in ordered_live_values(artifact, &common, static_cache)? {
        if !common.contains(&value_id) {
            continue;
        }
        let value = artifact
            .opt
            .values
            .get(usize::from(value_id))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value_id}")))?;
        if !scalar_value_kind_can_share_between_stamp_modes(&value.kind) {
            continue;
        }
        let dependencies = scalar_value_dependencies(artifact, value_id)?;
        if dependencies
            .iter()
            .all(|dependency| static_cache.contains(*dependency) || shared.contains(dependency))
        {
            shared.insert(value_id);
        }
    }
    Ok(shared)
}

fn scalar_value_kind_can_share_between_stamp_modes(kind: &OptValueKind) -> bool {
    matches!(
        kind,
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
            | OptValueKind::BranchUnknownFlow { .. }
            | OptValueKind::Unary { .. }
            | OptValueKind::Binary { .. }
            | OptValueKind::Select { .. }
    )
}

fn tail_live_values(live: &HashSet<ValueId>, shared_live: &HashSet<ValueId>) -> HashSet<ValueId> {
    live.difference(shared_live).copied().collect()
}

fn use_local_shared_stamp_values(plan: &SharedStampValuesPlan) -> bool {
    plan.boundary.len() >= MIN_LOCAL_SHARED_STAMP_VALUES
}

fn shared_stamp_value_refs(plan: &SharedStampValuesPlan) -> HashMap<ValueId, String> {
    if use_local_shared_stamp_values(plan) {
        return plan
            .boundary
            .iter()
            .map(|value| (*value, value_name(*value)))
            .collect();
    }
    plan.refs.clone()
}

fn emit_shared_stamp_values_binding(plan: &SharedStampValuesPlan, out: &mut String) {
    if !use_local_shared_stamp_values(plan) {
        out.push_str("        let common=self.eval_common_stamp_values(ctx);\n");
        return;
    }

    const FIELDS_PER_LINE: usize = 8;
    out.push_str("        let CommonStampValues {\n");
    for chunk in plan.boundary.chunks(FIELDS_PER_LINE) {
        out.push_str("            ");
        out.push_str(
            &chunk
                .iter()
                .map(|value_id| value_name(*value_id))
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push_str(",\n");
    }
    out.push_str("        }=self.eval_common_stamp_values(ctx);\n");
}

pub(super) fn emit_shared_stamp_values_struct(
    artifact: &CanonicalIrArtifact,
    plan: &SharedStampValuesPlan,
    out: &mut String,
) -> Result<(), RustBackendError> {
    const FIELDS_PER_LINE: usize = 6;
    out.push_str("struct CommonStampValues {\n");
    for chunk in plan.boundary.chunks(FIELDS_PER_LINE) {
        out.push_str("    ");
        let fields = chunk
            .iter()
            .map(|value_id| {
                let value = artifact
                    .opt
                    .values
                    .get(usize::from(*value_id))
                    .ok_or_else(|| {
                        unsupported(artifact, format!("missing scalar value {value_id}"))
                    })?;
                Ok(format!(
                    "{}: {}",
                    value_name(*value_id),
                    rust_type(value.value_type)
                ))
            })
            .collect::<Result<Vec<_>, RustBackendError>>()?;
        out.push_str(&fields.join(", "));
        out.push_str(",\n");
    }
    out.push_str("}\n\n");
    Ok(())
}

pub(super) fn emit_shared_stamp_values_method(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    static_cache: &ScalarStaticCache,
    ddt_slots: &DdtSlots,
    plan: &SharedStampValuesPlan,
    enforce_source_budget: bool,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let inline_values = shared_stamp_inline_values(artifact, static_cache, plan)?;
    let static_cache_refs = scalar_static_cache_refs_for_stamp(static_cache);
    let context = ValueEmitContext {
        cached_values: &static_cache.set,
        cached_value_refs: &static_cache_refs,
        inline_values: &inline_values,
        use_cached_fields: true,
        inline_uncached_constants: false,
        use_exp_helpers: true,
        limexp_max_expr: "LIMEXP_MAX".to_string(),
        temperature_expr: "ctx.temperature()".to_string(),
        thermal_voltage_expr: "ctx.thermal_voltage()".to_string(),
        ddt_slots: Some(ddt_slots),
        ddt_mode: DdtEmitMode::Transient,
        node_array_expr: "n",
        branch_array_expr: "br",
        param_given_expr: "pg",
        multiplicity_expr: "m",
        loop_index_exprs: HashMap::new(),
        runtime_loop_values: HashMap::new(),
        runtime_loop_derivatives: HashMap::new(),
        external_value_refs: HashMap::new(),
    };

    out.push_str("    #[inline(always)]\n");
    out.push_str(
        "    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {\n",
    );
    out.push_str("        let n=self.nodes;\n");
    out.push_str("        let nodes=n;\n");
    if live_values_need_branches(artifact, &plan.live) {
        out.push_str("        let br=self.branches;\n");
        out.push_str("        let branches=br;\n");
    }
    if static_cache.has_temperature_values() {
        out.push_str(
            "        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());\n",
        );
    }
    if live_values_need_params(artifact, &plan.live) {
        out.push_str("        let p=&(*self.params);\n");
    }
    if live_values_need_param_given(artifact, &plan.live) {
        out.push_str("        let pg=self.param_given.as_ref();\n");
        out.push_str("        let param_given=pg;\n");
    }
    if live_values_need_multiplicity(artifact, &plan.live) {
        out.push_str("        let m=self.multiplicity;\n");
        out.push_str("        let multiplicity=m;\n");
    }
    emit_scalar_static_cache_aliases(static_cache, out);
    emit_live_values(
        artifact,
        parameter_fields,
        &plan.live,
        static_cache,
        &context,
        enforce_source_budget,
        out,
    )?;
    out.push_str("        CommonStampValues {\n");
    const VALUES_PER_LINE: usize = 8;
    for chunk in plan.boundary.chunks(VALUES_PER_LINE) {
        out.push_str("            ");
        out.push_str(
            &chunk
                .iter()
                .map(|value_id| value_name(*value_id))
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push_str(",\n");
    }
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    Ok(())
}

fn live_values_need_params(artifact: &CanonicalIrArtifact, live: &HashSet<ValueId>) -> bool {
    artifact.opt.values.iter().any(|value| {
        live.contains(&value.id) && matches!(value.kind, OptValueKind::Parameter { .. })
    })
}

fn live_values_need_param_given(artifact: &CanonicalIrArtifact, live: &HashSet<ValueId>) -> bool {
    artifact.opt.values.iter().any(|value| {
        live.contains(&value.id) && matches!(value.kind, OptValueKind::ParamGiven { .. })
    })
}

fn live_values_need_branches(artifact: &CanonicalIrArtifact, live: &HashSet<ValueId>) -> bool {
    artifact.opt.values.iter().any(|value| {
        live.contains(&value.id)
            && matches!(
                value.kind,
                OptValueKind::BranchFlow { .. } | OptValueKind::BranchUnknownFlow { .. }
            )
    })
}

fn live_values_need_multiplicity(artifact: &CanonicalIrArtifact, live: &HashSet<ValueId>) -> bool {
    artifact
        .opt
        .values
        .iter()
        .any(|value| live.contains(&value.id) && matches!(value.kind, OptValueKind::Multiplicity))
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ScalarExpHelperUsage {
    limexp: bool,
    limexp_derivative: bool,
    limited_exp: bool,
    limited_exp_derivative: bool,
}

impl ScalarExpHelperUsage {
    fn is_empty(self) -> bool {
        !self.limexp && !self.limexp_derivative && !self.limited_exp && !self.limited_exp_derivative
    }
}

fn emit_scalar_exp_helpers(out: &mut String, usage: ScalarExpHelperUsage) {
    if usage.limexp {
        out.push_str("#[inline]\n");
        out.push_str("fn scalar_limexp(arg: f64) -> f64 {\n");
        out.push_str("    if arg < 80.0 { arg.exp() } else { LIMEXP_MAX * (1.0 + arg - 80.0) }\n");
        out.push_str("}\n\n");
    }
    if usage.limexp_derivative {
        out.push_str("#[inline]\n");
        out.push_str("fn scalar_limexp_derivative(arg: f64) -> f64 {\n");
        out.push_str("    if arg < 80.0 { arg.exp() } else { LIMEXP_MAX }\n");
        out.push_str("}\n\n");
    }
    if usage.limited_exp {
        out.push_str("#[inline]\n");
        out.push_str("fn scalar_limited_exp(arg: f64) -> f64 {\n");
        out.push_str("    if arg > 80.0 { LIMEXP_MAX * (1.0 + arg - 80.0) } else if arg < -80.0 { 1.804851387e-35 } else { arg.exp() }\n");
        out.push_str("}\n\n");
    }
    if usage.limited_exp_derivative {
        out.push_str("#[inline]\n");
        out.push_str("fn scalar_limited_exp_derivative(arg: f64) -> f64 {\n");
        out.push_str(
            "    if arg > 80.0 { LIMEXP_MAX } else if arg < -80.0 { 0.0 } else { arg.exp() }\n",
        );
        out.push_str("}\n\n");
    }
}

fn scalar_exp_helper_usage(artifact: &CanonicalIrArtifact) -> ScalarExpHelperUsage {
    let mut usage = ScalarExpHelperUsage::default();
    for value in &artifact.opt.values {
        match value.kind {
            OptValueKind::Unary {
                op: OptUnaryOp::LimExp,
                ..
            } => usage.limexp = true,
            OptValueKind::Unary {
                op: OptUnaryOp::LimExpDerivative,
                ..
            } => usage.limexp_derivative = true,
            OptValueKind::Unary {
                op: OptUnaryOp::LimitedExp,
                ..
            } => usage.limited_exp = true,
            OptValueKind::Unary {
                op: OptUnaryOp::LimitedExpDerivative,
                ..
            } => usage.limited_exp_derivative = true,
            _ => {}
        }
    }
    usage
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
    let no_inline_values = HashSet::new();
    let instance_context = ValueEmitContext {
        cached_values: &static_cache.set,
        cached_value_refs: &static_cache.refs,
        inline_values: &no_inline_values,
        use_cached_fields: true,
        inline_uncached_constants: true,
        use_exp_helpers: false,
        limexp_max_expr: format_f64(LIMEXP_MAX),
        temperature_expr: "temperature".to_string(),
        thermal_voltage_expr: "thermal_voltage".to_string(),
        ddt_slots: None,
        ddt_mode: DdtEmitMode::Transient,
        node_array_expr: "nodes",
        branch_array_expr: "branches",
        param_given_expr: "param_given",
        multiplicity_expr: "multiplicity",
        loop_index_exprs: HashMap::new(),
        runtime_loop_values: HashMap::new(),
        runtime_loop_derivatives: HashMap::new(),
        external_value_refs: HashMap::new(),
    };
    let temperature_context = ValueEmitContext {
        cached_values: &static_cache.set,
        cached_value_refs: &static_cache.refs,
        inline_values: &no_inline_values,
        use_cached_fields: true,
        inline_uncached_constants: true,
        use_exp_helpers: false,
        limexp_max_expr: format_f64(LIMEXP_MAX),
        temperature_expr: "temperature".to_string(),
        thermal_voltage_expr: "thermal_voltage".to_string(),
        ddt_slots: None,
        ddt_mode: DdtEmitMode::Transient,
        node_array_expr: "nodes",
        branch_array_expr: "branches",
        param_given_expr: "param_given",
        multiplicity_expr: "multiplicity",
        loop_index_exprs: HashMap::new(),
        runtime_loop_values: HashMap::new(),
        runtime_loop_derivatives: HashMap::new(),
        external_value_refs: HashMap::new(),
    };
    let mut methods = String::new();

    push_scalar_static_cache_state_fields(&mut extensions, static_cache);

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
            let expr = emit_value_expr(artifact, parameter_fields, value, &instance_context)?;
            let target = static_cache.cache_ref(*value_id).ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("missing static scalar cache slot {value_id}"),
                )
            })?;
            methods.push_str(&format!("        {target}={expr};\n"));
        }
        methods.push_str("    }\n");
        extensions
            .after_new
            .push_str("        instance.recompute_instance_static();\n");
        extensions
            .set_parameter_hook
            .push_str("self.recompute_instance_static();\n");
    }

    if static_cache.has_temperature_values() {
        push_temperature_cache_state_fields(&mut extensions);
        extensions
            .set_parameter_hook
            .push_str("self.invalidate_temperature_static();\n");
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
            let expr = emit_value_expr(artifact, parameter_fields, value, &temperature_context)?;
            let target = static_cache.cache_ref(*value_id).ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("missing temperature-static scalar cache slot {value_id}"),
                )
            })?;
            methods.push_str(&format!("        {target}={expr};\n"));
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

fn push_scalar_static_cache_state_fields(
    extensions: &mut device::StateFileExtensions,
    static_cache: &ScalarStaticCache,
) {
    if static_cache.has_f64_values() {
        extensions.instance_fields.push_str(&format!(
            "    pub(crate) scalar_static_f64: Box<[f64; {}]>,\n",
            static_cache.f64_count
        ));
        extensions
            .clone_fields
            .push_str("            scalar_static_f64: self.scalar_static_f64.clone(),\n");
        extensions.new_initializers.push_str(&format!(
            "            scalar_static_f64: boxed_zero_f64_array::<{}>(),\n",
            static_cache.f64_count
        ));
        extensions
            .restore_destructure_fields
            .push_str("            scalar_static_f64,\n");
        extensions
            .restore_initializers
            .push_str("            scalar_static_f64,\n");
    }
    if static_cache.has_bool_values() {
        extensions.instance_fields.push_str(&format!(
            "    pub(crate) scalar_static_bool: Box<[bool; {}]>,\n",
            static_cache.bool_count
        ));
        extensions
            .clone_fields
            .push_str("            scalar_static_bool: self.scalar_static_bool.clone(),\n");
        extensions.new_initializers.push_str(&format!(
            "            scalar_static_bool: boxed_zero_bool_array::<{}>(),\n",
            static_cache.bool_count
        ));
        extensions
            .restore_destructure_fields
            .push_str("            scalar_static_bool,\n");
        extensions
            .restore_initializers
            .push_str("            scalar_static_bool,\n");
    }
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
        if value_graph_contains_ddt(artifact, root, &mut HashSet::new())? {
            continue;
        }
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
        if value_graph_contains_ddt(artifact, root, &mut HashSet::new())? {
            continue;
        }
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

pub(super) fn emit_static_current_values_with_shared_plan(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    shared_plan: Option<&SharedStampValuesPlan>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let stamp_live = collect_stamp_live_values(artifact, roots, static_cache)?;
    let stamp_emit_live = shared_plan
        .map(|plan| tail_live_values(&stamp_live, &plan.live))
        .unwrap_or_else(|| stamp_live.clone());
    let stamp_inline_values =
        scalar_inline_values(artifact, &stamp_emit_live, static_cache, roots)?;
    let stamp_context = ValueEmitContext {
        cached_values: &static_cache.set,
        cached_value_refs: &static_cache.refs,
        inline_values: &stamp_inline_values,
        use_cached_fields: true,
        inline_uncached_constants: false,
        use_exp_helpers: false,
        limexp_max_expr: "LIMEXP_MAX".to_string(),
        temperature_expr: "ctx.temperature()".to_string(),
        thermal_voltage_expr: "ctx.thermal_voltage()".to_string(),
        ddt_slots: None,
        ddt_mode: DdtEmitMode::Transient,
        node_array_expr: "nodes",
        branch_array_expr: "branches",
        param_given_expr: "param_given",
        multiplicity_expr: "multiplicity",
        loop_index_exprs: HashMap::new(),
        runtime_loop_values: HashMap::new(),
        runtime_loop_derivatives: HashMap::new(),
        external_value_refs: shared_plan
            .map(|plan| plan.refs.clone())
            .unwrap_or_default(),
    };
    emit_live_values(
        artifact,
        parameter_fields,
        &stamp_emit_live,
        static_cache,
        &stamp_context,
        false,
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
    enforce_source_budget: bool,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let values = ordered_live_values(artifact, live, static_cache)?;
    let mut emitted_runtime_loops = HashSet::new();
    let mut source_budget_check_countdown = 1024usize;
    let mut let_emitter = CompactLetEmitter::new("        ");
    for value_id in values {
        if context.external_value_refs.contains_key(&value_id) {
            continue;
        }
        if context.inline_values.contains(&value_id) {
            continue;
        }
        let value = artifact
            .opt
            .values
            .get(usize::from(value_id))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value_id}")))?;
        if let Some(loop_id) = runtime_loop_result_id(&value.kind) {
            if emitted_runtime_loops.insert(loop_id) {
                let_emitter.flush(out);
                emit_runtime_loop(artifact, parameter_fields, loop_id, live, context, out)?;
                if enforce_source_budget {
                    reject_oversized_scalar_stamp_source_bytes_so_far(artifact, out)?;
                }
            }
            continue;
        }
        if matches!(
            value.kind,
            OptValueKind::RuntimeLoopVariable { .. }
                | OptValueKind::RuntimeLoopVariableDerivative { .. }
        ) {
            continue;
        }
        let expr = emit_value_expr(artifact, parameter_fields, value, context)?;
        let_emitter.push(out, &value_name(value.id), &expr);
        if enforce_source_budget {
            source_budget_check_countdown = source_budget_check_countdown.saturating_sub(1);
            if source_budget_check_countdown == 0 {
                let_emitter.flush(out);
                reject_oversized_scalar_stamp_source_bytes_so_far(artifact, out)?;
                source_budget_check_countdown = 1024;
            }
        }
    }
    let_emitter.flush(out);
    if !live.is_empty() {
        out.push('\n');
    }
    if enforce_source_budget {
        reject_oversized_scalar_stamp_source_bytes_so_far(artifact, out)?;
    }
    Ok(())
}

pub(super) fn emit_static_current_stamps_with_shared_plan(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    shared_plan: Option<&SharedStampValuesPlan>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let inline_values = HashSet::new();
    let mut context =
        local_stamp_context(static_cache, &inline_values, None, DdtEmitMode::Transient);
    if let Some(plan) = shared_plan {
        context.external_value_refs = plan.refs.clone();
    }
    emit_current_stamps(
        artifact,
        parameter_fields,
        roots,
        &context,
        None,
        false,
        out,
    )
}

pub(super) fn emit_ddt_current_stamps_with_shared_plan(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    ddt_slots: &DdtSlots,
    shared_plan: Option<&SharedStampValuesPlan>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let inline_values = HashSet::new();
    let mut context = local_stamp_context(
        static_cache,
        &inline_values,
        Some(ddt_slots),
        DdtEmitMode::Transient,
    );
    if let Some(plan) = shared_plan {
        context.external_value_refs = plan.refs.clone();
    }
    emit_current_stamps(
        artifact,
        parameter_fields,
        roots,
        &context,
        Some(ddt_slots),
        false,
        out,
    )
}

fn emit_current_stamps(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    context: &ValueEmitContext<'_>,
    ddt_slots: Option<&DdtSlots>,
    enforce_source_budget: bool,
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
                    context,
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
                    context,
                    ddt_slots,
                    out,
                )?;
            }
            MirEquationKind::Indirect => {
                return Err(unsupported(artifact, "indirect contributions"));
            }
        }
        if enforce_source_budget {
            reject_oversized_scalar_stamp_source_bytes_so_far(artifact, out)?;
        }
    }
    Ok(())
}

pub(super) fn emit_current_reactive_stamps_with_shared_plan(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    static_cache: &ScalarStaticCache,
    shared_plan: Option<&SharedStampValuesPlan>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let inline_values = HashSet::new();
    let mut context = local_stamp_context(
        static_cache,
        &inline_values,
        None,
        DdtEmitMode::ReactiveLinearized,
    );
    if let Some(plan) = shared_plan {
        context.external_value_refs = plan.refs.clone();
    }
    emit_current_reactive_stamps_with_context(
        artifact,
        parameter_fields,
        roots,
        &context,
        false,
        out,
    )
}

fn emit_current_reactive_stamps_with_context(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    roots: &HashMap<EquationId, ValueId>,
    context: &ValueEmitContext<'_>,
    enforce_source_budget: bool,
    out: &mut String,
) -> Result<(), RustBackendError> {
    for equation in &artifact.mir.equations {
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        emit_current_reactive_stamp(artifact, parameter_fields, equation, root, context, out)?;
        if enforce_source_budget {
            reject_oversized_scalar_stamp_source_bytes_so_far(artifact, out)?;
        }
    }
    Ok(())
}

pub(super) fn scalar_transient_current_lowered_variable(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    equation: &MirEquation,
    root: ValueId,
    static_cache: &ScalarStaticCache,
    branch_axis_count: usize,
    ddt_slots: Option<&DdtSlots>,
    shared_plan: Option<&SharedStampValuesPlan>,
) -> Result<LoweredVariable, RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = scalar_derivatives(artifact, equation, root)?;

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
    let mut node_derivatives = vec!["0.0".to_string(); artifact.mir.nodes.len()];
    let no_inline_values = HashSet::new();
    let mut context = local_stamp_context(
        static_cache,
        &no_inline_values,
        ddt_slots,
        DdtEmitMode::Transient,
    );
    if let Some(plan) = shared_plan {
        context.external_value_refs = plan.refs.clone();
    }
    for (node, value) in &derivatives.nodes {
        node_derivatives[*node as usize] = scaled_derivative_value_expr(
            artifact,
            parameter_fields,
            *value,
            &context,
            derivative_scale,
        )?;
    }
    let mut branch_derivatives = vec!["0.0".to_string(); branch_axis_count];
    for (branch, value) in &derivatives.branches {
        let index = *branch as usize;
        if index >= branch_axis_count {
            return Err(internal(
                artifact,
                format!("branch derivative lane {branch} exceeds axis count {branch_axis_count}"),
            ));
        }
        branch_derivatives[index] = scaled_derivative_value_expr(
            artifact,
            parameter_fields,
            *value,
            &context,
            derivative_scale,
        )?;
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
    parameter_fields: &HashMap<String, String>,
    equation: &MirEquation,
    root: ValueId,
    static_cache: &ScalarStaticCache,
    branch_axis_count: usize,
    shared_plan: Option<&SharedStampValuesPlan>,
) -> Result<LoweredVariable, RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    let mut node_derivatives = vec!["0.0".to_string(); artifact.mir.nodes.len()];
    let no_inline_values = HashSet::new();
    let mut context = local_stamp_context(
        static_cache,
        &no_inline_values,
        None,
        DdtEmitMode::ReactiveLinearized,
    );
    if let Some(plan) = shared_plan {
        context.external_value_refs = plan.refs.clone();
    }
    for (node, value) in &derivatives.nodes {
        node_derivatives[*node as usize] = value_ref(artifact, parameter_fields, *value, &context)?;
    }
    let mut branch_derivatives = vec!["0.0".to_string(); branch_axis_count];
    for (branch, value) in &derivatives.branches {
        let index = *branch as usize;
        if index >= branch_axis_count {
            return Err(internal(
                artifact,
                format!("branch derivative lane {branch} exceeds axis count {branch_axis_count}"),
            ));
        }
        branch_derivatives[index] = value_ref(artifact, parameter_fields, *value, &context)?;
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
                "        let {ddt_value}=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, {slot}, {root_expr});\n"
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
                "        let {idt_value}=eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, {slot}, {root_expr}, {ic_expr});\n"
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
    context: &ValueEmitContext<'_>,
    ddt_slots: Option<&DdtSlots>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    let pos = optional_node_local_expr(equation.branch.pos_node);
    let neg = optional_node_local_expr(equation.branch.neg_node);
    let root_ref = value_ref(artifact, parameter_fields, root, context)?;
    let root_expr = current_root_expr(root_value.value_type, &root_ref);
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
        ([(node0, value0)], []) => {
            out.push_str("        stamper.stamp_current_node1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([(node0, value0), (node1, value1)], []) => {
            out.push_str("        stamper.stamp_current_node2_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value1,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([(node0, value0), (node1, value1), (node2, value2)], []) => {
            out.push_str("        stamper.stamp_current_node3_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value1,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {node2},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value2,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, value0)]) => {
            out.push_str("        stamper.stamp_current_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, value0), (branch1, value1)]) => {
            out.push_str("        stamper.stamp_current_branch2_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {branch1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value1,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([(node0, node_value0)], [(branch0, branch_value0)]) => {
            out.push_str("        stamper.stamp_current_node1_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *node_value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *branch_value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([(node0, node_value0), (node1, node_value1)], [(branch0, branch_value0)]) => {
            out.push_str("        stamper.stamp_current_node2_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *node_value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *node_value1,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *branch_value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        _ => {
            emit_wide_current_stamp(
                artifact,
                &derivatives.nodes,
                &derivatives.branches,
                &pos,
                &neg,
                &root_expr,
                parameter_fields,
                context,
                derivative_scale.as_str(),
                out,
            )?;
        }
    }
    Ok(())
}

fn emit_potential_stamp(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    equation: &MirEquation,
    root: ValueId,
    context: &ValueEmitContext<'_>,
    ddt_slots: Option<&DdtSlots>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let root_value = artifact
        .opt
        .values
        .get(usize::from(root))
        .ok_or_else(|| unsupported(artifact, format!("missing root scalar value {root}")))?;
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    let branch_slot = potential_branch_slot(artifact, equation)?;
    let pos = optional_node_local_expr(equation.branch.pos_node);
    let neg = optional_node_local_expr(equation.branch.neg_node);
    let root_ref = value_ref(artifact, parameter_fields, root, context)?;
    let root_expr = current_root_expr(root_value.value_type, &root_ref);
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
        ([(node0, value0)], []) => {
            out.push_str("        stamper.stamp_potential_node1_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([(node0, value0), (node1, value1)], []) => {
            out.push_str("        stamper.stamp_potential_node2_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value1,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, value0)]) => {
            out.push_str("        stamper.stamp_potential_branch1_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, value0), (branch1, value1)]) => {
            out.push_str("        stamper.stamp_potential_branch2_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {branch1},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value1,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([(node0, node_value0)], [(branch0, branch_value0)]) => {
            out.push_str("        stamper.stamp_potential_node1_branch1_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *node_value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *branch_value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        ([(node0, node_value0), (node1, node_value1)], [(branch0, branch_value0)]) => {
            out.push_str("        stamper.stamp_potential_node2_branch1_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *node_value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *node_value1,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *branch_value0,
                    context,
                    derivative_scale.as_str()
                )?
            ));
            out.push_str("        );\n");
        }
        _ => {
            emit_wide_potential_stamp(
                artifact,
                &derivatives.nodes,
                &derivatives.branches,
                branch_slot,
                &root_expr,
                parameter_fields,
                context,
                derivative_scale.as_str(),
                out,
            )?;
        }
    }
    Ok(())
}

fn emit_current_reactive_stamp(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    equation: &MirEquation,
    root: ValueId,
    context: &ValueEmitContext<'_>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let derivatives = scalar_derivatives(artifact, equation, root)?;

    let pos = optional_node_local_expr(equation.branch.pos_node);
    let neg = optional_node_local_expr(equation.branch.neg_node);
    match (
        derivatives.nodes.as_slice(),
        derivatives.branches.as_slice(),
    ) {
        ([], []) => {}
        ([(node0, value0)], []) => {
            out.push_str("        stamper.stamp_current_reactive_node1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value0, context)?
            ));
            out.push_str("        );\n");
        }
        ([(node0, value0), (node1, value1)], []) => {
            out.push_str("        stamper.stamp_current_reactive_node2_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value0, context)?
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value1, context)?
            ));
            out.push_str("        );\n");
        }
        ([(node0, value0), (node1, value1), (node2, value2)], []) => {
            out.push_str("        stamper.stamp_current_reactive_node3_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value0, context)?
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value1, context)?
            ));
            out.push_str(&format!("            {node2},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value2, context)?
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, value0)]) => {
            out.push_str("        stamper.stamp_current_reactive_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value0, context)?
            ));
            out.push_str("        );\n");
        }
        ([], [(branch0, value0), (branch1, value1)]) => {
            out.push_str("        stamper.stamp_current_reactive_branch2_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value0, context)?
            ));
            out.push_str(&format!("            {branch1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *value1, context)?
            ));
            out.push_str("        );\n");
        }
        ([(node0, node_value0)], [(branch0, branch_value0)]) => {
            out.push_str("        stamper.stamp_current_reactive_node1_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *node_value0, context)?
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *branch_value0, context)?
            ));
            out.push_str("        );\n");
        }
        ([(node0, node_value0), (node1, node_value1)], [(branch0, branch_value0)]) => {
            out.push_str("        stamper.stamp_current_reactive_node2_branch1_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            {node0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *node_value0, context)?
            ));
            out.push_str(&format!("            {node1},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *node_value1, context)?
            ));
            out.push_str(&format!("            {branch0},\n"));
            out.push_str(&format!(
                "            multiplicity * ({}),\n",
                value_ref(artifact, parameter_fields, *branch_value0, context)?
            ));
            out.push_str("        );\n");
        }
        _ => {
            emit_wide_current_reactive_stamp(
                artifact,
                &derivatives.nodes,
                &derivatives.branches,
                &pos,
                &neg,
                parameter_fields,
                context,
                out,
            )?;
        }
    }
    Ok(())
}

fn emit_wide_current_reactive_stamp(
    artifact: &CanonicalIrArtifact,
    node_derivatives: &[(u32, ValueId)],
    branch_derivatives: &[(u32, ValueId)],
    pos: &str,
    neg: &str,
    parameter_fields: &HashMap<String, String>,
    context: &ValueEmitContext<'_>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    if branch_derivatives.is_empty() {
        return emit_wide_node_current_reactive_stamp(
            artifact,
            node_derivatives,
            pos,
            neg,
            parameter_fields,
            context,
            out,
        );
    }

    if node_derivatives.len() == artifact.mir.nodes.len()
        && branch_derivatives.len() == artifact.mir.branch_unknowns.len()
    {
        let mut node_values = vec!["0.0".to_string(); artifact.mir.nodes.len()];
        for (node, value) in node_derivatives {
            node_values[*node as usize] = value_ref(artifact, parameter_fields, *value, context)?;
        }
        let mut branch_values = vec!["0.0".to_string(); artifact.mir.branch_unknowns.len()];
        for (branch, value) in branch_derivatives {
            branch_values[*branch as usize] =
                value_ref(artifact, parameter_fields, *value, context)?;
        }
        out.push_str("        stamper.stamp_current_reactive_dense_local(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str(&format!("            &[{}],\n", node_values.join(",")));
        out.push_str(&format!("            &[{}],\n", branch_values.join(",")));
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
            .map(|(_, value)| value_ref(artifact, parameter_fields, *value, context))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        let branch_indices = branch_derivatives
            .iter()
            .map(|(branch, _)| branch.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let branch_values = branch_derivatives
            .iter()
            .map(|(_, value)| value_ref(artifact, parameter_fields, *value, context))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        out.push_str("        stamper.stamp_current_reactive_indexed_dense_local(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str(&format!("            &[{node_indices}],\n"));
        out.push_str(&format!("            &[{node_values}],\n"));
        out.push_str(&format!("            &[{branch_indices}],\n"));
        out.push_str(&format!("            &[{branch_values}],\n"));
        out.push_str("            multiplicity,\n");
        out.push_str("        );\n");
    }
    Ok(())
}

fn emit_wide_node_current_reactive_stamp(
    artifact: &CanonicalIrArtifact,
    derivatives: &[(u32, ValueId)],
    pos: &str,
    neg: &str,
    parameter_fields: &HashMap<String, String>,
    context: &ValueEmitContext<'_>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    if derivatives.len() == artifact.mir.nodes.len() {
        let mut node_derivatives = vec!["0.0".to_string(); artifact.mir.nodes.len()];
        for (node, value) in derivatives {
            node_derivatives[*node as usize] =
                value_ref(artifact, parameter_fields, *value, context)?;
        }
        out.push_str("        stamper.stamp_current_reactive_dense_local(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str(&format!("            &[{}],\n", node_derivatives.join(",")));
        out.push_str("            &[],\n");
        out.push_str("            multiplicity,\n");
        out.push_str("        );\n");
    } else {
        let node_indices = derivatives
            .iter()
            .map(|(node, _)| node.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let node_derivatives = derivatives
            .iter()
            .map(|(_, value)| value_ref(artifact, parameter_fields, *value, context))
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        out.push_str("        stamper.stamp_current_reactive_indexed_dense_local(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str(&format!("            &[{node_indices}],\n"));
        out.push_str(&format!("            &[{node_derivatives}],\n"));
        out.push_str("            &[],\n");
        out.push_str("            &[],\n");
        out.push_str("            multiplicity,\n");
        out.push_str("        );\n");
    }
    Ok(())
}

fn emit_wide_potential_stamp(
    artifact: &CanonicalIrArtifact,
    node_derivatives: &[(u32, ValueId)],
    branch_derivatives: &[(u32, ValueId)],
    branch_slot: usize,
    root_expr: &str,
    parameter_fields: &HashMap<String, String>,
    context: &ValueEmitContext<'_>,
    derivative_scale: &str,
    out: &mut String,
) -> Result<(), RustBackendError> {
    if node_derivatives.len() == artifact.mir.nodes.len()
        && branch_derivatives.len() == artifact.mir.branch_unknowns.len()
    {
        let mut node_values = vec!["0.0".to_string(); artifact.mir.nodes.len()];
        for (node, value) in node_derivatives {
            node_values[*node as usize] = scaled_derivative_value_expr(
                artifact,
                parameter_fields,
                *value,
                context,
                derivative_scale,
            )?;
        }
        let mut branch_values = vec!["0.0".to_string(); artifact.mir.branch_unknowns.len()];
        for (branch, value) in branch_derivatives {
            branch_values[*branch as usize] = scaled_derivative_value_expr(
                artifact,
                parameter_fields,
                *value,
                context,
                derivative_scale,
            )?;
        }
        out.push_str("        stamper.stamp_potential_dense_local(\n");
        out.push_str(&format!("            {branch_slot},\n"));
        out.push_str(&format!("            {root_expr},\n"));
        out.push_str(&format!("            &[{}],\n", node_values.join(",")));
        out.push_str(&format!("            &[{}],\n", branch_values.join(",")));
        out.push_str("        );\n");
    } else {
        let node_indices = node_derivatives
            .iter()
            .map(|(node, _)| node.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let node_values = node_derivatives
            .iter()
            .map(|(_, value)| {
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value,
                    context,
                    derivative_scale,
                )
            })
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        let branch_indices = branch_derivatives
            .iter()
            .map(|(branch, _)| branch.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let branch_values = branch_derivatives
            .iter()
            .map(|(_, value)| {
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value,
                    context,
                    derivative_scale,
                )
            })
            .collect::<Result<Vec<_>, _>>()?
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
            out.push_str("        stamper.stamp_potential_indexed_dense_local(\n");
            out.push_str(&format!("            {branch_slot},\n"));
            out.push_str(&format!("            {root_expr},\n"));
            out.push_str(&format!("            &[{node_indices}],\n"));
            out.push_str(&format!("            &[{node_values}],\n"));
            out.push_str(&format!("            &[{branch_indices}],\n"));
            out.push_str(&format!("            &[{branch_values}],\n"));
            out.push_str("        );\n");
        }
    }
    Ok(())
}

fn emit_wide_current_stamp(
    artifact: &CanonicalIrArtifact,
    node_derivatives: &[(u32, ValueId)],
    branch_derivatives: &[(u32, ValueId)],
    pos: &str,
    neg: &str,
    root_expr: &str,
    parameter_fields: &HashMap<String, String>,
    context: &ValueEmitContext<'_>,
    derivative_scale: &str,
    out: &mut String,
) -> Result<(), RustBackendError> {
    if node_derivatives.len() == artifact.mir.nodes.len()
        && branch_derivatives.len() == artifact.mir.branch_unknowns.len()
    {
        let mut node_values = vec!["0.0".to_string(); artifact.mir.nodes.len()];
        for (node, value) in node_derivatives {
            node_values[*node as usize] = scaled_derivative_value_expr(
                artifact,
                parameter_fields,
                *value,
                context,
                derivative_scale,
            )?;
        }
        let mut branch_values = vec!["0.0".to_string(); artifact.mir.branch_unknowns.len()];
        for (branch, value) in branch_derivatives {
            branch_values[*branch as usize] = scaled_derivative_value_expr(
                artifact,
                parameter_fields,
                *value,
                context,
                derivative_scale,
            )?;
        }
        out.push_str("        stamper.stamp_current_dense_local(\n");
        out.push_str(&format!("            {pos},\n"));
        out.push_str(&format!("            {neg},\n"));
        out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
        out.push_str(&format!("            &[{}],\n", node_values.join(",")));
        out.push_str(&format!("            &[{}],\n", branch_values.join(",")));
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
            .map(|(_, value)| {
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value,
                    context,
                    derivative_scale,
                )
            })
            .collect::<Result<Vec<_>, _>>()?
            .join(", ");
        let branch_indices = branch_derivatives
            .iter()
            .map(|(branch, _)| branch.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let branch_values = branch_derivatives
            .iter()
            .map(|(_, value)| {
                scaled_derivative_value_expr(
                    artifact,
                    parameter_fields,
                    *value,
                    context,
                    derivative_scale,
                )
            })
            .collect::<Result<Vec<_>, _>>()?
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
            out.push_str("        stamper.stamp_current_indexed_dense_local(\n");
            out.push_str(&format!("            {pos},\n"));
            out.push_str(&format!("            {neg},\n"));
            out.push_str(&format!("            multiplicity * ({root_expr}),\n"));
            out.push_str(&format!("            &[{node_indices}],\n"));
            out.push_str(&format!("            &[{node_values}],\n"));
            out.push_str(&format!("            &[{branch_indices}],\n"));
            out.push_str(&format!("            &[{branch_values}],\n"));
            out.push_str("            multiplicity,\n");
            out.push_str("        );\n");
        }
    }
    Ok(())
}

fn current_root_expr(value_type: OptValueType, root_name: &str) -> String {
    match value_type {
        OptValueType::Real => root_name.to_string(),
        OptValueType::Boolean => format!("if {root_name}{{1.0}}else{{0.0}}"),
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
            format!("if {}[{index}]{{1.0}}else{{0.0}}", context.param_given_expr)
        }
        OptValueKind::Temperature => context.temperature_expr.clone(),
        OptValueKind::ThermalVoltage => context.thermal_voltage_expr.clone(),
        OptValueKind::Multiplicity => context.multiplicity_expr.to_string(),
        OptValueKind::Time => "self.time".to_string(),
        OptValueKind::Analysis { query } => {
            format!(
                "if {}{{1.0}}else{{0.0}}",
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
        OptValueKind::Ddt { operator, input } => {
            emit_ddt_value_expr(artifact, parameter_fields, *operator, *input, context)?
        }
        OptValueKind::DdtScale => match context.ddt_mode {
            DdtEmitMode::Transient => "ddt_scale".to_string(),
            DdtEmitMode::ReactiveLinearized => "1.0".to_string(),
        },
        OptValueKind::NodePotential { node } => {
            format!(
                "ctx.node_voltage({}[{}])",
                context.node_array_expr,
                node.index()
            )
        }
        OptValueKind::BranchFlow { branch } => {
            let slot = branch_flow_slot(artifact, *branch)?;
            format!("ctx.branch_current({}[{slot}])", context.branch_array_expr)
        }
        OptValueKind::BranchUnknownFlow { branch_unknown } => {
            let slot = branch_unknown_flow_slot(artifact, *branch_unknown)?;
            format!("ctx.branch_current({}[{slot}])", context.branch_array_expr)
        }
        OptValueKind::LoopIndex { loop_id } => context
            .loop_index_exprs
            .get(loop_id)
            .cloned()
            .ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("loop index {loop_id} outside counted sum"),
                )
            })?,
        OptValueKind::CountedSum {
            loop_id,
            count,
            initial,
            term,
        } => emit_counted_sum_expr(
            artifact,
            parameter_fields,
            value.id,
            *loop_id,
            *count,
            *initial,
            *term,
            context,
        )?,
        OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::RuntimeLoopResult { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. } => {
            value_ref(artifact, parameter_fields, value.id, context)?
        }
        OptValueKind::Unary { op, input } => emit_unary_expr(
            *op,
            value_ref(artifact, parameter_fields, *input, context)?,
            value_type(artifact, *input)?,
            &context.limexp_max_expr,
            context.use_exp_helpers,
        ),
        OptValueKind::Binary { op, left, right } => {
            let left_type = value_type(artifact, *left)?;
            let right_type = value_type(artifact, *right)?;
            if *op == OptBinaryOp::Pow {
                emit_pow_expr(
                    artifact,
                    parameter_fields,
                    *left,
                    left_type,
                    *right,
                    right_type,
                    context,
                )?
            } else {
                emit_binary_expr(
                    *op,
                    value_ref(artifact, parameter_fields, *left, context)?,
                    left_type,
                    value_ref(artifact, parameter_fields, *right, context)?,
                    right_type,
                )
            }
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            let condition_type = value_type(artifact, *condition)?;
            let then_type = value_type(artifact, *then_value)?;
            let else_type = value_type(artifact, *else_value)?;
            format!(
                "(if {}{{{}}}else{{{}}})",
                truth_expr(
                    value_ref(artifact, parameter_fields, *condition, context)?,
                    condition_type,
                ),
                coerce_value_expr(
                    value_ref(artifact, parameter_fields, *then_value, context)?,
                    then_type,
                    value.value_type,
                ),
                coerce_value_expr(
                    value_ref(artifact, parameter_fields, *else_value, context)?,
                    else_type,
                    value.value_type,
                )
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

fn emit_counted_sum_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    owner: ValueId,
    loop_id: u32,
    count: ValueId,
    initial: ValueId,
    term: ValueId,
    context: &ValueEmitContext<'_>,
) -> Result<String, RustBackendError> {
    let count_expr = value_ref(artifact, parameter_fields, count, context)?;
    let initial_expr = value_ref(artifact, parameter_fields, initial, context)?;
    let accumulator_name = format!("counted_sum_{}_acc", owner.index());
    let count_name = format!("counted_sum_{}_count", owner.index());
    let counter_name = format!("counted_sum_{}_i", owner.index());
    let index_name = format!("counted_sum_{}_index", owner.index());

    let mut loop_live = HashSet::new();
    mark_counted_sum_term_live(artifact, term, context.cached_values, &mut loop_live)?;
    let mandatory = HashSet::new();
    let mut loop_inline_values: HashSet<ValueId> = context.inline_values.iter().copied().collect();
    loop_inline_values.extend(scalar_inline_values_for_live_values(
        artifact,
        &loop_live,
        context.cached_values,
        &mandatory,
        &[term],
    )?);

    let mut loop_context = ValueEmitContext {
        cached_values: context.cached_values,
        cached_value_refs: context.cached_value_refs,
        inline_values: &loop_inline_values,
        use_cached_fields: context.use_cached_fields,
        inline_uncached_constants: context.inline_uncached_constants,
        use_exp_helpers: context.use_exp_helpers,
        limexp_max_expr: context.limexp_max_expr.clone(),
        temperature_expr: context.temperature_expr.clone(),
        thermal_voltage_expr: context.thermal_voltage_expr.clone(),
        ddt_slots: context.ddt_slots,
        ddt_mode: context.ddt_mode,
        node_array_expr: context.node_array_expr,
        branch_array_expr: context.branch_array_expr,
        param_given_expr: context.param_given_expr,
        multiplicity_expr: context.multiplicity_expr,
        loop_index_exprs: context.loop_index_exprs.clone(),
        runtime_loop_values: context.runtime_loop_values.clone(),
        runtime_loop_derivatives: context.runtime_loop_derivatives.clone(),
        external_value_refs: context.external_value_refs.clone(),
    };
    loop_context
        .loop_index_exprs
        .insert(loop_id, index_name.clone());

    let loop_values = ordered_counted_sum_values(artifact, &loop_live, context.cached_values)?;

    let mut expr = String::new();
    expr.push_str("{\n");
    expr.push_str(&format!(
        "            let mut {accumulator_name}={initial_expr};\n"
    ));
    expr.push_str(&format!("            let {count_name}={count_expr};\n"));
    expr.push_str(&format!("            let mut {counter_name}: i64 = 0;\n"));
    expr.push_str(&format!(
        "            while ({counter_name} as f64) < {count_name} {{\n"
    ));
    expr.push_str(&format!(
        "                let {index_name}={counter_name} as f64;\n"
    ));
    for value_id in loop_values {
        let value = artifact
            .opt
            .values
            .get(usize::from(value_id))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value_id}")))?;
        if matches!(value.kind, OptValueKind::LoopIndex { .. }) {
            continue;
        }
        if loop_context.inline_values.contains(&value_id) {
            continue;
        }
        let value_expr = emit_value_expr(artifact, parameter_fields, value, &loop_context)?;
        expr.push_str(&format!(
            "                let {}={};\n",
            value_name(value.id),
            value_expr
        ));
    }
    let term_expr = value_ref(artifact, parameter_fields, term, &loop_context)?;
    expr.push_str(&format!(
        "                {accumulator_name} += {term_expr};\n"
    ));
    expr.push_str(&format!("                {counter_name} += 1;\n"));
    expr.push_str("            }\n");
    expr.push_str(&format!("            {accumulator_name}\n"));
    expr.push_str("        }");
    Ok(expr)
}

fn runtime_loop_source_binding_estimate(
    artifact: &CanonicalIrArtifact,
    loop_id: u32,
    live: &HashSet<ValueId>,
    cached_values: &HashSet<ValueId>,
) -> Result<usize, RustBackendError> {
    let runtime_loop = artifact
        .opt
        .runtime_loops
        .iter()
        .find(|runtime_loop| runtime_loop.loop_id == loop_id)
        .ok_or_else(|| unsupported(artifact, format!("missing runtime loop {loop_id}")))?;
    let derivative_lanes = runtime_loop_live_derivative_lanes(artifact, loop_id, live);

    let mut initializer_live = HashSet::new();
    for variable in &runtime_loop.variables {
        mark_counted_sum_term_live(
            artifact,
            variable.initial,
            cached_values,
            &mut initializer_live,
        )?;
        for lane in &derivative_lanes {
            if let Some(value) = derivative_value_for_lane(artifact, variable.initial, *lane)? {
                mark_counted_sum_term_live(artifact, value, cached_values, &mut initializer_live)?;
            }
        }
    }

    let mut condition_live = HashSet::new();
    mark_counted_sum_term_live(
        artifact,
        runtime_loop.condition,
        cached_values,
        &mut condition_live,
    )?;

    let mut body_live = HashSet::new();
    for assignment in &runtime_loop.assignments {
        mark_counted_sum_term_live(artifact, assignment.value, cached_values, &mut body_live)?;
        for lane in &derivative_lanes {
            if let Some(value) = derivative_value_for_lane(artifact, assignment.value, *lane)? {
                mark_counted_sum_term_live(artifact, value, cached_values, &mut body_live)?;
            }
        }
    }

    let initialization_lines = runtime_loop
        .variables
        .len()
        .saturating_mul(1usize.saturating_add(derivative_lanes.len()));
    let initializer_dependency_lines =
        ordered_counted_sum_values(artifact, &initializer_live, cached_values)?.len();
    let condition_lines =
        ordered_counted_sum_values(artifact, &condition_live, cached_values)?.len();
    let body_lines = ordered_counted_sum_values(artifact, &body_live, cached_values)?.len();
    let assignment_lines = runtime_loop.assignments.len();
    let result_lines = runtime_loop
        .variables
        .iter()
        .filter(|variable| live.contains(&variable.result))
        .count();
    let derivative_result_lines = artifact
        .opt
        .values
        .iter()
        .filter(|value| live.contains(&value.id))
        .filter(|value| {
            matches!(
                value.kind,
                OptValueKind::RuntimeLoopResultDerivative {
                    loop_id: value_loop_id,
                    ..
                } if value_loop_id == loop_id
            )
        })
        .count();

    Ok(initialization_lines
        .saturating_add(initializer_dependency_lines)
        .saturating_add(condition_lines)
        .saturating_add(body_lines)
        .saturating_add(assignment_lines)
        .saturating_add(result_lines)
        .saturating_add(derivative_result_lines))
}

fn emit_runtime_loop(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    loop_id: u32,
    live: &HashSet<ValueId>,
    context: &ValueEmitContext<'_>,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let runtime_loop = artifact
        .opt
        .runtime_loops
        .iter()
        .find(|runtime_loop| runtime_loop.loop_id == loop_id)
        .ok_or_else(|| unsupported(artifact, format!("missing runtime loop {loop_id}")))?;
    let derivative_lanes = runtime_loop_live_derivative_lanes(artifact, loop_id, live);

    let mut initializer_live = HashSet::new();
    let mut external_uses = Vec::new();
    for variable in &runtime_loop.variables {
        external_uses.push(variable.initial);
        mark_counted_sum_term_live(
            artifact,
            variable.initial,
            context.cached_values,
            &mut initializer_live,
        )?;
        for lane in &derivative_lanes {
            if let Some(value) = derivative_value_for_lane(artifact, variable.initial, *lane)? {
                external_uses.push(value);
                mark_counted_sum_term_live(
                    artifact,
                    value,
                    context.cached_values,
                    &mut initializer_live,
                )?;
            }
        }
    }

    let mut condition_live = HashSet::new();
    mark_counted_sum_term_live(
        artifact,
        runtime_loop.condition,
        context.cached_values,
        &mut condition_live,
    )?;

    let mut body_live = HashSet::new();
    external_uses.push(runtime_loop.condition);
    for assignment in &runtime_loop.assignments {
        external_uses.push(assignment.value);
        mark_counted_sum_term_live(
            artifact,
            assignment.value,
            context.cached_values,
            &mut body_live,
        )?;
        for lane in &derivative_lanes {
            if let Some(value) = derivative_value_for_lane(artifact, assignment.value, *lane)? {
                external_uses.push(value);
                mark_counted_sum_term_live(artifact, value, context.cached_values, &mut body_live)?;
            }
        }
    }

    let mut loop_live = initializer_live.clone();
    loop_live.extend(condition_live.iter().copied());
    loop_live.extend(body_live.iter().copied());
    let mandatory = HashSet::new();
    let mut loop_inline_values: HashSet<ValueId> = context.inline_values.iter().copied().collect();
    loop_inline_values.extend(scalar_inline_values_for_live_values(
        artifact,
        &loop_live,
        context.cached_values,
        &mandatory,
        &external_uses,
    )?);

    let mut loop_context = ValueEmitContext {
        cached_values: context.cached_values,
        cached_value_refs: context.cached_value_refs,
        inline_values: &loop_inline_values,
        use_cached_fields: context.use_cached_fields,
        inline_uncached_constants: context.inline_uncached_constants,
        use_exp_helpers: context.use_exp_helpers,
        limexp_max_expr: context.limexp_max_expr.clone(),
        temperature_expr: context.temperature_expr.clone(),
        thermal_voltage_expr: context.thermal_voltage_expr.clone(),
        ddt_slots: context.ddt_slots,
        ddt_mode: context.ddt_mode,
        node_array_expr: context.node_array_expr,
        branch_array_expr: context.branch_array_expr,
        param_given_expr: context.param_given_expr,
        multiplicity_expr: context.multiplicity_expr,
        loop_index_exprs: context.loop_index_exprs.clone(),
        runtime_loop_values: context.runtime_loop_values.clone(),
        runtime_loop_derivatives: context.runtime_loop_derivatives.clone(),
        external_value_refs: context.external_value_refs.clone(),
    };

    let initializer_values =
        ordered_counted_sum_values(artifact, &initializer_live, context.cached_values)?;
    emit_runtime_loop_inner_values(
        artifact,
        parameter_fields,
        initializer_values,
        &loop_context,
        "        ",
        out,
    )?;

    let mut init_let_emitter = CompactLetEmitter::new("        ");
    for (slot, variable) in runtime_loop.variables.iter().enumerate() {
        let slot = u32::try_from(slot).expect("runtime loop slot exceeds u32::MAX");
        let local = runtime_loop_value_name(loop_id, slot);
        let initial = coerce_value_expr(
            value_ref(artifact, parameter_fields, variable.initial, &loop_context)?,
            value_type(artifact, variable.initial)?,
            variable.value_type,
        );
        init_let_emitter.push_mut_typed(out, &local, rust_type(variable.value_type), &initial);
        loop_context
            .runtime_loop_values
            .insert((loop_id, slot), local);
        for lane in &derivative_lanes {
            let derivative_local = runtime_loop_derivative_name(loop_id, slot, *lane);
            let initial_derivative = if let Some(value) =
                derivative_value_for_lane(artifact, variable.initial, *lane)?
            {
                value_ref(artifact, parameter_fields, value, &loop_context)?
            } else {
                "0.0".to_string()
            };
            init_let_emitter.push_mut_typed(out, &derivative_local, "f64", &initial_derivative);
            loop_context
                .runtime_loop_derivatives
                .insert((loop_id, slot, *lane), derivative_local);
        }
    }
    init_let_emitter.flush(out);
    out.push_str("        {\n");
    let guard = runtime_loop_guard_name(loop_id);
    out.push_str(&format!("            let mut {guard}=0usize;\n"));

    let condition_values =
        ordered_counted_sum_values(artifact, &condition_live, context.cached_values)?;
    out.push_str("            while {\n");
    emit_runtime_loop_inner_values(
        artifact,
        parameter_fields,
        condition_values,
        &loop_context,
        "                ",
        out,
    )?;
    let condition = value_ref(
        artifact,
        parameter_fields,
        runtime_loop.condition,
        &loop_context,
    )?;
    let condition_type = value_type(artifact, runtime_loop.condition)?;
    out.push_str(&format!(
        "                {}\n",
        truth_expr(condition, condition_type)
    ));
    out.push_str("            } {\n");
    out.push_str(&format!("                {guard}+=1;\n"));
    out.push_str(&format!(
        "                assert!({guard}<=Self::MAX_ANALOG_LOOP_ITERATIONS,\"generated Verilog-A scalar runtime loop exceeded iteration guard\");\n"
    ));

    let body_values = ordered_counted_sum_values(artifact, &body_live, context.cached_values)?;
    emit_runtime_loop_inner_values(
        artifact,
        parameter_fields,
        body_values,
        &loop_context,
        "                ",
        out,
    )?;
    for assignment in &runtime_loop.assignments {
        let variable = runtime_loop
            .variables
            .get(usize::try_from(assignment.slot).expect("runtime loop slot exceeds usize::MAX"))
            .ok_or_else(|| {
                unsupported(
                    artifact,
                    format!(
                        "runtime loop {loop_id} assignment targets missing slot {}",
                        assignment.slot
                    ),
                )
            })?;
        let target = runtime_loop_value_name(loop_id, assignment.slot);
        let value = coerce_value_expr(
            value_ref(artifact, parameter_fields, assignment.value, &loop_context)?,
            value_type(artifact, assignment.value)?,
            variable.value_type,
        );
        let mut targets = vec![target];
        let mut values = vec![value];
        for lane in &derivative_lanes {
            let target = runtime_loop_derivative_name(loop_id, assignment.slot, *lane);
            let value = if let Some(value) =
                derivative_value_for_lane(artifact, assignment.value, *lane)?
            {
                value_ref(artifact, parameter_fields, value, &loop_context)?
            } else {
                "0.0".to_string()
            };
            targets.push(target);
            values.push(value);
        }
        out.push_str(&format!(
            "                ({})=({});\n",
            targets.join(","),
            values.join(",")
        ));
    }
    out.push_str("            }\n");
    out.push_str("        }\n");

    let mut result_let_emitter = CompactLetEmitter::new("        ");
    for (slot, variable) in runtime_loop.variables.iter().enumerate() {
        if live.contains(&variable.result) {
            let slot = u32::try_from(slot).expect("runtime loop slot exceeds u32::MAX");
            let local = runtime_loop_value_name(loop_id, slot);
            result_let_emitter.push(out, &value_name(variable.result), &local);
        }
    }
    for value in &artifact.opt.values {
        if !live.contains(&value.id) {
            continue;
        }
        if let OptValueKind::RuntimeLoopResultDerivative {
            loop_id: value_loop_id,
            slot,
            lane,
        } = value.kind
            && value_loop_id == loop_id
        {
            let local = runtime_loop_derivative_name(loop_id, slot, lane);
            result_let_emitter.push(out, &value_name(value.id), &local);
        }
    }
    result_let_emitter.flush(out);
    out.push('\n');
    Ok(())
}

fn emit_runtime_loop_inner_values(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    values: impl IntoIterator<Item = ValueId>,
    context: &ValueEmitContext<'_>,
    indent: &'static str,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let mut let_emitter = CompactLetEmitter::new(indent);
    for value_id in values {
        emit_runtime_loop_inner_value(
            artifact,
            parameter_fields,
            value_id,
            context,
            &mut let_emitter,
            out,
        )?;
    }
    let_emitter.flush(out);
    Ok(())
}

fn emit_runtime_loop_inner_value(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    value_id: ValueId,
    context: &ValueEmitContext<'_>,
    let_emitter: &mut CompactLetEmitter,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let value = artifact
        .opt
        .values
        .get(usize::from(value_id))
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value_id}")))?;
    if matches!(
        value.kind,
        OptValueKind::RuntimeLoopVariable { .. }
            | OptValueKind::RuntimeLoopVariableDerivative { .. }
            | OptValueKind::RuntimeLoopResult { .. }
            | OptValueKind::RuntimeLoopResultDerivative { .. }
    ) {
        return Ok(());
    }
    if context.inline_values.contains(&value_id) {
        return Ok(());
    }
    let expr = emit_value_expr(artifact, parameter_fields, value, context)?;
    let_emitter.push(out, &value_name(value.id), &expr);
    Ok(())
}

fn runtime_loop_live_derivative_lanes(
    artifact: &CanonicalIrArtifact,
    loop_id: u32,
    live: &HashSet<ValueId>,
) -> Vec<DerivativeLane> {
    let mut lanes: Vec<_> = artifact
        .opt
        .values
        .iter()
        .filter(|value| live.contains(&value.id))
        .filter_map(|value| match value.kind {
            OptValueKind::RuntimeLoopResultDerivative {
                loop_id: value_loop_id,
                lane,
                ..
            } if value_loop_id == loop_id => Some(lane),
            _ => None,
        })
        .collect();
    lanes.sort();
    lanes.dedup();
    lanes
}

fn runtime_loop_result_id(kind: &OptValueKind) -> Option<u32> {
    match kind {
        OptValueKind::RuntimeLoopResult { loop_id, .. }
        | OptValueKind::RuntimeLoopResultDerivative { loop_id, .. } => Some(*loop_id),
        _ => None,
    }
}

fn emit_ddt_value_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    operator: ExprId,
    input: ValueId,
    context: &ValueEmitContext<'_>,
) -> Result<String, RustBackendError> {
    match context.ddt_mode {
        DdtEmitMode::Transient => {
            let slots = context
                .ddt_slots
                .ok_or_else(|| unsupported(artifact, "ddt scalar value context"))?;
            let slot = slots.slot_for(operator).ok_or_else(|| {
                internal(
                    artifact,
                    format!("ddt expression {operator} has no generated state slot"),
                )
            })?;
            let input = value_ref(artifact, parameter_fields, input, context)?;
            Ok(format!(
                "eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, {slot}, {input})"
            ))
        }
        DdtEmitMode::ReactiveLinearized => Ok("0.0".to_string()),
    }
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
                "Pow" => emit_string_power_expr(&left, &right),
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
                "if param_given[{}]{{1.0}}else{{0.0}}",
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
        ("pow", [left, right]) => emit_string_power_expr(left, right),
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
    format!("(({value})!=0.0)")
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
    if let Some(OptValueKind::LoopIndex { loop_id }) = artifact
        .opt
        .values
        .get(usize::from(value))
        .map(|value| &value.kind)
    {
        return context
            .loop_index_exprs
            .get(loop_id)
            .cloned()
            .ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("loop index {loop_id} outside counted sum"),
                )
            });
    }

    if let Some(kind) = artifact
        .opt
        .values
        .get(usize::from(value))
        .map(|value| &value.kind)
    {
        match *kind {
            OptValueKind::RuntimeLoopVariable { loop_id, slot } => {
                return context
                    .runtime_loop_values
                    .get(&(loop_id, slot))
                    .cloned()
                    .ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!("runtime loop variable {loop_id}:{slot} outside loop"),
                        )
                    });
            }
            OptValueKind::RuntimeLoopVariableDerivative {
                loop_id,
                slot,
                lane,
            } => {
                return context
                    .runtime_loop_derivatives
                    .get(&(loop_id, slot, lane))
                    .cloned()
                    .ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!(
                                "runtime loop derivative variable {loop_id}:{slot}:{lane:?} outside loop"
                            ),
                        )
                    });
            }
            OptValueKind::RuntimeLoopResult { .. }
            | OptValueKind::RuntimeLoopResultDerivative { .. } => {
                return Ok(value_name(value));
            }
            _ => {}
        }
    }

    if context.use_cached_fields && context.cached_values.contains(&value) {
        return context
            .cached_value_refs
            .get(&value)
            .cloned()
            .ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("missing static scalar cache slot {value}"),
                )
            });
    }

    if let Some(value_ref) = context.external_value_refs.get(&value) {
        return Ok(value_ref.clone());
    }

    if context.inline_values.contains(&value) {
        let value_slot = artifact
            .opt
            .values
            .get(usize::from(value))
            .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
        return emit_value_expr(artifact, parameter_fields, value_slot, context);
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
                    "if {}[{}]{{1.0}}else{{0.0}}",
                    context.param_given_expr,
                    usize::from(parameter)
                ));
            }
            OptValueKind::Temperature => return Ok(context.temperature_expr.clone()),
            OptValueKind::ThermalVoltage => return Ok(context.thermal_voltage_expr.clone()),
            OptValueKind::Multiplicity => return Ok(context.multiplicity_expr.to_string()),
            OptValueKind::Time => return Ok("self.time".to_string()),
            OptValueKind::Analysis { ref query } => {
                return Ok(format!(
                    "if {}{{1.0}}else{{0.0}}",
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

fn real_constant_value(artifact: &CanonicalIrArtifact, value: ValueId) -> Option<f64> {
    match artifact
        .opt
        .values
        .get(usize::from(value))
        .map(|value| &value.kind)
    {
        Some(OptValueKind::RealConstant(value)) => Some(*value),
        _ => None,
    }
}

fn emit_unary_expr(
    op: OptUnaryOp,
    input: String,
    input_type: OptValueType,
    limexp_max: &str,
    use_exp_helpers: bool,
) -> String {
    let real_input = || coerce_value_expr(input.clone(), input_type, OptValueType::Real);
    match op {
        OptUnaryOp::Pos => real_input(),
        OptUnaryOp::Neg => format!("(-{})", real_input()),
        OptUnaryOp::Not => format!("(!{})", truth_expr(input, input_type)),
        OptUnaryOp::Exp => format!("{}.exp()", f64_method_receiver(&real_input())),
        OptUnaryOp::LimExp if use_exp_helpers => format!("scalar_limexp({})", real_input()),
        OptUnaryOp::LimExp => format!(
            "{{ let limexp_arg = {}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ {limexp_max} * (1.0 + (limexp_arg - 80.0)) }} }}",
            real_input()
        ),
        OptUnaryOp::LimExpDerivative if use_exp_helpers => {
            format!("scalar_limexp_derivative({})", real_input())
        }
        OptUnaryOp::LimExpDerivative => format!(
            "{{ let limexp_arg = {}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ {limexp_max} }} }}",
            real_input()
        ),
        OptUnaryOp::LimitedExp if use_exp_helpers => {
            format!("scalar_limited_exp({})", real_input())
        }
        OptUnaryOp::LimitedExp => format!(
            "{{ let limited_exp_arg = {}; if limited_exp_arg > 80.0 {{ {limexp_max} * (1.0 + limited_exp_arg - 80.0) }} else if limited_exp_arg < -80.0 {{ 1.804851387e-35 }} else {{ limited_exp_arg.exp() }} }}",
            real_input()
        ),
        OptUnaryOp::LimitedExpDerivative if use_exp_helpers => {
            format!("scalar_limited_exp_derivative({})", real_input())
        }
        OptUnaryOp::LimitedExpDerivative => format!(
            "{{ let limited_exp_arg = {}; if limited_exp_arg > 80.0 {{ {limexp_max} }} else if limited_exp_arg < -80.0 {{ 0.0 }} else {{ limited_exp_arg.exp() }} }}",
            real_input()
        ),
        OptUnaryOp::Ln => format!("{}.ln()", f64_method_receiver(&real_input())),
        OptUnaryOp::Sqrt => format!("{}.sqrt()", f64_method_receiver(&real_input())),
        OptUnaryOp::Abs => format!("{}.abs()", f64_method_receiver(&real_input())),
        OptUnaryOp::Sin => format!("{}.sin()", f64_method_receiver(&real_input())),
        OptUnaryOp::Cos => format!("{}.cos()", f64_method_receiver(&real_input())),
        OptUnaryOp::Tan => format!("{}.tan()", f64_method_receiver(&real_input())),
        OptUnaryOp::Sinh => format!("{}.sinh()", f64_method_receiver(&real_input())),
        OptUnaryOp::Cosh => format!("{}.cosh()", f64_method_receiver(&real_input())),
        OptUnaryOp::Tanh => format!("{}.tanh()", f64_method_receiver(&real_input())),
        OptUnaryOp::Atan => format!("{}.atan()", f64_method_receiver(&real_input())),
        OptUnaryOp::Asinh => format!("{}.asinh()", f64_method_receiver(&real_input())),
        OptUnaryOp::Floor => format!("{}.floor()", f64_method_receiver(&real_input())),
        OptUnaryOp::Ceil => format!("{}.ceil()", f64_method_receiver(&real_input())),
    }
}

fn emit_pow_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    left: ValueId,
    left_type: OptValueType,
    right: ValueId,
    right_type: OptValueType,
    context: &ValueEmitContext<'_>,
) -> Result<String, RustBackendError> {
    let left = coerce_value_expr(
        value_ref(artifact, parameter_fields, left, context)?,
        left_type,
        OptValueType::Real,
    );
    if let Some(exponent) = real_constant_value(artifact, right)
        && let Some(expr) = emit_constant_power_expr(&left, exponent)
    {
        return Ok(expr);
    }
    if let Some(exponent) = integer_power_exponent_expr(artifact, parameter_fields, right, context)?
    {
        return Ok(format!("{}.powi({exponent})", f64_method_receiver(&left)));
    }

    let right = coerce_value_expr(
        value_ref(artifact, parameter_fields, right, context)?,
        right_type,
        OptValueType::Real,
    );
    Ok(format!("f64::powf({left},{right})"))
}

fn emit_string_power_expr(base: &str, exponent: &str) -> String {
    if let Some(exponent) = numeric_power_exponent_literal(exponent)
        && let Some(expr) = emit_constant_power_expr(base, exponent)
    {
        expr
    } else {
        format!("{}.powf({exponent})", f64_method_receiver(base))
    }
}

fn numeric_power_exponent_literal(value: &str) -> Option<f64> {
    let mut value = value.trim();
    while let Some(inner) = value
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
    {
        value = inner.trim();
    }
    let value = value.strip_suffix("_f64").unwrap_or(value);
    value.parse::<f64>().ok()
}

fn emit_constant_power_expr(base: &str, exponent: f64) -> Option<String> {
    let exponent = integer_power_exponent(exponent)?;
    Some(match exponent {
        0 => "1.0".to_string(),
        1 => base.to_string(),
        2 => repeated_power_expr(base, 2),
        3 => repeated_power_expr(base, 3),
        4 => quartic_power_expr(base),
        _ => format!("{}.powi({exponent})", f64_method_receiver(base)),
    })
}

fn integer_power_exponent(value: f64) -> Option<i32> {
    if !value.is_finite() || value.fract() != 0.0 {
        return None;
    }
    if value < i32::MIN as f64 || value > i32::MAX as f64 {
        return None;
    }
    Some(value as i32)
}

fn integer_power_exponent_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    value: ValueId,
    context: &ValueEmitContext<'_>,
) -> Result<Option<String>, RustBackendError> {
    let value_slot = artifact
        .opt
        .values
        .get(usize::from(value))
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
    if let OptValueKind::RealConstant(exponent) = value_slot.kind {
        return Ok(integer_power_exponent(exponent).map(|exponent| exponent.to_string()));
    }

    if integer_power_exponent_range(artifact, value)?.is_none() {
        return Ok(None);
    }

    let exponent = value_ref(artifact, parameter_fields, value, context)?;
    Ok(Some(format!("({exponent} as i32)")))
}

#[derive(Clone, Copy)]
struct IntegerPowerExponentRange {
    min: i32,
    max: i32,
}

fn integer_power_exponent_range(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
) -> Result<Option<IntegerPowerExponentRange>, RustBackendError> {
    let value_slot = artifact
        .opt
        .values
        .get(usize::from(value))
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
    match value_slot.kind {
        OptValueKind::RealConstant(value) => {
            Ok(
                integer_power_exponent(value).map(|value| IntegerPowerExponentRange {
                    min: value,
                    max: value,
                }),
            )
        }
        OptValueKind::Parameter { parameter } => {
            parameter_integer_exponent_range(artifact, parameter)
        }
        OptValueKind::Select {
            then_value,
            else_value,
            ..
        } => {
            let Some(then_range) = integer_power_exponent_range(artifact, then_value)? else {
                return Ok(None);
            };
            let Some(else_range) = integer_power_exponent_range(artifact, else_value)? else {
                return Ok(None);
            };
            Ok(Some(IntegerPowerExponentRange {
                min: then_range.min.min(else_range.min),
                max: then_range.max.max(else_range.max),
            }))
        }
        OptValueKind::Unary {
            op: OptUnaryOp::Pos,
            input,
        } => integer_power_exponent_range(artifact, input),
        OptValueKind::Unary {
            op: OptUnaryOp::Neg,
            input,
        } => {
            let Some(input_range) = integer_power_exponent_range(artifact, input)? else {
                return Ok(None);
            };
            if input_range.min == i32::MIN {
                return Ok(None);
            }
            Ok(Some(IntegerPowerExponentRange {
                min: -input_range.max,
                max: -input_range.min,
            }))
        }
        OptValueKind::Binary { op, left, right } => {
            let Some(left_range) = integer_power_exponent_range(artifact, left)? else {
                return Ok(None);
            };
            let Some(right_range) = integer_power_exponent_range(artifact, right)? else {
                return Ok(None);
            };
            Ok(match op {
                OptBinaryOp::Add => checked_integer_exponent_range(
                    left_range.min as i64 + right_range.min as i64,
                    left_range.max as i64 + right_range.max as i64,
                ),
                OptBinaryOp::Sub => checked_integer_exponent_range(
                    left_range.min as i64 - right_range.max as i64,
                    left_range.max as i64 - right_range.min as i64,
                ),
                OptBinaryOp::Mul => {
                    let products = [
                        left_range.min as i64 * right_range.min as i64,
                        left_range.min as i64 * right_range.max as i64,
                        left_range.max as i64 * right_range.min as i64,
                        left_range.max as i64 * right_range.max as i64,
                    ];
                    let min = products.iter().copied().min().unwrap_or(0);
                    let max = products.iter().copied().max().unwrap_or(0);
                    checked_integer_exponent_range(min, max)
                }
                _ => None,
            })
        }
        _ => Ok(None),
    }
}

fn parameter_integer_exponent_range(
    artifact: &CanonicalIrArtifact,
    parameter: crate::canonical_ir::ParamId,
) -> Result<Option<IntegerPowerExponentRange>, RustBackendError> {
    let parameter_slot = artifact
        .mir
        .parameters
        .get(usize::from(parameter))
        .ok_or_else(|| unsupported(artifact, format!("missing parameter {parameter}")))?;
    if parameter_slot.value_type != CanonicalValueType::Integer {
        return Ok(None);
    }

    let mut min = i32::MIN;
    let mut max = i32::MAX;
    if let Some(range) = &parameter_slot.range {
        if let Some(bound) = range
            .min
            .and_then(|value| inclusive_integer_lower_bound(value, range.min_exclusive))
        {
            min = min.max(bound);
        }
        if let Some(bound) = range
            .max
            .and_then(|value| inclusive_integer_upper_bound(value, range.max_exclusive))
        {
            max = max.min(bound);
        }
    }

    if min > max {
        return Ok(None);
    }
    Ok(Some(IntegerPowerExponentRange { min, max }))
}

fn inclusive_integer_lower_bound(value: f64, exclusive: bool) -> Option<i32> {
    if !value.is_finite() {
        return None;
    }
    let bound = if exclusive {
        value.floor() + 1.0
    } else {
        value.ceil()
    };
    if bound > i32::MAX as f64 {
        None
    } else if bound <= i32::MIN as f64 {
        Some(i32::MIN)
    } else {
        Some(bound as i32)
    }
}

fn inclusive_integer_upper_bound(value: f64, exclusive: bool) -> Option<i32> {
    if !value.is_finite() {
        return None;
    }
    let bound = if exclusive {
        value.ceil() - 1.0
    } else {
        value.floor()
    };
    if bound < i32::MIN as f64 {
        None
    } else if bound >= i32::MAX as f64 {
        Some(i32::MAX)
    } else {
        Some(bound as i32)
    }
}

fn checked_integer_exponent_range(min: i64, max: i64) -> Option<IntegerPowerExponentRange> {
    if min < i32::MIN as i64 || max > i32::MAX as i64 || min > max {
        return None;
    }
    Some(IntegerPowerExponentRange {
        min: min as i32,
        max: max as i32,
    })
}

fn repeated_power_expr(base: &str, factors: usize) -> String {
    debug_assert!(factors >= 2);
    let mut product = String::from("pb");
    for _ in 1..factors {
        product.push_str("*pb");
    }
    format!("{{let pb={base};{product}}}")
}

fn quartic_power_expr(base: &str) -> String {
    format!("{{let pb={base};let ps=pb*pb;ps*ps}}")
}

fn emit_binary_expr(
    op: OptBinaryOp,
    left: String,
    left_type: OptValueType,
    right: String,
    right_type: OptValueType,
) -> String {
    match op {
        OptBinaryOp::Add => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            "+",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Sub => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            "-",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Mul => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            "*",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Div => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            "/",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Mod => binary_expr(
            format!(
                "{}.trunc()",
                f64_method_receiver(&coerce_value_expr(left, left_type, OptValueType::Real))
            ),
            "%",
            format!(
                "{}.trunc()",
                f64_method_receiver(&coerce_value_expr(right, right_type, OptValueType::Real))
            ),
        ),
        OptBinaryOp::Pow => emit_string_power_expr(
            &coerce_value_expr(left, left_type, OptValueType::Real),
            &coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Eq => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            "==",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Ne => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            "!=",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Lt => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            "<",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Le => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            "<=",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Gt => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            ">",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::Ge => binary_expr(
            coerce_value_expr(left, left_type, OptValueType::Real),
            ">=",
            coerce_value_expr(right, right_type, OptValueType::Real),
        ),
        OptBinaryOp::And => format!(
            "({}&&{})",
            truth_expr(left, left_type),
            truth_expr(right, right_type)
        ),
        OptBinaryOp::Or => format!(
            "({}||{})",
            truth_expr(left, left_type),
            truth_expr(right, right_type)
        ),
    }
}

fn binary_expr(left: String, op: &str, right: String) -> String {
    let separator = if right.starts_with('-') { " " } else { "" };
    format!("({left}{op}{separator}{right})")
}

fn coerce_value_expr(expr: String, source_type: OptValueType, target_type: OptValueType) -> String {
    match (source_type, target_type) {
        (OptValueType::Real, OptValueType::Real)
        | (OptValueType::Boolean, OptValueType::Boolean) => expr,
        (OptValueType::Real, OptValueType::Boolean) => truth_expr(expr, OptValueType::Real),
        (OptValueType::Boolean, OptValueType::Real) => format!("(if {expr}{{1.0}}else{{0.0}})"),
    }
}

fn truth_expr(expr: String, value_type: OptValueType) -> String {
    match value_type {
        OptValueType::Boolean => expr,
        OptValueType::Real => format!("(({expr})!=0.0)"),
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

fn reactive_equation_roots(
    artifact: &CanonicalIrArtifact,
    roots: &HashMap<EquationId, ValueId>,
) -> Result<HashMap<EquationId, ValueId>, RustBackendError> {
    let mut reactive_roots = ddt_equation_roots(artifact, roots);
    for equation in &artifact.mir.equations {
        let Some(root) = roots.get(&equation.id).copied() else {
            continue;
        };
        if reactive_roots.contains_key(&equation.id) {
            continue;
        }
        if value_graph_contains_ddt(artifact, root, &mut HashSet::new())? {
            reactive_roots.insert(equation.id, root);
        }
    }
    Ok(reactive_roots)
}

fn value_graph_contains_ddt(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    visited: &mut HashSet<ValueId>,
) -> Result<bool, RustBackendError> {
    if !visited.insert(value) {
        return Ok(false);
    }
    let value_slot = artifact
        .opt
        .values
        .get(usize::from(value))
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
    match value_slot.kind {
        OptValueKind::Ddt { .. } => Ok(true),
        OptValueKind::Ddx { value: input, .. } | OptValueKind::Unary { input, .. } => {
            value_graph_contains_ddt(artifact, input, visited)
        }
        OptValueKind::Binary { left, right, .. } => {
            Ok(value_graph_contains_ddt(artifact, left, visited)?
                || value_graph_contains_ddt(artifact, right, visited)?)
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => Ok(value_graph_contains_ddt(artifact, condition, visited)?
            || value_graph_contains_ddt(artifact, then_value, visited)?
            || value_graph_contains_ddt(artifact, else_value, visited)?),
        OptValueKind::CountedSum {
            count,
            initial,
            term,
            ..
        } => Ok(value_graph_contains_ddt(artifact, count, visited)?
            || value_graph_contains_ddt(artifact, initial, visited)?
            || value_graph_contains_ddt(artifact, term, visited)?),
        OptValueKind::RuntimeLoopResult { loop_id, .. }
        | OptValueKind::RuntimeLoopResultDerivative { loop_id, .. } => {
            runtime_loop_graph_contains_ddt(artifact, loop_id, visited)
        }
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::DdtScale
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. }
        | OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::EquationValue { .. } => Ok(false),
    }
}

fn runtime_loop_graph_contains_ddt(
    artifact: &CanonicalIrArtifact,
    loop_id: u32,
    visited: &mut HashSet<ValueId>,
) -> Result<bool, RustBackendError> {
    let runtime_loop = artifact
        .opt
        .runtime_loops
        .iter()
        .find(|runtime_loop| runtime_loop.loop_id == loop_id)
        .ok_or_else(|| unsupported(artifact, format!("missing runtime loop {loop_id}")))?;
    for variable in &runtime_loop.variables {
        if value_graph_contains_ddt(artifact, variable.initial, visited)? {
            return Ok(true);
        }
    }
    if value_graph_contains_ddt(artifact, runtime_loop.condition, visited)? {
        return Ok(true);
    }
    for assignment in &runtime_loop.assignments {
        if value_graph_contains_ddt(artifact, assignment.value, visited)? {
            return Ok(true);
        }
    }
    Ok(false)
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
        roots.get(&equation.id).copied().ok_or_else(|| {
            unsupported(
                artifact,
                format!("missing scalar root for equation {}", equation.id),
            )
        })?;
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
    let empty_cache = ScalarStaticCache::empty();
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
            OptValueKind::BranchUnknownFlow { branch_unknown } => {
                branch_unknown_flow_slot(artifact, branch_unknown)?;
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

fn scalar_inline_values(
    artifact: &CanonicalIrArtifact,
    live: &HashSet<ValueId>,
    static_cache: &ScalarStaticCache,
    roots: &HashMap<EquationId, ValueId>,
) -> Result<HashSet<ValueId>, RustBackendError> {
    let mandatory = mandatory_stamp_local_values(artifact, roots)?;
    scalar_inline_values_for_live_values(artifact, live, &static_cache.set, &mandatory, &[])
}

fn scalar_stamp_inline_values(
    artifact: &CanonicalIrArtifact,
    live: &HashSet<ValueId>,
    static_cache: &ScalarStaticCache,
    roots: &HashMap<EquationId, ValueId>,
) -> Result<HashSet<ValueId>, RustBackendError> {
    let mandatory = HashSet::new();
    let external_uses = stamp_external_values(artifact, roots)?;
    scalar_inline_values_for_live_values(
        artifact,
        live,
        &static_cache.set,
        &mandatory,
        &external_uses,
    )
}

fn shared_stamp_inline_values(
    artifact: &CanonicalIrArtifact,
    static_cache: &ScalarStaticCache,
    plan: &SharedStampValuesPlan,
) -> Result<HashSet<ValueId>, RustBackendError> {
    let mandatory = plan.boundary.iter().copied().collect::<HashSet<_>>();
    scalar_inline_values_for_live_values(
        artifact,
        &plan.live,
        &static_cache.set,
        &mandatory,
        &plan.boundary,
    )
}

fn mandatory_stamp_local_values(
    artifact: &CanonicalIrArtifact,
    roots: &HashMap<EquationId, ValueId>,
) -> Result<HashSet<ValueId>, RustBackendError> {
    let mut mandatory = HashSet::new();
    for root in roots.values().copied() {
        mandatory.insert(root);
        let root_value =
            artifact.opt.values.get(usize::from(root)).ok_or_else(|| {
                unsupported(artifact, format!("missing root scalar value {root}"))
            })?;
        for derivative in &root_value.derivatives {
            mandatory.insert(derivative.value);
        }
    }
    Ok(mandatory)
}

fn stamp_external_values(
    artifact: &CanonicalIrArtifact,
    roots: &HashMap<EquationId, ValueId>,
) -> Result<Vec<ValueId>, RustBackendError> {
    let mut values = Vec::new();
    for root in roots.values().copied() {
        values.push(root);
        let root_value =
            artifact.opt.values.get(usize::from(root)).ok_or_else(|| {
                unsupported(artifact, format!("missing root scalar value {root}"))
            })?;
        values.extend(
            root_value
                .derivatives
                .iter()
                .map(|derivative| derivative.value),
        );
    }
    Ok(values)
}

fn scalar_inline_values_for_live_values(
    artifact: &CanonicalIrArtifact,
    live: &HashSet<ValueId>,
    cached_values: &HashSet<ValueId>,
    mandatory: &HashSet<ValueId>,
    external_uses: &[ValueId],
) -> Result<HashSet<ValueId>, RustBackendError> {
    let mut use_counts = scalar_value_use_counts(artifact, live, cached_values)?;
    for value in external_uses {
        if live.contains(value) && !cached_values.contains(value) {
            let Some(count) = use_counts.get_mut(usize::from(*value)) else {
                return Err(unsupported(
                    artifact,
                    format!("missing scalar value {value}"),
                ));
            };
            *count = count.saturating_add(1);
        }
    }
    let candidates: HashSet<_> = live
        .iter()
        .copied()
        .filter(|value| !cached_values.contains(value))
        .filter(|value| !mandatory.contains(value))
        .filter(|value| {
            use_counts
                .get(usize::from(*value))
                .copied()
                .unwrap_or_default()
                == 1
        })
        .filter(|value| {
            artifact
                .opt
                .values
                .get(usize::from(*value))
                .is_some_and(|value| scalar_value_can_inline(&value.kind))
        })
        .collect();

    let mut costs = vec![None; artifact.opt.values.len()];
    let mut inline_values = HashSet::new();
    for value in &candidates {
        let cost = scalar_inline_expr_cost(artifact, *value, &candidates, &mut costs)?;
        if cost <= MAX_SCALAR_INLINE_EXPR_COST {
            inline_values.insert(*value);
        }
    }
    Ok(inline_values)
}

fn scalar_value_use_counts(
    artifact: &CanonicalIrArtifact,
    live: &HashSet<ValueId>,
    cached_values: &HashSet<ValueId>,
) -> Result<Vec<usize>, RustBackendError> {
    let mut counts = vec![0usize; artifact.opt.values.len()];
    for value in live {
        if cached_values.contains(value) {
            continue;
        }
        for dependency in scalar_value_dependencies(artifact, *value)? {
            if live.contains(&dependency) && !cached_values.contains(&dependency) {
                let Some(count) = counts.get_mut(usize::from(dependency)) else {
                    return Err(unsupported(
                        artifact,
                        format!("missing scalar value {dependency}"),
                    ));
                };
                *count = count.saturating_add(1);
            }
        }
    }
    Ok(counts)
}

fn scalar_inline_expr_cost(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    candidates: &HashSet<ValueId>,
    costs: &mut [Option<usize>],
) -> Result<usize, RustBackendError> {
    if !candidates.contains(&value) {
        return Ok(1);
    }
    let index = usize::from(value);
    if let Some(cost) = costs
        .get(index)
        .copied()
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?
    {
        return Ok(cost);
    }

    let mut cost = 1usize;
    for dependency in scalar_value_dependencies(artifact, value)? {
        let dependency_cost = if candidates.contains(&dependency) {
            let cost = scalar_inline_expr_cost(artifact, dependency, candidates, costs)?;
            if cost <= MAX_SCALAR_INLINE_EXPR_COST {
                cost
            } else {
                1
            }
        } else {
            1
        };
        cost = cost.saturating_add(dependency_cost);
        if cost > MAX_SCALAR_INLINE_EXPR_COST {
            break;
        }
    }
    if let Some(slot) = costs.get_mut(index) {
        *slot = Some(cost);
    }
    Ok(cost)
}

fn scalar_value_dependencies(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
) -> Result<Vec<ValueId>, RustBackendError> {
    let value_slot = artifact
        .opt
        .values
        .get(usize::from(value))
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
    let dependencies = match value_slot.kind {
        OptValueKind::Ddx {
            value,
            pos_node,
            neg_node,
        } => projected_ddx_derivative_values(artifact, value, pos_node, neg_node)?,
        OptValueKind::Ddt { input, .. } => vec![input],
        OptValueKind::CountedSum { count, initial, .. } => vec![count, initial],
        OptValueKind::RuntimeLoopResult { loop_id, .. } => {
            runtime_loop_initial_values(artifact, loop_id)?
        }
        OptValueKind::RuntimeLoopResultDerivative { loop_id, lane, .. } => {
            runtime_loop_initial_values_for_derivative_lane(artifact, loop_id, lane)?
        }
        OptValueKind::Unary { input, .. } => vec![input],
        OptValueKind::Binary { left, right, .. } => vec![left, right],
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => vec![condition, then_value, else_value],
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::DdtScale
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. }
        | OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::EquationValue { .. } => Vec::new(),
    };
    Ok(dependencies)
}

fn runtime_loop_initial_values(
    artifact: &CanonicalIrArtifact,
    loop_id: u32,
) -> Result<Vec<ValueId>, RustBackendError> {
    let runtime_loop = artifact
        .opt
        .runtime_loops
        .iter()
        .find(|runtime_loop| runtime_loop.loop_id == loop_id)
        .ok_or_else(|| unsupported(artifact, format!("missing runtime loop {loop_id}")))?;
    Ok(runtime_loop
        .variables
        .iter()
        .map(|variable| variable.initial)
        .collect())
}

fn runtime_loop_initial_values_for_derivative_lane(
    artifact: &CanonicalIrArtifact,
    loop_id: u32,
    lane: DerivativeLane,
) -> Result<Vec<ValueId>, RustBackendError> {
    let runtime_loop = artifact
        .opt
        .runtime_loops
        .iter()
        .find(|runtime_loop| runtime_loop.loop_id == loop_id)
        .ok_or_else(|| unsupported(artifact, format!("missing runtime loop {loop_id}")))?;
    let mut values = Vec::with_capacity(runtime_loop.variables.len() * 2);
    for variable in &runtime_loop.variables {
        values.push(variable.initial);
        if let Some(derivative) = derivative_value_for_lane(artifact, variable.initial, lane)? {
            values.push(derivative);
        }
    }
    Ok(values)
}

fn scalar_value_can_inline(kind: &OptValueKind) -> bool {
    matches!(
        kind,
        OptValueKind::RealConstant(_)
            | OptValueKind::BooleanConstant(_)
            | OptValueKind::Parameter { .. }
            | OptValueKind::ParamGiven { .. }
            | OptValueKind::Temperature
            | OptValueKind::ThermalVoltage
            | OptValueKind::Multiplicity
            | OptValueKind::Time
            | OptValueKind::Analysis { .. }
            | OptValueKind::DdtScale
            | OptValueKind::NodePotential { .. }
            | OptValueKind::BranchFlow { .. }
            | OptValueKind::BranchUnknownFlow { .. }
            | OptValueKind::LoopIndex { .. }
            | OptValueKind::Unary { .. }
            | OptValueKind::Binary { .. }
            | OptValueKind::Select { .. }
    )
}

fn mark_stamp_live_value(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    static_cache: &ScalarStaticCache,
    live: &mut HashSet<ValueId>,
) -> Result<(), RustBackendError> {
    mark_live_values_iterative(
        artifact,
        value,
        &static_cache.set,
        live,
        ScalarValueDependencyMode::Stamp,
    )
}

fn mark_counted_sum_term_live(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    cached_values: &HashSet<ValueId>,
    live: &mut HashSet<ValueId>,
) -> Result<(), RustBackendError> {
    mark_live_values_iterative(
        artifact,
        value,
        cached_values,
        live,
        ScalarValueDependencyMode::CountedSum,
    )
}

fn mark_live_values_iterative(
    artifact: &CanonicalIrArtifact,
    root: ValueId,
    cached_values: &HashSet<ValueId>,
    live: &mut HashSet<ValueId>,
    mode: ScalarValueDependencyMode,
) -> Result<(), RustBackendError> {
    let mut stack = vec![root];
    while let Some(value) = stack.pop() {
        if cached_values.contains(&value) || !live.insert(value) {
            continue;
        }
        let dependencies = scalar_value_dependency_values(artifact, value, mode)?;
        stack.extend(dependencies);
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ValueVisitState {
    Unvisited,
    Visiting,
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScalarValueDependencyMode {
    Stamp,
    CountedSum,
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

fn ordered_counted_sum_values(
    artifact: &CanonicalIrArtifact,
    live: &HashSet<ValueId>,
    cached_values: &HashSet<ValueId>,
) -> Result<Vec<ValueId>, RustBackendError> {
    let mut state = vec![ValueVisitState::Unvisited; artifact.opt.values.len()];
    let mut ordered = Vec::with_capacity(live.len());

    for value in &artifact.opt.values {
        if live.contains(&value.id) && !cached_values.contains(&value.id) {
            visit_counted_sum_value(
                artifact,
                value.id,
                live,
                cached_values,
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
    visit_ordered_value_iterative(
        artifact,
        value,
        live,
        &static_cache.set,
        state,
        ordered,
        ScalarValueDependencyMode::Stamp,
    )
}

fn visit_counted_sum_value(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    live: &HashSet<ValueId>,
    cached_values: &HashSet<ValueId>,
    state: &mut [ValueVisitState],
    ordered: &mut Vec<ValueId>,
) -> Result<(), RustBackendError> {
    visit_ordered_value_iterative(
        artifact,
        value,
        live,
        cached_values,
        state,
        ordered,
        ScalarValueDependencyMode::CountedSum,
    )
}

#[derive(Clone, Copy)]
enum OrderedVisitFrame {
    Enter(ValueId),
    Exit(ValueId),
}

fn visit_ordered_value_iterative(
    artifact: &CanonicalIrArtifact,
    root: ValueId,
    live: &HashSet<ValueId>,
    cached_values: &HashSet<ValueId>,
    state: &mut [ValueVisitState],
    ordered: &mut Vec<ValueId>,
    mode: ScalarValueDependencyMode,
) -> Result<(), RustBackendError> {
    let mut stack = vec![OrderedVisitFrame::Enter(root)];
    while let Some(frame) = stack.pop() {
        match frame {
            OrderedVisitFrame::Enter(value) => {
                if cached_values.contains(&value) || !live.contains(&value) {
                    continue;
                }
                let index = usize::from(value);
                match state
                    .get(index)
                    .copied()
                    .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?
                {
                    ValueVisitState::Done => continue,
                    ValueVisitState::Visiting => {
                        return Err(unsupported(
                            artifact,
                            format!("cyclic scalar value dependency at {value}"),
                        ));
                    }
                    ValueVisitState::Unvisited => {}
                }

                state[index] = ValueVisitState::Visiting;
                stack.push(OrderedVisitFrame::Exit(value));
                let dependencies = scalar_value_dependency_values(artifact, value, mode)?;
                for dependency in dependencies.into_iter().rev() {
                    stack.push(OrderedVisitFrame::Enter(dependency));
                }
            }
            OrderedVisitFrame::Exit(value) => {
                let index = usize::from(value);
                if state.get(index).copied() == Some(ValueVisitState::Visiting) {
                    state[index] = ValueVisitState::Done;
                    ordered.push(value);
                }
            }
        }
    }
    Ok(())
}

fn scalar_value_dependency_values(
    artifact: &CanonicalIrArtifact,
    value: ValueId,
    mode: ScalarValueDependencyMode,
) -> Result<Vec<ValueId>, RustBackendError> {
    let value_slot = artifact
        .opt
        .values
        .get(usize::from(value))
        .ok_or_else(|| unsupported(artifact, format!("missing scalar value {value}")))?;
    let dependencies = match value_slot.kind {
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
        | OptValueKind::BranchUnknownFlow { .. }
        | OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::EquationValue { .. }
        | OptValueKind::DdtScale => Vec::new(),
        OptValueKind::RuntimeLoopResult { loop_id, .. }
            if mode == ScalarValueDependencyMode::Stamp =>
        {
            runtime_loop_initial_values(artifact, loop_id)?
        }
        OptValueKind::RuntimeLoopResultDerivative { loop_id, lane, .. }
            if mode == ScalarValueDependencyMode::Stamp =>
        {
            runtime_loop_initial_values_for_derivative_lane(artifact, loop_id, lane)?
        }
        OptValueKind::RuntimeLoopResult { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. } => Vec::new(),
        OptValueKind::CountedSum {
            count,
            initial,
            term,
            ..
        } => match mode {
            ScalarValueDependencyMode::Stamp => vec![count, initial],
            ScalarValueDependencyMode::CountedSum => vec![count, initial, term],
        },
        OptValueKind::Ddx {
            value: input,
            pos_node,
            neg_node,
        } => {
            let mut dependencies = vec![input];
            dependencies.extend(projected_ddx_derivative_values(
                artifact, input, pos_node, neg_node,
            )?);
            dependencies
        }
        OptValueKind::Ddt { input, .. } | OptValueKind::Unary { input, .. } => vec![input],
        OptValueKind::Binary { left, right, .. } => vec![left, right],
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => vec![condition, then_value, else_value],
    };
    Ok(dependencies)
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
    let runtime_loop_assignments: usize = artifact
        .opt
        .runtime_loops
        .iter()
        .map(|runtime_loop| runtime_loop.assignments.len())
        .sum();
    if runtime_loop_assignments > MAX_SCALAR_RUNTIME_LOOP_ASSIGNMENTS {
        return Err(unsupported(
            artifact,
            format!(
                "scalar OptIR runtime loops have {runtime_loop_assignments} assignments; current scalar emitter budget is {MAX_SCALAR_RUNTIME_LOOP_ASSIGNMENTS}"
            ),
        ));
    }

    let runtime_loop_variables: usize = artifact
        .opt
        .runtime_loops
        .iter()
        .map(|runtime_loop| runtime_loop.variables.len())
        .sum();
    if runtime_loop_variables > MAX_SCALAR_RUNTIME_LOOP_VARIABLES {
        return Err(unsupported(
            artifact,
            format!(
                "scalar OptIR runtime loops have {runtime_loop_variables} variables; current scalar emitter budget is {MAX_SCALAR_RUNTIME_LOOP_VARIABLES}"
            ),
        ));
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
            OptValueKind::BranchUnknownFlow { branch_unknown } => {
                branch_unknown_flow_slot(artifact, branch_unknown)?;
            }
            OptValueKind::ParamGiven { .. }
            | OptValueKind::Temperature
            | OptValueKind::ThermalVoltage
            | OptValueKind::Multiplicity
            | OptValueKind::Time
            | OptValueKind::Analysis { .. }
            | OptValueKind::Ddx { .. }
            | OptValueKind::Ddt { .. }
            | OptValueKind::DdtScale => {}
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

fn value_name(value: ValueId) -> String {
    compact_scalar_identifier(value.index())
}

fn compact_scalar_identifier(mut index: u32) -> String {
    const FIRST: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const REST: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let first_base = FIRST.len() as u32;
    let rest_base = REST.len() as u32;
    let mut width = 1u32;
    let mut rest_scale = 1u32;

    loop {
        let capacity = first_base
            .checked_mul(rest_scale)
            .expect("scalar identifier capacity overflow");
        if index < capacity {
            let first = index / rest_scale;
            let mut remainder = index % rest_scale;
            let mut bytes = Vec::with_capacity(width as usize);
            bytes.push(FIRST[first as usize]);
            let mut suffix = vec![REST[0]; width.saturating_sub(1) as usize];
            for slot in suffix.iter_mut().rev() {
                *slot = REST[(remainder % rest_base) as usize];
                remainder /= rest_base;
            }
            bytes.extend(suffix);
            let mut name = String::from_utf8(bytes).expect("scalar identifier alphabet is UTF-8");
            if is_reserved_scalar_identifier(&name) {
                name.push('_');
            }
            return name;
        }
        index -= capacity;
        width += 1;
        rest_scale = rest_scale
            .checked_mul(rest_base)
            .expect("scalar identifier scale overflow");
    }
}

fn is_reserved_scalar_identifier(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "gen"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
            | "bool"
            | "f32"
            | "f64"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "Some"
            | "None"
            | "Ok"
            | "Err"
            | "ctx"
            | "stamper"
            | "p"
            | "n"
            | "br"
            | "pg"
            | "m"
            | "nodes"
            | "branches"
            | "param_given"
            | "multiplicity"
            | "sf"
            | "sb"
            | "common"
            | "timestep"
            | "ddt_state_current"
            | "ddt_state_previous"
            | "ddt_state_older"
            | "ddt_state_initialized"
            | "ddt_derivative_current"
            | "ddt_derivative_previous"
            | "idt_state_current"
            | "idt_state_previous"
            | "idt_state_initialized"
            | "ddt_active"
            | "ddt_scale"
            | "ddt_previous_value_scale"
            | "ddt_older_value_scale"
            | "ddt_previous_derivative_scale"
            | "idt_scale"
            | "counted_sum"
            | "limexp_arg"
            | "limited_exp_arg"
    )
}

fn runtime_loop_value_name(loop_id: u32, slot: u32) -> String {
    format!("r{loop_id}_{slot}")
}

fn runtime_loop_derivative_name(loop_id: u32, slot: u32, lane: DerivativeLane) -> String {
    match lane.kind {
        DerivativeLaneKind::Node => format!("r{loop_id}_{slot}n{}", lane.index),
        DerivativeLaneKind::BranchUnknown => format!("r{loop_id}_{slot}b{}", lane.index),
    }
}

fn runtime_loop_guard_name(loop_id: u32) -> String {
    format!("r{loop_id}g")
}

fn cached_or_local_value_name(value: ValueId, static_cache: &ScalarStaticCache) -> String {
    if static_cache.contains(value) {
        static_cache
            .cache_ref(value)
            .expect("cached scalar value must have generated state slot")
            .to_string()
    } else {
        value_name(value)
    }
}

fn scaled_derivative_value_expr(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    value: ValueId,
    context: &ValueEmitContext<'_>,
    scale: &str,
) -> Result<String, RustBackendError> {
    Ok(scaled_derivative_expr(
        value_ref(artifact, parameter_fields, value, context)?,
        scale,
    ))
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

fn branch_unknown_flow_slot(
    artifact: &CanonicalIrArtifact,
    branch_unknown: BranchUnknownId,
) -> Result<usize, RustBackendError> {
    artifact
        .mir
        .branch_unknowns
        .get(usize::from(branch_unknown))
        .map(|unknown| usize::from(unknown.id))
        .ok_or_else(|| unsupported(artifact, format!("missing branch unknown {branch_unknown}")))
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
    if let Some(typed) = typed_f64_literal(value) {
        format!("({typed})")
    } else {
        format!("({value})")
    }
}

fn typed_f64_literal(value: &str) -> Option<String> {
    let value = value.trim();
    if value.ends_with("_f64") || value.starts_with("f64::") {
        return None;
    }
    let saw_float_marker = scan_numeric_literal(value)?;
    if saw_float_marker {
        Some(format!("{value}_f64"))
    } else {
        None
    }
}

fn scan_numeric_literal(value: &str) -> Option<bool> {
    let unsigned = value.strip_prefix('-').unwrap_or(value);
    if unsigned.is_empty() || !unsigned.as_bytes()[0].is_ascii_digit() {
        return None;
    }

    let mut saw_digit = false;
    let mut saw_float_marker = false;
    let mut previous_was_exponent = false;
    for byte in unsigned.bytes() {
        match byte {
            b'0'..=b'9' => {
                saw_digit = true;
                previous_was_exponent = false;
            }
            b'.' => {
                saw_float_marker = true;
                previous_was_exponent = false;
            }
            b'e' | b'E' => {
                saw_float_marker = true;
                previous_was_exponent = true;
            }
            b'+' | b'-' if previous_was_exponent => {
                previous_was_exponent = false;
            }
            _ => return None,
        }
    }

    if saw_digit {
        Some(saw_float_marker)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_emitter_coerces_boolean_numeric_operands() {
        assert_eq!(
            coerce_value_expr(
                "flag".to_string(),
                OptValueType::Boolean,
                OptValueType::Real,
            ),
            "(if flag{1.0}else{0.0})"
        );
        assert_eq!(
            emit_binary_expr(
                OptBinaryOp::Mul,
                "flag".to_string(),
                OptValueType::Boolean,
                "x".to_string(),
                OptValueType::Real,
            ),
            "((if flag{1.0}else{0.0})*x)"
        );
        assert_eq!(
            emit_binary_expr(
                OptBinaryOp::Lt,
                "flag".to_string(),
                OptValueType::Boolean,
                "x".to_string(),
                OptValueType::Real,
            ),
            "((if flag{1.0}else{0.0})<x)"
        );
        assert_eq!(
            emit_binary_expr(
                OptBinaryOp::Mod,
                "x".to_string(),
                OptValueType::Real,
                "2.0".to_string(),
                OptValueType::Real,
            ),
            "((x).trunc()%(2.0_f64).trunc())"
        );
        assert_eq!(
            emit_unary_expr(
                OptUnaryOp::Sqrt,
                "flag".to_string(),
                OptValueType::Boolean,
                "1.0",
                false,
            ),
            "((if flag{1.0}else{0.0})).sqrt()"
        );
    }

    #[test]
    fn scalar_emitter_specializes_constant_integer_powers() {
        assert_eq!(emit_constant_power_expr("x", 0.0).as_deref(), Some("1.0"));
        assert_eq!(emit_constant_power_expr("x", 1.0).as_deref(), Some("x"));
        assert_eq!(
            emit_constant_power_expr("x", 2.0).as_deref(),
            Some("{let pb=x;pb*pb}")
        );
        assert_eq!(
            emit_constant_power_expr("x", 3.0).as_deref(),
            Some("{let pb=x;pb*pb*pb}")
        );
        assert_eq!(
            emit_constant_power_expr("x", 4.0).as_deref(),
            Some("{let pb=x;let ps=pb*pb;ps*ps}")
        );
        assert_eq!(emit_constant_power_expr("x", 0.5), None);
        assert_eq!(emit_string_power_expr("x", "2.0"), "{let pb=x;pb*pb}");
        assert_eq!(emit_string_power_expr("x", "(3.0)"), "{let pb=x;pb*pb*pb}");
        assert_eq!(
            emit_string_power_expr("x", "4.0_f64"),
            "{let pb=x;let ps=pb*pb;ps*ps}"
        );
        assert_eq!(emit_string_power_expr("x", "0.5"), "(x).powf(0.5)");
        assert_eq!(
            emit_binary_expr(
                OptBinaryOp::Pow,
                "x".to_string(),
                OptValueType::Real,
                "2.0".to_string(),
                OptValueType::Real,
            ),
            "{let pb=x;pb*pb}"
        );
    }

    #[test]
    fn scalar_source_line_budget_uses_compact_binding_estimate() {
        let compact_line_budget_values =
            MAX_SCALAR_STAMP_SOURCE_LINES - SCALAR_STAMP_SOURCE_LINE_OVERHEAD_RESERVE;
        let compact_binding_budget =
            compact_line_budget_values * MIN_COMPACT_SCALAR_VALUE_BINDINGS_PER_LINE;

        assert_eq!(
            scalar_stamp_packed_binding_line_estimate(compact_binding_budget),
            compact_line_budget_values
        );
        assert!(!scalar_stamp_source_line_estimate_exceeds_budget(
            compact_binding_budget
        ));
        assert!(scalar_stamp_source_line_estimate_exceeds_budget(
            compact_binding_budget + MIN_COMPACT_SCALAR_VALUE_BINDINGS_PER_LINE
        ));
        assert!(scalar_stamp_emitted_values_exceeds_budget(
            MAX_SCALAR_STAMP_EMITTED_VALUES + 1
        ));
    }

    #[test]
    fn scalar_emitted_value_budget_matches_live_value_budget() {
        assert_eq!(
            MAX_SCALAR_STAMP_EMITTED_VALUES,
            MAX_SCALAR_STAMP_LIVE_VALUES
        );
        assert!(!scalar_stamp_emitted_values_exceeds_budget(680_156));
    }

    #[test]
    fn compact_scalar_identifiers_are_short_and_reserved_safe() {
        assert_eq!(compact_scalar_identifier(0), "a");
        assert_eq!(compact_scalar_identifier(12), "m_");
        assert_eq!(compact_scalar_identifier(13), "n_");
        assert_eq!(compact_scalar_identifier(15), "p_");
        assert_eq!(compact_scalar_identifier(52), "a0");
        assert_eq!(compact_scalar_identifier(27_231), "gen_");

        let mut seen = HashSet::new();
        for index in 0..20_000 {
            let name = compact_scalar_identifier(index);
            assert!(
                seen.insert(name.clone()),
                "duplicate scalar identifier {name}"
            );
            let raw = name.trim_end_matches('_');
            assert!(
                !is_reserved_scalar_identifier(raw) || name.ends_with('_'),
                "unescaped reserved scalar identifier {name}"
            );
            let mut chars = name.chars();
            let first = chars.next().expect("identifier is not empty");
            assert!(first == '_' || first.is_ascii_alphabetic(), "{name}");
            assert!(
                chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric()),
                "{name}"
            );
        }
    }
}
