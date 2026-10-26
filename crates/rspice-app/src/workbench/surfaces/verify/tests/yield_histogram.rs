//! The Monte Carlo yield page: limits entered in their cells and spelled like
//! the values beside them, statistics from the retained samples, the yield the
//! judge counted, and the histogram drawn under the table.

use super::super::super::super::app_state::AppState;
use super::super::super::super::state::{YieldLimitBound, YieldLimitDraft};
use super::*;

fn fixture_yield_results(app: &RSpiceApp) -> Vec<rspice_results::yield_analysis::YieldResult> {
    verification_yield_results(app).to_vec()
}

/// A Spec cell and an axis tick state a quantity the way the measured values
/// beside them do: the SI prefix on the unit, one space in front of it.
///
/// The Spec column used to read `≥ 10M Hz` and `≤ 1.200m A` — the prefix split
/// off the unit it scales, and three zeros nobody authored — while the value
/// in the next cell read `11.90 MHz`.
#[test]
fn a_limit_and_a_tick_are_spelled_like_the_values_beside_them() {
    let bound = |min: Option<f64>, max: Option<f64>, unit: &str| {
        super::limit_cell(&SpecEntry {
            measurement: "m".to_owned(),
            expression: String::new(),
            min,
            max,
            unit: unit.to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        })
        .text
    };

    assert_eq!(bound(Some(10.0e6), None, "Hz"), "\u{2265} 10 MHz");
    assert_eq!(bound(None, Some(1.2e-3), "A"), "\u{2264} 1.2 mA");
    assert_eq!(bound(Some(40.0), None, "dB"), "\u{2265} 40 dB");
    assert_eq!(bound(Some(45.0), None, "deg"), "\u{2265} 45 deg");
    assert_eq!(bound(None, Some(2.0e-6), "s"), "\u{2264} 2 \u{b5}s");
    assert_eq!(bound(Some(1.62), Some(1.98), "V"), "1.62 V \u{2026} 1.98 V");
    assert_eq!(
        bound(None, None, "V"),
        "",
        "a bound of nothing says nothing"
    );

    // One space before the unit, the same one the values carry.
    assert_eq!(super::histogram::tick_text(38.0, 0.5, "dB"), "38 dB");
    assert_eq!(super::histogram::tick_text(10.0e6, 2.0e6, "Hz"), "10 MHz");
    assert_eq!(super::format_value(38.9, "dB"), "38.90 dB");
}

/// One row per output the trials measured. Its statistics come from the exact
/// retained samples, and its yield is the population the judge counted.
#[test]
fn yield_rows_compute_their_statistics_from_the_samples() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    let analysis = app
        .state
        .model
        .simulation
        .yield_analysis()
        .expect("the run holds a Monte Carlo analysis");
    let rows = yield_rows_of(&analysis.data, &results, &verification_specs(&app));
    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert_eq!((row.output.as_str(), row.unit.as_str()), ("gain", "dB"));
    let samples = &results[0].samples;
    assert_eq!(samples.len(), 40);
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let stats = row.stats.expect("forty samples have statistics");
    assert!((stats.mean - mean).abs() < 1.0e-12);
    assert_eq!(stats.min, 38.9);
    assert_eq!(
        stats.max,
        samples.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    );
    assert!((stats.max - 43.2).abs() < 1.0e-9);
    assert!(stats.std_dev > 0.5 && stats.std_dev < 2.0);
    let cpk = stats.cpk.expect("a lower limit has a one-sided Cpk");
    assert!((cpk - (stats.mean - 40.0) / (3.0 * stats.std_dev)).abs() < 1.0e-9);
    assert_eq!(row.judged, Some((37, 40)));
    assert_eq!(row.yield_percent(), Some(92.5));
    assert_eq!(
        row.verdict(&rspice_results::specification::MonteCarloSpecificationGate::NotGated),
        Some(Verdict::Fail)
    );
    assert_eq!(
        row.limit
            .as_ref()
            .expect("the plan limits gain")
            .limit_text(),
        "≥ 40 dB"
    );
    assert_eq!(
        yield_summary(&results, true),
        Some(super::toolbar::Summary::failing("yield 92.5 % (37 / 40)"))
    );
}

#[test]
fn the_shown_histogram_is_the_readers_pick_or_the_lowest_yield() {
    let row = |output: &str, passing: usize| YieldRow {
        output: output.to_owned(),
        unit: String::new(),
        limit: Some(SpecEntry {
            measurement: output.to_owned(),
            expression: String::new(),
            min: Some(0.0),
            max: None,
            unit: String::new(),
            scope: crate::state::SpecPointScope::AllPoints,
        }),
        stats: None,
        judged: Some((passing, 10)),
        nameable: true,
    };
    let rows = [row("gain", 9), row("bw", 7), row("pm", 10)];
    assert_eq!(shown_yield_row(&rows, None), Some(1));
    assert_eq!(shown_yield_row(&rows, Some("pm")), Some(2));
    assert_eq!(shown_yield_row(&rows, Some("gone")), Some(1));
    assert_eq!(shown_yield_row(&[], None), None);
}

/// A limit entered for an output is the plan's requirement on it, and the
/// retained trials are judged against it the next time the frame judges them.
/// A lower limit above the upper one is refused with the plan unchanged.
#[test]
fn a_limit_set_in_the_plan_rejudges_the_retained_trials() {
    let mut app = super::raster::monte_carlo_app();
    let plan_id = verification_plan_id(&app).expect("default plan");
    let definitions = |app: &RSpiceApp| {
        app.state
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .map(|payload| payload.specification_definitions.clone())
            .unwrap_or_default()
    };

    let receipt = set_limit(
        &mut app.state.model.workspace.content,
        plan_id,
        "gain",
        YieldLimitBound::Lower,
        Some(41.0),
        "dB",
    );
    assert_eq!(
        receipt.as_deref(),
        Ok("Set the lower limit of gain to 41 dB.")
    );
    let model = &mut app.state.model;
    model
        .simulation
        .refresh_yield_evidence(&model.sim_setup, &model.workspace.content);
    let results = fixture_yield_results(&app);
    assert_eq!(
        (results[0].pass_count, results[0].total_runs),
        (28, 40),
        "the twelve trials under 41 dB fail"
    );

    let before = definitions(&app);
    assert_eq!(
        set_limit(
            &mut app.state.model.workspace.content,
            plan_id,
            "gain",
            YieldLimitBound::Upper,
            Some(40.5),
            "dB",
        ),
        Err("specification minimum exceeds its maximum".to_owned())
    );
    assert_eq!(definitions(&app), before, "a refused limit changes nothing");
}

/// What a cell reads: `10 MHz`, `900m` and `40 dB` the way every field does,
/// an empty cell as no bound, and words as no number at all.
#[test]
fn a_limit_cell_reads_quantities_the_way_every_field_does() {
    let read = |text: &str, unit: &str| {
        parse_limit(
            text,
            unit,
            Default::default(),
            rspice_vocabulary::quantity::UiNumberLocale::default(),
        )
    };
    assert_eq!(read("10 MHz", "Hz"), Ok(Some(10.0e6)));
    assert_eq!(read("40 dB", "dB"), Ok(Some(40.0)));
    assert_eq!(read("40", "dB"), Ok(Some(40.0)));
    assert_eq!(read("900m", "V"), Ok(Some(0.9)));
    assert_eq!(read("  ", "dB"), Ok(None));
    assert!(read("forty", "dB").is_err());
}

/// Show one frame of `app`'s yield page with its gain limit cell open on
/// `text`, then press Enter in it.
fn type_limit(
    ctx: &egui::Context,
    mut app: RSpiceApp,
    bound: YieldLimitBound,
    text: &str,
) -> RSpiceApp {
    let seed = app
        .state
        .model
        .workspace
        .content
        .specs
        .iter()
        .find(|spec| spec.measurement == "gain")
        .and_then(|spec| match bound {
            YieldLimitBound::Lower => spec.min,
            YieldLimitBound::Upper => spec.max,
        })
        .map(|value| format!("{value} dB"))
        .unwrap_or_default();
    app.state.simulate.verification.yield_limit_draft = Some(YieldLimitDraft {
        output: "gain".to_owned(),
        bound,
        text: text.to_owned(),
        seed,
        focus: true,
    });
    // The cell takes the keyboard on the frame it opens.
    let enter = egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    for events in [Vec::new(), vec![enter]] {
        let _ = super::raster::page_pass(ctx, super::raster::page_input(events), |ui| {
            super::super::show(ui, &mut app);
        });
    }
    app
}

/// Enter applies a typed limit to the plan with a receipt, closes the cell,
/// and the next judgment follows it. Words, or a lower limit above the upper,
/// leave the plan as it was, say why under the toolbar, and keep the cell open
/// on what was typed.
#[test]
fn a_limit_typed_into_its_cell_is_the_plans_on_enter() {
    let mut app = super::raster::monte_carlo_app();
    app.state.simulate.verification_page = VerificationPage::Yield;
    let plan_id = verification_plan_id(&app).expect("default plan");
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let gain = |app: &RSpiceApp| {
        app.state
            .model
            .workspace
            .content
            .active_specs(plan_id)
            .iter()
            .find(|spec| spec.measurement == "gain")
            .map(|spec| (spec.min, spec.max))
    };

    app = type_limit(&ctx, app, YieldLimitBound::Lower, "41 dB");
    assert_eq!(gain(&app), Some((Some(41.0), None)));
    assert_eq!(app.state.simulate.verification.yield_limit_draft, None);
    assert_eq!(app.state.simulate.verification.action_problem, None);
    assert!(
        app.state.model.log_buffer.entries().any(|entry| entry
            .message
            .contains("Set the lower limit of gain to 41 dB.")),
        "the plan's receipt states the edit"
    );
    let model = &mut app.state.model;
    model
        .simulation
        .refresh_yield_evidence(&model.sim_setup, &model.workspace.content);
    assert_eq!(fixture_yield_results(&app)[0].pass_count, 28);

    for (bound, text) in [
        (YieldLimitBound::Lower, "forty"),
        (YieldLimitBound::Upper, "40.5 dB"),
    ] {
        app = type_limit(&ctx, app, bound, text);
        assert_eq!(gain(&app), Some((Some(41.0), None)), "{text}");
        let problem = app
            .state
            .simulate
            .verification
            .action_problem
            .clone()
            .expect("the refusal is stated");
        assert!(problem.contains("limit of gain was not set"), "{problem}");
        let draft = app
            .state
            .simulate
            .verification
            .yield_limit_draft
            .clone()
            .expect("the cell stays open");
        assert_eq!((draft.bound, draft.text.as_str()), (bound, text));
    }
}

/// Yield is derived, never stored: the session eframe restores, the same
/// session read strictly, and the project saved and reopened each judge the
/// retained trials against the limits they carried, and state the same yield.
#[test]
fn a_restored_session_and_a_reopened_project_state_the_same_yield() {
    struct Stored(String);

    impl eframe::Storage for Stored {
        fn get_string(&self, key: &str) -> Option<String> {
            (key == eframe::APP_KEY).then(|| self.0.clone())
        }

        fn set_string(&mut self, _: &str, _: String) {}

        fn remove_string(&mut self, _: &str) {}

        fn flush(&mut self) {}
    }

    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    assert_eq!((results[0].pass_count, results[0].total_runs), (37, 40));
    let judged = serde_json::to_value(&results).expect("the yield serializes");

    let session = ron::to_string(&app.state).expect("the session serializes as RON");
    let restored =
        AppState::restore_eframe_session(&Stored(session.clone())).expect("a stored session");
    let strict = ron::from_str::<Box<AppState>>(&session).expect("the session reads strictly");
    for (route, mut state) in [("eframe", Box::new(restored)), ("strict RON", strict)] {
        let model = &mut state.model;
        assert!(
            model
                .simulation
                .yield_results_for_active_dataset()
                .is_none(),
            "{route}: the yield is not stored"
        );
        model
            .simulation
            .refresh_yield_evidence(&model.sim_setup, &model.workspace.content);
        let results = model
            .simulation
            .yield_results_for_active_dataset()
            .expect("the restored run is judged");
        assert_eq!(
            serde_json::to_value(results).expect("the yield serializes"),
            judged,
            "{route}"
        );
    }

    let project =
        rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(&app.state)
            .expect("the project captures");
    let text = rspice_app_services::io::project_io::serialize_project_file(&project)
        .expect("the project serializes");
    let file = rspice_app_services::io::project_io::load_project_text(&text, None)
        .expect("the project reopens")
        .file;
    let (setup, _, _) = rspice_app_services::io::restore_execution_context(
        file.execution_context.expect("the project saves its plan"),
        file.workspace.project.id(),
    )
    .expect("the plan restores");
    let mut simulation =
        rspice_app_services::io::simulation_state_from_results(file.simulation_results)
            .expect("the runs restore");
    simulation.refresh_yield_evidence(&setup, &file.workspace);
    let results = simulation
        .yield_results_for_active_dataset()
        .expect("the reopened run is judged");
    assert_eq!(
        serde_json::to_value(results).expect("the yield serializes"),
        judged,
        "project file"
    );
}

/// The bins are laid so the limit is a bin edge: every sample in a failing
/// bar breaks the limit and every sample in a neutral bar meets it.
#[test]
fn histogram_bins_put_the_limit_on_an_edge_and_keep_every_sample() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    let model = super::histogram::histogram_model(&results[0].samples, Some(40.0), None)
        .expect("forty samples bin");
    assert_eq!(model.counts.iter().sum::<usize>(), 40);
    assert!(
        model.edges.iter().any(|edge| (edge - 40.0).abs() < 1.0e-9),
        "the limit is an edge: {:?}",
        model.edges
    );
    let failing = model
        .counts
        .iter()
        .zip(&model.failing)
        .filter(|(_, failing)| **failing)
        .map(|(count, _)| count)
        .sum::<usize>();
    assert_eq!(failing, 3, "exactly the three samples under 40 dB");
    assert_eq!(model.limits, [40.0]);
    assert!(model.axis.0 <= model.mean - 3.0 * model.std_dev);
    assert!(model.axis.1 >= model.mean + 3.0 * model.std_dev);
    assert!((8..=48).contains(&model.counts.len()));
    assert!(model.ticks.len() >= 4);
    assert!(model.ticks.windows(2).all(|pair| pair[1] > pair[0]));
    assert_eq!(super::histogram::tick_text(39.5, 0.5, "dB"), "39.5 dB");
    assert!(super::histogram::histogram_model(&[f64::NAN], Some(1.0), None).is_none());
}

/// Bars are neutral and failing bars a muted error tone; nothing in the
/// histogram is painted in the accent.
#[test]
fn histogram_bars_are_neutral_and_failing_bars_muted_error() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    let model = super::histogram::histogram_model(&results[0].samples, Some(40.0), None)
        .expect("forty samples bin");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut fills = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 320.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    super::histogram::paint_histogram(ui, "gain", "dB", &model, 300.0);
                });
            },
        );
        fills = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(rect) => Some(rect.fill),
                _ => None,
            })
            .collect::<Vec<_>>();
    }
    let t = Tokens::get(&ctx);
    assert!(!fills.contains(&t.color.accent));
    assert!(fills.contains(&t.color.err.gamma_multiply(0.45)));
    assert!(fills.contains(&t.color.text_dim.gamma_multiply(0.38)));
}

/// The selected row's histogram sits in the pane under the table, not stacked
/// above it, and the table keeps the page's grammar.
#[test]
fn the_yield_page_draws_the_selected_histogram_under_the_table() {
    let geometry =
        super::raster::page_geometry(super::raster::monte_carlo_app(), VerificationPage::Yield);
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 1);
    let split = geometry
        .one("split")
        .expect("the detail pane is split off the table");
    assert_eq!(split.top(), rows[0].bottom());
    let histogram = geometry
        .one("histogram")
        .expect("the selected row's histogram");
    assert_eq!(histogram.top(), split.bottom());
    assert!(
        (220.0..=340.0).contains(&histogram.height()),
        "{histogram:?}"
    );
}

/// The page as a 2x display shows it, with the gain row's lower limit open
/// for entry, written to `RSPICE_REVIEW_PNG` (or the temporary directory) for
/// a person to look at, and measured: 24 pt head and row, a column rule
/// between every pair of columns, every number ending the same distance short
/// of its column's rule, the open field inside its own cell, and the failing
/// verdict in the error ink.
#[test]
#[ignore = "writes a PNG for review"]
fn the_yield_page_renders_with_a_limit_cell_open() {
    const SCALE: f32 = 2.0;
    let mut app = super::raster::monte_carlo_app();
    app.state.simulate.verification.yield_limit_draft = Some(YieldLimitDraft {
        output: "gain".to_owned(),
        bound: YieldLimitBound::Lower,
        text: "40 dB".to_owned(),
        seed: "40 dB".to_owned(),
        focus: true,
    });
    let (canvas, geometry) = super::raster::page_render(app, VerificationPage::Yield);
    let path = std::env::var_os("RSPICE_REVIEW_PNG").map_or_else(
        || std::env::temp_dir().join("rspice-verify-yield.png"),
        std::path::PathBuf::from,
    );
    std::fs::write(&path, canvas.png(canvas.height())).expect("the review PNG is written");

    let device = |points: f32| (points * SCALE).round() as usize;
    let head = geometry.one("table.head").expect("table head");
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 1);
    let row = rows[0];
    assert_eq!((head.height(), row.height()), (24.0, 24.0));
    assert_eq!(row.top(), head.bottom());

    // The column rules, read off the head's pixels above its words: the
    // centre of every run of ink across its fill, in points.
    let y = device(head.top() + 2.0);
    let fill = canvas
        .pixel(device(head.left() + 2.0), y)
        .expect("the head is on the canvas");
    let mut rules = Vec::new();
    let mut start = None;
    for x in device(head.left()) + 1..device(head.right()) - 1 {
        match (canvas.pixel(x, y) != Some(fill), start) {
            (true, None) => start = Some(x),
            (false, Some(first)) => {
                rules.push((first + x) as f32 / 2.0 / SCALE);
                start = None;
            }
            _ => {}
        }
    }
    assert_eq!(
        rules.len(),
        9,
        "a rule between each of ten columns: {rules:?}"
    );

    // Every number ends the same distance short of its column's rule.
    let mut insets = geometry
        .right_edges()
        .into_iter()
        .map(|(column, edges)| {
            assert_eq!(edges.len(), 1, "{column}: {edges:?}");
            let index = column
                .trim_start_matches("table.right.")
                .parse::<usize>()
                .expect("a column index");
            (column, rules[index] - edges[0])
        })
        .collect::<Vec<_>>();
    assert_eq!(
        insets.len(),
        7,
        "every number column but the open one: {insets:?}"
    );
    insets.sort_by(|left, right| left.1.total_cmp(&right.1));
    assert!(
        insets[insets.len() - 1].1 - insets[0].1 <= 1.0,
        "numbers end on one inset: {insets:?}"
    );

    // The open field sits inside the Lower cell, between its two rules.
    let field = geometry.one("table.edit").expect("the open cell's field");
    assert!(
        field.left() > rules[0]
            && field.right() < rules[1]
            && field.top() >= row.top()
            && field.bottom() <= row.bottom(),
        "{field:?} inside {}..{} of {row:?}",
        rules[0],
        rules[1]
    );

    // The verdict cell's words are the error ink, exactly.
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let err = Tokens::get(&ctx).color.err;
    let verdict = egui::Rect::from_min_max(
        egui::pos2((rules[8] + 1.0) * SCALE, (row.top() + 1.0) * SCALE),
        egui::pos2((row.right() - 1.0) * SCALE, (row.bottom() - 2.0) * SCALE),
    );
    let inked = canvas
        .pixels_in(verdict)
        .filter(|pixel| *pixel == err)
        .count();
    assert!(
        inked >= 8,
        "Fail is painted in the error ink: {inked} pixels"
    );
}
