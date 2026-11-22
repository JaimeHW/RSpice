//! App-owned document authority and form lifetime for note placement.

use egui::Context;
use rspice_schematic_editor::annotations::note_placement;

use crate::state::{PendingDesignNotePlacement, Tool, entered_text};
use crate::ui::widgets::{CommandForm, DialogChoice};

use crate::workbench::app::schematic_editor_request_source;
use crate::workbench::app_state::Model;

/// Isolated draft for the Place text or design note transaction.
///
/// The text starts empty. The sample the form used to open holding was
/// written into the field as a value, so an untouched form placed a note that
/// said it.
#[derive(Debug, Clone, Default)]
pub struct DesignNoteDialogState {
    pub open: bool,
    pub(crate) kind: crate::state::DesignNoteKind,
    pub text: String,
    pub(crate) source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
    /// The note's size, in percent of the standard size.
    pub text_size_percent: u16,
    pub align: rspice_design::schematic::design_note::SymbolTextAlign,
    /// What "Add to the plan" last said, and the text it left: the receipt,
    /// or why the plan would not take the card. Typing drops it.
    pub(crate) plan_outcome: Option<(Result<String, String>, String)>,
}

impl DesignNoteDialogState {
    pub fn open(&mut self, source: rspice_schematic_editor::requests::EditorRequestSource) {
        *self = Self {
            open: true,
            kind: crate::state::DesignNoteKind::PlainText,
            text: String::new(),
            source: Some(source),
            text_size_percent: 100,
            align: Default::default(),
            plan_outcome: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

const TITLE: &str = "Place text or design note";
const PRIMARY: &str = "Place";
const ADD_TO_PLAN: &str = "Add to the plan";
const DESCRIPTION: &str = "Write the note, then click to place it.";

pub(in crate::workbench) fn render_design_note_dialog(
    dialog: &mut DesignNoteDialogState,
    model: &mut Model,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }
    recapture(dialog, model);
    if dialog
        .plan_outcome
        .as_ref()
        .is_some_and(|(_, text)| *text != dialog.text)
    {
        dialog.plan_outcome = None;
    }
    let validation = validate_draft(dialog, model);
    let to_plan = card_for_the_plan(dialog);
    let status = design_note_status(&dialog.text, &validation, dialog.plan_outcome.as_ref());
    let draft = &mut *dialog;
    let mut form = CommandForm::new(TITLE, PRIMARY)
        .describe(DESCRIPTION)
        .primary_enabled(validation.is_ok())
        // Enter types a line of the note; the primary places it.
        .primary_on_enter(false)
        .status(match &status {
            Ok(fact) => Ok(fact.as_deref()),
            Err(refusal) => Err(refusal.as_str()),
        });
    if to_plan.is_some() {
        form = form.secondary(ADD_TO_PLAN);
    }
    let choice = form.show(ctx, |rows| {
        note_placement::show(
            rows,
            &mut draft.kind,
            &mut draft.text,
            &mut draft.text_size_percent,
            &mut draft.align,
        )
    });

    match choice {
        DialogChoice::Primary => {
            if let Ok(pending) = validate_draft(dialog, model) {
                model.schematic.session.editor.pending_design_note = Some(pending);
                model.schematic.arm_tool(Tool::DesignNote);
                dialog.close();
            }
        }
        DialogChoice::Secondary => {
            if let Some(refused) = to_plan {
                add_to_the_plan(dialog, model, &refused);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            dialog.close();
        }
        DialogChoice::None => {}
    }
}

/// The analysis card a SPICE directive note's text holds that "Add to the
/// plan" can write into the plan: the first line the note refuses, when the
/// plan has a draft for it.
fn card_for_the_plan(
    dialog: &DesignNoteDialogState,
) -> Option<rspice_design::schematic::spice_directive::RefusedDirectiveLine> {
    (dialog.kind == crate::state::DesignNoteKind::SpiceDirective)
        .then(|| rspice_design::schematic::spice_directive::refused_directive_lines(&dialog.text))?
        .into_iter()
        .next()
        .filter(rspice_design::schematic::spice_directive::RefusedDirectiveLine::adds_to_plan)
}

/// Write `refused` into the active plan and take its line off the note, so
/// what is left is what the netlist takes.
pub(crate) fn add_to_the_plan(
    dialog: &mut DesignNoteDialogState,
    model: &mut Model,
    refused: &rspice_design::schematic::spice_directive::RefusedDirectiveLine,
) {
    let outcome = model.add_analysis_card_to_plan(&refused.card);
    if outcome.is_ok() {
        dialog.text =
            rspice_design::schematic::spice_directive::without_line(&dialog.text, refused.line);
    }
    dialog.plan_outcome = Some((outcome, dialog.text.clone()));
}

/// A form left open while the drawing changes, another sheet is opened, or
/// another instance of the cell is entered places on what is open now: the
/// kind and the text belong to no document, so the form reads the active
/// schematic again rather than asking to be closed and reopened.
fn recapture(dialog: &mut DesignNoteDialogState, model: &Model) {
    if model.schematic_edit_read_only() {
        return;
    }
    let current = schematic_editor_request_source(model);
    if dialog.source.as_ref() != Some(&current) {
        dialog.source = Some(current);
    }
}

/// What the footer says. An untouched form has refused nothing, so it says
/// what Place waits for as a plain fact; once there is text, a refusal is
/// stated as one.
fn design_note_status(
    text: &str,
    validation: &Result<PendingDesignNotePlacement, String>,
    plan_outcome: Option<&(Result<String, String>, String)>,
) -> Result<Option<String>, String> {
    // What the plan said answers the press that asked it, until the text
    // changes again.
    match plan_outcome {
        Some((Err(refusal), _)) => return Err(refusal.clone()),
        Some((Ok(receipt), _)) if validation.is_ok() || text.trim().is_empty() => {
            return Ok(Some(receipt.clone()));
        }
        _ => {}
    }
    if text.trim().is_empty() {
        return Ok(Some("Type the text to place.".to_owned()));
    }
    match validation {
        Ok(_) => Ok(None),
        Err(refusal) => Err(refusal.clone()),
    }
}

fn validate_draft(
    draft: &DesignNoteDialogState,
    model: &Model,
) -> Result<PendingDesignNotePlacement, String> {
    if model.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    // The form reads the open schematic again each frame, so a draft that
    // holds another one has not been drawn since the drawing changed.
    let current = schematic_editor_request_source(model);
    let Some(source) = draft.source.as_ref().filter(|source| **source == current) else {
        return Err("The open schematic changed.".to_owned());
    };
    match PendingDesignNotePlacement::new(
        draft.kind,
        entered_text(&draft.text),
        source.topology_version,
        &model.schematic.document().design_notes,
    ) {
        Ok(pending) => Ok(pending
            .with_source(source.clone())
            .with_lettering(draft.text_size_percent, draft.align)),
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod tests {
    use super::*;
    #[cfg(test)]
    use crate::workbench::design::test_host::DesignTestHost;

    /// Draw the dialog as its dialog step does.
    pub fn render(dialog: &mut DesignNoteDialogState, model: &mut Model, ctx: &Context) {
        render_design_note_dialog(dialog, model, ctx);
    }

    /// The draft's validation, over what the form reads.
    #[cfg(test)]
    fn validate_draft(state: &DesignTestHost) -> Result<PendingDesignNotePlacement, String> {
        super::validate_draft(&state.design.design_note, &state.model)
    }

    /// The content bug this form shipped: the sample text was written into
    /// the field as a value, so an untouched form placed a note reading
    /// "Bias network".
    #[test]
    fn an_untouched_form_places_no_note() {
        let mut state = DesignTestHost::default();
        let source = schematic_editor_request_source(&state);
        state.design.design_note.open(source);
        assert_eq!(state.design.design_note.text, "");
        let validation = validate_draft(&state);
        assert!(validation.is_err());
        // It refused nothing, but it says why Place is off.
        assert_eq!(
            design_note_status("", &validation, None),
            Ok(Some("Type the text to place.".to_owned()))
        );
        assert!(state.model.schematic.document().design_notes.is_empty());
    }

    /// A `.tran` typed on a SPICE directive note is refused on its line, and
    /// the form's "Add to the plan" writes it into the plan as the Analyses
    /// page would, leaving the note the cards the netlist takes.
    #[test]
    fn an_analysis_typed_on_a_directive_note_goes_to_the_plan_from_the_form() {
        use crate::simulation::plan::AnalysisKind;

        let mut state = DesignTestHost::default();
        let source = schematic_editor_request_source(&state);
        state.design.design_note.open(source);
        state.design.design_note.kind = crate::state::DesignNoteKind::SpiceDirective;
        state.design.design_note.text = ".param r=1k\n.tran 1n 1u".to_owned();
        assert_eq!(
            validate_draft(&state).unwrap_err(),
            "Line 2: .tran is an analysis, and the plan owns analyses."
        );
        let transients = |state: &DesignTestHost| {
            state
                .model
                .sim_setup
                .stable_analysis_plan()
                .expect("a plan")
                .instances()
                .iter()
                .filter(|instance| instance.kind() == AnalysisKind::Transient)
                .map(|instance| match instance.draft() {
                    crate::simulation::plan::AnalysisDraft::Transient(tran) => {
                        (tran.step.clone(), tran.stop.clone())
                    }
                    _ => unreachable!("a transient's draft"),
                })
                .collect::<Vec<_>>()
        };
        let before = transients(&state);

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let frame = |state: &mut DesignTestHost, events| {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(900.0, 700.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| render(&mut state.design.design_note, &mut state.model, ctx),
            )
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default()
        };
        let mut nodes = Vec::new();
        for _ in 0..3 {
            nodes = frame(&mut state, Vec::new());
        }
        let add = nodes
            .iter()
            .find(|(_, node)| node.label() == Some(ADD_TO_PLAN))
            .map(|(id, _)| *id)
            .expect("the form offers Add to the plan");
        let _ = frame(
            &mut state,
            vec![egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    action: egui::accesskit::Action::Click,
                    target_tree: egui::accesskit::TreeId::ROOT,
                    target_node: add,
                    data: None,
                },
            )],
        );

        let mut after = transients(&state);
        assert_eq!(after.len(), before.len() + 1, "{after:?}");
        after.retain(|transient| !before.contains(transient));
        assert_eq!(after, [("1n".to_owned(), "1u".to_owned())]);
        let note = &state.design.design_note;
        assert!(note.open, "the rest of the note is still to place");
        assert_eq!(note.text, ".param r=1k");
        let validation = validate_draft(&state);
        assert!(validation.is_ok(), "{validation:?}");
        let status = design_note_status(
            &state.design.design_note.text,
            &validation,
            state.design.design_note.plan_outcome.as_ref(),
        );
        let receipt = status.expect("a receipt").expect("said on the form");
        assert!(
            receipt.starts_with("Added .tran 1n 1u to the plan "),
            "{receipt}"
        );
        let nodes = frame(&mut state, Vec::new());
        assert!(
            !nodes
                .iter()
                .any(|(_, node)| node.label() == Some(ADD_TO_PLAN)),
            "nothing left for the plan"
        );
    }

    /// A note pasted decomposed is placed composed: the "é" it reads is the
    /// one character a typed one is.
    #[test]
    fn a_decomposed_note_is_placed_composed() {
        let mut state = DesignTestHost::default();
        let source = schematic_editor_request_source(&state);
        state.design.design_note.open(source);
        state.design.design_note.text = " Cafe\u{301} bias ".to_owned();
        let Ok(pending) = validate_draft(&state) else {
            panic!("valid draft");
        };
        assert_eq!(pending.text, "Caf\u{e9} bias");
    }

    #[test]
    fn valid_draft_freezes_authority_without_mutating_document() {
        let mut state = DesignTestHost::default();
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

    /// The size and alignment chosen on the form are the placed note's; a
    /// later change of them is one undo step.
    #[test]
    fn a_note_keeps_the_size_and_alignment_it_was_placed_with() {
        use crate::state::Point;
        use rspice_design::schematic::design_note::SymbolTextAlign;
        let mut state = DesignTestHost::default();
        state
            .design
            .design_note
            .open(schematic_editor_request_source(&state));
        state.design.design_note.text = "Power stage".to_owned();
        state.design.design_note.text_size_percent = 200;
        state.design.design_note.align = SymbolTextAlign::Right;
        let pending = validate_draft(&state).expect("valid draft");
        let id = state
            .model
            .schematic
            .place_pending_design_note(Point::new(40, 40), pending)
            .unwrap();
        let placed = state.model.schematic.document().design_notes[0].clone();
        assert_eq!(placed.id, id);
        assert_eq!(placed.text_size_percent, 200);
        assert_eq!(placed.alignment(), SymbolTextAlign::Right);

        state.model.schematic.init_undo_history();
        assert!(
            state
                .model
                .schematic
                .edit_design_note_properties(
                    placed.clone(),
                    placed.kind,
                    placed.text.clone(),
                    None,
                    (75, SymbolTextAlign::Center),
                )
                .unwrap()
        );
        let edited = &state.model.schematic.document().design_notes[0];
        assert_eq!(
            (edited.text_size_percent, edited.alignment()),
            (75, SymbolTextAlign::Center)
        );
        assert!(state.model.schematic.undo());
        assert_eq!(state.model.schematic.document().design_notes[0], placed);
        assert!(!state.model.schematic.can_undo());
    }

    /// A form left open while the drawing changes under it, or while another
    /// instance of the cell is entered, reads the open schematic again on its
    /// next frame and places there: it never asks to be closed and reopened.
    #[test]
    fn an_open_form_reads_a_changed_schematic_again_on_the_next_frame() {
        use crate::state::{CellViewRef, Point, ViewType};
        for change in ["content", "occurrence"] {
            let mut state = DesignTestHost::default();
            let master = CellViewRef::new("work", "note_child", "schematic");
            state.model.workspace.descend_into(
                "X1".to_owned(),
                master.clone(),
                ViewType::Schematic,
            );
            state
                .design
                .design_note
                .open(schematic_editor_request_source(&state));
            state.design.design_note.text = "Check the bias network".to_owned();
            match change {
                "content" => {
                    let pending = PendingDesignNotePlacement::new(
                        crate::state::DesignNoteKind::PlainText,
                        "Another note",
                        state.model.schematic.topology_version(),
                        &[],
                    )
                    .unwrap();
                    state
                        .model
                        .schematic
                        .place_pending_design_note(Point::origin(), pending)
                        .unwrap();
                }
                _ => {
                    state.model.workspace.ascend_one().unwrap();
                    state.model.workspace.descend_into(
                        "X2".to_owned(),
                        master,
                        ViewType::Schematic,
                    );
                }
            }
            assert!(validate_draft(&state).is_err(), "{change}");

            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
                render(&mut state.design.design_note, &mut state.model, ui.ctx());
            });

            assert!(state.design.design_note.open, "{change}");
            assert_eq!(state.design.design_note.text, "Check the bias network");
            let current = schematic_editor_request_source(&state);
            assert_eq!(state.design.design_note.source.as_ref(), Some(&current));
            let pending = validate_draft(&state).expect("the form places on the open schematic");
            assert_eq!(pending.source, Some(current), "{change}");
        }
    }

    #[test]
    fn stale_and_read_only_drafts_fail_closed() {
        use crate::state::{CellViewRef, Point, SheetDefinition, SheetPortPolicy, ViewType};
        for change in [
            "document",
            "occurrence",
            "sheet",
            "content",
            "read-only",
            "safe-mode",
        ] {
            let mut state = DesignTestHost::default();
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
            if !state.model.schematic_edit_read_only() {
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
