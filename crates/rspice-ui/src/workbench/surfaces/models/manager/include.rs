//! Specialist Models & PDKs page: includes.

use super::*;

pub(super) fn include_page(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    diagnostics: &ClosureFacts,
) {
    section_title(
        ui,
        "Model include graph",
        &format!(
            "{} files · {} edges · {} diagnostics",
            diagnostics.files,
            diagnostics.edges,
            diagnostics.diagnostics()
        ),
        |ui| {
            // It opens on the findings the last scan produced rather than
            // re-pinning outright: "resolve" used to mean "refresh and hope",
            // which accepted whatever the file had become without ever showing
            // the reader what that was.
            if ui
                .add_enabled(
                    !app.state.workbench.models_view.model_import_in_progress,
                    egui::Button::new("Resolve drift…"),
                )
                .clicked()
            {
                if let Some(library) = current_library_name(app) {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::ResolveDrift { library });
                } else {
                    receipt(
                        app,
                        Err("Select a model source to resolve first.".to_owned()),
                    );
                }
            }
            if ui.button("Export manifest").clicked() {
                export_include_manifest(app);
            }
        },
    );
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        ui.checkbox(
            &mut app.state.workbench.models_view.include_direct_only,
            "Direct dependencies only",
        );
        ui.add(
            egui::TextEdit::singleline(
                &mut app.state.workbench.models_view.include_definition_query,
            )
            .hint_text("Filter definitions or providers…")
            .desired_width(260.0),
        );
    });
    // Both panes below read the libraries while the page also writes selection
    // state, so what crosses that line is the projection each pane paints — a
    // handful of nodes and a name index. Cloning the libraries themselves to
    // dodge the borrow, which is what this did, copied the whole model corpus
    // and its retained source bytes on every frame.
    if app.state.model_library_manager.library_count() == 0 {
        page_empty_state(
            ui,
            "No model-source closure is loaded",
            "Add a library or attach a pack to build an authenticated include graph.",
        );
        return;
    }
    let nodes = closure_nodes(app);
    let definitions = definition_index(app.state);
    include_closure_graph(ui, app, &nodes, diagnostics);
    include_definition_table(ui, app, &definitions);
}

/// Nodes the closure graph draws before it stops.
///
/// A closure can hold hundreds of sources and this pane is a few hundred
/// pixels tall. What is dropped is always reported as a count — a graph that
/// silently stops at twelve reads as a complete graph of twelve.
const GRAPH_NODE_LIMIT: usize = 12;

/// Whether a source is the library's root or something the root includes
/// itself, as opposed to a transitive member reached through another file.
fn is_direct_closure_member(library: &ModelLibrary, path: &Path) -> bool {
    let Some(root) = library.root_path.as_deref() else {
        return false;
    };
    root == path
        || library
            .source_edges
            .iter()
            .any(|edge| edge.owner == root && edge.target == path)
}

/// One retained source the closure graph draws.
struct ClosureNode {
    path: PathBuf,
    library: String,
    digest: String,
}

/// The nodes and edges the graph pane draws, and what it left out.
struct ClosureGraph {
    nodes: Vec<ClosureNode>,
    /// Sources that passed the filter, drawn or not.
    matching: usize,
    /// Edges between two drawn nodes.
    edges: Vec<(PathBuf, PathBuf)>,
}

fn closure_nodes(app: &ManagerRenderContext<'_>) -> ClosureGraph {
    // "Direct dependencies only" means the root of each library plus whatever
    // the root itself includes; anything reached through another file is a
    // transitive member and folds away.
    let direct_only = app.state.workbench.models_view.include_direct_only;
    let libraries = app.state.model_library_manager.libraries_sorted();
    let mut nodes = Vec::new();
    let mut matching = 0usize;
    for library in &libraries {
        for source in &library.source_closure {
            if direct_only && !is_direct_closure_member(library, &source.path) {
                continue;
            }
            matching += 1;
            if nodes.len() < GRAPH_NODE_LIMIT {
                nodes.push(ClosureNode {
                    path: source.path.clone(),
                    library: library.name.clone(),
                    digest: short_digest(&source.digest.to_string()),
                });
            }
        }
    }
    let drawn = nodes
        .iter()
        .map(|node| node.path.clone())
        .collect::<BTreeSet<_>>();
    let edges = libraries
        .iter()
        .flat_map(|library| &library.source_edges)
        .filter(|edge| drawn.contains(&edge.owner) && drawn.contains(&edge.target))
        .map(|edge| (edge.owner.clone(), edge.target.clone()))
        .collect();
    ClosureGraph {
        nodes,
        matching,
        edges,
    }
}

fn include_closure_graph(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    graph: &ClosureGraph,
    diagnostics: &ClosureFacts,
) {
    let direct_only = app.state.workbench.models_view.include_direct_only;
    detail_pane(
        ui,
        "RESOLVED CLOSURE",
        Some(if direct_only {
            "root plus direct dependencies"
        } else {
            "root plus authenticated dependencies"
        }),
        |ui| {
            let graph_height = (ui.available_height() * 0.42).clamp(150.0, 230.0);
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), graph_height),
                Sense::hover(),
            );
            let t = Tokens::get(ui.ctx());
            ui.painter().rect(
                rect,
                2.0,
                t.color.bg_inset,
                Stroke::new(1.0, t.color.border),
                egui::StrokeKind::Inside,
            );

            let sources = &graph.nodes;
            let matching = graph.matching;
            let hidden = matching.saturating_sub(GRAPH_NODE_LIMIT);
            if sources.is_empty() {
                ui.painter().text(
                    egui::pos2(rect.center().x, rect.center().y - 10.0),
                    egui::Align2::CENTER_CENTER,
                    "No retained include closure",
                    theme::sans(tokens::FS_1, FontWeight::SemiBold),
                    t.color.text_dim,
                );
                ui.painter().text(
                    egui::pos2(rect.center().x, rect.center().y + 14.0),
                    egui::Align2::CENTER_CENTER,
                    "Loaded definitions are built-in or have no authenticated source graph.",
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_faint,
                );
            } else {
                let node_width = ((rect.width() - 54.0) / 3.0).clamp(120.0, 210.0);
                let node_height = 38.0;
                let columns = 3usize;
                let row_count = sources.len().div_ceil(columns);
                let row_gap = if row_count > 1 {
                    ((rect.height() - 36.0 - node_height * row_count as f32)
                        / (row_count - 1) as f32)
                        .clamp(8.0, 24.0)
                } else {
                    0.0
                };
                let x_gap = ((rect.width() - node_width * columns as f32) / 4.0).max(8.0);
                let mut node_rects = BTreeMap::new();
                for (index, source) in sources.iter().enumerate() {
                    let column = index % columns;
                    let row = index / columns;
                    let x = rect.left() + x_gap + column as f32 * (node_width + x_gap);
                    let y = rect.top() + 18.0 + row as f32 * (node_height + row_gap);
                    let node = egui::Rect::from_min_size(
                        egui::pos2(x, y),
                        egui::vec2(node_width, node_height),
                    );
                    node_rects.insert(source.path.clone(), node);
                    let selected = app
                        .state
                        .workbench
                        .models_view
                        .include_selected_source
                        .as_deref()
                        == Some(source.path.to_string_lossy().as_ref());
                    ui.painter().rect(
                        node,
                        3.0,
                        if selected {
                            t.color.accent.linear_multiply(0.16)
                        } else {
                            t.color.bg_panel
                        },
                        Stroke::new(
                            if selected { 1.5 } else { 1.0 },
                            if selected {
                                t.color.accent
                            } else {
                                t.color.border
                            },
                        ),
                        egui::StrokeKind::Inside,
                    );
                    ui.painter().text(
                        egui::pos2(node.left() + 8.0, node.top() + 12.0),
                        egui::Align2::LEFT_CENTER,
                        elide(ui, &path_label(&source.path), node.width() - 16.0, true),
                        theme::mono(tokens::FS_0, FontWeight::SemiBold),
                        t.color.text,
                    );
                    ui.painter().text(
                        egui::pos2(node.left() + 8.0, node.bottom() - 10.0),
                        egui::Align2::LEFT_CENTER,
                        elide(
                            ui,
                            &format!("{} · {}", source.library, source.digest),
                            node.width() - 16.0,
                            false,
                        ),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_faint,
                    );
                    let response = ui.interact(
                        node,
                        ui.id()
                            .with(("models-include-node", source.path.as_os_str())),
                        Sense::click(),
                    );
                    let node_label = format!(
                        "{} · {} · {}",
                        path_label(&source.path),
                        source.library,
                        source.digest
                    );
                    response.widget_info(|| {
                        egui::WidgetInfo::selected(
                            egui::WidgetType::SelectableLabel,
                            ui.is_enabled(),
                            selected,
                            node_label.clone(),
                        )
                    });
                    crate::ui::theme::paint_focus_ring(ui, &response, node);
                    if response.clicked() {
                        app.state.workbench.models_view.include_selected_source =
                            Some(source.path.to_string_lossy().into_owned());
                    }
                }

                for (edge_owner, edge_target) in &graph.edges {
                    if let (Some(owner), Some(target)) =
                        (node_rects.get(edge_owner), node_rects.get(edge_target))
                    {
                        ui.painter().arrow(
                            owner.center_bottom(),
                            target.center_top() - owner.center_bottom(),
                            Stroke::new(1.0, t.color.text_faint),
                        );
                    }
                }
            }

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 14.0;
                ui.label(
                    RichText::new(format!("{} pinned", diagnostics.files))
                        .small()
                        .color(t.color.text_dim),
                );
                if hidden > 0 {
                    ui.label(
                        RichText::new(format!(
                            "showing {} of {matching} · {hidden} not drawn",
                            GRAPH_NODE_LIMIT
                        ))
                        .small()
                        .color(t.color.warn),
                    );
                }
                ui.label(
                    RichText::new(format!("{} dependency edges", diagnostics.edges))
                        .small()
                        .color(t.color.text_dim),
                );
                ui.label(
                    RichText::new(format!(
                        "{} unpinned · {} cyclic",
                        diagnostics.unpinned_roots, diagnostics.cyclic_nodes
                    ))
                    .small()
                    .color(if diagnostics.diagnostics() == 0 {
                        t.color.ok
                    } else {
                        t.color.err
                    }),
                );
            });
        },
    );
}

/// One name an instance could reference, and everything that defines it.
///
/// Readable outside the manager so the Simulation Studio's Models page can
/// state contested definitions without computing them a second time. Two
/// answers to "is this name contested" would eventually differ, and the one
/// on the page an operator was looking at would be the one they believed.
pub(in crate::workbench::surfaces) struct DefinitionRow {
    pub(in crate::workbench::surfaces) definition: String,
    pub(in crate::workbench::surfaces) scope: crate::state::model_library::ModelConsumerScope,
    pub(in crate::workbench::surfaces) providers: Vec<String>,
    pub(in crate::workbench::surfaces) provider_list: String,
    pub(in crate::workbench::surfaces) resolution: String,
    /// The library the manager's resolution record picked, where it picked one.
    ///
    /// Carried rather than parsed back out of [`Self::resolution`], which is
    /// display prose: a sentence that becomes a load-bearing format is a
    /// sentence nobody can rewrite.
    pub(in crate::workbench::surfaces) resolved_provider: Option<String>,
}

impl DefinitionRow {
    /// A contested name has no winner: the duplicate has to be removed or
    /// renamed before an instance can bind at all.
    ///
    /// Several providers is not that on its own. The manager records an
    /// explicit resolution for a duplicate it settled, and a name it settled
    /// binds — so counting every multi-provider name as contested marked rows
    /// the RESOLUTION column beside them was already calling resolved, and
    /// inflated this card's own count of what has to be repaired.
    pub(in crate::workbench::surfaces) fn contested(&self) -> bool {
        self.providers.len() > 1 && self.resolved_provider.is_none()
    }
}

/// Every definition name across the loaded libraries, with its providers.
///
/// Model names and subcircuit names share one namespace as far as an instance
/// reference is concerned, so both are here for "contested" to mean anything.
pub(in crate::workbench::surfaces) fn definition_index(state: &AppState) -> Vec<DefinitionRow> {
    use crate::state::model_library::ModelConsumerScope;
    let mut providers = BTreeMap::<(ModelConsumerScope, String), BTreeSet<String>>::new();
    for library in state.model_library_manager.libraries_sorted() {
        let active_sections = library.active_section_names();
        for model in library.models.values() {
            providers
                .entry((
                    ModelConsumerScope::PrimitiveModel,
                    model.name.to_ascii_lowercase(),
                ))
                .or_default()
                .insert(library.name.clone());
        }
        for subcircuit in library.subcircuits.values() {
            if subcircuit.section.as_deref().is_none_or(|section| {
                active_sections
                    .iter()
                    .any(|active| active.eq_ignore_ascii_case(section))
            }) {
                providers
                    .entry((
                        ModelConsumerScope::Subcircuit,
                        subcircuit.name.to_ascii_lowercase(),
                    ))
                    .or_default()
                    .insert(library.name.clone());
            }
        }
    }
    providers
        .into_iter()
        .map(|((scope, definition), candidates)| {
            let resolved_provider = state
                .model_library_manager
                .model_resolution_record(scope, &definition)
                .map(|record| record.provider_library.clone());
            let resolution = resolved_provider.as_ref().map_or_else(
                || {
                    if candidates.len() > 1 {
                        "contested · fails closed".to_owned()
                    } else {
                        "unique".to_owned()
                    }
                },
                |library| format!("resolved · {library}"),
            );
            DefinitionRow {
                definition,
                scope,
                provider_list: candidates.iter().cloned().collect::<Vec<_>>().join(", "),
                providers: candidates.into_iter().collect(),
                resolution,
                resolved_provider,
            }
        })
        .collect()
}

fn include_definition_table(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    definitions: &[DefinitionRow],
) {
    let query = app
        .state
        .workbench
        .models_view
        .include_definition_query
        .trim()
        .to_ascii_lowercase();
    let matching = definitions
        .iter()
        .filter(|row| {
            query.is_empty()
                || row.definition.contains(&query)
                || row
                    .providers
                    .iter()
                    .any(|provider| provider.to_ascii_lowercase().contains(&query))
        })
        .collect::<Vec<_>>();
    let contested_count = definitions.iter().filter(|row| row.contested()).count();
    let mut conflict = None;
    let mut create_subcircuit_symbol = None;
    card(ui, |ui| {
        card_title(
            ui,
            "DEFINITION RESOLUTION",
            Some(&format!(
                "{} unique names · {contested_count} contested",
                definitions.len()
            )),
        );
        table_header(
            ui,
            &[
                ("DEFINITION", 0.25),
                ("KIND", 0.13),
                ("PROVIDERS", 0.37),
                ("RESOLUTION", 0.25),
            ],
        );
        if matching.is_empty() {
            empty_state(
                ui,
                "No definitions match.",
                "The filter searches definition names and every retained provider.",
            );
            return;
        }
        ScrollArea::vertical()
            .id_salt("models-include-definitions")
            .max_height(ui.available_height().max(140.0))
            .show_rows(ui, ROW_H, matching.len(), |ui, range| {
                for row in &matching[range] {
                    // This column used to print the first provider under the
                    // heading "WINNING PROVIDER", which asserted a resolution
                    // policy that does not exist.
                    let response = selectable_data_row(
                        ui,
                        false,
                        &[
                            (&row.definition, 0.25, true),
                            (row.scope.label(), 0.13, false),
                            (&row.provider_list, 0.37, false),
                            (&row.resolution, 0.25, true),
                        ],
                    );
                    if response.clicked() {
                        if row.scope == ModelConsumerScope::Subcircuit
                            && let Ok(Some(provider)) = app
                                .state
                                .model_library_manager
                                .effective_definition_provider(row.scope, &row.definition)
                        {
                            create_subcircuit_symbol =
                                Some((provider.library, row.definition.clone()));
                        } else if row.contested() {
                            conflict =
                                Some((row.definition.clone(), row.scope, row.providers.clone()));
                        }
                    }
                }
            });
    });
    if let Some((definition, scope, providers)) = conflict {
        let selected_provider = app
            .state
            .model_library_manager
            .model_resolution_record(scope, &definition)
            .map(|record| record.provider_library.clone())
            .or_else(|| providers.first().cloned())
            .unwrap_or_default();
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::DefinitionConflict {
            definition,
            scope,
            providers,
            selected_provider,
            reason: String::new(),
        });
    }
    if let Some((library, subcircuit)) = create_subcircuit_symbol {
        app.queue_subcircuit_symbol(&library, &subcircuit);
    }
}

#[cfg(test)]
mod tests;
