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
use super::model::{CodeOffset, NativeCurrentDependencies, NativeEntryOffsets, NativeModel};
use super::runtime::ExecutableMemory;
use super::{JitError, JitResult};
use crate::canonical_ir::{CanonicalIrArtifact, EquationId, MirModel, NodeId};
use crate::codegen::{
    AssignmentStep, BytecodeProgram, CompiledModel, Instruction, StampIndex, StampProgram,
};
use crate::native::x64::codegen::NativeAssignment;
use crate::vm::{CURRENT_PAIR_GROUND, terminal_pair_current_index};

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

    let mut image = Vec::new();
    let assignment = CodeOffset::new(image.len());
    append_assignment_entry(model, &mut image)
        .map_err(|error| context_jit_error(error, "assignments"))?;

    let mut parameter_defaults = Vec::with_capacity(model.parameters.len());
    for (parameter_index, parameter) in model.parameters.iter().enumerate() {
        let default_entry = if let Some(program) = &parameter.default_program {
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::ParameterDefault,
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
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::StaticCondition,
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
            let program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::Jacobian,
                &jacobian.program,
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
    for source in &model.noise_sources {
        let psd_program = NativeProgram::from_bytecode(
            model.name.clone(),
            EntryKind::StampValue,
            &source.psd_program,
            noise_limits,
        )
        .map_err(|error| context_jit_error(error, format!("noise psd {}", noise_psd.len())))?;
        noise_psd_current_dependencies.push(psd_program.current_pair_dependencies().to_vec());
        noise_psd_prior_current_dependencies
            .push(psd_program.prior_current_dependencies().to_vec());
        noise_psd.push(append_value_entry(&mut image, &psd_program)?);

        let exponent_entry = if let Some(program) = &source.exponent_program {
            let exponent_program = NativeProgram::from_bytecode(
                model.name.clone(),
                EntryKind::StampValue,
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

    let executable = ExecutableMemory::allocate(&image)?;
    NativeModel::from_executable_image_with_dependencies(
        model.num_variables,
        model.parameters.len(),
        executable,
        NativeEntryOffsets {
            assignment,
            parameter_defaults,
            static_conditions,
            stamp_values,
            jacobians,
            reactive_jacobians,
            noise_psd,
            noise_exponents,
        },
        NativeCurrentDependencies {
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
        },
    )
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

fn append_assignment_entry(model: &CompiledModel, image: &mut Vec<u8>) -> JitResult<()> {
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
    image.extend_from_slice(&bytes);
    Ok(())
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
    let offset = CodeOffset::new(image.len());
    let bytes = codegen::compile_value_function(program)?;
    image.extend_from_slice(&bytes);
    Ok(offset)
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
    use super::{NativeModel, compile_model_with_canonical_ir, lower_assignment_step};
    use crate::canonical_ir::{CanonicalIrArtifact, HirExprKind, OptModel};
    use crate::codegen::{
        AssignmentStep, BytecodeProgram, CompiledModel, Instruction, StampProgram,
    };
    use crate::device::VerilogADevice;
    use crate::native::expr::{NativeOp, PriorCurrentProbe};
    use crate::native::x64::codegen::NativeAssignment;
    use crate::native::{EvalContext, clear_native_runtime_error, take_native_runtime_error};
    use crate::vm::{Vm, VmContext};
    use crate::{CompilerOptions, VerilogACompiler};
    use smol_str::SmolStr;
    use std::path::{Path, PathBuf};

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
