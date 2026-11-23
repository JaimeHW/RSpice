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
                    &[],
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

/// Words that fit their room are left whole, also when they end in an
/// ellipsis of their own, which is no cut to move back to a word.
#[test]
fn words_that_fit_keep_their_own_ellipsis() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let text = ".subckt diffpair inp inn outp outn tail params\u{2026}";
    let font = crate::theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut shown = Vec::new();
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let full = ui
                .painter()
                .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
                .size()
                .x;
            for width in [full + 1.0, full * 0.8] {
                shown.push(elide_at_words(ui, text, &font, width));
            }
        });
    });

    assert_eq!(shown[0], text);
    let kept = shown[1].strip_suffix('\u{2026}').expect("a cut");
    assert!(text[kept.len()..].starts_with(' '), "{}", shown[1]);
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
