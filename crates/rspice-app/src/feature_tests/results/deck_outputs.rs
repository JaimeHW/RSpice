//! What a manual deck's run plots: the outputs the deck names with `.probe`, `.save`, `.print`
//! or `.plot`, ticked in the data browser and drawn, and nothing else. A deck that names none
//! plots nothing: every signal waits unticked in the data browser, and the plot says how to
//! plot one. Either way the run's results show once it ends.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use egui::accesskit::{Role, Toggled, TreeUpdate};
use egui::{Context, Pos2, Rect, vec2};

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::frame_work::{
    DatasetWalk, WorkCounts,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::workbench::workspace::Workspace;

const DECK: &str = "* outputs\nV1 a 0 SIN(0 1 1k)\nR1 a b 1k\nR2 b 0 1k\n.tran 10u 2m\n";

struct Window {
    ctx: Context,
    app: RSpiceApp,
    frame: eframe::Frame,
    time: f64,
}

impl Window {
    fn frame(&mut self) -> egui::FullOutput {
        self.time += 0.1;
        let (app, frame) = (&mut self.app, &mut self.frame);
        self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1600.0, 900.0))),
                time: Some(self.time),
                focused: true,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, frame),
        )
    }
}

/// What Results shows once the deck with `card` has run to its end: each signal's tick in
/// the data browser, by the signal's name, and whether the plot says nothing is plotted.
fn results_after_running(card: &str) -> (Vec<(String, bool)>, bool) {
    let root = unique_temp_dir("deck-outputs");
    let deck = root.join("outputs.cir");
    std::fs::write(&deck, format!("{DECK}{card}.end\n")).expect("the deck is written");
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state.open_deck_at(&deck, true),
        "the deck opens: {:?}",
        app.state.prompt_text()
    );
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut window = Window {
        ctx,
        app,
        frame: eframe::Frame::_new_kittest(),
        time: 0.0,
    };
    window.app.state.workbench.activate(Workspace::Netlist);
    window.frame();
    window.app.execute_command_key(SimulateCommand::Run.key());
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        window.frame();
        let execution = &window.app.state.model.simulation.execution;
        if !execution.has_active_execution() && !execution.trigger_simulation {
            break;
        }
        assert!(Instant::now() < deadline, "the deck's run ends");
        std::thread::sleep(Duration::from_millis(5));
    }
    window.app.state.workbench.activate(Workspace::Results);
    let mut tree = None;
    for _ in 0..3 {
        tree = window.frame().platform_output.accesskit_update.or(tree);
    }
    let _ = std::fs::remove_dir_all(&root);
    read(&tree.expect("the frame publishes its tree"))
}

fn read(tree: &TreeUpdate) -> (Vec<(String, bool)>, bool) {
    let mut ticks = Vec::new();
    let mut nothing_plotted = false;
    for (_, node) in &tree.nodes {
        let label = node.label().unwrap_or_default();
        if node.role() == Role::CheckBox
            && let Some(name) = label.strip_prefix("Plot ")
        {
            ticks.push((name.to_owned(), node.toggled() == Some(Toggled::True)));
        }
        nothing_plotted |= label.starts_with("Nothing plotted")
            || node
                .value()
                .unwrap_or_default()
                .starts_with("Nothing plotted");
    }
    ticks.sort();
    (ticks, nothing_plotted)
}

/// A deck that names an output with `.probe` or `.save` plots that output and nothing else:
/// its tick is the only one in the data browser, and it is drawn. Its measurement keeps every
/// signal the run wrote, each offered unticked beside it.
#[test]
fn a_deck_plots_and_ticks_the_outputs_it_names_and_no_other() {
    for card in [".probe V(b)\n", ".save v(b)\n"] {
        let card = format!("{card}.meas tran peak max V(a)\n");
        let (ticks, nothing_plotted) = results_after_running(&card);
        let ticked = ticks
            .iter()
            .filter(|(_, ticked)| *ticked)
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(ticked, ["B"], "{card:?}: the data browser ticks {ticks:?}");
        assert!(
            ticks.iter().any(|(name, _)| name == "A"),
            "{card:?}: the other signals are offered: {ticks:?}"
        );
        assert!(!nothing_plotted, "{card:?}: the named output is drawn");
    }
}

/// A deck that names no output plots nothing: every signal it wrote is in the data browser,
/// none ticked, and the plot says how to plot one.
#[test]
fn a_deck_that_names_no_output_plots_nothing_and_offers_every_signal() {
    let (ticks, nothing_plotted) = results_after_running("");
    let offered = ticks
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    assert!(
        ["A", "B", "I(V1)"]
            .iter()
            .all(|name| offered.contains(name)),
        "every signal is in the data browser: {ticks:?}"
    );
    assert!(
        ticks.iter().all(|(_, ticked)| !ticked),
        "nothing is ticked: {ticks:?}"
    );
    assert!(nothing_plotted, "the plot says nothing is plotted");
}

/// A run's results show once it ends, in Results open while it runs: the output the deck names
/// is drawn, nothing says the results failed validation, and neither the frame that keeps them
/// nor the frames after it validate or digest them again, which the run did as it converted
/// them.
#[test]
fn a_run_shows_its_results_once_it_ends_without_the_frame_validating_or_digesting_them() {
    let root = unique_temp_dir("deck-run-end");
    let deck = root.join("outputs.cir");
    std::fs::write(&deck, format!("{DECK}.probe V(b)\n.end\n")).expect("the deck is written");
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state.open_deck_at(&deck, true),
        "the deck opens: {:?}",
        app.state.prompt_text()
    );
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut window = Window {
        ctx,
        app,
        frame: eframe::Frame::_new_kittest(),
        time: 0.0,
    };
    window.app.state.workbench.activate(Workspace::Netlist);
    window.frame();
    window.app.execute_command_key(SimulateCommand::Run.key());
    window.app.state.workbench.activate(Workspace::Results);
    let deadline = Instant::now() + Duration::from_secs(120);
    // The frame the run ends on keeps its results; what that frame validates is counted.
    let ending = loop {
        let counted = WorkCounts::reset();
        window.frame();
        let execution = &window.app.state.model.simulation.execution;
        if !execution.has_active_execution() && !execution.trigger_simulation {
            break counted.since();
        }
        assert!(Instant::now() < deadline, "the deck's run ends");
        std::thread::sleep(Duration::from_millis(5));
    };
    let after = WorkCounts::reset();
    let mut tree = None;
    for _ in 0..3 {
        tree = window.frame().platform_output.accesskit_update.or(tree);
    }
    let after = after.since();
    let _ = std::fs::remove_dir_all(&root);
    let tree = tree.expect("the frame publishes its tree");
    let (ticks, nothing_plotted) = read(&tree);
    assert!(
        ticks.contains(&("B".to_owned(), true)),
        "the named output is ticked: {ticks:?}"
    );
    assert!(!nothing_plotted, "the named output is drawn");
    let failed = tree.nodes.iter().any(|(_, node)| {
        [node.label(), node.value()]
            .into_iter()
            .flatten()
            .any(|text| text.contains("failed validation"))
    });
    assert!(!failed, "nothing says the results failed validation");
    assert_eq!(
        (
            ending.get(DatasetWalk::EvidenceValidation),
            after.get(DatasetWalk::EvidenceValidation)
        ),
        (0, 0),
        "the run's frames validate its kept results again: {:?} then {:?}",
        ending.nonzero(),
        after.nonzero()
    );
    assert_eq!(
        (
            ending.get(DatasetWalk::DatasetDigest),
            after.get(DatasetWalk::DatasetDigest)
        ),
        (0, 0),
        "the run's frames digest its kept results again: {:?} then {:?}",
        ending.nonzero(),
        after.nonzero()
    );
}
