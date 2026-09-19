//! The Stimulus Library's browser: the project's definitions, grouped by the
//! shape they realize and shown as the waveforms they are.
//!
//! A definition list that states only a name and a keyword makes the reader
//! open fourteen definitions to find the one they meant. So every row draws the
//! saved revision's own waveform, evaluated once by the engine's own evaluator
//! and held against (name, revision) — the same picture, from the same painter,
//! the link dialog's pick list shows at the same size.
//!
//! Nothing here decides a fact. The groups are `StimulusFamily::ALL`, the key
//! figure is `placed_sources::source_key_figure_spelled`, the adopter counts are
//! the whole design's through `actions::stimulus::design_adopter_tally`, and the
//! dirty dot is the editor's own answer about its draft. This module owns where
//! those land and which press changes which of them.
//!
//! The scope strip above the tree and the tally below it are bands rather than
//! rows inside the scroll: a scope a reader has to scroll to reach is a scope
//! they will not find, and a footer inside the list leaves the moment the list
//! is longer than the dock.

mod row;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use egui::{Rect, ScrollArea, Sense, Ui, UiBuilder};

use crate::properties::source_preview;
use crate::simulation::placed_sources;
use crate::simulation::stimulus_realize::{DETACHED_NETS, WaveformTrace};
use crate::state::ContractStrength;
use crate::state::stimulus_library::definition::{
    StimulusDefinition, StimulusFamily, StimulusKind,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{ViewOption, view_switch};
use crate::workbench::app::actions::stimulus::{self as stimulus_actions, AdopterTally};
use crate::workbench::state::{NavigatorTreeNode, StimulusBrowserState, StimulusScope, Workspace};
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{WorkbenchIcon, labeled_icon_button};
use super::{empty_navigator_row, nav_matches};

/// The scope strip's band, and the tally band under the tree.
const SCOPE_BAND: f32 = 34.0;
const FOOTER_BAND: f32 = 30.0;
/// The inset every band keeps from the dock's edges.
const INSET: f32 = 8.0;

/// How many adopters each definition has, by the name the library spells.
type Tally = HashMap<String, AdopterTally>;

/// One family's run of definitions.
struct Group<'a> {
    family: StimulusFamily,
    expanded: bool,
    rows: Vec<Row<'a>>,
}

/// One definition, resolved before anything is painted.
struct Row<'a> {
    name: String,
    revision: u32,
    /// What the mini shows if this definition has no curve.
    gap: crate::properties::source_preview::MiniGap,
    /// `I · PER 1 ms · →2 ↑`, assembled from the owner of each part.
    meta: String,
    /// The editor is holding an unapplied draft for this definition.
    dirty: bool,
    /// That draft says something the deck would refuse.
    refused: bool,
    /// The saved revision's waveform, held by the browser rather than evaluated
    /// here.
    curve: &'a Result<WaveformTrace, String>,
}

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    if state.workspace.stimulus_library.is_empty() {
        show_empty(ui, state);
        return;
    }
    // The minis are lifted out of the workbench for the length of the frame so
    // the rows can borrow them: a row that cloned its curve would copy
    // sixty-four samples per definition per frame to draw a picture the cache
    // is already holding.
    let mut browser = std::mem::take(&mut state.workbench.stimulus_browser);
    let timing = crate::workbench::app::actions::property_edit::stimulus_preview_timing(state);
    browser.minis(&state.workspace.stimulus_library, timing);
    let tally = stimulus_actions::design_adopter_tally(state);
    let counts = scope_counts(state, &tally);
    let mut scope = browser.scope;
    let mut chosen = None;
    let mut folded = None;

    let frame = ui.available_rect_before_wrap();
    let (rect, _) = ui.allocate_exact_size(frame.size(), Sense::hover());
    let scope_rect =
        Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.top() + SCOPE_BAND));
    let footer_rect = Rect::from_min_max(
        egui::pos2(
            rect.left(),
            (rect.bottom() - FOOTER_BAND).max(scope_rect.bottom()),
        ),
        rect.max,
    );
    let tree_rect = Rect::from_min_max(
        egui::pos2(rect.left(), scope_rect.bottom()),
        egui::pos2(rect.right(), footer_rect.top()),
    );

    in_band(ui, scope_rect, |ui| {
        scope = scope_strip(ui, scope, &counts);
    });
    {
        let groups = groups(state, &browser, scope, &tally);
        let selected = state.workbench.selected_stimulus_definition.clone();
        in_band(ui, tree_rect, |ui| {
            if groups.is_empty() {
                no_match(ui, state, &mut scope);
                return;
            }
            ScrollArea::vertical()
                .id_salt("workbench.stimulus.navigator")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for group in &groups {
                        if row::group_row(ui, group.family, group.rows.len(), group.expanded) {
                            folded = Some(group.family);
                        }
                        if !group.expanded {
                            continue;
                        }
                        for definition in &group.rows {
                            let active = selected.as_deref().is_some_and(|current| {
                                current.eq_ignore_ascii_case(&definition.name)
                            });
                            if row::definition_row(ui, definition, active).clicked() {
                                chosen = Some(definition.name.clone());
                            }
                        }
                    }
                });
        });
    }
    in_band(ui, footer_rect, |ui| footer(ui, state, &counts));

    browser.scope = scope;
    state.workbench.stimulus_browser = browser;
    if let Some(family) = folded {
        state
            .workbench
            .navigator_trees
            .for_workspace(Workspace::Stimulus)
            .toggle(folded_node(family));
    }
    if let Some(name) = chosen {
        state.workbench.selected_stimulus_definition = Some(name);
    }
}

/// What the scope strip and the footer count.
///
/// Over the library as a whole rather than over the filtered listing: a strip
/// whose own numbers moved with the filter beside it could never say how much
/// of the library the reader is not looking at.
struct ScopeCounts {
    all: usize,
    adopted: usize,
    unadopted: usize,
    /// Every adopter in the design, and how many of them are behind.
    total: AdopterTally,
    /// The library-wide audit, at the two strengths `source_contract` speaks
    /// at — the same tally the Validate library verb reports.
    errors: usize,
    advisories: usize,
}

fn scope_counts(state: &AppState, tally: &Tally) -> ScopeCounts {
    let mut counts = ScopeCounts {
        all: state.workspace.stimulus_library.len(),
        adopted: 0,
        unadopted: 0,
        total: AdopterTally::default(),
        errors: 0,
        advisories: 0,
    };
    for definition in state.workspace.stimulus_library.definitions() {
        let held = tally.get(definition.name());
        let adopters = held.map_or(0, |held| held.adopters);
        if adopters > 0 {
            counts.adopted += 1;
        } else {
            counts.unadopted += 1;
        }
        counts.total.adopters += adopters;
        counts.total.behind += held.map_or(0, |held| held.behind);
        let record = live_record(state, definition);
        if record.card_text(DETACHED_NETS).is_err() {
            counts.errors += 1;
        }
        for finding in stimulus_actions::contract_findings(state, &record) {
            if finding.strength == ContractStrength::Refusal {
                counts.errors += 1;
            } else {
                counts.advisories += 1;
            }
        }
    }
    counts
}

/// The record a row states facts about: the draft when one is open, so the
/// browser's dot, the footer's tally and the instrument's audit strip are all
/// looking at the same unapplied edit.
fn live_record(state: &AppState, definition: &StimulusDefinition) -> StimulusDefinition {
    state
        .workbench
        .stimulus_editor
        .draft(definition.name())
        .map_or_else(|| definition.clone(), |draft| draft.working().clone())
}

/// The filtered library, in family order, with every group's fold state.
///
/// A family with no definition left after the filter and the scope has no
/// group: a reader narrowing a library to one name should not be handed eleven
/// empty headings to scroll past.
fn groups<'a>(
    state: &AppState,
    browser: &'a StimulusBrowserState,
    scope: StimulusScope,
    tally: &Tally,
) -> Vec<Group<'a>> {
    let query = state.workbench.navigator_filter().trim().to_lowercase();
    StimulusFamily::ALL
        .into_iter()
        .filter_map(|family| {
            let rows: Vec<Row<'a>> = state
                .workspace
                .stimulus_library
                .definitions()
                .iter()
                .filter(|definition| definition.family() == family)
                .filter_map(|definition| {
                    let held = tally.get(definition.name());
                    if !scope.admits(held.map_or(0, |held| held.adopters)) {
                        return None;
                    }
                    if !matches(definition, held, &query) {
                        return None;
                    }
                    Some(row(state, browser, definition, held))
                })
                .collect();
            if rows.is_empty() {
                return None;
            }
            Some(Group {
                family,
                // A filter narrows the tree to what it matched, so every group
                // it leaves standing is open: a reader who typed a name and was
                // shown a closed heading would read it as no match at all.
                expanded: !query.is_empty()
                    || !state
                        .workbench
                        .navigator_trees
                        .holds(Workspace::Stimulus, &folded_node(family)),
                rows,
            })
        })
        .collect()
}

/// Whether one definition answers the filter: its name, the family keyword its
/// card carries, the purpose it was saved with, or a placed source that adopted
/// it.
fn matches(definition: &StimulusDefinition, tally: Option<&AdopterTally>, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    nav_matches(query, definition.name())
        || nav_matches(query, definition.family().label())
        || nav_matches(query, &definition.purpose)
        || tally.is_some_and(|tally| {
            tally
                .references
                .iter()
                .any(|reference| nav_matches(query, reference))
        })
}

/// One definition's row.
fn row<'a>(
    state: &AppState,
    browser: &'a StimulusBrowserState,
    definition: &StimulusDefinition,
    tally: Option<&AdopterTally>,
) -> Row<'a> {
    let dirty = state.workbench.stimulus_editor.is_dirty(definition.name());
    Row {
        name: definition.name().to_owned(),
        revision: definition.revision(),
        gap: crate::properties::source_preview::MiniGap::of(definition),
        meta: meta_line(definition, tally),
        dirty,
        refused: dirty && draft_is_refused(state, definition),
        curve: browser
            .mini(definition.name(), definition.revision())
            .unwrap_or(&source_preview::NO_MINI),
    }
}

/// Whether the open draft says something the deck would refuse.
///
/// Asked only of a definition that has one. The engine contract over a saved
/// revision is what the footer's tally states; a per-row dot in error tone is
/// about work in progress, which is the only thing a press can still fix here.
fn draft_is_refused(state: &AppState, definition: &StimulusDefinition) -> bool {
    let record = live_record(state, definition);
    record.card_text(DETACHED_NETS).is_err()
        || stimulus_actions::contract_findings(state, &record)
            .iter()
            .any(|finding| finding.strength == ContractStrength::Refusal)
}

/// The dim line under a row's name: the quantity when it is the rarer one, the
/// one number that tells two definitions of a family apart, and who is carrying
/// a copy.
///
/// `I` rather than a column of its own, because a current source is the rarer
/// of the two and a letter in front of the figure is the whole of what
/// distinguishes them. The figure is
/// [`placed_sources::source_key_figure_spelled`], spelled the way the link
/// dialog spells it so one definition reads the same on both surfaces.
fn meta_line(definition: &StimulusDefinition, tally: Option<&AdopterTally>) -> String {
    let mut parts = Vec::new();
    if definition.kind() == StimulusKind::Current {
        parts.push(StimulusKind::Current.letter().to_owned());
    }
    let figure = placed_sources::source_key_figure_spelled(
        &definition.transient_component(),
        source_preview::display_spelling,
    );
    if !figure.is_empty() {
        parts.push(figure);
    }
    if let Some(tally) = tally.filter(|tally| tally.adopters > 0) {
        parts.push(format!(
            "\u{2192}{}{}",
            tally.adopters,
            if tally.behind > 0 { " \u{2191}" } else { "" }
        ));
    }
    parts.join(" \u{b7} ")
}

/// The All / Adopted / Unadopted strip, each with the count it would show.
fn scope_strip(ui: &mut Ui, scope: StimulusScope, counts: &ScopeCounts) -> StimulusScope {
    let labels = StimulusScope::ALL.map(|option| {
        format!(
            "{} {}",
            option.label(),
            match option {
                StimulusScope::All => counts.all,
                StimulusScope::Adopted => counts.adopted,
                StimulusScope::Unadopted => counts.unadopted,
            }
        )
    });
    let options: Vec<ViewOption<'_>> = labels
        .iter()
        .map(|label| ViewOption {
            label: label.as_str(),
            unavailable: None,
        })
        .collect();
    let selected = StimulusScope::ALL
        .iter()
        .position(|option| *option == scope)
        .unwrap_or_default();
    let mut chosen = scope;
    ui.add_space(7.0);
    ui.horizontal(|ui| {
        ui.add_space(INSET);
        if let Some(index) = view_switch(ui, "workbench.stimulus.scope", &options, selected) {
            chosen = StimulusScope::ALL[index];
        }
    });
    chosen
}

/// What the library amounts to: who is carrying it, and what the engine says
/// about it.
///
/// This counts placed sources where the scope strip above counts definitions,
/// so it says "adopters" — the word the inspector's own section uses for them.
/// Both spelled "adopted", the two numbers disagreed about a library they were
/// describing correctly.
fn footer(ui: &mut Ui, state: &AppState, counts: &ScopeCounts) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let adoption = if counts.total.adopters == 0 {
        messages.text(MessageId::StimulusNoAdopters)
    } else if counts.total.behind == 0 {
        messages.format(
            if counts.total.adopters == 1 {
                MessageId::StimulusBrowserAdoptedSingular
            } else {
                MessageId::StimulusBrowserAdopted
            },
            &[("count", &counts.total.adopters.to_string())],
        )
    } else {
        messages.format(
            if counts.total.adopters == 1 {
                MessageId::StimulusBrowserAdoptedBehindSingular
            } else {
                MessageId::StimulusBrowserAdoptedBehind
            },
            &[
                ("count", &counts.total.adopters.to_string()),
                ("behind", &counts.total.behind.to_string()),
            ],
        )
    };
    let audit = messages.format(
        MessageId::StimulusAuditCounts,
        &[
            ("errors", &counts.errors.to_string()),
            ("advisories", &counts.advisories.to_string()),
        ],
    );
    let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        egui::Stroke::new(1.0, palette.border),
    );
    let announcement = format!("{adoption} \u{b7} {audit}");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    ui.painter().text(
        egui::pos2(rect.left() + INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &adoption,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if counts.total.behind > 0 {
            palette.warn
        } else {
            palette.text_dim
        },
    );
    ui.painter().text(
        egui::pos2(rect.right() - INSET, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        &audit,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        if counts.errors > 0 {
            palette.err
        } else if counts.advisories > 0 {
            palette.warn
        } else {
            palette.text_faint
        },
    );
    let _ = response.on_hover_text(messages.text(MessageId::StimulusBrowserFooterHint));
}

/// A library nobody has authored into, and the one verb that resolves it.
fn show_empty(ui: &mut Ui, state: &mut AppState) {
    let messages = state.ui.messages();
    empty_navigator_row(ui, &messages.text(MessageId::StimulusBrowserEmpty));
    let label = messages.text(MessageId::StimulusNewDefinition);
    let mut create = false;
    ui.horizontal(|ui| {
        ui.add_space(INSET);
        create |= labeled_icon_button(ui, WorkbenchIcon::Add, &label, false, 150.0).clicked();
    });
    if create {
        stimulus_actions::new_definition(state);
    }
}

/// A filter and a scope that between them left nothing, and the one press that
/// undoes both.
fn no_match(ui: &mut Ui, state: &mut AppState, scope: &mut StimulusScope) {
    let messages = state.ui.messages();
    empty_navigator_row(ui, &messages.text(MessageId::StimulusBrowserNoMatch));
    let label = messages.text(MessageId::StimulusBrowserClearFilters);
    let mut cleared = false;
    ui.horizontal(|ui| {
        ui.add_space(INSET);
        cleared |= labeled_icon_button(ui, WorkbenchIcon::Close, &label, false, 150.0).clicked();
    });
    if cleared {
        state.workbench.clear_navigator_filter();
        *scope = StimulusScope::All;
    }
}

/// The key one family's group is remembered as **folded** by.
///
/// Folded rather than unfolded, because these groups open by default: a library
/// of fourteen definitions under twelve closed headings is a browser that shows
/// the reader nothing, and the tree state stores exactly the nodes a reader has
/// acted on.
fn folded_node(family: StimulusFamily) -> NavigatorTreeNode {
    NavigatorTreeNode::Master(format!("stimulus/folded/{}", family.label()))
}

/// Paint one band into exactly its rectangle, clipped to it.
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
    band.set_min_size(egui::vec2(rect.width(), 0.0));
    band.set_max_size(rect.size());
    content(&mut band);
}
