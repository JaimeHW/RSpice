//! The three-corpus model browser.
//!
//! A loaded project model, an installed distribution pack, and an indexed
//! definition are not interchangeable. This surface keeps those identities
//! separate while letting an engineer move from discovery to an executable
//! project source through one transactional workflow.

use super::*;

use rspice_core::library::{CatalogDefinitionPreview, SpiceLibraryIndex, SpicePack};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use crate::state::model_library::{
    ModelLevel, ModelLibraryManager, ModelSourceAuthority, ModelType, PackCatalogPage,
};
use crate::state::{Component, ComponentType, SchematicState};
use crate::workbench::state::{
    ModelCatalogScope, ModelLibraryFacet, ModelPackFacet, ModelProjectFacet,
};

const CATALOG_PAGE_LIMIT: usize = 200;
const CATALOG_WIDE_BREAKPOINT: f32 = 900.0;
const CATALOG_TOOLBAR_BREAKPOINT: f32 = 760.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ProjectCatalogStatus {
    Qualified,
    Ready,
    Review,
    Preview,
    Blocked,
}

impl ProjectCatalogStatus {
    const fn label(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Ready => "ready",
            Self::Review => "review",
            Self::Preview => "preview",
            Self::Blocked => "blocked",
        }
    }

    fn color(self, tokens: &Tokens) -> Color32 {
        match self {
            Self::Qualified | Self::Ready => tokens.color.ok,
            Self::Review | Self::Preview => tokens.color.warn,
            Self::Blocked => tokens.color.err,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum ProjectCatalogDefinition {
    Model(DeviceModel),
    Subcircuit(ModelSubcircuitInterface),
}

impl ProjectCatalogDefinition {
    pub(super) fn name(&self) -> &str {
        match self {
            Self::Model(model) => &model.name,
            Self::Subcircuit(subcircuit) => &subcircuit.name,
        }
    }

    const fn kind_token(&self) -> &'static str {
        match self {
            Self::Model(_) => "model",
            Self::Subcircuit(_) => "subcircuit",
        }
    }

    fn family_label(&self) -> String {
        match self {
            Self::Model(model) => format!(
                "{} · {}",
                model.model_type.display_name(),
                model.level.display_name()
            ),
            Self::Subcircuit(subcircuit) => {
                format!("Subcircuit · {} terminal(s)", subcircuit.ports.len())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct ProjectCatalogRecord {
    key: String,
    library: String,
    pub(super) definition: ProjectCatalogDefinition,
    usage: Vec<String>,
    pub(super) qualification: Option<QualificationModelSummary>,
    pub(super) status: ProjectCatalogStatus,
    pinned: bool,
}

#[derive(Debug, Clone)]
struct CachedPartQuery {
    signature: String,
    cancel: Arc<AtomicBool>,
    outcome: Arc<Mutex<Option<Result<PackCatalogPage, String>>>>,
}

#[derive(Debug, Clone)]
struct CachedDeviceCounts {
    root: String,
    outcome: Arc<Mutex<Option<Result<BTreeMap<String, usize>, String>>>>,
}

#[derive(Debug, Clone)]
enum CatalogTaskState<T> {
    Pending,
    Ready(Result<T, String>),
}

#[derive(Debug, Clone)]
struct CachedPartPreview {
    key: String,
    result: Result<CatalogDefinitionPreview, String>,
}

pub(super) fn models_catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    catalog_scope_bar(ui, app);
    ui.add_space(5.0);

    match app.state.workbench.model_catalog_scope {
        ModelCatalogScope::Project => project_catalog(ui, app),
        ModelCatalogScope::Packs => pack_catalog(ui, app),
        ModelCatalogScope::Library => part_catalog(ui, app),
    }

    render_pack_detach_confirmation(ui.ctx(), app);
}

fn catalog_scope_bar(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let project_count = app.state.model_library_manager.total_definition_count();
    let (pack_count, part_count) = app
        .state
        .model_library_manager
        .spice_packs()
        .map_or((0, 0), |index| (index.packs().len(), index.part_count()));
    let counts = [project_count, pack_count, part_count];
    let current = app.state.workbench.model_catalog_scope;

    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(7, 5))
        .show(ui, |ui| {
            ScrollArea::horizontal()
                .id_salt("models.catalog.scopes")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 5.0;
                        for (scope, count) in
                            ModelCatalogScope::ALL.into_iter().zip(counts.into_iter())
                        {
                            let label = format!("{}  {}", scope.label(), grouped_count(count));
                            let response = ui.add(
                                egui::Button::new(
                                    egui::RichText::new(label)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Medium)),
                                )
                                .selected(scope == current)
                                .min_size(egui::vec2(0.0, t.metrics.ctl_h)),
                            );
                            response.widget_info(|| {
                                egui::WidgetInfo::selected(
                                    egui::WidgetType::Button,
                                    ui.is_enabled(),
                                    scope == current,
                                    scope.label(),
                                )
                            });
                            if response.clicked() {
                                app.state.workbench.model_catalog_scope = scope;
                            }
                        }
                    });
                });
        });
}

fn project_catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    let records = project_catalog_records(app);
    let counts = ModelProjectFacet::ALL.map(|facet| {
        records
            .iter()
            .filter(|record| project_facet_matches(facet, record))
            .count()
    });
    project_toolbar(ui, app, counts);
    ui.add_space(5.0);

    let query = app
        .state
        .model_library_manager
        .filter_text
        .trim()
        .to_ascii_lowercase();
    let facet = app.state.workbench.model_project_facet;
    let visible = records
        .iter()
        .filter(|record| project_facet_matches(facet, record))
        .filter(|record| project_record_matches_query(record, &query))
        .cloned()
        .collect::<Vec<_>>();
    let selected = selected_project_record(app, &records);
    let selected_key = selected.as_ref().map(|record| record.key.as_str());
    let t = Tokens::get(ui.ctx());
    let rows = visible
        .iter()
        .map(|record| {
            let vectors = record.qualification.as_ref().map_or_else(
                || "interface".to_owned(),
                |qualification| {
                    if qualification.vectors == 0 {
                        "none".to_owned()
                    } else {
                        format!(
                            "{} / {} pass",
                            qualification.passing_vectors, qualification.vectors
                        )
                    }
                },
            );
            let usage = if record.usage.is_empty() {
                String::new()
            } else if record.usage.len() <= 3 {
                record.usage.join(" · ")
            } else {
                format!(
                    "{} · +{}",
                    record.usage[..2].join(" · "),
                    record.usage.len() - 2
                )
            };
            DataRow {
                key: record.key.clone(),
                selected: selected_key == Some(record.key.as_str()),
                cells: vec![
                    DataCell::mono(if record.pinned {
                        format!("◆ {}", record.definition.name())
                    } else {
                        record.definition.name().to_owned()
                    }),
                    DataCell::plain(record.definition.family_label()),
                    DataCell::mono(project_record_source(app, record)),
                    DataCell::mono(usage),
                    DataCell::mono(vectors),
                    DataCell::mono_colored(record.status.label(), record.status.color(&t)),
                ],
            }
        })
        .collect::<Vec<_>>();

    let columns = [
        ("Definition", 0.18),
        ("Family", 0.18),
        ("Source", 0.20),
        ("Used by", 0.18),
        ("Evidence", 0.14),
        ("Status", 0.12),
    ];
    let event = catalog_browser(
        ui,
        "models.catalog.project",
        &columns,
        &rows,
        if records.is_empty() {
            "No executable model sources are loaded in this project."
        } else {
            "No project models or subcircuits match the active facet and filter."
        },
        |ui| project_definition_detail(ui, app, selected.as_ref()),
    );
    if let Some(event) = event {
        if let Some(record) = records.iter().find(|record| record.key == event.key) {
            app.state
                .model_library_manager
                .select_library(&record.library);
            app.state.workbench.selected_model = match &record.definition {
                ProjectCatalogDefinition::Model(model) => Some(model.name.clone()),
                ProjectCatalogDefinition::Subcircuit(_) => None,
            };
            app.state.workbench.model_catalog_selected_project = Some(record.key.clone());
        }
    }
}

fn project_toolbar(ui: &mut Ui, app: &mut RSpiceApp, counts: [usize; 6]) {
    let width = ui.available_width();
    let current = app.state.workbench.model_project_facet;
    let mut requested = None;
    if width < CATALOG_TOOLBAR_BREAKPOINT {
        facet_strip(
            ui,
            "models.catalog.project.facets",
            ModelProjectFacet::ALL
                .into_iter()
                .zip(counts)
                .map(|(facet, count)| (facet.label(), count, facet == current, facet)),
            |facet| requested = Some(facet),
        );
        ui.add_space(4.0);
        catalog_search(
            ui,
            &mut app.state.model_library_manager.filter_text,
            "Filter project models, parameters, sources, or instances…",
        );
    } else {
        ui.horizontal(|ui| {
            ui.set_width(width);
            ui.allocate_ui_with_layout(
                egui::vec2((width - 300.0).max(1.0), ui.available_height()),
                Layout::left_to_right(Align::Center),
                |ui| {
                    facet_strip(
                        ui,
                        "models.catalog.project.facets",
                        ModelProjectFacet::ALL
                            .into_iter()
                            .zip(counts)
                            .map(|(facet, count)| (facet.label(), count, facet == current, facet)),
                        |facet| requested = Some(facet),
                    );
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(292.0, ui.available_height()),
                Layout::right_to_left(Align::Center),
                |ui| {
                    catalog_search(
                        ui,
                        &mut app.state.model_library_manager.filter_text,
                        "Filter project models…",
                    );
                },
            );
        });
    }
    if let Some(facet) = requested {
        app.state.workbench.model_project_facet = facet;
    }
}

pub(super) fn project_catalog_records(app: &RSpiceApp) -> Vec<ProjectCatalogRecord> {
    let usage = project_model_usage(app);
    let resolution_valid = app
        .state
        .model_library_manager
        .validate_definition_resolution()
        .is_ok();
    let contested = if resolution_valid {
        HashSet::new()
    } else {
        app.state
            .model_library_manager
            .definition_conflicts()
            .into_iter()
            .map(|conflict| conflict.normalized_name)
            .collect::<HashSet<_>>()
    };
    let mut records = Vec::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            let qualification = qualification_model_summary(app, library, model);
            let status = project_model_status(
                library,
                model,
                &qualification,
                contested.contains(&model.name.to_ascii_lowercase()),
            );
            let definition = ProjectCatalogDefinition::Model(model.clone());
            let key = project_catalog_key(&library.name, &definition);
            records.push(ProjectCatalogRecord {
                pinned: app.state.workbench.model_catalog_pinned.contains(&key),
                usage: usage
                    .get(&project_model_usage_key(&library.name, &model.name))
                    .cloned()
                    .unwrap_or_default(),
                key,
                library: library.name.clone(),
                definition,
                qualification: Some(qualification),
                status,
            });
        }
        for subcircuit in library.subcircuits.values() {
            let definition = ProjectCatalogDefinition::Subcircuit(subcircuit.clone());
            let key = project_catalog_key(&library.name, &definition);
            let status = if matches!(library.source_authority, ModelSourceAuthority::External)
                && library.source_closure.is_empty()
            {
                ProjectCatalogStatus::Blocked
            } else if subcircuit.ports.is_empty() {
                ProjectCatalogStatus::Review
            } else {
                ProjectCatalogStatus::Ready
            };
            records.push(ProjectCatalogRecord {
                pinned: app.state.workbench.model_catalog_pinned.contains(&key),
                usage: usage
                    .get(&project_subcircuit_usage_key(
                        &library.name,
                        subcircuit.section.as_deref(),
                        &subcircuit.name,
                    ))
                    .cloned()
                    .unwrap_or_default(),
                key,
                library: library.name.clone(),
                definition,
                qualification: None,
                status,
            });
        }
    }
    records.sort_by(|left, right| {
        left.definition
            .name()
            .to_ascii_lowercase()
            .cmp(&right.definition.name().to_ascii_lowercase())
            .then_with(|| {
                left.definition
                    .kind_token()
                    .cmp(right.definition.kind_token())
            })
            .then_with(|| {
                left.library
                    .to_ascii_lowercase()
                    .cmp(&right.library.to_ascii_lowercase())
            })
    });
    records
}

fn project_model_status(
    library: &ModelLibrary,
    model: &DeviceModel,
    qualification: &QualificationModelSummary,
    contested: bool,
) -> ProjectCatalogStatus {
    if contested
        || model_geometry_invalid(model)
        || qualification.source_error.is_some()
        || (matches!(library.source_authority, ModelSourceAuthority::External)
            && library.source_closure.is_empty())
    {
        return ProjectCatalogStatus::Blocked;
    }
    if qualification.suites == 0 || qualification.vectors == 0 {
        return ProjectCatalogStatus::Preview;
    }
    match qualification.gate {
        QualificationGate::Qualified => ProjectCatalogStatus::Qualified,
        QualificationGate::Review | QualificationGate::Unqualified => ProjectCatalogStatus::Review,
        QualificationGate::Blocked => ProjectCatalogStatus::Blocked,
    }
}

fn project_facet_matches(facet: ModelProjectFacet, record: &ProjectCatalogRecord) -> bool {
    match facet {
        ModelProjectFacet::All => true,
        ModelProjectFacet::Bound => !record.usage.is_empty(),
        ModelProjectFacet::Pinned => record.pinned,
        ModelProjectFacet::Review => matches!(
            record.status,
            ProjectCatalogStatus::Review | ProjectCatalogStatus::Blocked
        ),
        ModelProjectFacet::Preview => record.status == ProjectCatalogStatus::Preview,
        // Protected execution has no runtime model authority yet. Keeping the
        // real zero-result facet is more honest than inferring protection from
        // an ordinary external file.
        ModelProjectFacet::Protected => false,
    }
}

pub(super) fn project_record_matches_query(record: &ProjectCatalogRecord, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    if record
        .definition
        .name()
        .to_ascii_lowercase()
        .contains(query)
        || record.library.to_ascii_lowercase().contains(query)
        || record
            .usage
            .iter()
            .any(|usage| usage.to_ascii_lowercase().contains(query))
    {
        return true;
    }
    match &record.definition {
        ProjectCatalogDefinition::Model(model) => {
            model
                .model_type
                .display_name()
                .to_ascii_lowercase()
                .contains(query)
                || model
                    .spice_type
                    .as_deref()
                    .is_some_and(|value| value.to_ascii_lowercase().contains(query))
                || model
                    .file_path
                    .as_deref()
                    .is_some_and(|path| path.to_string_lossy().to_ascii_lowercase().contains(query))
                || model.parameters.iter().any(|(name, value)| {
                    name.to_ascii_lowercase().contains(query)
                        || value.to_string().to_ascii_lowercase().contains(query)
                })
                || model.string_parameters.iter().any(|(name, value)| {
                    name.to_ascii_lowercase().contains(query)
                        || value.to_ascii_lowercase().contains(query)
                })
        }
        ProjectCatalogDefinition::Subcircuit(subcircuit) => {
            "subcircuit".contains(query)
                || subcircuit
                    .ports
                    .iter()
                    .any(|port| port.to_ascii_lowercase().contains(query))
                || subcircuit.parameter_defaults.iter().any(|(name, value)| {
                    name.to_ascii_lowercase().contains(query)
                        || value.to_ascii_lowercase().contains(query)
                })
                || subcircuit
                    .file_path
                    .as_deref()
                    .is_some_and(|path| path.to_string_lossy().to_ascii_lowercase().contains(query))
        }
    }
}

fn selected_project_record(
    app: &RSpiceApp,
    records: &[ProjectCatalogRecord],
) -> Option<ProjectCatalogRecord> {
    if let Some(key) = app
        .state
        .workbench
        .model_catalog_selected_project
        .as_deref()
        && let Some(record) = records.iter().find(|record| record.key == key)
        && app.state.model_library_manager.selected_library.as_deref()
            == Some(record.library.as_str())
        && match &record.definition {
            ProjectCatalogDefinition::Model(model) => {
                app.state.workbench.selected_model.as_deref() == Some(model.name.as_str())
            }
            ProjectCatalogDefinition::Subcircuit(_) => app.state.workbench.selected_model.is_none(),
        }
    {
        return Some(record.clone());
    }
    let library = app
        .state
        .model_library_manager
        .selected_library
        .as_deref()?;
    let model = app.state.workbench.selected_model.as_deref()?;
    records
        .iter()
        .find(|record| {
            record.library == library
                && matches!(
                    &record.definition,
                    ProjectCatalogDefinition::Model(candidate) if candidate.name == model
                )
        })
        .cloned()
}

fn project_definition_detail(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: Option<&ProjectCatalogRecord>,
) {
    let Some(selected) = selected else {
        catalog_empty_detail(
            ui,
            "Select a project definition",
            "Inspect an exact model card or subcircuit interface, its authenticated source, and active-schematic consumers.",
        );
        return;
    };
    match &selected.definition {
        ProjectCatalogDefinition::Model(_) => project_model_detail(ui, app, selected),
        ProjectCatalogDefinition::Subcircuit(subcircuit) => {
            project_subcircuit_detail(ui, app, selected, subcircuit)
        }
    }
}

fn project_model_detail(ui: &mut Ui, app: &mut RSpiceApp, selected: &ProjectCatalogRecord) {
    let ProjectCatalogDefinition::Model(model) = &selected.definition else {
        return;
    };
    let Some(qualification) = selected.qualification.as_ref() else {
        return;
    };
    let Some(library) = app
        .state
        .model_library_manager
        .get_library(&selected.library)
        .cloned()
    else {
        catalog_empty_detail(
            ui,
            "Model source changed",
            "The selected model library no longer exists.",
        );
        return;
    };

    let t = Tokens::get(ui.ctx());
    let conflict = app
        .state
        .model_library_manager
        .definition_conflicts()
        .into_iter()
        .find(|conflict| conflict.normalized_name.eq_ignore_ascii_case(&model.name));
    let resolution = conflict.as_ref().and_then(|conflict| {
        app.state
            .model_library_manager
            .definition_resolution(&conflict.normalized_name)
            .cloned()
    });
    let resolution_error = app
        .state
        .model_library_manager
        .validate_definition_resolution()
        .err();
    let bind_target = project_model_bind_target(app, selected);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new(&model.name)
                .font(theme::sans(15.0, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.label(
            egui::RichText::new(format!(
                "{} · {}",
                model.model_type.display_name(),
                model.level.display_name()
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
        );
    });
    ui.add_space(5.0);
    ui.horizontal_wrapped(|ui| {
        let pin_label = if selected.pinned { "Unpin" } else { "Pin" };
        if Button::new(pin_label).show(ui).clicked() {
            if selected.pinned {
                app.state
                    .workbench
                    .model_catalog_pinned
                    .remove(&selected.key);
            } else {
                app.state
                    .workbench
                    .model_catalog_pinned
                    .insert(selected.key.clone());
            }
        }
        if Button::new("Open source").show(ui).clicked() {
            select_project_source_in_include_graph(
                app,
                &selected.library,
                model.file_path.as_ref().or(library.root_path.as_ref()),
            );
        }
        if Button::new("Model editor…").show(ui).clicked() {
            if let Err(error) = model_editor::open_project_model(
                app,
                selected.library.as_str(),
                model.name.as_str(),
            ) {
                app.state.push_user_message(ConsoleMessage::warning(format!(
                    "Cannot open model editor: {error}"
                )));
            }
        }
        if Button::new("Qualification…").show(ui).clicked() {
            app.state.workbench.models_page = ModelsPage::Qualification;
        }
        if Button::new("Correlation…").show(ui).clicked()
            && let Err(error) = app.state.workbench.navigate(
                SurfaceRoute::surface(SurfaceId::ModelCorrelation),
                RouteTransitionSource::User,
            )
        {
            app.state.push_user_message(ConsoleMessage::warning(format!(
                "Cannot open measurement correlation: {error}"
            )));
        }
        let bind_response = Button::new("Bind to selection")
            .enabled(bind_target.is_ok())
            .show(ui)
            .on_disabled_hover_text(
                bind_target
                    .as_ref()
                    .err()
                    .map(String::as_str)
                    .unwrap_or_default(),
            );
        if bind_response.clicked() {
            match bind_project_model_to_selected_component(app, selected) {
                Ok(message) => app.state.push_user_message(ConsoleMessage::info(message)),
                Err(error) => app.state.push_user_message(ConsoleMessage::warning(error)),
            }
        }
        if Button::new("Create bound symbol…")
            .accent()
            .show(ui)
            .clicked()
        {
            open_create_model_bound_symbol_dialog(&mut app.state);
        }
    });
    ui.add_space(8.0);

    property_card(ui, "Model and qualification", |ui| {
        property_row(ui, "Library", &selected.library);
        property_row(
            ui,
            "SPICE type",
            model.spice_type.as_deref().unwrap_or("not retained"),
        );
        property_row(
            ui,
            "Numeric level",
            &model
                .spice_level
                .map_or_else(|| "not declared".to_owned(), |level| level.to_string()),
        );
        property_row_toned(
            ui,
            "Release gate",
            selected.status.label(),
            selected.status.color(&t),
        );
        property_row_toned(
            ui,
            "Definition resolution",
            &match (&conflict, &resolution, &resolution_error) {
                (None, _, _) => "unique provider".to_owned(),
                (Some(conflict), Some(resolution), None) => format!(
                    "explicit winner {}/{} / {} providers",
                    resolution.provider_library,
                    resolution.provider_model,
                    conflict.providers.len()
                ),
                (Some(conflict), Some(resolution), Some(_)) => format!(
                    "selected {}/{} / precedence plan blocked / {} providers",
                    resolution.provider_library,
                    resolution.provider_model,
                    conflict.providers.len()
                ),
                (Some(conflict), None, _) => format!(
                    "unresolved / {} providers / simulation blocked",
                    conflict.providers.len()
                ),
            },
            if conflict.is_some() && resolution_error.is_some() {
                t.color.err
            } else {
                t.color.ok
            },
        );
        property_row(
            ui,
            "Qualification suites",
            &qualification.suites.to_string(),
        );
        property_row(
            ui,
            "Passing vectors",
            &format!(
                "{} / {}",
                qualification.passing_vectors, qualification.vectors
            ),
        );
        property_row(
            ui,
            "Where used",
            if selected.usage.is_empty() {
                "not bound in loaded schematics"
            } else {
                "bound in project"
            },
        );
    });
    ui.add_space(7.0);

    property_card(ui, "Source authority", |ui| {
        property_row(
            ui,
            "Authority",
            match library.source_authority {
                ModelSourceAuthority::BuiltIn => "built-in runtime",
                ModelSourceAuthority::External => "external pinned source",
                ModelSourceAuthority::RetainedImport { .. } => "read-only retained imported bytes",
                ModelSourceAuthority::ProjectOwned { .. } => "project-owned retained bytes",
            },
        );
        property_row(
            ui,
            "Definition",
            &model
                .file_path
                .as_deref()
                .or(library.root_path.as_deref())
                .map_or_else(|| "in-memory".to_owned(), |path| path.display().to_string()),
        );
        property_row(
            ui,
            "Source line",
            &model
                .source_line
                .map_or_else(|| "not retained".to_owned(), |line| line.to_string()),
        );
        property_row(
            ui,
            "Pinned closure",
            &format!("{} file(s)", library.source_closure.len()),
        );
        if let Some(pin) = library.source_closure.iter().find(|pin| {
            model
                .file_path
                .as_ref()
                .is_some_and(|path| path == &pin.path)
        }) {
            property_row(ui, "Content digest", &short_digest(&pin.digest.to_string()));
        }
    });
    ui.add_space(7.0);

    property_card(ui, "Resolved parameters", |ui| {
        let mut numeric = model.parameters.iter().collect::<Vec<_>>();
        numeric.sort_by_key(|(name, _)| name.to_ascii_lowercase());
        let mut strings = model.string_parameters.iter().collect::<Vec<_>>();
        strings.sort_by_key(|(name, _)| name.to_ascii_lowercase());
        if numeric.is_empty() && strings.is_empty() {
            property_row(ui, "Parameters", "none retained by this parser");
        } else {
            for (name, value) in numeric.into_iter().take(80) {
                property_row(ui, name, &format!("{value:.12}"));
            }
            for (name, value) in strings.into_iter().take(80) {
                property_row(ui, name, value);
            }
            let total = model.parameters.len() + model.string_parameters.len();
            if total > 160 {
                property_row(ui, "Additional", &format!("{} parameter(s)", total - 160));
            }
        }
    });
    ui.add_space(7.0);

    if let Some((start_line, preview)) = project_source_preview(&library, model) {
        source_preview_card(
            ui,
            "Authenticated card preview",
            start_line,
            &preview,
            false,
        );
    }
    ui.add_space(7.0);

    property_card(ui, "Where used", |ui| {
        if selected.usage.is_empty() {
            property_row(ui, "Consumers", "none in loaded schematic documents");
        } else {
            for usage in &selected.usage {
                property_row(ui, "Instance", usage);
            }
        }
    });
    ui.add_space(7.0);

    property_card(ui, "Characteristic preview", |ui| {
        property_row(
            ui,
            "Status",
            if qualification.vectors == 0 {
                "no executable vector"
            } else {
                "available from qualification evidence"
            },
        );
        ui.add_space(5.0);
        ui.horizontal_wrapped(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(
                    "RSpice never synthesizes a datasheet curve from catalog metadata. Open the qualification workspace to run or inspect the exact vector, source revision, tolerances, and result evidence.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        });
    });
}

fn project_subcircuit_detail(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &ProjectCatalogRecord,
    subcircuit: &ModelSubcircuitInterface,
) {
    let Some(library) = app
        .state
        .model_library_manager
        .get_library(&selected.library)
        .cloned()
    else {
        catalog_empty_detail(
            ui,
            "Subcircuit source changed",
            "The selected model library no longer exists.",
        );
        return;
    };
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new(&subcircuit.name)
                .font(theme::sans(15.0, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.label(
            egui::RichText::new(format!(
                "Subcircuit · {} terminal(s)",
                subcircuit.ports.len()
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
        );
    });
    ui.add_space(5.0);
    ui.horizontal_wrapped(|ui| {
        let pin_label = if selected.pinned { "Unpin" } else { "Pin" };
        if Button::new(pin_label).show(ui).clicked() {
            if selected.pinned {
                app.state
                    .workbench
                    .model_catalog_pinned
                    .remove(&selected.key);
            } else {
                app.state
                    .workbench
                    .model_catalog_pinned
                    .insert(selected.key.clone());
            }
        }
        if Button::new("Open source").show(ui).clicked() {
            select_project_source_in_include_graph(
                app,
                &selected.library,
                subcircuit.file_path.as_ref().or(library.root_path.as_ref()),
            );
        }
        let can_create = !subcircuit.ports.is_empty() && subcircuit.file_path.is_some();
        ui.add_enabled_ui(can_create, |ui| {
            if Button::new("Create bound symbol…")
                .accent()
                .show(ui)
                .clicked()
            {
                let result = open_create_subcircuit_bound_symbol_dialog(
                    &mut app.state,
                    selected.library.clone(),
                    subcircuit.name.clone(),
                    subcircuit
                        .file_path
                        .clone()
                        .expect("enabled action has an authenticated source path"),
                    subcircuit.ports.clone(),
                    subcircuit.section.clone(),
                    subcircuit.parameter_defaults.clone(),
                );
                if let Err(error) = result {
                    app.state.push_user_message(ConsoleMessage::warning(format!(
                        "Cannot create subcircuit symbol: {error}"
                    )));
                }
            }
        });
    });
    if subcircuit.ports.is_empty() {
        ui.label(
            egui::RichText::new(
                "This valid SPICE subcircuit has no external terminals, so it cannot become a schematic symbol.",
            )
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.warn),
        );
    }
    ui.add_space(8.0);

    property_card(ui, "Subcircuit interface", |ui| {
        property_row(ui, "Library", &selected.library);
        property_row_toned(
            ui,
            "Contract status",
            selected.status.label(),
            selected.status.color(&t),
        );
        property_row(
            ui,
            "Section",
            subcircuit.section.as_deref().unwrap_or("top level"),
        );
        property_row(ui, "Ordered terminals", &subcircuit.ports.len().to_string());
        property_row(
            ui,
            "Parameter defaults",
            &subcircuit.parameter_defaults.len().to_string(),
        );
        property_row(
            ui,
            "Where used",
            if selected.usage.is_empty() {
                "not bound in loaded schematics"
            } else {
                "bound in project"
            },
        );
    });
    ui.add_space(7.0);

    property_card(ui, "Ordered terminal contract", |ui| {
        if subcircuit.ports.is_empty() {
            property_row(ui, "Terminals", "none declared");
        } else {
            for (index, port) in subcircuit.ports.iter().enumerate() {
                property_row(ui, &format!("{}", index + 1), port);
            }
        }
    });
    ui.add_space(7.0);

    property_card(ui, "Instance parameter defaults", |ui| {
        if subcircuit.parameter_defaults.is_empty() {
            property_row(ui, "Parameters", "none declared");
        } else {
            for (name, value) in &subcircuit.parameter_defaults {
                property_row(ui, name, value);
            }
        }
    });
    ui.add_space(7.0);

    property_card(ui, "Source authority", |ui| {
        property_row(
            ui,
            "Authority",
            match library.source_authority {
                ModelSourceAuthority::BuiltIn => "built-in runtime",
                ModelSourceAuthority::External => "external pinned source",
                ModelSourceAuthority::RetainedImport { .. } => "read-only retained imported bytes",
                ModelSourceAuthority::ProjectOwned { .. } => "project-owned retained bytes",
            },
        );
        property_row(
            ui,
            "Definition",
            &subcircuit
                .file_path
                .as_deref()
                .or(library.root_path.as_deref())
                .map_or_else(|| "in-memory".to_owned(), |path| path.display().to_string()),
        );
        property_row(
            ui,
            "Source line",
            &subcircuit
                .source_line
                .map_or_else(|| "not retained".to_owned(), |line| line.to_string()),
        );
        property_row(
            ui,
            "Pinned closure",
            &format!("{} file(s)", library.source_closure.len()),
        );
        if let Some(pin) = library.source_closure.iter().find(|pin| {
            subcircuit
                .file_path
                .as_ref()
                .is_some_and(|path| path == &pin.path)
        }) {
            property_row(ui, "Content digest", &short_digest(&pin.digest.to_string()));
        }
    });
    ui.add_space(7.0);

    if let Some((start_line, preview, truncated)) =
        project_subcircuit_source_preview(&library, subcircuit)
    {
        source_preview_card(
            ui,
            "Authenticated subcircuit preview",
            start_line,
            &preview,
            truncated,
        );
        ui.add_space(7.0);
    }

    property_card(ui, "Where used", |ui| {
        if selected.usage.is_empty() {
            property_row(ui, "Consumers", "none in loaded schematic documents");
        } else {
            for usage in &selected.usage {
                property_row(ui, "Instance", usage);
            }
        }
    });
}

fn project_record_source(app: &RSpiceApp, record: &ProjectCatalogRecord) -> String {
    app.state
        .model_library_manager
        .get_library(&record.library)
        .map(|library| match &record.definition {
            ProjectCatalogDefinition::Model(model) => model_source_label(library, model),
            ProjectCatalogDefinition::Subcircuit(subcircuit) => subcircuit
                .file_path
                .as_deref()
                .or(library.root_path.as_deref())
                .map(path_display_name)
                .unwrap_or_else(|| "in-memory".to_owned()),
        })
        .unwrap_or_else(|| "source changed".to_owned())
}

fn project_catalog_key(library: &str, definition: &ProjectCatalogDefinition) -> String {
    match definition {
        ProjectCatalogDefinition::Model(model) => {
            format!("model\u{1e}{library}\u{1f}{}", model.name)
        }
        ProjectCatalogDefinition::Subcircuit(subcircuit) => format!(
            "subcircuit\u{1e}{library}\u{1f}{}\u{1f}{}",
            subcircuit.section.as_deref().unwrap_or_default(),
            subcircuit.name
        ),
    }
}

fn select_project_source_in_include_graph(
    app: &mut RSpiceApp,
    library: &str,
    source: Option<&std::path::PathBuf>,
) {
    app.state.workbench.model_include_selected_library = Some(library.to_owned());
    app.state.workbench.model_include_selected_source = source.cloned();
    app.state.workbench.models_page = ModelsPage::Include;
}

fn project_model_bind_target(
    app: &RSpiceApp,
    selected: &ProjectCatalogRecord,
) -> Result<(u64, String), String> {
    let ProjectCatalogDefinition::Model(model) = &selected.definition else {
        return Err("Select a model card before binding an instance.".to_owned());
    };
    if app.state.schematic_edit_read_only() {
        return Err(app.state.read_only_master_message());
    }
    if selected.status == ProjectCatalogStatus::Blocked {
        return Err(
            "This model is blocked by its source, geometry, qualification, or definition-resolution contract."
                .to_owned(),
        );
    }
    let library = app
        .state
        .model_library_manager
        .get_library(&selected.library)
        .ok_or_else(|| "The selected model library no longer exists.".to_owned())?;
    if !library.source_authority.has_execution_source() {
        return Err(
            "This catalog entry has no executable source. Attach or import an authenticated model source first."
                .to_owned(),
        );
    }
    if app
        .state
        .model_library_manager
        .definition_conflicts()
        .into_iter()
        .any(|conflict| conflict.normalized_name.eq_ignore_ascii_case(&model.name))
        && !app
            .state
            .model_library_manager
            .definition_resolution(&model.name)
            .is_some_and(|resolution| {
                resolution.provider_library == selected.library
                    && resolution.provider_model == model.name
            })
    {
        return Err(format!(
            "Resolve '{}' to the exact {}/{} provider in Include Graph before binding it.",
            model.name, selected.library, model.name
        ));
    }

    let component_id = app
        .state
        .schematic
        .selection
        .single_component()
        .ok_or_else(|| "Select exactly one schematic component to bind.".to_owned())?;
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .ok_or_else(|| "The selected schematic component no longer exists.".to_owned())?;
    if component.library_cell.is_some() {
        return Err(
            "Library-cell instances must be rebound through their model-bound symbol contract."
                .to_owned(),
        );
    }
    if !project_model_matches_component(model, component.kind) {
        return Err(format!(
            "{} '{}' cannot use a {} {} model card.",
            component.kind.display_name(),
            component.name,
            model.model_type.display_name(),
            model.level.display_name()
        ));
    }
    Ok((component_id, component.name.clone()))
}

fn bind_project_model_to_selected_component(
    app: &mut RSpiceApp,
    selected: &ProjectCatalogRecord,
) -> Result<String, String> {
    let ProjectCatalogDefinition::Model(model) = &selected.definition else {
        return Err("Select a model card before binding an instance.".to_owned());
    };
    let (component_id, component_name) = project_model_bind_target(app, selected)?;
    let before = crate::state::SchematicSnapshot::capture(&app.state.schematic);
    let component = app
        .state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == component_id)
        .ok_or_else(|| "The selected schematic component no longer exists.".to_owned())?;

    let previous_value = component.value.clone();
    let previous_params = component.params.clone();
    let mut params = crate::state::parse_params_string(&component.params);
    if component.kind == ComponentType::SaturableInductor {
        params.insert("model".to_owned(), model.name.clone());
    } else {
        component.value.clone_from(&model.name);
        params.remove("model");
    }
    params.insert("model_library".to_owned(), selected.library.clone());
    params.remove("model_corner");
    component.params = crate::state::format_params_string(&params);

    if component.value == previous_value && component.params == previous_params {
        return Ok(format!(
            "{} is already bound to {}/{}.",
            component_name, selected.library, model.name
        ));
    }

    app.state.schematic.is_dirty = true;
    app.state.schematic.bump_topology_version();
    app.state
        .schematic
        .commit_undo_from(before, "bind selected instance model");
    app.invalidate_simulation_preflight();
    Ok(format!(
        "Bound {} to {}/{}.",
        component_name, selected.library, model.name
    ))
}

fn project_model_matches_component(model: &DeviceModel, component: ComponentType) -> bool {
    let spice_type = model
        .spice_type
        .as_deref()
        .map(str::trim)
        .unwrap_or_default();
    let token_is = |candidates: &[&str]| {
        candidates
            .iter()
            .any(|candidate| spice_type.eq_ignore_ascii_case(candidate))
    };
    let soi = matches!(
        model.level,
        ModelLevel::BsimSoi | ModelLevel::BsimImg | ModelLevel::LUtsoi
    );
    match component {
        ComponentType::Nmos => {
            model.model_type == ModelType::Nmos && model.level != ModelLevel::Vdmos && !soi
        }
        ComponentType::Pmos => {
            model.model_type == ModelType::Pmos && model.level != ModelLevel::Vdmos && !soi
        }
        ComponentType::NVdmos => {
            (model.model_type == ModelType::Nmos && model.level == ModelLevel::Vdmos)
                || token_is(&["VDMOS", "NVDMOS"])
        }
        ComponentType::PVdmos => {
            (model.model_type == ModelType::Pmos && model.level == ModelLevel::Vdmos)
                || token_is(&["PVDMOS"])
        }
        ComponentType::NmosSoi => model.model_type == ModelType::Nmos && soi,
        ComponentType::PmosSoi => model.model_type == ModelType::Pmos && soi,
        ComponentType::NpnBjt | ComponentType::NpnBjt4 => {
            model.model_type == ModelType::Npn || token_is(&["NPN"])
        }
        ComponentType::PnpBjt | ComponentType::PnpBjt4 => {
            model.model_type == ModelType::Pnp || token_is(&["PNP"])
        }
        ComponentType::NpnBjt5 => {
            (model.model_type == ModelType::Npn || token_is(&["NPN"]))
                && model.level == ModelLevel::Vbic
        }
        ComponentType::PnpBjt5 => {
            (model.model_type == ModelType::Pnp || token_is(&["PNP"]))
                && model.level == ModelLevel::Vbic
        }
        ComponentType::Diode => {
            matches!(
                model.model_type,
                ModelType::Diode | ModelType::Varactor | ModelType::Esd
            ) || token_is(&["D", "DIODE", "DIODE_CMC", "JUNCAP"])
        }
        ComponentType::Njfet => token_is(&["NJF"]),
        ComponentType::Pjfet => token_is(&["PJF"]),
        ComponentType::Nmesfet => token_is(&["NMF", "NHFET"]),
        ComponentType::Pmesfet => token_is(&["PMF", "PHFET"]),
        ComponentType::VSwitch => token_is(&["SW", "SWITCH", "VSW", "VSWITCH"]),
        ComponentType::ISwitch => token_is(&["CSW", "ISW", "ISWITCH"]),
        ComponentType::SaturableInductor => token_is(&["CORE"]),
        ComponentType::Memristor => token_is(&["MEMRISTOR"]),
        ComponentType::LossyTransmissionLine => token_is(&["LTRA", "TXL"]),
        ComponentType::CoupledTransmissionLine => token_is(&["CPL"]),
        _ => false,
    }
}

fn project_source_preview(library: &ModelLibrary, model: &DeviceModel) -> Option<(usize, String)> {
    let preferred = model.file_path.as_ref().or(library.root_path.as_ref());
    let content = preferred
        .and_then(|path| {
            library
                .source_contents
                .iter()
                .find(|content| &content.path == path)
        })
        .or_else(|| library.source_contents.first())?;
    let source = std::str::from_utf8(&content.bytes).ok()?;
    let start_line = model.source_line.unwrap_or(1);
    let lines = source.lines().collect::<Vec<_>>();
    let start = start_line.checked_sub(1)?;
    if start >= lines.len() {
        return None;
    }
    let mut captured = Vec::new();
    for (offset, line) in lines[start..].iter().enumerate() {
        if offset > 0 {
            let trimmed = line.trim_start();
            if !trimmed.starts_with('+')
                && !trimmed.starts_with('*')
                && !trimmed.starts_with(';')
                && !trimmed.is_empty()
            {
                break;
            }
        }
        captured.push(*line);
        if captured.len() == 80 {
            break;
        }
    }
    (!captured.is_empty()).then(|| (start_line, captured.join("\n")))
}

fn project_subcircuit_source_preview(
    library: &ModelLibrary,
    subcircuit: &ModelSubcircuitInterface,
) -> Option<(usize, String, bool)> {
    let preferred = subcircuit.file_path.as_ref().or(library.root_path.as_ref());
    let content = preferred
        .and_then(|path| {
            library
                .source_contents
                .iter()
                .find(|content| &content.path == path)
        })
        .or_else(|| library.source_contents.first())?;
    let source = std::str::from_utf8(&content.bytes).ok()?;
    let start_line = subcircuit.source_line?;
    let lines = source.lines().collect::<Vec<_>>();
    let start = start_line.checked_sub(1)?;
    if start >= lines.len() {
        return None;
    }
    let mut captured = Vec::new();
    let mut complete = false;
    for line in &lines[start..] {
        captured.push(*line);
        if line
            .trim_start()
            .split_whitespace()
            .next()
            .is_some_and(|directive| directive.eq_ignore_ascii_case(".ends"))
        {
            complete = true;
            break;
        }
        if captured.len() == 240 {
            break;
        }
    }
    (!captured.is_empty()).then(|| (start_line, captured.join("\n"), !complete))
}

fn project_model_usage(app: &RSpiceApp) -> HashMap<String, Vec<String>> {
    let mut usage = HashMap::<String, Vec<String>>::new();
    let active_key = app.state.workspace.active_view.key();
    collect_schematic_model_usage(
        &app.state.schematic,
        &app.state.workspace.active_view.display_path(),
        &app.state.model_library_manager,
        &mut usage,
    );
    for (key, schematic) in &app.state.workspace.schematic_buffers {
        if key == &active_key {
            continue;
        }
        collect_schematic_model_usage(schematic, key, &app.state.model_library_manager, &mut usage);
    }
    for values in usage.values_mut() {
        values.sort_by_key(|value| value.to_ascii_lowercase());
        values.dedup();
    }
    usage
}

fn collect_schematic_model_usage(
    schematic: &SchematicState,
    owner: &str,
    manager: &ModelLibraryManager,
    usage: &mut HashMap<String, Vec<String>>,
) {
    for component in &schematic.components {
        if let Some(binding) = component.library_cell.as_ref() {
            let definition_name = binding
                .module_name
                .as_deref()
                .unwrap_or(binding.cell.as_str());
            if let Some(library) = manager.get_library(&binding.library) {
                if let Some(model) = library
                    .models
                    .values()
                    .find(|model| model.name.eq_ignore_ascii_case(definition_name))
                {
                    usage
                        .entry(project_model_usage_key(&library.name, &model.name))
                        .or_default()
                        .push(format!("{owner}/{}", component.name));
                    continue;
                }
                if let Some(subcircuit) = library.subcircuits.values().find(|subcircuit| {
                    subcircuit.name.eq_ignore_ascii_case(definition_name)
                        && option_eq_ignore_ascii_case(
                            subcircuit.section.as_deref(),
                            binding.model_section.as_deref(),
                        )
                }) {
                    usage
                        .entry(project_subcircuit_usage_key(
                            &library.name,
                            subcircuit.section.as_deref(),
                            &subcircuit.name,
                        ))
                        .or_default()
                        .push(format!("{owner}/{}", component.name));
                }
            }
            continue;
        }
        let Some(model) = explicit_component_model_name(component) else {
            continue;
        };
        let parameters = crate::state::parse_params_string(&component.params);
        let hinted_library = parameters
            .get("model_library")
            .map(String::as_str)
            .map(str::trim)
            .filter(|library| !library.is_empty());
        let Some((library, model)) = resolved_model_usage_provider(manager, &model, hinted_library)
        else {
            continue;
        };
        usage
            .entry(project_model_usage_key(&library, &model))
            .or_default()
            .push(format!("{}/{}", owner, component.name));
    }
}

fn resolved_model_usage_provider(
    manager: &ModelLibraryManager,
    model_name: &str,
    hinted_library: Option<&str>,
) -> Option<(String, String)> {
    if let Some(hinted_library) = hinted_library {
        let library = manager
            .libraries_sorted()
            .into_iter()
            .find(|library| library.name.eq_ignore_ascii_case(hinted_library))?;
        let model = library
            .models
            .values()
            .find(|model| model.name.eq_ignore_ascii_case(model_name))?;
        let conflict = manager
            .definition_conflicts()
            .into_iter()
            .find(|conflict| conflict.normalized_name.eq_ignore_ascii_case(model_name));
        if conflict.is_some()
            && !manager
                .definition_resolution(model_name)
                .is_some_and(|resolution| {
                    resolution.provider_library == library.name
                        && resolution.provider_model == model.name
                })
        {
            return None;
        }
        return Some((library.name.clone(), model.name.clone()));
    }

    if let Some(conflict) = manager
        .definition_conflicts()
        .into_iter()
        .find(|conflict| conflict.normalized_name.eq_ignore_ascii_case(model_name))
    {
        let resolution = manager.definition_resolution(&conflict.normalized_name)?;
        return conflict
            .providers
            .iter()
            .find(|provider| {
                provider.library == resolution.provider_library
                    && provider.model == resolution.provider_model
            })
            .map(|provider| (provider.library.clone(), provider.model.clone()));
    }

    let mut providers = manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| {
            library
                .models
                .values()
                .find(|model| model.name.eq_ignore_ascii_case(model_name))
                .map(|model| (library.name.clone(), model.name.clone()))
        });
    let provider = providers.next()?;
    providers.next().is_none().then_some(provider)
}

fn project_model_usage_key(library: &str, model: &str) -> String {
    format!(
        "model\u{1e}{}\u{1f}{}",
        library.to_ascii_lowercase(),
        model.to_ascii_lowercase()
    )
}

fn project_subcircuit_usage_key(library: &str, section: Option<&str>, subcircuit: &str) -> String {
    format!(
        "subcircuit\u{1e}{}\u{1f}{}\u{1f}{}",
        library.to_ascii_lowercase(),
        section.unwrap_or_default().to_ascii_lowercase(),
        subcircuit.to_ascii_lowercase()
    )
}

fn option_eq_ignore_ascii_case(left: Option<&str>, right: Option<&str>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.eq_ignore_ascii_case(right),
        (None, None) => true,
        _ => false,
    }
}

fn explicit_component_model_name(component: &Component) -> Option<String> {
    let parameters = crate::state::parse_params_string(&component.params);
    if let Some(model) = parameters
        .get("model")
        .map(String::as_str)
        .map(str::trim)
        .filter(|model| !model.is_empty())
    {
        return Some(model.to_owned());
    }
    if matches!(
        component.kind,
        ComponentType::NpnBjt
            | ComponentType::PnpBjt
            | ComponentType::NpnBjt4
            | ComponentType::PnpBjt4
            | ComponentType::NpnBjt5
            | ComponentType::PnpBjt5
            | ComponentType::VSwitch
            | ComponentType::ISwitch
            | ComponentType::Diode
            | ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos
            | ComponentType::NmosSoi
            | ComponentType::PmosSoi
            | ComponentType::Njfet
            | ComponentType::Pjfet
            | ComponentType::Nmesfet
            | ComponentType::Pmesfet
            | ComponentType::Memristor
            | ComponentType::LossyTransmissionLine
            | ComponentType::CoupledTransmissionLine
    ) {
        let value = component.value.trim();
        return (!value.is_empty()).then(|| value.to_owned());
    }
    None
}

fn pack_catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    let packs = app
        .state
        .model_library_manager
        .spice_packs()
        .map_or_else(Vec::new, |index| index.packs().to_vec());
    let counts = ModelPackFacet::ALL.map(|facet| {
        packs
            .iter()
            .filter(|pack| pack_facet_matches(app, facet, pack))
            .count()
    });
    pack_toolbar(ui, app, counts);
    ui.add_space(5.0);

    let query = app
        .state
        .workbench
        .model_pack_query
        .trim()
        .to_ascii_lowercase();
    let visible = packs
        .iter()
        .filter(|pack| pack_facet_matches(app, app.state.workbench.model_pack_facet, pack))
        .filter(|pack| pack_matches_query(pack, &query))
        .cloned()
        .collect::<Vec<_>>();
    let t = Tokens::get(ui.ctx());
    let rows = visible
        .iter()
        .map(|pack| {
            let attached = app.state.model_library_manager.is_pack_attached(&pack.id);
            let (status, tone) = if attached {
                ("attached", t.color.ok)
            } else if !pack.redistributable {
                ("browse only", t.color.warn)
            } else if pack.entry.is_some() {
                ("ready", t.color.info)
            } else {
                ("browse parts", t.color.text_dim)
            };
            DataRow {
                key: pack.id.clone(),
                selected: app.state.workbench.model_catalog_selected_pack.as_deref()
                    == Some(pack.id.as_str()),
                cells: vec![
                    DataCell::plain(&pack.name),
                    DataCell::plain(pack.category_label()),
                    DataCell::mono(if pack.devices.is_empty() {
                        "not classified".to_owned()
                    } else {
                        pack.devices.join(", ")
                    }),
                    DataCell::mono(grouped_count(pack.models_top)),
                    DataCell::mono(grouped_count(pack.subcircuits_top)),
                    DataCell::mono(format!("{} · {}", pack.spdx, pack.tier.display_name())),
                    DataCell::mono_colored(status, tone),
                ],
            }
        })
        .collect::<Vec<_>>();
    let selected = app
        .state
        .workbench
        .model_catalog_selected_pack
        .as_deref()
        .and_then(|id| packs.iter().find(|pack| pack.id == id))
        .cloned();
    let columns = [
        ("Pack", 0.21),
        ("Origin", 0.10),
        ("Device classes", 0.21),
        ("Models", 0.10),
        ("Subcircuits", 0.11),
        ("License", 0.17),
        ("State", 0.10),
    ];
    let event = catalog_browser(
        ui,
        "models.catalog.packs",
        &columns,
        &rows,
        if packs.is_empty() {
            "No installed model-pack manifest was found on this platform."
        } else {
            "No installed packs match the active facet and filter."
        },
        |ui| pack_detail(ui, app, selected.as_ref()),
    );
    if let Some(event) = event
        && packs.iter().any(|pack| pack.id == event.key)
    {
        app.state.workbench.model_catalog_selected_pack = Some(event.key);
    }
}

trait PackPresentation {
    fn category_label(&self) -> &'static str;
}

impl PackPresentation for SpicePack {
    fn category_label(&self) -> &'static str {
        match self.category.as_str() {
            "foundry" => "Foundry",
            "vendor" => "Vendor",
            "community" => "Community",
            "academic" => "Academic",
            "builtin" => "RSpice",
            _ => "Other",
        }
    }
}

fn pack_toolbar(ui: &mut Ui, app: &mut RSpiceApp, counts: [usize; 7]) {
    let width = ui.available_width();
    let current = app.state.workbench.model_pack_facet;
    let mut requested = None;
    if width < CATALOG_TOOLBAR_BREAKPOINT {
        facet_strip(
            ui,
            "models.catalog.pack.facets",
            ModelPackFacet::ALL
                .into_iter()
                .zip(counts)
                .map(|(facet, count)| (facet.label(), count, facet == current, facet)),
            |facet| requested = Some(facet),
        );
        ui.add_space(4.0);
        catalog_search(
            ui,
            &mut app.state.workbench.model_pack_query,
            "Filter installed packs, origins, devices, or licenses…",
        );
    } else {
        ui.horizontal(|ui| {
            ui.set_width(width);
            ui.allocate_ui_with_layout(
                egui::vec2((width - 300.0).max(1.0), ui.available_height()),
                Layout::left_to_right(Align::Center),
                |ui| {
                    facet_strip(
                        ui,
                        "models.catalog.pack.facets",
                        ModelPackFacet::ALL
                            .into_iter()
                            .zip(counts)
                            .map(|(facet, count)| (facet.label(), count, facet == current, facet)),
                        |facet| requested = Some(facet),
                    );
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(292.0, ui.available_height()),
                Layout::right_to_left(Align::Center),
                |ui| {
                    catalog_search(
                        ui,
                        &mut app.state.workbench.model_pack_query,
                        "Filter installed packs…",
                    );
                },
            );
        });
    }
    if let Some(facet) = requested {
        app.state.workbench.model_pack_facet = facet;
    }
}

fn pack_facet_matches(app: &RSpiceApp, facet: ModelPackFacet, pack: &SpicePack) -> bool {
    match facet {
        ModelPackFacet::All => true,
        ModelPackFacet::Attention => !pack.redistributable,
        ModelPackFacet::Attached => app.state.model_library_manager.is_pack_attached(&pack.id),
        ModelPackFacet::Foundry => pack.category == "foundry",
        ModelPackFacet::Vendor => pack.category == "vendor",
        ModelPackFacet::Community => matches!(pack.category.as_str(), "community" | "academic"),
        ModelPackFacet::Redistributable => pack.redistributable,
    }
}

fn pack_matches_query(pack: &SpicePack, query: &str) -> bool {
    query.is_empty()
        || pack.id.to_ascii_lowercase().contains(query)
        || pack.name.to_ascii_lowercase().contains(query)
        || pack.category.to_ascii_lowercase().contains(query)
        || pack.spdx.to_ascii_lowercase().contains(query)
        || pack
            .devices
            .iter()
            .any(|device| device.to_ascii_lowercase().contains(query))
}

fn pack_detail(ui: &mut Ui, app: &mut RSpiceApp, selected: Option<&SpicePack>) {
    let Some(pack) = selected else {
        catalog_empty_detail(
            ui,
            "Select an installed pack",
            "Inspect its exact local manifest, licensing authority, contents, and project attachment state.",
        );
        return;
    };
    let t = Tokens::get(ui.ctx());
    let attached = app.state.model_library_manager.is_pack_attached(&pack.id);
    let devices_label = if pack.devices.is_empty() {
        "not classified".to_owned()
    } else {
        pack.devices.join(", ")
    };
    ui.label(
        egui::RichText::new(&pack.name)
            .font(theme::sans(15.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(&pack.id)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(6.0);
    ui.horizontal_wrapped(|ui| {
        if Button::new("Browse parts").accent().show(ui).clicked() {
            app.state.workbench.model_catalog_scope = ModelCatalogScope::Library;
            app.state.workbench.model_catalog_selected_pack = Some(pack.id.clone());
            app.state.workbench.model_catalog_selected_part = None;
            app.state.workbench.model_catalog_part_page = 0;
        }
        if attached {
            if Button::new("Detach…")
                .enabled(project_catalog_writable(app))
                .show(ui)
                .on_disabled_hover_text(project_write_block_reason(app))
                .clicked()
            {
                app.state.workbench.model_catalog_detach_pack = Some(pack.id.clone());
            }
        } else {
            let sources_available = installed_pack_sources_available(app);
            let can_attach = sources_available
                && pack.redistributable
                && pack.entry.is_some()
                && project_catalog_writable(app);
            if Button::new("Attach pack")
                .enabled(can_attach)
                .show(ui)
                .on_disabled_hover_text(if !sources_available {
                    browser_pack_source_block_reason()
                } else if !pack.redistributable {
                    "This pack is browse-only because redistribution authority is not established."
                } else if pack.entry.is_none() {
                    "This pack has no declared aggregate entry source. Browse and add an individual model."
                } else {
                    project_write_block_reason(app)
                })
                .clicked()
            {
                attach_pack_to_project(app, pack);
            }
        }
    });
    ui.add_space(8.0);
    property_card(ui, "Installed manifest", |ui| {
        property_row(ui, "Origin", pack.category_label());
        property_row(ui, "Pack path", &pack.path.display().to_string());
        property_row(
            ui,
            "Entry source",
            &pack.entry.as_deref().map_or_else(
                || "not declared".to_owned(),
                |path| path.display().to_string(),
            ),
        );
        property_row(ui, "Files", &grouped_count(pack.files));
        property_row(ui, "On disk", &format_storage(pack.bytes));
        property_row(
            ui,
            "Update channel",
            "installed with the RSpice application release",
        );
    });
    ui.add_space(7.0);
    property_card(ui, "Contents", |ui| {
        property_row(ui, "Addressable models", &grouped_count(pack.models_top));
        property_row(
            ui,
            "Addressable subcircuits",
            &grouped_count(pack.subcircuits_top),
        );
        property_row(ui, "All model definitions", &grouped_count(pack.models));
        property_row(
            ui,
            "All subcircuit definitions",
            &grouped_count(pack.subcircuits),
        );
        property_row(ui, "Device classes", &devices_label);
    });
    ui.add_space(7.0);
    property_card(ui, "License and execution", |ui| {
        property_row(ui, "SPDX", &pack.spdx);
        property_row(ui, "License tier", pack.tier.display_name());
        property_row_toned(
            ui,
            "Redistribution",
            if pack.redistributable {
                "established"
            } else {
                "not established"
            },
            if pack.redistributable {
                t.color.ok
            } else {
                t.color.warn
            },
        );
        property_row_toned(
            ui,
            "Project state",
            if attached { "attached" } else { "not attached" },
            if attached {
                t.color.ok
            } else {
                t.color.text_dim
            },
        );
    });
}

fn attach_pack_to_project(app: &mut RSpiceApp, pack: &SpicePack) {
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate.attach_pack(&pack.id).and_then(|library| {
        app.publish_model_library_candidate(candidate)
            .map(|()| library)
    });
    match result {
        Ok(library) => app.state.push_user_message(ConsoleMessage::info(format!(
            "Attached installed pack '{}' as executable library '{}'.",
            pack.name, library
        ))),
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Could not attach installed pack '{}': {error}",
            pack.name
        ))),
    }
}

fn render_pack_detach_confirmation(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(pack_id) = app.state.workbench.model_catalog_detach_pack.clone() else {
        return;
    };
    let pack_name = app
        .state
        .model_library_manager
        .spice_packs()
        .and_then(|index| index.pack(&pack_id))
        .map_or_else(|| pack_id.clone(), |pack| pack.name.clone());

    let screen = ctx.content_rect();
    egui::Area::new(egui::Id::new("models.catalog.detach.backdrop"))
        .order(egui::Order::Foreground)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(screen.size(), Sense::click_and_drag());
            ui.painter()
                .rect_filled(rect, 0.0, Color32::from_black_alpha(145));
        });

    let mut cancel = false;
    let mut confirm = false;
    egui::Window::new("Detach installed model pack?")
        .id(egui::Id::new("models.catalog.detach.dialog"))
        .order(egui::Order::Foreground)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_max_width(430.0);
            ui.label(format!(
                "Every executable library loaded from “{pack_name}” will be removed from this project. The installed pack remains on disk and can still be browsed."
            ));
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if Button::new("Cancel").show(ui).clicked() {
                    cancel = true;
                }
                if Button::new("Detach pack")
                    .accent()
                    .destructive(true)
                    .show(ui)
                    .clicked()
                {
                    confirm = true;
                }
            });
        });
    if cancel {
        app.state.workbench.model_catalog_detach_pack = None;
    } else if confirm {
        let mut candidate = app.state.model_library_manager.clone();
        let result = candidate.detach_pack(&pack_id).and_then(|count| {
            app.publish_model_library_candidate(candidate)
                .map(|()| count)
        });
        match result {
            Ok(count) => {
                app.state.workbench.model_catalog_detach_pack = None;
                app.state.push_user_message(ConsoleMessage::info(format!(
                    "Detached {count} executable librar{} from installed pack '{}'.",
                    if count == 1 { "y" } else { "ies" },
                    pack_name
                )));
            }
            Err(error) => {
                app.state.workbench.model_catalog_detach_pack = None;
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Could not detach installed pack '{}': {error}",
                    pack_name
                )));
            }
        }
    }
}

fn part_catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    let counts_state = cached_device_counts(ui, app);
    let (counts, counts_pending, counts_error) = match &counts_state {
        CatalogTaskState::Ready(Ok(counts)) => (counts.clone(), false, None),
        CatalogTaskState::Pending => (BTreeMap::new(), true, None),
        CatalogTaskState::Ready(Err(error)) => (BTreeMap::new(), false, Some(error.clone())),
    };
    let facet_counts = ModelLibraryFacet::ALL.map(|facet| library_facet_count(facet, &counts));
    let previous_filter = part_filter_identity(app);
    part_toolbar(ui, app, facet_counts);
    if part_filter_identity(app) != previous_filter {
        app.state.workbench.model_catalog_part_page = 0;
        app.state.workbench.model_catalog_selected_part = None;
    }
    ui.add_space(5.0);

    let facet = app.state.workbench.model_library_facet;
    let devices = library_facet_devices(facet, &counts);
    let pack_filter = app.state.workbench.model_catalog_selected_pack.clone();
    let query = app.state.model_library_manager.filter_text.clone();
    let requested_page = app.state.workbench.model_catalog_part_page;
    let offset = requested_page.saturating_mul(CATALOG_PAGE_LIMIT);
    let page = if let Some(error) = counts_error {
        CatalogTaskState::Ready(Err(error))
    } else if counts_pending && facet != ModelLibraryFacet::All {
        CatalogTaskState::Pending
    } else {
        cached_part_query(ui, app, &query, pack_filter.as_deref(), &devices, offset)
    };
    let (hits, total_matches, query_error, query_pending) = match &page {
        CatalogTaskState::Pending => (Vec::new(), 0, None, true),
        CatalogTaskState::Ready(Ok(page)) => (page.hits.clone(), page.total_matches, None, false),
        CatalogTaskState::Ready(Err(error)) => (Vec::new(), 0, Some(error.as_str()), false),
    };
    let t = Tokens::get(ui.ctx());
    let rows = hits
        .iter()
        .map(|hit| {
            let key = pack_model_key(hit);
            let source = hit.source.as_deref().and_then(Path::file_name).map_or_else(
                || "not resolved".to_owned(),
                |name| name.to_string_lossy().into(),
            );
            DataRow {
                key: key.clone(),
                selected: app.state.workbench.model_catalog_selected_part.as_deref()
                    == Some(key.as_str()),
                cells: vec![
                    DataCell::mono(&hit.name),
                    DataCell::plain(&hit.kind),
                    DataCell::plain(&hit.device),
                    DataCell::plain(&hit.pack_name),
                    DataCell::mono(format!("{source}:{}", hit.line)),
                    DataCell::mono_colored(
                        if hit.redistributable {
                            "redistributable"
                        } else {
                            "browse only"
                        },
                        if hit.redistributable {
                            t.color.ok
                        } else {
                            t.color.warn
                        },
                    ),
                ],
            }
        })
        .collect::<Vec<_>>();
    let selected = app
        .state
        .workbench
        .model_catalog_selected_part
        .as_deref()
        .and_then(|key| hits.iter().find(|hit| pack_model_key(hit) == key))
        .cloned();
    let columns = [
        ("Part", 0.21),
        ("Kind", 0.09),
        ("Device class", 0.16),
        ("Pack", 0.22),
        ("Source", 0.18),
        ("Rights", 0.14),
    ];
    let empty = match (query_pending, query_error) {
        (true, _) => "Indexing and querying installed model parts…",
        (_, Some(error)) => error,
        _ => "No indexed parts match the active class, pack, and search.",
    };
    let available_height = ui.available_height().max(1.0);
    let browser_height = (available_height - 38.0).max(80.0).min(available_height);
    let event = ui
        .allocate_ui_with_layout(
            egui::vec2(ui.available_width(), browser_height),
            Layout::top_down(Align::Min),
            |ui| {
                catalog_browser(ui, "models.catalog.parts", &columns, &rows, empty, |ui| {
                    part_detail(ui, app, selected.as_ref())
                })
            },
        )
        .inner;
    if let Some(event) = event
        && hits.iter().any(|hit| pack_model_key(hit) == event.key)
    {
        app.state.workbench.model_catalog_selected_part = Some(event.key);
    }
    part_pager(
        ui,
        app,
        requested_page,
        total_matches,
        hits.len(),
        query_pending,
        query_error.is_some(),
        &t,
    );
}

fn part_toolbar(ui: &mut Ui, app: &mut RSpiceApp, counts: [usize; 7]) {
    if let Some(pack) = app.state.workbench.model_catalog_selected_pack.clone() {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new(format!("Pack filter: {pack}"))
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium)),
            );
            if Button::new("Clear pack").ghost().show(ui).clicked() {
                app.state.workbench.model_catalog_selected_pack = None;
                app.state.workbench.model_catalog_selected_part = None;
                app.state.workbench.model_catalog_part_page = 0;
            }
        });
        ui.add_space(4.0);
    }
    let width = ui.available_width();
    let current = app.state.workbench.model_library_facet;
    let mut requested = None;
    if width < CATALOG_TOOLBAR_BREAKPOINT {
        facet_strip(
            ui,
            "models.catalog.library.facets",
            ModelLibraryFacet::ALL
                .into_iter()
                .zip(counts)
                .map(|(facet, count)| (facet.label(), count, facet == current, facet)),
            |facet| requested = Some(facet),
        );
        ui.add_space(4.0);
        let total = app
            .state
            .model_library_manager
            .spice_packs()
            .map_or(0, |index| index.part_count());
        catalog_search(
            ui,
            &mut app.state.model_library_manager.filter_text,
            &format!(
                "Search {} parts by name, class, pack, or source…",
                grouped_count(total)
            ),
        );
    } else {
        ui.horizontal(|ui| {
            ui.set_width(width);
            ui.allocate_ui_with_layout(
                egui::vec2((width - 320.0).max(1.0), ui.available_height()),
                Layout::left_to_right(Align::Center),
                |ui| {
                    facet_strip(
                        ui,
                        "models.catalog.library.facets",
                        ModelLibraryFacet::ALL
                            .into_iter()
                            .zip(counts)
                            .map(|(facet, count)| (facet.label(), count, facet == current, facet)),
                        |facet| requested = Some(facet),
                    );
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(312.0, ui.available_height()),
                Layout::right_to_left(Align::Center),
                |ui| {
                    catalog_search(
                        ui,
                        &mut app.state.model_library_manager.filter_text,
                        "Search parts, classes, packs, or sources…",
                    );
                },
            );
        });
    }
    if let Some(facet) = requested {
        app.state.workbench.model_library_facet = facet;
        app.state.workbench.model_catalog_selected_part = None;
        app.state.workbench.model_catalog_part_page = 0;
    }
}

fn part_filter_identity(app: &RSpiceApp) -> (ModelLibraryFacet, Option<String>, String) {
    (
        app.state.workbench.model_library_facet,
        app.state.workbench.model_catalog_selected_pack.clone(),
        app.state.model_library_manager.filter_text.clone(),
    )
}

fn library_facet_devices(
    facet: ModelLibraryFacet,
    counts: &BTreeMap<String, usize>,
) -> Vec<String> {
    if facet == ModelLibraryFacet::All {
        return Vec::new();
    }
    counts
        .keys()
        .filter(|device| library_device_matches(facet, device))
        .cloned()
        .collect()
}

fn library_facet_count(facet: ModelLibraryFacet, counts: &BTreeMap<String, usize>) -> usize {
    counts
        .iter()
        .filter(|(device, _)| library_device_matches(facet, device))
        .map(|(_, count)| *count)
        .sum()
}

fn library_device_matches(facet: ModelLibraryFacet, device: &str) -> bool {
    if facet == ModelLibraryFacet::All {
        return true;
    }
    let device = device.to_ascii_lowercase();
    match facet {
        ModelLibraryFacet::All => true,
        ModelLibraryFacet::Mosfet => {
            device.contains("mos")
                || device.contains("soi")
                || matches!(device.as_str(), "nfet" | "pfet" | "finfet")
                || device.starts_with("psp")
        }
        ModelLibraryFacet::Bipolar => device.starts_with("bjt") || device.contains("igbt"),
        ModelLibraryFacet::Diode => device.contains("diode") || device.contains("zener"),
        ModelLibraryFacet::Jfet => {
            device.contains("jfet")
                || device.contains("mesfet")
                || device.contains("gasfet")
                || device.contains("hemt")
        }
        ModelLibraryFacet::Passive => {
            matches!(
                device.as_str(),
                "resistor"
                    | "capacitor"
                    | "inductor"
                    | "lcouple"
                    | "magnetic-core"
                    | "transmission-line"
            )
        }
        ModelLibraryFacet::Ic => {
            device == "subckt"
                || device.contains("digital")
                || device.contains("bridge")
                || device.contains("logic")
                || device.contains("opamp")
                || device.contains("comparator")
                || device.contains("regulator")
                || device.contains("optocoupler")
                || device.contains("timer")
                || device.contains("delay")
                || device.contains("gain")
        }
    }
}

fn part_pager(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    requested_page: usize,
    total_matches: usize,
    visible_matches: usize,
    pending: bool,
    failed: bool,
    t: &Tokens,
) {
    let page_count = total_matches.div_ceil(CATALOG_PAGE_LIMIT);
    let last_page = page_count.saturating_sub(1);
    if !pending && !failed && requested_page > last_page {
        app.state.workbench.model_catalog_part_page = last_page;
        app.state.workbench.model_catalog_selected_part = None;
        ui.ctx().request_repaint();
    }
    let current_page = requested_page.min(last_page);
    let start = current_page
        .saturating_mul(CATALOG_PAGE_LIMIT)
        .saturating_add((total_matches > 0) as usize);
    let end = current_page
        .saturating_mul(CATALOG_PAGE_LIMIT)
        .saturating_add(visible_matches)
        .min(total_matches);
    ui.horizontal(|ui| {
        let previous = Button::new("Previous")
            .enabled(!pending && !failed && current_page > 0)
            .show(ui);
        if previous.clicked() {
            app.state.workbench.model_catalog_part_page = current_page.saturating_sub(1);
            app.state.workbench.model_catalog_selected_part = None;
        }
        let next = Button::new("Next")
            .enabled(!pending && !failed && current_page + 1 < page_count)
            .show(ui);
        if next.clicked() {
            app.state.workbench.model_catalog_part_page = current_page.saturating_add(1);
            app.state.workbench.model_catalog_selected_part = None;
        }
        ui.separator();
        let status = if pending {
            "Indexing installed catalog…".to_owned()
        } else if failed {
            "Catalog query failed".to_owned()
        } else if total_matches == 0 {
            "0 exact matches".to_owned()
        } else {
            format!(
                "{}–{} of {} exact matches · page {} of {}",
                grouped_count(start),
                grouped_count(end),
                grouped_count(total_matches),
                grouped_count(current_page + 1),
                grouped_count(page_count)
            )
        };
        ui.label(
            egui::RichText::new(status)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if failed {
                    t.color.err
                } else {
                    t.color.text_faint
                }),
        );
    });
}

fn cached_device_counts(ui: &Ui, app: &RSpiceApp) -> CatalogTaskState<BTreeMap<String, usize>> {
    let root = app
        .state
        .model_library_manager
        .spice_packs()
        .map_or_else(String::new, |index| index.root().display().to_string());
    let cache_id = ui.make_persistent_id("models.catalog.device-counts");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<CachedDeviceCounts>(cache_id))
        && cached.root == root
    {
        return poll_catalog_task(ui, &cached.outcome);
    }

    let outcome = Arc::new(Mutex::new(None));
    let cached = CachedDeviceCounts {
        root,
        outcome: Arc::clone(&outcome),
    };
    ui.ctx()
        .data_mut(|data| data.insert_temp(cache_id, cached.clone()));
    let Some(index) = app.state.model_library_manager.shared_spice_packs() else {
        set_catalog_task_result(
            &outcome,
            Err("The shipped model-pack index is unavailable on this installation".to_owned()),
        );
        return poll_catalog_task(ui, &outcome);
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        let repaint = ui.ctx().clone();
        let worker_outcome = Arc::clone(&outcome);
        if let Err(error) = std::thread::Builder::new()
            .name("rspice-model-catalog-counts".to_owned())
            .spawn(move || {
                let result = index
                    .part_device_counts()
                    .map_err(|error| format!("Could not count shipped part classes: {error}"));
                set_catalog_task_result(&worker_outcome, result);
                repaint.request_repaint();
            })
        {
            set_catalog_task_result(
                &outcome,
                Err(format!("Could not start catalog index worker: {error}")),
            );
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let result = index
            .part_device_counts()
            .map_err(|error| format!("Could not count shipped part classes: {error}"));
        set_catalog_task_result(&outcome, result);
    }
    poll_catalog_task(ui, &outcome)
}

fn cached_part_query(
    ui: &Ui,
    app: &RSpiceApp,
    query: &str,
    pack: Option<&str>,
    devices: &[String],
    offset: usize,
) -> CatalogTaskState<PackCatalogPage> {
    let root = app
        .state
        .model_library_manager
        .spice_packs()
        .map_or_else(String::new, |index| index.root().display().to_string());
    let signature = format!(
        "{root}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{offset}",
        pack.unwrap_or(""),
        devices.join("\u{1e}"),
        query
    );
    let cache_id = ui.make_persistent_id("models.catalog.part-query");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<CachedPartQuery>(cache_id))
        && cached.signature == signature
    {
        return poll_catalog_task(ui, &cached.outcome);
    }

    if let Some(previous) = ui
        .ctx()
        .data(|data| data.get_temp::<CachedPartQuery>(cache_id))
    {
        previous.cancel.store(true, Ordering::Release);
    }
    let cancel = Arc::new(AtomicBool::new(false));
    let outcome = Arc::new(Mutex::new(None));
    let cached = CachedPartQuery {
        signature,
        cancel: Arc::clone(&cancel),
        outcome: Arc::clone(&outcome),
    };
    ui.ctx().data_mut(|data| {
        data.insert_temp(cache_id, cached);
    });
    let Some(index) = app.state.model_library_manager.shared_spice_packs() else {
        set_catalog_task_result(
            &outcome,
            Err("The shipped model-pack index is unavailable on this installation".to_owned()),
        );
        return poll_catalog_task(ui, &outcome);
    };
    let query = query.to_owned();
    let pack = pack.map(str::to_owned);
    let devices = devices.to_vec();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let repaint = ui.ctx().clone();
        let worker_outcome = Arc::clone(&outcome);
        let worker_cancel = Arc::clone(&cancel);
        if let Err(error) = std::thread::Builder::new()
            .name("rspice-model-catalog-query".to_owned())
            .spawn(move || {
                let device_refs = devices.iter().map(String::as_str).collect::<Vec<_>>();
                let result = ModelLibraryManager::query_pack_parts_from_index(
                    &index,
                    &query,
                    pack.as_deref(),
                    &device_refs,
                    offset,
                    CATALOG_PAGE_LIMIT,
                    || worker_cancel.load(Ordering::Acquire),
                );
                if !worker_cancel.load(Ordering::Acquire) {
                    set_catalog_task_result(&worker_outcome, result);
                    repaint.request_repaint();
                }
            })
        {
            set_catalog_task_result(
                &outcome,
                Err(format!("Could not start catalog query worker: {error}")),
            );
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let device_refs = devices.iter().map(String::as_str).collect::<Vec<_>>();
        let result = ModelLibraryManager::query_pack_parts_from_index(
            &index,
            &query,
            pack.as_deref(),
            &device_refs,
            offset,
            CATALOG_PAGE_LIMIT,
            || cancel.load(Ordering::Acquire),
        );
        set_catalog_task_result(&outcome, result);
    }
    poll_catalog_task(ui, &outcome)
}

fn poll_catalog_task<T: Clone>(
    ui: &Ui,
    outcome: &Arc<Mutex<Option<Result<T, String>>>>,
) -> CatalogTaskState<T> {
    match outcome.lock() {
        Ok(result) => match result.as_ref() {
            Some(result) => CatalogTaskState::Ready(result.clone()),
            None => {
                ui.ctx().request_repaint_after(Duration::from_millis(40));
                CatalogTaskState::Pending
            }
        },
        Err(_) => {
            CatalogTaskState::Ready(Err("The catalog worker result lock was poisoned".to_owned()))
        }
    }
}

fn set_catalog_task_result<T>(
    outcome: &Arc<Mutex<Option<Result<T, String>>>>,
    result: Result<T, String>,
) {
    if let Ok(mut outcome) = outcome.lock() {
        *outcome = Some(result);
    }
}

fn part_detail(ui: &mut Ui, app: &mut RSpiceApp, selected: Option<&PackModelHit>) {
    let Some(hit) = selected else {
        catalog_empty_detail(
            ui,
            "Select an indexed part",
            "RSpice will revalidate its exact pack, path, line, and definition before showing source or attaching anything to the project.",
        );
        return;
    };
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(&hit.name)
            .font(theme::sans(15.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(format!("{} · {} · {}", hit.device, hit.kind, hit.pack_name))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(6.0);
    ui.horizontal_wrapped(|ui| {
        let sources_available = installed_pack_sources_available(app);
        if hit.kind.eq_ignore_ascii_case("model") {
            if Button::new("Add model to project")
                .accent()
                .enabled(
                    sources_available && hit.redistributable && project_catalog_writable(app),
                )
                .show(ui)
                .on_disabled_hover_text(if !sources_available {
                    browser_pack_source_block_reason()
                } else if hit.redistributable {
                    project_write_block_reason(app)
                } else {
                    "This definition is browse-only because redistribution authority is not established."
                })
                .clicked()
            {
                activate_pack_model(app, hit);
            }
        } else if Button::new("Subcircuit interface…")
            .enabled(sources_available && hit.redistributable && project_catalog_writable(app))
            .show(ui)
            .on_disabled_hover_text(if !sources_available {
                browser_pack_source_block_reason()
            } else if hit.redistributable {
                project_write_block_reason(app)
            } else {
                "This subcircuit is browse-only because redistribution authority is not established."
            })
            .clicked()
        {
            activate_pack_subcircuit_interface(app, hit);
        }
        if Button::new("Open pack").show(ui).clicked() {
            app.state.workbench.model_catalog_scope = ModelCatalogScope::Packs;
            app.state.workbench.model_catalog_selected_pack = Some(hit.pack.clone());
        }
    });
    ui.add_space(8.0);
    property_card(ui, "Indexed identity", |ui| {
        property_row(ui, "Definition", &hit.name);
        property_row(ui, "Directive", &format!(".{}", hit.kind));
        property_row(ui, "Device class", &hit.device);
        property_row(ui, "Pack", &hit.pack_name);
        property_row(
            ui,
            "Source",
            &hit.source.as_deref().map_or_else(
                || "not resolved".to_owned(),
                |path| path.display().to_string(),
            ),
        );
        property_row(ui, "Source line", &hit.line.to_string());
        property_row_toned(
            ui,
            "Redistribution",
            if hit.redistributable {
                "established"
            } else {
                "not established · browse only"
            },
            if hit.redistributable {
                t.color.ok
            } else {
                t.color.warn
            },
        );
    });
    ui.add_space(7.0);
    property_card(ui, "Qualification", |ui| {
        property_row_toned(
            ui,
            "RSpice evidence",
            "none · engineering preview",
            t.color.warn,
        );
        property_row(ui, "Release gate", "excluded until qualified");
        ui.add_space(5.0);
        ui.horizontal_wrapped(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(
                    "Pack metadata and upstream parameters are not RSpice qualification evidence. After activation, author executable vectors and approve their exact source revision in Qualification.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        });
    });
    ui.add_space(7.0);
    if installed_pack_sources_available(app) {
        match cached_part_preview(ui, app, hit) {
            Ok(preview) => source_preview_card(
                ui,
                "Revalidated definition preview",
                preview.start_line,
                &preview.source,
                preview.truncated,
            ),
            Err(error) => property_card(ui, "Definition preview", |ui| {
                property_row_toned(ui, "Status", &error, t.color.err);
            }),
        }
    } else {
        property_card(ui, "Definition preview", |ui| {
            property_row_toned(
                ui,
                "Status",
                "Metadata index available · source bytes not installed in this browser",
                t.color.warn,
            );
            property_row(
                ui,
                "Activation",
                "Import the exact model source bundle through PDK settings",
            );
        });
    }
}

fn cached_part_preview(
    ui: &Ui,
    app: &RSpiceApp,
    hit: &PackModelHit,
) -> Result<CatalogDefinitionPreview, String> {
    let key = pack_model_key(hit);
    let cache_id = ui.make_persistent_id("models.catalog.part-preview");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<CachedPartPreview>(cache_id))
        && cached.key == key
    {
        return cached.result;
    }
    let result = app.state.model_library_manager.preview_pack_part(hit);
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            cache_id,
            CachedPartPreview {
                key,
                result: result.clone(),
            },
        );
    });
    result
}

fn activate_pack_model(app: &mut RSpiceApp, hit: &PackModelHit) {
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate.activate_pack_model(hit).and_then(|library| {
        app.publish_model_library_candidate(candidate)
            .map(|()| library)
    });
    match result {
        Ok(library) => {
            app.state.model_library_manager.select_library(&library);
            app.state.workbench.selected_model = Some(hit.name.clone());
            app.state.push_user_message(ConsoleMessage::info(format!(
                "Added shipped model '{}' from '{}' as executable project library '{}'. It remains an engineering preview until qualification evidence is approved.",
                hit.name, hit.pack_name, library
            )));
        }
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Could not add shipped model '{}': {error}",
            hit.name
        ))),
    }
}

fn activate_pack_subcircuit_interface(app: &mut RSpiceApp, hit: &PackModelHit) {
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate
        .activate_pack_subcircuit(hit)
        .and_then(|activated| {
            app.publish_model_library_candidate(candidate)?;
            open_create_subcircuit_bound_symbol_dialog(
                &mut app.state,
                activated.library.clone(),
                activated.name.clone(),
                activated.source_path.clone(),
                activated.ports.clone(),
                None,
                BTreeMap::new(),
            )?;
            Ok(activated)
        });
    match result {
        Ok(activated) => app.state.push_user_message(ConsoleMessage::info(format!(
            "Attached shipped subcircuit '{}' from executable library '{}' and opened its ordered terminal contract for symbol review.",
            activated.name, activated.library
        ))),
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Could not prepare shipped subcircuit '{}': {error}",
            hit.name
        ))),
    }
}

fn source_preview_card(ui: &mut Ui, title: &str, start_line: usize, source: &str, truncated: bool) {
    let t = Tokens::get(ui.ctx());
    property_card(ui, title, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(if truncated {
                    "bounded preview · truncated"
                } else {
                    "complete indexed definition"
                })
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if truncated {
                    t.color.warn
                } else {
                    t.color.text_dim
                }),
            );
            if Button::new("Copy").ghost().show(ui).clicked() {
                ui.ctx().copy_text(source.to_owned());
            }
        });
        egui::Frame::new()
            .fill(t.color.bg_inset)
            .stroke(Stroke::new(1.0, t.color.border))
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ScrollArea::both()
                    .id_salt(("models.catalog.preview", title))
                    .max_height(260.0)
                    .show(ui, |ui| {
                        for (offset, line) in source.lines().enumerate() {
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:>6}  {}",
                                    start_line + offset,
                                    line
                                ))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text),
                            );
                        }
                    });
            });
    });
}

fn catalog_browser(
    ui: &mut Ui,
    salt: &'static str,
    columns: &[(&str, f32)],
    rows: &[DataRow],
    empty_message: &str,
    detail: impl FnOnce(&mut Ui),
) -> Option<TableEvent> {
    let available = ui.available_size().max(Vec2::splat(1.0));
    let (viewport, _) = ui.allocate_exact_size(available, Sense::hover());
    let wide = viewport.width() >= CATALOG_WIDE_BREAKPOINT;
    if wide {
        let gap = 7.0;
        let table_width = (viewport.width() * 0.57).max(480.0);
        let table_rect = Rect::from_min_size(
            viewport.min,
            egui::vec2((table_width - gap).min(viewport.width()), viewport.height()),
        );
        let detail_rect = Rect::from_min_max(
            egui::pos2(table_rect.right() + gap, viewport.top()),
            viewport.right_bottom(),
        );
        let mut table_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(table_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        let event = data_table(
            &mut table_ui,
            salt,
            model_catalog_min_width(table_rect.width()),
            columns,
            rows,
            table_rect.size(),
            empty_message,
        );
        let mut detail_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(detail_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        ScrollArea::vertical()
            .id_salt((salt, "detail"))
            .auto_shrink([false, false])
            .show(&mut detail_ui, |ui| {
                ui.set_min_width(detail_rect.width());
                detail(ui);
            });
        event
    } else {
        let table_height = (viewport.height() * 0.48)
            .clamp(150.0, 340.0)
            .min((viewport.height() - 130.0).max(80.0));
        let table_rect =
            Rect::from_min_size(viewport.min, egui::vec2(viewport.width(), table_height));
        let detail_rect = Rect::from_min_max(
            egui::pos2(viewport.left(), table_rect.bottom() + 7.0),
            viewport.right_bottom(),
        );
        let mut table_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(table_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        let event = data_table(
            &mut table_ui,
            salt,
            model_catalog_min_width(table_rect.width()),
            columns,
            rows,
            table_rect.size(),
            empty_message,
        );
        let mut detail_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(detail_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        ScrollArea::vertical()
            .id_salt((salt, "detail"))
            .auto_shrink([false, false])
            .show(&mut detail_ui, |ui| {
                ui.set_min_width(detail_rect.width());
                detail(ui);
            });
        event
    }
}

fn facet_strip<T: Copy>(
    ui: &mut Ui,
    salt: &'static str,
    facets: impl IntoIterator<Item = (&'static str, usize, bool, T)>,
    mut select: impl FnMut(T),
) {
    let t = Tokens::get(ui.ctx());
    ScrollArea::horizontal()
        .id_salt(salt)
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                for (label, count, selected, value) in facets {
                    let text = format!("{label}  {}", grouped_count(count));
                    let response = ui.add(
                        egui::Button::new(
                            egui::RichText::new(text)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular)),
                        )
                        .selected(selected)
                        .min_size(egui::vec2(0.0, t.metrics.ctl_h)),
                    );
                    if response.clicked() {
                        select(value);
                    }
                }
            });
        });
}

fn catalog_search(ui: &mut Ui, value: &mut String, hint: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_sized(
        egui::vec2(ui.available_width().max(120.0), t.metrics.ctl_h),
        egui::TextEdit::singleline(value)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .hint_text(hint),
    );
}

fn catalog_empty_detail(ui: &mut Ui, title: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(18.0);
    ui.label(
        egui::RichText::new(title)
            .font(theme::sans(15.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(body)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn project_catalog_writable(app: &RSpiceApp) -> bool {
    app.state.project_lifecycle.project_open && !app.state.workbench.safe_mode.project_read_only()
}

fn installed_pack_sources_available(app: &RSpiceApp) -> bool {
    app.state
        .model_library_manager
        .spice_packs()
        .is_some_and(SpiceLibraryIndex::source_files_available)
}

fn browser_pack_source_block_reason() -> &'static str {
    "This browser includes the searchable pack metadata index, not the source corpus. Import the exact source bundle through PDK settings before activation."
}

fn project_write_block_reason(app: &RSpiceApp) -> &'static str {
    if !app.state.project_lifecycle.project_open {
        "Open a project before changing its executable model sources."
    } else {
        "The open project is read-only."
    }
}

fn pack_model_key(hit: &PackModelHit) -> String {
    let source = hit
        .source
        .as_ref()
        .map_or_else(String::new, |path| path.to_string_lossy().into_owned());
    format!(
        "pack\u{1f}{}\u{1f}{}\u{1f}{source}\u{1f}{}",
        hit.pack, hit.name, hit.line
    )
}

fn grouped_count(value: usize) -> String {
    let source = value.to_string();
    let mut result = String::with_capacity(source.len() + source.len() / 3);
    for (index, character) in source.chars().enumerate() {
        if index > 0 && (source.len() - index).is_multiple_of(3) {
            result.push(',');
        }
        result.push(character);
    }
    result
}

fn format_storage(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    let bytes = bytes as f64;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes / GIB)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes / MIB)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes / KIB)
    } else {
        format!("{} B", bytes as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_scopes_and_facets_match_the_reference_taxonomy() {
        assert_eq!(
            ModelCatalogScope::ALL.map(ModelCatalogScope::label),
            ["Project", "Installed packs", "RSpice library"]
        );
        assert_eq!(
            ModelPackFacet::ALL.map(ModelPackFacet::label),
            [
                "All",
                "Needs attention",
                "Attached",
                "Foundry",
                "Vendor",
                "Community",
                "Redistributable",
            ]
        );
        assert_eq!(
            ModelLibraryFacet::ALL.map(ModelLibraryFacet::label),
            [
                "All classes",
                "MOSFET",
                "Bipolar",
                "Diode",
                "JFET & HEMT",
                "Passive",
                "IC & macro",
            ]
        );
    }

    #[test]
    fn library_facets_are_exact_and_do_not_double_count_bipolar_as_mosfet() {
        let counts = BTreeMap::from([
            ("mosfet-n".to_owned(), 10),
            ("bjt-npn".to_owned(), 20),
            ("diode".to_owned(), 30),
            ("jfet-n".to_owned(), 40),
            ("resistor".to_owned(), 50),
            ("subckt".to_owned(), 60),
        ]);
        assert_eq!(library_facet_count(ModelLibraryFacet::All, &counts), 210);
        assert_eq!(library_facet_count(ModelLibraryFacet::Mosfet, &counts), 10);
        assert_eq!(library_facet_count(ModelLibraryFacet::Bipolar, &counts), 20);
        assert_eq!(library_facet_count(ModelLibraryFacet::Diode, &counts), 30);
        assert_eq!(library_facet_count(ModelLibraryFacet::Jfet, &counts), 40);
        assert_eq!(library_facet_count(ModelLibraryFacet::Passive, &counts), 50);
        assert_eq!(library_facet_count(ModelLibraryFacet::Ic, &counts), 60);
    }

    #[test]
    fn grouped_counts_and_storage_are_stable() {
        assert_eq!(grouped_count(0), "0");
        assert_eq!(grouped_count(999), "999");
        assert_eq!(grouped_count(1_000), "1,000");
        assert_eq!(grouped_count(199_934), "199,934");
        assert_eq!(format_storage(1_048_576), "1.0 MiB");
    }

    #[test]
    fn catalog_scope_controls_remain_accessible_on_phone_and_desktop_widths() {
        for width in [430.0, 1_240.0] {
            let mut app = RSpiceApp::test_instance();
            app.state.workbench.models_page = ModelsPage::Models;
            let ctx = egui::Context::default();
            ctx.enable_accesskit();
            crate::ui::Theme::default().apply(&ctx);
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 620.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE)
                        .show(ctx, |ui| models_catalog(ui, &mut app));
                },
            );
            let root = output
                .platform_output
                .accesskit_update
                .expect("catalog access tree");
            for label in ModelCatalogScope::ALL.map(ModelCatalogScope::label) {
                assert!(
                    root.nodes.iter().any(|(_, node)| {
                        node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
                    }),
                    "missing accessible scope '{label}' at width {width}"
                );
            }
            assert!(!output.shapes.is_empty());
        }
    }

    #[test]
    fn part_pager_is_bounded_and_accessible_on_phone_and_desktop_widths() {
        for width in [430.0, 1_240.0] {
            let mut app = RSpiceApp::test_instance();
            app.state.workbench.model_catalog_part_page = 1;
            let ctx = egui::Context::default();
            ctx.enable_accesskit();
            crate::ui::Theme::default().apply(&ctx);
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 80.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE)
                        .show(ctx, |ui| {
                            let tokens = Tokens::get(ui.ctx());
                            part_pager(ui, &mut app, 1, 450, 200, false, false, &tokens);
                        });
                },
            );
            let root = output
                .platform_output
                .accesskit_update
                .expect("pager access tree");
            for label in ["Previous", "Next"] {
                assert!(
                    root.nodes.iter().any(|(_, node)| {
                        node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
                    }),
                    "missing accessible pager control '{label}' at width {width}"
                );
            }
            assert_eq!(app.state.workbench.model_catalog_part_page, 1);
            assert!(!output.shapes.is_empty());
        }

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.model_catalog_part_page = usize::MAX;
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let tokens = Tokens::get(ui.ctx());
                part_pager(ui, &mut app, usize::MAX, 450, 0, false, false, &tokens);
            });
        });
        assert_eq!(app.state.workbench.model_catalog_part_page, 2);
    }

    #[test]
    fn explicit_component_model_usage_never_treats_passive_value_as_a_model() {
        let mut mos = Component::new(1, ComponentType::Nmos, crate::state::Point::origin())
            .with_name_value("M1", "nch_core");
        assert_eq!(
            explicit_component_model_name(&mos).as_deref(),
            Some("nch_core")
        );
        mos.params = "model=nch_override l=1u".to_owned();
        assert_eq!(
            explicit_component_model_name(&mos).as_deref(),
            Some("nch_override")
        );

        let resistor = Component::new(2, ComponentType::Resistor, crate::state::Point::origin())
            .with_name_value("R1", "10k");
        assert_eq!(explicit_component_model_name(&resistor), None);
    }

    fn binding_record(library: &str, model: DeviceModel) -> ProjectCatalogRecord {
        let definition = ProjectCatalogDefinition::Model(model);
        ProjectCatalogRecord {
            key: project_catalog_key(library, &definition),
            library: library.to_owned(),
            definition,
            usage: Vec::new(),
            qualification: None,
            status: ProjectCatalogStatus::Ready,
            pinned: false,
        }
    }

    fn install_binding_library(app: &mut RSpiceApp, library_name: &str, model: DeviceModel) {
        let mut library = ModelLibrary::new(library_name);
        library.source_authority = ModelSourceAuthority::External;
        library.add_model(model);
        app.state.model_library_manager.add_library(library);
    }

    #[test]
    fn catalog_binding_is_typed_provenanced_and_undoable() {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.clear();
        let mut model = DeviceModel::new("nch_svt", ModelType::Nmos);
        model.spice_type = Some("NMOS".to_owned());
        install_binding_library(&mut app, "foundry", model.clone());
        let selected = binding_record("foundry", model);
        let mut component = Component::new(7, ComponentType::Nmos, crate::state::Point::origin())
            .with_name_value("M7", "old_model");
        component.params = "w=2u l=1u model=override model_corner=ff".to_owned();
        app.state.schematic.components.push(component);
        app.state.schematic.selection.select_only_component(7);

        let message = bind_project_model_to_selected_component(&mut app, &selected)
            .expect("compatible authenticated model binds");

        assert_eq!(message, "Bound M7 to foundry/nch_svt.");
        let component = app
            .state
            .schematic
            .components
            .iter()
            .find(|component| component.id == 7)
            .expect("bound component");
        assert_eq!(component.value, "nch_svt");
        let params = crate::state::parse_params_string(&component.params);
        assert_eq!(params.get("l").map(String::as_str), Some("1u"));
        assert_eq!(params.get("w").map(String::as_str), Some("2u"));
        assert_eq!(
            params.get("model_library").map(String::as_str),
            Some("foundry")
        );
        assert!(!params.contains_key("model"));
        assert!(!params.contains_key("model_corner"));
        assert!(app.state.schematic.is_dirty);
        assert!(app.state.schematic.can_undo());
        assert!(app.state.schematic.undo());
        let restored = app
            .state
            .schematic
            .components
            .iter()
            .find(|component| component.id == 7)
            .expect("restored component");
        assert_eq!(restored.value, "old_model");
        assert_eq!(restored.params, "w=2u l=1u model=override model_corner=ff");
    }

    #[test]
    fn catalog_binding_rejects_incompatible_devices_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.clear();
        let mut model = DeviceModel::new("pch_svt", ModelType::Pmos);
        model.spice_type = Some("PMOS".to_owned());
        install_binding_library(&mut app, "foundry", model.clone());
        let selected = binding_record("foundry", model);
        let component = Component::new(9, ComponentType::Nmos, crate::state::Point::origin())
            .with_name_value("M9", "nch_old");
        app.state.schematic.components.push(component.clone());
        app.state.schematic.selection.select_only_component(9);

        let error = bind_project_model_to_selected_component(&mut app, &selected)
            .expect_err("PMOS card cannot bind to NMOS");

        assert!(error.contains("cannot use"));
        assert_eq!(app.state.schematic.components, vec![component]);
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn contested_catalog_binding_requires_the_exact_provider_resolution() {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.clear();
        let mut model = DeviceModel::new("shared_nch", ModelType::Nmos);
        model.spice_type = Some("NMOS".to_owned());
        install_binding_library(&mut app, "provider_a", model.clone());
        install_binding_library(&mut app, "provider_b", model.clone());
        let selected = binding_record("provider_a", model);
        app.state.schematic.components.push(
            Component::new(11, ComponentType::Nmos, crate::state::Point::origin())
                .with_name_value("M11", "old"),
        );
        app.state.schematic.selection.select_only_component(11);

        let error = project_model_bind_target(&app, &selected)
            .expect_err("unresolved provider must fail closed");
        assert!(error.contains("Resolve 'shared_nch'"));
        app.state
            .model_library_manager
            .resolve_definition_conflict("shared_nch", "provider_a", "shared_nch")
            .expect("exact provider resolution");
        assert!(project_model_bind_target(&app, &selected).is_ok());
    }

    #[test]
    fn where_used_attributes_contested_names_only_to_the_executable_provider() {
        let mut manager = ModelLibraryManager::new();
        let mut model = DeviceModel::new("shared_nch", ModelType::Nmos);
        model.spice_type = Some("NMOS".to_owned());
        let mut provider_a = ModelLibrary::new("provider_a");
        provider_a.add_model(model.clone());
        manager.add_library(provider_a);
        let mut provider_b = ModelLibrary::new("provider_b");
        provider_b.add_model(model);
        manager.add_library(provider_b);
        manager
            .resolve_definition_conflict("shared_nch", "provider_a", "shared_nch")
            .expect("exact provider resolution");
        let mut schematic = SchematicState::default();
        let mut component = Component::new(13, ComponentType::Nmos, crate::state::Point::origin())
            .with_name_value("M13", "shared_nch");
        component.params = "model_library=provider_a".to_owned();
        schematic.components.push(component);
        let mut usage = HashMap::new();

        collect_schematic_model_usage(&schematic, "work/top", &manager, &mut usage);

        assert_eq!(
            usage.get(&project_model_usage_key("provider_a", "shared_nch")),
            Some(&vec!["work/top/M13".to_owned()])
        );
        assert!(
            !usage.contains_key(&project_model_usage_key("provider_b", "shared_nch")),
            "the shadowed provider must not claim the instance"
        );
    }

    #[test]
    fn open_source_handoff_selects_the_exact_include_graph_member() {
        let mut app = RSpiceApp::test_instance();
        let source = std::path::PathBuf::from("models/pdk/corners.lib");

        select_project_source_in_include_graph(&mut app, "foundry", Some(&source));

        assert_eq!(app.state.workbench.models_page, ModelsPage::Include);
        assert_eq!(
            app.state
                .workbench
                .model_include_selected_library
                .as_deref(),
            Some("foundry")
        );
        assert_eq!(
            app.state.workbench.model_include_selected_source.as_deref(),
            Some(source.as_path())
        );
    }
}
