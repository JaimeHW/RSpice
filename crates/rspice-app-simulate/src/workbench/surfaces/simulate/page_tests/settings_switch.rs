//! The one settings control on these pages that paints itself.
//!
//! Split from the route tests because it is the only case here that presses
//! the pointer, and because what it judges is a widget rather than a page:
//! the row is `page_kit`'s, and the four settings that use it are only its
//! callers.

use egui::{Rect, vec2};

use super::super::page_kit;

/// The studio's own switch, connected, and announced as what it is.
///
/// Four settings rows on these pages were still `ui.checkbox`. That is egui's
/// tick box, not a control this design system ships: the same kind of decision
/// read as a tick box on four rows and as a switch on every other one, and
/// nothing told the reader that the two meant the same thing.
///
/// The replacement paints itself, so none of what egui gives a `Checkbox` for
/// free comes with it — the toggle, the disabled state and the announcement
/// all have to be written, and all three are asserted here. A row that painted
/// a switch and did not move the value would be the exact defect the design
/// bar exists to stop: a control that ships looking connected.
#[test]
fn a_settings_switch_toggles_on_click_and_announces_its_state() {
    fn frame(value: &mut bool, enabled: bool, click: bool) -> (bool, Vec<(String, bool)>) {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        // The row has to be laid out before a press can land on it, so the
        // first pass positions it and the second is the one that clicks.
        let mut changed = false;
        let mut announced = Vec::new();
        for pass in 0..2 {
            let at = egui::pos2(12.0, 8.0);
            let events = if click && pass == 1 {
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
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(420.0, 120.0))),
                    events,
                    ..egui::RawInput::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE)
                        .show(ctx, |ui| {
                            ui.add_enabled_ui(enabled, |ui| {
                                changed = page_kit::switch_row(
                                    ui,
                                    "Permit live delivery for outputs that request it",
                                    value,
                                );
                            });
                        });
                },
            );
            if pass == 1 {
                announced = output
                    .platform_output
                    .accesskit_update
                    .expect("AccessKit update")
                    .nodes
                    .iter()
                    .filter(|(_, node)| node.role() == egui::accesskit::Role::CheckBox)
                    .map(|(_, node)| {
                        (
                            node.label().unwrap_or_default().to_owned(),
                            node.toggled() == Some(egui::accesskit::Toggled::True),
                        )
                    })
                    .collect();
            }
        }
        (changed, announced)
    }

    let mut value = false;
    let (changed, announced) = frame(&mut value, true, true);
    assert!(changed, "a click on the row has to move the setting");
    assert!(value, "and the value it moved is the caller's");
    assert_eq!(
        announced,
        vec![(
            "Permit live delivery for outputs that request it".to_owned(),
            true
        )],
        "the row announces as a check box carrying the state it paints"
    );

    // A frame with no click leaves it exactly as it found it, so the toggle is
    // the click's doing rather than the render's.
    let (changed, announced) = frame(&mut value, true, false);
    assert!(!changed);
    assert!(value);
    assert_eq!(announced.first().map(|(_, on)| *on), Some(true));

    // And a disabled row is inert and says so, rather than being a switch that
    // silently ignores the reader.
    let mut value = false;
    let (changed, announced) = frame(&mut value, false, true);
    assert!(!changed, "a disabled row must not move the setting");
    assert!(!value);
    assert!(
        announced.is_empty()
            || announced
                .iter()
                .all(|(label, _)| label.contains("Permit live delivery")),
        "{announced:?}"
    );
}
