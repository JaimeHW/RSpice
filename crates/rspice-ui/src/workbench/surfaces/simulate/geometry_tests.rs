//! Where this surface puts what it paints.
//!
//! `tests` judges what the studio says and `click_tests` what a press on it
//! does. This is the third question and the one neither can ask: two things
//! laid out by two separate measurements have to agree about the space
//! between them, and only their coordinates answer that.

use egui::{Rect, vec2};

use crate::simulation::plan::AnalysisKind;
use crate::workbench::RSpiceApp;

/// One painted line of the Analyses route: its text and where it landed.
#[derive(Clone, Debug)]
struct PaintedLine {
    text: String,
    left: f32,
    right: f32,
    top: f32,
}

/// Every line of text a rendered frame painted, and where each landed.
fn painted_lines(shapes: &[egui::epaint::ClippedShape]) -> Vec<PaintedLine> {
    fn collect(shape: &egui::epaint::Shape, out: &mut Vec<PaintedLine>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push(PaintedLine {
                text: text.galley.job.text.clone(),
                left: text.pos.x,
                right: text.pos.x + text.galley.size().x,
                top: text.pos.y,
            }),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }

    let mut lines = Vec::new();
    for clipped in shapes {
        collect(&clipped.shape, &mut lines);
    }
    lines
}

/// Render the Analyses route at `width` and report every line it painted.
///
/// Positions, not just wording. The header's identity line and the
/// availability chip beside it are laid out by two separate measurements —
/// the chip is measured from the trailing inset, the line gets whatever is
/// left — and whether they agree is a question about coordinates that no
/// assertion over text alone can ask.
fn analyses_route_painted_lines(width: f32) -> Vec<PaintedLine> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.simulation_page = crate::workbench::state::SimulationPage::Analyses;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 900.0))),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::show(ui, &mut app));
        },
    );
    painted_lines(&output.shapes)
}

/// The analysis header's identity line never reaches its availability chip.
///
/// The two are positioned by different measurements. The chip is laid out from
/// the header's trailing inset; the identity line — `<kind> · <id> · lifecycle
/// <state>` — is given whatever the chip leaves and used to be hard-clipped at
/// that boundary. A hard clip cuts a glyph in half and says nothing about what
/// was removed: the line read `transient · 4f2a1b3c-9d` and there was no way
/// to tell that the identity continued, or that a lifecycle state had been on
/// the end of it.
///
/// So the line is elided, and both halves of that are pinned at the width the
/// gate measures. At 1000 the full line fits and stops short of the chip; at
/// 950 it does not fit, ends in the ellipsis, and still stops short. The
/// mockup's own rule for this row is `overflow:hidden; text-overflow:ellipsis`.
#[test]
fn the_analysis_header_identity_elides_instead_of_reaching_its_availability_chip() {
    /// The header band: the chip and the identity line are the only two things
    /// painted in the top 120 points of the form.
    const HEADER_BAND: f32 = 120.0;

    let header_geometry = |width: f32| -> (PaintedLine, PaintedLine) {
        let lines = analyses_route_painted_lines(width);
        let header: Vec<_> = lines
            .iter()
            .filter(|line| line.top < HEADER_BAND)
            .cloned()
            .collect();
        let chip = header
            .iter()
            .find(|line| line.text == "Production")
            .cloned()
            .unwrap_or_else(|| {
                panic!("the header states the analysis's availability at {width}: {header:?}")
            });
        // Matched on the kind rather than the whole line, because the line is
        // exactly what elision shortens and the instance id is minted per run.
        let identity = header
            .iter()
            .find(|line| line.text.starts_with("Transient · "))
            .cloned()
            .unwrap_or_else(|| {
                panic!("the header states the instance's identity at {width}: {header:?}")
            });
        (identity, chip)
    };

    let (identity, chip) = header_geometry(1000.0);
    assert!(
        identity.text.ends_with("· lifecycle draft"),
        "at 1000 the identity line fits whole: {:?}",
        identity.text
    );
    assert!(
        identity.right < chip.left,
        "the identity line runs to {} and the chip starts at {}",
        identity.right,
        chip.left
    );

    let (identity, chip) = header_geometry(950.0);
    assert!(
        identity.text.ends_with('\u{2026}'),
        "a line that no longer fits has to say it was shortened: {:?}",
        identity.text
    );
    assert!(
        identity.right < chip.left,
        "the elided identity line runs to {} and the chip starts at {}",
        identity.right,
        chip.left
    );
}

/// What the analysis catalogue drew, gathered into the rows it drew them in.
///
/// Every analysis code the frame painted, grouped by the top it landed on:
/// two cells of one grid row share a row rect, so they share a top, and two
/// cells of one column never do. That is the whole of what "two columns"
/// means here, and it is a question only the coordinates answer — the same
/// rows, the same wording and the same reading order come out of a
/// one-column layout.
fn analysis_catalogue_code_rows(viewport_width: f32) -> Vec<Vec<PaintedLine>> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.sim_setup.palette_open = true;
    let mut run = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(viewport_width, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| super::show_workflow_dialogs(ctx, &mut app),
        )
    };
    // A content-height surface lays out against its previous measurement.
    let _ = run();
    let output = run();
    let codes = AnalysisKind::MANIFEST_ORDER.map(AnalysisKind::code);
    let mut codes_painted = painted_lines(&output.shapes)
        .into_iter()
        .filter(|line| codes.contains(&line.text.as_str()))
        .collect::<Vec<_>>();
    codes_painted.sort_by(|a, b| a.top.total_cmp(&b.top).then(a.left.total_cmp(&b.left)));
    let mut rows: Vec<Vec<PaintedLine>> = Vec::new();
    for line in codes_painted {
        match rows.last_mut() {
            Some(row) if (row[0].top - line.top).abs() < 0.5 => row.push(line),
            _ => rows.push(vec![line]),
        }
    }
    rows
}

/// The catalogue's second column is decided by the body it is drawn in.
///
/// `DialogSize::AnalysisCatalog` is a 1180-point surface inset from the
/// viewport, so the body is 26 points narrower than the window and the rows
/// give another 13 to a solid scrollbar. A rule written against the viewport
/// is wrong by that much, and at 1199 it was: a single column of 57-point
/// rows drawn down a body 1173 wide, with room for a second column standing
/// empty beside them.
///
/// Both directions are pinned, because "wider windows get two columns" is not
/// the claim. The claim is that the body's own width decides. With the
/// current chrome the layout turns over at a 1160-point window; 1199 is above
/// it and carries two columns that each clear
/// [`ANALYSIS_CATALOG_COLUMN_MIN_WIDTH`], and 1100 is below it and carries
/// one.
#[test]
fn the_analysis_catalogue_takes_its_columns_from_its_body_not_the_viewport() {
    let wide = analysis_catalogue_code_rows(1_199.0);
    assert!(
        wide.len() > 8,
        "the catalogue drew its rows at 1199: {} row(s)",
        wide.len()
    );
    assert!(
        wide.iter().all(|row| row.len() <= 2),
        "no row holds more cells than the layout has columns: {wide:?}"
    );
    let pair = wide
        .iter()
        .find(|row| row.len() == 2)
        .unwrap_or_else(|| panic!("an 1199-point viewport pairs its rows: {wide:?}"));
    let column_width = pair[1].left - pair[0].left;
    assert!(
        column_width > super::ANALYSIS_CATALOG_COLUMN_MIN_WIDTH,
        "each of the two columns clears the row's own minimum: {column_width}"
    );

    let narrow = analysis_catalogue_code_rows(1_100.0);
    assert!(
        narrow.len() > 8,
        "the catalogue drew its rows at 1100: {} row(s)",
        narrow.len()
    );
    assert!(
        narrow.iter().all(|row| row.len() == 1),
        "a body that cannot carry two columns of that minimum draws one: {narrow:?}"
    );
}
