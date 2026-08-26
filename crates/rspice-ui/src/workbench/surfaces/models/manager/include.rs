//! Specialist Models & PDKs page: includes.

use super::*;

pub(super) fn include_page(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    diagnostics: &ClosureFacts,
) {
    // One walk of the corpus, before the page bar, because the bar's own
    // subtitle states what it found. Everything below reads this vector; the
    // unresolved-instance card in particular answers its question by lookup
    // into it rather than by a second scan.
    let definitions = definition_index(app.state);
    let contested = definitions.iter().filter(|row| row.contested()).count();
    section_title(
        ui,
        "Model include graph",
        &format!(
            "{} files · {} edges · {} definitions · {contested} contested · {}",
            diagnostics.files,
            diagnostics.edges,
            definitions.len(),
            if diagnostics.cyclic_nodes == 0 {
                "acyclic".to_owned()
            } else {
                format!("{} cyclic", diagnostics.cyclic_nodes)
            },
        ),
        // Outermost-right first: the band lays its actions out right to left.
        |ui| {
            if Button::new("Export manifest").accent().show(ui).clicked() {
                export_include_manifest(app);
            }
            // It opens on the findings the last scan produced rather than
            // re-pinning outright: "resolve" used to mean "refresh and hope",
            // which accepted whatever the file had become without ever showing
            // the reader what that was.
            if Button::new("Resolve drift…")
                .enabled(!app.state.workbench.models_view.model_import_in_progress)
                .show(ui)
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
    let unbound = unbound_instances(app.state, &definitions);
    include_closure_graph(ui, app, &nodes, diagnostics);
    include_definition_table(ui, app, &definitions);
    // The rules beside the instances they govern: the table above answers
    // "which file wins for this name", and neither of these does — one states
    // what happens when nothing wins, the other names every instance that has
    // nothing.
    ui.columns(2, |columns| {
        resolution_rules_card(&mut columns[0]);
        instance_binding_card(&mut columns[1], app, &unbound);
    });
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
                        dependency_edge(ui, *owner, *target, Stroke::new(1.0, t.color.text_faint));
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

/// The arrowhead one dependency connector carries, in points.
const EDGE_HEAD: f32 = 6.0;

/// Draw one dependency edge between two drawn closure nodes.
///
/// `Painter::arrow` sizes its head as a fraction of the shaft, so an edge that
/// crossed the pane grew a head the size of the pane — two enormous strokes
/// fanning past the graph's own border, which is what this used to paint. The
/// head is a fixed mark here, and the shaft leaves whichever side of the owner
/// faces the target so a same-row edge no longer runs through the node between
/// them.
fn dependency_edge(ui: &Ui, owner: egui::Rect, target: egui::Rect, stroke: Stroke) {
    let (from, to) = if target.top() >= owner.bottom() - 1.0 {
        (owner.center_bottom(), target.center_top())
    } else if target.top() <= owner.top() - 1.0 {
        (owner.center_top(), target.center_bottom())
    } else if target.center().x >= owner.center().x {
        (owner.right_center(), target.left_center())
    } else {
        (owner.left_center(), target.right_center())
    };
    ui.painter().line_segment([from, to], stroke);
    let shaft = to - from;
    if shaft.length() <= EDGE_HEAD {
        return;
    }
    let direction = shaft.normalized();
    let normal = egui::vec2(-direction.y, direction.x);
    let base = to - direction * EDGE_HEAD;
    ui.painter().add(egui::epaint::Shape::convex_polygon(
        vec![
            to,
            base + normal * (EDGE_HEAD * 0.45),
            base - normal * (EDGE_HEAD * 0.45),
        ],
        stroke.color,
        Stroke::NONE,
    ));
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
    pub(in crate::workbench::surfaces) resolution: String,
    /// The library the manager's resolution record picked, where it picked one.
    ///
    /// Carried rather than parsed back out of [`Self::resolution`], which is
    /// display prose: a sentence that becomes a load-bearing format is a
    /// sentence nobody can rewrite.
    pub(in crate::workbench::surfaces) resolved_provider: Option<String>,
    /// The `.lib` sections the providers declare this name inside.
    ///
    /// A name declared outside any section has none, which is not the same as
    /// a name whose section is unknown: the parser records the section every
    /// definition came from, so an empty list here is positive evidence that
    /// the definition sits in the file's unsectioned body. Where two providers
    /// declare the same name in different sections both are carried, because
    /// which one executes is a corner decision made on another page.
    pub(in crate::workbench::surfaces) sections: Vec<String>,
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

    /// The provider the flat executable namespace will use, where one exists.
    ///
    /// A contested name has none — that is what contested means — and the cell
    /// that shows this says so rather than printing the first candidate. The
    /// column that used to head "WINNING PROVIDER" printed exactly that first
    /// candidate for every row, which asserted a resolution policy the product
    /// does not have.
    fn effective_provider(&self) -> Option<&str> {
        self.resolved_provider
            .as_deref()
            .or_else(|| match self.providers.as_slice() {
                [only] => Some(only.as_str()),
                _ => None,
            })
    }

    /// Every other authenticated provider of the same name.
    ///
    /// For a settled duplicate these are the losers the override record left
    /// standing — recorded, never dropped. For a contested name it is every
    /// provider, since none of them won.
    fn other_candidates(&self) -> String {
        let effective = self.effective_provider();
        let others = self
            .providers
            .iter()
            .filter(|provider| Some(provider.as_str()) != effective)
            .cloned()
            .collect::<Vec<_>>();
        if others.is_empty() {
            "—".to_owned()
        } else {
            others.join(", ")
        }
    }

    /// Where the providers declare it.
    fn section_label(&self) -> String {
        if self.sections.is_empty() {
            "—".to_owned()
        } else {
            self.sections.join(", ")
        }
    }
}

/// Every definition name across the loaded libraries, with its providers.
///
/// Model names and subcircuit names share one namespace as far as an instance
/// reference is concerned, so both are here for "contested" to mean anything.
pub(in crate::workbench::surfaces) fn definition_index(state: &AppState) -> Vec<DefinitionRow> {
    use crate::state::model_library::ModelConsumerScope;
    let mut providers = BTreeMap::<(ModelConsumerScope, String), BTreeSet<String>>::new();
    // The section a name is declared inside comes out of the same walk rather
    // than a second lookup per row: the parser already recorded it on every
    // model and subcircuit, and asking the catalog again per definition would
    // turn one pass over the corpus into one per name in it.
    let mut sections = BTreeMap::<(ModelConsumerScope, String), BTreeSet<String>>::new();
    for library in state.model_library_manager.libraries_sorted() {
        let active_sections = library.active_section_names();
        for model in library.models.values() {
            let key = (
                ModelConsumerScope::PrimitiveModel,
                model.name.to_ascii_lowercase(),
            );
            providers
                .entry(key.clone())
                .or_default()
                .insert(library.name.clone());
            if let Some(section) = model.section.as_deref() {
                sections.entry(key).or_default().insert(section.to_owned());
            }
        }
        for subcircuit in library.subcircuits.values() {
            if subcircuit.section.as_deref().is_none_or(|section| {
                active_sections
                    .iter()
                    .any(|active| active.eq_ignore_ascii_case(section))
            }) {
                let key = (
                    ModelConsumerScope::Subcircuit,
                    subcircuit.name.to_ascii_lowercase(),
                );
                providers
                    .entry(key.clone())
                    .or_default()
                    .insert(library.name.clone());
                if let Some(section) = subcircuit.section.as_deref() {
                    sections.entry(key).or_default().insert(section.to_owned());
                }
            }
        }
    }
    providers
        .into_iter()
        .map(|(key, candidates)| {
            let (scope, definition) = key.clone();
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
                // Every provider, in order. The joined display spelling this
                // used to carry beside it went unread the moment the table
                // split the effective provider from the candidates that lost.
                providers: candidates.into_iter().collect(),
                resolution,
                resolved_provider,
                sections: sections
                    .remove(&key)
                    .map(|sections| sections.into_iter().collect())
                    .unwrap_or_default(),
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
                ("DEFINITION", 0.22),
                ("KIND", 0.11),
                ("PROVIDER", 0.20),
                ("SECTION", 0.11),
                ("OTHER CANDIDATES", 0.20),
                ("RESOLUTION", 0.16),
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
                    // The provider column carries the one that *executes*, and
                    // an em dash where nothing does. It used to print the first
                    // provider under the heading "WINNING PROVIDER", which
                    // asserted a resolution policy that does not exist.
                    let provider = row.effective_provider().unwrap_or("—");
                    let section = row.section_label();
                    let others = row.other_candidates();
                    let response = selectable_data_row(
                        ui,
                        false,
                        &[
                            (&row.definition, 0.22, true),
                            (row.scope.label(), 0.11, false),
                            (provider, 0.20, false),
                            (&section, 0.11, true),
                            (&others, 0.20, false),
                            (&row.resolution, 0.16, true),
                        ],
                    );
                    let announcement = format!(
                        "{} · {} · provider {provider} · section {section} · other candidates \
                         {others} · {}",
                        row.definition,
                        row.scope.label(),
                        row.resolution,
                    );
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        node.set_label(announcement.clone());
                    });
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

// ---------------------------------------------------------------------------
// What happens when a name does not resolve
// ---------------------------------------------------------------------------

/// One shipped resolution policy, its settled value, and who enforces it.
///
/// Every row here was read out of the code that refuses, and the owner column
/// names *that* code rather than the page a reader would like to edit it on.
/// Three of these are the engine's; none of them is a preference. The mockup
/// attributed four to the run plan and offered an "Edit in run plan…" control
/// beside them — there is nothing there to edit, so the control is not offered
/// and the owners say where the refusal actually comes from.
const RESOLUTION_RULES: [(&str, &str, &str); 5] = [
    (
        "Unresolved model name",
        "refused at the model-bindings stage",
        "run preparation",
    ),
    (
        "Duplicate definition",
        "an explicit record, or refused",
        "project catalog",
    ),
    (
        "Missing named section",
        "refused · no fallback body",
        "engine parser",
    ),
    (
        "Include cycle",
        "refused · the cycle is named",
        "engine parser",
    ),
    (
        "Host model search path",
        "none · the sealed closure only",
        "engine parser",
    ),
];

/// The rules in force, stated where the contested names are.
///
/// The mockup put a MODEL SEARCH PATH table under this card, listing the
/// directories a name would be looked for in. RSpice has no such list and the
/// absence is the design: `IncludeProcessor` resolves every dependency through
/// the sealed bundle and refuses anything not in it, so a search path table
/// here could only ever be a table of one row that is not a path. The last
/// rule states the absence instead.
fn resolution_rules_card(ui: &mut Ui) {
    card(ui, |ui| {
        card_title(ui, "RESOLUTION RULES IN FORCE", Some("fail closed"));
        table_header(
            ui,
            &[("RULE", 0.30), ("RESOLVED VALUE", 0.44), ("OWNER", 0.26)],
        );
        for (rule, value, owner) in RESOLUTION_RULES {
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_H), Sense::hover());
            paint_columns(
                ui,
                rect,
                &[
                    (rule, 0.30, false),
                    (value, 0.44, true),
                    (owner, 0.26, false),
                ],
            );
            // Painter text publishes nothing, and a rule a reader cannot hear
            // is a rule they will find out about from a refusal instead.
            let announcement = format!("{rule}: {value}. Enforced by {owner}.");
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Label);
                node.set_label(announcement.clone());
            });
        }
        ui.label(
            RichText::new(
                "None of these is a preference. A name two providers declare is recorded as \
                 contested and refuses to bind until an override record picks one; the loser is \
                 kept as a candidate rather than dropped.",
            )
            .small()
            .color(Tokens::get(ui.ctx()).color.text_faint),
        );
    });
}

/// One placed instance whose model name nothing in the closure defines.
struct UnboundInstance {
    component_id: u64,
    instance: String,
    sheet: String,
    reference: String,
}

/// Rows the callout lists before it reports the remainder.
const UNBOUND_ROWS: usize = 4;

/// Instances the closure cannot answer for.
///
/// Derived from the definition index this page has already built — so no second
/// walk of the corpus — plus one pass over the placed components of the active
/// sheet. [`ConsumerIndex`][index] next door answers a richer question (which
/// provider, and whether the instance's declared library agrees with it) and
/// pays `definition_providers` — a full walk of every library — once per
/// instance to do it. This page asks only the include graph's own question:
/// does *anything* in the closure define this name. The index it is already
/// holding answers that by lookup.
///
/// [index]: super::bindings::ConsumerIndex
fn unbound_instances(state: &AppState, definitions: &[DefinitionRow]) -> Vec<UnboundInstance> {
    let Some(schematic) = state.workspace.active_schematic() else {
        return Vec::new();
    };
    let defined = definitions
        .iter()
        .map(|row| row.definition.as_str())
        .collect::<BTreeSet<_>>();
    let sheet = state.workspace.active_view.display_path();
    let mut unbound = schematic
        .components
        .iter()
        .filter_map(|component| {
            let reference = explicit_component_model(component)?;
            if defined.contains(reference.to_ascii_lowercase().as_str()) {
                return None;
            }
            Some(UnboundInstance {
                component_id: component.id,
                instance: component.name.clone(),
                sheet: sheet.clone(),
                reference,
            })
        })
        .collect::<Vec<_>>();
    unbound.sort_by(|left, right| {
        left.instance
            .cmp(&right.instance)
            .then_with(|| left.reference.cmp(&right.reference))
    });
    unbound
}

/// Every instance the closure does not answer for, and the two ways out.
///
/// Exception-only: a design where every reference resolves says so in one line
/// rather than rendering an empty table. The routes are deliberately not a
/// second binding control — the catalog page owns binding, and two authors for
/// one repair is how two of them end up disagreeing — so "Bind a model…" takes
/// the reader there with the instance already selected.
fn instance_binding_card(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    unbound: &[UnboundInstance],
) {
    let t = Tokens::get(ui.ctx());
    let mut bind = None;
    let mut locate = None;
    card(ui, |ui| {
        card_title(
            ui,
            "INSTANCE BINDINGS",
            Some(&if unbound.is_empty() {
                "every reference resolves".to_owned()
            } else {
                format!(
                    "{} unresolved instance{}",
                    unbound.len(),
                    if unbound.len() == 1 { "" } else { "s" }
                )
            }),
        );
        if unbound.is_empty() {
            ui.label(
                RichText::new(
                    "Every model an instance on this sheet names is declared by a file in the \
                     authenticated closure above.",
                )
                .small()
                .color(t.color.ok),
            );
            return;
        }
        for instance in unbound.iter().take(UNBOUND_ROWS) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.label(RichText::new("unresolved").small().color(t.color.err));
                ui.label(
                    RichText::new(&instance.instance)
                        .monospace()
                        .small()
                        .color(t.color.text),
                );
                ui.label(
                    RichText::new(&instance.sheet)
                        .small()
                        .color(t.color.text_faint),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(&instance.reference)
                            .monospace()
                            .small()
                            .color(t.color.warn),
                    );
                });
            });
            ui.label(
                RichText::new(format!(
                    "No file in the closure declares '{}'. Netlisting refuses the design until \
                     the name resolves or the instance is removed; there is no host search path \
                     to fall back to.",
                    instance.reference
                ))
                .small()
                .color(t.color.text_dim),
            );
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                if Button::new("Bind a model…")
                    .show(ui)
                    .on_hover_text(
                        "Selects this instance and opens the catalog, where a model is bound to it.",
                    )
                    .clicked()
                {
                    bind = Some(instance.component_id);
                }
                if Button::new("Show instance")
                    .show(ui)
                    .on_hover_text("Selects this instance and opens the schematic it sits on.")
                    .clicked()
                {
                    locate = Some(instance.component_id);
                }
            });
        }
        if unbound.len() > UNBOUND_ROWS {
            ui.label(
                RichText::new(format!(
                    "…and {} more, all counted above. The catalog page lists every one.",
                    unbound.len() - UNBOUND_ROWS
                ))
                .small()
                .color(t.color.text_faint),
            );
        }
    });
    if let Some(component_id) = bind {
        app.state
            .schematic
            .selection
            .select_only_component(component_id);
        app.queue_command(Command::ModelsPage(ModelsPage::Models));
    }
    if let Some(component_id) = locate {
        app.state
            .schematic
            .selection
            .select_only_component(component_id);
        navigate_specialist(app, crate::workbench::SurfaceId::Design);
    }
}

#[cfg(test)]
mod tests;
