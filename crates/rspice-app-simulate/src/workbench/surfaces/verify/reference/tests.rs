//! The Reference view over a run and its reference: choosing the reference,
//! entering a tolerance, the comparison built only when it changed, a rule
//! with no target, and what the page says with nothing to compare.

use super::*;
use crate::product::AnalysisInstanceId;
use crate::state::{AnalysisResult, AnalysisType, RegressionComparisonMethod};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::test_host::SimulateTestHost;

/// Run 4 read against its reference, Run 3.
fn compared() -> FeatureHarness<SimulateFeature> {
    let mut harness = SimulateTestHost::harness();
    super::super::raster::seed_reference(&mut harness.model);
    harness
}

/// One frame of the Reference view at the review size.
fn draw(harness: &mut FeatureHarness<SimulateFeature>) {
    use super::super::raster::{page_input, page_pass};
    harness
        .wb
        .activate(crate::workbench::state::Workspace::Verify);
    harness.state.verification_page = VerificationPage::Reference;
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = page_pass(&ctx, page_input(Vec::new()), |ui| {
        super::super::show(ui, &mut harness.cx());
    });
}

fn comparison(harness: &FeatureHarness<SimulateFeature>) -> &ReferenceComparison {
    &harness.state.verification.reference
}

/// The row named `name`.
fn row<'h>(harness: &'h FeatureHarness<SimulateFeature>, name: &str) -> &'h ReferenceRow {
    comparison(harness)
        .rows
        .iter()
        .find(|row| row.name == name)
        .unwrap_or_else(|| panic!("a row named {name}"))
}

fn plan_revision(harness: &FeatureHarness<SimulateFeature>) -> ObjectRevision {
    harness
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .expect("the plan")
        .revision()
}

/// Every check the two runs share is a row: the measurements, then the
/// waveform. Bandwidth moved outside its 2 % and overshoot has no rule; the
/// toolbar counts what changed and names the reference.
#[test]
fn the_reference_view_compares_every_shared_check() {
    let mut harness = compared();
    draw(&mut harness);
    let names = comparison(&harness)
        .rows
        .iter()
        .map(|row| row.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["bw", "gain", "overshoot", "tsettle", "V(out)"]);
    let words = comparison(&harness)
        .rows
        .iter()
        .map(result_word)
        .collect::<Vec<_>>();
    assert_eq!(words, ["Fail", "Pass", "No tolerance", "Pass", "Pass"]);

    let cells = row_cells(row(&harness, "bw"));
    let texts = cells
        .iter()
        .map(|cell| cell.text.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        texts,
        [
            "bw".to_owned(),
            "18.20 MHz".to_owned(),
            "17.10 MHz".to_owned(),
            "-1.10 MHz".to_owned(),
            "-6.04 %".to_owned(),
            "rel 2 %".to_owned(),
            "Fail".to_owned(),
        ]
    );
    // Each value carries the unit its producer states, and one it states none
    // for is bare.
    let text = |name: &str, column: usize| row_cells(row(&harness, name))[column].text.clone();
    assert_eq!(text("tsettle", 1), "1.620 \u{00b5}s");
    assert_eq!(text("overshoot", 1), "12.60");
    assert!(
        text("V(out)", 3).starts_with("max ") && text("V(out)", 3).ends_with('V'),
        "a waveform's change is in its trace's unit: {}",
        text("V(out)", 3)
    );
    let compared = comparison(&harness).compared.as_ref().expect("a pair");
    assert_eq!((compared.changed, compared.outside), (5, 1));
    assert_eq!(compared.reference_run, 3);
}

/// Choosing a reference makes it the plan's, as a reference commit does: the
/// select offers exactly the runs that commit accepts, newest first.
#[test]
fn choosing_a_reference_commits_it() {
    let mut harness = compared();
    let plan_id = harness
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .expect("the plan")
        .id();
    let reference = harness.model.simulation.retained.runs[0].run_id;
    harness
        .model
        .workspace
        .content
        .ensure_active_plan_data(plan_id)
        .regression_baseline_run = None;
    draw(&mut harness);
    assert_eq!(comparison(&harness).compared, None);
    assert_eq!(
        empty_text(&harness.model, comparison(&harness)),
        "Choose a reference run to compare this run against."
    );
    let offered = comparison(&harness)
        .references
        .as_ref()
        .expect("the runs were looked for")
        .runs
        .iter()
        .map(|(index, _)| *index)
        .collect::<Vec<_>>();
    assert_eq!(offered, [0], "only Run 3: Run 4 is the run read");

    let before = plan_revision(&harness);
    choose_reference(&mut harness.cx(), 0).expect("the offered run commits");
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .and_then(|payload| payload.regression_baseline_run),
        Some(reference)
    );
    assert!(
        harness
            .model
            .simulation
            .retained
            .run_by_stable_id(reference)
            .expect("the reference stays retained")
            .retention()
            .is_pinned()
    );
    assert_eq!(plan_revision(&harness).get(), before.get() + 1);
    assert!(comparison(&harness).compared.is_some());
}

/// The select offers exactly the runs a reference commit accepts. A complete
/// run of another plan is sound data but no reference for this one, so it is
/// not offered; nor is a run sharing nothing with the run read, nor the run
/// read itself.
#[test]
fn the_reference_select_offers_only_the_runs_a_commit_accepts() {
    use super::super::raster::sealed_run;

    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let source_id = AnalysisInstanceId::new();
    let measured = |label: &str| {
        AnalysisResult::new(1, AnalysisType::Ac, label)
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)])
    };
    let current = sealed_run(4, plan_id, source_id, 2, measured("current"));
    let reference = sealed_run(3, plan_id, source_id, 2, measured("reference"));
    let other_plan = sealed_run(
        2,
        crate::product::SimulationPlanId::new(),
        source_id,
        2,
        measured("another plan"),
    );
    let unshared = sealed_run(
        1,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "nothing shared"),
    );
    let runs = [&current, &reference, &other_plan, &unshared].map(|run| run.run_id);
    harness.model.simulation.retained.runs = vec![current, reference, other_plan, unshared].into();
    assert!(harness.model.simulation.select_run(0));

    let offered = reference_runs(&harness.model);
    assert_eq!(offered, [1]);
    // A refused commit changes nothing, so every run the select leaves out is
    // tried first, then the one it offers.
    for (index, run_id) in runs.iter().enumerate() {
        if !offered.contains(&index) {
            assert!(
                commit_regression_baseline(&mut harness.cx(), *run_id).is_err(),
                "a run the select leaves out commits: {run_id}"
            );
        }
    }
    choose_reference(&mut harness.cx(), offered[0]).expect("the offered run commits");
}

/// A tolerance entered in its cell is the plan's at once, in one plan commit:
/// the row's result follows it before the next frame, and the export quotes
/// the tolerance as entered.
#[test]
fn entering_a_tolerance_judges_its_row_by_it_at_once() {
    let mut harness = compared();
    draw(&mut harness);
    assert_eq!(result_word(row(&harness, "bw")), "Fail");
    let target = row(&harness, "bw").target.clone().expect("a target");
    let compared = comparison(&harness).compared.as_ref().expect("a pair");
    let (junit, _) =
        regression_ci_documents(&compared.receipt, &compared.cases).expect("the documents");
    assert!(
        junit.contains("tolerance=rel 2 %"),
        "the failure quotes its rule"
    );
    let before = plan_revision(&harness);
    let builds = comparison(&harness).builds;

    set_tolerance(&mut harness.cx(), &target, "bw", "rel 10 %").expect("the tolerance is entered");

    assert_eq!(result_word(row(&harness, "bw")), "Pass");
    assert_eq!(plan_revision(&harness).get(), before.get() + 1);
    assert_eq!(comparison(&harness).builds, builds + 1);
    let compared = comparison(&harness).compared.as_ref().expect("a pair");
    assert_eq!(compared.outside, 0);
    let case = compared
        .cases
        .iter()
        .find(|case| case.name.ends_with("::bw[0]"))
        .expect("bw is a case");
    assert!(
        case.detail.ends_with("; tolerance=rel 10 %"),
        "{}",
        case.detail
    );
    let (_, tap) =
        regression_ci_documents(&compared.receipt, &compared.cases).expect("the documents");
    assert!(tap.contains("tolerance=rel 10 %"));

    // Entering what the cell already says changes nothing.
    set_tolerance(&mut harness.cx(), &target, "bw", "rel 10 %").expect("nothing to enter");
    assert_eq!(plan_revision(&harness).get(), before.get() + 1);
    // An empty cell leaves the target without a rule.
    set_tolerance(&mut harness.cx(), &target, "bw", "").expect("the rule is removed");
    assert_eq!(result_word(row(&harness, "bw")), "No tolerance");
    // What is not a tolerance is refused, and the plan keeps what it held.
    assert!(set_tolerance(&mut harness.cx(), &target, "bw", "wide").is_err());
    assert_eq!(plan_revision(&harness).get(), before.get() + 2);
}

/// The comparison is built once for a pair and its tolerances: a frame that
/// changed none of them builds nothing.
#[test]
fn an_unchanged_frame_builds_no_comparison() {
    let mut harness = compared();
    draw(&mut harness);
    assert_eq!(comparison(&harness).builds, 1);
    draw(&mut harness);
    draw(&mut harness);
    assert_eq!(comparison(&harness).builds, 1);
    assert!(harness.model.simulation.select_run(0));
    draw(&mut harness);
    assert_eq!(comparison(&harness).builds, 2, "another run read is built");
}

/// A rule whose target neither run holds is a row with a problem mark; it is
/// deleted in one plan commit.
#[test]
fn a_rule_with_no_target_shows_and_deletes() {
    let mut harness = compared();
    let plan_id = harness
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .expect("the plan")
        .id();
    let rules = &mut harness
        .model
        .workspace
        .content
        .ensure_active_plan_data(plan_id)
        .regression_tolerances;
    let mut orphan = rules[0].clone();
    orphan.target.name = "phase".to_owned();
    rules.push(orphan.clone());
    draw(&mut harness);
    let shown = comparison(&harness)
        .rows
        .iter()
        .find(|row| row.orphan())
        .expect("the rule is a row");
    assert_eq!(shown.target.as_ref(), Some(&orphan.target));
    assert_eq!(result_word(shown), "No target");
    assert!(!shown.editable());

    let before = plan_revision(&harness);
    delete_rule(&mut harness.cx(), &orphan.target).expect("the rule is deleted");
    assert!(comparison(&harness).rows.iter().all(|row| !row.orphan()));
    assert_eq!(plan_revision(&harness).get(), before.get() + 1);
    assert!(
        harness
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .expect("the plan's data")
            .regression_tolerances
            .iter()
            .all(|rule| rule.target != orphan.target)
    );
}

/// With one run there is nothing to compare against; with two and no
/// reference the page asks for one.
#[test]
fn each_empty_state_says_what_compares() {
    let mut harness = compared();
    let current = harness.model.simulation.retained.runs[1].clone();
    harness.model.simulation.retained.runs = vec![current].into();
    assert!(harness.model.simulation.select_run(0));
    draw(&mut harness);
    assert_eq!(comparison(&harness).compared, None);
    assert_eq!(
        empty_text(&harness.model, comparison(&harness)),
        "Run the plan again to compare against a reference."
    );

    let mut harness = SimulateTestHost::harness();
    draw(&mut harness);
    assert_eq!(
        empty_text(&harness.model, comparison(&harness)),
        "No run yet. Run the plan twice to compare one run against the other."
    );
}

/// A tolerance is stated as it is entered, and read back as it is stated.
#[test]
fn a_tolerance_reads_back_as_it_is_stated() {
    let mut harness = compared();
    draw(&mut harness);
    let measured = row(&harness, "gain").target.clone().expect("a target");
    let waveform = row(&harness, "V(out)").target.clone().expect("a target");
    for (target, text) in [
        (&measured, "abs 100m + rel 0.5 %"),
        (&measured, "rel 2 %"),
        (&measured, "abs 1m"),
        (&measured, "max(abs 1m, rel 2 %)"),
        (
            &waveform,
            "abs 20m + rel 2 % \u{00b7} skew 20n \u{00b7} 0 \u{2026} 3u",
        ),
    ] {
        let rule = parse_tolerance_rule(text, target)
            .expect("a tolerance")
            .expect("a rule");
        assert_eq!(format_tolerance_rule(&rule), text);
    }
    let rule = parse_tolerance_rule("10m + 2%", &measured)
        .expect("a tolerance")
        .expect("a rule");
    assert_eq!(
        (
            rule.method,
            rule.absolute_tolerance,
            rule.relative_tolerance
        ),
        (
            RegressionComparisonMethod::AbsoluteRelativeEnvelope,
            10e-3,
            0.02
        )
    );
    assert!(parse_tolerance_rule("abs 1m \u{00b7} skew 1n", &measured).is_err());
    assert!(parse_tolerance_rule("abs 5 %", &measured).is_err());
    assert!(parse_tolerance_rule("rel 1 % + rel 2 %", &measured).is_err());
}

/// The inspector states the selected row's runs, its rule in full and how far
/// the change was allowed to go.
#[test]
fn the_selected_row_states_its_rule_and_allowance() {
    let mut harness = compared();
    draw(&mut harness);
    let index = comparison(&harness)
        .rows
        .iter()
        .position(|row| row.name == "bw")
        .expect("bw");
    let key = comparison(&harness).keys[index].clone();
    harness
        .state
        .verification
        .reference_table
        .select(index, Some(key));
    let facts = selected_reference(&harness.state.verification).expect("a row is selected");
    assert_eq!((facts.title.as_str(), facts.result), ("bw", "Fail"));
    for (label, value) in [
        ("Reference", "18.20 MHz \u{00b7} Run 3"),
        ("Current", "17.10 MHz \u{00b7} Run 4"),
        ("Change", "-1.10 MHz (-6.04 %)"),
        ("Tolerance", "rel 2 %"),
        ("Allowed change", "364.0 kHz"),
    ] {
        assert!(
            facts
                .rows
                .iter()
                .any(|(row_label, row_value)| *row_label == label && *row_value == value),
            "{label} {value} in {:?}",
            facts.rows
        );
    }
}

/// Reference is one table of every compared check, and a selected waveform's
/// overlay directly under it. Nothing of the comparison is stored with the
/// session.
#[test]
fn the_reference_view_is_one_table_and_the_selected_waveform() {
    use super::super::raster::{PAGE_WIDTH, page_geometry};

    let mut harness = compared();
    refresh(&mut harness.cx());
    let index = comparison(&harness)
        .rows
        .iter()
        .position(|row| row.name == "V(out)")
        .expect("the waveform");
    let key = comparison(&harness).keys[index].clone();
    harness
        .state
        .verification
        .reference_table
        .select(index, Some(key));
    let serialized = serde_json::to_value(&harness.state.verification).expect("the session");
    for field in ["reference", "reference_table", "reference_edit"] {
        assert!(serialized.get(field).is_none(), "{field} is stored");
    }

    let geometry = page_geometry(harness, VerificationPage::Reference);
    let toolbar = geometry.one("toolbar").expect("toolbar");
    assert!(geometry.one("toolbar.summary").is_some());
    let head = geometry.one("table.head").expect("the table");
    assert_eq!(
        head.top(),
        toolbar.bottom(),
        "nothing between toolbar and table"
    );
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 5, "four measurements and one waveform");
    assert!(rows.iter().all(|row| row.height() == 24.0));
    assert_eq!(rows[0].top(), head.bottom());
    let detail = geometry.one("split").expect("the waveform's overlay");
    let last = rows.last().expect("rows");
    assert_eq!(
        detail.top(),
        last.bottom() + 1.0,
        "the overlay sits under the table"
    );
    assert_eq!(detail.left(), 0.0);
    assert_eq!(detail.right(), PAGE_WIDTH);
    for (column, edges) in geometry.right_edges() {
        assert_eq!(edges.len(), 1, "{column}: numbers end on one edge");
    }
    assert_eq!(geometry.outside_insets(PAGE_WIDTH), Vec::<String>::new());
}

/// A change between unitless values is spelled as they are: overshoot from
/// 12.60 to 12.10 moves by `-0.50`, never `-500 m`, which reads as metres.
#[test]
fn a_unitless_change_is_spelled_as_its_values_are() {
    let mut harness = compared();
    draw(&mut harness);
    let texts = row_cells(row(&harness, "overshoot"))
        .iter()
        .take(4)
        .map(|cell| cell.text.clone())
        .collect::<Vec<_>>();
    assert_eq!(texts, ["overshoot", "12.60", "12.10", "-0.50"]);
}
