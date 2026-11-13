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

/// A saved view keeps what its volts axis is drawn against, draws it so
/// when shown, keeps a change made inside it, and hands the quick view its
/// own choice back when it is left.
#[test]
fn a_saved_view_keeps_what_its_pane_is_drawn_against() {
    use super::super::super::persistent_document::{
        capture_pane_presentation, project_pane_presentation, projection,
    };
    use super::expression_units::{document_frame, save_single_pane_view};
    let mut state = lissajous_host();
    draw_against(&mut state, "V(a)");
    let pane_key = volts_pane(&mut state);
    let document_id = save_single_pane_view(&mut state);
    let kept = |state: &ResultsTestHost| {
        state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("the project keeps the document")
            .unit_abscissas()
            .iter()
            .map(|choice| (choice.unit.clone(), choice.abscissa.text().to_owned()))
            .collect::<Vec<_>>()
    };
    assert_eq!(kept(&state), [("V".to_owned(), "V(a)".to_owned())]);
    document_frame(&mut state, document_id);

    // The reader draws the view's pane against V(b) instead, inside the
    // pane's frame, between its projection and its capture.
    let pane = projection(state.results_reads(), document_id)
        .expect("the document projects")
        .pages[0]
        .panes[0]
        .clone();
    let viewer = crate::workbench::ResultViewer::from_viewer_document_id(&pane.viewer_id)
        .expect("the pane names a Results viewer");
    project_pane_presentation(&mut state.results_parts(), &pane, viewer)
        .expect("the pane projects onto its strip");
    let against_b = Abscissa::Signal {
        name: "V(b)".to_owned(),
    };
    state
        .results
        .results
        .session
        .unit_abscissas
        .insert(pane_key.clone(), UnitAbscissaChoice::new(against_b.clone()));
    capture_pane_presentation(&mut state.results_parts(), &pane, viewer);
    assert_eq!(kept(&state), [("V".to_owned(), "V(b)".to_owned())]);
    let reloaded: rspice_results::visualization_document::VisualizationDocument =
        serde_json::from_str(
            &serde_json::to_string(
                state
                    .model
                    .workspace
                    .content
                    .visualization_document(document_id)
                    .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(reloaded.unit_abscissas()[0].abscissa, against_b);

    state.results.results.session.leave_persistent_document();
    assert_eq!(
        state
            .results
            .results
            .session
            .unit_abscissas
            .get(&pane_key)
            .map(|choice| choice.abscissa.text()),
        Some("V(a)"),
        "the quick view has its choice back"
    );
    // Shown again, the view draws against V(b).
    document_frame(&mut state, document_id);
    assert_eq!(
        state
            .results
            .results
            .session
            .unit_abscissas
            .get(&pane_key)
            .map(|choice| &choice.abscissa),
        Some(&against_b)
    );
    // Back on the sweep inside the view, the view forgets the choice.
    project_pane_presentation(&mut state.results_parts(), &pane, viewer)
        .expect("the pane projects onto its strip");
    state
        .results
        .results
        .session
        .unit_abscissas
        .remove(&pane_key);
    capture_pane_presentation(&mut state.results_parts(), &pane, viewer);
    assert!(kept(&state).is_empty());
}

/// A host on a run the real controller solved, its first analysis active.
fn solved_host(deck: &str) -> ResultsTestHost {
    let run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch(deck);
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state
}

fn trace<'a>(model: &'a StripModel, name: &str) -> &'a rspice_results_ui::waves::StripTrace {
    model
        .traces
        .iter()
        .find(|trace| trace.source_waveform_name == name)
        .unwrap_or_else(|| panic!("the strip plots {name}"))
}

/// An NMOS output family, Id against Vds for three gate voltages, from a
/// nested DC sweep: each member's curve is that member's drain current
/// against its own drain voltage, sample for sample, and in saturation it
/// is the level-1 square law.
#[test]
fn a_mosfet_family_plots_each_members_drain_current_against_its_drain_voltage() {
    const KP: f64 = 100.0e-6;
    const W_OVER_L: f64 = 10.0;
    const VTO: f64 = 0.7;
    const LAMBDA: f64 = 0.02;
    let mut state = solved_host(
        "nmos output family\n\
         VD d 0 DC 0\n\
         VG g 0 DC 1\n\
         M1 d g 0 0 nch W=10u L=1u\n\
         .model nch nmos (level=1 vto=0.7 kp=100u lambda=0.02)\n\
         .dc VD 0 3 0.05 VG 1.5 2.5 0.5\n\
         .end\n",
    );
    state.results.results.session.viewer =
        rspice_results::result_presentation::ResultViewer::DcSweep;
    let models = super::lanes::strip(&mut state);
    let model = &models[0];
    let members = [("1.5", 1), ("2", 2), ("2.5", 3)];
    for (vgs, point) in members {
        let suffix = format!(" [VG={vgs}; point {point}]");
        let id = trace(model, &format!("I(VD){suffix}"));
        let vd = trace(model, &format!("V(D){suffix}"));
        let curve = rspice_results_ui::waves::xy::resample(
            (vd.x.as_slice(), vd.y.as_slice()),
            (id.x.as_slice(), id.y.as_slice()),
            crate::ui::plot::SampleInterpolation::Linear,
        )
        .expect("one sweep pairs");
        assert!(!curve.interpolated);
        assert_eq!(curve.x.len(), 61, "Vds from 0 to 3 V in 50 mV steps");
        for (k, (&x, &y)) in curve.x.iter().zip(&curve.y).enumerate() {
            assert_eq!(x.to_bits(), vd.y[k].to_bits());
            assert_eq!(y.to_bits(), id.y[k].to_bits());
            assert!((x - 0.05 * k as f64).abs() < 1.0e-9, "Vds[{k}] = {x}");
        }
        // Saturation at Vds = 3 V: Id = kp/2 · W/L · (Vgs − Vto)² · (1 + λ·Vds).
        let gate: f64 = vgs.parse().unwrap();
        let expected = 0.5 * KP * W_OVER_L * (gate - VTO).powi(2) * (1.0 + LAMBDA * 3.0);
        let solved = curve.y[60].abs();
        assert!(
            (solved - expected).abs() <= 1.0e-6 * expected,
            "Vgs = {gate}: Id(3 V) = {solved}, the square law gives {expected}"
        );
    }

    // The amperes pane drawn against the first member's drain voltage
    // draws one curve per member, each with its own arrows.
    let pane = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: "A".to_owned(),
    };
    let colors: Vec<egui::Color32> = members
        .iter()
        .map(|(vgs, point)| trace(model, &format!("I(VD) [VG={vgs}; point {point}]")).color)
        .collect();
    drop(models);
    state.results.results.session.unit_abscissas.insert(
        pane,
        UnitAbscissaChoice::new(Abscissa::Signal {
            name: "V(D) [VG=1.5; point 1]".to_owned(),
        }),
    );
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let shapes = frame(&ctx, &mut state, Vec::new());
    for color in colors {
        assert!(
            !arrowheads(&shapes, color).is_empty(),
            "every member is drawn with its arrows"
        );
    }
}

/// Vout against Vin from a slow transient ramp through a resistor into a
/// diode is the circuit's DC transfer curve: at every DC sweep point inside
/// the ramp the two agree within the transient's tolerance, reltol · |V| +
/// vntol.
#[test]
fn a_transient_ramp_drawn_against_its_input_is_the_dc_transfer_curve() {
    let circuit = "VIN in 0 PWL(0 0 1m 2)\n\
                   R1 in out 1k\n\
                   D1 out 0 dmod\n\
                   .model dmod d (is=1e-14 n=1)\n";
    let mut transient = solved_host(&format!("diode transfer\n{circuit}.tran 1u 1m\n.end\n"));
    let dc = solved_host(&format!(
        "diode transfer\n{}.dc VIN 0 2 0.01\n.end\n",
        circuit.replace("PWL(0 0 1m 2)", "DC 0")
    ));
    let dc_run = &dc.model.simulation.retained.runs[0].analyses[0];
    let dc_out = dc_run
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "V(OUT)")
        .expect("the sweep keeps V(OUT)");
    let models = super::lanes::strip(&mut transient);
    // The transient names its node columns bare; the DC sweep wraps them.
    let vin = trace(&models[0], "IN");
    let vout = trace(&models[0], "OUT");
    let curve = rspice_results_ui::waves::xy::resample(
        (vin.x.as_slice(), vin.y.as_slice()),
        (vout.x.as_slice(), vout.y.as_slice()),
        crate::ui::plot::SampleInterpolation::Linear,
    )
    .expect("one transient grid pairs");
    let mut compared = 0;
    for (&v_in, &v_out) in dc_out.x.iter().zip(dc_out.y.iter()) {
        if !(0.05..=1.95).contains(&v_in) {
            continue;
        }
        let drawn = rspice_results::sampling::sample_at_with(
            &curve.x,
            &curve.y,
            v_in,
            crate::ui::plot::SampleInterpolation::Linear,
        );
        let tolerance = 1.0e-3 * v_out.abs() + 1.0e-6;
        assert!(
            (drawn - v_out).abs() <= tolerance,
            "at Vin = {v_in}: the transient draws {drawn}, the DC sweep {v_out}"
        );
        compared += 1;
    }
    assert!(compared >= 180, "{compared} sweep points compared");

    // Drawn, the pane carries the curve's arrows.
    let color = vout.color;
    drop(models);
    let pane = volts_pane(&mut transient);
    transient.results.results.session.unit_abscissas.insert(
        pane,
        UnitAbscissaChoice::new(Abscissa::Signal {
            name: "IN".to_owned(),
        }),
    );
    let ctx = theme_context();
    frame(&ctx, &mut transient, Vec::new());
    assert!(!arrowheads(&frame(&ctx, &mut transient, Vec::new()), color).is_empty());
}
