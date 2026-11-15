//! Rendered symbol controls respect host capabilities and return requests without app state.

use super::*;
use rspice_design_model::cell_view::CellViewRef;

fn text_rect(shape: &egui::Shape, label: &str) -> Option<Rect> {
    match shape {
        egui::Shape::Text(text) if text.galley.text() == label => {
            Some(Rect::from_min_size(text.pos, text.galley.size()))
        }
        egui::Shape::Vec(shapes) => shapes.iter().find_map(|shape| text_rect(shape, label)),
        _ => None,
    }
}

fn click_label(label: &str, mut draw: impl FnMut(&mut Ui)) {
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let mut at = None;
    for frame in 0..4 {
        let events = if frame >= 2 {
            let pos = at.expect("the control must be painted before it is clicked");
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: frame == 2,
                    modifiers: egui::Modifiers::NONE,
                },
            ]
        } else {
            Vec::new()
        };
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1000.0, 750.0))),
                events,
                ..Default::default()
            },
            |ctx| draw(ctx),
        );
        if frame < 2 {
            at = output
                .shapes
                .iter()
                .find_map(|shape| text_rect(&shape.shape, label))
                .map(|rect| rect.center());
        }
    }
}

#[test]
fn generation_button_returns_its_source_only_when_enabled() {
    let source = SymbolRequestSource {
        project: rspice_vocabulary::product::ProjectId::new(),
        document: CellViewRef::new("work", "amp", "symbol"),
        occurrence: None,
        design_epoch: 3,
        document_epoch: 4,
        library_revision: 5,
        window: 1,
    };
    let ports = [PortSpec {
        name: "IN".to_owned(),
        direction: rspice_design_model::port::PortDirection::In,
    }];
    for enabled in [false, true] {
        let mut request = None;
        click_label("Generate from schematic", |ui| {
            let canvas = ui.max_rect();
            if let Some(emitted) = generate_overlay(
                ui.ctx(),
                canvas,
                &source,
                &ports,
                enabled,
                "schematic",
                true,
            ) {
                request = Some(emitted);
            }
        });
        assert_eq!(request.is_some(), enabled);
        if let Some(request) = request {
            assert_eq!(request.source, source);
            assert_eq!(request.action, SymbolSurfaceAction::GenerateFromSchematic);
        }
    }
}

/// The way to a first drawing lies on the canvas and takes none of its
/// room, so the canvas stands where it stood whether the card is up or not.
/// Once the symbol has a drawing the card fades and takes no input, so the
/// press that drew the first shape lands on the canvas, not on it. It was a
/// band above the canvas: the first shape took the band away and the canvas
/// jumped up by its height.
#[test]
fn the_generate_overlay_takes_no_room_and_fades_without_input() {
    let source = SymbolRequestSource {
        project: rspice_vocabulary::product::ProjectId::new(),
        document: CellViewRef::new("work", "amp", "symbol"),
        occurrence: None,
        design_epoch: 0,
        document_epoch: 0,
        library_revision: 0,
        window: 1,
    };
    let ports = [PortSpec {
        name: "IN".to_owned(),
        direction: rspice_design_model::port::PortDirection::In,
    }];
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let frame = |shown: bool, time: f64, events: Vec<egui::Event>| {
        let mut canvas = Rect::NOTHING;
        let mut request = None;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1000.0, 750.0))),
                time: Some(time),
                events,
                ..Default::default()
            },
            |ui| {
                let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click());
                canvas = rect;
                request =
                    generate_overlay(ui.ctx(), rect, &source, &ports, true, "schematic", shown);
            },
        );
        let button = output
            .shapes
            .iter()
            .find_map(|shape| text_rect(&shape.shape, "Generate from schematic"));
        (canvas, button, request)
    };
    // An area sizes itself unseen on its first frame.
    frame(true, 0.0, Vec::new());
    let (with_card, button, _) = frame(true, 0.02, Vec::new());
    let button = button.expect("the card is painted while the symbol has no drawing");
    assert!(
        with_card.contains_rect(button),
        "the card lies on the canvas: {button:?} in {with_card:?}"
    );
    let press = |pressed| egui::Event::PointerButton {
        pos: button.center(),
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    // The first shape is drawn: the card starts to fade from this frame.
    let (fading, still_painted, request) = frame(
        false,
        0.05,
        vec![egui::Event::PointerMoved(button.center()), press(true)],
    );
    assert_eq!(fading, with_card, "the canvas stands where it stood");
    assert!(
        still_painted.is_some(),
        "the card fades rather than vanishing"
    );
    assert!(request.is_none());
    let (_, _, request) = frame(false, 0.1, vec![press(false)]);
    assert!(request.is_none(), "a fading card takes no click");
    let (gone, painted, _) = frame(false, 1.0, Vec::new());
    assert_eq!(gone, with_card);
    assert!(painted.is_none(), "the card has faded: {painted:?}");
}

/// A symbol that matches, or that defines the interface itself, has nothing
/// said about it; a schematic that declares no pins is said so in those
/// words; a mismatch is stated by kind with exact counts.
#[test]
fn the_canvas_note_states_each_interface_case_in_its_own_words() {
    assert!(canvas_check_lines(PinSummary::default(), "schematic").is_empty());
    assert_eq!(
        canvas_check_lines(
            PinSummary {
                interface_empty: true,
                ..PinSummary::default()
            },
            "schematic"
        ),
        ["The schematic declares no pins"]
    );
    assert_eq!(
        canvas_check_lines(
            PinSummary {
                interface_empty: false,
                missing: 2,
                unplaced: 0,
                orphaned: 1,
            },
            "schematic"
        ),
        [
            "2 schematic pins have no symbol pin",
            "1 pin is not in the schematic"
        ]
    );
    assert_eq!(
        canvas_check_lines(
            PinSummary {
                interface_empty: false,
                missing: 1,
                unplaced: 3,
                orphaned: 2,
            },
            "module"
        ),
        [
            "1 module pin has no symbol pin",
            "2 pins are not in the module",
            "3 pins are not placed"
        ]
    );
}
