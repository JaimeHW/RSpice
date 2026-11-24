//! The netlist navigator: outline, diff, and source mapping.
//!
//! The outline is projected from the canonical netlist document rather than
//! from the editor buffer, so what it lists is what would actually be
//! simulated. Source mapping resolves a line back to the include and directive
//! that produced it, which is the only way a generated line can be traced to
//! the deck an engineer wrote.
//!
//! Every category the outline parser produces is reachable exactly once. The
//! five an engineer navigates constantly become disclosable structure groups
//! whose children are the declarations themselves; the remaining seven are
//! counted in the semantic index and jump to their first card. A category that
//! appeared in neither would be something the deck states and the navigator
//! hides.

use std::collections::BTreeSet;

use super::*;
use egui::{Response, ScrollArea, Ui};

use crate::state::OutlineEntry;
use crate::state::{OutlineRow, OutlineSection, OutlineSectionKind};
use crate::ui::panels::{WorkbenchIcon, section_header};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::documents::netlist_document::{NetlistDocumentState, NetlistRoot};
use crate::workbench::documents::source_editing::SourceEditingState;
use crate::workbench::{MessageCatalog, MessageId};
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::Model;

mod decks;
mod lib_sections;

/// Width reserved for a disclosure caret. Root and include rows leave it
/// empty so their icons line up with the groups' rather than sliding left.
const NETLIST_OUTLINE_CARET_COLUMN: f32 = 13.0;
const NETLIST_OUTLINE_CARET_SIZE: f32 = 9.0;
/// Indent of a declaration under its group, past the guide line.
const NETLIST_OUTLINE_CHILD_INDENT: f32 = 12.0;

/// The Netlist page's outline over what it reads and writes in place, and
/// what it asks of the whole application.
/// What the Netlist page's outline reads, and writes in place: the model,
/// the netlist editor's and the Verilog-A editor's state, the outline
/// groups the reader collapsed, the include tabs the reader opened, the runs,
/// the navigator's filter, whether the pointer is coarse and the message
/// catalogue.
pub(crate) struct NetlistNavigator<'a> {
    pub(crate) model: &'a mut Model,
    pub(crate) netlist: &'a mut NetlistDocumentState,
    pub(crate) source_editing: &'a mut SourceEditingState,
    pub(crate) outline_collapsed: &'a mut BTreeSet<OutlineSectionKind>,
    pub(crate) open_documents:
        &'a mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    pub(crate) runs: &'a mut RunsState,
    pub(crate) filter: &'a str,
    pub(crate) coarse_pointer: bool,
    pub(crate) messages: MessageCatalog,
}

/// A `.lib` card pointed at another section: the source the rewrite was
/// made from, the rewritten source, the include it edits (none for the
/// deck itself) and the line reporting it once committed.
#[derive(Debug)]
pub(crate) struct LibSectionRewrite {
    pub(crate) dependency: Option<String>,
    pub(crate) original: String,
    pub(crate) replacement: String,
    pub(crate) selected: String,
}

/// What the outline asked of the whole application once it had drawn.
#[derive(Debug, Default)]
pub(crate) struct NetlistNavigatorAsked {
    /// The last line the outline has to say.
    pub(crate) prompt: Option<String>,
    /// The cell view an include row opens in Design.
    pub(crate) open_in_design: Option<crate::state::CellViewRef>,
    /// The library section a card was rewritten to.
    pub(crate) use_section: Option<LibSectionRewrite>,
}

pub(in crate::workbench) fn netlist(
    ui: &mut Ui,
    parts: &mut NetlistNavigator<'_>,
) -> NetlistNavigatorAsked {
    let mut asked = NetlistNavigatorAsked::default();
    if parts.netlist.front.is_comparison() {
        netlist_diff(ui, parts);
        return asked;
    }
    if crate::workbench::documents::deck_files::reading_deck_in_front(parts.netlist) {
        reading_outline(ui, parts, &mut asked);
        return asked;
    }
    if let Some(missing) =
        crate::workbench::documents::deck_files::missing_deck_in_front(parts.model, parts.netlist)
    {
        missing_deck_outline(ui, parts, &missing, &mut asked);
        return asked;
    }
    let root_label =
        active_netlist_artifact_name(parts.model, parts.netlist, parts.runs, parts.messages);
    let messages = parts.messages;
    let Some(index) = crate::workbench::documents::netlist_document::visible_source_index_in(
        ui.ctx(),
        parts.model,
        parts.netlist,
    ) else {
        reading_outline(ui, parts, &mut asked);
        return asked;
    };
    lib_sections::refresh(parts.model, parts.netlist, parts.source_editing);
    let projection = kept_projection(
        ui.ctx(),
        &index,
        ProjectionInputs {
            query: parts.filter.to_owned(),
            root_label,
            source_mapped: parts.netlist.front.shows_generated(),
            collapsed: parts.outline_collapsed.clone(),
            includes: retained_include_facts(
                parts.model,
                parts.netlist,
                parts.source_editing,
                messages,
            ),
            messages,
        },
    );
    let project_index = crate::workbench::documents::netlist_document::language::project_index_in(
        ui.ctx(),
        parts.model,
        parts.netlist,
    );
    let symbol_query = parts.filter.trim().to_ascii_lowercase();
    let matching_symbol_indices = project_index.as_ref().map(|index| {
        if symbol_query.is_empty() {
            Vec::new()
        } else {
            index
                .symbols()
                .enumerate()
                .filter_map(|(position, symbol)| {
                    (symbol.name.to_ascii_lowercase().contains(&symbol_query)
                        || symbol.detail.to_ascii_lowercase().contains(&symbol_query)
                        || symbol
                            .definition
                            .display_name
                            .to_ascii_lowercase()
                            .contains(&symbol_query))
                    .then_some(position)
                })
                .collect::<Vec<_>>()
        }
    });
    let matching_reference_indices = project_index.as_ref().map(|index| {
        if symbol_query.is_empty() {
            Vec::new()
        } else {
            index
                .references()
                .filter_map(|(position, reference)| {
                    reference
                        .name
                        .to_ascii_lowercase()
                        .contains(&symbol_query)
                        .then_some(position)
                })
                .collect::<Vec<_>>()
        }
    });
    let tables = project_index.as_ref().map(|index| index.tables());
    let matching_hierarchy_indices = tables.as_ref().map(|tables| {
        tables
            .hierarchy()
            .iter()
            .enumerate()
            .filter_map(|(position, instance)| {
                (symbol_query.is_empty()
                    || instance.path.to_ascii_lowercase().contains(&symbol_query)
                    || instance.target.to_ascii_lowercase().contains(&symbol_query)
                    || instance
                        .location
                        .display_name
                        .to_ascii_lowercase()
                        .contains(&symbol_query))
                .then_some(position)
            })
            .collect::<Vec<_>>()
    });
    let active_line = parts.netlist.cursor_line.saturating_add(1);
    let height = outline_row_height(ui, parts.coarse_pointer);

    let mut goto = None;
    let mut goto_project_location = None;
    let mut toggled = None;
    let mut open_include = None;
    let mut open_in_design = None;
    let mut use_section = None;
    // A section rewrite edits the authored card, so a generated or snapshot
    // document offers the choice and states why it cannot take it rather than
    // hiding what the library declares.
    let source_editable =
        crate::workbench::documents::netlist_document::active_netlist_source_is_editable_in(
            parts.model,
            parts.netlist,
        );

    ScrollArea::vertical()
        .id_salt("workbench.netlist.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            decks::list(ui, parts, height, &mut asked);

            if projection.root_row.is_some() || !projection.groups.is_empty() {
                // No line count: the editor's own gutter numbers every line
                // of the deck this outline is drawn from.
                section_header(
                    ui,
                    &messages.text(MessageId::NetlistNavigatorStructure),
                    None,
                );
            }
            if let Some(root) = &projection.root_row
                && netlist_outline_row(
                    ui,
                    OutlineRowVisual {
                        label: &root.label,
                        meta: root.meta.as_deref(),
                        meta_tone: None,
                        icon: Some(netlist_outline_icon(root.kind)),
                        shape: NetlistOutlineRowShape::Leaf,
                        selected: root.contains_line(active_line),
                        enabled: root.target_line.is_some(),
                        height,
                        hover: None,
                        path: false,
                    },
                )
                .clicked()
            {
                goto = root.target_line;
            }
            for group in &projection.groups {
                if outline_group_row(ui, group, &index, active_line, height) {
                    toggled = Some(group.section);
                }
                if !group.expanded {
                    continue;
                }
                uniform_rows(ui, height, group.row_count(), |ui, position| {
                    // Declarations are named as they are drawn. A flat deck
                    // declares tens of thousands of devices and a frame shows
                    // forty of them.
                    let Some(child) = group.child(position, &index) else {
                        return;
                    };
                    if netlist_outline_row(
                        ui,
                        OutlineRowVisual {
                            label: &child.label,
                            meta: child.meta.as_deref(),
                            meta_tone: None,
                            icon: None,
                            shape: if child.nested {
                                NetlistOutlineRowShape::NestedChild
                            } else {
                                NetlistOutlineRowShape::Child
                            },
                            selected: child.contains_line(active_line),
                            enabled: true,
                            height,
                            hover: None,
                            path: false,
                        },
                    )
                    .clicked()
                    {
                        goto = Some(child.line);
                    }
                });
            }

            if !projection.include_rows.is_empty() {
                let dependency_status = netlist_dependency_status(parts.model, parts.netlist)
                    .map(|id| messages.text(id));
                section_header(
                    ui,
                    &messages.text(MessageId::NetlistNavigatorIncludes),
                    dependency_status.as_deref(),
                );
                for row in &projection.include_rows {
                    // A `.veriloga` card naming a cell view's module is that
                    // view's source: the row names the view and opens its tab
                    // in Design, the one place a module is edited.
                    if let Some(reference) =
                        crate::workbench::documents::source_editing::veriloga_dependency_view(
                            &parts.model.workspace,
                            &row.label,
                        )
                    {
                        let action = messages.text(MessageId::VerilogAOpenInDesign);
                        // Named the way its Design tab is, so the 312 px dock
                        // keeps the view's name beside the action.
                        let label = format!(
                            "{} \u{00b7} {}",
                            reference.cell,
                            crate::state::ViewType::VerilogA.caption_for(&reference.view)
                        );
                        let hover =
                            format!("{action}: {}\n{}", reference.display_path(), row.label);
                        let response = netlist_outline_row(
                            ui,
                            OutlineRowVisual {
                                label: &label,
                                meta: Some(&action),
                                meta_tone: Some(Tokens::get(ui.ctx()).color.accent),
                                icon: Some(WorkbenchIcon::VerilogA),
                                shape: NetlistOutlineRowShape::Leaf,
                                selected: row.contains_line(active_line),
                                enabled: true,
                                height,
                                hover: Some(&hover),
                                path: false,
                            },
                        );
                        if response.clicked() {
                            open_in_design = Some(reference);
                        }
                        continue;
                    }
                    // A sectioned library's row ends in its section's select,
                    // which states the bound section in the meta's place; a
                    // warning keeps its place beside it.
                    let selector = row
                        .sections
                        .as_ref()
                        .map(|_| section_selector_width(ui.available_width()));
                    let response = netlist_outline_row_beside(
                        ui,
                        OutlineRowVisual {
                            label: &row.label,
                            meta: row
                                .meta
                                .as_deref()
                                .filter(|_| selector.is_none() || row.shadowed),
                            meta_tone: row.shadowed.then(|| Tokens::get(ui.ctx()).color.warn),
                            icon: Some(netlist_outline_icon(row.kind)),
                            shape: NetlistOutlineRowShape::Leaf,
                            selected: row.contains_line(active_line),
                            enabled: row.target_line.is_some(),
                            height,
                            hover: row.tooltip.as_deref(),
                            path: true,
                        },
                        selector.map_or(0.0, |width| width + NETLIST_OUTLINE_ICON_GAP),
                    );
                    if let (Some(choice), Some(width)) = (row.sections.as_ref(), selector) {
                        section_selector(
                            ui,
                            &response,
                            width,
                            row,
                            source_editable,
                            messages,
                            &mut use_section,
                        );
                        section_choice_menu(
                            &response,
                            choice,
                            source_editable,
                            messages,
                            &mut use_section,
                        );
                    }
                    if response.clicked() {
                        open_include = Some(row.label.clone());
                    }
                }
            }

            if let Some(project_index) = project_index.as_ref() {
                let hierarchy_count = matching_hierarchy_indices.as_ref().map_or(0, Vec::len);
                if hierarchy_count > 0 {
                    let count_text = messages.format(
                        if hierarchy_count == 1 {
                            MessageId::NetlistNavigatorHierarchyInstanceSingular
                        } else {
                            MessageId::NetlistNavigatorHierarchyInstances
                        },
                        &[("count", &hierarchy_count.to_string())],
                    );
                    let hierarchy_meta = if tables
                        .as_ref()
                        .is_some_and(|tables| tables.hierarchy_truncated())
                    {
                        format!(
                            "{} · {}",
                            count_text,
                            messages.text(MessageId::NetlistNavigatorHierarchyBounded)
                        )
                    } else {
                        count_text
                    };
                    section_header(
                        ui,
                        &messages.text(MessageId::NetlistNavigatorHierarchy),
                        Some(&hierarchy_meta),
                    );
                    uniform_rows(ui, height, hierarchy_count, |ui, position| {
                        let Some(instance) = matching_hierarchy_indices
                            .as_ref()
                            .and_then(|matching| matching.get(position))
                            .and_then(|index| tables.as_ref()?.hierarchy().get(*index))
                        else {
                            return;
                        };
                        let label = format!("{} → {}", instance.path, instance.target);
                        let mut meta = format!(
                            "{}:{}",
                            instance.location.display_name, instance.location.line
                        );
                        if instance.recursive {
                            meta.push_str(" · ");
                            meta.push_str(
                                &messages.text(MessageId::NetlistNavigatorHierarchyRecursive),
                            );
                        }
                        if netlist_outline_row(
                            ui,
                            OutlineRowVisual {
                                label: &label,
                                meta: Some(&meta),
                                meta_tone: None,
                                icon: None,
                                shape: NetlistOutlineRowShape::Index,
                                selected: false,
                                enabled: true,
                                height,
                                hover: None,
                                path: false,
                            },
                        )
                        .clicked()
                        {
                            goto_project_location = Some(instance.location.clone());
                        }
                    });
                }
                let symbol_count = matching_symbol_indices.as_ref().map_or(0, |matching| {
                    if symbol_query.is_empty() {
                        project_index.symbol_count()
                    } else {
                        matching.len()
                    }
                });
                if symbol_count > 0 {
                    // The count, and the files it spans when the deck reads
                    // more than its own.
                    let files = project_index.source_count();
                    let symbols_meta = if files > 1 {
                        messages.format(
                            MessageId::NetlistNavigatorSymbolsInFiles,
                            &[
                                ("count", &symbol_count.to_string()),
                                ("files", &files.to_string()),
                            ],
                        )
                    } else {
                        symbol_count.to_string()
                    };
                    section_header(
                        ui,
                        &messages.text(MessageId::NetlistNavigatorProjectSymbols),
                        Some(&symbols_meta),
                    );
                    uniform_rows(ui, height, symbol_count, |ui, position| {
                        let symbol_index = if symbol_query.is_empty() {
                            position
                        } else {
                            matching_symbol_indices
                                .as_ref()
                                .and_then(|matching| matching.get(position))
                                .copied()
                                .unwrap_or(position)
                        };
                        let Some(symbol) = project_index.symbol(symbol_index) else {
                            return;
                        };
                        let meta = format!(
                            "{} · {}:{}",
                            symbol.kind.label(),
                            symbol.definition.display_name,
                            symbol.definition.line
                        );
                        if netlist_outline_row(
                            ui,
                            OutlineRowVisual {
                                label: &symbol.name,
                                meta: Some(&meta),
                                meta_tone: None,
                                icon: None,
                                shape: NetlistOutlineRowShape::Index,
                                selected: false,
                                enabled: true,
                                height,
                                hover: None,
                                path: false,
                            },
                        )
                        .clicked()
                        {
                            goto_project_location = Some(symbol.definition.clone());
                        }
                    });
                }
                let reference_count = matching_reference_indices.as_ref().map_or(0, Vec::len);
                if reference_count > 0 {
                    section_header(
                        ui,
                        &messages.text(MessageId::NetlistNavigatorProjectReferences),
                        Some(&messages.format(
                            if reference_count == 1 {
                                MessageId::NetlistNavigatorReferenceSingular
                            } else {
                                MessageId::NetlistNavigatorReferences
                            },
                            &[("count", &reference_count.to_string())],
                        )),
                    );
                    uniform_rows(ui, height, reference_count, |ui, position| {
                        let Some(reference) = matching_reference_indices
                            .as_ref()
                            .and_then(|matching| matching.get(position))
                            .and_then(|index| project_index.reference(*index))
                        else {
                            return;
                        };
                        let meta = format!(
                            "{} · {}:{}",
                            reference.kind.label(),
                            reference.location.display_name,
                            reference.location.line
                        );
                        if netlist_outline_row(
                            ui,
                            OutlineRowVisual {
                                label: &reference.name,
                                meta: Some(&meta),
                                meta_tone: None,
                                icon: None,
                                shape: NetlistOutlineRowShape::Index,
                                selected: false,
                                enabled: true,
                                height,
                                hover: None,
                                path: false,
                            },
                        )
                        .clicked()
                        {
                            goto_project_location = Some(reference.location.clone());
                        }
                    });
                }
            }

            if projection.show_source_mapping {
                section_header(
                    ui,
                    &messages.text(MessageId::NetlistNavigatorSourceMapping),
                    None,
                );
                netlist_source_mapping(ui, parts.model, parts.messages, active_line);
            }

            if projection.is_empty() {
                muted(ui, &messages.text(MessageId::NetlistNavigatorNoMatches));
            }
        });

    if let Some(section) = toggled {
        // Collapsing is stored, expanding clears the store, so the default for
        // a category the engineer has never touched stays "disclosed".
        let collapsed = &mut *parts.outline_collapsed;
        if !collapsed.remove(&section) {
            collapsed.insert(section);
        }
    }
    if let Some(line) = goto {
        // Keep selection feedback immediate and hand the exact one-based
        // declaration to the editor's caret/scroll transaction for the next
        // document frame.
        parts.netlist.cursor_line = line.saturating_sub(1);
        parts.netlist.requested_line = Some(line);
    }
    if let Some((line, section)) = use_section {
        match lib_sections::section_edit(parts.model, parts.netlist, line, &section, parts.messages)
        {
            Ok(rewrite) => asked.use_section = Some(rewrite),
            Err(text) => asked.prompt = Some(text),
        }
    }
    asked.open_in_design = open_in_design;
    if let Some(requested) = open_include.as_deref()
        && parts.netlist.front.shows_deck()
    {
        if let Err(error) = open_deck_include(parts, requested) {
            asked.prompt = Some(error);
        }
    } else if let Some(requested) = open_include {
        let dependency = active_canonical_netlist_document(parts.model, parts.netlist)
            .and_then(|document| {
                document
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.requested_locator() == requested)
            })
            .map(|dependency| {
                (
                    dependency.locator().logical_identity().to_owned(),
                    dependency.source().is_some(),
                )
            });
        match dependency {
            Some((identity, true)) => {
                if let Err(error) =
                    crate::workbench::documents::netlist_document::open_netlist_dependency_in(
                        parts.model,
                        parts.netlist,
                        parts.open_documents,
                        parts.runs,
                        &identity,
                        parts.messages,
                    )
                {
                    asked.prompt = Some(error);
                }
            }
            Some((_, false)) => {
                asked.prompt = Some(format!("Include '{requested}' was not found."));
            }
            None => {
                asked.prompt = Some(format!(
                    "Include '{requested}' is no longer one of the active netlist's includes."
                ));
            }
        }
    }
    if let Some(location) = goto_project_location
        && let Err(error) =
            crate::workbench::documents::netlist_document::language::open_project_location_in(
                parts.model,
                parts.netlist,
                parts.open_documents,
                parts.runs,
                &location,
                parts.messages,
            )
    {
        asked.prompt = Some(error);
    }
    asked
}

/// The outline of a deck whose file is missing: the project's decks, then
/// just the missing file, said to be not found, its last path on hover.
/// There is no text to outline or map back to its source.
fn missing_deck_outline(
    ui: &mut Ui,
    parts: &mut NetlistNavigator<'_>,
    missing: &crate::workbench::documents::deck_files::MissingDeck,
    asked: &mut NetlistNavigatorAsked,
) {
    let messages = parts.messages;
    let height = outline_row_height(ui, parts.coarse_pointer);
    let path = missing.path.display().to_string();
    let not_found = messages.text(MessageId::NetlistNavigatorDeckMissing);
    ScrollArea::vertical()
        .id_salt("workbench.netlist.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            decks::list(ui, parts, height, asked);
            section_header(
                ui,
                &messages.text(MessageId::NetlistNavigatorStructure),
                None,
            );
            let _ = netlist_outline_row(
                ui,
                OutlineRowVisual {
                    label: &missing.name,
                    meta: Some(not_found.as_str()),
                    meta_tone: Some(Tokens::get(ui.ctx()).color.warn),
                    icon: Some(WorkbenchIcon::File),
                    shape: NetlistOutlineRowShape::Leaf,
                    selected: true,
                    enabled: false,
                    height,
                    hover: Some(&path),
                    path: false,
                },
            );
        });
}

/// The outline while the text in front is read for the first time: the
/// decks, the Structure header and a busy line where the rows will be, so
/// the navigator says the text is being read rather than that it is empty.
fn reading_outline(
    ui: &mut Ui,
    parts: &mut NetlistNavigator<'_>,
    asked: &mut NetlistNavigatorAsked,
) {
    let messages = parts.messages;
    let height = outline_row_height(ui, parts.coarse_pointer);
    ScrollArea::vertical()
        .id_salt("workbench.netlist.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            decks::list(ui, parts, height, asked);
            section_header(
                ui,
                &messages.text(MessageId::NetlistNavigatorStructure),
                None,
            );
            // Laid as a row: the mark in the rows' icon column.
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), height),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.add_space(NETLIST_OUTLINE_PADDING_X + NETLIST_OUTLINE_CARET_COLUMN);
                    crate::ui::widgets::busy_mark(ui, NETLIST_OUTLINE_ICON_SIZE);
                    ui.label(messages.text(MessageId::NetlistReadingText));
                },
            );
        });
}

/// What the outline's rows are worked out from, besides the index.
#[derive(Clone, PartialEq)]
struct ProjectionInputs {
    query: String,
    root_label: String,
    source_mapped: bool,
    collapsed: BTreeSet<OutlineSectionKind>,
    includes: Vec<IncludeRowFacts>,
    messages: MessageCatalog,
}

/// The outline's rows for `index` and `inputs`, worked out once for each:
/// an idle frame reads the rows rather than walking every entry of the
/// outline again. The index is told by identity; a new text has a new one.
fn kept_projection(
    ctx: &egui::Context,
    index: &std::sync::Arc<crate::state::NetlistSourceIndex>,
    inputs: ProjectionInputs,
) -> std::sync::Arc<NetlistNavigatorProjection> {
    type Kept = (
        std::sync::Arc<crate::state::NetlistSourceIndex>,
        ProjectionInputs,
        std::sync::Arc<NetlistNavigatorProjection>,
    );
    let id = egui::Id::new("workbench.netlist.navigator.projection");
    if let Some(kept) = ctx.data(|data| data.get_temp::<std::sync::Arc<Kept>>(id))
        && std::sync::Arc::ptr_eq(&kept.0, index)
        && kept.1 == inputs
    {
        return std::sync::Arc::clone(&kept.2);
    }
    let projection = std::sync::Arc::new(NetlistNavigatorProjection::from_index(
        index,
        &inputs.query,
        &inputs.root_label,
        inputs.source_mapped,
        &inputs.collapsed,
        &inputs.includes,
        inputs.messages,
    ));
    let kept = (
        std::sync::Arc::clone(index),
        inputs,
        std::sync::Arc::clone(&projection),
    );
    ctx.data_mut(|data| data.insert_temp(id, std::sync::Arc::new(kept)));
    projection
}

fn outline_row_height(ui: &Ui, coarse_pointer: bool) -> f32 {
    if coarse_pointer || ui.ctx().content_rect().width() <= 820.0 {
        NETLIST_OUTLINE_TOUCH_ROW_HEIGHT
    } else {
        NETLIST_OUTLINE_ROW_HEIGHT
    }
}

fn outline_group_row(
    ui: &mut Ui,
    group: &NetlistOutlineGroup,
    index: &crate::state::NetlistSourceIndex,
    active_line: usize,
    height: f32,
) -> bool {
    let response = netlist_outline_row(
        ui,
        OutlineRowVisual {
            label: &group.row.label,
            meta: group.row.meta.as_deref(),
            meta_tone: None,
            icon: Some(netlist_outline_icon(group.row.kind)),
            shape: NetlistOutlineRowShape::Group {
                expanded: group.expanded,
            },
            // A collapsed group stands in for the declaration holding the
            // caret. An expanded one does not: its child is on screen and
            // says so itself, and two selected rows claim two carets.
            selected: !group.expanded && group.contains_line(index, active_line),
            enabled: true,
            height,
            hover: None,
            path: false,
        },
    );
    let expanded = group.expanded;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(expanded);
    });
    response.clicked()
}

/// Open the file an include line of the open deck names, as the check read
/// it, else where it is found now.
fn open_deck_include(parts: &mut NetlistNavigator<'_>, requested: &str) -> Result<(), String> {
    let traced = parts
        .netlist
        .diagnostics
        .includes()
        .iter()
        .find(|include| include.requested == requested)
        .map(|include| include.path.clone());
    #[cfg(not(target_arch = "wasm32"))]
    let traced = traced.or_else(|| {
        parts.netlist.open_deck.as_ref().and_then(|deck| {
            crate::workbench::documents::deck_files::resolve_include(parts.model, deck, requested)
        })
    });
    let path = traced.ok_or_else(|| format!("Include '{requested}' was not found."))?;
    crate::workbench::documents::deck_files::show_include(
        parts.model,
        parts.netlist,
        parts.open_documents,
        parts.runs,
        &path,
        parts.messages,
    )
}

pub(super) fn active_netlist_artifact_name(
    model: &Model,
    netlist: &NetlistDocumentState,
    runs: &RunsState,
    messages: crate::workbench::MessageCatalog,
) -> String {
    match netlist.front.root_kind() {
        NetlistRoot::Generated => "generated.sp".to_owned(),
        NetlistRoot::AddedLines => {
            messages.text(crate::workbench::MessageId::NetlistAddedLinesDocument)
        }
        NetlistRoot::Deck => netlist
            .open_deck
            .as_ref()
            .map(|deck| deck.root.name())
            .or_else(|| {
                model
                    .workspace
                    .content
                    .decks
                    .first()
                    .map(|deck| deck.name())
            })
            .unwrap_or_else(|| "deck.cir".to_owned()),
        NetlistRoot::Comparison => "generated.diff".to_owned(),
        // A synthetic filename, like the two above it: an identifier, never
        // translated copy.
        NetlistRoot::RunInput => {
            crate::workbench::documents::netlist_document::run_deck_snapshot_artifact_name_in(runs)
        }
    }
}

pub(super) fn active_canonical_netlist_document<'a>(
    model: &'a Model,
    netlist: &'a NetlistDocumentState,
) -> Option<&'a crate::state::NetlistDocument> {
    match netlist.front.root_kind() {
        NetlistRoot::Generated => model.netlist_artifacts.generated_document.as_ref(),
        NetlistRoot::AddedLines
        | NetlistRoot::Deck
        | NetlistRoot::Comparison
        | NetlistRoot::RunInput => None,
    }
}

/// What the canonical document and the retained resolution trace know about
/// one include row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IncludeRowFacts {
    /// The locator the include card wrote.
    pub(super) locator: String,
    /// The line of the deck that writes it, when the facts are for one line.
    pub(super) line: Option<usize>,
    /// The closure's verdict for this dependency; none for a deck's include,
    /// which is a file the deck reads where the engine found it.
    pub(super) state: Option<MessageId>,
    /// The chain stage that resolved it, already localized.
    pub(super) via: Option<String>,
    /// Every candidate the resolver tried, with the winner marked.
    pub(super) chain: Option<String>,
    /// The later candidate of the same relative name, when one exists.
    pub(super) shadowed_by: Option<String>,
    /// The `.lib` sections the retained bytes declare, in declaration order.
    /// A dependency that is not a sectioned library declares none.
    pub(super) sections: Vec<rspice_core::library::LibSectionSummary>,
}

/// Whether `path`, where the check read an include, is a file kept with the
/// open deck the project keeps as text.
fn kept_with_deck(netlist: &NetlistDocumentState, path: &std::path::Path) -> bool {
    let kept = rspice_design::deck::stored_file_path(path);
    netlist
        .open_deck
        .as_ref()
        .and_then(|deck| deck.stored_files.as_ref())
        .is_some_and(|files| files.iter().any(|file| file.path == kept))
}

/// The catalog word for the chain stage a dependency resolved through.
fn include_stage_text(
    messages: MessageCatalog,
    stage: rspice_core::netlist::IncludeSearchStage,
) -> String {
    use rspice_core::netlist::IncludeSearchStage as Stage;
    match stage {
        Stage::DriveRelative => messages.text(MessageId::NetlistIncludeViaDriveRelative),
        Stage::Absolute => messages.text(MessageId::NetlistIncludeViaAbsolute),
        Stage::IncludingFile => messages.text(MessageId::NetlistIncludeViaIncludingFile),
        Stage::TopLevel => messages.text(MessageId::NetlistIncludeViaTopLevel),
        Stage::Execution => messages.text(MessageId::NetlistIncludeViaExecution),
        Stage::LibraryPath(index) => messages.format(
            MessageId::NetlistIncludeViaSearchPath,
            &[("index", &(index + 1).to_string())],
        ),
        Stage::Conventional(directory) => messages.format(
            MessageId::NetlistIncludeViaConventional,
            &[("directory", directory.trim_end_matches('/'))],
        ),
        Stage::SealedEdge => messages.text(MessageId::NetlistIncludeViaRetainedBundle),
    }
}

/// [`include_stage_text`] for a stage of the chain this session resolves
/// through, which holds the project's `project_paths` own search paths first
/// and the library folders set in Preferences after them.
fn chain_stage_text(
    messages: MessageCatalog,
    stage: rspice_core::netlist::IncludeSearchStage,
    project_paths: usize,
) -> String {
    match stage {
        rspice_core::netlist::IncludeSearchStage::LibraryPath(index) if index >= project_paths => {
            messages.text(MessageId::NetlistIncludeViaLibraryFolder)
        }
        stage => include_stage_text(messages, stage),
    }
}

/// The whole ordered chain, winner marked, for a row's tooltip.
fn include_chain_text(
    messages: MessageCatalog,
    locator: &str,
    resolution: &rspice_core::netlist::IncludeResolution,
    project_paths: usize,
) -> String {
    let mut lines =
        vec![messages.format(MessageId::NetlistIncludeChainHeader, &[("name", locator)])];
    let mut winner_seen = false;
    for candidate in resolution.tried() {
        let verdict = if candidate.exists() && !winner_seen {
            winner_seen = true;
            messages.text(MessageId::NetlistIncludeChainWinner)
        } else if candidate.exists() {
            messages.text(MessageId::NetlistIncludeChainShadowed)
        } else {
            messages.text(MessageId::NetlistIncludeChainAbsent)
        };
        lines.push(format!(
            "{}  {}  \u{2014} {verdict}",
            chain_stage_text(messages, candidate.stage(), project_paths),
            candidate.path().display()
        ));
    }
    lines.join("\n")
}

/// Offer the sections the row's library declares, so binding a corner is a
/// choice from what the file has rather than a name typed into the card.
///
/// The bound section is listed and disabled: a menu entry that re-selects what
/// is already selected states a choice that does nothing. A read-only document
/// disables every entry and says which, rather than hiding the catalog — what
/// the library offers is true whether or not this deck may be edited.
fn section_choice_menu(
    response: &Response,
    choice: &IncludeSectionChoice,
    editable: bool,
    messages: MessageCatalog,
    selection: &mut Option<(usize, String)>,
) {
    egui::Popup::context_menu(response)
        .show(|ui| section_choice_entries(ui, choice, editable, messages, selection));
}

/// The width of an include row's section select: one share of the row, the
/// same on every sectioned row so the selects stand in one column.
fn section_selector_width(row_width: f32) -> f32 {
    (row_width * 0.38).clamp(72.0, 150.0)
}

/// The include row's section select, at the row's right edge: the bound
/// section in its well, every section the library declares in its list with
/// what each declares. Choosing one rewrites the row's own card. A read-only
/// deck shows the select disabled and says why; a card naming a section the
/// library does not declare shows that name, with none of the list chosen.
fn section_selector(
    ui: &mut Ui,
    row: &Response,
    width: f32,
    include: &NetlistNavigatorRow,
    editable: bool,
    messages: MessageCatalog,
    selection: &mut Option<(usize, String)>,
) {
    let Some(choice) = include.sections.as_ref() else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    let height = crate::ui::widgets::compact_height(&t.metrics).min(row.rect.height());
    let rect = egui::Rect::from_min_size(
        egui::pos2(
            row.rect.right() - NETLIST_OUTLINE_PADDING_X - width,
            row.rect.center().y - height * 0.5,
        ),
        egui::vec2(width, height),
    );
    if !ui.is_rect_visible(rect) {
        return;
    }
    let options = choice
        .available
        .iter()
        .map(|section| section.name.clone())
        .collect::<Vec<_>>();
    let details = choice
        .available
        .iter()
        .map(|section| section_detail_text(section, messages))
        .collect::<Vec<_>>();
    // The card's spelling of a declared section is the library's.
    let selected = choice.selected.as_deref().map_or_else(
        || messages.text(MessageId::NetlistLibSectionNone),
        |selected| {
            options
                .iter()
                .find(|name| name.eq_ignore_ascii_case(selected))
                .map_or_else(|| selected.to_owned(), String::clone)
        },
    );
    let label = messages.format(
        MessageId::NetlistLibSectionSelect,
        &[("name", &include.label), ("section", &selected)],
    );
    let picked = ui
        .scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.add_enabled_ui(editable, |ui| {
                let output = crate::ui::widgets::compact_select_mono_with_details(
                    ui,
                    &format!("netlist-lib-section-{}", choice.line),
                    &label,
                    &selected,
                    &options,
                    &details,
                    width,
                );
                let _ = output
                    .response
                    .on_disabled_hover_text(messages.text(MessageId::NetlistLibSectionReadOnly));
                output.picked
            })
            .inner
        })
        .inner;
    if let Some(name) = picked.and_then(|index| options.get(index))
        && choice
            .selected
            .as_deref()
            .is_none_or(|bound| !bound.eq_ignore_ascii_case(name))
    {
        *selection = Some((choice.line, name.clone()));
    }
}

/// The chooser's own rows, painted wherever the menu puts them.
pub(super) fn section_choice_entries(
    ui: &mut Ui,
    choice: &IncludeSectionChoice,
    editable: bool,
    messages: MessageCatalog,
    selection: &mut Option<(usize, String)>,
) {
    ui.label(
        egui::RichText::new(messages.text(MessageId::NetlistLibUseSection))
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(Tokens::get(ui.ctx()).color.text_faint),
    );
    for section in &choice.available {
        let bound = choice
            .selected
            .as_deref()
            .is_some_and(|selected| selected.eq_ignore_ascii_case(&section.name));
        let entry = ui.add_enabled(
            editable && !bound,
            egui::Button::new(section_row_text(section, messages)),
        );
        let entry = if bound {
            entry.on_disabled_hover_text(messages.text(MessageId::NetlistLibSectionAlreadyBound))
        } else if editable {
            entry
        } else {
            entry.on_disabled_hover_text(messages.text(MessageId::NetlistLibSectionReadOnly))
        };
        if entry.clicked() {
            *selection = Some((choice.line, section.name.clone()));
            ui.close();
        }
    }
}

/// One section's name and what it declares, for a tooltip line or a menu row.
///
/// A corner is chosen by what it holds, so the counts travel with the name
/// rather than being a second thing to go and look up. A section that declares
/// no subcircuits says so by omission: `0 subckts` on every row of a model
/// library would be noise on every row.
fn section_row_text(
    section: &rspice_core::library::LibSectionSummary,
    messages: MessageCatalog,
) -> String {
    format!(
        "{} · {}",
        section.name,
        section_detail_text(section, messages)
    )
}

/// What one section declares: its models, and its subcircuits when it has
/// any.
fn section_detail_text(
    section: &rspice_core::library::LibSectionSummary,
    messages: MessageCatalog,
) -> String {
    let models = messages.format(
        if section.model_count == 1 {
            MessageId::NetlistLibSectionModel
        } else {
            MessageId::NetlistLibSectionModels
        },
        &[("count", &section.model_count.to_string())],
    );
    if section.subcircuit_count == 0 {
        return models;
    }
    let subcircuits = messages.format(
        if section.subcircuit_count == 1 {
            MessageId::NetlistLibSectionSubcircuit
        } else {
            MessageId::NetlistLibSectionSubcircuits
        },
        &[("count", &section.subcircuit_count.to_string())],
    );
    format!("{models} · {subcircuits}")
}

/// Every section the library declares, with the bound one marked, for a row's
/// tooltip. The row itself has space for the bound name and nothing else.
fn section_catalog_text(
    locator: &str,
    choice: &IncludeSectionChoice,
    messages: MessageCatalog,
) -> String {
    let bound = messages.text(MessageId::NetlistLibSectionBoundMarker);
    let mut lines =
        vec![messages.format(MessageId::NetlistLibSectionsHeader, &[("name", locator)])];
    lines.extend(choice.available.iter().map(|section| {
        let row = section_row_text(section, messages);
        if choice
            .selected
            .as_deref()
            .is_some_and(|selected| selected.eq_ignore_ascii_case(&section.name))
        {
            format!("{row}  \u{2014} {bound}")
        } else {
            row
        }
    }));
    lines.join("\n")
}

/// What the canonical document knows about each retained dependency, keyed by
/// the locator its own card wrote. The section header states one verdict for
/// the whole closure; without this the row that earned an `error` cannot be
/// told from the ones that did not.
///
/// The chain each dependency resolved through comes from the engine's own
/// trace, retained when the closure was resolved, so no second resolver has to
/// be consulted to say where a file came from.
pub(super) fn retained_include_facts(
    model: &Model,
    netlist: &NetlistDocumentState,
    source_editing: &SourceEditingState,
    messages: MessageCatalog,
) -> Vec<IncludeRowFacts> {
    let project_paths = model.workspace.content.project.include_search_paths().len();
    if netlist.front.shows_deck() {
        // A deck is a file: its includes are the files the check read for
        // its lines, one row a line, so a library two lines name for two
        // sections is two rows and one tab. A line naming a file the check
        // could not read is marked missing, with what the check said.
        let buffer = &model.simulation.source.netlist_content;
        let missing = netlist.diagnostics.iter().filter_map(|finding| {
            let line = finding.line.filter(|_| {
                crate::workbench::documents::netlist_document::is_missing_file_finding(finding)
            })?;
            let raw = netlist.missing_file_line(line, buffer)?;
            let named = rspice_core::netlist::parse_include_directive(raw)
                .or_else(|| rspice_core::netlist::parse_lib_directive(raw).map(|(path, _)| path))?;
            Some(IncludeRowFacts {
                line: Some(line + 1),
                state: Some(MessageId::NetlistNavigatorDependencyMissing),
                via: None,
                chain: Some(finding.message.clone()),
                shadowed_by: None,
                sections: Vec::new(),
                locator: named.trim_matches(['\'', '"']).to_owned(),
            })
        });
        return netlist
            .diagnostics
            .includes()
            .iter()
            .map(|include| IncludeRowFacts {
                line: Some(include.line),
                state: None,
                // A file beside the file that names it is the ordinary case,
                // and for a deck kept as text a file kept with it; the row
                // names where else one was found.
                via: (include.resolution.stage()
                    != rspice_core::netlist::IncludeSearchStage::IncludingFile
                    && !kept_with_deck(netlist, &include.path))
                .then(|| chain_stage_text(messages, include.resolution.stage(), project_paths)),
                chain: Some(include_chain_text(
                    messages,
                    &include.requested,
                    &include.resolution,
                    project_paths,
                )),
                shadowed_by: include
                    .resolution
                    .shadowed()
                    .next()
                    .map(|candidate| candidate.path().display().to_string()),
                sections: include.sections.to_vec(),
                locator: include.requested.clone(),
            })
            .chain(missing)
            .collect();
    }
    active_canonical_netlist_document(model, netlist).map_or_else(Vec::new, |document| {
        document
            .dependencies()
            .iter()
            .map(|dependency| {
                let locator = dependency.requested_locator().to_owned();
                let resolution = source_editing.include_resolutions.get(&locator);
                IncludeRowFacts {
                    line: None,
                    state: Some(match dependency.resolution() {
                        crate::state::DependencyResolution::Missing { .. } => {
                            MessageId::NetlistNavigatorDependencyMissing
                        }
                        crate::state::DependencyResolution::Unresolved => {
                            MessageId::NetlistNavigatorDependencyUnresolved
                        }
                        crate::state::DependencyResolution::Resolved { .. } => {
                            dependency_authority_message(dependency.authority())
                        }
                    }),
                    via: resolution.map(|resolution| {
                        chain_stage_text(messages, resolution.stage(), project_paths)
                    }),
                    chain: resolution.map(|resolution| {
                        include_chain_text(messages, &locator, resolution, project_paths)
                    }),
                    shadowed_by: resolution.and_then(|resolution| {
                        resolution
                            .shadowed()
                            .next()
                            .map(|candidate| candidate.path().display().to_string())
                    }),
                    sections: lib_sections::declared(source_editing, &locator),
                    locator,
                }
            })
            .collect()
    })
}

/// The catalog word for a retained dependency's provenance.
const fn dependency_authority_message(
    authority: crate::state::DependencySourceAuthority,
) -> MessageId {
    match authority {
        crate::state::DependencySourceAuthority::External => {
            MessageId::NetlistNavigatorAuthorityExternal
        }
        crate::state::DependencySourceAuthority::Vendor => {
            MessageId::NetlistNavigatorAuthorityVendor
        }
        crate::state::DependencySourceAuthority::TechnologyPackage => {
            MessageId::NetlistNavigatorAuthorityTechnology
        }
        crate::state::DependencySourceAuthority::StandardLibrary => {
            MessageId::NetlistNavigatorAuthorityStandard
        }
    }
}

/// What is wrong with the include graph, or `None` when nothing is.
///
/// Every include resolving is the ordinary case: the section used to head
/// itself "resolved", which is the heading reporting that there is nothing to
/// report.
pub(super) fn netlist_dependency_status(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<MessageId> {
    // A run snapshot and a comparison are text with no include graph of their
    // own: there is nothing wrong to report about one, so nothing is.
    let document = active_canonical_netlist_document(model, netlist)?;
    if document.dependencies().iter().any(|dependency| {
        matches!(
            dependency.resolution(),
            crate::state::DependencyResolution::Missing { .. }
        )
    }) {
        Some(MessageId::NetlistNavigatorDependencyError)
    } else if document.dependency_graph_is_sealed() {
        None
    } else {
        Some(MessageId::NetlistNavigatorDependencyUnresolved)
    }
}

/// One changed region of a revision comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DiffHunk {
    pub(super) label: String,
    pub(super) meta: String,
    /// One-based line of the hunk header inside the comparison document.
    pub(super) line: usize,
    /// Last line of the hunk inside the comparison document.
    pub(super) end_line: usize,
}

impl DiffHunk {
    fn contains_line(&self, line: usize) -> bool {
        (self.line..=self.end_line).contains(&line)
    }
}

pub(super) fn netlist_diff(ui: &mut Ui, parts: &mut NetlistNavigator<'_>) {
    let messages = parts.messages;
    let query = NetlistNavigatorQuery::new(parts.filter);
    let hunks = diff_hunks(&parts.netlist.generated_diff_source, messages);
    let (additions, removals) = diff_totals(&parts.netlist.generated_diff_source);
    let matching = hunks
        .iter()
        .filter(|hunk| query.matches_diff_hunk(hunk))
        .cloned()
        .collect::<Vec<_>>();
    let height = outline_row_height(ui, parts.coarse_pointer);
    let active_line = parts.netlist.cursor_line.saturating_add(1);
    let mut goto = None;

    ScrollArea::vertical()
        .id_salt("workbench.netlist.diff.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            section_header(
                ui,
                &messages.text(MessageId::NetlistNavigatorRevisionComparison),
                Some(&messages.format(
                    MessageId::NetlistNavigatorDiffTotals,
                    &[
                        ("added", &additions.to_string()),
                        ("removed", &removals.to_string()),
                    ],
                )),
            );
            if hunks.is_empty() {
                muted(ui, &messages.text(MessageId::NetlistNavigatorNoDifference));
                return;
            }
            if matching.is_empty() {
                muted(
                    ui,
                    &messages.text(MessageId::NetlistNavigatorNoChangedRegion),
                );
                return;
            }
            uniform_rows(ui, height, matching.len(), |ui, index| {
                let hunk = &matching[index];
                if netlist_outline_row(
                    ui,
                    OutlineRowVisual {
                        label: &hunk.label,
                        meta: Some(&hunk.meta),
                        meta_tone: None,
                        // Every row in a comparison is a comparison; an icon
                        // repeating the section header decorates and no more.
                        icon: None,
                        shape: NetlistOutlineRowShape::Index,
                        selected: hunk.contains_line(active_line),
                        enabled: true,
                        height,
                        hover: None,
                        path: false,
                    },
                )
                .clicked()
                {
                    goto = Some(hunk.line);
                }
            });
        });

    if let Some(line) = goto {
        parts.netlist.cursor_line = line.saturating_sub(1);
        parts.netlist.requested_line = Some(line);
    }
}

/// The changed regions of a unified diff, addressed by their position in the
/// comparison document — that is the buffer the editor is showing, so a hunk
/// row navigates to the header the engineer can actually see.
pub(super) fn diff_hunks(diff: &str, messages: MessageCatalog) -> Vec<DiffHunk> {
    let mut hunks = Vec::<DiffHunk>::new();
    let mut additions = 0usize;
    let mut removals = 0usize;
    let mut previous = 0usize;
    let finish = |hunks: &mut Vec<DiffHunk>, additions: usize, removals: usize, end_line: usize| {
        if let Some(last) = hunks.last_mut() {
            last.meta = messages.format(
                MessageId::NetlistNavigatorDiffTotals,
                &[
                    ("added", &additions.to_string()),
                    ("removed", &removals.to_string()),
                ],
            );
            last.end_line = end_line.max(last.line);
        }
    };
    for (zero_line, line) in diff.lines().enumerate() {
        previous = zero_line + 1;
        if let Some(range) = line.strip_prefix("@@") {
            finish(&mut hunks, additions, removals, zero_line);
            additions = 0;
            removals = 0;
            hunks.push(DiffHunk {
                label: diff_hunk_label(range, messages),
                meta: String::new(),
                line: zero_line + 1,
                end_line: zero_line + 1,
            });
        } else if line.starts_with('+') && !line.starts_with("+++") {
            additions += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            removals += 1;
        }
    }
    finish(&mut hunks, additions, removals, previous);
    hunks
}

/// Name a hunk by the lines it occupies in the newer revision, which is the
/// side an engineer is reading the comparison to understand.
fn diff_hunk_label(range: &str, messages: MessageCatalog) -> String {
    let new_side = range
        .split_whitespace()
        .find_map(|token| token.strip_prefix('+'));
    let Some((start, count)) = new_side.map(|side| match side.split_once(',') {
        Some((start, count)) => (start.parse::<usize>().ok(), count.parse::<usize>().ok()),
        None => (side.parse::<usize>().ok(), Some(1)),
    }) else {
        return messages.text(MessageId::NetlistNavigatorChangedRegion);
    };
    match (start, count) {
        (Some(start), Some(0)) => messages.format(
            MessageId::NetlistNavigatorRemovedBeforeLine,
            &[("line", &start.to_string())],
        ),
        (Some(start), Some(1)) => messages.format(
            MessageId::NetlistNavigatorHunkLine,
            &[("line", &start.to_string())],
        ),
        (Some(start), Some(count)) => messages.format(
            MessageId::NetlistNavigatorHunkLines,
            &[
                ("first", &start.to_string()),
                ("last", &(start + count - 1).to_string()),
            ],
        ),
        _ => messages.text(MessageId::NetlistNavigatorChangedRegion),
    }
}

pub(super) fn diff_totals(diff: &str) -> (usize, usize) {
    crate::workbench::documents::netlist_document::diff_line_counts(diff)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NetlistNavigatorRowKind {
    Root,
    Parameters,
    Subcircuits,
    Instances,
    Models,
    Analyses,
    Measurements,
    Options,
    Outputs,
    Functions,
    Globals,
    OtherCards,
    Include,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorRow {
    pub(super) kind: NetlistNavigatorRowKind,
    pub(super) label: String,
    pub(super) meta: Option<String>,
    /// The full evidence behind `meta`, shown on hover rather than as a second
    /// line. Include rows carry their ordered search chain here.
    pub(super) tooltip: Option<String>,
    /// Whether the row's own meta carries a warning rather than a state word.
    pub(super) shadowed: bool,
    /// The library sections this row's own card can bind. Only an include of a
    /// sectioned `.lib` has any; every other row leaves it `None` and offers
    /// no section action.
    pub(super) sections: Option<IncludeSectionChoice>,
    pub(super) target_line: Option<usize>,
    pub(super) source_ranges: Vec<(usize, usize)>,
}

/// The section an include card selects, against the sections its library
/// declares. Both halves are needed to say anything: the card alone cannot
/// name the alternatives, and the library alone cannot say which one is bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IncludeSectionChoice {
    /// One-based line of the card in the visible source, which is the line the
    /// rewrite edits.
    pub(super) line: usize,
    /// Section the card names, `None` when the card is a plain `.include` of a
    /// file that happens to declare sections.
    pub(super) selected: Option<String>,
    /// Sections the library declares, in declaration order.
    pub(super) available: Vec<rspice_core::library::LibSectionSummary>,
}

impl NetlistNavigatorRow {
    pub(super) fn contains_line(&self, line: usize) -> bool {
        self.source_ranges
            .iter()
            .any(|(start, end)| (*start..=*end).contains(&line))
    }
}

/// One row under a structure group: a declaration, or the subcircuit the
/// declarations after it are written inside.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistOutlineChild {
    pub(super) label: String,
    pub(super) meta: Option<String>,
    pub(super) line: usize,
    pub(super) end_line: usize,
    /// Drawn under a subcircuit's row.
    pub(super) nested: bool,
}

impl NetlistOutlineChild {
    fn contains_line(&self, line: usize) -> bool {
        (self.line..=self.end_line).contains(&line)
    }
}

/// A disclosable structure group and the declarations it holds.
///
/// The declarations are held as outline indices rather than as rows, and a
/// row is named only when it is drawn: a deck that declares fifty thousand
/// devices would otherwise pay for all of them on every frame to show forty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistOutlineGroup {
    pub(super) row: NetlistNavigatorRow,
    pub(super) section: OutlineSectionKind,
    /// Indices into the source index's entries, in source order: unfiltered,
    /// the section's own, shared with the outline rather than listed again.
    entries: std::sync::Arc<[usize]>,
    /// The rows the group draws when a declaration sits inside a subcircuit:
    /// the top level's declarations, then each subcircuit's row and its own.
    /// `None` when every declaration is at the top level and the rows are the
    /// declarations.
    scoped: Option<std::sync::Arc<[OutlineRow]>>,
    pub(super) expanded: bool,
}

impl NetlistOutlineGroup {
    /// How many declarations this group holds, which is what its count
    /// states.
    #[cfg(test)]
    pub(super) fn declarations(&self) -> usize {
        self.entries.len()
    }

    /// How many rows the group draws when disclosed: its declarations, and a
    /// row for each subcircuit they are written inside.
    pub(super) fn row_count(&self) -> usize {
        self.scoped
            .as_ref()
            .map_or(self.entries.len(), |rows| rows.len())
    }

    /// The row at `position`, named from the index.
    pub(super) fn child(
        &self,
        position: usize,
        index: &crate::state::NetlistSourceIndex,
    ) -> Option<NetlistOutlineChild> {
        let entries = index.outline().entries();
        let slot = match &self.scoped {
            Some(rows) => *rows.get(position)?,
            None => OutlineRow::Declaration {
                entry: *self.entries.get(position)?,
                nested: false,
            },
        };
        match slot {
            OutlineRow::Declaration { entry, nested } => Some(NetlistOutlineChild {
                nested,
                ..outline_child(self.section, &entries.get(entry)?)
            }),
            OutlineRow::Subcircuit(entry) => {
                let subcircuit = entries.get(entry)?;
                Some(NetlistOutlineChild {
                    label: subcircuit.name().into_owned(),
                    meta: Some(subcircuit.keyword().into_owned()),
                    line: subcircuit.line(),
                    end_line: subcircuit.end_line(),
                    nested: false,
                })
            }
        }
    }

    /// Whether one of the declarations this group holds contains `line`.
    /// Entries are in source order, so this is a search: the group standing in
    /// for the caret must not cost the deck on every frame.
    pub(super) fn contains_line(
        &self,
        index: &crate::state::NetlistSourceIndex,
        line: usize,
    ) -> bool {
        let entries = index.outline().entries();
        self.entries
            .binary_search_by(|candidate| {
                let Some(entry) = entries.get(*candidate) else {
                    return std::cmp::Ordering::Less;
                };
                if entry.end_line() < line {
                    std::cmp::Ordering::Less
                } else if entry.line() > line {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .is_ok()
    }
}

struct GroupSpec {
    kind: NetlistNavigatorRowKind,
    section: OutlineSectionKind,
    label: MessageId,
}

/// The outline's groups, in the order a designer reads a deck: what it is
/// built from, then what it runs and reads out, then the rest. A deck's
/// `.if` blocks are read in its text, not listed.
const STRUCTURE_GROUPS: [GroupSpec; 11] = [
    GroupSpec {
        kind: NetlistNavigatorRowKind::Parameters,
        section: OutlineSectionKind::Parameters,
        label: MessageId::NetlistNavigatorParameters,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Subcircuits,
        section: OutlineSectionKind::Subcircuits,
        label: MessageId::NetlistNavigatorSubcircuits,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Instances,
        section: OutlineSectionKind::Devices,
        label: MessageId::NetlistNavigatorInstances,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Models,
        section: OutlineSectionKind::Models,
        label: MessageId::NetlistNavigatorModels,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Analyses,
        section: OutlineSectionKind::Analyses,
        label: MessageId::NetlistNavigatorAnalyses,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Measurements,
        section: OutlineSectionKind::Measurements,
        label: MessageId::NetlistNavigatorMeasurements,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Options,
        section: OutlineSectionKind::Options,
        label: MessageId::NetlistNavigatorOptions,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Outputs,
        section: OutlineSectionKind::Outputs,
        label: MessageId::NetlistNavigatorOutputs,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Functions,
        section: OutlineSectionKind::Functions,
        label: MessageId::NetlistNavigatorFunctions,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Globals,
        section: OutlineSectionKind::Globals,
        label: MessageId::NetlistNavigatorGlobals,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::OtherCards,
        section: OutlineSectionKind::Controls,
        label: MessageId::NetlistNavigatorControls,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorProjection {
    pub(super) line_count: usize,
    pub(super) root_row: Option<NetlistNavigatorRow>,
    pub(super) groups: Vec<NetlistOutlineGroup>,
    pub(super) include_rows: Vec<NetlistNavigatorRow>,
    pub(super) show_source_mapping: bool,
}

impl NetlistNavigatorProjection {
    pub(super) fn from_index(
        index: &crate::state::NetlistSourceIndex,
        query: &str,
        root_label: &str,
        source_mapped: bool,
        collapsed: &BTreeSet<OutlineSectionKind>,
        include_states: &[IncludeRowFacts],
        messages: MessageCatalog,
    ) -> Self {
        let outline = index.outline();
        let query = NetlistNavigatorQuery::new(query);
        let entries = outline.entries();
        let section = |kind: OutlineSectionKind| {
            outline
                .sections()
                .iter()
                .find(|section| section.kind() == kind)
        };
        let section_of = |kind: OutlineSectionKind| -> &[usize] {
            section(kind).map_or(&[][..], OutlineSection::entry_indices)
        };
        let resolved = |indices: &[usize]| -> Vec<OutlineEntry<'_>> {
            indices
                .iter()
                .filter_map(|index| entries.get(*index))
                .collect()
        };

        let root_entries = resolved(section_of(OutlineSectionKind::Source));
        let root_target = root_entries.first().copied().or_else(|| entries.first());
        let root_row = query
            .matches_group(root_label, root_entries.iter().copied())
            .then(|| {
                // No meta: the row names the document the tab above already
                // names, and "source of truth" named nothing at all.
                navigator_root_row(
                    NetlistNavigatorRowKind::Root,
                    root_label,
                    root_target,
                    &root_entries,
                )
            });

        let groups = STRUCTURE_GROUPS
            .iter()
            .filter_map(|spec| {
                outline_group(
                    spec,
                    section(spec.section)?,
                    entries,
                    &query,
                    collapsed,
                    messages,
                )
            })
            .collect::<Vec<_>>();

        let include_rows = resolved(section_of(OutlineSectionKind::Dependencies))
            .into_iter()
            .filter(|entry| query.matches_entry(entry))
            .map(|entry| {
                let label = include_entry_label(&entry.label());
                let facts = include_states.iter().find(|facts| {
                    facts.locator == label && facts.line.is_none_or(|line| line == entry.line())
                });
                // A locator the closure never retained is not "resolved" and
                // not "missing" either; the row says where it is written and
                // claims nothing about its fate.
                let Some(facts) = facts else {
                    return NetlistNavigatorRow {
                        kind: NetlistNavigatorRowKind::Include,
                        meta: Some(messages.format(
                            MessageId::NetlistNavigatorLine,
                            &[("line", &entry.line().to_string())],
                        )),
                        tooltip: Some(format!(
                            "{label}\n{}",
                            messages.text(MessageId::NetlistIncludeChainUntraced)
                        )),
                        shadowed: false,
                        sections: None,
                        label,
                        target_line: Some(entry.line()),
                        source_ranges: vec![(entry.line(), entry.end_line())],
                    };
                };
                // Which section the card binds comes from the card, not from
                // the library: a `.lib` file offers several and only the deck
                // says which one this deck takes.
                let sections = (!facts.sections.is_empty()).then(|| IncludeSectionChoice {
                    line: entry.line(),
                    selected: lib_sections::selected(&index.card(entry.line())),
                    available: facts.sections.clone(),
                });
                // The dock is 312 px wide and the include's own name is what
                // the row is for, so the meta column states one short phrase:
                // the warning if there is one, else which section of a
                // sectioned library is bound, else where the file came from,
                // else the closure's verdict. Everything else is the tooltip.
                let unresolved = matches!(
                    facts.state,
                    Some(
                        MessageId::NetlistNavigatorDependencyMissing
                            | MessageId::NetlistNavigatorDependencyUnresolved
                    )
                );
                let section_meta = sections.as_ref().map(|choice| match &choice.selected {
                    Some(name) => {
                        messages.format(MessageId::NetlistLibSectionBound, &[("name", name)])
                    }
                    None => messages.format(
                        MessageId::NetlistLibSectionUnbound,
                        &[("count", &choice.available.len().to_string())],
                    ),
                });
                let meta = match (&facts.shadowed_by, &section_meta, &facts.via) {
                    (Some(_), _, _) => messages.text(MessageId::NetlistIncludeShadowMarker),
                    (None, Some(section), _) if !unresolved => section.clone(),
                    (None, None, Some(via)) if !unresolved => via.clone(),
                    _ => facts.state.map_or_else(
                        || {
                            messages.format(
                                MessageId::NetlistNavigatorLine,
                                &[("line", &entry.line().to_string())],
                            )
                        },
                        |state| messages.text(state),
                    ),
                };
                let mut tooltip = facts
                    .state
                    .map(|state| format!("{}\n", messages.text(state)))
                    .unwrap_or_default();
                tooltip.push_str(&facts.chain.clone().unwrap_or_else(|| {
                    format!(
                        "{label}\n{}",
                        messages.text(MessageId::NetlistIncludeChainUntraced)
                    )
                }));
                if let Some(shadowed_by) = &facts.shadowed_by {
                    tooltip.push('\n');
                    tooltip.push_str(shadowed_by);
                }
                if let Some(choice) = &sections {
                    tooltip.push('\n');
                    tooltip.push_str(&section_catalog_text(&label, choice, messages));
                }
                NetlistNavigatorRow {
                    kind: NetlistNavigatorRowKind::Include,
                    meta: Some(meta),
                    tooltip: Some(tooltip),
                    shadowed: facts.shadowed_by.is_some(),
                    sections,
                    label,
                    target_line: Some(entry.line()),
                    source_ranges: vec![(entry.line(), entry.end_line())],
                }
            })
            .collect();

        Self {
            line_count: index.line_count(),
            root_row,
            groups,
            include_rows,
            show_source_mapping: source_mapped
                && (query.matches_text(&messages.text(MessageId::NetlistNavigatorSourceMapping))
                    || query.matches_text(&messages.text(MessageId::NetlistNavigatorProvenance))
                    || query.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.root_row.is_none()
            && self.groups.is_empty()
            && self.include_rows.is_empty()
            && !self.show_source_mapping
    }
}

fn outline_group(
    spec: &GroupSpec,
    section: &OutlineSection,
    entries: crate::state::OutlineEntries<'_>,
    query: &NetlistNavigatorQuery,
    collapsed: &BTreeSet<OutlineSectionKind>,
    messages: MessageCatalog,
) -> Option<NetlistOutlineGroup> {
    let entry_of = |position: &usize| entries.get(*position);
    // Both ends of a subcircuit share its section; the definition is the row.
    // The section's declarations and the rows they fall in were listed with
    // the outline, away from the frame: a group the filter does not narrow
    // shares them, so the frame the outline lands in reads the deck's count
    // of devices rather than walking them.
    let declared = section.declarations();
    // A group the deck declares nothing in is a row spent on nothing.
    let &first = declared.first()?;
    let declarations = declared.len();
    // The filter is matched against the label the reader can see. Matching the
    // English behind it would leave a translated navigator unsearchable by the
    // names it displays.
    let label = messages.text(spec.label);
    if !query.matches_group(&label, declared.iter().filter_map(entry_of)) {
        return None;
    }
    // A filter names the declarations it kept, so the group discloses them
    // whatever the stored preference says.
    let filtering = !query.is_empty() && !query.matches_text(&label);
    let expanded = filtering || !collapsed.contains(&spec.section);
    let (selected, scoped) = if filtering {
        let kept = declared
            .iter()
            .copied()
            .filter(|position| entry_of(position).is_some_and(|entry| query.matches_entry(&entry)))
            .collect::<Vec<_>>();
        let scoped = crate::state::rows_by_scope(&kept, entries).map(std::sync::Arc::from);
        (std::sync::Arc::from(kept), scoped)
    } else {
        (std::sync::Arc::clone(declared), section.rows().cloned())
    };
    let meta = if filtering {
        format!("{} of {declarations}", selected.len())
    } else {
        declarations.to_string()
    };
    let target = entry_of(&selected.first().copied().unwrap_or(first));
    Some(NetlistOutlineGroup {
        row: NetlistNavigatorRow {
            kind: spec.kind,
            label,
            meta: Some(meta),
            tooltip: None,
            shadowed: false,
            sections: None,
            target_line: target.map(|entry| entry.line()),
            // Containment is answered by the group from its own entries. A
            // span per declaration here would allocate the whole deck once a
            // frame to settle one question about one line.
            source_ranges: Vec::new(),
        },
        section: spec.section,
        scoped,
        entries: selected,
        expanded,
    })
}

/// Name one declaration and what it is bound to or written with, as the
/// engine's index of the deck reads them.
fn outline_child(section: OutlineSectionKind, entry: &OutlineEntry) -> NetlistOutlineChild {
    let (label, meta) = match section {
        OutlineSectionKind::Source | OutlineSectionKind::Dependencies => {
            (entry.label().into_owned(), None)
        }
        _ => (
            entry.name().into_owned(),
            entry.detail().map(std::borrow::Cow::into_owned),
        ),
    };
    NetlistOutlineChild {
        label,
        meta,
        line: entry.line(),
        end_line: entry.end_line(),
        nested: false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorQuery {
    pub(super) text: String,
    pub(super) line: Option<usize>,
}

impl NetlistNavigatorQuery {
    pub(super) fn new(query: &str) -> Self {
        let text = query.trim().to_lowercase();
        let line_literal = text
            .strip_prefix("line")
            .map(str::trim)
            .unwrap_or(&text)
            .trim_start_matches([':', '#']);
        let line = line_literal.parse::<usize>().ok().filter(|line| *line > 0);
        Self { text, line }
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn matches_text(&self, text: &str) -> bool {
        self.is_empty() || contains_folded(text, &self.text)
    }

    fn matches_entry(&self, entry: &OutlineEntry) -> bool {
        self.is_empty()
            || self
                .line
                .is_some_and(|line| (entry.line()..=entry.end_line()).contains(&line))
            || contains_folded(&entry.label(), &self.text)
    }

    fn matches_group<'a>(
        &self,
        label: &str,
        entries: impl IntoIterator<Item = OutlineEntry<'a>>,
    ) -> bool {
        self.matches_text(label) || entries.into_iter().any(|entry| self.matches_entry(&entry))
    }

    fn matches_diff_hunk(&self, hunk: &DiffHunk) -> bool {
        self.matches_text(&hunk.label) || self.matches_text(&hunk.meta)
    }
}

/// Whether `text` contains an already-lowercased `needle`, ignoring case.
///
/// A filter is tested against every declaration in the deck on every frame,
/// and a lowercase copy of each label is a copy of the deck. ASCII text folds
/// in place; anything else takes the allocating path, where the answer has to
/// match `str::to_lowercase` exactly rather than merely closely.
fn contains_folded(text: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if !text.is_ascii() || !needle.is_ascii() {
        return text.to_lowercase().contains(needle);
    }
    text.len() >= needle.len()
        && text
            .as_bytes()
            .windows(needle.len())
            .any(|window| window.eq_ignore_ascii_case(needle.as_bytes()))
}

/// The outline's own root row, which names the document and nothing else.
pub(super) fn navigator_root_row(
    kind: NetlistNavigatorRowKind,
    label: &str,
    target: Option<OutlineEntry<'_>>,
    entries: &[OutlineEntry<'_>],
) -> NetlistNavigatorRow {
    NetlistNavigatorRow {
        meta: None,
        ..navigator_group_row(kind, label, String::new(), target, entries)
    }
}

pub(super) fn navigator_group_row(
    kind: NetlistNavigatorRowKind,
    label: &str,
    meta: String,
    target: Option<OutlineEntry<'_>>,
    entries: &[OutlineEntry<'_>],
) -> NetlistNavigatorRow {
    NetlistNavigatorRow {
        kind,
        label: label.to_owned(),
        meta: Some(meta),
        tooltip: None,
        shadowed: false,
        sections: None,
        target_line: target.map(|entry| entry.line()),
        source_ranges: if entries.is_empty() {
            target
                .map(|entry| vec![(entry.line(), entry.end_line())])
                .unwrap_or_default()
        } else {
            entries
                .iter()
                .map(|entry| (entry.line(), entry.end_line()))
                .collect()
        },
    }
}

pub(super) fn include_entry_label(label: &str) -> String {
    label
        .split_once(char::is_whitespace)
        .map_or(label, |(_, locator)| locator)
        .trim_matches(['\'', '"'])
        .to_owned()
}

/// Which of `count` uniform rows a viewport can show, given the offset of the
/// first row from its top. `ScrollArea::show_rows` does this for a scroll area
/// it owns; the outline's declarations share one with the rest of the tree, so
/// the span is computed here and the remainder becomes space.
pub(super) fn visible_row_span(
    first_row_offset: f32,
    viewport_height: f32,
    row_height: f32,
    count: usize,
) -> std::ops::Range<usize> {
    if count == 0 || row_height <= 0.0 {
        return 0..0;
    }
    let first = ((-first_row_offset / row_height).floor()).max(0.0) as usize;
    let first = first.min(count);
    let end = (((viewport_height - first_row_offset) / row_height).ceil()).max(0.0) as usize;
    first..end.clamp(first, count)
}

fn uniform_rows(ui: &mut Ui, height: f32, count: usize, mut row: impl FnMut(&mut Ui, usize)) {
    let clip = ui.clip_rect();
    let span = visible_row_span(ui.cursor().top() - clip.top(), clip.height(), height, count);
    if span.start > 0 {
        ui.add_space(span.start as f32 * height);
    }
    for index in span.clone() {
        row(ui, index);
    }
    if span.end < count {
        ui.add_space((count - span.end) as f32 * height);
    }
}

/// How an outline row is placed: what occupies its left edge and where its
/// label starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NetlistOutlineRowShape {
    /// Icon and label, with the caret column left empty so a root or include
    /// row aligns with the groups above and below it.
    Leaf,
    /// A caret stating the current disclosure, then icon and label.
    Group { expanded: bool },
    /// A declaration under a group: guide line, indent, monospaced label.
    Child,
    /// A declaration under a subcircuit's row in a group: one indent further.
    NestedChild,
    /// A semantic-index row: label and count only, flush to the panel edge.
    Index,
}

pub(super) struct OutlineRowVisual<'a> {
    pub(super) label: &'a str,
    /// The label is a file's path. It gives way from its front, so the
    /// file's name stays in view, and the meta beside it keeps its place.
    pub(super) path: bool,
    pub(super) meta: Option<&'a str>,
    /// Severity tone for the meta column. `None` is the ordinary faint tone;
    /// a row states a colour only when the meta itself is a verdict.
    pub(super) meta_tone: Option<egui::Color32>,
    pub(super) icon: Option<WorkbenchIcon>,
    pub(super) shape: NetlistOutlineRowShape,
    pub(super) selected: bool,
    pub(super) enabled: bool,
    pub(super) height: f32,
    /// What the row says on hover. `None` says the row's whole text when the
    /// row had to cut it, and nothing when it did not.
    pub(super) hover: Option<&'a str>,
}

impl NetlistOutlineRowShape {
    /// Left edge of the label text, measured from the row's left edge.
    pub(super) fn label_left(self) -> f32 {
        match self {
            Self::Index => NETLIST_OUTLINE_PADDING_X,
            Self::Leaf | Self::Group { .. } => {
                NETLIST_OUTLINE_PADDING_X
                    + NETLIST_OUTLINE_CARET_COLUMN
                    + NETLIST_OUTLINE_ICON_SIZE
                    + NETLIST_OUTLINE_ICON_GAP
            }
            Self::Child => {
                NETLIST_OUTLINE_PADDING_X
                    + NETLIST_OUTLINE_CARET_COLUMN
                    + NETLIST_OUTLINE_ICON_SIZE
                    + NETLIST_OUTLINE_ICON_GAP
                    + NETLIST_OUTLINE_CHILD_INDENT
            }
            Self::NestedChild => Self::Child.label_left() + NETLIST_OUTLINE_CHILD_INDENT,
        }
    }
}

pub(super) fn netlist_outline_row(ui: &mut Ui, row: OutlineRowVisual<'_>) -> Response {
    netlist_outline_row_beside(ui, row, 0.0)
}

/// An outline row whose last `trailing` points are a control's, drawn over
/// the row once it is laid: the label and the meta stop short of it.
fn netlist_outline_row_beside(ui: &mut Ui, row: OutlineRowVisual<'_>, trailing: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row.height),
        if row.enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let label = row.label.to_owned();
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            row.enabled,
            row.selected,
            label.clone(),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    if row.selected || (row.enabled && response.hovered()) {
        ui.painter().rect_filled(
            rect,
            0.0,
            if row.selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if row.selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }

    let foreground = if !row.enabled {
        t.color.text_faint
    } else if row.selected {
        t.color.text
    } else {
        t.color.text_dim
    };
    let icon_center_x = rect.left()
        + NETLIST_OUTLINE_PADDING_X
        + NETLIST_OUTLINE_CARET_COLUMN
        + NETLIST_OUTLINE_ICON_SIZE * 0.5;
    if let NetlistOutlineRowShape::Group { expanded } = row.shape {
        let caret = if expanded {
            WorkbenchIcon::ChevronDown
        } else {
            WorkbenchIcon::ChevronRight
        };
        caret.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    rect.left() + NETLIST_OUTLINE_PADDING_X + NETLIST_OUTLINE_CARET_COLUMN * 0.5,
                    rect.center().y,
                ),
                egui::Vec2::splat(NETLIST_OUTLINE_CARET_SIZE),
            ),
            t.color.text_faint,
        );
    }
    if matches!(
        row.shape,
        NetlistOutlineRowShape::Child | NetlistOutlineRowShape::NestedChild
    ) {
        ui.painter().vline(
            icon_center_x,
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
    if row.shape == NetlistOutlineRowShape::NestedChild {
        ui.painter().vline(
            icon_center_x + NETLIST_OUTLINE_CHILD_INDENT,
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
    if let Some(icon) = row.icon {
        icon.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(icon_center_x, rect.center().y),
                egui::Vec2::splat(NETLIST_OUTLINE_ICON_SIZE),
            ),
            foreground,
        );
    }

    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let label_font = if matches!(
        row.shape,
        NetlistOutlineRowShape::Child | NetlistOutlineRowShape::NestedChild
    ) {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    let label_left = rect.left() + row.shape.label_left();
    // The control over the row's end keeps its `trailing` points.
    let right = rect.right() - trailing;
    let room = (right - NETLIST_OUTLINE_PADDING_X - label_left).max(0.0);
    let text_width = |text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), foreground)
            .size()
            .x
    };
    let (label, meta) = if row.path {
        // A path is read by its file's name, and the meta says where the
        // card is written: the meta keeps its place, and the path gives way
        // from its front.
        let meta = row
            .meta
            .map(|meta| crate::ui::panels::elide_text(ui, meta, &meta_font, room));
        let beside = meta
            .as_deref()
            .filter(|meta| !meta.is_empty())
            .map_or(0.0, |meta| {
                text_width(meta, &meta_font) + NETLIST_OUTLINE_ICON_GAP
            });
        let label = crate::ui::panels::elide_path_front(ui, row.label, &label_font, room - beside);
        (label, meta)
    } else {
        // The name is what the row is: it keeps its whole width, and the
        // value beside it is what gives way, ending in an ellipsis.
        let label = crate::ui::panels::elide_text(ui, row.label, &label_font, room);
        let label_width = text_width(&label, &label_font);
        let meta = row.meta.map(|meta| {
            crate::ui::panels::elide_text(
                ui,
                meta,
                &meta_font,
                room - label_width - NETLIST_OUTLINE_ICON_GAP,
            )
        });
        (label, meta)
    };
    let cut = label != row.label || meta.as_deref() != row.meta;
    ui.painter().with_clip_rect(rect).text(
        egui::pos2(label_left, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &label,
        label_font,
        foreground,
    );
    if let Some(meta) = meta.filter(|meta| !meta.is_empty()) {
        ui.painter().text(
            egui::pos2(right - NETLIST_OUTLINE_PADDING_X, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            meta,
            meta_font,
            row.meta_tone.unwrap_or(t.color.text_faint),
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    match (row.hover, row.meta) {
        (Some(hover), _) => response.on_hover_text(hover),
        (None, Some(meta)) if cut => response.on_hover_text(format!("{} {meta}", row.label)),
        (None, None) if cut => response.on_hover_text(row.label),
        (None, _) => response,
    }
}

pub(super) fn netlist_outline_icon(kind: NetlistNavigatorRowKind) -> WorkbenchIcon {
    match kind {
        NetlistNavigatorRowKind::Root => WorkbenchIcon::Code,
        NetlistNavigatorRowKind::Parameters => WorkbenchIcon::Sliders,
        NetlistNavigatorRowKind::Instances => WorkbenchIcon::Component,
        NetlistNavigatorRowKind::Models => WorkbenchIcon::Models,
        NetlistNavigatorRowKind::Analyses => WorkbenchIcon::Simulate,
        NetlistNavigatorRowKind::Measurements => WorkbenchIcon::Target,
        NetlistNavigatorRowKind::Subcircuits => WorkbenchIcon::Layers,
        NetlistNavigatorRowKind::Options => WorkbenchIcon::Settings,
        NetlistNavigatorRowKind::Outputs => WorkbenchIcon::Probe,
        NetlistNavigatorRowKind::Functions => WorkbenchIcon::Format,
        NetlistNavigatorRowKind::Globals => WorkbenchIcon::Supply,
        NetlistNavigatorRowKind::OtherCards => WorkbenchIcon::More,
        NetlistNavigatorRowKind::Include => WorkbenchIcon::File,
    }
}

/// The active line's provenance over the model and the message catalogue.
/// Going to what the line was written from is the editor's: Show in
/// schematic on the line itself.
pub(super) fn netlist_source_mapping(
    ui: &mut Ui,
    model: &Model,
    messages: MessageCatalog,
    active_line: usize,
) {
    let t = Tokens::get(ui.ctx());
    let mapping = model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .and_then(|artifact| artifact.source_map_entry(active_line))
        .map(|entry| {
            (
                entry.cell_identity().to_owned(),
                entry.view_identity().to_owned(),
                entry.instance_identity().map(str::to_owned),
            )
        });
    let Some((cell, view, instance)) = mapping else {
        muted(ui, &messages.text(MessageId::NetlistNavigatorNoProvenance));
        return;
    };
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 8,
            bottom: 8,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.label(
                egui::RichText::new(messages.format(
                    MessageId::NetlistNavigatorMappedLine,
                    &[("line", &active_line.to_string()), ("cell", &cell)],
                ))
                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                .color(t.color.text_dim),
            );
            ui.label(
                egui::RichText::new(view.as_str())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            if let Some(instance) = instance.as_deref() {
                ui.label(
                    egui::RichText::new(messages.format(
                        MessageId::NetlistNavigatorInstance,
                        &[("instance", instance)],
                    ))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
                );
            }
        });
}

#[cfg(test)]
mod tests;
