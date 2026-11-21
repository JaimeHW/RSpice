//! The strip's chrome on a real mixed run, judged through the frame the
//! reader sees: a pane with nothing plotted, the hidden-trace control, the
//! legend's values and the Monte Carlo envelope.

use super::lanes::{FRAME, review_strip, strip, well};
use super::*;

/// A context wearing `theme`, laid out the way the raster harness lays out.
fn themed_context(theme: crate::ui::Theme) -> egui::Context {
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    theme.apply(&ctx);
    ctx
}

/// One frame of the Waves sheet in its document well.
fn well_frame(
    ctx: &egui::Context,
    state: &mut ResultsTestHost,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
        events,
        ..Default::default()
    };
    ctx.run_ui(input, |ui| {
        well(ui);
        show(ui, &mut state.results_parts());
    })
}

/// A settled frame: fonts and sizing take a pass each.
fn settled(ctx: &egui::Context, state: &mut ResultsTestHost) -> egui::FullOutput {
    well_frame(ctx, state, Vec::new());
    well_frame(ctx, state, Vec::new());
    well_frame(ctx, state, Vec::new())
}

/// Every run of text the frame painted, with where it was laid out.
fn texts(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
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

fn text_rect(output: &egui::FullOutput, wanted: &str) -> Option<egui::Rect> {
    texts(output)
        .into_iter()
        .find(|(text, _)| text == wanted)
        .map(|(_, rect)| rect)
}

fn click(ctx: &egui::Context, state: &mut ResultsTestHost, at: egui::Pos2) {
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    well_frame(ctx, state, vec![egui::Event::PointerMoved(at)]);
    well_frame(ctx, state, vec![button(true)]);
    well_frame(ctx, state, vec![button(false)]);
}

/// The review's first current: its analysis and column, its name and its
/// strip's identity.
fn first_current(state: &mut ResultsTestHost) -> (usize, usize, String, AnalysisPresentationKey) {
    let models = strip(state);
    let model = &models[0];
    let trace = model
        .traces
        .iter()
        .find(|trace| trace.source_waveform_name.starts_with("I("))
        .expect("the review keeps its currents");
    (
        model.analysis_index(),
        trace.waveform_index,
        trace.source_waveform_name.clone(),
        model.analysis_key,
    )
}

/// The units of the panes the review strip draws, in order.
fn pane_units(state: &mut ResultsTestHost) -> Vec<String> {
    strip(state)[0]
        .unit_panes()
        .iter()
        .map(|pane| pane.unit.to_owned())
        .collect()
}

/// A pane of signals the dataset only hides by default, which nobody has
/// plotted, is not drawn: the review's currents open no "A" pane, as a
/// waveform viewer shows no strip for a signal until it is plotted. Ticked in
/// the signal browser, a current opens its pane; unticked there, it leaves
/// none behind; hidden from its pane's legend, it takes the pane with it.
#[test]
fn a_pane_only_the_default_hides_is_not_drawn() {
    use crate::workbench::documents::result_document::browser_plot::set_result_signal_plotted;
    use rspice_results::result_presentation::SourceWaveformPresentationKey;
    let ctx = themed_context(crate::ui::Theme::default());
    let mut state = review_strip(false);
    assert_eq!(pane_units(&mut state), ["V"]);
    let output = settled(&ctx, &mut state);
    let offered = texts(&output)
        .into_iter()
        .filter(|(text, _)| text.ends_with(" hidden"))
        .count();
    assert_eq!(offered, 1, "only the voltage pane offers hidden signals");

    let (analysis, waveform, name, strip_key) = first_current(&mut state);
    let key = SourceWaveformPresentationKey::new(strip_key, name);
    set_result_signal_plotted(&mut state.results_parts(), &key, true);
    assert_eq!(
        pane_units(&mut state),
        ["V", "A"],
        "a ticked current opens its pane"
    );
    set_result_signal_plotted(&mut state.results_parts(), &key, false);
    assert_eq!(
        pane_units(&mut state),
        ["V"],
        "unticked in the browser, the current leaves no pane behind"
    );
    set_result_signal_plotted(&mut state.results_parts(), &key, true);
    toggle_visibility(&mut state.results_parts(), analysis, waveform);
    assert_eq!(
        pane_units(&mut state),
        ["V"],
        "hidden from its legend, the current takes its pane with it"
    );

    // With nothing plotted at all, the strip says where signals are plotted
    // from.
    let mut state = review_strip(false);
    for analysis in &mut state.model.simulation.retained.runs[0].analyses {
        for waveform in &mut analysis.waveforms {
            waveform.visible = false;
        }
    }
    state.model.simulation.view.data_version =
        state.model.simulation.view.data_version.wrapping_add(1);
    assert!(pane_units(&mut state).is_empty());
    let output = settled(&ctx, &mut state);
    assert!(
        text_rect(
            &output,
            "Nothing plotted — tick a signal in the Data browser"
        )
        .is_some(),
        "an empty strip points at the signal browser: {:?}",
        texts(&output)
            .into_iter()
            .map(|(text, _)| text)
            .collect::<Vec<_>>()
    );
}

/// The legend lists what is plotted; the hidden traces wait behind one
/// control that counts them, and ticking one there plots it again.
#[test]
fn hidden_traces_are_collected_and_can_be_shown_again() {
    let ctx = themed_context(crate::ui::Theme::default());
    let mut state = review_strip(true);
    let output = settled(&ctx, &mut state);
    assert!(
        text_rect(&output, "B4").is_none(),
        "a hidden trace is not a legend chip"
    );
    let control = text_rect(&output, "3 hidden").expect("the hidden traces are counted");
    click(&ctx, &mut state, control.center());
    let menu = settled(&ctx, &mut state);
    let entry = text_rect(&menu, "B4").expect("the control lists the hidden traces");
    click(&ctx, &mut state, entry.center());
    let models = strip(&mut state);
    assert!(
        models[0]
            .traces
            .iter()
            .any(|trace| trace.name == "B4" && trace.visible),
        "ticking a hidden trace plots it"
    );
}

/// Hold `node` at `value` through the whole run.
fn hold(state: &mut ResultsTestHost, node: &str, value: f64) {
    for analysis in &mut state.model.simulation.retained.runs[0].analyses {
        for waveform in &mut analysis.waveforms {
            if waveform.name == node {
                waveform.y = vec![value; waveform.y.len()].into();
            }
        }
    }
    state.model.simulation.view.data_version =
        state.model.simulation.view.data_version.wrapping_add(1);
}

/// A trace far below atto or far above tera states its value at cursor A as
/// Cadence's suffix notation does: under the next SI prefix through yocto
/// and yotta, and past them in engineering notation with the unit. Never a
/// run of leading zeros under the last prefix, as TOP's `0.02402879 aV`
/// once read.
#[test]
fn the_legend_states_values_past_the_ends_of_the_prefix_ladder() {
    let ctx = themed_context(crate::ui::Theme::default());
    for (top, vin, top_text, vin_text) in [
        (2.402_879e-20, 7.5e24, "24.02879 zV", "7.5 YV"),
        (3.1e-23, 4.2e15, "31 yV", "4.2 PV"),
        (2.402_879e-26, 4.56e31, "24.02879e\u{2212}27 V", "45.6e30 V"),
    ] {
        let mut state = review_strip(false);
        hold(&mut state, "TOP", top);
        hold(&mut state, "VIN", vin);
        let output = settled(&ctx, &mut state);
        let painted = texts(&output);
        for (node, wanted) in [("TOP", top_text), ("VIN", vin_text)] {
            assert!(
                painted
                    .iter()
                    .any(|(text, _)| text.starts_with(node) && text.ends_with(wanted)),
                "{node}'s legend chip does not read {wanted}: {:?}",
                painted
                    .iter()
                    .filter(|(text, _)| text.starts_with(node))
                    .collect::<Vec<_>>()
            );
        }
        assert!(
            !painted
                .iter()
                .any(|(text, _)| text.contains(" 0.0") && text.ends_with('V')),
            "a value is parked on the last prefix: {painted:?}"
        );
    }
}

/// Three Monte Carlo trials of a flat output, each on a time grid of its own
/// that shares no instant with the others', as adaptive steps leave them.
fn three_trials() -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    for (index, (time, level)) in [
        (vec![0.0, 0.31, 0.72, 1.0], 0.2),
        (vec![0.01, 0.45, 0.99], 0.5),
        (vec![0.02, 0.55, 0.98], 0.8),
    ]
    .into_iter()
    .enumerate()
    {
        let provenance = AnalysisResultProvenance::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([3; 32]),
            Vec::new(),
        )
        .and_then(|provenance| {
            provenance.with_family_member(Some(
                rspice_results::provenance::AnalysisResultFamilyMember::MonteCarloTrial { index },
            ))
        })
        .expect("a trial's provenance");
        let levels = vec![level; time.len()];
        let mut trace = RetainedTrace::new("V(out)", time, levels, "").with_unit("V");
        trace.visible = true;
        run.add_analysis(
            AnalysisResult::new(index as u64 + 1, AnalysisType::Transient, "Transient")
                .with_waveforms(vec![trace])
                .with_provenance(provenance),
        );
    }
    state
}

/// The trial strokes and the envelope fill a frame painted in `hue`: each
/// long stroke's width and whether it is at full strength, and how many
/// vertices of fill the envelope took.
fn trial_strokes(output: &egui::FullOutput, hue: egui::Color32) -> (Vec<(f32, bool)>, usize) {
    fn walk(
        shape: &egui::Shape,
        hue: egui::Color32,
        strokes: &mut Vec<(f32, bool)>,
        fill: &mut usize,
    ) {
        match shape {
            egui::Shape::Path(path) if path.points.len() >= 2 => {
                let span = path.points.last().map_or(0.0, |last| last.x) - path.points[0].x;
                if let egui::epaint::ColorMode::Solid(color) = path.stroke.color
                    && span > 300.0
                {
                    if color == hue {
                        strokes.push((path.stroke.width, true));
                    } else if color == hue.gamma_multiply(0.40) {
                        strokes.push((path.stroke.width, false));
                    }
                }
            }
            egui::Shape::Mesh(mesh) => {
                *fill += mesh
                    .vertices
                    .iter()
                    .filter(|vertex| vertex.color == hue.gamma_multiply(0.28))
                    .count();
            }
            egui::Shape::Vec(inner) => inner
                .iter()
                .for_each(|shape| walk(shape, hue, strokes, fill)),
            _ => {}
        }
    }
    let (mut strokes, mut fill) = (Vec::new(), 0);
    for clipped in &output.shapes {
        walk(&clipped.shape, hue, &mut strokes, &mut fill);
    }
    (strokes, fill)
}

/// Trials that never share an instant still have an envelope: the band they
/// occupy, filled across the plot, where an envelope read only at shared
/// instants had nothing to draw.
#[test]
fn trials_on_their_own_grids_have_a_continuous_envelope() {
    let mut state = three_trials();
    state.results.results.session.show_family_envelope = true;
    let ctx = themed_context(crate::ui::Theme::default());
    let output = settled(&ctx, &mut state);
    let hue = state.results.results.models.last_built().expect("models")[0].traces[0].signal_color;
    let (strokes, fill) = trial_strokes(&output, hue);
    assert_eq!(
        strokes.len(),
        5,
        "each trial is stroked, and the envelope's two edges: {strokes:?}"
    );
    // Two fill vertices a screen column, across a plot of some 1100 columns.
    assert!(
        fill > 2 * 1000,
        "the envelope fills the plot: {fill} vertices"
    );
}
