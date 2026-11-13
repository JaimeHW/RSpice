//! A unit pane drawn against another signal: the pane's menu chooses it,
//! arrows mark the sweep's direction, and a cursor on the sweep reads the
//! instant and both coordinates where the curve is then.

use super::conversion::{click, find, texts};
use super::lanes::{frame, theme_context};
use super::*;
use egui::epaint::ClippedShape;
use rspice_results::visualization_document::Abscissa;
use rspice_results_ui::session::UnitAbscissaChoice;
use std::f64::consts::PI;

const OMEGA: f64 = 2.0 * PI * 1.0e3;
const PHI: f64 = PI / 3.0;

/// A 1 kHz transient over one period with V(b) leading V(a) by 60°.
fn lissajous_host() -> ResultsTestHost {
    let time: Vec<f64> = (0..=400).map(|step| f64::from(step) * 2.5e-6).collect();
    let a: Vec<f64> = time.iter().map(|t| (OMEGA * t).sin()).collect();
    let b: Vec<f64> = time.iter().map(|t| (OMEGA * t + PHI).sin()).collect();
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(a)", time.clone(), a, "#fff").with_unit("V"),
            RetainedTrace::new("V(b)", time, b, "#fff").with_unit("V"),
        ]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy provenance");
    run.restore_lifecycle(crate::state::SimulationRunLifecycle::LegacyUnknown, 0.0)
        .expect("legacy lifecycle");
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state
}

fn volts_pane(state: &mut ResultsTestHost) -> WavePanePresentationKey {
    let models = super::lanes::strip(state);
    WavePanePresentationKey {
        analysis: models[0].analysis_key,
        unit: "V".to_owned(),
    }
}

fn draw_against(state: &mut ResultsTestHost, signal: &str) {
    let pane = volts_pane(state);
    state.results.results.session.unit_abscissas.insert(
        pane,
        UnitAbscissaChoice::new(Abscissa::Signal {
            name: signal.to_owned(),
        }),
    );
}

/// The filled triangles a frame painted in `color`: the arrowheads.
fn arrowheads(shapes: &[ClippedShape], color: egui::Color32) -> Vec<Vec<egui::Pos2>> {
    fn walk(shape: &egui::Shape, color: egui::Color32, out: &mut Vec<Vec<egui::Pos2>>) {
        match shape {
            egui::Shape::Path(path)
                if path.closed && path.points.len() == 3 && path.fill == color =>
            {
                out.push(path.points.clone());
            }
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, color, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, color, &mut out);
    }
    out
}

/// Where the frame painted the volts pane's unit tag: the topmost "V".
fn volts_unit_tag(ctx: &egui::Context, state: &mut ResultsTestHost) -> egui::Pos2 {
    texts(&frame(ctx, state, Vec::new()))
        .into_iter()
        .filter(|(text, _)| text == "V")
        .min_by(|(_, a), (_, b)| a.top().total_cmp(&b.top()))
        .map(|(_, rect)| rect.center())
        .expect("the volts pane's header names its unit")
}

fn painted_texts(ctx: &egui::Context, state: &mut ResultsTestHost) -> Vec<(String, egui::Rect)> {
    frame(ctx, state, Vec::new());
    texts(&frame(ctx, state, Vec::new()))
}

/// A pane drawn against its sweep shows nothing new in its header. Its
/// context menu draws it against V(a); the header then tags it "vs V(a)"
/// and its X axis says how the two were paired. That axis label chooses
/// again, and the close button beside the tag draws the pane against time.
#[test]
fn the_panes_context_menu_and_x_axis_choose_what_it_is_drawn_against() {
    let mut state = lissajous_host();
    let ctx = theme_context();
    let painted = painted_texts(&ctx, &mut state);
    assert!(
        !painted.iter().any(|(text, _)| text.starts_with("vs ")),
        "a pane drawn against time has no tag: {painted:?}"
    );

    let unit = volts_unit_tag(&ctx, &mut state);
    click(&ctx, &mut state, unit, egui::PointerButton::Secondary);
    assert!(
        texts(&frame(&ctx, &mut state, Vec::new()))
            .iter()
            .any(|(text, _)| text == "Draw the V pane against"),
        "the header's context menu offers what the pane is drawn against"
    );
    let item = find(&ctx, &mut state, "V(a)");
    click(&ctx, &mut state, item, egui::PointerButton::Primary);
    let pane = volts_pane(&mut state);
    let drawn_against = |state: &ResultsTestHost| {
        state
            .results
            .results
            .session
            .unit_abscissas
            .get(&pane)
            .map(|choice| choice.abscissa.clone())
    };
    assert_eq!(
        drawn_against(&state),
        Some(Abscissa::Signal {
            name: "V(a)".to_owned()
        })
    );
    let painted = painted_texts(&ctx, &mut state);
    assert!(
        painted.iter().any(|(text, _)| text == "vs V(a)"),
        "{painted:?}"
    );
    let x_label = painted
        .iter()
        .find(|(text, _)| text.starts_with("V(a) \u{b7} at each t sample"))
        .map(|(_, rect)| rect.center())
        .unwrap_or_else(|| panic!("the X axis names the abscissa and the pairing: {painted:?}"));

    // The X axis label offers the same choices.
    click(&ctx, &mut state, x_label, egui::PointerButton::Primary);
    let item = find(&ctx, &mut state, "V(b)");
    click(&ctx, &mut state, item, egui::PointerButton::Primary);
    assert_eq!(
        drawn_against(&state),
        Some(Abscissa::Signal {
            name: "V(b)".to_owned()
        })
    );

    // One click beside the tag draws the pane against time again. The
    // button sits just past the tag chip's right edge (its text plus 9 px
    // of padding), one item gap and half a button on.
    let tag = painted_texts(&ctx, &mut state)
        .into_iter()
        .find(|(text, _)| text == "vs V(b)")
        .map(|(_, rect)| rect)
        .expect("the header tags what the pane is drawn against");
    let back = egui::pos2(tag.right() + 9.0 + 13.0, tag.center().y);
    click(&ctx, &mut state, back, egui::PointerButton::Primary);
    assert_eq!(drawn_against(&state), None);
    let painted = painted_texts(&ctx, &mut state);
    assert!(
        !painted.iter().any(|(text, _)| text.starts_with("vs ")),
        "{painted:?}"
    );
}

/// A Lissajous figure carries sparse arrowheads in its own colour, all on
/// the plot, and its reverse traversal points them the other way.
#[test]
fn a_lissajous_figure_carries_arrows_the_way_time_runs() {
    let mut state = lissajous_host();
    draw_against(&mut state, "V(a)");
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let shapes = frame(&ctx, &mut state, Vec::new());
    let color = super::lanes::strip(&mut state)[0]
        .traces
        .iter()
        .find(|trace| trace.name == "V(b)")
        .expect("V(b) is plotted")
        .color;
    let arrows = arrowheads(&shapes, color);
    assert!(
        (3..=24).contains(&arrows.len()),
        "an ellipse a few hundred points round carries a few arrows: {}",
        arrows.len()
    );
    // V(b) leads V(a), so the figure is travelled clockwise, as x = sin t,
    // y = cos t is: from (0, 0.87) at t = 0 both rise. On a screen whose Y
    // grows downward a clockwise turn has a positive cross product, and the
    // tip of every arrow turns about the figure's centre that way.
    let centre = arrows
        .iter()
        .flatten()
        .fold(egui::Vec2::ZERO, |sum, point| sum + point.to_vec2())
        / (arrows.len() * 3) as f32;
    for arrow in &arrows {
        let base = (arrow[1].to_vec2() + arrow[2].to_vec2()) * 0.5;
        let forward = arrow[0].to_vec2() - base;
        let radius = base - centre;
        let turn = radius.x * forward.y - radius.y * forward.x;
        assert!(turn > 0.0, "an arrow at {base:?} turns the wrong way");
    }
}

/// Cursor A at a quarter period stands where sin(ωt) = 1: its tag reads the
/// instant, V(a) = 1 V and V(b) = sin(π/2 + π/3) = 500 mV, the closed form.
#[test]
fn a_cursor_on_the_figure_reads_the_instant_and_both_coordinates() {
    let mut state = lissajous_host();
    draw_against(&mut state, "V(a)");
    super::lanes::place_cursors(&mut state, Some(0.25e-3), Some(0.5e-3));
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let painted: Vec<String> = texts(&frame(&ctx, &mut state, Vec::new()))
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    assert!(
        painted
            .iter()
            .any(|text| text.starts_with("A \u{b7} t 250 ")
                && text.ends_with("s \u{b7} V(a) 1 V \u{b7} 500 mV")),
        "{painted:?}"
    );
    // At half a period sin(ωt) is zero to the last bit f64 holds, and
    // sin(π + π/3) = −√3/2 = −0.8660254.
    assert!(
        painted
            .iter()
            .any(|text| text.starts_with("B \u{b7} t 500 ")
                && text.contains("s \u{b7} V(a) ")
                && text.ends_with(" \u{b7} -866.0254 mV")),
        "{painted:?}"
    );
}

/// The quick view's choice is the project's: it is saved with the result
/// presentation and drawn again when the project is reopened.
#[test]
fn the_project_keeps_what_a_pane_is_drawn_against() {
    let mut state = lissajous_host();
    draw_against(&mut state, "V(a)");
    let pane = volts_pane(&mut state);
    let saved = state
        .results
        .results
        .project_presentation(&state.model.simulation);
    assert_eq!(
        saved.unit_abscissas,
        [rspice_results::result_presentation::StripUnitAbscissa {
            analysis: pane.analysis,
            unit: "V".to_owned(),
            abscissa: Abscissa::Signal {
                name: "V(a)".to_owned()
            },
        }]
    );
    state.results.results.session.unit_abscissas.clear();
    crate::workbench::documents::result_document::restore_presentation(
        &mut state.results_parts(),
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap(),
    );
    assert_eq!(
        state
            .results
            .results
            .session
            .unit_abscissas
            .get(&pane)
            .map(|choice| &choice.abscissa),
        Some(&Abscissa::Signal {
            name: "V(a)".to_owned()
        })
    );
}
