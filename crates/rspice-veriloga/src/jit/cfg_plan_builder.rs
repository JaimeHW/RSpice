//! The CFG route's model plan.
//!
//! [`build_model_plan_with_canonical_ir`] lowers every value entry from MIR's
//! flat postfix stream. This is the second constructor: the same
//! [`NativeModelPlan`], with `stamp_values`, `jacobians`, `reactive_jacobians`,
//! `noise_psd` and `noise_exponents` lowered instead from the canonical CFG
//! through [`lower_cfg_function`] and [`scalarize_lanes`], adopted as
//! [`PlanProgram::Blocks`].
//!
//! It is not the default. Production still builds the postfix plan; this one is
//! reached by the CFG-versus-MIR census (`native::cfg_mir_census`), which is
//! the evidence the flip is decided on.
//!
//! # What stays postfix, and why that is not a half measure
//!
//! `parameter_defaults`, `static_conditions` and both assignment passes keep
//! their MIR lowering. A parameter default is evaluated once, before any body
//! runs, from a MIR expression the CFG does not carry; the assignment passes
//! write the runtime variable slots the *postfix* value entries read, and the
//! CFG route recomputes those values inline instead. Leaving them alone is
//! therefore what makes the two plans comparable at all: both fill `variables`
//! by the same code, and the value entries then have to agree about what that
//! operating point evaluates to.
//!
//! # All or nothing, per module
//!
//! Every refusal is per *module*, never per entry. A plan that took the CFG
//! form for the entries it could lower and the postfix form for the rest would
//! be a third thing neither census measured, and its evidence would say nothing
//! about either route. So a construct the CFG route will not lower, a shipped
//! Jacobian column the derivative pass numbers differently, or a shipped noise
//! source with no CFG process all refuse the whole module by name — see
//! [`CfgPlanRefusal`] for the list.
//!
//! What is *not* a refusal is a difference the two routes are entitled to:
//! grouped noise, where the shipped PSD program and the CFG process hold
//! different quantities on purpose. That is recorded on the report
//! ([`CfgPlanReport::grouped_noise`]) for the census to classify, because
//! refusing it would refuse thirty-four of the forty-three shipped modules and
//! take every entry they *do* agree on down with them.
//!
//! # The one place a program is built rather than lowered
//!
//! The shipped planner decides Jacobian sparsity with a conservative structural
//! reachability mask and then drops whatever its simplifier folds to a literal
//! zero. Liveness on the CFG is exact, so it drops entries the shipped planner
//! keeps: the shipped plan carries a program that always returns zero and the
//! CFG route carries no value at all. The plan shape is the shipped one — a
//! backend indexes `jacobians[stamp][entry]` positionally — so those entries
//! need a program, and the program the CFG route's own analysis implies is the
//! constant zero. They are counted ([`CfgPlanReport::structural_zeros`]) and the
//! census checks each one against what the shipped entry actually evaluates to,
//! because "the shipped program is identically zero" is a claim about the
//! corpus and not a theorem.

use std::collections::HashMap;

use super::cfg_lanes::scalarize_lanes;
use super::cfg_program::{CfgRuntimeBindings, lower_cfg_function};
use super::expr::NativeOp;
use super::model_plan::NativeModelPlan;
use super::plan_builder::{
    build_model_plan_with_canonical_ir, canonical_branch_unknown_runtime_map,
};
use super::plan_program::{BlockProgram, PlanProgram};
use super::ssa::{BlockId, BuilderTerminator, Program, ProgramBuilder, ValueType};
use super::{JitError, JitResult};
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::cfg_lower::CfgNoiseProcess;
use crate::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgFunction, CfgStateAllocation, CfgValueKind, MirModel, ValueId,
    differentiate, prune_cfg_to_outputs,
};
use crate::codegen::state_renumbering::StateSlotMapping;
use crate::codegen::{ColumnAxis, CompiledModel};
use crate::rust_backend::canonical::stored_charges;

/// Why the CFG route cannot build a plan for a module.
///
/// Named rather than a string so the census can tabulate refusals by class and
/// a reader can tell "this construct is not lowered yet" from "these two routes
/// disagree about what the model is".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CfgPlanRefusal {
    /// The shipped postfix plan, which this one is built on top of, refused the
    /// module. Nothing about the CFG route is implicated.
    ShippedPlan,
    /// The HIR body does not lower to a CFG at all.
    CfgLowering,
    /// The CFG's analog-operator sites cannot be resolved onto the runtime's
    /// state records. This is where [`crate::canonical_ir::HirExecutedCorrespondence`]
    /// refuses a site it does not cover — a `case` arm's condition, a prologue
    /// statement, a named block's local initializer.
    StateAllocation,
    /// The derivative pass refused the body.
    Differentiate,
    /// The lane scalarizer refused the differentiated body.
    Scalarize,
    /// A value the plan needs has no scalar after lane scalarization.
    NoScalar,
    /// [`lower_cfg_function`] refused an entry: the W-C refusal classes.
    Lowering,
    /// The shipped model and the CFG disagree about how many equations the
    /// module has, so no entry can be paired with the residual it belongs to.
    EquationsUnpaired,
    /// A shipped Jacobian column names a lane outside the derivative pass's
    /// seed list, so the two routes number the unknowns differently.
    LaneUnmapped,
    /// A stamp has reactive Jacobian entries but the CFG route found no stored
    /// charge for its equation.
    ChargeMissing,
    /// A shipped noise source has no CFG process with its id.
    NoiseUnpaired,
    /// A block program addresses a state slot the module's
    /// [`StateSlotMapping`] never allocated.
    SlotUnclaimed,
}

impl CfgPlanRefusal {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ShippedPlan => "shipped-plan",
            Self::CfgLowering => "cfg-lowering",
            Self::StateAllocation => "state-allocation",
            Self::Differentiate => "differentiate",
            Self::Scalarize => "scalarize",
            Self::NoScalar => "no-scalar",
            Self::Lowering => "lowering",
            Self::EquationsUnpaired => "equations-unpaired",
            Self::LaneUnmapped => "lane-unmapped",
            Self::ChargeMissing => "charge-missing",
            Self::NoiseUnpaired => "noise-unpaired",
            Self::SlotUnclaimed => "slot-unclaimed",
        }
    }
}

/// A module the CFG route will not build a plan for, with the detail the
/// refusing pass gave.
#[derive(Debug, Clone)]
pub(crate) struct CfgPlanRefused {
    pub(crate) module: String,
    pub(crate) class: CfgPlanRefusal,
    pub(crate) detail: String,
}

impl std::fmt::Display for CfgPlanRefused {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "the CFG route refuses {}: {}: {}",
            self.module,
            self.class.name(),
            self.detail
        )
    }
}

impl From<CfgPlanRefused> for JitError {
    fn from(refused: CfgPlanRefused) -> Self {
        Self::unsupported_native_coverage(
            refused.module.clone(),
            format!(
                "CFG-built plan, {}: {}",
                refused.class.name(),
                refused.detail
            ),
        )
    }
}

/// What the CFG route did for one module, in the terms the flip is judged on.
#[derive(Debug, Clone, Default)]
pub(crate) struct CfgPlanReport {
    pub(crate) stamp_values: usize,
    pub(crate) jacobians: usize,
    pub(crate) reactive_jacobians: usize,
    pub(crate) noise_values: usize,
    /// Whether this module's noise reaches the runtime as a grouped complex
    /// injection.
    ///
    /// It changes what the shipped `psd_program` *is*. Under the flat schema it
    /// is the syntactic noise power, which is also what the CFG process's `psd`
    /// value holds; under the grouped schema the routing amplitude is folded
    /// into the injection instead, and the two are no longer the same quantity.
    /// Recorded rather than refused, because refusing here would refuse almost
    /// the whole shipped corpus — measured: thirty-four of the forty-three
    /// modules carry schema 1 — and the entries that *are* the same quantity on
    /// those modules would go unmeasured with them.
    pub(crate) grouped_noise: bool,
    /// Shipped Jacobian and reactive-Jacobian entries the CFG route's liveness
    /// found no value for, given the constant zero its analysis implies. Keyed
    /// so the census can name each one it checks.
    pub(crate) structural_zeros: Vec<CfgPlanEntry>,
    /// Noise magnitudes that only lowered after the derivative pass resolved a
    /// `ddx` inside them — the shipped emitter's own two-pass rule, kept.
    pub(crate) noise_from_differentiated: usize,
    /// Instructions in the largest block entry, with its identity.
    pub(crate) largest_entry: Option<(CfgPlanEntry, usize)>,
}

/// One value entry's position in the plan, for a message that names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CfgPlanEntry {
    StampValue(usize),
    Jacobian(usize, usize),
    ReactiveJacobian(usize, usize),
    NoisePsd(usize),
    NoiseExponent(usize),
}

impl std::fmt::Display for CfgPlanEntry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StampValue(stamp) => write!(formatter, "stamp_values[{stamp}]"),
            Self::Jacobian(stamp, entry) => write!(formatter, "jacobians[{stamp}][{entry}]"),
            Self::ReactiveJacobian(stamp, entry) => {
                write!(formatter, "reactive_jacobians[{stamp}][{entry}]")
            }
            Self::NoisePsd(source) => write!(formatter, "noise_psd[{source}]"),
            Self::NoiseExponent(source) => write!(formatter, "noise_exponents[{source}]"),
        }
    }
}

/// A CFG-built plan and the figures it reports about itself.
#[derive(Debug)]
pub(crate) struct CfgModelPlan {
    pub(crate) plan: NativeModelPlan,
    pub(crate) report: CfgPlanReport,
}

/// Which lane a shipped Jacobian column stands for.
///
/// The inverse of the seed-list construction below, and the same arithmetic the
/// generated backend's `emit_row` performs when it splits a lane index back
/// into a node column and a branch column.
pub(crate) fn shipped_entry_lane(axis: &ColumnAxis, node_count: usize) -> usize {
    match axis {
        ColumnAxis::Node(node) => *node,
        ColumnAxis::Branch(branch) => node_count + *branch,
    }
}

/// The seed list, in the order every consumer of the derivative pass already
/// uses: node potentials, then branch unknowns, then the limiter correction if
/// the model limits at all.
///
/// Read off the generated-Rust backend rather than invented here, because it is
/// the one shipped consumer of CFG-level AD and the lane index means "unknown
/// number n" only if both agree. Its own words for putting the correction last:
/// "a model without `$limit` carries no lane for it and every other lane index
/// still means 'unknown number n'".
pub(crate) fn derivative_seeds(cfg: &CfgModel, mir: &MirModel) -> (Vec<AdSeed>, Option<usize>) {
    let limits = cfg
        .function
        .values
        .iter()
        .any(|value| matches!(value.kind, CfgValueKind::Limit { .. }));
    let seeds: Vec<AdSeed> = (0..mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain((0..mir.branch_unknowns.len()).map(|index| AdSeed::BranchUnknownFlow(index.into())))
        .chain(limits.then_some(AdSeed::LimiterCorrection))
        .collect();
    let correction = limits.then(|| seeds.len() - 1);
    (seeds, correction)
}

/// The block program a structurally absent entry gets: one instruction, one
/// block, returning zero.
fn constant_zero_program() -> JitResult<Program> {
    let entry = BlockId::new(0)?;
    let mut builder = ProgramBuilder::new(&[Vec::new()])?;
    builder.begin_block(entry)?;
    let zero = builder.push(NativeOp::Const(0.0), &[], ValueType::F64)?;
    builder.end_block(BuilderTerminator::Return(zero))?;
    builder.finish(entry, entry)
}

/// Build the CFG route's plan for `model`, or say why it cannot be built.
///
/// The postfix plan is built first and kept: it validates the canonical
/// artifact against the compiled model, and its assignment passes, parameter
/// defaults, static conditions and published current pairs are the CFG plan's
/// too. Only the five program-bearing value fields are replaced.
pub(crate) fn build_model_plan_from_canonical_cfg(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> Result<CfgModelPlan, CfgPlanRefused> {
    let module = model.name.to_string();
    let refuse = |class: CfgPlanRefusal, detail: String| CfgPlanRefused {
        module: module.clone(),
        class,
        detail,
    };

    let mut plan = build_model_plan_with_canonical_ir(model, artifact)
        .map_err(|error| refuse(CfgPlanRefusal::ShippedPlan, error.to_string()))?;

    let mut cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).map_err(|diagnostics| {
        refuse(
            CfgPlanRefusal::CfgLowering,
            diagnostics
                .first()
                .map_or_else(|| "unknown".to_string(), |first| first.message.to_string()),
        )
    })?;
    if model.stamp_programs.len() != cfg.residuals.len() {
        return Err(refuse(
            CfgPlanRefusal::EquationsUnpaired,
            format!(
                "{} shipped equations against {} canonical residuals",
                model.stamp_programs.len(),
                cfg.residuals.len()
            ),
        ));
    }

    // Charges first: the extraction *builds* the values a scaled or summed
    // charge needs (`k * ddt(q)` stores `k * q`, which exists nowhere until it
    // is spliced in), so it has to run before the state allocation is read off
    // the function and before anything is differentiated.
    let charges = stored_charges(&mut cfg.function, &cfg.residuals);
    let state = CfgStateAllocation::build(&artifact.hir, &cfg.function).map_err(|errors| {
        refuse(
            CfgPlanRefusal::StateAllocation,
            errors
                .first()
                .map_or_else(|| "unknown".to_string(), ToString::to_string),
        )
    })?;

    let (seeds, correction_lane) = derivative_seeds(&cfg, &artifact.mir);
    let mut differentiated = differentiate(&cfg.function, &seeds)
        .map_err(|error| refuse(CfgPlanRefusal::Differentiate, format!("{error:?}")))?;
    // Every read-out before anything lowers: taking one appends an instruction,
    // so a function captured earlier would not contain the later ones.
    let jacobian_rows: Vec<Vec<Option<ValueId>>> = cfg
        .residuals
        .iter()
        .map(|residual| differentiated.derivative_row(*residual))
        .collect();
    let reactive_rows: Vec<Vec<Option<ValueId>>> = charges
        .iter()
        .map(|charge| match charge {
            Some(charge) => differentiated.derivative_row(*charge),
            None => Vec::new(),
        })
        .collect();
    let scalarized = scalarize_lanes(&differentiated.function)
        .map_err(|error| refuse(CfgPlanRefusal::Scalarize, error.to_string()))?;

    let branch_unknowns = canonical_branch_unknown_runtime_map(model, &artifact.mir)
        .map_err(|error| refuse(CfgPlanRefusal::ShippedPlan, error.to_string()))?;
    // The CFG numbers event-controlled state in HIR declaration order; the
    // runtime numbers every variable the model has. Names are what the two
    // spaces share.
    let event_state_variables: Vec<Option<usize>> = artifact
        .hir
        .variables
        .iter()
        .filter(|variable| variable.is_state)
        .map(|variable| {
            model
                .variable_names
                .iter()
                .position(|name| *name == variable.name)
        })
        .collect();
    let bindings = CfgRuntimeBindings::from_mir(
        module.as_str(),
        &artifact.mir,
        branch_unknowns,
        event_state_variables,
    );
    let slots = StateSlotMapping::build(model, &artifact.hir, &artifact.mir);

    let mut report = CfgPlanReport {
        grouped_noise: model.noise_process_schema >= 1 && !model.noise_sources.is_empty(),
        ..CfgPlanReport::default()
    };
    let node_count = artifact.mir.nodes.len();

    // One lowering closure, so every entry reaches the block model by the same
    // three steps: prune to the output, lower, adopt through the module's slot
    // mapping.
    let lower = |function: &CfgFunction,
                 output: ValueId,
                 entry: CfgPlanEntry,
                 report: &mut CfgPlanReport|
     -> Result<PlanProgram, CfgPlanRefused> {
        let (pruned, outputs) = prune_cfg_to_outputs(function, &[output]);
        let program = lower_cfg_function(&pruned, outputs[0], &state, &bindings)
            .map_err(|error| refuse(CfgPlanRefusal::Lowering, format!("{entry}: {error}")))?;
        let instructions = program.instructions().len();
        if report
            .largest_entry
            .is_none_or(|(_, largest)| instructions > largest)
        {
            report.largest_entry = Some((entry, instructions));
        }
        let adopted = BlockProgram::adopt(module.as_str(), program, &slots)
            .map_err(|error| refuse(CfgPlanRefusal::SlotUnclaimed, format!("{entry}: {error}")))?;
        Ok(PlanProgram::Blocks(adopted))
    };

    let scalar = |value: ValueId, entry: CfgPlanEntry| -> Result<ValueId, CfgPlanRefused> {
        scalarized
            .scalar(value)
            .ok_or_else(|| refuse(CfgPlanRefusal::NoScalar, entry.to_string()))
    };

    // The lane a shipped column stands for, refusing the module if the two
    // routes have numbered the unknowns differently. The limiter correction is
    // a displacement rather than a coordinate, so a shipped column landing on
    // it would mean the same thing.
    let require_lane = |axis: &ColumnAxis,
                        node_count: usize,
                        entry: CfgPlanEntry|
     -> Result<usize, CfgPlanRefused> {
        let lane = shipped_entry_lane(axis, node_count);
        if lane >= seeds.len() || Some(lane) == correction_lane {
            return Err(refuse(
                CfgPlanRefusal::LaneUnmapped,
                format!("{entry}: lane {lane} of {} seeds", seeds.len()),
            ));
        }
        Ok(lane)
    };

    // ---- stamp values and Jacobians ------------------------------------
    //
    // Both come off the *scalarized differentiated* body rather than the
    // primal, and that is not incidental: a primal CFG containing an
    // undifferentiated `ddx` has no value until the pass that resolves one has
    // run, and ten shipped modules are in that position. Running the pass first
    // is what makes their residuals lowerable at all.
    let mut stamp_values = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobians = Vec::with_capacity(model.stamp_programs.len());
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        // Slice the whole equation out of the body once, then each of its
        // entries out of that slice. Pruning is linear in the function it is
        // given, so slicing per entry from the *model* would cost the entry
        // count times a compact model's whole body — quadratic, and measured:
        // it took asmhemt past twenty gigabytes of working set and into paging
        // before this loop was written this way. Slicing the equation first
        // makes every inner pass linear in the equation instead. The result is
        // the same function either way: pruning is idempotent, and composing
        // two slices keeps exactly what the inner one asked for.
        let residual = cfg.residuals[stamp_index];
        let stamp_entry = CfgPlanEntry::StampValue(stamp_index);
        let mut row_entries: Vec<(CfgPlanEntry, ValueId)> =
            vec![(stamp_entry, scalar(residual, stamp_entry)?)];
        for (entry_index, jacobian) in stamp.jacobian_programs.iter().enumerate() {
            let entry = CfgPlanEntry::Jacobian(stamp_index, entry_index);
            let lane = require_lane(&jacobian.col_axis, node_count, entry)?;
            if let Some(value) = jacobian_rows[stamp_index].get(lane).copied().flatten() {
                row_entries.push((entry, scalar(value, entry)?));
            }
        }
        for (entry_index, reactive) in stamp.reactive_jacobians.iter().enumerate() {
            let entry = CfgPlanEntry::ReactiveJacobian(stamp_index, entry_index);
            let lane = require_lane(&reactive.col_axis, node_count, entry)?;
            if let Some(value) = reactive_rows[stamp_index].get(lane).copied().flatten() {
                row_entries.push((entry, scalar(value, entry)?));
            }
        }
        let row_outputs: Vec<ValueId> = row_entries.iter().map(|(_, value)| *value).collect();
        let (row_function, row_mapped) = prune_cfg_to_outputs(&scalarized.function, &row_outputs);
        let row_values: HashMap<CfgPlanEntry, ValueId> = row_entries
            .iter()
            .map(|(entry, _)| *entry)
            .zip(row_mapped)
            .collect();

        stamp_values.push(lower(
            &row_function,
            row_values[&stamp_entry],
            stamp_entry,
            &mut report,
        )?);
        report.stamp_values += 1;

        let mut row = Vec::with_capacity(stamp.jacobian_programs.len());
        for entry_index in 0..stamp.jacobian_programs.len() {
            let entry = CfgPlanEntry::Jacobian(stamp_index, entry_index);
            match row_values.get(&entry).copied() {
                Some(output) => {
                    row.push(lower(&row_function, output, entry, &mut report)?);
                    report.jacobians += 1;
                }
                None => {
                    report.structural_zeros.push(entry);
                    row.push(PlanProgram::Blocks(
                        BlockProgram::adopt(
                            module.as_str(),
                            constant_zero_program().map_err(|error| {
                                refuse(CfgPlanRefusal::Lowering, format!("{entry}: {error}"))
                            })?,
                            &slots,
                        )
                        .map_err(|error| {
                            refuse(CfgPlanRefusal::SlotUnclaimed, format!("{entry}: {error}"))
                        })?,
                    ));
                }
            }
        }
        jacobians.push(row);

        let mut reactive_row = Vec::with_capacity(stamp.reactive_jacobians.len());
        if !stamp.reactive_jacobians.is_empty() && charges[stamp_index].is_none() {
            return Err(refuse(
                CfgPlanRefusal::ChargeMissing,
                format!(
                    "stamp {stamp_index} has {} shipped reactive entries and no stored charge",
                    stamp.reactive_jacobians.len()
                ),
            ));
        }
        for entry_index in 0..stamp.reactive_jacobians.len() {
            let entry = CfgPlanEntry::ReactiveJacobian(stamp_index, entry_index);
            match row_values.get(&entry).copied() {
                Some(output) => {
                    reactive_row.push(lower(&row_function, output, entry, &mut report)?);
                    report.reactive_jacobians += 1;
                }
                None => {
                    report.structural_zeros.push(entry);
                    reactive_row.push(PlanProgram::Blocks(
                        BlockProgram::adopt(
                            module.as_str(),
                            constant_zero_program().map_err(|error| {
                                refuse(CfgPlanRefusal::Lowering, format!("{entry}: {error}"))
                            })?,
                            &slots,
                        )
                        .map_err(|error| {
                            refuse(CfgPlanRefusal::SlotUnclaimed, format!("{entry}: {error}"))
                        })?,
                    ));
                }
            }
        }
        reactive_jacobians.push(reactive_row);
    }

    // ---- noise ---------------------------------------------------------
    //
    // A noise magnitude is an ordinary value of the body, so it lowers from the
    // primal; the shipped emitter's two-pass rule is kept because six shipped
    // models read a `ddx` inside a noise power and only the AD pass resolves
    // one.
    let processes: HashMap<usize, &CfgNoiseProcess> = cfg
        .noise_processes
        .iter()
        .filter_map(|process| {
            usize::try_from(process.process_id)
                .ok()
                .map(|id| (id, process))
        })
        .collect();
    let mut noise_psd = Vec::with_capacity(model.noise_sources.len());
    let mut noise_exponents = Vec::with_capacity(model.noise_sources.len());
    for (source_index, source) in model.noise_sources.iter().enumerate() {
        let process = processes.get(&source.process_id).ok_or_else(|| {
            refuse(
                CfgPlanRefusal::NoiseUnpaired,
                format!(
                    "shipped source {source_index} names process {}",
                    source.process_id
                ),
            )
        })?;
        let lower_noise = |value: ValueId,
                           entry: CfgPlanEntry,
                           report: &mut CfgPlanReport|
         -> Result<PlanProgram, CfgPlanRefused> {
            match lower(&cfg.function, value, entry, report) {
                Ok(program) => Ok(program),
                Err(primal) if primal.class == CfgPlanRefusal::Lowering => {
                    let Some(resolved) = scalarized.scalar(value) else {
                        return Err(primal);
                    };
                    let program = lower(&scalarized.function, resolved, entry, report)?;
                    report.noise_from_differentiated += 1;
                    Ok(program)
                }
                Err(other) => Err(other),
            }
        };
        noise_psd.push(lower_noise(
            process.psd,
            CfgPlanEntry::NoisePsd(source_index),
            &mut report,
        )?);
        report.noise_values += 1;
        let exponent = match (process.exponent, source.exponent_program.as_ref()) {
            (Some(exponent), Some(_)) => {
                report.noise_values += 1;
                Some(lower_noise(
                    exponent,
                    CfgPlanEntry::NoiseExponent(source_index),
                    &mut report,
                )?)
            }
            (None, None) => None,
            (canonical, shipped) => {
                return Err(refuse(
                    CfgPlanRefusal::NoiseUnpaired,
                    format!(
                        "source {source_index} exponent: canonical={} shipped={}",
                        canonical.is_some(),
                        shipped.is_some()
                    ),
                ));
            }
        };
        noise_exponents.push(exponent);
    }

    // ---- the read-sets the runtime validates a dispatch against ---------
    //
    // Recomputed from the block programs rather than carried over: the two
    // routes do not reach the same unknowns, and a borrowed set would describe
    // a program that was never compiled.
    let dependencies = &mut plan.current_dependencies;
    dependencies.stamp_values = read_set(&stamp_values, PlanProgram::current_pair_dependencies);
    dependencies.stamp_value_prior_currents =
        read_set(&stamp_values, PlanProgram::prior_current_dependencies);
    dependencies.stamp_value_branch_unknowns =
        read_set(&stamp_values, PlanProgram::branch_unknown_dependencies);
    dependencies.jacobians = read_set_rows(&jacobians, PlanProgram::current_pair_dependencies);
    dependencies.jacobian_prior_currents =
        read_set_rows(&jacobians, PlanProgram::prior_current_dependencies);
    dependencies.jacobian_branch_unknowns =
        read_set_rows(&jacobians, PlanProgram::branch_unknown_dependencies);
    dependencies.reactive_jacobians =
        read_set_rows(&reactive_jacobians, PlanProgram::current_pair_dependencies);
    dependencies.reactive_jacobian_prior_currents =
        read_set_rows(&reactive_jacobians, PlanProgram::prior_current_dependencies);
    dependencies.reactive_jacobian_branch_unknowns = read_set_rows(
        &reactive_jacobians,
        PlanProgram::branch_unknown_dependencies,
    );
    dependencies.noise_psd = read_set(&noise_psd, PlanProgram::current_pair_dependencies);
    dependencies.noise_psd_prior_currents =
        read_set(&noise_psd, PlanProgram::prior_current_dependencies);
    dependencies.noise_psd_branch_unknowns =
        read_set(&noise_psd, PlanProgram::branch_unknown_dependencies);
    dependencies.noise_exponents =
        read_set_optional(&noise_exponents, PlanProgram::current_pair_dependencies);
    dependencies.noise_exponent_prior_currents =
        read_set_optional(&noise_exponents, PlanProgram::prior_current_dependencies);
    dependencies.noise_exponent_branch_unknowns =
        read_set_optional(&noise_exponents, PlanProgram::branch_unknown_dependencies);

    plan.stamp_values = stamp_values;
    plan.jacobians = jacobians;
    plan.reactive_jacobians = reactive_jacobians;
    plan.noise_psd = noise_psd;
    plan.noise_exponents = noise_exponents;
    plan.validate_shape(model)
        .map_err(|error| refuse(CfgPlanRefusal::EquationsUnpaired, error.to_string()))?;
    Ok(CfgModelPlan { plan, report })
}

fn read_set(entries: &[PlanProgram], read: fn(&PlanProgram) -> &[usize]) -> Vec<Vec<usize>> {
    entries.iter().map(|entry| read(entry).to_vec()).collect()
}

fn read_set_rows(
    rows: &[Vec<PlanProgram>],
    read: fn(&PlanProgram) -> &[usize],
) -> Vec<Vec<Vec<usize>>> {
    rows.iter().map(|row| read_set(row, read)).collect()
}

fn read_set_optional(
    entries: &[Option<PlanProgram>],
    read: fn(&PlanProgram) -> &[usize],
) -> Vec<Vec<usize>> {
    entries
        .iter()
        .map(|entry| {
            entry
                .as_ref()
                .map_or_else(Vec::new, |entry| read(entry).to_vec())
        })
        .collect()
}
