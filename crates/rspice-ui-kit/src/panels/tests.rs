//! Tests for the workbench visual primitives.

use super::*;

#[test]
fn workspace_title_row_uses_visible_pane_not_offscreen_content_extent() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut measured_width = f32::INFINITY;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(900.0, 700.0))),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let content_rect =
                    Rect::from_min_max(egui::pos2(306.0, 80.0), egui::pos2(1_206.0, 700.0));
                let mut surface = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
                surface.set_clip_rect(Rect::from_min_max(
                    content_rect.min,
                    egui::pos2(900.0, 700.0),
                ));
                workspace_title_row(&mut surface, |row| {
                    measured_width = row.available_width();
                });
            });
        },
    );

    assert!(
        measured_width <= 578.0,
        "title content escaped the 594 px visible pane: {measured_width}"
    );
}

fn painted_text_rect(shape: &Shape, wanted: &str) -> Option<Rect> {
    match shape {
        Shape::Text(text) if text.galley.text() == wanted => {
            Some(Rect::from_min_size(text.pos, text.galley.size()))
        }
        Shape::Vec(shapes) => shapes
            .iter()
            .find_map(|shape| painted_text_rect(shape, wanted)),
        _ => None,
    }
}

/// A press on an action row's title is the row's click. egui hands a
/// press to the topmost widget under it, so a selectable label laid over
/// the row takes every press that lands on the text.
#[test]
fn an_action_row_takes_a_click_that_lands_on_its_title() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut title = Rect::NOTHING;
    let mut clicked = false;
    // Two passes to lay the row out against the fonts, then one that
    // presses and releases on the centre of the painted title.
    for pass in 0..3 {
        let at = title.center();
        let events = if pass == 2 {
            vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                },
            ]
        } else {
            Vec::new()
        };
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(480.0, 200.0))),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    clicked |= action_row(
                        ui,
                        "New project",
                        "Analog, RF or mixed-signal · blank top cell",
                        WorkbenchIcon::Add,
                        true,
                    )
                    .clicked();
                });
            },
        );
        if pass < 2 {
            title = output
                .shapes
                .iter()
                .find_map(|clipped| painted_text_rect(&clipped.shape, "New project"))
                .unwrap_or(Rect::NOTHING);
        }
    }
    assert!(title.is_positive(), "the title was never painted");
    assert!(
        clicked,
        "a press on the title at {title:?} did not click the row"
    );
}

/// A painted label that truncates keeps its full text one hover away, as
/// a truncated `egui::Label` does: the text is painted once in the slot
/// and once more in the tooltip.
#[test]
fn a_truncated_painted_label_shows_its_full_text_on_hover() {
    const FULL: &str = "//lab-server/projects/analog/precision-afe-front-end.rspiceproj";
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.all_styles_mut(|style| {
        style.interaction.tooltip_delay = 0.0;
        style.interaction.show_tooltips_only_when_still = false;
    });
    let mut slot = Rect::NOTHING;
    let mut output = None;
    // One pass to lay the slot out, then hover its centre.
    for pass in 0..4 {
        let events = if pass == 0 {
            Vec::new()
        } else {
            vec![egui::Event::PointerMoved(slot.center())]
        };
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(480.0, 200.0))),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.allocate_ui(Vec2::new(120.0, 20.0), |ui| {
                        slot = painted_label(ui, FULL, egui::TextWrapMode::Truncate).rect;
                    });
                });
            },
        ));
    }
    let copies = output
        .expect("four passes ran")
        .shapes
        .iter()
        .filter(|clipped| painted_text_rect(&clipped.shape, FULL).is_some())
        .count();
    assert_eq!(copies, 2, "the elided slot and its tooltip");
}

fn shape_contains_text(shape: &Shape) -> bool {
    match shape {
        Shape::Text(_) => true,
        Shape::Vec(shapes) => shapes.iter().any(shape_contains_text),
        _ => false,
    }
}

#[test]
fn semantic_status_marks_are_font_independent_vector_geometry() {
    let ctx = egui::Context::default();
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            for mark in [StatusMark::Warning, StatusMark::Failure] {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                paint_status_mark(ui.painter(), rect, mark, Color32::WHITE);
            }
        });
    });

    let status_shapes = output
        .shapes
        .iter()
        .filter(|shape| {
            matches!(
                &shape.shape,
                Shape::Path(_) | Shape::LineSegment { .. } | Shape::Circle(_)
            )
        })
        .collect::<Vec<_>>();
    // The warning triangle.
    assert_eq!(
        status_shapes
            .iter()
            .filter(|shape| matches!(&shape.shape, Shape::Path(_)))
            .count(),
        1
    );
    // The failure cross, which is two strokes.
    assert_eq!(
        status_shapes
            .iter()
            .filter(|shape| matches!(&shape.shape, Shape::LineSegment { .. }))
            .count(),
        2
    );
    // No mark is a dot: the kit has no neutral mark to paint one with.
    assert_eq!(
        status_shapes
            .iter()
            .filter(|shape| matches!(&shape.shape, Shape::Circle(_)))
            .count(),
        0
    );
    assert!(
        output
            .shapes
            .iter()
            .all(|shape| !shape_contains_text(&shape.shape))
    );
}

#[test]
fn section_header_uses_measured_copy_before_eliding() {
    use section::{SECTION_HEADER_COLUMN_GAP, section_header_column_widths, section_header_wraps};
    let (title, meta) = section_header_column_widths(210.0, 47.0, 91.0, true);
    assert_eq!(meta, 91.0);
    assert!((title + SECTION_HEADER_COLUMN_GAP + meta - 210.0).abs() <= 0.001);
    assert!(title >= 47.0);
    assert!(!section_header_wraps(210.0, 47.0, 91.0, true));
}

#[test]
fn property_controls_and_section_headers_expose_their_full_accessible_names() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut value = "R1".to_owned();
    let mut tunable_value = "1k".to_owned();
    let mut selected = "tt".to_owned();
    let read_only_value = "vendor_analog/OPA189/precision_zero_drift";
    let options = vec![
        ("tt".to_owned(), "Typical".to_owned()),
        ("ff".to_owned(), "Fast".to_owned()),
    ];

    let nodes = ctx
        .run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(160.0);
                section_header(
                    ui,
                    "Long engineering section title",
                    Some("complete metadata"),
                );
                property_row_input(ui, "Instance", &mut value, false);
                property_row_input_action(
                    ui,
                    "Value",
                    &mut tunable_value,
                    false,
                    PropertyRowAction {
                        icon: WorkbenchIcon::Sliders,
                        label: "Tune value",
                        enabled: true,
                        disabled_reason: None,
                    },
                );
                property_row_combo(
                    ui,
                    "Model section",
                    "accessible-model-section",
                    &mut selected,
                    &options,
                    true,
                );
                property_row(ui, "Library cell", read_only_value);
            });
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Heading
            && node.label() == Some("Long engineering section title, complete metadata")
            && node.level() == Some(3)
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput && node.label() == Some("Instance")
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput && node.label() == Some("Value")
    }));
    assert!(
        nodes
            .iter()
            .any(|(_, node)| node.label() == Some("Model section"))
    );
    assert!(nodes.iter().any(|(_, node)| {
        node.label() == Some("Library cell") && node.value() == Some(read_only_value)
    }));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn empty_state_recovery_actions_are_exposed_as_named_buttons() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    ctx.enable_accesskit();

    let nodes = ctx
        .run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(900.0, 700.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    empty_state_with_actions(
                        ui,
                        WorkbenchIcon::Code,
                        "No project source workspace",
                        "Create or import one to continue.",
                        |ui| {
                            let _ = ui.button("Create source workspace…");
                            let _ = ui.button("Import root source…");
                        },
                    );
                });
            },
        )
        .platform_output
        .accesskit_update
        .expect("AccessKit empty-state tree")
        .nodes;

    for label in ["Create source workspace…", "Import root source…"] {
        let buttons = nodes
            .iter()
            .filter(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            })
            .count();
        assert_eq!(buttons, 1, "{label} is published {buttons} times");
    }
}

#[test]
fn long_read_only_cells_do_not_reflow_property_rows() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut short_height = 0.0;
    let mut long_value_height = 0.0;
    let mut long_label_height = 0.0;

    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(180.0);
            short_height = property_row(ui, "View", "schematic").rect.height();
            long_value_height = property_row(
                ui,
                "Library cell",
                "vendor_analog/OPA189/precision_zero_drift/amplifier_symbol",
            )
            .rect
            .height();
            long_label_height = property_row(
                ui,
                "Inherited technology and model binding authority",
                "project",
            )
            .rect
            .height();
        });
    });

    assert_eq!(short_height, PANEL_ROW_H);
    assert_eq!(long_value_height, short_height);
    assert_eq!(long_label_height, short_height);
}

#[test]
fn wide_property_rows_stop_pushing_the_value_column_rightwards() {
    // A two-column dialog that collapses at its 820 pt breakpoint hands its
    // aside the full dialog width. Without the cap each value would start
    // ~40% across, leaving a gap that reads as unfinished layout.
    let inner = 780.0 - 2.0 * PANEL_INSET;
    let (label_column, gap, value_column) = property_row_columns(inner);
    assert_eq!(label_column, PROPERTY_LABEL_MAX_W);
    assert!(label_column < (inner - PROPERTY_ROW_GAP) * PROPERTY_LABEL_FRACTION);
    assert!((label_column + gap + value_column - inner).abs() <= 0.001);

    // The cap is an upper bound, not a fixed column: the label column never
    // grows past it however wide the row gets.
    assert_eq!(property_row_columns(2_000.0).0, PROPERTY_LABEL_MAX_W);
}

/// Every section header, and the stacked field label, paints its words
/// exactly as the caller wrote them: sentence case stays sentence case, an
/// acronym stays an acronym, and no letter tracking spreads them.
#[test]
fn every_section_header_and_field_label_paints_its_title_verbatim() {
    fn texts(shape: &Shape, out: &mut Vec<egui::epaint::TextShape>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| texts(shape, out)),
            Shape::Text(text) => out.push(text.clone()),
            _ => {}
        }
    }
    const TITLE: &str = "Simulation parameters";
    type Header = fn(&mut Ui);
    let headers: [(&str, Header); 5] = [
        ("section_header", |ui| section_header(ui, TITLE, Some("3"))),
        ("section_header_explained", |ui| {
            section_header_explained(ui, TITLE, "3", "Three runs")
        }),
        ("section_header_action", |ui| {
            let _ = section_header_action(ui, TITLE, "Properties\u{2026}", true);
        }),
        ("code_inspector_section", |ui| {
            code_inspector_section(ui, TITLE, Some("3"), None, |_| {})
        }),
        ("widgets::field_label", |ui| {
            crate::widgets::field_label(ui, TITLE, |_| {})
        }),
    ];
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    for (name, header) in headers {
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(420.0);
                header(ui);
            });
        });
        let mut painted = Vec::new();
        for clipped in &output.shapes {
            texts(&clipped.shape, &mut painted);
        }
        let title = painted
            .iter()
            .find(|text| text.galley.text() == TITLE)
            .unwrap_or_else(|| {
                panic!(
                    "{name} did not paint {TITLE:?}; painted {:?}",
                    painted
                        .iter()
                        .map(|text| text.galley.text())
                        .collect::<Vec<_>>()
                )
            });
        assert!(
            title
                .galley
                .job
                .sections
                .iter()
                .all(|section| section.format.extra_letter_spacing == 0.0),
            "{name} tracks its title"
        );
    }
}

#[test]
fn card_content_remains_vertical_inside_a_horizontal_parent() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut first = egui::Rect::NOTHING;
    let mut second = egui::Rect::NOTHING;

    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                card(ui, "Status", |ui| {
                    first = ui.label("first").rect;
                    second = ui.label("second").rect;
                });
            });
        });
    });

    assert!(second.top() >= first.bottom());
}

#[test]
fn a_long_path_keeps_its_root_and_the_folder_it_names() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let path = r"C:\Users\Example\AppData\Local\RSpice\.rspice-recovery";
    let font = crate::theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut shortened = Vec::new();
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let full = ui
                .painter()
                .layout_no_wrap(path.to_owned(), font.clone(), Color32::WHITE)
                .size()
                .x;
            for width in [full + 1.0, full * 0.7, full * 0.45] {
                shortened.push(elide_path(ui, path, &font, width));
            }
        });
    });

    assert_eq!(shortened[0], path, "a path that fits is left alone");
    for elided in &shortened[1..] {
        assert!(elided.starts_with("C:\\\u{2026}\\"), "{elided}");
        assert!(elided.ends_with(r"\.rspice-recovery"), "{elided}");
    }
    assert!(shortened[1].len() > shortened[2].len());
}

/// The text shape that paints `wanted`, wherever it is nested.
fn painted_text(shape: &Shape, wanted: &str) -> Option<egui::epaint::TextShape> {
    match shape {
        Shape::Text(text) if text.galley.text() == wanted => Some(text.clone()),
        Shape::Vec(shapes) => shapes.iter().find_map(|shape| painted_text(shape, wanted)),
        _ => None,
    }
}

/// Every stroke colour a pass painted outside text: the marks.
fn stroke_colors(shapes: &[egui::epaint::ClippedShape]) -> Vec<Color32> {
    fn walk(shape: &Shape, colors: &mut Vec<Color32>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, colors)),
            Shape::LineSegment { stroke, .. } => colors.push(stroke.color),
            Shape::Path(path) => {
                if let egui::epaint::ColorMode::Solid(color) = path.stroke.color {
                    colors.push(color);
                }
            }
            _ => {}
        }
    }
    let mut colors = Vec::new();
    for shape in shapes {
        walk(&shape.shape, &mut colors);
    }
    colors
}

/// A row with no problem is a plain property row to the pixel; a row with one
/// keeps its value in primary text and moves it right by the mark's advance,
/// and the mark is the only colour on it.
#[test]
fn a_problem_row_keeps_its_value_neutral_and_only_marks_a_problem() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let palette = Tokens::default().color;
    let render = |add: &dyn Fn(&mut Ui)| {
        ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(280.0);
                add(ui);
            });
        })
        .shapes
    };
    let plain = render(&|ui| {
        property_row(ui, "Terminal", "unplaced");
    });
    let clear = render(&|ui| {
        property_row_problem(ui, "Terminal", "unplaced", None);
    });
    let value = |shapes: &[egui::epaint::ClippedShape]| {
        shapes
            .iter()
            .find_map(|shape| painted_text(&shape.shape, "unplaced"))
            .expect("the value is painted")
    };
    assert_eq!(value(&clear).pos, value(&plain).pos);
    assert_eq!(value(&clear).fallback_color, palette.text);
    assert!(stroke_colors(&clear).is_empty(), "no problem, no mark");

    for problem in [Problem::Warning, Problem::Failure] {
        let marked = render(&|ui| {
            property_row_problem(ui, "Terminal", "unplaced", Some(problem));
        });
        let shown = value(&marked);
        assert_eq!(shown.fallback_color, palette.text, "{problem:?}");
        assert!(
            (shown.pos.x - value(&plain).pos.x - crate::widgets::PROBLEM_MARK_ADVANCE).abs() < 0.5,
            "{problem:?} value at {:?}",
            shown.pos
        );
        let marks = stroke_colors(&marked);
        assert!(!marks.is_empty(), "{problem:?} painted no mark");
        assert!(
            marks.iter().all(|color| *color == problem.color(&palette)),
            "{problem:?} mark {marks:?}"
        );
    }
}

/// A code inspector section's status is faint metadata with no dot in front of
/// it; a problem adds its mark, and is the only colour in the header. The
/// title is painted as written.
#[test]
fn a_code_inspector_section_paints_no_status_dot() {
    fn walk(shape: &Shape, circles: &mut usize, texts: &mut Vec<egui::epaint::TextShape>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, circles, texts)),
            Shape::Circle(_) => *circles += 1,
            Shape::Text(text) => texts.push(text.clone()),
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let palette = Tokens::default().color;
    for problem in [None, Some(Problem::Warning), Some(Problem::Failure)] {
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                code_inspector_section(ui, "Diagnostics", Some("2 advisories"), problem, |_| {});
                code_inspector_section(ui, "Build targets", Some("3"), None, |_| {});
            });
        });
        let mut circles = 0;
        let mut texts = Vec::new();
        for clipped in &output.shapes {
            walk(&clipped.shape, &mut circles, &mut texts);
        }
        assert_eq!(circles, 0, "{problem:?}: a status dot was painted");
        for status in ["2 advisories", "3"] {
            let painted = texts
                .iter()
                .find(|text| text.galley.text() == status)
                .unwrap_or_else(|| panic!("{status:?} not painted"));
            assert_eq!(painted.fallback_color, palette.text_faint, "{status:?}");
        }
        assert!(texts.iter().any(|text| text.galley.text() == "Diagnostics"));
        // Every stroke but the section's own divider.
        let marks = stroke_colors(&output.shapes)
            .into_iter()
            .filter(|color| *color != palette.border)
            .collect::<Vec<_>>();
        match problem {
            None => assert!(marks.is_empty(), "a clear header paints a mark"),
            Some(problem) => {
                assert!(!marks.is_empty(), "{problem:?} painted no mark");
                assert!(marks.iter().all(|color| *color == problem.color(&palette)));
            }
        }
    }
}

/// Every text shape a pass painted, wherever it is nested.
fn all_texts(shapes: &[egui::epaint::ClippedShape]) -> Vec<egui::epaint::TextShape> {
    fn walk(shape: &Shape, out: &mut Vec<egui::epaint::TextShape>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            Shape::Text(text) => out.push(text.clone()),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for shape in shapes {
        walk(&shape.shape, &mut out);
    }
    out
}

/// The ink a text shape paints in: its override, else its first stated
/// section colour, else the painter's fallback.
fn ink(text: &egui::epaint::TextShape) -> Color32 {
    text.override_text_color.unwrap_or_else(|| {
        text.galley
            .job
            .sections
            .iter()
            .map(|section| section.format.color)
            .find(|color| *color != Color32::PLACEHOLDER)
            .unwrap_or(text.fallback_color)
    })
}

#[cfg(not(target_arch = "wasm32"))]
/// A typed value longer than its field ends in an ellipsis while the field
/// rests, inside the field, as a read-only value does. Being edited it is
/// laid out whole, so the caret indexes the text as typed.
#[test]
fn a_cut_typed_value_ends_in_an_ellipsis_until_it_is_edited() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let long = "Laboratory test panel for the bias network".to_owned();
    let mut typed = long.clone();
    let pass = |typed: &mut String| {
        let mut field = None;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(240.0, 200.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        field = Some(property_row_input(ui, "Size name", typed, false));
                    });
            },
        );
        let field = field.expect("the row was shown");
        let shown = output
            .shapes
            .iter()
            .find_map(|clipped| {
                painted_text(&clipped.shape, &long).map(|text| (text, clipped.clip_rect))
            })
            .expect("the value was painted");
        (field, shown)
    };
    let (field, (resting, clip)) = pass(&mut typed);
    assert!(
        resting.galley.elided,
        "a resting cut value ends in an ellipsis"
    );
    let painted_right = resting.pos.x + resting.galley.size().x;
    assert!(
        painted_right <= clip.right() + 0.5 && painted_right <= field.rect.right(),
        "the ellipsis is inside the field: {painted_right} against {clip:?}"
    );
    assert_eq!(ink(&resting), Tokens::default().color.text);

    ctx.memory_mut(|memory| memory.request_focus(field.id));
    let (_, (editing, _)) = pass(&mut typed);
    assert!(!editing.galley.elided, "an edited value is laid out whole");
    assert_eq!(typed, long, "the value itself is never cut");
}

/// A choice whose full name outruns its field shows its short name closed,
/// in value ink, while the menu names every choice in full.
#[test]
fn a_short_named_choice_shows_its_name_closed() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut selected = "a3".to_owned();
    let options = vec![
        ("a3".to_owned(), "ISO A3 · 420 × 297 mm".to_owned()),
        ("a4".to_owned(), "ISO A4 · 297 × 210 mm".to_owned()),
    ];
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(278.0);
            property_row_combo_showing(
                ui,
                "Size",
                "short-name",
                &mut selected,
                &options,
                "ISO A3",
                true,
            );
        });
    });
    let texts = all_texts(&output.shapes);
    let closed = texts
        .iter()
        .find(|text| text.galley.text() == "ISO A3")
        .expect("the short name was painted");
    assert!(!closed.galley.elided, "the short name fits its field");
    assert_eq!(ink(closed), Tokens::default().color.text);
    assert!(
        texts.iter().all(|text| !text.galley.text().contains('·')),
        "the full names wait in the menu"
    );
}

/// A checklist states its summary closed, opens to one box per item, reports
/// the one box ticked, and stays open for the next.
#[test]
fn a_checklist_reports_the_box_ticked_and_stays_open() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut items = vec![
        ("Checked by".to_owned(), false),
        ("Language".to_owned(), true),
    ];
    let run = |events: Vec<egui::Event>, items: &[(String, bool)]| {
        let mut toggled = None;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(320.0, 400.0))),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let on = items.iter().filter(|(_, on)| *on).count();
                    let summary = format!("{on} of {} shown", items.len());
                    toggled = property_row_checklist(
                        ui,
                        "Optional boxes",
                        "boxes",
                        &summary,
                        items,
                        true,
                    );
                });
            },
        );
        (all_texts(&output.shapes), toggled)
    };
    let at = |texts: &[egui::epaint::TextShape], wanted: &str| {
        texts
            .iter()
            .find(|text| text.galley.text() == wanted)
            .map(|text| Rect::from_min_size(text.pos, text.galley.size()).center())
    };
    let press = |pos: Pos2, pressed: bool| {
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            },
        ]
    };

    let (closed, _) = run(Vec::new(), &items);
    let summary = closed
        .iter()
        .find(|text| text.galley.text() == "1 of 2 shown")
        .expect("the closed field states the summary");
    assert_eq!(ink(summary), Tokens::default().color.text);
    assert_eq!(at(&closed, "Checked by"), None, "closed, it lists nothing");

    let field = at(&closed, "1 of 2 shown").unwrap();
    run(press(field, true), &items);
    run(press(field, false), &items);
    let (open, _) = run(Vec::new(), &items);
    let checked_by = at(&open, "Checked by").expect("open, it lists every box");
    assert!(at(&open, "Language").is_some());

    run(press(checked_by, true), &items);
    let (_, toggled) = run(press(checked_by, false), &items);
    assert_eq!(toggled, Some(0), "the box ticked is reported");
    items[0].1 = true;

    let (still, toggled) = run(Vec::new(), &items);
    assert_eq!(toggled, None);
    assert!(
        at(&still, "Language").is_some(),
        "the list stays open for the next box"
    );
    assert!(at(&still, "2 of 2 shown").is_some());
}

/// A default, the value that applies while a field is empty, reads a step
/// above the form a value is written in and a step below a typed value.
#[test]
fn a_default_reads_apart_from_the_form_a_value_is_written_in() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let (mut document_type, mut date, mut title) = (String::new(), String::new(), "top".to_owned());
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(320.0);
            property_row_input_with_default(
                ui,
                "Document type",
                &mut document_type,
                "Circuit diagram",
                false,
            );
            property_row_input_with_hint(ui, "Date of issue", &mut date, "YYYY-MM-DD", false);
            property_row_input(ui, "Title", &mut title, false);
        });
    });
    let painted = |wanted: &str| {
        output
            .shapes
            .iter()
            .find_map(|clipped| painted_text(&clipped.shape, wanted))
            .unwrap_or_else(|| panic!("{wanted:?} was not painted"))
    };
    let palette = Tokens::default().color;
    assert_eq!(ink(&painted("Circuit diagram")), palette.text_dim);
    assert_eq!(ink(&painted("YYYY-MM-DD")), palette.text_faint);
    assert_eq!(ink(&painted("top")), palette.text);
}

/// The stretches of columns of `canvas` carrying the ink of text, left to
/// right.
fn ink_stretches(canvas: &crate::raster::Canvas) -> Vec<std::ops::Range<usize>> {
    let background = canvas.background().to_array();
    let inked = |x: usize| {
        (0..canvas.height()).any(|y| {
            canvas.pixel(x, y).is_some_and(|pixel| {
                let pixel = pixel.to_array();
                (0..3).any(|channel| pixel[channel].abs_diff(background[channel]) > 96)
            })
        })
    };
    let mut stretches: Vec<std::ops::Range<usize>> = Vec::new();
    for x in (0..canvas.width()).filter(|x| inked(*x)) {
        match stretches.last_mut() {
            Some(stretch) if stretch.end == x => stretch.end = x + 1,
            _ => stretches.push(x..x + 1),
        }
    }
    stretches
}

#[test]
fn a_value_typed_in_arabic_is_drawn_in_the_order_it_is_read() {
    // Arabic reads from the right, and the number inside the value reads
    // left to right where it stands: last, so first from the left, drawn
    // exactly where the number alone is drawn.
    let drawn = |value: &str| {
        ink_stretches(&crate::raster::render(egui::vec2(320.0, 32.0), |ui, _| {
            property_row(ui, "", value);
        }))
    };
    let phrase = drawn("مقاومة 47");
    let number = drawn("47");
    assert!(
        !number.is_empty() && phrase.len() > number.len(),
        "{phrase:?}"
    );
    for (in_phrase, alone) in phrase.iter().zip(&number) {
        assert!(
            in_phrase.start.abs_diff(alone.start) <= 1 && in_phrase.end.abs_diff(alone.end) <= 1,
            "the number leads the value: {phrase:?} against {number:?}"
        );
    }
}

/// One pass over a title-block field holding `value`, at `time` seconds,
/// returning the field's editor.
fn title_field_pass(
    ctx: &egui::Context,
    time: f64,
    events: Vec<egui::Event>,
    value: &mut String,
) -> Response {
    let mut field = None;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(480.0, 120.0))),
            time: Some(time),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(400.0);
                field = Some(property_row_input(ui, "Title", value, false));
            });
        },
    );
    field.expect("the row ran")
}

#[test]
#[ignore = "needs the emilk/egui#8576 fix, which no egui release has yet"]
fn a_title_field_holding_arabic_is_edited_in_the_order_it_is_typed() {
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::Theme::default().apply(&ctx);
    let mut value = String::from("عدد 123 مرحبا");
    let mut time = 0.0;
    let mut pass = |events: Vec<egui::Event>, value: &mut String| {
        time += 1.0 / 60.0;
        title_field_pass(&ctx, time, events, value)
    };
    pass(Vec::new(), &mut value);
    let at = pass(Vec::new(), &mut value).rect.center();
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    pass(
        vec![egui::Event::PointerMoved(at), button(true)],
        &mut value,
    );
    assert!(pass(vec![button(false)], &mut value).has_focus());
    let key = |key| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    pass(vec![key(egui::Key::End)], &mut value);
    for _ in 0..3 {
        pass(vec![key(egui::Key::ArrowLeft)], &mut value);
    }
    pass(vec![egui::Event::Text("x".to_owned())], &mut value);
    assert_eq!(value, "عدد 123 مرxحبا");
}

/// An inspector's typed value, its choice and its read-only facts read at
/// one size, the size a field holds its value at: names and numbers in the
/// mono face, a choice and a sentence in the body face. The rows stay one
/// height.
#[test]
fn an_inspectors_values_read_at_the_field_size() {
    fn walk(shape: &Shape, into: &mut Vec<(String, egui::FontId)>) {
        match shape {
            Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
            Shape::Text(text) => into.push((
                text.galley.text().to_owned(),
                text.galley.job.sections[0].format.font_id.clone(),
            )),
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut typed = "10k".to_owned();
    let mut selected = "a4".to_owned();
    let options = vec![("a4".to_owned(), "ISO A4".to_owned())];
    let mut heights = Vec::new();
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(320.0);
            heights.clear();
            // What each row advances the cursor by, its spacing included.
            let mut advance = |ui: &mut Ui, row: &mut dyn FnMut(&mut Ui)| {
                let top = ui.cursor().top();
                row(ui);
                heights.push(ui.cursor().top() - top);
            };
            advance(ui, &mut |ui| {
                let _ = property_row(ui, "Instance", "R1");
            });
            advance(ui, &mut |ui| {
                let _ = property_row_input(ui, "Resistance", &mut typed, false);
            });
            advance(ui, &mut |ui| {
                let _ = property_row_combo(
                    ui,
                    "Size",
                    "field-size-size",
                    &mut selected,
                    &options,
                    true,
                );
            });
            advance(ui, &mut |ui| {
                let _ = property_row_problem(ui, "Net", "out", None);
            });
            let _ = property_row_wrapped(ui, "Rule", "Keep each member's own contract.");
        });
    });
    let mut texts = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut texts);
    }
    let font_of = |value: &str| {
        texts
            .iter()
            .find(|(text, _)| text == value)
            .map(|(_, font)| font.clone())
            .unwrap_or_else(|| panic!("{value:?} is painted"))
    };
    for value in ["R1", "10k", "out"] {
        assert_eq!(font_of(value), theme::field_mono(), "{value:?}");
    }
    for value in ["ISO A4", "Keep each member's own contract."] {
        assert_eq!(font_of(value), theme::field_sans(), "{value:?}");
    }
    assert!(
        heights
            .windows(2)
            .all(|pair| (pair[0] - pair[1]).abs() < 0.5),
        "row heights {heights:?}"
    );
}
