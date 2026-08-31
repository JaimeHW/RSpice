//! Shared validation of the semantic identity between canonical IR artifacts
//! and executable bytecode models.
//!
//! This module is deliberately backend-neutral. Native/WASM add their own ABI
//! coverage checks after this validation, while the portable CFG evaluator
//! uses the same identity and layout contract.

use crate::canonical_ir::{CanonicalIrArtifact, MirEquationKind, MirModel, NodeId};
use crate::codegen::{CompiledModel, StampIndex, StampProgram};

const GROUND_ENDPOINT: usize = usize::MAX;

pub(crate) fn validate_canonical_artifact_identity_for_model(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> Result<(), String> {
    artifact.validate().map_err(|diagnostics| {
        diagnostics
            .first()
            .map(|diagnostic| diagnostic.message.to_string())
            .unwrap_or_else(|| "canonical artifact validation failed".into())
    })?;

    if artifact.mir.module_name != model.name {
        return Err(format!(
            "canonical module '{}' does not match compiled model '{}'",
            artifact.mir.module_name, model.name
        ));
    }
    if artifact.mir.equations.len() != model.stamp_programs.len() {
        return Err(format!(
            "canonical equation count {} does not match stamp program count {}",
            artifact.mir.equations.len(),
            model.stamp_programs.len()
        ));
    }

    validate_source_digest(model, artifact)?;
    validate_parameters(model, &artifact.mir)?;
    for (index, (equation, stamp)) in artifact
        .mir
        .equations
        .iter()
        .zip(&model.stamp_programs)
        .enumerate()
    {
        validate_equation_matches_stamp(model, &artifact.mir, index, equation, stamp)?;
    }
    Ok(())
}

fn validate_source_digest(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> Result<(), String> {
    if model.source_digest.is_empty() {
        return Err(concat!(
            "compiled model is missing source digest for canonical execution; ",
            "rebuild it with a digest-aware compiler/codegen path"
        )
        .into());
    }
    if model.source_digest != artifact.metadata.source_digest {
        return Err(format!(
            "canonical source digest '{}' does not match compiled model source digest '{}'",
            artifact.metadata.source_digest, model.source_digest
        ));
    }
    Ok(())
}

fn validate_parameters(model: &CompiledModel, mir: &MirModel) -> Result<(), String> {
    if mir.parameters.len() != model.parameters.len() {
        return Err(format!(
            "canonical parameter count {} does not match compiled parameter count {}",
            mir.parameters.len(),
            model.parameters.len()
        ));
    }

    for (index, (canonical, compiled)) in mir.parameters.iter().zip(&model.parameters).enumerate() {
        if canonical.name != compiled.name {
            return Err(format!(
                "canonical parameter {index} name '{}' does not match compiled parameter '{}'",
                canonical.name, compiled.name
            ));
        }
        if compiled.default_program.is_none()
            && !canonical
                .default
                .is_some_and(|default| default.to_bits() == compiled.default.to_bits())
        {
            return Err(format!(
                "canonical parameter '{}' default {:?} does not match compiled default {}",
                canonical.name, canonical.default, compiled.default
            ));
        }
        if compiled.default_program.is_some() && canonical.default_expr.is_none() {
            return Err(format!(
                "canonical parameter '{}' is missing dependent default expression",
                canonical.name
            ));
        }

        let resolve_range_parameter = |name: &smol_str::SmolStr| {
            mir.parameters
                .iter()
                .position(|parameter| parameter.name == *name)
                .ok_or_else(|| {
                    format!(
                        "canonical parameter '{}' range references unknown parameter '{}'",
                        canonical.name, name
                    )
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
                    .collect::<Result<Vec<_>, _>>()
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
            return Err(format!(
                "canonical parameter '{}' range metadata does not match compiled parameter metadata",
                canonical.name
            ));
        }
    }
    Ok(())
}

fn validate_equation_matches_stamp(
    model: &CompiledModel,
    mir: &MirModel,
    index: usize,
    equation: &crate::canonical_ir::MirEquation,
    stamp: &StampProgram,
) -> Result<(), String> {
    let expected_kind = compiled_equation_kind(stamp);
    if equation.kind != expected_kind {
        return Err(format!(
            "canonical equation {index} kind {:?} does not match compiled stamp kind {:?}",
            equation.kind, expected_kind
        ));
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
    let matches = match expected_kind {
        MirEquationKind::Current => (canonical_pos, canonical_neg) == (compiled_pos, compiled_neg),
        MirEquationKind::Potential | MirEquationKind::Indirect => {
            (canonical_pos, canonical_neg) == (compiled_pos, compiled_neg)
                || (canonical_pos, canonical_neg) == (compiled_neg, compiled_pos)
        }
    };
    if !matches {
        return Err(format!(
            "canonical equation {index} branch {} does not match compiled stamp branch {}",
            format_pair(canonical_pos, canonical_neg),
            format_pair(compiled_pos, compiled_neg)
        ));
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
) -> Result<(usize, usize), String> {
    let ordinal = stamp
        .branch_ordinal
        .ok_or_else(|| format!("compiled stamp {stamp_index} has no branch source ordinal"))?;
    let source = model.branch_sources.get(ordinal).ok_or_else(|| {
        format!(
            "compiled stamp {stamp_index} branch ordinal {ordinal} is outside branch source table"
        )
    })?;
    Ok((
        compiled_branch_endpoint(model, &source.pos)?,
        compiled_branch_endpoint(model, &source.neg)?,
    ))
}

fn infer_current_unified_pair(
    model: &CompiledModel,
    program: &StampProgram,
) -> Option<(usize, usize)> {
    let mut pos_endpoint = None;
    let mut neg_endpoint = None;
    for location in &program.stamp_locations {
        let endpoint = stamp_row_unified_endpoint(model, &location.row)?;
        if location.sign < 0.0 {
            if pos_endpoint.replace(endpoint).is_some() {
                return None;
            }
        } else if location.sign > 0.0 && neg_endpoint.replace(endpoint).is_some() {
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
        StampIndex::Terminal(terminal) if *terminal < model.num_terminals => Some(*terminal),
        StampIndex::Internal(internal) if *internal < model.internal_nodes => {
            Some(model.num_terminals + *internal)
        }
        StampIndex::Ground => Some(GROUND_ENDPOINT),
        _ => None,
    }
}

fn compiled_branch_endpoint(model: &CompiledModel, index: &StampIndex) -> Result<usize, String> {
    match index {
        StampIndex::Terminal(terminal) if *terminal < model.num_terminals => Ok(*terminal),
        StampIndex::Terminal(terminal) => Err(format!(
            "compiled branch source terminal {terminal} exceeds terminal count {}",
            model.num_terminals
        )),
        StampIndex::Internal(internal) if *internal < model.internal_nodes => {
            Ok(model.num_terminals + *internal)
        }
        StampIndex::Internal(internal) => Err(format!(
            "compiled branch source internal node {internal} exceeds internal node count {}",
            model.internal_nodes
        )),
        StampIndex::Ground => Ok(GROUND_ENDPOINT),
        StampIndex::Branch(branch) => Err(format!(
            "compiled branch source endpoint unexpectedly references branch {branch}"
        )),
    }
}

fn canonical_branch_endpoint(
    model: &CompiledModel,
    mir: &MirModel,
    node_id: Option<NodeId>,
) -> Result<usize, String> {
    let Some(node_id) = node_id else {
        return Ok(GROUND_ENDPOINT);
    };
    let node_index = usize::from(node_id);
    let node = mir
        .nodes
        .get(node_index)
        .filter(|node| node.id == node_id)
        .ok_or_else(|| {
            format!("canonical branch endpoint node {node_id} is outside MIR node table")
        })?;
    if node.is_external {
        let terminal_name = model.terminal_names.get(node_index).ok_or_else(|| {
            format!(
                "canonical branch endpoint terminal {node_index} exceeds compiled terminal count {}",
                model.num_terminals
            )
        })?;
        if terminal_name != &node.name {
            return Err(format!(
                "canonical branch endpoint terminal {node_index} names '{}' but compiled terminal is '{}'",
                node.name, terminal_name
            ));
        }
        return Ok(node_index);
    }

    let external_count = mir.nodes.iter().filter(|node| node.is_external).count();
    let internal_index = node_index.checked_sub(external_count).ok_or_else(|| {
        format!(
            "canonical internal branch endpoint {} appears before external nodes",
            node.name
        )
    })?;
    if internal_index < model.internal_nodes {
        return Ok(model.num_terminals + internal_index);
    }
    Err(format!(
        "canonical branch endpoint internal node {internal_index} exceeds compiled internal node count {}",
        model.internal_nodes
    ))
}

fn format_pair(pos: usize, neg: usize) -> String {
    fn endpoint(value: usize) -> String {
        if value == GROUND_ENDPOINT {
            "ground".into()
        } else {
            value.to_string()
        }
    }
    format!("{}->{}", endpoint(pos), endpoint(neg))
}
