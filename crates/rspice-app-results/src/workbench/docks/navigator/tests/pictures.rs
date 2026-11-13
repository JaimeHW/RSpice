//! Review pictures of the data browser beside the strip, on the C12 LDO
//! testbench: the instance tree, the kind list, a glob narrowing the tree,
//! the signal-set menu, the Inspector on a selected trace, and Add
//! measurement taking a trace from the strip. Written only when
//! `RSPICE_PICTURES` names a folder.

use std::path::Path;

use rspice_app_runs::simulation::controller::test_execution::run_manual_batch;
use rspice_results::calculator::catalogue;
use rspice_results_ui::document_dialogs::ResultDocumentDialog;
use rspice_results_ui::document_dialogs::measurement::MeasurementDraft;
use rspice_results_ui::selection::{ResultBrowserSelectionKey, SelectedResultTrace};
use rspice_results_ui::session::TracePick;

use crate::state::SimulationRun;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultsTestHost, waves,
};

use super::super::results::{plotted_event_rows, results, set_result_browser_instance_expanded};
use super::super::results_sets::save_signal_set;

/// The picture, in points; it is drawn at twice that, as a 2x display does.
const SIZE: egui::Vec2 = egui::vec2(1280.0, 820.0);

/// `path` with its includes read in place and its HDL sources named by
/// absolute path, since a manual deck has no folder to resolve either
/// against. A deck reads a backslash as an escape, so paths go in with
/// forward slashes.
fn inlined(path: &Path) -> String {
    let folder = path.parent().expect("a deck sits in a folder");
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()))
        .lines()
        .map(|line| {
            if let Some(file) = line.strip_prefix(".include ") {
                inlined(&folder.join(file.trim()))
            } else if let Some(file) = line.strip_prefix(".hdl \"") {
                let file = folder.join(file.trim_end_matches('"'));
                format!(".hdl \"{}\"", file.display().to_string().replace('\\', "/"))
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The C12 testbench over its first two trim codes, with the testbench's
/// verdict, its real reading and the trim shift register saved as well.
fn c12_run() -> SimulationRun {
    let deck = inlined(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/mixed-corpus/c12-ldo-tb/deck/full.tran.cir"),
    )
    .replace(
        ".esave xtb.ok__event_0",
        ".esave xtb.ok__event_0 xtb.pass xtb.mv meas xtrim.sh",
    )
    .replace(".tran 0.5u 1.65m", ".tran 0.5u 210u")
    .lines()
    .filter(|line| !line.starts_with(".measure"))
    .collect::<Vec<_>>()
    .join("\n");
    run_manual_batch(&deck)
}

/// The browser docked at the left and the strip beside it.
struct Review {
    host: ResultsTestHost,
    ctx: egui::Context,
    /// Whether the Inspector is docked at the right.
    inspector: bool,
}

impl Review {
    fn new(run: SimulationRun) -> Self {
        let mut host = ResultsTestHost::default();
        host.model.simulation.retained.runs.push(run);
        host.model.simulation.view.active_run_idx = Some(0);
        host.workbench.workspace = crate::workbench::state::Workspace::Results;
        let ctx = egui::Context::default();
        ctx.set_os(egui::os::OperatingSystem::Nix);
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        Self {
            host,
            ctx,
            inspector: false,
        }
    }

    fn analysis(&self) -> AnalysisPresentationKey {
        let run = self.host.model.simulation.active_run().expect("a run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    }

    fn frame(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        let Self {
            host,
            ctx,
            inspector,
        } = self;
        let mut input = rspice_ui_kit::raster::display_input(SIZE, 2.0);
        input.events = events;
        let mut run_details = None;
        ctx.run_ui(input, |ctx| {
            egui::Panel::left("browser")
                .exact_size(300.0)
                .show(ctx, |ui| {
                    results(ui, &mut host.results_parts(), &mut run_details);
                });
            if *inspector {
                egui::Panel::right("inspector")
                    .exact_size(300.0)
                    .show(ctx, |ui| {
                        crate::workbench::docks::inspector::results::results(
                            ui,
                            &mut host.results_parts(),
                            &mut run_details,
                        );
                    });
            }
            egui::CentralPanel::default().show(ctx, |ui| {
                waves::show(ui, &mut host.results_parts());
            });
            crate::workbench::documents::result_document::show_document_dialog(
                ctx,
                &mut host.results_parts(),
            );
        })
    }

    /// Where the frame drew the widget spoken as `label`.
    fn find(&mut self, label: &str) -> egui::Rect {
        let output = self.frame(Vec::new());
        let update = output.platform_output.accesskit_update.expect("AccessKit");
        update
            .nodes
            .iter()
            .find_map(|(_, node)| {
                let bounds = node.bounds()?;
                (node.label().map(str::trim) == Some(label)).then(|| {
                    egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    )
                })
            })
            .unwrap_or_else(|| {
                let labels = update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| node.label().map(str::to_owned))
                    .filter(|label| label.contains("Sets") || label.contains('·'))
                    .collect::<Vec<_>>();
                panic!("no widget spoken as {label}: {labels:?}")
            })
    }

    fn click(&mut self, at: egui::Pos2) {
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![egui::Event::PointerMoved(at)]);
        self.frame(vec![button(true)]);
        self.frame(vec![button(false)]);
    }

    fn write(&mut self, directory: &str, name: &str, events: Vec<egui::Event>) {
        self.frame(events);
        // Long enough for a menu's fade to finish.
        for _ in 0..12 {
            self.frame(Vec::new());
        }
        let output = self.frame(Vec::new());
        let canvas = rspice_ui_kit::raster::rasterize(&self.ctx, SIZE, &output);
        std::fs::write(
            format!("{directory}/{name}.png"),
            canvas.png(canvas.height()),
        )
        .expect("the picture is written");
    }
}

#[test]
#[ignore = "writes review pictures; runs the C12 testbench"]
fn write_the_browser_pictures() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let mut run = c12_run();
    // The regulation loop on the plot: the output, the reference and the
    // feedback node inside the regulator.
    let mut feedback = String::new();
    let mut output = String::new();
    for analysis in &mut run.analyses {
        for waveform in &mut analysis.waveforms {
            let name = waveform.name.as_str();
            let bare = name
                .strip_prefix("V(")
                .and_then(|inner| inner.strip_suffix(')'))
                .unwrap_or(name);
            if bare == "xldo.FB" {
                feedback = waveform.name.clone();
            }
            if bare == "VOUT" {
                output = waveform.name.clone();
            }
            waveform.visible = matches!(bare, "VOUT" | "xldo.VREF" | "xldo.FB");
        }
    }
    let mut review = Review::new(run);
    // The trim word and the testbench's verdict in the lanes, ticked the way
    // a reader ticks them.
    for (query, row) in [
        ("xtrim.sh*", "Select Digital bus xtrim.sh[3:0]"),
        ("xtb.pass", "Select Digital net xtb.pass"),
    ] {
        review.host.workbench.set_navigator_filter(query);
        let at = review.find(row).center();
        review.click(at);
    }
    review.host.workbench.set_navigator_filter("");
    let analysis = review.analysis();
    for instance in ["xldo", "xldo.mp", "xtb"] {
        set_result_browser_instance_expanded(&review.ctx, analysis, instance, true);
    }
    review.write(&directory, "browser-tree", vec![egui::Event::PointerGone]);

    // The kind list open, with the switch that shows the simulator's
    // internal nodes, off.
    let kinds = review.find("Quantity kind").center();
    review.click(kinds);
    review.write(&directory, "browser-kinds", vec![egui::Event::PointerGone]);
    review.click(kinds);

    // The feedback node selected: the Inspector's swatch beside the strip
    // that drew its curve.
    review.host.results.results.session.selected_trace = Some(SelectedResultTrace::from_identity(
        analysis,
        feedback.clone(),
    ));
    review.inspector = true;
    review.write(
        &directory,
        "inspector-swatch",
        vec![egui::Event::PointerGone],
    );
    review.inspector = false;
    review.host.results.results.session.selected_trace = None;

    // Add measurement's Pick on its signal: the form steps aside for the
    // strip, and the trace clicked there comes back filled in.
    *review.host.results_parts().result_document_dialog =
        Some(ResultDocumentDialog::Measurement(MeasurementDraft {
            expression: format!("rms({feedback})"),
            function: catalogue::find("rms"),
            arguments: vec![feedback.clone()],
            picking: Some(0),
            ..MeasurementDraft::default()
        }));
    review.write(
        &directory,
        "measurement-pick",
        vec![egui::Event::PointerGone],
    );
    review.host.results.results.session.trace_pick = TracePick::Picked {
        source: output,
        name: "VOUT".to_owned(),
    };
    review.write(
        &directory,
        "measurement-picked",
        vec![egui::Event::PointerGone],
    );
    *review.host.results_parts().result_document_dialog = None;

    review.host.workbench.set_navigator_filter("*.m[1-4]*");
    review.write(&directory, "browser-glob", vec![egui::Event::PointerGone]);
    review.host.workbench.set_navigator_filter("");

    let mut ticked = crate::workbench::commands::result_navigation::ticked_result_quantities(
        &review.host.model,
        &review.host.results.results,
    );
    ticked.extend(plotted_event_rows(&review.host.results_parts()));
    let (analog, events): (Vec<_>, Vec<_>) = ticked
        .into_iter()
        .partition(|key| matches!(key, ResultBrowserSelectionKey::Waveform(_)));
    assert_eq!(analog.len(), 3, "the regulation loop");
    assert!(
        events.len() >= 2,
        "the trim word and the verdict: {events:?}"
    );
    save_signal_set(&mut review.host.results_parts(), "Regulation loop", &analog)
        .expect("the loop is saved");
    save_signal_set(&mut review.host.results_parts(), "Trim word", &events)
        .expect("the trim word is saved");
    let sets = review.find("Sets 2").center();
    review.click(sets);
    let set = review.find("Regulation loop · 3").center();
    review.write(
        &directory,
        "browser-sets",
        vec![egui::Event::PointerMoved(set)],
    );
}
