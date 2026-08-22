//! Where this surface puts what it paints.
//!
//! `tests` judges what the studio says and `click_tests` what a press on it
//! does. This is the third question and the one neither can ask: two things
//! laid out by two separate measurements have to agree about the space
//! between them, and only their coordinates answer that.

use egui::{Rect, vec2};

use crate::workbench::RSpiceApp;

/// One painted line of the Analyses route: its text and where it landed.
#[derive(Clone, Debug)]
struct PaintedLine {
    text: String,
    left: f32,
    right: f32,
    top: f32,
}

/// Render the Analyses route at `width` and report every line it painted.
///
/// Positions, not just wording. The header's identity line and the
/// availability chip beside it are laid out by two separate measurements —
/// the chip is measured from the trailing inset, the line gets whatever is
/// left — and whether they agree is a question about coordinates that no
/// assertion over text alone can ask.
fn analyses_route_painted_lines(width: f32) -> Vec<PaintedLine> {
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
    let mut lines = Vec::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut lines);
    }
    lines
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
