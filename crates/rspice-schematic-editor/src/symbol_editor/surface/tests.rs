//! Rendered symbol controls respect host capabilities and return requests without app state.

use super::*;

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
        project: rspice_app_types::product::ProjectId::new(),
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
        click_label("Generate from schematic", |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                if let Some(emitted) = empty_generate_state(ui, &source, &ports, enabled) {
                    request = Some(emitted);
                }
            });
        });
        assert_eq!(request.is_some(), enabled);
        if let Some(request) = request {
            assert_eq!(request.source, source);
            assert_eq!(request.action, SymbolSurfaceAction::GenerateFromSchematic);
        }
    }
}

#[test]
fn save_button_requires_passing_checks_a_note_and_publish_capability() {
    for (passed, note, can_publish, accepted) in [
        (true, "Reviewed", true, true),
        (false, "Reviewed", true, false),
        (true, " ", true, false),
        (true, "Reviewed", false, false),
    ] {
        let checks = [SymbolSaveCheck {
            label: "Pin count",
            expected: "1".to_owned(),
            observed: "1".to_owned(),
            passed,
        }];
        let mut note = note.to_owned();
        let mut selected = false;
        click_label("Save symbol revision", |ctx| {
            selected |=
                show_save_symbol_dialog(ctx.ctx(), &checks, 3, &mut note, None, can_publish)
                    == DialogChoice::Primary;
        });
        assert_eq!(selected, accepted);
    }
}
