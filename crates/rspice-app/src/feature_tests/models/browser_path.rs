//! The Models workspace along the browser qualification's Models path,
//! replayed through whole frames of the application: the catalog's rows
//! pressed where their announcements say they are and walked with the arrow
//! keys.

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use crate::feature_tests::replay::{Replay, only};

/// A catalog row pressed at the centre of its announced bounds takes the
/// selection and the keyboard, and the arrow keys then walk the catalog a row
/// at a time, past the rows the table first showed.
#[test]
fn the_arrow_keys_walk_the_catalog_from_a_pressed_row() {
    let mut replay = Replay::new();
    replay.frames(Vec::new());
    replay.choose_command("Models");
    only(
        &replay.frames(Vec::new()),
        "RSPICE_DIODE in RSpice Foundation",
        Role::Button,
    );
    let order: Vec<String> = replay
        .frames(Vec::new())
        .iter()
        .filter(|control| control.role == Role::Button)
        .filter_map(|control| control.label.strip_suffix(" in RSpice Foundation"))
        .map(str::to_owned)
        .collect();
    assert!(order.len() >= 3, "the catalog shows its rows: {order:?}");

    replay.click("RSPICE_NMOS in RSpice Foundation", Role::Button);
    assert_eq!(
        replay.app.state.models.selected_model.as_deref(),
        Some("RSPICE_NMOS"),
        "the pressed row did not take the selection"
    );
    let at = order
        .iter()
        .position(|model| model == "RSPICE_NMOS")
        .expect("the NMOS row is listed");
    replay.key(Key::ArrowDown, Modifiers::NONE);
    assert_eq!(
        replay.app.state.models.selected_model.as_deref(),
        Some(order[at + 1].as_str()),
        "the down arrow moved the selection to the next row"
    );
    replay.key(Key::ArrowUp, Modifiers::NONE);
    replay.key(Key::ArrowUp, Modifiers::NONE);
    assert_eq!(
        replay.app.state.models.selected_model.as_deref(),
        Some(order[at - 1].as_str()),
        "the up arrow moved it back past the pressed row"
    );

    // Walking on past the last row the table first showed scrolls the rest in.
    let models = replay
        .app
        .state
        .model
        .model_library_manager
        .get_library("RSpice Foundation")
        .map(|library| library.model_count())
        .expect("the foundation library is loaded");
    for _ in 0..models {
        replay.key(Key::ArrowDown, Modifiers::NONE);
    }
    let last = replay.app.state.models.selected_model.clone();
    let shown = replay.frames(Vec::new());
    assert!(
        last.as_deref().is_some_and(|model| {
            shown
                .iter()
                .any(|control| control.label == format!("{model} in RSpice Foundation"))
        }),
        "the selected row {last:?} is on screen"
    );
    assert!(
        !order.contains(last.as_ref().expect("a row is selected")),
        "the arrows stopped at {last:?}, inside the rows first shown {order:?}"
    );
}
