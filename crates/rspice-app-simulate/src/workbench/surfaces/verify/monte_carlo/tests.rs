//! Monte Carlo: every number is the sheet's, the joint yield and the gate in
//! the toolbar, a point select over trials run at several points, the
//! selected row's histogram beside its failing trials, and a limit entered in
//! its cell.

use egui::Color32;

use super::super::histogram::HistogramModel;
use super::super::raster::{
    PageGeometry, VerifyStage, page_input, page_pass, refresh_verification, seed_monte_carlo,
    seed_monte_carlo_at_two_points, seed_monte_carlo_over_different_trials,
};
use super::*;
use crate::ui::widgets::table::{Selection, TABLE_HEAD_H};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::test_host::SimulateTestHost;

/// Forty trials of the plan's Monte Carlo analysis, judged against its 40 dB
/// gain floor: three are under it.
fn monte_carlo_app() -> FeatureHarness<SimulateFeature> {
    let mut harness = SimulateTestHost::harness();
    seed_monte_carlo(&mut harness.model);
    harness.open(VerificationPage::MonteCarlo);
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

fn frame(
    ctx: &egui::Context,
    harness: &mut FeatureHarness<SimulateFeature>,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, PageGeometry) {
    page_pass(ctx, page_input(events), |ui| harness.draw(ui))
}

fn settled(
    ctx: &egui::Context,
    harness: &mut FeatureHarness<SimulateFeature>,
) -> (egui::FullOutput, PageGeometry) {
    let _ = frame(ctx, harness, Vec::new());
    let _ = frame(ctx, harness, Vec::new());
    frame(ctx, harness, Vec::new())
}

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

/// Select the specification row `row` of the sheet.
fn select_row(harness: &mut FeatureHarness<SimulateFeature>, row: usize) {
    harness.state.verification.monte_carlo_table.selection = Some(Selection {
        row: 0,
        key: Some(RowKey::Number(row as u64)),
        column: None,
    });
}

const GATE_95: MonteCarloSpecificationGate =
    MonteCarloSpecificationGate::YieldAtLeast { percent: 95.0 };

/// A row states the sheet's own statistics, and the toolbar its joint yield
/// with the Wilson interval, in the failing tone when the gate does not
/// clear.
#[test]
fn each_row_states_the_sheets_statistics() {
    let harness = monte_carlo_app();
    let sheet = sheet(&harness);
    assert_eq!(monte_carlo_rows(sheet, None), [0]);
    let statistics = sheet.rows[0].statistics.as_ref().expect("trials");
    let shown = population(statistics, None).expect("every trial");
    assert_eq!(shown.trials.len(), 40);
    let count = shown.yield_count.expect("a limited row has a yield");
    assert_eq!((count.passing, count.total), (37, 40));
    assert_eq!(count, statistics.yield_count.expect("the sheet's"));
    assert_eq!(shown.distribution.mean, statistics.distribution.mean);
    assert_eq!(shown.distribution.min, 38.9);
    assert_eq!(
        shown
            .failing()
            .iter()
            .map(|position| statistics.trials[*position].index())
            .collect::<Vec<_>>(),
        [Some(7), Some(19), Some(31)]
    );

    assert!(!yield_holds(&count, &MonteCarloSpecificationGate::NotGated));
    assert!(yield_holds(
        &count,
        &MonteCarloSpecificationGate::YieldAtLeast { percent: 90.0 }
    ));
    assert!(!yield_holds(&count, &GATE_95));
    let line = "Joint yield 92.5 % (37 / 40) \u{00b7} 95 % CI 80\u{2013}97 %";
    assert_eq!(
        monte_carlo_summary(sheet, &MonteCarloSpecificationGate::NotGated),
        Summary::failing(line)
    );
    assert_eq!(
        monte_carlo_summary(
            sheet,
            &MonteCarloSpecificationGate::YieldAtLeast { percent: 90.0 }
        ),
        Summary::plain(format!("{line} \u{00b7} clears yield \u{2265} 90 %"))
    );
    assert_eq!(
        monte_carlo_summary(sheet, &GATE_95),
        Summary::failing(format!("{line} \u{00b7} gain below yield \u{2265} 95 %"))
    );
}

/// The selected row's detail fills the page under the table as one frame:
/// one head band across both halves at the table head's height, the
/// histogram half and the list half sharing their top and their foot, which
/// is the view's.
#[test]
fn the_detail_fills_the_page_as_one_frame() {
    let ctx = context();
    let mut harness = monte_carlo_app();
    select_row(&mut harness, 0);
    let (output, geometry) = settled(&ctx, &mut harness);
    let rows = geometry.all("table.row");
    assert_eq!(
        rows.len(),
        4,
        "the specification and its three failing trials"
    );
    let toolbar = geometry.one("toolbar").expect("toolbar");
    let heads = geometry.all("table.head");
    assert_eq!(heads.len(), 2);
    assert_eq!(heads[0].top(), toolbar.bottom());
    let split = geometry.one("split").expect("the detail is split off");
    assert_eq!(split.top(), rows[0].bottom(), "the table takes its rows");

    let band = geometry.one("detail.band").expect("one head band");
    let plot = geometry.one("detail.plot").expect("the histogram half");
    let list = geometry.one("detail.list").expect("the list half");
    assert_eq!(band.top(), split.bottom());
    assert_eq!(band.height(), TABLE_HEAD_H);
    assert_eq!((band.left(), band.right()), (plot.left(), list.right()));
    assert_eq!(plot.top(), band.bottom(), "the plot is under the band");
    assert_eq!(list.top(), band.top(), "the list's heads are in the band");
    assert_eq!(plot.bottom(), super::super::raster::PAGE_HEIGHT);
    assert_eq!(list.bottom(), plot.bottom(), "both halves run to the foot");
    assert!(list.left() >= plot.right());
    assert_eq!(geometry.one("histogram"), Some(plot));

    assert_eq!(heads[1].y_range(), band.y_range(), "one band across both");
    assert_eq!(heads[1].x_range(), list.x_range());
    let words = painted(&output);
    assert!(
        words
            .iter()
            .any(|(words, _)| words == "gain \u{00b7} 40 trials \u{00b7} 3 fail"),
        "the plot's title states the output, its trials and its failures"
    );
    for seed in ["0x1A32", "0x1A3E", "0x1A4A"] {
        assert!(words.iter().any(|(words, _)| words == seed), "{seed}");
    }
    // The value a trial failed with stands beside its number, the seed last,
    // so a list scrolled on a narrow page still shows the value.
    let left_of = |wanted: &str| {
        shapes(&output)
            .iter()
            .find_map(|shape| match shape {
                egui::Shape::Text(text) if text.galley.text() == wanted => Some(text.pos.x),
                _ => None,
            })
            .expect(wanted)
    };
    assert!(left_of("39.20 dB") < left_of("0x1A32"));

    // A failing trial is the inspector's subject once selected.
    let failing = population(
        sheet(&harness).rows[0].statistics.as_ref().expect("trials"),
        None,
    )
    .expect("every trial")
    .failing();
    harness
        .state
        .verification
        .monte_carlo_trials_table
        .selection = Some(Selection {
        row: 0,
        key: Some(RowKey::Number(failing[0] as u64)),
        column: None,
    });
    let facts = trial_facts(&harness.model, &harness.state.verification);
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].title, "Trial 8 \u{00b7} gain");
    assert_eq!(
        facts[0].rows,
        [
            ("Value".to_owned(), "39.20 dB".to_owned()),
            ("Limit".to_owned(), "\u{2265} 40 dB".to_owned()),
            ("Seed".to_owned(), "0x1A32".to_owned()),
            ("Analysis".to_owned(), "Monte Carlo".to_owned()),
        ]
    );
}

/// On a phone the detail stacks: the histogram over the failing trials, each
/// the page's full width under its own head band, the list as tall as its
/// rows and the plot taking the rest down to the page's foot.
#[test]
fn on_a_phone_the_histogram_sits_over_the_failing_trials() {
    let ctx = context();
    let mut harness = monte_carlo_app();
    select_row(&mut harness, 0);
    let phone = egui::vec2(390.0, 844.0);
    let mut geometry = PageGeometry::default();
    for _ in 0..3 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, phone)),
            ..Default::default()
        };
        geometry = page_pass(&ctx, input, |ui| harness.draw(ui)).1;
    }
    let split = geometry.one("split").expect("the detail is split off");
    let band = geometry.one("detail.band").expect("the plot's head band");
    let plot = geometry.one("detail.plot").expect("the histogram");
    let list = geometry.one("detail.list").expect("the list");
    assert_eq!(split.x_range(), egui::Rangef::new(0.0, phone.x));
    assert_eq!(band.top(), split.bottom());
    assert_eq!(band.height(), TABLE_HEAD_H);
    for part in [band, plot, list] {
        assert_eq!(part.x_range(), split.x_range(), "{part:?} is full width");
    }
    assert_eq!(plot.top(), band.bottom(), "the plot is under its band");
    assert_eq!(
        list.top(),
        plot.bottom() + 1.0,
        "the list is under the plot"
    );
    assert_eq!(list.bottom(), phone.y, "the list runs to the foot");
    assert_eq!(
        list.height(),
        TABLE_HEAD_H + 3.0 * TABLE_ROW_H,
        "the list is as tall as its three failing trials"
    );
    assert!(plot.height() > list.height(), "the plot takes the rest");
    assert_eq!(geometry.one("histogram"), Some(plot));
    let heads = geometry.all("table.head");
    assert_eq!(heads.len(), 2);
    assert_eq!(heads[1].top(), list.top(), "the list's heads are its band");
    assert_eq!(heads[1].x_range(), list.x_range());
}

/// Nothing selected, the view selects the first row whose yield does not
/// hold, else the first, so the row's wash says whose detail shows.
#[test]
fn nothing_selected_selects_the_first_failing_row_else_the_first() {
    let ctx = context();
    let selected = |harness: &FeatureHarness<SimulateFeature>| {
        harness
            .state
            .verification
            .monte_carlo_table
            .selected_key()
            .cloned()
    };
    let mut harness = SimulateTestHost::harness();
    seed_monte_carlo_over_different_trials(&mut harness.model);
    harness.open(VerificationPage::MonteCarlo);
    assert_eq!(selected(&harness), None);
    let (_, geometry) = settled(&ctx, &mut harness);
    assert_eq!(
        selected(&harness),
        Some(RowKey::Number(0)),
        "every yield holds: the first row"
    );
    assert_eq!(geometry.one("table.selected"), geometry.one("table.line.0"));
    assert!(geometry.one("histogram").is_some());

    let limit = super::super::raster::limit;
    super::super::raster::set_limits(
        &mut harness.model,
        vec![
            limit("gain", Some(40.0), None, "dB"),
            limit("idd", None, Some(0.85e-3), "A"),
        ],
    );
    harness.state.verification.monte_carlo_table.selection = None;
    let (_, geometry) = settled(&ctx, &mut harness);
    assert_eq!(
        selected(&harness),
        Some(RowKey::Number(1)),
        "the supply current fails its ceiling"
    );
    assert_eq!(geometry.one("table.selected"), geometry.one("table.line.1"));

    // A row the reader picked stays picked.
    select_row(&mut harness, 0);
    let _ = settled(&ctx, &mut harness);
    assert_eq!(selected(&harness), Some(RowKey::Number(0)));
}

/// A failing trial opens its Monte Carlo analysis in Results narrowed to
/// that trial.
#[test]
fn a_failing_trial_opens_in_results_as_its_member() {
    let mut harness = monte_carlo_app();
    let statistics = sheet(&harness).rows[0].statistics.clone().expect("trials");
    let trial = &statistics.trials[19];
    let evidence = trial_evidence(harness.model.simulation.active_run().expect("run"), trial)
        .expect("the trial is a member of its family");
    assert_eq!(evidence.sample, Some(19));
    open_in_results(&mut harness.cx(), evidence);
    assert_eq!(
        harness
            .wb
            .outbox
            .take::<crate::workbench::framework::requests::SetFamilyFilter>()
            .map(|filter| filter.0),
        Some("sample = 20".to_owned())
    );
    assert_eq!(
        harness.wb.workspace,
        crate::workbench::state::Workspace::Results
    );
}

/// The navigator's Monte Carlo line fails exactly when the toolbar does: a
/// yield that clears the plan's 90 % gate is plain though three trials fail
/// their limit, and one under a 95 % gate fails in both.
#[test]
fn the_navigator_takes_the_toolbars_tone() {
    for (percent, failing) in [(90.0, false), (95.0, true)] {
        let mut harness = monte_carlo_app();
        let plan_id = verification_plan_id(&harness.model).expect("default plan");
        harness
            .model
            .workspace
            .content
            .ensure_active_plan_data(plan_id)
            .specification_policy
            .monte_carlo = MonteCarloSpecificationGate::YieldAtLeast { percent };
        let gate = monte_carlo_gate(&harness.model);
        assert_eq!(gate, MonteCarloSpecificationGate::YieldAtLeast { percent });
        let sheet = sheet(&harness);
        assert!(
            population(sheet.rows[0].statistics.as_ref().expect("trials"), None)
                .is_some_and(|shown| shown.failing().len() == 3),
            "three trials fail at every gate"
        );
        let toolbar = monte_carlo_summary(sheet, &gate).problem;
        let navigator = crate::workbench::docks::navigator::verify::verification_flow_presentation(
            &harness.model,
            &harness.state,
            VerificationPage::MonteCarlo,
        )
        .problem;
        assert_eq!(toolbar, failing.then_some(Problem::Failure), "{percent} %");
        assert_eq!(navigator, toolbar, "{percent} %");
    }
}

/// Two specifications judged over different trials have no joint yield,
/// and the toolbar says why.
#[test]
fn a_joint_yield_over_different_trials_is_not_computed() {
    let mut harness = SimulateTestHost::harness();
    seed_monte_carlo_over_different_trials(&mut harness.model);
    let sheet = sheet(&harness);
    assert_eq!(
        sheet.joint_yield,
        Some(rspice_results::verification::JointYield::DifferentTrials)
    );
    assert_eq!(monte_carlo_rows(sheet, None), [0, 1]);
    assert_eq!(
        monte_carlo_summary(sheet, &MonteCarloSpecificationGate::NotGated),
        Summary::plain("Joint yield not computed: judged over different trials")
    );
}

/// Trials run at two points: the toolbar offers each, "All points" first,
/// and the table and the failing trials follow the one picked.
#[test]
fn a_point_select_narrows_the_table_and_the_trials() {
    let ctx = context();
    let mut harness = SimulateTestHost::harness();
    seed_monte_carlo_at_two_points(&mut harness.model);
    harness.open(VerificationPage::MonteCarlo);
    let sheet_now = sheet(&harness);
    assert_eq!(trial_points(sheet_now), [0, 1]);
    assert!(sheet_now.points[0].nominal);
    let statistics = sheet_now.rows[0].statistics.as_ref().expect("trials");
    let count = |point| {
        population(statistics, point)
            .and_then(|shown| shown.yield_count)
            .map(|count| (count.passing, count.total))
    };
    assert_eq!(count(None), Some((70, 80)));
    assert_eq!(count(Some(0)), Some((37, 40)));
    assert_eq!(count(Some(1)), Some((33, 40)));
    assert_eq!(
        population(statistics, Some(1)).map(|shown| shown.failing().len()),
        Some(7)
    );

    select_row(&mut harness, 0);
    let (_, geometry) = settled(&ctx, &mut harness);
    assert!(geometry.one("toolbar.run").is_some(), "a point select");
    assert_eq!(geometry.all("table.row").len(), 1 + 10);

    harness.state.verification.monte_carlo_point = Some(1);
    let (_, geometry) = settled(&ctx, &mut harness);
    assert_eq!(geometry.all("table.row").len(), 1 + 7);

    // A point the trials no longer ran at states every point.
    harness.state.verification.monte_carlo_point = Some(9);
    assert_eq!(
        shown_point(sheet(&harness), &harness.state.verification),
        None
    );
}

/// With no trial selected, the inspector states each campaign the sheet's
/// statistics came from; two analyses of one label are told apart by the
/// point each ran at, and one campaign is simply the Monte Carlo run.
#[test]
fn the_campaigns_behind_the_statistics_are_named_by_their_points() {
    let mut harness = SimulateTestHost::harness();
    seed_monte_carlo_at_two_points(&mut harness.model);
    let sheet_now = sheet(&harness);
    let campaigns = campaign_facts(&harness.model, Some("12.4 s"));
    let titles = campaigns
        .iter()
        .map(|campaign| campaign.title.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        titles,
        [0, 1].map(|point| format!(
            "Monte Carlo \u{00b7} {}",
            point_name(&sheet_now.points[point])
        ))
    );
    assert_ne!(titles[0], titles[1]);
    assert_eq!(
        campaigns[0].rows,
        [
            ("Samples", "40 / 40"),
            ("Seed", "0x1A2B"),
            ("Sampling", "Pseudo-random"),
            ("Duration", "12.4 s"),
        ]
        .map(|(label, value)| (label.to_owned(), value.to_owned()))
    );

    let mut harness = SimulateTestHost::harness();
    seed_monte_carlo(&mut harness.model);
    let titles = campaign_facts(&harness.model, None)
        .into_iter()
        .map(|campaign| (campaign.title, campaign.rows.len()))
        .collect::<Vec<_>>();
    assert_eq!(titles, [("Monte Carlo".to_owned(), 3)]);
    assert!(campaign_facts(&SimulateTestHost::harness().model, None).is_empty());
}

/// One run's trials at one point offer no point select.
#[test]
fn trials_at_one_point_offer_no_select() {
    let ctx = context();
    let mut harness = monte_carlo_app();
    assert!(trial_points(sheet(&harness)).is_empty());
    let (_, geometry) = settled(&ctx, &mut harness);
    assert!(geometry.one("toolbar.run").is_none());
}

/// A run with no Monte Carlo analysis is one line and the action that adds
/// one; a run whose trials no specification measures says so.
#[test]
fn no_monte_carlo_analysis_is_one_line_and_its_action() {
    let ctx = context();
    let mut harness = SimulateTestHost::harness();
    harness.open(VerificationPage::MonteCarlo);
    let (output, geometry) = settled(&ctx, &mut harness);
    let toolbar = geometry.one("toolbar").expect("toolbar");
    let empty = geometry.one("empty").expect("one line");
    assert_eq!(empty.top(), toolbar.bottom());
    assert!(geometry.one("table.head").is_none());
    assert!(geometry.one("toolbar.summary").is_none());
    let words = painted(&output);
    assert!(
        words
            .iter()
            .any(|(words, _)| words == "This run has no Monte Carlo analysis.")
    );
    assert!(
        words
            .iter()
            .any(|(words, _)| words == "Add a Monte Carlo analysis")
    );

    let mut harness = monte_carlo_app();
    super::super::raster::set_limits(&mut harness.model, Vec::new());
    let (output, geometry) = settled(&ctx, &mut harness);
    assert!(geometry.one("table.head").is_none());
    assert!(
        painted(&output)
            .iter()
            .any(|(words, _)| words == "No specification measures this run's Monte Carlo trials.")
    );
}

/// Open `harness`'s gain limit cell on `text` and press Enter in it.
fn type_limit(
    ctx: &egui::Context,
    mut harness: FeatureHarness<SimulateFeature>,
    text: &str,
) -> FeatureHarness<SimulateFeature> {
    let seed = limit_text(&sheet(&harness).rows[0]);
    harness.state.verification.yield_limit_draft = Some(YieldLimitDraft {
        output: "gain".to_owned(),
        text: text.to_owned(),
        seed,
        focus: true,
    });
    let enter = egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    for events in [Vec::new(), vec![enter]] {
        let _ = frame(ctx, &mut harness, events);
    }
    harness
}

/// Enter applies a typed limit to the plan with a receipt and closes the
/// cell. Words, or a range turned upside down, leave the plan as it was, say
/// why under the toolbar, and keep the cell open on what was typed.
#[test]
fn a_limit_typed_into_its_cell_is_the_plans_on_enter() {
    let mut harness = monte_carlo_app();
    let plan_id = verification_plan_id(&harness.model).expect("default plan");
    let ctx = context();
    let gain = |harness: &FeatureHarness<SimulateFeature>| {
        harness
            .model
            .workspace
            .content
            .active_specs(plan_id)
            .iter()
            .find(|spec| spec.measurement == "gain")
            .map(|spec| (spec.min, spec.max))
    };

    harness = type_limit(&ctx, harness, "\u{2265} 41 dB");
    assert_eq!(gain(&harness), Some((Some(41.0), None)));
    assert_eq!(harness.state.verification.yield_limit_draft, None);
    assert_eq!(harness.state.verification.action_problem, None);
    assert!(
        harness.model.log_buffer.entries().any(|entry| entry
            .message
            .contains("Set the limit of gain to \u{2265} 41 dB.")),
        "the plan's receipt states the edit"
    );
    refresh_verification(&mut harness.model);
    assert_eq!(
        sheet(&harness).rows[0]
            .statistics
            .as_ref()
            .and_then(|statistics| statistics.yield_count)
            .map(|count| count.passing),
        Some(28)
    );

    for text in ["forty", "41 \u{2026} 40.5"] {
        harness = type_limit(&ctx, harness, text);
        assert_eq!(gain(&harness), Some((Some(41.0), None)), "{text}");
        let problem = harness
            .state
            .verification
            .action_problem
            .clone()
            .expect("the refusal is stated");
        assert!(problem.contains("limit of gain was not set"), "{problem}");
        let draft = harness
            .state
            .verification
            .yield_limit_draft
            .clone()
            .expect("the cell stays open");
        assert_eq!(draft.text, text);
    }
}

/// A limit and a tick state a quantity the way the values beside them do:
/// the SI prefix on the unit, one space in front of it.
#[test]
fn a_limit_and_a_tick_are_spelled_like_the_values_beside_them() {
    let mut harness = monte_carlo_app();
    let limit = super::super::raster::limit;
    super::super::raster::set_limits(
        &mut harness.model,
        vec![
            limit("bw", Some(10.0e6), None, "Hz"),
            limit("idd", None, Some(1.2e-3), "A"),
            limit("gain", Some(40.0), None, "dB"),
            limit("pm", Some(45.0), None, "deg"),
            limit("tsettle", None, Some(2.0e-6), "s"),
            limit("vout", Some(1.62), Some(1.98), "V"),
            limit("vbias", None, None, "V"),
        ],
    );
    refresh_verification(&mut harness.model);
    assert_eq!(
        sheet(&harness)
            .rows
            .iter()
            .map(limit_text)
            .collect::<Vec<_>>(),
        [
            "\u{2265} 10 MHz",
            "\u{2264} 1.2 mA",
            "\u{2265} 40 dB",
            "\u{2265} 45 deg",
            "\u{2264} 2 \u{b5}s",
            "1.62 V \u{2026} 1.98 V",
            "",
        ]
    );
    assert_eq!(super::super::histogram::tick_text(38.0, 0.5, "dB"), "38 dB");
    assert_eq!(
        super::super::histogram::tick_text(10.0e6, 2.0e6, "Hz"),
        "10 MHz"
    );
    assert_eq!(format_value(38.9, "dB"), "38.90 dB");
}

/// The bins put the limit on an edge and keep every sample; a bar's failing
/// part counts the trials the sheet failed, not the samples under the line.
#[test]
fn the_histogram_counts_the_sheets_failing_trials() {
    let harness = monte_carlo_app();
    let statistics = sheet(&harness).rows[0].statistics.as_ref().expect("trials");
    let shown = population(statistics, None).expect("every trial");
    let model = histogram_model(&shown.samples(), &[FLOOR_40]).expect("forty samples bin");
    assert_eq!(model.counts.iter().sum::<usize>(), 40);
    assert_eq!(model.failing.iter().sum::<usize>(), 3);
    assert!(
        model.edges.iter().any(|edge| (edge - 40.0).abs() < 1.0e-9),
        "the limit is an edge: {:?}",
        model.edges
    );
    assert_eq!(model.limits, [FLOOR_40]);
    assert!(model.axis.0 <= model.mean - 3.0 * model.std_dev);
    assert!(model.axis.1 >= model.mean + 3.0 * model.std_dev);
    assert!((8..=48).contains(&model.counts.len()));
    assert_eq!(model.count_top(), 12.0, "ten at the peak, and headroom");

    // A trial the sheet failed above the line is counted as failing.
    let guarded = histogram_model(&[(40.5, false), (41.0, true), (41.5, true)], &[FLOOR_40])
        .expect("three samples bin");
    assert_eq!(guarded.failing.iter().sum::<usize>(), 1);
    assert!(histogram_model(&[(f64::NAN, true)], &[FLOOR_40]).is_none());
}

const FLOOR_40: Bound = Bound {
    value: 40.0,
    side: Side::Floor,
};

/// A range's two bounds each fail on their own side, and each is tagged
/// with its own comparison.
#[test]
fn a_ranges_bounds_fail_outward() {
    let bounds = bounds_of(&SpecificationComparison::Range {
        minimum: 1.62,
        maximum: 1.98,
    });
    assert_eq!(
        bounds
            .iter()
            .map(|bound| (bound.side, bound.tag("V")))
            .collect::<Vec<_>>(),
        [
            (Side::Floor, "\u{2265} 1.62 V".to_owned()),
            (Side::Ceiling, "\u{2264} 1.98 V".to_owned()),
        ]
    );
}

/// The forty-trial gain histogram as the view paints it.
fn gain_histogram(ctx: &egui::Context) -> (HistogramModel, egui::FullOutput) {
    let harness = monte_carlo_app();
    let statistics = sheet(&harness).rows[0].statistics.as_ref().expect("trials");
    let model = histogram_model(
        &population(statistics, None).expect("every trial").samples(),
        &[FLOOR_40],
    )
    .expect("forty samples bin");
    let mut output = None;
    for _ in 0..2 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 420.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    super::super::histogram::paint_histogram(ui, "gain", "dB", &model);
                });
            },
        ));
    }
    (model, output.expect("two frames ran"))
}

/// Every shape a frame painted, flattened.
fn shapes(output: &egui::FullOutput) -> Vec<egui::Shape> {
    fn walk(shape: &egui::Shape, out: &mut Vec<egui::Shape>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            shape => out.push(shape.clone()),
        }
    }
    let mut out = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// The limit and the mean cannot be mistaken for each other: the limit is a
/// solid rule in the failing ink, tagged with its comparison at the top; the
/// mean a dot with a dashed drop line, tagged with its value. The ticks carry
/// no unit, which the axis states once, and the count axis is named.
#[test]
fn the_limit_and_the_mean_are_drawn_apart() {
    let ctx = context();
    let (model, output) = gain_histogram(&ctx);
    let t = Tokens::get(&ctx);
    let spec = super::super::histogram::histogram_spec("dB", &model, ("gain histogram", ""), &t);
    let limit = spec
        .markers
        .iter()
        .find(|marker| marker.x == 40.0)
        .expect("the limit is tagged");
    assert_eq!(limit.shape, crate::ui::plot::MarkerShape::LimitLine);
    assert_eq!(limit.color, t.color.err);
    assert_eq!(limit.label, "\u{2265} 40 dB");
    let mean = spec
        .markers
        .iter()
        .find(|marker| marker.x == model.mean)
        .expect("the mean is tagged");
    assert_eq!(mean.shape, crate::ui::plot::MarkerShape::Point);
    assert!(mean.drop_line);
    assert_eq!(mean.color, t.color.accent);
    assert_eq!(
        mean.label,
        format!("\u{00b5} {}", format_value(model.mean, "dB"))
    );
    assert_eq!(
        spec.markers
            .iter()
            .filter(|marker| marker.label.ends_with('\u{03c3}'))
            .count(),
        2,
        "mean \u{00b1} 3 standard deviations"
    );
    assert_eq!(spec.x.unit, "dB");
    assert!(spec.x.ticks.iter().all(|(_, label)| !label.contains("dB")));
    assert_eq!(spec.y.unit, "trials");

    // The limit's rule is the one solid rule two points wide.
    let rules = shapes(&output)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::LineSegment { points, stroke }
                if points[0].x == points[1].x && stroke.width >= 2.0 =>
            {
                Some(stroke.color)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(rules, [t.color.err]);
}

/// Bars wear the Results histogram's tones, the accent wash in an accent
/// outline, and the failing part of a bar the same wash and outline in the
/// failing ink; the canvas theme is worn again once the plot has drawn.
#[test]
fn bars_wear_the_results_tones_and_failing_parts_the_failing_ink() {
    let ctx = context();
    let canvas = Tokens::get(&ctx).color;
    let (_, output) = gain_histogram(&ctx);
    let t = Tokens::get(&ctx);
    assert_eq!(t.color, canvas, "the page's inks are put back");
    let bars = shapes(&output)
        .into_iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(rect) if rect.stroke.width > 0.0 => {
                Some((rect.fill, rect.stroke.color))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(bars.contains(&(t.color.accent_dim, t.color.accent)));
    let failing = t
        .color
        .err
        .gamma_multiply(f32::from(t.color.accent_dim.a()) / 255.0);
    assert!(bars.contains(&(failing, t.color.err)));
    assert!(
        bars.iter()
            .all(|bar| *bar == (t.color.accent_dim, t.color.accent)
                || *bar == (failing, t.color.err)
                || bar.0 == t.color.bg_elevated),
        "{bars:?}"
    );
}

/// On a touch phone the failing-trials list is as tall as its rows at the
/// touch target, not at a fine pointer's grid row, so every trial shows.
#[test]
fn a_touch_phone_lists_every_failing_trial_at_the_touch_row_height() {
    let frame = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(390.0, 700.0));
    let touch = LineHeights {
        head: 44.0,
        row: 44.0,
        band: 24.0,
    };
    let layout = DetailLayout::stacked(frame, 3, touch);
    assert_eq!(layout.list.height(), 44.0 + 3.0 * 44.0);
    assert_eq!(layout.band.height(), 24.0);
    assert_eq!(layout.list.bottom(), frame.bottom());
}
