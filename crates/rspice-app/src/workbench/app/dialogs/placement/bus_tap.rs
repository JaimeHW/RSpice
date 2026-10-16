//! App-owned form, source validation and arming for bus-tap placement.

use crate::state::Tool;
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use egui::Context;
use rspice_schematic_editor::{
    bus_tap_placement::{self, DraftValidation},
    requests::EditorRequestSource,
    session::bus::PendingBusTapPlacement,
};

const TITLE: &str = "Place bus tap";
const PRIMARY: &str = "Place";
const DESCRIPTION: &str = "Connect one bit or a range of a bus.";

impl RSpiceApp {
    pub(in crate::workbench) fn render_bus_tap_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.bus_tap.open {
            return;
        }

        let validation = match placement_source(&self.state) {
            Ok(_) => self.state.dialogs.bus_tap.fields.validate(),
            Err(message) => DraftValidation::Invalid(message),
        };
        // The sheet this tap goes on: the one thing about the form that is not
        // in the form.
        let view_path = self.state.workspace.content.active_view.display_path();

        let fields = &mut self.state.dialogs.bus_tap.fields;
        let choice = CommandForm::new(TITLE, PRIMARY)
            .context(Some(view_path.as_str()))
            .describe(DESCRIPTION)
            .primary_enabled(validation.can_commit())
            .status(validation.status())
            .status_lines(2)
            .show(ctx, |rows| {
                bus_tap_placement::show(rows, &validation, fields)
            });

        match choice {
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
                self.state.dialogs.bus_tap.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

pub(crate) fn open_bus_tap(state: &mut AppState) {
    let source = crate::workbench::app::schematic_editor_request_source(state);
    let bus = sole_declared_bus(state);
    state.dialogs.bus_tap.open(source, bus);
}

/// The bus a tap starts from when the sheet leaves no doubt: the one
/// declaration it draws.
///
/// A sample value written into the field — this form opened holding
/// `DATA[15:0]` — is a tap on a bus the design may not have, placed by a
/// reader who pressed Enter. One declared bus is not a sample; it is the only
/// answer. Anything else starts empty, with the shape of a declaration as the
/// field's hint.
fn sole_declared_bus(state: &AppState) -> String {
    let mut declarations = state
        .schematic
        .document()
        .buses
        .iter()
        .filter_map(|bus| bus.declaration.as_ref().map(ToString::to_string))
        .collect::<Vec<_>>();
    declarations.sort();
    declarations.dedup();
    match declarations.as_slice() {
        [only] => only.clone(),
        _ => String::new(),
    }
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
    use crate::state::{Bus, BusDeclaration, BusTapOrientation, Point};
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

    /// Draw one declared bus on the active sheet.
    fn declare_bus(state: &mut AppState, declaration: &str) {
        let id = state.schematic.next_id();
        let declaration = BusDeclaration::parse(declaration).expect("declaration");
        state.schematic.document_mut_for_test().buses.push(
            Bus::segment(id, Point::new(0, 0), Point::new(40, 0), Some(declaration))
                .expect("bus segment"),
        );
    }

    /// Type a complete tap into the open form.
    fn fill(state: &mut AppState, bus: &str, slice: &str) {
        state.dialogs.bus_tap.fields.bus = bus.to_owned();
        state.dialogs.bus_tap.fields.slice = slice.to_owned();
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
        assert!(app.state.schematic.session.editor.pending_bus_tap.is_none());
    }

    /// The content bug this form shipped: it opened holding `DATA[15:0]` and
    /// `DATA[7:0]` as values, so Enter on an untouched form placed a tap on a
    /// bus the design had never declared.
    #[test]
    fn an_untouched_form_taps_nothing_and_prefills_only_the_one_declared_bus() {
        let mut app = RSpiceApp::test_instance();
        open_bus_tap(&mut app.state);
        assert_eq!(app.state.dialogs.bus_tap.fields.bus, "");
        assert_eq!(app.state.dialogs.bus_tap.fields.slice, "");
        assert!(!app.state.dialogs.bus_tap.fields.validate().can_commit());

        // One declared bus is the only answer, so the field starts there. The
        // slice is still the reader's, so the form still cannot commit.
        app.state.dialogs.bus_tap.close();
        declare_bus(&mut app.state, "DATA[15:0]");
        open_bus_tap(&mut app.state);
        assert_eq!(app.state.dialogs.bus_tap.fields.bus, "DATA[15:0]");
        assert_eq!(app.state.dialogs.bus_tap.fields.slice, "");
        assert!(!app.state.dialogs.bus_tap.fields.validate().can_commit());

        // Two declarations are a choice, and a form does not choose for the
        // reader.
        app.state.dialogs.bus_tap.close();
        declare_bus(&mut app.state, "ADDR[7:0]");
        open_bus_tap(&mut app.state);
        assert_eq!(app.state.dialogs.bus_tap.fields.bus, "");
    }

    #[test]
    fn opening_captures_the_source_and_closing_releases_it() {
        let mut draft = BusTapDialogState::default();
        let source = test_source();
        draft.open(source.clone(), String::new());

        assert!(draft.open);
        assert_eq!(draft.source.as_ref(), Some(&source));
        assert_eq!(draft.fields.orientation, BusTapOrientation::Automatic);
        draft.close();
        assert!(draft.source.is_none());
        let mut next = source;
        next.document_epoch += 1;
        draft.open(next.clone(), String::new());
        assert_eq!(draft.source.as_ref(), Some(&next));
    }

    /// Escape leaves at once. The draft owns nothing, so there is nothing to
    /// confirm the discard of.
    #[test]
    fn escape_closes_the_form_at_once_and_places_nothing() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_bus_tap(&mut app.state);
        fill(&mut app.state, "DATA[15:0]", "DATA[7:0]");

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_bus_tap_dialog(ctx)
        });

        assert!(!app.state.dialogs.bus_tap.open);
        assert!(app.state.schematic.session.editor.pending_bus_tap.is_none());
        assert_eq!(app.state.schematic.session.editor.tool, Tool::Select);
    }

    #[test]
    fn rendered_primary_revalidates_publishes_and_arms_the_bus_tap_tool() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_bus_tap(&mut app.state);
        fill(&mut app.state, "DATA[15:0]", "DATA[7:0]");

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
            fill(&mut app.state, "DATA[15:0]", "DATA[7:0]");
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
}
