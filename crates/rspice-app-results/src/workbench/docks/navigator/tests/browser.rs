//! The whole browser in whole frames: the tree folding and opening, a glob
//! narrowing it, and the height its rows keep at every interface scale.

use rspice_results::result_presentation::ResultArtifactPresentationKey;
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultsTestHost, set_result_quantity_ticked, test_fixtures,
};

use super::super::results::results;

/// A transient run of a small hierarchical design: two top-level nets, an
/// instance `x1` holding a voltage, a current and a nested instance, and a
/// digital net inside `xsar`.
pub(super) fn hierarchical_run() -> ResultsTestHost {
    let mut host = ResultsTestHost::default();
    let trace = |name: &str| RetainedTrace::new(name, vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            trace("V(out)"),
            trace("V(x1.a)"),
            trace("I(X1.V1)"),
            trace("V(x1.x2.b)"),
            trace("D(xsar.q)"),
            trace("V(in)"),
        ]),
    );
    host
}

/// The browser in a 300 px dock, frame by frame.
pub(super) struct Browser {
    pub(super) host: ResultsTestHost,
    pub(super) ctx: egui::Context,
}

/// One accessible widget the frame drew: what it is spoken as, and where.
#[derive(Debug, Clone)]
pub(super) struct Node {
    pub(super) label: String,
    pub(super) rect: egui::Rect,
}

impl Browser {
    pub(super) fn new(host: ResultsTestHost) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        Self { host, ctx }
    }

    pub(super) fn frame(&mut self, events: Vec<egui::Event>) -> Vec<Node> {
        let Self { host, ctx } = self;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(300.0, 900.0),
            )),
            events,
            ..Default::default()
        };
        let mut run_details = None;
        let output = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                results(ui, &mut host.results_parts(), &mut run_details);
            });
        });
        // AccessKit bounds are in points, whatever the interface scale.
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
            .iter()
            .filter_map(|(_, node)| {
                let label = node.label().or(node.value())?.to_owned();
                let bounds = node.bounds()?;
                Some(Node {
                    label,
                    rect: egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                })
            })
            .collect()
    }

    /// Hover, press and release on `at`, then draw the frame that shows
    /// what the click did.
    pub(super) fn click(&mut self, at: egui::Pos2) -> Vec<Node> {
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![egui::Event::PointerMoved(at)]);
        self.frame(vec![button(true)]);
        self.frame(vec![button(false)]);
        self.frame(Vec::new())
    }
}

pub(super) fn find<'a>(nodes: &'a [Node], label: &str) -> Option<&'a Node> {
    nodes.iter().find(|node| node.label == label)
}

fn labels(nodes: &[Node]) -> Vec<&str> {
    nodes.iter().map(|node| node.label.as_str()).collect()
}

#[test]
fn an_instance_folds_and_opens_what_it_owns() {
    let mut browser = Browser::new(hierarchical_run());
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    // The top of the design and its instances, folded.
    assert!(
        find(&nodes, "V(out), Voltage").is_some(),
        "{:?}",
        labels(&nodes)
    );
    assert!(find(&nodes, "D(xsar.q), Digital net").is_none());
    let x1 = find(&nodes, "x1, instance, 3 quantities, collapsed")
        .unwrap_or_else(|| panic!("x1 folds its three quantities: {:?}", labels(&nodes)))
        .rect;
    assert!(find(&nodes, "xsar, instance, 1 quantity, collapsed").is_some());
    assert!(
        find(&nodes, "V(x1.a), Voltage").is_none(),
        "x1 starts folded"
    );

    let nodes = browser.click(x1.center());
    assert!(
        find(&nodes, "x1, instance, 3 quantities, expanded").is_some(),
        "{:?}",
        labels(&nodes)
    );
    let a = find(&nodes, "V(x1.a), Voltage").expect("an open x1 lists its voltage");
    let current = find(&nodes, "I(X1.V1), Current").expect("and its current");
    let nested = find(&nodes, "x1.x2, instance, 1 quantity, collapsed")
        .expect("and its own instance, folded");
    assert!(nested.rect.top() < a.rect.top(), "instances list first");
    assert!(
        a.rect.top() < current.rect.top(),
        "then quantities, voltages ahead of currents"
    );
    assert!(find(&nodes, "V(x1.x2.b), Voltage").is_none());

    let nodes = browser.click(x1.center());
    assert!(find(&nodes, "x1, instance, 3 quantities, collapsed").is_some());
    assert!(find(&nodes, "V(x1.a), Voltage").is_none(), "x1 folds again");
}

#[test]
fn a_glob_opens_the_branches_that_match_and_counts_them() {
    let mut host = hierarchical_run();
    host.workbench.set_navigator_filter("x1.*");
    let mut browser = Browser::new(host);
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    for shown in [
        "x1, instance, 3 quantities, expanded",
        "x1.x2, instance, 1 quantity, expanded",
        "V(x1.a), Voltage",
        "I(X1.V1), Current",
        "V(x1.x2.b), Voltage",
        "3 / 6",
    ] {
        assert!(
            find(&nodes, shown).is_some(),
            "{shown}: {:?}",
            labels(&nodes)
        );
    }
    for hidden in [
        "V(out), Voltage",
        "V(in), Voltage",
        "D(xsar.q), Digital net",
    ] {
        assert!(
            find(&nodes, hidden).is_none(),
            "{hidden}: {:?}",
            labels(&nodes)
        );
    }

    // A character class over the leaf, case-blind against the instance.
    browser.host.workbench.set_navigator_filter("*X[0-9].v?");
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    assert!(
        find(&nodes, "I(X1.V1), Current").is_some(),
        "{:?}",
        labels(&nodes)
    );
    assert!(find(&nodes, "V(x1.a), Voltage").is_none());
    assert!(find(&nodes, "1 / 6").is_some());
}

#[test]
fn rows_are_dense_and_hold_their_text_at_every_interface_scale() {
    for scale in [1.0, 1.1, 1.25, 1.5, 1.75, 2.0] {
        let mut browser = Browser::new(hierarchical_run());
        browser.ctx.set_zoom_factor(scale);
        browser.frame(Vec::new());
        let nodes = browser.frame(Vec::new());
        assert!((browser.ctx.pixels_per_point() - scale).abs() < 1e-6);
        let row = find(&nodes, "V(out), Voltage")
            .expect("a top-level row")
            .rect;
        assert!(
            (row.height() - 22.0).abs() < 0.5,
            "a compact row is 22 px at scale {scale}, not {}",
            row.height()
        );
        // Rows abut: the stride is the row, with no gap between them.
        let neighbour = find(&nodes, "V(in), Voltage").expect("its neighbour").rect;
        let stride = (neighbour.top() - row.top()).abs();
        assert!(
            (stride - 22.0).abs() < 0.5,
            "rows are 22 px apart at scale {scale}, not {stride}"
        );
        let head = find(&nodes, "x1, instance, 3 quantities, collapsed")
            .expect("an instance row")
            .rect;
        assert!(
            (head.height() - 22.0).abs() < 0.5,
            "instance rows share the stride"
        );
        // The name's line fits the row with room to spare at this scale.
        let line = browser.ctx.fonts_mut(|fonts| {
            fonts.row_height(&crate::ui::theme::mono(
                crate::ui::tokens::FS_0,
                crate::ui::theme::FontWeight::Regular,
            ))
        });
        assert!(
            line + 4.0 <= row.height(),
            "an {line} px line clips in a {} px row at scale {scale}",
            row.height()
        );
    }
}

#[test]
fn a_net_on_the_lanes_is_a_ticked_row() {
    let mut host = ResultsTestHost::default();
    host.model
        .simulation
        .start_run()
        .add_analysis(test_fixtures::events_analysis());
    let run = host.model.simulation.active_run().expect("a run");
    let clk = ResultBrowserSelectionKey::Artifact(ResultArtifactPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        "payload/transient-events/net/D(clk)",
    ));
    set_result_quantity_ticked(&mut host.results_parts(), &clk, true);
    let mut browser = Browser::new(host);
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    // The band counts it, so a set saves it and a drag carries it.
    assert!(find(&nodes, "1 ticked").is_some(), "{:?}", labels(&nodes));
}

/// A run that records no source revision to compare with the design, as a
/// netlist the reader ran or a file they imported does, heads its dataset
/// row with its name and count alone. "Unresolved" stated a relationship the
/// reader could neither see nor do anything about.
#[test]
fn a_dataset_without_a_source_revision_heads_its_row_with_its_name_alone() {
    let mut host = hierarchical_run();
    let run = host.model.simulation.active_run_mut().expect("a run");
    run.mark_running().expect("the run starts");
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .expect("the run finishes");
    let run_label = run.label.clone();
    let mut browser = Browser::new(host);
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    assert!(
        find(&nodes, &format!("{run_label}, 6 quantities, expanded")).is_some(),
        "{:?}",
        labels(&nodes)
    );
    assert!(
        nodes.iter().all(|node| !node.label.contains("unresolved")),
        "{:?}",
        labels(&nodes)
    );
}
