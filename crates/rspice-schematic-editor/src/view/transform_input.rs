//! Shared focus and keyboard input for transactional schematic transforms.

use super::drawing_sheet::DRAWING_SHEET_UNITS_PER_MM;
use crate::session::transform::TransformCanvasSession;
use egui::{Response, Ui};
use rspice_design_model::Point;

#[derive(Debug, Clone)]
pub struct TransformInputTransition<T = TransformCanvasSession> {
    pub expected: T,
    pub next: T,
    pub commit: bool,
}

pub(crate) fn retain_canvas_focus_from_pointer(response: &Response) {
    // Touch taps are reported as primary clicks by egui. A drag claims focus as
    // soon as it crosses the drag threshold so keyboard continuation works after
    // either pointer interaction without stealing focus from unrelated controls.
    if response.clicked_by(egui::PointerButton::Primary)
        || response.drag_started_by(egui::PointerButton::Primary)
    {
        response.request_focus();
    }
}

/// What the keyboard asked of an armed transform this frame.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct TransformKeys {
    /// The arrow keys, one grid step each.
    pub step: Point,
    /// Enter: land the transform.
    pub commit: bool,
    /// The distance typed before Enter, in drawing units, which the
    /// transform lands in place of its preview.
    pub typed: Option<Point>,
}

/// The arrow keys and Enter while the canvas has focus. Enter over a typed
/// distance that reads as two values lands that distance; over one that does
/// not, it is taken and lands nothing, and the readout says what is missing.
pub(crate) fn consume_keyboard(
    ui: &Ui,
    canvas_has_focus: bool,
    grid_size: i32,
    typed: &str,
) -> TransformKeys {
    if !canvas_has_focus {
        return TransformKeys::default();
    }

    ui.input_mut(|input| {
        let mut step = Point::origin();
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
            step.x = -grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
            step.x = grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
            step.y = -grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
            step.y = grid_size;
        }
        if !input.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
            return TransformKeys {
                step,
                ..TransformKeys::default()
            };
        }
        if typed.trim().is_empty() {
            return TransformKeys {
                step,
                commit: true,
                typed: None,
            };
        }
        match read_typed_distance(typed) {
            TypedDistance::Complete(distance) => TransformKeys {
                step,
                commit: true,
                typed: Some(distance),
            },
            TypedDistance::Partial | TypedDistance::Refused(_) => TransformKeys {
                step,
                ..TransformKeys::default()
            },
        }
    })
}

/// Take the keys that spell a distance into `typed` while a transform is
/// armed and the canvas has focus: a digit, a minus sign or a decimal point
/// begins one, and while it holds anything a separator (space or comma), a
/// sign and the letters of "mm" and "in" carry on. They are taken before any
/// shortcut sees them, so the minus types a sign rather than zooming out and
/// the m of "mm" does not arm Move. While the entry holds anything Backspace
/// takes back its last character and Esc clears it; neither leaves the
/// transform.
///
/// Returns whether this frame's keys belong to the entry, an Enter that will
/// land it included, so the caller holds shortcuts back for the frame.
pub fn capture_typed_distance(input: &mut egui::InputState, typed: &mut String) -> bool {
    let modifiers = input.modifiers;
    if modifiers.ctrl || modifiers.command || modifiers.mac_cmd || modifiers.alt {
        return false;
    }
    let mut took = false;
    if !typed.is_empty() {
        if input.consume_key(egui::Modifiers::NONE, egui::Key::Backspace) {
            typed.pop();
            took = true;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
            typed.clear();
            took = true;
        }
    }
    let mut took_text = false;
    input.events.retain(|event| {
        let egui::Event::Text(text) = event else {
            return true;
        };
        let mut entry = typed.clone();
        for character in text.chars() {
            let begins = character.is_ascii_digit() || matches!(character, '-' | '.');
            let carries = matches!(character, ' ' | ',' | '+' | 'm' | 'i' | 'n');
            if !(begins || (carries && !entry.is_empty())) {
                return true;
            }
            entry.push(character);
        }
        *typed = entry;
        took_text = true;
        false
    });
    if took_text {
        // The key presses behind the characters are the entry's too, so no
        // shortcut bound to them runs.
        input.events.retain(|event| {
            !matches!(
                event,
                egui::Event::Key { key, .. } if spells_a_distance(*key)
            )
        });
    }
    took |= took_text;
    took || (!typed.is_empty() && input.key_pressed(egui::Key::Enter))
}

fn spells_a_distance(key: egui::Key) -> bool {
    use egui::Key;
    matches!(
        key,
        Key::Num0
            | Key::Num1
            | Key::Num2
            | Key::Num3
            | Key::Num4
            | Key::Num5
            | Key::Num6
            | Key::Num7
            | Key::Num8
            | Key::Num9
            | Key::Minus
            | Key::Plus
            | Key::Equals
            | Key::Period
            | Key::Comma
            | Key::Space
            | Key::M
            | Key::I
            | Key::N
    )
}

/// What a typed distance says so far.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedDistance {
    /// Two values, in drawing units.
    Complete(Point),
    /// Fewer than two values, or a value still being typed.
    Partial,
    /// Why the entry cannot be a distance.
    Refused(String),
}

/// Read "dx dy" (a space or comma between them) in millimetres, the unit
/// the grid is named in; a value may say "mm", or "in" for inches. The
/// drawing counts quarter millimetres, so a value lands on the nearest one,
/// as a typed coordinate lands on the database grid in Virtuoso.
pub fn read_typed_distance(text: &str) -> TypedDistance {
    let values: Vec<&str> = text
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|value| !value.is_empty())
        .collect();
    let still_typing = !text.ends_with([',', ' ']);
    let mut distance = [0; 2];
    for (index, value) in values.iter().enumerate() {
        if index == 2 {
            return TypedDistance::Refused("type two distances, dx dy".to_owned());
        }
        match drawing_units(value) {
            Some(units) => distance[index] = units,
            None if still_typing && index + 1 == values.len() => return TypedDistance::Partial,
            None => return TypedDistance::Refused(format!("{value} is not a distance")),
        }
    }
    if values.len() == 2 {
        TypedDistance::Complete(Point::new(distance[0], distance[1]))
    } else {
        TypedDistance::Partial
    }
}

fn drawing_units(value: &str) -> Option<i32> {
    let (number, millimetres_per) = if let Some(number) = value.strip_suffix("mm") {
        (number, 1.0)
    } else if let Some(number) = value.strip_suffix("in") {
        (number, 25.4)
    } else {
        (value, 1.0)
    };
    let number = number
        .parse::<f64>()
        .ok()
        .filter(|number| number.is_finite())?;
    let units = (number * millimetres_per * DRAWING_SHEET_UNITS_PER_MM).round();
    (units.abs() <= f64::from(i32::MAX)).then_some(units as i32)
}

/// A distance in drawing units as the sheet reads it: "5, 2.5 mm", and on a
/// sheet that shows inches its length in inches as well, "5, 2.5 mm (0.197,
/// 0.098 in)", as the grid pitch is named there.
pub fn distance_label(distance: Point, inches: bool) -> String {
    let millimetres = |units: i32| f64::from(units) / DRAWING_SHEET_UNITS_PER_MM;
    let metric = format!(
        "{}, {} mm",
        millimetres(distance.x),
        millimetres(distance.y)
    );
    if !inches {
        return metric;
    }
    let inch = |units: i32| {
        let fixed = format!("{:.3}", millimetres(units) / 25.4);
        fixed.trim_end_matches('0').trim_end_matches('.').to_owned()
    };
    format!("{metric} ({}, {} in)", inch(distance.x), inch(distance.y))
}

/// The pointer readout while a distance is being typed: what it reads as
/// and that Enter lands it (`lands`, such as "moves"), what is still to
/// type, or why it cannot be a distance. The flag is false for the last.
pub fn typed_distance_readout(typed: &str, inches: bool, lands: &str) -> (bool, String) {
    match read_typed_distance(typed) {
        TypedDistance::Complete(distance) => (
            true,
            format!(
                "\u{0394} {} \u{b7} Enter {lands}",
                distance_label(distance, inches)
            ),
        ),
        TypedDistance::Partial => (
            true,
            format!(
                "\u{0394} {} \u{b7} type dx dy in mm, then Enter",
                typed.trim()
            ),
        ),
        TypedDistance::Refused(why) => (false, format!("\u{0394} {} \u{b7} {why}", typed.trim())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn move_keyboard_input() -> egui::RawInput {
        egui::RawInput {
            events: [egui::Key::ArrowRight, egui::Key::Enter]
                .into_iter()
                .map(|key| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                })
                .collect(),
            ..Default::default()
        }
    }
    #[test]
    fn armed_move_keyboard_leaves_keys_unconsumed_without_canvas_focus() {
        let ctx = egui::Context::default();
        let mut intent = None;
        let mut keys_remain = None;

        let _ = ctx.run_ui(move_keyboard_input(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // accessibility-pointer-shim: test-only canvas focus harness.
                let response = ui.interact(
                    ui.max_rect(),
                    egui::Id::new("unfocused-move-canvas"),
                    egui::Sense::click_and_drag(),
                );
                assert!(!response.has_focus());
                intent = Some(consume_keyboard(ui, response.has_focus(), 10, ""));
                keys_remain = Some(ui.input(|input| {
                    (
                        input.key_pressed(egui::Key::ArrowRight),
                        input.key_pressed(egui::Key::Enter),
                    )
                }));
            });
        });

        assert_eq!(intent, Some(TransformKeys::default()));
        assert_eq!(keys_remain, Some((true, true)));
    }
    #[test]
    fn armed_move_keyboard_consumes_keys_when_canvas_has_focus() {
        let ctx = egui::Context::default();
        let mut intent = None;
        let mut keys_remain = None;

        let _ = ctx.run_ui(move_keyboard_input(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // accessibility-pointer-shim: test-only canvas focus harness.
                let response = ui.interact(
                    ui.max_rect(),
                    egui::Id::new("focused-move-canvas"),
                    egui::Sense::click_and_drag(),
                );
                response.request_focus();
                assert!(response.has_focus());
                intent = Some(consume_keyboard(ui, response.has_focus(), 10, ""));
                keys_remain = Some(ui.input(|input| {
                    (
                        input.key_pressed(egui::Key::ArrowRight),
                        input.key_pressed(egui::Key::Enter),
                    )
                }));
            });
        });

        assert_eq!(
            intent,
            Some(TransformKeys {
                step: Point::new(10, 0),
                commit: true,
                typed: None,
            })
        );
        assert_eq!(keys_remain, Some((false, false)));
    }

    /// "dx dy" reads in millimetres (four drawing units to the millimetre),
    /// a value may say mm or in, and a value lands on the nearest quarter
    /// millimetre the drawing counts.
    #[test]
    fn a_typed_distance_reads_in_millimetres() {
        for (typed, distance) in [
            ("5 2.5", Point::new(20, 10)),
            ("-5,0", Point::new(-20, 0)),
            ("  .25  , -1  ", Point::new(1, -4)),
            ("5mm 0.1in", Point::new(20, 10)),
            ("0.3 0", Point::new(1, 0)),
        ] {
            assert_eq!(
                read_typed_distance(typed),
                TypedDistance::Complete(distance),
                "{typed:?}"
            );
        }
        for partial in ["", "5", "5 ", "-", "5 -", "5 2.5m", "5 0.1i"] {
            assert_eq!(
                read_typed_distance(partial),
                TypedDistance::Partial,
                "{partial:?}"
            );
        }
        assert_eq!(
            read_typed_distance("5mn 0"),
            TypedDistance::Refused("5mn is not a distance".to_owned())
        );
        assert_eq!(
            read_typed_distance("1 2 3"),
            TypedDistance::Refused("type two distances, dx dy".to_owned())
        );
    }

    #[test]
    fn a_distance_reads_in_millimetres_and_on_an_inch_sheet_in_inches_too() {
        assert_eq!(distance_label(Point::new(20, -10), false), "5, -2.5 mm");
        assert_eq!(
            distance_label(Point::new(20, 0), true),
            "5, 0 mm (0.197, 0 in)"
        );
    }

    fn typed_keys(text: &str) -> Vec<egui::Event> {
        text.chars()
            .flat_map(|character| {
                let key = match character {
                    '-' => egui::Key::Minus,
                    ' ' => egui::Key::Space,
                    'm' => egui::Key::M,
                    other => egui::Key::from_name(&other.to_string()).expect("a key"),
                };
                [
                    egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    },
                    egui::Event::Text(character.to_string()),
                ]
            })
            .collect()
    }

    /// The keys that spell a distance are the entry's: the minus and the m
    /// of "mm" leave no key press behind for zoom out or Move. An m with
    /// nothing typed is Move's, and Backspace and Esc edit only a begun entry.
    #[test]
    fn the_keys_that_spell_a_distance_are_taken_before_any_shortcut() {
        let ctx = egui::Context::default();
        let mut typed = String::new();
        let mut keys_left = Vec::new();
        for events in [
            typed_keys("m"),
            typed_keys("-5mm 2"),
            vec![egui::Event::Key {
                key: egui::Key::Backspace,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
        ] {
            let _ = ctx.run_ui(
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                |ctx| {
                    let took = ctx.input_mut(|input| capture_typed_distance(input, &mut typed));
                    keys_left.push((
                        took,
                        ctx.input(|input| {
                            [egui::Key::M, egui::Key::Minus, egui::Key::Backspace]
                                .map(|key| input.key_pressed(key))
                        }),
                    ));
                },
            );
        }

        assert_eq!(typed, "-5mm ");
        assert_eq!(
            keys_left,
            [
                (false, [true, false, false]),
                (true, [false, false, false]),
                (true, [false, false, false]),
            ]
        );
    }
}
