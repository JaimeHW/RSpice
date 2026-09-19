//! What Create pins states, arms and refuses.
//!
//! The claims worth pinning here are the ones a reader cannot check by eye:
//! that a batch is split the way the model splits names, that a coercion moves
//! the field the reader did not touch rather than refusing the one they did,
//! that pressing the primary changes nothing in the document, and that the
//! whole surface fits on a phone with the worst draft anyone could type into
//! it.

use super::*;
use crate::state::{Point, PortDirection, PortDiscipline, PortSignalType};
#[cfg(not(target_arch = "wasm32"))]
use crate::ui::tokens::Mode;

fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1_100.0, 850.0),
        )),
        events,
        ..Default::default()
    }
}

fn key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

fn open_with(state: &mut AppState, names: &str) {
    open_create_pins(state);
    state.dialogs.pin_port.names = names.to_owned();
}

/// One port already on the sheet, so duplicate refusals have something to
/// collide with.
fn place_port(state: &mut AppState, name: &str) {
    let id = state
        .schematic
        .add_component(crate::state::ComponentType::Port, Point::origin());
    state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed port exists")
        .value = name.to_owned();
}

#[test]
fn names_split_on_whitespace_and_each_is_validated_in_the_models_words() {
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "  INP   INN\tOUT  ");
    assert_eq!(
        draft(&app.state),
        Draft::Ready(vec!["INP".to_owned(), "INN".to_owned(), "OUT".to_owned()])
    );

    // A refusal is the model's sentence, prefixed with the name it is about
    // only when there is more than one name to be about.
    app.state.dialogs.pin_port.names = "DATA[3]".to_owned();
    let expected = app
        .state
        .schematic
        .validate_new_port_name("DATA[3]")
        .expect_err("one member of a bus is not a pin")
        .to_string();
    assert_eq!(draft(&app.state), Draft::Refused(expected.clone()));

    app.state.dialogs.pin_port.names = "OK DATA[3]".to_owned();
    assert_eq!(
        draft(&app.state),
        Draft::Refused(format!("DATA[3]: {expected}"))
    );
}

#[test]
fn a_name_listed_twice_is_refused_before_arming() {
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "EN OUT en");
    assert_eq!(
        draft(&app.state),
        Draft::Refused("en is listed twice.".to_owned())
    );

    let names = (0..=MAX_NAMES)
        .map(|index| format!("P{index}"))
        .collect::<Vec<_>>()
        .join(" ");
    app.state.dialogs.pin_port.names = names;
    assert_eq!(draft(&app.state), Draft::Refused(TOO_MANY.to_owned()));
}

#[test]
fn supply_coerces_signal_to_power_and_power_coerces_direction() {
    let mut signal = PortSignalType::Analog;
    coerce_from_direction(PortDirection::Supply, &mut signal);
    assert_eq!(signal, PortSignalType::Power);
    // Leaving Supply for a one-way direction leaves Power behind: (In, Power)
    // is not a contract the model accepts, and the reader asked for In.
    coerce_from_direction(PortDirection::In, &mut signal);
    assert_eq!(signal, PortSignalType::Analog);
    // Inout carries power without being a rail, so nothing moves.
    let mut power = PortSignalType::Power;
    coerce_from_direction(PortDirection::InOut, &mut power);
    assert_eq!(power, PortSignalType::Power);

    let mut direction = PortDirection::In;
    coerce_from_signal(PortSignalType::Power, &mut direction);
    assert_eq!(direction, PortDirection::InOut);
    let mut supply = PortDirection::Supply;
    coerce_from_signal(PortSignalType::Logic, &mut supply);
    assert_eq!(supply, PortDirection::InOut);
    // Coercion is total: from every pair the form can be in, picking any
    // direction or any signal type lands on a pair the model accepts. This is
    // the whole claim — a coerced field is never a refused one.
    let directions = [
        PortDirection::In,
        PortDirection::Out,
        PortDirection::InOut,
        PortDirection::Supply,
    ];
    let signals = [
        PortSignalType::Analog,
        PortSignalType::Logic,
        PortSignalType::Power,
    ];
    let mut state = crate::state::SchematicState::default();
    let mut placeable = |direction: PortDirection, signal: PortSignalType, picked: &str| {
        let pending = crate::state::PendingPortPlacement::from_contract(
            format!("PIN_{}", state.next_interface_order()),
            direction,
            signal,
            PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        state
            .place_pending_port(Point::origin(), pending)
            .unwrap_or_else(|error| {
                panic!("picking {picked} settled on {direction:?}/{signal:?}: {error}")
            });
    };
    for was_direction in directions {
        for was_signal in signals {
            for picked in directions {
                let mut signal = was_signal;
                coerce_from_direction(picked, &mut signal);
                placeable(picked, signal, &format!("{picked:?}"));
            }
            for picked in signals {
                let mut direction = was_direction;
                coerce_from_signal(picked, &mut direction);
                placeable(direction, picked, &format!("{picked:?}"));
            }
        }
    }
}

#[test]
fn discipline_follows_signal_until_touched() {
    assert_eq!(discipline_for(PortSignalType::Logic), PortDiscipline::Logic);
    assert_eq!(
        discipline_for(PortSignalType::Analog),
        PortDiscipline::Electrical
    );
    assert_eq!(
        discipline_for(PortSignalType::Power),
        PortDiscipline::Electrical
    );

    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "EN");
    assert!(!app.state.dialogs.pin_port.discipline_touched);
    app.state.dialogs.pin_port.signal_type = PortSignalType::Logic;
    app.state.dialogs.pin_port.discipline = discipline_for(PortSignalType::Logic);
    assert_eq!(app.state.dialogs.pin_port.discipline, PortDiscipline::Logic);

    // Once picked, the discipline is the reader's and nothing moves it.
    app.state.dialogs.pin_port.discipline = PortDiscipline::Wreal;
    app.state.dialogs.pin_port.discipline_touched = true;
    app.state.dialogs.pin_port.signal_type = PortSignalType::Analog;
    assert_eq!(app.state.dialogs.pin_port.discipline, PortDiscipline::Wreal);
}

#[test]
fn the_derived_line_states_pins_conductors_and_port_list_positions() {
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "EN");
    assert_eq!(
        declaration_line(&app.state, draft(&app.state).names()).as_deref(),
        Some("1 pin \u{00b7} 1 conductor \u{00b7} port-list position 1")
    );
    assert_eq!(deck_bits_line(draft(&app.state).names()), None);

    app.state.dialogs.pin_port.names = "EN DATA[7:0] OUT".to_owned();
    assert_eq!(
        declaration_line(&app.state, draft(&app.state).names()).as_deref(),
        Some("3 pins \u{00b7} 10 conductors \u{00b7} port-list positions 1 to 3")
    );
    // The deck bits are shown for the one case a reader cannot work out: a
    // single name that declares a range.
    assert_eq!(deck_bits_line(draft(&app.state).names()), None);

    app.state.dialogs.pin_port.names = "DATA[7:0]".to_owned();
    assert_eq!(
        deck_bits_line(draft(&app.state).names()).as_deref(),
        Some("DATA#7 DATA#6 DATA#5 DATA#4 DATA#3 DATA#2 DATA#1 DATA#0")
    );

    // An empty draft states nothing rather than stating zero.
    app.state.dialogs.pin_port.names.clear();
    assert_eq!(
        declaration_line(&app.state, draft(&app.state).names()),
        None
    );
}

#[test]
fn enter_arms_the_whole_sequence_without_touching_the_document() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "INP INN OUT VDD");
    app.state.dialogs.pin_port.signal_type = PortSignalType::Logic;
    app.state.dialogs.pin_port.discipline = PortDiscipline::Logic;

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        app.render_pin_port_dialog(ctx)
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        app.render_pin_port_dialog(ctx)
    });

    assert!(!app.state.dialogs.pin_port.open);
    assert_eq!(
        app.state.schematic.tool,
        Tool::Place(crate::state::ComponentType::Port)
    );
    let sequence = app
        .state
        .schematic
        .pending_port_sequence
        .as_ref()
        .expect("the batch is armed");
    assert_eq!(
        sequence.names.iter().cloned().collect::<Vec<_>>(),
        ["INP", "INN", "OUT", "VDD"]
    );
    assert_eq!(sequence.total, 4);
    assert_eq!(sequence.direction, PortDirection::In);
    assert_eq!(sequence.signal_type, PortSignalType::Logic);
    assert_eq!(sequence.discipline, PortDiscipline::Logic);
    assert!(sequence.authority.is_some());

    assert!(app.state.schematic.components.is_empty());
    assert!(!app.state.schematic.is_dirty);
    assert!(!app.state.schematic.can_undo());
    // The canvas owns the keyboard, so R, M and Esc work before the pointer
    // has moved over it.
    assert_eq!(
        ctx.memory(|memory| memory.focused()),
        Some(egui::Id::new("rspice-schematic-canvas-interaction"))
    );
}

#[test]
fn reopening_while_armed_offers_the_remaining_names() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "INP INN OUT");
    app.state.dialogs.pin_port.signal_type = PortSignalType::Logic;
    app.state.dialogs.pin_port.discipline_touched = true;
    app.state.dialogs.pin_port.discipline = PortDiscipline::Wreal;

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        app.render_pin_port_dialog(ctx)
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        app.render_pin_port_dialog(ctx)
    });
    app.state
        .schematic
        .pending_port_sequence
        .as_mut()
        .expect("armed")
        .advance();

    open_create_pins(&mut app.state);
    assert!(app.state.dialogs.pin_port.open);
    assert_eq!(app.state.dialogs.pin_port.names, "INN OUT");
    assert_eq!(
        app.state.dialogs.pin_port.signal_type,
        PortSignalType::Logic
    );
    assert_eq!(app.state.dialogs.pin_port.discipline, PortDiscipline::Wreal);
}

/// The prefill suggests from the last batch and nothing else. It is empty the
/// first time, because a name invented before the reader has named anything is
/// sample data.
#[test]
fn the_prefill_is_empty_until_a_batch_has_been_armed() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_create_pins(&mut app.state);
    assert_eq!(app.state.dialogs.pin_port.names, "");

    app.state.dialogs.pin_port.names = "BIAS".to_owned();
    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        app.render_pin_port_dialog(ctx)
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
        app.render_pin_port_dialog(ctx)
    });
    app.state.schematic.cancel_tool();
    place_port(&mut app.state, "BIAS");

    open_create_pins(&mut app.state);
    assert_eq!(app.state.dialogs.pin_port.names, "BIAS_2");
}

#[test]
fn a_read_only_or_changed_document_blocks_the_form_and_says_so() {
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "EN");
    app.state.schematic.read_only = true;
    assert_eq!(draft(&app.state), Draft::Blocked(READ_ONLY));

    app.state.schematic.read_only = false;
    assert!(matches!(draft(&app.state), Draft::Ready(_)));
    app.state.active_schematic_epoch = app.state.active_schematic_epoch.wrapping_add(1);
    assert_eq!(draft(&app.state), Draft::Blocked(DOCUMENT_CHANGED));
}

#[test]
fn a_blocked_form_never_arms_on_enter() {
    for blocked in [0, 1] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_with(&mut app.state, "EN");
        if blocked == 0 {
            app.state.schematic.read_only = true;
        } else {
            app.state.active_schematic_epoch = app.state.active_schematic_epoch.wrapping_add(1);
        }

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_pin_port_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_pin_port_dialog(ctx)
        });

        assert!(app.state.dialogs.pin_port.open);
        assert_eq!(app.state.schematic.tool, Tool::Select);
        assert!(app.state.schematic.pending_port_sequence.is_none());
        assert!(app.state.schematic.components.is_empty());
    }
}

/// Escape closes at once. There is no draft worth a confirmation here: the
/// form owns nothing but four answers, and the command is one chord away.
#[test]
fn escape_closes_the_form_at_once() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    open_with(&mut app.state, "INP INN");

    let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
        app.render_pin_port_dialog(ctx)
    });
    let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
        app.render_pin_port_dialog(ctx)
    });

    assert!(!app.state.dialogs.pin_port.open);
    assert!(app.state.schematic.pending_port_sequence.is_none());
    assert!(app.state.schematic.components.is_empty());
}

/// A duplicate of a name already on the sheet is refused before arming, in the
/// model's words.
#[test]
fn a_name_already_on_the_sheet_is_refused_before_arming() {
    let mut app = RSpiceApp::test_instance();
    place_port(&mut app.state, "BIAS_EN");
    open_with(&mut app.state, "bias_en");
    let expected = app
        .state
        .schematic
        .validate_new_port_name("bias_en")
        .expect_err("a taken name is refused")
        .to_string();
    assert_eq!(draft(&app.state), Draft::Refused(expected));
}

#[test]
fn every_rendered_string_is_free_of_mojibake_markers() {
    let mut strings = vec![
        TITLE,
        PRIMARY,
        DESCRIPTION,
        NAMES_LABEL,
        NAMES_HINT,
        DIRECTION_LABEL,
        SIGNAL_LABEL,
        DISCIPLINE_LABEL,
        READ_ONLY,
        DOCUMENT_CHANGED,
        TOO_MANY,
    ];
    strings.extend(DIRECTION_SEGMENTS);
    strings.extend(SIGNAL_SEGMENTS);
    for value in strings {
        for forbidden in ['\u{00c2}', '\u{00e2}', '\u{fffd}'] {
            assert!(
                !value.contains(forbidden),
                "mojibake in a rendered string: {value:?}"
            );
        }
    }
}

// ============================================================================
// Fit
// ============================================================================

/// Two dozen long names, which is the longest derived line the form can state.
#[cfg(not(target_arch = "wasm32"))]
fn two_dozen_names() -> String {
    (0..24)
        .map(|index| format!("VERY_LONG_INTERFACE_NAME_{index}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The two tallest drafts: a valid batch, whose derived line wraps, and a
/// refused one, whose message runs the full two lines the slot reserves.
#[cfg(not(target_arch = "wasm32"))]
fn worst_cases() -> [(&'static str, fn(&mut AppState)); 2] {
    [
        ("valid", |state: &mut AppState| {
            open_create_pins(state);
            state.dialogs.pin_port.names = two_dozen_names();
        }),
        ("refused", |state: &mut AppState| {
            place_port(state, "VERY_LONG_INTERFACE_NAME_7");
            open_create_pins(state);
            state.dialogs.pin_port.names = two_dozen_names();
        }),
    ]
}

/// Nothing the form paints leaves the card, the card never outgrows the
/// viewport, and the footer stays inside it — at the desktop width, at the
/// breakpoint, and on a portrait phone, in both themes.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn create_pins_fits_every_viewport() {
    for screen in [
        egui::vec2(1024.0, 640.0),
        egui::vec2(760.0, 900.0),
        egui::vec2(390.0, 844.0),
    ] {
        for mode in [Mode::Dark, Mode::Light] {
            for (draft_stem, prepare) in worst_cases() {
                let mut app = RSpiceApp::test_instance();
                prepare(&mut app.state);
                let names = app.state.dialogs.pin_port.names.clone();
                let painted =
                    crate::ui::widgets::painted_runs::painted_runs(screen, mode, 3, |ctx| {
                        app.render_pin_port_dialog(ctx);
                    });
                let label = format!("{screen:?} {mode:?} {draft_stem}");
                painted.assert_inside_clip_and_surface_with_horizontal_scroll(&label, &[&names]);
                assert!(
                    painted.surface.height() < screen.y,
                    "{label}: the form is taller than the viewport ({:?})",
                    painted.surface
                );
                for verb in [PRIMARY, "Cancel"] {
                    let rect = painted
                        .rect_of(verb)
                        .unwrap_or_else(|| panic!("{label}: {verb} was not painted"));
                    assert!(
                        painted.surface.expand(1.0).contains_rect(rect),
                        "{label}: {verb} at {rect:?} is outside the surface {:?}",
                        painted.surface
                    );
                }
                // The footer is one row: the two verbs share a baseline.
                let primary = painted.rect_of(PRIMARY).expect("primary");
                let ghost = painted.rect_of("Cancel").expect("ghost");
                assert!(
                    (primary.center().y - ghost.center().y).abs() <= 1.0,
                    "{label}: the footer stacked ({primary:?} / {ghost:?})"
                );
            }
        }
    }
}

// ============================================================================
// Renders for a human to look at
// ============================================================================

/// Every reviewable state of the form, at every viewport, in both themes.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_create_pins() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    for (stem, prepare) in [
        ("empty", (|_: &mut AppState| {}) as fn(&mut AppState)),
        ("four-names", |state: &mut AppState| {
            state.dialogs.pin_port.names = "INP INN OUT VDD".to_owned();
        }),
        ("refusal", |state: &mut AppState| {
            place_port(state, "OUT");
            state.dialogs.pin_port.names = "INP INN OUT VDD".to_owned();
        }),
        ("read-only", |state: &mut AppState| {
            state.dialogs.pin_port.names = "INP INN".to_owned();
            state.schematic.read_only = true;
        }),
    ] {
        for (size_stem, size) in [
            ("1024x640", egui::vec2(1024.0, 640.0)),
            ("760x900", egui::vec2(760.0, 900.0)),
            ("390x844", egui::vec2(390.0, 844.0)),
        ] {
            for (mode_stem, mode) in [("dark", Mode::Dark), ("light", Mode::Light)] {
                let mut app = RSpiceApp::test_instance();
                open_create_pins(&mut app.state);
                prepare(&mut app.state);
                let canvas = crate::ui::raster::render_themed(
                    crate::ui::Theme {
                        mode,
                        ..crate::ui::Theme::default()
                    },
                    size,
                    |ui, _background| {
                        app.render_pin_port_dialog(ui.ctx());
                    },
                );
                let height = canvas.content_height().max(1);
                let path =
                    directory.join(format!("create-pins-{stem}-{size_stem}-{mode_stem}.png"));
                std::fs::write(&path, canvas.png(height)).expect("write dialog render");
                writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                    .expect("write raster qualification report");
            }
        }
    }
}
