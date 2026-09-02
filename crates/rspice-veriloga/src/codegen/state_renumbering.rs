//! Renumber a compiled model's state slots from per-*emission* to per-*site*.
//!
//! The bytecode generator hands out a fresh scalar-state slot at each
//! *emission* of an integration operator, and it compiles one source operator
//! more than once: a statement is compiled twice when the module has noise
//! (once as `assignment_steps`, again as the noise-shadowed
//! `noise_assignment_steps`), and a contribution's operator is compiled again
//! inside every Jacobian entry the product rule leaves it in. One canonical
//! `ddt` site therefore owned two or more runtime records, each integrating its
//! own copy of the history.
//!
//! That was survivable while exactly one route evaluated each program. It stops
//! being survivable at the CFG flip, where the canonical route supplies
//! residuals, Jacobians and noise while assignments stay MIR-lowered: a `ddt`
//! or `$limit` in an assignment is then evaluated twice per solve. Under
//! per-site numbering the second evaluation writes the record the first one
//! wrote, which is idempotent; under per-emission numbering it writes a
//! *different* record, and the two integrate different histories.
//!
//! [`crate::canonical_ir::state`] records the per-site numbering — keyed by the
//! operator expression's `ExprId`, ranked within its family along the module's
//! executed roots. This pass rewrites every slot index in a [`CompiledModel`]
//! into that numbering, so both the VM and the JIT address one record per site.
//!
//! # How a slot is told which site owns it
//!
//! [`StateSlotMapping::build`] walks every program of the model and pairs each
//! with the canonical expression it was compiled from, using
//! [`pair_canonical_state_slots`]: within one family the *k*-th emission is the
//! *k*-th site. Which expression each pass pairs against:
//!
//! * **parameters** — the parameter's own default, bound or exclude
//!   expression.
//! * **assignments** and the **noise-assignment clone** — the module's whole
//!   statement pass at once, against
//!   [`CanonicalStateLayout::statement_prefix`]. Pass level rather than step
//!   level because the step-to-statement correspondence is a reconstruction
//!   from target variable indices, and the question the renumbering actually
//!   asks is whether the *k*-th emission of a family in the pass is the *k*-th
//!   statement site of that family.
//! * **equation value programs** and their **resistive derivative programs** —
//!   the equation's expression.
//! * **reactive derivative programs**, **noise-source programs** and the
//!   **`zi` definition operand programs** — no canonical root. A reactive
//!   Jacobian is lowered from a MIR rebuilt around the extracted charge, so
//!   there is no expression to pair it against. Those programs are still
//!   *rewritten*: their slots have to have been claimed by some rooted program
//!   of the same module, and a slot no rooted program claims is a refusal
//!   ([`StateRenumberingError::SlotUnclaimed`]) rather than a slot left at its
//!   emitted number.
//!
//! # Compile-time arrays
//!
//! Three families are addressed by the same slot mechanism but read
//! *compile-time* data rather than accepted state: `laplace` reads
//! [`CompiledModel::laplace_filters`], `zi` reads
//! [`CompiledModel::zi_filters`] and its definitions, and `$table_model` reads
//! [`CompiledModel::lookup_tables`]. Renumbering an instruction of one of those
//! families without moving the datum it names would point it at the wrong
//! filter, so the pass permutes those arrays alongside the instructions. In
//! practice the generator allocates those families through a site map or by
//! content, so the two numberings already agree and the permutation is the
//! identity; the code is here because "already agrees" is a property of the
//! shipped corpus, not a guarantee about a user's module.
//!
//! # Refusal
//!
//! Every failure is named. The state-slot pairing census
//! (`crate::native::state_pairing_census`) measures this same mapping over all
//! forty-three shipped modules and finds it total, functional and complete —
//! but a user module is not the shipped corpus, and a module whose slots cannot
//! be told which site owns them must refuse to compile rather than be silently
//! misnumbered.

use std::collections::{HashMap, HashSet};

use crate::canonical_ir::{
    CanonicalStateFamily, CanonicalStateLayout, CanonicalStateOperator, ExprId, HirModel, MirModel,
};
use crate::codegen::state_slots::{
    CanonicalStateSiteScan, carries_state, pair_canonical_state_slots,
};
use crate::codegen::{
    AssignmentStep, BytecodeProgram, CompiledModel, CompiledZiPolynomial, Instruction,
};

/// Why a compiled model could not be given the canonical per-site state
/// numbering.
///
/// Each variant names the module, so a refusal points at the source that has to
/// change rather than at the compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateRenumberingError {
    /// A program and the canonical expression it was compiled from disagree
    /// about how many records of a family they hold, so no correlation exists.
    PairingIncomplete { module: String, detail: String },
    /// Two canonical sites claim one emitted slot, which would make the
    /// rewrite a non-function.
    SlotClaimedTwice { module: String, detail: String },
    /// A paired site is not numbered by the module's executed layout, so there
    /// is no site slot to rewrite into.
    SiteNotNumbered { module: String, detail: String },
    /// A slot some instruction addresses is claimed by no canonical site,
    /// which would leave that instruction reading a record nothing writes.
    SlotUnclaimed {
        module: String,
        family: CanonicalStateFamily,
        slot: usize,
    },
    /// A family backed by a compile-time array moves, but its new numbering is
    /// not a permutation of the array's indices, so the data cannot follow the
    /// instructions.
    BackedArrayNotPermutable {
        module: String,
        family: CanonicalStateFamily,
        detail: String,
    },
}

impl std::fmt::Display for StateRenumberingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PairingIncomplete { module, detail } => write!(
                f,
                "module `{module}` state slots cannot be correlated with their canonical sites: \
                 {detail}"
            ),
            Self::SlotClaimedTwice { module, detail } => write!(
                f,
                "module `{module}` has an emitted state slot claimed by two canonical sites: \
                 {detail}"
            ),
            Self::SiteNotNumbered { module, detail } => write!(
                f,
                "module `{module}` owns a state site the executed layout does not number: {detail}"
            ),
            Self::SlotUnclaimed {
                module,
                family,
                slot,
            } => write!(
                f,
                "module `{module}` addresses {family:?} state slot {slot}, which no canonical site \
                 claims"
            ),
            Self::BackedArrayNotPermutable {
                module,
                family,
                detail,
            } => write!(
                f,
                "module `{module}` renumbers its {family:?} records, but the compiled data they \
                 name cannot follow: {detail}"
            ),
        }
    }
}

impl std::error::Error for StateRenumberingError {}

/// Which of the generator's passes a program belongs to.
///
/// Named rather than described because a refusal is only actionable if it says
/// which pass compiled the program that could not be paired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pass {
    Parameter,
    Assignment,
    NoiseAssignment,
    EquationPrimal,
    EquationDerivative,
    ReactiveDerivative,
    NoiseSource,
    ZiDefinition,
}

impl Pass {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Parameter => "parameter",
            Self::Assignment => "assignment",
            Self::NoiseAssignment => "noise-assignment",
            Self::EquationPrimal => "equation-primal",
            Self::EquationDerivative => "equation-derivative",
            Self::ReactiveDerivative => "reactive-derivative",
            Self::NoiseSource => "noise-source",
            Self::ZiDefinition => "zi-definition",
        }
    }
}

/// The renumbering one module would get, and everything that could stop it.
///
/// Built once and read two ways: [`renumber_state_slots_to_canonical_sites`]
/// refuses on any defect and then applies the map, while the state-slot pairing
/// census reports the same fields as columns over the shipped corpus. Sharing
/// the construction is deliberate — the census verifies the code the compiler
/// runs, not a second implementation of it.
#[derive(Default)]
pub(crate) struct StateSlotMapping {
    /// Bytecode programs walked, in every pass.
    pub(crate) programs: usize,
    /// Of those, the ones carrying at least one state instruction.
    pub(crate) state_programs: usize,
    /// Of those, the ones every operator family paired for.
    pub(crate) paired_units: usize,
    /// State-carrying programs in a pass with no canonical root to pair
    /// against.
    pub(crate) unrooted_state_programs: usize,
    /// The renumbering: an emitted slot and the canonical site that claims it,
    /// with that site's number in the executed layout.
    pub(crate) map: HashMap<(CanonicalStateFamily, usize), (ExprId, u32)>,
    /// Every `(family, slot)` any program of the module addresses.
    pub(crate) allocated: HashSet<(CanonicalStateFamily, usize)>,
    /// Rooted programs whose slot counts do not match their canonical
    /// expression.
    pub(crate) mismatches: Vec<String>,
    /// Emitted slots claimed by two different canonical sites.
    pub(crate) conflicts: Vec<String>,
    /// Paired sites the executed layout does not number.
    pub(crate) unnumbered: Vec<String>,
    /// State-carrying programs in a pass with no canonical root, listed
    /// separately from [`Self::mismatches`]: those slots are still rewritable
    /// as long as a rooted program of the same module claimed them, which is
    /// what the unreached tally decides.
    pub(crate) unrooted: Vec<String>,
}

impl StateSlotMapping {
    /// Walk every program of one compiled module, pairing what has a root.
    pub(crate) fn build(model: &CompiledModel, hir: &HirModel, mir: &MirModel) -> Self {
        let layout = CanonicalStateLayout::from_hir(hir);
        let statements = CanonicalStateLayout::statement_prefix(hir);
        let mut mapping = Self::default();
        let mut scans: HashMap<ExprId, CanonicalStateSiteScan> = HashMap::new();

        mapping.walk_parameters(model, mir, &layout, &mut scans);
        mapping.pair_assignment_pass(
            &layout,
            &statements,
            Pass::Assignment,
            &model.assignment_steps,
        );
        mapping.pair_assignment_pass(
            &layout,
            &statements,
            Pass::NoiseAssignment,
            &model.noise_assignment_steps,
        );
        mapping.walk_stamps(model, mir, &layout, &mut scans);
        mapping.walk_unrooted(model);

        mapping
    }

    /// Every `(family, slot)` addressed by some program that no canonical site
    /// claims.
    ///
    /// The denominator of the rewrite: an instruction whose slot is not here
    /// has a site to be renumbered into, and one that is here does not.
    pub(crate) fn unreached(&self) -> Vec<(CanonicalStateFamily, usize)> {
        let mut unreached = self
            .allocated
            .iter()
            .filter(|key| !self.map.contains_key(*key))
            .copied()
            .collect::<Vec<_>>();
        unreached.sort_by_key(|(family, slot)| (format!("{family:?}"), *slot));
        unreached
    }

    /// Slots whose canonical site number is not the number they were emitted
    /// with — what adopting the per-site numbering actually moves.
    pub(crate) fn moved(&self) -> usize {
        self.map
            .iter()
            .filter(|((_, emitted), (_, site))| *emitted != *site as usize)
            .count()
    }

    /// The first defect that makes the module unrenumberable, if any.
    fn refusal(&self, module: &str) -> Option<StateRenumberingError> {
        if let Some(detail) = self.mismatches.first() {
            return Some(StateRenumberingError::PairingIncomplete {
                module: module.to_owned(),
                detail: detail.clone(),
            });
        }
        if let Some(detail) = self.conflicts.first() {
            return Some(StateRenumberingError::SlotClaimedTwice {
                module: module.to_owned(),
                detail: detail.clone(),
            });
        }
        if let Some(detail) = self.unnumbered.first() {
            return Some(StateRenumberingError::SiteNotNumbered {
                module: module.to_owned(),
                detail: detail.clone(),
            });
        }
        if let Some((family, slot)) = self.unreached().first().copied() {
            return Some(StateRenumberingError::SlotUnclaimed {
                module: module.to_owned(),
                family,
                slot,
            });
        }
        None
    }

    /// The site number an emitted slot is rewritten into.
    fn target(&self, family: CanonicalStateFamily, emitted: usize) -> Option<usize> {
        self.map
            .get(&(family, emitted))
            .map(|(_, slot)| *slot as usize)
    }

    /// Record every slot the program addresses, whether or not it is paired.
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

    /// Pair one program against the canonical expression it was compiled from.
    fn pair_rooted_program(
        &mut self,
        mir: &MirModel,
        layout: &CanonicalStateLayout,
        scans: &mut HashMap<ExprId, CanonicalStateSiteScan>,
        pass: Pass,
        label: &str,
        root: ExprId,
        program: &BytecodeProgram,
    ) {
        self.programs += 1;
        self.note_allocated(program);
        if !carries_state(program) {
            return;
        }
        self.state_programs += 1;

        // Memoized per root: an equation's scan is walked once and reused by
        // its value program and each of its derivative programs, which on a
        // compact model is the difference between one traversal of a large tree
        // and one per Jacobian entry.
        let scan = match scans.entry(root) {
            std::collections::hash_map::Entry::Occupied(held) => held.into_mut(),
            std::collections::hash_map::Entry::Vacant(slot) => {
                match CanonicalStateSiteScan::for_expression(mir, root) {
                    Ok(scan) => slot.insert(scan),
                    Err(missing) => {
                        self.mismatches.push(format!(
                            "{} {label}: canonical scan of expression {root} failed: {missing}",
                            pass.name()
                        ));
                        return;
                    }
                }
            }
        };

        let mut paired = true;
        for operator in CanonicalStateOperator::ALL {
            match pair_canonical_state_slots(root, scan, program, operator) {
                Ok(pairs) => {
                    for (site, emitted) in pairs {
                        self.claim(
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
                    self.mismatches
                        .push(format!("{} {label}: {error}", pass.name()));
                }
            }
        }
        if paired {
            self.paired_units += 1;
        }
    }

    /// Note a program in a pass with no canonical root.
    fn note_unrooted_program(&mut self, pass: Pass, label: &str, program: &BytecodeProgram) {
        self.programs += 1;
        self.note_allocated(program);
        if carries_state(program) {
            self.state_programs += 1;
            self.unrooted_state_programs += 1;
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
            self.unrooted.push(format!(
                "{} {label}: carries {families:?} state with no canonical root to pair against",
                pass.name()
            ));
        }
    }

    fn walk_parameters(
        &mut self,
        model: &CompiledModel,
        mir: &MirModel,
        layout: &CanonicalStateLayout,
        scans: &mut HashMap<ExprId, CanonicalStateSiteScan>,
    ) {
        for (index, parameter) in model.parameters.iter().enumerate() {
            let canonical = mir.parameters.get(index);
            let range = canonical.and_then(|parameter| parameter.range.as_ref());
            // Named slots rather than two collected lists. A parameter with a
            // constant default and an expression-valued bound compiles a `min`
            // program and no `default` program, so a positional zip over the
            // *present* programs would hand the bound program the default's
            // expression - a pairing that could refuse a module for a mistake
            // made here rather than by the generator.
            let mut slots: Vec<(String, &BytecodeProgram, Option<ExprId>)> = Vec::new();
            if let Some(program) = parameter.default_program.as_ref() {
                slots.push((
                    format!("parameter[{index}].default"),
                    program,
                    canonical
                        .and_then(|parameter| parameter.default_expr.as_ref().map(|expr| expr.id)),
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
                    Some(root) => self.pair_rooted_program(
                        mir,
                        layout,
                        scans,
                        Pass::Parameter,
                        &label,
                        root,
                        program,
                    ),
                    None => self.note_unrooted_program(Pass::Parameter, &label, program),
                }
            }
        }
    }

    fn walk_stamps(
        &mut self,
        model: &CompiledModel,
        mir: &MirModel,
        layout: &CanonicalStateLayout,
        scans: &mut HashMap<ExprId, CanonicalStateSiteScan>,
    ) {
        for (index, stamp) in model.stamp_programs.iter().enumerate() {
            let root = mir
                .equations
                .get(index)
                .map(|equation| equation.expression.id);
            let Some(root) = root else {
                self.note_unrooted_program(
                    Pass::EquationPrimal,
                    &format!("stamp[{index}].value"),
                    &stamp.value_program,
                );
                continue;
            };
            self.pair_rooted_program(
                mir,
                layout,
                scans,
                Pass::EquationPrimal,
                &format!("stamp[{index}].value"),
                root,
                &stamp.value_program,
            );
            if let Some(condition) = &stamp.static_condition {
                self.pair_rooted_program(
                    mir,
                    layout,
                    scans,
                    Pass::EquationPrimal,
                    &format!("stamp[{index}].static_condition"),
                    root,
                    condition,
                );
            }
            // One `compile_expr` per *pair* of entries on a current
            // contribution, one per entry on a branch row: the positive and
            // negative KCL rows share one compiled derivative by `clone()`, so
            // walking both would pair the same program twice.
            let stride = if stamp.branch_ordinal.is_none() { 2 } else { 1 };
            for (position, entry) in stamp.jacobian_programs.iter().step_by(stride).enumerate() {
                self.pair_rooted_program(
                    mir,
                    layout,
                    scans,
                    Pass::EquationDerivative,
                    &format!("stamp[{index}].jacobian[{position}]"),
                    root,
                    &entry.program,
                );
            }
        }
    }

    /// The passes with no canonical root: reactive derivatives, noise sources
    /// and `zi` definition operands.
    ///
    /// Walked so their slots enter [`Self::allocated`]. They contribute nothing
    /// to the map, so a slot only they address is unreached and refuses the
    /// module.
    fn walk_unrooted(&mut self, model: &CompiledModel) {
        for (index, stamp) in model.stamp_programs.iter().enumerate() {
            let stride = if stamp.branch_ordinal.is_none() { 2 } else { 1 };
            for (position, entry) in stamp.reactive_jacobians.iter().step_by(stride).enumerate() {
                self.note_unrooted_program(
                    Pass::ReactiveDerivative,
                    &format!("stamp[{index}].reactive_jacobian[{position}]"),
                    &entry.program,
                );
            }
        }

        for (index, source) in model.noise_sources.iter().enumerate() {
            self.note_unrooted_program(
                Pass::NoiseSource,
                &format!("noise[{index}].psd"),
                &source.psd_program,
            );
            if let Some(program) = &source.exponent_program {
                self.note_unrooted_program(
                    Pass::NoiseSource,
                    &format!("noise[{index}].exponent"),
                    program,
                );
            }
            for (position, injection) in source.injections.iter().enumerate() {
                self.note_unrooted_program(
                    Pass::NoiseSource,
                    &format!("noise[{index}].injection[{position}].gain"),
                    &injection.gain_program,
                );
            }
        }

        for (index, definition) in model.zi_filter_definitions.iter().enumerate() {
            let mut note = |suffix: String, program: &BytecodeProgram| {
                self.note_unrooted_program(
                    Pass::ZiDefinition,
                    &format!("zi_definition[{index}].{suffix}"),
                    program,
                );
            };
            for (half, polynomial) in [
                ("numerator", &definition.numerator),
                ("denominator", &definition.denominator),
            ] {
                match polynomial {
                    CompiledZiPolynomial::Coefficients(programs) => {
                        for (position, program) in programs.iter().enumerate() {
                            note(format!("{half}.coefficient[{position}]"), program);
                        }
                    }
                    CompiledZiPolynomial::Roots(pairs) => {
                        for (position, (real, imaginary)) in pairs.iter().enumerate() {
                            note(format!("{half}.root[{position}].real"), real);
                            note(format!("{half}.root[{position}].imaginary"), imaginary);
                        }
                    }
                }
            }
            note("period".to_owned(), &definition.period);
            note("first_transition".to_owned(), &definition.first_transition);
        }
    }

    /// Pair a whole assignment pass against the module's statement sites.
    ///
    /// The pass-level pairing is the pairing function's own rule applied to the
    /// concatenation: within one family the *k*-th emission is the *k*-th site,
    /// and a length disagreement is a refusal rather than a tolerance. Spelled
    /// against [`CanonicalStateLayout::statement_prefix`] rather than against a
    /// scan because the layout is the numbering the rewrite writes *into*.
    fn pair_assignment_pass(
        &mut self,
        layout: &CanonicalStateLayout,
        statements: &CanonicalStateLayout,
        pass: Pass,
        steps: &[AssignmentStep],
    ) {
        self.note_assignment_pass_programs(steps);
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
                self.mismatches.push(format!(
                    "{} pass: statements own {} {} sites {sites:?} but the pass emits {} slots \
                     {slots:?}",
                    pass.name(),
                    sites.len(),
                    operator.name(),
                    slots.len(),
                ));
                continue;
            }
            for (site, slot) in sites.into_iter().zip(slots) {
                self.claim(
                    layout,
                    operator.family(),
                    slot,
                    site,
                    &format!("{} pass", pass.name()),
                );
            }
        }
        if paired {
            self.paired_units += 1;
        }
    }

    /// Count the programs of an assignment pass and record every slot they
    /// address.
    fn note_assignment_pass_programs(&mut self, steps: &[AssignmentStep]) {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    self.programs += 1;
                    self.note_allocated(&assignment.program);
                    self.state_programs += usize::from(carries_state(&assignment.program));
                }
                AssignmentStep::AssignIndexed { index, value, .. } => {
                    for program in [value, index] {
                        self.programs += 1;
                        self.note_allocated(program);
                        self.state_programs += usize::from(carries_state(program));
                    }
                }
                AssignmentStep::Loop { condition, body } => {
                    self.programs += 1;
                    self.note_allocated(condition);
                    self.state_programs += usize::from(carries_state(condition));
                    self.note_assignment_pass_programs(body);
                }
            }
        }
    }
}

/// Every state instruction of an assignment pass, in the order the generator
/// emitted them.
///
/// Including the ordering detail that decides the answer:
/// `compile_assignment_items` binds an indexed assignment's *value* program
/// before it compiles the index expression.
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

/// Rewrite every state slot of a compiled model into its canonical site's
/// number.
///
/// Returns how many `(family, slot)` pairs moved; zero means the generator's
/// numbering already was the per-site one, which is the case for thirty-one of
/// the forty-three shipped modules.
///
/// The pass is idempotent: running it on a model it has already renumbered
/// maps each site's slot to itself.
pub(crate) fn renumber_state_slots_to_canonical_sites(
    model: &mut CompiledModel,
    hir: &HirModel,
    mir: &MirModel,
) -> Result<usize, StateRenumberingError> {
    let mapping = StateSlotMapping::build(model, hir, mir);
    let module = model.name.to_string();
    if let Some(refusal) = mapping.refusal(&module) {
        return Err(refusal);
    }
    let moved = mapping.moved();
    if moved == 0 {
        return Ok(0);
    }

    permute_backed_arrays(model, &mapping, &module)?;

    let mut rewrite = |program: &mut BytecodeProgram| {
        for instruction in &mut program.instructions {
            for operator in CanonicalStateOperator::ALL {
                let Some(emitted) = operator.bytecode_slot(instruction) else {
                    continue;
                };
                // `refusal` established that every addressed slot is mapped,
                // so the lookup cannot miss; leaving the slot alone if it
                // somehow did would be the one outcome worth avoiding, and the
                // debug assertion says so without costing a release branch.
                if let Some(site) = mapping.target(operator.family(), emitted) {
                    operator.rewrite_bytecode_slot(instruction, site);
                } else {
                    debug_assert!(
                        false,
                        "unmapped {:?} slot {emitted} survived the refusal check",
                        operator.family()
                    );
                }
                break;
            }
        }
    };
    for_each_program_mut(model, &mut rewrite);

    Ok(moved)
}

/// Move the compile-time data a renumbered family names so it stays under its
/// instruction.
fn permute_backed_arrays(
    model: &mut CompiledModel,
    mapping: &StateSlotMapping,
    module: &str,
) -> Result<(), StateRenumberingError> {
    permute(
        CanonicalStateFamily::LookupTable,
        mapping,
        module,
        &mut model.lookup_tables,
    )?;
    permute(
        CanonicalStateFamily::LaplaceFilter,
        mapping,
        module,
        &mut model.laplace_filters,
    )?;
    // The two `zi` arrays are index-aligned: the definition at slot `n`
    // freezes into the filter at slot `n`, so they permute together or the
    // lazy freeze writes the wrong site's coefficients.
    permute(
        CanonicalStateFamily::ZiFilter,
        mapping,
        module,
        &mut model.zi_filters,
    )?;
    permute(
        CanonicalStateFamily::ZiFilter,
        mapping,
        module,
        &mut model.zi_filter_definitions,
    )
}

/// Reorder one compile-time array so entry `i` moves to the site slot that
/// claims emitted slot `i`.
fn permute<T>(
    family: CanonicalStateFamily,
    mapping: &StateSlotMapping,
    module: &str,
    items: &mut Vec<T>,
) -> Result<(), StateRenumberingError> {
    let targets = (0..items.len())
        .map(|emitted| mapping.target(family, emitted).unwrap_or(emitted))
        .collect::<Vec<_>>();
    if targets.iter().copied().eq(0..items.len()) {
        return Ok(());
    }
    let mut seen = vec![false; items.len()];
    for &target in &targets {
        let Some(slot) = seen.get_mut(target) else {
            return Err(StateRenumberingError::BackedArrayNotPermutable {
                module: module.to_owned(),
                family,
                detail: format!(
                    "canonical slot {target} is outside the {} compiled entries",
                    items.len()
                ),
            });
        };
        if std::mem::replace(slot, true) {
            return Err(StateRenumberingError::BackedArrayNotPermutable {
                module: module.to_owned(),
                family,
                detail: format!("two compiled entries renumber to canonical slot {target}"),
            });
        }
    }

    let mut placed: Vec<Option<T>> = (0..items.len()).map(|_| None).collect();
    for (emitted, item) in items.drain(..).enumerate() {
        placed[targets[emitted]] = Some(item);
    }
    items.extend(placed.into_iter().map(|item| {
        item.expect("every canonical slot was claimed exactly once by the permutation check")
    }));
    Ok(())
}

/// Every bytecode program a compiled model holds.
///
/// Exhaustively destructured on purpose: a new program-bearing field on
/// [`CompiledModel`] stops the build here rather than shipping a program whose
/// state slots keep the emitted numbering while the rest of the module moves to
/// the per-site one.
fn for_each_program_mut(model: &mut CompiledModel, visit: &mut impl FnMut(&mut BytecodeProgram)) {
    let CompiledModel {
        name: _,
        source_digest: _,
        num_terminals: _,
        terminal_names: _,
        parameters,
        num_variables: _,
        variable_names: _,
        event_state_variables: _,
        assignment_steps,
        noise_assignment_steps,
        stamp_programs,
        lookup_tables: _,
        internal_nodes: _,
        branch_sources: _,
        laplace_filters: _,
        zi_filters: _,
        zi_filter_definitions,
        noise_process_schema: _,
        noise_sources,
    } = model;

    for parameter in parameters.iter_mut() {
        for program in parameter
            .default_program
            .iter_mut()
            .chain(parameter.min_program.iter_mut())
            .chain(parameter.max_program.iter_mut())
            .chain(parameter.exclude_programs.iter_mut())
        {
            visit(program);
        }
    }

    for steps in [assignment_steps, noise_assignment_steps] {
        visit_assignment_steps_mut(steps, visit);
    }

    for stamp in stamp_programs.iter_mut() {
        visit(&mut stamp.value_program);
        if let Some(condition) = stamp.static_condition.as_mut() {
            visit(condition);
        }
        for entry in stamp
            .jacobian_programs
            .iter_mut()
            .chain(stamp.reactive_jacobians.iter_mut())
        {
            visit(&mut entry.program);
        }
    }

    for definition in zi_filter_definitions.iter_mut() {
        for polynomial in [&mut definition.numerator, &mut definition.denominator] {
            match polynomial {
                CompiledZiPolynomial::Coefficients(programs) => {
                    for program in programs.iter_mut() {
                        visit(program);
                    }
                }
                CompiledZiPolynomial::Roots(pairs) => {
                    for (real, imaginary) in pairs.iter_mut() {
                        visit(real);
                        visit(imaginary);
                    }
                }
            }
        }
        visit(&mut definition.period);
        visit(&mut definition.first_transition);
    }

    for source in noise_sources.iter_mut() {
        visit(&mut source.psd_program);
        if let Some(program) = source.exponent_program.as_mut() {
            visit(program);
        }
        for injection in source.injections.iter_mut() {
            visit(&mut injection.gain_program);
        }
    }
}

fn visit_assignment_steps_mut(
    steps: &mut [AssignmentStep],
    visit: &mut impl FnMut(&mut BytecodeProgram),
) {
    for step in steps.iter_mut() {
        match step {
            AssignmentStep::Assign(assignment) => visit(&mut assignment.program),
            AssignmentStep::AssignIndexed { index, value, .. } => {
                visit(value);
                visit(index);
            }
            AssignmentStep::Loop { condition, body } => {
                visit(condition);
                visit_assignment_steps_mut(body, visit);
            }
        }
    }
}
