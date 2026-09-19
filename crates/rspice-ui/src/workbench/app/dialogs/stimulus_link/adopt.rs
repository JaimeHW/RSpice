//! Adopting a library definition onto one placed source.
//!
//! The left pane is the whole library, grouped by what adopting each definition
//! would *do* to this instance rather than by what it is: a plain copy, a
//! re-place, or a refusal. That grouping is the reason the list is worth
//! reading — a name and a family alone leave the reader to work the consequence
//! out twenty-four times — and it comes from `AdoptionFit`, which is the
//! model's answer rather than a second reading of it here.
//!
//! The right pane answers the one question the left cannot: what the deck line
//! becomes. It stacks the instance's current waveform over the waveform it
//! would carry, on one shared time axis and each on its own y range. Overlaid,
//! a 5 V pulse flattens a 3 mV sine into the axis.

use egui::{Key, Modifiers, Rect, Sense, Ui, vec2};

use crate::properties::source_preview;
use crate::simulation::placed_sources;
use crate::simulation::stimulus_realize::{self, WaveformTrace};
use crate::state::Component;
use crate::state::stimulus_library::definition::{StimulusDefinition, StimulusFamily};
use crate::state::stimulus_library::provenance::AdoptionFit;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogSize};
use crate::workbench::app_state::AppState;

use super::shell::{self, Fact, MiniCache};
use super::{StimulusLinkDialogState, consequence, realized};

/// The definition list's pane, and the width it falls back to when the dialog
/// itself has been narrowed to the viewport.
const LIST_WIDTH: f32 = 300.0;
const NARROW_LIST_WIDTH: f32 = 260.0;
/// Below this body width the list gives up its forty points first: a strip
/// loses more by narrowing than a column of names does.
const NARROW_BODY: f32 = 840.0;

/// One definition row, and the group label above a run of them.
const ROW_HEIGHT: f32 = 36.0;
const GROUP_LABEL_HEIGHT: f32 = 24.0;

/// The body's own bounds. Below the minimum a two-pane dialog reads as a strip
/// of controls; above the maximum the list has outgrown a modal and belongs in
/// the Stimulus Library workspace.
const MIN_BODY: f32 = 300.0;
const MAX_BODY: f32 = 460.0;

/// What the modal is for, for a reader who arrives at it through the
/// accessibility tree rather than through the schematic.
const PURPOSE: &str =
    "Copy a stimulus definition's card onto this instance and record the revision it came from";

/// One definition as the list states it.
pub(super) struct AdoptRow<'a> {
    pub(super) name: String,
    pub(super) revision: u32,
    /// What the mini shows if this definition has no curve.
    pub(super) gap: source_preview::MiniGap,
    /// The dim line under the name, in the order a narrow row should give them
    /// up: the family when its group does not already say it, the key figure,
    /// then the levels. Held apart rather than joined so a row too narrow for
    /// all three drops the last one whole instead of eliding a number.
    pub(super) meta: Vec<String>,
    /// Why this definition cannot be adopted onto this instance at all.
    pub(super) refusal: Option<String>,
    /// The saved waveform, held by the session rather than evaluated here.
    curve: &'a Result<WaveformTrace, String>,
}

/// One run of rows under one label.
pub(super) struct AdoptGroup<'a> {
    pub(super) label: String,
    pub(super) rows: Vec<AdoptRow<'a>>,
}

/// Every definition the library holds, grouped by what adopting it would do and
/// narrowed by the filter.
///
/// An empty group is absent rather than headed by nothing: a project with no
/// current sources should not be told twice that it has none.
pub(super) fn adopt_groups<'a>(
    state: &AppState,
    component: &Component,
    filter: &str,
    cache: &'a MiniCache,
) -> Vec<AdoptGroup<'a>> {
    let reference = component.spice_instance_name();
    let unit = placed_sources::source_unit(component);
    let needle = filter.trim().to_ascii_lowercase();
    let mut ordered: Vec<&StimulusDefinition> = state
        .workspace
        .stimulus_library
        .definitions()
        .iter()
        .filter(|definition| matches(definition, &needle))
        .collect();
    ordered.sort_by_key(|definition| {
        (
            StimulusFamily::ALL
                .iter()
                .position(|family| *family == definition.family())
                .unwrap_or(StimulusFamily::ALL.len()),
            definition.name().to_ascii_lowercase(),
        )
    });

    let mut same = Vec::new();
    let mut replace = Vec::new();
    let mut refused = Vec::new();
    for definition in ordered {
        let fit = definition.adoption_fit(component);
        let curve = cache.get(&(definition.name().to_owned(), definition.revision()));
        let row = AdoptRow {
            name: definition.name().to_owned(),
            revision: definition.revision(),
            gap: source_preview::MiniGap::of(definition),
            meta: meta_line(definition, fit == AdoptionFit::Same, unit, curve),
            refusal: (fit == AdoptionFit::Kind).then(|| definition.kind_refusal(component)),
            curve: curve.unwrap_or(&source_preview::NO_MINI),
        };
        match fit {
            AdoptionFit::Same => same.push(row),
            AdoptionFit::Replace { .. } => replace.push(row),
            AdoptionFit::Kind => refused.push(row),
        }
    }

    let family = StimulusFamily::of(component.kind).map_or("this family", StimulusFamily::label);
    let other_quantity = if unit == "V" {
        "Current sources"
    } else {
        "Voltage sources"
    };
    [
        (
            format!("Same family \u{00b7} {family} \u{00b7} copies parameters"),
            same,
        ),
        (
            format!("Other families \u{00b7} re-places {reference}"),
            replace,
        ),
        (
            format!("{other_quantity} \u{00b7} cannot drive {reference}"),
            refused,
        ),
    ]
    .into_iter()
    .filter(|(_, rows)| !rows.is_empty())
    .map(|(label, rows)| AdoptGroup { label, rows })
    .collect()
}

/// Whether one definition answers the filter: its name, the family keyword its
/// card carries, or the purpose it was saved with.
fn matches(definition: &StimulusDefinition, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    definition.name().to_ascii_lowercase().contains(needle)
        || definition
            .family()
            .label()
            .to_ascii_lowercase()
            .contains(needle)
        || definition.purpose.to_ascii_lowercase().contains(needle)
}

/// The dim line under a row's name.
///
/// The key figure is [`placed_sources::source_key_figure`]'s, the levels are
/// read off the mini's own trace through [`WaveformTrace::readouts`], and the family
/// is only said when the group above the row does not already say it. Nothing
/// here parses a parameter string: a row that did would disagree with the mini
/// beside it the first time the engine substituted an omitted field.
fn meta_line(
    definition: &StimulusDefinition,
    in_family_group: bool,
    unit: &str,
    curve: Option<&Result<WaveformTrace, String>>,
) -> Vec<String> {
    let mut parts = Vec::new();
    if !in_family_group {
        parts.push(definition.family().label().to_owned());
    }
    let figure = placed_sources::source_key_figure_spelled(
        &definition.transient_component(),
        shell::display_spelling,
    );
    if !figure.is_empty() {
        parts.push(figure);
    }
    // A flat curve's two rails are the one level the key figure already states,
    // so saying it again would spend the row's narrowest column on nothing.
    if let Some(readouts) = curve
        .and_then(|curve| curve.as_ref().ok())
        .and_then(WaveformTrace::readouts)
        .filter(|readouts| readouts.span() > 0.0)
    {
        parts.push(source_preview::level_span_label(readouts, unit));
    }
    parts
}

/// Render the adopt transaction.
pub(super) fn render(
    ctx: &egui::Context,
    state: &mut AppState,
    session: &StimulusLinkDialogState,
    component: &Component,
    cache: &mut MiniCache,
) {
    source_preview::ensure_minis(cache, &state.workspace.stimulus_library, session.timing);
    let groups = adopt_groups(state, component, &session.filter, cache);
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
    let after = picked
        .as_ref()
        .and_then(|definition| realized(component, definition));
    let after_card = after.as_ref().map(|candidate| {
        stimulus_realize::source_card_text(
            candidate,
            [session.nets[0].as_str(), session.nets[1].as_str()],
        )
        .unwrap_or_else(|errors| errors.join("; "))
    });
    let note = session.error.clone().unwrap_or_else(|| match &picked {
        Some(definition) => consequence(component, definition),
        None => format!("Pick a definition to copy onto {}.", session.reference),
    });
    let tone = if session.error.is_some() {
        DialogHintTone::Error
    } else if replaces {
        DialogHintTone::Warn
    } else {
        DialogHintTone::Neutral
    };
    let title = format!("Adopt a definition onto {}", session.reference);
    let body_height = body_height(picked.is_some(), after_card.as_deref(), &session.card);

    let dialog = Dialog::new(
        "STIMULUS LIBRARY",
        title.clone(),
        if replaces {
            "Re-place and adopt"
        } else {
            "Adopt"
        },
    )
    .description(PURPOSE)
    .size(DialogSize::StimulusLink)
    .without_header()
    .flush_body()
    .manual_body_scroll()
    .ghost("Cancel")
    .primary_enabled(picked.is_some())
    .hint(note)
    .hint_tone(tone)
    .initial_focus(DialogInitialFocus::BodyControl);

    let mut pick = session.pick.clone();
    let mut filter = session.filter.clone();
    let mut closed = false;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        closed = shell::header(
            ui,
            &shell::Identity {
                eyebrow: "STIMULUS LIBRARY",
                title: &title,
                card: &session.card,
                chip: &session.chip,
                provenance: session.provenance,
            },
        );
        let list_width = if ui.available_width() < NARROW_BODY {
            NARROW_LIST_WIDTH
        } else {
            LIST_WIDTH
        };
        let mut focus = None;
        shell::panes(
            ui,
            list_width,
            body_height,
            |ui| focus = Some(list_pane(ui, &groups, &mut filter, &mut pick)),
            |ui| {
                preview_pane(
                    ui,
                    session,
                    component,
                    picked.as_ref(),
                    after.as_ref(),
                    after_card.as_deref(),
                );
            },
        );
        focus
    });

    state.dialogs.stimulus_link.pick = pick;
    state.dialogs.stimulus_link.filter = filter;
    if closed {
        state.dialogs.stimulus_link.close();
        return;
    }
    match choice {
        DialogChoice::Primary => {
            let Some(definition) = picked else { return };
            match super::commit_adoption(state, session.component_id, &definition) {
                Ok(line) => {
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(line));
                    state.dialogs.stimulus_link.close();
                }
                Err(refusal) => state.dialogs.stimulus_link.error = Some(refusal),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => state.dialogs.stimulus_link.close(),
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

/// How tall the body is: exactly what the preview pane needs, held between the
/// two bounds a modal list stays readable in.
fn body_height(picked: bool, after_card: Option<&str>, card: &str) -> f32 {
    let identical = after_card == Some(card);
    // The instrument, then everything the pane hangs under it. Written as the
    // same sum the pane paints, in the same order, so a band added to one is
    // visibly missing from the other.
    let mut height = 16.0;
    height +=
        source_preview::instrument_height(if picked { 2 } else { 1 }, source_preview::STRIP_ROW);
    if !picked {
        height += 6.0 + shell::CAPTION_HEIGHT;
    }
    height += 12.0;
    height += shell::card_block_height(usize::from(after_card.is_some() && !identical) + 1);
    if identical {
        height += 4.0 + 16.0;
    }
    if picked {
        height += 12.0 + 3.0 * shell::FACT_PITCH;
    }
    height += 16.0;
    height.clamp(MIN_BODY, MAX_BODY)
}

/// The left pane: the filter, then the groups. Returns the filter field's id,
/// which is where the dialog opens its focus.
fn list_pane(
    ui: &mut Ui,
    groups: &[AdoptGroup<'_>],
    filter: &mut String,
    pick: &mut Option<String>,
) -> egui::Id {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let filter_id = ui.id().with("stimulus-adopt-filter");
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: shell::PANE_INSET,
            right: shell::PANE_INSET,
            top: 14,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.set_width(width - 2.0 * f32::from(shell::PANE_INSET));
            let response = ui.add_sized(
                [ui.available_width(), t.metrics.ctl_h],
                egui::TextEdit::singleline(filter)
                    .id(filter_id)
                    .hint_text("Filter definitions")
                    .font(egui::TextStyle::Monospace)
                    .margin(egui::Margin::symmetric(8, 4)),
            );
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_label("Filter definitions");
                node.set_description("Matches a definition's name, family or purpose");
            });
            // Escape belongs to the filter while the filter holds something and
            // to the dialog otherwise. Consumed here, before the kit reads it,
            // so clearing a search does not also abandon the transaction.
            if response.has_focus()
                && !filter.is_empty()
                && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape))
            {
                filter.clear();
            }
            step_selection(ui, groups, pick);
            focus_filter_on_typing(ui, filter_id);
            ui.add_space(10.0);
            egui::ScrollArea::vertical()
                .id_salt("stimulus-adopt-definitions")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (index, group) in groups.iter().enumerate() {
                        if index > 0 {
                            ui.add_space(6.0);
                        }
                        group_label(ui, &group.label);
                        for row in &group.rows {
                            let selected = pick.as_deref() == Some(row.name.as_str());
                            if definition_row(ui, row, selected).clicked() {
                                *pick = Some(row.name.clone());
                            }
                        }
                    }
                    if groups.is_empty() {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("No definition matches this filter")
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                    }
                });
        });
    filter_id
}

/// Move the pick with the arrow keys, across the group boundaries and over the
/// rows that cannot be adopted.
///
/// A disabled row is skipped rather than selected and then refused: the primary
/// reads its enablement from the pick, so landing on one would leave a reader
/// pressing a button that does nothing with no sentence saying why.
fn step_selection(ui: &Ui, groups: &[AdoptGroup<'_>], pick: &mut Option<String>) {
    let step = ui.input_mut(|input| {
        i32::from(input.consume_key(Modifiers::NONE, Key::ArrowDown))
            - i32::from(input.consume_key(Modifiers::NONE, Key::ArrowUp))
    });
    if step == 0 {
        return;
    }
    if let Some(next) = stepped(groups, pick.as_deref(), step) {
        *pick = Some(next);
    }
}

/// Where `step` rows from `pick` lands, over the rows a reader can reach.
///
/// Separated from the input read so the rule can be stated without a frame: the
/// claim worth holding is that no walk of this list ever arrives on a row the
/// primary would refuse.
pub(super) fn stepped(groups: &[AdoptGroup<'_>], pick: Option<&str>, step: i32) -> Option<String> {
    let reachable: Vec<&str> = groups
        .iter()
        .flat_map(|group| group.rows.iter())
        .filter(|row| row.refusal.is_none())
        .map(|row| row.name.as_str())
        .collect();
    let first = *reachable.first()?;
    let next = match pick.and_then(|name| reachable.iter().position(|row| *row == name)) {
        Some(index) => {
            let moved = index as i32 + step;
            reachable[moved.clamp(0, reachable.len() as i32 - 1) as usize]
        }
        None => first,
    };
    Some(next.to_owned())
}

/// Send a keystroke to the filter wherever the reader's focus happens to be.
///
/// The list is the surface people arrive at, and typing into it means one
/// thing: narrow it. The character is not lost, because focus is granted before
/// the field is added on this same frame.
fn focus_filter_on_typing(ui: &Ui, filter_id: egui::Id) {
    let typed = ui.input(|input| {
        input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Text(_)))
    });
    if typed && !ui.memory(|memory| memory.has_focus(filter_id)) {
        ui.memory_mut(|memory| memory.request_focus(filter_id));
    }
}

/// One group's quiet label.
fn group_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), GROUP_LABEL_HEIGHT),
        Sense::hover(),
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, label));
    ui.painter().text(
        egui::pos2(rect.left(), rect.bottom() - 7.0),
        egui::Align2::LEFT_BOTTOM,
        label,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
}

/// One definition row: its saved waveform, its name over its facts, and the
/// revision this copy would come from.
///
/// Every run is painted rather than added as a label: a label inside a click
/// rect swallows the press that was meant for the row.
fn definition_row(ui: &mut Ui, row: &AdoptRow<'_>, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let enabled = row.refusal.is_none();
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), ROW_HEIGHT),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let meta = row.meta.join(" \u{00b7} ");
    let announcement = match row.refusal.as_deref() {
        Some(refusal) => format!("{} r{} {meta}. {refusal}", row.name, row.revision),
        None => format!("{} r{} {meta}", row.name, row.revision),
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
        ui.painter().rect_filled(
            Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    } else if enabled && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    let mini = Rect::from_min_size(
        egui::pos2(
            rect.left() + 10.0,
            rect.center().y - source_preview::MINI_SIZE.y * 0.5,
        ),
        source_preview::MINI_SIZE,
    );
    source_preview::paint_mini(ui.painter(), &t, mini, row.curve, row.gap, enabled);

    let revision_left = rect.right() - 34.0;
    ui.painter().rect_filled(
        Rect::from_min_size(
            egui::pos2(revision_left, rect.center().y - 7.0),
            vec2(24.0, 14.0),
        ),
        3.0,
        t.color.bg_inset,
    );
    ui.painter().text(
        egui::pos2(revision_left + 12.0, rect.center().y),
        egui::Align2::CENTER_CENTER,
        format!("r{}", row.revision),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );

    let text_left = mini.right() + 10.0;
    let text_width = (revision_left - 8.0 - text_left).max(20.0);
    let name_colour = if enabled {
        t.color.text
    } else {
        t.color.text_faint
    };
    elided(
        ui,
        egui::pos2(text_left, rect.top() + 11.0),
        text_width,
        &row.name,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        name_colour,
    );
    // The meta line says what the definition *is*, on a refused row as much as
    // on an adoptable one: why it is refused is on the row's hover, where a
    // whole sentence has room, and repeating it here only elides it.
    let meta_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let fitted = fitted_meta(ui, &row.meta, text_width, meta_font.clone());
    elided(
        ui,
        egui::pos2(text_left, rect.bottom() - 10.0),
        text_width,
        &fitted,
        meta_font,
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect);
    match row.refusal.as_deref() {
        Some(refusal) => response.on_hover_text(refusal),
        None => response,
    }
}

/// As many of a row's meta segments as fit the track it has, longest prefix
/// first.
///
/// A segment is a whole fact — a family, a period, a pair of rails — so a row
/// too narrow for all of them gives up the last one entirely rather than
/// printing half of a number and an ellipsis, which is a figure a reader can
/// misread rather than one they can see is missing.
fn fitted_meta(ui: &mut Ui, parts: &[String], width: f32, font: egui::FontId) -> String {
    let mut kept = parts.len();
    while kept > 1 {
        let line = parts[..kept].join(" \u{00b7} ");
        if shell::measured_width(ui, &line, font.clone()) <= width {
            return line;
        }
        kept -= 1;
    }
    parts.first().cloned().unwrap_or_default()
}

/// One painted run, centred on `anchor` and elided into `width`.
fn elided(
    ui: &mut Ui,
    anchor: egui::Pos2,
    width: f32,
    text: &str,
    font: egui::FontId,
    color: egui::Color32,
) {
    let rect = Rect::from_min_max(
        egui::pos2(anchor.x, anchor.y - 8.0),
        egui::pos2(anchor.x + width, anchor.y + 8.0),
    );
    shell::painted_run(ui, rect, false, text, font, color);
}

/// The right pane: the two strips, the card the transaction writes, and the
/// three facts a reader checks before committing.
fn preview_pane(
    ui: &mut Ui,
    session: &StimulusLinkDialogState,
    component: &Component,
    picked: Option<&StimulusDefinition>,
    after: Option<&Component>,
    after_card: Option<&str>,
) {
    let width = ui.available_width();
    let unit = placed_sources::source_unit(component);
    let family = StimulusFamily::of(component.kind).map_or_else(
        || component.kind.display_name().to_owned(),
        |family| family.label().to_owned(),
    );
    egui::Frame::NONE
        .inner_margin(egui::Margin::same(shell::PANE_INSET))
        .show(ui, |ui| {
            ui.set_width(width - 2.0 * f32::from(shell::PANE_INSET));
            ui.spacing_mut().item_spacing.y = 0.0;
            let now = source_preview::source_curve(component, session.timing);
            let next =
                after.map(|candidate| source_preview::source_curve(candidate, session.timing));
            let after_caption = picked.map(|definition| {
                format!(
                    "After adoption \u{00b7} {} r{} \u{00b7} {}",
                    definition.name(),
                    definition.revision(),
                    definition.family().label()
                )
            });
            let now_caption = format!("Now \u{00b7} {family}");
            let window = session.timing.caption();
            let mut strips = vec![source_preview::Strip {
                curve: &now,
                unit,
                accent: false,
                caption: &now_caption,
                label: "the waveform this instance carries now",
            }];
            if let (Some(next), Some(caption)) = (next.as_ref(), after_caption.as_deref()) {
                strips.push(source_preview::Strip {
                    curve: next,
                    unit,
                    accent: true,
                    caption,
                    label: "the waveform this instance would carry",
                });
            }
            source_preview::paint_instrument(
                ui,
                &source_preview::Instrument {
                    strips: &strips,
                    row_height: source_preview::STRIP_ROW,
                    stop: session.timing.tstop,
                    window: Some(&window),
                },
            );
            if picked.is_none() {
                ui.add_space(6.0);
                shell::caption_row(
                    ui,
                    &format!(
                        "Pick a definition to see the card {} would hold",
                        session.reference
                    ),
                    None,
                );
            }
            ui.add_space(12.0);
            shell::card_block(ui, &session.card, after_card);
            if after_card == Some(session.card.as_str()) {
                ui.add_space(4.0);
                ui.add(egui::Label::new(
                    egui::RichText::new(format!(
                        "{} already holds this card; adopting records the receipt",
                        session.reference
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_faint),
                ));
            }
            let Some(definition) = picked else {
                return;
            };
            ui.add_space(12.0);
            let re_placed = matches!(
                definition.adoption_fit(component),
                AdoptionFit::Replace { .. }
            );
            let from_family = format!("{family} source");
            let to_family = format!("{} source", definition.family().label());
            let terminals = format!("{}, {} kept", session.nets[0], session.nets[1]);
            let adopted = format!("adopted \u{00b7} r{}", definition.revision());
            shell::facts(
                ui,
                96.0,
                &[
                    (
                        "Component",
                        if re_placed {
                            Fact::Transition(&from_family, &to_family)
                        } else {
                            Fact::Plain(&from_family)
                        },
                    ),
                    ("Terminals", Fact::Plain(&terminals)),
                    ("Provenance", Fact::Transition(&session.chip, &adopted)),
                ],
            );
        });
}
