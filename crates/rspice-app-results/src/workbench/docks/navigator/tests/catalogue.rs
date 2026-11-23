//! The browser over an analysis too large to walk on a frame: its signals
//! are catalogued a slice a frame and listed away from the frame, and a frame
//! makes rows only for what is on screen.

use std::cell::Cell;
use std::time::{Duration, Instant};

use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
use crate::workbench::documents::result_document::ResultsTestHost;

use super::super::results_catalogue::{CATALOGUE_SLICE, ROWS_MADE, SIGNALS_CATALOGUED};
use super::browser::{Browser, Node};

/// A finished run of `signals` nodes at the top of the design, none of them
/// on the plot.
fn wide_run(signals: usize) -> ResultsTestHost {
    let mut host = ResultsTestHost::default();
    host.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(
            (0..signals)
                .map(|node| {
                    RetainedTrace::new(
                        format!("V(n{node})"),
                        vec![0.0, 1.0],
                        vec![0.0, 1.0],
                        "#fff",
                    )
                })
                .collect(),
        ),
    );
    let run = host.model.simulation.active_run_mut().expect("a run");
    run.mark_running().expect("the run starts");
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .expect("the run finishes");
    for waveform in &mut run.analyses[0].waveforms {
        waveform.visible = false;
    }
    host
}

/// Whether a row of signal `name` is on screen.
fn shows(nodes: &[Node], name: &str) -> bool {
    let row = format!("{name},");
    nodes.iter().any(|node| node.label.starts_with(&row))
}

/// One frame of `browser`: what it drew, the signals it copied into a
/// catalogue and the signal rows it made.
fn frame(browser: &mut Browser) -> (Vec<Node>, usize, usize) {
    SIGNALS_CATALOGUED.with(|copied| copied.set(0));
    ROWS_MADE.with(|made| made.set(0));
    let nodes = browser.frame(Vec::new());
    (
        nodes,
        SIGNALS_CATALOGUED.with(Cell::get),
        ROWS_MADE.with(Cell::get),
    )
}

/// Draw frames until `name` shows, holding each frame to a slice of copying
/// and to the rows a screen holds; the listing is made off the frame
/// meanwhile.
fn frames_until_shown(browser: &mut Browser, name: &str, signals: usize) -> Vec<Node> {
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        let (nodes, copied, made) = frame(browser);
        assert!(
            copied <= CATALOGUE_SLICE,
            "a frame copied {copied} of {signals} signals into the catalogue"
        );
        assert!(made < 200, "a frame made {made} rows of {signals} signals");
        if shows(&nodes, name) {
            return nodes;
        }
        assert!(
            Instant::now() < deadline,
            "{name} was never listed: {:?}",
            nodes.iter().map(|node| &node.label).collect::<Vec<_>>()
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// A large analysis is listed without any frame walking it: a frame copies
/// at most a slice of it into its catalogue, the listing is made away from
/// the frame, and a frame makes a row only for each signal on screen. A
/// query over it is answered the same way, and once answered an idle frame
/// copies nothing and makes only the rows it draws.
#[test]
fn a_large_analysis_is_listed_without_a_frame_walking_it() {
    const SIGNALS: usize = 4 * CATALOGUE_SLICE;
    let mut browser = Browser::new(wide_run(SIGNALS));
    frames_until_shown(&mut browser, "V(n0)", SIGNALS);

    let (nodes, copied, made) = frame(&mut browser);
    assert!(shows(&nodes, "V(n0)"));
    assert_eq!(copied, 0, "an idle frame copied signals again");
    assert!(
        made > 0 && made < 200,
        "an idle frame made {made} rows of {SIGNALS} signals"
    );

    let last = format!("V(n{})", SIGNALS - 1);
    browser.host.workbench.set_navigator_filter(&last);
    let nodes = frames_until_shown(&mut browser, &last, SIGNALS);
    assert!(
        !shows(&nodes, "V(n0)"),
        "the query's listing replaced the whole one"
    );
}
