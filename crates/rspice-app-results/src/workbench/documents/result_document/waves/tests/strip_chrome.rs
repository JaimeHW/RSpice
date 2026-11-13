//! The strip's chrome on a real mixed run, judged through the frame the
//! reader sees: text against what it is painted on in every theme the
//! preferences allow, the trace palette and the cursors, the gutter's
//! columns, a pane with nothing plotted, and the hidden-trace control.

use super::lanes::{
    FRAME, REVIEW_NODES, REVIEW_PLOTTED, cursor_lines, review_strip, strip, verticals, well,
};
use super::*;
use rspice_ui_kit::EngineeringCanvasTheme;
use rspice_ui_kit::palette::perceptual_distance;
use rspice_ui_kit::tokens::Mode;

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

/// Every colour a curve was stroked in: a line or path whose colour is one
/// of the palette's hues.
fn stroked_hues(output: &egui::FullOutput, palette: &[egui::Color32]) -> Vec<egui::Color32> {
    fn walk(shape: &egui::Shape, out: &mut Vec<egui::Color32>) {
        match shape {
            egui::Shape::LineSegment { stroke, .. } => out.push(stroke.color),
            egui::Shape::Path(path) => {
                if let egui::epaint::ColorMode::Solid(color) = path.stroke.color {
                    out.push(color);
                }
            }
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut out);
    }
    let mut hues: Vec<egui::Color32> = out
        .into_iter()
        .filter(|color| palette.contains(color))
        .collect();
    hues.sort_by_key(|color| color.to_array());
    hues.dedup();
    hues
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

/// Every run of text the strip paints stands 4.5:1 off what is painted
/// behind it, in every interface mode, over either canvas, with color-safe
/// traces on and off: the lanes' names and values, the bus words, the tick
/// labels and units, the legend and the axis.
#[test]
fn every_run_of_text_reads_off_its_ground_in_every_theme() {
    let mut failures = Vec::new();
    for mode in Mode::ALL {
        for canvas_theme in EngineeringCanvasTheme::ALL {
            for colorblind_traces in [false, true] {
                let theme = crate::ui::Theme {
                    mode,
                    canvas_theme,
                    colorblind_traces,
                    ..crate::ui::Theme::default()
                };
                let ctx = themed_context(theme);
                let mut state = review_strip(true);
                let output = settled(&ctx, &mut state);
                let contrasts = rspice_ui_kit::raster::text_contrasts(&ctx, FRAME, &output);
                assert!(contrasts.len() > 40, "{theme:?}: the strip paints its text");
                failures.extend(
                    contrasts
                        .into_iter()
                        .filter(|text| text.ratio < 4.5)
                        .map(|text| {
                            format!(
                                "{mode:?}/{canvas_theme:?}/{colorblind_traces}: {:?} {:.2}:1 ({:?} on {:?})",
                                text.text, text.ratio, text.ink, text.ground
                            )
                        }),
                );
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Ten nodes on one pane draw in all eight hues, each 0.09 apart, and the
/// ninth and tenth repeat a hue in a dash of their own. Cursor B is drawn,
/// across the lanes and the pane, in a colour none of those curves has.
#[test]
fn eight_hues_are_drawn_and_cursor_b_is_none_of_them() {
    for canvas_theme in EngineeringCanvasTheme::ALL {
        let ctx = themed_context(crate::ui::Theme {
            canvas_theme,
            ..crate::ui::Theme::default()
        });
        let mut state = review_strip(true);
        for analysis in &mut state.model.simulation.retained.runs[0].analyses {
            for waveform in &mut analysis.waveforms {
                waveform.visible = REVIEW_NODES.contains(&waveform.name.as_str());
            }
        }
        state.model.simulation.view.data_version =
            state.model.simulation.view.data_version.wrapping_add(1);
        let output = settled(&ctx, &mut state);
        let tokens = Tokens::get(&ctx);
        let hues = stroked_hues(&output, &tokens.color.traces);
        assert_eq!(
            hues.len(),
            8,
            "{canvas_theme:?}: ten curves use all eight hues"
        );
        for (index, hue) in hues.iter().enumerate() {
            for other in &hues[index + 1..] {
                assert!(
                    perceptual_distance(*hue, *other) >= 0.09,
                    "{canvas_theme:?}"
                );
            }
            assert!(
                perceptual_distance(tokens.color.canvas_cursor_b, *hue) >= 0.09,
                "{canvas_theme:?}: cursor B is near {hue:?}"
            );
        }
        let crossings = cursor_lines(&verticals(&output.shapes), tokens.color.canvas_cursor_b);
        assert!(
            crossings.len() >= 2,
            "{canvas_theme:?}: cursor B crosses the lanes and the pane"
        );
        let models = strip(&mut state);
        let model = &models[0];
        let look =
            |trace: &StripTrace| rspice_results_ui::waves::pane::SwatchStyle::of(trace).look(false);
        // No two plotted curves share a hue and a dash, however far apart
        // their columns sit in the dataset.
        let plotted: Vec<&StripTrace> = model.traces.iter().filter(|trace| trace.visible).collect();
        assert_eq!(plotted.len(), REVIEW_NODES.len(), "{canvas_theme:?}");
        for (index, trace) in plotted.iter().enumerate() {
            for other in &plotted[index + 1..] {
                assert!(
                    trace.color != other.color || look(trace).dash != look(other).dash,
                    "{canvas_theme:?}: {} and {} draw alike",
                    trace.name,
                    other.name
                );
            }
        }
        let slot = |slot: usize| {
            model
                .traces
                .iter()
                .find(|trace| trace.palette_slot == Some(slot))
                .unwrap_or_else(|| panic!("a trace sits in slot {slot}"))
        };
        let first = slot(0);
        let ninth = model
            .traces
            .iter()
            .filter(|trace| trace.palette_slot.is_some_and(|slot| slot >= 8))
            .min_by_key(|trace| trace.palette_slot)
            .expect("ten nodes reach past the eighth slot");
        assert_ne!(
            look(first).dash,
            look(ninth).dash,
            "a repeated hue is dashed"
        );
    }
}

/// The lanes' time axis and the panes' plot area start on one edge at the
/// default name column and at widths the reader dragged to, so cursor A
/// crosses them at one x; the bus keeps its whole name.
#[test]
fn the_gutter_aligns_lanes_and_panes_at_two_widths() {
    let ctx = themed_context(crate::ui::Theme::default());
    let mut lefts = Vec::new();
    for width in [None, Some(120.0), Some(180.0)] {
        let mut state = review_strip(true);
        state.results.results.session.lane_name_width = width;
        let output = settled(&ctx, &mut state);
        let lines = verticals(&output.shapes);
        let crossings = cursor_lines(&lines, Tokens::get(&ctx).color.canvas_cursor_a);
        assert!(
            crossings.len() >= 2,
            "{width:?}: cursor A crosses lanes and panes"
        );
        for pane in &crossings[1..] {
            assert!(
                (pane.x - crossings[0].x).abs() <= 0.5,
                "{width:?}: cursor A at x {} on the lanes and {} on a pane",
                crossings[0].x,
                pane.x
            );
        }
        let frame = Tokens::get(&ctx).color.border_strong;
        let plot_left = lines
            .iter()
            .filter(|line| line.color == frame && line.bottom - line.top > 40.0)
            .map(|line| line.x)
            .fold(f32::INFINITY, f32::min);
        lefts.push(plot_left);
        assert!(
            text_rect(&output, "code").is_some(),
            "{width:?}: the bus is named whole"
        );
    }
    let moved = lefts[2] - lefts[1];
    assert!(
        (moved - 60.0).abs() <= 0.5,
        "the plot area moves with the name column: {lefts:?}"
    );
}

/// The name column opens as wide as the longest name it shows, so a long
/// hierarchical bus name reads whole where the old fixed column elided it,
/// and keeps within a quarter of the strip. A width the reader dragged to
/// still rules, eliding the name again.
#[test]
fn the_name_column_opens_as_wide_as_its_longest_name() {
    use rspice_results_ui::waves::lanes::LaneKey;
    const LONG: &str = "xtop.xsar_core.result_code";
    let ctx = themed_context(crate::ui::Theme::default());
    let mut state = review_strip(true);
    for layout in state.results.results.session.event_lanes.values_mut() {
        for bus in &mut layout.user_buses {
            bus.name = LONG.to_owned();
        }
        for key in &mut layout.order {
            if *key == LaneKey::Bus("code".to_owned()) {
                *key = LaneKey::Bus(LONG.to_owned());
            }
        }
    }
    let output = settled(&ctx, &mut state);
    let name = text_rect(&output, LONG).expect("the bus is named whole");
    assert!(
        name.right() <= FRAME.x * 0.25,
        "the column takes more than a quarter of the strip: {name:?}"
    );
    state.results.results.session.lane_name_width = Some(60.0);
    let output = settled(&ctx, &mut state);
    assert!(
        text_rect(&output, LONG).is_none(),
        "the dragged width is not the reader's"
    );
}

/// A pane whose traces are all hidden is its header row and nothing below
/// it: the shared axis starts right under it, and no hint fills a pane
/// height with a sentence.
#[test]
fn a_pane_with_every_trace_hidden_is_its_header_alone() {
    let ctx = themed_context(crate::ui::Theme::default());
    let mut state = review_strip(false);
    let output = settled(&ctx, &mut state);
    let found = texts(&output);
    assert!(
        !found
            .iter()
            .any(|(text, _)| text.starts_with("No visible traces")),
        "a hidden pane states nothing in a pane-sized well"
    );
    let header = found
        .iter()
        .filter(|(text, _)| text.ends_with(" hidden"))
        .map(|(_, rect)| *rect)
        .max_by(|a, b| a.top().total_cmp(&b.top()))
        .expect("the hidden pane offers its traces in its header");
    let axis = found
        .iter()
        .find(|(text, _)| text.starts_with("TIME"))
        .map(|(_, rect)| *rect)
        .expect("the shared axis labels itself");
    assert!(
        axis.top() - header.bottom() < rspice_results_ui::waves::header::WAVE_PANE_HEADER_HEIGHT,
        "the axis follows the header: header ends at {}, axis starts at {}",
        header.bottom(),
        axis.top()
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

/// A curve the reader adds takes the next hue after the seven the dataset
/// plots, as a waveform viewer gives a newly plotted signal the next free
/// colour, and hiding a plotted curve recolours none of the others.
#[test]
fn a_curve_the_reader_adds_takes_the_next_free_hue() {
    use rspice_results::result_presentation::SourceWaveformPresentationKey;
    let mut state = review_strip(true);
    let before = strip(&mut state);
    let model = &before[0];
    let current = model
        .traces
        .iter()
        .find(|trace| trace.source_waveform_name.starts_with("I("))
        .expect("the review keeps its currents")
        .source_waveform_name
        .clone();
    let shown =
        |name: &str| SourceWaveformPresentationKey::new(model.analysis_key, name.to_owned());
    let palette = Tokens::default().color.traces;
    let session = &mut state.results.results.session;
    session.waveform_visibility.insert(shown(&current), true);
    session.waveform_visibility.insert(shown("TOP"), false);
    let after = strip(&mut state);
    let trace = |models: &[StripModel], name: &str| {
        models[0]
            .traces
            .iter()
            .find(|trace| trace.source_waveform_name == name)
            .map(|trace| (trace.palette_slot, trace.color))
            .unwrap_or_else(|| panic!("{name} is a curve of the strip"))
    };
    assert_eq!(
        trace(&after, &current),
        (Some(7), palette[7]),
        "the eighth curve takes the eighth hue"
    );
    for name in REVIEW_PLOTTED {
        assert_eq!(
            trace(&after, name),
            trace(&before, name),
            "{name} kept its hue when TOP was hidden and {current} was added"
        );
    }
}

/// Hold `node` at `value` through the whole run.
fn hold(state: &mut ResultsTestHost, node: &str, value: f64) {
    for analysis in &mut state.model.simulation.retained.runs[0].analyses {
        for waveform in &mut analysis.waveforms {
            if waveform.name == node {
                waveform.y = std::sync::Arc::new(vec![value; waveform.y.len()]);
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
        (2.402_879e-26, 4.56e31, "24.02879e-27 V", "45.6e30 V"),
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

/// The strip as the SAR run leaves it, with TOP a few zeptovolts off
/// ground at cursor A, written under `RSPICE_PICTURES` when it is set.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_tiny_value_legend_picture() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let mut state = review_strip(false);
    let canvas =
        rspice_ui_kit::raster::render_themed(crate::ui::Theme::default(), FRAME, |ui, _| {
            well(ui);
            show(ui, &mut state.results_parts());
        });
    std::fs::write(
        format!("{directory}/legend-tiny-value.png"),
        canvas.png(canvas.height()),
    )
    .expect("the picture is written");
}
