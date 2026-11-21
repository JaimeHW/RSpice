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

/// [`hierarchical_run`] once the run has ended, with none of its signals
/// on the plot: what a reader ticks from.
pub(super) fn finished_hierarchical_run() -> ResultsTestHost {
    let mut host = hierarchical_run();
    let run = host.model.simulation.active_run_mut().expect("a run");
    run.mark_running().expect("the run starts");
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .expect("the run finishes");
    for waveform in &mut run.analyses[0].waveforms {
        waveform.visible = false;
    }
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
    /// A tick box's state; `false` for anything that is not one.
    pub(super) ticked: bool,
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
                    ticked: node.toggled() == Some(egui::accesskit::Toggled::True),
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
    // And the row's own box says so: one answer for the count and the row.
    let row = nodes
        .iter()
        .find(|node| node.label.starts_with("Select") && node.label.contains("clk"))
        .unwrap_or_else(|| panic!("no tick box for clk: {:?}", labels(&nodes)));
    assert!(
        row.ticked,
        "{} reads unticked while its lane is plotted",
        row.label
    );
}

/// The engine folds every node name to upper case; an event net's row reads
/// as the drawing spells the net.
#[test]
fn an_event_net_row_reads_as_drawn() {
    let mut host = ResultsTestHost::default();
    host.model
        .schematic
        .add_net_label(crate::state::Point::new(0, 0), "Clk".to_owned());
    host.model
        .simulation
        .start_run()
        .add_analysis(test_fixtures::events_analysis());
    let mut browser = Browser::new(host);
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    assert!(
        find(&nodes, "Digital net Clk").is_some()
            || nodes
                .iter()
                .any(|node| node.label.starts_with("Select Digital net Clk")),
        "{:?}",
        labels(&nodes)
    );
    assert!(
        !nodes
            .iter()
            .any(|node| node.label.contains("Digital net clk")),
        "{:?}",
        labels(&nodes)
    );
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

/// The nodes the simulator generated for a device stay out of the tree until
/// the reader asks for them, as SimVision's and EZwave's browsers keep
/// theirs: the kind list closes with "Show internal nodes", and turning it on
/// lists them where they belong.
#[test]
fn internal_nodes_wait_behind_the_kind_lists_switch() {
    let mut host = ResultsTestHost::default();
    let trace = |name: &str| RetainedTrace::new(name, vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            trace("V(out)"),
            trace("V(xldo.fb)"),
            trace("V(xldo.mp.__gint)"),
            trace("I(xldo.mp.__rg)"),
            trace("V(xldo.mp.d)"),
        ]),
    );
    let run = host.model.simulation.active_run().expect("a run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let mut browser = Browser::new(host);
    for instance in ["xldo", "xldo.mp"] {
        super::super::results::set_result_browser_instance_expanded(
            &browser.ctx,
            analysis,
            instance,
            true,
        );
    }
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    assert!(
        nodes.iter().all(|node| !node.label.contains("__")),
        "{:?}",
        labels(&nodes)
    );
    // The instance counts what it lists; nothing reads as filtered away.
    assert!(
        find(&nodes, "xldo.mp, instance, 1 quantity, expanded").is_some(),
        "{:?}",
        labels(&nodes)
    );
    assert!(find(&nodes, "Signals, 3").is_some(), "{:?}", labels(&nodes));

    let kind = find(&nodes, "Quantity kind")
        .expect("the kind list")
        .rect
        .center();
    let nodes = browser.click(kind);
    let switch = find(&nodes, "Show internal nodes")
        .expect("the kind list closes with the switch")
        .rect
        .center();
    browser.click(switch);
    let nodes = browser.frame(Vec::new());
    assert!(
        find(&nodes, "xldo.mp, instance, 3 quantities, expanded").is_some(),
        "{:?}",
        labels(&nodes)
    );
    assert!(find(&nodes, "Signals, 5").is_some(), "{:?}", labels(&nodes));
    assert!(
        nodes
            .iter()
            .any(|node| node.label.starts_with("V(xldo.mp.__gint)")),
        "{:?}",
        labels(&nodes)
    );
    assert!(
        nodes
            .iter()
            .any(|node| node.label.starts_with("I(xldo.mp.__rg)")),
        "{:?}",
        labels(&nodes)
    );
}

/// The engine names instances and devices in upper case; the tree's
/// instance row and a current's row read as the drawing places them.
#[test]
fn an_instance_and_a_device_read_as_drawn() {
    use crate::state::{ComponentType, Point};
    let mut host = ResultsTestHost::default();
    for (kind, name, x) in [
        (ComponentType::CellInstance, "xTb", 0),
        (ComponentType::Capacitor, "Cout", 200),
    ] {
        let id = host.model.schematic.add_component(kind, Point::new(x, 0));
        host.model
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the placed part")
            .name = name.to_owned();
    }
    let trace = |name: &str| RetainedTrace::new(name, vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![trace("V(XTB.OK)"), trace("I(COUT)")]),
    );
    let mut browser = Browser::new(host);
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    assert!(
        find(&nodes, "xTb, instance, 1 quantity, collapsed").is_some(),
        "{:?}",
        labels(&nodes)
    );
    assert!(
        nodes.iter().any(|node| node.label.contains("I(Cout)")),
        "{:?}",
        labels(&nodes)
    );
    assert!(
        !nodes
            .iter()
            .any(|node| node.label.contains("XTB") || node.label.contains("COUT")),
        "{:?}",
        labels(&nodes)
    );
}

/// The engine names a measurement in upper case; its row reads as the plan
/// that authors it spells it.
#[test]
fn a_measurement_reads_as_its_plan_spells_it() {
    use rspice_results::specification::{SpecEntry, SpecPointScope, SpecificationDefinition};
    let mut host = ResultsTestHost::default();
    host.model
        .workspace
        .content
        .replace_active_specification_definitions(
            crate::product::SimulationPlanId::new(),
            vec![SpecificationDefinition::new_from_projection(&SpecEntry {
                measurement: "okt".to_owned(),
                expression: ".MEAS TRAN okt WHEN D(ok)=0.5 RISE=1".to_owned(),
                min: None,
                max: None,
                unit: String::new(),
                scope: SpecPointScope::AllPoints,
            })],
        );
    let trace = RetainedTrace::new("V(OUT)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
    let mut analysis =
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![trace]);
    analysis.measurements = vec![rspice_core::MeasureResult::success("OKT", 1.6e-3)];
    host.model.simulation.start_run().add_analysis(analysis);
    let mut browser = Browser::new(host);
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    assert!(
        nodes.iter().any(|node| node.label.contains(".MEAS okt")),
        "{:?}",
        labels(&nodes)
    );
    assert!(
        !nodes.iter().any(|node| node.label.contains("OKT")),
        "{:?}",
        labels(&nodes)
    );
}

/// The row model is built once and kept while nothing it lists changes, and
/// built again when the query changes: building it walks every retained
/// signal and copies its name, which an idle frame used to do.
#[test]
fn the_row_model_is_kept_until_what_it_lists_changes() {
    use crate::workbench::documents::result_document::frame_work::{DatasetWalk, WorkCounts};

    let mut browser = Browser::new(hierarchical_run());
    browser.frame(Vec::new());
    let idle = WorkCounts::reset();
    for _ in 0..5 {
        browser.frame(Vec::new());
    }
    assert_eq!(
        idle.since().get(DatasetWalk::BrowserRows),
        0,
        "an idle frame built the row model again"
    );
    browser.host.workbench.set_navigator_filter("x1.*");
    let filtered = WorkCounts::reset();
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    assert_eq!(filtered.since().get(DatasetWalk::BrowserRows), 1);
    assert!(
        find(&nodes, "V(out), Voltage").is_none() && find(&nodes, "V(x1.a), Voltage").is_some(),
        "the rows are the new query's: {:?}",
        labels(&nodes)
    );
}

/// A measurement of a run that kept no curves is not offered the table, and
/// its menu says why: Open in active result pane and Open as table did
/// nothing and said nothing.
#[test]
fn a_measurement_of_a_run_without_curves_is_not_offered_the_table() {
    use super::super::results_actions::viewer_refusal;
    use super::super::results_rows::retained_result_artifacts;
    use rspice_results::result_presentation::ResultViewer;

    let mut host = ResultsTestHost::default();
    let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    analysis.measurements = vec![rspice_core::MeasureResult::success("TDELAY", 1.6e-3)];
    host.model.simulation.start_run().add_analysis(analysis);
    let measurement = {
        let run = host.model.simulation.active_run().expect("a run");
        let analysis = &run.analyses[0];
        retained_result_artifacts(
            analysis,
            AnalysisPresentationKey::new(run.dataset_id, analysis),
        )
        .into_iter()
        .find(|artifact| artifact.name == ".MEAS TDELAY")
        .expect("the measurement has a row")
    };
    assert_eq!(measurement.viewer, ResultViewer::Table);
    let parts = host.results_parts();
    assert_eq!(
        viewer_refusal(&parts, &measurement, ResultViewer::Table),
        Some("Requires an analysis with retained samples")
    );
}

/// Ticking the first signal brings the band of batch actions in under the
/// rows: no row moves under the pointer, so a second click where the first
/// landed does not untick what it ticked.
#[test]
fn the_first_tick_moves_no_row() {
    let mut browser = Browser::new(finished_hierarchical_run());
    browser.frame(Vec::new());
    let before = browser.frame(Vec::new());
    let tick_of = |nodes: &[Node], name: &str| {
        nodes
            .iter()
            .find(|node| node.label == format!("Plot {name}"))
            .cloned()
            .unwrap_or_else(|| panic!("no tick box for {name}: {:?}", labels(nodes)))
    };
    let (input, output) = (tick_of(&before, "V(in)"), tick_of(&before, "V(out)"));
    assert!(!input.ticked && find(&before, "1 ticked").is_none());

    let after = browser.click(input.rect.center());
    let ticked = tick_of(&after, "V(in)");
    assert!(ticked.ticked, "the click ticks V(in)");
    assert_eq!(ticked.rect, input.rect, "V(in) stays under the pointer");
    assert_eq!(tick_of(&after, "V(out)").rect, output.rect);
    let band = find(&after, "1 ticked").expect("the band counts the tick");
    assert!(
        band.rect.top() >= ticked.rect.bottom(),
        "the band stands under the rows: {:?} against {:?}",
        band.rect,
        ticked.rect
    );
}

/// The selection bar clears the ticks itself, and its Actions menu saves
/// them as a set: Clear was a row of the menu, and a set was saved only
/// from the Sets menu up in the header.
#[test]
fn the_selection_bar_clears_the_ticks_and_its_menu_saves_them_as_a_set() {
    let mut browser = Browser::new(finished_hierarchical_run());
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    let at = |nodes: &[Node], label: &str| {
        find(nodes, label)
            .unwrap_or_else(|| panic!("no {label}: {:?}", labels(nodes)))
            .rect
            .center()
    };
    let nodes = browser.click(at(&nodes, "Plot V(in)"));
    let nodes = browser.click(at(&nodes, "Plot V(out)"));
    assert!(find(&nodes, "2 ticked").is_some(), "{:?}", labels(&nodes));

    let menu = browser.click(at(&nodes, "Actions"));
    assert!(
        find(&menu, "Save ticked as set\u{2026}").is_some(),
        "{:?}",
        labels(&menu)
    );
    let clear = find(&menu, "Clear ticks").expect("Clear stands on the bar");
    assert!(
        clear.rect.top() > at(&nodes, "Plot V(out)").y,
        "Clear stands on the bar under the rows, not in the menu: {:?}",
        clear.rect
    );

    let closed = browser.click(at(&menu, "Actions"));
    let cleared = browser.click(at(&closed, "Clear ticks"));
    assert!(
        find(&cleared, "2 ticked").is_none(),
        "{:?}",
        labels(&cleared)
    );
    assert!(
        cleared
            .iter()
            .filter(|node| node.label.starts_with("Plot "))
            .all(|node| !node.ticked),
        "every signal is unticked"
    );
}

/// The flat list in the order the view menu names: the order the run wrote
/// its signals in, or by unit and then by name.
#[test]
fn the_flat_list_sorts_in_run_order_and_by_unit() {
    let mut browser = Browser::new(finished_hierarchical_run());
    browser.frame(Vec::new());
    let nodes = browser.frame(Vec::new());
    let at = |nodes: &[Node], label: &str| {
        find(nodes, label)
            .unwrap_or_else(|| panic!("no {label}: {:?}", labels(nodes)))
            .rect
            .center()
    };
    let rows = |nodes: &[Node]| {
        let mut ticks: Vec<&Node> = nodes
            .iter()
            .filter(|node| node.label.starts_with("Plot "))
            .collect();
        ticks.sort_by(|above, below| above.rect.top().total_cmp(&below.rect.top()));
        ticks
            .iter()
            .map(|node| node.label["Plot ".len()..].to_owned())
            .collect::<Vec<_>>()
    };
    let nodes = browser.click(at(&nodes, "View options"));
    let nodes = browser.click(at(&nodes, "Flat"));
    let nodes = browser.click(at(&nodes, "View options"));
    let nodes = browser.click(at(&nodes, "Run order"));
    assert_eq!(
        rows(&nodes),
        [
            "V(out)",
            "V(x1.a)",
            "I(X1.V1)",
            "V(x1.x2.b)",
            "D(xsar.q)",
            "V(in)"
        ],
        "as the run wrote them"
    );

    let nodes = browser.click(at(&nodes, "View options"));
    let nodes = browser.click(at(&nodes, "Unit"));
    let by_unit = rows(&nodes);
    let volts: Vec<&str> = by_unit
        .iter()
        .map(String::as_str)
        .filter(|name| name.starts_with("V("))
        .collect();
    assert_eq!(volts, ["V(in)", "V(out)", "V(x1.a)", "V(x1.x2.b)"]);
    let amps = by_unit.iter().position(|name| name == "I(X1.V1)").unwrap();
    let first_volt = by_unit.iter().position(|name| name == "V(in)").unwrap();
    assert!(amps < first_volt, "A before V: {by_unit:?}");
}
