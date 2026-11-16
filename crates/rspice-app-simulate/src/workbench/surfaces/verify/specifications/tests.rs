//! The Specifications view: what it states of the sheet, and how each edit
//! made from it reaches the plan — as one plan change, judged again at once,
//! without running again.

use rspice_results::specification::{PreparedSpecification, PreparedSpecificationPolicy};
use rspice_results::verification::RowStatus;

use super::super::raster::{limit, page_input, page_pass, seed_corner, set_limits};
use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SpecificationPolicy};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::test_host::SimulateTestHost;
use crate::workbench::state::Workspace;

type Harness = FeatureHarness<SimulateFeature>;

fn refresh(harness: &mut Harness) {
    let model = &mut harness.model;
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
}

fn plan_id(harness: &Harness) -> SimulationPlanId {
    harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the harness plan is stable")
        .id()
}

fn revision(harness: &Harness) -> ObjectRevision {
    harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the harness plan is stable")
        .revision()
}

fn definitions(harness: &Harness) -> Vec<SpecificationDefinition> {
    harness
        .model
        .workspace
        .content
        .plan_data(plan_id(harness))
        .map(|payload| payload.specification_definitions.clone())
        .unwrap_or_default()
}

fn sheet(harness: &Harness) -> &Sheet {
    harness
        .model
        .simulation
        .verification_sheet()
        .expect("the run has been judged")
}

fn row_status(harness: &Harness, measurement: &str) -> Option<RowStatus> {
    sheet(harness)
        .rows
        .iter()
        .find(|row| row.measurement == measurement)
        .and_then(|row| row.status)
}

/// Make the plan's one run a run of it, sealed with the limits the plan holds
/// now, whose one analysis measured `measured`.
fn run_against_the_plan(harness: &mut Harness, measured: &[(&str, f64)]) {
    let plan_id = plan_id(harness);
    let payload = harness.model.workspace.content.plan_data(plan_id);
    let specifications = match payload {
        Some(payload) if !payload.specification_definitions.is_empty() => payload
            .specification_definitions
            .iter()
            .filter_map(|definition| {
                PreparedSpecification::from_definition(definition.clone()).ok()
            })
            .collect(),
        _ => harness
            .model
            .workspace
            .content
            .active_specs(plan_id)
            .iter()
            .filter_map(|entry| PreparedSpecification::new(entry.clone()).ok())
            .collect(),
    };
    let policy = payload
        .and_then(|payload| {
            PreparedSpecificationPolicy::new(payload.specification_policy.clone()).ok()
        })
        .unwrap_or_default();
    let source = AnalysisInstanceId::new();
    let task = crate::state::PreparedRunTaskReceipt::new(
        source,
        ObjectRevision::INITIAL,
        Vec::new(),
        2,
        ContentDigest::from_bytes([2; 32]),
    )
    .expect("fixture task receipt is internally valid");
    let receipt = crate::state::PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(plan_id),
        project_revision: ObjectRevision::INITIAL,
        prepared_snapshot_digest: ContentDigest::from_bytes([0x5a; 32]),
        source_content_digest: ContentDigest::from_bytes([0x41; 32]),
        source_check_receipt: crate::state::PreparedSourceCheckReceipt::SchematicDrc(
            ContentDigest::from_bytes([0x42; 32]),
        ),
        project_model_sources: Vec::new(),
        specifications,
        specification_policy: policy,
        tasks: vec![task],
    })
    .expect("fixture run receipt is internally valid");
    let mut run = SimulationRun::new_prepared(1, receipt);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_measurements(
                measured
                    .iter()
                    .map(|(name, value)| rspice_core::MeasureResult::success(name, *value))
                    .collect(),
            )
            .with_provenance(
                crate::state::AnalysisResultProvenance::new(
                    source,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x5a; 32]),
                    Vec::new(),
                )
                .expect("fixture provenance is internally valid"),
            ),
    );
    harness.model.simulation.retained.runs = vec![run].into();
    assert!(harness.model.simulation.select_run(0));
    refresh(harness);
}

/// Every word a pass painted, in paint order.
fn painted(output: &egui::FullOutput) -> Vec<String> {
    fn walk(shape: &egui::Shape, words: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => words.push(text.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, words)),
            _ => {}
        }
    }
    let mut words = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut words);
    }
    words
}

/// Draw the Verify surface three times, as a reader's first look does, and
/// say what the last pass painted and where.
fn draw(harness: &mut Harness) -> (egui::FullOutput, super::super::raster::PageGeometry) {
    harness.wb.activate(Workspace::Verify);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut last = None;
    for _ in 0..3 {
        last = Some(page_pass(&ctx, page_input(Vec::new()), |ui| {
            super::super::show(ui, &mut harness.cx());
        }));
    }
    last.expect("three passes")
}

#[test]
fn verify_opens_on_its_specifications() {
    let harness = SimulateTestHost::harness();
    assert_eq!(
        harness.state.verification_page,
        VerificationPage::Specifications
    );
    assert_eq!(
        VerificationPage::NAVIGATION[0],
        VerificationPage::Specifications
    );
    assert_eq!(VerificationPage::Specifications.label(), "Specifications");
    assert_eq!(
        crate::workbench::simulate::commands::SimulateCommand::Verify(
            VerificationPage::Specifications
        )
        .key()
        .as_str(),
        "verify-specifications"
    );
}

/// The table states the sheet's own words for every row, and the table, the
/// toolbar and the navigator count the same specifications and failures.
#[test]
fn the_table_the_toolbar_and_the_navigator_state_the_sheet() {
    let mut harness = SimulateTestHost::harness();
    seed_corner(&mut harness.model);
    refresh(&mut harness);
    let sheet = sheet(&harness).clone();
    assert!(sheet.is_judged());
    assert_eq!(counts_line(&sheet), "4 specifications \u{b7} 1 fails");

    let view = sheet_view(&harness.model, None);
    assert_eq!(
        view.columns,
        [
            Col::Specification,
            Col::Limit,
            Col::Worst,
            Col::At,
            Col::Margin,
            Col::Status
        ],
        "seven points name where the worst was; no trials, no yield, and no point the run marks nominal"
    );
    let cell = |measurement: &str, col: Col| {
        let row = view
            .row(measurement)
            .expect("a row for every specification");
        view.rows[row].cells[view.column(col).expect("a shown column")]
            .text
            .clone()
    };
    for row in &sheet.rows {
        let measurement = row.measurement.as_str();
        assert_eq!(
            cell(measurement, Col::Status),
            row.status.map_or("\u{2014}", status_word),
            "{measurement}"
        );
        let worst = row.worst.as_ref().expect("every row has a worst value");
        assert_eq!(
            cell(measurement, Col::Worst),
            format_value(worst.value.expect("a measured worst"), &row.unit)
        );
        assert_eq!(
            cell(measurement, Col::At),
            worst_at(&sheet, worst).expect("a named point")
        );
    }
    assert_eq!(cell("gain", Col::Status), "Fail");
    assert_eq!(cell("bw", Col::Status), "Pass");
    assert_eq!(cell("gain", Col::Limit), "\u{2265} 40 dB");
    assert_eq!(cell("idd", Col::Limit), "\u{2264} 1.2 mA");
    assert_eq!(cell("bw", Col::Limit), "\u{2265} 10 MHz");

    let summary = view.summary.as_ref().expect("a summary").text.clone();
    assert!(summary.starts_with(&counts_line(&sheet)), "{summary}");
    let navigator = crate::workbench::docks::navigator::verify::verification_flow_presentation(
        &harness.model,
        &harness.state,
        VerificationPage::Specifications,
    );
    assert_eq!(navigator.detail, counts_line(&sheet));
    assert_eq!(navigator.problem, Some(Problem::Failure));

    let (output, geometry) = draw(&mut harness);
    assert_eq!(geometry.all("table.row").len(), 4);
    assert!(geometry.one("toolbar.summary").is_some());
    let words = painted(&output);
    assert!(
        words.iter().any(|word| word == "Specifications"),
        "{words:?}"
    );
}

/// The shell's filter field narrows the table to the rows it matches, and
/// cleared shows them all again.
#[test]
fn the_shell_filter_narrows_the_rows() {
    let mut harness = SimulateTestHost::harness();
    seed_corner(&mut harness.model);
    refresh(&mut harness);
    harness.wb.activate(Workspace::Verify);
    harness.wb.set_navigator_filter("idd");
    let (_, geometry) = draw(&mut harness);
    assert_eq!(geometry.all("table.row").len(), 1);
    harness.wb.set_navigator_filter("");
    let (_, geometry) = draw(&mut harness);
    assert_eq!(geometry.all("table.row").len(), 4);
}

/// A plan with no run keeps its rows, with dashes where a verdict would be,
/// and the summary says why nothing is judged.
#[test]
fn without_a_run_the_rows_stay_and_the_summary_says_why() {
    let mut harness = SimulateTestHost::harness();
    set_limits(
        &mut harness.model,
        vec![limit("gain", Some(40.0), None, "dB")],
    );
    harness.model.simulation.clear_runs();
    refresh(&mut harness);
    let view = sheet_view(&harness.model, None);
    assert_eq!(view.rows.len(), 1);
    let status = view.column(Col::Status).expect("a status column");
    assert_eq!(view.rows[0].cells[status].text, "\u{2014}");
    let summary = view.summary.expect("a summary").text;
    assert!(
        summary == "1 specification \u{b7} not judged until the plan runs",
        "{summary}"
    );
}

/// A limit typed after the run is one plan change, judged against the run
/// already held — no run is started — and the summary says the limits moved
/// since it. One undo puts the limit back.
#[test]
fn a_limit_typed_after_the_run_rejudges_it_without_running_again() {
    let mut harness = SimulateTestHost::harness();
    set_limits(
        &mut harness.model,
        vec![limit("gain", Some(41.0), None, "dB")],
    );
    run_against_the_plan(&mut harness, &[("gain", 40.0)]);
    assert_eq!(row_status(&harness, "gain"), Some(RowStatus::Fail));
    assert_eq!(sheet(&harness).summary.limits, LimitsSinceRun::Unchanged);

    let before = revision(&harness);
    let view = sheet_view(&harness.model, Some("gain"));
    let row = &view.rows[view.row("gain").expect("the gain row")];
    let typed = SpecificationCellDraft {
        measurement: "gain".to_owned(),
        cell: SpecificationCell::Limit,
        text: "\u{2265} 30 dB".to_owned(),
        seed: "\u{2265} 41 dB".to_owned(),
        focus: false,
    };
    apply_cell(&mut harness.cx(), row, &typed).expect("the limit is set");

    assert_eq!(revision(&harness), before.next().expect("a next revision"));
    assert_eq!(
        definitions(&harness)[0].comparison,
        SpecificationComparison::Minimum { limit: 30.0 }
    );
    assert_eq!(row_status(&harness, "gain"), Some(RowStatus::Pass));
    assert_eq!(sheet(&harness).summary.limits, LimitsSinceRun::Edited);
    assert_eq!(harness.model.simulation.retained.runs.len(), 1);
    let summary = sheet_view(&harness.model, None)
        .summary
        .expect("a summary")
        .text;
    assert!(summary.ends_with("limits edited since Run 1"), "{summary}");

    assert!(harness.model.can_undo_simulation_setup());
    let _ = harness.model.undo_simulation_setup();
    refresh(&mut harness);
    assert_eq!(row_status(&harness, "gain"), Some(RowStatus::Fail));
}

/// A limit the reader cannot mean is refused with the reason, and the plan
/// keeps the one it had.
#[test]
fn a_limit_that_says_no_direction_is_refused() {
    let mut harness = SimulateTestHost::harness();
    set_limits(&mut harness.model, vec![limit("gain", None, None, "dB")]);
    refresh(&mut harness);
    let before = revision(&harness);
    let view = sheet_view(&harness.model, Some("gain"));
    let typed = SpecificationCellDraft {
        measurement: "gain".to_owned(),
        cell: SpecificationCell::Limit,
        text: "40".to_owned(),
        seed: String::new(),
        focus: false,
    };
    let refused = apply_cell(&mut harness.cx(), &view.rows[0], &typed).expect_err("no direction");
    assert!(refused.contains("\u{2265} 40 or \u{2264} 40"), "{refused}");
    assert_eq!(revision(&harness), before);
}

#[test]
fn every_way_of_writing_a_limit_reads_as_written() {
    let policy = rspice_vocabulary::quantity::QuantityPresentationPolicy::default();
    let locale = rspice_vocabulary::quantity::UiNumberLocale::default();
    let tracked = SpecificationComparison::Tracked;
    let read = |text: &str, unit: &str| parse_limit(text, unit, &tracked, policy, locale);
    assert_eq!(
        read("\u{2265} 40 dB", "dB"),
        Ok(SpecificationComparison::Minimum { limit: 40.0 })
    );
    assert_eq!(
        read(">= 40", "dB"),
        Ok(SpecificationComparison::Minimum { limit: 40.0 })
    );
    assert_eq!(
        read("<= 1.2m", "A"),
        Ok(SpecificationComparison::Maximum { limit: 1.2e-3 })
    );
    assert_eq!(
        read("10M..20M", "Hz"),
        Ok(SpecificationComparison::Range {
            minimum: 10.0e6,
            maximum: 20.0e6
        })
    );
    assert_eq!(
        read("50 \u{b1} 1", "\u{3a9}"),
        Ok(SpecificationComparison::EqualWithin {
            target: 50.0,
            tolerance: 1.0
        })
    );
    assert_eq!(read("", "dB"), Ok(SpecificationComparison::Tracked));
    assert_eq!(
        parse_limit(
            "42 dB",
            "dB",
            &SpecificationComparison::Minimum { limit: 40.0 },
            policy,
            locale
        ),
        Ok(SpecificationComparison::Minimum { limit: 42.0 }),
        "a bare value keeps the direction it replaces"
    );
    // What a cell opens with reads back as the limit it shows.
    for comparison in [
        SpecificationComparison::Minimum { limit: 40.0 },
        SpecificationComparison::Maximum { limit: 1.2e-3 },
        SpecificationComparison::Range {
            minimum: 10.0e6,
            maximum: 20.0e6,
        },
    ] {
        let spelled = limit_spelling(&comparison, "Hz", policy, locale);
        assert_eq!(
            parse_limit(&spelled, "Hz", &tracked, policy, locale),
            Ok(comparison),
            "{spelled}"
        );
    }
    let half_a_decibel = SpecificationComparison::Minimum { limit: 0.5 };
    let spelled = limit_spelling(&half_a_decibel, "dB", policy, locale);
    assert_eq!(spelled, "\u{2265} 0.5 dB");
    assert_eq!(
        parse_limit(&spelled, "dB", &tracked, policy, locale),
        Ok(half_a_decibel)
    );
}

/// A margin reads like the value it is the difference of: no finer than the
/// worst value's last digit, in its own prefix, signed either way.
#[test]
fn a_margin_carries_the_resolution_of_its_worst_value() {
    let mut harness = SimulateTestHost::harness();
    seed_corner(&mut harness.model);
    refresh(&mut harness);
    let view = sheet_view(&harness.model, None);
    let cell = |measurement: &str, col: Col| {
        let row = view.row(measurement).expect("a row");
        view.rows[row].cells[view.column(col).expect("a shown column")]
            .text
            .clone()
    };
    for (measurement, worst, margin) in [
        ("gain", "39.40 dB", "-0.60 dB"),
        ("bw", "11.90 MHz", "+1.90 MHz"),
        ("pm", "47.00 deg", "+2.00 deg"),
        ("idd", "1.160 mA", "+40 \u{b5}A"),
    ] {
        assert_eq!(cell(measurement, Col::Worst), worst, "{measurement}");
        assert_eq!(cell(measurement, Col::Margin), margin, "{measurement}");
    }

    // Under the value's last digit a margin is zero spelled as the value is,
    // on its side; past four significant digits it rounds as a value does.
    assert_eq!(difference_text(-0.004, 39.996, "dB"), "-0.00 dB");
    assert_eq!(difference_text(0.0, 1.2e-3, "A"), "0.000 mA");
    assert_eq!(difference_text(12.345_6, 0.5, "V"), "+12.35 V");
    assert_eq!(difference_text(-2.4e-9, 1.0e-6, "s"), "-2 ns");
}

/// A level in decibels, a percentage and an angle take no SI prefix: six
/// tenths of a decibel is `-0.6000 dB`, never `-600.0 mdB`.
#[test]
fn a_level_or_an_angle_takes_no_si_prefix() {
    assert_eq!(format_value(-0.6, "dB"), "-0.6000 dB");
    assert_eq!(format_value(2.0, "deg"), "2.000 deg");
    assert_eq!(format_value(40.0e-6, "A"), "40.00 \u{b5}A");
    assert_eq!(
        limit_text(&SpecificationComparison::Maximum { limit: 1.2e-3 }, "A").as_deref(),
        Some("\u{2264} 1.2 mA")
    );
}

#[test]
fn a_name_typed_into_its_cell_renames_the_specification_not_its_measurement() {
    let mut harness = SimulateTestHost::harness();
    set_limits(
        &mut harness.model,
        vec![limit("gain", Some(40.0), None, "dB")],
    );
    refresh(&mut harness);
    let before = revision(&harness);
    let view = sheet_view(&harness.model, Some("gain"));
    let typed = SpecificationCellDraft {
        measurement: "gain".to_owned(),
        cell: SpecificationCell::Name,
        text: "Open-loop gain".to_owned(),
        seed: "gain".to_owned(),
        focus: false,
    };
    apply_cell(&mut harness.cx(), &view.rows[0], &typed).expect("the name is set");
    assert_eq!(revision(&harness), before.next().expect("a next revision"));
    let definition = &definitions(&harness)[0];
    assert_eq!(definition.requirement_name, "Open-loop gain");
    assert_eq!(definition.measurement, "gain");
    let view = sheet_view(&harness.model, None);
    assert_eq!(view.rows[0].cells[0].text, "Open-loop gain");
}

/// Add offers the run's measurements no specification names, and adds the
/// one picked tracked and selected; New measurement adds a fresh name.
#[test]
fn adding_offers_the_run_measurements_no_specification_names() {
    let mut harness = SimulateTestHost::harness();
    set_limits(
        &mut harness.model,
        vec![limit("gain", Some(40.0), None, "dB")],
    );
    run_against_the_plan(&mut harness, &[("gain", 42.0), ("bw", 1.0e6)]);
    assert_eq!(
        unnamed_measurements(&harness.model),
        [("bw".to_owned(), String::new())]
    );

    let before = revision(&harness);
    add_specification(&mut harness.cx(), "bw", "");
    assert_eq!(revision(&harness), before.next().expect("a next revision"));
    let added = definitions(&harness);
    assert_eq!(added.len(), 2);
    assert_eq!(added[1].measurement, "bw");
    assert_eq!(added[1].comparison, SpecificationComparison::Tracked);
    assert_eq!(harness.state.selected_specification.as_deref(), Some("bw"));
    assert!(unnamed_measurements(&harness.model).is_empty());
    assert_eq!(row_status(&harness, "bw"), Some(RowStatus::Tracked));

    assert_eq!(new_measurement_name(&harness.model), "meas1");
    add_specification(&mut harness.cx(), "meas1", "");
    assert_eq!(definitions(&harness).len(), 3);
    assert_eq!(
        harness.state.selected_specification.as_deref(),
        Some("meas1")
    );
    assert_eq!(new_measurement_name(&harness.model), "meas2");
}

#[test]
fn deleting_a_specification_is_one_plan_change() {
    let mut harness = SimulateTestHost::harness();
    set_limits(
        &mut harness.model,
        vec![
            limit("gain", Some(40.0), None, "dB"),
            limit("bw", Some(10.0e6), None, "Hz"),
        ],
    );
    refresh(&mut harness);
    harness.state.selected_specification = Some("gain".to_owned());
    let before = revision(&harness);
    delete_specification(&mut harness.cx(), "gain");
    assert_eq!(revision(&harness), before.next().expect("a next revision"));
    let left = definitions(&harness);
    assert_eq!(left.len(), 1);
    assert_eq!(left[0].measurement, "bw");
    assert_eq!(harness.state.selected_specification, None);
    assert_eq!(sheet(&harness).rows.len(), 1);
}

/// The acceptance settings commit as one plan change each; a gate the plan
/// cannot hold is refused, and the plan keeps the policy it had.
#[test]
fn an_acceptance_policy_commits_atomically_and_refuses_an_invalid_yield_gate() {
    use crate::state::{
        MissingMeasurementPolicy, MonteCarloSpecificationGate, NominalFailurePolicy,
        RegressionSpecificationPolicy,
    };

    let mut harness = SimulateTestHost::harness();
    let id = plan_id(&harness);
    let before = revision(&harness);
    let accepted = SpecificationPolicy {
        nominal_failure: NominalFailurePolicy::RecordDisposition,
        monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 99.9 },
        regression: RegressionSpecificationPolicy::LimitOnly,
        missing_measurement: MissingMeasurementPolicy::ReportUnmapped,
    };
    super::super::specification_edit::edit_policy(&mut harness.cx(), accepted.clone())
        .expect("the policy commits");
    let stored = &harness
        .model
        .workspace
        .content
        .plan_data(id)
        .expect("plan payload")
        .specification_policy;
    assert!(stored.bitwise_eq(&accepted));
    assert_eq!(revision(&harness), before.next().expect("a next revision"));

    let after_accept = revision(&harness);
    super::super::specification_edit::edit_policy(
        &mut harness.cx(),
        SpecificationPolicy {
            monte_carlo: MonteCarloSpecificationGate::YieldAtLeast { percent: 100.1 },
            ..accepted.clone()
        },
    )
    .expect_err("no yield passes more than every trial");
    let stored = &harness
        .model
        .workspace
        .content
        .plan_data(id)
        .expect("plan payload")
        .specification_policy;
    assert!(stored.bitwise_eq(&accepted));
    assert_eq!(
        revision(&harness),
        after_accept,
        "a refused policy cannot move the plan revision"
    );
    assert!(harness.model.analysis_lifecycle_status.is_refusal());
}

/// The sheet as shown leaves the file: the columns on screen, the rows the
/// filter keeps, a dash as an empty field.
#[test]
fn the_csv_is_the_sheet_as_shown() {
    let mut harness = SimulateTestHost::harness();
    seed_corner(&mut harness.model);
    refresh(&mut harness);
    let view = sheet_view(&harness.model, None);
    let text = sheet_csv(&view, "");
    let lines = text.lines().collect::<Vec<_>>();
    assert_eq!(lines[0], "Specification,Limit,Worst,At,Margin,Status");
    assert_eq!(lines.len(), 5);
    assert!(lines[1].starts_with("gain,\u{2265} 40 dB,"), "{}", lines[1]);
    assert!(lines[1].ends_with(",Fail"), "{}", lines[1]);
    let filtered = sheet_csv(&view, "idd");
    assert_eq!(filtered.lines().count(), 2);
}
