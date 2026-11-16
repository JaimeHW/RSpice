//! Corners: a run-set sweep and a corner family both fill it, the head names
//! every point without a legend, Transpose and Failing only rearrange the
//! same cells, a selection is the same cell however the rows are sorted, and
//! every colour is the sheet's.

use egui::{Color32, Event, Modifiers, PointerButton, Pos2};

use super::super::raster::{
    PageGeometry, VerifyStage, page_input, page_pass, refresh_verification, seed_corner,
    seed_run_set,
};
use super::super::table::{Selection, Sort, SortDirection};
use super::*;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::test_host::SimulateTestHost;

const PROCESSES: [&str; 5] = ["TT", "SS", "FF", "SF", "FS"];
const SUPPLIES: [f64; 3] = [1.62, 1.8, 1.98];
const TEMPERATURES: [f64; 3] = [-40.0, 27.0, 125.0];

/// A run set's sweep of `processes` × `supplies` × `temperatures`, with no
/// corner analysis, open on Corners.
fn run_set_app(
    processes: &[&str],
    supplies: &[f64],
    temperatures: &[f64],
) -> FeatureHarness<SimulateFeature> {
    let mut harness = SimulateTestHost::harness();
    seed_run_set(&mut harness.model, processes, supplies, temperatures);
    harness.open(VerificationPage::Corners);
    harness
}

/// The seven-corner family run, open on Corners.
fn corner_app() -> FeatureHarness<SimulateFeature> {
    let mut harness = SimulateTestHost::harness();
    seed_corner(&mut harness.model);
    harness.open(VerificationPage::Corners);
    harness
}

fn sheet(harness: &FeatureHarness<SimulateFeature>) -> &Sheet {
    harness
        .model
        .simulation
        .verification_sheet()
        .expect("the fixture judged its run")
}

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// One frame of the open page over `events`.
fn frame(
    ctx: &egui::Context,
    harness: &mut FeatureHarness<SimulateFeature>,
    events: Vec<Event>,
) -> (egui::FullOutput, PageGeometry) {
    page_pass(ctx, page_input(events), |ui| harness.draw(ui))
}

/// The page laid out until it settles.
fn settled(
    ctx: &egui::Context,
    harness: &mut FeatureHarness<SimulateFeature>,
) -> (egui::FullOutput, PageGeometry) {
    let _ = frame(ctx, harness, Vec::new());
    let _ = frame(ctx, harness, Vec::new());
    frame(ctx, harness, Vec::new())
}

/// Every run of words the frame painted, with its colour.
fn painted(output: &egui::FullOutput) -> Vec<(String, Color32)> {
    fn walk(shape: &egui::Shape, words: &mut Vec<(String, Color32)>) {
        match shape {
            egui::Shape::Text(text) => {
                words.push((text.galley.text().to_owned(), text.fallback_color));
            }
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, words);
                }
            }
            _ => {}
        }
    }
    let mut words = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut words);
    }
    words
}

fn click(pos: Pos2) -> Vec<Event> {
    let edge = |pressed| Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    vec![Event::PointerMoved(pos), edge(true), edge(false)]
}

/// The toolbar's line, as it was painted.
fn summary_words(output: &egui::FullOutput) -> Option<String> {
    painted(output)
        .into_iter()
        .map(|(words, _)| words)
        .find(|words| words.contains(" points \u{00b7} "))
}

#[test]
fn a_run_set_sweep_with_no_corner_analysis_fills_corners() {
    let mut harness = run_set_app(&["TT", "SS", "FF"], &SUPPLIES, &[27.0, 125.0]);
    assert!(
        harness.model.simulation.active_run().is_some_and(|run| run
            .analyses
            .iter()
            .all(|analysis| analysis.analysis_type != crate::state::AnalysisType::Corner)),
        "the run holds no corner analysis"
    );
    let sheet = sheet(&harness);
    assert_eq!(sheet.points.len(), 18);
    assert_eq!(
        sheet.axes,
        [
            PointAxis::Process,
            PointAxis::Supply,
            PointAxis::Temperature
        ]
    );
    assert_eq!(
        corners_summary(sheet),
        Summary::failing("18 points \u{00b7} 1 fails at SS 1.62 V 125 \u{00b0}C")
    );

    let ctx = context();
    let (output, geometry) = settled(&ctx, &mut harness);
    assert_eq!(
        geometry.all("table.row").len(),
        2,
        "gain and supply current"
    );
    assert_eq!(geometry.all("table.band").len(), 2, "process and supply");
    assert!(geometry.one("empty").is_none());
    assert!(geometry.one("toolbar.summary").is_some());
    assert_eq!(
        summary_words(&output).as_deref(),
        Some("18 points \u{00b7} 1 fails at SS 1.62 V 125 \u{00b0}C")
    );
    let words = painted(&output);
    assert!(
        words
            .iter()
            .any(|(words, _)| words == "Nominal 27 \u{00b0}C"),
        "the nominal point's head carries the mark"
    );
}

#[test]
fn a_corner_family_fills_corners_too() {
    let mut harness = corner_app();
    let sheet = sheet(&harness);
    assert_eq!(sheet.points.len(), 7);
    assert_eq!(sheet.rows.len(), 4);
    assert!(
        sheet.points.iter().all(|point| !point.nominal),
        "a corner family names no nominal point"
    );
    let heads = point_heads(sheet);
    assert_eq!(heads.bands.len(), 2);
    assert_eq!(
        heads.bands[0]
            .iter()
            .map(|span| (span.label.as_str(), span.first, span.last))
            .collect::<Vec<_>>(),
        [
            ("TT", 4, 4),
            ("SS", 5, 6),
            ("FF", 7, 8),
            ("SF", 9, 9),
            ("FS", 10, 10)
        ]
    );
    assert_eq!(
        heads.heads,
        [
            "27 \u{00b0}C",
            "125 \u{00b0}C",
            "\u{2212}40 \u{00b0}C",
            "125 \u{00b0}C",
            "\u{2212}40 \u{00b0}C",
            "27 \u{00b0}C",
            "27 \u{00b0}C"
        ]
    );
    assert_eq!(
        corners_summary(sheet),
        Summary::failing("7 points \u{00b7} 1 fails at SS 1.62 V 125 \u{00b0}C")
    );
    let ctx = context();
    let (_, geometry) = settled(&ctx, &mut harness);
    assert_eq!(geometry.all("table.row").len(), 4);
}

/// Forty-five points read without a legend: the process band spans its nine
/// columns, the supply band runs of equal neighbours that never cross a
/// process, and the head row the temperature.
#[test]
fn five_by_three_by_three_heads_merge_equal_neighbours() {
    let harness = run_set_app(&PROCESSES, &SUPPLIES, &TEMPERATURES);
    let sheet = sheet(&harness);
    assert_eq!(sheet.points.len(), 45);
    let heads = point_heads(sheet);
    assert_eq!(heads.bands.len(), 2);
    let processes = &heads.bands[0];
    assert_eq!(
        processes
            .iter()
            .map(|span| (span.label.as_str(), span.last - span.first + 1))
            .collect::<Vec<_>>(),
        PROCESSES.map(|process| (process, 9)),
        "one span of nine per process"
    );
    assert_eq!(processes[0].first, LEADING);
    let supplies = &heads.bands[1];
    assert_eq!(
        supplies.len(),
        16,
        "three supplies per process, and the nominal supply split off"
    );
    for span in supplies {
        assert!(
            processes
                .iter()
                .any(|process| process.first <= span.first && span.last <= process.last),
            "{span:?} crosses a process"
        );
    }
    assert!(supplies.iter().all(|span| span.label.ends_with(" V")));
    assert_eq!(heads.heads.len(), 45);
    assert_eq!(heads.heads[0], "Nominal 27 \u{00b0}C");
    assert!(heads.heads[1..].iter().all(|head| {
        ["\u{2212}40 \u{00b0}C", "27 \u{00b0}C", "125 \u{00b0}C"].contains(&head.as_str())
    }));
}

/// A run that varies one coordinate has no band: the head row says it all.
#[test]
fn one_varying_axis_has_no_band() {
    let mut harness = run_set_app(&["TT"], &[1.8], &TEMPERATURES);
    let sheet = sheet(&harness);
    assert_eq!(sheet.axes, [PointAxis::Temperature]);
    let heads = point_heads(sheet);
    assert!(heads.bands.is_empty());
    assert_eq!(
        heads.heads,
        [
            "Nominal 27 \u{00b0}C",
            "\u{2212}40 \u{00b0}C",
            "125 \u{00b0}C"
        ]
    );
    let ctx = context();
    let (_, geometry) = settled(&ctx, &mut harness);
    assert!(geometry.one("table.band").is_none());
    let toolbar = geometry.one("toolbar").expect("toolbar");
    let head = geometry.one("table.head").expect("head");
    assert_eq!(head.top(), toolbar.bottom());
}

#[test]
fn transpose_and_failing_only_rearrange_the_same_cells() {
    let mut harness = run_set_app(&["TT", "SS", "FF"], &SUPPLIES, &[27.0, 125.0]);
    let sheet = sheet(&harness);
    assert_eq!(shown_specifications(sheet, false), [0, 1]);
    assert_eq!(
        shown_specifications(sheet, true),
        [0],
        "supply current is near its ceiling, not over it"
    );
    assert_eq!(shown_points(sheet, false).len(), 18);
    let failing = shown_points(sheet, true);
    assert_eq!(failing.len(), 1);
    assert_eq!(sheet.points[failing[0]].label, "SS 1.62 V 125 \u{00b0}C");
    assert_eq!(point_status(sheet, failing[0]), Some(CellStatus::Fail));
    let nominal = sheet.points.iter().position(|point| point.nominal);
    assert_eq!(nominal, Some(0));

    let ctx = context();
    let rows = |harness: &mut FeatureHarness<SimulateFeature>, transposed, failing_only| {
        harness.state.verification.corners_transposed = transposed;
        harness.state.verification.corners_failing_only = failing_only;
        let (_, geometry) = settled(&ctx, harness);
        (
            geometry.all("table.row").len(),
            geometry.all("table.band").len(),
        )
    };
    assert_eq!(rows(&mut harness, false, false), (2, 2));
    assert_eq!(rows(&mut harness, false, true), (1, 2));
    assert_eq!(
        rows(&mut harness, true, false),
        (18, 0),
        "a point per row, and no band over the specifications"
    );
    assert_eq!(rows(&mut harness, true, true), (1, 0));
}

/// The toolbar's Transpose turns the table and keeps the selected cell: the
/// same specification at the same point.
#[test]
fn transposing_keeps_the_selected_cell() {
    let mut harness = run_set_app(&["TT", "SS"], &[1.62, 1.8], &[27.0, 125.0]);
    let point = sheet(&harness)
        .points
        .iter()
        .position(|point| point.label == "SS 1.62 V 125 \u{00b0}C")
        .expect("the slow corner");
    harness.state.verification.corners_table.selection = Some(Selection {
        row: 0,
        key: Some(RowKey::Number(0)),
        column: Some(LEADING + point),
    });
    assert_eq!(
        selected_point(sheet(&harness), &harness.state.verification),
        Some(SelectedPoint {
            point,
            row: Some(0)
        })
    );
    let ctx = context();
    let (_, geometry) = settled(&ctx, &mut harness);
    // The toolbar lays its actions out right to left; Transpose is the first.
    let transpose = geometry
        .all("toolbar.action")
        .into_iter()
        .min_by(|left, right| left.left().total_cmp(&right.left()))
        .expect("Transpose stands on the row");
    let _ = frame(&ctx, &mut harness, click(transpose.center()));
    assert!(harness.state.verification.corners_transposed);
    let selection = harness
        .state
        .verification
        .corners_table
        .selection
        .clone()
        .expect("the selection survives");
    assert_eq!(selection.key, Some(RowKey::Number(point as u64)));
    assert_eq!(
        selected_point(sheet(&harness), &harness.state.verification),
        Some(SelectedPoint {
            point,
            row: Some(0)
        })
    );
}

/// Sorting a transposed column moves the rows, and the selection follows its
/// point rather than its place.
#[test]
fn sorting_a_transposed_column_keeps_the_selected_point() {
    let mut harness = run_set_app(&["TT", "SS", "FF"], &SUPPLIES, &[27.0, 125.0]);
    harness.state.verification.corners_transposed = true;
    let sheet_now = sheet(&harness);
    let slow = sheet_now
        .points
        .iter()
        .position(|point| point.label == "SS 1.62 V 125 \u{00b0}C")
        .expect("the slow corner");
    assert_ne!(slow, 0);
    let gain = transposed_leading(sheet_now);
    harness.state.verification.corners_table.selection = Some(Selection {
        row: slow,
        key: Some(RowKey::Number(slow as u64)),
        column: Some(gain),
    });
    harness.state.verification.corners_table.sort = Some(Sort {
        column: gain,
        direction: SortDirection::Ascending,
    });
    let ctx = context();
    let (_, geometry) = settled(&ctx, &mut harness);
    let selection = harness
        .state
        .verification
        .corners_table
        .selection
        .clone()
        .expect("the selection survives the sort");
    assert_eq!(selection.key, Some(RowKey::Number(slow as u64)));
    assert_eq!(
        geometry.one("table.selected").map(|row| row.top()),
        geometry.all("table.row").first().map(|row| row.top()),
        "the lowest gain sorts first, and the selection moved with it"
    );
    assert_eq!(
        selected_point(sheet(&harness), &harness.state.verification),
        Some(SelectedPoint {
            point: slow,
            row: Some(0)
        })
    );
}

/// The view inks a cell by the sheet's status for it and nothing else: with
/// a guard band on gain, values above the bare 40 dB floor fail, which only
/// the sheet can know.
#[test]
fn every_colour_is_the_sheets_status() {
    let mut harness = corner_app();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let content = &mut harness.model.workspace.content;
    let mut definitions = content
        .ensure_active_plan_data(plan_id)
        .specification_definitions
        .clone();
    let gain = definitions
        .iter_mut()
        .find(|definition| definition.measurement == "gain")
        .expect("the plan limits gain");
    gain.guard_band = Some(2.5);
    content.replace_active_specification_definitions(plan_id, definitions);
    refresh_verification(&mut harness.model);
    let row = &sheet(&harness).rows[0];
    assert_eq!(row.guard_band, 2.5);
    assert_eq!(
        row.cells.iter().map(|cell| cell.status).collect::<Vec<_>>(),
        [
            CellStatus::Fail,
            CellStatus::Fail,
            CellStatus::Pass,
            CellStatus::Pass,
            CellStatus::Pass,
            CellStatus::Fail,
            CellStatus::Pass
        ],
        "42.3 and 42.4 dB clear the floor but not its guard band"
    );

    let ctx = context();
    let (output, _) = settled(&ctx, &mut harness);
    let t = crate::ui::tokens::Tokens::get(&ctx);
    let words = painted(&output);
    // Min and Max restate a value neutrally; its own point's cell carries
    // the status.
    let inks = |shown: &str| {
        words
            .iter()
            .filter(|(words, _)| words == shown)
            .map(|(_, colour)| *colour)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        inks("42.30 dB"),
        [t.color.err],
        "fails only by its guard band"
    );
    assert_eq!(inks("39.40 dB"), [t.color.text, t.color.err]);
    assert_eq!(
        inks("44.20 dB"),
        [t.color.text, t.color.text],
        "a pass is neutral"
    );
    assert_eq!(
        inks("47.00 deg"),
        [t.color.text, t.color.warn],
        "near its floor"
    );
}

/// A point outside a specification's scope reads "n/a" in the faint
/// register; a pass reads its value.
#[test]
fn a_point_outside_the_scope_reads_not_applicable() {
    let mut harness = run_set_app(&["TT", "SS"], &[1.8], &[27.0, 125.0]);
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let mut idd = super::super::raster::limit("idd", None, Some(1.2e-3), "A");
    idd.scope = crate::state::SpecPointScope::Nominal;
    harness.model.workspace.content.replace_active_specs(
        plan_id,
        vec![
            super::super::raster::limit("gain", Some(40.0), None, "dB"),
            idd,
        ],
    );
    refresh_verification(&mut harness.model);
    let row = &sheet(&harness).rows[1];
    assert_eq!(row.cells[0].status, CellStatus::Pass);
    assert!(
        row.cells[1..]
            .iter()
            .all(|cell| cell.status == CellStatus::Excluded)
    );
    let ctx = context();
    let (output, _) = settled(&ctx, &mut harness);
    let t = crate::ui::tokens::Tokens::get(&ctx);
    let words = painted(&output);
    assert_eq!(
        words
            .iter()
            .filter(|(words, colour)| words == "n/a" && *colour == t.color.text_faint)
            .count(),
        3
    );
}

/// Double-clicking a cell opens the analysis that gave its value: a run-set
/// point's own analysis, or a corner family's on that member.
#[test]
fn a_cell_opens_its_analysis_and_point_in_results() {
    let mut harness = run_set_app(&["TT", "SS"], &[1.8], &[27.0, 125.0]);
    let sheet_now = sheet(&harness);
    let point = 2;
    let evidence = evidence_at(
        harness.model.simulation.active_run().expect("run"),
        sheet_now,
        0,
        point,
    )
    .expect("an analysis gave the value");
    assert_eq!(evidence.sample, None);
    let analysis = &harness.model.simulation.active_run().expect("run").analyses[evidence.analysis];
    assert_eq!(
        analysis
            .provenance()
            .and_then(|provenance| provenance.pvt_point())
            .map(crate::state::AnalysisResultPvtPoint::process),
        sheet_now.points[point].process.as_deref()
    );
    open_in_results(&mut harness.cx(), evidence);
    assert_eq!(
        harness.model.simulation.view.active_analysis_idx,
        Some(evidence.analysis)
    );
    assert_eq!(
        harness.wb.workspace,
        crate::workbench::state::Workspace::Results
    );
    assert!(
        harness
            .wb
            .outbox
            .take::<crate::workbench::framework::requests::SetFamilyFilter>()
            .is_none(),
        "a run-set point is a whole analysis"
    );

    let mut harness = corner_app();
    let evidence = evidence_at(
        harness.model.simulation.active_run().expect("run"),
        sheet(&harness),
        0,
        3,
    )
    .expect("the family gave the value");
    assert_eq!(evidence.sample, Some(3));
    open_in_results(&mut harness.cx(), evidence);
    assert_eq!(
        harness
            .wb
            .outbox
            .take::<crate::workbench::framework::requests::SetFamilyFilter>()
            .map(|filter| filter.0),
        Some("sample = 4".to_owned())
    );
}

/// The inspector states the selected cell's point and what it gave.
#[test]
fn the_selected_cell_states_its_point_value_margin_and_analysis() {
    let mut harness = run_set_app(&["TT", "SS"], &[1.62, 1.8], &[27.0, 125.0]);
    let point = sheet(&harness)
        .points
        .iter()
        .position(|point| point.label == "SS 1.62 V 125 \u{00b0}C")
        .expect("the slow corner");
    harness.state.verification.corners_table.selection = Some(Selection {
        row: 0,
        key: Some(RowKey::Number(0)),
        column: Some(LEADING + point),
    });
    let facts = point_facts(&harness.model, &harness.state.verification);
    assert_eq!(facts.len(), 2);
    assert_eq!(facts[0].title, "SS 1.62 V 125 \u{00b0}C");
    assert_eq!(
        facts[0].rows,
        [
            ("Process".to_owned(), "SS".to_owned()),
            ("Supply".to_owned(), "1.62 V".to_owned()),
            ("Temperature".to_owned(), "125 \u{00b0}C".to_owned()),
            ("Nominal".to_owned(), "No".to_owned()),
        ]
    );
    assert_eq!(facts[1].title, "gain");
    let value = |label: &str| {
        facts[1]
            .rows
            .iter()
            .find(|(name, _)| name == label)
            .map(|(_, value)| value.as_str())
    };
    assert_eq!(value("Value"), Some("39.40 dB"));
    let margin = sheet(&harness).rows[0].cells[point]
        .signed_margin
        .expect("a judged value has a margin");
    assert!(margin < 0.0);
    assert_eq!(value("Margin"), Some(margin_text(margin, "dB").as_str()));
    assert_eq!(value("Limit"), Some("\u{2265} 40 dB"));
    assert_eq!(value("Status"), Some("Fail"));
    assert_eq!(value("Analysis"), Some("Transient"));

    harness.state.verification.corners_table.selection = None;
    assert!(point_facts(&harness.model, &harness.state.verification).is_empty());
}

/// The file holds the sheet as the page shows it: values at full precision,
/// every coordinate of a point in its head.
#[test]
fn the_exported_file_is_the_sheet_as_shown() {
    let harness = run_set_app(&["TT", "SS"], &[1.62, 1.8], &[27.0, 125.0]);
    let sheet = sheet(&harness);
    let plain = corners_csv(sheet, false, false);
    let lines = plain.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].starts_with(
        "Specification,Unit,Limit,Min,Max,TT \u{00b7} 1.8 V \u{00b7} 27 \u{00b0}C (nominal),"
    ));
    assert!(lines[1].starts_with("gain,dB,\u{2265} 40 dB,39.4,"));
    let failing = corners_csv(sheet, false, true);
    assert_eq!(failing.lines().count(), 2, "only gain fails");
    let turned = corners_csv(sheet, true, true);
    let lines = turned.lines().collect::<Vec<_>>();
    assert_eq!(
        lines[0],
        "Point,Process,Supply,Temperature,Status,gain (dB),idd (A)"
    );
    assert_eq!(lines.len(), 2);
    assert!(lines[1].starts_with(
        "SS \u{00b7} 1.62 V \u{00b7} 125 \u{00b0}C,SS,1.62 V,125 \u{00b0}C,Fail,39.4,"
    ));
}

/// Fewer than two points is one line under the toolbar with the action that
/// adds corners; no run at all says so too.
#[test]
fn one_point_is_one_line_and_the_run_set() {
    let ctx = context();
    let mut harness = run_set_app(&["TT"], &[1.8], &[27.0]);
    assert_eq!(sheet(&harness).points.len(), 1);
    let (output, geometry) = settled(&ctx, &mut harness);
    let toolbar = geometry.one("toolbar").expect("toolbar");
    let empty = geometry.one("empty").expect("one line");
    assert_eq!(empty.top(), toolbar.bottom());
    assert!(geometry.one("table.head").is_none());
    assert!(geometry.one("toolbar.summary").is_none());
    assert!(geometry.all("toolbar.action").is_empty());
    let words = painted(&output);
    assert!(words.iter().any(|(words, _)| words
        == "This run solved one point. Add process, supply or temperature axes to the run set to \
            verify across corners."));
    assert!(words.iter().any(|(words, _)| words == "Open run set"));

    let mut empty = SimulateTestHost::harness();
    empty.open(VerificationPage::Corners);
    let (_, geometry) = settled(&ctx, &mut empty);
    assert!(geometry.one("empty").is_some());
    assert!(geometry.one("table.head").is_none());
}
