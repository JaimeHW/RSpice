//! The Stimulus Library instrument.
//!
//! One fused instrument rather than five panels: a definition is a single
//! object, and the five bands are five things to know about that one object —
//! what it is, what it does, what it is made of, what it emits, and what the
//! engine will say about it. They share a frame and are separated by a rule,
//! because a card per band would put four scroll bars and four margins between
//! facts that have to be read together.
//!
//! Nothing here interprets a waveform, a parameter name or a lifecycle word.
//! The curve and its readouts come from `simulation::stimulus_realize`, which
//! is the engine's own evaluator and resolvers; the parameter rows come from
//! the `PropertySheet` the registry already holds for the component type; the
//! findings come from `state::source_contract`; the card comes from the
//! netlister; the provenance chips come from `ProvenanceState`. This module
//! owns exactly one thing: where those facts land on screen, and which
//! keystroke changes which of them.
//!
//! The instrument never scrolls as a whole. It is laid out into the stage it
//! is given, and the two bands whose content is unbounded — the parameter
//! program and the realization list — scroll inside their own band. A stage
//! that scrolled would put the audit strip below the fold, which is the one
//! place a reader must never have to go looking for.

mod audit;
mod identity;
mod program;
mod proof;
mod pwl;
mod realization;

#[cfg(test)]
mod tests;

use egui::{Rect, Sense, Ui, UiBuilder, Vec2};

use crate::simulation::stimulus_realize::{PreviewTiming, StimulusRealization};
use crate::state::ContractStrength;
use crate::state::stimulus_library::definition::{
    StimulusDefinition, StimulusFamily, StimulusKind,
};
use crate::ui::tokens::Tokens;
use crate::workbench::app::actions::stimulus::StimulusAdopter;
use crate::workbench::state::PreviewSpan;
use crate::workbench::{AppState, MessageId};

use super::super::design_system::{WorkbenchIcon, empty_state_with_actions, labeled_icon_button};

/// One thing the reader asked the instrument to do.
///
/// Collected while painting and applied afterwards, because a band that
/// mutated the library mid-frame would leave the bands after it describing a
/// definition the ones before it did not.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum StageAction {
    /// Rename the working record.
    Rename(String),
    /// Switch the waveform shape, which resets the shape parameters.
    SetFamily(StimulusFamily),
    /// Switch the driven quantity, which keeps the card.
    SetKind(StimulusKind),
    /// Keep the text a reader is typing, without making it an edit yet.
    TypeField {
        /// The property name the sheet uses.
        field: String,
        /// The text as it currently stands.
        value: String,
    },
    /// Write one waveform field, in the instance's own spelling.
    CommitField {
        /// The property name the sheet uses.
        field: String,
        /// Authored text, empty meaning "leave it to the engine".
        value: String,
    },
    /// Replace the authored PWL table.
    SetPoints(String),
    /// Select one PWL row, from the table or from its marker.
    SelectPoint(Option<usize>),
    /// Look through a different span.
    SetSpan(PreviewSpan),
    /// Publish the draft as the next revision.
    Apply,
    /// Discard the draft, as one undoable step.
    Revert,
    /// Copy the definition into an editable copy.
    Duplicate,
    /// Remove the definition. Adopters keep their cards.
    Delete,
    /// Import a data file and retain its bytes in the definition.
    ImportDataFile,
    /// Copy text to the clipboard.
    Copy(String),
    /// Arm the placement cursor with the saved revision.
    Place,
    /// Re-adopt the current revision onto one placed source.
    Readopt(u64),
    /// Open Component Properties for one placed source.
    OpenProperties(u64),
}

/// One line of the audit strip.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Finding {
    /// Whether it blocks Apply.
    pub blocking: bool,
    /// What it says.
    pub message: String,
}

/// Everything the five bands read, resolved once per frame.
pub(super) struct Stage {
    /// The definition as the library holds it.
    pub saved: StimulusDefinition,
    /// The definition as it is being edited, including the field in progress.
    pub working: StimulusDefinition,
    /// Whether the working record says anything the saved one does not.
    pub dirty: bool,
    /// Whether a step back is available.
    pub can_undo: bool,
    /// The engine's reading of the working record over the chosen span.
    pub realization: StimulusRealization,
    /// The transient the preview resolved omitted fields against.
    pub timing: PreviewTiming,
    /// Which span the surface is looking through.
    pub span: PreviewSpan,
    /// Which PWL row is selected.
    pub selected_point: Option<usize>,
    /// The definition's own card, or the generator's refusals.
    pub card: Result<String, Vec<String>>,
    /// Every placed source in the design that adopted this definition.
    pub adopters: Vec<StimulusAdopter>,
    /// Everything the audit strip states, errors first.
    pub findings: Vec<Finding>,
    /// The field the reader is typing into, if any.
    pub editing_field: Option<String>,
    /// Whether this frame should put the caret in the name field.
    pub focus_name: bool,
}

impl Stage {
    /// How many findings block Apply.
    pub fn errors(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.blocking)
            .count()
    }

    /// How many findings state something the card's labels do not.
    pub fn advisories(&self) -> usize {
        self.findings.len() - self.errors()
    }

    /// Why Apply is unavailable, or `None` when it is available.
    pub fn apply_block(&self) -> Option<String> {
        if let Some(error) = self.findings.iter().find(|finding| finding.blocking) {
            return Some(format!("Blocked · {}", error.message));
        }
        (!self.dirty).then(|| "No draft to apply".to_owned())
    }
}

/// Band heights, in the order the instrument stacks them.
///
/// Every band but the proof surface is as tall as what it holds, and says so
/// before it is painted (`program::content_height` and its two siblings): an
/// identity row, a grid of fields, a card list and a verdict strip do not
/// become more legible with more room, and they become unusable with less — a
/// fixed slot for the program showed three and a half of a pulse's seven
/// fields and one of a PWL's points whatever the screen. The plot does gain
/// from room, so it takes what is left, which is also what makes the
/// instrument fit rather than scroll.
const IDENTITY_HEIGHT: f32 = 36.0;
/// Below this the plot is a line rather than a waveform, so the program band
/// yields — and scrolls — instead: a squashed curve proves nothing.
const PROOF_MINIMUM_HEIGHT: f32 = 120.0;
const SEAM: f32 = 1.0;

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    if state.workspace.stimulus_library.is_empty() {
        show_empty(ui, state);
        return;
    }
    select_first_if_unresolved(state);
    let Some(stage) = resolve(state) else {
        show_empty(ui, state);
        return;
    };
    let mut actions = Vec::new();
    paint(ui, state, &stage, &mut actions);
    ask_before_deleting(ui, state);
    for action in actions {
        apply(state, &stage, action);
    }
}

/// The second question a delete asks when it would cost something.
///
/// A definition nothing has adopted and nothing is editing is removed on the
/// press; one with adopters or an unapplied draft says what deleting it costs
/// first, because both of those are work the press throws away.
fn ask_before_deleting(ui: &Ui, state: &mut AppState) {
    use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

    let Some(name) = state
        .workbench
        .stimulus_editor
        .pending_delete()
        .map(str::to_owned)
    else {
        return;
    };
    let consequence = crate::workbench::app::actions::stimulus::delete_needs_confirmation(state)
        .unwrap_or_default();
    let choice = Dialog::prompt(format!("Delete \u{201c}{name}\u{201d}?"), "Delete")
        .description(consequence.clone())
        .size(DialogSize::Confirmation)
        .ghost("Cancel")
        .show(ui.ctx(), |ui| {
            ui.label(consequence.as_str());
        });
    match choice {
        DialogChoice::Primary => {
            state.workbench.stimulus_editor.take_pending_delete();
            crate::workbench::app::actions::stimulus::delete_definition(state);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.workbench.stimulus_editor.take_pending_delete();
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// The empty state, which offers the one verb that resolves it.
fn show_empty(ui: &mut Ui, state: &mut AppState) {
    let messages = state.ui.messages();
    let title = messages.text(MessageId::StimulusLibraryEmpty);
    let detail = messages.text(MessageId::StimulusLibraryEmptyDetail);
    let label = messages.text(MessageId::StimulusNewDefinition);
    let mut create = false;
    empty_state_with_actions(ui, WorkbenchIcon::Source, &title, &detail, |ui| {
        create |= labeled_icon_button(ui, WorkbenchIcon::Add, &label, false, 150.0).clicked();
    });
    if create {
        crate::workbench::app::actions::stimulus::new_definition(state);
    }
}

/// Read the definition the browser is on, opening its draft if this is the
/// first frame that asked for it.
///
/// A selection the library no longer holds resolves to nothing rather than to
/// an empty definition, and the caller falls back to the empty stage — which
/// cannot happen after [`select_first_if_unresolved`] except in the single
/// frame a delete lands on.
fn resolve(state: &mut AppState) -> Option<Stage> {
    let name = state.workbench.selected_stimulus_definition.clone()?;
    let saved = state.workspace.stimulus_library.get(&name)?.clone();
    let timing = crate::workbench::app::actions::property_edit::stimulus_preview_timing(state);
    let editor = &mut state.workbench.stimulus_editor;
    let draft = editor.draft_for(&saved);
    let working = draft.working().clone();
    let dirty = draft.is_dirty();
    let can_undo = draft.can_undo();
    let span = editor.span;
    let selected_point = editor.selected_point(saved.name());
    let editing_field = editor.editing_field(saved.name()).map(str::to_owned);
    let focus_name = editor.take_focus_name();
    let live = editor.live_record(saved.name(), &working);
    let card = StimulusRealization::card(&live);
    let adopters = crate::workbench::app::actions::stimulus::design_adopters(state, &name);
    let findings = findings(state, &live, &card);
    let realization = state
        .workbench
        .stimulus_editor
        .realization(&live, span, timing)
        .clone();
    Some(Stage {
        saved,
        working: live,
        dirty,
        can_undo,
        realization,
        timing,
        span,
        selected_point,
        card,
        adopters,
        findings,
        editing_field,
        focus_name,
    })
}

/// Select the first definition when the browser is on none, or on one the
/// library no longer holds.
///
/// The mockup does this rather than showing a "nothing selected" stage over a
/// list of definitions: the instrument's subject is a definition, and a
/// library that holds one always has a first.
fn select_first_if_unresolved(state: &mut AppState) {
    let resolved = state
        .workbench
        .selected_stimulus_definition
        .as_deref()
        .is_some_and(|name| state.workspace.stimulus_library.get(name).is_some());
    if resolved {
        return;
    }
    state.workbench.selected_stimulus_definition = state
        .workspace
        .stimulus_library
        .definitions()
        .first()
        .map(|definition| definition.name().to_owned());
}

/// Everything the audit strip states about this record.
///
/// Errors are the two things that stop a definition from becoming a card: the
/// generator refusing to write one, and a name the library cannot hold. The
/// rest is `source_contract`, at the strength that module assigns — a refusal
/// blocks Apply because the deck would refuse it, and an advisory does not
/// because the card runs.
fn findings(
    state: &AppState,
    record: &StimulusDefinition,
    card: &Result<String, Vec<String>>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for message in name_findings(state, record) {
        findings.push(Finding {
            blocking: true,
            message,
        });
    }
    if let Err(errors) = card {
        for message in errors {
            findings.push(Finding {
                blocking: true,
                message: message.clone(),
            });
        }
    }
    for finding in crate::workbench::app::actions::stimulus::contract_findings(state, record) {
        findings.push(Finding {
            blocking: finding.strength == ContractStrength::Refusal,
            message: finding.message,
        });
    }
    findings.sort_by_key(|finding| !finding.blocking);
    findings
}

/// Why this name cannot be the definition's, if it cannot.
fn name_findings(state: &AppState, record: &StimulusDefinition) -> Vec<String> {
    use crate::state::stimulus_library::definition::is_spice_identifier;

    let mut findings = Vec::new();
    let name = record.name();
    if !is_spice_identifier(name) {
        findings.push(format!(
            "'{name}' is not a SPICE identifier, so no placed source could carry it"
        ));
    }
    let selected = state.workbench.selected_stimulus_definition.as_deref();
    let collides = state
        .workspace
        .stimulus_library
        .definitions()
        .iter()
        .any(|other| {
            other.name().eq_ignore_ascii_case(name)
                && !selected.is_some_and(|current| other.name().eq_ignore_ascii_case(current))
        });
    if collides {
        findings.push(format!("This project already defines '{name}'"));
    }
    findings
}

/// Lay the five bands into the stage and paint them.
fn paint(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let frame = ui.available_rect_before_wrap();
    let (rect, _) = ui.allocate_exact_size(frame.size(), Sense::hover());
    let seam_color = Tokens::get(ui.ctx()).color.border_strong;
    let realization = realization::content_height(stage);
    let audit = audit::content_height(stage);
    let fixed = IDENTITY_HEIGHT + realization + audit + 4.0 * SEAM;
    let proof_and_program = (rect.height() - fixed).max(PROOF_MINIMUM_HEIGHT);
    let program = program::content_height(ui, state, stage, rect.width())
        .min((proof_and_program - PROOF_MINIMUM_HEIGHT).max(0.0));
    let proof = proof_and_program - program;

    let mut top = rect.top();
    let mut band = |height: f32| {
        let band = Rect::from_min_max(
            egui::pos2(rect.left(), top),
            egui::pos2(rect.right(), top + height),
        );
        top += height + SEAM;
        band
    };
    let identity_rect = band(IDENTITY_HEIGHT);
    let proof_rect = band(proof);
    let program_rect = band(program);
    let realization_rect = band(realization);
    let audit_rect = band(audit);

    for seam in [identity_rect, proof_rect, program_rect, realization_rect] {
        ui.painter().hline(
            rect.x_range(),
            seam.bottom(),
            egui::Stroke::new(SEAM, seam_color),
        );
    }

    in_band(ui, identity_rect, |ui| {
        identity::show(ui, state, stage, actions);
    });
    in_band(ui, proof_rect, |ui| {
        proof::show(ui, state, stage, actions);
    });
    in_band(ui, program_rect, |ui| {
        program::show(ui, state, stage, actions);
    });
    in_band(ui, realization_rect, |ui| {
        realization::show(ui, state, stage, actions);
    });
    in_band(ui, audit_rect, |ui| {
        audit::show(ui, state, stage);
    });
}

/// Lay one row out from both ends at once.
///
/// egui rows have a single cursor, so a right-aligned group added after a
/// greedy label is handed whatever is left — which at a narrow stage is a
/// negative width, and egui refuses that outright. Giving each end its own
/// rectangle makes the two halves independent: the trailing verbs keep their
/// room and the leading text is elided into what remains.
pub(super) fn split_row(
    ui: &mut Ui,
    height: f32,
    trailing_width: f32,
    leading: impl FnOnce(&mut Ui),
    trailing: impl FnOnce(&mut Ui),
) {
    let available = ui.available_rect_before_wrap();
    if available.width() <= 1.0 || available.height() <= 1.0 {
        return;
    }
    let height = height.min(available.height());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(available.width(), height), Sense::hover());
    // A trailing group narrower than one icon button is not a group, so the
    // row keeps its leading half whole and drops it rather than handing it a
    // sliver nothing fits in.
    let trailing_width = trailing_width.clamp(0.0, (rect.width() - 48.0).max(0.0));
    let trailing_width = if trailing_width < 24.0 {
        0.0
    } else {
        trailing_width
    };
    let lead = Rect::from_min_max(
        rect.min,
        egui::pos2(rect.right() - trailing_width, rect.bottom()),
    );
    let tail = Rect::from_min_max(egui::pos2(lead.right(), rect.top()), rect.max);
    let clip = ui.clip_rect();
    let mut lead_ui = ui.new_child(UiBuilder::new().max_rect(lead));
    lead_ui.set_clip_rect(clip.intersect(lead));
    lead_ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), leading);
    if tail.width() < 24.0 {
        return;
    }
    let mut tail_ui = ui.new_child(UiBuilder::new().max_rect(tail));
    tail_ui.set_clip_rect(clip.intersect(tail));
    tail_ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), trailing);
}

/// One run of text in the trailing half of a [`split_row`], elided to a width
/// the caller states.
///
/// A `Label` in a right-to-left slot anchors at the slot's edge and grows the
/// wrong way, so trailing text is laid out here as a fixed-size item — the same
/// allocation a button makes, which the layout places correctly — and elided
/// against an explicit budget rather than against a remaining width the slot
/// cannot report.
pub(super) fn trailing_text(
    ui: &mut Ui,
    text: &str,
    font: egui::FontId,
    color: egui::Color32,
    max_width: f32,
) -> egui::Response {
    let shown = crate::workbench::design_system::elide_text(ui, text, &font, max_width.max(0.0));
    let galley = ui.painter().layout_no_wrap(shown.clone(), font, color);
    let (rect, response) = ui.allocate_exact_size(galley.size(), Sense::hover());
    ui.painter().galley(rect.min, galley, color);
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    if shown != text {
        response.on_hover_text(text)
    } else {
        response
    }
}

/// Paint one band into exactly its rectangle, clipped to it.
///
/// The clip is what makes the band contract real: a band that overflowed would
/// paint over the seam below it and the instrument would read as one long
/// column with rules drawn through it.
fn in_band(ui: &mut Ui, rect: Rect, content: impl FnOnce(&mut Ui)) {
    if rect.height() <= 0.0 {
        return;
    }
    let mut band = ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    band.set_clip_rect(ui.clip_rect().intersect(rect));
    band.set_min_size(Vec2::new(rect.width(), 0.0));
    band.set_max_size(rect.size());
    content(&mut band);
}

/// Carry out one thing the reader asked for.
fn apply(state: &mut AppState, stage: &Stage, action: StageAction) {
    use crate::workbench::app::actions::stimulus;

    match action {
        StageAction::Rename(name) => {
            state.workbench.stimulus_editor.clear_field();
            stimulus::edit_name(state, &name);
        }
        StageAction::SetFamily(family) => {
            state.workbench.stimulus_editor.clear_field();
            stimulus::edit_family(state, family);
        }
        StageAction::SetKind(kind) => {
            state.workbench.stimulus_editor.clear_field();
            stimulus::edit_kind(state, kind);
        }
        StageAction::TypeField { field, value } => {
            let name = stage.saved.name().to_owned();
            state
                .workbench
                .stimulus_editor
                .set_field_text(&name, &field, value);
        }
        StageAction::CommitField { field, value } => {
            state.workbench.stimulus_editor.clear_field();
            stimulus::edit_field(state, &field, &value);
        }
        StageAction::SetPoints(points) => {
            state.workbench.stimulus_editor.clear_field();
            stimulus::edit_field(state, "pwl_data", &points);
        }
        StageAction::SelectPoint(index) => {
            let name = stage.saved.name().to_owned();
            state.workbench.stimulus_editor.select_point(&name, index);
        }
        StageAction::SetSpan(span) => state.workbench.stimulus_editor.span = span,
        StageAction::Apply => stimulus::apply_draft(state),
        StageAction::Revert => stimulus::revert_draft(state),
        StageAction::Duplicate => stimulus::duplicate_definition(state),
        StageAction::Delete => {
            if stimulus::delete_needs_confirmation(state).is_some() {
                let name = stage.saved.name().to_owned();
                state.workbench.stimulus_editor.ask_before_deleting(&name);
            } else {
                stimulus::delete_definition(state);
            }
        }
        StageAction::ImportDataFile => stimulus::import_data_file(state),
        StageAction::Copy(text) => state.ui.clipboard_text_request = Some(text),
        StageAction::Place => stimulus::place_selected_definition(state),
        StageAction::Readopt(id) => {
            stimulus::readopt_adopter(state, id, stage.saved.name());
        }
        StageAction::OpenProperties(id) => {
            crate::workbench::app::open_property_editor(state, id);
        }
    }
}
