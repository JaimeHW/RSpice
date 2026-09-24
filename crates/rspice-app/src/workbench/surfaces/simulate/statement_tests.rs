//! The statement the Analyses page shows is the statement the engine parses.
//!
//! A displayed directive is a claim about the deck. If the page spells it and
//! the queue builder emits it, the two are written separately and will drift —
//! and the failure is silent and confident: the reader sees a plausible SPICE
//! card, the engine is handed a different one, and nothing in either surface
//! says so. That is worse than showing nothing at all.
//!
//! So the page does not spell anything. It calls
//! [`crate::simulation::SimulationController::analysis_draft_directive`], which
//! is also what `simulation::controller::directive_parse_ratchet` calls before
//! feeding the result to the real engine parser. These tests pin that the
//! page's own accessor returns exactly that string, and that the string still
//! parses when it arrives from the page's path rather than the ratchet's.

use crate::simulation::controller::SimulationController;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::workbench::RSpiceApp;

#[test]
fn the_displayed_statement_is_the_directive_the_controller_emits() {
    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.sync_active_schematic_to_workspace();

    for kind in [
        AnalysisKind::OperatingPoint,
        AnalysisKind::Transient,
        AnalysisKind::Ac,
        AnalysisKind::DcSweep,
    ] {
        let draft = AnalysisDraft::for_kind(kind);
        let displayed =
            super::plan_statement_for(&mut app.state, &app.simulation_controller, &draft);

        // The same call the page makes, against the same projection, reached
        // independently here. If the page ever starts spelling its own
        // statement this diverges immediately.
        let mut projected = app.state.clone();
        projected.sim_setup.apply_analysis_draft_projection(&draft);
        let emitted = SimulationController::new().analysis_draft_directive(&projected, &draft);

        assert_eq!(
            displayed,
            emitted,
            "{} must display the directive it emits, not a re-spelling of it",
            kind.label()
        );
    }
}

#[test]
fn a_displayed_statement_is_a_directive_the_engine_accepts() {
    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.sync_active_schematic_to_workspace();

    // A deck the directive can be appended to. Deliberately the smallest thing
    // that parses: this test is about the card the page showed, not about the
    // fixture around it.
    const DECK: &str = "* statement fixture\nV1 in 0 DC 1\nR1 in out 1k\nR2 out 0 1k\n";

    let mut checked = 0_usize;
    for kind in AnalysisKind::ALL {
        if kind.execution_blocker().is_some() {
            continue;
        }
        let draft = AnalysisDraft::for_kind(kind);
        // A default draft that needs an authored output or source emits no
        // directive at all, and the page shows the refusal instead. That is
        // the editor's contract, not a claim about the parser.
        let Ok(statement) =
            super::plan_statement_for(&mut app.state, &app.simulation_controller, &draft)
        else {
            continue;
        };
        checked += 1;
        let deck = format!("{DECK}{statement}\n.end\n");
        assert!(
            rspice_core::netlist::parse_netlist(&deck).is_ok(),
            "{} displays `{statement}`, which the engine refuses",
            kind.label()
        );
    }

    assert!(
        checked > 0,
        "the sweep must actually reach some statements, or it proves nothing"
    );
}

#[test]
fn reading_a_statement_leaves_the_plan_exactly_as_it_found_it() {
    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.sync_active_schematic_to_workspace();

    // The helper projects the draft into the live setup view and restores it,
    // because cloning a whole application once a frame to read one line would
    // be the most expensive thing on the route. Restoring it *exactly* is what
    // makes that safe, and an inexact restore would silently rewrite the
    // operator's configuration as a side effect of looking at it.
    // Compared as serialized state rather than by adding a `PartialEq` derive
    // to a struct this large: the question is whether any field moved, and the
    // serialization already answers exactly that over every field.
    let before = serde_json::to_vec(&app.state.sim_setup).expect("the setup view serializes");
    let _ = super::plan_statement_for(
        &mut app.state,
        &app.simulation_controller,
        &AnalysisDraft::for_kind(AnalysisKind::Transient),
    );
    let _ = super::plan_statement_for(
        &mut app.state,
        &app.simulation_controller,
        &AnalysisDraft::for_kind(AnalysisKind::Ac),
    );
    let after = serde_json::to_vec(&app.state.sim_setup).expect("the setup view serializes");

    assert!(
        before == after,
        "reading a plan statement must not change the setup it read"
    );
}

/// A NOISE instance's frequency axis, beside an AC sibling that disagrees.
///
/// Noise is the one analysis on this surface whose axis has two owners in the
/// code. [`crate::simulation::plan::NoiseDraft`] carries its own band, count
/// and mode; the retired singleton view carries a *second* copy, and its noise
/// projection reads the count and the mode out of the **AC** slots rather than
/// the noise ones — `SimSetupState::legacy_analysis_draft` builds
/// `AnalysisKind::Noise` from `self.ac.points` and `self.ac.sweep`. The plan
/// projection then applies AC last on purpose, so on a plan holding both, the
/// AC instance is what those slots end up holding.
///
/// Both facts are true at once and neither is obvious, so both are pinned
/// here on one plan whose two instances disagree about every field of the
/// axis. The instance path is what a run dispatches and what the Analyses page
/// displays: each instance states its own axis, and the sibling's numbers do
/// not appear in it. The singleton path is the compatibility view, and the
/// derivation it performs is stated rather than left to be discovered by
/// whoever next changes the ordering in
/// `SimSetupState::refresh_legacy_analysis_projections`.
#[test]
fn a_noise_instance_and_its_ac_sibling_each_state_their_own_frequency_axis() {
    use crate::product::AnalysisInstanceId;
    use crate::simulation::config::NoiseSweepType;

    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.sync_active_schematic_to_workspace();

    let (ac, noise) = {
        let plan = app
            .state
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("the fixture plan is migrated");
        // Both small-signal kinds take their linearization from an operating
        // point, and a plan whose prerequisites are unbound does not freeze.
        // The lookups finish before `insert` borrows the plan mutably, so each
        // is two statements rather than one `unwrap_or_else`.
        let existing_op = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::OperatingPoint)
            .map(|instance| instance.id());
        let op = match existing_op {
            Some(id) => id,
            None => {
                plan.insert_at(AnalysisKind::OperatingPoint, 0)
                    .expect("OP inserts")
                    .0
            }
        };
        let existing_ac = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == AnalysisKind::Ac)
            .map(|instance| instance.id());
        let ac = match existing_ac {
            Some(id) => id,
            None => plan.insert(AnalysisKind::Ac).expect("AC inserts").0,
        };
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
            .expect("AC binds its operating point");
        plan.edit(ac, |draft| {
            let AnalysisDraft::Ac(draft) = draft else {
                panic!("an AC instance holds an AC draft");
            };
            draft.sweep = 0;
            draft.points = "7".to_owned();
            draft.fstart = "2".to_owned();
            draft.fstop = "2Meg".to_owned();
        })
        .expect("the AC axis is authorable");

        let noise = plan.insert(AnalysisKind::Noise).expect("NOISE inserts").0;
        plan.edit(noise, |draft| {
            let AnalysisDraft::Noise(draft) = draft else {
                panic!("a NOISE instance holds a noise draft");
            };
            // Every field of the axis disagrees with the AC sibling's, so a
            // line that borrowed any one of them fails here by that field.
            draft.output = "out".to_owned();
            draft.input = "V1".to_owned();
            draft.sweep = NoiseSweepType::Linear;
            draft.points = "47".to_owned();
            draft.fstart = "10".to_owned();
            draft.fstop = "100k".to_owned();
        })
        .expect("the noise axis is authorable");
        plan.bind_dependency(noise, AnalysisKind::OperatingPoint, op)
            .expect("NOISE binds its operating point");
        (ac, noise)
    };

    let frozen = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the fixture plan is migrated")
        .freeze()
        .expect("the fixture plan freezes");
    let setup = app.state.sim_setup.clone();
    // Exactly the two steps `SimulationController::build_queue_from_plan`
    // takes per instance: project that instance's own draft into the retired
    // view, then ask the controller for the card. Nothing is re-spelled here.
    let directive = |id: AnalysisInstanceId| -> String {
        let instance = frozen
            .instances()
            .iter()
            .find(|instance| instance.id() == id)
            .expect("the instance is in the frozen plan");
        let mut projected = app.state.clone();
        projected.sim_setup = setup
            .frozen_instance_projection(&frozen, instance)
            .expect("the instance projects");
        app.simulation_controller
            .analysis_draft_directive(&projected, instance.draft())
            .expect("both instances are completely authored")
    };

    assert_eq!(
        directive(ac),
        ".ac dec 7 2 2000000",
        "the AC instance states its own axis"
    );
    assert_eq!(
        directive(noise),
        ".noise V(out) V1 lin 47 10 100000",
        "the NOISE instance states its own axis, not the AC sibling's"
    );

    // And both are cards the engine reads, which is what makes the strings
    // above a claim about the deck rather than about a formatter.
    const DECK: &str = "* noise beside ac\nV1 in 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n";
    for id in [ac, noise] {
        let deck = format!("{DECK}{}\n.end\n", directive(id));
        assert!(
            rspice_core::netlist::parse_netlist(&deck).is_ok(),
            "the engine refuses `{}`",
            directive(id)
        );
    }

    // The retired singleton view, which is the one place the AC sibling does
    // supply a noise axis. Its band comes from the noise slots and its count
    // and mode from the AC ones, because that is what `legacy_analysis_draft`
    // reads — and because the plan projection applies AC last so that those
    // slots hold the AC instance rather than the noise instance that wrote
    // them a moment earlier.
    app.state.sim_setup.refresh_legacy_analysis_projections();
    let AnalysisDraft::Noise(singleton) = app
        .state
        .sim_setup
        .legacy_analysis_draft(AnalysisKind::Noise)
    else {
        panic!("the noise kind captures a noise draft");
    };
    assert_eq!(singleton.fstart, "10", "the band is the noise instance's");
    assert_eq!(singleton.fstop, "100k", "the band is the noise instance's");
    assert_eq!(
        singleton.points, "7",
        "the singleton view derives the count from the AC slots"
    );
    assert_eq!(
        singleton.sweep,
        NoiseSweepType::Decade,
        "the singleton view derives the mode from the AC slots"
    );
}
