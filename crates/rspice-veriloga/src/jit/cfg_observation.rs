//! Publish named values using the same directional AD as the equation kernel.

use std::collections::HashMap;

use super::cfg_lanes::scalarize_lanes;
use super::cfg_program::{CfgRuntimeBindings, lower_cfg_function_to_variables};
use super::current_dependencies::JitCurrentDependencies;
use super::model_plan::NativeObservationPlan;
use super::plan_builder::canonical_branch_unknown_runtime_map;
use super::plan_program::{BlockProgram, PlanProgram};
use super::{JitError, JitResult};
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::{
    CanonicalIrArtifact, CfgStateAllocation, differentiate, prune_cfg_to_outputs,
};
use crate::codegen::CompiledModel;
use crate::codegen::state_renumbering::StateSlotMapping;

pub(super) fn build(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<NativeObservationPlan> {
    let refuse = |detail: String| JitError::InvalidCanonicalIr {
        model: model.name.clone(),
        detail: detail.into(),
    };
    let cfg = CfgModel::from_hir_for_executable_observation(&artifact.hir, &artifact.mir)
        .map_err(|errors| refuse(format!("canonical observation lowering: {errors:?}")))?;
    let variables: HashMap<_, _> = model
        .variable_names
        .iter()
        .enumerate()
        .map(|(slot, name)| (name.as_str(), slot))
        .collect();
    if variables.len() != model.num_variables || model.variable_names.len() != model.num_variables {
        return Err(refuse(
            "observation requires a unique name for each runtime variable".into(),
        ));
    }
    let mut outputs = Vec::new();
    let mut targets = Vec::new();
    for (variable, value) in artifact.hir.variables.iter().zip(&cfg.observation_roots) {
        if variable.name.contains('@') || variable.name.starts_with("__guard") {
            continue;
        }
        let slot = variables
            .get(variable.name.as_str())
            .copied()
            .ok_or_else(|| {
                refuse(format!(
                    "observation variable '{}' has no runtime slot",
                    variable.name
                ))
            })?;
        targets.push(slot);
        outputs.push(*value);
    }
    if cfg.observation_roots.len() != artifact.hir.variables.len() {
        return Err(refuse(
            "observation output layout does not match HIR variables".into(),
        ));
    }
    let state = CfgStateAllocation::build(&artifact.hir, &cfg.function)
        .map_err(|errors| refuse(format!("observation state allocation: {errors:?}")))?;
    let event_state_variables: Vec<_> = artifact
        .hir
        .variables
        .iter()
        .filter(|variable| variable.is_state)
        .map(|variable| variables.get(variable.name.as_str()).copied())
        .collect();
    if event_state_variables.len() != model.event_state_variables.len()
        || event_state_variables
            .iter()
            .zip(&model.event_state_variables)
            .any(|(actual, expected)| *actual != Some(*expected))
    {
        return Err(refuse(
            "observation state slots disagree with the runtime layout".into(),
        ));
    }
    let bindings = CfgRuntimeBindings::from_mir(
        model.name.as_str(),
        &artifact.mir,
        canonical_branch_unknown_runtime_map(model, &artifact.mir)?,
        event_state_variables,
    );
    let (pruned, outputs) = prune_cfg_to_outputs(&cfg.function, &outputs);
    // ddx resolves its own directional axes. No extra Jacobian lanes are
    // requested for a readback, and no derivative-order ceiling is introduced.
    let resolved = differentiate(&pruned, &[])
        .map_err(|error| refuse(format!("observation differentiation: {error:?}")))?;
    let scalarized = scalarize_lanes(&resolved.function)
        .map_err(|error| refuse(format!("observation scalarization: {error}")))?;
    let outputs = outputs
        .into_iter()
        .map(|value| {
            scalarized
                .scalar(value)
                .ok_or_else(|| refuse("observation output has no scalar value".into()))
        })
        .collect::<JitResult<Vec<_>>>()?;
    let (pruned, outputs) = prune_cfg_to_outputs(&scalarized.function, &outputs);
    let publications: Vec<_> = outputs.into_iter().zip(targets).collect();
    let program = lower_cfg_function_to_variables(
        &pruned,
        &publications,
        model.num_variables,
        &state,
        &bindings,
    )?;
    let slots = StateSlotMapping::build(model, &artifact.hir, &artifact.mir);
    let program = PlanProgram::Blocks(
        BlockProgram::adopt(model.name.as_str(), program, &slots)
            .map_err(|error| refuse(format!("observation state mapping: {error}")))?,
    );
    let current_dependencies = JitCurrentDependencies {
        assignment_current_pairs: program.current_pair_dependencies().to_vec(),
        assignment_prior_currents: program.prior_current_dependencies().to_vec(),
        assignment_branch_unknowns: program.branch_unknown_dependencies().to_vec(),
        ..JitCurrentDependencies::default()
    };
    Ok(NativeObservationPlan {
        program,
        current_dependencies,
    })
}
