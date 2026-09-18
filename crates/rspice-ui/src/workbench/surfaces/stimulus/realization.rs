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
//! The list is the active sheet's instances. A definition adopted on another
//! sheet of the same design is a real adopter and is not listed here; the
//! hierarchy-wide list arrives with the inspector, which owns the projection
//! that resolves an occurrence to its component.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
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
const DEFINITION_TRAIL_WIDTH: f32 = 260.0;
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
            card_well(ui, &text, color);
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
                DEFINITION_TRAIL_WIDTH - TRAIL_INSET - BUTTON.x - TRAIL_GAP,
            );
        },
    );
    actions.append(&mut trailing);
}

/// One placed adopter: its card, its provenance, and the two verbs on it.
fn adopter_line(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    adopter: &super::AdopterRow,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let (text, color) = match &adopter.card {
        Ok(card) => (card.clone(), palette.text_dim),
        Err(error) => (error.clone(), palette.err),
    };
    let mut trailing = Vec::new();
    super::split_row(
        ui,
        ROW_HEIGHT,
        ADOPTER_TRAIL_WIDTH,
        |ui| {
            ui.add_space(INSET);
            card_well(ui, &text, color);
        },
        |ui| {
            ui.add_space(8.0);
            if icon_button(
                ui,
                WorkbenchIcon::Sliders,
                &messages.format(
                    MessageId::StimulusOpenProperties,
                    &[("instance", adopter.name.as_str())],
                ),
                false,
                BUTTON,
            )
            .clicked()
            {
                trailing.push(StageAction::OpenProperties(adopter.id));
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
                trailing.push(StageAction::Readopt(adopter.id));
            }
            ui.add_space(TRAIL_GAP);
            super::trailing_text(
                ui,
                &adopter.chip,
                theme::mono(tokens::FS_MICRO, FontWeight::Regular),
                if adopter.behind || adopter.modified {
                    palette.warn
                } else {
                    palette.text_faint
                },
                ADOPTER_TRAIL_WIDTH - TRAIL_INSET - BUTTON.x - READOPT_WIDTH - 3.0 * TRAIL_GAP,
            );
        },
    );
    actions.append(&mut trailing);
}

/// One card, in the inset well every card on this band sits in.
///
/// The well is what says "this is text the netlister wrote" rather than a
/// sentence about it, and it is sized to the card so a short one does not
/// leave a slab of inset beside it. A card longer than its room is elided and
/// keeps its whole text under the pointer.
fn card_well(ui: &mut Ui, text: &str, color: egui::Color32) {
    const PAD: f32 = 6.0;
    let palette = Tokens::get(ui.ctx()).color;
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let room = (ui.available_width() - 2.0 * PAD - INSET).max(0.0);
    let shown = crate::workbench::design_system::elide_text(ui, text, &font, room);
    let elided = shown != text;
    let galley = ui.painter().layout_no_wrap(shown, font, color);
    let (rect, response) = ui.allocate_exact_size(
        egui::Vec2::new(galley.size().x + 2.0 * PAD, 18.0),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 2.0, palette.bg_inset);
    ui.painter().galley(
        egui::pos2(rect.left() + PAD, rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    if elided {
        response.on_hover_text(text);
    }
}
