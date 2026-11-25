//! The sensitivity sheet's tables sort, filter and copy as every sheet's do,
//! and its words read as the other sheets' do.

use super::mismatch::{self, ActiveMismatch, MismatchPlan, MismatchView};
use super::study::{self, ActiveStudy, StudyPlan, StudyView};
use super::*;

fn rows(named: &[(&str, f64)]) -> Vec<SensitivityResultRow> {
    named
        .iter()
        .map(|(parameter, normalized)| SensitivityResultRow {
            parameter: (*parameter).to_owned(),
            raw: (normalized * 10.0).into(),
            normalized: (*normalized).into(),
        })
        .collect()
}

/// Every run of text a frame painted, with where it was laid out.
fn painted(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

fn frame(
    ctx: &egui::Context,
    events: Vec<egui::Event>,
    mut draw: impl FnMut(&mut Ui),
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 700.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| draw(ui));
        },
    )
}

fn press(at: egui::Pos2, button: egui::PointerButton, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: at,
        button,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

/// Press and let go of `button` at `at`, one frame each, as a hand does.
fn click(
    ctx: &egui::Context,
    at: egui::Pos2,
    button: egui::PointerButton,
    mut draw: impl FnMut(&mut Ui),
) -> egui::FullOutput {
    frame(ctx, vec![egui::Event::PointerMoved(at)], &mut draw);
    frame(ctx, vec![press(at, button, true)], &mut draw);
    frame(ctx, vec![press(at, button, false)], &mut draw)
}

fn place(output: &egui::FullOutput, text: &str) -> egui::Pos2 {
    painted(output)
        .into_iter()
        .rfind(|(painted, _)| painted == text)
        .map(|(_, rect)| rect.center())
        .unwrap_or_else(|| panic!("{text:?} is not drawn"))
}

/// The named rows a frame painted, top to bottom.
fn listed_order<'a>(output: &egui::FullOutput, names: &[&'a str]) -> Vec<&'a str> {
    let mut found: Vec<(f32, &str)> = painted(output)
        .into_iter()
        .filter_map(|(text, rect)| {
            names
                .iter()
                .find(|name| **name == text)
                .map(|name| (rect.center().y, *name))
        })
        .collect();
    found.sort_by(|left, right| left.0.total_cmp(&right.0));
    found.into_iter().map(|(_, name)| name).collect()
}

fn copied(output: egui::FullOutput) -> Option<String> {
    output
        .platform_output
        .commands
        .into_iter()
        .find_map(|command| match command {
            egui::OutputCommand::CopyText(text) => Some(text),
            _ => None,
        })
}

/// A filter narrows the ranking to the parameters it names, ignoring case,
/// and each keeps its place in the whole ranking and the whole scale.
#[test]
fn a_filter_lists_the_parameters_it_names_at_their_places_in_the_whole_ranking() {
    let rows = rows(&[("r1", 0.5), ("m1.vth", -2.0), ("r2", 1.0), ("M1.u0", 0.25)]);
    let plan = SensitivityPlan::new(&rows, "m1");
    assert_eq!(plan.order(), [1, 3]);
    assert_eq!(plan.ranks(), [0, 3]);
    assert_eq!(plan.total(), 4);
    assert_eq!(plan.max_magnitude(), 2.0);
    assert_eq!(
        listed_count(plan.order().len(), plan.total(), "parameters"),
        "2 of 4 parameters"
    );
    let every = SensitivityPlan::new(&rows, "  ");
    assert_eq!(every.order(), [1, 2, 0, 3]);
    assert_eq!(listed_count(4, 4, "parameters"), "4 parameters");
}

/// The panel's table opens ranked by magnitude; pressing a column's head
/// sorts by that column's signed values; a row's menu copies the row with
/// every digit the run holds, at its place in the ranking.
#[test]
fn the_ranked_table_sorts_by_a_column_and_copies_a_row_exactly() {
    let rows = rows(&[
        ("r1", 0.5),
        ("m1.vth", -2.0),
        ("r2", 1.0),
        ("m1.u0", 0.123_456_789_012),
    ]);
    let names = ["r1", "m1.vth", "r2", "m1.u0"];
    let plan = SensitivityPlan::new(&rows, "");
    let policy = QuantityPresentationPolicy::default();
    let view = SensitivityView {
        analysis_label: "SENS",
        output: "V(out)",
        result_mode: SensitivityResultMode::Dc,
        rows: &rows,
        quantity_policy: policy,
    };
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let mut draw = |ui: &mut Ui| right_panel(ui, ActiveSensitivity::Ready(view), Some(&plan), 4);
    frame(&ctx, Vec::new(), &mut draw);
    let ranked = frame(&ctx, Vec::new(), &mut draw);
    assert_eq!(
        listed_order(&ranked, &names),
        ["m1.vth", "r2", "r1", "m1.u0"]
    );

    let head = place(&ranked, "Normalized");
    click(&ctx, head, egui::PointerButton::Primary, &mut draw);
    let sorted = frame(&ctx, Vec::new(), &mut draw);
    assert_eq!(
        listed_order(&sorted, &names),
        ["m1.vth", "m1.u0", "r1", "r2"],
        "ascending by the signed normalized sensitivity"
    );

    let row = place(&sorted, "m1.u0");
    click(&ctx, row, egui::PointerButton::Secondary, &mut draw);
    let menu = frame(&ctx, Vec::new(), &mut draw);
    let item = place(&menu, "Copy exact row");
    let copied = copied(click(&ctx, item, egui::PointerButton::Primary, &mut draw))
        .expect("Copy exact row puts the row on the clipboard");
    let exact = |value: f64| policy.copy_si_value(value, "").trim_end().to_owned();
    assert_eq!(
        copied,
        format!(
            "4\tm1.u0\t{}\t{}",
            exact(0.123_456_789_012),
            exact(1.234_567_890_12)
        )
    );
}

/// A filtered panel lists only the rows the filter keeps, each at its place
/// in the whole ranking, and says so when it keeps none.
#[test]
fn the_filtered_panel_keeps_each_rows_rank() {
    let rows = rows(&[("r1", 0.5), ("m1.vth", -2.0), ("r2", 1.0), ("m1.u0", 0.25)]);
    let view = SensitivityView {
        analysis_label: "SENS",
        output: "V(out)",
        result_mode: SensitivityResultMode::Dc,
        rows: &rows,
        quantity_policy: QuantityPresentationPolicy::default(),
    };
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let plan = SensitivityPlan::new(&rows, "R");
    let mut draw = |ui: &mut Ui| right_panel(ui, ActiveSensitivity::Ready(view), Some(&plan), 4);
    frame(&ctx, Vec::new(), &mut draw);
    let filtered = frame(&ctx, Vec::new(), &mut draw);
    assert_eq!(
        listed_order(&filtered, &["r1", "m1.vth", "r2", "m1.u0"]),
        ["r2", "r1"]
    );
    let texts: Vec<String> = painted(&filtered)
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    // r2 ranks second and r1 third in the whole list.
    assert_eq!(
        listed_order(&filtered, &["2", "3"]),
        ["2", "3"],
        "{texts:?}"
    );

    let none = SensitivityPlan::new(&rows, "c1");
    let mut draw = |ui: &mut Ui| right_panel(ui, ActiveSensitivity::Ready(view), Some(&none), 4);
    let empty = frame(&ctx, Vec::new(), &mut draw);
    place(&empty, "No parameter matches the filter.");
}

/// The study sheet's title separates its parts as every sheet's does, and
/// a filter states how many of the variables it lists.
#[test]
fn the_study_title_reads_with_the_sheets_separators() {
    let evidence = study::swept_evidence();
    let view = StudyView {
        analysis_label: "SENS",
        evidence: &evidence,
        quantity_policy: QuantityPresentationPolicy::default(),
    };
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    for (filter, count) in [("", "2 variables"), ("r1", "1 of 2 variables")] {
        let plan = StudyPlan::new(&evidence, 0, filter);
        let drawn = frame(&ctx, Vec::new(), |ui| {
            study::show(ui, ActiveStudy::Ready(view), Some(&plan), 4);
        });
        let texts: Vec<String> = painted(&drawn).into_iter().map(|(text, _)| text).collect();
        assert!(
            texts
                .iter()
                .any(|text| text.starts_with("V(OUT) \u{b7} AC sweep") && text.ends_with(count)),
            "{texts:?}"
        );
        assert!(!texts.iter().any(|text| text.contains(" - ")), "{texts:?}");
    }
}

/// The mismatch sheet spells a standard deviation σ wherever it names one,
/// and its table lists the contributors a filter keeps by instance or
/// parameter.
#[test]
fn the_mismatch_panel_spells_sigma_and_filters_by_instance() {
    let evidence = mismatch::evidence_fixture();
    let view = MismatchView {
        analysis_label: "DCMATCH",
        evidence: &evidence,
        significant_digits: 4,
        quantity_policy: QuantityPresentationPolicy::default(),
    };
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let plan = MismatchPlan::new(&evidence, "r2");
    let mut draw = |ui: &mut Ui| {
        mismatch::right_panel(ui, ActiveMismatch::Ready(view), Some(&plan));
    };
    frame(&ctx, Vec::new(), &mut draw);
    let drawn = frame(&ctx, Vec::new(), &mut draw);
    let texts: Vec<String> = painted(&drawn).into_iter().map(|(text, _)| text).collect();
    for wanted in ["\u{3c3} total", "\u{3c3} mismatch", "\u{3c3} param", "R2V"] {
        assert!(
            texts.iter().any(|text| text == wanted),
            "{wanted:?} in {texts:?}"
        );
    }
    assert!(
        !texts
            .iter()
            .any(|text| text.to_lowercase().contains("sigma")),
        "{texts:?}"
    );
    assert!(!texts.iter().any(|text| text == "R1V"), "{texts:?}");
}
