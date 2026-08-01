//! The project dependency ledger.
//!
//! Rows are derived from the live design-library catalog, model-source
//! authority, and the exact technology binding accepted by the project. This
//! module does not maintain a second lock model: export serializes the same
//! derived view shown on screen.

use serde::Serialize;

use super::*;
use crate::state::model_library::ModelSourceAuthority;
use crate::workbench::app_state::AppState;

const DEPENDENCY_SPLIT_BREAKPOINT: f32 = 640.0;
const DEPENDENCY_LEFT_SHARE: f32 = 0.40;

#[derive(Debug, Clone, Serialize)]
struct DependencyManifest {
    schema_version: u16,
    project_id: String,
    project_revision: u64,
    technology: Option<TechnologyManifest>,
    resources: Vec<DependencyRow>,
}

#[derive(Debug, Clone, Serialize)]
struct TechnologyManifest {
    label: String,
    model_library: String,
    root_source: String,
    source_count: usize,
    dependency_edge_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DependencyRow {
    identity: String,
    resource_type: String,
    authority: String,
    version: String,
    source: String,
    digest: String,
    status: String,
}

pub(super) fn dependencies(ui: &mut Ui, app: &mut RSpiceApp) {
    dependency_context_strip(ui, &mut app.state);
    let width = visible_workspace_width(ui);
    if width >= DEPENDENCY_SPLIT_BREAKPOINT {
        let left_width = (width * DEPENDENCY_LEFT_SHARE).floor().max(240.0);
        let right_width = (width - left_width - 1.0).max(320.0);
        let shown = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui_with_layout(vec2(left_width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.set_width(left_width);
                technology_binding_panel(ui, app);
            });
            ui.allocate_ui_with_layout(
                vec2(right_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_width(right_width);
                    dependency_inventory(ui, &mut app.state, right_width);
                },
            );
        });
        ui.painter().vline(
            shown.response.rect.left() + left_width + 0.5,
            shown.response.rect.y_range(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border_strong),
        );
    } else {
        technology_binding_panel(ui, app);
        dependency_inventory(ui, &mut app.state, width);
    }
}

fn dependency_context_strip(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let rows = dependency_rows(state);
    let filter = state
        .workbench
        .project_dependency_filter
        .trim()
        .to_ascii_lowercase();
    let visible = rows.iter().filter(|row| row_matches(row, &filter)).count();
    let accepted = rows
        .iter()
        .filter(|row| matches!(row.status.as_str(), "accepted" | "available" | "embedded"))
        .count();
    let review = rows.len().saturating_sub(accepted);
    let shown = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let search_row = egui::Frame::new()
                .inner_margin(Margin::symmetric(10, 4))
                .show(ui, |ui| {
                    let width = visible_workspace_width(ui);
                    let search = ui.add_sized(
                        [width, 28.0],
                        egui::TextEdit::singleline(&mut state.workbench.project_dependency_filter)
                            .hint_text("Dependency, type, source, version\u{2026}"),
                    );
                    ui.ctx().accesskit_node_builder(search.id, |node| {
                        node.set_label("Filter project dependencies");
                    });
                });
            ui.painter().hline(
                search_row.response.rect.x_range(),
                search_row.response.rect.bottom(),
                Stroke::new(1.0, t.color.border),
            );
            egui::Frame::new()
                .fill(t.color.bg_panel)
                .inner_margin(Margin::symmetric(10, 3))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        context_fact(
                            ui,
                            "Closure /",
                            &format!("{accepted} resolved \u{00b7} {review} review"),
                            if review == 0 {
                                t.color.ok
                            } else {
                                t.color.warn
                            },
                        );
                        ui.separator();
                        context_fact(
                            ui,
                            "Manifest",
                            &format!("project \u{00b7} {visible} shown"),
                            t.color.text,
                        );
                    });
                });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn context_fact(ui: &mut Ui, label: &str, value: &str, color: Color32) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
    ui.label(
        egui::RichText::new(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn technology_binding_panel(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let binding = app.state.workspace.project.technology_binding().cloned();
    let contract = app.state.validate_project_technology_contract();
    let contract_ok = binding.is_some() && contract.is_ok();
    workspace_table_panel_header(
        ui,
        "Technology & simulator binding",
        if contract_ok { "PINNED" } else { "REVIEW" },
        if contract_ok {
            t.color.ok
        } else {
            t.color.warn
        },
    );
    let (label, library, source, sources, edges) = binding.as_ref().map_or_else(
        || {
            (
                "Not attached".to_owned(),
                "No executable model set".to_owned(),
                "No authenticated root source".to_owned(),
                0,
                0,
            )
        },
        |binding| {
            (
                binding.display_label(),
                binding.model_library().to_owned(),
                binding.root_source().display().to_string(),
                binding.source_closure().len(),
                binding.source_edges().len(),
            )
        },
    );
    property_row(ui, "Technology", &label);
    property_row(ui, "Model library", &library);
    property_row(ui, "Root source", &source);
    property_row(
        ui,
        "Source closure",
        &format!("{sources} files \u{00b7} {edges} edges"),
    );
    if let Some(pin) = binding
        .as_ref()
        .and_then(crate::state::ProjectTechnologyBinding::signed_package)
    {
        property_row(
            ui,
            "Signed package",
            &format!(
                "{} \u{00b7} {} \u{00b7} {} nm \u{00b7} {}",
                pin.package_id(),
                pin.revision(),
                pin.process_node_nm(),
                pin.stack_name()
            ),
        );
        property_row(
            ui,
            "Publisher authority",
            &format!(
                "{} \u{00b7} key {}",
                pin.publisher_id(),
                pin.signing_key_id()
            ),
        );
        property_row(
            ui,
            "Package digests",
            &format!(
                "manifest {} \u{00b7} archive {}",
                short_identity(&pin.manifest_digest().to_string()),
                short_identity(&pin.archive_digest().to_string())
            ),
        );
    } else {
        property_row(ui, "Signed package", "Not attached");
    }
    property_row(
        ui,
        "Model catalog",
        &format!(
            "{} libraries \u{00b7} {} models",
            app.state.model_library_manager.library_count(),
            app.state.model_library_manager.total_model_count()
        ),
    );
    property_row(
        ui,
        "Execution",
        crate::state::ExecutionTarget::current().label(),
    );
    let contract_message = contract.err().unwrap_or_else(|| {
        "Model sources, signed package bytes, publisher signature, and runtime compatibility validate exactly."
            .to_owned()
    });
    egui::Frame::new()
        .fill(if contract_ok {
            theme::mix(t.color.bg_panel, t.color.ok, 0.06)
        } else {
            theme::mix(t.color.bg_panel, t.color.warn, 0.08)
        })
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(&contract_message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if Button::new(TECHNOLOGY_SURFACE_ACTION)
                    .accent()
                    .show(ui)
                    .clicked()
                {
                    open_technology_attachment_dialog(app);
                }
                if Button::new("Model paths\u{2026}").show(ui).clicked() {
                    Command::PdkSettings.execute(app);
                }
                if Button::new("Export manifest\u{2026}").show(ui).clicked() {
                    export_dependency_manifest(ui.ctx(), &mut app.state);
                }
            });
        });
}

fn dependency_inventory(ui: &mut Ui, state: &mut AppState, pane_width: f32) {
    let t = Tokens::get(ui.ctx());
    let rows = dependency_rows(state);
    let filter = state
        .workbench
        .project_dependency_filter
        .trim()
        .to_ascii_lowercase();
    let visible_count = rows.iter().filter(|row| row_matches(row, &filter)).count();
    let meta = format!("{visible_count} SHOWN");
    workspace_table_panel_header(ui, "Resolved dependency ledger", &meta, t.color.ok);

    // `ScrollArea` may report the width of its unconstrained child rather
    // than the visible split pane. Bind the ledger to the actual clip width
    // so the status column cannot render underneath the Inspector.
    let available = pane_width.min(visible_workspace_width(ui)).max(1.0);
    ScrollArea::horizontal()
        .id_salt("workbench.project.dependencies.inventory")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            // The reference ledger fits all contract fields in its allocated
            // pane. Elision preserves that overview at compact desktop
            // widths; horizontal scrolling remains available only when a
            // genuinely narrow touch layout cannot honor the columns.
            let width = available;
            ui.set_min_width(width);
            let compact = width < 480.0;
            if compact {
                workspace_table_row(
                    ui,
                    width,
                    ["DEPENDENCY", "TYPE", "VERSION", "SOURCE", "STATUS"],
                    [0.25, 0.15, 0.13, 0.27, 0.20],
                    true,
                    &[],
                    &[],
                );
            } else {
                workspace_table_row(
                    ui,
                    width,
                    [
                        "DEPENDENCY",
                        "TYPE",
                        "VERSION",
                        "SOURCE",
                        "DIGEST",
                        "STATUS",
                    ],
                    [0.19, 0.13, 0.10, 0.22, 0.17, 0.19],
                    true,
                    &[],
                    &[],
                );
            }
            if rows.is_empty() {
                workspace_empty_table_row(
                    ui,
                    width,
                    "No design or executable model dependencies are currently registered.",
                );
                return;
            }
            let mut any_visible = false;
            for row in rows.iter().filter(|row| row_matches(row, &filter)) {
                any_visible = true;
                let status_color = match row.status.as_str() {
                    "accepted" | "available" | "embedded" => t.color.ok,
                    "catalog only" => t.color.text_dim,
                    _ => t.color.warn,
                };
                let (response, _) = if compact {
                    workspace_table_row(
                        ui,
                        width,
                        [
                            row.identity.as_str(),
                            row.resource_type.as_str(),
                            row.version.as_str(),
                            row.source.as_str(),
                            row.status.as_str(),
                        ],
                        [0.25, 0.15, 0.13, 0.27, 0.20],
                        false,
                        &[],
                        &[(4, status_color)],
                    )
                } else {
                    workspace_table_row(
                        ui,
                        width,
                        [
                            row.identity.as_str(),
                            row.resource_type.as_str(),
                            row.version.as_str(),
                            row.source.as_str(),
                            row.digest.as_str(),
                            row.status.as_str(),
                        ],
                        [0.19, 0.13, 0.10, 0.22, 0.17, 0.19],
                        false,
                        &[4],
                        &[(5, status_color)],
                    )
                };
                if state.workbench.project_dependency_selection.as_deref()
                    == Some(row.identity.as_str())
                {
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            response.rect.left_top(),
                            vec2(2.0, response.rect.height()),
                        ),
                        0.0,
                        t.color.accent,
                    );
                }
                let response = response.on_hover_text(format!(
                    "{}\nAuthority: {}\nSource: {}",
                    row.identity, row.authority, row.source
                ));
                if response.clicked() {
                    state.workbench.project_dependency_selection = Some(row.identity.clone());
                }
            }
            if !any_visible {
                workspace_empty_table_row(ui, width, "No dependency matches the current filter.");
            }
        });
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if Button::new("Dependency audit\u{2026}").show(ui).clicked() {
                    state.ui.toasts.success(
                        ui.ctx(),
                        "Dependency audit",
                        format!("{visible_count} visible resources projected from live authority."),
                    );
                }
                if Button::new("Export manifest\u{2026}").show(ui).clicked() {
                    export_dependency_manifest(ui.ctx(), state);
                }
            });
        });
}

fn dependency_rows(state: &AppState) -> Vec<DependencyRow> {
    let mut rows = Vec::new();
    for library in state.library_manager.libraries_sorted() {
        rows.push(DependencyRow {
            identity: library.name.clone(),
            resource_type: "Design library".to_owned(),
            authority: if library.read_only {
                "Read only".to_owned()
            } else {
                "Project writable".to_owned()
            },
            version: nonempty_or_dash(&library.technology),
            source: library.path.as_ref().map_or_else(
                || "Embedded project data".to_owned(),
                |path| display_path(path),
            ),
            digest: "—".to_owned(),
            status: if library.path.is_some() {
                "available".to_owned()
            } else {
                "embedded".to_owned()
            },
        });
    }
    for library in state.model_library_manager.libraries_sorted() {
        let authority = match library.source_authority {
            ModelSourceAuthority::BuiltIn => "Built-in catalog",
            ModelSourceAuthority::External => "External source",
            ModelSourceAuthority::RetainedImport { .. } => "Retained imported source",
            ModelSourceAuthority::ProjectOwned { .. } => "Project owned",
        };
        let status = if !library.source_authority.has_execution_source() {
            "catalog only"
        } else if library.source_closure.is_empty() {
            "source not pinned"
        } else if library.source_contents.len() != library.source_closure.len() {
            "retained bytes incomplete"
        } else {
            "accepted"
        };
        let digest = library
            .source_closure
            .iter()
            .find(|pin| library.root_path.as_ref() == Some(&pin.path))
            .or_else(|| library.source_closure.first())
            .map_or_else(
                || "—".to_owned(),
                |pin| short_identity(&pin.digest.to_string()),
            );
        rows.push(DependencyRow {
            identity: library.name.clone(),
            resource_type: "Model library".to_owned(),
            authority: authority.to_owned(),
            version: nonempty_or_dash(&library.version),
            source: library.root_path.as_ref().map_or_else(
                || "No executable source".to_owned(),
                |path| display_path(path),
            ),
            digest,
            status: status.to_owned(),
        });
    }
    rows.sort_by(|left, right| {
        left.resource_type
            .cmp(&right.resource_type)
            .then_with(|| left.identity.cmp(&right.identity))
    });
    rows
}

fn row_matches(row: &DependencyRow, filter: &str) -> bool {
    let filter = filter.to_ascii_lowercase();
    filter.is_empty()
        || [
            &row.identity,
            &row.resource_type,
            &row.authority,
            &row.version,
            &row.source,
            &row.digest,
            &row.status,
        ]
        .iter()
        .any(|value| value.to_ascii_lowercase().contains(&filter))
}

fn nonempty_or_dash(value: &str) -> String {
    if value.trim().is_empty() {
        "—".to_owned()
    } else {
        value.to_owned()
    }
}

fn display_path(path: &std::path::Path) -> String {
    path.display().to_string()
}

fn dependency_manifest(state: &AppState) -> DependencyManifest {
    let technology =
        state
            .workspace
            .project
            .technology_binding()
            .map(|binding| TechnologyManifest {
                label: binding.display_label(),
                model_library: binding.model_library().to_owned(),
                root_source: binding.root_source().display().to_string(),
                source_count: binding.source_closure().len(),
                dependency_edge_count: binding.source_edges().len(),
            });
    DependencyManifest {
        schema_version: 1,
        project_id: state.workspace.project.id().to_string(),
        project_revision: state.workspace.project.revision().get(),
        technology,
        resources: dependency_rows(state),
    }
}

fn export_dependency_manifest(ctx: &Context, state: &mut AppState) {
    let contents = match serde_json::to_vec_pretty(&dependency_manifest(state)) {
        Ok(contents) => contents,
        Err(error) => {
            state.ui.toasts.error_with_title(
                ctx,
                "Manifest export failed",
                format!("Dependency manifest serialization failed: {error}"),
            );
            return;
        }
    };
    let filename = format!(
        "{}-dependencies.json",
        state
            .workspace
            .project
            .display_name()
            .chars()
            .map(|character| if character.is_alphanumeric() {
                character
            } else {
                '-'
            })
            .collect::<String>()
            .trim_matches('-')
    );

    match crate::workbench::workflows::export_workflow::publish_project_manifest(
        &filename, &contents,
    ) {
        Ok(Some(receipt)) => state.ui.toasts.success(ctx, "Manifest exported", receipt),
        Ok(None) => {}
        Err(error) => state
            .ui
            .toasts
            .error_with_title(ctx, "Manifest export failed", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependency_filter_matches_every_visible_field() {
        let row = DependencyRow {
            identity: "models_tt".to_owned(),
            resource_type: "Model library".to_owned(),
            authority: "Project owned".to_owned(),
            version: "2.1".to_owned(),
            source: "/models/tt.lib".to_owned(),
            digest: "12ab".to_owned(),
            status: "accepted".to_owned(),
        };
        assert!(row_matches(&row, "project"));
        assert!(row_matches(&row, "tt.lib"));
        assert!(row_matches(&row, "12AB"));
        assert!(!row_matches(&row, "missing"));
    }
}
