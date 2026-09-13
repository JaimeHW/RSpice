//! Match the portable AD staging layout to canonical assignment expressions.
//!
//! The compiled statements identify temporary storage and publication order;
//! MIR remains the source of every computed value and derivative. Only the
//! final single-variable copies lower directly from their transport program.

use super::*;

fn invalid(model: &CompiledModel, detail: impl Into<String>) -> JitError {
    JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: format!("canonical ddx staging: {}", detail.into()).into(),
    }
}

fn copied_slot(program: &BytecodeProgram, slots: &HashSet<usize>) -> Option<usize> {
    match program.instructions.as_slice() {
        [Instruction::PushVariable(slot)] if slots.contains(slot) => Some(*slot),
        _ => None,
    }
}

pub(super) fn prefix_start(
    model: &CompiledModel,
    program: &BytecodeProgram,
    slots: &HashSet<usize>,
    positions: &HashMap<usize, usize>,
) -> JitResult<Option<usize>> {
    copied_slot(program, slots)
        .map(|slot| {
            positions.get(&slot).copied().ok_or_else(|| {
                invalid(
                    model,
                    format!("temporary {slot} has no preceding definition in this block"),
                )
            })
        })
        .transpose()
}

pub(super) fn validate_complete(
    model: &CompiledModel,
    live: &[bool],
    cursor: &AssignmentProgramCursor<'_>,
) -> JitResult<()> {
    for (slot, name) in model.variable_names.iter().enumerate() {
        if cursor.staging_slots.contains(&slot)
            && live.get(slot).copied().unwrap_or(false)
            && !cursor.emitted_staging.contains(&slot)
        {
            return Err(invalid(
                model,
                format!("live temporary '{name}' has no canonical assignment owner"),
            ));
        }
    }
    Ok(())
}

#[derive(Clone, PartialEq, Eq)]
enum ExpressionRole {
    Index,
    Value(Vec<CanonicalDerivativeAxis>),
}

fn assign_role(
    model: &CompiledModel,
    roles: &mut HashMap<usize, ExpressionRole>,
    slot: usize,
    role: ExpressionRole,
) -> JitResult<()> {
    if let Some(previous) = roles.insert(slot, role.clone())
        && previous != role
    {
        return Err(invalid(
            model,
            format!("temporary {slot} represents different expressions"),
        ));
    }
    Ok(())
}

fn value_role(
    model: &CompiledModel,
    hir: &HirModel,
    assignment: &HirAssignment,
    step: &AssignmentStep,
    shadows: &AssignmentShadowIndex,
) -> JitResult<ExpressionRole> {
    let axes = match step {
        AssignmentStep::Assign(write) if assignment.index.is_none() => {
            let primal = validate_canonical_scalar_assignment_target(model, assignment)?;
            validate_assignment_target(model, write.var_index)?;
            if write.var_index == primal {
                Vec::new()
            } else {
                let name = &model.variable_names[write.var_index];
                let prefix = format!("{}@", assignment.target_name);
                name.strip_prefix(prefix.as_str())
                    .and_then(derivative_shadow_axes_from_suffix)
                    .ok_or_else(|| {
                        invalid(
                            model,
                            format!("'{name}' is not a shadow of '{}'", assignment.target_name),
                        )
                    })?
            }
        }
        AssignmentStep::AssignIndexed {
            base, len, lower, ..
        } if assignment.index.is_some() => {
            let primal = canonical_assignment_array_range(model, hir, assignment)?;
            if (*base, *len, *lower) == primal {
                Vec::new()
            } else {
                shadows
                    .array_shadows(assignment.target_name.as_str())
                    .iter()
                    .find(|shadow| (shadow.base, shadow.len, shadow.lower) == (*base, *len, *lower))
                    .map(|shadow| shadow.axes.clone())
                    .ok_or_else(|| {
                        invalid(
                            model,
                            format!(
                                "array range {base}/{len}/{lower} is not a shadow of '{}'",
                                assignment.target_name
                            ),
                        )
                    })?
            }
        }
        _ => {
            return Err(invalid(
                model,
                "staged write does not match its canonical target",
            ));
        }
    };
    Ok(ExpressionRole::Value(axes))
}

fn programs(step: &AssignmentStep) -> Option<(Option<&BytecodeProgram>, &BytecodeProgram)> {
    match step {
        AssignmentStep::Assign(write) => Some((None, &write.program)),
        AssignmentStep::AssignIndexed { index, value, .. } => Some((Some(index), value)),
        _ => None,
    }
}

pub(super) fn lower(
    model: &CompiledModel,
    hir: &HirModel,
    mir: &MirModel,
    assignment: &HirAssignment,
    steps: &[AssignmentStep],
    live: &[bool],
    shadows: &AssignmentShadowIndex,
    cursor: &mut AssignmentProgramCursor<'_>,
    limits: NativeLoweringLimits<'_>,
) -> JitResult<Vec<NativeAssignment>> {
    let mut definitions = HashMap::new();
    let mut roles = HashMap::new();
    let mut publishing = false;
    for step in steps {
        if let AssignmentStep::Assign(write) = step
            && cursor.staging_slots.contains(&write.var_index)
        {
            if publishing {
                return Err(invalid(
                    model,
                    "temporary is computed after publication begins",
                ));
            }
            if definitions
                .insert(write.var_index, &write.program)
                .is_some()
            {
                return Err(invalid(
                    model,
                    format!("temporary {} is defined twice", write.var_index),
                ));
            }
            continue;
        }
        publishing = true;
        let (index, value) = programs(step).ok_or_else(|| {
            invalid(
                model,
                "staging group crosses a task, loop or initialization boundary",
            )
        })?;
        let role = value_role(model, hir, assignment, step, shadows)?;
        if let Some(slot) = copied_slot(value, &cursor.staging_slots) {
            assign_role(model, &mut roles, slot, role)?;
        }
        if let Some(slot) = index.and_then(|program| copied_slot(program, &cursor.staging_slots)) {
            assign_role(model, &mut roles, slot, ExpressionRole::Index)?;
        }
    }
    for slot in definitions.keys() {
        if !roles.contains_key(slot) {
            return Err(invalid(
                model,
                format!("temporary {slot} has no publication"),
            ));
        }
    }
    for slot in roles.keys() {
        if !definitions.contains_key(slot) {
            return Err(invalid(
                model,
                format!("copy references undefined temporary {slot}"),
            ));
        }
    }
    let primal = steps
        .last()
        .ok_or_else(|| invalid(model, "empty staging group"))?;
    if value_role(model, hir, assignment, primal, shadows)? != ExpressionRole::Value(Vec::new()) {
        return Err(invalid(
            model,
            "staging group does not end with its primal write",
        ));
    }
    let (raw_index, raw_value) = programs(primal).unwrap();
    let resolve = |program| match copied_slot(program, &cursor.staging_slots) {
        Some(slot) => definitions.get(&slot).copied().ok_or_else(|| {
            invalid(
                model,
                format!("temporary {slot} is outside its assignment group"),
            )
        }),
        None => Ok(program),
    };
    let source_value = resolve(raw_value)?;
    let source_index = raw_index.map(resolve).transpose()?;
    let lower_expression = |role: &ExpressionRole| -> JitResult<NativeProgram> {
        match role {
            ExpressionRole::Index => {
                let expression = assignment
                    .index
                    .as_ref()
                    .ok_or_else(|| invalid(model, "scalar write has a staged array index"))?;
                lower_canonical_assignment_expression_program(
                    model,
                    mir,
                    expression.id,
                    source_index.ok_or_else(|| invalid(model, "missing original array index"))?,
                    limits,
                )
            }
            ExpressionRole::Value(axes) if axes.is_empty() => {
                lower_canonical_assignment_expression_program(
                    model,
                    mir,
                    assignment.expr.id,
                    source_value,
                    limits,
                )
            }
            ExpressionRole::Value(axes) => lower_canonical_shadow_program(
                model,
                mir,
                assignment.expr.id,
                source_value,
                axes,
                limits,
            ),
        }
    };
    let copy_or_lower = |program: &BytecodeProgram,
                         role: &ExpressionRole|
     -> JitResult<NativeProgram> {
        if let Some(slot) = copied_slot(program, &cursor.staging_slots) {
            if !definitions.contains_key(&slot) {
                return Err(invalid(
                    model,
                    format!("copy references undefined temporary {slot}"),
                ));
            }
            NativeProgram::from_bytecode(model.name.clone(), EntryKind::Assignment, program, limits)
        } else {
            lower_expression(role)
        }
    };

    let mut lowered = Vec::new();
    let mut emitted = Vec::new();
    for step in steps {
        if let AssignmentStep::Assign(write) = step
            && cursor.staging_slots.contains(&write.var_index)
        {
            if live.get(write.var_index).copied().unwrap_or(false) {
                let role = roles.get(&write.var_index).ok_or_else(|| {
                    invalid(
                        model,
                        format!("temporary {} has no publication", write.var_index),
                    )
                })?;
                lowered.push(NativeAssignment::Direct {
                    var_index: write.var_index,
                    program: lower_expression(role)?,
                });
                emitted.push(write.var_index);
            }
            continue;
        }
        let role = value_role(model, hir, assignment, step, shadows)?;
        match step {
            AssignmentStep::Assign(write) => {
                if live.get(write.var_index).copied().unwrap_or(false) {
                    lowered.push(NativeAssignment::Direct {
                        var_index: write.var_index,
                        program: copy_or_lower(&write.program, &role)?,
                    });
                }
            }
            AssignmentStep::AssignIndexed {
                base,
                len,
                lower,
                index,
                value,
            } => {
                if assignment_range_live(*base, *len, live) {
                    let index = copy_or_lower(index, &ExpressionRole::Index)?;
                    let value = copy_or_lower(value, &role)?;
                    if let Some(var_index) =
                        constant_indexed_assignment_slot(&index, *base, *len, *lower)
                    {
                        validate_assignment_target(model, var_index)?;
                        lowered.push(NativeAssignment::Direct {
                            var_index,
                            program: value,
                        });
                    } else {
                        lowered.push(NativeAssignment::Indexed {
                            base: *base,
                            len: *len,
                            lower: *lower,
                            index,
                            value,
                        });
                    }
                }
            }
            _ => return Err(invalid(model, "unexpected statement inside staging group")),
        }
    }
    cursor.emitted_staging.extend(emitted);
    Ok(lowered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_ddx_staging_layouts_refuse_before_execution() {
        let report = crate::VerilogACompiler::default()
            .compile_runtime(
                "module staged(p,n); inout p,n; electrical p,n; real x,y; analog begin x=V(p,n)*V(p,n)*V(p,n); x=ddx(x,V(p,n)); y=ddx(x,V(p,n)); I(p,n)<+y; end endmodule",
                None,
            )
            .unwrap();
        let first = report.model.assignment_steps.iter().position(|step| {
            matches!(step, AssignmentStep::Assign(write) if report.model.variable_names[write.var_index].starts_with("@ddx_update"))
        }).unwrap();
        for corruption in ["missing definition", "late definition", "foreign target"] {
            let mut model = report.model.clone();
            match corruption {
                "missing definition" => {
                    model.assignment_steps.remove(first);
                }
                "late definition" => {
                    // Publish a derivative before the original value is frozen.
                    let publication = (first..model.assignment_steps.len()).find(|&position| {
                        matches!(&model.assignment_steps[position], AssignmentStep::Assign(write) if !model.variable_names[write.var_index].starts_with("@ddx_update"))
                    }).unwrap();
                    model.assignment_steps.swap(first, publication);
                }
                "foreign target" => {
                    let publication = model.assignment_steps.iter_mut().skip(first).find(|step| {
                        matches!(step, AssignmentStep::Assign(write) if !model.variable_names[write.var_index].starts_with("@ddx_update"))
                    }).unwrap();
                    let AssignmentStep::Assign(write) = publication else {
                        unreachable!()
                    };
                    write.var_index = model
                        .variable_names
                        .iter()
                        .position(|name| name == "y")
                        .unwrap();
                }
                _ => unreachable!(),
            }
            let error = build_model_plan_with_canonical_ir(&model, &report.canonical_ir)
                .expect_err("malformed staging must refuse")
                .to_string();
            assert!(
                error.contains("canonical ddx staging"),
                "{corruption}: {error}"
            );
        }
    }
}
