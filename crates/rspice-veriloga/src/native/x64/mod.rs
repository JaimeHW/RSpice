pub(crate) mod codegen;
pub mod encoder;

use super::expr::{
    BranchUnknownRuntimeMapping, EntryKind, NativeLoweringLimits, NativeOp, NativeProgram,
    PriorCurrentProbe, canonical_above_slots_for_equation, canonical_absdelay_slots_for_equation,
    canonical_cross_slots_for_equation, canonical_ddt_slots_for_equation,
    canonical_idt_slots_for_equation, canonical_idtmod_slots_for_equation,
    canonical_laplace_slots_for_equation, canonical_limit_slots_for_equation,
    canonical_slew_slots_for_equation, canonical_table_lookup_slots_for_equation,
    canonical_timer_slots_for_equation, canonical_transition_slots_for_equation,
    canonical_zi_slots_for_equation, constant_dynamic_variable_slot,
};
use super::model::{
    CodeOffset, NativeCurrentDependencies, NativeEntryOffsets, NativeEntryStarts, NativeModel,
    NativeRequiredStorage,
};
use super::runtime::ExecutableMemory;
use super::{JitError, JitResult};
use crate::canonical_ir::{
    CanonicalIrArtifact, EquationId, ExprId, HirAnalogOperator, HirExprKind, HirExpression,
    HirLaplaceKind, HirZiKind, MirEquationKind, MirModel, NodeId, SourceSpanRef,
};
use crate::codegen::{
    AssignmentStep, BytecodeProgram, CompiledModel, CompiledNoiseSource, StampIndex, StampProgram,
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
    let mut entry_starts = Vec::new();
    let (assignment, assignment_dependencies) =
        append_assignment_entry(model, &mut image, &mut entry_starts)?;

    let mut parameter_defaults = Vec::with_capacity(model.parameters.len());
    for (parameter_index, parameter) in model.parameters.iter().enumerate() {
        let default_entry = if let Some(program) = &parameter.default_program {
            let program = lower_parameter_default_program(
                model,
                canonical_mir,
                parameter_index,
                program,
                base_limits,
            )?;
            Some(append_value_entry(&mut image, &mut entry_starts, &program)?)
        } else {
            None
        };
        parameter_defaults.push(default_entry);
    }

    let mut static_conditions = Vec::with_capacity(model.stamp_programs.len());
    let mut static_condition_branch_unknown_dependencies =
        Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_values = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_value_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_value_prior_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut stamp_value_branch_unknown_dependencies =
        Vec::with_capacity(model.stamp_programs.len());
    let mut jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobian_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobian_prior_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobian_branch_unknown_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobian_current_dependencies = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobian_prior_current_dependencies =
        Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobian_branch_unknown_dependencies =
        Vec::with_capacity(model.stamp_programs.len());
    let mut noise_psd = Vec::with_capacity(model.noise_sources.len());
    let mut noise_psd_current_dependencies = Vec::with_capacity(model.noise_sources.len());
    let mut noise_psd_prior_current_dependencies = Vec::with_capacity(model.noise_sources.len());
    let mut noise_psd_branch_unknown_dependencies = Vec::with_capacity(model.noise_sources.len());
    let mut noise_exponents = Vec::with_capacity(model.noise_sources.len());
    let mut noise_exponent_current_dependencies = Vec::with_capacity(model.noise_sources.len());
    let mut noise_exponent_prior_current_dependencies =
        Vec::with_capacity(model.noise_sources.len());
    let mut noise_exponent_branch_unknown_dependencies =
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
            )?;
            static_condition_branch_unknown_dependencies
                .push(program.branch_unknown_dependencies().to_vec());
            Some(append_value_entry(&mut image, &mut entry_starts, &program)?)
        } else {
            static_condition_branch_unknown_dependencies.push(Vec::new());
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
        )?;
        stamp_value_current_dependencies.push(program.current_pair_dependencies().to_vec());
        stamp_value_prior_current_dependencies.push(program.prior_current_dependencies().to_vec());
        stamp_value_branch_unknown_dependencies
            .push(program.branch_unknown_dependencies().to_vec());
        stamp_values.push(append_value_entry(&mut image, &mut entry_starts, &program)?);

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
        let mut stamp_jacobian_branch_unknown_dependencies =
            Vec::with_capacity(stamp.jacobian_programs.len());
        for jacobian in &stamp.jacobian_programs {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Jacobian,
                &jacobian.program,
                jacobian_limits,
            )?;
            stamp_jacobian_current_dependencies.push(program.current_pair_dependencies().to_vec());
            stamp_jacobian_prior_current_dependencies
                .push(program.prior_current_dependencies().to_vec());
            stamp_jacobian_branch_unknown_dependencies
                .push(program.branch_unknown_dependencies().to_vec());
            stamp_jacobians.push(append_value_entry(&mut image, &mut entry_starts, &program)?);
        }
        jacobians.push(stamp_jacobians);
        jacobian_current_dependencies.push(stamp_jacobian_current_dependencies);
        jacobian_prior_current_dependencies.push(stamp_jacobian_prior_current_dependencies);
        jacobian_branch_unknown_dependencies.push(stamp_jacobian_branch_unknown_dependencies);

        let mut stamp_reactive_jacobians = Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_current_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_prior_current_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_branch_unknown_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        for reactive_jacobian in &stamp.reactive_jacobians {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::ReactiveJacobian,
                &reactive_jacobian.program,
                base_limits,
            )?;
            stamp_reactive_jacobian_current_dependencies
                .push(program.current_pair_dependencies().to_vec());
            stamp_reactive_jacobian_prior_current_dependencies
                .push(program.prior_current_dependencies().to_vec());
            stamp_reactive_jacobian_branch_unknown_dependencies
                .push(program.branch_unknown_dependencies().to_vec());
            stamp_reactive_jacobians.push(append_value_entry(
                &mut image,
                &mut entry_starts,
                &program,
            )?);
        }
        reactive_jacobians.push(stamp_reactive_jacobians);
        reactive_jacobian_current_dependencies.push(stamp_reactive_jacobian_current_dependencies);
        reactive_jacobian_prior_current_dependencies
            .push(stamp_reactive_jacobian_prior_current_dependencies);
        reactive_jacobian_branch_unknown_dependencies
            .push(stamp_reactive_jacobian_branch_unknown_dependencies);

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
        )?;
        noise_psd_current_dependencies.push(psd_program.current_pair_dependencies().to_vec());
        noise_psd_prior_current_dependencies
            .push(psd_program.prior_current_dependencies().to_vec());
        noise_psd_branch_unknown_dependencies
            .push(psd_program.branch_unknown_dependencies().to_vec());
        noise_psd.push(append_value_entry(
            &mut image,
            &mut entry_starts,
            &psd_program,
        )?);

        let exponent_entry = if let Some(program) = &source.exponent_program {
            let exponent_program = lower_noise_exponent_program(
                model,
                canonical_noise_plan.as_ref(),
                source_index,
                source,
                program,
                noise_limits,
            )?;
            noise_exponent_current_dependencies
                .push(exponent_program.current_pair_dependencies().to_vec());
            noise_exponent_prior_current_dependencies
                .push(exponent_program.prior_current_dependencies().to_vec());
            noise_exponent_branch_unknown_dependencies
                .push(exponent_program.branch_unknown_dependencies().to_vec());
            Some(append_value_entry(
                &mut image,
                &mut entry_starts,
                &exponent_program,
            )?)
        } else {
            noise_exponent_current_dependencies.push(Vec::new());
            noise_exponent_prior_current_dependencies.push(Vec::new());
            noise_exponent_branch_unknown_dependencies.push(Vec::new());
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
        assignment_current_pairs: assignment_dependencies.current_pairs,
        assignment_prior_currents: assignment_dependencies.prior_currents,
        assignment_branch_unknowns: assignment_dependencies.branch_unknowns,
        static_condition_branch_unknowns: static_condition_branch_unknown_dependencies,
        stamp_values: stamp_value_current_dependencies,
        stamp_value_prior_currents: stamp_value_prior_current_dependencies,
        stamp_value_branch_unknowns: stamp_value_branch_unknown_dependencies,
        jacobians: jacobian_current_dependencies,
        jacobian_prior_currents: jacobian_prior_current_dependencies,
        jacobian_branch_unknowns: jacobian_branch_unknown_dependencies,
        reactive_jacobians: reactive_jacobian_current_dependencies,
        reactive_jacobian_prior_currents: reactive_jacobian_prior_current_dependencies,
        reactive_jacobian_branch_unknowns: reactive_jacobian_branch_unknown_dependencies,
        noise_psd: noise_psd_current_dependencies,
        noise_psd_prior_currents: noise_psd_prior_current_dependencies,
        noise_psd_branch_unknowns: noise_psd_branch_unknown_dependencies,
        noise_exponents: noise_exponent_current_dependencies,
        noise_exponent_prior_currents: noise_exponent_prior_current_dependencies,
        noise_exponent_branch_unknowns: noise_exponent_branch_unknown_dependencies,
    };
    validate_compiled_entry_shape(model, &entries, &current_dependencies)?;

    let executable = ExecutableMemory::allocate(&image)?;
    NativeModel::from_executable_image_with_dependencies(
        model.num_terminals,
        model.internal_nodes,
        model.num_variables,
        model.parameters.len(),
        model.branch_sources.len(),
        executable,
        entries,
        NativeEntryStarts::new(entry_starts),
        current_dependencies,
        NativeRequiredStorage::for_model(model),
    )
}

fn lower_parameter_default_program(
    model: &CompiledModel,
    canonical_mir: Option<&MirModel>,
    parameter_index: usize,
    bytecode_program: &BytecodeProgram,
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

fn append_canonical_expr(mir: &mut MirModel, kind: HirExprKind, span: SourceSpanRef) -> ExprId {
    let id = ExprId::from(mir.expressions.len());
    mir.expressions.push(HirExpression { id, kind, span });
    id
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
        "static-condition branch-unknown dependency",
        dependencies.static_condition_branch_unknowns.len(),
        entries.static_conditions.len(),
    )?;
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
        "stamp-value branch-unknown dependency",
        dependencies.stamp_value_branch_unknowns.len(),
        entries.stamp_values.len(),
    )?;
    validate_nested_current_dependency_shape(
        model,
        "jacobian current dependency",
        &dependencies.jacobians,
        &entries.jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "jacobian prior-current dependency",
        &dependencies.jacobian_prior_currents,
        &entries.jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "jacobian branch-unknown dependency",
        &dependencies.jacobian_branch_unknowns,
        &entries.jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "reactive-jacobian current dependency",
        &dependencies.reactive_jacobians,
        &entries.reactive_jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "reactive-jacobian prior-current dependency",
        &dependencies.reactive_jacobian_prior_currents,
        &entries.reactive_jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "reactive-jacobian branch-unknown dependency",
        &dependencies.reactive_jacobian_branch_unknowns,
        &entries.reactive_jacobians,
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
        "noise PSD branch-unknown dependency",
        dependencies.noise_psd_branch_unknowns.len(),
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
    validate_compiled_entry_count(
        model,
        "noise exponent branch-unknown dependency",
        dependencies.noise_exponent_branch_unknowns.len(),
        entries.noise_exponents.len(),
    )?;
    Ok(())
}

fn validate_nested_current_dependency_shape(
    model: &CompiledModel,
    label: &str,
    dependencies: &[Vec<Vec<usize>>],
    entries: &[Vec<CodeOffset>],
) -> JitResult<()> {
    validate_compiled_entry_count(
        model,
        format!("{label} stamp"),
        dependencies.len(),
        entries.len(),
    )?;
    for (stamp_index, (dependency_entries, entry_offsets)) in
        dependencies.iter().zip(entries).enumerate()
    {
        validate_compiled_entry_count(
            model,
            format!("{label} {stamp_index}"),
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

    validate_canonical_source_digest_for_model(model, artifact)?;
    validate_canonical_parameters_for_model(model, &artifact.mir)?;

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

        if compiled.default_program.is_none() {
            let matches_default = canonical
                .default
                .is_some_and(|default| default.to_bits() == compiled.default.to_bits());
            if !matches_default {
                return Err(JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical parameter '{}' default {:?} does not match compiled default {}",
                        canonical.name, canonical.default, compiled.default
                    )
                    .into(),
                });
            }
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

    let Some((compiled_pos, compiled_neg)) = compiled_pair else {
        return Ok(());
    };
    let branch_matches = match expected_kind {
        MirEquationKind::Current => (canonical_pos, canonical_neg) == (compiled_pos, compiled_neg),
        MirEquationKind::Potential | MirEquationKind::Indirect => {
            same_unordered_current_pair(canonical_pos, canonical_neg, compiled_pos, compiled_neg)
        }
    };
    if !branch_matches {
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

fn same_unordered_current_pair(
    lhs_pos: usize,
    lhs_neg: usize,
    rhs_pos: usize,
    rhs_neg: usize,
) -> bool {
    (lhs_pos, lhs_neg) == (rhs_pos, rhs_neg) || (lhs_pos, lhs_neg) == (rhs_neg, rhs_pos)
}

fn validate_canonical_source_digest_for_model(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<()> {
    if model.source_digest.is_empty() {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: concat!(
                "compiled model is missing source digest for canonical native compilation; ",
                "rebuild it with a digest-aware compiler/codegen path"
            )
            .into(),
        });
    }

    if model.source_digest != artifact.metadata.source_digest {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical source digest '{}' does not match compiled model source digest '{}'",
                artifact.metadata.source_digest, model.source_digest
            )
            .into(),
        });
    }

    Ok(())
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
        let Some(terminal_name) = model.terminal_names.get(node_index) else {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical branch endpoint terminal {node_index} exceeds compiled terminal count {}",
                    model.num_terminals
                )
                .into(),
            });
        };
        if terminal_name != &node.name {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical branch endpoint terminal {node_index} names '{}' but compiled terminal is '{}'",
                    node.name, terminal_name
                )
                .into(),
            });
        }
        return Ok(node_index);
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

#[derive(Debug, Default)]
struct AssignmentDependencies {
    current_pairs: Vec<usize>,
    prior_currents: Vec<usize>,
    branch_unknowns: Vec<usize>,
}

fn append_assignment_entry(
    model: &CompiledModel,
    image: &mut Vec<u8>,
    entry_starts: &mut Vec<CodeOffset>,
) -> JitResult<(CodeOffset, AssignmentDependencies)> {
    let assignments = model
        .assignment_steps
        .iter()
        .map(|step| lower_assignment_step(model, step))
        .collect::<JitResult<Vec<_>>>()?;
    let mut dependencies = AssignmentDependencies::default();
    collect_assignment_dependencies(&assignments, &mut dependencies);

    let bytes = if assignments.is_empty() {
        vec![0xC3]
    } else {
        codegen::compile_assignment_pass_function(&assignments)?
    };
    let offset = align_image_for_entry(image, entry_starts);
    image.extend_from_slice(&bytes);
    Ok((offset, dependencies))
}

fn collect_assignment_dependencies(
    assignments: &[NativeAssignment],
    dependencies: &mut AssignmentDependencies,
) {
    for assignment in assignments {
        match assignment {
            NativeAssignment::Direct { program, .. } => {
                collect_assignment_program_dependencies(program, dependencies);
            }
            NativeAssignment::Indexed { index, value, .. } => {
                collect_assignment_program_dependencies(index, dependencies);
                collect_assignment_program_dependencies(value, dependencies);
            }
            NativeAssignment::Loop { condition, body } => {
                collect_assignment_program_dependencies(condition, dependencies);
                collect_assignment_dependencies(body, dependencies);
            }
        }
    }
}

fn collect_assignment_program_dependencies(
    program: &NativeProgram,
    dependencies: &mut AssignmentDependencies,
) {
    push_unique_indices(
        &mut dependencies.current_pairs,
        program.current_pair_dependencies(),
    );
    push_unique_indices(
        &mut dependencies.prior_currents,
        program.prior_current_dependencies(),
    );
    push_unique_indices(
        &mut dependencies.branch_unknowns,
        program.branch_unknown_dependencies(),
    );
}

fn push_unique_indices(target: &mut Vec<usize>, source: &[usize]) {
    for index in source {
        if !target.contains(index) {
            target.push(*index);
        }
    }
}

fn lower_assignment_step(
    model: &CompiledModel,
    step: &AssignmentStep,
) -> JitResult<NativeAssignment> {
    let limits = NativeLoweringLimits::for_model(model);
    match step {
        AssignmentStep::Assign(assignment) => {
            validate_assignment_target(model, assignment.var_index)?;
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
                validate_assignment_target(model, var_index)?;
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

fn validate_assignment_target(model: &CompiledModel, var_index: usize) -> JitResult<()> {
    if var_index >= model.num_variables {
        return Err(JitError::InternalCompilerError {
            model: model.name.clone(),
            detail: format!(
                "native assignment target variable {var_index} outside variable storage length {}",
                model.num_variables
            )
            .into(),
        });
    }

    Ok(())
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

fn append_value_entry(
    image: &mut Vec<u8>,
    entry_starts: &mut Vec<CodeOffset>,
    program: &NativeProgram,
) -> JitResult<CodeOffset> {
    let bytes = codegen::compile_value_function(program)?;
    let offset = align_image_for_entry(image, entry_starts);
    image.extend_from_slice(&bytes);
    Ok(offset)
}

fn align_image_for_entry(image: &mut Vec<u8>, entry_starts: &mut Vec<CodeOffset>) -> CodeOffset {
    let padding = (ENTRY_ALIGNMENT - (image.len() % ENTRY_ALIGNMENT)) % ENTRY_ALIGNMENT;
    image.resize(image.len() + padding, X64_NOP);
    let offset = CodeOffset::new(image.len());
    entry_starts.push(offset);
    offset
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
    use crate::canonical_ir::{
        BranchUnknownId, CanonicalIrArtifact, HirContributionKind, HirExprKind, MirBranchUnknown,
        MirEquationKind, NodeId, OptModel,
    };
    use crate::codegen::{AssignmentStep, BytecodeProgram, CompiledModel, Instruction};
    use crate::native::EvalContext;
    use crate::native::expr::{EntryKind, NativeLoweringLimits, NativeOp, NativeProgram};
    use crate::native::model::{CodeOffset, NativeCurrentDependencies, NativeEntryOffsets};
    use crate::native::runtime::ExecutableMemory;
    use crate::native::x64::codegen::NativeAssignment;
    use crate::{CompilerOptions, VerilogACompiler};
    use smol_str::SmolStr;

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

    fn rebuild_canonical_artifact(artifact: CanonicalIrArtifact) -> CanonicalIrArtifact {
        let CanonicalIrArtifact {
            metadata, hir, mir, ..
        } = artifact;
        let opt = OptModel::from_mir(&mir).expect("synthetic canonical MIR still validates");
        CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
            .expect("synthetic canonical artifact has refreshed digests")
    }

    #[test]
    fn append_value_entry_aligns_vector_literal_entries_for_concatenated_images() {
        let program = NativeProgram::from_bytecode(
            "aligned-vector-entry",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushVariable(0),
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.0),
                    Instruction::IfElse,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 1, 0),
        )
        .expect("variable ifelse lowers to native program");
        let mut image = vec![0xC3];
        let mut entry_starts = Vec::new();

        let offset = super::append_value_entry(&mut image, &mut entry_starts, &program)
            .expect("append aligned value entry");

        assert_eq!(offset.as_usize(), super::ENTRY_ALIGNMENT);
        assert!(entry_starts.contains(&offset));
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
        let vars = [2.0];

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 1.0_f64.to_bits());
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
            native
                .run_stamp_value(0, &ctx, std::ptr::null())
                .expect("stamp value entry"),
            (voltages[0] - voltages[1]) / params[0]
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

        assert_eq!(value.to_bits(), 6.0_f64.to_bits());
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_parameter_defaults() {
        let source = r#"
module native_canonical_param_guard(p, n);
  inout p, n;
  electrical p, n;
  parameter real base = 2.0;
  parameter real derived = base * 3.0;
  analog I(p, n) <+ V(p, n) * derived;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        artifact.hir.parameters[0].default = Some(4.0);
        artifact.mir.parameters[0].default = Some(4.0);
        let artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-source mismatched parameter default must be rejected");
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
    fn compile_model_with_canonical_ir_rejects_mismatched_stamp_branch() {
        let source = r#"
module native_canonical_shape_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        artifact.hir.contributions[0].branch = "n,p".into();
        artifact.hir.contributions[0].declared_branch = None;
        artifact.mir.equations[0].branch.label = "n,p".into();
        artifact.mir.equations[0].branch.declared_name = None;
        artifact.mir.equations[0].branch.pos_node = Some(NodeId::new(1));
        artifact.mir.equations[0].branch.neg_node = Some(NodeId::new(0));
        let artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-source reversed canonical branch must be rejected");
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
        let source = r#"
module native_canonical_kind_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        artifact.hir.contributions[0].kind = HirContributionKind::Potential;
        artifact.mir.equations[0].kind = MirEquationKind::Potential;
        let equation = artifact.mir.equations[0].clone();
        artifact.mir.branch_unknowns = vec![MirBranchUnknown {
            id: BranchUnknownId::new(0),
            equation: equation.id,
            declared_name: equation.branch.declared_name,
            pos_node: equation.branch.pos_node,
            neg_node: equation.branch.neg_node,
        }];
        let artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-source wrong-kind canonical equation must be rejected");
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
        let params = [2.0_f64];
        let voltages = [0.5_f64, 0.0_f64, 0.25_f64];
        let ctx = eval_context(&params, &voltages);

        let white_psd = native
            .run_noise_psd(0, &ctx, std::ptr::null())
            .expect("white noise PSD entry");
        let flicker_psd = native
            .run_noise_psd(1, &ctx, std::ptr::null())
            .expect("flicker noise PSD entry");
        let flicker_exponent = native
            .run_noise_exponent(1, &ctx, std::ptr::null())
            .expect("flicker source has native exponent entry");

        assert_close(
            "white PSD must come from canonical MIR, not poisoned bytecode",
            (2.0_f64 + 0.25).powi(2) * 3.0,
            white_psd,
        );
        assert_close(
            "flicker PSD must come from canonical MIR, not poisoned bytecode",
            (2.0_f64 - 0.5).powi(2) * 4.0,
            flicker_psd,
        );
        assert_eq!(flicker_exponent.to_bits(), 1.25_f64.to_bits());
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_noise_kind() {
        let source = r#"
module native_canonical_noise_kind_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ white_noise(3.0, "thermal");
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let noise_index = artifact
            .mir
            .expressions
            .iter()
            .position(|expr| match &expr.kind {
                HirExprKind::NoiseSource { .. } => true,
                HirExprKind::SystemFunction { name, .. } | HirExprKind::Call { name, .. } => {
                    name.trim_start_matches('$') == "white_noise"
                }
                _ => false,
            })
            .expect("fixture has canonical noise source");
        let mut exponent_expr = None;
        match &mut artifact.mir.expressions[noise_index].kind {
            HirExprKind::NoiseSource { source, .. } => {
                *source = "flicker".into();
            }
            HirExprKind::SystemFunction { name, args } | HirExprKind::Call { name, args } => {
                *name = "flicker_noise".into();
                exponent_expr = args.get(1).copied();
            }
            _ => unreachable!("fixture expression was selected as a canonical noise source"),
        }
        if let Some(expr) = exponent_expr {
            artifact.mir.expressions[usize::from(expr)].kind = HirExprKind::Number {
                value: 1.0,
                raw: "1.0".into(),
            };
            artifact.hir.expressions[usize::from(expr)].kind =
                artifact.mir.expressions[usize::from(expr)].kind.clone();
        }
        artifact.hir.expressions[noise_index].kind =
            artifact.mir.expressions[noise_index].kind.clone();
        let artifact = rebuild_canonical_artifact(artifact);

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
        assert_eq!(
            native
                .run_stamp_value(0, &ctx, std::ptr::null())
                .expect("first stamp value entry"),
            1.0
        );
        assert_eq!(
            native
                .run_stamp_value(1, &ctx, std::ptr::null())
                .expect("second stamp value entry"),
            6.0
        );
        assert_eq!(
            native.stamp_value_branch_unknowns(1),
            Some([0_usize].as_slice())
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_maps_reversed_duplicate_branch_unknowns() {
        let source = r#"
module native_canonical_reversed_duplicate_vsrc(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    V(n, p) <+ 1.0;
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
            "compiled solver allocation should merge opposite branch orientations"
        );
        assert_eq!(artifact.mir.branch_unknowns.len(), 2);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("reversed canonical branch unknown maps to runtime branch slot");

        let branch_unknowns = [3.0_f64];
        let mut ctx = eval_context(&[], &[0.0, 0.0]);
        ctx.branch_unknowns = branch_unknowns.as_ptr();
        assert_eq!(
            native
                .run_stamp_value(1, &ctx, std::ptr::null())
                .expect("second stamp value entry"),
            -6.0
        );
        assert_eq!(
            native.stamp_value_branch_unknowns(1),
            Some([0_usize].as_slice())
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_maps_named_branch_unknowns() {
        let source = r#"
module native_canonical_named_duplicate_vsrc(p, n);
  inout p, n;
  electrical p, n;
  branch (p, n) probe;
  analog begin
    V(n, p) <+ 1.0;
    V(probe) <+ I(probe) * 2.0;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert_eq!(model.branch_sources.len(), 1);
        assert_eq!(artifact.mir.branch_unknowns.len(), 2);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("named canonical branch unknown maps to runtime branch slot");

        let branch_unknowns = [3.0_f64];
        let mut ctx = eval_context(&[], &[0.0, 0.0]);
        ctx.branch_unknowns = branch_unknowns.as_ptr();
        assert_eq!(
            native
                .run_stamp_value(1, &ctx, std::ptr::null())
                .expect("named branch stamp value entry"),
            -6.0
        );
        assert_eq!(
            native.stamp_value_branch_unknowns(1),
            Some([0_usize].as_slice())
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_source_digest_mismatch() {
        let model_source = r#"
module native_canonical_digest_guard(p, n);
  inout p, n;
  electrical p, n;
  real g;
  analog begin
    g = 1.0;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#;
        let artifact_source = r#"
module native_canonical_digest_guard(p, n);
  inout p, n;
  electrical p, n;
  real g;
  analog begin
    g = 2.0;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(model_source)
            .expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(artifact_source)
            .expect("compile canonical IR");

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("mismatched canonical source must hard-fail");
        let message = error.to_string();
        assert!(
            message.contains("canonical source digest"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical digest mismatch must preserve hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_missing_model_source_digest() {
        let source = r#"
module native_canonical_missing_digest_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        model.source_digest = SmolStr::default();

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("digest-less compiled model must hard-fail");
        let message = error.to_string();
        assert!(
            message.contains("missing source digest"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("digest-aware compiler/codegen path"),
            "missing-digest error must name the required rebuild path: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "missing-digest error must preserve hard-JIT contract: {message}"
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
    fn compile_model_with_canonical_ir_lowers_static_condition_from_mir() {
        let source = r#"
`include "disciplines.vams"
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
        assert_ne!(
            program.ops(),
            &[NativeOp::Const(0.0)],
            "canonical static guard must not use poisoned bytecode"
        );

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical static condition compiles to native x64");
        let params: Vec<f64> = model.parameters.iter().map(|param| param.default).collect();
        let ctx = eval_context(&params, &[2.0, 0.0]);
        let mut vars = vec![0.0_f64; native.num_variables.max(1)];
        native.run_assignments(&ctx, vars.as_mut_ptr());
        let active = native
            .run_static_condition(stamp_index, &ctx, vars.as_ptr())
            .expect("static condition has native entry");

        assert_eq!(active.to_bits(), 1.0_f64.to_bits());
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_missing_static_condition_guard() {
        let source = r#"
`include "disciplines.vams"
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
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model
                .stamp_programs
                .iter()
                .any(|stamp| stamp.static_condition.is_some()),
            "fixture must include a compiled static condition"
        );
        let root = usize::from(artifact.mir.equations[0].expression.id);
        let replacement = HirExprKind::Number {
            value: 1.0,
            raw: "1.0".into(),
        };
        artifact.hir.expressions[root].kind = replacement.clone();
        artifact.mir.expressions[root].kind = replacement;
        artifact.hir.contributions[0].expression.kind = "number".into();
        artifact.mir.equations[0].expression.kind = "number".into();
        artifact = rebuild_canonical_artifact(artifact);

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
            native
                .run_stamp_value(0, &ctx, vars.as_ptr())
                .expect("stamp value entry"),
            1.25,
            "stamp value",
        );
        let expected_jacobian = native
            .run_jacobian(0, 0, &ctx, vars.as_ptr())
            .expect("Jacobian entry");
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
                native
                    .run_stamp_value(
                        0,
                        std::hint::black_box(&ctx),
                        std::hint::black_box(vars.as_ptr()),
                    )
                    .expect("stamp value entry")
            },
        );
        run_native_model_entry_microbench(
            "assignment_fed",
            "jacobian0",
            iterations,
            samples,
            expected_jacobian,
            || {
                native
                    .run_jacobian(
                        0,
                        0,
                        std::hint::black_box(&ctx),
                        std::hint::black_box(vars.as_ptr()),
                    )
                    .expect("Jacobian entry")
            },
        );
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
    fn lower_assignment_step_rejects_direct_target_outside_variable_storage() {
        let model = compiled_model_with_variables(1);
        let step = AssignmentStep::Assign(crate::codegen::AssignmentProgram {
            var_index: 1,
            program: crate::codegen::BytecodeProgram {
                instructions: vec![crate::codegen::Instruction::PushConst(11.0)],
            },
        });

        let error = lower_assignment_step(&model, &step)
            .expect_err("native assignment target must stay inside variable storage");
        let message = error.to_string();
        assert!(
            message.contains("assignment target variable 1"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "native assignment target error must preserve hard-JIT contract: {message}"
        );
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
            assignment_current_pairs: Vec::new(),
            assignment_prior_currents: Vec::new(),
            assignment_branch_unknowns: Vec::new(),
            static_condition_branch_unknowns: vec![Vec::new(); entries.static_conditions.len()],
            stamp_values: vec![Vec::new(); entries.stamp_values.len()],
            stamp_value_prior_currents: vec![Vec::new(); entries.stamp_values.len()],
            stamp_value_branch_unknowns: vec![Vec::new(); entries.stamp_values.len()],
            jacobians: empty_nested_dependencies(&entries.jacobians),
            jacobian_prior_currents: empty_nested_dependencies(&entries.jacobians),
            jacobian_branch_unknowns: empty_nested_dependencies(&entries.jacobians),
            reactive_jacobians: empty_nested_dependencies(&entries.reactive_jacobians),
            reactive_jacobian_prior_currents: empty_nested_dependencies(
                &entries.reactive_jacobians,
            ),
            reactive_jacobian_branch_unknowns: empty_nested_dependencies(
                &entries.reactive_jacobians,
            ),
            noise_psd: vec![Vec::new(); entries.noise_psd.len()],
            noise_psd_prior_currents: vec![Vec::new(); entries.noise_psd.len()],
            noise_psd_branch_unknowns: vec![Vec::new(); entries.noise_psd.len()],
            noise_exponents: vec![Vec::new(); entries.noise_exponents.len()],
            noise_exponent_prior_currents: vec![Vec::new(); entries.noise_exponents.len()],
            noise_exponent_branch_unknowns: vec![Vec::new(); entries.noise_exponents.len()],
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
            source_digest: SmolStr::default(),
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

    fn assert_close(label: &str, expected: f64, actual: f64) {
        let tolerance = 1.0e-12_f64.max(expected.abs() * 1.0e-12);
        assert!(
            (actual - expected).abs() <= tolerance,
            "{label}: expected {expected}, got {actual}"
        );
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
