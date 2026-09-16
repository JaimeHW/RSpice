//! Tests for the workbench visual primitives.

use super::*;

#[test]
fn workspace_title_row_uses_visible_pane_not_offscreen_content_extent() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
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
    crate::ui::Theme::default().apply(&ctx);
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
    crate::ui::Theme::default().apply(&ctx);
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
            for mark in [
                StatusMark::Success,
                StatusMark::Warning,
                StatusMark::Failure,
                StatusMark::Neutral,
            ] {
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
    assert_eq!(
        status_shapes
            .iter()
            .filter(|shape| matches!(&shape.shape, Shape::Path(_)))
            .count(),
        2
    );
    assert_eq!(
        status_shapes
            .iter()
            .filter(|shape| matches!(&shape.shape, Shape::LineSegment { .. }))
            .count(),
        2
    );
    assert_eq!(
        status_shapes
            .iter()
            .filter(|shape| matches!(&shape.shape, Shape::Circle(_)))
            .count(),
        1
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
    let (title, meta) = section_header_column_widths(210.0, 47.0, 91.0, true);
    assert_eq!(meta, 91.0);
    assert!((title + SECTION_HEADER_COLUMN_GAP + meta - 210.0).abs() <= 0.001);
    assert!(title >= 47.0);
    assert!(!section_header_wraps(210.0, 47.0, 91.0, true));
}

#[test]
fn overconstrained_section_header_wraps_instead_of_eliding_both_columns() {
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
        section_header_full_text("LONG ENGINEERING TITLE", Some("complete metadata")),
        "LONG ENGINEERING TITLE, complete metadata"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn property_controls_and_section_headers_expose_their_full_accessible_names() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
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
                    WorkbenchIcon::Sliders,
                    "Tune value",
                    true,
                    None,
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
            && node.label() == Some("LONG ENGINEERING SECTION TITLE, complete metadata")
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
    crate::ui::Theme::default().apply(&ctx);
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
    crate::ui::Theme::default().apply(&ctx);
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
    crate::ui::Theme::default().apply(&ctx);
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

    assert_eq!(short_height, PROPERTY_ROW_MIN_H);
    assert_eq!(long_value_height, short_height);
    assert_eq!(long_label_height, short_height);
}

#[test]
fn narrow_property_rows_keep_the_proportional_label_split() {
    // The inspector dock clamps its width to 278..=440, and the widest
    // in-surface inspector is 330. None of them reach the cap, so every
    // narrow property list lays out exactly as it did before the cap.
    for width in [278.0_f32, 281.6, 312.0, 330.0, 440.0] {
        let (label_column, gap, value_column) = property_row_columns(width);
        let inner = width - 2.0 * PROPERTY_ROW_PAD;
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
    let inner = 780.0 - 2.0 * PROPERTY_ROW_PAD;
    let (label_column, gap, value_column) = property_row_columns(780.0);
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
    crate::ui::Theme::default().apply(&ctx);
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

#[test]
fn schematic_section_heading_uses_the_mockup_tracking() {
    assert_eq!(tokens::FS_2, 13.0);
    assert!((SCHEMATIC_SECTION_TITLE_TRACKING - 0.715).abs() <= 0.001);
}

#[test]
fn card_content_remains_vertical_inside_a_horizontal_parent() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
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
    crate::ui::Theme::default().apply(&ctx);
    let path = r"C:\Users\Example\AppData\Local\RSpice\.rspice-recovery";
    let font = crate::ui::theme::mono(tokens::FS_0, FontWeight::Regular);
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
