//! Canonical model-plan construction shared by x64, AArch64, and WebAssembly.
//!
//! This module owns all semantic matching, entry lowering, dependency
//! discovery, and assignment scheduling. Backend emitters receive the same
//! validated [`NativeModelPlan`] and may only choose target representation.

use super::assignment::NativeAssignment;
use super::current_dependencies::JitCurrentDependencies as NativeCurrentDependencies;
use super::expr::{
    BranchUnknownRuntimeMapping, CanonicalDerivativeAxis, CanonicalStateOperator, EntryKind,
    NativeIdentifierIndex, NativeLoweringLimits, NativeOp, NativeProgram, PriorCurrentProbe,
    canonical_state_slots_for_expression, canonical_table_lookup_slots_for_equation,
    constant_dynamic_variable_slot, native_op_stack_effect,
};
use super::model_plan::NativeModelPlan;
use super::{JitError, JitResult};
use crate::canonical_ir::{
    CanonicalIrArtifact, EquationId, ExprId, HirAnalogOperator, HirAssignment, HirExprKind,
    HirExprRef, HirExpression, HirLaplaceKind, HirLoop, HirModel, HirStatement, HirZiKind,
    MirEquationKind, MirModel, NodeId, SourceSpanRef,
};
use crate::codegen::{
    AssignmentStep, BytecodeProgram, ColumnAxis, CompiledModel, CompiledNoiseSource, Instruction,
    JacobianEntry, StampIndex, StampProgram,
};
use crate::vm::{CURRENT_PAIR_GROUND, terminal_pair_current_index};
use smol_str::SmolStr;
use std::collections::HashMap;

/// Build the architecture-neutral native plan consumed by every machine
/// backend. The lowering implementation is still colocated here while the
/// original x64 model compiler is split into planning and image-emission
/// phases; no A64-specific decision enters this plan.
pub(crate) fn build_model_plan_with_canonical_ir(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<NativeModelPlan> {
    validate_canonical_artifact_for_model(model, artifact)?;
    build_model_plan_inner(model, Some(artifact))
}

#[cfg(feature = "native-bytecode-contract-tests")]
pub(crate) fn build_model_plan_from_bytecode(model: &CompiledModel) -> JitResult<NativeModelPlan> {
    build_model_plan_inner(model, None)
}

fn build_model_plan_inner(
    model: &CompiledModel,
    canonical_artifact: Option<&CanonicalIrArtifact>,
) -> JitResult<NativeModelPlan> {
    super::coverage::validate_jit_coverage(model)?;
    let canonical_mir = canonical_artifact.map(|artifact| &artifact.mir);
    let canonical_branch_unknown_map = match canonical_mir {
        Some(mir) => canonical_branch_unknown_runtime_map(model, mir)?,
        None => Vec::new(),
    };
    let identifier_index =
        canonical_mir.map(|mir| NativeIdentifierIndex::new(mir, &model.variable_names));
    let base_limits = NativeLoweringLimits::for_model(model)
        .with_canonical_branch_unknown_map(&canonical_branch_unknown_map);
    let base_limits = match &identifier_index {
        Some(identifier_index) => base_limits.with_identifier_index(identifier_index),
        None => base_limits,
    };
    let base_limits = if canonical_mir.is_some() {
        base_limits.with_prevalidated_mir()
    } else {
        base_limits
    };
    let canonical_noise_plan = match canonical_mir {
        Some(mir) => Some(build_canonical_noise_plan(model, mir)?),
        None => None,
    };
    let mut assignment_prior_current_probes = Vec::new();
    if canonical_mir.is_some() {
        for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
            if stamp.branch_ordinal.is_none()
                && let Some((pos, neg)) = infer_current_unified_pair(model, stamp)
            {
                push_completed_current_probe_aliases(
                    &mut assignment_prior_current_probes,
                    stamp_index,
                    pos,
                    neg,
                    model.num_terminals,
                );
            }
        }
    }
    let (assignments, post_assignments, assignment_dependencies, post_assignment_dependencies) =
        lower_assignment_phases(
            model,
            canonical_artifact,
            base_limits.with_prior_current_probes(&assignment_prior_current_probes),
        )?;

    let parameter_defaults = model
        .parameters
        .iter()
        .enumerate()
        .map(|(parameter_index, parameter)| {
            parameter
                .default_program
                .as_ref()
                .map(|program| {
                    lower_parameter_default_program(
                        model,
                        canonical_mir,
                        parameter_index,
                        program,
                        base_limits,
                    )
                })
                .transpose()
        })
        .collect::<JitResult<Vec<_>>>()?;

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
    let mut available_current_pairs = Vec::new();
    let mut prior_current_probes = Vec::new();

    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        let static_condition = stamp
            .static_condition
            .as_ref()
            .map(|condition| {
                lower_static_condition_program(
                    model,
                    canonical_mir,
                    stamp_index,
                    condition,
                    base_limits,
                )
            })
            .transpose()?;
        static_condition_branch_unknown_dependencies.push(
            static_condition.as_ref().map_or_else(Vec::new, |program| {
                program.branch_unknown_dependencies().to_vec()
            }),
        );
        static_conditions.push(static_condition);

        let value_limits = base_limits
            .with_available_current_pairs(&available_current_pairs)
            .with_prior_current_probes(&prior_current_probes);
        let value = lower_stamp_value_program(
            model,
            canonical_mir,
            stamp_index,
            &stamp.value_program,
            value_limits,
        )?;
        stamp_value_current_dependencies.push(value.current_pair_dependencies().to_vec());
        stamp_value_prior_current_dependencies.push(value.prior_current_dependencies().to_vec());
        stamp_value_branch_unknown_dependencies.push(value.branch_unknown_dependencies().to_vec());
        stamp_values.push(value);

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
        let jacobian_table_lookup_slots = match canonical_mir {
            Some(mir) => canonical_table_lookup_slots_for_equation(
                model.name.clone(),
                mir,
                canonical_equation_id(model, stamp_index)?,
                &stamp.value_program,
            )?,
            None => Vec::new(),
        };
        let mut stamp_jacobians = Vec::with_capacity(stamp.jacobian_programs.len());
        let mut stamp_jacobian_current_dependencies =
            Vec::with_capacity(stamp.jacobian_programs.len());
        let mut stamp_jacobian_prior_current_dependencies =
            Vec::with_capacity(stamp.jacobian_programs.len());
        let mut stamp_jacobian_branch_unknown_dependencies =
            Vec::with_capacity(stamp.jacobian_programs.len());
        for jacobian in &stamp.jacobian_programs {
            let program = lower_jacobian_program(
                model,
                canonical_mir,
                stamp_index,
                jacobian,
                jacobian_limits,
                &jacobian_table_lookup_slots,
            )?;
            stamp_jacobian_current_dependencies.push(program.current_pair_dependencies().to_vec());
            stamp_jacobian_prior_current_dependencies
                .push(program.prior_current_dependencies().to_vec());
            stamp_jacobian_branch_unknown_dependencies
                .push(program.branch_unknown_dependencies().to_vec());
            stamp_jacobians.push(program);
        }
        jacobians.push(stamp_jacobians);
        jacobian_current_dependencies.push(stamp_jacobian_current_dependencies);
        jacobian_prior_current_dependencies.push(stamp_jacobian_prior_current_dependencies);
        jacobian_branch_unknown_dependencies.push(stamp_jacobian_branch_unknown_dependencies);

        let canonical_reactive_mir = match canonical_mir {
            Some(mir) if !stamp.reactive_jacobians.is_empty() => Some(canonical_reactive_mir(
                model,
                mir,
                canonical_equation_id(model, stamp_index)?,
            )?),
            _ => None,
        };
        let mut stamp_reactive_jacobians = Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_current_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_prior_current_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        let mut stamp_reactive_jacobian_branch_unknown_dependencies =
            Vec::with_capacity(stamp.reactive_jacobians.len());
        for reactive_jacobian in &stamp.reactive_jacobians {
            let program = lower_reactive_jacobian_program(
                model,
                canonical_reactive_mir.as_ref(),
                stamp_index,
                reactive_jacobian,
                base_limits,
            )?;
            stamp_reactive_jacobian_current_dependencies
                .push(program.current_pair_dependencies().to_vec());
            stamp_reactive_jacobian_prior_current_dependencies
                .push(program.prior_current_dependencies().to_vec());
            stamp_reactive_jacobian_branch_unknown_dependencies
                .push(program.branch_unknown_dependencies().to_vec());
            stamp_reactive_jacobians.push(program);
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
    for (source_index, source) in model.noise_sources.iter().enumerate() {
        let psd = lower_noise_psd_program(
            model,
            canonical_noise_plan.as_ref(),
            source_index,
            source,
            &source.psd_program,
            noise_limits,
        )?;
        noise_psd_current_dependencies.push(psd.current_pair_dependencies().to_vec());
        noise_psd_prior_current_dependencies.push(psd.prior_current_dependencies().to_vec());
        noise_psd_branch_unknown_dependencies.push(psd.branch_unknown_dependencies().to_vec());
        noise_psd.push(psd);

        let exponent = source
            .exponent_program
            .as_ref()
            .map(|program| {
                lower_noise_exponent_program(
                    model,
                    canonical_noise_plan.as_ref(),
                    source_index,
                    source,
                    program,
                    noise_limits,
                )
            })
            .transpose()?;
        noise_exponent_current_dependencies.push(
            exponent.as_ref().map_or_else(Vec::new, |program| {
                program.current_pair_dependencies().to_vec()
            }),
        );
        noise_exponent_prior_current_dependencies.push(
            exponent.as_ref().map_or_else(Vec::new, |program| {
                program.prior_current_dependencies().to_vec()
            }),
        );
        noise_exponent_branch_unknown_dependencies.push(
            exponent.as_ref().map_or_else(Vec::new, |program| {
                program.branch_unknown_dependencies().to_vec()
            }),
        );
        noise_exponents.push(exponent);
    }

    let published_current_pairs = model
        .stamp_programs
        .iter()
        .map(|stamp| {
            let Some((pos, neg)) = (stamp.branch_ordinal.is_none())
                .then(|| infer_current_terminal_pair(stamp))
                .flatten()
            else {
                return Ok(None);
            };
            let forward = terminal_pair_current_index(pos, neg, model.num_terminals)
                .ok_or_else(|| current_pair_unavailable(model, pos, neg))?;
            let reverse = terminal_pair_current_index(neg, pos, model.num_terminals)
                .ok_or_else(|| current_pair_unavailable(model, neg, pos))?;
            Ok(Some((forward, reverse)))
        })
        .collect::<JitResult<Vec<_>>>()?;
    let current_dependencies = NativeCurrentDependencies {
        assignment_current_pairs: assignment_dependencies.current_pairs,
        assignment_prior_currents: assignment_dependencies.prior_currents,
        assignment_branch_unknowns: assignment_dependencies.branch_unknowns,
        post_assignment_current_pairs: post_assignment_dependencies.current_pairs,
        post_assignment_prior_currents: post_assignment_dependencies.prior_currents,
        post_assignment_branch_unknowns: post_assignment_dependencies.branch_unknowns,
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
    let plan = NativeModelPlan {
        assignments,
        post_assignments,
        parameter_defaults,
        static_conditions,
        stamp_values,
        jacobians,
        reactive_jacobians,
        noise_psd,
        noise_exponents,
        published_current_pairs,
        current_dependencies,
    };
    plan.validate_shape(model)?;
    Ok(plan)
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

fn lower_jacobian_program(
    model: &CompiledModel,
    canonical_mir: Option<&MirModel>,
    stamp_index: usize,
    jacobian: &JacobianEntry,
    limits: NativeLoweringLimits<'_>,
    table_lookup_slots: &[(ExprId, usize)],
) -> JitResult<NativeProgram> {
    if let Some(mir) = canonical_mir {
        let equation_id = canonical_equation_id(model, stamp_index)?;
        let axis = canonical_derivative_axis_for_column(model, mir, &jacobian.col_axis)?;
        let state_slots = CanonicalExpressionStateSlots::for_equation(
            model,
            mir,
            equation_id,
            &jacobian.program,
        )?;
        return NativeProgram::from_mir_derivative(
            model.name.clone(),
            EntryKind::Jacobian,
            mir,
            equation_id,
            axis,
            // Jacobian bytecode carries TableDerivative rather than
            // TableLookup, so its table id comes from the stamp-value
            // program. Apply that mapping last: an otherwise-empty state
            // scan must not replace it with an empty table map.
            state_slots
                .apply(limits)
                .with_canonical_table_lookup_slots(table_lookup_slots),
        );
    }

    NativeProgram::from_bytecode(
        model.name.clone(),
        EntryKind::Jacobian,
        &jacobian.program,
        limits,
    )
}

fn lower_reactive_jacobian_program(
    model: &CompiledModel,
    canonical_reactive_mir: Option<&MirModel>,
    stamp_index: usize,
    reactive_jacobian: &JacobianEntry,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    if let Some(reactive_mir) = canonical_reactive_mir {
        let equation_id = canonical_equation_id(model, stamp_index)?;
        let axis =
            canonical_derivative_axis_for_column(model, reactive_mir, &reactive_jacobian.col_axis)?;
        let table_lookup_slots = canonical_table_lookup_slots_for_equation(
            model.name.clone(),
            reactive_mir,
            equation_id,
            &reactive_jacobian.program,
        )?;
        return NativeProgram::from_mir_derivative(
            model.name.clone(),
            EntryKind::ReactiveJacobian,
            reactive_mir,
            equation_id,
            axis,
            limits.with_canonical_table_lookup_slots(&table_lookup_slots),
        );
    }

    NativeProgram::from_bytecode(
        model.name.clone(),
        EntryKind::ReactiveJacobian,
        &reactive_jacobian.program,
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

fn canonical_reactive_mir(
    model: &CompiledModel,
    mir: &MirModel,
    equation_id: EquationId,
) -> JitResult<MirModel> {
    let equation_index = usize::from(equation_id);
    let equation = mir
        .equations
        .get(equation_index)
        .filter(|equation| equation.id == equation_id)
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!("canonical equation {equation_id} is outside equation arena").into(),
        })?;
    let mut reactive_mir = mir.clone();
    let root =
        canonical_extract_reactive_charge(model, &mut reactive_mir, equation.expression.id)?
            .ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "compiled reactive Jacobian for stamp {equation_index} has no canonical ddt charge expression"
                )
                .into(),
            })?;
    let root_ref = canonical_expr_ref(model, &reactive_mir, root)?;
    reactive_mir.equations[equation_index].expression = root_ref;
    Ok(reactive_mir)
}

fn canonical_is_ddt_call(name: &str) -> bool {
    name.strip_prefix('$')
        .unwrap_or(name)
        .eq_ignore_ascii_case("ddt")
}

fn canonical_extract_reactive_charge(
    model: &CompiledModel,
    mir: &mut MirModel,
    expr_id: ExprId,
) -> JitResult<Option<ExprId>> {
    let expression = canonical_expression(model, mir, expr_id)?.clone();
    match expression.kind {
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Ddt { expr, .. },
        } => Ok(Some(expr)),
        HirExprKind::Call { name, args } if canonical_is_ddt_call(name.as_str()) => {
            match args.as_slice() {
                [expr] => Ok(Some(*expr)),
                _ => Err(JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical ddt call {expr_id} has {} operands, expected 1",
                        args.len()
                    )
                    .into(),
                }),
            }
        }
        HirExprKind::Binary { op, left, right } if op == "Add" || op == "Sub" => {
            let left_charge = canonical_extract_reactive_charge(model, mir, left)?;
            let right_charge = canonical_extract_reactive_charge(model, mir, right)?;
            if left_charge.is_none() && right_charge.is_none() {
                return Ok(None);
            }
            let zero = append_canonical_number(mir, 0.0, "0.0", expression.span);
            let left = left_charge.unwrap_or(zero);
            let right = right_charge.unwrap_or(zero);
            Ok(Some(append_canonical_binary(
                mir,
                if op == "Add" { "Add" } else { "Sub" },
                left,
                right,
                expression.span,
            )))
        }
        HirExprKind::Binary { op, left, right } if op == "Mul" => {
            let left_has_ddt = canonical_expr_contains_ddt(model, mir, left)?;
            let right_has_ddt = canonical_expr_contains_ddt(model, mir, right)?;
            match (left_has_ddt, right_has_ddt) {
                (false, false) => Ok(None),
                (false, true) => {
                    let charge = canonical_extract_reactive_charge(model, mir, right)?;
                    Ok(charge
                        .map(|charge| append_canonical_binary(mir, "Mul", left, charge, expression.span)))
                }
                (true, false) => {
                    let charge = canonical_extract_reactive_charge(model, mir, left)?;
                    Ok(charge
                        .map(|charge| append_canonical_binary(mir, "Mul", charge, right, expression.span)))
                }
                (true, true) => Err(JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical equation with reactive Jacobian places ddt on both sides of product {expr_id}"
                    )
                    .into(),
                }),
            }
        }
        HirExprKind::Binary { op, left, right } if op == "Div" => {
            if canonical_expr_contains_ddt(model, mir, right)? {
                return Err(JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical equation with reactive Jacobian places ddt in divisor {expr_id}"
                    )
                    .into(),
                });
            }
            let charge = canonical_extract_reactive_charge(model, mir, left)?;
            Ok(charge
                .map(|charge| append_canonical_binary(mir, "Div", charge, right, expression.span)))
        }
        HirExprKind::Unary { op, operand } if op == "Neg" || op == "Pos" => {
            let charge = canonical_extract_reactive_charge(model, mir, operand)?;
            Ok(charge.map(|charge| {
                append_canonical_expr(
                    mir,
                    HirExprKind::Unary {
                        op: op.clone(),
                        operand: charge,
                    },
                    expression.span,
                )
            }))
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            let then_charge = canonical_extract_reactive_charge(model, mir, then_expr)?;
            let else_charge = canonical_extract_reactive_charge(model, mir, else_expr)?;
            if then_charge.is_none() && else_charge.is_none() {
                return Ok(None);
            }
            let zero = append_canonical_number(mir, 0.0, "0.0", expression.span);
            Ok(Some(append_canonical_expr(
                mir,
                HirExprKind::Conditional {
                    condition,
                    then_expr: then_charge.unwrap_or(zero),
                    else_expr: else_charge.unwrap_or(zero),
                },
                expression.span,
            )))
        }
        _ => {
            if canonical_expr_contains_ddt(model, mir, expr_id)? {
                return Err(JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical equation with reactive Jacobian contains ddt in unsupported expression {}",
                        canonical_expr_kind_name(&expression.kind)
                    )
                    .into(),
                });
            }
            Ok(None)
        }
    }
}

fn canonical_expr_contains_ddt(
    model: &CompiledModel,
    mir: &MirModel,
    expr_id: ExprId,
) -> JitResult<bool> {
    let expression = canonical_expression(model, mir, expr_id)?;
    match &expression.kind {
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Ddt { .. },
        } => Ok(true),
        HirExprKind::Call { name, .. } if canonical_is_ddt_call(name.as_str()) => Ok(true),
        HirExprKind::NullArgument
        | HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. } => Ok(false),
        HirExprKind::SystemFunction { args, .. }
        | HirExprKind::Call { args, .. }
        | HirExprKind::ArrayLiteral { elements: args }
        | HirExprKind::NoiseSource { operands: args, .. } => {
            canonical_expr_list_contains_ddt(model, mir, args)
        }
        HirExprKind::Unary { operand, .. } | HirExprKind::ArrayAccess { index: operand, .. } => {
            canonical_expr_contains_ddt(model, mir, *operand)
        }
        HirExprKind::Binary { left, right, .. } => {
            Ok(canonical_expr_contains_ddt(model, mir, *left)?
                || canonical_expr_contains_ddt(model, mir, *right)?)
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => Ok(canonical_expr_contains_ddt(model, mir, *condition)?
            || canonical_expr_contains_ddt(model, mir, *then_expr)?
            || canonical_expr_contains_ddt(model, mir, *else_expr)?),
        HirExprKind::AnalogOperator { op } => {
            canonical_analog_operator_contains_ddt(model, mir, op)
        }
        HirExprKind::Laplace { expr, kind } => {
            if canonical_expr_contains_ddt(model, mir, *expr)? {
                return Ok(true);
            }
            canonical_laplace_kind_contains_ddt(model, mir, kind)
        }
        HirExprKind::Zi {
            expr,
            kind,
            period,
            transition,
            first_transition,
        } => {
            if canonical_expr_contains_ddt(model, mir, *expr)?
                || canonical_expr_contains_ddt(model, mir, *period)?
                || transition
                    .map(|expr| canonical_expr_contains_ddt(model, mir, expr))
                    .transpose()?
                    .unwrap_or(false)
                || first_transition
                    .map(|expr| canonical_expr_contains_ddt(model, mir, expr))
                    .transpose()?
                    .unwrap_or(false)
            {
                return Ok(true);
            }
            canonical_zi_kind_contains_ddt(model, mir, kind)
        }
    }
}

fn canonical_expr_list_contains_ddt(
    model: &CompiledModel,
    mir: &MirModel,
    exprs: &[ExprId],
) -> JitResult<bool> {
    for expr in exprs {
        if canonical_expr_contains_ddt(model, mir, *expr)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn canonical_analog_operator_contains_ddt(
    model: &CompiledModel,
    mir: &MirModel,
    op: &HirAnalogOperator,
) -> JitResult<bool> {
    match op {
        HirAnalogOperator::Limit {
            proposed,
            candidate,
            type_metadata,
            ..
        } => Ok(canonical_expr_contains_ddt(model, mir, *proposed)?
            || canonical_expr_contains_ddt(model, mir, *candidate)?
            || canonical_optional_expr_contains_ddt(model, mir, *type_metadata)?),
        HirAnalogOperator::LimiterArgument { .. } => Ok(false),
        HirAnalogOperator::Ddt { .. } => Ok(true),
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => Ok(canonical_expr_contains_ddt(model, mir, *expr)?
            || canonical_optional_expr_contains_ddt(model, mir, *ic)?
            || canonical_optional_expr_contains_ddt(model, mir, *assert)?
            || canonical_optional_expr_contains_ddt(model, mir, *abstol)?),
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => Ok(canonical_expr_contains_ddt(model, mir, *expr)?
            || canonical_optional_expr_contains_ddt(model, mir, *ic)?
            || canonical_optional_expr_contains_ddt(model, mir, *modulus)?
            || canonical_optional_expr_contains_ddt(model, mir, *offset)?
            || canonical_optional_expr_contains_ddt(model, mir, *abstol)?),
        HirAnalogOperator::Ddx { expr, probe } => {
            Ok(canonical_expr_contains_ddt(model, mir, *expr)?
                || canonical_expr_contains_ddt(model, mir, *probe)?)
        }
        HirAnalogOperator::Limexp { expr } | HirAnalogOperator::LastCrossing { expr, .. } => {
            canonical_expr_contains_ddt(model, mir, *expr)
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => Ok(canonical_expr_contains_ddt(model, mir, *expr)?
            || canonical_expr_contains_ddt(model, mir, *delay)?
            || canonical_optional_expr_contains_ddt(model, mir, *max_delay)?),
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => Ok(canonical_expr_contains_ddt(model, mir, *expr)?
            || canonical_optional_expr_contains_ddt(model, mir, *delay)?
            || canonical_optional_expr_contains_ddt(model, mir, *rise)?
            || canonical_optional_expr_contains_ddt(model, mir, *fall)?
            || canonical_optional_expr_contains_ddt(model, mir, *tolerance)?),
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => Ok(canonical_expr_contains_ddt(model, mir, *expr)?
            || canonical_optional_expr_contains_ddt(model, mir, *max_rise)?
            || canonical_optional_expr_contains_ddt(model, mir, *max_fall)?),
    }
}

fn canonical_laplace_kind_contains_ddt(
    model: &CompiledModel,
    mir: &MirModel,
    kind: &HirLaplaceKind,
) -> JitResult<bool> {
    match kind {
        HirLaplaceKind::ZeroPole { zeros, poles } => {
            Ok(canonical_expr_list_contains_ddt(model, mir, zeros)?
                || canonical_expr_list_contains_ddt(model, mir, poles)?)
        }
        HirLaplaceKind::ZeroDenominator { zeros, denominator } => {
            Ok(canonical_expr_list_contains_ddt(model, mir, zeros)?
                || canonical_expr_list_contains_ddt(model, mir, denominator)?)
        }
        HirLaplaceKind::NumeratorPole { numerator, poles } => {
            Ok(canonical_expr_list_contains_ddt(model, mir, numerator)?
                || canonical_expr_list_contains_ddt(model, mir, poles)?)
        }
        HirLaplaceKind::NumeratorDenominator {
            numerator,
            denominator,
        } => Ok(canonical_expr_list_contains_ddt(model, mir, numerator)?
            || canonical_expr_list_contains_ddt(model, mir, denominator)?),
    }
}

fn canonical_zi_kind_contains_ddt(
    model: &CompiledModel,
    mir: &MirModel,
    kind: &HirZiKind,
) -> JitResult<bool> {
    match kind {
        HirZiKind::ZeroPole { zeros, poles } => {
            Ok(canonical_expr_list_contains_ddt(model, mir, zeros)?
                || canonical_expr_list_contains_ddt(model, mir, poles)?)
        }
        HirZiKind::ZeroDenominator { zeros, denominator } => {
            Ok(canonical_expr_list_contains_ddt(model, mir, zeros)?
                || canonical_expr_list_contains_ddt(model, mir, denominator)?)
        }
        HirZiKind::NumeratorPole { numerator, poles } => {
            Ok(canonical_expr_list_contains_ddt(model, mir, numerator)?
                || canonical_expr_list_contains_ddt(model, mir, poles)?)
        }
        HirZiKind::NumeratorDenominator {
            numerator,
            denominator,
        } => Ok(canonical_expr_list_contains_ddt(model, mir, numerator)?
            || canonical_expr_list_contains_ddt(model, mir, denominator)?),
    }
}

fn canonical_optional_expr_contains_ddt(
    model: &CompiledModel,
    mir: &MirModel,
    expr: Option<ExprId>,
) -> JitResult<bool> {
    match expr {
        Some(expr) => canonical_expr_contains_ddt(model, mir, expr),
        None => Ok(false),
    }
}

fn canonical_expr_ref(
    model: &CompiledModel,
    mir: &MirModel,
    expr_id: ExprId,
) -> JitResult<HirExprRef> {
    let expression = canonical_expression(model, mir, expr_id)?;
    Ok(HirExprRef {
        id: expr_id,
        kind: canonical_expr_ref_kind(&expression.kind).into(),
        span: expression.span,
    })
}

fn canonical_expr_ref_kind(kind: &HirExprKind) -> &'static str {
    match kind {
        HirExprKind::NullArgument => "null_argument",
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
    equation_id: EquationId,
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
        return NativeProgram::from_mir_expression_for_equation(
            model.name.clone(),
            EntryKind::StampValue,
            &plan.mir,
            entry.equation_id,
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
        return NativeProgram::from_mir_expression_for_equation(
            model.name.clone(),
            EntryKind::StampValue,
            &plan.mir,
            entry.equation_id,
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
                equation_id: canonical_equation_id(model, equation_index)?,
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
                equation_id: canonical_equation_id(model, equation_index)?,
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
        HirExprKind::NullArgument
        | HirExprKind::Number { .. }
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
        HirExprKind::Zi {
            expr,
            kind,
            period,
            transition,
            first_transition,
        } => Ok(canonical_expr_contains_noise(model, mir, *expr)?
            || canonical_expr_contains_noise(model, mir, *period)?
            || transition
                .map(|expr| canonical_expr_contains_noise(model, mir, expr))
                .transpose()?
                .unwrap_or(false)
            || first_transition
                .map(|expr| canonical_expr_contains_noise(model, mir, expr))
                .transpose()?
                .unwrap_or(false)
            || canonical_zi_kind_contains_noise(model, mir, kind)?),
    }
}

fn canonical_analog_operator_contains_noise(
    model: &CompiledModel,
    mir: &MirModel,
    op: &HirAnalogOperator,
) -> JitResult<bool> {
    match op {
        HirAnalogOperator::Limit {
            proposed,
            candidate,
            type_metadata,
            ..
        } => Ok(canonical_expr_contains_noise(model, mir, *proposed)?
            || canonical_expr_contains_noise(model, mir, *candidate)?
            || canonical_optional_expr_contains_noise(model, mir, *type_metadata)?),
        HirAnalogOperator::LimiterArgument { .. } => Ok(false),
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

pub(crate) fn lower_static_condition_program(
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
        HirExprKind::NullArgument => "null_argument",
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

        let resolve_range_parameter = |name: &SmolStr| {
            mir.parameters
                .iter()
                .position(|parameter| parameter.name == *name)
                .ok_or_else(|| JitError::InvalidCanonicalIr {
                    model: model.name.clone(),
                    detail: format!(
                        "canonical parameter '{}' range references unknown parameter '{}'",
                        canonical.name, name
                    )
                    .into(),
                })
        };
        let canonical_min = canonical.range.as_ref().and_then(|range| range.min);
        let canonical_max = canonical.range.as_ref().and_then(|range| range.max);
        let canonical_min_parameter = canonical
            .range
            .as_ref()
            .and_then(|range| range.min_parameter.as_ref())
            .map(resolve_range_parameter)
            .transpose()?;
        let canonical_max_parameter = canonical
            .range
            .as_ref()
            .and_then(|range| range.max_parameter.as_ref())
            .map(resolve_range_parameter)
            .transpose()?;
        let canonical_exclude_parameters = canonical
            .range
            .as_ref()
            .map(|range| {
                range
                    .exclude_parameters
                    .iter()
                    .map(resolve_range_parameter)
                    .collect::<JitResult<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();
        let canonical_exclude = canonical
            .range
            .as_ref()
            .map(|range| range.exclude.as_slice())
            .unwrap_or_default();
        let same_optional_float = |left: Option<f64>, right: Option<f64>| match (left, right) {
            (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
            (None, None) => true,
            _ => false,
        };
        let same_float_slice = |left: &[f64], right: &[f64]| {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| left.to_bits() == right.to_bits())
        };
        if !same_optional_float(canonical_min, compiled.min)
            || !same_optional_float(canonical_max, compiled.max)
            || canonical_min_parameter != compiled.min_parameter
            || canonical_max_parameter != compiled.max_parameter
            || canonical
                .range
                .as_ref()
                .and_then(|range| range.min_expression.as_ref())
                .is_some()
                != compiled.min_program.is_some()
            || canonical
                .range
                .as_ref()
                .and_then(|range| range.max_expression.as_ref())
                .is_some()
                != compiled.max_program.is_some()
            || canonical
                .range
                .as_ref()
                .is_some_and(|range| range.min_exclusive)
                != compiled.min_exclusive
            || canonical
                .range
                .as_ref()
                .is_some_and(|range| range.max_exclusive)
                != compiled.max_exclusive
            || !same_float_slice(canonical_exclude, &compiled.exclude)
            || canonical_exclude_parameters != compiled.exclude_parameters
            || canonical
                .range
                .as_ref()
                .map(|range| range.exclude_expressions.len())
                .unwrap_or_default()
                != compiled.exclude_programs.len()
        {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical parameter '{}' range metadata does not match compiled parameter metadata",
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

pub(crate) fn canonical_branch_unknown_runtime_map(
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
        let state_slots =
            CanonicalExpressionStateSlots::for_equation(model, mir, equation_id, bytecode_program)?;
        return NativeProgram::from_mir_equation(
            model.name.clone(),
            EntryKind::StampValue,
            mir,
            equation_id,
            state_slots.apply(limits),
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

fn lower_assignment_phases(
    model: &CompiledModel,
    canonical_artifact: Option<&CanonicalIrArtifact>,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<(
    Vec<NativeAssignment>,
    Vec<NativeAssignment>,
    AssignmentDependencies,
    AssignmentDependencies,
)> {
    let (assignments, post_assignments) = match canonical_artifact {
        Some(artifact) => {
            let assignments = lower_live_canonical_assignment_statements(
                model,
                &artifact.hir,
                &artifact.mir,
                limits,
            )?;
            split_canonical_assignment_phases(model, &artifact.mir, assignments, limits)?
        }
        None => {
            let live_assignment_steps = live_native_assignment_steps(model);
            let assignments = live_assignment_steps
                .iter()
                .map(|step| lower_assignment_step_with_limits(model, step, limits))
                .collect::<JitResult<Vec<_>>>()?;
            (assignments, Vec::new())
        }
    };
    let mut assignment_dependencies = AssignmentDependencies::default();
    collect_assignment_dependencies(&assignments, &mut assignment_dependencies);
    let mut post_assignment_dependencies = AssignmentDependencies::default();
    collect_assignment_dependencies(&post_assignments, &mut post_assignment_dependencies);
    Ok((
        assignments,
        post_assignments,
        assignment_dependencies,
        post_assignment_dependencies,
    ))
}

fn split_canonical_assignment_phases(
    model: &CompiledModel,
    mir: &MirModel,
    mut assignments: Vec<NativeAssignment>,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<(Vec<NativeAssignment>, Vec<NativeAssignment>)> {
    let Some(split_index) = assignments
        .iter()
        .position(native_assignment_reads_contribution_current)
    else {
        return Ok((assignments, Vec::new()));
    };

    let post_assignments = assignments.split_off(split_index);
    let mut post_targets = vec![false; model.num_variables];
    mark_native_assignment_targets(&post_assignments, &mut post_targets);

    let mut pre_current_roots = vec![false; model.num_variables];
    mark_canonical_entry_variable_roots(model, mir, limits, false, &mut pre_current_roots)?;
    propagate_live_assignment_slots(model, &mut pre_current_roots);

    if let Some(slot) = post_targets
        .iter()
        .zip(&pre_current_roots)
        .position(|(post, required)| *post && *required)
    {
        let name = model
            .variable_names
            .get(slot)
            .map(SmolStr::as_str)
            .unwrap_or("<unnamed>");
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "assignment variable '{name}' (slot {slot}) depends on a contribution current but is required before contribution-current evaluation"
            )
            .into(),
        });
    }

    Ok((assignments, post_assignments))
}

fn native_assignment_reads_contribution_current(assignment: &NativeAssignment) -> bool {
    match assignment {
        NativeAssignment::Direct { program, .. } => {
            native_program_reads_contribution_current(program)
        }
        NativeAssignment::Indexed { index, value, .. } => {
            native_program_reads_contribution_current(index)
                || native_program_reads_contribution_current(value)
        }
        NativeAssignment::Loop { condition, body } => {
            native_program_reads_contribution_current(condition)
                || body
                    .iter()
                    .any(native_assignment_reads_contribution_current)
        }
    }
}

fn native_program_reads_contribution_current(program: &NativeProgram) -> bool {
    !program.current_pair_dependencies().is_empty()
        || !program.prior_current_dependencies().is_empty()
}

fn mark_native_assignment_targets(assignments: &[NativeAssignment], targets: &mut [bool]) {
    for assignment in assignments {
        match assignment {
            NativeAssignment::Direct { var_index, .. } => {
                if let Some(target) = targets.get_mut(*var_index) {
                    *target = true;
                }
            }
            NativeAssignment::Indexed { base, len, .. } => {
                let end = base.saturating_add(*len).min(targets.len());
                for target in targets.iter_mut().take(end).skip(*base) {
                    *target = true;
                }
            }
            NativeAssignment::Loop { body, .. } => {
                mark_native_assignment_targets(body, targets);
            }
        }
    }
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

fn live_native_assignment_steps(model: &CompiledModel) -> Vec<AssignmentStep> {
    let live = live_assignment_slots(model);
    filter_live_assignment_steps(&model.assignment_steps, &live)
}

fn live_assignment_slots(model: &CompiledModel) -> Vec<bool> {
    let mut live = native_assignment_roots(model);
    propagate_live_assignment_slots(model, &mut live);
    live
}

pub(crate) fn live_canonical_assignment_slots(
    model: &CompiledModel,
    mir: &MirModel,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<bool>> {
    let mut live = native_observable_assignment_roots(model);
    mark_canonical_entry_variable_roots(model, mir, limits, true, &mut live)?;
    propagate_live_assignment_slots(model, &mut live);
    Ok(live)
}

fn propagate_live_assignment_slots(model: &CompiledModel, live: &mut [bool]) {
    loop {
        let mut changed = false;
        propagate_assignment_liveness(&model.assignment_steps, live, &mut changed);
        if !changed {
            break;
        }
    }
}

struct AssignmentShadowIndex {
    scalar: HashMap<String, Vec<ScalarDerivativeShadow>>,
    arrays: HashMap<String, Vec<ArrayDerivativeShadow>>,
    malformed_arrays: HashMap<String, Vec<MalformedArrayDerivativeShadow>>,
}

#[derive(Default)]
struct AssignmentProgramCursor<'a> {
    scalar: HashMap<usize, Vec<&'a BytecodeProgram>>,
    scalar_next: HashMap<usize, usize>,
    indexed: HashMap<(usize, usize, i64), Vec<(&'a BytecodeProgram, &'a BytecodeProgram)>>,
    indexed_next: HashMap<(usize, usize, i64), usize>,
}

impl<'a> AssignmentProgramCursor<'a> {
    fn for_steps(steps: &'a [AssignmentStep]) -> Self {
        let mut cursor = Self::default();
        cursor.collect_steps(steps);
        cursor
    }

    fn collect_steps(&mut self, steps: &'a [AssignmentStep]) {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    self.scalar
                        .entry(assignment.var_index)
                        .or_default()
                        .push(&assignment.program);
                }
                AssignmentStep::AssignIndexed {
                    base,
                    len,
                    lower,
                    index,
                    value,
                } => {
                    self.indexed
                        .entry((*base, *len, *lower))
                        .or_default()
                        .push((index, value));
                }
                AssignmentStep::Loop { body, .. } => self.collect_steps(body),
            }
        }
    }

    fn next_scalar(&mut self, var_index: usize) -> Option<&'a BytecodeProgram> {
        let next = self.scalar_next.entry(var_index).or_default();
        let program = self
            .scalar
            .get(&var_index)
            .and_then(|programs| programs.get(*next))
            .copied();
        *next += usize::from(program.is_some());
        program
    }

    fn next_indexed(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
    ) -> Option<(&'a BytecodeProgram, &'a BytecodeProgram)> {
        let key = (base, len, lower);
        let next = self.indexed_next.entry(key).or_default();
        let programs = self
            .indexed
            .get(&key)
            .and_then(|programs| programs.get(*next))
            .copied();
        *next += usize::from(programs.is_some());
        programs
    }
}

struct ScalarDerivativeShadow {
    var_index: usize,
    name: SmolStr,
    axes: Vec<CanonicalDerivativeAxis>,
}

struct ArrayDerivativeShadow {
    suffix: String,
    axes: Vec<CanonicalDerivativeAxis>,
    base: usize,
    len: usize,
    lower: i64,
}

struct MalformedArrayDerivativeShadow {
    logical_index: i64,
    name: SmolStr,
}

struct ArrayDerivativeShadowAccumulator {
    axes: Vec<CanonicalDerivativeAxis>,
    slots: Vec<(i64, usize)>,
}

impl AssignmentShadowIndex {
    fn for_model(model: &CompiledModel) -> JitResult<Self> {
        let mut scalar: HashMap<String, Vec<ScalarDerivativeShadow>> = HashMap::new();
        let mut array_accumulators: HashMap<(String, String), ArrayDerivativeShadowAccumulator> =
            HashMap::new();
        let mut array_order: Vec<(String, String)> = Vec::new();
        let mut malformed_arrays: HashMap<String, Vec<MalformedArrayDerivativeShadow>> =
            HashMap::new();

        for (slot, name) in model.variable_names.iter().enumerate() {
            let name_str = name.as_str();
            if let Some((array_name, logical_index, suffix)) = parse_array_variable_name(name_str) {
                if let Some(raw_suffix) = suffix.strip_prefix('@') {
                    if let Some(axes) = derivative_shadow_axes_from_suffix(raw_suffix) {
                        let key = (array_name.to_string(), suffix.to_string());
                        if !array_accumulators.contains_key(&key) {
                            array_order.push(key.clone());
                        }
                        array_accumulators
                            .entry(key)
                            .or_insert_with(|| ArrayDerivativeShadowAccumulator {
                                axes,
                                slots: Vec::new(),
                            })
                            .slots
                            .push((logical_index, slot));
                    } else {
                        malformed_arrays
                            .entry(array_name.to_string())
                            .or_default()
                            .push(MalformedArrayDerivativeShadow {
                                logical_index,
                                name: name.clone(),
                            });
                    }
                }
                continue;
            }

            let Some((base, raw_suffix)) = name_str.split_once('@') else {
                continue;
            };
            if base.is_empty() || base.contains('[') {
                continue;
            }
            let Some(axes) = derivative_shadow_axes_from_suffix(raw_suffix) else {
                continue;
            };
            scalar
                .entry(base.to_string())
                .or_default()
                .push(ScalarDerivativeShadow {
                    var_index: slot,
                    name: name.clone(),
                    axes,
                });
        }

        let mut arrays: HashMap<String, Vec<ArrayDerivativeShadow>> = HashMap::new();
        for key in array_order {
            let Some(mut accumulator) = array_accumulators.remove(&key) else {
                continue;
            };
            accumulator.slots.sort_by_key(|(index, _)| *index);
            let Some((lower, base)) = accumulator.slots.first().copied() else {
                continue;
            };
            for (offset, (logical_index, slot)) in accumulator.slots.iter().enumerate() {
                let expected_index = checked_logical_index(model, key.0.as_str(), lower, offset)?;
                let expected_slot = base + offset;
                if *logical_index != expected_index || *slot != expected_slot {
                    return Err(JitError::InvalidCanonicalIr {
                        model: model.name.clone(),
                        detail: format!(
                            "canonical indexed assignment '{}' shadow suffix '{}' is not contiguous in compiled variable storage",
                            key.0, key.1
                        )
                        .into(),
                    });
                }
            }
            arrays
                .entry(key.0)
                .or_default()
                .push(ArrayDerivativeShadow {
                    suffix: key.1,
                    axes: accumulator.axes,
                    base,
                    len: accumulator.slots.len(),
                    lower,
                });
        }

        Ok(Self {
            scalar,
            arrays,
            malformed_arrays,
        })
    }

    fn scalar_shadows(&self, target_name: &str) -> &[ScalarDerivativeShadow] {
        self.scalar.get(target_name).map_or(&[], Vec::as_slice)
    }

    fn array_shadows(&self, array_name: &str) -> &[ArrayDerivativeShadow] {
        self.arrays.get(array_name).map_or(&[], Vec::as_slice)
    }

    fn malformed_array_shadows(&self, array_name: &str) -> &[MalformedArrayDerivativeShadow] {
        self.malformed_arrays
            .get(array_name)
            .map_or(&[], Vec::as_slice)
    }
}

fn parse_array_variable_name(name: &str) -> Option<(&str, i64, &str)> {
    let (array_name, remainder) = name.split_once('[')?;
    let (index, suffix) = remainder.split_once(']')?;
    Some((array_name, index.parse::<i64>().ok()?, suffix))
}

fn checked_logical_upper_bound(
    model: &CompiledModel,
    label: &str,
    lower: i64,
    len: usize,
) -> JitResult<i64> {
    let len = i64::try_from(len).map_err(|_| JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!("canonical indexed assignment '{label}' length {len} does not fit i64")
            .into(),
    })?;
    lower.checked_add(len).ok_or_else(|| JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!(
            "canonical indexed assignment '{label}' logical range {lower} plus length {len} overflows i64"
        )
        .into(),
    })
}

pub(crate) fn checked_logical_index(
    model: &CompiledModel,
    label: &str,
    lower: i64,
    offset: usize,
) -> JitResult<i64> {
    let offset = i64::try_from(offset).map_err(|_| JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!("canonical indexed assignment '{label}' offset {offset} does not fit i64")
            .into(),
    })?;
    lower
        .checked_add(offset)
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical indexed assignment '{label}' logical index {lower} plus offset {offset} overflows i64"
            )
            .into(),
        })
}

pub(crate) fn ranges_overlap(
    model: &CompiledModel,
    label: &str,
    left_lower: i64,
    left_len: usize,
    right_lower: i64,
    right_len: usize,
) -> JitResult<bool> {
    let left_upper = checked_logical_upper_bound(model, label, left_lower, left_len)?;
    let right_upper = checked_logical_upper_bound(model, label, right_lower, right_len)?;
    Ok(left_lower < right_upper && right_lower < left_upper)
}

fn lower_live_canonical_assignment_statements(
    model: &CompiledModel,
    hir: &HirModel,
    mir: &MirModel,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    let live = live_canonical_assignment_slots(model, mir, limits)?;
    let shadow_index = AssignmentShadowIndex::for_model(model)?;
    let mut program_cursor = AssignmentProgramCursor::for_steps(&model.assignment_steps);
    lower_canonical_assignment_statements(
        model,
        hir,
        mir,
        &hir.statements,
        &live,
        &shadow_index,
        &mut program_cursor,
        limits,
    )
}

fn lower_canonical_assignment_statements(
    model: &CompiledModel,
    hir: &HirModel,
    mir: &MirModel,
    statements: &[HirStatement],
    live: &[bool],
    shadow_index: &AssignmentShadowIndex,
    program_cursor: &mut AssignmentProgramCursor<'_>,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    let mut assignments = Vec::new();
    for statement in statements {
        assignments.extend(lower_canonical_assignment_statement(
            model,
            hir,
            mir,
            statement,
            live,
            shadow_index,
            program_cursor,
            limits,
        )?);
    }
    Ok(assignments)
}

fn lower_canonical_assignment_statement(
    model: &CompiledModel,
    hir: &HirModel,
    mir: &MirModel,
    statement: &HirStatement,
    live: &[bool],
    shadow_index: &AssignmentShadowIndex,
    program_cursor: &mut AssignmentProgramCursor<'_>,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    match statement {
        HirStatement::Assignment(assignment) => lower_canonical_assignment(
            model,
            hir,
            mir,
            assignment,
            live,
            shadow_index,
            program_cursor,
            limits,
        ),
        HirStatement::Loop(loop_statement) => lower_canonical_assignment_loop(
            model,
            hir,
            mir,
            loop_statement,
            live,
            shadow_index,
            program_cursor,
            limits,
        ),
    }
}

fn lower_canonical_assignment(
    model: &CompiledModel,
    hir: &HirModel,
    mir: &MirModel,
    assignment: &HirAssignment,
    live: &[bool],
    shadow_index: &AssignmentShadowIndex,
    program_cursor: &mut AssignmentProgramCursor<'_>,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    if let Some(index) = &assignment.index {
        return lower_canonical_indexed_assignment(
            model,
            hir,
            mir,
            assignment,
            index.id,
            live,
            shadow_index,
            program_cursor,
            limits,
        );
    }

    let var_index = validate_canonical_scalar_assignment_target(model, assignment)?;
    let bytecode_program = program_cursor.next_scalar(var_index);
    let mut assignments = canonical_scalar_shadow_assignments(
        model,
        mir,
        assignment.target_name.as_str(),
        assignment.expr.id,
        bytecode_program,
        live,
        shadow_index,
        limits,
    )?;
    if live.get(var_index).copied().unwrap_or(false) {
        let bytecode_program = bytecode_program.ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical assignment '{}' has no matching compiled assignment program",
                assignment.target_name
            )
            .into(),
        })?;
        let program = lower_canonical_assignment_expression_program(
            model,
            mir,
            assignment.expr.id,
            bytecode_program,
            limits,
        )?;
        trace_assignment_program_stack(model, assignment.target_name.as_str(), &program);
        assignments.push(NativeAssignment::Direct { var_index, program });
    }
    Ok(assignments)
}

fn lower_canonical_indexed_assignment(
    model: &CompiledModel,
    hir: &HirModel,
    mir: &MirModel,
    assignment: &HirAssignment,
    index_expr: ExprId,
    live: &[bool],
    shadow_index: &AssignmentShadowIndex,
    program_cursor: &mut AssignmentProgramCursor<'_>,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    let (base, len, lower) = canonical_assignment_array_range(model, hir, assignment)?;
    let bytecode_programs = program_cursor.next_indexed(base, len, lower);
    let mut assignments = canonical_array_shadow_assignments(
        model,
        mir,
        assignment.target_name.as_str(),
        base,
        len,
        lower,
        index_expr,
        assignment.expr.id,
        bytecode_programs,
        live,
        shadow_index,
        limits,
    )?;
    if !assignment_range_live(base, len, live) {
        return Ok(assignments);
    }
    let (index_program, value_program) =
        bytecode_programs.ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical indexed assignment '{}' has no matching compiled assignment program",
                assignment.target_name
            )
            .into(),
        })?;
    let index = lower_canonical_assignment_expression_program(
        model,
        mir,
        index_expr,
        index_program,
        limits,
    )?;
    let value = lower_canonical_assignment_expression_program(
        model,
        mir,
        assignment.expr.id,
        value_program,
        limits,
    )?;
    trace_assignment_program_stack(model, assignment.target_name.as_str(), &value);
    if let Some(var_index) = constant_indexed_assignment_slot(&index, base, len, lower) {
        validate_assignment_target(model, var_index)?;
        assignments.push(NativeAssignment::Direct {
            var_index,
            program: value,
        });
        return Ok(assignments);
    }
    assignments.push(NativeAssignment::Indexed {
        base,
        len,
        lower,
        index,
        value,
    });
    Ok(assignments)
}

fn lower_canonical_assignment_loop(
    model: &CompiledModel,
    hir: &HirModel,
    mir: &MirModel,
    loop_statement: &HirLoop,
    live: &[bool],
    shadow_index: &AssignmentShadowIndex,
    program_cursor: &mut AssignmentProgramCursor<'_>,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    let body = lower_canonical_assignment_statements(
        model,
        hir,
        mir,
        &loop_statement.body,
        live,
        shadow_index,
        program_cursor,
        limits,
    )?;
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let condition = NativeProgram::from_mir_expression(
        model.name.clone(),
        EntryKind::Assignment,
        mir,
        loop_statement.condition.id,
        limits,
    )?;
    Ok(vec![NativeAssignment::Loop { condition, body }])
}

fn canonical_scalar_shadow_assignments(
    model: &CompiledModel,
    mir: &MirModel,
    target_name: &str,
    expr_id: ExprId,
    bytecode_program: Option<&BytecodeProgram>,
    live: &[bool],
    shadow_index: &AssignmentShadowIndex,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    let mut assignments = Vec::new();
    for shadow in shadow_index.scalar_shadows(target_name) {
        if !live.get(shadow.var_index).copied().unwrap_or(false) {
            continue;
        }
        validate_assignment_target(model, shadow.var_index)?;
        let bytecode_program = bytecode_program.ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical assignment '{target_name}' derivative shadow '{}' has no matching compiled assignment program",
                shadow.name
            )
            .into(),
        })?;
        let program = lower_canonical_shadow_program(
            model,
            mir,
            expr_id,
            bytecode_program,
            &shadow.axes,
            limits,
        )?;
        trace_assignment_program_stack(model, shadow.name.as_str(), &program);
        assignments.push(NativeAssignment::Direct {
            var_index: shadow.var_index,
            program,
        });
    }
    Ok(assignments)
}

fn canonical_array_shadow_assignments(
    model: &CompiledModel,
    mir: &MirModel,
    array_name: &str,
    source_base: usize,
    source_len: usize,
    source_lower: i64,
    index_expr: ExprId,
    value_expr: ExprId,
    bytecode_programs: Option<(&BytecodeProgram, &BytecodeProgram)>,
    live: &[bool],
    shadow_index: &AssignmentShadowIndex,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    let mut assignments = Vec::new();
    let source_upper = checked_logical_upper_bound(model, array_name, source_lower, source_len)?;
    for malformed in shadow_index.malformed_array_shadows(array_name) {
        if malformed.logical_index >= source_lower && malformed.logical_index < source_upper {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical indexed assignment '{array_name}' has malformed derivative shadow '{}'",
                    malformed.name
                )
                .into(),
            });
        }
    }
    for shadow in shadow_index.array_shadows(array_name) {
        if !ranges_overlap(
            model,
            array_name,
            shadow.lower,
            shadow.len,
            source_lower,
            source_len,
        )? {
            continue;
        }
        if shadow.len != source_len || shadow.lower != source_lower {
            let shadow_upper =
                checked_logical_upper_bound(model, array_name, shadow.lower, shadow.len)?;
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical indexed assignment '{array_name}' shadow suffix '{}' range {}..{} does not match source range {source_lower}..{}",
                    shadow.suffix,
                    shadow.lower,
                    shadow_upper,
                    source_upper
                )
                .into(),
            });
        }
        if shadow.base == source_base {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical indexed assignment '{array_name}' shadow suffix '{}' aliases source array storage",
                    shadow.suffix
                )
                .into(),
            });
        }
        if !assignment_range_live(shadow.base, shadow.len, live) {
            continue;
        }

        let (index_program, value_program) =
            bytecode_programs.ok_or_else(|| JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical indexed assignment '{array_name}' derivative shadow '{}' has no matching compiled assignment program",
                    shadow.suffix
                )
                .into(),
            })?;
        let index = lower_canonical_assignment_expression_program(
            model,
            mir,
            index_expr,
            index_program,
            limits,
        )?;
        let value = lower_canonical_shadow_program(
            model,
            mir,
            value_expr,
            value_program,
            &shadow.axes,
            limits,
        )?;
        trace_assignment_program_stack(model, &format!("{array_name}{}", shadow.suffix), &value);
        if let Some(var_index) =
            constant_indexed_assignment_slot(&index, shadow.base, shadow.len, shadow.lower)
        {
            validate_assignment_target(model, var_index)?;
            assignments.push(NativeAssignment::Direct {
                var_index,
                program: value,
            });
        } else {
            assignments.push(NativeAssignment::Indexed {
                base: shadow.base,
                len: shadow.len,
                lower: shadow.lower,
                index,
                value,
            });
        }
    }
    Ok(assignments)
}

fn lower_canonical_assignment_expression_program(
    model: &CompiledModel,
    mir: &MirModel,
    expr_id: ExprId,
    bytecode_program: &BytecodeProgram,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    let slots =
        CanonicalExpressionStateSlots::for_expression(model, mir, expr_id, bytecode_program)?;
    NativeProgram::from_mir_expression(
        model.name.clone(),
        EntryKind::Assignment,
        mir,
        expr_id,
        slots.apply(limits),
    )
}

#[derive(Default)]
struct CanonicalExpressionStateSlots {
    ddt: Vec<(ExprId, usize)>,
    idt: Vec<(ExprId, usize)>,
    idtmod: Vec<(ExprId, usize)>,
    transition: Vec<(ExprId, usize)>,
    slew: Vec<(ExprId, usize)>,
    absdelay: Vec<(ExprId, usize)>,
    laplace: Vec<(ExprId, usize)>,
    zi: Vec<(ExprId, usize)>,
    cross: Vec<(ExprId, usize)>,
    above: Vec<(ExprId, usize)>,
    timer: Vec<(ExprId, usize)>,
    limit: Vec<(ExprId, usize)>,
    table_lookup: Vec<(ExprId, usize)>,
}

impl CanonicalExpressionStateSlots {
    const OPERATORS: [CanonicalStateOperator; 13] = [
        CanonicalStateOperator::Ddt,
        CanonicalStateOperator::Idt,
        CanonicalStateOperator::IdtMod,
        CanonicalStateOperator::Transition,
        CanonicalStateOperator::Slew,
        CanonicalStateOperator::Absdelay,
        CanonicalStateOperator::Laplace,
        CanonicalStateOperator::Zi,
        CanonicalStateOperator::Cross,
        CanonicalStateOperator::Above,
        CanonicalStateOperator::Timer,
        CanonicalStateOperator::Limit,
        CanonicalStateOperator::TableLookup,
    ];

    fn for_equation(
        model: &CompiledModel,
        mir: &MirModel,
        equation_id: EquationId,
        bytecode_program: &BytecodeProgram,
    ) -> JitResult<Self> {
        let equation = mir.equations.get(usize::from(equation_id)).ok_or_else(|| {
            JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!("canonical equation {equation_id} is outside MIR equation arena")
                    .into(),
            }
        })?;
        Self::for_expression(model, mir, equation.expression.id, bytecode_program)
    }

    fn for_expression(
        model: &CompiledModel,
        mir: &MirModel,
        expr_id: ExprId,
        bytecode_program: &BytecodeProgram,
    ) -> JitResult<Self> {
        if !bytecode_program.instructions.iter().any(|instruction| {
            Self::OPERATORS
                .iter()
                .any(|operator| operator.bytecode_slot(instruction).is_some())
        }) {
            return Ok(Self::default());
        }
        let collect = |operator| {
            canonical_state_slots_for_expression(
                model.name.clone(),
                mir,
                expr_id,
                bytecode_program,
                operator,
            )
        };
        Ok(Self {
            ddt: collect(CanonicalStateOperator::Ddt)?,
            idt: collect(CanonicalStateOperator::Idt)?,
            idtmod: collect(CanonicalStateOperator::IdtMod)?,
            transition: collect(CanonicalStateOperator::Transition)?,
            slew: collect(CanonicalStateOperator::Slew)?,
            absdelay: collect(CanonicalStateOperator::Absdelay)?,
            laplace: collect(CanonicalStateOperator::Laplace)?,
            zi: collect(CanonicalStateOperator::Zi)?,
            cross: collect(CanonicalStateOperator::Cross)?,
            above: collect(CanonicalStateOperator::Above)?,
            timer: collect(CanonicalStateOperator::Timer)?,
            limit: collect(CanonicalStateOperator::Limit)?,
            table_lookup: collect(CanonicalStateOperator::TableLookup)?,
        })
    }

    fn apply<'a>(&'a self, limits: NativeLoweringLimits<'a>) -> NativeLoweringLimits<'a> {
        limits
            .with_canonical_ddt_slots(&self.ddt)
            .with_canonical_idt_slots(&self.idt)
            .with_canonical_idtmod_slots(&self.idtmod)
            .with_canonical_transition_slots(&self.transition)
            .with_canonical_slew_slots(&self.slew)
            .with_canonical_absdelay_slots(&self.absdelay)
            .with_canonical_laplace_slots(&self.laplace)
            .with_canonical_zi_slots(&self.zi)
            .with_canonical_cross_slots(&self.cross)
            .with_canonical_above_slots(&self.above)
            .with_canonical_timer_slots(&self.timer)
            .with_canonical_limit_slots(&self.limit)
            .with_canonical_table_lookup_slots(&self.table_lookup)
    }
}

fn trace_assignment_program_stack(model: &CompiledModel, label: &str, program: &NativeProgram) {
    if std::env::var_os("RSPICE_NATIVE_TRACE_ASSIGNMENT_STACK").is_some()
        && program.max_stack_depth() > 16
    {
        let mut depth = 0usize;
        let mut max_depth = 0usize;
        let mut max_index = 0usize;
        for (index, op) in program.ops().iter().enumerate() {
            let (pop, push) = native_op_stack_effect(op);
            depth = depth.saturating_sub(pop) + push;
            if depth > max_depth {
                max_depth = depth;
                max_index = index;
            }
        }
        let start = max_index.saturating_sub(24);
        let end = (max_index + 24).min(program.ops().len());
        eprintln!(
            "native-assignment-stack model={} target={} depth={} max_index={} window={:?}",
            model.name,
            label,
            program.max_stack_depth(),
            max_index,
            &program.ops()[start..end]
        );
    }
}

fn lower_canonical_shadow_program(
    model: &CompiledModel,
    mir: &MirModel,
    expr_id: ExprId,
    bytecode_program: &BytecodeProgram,
    axes: &[CanonicalDerivativeAxis],
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeProgram> {
    let slots =
        CanonicalExpressionStateSlots::for_expression(model, mir, expr_id, bytecode_program)?;
    let limits = slots.apply(limits);
    match axes {
        [axis] => NativeProgram::from_mir_expression_derivative(
            model.name.clone(),
            EntryKind::Assignment,
            mir,
            EquationId::new(0),
            expr_id,
            *axis,
            limits,
        ),
        [first, second] => NativeProgram::from_mir_expression_second_derivative(
            model.name.clone(),
            EntryKind::Assignment,
            mir,
            EquationId::new(0),
            expr_id,
            *first,
            *second,
            limits,
        ),
        [first, second, third] => NativeProgram::from_mir_expression_third_derivative(
            model.name.clone(),
            EntryKind::Assignment,
            mir,
            EquationId::new(0),
            expr_id,
            *first,
            *second,
            *third,
            limits,
        ),
        _ => Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical assignment derivative shadow has unsupported order {}",
                axes.len()
            )
            .into(),
        }),
    }
}

pub(crate) fn derivative_shadow_axes_from_suffix(
    suffix: &str,
) -> Option<Vec<CanonicalDerivativeAxis>> {
    let mut axes = Vec::new();
    for part in suffix.split('@') {
        axes.push(derivative_shadow_axis(part)?);
    }
    (!axes.is_empty()).then_some(axes)
}

fn derivative_shadow_axis(part: &str) -> Option<CanonicalDerivativeAxis> {
    if let Some(index) = part.strip_prefix("dI") {
        return index
            .parse::<usize>()
            .ok()
            .map(CanonicalDerivativeAxis::Branch);
    }
    let index = part.strip_prefix('d')?.parse::<usize>().ok()?;
    Some(CanonicalDerivativeAxis::Node(NodeId::from(index)))
}

fn validate_canonical_scalar_assignment_target(
    model: &CompiledModel,
    assignment: &HirAssignment,
) -> JitResult<usize> {
    let var_index = usize::from(assignment.target);
    validate_assignment_target(model, var_index)?;
    let Some(name) = model.variable_names.get(var_index) else {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical assignment target {var_index} has no compiled variable name"
            )
            .into(),
        });
    };
    if name != &assignment.target_name {
        return Err(JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical assignment target '{}' at slot {var_index} does not match compiled variable '{}'",
                assignment.target_name, name
            )
            .into(),
        });
    }
    Ok(var_index)
}

fn canonical_assignment_array_range(
    model: &CompiledModel,
    hir: &HirModel,
    assignment: &HirAssignment,
) -> JitResult<(usize, usize, i64)> {
    let array = hir
        .arrays
        .iter()
        .find(|array| array.base == assignment.target && array.name == assignment.target_name)
        .ok_or_else(|| JitError::InvalidCanonicalIr {
            model: model.name.clone(),
            detail: format!(
                "canonical indexed assignment '{}' has no matching array metadata",
                assignment.target_name
            )
            .into(),
        })?;
    let base = usize::from(array.base);
    let len = usize::try_from(array.len).map_err(|_| JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!(
            "canonical indexed assignment '{}' length {} does not fit usize",
            array.name, array.len
        )
        .into(),
    })?;
    let lower = array.lower;
    let _upper = checked_logical_upper_bound(model, array.name.as_str(), lower, len)?;
    super::coverage::validate_assignment_range(model, base, len)?;
    for offset in 0..len {
        let slot = base + offset;
        let logical_index = checked_logical_index(model, array.name.as_str(), lower, offset)?;
        let expected = format!("{}[{logical_index}]", array.name);
        let Some(actual) = model.variable_names.get(slot) else {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical indexed assignment '{}' slot {slot} has no compiled variable name",
                    array.name
                )
                .into(),
            });
        };
        if actual.as_str() != expected {
            return Err(JitError::InvalidCanonicalIr {
                model: model.name.clone(),
                detail: format!(
                    "canonical indexed assignment '{}' slot {slot} expected compiled variable '{expected}', found '{actual}'",
                    array.name
                )
                .into(),
            });
        }
    }
    Ok((base, len, lower))
}

pub(crate) fn native_assignment_roots(model: &CompiledModel) -> Vec<bool> {
    let mut live = native_observable_assignment_roots(model);
    mark_bytecode_entry_variable_roots(model, &mut live);
    live
}

fn native_observable_assignment_roots(model: &CompiledModel) -> Vec<bool> {
    let mut live = vec![false; model.num_variables];
    for (index, name) in model.variable_names.iter().enumerate().take(live.len()) {
        if native_assignment_root_is_externally_observable(name.as_str()) {
            live[index] = true;
        }
    }
    live
}

fn mark_bytecode_entry_variable_roots(model: &CompiledModel, live: &mut [bool]) {
    for stamp in &model.stamp_programs {
        if let Some(condition) = &stamp.static_condition {
            mark_program_variable_reads(condition, live);
        }
        mark_program_variable_reads(&stamp.value_program, live);
        for jacobian in &stamp.jacobian_programs {
            mark_program_variable_reads(&jacobian.program, live);
        }
        for jacobian in &stamp.reactive_jacobians {
            mark_program_variable_reads(&jacobian.program, live);
        }
    }
    for source in &model.noise_sources {
        mark_program_variable_reads(&source.psd_program, live);
        if let Some(program) = &source.exponent_program {
            mark_program_variable_reads(program, live);
        }
    }
}

fn mark_canonical_entry_variable_roots(
    model: &CompiledModel,
    mir: &MirModel,
    limits: NativeLoweringLimits<'_>,
    include_noise: bool,
    live: &mut [bool],
) -> JitResult<()> {
    let canonical_noise_plan = if model.noise_sources.is_empty() {
        None
    } else {
        Some(build_canonical_noise_plan(model, mir)?)
    };
    let mut available_current_pairs = Vec::new();
    let mut prior_current_probes = Vec::new();

    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        if let Some(condition) = &stamp.static_condition {
            let program =
                lower_static_condition_program(model, Some(mir), stamp_index, condition, limits)?;
            mark_native_program_variable_reads(&program, live);
        }

        let value_limits = limits
            .with_available_current_pairs(&available_current_pairs)
            .with_prior_current_probes(&prior_current_probes);
        let program = lower_stamp_value_program(
            model,
            Some(mir),
            stamp_index,
            &stamp.value_program,
            value_limits,
        )?;
        mark_native_program_variable_reads(&program, live);

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
        let jacobian_limits = limits
            .with_available_current_pairs(&jacobian_current_pairs)
            .with_prior_current_probes(&jacobian_prior_current_probes);
        let jacobian_table_lookup_slots = canonical_table_lookup_slots_for_equation(
            model.name.clone(),
            mir,
            canonical_equation_id(model, stamp_index)?,
            &stamp.value_program,
        )?;

        for jacobian in &stamp.jacobian_programs {
            let program = lower_jacobian_program(
                model,
                Some(mir),
                stamp_index,
                jacobian,
                jacobian_limits,
                &jacobian_table_lookup_slots,
            )?;
            mark_native_program_variable_reads(&program, live);
        }

        let reactive_mir = (!stamp.reactive_jacobians.is_empty())
            .then(|| canonical_reactive_mir(model, mir, canonical_equation_id(model, stamp_index)?))
            .transpose()?;
        for reactive_jacobian in &stamp.reactive_jacobians {
            let program = lower_reactive_jacobian_program(
                model,
                reactive_mir.as_ref(),
                stamp_index,
                reactive_jacobian,
                limits,
            )?;
            mark_native_program_variable_reads(&program, live);
        }

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

    if include_noise {
        let noise_limits = limits
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
            mark_native_program_variable_reads(&psd_program, live);

            if let Some(program) = &source.exponent_program {
                let exponent_program = lower_noise_exponent_program(
                    model,
                    canonical_noise_plan.as_ref(),
                    source_index,
                    source,
                    program,
                    noise_limits,
                )?;
                mark_native_program_variable_reads(&exponent_program, live);
            }
        }
    }

    Ok(())
}

fn mark_native_program_variable_reads(program: &NativeProgram, live: &mut [bool]) {
    for op in program.ops() {
        match op {
            NativeOp::LoadVariable(index) => {
                if *index < live.len() {
                    live[*index] = true;
                }
            }
            NativeOp::LoadVariableDyn { base, len, .. } => {
                let end = base.saturating_add(*len).min(live.len());
                for slot in live.iter_mut().take(end).skip(*base) {
                    *slot = true;
                }
            }
            _ => {}
        }
    }
}

fn native_assignment_root_is_externally_observable(name: &str) -> bool {
    !name.contains('@') && !name.starts_with("__guard")
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

#[cfg(all(test, target_arch = "x86_64"))]
#[cfg(feature = "native")]
pub(crate) fn lower_assignment_step(
    model: &CompiledModel,
    step: &AssignmentStep,
) -> JitResult<NativeAssignment> {
    lower_assignment_step_with_limits(model, step, NativeLoweringLimits::for_model(model))
}

fn lower_assignment_step_with_limits(
    model: &CompiledModel,
    step: &AssignmentStep,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<NativeAssignment> {
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
                .map(|step| lower_assignment_step_with_limits(model, step, limits))
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

pub(crate) fn infer_current_terminal_pair(program: &StampProgram) -> Option<(usize, usize)> {
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

pub(crate) fn infer_current_unified_pair(
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

pub(crate) fn push_prior_current_probe_aliases(
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

fn push_completed_current_probe_aliases(
    probes: &mut Vec<PriorCurrentProbe>,
    current_index: usize,
    pos: usize,
    neg: usize,
    terminal_count: usize,
) {
    push_prior_current_probe_aliases(probes, current_index, pos, neg);
    if pos < terminal_count {
        push_prior_current_probe_alias(
            probes,
            PriorCurrentProbe {
                pos,
                neg: CURRENT_PAIR_GROUND,
                current_index,
                inverted: false,
            },
        );
        push_prior_current_probe_alias(
            probes,
            PriorCurrentProbe {
                pos: CURRENT_PAIR_GROUND,
                neg: pos,
                current_index,
                inverted: true,
            },
        );
    }
    if neg < terminal_count {
        push_prior_current_probe_alias(
            probes,
            PriorCurrentProbe {
                pos: neg,
                neg: CURRENT_PAIR_GROUND,
                current_index,
                inverted: true,
            },
        );
        push_prior_current_probe_alias(
            probes,
            PriorCurrentProbe {
                pos: CURRENT_PAIR_GROUND,
                neg,
                current_index,
                inverted: false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompilerOptions, VerilogACompiler};

    /// Build a plan the way both machine backends and the browser emitter do.
    fn plan(source: &str, module: &str) -> (CompiledModel, NativeModelPlan) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let report = compiler
            .compile_runtime(source, Some(module))
            .expect("compile runtime artifacts");
        let plan = build_model_plan_with_canonical_ir(&report.model, &report.canonical_ir)
            .expect("build canonical model plan");
        (report.model, plan)
    }

    const RESISTOR: &str = r#"
`include "disciplines.vams"
module plan_resistor(p, n);
  inout p, n;
  electrical p, n;
  parameter real resistance = 2.0 from (0:inf);
  real voltage;
  analog begin
    voltage = V(p, n);
    I(p, n) <+ voltage / resistance;
  end
endmodule
"#;

    /// Everything downstream trusts the plan's shape, so it is validated
    /// against the compiled model it was derived from rather than assumed.
    #[test]
    fn canonical_plan_shape_matches_its_compiled_model() {
        let (model, plan) = plan(RESISTOR, "plan_resistor");
        plan.validate_shape(&model)
            .expect("planned shape must match the compiled model");
        assert_eq!(plan.stamp_values.len(), model.stamp_programs.len());
        assert_eq!(plan.parameter_defaults.len(), model.parameters.len());
        assert_eq!(
            plan.published_current_pairs.len(),
            model.stamp_programs.len()
        );
        assert!(!plan.assignments.is_empty(), "the model assigns a variable");
    }

    /// The bytecode lane must produce the same shape as the canonical lane,
    /// because the contract tests compile through it and the production path
    /// does not.
    #[cfg(feature = "native-bytecode-contract-tests")]
    #[test]
    fn bytecode_and_canonical_lanes_agree_on_plan_shape() {
        let (model, canonical) = plan(RESISTOR, "plan_resistor");
        let bytecode = build_model_plan_from_bytecode(&model).expect("build bytecode plan");
        bytecode
            .validate_shape(&model)
            .expect("bytecode plan shape must match the compiled model");
        assert_eq!(bytecode.stamp_values.len(), canonical.stamp_values.len());
        assert_eq!(bytecode.jacobians.len(), canonical.jacobians.len());
        assert_eq!(
            bytecode.published_current_pairs,
            canonical.published_current_pairs
        );
    }

    /// A model whose expressions read no earlier contribution must be eligible
    /// to fuse. Both the machine backends and the browser emitter gate their
    /// whole-model drivers on this, so a regression here silently costs the
    /// browser one JavaScript round trip per scalar.
    #[test]
    fn a_model_without_prior_current_reads_is_fusion_eligible() {
        let (_, plan) = plan(RESISTOR, "plan_resistor");
        assert!(plan.current_dependencies.evaluation_kernel_order_safe());
        assert!(plan.current_dependencies.stamp_kernel_order_safe());
    }

    #[test]
    fn ddt_and_noise_intrinsics_are_recognized_by_exact_name() {
        assert!(canonical_is_ddt_call("ddt"));
        assert!(!canonical_is_ddt_call("ddx"));
        assert!(!canonical_is_ddt_call("ddt_"));
        assert!(canonical_noise_intrinsic_kind("white_noise").is_some());
        assert!(canonical_noise_intrinsic_kind("flicker_noise").is_some());
        assert!(canonical_noise_intrinsic_kind("noise_table").is_some());
        assert!(canonical_noise_intrinsic_kind("white_noise_").is_none());
        assert!(canonical_noise_intrinsic_kind("exp").is_none());
    }

    #[test]
    fn derivative_shadow_axes_reject_malformed_suffixes() {
        assert!(derivative_shadow_axes_from_suffix("").is_none());
        assert!(derivative_shadow_axes_from_suffix("@").is_none());
        assert!(derivative_shadow_axes_from_suffix("not_an_axis").is_none());
    }

    #[test]
    fn logical_index_and_range_overlap_reject_out_of_range_arrays() {
        let (model, _) = plan(RESISTOR, "plan_resistor");
        assert_eq!(
            checked_logical_index(&model, "array", 0, 3).expect("in-range offset"),
            3
        );
        assert!(
            checked_logical_index(&model, "array", i64::MAX, 1).is_err(),
            "an offset that overflows the logical index space must be rejected"
        );

        // Ranges are half-open, so touching endpoints do not overlap.
        assert!(!ranges_overlap(&model, "array", 0, 2, 2, 2).expect("disjoint ranges"));
        assert!(ranges_overlap(&model, "array", 0, 3, 2, 2).expect("overlapping ranges"));
        assert!(!ranges_overlap(&model, "array", 5, 2, 0, 2).expect("disjoint reversed ranges"));
    }
}
