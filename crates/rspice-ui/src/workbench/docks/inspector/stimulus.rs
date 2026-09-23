//! What the inspector says about the definition the browser is on.
//!
//! The stage states what a definition *is* — its shape, its quantity, the
//! fields a card carries. This dock states everything about it that is not in
//! the definition: who in the design is carrying a copy, which analyses read
//! those copies, what applying a revision would cost, what the engine will do
//! with a card of this family, and the record's own bookkeeping. That is the
//! split every other workspace's inspector keeps between the object and the
//! world it sits in.
//!
//! Nothing here derives a fact. The adopters are
//! [`crate::workbench::app::actions::stimulus::design_adopters`], the consumers
//! are the ones the plan resolved onto those adopters, the cards come from the
//! netlister through the same walk the Studio's Excitations ledger lists, the
//! engine contract is `source_contract` asked about an empty card of this
//! family, and the purpose writes through the draft so it is undoable and
//! dirties the definition exactly as a waveform field does.

use egui::{Sense, Ui};

use crate::simulation::placed_sources::SourceConsumer;
use crate::state::ContractStrength;
use crate::state::stimulus_library::definition::{StimulusDefinition, StimulusFamily};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app::actions::stimulus::{self as stimulus_actions, StimulusAdopter};
use crate::workbench::state::{PURPOSE_FIELD, SimulationPage, Workspace};
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{
    card_well, property_row, property_row_input_with_hint, property_row_wrapped,
};
use super::section_header;

/// Air between two adopter cards, and the inset one keeps.
const CARD_GAP: f32 = 8.0;
const CARD_INSET: i8 = 8;
/// A card's verbs sit on one row, because they are three readings of the same
/// instance rather than three steps; this is the air between them and the
/// height they claim.
const ACTION_GAP: f32 = 6.0;
const ACTION_HEIGHT: f32 = 24.0;
/// The column a property row's label starts in, which every sentence and every
/// card in this dock lines up with.
const SECTION_PAD: f32 = 10.0;

/// One thing the reader asked the inspector to do, collected while painting so
/// no section mutates the design under the sections after it.
enum Verb {
    /// Show one adopter on the drawing it is in.
    Reveal(usize),
    /// Open Component Properties on one adopter.
    Properties(usize),
    /// Copy the library's revision back onto one adopter.
    Readopt(usize),
    /// Open the plan page that lists which analyses read which excitation.
    OpenPlanReaders,
    /// Keep the purpose a reader is typing, without making it an edit yet.
    TypePurpose(String),
    /// Write the purpose into the draft, as one undoable step.
    CommitPurpose(String),
}

pub(in crate::workbench::docks) fn show(ui: &mut Ui, state: &mut AppState) {
    let messages = state.ui.messages();
    let library = &state.workspace.stimulus_library;
    let Some(definition) = state
        .workbench
        .selected_stimulus_definition
        .as_deref()
        .and_then(|name| library.get(name))
        .cloned()
    else {
        section_header(ui, &messages.text(MessageId::StimulusLibrarySection), None);
        property_row(
            ui,
            &messages.text(MessageId::StimulusFieldDefinitions),
            &library.len().to_string(),
        );
        property_row(
            ui,
            &messages.text(MessageId::StimulusFieldSelection),
            &messages.text(MessageId::StimulusNone),
        );
        return;
    };

    let adopters = stimulus_actions::design_adopters(state, definition.name());
    let mut verbs = Vec::new();
    adopters_section(ui, state, &adopters, &definition, &mut verbs);
    if consumers_section(ui, state, &adopters) {
        verbs.push(Verb::OpenPlanReaders);
    }
    impact_section(ui, state, &adopters, &definition);
    contract_section(ui, state, &definition);
    definition_section(ui, state, &definition, &mut verbs);
    for verb in verbs {
        apply(state, &adopters, verb);
    }
}

/// Who in the design is carrying a copy, and the verbs on each of them.
fn adopters_section(
    ui: &mut Ui,
    state: &AppState,
    adopters: &[StimulusAdopter],
    definition: &StimulusDefinition,
    verbs: &mut Vec<Verb>,
) {
    let messages = state.ui.messages();
    section_header(
        ui,
        &messages.text(MessageId::StimulusAdoptersSection),
        Some(&adopters.len().to_string()),
    );
    if adopters.is_empty() {
        note(ui, &messages.text(MessageId::StimulusAdoptersNone));
        return;
    }
    for (index, adopter) in adopters.iter().enumerate() {
        adopter_card(ui, state, adopter, definition, index, verbs);
        ui.add_space(CARD_GAP);
    }
}

/// One adopter: which instance it is, where the run reaches it, the card it
/// really emits, and what can be done to it from here.
fn adopter_card(
    ui: &mut Ui,
    state: &AppState,
    adopter: &StimulusAdopter,
    definition: &StimulusDefinition,
    index: usize,
    verbs: &mut Vec<Verb>,
) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    egui::Frame::NONE
        .fill(palette.bg_panel)
        .stroke(egui::Stroke::new(1.0, palette.border))
        .corner_radius(3.0)
        .inner_margin(egui::Margin::same(CARD_INSET))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 4.0;
            let chip_tone = if adopter.behind || adopter.modified {
                palette.warn
            } else {
                palette.text_faint
            };
            // The instance, where the run reaches it and where it stands share
            // one line: three registers of one fact, and a reader comparing two
            // cards reads down one column for each of them.
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 16.0), Sense::hover());
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Label,
                    true,
                    format!(
                        "{} \u{b7} {} \u{b7} {}",
                        adopter.source.reference, adopter.occurrence, adopter.chip
                    ),
                )
            });
            let reference = ui.painter().layout_no_wrap(
                adopter.source.reference.clone(),
                theme::mono(tokens::FS_1, FontWeight::Medium),
                palette.text,
            );
            let reference_width = reference.size().x;
            ui.painter().galley(
                egui::pos2(rect.left(), rect.center().y - reference.size().y * 0.5),
                reference,
                palette.text,
            );
            ui.painter().text(
                egui::pos2(rect.left() + reference_width + 8.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &adopter.occurrence,
                theme::mono(tokens::FS_MICRO, FontWeight::Regular),
                palette.text_faint,
            );
            ui.painter().text(
                rect.right_center(),
                egui::Align2::RIGHT_CENTER,
                &adopter.chip,
                theme::mono(tokens::FS_MICRO, FontWeight::Regular),
                chip_tone,
            );
            let (card, tone) = match &adopter.source.card {
                Ok(card) => (card.as_str(), palette.text_dim),
                Err(refusal) => (refusal.as_str(), palette.err),
            };
            card_well(ui, card, tone, 0.0);
            // Every verb but the first addresses the instance by a component
            // id, which means something only inside the buffer on screen. An
            // adopter drawn elsewhere is therefore offered the one verb that
            // reaches it and a sentence saying how to reach the rest, never a
            // control that is present and does nothing.
            let verbs_here = if adopter.on_this_sheet {
                2 + usize::from(adopter.offers_readoption)
            } else {
                1
            };
            let readopt = messages.format(
                MessageId::StimulusReadopt,
                &[("revision", &definition.revision().to_string())],
            );
            let track = ui.available_width();
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = ACTION_GAP;
                let width = (track - ACTION_GAP * (verbs_here - 1) as f32) / verbs_here as f32;
                if action(
                    ui,
                    &messages.text(MessageId::StimulusShowInSchematic),
                    width,
                ) {
                    verbs.push(Verb::Reveal(index));
                }
                if !adopter.on_this_sheet {
                    return;
                }
                if action(
                    ui,
                    &messages.text(MessageId::StimulusOpenPropertiesShort),
                    width,
                ) {
                    verbs.push(Verb::Properties(index));
                }
                if adopter.offers_readoption && action(ui, &readopt, width) {
                    verbs.push(Verb::Readopt(index));
                }
            });
            if !adopter.on_this_sheet {
                note(ui, &messages.text(MessageId::StimulusAdopterElsewhere));
            }
        });
}

/// Which analyses read the copies, and as what.
///
/// Absent when no adopter is read at all: a heading over nothing is a section
/// the reader has to rule out, and the Edit impact block below already states
/// how many placed sources carry the definition.
///
/// A row opens the plan page that lists these readings, which is the route the
/// Studio's own Excitations rail takes — this dock states which analyses read a
/// definition, and the plan is where one is changed.
fn consumers_section(ui: &mut Ui, state: &AppState, adopters: &[StimulusAdopter]) -> bool {
    let messages = state.ui.messages();
    let rows: Vec<(&str, &SourceConsumer)> = adopters
        .iter()
        .flat_map(|adopter| {
            adopter
                .source
                .consumers
                .iter()
                .map(move |consumer| (adopter.source.reference.as_str(), consumer))
        })
        .collect();
    if rows.is_empty() {
        return false;
    }
    section_header(
        ui,
        &messages.text(MessageId::StimulusConsumersSection),
        Some(&rows.len().to_string()),
    );
    let mut opened = false;
    for (reference, consumer) in rows {
        let text = messages.format(
            if consumer.reads() {
                MessageId::StimulusConsumerRow
            } else {
                MessageId::StimulusConsumerDisabled
            },
            &[
                ("analysis", consumer.analysis.as_str()),
                ("role", consumer.role),
            ],
        );
        let response = property_row_wrapped(ui, reference, &text).interact(Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                format!("{reference}: {text}"),
            )
        });
        theme::paint_focus_ring(ui, &response, response.rect);
        opened |= response.clicked();
    }
    opened
}

/// What publishing a revision would and would not change.
///
/// Every line is a fact the product already backs: the count is the walk's, the
/// revision is the record's, and the two sentences about adopters restate the
/// rule `StimulusLibrary::apply` is written to — adopters own their cards, so
/// publishing moves the library and nothing else.
fn impact_section(
    ui: &mut Ui,
    state: &AppState,
    adopters: &[StimulusAdopter],
    definition: &StimulusDefinition,
) {
    let messages = state.ui.messages();
    section_header(ui, &messages.text(MessageId::StimulusImpactSection), None);
    property_row(
        ui,
        &messages.text(MessageId::StimulusImpactReadBy),
        &messages.format(
            if adopters.len() == 1 {
                MessageId::StimulusImpactReadBySingular
            } else {
                MessageId::StimulusImpactReadByPlural
            },
            &[("count", &adopters.len().to_string())],
        ),
    );
    let next = definition.revision().saturating_add(1).to_string();
    property_row_wrapped(
        ui,
        &messages.text(MessageId::StimulusImpactOnApply),
        &messages.format(
            if adopters.is_empty() {
                MessageId::StimulusImpactOnApplyAlone
            } else {
                MessageId::StimulusImpactOnApplyAdopted
            },
            &[("revision", &next)],
        ),
    );
    if !adopters.is_empty() {
        property_row_wrapped(
            ui,
            &messages.text(MessageId::StimulusImpactReadopt),
            &messages.text(MessageId::StimulusImpactReadoptDetail),
        );
    }
    property_row_wrapped(
        ui,
        &messages.text(MessageId::StimulusImpactFrozen),
        &messages.text(MessageId::StimulusImpactFrozenDetail),
    );
}

/// What the engine will do with a card of this family that authors nothing.
///
/// Asked of `source_contract` rather than written out here: these are the
/// substitutions and clamps the evaluator performs, one owner states them, and
/// a second table in a dock is a table that goes stale the first time a rule
/// changes. An empty card is the subject because that is the family's contract
/// rather than this definition's — what this definition's own fields provoke is
/// the instrument's audit strip, one pane away.
fn contract_section(ui: &mut Ui, state: &AppState, definition: &StimulusDefinition) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let family = definition.family();
    section_header(
        ui,
        &messages.format(
            MessageId::StimulusContractSection,
            &[("family", family.label())],
        ),
        None,
    );
    let empty = empty_card(family, definition);
    // The card the netlister writes for a definition of this family that
    // authors nothing: the positional field order the engine reads, and what
    // it is handed where a field was left out. It is the netlister's own text
    // rather than a template restated here, so it cannot describe a card the
    // deck would not carry.
    if let Some(record) = empty.as_ref() {
        let (text, tone) =
            match record.card_text(crate::simulation::stimulus_realize::DETACHED_NETS) {
                Ok(card) => (card, palette.text_dim),
                Err(errors) => (errors.join("; "), palette.err),
            };
        caption(ui, &messages.text(MessageId::StimulusContractCard));
        ui.horizontal(|ui| {
            ui.add_space(SECTION_PAD);
            card_well(ui, &text, tone, SECTION_PAD);
        });
        ui.add_space(6.0);
    }
    let findings = empty
        .map(|record| stimulus_actions::contract_findings(state, &record))
        .unwrap_or_default();
    if findings.is_empty() {
        note(
            ui,
            &messages.format(
                MessageId::StimulusContractClean,
                &[("family", family.label())],
            ),
        );
        return;
    }
    for finding in findings {
        let response = property_row_wrapped(ui, finding.field, &finding.message);
        if finding.strength == ContractStrength::Refusal {
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    response.rect.left_top(),
                    egui::vec2(2.0, response.rect.height()),
                ),
                0.0,
                palette.err,
            );
        }
    }
}

/// A definition of this family and quantity that authors no field at all.
///
/// Under this definition's own name, so the card reads as a line this project
/// could carry rather than as a specimen from somewhere else. What separates it
/// from the definition's real card is the caption above it, not a second
/// instance name the library does not hold.
fn empty_card(
    family: StimulusFamily,
    definition: &StimulusDefinition,
) -> Option<StimulusDefinition> {
    StimulusDefinition::new(definition.name(), family.component_type(definition.kind())).ok()
}

/// A quiet line naming what the block under it is.
fn caption(ui: &mut Ui, text: &str) {
    let palette = Tokens::get(ui.ctx()).color;
    ui.horizontal(|ui| {
        ui.add_space(SECTION_PAD);
        ui.label(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                .color(palette.text_faint),
        );
    });
}

/// The record's own bookkeeping, and the one field of it a reader edits here.
fn definition_section(
    ui: &mut Ui,
    state: &AppState,
    definition: &StimulusDefinition,
    verbs: &mut Vec<Verb>,
) {
    let messages = state.ui.messages();
    let editor = &state.workbench.stimulus_editor;
    let dirty = editor.is_dirty(definition.name());
    section_header(
        ui,
        &messages.text(MessageId::StimulusDefinitionSection),
        Some(definition.name()),
    );
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldOwner),
        &messages.text(MessageId::StimulusOwnerProject),
    );
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldRevision),
        &messages.format(
            if dirty {
                MessageId::StimulusRevisionDraft
            } else {
                MessageId::StimulusRevisionSaved
            },
            &[("revision", &definition.revision().to_string())],
        ),
    );
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldModified),
        &crate::time_compat::utc_stamp(definition.modified_unix_ms()),
    );
    let working = editor.draft(definition.name()).map_or_else(
        || definition.purpose.clone(),
        |draft| draft.working().purpose.clone(),
    );
    let mut purpose = editor
        .field_text(definition.name(), PURPOSE_FIELD)
        .map_or(working, str::to_owned);
    let response = property_row_input_with_hint(
        ui,
        &messages.text(MessageId::StimulusFieldPurpose),
        &mut purpose,
        &messages.text(MessageId::StimulusPurposeHint),
        false,
    );
    if response.lost_focus() {
        verbs.push(Verb::CommitPurpose(purpose));
    } else if response.changed() || response.gained_focus() {
        verbs.push(Verb::TypePurpose(purpose));
    }
    property_row_wrapped(
        ui,
        &messages.text(MessageId::StimulusFieldStored),
        &messages.text(MessageId::StimulusStoredInProject),
    );
}

/// One verb on an adopter card, at the width its row divided between them.
fn action(ui: &mut Ui, label: &str, width: f32) -> bool {
    ui.add_sized(
        [width.max(24.0), ACTION_HEIGHT],
        egui::Button::new(
            egui::RichText::new(label).font(theme::sans(tokens::FS_0, FontWeight::Regular)),
        ),
    )
    .clicked()
}

/// One sentence in the dock's own quiet register, wrapped into its width and
/// inset to the column every property row's label starts in.
fn note(ui: &mut Ui, text: &str) {
    let palette = Tokens::get(ui.ctx()).color;
    ui.horizontal(|ui| {
        ui.add_space(SECTION_PAD);
        ui.add(
            egui::Label::new(
                egui::RichText::new(text)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(palette.text_dim),
            )
            .wrap(),
        );
    });
    ui.add_space(4.0);
}

/// Carry out one thing the reader asked for.
fn apply(state: &mut AppState, adopters: &[StimulusAdopter], verb: Verb) {
    let name = state
        .workbench
        .selected_stimulus_definition
        .clone()
        .unwrap_or_default();
    match verb {
        Verb::Reveal(index) => {
            if let Some(adopter) = adopters.get(index) {
                crate::workbench::app::actions::reveal::placed_instance(
                    state,
                    adopter.source.occurrence.as_ref(),
                    adopter.source.component_id,
                );
            }
        }
        Verb::Properties(index) => {
            if let Some(adopter) = adopters.get(index) {
                crate::workbench::app::open_property_editor(state, adopter.source.component_id);
            }
        }
        Verb::Readopt(index) => {
            if let Some(adopter) = adopters.get(index) {
                stimulus_actions::readopt_adopter(state, adopter.source.component_id, &name);
            }
        }
        Verb::OpenPlanReaders => {
            state.workbench.simulation_page = SimulationPage::Excitations;
            state.workbench.activate(Workspace::Simulate);
        }
        Verb::TypePurpose(text) => {
            state
                .workbench
                .stimulus_editor
                .set_field_text(&name, PURPOSE_FIELD, text);
        }
        Verb::CommitPurpose(text) => {
            state.workbench.stimulus_editor.clear_field();
            stimulus_actions::edit_field(state, PURPOSE_FIELD, &text);
        }
    }
}
