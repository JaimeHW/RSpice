//! Generating a device from the canonical CFG, rather than from a tier.
//!
//! This is the emitter the rebuild exists to produce, wired to the device
//! contract the tiers already satisfy: `state.rs` holds the parameters and the
//! per-instance state, `stamp.rs` evaluates the body and writes the matrix, and
//! `noise.rs` evaluates the small-signal powers. The last two are both the
//! output of [`super::emit`] over a simplified CFG — `stamp.rs` over a
//! differentiated and scheduled one, `noise.rs` over the primal body with no
//! derivative lanes at all, because a noise power is a magnitude and nothing
//! asks for its slope.
//!
//! ## What the tiers did that this does not
//!
//! *Scalarised derivatives.* A tier gives every lane its own value, so a wide
//! MOSFET carries a hundred thousand of them. Here a derivative is one packed
//! value over its own live lane set.
//!
//! *Flattened guards.* A tier turns `if` into arithmetic over both arms. Here
//! the control flow survives into the generated Rust, so the code skips the work
//! the model said to skip.
//!
//! *Zeros.* 202 of 931 stamp arguments in the tier output are literal
//! `multiplicity * 0.0`. [`super::stamp_plan`] decides which entries exist at
//! all, and the ones that do not are simply absent.
//!
//! ## Stages are functions, and that is why outputs need slots
//!
//! [`crate::canonical_ir::schedule::split`] cuts the body by how often each
//! value goes stale. Every class coarser than Newton becomes its own `fn` on
//! `Instance` that writes what later readers need into a slot array; the Newton
//! body runs in `stamp` and reads those slots. The instance and temperature
//! stages are guarded by validity flags, which is where the caching is. The
//! timestep stage runs on every call — nothing in the device contract tells
//! `stamp` that a new timestep began, and recomputing is correct, merely
//! uncached.
//!
//! Splitting at all is decided per model by
//! [`crate::canonical_ir::schedule::worth_splitting`], because a body that is
//! 97% Newton pays for the staged loads and gets nothing back. Most compact
//! models decline it.
//!
//! ## Charge storage
//!
//! A reactive stamp writes `d(charge)/d(unknown)`, not the residual's Jacobian,
//! so it needs the `ddt` operand rather than the `ddt` result. A contribution is
//! reactive when its residual *is* a `ddt` — which is how MIR presents it, one
//! equation per `<+` statement — and the charge is that operator's input,
//! differentiated by the same pass in the same body.
//!
//! ## What it refuses
//!
//! `$limit`, indirect contributions, and an unresolved flow probe. Each is a
//! piece the canonical level has not finished, and a device that quietly
//! computed something else would be worse than one that is not generated: the
//! caller falls back to a tier, which is what the tiers are still there for.

use std::collections::HashMap;
use std::fmt::Write as _;

use crate::canonical_ir::cfg::{
    CfgBinaryOp, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp, CfgValue, CfgValueKind,
    CfgValueType,
};
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::schedule::{InvalidationClass, Stage, schedule, split, worth_splitting};
use crate::canonical_ir::{
    AdSeed, BlockId, CanonicalIrArtifact, ExprId, MirEquationKind, ValueId, differentiate,
    optimize_cfg,
};

use super::emit::{EmitBindings, RUNTIME_PRELUDE, emit_body};
use super::expr::parameter_field_names;
use super::stamp_plan::{StampPlan, StampRow, split_row};
use super::{GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames};
use super::{RustTranspileOptions, device};

pub fn generate_device(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
) -> Result<GeneratedRustDevice, RustBackendError> {
    let plan = ModelPlan::build(artifact)?;

    let names = RustDeviceNames::new(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        artifact.metadata.source_digest.as_str(),
    );
    let parameter_fields = parameter_field_names(artifact);

    let stamp = plan.stamp_file(artifact, options)?;
    let state = device::generate_state_file_with_extensions(
        artifact,
        options,
        &parameter_fields,
        plan.ddt_slots.len(),
        plan.idt_slots.len(),
        artifact.mir.branch_unknowns.len(),
        &plan.state_extensions(),
    )?;

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
        plan.noise_file(artifact, options)?,
    ];

    Ok(GeneratedRustDevice {
        module_name: artifact.mir.module_name.to_string(),
        public_model_name: names.public_model_name,
        folder_name: names.folder,
        source_digest: artifact.metadata.source_digest.to_string(),
        files,
    })
}

/// Every noise magnitude the model declares, as one body.
///
/// Emitted from the *primal* CFG rather than the differentiated one: a noise
/// power is a magnitude, and nothing asks for its derivative. That is the whole
/// saving. The generator this replaces re-derived each magnitude from HIR
/// through a hand-written liveness index and emitted a schedule per source,
/// which is why `r3_cmc` — a resistor — carries 3,722 lines of it for six
/// magnitudes, and why `noise.rs` was 50.8% of the whole generated tree.
///
/// Not folded into the stamp's body and cached, the way the reactive matrix is.
/// `evaluate_noise_sources` is handed the DC solution as an argument and builds
/// its own context from it, so it does not run after a `stamp` at that solution
/// and has no cache to read. Reading one anyway would make the answer depend on
/// a call order the device contract does not promise.
struct NoisePlan {
    function: CfgFunction,
    outputs: Vec<ValueId>,
    sources: Vec<NoiseSourceValues>,
}

/// Where one source's magnitudes sit in [`NoisePlan::outputs`].
struct NoiseSourceValues {
    active: usize,
    psd: usize,
    exponent: Option<usize>,
    table: Vec<usize>,
    /// Multiplicity multiplies a current source's power and divides a
    /// potential source's, matching the contribution it was lifted from.
    is_current: bool,
}

/// One matrix's worth of stamps: the rows to write, and where each row's values
/// sit in the shared output list.
struct Stamps {
    rows: Vec<StampRow>,
    /// Parallel to `rows`: `(residual, one per surviving derivative)`, as
    /// indices into [`ModelPlan::outputs`].
    positions: Vec<(usize, Vec<usize>)>,
    /// Parallel to `rows`: where each row's limiter correction landed, for the
    /// rows that have one.
    corrections: Vec<Option<usize>>,
}

struct ModelPlan {
    /// One body computing both matrices' worth of values.
    ///
    /// Not two, and the corpus is what settled it: separate simplifications and
    /// separate emissions gave `hisimhv_va` 5.2 MB of stamp against a 2.2 MB
    /// whole body, because the charge in a wide MOSFET depends on nearly
    /// everything the conduction path computes and pruning has nothing to take
    /// away. Sharing is close to free in the other direction — the conduction
    /// Jacobian is `ddt_scale * d(q)/du`, so `d(q)/du` is already an operand it
    /// holds, and asking for it costs one lane read.
    function: CfgFunction,
    outputs: Vec<ValueId>,
    conduction: Stamps,
    /// Empty when no contribution stores charge.
    reactive: Stamps,
    /// The conduction body cut by invalidation class, or empty when the split
    /// was measured not to be worth taking for this model.
    stages: Vec<Stage>,
    slots: usize,
    node_count: usize,
    /// Branch-unknown ordinal per equation, for the potential stamps.
    branch_of_equation: Vec<Option<usize>>,
    /// The noise magnitudes, as their own body.
    ///
    /// `None` where the canonical level cannot express them and the generator
    /// being replaced can — which today is exactly a magnitude that reads `ddx`.
    /// Refusing the whole device over it would trade a working noise file for no
    /// device at all, so this one file falls back and the rest does not.
    noise: Option<NoisePlan>,
    /// One history slot per `ddt` in the body, allocated from the CFG.
    ///
    /// Not from `device::collect_ddt_slots`, and the reason is worth stating.
    /// That walks `mir.equations` and `hir.statements`; the CFG is lowered from
    /// `hir.body`, the structured region tree. The front end builds those from
    /// *separate copies* of the same expression tree — a two-terminal capacitor
    /// arena holds `ddt` twice, at ids 4 and 8 — so an operator id the CFG
    /// carries is not one that walk ever saw, and every lookup missed. The CFG
    /// is what this backend emits, so it is also what decides how many slots
    /// there are and which is which.
    ddt_slots: HashMap<ExprId, usize>,
    /// One history slot per `idt`, allocated from the CFG for the same reason.
    idt_slots: HashMap<ExprId, usize>,
    /// One anchor slot per `$limit`, holding what it returned on the previous
    /// Newton iteration. `Limit` and its `LimitPrevious` readers carry the same
    /// operator id, so they resolve to the same slot by construction.
    limit_slots: HashMap<ExprId, usize>,
}

impl ModelPlan {
    fn build(artifact: &CanonicalIrArtifact) -> Result<Self, RustBackendError> {
        let mut cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).map_err(|diagnostics| {
            let mut reasons: Vec<String> = diagnostics
                .iter()
                .map(|diagnostic| diagnostic.message.to_string())
                .collect();
            reasons.sort();
            reasons.dedup();
            reasons.truncate(4);
            unsupported(
                artifact,
                format!("the body does not lower to a CFG: {}", reasons.join("; ")),
            )
        })?;
        reject_unsupported_kinds(artifact, &cfg.function)?;

        // In value order, which is the lowering's order, so the numbering is a
        // property of the model rather than of a hash map's iteration.
        let mut ddt_slots: HashMap<ExprId, usize> = HashMap::new();
        let mut idt_slots: HashMap<ExprId, usize> = HashMap::new();
        let mut limit_slots: HashMap<ExprId, usize> = HashMap::new();
        for value in &cfg.function.values {
            match &value.kind {
                CfgValueKind::Ddt { operator, .. } => {
                    let next = ddt_slots.len();
                    ddt_slots.entry(*operator).or_insert(next);
                }
                CfgValueKind::Idt { operator, .. } => {
                    let next = idt_slots.len();
                    idt_slots.entry(*operator).or_insert(next);
                }
                // `LimitPrevious` is included so a `$limit` whose body reads the
                // previous iterate before the `Limit` value is built still finds
                // a slot; both carry the id of the same `$limit` call.
                CfgValueKind::Limit { operator, .. }
                | CfgValueKind::LimitPrevious { operator, .. } => {
                    let next = limit_slots.len();
                    limit_slots.entry(*operator).or_insert(next);
                }
                _ => {}
            }
        }

        // The correction lane goes last, and only where the model limits, so a
        // model without `$limit` carries no lane for it and every other lane
        // index still means "unknown number n".
        let limits = cfg
            .function
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::Limit { .. }));
        let seeds: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .chain(
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| AdSeed::BranchUnknownFlow(index.into())),
            )
            .chain(limits.then_some(AdSeed::LimiterCorrection))
            .collect();
        let correction_lane = limits.then(|| seeds.len() - 1);

        let residuals: Vec<ValueId> = artifact
            .mir
            .equations
            .iter()
            .map(|equation| cfg.residuals[usize::from(equation.contribution)])
            .collect();
        // Before differentiation, because a guarded charge is recovered by
        // *adding* a merge to the graph and a value added afterwards would have
        // no derivative. Differentiating a block parameter is the ordinary case,
        // so nothing else has to know this happened.
        let charges = stored_charges(&mut cfg.function, &residuals);
        cfg.function
            .validate()
            .map_err(|error| unsupported(artifact, format!("charge recovery: {error}")))?;

        let mut differentiated = differentiate(&cfg.function, &seeds)
            .map_err(|error| unsupported(artifact, format!("differentiation: {error}")))?;

        // Every read-out first, and both bodies' worth of them: taking a lane
        // appends an instruction, so a row taken after a simplification would
        // name values the simplified function does not have.
        let conduction_rows: Vec<Vec<Option<ValueId>>> = residuals
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

        // Every read-out is taken, so the function has stopped growing and the
        // noise slice can be cut from it.
        //
        // The primal body first, and the differentiated one only if that fails.
        // A magnitude wants no derivative, so cutting from the primal is what
        // keeps the slice to the arithmetic the powers actually name — but
        // `ddx` is a symbolic derivative only the AD pass resolves, and six
        // shipped models read one inside a noise power. Measured both ways over
        // the corpus: always-primal costs those six 5.4 MB of fallback, and
        // always-differentiated costs the other 35 more than it saves the six,
        // because the AD pass leaves bookkeeping the magnitudes can reach.
        let noise = plan_noise(artifact, &cfg, &cfg.function)
            .or_else(|| plan_noise(artifact, &cfg, &differentiated.function));

        // A contribution with no `ddt` stores no charge. Its row is kept and
        // emptied rather than dropped, so both row lists stay parallel to
        // `mir.equations` and an equation's index means the same thing in each.
        let charged = charges.iter().any(Option::is_some);
        let charge_values: Vec<ValueId> = charges
            .iter()
            .zip(&residuals)
            .map(|(charge, residual)| charge.unwrap_or(*residual))
            .collect();

        let mut conduction = plan_stamps(artifact, &residuals, &conduction_rows, correction_lane);
        // The reactive matrix stamps no residual, so a charge's correction lane
        // has nothing to correct: it is split out and dropped rather than
        // written.
        let mut reactive = plan_stamps(artifact, &charge_values, &reactive_rows, correction_lane);
        for row in &mut reactive.rows {
            row.correction = None;
        }
        for (index, charge) in charges.iter().enumerate() {
            if charge.is_none() {
                reactive.rows[index].derivatives.clear();
            }
        }
        if !charged {
            reactive.rows.clear();
        }

        // One simplification over both, so what the two matrices share is
        // computed once.
        let mut wanted = conduction.wanted();
        let reactive_wanted = reactive.wanted();
        wanted.extend_from_slice(&reactive_wanted);
        let (function, mapped) = optimize_cfg(&differentiated.function, &wanted);
        conduction.remap(&mapped[..wanted.len() - reactive_wanted.len()]);
        reactive.remap(&mapped[wanted.len() - reactive_wanted.len()..]);
        conduction.drop_zeros(&function);
        reactive.drop_zeros(&function);

        // The output list both stamps read from, conduction first.
        let mut outputs = Vec::new();
        let conduction = Stamps::place(conduction, &mut outputs);
        let reactive = Stamps::place(reactive, &mut outputs);

        let schedule = schedule(&function);
        let stages = split(&function, &schedule, &outputs)
            .map_err(|error| unsupported(artifact, format!("invalidation split: {error}")))?;
        let (stages, slots) = if worth_splitting(&function, &stages) {
            let slots = stages
                .iter()
                .flat_map(|stage| stage.exports.iter().map(|(slot, _)| *slot as usize + 1))
                .max()
                .unwrap_or(0);
            (stages, slots)
        } else {
            (Vec::new(), 0)
        };

        let branch_of_equation = artifact
            .mir
            .equations
            .iter()
            .map(|equation| {
                artifact
                    .mir
                    .branch_unknowns
                    .iter()
                    .find(|unknown| unknown.equation == equation.id)
                    .map(|unknown| usize::from(unknown.id))
            })
            .collect();

        Ok(Self {
            function,
            outputs,
            conduction,
            reactive,
            stages,
            slots,
            node_count: artifact.mir.nodes.len(),
            branch_of_equation,
            noise,
            ddt_slots,
            idt_slots,
            limit_slots,
        })
    }
}

/// Match the lowered noise sources to the plan the descriptors come from, and
/// reduce the body to just the magnitudes.
///
/// The correspondence is checked rather than assumed. A source is named by the
/// contribution it was written in and its position among that contribution's
/// sources, because the plan is extracted from a second lowering of the same
/// expressions and shares no expression ids with the body. Where the two
/// disagree this returns `None` rather than guessing: a noise source silently
/// given another source's power would be reported under the wrong mechanism at
/// the wrong branch, which reads as a physics result rather than as a compiler
/// fault. `None` costs the model nothing but the smaller file.
fn plan_noise(
    artifact: &CanonicalIrArtifact,
    cfg: &CfgModel,
    function: &CfgFunction,
) -> Option<NoisePlan> {
    let planned = &artifact.noise_sources.sources;
    // A silent model falls back too: the generator being replaced already emits
    // the empty evaluator for one, and this emitter has no reason to.
    if planned.is_empty() || cfg.noise.len() != planned.len() {
        return None;
    }

    let mut wanted = Vec::new();
    let mut sources = Vec::with_capacity(planned.len());
    for (index, source) in planned.iter().enumerate() {
        let contribution = artifact
            .mir
            .equations
            .get(usize::from(source.equation))
            .map(|equation| equation.contribution)?;
        let ordinal = planned[..index]
            .iter()
            .filter(|earlier| earlier.equation == source.equation)
            .count();
        let lowered = cfg
            .noise
            .iter()
            .find(|lowered| lowered.contribution == contribution && lowered.ordinal == ordinal)?;

        let table_width = source.table.as_ref().map_or(0, |table| table.operands.len());
        if lowered.kind != source.kind
            || lowered.exponent.is_some() != source.exponent.is_some()
            || lowered.table.len() != table_width
        {
            return None;
        }

        let mut place = |value: ValueId| {
            wanted.push(value);
            wanted.len() - 1
        };
        sources.push(NoiseSourceValues {
            active: place(lowered.active),
            psd: place(lowered.psd),
            exponent: lowered.exponent.map(&mut place),
            table: lowered.table.iter().copied().map(place).collect(),
            is_current: source.is_current,
        });
    }

    let (mut function, outputs) = optimize_cfg(function, &wanted);

    // One kind cannot appear in this body. `ddt` reads and writes per-instance
    // history and `evaluate_noise_sources` takes `&self`, so a magnitude that
    // depended on one would advance the transient state while the noise
    // analysis merely read it.
    //
    // Whether any `ddt` is *left* is not the question — a charge storing one is
    // the residual of its own contribution, and a residual can survive into a
    // slice on an edge nothing here reads. The question is whether a magnitude
    // reads one, and only reachability answers it.
    let live = reachable(&function, &outputs);
    for (index, value) in function.values.iter_mut().enumerate() {
        // `ddt` reads and writes per-instance history, and a magnitude that
        // reached one would advance the transient state while the noise
        // analysis merely read it. A `ddx` is unresolved, which only happens in
        // the primal body — returning `None` on one is what sends the model
        // round again against the differentiated function.
        if !matches!(
            value.kind,
            CfgValueKind::Ddt { .. } | CfgValueKind::Ddx { .. }
        ) {
            continue;
        }
        if live[index] {
            return None;
        }
        // Nothing the magnitudes read can observe this, by the walk just done,
        // so it becomes a constant rather than a call: left as it was, the body
        // would still advance the history of a charge no noise source mentions.
        value.kind = CfgValueKind::RealConstant(0.0);
    }
    Some(NoisePlan {
        function,
        outputs,
        sources,
    })
}

/// Every value `roots` can read, following a block parameter back through the
/// terminators that supply it.
fn reachable(function: &CfgFunction, roots: &[ValueId]) -> Vec<bool> {
    let mut declared: HashMap<ValueId, (usize, usize)> = HashMap::new();
    for (block, data) in function.blocks.iter().enumerate() {
        for (index, parameter) in data.params.iter().enumerate() {
            declared.insert(*parameter, (block, index));
        }
    }

    let mut live = vec![false; function.values.len()];
    let mut work: Vec<ValueId> = roots.to_vec();
    while let Some(value) = work.pop() {
        if std::mem::replace(&mut live[usize::from(value)], true) {
            continue;
        }
        work.extend(function.value(value).kind.operands());
        let Some((block, index)) = declared.get(&value).copied() else {
            continue;
        };
        // A merge reads whatever each edge into it carries, and — because which
        // edge was taken decides the answer — the condition that chose.
        for source in &function.blocks {
            match &source.terminator {
                CfgTerminator::Jump { target, args } if usize::from(*target) == block => {
                    work.extend(args.get(index).copied());
                }
                CfgTerminator::Branch {
                    condition,
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                } => {
                    let mut taken = false;
                    if usize::from(*then_target) == block {
                        work.extend(then_args.get(index).copied());
                        taken = true;
                    }
                    if usize::from(*else_target) == block {
                        work.extend(else_args.get(index).copied());
                        taken = true;
                    }
                    if taken {
                        work.push(*condition);
                    }
                }
                _ => {}
            }
        }
    }
    live
}

/// The rows one matrix writes, before simplification has run.
///
/// `values` is parallel to `mir.equations`, holding whichever quantity this
/// matrix stamps — the residual for conduction, the stored charge for the
/// reactive one.
fn plan_stamps(
    artifact: &CanonicalIrArtifact,
    values: &[ValueId],
    rows: &[Vec<Option<ValueId>>],
    correction_lane: Option<usize>,
) -> StampPlan {
    let mut plan = StampPlan {
        rows: Vec::with_capacity(artifact.mir.equations.len()),
        structurally_absent: 0,
        folded_to_zero: 0,
    };
    for (index, equation) in artifact.mir.equations.iter().enumerate() {
        let row = rows.get(index).cloned().unwrap_or_default();
        plan.rows.push(split_row(
            equation.branch.pos_node,
            equation.branch.neg_node,
            equation.kind,
            values[index],
            row,
            correction_lane,
            &mut plan.structurally_absent,
        ));
    }
    plan
}

impl Stamps {
    /// Append this matrix's values to the shared output list, recording where
    /// each row's landed.
    fn place(plan: StampPlan, outputs: &mut Vec<ValueId>) -> Self {
        let mut positions = Vec::with_capacity(plan.rows.len());
        let mut corrections = Vec::with_capacity(plan.rows.len());
        for row in &plan.rows {
            let residual = outputs.len();
            outputs.push(row.residual);
            let derivatives = row
                .derivatives
                .iter()
                .map(|(_, value)| {
                    outputs.push(*value);
                    outputs.len() - 1
                })
                .collect();
            corrections.push(row.correction.map(|value| {
                outputs.push(value);
                outputs.len() - 1
            }));
            positions.push((residual, derivatives));
        }
        Self {
            rows: plan.rows,
            positions,
            corrections,
        }
    }

    /// How many cached values the reactive stamp reads.
    fn width(&self) -> usize {
        self.rows
            .iter()
            .map(|row| 1 + row.derivatives.len())
            .sum()
    }
}

impl ModelPlan {
    fn stamp_file(
        &self,
        artifact: &CanonicalIrArtifact,
        options: &RustTranspileOptions,

    ) -> Result<String, RustBackendError> {
        let mut out = String::new();
        out.push_str(
            "#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]\n\n",
        );
        let _ = writeln!(
            out,
            "use super::state::Instance;\nuse {}::{{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};",
            options.runtime_path
        );
        out.push_str(RUNTIME_PRELUDE);
        out.push_str(EVAL_DDT);
        if !self.idt_slots.is_empty() {
            out.push_str(EVAL_IDT);
        }
        out.push_str("impl Instance {\n");

        for stage in &self.stages {
            if stage.class == InvalidationClass::Newton {
                continue;
            }
            self.emit_cached_stage(artifact, stage, &mut out)?;
        }
        self.emit_stamp(artifact, &mut out)?;
        self.emit_stamp_reactive(&mut out)?;

        out.push_str("}\n");
        Ok(out)
    }

    /// A stage coarser than Newton: run it once and cache what later readers
    /// take from it.
    fn emit_cached_stage(
        &self,
        artifact: &CanonicalIrArtifact,
        stage: &Stage,

        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let name = stage_fn_name(stage.class);
        let produced: Vec<ValueId> = stage.exports.iter().map(|(_, value)| *value).collect();
        let (body, names) = emit_body(&stage.function, &produced, &bindings())
            .map_err(|error| unsupported(artifact, format!("{name}: {error}")))?;

        let _ = writeln!(
            out,
            "    fn {name}(&mut self, ctx: &GeneratedEvalContext<'_>) {{"
        );
        match stage.class {
            InvalidationClass::Temperature => out.push_str(
                "        let temperature = ctx.temperature();\n\
                 \x20       let thermal_voltage = ctx.thermal_voltage();\n\
                 \x20       if self.canonical_temperature_valid\n\
                 \x20           && self.canonical_temperature == temperature\n\
                 \x20           && self.canonical_thermal_voltage == thermal_voltage\n\
                 \x20       {\n            return;\n        }\n",
            ),
            InvalidationClass::Instance => out
                .push_str("        if self.canonical_instance_valid {\n            return;\n        }\n"),
            // Nothing tells `stamp` that a new timestep began, so this one is
            // recomputed rather than cached.
            _ => {}
        }

        // Captured through a block so the immutable borrow of the slot array
        // ends before the writes into it begin.
        let _ = writeln!(
            out,
            "        let produced: [f64; {}] = {{",
            produced.len().max(1)
        );
        self.emit_prologue(artifact, &stage.function, 3, out)?;
        out.push_str(&indent(&body, 3));
        if produced.is_empty() {
            out.push_str("            [0.0]\n");
        } else {
            let _ = writeln!(out, "            [{}]", names.join(", "));
        }
        out.push_str("        };\n");
        for (index, (slot, _)) in stage.exports.iter().enumerate() {
            let _ = writeln!(
                out,
                "        self.canonical_staged[{slot}] = produced[{index}];"
            );
        }
        match stage.class {
            InvalidationClass::Temperature => out.push_str(
                "        self.canonical_temperature = temperature;\n\
                 \x20       self.canonical_thermal_voltage = thermal_voltage;\n\
                 \x20       self.canonical_temperature_valid = true;\n",
            ),
            InvalidationClass::Instance => {
                out.push_str("        self.canonical_instance_valid = true;\n")
            }
            _ => {}
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    fn emit_stamp(
        &self,
        artifact: &CanonicalIrArtifact,

        out: &mut String,
    ) -> Result<(), RustBackendError> {
        out.push_str(
            "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
        );
        // Cleared per evaluation, so "was this device limited?" is a question
        // about *this* iteration. Only when limiting is on: with it off the flag
        // is never set, and clearing it would erase a damped step recorded by
        // the iteration that is about to be re-judged.
        if !self.limit_slots.is_empty() {
            out.push_str(
                "        if ctx.limiting_enabled() {\n            self.canonical_limit.active = false;\n        }\n",
            );
        }
        for stage in &self.stages {
            if stage.class == InvalidationClass::Newton {
                continue;
            }
            let _ = writeln!(out, "        self.{}(ctx);", stage_fn_name(stage.class));
        }

        let newton = self
            .stages
            .iter()
            .find(|stage| stage.class == InvalidationClass::Newton);
        let function = newton.map_or(&self.function, |stage| &stage.function);
        let (body, values) = self.newton_outputs(artifact, newton)?;
        self.emit_prologue(artifact, function, 2, out)?;
        out.push_str(&indent(&body, 2));

        for (index, row) in self.conduction.rows.iter().enumerate() {
            let (residual, derivatives) = &self.conduction.positions[index];
            let residual = self.corrected_residual(index, &values, *residual);
            self.emit_row(
                row,
                &residual,
                &derivatives
                    .iter()
                    .map(|at| values[*at].clone())
                    .collect::<Vec<_>>(),
                index,
                Reactive::No,
                out,
            )?;
        }

        // The charge and its derivatives were computed here whether or not
        // anything asked for them, so this is where they are kept.
        for (index, (residual, derivatives)) in self.reactive.positions.iter().enumerate() {
            let mut at = self.reactive_base(index);
            let _ = writeln!(
                out,
                "        self.canonical_reactive[{at}] = {};",
                values[*residual]
            );
            for position in derivatives {
                at += 1;
                let _ = writeln!(
                    out,
                    "        self.canonical_reactive[{at}] = {};",
                    values[*position]
                );
            }
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    /// The reactive matrix, written from what `stamp` already worked out.
    ///
    /// No body at all, and that is the design rather than a shortcut. Two
    /// reasons it has to be this way:
    ///
    /// *Correctness.* An `eval_ddt` call reads and writes per-instance history,
    /// so a second evaluation from here would advance the transient state a
    /// second time for one solve. The tier being replaced avoids it by keeping
    /// the `ddt` calls out of what it shares; keeping no body avoids it
    /// outright.
    ///
    /// *Physics, and it agrees.* The reactive matrix is `d(charge)/d(unknown)`
    /// at the operating point, and an AC sweep holds that point fixed across
    /// every frequency. Recomputing it per point would produce the same numbers
    /// more slowly.
    fn emit_stamp_reactive(&self, out: &mut String) -> Result<(), RustBackendError> {
        out.push_str(
            "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
        );
        if self.reactive.rows.is_empty() {
            out.push_str("    }\n\n");
            return Ok(());
        }
        out.push_str("        let multiplicity = self.multiplicity;\n");
        out.push_str("        let cached = &*self.canonical_reactive;\n");
        for (index, row) in self.reactive.rows.iter().enumerate() {
            if row.derivatives.is_empty() {
                continue;
            }
            let base = self.reactive_base(index);
            let derivatives: Vec<String> = (1..=row.derivatives.len())
                .map(|offset| format!("cached[{}]", base + offset))
                .collect();
            self.emit_row(
                row,
                &format!("cached[{base}]"),
                &derivatives,
                index,
                Reactive::Yes,
                out,
            )?;
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    /// `noise.rs`: the descriptor table, then one body for every magnitude.
    fn noise_file(
        &self,
        artifact: &CanonicalIrArtifact,
        options: &RustTranspileOptions,
    ) -> Result<GeneratedRustFile, RustBackendError> {
        let Some(noise) = &self.noise else {
            return super::noise::generate_noise_file(artifact, options);
        };
        let function = &noise.function;
        let mut out = String::new();
        out.push_str(
            "#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]\n\n\
             use super::state::Instance;\n",
        );
        let _ = writeln!(
            out,
            "use {}::GeneratedEvalContext;\npub use {}::{{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor}};\n",
            options.runtime_path, options.runtime_path
        );
        // Emitted only where the body reaches something in it: on a model whose
        // magnitudes are plain arithmetic it would be a page of unused types in
        // every device. Two things reach it, and the second is what the corpus
        // found — a resolved `ddx` is a lane read, so a magnitude using one
        // needs the packed-value support `stamp.rs` carries, *and* the clamped
        // exponentials are functions in here rather than method calls, so a
        // magnitude with a diode in it needs the prelude with no packed value
        // anywhere. Twenty of the corpus models are the second case.
        if function.values.iter().any(|value| {
            value.value_type.shape().is_some()
                || matches!(
                    value.kind,
                    CfgValueKind::Unary {
                        op: CfgUnaryOp::LimExp
                            | CfgUnaryOp::LimitedExp
                            | CfgUnaryOp::LimitedExpDerivative,
                        ..
                    }
                )
        }) {
            out.push_str(RUNTIME_PRELUDE);
        }
        out.push_str(&super::noise::descriptor_table(artifact));
        out.push_str("\nimpl Instance {\n");
        out.push_str(
            "    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {\n\
             \x20       if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {\n\
             \x20           return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });\n\
             \x20       }\n",
        );
        let (body, values) = emit_body(function, &noise.outputs, &bindings())
            .map_err(|error| unsupported(artifact, format!("noise body: {error}")))?;
        self.emit_noise_prologue(artifact, function, &mut out);
        out.push_str(&indent(&body, 2));

        for (index, source) in noise.sources.iter().enumerate() {
            // The guard is the control flow the source was written in, already
            // merged into one value by the lowering. An inactive source still
            // has to be visited: the analysis pairs visits with descriptors by
            // index, so skipping one would shift every source after it.
            //
            // A source that no control flow guards merges to a constant, and
            // most do. Emitting the branch anyway would put an arm that cannot
            // run into every device that declares noise at all.
            let always = match function.value(noise.outputs[source.active]).kind {
                CfgValueKind::RealConstant(active) => Some(active != 0.0),
                _ => None,
            };
            if always == Some(false) {
                let _ = writeln!(
                    out,
                    "        if !visitor.visit({index}, GeneratedNoiseEvaluationRef {{ active: false, psd: 0.0, exponent: None, table_operands: &[] }}) {{ return Ok(()); }}"
                );
                continue;
            }
            if always.is_none() {
                let _ = writeln!(
                    out,
                    "        if {} == 0.0 {{\n\
                     \x20           if !visitor.visit({index}, GeneratedNoiseEvaluationRef {{ active: false, psd: 0.0, exponent: None, table_operands: &[] }}) {{ return Ok(()); }}\n\
                     \x20       }} else {{",
                    values[source.active]
                );
            } else {
                out.push_str("        {\n");
            }
            // The order of the checks, and the scaling written as one operation,
            // are the generator this replaces: same rejections, and a power that
            // is bit-identical rather than merely close.
            let _ = writeln!(out, "            let psd = {};", values[source.psd]);
            emit_noise_check(&mut out, index, "psd", "psd");
            let _ = writeln!(
                out,
                "            if psd < 0.0 {{ return Err(GeneratedNoiseEvaluationError::NegativePower {{ index: {index}, value: psd }}); }}"
            );
            match source.exponent {
                Some(at) => {
                    let _ = writeln!(
                        out,
                        "            let exponent: Option<f64> = Some({});",
                        values[at]
                    );
                    let _ = writeln!(
                        out,
                        "            if let Some(value) = exponent {{ if !value.is_finite() {{ return Err(GeneratedNoiseEvaluationError::NonFinite {{ index: {index}, quantity: \"exponent\", value }}); }} }}"
                    );
                }
                None => out.push_str("            let exponent: Option<f64> = None;\n"),
            }
            for (operand, at) in source.table.iter().enumerate() {
                let _ = writeln!(
                    out,
                    "            let noise_table_operand_{operand} = {};",
                    values[*at]
                );
                emit_noise_check(
                    &mut out,
                    index,
                    &format!("table operand {operand}"),
                    &format!("noise_table_operand_{operand}"),
                );
            }
            let _ = writeln!(
                out,
                "            let table_operands = [{}];",
                (0..source.table.len())
                    .map(|operand| format!("noise_table_operand_{operand}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let scaled = if source.is_current {
                "psd * self.multiplicity"
            } else {
                "psd / self.multiplicity"
            };
            let _ = writeln!(out, "            let psd = {scaled};");
            emit_noise_check(&mut out, index, "scaled psd", "psd");
            let _ = writeln!(
                out,
                "            if !visitor.visit({index}, GeneratedNoiseEvaluationRef {{ active: true, psd, exponent, table_operands: &table_operands }}) {{ return Ok(()); }}\n\
                 \x20       }}"
            );
        }
        out.push_str("        Ok(())\n    }\n}\n");

        Ok(GeneratedRustFile {
            relative_path: "noise.rs".to_string(),
            contents: out,
        })
    }

    /// The noise body's bindings.
    ///
    /// Not [`Self::emit_prologue`]: that one forces `multiplicity` in because
    /// every stamper call scales by it, and binds the slot array because a
    /// stamp can read a slot the body never touches. Neither is true here —
    /// this body stamps nothing and runs unstaged — and an unused binding of
    /// `self.canonical_staged` would not even compile on a model that has none.
    fn emit_noise_prologue(
        &self,
        artifact: &CanonicalIrArtifact,
        function: &CfgFunction,
        out: &mut String,
    ) {
        let mut wants = Wants::default();
        for value in &function.values {
            wants.observe(&value.kind);
        }
        if wants.parameters {
            out.push_str("        let parameters = &self.params.values;\n");
        }
        if wants.parameter_given {
            out.push_str("        let parameter_given = &*self.param_given;\n");
        }
        if wants.multiplicity {
            out.push_str("        let multiplicity = self.multiplicity;\n");
        }
        if wants.time {
            out.push_str("        let time = self.time;\n");
        }
        if wants.temperature {
            out.push_str("        let temperature = ctx.temperature();\n");
        }
        if wants.thermal_voltage {
            out.push_str("        let thermal_voltage = ctx.thermal_voltage();\n");
        }
        if wants.node_potentials {
            let _ = writeln!(
                out,
                "        let node_potentials = [{}];",
                (0..self.node_count)
                    .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.branch_unknown_flows {
            let _ = writeln!(
                out,
                "        let branch_unknown_flows = [{}];",
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| format!("ctx.branch_current(self.branches[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.ddt_scale {
            out.push_str(
                "        let ddt_scale_value = self.ddt_coefficients.derivative_scale;\n\
                 \x20       let ddt_scale = move || ddt_scale_value;\n",
            );
        }
        // `evaluate_noise_sources` takes `&self`, runs at a fixed operating
        // point, and is not a Newton step — there is nothing to damp and no
        // history it may write. Both bindings are therefore the
        // limiting-disabled ones, which is what the device itself does when
        // `ctx.limiting_enabled()` is false, so a magnitude that reads a limited
        // voltage reads the proposed one.
        if wants.limit {
            out.push_str(
                "        let limit = |_operator: usize, proposed: f64, _candidate: f64| proposed;\n",
            );
        }
        if wants.limit_previous {
            out.push_str("        let limit_previous = |_operator: usize, proposed: f64| proposed;\n");
        }
    }

    /// Where equation `index`'s reactive values start in the cache.
    fn reactive_base(&self, index: usize) -> usize {
        self.reactive.rows[..index]
            .iter()
            .map(|row| 1 + row.derivatives.len())
            .sum()
    }

    /// The Newton body, and an expression per conduction output.
    ///
    /// When the body is split, an output a coarser stage owns is read from its
    /// slot rather than recomputed, which is the whole point of the split.
    fn newton_outputs(
        &self,
        artifact: &CanonicalIrArtifact,
        newton: Option<&Stage>,
    ) -> Result<(String, Vec<String>), RustBackendError> {
        let Some(newton) = newton else {
            let (body, names) = emit_body(
                &self.function,
                &self.outputs,
                &bindings(),
            )
            .map_err(|error| unsupported(artifact, format!("body: {error}")))?;
            return Ok((body, names));
        };

        let owned: Vec<(usize, ValueId)> = newton
            .outputs
            .iter()
            .enumerate()
            .filter_map(|(index, value)| value.map(|value| (index, value)))
            .collect();
        let (body, names) = emit_body(
            &newton.function,
            &owned.iter().map(|(_, value)| *value).collect::<Vec<_>>(),
            &bindings(),
        )
        .map_err(|error| unsupported(artifact, format!("newton stage: {error}")))?;

        let mut values = vec![String::new(); self.outputs.len()];
        for ((index, _), name) in owned.iter().zip(names) {
            values[*index] = name;
        }
        for (index, value) in values.iter_mut().enumerate() {
            if !value.is_empty() {
                continue;
            }
            let slot = self
                .stages
                .iter()
                .find_map(|stage| stage.outputs[index].and_then(|held| stage.slot_of(held)))
                .ok_or_else(|| {
                    // `split` is supposed to make this unreachable by demanding
                    // every output at the deepest class; if it ever is reached,
                    // the alternative is a silent zero in the matrix.
                    unsupported(
                        artifact,
                        format!("stamp output {index} is computed by no stage and cached by none"),
                    )
                })?;
            *value = format!("staged[{slot}]");
        }
        Ok((body, values))
    }

    /// One equation's stamper calls.
    /// The residual as the matrix wants it, with the limiter's displacement
    /// taken back out.
    ///
    /// The model evaluated its currents at the limited operating point `L`
    /// while the solver is asking about `v`, and the Jacobian reported is
    /// `dI/dv` at `L` by the `dL/dv := 1` convention. Linearising there gives
    /// `I(L) + G*(v - L)`, so what has to be subtracted is `G*(L - v)` — the
    /// directional derivative along the displacement, which is exactly what the
    /// correction lane accumulated. Any `ddt` scaling is already inside it,
    /// because this pass differentiates the residual rather than the charge.
    ///
    /// Guarded on `limiting_enabled` because a probe with limiting off must see
    /// the undamped equations: that is the mode the derivative oracles measure,
    /// and correcting there would make the stamp disagree with its own
    /// currents.
    fn corrected_residual(&self, equation: usize, values: &[String], residual: usize) -> String {
        match self.conduction.corrections.get(equation).copied().flatten() {
            Some(at) => format!(
                "(({}) - (if ctx.limiting_enabled() {{ {} }} else {{ 0.0 }}))",
                values[residual], values[at]
            ),
            None => values[residual].clone(),
        }
    }

    fn emit_row(
        &self,
        row: &StampRow,
        residual: &str,
        derivatives: &[String],
        equation: usize,
        reactive: Reactive,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let mut nodes = Vec::new();
        let mut node_values = Vec::new();
        let mut branches = Vec::new();
        let mut branch_values = Vec::new();
        for ((unknown, _), value) in row.derivatives.iter().zip(derivatives) {
            if *unknown < self.node_count {
                nodes.push(unknown.to_string());
                node_values.push(value.clone());
            } else {
                branches.push((unknown - self.node_count).to_string());
                branch_values.push(value.clone());
            }
        }
        let pos = optional_node(row.pos);
        let neg = optional_node(row.neg);

        match (row.kind, reactive) {
            (MirEquationKind::Current, Reactive::No) => {
                let _ = writeln!(
                    out,
                    "        stamper.stamp_current_sparse_local::<{}, {}>(\n\
                     \x20           {pos},\n            {neg},\n            multiplicity * ({residual}),\n\
                     \x20           [{}],\n            [{}],\n            [{}],\n            [{}],\n\
                     \x20           multiplicity,\n        );",
                    nodes.len(),
                    branches.len(),
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Current, Reactive::Yes) => {
                let _ = writeln!(
                    out,
                    "        stamper.stamp_current_reactive_indexed_dense_local(\n\
                     \x20           {pos},\n            {neg},\n            &[{}],\n            &[{}],\n\
                     \x20           &[{}],\n            &[{}],\n            multiplicity,\n        );",
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Potential, Reactive::No) => {
                let branch = self.branch_of_equation[equation].ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("potential equation {equation} has no branch unknown"),
                    )
                })?;
                let _ = writeln!(
                    out,
                    "        stamper.stamp_potential_branch_local({pos}, {neg}, {branch}, multiplicity);"
                );
                let _ = writeln!(
                    out,
                    "        stamper.stamp_potential_sparse_local::<{}, {}>(\n\
                     \x20           {branch},\n            {residual},\n\
                     \x20           [{}],\n            [{}],\n            [{}],\n            [{}],\n        );",
                    nodes.len(),
                    branches.len(),
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Potential, Reactive::Yes) => {
                let branch = self.branch_of_equation[equation].ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("potential equation {equation} has no branch unknown"),
                    )
                })?;
                let _ = writeln!(
                    out,
                    "        stamper.stamp_potential_reactive_indexed_dense_local(\n\
                     \x20           {branch},\n            &[{}],\n            &[{}],\n\
                     \x20           &[{}],\n            &[{}],\n        );",
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Indirect, _) => {}
        }
        Ok(())
    }

    /// Everything an emitted body expects to find in scope.
    ///
    /// Only what the body actually reads: a leaf the function does not carry
    /// would otherwise emit a `ctx` call for a quantity nothing wants, and in
    /// the instance stage there is no bias to read it from.
    fn emit_prologue(
        &self,
        artifact: &CanonicalIrArtifact,
        function: &CfgFunction,
        depth: usize,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let pad = "    ".repeat(depth);
        let mut wants = Wants::default();
        for value in &function.values {
            wants.observe(&value.kind);
        }
        // Every stamper call scales by it, whether or not the body reads it.
        wants.multiplicity = true;
        // And a stamper call can read a slot the body never touches: an output
        // a coarse stage owns is written straight into the stamp as
        // `staged[..]`. `I(a, c) <+ bias` in a model that also splits is the
        // shape — the Newton body has no staged operand at all, and the slot
        // still has to be in scope.
        wants.staged |= self.slots > 0;

        if wants.parameters {
            let _ = writeln!(out, "{pad}let parameters = &self.params.values;");
        }
        if wants.parameter_given {
            let _ = writeln!(out, "{pad}let parameter_given = &*self.param_given;");
        }
        if wants.multiplicity {
            let _ = writeln!(out, "{pad}let multiplicity = self.multiplicity;");
        }
        if wants.time {
            let _ = writeln!(out, "{pad}let time = self.time;");
        }
        if wants.temperature {
            let _ = writeln!(out, "{pad}let temperature = ctx.temperature();");
        }
        if wants.thermal_voltage {
            let _ = writeln!(out, "{pad}let thermal_voltage = ctx.thermal_voltage();");
        }
        if wants.staged {
            let _ = writeln!(out, "{pad}let staged = &*self.canonical_staged;");
        }
        if wants.node_potentials {
            let _ = writeln!(
                out,
                "{pad}let node_potentials = [{}];",
                (0..self.node_count)
                    .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.branch_unknown_flows {
            let _ = writeln!(
                out,
                "{pad}let branch_unknown_flows = [{}];",
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| format!("ctx.branch_current(self.branches[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.ddt_scale {
            let _ = writeln!(
                out,
                "{pad}let ddt_scale_value = self.ddt_coefficients.derivative_scale;"
            );
            let _ = writeln!(out, "{pad}let ddt_scale = move || ddt_scale_value;");
        }
        if wants.idt_scale {
            // Zero where there is no step, which is what makes an integral
            // contribute nothing to the Jacobian at an operating point rather
            // than an arbitrary multiple of the last timestep.
            let _ = writeln!(
                out,
                "{pad}let idt_scale_value = if self.ddt_coefficients.active {{ self.timestep }} else {{ 0.0 }};\n\
                 {pad}let idt_scale = move || idt_scale_value;"
            );
        }
        // One binding for both operators, and before either closure is built.
        // `idt` used to bind it itself unless `ddt` was going to, which put the
        // binding *after* the closure that read it whenever a model had both —
        // and only PSP-NQS has both, so nothing caught it until the corpus was
        // compiled. The two closures reach disjoint fields through it.
        if wants.ddt || wants.idt {
            let _ = writeln!(out, "{pad}let ddt_state = self.stamp_state.as_mut();");
        }
        if wants.idt {
            let mut arms: Vec<String> = Vec::new();
            for value in &function.values {
                if let CfgValueKind::Idt { operator, .. } = &value.kind {
                    let slot = self.idt_slots.get(operator).copied().ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!("an idt at {operator} with no generated state slot"),
                        )
                    })?;
                    let arm = format!("{} => {slot}usize, ", usize::from(*operator));
                    if !arms.contains(&arm) {
                        arms.push(arm);
                    }
                }
            }
            // The same out-of-range fallback `ddt` uses, and for the same
            // reason: integrating into the wrong history looks like a converged
            // answer.
            let resolve = match arms.len() {
                1 => arms[0]
                    .split_once("=> ")
                    .map(|(_, slot)| slot.trim_end_matches(", ").to_string())
                    .unwrap_or_else(|| "usize::MAX".to_string()),
                _ => format!("match operator {{ {}_ => usize::MAX }}", arms.concat()),
            };
            let _ = writeln!(
                out,
                "{pad}let idt_active = self.ddt_coefficients.active;\n\
                 {pad}let idt_step = self.timestep;\n\
                 {pad}let mut idt = |operator: usize, value: f64, ic: f64| -> f64 {{\n\
                 {pad}    let _ = operator;\n\
                 {pad}    let slot = {resolve};\n\
                 {pad}    rspice_eval_idt(\n\
                 {pad}        &mut ddt_state.idt_current,\n\
                 {pad}        &mut ddt_state.idt_previous,\n\
                 {pad}        &mut ddt_state.idt_initialized,\n\
                 {pad}        idt_active,\n\
                 {pad}        idt_step,\n\
                 {pad}        slot,\n\
                 {pad}        value,\n\
                 {pad}        ic,\n\
                 {pad}    )\n\
                 {pad}}};"
            );
        }
        if wants.ddt {
            // `ddt` is the one binding that is a call rather than an expression,
            // because it reads and writes per-instance history. The operator id
            // the CFG carries is a source expression, so the slot it was
            // assigned is resolved here rather than looked up at run time.
            let mut arms: Vec<String> = Vec::new();
            for value in &function.values {
                if let CfgValueKind::Ddt { operator, .. } = &value.kind {
                    let slot = self.ddt_slots.get(operator).copied().ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!("a ddt at {operator} with no generated state slot"),
                        )
                    })?;
                    let arm = format!("{} => {slot}usize, ", usize::from(*operator));
                    if !arms.contains(&arm) {
                        arms.push(arm);
                    }
                }
            }
            // The arms cover every `ddt` the body holds, so the fallback is
            // unreachable — and it resolves to an out-of-range slot rather than
            // to slot zero, because if the invariant ever breaks, integrating a
            // charge into the wrong history is the one failure that would look
            // like a converged answer.
            let resolve = match arms.len() {
                1 => arms[0]
                    .split_once("=> ")
                    .map(|(_, slot)| slot.trim_end_matches(", ").to_string())
                    .unwrap_or_else(|| "usize::MAX".to_string()),
                _ => format!("match operator {{ {}_ => usize::MAX }}", arms.concat()),
            };
            let _ = writeln!(
                out,
                "{pad}let ddt_active = self.ddt_coefficients.active;\n\
                 {pad}let ddt_coefficients = self.ddt_coefficients;\n\
                 {pad}let mut ddt = |operator: usize, value: f64| -> f64 {{\n\
                 {pad}    let _ = operator;\n\
                 {pad}    let slot = {resolve};\n\
                 {pad}    rspice_eval_ddt(\n\
                 {pad}        &mut ddt_state.ddt_current,\n\
                 {pad}        &mut ddt_state.ddt_previous,\n\
                 {pad}        &mut ddt_state.ddt_older,\n\
                 {pad}        &mut ddt_state.ddt_initialized,\n\
                 {pad}        &mut ddt_state.ddt_derivative_current,\n\
                 {pad}        &mut ddt_state.ddt_derivative_previous,\n\
                 {pad}        ddt_active,\n\
                 {pad}        ddt_coefficients.derivative_scale,\n\
                 {pad}        ddt_coefficients.previous_value_scale,\n\
                 {pad}        ddt_coefficients.older_value_scale,\n\
                 {pad}        ddt_coefficients.previous_derivative_scale,\n\
                 {pad}        slot,\n\
                 {pad}        value,\n\
                 {pad}    )\n\
                 {pad}}};"
            );
        }
        if wants.limit || wants.limit_previous {
            let resolve = self.limit_slot_resolver(artifact, function)?;
            // The anchors are *copied* out before any write, which is what makes
            // "the value this `$limit` returned on the previous iteration" mean
            // that regardless of emission order. Reading the live array instead
            // would hand a `$limit` its own current value the moment a body read
            // the previous iterate after the limiter had already run, and
            // arrays this small are one register-width memcpy.
            //
            // It also settles the borrow: the reader closes over copies, so the
            // writer can hold the state mutably without the two colliding.
            let _ = writeln!(
                out,
                "{pad}let limiting_enabled = ctx.limiting_enabled();\n\
                 {pad}let limit_state = self.canonical_limit.as_mut();\n\
                 {pad}let limit_anchor = limit_state.previous;\n\
                 {pad}let limit_initialized = limit_state.initialized;"
            );
            if wants.limit_previous {
                let _ = writeln!(
                    out,
                    "{pad}let limit_previous = move |operator: usize, proposed: f64| -> f64 {{\n\
                     {pad}    let _ = operator;\n\
                     {pad}    let slot = {resolve};\n\
                     {pad}    if limiting_enabled && limit_initialized[slot] {{\n\
                     {pad}        limit_anchor[slot]\n\
                     {pad}    }} else {{\n\
                     {pad}        proposed\n\
                     {pad}    }}\n\
                     {pad}}};"
                );
            }
            if wants.limit {
                let _ = writeln!(
                    out,
                    "{pad}let mut limit = |operator: usize, proposed: f64, candidate: f64| -> f64 {{\n\
                     {pad}    let _ = operator;\n\
                     {pad}    if !limiting_enabled {{\n\
                     {pad}        return proposed;\n\
                     {pad}    }}\n\
                     {pad}    let slot = {resolve};\n\
                     {pad}    limit_state.active |= candidate != proposed;\n\
                     {pad}    limit_state.previous[slot] = candidate;\n\
                     {pad}    limit_state.initialized[slot] = true;\n\
                     {pad}    candidate\n\
                     {pad}}};"
                );
            }
        }
        Ok(())
    }

    /// A `match` from an operator id to the state slot it was assigned, or the
    /// bare slot when there is only one.
    ///
    /// The fallback resolves to an out-of-range slot rather than to zero, for
    /// the reason `ddt` gives: reading or writing the wrong history is the one
    /// failure that looks like a converged answer.
    fn limit_slot_resolver(
        &self,
        artifact: &CanonicalIrArtifact,
        function: &CfgFunction,
    ) -> Result<String, RustBackendError> {
        let mut arms: Vec<String> = Vec::new();
        for value in &function.values {
            let (CfgValueKind::Limit { operator, .. }
            | CfgValueKind::LimitPrevious { operator, .. }) = &value.kind
            else {
                continue;
            };
            let operator = *operator;
            let slot = self.limit_slots.get(&operator).copied().ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("a $limit at {operator} with no generated state slot"),
                )
            })?;
            let arm = format!("{} => {slot}usize, ", usize::from(operator));
            if !arms.contains(&arm) {
                arms.push(arm);
            }
        }
        Ok(match arms.len() {
            1 => arms[0]
                .split_once("=> ")
                .map(|(_, slot)| slot.trim_end_matches(", ").to_string())
                .unwrap_or_else(|| "usize::MAX".to_string()),
            _ => format!("match operator {{ {}_ => usize::MAX }}", arms.concat()),
        })
    }

    fn state_extensions(&self) -> device::StateFileExtensions {
        let mut extensions = device::StateFileExtensions::default();
        self.push_limit_state_fields(&mut extensions);
        if self.slots > 0 || self.reactive.width() > 0 {
            extensions.support_types.push_str(
                "fn canonical_boxed_zero_f64<const N: usize>() -> Box<[f64; N]> {\n\
                 \x20   // SAFETY: every slot is an f64, and all-zero bytes are 0.0.\n\
                 \x20   let mut boxed = Box::<[f64; N]>::new_uninit();\n\
                 \x20   unsafe {\n\
                 \x20       std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n\
                 \x20       boxed.assume_init()\n\
                 \x20   }\n\
                 }\n\n",
            );
        }
        let reactive = self.reactive.width();
        if reactive > 0 {
            let _ = write!(
                extensions.instance_fields,
                "    pub(crate) canonical_reactive: Box<[f64; {reactive}]>,\n"
            );
            extensions
                .clone_fields
                .push_str("            canonical_reactive: self.canonical_reactive.clone(),\n");
            extensions
                .new_initializers
                .push_str("            canonical_reactive: canonical_boxed_zero_f64(),\n");
        }
        if self.slots == 0 {
            return extensions;
        }
        let slots = self.slots;
        let _ = write!(
            extensions.instance_fields,
            "    pub(crate) canonical_staged: Box<[f64; {slots}]>,\n\
             \x20   pub(crate) canonical_instance_valid: bool,\n\
             \x20   pub(crate) canonical_temperature_valid: bool,\n\
             \x20   pub(crate) canonical_temperature: f64,\n\
             \x20   pub(crate) canonical_thermal_voltage: f64,\n"
        );
        extensions.clone_fields.push_str(
            "            canonical_staged: self.canonical_staged.clone(),\n\
             \x20           canonical_instance_valid: self.canonical_instance_valid,\n\
             \x20           canonical_temperature_valid: self.canonical_temperature_valid,\n\
             \x20           canonical_temperature: self.canonical_temperature,\n\
             \x20           canonical_thermal_voltage: self.canonical_thermal_voltage,\n",
        );
        extensions.new_initializers.push_str(
            "            canonical_staged: canonical_boxed_zero_f64(),\n\
             \x20           canonical_instance_valid: false,\n\
             \x20           canonical_temperature_valid: false,\n\
             \x20           canonical_temperature: 0.0,\n\
             \x20           canonical_thermal_voltage: 0.0,\n",
        );
        // A parameter write invalidates both caches, because a parameter is
        // read by every class.
        extensions.set_parameter_hook.push_str(
            "self.canonical_instance_valid = false;\nself.canonical_temperature_valid = false;\n",
        );
        extensions
    }

    /// Per-instance limiter state: the anchor each `$limit` returned last, and
    /// whether the device was limited at all on this iteration.
    ///
    /// The `active` flag is the engine's, not this backend's — it is how a
    /// device says "do not call this converged, the step I was given was
    /// damped" — so the rollback and checkpoint wiring around it matches the
    /// tier being replaced field for field. Getting that wrong does not fail to
    /// compile; it converges early.
    fn push_limit_state_fields(&self, extensions: &mut device::StateFileExtensions) {
        let count = self.limit_slots.len();
        if count == 0 {
            return;
        }
        extensions.support_types.push_str(
            "#[derive(Clone)]\n\
             pub(crate) struct CanonicalLimitState<const N: usize> {\n\
             \x20   pub(crate) previous: [f64; N],\n\
             \x20   pub(crate) initialized: [bool; N],\n\
             \x20   pub(crate) active: bool,\n\
             }\n\n\
             impl<const N: usize> CanonicalLimitState<N> {\n\
             \x20   fn new_box() -> Box<Self> {\n\
             \x20       let mut boxed = Box::<Self>::new_uninit();\n\
             \x20       unsafe {\n\
             \x20           // SAFETY: every field is an f64, a bool, or an array of them; all-zero bytes are valid values.\n\
             \x20           std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n\
             \x20           boxed.assume_init()\n\
             \x20       }\n\
             \x20   }\n\
             }\n\n",
        );
        let _ = write!(
            extensions.instance_fields,
            "    pub(crate) canonical_limit: Box<CanonicalLimitState<{count}>>,\n"
        );
        extensions
            .clone_fields
            .push_str("            canonical_limit: self.canonical_limit.clone(),\n");
        extensions
            .new_initializers
            .push_str("            canonical_limit: CanonicalLimitState::new_box(),\n");
        extensions.limiter_converged_expr = "!self.canonical_limit.active".to_string();
        extensions.rollback_value_count = count;
        extensions.rollback_flag_count = count + 1;
        extensions
            .rollback_capture_values
            .push_str("        values.extend_from_slice(&self.canonical_limit.previous);\n");
        extensions.rollback_capture_flags.push_str(
            "        flags.extend_from_slice(&self.canonical_limit.initialized);\n\
             \x20       flags.push(self.canonical_limit.active);\n",
        );
        let _ = write!(
            extensions.rollback_restore_fields,
            "        let (field, remaining) = rollback_values.split_at({count});\n\
             \x20       self.canonical_limit.previous.copy_from_slice(field);\n\
             \x20       rollback_values = remaining;\n\
             \x20       let (field, remaining) = rollback_flags.split_at({count});\n\
             \x20       self.canonical_limit.initialized.copy_from_slice(field);\n\
             \x20       rollback_flags = remaining;\n\
             \x20       let (active, remaining) = rollback_flags.split_first().expect(\"generated limiter rollback active flag\");\n\
             \x20       self.canonical_limit.active = *active;\n\
             \x20       rollback_flags = remaining;\n"
        );
        extensions.checkpoint_capture_fields =
            "            limiter_anchor: self.canonical_limit.previous.to_vec(),\n\
             \x20           limiter_initialized: self.canonical_limit.initialized.to_vec(),\n"
                .to_string();
        let _ = write!(
            extensions.checkpoint_shape_checks,
            "        if state.limiter_anchor.len() != {count} || state.limiter_initialized.len() != {count} {{\n\
             \x20           return Err(format!(\"generated limiter checkpoint shape mismatch: expected {count}, found {{}} / {{}}\", state.limiter_anchor.len(), state.limiter_initialized.len()));\n\
             \x20       }}\n"
        );
        // Restored state is a fresh operating point, not a damped step, so the
        // flag starts clear rather than being carried across.
        extensions.checkpoint_restore_fields.push_str(
            "        self.canonical_limit.previous.copy_from_slice(&state.limiter_anchor);\n\
             \x20       self.canonical_limit.initialized.copy_from_slice(&state.limiter_initialized);\n\
             \x20       self.canonical_limit.active = false;\n",
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Reactive {
    No,
    Yes,
}

/// Which bindings a body actually reads.
#[derive(Default)]
struct Wants {
    parameters: bool,
    parameter_given: bool,
    node_potentials: bool,
    branch_unknown_flows: bool,
    temperature: bool,
    thermal_voltage: bool,
    multiplicity: bool,
    time: bool,
    ddt: bool,
    ddt_scale: bool,
    idt: bool,
    idt_scale: bool,
    limit: bool,
    limit_previous: bool,
    staged: bool,
}

impl Wants {
    fn observe(&mut self, kind: &CfgValueKind) {
        match kind {
            CfgValueKind::Parameter(_) => self.parameters = true,
            CfgValueKind::ParameterGiven(_) => self.parameter_given = true,
            CfgValueKind::NodePotential(_) => self.node_potentials = true,
            CfgValueKind::BranchUnknownFlow(_) => self.branch_unknown_flows = true,
            CfgValueKind::Temperature => self.temperature = true,
            CfgValueKind::ThermalVoltage => self.thermal_voltage = true,
            CfgValueKind::Multiplicity => self.multiplicity = true,
            CfgValueKind::Time => self.time = true,
            CfgValueKind::Ddt { .. } => self.ddt = true,
            CfgValueKind::DdtScale => self.ddt_scale = true,
            CfgValueKind::Idt { .. } => self.idt = true,
            CfgValueKind::IdtScale => self.idt_scale = true,
            CfgValueKind::Limit { .. } => self.limit = true,
            CfgValueKind::LimitPrevious { .. } => self.limit_previous = true,
            CfgValueKind::Staged { .. } => self.staged = true,
            _ => {}
        }
    }
}

/// The charge a contribution stores, if it is a `ddt` and nothing else.
///
/// A residual is an accumulator, so `I(a, b) <+ ddt(q)` arrives as `0 + ddt(q)`
/// and simplification has not run yet — the zero is peeled here rather than
/// relied on. What is deliberately *not* accepted is a residual mixing stored
/// charge with conduction in one statement: separating those needs the reactive
/// part tracked through the arithmetic, and calling the whole expression a
/// charge would put conduction into the reactive matrix.
fn stored_charges(function: &mut CfgFunction, residuals: &[ValueId]) -> Vec<Option<ValueId>> {
    let reaches = values_reaching_a_ddt(function);
    let mut insertions: Vec<(ValueId, ValueId)> = Vec::new();
    let charges: Vec<Option<ValueId>> = residuals
        .iter()
        .map(|residual| {
            resolve_charge(function, &reaches, *residual, 0)
                .and_then(|charge| materialise_charge(function, &charge, &mut insertions))
        })
        .collect();
    // Once, at the end: an instruction list cannot be rebuilt while it is still
    // being read for the next residual, and two residuals routinely share a
    // subexpression.
    apply_insertions(function, &insertions);
    charges
}

/// Splice each built instruction in directly after the one it mirrors.
fn apply_insertions(function: &mut CfgFunction, insertions: &[(ValueId, ValueId)]) {
    if insertions.is_empty() {
        return;
    }
    let mut after: HashMap<ValueId, Vec<ValueId>> = HashMap::new();
    for (anchor, value) in insertions {
        after.entry(*anchor).or_default().push(*value);
    }
    for block in &mut function.blocks {
        if !block
            .instructions
            .iter()
            .any(|instruction| after.contains_key(&instruction.result))
        {
            continue;
        }
        let mut rebuilt = Vec::with_capacity(block.instructions.len() + insertions.len());
        for instruction in std::mem::take(&mut block.instructions) {
            let anchor = instruction.result;
            rebuilt.push(instruction);
            if let Some(values) = after.get(&anchor) {
                rebuilt.extend(values.iter().map(|result| CfgInstruction { result: *result }));
            }
        }
        block.instructions = rebuilt;
    }
}

impl Charge {
    fn is_nothing(&self) -> bool {
        matches!(self, Self::Nothing)
    }
}

/// How deep a chain of merges to follow before giving up.
///
/// A charge behind more than a handful of nested guards is not a shape any
/// released model has, and a bound is what keeps a loop-carried residual — where
/// a block parameter can reach itself — from recursing forever.
const MAX_CHARGE_MERGE_DEPTH: usize = 8;

/// What a residual stores, worked out before any of it is built.
///
/// Resolution is separated from construction because a merge has to be created
/// bottom-up: the parameter that carries a guarded charge can only be added once
/// every arm's charge exists to be passed on the edges.
enum Charge {
    /// A value the graph already holds — the operand of a `ddt`, or one side of
    /// an operation that carries no charge of its own.
    Value(ValueId),
    /// This path stores nothing. At the top that means the contribution is not
    /// reactive at all; inside a merge it is the arm that was not taken, and
    /// inside a sum it is the conduction half.
    Nothing,
    /// One charge per edge into `block`, in the order [`edges_into`] gives them.
    Merge { block: BlockId, arms: Vec<Charge> },
    /// An operation the charge needs that the graph only has in its `ddt` form.
    ///
    /// `I(db) <+ TYPE * ddt(QD)` stores `TYPE * QD`, and that product exists
    /// nowhere until it is built. It is inserted directly after `anchor` — the
    /// instruction it mirrors — so its operands are in scope exactly where the
    /// original's were, without any dominance question to answer.
    Op {
        anchor: ValueId,
        kind: Box<ChargeOp>,
    },
}

enum ChargeOp {
    Binary {
        op: CfgBinaryOp,
        left: Charge,
        right: Charge,
    },
    Unary {
        op: CfgUnaryOp,
        input: Charge,
    },
}

/// Which values can reach a `ddt` at all.
///
/// A fixed point rather than one pass, because a loop-carried block parameter
/// depends on values computed from itself. It exists so resolution can answer
/// "stores nothing" in O(1) instead of walking a residual's whole expression
/// DAG as a tree — which on a compact model is both exponential and, if it were
/// bounded to stop that, silently wrong for anything deeper than the bound.
fn values_reaching_a_ddt(function: &CfgFunction) -> Vec<bool> {
    let incoming = {
        let mut incoming: HashMap<ValueId, Vec<ValueId>> = HashMap::new();
        for block in &function.blocks {
            for (target, args) in outgoing_edges(block) {
                for (param, argument) in function.block(target).params.iter().zip(args) {
                    incoming.entry(*param).or_default().push(argument);
                }
            }
        }
        incoming
    };
    let mut reaches = vec![false; function.values.len()];
    loop {
        let mut changed = false;
        for value in &function.values {
            let index = usize::from(value.id);
            if reaches[index] {
                continue;
            }
            let reached = match &value.kind {
                CfgValueKind::Ddt { .. } => true,
                CfgValueKind::BlockParameter => incoming
                    .get(&value.id)
                    .is_some_and(|args| args.iter().any(|arg| reaches[usize::from(*arg)])),
                kind => kind
                    .operands()
                    .into_iter()
                    .any(|operand| reaches[usize::from(operand)]),
            };
            if reached {
                reaches[index] = true;
                changed = true;
            }
        }
        if !changed {
            return reaches;
        }
    }
}

fn outgoing_edges(block: &crate::canonical_ir::cfg::CfgBlock) -> Vec<(BlockId, Vec<ValueId>)> {
    match &block.terminator {
        CfgTerminator::Jump { target, args } => vec![(*target, args.clone())],
        CfgTerminator::Branch {
            then_target,
            then_args,
            else_target,
            else_args,
            ..
        } => vec![
            (*then_target, then_args.clone()),
            (*else_target, else_args.clone()),
        ],
        CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
    }
}

/// `merges` counts only nested merges — arithmetic recurses freely, because in
/// SSA it is a DAG and the only way back to a value already on the stack is
/// through a loop-carried block parameter.
fn resolve_charge(
    function: &CfgFunction,
    reaches: &[bool],
    residual: ValueId,
    merges: usize,
) -> Option<Charge> {
    if !reaches.get(usize::from(residual)).copied().unwrap_or(false) {
        return Some(Charge::Nothing);
    }
    if merges > MAX_CHARGE_MERGE_DEPTH {
        return None;
    }

    match &function.value(residual).kind {
        CfgValueKind::Ddt { input, .. } => Some(Charge::Value(*input)),
        CfgValueKind::RealConstant(constant) if *constant == 0.0 => Some(Charge::Nothing),
        // Linear arithmetic is pushed inside the `ddt`, which is what makes a
        // scaled or summed charge recoverable. `k * ddt(q)` stores `k * q`;
        // `ddt(q1) + ddt(q2)` stores `q1 + q2`. Only the operations that
        // commute with `d/dt` are followed — a product of two charges is not
        // linear in either, so it is refused rather than approximated.
        CfgValueKind::Binary { op, left, right } => {
            let (op, left, right) = (*op, *left, *right);
            let charged_left = resolve_charge(function, reaches, left, merges);
            let charged_right = resolve_charge(function, reaches, right, merges);
            let stores =
                |charge: &Option<Charge>| matches!(charge, Some(charge) if !charge.is_nothing());
            match (op, stores(&charged_left), stores(&charged_right)) {
                // A sum of a conduction term and a charge stores only the
                // charge — the conduction half is already in the residual and
                // the reactive matrix does not want it. That is what the `0 + x`
                // accumulator is, generalised.
                (CfgBinaryOp::Add | CfgBinaryOp::Sub, true, false) => charged_left,
                (CfgBinaryOp::Add, false, true) => charged_right,
                (CfgBinaryOp::Sub, false, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Unary {
                        op: CfgUnaryOp::Neg,
                        input: charged_right?,
                    }),
                }),
                (CfgBinaryOp::Add | CfgBinaryOp::Sub, true, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: charged_right?,
                    }),
                }),
                (CfgBinaryOp::Mul, true, false) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: Charge::Value(right),
                    }),
                }),
                (CfgBinaryOp::Mul, false, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: Charge::Value(left),
                        right: charged_right?,
                    }),
                }),
                // Dividing a charge by something that carries none is still
                // linear in the charge; the other way round is not.
                (CfgBinaryOp::Div, true, false) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: Charge::Value(right),
                    }),
                }),
                (_, false, false) => Some(Charge::Nothing),
                _ => None,
            }
        }
        CfgValueKind::Unary {
            op: op @ CfgUnaryOp::Neg,
            input,
        } => {
            let (op, input) = (*op, *input);
            match resolve_charge(function, reaches, input, merges)? {
                Charge::Nothing => Some(Charge::Nothing),
                charge => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Unary { op, input: charge }),
                }),
            }
        }
        // A guarded contribution. `I(a, b) <+ ddt(q)` inside an `if` reaches its
        // equation as a merge of the `ddt` from the arm that ran and zero from
        // the arm that did not, so matching the residual against `Ddt` alone
        // finds nothing and the whole reactive contribution disappears — silent
        // in DC, wrong in AC and transient. Self-heating blocks are guarded as a
        // matter of course, so this is the common case rather than an edge one.
        CfgValueKind::BlockParameter => {
            let block = owning_block(function, residual)?;
            let position = function
                .block(block)
                .params
                .iter()
                .position(|param| *param == residual)?;
            let mut arms = Vec::new();
            let mut stores = false;
            for (source, slot) in edges_into(function, block) {
                let argument = *edge_arguments(function, source, slot).get(position)?;
                let arm = resolve_charge(function, reaches, argument, merges + 1)?;
                stores |= !matches!(arm, Charge::Nothing);
                arms.push(arm);
            }
            // Every arm storing nothing is a merge worth building only if some
            // arm stores something; otherwise the contribution is conduction and
            // adding a parameter for it would be noise in the graph.
            stores.then_some(Charge::Merge { block, arms })
        }
        _ => None,
    }
}

fn materialise_charge(
    function: &mut CfgFunction,
    charge: &Charge,
    insertions: &mut Vec<(ValueId, ValueId)>,
) -> Option<ValueId> {
    match charge {
        Charge::Value(value) => Some(*value),
        Charge::Nothing => None,
        Charge::Op { anchor, kind } => {
            let result = match &**kind {
                ChargeOp::Binary { op, left, right } => {
                    let left = materialise_operand(function, left, insertions);
                    let right = materialise_operand(function, right, insertions);
                    push_value(
                        function,
                        CfgValueType::Real,
                        CfgValueKind::Binary {
                            op: *op,
                            left,
                            right,
                        },
                    )
                }
                ChargeOp::Unary { op, input } => {
                    let input = materialise_operand(function, input, insertions);
                    push_value(
                        function,
                        CfgValueType::Real,
                        CfgValueKind::Unary { op: *op, input },
                    )
                }
            };
            insertions.push((*anchor, result));
            Some(result)
        }
        Charge::Merge { block, arms } => {
            // Depth first: an arm that is itself a merge has to own a parameter
            // before this one can name it on an edge.
            let mut arguments = Vec::with_capacity(arms.len());
            for arm in arms {
                let value = match materialise_charge(function, arm, insertions) {
                    Some(value) => value,
                    None => zero_constant(function),
                };
                arguments.push(value);
            }
            let parameter = push_value(function, CfgValueType::Real, CfgValueKind::BlockParameter);
            function.blocks[usize::from(*block)].params.push(parameter);
            for ((source, slot), argument) in edges_into(function, *block).into_iter().zip(arguments)
            {
                edge_arguments_mut(function, source, slot).push(argument);
            }
            Some(parameter)
        }
    }
}

/// An operand of a built operation, with "stores nothing" spelled as a zero.
fn materialise_operand(
    function: &mut CfgFunction,
    charge: &Charge,
    insertions: &mut Vec<(ValueId, ValueId)>,
) -> ValueId {
    match materialise_charge(function, charge, insertions) {
        Some(value) => value,
        None => zero_constant(function),
    }
}

/// Which arm of a terminator an edge leaves by.
#[derive(Clone, Copy, PartialEq, Eq)]
enum EdgeSlot {
    Jump,
    Then,
    Else,
}

/// Every edge into `block`, in a deterministic order: by source block id, then
/// then-arm before else-arm. Parameters and arguments are index-aligned, so this
/// order is the contract between resolution and construction.
fn edges_into(function: &CfgFunction, block: BlockId) -> Vec<(BlockId, EdgeSlot)> {
    let mut edges = Vec::new();
    for source in &function.blocks {
        match &source.terminator {
            CfgTerminator::Jump { target, .. } if *target == block => {
                edges.push((source.id, EdgeSlot::Jump));
            }
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => {
                if *then_target == block {
                    edges.push((source.id, EdgeSlot::Then));
                }
                if *else_target == block {
                    edges.push((source.id, EdgeSlot::Else));
                }
            }
            _ => {}
        }
    }
    edges
}

fn edge_arguments(function: &CfgFunction, source: BlockId, slot: EdgeSlot) -> &[ValueId] {
    match (&function.block(source).terminator, slot) {
        (CfgTerminator::Jump { args, .. }, EdgeSlot::Jump) => args,
        (CfgTerminator::Branch { then_args, .. }, EdgeSlot::Then) => then_args,
        (CfgTerminator::Branch { else_args, .. }, EdgeSlot::Else) => else_args,
        _ => &[],
    }
}

fn edge_arguments_mut(
    function: &mut CfgFunction,
    source: BlockId,
    slot: EdgeSlot,
) -> &mut Vec<ValueId> {
    match (
        &mut function.blocks[usize::from(source)].terminator,
        slot,
    ) {
        (CfgTerminator::Jump { args, .. }, EdgeSlot::Jump) => args,
        (CfgTerminator::Branch { then_args, .. }, EdgeSlot::Then) => then_args,
        (CfgTerminator::Branch { else_args, .. }, EdgeSlot::Else) => else_args,
        _ => unreachable!("edges_into only reports slots the terminator has"),
    }
}

fn owning_block(function: &CfgFunction, parameter: ValueId) -> Option<BlockId> {
    function
        .blocks
        .iter()
        .find(|block| block.params.contains(&parameter))
        .map(|block| block.id)
}

fn zero_constant(function: &mut CfgFunction) -> ValueId {
    if let Some(existing) = function
        .values
        .iter()
        .find(|value| matches!(value.kind, CfgValueKind::RealConstant(constant) if constant == 0.0))
    {
        return existing.id;
    }
    push_value(function, CfgValueType::Real, CfgValueKind::RealConstant(0.0))
}

fn push_value(
    function: &mut CfgFunction,
    value_type: CfgValueType,
    kind: CfgValueKind,
) -> ValueId {
    let id = ValueId::from(function.values.len());
    function.values.push(CfgValue {
        id,
        value_type,
        kind,
    });
    id
}

fn reject_unsupported_kinds(
    artifact: &CanonicalIrArtifact,
    function: &CfgFunction,
) -> Result<(), RustBackendError> {
    for value in &function.values {
        match &value.kind {
            CfgValueKind::BranchFlow(branch) => {
                return Err(unsupported(
                    artifact,
                    format!("an unresolved flow probe on {branch}"),
                ));
            }
            _ => {}
        }
    }
    if artifact
        .mir
        .equations
        .iter()
        .any(|equation| equation.kind == MirEquationKind::Indirect)
    {
        return Err(unsupported(artifact, "an indirect contribution"));
    }
    Ok(())
}

fn stage_fn_name(class: InvalidationClass) -> &'static str {
    match class {
        InvalidationClass::Instance => "canonical_instance_stage",
        InvalidationClass::Temperature => "canonical_temperature_stage",
        InvalidationClass::Timestep => "canonical_timestep_stage",
        InvalidationClass::Newton => "canonical_newton_stage",
    }
}

/// A branch endpoint, as the stamper wants it.
///
/// The *local* ordinal, not `self.nodes[..]`. The stamper resolves a node to a
/// matrix axis through its own per-instance cache, which is keyed by the
/// model's node numbering; handing it the global index would look plausible and
/// address a different node. `ctx.node_voltage`, by contrast, does take the
/// global one — the two are a real distinction and not interchangeable.
fn optional_node(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some({})", usize::from(node)))
        .unwrap_or_else(|| "None".to_string())
}

/// Reject a magnitude that is not a number before the analysis integrates it.
fn emit_noise_check(out: &mut String, index: usize, quantity: &str, value: &str) {
    let _ = writeln!(
        out,
        "            if !({value}).is_finite() {{ return Err(GeneratedNoiseEvaluationError::NonFinite {{ index: {index}, quantity: {quantity:?}, value: {value} }}); }}"
    );
}

fn bindings() -> EmitBindings {
    EmitBindings {
        analysis: "ctx.analysis".into(),
        simparam: "ctx.simparam_or".into(),
        ..EmitBindings::default()
    }
}

fn indent(body: &str, levels: usize) -> String {
    let pad = "    ".repeat(levels);
    let mut out = String::with_capacity(body.len() + body.len() / 8);
    for line in body.lines() {
        if !line.is_empty() {
            out.push_str(&pad);
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}

/// The one runtime helper an emitted body cannot express: `ddt` reads and writes
/// per-instance history, so it is a call rather than an expression.
/// `idt`'s counterpart, and a call for the same reason: it carries a running
/// total across evaluations.
///
/// A step that is not integrating returns the initial condition and *records*
/// it, so the next step that does integrate starts from there rather than from
/// whatever the last operating-point solve happened to leave behind.
const EVAL_IDT: &str = r#"
#[inline]
fn rspice_eval_idt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    active: bool,
    step: f64,
    slot: usize,
    value: f64,
    ic: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated idt state slot out of range");
    let started_from = if initialized[slot] { previous[slot] } else { ic };
    let total = if active { started_from + value * step } else { ic };
    current[slot] = total;
    if !active {
        previous[slot] = total;
        initialized[slot] = true;
    }
    total
}
"#;

const EVAL_DDT: &str = r#"
#[inline]
fn rspice_eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    active: bool,
    scale: f64,
    previous_value_scale: f64,
    older_value_scale: f64,
    previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if active {
        let result = value * scale
            - previous_value * previous_value_scale
            - older_value * older_value_scale
            - derivative_previous[slot] * previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

"#;
