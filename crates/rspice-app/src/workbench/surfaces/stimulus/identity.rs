//! What this definition is, and the verbs that change it.
//!
//! One row: the family's own mark, the name, the two things that decide which
//! card the netlister writes, where the definition stands against the library,
//! who has adopted it, and the four verbs. It is the band a reader looks at to
//! answer "which definition am I editing and is it saved", so nothing that
//! cannot be read at a glance belongs in it.

use egui::{Pos2, Sense, Ui, Vec2};

use crate::state::stimulus_library::definition::{StimulusFamily, StimulusKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::state::NAME_FIELD;
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{
    Problem, WorkbenchIcon, icon_button, labeled_icon_button_sized, paint_problem_mark,
};
use super::{Stage, StageAction};

/// Width of the editable name field. Wide enough for the longest name the
/// library's own generator produces plus a descriptive suffix, and fixed so
/// that the verbs on the right do not move when a name is typed.
const NAME_WIDTH: f32 = 168.0;
const COMBO_WIDTH: f32 = 96.0;
const KIND_WIDTH: f32 = 56.0;
const BUTTON: Vec2 = Vec2::new(28.0, 26.0);
/// Side of the lifecycle chip's problem mark: the kit's own, so it matches
/// every other marked line in the product.
use crate::ui::widgets::PROBLEM_MARK_SIDE as CHIP_MARK_SIDE;
/// The air after it. Not the kit's `PROBLEM_MARK_GAP`: a chip sets its mark
/// inside its own padding, and 5 pt is what keeps the mark off the chip edge
/// without the words drifting out of the chip.
const CHIP_MARK_GAP: f32 = 5.0;
/// Room the five verbs keep at the right of the identity row: Apply plus four
/// icon buttons and the spacing between them.
const VERB_WIDTH: f32 = 250.0;

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let messages = state.ui.messages();
    let tokens = Tokens::get(ui.ctx());
    let mut trailing = Vec::new();
    super::split_row(
        ui,
        ui.available_height(),
        VERB_WIDTH,
        |ui| {
            ui.add_space(8.0);
            let (mark, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
            // Which waveform family this is, drawn as its own shape. Accent
            // would read as a selection the glyph does not carry.
            paint_family_mark(
                ui.painter(),
                mark,
                stage.working.family(),
                tokens.color.text_dim,
            );
            ui.add_space(6.0);

            name_field(ui, state, stage, actions);
            ui.add_space(6.0);
            family_combo(
                ui,
                &messages.text(MessageId::StimulusFieldFamily),
                stage,
                actions,
            );
            kind_combo(
                ui,
                &messages.text(MessageId::StimulusFieldQuantity),
                stage,
                actions,
            );

            ui.add_space(8.0);
            lifecycle_chip(ui, state, stage);
            ui.add_space(8.0);
            adopter_summary(ui, state, stage);
        },
        |ui| {
            ui.add_space(8.0);
            apply_button(ui, state, stage, &mut trailing);
            ui.add_space(4.0);
            if icon_button(
                ui,
                WorkbenchIcon::Undo,
                &messages.text(MessageId::StimulusRevert),
                false,
                BUTTON,
            )
            .clicked()
                && stage.dirty
            {
                trailing.push(StageAction::Revert);
            }
            ui.add_space(6.0);
            if icon_button(
                ui,
                WorkbenchIcon::Trash,
                &delete_label(&messages, stage),
                false,
                BUTTON,
            )
            .clicked()
            {
                trailing.push(StageAction::Delete);
            }
            if icon_button(
                ui,
                WorkbenchIcon::Copy,
                &messages.text(MessageId::StimulusDuplicate),
                false,
                BUTTON,
            )
            .clicked()
            {
                trailing.push(StageAction::Duplicate);
            }
            ui.add_space(6.0);
            // Leftmost of the group, clear of the two verbs that change the
            // record: placing is the one verb here that acts on the design.
            if icon_button(
                ui,
                WorkbenchIcon::Source,
                &messages.text(MessageId::StimulusPlaceHint),
                false,
                BUTTON,
            )
            .clicked()
            {
                trailing.push(StageAction::Place);
            }
        },
    );
    actions.append(&mut trailing);
}

/// The name, as an editable identifier.
///
/// Renaming is an edit like any other: it lands on the working record, and it
/// is Apply that rewrites every adopter's provenance to the new name. Typing a
/// name the library already holds is reported by the audit strip and blocks
/// Apply rather than being refused keystroke by keystroke, because a name is
/// typed through states that are not yet valid.
fn name_field(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let messages = state.ui.messages();
    let label = messages.text(MessageId::StimulusFieldName);
    let editing = stage.editing_field.as_deref() == Some(NAME_FIELD);
    let mut text = if editing {
        state
            .workbench
            .stimulus_editor
            .field_text(stage.saved.name(), NAME_FIELD)
            .unwrap_or_default()
            .to_owned()
    } else {
        stage.working.name().to_owned()
    };
    // The field the rule named, not a sentence that happens to quote the name:
    // a definition called `td` would otherwise take every advisory about TD.
    let objection = stage
        .field_findings(NAME_FIELD)
        .map(|finding| finding.message.as_str())
        .collect::<Vec<_>>();
    let response = ui.add(
        egui::TextEdit::singleline(&mut text)
            .font(egui::TextStyle::Monospace)
            .desired_width(NAME_WIDTH)
            .margin(egui::Margin::symmetric(6, 3)),
    );
    // A refused field outlines, exactly as every other field on the stage
    // does. The typed name stays in the ordinary ink: what the reader entered
    // is not itself the error.
    if !objection.is_empty() {
        let tokens = Tokens::get(ui.ctx());
        ui.painter().rect_stroke(
            response.rect,
            tokens.radius,
            egui::Stroke::new(1.0, tokens.color.err),
            egui::StrokeKind::Inside,
        );
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label.as_str());
    });
    // An outlined name says that it cannot be this definition's; the hover
    // says why, and it is the only place that does.
    let response = if objection.is_empty() {
        response
    } else {
        response.on_hover_text(objection.join("\n"))
    };
    if stage.focus_name {
        response.request_focus();
    }
    if response.lost_focus() {
        actions.push(StageAction::Rename(text));
    } else if response.changed() || response.gained_focus() {
        actions.push(StageAction::TypeField {
            field: NAME_FIELD.to_owned(),
            value: text,
        });
    }
}

fn family_combo(ui: &mut Ui, label: &str, stage: &Stage, actions: &mut Vec<StageAction>) {
    let current = stage.working.family();
    let response = egui::ComboBox::from_id_salt("workbench.stimulus.identity.family")
        .width(COMBO_WIDTH)
        .selected_text(current.label())
        .show_ui(ui, |ui| {
            for family in StimulusFamily::ALL {
                if ui
                    .selectable_label(family == current, family.label())
                    .clicked()
                    && family != current
                {
                    actions.push(StageAction::SetFamily(family));
                }
            }
        });
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| node.set_label(label));
}

fn kind_combo(ui: &mut Ui, label: &str, stage: &Stage, actions: &mut Vec<StageAction>) {
    let current = stage.working.kind();
    let response = egui::ComboBox::from_id_salt("workbench.stimulus.identity.kind")
        .width(KIND_WIDTH)
        .selected_text(current.letter())
        .show_ui(ui, |ui| {
            for kind in [StimulusKind::Voltage, StimulusKind::Current] {
                if ui
                    .selectable_label(kind == current, kind.letter())
                    .clicked()
                    && kind != current
                {
                    actions.push(StageAction::SetKind(kind));
                }
            }
        });
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| node.set_label(label));
}

/// Where this definition stands: published, edited, or edited and refused.
fn lifecycle_chip(ui: &mut Ui, state: &AppState, stage: &Stage) {
    let messages = state.ui.messages();
    let errors = stage.errors();
    // Each state's fact, then the same fact at the length a narrow band can
    // give it. The short form is the lifecycle word alone, because the detail
    // it drops — which revision, how many errors — is stated whole by the
    // realization band and the audit strip under it, while the word is the one
    // thing only this chip says.
    //
    // Saved and unapplied are where the definition is; only a draft the
    // generator refused is a problem, and it is the words that say which of
    // the three this is.
    let (spellings, problem) = if !stage.dirty {
        (
            vec![
                messages.format(
                    MessageId::StimulusSavedRevision,
                    &[("revision", &stage.saved.revision().to_string())],
                ),
                messages.text(MessageId::StimulusSaved),
            ],
            None,
        )
    } else if errors > 0 {
        (
            vec![
                messages.format(
                    if errors == 1 {
                        MessageId::StimulusDraftErrorSingular
                    } else {
                        MessageId::StimulusDraftErrors
                    },
                    &[("count", &errors.to_string())],
                ),
                messages.text(MessageId::StimulusDraft),
            ],
            Some(Problem::Failure),
        )
    } else {
        (
            vec![
                messages.text(MessageId::StimulusDraftNotApplied),
                messages.text(MessageId::StimulusDraft),
            ],
            None,
        )
    };
    let Some(response) = chip(ui, &spellings, problem) else {
        return;
    };
    // The whole fact under the pointer whatever was painted, and — because a
    // reader looking at "not applied" is exactly the reader who wants it —
    // that undo will take the draft back.
    let hover = match (spellings.first(), stage.can_undo) {
        (Some(full), true) => format!(
            "{full}\n{}",
            messages.text(MessageId::StimulusUndoAvailable)
        ),
        (Some(full), false) => full.clone(),
        (None, _) => return,
    };
    response.on_hover_text(hover);
}

/// Who is carrying a copy of this definition, and how far each has drifted.
///
/// Three spellings, longest first, and the first that fits whole is the one
/// painted: every instance by name, then how many there are, then nothing. A
/// fact cut off mid-word (`no ad…`) is worse than the fact missing, and it is
/// never missing from the instrument — the realization band heads its list
/// with the same count.
fn adopter_summary(ui: &mut Ui, state: &AppState, stage: &Stage) {
    let messages = state.ui.messages();
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let named = if stage.adopters.is_empty() {
        messages.text(MessageId::StimulusNoAdopters)
    } else {
        stage
            .adopters
            .iter()
            .map(|adopter| {
                format!(
                    "{}{}{}",
                    adopter.source.reference,
                    if adopter.behind { "\u{2191}" } else { "" },
                    if adopter.modified { "\u{270e}" } else { "" }
                )
            })
            .collect::<Vec<_>>()
            .join(" \u{b7} ")
    };
    let mut spellings = vec![named];
    if !stage.adopters.is_empty() {
        spellings.push(messages.format(
            if stage.adopters.len() == 1 {
                MessageId::StimulusAdopterCountSingular
            } else {
                MessageId::StimulusAdopterCount
            },
            &[("count", &stage.adopters.len().to_string())],
        ));
    }
    let room = ui.available_width() - 8.0;
    let Some(galley) = spellings
        .into_iter()
        .map(|text| {
            ui.painter()
                .layout_no_wrap(text, font.clone(), tokens.color.text_dim)
        })
        .find(|galley| galley.size().x <= room)
    else {
        return;
    };
    let text = galley.job.text.clone();
    let (rect, response) = ui.allocate_exact_size(galley.size(), Sense::hover());
    ui.painter().galley(rect.min, galley, tokens.color.text_dim);
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &text));
    if !stage.adopters.is_empty() {
        response.on_hover_text(
            stage
                .adopters
                .iter()
                .map(|adopter| format!("{} \u{b7} {}", adopter.source.reference, adopter.chip))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
}

/// Apply, and the reason it is unavailable when it is.
fn apply_button(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let messages = state.ui.messages();
    let label = messages.text(MessageId::StimulusApply);
    let block = stage.apply_block();
    let response = ui
        .add_enabled_ui(block.is_none(), |ui| {
            labeled_icon_button_sized(ui, WorkbenchIcon::Check, &label, false, 82.0, 26.0)
        })
        .inner;
    match block {
        Some(reason) => {
            response.on_disabled_hover_text(reason);
        }
        None => {
            if response.clicked() {
                actions.push(StageAction::Apply);
            }
        }
    }
}

/// What Delete promises, which depends on who is reading this definition.
fn delete_label(messages: &crate::workbench::MessageCatalog, stage: &Stage) -> String {
    if stage.adopters.is_empty() {
        messages.text(MessageId::StimulusDelete)
    } else {
        messages.format(
            if stage.adopters.len() == 1 {
                MessageId::StimulusDeleteWithAdopterSingular
            } else {
                MessageId::StimulusDeleteWithAdopters
            },
            &[("count", &stage.adopters.len().to_string())],
        )
    }
}

/// One status chip, painted rather than labelled so a press anywhere in the
/// band reaches the control under it.
///
/// `spellings` are the same fact at decreasing length, longest first, and the
/// first that fits its own frame whole is the one painted — the rule
/// [`adopter_summary`] follows, for the same reason: this band ends where the
/// verbs begin, and a lifecycle elided into them (`draft · 1 erro`) is a state
/// a reader has to guess at. Whatever is painted, the whole spelling is what
/// the chip announces and what it says under the pointer.
///
/// `None` when not even the shortest fits, which is the one case where a band
/// this narrow has nothing to give.
fn chip(ui: &mut Ui, spellings: &[String], problem: Option<Problem>) -> Option<egui::Response> {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mark_column = if problem.is_some() {
        CHIP_MARK_SIDE + CHIP_MARK_GAP
    } else {
        0.0
    };
    let room = ui.available_width() - mark_column;
    let galley = spellings
        .iter()
        .map(|text| {
            ui.painter()
                .layout_no_wrap(text.clone(), font.clone(), tokens.color.text_dim)
        })
        .find(|galley| galley.size().x + 14.0 <= room)?;
    let text = spellings.first()?.as_str();
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(mark_column + galley.size().x + 14.0, 19.0),
        Sense::hover(),
    );
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            egui::Rect::from_center_size(
                Pos2::new(rect.left() + 7.0 + CHIP_MARK_SIDE / 2.0, rect.center().y),
                Vec2::splat(CHIP_MARK_SIDE),
            ),
            problem,
        );
    }
    ui.painter().galley(
        Pos2::new(
            rect.left() + 7.0 + mark_column,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        tokens.color.text_dim,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    Some(response)
}

/// The family's mark, from the one painter of it.
///
/// It moved beside the mini painter when the library browser's group headings
/// needed the same twelve shapes: a second set drawn one layer up would be the
/// same twelve waveforms at a second weight, and a reader would have to learn
/// which of the two they were looking at.
pub(super) use crate::properties::source_preview::paint_family_mark;
