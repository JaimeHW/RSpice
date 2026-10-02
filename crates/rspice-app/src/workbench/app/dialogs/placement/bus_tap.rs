//! App-owned modal, source validation and arming for bus-tap placement.

use crate::state::Tool;
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use egui::Context;
use rspice_schematic_editor::{
    bus_tap_placement::{self, DraftValidation},
    requests::EditorRequestSource,
    session::bus::PendingBusTapPlacement,
};

const EYEBROW: &str = "SCHEMATIC \u{00b7} CONNECTIVITY";
const TITLE: &str = "Place bus tap";
const PRIMARY: &str = "Arm bus-tap tool";
const BODY: &str = "Create a typed scalar or slice connection from a declared bus with direction and naming validation.";
const DIALOG_SIZE: DialogSize = DialogSize::Transaction;
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No project or result data has been changed.";

impl RSpiceApp {
    pub(in crate::workbench) fn render_bus_tap_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.bus_tap.open {
            return;
        }

        let validation = match placement_source(&self.state) {
            Ok(_) => self.state.dialogs.bus_tap.fields.validate(),
            Err(message) => DraftValidation::Invalid(message),
        };
        let can_commit = validation.can_commit();

        let discard_confirm = self.state.dialogs.bus_tap.discard_confirm;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(BODY)
            .size(DIALOG_SIZE)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(can_commit)
            .initial_focus(DialogInitialFocus::Control(
                bus_tap_placement::bus_field_id(),
            ));
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }
        let mut response = dialog.show_transaction(ctx, |ui| {
            let (focus, edited) =
                bus_tap_placement::show(ui, &validation, &mut self.state.dialogs.bus_tap.fields);
            if edited {
                self.state.dialogs.bus_tap.mark_edited();
            }
            focus
        });

        match response.choice {
            DialogChoice::Primary => {
                // The footer follows the body in the same immediate-mode
                // frame. Re-parse the post-edit draft so Enter can never
                // publish the prior frame's valid contract after a field was
                // changed to an invalid value.
                if let Ok(source) = placement_source(&self.state)
                    && let DraftValidation::Valid(pending) =
                        self.state.dialogs.bus_tap.fields.validate()
                {
                    let placement = PendingBusTapPlacement::new(pending, source.clone());
                    self.state.schematic.session.editor.pending_bus_tap = Some(placement);
                    self.state.schematic.arm_tool(Tool::BusTap);
                    self.state.dialogs.bus_tap.close();
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.bus_tap.attempt_close();
                if self.state.dialogs.bus_tap.open {
                    response.retain_cancel_focus(DialogInitialFocus::Ghost);
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

pub(crate) fn open_bus_tap(state: &mut AppState) {
    let source = crate::workbench::app::schematic_editor_request_source(state);
    state.dialogs.bus_tap.open(source);
}

fn placement_source(state: &AppState) -> Result<&EditorRequestSource, String> {
    if state.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let source = state.dialogs.bus_tap.source.as_ref().ok_or_else(|| {
        "The bus-tap placement context is unavailable. Close and reopen Place bus tap.".to_owned()
    })?;
    if source != &crate::workbench::app::schematic_editor_request_source(state) {
        return Err(
            "The schematic source or editing scope changed. Close and reopen Place bus tap."
                .to_owned(),
        );
    }
    Ok(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BusTapOrientation;
    use crate::workbench::app::dialogs::state::BusTapDialogState;

    fn test_source() -> EditorRequestSource {
        crate::workbench::app::schematic_editor_request_source(&AppState::default())
    }

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

    fn replace_text(value: &str) -> Vec<egui::Event> {
        vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: Some(egui::Key::A),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
            },
            egui::Event::Paste(value.to_owned()),
        ]
    }

    #[test]
    fn opening_input_reaches_the_bus_field() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_bus_tap(&mut app.state);

        let _ = ctx.run_ui(dialog_input(replace_text("DATA[31:0]")), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });

        assert_eq!(app.state.dialogs.bus_tap.fields.bus, "DATA[31:0]");
        assert!(app.state.dialogs.bus_tap.dirty);
        assert!(app.state.schematic.session.editor.pending_bus_tap.is_none());
    }

    #[test]
    fn edited_cancel_then_enter_discards_and_restores_workspace_focus() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let workspace_id = egui::Id::new("bus-tap-test-workspace");
        let mut workspace_text = "workspace".to_owned();
        let mut render = |events, body: &mut dyn FnMut(&Context)| {
            let _ = ctx.run_ui(dialog_input(events), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut workspace_text).id(workspace_id));
                });
                body(ctx);
            });
        };
        render(Vec::new(), &mut |ctx| app.render_bus_tap_dialog(ctx));
        ctx.memory_mut(|memory| memory.request_focus(workspace_id));
        open_bus_tap(&mut app.state);
        render(Vec::new(), &mut |ctx| app.render_bus_tap_dialog(ctx));

        let mut events = replace_text("DATA[31:0]");
        events.extend([key_event(egui::Key::Escape), key_event(egui::Key::Enter)]);
        render(events, &mut |ctx| app.render_bus_tap_dialog(ctx));
        assert_eq!(app.state.dialogs.bus_tap.fields.bus, "DATA[31:0]");
        assert!(app.state.dialogs.bus_tap.discard_confirm);
        assert!(app.state.dialogs.bus_tap.open);
        for _ in 0..4 {
            render(Vec::new(), &mut |ctx| app.render_bus_tap_dialog(ctx));
        }

        assert!(!app.state.dialogs.bus_tap.open);
        assert!(app.state.schematic.session.editor.pending_bus_tap.is_none());
        assert_eq!(app.state.schematic.session.editor.tool, Tool::Select);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(workspace_id));
        assert_eq!(workspace_text, "workspace");
    }

    #[test]
    fn dialog_opens_with_the_mockup_values_and_a_valid_clean_draft() {
        let mut draft = BusTapDialogState::default();
        let source = test_source();
        draft.open(source.clone());

        assert!(draft.open);
        assert_eq!(draft.fields.bus, "DATA[15:0]");
        assert_eq!(draft.fields.slice, "DATA[7:0]");
        assert_eq!(draft.fields.orientation, BusTapOrientation::Automatic);
        assert!(!draft.dirty);
        assert!(!draft.discard_confirm);
        assert!(draft.fields.validate().can_commit());
        assert_eq!(draft.source.as_ref(), Some(&source));
        draft.close();
        assert!(draft.source.is_none());
        let mut next = source;
        next.document_epoch += 1;
        draft.open(next.clone());
        assert_eq!(draft.source.as_ref(), Some(&next));
    }

    #[test]
    fn edited_drafts_require_an_explicit_second_discard_action() {
        let mut draft = BusTapDialogState::default();
        draft.open(test_source());
        assert!(draft.attempt_close());
        assert!(!draft.open);

        draft.open(test_source());
        draft.mark_edited();
        assert!(!draft.attempt_close());
        assert!(draft.open);
        assert!(draft.discard_confirm);

        draft.mark_edited();
        assert!(!draft.discard_confirm);
        assert!(!draft.attempt_close());
        assert!(draft.attempt_close());
        assert!(!draft.open);
    }

    #[test]
    fn rendered_primary_revalidates_publishes_and_arms_the_bus_tap_tool() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_bus_tap(&mut app.state);

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });

        assert!(!app.state.dialogs.bus_tap.open);
        assert_eq!(app.state.schematic.session.editor.tool, Tool::BusTap);
        let pending = app
            .state
            .schematic
            .session
            .editor
            .pending_bus_tap
            .as_ref()
            .expect("validated pending bus tap");
        assert_eq!(
            pending.configuration.bus_declaration.to_string(),
            "DATA[15:0]"
        );
        assert_eq!(pending.configuration.slice.to_string(), "DATA[7:0]");
        assert!(pending.authority.matches(
            &crate::workbench::app::schematic_editor_request_source(&app.state)
        ));
    }

    #[test]
    fn retained_dialog_cannot_arm_after_its_context_or_permission_changes() {
        for change in [
            "design",
            "buffer",
            "occurrence",
            "sheet",
            "content",
            "read-only",
            "safe-mode",
            "missing",
        ] {
            let ctx = Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut app = RSpiceApp::test_instance();
            let master = crate::state::CellViewRef::new("work", "tap_parent", "schematic");
            app.state.workspace.descend_into(
                "X1".to_owned(),
                master.clone(),
                crate::state::ViewType::Schematic,
            );
            let first = app
                .state
                .workspace
                .content
                .design_management
                .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
                .unwrap();
            open_bus_tap(&mut app.state);
            let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
                app.render_bus_tap_dialog(ctx)
            });
            match change {
                "design" => app.state.design_execution_epoch += 1,
                "buffer" => app.state.active_schematic_epoch += 1,
                "occurrence" => {
                    app.state.workspace.ascend_one().unwrap();
                    app.state.workspace.descend_into(
                        "X2".to_owned(),
                        master.clone(),
                        crate::state::ViewType::Schematic,
                    );
                }
                "sheet" => {
                    let catalog = app
                        .state
                        .workspace
                        .content
                        .design_management
                        .sheet_catalog_mut(&master.key())
                        .unwrap();
                    let second = catalog
                        .create_sheet(
                            crate::state::SheetDefinition {
                                name: "Sheet 2".to_owned(),
                                template: crate::state::SheetTemplate::AnalogSchematic,
                                port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                                explicit_page_number: Some(2),
                            },
                            Some(first),
                        )
                        .unwrap();
                    catalog.set_active(second).unwrap();
                }
                "content" => {
                    app.state.schematic.add_component(
                        crate::state::ComponentType::Resistor,
                        crate::state::Point::new(40, 20),
                    );
                }
                "read-only" => app.state.schematic.session.read_only = true,
                "safe-mode" => app.state.workbench.safe_mode.activate(
                    crate::workbench::state::LocalSafeModeOptions {
                        open_project_read_only: true,
                        ..Default::default()
                    },
                    "bus-tap test".to_owned(),
                ),
                "missing" => app.state.dialogs.bus_tap.source = None,
                _ => unreachable!(),
            }
            assert!(placement_source(&app.state).is_err(), "{change}");
            let content = app.state.schematic.content_version();
            let topology = app.state.schematic.topology_version();
            let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
                app.render_bus_tap_dialog(ctx)
            });
            assert!(app.state.dialogs.bus_tap.open, "{change}");
            assert_eq!(
                app.state.schematic.session.editor.tool,
                Tool::Select,
                "{change}"
            );
            assert!(
                app.state.schematic.session.editor.pending_bus_tap.is_none(),
                "{change}"
            );
            assert_eq!(app.state.schematic.content_version(), content, "{change}");
            assert_eq!(app.state.schematic.topology_version(), topology, "{change}");
        }
    }

    #[test]
    fn rendered_escape_requires_two_actions_for_an_edited_draft() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_bus_tap(&mut app.state);
        app.state.dialogs.bus_tap.mark_edited();

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        assert!(app.state.dialogs.bus_tap.open);
        assert!(app.state.dialogs.bus_tap.discard_confirm);

        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        assert!(!app.state.dialogs.bus_tap.open);
    }
}
