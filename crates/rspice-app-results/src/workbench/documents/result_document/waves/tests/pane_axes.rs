//! A pane's chrome and its Y axis, judged through the frame the reader sees:
//! the header carries the legend and no unit tag, its scale toggle stands in
//! the strip's one column of them, a click anywhere on a pane makes it the
//! active one, the panes of a strip share its height equally,
//! every Y tick is labelled with the unit stated above the column, and the
//! automatic Y fit follows the X window until the reader pins a Y of their
//! own.

use super::lanes::{FRAME, verticals, well};
use super::*;
use rspice_ui_kit::EngineeringCanvasTheme;
use rspice_ui_kit::tokens::Mode;

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

/// A transient with one sine in each of `units`, one pane per unit.
fn panes_of(units: &[&str]) -> ResultsTestHost {
    let time: Vec<f64> = (0..256).map(|index| f64::from(index) * 1.0e-6).collect();
    transient(
        units
            .iter()
            .enumerate()
            .map(|(ordinal, unit)| {
                let scale = 10f64.powi(ordinal as i32);
                let values: Vec<f64> = time.iter().map(|t| scale * (t * 3.0e4).sin()).collect();
                RetainedTrace::new(format!("S{ordinal}"), time.clone(), values, "#fff")
                    .with_unit(*unit)
            })
            .collect(),
    )
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

/// The horizontal grid lines painted across `area`.
fn grid_rows(ctx: &egui::Context, output: &egui::FullOutput, area: egui::Rect) -> Vec<f32> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(f32, f32, egui::Color32)>) {
        match shape {
            egui::Shape::LineSegment { points, stroke }
                if (points[0].y - points[1].y).abs() < 1e-3 =>
            {
                out.push((points[0].y, points[0].x.min(points[1].x), stroke.color));
            }
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let grid = Tokens::get(ctx).color.canvas_grid;
    let mut lines = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut lines);
    }
    lines
        .into_iter()
        .filter(|(y, left, color)| {
            *color == grid
                && (left - area.left()).abs() < 0.5
                && *y >= area.top() - 0.5
                && *y <= area.bottom() + 0.5
        })
        .map(|(y, ..)| y)
        .collect()
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

/// No pane header carries its unit: each unit is stated once, as its axis'
/// caption, above the tick column and left of the plot, and no control of
/// the frame is named by a unit.
#[test]
fn no_pane_header_carries_a_unit_chip() {
    let ctx = context(crate::ui::Theme::default());
    let mut state = bh_strip();
    let output = settled(&ctx, &mut state, FRAME);
    let areas = plot_areas(&ctx, &output);
    assert_eq!(areas.len(), 2, "the H and B panes draw: {areas:?}");
    for (unit, area) in ["A/m", "T"].into_iter().zip(&areas) {
        let drawn: Vec<egui::Rect> = texts(&output)
            .into_iter()
            .filter(|(text, _)| text == unit)
            .map(|(_, rect)| rect)
            .collect();
        assert_eq!(drawn.len(), 1, "{unit} is stated once: {drawn:?}");
        assert!(
            drawn[0].right() <= area.left() && drawn[0].bottom() <= area.top(),
            "{unit} at {:?} is not above the tick column of {area:?}",
            drawn[0]
        );
        let named = output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some(unit))
            });
        assert!(!named, "a control is named {unit}");
    }
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

/// The panes of a strip take its height in equal shares, for two, three and
/// four panes.
#[test]
fn the_panes_of_a_strip_take_equal_heights() {
    let ctx = context(crate::ui::Theme::default());
    let units = ["V", "A", "W", "T"];
    for count in 2..=4 {
        let mut state = panes_of(&units[..count]);
        let output = settled(&ctx, &mut state, FRAME);
        let areas = plot_areas(&ctx, &output);
        assert_eq!(areas.len(), count, "{count} panes draw: {areas:?}");
        for area in &areas[1..] {
            assert!(
                (area.height() - areas[0].height()).abs() <= 1.0,
                "{count} panes: heights {:?}",
                areas.iter().map(|area| area.height()).collect::<Vec<_>>()
            );
        }
    }
}

/// Every scale toggle of a strip stands in one right-hand column: each
/// pane's row ends with its icon under the strip header's window control,
/// and its "log Y" chip the strip header's chip gap left of that, at every
/// pane count and at a narrow width. The strip header's own "log X" chip,
/// where it has one, ends on the same edge.
#[test]
fn the_scale_toggles_of_a_strip_stand_in_one_column() {
    let ctx = context(crate::ui::Theme::default());
    let units = ["V", "A", "W"];
    for (count, size) in [
        (1, FRAME),
        (2, FRAME),
        (3, FRAME),
        (3, egui::vec2(520.0, FRAME.y)),
    ] {
        let mut state = panes_of(&units[..count]);
        let output = settled(&ctx, &mut state, size);
        let window = control(&output, "Maximize strip");
        let icons = controls(&output, "Fit Y to the traces in view");
        let chips = controls(&output, "log Y");
        assert_eq!(icons.len(), count, "{count} panes at {size:?}: {icons:?}");
        assert_eq!(chips.len(), count, "{count} panes at {size:?}: {chips:?}");
        for (icon, chip) in icons.iter().zip(&chips) {
            assert!(
                (icon.left() - window.left()).abs() <= 0.5
                    && (icon.right() - window.right()).abs() <= 0.5,
                "{count} panes at {size:?}: the pane icon {icon:?} stands under the \
                 strip's window control {window:?}"
            );
            assert!(
                (icon.left() - chip.right() - 5.0).abs() <= 0.5,
                "{count} panes at {size:?}: \"log Y\" {chip:?} ends the strip header's \
                 chip gap left of its icon {icon:?}"
            );
            for log_x in controls(&output, "log X") {
                assert!(
                    (log_x.right() - chip.right()).abs() <= 0.5,
                    "{count} panes at {size:?}: \"log X\" {log_x:?} and \"log Y\" \
                     {chip:?} end on one edge"
                );
            }
        }
    }
}

/// The light interface over the light canvas.
fn light() -> crate::ui::Theme {
    crate::ui::Theme {
        mode: Mode::Light,
        canvas_theme: EngineeringCanvasTheme::Light,
        ..crate::ui::Theme::default()
    }
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

/// Whether the strip draws the trace of `name`.
fn drawn(state: &mut ResultsTestHost, name: &str) -> bool {
    super::lanes::strip(state)[0]
        .traces
        .iter()
        .any(|trace| trace.source_waveform_name == name && trace.visible)
}

/// One volts pane with more to name than its row holds: three drawn
/// signals whose hierarchical names each take the share of the row a name
/// may, and ten the reader has hidden.
fn crowded_pane() -> ResultsTestHost {
    let time: Vec<f64> = (0..64).map(|index| f64::from(index) * 1.0e-6).collect();
    let drawn = (0..3).map(|ordinal| {
        format!("V(xtop.xcore.xstage{ordinal}.xbias.xmirror.output_node_of_the_cascode_{ordinal})")
    });
    let hidden = (0..10).map(|ordinal| format!("V(h{ordinal})"));
    let mut state = transient(
        drawn
            .chain(hidden)
            .enumerate()
            .map(|(ordinal, name)| {
                let values: Vec<f64> = time
                    .iter()
                    .map(|t| (ordinal as f64 + 1.0) * (t * 1.0e5).sin())
                    .collect();
                RetainedTrace::new(name, time.clone(), values, "#fff").with_unit("V")
            })
            .collect(),
    );
    show_only(&mut state, |name| !name.starts_with("V(h"));
    state
}

/// Whatever a pane's row cannot name waits behind one control right after
/// its last entry: "+1 · 10 hidden" for one drawn signal there is no room
/// for and ten the reader hid, left of the strip's column of scale and
/// window controls.
#[test]
fn the_trace_list_counts_what_the_row_cannot_name() {
    let ctx = context(crate::ui::Theme::default());
    let mut state = crowded_pane();
    let output = settled(&ctx, &mut state, FRAME);
    let list = control(&output, "+1 · 10 hidden");
    let log = control(&output, "log Y");
    let named: Vec<egui::Rect> = texts(&output)
        .into_iter()
        .filter(|(text, rect)| {
            text.starts_with("V(xtop") && (rect.center().y - list.center().y).abs() < 4.0
        })
        .map(|(_, rect)| rect)
        .collect();
    assert_eq!(named.len(), 2, "two of the three drawn signals are named");
    let last = named
        .iter()
        .map(|rect| rect.right())
        .fold(f32::NEG_INFINITY, f32::max);
    assert!(
        list.left() > last && list.left() - last < 16.0,
        "the list follows the last entry: entry ends at {last}, list {list:?}"
    );
    assert!(
        log.left() - list.right() > 40.0,
        "the list is the legend's, not the scale column's: list {list:?}, log Y {log:?}"
    );
}

/// The legend's control, the scale chip and the fit icon are one family of
/// header control: each lies inside the header band, and all three take one
/// height on one centre line.
#[test]
fn every_header_control_shares_one_height_inside_the_band() {
    for theme in [crate::ui::Theme::default(), light()] {
        let ctx = context(theme);
        let mut state = crowded_pane();
        let output = settled(&ctx, &mut state, FRAME);
        let band = control(&output, "V pane");
        let header = [
            control(&output, "+1 · 10 hidden"),
            control(&output, "log Y"),
            control(&output, "Fit Y to the traces in view"),
        ];
        for rect in header {
            assert!(
                band.expand(0.5).contains_rect(rect),
                "{rect:?} lies inside the header band {band:?}"
            );
            assert!(
                (rect.height() - header[1].height()).abs() <= 0.5
                    && (rect.center().y - header[1].center().y).abs() <= 0.5,
                "one height on one centre line: {header:?}"
            );
            assert!(
                (rect.center().y - band.center().y).abs() <= 1.0,
                "centred in the band: {rect:?} in {band:?}"
            );
        }
        // The legend's entries stand on the same centre line.
        for ordinal in 0..2 {
            let swatch = control(
                &output,
                &format!(
                    "Toggle V(xtop.xcore.xstage{ordinal}.xbias.xmirror.output_node_of_the_cascode_{ordinal}) visibility"
                ),
            );
            assert!(
                (swatch.center().y - band.center().y).abs() <= 1.0,
                "the entry's swatch {swatch:?} is centred in the band {band:?}"
            );
        }
    }
}

/// Ticking a hidden signal in the trace list draws it, and the list stays
/// open for the next one, listing it among the drawn ones now.
#[test]
fn ticking_a_hidden_trace_in_the_list_draws_it() {
    let ctx = context(crate::ui::Theme::default());
    let mut state = crowded_pane();
    let output = settled(&ctx, &mut state, FRAME);
    click(
        &ctx,
        &mut state,
        control(&output, "+1 · 10 hidden").center(),
    );
    let menu = settled(&ctx, &mut state, FRAME);
    let names: Vec<egui::Rect> = (0..10)
        .map(|ordinal| control(&menu, &format!("V(h{ordinal})")))
        .collect();
    assert!(
        names.windows(2).all(|pair| pair[0].top() < pair[1].top()),
        "the hidden signals are listed in legend order"
    );
    let drawn_first = control(
        &menu,
        "V(xtop.xcore.xstage2.xbias.xmirror.output_node_of_the_cascode_2)",
    );
    assert!(
        drawn_first.bottom() <= names[0].top(),
        "the drawn signals come first, the overflowed one by its whole name"
    );
    assert!(!drawn(&mut state, "V(h3)"));
    click(&ctx, &mut state, names[3].center());
    assert!(drawn(&mut state, "V(h3)"), "ticking it draws it");
    let after = settled(&ctx, &mut state, FRAME);
    assert!(
        controls(&after, "V(h4)").len() == 1,
        "the list stays open for the next one"
    );
    assert!(
        !controls(&after, "+1 · 9 hidden").is_empty()
            || !controls(&after, "+2 · 9 hidden").is_empty(),
        "the list counts nine hidden now"
    );
}

/// An NMOS output family from a nested DC sweep, `.dc VD 0 3 0.05 VG 1 3
/// 0.5`, with only the drain current through the sense source drawn: five
/// members of I(VSENSE) and every other vector hidden.
fn nmos_output_family() -> ResultsTestHost {
    let run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch(
        "nmos output family\n\
         VD d 0 DC 0\n\
         VG g 0 DC 1\n\
         VSENSE d dr DC 0\n\
         M1 dr g 0 0 nch W=10u L=1u\n\
         .model nch nmos (level=1 vto=0.7 kp=100u lambda=0.02)\n\
         .dc VD 0 3 0.05 VG 1 3 0.5\n\
         .end\n",
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.results.results.session.viewer = super::super::super::ResultViewer::DcSweep;
    show_only(&mut state, |name| name.starts_with("I(VSENSE)"));
    state
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
    for theme in [crate::ui::Theme::default(), light()] {
        let ctx = context(theme);
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
}

/// A pane that draws nothing takes no row: with every voltage of the DC
/// family hidden the strip is its current pane alone, and plotting V(D)
/// from the data browser brings the voltage pane back, the two sharing the
/// strip's height equally.
#[test]
fn a_pane_with_nothing_drawn_takes_no_row_until_a_signal_is_plotted() {
    use crate::workbench::documents::result_document::browser_plot::set_result_signal_plotted;
    use rspice_results::result_presentation::SourceWaveformPresentationKey;
    let ctx = context(crate::ui::Theme::default());
    let mut state = nmos_output_family();
    let output = settled(&ctx, &mut state, FRAME);
    assert_eq!(plot_areas(&ctx, &output).len(), 1, "one pane row");
    assert!(
        controls(&output, "V pane").is_empty(),
        "no header for the hidden voltages"
    );
    let key = {
        let models = super::lanes::strip(&mut state);
        let model = &models[0];
        let trace = model
            .traces
            .iter()
            .find(|trace| trace.source_waveform_name.starts_with("V(D)"))
            .expect("the run keeps V(D)");
        SourceWaveformPresentationKey::new(model.analysis_key, trace.source_waveform_name.clone())
    };
    set_result_signal_plotted(&mut state.results_parts(), &key, true);
    let output = settled(&ctx, &mut state, FRAME);
    let areas = plot_areas(&ctx, &output);
    assert_eq!(areas.len(), 2, "V(D) brings its pane back");
    assert!(
        (areas[0].height() - areas[1].height()).abs() <= 1.0,
        "equal heights: {areas:?}"
    );
}

/// Every Y grid line carries its label, and the unit caption overlaps none
/// of them, from a tall strip down to four panes in a short one.
#[test]
fn every_y_tick_is_labelled_and_the_caption_overlaps_none() {
    let units = ["V", "A", "W", "T"];
    for theme in [crate::ui::Theme::default(), light()] {
        let ctx = context(theme);
        for height in [760.0, 560.0, 440.0] {
            let size = egui::vec2(FRAME.x, height);
            let mut state = panes_of(&units);
            let output = settled(&ctx, &mut state, size);
            let areas = plot_areas(&ctx, &output);
            assert_eq!(areas.len(), units.len(), "{height}: {areas:?}");
            for (unit, area) in units.iter().zip(&areas) {
                let rows = grid_rows(&ctx, &output, *area);
                let labels = tick_labels(&output, *area);
                assert!(rows.len() >= 2, "{height} {unit}: {} rows", rows.len());
                assert_eq!(
                    labels.len(),
                    rows.len(),
                    "{height} {unit}: {} labels for {} grid rows ({:.0} px pane)",
                    labels.len(),
                    rows.len(),
                    area.height()
                );
                let caption = texts(&output)
                    .into_iter()
                    .find(|(text, rect)| text == unit && rect.right() <= area.left())
                    .map(|(_, rect)| rect)
                    .unwrap_or_else(|| panic!("{height}: the {unit} axis states its unit"));
                for (value, label) in &labels {
                    assert!(
                        !caption.intersects(*label),
                        "{height} {unit}: the caption {caption:?} covers {value} at {label:?}"
                    );
                }
            }
        }
    }
}

/// How tall a pane must be to label three Y ticks, and what a shorter one
/// draws: four panes of sines of assorted scales in strips of shrinking
/// height, reported per pane height as the fewest labels any pane drew.
///
/// `cargo test -p rspice-app-results pane_axes::measure_ -- --ignored --nocapture`
#[test]
#[ignore = "measurement harness"]
fn measure_the_labels_a_short_pane_draws() {
    use std::io::Write as _;
    let ctx = context(crate::ui::Theme::default());
    let units = ["V", "A", "W", "T"];
    let mut lines = Vec::new();
    for height in (160..=900).rev().step_by(20) {
        let size = egui::vec2(FRAME.x, height as f32);
        let mut state = panes_of(&units);
        let output = settled(&ctx, &mut state, size);
        let areas = plot_areas(&ctx, &output);
        let fewest = areas
            .iter()
            .map(|area| tick_labels(&output, *area).len())
            .min()
            .unwrap_or(0);
        let rows = areas
            .iter()
            .map(|area| grid_rows(&ctx, &output, *area).len())
            .min()
            .unwrap_or(0);
        lines.push(format!(
            "strip {height:4} px: {} panes drawn, plot {:5.1} px, fewest labels {fewest}, fewest rows {rows}",
            areas.len(),
            areas.first().map_or(0.0, |area| area.height()),
        ));
    }
    let mut out = std::io::stdout().lock();
    writeln!(out, "{}", lines.join("\n")).expect("the report is written");
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
    set_shared_x_view(
        &mut state.results.results.session,
        analysis,
        2,
        Some(window),
    );
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
        1,
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

/// The pictures for review, written under `RSPICE_PICTURES` when it is set:
/// the B–H strip zoomed to 1–1.5 ms, the Bode family and a three-pane
/// transient, in both interface modes over both canvases.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_pane_pictures() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let themes = [
        ("dark-dark", Mode::Dark, EngineeringCanvasTheme::Dark),
        ("light-light", Mode::Light, EngineeringCanvasTheme::Light),
        ("dark-light", Mode::Dark, EngineeringCanvasTheme::Light),
        ("light-dark", Mode::Light, EngineeringCanvasTheme::Dark),
    ];
    let three = || {
        let time: Vec<f64> = (0..=4_000).map(|index| f64::from(index) * 2.5e-7).collect();
        let wave = |scale: f64, offset: f64, f: f64| -> Vec<f64> {
            time.iter()
                .map(|t| offset + scale * (2.0 * std::f64::consts::PI * f * t).sin())
                .collect()
        };
        transient(vec![
            RetainedTrace::new("V(out)", time.clone(), wave(2.5, 2.5, 1.0e3), "#fff"),
            RetainedTrace::new("V(in)", time.clone(), wave(1.0, 2.5, 1.0e3), "#fff"),
            RetainedTrace::new("I(VDD)", time.clone(), wave(2.0e-3, -4.0e-3, 2.0e3), "#fff"),
            RetainedTrace::new("P(M1)", time.clone(), wave(5.0e-3, 6.0e-3, 2.0e3), "#fff")
                .with_unit("W"),
        ])
    };
    for (name, mode, canvas_theme) in themes {
        let theme = crate::ui::Theme {
            mode,
            canvas_theme,
            ..crate::ui::Theme::default()
        };
        let mut pictures: Vec<(&str, ResultsTestHost)> = Vec::new();
        let mut bh = bh_strip();
        zoom_in_time(&mut bh, (1.0e-3, 1.5e-3));
        let key = pane_key(&mut bh, "A/m");
        bh.results.results.session.active_wave_pane = Some(key);
        pictures.push(("bh-zoomed", bh));
        pictures.push(("bode", bode_family()));
        pictures.push(("dc-family", nmos_output_family()));
        pictures.push(("three-pane", three()));
        for (picture, mut state) in pictures {
            let canvas = rspice_ui_kit::raster::render_themed(theme, FRAME, |ui, _| {
                well(ui);
                show(ui, &mut state.results_parts());
            });
            std::fs::write(
                format!("{directory}/{picture}-{name}.png"),
                canvas.png(canvas.height()),
            )
            .expect("the picture is written");
        }
    }
}
