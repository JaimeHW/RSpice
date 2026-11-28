//! The commit rule every Simulate form field follows.
//!
//! A text field holds what is typed while it has the keyboard and commits
//! when it lets go of it: Enter, Tab, or a press anywhere else. A select,
//! switch or tick commits at once. Run, Check, and a switch of page, test or
//! workspace first ask the open field to let go, and wait for its form to
//! draw that. Text that does not read is never committed: it stays in its
//! field, outlined with the reason, whatever asked for the commit does not
//! start, and the field takes the keyboard back.

use egui::{Id, Response, Stroke, Ui};

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisDraft, NumericOverrideOption};
use crate::ui::tokens::Tokens;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::slots::SimulateState;
use crate::workbench::state::{DraftEdit, OpenDraft, OpenField};

use super::advanced_options::{self, OptionEdit};

/// Where a form keeps, for the frame it draws in, the field that has the
/// keyboard.
fn mark_slot() -> Id {
    Id::new("rspice.simulate.open-field")
}

/// Where a form tells its open field, for the frame it draws in, what to do.
fn cue_slot() -> Id {
    Id::new("rspice.simulate.field-cue")
}

/// What the open field does in the frame its form draws.
#[derive(Clone, Default)]
struct FieldCue {
    id: Option<Id>,
    release: bool,
    refocus: bool,
    refusal: Option<String>,
}

/// A text field drawn by `draw`, under the commit rule. `label` is what its
/// row states, and what a refusal names it by.
pub(crate) fn text_field(
    ui: &mut Ui,
    label: &str,
    draw: impl FnOnce(&mut Ui) -> Response,
) -> Response {
    let mut response = draw(ui);
    let cue = ui
        .data(|data| data.get_temp::<FieldCue>(cue_slot()))
        .filter(|cue| cue.id == Some(response.id));
    if let Some(cue) = cue {
        if cue.refocus {
            response.request_focus();
        }
        if cue.release {
            // After the field drew, so keystrokes of this frame land in it
            // before it lets go, and its row reads the let-go as Tab's.
            response.surrender_focus();
        }
        if let Some(reason) = cue.refusal {
            let t = Tokens::get(ui.ctx());
            ui.painter().rect_stroke(
                response.rect,
                t.radius,
                Stroke::new(1.0, t.color.err),
                egui::StrokeKind::Inside,
            );
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_description(reason.clone());
                node.set_invalid(egui::accesskit::Invalid::True);
            });
            response = response.on_hover_text(reason);
        }
    }
    if response.has_focus() {
        let open = OpenField {
            id: response.id,
            label: label.to_owned(),
        };
        ui.data_mut(|data| data.insert_temp(mark_slot(), open));
    }
    response
}

/// Before a form draws: tell `open`'s field what to do this frame, and
/// forget which field had the keyboard last frame.
pub(super) fn begin_form(ui: &Ui, open: Option<&mut OpenDraft>) {
    let cue = open.map_or_else(FieldCue::default, |open| FieldCue {
        id: Some(open.field.id),
        release: std::mem::take(&mut open.release),
        refocus: std::mem::take(&mut open.refocus),
        refusal: open.refusal.clone(),
    });
    ui.data_mut(|data| {
        data.insert_temp(cue_slot(), cue);
        data.remove::<OpenField>(mark_slot());
    });
}

/// After a form drew: the field that has the keyboard, if one does.
pub(super) fn end_form(ui: &Ui) -> Option<OpenField> {
    ui.data_mut(|data| {
        data.remove::<FieldCue>(cue_slot());
        let open = data.get_temp::<OpenField>(mark_slot());
        data.remove::<OpenField>(mark_slot());
        open
    })
}

/// Ask the open field to let go before a command that commits it runs.
/// True when the command has to wait: for the field's form to draw the
/// let-go, and then once for the next frame, since a frame that edits the
/// plan drops every check started in it.
pub fn release_open_field(state: &mut SimulateState) -> bool {
    if std::mem::take(&mut state.form_committed) {
        return true;
    }
    match &mut state.open_draft {
        Some(open) if open.refusal.is_none() && !open.release => {
            open.release = true;
            true
        }
        _ => false,
    }
}

/// Commit whatever field is open, before something that reads what it
/// holds starts. A field whose text does not read keeps it, takes the
/// keyboard back and is named in the error, which the status line states.
pub fn commit_open_drafts(cx: &mut Cx<'_, SimulateFeature>) -> Result<(), String> {
    let Some(open) = cx.state.open_draft.take() else {
        return Ok(());
    };
    if committed_draft(cx.model, open.edit.instance()).is_none() {
        // Its analysis is gone, and with it the field: nothing is left to
        // commit it into.
        return Ok(());
    }
    if let Some(reason) = open.refusal.clone() {
        return refuse(cx, open, reason);
    }
    let outcome = match &open.edit {
        DraftEdit::Analysis { instance, draft } => {
            commit_analysis(cx, *instance, draft, &open.field.label)
        }
        DraftEdit::AnalysisOption {
            instance,
            option,
            text,
        } => commit_option(cx, *instance, *option, text),
    };
    match outcome {
        Ok(()) => Ok(()),
        Err(reason) => {
            let reason = field_reason(&open.field.label, &reason);
            refuse(cx, open, reason)
        }
    }
}

/// Keep `open` with `reason`, give it the keyboard back and say why.
fn refuse(
    cx: &mut Cx<'_, SimulateFeature>,
    mut open: OpenDraft,
    reason: String,
) -> Result<(), String> {
    open.refusal = Some(reason.clone());
    open.refocus = true;
    open.release = false;
    cx.state.open_draft = Some(open);
    cx.wb.prompt(reason.clone());
    Err(reason)
}

/// `reason` as the refusal of the field labelled `label`: the field's name,
/// then what is wrong, once.
pub(super) fn field_reason(label: &str, reason: &str) -> String {
    let detail = reason
        .split_once(':')
        .filter(|(named, _)| named.trim().eq_ignore_ascii_case(label))
        .map_or(reason, |(_, detail)| detail)
        .trim();
    format!("{label}: {detail}")
}

/// Commit an analysis's fields as its form holds them, after the field
/// labelled `field` was typed in. Refused, with the reason, when they hold a
/// problem the committed fields do not.
pub(super) fn commit_analysis(
    cx: &mut Cx<'_, SimulateFeature>,
    instance: AnalysisInstanceId,
    working: &AnalysisDraft,
    field: &str,
) -> Result<(), String> {
    let Some(committed) = committed_draft(cx.model, instance) else {
        return Ok(());
    };
    if same_draft(working, &committed) {
        return Ok(());
    }
    if let Some(problem) = new_problem(cx.model, working, &committed) {
        return Err(problem);
    }
    super::commit_draft(cx, instance, working.clone(), Some(field));
    Ok(())
}

/// The fields of `instance` as the plan holds them.
pub(super) fn committed_draft(
    model: &Model,
    instance: AnalysisInstanceId,
) -> Option<AnalysisDraft> {
    model
        .sim_setup
        .stable_analysis_plan()
        .ok()?
        .instance(instance)
        .map(|target| target.draft().clone())
}

/// Whether two drafts hold the same values.
pub(super) fn same_draft(left: &AnalysisDraft, right: &AnalysisDraft) -> bool {
    match (serde_json::to_vec(left), serde_json::to_vec(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

/// The problem `working` has that `committed` does not: what an edit to one
/// field brought, since every other field of it is committed already.
pub(super) fn new_problem(
    model: &Model,
    working: &AnalysisDraft,
    committed: &AnalysisDraft,
) -> Option<String> {
    let reads_sources =
        |draft: &AnalysisDraft| matches!(draft, AnalysisDraft::Envelope(_) | AnalysisDraft::Pss(_));
    let sources = (reads_sources(working) || reads_sources(committed))
        .then(|| super::build_envelope_source_catalog(model));
    let problem = |draft: &AnalysisDraft| {
        super::analysis_validation_error(
            model,
            draft,
            sources.as_ref().filter(|_| reads_sources(draft)),
        )
    };
    let after = problem(working)?;
    (problem(committed).as_deref() != Some(after.as_str())).then_some(after)
}

/// Commit one option field of `instance` holding `text`. Refused, with the
/// reason, when the option does not take it.
pub(super) fn commit_option(
    cx: &mut Cx<'_, SimulateFeature>,
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
    text: &str,
) -> Result<(), String> {
    let Some(draft) = committed_draft(cx.model, instance) else {
        return Ok(());
    };
    let edit = super::analysis_form::options::offered(cx.model, &draft, instance)
        .iter()
        .flat_map(|section| &section.rows)
        .find(|row| row.option == option)
        .and_then(|row| advanced_options::well_edit(row, text));
    match edit {
        Some(edit) => commit_option_edit(cx, instance, &edit),
        None => Ok(()),
    }
}

/// Write one option edit to the plan: a value, or the option returned to
/// the plan's own. The writer states a refusal itself; the reason comes back
/// so the field can keep its text and say it.
fn commit_option_edit(
    cx: &mut Cx<'_, SimulateFeature>,
    instance: AnalysisInstanceId,
    edit: &OptionEdit,
) -> Result<(), String> {
    match edit {
        OptionEdit::Set(option, authored) => {
            super::page_solver::write_numeric_record(cx, instance, *option, authored.trim())
        }
        OptionEdit::Clear(option) => {
            let cleared = cx
                .model
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| plan.instance(instance))
                .and_then(|target| target.numeric_override().cloned())
                .map(|mut record| {
                    record.clear(*option);
                    record
                });
            match cleared {
                Some(record) => {
                    super::lifecycle::commit_numeric_override(cx, instance, Some(record))
                }
                None => Ok(()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::field_reason;

    #[test]
    fn a_refusal_names_its_field_once() {
        assert_eq!(
            field_reason("Stop time", "stop time: unsupported SPICE suffix 'q'"),
            "Stop time: unsupported SPICE suffix 'q'"
        );
        assert_eq!(
            field_reason("Reltol", "value is empty"),
            "Reltol: value is empty"
        );
    }
}
