//! Is the canonical-to-bytecode state-slot pairing total over the shipped
//! corpus, and does it still induce a renumbering?
//!
//! [`crate::canonical_ir::state`] records that three numberings of a module's
//! analog-operator records exist and that the JIT runtime takes the per-*site*
//! one. Moving it there means rewriting every slot index the bytecode generator
//! handed out per *emission* into the site's own number. A rewrite is only well
//! defined if, for every program the generator compiled, each state instruction
//! can be told which canonical site it came from.
//!
//! [`StateSlotMapping`] is that rewrite. This census asks it about **every**
//! program of **every** shipped module, in every context `generate_from_ir`
//! compiles, and reports four things per module:
//!
//! * **programs paired** — how many carry state and pair without error;
//! * **count mismatches** — the pairing's own refusal, with the context, the
//!   canonical count and the bytecode count, which is what the renumbering
//!   refuses the module for;
//! * **unreached slots** — a `(family, slot)` some program addresses that no
//!   pairing ever named. A renumbering that left one behind would leave an
//!   instruction reading a record nothing writes;
//! * **conflicts** — one emitted slot claimed by two different canonical sites,
//!   which would make the rewrite a non-function.
//!
//! The census does not build that map itself: it reads
//! [`StateSlotMapping::build`], the same construction
//! [`renumber_state_slots_to_canonical_sites`] runs inside the compiler. What
//! the census measures is therefore the shipped code rather than a second
//! implementation of it, and `moved` is what that code would still change.
//!
//! ## `moved` is now zero, and that is the assertion
//!
//! Before W-F1b, twelve modules reported `moved > 0`: the generator's
//! per-emission numbering and the canonical per-site numbering genuinely
//! disagreed for them. The compiler now renumbers at
//! `compile_file_runtime_with_metadata`, which is the entry this census's model
//! provider reads, so the models arriving here are already in the per-site
//! numbering and the mapping is the identity. A module reporting `moved > 0`
//! means the renumbering did not reach it, and that is a failure rather than a
//! measurement.
//!
//! ## Which canonical expression each context is paired against
//!
//! See [`crate::codegen::state_renumbering`], which owns the walk and documents
//! the root each pass is paired with.
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

use super::census_models::shipped_census_models_matching;
use super::cfg_census::{PrefixShape, integration_emission_contexts, prefix_shape};
use crate::canonical_ir::{CanonicalStateFamily, CanonicalStateLayout};
use crate::codegen::state_renumbering::{
    StateSlotMapping, renumber_state_slots_to_canonical_sites,
};

/// Whether every shipped module's emitted state slots can be told which
/// canonical site owns them, and whether the compiler has already put them
/// there.
///
/// See the module documentation for what each column means. The assertions are
/// the three properties a renumbering cannot be built without — no conflict, no
/// unreached slot, no unpairable program — plus the one the renumbering
/// establishes: nothing left to move.
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

        let mapping = StateSlotMapping::build(&runtime.model, &artifact.hir, &artifact.mir);

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

        let unreached = mapping.unreached();
        let moved = mapping.moved();

        total_programs += mapping.programs;
        total_state_programs += mapping.state_programs;
        total_paired += mapping.paired_units;
        if !mapping.mismatches.is_empty() || !mapping.unrooted.is_empty() {
            models_with_mismatch += 1;
        }
        if !unreached.is_empty() {
            models_with_unreached += 1;
        }
        if !mapping.conflicts.is_empty() {
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
            mapping.programs,
            mapping.state_programs,
            mapping.paired_units,
            mapping.unrooted_state_programs,
            mapping.mismatches.len(),
            mapping.conflicts.len(),
            mapping.unnumbered.len(),
            mapping.allocated.len(),
            mapping.map.len(),
            unreached.len(),
            moved,
            tags.len(),
        );
        for mismatch in mapping.mismatches.iter().chain(&mapping.unrooted) {
            println!("  pairing-mismatch model={module} {mismatch}");
        }
        for conflict in &mapping.conflicts {
            println!("  pairing-conflict model={module} {conflict}");
        }
        for unnumbered in &mapping.unnumbered {
            println!("  pairing-unnumbered model={module} {unnumbered}");
        }
        for (family, slot) in &unreached {
            println!("  pairing-unreached model={module} family={family:?} slot={slot}");
        }
        if !mapping.mismatches.is_empty()
            || !mapping.conflicts.is_empty()
            || !mapping.unrooted.is_empty()
        {
            refusals.push(module.clone());
        }

        // The compiler's own entry point, run a second time on a model that
        // has already been through it. A pass that is not idempotent would
        // renumber twice and land somewhere neither numbering names.
        let mut again = runtime.model.clone();
        let moved_again =
            renumber_state_slots_to_canonical_sites(&mut again, &artifact.hir, &artifact.mir)
                .unwrap_or_else(|error| panic!("re-renumbering {module}: {error}"));
        assert_eq!(
            moved_again, 0,
            "renumbering {module} a second time moved {moved_again} slots, so the pass is not \
             idempotent"
        );
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
    assert_eq!(
        models_renumbered, 0,
        "a shipped module still carries the generator's per-emission state numbering, so the \
         compiler's renumbering did not reach it"
    );
    if filter.is_none() {
        assert_eq!(models, 43, "the shipped census is 43 modules");
    }
}
