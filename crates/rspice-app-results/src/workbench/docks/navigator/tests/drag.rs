//! Dragging browser rows onto the strip plots them.

use rspice_results::result_presentation::SourceWaveformPresentationKey;
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use crate::workbench::documents::result_document::browser_plot::{
    last_strip_models, plotted_signal_color,
};
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultQuantityDrag, ResultsTestHost, set_result_quantity_ticked, waves,
};

use super::super::results::results;
use super::browser::{Node, find, hierarchical_run};

const NAMES: [&str; 6] = [
    "V(out)",
    "V(x1.a)",
    "I(X1.V1)",
    "V(x1.x2.b)",
    "D(xsar.q)",
    "V(in)",
];

fn key(host: &ResultsTestHost, name: &str) -> ResultBrowserSelectionKey {
    let run = host.model.simulation.active_run().expect("a run");
    ResultBrowserSelectionKey::Waveform(SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        name,
    ))
}

fn plotted(host: &ResultsTestHost) -> Vec<&'static str> {
    let ticked = crate::workbench::commands::result_navigation::ticked_result_quantities(
        &host.model,
        &host.results.results,
    );
    NAMES
        .into_iter()
        .filter(|name| ticked.contains(&key(host, name)))
        .collect()
}

/// The browser docked at the left and the strips beside it, as the Results
/// workspace lays them out.
struct Workspace {
    host: ResultsTestHost,
    ctx: egui::Context,
}

impl Workspace {
    fn new(host: ResultsTestHost) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        Self { host, ctx }
    }

    fn frame(&mut self, events: Vec<egui::Event>) -> Vec<Node> {
        let Self { host, ctx } = self;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 800.0),
            )),
            events,
            ..Default::default()
        };
        let mut run_details = None;
        let output = ctx.run_ui(input, |ctx| {
            egui::Panel::left("browser")
                .exact_size(300.0)
                .show(ctx, |ui| {
                    results(ui, &mut host.results_parts(), &mut run_details);
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                waves::show(ui, &mut host.results_parts());
            });
        });
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
            .iter()
            .filter_map(|(_, node)| {
                let bounds = node.bounds()?;
                Some(Node {
                    label: node.label().or(node.value())?.to_owned(),
                    rect: egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                })
            })
            .collect()
    }

    /// Press on `from`, drag past the click distance, carry the rows to
    /// `to`, and report what was being carried there before letting go.
    fn drag(&mut self, from: egui::Pos2, to: egui::Pos2) -> Option<ResultQuantityDrag> {
        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![egui::Event::PointerMoved(from)]);
        self.frame(vec![button(from, true)]);
        self.frame(vec![egui::Event::PointerMoved(
            from + egui::vec2(0.0, 12.0),
        )]);
        self.frame(vec![egui::Event::PointerMoved(to)]);
        let carried =
            egui::DragAndDrop::payload::<ResultQuantityDrag>(&self.ctx).map(|drag| (*drag).clone());
        self.frame(vec![button(to, false)]);
        self.frame(Vec::new());
        carried
    }
}

fn workspace_with(plotted: &[&str]) -> Workspace {
    let mut host = hierarchical_run();
    for name in NAMES {
        let key = key(&host, name);
        set_result_quantity_ticked(&mut host.results_parts(), &key, plotted.contains(&name));
    }
    let mut workspace = Workspace::new(host);
    workspace.frame(Vec::new());
    workspace
}

const STRIP: egui::Pos2 = egui::pos2(650.0, 420.0);

#[test]
fn a_row_dropped_on_the_strip_goes_on_the_plot() {
    let mut workspace = workspace_with(&["V(out)"]);
    let nodes = workspace.frame(Vec::new());
    let row = find(&nodes, "V(in), Voltage")
        .expect("an unplotted row")
        .rect;
    let carried = workspace
        .drag(row.center(), STRIP)
        .expect("the row is carried");
    assert_eq!(carried.keys, [key(&workspace.host, "V(in)")]);
    assert_eq!(plotted(&workspace.host), ["V(out)", "V(in)"]);
    assert!(
        egui::DragAndDrop::payload::<ResultQuantityDrag>(&workspace.ctx).is_none(),
        "the drop takes the rows"
    );
}

#[test]
fn a_row_let_go_over_the_browser_plots_nothing() {
    let mut workspace = workspace_with(&["V(out)"]);
    let nodes = workspace.frame(Vec::new());
    let row = find(&nodes, "V(in), Voltage")
        .expect("an unplotted row")
        .rect;
    workspace.drag(row.center(), egui::pos2(150.0, 700.0));
    assert_eq!(plotted(&workspace.host), ["V(out)"]);
}

#[test]
fn an_empty_plot_takes_a_drop_and_draws_the_strip() {
    let mut workspace = workspace_with(&[]);
    let nodes = workspace.frame(Vec::new());
    let row = find(&nodes, "V(out), Voltage").expect("a row").rect;
    workspace.drag(row.center(), STRIP);
    assert_eq!(plotted(&workspace.host), ["V(out)"]);
}

#[test]
fn an_instance_dragged_onto_the_strip_plots_everything_under_it() {
    let mut workspace = workspace_with(&["V(out)"]);
    let nodes = workspace.frame(Vec::new());
    let x1 = find(&nodes, "x1, instance, 3 quantities, collapsed")
        .expect("the x1 instance")
        .rect;
    let carried = workspace
        .drag(x1.center(), STRIP)
        .expect("the instance is carried");
    assert_eq!(carried.keys.len(), 3);
    assert_eq!(
        plotted(&workspace.host),
        ["V(out)", "V(x1.a)", "I(X1.V1)", "V(x1.x2.b)"]
    );
}

#[test]
fn a_ticked_row_carries_the_whole_ticked_selection() {
    let mut workspace = workspace_with(&["V(out)", "V(in)"]);
    let nodes = workspace.frame(Vec::new());
    let row = find(&nodes, "V(in), Voltage").expect("a ticked row").rect;
    let carried = workspace
        .drag(row.center(), STRIP)
        .expect("the selection is carried");
    assert_eq!(carried.keys.len(), 2);
    for name in ["V(out)", "V(in)"] {
        assert!(carried.keys.contains(&key(&workspace.host, name)), "{name}");
    }
    assert_eq!(plotted(&workspace.host), ["V(out)", "V(in)"]);
}

#[test]
fn a_plotted_row_reads_in_the_colour_the_strip_drew_its_curve_in() {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    // The dataset leaves its first column off the plot, so V(b) is the first
    // curve drawn and takes the first hue, where its column would give it
    // the second.
    let trace = |name: &str, visible: bool| {
        let mut trace = RetainedTrace::new(name, vec![0.0, 1.0], vec![0.0, 1.0], "");
        trace.visible = visible;
        trace
    };
    let mut host = ResultsTestHost::default();
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            trace("V(a)", false),
            trace("V(b)", true),
            trace("V(c)", true),
        ]),
    );
    let mut workspace = Workspace::new(host);
    workspace.frame(Vec::new());
    let palette = crate::ui::tokens::Tokens::get(&workspace.ctx).color.traces;
    let run = workspace.host.model.simulation.active_run().expect("a run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let parts = workspace.host.results_parts();
    let models = last_strip_models(&parts);
    let drawn = models
        .and_then(|models| models.iter().find(|model| model.analysis_key == analysis))
        .and_then(|model| model.traces.iter().find(|trace| trace.waveform_index == 1))
        .expect("the strip drew V(b)")
        .color;
    assert_eq!(
        plotted_signal_color(models, &palette, analysis, 1, ""),
        drawn
    );
    assert_ne!(
        plotted_signal_color(None, &palette, analysis, 1, ""),
        drawn,
        "its column's hue is not its curve's"
    );
}
