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
use crate::workbench::app_state::Model;
use crate::workbench::slots::DesignState;

/// Air between the command's name and its options.
const NAME_GAP: f32 = 12.0;

pub(super) fn show(ui: &mut Ui, model: &mut Model, design: &mut DesignState) {
    let top = ui.cursor().top();
    options(ui, model, design);
    let height = ui.cursor().top() - top;
    hold_drawing_still(ui, model, height);
    design.tool_strip_height = height;
}

/// The strip takes its height from the top of the canvas, so the canvas
/// moves down when the strip comes and back up when it goes. The drawing
/// must not move with it: whatever the pointer was over when M or S was
/// pressed is still under it for the click that follows. The pan takes the
/// change, once per change of height, per document.
fn hold_drawing_still(ui: &Ui, model: &mut Model, height: f32) {
    let id = egui::Id::new((
        "tool-options-strip-height",
        model.workspace.content.active_key(),
    ));
    let last = ui.data_mut(|data| {
        let last = data.get_temp::<f32>(id);
        data.insert_temp(id, height);
        last
    });
    if let Some(last) = last
        && last != height
    {
        model.schematic.session.editor.pan.1 -= f64::from(height - last);
    }
}

fn options(ui: &mut Ui, model: &mut Model, design: &mut DesignState) {
    match model.schematic.session.editor.tool {
        Tool::Bus => strip(ui, "Bus", |ui| bus_name(ui, model)),
        // A supply rail armed from the shelf places its one name; it has no
        // count to turn off.
        tool @ (Tool::Label | Tool::OffSheetConnector)
            if model
                .schematic
                .session
                .editor
                .pending_net_label_sequence
                .as_ref()
                .is_none_or(|sequence| sequence.repeated_name().is_none()) =>
        {
            strip(ui, tool.display_name(), |ui| label_count_up(ui, model));
        }
        Tool::MoveSelection if design.move_selection.armed => {
            strip(ui, "Move", |ui| {
                let mode = &mut design.move_selection.mode;
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
        Tool::StretchSelection if design.stretch_selection.armed => {
            strip(ui, "Stretch", |ui| {
                let policy = &mut design.stretch_selection.policy;
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

/// The Label strip's Count up: a name ending in a number counts up on each
/// click (OUT1, OUT2, OUT3), or with it off places again (OUT1, OUT1), the
/// usual way to join two points by name. A count already armed follows the
/// box at once, and the ghost shows the name the next click places.
fn label_count_up(ui: &mut Ui, model: &mut Model) {
    let editor = &mut model.schematic.session.editor;
    let mut count_up = editor.net_label_count_up;
    if ui
        .checkbox(&mut count_up, "Count up")
        .on_hover_text("A name ending in a number counts up on each click; off, it places again")
        .changed()
    {
        editor.set_net_label_count_up(count_up);
    }
}

/// The Bus strip's Name: what every bus drawn next is called, typed before
/// the first click, as Virtuoso's Add Wire form takes a wire name. A bus
/// being routed takes a name typed while it is drawn. Beside it, the width
/// the name declares or why it is refused.
fn bus_name(ui: &mut Ui, model: &mut Model) {
    let t = Tokens::get(ui.ctx());
    let drawing = &mut model.schematic.session.editor.bus_drawing;
    ui.label("Name");
    let response = ui.add(
        egui::TextEdit::singleline(&mut drawing.name)
            .id(egui::Id::new("bus-strip-name"))
            .hint_text(rspice_schematic_editor::bus_tap_placement::BUS_SHAPE)
            .desired_width(140.0),
    );
    let named = drawing.named();
    if response.changed()
        && drawing.active
        && let Ok(declaration) = &named
    {
        drawing.declaration = declaration.clone();
    }
    match named {
        Ok(Some(declaration)) => {
            ui.label(
                egui::RichText::new(format!("{} bits", declaration.width()))
                    .color(t.color.text_dim),
            );
        }
        Ok(None) => {}
        Err(error) => {
            ui.label(egui::RichText::new(format!("{error}")).color(t.color.warn));
        }
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
    use crate::workbench::design::test_host::DesignTestHost;

    fn armed_move() -> DesignTestHost {
        let mut state = DesignTestHost::default();
        let id = state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        crate::workbench::app::arm_move_selection_in(
            &mut state.design.move_selection,
            &mut state.design.stretch_selection,
            &mut state.design.array_selection,
            &mut state.model,
        );
        state
    }

    fn armed_stretch() -> DesignTestHost {
        let mut state = DesignTestHost::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(7, vec![Point::new(0, 0), Point::new(20, 0)]));
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_wire(7);
        crate::workbench::app::arm_stretch_selection_in(
            &mut state.design.stretch_selection,
            &mut state.design.move_selection,
            &mut state.design.array_selection,
            state.design.schematic_visibility.wire_routing,
            &mut state.model,
            &mut state.workbench,
        );
        state
    }

    /// The strip belongs to the armed command, not to the canvas: it is there
    /// while Move or Stretch is running and at no other time.
    #[test]
    fn the_strip_is_there_only_while_a_tool_with_an_option_is_armed() {
        fn painted(state: &mut DesignTestHost) -> bool {
            let mut shown = false;
            let _ = crate::ui::raster::render(egui::vec2(900.0, 60.0), |ui, _background| {
                let before = ui.cursor().top();
                show(ui, &mut state.model, &mut state.design);
                shown = ui.cursor().top() > before;
            });
            shown
        }

        assert!(!painted(&mut DesignTestHost::default()));
        assert!(painted(&mut armed_move()));
        assert!(painted(&mut armed_stretch()));
        let mut bus = DesignTestHost::default();
        bus.model.schematic.arm_tool(Tool::Bus);
        assert!(painted(&mut bus), "the bus tool shows its Name");

        let mut cancelled = armed_move();
        crate::workbench::app::cancel_armed_move_selection_in(
            &mut cancelled.design.move_selection,
            &mut cancelled.model,
        );
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
        let frame = |state: &mut DesignTestHost| {
            let mut measured = (0.0, 0.0);
            let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
                show(ui, &mut state.model, &mut state.design);
                let canvas_top = f64::from(ui.cursor().top());
                measured = (
                    canvas_top,
                    canvas_top + state.model.schematic.session.editor.pan.1,
                );
            });
            measured
        };
        let mut state = armed_stretch();
        crate::workbench::app::cancel_armed_stretch_selection_in(
            &mut state.design.stretch_selection,
            &mut state.model,
        );
        let (resting_top, resting) = frame(&mut state);

        crate::workbench::app::arm_stretch_selection_in(
            &mut state.design.stretch_selection,
            &mut state.design.move_selection,
            &mut state.design.array_selection,
            state.design.schematic_visibility.wire_routing,
            &mut state.model,
            &mut state.workbench,
        );
        let (armed_top, armed) = frame(&mut state);
        assert!(armed_top > resting_top, "the strip took its height");
        assert_eq!(armed, resting, "arming moved the drawing");

        crate::workbench::app::cancel_armed_stretch_selection_in(
            &mut state.design.stretch_selection,
            &mut state.model,
        );
        let (done_top, done) = frame(&mut state);
        assert_eq!(done_top, resting_top);
        assert_eq!(done, resting, "ending the command moved the drawing");
    }

    /// A label clicked on a pin end lands on that pin end and joins the
    /// pin's net. Arming the Label tool brings its strip in over the drawn
    /// canvas, and the canvas, which keeps the drawing in sight when it
    /// changes size, took the strip's band for a resize on top of the pan
    /// the strip had already taken up: the drawing moved half the strip's
    /// height out from under the pointer, and the click aimed at the pin end
    /// landed on the grid point beside it.
    #[test]
    fn a_label_clicked_on_a_pin_end_lands_on_it_and_joins_its_net() {
        use crate::state::{Cell, CellViewRef, Library, View, ViewType};
        use crate::workbench::app::dialogs::placement::net_label::{DraftValidation, tests};
        use crate::workbench::design::DesignFeature;
        use crate::workbench::framework::feature::harness::FeatureHarness;

        let mut host = DesignTestHost::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("top");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
        host.model.library_manager.add_library(library);
        host.open_workspace_view(CellViewRef::new("work", "top", "schematic"));
        let id = host
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(100, 100));
        let (pin, end) = host.model.schematic.document().components[0]
            .terminal_positions_resolved(None)
            .into_iter()
            .next()
            .expect("a resistor has pins");
        let DesignTestHost {
            design,
            model,
            workbench,
            ..
        } = host;
        let mut harness = FeatureHarness::<DesignFeature>::with_model(model, *workbench);
        harness.state = *design;

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let frame = |harness: &mut FeatureHarness<DesignFeature>, events: Vec<egui::Event>| {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 700.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| {
                    // As the shell lends the surface its context: no modal open.
                    let mut cx = harness
                        .cx()
                        .with_application_modal(false)
                        .with_dialog_modal(false);
                    super::super::show_surface(&mut cx, ui);
                },
            );
        };
        frame(&mut harness, Vec::new());
        // At 100 % with the drawing's origin on the canvas's corner, a pin
        // end's place on the screen is its coordinates from that corner.
        let editor = &mut harness.model.schematic.session.editor;
        (editor.needs_fit, editor.needs_drawing_sheet_fit) = (false, false);
        (editor.zoom, editor.pan) = (1.0, (0.0, 0.0));
        frame(&mut harness, Vec::new());
        let resting = harness.state.touch_strip_canvas.expect("the canvas");
        let at = resting.min + egui::vec2(end.x as f32, end.y as f32);

        harness.model.schematic.arm_tool(Tool::Label);
        frame(&mut harness, vec![egui::Event::PointerMoved(at)]);
        let armed = harness.state.touch_strip_canvas.expect("the canvas");
        assert!(armed.top() > resting.top(), "the Label strip took its band");
        for pressed in [true, false] {
            frame(
                &mut harness,
                vec![egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                }],
            );
        }
        let entry = &mut harness.state.net_label_placement;
        assert!(entry.open, "the click opens the label's entry");
        assert_eq!(
            entry.anchor,
            Some(end),
            "the label is anchored on the pin end"
        );

        entry.name = "VIN".to_owned();
        let DraftValidation::Valid(commit) = tests::validate_draft(entry, &harness.model) else {
            panic!("VIN is a net name");
        };
        tests::apply_commit(&mut harness.model, &mut harness.wb, *commit).expect("placed");
        let nets = rspice_design::connectivity::summary::design_nets(&harness.model.schematic);
        let net = nets
            .iter()
            .find(|net| net.name == "VIN")
            .expect("the label names a net");
        assert!(
            net.terminals
                .iter()
                .any(|terminal| terminal.component_id == id && terminal.pin == pin),
            "the pin {pin} is on VIN: {:?}",
            net.terminals
                .iter()
                .map(|terminal| (terminal.component_id, &terminal.pin))
                .collect::<Vec<_>>()
        );
    }
}
