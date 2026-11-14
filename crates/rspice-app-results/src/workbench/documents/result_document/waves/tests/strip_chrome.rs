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

/// Every stroke `output` paints on the canvas, with its ink: on a canvas
/// fill, and not on an interface surface laid over it (a tag, a menu), whose
/// outline and rules belong to that surface.
fn canvas_strokes(
    ctx: &egui::Context,
    output: &egui::FullOutput,
) -> Vec<(egui::Rect, egui::Color32)> {
    struct Walk {
        canvas: egui::Color32,
        surface_fills: [egui::Color32; 5],
        grounds: Vec<egui::Rect>,
        surfaces: Vec<egui::Rect>,
        strokes: Vec<(egui::Rect, egui::Color32)>,
    }
    fn walk(shape: &egui::Shape, into: &mut Walk) {
        let rect = shape.visual_bounding_rect();
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
            egui::Shape::Rect(fill) if fill.fill == into.canvas => into.grounds.push(fill.rect),
            egui::Shape::Rect(fill) if into.surface_fills.contains(&fill.fill) => {
                into.surfaces.push(fill.rect);
            }
            egui::Shape::Rect(outline) if outline.fill.a() == 0 && outline.stroke.width > 0.0 => {
                into.strokes.push((rect, outline.stroke.color));
            }
            egui::Shape::LineSegment { stroke, .. } => into.strokes.push((rect, stroke.color)),
            egui::Shape::Circle(circle) if circle.fill.a() == 0 && circle.stroke.width > 0.0 => {
                into.strokes.push((rect, circle.stroke.color));
            }
            egui::Shape::Path(path) if path.fill.a() == 0 && path.stroke.width > 0.0 => {
                if let egui::epaint::ColorMode::Solid(color) = path.stroke.color {
                    into.strokes.push((rect, color));
                }
            }
            _ => {}
        }
    }
    let c = Tokens::get(ctx).color;
    let mut found = Walk {
        canvas: c.canvas_bg,
        surface_fills: [
            c.bg_app,
            c.bg_panel,
            c.bg_panel_2,
            c.bg_inset,
            c.bg_elevated,
        ],
        grounds: Vec::new(),
        surfaces: Vec::new(),
        strokes: Vec::new(),
    };
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut found);
    }
    let within = |outer: &egui::Rect, inner: &egui::Rect| outer.expand(1.0).contains_rect(*inner);
    // A surface counts only where it lies on a canvas: the panel a canvas
    // sits in is not laid over it.
    let surfaces: Vec<_> = found
        .surfaces
        .iter()
        .filter(|surface| found.grounds.iter().any(|ground| within(ground, surface)))
        .collect();
    found
        .strokes
        .into_iter()
        .filter(|(rect, _)| {
            found.grounds.iter().any(|ground| within(ground, rect))
                && !surfaces.iter().any(|surface| within(surface, rect))
        })
        .collect()
}

/// A strip of a volts pane over an amps pane, both plotted.
fn two_panes() -> ResultsTestHost {
    let time: Vec<f64> = (0..64).map(|index| f64::from(index) * 1.0e-6).collect();
    let wave =
        |scale: f64| -> Vec<f64> { time.iter().map(|t| scale * (t * 2.0e5).sin()).collect() };
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(out)", time.clone(), wave(1.0), "#fff"),
            RetainedTrace::new("I(R1)", time.clone(), wave(1.0e-3), "#fff"),
        ]),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state
}

/// The marks the strip draws on its canvas take the canvas theme's inks in
/// all four theme combinations: each pane's frame is the canvas's own strong
/// border, and no frame, grid line or seam is drawn in an interface border
/// grey. A light interface drew the frames in its own light grey, which
/// glared on the dark canvas, and a dark one drew them in a grey that
/// vanished into paper.
#[test]
fn the_strips_marks_take_the_canvas_inks_in_every_theme() {
    let mut failures = Vec::new();
    for mode in [Mode::Dark, Mode::Light] {
        for canvas_theme in EngineeringCanvasTheme::ALL {
            let ctx = themed_context(crate::ui::Theme {
                mode,
                canvas_theme,
                ..crate::ui::Theme::default()
            });
            let mut state = two_panes();
            let output = settled(&ctx, &mut state);
            let c = Tokens::get(&ctx).color;
            let strokes = canvas_strokes(&ctx, &output);
            let frames = strokes
                .iter()
                .filter(|(_, ink)| *ink == c.canvas_frame)
                .count();
            if frames < 2 {
                failures.push(format!(
                    "{mode:?}/{canvas_theme:?}: {frames} frame lines in the canvas frame ink"
                ));
            }
            let canvas_inks = [c.canvas_frame, c.canvas_grid];
            for (rect, ink) in strokes {
                if let Some((name, _)) = [("border", c.border), ("strong border", c.border_strong)]
                    .into_iter()
                    .find(|(_, interface)| *interface == ink && !canvas_inks.contains(&ink))
                {
                    failures.push(format!(
                        "{mode:?}/{canvas_theme:?}: a mark in the interface's {name} at {:?}",
                        rect.min
                    ));
                }
            }
        }
    }
    failures.dedup();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// One pane of volts at a narrow width: a dozen plotted signals with long
/// hierarchical names, and ten the dataset does not plot.
fn crowded_pane() -> ResultsTestHost {
    let time: Vec<f64> = (0..64).map(|index| f64::from(index) * 1.0e-6).collect();
    let waveforms = (0..22)
        .map(|index| {
            let values: Vec<f64> = time
                .iter()
                .map(|t| (t * 2.0e5 + f64::from(index)).sin())
                .collect();
            let mut waveform = RetainedTrace::new(
                format!("V(xtop.xcore.node_{index:02})"),
                time.clone(),
                values,
                "#fff",
            );
            waveform.visible = index < 12;
            waveform
        })
        .collect();
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(waveforms),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state
}

/// The crowded pane's strip, settled, in a well `width` wide.
fn crowded_frame(ctx: &egui::Context, state: &mut ResultsTestHost, width: f32) -> egui::FullOutput {
    ctx.enable_accesskit();
    let mut output = None;
    for events in [vec![egui::Event::PointerGone], Vec::new(), Vec::new()] {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 420.0),
            )),
            events,
            ..Default::default()
        };
        output = Some(ctx.run_ui(input, |ui| {
            well(ui);
            show(ui, &mut state.results_parts());
        }));
    }
    output.expect("a frame ran")
}

/// A pane's hidden control is painted whole however crowded its row: at a
/// narrow width, with a dozen signals plotted and ten hidden, the plotted
/// ones that do not fit go behind the overflow control, and "10 hidden" is
/// neither clipped nor under the log and autoscale buttons at the row's
/// end. The buttons took their width after the legend had been laid out,
/// and the hidden control was the entry they cut, to "10 hidd".
#[test]
fn the_hidden_control_shows_whole_on_a_crowded_row() {
    fn painted(
        shape: &egui::Shape,
        clip: egui::Rect,
        out: &mut Vec<(String, egui::Rect, egui::Rect)>,
    ) {
        match shape {
            egui::Shape::Text(text) => out.push((
                text.galley.text().to_owned(),
                text.visual_bounding_rect(),
                clip,
            )),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| painted(shape, clip, out)),
            _ => {}
        }
    }
    let mut failures = Vec::new();
    for (width, cursor) in (16..=46)
        .flat_map(|step| [None, Some(13.0e-6)].map(|cursor| (f32::from(step as u8) * 20.0, cursor)))
    {
        let ctx = themed_context(crate::ui::Theme::default());
        let mut state = crowded_pane();
        super::lanes::place_cursors(&mut state, cursor, None);
        let output = crowded_frame(&ctx, &mut state, width);
        let mut texts = Vec::new();
        for clipped in &output.shapes {
            painted(&clipped.shape, clipped.clip_rect, &mut texts);
        }
        let Some((_, chip, clip)) = texts.iter().find(|(text, ..)| text == "10 hidden") else {
            failures.push(format!("{width} {cursor:?}: no whole hidden control"));
            continue;
        };
        if !clip.contains_rect(*chip) {
            failures.push(format!(
                "{width} {cursor:?}: the hidden control is clipped: {chip:?} in {clip:?}"
            ));
        }
        // The plotted signals that do not fit wait behind the overflow
        // control, which is whole too.
        let overflow = texts.iter().find(|(text, ..)| {
            text.strip_prefix('+')
                .is_some_and(|count| count.parse::<usize>().is_ok())
        });
        match overflow {
            Some((text, rect, clip)) if !clip.contains_rect(*rect) => failures.push(format!(
                "{width} {cursor:?}: the overflow control {text:?} is clipped"
            )),
            None => failures.push(format!("{width} {cursor:?}: no overflow control")),
            Some(_) => {}
        }
        let buttons: Vec<egui::Rect> = output
            .platform_output
            .accesskit_update
            .as_ref()
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter(|(_, node)| {
                        matches!(node.label(), Some("log Y" | "Fit Y to the traces in view"))
                    })
                    .filter_map(|(_, node)| node.bounds())
                    .map(|bounds| {
                        egui::Rect::from_min_max(
                            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        if buttons.len() != 2 {
            failures.push(format!("{width} {cursor:?}: {} row buttons", buttons.len()));
        }
        for button in buttons {
            if button.intersects(*chip) {
                failures.push(format!(
                    "{width} {cursor:?}: the hidden control {chip:?} runs under a button {button:?}"
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The crowded row for review, written under `RSPICE_PICTURES` when it is
/// set: 760 px wide with cursor A placed, where the hidden control was cut.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_crowded_row_picture() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let mut state = crowded_pane();
    super::lanes::place_cursors(&mut state, Some(13.0e-6), None);
    let canvas = rspice_ui_kit::raster::render(egui::vec2(760.0, 420.0), |ui, _| {
        well(ui);
        show(ui, &mut state.results_parts());
    });
    std::fs::write(
        format!("{directory}/pane-crowded-row.png"),
        canvas.png(canvas.height()),
    )
    .expect("the picture is written");
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
        let frame = Tokens::get(&ctx).color.canvas_frame;
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

/// A pane whose traces the reader has hidden is its header row and nothing
/// below it: the shared axis starts right under it, and no hint fills a pane
/// height with a sentence.
#[test]
fn a_pane_with_every_trace_hidden_is_its_header_alone() {
    let ctx = themed_context(crate::ui::Theme::default());
    let mut state = review_strip(false);
    // The reader plots one of the currents and hides it again from the
    // legend.
    let (analysis, waveform, ..) = first_current(&mut state);
    toggle_visibility(&mut state.results_parts(), analysis, waveform);
    toggle_visibility(&mut state.results_parts(), analysis, waveform);
    assert_eq!(pane_units(&mut state), ["V", "A"]);
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

/// A pane of signals the dataset only hides by default, which nobody has
/// plotted, is not drawn: the review's currents open no "A" pane, as a
/// waveform viewer shows no strip for a signal until it is plotted. Ticked in
/// the signal browser, a current opens its pane; unticked there, it leaves
/// none behind; hidden from its pane's legend, it keeps the pane's header.
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
        ["V", "A"],
        "hidden from its legend, the current keeps its pane's header"
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
        text_rect(&output, "Nothing plotted — tick a signal in Signals").is_some(),
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

/// The trial the pointer is on strokes at the active run's width over its
/// siblings, which step back to an overlay's weight; a pointer on none of
/// them leaves every trial as it was.
#[test]
fn the_trial_under_the_pointer_is_highlighted_over_its_siblings() {
    let mut state = three_trials();
    let ctx = themed_context(crate::ui::Theme::default());
    settled(&ctx, &mut state);
    let hue = state.results.results.models.last_built().expect("models")[0].traces[0].signal_color;
    let (mut highlighted, mut plain) = (0, 0);
    for y in (60..(FRAME.y as i32 - 60)).step_by(4) {
        let pointer = egui::pos2(600.0, y as f32);
        let output = well_frame(&ctx, &mut state, vec![egui::Event::PointerMoved(pointer)]);
        let (mut strokes, _) = trial_strokes(&output, hue);
        strokes.sort_by(|left, right| left.0.total_cmp(&right.0));
        match strokes.as_slice() {
            [(_, false), (_, false), (width, true)] => {
                assert!(
                    *width > 1.0,
                    "a trial is stroked at 1 pt and lifted past it: {width}"
                );
                highlighted += 1;
            }
            [(_, true), (_, true), (_, true)] => plain += 1,
            other => panic!("a pointer at {y} drew the trials as {other:?}"),
        }
    }
    assert!(
        highlighted >= 3,
        "each trial was highlighted in turn: {highlighted}"
    );
    assert!(
        plain > highlighted,
        "a pointer between trials highlights none: {plain}"
    );
}

/// The C6 buck converter's Monte Carlo transient as the strip draws it: its
/// three trials of the output voltage and the inductor current, each signal
/// one family; then the same with each family's envelope under its members;
/// then twenty microseconds of the settled ripple with the pointer on one
/// trial of the inductor current. Written under `RSPICE_PICTURES` when it is
/// set.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_monte_carlo_family_picture() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus/c06-buck/deck/vdig.mc-tran.cir");
    let source = std::fs::read_to_string(&path).expect("the corpus deck is readable");
    let mut run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch_at(
        &source,
        &path,
        crate::state::SimulationRunLifecycle::Completed,
    );
    for analysis in &mut run.analyses {
        for waveform in &mut analysis.waveforms {
            let name = waveform.name.to_ascii_lowercase();
            waveform.visible = name == "vout" || name == "i(lbuck)";
        }
    }
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    let analysis = {
        let presentation = state.model.preferences.result_presentation_policy();
        let models = cached_models(
            &state.model.simulation,
            &mut state.results.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
        models[0].analysis_key
    };
    for (envelope, window, pointer, name) in [
        (false, None, None, "monte-carlo-family"),
        (true, None, None, "monte-carlo-envelope"),
        (
            true,
            Some((400e-6, 420e-6)),
            Some(egui::pos2(700.0, 570.0)),
            "monte-carlo-highlight",
        ),
    ] {
        state.results.results.session.show_family_envelope = envelope;
        set_shared_x_view(&mut state.results.results.session, analysis, 2, window);
        // The inductor current's pane reads the ripple, not the start-up.
        state
            .results
            .results
            .session
            .analysis_plot_view_pane_mut(super::super::super::ResultViewer::Waves, analysis, 1)
            .y = window.map(|_| (0.82, 0.98));
        state.results.results.models.invalidate();
        let pass = |ui: &mut egui::Ui, _: egui::Color32| {
            well(ui);
            show(ui, &mut state.results_parts());
        };
        let canvas = match pointer {
            Some(pointer) => rspice_ui_kit::raster::render_with_pointer(FRAME, pointer, pass),
            None => rspice_ui_kit::raster::render_themed(crate::ui::Theme::default(), FRAME, pass),
        };
        std::fs::write(
            format!("{directory}/{name}.png"),
            canvas.png(canvas.height()),
        )
        .expect("the picture is written");
    }
}
