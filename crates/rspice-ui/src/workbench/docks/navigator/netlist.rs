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
use crate::state::{OutlineSection, OutlineSectionKind};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::{MessageCatalog, MessageId};

mod lib_sections;

/// Width reserved for a disclosure caret. Root and include rows leave it
/// empty so their icons line up with the groups' rather than sliding left.
const NETLIST_OUTLINE_CARET_COLUMN: f32 = 13.0;
const NETLIST_OUTLINE_CARET_SIZE: f32 = 9.0;
/// Indent of a declaration under its group, past the guide line.
const NETLIST_OUTLINE_CHILD_INDENT: f32 = 12.0;

pub(super) fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.ui.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        netlist_diff(ui, app);
        return;
    }
    let root_label = active_netlist_artifact_name(&app.state);
    let messages = app.state.ui.messages();
    let index = crate::workbench::documents::netlist_document::visible_source_index(&mut app.state);
    lib_sections::refresh(&mut app.state);
    let projection = NetlistNavigatorProjection::from_index(
        &index,
        app.state.workbench.navigator_filter(),
        &root_label,
        app.state.ui.netlist.active_document == ActiveNetlistDocument::Generated,
        &app.state.workbench.netlist_outline_collapsed,
        &retained_include_facts(&app.state, messages),
        messages,
    );
    let project_index =
        crate::workbench::documents::netlist_document::language::project_index(&mut app.state);
    let symbol_query = app
        .state
        .workbench
        .navigator_filter()
        .trim()
        .to_ascii_lowercase();
    let matching_symbol_indices = project_index.as_ref().map(|index| {
        if symbol_query.is_empty() {
            Vec::new()
        } else {
            index
                .symbols()
                .iter()
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
                .iter()
                .enumerate()
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
    let matching_hierarchy_indices = project_index.as_ref().map(|index| {
        index
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
    let active_line = app.state.ui.netlist.cursor_line.saturating_add(1);
    let height = outline_row_height(ui, &app.state);

    let mut goto = None;
    let mut goto_project_location = None;
    let mut toggled = None;
    let mut open_include = None;
    let mut use_section = None;
    // A section rewrite edits the authored card, so a generated or snapshot
    // document offers the choice and states why it cannot take it rather than
    // hiding what the library declares.
    let source_editable =
        crate::workbench::documents::netlist_document::active_netlist_source_is_editable(
            &app.state,
        );

    ScrollArea::vertical()
        .id_salt("workbench.netlist.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            if projection.root_row.is_some() || !projection.groups.is_empty() {
                section_header(
                    ui,
                    &messages.text(MessageId::NetlistNavigatorStructure),
                    Some(&messages.format(
                        if projection.line_count == 1 {
                            MessageId::NetlistNavigatorLineSingular
                        } else {
                            MessageId::NetlistNavigatorLines
                        },
                        &[("count", &projection.line_count.to_string())],
                    )),
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
                if group.declarations() == 0 {
                    outline_child_note(ui, &messages.text(group.empty_note), height);
                    continue;
                }
                uniform_rows(ui, height, group.declarations(), |ui, position| {
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
                            shape: NetlistOutlineRowShape::Child,
                            selected: child.contains_line(active_line),
                            enabled: true,
                            height,
                        },
                    )
                    .clicked()
                    {
                        goto = Some(child.line);
                    }
                });
            }

            if !projection.include_rows.is_empty() {
                section_header(
                    ui,
                    &messages.text(MessageId::NetlistNavigatorIncludes),
                    Some(&messages.text(netlist_dependency_status(&app.state))),
                );
                for row in &projection.include_rows {
                    let mut response = netlist_outline_row(
                        ui,
                        OutlineRowVisual {
                            label: &row.label,
                            meta: row.meta.as_deref(),
                            meta_tone: row.shadowed.then(|| Tokens::get(ui.ctx()).color.warn),
                            icon: Some(netlist_outline_icon(row.kind)),
                            shape: NetlistOutlineRowShape::Leaf,
                            selected: row.contains_line(active_line),
                            enabled: row.target_line.is_some(),
                            height,
                        },
                    );
                    if let Some(tooltip) = row.tooltip.as_deref() {
                        response = response.on_hover_text(tooltip);
                    }
                    if let Some(choice) = row.sections.as_ref() {
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
                    let hierarchy_meta = if project_index.hierarchy_truncated() {
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
                            .and_then(|index| project_index.hierarchy().get(*index))
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
                        project_index.symbols().len()
                    } else {
                        matching.len()
                    }
                });
                if symbol_count > 0 {
                    section_header(
                        ui,
                        &messages.text(MessageId::NetlistNavigatorProjectSymbols),
                        Some(&messages.format(
                            if symbol_count == 1 {
                                MessageId::NetlistNavigatorSymbolSingular
                            } else {
                                MessageId::NetlistNavigatorSymbols
                            },
                            &[
                                ("count", &symbol_count.to_string()),
                                ("sources", &project_index.source_count().to_string()),
                            ],
                        )),
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
                        let Some(symbol) = project_index.symbols().get(symbol_index) else {
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
                            .and_then(|index| project_index.references().get(*index))
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
                            },
                        )
                        .clicked()
                        {
                            goto_project_location = Some(reference.location.clone());
                        }
                    });
                }
            }

            if !projection.semantic_rows.is_empty() {
                section_header(
                    ui,
                    &messages.text(MessageId::NetlistNavigatorSemanticIndex),
                    Some(&messages.format(
                        if projection.semantic_cards == 1 {
                            MessageId::NetlistNavigatorDirectiveSingular
                        } else {
                            MessageId::NetlistNavigatorDirectives
                        },
                        &[("count", &projection.semantic_cards.to_string())],
                    )),
                );
                for row in &projection.semantic_rows {
                    if netlist_outline_row(
                        ui,
                        OutlineRowVisual {
                            label: &row.label,
                            meta: Some(&row.meta),
                            meta_tone: None,
                            icon: None,
                            shape: NetlistOutlineRowShape::Index,
                            selected: false,
                            enabled: true,
                            height,
                        },
                    )
                    .clicked()
                    {
                        goto = Some(row.line);
                    }
                }
            }

            if projection.show_source_mapping {
                section_header(
                    ui,
                    &messages.text(MessageId::NetlistNavigatorSourceMapping),
                    None,
                );
                netlist_source_mapping(ui, app, active_line);
            }

            if projection.is_empty() {
                muted(ui, &messages.text(MessageId::NetlistNavigatorNoMatches));
            }
        });

    if let Some(section) = toggled {
        // Collapsing is stored, expanding clears the store, so the default for
        // a category the engineer has never touched stays "disclosed".
        let collapsed = &mut app.state.workbench.netlist_outline_collapsed;
        if !collapsed.remove(&section) {
            collapsed.insert(section);
        }
    }
    if let Some(line) = goto {
        // Keep selection feedback immediate and hand the exact one-based
        // declaration to the editor's caret/scroll transaction for the next
        // document frame.
        app.state.ui.netlist.cursor_line = line.saturating_sub(1);
        app.state.ui.netlist.requested_line = Some(line);
    }
    if let Some((line, section)) = use_section {
        match lib_sections::use_section(&mut app.state, line, &section) {
            Ok(applied) => app
                .state
                .push_user_message(crate::diagnostics::ConsoleMessage::info(applied)),
            Err(error) => app
                .state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(error)),
        }
    }
    if let Some(requested) = open_include {
        let dependency = active_canonical_netlist_document(&app.state)
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
                    crate::workbench::documents::netlist_document::open_netlist_dependency(
                        &mut app.state,
                        &identity,
                    )
                {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
                }
            }
            Some((identity, false)) => {
                crate::workbench::workflows::netlist_workflow::request_dependency_relink(
                    &mut app.state,
                    &identity,
                );
            }
            None => {
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                        "Include {requested:?} is not retained in the active dependency closure."
                    )));
            }
        }
    }
    if let Some(location) = goto_project_location
        && let Err(error) =
            crate::workbench::documents::netlist_document::language::open_project_location(
                &mut app.state,
                &location,
            )
    {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
    }
}

fn outline_row_height(ui: &Ui, state: &crate::workbench::AppState) -> f32 {
    if state.workbench.coarse_pointer || ui.ctx().content_rect().width() <= 820.0 {
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
        },
    );
    let expanded = group.expanded;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(expanded);
    });
    response.clicked()
}

pub(super) fn active_netlist_artifact_name(state: &crate::workbench::AppState) -> String {
    match state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => "generated.sp".to_owned(),
        ActiveNetlistDocument::OwnedSource => state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .or_else(|| {
                state
                    .workspace
                    .netlist_source_path
                    .as_deref()
                    .and_then(std::path::Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "owned-source.sp".to_owned()),
        ActiveNetlistDocument::GeneratedDiff => "generated.diff".to_owned(),
        // A synthetic filename, like the two above it: an identifier, never
        // translated copy.
        ActiveNetlistDocument::RunSnapshot => {
            crate::workbench::documents::netlist_document::run_deck_snapshot_artifact_name(state)
        }
    }
}

pub(super) fn active_canonical_netlist_document(
    state: &crate::workbench::AppState,
) -> Option<&crate::state::NetlistDocument> {
    match state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => state.ui.netlist.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => state.ui.netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => None,
    }
}

/// What the canonical document and the retained resolution trace know about
/// one include row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IncludeRowFacts {
    /// The locator the include card wrote.
    pub(super) locator: String,
    /// The closure's verdict for this dependency.
    pub(super) state: MessageId,
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

/// The whole ordered chain, winner marked, for a row's tooltip.
fn include_chain_text(
    messages: MessageCatalog,
    locator: &str,
    resolution: &rspice_core::netlist::IncludeResolution,
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
            include_stage_text(messages, candidate.stage()),
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
    let models = messages.format(
        if section.model_count == 1 {
            MessageId::NetlistLibSectionModel
        } else {
            MessageId::NetlistLibSectionModels
        },
        &[("count", &section.model_count.to_string())],
    );
    if section.subcircuit_count == 0 {
        return format!("{} · {models}", section.name);
    }
    let subcircuits = messages.format(
        if section.subcircuit_count == 1 {
            MessageId::NetlistLibSectionSubcircuit
        } else {
            MessageId::NetlistLibSectionSubcircuits
        },
        &[("count", &section.subcircuit_count.to_string())],
    );
    format!("{} · {models} · {subcircuits}", section.name)
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
    state: &crate::workbench::AppState,
    messages: MessageCatalog,
) -> Vec<IncludeRowFacts> {
    active_canonical_netlist_document(state).map_or_else(Vec::new, |document| {
        document
            .dependencies()
            .iter()
            .map(|dependency| {
                let locator = dependency.requested_locator().to_owned();
                let resolution = state.ui.code_workspace.include_resolutions.get(&locator);
                IncludeRowFacts {
                    state: match dependency.resolution() {
                        crate::state::DependencyResolution::Missing { .. } => {
                            MessageId::NetlistNavigatorDependencyMissing
                        }
                        crate::state::DependencyResolution::Unresolved => {
                            MessageId::NetlistNavigatorDependencyUnresolved
                        }
                        crate::state::DependencyResolution::Resolved { .. } => {
                            dependency_authority_message(dependency.authority())
                        }
                    },
                    via: resolution
                        .map(|resolution| include_stage_text(messages, resolution.stage())),
                    chain: resolution
                        .map(|resolution| include_chain_text(messages, &locator, resolution)),
                    shadowed_by: resolution.and_then(|resolution| {
                        resolution
                            .shadowed()
                            .next()
                            .map(|candidate| candidate.path().display().to_string())
                    }),
                    sections: lib_sections::declared(state, &locator),
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

pub(super) fn netlist_dependency_status(state: &crate::workbench::AppState) -> MessageId {
    let Some(document) = active_canonical_netlist_document(state) else {
        return MessageId::NetlistNavigatorDependencyUnavailable;
    };
    if document.dependencies().iter().any(|dependency| {
        matches!(
            dependency.resolution(),
            crate::state::DependencyResolution::Missing { .. }
        )
    }) {
        MessageId::NetlistNavigatorDependencyError
    } else if document.dependency_graph_is_sealed() {
        MessageId::NetlistNavigatorDependencyResolved
    } else {
        MessageId::NetlistNavigatorDependencyUnresolved
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

pub(super) fn netlist_diff(ui: &mut Ui, app: &mut RSpiceApp) {
    let messages = app.state.ui.messages();
    let query = NetlistNavigatorQuery::new(app.state.workbench.navigator_filter());
    let hunks = diff_hunks(&app.state.ui.netlist.generated_diff_source, messages);
    let (additions, removals) = diff_totals(&app.state.ui.netlist.generated_diff_source);
    let matching = hunks
        .iter()
        .filter(|hunk| query.matches_diff_hunk(hunk))
        .cloned()
        .collect::<Vec<_>>();
    let height = outline_row_height(ui, &app.state);
    let active_line = app.state.ui.netlist.cursor_line.saturating_add(1);
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
                    },
                )
                .clicked()
                {
                    goto = Some(hunk.line);
                }
            });
        });

    if let Some(line) = goto {
        app.state.ui.netlist.cursor_line = line.saturating_sub(1);
        app.state.ui.netlist.requested_line = Some(line);
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
    diff.lines().fold((0, 0), |(added, removed), line| {
        if line.starts_with('+') && !line.starts_with("+++") {
            (added + 1, removed)
        } else if line.starts_with('-') && !line.starts_with("---") {
            (added, removed + 1)
        } else {
            (added, removed)
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NetlistNavigatorRowKind {
    Root,
    Parameters,
    Instances,
    Models,
    Analyses,
    Measurements,
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

/// One declaration under a structure group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistOutlineChild {
    pub(super) label: String,
    pub(super) meta: Option<String>,
    pub(super) line: usize,
    pub(super) end_line: usize,
}

impl NetlistOutlineChild {
    fn contains_line(&self, line: usize) -> bool {
        (self.line..=self.end_line).contains(&line)
    }
}

/// A disclosable structure group and the declarations it holds.
///
/// The declarations are held as outline indices rather than as rows. Naming
/// one means tokenizing its card, and a deck that declares fifty thousand
/// devices would pay for all of them on every frame to show forty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistOutlineGroup {
    pub(super) row: NetlistNavigatorRow,
    pub(super) section: OutlineSectionKind,
    /// Indices into the source index's entries, in source order.
    entries: Vec<usize>,
    pub(super) expanded: bool,
    /// What the deck does not declare, said in the group's own place. An empty
    /// group and a filtered-out group must not look alike.
    pub(super) empty_note: MessageId,
}

impl NetlistOutlineGroup {
    /// How many declarations this group discloses, which is what its count
    /// states and how many rows it occupies.
    pub(super) fn declarations(&self) -> usize {
        self.entries.len()
    }

    /// The declaration at `position`, named from its own card.
    pub(super) fn child(
        &self,
        position: usize,
        index: &crate::state::NetlistSourceIndex,
    ) -> Option<NetlistOutlineChild> {
        let entry = index
            .outline()
            .entries()
            .get(*self.entries.get(position)?)?;
        Some(outline_child(self.section, entry, index.card(entry.line())))
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

/// A parsed category the structure tree does not promote to a group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistSemanticRow {
    pub(super) label: String,
    pub(super) meta: String,
    pub(super) line: usize,
}

struct GroupSpec {
    kind: NetlistNavigatorRowKind,
    section: OutlineSectionKind,
    label: MessageId,
    empty_note: MessageId,
}

const STRUCTURE_GROUPS: [GroupSpec; 5] = [
    GroupSpec {
        kind: NetlistNavigatorRowKind::Parameters,
        section: OutlineSectionKind::Parameters,
        label: MessageId::NetlistNavigatorParameters,
        empty_note: MessageId::NetlistNavigatorNoParameters,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Instances,
        section: OutlineSectionKind::Devices,
        label: MessageId::NetlistNavigatorInstances,
        empty_note: MessageId::NetlistNavigatorNoInstances,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Models,
        section: OutlineSectionKind::Models,
        label: MessageId::NetlistNavigatorModels,
        empty_note: MessageId::NetlistNavigatorNoModels,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Analyses,
        section: OutlineSectionKind::Analyses,
        label: MessageId::NetlistNavigatorAnalyses,
        empty_note: MessageId::NetlistNavigatorNoAnalyses,
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Measurements,
        section: OutlineSectionKind::Measurements,
        label: MessageId::NetlistNavigatorMeasurements,
        empty_note: MessageId::NetlistNavigatorNoMeasurements,
    },
];

struct IndexSpec {
    section: OutlineSectionKind,
    label: MessageId,
    unit: MessageId,
    units: MessageId,
}

const SEMANTIC_INDEX: [IndexSpec; 7] = [
    IndexSpec {
        section: OutlineSectionKind::Subcircuits,
        label: MessageId::NetlistNavigatorHierarchy,
        unit: MessageId::NetlistNavigatorUnitDefinition,
        units: MessageId::NetlistNavigatorUnitDefinitions,
    },
    IndexSpec {
        section: OutlineSectionKind::Globals,
        label: MessageId::NetlistNavigatorGlobals,
        unit: MessageId::NetlistNavigatorUnitDeclaration,
        units: MessageId::NetlistNavigatorUnitDeclarations,
    },
    IndexSpec {
        section: OutlineSectionKind::Functions,
        label: MessageId::NetlistNavigatorFunctions,
        unit: MessageId::NetlistNavigatorUnitDefinition,
        units: MessageId::NetlistNavigatorUnitDefinitions,
    },
    IndexSpec {
        section: OutlineSectionKind::Options,
        label: MessageId::NetlistNavigatorOptions,
        unit: MessageId::NetlistNavigatorUnitCard,
        units: MessageId::NetlistNavigatorUnitCards,
    },
    IndexSpec {
        section: OutlineSectionKind::Outputs,
        label: MessageId::NetlistNavigatorOutputs,
        unit: MessageId::NetlistNavigatorUnitDirective,
        units: MessageId::NetlistNavigatorUnitDirectives,
    },
    IndexSpec {
        section: OutlineSectionKind::Conditionals,
        label: MessageId::NetlistNavigatorConditionals,
        unit: MessageId::NetlistNavigatorUnitCard,
        units: MessageId::NetlistNavigatorUnitCards,
    },
    IndexSpec {
        section: OutlineSectionKind::Controls,
        label: MessageId::NetlistNavigatorControls,
        unit: MessageId::NetlistNavigatorUnitDirective,
        units: MessageId::NetlistNavigatorUnitDirectives,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorProjection {
    pub(super) line_count: usize,
    pub(super) root_row: Option<NetlistNavigatorRow>,
    pub(super) groups: Vec<NetlistOutlineGroup>,
    pub(super) include_rows: Vec<NetlistNavigatorRow>,
    pub(super) semantic_rows: Vec<NetlistSemanticRow>,
    /// Cards covered by the semantic index, stated in its header so the
    /// section's own claim can be checked against the deck.
    pub(super) semantic_cards: usize,
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
        let section_of = |kind: OutlineSectionKind| -> &[usize] {
            outline
                .sections()
                .iter()
                .find(|section| section.kind() == kind)
                .map_or(&[][..], OutlineSection::entry_indices)
        };
        let resolved = |indices: &[usize]| -> Vec<&OutlineEntry> {
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
                navigator_group_row(
                    NetlistNavigatorRowKind::Root,
                    root_label,
                    messages.text(if source_mapped {
                        MessageId::NetlistNavigatorRoot
                    } else {
                        MessageId::NetlistNavigatorSourceOfTruth
                    }),
                    root_target,
                    &root_entries,
                )
            });

        let groups = STRUCTURE_GROUPS
            .iter()
            .filter_map(|spec| {
                outline_group(
                    spec,
                    section_of(spec.section),
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
                let label = include_entry_label(entry.label());
                let facts = include_states.iter().find(|facts| facts.locator == label);
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
                        tooltip: Some(messages.text(MessageId::NetlistIncludeChainUntraced)),
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
                    selected: lib_sections::selected(index.card(entry.line())),
                    available: facts.sections.clone(),
                });
                // The dock is 312 px wide and the include's own name is what
                // the row is for, so the meta column states one short phrase:
                // the warning if there is one, else which section of a
                // sectioned library is bound, else where the file came from,
                // else the closure's verdict. Everything else is the tooltip.
                let unresolved = matches!(
                    facts.state,
                    MessageId::NetlistNavigatorDependencyMissing
                        | MessageId::NetlistNavigatorDependencyUnresolved
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
                    _ => messages.text(facts.state),
                };
                let mut tooltip = messages.text(facts.state);
                tooltip.push('\n');
                tooltip.push_str(
                    &facts
                        .chain
                        .clone()
                        .unwrap_or_else(|| messages.text(MessageId::NetlistIncludeChainUntraced)),
                );
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

        let mut semantic_cards = 0usize;
        let semantic_rows = SEMANTIC_INDEX
            .iter()
            .filter_map(|spec| {
                let section = section_of(spec.section);
                let entry_of = |position: &usize| entries.get(*position);
                let counted = if spec.section == OutlineSectionKind::Subcircuits {
                    // The section holds both ends of every definition; only the
                    // opening card is a definition.
                    section
                        .iter()
                        .filter_map(entry_of)
                        .filter(|entry| entry.kind() == OutlineEntryKind::Subcircuit)
                        .count()
                } else {
                    section.len()
                };
                // The filter matches what the reader can see, so it is compared
                // against the drawn label rather than the English one behind it.
                let label = messages.text(spec.label);
                if counted == 0 || !query.matches_group(&label, section.iter().filter_map(entry_of))
                {
                    return None;
                }
                // The header has to be the sum of the rows under it, or a
                // reader who adds them up finds the section disagreeing
                // with itself.
                semantic_cards += counted;
                Some(NetlistSemanticRow {
                    label,
                    meta: messages.format(
                        MessageId::NetlistNavigatorCounted,
                        &[
                            ("count", &counted.to_string()),
                            (
                                "unit",
                                &messages.text(if counted == 1 { spec.unit } else { spec.units }),
                            ),
                        ],
                    ),
                    line: section
                        .first()
                        .and_then(entry_of)
                        .map_or(1, OutlineEntry::line),
                })
            })
            .collect::<Vec<_>>();

        Self {
            line_count: index.line_count(),
            root_row,
            groups,
            include_rows,
            semantic_rows,
            semantic_cards,
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
            && self.semantic_rows.is_empty()
            && !self.show_source_mapping
    }
}

fn outline_group(
    spec: &GroupSpec,
    section: &[usize],
    entries: &[OutlineEntry],
    query: &NetlistNavigatorQuery,
    collapsed: &BTreeSet<OutlineSectionKind>,
    messages: MessageCatalog,
) -> Option<NetlistOutlineGroup> {
    let entry_of = |position: &usize| entries.get(*position);
    // The filter is matched against the label the reader can see. Matching the
    // English behind it would leave a translated navigator unsearchable by the
    // names it displays.
    let label = messages.text(spec.label);
    if !query.matches_group(&label, section.iter().filter_map(entry_of)) {
        return None;
    }
    // A filter names the declarations it kept, so the group discloses them
    // whatever the stored preference says.
    let filtering = !query.is_empty() && !query.matches_text(&label);
    let expanded = filtering || !collapsed.contains(&spec.section);
    let selected = if filtering {
        section
            .iter()
            .copied()
            .filter(|position| entry_of(position).is_some_and(|entry| query.matches_entry(entry)))
            .collect::<Vec<_>>()
    } else {
        section.to_vec()
    };
    let meta = if filtering {
        format!("{} of {}", selected.len(), section.len())
    } else {
        section.len().to_string()
    };
    let target = selected
        .first()
        .or_else(|| section.first())
        .and_then(entry_of);
    Some(NetlistOutlineGroup {
        row: NetlistNavigatorRow {
            kind: spec.kind,
            label,
            meta: Some(meta),
            tooltip: None,
            shadowed: false,
            sections: None,
            target_line: target.map(OutlineEntry::line),
            // Containment is answered by the group from its own entries. A
            // span per declaration here would allocate the whole deck once a
            // frame to settle one question about one line.
            source_ranges: Vec::new(),
        },
        section: spec.section,
        entries: selected,
        expanded,
        empty_note: spec.empty_note,
    })
}

/// Name one declaration and what it binds to. The outline entry keeps a head
/// and one detail token, which names a card but does not describe it, so the
/// binding is read back off the card itself.
fn outline_child(
    section: OutlineSectionKind,
    entry: &OutlineEntry,
    card: &str,
) -> NetlistOutlineChild {
    let tokens = crate::state::card_tokens(card);
    let (label, meta) = match section {
        OutlineSectionKind::Parameters => parameter_child(&tokens),
        OutlineSectionKind::Devices => (
            entry.label().to_owned(),
            device_binding(&tokens).map(str::to_owned),
        ),
        OutlineSectionKind::Models => (
            tokens
                .get(1)
                .cloned()
                .unwrap_or_else(|| entry.label().to_owned()),
            positional_tokens(&tokens).get(2).cloned(),
        ),
        OutlineSectionKind::Analyses => (
            tokens
                .first()
                .cloned()
                .unwrap_or_else(|| entry.label().to_owned()),
            (tokens.len() > 1).then(|| tokens[1..].join(" ")),
        ),
        OutlineSectionKind::Measurements => (
            tokens
                .get(2)
                .cloned()
                .unwrap_or_else(|| entry.label().to_owned()),
            tokens.get(1).cloned(),
        ),
        _ => (entry.label().to_owned(), None),
    };
    NetlistOutlineChild {
        label,
        meta,
        line: entry.line(),
        end_line: entry.end_line(),
    }
}

/// A `.param` card declares one or more names. One name is shown with the
/// expression it is bound to; several are listed without one, because the
/// row would otherwise attribute the first value to all of them.
fn parameter_child(tokens: &[String]) -> (String, Option<String>) {
    let assignments = parameter_assignments(tokens);
    match assignments.as_slice() {
        [] => (tokens.join(" "), None),
        [(name, value)] => (name.clone(), value.clone()),
        several => (
            several
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            None,
        ),
    }
}

/// The names a card assigns, each with its value where the deck wrote one the
/// row can quote whole. SPICE accepts `n=v`, `n =v`, `n= v` and `n = v`, and
/// all four have to read the same way.
fn parameter_assignments(tokens: &[String]) -> Vec<(String, Option<String>)> {
    // Reduce the card to lexemes and the `=` between them, so the four
    // spellings all become the same three-lexeme sequence.
    let mut lexemes = Vec::<&str>::new();
    for token in tokens.iter().skip(1) {
        let mut rest = token.as_str();
        while let Some(at) = rest.find('=') {
            if at > 0 {
                lexemes.push(&rest[..at]);
            }
            lexemes.push("=");
            rest = &rest[at + 1..];
        }
        if !rest.is_empty() {
            lexemes.push(rest);
        }
    }
    (1..lexemes.len().saturating_sub(1))
        .filter(|index| lexemes[*index] == "=")
        .filter_map(|index| {
            let (name, value) = (lexemes[index - 1], lexemes[index + 1]);
            (name != "=").then(|| {
                (
                    name.to_owned(),
                    (value != "=" && expression_is_whole(value)).then(|| value.to_owned()),
                )
            })
        })
        .collect()
}

/// Whether a value lexeme is the whole expression the deck wrote.
///
/// `gain = {2 * k}` arrives here already split on whitespace, and `{2` is not
/// the value of anything — a row that showed it would misquote the deck.
fn expression_is_whole(value: &str) -> bool {
    let mut depth = 0i32;
    for character in value.chars() {
        match character {
            '{' | '(' => depth += 1,
            '}' | ')' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return false;
        }
    }
    depth == 0
}

/// Everything before the first `name=value` argument. A scan that only looked
/// for `=` inside a token would read the value of a spaced assignment as one
/// more positional field.
fn positional_tokens(tokens: &[String]) -> &[String] {
    for (index, token) in tokens.iter().enumerate() {
        if token.contains('=')
            || tokens
                .get(index + 1)
                .is_some_and(|next| next.starts_with('='))
        {
            return &tokens[..index];
        }
    }
    tokens
}

/// The model or subcircuit an instance binds to, when the card's element
/// letter fixes its terminal count and therefore says which field that is.
/// Anything else returns nothing rather than naming a node as a model.
fn device_binding(tokens: &[String]) -> Option<&str> {
    let positional = positional_tokens(tokens);
    let letter = positional.first()?.chars().next()?.to_ascii_uppercase();
    let binding = match letter {
        // Two terminals then the value expression.
        'R' | 'C' | 'L' => positional.get(3).map(String::as_str),
        // Fixed terminal count then the model or subcircuit master.
        'X' | 'D' | 'Q' | 'J' | 'Z' | 'M' => {
            let minimum = match letter {
                'X' => 2,
                'D' => 4,
                'M' => 6,
                _ => 5,
            };
            (positional.len() >= minimum)
                .then(|| positional.last())?
                .map(String::as_str)
        }
        _ => None,
    };
    binding.filter(|value| expression_is_whole(value))
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
            || contains_folded(entry.label(), &self.text)
    }

    fn matches_group<'a>(
        &self,
        label: &str,
        entries: impl IntoIterator<Item = &'a OutlineEntry>,
    ) -> bool {
        self.matches_text(label) || entries.into_iter().any(|entry| self.matches_entry(entry))
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

pub(super) fn navigator_group_row(
    kind: NetlistNavigatorRowKind,
    label: &str,
    meta: String,
    target: Option<&OutlineEntry>,
    entries: &[&OutlineEntry],
) -> NetlistNavigatorRow {
    NetlistNavigatorRow {
        kind,
        label: label.to_owned(),
        meta: Some(meta),
        tooltip: None,
        shadowed: false,
        sections: None,
        target_line: target.map(OutlineEntry::line),
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
    /// A semantic-index row: label and count only, flush to the panel edge.
    Index,
}

pub(super) struct OutlineRowVisual<'a> {
    pub(super) label: &'a str,
    pub(super) meta: Option<&'a str>,
    /// Severity tone for the meta column. `None` is the ordinary faint tone;
    /// a row states a colour only when the meta itself is a verdict.
    pub(super) meta_tone: Option<egui::Color32>,
    pub(super) icon: Option<WorkbenchIcon>,
    pub(super) shape: NetlistOutlineRowShape,
    pub(super) selected: bool,
    pub(super) enabled: bool,
    pub(super) height: f32,
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
        }
    }
}

pub(super) fn netlist_outline_row(ui: &mut Ui, row: OutlineRowVisual<'_>) -> Response {
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
    if row.shape == NetlistOutlineRowShape::Child {
        ui.painter().vline(
            icon_center_x,
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
    let meta_width = row.meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(meta.to_owned(), meta_font.clone(), t.color.text_faint)
            .size()
            .x
    });
    let label_left = rect.left() + row.shape.label_left();
    let label_right = if row.meta.is_some() {
        rect.right() - NETLIST_OUTLINE_PADDING_X - meta_width - NETLIST_OUTLINE_ICON_GAP
    } else {
        rect.right() - NETLIST_OUTLINE_PADDING_X
    };
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            row.label,
            if row.shape == NetlistOutlineRowShape::Child {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            foreground,
        );
    if let Some(meta) = row.meta {
        ui.painter().text(
            egui::pos2(rect.right() - NETLIST_OUTLINE_PADDING_X, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            meta,
            meta_font,
            row.meta_tone.unwrap_or(t.color.text_faint),
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

/// State, where the declarations would be, that a group holds none.
fn outline_child_note(ui: &mut Ui, note: &str, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), note));
    if !ui.is_rect_visible(rect) {
        return;
    }
    ui.painter().vline(
        rect.left()
            + NETLIST_OUTLINE_PADDING_X
            + NETLIST_OUTLINE_CARET_COLUMN
            + NETLIST_OUTLINE_ICON_SIZE * 0.5,
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(
            rect.left() + NetlistOutlineRowShape::Child.label_left(),
            rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        note,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

pub(super) fn netlist_outline_icon(kind: NetlistNavigatorRowKind) -> WorkbenchIcon {
    match kind {
        NetlistNavigatorRowKind::Root => WorkbenchIcon::Code,
        NetlistNavigatorRowKind::Parameters => WorkbenchIcon::Sliders,
        NetlistNavigatorRowKind::Instances => WorkbenchIcon::Component,
        NetlistNavigatorRowKind::Models => WorkbenchIcon::Models,
        NetlistNavigatorRowKind::Analyses => WorkbenchIcon::Simulate,
        NetlistNavigatorRowKind::Measurements => WorkbenchIcon::Target,
        NetlistNavigatorRowKind::Include => WorkbenchIcon::File,
    }
}

pub(super) fn netlist_source_mapping(ui: &mut Ui, app: &mut RSpiceApp, active_line: usize) {
    let t = Tokens::get(ui.ctx());
    let mapping = app
        .state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .and_then(|artifact| artifact.source_map_entry(active_line))
        .map(|entry| {
            (
                entry.cell_identity().to_owned(),
                entry.view_identity().to_owned(),
                entry.instance_identity().map(str::to_owned),
                entry.component_id(),
            )
        });
    let messages = app.state.ui.messages();
    let Some((cell, view, instance, component_id)) = mapping else {
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
                egui::RichText::new(view)
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
            if let Some(component_id) = component_id
                && ui
                    .button(messages.text(MessageId::NetlistNavigatorCrossProbe))
                    .clicked()
            {
                app.state.schematic.selection.clear();
                app.state.schematic.selection.select_component(component_id);
                let announced = messages.format(
                    MessageId::NetlistNavigatorCrossProbeSucceeded,
                    &[
                        ("line", &active_line.to_string()),
                        ("component", &component_id.to_string()),
                    ],
                );
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::info(announced));
            }
        });
}

/// A navigator row that states, in the row's own place, why a section is
/// empty — a filtered-out list and an empty design must never look alike.
pub(super) fn empty_navigator_row(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width().max(1.0), t.metrics.row_h.max(29.0)),
        egui::Sense::hover(),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        message,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), message)
    });
}

pub(super) fn nav_row(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
) -> bool {
    nav_row_response(ui, icon, label, selected, meta).clicked()
}

pub(super) fn nav_row_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
) -> Response {
    nav_row_indented_response(ui, icon, label, selected, meta, 0)
}

pub(super) fn nav_row_indented(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> bool {
    nav_row_indented_response(ui, icon, label, selected, meta, level).clicked()
}

pub(super) fn nav_row_indented_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> Response {
    nav_row_indented_styled(ui, icon, label, selected, meta, level, false)
}

pub(super) fn schematic_nav_row_indented_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
    expanded: bool,
    child_guide: bool,
) -> Response {
    let response = nav_row_indented_styled_with_metrics(
        ui,
        icon,
        label,
        selected,
        meta,
        level,
        mono,
        SCHEMATIC_NAV_ROW_HEIGHT,
        SCHEMATIC_NAV_LABEL_SIZE,
        SCHEMATIC_NAV_META_SIZE,
        expanded,
        child_guide,
        // accessibility-pointer-shim: the sense is forwarded to
        // `nav_row_indented_styled_with_metrics`, which allocates the row and
        // owns both its WidgetInfo and its focus ring.
        egui::Sense::click(),
    );
    // One level deeper than the row draws itself: the rail's own level 0 is
    // the section band above it, which is what Left climbs out to.
    super::rail::row(ui, &response, level + 1, None);
    response
}

pub(super) fn schematic_nav_row_indented_drag_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
    expanded: bool,
    child_guide: bool,
) -> Response {
    let response = nav_row_indented_styled_with_metrics(
        ui,
        icon,
        label,
        selected,
        meta,
        level,
        mono,
        SCHEMATIC_NAV_ROW_HEIGHT,
        SCHEMATIC_NAV_LABEL_SIZE,
        SCHEMATIC_NAV_META_SIZE,
        expanded,
        child_guide,
        // accessibility-pointer-shim: same forwarding wrapper, drag variant.
        egui::Sense::click_and_drag(),
    );
    super::rail::row(ui, &response, level + 1, None);
    response
}

#[cfg(test)]
mod tests;
