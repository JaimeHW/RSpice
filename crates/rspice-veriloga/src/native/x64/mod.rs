pub(crate) mod codegen;
pub mod encoder;

use super::expr::{
    BranchUnknownRuntimeMapping, CanonicalDerivativeAxis, EntryKind, NativeLoweringLimits,
    NativeOp, NativeProgram, PriorCurrentProbe, canonical_above_slots_for_equation,
    canonical_absdelay_slots_for_equation, canonical_cross_slots_for_equation,
    canonical_ddt_slots_for_equation, canonical_idt_slots_for_equation,
    canonical_idtmod_slots_for_equation, canonical_laplace_slots_for_equation,
    canonical_limit_slots_for_equation, canonical_slew_slots_for_equation,
    canonical_table_lookup_slots_for_equation, canonical_timer_slots_for_equation,
    canonical_transition_slots_for_equation, canonical_zi_slots_for_equation,
    constant_dynamic_variable_slot,
};
use super::model::{CodeOffset, NativeCurrentDependencies, NativeEntryOffsets, NativeModel};
use super::runtime::ExecutableMemory;
use super::{JitError, JitResult};
use crate::canonical_ir::{
    CanonicalIrArtifact, EquationId, ExprId, HirAnalogOperator, HirExprKind, HirExpression,
    HirLaplaceKind, HirZiKind, MirEquationKind, MirModel, NodeId, SourceSpanRef,
};
use crate::codegen::{
    AssignmentStep, BytecodeProgram, ColumnAxis, CompiledModel, CompiledNoiseSource, Instruction,
    JacobianEntry, StampIndex, StampProgram,
};
use crate::native::x64::codegen::NativeAssignment;
use crate::vm::{CURRENT_PAIR_GROUND, terminal_pair_current_index};
use smol_str::SmolStr;

const ENTRY_ALIGNMENT: usize = 16;
const X64_NOP: u8 = 0x90;

pub(crate) fn compile_model(model: &CompiledModel) -> JitResult<NativeModel> {
    compile_model_inner(model, None)
}

pub(crate) fn compile_model_with_canonical_ir(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<NativeModel> {
    validate_canonical_artifact_for_model(model, artifact)?;
    compile_model_inner(model, Some(&artifact.mir))
}

fn compile_model_inner(
    model: &CompiledModel,
    canonical_mir: Option<&MirModel>,
) -> JitResult<NativeModel> {
    super::validate_native_coverage(model)?;
    let canonical_branch_unknown_map = match canonical_mir {
        Some(mir) => canonical_branch_unknown_runtime_map(model, mir)?,
        None => Vec::new(),
    };
    let base_limits = NativeLoweringLimits::for_model(model)
        .with_canonical_branch_unknown_map(&canonical_branch_unknown_map);
    let canonical_noise_plan = match canonical_mir {
        Some(mir) => Some(build_canonical_noise_plan(model, mir)?),
        None => None,
    };

    let mut image = Vec::new();
    let assignment = append_assignment_entry(model, &mut image)
        .map_err(|error| context_jit_error(error, "assignments"))?;

    let mut parameter_defaults = Vec::with_capacity(model.parameters.len());
    for (parameter_index, parameter) in model.parameters.iter().enumerate() {
        let default_entry = if let Some(program) = &parameter.default_program {
            let program = lower_parameter_default_program(
                model,
                canonical_mir,
                parameter_index,
                program,
                base_limits,
            )
            .map_err(|error| {
                context_jit_error(error, format!("parameter default {parameter_index}"))
            })?;
            Some(append_value_entry(&mut image, &program)?)
        } else {
            None
        };
        parameter_defaults.push(default_entry);
    }

    let mut static_conditions = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_values = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_value_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_value_prior_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobian_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobian_prior_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobian_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobian_prior_current_dependencies =
        Vec::with_capacity(model.stamp_programs.len());
    let mut noise_psd = Vec::with_capacity(model.noise_sources.len());
    let mut noise_psd_current_dependencies = Vec::with_capacity(model.noise_sources.len());
    let mut noise_psd_prior_current_dependencies = Vec::with_capacity(model.noise_sources.len());
    let mut noise_exponents = Vec::with_capacity(model.noise_sources.len());
    let mut noise_exponent_current_dependencies = Vec::with_capacity(model.noise_sources.len());
    let mut noise_exponent_prior_current_dependencies =
        Vec::with_capacity(model.noise_sources.len());
    let mut available_current_pairs = Vec::new();
    let mut prior_current_probes = Vec::new();

    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        let static_condition = if let Some(condition) = &stamp.static_condition {
            let program = lower_static_condition_program(
                model,
                canonical_mir,
                stamp_index,
                condition,
                base_limits,
            )
            .map_err(|error| context_jit_error(error, format!("static condition {stamp_index}")))?;
            Some(append_value_entry(&mut image, &program)?)
        } else {
            None
        };
        static_conditions.push(static_condition);

        let value_limits = base_limits
            .with_available_current_pairs(&available_current_pairs)
            .with_prior_current_probes(&prior_current_probes);
        let program = lower_stamp_value_program(
            model,
            canonical_mir,
            stamp_index,
            &stamp.value_program,
            value_limits,
        )
        .map_err(|error| context_jit_error(error, format!("stamp value {stamp_index}")))?;
        stamp_value_current_dependencies.push(program.current_pair_dependencies().to_vec());
        stamp_value_prior_current_dependencies.push(program.prior_current_dependencies().to_vec());
        stamp_values.push(append_value_entry(&mut image, &program)?);

        let mut jacobian_current_pairs = available_current_pairs.clone();
        if let Some((pos, neg)) = infer_current_terminal_pair(stamp) {
            push_current_pair_indices(
                model,
                &mut jacobian_current_pairs,
                model.num_terminals,
                pos,
                neg,
            )?;
        }
        let mut jacobian_prior_current_probes = prior_current_probes.clone();
        if stamp.branch_ordinal.is_none()
            && let Some((pos, neg)) = infer_current_unified_pair(model, stamp)
        {
            push_prior_current_probe_aliases(
                &mut jacobian_prior_current_probes,
                stamp_index,
                pos,
                neg,
            );
        }
        let jacobian_limits = base_limits
            .with_available_current_pairs(&jacobian_current_pairs)
            .with_prior_current_probes(&jacobian_prior_current_probes);

        let mut stamp_jacobians = Vec::with_capacity(stamp.jacobian_programs.len());
        let mut stamp_jacobian_current_dependencies =
            Vec::with_capacity(stamp.jacobian_programs.len());
        let mut stamp_jacobian_prior_current_dependencies =
            Vec::with_capacity(stamp.jacobian_programs.len());
        for jacobian in &stamp.jacobian_programs {
            let program = lower_jacobian_program(
                model,
                canonical_mir,
                stamp_index,
                &stamp.value_program,
                jacobian,
                jacobian_limits,
            )
            .map_err(|error| {
                context_jit_error(
                    error,
                    format!("jacobian {stamp_index}.{}", stamp_jacobians.len()),
                )
            })?;
            stamp_jacobian_current_dependencies.push(program.current_pair_dependencies().to_vec());
            stamp_jacobian_prior_current_dependencies
                .push(program.prior_current_dependencies().to_vec());
            stamp_jacobians.push(append_value_entry(&mut image, &program)?);
        }
        jacobians.push(stamp_jacobians);
        jacobian_current_dependencies.push(stamp_jacobian_current_dependencies);
        jacobian_prior_current_dependencies.push(stamp_jacobian_prior_current_dependencies);

        let mut stamp_reactive_jacobians = Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_current_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_prior_current_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        for reactive_jacobian in &stamp.reactive_jacobians {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::ReactiveJacobian,
                &reactive_jacobian.program,
                base_limits,
            )
            .map_err(|error| {
                context_jit_error(
                    error,
                    format!(
                        "reactive jacobian {stamp_index}.{}",
                        stamp_reactive_jacobians.len()
                    ),
                )
            })?;
            stamp_reactive_jacobian_current_dependencies
                .push(program.current_pair_dependencies().to_vec());
            stamp_reactive_jacobian_prior_current_dependencies
                .push(program.prior_current_dependencies().to_vec());
            stamp_reactive_jacobians.push(append_value_entry(&mut image, &program)?);
        }
        reactive_jacobians.push(stamp_reactive_jacobians);
        reactive_jacobian_current_dependencies.push(stamp_reactive_jacobian_current_dependencies);
        reactive_jacobian_prior_current_dependencies
            .push(stamp_reactive_jacobian_prior_current_dependencies);

        if let Some((pos, neg)) = infer_current_terminal_pair(stamp) {
            push_current_pair_indices(
                model,
                &mut available_current_pairs,
                model.num_terminals,
                pos,
                neg,
            )?;
        }
        if stamp.branch_ordinal.is_none()
            && let Some((pos, neg)) = infer_current_unified_pair(model, stamp)
        {
            push_prior_current_probe_aliases(&mut prior_current_probes, stamp_index, pos, neg);
        }
    }

    let noise_limits = base_limits
        .with_available_current_pairs(&available_current_pairs)
        .with_prior_current_probes(&prior_current_probes);
    for (source_index, source) in model.noise_sources.iter().enumerate() {
        let psd_program = lower_noise_psd_program(
            model,
            canonical_noise_plan.as_ref(),
            source_index,
            source,
            &source.psd_program,
            noise_limits,
        )
        .map_err(|error| context_jit_error(error, format!("noise psd {}", noise_psd.len())))?;
        noise_psd_current_dependencies.push(psd_program.current_pair_dependencies().to_vec());
        noise_psd_prior_current_dependencies
            .push(psd_program.prior_current_dependencies().to_vec());
        noise_psd.push(append_value_entry(&mut image, &psd_program)?);

        let exponent_entry = if let Some(program) = &source.exponent_program {
            let exponent_program = lower_noise_exponent_program(
                model,
                canonical_noise_plan.as_ref(),
                source_index,
                source,
                program,
                noise_limits,
            )
            .map_err(|error| {
                context_jit_error(error, format!("noise exponent {}", noise_exponents.len()))
            })?;
            noise_exponent_current_dependencies
                .push(exponent_program.current_pair_dependencies().to_vec());
            noise_exponent_prior_current_dependencies
                .push(exponent_program.prior_current_dependencies().to_vec());
            Some(append_value_entry(&mut image, &exponent_program)?)
        } else {
            noise_exponent_current_dependencies.push(Vec::new());
            noise_exponent_prior_current_dependencies.push(Vec::new());
            None
        };
        noise_exponents.push(exponent_entry);
    }

    let entries = NativeEntryOffsets {
        assignment,
        parameter_defaults,
        static_conditions,
        stamp_values,
        jacobians,
        reactive_jacobians,
        noise_psd,
        noise_exponents,
    };
    let current_dependencies = NativeCurrentDependencies {
        stamp_values: stamp_value_current_dependencies,
        stamp_value_prior_currents: stamp_value_prior_current_dependencies,
        jacobians: jacobian_current_dependencies,
        jacobian_prior_currents: jacobian_prior_current_dependencies,
        reactive_jacobians: reactive_jacobian_current_dependencies,
        reactive_jacobian_prior_currents: reactive_jacobian_prior_current_dependencies,
        noise_psd: noise_psd_current_dependencies,
        noise_psd_prior_currents: noise_psd_prior_current_dependencies,
        noise_exponents: noise_exponent_current_dependencies,
        noise_exponent_prior_currents: noise_exponent_prior_current_dependencies,
    };
    validate_compiled_entry_shape(model, &entries, &current_dependencies)?;

    let executable = ExecutableMemory::allocate(&image)?;
    NativeModel::from_executable_image_with_dependencies(
        model.num_variables,
        model.parameters.len(),
        executable,
        entries,
        current_dependencies,
    )
}

fn lower_static_condition_program(
    model: &CompiledModel,
    canonical_mir: Option<&MirModel>,
    stamp_index: usize,
    bytecode_program: &BytecodeProgram,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    if let Some(mir) = canonical_mir {
        let (mir, expr) = canonical_static_condition_expr(model, mir, stamp_index)?;
        return NativeProgram::from_mir_expression(
            model.name.clone(),
            EntryKind::StaticCondition,
            &mir,
            expr,
            limits,
        );
    }

    NativeProgram::from_bytecode(
        model.name.clone(),
        EntryKind::StaticCondition,
        bytecode_program,
        limits,
    )
}

fn lower_jacobian_program(
    model: &CompiledModel,
    canonical_mir: Option<&MirModel>,
    stamp_index: usize,
    stamp_value_program: &BytecodeProgram,
    jacobian: &JacobianEntry,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    if let Some(mir) = canonical_mir {
        let equation_id = canonical_equation_id(model, stamp_index)?;
        let axis = canonical_derivative_axis_for_column(model, mir, &jacobian.col_axis)?;
        let table_lookup_slots = canonical_table_lookup_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            stamp_value_program,
        )?;
        let limits = limits.with_canonical_table_lookup_slots(&table_lookup_slots);
        let canonical = NativeProgram::from_mir_derivative(
            model.name.clone(),
            EntryKind::Jacobian,
            mir,
            equation_id,
            axis,
            limits,
        );
        match canonical {
            Ok(program) => return Ok(program),
            Err(JitError::UnsupportedCanonicalOp { .. }) => {}
            Err(error) => return Err(error),
        }
    }

    NativeProgram::from_bytecode(
        model.name.clone(),
        EntryKind::Jacobian,
        &jacobian.program,
        limits,
    )
}

fn canonical_equation_id(model: &CompiledModel, stamp_index: usize) -> JitResult<EquationId> {
    u32::try_from(stamp_index)
        .map(EquationId::new)
        .map_err(|_| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!("stamp index {stamp_index} exceeds canonical equation id range").into(),
        })
}

fn canonical_derivative_axis_for_column(
    model: &CompiledModel,
    mir: &MirModel,
    col_axis: &ColumnAxis,
) -> JitResult<CanonicalDerivativeAxis> {
    match col_axis {
        ColumnAxis::Node(node) => {
            let canonical_node = canonical_node_id_for_compiled_axis(model, mir, *node)?;
            Ok(CanonicalDerivativeAxis::Node(canonical_node))
        }
        ColumnAxis::Branch(branch) if *branch < model.branch_sources.len() => {
            Ok(CanonicalDerivativeAxis::Branch(*branch))
        }
        ColumnAxis::Branch(branch) => Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "compiled jacobian branch axis {branch} exceeds branch source count {}",
                model.branch_sources.len()
            )
            .into(),
        }),
    }
}

fn canonical_node_id_for_compiled_axis(
    model: &CompiledModel,
    mir: &MirModel,
    node: usize,
) -> JitResult<NodeId> {
    let node_count = model
        .num_terminals
        .checked_add(model.internal_nodes)
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: "compiled node count overflows usize".into(),
        })?;
    if node >= node_count {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!("compiled jacobian node axis {node} exceeds node count {node_count}")
                .into(),
        });
    }
    let node_id =
        u32::try_from(node)
            .map(NodeId::new)
            .map_err(|_| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "compiled jacobian node axis {node} exceeds canonical node id range"
                )
                .into(),
            })?;
    let canonical_node = mir
        .nodes
        .get(node)
        .filter(|canonical_node| canonical_node.id == node_id)
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!("canonical node {node_id} is outside MIR node table").into(),
        })?;
    let expected_external = node < model.num_terminals;
    if canonical_node.is_external != expected_external {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical node {node_id} external flag {} does not match compiled jacobian node axis {node}",
                canonical_node.is_external
            )
            .into(),
        });
    }
    Ok(node_id)
}

fn canonical_static_condition_expr(
    model: &CompiledModel,
    canonical_mir: &MirModel,
    stamp_index: usize,
) -> JitResult<(MirModel, ExprId)> {
    let equation =
        canonical_mir
            .equations
            .get(stamp_index)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!("canonical equation {stamp_index} is outside equation table")
                    .into(),
            })?;
    let mut synthetic = canonical_mir.clone();
    let expr = extract_leading_static_condition(
        model,
        &mut synthetic,
        stamp_index,
        equation.expression.id,
    )?;
    Ok((synthetic, expr))
}

fn extract_leading_static_condition(
    model: &CompiledModel,
    mir: &mut MirModel,
    stamp_index: usize,
    root: ExprId,
) -> JitResult<ExprId> {
    let mut condition = None;
    let mut current = root;

    loop {
        let expression = canonical_expression(model, mir, current)?.clone();
        match expression.kind {
            HirExprKind::Conditional {
                condition: guard,
                then_expr,
                else_expr,
            } if is_canonical_zero(model, mir, else_expr)? => {
                condition = Some(match condition {
                    Some(previous) => {
                        append_canonical_binary(mir, "And", previous, guard, expression.span)
                    }
                    None => guard,
                });
                current = then_expr;
            }
            _ => {
                return condition.ok_or_else(|| JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical equation {stamp_index} is missing leading static-condition guard"
                    )
                    .into(),
                });
            }
        }
    }
}

fn is_canonical_zero(model: &CompiledModel, mir: &MirModel, expr_id: ExprId) -> JitResult<bool> {
    match &canonical_expression(model, mir, expr_id)?.kind {
        HirExprKind::Number { value, .. } => Ok(value.to_bits() == 0.0_f64.to_bits()),
        _ => Ok(false),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CanonicalNoiseKind {
    White,
    Flicker,
    Table,
}

impl CanonicalNoiseKind {
    fn from_canonical_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "white" => Some(Self::White),
            "flicker" => Some(Self::Flicker),
            "table" => Some(Self::Table),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::White => "white",
            Self::Flicker => "flicker",
            Self::Table => "table",
        }
    }
}

#[derive(Debug, Clone)]
struct CanonicalNoiseEntry {
    kind: CanonicalNoiseKind,
    name: Option<SmolStr>,
    psd_expr: ExprId,
    exponent_expr: Option<ExprId>,
    table_points: Option<Vec<(f64, f64)>>,
}

struct CanonicalNoiseLoweringPlan {
    mir: MirModel,
    entries: Vec<CanonicalNoiseEntry>,
}

impl CanonicalNoiseLoweringPlan {
    fn entry(&self, model: &CompiledModel, source_index: usize) -> JitResult<&CanonicalNoiseEntry> {
        self.entries
            .get(source_index)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical noise plan is missing compiled noise source {source_index}"
                )
                .into(),
            })
    }
}

fn lower_noise_psd_program(
    model: &CompiledModel,
    canonical_noise_plan: Option<&CanonicalNoiseLoweringPlan>,
    source_index: usize,
    source: &CompiledNoiseSource,
    bytecode_program: &BytecodeProgram,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    if let Some(plan) = canonical_noise_plan {
        let entry = plan.entry(model, source_index)?;
        validate_canonical_noise_entry_matches_source(model, source_index, source, entry)?;
        return NativeProgram::from_mir_expression(
            model.name.clone(),
            EntryKind::StampValue,
            &plan.mir,
            entry.psd_expr,
            limits,
        );
    }

    NativeProgram::from_bytecode(
        model.name.clone(),
        EntryKind::StampValue,
        bytecode_program,
        limits,
    )
}

fn lower_noise_exponent_program(
    model: &CompiledModel,
    canonical_noise_plan: Option<&CanonicalNoiseLoweringPlan>,
    source_index: usize,
    source: &CompiledNoiseSource,
    bytecode_program: &BytecodeProgram,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    if let Some(plan) = canonical_noise_plan {
        let entry = plan.entry(model, source_index)?;
        validate_canonical_noise_entry_matches_source(model, source_index, source, entry)?;
        let expr = entry
            .exponent_expr
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical noise source {source_index} is missing flicker exponent expression"
                )
                .into(),
            })?;
        return NativeProgram::from_mir_expression(
            model.name.clone(),
            EntryKind::StampValue,
            &plan.mir,
            expr,
            limits,
        );
    }

    NativeProgram::from_bytecode(
        model.name.clone(),
        EntryKind::StampValue,
        bytecode_program,
        limits,
    )
}

fn build_canonical_noise_plan(
    model: &CompiledModel,
    canonical_mir: &MirModel,
) -> JitResult<CanonicalNoiseLoweringPlan> {
    let mut mir = canonical_mir.clone();
    let mut canonical_by_equation = Vec::with_capacity(model.stamp_programs.len());

    for equation_index in 0..model.stamp_programs.len() {
        let expected_count = model
            .noise_sources
            .iter()
            .filter(|source| source.program_idx == equation_index)
            .count();
        if expected_count == 0 {
            canonical_by_equation.push(Vec::new());
            continue;
        }

        let root = mir
            .equations
            .get(equation_index)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!("canonical equation {equation_index} is outside equation table")
                    .into(),
            })?
            .expression
            .id;
        let entries = extract_canonical_noise_entries(model, &mut mir, equation_index, root)?;
        if entries.len() != expected_count {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical equation {equation_index} has {} noise sources but compiled model has {expected_count}",
                    entries.len()
                )
                .into(),
            });
        }
        canonical_by_equation.push(entries);
    }

    let mut next_by_equation = vec![0_usize; model.stamp_programs.len()];
    let mut entries = Vec::with_capacity(model.noise_sources.len());
    for (source_index, source) in model.noise_sources.iter().enumerate() {
        let Some(next) = next_by_equation.get_mut(source.program_idx) else {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "compiled noise source {source_index} references stamp {} outside canonical equation table",
                    source.program_idx
                )
                .into(),
            });
        };
        let entry = canonical_by_equation[source.program_idx]
            .get(*next)
            .cloned()
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical equation {} is missing noise source ordinal {}",
                    source.program_idx, *next
                )
                .into(),
            })?;
        *next += 1;
        validate_canonical_noise_entry_matches_source(model, source_index, source, &entry)?;
        entries.push(entry);
    }

    Ok(CanonicalNoiseLoweringPlan { mir, entries })
}

fn extract_canonical_noise_entries(
    model: &CompiledModel,
    mir: &mut MirModel,
    equation_index: usize,
    root: ExprId,
) -> JitResult<Vec<CanonicalNoiseEntry>> {
    let span = mir
        .equations
        .get(equation_index)
        .map(|equation| equation.span)
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!("canonical equation {equation_index} is outside equation table").into(),
        })?;
    let amplitude = append_canonical_number(mir, 1.0, "1.0", span);
    let mut entries = Vec::new();
    extract_canonical_noise_expr(model, mir, equation_index, root, amplitude, &mut entries)?;
    Ok(entries)
}

fn extract_canonical_noise_expr(
    model: &CompiledModel,
    mir: &mut MirModel,
    equation_index: usize,
    expr_id: ExprId,
    amplitude: ExprId,
    out: &mut Vec<CanonicalNoiseEntry>,
) -> JitResult<()> {
    if !canonical_expr_contains_noise(model, mir, expr_id)? {
        return Ok(());
    }

    let expression = canonical_expression(model, mir, expr_id)?.clone();
    match expression.kind {
        HirExprKind::NoiseSource {
            source,
            operands,
            name,
        } => {
            if canonical_expr_list_contains_noise(model, mir, &operands)? {
                return Err(unsupported_canonical_noise_position(
                    model,
                    equation_index,
                    "noise-source operand",
                ));
            }
            let kind =
                CanonicalNoiseKind::from_canonical_name(source.as_str()).ok_or_else(|| {
                    JitError::InvalidCanonicalIr {
                        model: model.name.clone(),
                        detail: format!(
                            "canonical equation {equation_index} has unsupported noise source '{}'",
                            source
                        )
                        .into(),
                    }
                })?;
            let amplitude_square =
                append_canonical_binary(mir, "Mul", amplitude, amplitude, expression.span);
            let (psd_expr, exponent_expr, table_points) = match kind {
                CanonicalNoiseKind::White => {
                    require_canonical_noise_operand_count(
                        model,
                        equation_index,
                        kind,
                        &operands,
                        1,
                    )?;
                    let psd = append_canonical_binary(
                        mir,
                        "Mul",
                        amplitude_square,
                        operands[0],
                        expression.span,
                    );
                    (psd, None, None)
                }
                CanonicalNoiseKind::Flicker => {
                    require_canonical_noise_operand_count(
                        model,
                        equation_index,
                        kind,
                        &operands,
                        2,
                    )?;
                    let psd = append_canonical_binary(
                        mir,
                        "Mul",
                        amplitude_square,
                        operands[0],
                        expression.span,
                    );
                    (psd, Some(operands[1]), None)
                }
                CanonicalNoiseKind::Table => {
                    let table_points =
                        canonical_table_points(model, mir, equation_index, &operands)?;
                    (amplitude_square, None, Some(table_points))
                }
            };
            out.push(CanonicalNoiseEntry {
                kind,
                name,
                psd_expr,
                exponent_expr,
                table_points,
            });
            Ok(())
        }
        HirExprKind::SystemFunction { name, args } | HirExprKind::Call { name, args } => {
            let Some(kind) = canonical_noise_intrinsic_kind(name.as_str()) else {
                return Err(unsupported_canonical_noise_position(
                    model,
                    equation_index,
                    "function call",
                ));
            };
            let name = canonical_optional_noise_name(model, mir, equation_index, &args, kind)?;
            let amplitude_square =
                append_canonical_binary(mir, "Mul", amplitude, amplitude, expression.span);
            let (psd_expr, exponent_expr, table_points) = match kind {
                CanonicalNoiseKind::White => {
                    require_canonical_noise_arg_range(
                        model,
                        equation_index,
                        kind,
                        args.len(),
                        1,
                        2,
                    )?;
                    reject_nested_canonical_noise(model, mir, equation_index, &args[..1])?;
                    let psd = append_canonical_binary(
                        mir,
                        "Mul",
                        amplitude_square,
                        args[0],
                        expression.span,
                    );
                    (psd, None, None)
                }
                CanonicalNoiseKind::Flicker => {
                    require_canonical_noise_arg_range(
                        model,
                        equation_index,
                        kind,
                        args.len(),
                        2,
                        3,
                    )?;
                    reject_nested_canonical_noise(model, mir, equation_index, &args[..2])?;
                    let psd = append_canonical_binary(
                        mir,
                        "Mul",
                        amplitude_square,
                        args[0],
                        expression.span,
                    );
                    (psd, Some(args[1]), None)
                }
                CanonicalNoiseKind::Table => {
                    require_canonical_noise_arg_range(
                        model,
                        equation_index,
                        kind,
                        args.len(),
                        1,
                        2,
                    )?;
                    reject_nested_canonical_noise(model, mir, equation_index, &args[..1])?;
                    let table_points =
                        canonical_table_points_from_expr(model, mir, equation_index, args[0])?;
                    (amplitude_square, None, Some(table_points))
                }
            };
            out.push(CanonicalNoiseEntry {
                kind,
                name,
                psd_expr,
                exponent_expr,
                table_points,
            });
            Ok(())
        }
        HirExprKind::Binary { op, left, right } => match op.as_str() {
            "Add" | "Sub" => {
                extract_canonical_noise_expr(model, mir, equation_index, left, amplitude, out)?;
                extract_canonical_noise_expr(model, mir, equation_index, right, amplitude, out)
            }
            "Mul" => {
                let left_has_noise = canonical_expr_contains_noise(model, mir, left)?;
                let right_has_noise = canonical_expr_contains_noise(model, mir, right)?;
                match (left_has_noise, right_has_noise) {
                    (true, true) => Err(unsupported_canonical_noise_position(
                        model,
                        equation_index,
                        "product of noise terms",
                    )),
                    (true, false) => {
                        let scaled_amplitude =
                            append_canonical_binary(mir, "Mul", amplitude, right, expression.span);
                        extract_canonical_noise_expr(
                            model,
                            mir,
                            equation_index,
                            left,
                            scaled_amplitude,
                            out,
                        )
                    }
                    (false, true) => {
                        let scaled_amplitude =
                            append_canonical_binary(mir, "Mul", amplitude, left, expression.span);
                        extract_canonical_noise_expr(
                            model,
                            mir,
                            equation_index,
                            right,
                            scaled_amplitude,
                            out,
                        )
                    }
                    (false, false) => Ok(()),
                }
            }
            "Div" => {
                if canonical_expr_contains_noise(model, mir, right)? {
                    return Err(unsupported_canonical_noise_position(
                        model,
                        equation_index,
                        "divisor",
                    ));
                }
                let scaled_amplitude =
                    append_canonical_binary(mir, "Div", amplitude, right, expression.span);
                extract_canonical_noise_expr(
                    model,
                    mir,
                    equation_index,
                    left,
                    scaled_amplitude,
                    out,
                )
            }
            _ => Err(unsupported_canonical_noise_position(
                model,
                equation_index,
                "nonlinear binary expression",
            )),
        },
        HirExprKind::Unary { op, operand } => match op.as_str() {
            "Neg" | "Pos" => {
                extract_canonical_noise_expr(model, mir, equation_index, operand, amplitude, out)
            }
            _ => Err(unsupported_canonical_noise_position(
                model,
                equation_index,
                "nonlinear unary expression",
            )),
        },
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            if canonical_expr_contains_noise(model, mir, condition)? {
                return Err(unsupported_canonical_noise_position(
                    model,
                    equation_index,
                    "conditional guard",
                ));
            }
            if canonical_expr_contains_noise(model, mir, then_expr)? {
                let one = append_canonical_number(mir, 1.0, "1.0", expression.span);
                let zero = append_canonical_number(mir, 0.0, "0.0", expression.span);
                let gate = append_canonical_expr(
                    mir,
                    HirExprKind::Conditional {
                        condition,
                        then_expr: one,
                        else_expr: zero,
                    },
                    expression.span,
                );
                let gated_amplitude =
                    append_canonical_binary(mir, "Mul", amplitude, gate, expression.span);
                extract_canonical_noise_expr(
                    model,
                    mir,
                    equation_index,
                    then_expr,
                    gated_amplitude,
                    out,
                )?;
            }
            if canonical_expr_contains_noise(model, mir, else_expr)? {
                let one = append_canonical_number(mir, 1.0, "1.0", expression.span);
                let zero = append_canonical_number(mir, 0.0, "0.0", expression.span);
                let gate = append_canonical_expr(
                    mir,
                    HirExprKind::Conditional {
                        condition,
                        then_expr: zero,
                        else_expr: one,
                    },
                    expression.span,
                );
                let gated_amplitude =
                    append_canonical_binary(mir, "Mul", amplitude, gate, expression.span);
                extract_canonical_noise_expr(
                    model,
                    mir,
                    equation_index,
                    else_expr,
                    gated_amplitude,
                    out,
                )?;
            }
            Ok(())
        }
        _ => Err(unsupported_canonical_noise_position(
            model,
            equation_index,
            "nonlinear or dynamic position",
        )),
    }
}

fn canonical_expr_contains_noise(
    model: &CompiledModel,
    mir: &MirModel,
    expr_id: ExprId,
) -> JitResult<bool> {
    let expression = canonical_expression(model, mir, expr_id)?;
    match &expression.kind {
        HirExprKind::NoiseSource { .. } => Ok(true),
        HirExprKind::SystemFunction { name, .. } | HirExprKind::Call { name, .. }
            if canonical_noise_intrinsic_kind(name.as_str()).is_some() =>
        {
            Ok(true)
        }
        HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. } => Ok(false),
        HirExprKind::SystemFunction { args, .. }
        | HirExprKind::Call { args, .. }
        | HirExprKind::ArrayLiteral { elements: args } => {
            canonical_expr_list_contains_noise(model, mir, args)
        }
        HirExprKind::Unary { operand, .. } | HirExprKind::ArrayAccess { index: operand, .. } => {
            canonical_expr_contains_noise(model, mir, *operand)
        }
        HirExprKind::Binary { left, right, .. } => {
            Ok(canonical_expr_contains_noise(model, mir, *left)?
                || canonical_expr_contains_noise(model, mir, *right)?)
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => Ok(canonical_expr_contains_noise(model, mir, *condition)?
            || canonical_expr_contains_noise(model, mir, *then_expr)?
            || canonical_expr_contains_noise(model, mir, *else_expr)?),
        HirExprKind::AnalogOperator { op } => {
            canonical_analog_operator_contains_noise(model, mir, op)
        }
        HirExprKind::Laplace { expr, kind } => {
            Ok(canonical_expr_contains_noise(model, mir, *expr)?
                || canonical_laplace_kind_contains_noise(model, mir, kind)?)
        }
        HirExprKind::Zi { expr, kind } => Ok(canonical_expr_contains_noise(model, mir, *expr)?
            || canonical_zi_kind_contains_noise(model, mir, kind)?),
    }
}

fn canonical_analog_operator_contains_noise(
    model: &CompiledModel,
    mir: &MirModel,
    op: &HirAnalogOperator,
) -> JitResult<bool> {
    match op {
        HirAnalogOperator::Ddt { expr, abstol } => {
            Ok(canonical_expr_contains_noise(model, mir, *expr)?
                || canonical_optional_expr_contains_noise(model, mir, *abstol)?)
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => Ok(canonical_expr_contains_noise(model, mir, *expr)?
            || canonical_optional_expr_contains_noise(model, mir, *ic)?
            || canonical_optional_expr_contains_noise(model, mir, *assert)?
            || canonical_optional_expr_contains_noise(model, mir, *abstol)?),
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => Ok(canonical_expr_contains_noise(model, mir, *expr)?
            || canonical_optional_expr_contains_noise(model, mir, *ic)?
            || canonical_optional_expr_contains_noise(model, mir, *modulus)?
            || canonical_optional_expr_contains_noise(model, mir, *offset)?
            || canonical_optional_expr_contains_noise(model, mir, *abstol)?),
        HirAnalogOperator::Ddx { expr, probe } => {
            Ok(canonical_expr_contains_noise(model, mir, *expr)?
                || canonical_expr_contains_noise(model, mir, *probe)?)
        }
        HirAnalogOperator::Limexp { expr } | HirAnalogOperator::LastCrossing { expr, .. } => {
            canonical_expr_contains_noise(model, mir, *expr)
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => Ok(canonical_expr_contains_noise(model, mir, *expr)?
            || canonical_expr_contains_noise(model, mir, *delay)?
            || canonical_optional_expr_contains_noise(model, mir, *max_delay)?),
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => Ok(canonical_expr_contains_noise(model, mir, *expr)?
            || canonical_optional_expr_contains_noise(model, mir, *delay)?
            || canonical_optional_expr_contains_noise(model, mir, *rise)?
            || canonical_optional_expr_contains_noise(model, mir, *fall)?
            || canonical_optional_expr_contains_noise(model, mir, *tolerance)?),
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => Ok(canonical_expr_contains_noise(model, mir, *expr)?
            || canonical_optional_expr_contains_noise(model, mir, *max_rise)?
            || canonical_optional_expr_contains_noise(model, mir, *max_fall)?),
    }
}

fn canonical_laplace_kind_contains_noise(
    model: &CompiledModel,
    mir: &MirModel,
    kind: &HirLaplaceKind,
) -> JitResult<bool> {
    match kind {
        HirLaplaceKind::ZeroPole { zeros, poles } => {
            Ok(canonical_expr_list_contains_noise(model, mir, zeros)?
                || canonical_expr_list_contains_noise(model, mir, poles)?)
        }
        HirLaplaceKind::ZeroDenominator { zeros, denominator } => {
            Ok(canonical_expr_list_contains_noise(model, mir, zeros)?
                || canonical_expr_list_contains_noise(model, mir, denominator)?)
        }
        HirLaplaceKind::NumeratorPole { numerator, poles } => {
            Ok(canonical_expr_list_contains_noise(model, mir, numerator)?
                || canonical_expr_list_contains_noise(model, mir, poles)?)
        }
        HirLaplaceKind::NumeratorDenominator {
            numerator,
            denominator,
        } => Ok(canonical_expr_list_contains_noise(model, mir, numerator)?
            || canonical_expr_list_contains_noise(model, mir, denominator)?),
    }
}

fn canonical_zi_kind_contains_noise(
    model: &CompiledModel,
    mir: &MirModel,
    kind: &HirZiKind,
) -> JitResult<bool> {
    match kind {
        HirZiKind::ZeroPole { zeros, poles } => {
            Ok(canonical_expr_list_contains_noise(model, mir, zeros)?
                || canonical_expr_list_contains_noise(model, mir, poles)?)
        }
        HirZiKind::ZeroDenominator { zeros, denominator } => {
            Ok(canonical_expr_list_contains_noise(model, mir, zeros)?
                || canonical_expr_list_contains_noise(model, mir, denominator)?)
        }
        HirZiKind::NumeratorPole { numerator, poles } => {
            Ok(canonical_expr_list_contains_noise(model, mir, numerator)?
                || canonical_expr_list_contains_noise(model, mir, poles)?)
        }
        HirZiKind::NumeratorDenominator {
            numerator,
            denominator,
        } => Ok(canonical_expr_list_contains_noise(model, mir, numerator)?
            || canonical_expr_list_contains_noise(model, mir, denominator)?),
    }
}

fn canonical_expr_list_contains_noise(
    model: &CompiledModel,
    mir: &MirModel,
    exprs: &[ExprId],
) -> JitResult<bool> {
    for expr in exprs {
        if canonical_expr_contains_noise(model, mir, *expr)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn reject_nested_canonical_noise(
    model: &CompiledModel,
    mir: &MirModel,
    equation_index: usize,
    exprs: &[ExprId],
) -> JitResult<()> {
    if canonical_expr_list_contains_noise(model, mir, exprs)? {
        return Err(unsupported_canonical_noise_position(
            model,
            equation_index,
            "noise-source operand",
        ));
    }
    Ok(())
}

fn canonical_optional_expr_contains_noise(
    model: &CompiledModel,
    mir: &MirModel,
    expr: Option<ExprId>,
) -> JitResult<bool> {
    match expr {
        Some(expr) => canonical_expr_contains_noise(model, mir, expr),
        None => Ok(false),
    }
}

fn canonical_table_points(
    model: &CompiledModel,
    mir: &MirModel,
    equation_index: usize,
    operands: &[ExprId],
) -> JitResult<Vec<(f64, f64)>> {
    if operands.len() % 2 != 0 {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical equation {equation_index} table noise has odd operand count {}",
                operands.len()
            )
            .into(),
        });
    }

    let mut points = Vec::with_capacity(operands.len() / 2);
    for pair in operands.chunks_exact(2) {
        points.push((
            canonical_number_value(model, mir, pair[0])?,
            canonical_number_value(model, mir, pair[1])?,
        ));
    }
    Ok(points)
}

fn canonical_table_points_from_expr(
    model: &CompiledModel,
    mir: &MirModel,
    equation_index: usize,
    expr_id: ExprId,
) -> JitResult<Vec<(f64, f64)>> {
    match &canonical_expression(model, mir, expr_id)?.kind {
        HirExprKind::ArrayLiteral { elements } => {
            canonical_table_points(model, mir, equation_index, elements)
        }
        other => Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical equation {equation_index} table noise data is {}, expected array literal",
                canonical_expr_kind_name(other)
            )
            .into(),
        }),
    }
}

fn canonical_number_value(
    model: &CompiledModel,
    mir: &MirModel,
    expr_id: ExprId,
) -> JitResult<f64> {
    match &canonical_expression(model, mir, expr_id)?.kind {
        HirExprKind::Number { value, .. } => Ok(*value),
        other => Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical table noise point expression {expr_id} is {}, expected number",
                canonical_expr_kind_name(other)
            )
            .into(),
        }),
    }
}

fn canonical_noise_intrinsic_kind(name: &str) -> Option<CanonicalNoiseKind> {
    let normalized = name.trim_start_matches('$').to_ascii_lowercase();
    match normalized.as_str() {
        "white_noise" => Some(CanonicalNoiseKind::White),
        "flicker_noise" => Some(CanonicalNoiseKind::Flicker),
        "noise_table" | "noise_table_log" => Some(CanonicalNoiseKind::Table),
        _ => None,
    }
}

fn canonical_optional_noise_name(
    model: &CompiledModel,
    mir: &MirModel,
    equation_index: usize,
    args: &[ExprId],
    kind: CanonicalNoiseKind,
) -> JitResult<Option<SmolStr>> {
    let name_index = match kind {
        CanonicalNoiseKind::White | CanonicalNoiseKind::Table => 1,
        CanonicalNoiseKind::Flicker => 2,
    };
    let Some(expr_id) = args.get(name_index).copied() else {
        return Ok(None);
    };
    match &canonical_expression(model, mir, expr_id)?.kind {
        HirExprKind::StringLiteral { value } => Ok(Some(value.clone())),
        other => Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical equation {equation_index} {} noise name argument is {}, expected string",
                kind.as_str(),
                canonical_expr_kind_name(other)
            )
            .into(),
        }),
    }
}

fn validate_canonical_noise_entry_matches_source(
    model: &CompiledModel,
    source_index: usize,
    source: &CompiledNoiseSource,
    entry: &CanonicalNoiseEntry,
) -> JitResult<()> {
    let compiled_kind = compiled_noise_kind(source);
    if entry.kind != compiled_kind {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical noise source {source_index} kind '{}' does not match compiled kind '{}'",
                entry.kind.as_str(),
                compiled_kind.as_str()
            )
            .into(),
        });
    }
    if entry.name != source.name {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical noise source {source_index} name {:?} does not match compiled name {:?}",
                entry.name, source.name
            )
            .into(),
        });
    }
    if entry.exponent_expr.is_some() != source.exponent_program.is_some() {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical noise source {source_index} exponent presence does not match compiled model"
            )
            .into(),
        });
    }
    if let Some((compiled_points, _)) = &source.table {
        let Some(canonical_points) = &entry.table_points else {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical noise source {source_index} is missing table point metadata"
                )
                .into(),
            });
        };
        if !noise_table_points_match(canonical_points, compiled_points) {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical noise source {source_index} table points do not match compiled model"
                )
                .into(),
            });
        }
    }
    Ok(())
}

fn compiled_noise_kind(source: &CompiledNoiseSource) -> CanonicalNoiseKind {
    if source.table.is_some() {
        CanonicalNoiseKind::Table
    } else if source.exponent_program.is_some() {
        CanonicalNoiseKind::Flicker
    } else {
        CanonicalNoiseKind::White
    }
}

fn noise_table_points_match(canonical: &[(f64, f64)], compiled: &[(f64, f64)]) -> bool {
    canonical.len() == compiled.len()
        && canonical.iter().zip(compiled).all(|((cf, cp), (bf, bp))| {
            cf.to_bits() == bf.to_bits() && cp.to_bits() == bp.to_bits()
        })
}

fn require_canonical_noise_operand_count(
    model: &CompiledModel,
    equation_index: usize,
    kind: CanonicalNoiseKind,
    operands: &[ExprId],
    expected: usize,
) -> JitResult<()> {
    if operands.len() == expected {
        return Ok(());
    }
    Err(JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!(
            "canonical equation {equation_index} {} noise source has {} operands, expected {expected}",
            kind.as_str(),
            operands.len()
        )
        .into(),
    })
}

fn require_canonical_noise_arg_range(
    model: &CompiledModel,
    equation_index: usize,
    kind: CanonicalNoiseKind,
    actual: usize,
    min: usize,
    max: usize,
) -> JitResult<()> {
    if (min..=max).contains(&actual) {
        return Ok(());
    }
    Err(JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!(
            "canonical equation {equation_index} {} noise source has {actual} arguments, expected {min}..={max}",
            kind.as_str()
        )
        .into(),
    })
}

fn canonical_expression<'a>(
    model: &CompiledModel,
    mir: &'a MirModel,
    expr_id: ExprId,
) -> JitResult<&'a HirExpression> {
    mir.expressions
        .get(usize::from(expr_id))
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!("canonical expression {expr_id} is outside MIR expression arena")
                .into(),
        })
}

fn append_canonical_number(
    mir: &mut MirModel,
    value: f64,
    raw: &'static str,
    span: SourceSpanRef,
) -> ExprId {
    append_canonical_expr(
        mir,
        HirExprKind::Number {
            value,
            raw: raw.into(),
        },
        span,
    )
}

fn append_canonical_binary(
    mir: &mut MirModel,
    op: &'static str,
    left: ExprId,
    right: ExprId,
    span: SourceSpanRef,
) -> ExprId {
    append_canonical_expr(
        mir,
        HirExprKind::Binary {
            op: op.into(),
            left,
            right,
        },
        span,
    )
}

fn append_canonical_expr(mir: &mut MirModel, kind: HirExprKind, span: SourceSpanRef) -> ExprId {
    let id = ExprId::from(mir.expressions.len());
    mir.expressions.push(HirExpression { id, kind, span });
    id
}

fn unsupported_canonical_noise_position(
    model: &CompiledModel,
    equation_index: usize,
    position: &str,
) -> JitError {
    JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!(
            "canonical equation {equation_index} places noise source in a {position}, which cannot be lowered as a native noise-analysis entry"
        )
        .into(),
    }
}

fn canonical_expr_kind_name(kind: &HirExprKind) -> &'static str {
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
        HirExprKind::AnalogOperator { .. } => "analog_operator",
        HirExprKind::Laplace { .. } => "laplace",
        HirExprKind::Zi { .. } => "zi",
        HirExprKind::NoiseSource { .. } => "noise_source",
    }
}

fn lower_parameter_default_program(
    model: &CompiledModel,
    canonical_mir: Option<&MirModel>,
    parameter_index: usize,
    bytecode_program: &crate::codegen::BytecodeProgram,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    if let Some(mir) = canonical_mir {
        let parameter =
            mir.parameters
                .get(parameter_index)
                .ok_or_else(|| JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical parameter index {parameter_index} is outside parameter table"
                    )
                    .into(),
                })?;
        let expr = parameter
            .default_expr
            .as_ref()
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical parameter '{}' is missing dependent default expression",
                    parameter.name
                )
                .into(),
            })?;
        return NativeProgram::from_mir_expression(
            model.name.clone(),
            EntryKind::ParameterDefault,
            mir,
            expr.id,
            limits,
        );
    }

    NativeProgram::from_bytecode(
        model.name.clone(),
        EntryKind::ParameterDefault,
        bytecode_program,
        limits,
    )
}

fn validate_compiled_entry_shape(
    model: &CompiledModel,
    entries: &NativeEntryOffsets,
    dependencies: &NativeCurrentDependencies,
) -> JitResult<()> {
    validate_compiled_entry_count(
        model,
        "parameter-default",
        entries.parameter_defaults.len(),
        model.parameters.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "static-condition",
        entries.static_conditions.len(),
        model.stamp_programs.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "stamp-value",
        entries.stamp_values.len(),
        model.stamp_programs.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "jacobian stamp",
        entries.jacobians.len(),
        model.stamp_programs.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "reactive-jacobian stamp",
        entries.reactive_jacobians.len(),
        model.stamp_programs.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise PSD",
        entries.noise_psd.len(),
        model.noise_sources.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise exponent",
        entries.noise_exponents.len(),
        model.noise_sources.len(),
    )?;

    for (index, (entry, parameter)) in entries
        .parameter_defaults
        .iter()
        .zip(&model.parameters)
        .enumerate()
    {
        if entry.is_some() != parameter.default_program.is_some() {
            return Err(compiled_entry_shape_error(
                model,
                format!("parameter-default {index} optional entry does not match compiled program"),
            ));
        }
    }

    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        if entries.static_conditions[stamp_index].is_some() != stamp.static_condition.is_some() {
            return Err(compiled_entry_shape_error(
                model,
                format!(
                    "static-condition {stamp_index} optional entry does not match compiled guard"
                ),
            ));
        }

        validate_compiled_entry_count(
            model,
            format!("jacobian {stamp_index}"),
            entries.jacobians[stamp_index].len(),
            stamp.jacobian_programs.len(),
        )?;
        validate_compiled_entry_count(
            model,
            format!("reactive-jacobian {stamp_index}"),
            entries.reactive_jacobians[stamp_index].len(),
            stamp.reactive_jacobians.len(),
        )?;
    }

    for (index, (entry, source)) in entries
        .noise_exponents
        .iter()
        .zip(&model.noise_sources)
        .enumerate()
    {
        if entry.is_some() != source.exponent_program.is_some() {
            return Err(compiled_entry_shape_error(
                model,
                format!("noise exponent {index} optional entry does not match compiled program"),
            ));
        }
    }

    validate_current_dependency_shape(model, entries, dependencies)?;
    Ok(())
}

fn validate_current_dependency_shape(
    model: &CompiledModel,
    entries: &NativeEntryOffsets,
    dependencies: &NativeCurrentDependencies,
) -> JitResult<()> {
    validate_compiled_entry_count(
        model,
        "stamp-value current dependency",
        dependencies.stamp_values.len(),
        entries.stamp_values.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "stamp-value prior-current dependency",
        dependencies.stamp_value_prior_currents.len(),
        entries.stamp_values.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "jacobian current dependency stamp",
        dependencies.jacobians.len(),
        entries.jacobians.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "jacobian prior-current dependency stamp",
        dependencies.jacobian_prior_currents.len(),
        entries.jacobians.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "reactive-jacobian current dependency stamp",
        dependencies.reactive_jacobians.len(),
        entries.reactive_jacobians.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "reactive-jacobian prior-current dependency stamp",
        dependencies.reactive_jacobian_prior_currents.len(),
        entries.reactive_jacobians.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise PSD current dependency",
        dependencies.noise_psd.len(),
        entries.noise_psd.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise PSD prior-current dependency",
        dependencies.noise_psd_prior_currents.len(),
        entries.noise_psd.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise exponent current dependency",
        dependencies.noise_exponents.len(),
        entries.noise_exponents.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise exponent prior-current dependency",
        dependencies.noise_exponent_prior_currents.len(),
        entries.noise_exponents.len(),
    )?;

    for (stamp_index, (dependency_entries, entry_offsets)) in dependencies
        .jacobians
        .iter()
        .zip(&entries.jacobians)
        .enumerate()
    {
        validate_compiled_entry_count(
            model,
            format!("jacobian current dependency {stamp_index}"),
            dependency_entries.len(),
            entry_offsets.len(),
        )?;
    }
    for (stamp_index, (dependency_entries, entry_offsets)) in dependencies
        .jacobian_prior_currents
        .iter()
        .zip(&entries.jacobians)
        .enumerate()
    {
        validate_compiled_entry_count(
            model,
            format!("jacobian prior-current dependency {stamp_index}"),
            dependency_entries.len(),
            entry_offsets.len(),
        )?;
    }
    for (stamp_index, (dependency_entries, entry_offsets)) in dependencies
        .reactive_jacobians
        .iter()
        .zip(&entries.reactive_jacobians)
        .enumerate()
    {
        validate_compiled_entry_count(
            model,
            format!("reactive-jacobian current dependency {stamp_index}"),
            dependency_entries.len(),
            entry_offsets.len(),
        )?;
    }
    for (stamp_index, (dependency_entries, entry_offsets)) in dependencies
        .reactive_jacobian_prior_currents
        .iter()
        .zip(&entries.reactive_jacobians)
        .enumerate()
    {
        validate_compiled_entry_count(
            model,
            format!("reactive-jacobian prior-current dependency {stamp_index}"),
            dependency_entries.len(),
            entry_offsets.len(),
        )?;
    }

    Ok(())
}

fn validate_compiled_entry_count(
    model: &CompiledModel,
    label: impl std::fmt::Display,
    actual: usize,
    expected: usize,
) -> JitResult<()> {
    if actual != expected {
        return Err(compiled_entry_shape_error(
            model,
            format!("{label} entry shape {actual} does not match compiled shape {expected}"),
        ));
    }
    Ok(())
}

fn compiled_entry_shape_error(
    model: &CompiledModel,
    detail: impl Into<smol_str::SmolStr>,
) -> JitError {
    JitError::InternalCompilerError {
        model: model.name.clone(),
        detail: detail.into(),
    }
}

fn context_jit_error(error: JitError, context: impl std::fmt::Display) -> JitError {
    let prefix = context.to_string();
    match error {
        JitError::UnsupportedCanonicalOp { model, op } => JitError::UnsupportedCanonicalOp {
            model,
            op: format!("{prefix}: {op}").into(),
        },
        JitError::UnsupportedNativeCoverage { model, feature } => {
            JitError::UnsupportedNativeCoverage {
                model,
                feature: format!("{prefix}: {feature}").into(),
            }
        }
        JitError::InvalidCanonicalIr { model, detail } => JitError::InvalidCanonicalIr {
            model,
            detail: format!("{prefix}: {detail}").into(),
        },
        JitError::Lowering { model, detail } => JitError::Lowering {
            model,
            detail: format!("{prefix}: {detail}").into(),
        },
        JitError::Verifier { model, detail } => JitError::Verifier {
            model,
            detail: format!("{prefix}: {detail}").into(),
        },
        JitError::RegisterAllocation { model, detail } => JitError::RegisterAllocation {
            model,
            detail: format!("{prefix}: {detail}").into(),
        },
        JitError::Encoding { model, detail } => JitError::Encoding {
            model,
            detail: format!("{prefix}: {detail}").into(),
        },
        JitError::Relocation { model, detail } => JitError::Relocation {
            model,
            detail: format!("{prefix}: {detail}").into(),
        },
        JitError::AbiMismatch { model, detail } => JitError::AbiMismatch {
            model,
            detail: format!("{prefix}: {detail}").into(),
        },
        JitError::MissingEntryPoint { model, entry } => JitError::MissingEntryPoint {
            model,
            entry: format!("{prefix}: {entry}").into(),
        },
        JitError::InternalCompilerError { model, detail } => JitError::InternalCompilerError {
            model,
            detail: format!("{prefix}: {detail}").into(),
        },
        other => other,
    }
}

fn validate_canonical_artifact_for_model(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<()> {
    artifact
        .validate()
        .map_err(|diagnostics| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: diagnostics
                .first()
                .map(|diagnostic| diagnostic.message.clone())
                .unwrap_or_else(|| "canonical artifact validation failed".into())
                .into(),
        })?;

    if artifact.mir.module_name != model.name {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical module '{}' does not match compiled model '{}'",
                artifact.mir.module_name, model.name
            )
            .into(),
        });
    }

    validate_canonical_parameters_for_model(model, &artifact.mir)?;

    if artifact.mir.equations.len() != model.stamp_programs.len() {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical equation count {} does not match stamp program count {}",
                artifact.mir.equations.len(),
                model.stamp_programs.len()
            )
            .into(),
        });
    }

    for (index, (equation, stamp)) in artifact
        .mir
        .equations
        .iter()
        .zip(&model.stamp_programs)
        .enumerate()
    {
        validate_canonical_equation_matches_stamp(model, &artifact.mir, index, equation, stamp)?;
    }

    Ok(())
}

fn validate_canonical_parameters_for_model(model: &CompiledModel, mir: &MirModel) -> JitResult<()> {
    if mir.parameters.len() != model.parameters.len() {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical parameter count {} does not match compiled parameter count {}",
                mir.parameters.len(),
                model.parameters.len()
            )
            .into(),
        });
    }

    for (index, (canonical, compiled)) in mir.parameters.iter().zip(&model.parameters).enumerate() {
        if canonical.name != compiled.name {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical parameter {index} name '{}' does not match compiled parameter '{}'",
                    canonical.name, compiled.name
                )
                .into(),
            });
        }
        if compiled.default_program.is_none() && canonical.default != Some(compiled.default) {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical parameter '{}' default {:?} does not match compiled default {}",
                    canonical.name, canonical.default, compiled.default
                )
                .into(),
            });
        }
        if compiled.default_program.is_some() && canonical.default_expr.is_none() {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical parameter '{}' is missing dependent default expression",
                    canonical.name
                )
                .into(),
            });
        }
    }

    Ok(())
}

fn validate_canonical_equation_matches_stamp(
    model: &CompiledModel,
    mir: &MirModel,
    index: usize,
    equation: &crate::canonical_ir::MirEquation,
    stamp: &StampProgram,
) -> JitResult<()> {
    let expected_kind = compiled_equation_kind(stamp);
    if equation.kind != expected_kind {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical equation {index} kind {:?} does not match compiled stamp kind {:?}",
                equation.kind, expected_kind
            )
            .into(),
        });
    }

    let canonical_pos = canonical_branch_endpoint(model, mir, equation.branch.pos_node)?;
    let canonical_neg = canonical_branch_endpoint(model, mir, equation.branch.neg_node)?;
    let compiled_pair = match expected_kind {
        MirEquationKind::Current => infer_current_unified_pair(model, stamp),
        MirEquationKind::Potential | MirEquationKind::Indirect => {
            Some(compiled_branch_pair_for_stamp(model, stamp, index)?)
        }
    };

    if let Some((compiled_pos, compiled_neg)) = compiled_pair
        && (canonical_pos, canonical_neg) != (compiled_pos, compiled_neg)
    {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical equation {index} branch {} does not match compiled stamp branch {}",
                format_current_pair(canonical_pos, canonical_neg),
                format_current_pair(compiled_pos, compiled_neg)
            )
            .into(),
        });
    }

    Ok(())
}

fn compiled_equation_kind(stamp: &StampProgram) -> MirEquationKind {
    if stamp.indirect {
        MirEquationKind::Indirect
    } else if stamp.branch_ordinal.is_some() {
        MirEquationKind::Potential
    } else {
        MirEquationKind::Current
    }
}

fn compiled_branch_pair_for_stamp(
    model: &CompiledModel,
    stamp: &StampProgram,
    stamp_index: usize,
) -> JitResult<(usize, usize)> {
    let ordinal = stamp
        .branch_ordinal
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!("compiled stamp {stamp_index} has no branch source ordinal").into(),
        })?;
    let source = model
        .branch_sources
        .get(ordinal)
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "compiled stamp {stamp_index} branch ordinal {ordinal} is outside branch source table"
            )
            .into(),
        })?;
    Ok((
        compiled_branch_endpoint(model, &source.pos)?,
        compiled_branch_endpoint(model, &source.neg)?,
    ))
}

fn canonical_branch_unknown_runtime_map(
    model: &CompiledModel,
    mir: &MirModel,
) -> JitResult<Vec<BranchUnknownRuntimeMapping>> {
    if mir.branch_unknowns.is_empty() {
        return Ok(Vec::new());
    }

    let runtime_sources = model
        .branch_sources
        .iter()
        .enumerate()
        .map(|(index, source)| {
            Ok((
                index,
                compiled_branch_endpoint(model, &source.pos)?,
                compiled_branch_endpoint(model, &source.neg)?,
            ))
        })
        .collect::<JitResult<Vec<_>>>()?;

    mir.branch_unknowns
        .iter()
        .map(|unknown| {
            let pos = canonical_branch_endpoint(model, mir, unknown.pos_node)?;
            let neg = canonical_branch_endpoint(model, mir, unknown.neg_node)?;
            let Some(mapping) =
                runtime_sources
                    .iter()
                    .find_map(|(index, source_pos, source_neg)| {
                        if *source_pos == pos && *source_neg == neg {
                            Some(BranchUnknownRuntimeMapping {
                                runtime_index: *index,
                                inverted: false,
                            })
                        } else if *source_pos == neg && *source_neg == pos {
                            Some(BranchUnknownRuntimeMapping {
                                runtime_index: *index,
                                inverted: true,
                            })
                        } else {
                            None
                        }
                    })
            else {
                return Err(JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical branch unknown {} for {} has no compiled solver branch source",
                        unknown.id,
                        format_current_pair(pos, neg)
                    )
                    .into(),
                });
            };
            Ok(mapping)
        })
        .collect()
}

fn compiled_branch_endpoint(model: &CompiledModel, index: &StampIndex) -> JitResult<usize> {
    match index {
        StampIndex::Terminal(term) if *term < model.num_terminals => Ok(*term),
        StampIndex::Terminal(term) => Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "compiled branch source terminal {term} exceeds terminal count {}",
                model.num_terminals
            )
            .into(),
        }),
        StampIndex::Internal(internal) if *internal < model.internal_nodes => {
            Ok(model.num_terminals + *internal)
        }
        StampIndex::Internal(internal) => Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "compiled branch source internal node {internal} exceeds internal node count {}",
                model.internal_nodes
            )
            .into(),
        }),
        StampIndex::Ground => Ok(CURRENT_PAIR_GROUND),
        StampIndex::Branch(branch) => Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "compiled branch source endpoint unexpectedly references branch {branch}"
            )
            .into(),
        }),
    }
}

fn canonical_branch_endpoint(
    model: &CompiledModel,
    mir: &MirModel,
    node_id: Option<NodeId>,
) -> JitResult<usize> {
    let Some(node_id) = node_id else {
        return Ok(CURRENT_PAIR_GROUND);
    };
    let node_index = usize::from(node_id);
    let node = mir
        .nodes
        .get(node_index)
        .filter(|node| node.id == node_id)
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!("canonical branch endpoint node {node_id} is outside MIR node table")
                .into(),
        })?;

    if node.is_external {
        if node_index < model.num_terminals {
            return Ok(node_index);
        }
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical branch endpoint terminal {node_index} exceeds compiled terminal count {}",
                model.num_terminals
            )
            .into(),
        });
    }

    let external_count = mir.nodes.iter().filter(|node| node.is_external).count();
    let internal_index =
        node_index
            .checked_sub(external_count)
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical internal branch endpoint {} appears before external nodes",
                    node.name
                )
                .into(),
            })?;
    if internal_index < model.internal_nodes {
        return Ok(model.num_terminals + internal_index);
    }

    Err(JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!(
            "canonical branch endpoint internal node {internal_index} exceeds compiled internal node count {}",
            model.internal_nodes
        )
        .into(),
    })
}

fn lower_stamp_value_program(
    model: &CompiledModel,
    canonical_mir: Option<&MirModel>,
    stamp_index: usize,
    bytecode_program: &crate::codegen::BytecodeProgram,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    if let Some(mir) = canonical_mir {
        let equation_id = u32::try_from(stamp_index)
            .map(EquationId::new)
            .map_err(|_| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!("stamp index {stamp_index} exceeds canonical equation id range")
                    .into(),
            })?;
        let ddt_slots = canonical_ddt_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let idt_slots = canonical_idt_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let idtmod_slots = canonical_idtmod_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let transition_slots = canonical_transition_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let slew_slots = canonical_slew_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let absdelay_slots = canonical_absdelay_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let laplace_slots = canonical_laplace_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let zi_slots = canonical_zi_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let cross_slots = canonical_cross_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let above_slots = canonical_above_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let timer_slots = canonical_timer_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let limit_slots = canonical_limit_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        let table_lookup_slots = canonical_table_lookup_slots_for_equation(
            model.name.clone(),
            mir,
            equation_id,
            bytecode_program,
        )?;
        return NativeProgram::from_mir_equation(
            model.name.clone(),
            EntryKind::StampValue,
            mir,
            equation_id,
            limits
                .with_canonical_ddt_slots(&ddt_slots)
                .with_canonical_idt_slots(&idt_slots)
                .with_canonical_idtmod_slots(&idtmod_slots)
                .with_canonical_transition_slots(&transition_slots)
                .with_canonical_slew_slots(&slew_slots)
                .with_canonical_absdelay_slots(&absdelay_slots)
                .with_canonical_laplace_slots(&laplace_slots)
                .with_canonical_zi_slots(&zi_slots)
                .with_canonical_cross_slots(&cross_slots)
                .with_canonical_above_slots(&above_slots)
                .with_canonical_timer_slots(&timer_slots)
                .with_canonical_limit_slots(&limit_slots)
                .with_canonical_table_lookup_slots(&table_lookup_slots),
        );
    }

    NativeProgram::from_bytecode(
        model.name.clone(),
        EntryKind::StampValue,
        bytecode_program,
        limits,
    )
}

fn append_assignment_entry(model: &CompiledModel, image: &mut Vec<u8>) -> JitResult<CodeOffset> {
    let live_assignment_steps = live_native_assignment_steps(model);
    let assignments = live_assignment_steps
        .iter()
        .map(|step| lower_assignment_step(model, step))
        .collect::<JitResult<Vec<_>>>()?;

    let bytes = if assignments.is_empty() {
        vec![0xC3]
    } else {
        codegen::compile_assignment_pass_function(&assignments)?
    };
    let offset = align_image_for_entry(image);
    image.extend_from_slice(&bytes);
    Ok(offset)
}

fn live_native_assignment_steps(model: &CompiledModel) -> Vec<AssignmentStep> {
    let mut live = native_assignment_roots(model);
    loop {
        let mut changed = false;
        propagate_assignment_liveness(&model.assignment_steps, &mut live, &mut changed);
        if !changed {
            break;
        }
    }
    filter_live_assignment_steps(&model.assignment_steps, &live)
}

fn native_assignment_roots(model: &CompiledModel) -> Vec<bool> {
    let mut live = vec![false; model.num_variables];
    for index in 0..model.variable_names.len().min(live.len()) {
        // Named variables are externally observable through
        // VerilogADevice::variable(s), so native JIT must update them even
        // when no contribution reads them later in the evaluation pass.
        live[index] = true;
    }
    for stamp in &model.stamp_programs {
        if let Some(condition) = &stamp.static_condition {
            mark_program_variable_reads(condition, &mut live);
        }
        mark_program_variable_reads(&stamp.value_program, &mut live);
        for jacobian in &stamp.jacobian_programs {
            mark_program_variable_reads(&jacobian.program, &mut live);
        }
        for jacobian in &stamp.reactive_jacobians {
            mark_program_variable_reads(&jacobian.program, &mut live);
        }
    }
    for source in &model.noise_sources {
        mark_program_variable_reads(&source.psd_program, &mut live);
        if let Some(program) = &source.exponent_program {
            mark_program_variable_reads(program, &mut live);
        }
    }
    live
}

fn propagate_assignment_liveness(steps: &[AssignmentStep], live: &mut [bool], changed: &mut bool) {
    for step in steps.iter().rev() {
        match step {
            AssignmentStep::Assign(assignment) => {
                if assignment.var_index < live.len() && live[assignment.var_index] {
                    mark_program_variable_reads_changed(&assignment.program, live, changed);
                }
            }
            AssignmentStep::AssignIndexed {
                base,
                len,
                index,
                value,
                ..
            } => {
                if assignment_range_live(*base, *len, live) {
                    mark_program_variable_reads_changed(index, live, changed);
                    mark_program_variable_reads_changed(value, live, changed);
                }
            }
            AssignmentStep::Loop { condition, body } => {
                propagate_assignment_liveness(body, live, changed);
                if assignment_steps_write_live(body, live) {
                    mark_program_variable_reads_changed(condition, live, changed);
                }
            }
        }
    }
}

fn filter_live_assignment_steps(steps: &[AssignmentStep], live: &[bool]) -> Vec<AssignmentStep> {
    steps
        .iter()
        .filter_map(|step| match step {
            AssignmentStep::Assign(assignment) => (assignment.var_index < live.len()
                && live[assignment.var_index])
                .then(|| step.clone()),
            AssignmentStep::AssignIndexed { base, len, .. } => {
                assignment_range_live(*base, *len, live).then(|| step.clone())
            }
            AssignmentStep::Loop { condition, body } => {
                let body = filter_live_assignment_steps(body, live);
                (!body.is_empty()).then(|| AssignmentStep::Loop {
                    condition: condition.clone(),
                    body,
                })
            }
        })
        .collect()
}

fn assignment_steps_write_live(steps: &[AssignmentStep], live: &[bool]) -> bool {
    steps.iter().any(|step| match step {
        AssignmentStep::Assign(assignment) => {
            assignment.var_index < live.len() && live[assignment.var_index]
        }
        AssignmentStep::AssignIndexed { base, len, .. } => assignment_range_live(*base, *len, live),
        AssignmentStep::Loop { body, .. } => assignment_steps_write_live(body, live),
    })
}

fn assignment_range_live(base: usize, len: usize, live: &[bool]) -> bool {
    base.checked_add(len)
        .and_then(|end| live.get(base..end))
        .is_some_and(|range| range.iter().any(|slot| *slot))
}

fn mark_program_variable_reads(program: &BytecodeProgram, live: &mut [bool]) {
    let mut changed = false;
    mark_program_variable_reads_changed(program, live, &mut changed);
}

fn mark_program_variable_reads_changed(
    program: &BytecodeProgram,
    live: &mut [bool],
    changed: &mut bool,
) {
    for instruction in &program.instructions {
        match *instruction {
            Instruction::PushVariable(index) => mark_variable_live(index, live, changed),
            Instruction::PushVariableDyn { base, len, .. } => {
                if let Some(end) = base.checked_add(len) {
                    for index in base..end.min(live.len()) {
                        mark_variable_live(index, live, changed);
                    }
                }
            }
            _ => {}
        }
    }
}

fn mark_variable_live(index: usize, live: &mut [bool], changed: &mut bool) {
    if let Some(slot) = live.get_mut(index)
        && !*slot
    {
        *slot = true;
        *changed = true;
    }
}

fn lower_assignment_step(
    model: &CompiledModel,
    step: &AssignmentStep,
) -> JitResult<NativeAssignment> {
    let limits = NativeLoweringLimits::for_model(model);
    match step {
        AssignmentStep::Assign(assignment) => {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                &assignment.program,
                limits,
            )?;
            Ok(NativeAssignment::Direct {
                var_index: assignment.var_index,
                program,
            })
        }
        AssignmentStep::AssignIndexed {
            base,
            len,
            lower,
            index,
            value,
        } => {
            let index = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                index,
                limits,
            )?;
            let value = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                value,
                limits,
            )?;
            if let Some(var_index) = constant_indexed_assignment_slot(&index, *base, *len, *lower) {
                return Ok(NativeAssignment::Direct {
                    var_index,
                    program: value,
                });
            }
            Ok(NativeAssignment::Indexed {
                base: *base,
                len: *len,
                lower: *lower,
                index,
                value,
            })
        }
        AssignmentStep::Loop { condition, body } => {
            let condition = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Assignment,
                condition,
                limits,
            )?;
            let body = body
                .iter()
                .map(|step| lower_assignment_step(model, step))
                .collect::<JitResult<Vec<_>>>()?;
            Ok(NativeAssignment::Loop { condition, body })
        }
    }
}

fn constant_indexed_assignment_slot(
    index: &NativeProgram,
    base: usize,
    len: usize,
    lower: i64,
) -> Option<usize> {
    match index.ops() {
        [NativeOp::Const(raw_index)] => {
            constant_dynamic_variable_slot(*raw_index, base, len, lower)
        }
        _ => None,
    }
}

fn append_value_entry(image: &mut Vec<u8>, program: &NativeProgram) -> JitResult<CodeOffset> {
    let bytes = codegen::compile_value_function(program)?;
    let offset = align_image_for_entry(image);
    image.extend_from_slice(&bytes);
    Ok(offset)
}

fn align_image_for_entry(image: &mut Vec<u8>) -> CodeOffset {
    let padding = (ENTRY_ALIGNMENT - (image.len() % ENTRY_ALIGNMENT)) % ENTRY_ALIGNMENT;
    image.resize(image.len() + padding, X64_NOP);
    CodeOffset::new(image.len())
}

fn infer_current_terminal_pair(program: &StampProgram) -> Option<(usize, usize)> {
    let mut pos_endpoint = None;
    let mut neg_endpoint = None;

    for loc in &program.stamp_locations {
        let endpoint = match loc.row {
            StampIndex::Terminal(term) => term,
            StampIndex::Ground => CURRENT_PAIR_GROUND,
            _ => continue,
        };

        if loc.sign < 0.0 {
            if pos_endpoint.replace(endpoint).is_some() {
                return None;
            }
        } else if loc.sign > 0.0 && neg_endpoint.replace(endpoint).is_some() {
            return None;
        }
    }

    match (pos_endpoint, neg_endpoint) {
        (Some(pos), Some(neg)) if pos != neg => Some((pos, neg)),
        _ => None,
    }
}

fn infer_current_unified_pair(
    model: &CompiledModel,
    program: &StampProgram,
) -> Option<(usize, usize)> {
    let mut pos_endpoint = None;
    let mut neg_endpoint = None;

    for loc in &program.stamp_locations {
        let endpoint = stamp_row_unified_endpoint(model, &loc.row)?;

        if loc.sign < 0.0 {
            if pos_endpoint.replace(endpoint).is_some() {
                return None;
            }
        } else if loc.sign > 0.0 && neg_endpoint.replace(endpoint).is_some() {
            return None;
        }
    }

    match (pos_endpoint, neg_endpoint) {
        (Some(pos), Some(neg)) if pos != neg => Some((pos, neg)),
        _ => None,
    }
}

fn stamp_row_unified_endpoint(model: &CompiledModel, index: &StampIndex) -> Option<usize> {
    match index {
        StampIndex::Terminal(term) if *term < model.num_terminals => Some(*term),
        StampIndex::Internal(internal) if *internal < model.internal_nodes => {
            Some(model.num_terminals + *internal)
        }
        StampIndex::Ground => Some(CURRENT_PAIR_GROUND),
        _ => None,
    }
}

fn push_prior_current_probe_aliases(
    probes: &mut Vec<PriorCurrentProbe>,
    current_index: usize,
    pos: usize,
    neg: usize,
) {
    push_prior_current_probe_alias(
        probes,
        PriorCurrentProbe {
            pos,
            neg,
            current_index,
            inverted: false,
        },
    );
    if pos != neg {
        push_prior_current_probe_alias(
            probes,
            PriorCurrentProbe {
                pos: neg,
                neg: pos,
                current_index,
                inverted: true,
            },
        );
    }
}

fn push_prior_current_probe_alias(probes: &mut Vec<PriorCurrentProbe>, probe: PriorCurrentProbe) {
    if !probes.contains(&probe) {
        probes.push(probe);
    }
}

fn push_current_pair_indices(
    model: &CompiledModel,
    available_current_pairs: &mut Vec<usize>,
    terminal_count: usize,
    pos: usize,
    neg: usize,
) -> JitResult<()> {
    let forward = terminal_pair_current_index(pos, neg, terminal_count)
        .ok_or_else(|| current_pair_unavailable(model, pos, neg))?;
    if !available_current_pairs.contains(&forward) {
        available_current_pairs.push(forward);
    }

    let reverse = terminal_pair_current_index(neg, pos, terminal_count)
        .ok_or_else(|| current_pair_unavailable(model, neg, pos))?;
    if !available_current_pairs.contains(&reverse) {
        available_current_pairs.push(reverse);
    }

    Ok(())
}

fn current_pair_unavailable(model: &CompiledModel, pos: usize, neg: usize) -> JitError {
    JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!(
            "PushCurrent terminal pair {} cannot be represented",
            format_current_pair(pos, neg)
        )
        .into(),
    }
}

fn format_current_pair(pos: usize, neg: usize) -> String {
    format!(
        "{},{}",
        format_current_endpoint(pos),
        format_current_endpoint(neg)
    )
}

fn format_current_endpoint(endpoint: usize) -> String {
    if endpoint == CURRENT_PAIR_GROUND {
        "ground".to_string()
    } else {
        endpoint.to_string()
    }
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{
        NativeModel, compile_model_with_canonical_ir, lower_assignment_step,
        lower_static_condition_program, validate_compiled_entry_shape,
    };
    use crate::canonical_ir::{CanonicalIrArtifact, HirExprKind, OptModel};
    use crate::codegen::{
        AssignmentStep, BytecodeProgram, ColumnAxis, CompiledModel, Instruction, StampProgram,
    };
    use crate::device::VerilogADevice;
    use crate::native::expr::{
        EntryKind, NativeLoweringLimits, NativeOp, NativeProgram, PriorCurrentProbe,
    };
    use crate::native::model::{CodeOffset, NativeCurrentDependencies, NativeEntryOffsets};
    use crate::native::runtime::ExecutableMemory;
    use crate::native::x64::codegen::NativeAssignment;
    use crate::native::{EvalContext, clear_native_runtime_error, take_native_runtime_error};
    use crate::vm::{Vm, VmContext};
    use crate::{CompilerOptions, VerilogACompiler};
    use smol_str::SmolStr;
    use std::path::{Path, PathBuf};

    #[test]
    fn append_value_entry_aligns_nonzero_entry_offsets() {
        let program = NativeProgram::from_bytecode(
            "aligned-entry",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(12.0),
                    Instruction::PushConst(0.5),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0),
        )
        .expect("constant add lowers to native program");
        let mut image = vec![0xC3];

        let offset =
            super::append_value_entry(&mut image, &program).expect("append aligned value entry");

        assert_eq!(offset.as_usize(), super::ENTRY_ALIGNMENT);
        assert_eq!(offset.as_usize() % super::ENTRY_ALIGNMENT, 0);
        assert!(
            image[1..offset.as_usize()]
                .iter()
                .all(|byte| *byte == super::X64_NOP),
            "entry padding should be x64 NOPs"
        );

        let memory = ExecutableMemory::allocate(&image).expect("allocate aligned image");
        let entry = memory
            .ptr_at(offset.as_usize())
            .expect("aligned entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()), 12.5);
    }

    fn canonical_artifact_with_unsupported_root(
        compiler: &VerilogACompiler,
        source: &str,
    ) -> CanonicalIrArtifact {
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let metadata = artifact.metadata.clone();
        let mut hir = artifact.hir.clone();
        let mut mir = artifact.mir.clone();
        let root = usize::from(mir.equations[0].expression.id);
        let unsupported = HirExprKind::StringLiteral {
            value: "unsupported-native-expression".into(),
        };
        hir.expressions[root].kind = unsupported.clone();
        mir.expressions[root].kind = unsupported;
        hir.contributions[0].expression.kind = "string".into();
        mir.equations[0].expression.kind = "string".into();
        let opt = OptModel::from_mir(&mir).expect("synthetic canonical MIR still validates");
        CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
            .expect("synthetic canonical artifact has refreshed digests")
    }

    #[test]
    fn compile_model_with_canonical_ir_executes_mir_stamp_value() {
        let source = r#"
module native_canonical_res(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  analog begin
    I(p, n) <+ V(p, n) / r;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical MIR stamp value compiles to native x64");

        assert_eq!(native.native_stamp_count(), 1);
        let params = [2.0_f64];
        let voltages = [5.0_f64, 1.0_f64];
        let ctx = eval_context(&params, &voltages);
        assert_eq!(
            native.run_stamp_value(0, &ctx, std::ptr::null()),
            (voltages[0] - voltages[1]) / params[0]
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_stamp_branch() {
        let model_source = r#"
module native_canonical_shape_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let artifact_source = r#"
module native_canonical_shape_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(n, p) <+ V(n, p);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(model_source)
            .expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(artifact_source)
            .expect("compile mismatched canonical IR");

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-name reversed canonical branch must be rejected");
        let message = error.to_string();
        assert!(
            message.contains("canonical equation 0 branch"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_stamp_kind() {
        let model_source = r#"
module native_canonical_kind_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let artifact_source = r#"
module native_canonical_kind_guard(p, n);
  inout p, n;
  electrical p, n;
  analog V(p, n) <+ 1.0;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(model_source)
            .expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(artifact_source)
            .expect("compile wrong-kind canonical IR");

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-name wrong-kind canonical equation must be rejected");
        let message = error.to_string();
        assert!(
            message.contains("canonical equation 0 kind"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical kind mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_uses_mir_parameter_defaults() {
        let source = r#"
module native_canonical_param_default(p, n);
  inout p, n;
  electrical p, n;
  parameter real base = 2.0;
  parameter real derived = base * 3.0;
  analog I(p, n) <+ V(p, n) * derived;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        model.parameters[1].default_program = Some(BytecodeProgram {
            instructions: vec![Instruction::PushConst(99.0)],
        });

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical parameter default compiles to native x64");
        let params = [2.0_f64, 0.0];
        let ctx = eval_context(&params, &[]);
        let value = native
            .run_parameter_default(1, &ctx, std::ptr::null())
            .expect("derived parameter default has native entry");

        assert_eq!(value, 6.0);
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_parameter_defaults() {
        let model_source = r#"
module native_canonical_param_guard(p, n);
  inout p, n;
  electrical p, n;
  parameter real base = 2.0;
  parameter real derived = base * 3.0;
  analog I(p, n) <+ V(p, n) * derived;
endmodule
"#;
        let artifact_source = r#"
module native_canonical_param_guard(p, n);
  inout p, n;
  electrical p, n;
  parameter real base = 4.0;
  parameter real derived = base * 3.0;
  analog I(p, n) <+ V(p, n) * derived;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(model_source)
            .expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(artifact_source)
            .expect("compile mismatched canonical IR");

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-name mismatched parameter default must be rejected");
        let message = error.to_string();
        assert!(
            message.contains("canonical parameter 'base' default"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical parameter mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_static_condition_from_mir() {
        let source = r#"
module native_canonical_static_guard(p, n);
  inout p, n;
  electrical p, n;
  parameter real enabled = 1.0;
  analog begin
    if (enabled > 0.5) begin
      I(p, n) <+ V(p, n);
    end
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let stamp_index = model
            .stamp_programs
            .iter()
            .position(|stamp| stamp.static_condition.is_some())
            .expect("fixture must include a static condition");
        model.stamp_programs[stamp_index].static_condition = Some(BytecodeProgram {
            instructions: vec![Instruction::PushConst(0.0)],
        });

        let program = lower_static_condition_program(
            &model,
            Some(&artifact.mir),
            stamp_index,
            model.stamp_programs[stamp_index]
                .static_condition
                .as_ref()
                .expect("poisoned static condition bytecode exists"),
            NativeLoweringLimits::for_model(&model),
        )
        .expect("canonical static condition lowers to native ops");
        assert_eq!(program.ops(), &[NativeOp::LoadVariable(0)]);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical static condition compiles to native x64");
        let mut context = native_model_benchmark_context(&model, "canonical_static_guard");
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);
        native.run_assignments(&ctx, context.variables.as_mut_ptr());
        if let Some(error) = take_native_runtime_error() {
            panic!("native assignment failed before static condition: {error}");
        }
        let ctx = eval_context_from_vm_context(&mut context);
        let active = native
            .run_static_condition(stamp_index, &ctx, context.variables.as_ptr())
            .expect("static condition has native entry");

        assert_eq!(active.to_bits(), 1.0_f64.to_bits());
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_missing_static_condition_guard() {
        let model_source = r#"
module native_canonical_static_guard_shape(p, n);
  inout p, n;
  electrical p, n;
  parameter real enabled = 1.0;
  analog begin
    if (enabled > 0.5) begin
      I(p, n) <+ V(p, n);
    end
  end
endmodule
"#;
        let artifact_source = r#"
module native_canonical_static_guard_shape(p, n);
  inout p, n;
  electrical p, n;
  parameter real enabled = 1.0;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(model_source)
            .expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(artifact_source)
            .expect("compile mismatched canonical IR");
        assert!(
            model
                .stamp_programs
                .iter()
                .any(|stamp| stamp.static_condition.is_some()),
            "fixture must include a compiled static condition"
        );

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("missing canonical static guard must fail native compile");
        let message = error.to_string();
        assert!(
            message.contains("missing leading static-condition guard"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical static guard mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_noise_entries_from_mir() {
        let source = r#"
module native_canonical_noise_guard(p, n, ctrl);
  inout p, n, ctrl;
  electrical p, n, ctrl;
  parameter real scale = 2.0;
  analog begin
    I(p, n) <+ (scale + V(ctrl, n)) * white_noise(3.0, "thermal");
    I(p, n) <+ (scale - V(p, n)) * flicker_noise(4.0, 1.25, "flicker");
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert_eq!(model.noise_sources.len(), 2);
        assert!(
            model.noise_sources[1].exponent_program.is_some(),
            "fixture must include a flicker exponent"
        );

        model.noise_sources[0].psd_program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(99.0)],
        };
        model.noise_sources[1].psd_program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(88.0)],
        };
        model.noise_sources[1].exponent_program = Some(BytecodeProgram {
            instructions: vec![Instruction::PushConst(77.0)],
        });

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical noise entries compile to native x64");
        let mut context = native_model_benchmark_context(&model, "canonical_noise_guard");
        context.voltages[0] = 0.5;
        context.voltages[1] = 0.0;
        context.voltages[2] = 0.25;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);

        let white_psd = native.run_noise_psd(0, &ctx, context.variables.as_ptr());
        let flicker_psd = native.run_noise_psd(1, &ctx, context.variables.as_ptr());
        let flicker_exponent = native
            .run_noise_exponent(1, &ctx, context.variables.as_ptr())
            .expect("flicker source has native exponent entry");

        assert_finite_close(
            "canonical_noise_guard",
            "white noise psd",
            (2.0_f64 + 0.25).powi(2) * 3.0,
            white_psd,
        )
        .expect("white PSD must come from canonical MIR, not poisoned bytecode");
        assert_finite_close(
            "canonical_noise_guard",
            "flicker noise psd",
            (2.0_f64 - 0.5).powi(2) * 4.0,
            flicker_psd,
        )
        .expect("flicker PSD must come from canonical MIR, not poisoned bytecode");
        assert_eq!(flicker_exponent.to_bits(), 1.25_f64.to_bits());
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_noise_kind() {
        let model_source = r#"
module native_canonical_noise_kind_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ white_noise(3.0, "thermal");
endmodule
"#;
        let artifact_source = r#"
module native_canonical_noise_kind_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ flicker_noise(3.0, 1.0, "thermal");
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(model_source)
            .expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(artifact_source)
            .expect("compile mismatched canonical IR");

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("canonical/compiled noise kind drift must fail native compile");
        let message = error.to_string();
        assert!(
            message.contains("canonical noise source 0 kind"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical noise mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_maps_duplicate_potential_branch_unknowns() {
        let source = r#"
module native_canonical_duplicate_vsrc(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    V(p, n) <+ 1.0;
    V(p, n) <+ I(p, n) * 2.0;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert_eq!(
            model.branch_sources.len(),
            1,
            "compiled solver allocation should merge same-branch potential contributions"
        );
        assert_eq!(
            artifact.mir.branch_unknowns.len(),
            2,
            "canonical MIR keeps a dense branch unknown per potential equation"
        );

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("duplicate canonical potential branch unknowns map to runtime branch slot");

        let branch_unknowns = [3.0_f64];
        let mut ctx = eval_context(&[], &[0.0, 0.0]);
        ctx.branch_unknowns = branch_unknowns.as_ptr();
        assert_eq!(native.run_stamp_value(0, &ctx, std::ptr::null()), 1.0);
        assert_eq!(native.run_stamp_value(1, &ctx, std::ptr::null()), 6.0);
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_node_jacobians_from_mir() {
        let source = r#"
module native_canonical_node_jacobian(p, n);
  inout p, n;
  electrical p, n;
  parameter real g = 2.0;
  analog I(p, n) <+ g * V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model.stamp_programs[0]
                .jacobian_programs
                .iter()
                .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Node(0))),
            "fixture must include positive-node Jacobian"
        );
        assert!(
            model.stamp_programs[0]
                .jacobian_programs
                .iter()
                .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Node(1))),
            "fixture must include negative-node Jacobian"
        );
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical node Jacobians compile to native x64");
        let mut context = native_model_benchmark_context(&model, "canonical_node_jacobian");
        context.voltages[0] = 5.0;
        context.voltages[1] = 1.0;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);

        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            2.0,
            "canonical_node_jacobian d/dp",
        );
        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -2.0,
            "canonical_node_jacobian d/dn",
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_branch_jacobians_from_mir() {
        for (name, body, expected) in [
            ("forward", "V(p, n) <+ r * I(p, n);", 2.0_f64),
            ("reversed", "V(p, n) <+ r * I(n, p);", -2.0_f64),
        ] {
            let source = format!(
                r#"
module native_canonical_branch_jacobian_{name}(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  analog {body}
endmodule
"#
            );
            let compiler = VerilogACompiler::new(CompilerOptions::default());
            let mut model = compiler.compile(&source).expect("compile bytecode model");
            let artifact = compiler
                .compile_canonical_ir(&source)
                .expect("compile canonical IR");
            assert_eq!(
                model.branch_sources.len(),
                1,
                "fixture must allocate one runtime branch unknown"
            );
            assert!(
                model.stamp_programs[0]
                    .jacobian_programs
                    .iter()
                    .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Branch(0))),
                "fixture must include branch-current Jacobian"
            );
            poison_jacobian_bytecode(&mut model, 99.0);

            let native = compile_model_with_canonical_ir(&model, &artifact)
                .expect("canonical branch Jacobian compiles to native x64");
            let mut context = native_model_benchmark_context(&model, name);
            context.branch_current_values[0] = 3.0;
            resolve_native_parameter_defaults(&model, &native, &mut context);
            let ctx = eval_context_from_vm_context(&mut context);

            assert_jacobian_axis_value(
                &model,
                &native,
                &ctx,
                context.variables.as_ptr(),
                0,
                |axis| matches!(axis, ColumnAxis::Branch(0)),
                expected,
                format!("canonical_branch_jacobian_{name} d/dI0"),
            );
        }
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_branch_shadow_jacobian_from_mir() {
        let source = r#"
module native_canonical_branch_shadow_jacobian(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  real x;
  analog begin
    x = I(p, n);
    V(p, n) <+ r * x;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model
                .variable_names
                .iter()
                .any(|name| name.as_str() == "x@dI0"),
            "fixture must include branch-current derivative shadow"
        );
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical branch-shadow Jacobian compiles to native x64");
        let mut context = native_model_benchmark_context(&model, "canonical_branch_shadow");
        context.branch_current_values[0] = 3.0;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);
        native.run_assignments(&ctx, context.variables.as_mut_ptr());
        if let Some(error) = take_native_runtime_error() {
            panic!("native assignment failed before branch-shadow Jacobian: {error}");
        }
        let ctx = eval_context_from_vm_context(&mut context);

        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Branch(0)),
            2.0,
            "canonical_branch_shadow_jacobian d/dI0",
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_advanced_jacobians_from_mir() {
        let cases = [
            (
                "limit",
                "$limit(V(p, n), 0.5)",
                0.4_f64,
                1.0e-12_f64,
                1.0_f64,
            ),
            (
                "table",
                "$table_model(V(p, n), 0.0, 0.0, 1.0, 2.0, 2.0, 8.0)",
                1.5_f64,
                1.0e-12_f64,
                6.0_f64,
            ),
            ("idt", "idt(V(p, n), 0.0)", 0.4_f64, 0.25_f64, 0.25_f64),
            (
                "absdelay",
                "absdelay(V(p, n), 1.0e-9)",
                0.4_f64,
                1.0e-12_f64,
                1.0_f64,
            ),
            (
                "transition",
                "transition(V(p, n), 0.0, 1.0e-9, 1.0e-9)",
                0.4_f64,
                1.0e-12_f64,
                1.0_f64,
            ),
            (
                "slew",
                "slew(V(p, n), 1.0e9, 1.0e9)",
                0.4_f64,
                1.0e-12_f64,
                1.0_f64,
            ),
            (
                "laplace",
                "laplace_nd(V(p, n), {2.0}, {1.0})",
                0.4_f64,
                1.0e-12_f64,
                2.0_f64,
            ),
            (
                "zi",
                "zi_nd(V(p, n), {0.25}, {1.0, -0.75}, 1.0e-6)",
                0.4_f64,
                1.0e-12_f64,
                1.0_f64,
            ),
        ];

        for (name, expr, voltage, timestep, expected) in cases {
            let source = format!(
                r#"
module native_canonical_advanced_jacobian_{name}(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ {expr};
endmodule
"#
            );
            let compiler = VerilogACompiler::new(CompilerOptions::default());
            let mut model = compiler.compile(&source).expect("compile bytecode model");
            let artifact = compiler
                .compile_canonical_ir(&source)
                .expect("compile canonical IR");
            assert!(
                model.stamp_programs[0]
                    .jacobian_programs
                    .iter()
                    .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Node(0))),
                "{name}: fixture must include positive-node Jacobian"
            );
            poison_jacobian_bytecode(&mut model, 99.0);

            let native =
                compile_model_with_canonical_ir(&model, &artifact).unwrap_or_else(|error| {
                    panic!("{name}: canonical Jacobian compile failed: {error}")
                });
            let mut context = native_model_benchmark_context(&model, name);
            context.voltages[0] = voltage;
            context.voltages[1] = 0.0;
            context.timestep = timestep;
            resolve_native_parameter_defaults(&model, &native, &mut context);
            let ctx = eval_context_from_vm_context(&mut context);

            assert_jacobian_axis_value(
                &model,
                &native,
                &ctx,
                context.variables.as_ptr(),
                0,
                |axis| matches!(axis, ColumnAxis::Node(0)),
                expected,
                format!("canonical_advanced_jacobian_{name} d/dp"),
            );
        }
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_ddx_jacobians_from_mir() {
        let source = r#"
module native_canonical_ddx_jacobian(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ ddx(V(p, n) * V(p, n), V(p, n));
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(!model.stamp_programs[0].jacobian_programs.is_empty());
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical ddx Jacobians compile to native x64");
        let mut context = native_model_benchmark_context(&model, "canonical_ddx_jacobian");
        context.voltages[0] = 3.0;
        context.voltages[1] = 0.0;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);

        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            2.0,
            "canonical_ddx_jacobian d/dp",
        );
        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -2.0,
            "canonical_ddx_jacobian d/dn",
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_math_ddx_jacobians_from_mir() {
        let source = r#"
module native_canonical_math_ddx_jacobian(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ ddx(
      sqrt(V(p, n) + 4.0)
    + exp(0.1 * V(p, n))
    + log(V(p, n) + 3.0)
    + log10(V(p, n) + 3.0)
    + sin(0.2 * V(p, n))
    + cos(0.15 * V(p, n))
    + tan(0.1 * V(p, n))
    + sinh(0.2 * V(p, n))
    + cosh(0.15 * V(p, n))
    + tanh(0.3 * V(p, n))
    + asinh(0.4 * V(p, n))
    + acosh(V(p, n) + 2.0)
    + atanh(0.1 * V(p, n))
    + asin(0.07 * V(p, n))
    + acos(0.05 * V(p, n))
    + atan(0.2 * V(p, n))
    + atan2(V(p, n), V(p, n) + 1.0)
    + hypot(2.0 * V(p, n), V(p, n) + 3.0)
    + pow(V(p, n) + 2.0, 2.5),
    V(p, n));
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(!model.stamp_programs[0].jacobian_programs.is_empty());
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical math ddx Jacobians compile to native x64");
        let x = 0.4_f64;
        let expected = central_second_derivative(
            |x| {
                (x + 4.0).sqrt()
                    + (0.1 * x).exp()
                    + (x + 3.0).ln()
                    + (x + 3.0).log10()
                    + (0.2 * x).sin()
                    + (0.15 * x).cos()
                    + (0.1 * x).tan()
                    + (0.2 * x).sinh()
                    + (0.15 * x).cosh()
                    + (0.3 * x).tanh()
                    + (0.4 * x).asinh()
                    + (x + 2.0).acosh()
                    + (0.1 * x).atanh()
                    + (0.07 * x).asin()
                    + (0.05 * x).acos()
                    + (0.2 * x).atan()
                    + x.atan2(x + 1.0)
                    + (2.0 * x).hypot(x + 3.0)
                    + (x + 2.0).powf(2.5)
            },
            x,
        );
        let mut context = native_model_benchmark_context(&model, "canonical_math_ddx_jacobian");
        context.voltages[0] = x;
        context.voltages[1] = 0.0;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);

        assert_jacobian_axis_approx(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            expected,
            1.0e-5,
            "canonical_math_ddx_jacobian d/dp",
        );
        assert_jacobian_axis_approx(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -expected,
            1.0e-5,
            "canonical_math_ddx_jacobian d/dn",
        );
    }

    #[test]
    fn native_x64_generated_model_plan_publishes_dense_entrypoint_surface() {
        let source = r#"
`include "disciplines.vams"
module native_generated_plan_guard(p, n, ctrl);
  inout p, n, ctrl;
  electrical p, n, ctrl;
  parameter real base = 1.5;
  parameter real gain = base * 2.0;
  parameter real enable_a = 1.0;
  parameter real enable_b = 1.0;
  real vp;
  real vc;
  real g0;
  real g1;
  real g2;
  real g3;
  real accum;
  analog begin
    vp = V(p, n);
    vc = V(ctrl, n);
    g0 = gain + vc * 0.1;
    g1 = g0 * g0 + 0.25;
    g2 = sqrt(g1) + exp(0.01 * vp);
    g3 = sin(vc) - cos(vp);
    accum = g0 + g1 + g2 + g3;
    if (enable_a > 0.5) begin
      I(p, n) <+ g0 * vp;
      I(p, n) <+ g1 * vp;
      I(ctrl, n) <+ 0.5 * vc;
    end
    if (enable_b > 0.5) begin
      I(p, n) <+ accum * 0.125;
    end
    I(p, n) <+ g2 * vp + g3;
    I(p, n) <+ ddt(1.0e-12 * vp);
    I(p, n) <+ white_noise(1.0e-18 * (1.0 + abs(vp)), "thermal");
    I(p, n) <+ flicker_noise(2.0e-18 * (1.0 + abs(vc)), 1.0, "flicker");
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile generated model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile generated canonical IR");

        let expected_parameter_defaults = model
            .parameters
            .iter()
            .filter(|parameter| parameter.default_program.is_some())
            .count();
        let expected_static_conditions = model
            .stamp_programs
            .iter()
            .filter(|stamp| stamp.static_condition.is_some())
            .count();
        let expected_jacobians = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum::<usize>();
        let expected_reactive_jacobians = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.reactive_jacobians.len())
            .sum::<usize>();
        let expected_noise_entries = model.noise_sources.len()
            + model
                .noise_sources
                .iter()
                .filter(|source| source.exponent_program.is_some())
                .count();

        assert!(
            count_assignment_steps(&model.assignment_steps) >= 7,
            "fixture must exercise a dense assignment pass"
        );
        assert!(
            model.stamp_programs.len() >= 6,
            "fixture must publish multiple stamp entry points"
        );
        assert!(
            expected_parameter_defaults >= 1,
            "fixture must include dependent parameter defaults"
        );
        assert!(
            expected_static_conditions >= 2,
            "fixture must include parameter-static contribution guards"
        );
        assert!(
            expected_jacobians >= model.stamp_programs.len(),
            "fixture must publish Jacobian coverage for every stamp"
        );
        assert!(
            expected_reactive_jacobians >= 1,
            "fixture must include reactive Jacobian coverage"
        );
        assert!(
            expected_noise_entries >= 3,
            "fixture must include white-noise PSD plus flicker PSD/exponent entries"
        );

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("generated model compiles to native x64 without fallback");
        let stats = native.plan_stats();
        assert_eq!(stats.assignment_entry_points, 1);
        assert_eq!(
            stats.parameter_default_entry_points,
            expected_parameter_defaults
        );
        assert_eq!(
            stats.static_condition_entry_points,
            expected_static_conditions
        );
        assert_eq!(stats.stamp_value_entry_points, model.stamp_programs.len());
        assert_eq!(stats.jacobian_entry_points, expected_jacobians);
        assert_eq!(
            stats.reactive_jacobian_entry_points,
            expected_reactive_jacobians
        );
        assert_eq!(stats.noise_source_entry_points, expected_noise_entries);
        assert_eq!(native.native_stamp_count(), model.stamp_programs.len());

        let mut context = native_model_benchmark_context(&model, "generated_plan_guard");
        context.voltages[0] = 0.8;
        context.voltages[1] = 0.0;
        context.voltages[2] = 0.2;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let oracle_stats =
            assert_native_matches_bytecode_finite_entries(&model, &native, context, "generated")
                .expect("generated-model native values match bytecode oracle");
        assert!(
            oracle_stats.variables >= 7
                && oracle_stats.stamps >= 6
                && oracle_stats.jacobians >= expected_jacobians
                && oracle_stats.reactive_jacobians >= expected_reactive_jacobians,
            "oracle stats did not cover the dense surface: variables={} stamps={} jacobians={} reactive_jacobians={}",
            oracle_stats.variables,
            oracle_stats.stamps,
            oracle_stats.jacobians,
            oracle_stats.reactive_jacobians
        );
    }

    #[test]
    fn native_x64_compiled_entry_shape_accepts_compiled_model_surface() {
        let model = compile_shape_validator_model();
        let (entries, dependencies) = expected_native_entry_shape(&model);
        validate_compiled_entry_shape(&model, &entries, &dependencies)
            .expect("compiled-model entry shape validates before publication");
    }

    #[test]
    fn native_x64_compiled_entry_shape_rejects_backend_table_drift() {
        let model = compile_shape_validator_model();
        let (mut entries, dependencies) = expected_native_entry_shape(&model);
        let stamp_index = entries
            .jacobians
            .iter()
            .position(|entries| !entries.is_empty())
            .expect("shape fixture has jacobian entries");
        entries.jacobians[stamp_index].pop();

        let error = validate_compiled_entry_shape(&model, &entries, &dependencies)
            .expect_err("missing jacobian entry must fail before executable memory publication");
        let message = error.to_string();
        assert!(
            message.contains(&format!("jacobian {stamp_index}")),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "shape validation must keep the hard-JIT failure contract: {message}"
        );

        let (entries, mut dependencies) = expected_native_entry_shape(&model);
        dependencies.reactive_jacobians.pop();
        let error = validate_compiled_entry_shape(&model, &entries, &dependencies)
            .expect_err("missing dependency table must fail before publication");
        let message = error.to_string();
        assert!(
            message.contains("reactive-jacobian current dependency stamp"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "dependency validation must keep the hard-JIT failure contract: {message}"
        );
    }

    #[test]
    #[ignore = "release-only source-level native x64 throughput probe; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_model_microbench_reports_entrypoint_throughput() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 model microbench is release-only; rerun with --release"
        );

        let source = r#"
`include "disciplines.vams"
module native_model_microbench(p, n);
  inout p, n;
  electrical p, n;
  real g;
  analog begin
    g = 0.25;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#;
        let (model, native) = compile_native_microbench_model(source);
        assert!(
            !model.assignment_steps.is_empty(),
            "fixture must exercise the native assignment pass"
        );
        assert_eq!(native.native_stamp_count(), 1);
        assert!(
            !model.stamp_programs[0].jacobian_programs.is_empty(),
            "fixture must exercise native jacobian entrypoints"
        );

        let params = [];
        let voltages = [8.0_f64, 3.0];
        let ctx = eval_context(&params, &voltages);
        let mut vars = vec![0.0_f64; native.num_variables.max(1)];
        native.run_assignments(&ctx, vars.as_mut_ptr());
        assert_near(vars[0], 0.25, "assignment output");
        assert_near(
            native.run_stamp_value(0, &ctx, vars.as_ptr()),
            1.25,
            "stamp value",
        );
        let expected_jacobian = native.run_jacobian(0, 0, &ctx, vars.as_ptr());
        assert!(
            (expected_jacobian.abs() - 0.25).abs() <= 1.0e-12,
            "first jacobian entry should be +/-0.25, got {expected_jacobian}"
        );

        let iterations = native_model_microbench_iterations();
        let samples = native_model_microbench_samples();
        eprintln!("native-x64-model-microbench iterations={iterations} samples={samples}");

        run_native_model_entry_microbench(
            "assignment_fed",
            "assignments",
            iterations,
            samples,
            0.25,
            || {
                native.run_assignments(
                    std::hint::black_box(&ctx),
                    std::hint::black_box(vars.as_mut_ptr()),
                );
                std::hint::black_box(vars[0])
            },
        );
        run_native_model_entry_microbench(
            "assignment_fed",
            "stamp_value",
            iterations,
            samples,
            1.25,
            || {
                native.run_stamp_value(
                    0,
                    std::hint::black_box(&ctx),
                    std::hint::black_box(vars.as_ptr()),
                )
            },
        );
        run_native_model_entry_microbench(
            "assignment_fed",
            "jacobian0",
            iterations,
            samples,
            expected_jacobian,
            || {
                native.run_jacobian(
                    0,
                    0,
                    std::hint::black_box(&ctx),
                    std::hint::black_box(vars.as_ptr()),
                )
            },
        );
    }

    #[test]
    #[ignore = "release-only shipped-model native x64 throughput probe; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_shipped_model_microbench_reports_sweep_throughput() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 shipped-model microbench is release-only; rerun with --release"
        );

        let cases = [
            (
                "juncap200",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "juncap200.va"]),
                None,
            ),
            (
                "r3_cmc",
                shipped_cmc_model_path(&["r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"]),
                None,
            ),
            (
                "diode_cmc",
                shipped_cmc_model_path(&["diode_cmc_3.0_20250714", "vacode", "diode_cmc.va"]),
                Some("DIODE_CMC"),
            ),
            (
                "vbic13_4t",
                shipped_veriloga_model_path(&["vbic_1.3", "vacode", "vbic_1p3.va"]),
                Some("vbic13_4t"),
            ),
            (
                "bsimbulk",
                shipped_cmc_model_path(&["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"]),
                Some("bsimbulk"),
            ),
            (
                "bsimcmg",
                shipped_cmc_model_path(&["BSIM-CMG_112.1.0_04282026", "code", "bsimcmg.va"]),
                Some("bsimcmg_va"),
            ),
            (
                "psp104",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104.va"]),
                Some("PSP104VA"),
            ),
            (
                "psp104_nqs",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104_nqs.va"]),
                Some("PSPNQS104VA"),
            ),
            (
                "hicuml0",
                shipped_cmc_model_path(&["hicumL0_v2p1p0_files", "hicumL0_v2p1p0.va"]),
                Some("hicumL0va"),
            ),
            (
                "hicuml2",
                shipped_cmc_model_path(&["hicumL2_v320_files", "hicumL2_v320.va"]),
                Some("hicumL2va"),
            ),
            (
                "bsimimg",
                shipped_cmc_model_path(&["BSIM-IMG_103.0.0_20200102", "code", "bsimimg.va"]),
                Some("bsimimg"),
            ),
            (
                "bsimsoi461",
                shipped_veriloga_model_path(&["bsimsoi_4.6.1", "vacode", "bsimsoi.va"]),
                Some("bsimsoi_va"),
            ),
            (
                "bsimsoi47",
                shipped_cmc_model_path(&["BSIM-SOI_4.7.0_05192025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "bsimsoi100",
                shipped_cmc_model_path(&["BSIM_SOI_100.1.1_09152025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "l_utsoi102",
                shipped_cmc_model_path(&[
                    "L_UTSOI_102.9.0_code_package",
                    "vacode",
                    "L_UTSOI_102.va",
                ]),
                Some("l_utsoi"),
            ),
            (
                "hisimhv",
                shipped_cmc_model_path(&[
                    "HiSIM_HV_2.5.1_Release_20230209",
                    "HiSIM_HV_2.5.1_VA-Code",
                    "hisimhv_va",
                    "hisimhv.va",
                ]),
                Some("hisimhv_va"),
            ),
            (
                "hisimsoi",
                shipped_cmc_model_path(&[
                    "HiSIM_SOI_1.5.0_Release_20211008",
                    "HiSIM_SOI_1.5.0_VA-Code",
                    "hisimsoi_va",
                    "hisimsoi.va",
                ]),
                Some("hisimsoi_va"),
            ),
            (
                "asmhemt",
                shipped_cmc_model_path(&["ASM-HEMT101.6.0_05132026", "vacode", "asmhemt.va"]),
                Some("asmhemt"),
            ),
        ];
        let iterations = shipped_model_microbench_iterations();
        let samples = shipped_model_microbench_samples();
        eprintln!("native-x64-shipped-microbench iterations={iterations} samples={samples}");

        for (name, path, module) in cases {
            run_shipped_model_microbench_case(name, &path, module, iterations, samples);
        }
    }

    #[test]
    #[ignore = "release-only shipped-model native x64 finite oracle; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_shipped_model_finite_entries_match_bytecode_reference() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 shipped-model finite oracle is release-only; rerun with --release"
        );

        let cases = [
            (
                "juncap200",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "juncap200.va"]),
                None,
            ),
            (
                "bsimcmg",
                shipped_cmc_model_path(&["BSIM-CMG_112.1.0_04282026", "code", "bsimcmg.va"]),
                Some("bsimcmg_va"),
            ),
            (
                "r3_cmc",
                shipped_cmc_model_path(&["r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"]),
                None,
            ),
            (
                "diode_cmc",
                shipped_cmc_model_path(&["diode_cmc_3.0_20250714", "vacode", "diode_cmc.va"]),
                Some("DIODE_CMC"),
            ),
            (
                "vbic13_4t",
                shipped_veriloga_model_path(&["vbic_1.3", "vacode", "vbic_1p3.va"]),
                Some("vbic13_4t"),
            ),
            (
                "bsimbulk",
                shipped_cmc_model_path(&["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"]),
                Some("bsimbulk"),
            ),
            (
                "psp104",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104.va"]),
                Some("PSP104VA"),
            ),
            (
                "bsimimg",
                shipped_cmc_model_path(&["BSIM-IMG_103.0.0_20200102", "code", "bsimimg.va"]),
                Some("bsimimg"),
            ),
            (
                "psp104_nqs",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104_nqs.va"]),
                Some("PSPNQS104VA"),
            ),
            (
                "hicuml0",
                shipped_cmc_model_path(&["hicumL0_v2p1p0_files", "hicumL0_v2p1p0.va"]),
                Some("hicumL0va"),
            ),
            (
                "hicuml2",
                shipped_cmc_model_path(&["hicumL2_v320_files", "hicumL2_v320.va"]),
                Some("hicumL2va"),
            ),
            (
                "bsimsoi47",
                shipped_cmc_model_path(&["BSIM-SOI_4.7.0_05192025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "bsimsoi100",
                shipped_cmc_model_path(&["BSIM_SOI_100.1.1_09152025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "hisimsoi",
                shipped_cmc_model_path(&[
                    "HiSIM_SOI_1.5.0_Release_20211008",
                    "HiSIM_SOI_1.5.0_VA-Code",
                    "hisimsoi_va",
                    "hisimsoi.va",
                ]),
                Some("hisimsoi_va"),
            ),
            (
                "asmhemt",
                shipped_cmc_model_path(&["ASM-HEMT101.6.0_05132026", "vacode", "asmhemt.va"]),
                Some("asmhemt"),
            ),
        ];

        for (name, path, module) in cases {
            assert_shipped_model_finite_entries_match_bytecode(name, &path, module);
        }
    }

    #[test]
    #[ignore = "release-only shipped-model VerilogADevice native x64 probe; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_shipped_model_devices_run_without_interpreter_fallback() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 shipped-model device probe is release-only; rerun with --release"
        );

        let cases = [
            (
                "juncap200",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "juncap200.va"]),
                None,
            ),
            (
                "bsimcmg",
                shipped_cmc_model_path(&["BSIM-CMG_112.1.0_04282026", "code", "bsimcmg.va"]),
                Some("bsimcmg_va"),
            ),
            (
                "r3_cmc",
                shipped_cmc_model_path(&["r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"]),
                None,
            ),
            (
                "diode_cmc",
                shipped_cmc_model_path(&["diode_cmc_3.0_20250714", "vacode", "diode_cmc.va"]),
                Some("DIODE_CMC"),
            ),
            (
                "vbic13_4t",
                shipped_veriloga_model_path(&["vbic_1.3", "vacode", "vbic_1p3.va"]),
                Some("vbic13_4t"),
            ),
            (
                "bsimbulk",
                shipped_cmc_model_path(&["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"]),
                Some("bsimbulk"),
            ),
            (
                "psp104",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104.va"]),
                Some("PSP104VA"),
            ),
            (
                "bsimimg",
                shipped_cmc_model_path(&["BSIM-IMG_103.0.0_20200102", "code", "bsimimg.va"]),
                Some("bsimimg"),
            ),
            (
                "psp104_nqs",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104_nqs.va"]),
                Some("PSPNQS104VA"),
            ),
            (
                "hicuml0",
                shipped_cmc_model_path(&["hicumL0_v2p1p0_files", "hicumL0_v2p1p0.va"]),
                Some("hicumL0va"),
            ),
            (
                "hicuml2",
                shipped_cmc_model_path(&["hicumL2_v320_files", "hicumL2_v320.va"]),
                Some("hicumL2va"),
            ),
            (
                "bsimsoi47",
                shipped_cmc_model_path(&["BSIM-SOI_4.7.0_05192025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "bsimsoi100",
                shipped_cmc_model_path(&["BSIM_SOI_100.1.1_09152025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "hisimsoi",
                shipped_cmc_model_path(&[
                    "HiSIM_SOI_1.5.0_Release_20211008",
                    "HiSIM_SOI_1.5.0_VA-Code",
                    "hisimsoi_va",
                    "hisimsoi.va",
                ]),
                Some("hisimsoi_va"),
            ),
            (
                "asmhemt",
                shipped_cmc_model_path(&["ASM-HEMT101.6.0_05132026", "vacode", "asmhemt.va"]),
                Some("asmhemt"),
            ),
        ];

        for (name, path, module) in cases {
            run_shipped_model_device_probe(name, &path, module);
        }
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_unsupported_mir_stamp() {
        let source = r#"
`include "disciplines.vams"
module native_canonical_unsupported(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = canonical_artifact_with_unsupported_root(&compiler, source);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("unsupported canonical stamp must not fall back to bytecode");

        assert!(
            error.to_string().contains("expression kind string"),
            "{error}"
        );
    }

    #[test]
    fn lower_assignment_step_folds_constant_indexed_write_to_direct_assignment() {
        let model = compiled_model_with_variables(4);
        let step = indexed_assignment_step(
            1,
            3,
            1,
            vec![Instruction::PushConst(2.49)],
            vec![Instruction::PushConst(11.0)],
        );

        let assignment = lower_assignment_step(&model, &step).expect("lower indexed assignment");

        match assignment {
            NativeAssignment::Direct { var_index, program } => {
                assert_eq!(var_index, 2);
                assert_eq!(program.ops(), &[NativeOp::Const(11.0)]);
            }
            other => panic!("expected direct assignment, got {other:?}"),
        }
    }

    #[test]
    fn lower_assignment_step_preserves_unsafe_indexed_writes_on_helper_path() {
        let cases = [
            ("dynamic", vec![Instruction::PushVariable(0)], 0),
            ("nan", vec![Instruction::PushConst(f64::NAN)], 0),
            ("infinity", vec![Instruction::PushConst(f64::INFINITY)], 0),
            (
                "huge finite",
                vec![Instruction::PushConst(1.0e300)],
                i64::MAX,
            ),
            ("out of range", vec![Instruction::PushConst(2.0)], 0),
        ];

        for (name, index, lower) in cases {
            let model = compiled_model_with_variables(1);
            let step = indexed_assignment_step(
                0,
                1,
                lower,
                index.clone(),
                vec![Instruction::PushConst(11.0)],
            );

            let assignment =
                lower_assignment_step(&model, &step).expect("lower indexed assignment");

            match assignment {
                NativeAssignment::Indexed {
                    base,
                    len,
                    lower: actual_lower,
                    index: index_program,
                    value,
                } => {
                    assert_eq!(base, 0, "{name}");
                    assert_eq!(len, 1, "{name}");
                    assert_eq!(actual_lower, lower, "{name}");
                    assert_eq!(value.ops(), &[NativeOp::Const(11.0)], "{name}");
                    assert!(
                        !index_program.ops().is_empty(),
                        "{name}: index program must remain on helper path"
                    );
                }
                other => panic!("{name}: expected indexed helper path, got {other:?}"),
            }
        }
    }

    fn compile_shape_validator_model() -> CompiledModel {
        let source = r#"
`include "disciplines.vams"
module native_shape_validator(p, n);
  inout p, n;
  electrical p, n;
  parameter real base = 1.0;
  parameter real gain = base * 2.0;
  real v;
  analog begin
    v = V(p, n);
    if (gain > 0.0) begin
      I(p, n) <+ gain * v;
    end
    I(p, n) <+ ddt(1.0e-12 * v);
    I(p, n) <+ flicker_noise(1.0e-18 * (1.0 + abs(v)), 1.0, "flicker");
  end
endmodule
"#;
        VerilogACompiler::new(CompilerOptions::default())
            .compile(source)
            .expect("compile native shape validator model")
    }

    fn expected_native_entry_shape(
        model: &CompiledModel,
    ) -> (NativeEntryOffsets, NativeCurrentDependencies) {
        let offset = CodeOffset::new(0);
        let entries = NativeEntryOffsets {
            assignment: offset,
            parameter_defaults: model
                .parameters
                .iter()
                .map(|parameter| parameter.default_program.as_ref().map(|_| offset))
                .collect(),
            static_conditions: model
                .stamp_programs
                .iter()
                .map(|stamp| stamp.static_condition.as_ref().map(|_| offset))
                .collect(),
            stamp_values: vec![offset; model.stamp_programs.len()],
            jacobians: model
                .stamp_programs
                .iter()
                .map(|stamp| vec![offset; stamp.jacobian_programs.len()])
                .collect(),
            reactive_jacobians: model
                .stamp_programs
                .iter()
                .map(|stamp| vec![offset; stamp.reactive_jacobians.len()])
                .collect(),
            noise_psd: vec![offset; model.noise_sources.len()],
            noise_exponents: model
                .noise_sources
                .iter()
                .map(|source| source.exponent_program.as_ref().map(|_| offset))
                .collect(),
        };
        let dependencies = NativeCurrentDependencies {
            stamp_values: vec![Vec::new(); entries.stamp_values.len()],
            stamp_value_prior_currents: vec![Vec::new(); entries.stamp_values.len()],
            jacobians: empty_nested_dependencies(&entries.jacobians),
            jacobian_prior_currents: empty_nested_dependencies(&entries.jacobians),
            reactive_jacobians: empty_nested_dependencies(&entries.reactive_jacobians),
            reactive_jacobian_prior_currents: empty_nested_dependencies(
                &entries.reactive_jacobians,
            ),
            noise_psd: vec![Vec::new(); entries.noise_psd.len()],
            noise_psd_prior_currents: vec![Vec::new(); entries.noise_psd.len()],
            noise_exponents: vec![Vec::new(); entries.noise_exponents.len()],
            noise_exponent_prior_currents: vec![Vec::new(); entries.noise_exponents.len()],
        };
        (entries, dependencies)
    }

    fn empty_nested_dependencies(entries: &[Vec<CodeOffset>]) -> Vec<Vec<Vec<usize>>> {
        entries
            .iter()
            .map(|stamp_entries| vec![Vec::new(); stamp_entries.len()])
            .collect()
    }

    fn compile_native_microbench_model(source: &str) -> (CompiledModel, NativeModel) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("source-level canonical model compiles to native x64");
        (model, native)
    }

    fn run_shipped_model_microbench_case(
        name: &str,
        path: &Path,
        module: Option<&str>,
        iterations: usize,
        samples: usize,
    ) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let compile_start = std::time::Instant::now();
        let runtime = compiler
            .compile_file_runtime_with_metadata(path, module)
            .unwrap_or_else(|error| {
                panic!(
                    "compile shipped Verilog-A model {name} at {}: {error}",
                    path.display()
                )
            });
        let compile_elapsed = compile_start.elapsed();
        let native_start = std::time::Instant::now();
        let native = compile_model_with_canonical_ir(&runtime.model, &runtime.canonical_ir)
            .unwrap_or_else(|error| panic!("native x64 compile shipped model {name}: {error}"));
        let native_compile_elapsed = native_start.elapsed();
        let mut context = native_model_benchmark_context(&runtime.model, name);
        resolve_native_parameter_defaults(&runtime.model, &native, &mut context);
        let nonfinite_reference =
            collect_bytecode_nonfinite_reference(&runtime.model, context.clone())
                .unwrap_or_else(|error| panic!("{name}: bytecode non-finite reference: {error}"));
        if !nonfinite_reference.is_empty() {
            eprintln!(
                "native-x64-shipped-microbench model={name} bytecode_nonfinite_reference stamps={} jacobians={} reactive_jacobians={}",
                nonfinite_reference.stamps.len(),
                nonfinite_reference.jacobians.len(),
                nonfinite_reference.reactive_jacobians.len(),
            );
        }
        let mut sanity_checksum = run_native_model_sweep_once(
            &runtime.model,
            &native,
            &mut context,
            name,
            &nonfinite_reference,
        );
        assert!(
            sanity_checksum.is_finite(),
            "{name}: shipped-model sanity sweep checksum must stay finite"
        );

        let warmup_iterations = (iterations / 10).max(1);
        sanity_checksum += run_native_model_sweep_sample(
            &runtime.model,
            &native,
            &mut context,
            name,
            warmup_iterations,
            &nonfinite_reference,
        );
        assert!(
            sanity_checksum.is_finite(),
            "{name}: shipped-model warmup checksum must stay finite"
        );

        let mut sample_ns_per_sweep = Vec::with_capacity(samples);
        let mut checksum = 0.0_f64;
        for _ in 0..samples {
            let start = std::time::Instant::now();
            checksum += run_native_model_sweep_sample(
                &runtime.model,
                &native,
                &mut context,
                name,
                iterations,
                &nonfinite_reference,
            );
            let elapsed = start.elapsed();
            sample_ns_per_sweep.push(elapsed.as_nanos() as f64 / iterations as f64);
        }
        sample_ns_per_sweep.sort_by(|left, right| left.total_cmp(right));
        let min_ns_per_sweep = sample_ns_per_sweep[0];
        let median_ns_per_sweep = sample_ns_per_sweep[sample_ns_per_sweep.len() / 2];
        let checksum = std::hint::black_box(checksum);
        assert!(
            checksum.is_finite(),
            "{name}: shipped-model benchmark checksum must stay finite"
        );
        let stats = native.plan_stats();
        eprintln!(
            "native-x64-shipped-microbench model={name} compile_ms={:.3} native_compile_ms={:.3} dependencies={} params={} vars={} assignments={} stamps={} jacobians={} reactive_jacobians={} min_ns_per_sweep={min_ns_per_sweep:.3} median_ns_per_sweep={median_ns_per_sweep:.3} checksum={checksum:.17e}",
            compile_elapsed.as_secs_f64() * 1000.0,
            native_compile_elapsed.as_secs_f64() * 1000.0,
            runtime.dependencies.len(),
            runtime.model.parameters.len(),
            runtime.model.num_variables,
            count_assignment_steps(&runtime.model.assignment_steps),
            stats.stamp_value_entry_points,
            stats.jacobian_entry_points,
            stats.reactive_jacobian_entry_points,
        );
    }

    fn assert_shipped_model_finite_entries_match_bytecode(
        name: &str,
        path: &Path,
        module: Option<&str>,
    ) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let runtime = compiler
            .compile_file_runtime_with_metadata(path, module)
            .unwrap_or_else(|error| {
                panic!(
                    "compile shipped Verilog-A oracle model {name} at {}: {error}",
                    path.display()
                )
            });
        let native = compile_model_with_canonical_ir(&runtime.model, &runtime.canonical_ir)
            .unwrap_or_else(|error| panic!("native x64 compile oracle model {name}: {error}"));
        let mut context = native_model_benchmark_context(&runtime.model, name);
        resolve_native_parameter_defaults(&runtime.model, &native, &mut context);

        let stats =
            assert_native_matches_bytecode_finite_entries(&runtime.model, &native, context, name)
                .unwrap_or_else(|error| panic!("{name}: finite native oracle failed: {error}"));
        eprintln!(
            "native-x64-shipped-oracle model={name} variables={} stamps={} jacobians={} reactive_jacobians={} skipped_nonfinite={}",
            stats.variables,
            stats.stamps,
            stats.jacobians,
            stats.reactive_jacobians,
            stats.skipped_nonfinite,
        );
    }

    fn run_shipped_model_device_probe(name: &str, path: &Path, module: Option<&str>) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let runtime = compiler
            .compile_file_runtime_with_metadata(path, module)
            .unwrap_or_else(|error| {
                panic!(
                    "compile shipped Verilog-A device probe model {name} at {}: {error}",
                    path.display()
                )
            });
        let model = std::sync::Arc::new(runtime.model.clone());
        let nodes = (1..=model.num_terminals).collect::<Vec<_>>();
        let mut device = VerilogADevice::try_new_with_canonical_ir(
            format!("{name}_device"),
            std::sync::Arc::clone(&model),
            &runtime.canonical_ir,
            &nodes,
        )
        .unwrap_or_else(|error| {
            panic!("{name}: shipped device native construction failed: {error}")
        });
        assert!(
            device.is_using_native(),
            "{name}: shipped device must use native code"
        );

        let terminal_count = model.num_terminals;
        let internal_indices = ((terminal_count + 1)
            ..(terminal_count + 1 + device.num_internal_nodes()))
            .collect::<Vec<_>>();
        device.set_internal_node_indices(&internal_indices);
        let branch_start = terminal_count + 1 + device.num_internal_nodes();
        let branch_indices =
            (branch_start..(branch_start + device.num_branch_unknowns())).collect::<Vec<_>>();
        device.set_branch_current_indices(&branch_indices);

        let solution_len =
            terminal_count + device.num_internal_nodes() + device.num_branch_unknowns();
        let mut solution = vec![0.0; solution_len.max(1)];
        for (terminal, node) in nodes.iter().copied().enumerate() {
            solution[node - 1] = shipped_device_terminal_bias(name, terminal);
        }

        device
            .try_update_all_voltages(&solution)
            .unwrap_or_else(|error| {
                panic!("{name}: shipped device voltage update failed: {error}")
            });
        let currents = device.try_evaluate().unwrap_or_else(|error| {
            panic!("{name}: shipped device native evaluate failed: {error}")
        });
        assert_eq!(
            currents.len(),
            model.stamp_programs.len(),
            "{name}: device evaluate must return one current per stamp"
        );
        let finite_currents = currents.iter().filter(|value| value.is_finite()).count();
        assert!(
            finite_currents > 0,
            "{name}: shipped device native evaluate must produce finite currents"
        );

        let mut matrix_entries = 0usize;
        let mut rhs_entries = 0usize;
        let mut matrix_l1 = 0.0_f64;
        let mut rhs_l1 = 0.0_f64;
        device
            .try_stamp(
                &solution,
                |row, col, value| {
                    assert!(
                        value.is_finite(),
                        "{name}: non-finite matrix stamp ({row},{col})={value}"
                    );
                    matrix_entries += 1;
                    matrix_l1 += value.abs();
                },
                |row, value| {
                    assert!(
                        value.is_finite(),
                        "{name}: non-finite rhs stamp ({row})={value}"
                    );
                    rhs_entries += 1;
                    rhs_l1 += value.abs();
                },
            )
            .unwrap_or_else(|error| panic!("{name}: shipped device native stamp failed: {error}"));
        assert!(
            matrix_entries > 0,
            "{name}: shipped device native stamp must produce matrix entries"
        );
        assert!(
            rhs_entries > 0,
            "{name}: shipped device native stamp must produce RHS entries"
        );

        let mut reactive_entries = 0usize;
        let mut reactive_l1 = 0.0_f64;
        device
            .try_stamp_reactive(&solution, |row, col, value| {
                assert!(
                    value.is_finite(),
                    "{name}: non-finite reactive stamp ({row},{col})={value}"
                );
                reactive_entries += 1;
                reactive_l1 += value.abs();
            })
            .unwrap_or_else(|error| {
                panic!("{name}: shipped device native reactive stamp failed: {error}")
            });
        assert!(
            reactive_entries > 0,
            "{name}: shipped device native reactive stamp must produce entries"
        );

        eprintln!(
            "native-x64-shipped-device model={name} native_chunks={} finite_currents={} matrix_entries={} rhs_entries={} reactive_entries={} matrix_l1={matrix_l1:.17e} rhs_l1={rhs_l1:.17e} reactive_l1={reactive_l1:.17e}",
            device.native_chunk_count(),
            finite_currents,
            matrix_entries,
            rhs_entries,
            reactive_entries,
        );
    }

    fn shipped_device_terminal_bias(name: &str, terminal: usize) -> f64 {
        match name {
            "juncap200" => [0.2, 0.0].get(terminal).copied().unwrap_or(0.0),
            "bsimcmg" => [0.05, 0.7, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "bsimbulk" => [0.05, 0.7, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "psp104" => [0.05, 0.7, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "r3_cmc" => [0.1, 0.0, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "diode_cmc" => [0.7, 0.0, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "vbic13_4t" => [0.2, 0.75, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "bsimimg" | "hisimsoi" => [0.05, 0.7, 0.0, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "bsimsoi47" => [0.05, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "bsimsoi100" => [0.05, 0.7, 0.0, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "psp104_nqs" => [0.05, 0.7, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "hicuml0" => [0.2, 0.8, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "hicuml2" => [0.2, 0.8, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "asmhemt" => [0.1, 0.3, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            _ => 0.0,
        }
    }

    #[derive(Default)]
    struct FiniteOracleStats {
        variables: usize,
        stamps: usize,
        jacobians: usize,
        reactive_jacobians: usize,
        skipped_nonfinite: usize,
    }

    fn assert_native_matches_bytecode_finite_entries(
        model: &CompiledModel,
        native: &NativeModel,
        base_context: VmContext,
        name: &str,
    ) -> Result<FiniteOracleStats, String> {
        let mut bytecode_context = base_context.clone();
        bytecode_context.clear_currents();
        bytecode_context
            .currents
            .resize(model.stamp_programs.len(), 0.0);
        let mut vm = Vm::new(&mut bytecode_context);
        let live_assignment_steps = super::live_native_assignment_steps(model);
        execute_bytecode_assignment_steps(&mut vm, &live_assignment_steps)
            .map_err(|error| error.to_string())?;

        let mut native_context = base_context;
        native_context.clear_currents();
        native_context
            .currents
            .resize(model.stamp_programs.len(), 0.0);
        let mut ctx = eval_context_from_vm_context(&mut native_context);
        native.run_assignments(&ctx, native_context.variables.as_mut_ptr());
        if let Some(error) = take_native_runtime_error() {
            return Err(format!("native runtime error during assignments: {error}"));
        }

        let mut stats = FiniteOracleStats::default();
        for index in 0..model.num_variables {
            assert_close_or_skip_nonfinite(
                name,
                format!("variable {index}"),
                vm.context.variables[index],
                native_context.variables[index],
                &mut stats.variables,
                &mut stats.skipped_nonfinite,
            )?;
        }

        let mut prior_current_probes = Vec::new();
        for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
            let bytecode_active = if let Some(condition) = &stamp.static_condition {
                let reference = vm.execute(condition).map_err(|error| error.to_string())?;
                ctx = eval_context_from_vm_context(&mut native_context);
                let actual = native
                    .run_static_condition(stamp_index, &ctx, native_context.variables.as_ptr())
                    .ok_or_else(|| format!("missing native static condition {stamp_index}"))?;
                assert_finite_close(
                    name,
                    format!("static_condition {stamp_index}"),
                    reference,
                    actual,
                )?;
                reference.abs() > 1.0e-15
            } else {
                true
            };

            let native_active = if stamp.static_condition.is_some() {
                ctx = eval_context_from_vm_context(&mut native_context);
                native
                    .run_static_condition(stamp_index, &ctx, native_context.variables.as_ptr())
                    .ok_or_else(|| format!("missing native static condition {stamp_index}"))?
                    .abs()
                    > 1.0e-15
            } else {
                true
            };
            if bytecode_active != native_active {
                return Err(format!(
                    "static condition {stamp_index} active mismatch: bytecode={bytecode_active} native={native_active}"
                ));
            }
            if !bytecode_active {
                push_prior_current_probe_aliases_for_stamp(
                    model,
                    &mut prior_current_probes,
                    stamp_index,
                    stamp,
                );
                continue;
            }

            let reference = execute_bytecode_value_program_with_prior_current_probes(
                &mut vm,
                &stamp.value_program,
                &prior_current_probes,
            )
            .map_err(|error| error.to_string())?;
            ctx = eval_context_from_vm_context(&mut native_context);
            let actual =
                native.run_stamp_value(stamp_index, &ctx, native_context.variables.as_ptr());
            assert_close_or_skip_nonfinite(
                name,
                format!("stamp {stamp_index}"),
                reference,
                actual,
                &mut stats.stamps,
                &mut stats.skipped_nonfinite,
            )?;
            vm.context.currents[stamp_index] = reference;
            native_context.currents[stamp_index] = actual;
            if stamp.branch_ordinal.is_none()
                && let Some((pos, neg)) = super::infer_current_terminal_pair(stamp)
            {
                vm.context.set_branch_current(pos, neg, reference);
                native_context.set_branch_current(pos, neg, actual);
            }

            let mut jacobian_prior_current_probes = prior_current_probes.clone();
            push_prior_current_probe_aliases_for_stamp(
                model,
                &mut jacobian_prior_current_probes,
                stamp_index,
                stamp,
            );
            for entry_index in 0..stamp.jacobian_programs.len() {
                let reference = execute_bytecode_value_program_with_prior_current_probes(
                    &mut vm,
                    &stamp.jacobian_programs[entry_index].program,
                    &jacobian_prior_current_probes,
                )
                .map_err(|error| error.to_string())?;
                ctx = eval_context_from_vm_context(&mut native_context);
                let actual = native.run_jacobian(
                    stamp_index,
                    entry_index,
                    &ctx,
                    native_context.variables.as_ptr(),
                );
                assert_close_or_skip_nonfinite(
                    name,
                    format!("jacobian {stamp_index}.{entry_index}"),
                    reference,
                    actual,
                    &mut stats.jacobians,
                    &mut stats.skipped_nonfinite,
                )?;
            }

            for entry_index in 0..stamp.reactive_jacobians.len() {
                let reference = vm
                    .execute(&stamp.reactive_jacobians[entry_index].program)
                    .map_err(|error| error.to_string())?;
                ctx = eval_context_from_vm_context(&mut native_context);
                let actual = native.run_reactive_jacobian(
                    stamp_index,
                    entry_index,
                    &ctx,
                    native_context.variables.as_ptr(),
                );
                assert_close_or_skip_nonfinite(
                    name,
                    format!("reactive_jacobian {stamp_index}.{entry_index}"),
                    reference,
                    actual,
                    &mut stats.reactive_jacobians,
                    &mut stats.skipped_nonfinite,
                )?;
            }

            push_prior_current_probe_aliases_for_stamp(
                model,
                &mut prior_current_probes,
                stamp_index,
                stamp,
            );
        }

        if let Some(error) = take_native_runtime_error() {
            return Err(format!(
                "native runtime error during finite oracle: {error}"
            ));
        }
        Ok(stats)
    }

    fn assert_close_or_skip_nonfinite(
        name: &str,
        entry: impl std::fmt::Display,
        reference: f64,
        actual: f64,
        matched_count: &mut usize,
        skipped_nonfinite: &mut usize,
    ) -> Result<(), String> {
        if !reference.is_finite() {
            *skipped_nonfinite += 1;
            return Ok(());
        }
        assert_finite_close(name, entry, reference, actual)?;
        *matched_count += 1;
        Ok(())
    }

    fn assert_finite_close(
        name: &str,
        entry: impl std::fmt::Display,
        reference: f64,
        actual: f64,
    ) -> Result<(), String> {
        let entry = entry.to_string();
        if !reference.is_finite() {
            return Err(format!(
                "{entry}: bytecode reference is non-finite: {reference}"
            ));
        }
        if !actual.is_finite() {
            return Err(format!(
                "{entry}: native value is non-finite while bytecode is finite: bytecode={reference} native={actual}"
            ));
        }
        if reference == actual {
            return Ok(());
        }
        let scale = reference.abs().max(actual.abs()).max(1.0);
        let tolerance = 1.0e-8 * scale;
        let delta = (reference - actual).abs();
        if delta <= tolerance {
            return Ok(());
        }
        Err(format!(
            "{entry}: {name} native/reference mismatch: bytecode={reference:.17e} native={actual:.17e} delta={delta:.17e} tolerance={tolerance:.17e}"
        ))
    }

    fn poison_jacobian_bytecode(model: &mut CompiledModel, value: f64) {
        for stamp in &mut model.stamp_programs {
            for jacobian in &mut stamp.jacobian_programs {
                jacobian.program = BytecodeProgram {
                    instructions: vec![Instruction::PushConst(value)],
                };
            }
        }
    }

    fn assert_jacobian_axis_value(
        model: &CompiledModel,
        native: &NativeModel,
        ctx: &EvalContext,
        variables: *const f64,
        stamp_index: usize,
        matches_axis: impl Fn(&ColumnAxis) -> bool,
        expected: f64,
        label: impl std::fmt::Display,
    ) {
        let label = label.to_string();
        let entry_index = model.stamp_programs[stamp_index]
            .jacobian_programs
            .iter()
            .position(|jacobian| matches_axis(&jacobian.col_axis))
            .unwrap_or_else(|| panic!("{label}: missing matching Jacobian axis"));
        let actual = native.run_jacobian(stamp_index, entry_index, ctx, variables);
        if let Err(error) = assert_finite_close("canonical Jacobian", &label, expected, actual) {
            panic!("{error}");
        }
    }

    fn assert_jacobian_axis_approx(
        model: &CompiledModel,
        native: &NativeModel,
        ctx: &EvalContext,
        variables: *const f64,
        stamp_index: usize,
        matches_axis: impl Fn(&ColumnAxis) -> bool,
        expected: f64,
        relative_tolerance: f64,
        label: impl std::fmt::Display,
    ) {
        let label = label.to_string();
        let entry_index = model.stamp_programs[stamp_index]
            .jacobian_programs
            .iter()
            .position(|jacobian| matches_axis(&jacobian.col_axis))
            .unwrap_or_else(|| panic!("{label}: missing matching Jacobian axis"));
        let actual = native.run_jacobian(stamp_index, entry_index, ctx, variables);
        assert!(
            actual.is_finite(),
            "{label}: native Jacobian is non-finite: {actual}"
        );
        let tolerance = relative_tolerance * expected.abs().max(actual.abs()).max(1.0);
        let delta = (actual - expected).abs();
        assert!(
            delta <= tolerance,
            "{label}: canonical Jacobian mismatch: expected={expected:.17e} actual={actual:.17e} delta={delta:.17e} tolerance={tolerance:.17e}"
        );
    }

    fn central_second_derivative(f: impl Fn(f64) -> f64, x: f64) -> f64 {
        let h = 1.0e-4_f64;
        (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h)
    }

    fn native_model_benchmark_context(model: &CompiledModel, name: &str) -> VmContext {
        let mut context = VmContext::with_internal_nodes(model.num_terminals, model.internal_nodes);
        context.voltages.fill(0.0);
        context.internal_voltages.fill(0.0);
        if name == "bsimcmg" && context.voltages.len() >= 5 {
            context.voltages[0] = 0.05;
            context.voltages[1] = 0.7;
            context.voltages[2] = 0.0;
            context.voltages[3] = 0.0;
            context.voltages[4] = 0.0;
        }
        context.parameters = model
            .parameters
            .iter()
            .map(|parameter| parameter.default)
            .collect();
        context.param_given = vec![false; model.parameters.len()];
        context.variables = vec![0.0; model.num_variables.max(1)];
        context.currents = vec![0.0; model.stamp_programs.len()];
        context.branch_current_values = vec![0.0; model.branch_sources.len()];
        context.lookup_tables = model.lookup_tables.clone();
        context.laplace_filters = model.laplace_filters.clone();
        context.zi_filters = model.zi_filters.clone();
        context.time = 1.0e-9;
        context.timestep = 1.0e-12;
        preallocate_native_benchmark_context(&mut context, model);
        context
    }

    fn resolve_native_parameter_defaults(
        model: &CompiledModel,
        native: &NativeModel,
        context: &mut VmContext,
    ) {
        for index in 0..model.parameters.len() {
            let ctx = eval_context_from_vm_context(context);
            if let Some(value) =
                native.run_parameter_default(index, &ctx, context.variables.as_ptr())
            {
                context.parameters[index] = value;
            }
        }
    }

    fn run_native_model_sweep_sample(
        model: &CompiledModel,
        native: &NativeModel,
        context: &mut VmContext,
        name: &str,
        iterations: usize,
        nonfinite_reference: &NonFiniteReference,
    ) -> f64 {
        let mut checksum = 0.0_f64;
        for _ in 0..iterations {
            checksum += std::hint::black_box(run_native_model_sweep_once(
                model,
                native,
                context,
                name,
                nonfinite_reference,
            ));
        }
        std::hint::black_box(checksum)
    }

    fn run_native_model_sweep_once(
        model: &CompiledModel,
        native: &NativeModel,
        context: &mut VmContext,
        name: &str,
        nonfinite_reference: &NonFiniteReference,
    ) -> f64 {
        clear_native_runtime_error();
        context.clear_currents();
        context.currents.resize(model.stamp_programs.len(), 0.0);

        let mut ctx = eval_context_from_vm_context(context);
        native.run_assignments(&ctx, context.variables.as_mut_ptr());

        let mut checksum = 0.0_f64;
        for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
            ctx = eval_context_from_vm_context(context);
            if let Some(active) =
                native.run_static_condition(stamp_index, &ctx, context.variables.as_ptr())
                && active.abs() <= 1.0e-15
            {
                continue;
            }

            ctx = eval_context_from_vm_context(context);
            let value = native.run_stamp_value(stamp_index, &ctx, context.variables.as_ptr());
            if !value.is_finite() {
                assert!(
                    nonfinite_reference.stamps.contains(&stamp_index),
                    "{name}: non-finite stamp {stamp_index} value {value}"
                );
            } else {
                checksum += value;
            }
            context.currents[stamp_index] = value;
            if stamp.branch_ordinal.is_none()
                && let Some((pos, neg)) = super::infer_current_terminal_pair(stamp)
            {
                context.set_branch_current(pos, neg, value);
            }

            for entry_index in 0..stamp.jacobian_programs.len() {
                ctx = eval_context_from_vm_context(context);
                let value =
                    native.run_jacobian(stamp_index, entry_index, &ctx, context.variables.as_ptr());
                if !value.is_finite() {
                    assert!(
                        nonfinite_reference
                            .jacobians
                            .contains(&(stamp_index, entry_index)),
                        "{name}: non-finite jacobian {stamp_index}.{entry_index} value {value}"
                    );
                    continue;
                }
                checksum += value;
            }

            for entry_index in 0..stamp.reactive_jacobians.len() {
                ctx = eval_context_from_vm_context(context);
                let value = native.run_reactive_jacobian(
                    stamp_index,
                    entry_index,
                    &ctx,
                    context.variables.as_ptr(),
                );
                if !value.is_finite() {
                    assert!(
                        nonfinite_reference
                            .reactive_jacobians
                            .contains(&(stamp_index, entry_index)),
                        "{name}: non-finite reactive jacobian {stamp_index}.{entry_index} value {value}"
                    );
                    continue;
                }
                checksum += value;
            }
        }

        if let Some(error) = take_native_runtime_error() {
            panic!("{name}: native runtime error during shipped-model sweep: {error}");
        }
        checksum
    }

    #[derive(Default)]
    struct NonFiniteReference {
        stamps: Vec<usize>,
        jacobians: Vec<(usize, usize)>,
        reactive_jacobians: Vec<(usize, usize)>,
    }

    impl NonFiniteReference {
        fn is_empty(&self) -> bool {
            self.stamps.is_empty()
                && self.jacobians.is_empty()
                && self.reactive_jacobians.is_empty()
        }
    }

    fn collect_bytecode_nonfinite_reference(
        model: &CompiledModel,
        mut context: VmContext,
    ) -> Result<NonFiniteReference, String> {
        let mut reference = NonFiniteReference::default();
        context.clear_currents();
        context.currents.resize(model.stamp_programs.len(), 0.0);
        let mut vm = Vm::new(&mut context);
        let live_assignment_steps = super::live_native_assignment_steps(model);
        execute_bytecode_assignment_steps(&mut vm, &live_assignment_steps)
            .map_err(|error| error.to_string())?;

        let mut prior_current_probes = Vec::new();
        for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
            if let Some(condition) = &stamp.static_condition
                && vm
                    .execute(condition)
                    .map_err(|error| error.to_string())?
                    .abs()
                    <= 1.0e-15
            {
                push_prior_current_probe_aliases_for_stamp(
                    model,
                    &mut prior_current_probes,
                    stamp_index,
                    stamp,
                );
                continue;
            }

            let value = execute_bytecode_value_program_with_prior_current_probes(
                &mut vm,
                &stamp.value_program,
                &prior_current_probes,
            )
            .map_err(|error| error.to_string())?;
            if !value.is_finite() {
                reference.stamps.push(stamp_index);
            }
            vm.context.currents[stamp_index] = value;
            if stamp.branch_ordinal.is_none()
                && let Some((pos, neg)) = super::infer_current_terminal_pair(stamp)
            {
                vm.context.set_branch_current(pos, neg, value);
            }

            let mut jacobian_prior_current_probes = prior_current_probes.clone();
            push_prior_current_probe_aliases_for_stamp(
                model,
                &mut jacobian_prior_current_probes,
                stamp_index,
                stamp,
            );
            for entry_index in 0..stamp.jacobian_programs.len() {
                let value = execute_bytecode_value_program_with_prior_current_probes(
                    &mut vm,
                    &stamp.jacobian_programs[entry_index].program,
                    &jacobian_prior_current_probes,
                )
                .map_err(|error| error.to_string())?;
                if !value.is_finite() {
                    reference.jacobians.push((stamp_index, entry_index));
                }
            }

            for entry_index in 0..stamp.reactive_jacobians.len() {
                let value = vm
                    .execute(&stamp.reactive_jacobians[entry_index].program)
                    .map_err(|error| error.to_string())?;
                if !value.is_finite() {
                    reference
                        .reactive_jacobians
                        .push((stamp_index, entry_index));
                }
            }

            push_prior_current_probe_aliases_for_stamp(
                model,
                &mut prior_current_probes,
                stamp_index,
                stamp,
            );
        }

        Ok(reference)
    }

    fn execute_bytecode_value_program_with_prior_current_probes(
        vm: &mut Vm<'_>,
        program: &crate::codegen::BytecodeProgram,
        prior_current_probes: &[PriorCurrentProbe],
    ) -> Result<f64, crate::vm::VmError> {
        let mut rewritten = None;
        for (index, instruction) in program.instructions.iter().enumerate() {
            let Instruction::PushCurrent(pos, neg) = *instruction else {
                continue;
            };
            if vm.context.try_current(pos, neg).is_ok() {
                continue;
            }
            let Some(value) =
                prior_current_probe_value(vm.context, prior_current_probes, pos, neg)?
            else {
                continue;
            };
            let rewritten = rewritten.get_or_insert_with(|| program.clone());
            rewritten.instructions[index] = Instruction::PushConst(value);
        }

        if let Some(rewritten) = rewritten {
            vm.execute(&rewritten)
        } else {
            vm.execute(program)
        }
    }

    fn prior_current_probe_value(
        context: &VmContext,
        probes: &[PriorCurrentProbe],
        pos: usize,
        neg: usize,
    ) -> Result<Option<f64>, crate::vm::VmError> {
        let mut value = None;
        for probe in probes
            .iter()
            .filter(|probe| probe.pos == pos && probe.neg == neg)
        {
            let current = context.currents.get(probe.current_index).copied().ok_or(
                crate::vm::VmError::InvalidInstruction("missing prior contribution current slot"),
            )?;
            let current = if probe.inverted { -current } else { current };
            value = Some(value.unwrap_or(0.0) + current);
        }
        Ok(value)
    }

    fn push_prior_current_probe_aliases_for_stamp(
        model: &CompiledModel,
        probes: &mut Vec<PriorCurrentProbe>,
        stamp_index: usize,
        stamp: &StampProgram,
    ) {
        if stamp.branch_ordinal.is_none()
            && let Some((pos, neg)) = super::infer_current_unified_pair(model, stamp)
        {
            super::push_prior_current_probe_aliases(probes, stamp_index, pos, neg);
        }
    }

    fn execute_bytecode_assignment_steps(
        vm: &mut Vm<'_>,
        steps: &[AssignmentStep],
    ) -> Result<(), crate::vm::VmError> {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    let value = vm.execute(&assignment.program)?;
                    vm.context.variables[assignment.var_index] = value;
                }
                AssignmentStep::AssignIndexed {
                    base,
                    len,
                    lower,
                    index,
                    value,
                } => {
                    let raw_index = vm.execute(index)?;
                    let slot = Vm::array_slot(raw_index, *base, *len, *lower)?;
                    let value = vm.execute(value)?;
                    vm.context.variables[slot] = value;
                }
                AssignmentStep::Loop { condition, body } => {
                    let mut iterations = 0usize;
                    while vm.execute(condition)?.abs() > 1.0e-15 {
                        execute_bytecode_assignment_steps(vm, body)?;
                        iterations += 1;
                        assert!(
                            iterations < 100_000,
                            "bytecode reference loop exceeded shipped benchmark iteration guard"
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn preallocate_native_benchmark_context(context: &mut VmContext, model: &CompiledModel) {
        let mut max_state = None;
        let mut max_delay_buffer = None;
        let mut max_transition_filter = None;
        let mut max_slew_filter = None;
        let mut max_cross_detector = None;

        let mut scan_program = |program: &BytecodeProgram| {
            for instruction in &program.instructions {
                match instruction {
                    Instruction::DdtState(idx)
                    | Instruction::IdtState(idx)
                    | Instruction::IdtModState(idx)
                    | Instruction::LimitState(idx) => {
                        update_max_slot(&mut max_state, *idx);
                    }
                    Instruction::AbsDelayState(idx) => {
                        update_max_slot(&mut max_delay_buffer, *idx);
                    }
                    Instruction::TransitionState(idx) => {
                        update_max_slot(&mut max_transition_filter, *idx);
                    }
                    Instruction::SlewState(idx) => {
                        update_max_slot(&mut max_slew_filter, *idx);
                    }
                    Instruction::CrossState(idx) => {
                        update_max_slot(&mut max_cross_detector, *idx);
                    }
                    _ => {}
                }
            }
        };

        for parameter in &model.parameters {
            if let Some(program) = &parameter.default_program {
                scan_program(program);
            }
        }
        scan_assignment_steps(&model.assignment_steps, &mut scan_program);
        for stamp in &model.stamp_programs {
            if let Some(condition) = &stamp.static_condition {
                scan_program(condition);
            }
            scan_program(&stamp.value_program);
            for jacobian in &stamp.jacobian_programs {
                scan_program(&jacobian.program);
            }
            for jacobian in &stamp.reactive_jacobians {
                scan_program(&jacobian.program);
            }
        }
        for source in &model.noise_sources {
            scan_program(&source.psd_program);
            if let Some(program) = &source.exponent_program {
                scan_program(program);
            }
        }

        if let Some(max_idx) = max_state {
            context.allocate_states(max_idx + 1);
        }
        if let Some(max_idx) = max_delay_buffer {
            context.allocate_delay_buffers(max_idx + 1);
        }
        if let Some(max_idx) = max_transition_filter {
            context.allocate_transition_filters(max_idx + 1);
        }
        if let Some(max_idx) = max_slew_filter {
            context.allocate_slew_filters(max_idx + 1);
        }
        if let Some(max_idx) = max_cross_detector {
            context.allocate_cross_detectors(max_idx + 1);
        }
    }

    fn scan_assignment_steps(
        steps: &[AssignmentStep],
        scan_program: &mut impl FnMut(&BytecodeProgram),
    ) {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    scan_program(&assignment.program);
                }
                AssignmentStep::AssignIndexed { index, value, .. } => {
                    scan_program(index);
                    scan_program(value);
                }
                AssignmentStep::Loop { condition, body } => {
                    scan_program(condition);
                    scan_assignment_steps(body, scan_program);
                }
            }
        }
    }

    fn eval_context_from_vm_context(context: &mut VmContext) -> EvalContext {
        EvalContext {
            voltages: context.voltages.as_ptr(),
            internal_voltages: context.internal_voltages.as_ptr(),
            params: context.parameters.as_ptr(),
            branch_currents: context.terminal_pair_currents_ptr(),
            branch_currents_len: context.terminal_pair_currents_len(),
            currents: context.currents.as_ptr(),
            currents_len: context.currents.len(),
            num_terminals: context.terminal_count(),
            port_connected: context.port_connected.as_ptr(),
            port_connected_len: context.port_connected.len(),
            temperature: context.temperature,
            time: context.time,
            timestep: context.timestep,
            state_prev: if context.state_values_prev.is_empty() {
                std::ptr::null()
            } else {
                context.state_values_prev.as_ptr()
            },
            state_values: if context.state_values.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_values.as_mut_ptr()
            },
            state_initialized: if context.state_initialized.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_initialized.as_mut_ptr() as *mut u8
            },
            state_initialized_len: context.state_initialized.len(),
            lookup_tables: if context.lookup_tables.is_empty() {
                std::ptr::null()
            } else {
                context.lookup_tables.as_ptr()
            },
            lookup_tables_len: context.lookup_tables.len(),
            laplace_filters: if context.laplace_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.laplace_filters.as_mut_ptr()
            },
            laplace_filters_len: context.laplace_filters.len(),
            param_given: context.param_given.as_ptr() as *const u8,
            param_given_len: context.param_given.len(),
            branch_unknowns: if context.branch_current_values.is_empty() {
                std::ptr::null()
            } else {
                context.branch_current_values.as_ptr()
            },
            analysis_type: context.analysis_type,
            multiplicity: context.multiplicity,
            zi_filters: if context.zi_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.zi_filters.as_mut_ptr()
            },
            zi_filters_len: context.zi_filters.len(),
            transition_filters: if context.transition_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.transition_filters.as_mut_ptr()
            },
            transition_filters_len: context.transition_filters.len(),
            slew_filters: if context.slew_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.slew_filters.as_mut_ptr()
            },
            slew_filters_len: context.slew_filters.len(),
            delay_buffers: if context.delay_buffers.is_empty() {
                std::ptr::null_mut()
            } else {
                context.delay_buffers.as_mut_ptr()
            },
            delay_buffers_len: context.delay_buffers.len(),
            cross_detectors: if context.cross_detectors.is_empty() {
                std::ptr::null_mut()
            } else {
                context.cross_detectors.as_mut_ptr()
            },
            cross_detectors_len: context.cross_detectors.len(),
            state_prev_len: context.state_values_prev.len(),
            state_values_len: context.state_values.len(),
        }
    }

    fn update_max_slot(max_slot: &mut Option<usize>, idx: usize) {
        *max_slot = Some(max_slot.map_or(idx, |prev| prev.max(idx)));
    }

    fn count_assignment_steps(steps: &[AssignmentStep]) -> usize {
        steps
            .iter()
            .map(|step| match step {
                AssignmentStep::Assign(_) | AssignmentStep::AssignIndexed { .. } => 1,
                AssignmentStep::Loop { body, .. } => 1 + count_assignment_steps(body),
            })
            .sum()
    }

    fn shipped_cmc_model_path(parts: &[&str]) -> PathBuf {
        let mut path = shipped_veriloga_model_path(&["cmc"]);
        for part in parts {
            path = path.join(part);
        }
        assert!(
            path.exists(),
            "required shipped CMC model fixture missing: {}",
            path.display()
        );
        path
    }

    fn shipped_veriloga_model_path(parts: &[&str]) -> PathBuf {
        let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("models")
            .join("veriloga");
        for part in parts {
            path = path.join(part);
        }
        assert!(
            path.exists(),
            "required shipped Verilog-A model fixture missing: {}",
            path.display()
        );
        path
    }

    fn shipped_model_microbench_iterations() -> usize {
        2_000
    }

    fn shipped_model_microbench_samples() -> usize {
        5
    }

    fn run_native_model_entry_microbench<F>(
        case: &str,
        entry: &str,
        iterations: usize,
        samples: usize,
        expected: f64,
        mut f: F,
    ) where
        F: FnMut() -> f64,
    {
        assert_near(f(), expected, entry);

        let warmup_iterations = iterations / 10;
        let warmup_checksum = run_native_model_microbench_sample(&mut f, warmup_iterations.max(1));
        assert!(
            warmup_checksum.is_finite(),
            "{case} {entry}: native model microbench warmup checksum must stay finite"
        );

        let mut sample_ns_per_eval = Vec::with_capacity(samples);
        let mut checksum = 0.0_f64;
        for _ in 0..samples {
            let start = std::time::Instant::now();
            checksum += run_native_model_microbench_sample(&mut f, iterations);
            let elapsed = start.elapsed();
            sample_ns_per_eval.push(elapsed.as_nanos() as f64 / iterations as f64);
        }
        sample_ns_per_eval.sort_by(|left, right| left.total_cmp(right));
        let min_ns_per_eval = sample_ns_per_eval[0];
        let median_ns_per_eval = sample_ns_per_eval[sample_ns_per_eval.len() / 2];
        let checksum = std::hint::black_box(checksum);
        assert!(
            checksum.is_finite(),
            "{case} {entry}: native model microbench checksum must stay finite"
        );
        eprintln!(
            "native-x64-model-microbench case={case} entry={entry} min_ns_per_eval={min_ns_per_eval:.3} median_ns_per_eval={median_ns_per_eval:.3} checksum={checksum:.17e}",
        );
    }

    fn run_native_model_microbench_sample<F>(f: &mut F, iterations: usize) -> f64
    where
        F: FnMut() -> f64,
    {
        let mut checksum = 0.0_f64;
        for _ in 0..iterations {
            checksum += std::hint::black_box(f());
        }
        std::hint::black_box(checksum)
    }

    fn native_model_microbench_iterations() -> usize {
        5_000_000
    }

    fn native_model_microbench_samples() -> usize {
        5
    }

    fn assert_near(got: f64, expected: f64, name: &str) {
        assert!(
            (got - expected).abs() <= 1.0e-12,
            "{name}: got {got:.17e}, expected {expected:.17e}"
        );
    }

    fn indexed_assignment_step(
        base: usize,
        len: usize,
        lower: i64,
        index: Vec<Instruction>,
        value: Vec<Instruction>,
    ) -> AssignmentStep {
        AssignmentStep::AssignIndexed {
            base,
            len,
            lower,
            index: BytecodeProgram {
                instructions: index,
            },
            value: BytecodeProgram {
                instructions: value,
            },
        }
    }

    fn compiled_model_with_variables(num_variables: usize) -> CompiledModel {
        CompiledModel {
            name: SmolStr::new("native_x64_assignment_test"),
            num_terminals: 0,
            terminal_names: Vec::new(),
            parameters: Vec::new(),
            num_variables,
            variable_names: Vec::new(),
            assignment_steps: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: 0,
            branch_sources: Vec::new(),
            laplace_filters: Vec::new(),
            zi_filters: Vec::new(),
            noise_sources: Vec::new(),
        }
    }

    fn eval_context(params: &[f64], voltages: &[f64]) -> EvalContext {
        EvalContext {
            voltages: voltages.as_ptr(),
            internal_voltages: std::ptr::null(),
            params: params.as_ptr(),
            branch_currents: std::ptr::null(),
            branch_currents_len: 0,
            currents: std::ptr::null(),
            currents_len: 0,
            num_terminals: voltages.len(),
            port_connected: std::ptr::null(),
            port_connected_len: 0,
            temperature: 0.0,
            time: 0.0,
            timestep: 0.0,
            state_prev: std::ptr::null(),
            state_values: std::ptr::null_mut(),
            state_initialized: std::ptr::null_mut(),
            state_initialized_len: 0,
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
            param_given_len: 0,
            branch_unknowns: std::ptr::null(),
            analysis_type: 0,
            multiplicity: 1.0,
            zi_filters: std::ptr::null_mut(),
            zi_filters_len: 0,
            transition_filters: std::ptr::null_mut(),
            transition_filters_len: 0,
            slew_filters: std::ptr::null_mut(),
            slew_filters_len: 0,
            delay_buffers: std::ptr::null_mut(),
            delay_buffers_len: 0,
            cross_detectors: std::ptr::null_mut(),
            cross_detectors_len: 0,
            state_prev_len: 0,
            state_values_len: 0,
        }
    }
}
