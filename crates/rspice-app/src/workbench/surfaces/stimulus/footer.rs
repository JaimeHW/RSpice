//! The specification this definition emits.
//!
//! One line, and the verb that copies it. It is the only thing in the
//! instrument that is neither a field nor a picture: the exact text the
//! netlister wrote, including the positional slots it filled for the fields
//! that were left empty, which is the one place "what I typed" meets "what
//! gets simulated".
//!
//! The specification rather than a whole card. A library record's card head —
//! `name p n` — is scaffolding: the nets are placeholders because a definition
//! has no sheet to resolve them against, and the instance name is the
//! library's key rather than a placed source's reference, so the line reads as
//! a card and would not parse as one. The specification is true, exact, and
//! what every adopter's card carries verbatim. Real cards, between real nets,
//! are what the inspector lists against the adopters that hold them.
//!
//! Nothing is said here about whether the definition is sound. A refusal is
//! shown in the specification's place, because a generator that refused wrote
//! no text to show; everything else `source_contract` says is shown at the
//! field it is about, which is where the reader who can act on it is looking.
//! The identity band's lifecycle chip states whether there is anything wrong
//! at all, and Apply states what is blocking it.

use egui::{Sense, Ui, UiBuilder, Vec2};

use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{WorkbenchIcon, card_well, icon_button};
use super::{Stage, StageAction};

/// The band's one line.
///
/// Fixed, and small enough that the plot keeps the room: a band that measured
/// itself against the record would move the waveform every time a field was
/// typed.
pub(super) const HEIGHT: f32 = 28.0;
const INSET: f32 = 10.0;
const BUTTON: Vec2 = Vec2::new(24.0, 20.0);
/// Air between the specification and the verb that copies it.
const GAP: f32 = 6.0;

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let messages = state.ui.messages();
    let band = ui.available_rect_before_wrap();
    ui.spacing_mut().item_spacing.y = 0.0;

    let (row, _) = ui.allocate_exact_size(Vec2::new(band.width(), HEIGHT), Sense::hover());
    let mut row = ui.new_child(
        UiBuilder::new()
            .id_salt("workbench.stimulus.footer")
            .max_rect(row)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    row.set_clip_rect(ui.clip_rect().intersect(band));
    row.add_space(INSET);

    let spec = stage.spec.clone().map_err(|errors| errors.join("; "));
    // The room kept at the row's right edge, so a long specification is elided
    // against the track it really has rather than running under the verb.
    let trailing = GAP + BUTTON.x + INSET;
    card_well(&mut row, spec.as_deref().map_err(String::as_str), trailing);
    if let Ok(text) = &spec {
        row.add_space(GAP);
        if icon_button(
            &mut row,
            WorkbenchIcon::Copy,
            &messages.text(MessageId::StimulusCopySpec),
            false,
            BUTTON,
        )
        .clicked()
        {
            actions.push(StageAction::Copy(text.clone()));
        }
    }
}
