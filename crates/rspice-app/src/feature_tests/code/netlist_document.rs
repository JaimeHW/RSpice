//! The netlist document over the whole session: a run's deck snapshot
//! beside the open deck, and a sweep point's executed deck on screen.

use crate::workbench::AppState;
use rspice_app_code::workbench::documents::netlist_document::{
    NetlistRoot, active_netlist_source_is_editable_in, close_run_deck_snapshot,
    open_run_deck_snapshot_in,
};

/// The deck a run consumed is not the deck in the editor: opening the snapshot
/// must project the sealed bytes, and the working source must survive intact.
#[test]
fn run_deck_snapshot_projects_the_sealed_deck_and_leaves_the_working_deck_alone() {
    const RAN: &str = "ran deck\nR1 out 0 1k\n.op\n.end\n";
    const EDITED: &str = "ran deck\nR1 out 0 2k\n.op\n.end\n";

    let mut state = AppState::default();
    state.seed_test_deck(EDITED.to_owned());
    state.runs.manual_run.last_run_buffer = Some(RAN.to_owned());
    state.runs.manual_run.last_run_id = Some(3);

    assert!(open_run_deck_snapshot_in(
        &mut state.model,
        &mut state.code.netlist,
        &state.runs
    ));
    assert_eq!(state.code.netlist.front.root_kind(), NetlistRoot::RunInput);
    assert_eq!(state.model.simulation.source.netlist_content, RAN);
    assert!(!active_netlist_source_is_editable_in(
        &state.model,
        &state.code.netlist
    ));
    assert_eq!(
        state
            .code
            .netlist
            .open_deck
            .as_ref()
            .map(|deck| deck.root.text.as_str()),
        Some(EDITED)
    );
    assert_eq!(
        state.code.netlist.run_target(),
        rspice_app_code::workbench::documents::netlist_tabs::RunTarget::Deck(
            state
                .code
                .netlist
                .open_deck
                .as_ref()
                .expect("the deck stays open")
                .id
        ),
        "with the run's input in front, Run still runs the deck"
    );

    assert!(close_run_deck_snapshot(
        &mut state.model,
        &mut state.code.netlist
    ));
    assert_eq!(state.code.netlist.front.root_kind(), NetlistRoot::Deck);
    assert_eq!(state.model.simulation.source.netlist_content, EDITED);
}

/// A sweep point's executed deck is the text on screen on every frame after it
/// is opened, not only the first, whatever deck the last manual run left
/// behind: the strip above it names that point, so any other text under it
/// would be a different deck under the wrong name.
#[test]
fn a_sweep_points_executed_deck_stays_on_screen_on_the_next_frames() {
    use crate::workbench::app::veriloga_design_tests::{
        DESKTOP, frame, open_project_app, themed_context,
    };
    use rspice_app_code::workbench::documents::netlist_document::reveal_executed_deck;
    use rspice_app_state::state::{ExecutedDeck, ExecutedDeckPoint};

    const MANUAL: &str = "manual deck\nR1 out 0 1k\n.op\n.end\n";
    const NOMINAL: &str = "task deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    const HOT: &str = "task deck\n.OPTIONS TEMP=125\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

    let mut app = open_project_app();
    app.state.runs.manual_run.last_run_buffer = Some(MANUAL.to_owned());
    app.state.runs.manual_run.last_run_id = Some(3);
    let point = |label: &str, deck: &str| ExecutedDeckPoint {
        label: label.to_owned(),
        deck: deck.into(),
        model_sources: Vec::new(),
    };
    app.state
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: 4,
            points: vec![point("TT 27C", NOMINAL), point("TT 125C", HOT)],
        });
    let state = &mut app.state;
    assert!(reveal_executed_deck(
        &mut state.model,
        &mut state.code.netlist,
        &state.runs,
        &mut state.workbench,
        4,
        1
    ));

    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    let output = frame(&ctx, &mut app, DESKTOP);

    assert_eq!(
        app.state.model.simulation.source.netlist_content, HOT,
        "the editor still holds the point's deck"
    );
    let painted = painted_text(&output);
    assert!(
        painted.contains(".OPTIONS TEMP=125"),
        "the point's own card is painted: {painted}"
    );
    assert!(
        !painted.contains("manual deck"),
        "the last manual run's deck is not painted under the point's name"
    );
}

/// Every string `output` painted, one per line.
pub(super) fn painted_text(output: &egui::FullOutput) -> String {
    fn collect(shape: &egui::epaint::Shape, text: &mut String) {
        match shape {
            egui::epaint::Shape::Text(shape) => {
                text.push_str(shape.galley.text());
                text.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, text);
                }
            }
            _ => {}
        }
    }
    let mut text = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut text);
    }
    text
}
