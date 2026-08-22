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

use crate::simulation::multi_run::AnalysisSpec;
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
/// or ask for something no default can invent — and which ones those are is
/// [`KINDS_THAT_STATE_NO_DIRECTIVE`], not something the reader has to work out
/// from the gap.
const KINDS_THAT_EMIT_A_DIRECTIVE: usize = 26;

/// The kinds whose default draft reaches no directive, and what they are
/// waiting for.
///
/// The walk used to skip them with a bare `let Ok(..) else { continue }`, so an
/// emitter that *started* failing would lower the count and name nothing: the
/// floor above is one short of the catalogue for exactly one kind, and no test
/// said which. Naming them makes the exemption a decision. A kind added here
/// has to state what a default draft cannot invent for it; a kind that stops
/// emitting without being added fails this test.
///
/// Each entry is `{label} — {the refusal the emitter returned}`, so the
/// exemption is the emitter's own words rather than a second account of them.
///
/// One kind, and its reason is the good one: an envelope run is a carrier
/// modulated by sources in the user's own design, and no default draft can
/// invent which of them carries the modulation. That is the editor's contract
/// — [`crate::simulation::dialog::EnvelopeConfig`] refuses the draft before a
/// directive is ever asked for — and it is why this ratchet's floor is one
/// short of the catalogue's non-blocked kinds rather than equal to it.
const KINDS_THAT_STATE_NO_DIRECTIVE: &[&str] =
    &["Envelope — invalid envelope settings: At least one modulation source is required"];

/// The directive `kind` writes from its default draft, or why it writes none.
///
/// Deliberately no local expansion of the three-step build: this calls the same
/// [`SimulationController::analysis_draft_directive`] the Analyses page shows
/// its operator, so what this ratchet proves parses is exactly the string that
/// surface displays. Inlining the steps here again would let the two drift and
/// leave the page claiming a directive the engine had never been offered.
fn directive_for(kind: AnalysisKind) -> Result<String, String> {
    let controller = SimulationController::new();
    let draft = fixture_draft(kind);
    let state = engine_facing_state(&draft);
    controller.analysis_draft_directive(&state, &draft)
}

#[test]
fn every_emitted_analysis_directive_parses_as_the_engine_reads_it() {
    let mut refused: Vec<String> = Vec::new();
    let mut silent: Vec<String> = Vec::new();
    let mut emitted = 0_usize;

    for kind in AnalysisKind::ALL {
        // A kind with no solver in this build is refused by name long before a
        // deck is written, and makes no claim about the parser.
        if kind.execution_blocker().is_some() {
            continue;
        }
        // A draft that needs an authored output or source never reaches a
        // directive at all. That is the editor's contract, not the parser's —
        // but it is recorded rather than skipped, so the set of kinds this
        // ratchet does not cover is stated instead of inferred.
        let directive = match directive_for(kind) {
            Ok(directive) => directive,
            Err(reason) => {
                silent.push(format!("{} — {reason}", kind.label()));
                continue;
            }
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
    assert_eq!(
        silent, KINDS_THAT_STATE_NO_DIRECTIVE,
        "the kinds this ratchet does not cover are declared, not discovered: update \
         KINDS_THAT_STATE_NO_DIRECTIVE, or fix the emitter that stopped writing a card"
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
        panic!(
            "an HB plan writes exactly one .HB card: {:?}",
            netlist.analyses
        );
    };
    assert_eq!(
        frequencies,
        &[crate::simulation::dialog::hb::HbConfig::default().fundamental_freq]
    );
}

/// An inherited temperature axis reaches the deck as the axis the plan
/// declared, and reads back as those exact temperatures.
///
/// The walk above proves every kind's *default* draft writes a card the parser
/// accepts. It cannot prove more than that, because a default draft is by
/// construction the one configuration nobody authored — and the two settings
/// below are the ones where the card's content comes from somewhere other than
/// the form that emitted it.
///
/// This is the first: `TempAxisMode::InheritRunSetAxis` means the form states
/// no temperatures at all and the plan's own run-set axis supplies them. A
/// directive that fell back to the retained Start/Stop/Step would still parse
/// — it is a well-formed `.step temp` either way — so parsing is exactly the
/// property that cannot catch it. The values are read back out of the parsed
/// card and compared with the declaration they came from.
#[test]
fn an_inherited_temperature_axis_round_trips_as_the_axis_the_plan_declared() {
    use crate::simulation::plan::AnalysisDraft;
    use crate::simulation::run_set::{RunSetAction, RunSetDimensionKind, dispatch};

    const DECLARED: [f64; 3] = [-55.0, 27.0, 150.0];

    let mut draft = fixture_draft(AnalysisKind::Temperature);
    let AnalysisDraft::Temperature(temp) = &mut draft else {
        panic!("the temperature kind carries a temperature draft");
    };
    // `TempAxisMode::ALL[1]`. Set before the projection below, because that is
    // what the builder reads.
    temp.axis_mode_idx = 1;
    // A draft that has never been opened is re-seeded from the default config
    // by `ensure_initialized`, which would take the axis mode back with it.
    temp.initialized = true;
    // The retained range stays authored and stays *wrong* on purpose: if the
    // emitter ever falls back to it, the card below is `-40 125 25` and the
    // three declared temperatures are gone.
    temp.temp_start = "-40".to_owned();
    temp.temp_stop = "125".to_owned();
    temp.temp_step = "25".to_owned();

    let mut state = engine_facing_state(&draft);
    let axis = state
        .sim_setup
        .run_set
        .dimensions
        .iter()
        .find(|dimension| dimension.kind == RunSetDimensionKind::Temperature)
        .expect("the default run set declares a temperature axis")
        .id
        .clone();
    let transaction = dispatch(
        &mut state.sim_setup.run_set,
        RunSetAction::SetValues {
            id: axis,
            // One value per line: the axis is authored the way the run-set
            // page authors it, not through a spelling of this test's own.
            text: "-55\n27\n150".to_owned(),
        },
        1,
    );
    assert!(
        transaction.was_adopted(),
        "the fixture axis must actually be declared: {:?}",
        transaction.receipt
    );
    assert_eq!(
        state
            .sim_setup
            .run_set
            .declared_temperatures_celsius(state.sim_setup.reference_pvt),
        Some(DECLARED.to_vec()),
        "the premise of this test is the declaration it inherits"
    );

    let directive = SimulationController::new()
        .analysis_draft_directive(&state, &draft)
        .expect("an inherited axis emits a card");
    assert_eq!(directive, ".step temp list -55 27 150");

    let deck = format!("{FIXTURE_DECK}.op\n{directive}\n.end\n");
    let netlist = rspice_core::netlist::parse_netlist(&deck)
        .unwrap_or_else(|error| panic!("the inherited axis must parse: {error}\n{deck}"));
    let step = netlist
        .analyses
        .iter()
        .find_map(|command| match command {
            rspice_core::netlist::AnalysisCommand::Step(step) => Some(step),
            _ => None,
        })
        .expect("the deck carries one .step card");
    assert_eq!(step.target, rspice_core::netlist::StepTarget::Temp);
    let rspice_core::netlist::StepSweep::List(values) = &step.sweep else {
        panic!(
            "an inherited axis is a list of declared temperatures, not a range: {:?}",
            step.sweep
        );
    };
    assert_eq!(
        values.as_slice(),
        DECLARED.as_slice(),
        "the parsed card must carry the declared axis, temperature for temperature"
    );
}

/// An autonomous PSS reaches the deck as autonomous, and reads back that way.
///
/// The second setting the parse walk cannot see. `.pss` is not a card the
/// engine's own netlist parser owns — it has no `AnalysisCommand` for it — so
/// `parse_netlist` accepting the line proves only that the line is well formed.
/// The reader that turns it back into an analysis is the studio's own,
/// `controller::manual_deck`, which is what a hand-written deck goes through,
/// and that is where a dropped `autonomous=` or a lost oscillator node would
/// show up: an oscillator solved as a driven circuit converges on the trivial
/// answer and reports it confidently.
///
/// So the round trip is closed against that reader. Driven is asserted beside
/// autonomous because a flag that is always true reads the same as one that is
/// carried.
#[test]
fn an_autonomous_pss_round_trips_through_the_deck_reader_as_autonomous() {
    use crate::simulation::plan::AnalysisDraft;

    let read_back = |osc_mode: bool| -> AnalysisSpec {
        let mut draft = fixture_draft(AnalysisKind::Pss);
        let AnalysisDraft::Pss(pss) = &mut draft else {
            panic!("the PSS kind carries a PSS draft");
        };
        pss.initialized = true;
        pss.osc_mode = osc_mode;
        // An autonomous run finds its own period and the editor refuses a tone
        // beside it, so the two modes are configured as the form allows them:
        // a node to watch, or a source to be driven by. `FIXTURE_DECK` places
        // `VSRC`, so the driven tone is a name the deck actually carries.
        if osc_mode {
            pss.osc_node = "n_out".to_owned();
            pss.tone_sources = String::new();
        } else {
            pss.osc_node = String::new();
            pss.tone_sources = "VSRC".to_owned();
        }
        let state = engine_facing_state(&draft);
        let directive = SimulationController::new()
            .analysis_draft_directive(&state, &draft)
            .expect("a PSS draft emits a card");
        assert_eq!(
            directive.contains("autonomous=yes"),
            osc_mode,
            "the card must state the mode it was configured in: {directive}"
        );

        let deck = format!("{FIXTURE_DECK}{directive}\n.end\n");
        let queue = super::manual_deck::build_manual_deck_queue(&state, &deck)
            .unwrap_or_else(|errors| panic!("the deck reader refused: {}", errors.join("; ")));
        queue
            .into_iter()
            .map(|queued| queued.spec)
            .find(|spec| matches!(spec, AnalysisSpec::Pss { .. }))
            .expect("the deck reader recovers the PSS")
    };

    let AnalysisSpec::Pss {
        oscillator_mode,
        oscillator_node,
        tone_sources,
        ..
    } = read_back(true)
    else {
        unreachable!("filtered to the PSS spec");
    };
    assert!(
        oscillator_mode,
        "an autonomous PSS that reads back as driven would solve the trivial answer"
    );
    assert_eq!(oscillator_node.as_deref(), Some("n_out"));
    assert!(
        tone_sources.is_empty(),
        "an autonomous run has no driven tone: {tone_sources:?}"
    );

    let AnalysisSpec::Pss {
        oscillator_mode,
        oscillator_node,
        tone_sources,
        ..
    } = read_back(false)
    else {
        unreachable!("filtered to the PSS spec");
    };
    assert!(!oscillator_mode);
    assert_eq!(oscillator_node, None);
    assert_eq!(tone_sources, vec!["VSRC".to_owned()]);
}
