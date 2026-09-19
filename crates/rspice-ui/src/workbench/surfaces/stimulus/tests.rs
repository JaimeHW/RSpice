//! What the instrument says, what its verbs do, and whether it fits.

use egui::{Pos2, Rect, vec2};

use super::*;
use crate::state::stimulus_library::definition::StimulusFamily;
use crate::state::stimulus_library::fixtures;
use crate::state::{Component, ComponentType, Point};
use crate::workbench::RSpiceApp;

/// The two viewports the instrument is held to: the workspace's smallest
/// supported window, and the ordinary laptop one.
const VIEWPORTS: [(&str, f32, f32); 2] = [("1024x640", 1024.0, 640.0), ("1440x900", 1440.0, 900.0)];

/// The centre column the shell leaves this stage at one viewport.
///
/// Read off `LayoutSpec`, which owns every one of these numbers, rather than
/// measured from a rendered frame: the gate is about whether the instrument
/// fits the space the shell gives it, and the shell decides that space from
/// exactly these fields.
fn stage_size(width: f32, height: f32) -> egui::Vec2 {
    use crate::workbench::design_system::ACTIVITY_RAIL_W;
    use crate::workbench::layout::LayoutSpec;
    use crate::workbench::state::WorkbenchState;

    let mut workbench = WorkbenchState::default();
    workbench.workspace = crate::workbench::state::Workspace::Stimulus;
    let layout = LayoutSpec::resolve(width, height, &workbench);
    let columns = width
        - if layout.show_activity_rail {
            ACTIVITY_RAIL_W
        } else {
            0.0
        }
        - if layout.show_navigator_dock {
            layout.navigator_width
        } else {
            0.0
        }
        - if layout.show_inspector_dock {
            layout.inspector_width
        } else {
            0.0
        };
    let rows = height
        - layout.title_bar_height
        - layout.toolbar_height
        - if layout.show_status_bar {
            layout.status_bar_height
        } else {
            0.0
        }
        // The collapsed console keeps its tab track, which is the one chrome
        // row the layout spec states only as a height it can grow to.
        - if layout.show_console_strip { 31.0 } else { 0.0 };
    vec2(columns.max(320.0), rows.max(240.0))
}

/// An application whose project holds the fixture library.
fn seeded(selection: &str) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.stimulus_library = fixtures::library();
    app.state.workbench.workspace = crate::workbench::state::Workspace::Stimulus;
    app.state.workbench.selected_stimulus_definition = Some(selection.to_owned());
    app
}

/// One placed source that has adopted a fixture definition.
fn adopter(state: &mut AppState, name: &str, instance: &str) -> u64 {
    let definition = state
        .workspace
        .stimulus_library
        .get(name)
        .cloned()
        .expect("fixture definition");
    let id = 7;
    let mut component = Component::new(id, definition.component_type(), Point::new(4, 4));
    component.name = instance.to_owned();
    definition.adopt_onto(&mut component).expect("adopt");
    state.schematic.components.push(component);
    id
}

/// Render the stage at one size and return every string it publishes.
fn published(state: &mut AppState, size: egui::Vec2) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, state));
        },
    );
    output
        .platform_output
        .accesskit_update
        .expect("AccessKit stage tree")
        .nodes
        .into_iter()
        .flat_map(|(_, node)| {
            [
                node.label().map(str::to_owned),
                node.value().map(str::to_owned),
            ]
        })
        .flatten()
        .collect()
}

/// Every text shape the stage painted, with the clip rectangle it landed in.
fn painted(state: &mut AppState, size: egui::Vec2) -> Vec<(String, Rect, Rect)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| show(ui, state));
        },
    );
    let mut lines = Vec::new();
    for clipped in &output.shapes {
        collect_text(&clipped.shape, clipped.clip_rect, &mut lines);
    }
    lines
}

fn collect_text(shape: &egui::epaint::Shape, clip: Rect, out: &mut Vec<(String, Rect, Rect)>) {
    match shape {
        egui::epaint::Shape::Text(text) => out.push((
            text.galley.job.text.clone(),
            Rect::from_min_size(text.pos, text.galley.size()),
            clip,
        )),
        egui::epaint::Shape::Vec(shapes) => {
            for shape in shapes {
                collect_text(shape, clip, out);
            }
        }
        _ => {}
    }
}

#[test]
fn an_empty_library_offers_the_one_verb_that_resolves_it() {
    let mut app = RSpiceApp::test_instance();
    let published = published(&mut app.state, vec2(900.0, 470.0));
    assert!(published.contains(&"No stimulus definitions".to_owned()));
    assert!(published.contains(&"New definition".to_owned()));
}

/// A library with definitions and no selection reads the first one rather
/// than showing an empty centre over a list.
#[test]
fn a_library_with_no_selection_opens_on_its_first_definition() {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.stimulus_library = fixtures::library();
    let published = published(&mut app.state, vec2(900.0, 470.0));
    assert_eq!(
        app.state.workbench.selected_stimulus_definition.as_deref(),
        Some("sensor_diff_1k")
    );
    assert!(published.contains(&"sensor_diff_1k".to_owned()));
}

/// Each band publishes something a reader can find it by.
#[test]
fn every_band_publishes_its_own_strings() {
    let mut app = seeded("sensor_diff_1k");
    let published = published(&mut app.state, vec2(1000.0, 470.0));
    for expected in [
        // Identity.
        "Name",
        "Family",
        "Quantity",
        "saved \u{b7} r1",
        "no adopters",
        "Apply",
        // Proof.
        "Fit",
        "Transient",
        "engine evaluator",
        // Program. Section heads publish their title uppercased, which is how
        // a reader hears them in every other workspace.
        "BIAS & SMALL-SIGNAL",
        "TRANSIENT SHAPE \u{b7} SIN",
        "Frequency (FREQ) *",
        // Realization.
        "Realization \u{b7} no adopters",
        // Audit.
        "valid",
    ] {
        assert!(
            published.iter().any(|text| text == expected),
            "the stage never published {expected:?}"
        );
    }
}

/// A defect is stated where a curve would have been, and the readouts that
/// still hold are kept.
#[test]
fn a_noise_definition_states_why_it_has_no_curve() {
    let mut app = seeded("supply_trnoise");
    let published = published(&mut app.state, vec2(1000.0, 470.0));
    assert!(
        published
            .iter()
            .any(|text| text.contains("TRNOISE") && text.contains("no waveform")),
        "{published:?}"
    );
    assert!(published.iter().any(|text| text.starts_with("RMS ")));
}

/// A waveform centred on zero states zero, not the rounding two float steps
/// leave behind.
///
/// The midpoint tick of a symmetric range is `minimum + (maximum - minimum) /
/// 2`, which for an AM carrier about 0 V came out as `-6.938893903907228e-16`
/// — below the engineering ladder's last decade, so the formatter fell back to
/// a bare exponent, and a label that long was painted off the left of the
/// stage. Both halves are pinned here: the value snaps to zero, and every tick
/// label is short enough to be a tick label.
#[test]
fn a_waveform_centred_on_zero_states_zero_on_its_axis() {
    let mut app = seeded("emi_am_150k");
    let size = stage_size(1024.0, 640.0);
    // Axis ticks are painted, not announced, so they are read off the frame's
    // own text shapes rather than out of the accessibility tree.
    let painted = painted(&mut app.state, size);
    let texts = painted
        .iter()
        .map(|(text, ..)| text.as_str())
        .collect::<Vec<_>>();
    assert!(
        texts.contains(&"0"),
        "the axis never stated zero: {texts:?}"
    );
    for (text, rect, _) in &painted {
        assert!(
            !text.contains("e-1") && !text.contains("e-0"),
            "a tick fell off the engineering ladder: {text:?}"
        );
        assert!(
            rect.left() >= -0.5,
            "{text:?} is painted left of the stage at {rect:?}"
        );
    }
}

/// A window holding five hundred cycles says so where the reader is looking,
/// and the plot announces itself as a band rather than as a waveform. A window
/// that carries its shape says nothing, because there is nothing to warn about.
#[test]
fn a_many_cycle_window_announces_its_band_and_a_curve_does_not() {
    let mut app = many_cycle_train();
    let band = published(&mut app.state, stage_size(1024.0, 640.0));
    assert!(
        band.iter().any(|text| text == "500 cycles \u{b7} envelope"),
        "the strip never stated what it is drawing: {band:?}"
    );
    assert!(
        band.iter()
            .any(|text| text.contains("Engine-evaluated waveform of")
                && text.contains("500 cycles \u{b7} envelope")),
        "the plot announced itself as a waveform: {band:?}"
    );

    let mut app = seeded("sensor_diff_1k");
    let curve = published(&mut app.state, stage_size(1024.0, 640.0));
    assert!(
        !curve.iter().any(|text| text.contains("envelope")),
        "a curve called itself a band: {curve:?}"
    );
}

/// A pulse train with five hundred cycles in the plan's transient window, which
/// is the shape a uniform grid cannot draw.
fn many_cycle_train() -> RSpiceApp {
    let mut app = seeded("bridge_cal_step");
    for (field, value) in [("tr", "1n"), ("tf", "1n"), ("pw", "1u"), ("per", "2u")] {
        crate::workbench::app::actions::stimulus::edit_field(&mut app.state, field, value);
    }
    app.state.workbench.stimulus_editor.span = crate::workbench::state::PreviewSpan::Transient;
    app
}

#[test]
fn a_family_switch_resets_the_shape_parameters_and_undo_restores_them() {
    let mut app = seeded("bridge_cal_step");
    let before = app
        .state
        .workspace
        .stimulus_library
        .get("bridge_cal_step")
        .expect("fixture")
        .params
        .clone();

    crate::workbench::app::actions::stimulus::edit_family(&mut app.state, StimulusFamily::Sin);
    let draft = app
        .state
        .workbench
        .stimulus_editor
        .draft("bridge_cal_step")
        .expect("draft");
    assert_eq!(
        draft.working().component_type(),
        ComponentType::VoltageSourceSin
    );
    assert_eq!(draft.working().params, "");

    assert!(crate::workbench::app::actions::stimulus::undo_draft(
        &mut app.state
    ));
    let draft = app
        .state
        .workbench
        .stimulus_editor
        .draft("bridge_cal_step")
        .expect("draft");
    assert_eq!(draft.working().params, before);
    assert!(!draft.is_dirty());
}

/// Apply is blocked while a refusal stands and available once it is fixed.
#[test]
fn apply_is_blocked_by_an_error_and_available_after_the_fix() {
    let mut app = seeded("bridge_cal_step");
    crate::workbench::app::actions::stimulus::edit_name(&mut app.state, "sensor_diff_1k");
    let stage = resolve(&mut app.state).expect("stage");
    assert!(stage.errors() > 0);
    assert!(stage.apply_block().is_some());

    crate::workbench::app::actions::stimulus::edit_name(&mut app.state, "bridge_cal_step_2");
    let stage = resolve(&mut app.state).expect("stage");
    assert_eq!(stage.errors(), 0, "{:?}", stage.findings);
    assert!(stage.apply_block().is_none());
}

/// A rename is published as a rename of the library's record, and every
/// adopter keeps the receipt it took. Nothing reads "definition removed"
/// because of a corrected spelling, and nothing reaches into the design to
/// make that true — which is what keeps an adopter on an unloaded sheet as
/// safe as the one on this one.
#[test]
fn applying_a_rename_repoints_every_adopter() {
    let mut app = seeded("bridge_cal_step");
    let id = adopter(&mut app.state, "bridge_cal_step", "V1");
    crate::workbench::app::actions::stimulus::edit_name(&mut app.state, "bridge_cal_pulse");
    crate::workbench::app::actions::stimulus::apply_draft(&mut app.state);

    assert_eq!(
        app.state
            .workspace
            .stimulus_library
            .definitions()
            .iter()
            .filter(|definition| definition.name().starts_with("bridge_cal"))
            .count(),
        1,
        "a rename moves one record rather than leaving two"
    );
    assert_eq!(
        app.state
            .workspace
            .stimulus_library
            .get("bridge_cal_step")
            .map(|definition| definition.name()),
        Some("bridge_cal_pulse"),
        "the name an adopter copied still resolves to the record"
    );
    let definition = app
        .state
        .workspace
        .stimulus_library
        .get("bridge_cal_pulse")
        .expect("the renamed definition");
    assert_eq!(definition.revision(), 2);
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the adopter");
    assert_eq!(
        component
            .stimulus_provenance
            .as_ref()
            .map(|provenance| provenance.definition.as_str()),
        Some("bridge_cal_step"),
        "the receipt keeps the name it copied; the library resolves it"
    );
    assert_eq!(
        app.state
            .workspace
            .stimulus_library
            .provenance_state(component)
            .label(),
        "behind \u{b7} library r2"
    );
}

/// The instrument fits the centre column at both viewports, for every family.
///
/// Three things are held, and the difference between them is deliberate:
///
/// - No text is cut sideways. A band elides what does not fit, so a galley
///   running past the side of its clip is a label someone forgot to elide.
/// - Nothing is painted below the stage. The instrument never scrolls as a
///   whole, so text past the bottom would be text nobody can reach.
/// - Text may be cut vertically, but only by the viewport of a band that
///   scrolls internally — the program columns, the point table, the
///   realization list and the audit strip. A row scrolled out of view has an
///   empty clip and paints nothing, so it is not a line on the screen at all.
#[test]
fn every_family_fits_the_stage_at_both_supported_viewports() {
    for (viewport, width, height) in VIEWPORTS {
        let size = stage_size(width, height);
        for name in fixtures::names() {
            let mut app = seeded(name);
            for (text, rect, clip) in painted(&mut app.state, size) {
                if !clip.is_positive() {
                    continue;
                }
                assert!(
                    rect.left() >= clip.left() - 0.5 && rect.right() <= clip.right() + 0.5,
                    "{viewport} {name}: {text:?} at {rect:?} is cut by {clip:?}"
                );
                // What shows of a galley is its intersection with its clip,
                // so a clip inside the stage is text inside the stage.
                assert!(
                    clip.bottom() <= size.y + 0.5 && clip.right() <= size.x + 0.5,
                    "{viewport} {name}: {text:?} is painted through {clip:?}, past the stage"
                );
            }
        }
    }
}

/// The centre column the gate measures, stated so a layout change that shrinks
/// it fails here rather than in a raster nobody looks at.
#[test]
fn the_measured_stage_is_the_column_the_shell_leaves() {
    let smallest = stage_size(1024.0, 640.0);
    let laptop = stage_size(1440.0, 900.0);
    assert!(smallest.x >= 320.0 && smallest.y >= 240.0);
    assert!(laptop.x > smallest.x && laptop.y > smallest.y);
}

/// The proof surface is evaluated when something it depends on changes, and
/// not otherwise.
#[test]
fn a_frame_that_changed_nothing_does_not_re_evaluate_the_waveform() {
    let mut app = seeded("sensor_diff_1k");
    let size = stage_size(1024.0, 640.0);
    let _ = published(&mut app.state, size);
    let after_first = app.state.workbench.stimulus_editor.evaluations();
    let _ = published(&mut app.state, size);
    assert_eq!(
        app.state.workbench.stimulus_editor.evaluations(),
        after_first
    );

    crate::workbench::app::actions::stimulus::edit_field(&mut app.state, "va", "9m");
    let _ = published(&mut app.state, size);
    assert_eq!(
        app.state.workbench.stimulus_editor.evaluations(),
        after_first + 1
    );
}

/// Write one render per state a reviewer has to look at.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn print_stimulus_renders_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    for (viewport, width, height) in VIEWPORTS {
        let size = stage_size(width, height);
        for (name, seed) in review_states() {
            let mut app = seed();
            let canvas = crate::ui::raster::render(size, |ui, background| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(background))
                    .show(ui, |ui| show(ui, &mut app.state));
            });
            let rendered_height = canvas.content_height().max(1);
            let path = directory.join(format!("stimulus-{name}-{viewport}.png"));
            std::fs::write(&path, canvas.png(rendered_height)).expect("write render");
            writeln!(
                report,
                "{} {}x{}",
                path.display(),
                canvas.width(),
                rendered_height
            )
            .expect("write raster report");
        }
    }
}

/// One reviewed state: the name its render is filed under, and how to build it.
type ReviewState = (&'static str, fn() -> RSpiceApp);

/// The states a review covers: one per layout the stage has a distinct shape
/// for, plus the two that cannot be reached by picking a definition.
fn review_states() -> Vec<ReviewState> {
    vec![
        ("empty", RSpiceApp::test_instance as fn() -> RSpiceApp),
        ("sin", || seeded("sensor_diff_1k")),
        ("pulse-edges-only", || {
            let mut app = seeded("bridge_cal_step");
            crate::workbench::app::actions::stimulus::edit_field(&mut app.state, "pw", "");
            crate::workbench::app::actions::stimulus::edit_field(&mut app.state, "per", "");
            app
        }),
        ("pwl", || {
            let mut app = seeded("vdd_ramp_1ms");
            // Opening the draft is what gives the definition a slot to hold a
            // point selection in.
            let _ = resolve(&mut app.state);
            app.state
                .workbench
                .stimulus_editor
                .select_point("vdd_ramp_1ms", Some(1));
            app
        }),
        ("pulse-many-cycles", many_cycle_train as fn() -> RSpiceApp),
        ("pwl-file", || seeded("bridge_meas_step")),
        ("trnoise", || seeded("supply_trnoise")),
        ("draft-with-error", || {
            let mut app = seeded("bridge_cal_step");
            crate::workbench::app::actions::stimulus::edit_name(&mut app.state, "sensor_diff_1k");
            app
        }),
    ]
}

/// The number a tick label states, read back the way a reader would.
fn tick_value(text: &str) -> Option<f64> {
    let digits = text.strip_suffix('V').unwrap_or(text);
    crate::quantity::parse_engineering_value(&digits.replace('\u{b5}', "u")).ok()
}

/// The value axis of one painted frame: `(value, y)` for every label in the
/// right-aligned column left of the plot.
fn value_axis(painted: &[(String, Rect, Rect)]) -> Vec<(f64, f32)> {
    let mut columns = std::collections::BTreeMap::<i32, Vec<(f64, f32)>>::new();
    for (text, rect, _) in painted {
        let Some(value) = tick_value(text) else {
            continue;
        };
        // Labels that end at the same x are one right-aligned column.
        columns
            .entry((rect.right() * 2.0).round() as i32)
            .or_default()
            .push((value, rect.center().y));
    }
    columns
        .into_values()
        .filter(|ticks| ticks.iter().any(|(_, y)| (y - ticks[0].1).abs() > 1.0))
        .max_by_key(Vec::len)
        .unwrap_or_default()
}

/// A value label is drawn at the value it states.
///
/// The first version of the axis labelled the padded ends of the range and let
/// the formatter round them, so a 2 mV sine read `2m` at a line 2.48 mV up and
/// a ramp to 5 V read `-1`, `2`, `6`. Every label is held here to be a number
/// on one straight scale with every other: equal steps in value are equal
/// steps on the screen, which a rounded label at an unrounded position breaks.
#[test]
fn every_value_tick_is_drawn_at_the_value_it_states() {
    for name in [
        "sensor_diff_1k",
        "vdd_ramp_1ms",
        "bridge_cal_step",
        "emi_am_150k",
    ] {
        let mut app = seeded(name);
        let painted = painted(&mut app.state, stage_size(1440.0, 900.0));
        let mut axis = value_axis(&painted);
        axis.sort_by(|left, right| left.0.total_cmp(&right.0));
        assert!(axis.len() >= 3, "{name}: the axis states {axis:?}");
        let (first, last) = (axis[0], axis[axis.len() - 1]);
        let scale = (last.1 - first.1) / (last.0 - first.0) as f32;
        for (value, y) in &axis {
            let expected = first.1 + (*value - first.0) as f32 * scale;
            assert!(
                (y - expected).abs() <= 0.75,
                "{name}: the label for {value} is at y={y}, and a straight scale puts it at \
                 {expected}: {axis:?}"
            );
        }
    }
}

/// On a laptop the program shows every field it has: nothing is left to a
/// scroll bar while the plot above it holds four hundred points of height.
#[test]
fn a_laptop_stage_shows_every_field_without_scrolling() {
    for name in ["sensor_diff_1k", "bridge_cal_step", "supply_trnoise"] {
        let mut app = seeded(name);
        let size = stage_size(1440.0, 900.0);
        let painted = painted(&mut app.state, size);
        let stage = resolve(&mut app.state).expect("stage");
        let sheet = app
            .state
            .property_registry
            .get(stage.working.component_type())
            .expect("sheet");
        for definition in program::shape_fields(sheet, &stage) {
            let shown = painted.iter().find(|(text, _, clip)| {
                clip.is_positive()
                    && (text.starts_with(definition.display_name.as_str())
                        || definition
                            .display_name
                            .strip_suffix(')')
                            .and_then(|inner| inner.rsplit('(').next())
                            .is_some_and(|word| {
                                text == word || text.strip_suffix(" *") == Some(word)
                            }))
            });
            let (text, rect, clip) = shown.unwrap_or_else(|| {
                panic!(
                    "{name}: no row was painted for {:?}",
                    definition.display_name
                )
            });
            assert!(
                clip.contains_rect(*rect),
                "{name}: {text:?} at {rect:?} is cut by {clip:?}"
            );
        }
    }
}

/// The point table and the scalar fields under it are two blocks, one above
/// the other, at either size.
#[test]
fn a_point_table_never_runs_under_the_fields_below_it() {
    for (viewport, width, height) in VIEWPORTS {
        let mut app = seeded("vdd_ramp_1ms");
        let painted = painted(&mut app.state, stage_size(width, height));
        let visible = |wanted: &dyn Fn(&str) -> bool| {
            painted
                .iter()
                .filter(|(text, _, clip)| clip.is_positive() && wanted(text))
                .map(|(_, rect, _)| *rect)
                .collect::<Vec<_>>()
        };
        let rows = visible(&|text| text == "1" || text == "2" || text == "3");
        let fields = visible(&|text| text.starts_with("Delay") || text == "TD");
        let last_row = rows
            .iter()
            .map(Rect::bottom)
            .fold(f32::NEG_INFINITY, f32::max);
        for field in &fields {
            assert!(
                field.top() >= last_row - 0.5,
                "{viewport}: a field at {field:?} starts above the table's last row at {last_row}"
            );
        }
        assert!(!rows.is_empty(), "{viewport}: no point row was painted");
    }
}

/// The identity band says a fact whole or not at all.
#[test]
fn the_identity_band_never_cuts_a_fact_short() {
    for name in fixtures::names() {
        let mut app = seeded(name);
        for (text, rect, _) in painted(&mut app.state, stage_size(1024.0, 640.0)) {
            assert!(
                !(rect.top() < IDENTITY_HEIGHT && text.contains('\u{2026}')),
                "{name}: the identity band elided {text:?}"
            );
        }
    }
}

/// Breakpoint labels a pixel apart are one label, not two painted over each
/// other.
#[test]
fn guide_labels_that_would_touch_are_merged() {
    let mut app = seeded("bridge_cal_step");
    let painted = painted(&mut app.state, stage_size(1024.0, 640.0));
    let labels = painted
        .iter()
        .filter(|(text, ..)| {
            text.split('\u{b7}')
                .all(|part| matches!(part, "TD" | "TR" | "PW" | "TF" | "PER"))
        })
        .collect::<Vec<_>>();
    assert!(!labels.is_empty(), "the pulse painted no breakpoint label");
    for (index, (text, rect, _)) in labels.iter().enumerate() {
        for (other, other_rect, _) in &labels[index + 1..] {
            assert!(
                !rect.intersects(*other_rect),
                "{text:?} at {rect:?} is painted over {other:?} at {other_rect:?}"
            );
        }
    }
}

/// One error is `1 error`.
#[test]
fn a_count_of_one_is_singular() {
    let mut app = seeded("bridge_cal_step");
    crate::workbench::app::actions::stimulus::edit_name(&mut app.state, "sensor_diff_1k");
    let published = published(&mut app.state, vec2(1000.0, 470.0));
    assert!(
        published.iter().any(|text| text == "1 error"),
        "{published:?}"
    );
    assert!(
        published.iter().any(|text| text == "draft \u{b7} 1 error"),
        "{published:?}"
    );
    for text in &published {
        assert!(
            !text.contains("(s)") && !text.contains("(ies)"),
            "{text:?} hedges its own count"
        );
    }
}
