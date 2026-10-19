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
fn overconstrained_section_header_wraps_instead_of_eliding_both_columns() {
    use section::{
        SECTION_HEADER_COLUMN_GAP, section_header_column_widths, section_header_full_text,
        section_header_wraps,
    };
    let (title, meta) = section_header_column_widths(100.0, 80.0, 80.0, true);
    assert!((title - 50.6).abs() <= 0.001);
    assert!((meta - 41.4).abs() <= 0.001);
    assert!((title + SECTION_HEADER_COLUMN_GAP + meta - 100.0).abs() <= 0.001);
    assert!(section_header_wraps(100.0, 80.0, 80.0, true));

    assert_eq!(
        section_header_column_widths(100.0, 180.0, 0.0, false),
        (100.0, 0.0)
    );
    assert!(section_header_wraps(100.0, 180.0, 0.0, false));
    assert_eq!(
        section_header_full_text("Long engineering title", Some("complete metadata")),
        "Long engineering title, complete metadata"
    );
}

#[cfg(not(target_arch = "wasm32"))]
/// A choice row's value and a typed row's value start on one edge, so a
/// column of mixed controls reads as one column.
#[test]
fn choice_and_typed_rows_start_their_values_on_one_edge() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut typed = "Bias network".to_owned();
    let mut selected = "a4".to_owned();
    let options = vec![("a4".to_owned(), "ISO A4".to_owned())];
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(320.0);
            property_row_combo(ui, "Size", "one-edge-size", &mut selected, &options, true);
            property_row_input(ui, "Sheet title", &mut typed, false);
        });
    });
    let find = |wanted: &str| {
        output
            .shapes
            .iter()
            .find_map(|clipped| painted_text_rect(&clipped.shape, wanted))
            .unwrap_or_else(|| panic!("{wanted:?} was not painted"))
    };
    let choice = find("ISO A4");
    let typed = find("Bias network");
    assert!(
        (choice.left() - typed.left()).abs() < 0.5,
        "choice text {choice:?}, typed text {typed:?}"
    );
}

#[cfg(not(target_arch = "wasm32"))]
/// A choice longer than its field is cut to the field. Laid out whole, it
/// widened its row, and every row after it ran past the panel's edge.
#[test]
fn a_choice_longer_than_its_field_is_cut_to_it() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let long = "Letters, digits and every punctuation mark the netlist keeps".to_owned();
    let mut selected = "long".to_owned();
    let options = vec![("long".to_owned(), long.clone())];
    let mut typed = "R1".to_owned();
    let mut rows = Rect::NOTHING;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(240.0, 200.0))),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    property_row_combo(
                        ui,
                        "Net names",
                        "cut-choice",
                        &mut selected,
                        &options,
                        true,
                    );
                    property_row_input(ui, "Instance", &mut typed, false);
                    rows = ui.min_rect();
                });
        },
    );
    assert!(rows.right() <= 240.0, "the rows ran to {}", rows.right());
    let choice = output
        .shapes
        .iter()
        .find_map(|clipped| painted_text_rect(&clipped.shape, &long))
        .expect("the choice was painted");
    assert!(choice.right() < rows.right(), "{choice:?} in {rows:?}");
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
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
        }));
    }
}

#[test]
fn empty_state_recovery_action_group_is_centered_under_the_copy() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut content_center = 0.0;
    let mut first = Rect::NOTHING;
    let mut second = Rect::NOTHING;

    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(900.0, 700.0))),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                content_center = ui.available_rect_before_wrap().center().x;
                empty_state_with_actions(
                    ui,
                    WorkbenchIcon::Code,
                    "No project source workspace",
                    "Create or import one to continue.",
                    |ui| {
                        first = ui.button("Create source workspace…").rect;
                        second = ui.button("Import root source…").rect;
                    },
                );
            });
        },
    );

    let group_center = first.union(second).center().x;
    assert!(
        (group_center - content_center).abs() <= 1.0,
        "recovery action group centered at {group_center}, expected {content_center}"
    );
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
fn narrow_property_rows_keep_the_proportional_label_split() {
    // The inspector dock clamps its width to 278..=440, and the widest
    // in-surface inspector is 330. None of them reach the cap, so every
    // narrow property list lays out exactly as it did before the cap.
    for width in [278.0_f32, 281.6, 312.0, 330.0, 440.0] {
        let inner = width - 2.0 * PANEL_INSET;
        let (label_column, gap, value_column) = property_row_columns(inner);
        let columns = inner - PROPERTY_ROW_GAP;
        assert!(
            (label_column - columns * PROPERTY_LABEL_FRACTION).abs() <= 0.001,
            "the cap bound at {width} px and narrowed a dock-width label column"
        );
        assert!((label_column + gap + value_column - inner).abs() <= 0.001);
    }
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

#[test]
fn the_label_column_cap_clears_the_widest_label_the_workbench_ships() {
    // Widest property-row label in the workbench, from the simulation plan
    // manager's aside. The cap has to clear it, or capping the column would
    // trade a layout gap for newly elided labels on wide rows.
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut widest = f32::INFINITY;
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            widest = ui
                .painter()
                .layout_no_wrap(
                    "Variables, outputs, specifications".to_owned(),
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    Color32::WHITE,
                )
                .size()
                .x;
        });
    });

    assert!(
        widest <= PROPERTY_LABEL_MAX_W,
        "the widest shipped label needs {widest} px but the cap allows \
         {PROPERTY_LABEL_MAX_W}"
    );
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

    for (ink, expected) in [
        (Ink::Text, palette.text),
        (Ink::Dim, palette.text_dim),
        (Ink::Faint, palette.text_faint),
    ] {
        let inked = render(&|ui| {
            property_row_ink(ui, "Terminal", "unplaced", ink);
        });
        assert_eq!(value(&inked).fallback_color, expected, "{ink:?}");
        assert_eq!(value(&inked).pos, value(&plain).pos, "{ink:?}");
    }

    // The schematic inspector's variant sets its value a size up, and is its
    // own plain row when there is nothing to mark.
    let schematic = render(&|ui| {
        schematic_property_row(ui, "Terminal", "unplaced");
    });
    let schematic_clear = render(&|ui| {
        schematic_property_row_problem(ui, "Terminal", "unplaced", None);
    });
    assert_eq!(value(&schematic_clear).pos, value(&schematic).pos);
    assert_eq!(
        value(&schematic_clear).galley.size(),
        value(&schematic).galley.size()
    );
    let schematic_marked = render(&|ui| {
        schematic_property_row_problem(ui, "Terminal", "unplaced", Some(Problem::Warning));
    });
    assert_eq!(value(&schematic_marked).fallback_color, palette.text);
    assert!(
        stroke_colors(&schematic_marked)
            .iter()
            .all(|color| *color == palette.warn)
    );
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

/// A problem line starts on the inset a note starts on, with its words after
/// the mark.
#[test]
fn a_problem_line_sits_on_the_note_inset() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut panel_left = 0.0;
    let shapes = ctx
        .run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(280.0);
                panel_left = ui.max_rect().left();
                inspector::muted_inspector_copy(ui, "A note");
                problem_line(ui, Problem::Warning, "A problem");
            });
        })
        .shapes;
    let note = shapes
        .iter()
        .find_map(|shape| painted_text(&shape.shape, "A note"))
        .expect("note painted");
    let problem = shapes
        .iter()
        .find_map(|shape| painted_text(&shape.shape, "A problem"))
        .expect("problem painted");
    assert!(
        (problem.pos.x - note.pos.x - crate::widgets::PROBLEM_MARK_ADVANCE).abs() < 0.5,
        "note {:?}, problem {:?}",
        note.pos,
        problem.pos
    );
    assert!((note.pos.x - panel_left - PANEL_INSET).abs() < 0.5);
}

/// A header too narrow for its title and metadata side by side sets the
/// metadata under the title, from the same left edge, inside its band.
#[test]
fn a_wrapped_section_header_sets_its_metadata_under_the_title() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let meta = "inherited \u{b7} project default";
    let mut header = Rect::NOTHING;
    let mut shapes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.allocate_ui(Vec2::new(200.0, 80.0), |ui| {
                    // The band ends where the cursor stops, with no spacing.
                    ui.spacing_mut().item_spacing.y = 0.0;
                    let top = ui.cursor().top();
                    section_header(ui, "Drawing sheet", Some(meta));
                    header = Rect::from_min_max(
                        Pos2::new(ui.min_rect().left(), top),
                        Pos2::new(ui.min_rect().right(), ui.cursor().top()),
                    );
                });
            });
        });
        shapes = output.shapes;
    }
    let find = |wanted: &str| {
        shapes
            .iter()
            .find_map(|shape| painted_text_rect(&shape.shape, wanted))
            .unwrap_or_else(|| panic!("{wanted:?} was not painted"))
    };
    let title = find("Drawing sheet");
    let metadata = find(meta);

    assert!(
        (metadata.left() - title.left()).abs() < 0.5,
        "{title:?} {metadata:?}"
    );
    assert!(metadata.top() >= title.bottom(), "{title:?} {metadata:?}");
    assert!(title.top() >= header.top() && metadata.bottom() <= header.bottom());
    let above = title.top() - header.top();
    let below = header.bottom() - metadata.bottom();
    assert!(
        (above - below).abs() <= 2.0,
        "unbalanced: {above} above, {below} below"
    );
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

/// Every section header helper paints the one header: the same height, the
/// same band reaching the panel's edges, and its title in the same face, size
/// and colour, starting on the panel inset — inside a panel body and out of
/// one.
#[test]
fn every_section_header_helper_paints_the_same_header() {
    const TITLE: &str = "Simulation parameters";
    type Header = fn(&mut Ui);
    let headers: [(&str, Header); 4] = [
        ("section_header", |ui| {
            section_header(ui, TITLE, Some("editable"))
        }),
        ("section_header_explained", |ui| {
            section_header_explained(ui, TITLE, "editable", "Every field can be typed into.")
        }),
        ("section_header_action", |ui| {
            let _ = section_header_action(ui, TITLE, "Properties\u{2026}", true);
        }),
        ("code_inspector_section", |ui| {
            code_inspector_section(ui, TITLE, Some("editable"), None, |_| {});
        }),
    ];
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let palette = Tokens::default().color;
    let band = Color32::from_rgba_unmultiplied(
        palette.bg_panel_2.r(),
        palette.bg_panel_2.g(),
        palette.bg_panel_2.b(),
        204,
    );
    for in_body in [false, true] {
        let mut seen: Option<(f32, egui::FontId, Color32, f32, Rect)> = None;
        for (name, header) in headers {
            let mut panel = Rect::NOTHING;
            let output = ctx.run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        ui.allocate_ui(Vec2::new(312.0, 200.0), |ui| {
                            panel = ui.max_rect();
                            if in_body {
                                panel_body(ui, header);
                            } else {
                                header(ui);
                            }
                        });
                    });
            });
            let title = all_texts(&output.shapes)
                .into_iter()
                .find(|text| text.galley.text() == TITLE)
                .unwrap_or_else(|| panic!("{name} did not paint its title"));
            let section = &title.galley.job.sections[0].format;
            let bands: Vec<Rect> = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    Shape::Rect(rect) if rect.fill == band => Some(rect.rect),
                    _ => None,
                })
                .collect();
            assert_eq!(bands.len(), 1, "{name} (body: {in_body}) painted {bands:?}");
            let band = bands[0];
            assert!(
                (band.left() - panel.left()).abs() < 0.5 && (band.right() - 312.0).abs() < 0.5,
                "{name} (body: {in_body}) band {band:?} does not reach the panel's edges"
            );
            assert!(
                (title.pos.x - panel.left() - PANEL_INSET).abs() < 0.5,
                "{name} (body: {in_body}) title at {}",
                title.pos.x
            );
            let painted = (
                band.height(),
                section.font_id.clone(),
                section.color,
                title.pos.y - band.top(),
                band,
            );
            assert_eq!(painted.0, PANEL_SECTION_H, "{name} (body: {in_body})");
            match &seen {
                None => seen = Some(painted),
                Some(first) => {
                    assert_eq!(first.0, painted.0, "{name} (body: {in_body}) height");
                    assert_eq!(first.1, painted.1, "{name} (body: {in_body}) title face");
                    assert_eq!(first.2, painted.2, "{name} (body: {in_body}) title colour");
                    assert!(
                        (first.3 - painted.3).abs() < 0.5,
                        "{name} (body: {in_body}) title baseline"
                    );
                }
            }
        }
    }
}

/// Whatever is placed in a panel body starts on the panel inset and stops on
/// it — a bare label, a button row, a field, a note, a problem, a property
/// row — because the body put it there, and a field with no width of its own
/// spans the body. The rows stack with no gap; a header leaves its one gap
/// under itself.
#[test]
fn a_widget_placed_in_a_panel_body_starts_on_the_inset() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut field = Rect::NOTHING;
    let mut button = Rect::NOTHING;
    let mut rows = Vec::new();
    let mut text = String::from("expression");
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.allocate_ui(Vec2::new(312.0, 400.0), |ui| {
                    panel_body(ui, |ui| {
                        section_header(ui, "Debug", Some("inactive"));
                        rows.push(property_row(ui, "Current line", "\u{2014}").rect);
                        rows.push(property_row(ui, "Exception policy", "uncaught").rect);
                        ui.label("Breakpoints");
                        button = crate::widgets::Button::new("Start").show(ui).rect;
                        field = ui.text_edit_singleline(&mut text).rect;
                        inspector::muted_inspector_copy(ui, "A note");
                        problem_line(ui, Problem::Warning, "A problem");
                    });
                });
            });
    });
    let texts = all_texts(&output.shapes);
    let left_of = |wanted: &str| {
        texts
            .iter()
            .find(|text| text.galley.text() == wanted)
            .map(|text| text.pos.x)
            .unwrap_or_else(|| panic!("{wanted:?} was not painted"))
    };
    for wanted in [
        "Debug",
        "Current line",
        "Exception policy",
        "Breakpoints",
        "A note",
    ] {
        assert!(
            (left_of(wanted) - PANEL_INSET).abs() < 0.5,
            "{wanted:?} starts at {}",
            left_of(wanted)
        );
    }
    assert!(
        (left_of("A problem") - PANEL_INSET - crate::widgets::PROBLEM_MARK_ADVANCE).abs() < 0.5
    );
    assert!(
        (button.left() - PANEL_INSET).abs() < 0.5,
        "button {button:?}"
    );
    assert!((field.left() - PANEL_INSET).abs() < 0.5, "field {field:?}");
    assert!(
        (field.right() - (312.0 - PANEL_INSET)).abs() < 0.5,
        "a field with no width of its own spans the body: {field:?}"
    );
    let band = output
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            Shape::Rect(rect) if rect.rect.height() == PANEL_SECTION_H => Some(rect.rect),
            _ => None,
        })
        .expect("the header painted its band");
    assert_eq!(band.left(), 0.0, "the band reaches the panel's edge");
    assert_eq!(rows[0].top() - band.bottom(), PANEL_HEADER_GAP);
    assert_eq!(rows[0].height(), PANEL_ROW_H);
    assert_eq!(rows[1].top(), rows[0].bottom(), "rows stack with no gap");
}

/// A row painted outside any panel body keeps the inset itself, so a property
/// list in a dialog lines up exactly as one in a panel does.
#[test]
fn a_row_outside_a_panel_body_keeps_the_inset_itself() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let mut lefts = Vec::new();
    for in_body in [false, true] {
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.allocate_ui(Vec2::new(312.0, 100.0), |ui| {
                        if in_body {
                            panel_body(ui, |ui| {
                                property_row(ui, "Instance", "D1");
                            });
                        } else {
                            property_row(ui, "Instance", "D1");
                        }
                    });
                });
        });
        let label = all_texts(&output.shapes)
            .into_iter()
            .find(|text| text.galley.text() == "Instance")
            .expect("label painted");
        lefts.push(label.pos.x);
    }
    assert!((lefts[0] - PANEL_INSET).abs() < 0.5, "{lefts:?}");
    assert!((lefts[0] - lefts[1]).abs() < 0.5, "{lefts:?}");
}
