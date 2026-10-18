//! The armed tool's options strip.
//!
//! A canvas command that has a choice left to make shows it here, over the
//! drawing, while the command runs — the way Virtuoso puts command options
//! beside the cursor rather than in front of it. The strip appears only while
//! a tool that has an option is armed, states the command and that option,
//! and nothing else: the snap is in the status bar, the selection is
//! highlighted on the canvas, and Escape cancels every command in the editor.

use egui::Ui;

use crate::state::{MoveSelectionMode, StretchOrthogonalPolicy, Tool};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{SegmentedWidth, segmented};
use crate::workbench::AppState;

/// Air between the command's name and its options.
const NAME_GAP: f32 = 12.0;

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    let top = ui.cursor().top();
    options(ui, state);
    hold_drawing_still(ui, state, ui.cursor().top() - top);
}

/// The strip takes its height from the top of the canvas, so the canvas
/// moves down when the strip comes and back up when it goes. The drawing
/// must not move with it: whatever the pointer was over when M or S was
/// pressed is still under it for the click that follows. The pan takes the
/// change, once per change of height, per document.
fn hold_drawing_still(ui: &Ui, state: &mut AppState, height: f32) {
    let id = egui::Id::new((
        "tool-options-strip-height",
        state.workspace.content.active_key(),
    ));
    let last = ui.data_mut(|data| {
        let last = data.get_temp::<f32>(id);
        data.insert_temp(id, height);
        last
    });
    if let Some(last) = last
        && last != height
    {
        state.schematic.session.editor.pan.1 -= f64::from(height - last);
    }
}

fn options(ui: &mut Ui, state: &mut AppState) {
    match state.schematic.session.editor.tool {
        Tool::MoveSelection if state.dialogs.move_selection.armed => {
            strip(ui, "Move", |ui| {
                let mode = &mut state.dialogs.move_selection.mode;
                let mut selected = MoveSelectionMode::ALL
                    .iter()
                    .position(|candidate| candidate == mode)
                    .unwrap_or(0);
                let labels = MoveSelectionMode::ALL.map(MoveSelectionMode::label);
                if segmented(
                    ui,
                    "move-selection-mode",
                    &labels,
                    &mut selected,
                    SegmentedWidth::Natural,
                ) {
                    *mode = MoveSelectionMode::ALL[selected];
                }
            });
        }
        Tool::StretchSelection if state.dialogs.stretch_selection.armed => {
            strip(ui, "Stretch", |ui| {
                let policy = &mut state.dialogs.stretch_selection.policy;
                let mut orthogonal = *policy == StretchOrthogonalPolicy::PreserveOrthogonal;
                if ui
                    .checkbox(&mut orthogonal, "Keep wires orthogonal")
                    .changed()
                {
                    *policy = if orthogonal {
                        StretchOrthogonalPolicy::PreserveOrthogonal
                    } else {
                        StretchOrthogonalPolicy::AllowDiagonal
                    };
                }
            });
        }
        _ => {}
    }
}

fn strip(ui: &mut Ui, command: &str, options: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(12, 5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = NAME_GAP;
                ui.label(
                    egui::RichText::new(command)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                options(ui);
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point, Wire};

    fn armed_move() -> AppState {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        crate::workbench::app::arm_move_selection(&mut state);
        state
    }

    fn armed_stretch() -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(7, vec![Point::new(0, 0), Point::new(20, 0)]));
        state.schematic.session.editor.selection.select_wire(7);
        crate::workbench::app::arm_stretch_selection(&mut state);
        state
    }

    /// The strip belongs to the armed command, not to the canvas: it is there
    /// while Move or Stretch is running and at no other time.
    #[test]
    fn the_strip_is_there_only_while_a_tool_with_an_option_is_armed() {
        fn painted(state: &mut AppState) -> bool {
            let mut shown = false;
            let _ = crate::ui::raster::render(egui::vec2(900.0, 60.0), |ui, _background| {
                let before = ui.cursor().top();
                show(ui, state);
                shown = ui.cursor().top() > before;
            });
            shown
        }

        assert!(!painted(&mut AppState::default()));
        assert!(painted(&mut armed_move()));
        assert!(painted(&mut armed_stretch()));

        let mut cancelled = armed_move();
        crate::workbench::app::cancel_armed_move_selection(&mut cancelled);
        assert!(!painted(&mut cancelled));
    }

    /// Pressing S or M brings the strip in above the canvas and pushes the
    /// canvas down by its height. The drawing moved down with it, out from
    /// under the pointer, so the click meant to take the segment the pointer
    /// was on landed 38 px beside it. The pan now takes up the strip's height
    /// as it comes and as it goes.
    #[test]
    fn the_drawing_stays_put_when_the_strip_comes_and_goes() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        // Where the canvas starts (the cursor below the strip) and where the
        // drawing's origin lands on screen.
        let frame = |state: &mut AppState| {
            let mut measured = (0.0, 0.0);
            let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
                show(ui, state);
                let canvas_top = f64::from(ui.cursor().top());
                measured = (
                    canvas_top,
                    canvas_top + state.schematic.session.editor.pan.1,
                );
            });
            measured
        };
        let mut state = armed_stretch();
        crate::workbench::app::cancel_armed_stretch_selection(&mut state);
        let (resting_top, resting) = frame(&mut state);

        crate::workbench::app::arm_stretch_selection(&mut state);
        let (armed_top, armed) = frame(&mut state);
        assert!(armed_top > resting_top, "the strip took its height");
        assert_eq!(armed, resting, "arming moved the drawing");

        crate::workbench::app::cancel_armed_stretch_selection(&mut state);
        let (done_top, done) = frame(&mut state);
        assert_eq!(done_top, resting_top);
        assert_eq!(done, resting, "ending the command moved the drawing");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_tool_options_strip() {
        use crate::ui::tokens::Mode;
        use std::io::Write as _;

        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        let stderr = std::io::stderr();
        let mut report = stderr.lock();

        for (stem, mut state) in [("move", armed_move()), ("stretch", armed_stretch())] {
            for (mode_stem, mode) in [("dark", Mode::Dark), ("light", Mode::Light)] {
                let canvas = crate::ui::raster::render_themed(
                    crate::ui::Theme {
                        mode,
                        ..crate::ui::Theme::default()
                    },
                    egui::vec2(900.0, 64.0),
                    |ui, _background| show(ui, &mut state),
                );
                let height = canvas.content_height().max(1);
                let path = directory.join(format!("tool-options-{stem}-{mode_stem}.png"));
                std::fs::write(&path, canvas.png(height)).expect("write strip render");
                writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                    .expect("write raster qualification report");
            }
        }
    }
}
