//! The dock's Plot page on Design, as the reader sees it: one analysis of
//! the shown run, the lead one until the reader probes or picks another, its
//! panes fitted to the dock with a line counting those it has no room for,
//! and nothing on it that scrolls.

use super::availability::ac_analysis;
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_flow_tests::User;
use rspice_app_design::schematic::view::{PlotProbedTrace, ProbePlot};
use rspice_app_state::product::ContentDigest;
use rspice_app_state::simulation::plan::AnalysisKind;
use rspice_app_state::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisType, RetainedTrace, SimulationRun,
    SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::docks::console::CONSOLE_BODY_PADDING_TOP;
use rspice_results::result_presentation::{AnalysisPresentationKey, WavePanePresentationKey};
use rspice_results::visualization_document::{Strip, StripAxis, StripMember};
use rspice_results_ui::selection::SelectedResultTrace;

/// The run's analyses in the order it wrote them: the AC sweep, then the
/// transient the plan leads with.
const AC: usize = 0;
const TRAN: usize = 1;

/// A signal of the transient: `amplitude` in `unit`, a sine over 200 µs.
fn wave(name: &str, unit: &str, amplitude: f64) -> RetainedTrace {
    let time: Vec<f64> = (0..=200).map(|index| f64::from(index) * 1.0e-6).collect();
    let values: Vec<f64> = time.iter().map(|t| amplitude * (t * 3.0e4).sin()).collect();
    RetainedTrace::new(name, time, values, "#4fa3ff").with_unit(unit)
}

/// Three signals of three units, a pane each.
fn three_units() -> Vec<RetainedTrace> {
    vec![
        wave("V(in)", "V", 1.0),
        wave("I(R1)", "A", 1.0e-3),
        wave("P(R1)", "W", 1.0e-3),
    ]
}

/// Design over a finished run of a plan that leads with a transient and
/// sweeps AC after it, the run having written the sweep first; the dock's
/// Plot page is in front, `height` tall, and the transient retains
/// `traces`.
fn plot_page(traces: Vec<RetainedTrace>, height: f32) -> User {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the project's plan");
    let tran = plan.instances()[0].id();
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("the sweep joins");
    let revision = plan.revision();
    let provenance = |authored| {
        AnalysisResultProvenance::new(
            authored,
            revision,
            ContentDigest::from_bytes([0x42; 32]),
            Vec::new(),
        )
        .expect("the analysis' provenance")
    };
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(ac_analysis().with_provenance(provenance(ac)));
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Transient, "TRAN")
            .with_provenance(provenance(tran))
            .with_waveforms(traces),
    );
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.model.simulation.select_run(0);
    app.state.workbench.activate(Workspace::Design);
    app.state.workbench.show_plot_page();
    app.state.workbench.plot_height = Some(height);
    let mut user = User::new(app);
    user.settle();
    user
}

fn shown(user: &User) -> Option<usize> {
    user.app.state.model.simulation.view.active_analysis_idx
}

/// Where the dock's Plot page stands: the largest region named for it, with
/// the inset at its top its toolbar's band covers.
fn page(user: &User) -> egui::Rect {
    let region = user
        .named
        .iter()
        .filter(|(name, _)| name == "Plot")
        .map(|(_, rect)| *rect)
        .max_by(|a, b| a.area().total_cmp(&b.area()))
        .expect("the Plot page is in front");
    egui::Rect::from_min_max(
        region.min - egui::vec2(0.0, CONSOLE_BODY_PADDING_TOP),
        region.max,
    )
}

/// What the Plot page names, with where.
fn on_page(user: &User) -> Vec<(String, egui::Rect)> {
    let page = page(user);
    user.named
        .iter()
        .filter(|(name, rect)| {
            // A control of the dock's edge, as the inspector's resize
            // grip, stands mostly beside the page.
            let overlap = page.intersect(*rect);
            name != "Plot"
                && overlap.width() > 0.0
                && overlap.height() > 0.0
                && overlap.area() * 2.0 >= rect.area()
        })
        .cloned()
        .collect()
}

/// Whether the page plots `signal`: its legend entry is on the page.
fn plots(user: &User, signal: &str) -> bool {
    let entry = format!("Toggle {signal} visibility");
    on_page(user).iter().any(|(name, _)| *name == entry)
}

/// The panes the page draws, top to bottom.
fn panes(user: &User) -> Vec<(String, egui::Rect)> {
    let mut panes: Vec<(String, egui::Rect)> = on_page(user)
        .into_iter()
        .filter(|(name, _)| name.ends_with(" pane") && !name.starts_with("Open in Results"))
        .collect();
    panes.sort_by(|a, b| a.1.top().total_cmp(&b.1.top()));
    panes
}

/// The overflow line's link, and how many panes it counts.
fn overflow(user: &User) -> Option<(String, usize)> {
    on_page(user).into_iter().find_map(|(name, _)| {
        let count = name
            .strip_prefix("Open in Results: +")?
            .split(' ')
            .next()?
            .parse()
            .ok()?;
        Some((name, count))
    })
}

/// What the last frame painted as text on the page, with where.
fn texts(user: &User) -> Vec<(String, egui::Rect)> {
    fn walk(shape: &egui::Shape, clip: egui::Rect, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::Shape::Text(text) => {
                let rect = text.visual_bounding_rect();
                if clip.intersects(rect) {
                    out.push((text.galley.text().to_owned(), rect));
                }
            }
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, clip, out);
                }
            }
            _ => {}
        }
    }
    let page = page(user);
    let mut out = Vec::new();
    for clipped in &user.shapes {
        walk(&clipped.shape, clipped.clip_rect.intersect(page), &mut out);
    }
    out
}

/// A tick label's value: a number, an SI prefix and a unit after it.
fn tick_value(label: &str) -> Option<f64> {
    let label = label.replace('\u{2212}', "-");
    let number = label.split_whitespace().next()?;
    let (digits, scale) = match number.chars().last()? {
        'k' => (&number[..number.len() - 1], 1.0e3),
        'm' => (&number[..number.len() - 1], 1.0e-3),
        'µ' => (&number[..number.len() - 'µ'.len_utf8()], 1.0e-6),
        'n' => (&number[..number.len() - 1], 1.0e-9),
        _ => (number, 1.0),
    };
    digits.parse::<f64>().ok().map(|value| value * scale)
}

/// The run shows its lead analysis on the page, the transient the toolbar's
/// run chip names, though the run wrote the sweep first; the sweep's
/// signals are not plotted beside it.
#[test]
fn the_page_shows_the_runs_lead_analysis() {
    let user = plot_page(vec![wave("V(in)", "V", 1.0)], 320.0);
    assert_eq!(shown(&user), Some(TRAN), "the lead analysis is shown");
    assert!(plots(&user, "V(in)"), "the transient is plotted");
    assert!(
        !plots(&user, "|V(out)|"),
        "one analysis is plotted: {:?}",
        on_page(&user)
    );
}

/// The band's select names both analyses as the data browser does, and
/// picking the sweep shows it in the transient's place.
#[test]
fn the_analysis_select_lists_both_and_switches() {
    let mut user = plot_page(vec![wave("V(in)", "V", 1.0)], 320.0);
    assert!(user.find("AC Analysis").is_none(), "the list is closed");
    user.click("Analysis");
    for analysis in ["TRAN", "AC Analysis"] {
        assert!(
            user.find(analysis).is_some(),
            "the select lists {analysis}: {:?}",
            user.named
        );
    }
    user.click("AC Analysis");
    assert_eq!(shown(&user), Some(AC), "the pick is shown");
    assert!(plots(&user, "|V(out)|"), "the sweep is plotted");
    assert!(!plots(&user, "V(in)"), "the transient gives way");
}

/// A probe of a signal the other analysis holds switches the page to that
/// analysis.
#[test]
fn probing_the_other_analysis_switches_the_page() {
    let mut user = plot_page(vec![wave("V(in)", "V", 1.0)], 320.0);
    assert_eq!(shown(&user), Some(TRAN));
    let run = user
        .app
        .state
        .model
        .simulation
        .retained
        .runs
        .get(0)
        .expect("the run");
    let trace = SelectedResultTrace::from_run_indices(run, AC, 0).expect("the sweep's magnitude");
    user.app.state.workbench.outbox.push(PlotProbedTrace {
        trace,
        plot: ProbePlot::Show,
        display: "V(out)".to_owned(),
    });
    user.settle();
    assert_eq!(shown(&user), Some(AC), "the probed analysis is shown");
    assert!(plots(&user, "|V(out)|"), "the probed signal is plotted");
    assert!(!plots(&user, "V(in)"), "the transient gives way");
}

/// The page draws as many panes as its dock has room for at the least a
/// pane reads at, every one when it is tall enough, and counts the rest on
/// a line whose link opens Results on the analysis; a dock with room for
/// two of three draws two and counts one.
#[test]
fn panes_without_room_are_counted_on_a_line_that_opens_results() {
    let mut user = plot_page(three_units(), 140.0);
    let mut seen: Vec<(f32, usize)> = Vec::new();
    for step in 0..=24 {
        let height = 140.0 + 15.0 * step as f32;
        user.app.state.workbench.plot_height = Some(height);
        user.settle();
        let (drawn, counted) = (panes(&user).len(), overflow(&user).map_or(0, |(_, n)| n));
        assert_eq!(
            drawn + counted,
            3,
            "every pane is drawn or counted at {height}"
        );
        if let Some((_, last)) = seen.last() {
            assert!(
                drawn >= *last,
                "a taller dock draws no fewer panes at {height}"
            );
        }
        seen.push((height, drawn));
    }
    assert!(
        seen.iter().any(|(_, drawn)| *drawn == 3),
        "a tall dock draws every pane: {seen:?}"
    );
    let (height, _) = seen
        .iter()
        .find(|(_, drawn)| *drawn == 2)
        .unwrap_or_else(|| panic!("no dock fits two panes: {seen:?}"));
    user.app.state.workbench.plot_height = Some(*height);
    user.settle();
    let (link, counted) = overflow(&user).expect("the line counting the third");
    assert_eq!(counted, 1);
    assert_eq!(link, "Open in Results: +1 pane");
    assert!(
        !plots(&user, "P(R1)"),
        "the stack's last pane is the one counted"
    );
    // The active pane is drawn first, then the stack's first.
    let run = user
        .app
        .state
        .model
        .simulation
        .active_run()
        .expect("the run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[TRAN]);
    user.app.state.results.results.session.active_wave_pane = Some(WavePanePresentationKey {
        analysis,
        unit: "W".to_owned(),
    });
    user.settle();
    assert!(
        plots(&user, "P(R1)") && plots(&user, "V(in)") && !plots(&user, "I(R1)"),
        "the active pane and the first are drawn: {:?}",
        panes(&user)
    );
    user.click(&link);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(shown(&user), Some(TRAN), "Results opens on the analysis");
}

/// Nothing on the page scrolls at any dock height: every name it draws is
/// inside it, and a wheel anywhere on it moves none of them.
#[test]
fn nothing_on_the_page_scrolls() {
    let wheel = |delta: f32| egui::Event::MouseWheel {
        unit: egui::MouseWheelUnit::Point,
        delta: egui::vec2(0.0, delta),
        phase: egui::TouchPhase::Move,
        modifiers: egui::Modifiers::NONE,
    };
    let placed = |user: &User| -> Vec<(String, f32)> {
        on_page(user)
            .into_iter()
            .filter(|(name, _)| {
                name.ends_with(" pane") || name.starts_with("Toggle ") || name == "Analysis"
            })
            .map(|(name, rect)| (name, rect.top()))
            .collect()
    };
    let mut user = plot_page(three_units(), 140.0);
    for height in [140.0, 200.0, 260.0, 340.0, 460.0, 620.0] {
        user.app.state.workbench.plot_height = Some(height);
        user.settle();
        let page = page(&user);
        for (name, rect) in on_page(&user) {
            assert!(
                page.expand(0.5).contains_rect(rect),
                "{name} at {rect:?} runs past the page {page:?} at {height}"
            );
        }
        let before = placed(&user);
        assert!(!before.is_empty(), "the page names its panes at {height}");
        for share in [0.3, 0.55, 0.8, 0.97] {
            let at = egui::pos2(page.center().x, page.top() + share * page.height());
            user.frame(vec![egui::Event::PointerMoved(at), wheel(-600.0)]);
            user.frame(vec![wheel(-600.0)]);
        }
        user.settle();
        assert_eq!(placed(&user), before, "the page scrolled at {height}");
    }
}

/// A pane holding a voltage and a current reads each against its own axis
/// on the page, the volts on the left and the amps on the right.
#[test]
fn a_pane_of_two_units_keeps_both_axes() {
    let mut user = plot_page(
        vec![wave("V(in)", "V", 1.0), wave("I(R1)", "A", 400.0)],
        320.0,
    );
    let simulation = &user.app.state.model.simulation;
    let run = simulation.active_run().expect("the run");
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[TRAN]);
    let member = |name: &str| StripMember::Signal {
        name: name.to_owned(),
        form: Default::default(),
    };
    user.app.state.results.results.session.strip_layouts.insert(
        key,
        vec![Strip {
            key: "strip-1".to_owned(),
            weight: None,
            locked: false,
            axes: vec![
                StripAxis {
                    unit: "V".to_owned(),
                    members: vec![member("V(in)")],
                },
                StripAxis {
                    unit: "A".to_owned(),
                    members: vec![member("I(R1)")],
                },
            ],
        }],
    );
    // The strips are drawn again for the arrangement.
    user.app.state.model.simulation.view.data_version += 1;
    user.settle();
    assert_eq!(panes(&user).len(), 1, "one pane: {:?}", panes(&user));
    let page = page(&user);
    let column = |left: bool, values: fn(f64) -> bool| -> usize {
        let mut rows: Vec<f32> = texts(&user)
            .into_iter()
            .filter(|(_, rect)| (rect.center().x < page.center().x) == left)
            .filter_map(|(text, rect)| tick_value(&text).map(|value| (value, rect)))
            .filter(|(value, _)| values(value.abs()))
            .map(|(_, rect)| rect.center().y)
            .collect();
        rows.sort_by(f32::total_cmp);
        rows.dedup_by(|a, b| (*a - *b).abs() < 4.0);
        rows.len()
    };
    assert!(
        column(true, |value| (0.1..=2.0).contains(&value)) >= 2,
        "the volts read on the left: {:?}",
        texts(&user)
    );
    assert!(
        column(false, |value| value >= 50.0) >= 2,
        "the amps read on the right: {:?}",
        texts(&user)
    );
}
