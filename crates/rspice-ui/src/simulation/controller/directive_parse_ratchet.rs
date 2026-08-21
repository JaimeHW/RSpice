//! Every directive the studio writes must be one the engine can read back.
//!
//! An analysis reaches the solver as two things: a typed `AnalysisSpec` the
//! runner dispatches on, and a SPICE directive spliced into the executable
//! deck. Preparation parses that deck before any of it runs, so a directive
//! the parser refuses does not degrade the run — it refuses the whole plan,
//! every analysis in it, before a single solve starts.
//!
//! Harmonic balance shipped that way. `HbConfig::to_spice` wrote
//! `.hb 1G harmonics=9 oversample=2`, the `.HB` arm of the netlist parser
//! reads a `.HB` card as frequencies and nothing else (Xyce's spelling, and
//! the only dialect that spells `.HB` at all), and so every plan holding an
//! HB instance failed preparation. The analysis was advertised as available
//! with no execution blocker against it, and could not be run at all.
//!
//! Nothing caught it because each emitter was only ever asserted against its
//! own expected string. A string assertion passes for a key no parser knows —
//! it is the same blind spot `dialog::options::model` names, and the same
//! answer: read the emitted text back through the real parser.
//!
//! This walks the whole catalogue rather than harmonic balance alone, because
//! the defect is not about HB. Any emitter can drift from the parser, and the
//! next one to do it will be found here rather than by a user whose plan will
//! not start.

use crate::simulation::plan::AnalysisKind;

use super::SimulationController;
use super::projection_ratchet::{engine_facing_state, fixture_draft};

/// A deck with the nodes and sources the fixture drafts name.
///
/// `VSRC` and `V(n_out)` are what [`projection_ratchet`]'s design-named
/// fixture fills in, so a directive that carries a source or an output names
/// something this circuit actually has. The probe inductor is here for the
/// loop-stability kinds, which designate a series element rather than a node.
///
/// [`projection_ratchet`]: super::projection_ratchet
const FIXTURE_DECK: &str = "\
directive parse ratchet
VSRC in 0 SIN(0 1 1k)
R1 in mid 1k
LPRB mid n_out 1n
C1 n_out 0 1n
R2 n_out 0 10k
";

/// How many kinds reach a directive from their default draft on this fixture.
///
/// A floor, not a census: it may rise freely, and a change that lowers it has
/// narrowed what this ratchet watches and should say so out loud rather than
/// quietly re-baseline. The kinds below it either carry an execution blocker
/// or ask for something no default can invent.
const KINDS_THAT_EMIT_A_DIRECTIVE: usize = 26;

/// The directive `kind` writes from its default draft, or why it writes none.
fn directive_for(kind: AnalysisKind) -> Result<String, String> {
    let controller = SimulationController::new();
    let draft = fixture_draft(kind);
    let state = engine_facing_state(&draft);

    let spec = match controller.build_manifest_preview_spec(&state, &draft) {
        Ok(Some(spec)) => spec,
        Ok(None) => controller.build_analysis_spec_for_index(&state, kind.legacy_index())?,
        Err(error) => return Err(error),
    };
    controller.analysis_spec_to_spice_line(&state, &spec)
}

#[test]
fn every_emitted_analysis_directive_parses_as_the_engine_reads_it() {
    let mut refused: Vec<String> = Vec::new();
    let mut emitted = 0_usize;

    for kind in AnalysisKind::ALL {
        // A kind with no solver in this build is refused by name long before a
        // deck is written, and makes no claim about the parser.
        if kind.execution_blocker().is_some() {
            continue;
        }
        // A draft that needs an authored output or source never reaches a
        // directive at all. That is the editor's contract, not the parser's.
        let Ok(directive) = directive_for(kind) else {
            continue;
        };
        emitted += 1;

        let deck = format!("{FIXTURE_DECK}{directive}\n.end\n");
        if let Err(error) = rspice_core::netlist::parse_netlist(&deck) {
            refused.push(format!(
                "{} emits `{directive}`, which the engine refuses: {error}",
                kind.label()
            ));
        }
    }

    assert!(
        refused.is_empty(),
        "directives the studio writes and the engine cannot read — a plan holding any of these \
         analyses fails preparation outright:\n  {}",
        refused.join("\n  ")
    );
    assert!(
        emitted >= KINDS_THAT_EMIT_A_DIRECTIVE,
        "only {emitted} kinds reached a directive; this ratchet is only worth what it covers"
    );
}

#[test]
fn the_harmonic_balance_directive_carries_the_tones_it_was_given() {
    // The specific case that was broken, pinned end to end: what the dialog
    // writes is what the parser hands the engine back.
    let directive =
        directive_for(AnalysisKind::HarmonicBalance).expect("a default HB draft emits a card");
    let deck = format!("{FIXTURE_DECK}{directive}\n.end\n");
    let netlist = rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the HB directive must parse: {error}\n{deck}"));

    let [rspice_core::netlist::AnalysisCommand::Hb { frequencies }] = netlist.analyses.as_slice()
    else {
        panic!("an HB plan writes exactly one .HB card: {:?}", netlist.analyses);
    };
    assert_eq!(
        frequencies,
        &[crate::simulation::dialog::hb::HbConfig::default().fundamental_freq]
    );
}
