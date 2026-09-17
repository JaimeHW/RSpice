//! Linking a placed source to the project's stimulus library.
//!
//! Two directions, one transaction, one dialog. Adopting copies a definition
//! onto the instance; extracting publishes the instance's card as a definition
//! and points the instance at it. Both are one edit of one component against
//! one library, and both have to show the reader the same three things before
//! they commit — which instance, which card, and what the card becomes — so
//! splitting them would have meant two surfaces restating the same evidence
//! and drifting apart on it.
//!
//! Nothing here decides a lifecycle word or a refusal. `AdoptionFit`,
//! `kind_refusal`, `replace_warning`, `adopt_onto` and `extract_from` are the
//! model's, the card comes from the netlist generator through
//! `stimulus_realize`, and the curve is painted by
//! [`crate::properties::source_preview`] — the same painter the component
//! editor's evidence pane uses, because this dialog draws the same source.
//!
//! **Adopting across families re-places the instance.** Family is waveform
//! shape and it is spelled as a `ComponentType`, so a `SIN` definition on a
//! `PULSE` source is a new component type, symbol and property sheet. It is
//! performed as the narrowest possible re-type — the component's `kind` is
//! changed and the definition copied onto it in one component transaction —
//! rather than through the replace-instance machinery, which resolves a
//! library master and maps a pin contract that these 24 types do not have.
//! Every independent source shares one terminal geometry
//! (`component_type::geometry`: `+` at `(0, -20)`, `-` at `(0, 20)`), so both
//! terminals stay exactly where they were and every attached conductor keeps
//! its net; undo restores the old type and the old card together because the
//! whole thing is one entry.

use egui::{Align, Context, Layout, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::properties::source_preview;
use crate::simulation::stimulus_realize::{self, PreviewTiming};
use crate::state::Component;
use crate::state::stimulus_library::definition::{
    StimulusDefinition, StimulusDefinitionError, StimulusFamily,
};
use crate::state::stimulus_library::provenance::AdoptionFit;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::app::dialogs::review_primitives::{input_field, purpose_line};
use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;

/// The definition list's own column, wide enough for a name and its facts and
/// narrow enough to leave the preview a readable card.
const LIST_WIDTH: f32 = 396.0;
/// The two columns' shared height. Chosen so the whole surface, with its
/// purpose strip and footer, fits a 1024 x 640 viewport with the dialog's own
/// gutters — the smallest desktop geometry the shell gates.
const BODY_HEIGHT: f32 = 372.0;
/// One definition row.
const ROW_HEIGHT: f32 = 44.0;

const ADOPT_DESCRIPTION: &str = "Copy a library definition's card onto this instance and record which revision it came from. The instance keeps owning its card.";
const EXTRACT_DESCRIPTION: &str = "Publish this instance's card as a project stimulus definition. The card is unchanged; the instance becomes its first adopter.";

/// Which direction the dialog is open in.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum StimulusLinkMode {
    /// Copy a definition onto the instance.
    #[default]
    Adopt,
    /// Publish the instance's card as a definition.
    Extract,
}

/// One open link transaction over one placed source.
///
/// The instance's identity, card and nets are captured when the dialog opens
/// rather than resolved per frame: resolving the nets walks the sheet, and the
/// dialog is modal, so nothing can move underneath it while it is up.
#[derive(Debug, Clone, Default)]
pub(crate) struct StimulusLinkDialogState {
    pub(crate) open: bool,
    mode: StimulusLinkMode,
    component_id: u64,
    reference: String,
    chip: String,
    card: String,
    nets: [String; 2],
    timing: PreviewTiming,
    /// Adopt: the definition the reader has picked.
    pick: Option<String>,
    /// Extract: the name and purpose being authored.
    name: String,
    purpose: String,
    /// The refusal the last commit attempt produced.
    error: Option<String>,
}

impl StimulusLinkDialogState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// Whether one instance can be linked at all: an editable independent source.
pub(crate) fn stimulus_link_target(state: &AppState, component_id: u64) -> Option<&Component> {
    if state.schematic_edit_read_only() {
        return None;
    }
    state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .filter(|component| stimulus_realize::is_independent_source(component.kind))
}

/// Open the dialog over one placed source.
///
/// Returns the sentence to show the reader when it refuses, so the surface
/// that asked states it where they are looking.
pub(crate) fn open_stimulus_link(
    state: &mut AppState,
    component_id: u64,
    mode: StimulusLinkMode,
) -> Result<(), String> {
    let Some(component) = stimulus_link_target(state, component_id) else {
        return Err(
            "Select one independent source on an editable sheet to link it to the stimulus \
             library."
                .to_owned(),
        );
    };
    if mode == StimulusLinkMode::Adopt && state.workspace.stimulus_library.is_empty() {
        return Err(
            "This project holds no stimulus definitions yet. Save this source as one first, or \
             author one in the Stimulus Library workspace."
                .to_owned(),
        );
    }
    let component = component.clone();
    let nets = instance_nets(state, component_id);
    let card = stimulus_realize::source_card_text(&component, [nets[0].as_str(), nets[1].as_str()])
        .unwrap_or_else(|errors| errors.join("; "));
    let reference = component.spice_instance_name();
    let name = format!(
        "{}_{}",
        reference.to_lowercase(),
        family_stem(&component).unwrap_or("stimulus")
    );
    // The definition the instance already adopted, when the library still
    // holds it; otherwise the first one whose card this instance already
    // carries, which is what a reader saving and re-adopting almost always
    // means.
    let pick = component
        .stimulus_provenance
        .as_ref()
        .map(|provenance| provenance.definition.clone())
        .filter(|name| state.workspace.stimulus_library.get(name).is_some())
        .or_else(|| first_equal_definition(state, &component));

    let session = StimulusLinkDialogState {
        open: true,
        mode,
        component_id,
        chip: state
            .workspace
            .stimulus_library
            .provenance_state(&component)
            .label(),
        reference,
        card,
        nets,
        timing: crate::workbench::app::actions::property_edit::stimulus_preview_timing(state),
        pick,
        name,
        purpose: String::new(),
        error: None,
    };
    state.dialogs.stimulus_link = session;
    Ok(())
}

/// The definition an instance's own card already equals, if the library holds
/// one. It is the pick a reader almost always wants when nothing was adopted.
fn first_equal_definition(state: &AppState, component: &Component) -> Option<String> {
    state
        .workspace
        .stimulus_library
        .definitions()
        .iter()
        .find(|definition| {
            use crate::state::stimulus_library::definition::normalize_params;

            definition.adoption_fit(component) == AdoptionFit::Same
                && definition.value.trim() == component.value.trim()
                && normalize_params(&definition.params) == normalize_params(&component.params)
        })
        .map(|definition| definition.name().to_owned())
}

/// The family's name fragment, taken from the keyword the card carries so the
/// generated name and the card cannot disagree about what this source is.
fn family_stem(component: &Component) -> Option<&'static str> {
    Some(match StimulusFamily::of(component.kind)? {
        StimulusFamily::Dc => "dc",
        StimulusFamily::Ac => "ac",
        StimulusFamily::Pulse => "pulse",
        StimulusFamily::Sin => "sin",
        StimulusFamily::Pwl => "pwl",
        StimulusFamily::PwlFile => "pwl_file",
        StimulusFamily::Exp => "exp",
        StimulusFamily::Sffm => "sffm",
        StimulusFamily::Am => "am",
        StimulusFamily::Pat => "pat",
        StimulusFamily::Trnoise => "trnoise",
        StimulusFamily::Trrandom => "trrandom",
    })
}

/// The two nets this instance sits across, as the deck spells them.
///
/// Read through the same walk the Excitations page reads, so the card in the
/// dialog header is the card in the deck. A source whose terminals are not
/// both wired has no net names to show, and the detached pair the preview
/// bridge uses stands in — the card's shape is what the reader is here for.
fn instance_nets(state: &AppState, component_id: u64) -> [String; 2] {
    let detached = || {
        [
            stimulus_realize::DETACHED_NETS[0].to_owned(),
            stimulus_realize::DETACHED_NETS[1].to_owned(),
        ]
    };
    let sources = crate::simulation::placed_sources::placed_sources(
        &state.schematic,
        &state.workspace.stimulus_library,
        None,
    );
    let Some(source) = sources
        .iter()
        .find(|source| source.component_id == component_id)
    else {
        return detached();
    };
    match source.nets.as_slice() {
        [positive, negative, ..] if !positive.is_empty() && !negative.is_empty() => {
            [positive.clone(), negative.clone()]
        }
        _ => detached(),
    }
}

/// One row of the adoption list.
struct AdoptRow {
    name: String,
    revision: u32,
    fit: AdoptionFit,
    /// `r2 · V · SIN · 1 kHz`, plus what adopting it would do to the instance.
    detail: String,
    /// Why this definition cannot be adopted onto this instance at all.
    refusal: Option<String>,
}

/// Every definition, in the order a reader should consider them: the ones that
/// are a plain copy, then the ones that re-place the instance, then the ones
/// that cannot be adopted at all; within each, the library's own family order
/// and then the name.
fn adopt_rows(state: &AppState, component: &Component) -> Vec<AdoptRow> {
    let mut rows: Vec<(usize, usize, String, AdoptRow)> = state
        .workspace
        .stimulus_library
        .definitions()
        .iter()
        .map(|definition| {
            let fit = definition.adoption_fit(component);
            let identity = crate::simulation::placed_sources::source_identity_line(
                &definition.transient_component(),
            )
            .unwrap_or_else(|| definition.family().label().to_owned());
            let detail = match fit {
                AdoptionFit::Same => format!("r{} \u{00b7} {identity}", definition.revision()),
                AdoptionFit::Replace { .. } => format!(
                    "r{} \u{00b7} {identity} \u{00b7} re-places the instance",
                    definition.revision()
                ),
                AdoptionFit::Kind => format!("r{} \u{00b7} {identity}", definition.revision()),
            };
            let order = match fit {
                AdoptionFit::Same => 0,
                AdoptionFit::Replace { .. } => 1,
                AdoptionFit::Kind => 2,
            };
            let family = StimulusFamily::ALL
                .iter()
                .position(|family| *family == definition.family())
                .unwrap_or(StimulusFamily::ALL.len());
            (
                order,
                family,
                definition.name().to_ascii_lowercase(),
                AdoptRow {
                    name: definition.name().to_owned(),
                    revision: definition.revision(),
                    fit,
                    detail,
                    refusal: (fit == AdoptionFit::Kind).then(|| definition.kind_refusal(component)),
                },
            )
        })
        .collect();
    rows.sort_by(|left, right| (left.0, left.1, &left.2).cmp(&(right.0, right.1, &right.2)));
    rows.into_iter().map(|(_, _, _, row)| row).collect()
}

/// The instance as it would be once `definition` is on it, including the
/// re-type a family change performs. `None` when the definition cannot be
/// adopted onto it at all.
fn realized(component: &Component, definition: &StimulusDefinition) -> Option<Component> {
    if definition.adoption_fit(component) == AdoptionFit::Kind {
        return None;
    }
    let mut candidate = component.clone();
    candidate.kind = definition.component_type();
    definition.adopt_onto(&mut candidate).ok()?;
    Some(candidate)
}

/// What confirming would do, in one sentence the reader can check.
fn consequence(component: &Component, definition: &StimulusDefinition) -> String {
    match definition.adoption_fit(component) {
        AdoptionFit::Same => format!(
            "Copies {} r{} onto {}: the bias, the AC layer and every waveform field become the \
             definition's. The instance keeps its name, position and nets, and records the \
             revision it copied.",
            definition.name(),
            definition.revision(),
            component.spice_instance_name()
        ),
        fit @ AdoptionFit::Replace { .. } => fit
            .replace_warning()
            .unwrap_or_else(|| definition.kind_refusal(component)),
        AdoptionFit::Kind => definition.kind_refusal(component),
    }
}

/// Render the open link transaction.
pub(crate) fn render_stimulus_link_dialog(ctx: &Context, state: &mut AppState) {
    if !state.dialogs.stimulus_link.open {
        return;
    }
    let session = state.dialogs.stimulus_link.clone();
    let Some(component) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == session.component_id)
        .cloned()
    else {
        state.dialogs.stimulus_link.close();
        return;
    };

    match session.mode {
        StimulusLinkMode::Adopt => render_adopt(ctx, state, &session, &component),
        StimulusLinkMode::Extract => render_extract(ctx, state, &session, &component),
    }
}

fn render_adopt(
    ctx: &Context,
    state: &mut AppState,
    session: &StimulusLinkDialogState,
    component: &Component,
) {
    let rows = adopt_rows(state, component);
    let picked = session.pick.as_ref().and_then(|name| {
        state
            .workspace
            .stimulus_library
            .get(name)
            .filter(|definition| definition.adoption_fit(component) != AdoptionFit::Kind)
            .cloned()
    });
    let replaces = picked.as_ref().is_some_and(|definition| {
        matches!(
            definition.adoption_fit(component),
            AdoptionFit::Replace { .. }
        )
    });
    let primary = if replaces {
        "Replace and adopt"
    } else {
        "Adopt definition"
    };
    let mut dialog = Dialog::new(
        "STIMULUS LIBRARY \u{00b7} ADOPT",
        format!("Adopt a stimulus definition \u{00b7} {}", session.reference),
        primary,
    )
    .description(ADOPT_DESCRIPTION)
    .size(DialogSize::WideWorkflow)
    .initial_height(BODY_HEIGHT + 58.0)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(picked.is_some())
    .initial_focus(DialogInitialFocus::BodyControl);
    if replaces {
        dialog = dialog.destructive();
    }
    if let Some(error) = session.error.as_deref() {
        dialog = dialog.hint(error);
    }

    let mut pick = session.pick.clone();
    let mut focus = None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        purpose_line(ui, ADOPT_DESCRIPTION);
        instance_header(ui, session);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(
                vec2(LIST_WIDTH, BODY_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(LIST_WIDTH, BODY_HEIGHT));
                    egui::ScrollArea::vertical()
                        .id_salt("stimulus-link-definitions")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for row in &rows {
                                let selected = pick.as_deref() == Some(row.name.as_str());
                                let response = definition_row(ui, row, selected);
                                if focus.is_none() && selected {
                                    focus = Some(response.id);
                                }
                                if response.clicked() {
                                    pick = Some(row.name.clone());
                                }
                            }
                        });
                },
            );
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), BODY_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(ui.available_width(), BODY_HEIGHT));
                    adoption_preview(ui, session, component, picked.as_ref());
                },
            );
        });
        focus
    });

    state.dialogs.stimulus_link.pick = pick;
    match choice {
        DialogChoice::Primary => {
            let Some(definition) = picked else { return };
            match commit_adoption(state, session.component_id, &definition) {
                Ok(line) => {
                    state.push_user_message(ConsoleMessage::info(line));
                    state.dialogs.stimulus_link.close();
                }
                Err(refusal) => state.dialogs.stimulus_link.error = Some(refusal),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => state.dialogs.stimulus_link.close(),
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn render_extract(
    ctx: &Context,
    state: &mut AppState,
    session: &StimulusLinkDialogState,
    component: &Component,
) {
    let refusal = extract_refusal(state, component, &session.name);
    let dialog = Dialog::new(
        "STIMULUS LIBRARY \u{00b7} SAVE",
        "Save as library definition",
        "Save definition",
    )
    .description(EXTRACT_DESCRIPTION)
    .size(DialogSize::WideWorkflow)
    .initial_height(BODY_HEIGHT + 58.0)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(refusal.is_none())
    .initial_focus(DialogInitialFocus::BodyControl);

    let mut name = session.name.clone();
    let mut purpose = session.purpose.clone();
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        purpose_line(ui, EXTRACT_DESCRIPTION);
        instance_header(ui, session);
        let mut focus = None;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(
                vec2(LIST_WIDTH, BODY_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(LIST_WIDTH, BODY_HEIGHT));
                    egui::Frame::NONE
                        .inner_margin(egui::Margin::same(12))
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 10.0;
                            focus = Some(
                                input_field(
                                    ui,
                                    "Definition name",
                                    &mut name,
                                    "one unquoted SPICE identifier",
                                    refusal.as_deref(),
                                    "The name this definition is listed and placed under.",
                                )
                                .id,
                            );
                            input_field(
                                ui,
                                "Purpose",
                                &mut purpose,
                                "what this stimulus is for",
                                None,
                                "Shown beside the definition wherever it is offered.",
                            );
                            fact_rows(
                                ui,
                                &[
                                    ("Family", family_line(component)),
                                    ("Saved as", "r1 · project document".to_owned()),
                                    (
                                        "Afterwards",
                                        format!(
                                            "{} · adopted · r1 · card unchanged",
                                            session.reference
                                        ),
                                    ),
                                ],
                            );
                        });
                },
            );
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), BODY_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(ui.available_width(), BODY_HEIGHT));
                    preview_column(
                        ui,
                        "The card this definition publishes",
                        &session.card,
                        component,
                        session.timing,
                        "Every adopter of this definition realizes exactly this card. Saving it \
                         does not change the instance it came from.",
                    );
                },
            );
        });
        focus
    });

    state.dialogs.stimulus_link.name = name;
    state.dialogs.stimulus_link.purpose = purpose;
    match choice {
        DialogChoice::Primary => match commit_extraction(state, session.component_id) {
            Ok(line) => {
                state.push_user_message(ConsoleMessage::info(line));
                state.dialogs.stimulus_link.close();
            }
            Err(refusal) => state.dialogs.stimulus_link.error = Some(refusal),
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => state.dialogs.stimulus_link.close(),
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

/// Why this name cannot be saved, in the model's own words.
fn extract_refusal(state: &AppState, component: &Component, name: &str) -> Option<String> {
    if let Err(error) = StimulusDefinition::new(name, component.kind) {
        return Some(error.to_string());
    }
    state
        .workspace
        .stimulus_library
        .get(name)
        .map(|held| StimulusDefinitionError::DuplicateName(held.name().to_owned()).to_string())
}

/// `V · SIN` — the two facts a definition's identity is made of.
fn family_line(component: &Component) -> String {
    match (
        StimulusFamily::of(component.kind),
        crate::state::stimulus_library::definition::StimulusKind::of(component.kind),
    ) {
        (Some(family), Some(kind)) => format!("{} \u{00b7} {}", family.label(), kind.word()),
        _ => component.kind.display_name().to_owned(),
    }
}

/// The instance this transaction is about: what it is called, what it says
/// now, and where it stands with the library.
fn instance_header(ui: &mut Ui, session: &StimulusLinkDialogState) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel_2)
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width() - 24.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.label(
                    egui::RichText::new(&session.reference)
                        .font(theme::mono(tokens::FS_1, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(&session.card)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    )
                    .truncate(),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(&session.chip)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                });
            });
        });
}

/// One definition row: the family mark, the name, and what adopting it does.
///
/// A definition of the other quantity stays listed and is drawn disabled with
/// the model's refusal on it, because the authored domain is the point: a list
/// that silently dropped every current definition would teach a reader that
/// the project has none.
fn definition_row(ui: &mut Ui, row: &AdoptRow, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let enabled = row.refusal.is_none();
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), ROW_HEIGHT),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let announcement = match row.refusal.as_deref() {
        Some(refusal) => format!("{} · r{} · {refusal}", row.name, row.revision),
        None => format!("{} · {}", row.name, row.detail),
    };
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            enabled,
            selected,
            &announcement,
        )
    });
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
    } else if response.hovered() && enabled {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let text = if enabled {
        t.color.text
    } else {
        t.color.text_faint
    };
    ui.painter().text(
        egui::pos2(rect.left() + 12.0, rect.top() + 11.0),
        egui::Align2::LEFT_CENTER,
        &row.name,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        text,
    );
    let detail_colour = match row.fit {
        AdoptionFit::Replace { .. } => t.color.warn,
        _ if !enabled => t.color.text_faint,
        _ => t.color.text_dim,
    };
    ui.painter().text(
        egui::pos2(rect.left() + 12.0, rect.bottom() - 12.0),
        egui::Align2::LEFT_CENTER,
        &row.detail,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        detail_colour,
    );
    theme::paint_focus_ring(ui, &response, rect);
    match row.refusal.as_deref() {
        Some(refusal) => response.on_hover_text(refusal),
        None => response,
    }
}

/// The right-hand column of the adopt mode: the card the instance would carry
/// afterwards, the engine's drawing of it, and the consequence sentence.
fn adoption_preview(
    ui: &mut Ui,
    session: &StimulusLinkDialogState,
    component: &Component,
    picked: Option<&StimulusDefinition>,
) {
    let Some(definition) = picked else {
        column_note(
            ui,
            "Pick a definition to see the card it would leave on this instance.",
        );
        return;
    };
    let Some(candidate) = realized(component, definition) else {
        column_note(ui, &definition.kind_refusal(component));
        return;
    };
    let card = stimulus_realize::source_card_text(
        &candidate,
        [session.nets[0].as_str(), session.nets[1].as_str()],
    )
    .unwrap_or_else(|errors| errors.join("; "));
    preview_column(
        ui,
        "The card after adoption",
        &card,
        &candidate,
        session.timing,
        &consequence(component, definition),
    );
}

/// A card, its engine-evaluated curve, and the sentence that explains them.
fn preview_column(
    ui: &mut Ui,
    heading: &str,
    card: &str,
    component: &Component,
    timing: PreviewTiming,
    sentence: &str,
) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.set_width(ui.available_width() - 24.0);
            ui.spacing_mut().item_spacing.y = 8.0;
            ui.label(
                egui::RichText::new(heading)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(card)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text),
                )
                .wrap(),
            );
            let curve = source_preview::source_curve(component, timing);
            source_preview::paint_source_preview(ui, &curve, timing);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(sentence)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

fn column_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.set_width(ui.available_width() - 24.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

fn fact_rows(ui: &mut Ui, facts: &[(&str, String)]) {
    let t = Tokens::get(ui.ctx());
    for (label, value) in facts {
        ui.horizontal(|ui| {
            ui.set_min_height(19.0);
            ui.label(
                egui::RichText::new(*label)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(value)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text),
                    )
                    .truncate(),
                );
            });
        });
    }
}

/// Copy one definition onto the instance, re-typing it first when the family
/// differs. One component transaction, so one undo restores the type and the
/// card together.
pub(crate) fn commit_adoption(
    state: &mut AppState,
    component_id: u64,
    definition: &StimulusDefinition,
) -> Result<String, String> {
    let expected = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "That instance is no longer on this sheet.".to_owned())?;
    let re_placed = match definition.adoption_fit(&expected) {
        AdoptionFit::Same => false,
        AdoptionFit::Replace { .. } => true,
        AdoptionFit::Kind => return Err(definition.kind_refusal(&expected)),
    };
    let candidate =
        realized(&expected, definition).ok_or_else(|| definition.kind_refusal(&expected))?;
    let reference = candidate.spice_instance_name();
    if re_placed {
        // Not a property edit, and the property-edit transaction says so:
        // `prepare_component_edit` refuses any candidate whose type differs
        // from the one on the sheet, because a typed parameter commit must
        // never be able to change what a component *is*. A family change is
        // exactly that change, so it is written as its own operation — one
        // undo entry, the instance replaced in place, and the topology version
        // bumped because the netlister must re-read the card.
        if state.schematic_edit_read_only() {
            return Err(format!(
                "{reference} was not re-placed: the active schematic view is read-only."
            ));
        }
        let description = format!("adopt {} onto {reference}", definition.name());
        let replaced = state.schematic.with_undo(description, |schematic| {
            if let Some(held) = schematic
                .components
                .iter_mut()
                .find(|component| component.id == component_id)
            {
                *held = candidate;
            }
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        });
        if !replaced {
            return Err(format!(
                "{reference} was not re-placed: the active schematic view is read-only."
            ));
        }
    } else {
        state.edit_component_transaction(&expected, candidate, "adopt stimulus definition")?;
    }
    Ok(if re_placed {
        format!(
            "{reference} was re-placed as {} and adopted {} r{}; both terminals kept their nets.",
            definition.component_type().display_name(),
            definition.name(),
            definition.revision(),
        )
    } else {
        format!(
            "{reference} adopted {} r{}.",
            definition.name(),
            definition.revision(),
        )
    })
}

/// Copy the library's current revision of `definition` back onto one instance
/// that already adopted it.
///
/// Narrower than [`commit_adoption`] on purpose, because the model is: an
/// instance naming some other definition is adopting, not re-adopting, and
/// `readopt_onto` is where that refusal is written. Every surface that offers
/// the verb — the Studio's Excitations rows, the component editor's evidence
/// pane, the schematic's own menu — comes through here, so one re-adoption is
/// one transaction and one console sentence wherever it is asked for.
pub(crate) fn commit_readoption(
    state: &mut AppState,
    component_id: u64,
    definition: &str,
) -> Result<String, String> {
    let held = state
        .workspace
        .stimulus_library
        .get(definition)
        .cloned()
        .ok_or_else(|| format!("This project defines no '{definition}' to re-adopt."))?;
    let expected = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "That instance is no longer on this sheet.".to_owned())?;
    let mut candidate = expected.clone();
    held.readopt_onto(&mut candidate)?;
    let reference = candidate.spice_instance_name();
    state.edit_component_transaction(&expected, candidate, "re-adopt stimulus definition")?;
    Ok(format!(
        "{reference} re-adopted {definition} r{}; its card is the library's again.",
        held.revision()
    ))
}

/// Publish the instance's card as a new `r1` definition and point the instance
/// at it.
fn commit_extraction(state: &mut AppState, component_id: u64) -> Result<String, String> {
    let session = state.dialogs.stimulus_link.clone();
    let expected = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "That instance is no longer on this sheet.".to_owned())?;
    if let Some(refusal) = extract_refusal(state, &expected, &session.name) {
        return Err(refusal);
    }
    let mut candidate = expected.clone();
    let definition =
        StimulusDefinition::extract_from(&mut candidate, &session.name, &session.purpose)
            .map_err(|error| error.to_string())?;
    let name = definition.name().to_owned();
    state.edit_component_transaction(&expected, candidate, "save stimulus definition")?;
    state
        .workspace
        .stimulus_library
        .insert(definition)
        .map_err(|error| error.to_string())?;
    state.workbench.selected_stimulus_definition = Some(name.clone());
    Ok(format!(
        "{} saved {name} as r1 in this project's stimulus library and now reads adopted \u{00b7} \
         r1.",
        session.reference
    ))
}

/// Open the Stimulus Library workspace on one definition.
pub(crate) fn open_stimulus_definition(state: &mut AppState, name: &str) {
    state.workbench.selected_stimulus_definition = Some(name.to_owned());
    state.workbench.activate(Workspace::Stimulus);
}

#[cfg(test)]
mod tests;
