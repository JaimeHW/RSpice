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
