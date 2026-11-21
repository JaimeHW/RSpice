//! The strip's geometry, latches and request routing, through real frames.
use super::*;
use egui::accesskit;

fn source() -> EditorRequestSource {
    EditorRequestSource {
        project: rspice_vocabulary::product::ProjectId::new(),
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
    canvas: StripCanvas::Schematic,
    rotate: true,
    mirror: true,
    cancel: false,
    undo: true,
    redo: true,
    fit: true,
};

/// What one frame of the strip exposed: each button's label, bounds and
/// toggle state.
struct Strip {
    buttons: Vec<(String, accesskit::Rect, Option<accesskit::Toggled>)>,
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
    Strip { buttons }
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
