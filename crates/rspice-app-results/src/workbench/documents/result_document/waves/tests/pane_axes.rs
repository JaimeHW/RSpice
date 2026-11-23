//! A pane's chrome and its Y axis, judged through the frame the reader sees:
//! a click anywhere on a pane makes it the active one, the trace list draws
//! a hidden trace, a Bode strip names each trace in the pane that draws it,
//! and the automatic Y fit follows the X window until the reader pins a Y of
//! their own.

use super::lanes::{FRAME, verticals, well};
use super::*;

/// Two milliseconds of a core driven through its loop: the field H in A/m,
/// which runs 0 to 400 between 1 and 1.5 ms and −400 to 400 over the whole
/// record, and the flux density B in T, which saturates.
fn bh_strip() -> ResultsTestHost {
    let time: Vec<f64> = (0..=2_000).map(|index| f64::from(index) * 1.0e-6).collect();
    let field: Vec<f64> = time
        .iter()
        .map(|t| -400.0 * (2.0 * std::f64::consts::PI * 500.0 * t).sin())
        .collect();
    let flux: Vec<f64> = field.iter().map(|h| 1.2 * (h / 150.0).tanh()).collect();
    transient(vec![
        RetainedTrace::new("H(core)", time.clone(), field, "#fff").with_unit("A/m"),
        RetainedTrace::new("B(core)", time, flux, "#fff").with_unit("T"),
    ])
}

/// One transient of `waveforms`, the active run.
fn transient(waveforms: Vec<RetainedTrace>) -> ResultsTestHost {
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(waveforms),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state
}

fn context(theme: crate::ui::Theme) -> egui::Context {
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    theme.apply(&ctx);
    ctx.enable_accesskit();
    ctx
}

/// One frame of the Waves sheet in a document well of `size`.
fn frame_of(
    ctx: &egui::Context,
    state: &mut ResultsTestHost,
    size: egui::Vec2,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
        events,
        ..Default::default()
    };
    ctx.run_ui(input, |ui| {
        well(ui);
        show(ui, &mut state.results_parts());
    })
}

/// A settled frame: fonts and sizing take a pass each.
fn settled(ctx: &egui::Context, state: &mut ResultsTestHost, size: egui::Vec2) -> egui::FullOutput {
    frame_of(ctx, state, size, Vec::new());
    frame_of(ctx, state, size, Vec::new());
    frame_of(ctx, state, size, Vec::new())
}

fn click(ctx: &egui::Context, state: &mut ResultsTestHost, at: egui::Pos2) {
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    frame_of(ctx, state, FRAME, vec![egui::Event::PointerMoved(at)]);
    frame_of(ctx, state, FRAME, vec![button(true)]);
    frame_of(ctx, state, FRAME, vec![button(false)]);
}

fn texts(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
    super::conversion::texts(&output.shapes)
}

/// Each pane's plot area, top to bottom, found by its frame: the canvas's
/// strong vertical down the plot's left edge.
fn plot_areas(ctx: &egui::Context, output: &egui::FullOutput) -> Vec<egui::Rect> {
    let frame = Tokens::get(ctx).color.canvas_frame;
    let mut areas: Vec<egui::Rect> = verticals(&output.shapes)
        .into_iter()
        .filter(|line| line.color == frame && line.bottom - line.top > 8.0)
        .map(|line| {
            egui::Rect::from_min_max(
                egui::pos2(line.x, line.top),
                egui::pos2(FRAME.x, line.bottom),
            )
        })
        .collect();
    areas.sort_by(|a, b| a.top().total_cmp(&b.top()));
    areas
}

/// A tick label's value: a plain number with an SI prefix.
fn tick_value(label: &str) -> Option<f64> {
    let label = label.replace('−', "-");
    let (number, scale) = match label.chars().last()? {
        'k' => (&label[..label.len() - 1], 1.0e3),
        'M' => (&label[..label.len() - 1], 1.0e6),
        'm' => (&label[..label.len() - 1], 1.0e-3),
        'n' => (&label[..label.len() - 1], 1.0e-9),
        'p' => (&label[..label.len() - 1], 1.0e-12),
        'µ' => (&label[..label.len() - 'µ'.len_utf8()], 1.0e-6),
        _ => (label.as_str(), 1.0),
    };
    number.parse::<f64>().ok().map(|value| value * scale)
}

/// The Y tick labels drawn beside `area`, with where each was drawn.
fn tick_labels(output: &egui::FullOutput, area: egui::Rect) -> Vec<(f64, egui::Rect)> {
    texts(output)
        .into_iter()
        .filter(|(_, rect)| {
            rect.right() <= area.left()
                && rect.center().y >= area.top() - 8.0
                && rect.center().y <= area.bottom() + 8.0
        })
        .filter_map(|(text, rect)| tick_value(&text).map(|value| (value, rect)))
        .collect()
}

fn analysis(state: &mut ResultsTestHost) -> AnalysisPresentationKey {
    let presentation = state.model.preferences.result_presentation_policy();
    cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    )[0]
    .analysis_key
}

fn pane_key(state: &mut ResultsTestHost, unit: &str) -> WavePanePresentationKey {
    WavePanePresentationKey {
        analysis: analysis(state),
        unit: unit.to_owned(),
    }
}

/// Where the control named `name` is.
fn control(output: &egui::FullOutput, name: &str) -> egui::Rect {
    controls(output, name)
        .first()
        .copied()
        .unwrap_or_else(|| panic!("the frame publishes {name:?}"))
}

/// Every control of the frame named `name`, top to bottom.
fn controls(output: &egui::FullOutput, name: &str) -> Vec<egui::Rect> {
    let mut found: Vec<egui::Rect> = output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| &update.nodes)
        .filter(|(_, node)| node.label() == Some(name))
        .filter_map(|(_, node)| node.bounds())
        .map(|bounds| {
            egui::Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            )
        })
        .collect();
    found.sort_by(|a, b| a.top().total_cmp(&b.top()));
    found
}

/// A click on a pane's plot, or on its header band where no control is,
/// makes that pane the active one; pointing at a pane does not.
#[test]
fn a_click_anywhere_on_a_pane_makes_it_the_active_one() {
    let ctx = context(crate::ui::Theme::default());
    let mut state = bh_strip();
    let output = settled(&ctx, &mut state, FRAME);
    let areas = plot_areas(&ctx, &output);
    let flux = areas[1].center() - egui::vec2(200.0, 0.0);
    state.results.results.session.active_wave_pane = None;
    frame_of(
        &ctx,
        &mut state,
        FRAME,
        vec![egui::Event::PointerMoved(flux)],
    );
    frame_of(&ctx, &mut state, FRAME, Vec::new());
    assert_eq!(
        state.results.results.session.active_wave_pane, None,
        "pointing at the B pane made it active"
    );
    click(&ctx, &mut state, flux);
    let flux_key = pane_key(&mut state, "T");
    assert_eq!(
        state.results.results.session.active_wave_pane,
        Some(flux_key)
    );
    // The H pane's header band, between its legend and its actions.
    let legend = texts(&output)
        .into_iter()
        .find(|(text, _)| text.starts_with("H(core)"))
        .map(|(_, rect)| rect)
        .expect("the H pane names its trace");
    click(
        &ctx,
        &mut state,
        egui::pos2(FRAME.x * 0.6, legend.center().y),
    );
    let field_key = pane_key(&mut state, "A/m");
    assert_eq!(
        state.results.results.session.active_wave_pane,
        Some(field_key)
    );
}

/// Draw only the traces whose source waveform `keep` accepts, as a reader
/// ticking and unticking them would.
fn show_only(state: &mut ResultsTestHost, keep: impl Fn(&str) -> bool) {
    let toggles: Vec<(usize, usize)> = {
        let models = super::lanes::strip(state);
        let model = &models[0];
        model
            .traces
            .iter()
            .filter(|trace| trace.visible != keep(&trace.source_waveform_name))
            .map(|trace| (model.analysis_index(), trace.waveform_index))
            .collect()
    };
    for (analysis, waveform) in toggles {
        toggle_visibility(&mut state.results_parts(), analysis, waveform);
    }
}

/// A pane of more curves than its row can name names the first ones that
/// fit and counts every other one on its trace list's control.
#[test]
fn a_pane_of_many_curves_counts_every_curve_its_row_cannot_name() {
    let ctx = context(crate::ui::Theme::default());
    let time: Vec<f64> = (0..8).map(|index| f64::from(index) * 1.0e-6).collect();
    let mut state = transient(
        (0..400)
            .map(|ordinal| {
                let values: Vec<f64> = time.iter().map(|t| f64::from(ordinal) * t).collect();
                RetainedTrace::new(format!("V(n{ordinal})"), time.clone(), values, "#fff")
                    .with_unit("V")
            })
            .collect(),
    );
    show_only(&mut state, |_| true);
    let output = settled(&ctx, &mut state, FRAME);
    let named = legend_names(&ctx, &output).len();
    let counted: usize = output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| &update.nodes)
        .find_map(|(_, node)| node.label()?.strip_prefix('+')?.parse().ok())
        .expect("the row counts the curves it cannot name");
    assert!(named > 0, "the row names the curves that fit");
    assert_eq!(named + counted, 400, "every curve is named or counted");
}

/// The names a frame's pane legends give, each with the pane, counted from
/// the top, whose header row it stands in.
fn legend_names(ctx: &egui::Context, output: &egui::FullOutput) -> Vec<(String, usize)> {
    let areas = plot_areas(ctx, output);
    output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| &update.nodes)
        .filter_map(|(_, node)| {
            let name = node
                .label()?
                .strip_prefix("Toggle ")?
                .strip_suffix(" visibility")?;
            let bounds = node.bounds()?;
            let y = ((bounds.y0 + bounds.y1) * 0.5) as f32;
            let pane = areas.iter().position(|area| y < area.top())?;
            (pane == 0 || y > areas[pane - 1].bottom()).then(|| (name.to_owned(), pane))
        })
        .collect()
}

/// A pane's header names exactly what the pane draws: in a Bode strip the
/// magnitudes in the top pane's row and the phases in the phase pane's,
/// each once, and no pane that draws traces has an empty legend.
#[test]
fn a_bode_strip_names_each_trace_in_the_pane_that_draws_it() {
    let ctx = context(crate::ui::Theme::default());
    let mut state = bode_family();
    let output = settled(&ctx, &mut state, FRAME);
    assert_eq!(plot_areas(&ctx, &output).len(), 2, "magnitude over phase");
    let mut names = legend_names(&ctx, &output);
    names.sort();
    let expected: Vec<(String, usize)> = (0..3)
        .map(|ordinal| (format!("phase(V(o{ordinal}))"), 1))
        .chain((0..3).map(|ordinal| (format!("|V(o{ordinal})|"), 0)))
        .collect();
    assert_eq!(names, expected, "each name once, in the pane that draws it");
}

/// A dimensionless curve on a strip's second axis is read against a column
/// that names no unit: its top label is a number, never a number and "1".
#[test]
fn a_dimensionless_second_axis_names_no_unit() {
    use rspice_results::visualization_document::{Strip, StripAxis, StripMember};
    let ctx = context(crate::ui::Theme::default());
    let time: Vec<f64> = (0..=200).map(|index| f64::from(index) * 1.0e-6).collect();
    let sine: Vec<f64> = time.iter().map(|t| (2.0e4 * t).sin()).collect();
    let ratio: Vec<f64> = time.iter().map(|t| 4.0 * t / 2.0e-4).collect();
    let mut state = transient(vec![
        RetainedTrace::new("V(out)", time.clone(), sine, "#fff"),
        RetainedTrace::new("gain", time, ratio, "#fff").with_unit("1"),
    ]);
    let member = |name: &str| StripMember::Signal {
        name: name.to_owned(),
        form: Default::default(),
    };
    let key = analysis(&mut state);
    state.results.results.session.strip_layouts.insert(
        key,
        vec![Strip {
            key: "strip-1".to_owned(),
            weight: None,
            locked: false,
            axes: vec![
                StripAxis {
                    unit: "V".to_owned(),
                    members: vec![member("V(out)")],
                },
                StripAxis {
                    unit: "1".to_owned(),
                    members: vec![member("gain")],
                },
            ],
        }],
    );
    // The models are drawn again for the arrangement.
    state.model.simulation.view.data_version += 1;
    let output = settled(&ctx, &mut state, FRAME);
    let words = texts(&output);
    let gain = words
        .iter()
        .find(|(text, _)| text == "gain")
        .map(|(_, rect)| *rect)
        .expect("the gain's legend entry");
    let volts = words
        .iter()
        .find(|(text, _)| text == "V(out)")
        .map(|(_, rect)| *rect)
        .expect("the output's legend entry");
    assert!(
        (gain.center().y - volts.center().y).abs() < 1.0,
        "one strip names both curves"
    );
    let right: Vec<String> = words
        .into_iter()
        .filter(|(_, rect)| rect.left() >= gain.left() + 500.0)
        .map(|(text, _)| text)
        .collect();
    assert!(
        right.iter().any(|text| text == "4"),
        "the second axis is labelled: {right:?}"
    );
    assert!(
        right.iter().all(|text| !text.ends_with(" 1")),
        "no unit on a dimensionless axis: {right:?}"
    );
}

/// A strip's header names its analysis once: a run's label that only names
/// the kind again ("TRAN" beside TR, "SP" beside SP) is not repeated beside
/// the kind tag, and a label of the run's own stays.
#[test]
fn a_strip_header_names_its_analysis_once() {
    let ctx = context(crate::ui::Theme::default());
    let time: Vec<f64> = (0..=20).map(|index| f64::from(index) * 1.0e-6).collect();
    let mut state = transient(vec![RetainedTrace::new(
        "V(out)",
        time.clone(),
        vec![1.0; 21],
        "#fff",
    )]);
    state.model.simulation.retained.runs[0].analyses[0].label = "TRAN".to_owned();
    let words: Vec<String> = texts(&settled(&ctx, &mut state, FRAME))
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    assert!(words.iter().any(|text| text == "TR"), "{words:?}");
    assert!(!words.iter().any(|text| text == "TRAN"), "{words:?}");
    state.model.simulation.retained.runs[0].analyses[0].label = "Startup".to_owned();
    state.model.simulation.view.data_version += 1;
    let words: Vec<String> = texts(&settled(&ctx, &mut state, FRAME))
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    assert!(words.iter().any(|text| text == "Startup"), "{words:?}");
}

/// The values the H pane's Y axis is labelled with.
fn field_ticks(ctx: &egui::Context, state: &mut ResultsTestHost) -> Vec<f64> {
    let output = settled(ctx, state, FRAME);
    let areas = plot_areas(ctx, &output);
    tick_labels(&output, areas[0])
        .into_iter()
        .map(|(value, _)| value)
        .collect()
}

fn zoom_in_time(state: &mut ResultsTestHost, window: (f64, f64)) {
    let analysis = analysis(state);
    set_shared_x_view(&mut state.results.results.session, analysis, Some(window));
}

/// Zoomed in time to where H runs from 0 to 400, the H pane's axis fits that
/// window, not the −400 to 400 of the whole record.
#[test]
fn a_zoom_in_time_refits_y_to_the_window() {
    let ctx = context(crate::ui::Theme::default());
    let mut state = bh_strip();
    let whole = field_ticks(&ctx, &mut state);
    assert!(
        whole.iter().any(|value| *value <= -300.0),
        "the whole record reaches −400: {whole:?}"
    );
    zoom_in_time(&mut state, (1.0e-3, 1.5e-3));
    let window = field_ticks(&ctx, &mut state);
    assert!(
        window.iter().all(|value| (-50.0..=450.0).contains(value))
            && window.iter().any(|value| *value >= 300.0),
        "the window's 0 to 400 is not what the axis fits: {window:?}"
    );
}

/// A Y range the reader pinned holds through a zoom in time, and the pane's
/// fit control hands the axis back to the window's fit.
#[test]
fn a_pinned_y_holds_through_a_zoom_in_time_until_fit() {
    let ctx = context(crate::ui::Theme::default());
    let mut state = bh_strip();
    let analysis = analysis(&mut state);
    state
        .results
        .results
        .session
        .analysis_plot_view_pane_mut(super::super::super::ResultViewer::Waves, analysis, 0)
        .y = Some((-1_000.0, 1_000.0));
    zoom_in_time(&mut state, (1.0e-3, 1.5e-3));
    let pinned = field_ticks(&ctx, &mut state);
    assert!(
        pinned.iter().any(|value| *value <= -500.0),
        "the pinned −1000 to 1000 did not hold: {pinned:?}"
    );
    let output = settled(&ctx, &mut state, FRAME);
    let fit = control(&output, "Fit Y to the traces in view");
    click(&ctx, &mut state, fit.center());
    let fitted = field_ticks(&ctx, &mut state);
    assert!(
        fitted.iter().all(|value| (-50.0..=450.0).contains(value)),
        "fitting did not refit to the window: {fitted:?}"
    );
}

/// A logarithmic pane fits the positive values the window shows: a current
/// that starts from exactly zero is still offered the axis, and the zero
/// leaves the fit alone.
#[test]
fn a_logarithmic_pane_fits_the_positive_values_in_view() {
    let ctx = context(crate::ui::Theme::default());
    let time: Vec<f64> = (0..=1_000).map(|index| f64::from(index) * 1.0e-6).collect();
    let current: Vec<f64> = time
        .iter()
        .map(|t| {
            if *t == 0.0 {
                0.0
            } else {
                1.0e-12 * (t * 2.0e4).exp()
            }
        })
        .collect();
    let mut state = transient(vec![
        RetainedTrace::new("I(D1)", time, current, "#fff").with_unit("A"),
    ]);
    let models = {
        let presentation = state.model.preferences.result_presentation_policy();
        cached_models(
            &state.model.simulation,
            &mut state.results.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        )
    };
    let panes = models[0].unit_panes();
    set_pane_log_y(
        &mut state.results.results.session,
        &models[0],
        &panes[0],
        true,
    );
    let analysis = models[0].analysis_key;
    set_shared_x_view(
        &mut state.results.results.session,
        analysis,
        Some((5.0e-4, 1.0e-3)),
    );
    let output = settled(&ctx, &mut state, FRAME);
    assert!(pane_log_y(
        &state.results.results.session,
        &models[0],
        &panes[0]
    ));
    let areas = plot_areas(&ctx, &output);
    let labels: Vec<f64> = tick_labels(&output, areas[0])
        .into_iter()
        .map(|(value, _)| value)
        .collect();
    // In the window the current runs from 1e-12·e^10 ≈ 22 nA to e^20 pA ≈ 0.49 mA.
    assert!(
        !labels.is_empty() && labels.iter().all(|value| (1.0e-9..=1.0e-2).contains(value)),
        "the logarithmic axis does not fit the window's 22 nA to 0.49 mA: {labels:?}"
    );
}

/// Three first-order low-passes at 1 kHz, 10 kHz and 100 kHz on the Bode
/// sheet: a magnitude pane in dB over a phase pane in degrees.
fn bode_family() -> ResultsTestHost {
    let frequency: Vec<f64> = (0..=240)
        .map(|index| 10f64.powf(f64::from(index) / 40.0))
        .collect();
    let mut waveforms = Vec::new();
    for (ordinal, corner) in [1.0e3, 1.0e4, 1.0e5].into_iter().enumerate() {
        let ratio = |f: &f64| f / corner;
        waveforms.push(RetainedTrace::new(
            format!("|V(o{ordinal})|"),
            frequency.clone(),
            frequency
                .iter()
                .map(|f| 1.0 / ratio(f).mul_add(ratio(f), 1.0).sqrt())
                .collect::<Vec<f64>>(),
            "#fff",
        ));
        waveforms.push(RetainedTrace::new(
            format!("phase(V(o{ordinal}))"),
            frequency.clone(),
            frequency
                .iter()
                .map(|f| -ratio(f).atan().to_degrees())
                .collect::<Vec<f64>>(),
            "#fff",
        ));
    }
    let mut state = ResultsTestHost::default();
    state
        .model
        .simulation
        .start_run()
        .add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(waveforms));
    state.results.results.session.viewer = super::super::super::ResultViewer::Bode;
    state
}
