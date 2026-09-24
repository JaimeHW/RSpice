//! The cards this definition and its adopters emit.
//!
//! One line for the definition — the exact text adoption copies, between named
//! rather than resolved nets, because a library definition has no sheet — and
//! one line per placed source that adopted it, with the card that instance
//! really emits and where it stands against the library. Adopters own their
//! cards: publishing a revision moves the library and leaves every instance
//! reading "behind" until someone re-adopts, and that is what this band is for
//! looking at.
//!
//! The list is the whole design's, through
//! [`crate::workbench::app::actions::stimulus::design_adopters`]: a definition
//! adopted inside a child master drives the circuit exactly as one adopted at
//! the root does, and a master two instances reach adopts it twice. A row drawn
//! in another occurrence says where it is and carries no verbs, because both of
//! them address an instance by a component id that means something only inside
//! the buffer on screen.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, MessageId};

use crate::workbench::app::actions::stimulus::StimulusAdopter;

use super::super::super::design_system::{
    WorkbenchIcon, card_well, icon_button, labeled_icon_button_sized,
};
use super::{Stage, StageAction};

const ROW_HEIGHT: f32 = 24.0;
/// How many adopters the band shows before its list scrolls. Three is what a
/// definition in ordinary use has; the inspector lists every one.
const VISIBLE_ADOPTERS: usize = 3;
const BAND_FOOT: f32 = 4.0;
const INSET: f32 = 10.0;
const BUTTON: egui::Vec2 = egui::Vec2::new(24.0, 20.0);
const READOPT_WIDTH: f32 = 104.0;
/// Room each row keeps at its right edge, sized to the verbs and note it
/// carries there. The card on the left is elided into what remains.
const HEADER_NOTE_WIDTH: f32 = 170.0;
const DEFINITION_TRAIL_WIDTH: f32 = 352.0;
/// The Place verb on the definition line, which is the row that states what
/// adoption copies — and therefore the row a reader is on when they decide to
/// put one on the sheet. Wide enough for its icon and its whole word: a verb
/// elided to `Plac…` is a control that reads as broken.
const PLACE_WIDTH: f32 = 86.0;
const ADOPTER_TRAIL_WIDTH: f32 = 280.0;
/// Horizontal room the fixed items of a trailing group take, so its text is
/// elided against what is actually left.
const TRAIL_GAP: f32 = 6.0;
const TRAIL_INSET: f32 = 8.0;

/// How tall the band is: its head, the definition's card, and the adopters it
/// shows without scrolling.
pub(super) fn content_height(stage: &Stage) -> f32 {
    ROW_HEIGHT * (2 + stage.adopters.len().min(VISIBLE_ADOPTERS)) as f32 + BAND_FOOT
}

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    ui.spacing_mut().item_spacing.y = 0.0;
    super::split_row(
        ui,
        ROW_HEIGHT,
        HEADER_NOTE_WIDTH,
        |ui| {
            ui.add_space(INSET);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(if stage.adopters.is_empty() {
                        messages.text(MessageId::StimulusRealizationNoAdopters)
                    } else {
                        messages.format(
                            if stage.adopters.len() == 1 {
                                MessageId::StimulusRealizationSingular
                            } else {
                                MessageId::StimulusRealization
                            },
                            &[("count", &stage.adopters.len().to_string())],
                        )
                    })
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(palette.text_dim),
                )
                .truncate(),
            );
        },
        |ui| {
            ui.add_space(8.0);
            super::trailing_text(
                ui,
                &messages.text(MessageId::StimulusAdoptersOwnCards),
                theme::sans(tokens::FS_MICRO, FontWeight::Regular),
                palette.text_faint,
                HEADER_NOTE_WIDTH - TRAIL_INSET,
            );
        },
    );
    egui::ScrollArea::vertical()
        .id_salt("workbench.stimulus.realization")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            definition_line(ui, state, stage, actions);
            for adopter in &stage.adopters {
                adopter_line(ui, state, stage, adopter, actions);
            }
        });
}

/// The definition's own card: what adoption copies.
fn definition_line(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let (text, color) = match &stage.card {
        Ok(card) => (card.clone(), palette.text),
        Err(errors) => (errors.join("; "), palette.err),
    };
    let mut trailing = Vec::new();
    super::split_row(
        ui,
        ROW_HEIGHT,
        DEFINITION_TRAIL_WIDTH,
        |ui| {
            ui.add_space(INSET);
            card_well(ui, &text, color, INSET);
        },
        |ui| {
            ui.add_space(8.0);
            if stage.card.is_ok()
                && icon_button(
                    ui,
                    WorkbenchIcon::Copy,
                    &messages.text(MessageId::StimulusCopyCard),
                    false,
                    BUTTON,
                )
                .clicked()
            {
                trailing.push(StageAction::Copy(text.clone()));
            }
            ui.add_space(TRAIL_GAP);
            if labeled_icon_button_sized(
                ui,
                WorkbenchIcon::Source,
                &messages.text(MessageId::StimulusPlace),
                false,
                PLACE_WIDTH,
                20.0,
            )
            .on_hover_text(messages.text(MessageId::StimulusPlaceHint))
            .clicked()
            {
                trailing.push(StageAction::Place);
            }
            ui.add_space(TRAIL_GAP);
            super::trailing_text(
                ui,
                &messages.format(
                    if stage.dirty {
                        MessageId::StimulusDefinitionCardDraft
                    } else {
                        MessageId::StimulusDefinitionCard
                    },
                    &[("revision", &stage.saved.revision().to_string())],
                ),
                theme::sans(tokens::FS_MICRO, FontWeight::Regular),
                palette.text_faint,
                DEFINITION_TRAIL_WIDTH - TRAIL_INSET - BUTTON.x - PLACE_WIDTH - 3.0 * TRAIL_GAP,
            );
        },
    );
    actions.append(&mut trailing);
}

/// One placed adopter: its card, its provenance, and the two verbs on it.
///
/// An adopter drawn inside another occurrence carries neither verb. Both of
/// them address the instance by a component id that is unique only inside one
/// buffer, so offering them for a row of a child master would edit whichever
/// instance of the open sheet happened to carry that id; the row states which
/// occurrence to open instead.
fn adopter_line(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    adopter: &StimulusAdopter,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let (text, color) = match &adopter.source.card {
        Ok(card) => (card.clone(), palette.text_dim),
        Err(error) => (error.clone(), palette.err),
    };
    let mut trailing = Vec::new();
    let mut note_width = ADOPTER_TRAIL_WIDTH - TRAIL_INSET - TRAIL_GAP;
    super::split_row(
        ui,
        ROW_HEIGHT,
        ADOPTER_TRAIL_WIDTH,
        |ui| {
            ui.add_space(INSET);
            card_well(ui, &text, color, INSET);
        },
        |ui| {
            ui.add_space(8.0);
            if adopter.on_this_sheet {
                note_width -= BUTTON.x + READOPT_WIDTH + 2.0 * TRAIL_GAP;
                if icon_button(
                    ui,
                    WorkbenchIcon::Sliders,
                    &messages.format(
                        MessageId::StimulusOpenProperties,
                        &[("instance", adopter.source.reference.as_str())],
                    ),
                    false,
                    BUTTON,
                )
                .clicked()
                {
                    trailing.push(StageAction::OpenProperties(adopter.source.component_id));
                }
                ui.add_space(6.0);
                if adopter.offers_readoption
                    && labeled_icon_button_sized(
                        ui,
                        WorkbenchIcon::Refresh,
                        &messages.format(
                            MessageId::StimulusReadopt,
                            &[("revision", &stage.saved.revision().to_string())],
                        ),
                        false,
                        READOPT_WIDTH,
                        20.0,
                    )
                    .clicked()
                {
                    trailing.push(StageAction::Readopt(adopter.source.component_id));
                }
                ui.add_space(TRAIL_GAP);
            }
            let note = if adopter.on_this_sheet {
                adopter.chip.clone()
            } else {
                format!("{} \u{b7} {}", adopter.occurrence, adopter.chip)
            };
            super::trailing_text(
                ui,
                &note,
                theme::mono(tokens::FS_MICRO, FontWeight::Regular),
                if adopter.behind || adopter.modified {
                    palette.warn
                } else {
                    palette.text_faint
                },
                note_width,
            );
        },
    );
    actions.append(&mut trailing);
}
