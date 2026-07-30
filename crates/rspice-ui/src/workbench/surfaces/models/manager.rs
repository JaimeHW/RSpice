//! Mounted Models & PDKs manager composition.
//!
//! The legacy surface remains in the parent module only as reusable rendering
//! and qualification code. This module owns the current six-page workbench,
//! corpus scopes, guarded source/pack actions, and model detail composition.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use egui::{Align, Color32, Layout, RichText, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::state::model_library::{
    DeviceModel, ModelLibrary, ModelSourceAuthority, PackModelHit, ProcessCorner,
};
use crate::state::{
    CellViewRef, Component, ComponentType, ModelBoundSymbolDefinition, SymbolDocument, ViewType,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::design_history::publish_model_library_candidate;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::{
    ModelPackFacet, ModelsCatalogScope, ModelsOperationalState, ModelsPage, ModelsWorkbenchDialog,
    ProjectModelFacet, RSpicePartFacet,
};

const ROW_H: f32 = 27.0;
const CATALOG_LIMIT: usize = 160;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);
    ui.visuals_mut().widgets.noninteractive.bg_fill = t.color.bg_panel;

    manager_toolbar(ui, app);
    page_tabs(ui, app);
    show_action_receipt(ui, app);

    match app.state.workbench.models_page {
        ModelsPage::Models => catalog_page(ui, app),
        ModelsPage::Symbols => symbols_page(ui, app),
        ModelsPage::Corners => corners_page(ui, app),
        ModelsPage::Bins => bins_page(ui, app),
        ModelsPage::Include => include_page(ui, app),
        ModelsPage::Qualification => super::qualification(ui, app),
    }
    render_dialog(ui, app);
}

fn manager_toolbar(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("MODELS & PDKS")
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.separator();
                if ui.button("＋ Add library").clicked() {
                    Command::PdkSettings.execute(app);
                }
                if ui.button("↻ Rescan libraries").clicked() {
                    let discovered = app.state.pdk_config.discover_model_files().len();
                    let errors = app.state.pdk_config.scan_errors.len();
                    app.state.model_library_manager.discover_spice_packs();
                    receipt(
                        app,
                        if errors == 0 {
                            Ok(format!(
                                "Rescan completed: {discovered} configured model sources and {} shipped packs are available.",
                                app.state
                                    .model_library_manager
                                    .spice_packs()
                                    .map_or(0, |index| index.packs().len())
                            ))
                        } else {
                            Err(format!(
                                "Rescan found {discovered} sources and {errors} path errors. Open PDK settings for details."
                            ))
                        },
                    );
                }
                if ui.button("⌁ Compile Verilog-A").clicked() {
                    Command::CompileVerilogA.execute(app);
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let libraries = app.state.model_library_manager.library_count();
                    let models = app.state.model_library_manager.total_model_count();
                    let state = if models == 0
                        && app.state.workbench.models_view.operational_state
                            == ModelsOperationalState::Ready
                    {
                        ModelsOperationalState::Empty
                    } else {
                        app.state.workbench.models_view.operational_state
                    };
                    let state_color = match state {
                        ModelsOperationalState::Ready | ModelsOperationalState::Recovered => {
                            t.color.ok
                        }
                        ModelsOperationalState::Empty
                        | ModelsOperationalState::Loading
                        | ModelsOperationalState::Cancelled => t.color.text_dim,
                        ModelsOperationalState::InvalidInput
                        | ModelsOperationalState::ReadOnly
                        | ModelsOperationalState::Offline
                        | ModelsOperationalState::Stale
                        | ModelsOperationalState::Permission
                        | ModelsOperationalState::Entitlement
                        | ModelsOperationalState::Rollback
                        | ModelsOperationalState::Partial => t.color.warn,
                        ModelsOperationalState::ExecutionError
                        | ModelsOperationalState::Conflict
                        | ModelsOperationalState::Corrupted => t.color.err,
                    };
                    ui.label(
                        RichText::new(state.label())
                            .small()
                            .strong()
                            .color(state_color),
                    )
                    .on_hover_text("Current Models & PDKs workflow state");
                    ui.separator();
                    ui.label(
                        RichText::new(format!("{libraries} sources · {models} loaded"))
                            .monospace()
                            .small()
                            .color(t.color.text_dim),
                    );
                });
            });
        });
}

fn page_tabs(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .show(ui, |ui| {
            ScrollArea::horizontal()
                .id_salt("models-pdks-page-tabs")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 1.0;
                        for page in ModelsPage::ALL {
                            if ui
                                .selectable_label(
                                    app.state.workbench.models_page == page,
                                    page.label(),
                                )
                                .clicked()
                            {
                                app.state.workbench.models_page = page;
                            }
                        }
                    });
                });
        });
}

fn show_action_receipt(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(result) = app.state.workbench.models_view.action_receipt.clone() else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    let (message, color) = match result {
        Ok(ref message) => (message.as_str(), t.color.ok),
        Err(ref message) => (message.as_str(), t.color.err),
    };
    egui::Frame::NONE
        .fill(color.linear_multiply(0.08))
        .stroke(Stroke::new(1.0, color.linear_multiply(0.65)))
        .inner_margin(egui::Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(message).color(color).small());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.small_button("×").on_hover_text("Dismiss").clicked() {
                        app.state.workbench.models_view.action_receipt = None;
                        app.state.workbench.models_view.operational_state =
                            ModelsOperationalState::Ready;
                    }
                });
            });
        });
}

fn catalog_page(ui: &mut Ui, app: &mut RSpiceApp) {
    catalog_scope_strip(ui, app);
    match app.state.workbench.models_view.catalog_scope {
        ModelsCatalogScope::Project => project_catalog(ui, app),
        ModelsCatalogScope::InstalledPacks => pack_catalog(ui, app),
        ModelsCatalogScope::RSpiceLibrary => parts_catalog(ui, app),
    }
}

fn catalog_scope_strip(ui: &mut Ui, app: &mut RSpiceApp) {
    let loaded = app.state.model_library_manager.total_model_count();
    let packs = app
        .state
        .model_library_manager
        .spice_packs()
        .map_or(0, |index| index.packs().len());
    let parts = app.state.model_library_manager.pack_definition_count();
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(7, 4))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for (scope, count) in [
                    (ModelsCatalogScope::Project, loaded),
                    (ModelsCatalogScope::InstalledPacks, packs),
                    (ModelsCatalogScope::RSpiceLibrary, parts),
                ] {
                    let text = format!("{}  {}", scope.label(), count);
                    if ui
                        .selectable_label(
                            app.state.workbench.models_view.catalog_scope == scope,
                            text,
                        )
                        .clicked()
                    {
                        app.state.workbench.models_view.catalog_scope = scope;
                        app.state.workbench.models_view.catalog_query.clear();
                    }
                }
            });
        });
}

#[derive(Clone)]
struct ProjectModelRow {
    library: String,
    model: DeviceModel,
    source: String,
    pinned: bool,
    review: bool,
    protected: bool,
    usages: Vec<String>,
    vectors: usize,
}

fn project_catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    project_filter_bar(ui, app);
    let query = app
        .state
        .workbench
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    let facet = app.state.workbench.models_view.project_facet;
    let mut rows = Vec::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            let usages = model_consumers(app, &model.name);
            let pinned =
                !library.source_closure.is_empty() || library.source_authority.is_project_owned();
            let review = model_geometry_invalid(model)
                || (library.root_path.is_some() && library.source_closure.is_empty())
                || model.description.trim().is_empty();
            let protected = matches!(library.source_authority, ModelSourceAuthority::BuiltIn);
            let vectors = library
                .model_qualification
                .get(&model.name)
                .map_or(0, |qualification| {
                    qualification
                        .suites
                        .iter()
                        .map(|suite| suite.vectors.len())
                        .sum()
                });
            let source = model
                .file_path
                .as_deref()
                .or(library.root_path.as_deref())
                .map(path_label)
                .unwrap_or_else(|| match library.source_authority {
                    ModelSourceAuthority::BuiltIn => "RSpice built-in".to_owned(),
                    ModelSourceAuthority::External => "external source".to_owned(),
                    ModelSourceAuthority::ProjectOwned { .. } => "project source".to_owned(),
                });
            let haystack = format!(
                "{} {} {} {} {}",
                model.name,
                model.description,
                model.model_type.display_name(),
                library.name,
                model
                    .parameters
                    .iter()
                    .map(|(name, value)| format!("{name}={value}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .to_ascii_lowercase();
            let facet_match = match facet {
                ProjectModelFacet::All => true,
                ProjectModelFacet::Bound => !usages.is_empty(),
                ProjectModelFacet::Pinned => pinned,
                ProjectModelFacet::Review => review,
                ProjectModelFacet::Protected => protected,
            };
            if facet_match && (query.is_empty() || haystack.contains(&query)) {
                rows.push(ProjectModelRow {
                    library: library.name.clone(),
                    model: model.clone(),
                    source,
                    pinned,
                    review,
                    protected,
                    usages,
                    vectors,
                });
            }
        }
    }
    rows.sort_by(|left, right| {
        left.model
            .name
            .to_ascii_lowercase()
            .cmp(&right.model.name.to_ascii_lowercase())
            .then_with(|| left.library.cmp(&right.library))
    });

    let table_h = (ui.available_height() * 0.43).clamp(150.0, 310.0);
    card(ui, |ui| {
        table_header(
            ui,
            &[
                ("MODEL", 0.20),
                ("FAMILY", 0.17),
                ("SOURCE", 0.22),
                ("USED BY", 0.16),
                ("VECTORS", 0.10),
                ("STATUS", 0.15),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-project-table")
            .max_height(table_h)
            .show(ui, |ui| {
                if rows.is_empty() {
                    empty_state(
                        ui,
                        "No models match the current catalog filter.",
                        "Search covers names, families, sources, libraries, consumers and resolved parameters.",
                    );
                    if ui.button("Clear filter").clicked() {
                        app.state.workbench.models_view.catalog_query.clear();
                        app.state.workbench.models_view.project_facet = ProjectModelFacet::All;
                    }
                } else {
                    for row in &rows {
                        project_model_row(ui, app, row);
                    }
                }
            });
    });

    selected_model_detail(ui, app);
}

fn project_filter_bar(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.horizontal_wrapped(|ui| {
        for facet in ProjectModelFacet::ALL {
            let count = project_facet_count(app, facet);
            if ui
                .selectable_label(
                    app.state.workbench.models_view.project_facet == facet,
                    format!("{} {}", facet.label(), count),
                )
                .clicked()
            {
                app.state.workbench.models_view.project_facet = facet;
            }
        }
        ui.add(
            egui::TextEdit::singleline(&mut app.state.workbench.models_view.catalog_query)
                .hint_text("Search models, parameters or consumers…")
                .desired_width(250.0),
        );
    });
}

fn project_facet_count(app: &RSpiceApp, facet: ProjectModelFacet) -> usize {
    app.state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library.models.values().map(move |model| {
                let bound = !model_consumers(app, &model.name).is_empty();
                let pinned = !library.source_closure.is_empty()
                    || library.source_authority.is_project_owned();
                let review = model_geometry_invalid(model)
                    || (library.root_path.is_some() && library.source_closure.is_empty())
                    || model.description.trim().is_empty();
                let protected = matches!(library.source_authority, ModelSourceAuthority::BuiltIn);
                match facet {
                    ProjectModelFacet::All => true,
                    ProjectModelFacet::Bound => bound,
                    ProjectModelFacet::Pinned => pinned,
                    ProjectModelFacet::Review => review,
                    ProjectModelFacet::Protected => protected,
                }
            })
        })
        .filter(|matches| *matches)
        .count()
}

fn project_model_row(ui: &mut Ui, app: &mut RSpiceApp, row: &ProjectModelRow) {
    let selected = app.state.model_library_manager.selected_library.as_deref()
        == Some(row.library.as_str())
        && app.state.workbench.selected_model.as_deref() == Some(row.model.name.as_str());
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_H), Sense::click());
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, t.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    if response.clicked() {
        app.state.model_library_manager.select_library(&row.library);
        app.state.workbench.selected_model = Some(row.model.name.clone());
    }
    let status = if row.review {
        "review"
    } else if row.protected {
        "protected"
    } else if row.pinned {
        "pinned"
    } else {
        "unsealed"
    };
    paint_columns(
        ui,
        rect,
        &[
            (&row.model.name, 0.20, true),
            (row.model.model_type.display_name(), 0.17, false),
            (&row.source, 0.22, false),
            (
                if row.usages.is_empty() {
                    "—"
                } else {
                    row.usages[0].as_str()
                },
                0.16,
                false,
            ),
            (&row.vectors.to_string(), 0.10, true),
            (status, 0.15, true),
        ],
    );
}

fn selected_model_detail(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(library_name) = app.state.model_library_manager.selected_library.clone() else {
        empty_state(
            ui,
            "Select a model to inspect its exact source and resolved contract.",
            "The detail area never invents a model when the catalog selection is empty.",
        );
        return;
    };
    let Some(model_name) = app.state.workbench.selected_model.clone() else {
        empty_state(
            ui,
            "Select a model to inspect its exact source and resolved contract.",
            "Use the table above or choose a model source in the Navigator.",
        );
        return;
    };
    let Some((library, model)) = app
        .state
        .model_library_manager
        .get_library(&library_name)
        .and_then(|library| library.get_model(&model_name).map(|model| (library, model)))
        .map(|(library, model)| (library.clone(), model.clone()))
    else {
        empty_state(
            ui,
            "The selected model no longer resolves.",
            "Rescan or clear the selection; stale identities are never retargeted automatically.",
        );
        return;
    };
    let usages = model_consumers(app, &model.name);
    let selected_component = exactly_one_selected_component(app);
    let source_available = !library.source_contents.is_empty() || model.file_path.is_some();

    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(&model.name)
                .monospace()
                .strong()
                .color(Tokens::get(ui.ctx()).color.text),
        );
        ui.label(
            RichText::new(format!(
                "{} · {} · {}",
                model.model_type.display_name(),
                model.level.display_name(),
                library.name
            ))
            .small(),
        );
        if ui
            .add_enabled(
                library.root_path.is_some(),
                egui::Button::new(if library.source_closure.is_empty() {
                    "Pin source"
                } else {
                    "Refresh pin"
                }),
            )
            .on_disabled_hover_text("Built-in sources do not have an external file to pin.")
            .clicked()
        {
            refresh_library(app, &library);
        }
        if ui
            .add_enabled(source_available, egui::Button::new("Open source"))
            .on_disabled_hover_text("This built-in definition has no source document.")
            .clicked()
        {
            open_model_source(app, &library, &model);
        }
        if ui.button("Compare…").clicked() {
            open_model_compare(app, &library.name, &model.name);
        }
        let project_owned = library.source_authority.is_project_owned();
        if ui
            .button(if project_owned {
                "Model editor…"
            } else {
                "Author project copy…"
            })
            .clicked()
        {
            if project_owned {
                Command::ModelEditor.execute(app);
            } else {
                Command::ModelCreateProjectCopy.execute(app);
            }
        }
        if ui.button("Qualification").clicked() {
            app.state.workbench.models_page = ModelsPage::Qualification;
        }
        if ui
            .add_enabled(
                selected_component.is_some(),
                egui::Button::new("Bind to selection…"),
            )
            .on_disabled_hover_text("Select exactly one compatible schematic instance first.")
            .clicked()
            && let Some(component_id) = selected_component
        {
            let result = crate::workbench::docks::bind_component_model_from_catalog(
                app,
                component_id,
                &library.name,
                &model.name,
            )
            .map(|()| {
                format!(
                    "Bound selected instance to model '{} / {}'.",
                    library.name, model.name
                )
            });
            receipt(app, result);
        }
    });

    ScrollArea::vertical()
        .id_salt("models-selected-detail")
        .show(ui, |ui| {
            let wide = ui.available_width() >= 760.0;
            if wide {
                ui.columns(2, |columns| {
                    parameter_card(&mut columns[0], &library, &model);
                    characteristic_card(&mut columns[1], &model);
                });
                ui.columns(2, |columns| {
                    qualification_card(&mut columns[0], &library, &model);
                    usage_card(&mut columns[1], &model, &usages, app);
                });
            } else {
                parameter_card(ui, &library, &model);
                characteristic_card(ui, &model);
                qualification_card(ui, &library, &model);
                usage_card(ui, &model, &usages, app);
            }
        });
}

fn parameter_card(ui: &mut Ui, library: &ModelLibrary, model: &DeviceModel) {
    card(ui, |ui| {
        card_title(
            ui,
            "RESOLVED PARAMETERS",
            Some(&format!("{} values", model.parameters.len())),
        );
        let mut values = model
            .parameters
            .iter()
            .map(|(name, value)| (name.clone(), value.to_string(), "source card"))
            .collect::<Vec<_>>();
        values.extend(
            model
                .string_parameters
                .iter()
                .map(|(name, value)| (name.clone(), value.clone(), "source string")),
        );
        values.sort_by(|left, right| left.0.cmp(&right.0));
        if values.is_empty() {
            empty_state(
                ui,
                "No parameter card is attached to this model.",
                "Built-in equation defaults remain owned by the engine.",
            );
        } else {
            for (name, value, origin) in values.into_iter().take(24) {
                property(ui, &name, &value, origin);
            }
        }
        if let Some(metadata) = library.model_definition_metadata.get(&model.name) {
            ui.separator();
            property(
                ui,
                "Schema",
                &format!("{} typed fields", metadata.parameters.len()),
                "project definition",
            );
        }
    });
}

fn characteristic_card(ui: &mut Ui, model: &DeviceModel) {
    card(ui, |ui| {
        card_title(ui, "CHARACTERISTIC", Some("analytic preview"));
        let Some(vth) = model.vth0.or_else(|| model.parameters.get("vth0").copied()) else {
            empty_state(
                ui,
                "No function-defined characteristic is available.",
                "Open Model Editor or attach qualification vectors to add an evidence-backed plot.",
            );
            return;
        };
        let vmax = model.vdd.unwrap_or(1.8).max(vth + 0.2);
        let desired = egui::vec2(ui.available_width().max(120.0), 112.0);
        let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
        let t = Tokens::get(ui.ctx());
        ui.painter().rect(
            rect,
            2.0,
            t.color.bg_inset,
            Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        let plot = rect.shrink2(egui::vec2(10.0, 9.0));
        ui.painter().line_segment(
            [plot.left_bottom(), plot.right_bottom()],
            Stroke::new(1.0, t.color.border_strong),
        );
        ui.painter().line_segment(
            [plot.left_bottom(), plot.left_top()],
            Stroke::new(1.0, t.color.border_strong),
        );
        let points = (0..=48)
            .map(|index| {
                let voltage = vmax * f64::from(index) / 48.0;
                let current = (voltage - vth).max(0.0).powi(2);
                let max_current = (vmax - vth).max(0.01).powi(2);
                egui::pos2(
                    plot.left() + plot.width() * index as f32 / 48.0,
                    plot.bottom() - plot.height() * (current / max_current) as f32,
                )
            })
            .collect::<Vec<_>>();
        ui.painter()
            .add(egui::Shape::line(points, Stroke::new(1.8, t.color.accent)));
        ui.label(
            RichText::new(format!(
                "Square-law preview from retained VTH0={vth:.4} V · range 0…{vmax:.3} V"
            ))
            .small()
            .color(t.color.text_dim),
        );
    });
}

fn qualification_card(ui: &mut Ui, library: &ModelLibrary, model: &DeviceModel) {
    card(ui, |ui| {
        card_title(ui, "QUALIFICATION", Some("source-owned evidence"));
        if let Some(state) = library.model_qualification.get(&model.name) {
            property(ui, "Suites", &state.suites.len().to_string(), "retained");
            property(
                ui,
                "Vectors",
                &state
                    .suites
                    .iter()
                    .map(|suite| suite.vectors.len())
                    .sum::<usize>()
                    .to_string(),
                "declared",
            );
            property(
                ui,
                "Evidence",
                &state.evidence.len().to_string(),
                "immutable",
            );
            property(
                ui,
                "Releases",
                &state.releases.len().to_string(),
                "promoted",
            );
            let open = state
                .vector_dispositions
                .iter()
                .filter(|disposition| disposition.is_open())
                .count();
            property(
                ui,
                "Open dispositions",
                &open.to_string(),
                if open == 0 {
                    "clean"
                } else {
                    "review required"
                },
            );
        } else {
            empty_state(
                ui,
                "This model has no qualification suite.",
                "Qualification claims remain empty until a retained suite and exact-source evidence exist.",
            );
        }
    });
}

fn usage_card(ui: &mut Ui, model: &DeviceModel, usages: &[String], app: &mut RSpiceApp) {
    card(ui, |ui| {
        card_title(
            ui,
            "WHERE USED",
            Some(&format!("{} consumers", usages.len())),
        );
        if usages.is_empty() {
            empty_state(
                ui,
                "Not bound in the active project.",
                "Place an instance or select one and use Bind to selection.",
            );
        } else {
            for usage in usages.iter().take(12) {
                if ui.link(usage).clicked() {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::BindingTrace {
                            model: model.name.clone(),
                            consumers: usages.to_vec(),
                        });
                }
            }
        }
    });
}

fn pack_catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    let packs = app
        .state
        .model_library_manager
        .spice_packs()
        .map(|index| index.packs().to_vec())
        .unwrap_or_default();
    let facet = app.state.workbench.models_view.pack_facet;
    let query = app
        .state
        .workbench
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    ui.horizontal_wrapped(|ui| {
        for candidate in ModelPackFacet::ALL {
            if ui
                .selectable_label(
                    facet == candidate,
                    format!(
                        "{} {}",
                        candidate.label(),
                        pack_facet_count(app, &packs, candidate)
                    ),
                )
                .clicked()
            {
                app.state.workbench.models_view.pack_facet = candidate;
            }
        }
        ui.add(
            egui::TextEdit::singleline(&mut app.state.workbench.models_view.catalog_query)
                .hint_text("Search installed packs…")
                .desired_width(210.0),
        );
    });
    let visible = packs
        .iter()
        .filter(|pack| {
            let attached = attached_library_for_pack(app, &pack.id).is_some();
            let facet_match = match facet {
                ModelPackFacet::All => true,
                ModelPackFacet::NeedsAttention => pack.entry.is_none() || !pack.redistributable,
                ModelPackFacet::Attached => attached,
                ModelPackFacet::Foundry => pack.category.eq_ignore_ascii_case("foundry"),
                ModelPackFacet::Vendor => pack.category.eq_ignore_ascii_case("vendor"),
                ModelPackFacet::Community => pack.category.eq_ignore_ascii_case("community"),
                ModelPackFacet::Redistributable => pack.redistributable,
            };
            let haystack = format!("{} {} {} {}", pack.id, pack.name, pack.category, pack.spdx)
                .to_ascii_lowercase();
            facet_match && (query.is_empty() || haystack.contains(&query))
        })
        .cloned()
        .collect::<Vec<_>>();
    let table_h = (ui.available_height() * 0.53).clamp(180.0, 360.0);
    card(ui, |ui| {
        table_header(
            ui,
            &[
                ("PACK", 0.25),
                ("CONTENTS", 0.18),
                ("ORIGIN", 0.12),
                ("PARTS", 0.11),
                ("LICENSE", 0.17),
                ("STATE", 0.17),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-pack-table")
            .max_height(table_h)
            .show(ui, |ui| {
                if visible.is_empty() {
                    empty_state(
                        ui,
                        "No pack matches this facet.",
                        "Facets derive from the installed corpus manifest and live project attachments.",
                    );
                    if ui.button("Clear filter").clicked() {
                        app.state.workbench.models_view.pack_facet = ModelPackFacet::All;
                        app.state.workbench.models_view.catalog_query.clear();
                    }
                }
                for pack in &visible {
                    let selected = app.state.workbench.models_view.selected_pack.as_deref()
                        == Some(pack.id.as_str());
                    let attached = attached_library_for_pack(app, &pack.id).is_some();
                    let state = if attached {
                        "attached"
                    } else if pack.entry.is_none() {
                        "no entry"
                    } else if !pack.redistributable {
                        "license review"
                    } else {
                        "available"
                    };
                    selectable_data_row(
                        ui,
                        selected,
                        &[
                            (&pack.name, 0.25, false),
                            (
                                &format!("{} models · {} subckts", pack.models, pack.subcircuits),
                                0.18,
                                false,
                            ),
                            (&pack.category, 0.12, false),
                            (
                                &(pack.models_top + pack.subcircuits_top).to_string(),
                                0.11,
                                true,
                            ),
                            (&pack.spdx, 0.17, true),
                            (state, 0.17, true),
                        ],
                    )
                    .clicked()
                    .then(|| {
                        app.state.workbench.models_view.selected_pack = Some(pack.id.clone())
                    });
                }
            });
    });
    pack_detail(ui, app, &packs);
}

fn pack_facet_count(
    app: &RSpiceApp,
    packs: &[rspice_core::library::SpicePack],
    facet: ModelPackFacet,
) -> usize {
    packs
        .iter()
        .filter(|pack| match facet {
            ModelPackFacet::All => true,
            ModelPackFacet::NeedsAttention => pack.entry.is_none() || !pack.redistributable,
            ModelPackFacet::Attached => attached_library_for_pack(app, &pack.id).is_some(),
            ModelPackFacet::Foundry => pack.category.eq_ignore_ascii_case("foundry"),
            ModelPackFacet::Vendor => pack.category.eq_ignore_ascii_case("vendor"),
            ModelPackFacet::Community => pack.category.eq_ignore_ascii_case("community"),
            ModelPackFacet::Redistributable => pack.redistributable,
        })
        .count()
}

fn pack_detail(ui: &mut Ui, app: &mut RSpiceApp, packs: &[rspice_core::library::SpicePack]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_pack
        .as_deref()
        .and_then(|id| packs.iter().find(|pack| pack.id == id))
        .cloned()
        .or_else(|| packs.first().cloned());
    let Some(pack) = selected else {
        empty_state(
            ui,
            "No shipped model corpus is installed.",
            "Set RSPICE_MODELS_DIR or install the versioned model-pack tree, then rescan.",
        );
        return;
    };
    app.state.workbench.models_view.selected_pack = Some(pack.id.clone());
    let attached = attached_library_for_pack(app, &pack.id);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(&pack.name).strong());
        ui.label(RichText::new(&pack.id).monospace().small());
        if ui.button("Browse parts").clicked() {
            app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::RSpiceLibrary;
            app.state.workbench.models_view.catalog_query.clear();
            app.state.workbench.models_view.selected_pack = Some(pack.id.clone());
        }
        if let Some(library) = attached.as_deref() {
            if ui.button("Refresh snapshot").clicked()
                && let Some(library) = app
                    .state
                    .model_library_manager
                    .get_library(library)
                    .cloned()
            {
                refresh_library(app, &library);
            }
            if ui.button("Detach…").clicked() {
                app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
                    pack_id: pack.id.clone(),
                    attach: false,
                });
            }
        } else if ui
            .add_enabled(
                pack.entry.is_some() && pack.redistributable,
                egui::Button::new("Attach…"),
            )
            .on_disabled_hover_text(if !pack.redistributable {
                "This pack has no established redistribution grant."
            } else {
                "The pack manifest has no attachable entry file."
            })
            .clicked()
        {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
                pack_id: pack.id.clone(),
                attach: true,
            });
        }
    });
    card(ui, |ui| {
        card_title(ui, "PACK CONTRACT", Some(&pack.category));
        property(
            ui,
            "Contents",
            &format!(
                "{} addressable · {} total definitions · {} files",
                pack.models_top + pack.subcircuits_top,
                pack.models + pack.subcircuits,
                pack.files
            ),
            "manifest",
        );
        property(ui, "License", &pack.spdx, pack.tier.display_name());
        property(
            ui,
            "Redistributable",
            if pack.redistributable { "yes" } else { "no" },
            "enforced before project embedding",
        );
        property(
            ui,
            "Attachment",
            attached.as_deref().unwrap_or("not attached"),
            if attached.is_some() {
                "authenticated project source"
            } else {
                "corpus only"
            },
        );
        property(
            ui,
            "Entry",
            pack.entry
                .as_deref()
                .map(path_label)
                .as_deref()
                .unwrap_or("not declared"),
            "pack manifest",
        );
    });
}

fn parts_catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    let facet = app.state.workbench.models_view.part_facet;
    ui.horizontal_wrapped(|ui| {
        for candidate in RSpicePartFacet::ALL {
            if ui
                .selectable_label(facet == candidate, candidate.label())
                .clicked()
            {
                app.state.workbench.models_view.part_facet = candidate;
                app.state.workbench.models_view.selected_part = None;
            }
        }
        ui.add(
            egui::TextEdit::singleline(&mut app.state.workbench.models_view.catalog_query)
                .hint_text(format!(
                    "Search {} parts by name, class or pack…",
                    app.state.model_library_manager.pack_definition_count()
                ))
                .desired_width(300.0),
        );
    });
    let mut hits = app
        .state
        .model_library_manager
        .browse_pack_models(
            &app.state.workbench.models_view.catalog_query,
            facet.device_filter(),
            CATALOG_LIMIT,
        )
        .unwrap_or_else(|error| {
            receipt(app, Err(error));
            Vec::new()
        });
    if let Some(pack_id) = app.state.workbench.models_view.selected_pack.as_deref() {
        hits.retain(|hit| hit.pack == pack_id);
    }
    let table_h = (ui.available_height() * 0.46).clamp(170.0, 330.0);
    card(ui, |ui| {
        table_header(
            ui,
            &[
                ("PART", 0.20),
                ("DESCRIPTION", 0.22),
                ("CLASS", 0.14),
                ("KIND", 0.10),
                ("PACK", 0.19),
                ("AVAILABILITY", 0.15),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-parts-table")
            .max_height(table_h)
            .show(ui, |ui| {
                if hits.is_empty() {
                    empty_state(
                        ui,
                        "No addressable part matches the current search and class.",
                        "Private helper models declared inside macromodel bodies are intentionally excluded.",
                    );
                    if ui.button("Clear search").clicked() {
                        app.state.workbench.models_view.catalog_query.clear();
                        app.state.workbench.models_view.part_facet = RSpicePartFacet::All;
                        app.state.workbench.models_view.selected_pack = None;
                    }
                }
                for hit in &hits {
                    let key = part_key(hit);
                    let selected = app.state.workbench.models_view.selected_part.as_deref()
                        == Some(key.as_str());
                    let availability = if hit.restricted {
                        "restricted"
                    } else if hit.source.as_ref().is_some_and(|path| path.is_file()) {
                        "on disk"
                    } else {
                        "sync required"
                    };
                    if selectable_data_row(
                        ui,
                        selected,
                        &[
                            (&hit.name, 0.20, true),
                            (&hit.pack_name, 0.22, false),
                            (&hit.device, 0.14, false),
                            (&hit.kind, 0.10, true),
                            (&hit.pack, 0.19, true),
                            (availability, 0.15, true),
                        ],
                    )
                    .clicked()
                    {
                        app.state.workbench.models_view.selected_part = Some(key);
                    }
                }
            });
    });
    selected_part_detail(ui, app, &hits);
}

fn selected_part_detail(ui: &mut Ui, app: &mut RSpiceApp, hits: &[PackModelHit]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_part
        .as_deref()
        .and_then(|key| hits.iter().find(|hit| part_key(hit) == key))
        .cloned()
        .or_else(|| hits.first().cloned());
    let Some(hit) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_part = Some(part_key(&hit));
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(&hit.name).monospace().strong());
        ui.label(format!("{} · {} · {}", hit.device, hit.kind, hit.pack_name));
        if ui.button("Show pack").clicked() {
            app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::InstalledPacks;
            app.state.workbench.models_view.selected_pack = Some(hit.pack.clone());
            app.state.workbench.models_view.catalog_query.clear();
        }
        if ui
            .add_enabled(
                hit.source.as_ref().is_some_and(|path| path.is_file())
                    && hit.redistributable
                    && !hit.restricted,
                egui::Button::new("Add to project…"),
            )
            .on_disabled_hover_text(if hit.restricted || !hit.redistributable {
                "The source is not licensed for embedding in a project."
            } else {
                "The card is not present on disk; rescan or sync the corpus."
            })
            .clicked()
        {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPart {
                pack_id: hit.pack.clone(),
                part_name: hit.name.clone(),
            });
        }
        if ui.button("Open qualification").clicked() {
            app.state.workbench.models_page = ModelsPage::Qualification;
        }
        if ui
            .add_enabled(
                hit.source.as_ref().is_some_and(|path| path.is_file()),
                egui::Button::new("Open card"),
            )
            .clicked()
            && let Some(source) = hit.source.as_ref()
        {
            match std::fs::read_to_string(source) {
                Ok(body) => {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::SourcePreview {
                            title: hit.name.clone(),
                            subtitle: format!(
                                "{}:{} · read-only corpus source",
                                source.display(),
                                hit.line
                            ),
                            source: body,
                            read_only: true,
                        });
                }
                Err(error) => receipt(
                    app,
                    Err(format!("Could not open '{}': {error}", source.display())),
                ),
            }
        }
    });
    card(ui, |ui| {
        card_title(ui, "DEFINITION", Some(&hit.kind));
        property(ui, "Name", &hit.name, "catalog identity");
        property(ui, "Device class", &hit.device, "canonical");
        property(ui, "Pack", &hit.pack_name, &hit.pack);
        property(
            ui,
            "Source",
            hit.source
                .as_deref()
                .map(path_label)
                .as_deref()
                .unwrap_or("not on disk"),
            &format!("line {}", hit.line),
        );
        property(
            ui,
            "Project eligibility",
            if hit.redistributable && !hit.restricted {
                "eligible"
            } else {
                "blocked"
            },
            "license policy",
        );
    });
}

#[derive(Clone)]
struct SymbolRow {
    reference: CellViewRef,
    read_only: bool,
    family: String,
    pins: Vec<String>,
    form: String,
    template: String,
    status: String,
    definition: Option<ModelBoundSymbolDefinition>,
    diagnostics: Vec<String>,
}

fn symbols_page(ui: &mut Ui, app: &mut RSpiceApp) {
    section_title(
        ui,
        "Symbols, pins & device forms",
        "Project and technology symbol contracts · pin order is netlist order",
        |ui| {
            if ui.button("Library manager").clicked() {
                navigate_specialist(app, crate::workbench::SurfaceId::LibraryCellviewManager);
            }
            if ui.button("Import symbol").clicked() {
                super::open_symbol_import_dialog(&mut app.state);
            }
            if ui.button("Form designer").clicked() {
                super::open_symbol_parameter_form_dialog(&mut app.state);
            }
            if ui.button("Create symbol").clicked() {
                super::open_create_model_bound_symbol_dialog(&mut app.state);
            }
        },
    );
    let rows = symbol_rows(app);
    let project_count = rows.iter().filter(|row| !row.read_only).count();
    let technology_count = rows.iter().filter(|row| row.read_only).count();
    ui.horizontal(|ui| {
        ui.label(format!(
            "{project_count} project · {technology_count} technology · {} total",
            rows.len()
        ));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                RichText::new("pin/provider mismatches block netlisting")
                    .small()
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
        });
    });
    let table_h = (ui.available_height() * 0.42).clamp(170.0, 320.0);
    card(ui, |ui| {
        table_header(
            ui,
            &[
                ("SYMBOL", 0.24),
                ("BOUND FAMILY", 0.20),
                ("PINS", 0.24),
                ("FORM", 0.17),
                ("STATUS", 0.15),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-symbol-registry")
            .max_height(table_h)
            .show(ui, |ui| {
                if rows.is_empty() {
                    empty_state(
                        ui,
                        "No symbol views are present in the loaded design libraries.",
                        "Import a symbol or create a model-bound symbol to establish an executable pin contract.",
                    );
                }
                for row in &rows {
                    let key = symbol_key(&row.reference);
                    let selected = app.state.workbench.models_view.selected_symbol.as_deref()
                        == Some(key.as_str());
                    if selectable_data_row(
                        ui,
                        selected,
                        &[
                            (
                                &format!("{}/{}", row.reference.cell, row.reference.view),
                                0.24,
                                true,
                            ),
                            (&row.family, 0.20, false),
                            (&row.pins.join(" "), 0.24, true),
                            (&row.form, 0.17, false),
                            (&row.status, 0.15, true),
                        ],
                    )
                    .clicked()
                    {
                        app.state.workbench.models_view.selected_symbol = Some(key);
                        app.state.library_manager.select_view(
                            &row.reference.library,
                            &row.reference.cell,
                            &row.reference.view,
                        );
                    }
                }
            });
    });
    symbol_detail(ui, app, &rows);
}

fn symbol_rows(app: &RSpiceApp) -> Vec<SymbolRow> {
    let mut rows = Vec::new();
    for library in app.state.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            for view in cell
                .views_sorted()
                .into_iter()
                .filter(|view| view.view_type == ViewType::Symbol)
            {
                let definition_result = ModelBoundSymbolDefinition::load_from_view(view);
                let document_result = SymbolDocument::load_from_view(view);
                let definition = definition_result.as_ref().ok().and_then(Clone::clone);
                let pins = document_result
                    .as_ref()
                    .map(|document| {
                        document
                            .pins
                            .iter()
                            .map(|pin| pin.name.clone())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                let family = definition
                    .as_ref()
                    .and_then(|definition| definition.netlist.model.as_ref())
                    .map(|model| format!("{}/{}", model.library, model.model))
                    .unwrap_or_else(|| super::symbol_model_family(app, cell));
                let form = definition
                    .as_ref()
                    .map(super::symbol_parameter_form_label)
                    .unwrap_or_else(|| "legacy / none".to_owned());
                let template = definition
                    .as_ref()
                    .map(|definition| definition.netlist.template.clone())
                    .filter(|template| !template.trim().is_empty())
                    .unwrap_or_else(|| "not defined".to_owned());
                let mut diagnostics = Vec::new();
                if let Err(error) = definition_result {
                    diagnostics.push(format!("Definition metadata: {error}"));
                }
                if let Err(error) = &document_result {
                    diagnostics.push(format!("Symbol document: {error}"));
                }
                if let (Some(definition), Ok(document)) = (&definition, &document_result) {
                    let expected = definition
                        .pins
                        .iter()
                        .map(|pin| pin.name.to_ascii_lowercase())
                        .collect::<Vec<_>>();
                    let observed = document
                        .pins
                        .iter()
                        .map(|pin| pin.name.to_ascii_lowercase())
                        .collect::<Vec<_>>();
                    if expected != observed {
                        diagnostics.push(format!(
                            "Blocking pin mismatch: provider {:?}, symbol {:?}",
                            expected, observed
                        ));
                    }
                    if let Err(error) = definition.validate() {
                        diagnostics.push(format!("Executable contract: {error}"));
                    }
                } else if definition.is_none() {
                    diagnostics
                        .push("Legacy symbol has no typed model/netlist/form contract.".to_owned());
                }
                let status = if diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.contains("Blocking"))
                {
                    "pin mismatch"
                } else if !diagnostics.is_empty() {
                    "review"
                } else if library.read_only {
                    "read-only"
                } else {
                    "bound"
                }
                .to_owned();
                rows.push(SymbolRow {
                    reference: CellViewRef::new(&library.name, &cell.name, &view.name),
                    read_only: library.read_only,
                    family,
                    pins,
                    form,
                    template,
                    status,
                    definition,
                    diagnostics,
                });
            }
        }
    }
    rows.sort_by(|left, right| {
        left.reference
            .library
            .to_ascii_lowercase()
            .cmp(&right.reference.library.to_ascii_lowercase())
            .then_with(|| {
                left.reference
                    .cell
                    .to_ascii_lowercase()
                    .cmp(&right.reference.cell.to_ascii_lowercase())
            })
    });
    rows
}

fn symbol_detail(ui: &mut Ui, app: &mut RSpiceApp, rows: &[SymbolRow]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_symbol
        .as_deref()
        .and_then(|key| rows.iter().find(|row| symbol_key(&row.reference) == key))
        .cloned()
        .or_else(|| rows.first().cloned());
    let Some(row) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_symbol = Some(symbol_key(&row.reference));
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(format!(
                "{}/{}/{}",
                row.reference.library, row.reference.cell, row.reference.view
            ))
            .monospace()
            .strong(),
        );
        if row.read_only {
            ui.label(
                RichText::new("technology-owned · read-only")
                    .small()
                    .color(Tokens::get(ui.ctx()).color.info),
            );
        }
        if ui
            .button(if row.read_only {
                "Author a variant…"
            } else {
                "Open symbol editor"
            })
            .clicked()
        {
            if row.read_only {
                super::open_create_model_bound_symbol_dialog(&mut app.state);
            } else {
                app.state.open_workspace_view(row.reference.clone());
                app.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Design);
            }
        }
        if ui
            .add_enabled(!row.read_only, egui::Button::new("Edit form…"))
            .on_disabled_hover_text(
                "Technology symbols must be copied into the project before editing.",
            )
            .clicked()
        {
            app.state.library_manager.select_view(
                &row.reference.library,
                &row.reference.cell,
                &row.reference.view,
            );
            super::open_symbol_parameter_form_dialog(&mut app.state);
        }
    });
    if !row.diagnostics.is_empty() {
        card(ui, |ui| {
            card_title(ui, "BLOCKING CONTRACT FINDINGS", Some(&row.status));
            for diagnostic in &row.diagnostics {
                ui.label(
                    RichText::new(format!("⚠ {diagnostic}"))
                        .small()
                        .color(Tokens::get(ui.ctx()).color.err),
                );
            }
        });
    }
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(
                ui,
                "PIN PROVIDER MAP",
                Some(&format!("{} pins", row.pins.len())),
            );
            if let Some(definition) = &row.definition {
                for (index, pin) in definition.pins.iter().enumerate() {
                    property(
                        ui,
                        &format!("{:02} {}", index + 1, pin.name),
                        &format!("{:?}", pin.direction),
                        "netlist order",
                    );
                }
            } else {
                for (index, pin) in row.pins.iter().enumerate() {
                    property(ui, &format!("{:02}", index + 1), pin, "legacy symbol");
                }
            }
        });
        card(&mut columns[1], |ui| {
            card_title(ui, "NETLIST CONTRACT", Some(&row.family));
            property(ui, "Template", &row.template, "validated token grammar");
            if let Some(definition) = &row.definition {
                property(
                    ui,
                    "Prefix",
                    &definition.netlist.device_prefix,
                    "instance reference",
                );
                property(
                    ui,
                    "Parameters",
                    &definition.netlist.parameter_order.join(" "),
                    "emission order",
                );
                property(
                    ui,
                    "Revision",
                    &definition.identity.revision.to_string(),
                    &definition.identity.binding_id,
                );
            }
        });
    });
    card(ui, |ui| {
        let field_count = row
            .definition
            .as_ref()
            .map_or(0, |definition| definition.parameter_form.fields().count());
        card_title(
            ui,
            "CDF / COMPONENT FORM",
            Some(&format!("{field_count} typed fields")),
        );
        if let Some(definition) = &row.definition {
            if field_count == 0 {
                empty_state(
                    ui,
                    "No component-form fields are declared.",
                    "The executable template emits no editable instance parameters.",
                );
            }
            for section in &definition.parameter_form.sections {
                ui.label(RichText::new(&section.label).strong());
                for field in &section.fields {
                    property(
                        ui,
                        &field.key,
                        &format!(
                            "{:?} · {:?} · {}",
                            field.property_type,
                            field.visibility,
                            field.unit.as_deref().unwrap_or("unitless")
                        ),
                        if field.required {
                            "required"
                        } else {
                            "optional"
                        },
                    );
                }
            }
        } else {
            empty_state(
                ui,
                "This legacy symbol has no typed component form.",
                "Open Form designer to publish an explicit parameter contract.",
            );
        }
    });
}

fn corners_page(ui: &mut Ui, app: &mut RSpiceApp) {
    let rows = corner_rows(app);
    let unresolved = rows.iter().filter(|row| !row.resolved).count();
    section_title(
        ui,
        "Corners & sections",
        &format!(
            "{} bindings · {} unresolved · fail closed before run expansion",
            rows.len(),
            unresolved
        ),
        |ui| {
            if ui.button("Import section map").clicked() {
                Command::PdkSettings.execute(app);
            }
            if ui.button("Add corner…").clicked() {
                if let Some(library) = app
                    .state
                    .model_library_manager
                    .selected_library
                    .clone()
                    .or_else(|| {
                        app.state
                            .model_library_manager
                            .libraries_sorted()
                            .first()
                            .map(|library| library.name.clone())
                    })
                {
                    app.state.workbench.models_view.dialog =
                        Some(ModelsWorkbenchDialog::AddCorner {
                            library,
                            name: String::new(),
                            temperature_c: "27".to_owned(),
                            supply_factor: "1.0".to_owned(),
                        });
                } else {
                    receipt(
                        app,
                        Err("Attach a model library before adding a corner.".to_owned()),
                    );
                }
            }
            if ui.button("Validate bindings").clicked() {
                receipt(
                    app,
                    if unresolved == 0 {
                        Ok(format!(
                            "Validated all {} process-corner bindings.",
                            rows.len()
                        ))
                    } else {
                        Err(format!(
                            "Corner validation found {unresolved} bindings without an exact source section."
                        ))
                    },
                );
            }
        },
    );
    card(ui, |ui| {
        card_title(ui, "PACKAGE CANDIDATE", Some("migration review"));
        ui.label(
            RichText::new(
                "No unreviewed technology-package candidate is retained. Importing a new section map creates a transactional review before bindings change.",
            )
            .small()
            .color(Tokens::get(ui.ctx()).color.text_dim),
        );
    });
    let table_h = (ui.available_height() * 0.45).clamp(180.0, 340.0);
    card(ui, |ui| {
        table_header(
            ui,
            &[
                ("CORNER", 0.12),
                ("MOS", 0.13),
                ("BJT", 0.11),
                ("PASSIVES", 0.13),
                ("STAT", 0.10),
                ("AGING", 0.10),
                ("TEMP", 0.13),
                ("STATUS", 0.18),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-corner-matrix")
            .max_height(table_h)
            .show(ui, |ui| {
                if rows.is_empty() {
                    empty_state(
                        ui,
                        "No process-corner bindings are present.",
                        "Import a PDK section map or attach a sectioned model library.",
                    );
                }
                for row in &rows {
                    let selected = app.state.workbench.models_view.selected_corner.as_deref()
                        == Some(row.key.as_str());
                    if selectable_data_row(
                        ui,
                        selected,
                        &[
                            (&row.corner.name.to_uppercase(), 0.12, true),
                            (
                                &format!("{}/{}", row.corner.nmos_corner, row.corner.pmos_corner),
                                0.13,
                                true,
                            ),
                            (if row.resolved { "section" } else { "—" }, 0.11, true),
                            (if row.resolved { "section" } else { "—" }, 0.13, true),
                            (if row.has_statistics { "bound" } else { "—" }, 0.10, true),
                            (if row.has_aging { "evidence" } else { "—" }, 0.10, true),
                            (&format!("{:.1} °C", row.corner.temperature), 0.13, true),
                            (
                                if row.resolved {
                                    "resolved"
                                } else {
                                    "unresolved"
                                },
                                0.18,
                                true,
                            ),
                        ],
                    )
                    .clicked()
                    {
                        app.state.workbench.models_view.selected_corner = Some(row.key.clone());
                        select_corner(app, &row.library, &row.corner.name);
                    }
                }
            });
    });
    corner_detail(ui, app, &rows);
}

#[derive(Clone)]
struct CornerRow {
    key: String,
    library: String,
    corner: ProcessCorner,
    resolved: bool,
    has_statistics: bool,
    has_aging: bool,
    source: Option<String>,
}

fn corner_rows(app: &RSpiceApp) -> Vec<CornerRow> {
    let mut rows = Vec::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        let has_statistics = library
            .model_definition_metadata
            .values()
            .any(|metadata| !metadata.statistics.variables.is_empty());
        let has_aging = library
            .model_qualification
            .values()
            .any(|qualification| !qualification.evidence.is_empty());
        for corner in library.corners.values() {
            let source = corner
                .file_path
                .as_deref()
                .or(library.root_path.as_deref())
                .map(|path| path.display().to_string());
            let retained_section = library.source_contents.iter().any(|content| {
                String::from_utf8_lossy(&content.bytes)
                    .to_ascii_lowercase()
                    .contains(&format!(".lib {}", corner.name.to_ascii_lowercase()))
            });
            let resolved = source.is_some()
                && (retained_section
                    || (corner.name.eq_ignore_ascii_case("tt") && library.corners.len() == 1));
            rows.push(CornerRow {
                key: format!("{}\u{1f}{}", library.name, corner.name),
                library: library.name.clone(),
                corner: corner.clone(),
                resolved,
                has_statistics,
                has_aging,
                source,
            });
        }
    }
    rows.sort_by(|left, right| {
        left.corner
            .name
            .to_ascii_lowercase()
            .cmp(&right.corner.name.to_ascii_lowercase())
            .then_with(|| left.library.cmp(&right.library))
    });
    rows
}

fn corner_detail(ui: &mut Ui, app: &mut RSpiceApp, rows: &[CornerRow]) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_corner
        .as_deref()
        .and_then(|key| rows.iter().find(|row| row.key == key))
        .cloned()
        .or_else(|| rows.first().cloned());
    let Some(row) = selected else {
        return;
    };
    app.state.workbench.models_view.selected_corner = Some(row.key.clone());
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(format!(
                "{} / {}",
                row.library,
                row.corner.name.to_uppercase()
            ))
            .monospace()
            .strong(),
        );
        if !row.resolved {
            ui.label(
                RichText::new("unresolved · run expansion blocked")
                    .small()
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
        if ui
            .add_enabled(row.source.is_some(), egui::Button::new("Open source"))
            .clicked()
            && let Some(library) = app
                .state
                .model_library_manager
                .get_library(&row.library)
                .cloned()
        {
            if let Some(model) = library.models.values().next().cloned() {
                open_model_source(app, &library, &model);
            }
        }
        if ui.button("View include graph").clicked() {
            app.state.workbench.models_page = ModelsPage::Include;
        }
        if ui.button("Model editor…").clicked() {
            Command::ModelEditor.execute(app);
        }
    });
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(ui, "SECTION BINDING", Some(&row.library));
            property(ui, "NMOS", &row.corner.nmos_corner, "exact section axis");
            property(ui, "PMOS", &row.corner.pmos_corner, "exact section axis");
            property(
                ui,
                "Source",
                row.source.as_deref().unwrap_or("not bound"),
                if row.resolved {
                    "retained"
                } else {
                    "unresolved"
                },
            );
            property(
                ui,
                "Supply factor",
                &format!("{:.6}", row.corner.vdd_factor),
                "environment axis",
            );
            property(
                ui,
                "Temperature",
                &format!("{:.3} °C", row.corner.temperature),
                "environment axis",
            );
        });
        card(&mut columns[1], |ui| {
            card_title(ui, "STATISTICAL & AGING", Some("evidence projection"));
            property(
                ui,
                "Statistical variables",
                if row.has_statistics {
                    "declared"
                } else {
                    "none"
                },
                "model schema",
            );
            property(
                ui,
                "Aging evidence",
                if row.has_aging { "retained" } else { "none" },
                "qualification",
            );
            property(
                ui,
                "Binding policy",
                if row.resolved {
                    "executable"
                } else {
                    "fail closed"
                },
                "run expansion",
            );
        });
    });
}

fn select_corner(app: &mut RSpiceApp, library_name: &str, corner_name: &str) {
    let Some(library) = app
        .state
        .model_library_manager
        .get_library(library_name)
        .cloned()
    else {
        receipt(
            app,
            Err(format!("Library '{library_name}' no longer exists.")),
        );
        return;
    };
    app.state.model_library_manager.select_library(library_name);
    if let Some(root) = library.root_path {
        let mut candidate = app.state.model_library_manager.clone();
        let result = candidate
            .load_library_file(&root, Some(corner_name))
            .and_then(|loaded| {
                if loaded != library_name {
                    return Err(format!(
                        "Corner selection resolved library '{loaded}' instead of '{library_name}'."
                    ));
                }
                publish_model_library_candidate(
                    &mut app.state,
                    candidate,
                    library_name,
                    format!("select model corner {corner_name}"),
                )
                .map(|revision| {
                    format!(
                        "Selected exact section '{corner_name}' for '{library_name}' at revision {}.",
                        revision.get()
                    )
                })
            });
        receipt(app, result);
    } else if let Some(library) = app
        .state
        .model_library_manager
        .get_library_mut(library_name)
    {
        library.select_corner(corner_name);
    }
}

fn symbol_key(reference: &CellViewRef) -> String {
    format!(
        "{}\u{1f}{}\u{1f}{}",
        reference.library, reference.cell, reference.view
    )
}

fn bins_page(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut families = BTreeMap::<String, Vec<(String, DeviceModel)>>::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            if model.l_min.is_some()
                || model.l_max.is_some()
                || model.w_min.is_some()
                || model.w_max.is_some()
            {
                families
                    .entry(model.model_type.display_name().to_owned())
                    .or_default()
                    .push((library.name.clone(), model.clone()));
            }
        }
    }
    let findings = geometry_findings(&families);
    section_title(
        ui,
        "Bins & geometry",
        &format!(
            "{} binned families · {} cards · {} findings",
            families.len(),
            families.values().map(Vec::len).sum::<usize>(),
            findings.len()
        ),
        |ui| {
            if ui.button("Import bin map").clicked() {
                Command::PdkSettings.execute(app);
            }
            if ui.button("Edit cards…").clicked() {
                Command::ModelEditor.execute(app);
            }
            if ui.button("Audit all families").clicked() {
                receipt(
                    app,
                    if findings.is_empty() {
                        Ok(
                            "Geometry audit completed with no overlapping card envelopes."
                                .to_owned(),
                        )
                    } else {
                        Err(format!(
                            "Geometry audit found {} overlapping or incomplete envelopes.",
                            findings.len()
                        ))
                    },
                );
            }
            if ui
                .add_enabled(
                    exactly_one_selected_component(app).is_some(),
                    egui::Button::new("Trace schematic"),
                )
                .clicked()
            {
                let model = app
                    .state
                    .workbench
                    .selected_model
                    .clone()
                    .unwrap_or_else(|| "selected geometry family".to_owned());
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::BindingTrace {
                        consumers: model_consumers(app, &model),
                        model,
                    });
            }
        },
    );
    if families.is_empty() {
        empty_state(
            ui,
            "No loaded model publishes a geometry envelope.",
            "Attach a binned PDK library or author L/W bounds in Model Editor.",
        );
        return;
    }
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(ui, "BIN FAMILIES", Some("live loaded models"));
            for (family, cards) in &families {
                if ui
                    .selectable_label(
                        app.state
                            .workbench
                            .models_view
                            .selected_bin_family
                            .as_deref()
                            == Some(family.as_str()),
                        format!("{family}  ·  {} cards", cards.len()),
                    )
                    .clicked()
                {
                    app.state.workbench.models_view.selected_bin_family = Some(family.clone());
                }
            }
        });
        card(&mut columns[1], |ui| {
            card_title(ui, "AUDIT FINDINGS", Some("fail closed on overlap"));
            if findings.is_empty() {
                ui.label(
                    RichText::new("✓ Every declared envelope is non-overlapping.")
                        .color(Tokens::get(ui.ctx()).color.ok),
                );
            } else {
                for finding in findings.iter().take(10) {
                    ui.label(
                        RichText::new(format!("⚠ {finding}"))
                            .small()
                            .color(Tokens::get(ui.ctx()).color.err),
                    );
                }
            }
        });
    });
    let selected = app
        .state
        .workbench
        .models_view
        .selected_bin_family
        .clone()
        .or_else(|| families.keys().next().cloned());
    if let Some(selected) = selected {
        app.state.workbench.models_view.selected_bin_family = Some(selected.clone());
        if let Some(cards) = families.get(&selected) {
            geometry_map(ui, &selected, cards);
            geometry_instance_table(ui, app, cards);
        }
    }
}

fn geometry_findings(families: &BTreeMap<String, Vec<(String, DeviceModel)>>) -> Vec<String> {
    let mut findings = Vec::new();
    for (family, cards) in families {
        for (index, (_, left)) in cards.iter().enumerate() {
            if model_geometry_invalid(left) {
                findings.push(format!(
                    "{family}/{} has an inverted L/W envelope",
                    left.name
                ));
            }
            for (_, right) in cards.iter().skip(index + 1) {
                if envelopes_overlap(left, right) {
                    findings.push(format!("{family}/{} overlaps {}", left.name, right.name));
                }
            }
        }
    }
    findings
}

fn envelopes_overlap(left: &DeviceModel, right: &DeviceModel) -> bool {
    let (Some(ll), Some(lh), Some(lw), Some(wh)) = (left.l_min, left.l_max, left.w_min, left.w_max)
    else {
        return false;
    };
    let (Some(rl), Some(rh), Some(rw), Some(rwh)) =
        (right.l_min, right.l_max, right.w_min, right.w_max)
    else {
        return false;
    };
    ll < rh && rl < lh && lw < rwh && rw < wh
}

fn geometry_map(ui: &mut Ui, family: &str, cards: &[(String, DeviceModel)]) {
    card(ui, |ui| {
        card_title(ui, "LOG L/W MAP", Some(family));
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 210.0), Sense::hover());
        let t = Tokens::get(ui.ctx());
        ui.painter().rect(
            rect,
            2.0,
            t.color.bg_inset,
            Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        let plot = rect.shrink(20.0);
        ui.painter().line_segment(
            [plot.left_bottom(), plot.right_bottom()],
            Stroke::new(1.0, t.color.border_strong),
        );
        ui.painter().line_segment(
            [plot.left_bottom(), plot.left_top()],
            Stroke::new(1.0, t.color.border_strong),
        );
        let finite = cards
            .iter()
            .filter_map(|(_, model)| {
                Some((
                    model.l_min?.max(f64::MIN_POSITIVE).log10(),
                    model.l_max?.max(f64::MIN_POSITIVE).log10(),
                    model.w_min?.max(f64::MIN_POSITIVE).log10(),
                    model.w_max?.max(f64::MIN_POSITIVE).log10(),
                    model.name.as_str(),
                ))
            })
            .collect::<Vec<_>>();
        let min_l = finite
            .iter()
            .map(|value| value.0)
            .reduce(f64::min)
            .unwrap_or(-9.0);
        let max_l = finite
            .iter()
            .map(|value| value.1)
            .reduce(f64::max)
            .unwrap_or(-3.0);
        let min_w = finite
            .iter()
            .map(|value| value.2)
            .reduce(f64::min)
            .unwrap_or(-9.0);
        let max_w = finite
            .iter()
            .map(|value| value.3)
            .reduce(f64::max)
            .unwrap_or(-3.0);
        let map_x = |value: f64| {
            plot.left() + plot.width() * ((value - min_l) / (max_l - min_l).max(1e-12)) as f32
        };
        let map_y = |value: f64| {
            plot.bottom() - plot.height() * ((value - min_w) / (max_w - min_w).max(1e-12)) as f32
        };
        for (index, (l0, l1, w0, w1, name)) in finite.iter().enumerate() {
            let bin = egui::Rect::from_min_max(
                egui::pos2(map_x(*l0), map_y(*w1)),
                egui::pos2(map_x(*l1), map_y(*w0)),
            );
            let color = [
                t.color.accent,
                t.color.info,
                t.color.ok,
                t.color.warn,
                t.color.text_dim,
            ][index % 5];
            ui.painter().rect(
                bin,
                1.0,
                color.linear_multiply(0.12),
                Stroke::new(1.2, color),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                bin.center(),
                egui::Align2::CENTER_CENTER,
                *name,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                color,
            );
        }
    });
}

fn geometry_instance_table(ui: &mut Ui, app: &RSpiceApp, cards: &[(String, DeviceModel)]) {
    card(ui, |ui| {
        card_title(ui, "INSTANCE RESOLUTION", Some("active schematic"));
        let Some(schematic) = app.state.workspace.active_schematic() else {
            empty_state(
                ui,
                "No active schematic is available.",
                "Open a project schematic to trace placed geometry.",
            );
            return;
        };
        let names = cards
            .iter()
            .map(|(_, model)| model.name.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let matches = schematic
            .components
            .iter()
            .filter(|component| {
                explicit_component_model(component)
                    .is_some_and(|model| names.contains(&model.to_ascii_lowercase()))
            })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            empty_state(
                ui,
                "No placed instance resolves to this family.",
                "Binding resolution is derived from the active schematic, not fixture counts.",
            );
        }
        for component in matches {
            property(
                ui,
                &component.name,
                explicit_component_model(component)
                    .as_deref()
                    .unwrap_or("unbound"),
                &component.params,
            );
        }
    });
}

fn include_page(ui: &mut Ui, app: &mut RSpiceApp) {
    let diagnostics = super::include_diagnostics(app);
    section_title(
        ui,
        "Model include graph",
        &format!(
            "{} files · {} edges · {} diagnostics",
            diagnostics.files,
            diagnostics.edges,
            diagnostics.unpinned_roots + diagnostics.cyclic_nodes
        ),
        |ui| {
            if ui.button("Resolve drift…").clicked() {
                if let Some(library) = app.state.model_library_manager.current_library().cloned() {
                    refresh_library(app, &library);
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
    let libraries = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    if libraries.is_empty() {
        empty_state(
            ui,
            "No model-source closure is loaded.",
            "Add a library or attach a pack to build an authenticated include graph.",
        );
        return;
    }
    ui.columns(2, |columns| {
        card(&mut columns[0], |ui| {
            card_title(
                ui,
                "ORDERED CLOSURE",
                Some("root plus retained dependencies"),
            );
            ScrollArea::vertical()
                .id_salt("models-include-nodes")
                .max_height(270.0)
                .show(ui, |ui| {
                    for library in &libraries {
                        ui.label(
                            RichText::new(format!(
                                "{} · {} sources",
                                library.name,
                                library.source_closure.len()
                            ))
                            .strong(),
                        );
                        for (index, source) in library.source_closure.iter().enumerate() {
                            let label = format!(
                                "{:02}  {}  {}",
                                index + 1,
                                path_label(&source.path),
                                short_digest(&source.digest.to_string())
                            );
                            if ui
                                .selectable_label(
                                    app.state
                                        .workbench
                                        .models_view
                                        .include_selected_source
                                        .as_deref()
                                        == Some(source.path.to_string_lossy().as_ref()),
                                    label,
                                )
                                .clicked()
                            {
                                app.state.workbench.models_view.include_selected_source =
                                    Some(source.path.to_string_lossy().into_owned());
                            }
                        }
                    }
                });
        });
        card(&mut columns[1], |ui| {
            card_title(ui, "RESOLUTION HEALTH", Some("authenticated graph"));
            property(
                ui,
                "Pinned files",
                &diagnostics.files.to_string(),
                "digest retained",
            );
            property(
                ui,
                "Dependency edges",
                &diagnostics.edges.to_string(),
                "owner → target",
            );
            property(
                ui,
                "Unpinned roots",
                &diagnostics.unpinned_roots.to_string(),
                if diagnostics.unpinned_roots == 0 {
                    "clean"
                } else {
                    "blocks execution"
                },
            );
            property(
                ui,
                "Cycle nodes",
                &diagnostics.cyclic_nodes.to_string(),
                if diagnostics.cyclic_nodes == 0 {
                    "clean"
                } else {
                    "blocks execution"
                },
            );
        });
    });
    include_definition_table(ui, app, &libraries);
}

fn include_definition_table(ui: &mut Ui, app: &mut RSpiceApp, libraries: &[ModelLibrary]) {
    let query = app
        .state
        .workbench
        .models_view
        .include_definition_query
        .trim()
        .to_ascii_lowercase();
    let mut providers = BTreeMap::<String, Vec<String>>::new();
    for library in libraries {
        for model in library.models.values() {
            providers
                .entry(model.name.to_ascii_lowercase())
                .or_default()
                .push(library.name.clone());
        }
    }
    card(ui, |ui| {
        card_title(
            ui,
            "DEFINITION RESOLUTION",
            Some(&format!("{} unique names", providers.len())),
        );
        table_header(
            ui,
            &[
                ("DEFINITION", 0.25),
                ("KIND", 0.13),
                ("WINNING PROVIDER", 0.25),
                ("OTHER CANDIDATES", 0.20),
                ("RESOLUTION", 0.17),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-include-definitions")
            .max_height(260.0)
            .show(ui, |ui| {
                let mut shown = 0;
                for (definition, candidates) in &providers {
                    if !query.is_empty()
                        && !definition.contains(&query)
                        && !candidates
                            .iter()
                            .any(|provider| provider.to_ascii_lowercase().contains(&query))
                    {
                        continue;
                    }
                    shown += 1;
                    let contested = candidates.len() > 1;
                    let other_candidates = if contested {
                        (candidates.len() - 1).to_string()
                    } else {
                        "—".to_owned()
                    };
                    let response = selectable_data_row(
                        ui,
                        false,
                        &[
                            (definition, 0.25, true),
                            ("model", 0.13, true),
                            (&candidates[0], 0.25, false),
                            (&other_candidates, 0.20, true),
                            (if contested { "contested" } else { "unique" }, 0.17, true),
                        ],
                    );
                    if contested && response.clicked() {
                        app.state.workbench.models_view.dialog =
                            Some(ModelsWorkbenchDialog::DefinitionConflict {
                                definition: definition.clone(),
                                providers: candidates.clone(),
                            });
                    }
                }
                if shown == 0 {
                    empty_state(
                        ui,
                        "No definitions match.",
                        "The filter searches definition names and every retained provider.",
                    );
                }
            });
    });
}

fn render_dialog(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(dialog) = app.state.workbench.models_view.dialog.clone() else {
        return;
    };
    match dialog {
        ModelsWorkbenchDialog::SourcePreview {
            title,
            subtitle,
            source,
            read_only,
        } => {
            let mut open = true;
            egui::Window::new(title)
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_size(egui::vec2(760.0, 520.0))
                .show(ui.ctx(), |ui| {
                    ui.label(RichText::new(subtitle).monospace().small());
                    ui.separator();
                    let mut body = source;
                    ScrollArea::both().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut body)
                                .font(egui::TextStyle::Monospace)
                                .interactive(!read_only)
                                .desired_width(f32::INFINITY)
                                .desired_rows(26),
                        );
                    });
                    if !read_only {
                        ui.label(
                            "Editable project definitions must be saved through Model Editor so validation and revision history remain atomic.",
                        );
                    }
                });
            if !open {
                app.state.workbench.models_view.dialog = None;
            }
        }
        ModelsWorkbenchDialog::CompareModels {
            left_library,
            left_model,
            right_library,
            right_model,
        } => {
            let mut open = true;
            egui::Window::new("Compare model definitions")
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_size(egui::vec2(760.0, 500.0))
                .show(ui.ctx(), |ui| {
                    compare_models(
                        ui,
                        app,
                        &left_library,
                        &left_model,
                        &right_library,
                        &right_model,
                    );
                });
            if !open {
                app.state.workbench.models_view.dialog = None;
            }
        }
        ModelsWorkbenchDialog::ConfirmPack { pack_id, attach } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new(if attach {
                "Attach model pack"
            } else {
                "Detach model pack"
            })
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.label(if attach {
                    "RSpice will authenticate the pack entry and publish its retained include closure as one undoable project revision."
                } else {
                    "RSpice will remove the attached source as one undoable project revision. Existing instance references may become unresolved."
                });
                ui.label(RichText::new(&pack_id).monospace().strong());
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        decision = Some(false);
                    }
                    if ui
                        .button(if attach { "Attach pack" } else { "Detach pack" })
                        .clicked()
                    {
                        decision = Some(true);
                    }
                });
            });
            if decision == Some(true) {
                if attach {
                    attach_pack(app, &pack_id);
                } else {
                    detach_pack(app, &pack_id);
                }
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            }
        }
        ModelsWorkbenchDialog::ConfirmPart { pack_id, part_name } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new("Add shipped part to project")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "The exact installed source file will be parsed, license-checked, pinned by digest, and published as one undoable project revision.",
                    );
                    ui.label(
                        RichText::new(format!("{part_name} · {pack_id}"))
                            .monospace()
                            .strong(),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            decision = Some(false);
                        }
                        if ui.button("Add to project").clicked() {
                            decision = Some(true);
                        }
                    });
                });
            if decision == Some(true) {
                add_part(app, &pack_id, &part_name);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            }
        }
        ModelsWorkbenchDialog::DefinitionConflict {
            definition,
            providers,
        } => {
            let mut open = true;
            egui::Window::new("Contested model definition")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(format!(
                        "'{definition}' is provided by {} loaded libraries. Primitive SPICE instances cannot safely bind until the duplicate is removed or renamed.",
                        providers.len()
                    ));
                    for provider in &providers {
                        ui.label(RichText::new(provider).monospace());
                    }
                    if ui.button("Open Model Editor").clicked() {
                        Command::ModelEditor.execute(app);
                        app.state.workbench.models_view.dialog = None;
                    }
                });
            if !open {
                app.state.workbench.models_view.dialog = None;
            }
        }
        ModelsWorkbenchDialog::BindingTrace { model, consumers } => {
            let mut open = true;
            egui::Window::new("Model binding trace")
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .show(ui.ctx(), |ui| {
                    ui.label(RichText::new(model).monospace().strong());
                    if consumers.is_empty() {
                        empty_state(
                            ui,
                            "No consumer resolves to this model.",
                            "The trace was derived from the active schematic.",
                        );
                    } else {
                        for consumer in consumers {
                            ui.label(consumer);
                        }
                    }
                });
            if !open {
                app.state.workbench.models_view.dialog = None;
            }
        }
        ModelsWorkbenchDialog::AddCorner {
            library,
            mut name,
            mut temperature_c,
            mut supply_factor,
        } => {
            let mut open = true;
            let mut decision = None;
            egui::Window::new("Add process corner")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "The corner definition is published as one guarded project revision. A source-backed library must still resolve an exact section with the same name before execution.",
                    );
                    ui.label(RichText::new(&library).monospace().strong());
                    ui.horizontal(|ui| {
                        ui.label("Section name");
                        ui.text_edit_singleline(&mut name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Temperature °C");
                        ui.text_edit_singleline(&mut temperature_c);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Supply factor");
                        ui.text_edit_singleline(&mut supply_factor);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            decision = Some(false);
                        }
                        if ui.button("Add corner").clicked() {
                            decision = Some(true);
                        }
                    });
                });
            if decision == Some(true) {
                add_corner(app, &library, &name, &temperature_c, &supply_factor);
                app.state.workbench.models_view.dialog = None;
            } else if decision == Some(false) || !open {
                app.state.workbench.models_view.dialog = None;
                app.state.workbench.models_view.operational_state =
                    ModelsOperationalState::Cancelled;
            } else {
                app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::AddCorner {
                    library,
                    name,
                    temperature_c,
                    supply_factor,
                });
            }
        }
    }
}

fn compare_models(
    ui: &mut Ui,
    app: &RSpiceApp,
    left_library: &str,
    left_model: &str,
    right_library: &str,
    right_model: &str,
) {
    let left = app
        .state
        .model_library_manager
        .get_library(left_library)
        .and_then(|library| library.get_model(left_model));
    let right = app
        .state
        .model_library_manager
        .get_library(right_library)
        .and_then(|library| library.get_model(right_model));
    let (Some(left), Some(right)) = (left, right) else {
        empty_state(
            ui,
            "One comparison definition no longer resolves.",
            "Close this comparison and select live catalog rows again.",
        );
        return;
    };
    ui.label(
        RichText::new(format!(
            "{left_library}/{left_model}  ↔  {right_library}/{right_model}"
        ))
        .monospace(),
    );
    let keys = left
        .parameters
        .keys()
        .chain(right.parameters.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    table_header(
        ui,
        &[
            ("PARAMETER", 0.30),
            ("LEFT", 0.27),
            ("RIGHT", 0.27),
            ("STATE", 0.16),
        ],
    );
    ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
        for key in keys {
            let left_value = left.parameters.get(&key).map(ToString::to_string);
            let right_value = right.parameters.get(&key).map(ToString::to_string);
            let state = if left_value == right_value {
                "same"
            } else if left_value.is_none() {
                "right only"
            } else if right_value.is_none() {
                "left only"
            } else {
                "changed"
            };
            selectable_data_row(
                ui,
                false,
                &[
                    (&key, 0.30, true),
                    (left_value.as_deref().unwrap_or("—"), 0.27, true),
                    (right_value.as_deref().unwrap_or("—"), 0.27, true),
                    (state, 0.16, true),
                ],
            );
        }
    });
}

fn open_model_compare(app: &mut RSpiceApp, left_library: &str, left_model: &str) {
    let right = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library
                .models
                .values()
                .map(move |model| (library.name.clone(), model.name.clone()))
        })
        .find(|(library, model)| library != left_library || model != left_model);
    if let Some((right_library, right_model)) = right {
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::CompareModels {
            left_library: left_library.to_owned(),
            left_model: left_model.to_owned(),
            right_library,
            right_model,
        });
    } else {
        receipt(
            app,
            Err("A comparison requires at least two loaded model definitions.".to_owned()),
        );
    }
}

fn open_model_source(app: &mut RSpiceApp, library: &ModelLibrary, model: &DeviceModel) {
    let content = model
        .file_path
        .as_ref()
        .and_then(|path| {
            library
                .source_contents
                .iter()
                .find(|source| source.path == *path)
        })
        .or_else(|| library.source_contents.first());
    if let Some(content) = content {
        app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::SourcePreview {
            title: model.name.clone(),
            subtitle: format!(
                "{} · {}",
                content.path.display(),
                match library.source_authority {
                    ModelSourceAuthority::ProjectOwned { .. } => "project revision",
                    ModelSourceAuthority::External => "pinned external bytes",
                    ModelSourceAuthority::BuiltIn => "built-in",
                }
            ),
            source: String::from_utf8_lossy(&content.bytes).into_owned(),
            read_only: !library.source_authority.is_project_owned(),
        });
    } else if let Some(path) = model.file_path.as_ref().or(library.root_path.as_ref()) {
        match std::fs::read_to_string(path) {
            Ok(source) => {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::SourcePreview {
                        title: model.name.clone(),
                        subtitle: format!("{} · live unpinned source", path.display()),
                        source,
                        read_only: true,
                    });
            }
            Err(error) => receipt(
                app,
                Err(format!(
                    "Could not read model source '{}': {error}",
                    path.display()
                )),
            ),
        }
    }
}

fn refresh_library(app: &mut RSpiceApp, library: &ModelLibrary) {
    let Some(root) = library.root_path.clone() else {
        receipt(
            app,
            Err(format!(
                "Library '{}' has no external root to refresh.",
                library.name
            )),
        );
        return;
    };
    let mut candidate = app.state.model_library_manager.clone();
    let section = library.selected_corner.as_deref();
    let result = candidate
        .load_library_file(&root, section)
        .and_then(|loaded| {
            if loaded != library.name {
                return Err(format!(
                    "Refresh resolved library '{loaded}' instead of expected '{}'.",
                    library.name
                ));
            }
            publish_model_library_candidate(
                &mut app.state,
                candidate,
                &library.name,
                format!("refresh model library {}", library.name),
            )
            .map(|revision| {
                format!(
                    "Refreshed and pinned '{}' at project revision {}.",
                    library.name,
                    revision.get()
                )
            })
        });
    receipt(app, result);
}

fn attach_pack(app: &mut RSpiceApp, pack_id: &str) {
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate.attach_spice_pack(pack_id).and_then(|library| {
        publish_model_library_candidate(
            &mut app.state,
            candidate,
            &library,
            format!("attach model pack {pack_id}"),
        )
        .map(|revision| {
            format!(
                "Attached pack '{pack_id}' as library '{library}' at project revision {}.",
                revision.get()
            )
        })
    });
    receipt(app, result);
}

fn detach_pack(app: &mut RSpiceApp, pack_id: &str) {
    let Some(library) = attached_library_for_pack(app, pack_id) else {
        receipt(
            app,
            Err(format!("Pack '{pack_id}' is not attached to this project.")),
        );
        return;
    };
    let mut candidate = app.state.model_library_manager.clone();
    candidate.remove_library(&library);
    let result = publish_model_library_candidate(
        &mut app.state,
        candidate,
        &library,
        format!("detach model pack {pack_id}"),
    )
    .map(|revision| {
        format!(
            "Detached pack '{pack_id}' from library '{library}' at project revision {}.",
            revision.get()
        )
    });
    receipt(app, result);
}

fn add_part(app: &mut RSpiceApp, pack_id: &str, part_name: &str) {
    let mut candidate = app.state.model_library_manager.clone();
    let result = candidate
        .add_spice_part(pack_id, part_name)
        .and_then(|library| {
            if app
                .state
                .model_library_manager
                .get_library(&library)
                .is_some()
            {
                return Ok(format!(
                    "Part '{part_name}' is already available through library '{library}'."
                ));
            }
            publish_model_library_candidate(
                &mut app.state,
                candidate,
                &library,
                format!("add shipped model part {part_name}"),
            )
            .map(|revision| {
                format!(
                    "Added '{part_name}' from pack '{pack_id}' at project revision {}.",
                    revision.get()
                )
            })
        });
    receipt(app, result);
}

fn add_corner(
    app: &mut RSpiceApp,
    library_name: &str,
    name: &str,
    temperature_c: &str,
    supply_factor: &str,
) {
    let name = name.trim();
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        receipt(
            app,
            Err("Corner name must use ASCII letters, digits, or underscore.".to_owned()),
        );
        return;
    }
    let temperature = match temperature_c.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && (-273.15..=1000.0).contains(&value) => value,
        _ => {
            receipt(
                app,
                Err("Corner temperature must be finite and at least absolute zero.".to_owned()),
            );
            return;
        }
    };
    let supply = match supply_factor.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && value > 0.0 && value <= 10.0 => value,
        _ => {
            receipt(
                app,
                Err(
                    "Supply factor must be a finite value greater than 0 and no more than 10."
                        .to_owned(),
                ),
            );
            return;
        }
    };
    let Some(source) = app
        .state
        .model_library_manager
        .get_library(library_name)
        .cloned()
    else {
        receipt(
            app,
            Err(format!("Library '{library_name}' no longer exists.")),
        );
        return;
    };
    if source
        .corners
        .keys()
        .any(|existing| existing.eq_ignore_ascii_case(name))
    {
        receipt(
            app,
            Err(format!(
                "Corner '{name}' already exists in library '{library_name}'."
            )),
        );
        return;
    }
    let mut candidate = app.state.model_library_manager.clone();
    let library = candidate
        .get_library_mut(library_name)
        .expect("the source library was resolved above");
    let mut corner = ProcessCorner::new(name);
    corner.description = format!("Project corner {name}");
    corner.nmos_corner = name.to_owned();
    corner.pmos_corner = name.to_owned();
    corner.temperature = temperature;
    corner.vdd_factor = supply;
    library.corners.insert(name.to_owned(), corner);
    let result = publish_model_library_candidate(
        &mut app.state,
        candidate,
        library_name,
        format!("add model corner {name}"),
    )
    .map(|revision| {
        format!(
            "Added corner '{name}' to '{library_name}' at project revision {}.",
            revision.get()
        )
    });
    receipt(app, result);
}

fn attached_library_for_pack(app: &RSpiceApp, pack_id: &str) -> Option<String> {
    let index = app.state.model_library_manager.spice_packs()?;
    let pack = index.pack(pack_id)?;
    let directory = index.root().join(&pack.path);
    app.state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| {
            library
                .root_path
                .as_deref()
                .is_some_and(|root| root.starts_with(&directory))
        })
        .map(|library| library.name.clone())
}

fn export_include_manifest(app: &mut RSpiceApp) {
    let manifest = build_include_manifest(app);
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(path) = rfd::FileDialog::new()
            .set_file_name("rspice-model-closure.json")
            .add_filter("JSON", &["json"])
            .save_file()
        else {
            return;
        };
        let result = std::fs::write(&path, manifest.as_bytes())
            .map(|()| format!("Exported model closure manifest to '{}'.", path.display()))
            .map_err(|error| format!("Could not export '{}': {error}", path.display()));
        receipt(app, result);
    }
    #[cfg(target_arch = "wasm32")]
    {
        let result = crate::workbench::browser::download::download_bytes_file(
            Path::new("rspice-model-closure.json"),
            manifest.as_bytes(),
            "application/json;charset=utf-8",
        )
        .map(|()| "Handed the model closure manifest to the browser download manager.".to_owned())
        .map_err(|error| format!("Could not start the model closure download: {error}"));
        receipt(app, result);
    }
}

fn build_include_manifest(app: &RSpiceApp) -> String {
    let libraries = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            serde_json::json!({
                "library": library.name,
                "authority": match library.source_authority {
                    ModelSourceAuthority::BuiltIn => "built-in",
                    ModelSourceAuthority::External => "external",
                    ModelSourceAuthority::ProjectOwned { .. } => "project-owned",
                },
                "root": library.root_path.as_ref().map(|path| path.to_string_lossy()),
                "selected_section": library.selected_corner,
                "sources": library.source_closure.iter().map(|source| serde_json::json!({
                    "path": source.path.to_string_lossy(),
                    "digest": source.digest.to_string(),
                })).collect::<Vec<_>>(),
                "edges": library.source_edges.iter().map(|edge| serde_json::json!({
                    "owner": edge.owner.to_string_lossy(),
                    "requested": edge.requested_path,
                    "target": edge.target.to_string_lossy(),
                })).collect::<Vec<_>>(),
            })
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&serde_json::json!({
        "schema": 1,
        "project_revision": app.state.workspace.project.revision(),
        "libraries": libraries,
    }))
    .unwrap_or_else(|error| format!("{{\"error\":\"{error}\"}}"))
}

fn model_consumers(app: &RSpiceApp, model_name: &str) -> Vec<String> {
    let Some(schematic) = app.state.workspace.active_schematic() else {
        return Vec::new();
    };
    let mut consumers = schematic
        .components
        .iter()
        .filter(|component| component_uses_model(component, model_name))
        .map(|component| {
            format!(
                "{} · {} · ({}, {})",
                component.name,
                component.kind.display_name(),
                component.pos.x,
                component.pos.y
            )
        })
        .collect::<Vec<_>>();
    consumers.sort();
    consumers
}

fn component_uses_model(component: &Component, model_name: &str) -> bool {
    component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.module_name.as_deref())
        .is_some_and(|model| model.eq_ignore_ascii_case(model_name))
        || explicit_component_model(component)
            .is_some_and(|model| model.eq_ignore_ascii_case(model_name))
}

fn explicit_component_model(component: &Component) -> Option<String> {
    let params = crate::state::parse_params_string(&component.params);
    let parameter = params
        .get("model")
        .map(String::as_str)
        .map(str::trim)
        .filter(|model| !model.is_empty());
    let value = component.value.trim();
    match component.kind {
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
        | ComponentType::CoupledTransmissionLine => parameter
            .or((!value.is_empty()).then_some(value))
            .map(str::to_owned),
        ComponentType::SaturableInductor => parameter.map(str::to_owned),
        _ => None,
    }
}

fn exactly_one_selected_component(app: &RSpiceApp) -> Option<u64> {
    let selection = &app.state.schematic.selection.components;
    (selection.len() == 1).then(|| *selection.iter().next().expect("one selected component"))
}

fn model_geometry_invalid(model: &DeviceModel) -> bool {
    model
        .l_min
        .zip(model.l_max)
        .is_some_and(|(min, max)| min > max)
        || model
            .w_min
            .zip(model.w_max)
            .is_some_and(|(min, max)| min > max)
}

fn path_label(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

fn short_digest(digest: &str) -> String {
    digest.chars().take(12).collect()
}

fn part_key(hit: &PackModelHit) -> String {
    format!("{}\u{1f}{}\u{1f}{}", hit.pack, hit.name, hit.line)
}

fn receipt(app: &mut RSpiceApp, result: Result<String, String>) {
    if let Err(message) = &result {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(message.clone()));
    }
    app.state.workbench.models_view.operational_state = match &result {
        Ok(message) if message.to_ascii_lowercase().contains("recover") => {
            ModelsOperationalState::Recovered
        }
        Ok(_) => ModelsOperationalState::Ready,
        Err(message) => ModelsOperationalState::from_failure(message),
    };
    app.state.workbench.models_view.action_receipt = Some(result);
}

fn navigate_specialist(app: &mut RSpiceApp, surface: crate::workbench::SurfaceId) {
    if let Err(error) = app.state.workbench.navigate(
        crate::workbench::SurfaceRoute::surface(surface),
        crate::workbench::RouteTransitionSource::User,
    ) {
        receipt(
            app,
            Err(format!("Cannot open {}: {error}", surface.label())),
        );
    }
}

fn section_title(ui: &mut Ui, title: &str, subtitle: &str, actions: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(9, 7))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(title)
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(RichText::new(subtitle).small().color(t.color.text_dim));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), actions);
            });
        });
}

fn card(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(7))
        .show(ui, content);
}

fn card_title(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text_dim),
        );
        if let Some(meta) = meta {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(meta)
                        .small()
                        .monospace()
                        .color(t.color.text_faint),
                );
            });
        }
    });
    ui.separator();
}

fn property(ui: &mut Ui, name: &str, value: &str, origin: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.label(RichText::new(name).small().color(t.color.text_dim));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(origin).small().color(t.color.text_faint));
            ui.label(RichText::new(value).small().monospace().color(t.color.text));
        });
    });
}

fn empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(title).strong().color(t.color.text_dim));
        ui.label(RichText::new(detail).small().color(t.color.text_faint));
    });
    ui.add_space(10.0);
}

fn table_header(ui: &mut Ui, columns: &[(&str, f32)]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    let mut x = rect.left() + 5.0;
    for (label, fraction) in columns {
        let width = rect.width() * fraction;
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            *label,
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_faint,
        );
        x += width;
    }
}

fn selectable_data_row(
    ui: &mut Ui,
    selected: bool,
    columns: &[(&str, f32, bool)],
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_H), Sense::click());
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, t.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    paint_columns(ui, rect, columns);
    response
}

fn paint_columns(ui: &Ui, rect: egui::Rect, columns: &[(&str, f32, bool)]) {
    let t = Tokens::get(ui.ctx());
    let mut x = rect.left() + 5.0;
    for (value, fraction, mono) in columns {
        let width = rect.width() * fraction;
        let clipped = elide(ui, value, width - 9.0, *mono);
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            clipped,
            if *mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            t.color.text_dim,
        );
        x += width;
    }
}

fn elide(ui: &Ui, value: &str, max_width: f32, mono: bool) -> String {
    let font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    if ui
        .painter()
        .layout_no_wrap(value.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return value.to_owned();
    }
    let mut output = value.to_owned();
    while output.chars().count() > 1 {
        output.pop();
        let candidate = format!("{output}…");
        if ui
            .painter()
            .layout_no_wrap(candidate.clone(), font.clone(), Color32::WHITE)
            .size()
            .x
            <= max_width
        {
            return candidate;
        }
    }
    "…".to_owned()
}
