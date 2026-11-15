//! Editing a symbol in place, driven through whole frames of the application.

use egui::accesskit::Role;

use crate::feature_tests::replay::{Control, Replay};
use crate::state::{CellViewRef, Point, Rotation};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design::commands::DesignCommand;

use super::symbol_in_place::amp_project;

/// A whole application on a sheet placing the `amp` symbol once, unturned
/// at (100, 80), its camera at 200%.
struct InPlace {
    replay: Replay,
    parent: CellViewRef,
    symbol: CellViewRef,
    instance: u64,
}

impl InPlace {
    fn new() -> Self {
        let mut replay = Replay::new();
        let (parent, symbol, instances) = amp_project(
            &mut replay.app,
            &[(Point::new(100, 80), Rotation::R0, false)],
        );
        replay.frames(vec![]);
        Self {
            replay,
            parent,
            symbol,
            instance: instances[0],
        }
    }

    /// Select the instance and edit its symbol in place by the command.
    fn enter(&mut self) -> Vec<Control> {
        self.replay
            .app
            .state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(self.instance);
        self.replay
            .app
            .execute_command_key(DesignCommand::EditSymbolInPlace.key());
        assert_eq!(
            self.replay.app.state.model.workspace.content.active_view,
            self.symbol
        );
        self.replay.frames(vec![])
    }

    fn in_place_zoom(&self) -> f64 {
        crate::workbench::app::in_place_session(
            &self.replay.app.state.design.symbol.in_place,
            &self.replay.app.state.model,
        )
        .expect("the symbol is edited in place")
        .zoom
    }
}

/// Press and let go of the primary button at `at`, a frame each.
fn click_at(replay: &mut Replay, at: egui::Pos2) -> Vec<Control> {
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(at), button(true)]);
    replay.frames(vec![button(false)])
}

/// The centre of the canvas the frame announces.
fn canvas_centre(controls: &[Control]) -> egui::Pos2 {
    controls
        .iter()
        .find(|control| control.role == Role::Canvas)
        .expect("the canvas is announced")
        .center
}

/// The status bar's prompt.
fn prompt(controls: &[Control]) -> String {
    controls
        .iter()
        .find(|control| control.role == Role::Status)
        .map(|control| control.value.clone())
        .unwrap_or_default()
}

/// A line half drawn in place holds the session when the breadcrumb is
/// pressed: the prompt names what is in flight, the line keeps its point,
/// and Escape then cancels the line, as it would anywhere.
#[test]
fn a_shape_in_flight_holds_the_in_place_edit_by_name() {
    let mut fixture = InPlace::new();
    let entered = fixture.enter();
    // The first press gives the canvas the keyboard; the second, with the
    // line tool armed, sets the line's first point.
    click_at(&mut fixture.replay, canvas_centre(&entered));
    fixture.replay.key(egui::Key::L, egui::Modifiers::NONE);
    click_at(&mut fixture.replay, canvas_centre(&entered));
    assert_eq!(
        fixture
            .replay
            .app
            .state
            .design
            .symbol
            .editor
            .draft_activity(),
        Some("a shape is being drawn")
    );

    let after = fixture.replay.click("top", Role::Link);
    assert_eq!(
        fixture.replay.app.state.model.workspace.content.active_view, fixture.symbol,
        "the breadcrumb did not leave"
    );
    assert_eq!(
        fixture
            .replay
            .app
            .state
            .design
            .symbol
            .editor
            .pending_polyline
            .len(),
        1,
        "the line kept its point"
    );
    assert!(
        prompt(&after).contains("a shape is being drawn"),
        "{}",
        prompt(&after)
    );

    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    assert_eq!(
        fixture
            .replay
            .app
            .state
            .design
            .symbol
            .editor
            .draft_activity(),
        None,
        "Escape cancelled the line"
    );
    fixture.replay.click("top", Role::Link);
    assert_eq!(
        fixture.replay.app.state.model.workspace.content.active_view, fixture.parent,
        "with nothing in flight the breadcrumb leaves"
    );
}

/// The status bar's zoom chip: the one button whose name is a percentage.
fn zoom_chip(controls: &[Control]) -> String {
    let chips: Vec<&str> = controls
        .iter()
        .filter(|control| control.role == Role::Button && control.label.ends_with('%'))
        .map(|control| control.label.as_str())
        .collect();
    assert_eq!(chips.len(), 1, "one zoom chip: {chips:?}");
    chips[0].to_owned()
}

/// The zoom chip reads the canvas in front. Edited in place, that is the
/// parent sheet's camera the session carries, not the zoom the symbol's own
/// view was last left at, and it follows the in-place view as it zooms.
#[test]
fn the_zoom_chip_reads_the_in_place_view() {
    let mut fixture = InPlace::new();
    let before = fixture.replay.frames(vec![]);
    assert_eq!(zoom_chip(&before), "200%");
    let entered = fixture.enter();
    assert_eq!(zoom_chip(&entered), "200%", "the sheet's own zoom");

    Command::ZoomIn.execute(&mut fixture.replay.app);
    let nearer = fixture.replay.frames(vec![]);
    let zoom = fixture.in_place_zoom();
    assert!(zoom > 2.0, "Zoom in magnified the in-place view");
    assert_eq!(
        zoom_chip(&nearer),
        format!("{}%", (zoom * 100.0).round()),
        "the chip follows the in-place view"
    );
}
