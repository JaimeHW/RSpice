//! The net highlight on the canvas, through real frames of the whole application: what
//! leaves it lit, what puts it out, and how it follows the drawing as it is edited.

use super::gestures_drawn::Fixture;
use rspice_app_state::state::Point;

/// Wires 1 (100,100)-(200,100) and 2 (100,100)-(100,200) of the fixture,
/// named VOUT by a label at the free end of wire 1.
fn vout() -> Fixture {
    let mut fixture = Fixture::new(true);
    let schematic = &mut fixture.app.state.model.schematic;
    schematic.add_net_label(Point::new(200, 100), "VOUT".to_owned());
    schematic.clear_undo_history();
    fixture.app.state.sync_active_schematic_to_workspace();
    fixture.frame(Vec::new(), true);
    assert!(
        fixture.app.state.light_net_named("VOUT"),
        "the label names the net"
    );
    fixture
}

fn lit(fixture: &Fixture) -> Option<&str> {
    let highlight = &fixture
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .net_highlight;
    highlight
        .active
        .then_some(highlight.selected_net_name.as_deref())
        .flatten()
}

fn wire_is_lit(fixture: &Fixture, id: u64) -> bool {
    fixture
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .net_highlight
        .is_wire_highlighted(id)
}

/// A press and release of `button` at `at` on the canvas, the pointer
/// having arrived there first.
fn click(fixture: &mut Fixture, at: egui::Vec2, button: egui::PointerButton) {
    let pos = fixture.origin + at;
    fixture.frame(vec![egui::Event::PointerMoved(pos)], true);
    for pressed in [true, false] {
        fixture.frame(
            vec![egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers: egui::Modifiers::NONE,
            }],
            true,
        );
    }
}

fn tap(fixture: &mut Fixture, key: egui::Key) {
    for pressed in [true, false] {
        fixture.frame(
            vec![egui::Event::Key {
                key,
                physical_key: Some(key),
                pressed,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            true,
        );
    }
}

/// A click selects and leaves the lit net lit, so it can be acted on; Esc
/// puts it out. Any click used to clear it, a right-click included, so the
/// lit net could not be shown in the netlist or have its properties opened
/// without going dark, and Esc never cleared it at all.
#[test]
fn a_lit_net_stays_lit_through_clicks_and_menus_until_escape() {
    let mut fixture = vout();

    click(
        &mut fixture,
        egui::vec2(150.0, 100.0),
        egui::PointerButton::Primary,
    );
    assert!(
        fixture
            .app
            .state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .contains(&1),
        "the click selects the wire"
    );
    assert_eq!(lit(&fixture), Some("VOUT"), "and the net stays lit");

    click(
        &mut fixture,
        egui::vec2(400.0, 300.0),
        egui::PointerButton::Primary,
    );
    assert!(
        fixture
            .app
            .state
            .model
            .schematic
            .session
            .editor
            .selection
            .is_empty(),
        "a click on the bare sheet lets go of the selection"
    );
    assert_eq!(lit(&fixture), Some("VOUT"), "and not of the light");

    tap(&mut fixture, egui::Key::Escape);
    assert_eq!(lit(&fixture), None, "Esc puts the light out");
    assert!(!wire_is_lit(&fixture, 1));

    assert!(fixture.app.state.light_net_named("VOUT"));
    click(
        &mut fixture,
        egui::vec2(150.0, 100.0),
        egui::PointerButton::Secondary,
    );
    assert_eq!(
        lit(&fixture),
        Some("VOUT"),
        "the menu opened on the lit net acts on it lit"
    );
}

/// The light is the net, not the wires it covered when it was lit: a wire
/// drawn onto it lights, and a cut leaves lit only the part that is still
/// that net.
#[test]
fn a_lit_net_follows_the_wires_drawn_onto_it_and_cut_from_it() {
    let mut fixture = vout();
    assert!(wire_is_lit(&fixture, 1) && wire_is_lit(&fixture, 2));

    let mut drawn = None;
    fixture
        .app
        .state
        .model
        .schematic
        .with_undo("draw wire", |schematic| {
            drawn = schematic.add_wire(vec![Point::new(100, 200), Point::new(200, 200)]);
        });
    let drawn = drawn.expect("the wire is drawn");
    fixture.frame(Vec::new(), true);
    assert!(
        wire_is_lit(&fixture, drawn),
        "the wire drawn onto VOUT lights"
    );

    // Cut wire 2, which joined the new wire to the labelled half.
    click(
        &mut fixture,
        egui::vec2(100.0, 150.0),
        egui::PointerButton::Primary,
    );
    tap(&mut fixture, egui::Key::Delete);
    assert!(
        !fixture
            .app
            .state
            .model
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.id == 2),
        "wire 2 is deleted"
    );
    assert_eq!(lit(&fixture), Some("VOUT"));
    assert!(wire_is_lit(&fixture, 1), "the labelled half is still VOUT");
    assert!(
        !wire_is_lit(&fixture, drawn),
        "the half cut off VOUT goes dark"
    );
}
