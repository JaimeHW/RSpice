//! App-owned document authority and form lifetime for note placement.

use egui::Context;
use rspice_schematic_editor::annotations::note_placement;

use crate::state::{PendingDesignNotePlacement, Tool};
use crate::ui::widgets::{CommandForm, DialogChoice};

use crate::workbench::app::{RSpiceApp, schematic_editor_request_source};
use crate::workbench::app_state::AppState;

/// Isolated draft for the Place text or design note transaction.
///
/// The text starts empty. The sample the form used to open holding was
/// written into the field as a value, so an untouched form placed a note that
/// said it.
#[derive(Debug, Clone, Default)]
pub(crate) struct DesignNoteDialogState {
    pub(crate) open: bool,
    pub(crate) kind: crate::state::DesignNoteKind,
    pub(crate) text: String,
    pub(crate) source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
}

impl DesignNoteDialogState {
    pub(crate) fn open(&mut self, source: rspice_schematic_editor::requests::EditorRequestSource) {
        *self = Self {
            open: true,
            kind: crate::state::DesignNoteKind::PlainText,
            text: String::new(),
            source: Some(source),
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

const TITLE: &str = "Place text or design note";
const PRIMARY: &str = "Place";
const DESCRIPTION: &str = "Write the note, then click to place it.";

impl RSpiceApp {
    pub(in crate::workbench) fn render_design_note_dialog(&mut self, ctx: &Context) {
        if !self.state.design.design_note.open {
            return;
        }
        let validation = validate_draft(&self.state);
        // An untouched form has nothing to answer for: the primary is off and
        // the slot stays blank until the reader has typed something to refuse.
        let refusal = if self.state.design.design_note.text.trim().is_empty() {
            None
        } else {
            validation.as_ref().err().cloned()
        };
        let view_path = self
            .state
            .model
            .workspace
            .content
            .active_view
            .display_path();

        let draft = &mut self.state.design.design_note;
        let choice = CommandForm::new(TITLE, PRIMARY)
            .context(Some(view_path.as_str()))
            .describe(DESCRIPTION)
            .primary_enabled(validation.is_ok())
            // Enter types a line of the note; the primary places it.
            .primary_on_enter(false)
            .status(match refusal.as_deref() {
                Some(message) => Err(message),
                None => Ok(None),
            })
            .status_lines(2)
            .show(ctx, |rows| {
                note_placement::show(rows, &mut draft.kind, &mut draft.text)
            });

        match choice {
            DialogChoice::Primary => {
                if let Ok(pending) = validate_draft(&self.state) {
                    self.state
                        .model
                        .schematic
                        .session
                        .editor
                        .pending_design_note = Some(pending);
                    self.state.model.schematic.arm_tool(Tool::DesignNote);
                    self.state.design.design_note.close();
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.design.design_note.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> Result<PendingDesignNotePlacement, String> {
    let draft = &state.design.design_note;
    if state.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let Some(source) = draft.source.as_ref() else {
        return Err("Reopen Place text or note to capture the active schematic.".to_owned());
    };
    if source.design_epoch != state.model.design_execution_epoch {
        return Err("The design document changed. Close and reopen Place text or note.".to_owned());
    }
    if source.document_epoch != state.model.active_schematic_epoch {
        return Err(
            "The active schematic buffer changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if source.topology_version != state.model.schematic.topology_version() {
        return Err(
            "The schematic topology changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if source.document.display_path() != state.model.workspace.content.active_view.display_path() {
        return Err(
            "The active cell/view changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if *source != schematic_editor_request_source(state) {
        return Err(
            "The active schematic context changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    match PendingDesignNotePlacement::new(
        draft.kind,
        draft.text.clone(),
        source.topology_version,
        &state.model.schematic.document().design_notes,
    ) {
        Ok(pending) => Ok(pending.with_source(source.clone())),
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_input_reaches_design_note_text() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let mut app = RSpiceApp::test_instance();
        let source = schematic_editor_request_source(&app.state);
        app.state.design.design_note.open(source);
        // The form asks for the caret once its rows exist, so the frame that
        // builds them is the frame before the one that can be typed into.
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| app.render_design_note_dialog(ctx),
        );
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 800.0),
                )),
                events: vec![
                    egui::Event::Key {
                        key: egui::Key::A,
                        physical_key: Some(egui::Key::A),
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
                    },
                    egui::Event::Paste("Browser recovery qualification".to_owned()),
                ],
                ..Default::default()
            },
            |ctx| app.render_design_note_dialog(ctx),
        );
        assert_eq!(
            app.state.design.design_note.text,
            "Browser recovery qualification"
        );
        assert!(app.state.model.schematic.document().design_notes.is_empty());
    }

    /// The content bug this form shipped: the sample text was written into
    /// the field as a value, so an untouched form placed a note reading
    /// "Bias network".
    #[test]
    fn an_untouched_form_places_no_note() {
        let mut state = AppState::default();
        let source = schematic_editor_request_source(&state);
        state.design.design_note.open(source);
        assert_eq!(state.design.design_note.text, "");
        assert!(validate_draft(&state).is_err());
        assert!(state.model.schematic.document().design_notes.is_empty());
    }

    #[test]
    fn valid_draft_freezes_authority_without_mutating_document() {
        let mut state = AppState::default();
        assert!(validate_draft(&state).is_err());
        let source = schematic_editor_request_source(&state);
        state.design.design_note.open(source.clone());
        state.design.design_note.text = "Check the bias network".to_owned();
        let Ok(pending) = validate_draft(&state) else {
            panic!("valid draft");
        };
        assert_eq!(pending.text, "Check the bias network");
        assert_eq!(pending.source.as_ref(), Some(&source));
        assert_eq!(state.design.design_note.source.as_ref(), Some(&source));
        state.design.design_note.close();
        assert!(state.design.design_note.source.is_none());
        assert_eq!(pending.source, Some(source));
        assert!(state.model.schematic.document().design_notes.is_empty());
    }

    #[test]
    fn stale_and_read_only_drafts_fail_closed() {
        use crate::state::{
            CellViewRef, Point, SheetDefinition, SheetPortPolicy, SheetTemplate, ViewType,
        };
        for change in [
            "document",
            "occurrence",
            "sheet",
            "content",
            "read-only",
            "safe-mode",
        ] {
            let mut state = AppState::default();
            let master = CellViewRef::new("work", "note_child", "schematic");
            state.model.workspace.descend_into(
                "X1".to_owned(),
                master.clone(),
                ViewType::Schematic,
            );
            let first = state
                .model
                .workspace
                .content
                .design_management
                .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
                .unwrap();
            let source = schematic_editor_request_source(&state);
            state.design.design_note.open(source.clone());
            state.design.design_note.text = "Check the bias network".to_owned();
            assert!(validate_draft(&state).is_ok());
            match change {
                "document" => state.model.design_execution_epoch += 1,
                "occurrence" => {
                    state.model.workspace.ascend_one().unwrap();
                    state.model.workspace.descend_into(
                        "X2".to_owned(),
                        master,
                        ViewType::Schematic,
                    );
                }
                "sheet" => {
                    let catalog = state
                        .model
                        .workspace
                        .content
                        .design_management
                        .sheet_catalog_mut(&master.key())
                        .unwrap();
                    let second = catalog
                        .create_sheet(
                            SheetDefinition {
                                name: "Sheet 2".to_owned(),
                                template: SheetTemplate::AnalogSchematic,
                                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                                explicit_page_number: Some(2),
                            },
                            Some(first),
                        )
                        .unwrap();
                    catalog.set_active(second).unwrap();
                }
                "content" => {
                    let topology = state.model.schematic.topology_version();
                    let pending = PendingDesignNotePlacement::new(
                        crate::state::DesignNoteKind::PlainText,
                        "Another note",
                        topology,
                        &[],
                    )
                    .unwrap();
                    state
                        .model
                        .schematic
                        .place_pending_design_note(Point::origin(), pending)
                        .unwrap();
                    assert_eq!(state.model.schematic.topology_version(), topology);
                    assert_ne!(
                        state.model.schematic.content_version(),
                        source.content_version
                    );
                    state.model.schematic.init_undo_history();
                }
                "read-only" => state.model.schematic.session.read_only = true,
                "safe-mode" => state.model.safe_mode.activate(
                    crate::state::write_policy::LocalSafeModeOptions {
                        open_project_read_only: true,
                        ..Default::default()
                    },
                    "note dialog test".to_owned(),
                ),
                _ => unreachable!(),
            }
            let notes = state.model.schematic.document().design_notes.clone();
            assert!(validate_draft(&state).is_err(), "{change}");
            assert_eq!(state.design.design_note.source, Some(source));
            if !state.schematic_edit_read_only() {
                state
                    .design
                    .design_note
                    .open(schematic_editor_request_source(&state));
                state.design.design_note.text = "Check the bias network".to_owned();
                assert!(validate_draft(&state).is_ok(), "fresh {change}");
            }
            assert_eq!(state.model.schematic.document().design_notes, notes);
            assert!(!state.model.schematic.can_undo());
        }
    }
}
