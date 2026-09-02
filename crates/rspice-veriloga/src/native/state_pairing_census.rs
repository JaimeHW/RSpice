//! Is the canonical-to-bytecode state-slot pairing total over the shipped
//! corpus, and does it induce a renumbering?
//!
//! [`crate::canonical_ir::state`] records that three numberings of a module's
//! analog-operator records exist and that the JIT runtime will take the
//! per-*site* one. Moving it there means rewriting every slot index the
//! bytecode generator handed out per *emission* into the site's own number. A
//! rewrite is only well defined if, for every program the generator compiled,
//! each state instruction can be told which canonical site it came from.
//!
//! [`pair_canonical_state_slots`] is the existing answer to that question, but
//! it has only ever been asked about the programs the native plan lowers, one
//! model at a time, and never asked whether the answer covers the module. This
//! census asks it about **every** program of **every** shipped module, in every
//! context `generate_from_ir` compiles, and reports four things per module:
//!
//! * **programs paired** — how many carry state and pair without error;
//! * **count mismatches** — the pairing's own refusal, with the context, the
//!   canonical count and the bytecode count, which is what a renumbering would
//!   have to refuse the module for;
//! * **unreached slots** — a `(family, slot)` some program addresses that no
//!   pairing ever named. A renumbering that left one behind would leave an
//!   instruction reading a record nothing writes;
//! * **conflicts** — one emitted slot claimed by two different canonical sites,
//!   which would make the rewrite a non-function.
//!
//! The per-emission-to-per-site map is *built* here rather than described:
//! every successful pairing contributes `(family, emitted slot) -> site`, and
//! the conflict and unreached tallies are that map's totality and functionality
//! measured over the corpus. That map is the renumbering.
//!
//! ## Which canonical expression each context is paired against
//!
//! * **parameters** — the parameter's own default, bound or exclude
//!   expression, which is the root `lower_parameter_default_program` lowers.
//! * **assignments** and the **noise-assignment clone** — the module's whole
//!   statement pass at once, against
//!   [`CanonicalStateLayout::statement_prefix`]. Pass level rather than step
//!   level on purpose: the step-to-statement correspondence is
//!   `AssignmentProgramCursor`'s reconstruction from target variable indices,
//!   and a census that guessed it would report the guess's failures as the
//!   pairing's. The pass-level question is also the one the renumbering asks —
//!   whether the *k*-th emission of a family in the assignment pass is the
//!   *k*-th statement site of that family.
//! * **equation value programs** and their **resistive derivative programs** —
//!   the equation's expression, which is what `lower_stamp_value_program` and
//!   `lower_jacobian_program` pair against today.
//! * **reactive derivative programs** and **noise-source programs** — no
//!   canonical root. The plan builder pairs neither (a reactive Jacobian gets
//!   only its table-lookup slots, from a MIR rebuilt around the extracted
//!   charge), so this census does not invent one: it measures whether those
//!   programs carry state slots at all, because a nonzero count there is a gap
//!   the renumbering would have to close before it could be total.
//!
//! The modules come from [`census_models`](super::census_models), the shared
//! front-end provider every whole-corpus census reads, so this one costs a
//! cache read rather than a forty-third compile of the same tree.
//! `RSPICE_CFG_CENSUS_FILTER` narrows it to one module; unlike the state-slot
//! numbering census, nothing here asserts a corpus-wide shape, so a filtered
//! run is meaningful on its own.
//!
//! `#[ignore]`d: this is release-qualification work over the whole shipped
//! corpus. Run it with
//! `--release --features native --lib state_pairing -- --ignored --nocapture --test-threads=1`.

use std::collections::{HashMap, HashSet};

use smol_str::SmolStr;

use super::census_models::shipped_census_models_matching;
use super::cfg_census::{PrefixShape, integration_emission_contexts, prefix_shape};
use super::expr::{CanonicalStateSiteScan, pair_canonical_state_slots};
use crate::canonical_ir::{
    CanonicalStateFamily, CanonicalStateLayout, CanonicalStateOperator, ExprId, HirModel, MirModel,
};
use crate::codegen::{AssignmentStep, BytecodeProgram, CompiledModel, Instruction};

/// Which of the generator's passes a program belongs to.
///
/// The same partition [`integration_emission_contexts`] tags slots with, named
/// here as well because a mismatch is only actionable if it says which pass
/// compiled the program that could not be paired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pass {
    Parameter,
    Assignment,
    NoiseAssignment,
    EquationPrimal,
    EquationDerivative,
    ReactiveDerivative,
    NoiseSource,
}

impl Pass {
    fn name(self) -> &'static str {
        match self {
            Self::Parameter => "parameter",
            Self::Assignment => "assignment",
            Self::NoiseAssignment => "noise-assignment",
            Self::EquationPrimal => "equation-primal",
            Self::EquationDerivative => "equation-derivative",
            Self::ReactiveDerivative => "reactive-derivative",
            Self::NoiseSource => "noise-source",
        }
    }
}

/// What one module's programs said when each was paired with its canonical
/// root.
#[derive(Default)]
struct ModelCensus {
    /// Bytecode programs walked, in every pass.
    programs: usize,
    /// Of those, the ones carrying at least one state instruction.
    state_programs: usize,
    /// Of those, the ones every operator family paired for.
    paired_units: usize,
    /// State-carrying programs in a pass with no canonical root to pair
    /// against.
    unrooted_state_programs: usize,
    /// The renumbering this module would get: an emitted slot and the canonical
    /// site that claims it.
    map: HashMap<(CanonicalStateFamily, usize), (ExprId, u32)>,
    /// Every `(family, slot)` any program of the module addresses.
    allocated: HashSet<(CanonicalStateFamily, usize)>,
    mismatches: Vec<String>,
    conflicts: Vec<String>,
    /// A paired site the executed layout does not number, which would leave the
    /// rewrite with no site slot to write.
    unnumbered: Vec<String>,
}

impl ModelCensus {
    /// Record every slot the program addresses, whether or not it is paired.
    ///
    /// The denominator of the "unreached" figure: a renumbering has to account
    /// for every slot some instruction reads, not only for the ones a pairing
    /// happened to name.
    fn note_allocated(&mut self, program: &BytecodeProgram) {
        for instruction in &program.instructions {
            for operator in CanonicalStateOperator::ALL {
                if let Some(slot) = operator.bytecode_slot(instruction) {
                    self.allocated.insert((operator.family(), slot));
                }
            }
        }
    }

    /// Attach one emitted slot to the canonical site that owns it.
    fn claim(
        &mut self,
        layout: &CanonicalStateLayout,
        family: CanonicalStateFamily,
        emitted: usize,
        site: ExprId,
        where_: &str,
    ) {
        let Some(numbered) = layout.site(site) else {
            self.unnumbered.push(format!(
                "{where_}: canonical site {site} owns emitted {family:?} slot {emitted} but the \
                 executed layout numbers no record for it"
            ));
            return;
        };
        match self.map.entry((family, emitted)) {
            std::collections::hash_map::Entry::Occupied(held) => {
                let (first, _) = *held.get();
                if first != site {
                    self.conflicts.push(format!(
                        "{where_}: emitted {family:?} slot {emitted} is claimed by canonical sites \
                         {first} and {site}"
                    ));
                }
            }
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert((site, numbered.slot));
            }
        }
    }
}

/// Whether the program contains any instruction addressing a state record.
fn carries_state(program: &BytecodeProgram) -> bool {
    program.instructions.iter().any(|instruction| {
        CanonicalStateOperator::ALL
            .iter()
            .any(|operator| operator.bytecode_slot(instruction).is_some())
    })
}

/// Pair one program against the canonical expression it was compiled from.
///
/// Mirrors `CanonicalExpressionStateSlots::for_expression`: one traversal of
/// the canonical tree, thirteen pairings against it, and the same early-out for
/// a program with no state at all.
#[allow(clippy::too_many_arguments)]
fn pair_rooted_program(
    census: &mut ModelCensus,
    model: &SmolStr,
    mir: &MirModel,
    layout: &CanonicalStateLayout,
    scans: &mut HashMap<ExprId, CanonicalStateSiteScan>,
    pass: Pass,
    label: &str,
    root: ExprId,
    program: &BytecodeProgram,
) {
    census.programs += 1;
    census.note_allocated(program);
    if !carries_state(program) {
        return;
    }
    census.state_programs += 1;

    // Memoized per root: an equation's scan is walked once and reused by its
    // value program and each of its derivative programs, which on a compact
    // model is the difference between one traversal of a large tree and one per
    // Jacobian entry.
    let scan = match scans.entry(root) {
        std::collections::hash_map::Entry::Occupied(held) => held.into_mut(),
        std::collections::hash_map::Entry::Vacant(slot) => {
            match CanonicalStateSiteScan::for_expression(model, mir, root) {
                Ok(scan) => slot.insert(scan),
                Err(error) => {
                    census.mismatches.push(format!(
                        "{} {label}: canonical scan of expression {root} failed: {error}",
                        pass.name()
                    ));
                    return;
                }
            }
        }
    };

    let mut paired = true;
    for operator in CanonicalStateOperator::ALL {
        match pair_canonical_state_slots(model.clone(), root, scan, program, operator) {
            Ok(pairs) => {
                for (site, emitted) in pairs {
                    census.claim(
                        layout,
                        operator.family(),
                        emitted,
                        site,
                        &format!("{} {label}", pass.name()),
                    );
                }
            }
            Err(error) => {
                paired = false;
                census
                    .mismatches
                    .push(format!("{} {label}: {error}", pass.name()));
            }
        }
    }
    if paired {
        census.paired_units += 1;
    }
}

/// Note a program in a pass with no canonical root.
fn note_unrooted_program(
    census: &mut ModelCensus,
    pass: Pass,
    label: &str,
    program: &BytecodeProgram,
) {
    census.programs += 1;
    census.note_allocated(program);
    if carries_state(program) {
        census.state_programs += 1;
        census.unrooted_state_programs += 1;
        let families = CanonicalStateOperator::ALL
            .iter()
            .filter(|operator| {
                program
                    .instructions
                    .iter()
                    .any(|instruction| operator.bytecode_slot(instruction).is_some())
            })
            .map(|operator| operator.name())
            .collect::<Vec<_>>();
        census.mismatches.push(format!(
            "{} {label}: carries {families:?} state with no canonical root to pair against",
            pass.name()
        ));
    }
}

/// Every state instruction of an assignment pass, in the order the generator
/// emitted them.
///
/// The traversal `integration_emission_contexts` uses, including the ordering
/// detail that decides the answer: `compile_assignment_items` binds an indexed
/// assignment's *value* program before it compiles the index expression.
fn assignment_pass_state(steps: &[AssignmentStep], out: &mut Vec<Instruction>) {
    for step in steps {
        match step {
            AssignmentStep::Assign(assignment) => push_state(&assignment.program, out),
            AssignmentStep::AssignIndexed { index, value, .. } => {
                push_state(value, out);
                push_state(index, out);
            }
            AssignmentStep::Loop { condition, body } => {
                push_state(condition, out);
                assignment_pass_state(body, out);
            }
        }
    }
}

fn push_state(program: &BytecodeProgram, out: &mut Vec<Instruction>) {
    out.extend(
        program
            .instructions
            .iter()
            .filter(|instruction| {
                CanonicalStateOperator::ALL
                    .iter()
                    .any(|operator| operator.bytecode_slot(instruction).is_some())
            })
            .cloned(),
    );
}

/// Count the programs of an assignment pass and record every slot they address.
fn note_assignment_pass_programs(census: &mut ModelCensus, steps: &[AssignmentStep]) {
    for step in steps {
        match step {
            AssignmentStep::Assign(assignment) => {
                census.programs += 1;
                census.note_allocated(&assignment.program);
                census.state_programs += usize::from(carries_state(&assignment.program));
            }
            AssignmentStep::AssignIndexed { index, value, .. } => {
                for program in [value, index] {
                    census.programs += 1;
                    census.note_allocated(program);
                    census.state_programs += usize::from(carries_state(program));
                }
            }
            AssignmentStep::Loop { condition, body } => {
                census.programs += 1;
                census.note_allocated(condition);
                census.state_programs += usize::from(carries_state(condition));
                note_assignment_pass_programs(census, body);
            }
        }
    }
}

/// Pair a whole assignment pass against the module's statement sites.
///
/// The pass-level pairing is the pairing function's own rule applied to the
/// concatenation: within one family the *k*-th emission is the *k*-th site, and
/// a length disagreement is a refusal rather than a tolerance. Spelled against
/// [`CanonicalStateLayout::statement_prefix`] rather than against a scan
/// because the layout is the numbering the rewrite writes *into*, so a pass
/// that pairs here pairs against the thing that will consume it.
fn pair_assignment_pass(
    census: &mut ModelCensus,
    layout: &CanonicalStateLayout,
    statements: &CanonicalStateLayout,
    pass: Pass,
    steps: &[AssignmentStep],
) {
    note_assignment_pass_programs(census, steps);
    let mut emitted = Vec::new();
    assignment_pass_state(steps, &mut emitted);
    if emitted.is_empty() {
        return;
    }
    let mut paired = true;
    for operator in CanonicalStateOperator::ALL {
        let sites = statements
            .sites()
            .iter()
            .filter(|site| site.kind == operator)
            .map(|site| site.operator)
            .collect::<Vec<_>>();
        let slots = emitted
            .iter()
            .filter_map(|instruction| operator.bytecode_slot(instruction))
            .collect::<Vec<_>>();
        if sites.len() != slots.len() {
            paired = false;
            census.mismatches.push(format!(
                "{} pass: statements own {} {} sites {sites:?} but the pass emits {} slots {slots:?}",
                pass.name(),
                sites.len(),
                operator.name(),
                slots.len(),
            ));
            continue;
        }
        for (site, slot) in sites.into_iter().zip(slots) {
            census.claim(
                layout,
                operator.family(),
                slot,
                site,
                &format!("{} pass", pass.name()),
            );
        }
    }
    if paired {
        census.paired_units += 1;
    }
}

/// Walk every program of one compiled module, pairing what has a root.
fn census_model(model: &CompiledModel, hir: &HirModel, mir: &MirModel) -> ModelCensus {
    let layout = CanonicalStateLayout::from_hir(hir);
    let statements = CanonicalStateLayout::statement_prefix(hir);
    let mut census = ModelCensus::default();
    let mut scans: HashMap<ExprId, CanonicalStateSiteScan> = HashMap::new();

    for (index, parameter) in model.parameters.iter().enumerate() {
        let canonical = mir.parameters.get(index);
        let range = canonical.and_then(|parameter| parameter.range.as_ref());
        // Named slots rather than two collected lists. A parameter with a
        // constant default and an expression-valued bound compiles a `min`
        // program and no `default` program, so a positional zip over the
        // *present* programs would hand the bound program the default's
        // expression — a pairing that could refuse a module for a mistake made
        // here rather than by the generator.
        let mut slots: Vec<(String, &BytecodeProgram, Option<ExprId>)> = Vec::new();
        if let Some(program) = parameter.default_program.as_ref() {
            slots.push((
                format!("parameter[{index}].default"),
                program,
                canonical.and_then(|parameter| parameter.default_expr.as_ref().map(|expr| expr.id)),
            ));
        }
        if let Some(program) = parameter.min_program.as_ref() {
            slots.push((
                format!("parameter[{index}].min"),
                program,
                range.and_then(|range| range.min_expression.as_ref().map(|expr| expr.id)),
            ));
        }
        if let Some(program) = parameter.max_program.as_ref() {
            slots.push((
                format!("parameter[{index}].max"),
                program,
                range.and_then(|range| range.max_expression.as_ref().map(|expr| expr.id)),
            ));
        }
        for (position, program) in parameter.exclude_programs.iter().enumerate() {
            slots.push((
                format!("parameter[{index}].exclude[{position}]"),
                program,
                range
                    .and_then(|range| range.exclude_expressions.get(position))
                    .map(|expr| expr.id),
            ));
        }
        for (label, program, root) in slots {
            match root {
                Some(root) => pair_rooted_program(
                    &mut census,
                    &model.name,
                    mir,
                    &layout,
                    &mut scans,
                    Pass::Parameter,
                    &label,
                    root,
                    program,
                ),
                None => note_unrooted_program(&mut census, Pass::Parameter, &label, program),
            }
        }
    }

    pair_assignment_pass(
        &mut census,
        &layout,
        &statements,
        Pass::Assignment,
        &model.assignment_steps,
    );
    pair_assignment_pass(
        &mut census,
        &layout,
        &statements,
        Pass::NoiseAssignment,
        &model.noise_assignment_steps,
    );

    for (index, stamp) in model.stamp_programs.iter().enumerate() {
        let root = mir
            .equations
            .get(index)
            .map(|equation| equation.expression.id);
        let Some(root) = root else {
            note_unrooted_program(
                &mut census,
                Pass::EquationPrimal,
                &format!("stamp[{index}].value"),
                &stamp.value_program,
            );
            continue;
        };
        pair_rooted_program(
            &mut census,
            &model.name,
            mir,
            &layout,
            &mut scans,
            Pass::EquationPrimal,
            &format!("stamp[{index}].value"),
            root,
            &stamp.value_program,
        );
        if let Some(condition) = &stamp.static_condition {
            pair_rooted_program(
                &mut census,
                &model.name,
                mir,
                &layout,
                &mut scans,
                Pass::EquationPrimal,
                &format!("stamp[{index}].static_condition"),
                root,
                condition,
            );
        }
        // One `compile_expr` per *pair* of entries on a current contribution,
        // one per entry on a branch row: the positive and negative KCL rows
        // share one compiled derivative by `clone()`, so walking both would
        // pair the same program twice.
        let stride = if stamp.branch_ordinal.is_none() { 2 } else { 1 };
        for (position, entry) in stamp.jacobian_programs.iter().step_by(stride).enumerate() {
            pair_rooted_program(
                &mut census,
                &model.name,
                mir,
                &layout,
                &mut scans,
                Pass::EquationDerivative,
                &format!("stamp[{index}].jacobian[{position}]"),
                root,
                &entry.program,
            );
        }
        for (position, entry) in stamp.reactive_jacobians.iter().step_by(stride).enumerate() {
            note_unrooted_program(
                &mut census,
                Pass::ReactiveDerivative,
                &format!("stamp[{index}].reactive_jacobian[{position}]"),
                &entry.program,
            );
        }
    }

    for (index, source) in model.noise_sources.iter().enumerate() {
        note_unrooted_program(
            &mut census,
            Pass::NoiseSource,
            &format!("noise[{index}].psd"),
            &source.psd_program,
        );
        if let Some(program) = &source.exponent_program {
            note_unrooted_program(
                &mut census,
                Pass::NoiseSource,
                &format!("noise[{index}].exponent"),
                program,
            );
        }
        for (position, injection) in source.injections.iter().enumerate() {
            note_unrooted_program(
                &mut census,
                Pass::NoiseSource,
                &format!("noise[{index}].injection[{position}].gain"),
                &injection.gain_program,
            );
        }
    }

    census
}

/// Whether every shipped module's emitted state slots can be told which
/// canonical site owns them, which is the precondition for numbering the JIT
/// runtime's records per site.
///
/// See the module documentation for what each column means and for which
/// canonical expression each pass is paired against. The assertions are the two
/// properties a renumbering cannot be built without — no conflict and no
/// unreached slot — and they are assertions rather than reported figures
/// because a module failing either is a named refusal for the move, not a
/// measurement of it.
#[test]
#[ignore = "release qualification; run with --release --features native --lib state_pairing -- --ignored --nocapture --test-threads=1"]
fn the_canonical_bytecode_state_pairing_is_censused_over_the_shipped_corpus() {
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();

    let mut models = 0_usize;
    let mut total_programs = 0_usize;
    let mut total_state_programs = 0_usize;
    let mut total_paired = 0_usize;
    let mut models_with_mismatch = 0_usize;
    let mut models_with_unreached = 0_usize;
    let mut models_with_conflict = 0_usize;
    let mut models_renumbered = 0_usize;
    let mut appending = 0_usize;
    let mut interleaving = 0_usize;
    let mut refusals: Vec<String> = Vec::new();

    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = &shipped.name;
        let runtime = &shipped;
        let artifact = &runtime.canonical_ir;
        models += 1;

        let census = census_model(&runtime.model, &artifact.hir, &artifact.mir);

        let statement_sites = CanonicalStateLayout::statement_prefix(&artifact.hir)
            .family_len(CanonicalStateFamily::Integration);
        let per_site = CanonicalStateLayout::from_hir(&artifact.hir)
            .family_len(CanonicalStateFamily::Integration);
        let tags = integration_emission_contexts(&runtime.model);
        let shape = prefix_shape(
            &tags,
            statement_sites,
            per_site.saturating_sub(statement_sites),
        );

        let mut unreached: Vec<(CanonicalStateFamily, usize)> = census
            .allocated
            .iter()
            .filter(|key| !census.map.contains_key(*key))
            .copied()
            .collect();
        unreached.sort_by_key(|(family, slot)| (format!("{family:?}"), *slot));

        // Whether adopting the per-site numbering moves anything for this
        // module: a slot whose site number is not its emitted number.
        let moved = census
            .map
            .iter()
            .filter(|((_, emitted), (_, site))| *emitted != *site as usize)
            .count();

        total_programs += census.programs;
        total_state_programs += census.state_programs;
        total_paired += census.paired_units;
        if !census.mismatches.is_empty() {
            models_with_mismatch += 1;
        }
        if !unreached.is_empty() {
            models_with_unreached += 1;
        }
        if !census.conflicts.is_empty() {
            models_with_conflict += 1;
        }
        if moved > 0 {
            models_renumbered += 1;
        }
        if per_site != tags.len() {
            match shape {
                PrefixShape::Append | PrefixShape::Identical => appending += 1,
                PrefixShape::Interleave => interleaving += 1,
            }
        }

        println!(
            "pairing model={module} programs={} state_programs={} paired_units={} \
                 unrooted_state={} mismatches={} conflicts={} unnumbered={} \
                 allocated={} mapped={} unreached={} moved={} \
                 per_site={per_site} per_emission={} shape={shape:?}",
            census.programs,
            census.state_programs,
            census.paired_units,
            census.unrooted_state_programs,
            census.mismatches.len(),
            census.conflicts.len(),
            census.unnumbered.len(),
            census.allocated.len(),
            census.map.len(),
            unreached.len(),
            moved,
            tags.len(),
        );
        for mismatch in &census.mismatches {
            println!("  pairing-mismatch model={module} {mismatch}");
        }
        for conflict in &census.conflicts {
            println!("  pairing-conflict model={module} {conflict}");
        }
        for unnumbered in &census.unnumbered {
            println!("  pairing-unnumbered model={module} {unnumbered}");
        }
        for (family, slot) in &unreached {
            println!("  pairing-unreached model={module} family={family:?} slot={slot}");
        }
        if !census.mismatches.is_empty() || !census.conflicts.is_empty() {
            refusals.push(module.clone());
        }
    }

    println!(
        "pairing models={models} programs={total_programs} state_programs={total_state_programs} \
         paired_units={total_paired} models_with_mismatch={models_with_mismatch} \
         models_with_unreached={models_with_unreached} models_with_conflict={models_with_conflict} \
         models_renumbered={models_renumbered} append={appending} interleave={interleaving} \
         refusals={refusals:?}"
    );

    assert_eq!(
        models_with_conflict, 0,
        "an emitted state slot is claimed by two canonical sites, so the per-site renumbering is \
         not a function on this corpus"
    );
    assert_eq!(
        models_with_unreached, 0,
        "a state slot some program addresses is named by no canonical site, so a per-site \
         renumbering would leave an instruction reading a record nothing writes"
    );
    assert_eq!(
        models_with_mismatch, 0,
        "the canonical-to-bytecode state pairing is not total over the shipped corpus: {refusals:?}"
    );
    if filter.is_none() {
        assert_eq!(models, 43, "the shipped census is 43 modules");
    }
}
