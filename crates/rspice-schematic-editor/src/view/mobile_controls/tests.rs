//! The strip's geometry, latches and request routing, through real frames.
use super::*;
use egui::accesskit;

fn source() -> EditorRequestSource {
    EditorRequestSource {
        project: rspice_app_types::product::ProjectId::new(),
        document: rspice_design_model::cell_view::CellViewRef::new("work", "amp", "schematic"),
        occurrence: None,
        design_epoch: 1,
        document_epoch: 2,
        content_version: 3,
        topology_version: 4,
        symbol_revision: 5,
        sheet: None,
    }
}

const EVERYTHING: MobileCanvasCapabilities = MobileCanvasCapabilities {
    rotate: true,
    mirror: true,
    cancel: false,
    undo: true,
    redo: true,
    fit: true,
};

/// What one frame of the strip exposed: its bounds and each button's
/// label, bounds and toggle state.
struct Strip {
    bounds: accesskit::Rect,
    buttons: Vec<(String, accesskit::Rect, Option<accesskit::Toggled>)>,
    labels: Vec<String>,
}

fn frame(
    ctx: &Context,
    screen: Vec2,
    capabilities: MobileCanvasCapabilities,
    events: Vec<egui::Event>,
    requested: &mut Option<MobileCanvasRequest>,
) -> Strip {
    let source = source();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
            events,
            ..Default::default()
        },
        |ui| {
            if let Some(request) = show(&ui.ctx().clone(), ui.max_rect(), &source, capabilities) {
                *requested = Some(request);
            }
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("strip accessibility tree")
        .nodes;
    let bounds = nodes
        .iter()
        .find(|(_, node)| node.role() == accesskit::Role::Toolbar)
        .and_then(|(_, node)| node.bounds())
        .expect("the strip is a toolbar");
    let buttons = nodes
        .iter()
        .filter(|(_, node)| node.role() == accesskit::Role::Button)
        .map(|(_, node)| {
            (
                node.label().unwrap_or_default().to_owned(),
                node.bounds().expect("button bounds"),
                node.toggled(),
            )
        })
        .collect();
    let labels = nodes
        .iter()
        .filter_map(|(_, node)| node.label().or(node.value()).map(str::to_owned))
        .collect();
    Strip {
        bounds,
        buttons,
        labels,
    }
}

fn context() -> Context {
    let ctx = Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx
}

fn centre(bounds: accesskit::Rect) -> egui::Pos2 {
    egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    )
}

/// Centred top to bottom, to the pixel the odd half point rounds to.
fn assert_centred(bounds: accesskit::Rect, height: f64) {
    let (above, below) = (bounds.y0, height - bounds.y1);
    assert!((above - below).abs() <= 1.0, "{above} above, {below} below");
}

fn press(pos: egui::Pos2, pressed: bool) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(pos),
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

/// D1 section 10: on a canvas 420 pt tall or more, eight 44 pt buttons in
/// four groups, 4 pt apart inside a group and 9 pt (a 1 pt rule with 4 pt
/// either side) between groups; 405 x 54 pt, 8 pt in from the right edge
/// and centred top to bottom. G-12: the old strip was three zoom buttons
/// under a block of guidance text, and only at 620 px or less.
#[test]
fn the_tablet_strip_is_eight_thumb_buttons_in_four_groups() {
    let ctx = context();
    let strip = frame(
        &ctx,
        Vec2::new(1024.0, 768.0),
        EVERYTHING,
        vec![],
        &mut None,
    );
    assert_eq!(strip.bounds.x1 - strip.bounds.x0, 54.0);
    assert_eq!(strip.bounds.y1 - strip.bounds.y0, 405.0);
    assert_eq!(1024.0 - strip.bounds.x1, 8.0);
    assert_centred(strip.bounds, 768.0);

    let mut buttons = strip.buttons.clone();
    buttons.sort_by(|a, b| a.1.y0.total_cmp(&b.1.y0));
    let order: Vec<&str> = buttons.iter().map(|(label, ..)| label.as_str()).collect();
    assert_eq!(
        order,
        [
            "Select with a box",
            "Add to selection",
            "Rotate clockwise",
            "Mirror",
            "Cancel",
            "Undo",
            "Redo",
            "Fit drawing sheet",
        ]
    );
    for (label, bounds, _) in &buttons {
        assert_eq!(bounds.x1 - bounds.x0, 44.0, "{label}");
        assert_eq!(bounds.y1 - bounds.y0, 44.0, "{label}");
    }
    let gaps: Vec<f64> = buttons
        .windows(2)
        .map(|pair| pair[1].1.y0 - pair[0].1.y1)
        .collect();
    assert_eq!(gaps, [4.0, 9.0, 4.0, 9.0, 4.0, 4.0, 9.0]);
    assert_eq!(buttons[0].1.y0 - strip.bounds.y0, 5.0, "padding and border");
}

/// A phone in landscape: five buttons, 246 pt, one group. Mirror and Redo
/// go to the long-press menu, and the box latch gives way to Cancel while
/// an operation is live.
#[test]
fn the_phone_strip_is_five_buttons_and_cancel_takes_the_first_while_live() {
    for live in [false, true] {
        let ctx = context();
        let capabilities = MobileCanvasCapabilities {
            cancel: live,
            ..EVERYTHING
        };
        let strip = frame(
            &ctx,
            Vec2::new(844.0, 390.0),
            capabilities,
            vec![],
            &mut None,
        );
        assert_eq!(strip.bounds.y1 - strip.bounds.y0, 246.0);
        assert_centred(strip.bounds, 390.0);
        let mut buttons = strip.buttons.clone();
        buttons.sort_by(|a, b| a.1.y0.total_cmp(&b.1.y0));
        let order: Vec<&str> = buttons.iter().map(|(label, ..)| label.as_str()).collect();
        let first = if live { "Cancel" } else { "Select with a box" };
        assert_eq!(
            order,
            [
                first,
                "Add to selection",
                "Rotate clockwise",
                "Undo",
                "Fit drawing sheet"
            ]
        );
        let gaps: Vec<f64> = buttons
            .windows(2)
            .map(|pair| pair[1].1.y0 - pair[0].1.y1)
            .collect();
        assert_eq!(gaps, [4.0; 4]);
    }
}

/// D1 removes the guidance block (it said "Drag to pan" when a drag moved
/// the part under the finger, audit E-11) and the zoom buttons, which
/// pinch, double tap and the View menu cover.
#[test]
fn the_strip_carries_no_guidance_text_and_no_zoom_steps() {
    let ctx = context();
    let strip = frame(&ctx, Vec2::new(390.0, 844.0), EVERYTHING, vec![], &mut None);
    for label in &strip.labels {
        assert!(!label.contains("Drag to pan"), "{label}");
        assert!(!label.contains("Pinch"), "{label}");
        assert!(!label.starts_with("Zoom"), "{label}");
    }
}

/// A latch toggles on a tap, reads as a pressed toggle to assistive
/// technology, and is reported so the host can say what a drag now does.
#[test]
fn a_latch_toggles_on_a_tap_and_holds_until_tapped_again() {
    let ctx = context();
    let screen = Vec2::new(1024.0, 768.0);
    // The strip's first frame is egui's sizing pass, which takes no input.
    frame(&ctx, screen, EVERYTHING, vec![], &mut None);
    let strip = frame(&ctx, screen, EVERYTHING, vec![], &mut None);
    let target = |strip: &Strip, label: &str| {
        strip
            .buttons
            .iter()
            .find(|(name, ..)| name == label)
            .map(|(_, bounds, toggled)| (centre(*bounds), *toggled))
            .unwrap_or_else(|| panic!("no {label}"))
    };
    let (select_box, toggled) = target(&strip, "Select with a box");
    assert_eq!(toggled, Some(accesskit::Toggled::False));
    assert_eq!(target(&strip, "Undo").1, None, "a command is not a toggle");

    for (round, expected) in [(0, true), (1, false)] {
        let mut requested = None;
        frame(
            &ctx,
            screen,
            EVERYTHING,
            press(select_box, true),
            &mut requested,
        );
        frame(
            &ctx,
            screen,
            EVERYTHING,
            press(select_box, false),
            &mut requested,
        );
        // The button names its state as it paints, before the tap lands; the
        // next frame reads the new state.
        let strip = frame(&ctx, screen, EVERYTHING, vec![], &mut None);
        let request = requested.expect("the tap is reported");
        assert_eq!(
            request.action,
            MobileCanvasAction::SelectBox,
            "round {round}"
        );
        assert_eq!(touch_latches(&ctx).select_box, expected, "round {round}");
        assert!(!touch_latches(&ctx).select_add);
        let toggled = if expected {
            accesskit::Toggled::True
        } else {
            accesskit::Toggled::False
        };
        assert_eq!(target(&strip, "Select with a box").1, Some(toggled));
    }
}

#[test]
fn requests_keep_the_pressed_document_and_honor_disabled_controls() {
    for change in ["none", "document", "disabled"] {
        let ctx = context();
        let original = source();
        let mut current = original.clone();
        let mut target = None;
        let mut requested = None;
        for step in 0..4 {
            if step == 3 && change == "document" {
                current.document_epoch += 1;
            }
            let events = if step >= 2 {
                press(target.expect("undo button"), step == 2)
            } else {
                Vec::new()
            };
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        Vec2::new(1024.0, 768.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| {
                    if let Some(request) = show(
                        &ui.ctx().clone(),
                        ui.max_rect(),
                        &current,
                        MobileCanvasCapabilities {
                            undo: change != "disabled",
                            ..Default::default()
                        },
                    ) {
                        requested = Some(request);
                    }
                },
            );
            if step < 2 {
                target = output
                    .platform_output
                    .accesskit_update
                    .unwrap()
                    .nodes
                    .iter()
                    .find(|(_, node)| node.label() == Some("Undo"))
                    .and_then(|(_, node)| node.bounds())
                    .map(centre);
            }
        }
        if change == "disabled" {
            assert!(requested.is_none());
        } else {
            let request = requested.expect("enabled undo request");
            assert_eq!(request.action, MobileCanvasAction::Undo);
            assert_eq!(request.source, original);
        }
    }
}

/// The strip on a phone held upright (the tablet strip, since the canvas is
/// tall), a phone in landscape and a tablet, in both themes, over the
/// canvas colour, with the box latch on and Redo unavailable.
#[test]
#[ignore = "writes review PNGs to RSPICE_RASTER_DIR"]
fn render_touch_strip_review() {
    for (name, size) in [
        ("phone-390x844", Vec2::new(390.0, 844.0)),
        ("phone-landscape-844x390", Vec2::new(844.0, 390.0)),
        ("tablet-1024x768", Vec2::new(1024.0, 768.0)),
    ] {
        for (mode, theme) in crate::view::lod::review::themes() {
            let source = source();
            let canvas = rspice_ui_kit::raster::render_themed(theme, size, |ui, _| {
                let palette = rspice_ui_kit::tokens::active_palette();
                ui.painter()
                    .rect_filled(ui.max_rect(), 0.0, palette.canvas_bg);
                set_touch_latches(
                    ui.ctx(),
                    TouchLatches {
                        select_box: true,
                        select_add: false,
                    },
                );
                show(
                    &ui.ctx().clone(),
                    ui.max_rect(),
                    &source,
                    MobileCanvasCapabilities {
                        redo: false,
                        ..EVERYTHING
                    },
                );
            });
            crate::view::lod::review::write_png(
                &format!("touch-strip-{name}-{mode}"),
                &canvas,
                size.y as usize,
            );
        }
    }
}
