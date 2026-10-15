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
use crate::state::stimulus_library::definition::StimulusDefinition;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app::actions::stimulus::{self as stimulus_actions, StimulusAdopter};
use crate::workbench::state::{PURPOSE_FIELD, SimulationPage, Workspace};
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{
    card_well, elide_text, panel_bleed, property_row, property_row_input_with_hint,
    property_row_wrapped,
};
use super::{muted_inspector_copy, section_header};

/// The line that names one adopter, and the air an entry keeps either side of
/// the rule that separates it from the next.
const NAME_ROW_HEIGHT: f32 = 20.0;
const ENTRY_FOOT: f32 = 7.0;
/// Air between the name line and the card under it.
const CARD_GAP: f32 = 3.0;
/// The verb row under an adopter's card: how tall one verb is, the air between
/// two of them, and the air above the row.
///
/// egui gives the row the theme's interact height rather than its content's,
/// so the region is four points taller than the verbs in it and the air under
/// the row measures four more than [`ENTRY_FOOT`] says. Measured at the dock's
/// 312 pt width: name line 67..87, card 90..108, verbs 116..136, rule at 147,
/// next entry at 155.
const ACTION_HEIGHT: f32 = 20.0;
const ACTION_GAP: f32 = 4.0;
const ACTION_GAP_Y: f32 = 4.0;
/// How far a verb's hover fill reaches past its words, which is what the kit's
/// own header actions keep.
const ACTION_PAD: f32 = 6.0;
/// Least air between the instance an entry names and the state it is in.
const CHIP_GAP: f32 = 8.0;

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
    let library = &state.workspace.content.stimulus_library;
    let Some(definition) = state
        .workbench
        .selected_stimulus_definition
        .as_deref()
        .and_then(|name| library.get(name))
        .cloned()
    else {
        // Nothing selected is not a subject. A section headed to say so,
        // over a count the browser's own band prints, is a panel of two rows
        // about the absence of one.
        return;
    };

    let adopters = stimulus_actions::design_adopters(state, definition.name());
    let mut verbs = Vec::new();
    adopters_section(ui, state, &adopters, &definition, &mut verbs);
    if consumers_section(ui, state, &adopters) {
        verbs.push(Verb::OpenPlanReaders);
    }
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
        muted_inspector_copy(ui, &messages.text(MessageId::StimulusAdoptersNone));
        return;
    }
    for (index, adopter) in adopters.iter().enumerate() {
        if index > 0 {
            adopter_rule(ui);
        }
        adopter_entry(ui, state, adopter, definition, index, verbs);
    }
}

/// The hairline between two adopters.
///
/// A rule across the panel, not a box around each instance: the section is one
/// list of one kind of thing, so one edge between two entries says what four
/// edges around each of them said — and says it in the register the panel's
/// own section headers are ruled in, rather than in a second one.
fn adopter_rule(ui: &mut Ui) {
    let palette = Tokens::get(ui.ctx()).color;
    ui.add_space(ENTRY_FOOT);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(
        panel_bleed(ui, rect).x_range(),
        rect.center().y,
        egui::Stroke::new(1.0, palette.border),
    );
    ui.add_space(ENTRY_FOOT);
}

/// One adopter: which instance it is, where the run reaches it, the card it
/// really emits, and what can be done to it from here.
///
/// Three lines on the panel's own inset, with no container. A box around each
/// instance drew four edges to say what the rule between two of them says, and
/// charged every entry two insets, a corner radius and a second surface tone
/// for it — which is what made a list of two instances read as a feed of two
/// cards rather than as a section of the inspector.
fn adopter_entry(
    ui: &mut Ui,
    state: &AppState,
    adopter: &StimulusAdopter,
    definition: &StimulusDefinition,
    index: usize,
    verbs: &mut Vec<Verb>,
) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    // The chip is the decision to re-adopt: an instance in step with the
    // library states nothing, and a drift is a fact rather than a tone — the
    // Re-adopt verb under it is what acts on it.
    let chip = (adopter.behind || adopter.modified).then_some(adopter.chip.as_str());
    // The occurrence is where the run reaches this instance; at the design
    // root "/" is not a place the reader has to be told about.
    let occurrence = (adopter.occurrence != "/").then_some(adopter.occurrence.as_str());
    // The instance, where the run reaches it and where it stands share one
    // line: three registers of one fact, and a reader comparing two entries
    // reads down one column for each of them.
    let (row, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), NAME_ROW_HEIGHT),
        Sense::hover(),
    );
    let spoken = std::iter::once(adopter.source.reference.as_str())
        .chain(occurrence)
        .chain(chip)
        .collect::<Vec<_>>()
        .join(" \u{b7} ");
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &spoken));
    let reference = ui.painter().layout_no_wrap(
        adopter.source.reference.clone(),
        theme::mono(tokens::FS_1, FontWeight::Medium),
        palette.text,
    );
    let reference_width = reference.size().x;
    ui.painter().galley(
        egui::pos2(row.left(), row.center().y - reference.size().y * 0.5),
        reference,
        palette.text,
    );
    let micro = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let mut spent = reference_width;
    if let Some(occurrence) = occurrence {
        let galley =
            ui.painter()
                .layout_no_wrap(occurrence.to_owned(), micro.clone(), palette.text_faint);
        ui.painter().galley(
            egui::pos2(
                row.left() + reference_width + 8.0,
                row.center().y - galley.size().y * 0.5,
            ),
            galley.clone(),
            palette.text_faint,
        );
        spent += 8.0 + galley.size().x;
    }
    // The chip ends where the panel's rows end, so two entries state their
    // drift in one column rather than each against its own box.
    let mut shortened = None;
    if let Some(chip) = chip {
        let shown = elide_text(ui, chip, &micro, (row.width() - spent - CHIP_GAP).max(0.0));
        ui.painter().text(
            row.right_center(),
            egui::Align2::RIGHT_CENTER,
            &shown,
            micro,
            palette.text_dim,
        );
        shortened = (shown != chip).then(|| chip.to_owned());
    }
    // One hover, whichever the line most needs to say: an adopter drawn in
    // another occurrence carries one of its three verbs and says why here,
    // rather than spending two wrapped lines of the panel on it for every such
    // instance; otherwise a chip the line had to shorten states itself whole.
    let hover = if adopter.on_this_sheet {
        shortened
    } else {
        Some(messages.text(MessageId::StimulusAdopterElsewhere))
    };
    if let Some(hover) = hover {
        response.on_hover_text(hover);
    }

    ui.add_space(CARD_GAP);
    card_well(
        ui,
        adopter.source.card.as_deref().map_err(String::as_str),
        0.0,
    );
    adopter_verbs(ui, state, adopter, definition, index, verbs);
}

/// The verbs on one adopter, under the card they act on.
///
/// Each is as wide as its own words and they run from the panel's inset, so
/// the first verb is at one x on every entry. Stretched across the panel in
/// equal thirds — which is what the boxed card did — the same verb was a
/// different width and a different place on every entry, and a two-verb
/// adopter's buttons came out half as wide again as a three-verb one's.
///
/// Every verb but the first addresses the instance by a component id, which
/// means something only inside the buffer on screen. An adopter drawn in
/// another occurrence is therefore offered the one verb that reaches it, and
/// carries no control that would edit whichever instance of the open sheet
/// happened to share that id.
fn adopter_verbs(
    ui: &mut Ui,
    state: &AppState,
    adopter: &StimulusAdopter,
    definition: &StimulusDefinition,
    index: usize,
    verbs: &mut Vec<Verb>,
) {
    let messages = state.ui.messages();
    ui.add_space(ACTION_GAP_Y);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = ACTION_GAP;
        if action(ui, &messages.text(MessageId::StimulusShowInSchematic)) {
            verbs.push(Verb::Reveal(index));
        }
        if !adopter.on_this_sheet {
            return;
        }
        if action(ui, &messages.text(MessageId::StimulusOpenPropertiesShort)) {
            verbs.push(Verb::Properties(index));
        }
        if adopter.offers_readoption
            && action(
                ui,
                &messages.format(
                    MessageId::StimulusReadopt,
                    &[("revision", &definition.revision().to_string())],
                ),
            )
        {
            verbs.push(Verb::Readopt(index));
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
    // No name in the header's metadata: the browser's selected row and the
    // stage's own title both name the definition this panel is about.
    section_header(
        ui,
        &messages.text(MessageId::StimulusDefinitionSection),
        None,
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
    // Owner and Stored were two rows that never varied: every definition is
    // the project's, and every one of them is in the project document.
}

/// One verb on an adopter, at the width of its own words.
///
/// Quiet until it is under the pointer, which is how every other verb the kit
/// hangs off a panel row behaves: three outlined boxes under each entry are
/// three more containers in a section whose whole fault was containers, and a
/// verb that is permanently drawn as a box competes with the card above it for
/// the entry's one piece of emphasis.
fn action(ui: &mut Ui, label: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, t.color.text_dim);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(galley.size().x + 2.0 * ACTION_PAD, ACTION_HEIGHT),
        Sense::click(),
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    let hovered = ui.is_enabled() && response.hovered();
    if hovered {
        ui.painter().rect(
            rect,
            t.radius,
            t.color.bg_hover,
            egui::Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
    }
    let ink = if hovered {
        t.color.text
    } else {
        t.color.text_dim
    };
    ui.painter().galley(
        egui::pos2(
            rect.left() + ACTION_PAD,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        ink,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    response.clicked()
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
