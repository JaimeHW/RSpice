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
