//! What this definition is, and the verbs that change it.
//!
//! One row: the family's own mark, the name, the two things that decide which
//! card the netlister writes, where the definition stands against the library,
//! who has adopted it, and the four verbs. It is the band a reader looks at to
//! answer "which definition am I editing and is it saved", so nothing that
//! cannot be read at a glance belongs in it.

use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::state::stimulus_library::definition::{StimulusFamily, StimulusKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::state::NAME_FIELD;
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
use super::{Stage, StageAction};

/// Width of the editable name field. Wide enough for the longest name the
/// library's own generator produces plus a descriptive suffix, and fixed so
/// that the verbs on the right do not move when a name is typed.
const NAME_WIDTH: f32 = 168.0;
const COMBO_WIDTH: f32 = 96.0;
const KIND_WIDTH: f32 = 56.0;
const BUTTON: Vec2 = Vec2::new(28.0, 26.0);
/// Room the four verbs keep at the right of the identity row: Apply plus three
/// icon buttons and the spacing between them.
const VERB_WIDTH: f32 = 208.0;

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
            paint_family_mark(
                ui.painter(),
                mark,
                stage.working.family(),
                tokens.color.accent,
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
    let invalid = stage
        .findings
        .iter()
        .any(|finding| finding.blocking && finding.message.contains(stage.working.name()));
    let mut edit = egui::TextEdit::singleline(&mut text)
        .font(egui::TextStyle::Monospace)
        .desired_width(NAME_WIDTH)
        .margin(egui::Margin::symmetric(6, 3));
    if invalid {
        edit = edit.text_color(Tokens::get(ui.ctx()).color.err);
    }
    let response = ui.add(edit);
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label.as_str());
    });
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
    let tokens = Tokens::get(ui.ctx());
    let errors = stage.errors();
    let (text, color) = if !stage.dirty {
        (
            messages.format(
                MessageId::StimulusSavedRevision,
                &[("revision", &stage.saved.revision().to_string())],
            ),
            tokens.color.ok,
        )
    } else if errors > 0 {
        (
            messages.format(
                if errors == 1 {
                    MessageId::StimulusDraftErrorSingular
                } else {
                    MessageId::StimulusDraftErrors
                },
                &[("count", &errors.to_string())],
            ),
            tokens.color.err,
        )
    } else {
        (
            messages.text(MessageId::StimulusDraftNotApplied),
            tokens.color.warn,
        )
    };
    let response = chip(ui, &text, color);
    // Undo is what takes a draft back, and a reader looking at the chip that
    // says "not applied" is exactly the reader who wants to know that.
    if stage.can_undo {
        response.on_hover_text(messages.text(MessageId::StimulusUndoAvailable));
    }
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
                    adopter.name,
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
                .map(|adopter| format!("{} \u{b7} {}", adopter.name, adopter.chip))
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
fn chip(ui: &mut Ui, text: &str, color: Color32) -> egui::Response {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let galley = ui.painter().layout_no_wrap(text.to_owned(), font, color);
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(galley.size().x + 14.0, 19.0), Sense::hover());
    ui.painter().rect(
        rect,
        tokens.radius,
        color.gamma_multiply(0.14),
        Stroke::new(1.0, color.gamma_multiply(0.5)),
        egui::StrokeKind::Inside,
    );
    ui.painter().galley(
        Pos2::new(rect.left() + 7.0, rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    response
}

/// The family's mark, drawn as geometry.
///
/// Vectors rather than glyphs: the bundled text faces do not carry a sine
/// wave, a pulse train or a noise band, and a missing glyph is a tofu box in
/// the one place the reader looks first to know what they are editing.
pub(super) fn paint_family_mark(
    painter: &egui::Painter,
    rect: Rect,
    family: StimulusFamily,
    color: Color32,
) {
    let stroke = Stroke::new(1.3, color);
    let box_rect = rect.shrink(2.0);
    let x = |fraction: f32| egui::lerp(box_rect.left()..=box_rect.right(), fraction);
    let y = |fraction: f32| egui::lerp(box_rect.bottom()..=box_rect.top(), fraction);
    let line = |points: Vec<Pos2>| painter.add(egui::Shape::line(points, stroke));
    match family {
        StimulusFamily::Dc => {
            line(vec![Pos2::new(x(0.0), y(0.5)), Pos2::new(x(1.0), y(0.5))]);
        }
        StimulusFamily::Ac => {
            line(vec![Pos2::new(x(0.0), y(0.5)), Pos2::new(x(1.0), y(0.5))]);
            line(sine(&x, &y, 1.0, 0.18));
        }
        StimulusFamily::Sin => {
            line(sine(&x, &y, 1.0, 0.42));
        }
        StimulusFamily::Pulse => {
            line(vec![
                Pos2::new(x(0.0), y(0.15)),
                Pos2::new(x(0.2), y(0.15)),
                Pos2::new(x(0.2), y(0.85)),
                Pos2::new(x(0.6), y(0.85)),
                Pos2::new(x(0.6), y(0.15)),
                Pos2::new(x(1.0), y(0.15)),
            ]);
        }
        StimulusFamily::Pwl | StimulusFamily::PwlFile => {
            line(vec![
                Pos2::new(x(0.0), y(0.2)),
                Pos2::new(x(0.3), y(0.85)),
                Pos2::new(x(0.6), y(0.4)),
                Pos2::new(x(1.0), y(0.6)),
            ]);
        }
        StimulusFamily::Exp => {
            line(
                (0..=12_u8)
                    .map(|step| {
                        let fraction = f32::from(step) / 12.0;
                        Pos2::new(x(fraction), y(0.15 + 0.7 * (1.0 - (-3.0 * fraction).exp())))
                    })
                    .collect(),
            );
        }
        StimulusFamily::Sffm => {
            line(sine(&x, &y, 3.0, 0.35));
        }
        StimulusFamily::Am => {
            line(sine(&x, &y, 4.0, 0.42));
            line(vec![
                Pos2::new(x(0.0), y(0.6)),
                Pos2::new(x(0.5), y(0.95)),
                Pos2::new(x(1.0), y(0.6)),
            ]);
        }
        StimulusFamily::Pat => {
            line(vec![
                Pos2::new(x(0.0), y(0.15)),
                Pos2::new(x(0.25), y(0.15)),
                Pos2::new(x(0.25), y(0.85)),
                Pos2::new(x(0.5), y(0.85)),
                Pos2::new(x(0.5), y(0.15)),
                Pos2::new(x(0.75), y(0.15)),
                Pos2::new(x(0.75), y(0.85)),
                Pos2::new(x(1.0), y(0.85)),
            ]);
        }
        StimulusFamily::Trnoise => {
            line(
                (0..=16_u8)
                    .map(|step| {
                        let fraction = f32::from(step) / 16.0;
                        let jitter =
                            [0.5, 0.8, 0.3, 0.65, 0.2, 0.75, 0.45, 0.6][usize::from(step) % 8];
                        Pos2::new(x(fraction), y(jitter))
                    })
                    .collect(),
            );
        }
        StimulusFamily::Trrandom => {
            line(vec![
                Pos2::new(x(0.0), y(0.4)),
                Pos2::new(x(0.25), y(0.4)),
                Pos2::new(x(0.25), y(0.8)),
                Pos2::new(x(0.5), y(0.8)),
                Pos2::new(x(0.5), y(0.25)),
                Pos2::new(x(0.75), y(0.25)),
                Pos2::new(x(0.75), y(0.6)),
                Pos2::new(x(1.0), y(0.6)),
            ]);
        }
    }
}

/// `cycles` periods of a sine across the mark, at `amplitude` of its height.
fn sine(
    x: &impl Fn(f32) -> f32,
    y: &impl Fn(f32) -> f32,
    cycles: f32,
    amplitude: f32,
) -> Vec<Pos2> {
    (0..=24_u8)
        .map(|step| {
            let fraction = f32::from(step) / 24.0;
            Pos2::new(
                x(fraction),
                y(0.5 + amplitude * (fraction * cycles * std::f32::consts::TAU).sin()),
            )
        })
        .collect()
}
