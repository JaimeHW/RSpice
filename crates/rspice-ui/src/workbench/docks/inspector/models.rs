//! Models inspector and state-backed page summaries.

use egui::{Align2, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::design_system::{
    StatusMark, WorkbenchIcon, property_row, property_row_status,
};
use crate::workbench::{AppState, RSpiceApp};

use super::section_header;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelAvailability {
    ProjectOwned,
    ExternalPinned,
    ExternalUnpinned,
    BuiltIn,
}

impl ModelAvailability {
    const fn label(self) -> &'static str {
        match self {
            Self::ProjectOwned => "Project source",
            Self::ExternalPinned => "Pinned external source",
            Self::ExternalUnpinned => "Source review required",
            Self::BuiltIn => "Catalog metadata",
        }
    }

    const fn detail(self) -> &'static str {
        match self {
            Self::ProjectOwned => "Editable, revision-owned definition",
            Self::ExternalPinned => "Authenticated closure retained",
            Self::ExternalUnpinned => "No retained source closure",
            Self::BuiltIn => "No executable source deck",
        }
    }

    const fn mark(self) -> StatusMark {
        match self {
            Self::ProjectOwned | Self::ExternalPinned => StatusMark::Success,
            Self::ExternalUnpinned => StatusMark::Failure,
            Self::BuiltIn => StatusMark::Neutral,
        }
    }
}

fn model_availability(library: &crate::state::model_library::ModelLibrary) -> ModelAvailability {
    match library.source_authority {
        crate::state::model_library::ModelSourceAuthority::ProjectOwned { .. } => {
            ModelAvailability::ProjectOwned
        }
        crate::state::model_library::ModelSourceAuthority::External
            if !library.source_closure.is_empty()
                && library.source_closure.len() == library.source_contents.len() =>
        {
            ModelAvailability::ExternalPinned
        }
        crate::state::model_library::ModelSourceAuthority::External => {
            ModelAvailability::ExternalUnpinned
        }
        crate::state::model_library::ModelSourceAuthority::BuiltIn => ModelAvailability::BuiltIn,
    }
}

fn model_identity_hero(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let selected = app
        .state
        .model_library_manager
        .current_library()
        .and_then(|library| {
            app.state
                .workbench
                .selected_model
                .as_deref()
                .and_then(|name| library.models.get(name))
                .map(|model| (library, model))
        });
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 88.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    WorkbenchIcon::Models.paint(
        ui.painter(),
        Rect::from_center_size(
            Pos2::new(rect.left() + 28.0, rect.top() + 31.0),
            Vec2::splat(26.0),
        ),
        t.color.accent,
    );
    let text_left = rect.left() + 52.0;
    if let Some((library, model)) = selected {
        let availability = model_availability(library);
        ui.painter().text(
            Pos2::new(text_left, rect.top() + 11.0),
            Align2::LEFT_TOP,
            format!("MODEL / {}", library.name),
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_dim,
        );
        ui.painter().text(
            Pos2::new(text_left, rect.top() + 29.0),
            Align2::LEFT_TOP,
            &model.name,
            theme::mono(tokens::FS_3, FontWeight::SemiBold),
            t.color.text,
        );
        ui.painter().text(
            Pos2::new(text_left, rect.top() + 51.0),
            Align2::LEFT_TOP,
            format!(
                "{} / {}",
                model.model_type.display_name(),
                availability.label()
            ),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            match availability {
                ModelAvailability::ProjectOwned | ModelAvailability::ExternalPinned => t.color.ok,
                ModelAvailability::ExternalUnpinned => t.color.err,
                ModelAvailability::BuiltIn => t.color.info,
            },
        );
    } else {
        ui.painter().text(
            Pos2::new(text_left, rect.top() + 19.0),
            Align2::LEFT_TOP,
            "NO MODEL SELECTED",
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_dim,
        );
        ui.painter().text(
            Pos2::new(text_left, rect.top() + 41.0),
            Align2::LEFT_TOP,
            "Choose a project model to inspect its source contract.",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            selected.map_or("No model selected", |(_, model)| model.name.as_str()),
        )
    });
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.workbench.models_page == crate::workbench::state::ModelsPage::Models {
        match app.state.workbench.models_view.catalog_scope {
            crate::workbench::state::ModelsCatalogScope::InstalledPacks => {
                model_pack_inspector(ui, &mut app.state);
                return;
            }
            crate::workbench::state::ModelsCatalogScope::RSpiceLibrary => {
                shipped_part_inspector(ui, &mut app.state);
                return;
            }
            crate::workbench::state::ModelsCatalogScope::Project => {}
        }
    }
    model_identity_hero(ui, app);
    section_header(ui, "Identity", None);
    property_row(
        ui,
        "Library",
        app.state
            .model_library_manager
            .selected_library
            .as_deref()
            .unwrap_or("None"),
    );
    let selected_library = app.state.model_library_manager.current_library().cloned();
    if let Some(library) = selected_library {
        property_row(
            ui,
            "PDK",
            if library.pdk_name.is_empty() {
                "Unspecified"
            } else {
                &library.pdk_name
            },
        );
        property_row(
            ui,
            "Technology",
            if library.technology_node.is_empty() {
                "Unspecified"
            } else {
                &library.technology_node
            },
        );
        property_row(
            ui,
            "Version",
            if library.version.is_empty() {
                "Unspecified"
            } else {
                &library.version
            },
        );
        property_row(ui, "Models", &library.model_count().to_string());
        property_row(ui, "Corners", &library.corner_count().to_string());
        section_header(ui, "Availability", None);
        let availability = model_availability(&library);
        let t = Tokens::get(ui.ctx());
        property_row_status(
            ui,
            "State",
            availability.label(),
            match availability {
                ModelAvailability::ProjectOwned | ModelAvailability::ExternalPinned => t.color.ok,
                ModelAvailability::ExternalUnpinned => t.color.err,
                ModelAvailability::BuiltIn => t.color.info,
            },
            availability.mark(),
        );
        property_row(ui, "Contract", availability.detail());
        property_row(
            ui,
            "Authority",
            match library.source_authority {
                crate::state::model_library::ModelSourceAuthority::BuiltIn => "RSpice built-in",
                crate::state::model_library::ModelSourceAuthority::External => "External",
                crate::state::model_library::ModelSourceAuthority::ProjectOwned { .. } => {
                    "Project-owned"
                }
            },
        );
        property_row(
            ui,
            "Pinned sources",
            &library.source_closure.len().to_string(),
        );
        property_row(ui, "Include edges", &library.source_edges.len().to_string());
        if let Some(root) = library.root_path.as_deref() {
            property_row(ui, "Source root", &root.display().to_string());
        }
        if let Some(revision) = library.project_source_revision() {
            property_row(ui, "Revision", &revision.get().to_string());
        }
        property_row(
            ui,
            "Selected corner",
            library.selected_corner.as_deref().unwrap_or("None"),
        );
        if let Some(model_name) = &app.state.workbench.selected_model
            && let Some(model) = library.models.get(model_name)
        {
            section_header(ui, "Definition", None);
            property_row(ui, "Name", &model.name);
            property_row(ui, "Type", &format!("{:?}", model.model_type));
            property_row(ui, "Level", &format!("{:?}", model.level));
            property_row(ui, "Parameters", &model.parameters.len().to_string());
            property_row(
                ui,
                "Source",
                &model.file_path.as_deref().map_or_else(
                    || "Not declared".to_owned(),
                    |path| path.display().to_string(),
                ),
            );
            property_row(
                ui,
                "Typed metadata",
                if library.model_definition_metadata.contains_key(model_name) {
                    "Present"
                } else {
                    "Not declared"
                },
            );
            if let Some(qualification) = library.model_qualification.get(model_name) {
                property_row(ui, "Suites", &qualification.suites.len().to_string());
                property_row(ui, "Evidence", &qualification.evidence.len().to_string());
                property_row(ui, "Releases", &qualification.releases.len().to_string());
            } else {
                property_row(ui, "Qualification", "No retained suite");
            }
            if let Some(correlation) = library.model_correlation.get(model_name) {
                property_row(
                    ui,
                    "Correlation suites",
                    &correlation.suites.len().to_string(),
                );
                property_row(
                    ui,
                    "Correlation evidence",
                    &correlation.evidence.len().to_string(),
                );
            }
            if let Some(vdd) = model.vdd {
                property_row(ui, "Nominal VDD", &format!("{vdd:.6} V"));
            }
            if let Some(vth) = model.vth0 {
                property_row(ui, "Threshold", &format!("{vth:.6} V"));
            }
            if let (Some(l_min), Some(l_max)) = (model.l_min, model.l_max) {
                property_row(ui, "Length envelope", &format!("{l_min:.6e}…{l_max:.6e} m"));
            }
            if let (Some(w_min), Some(w_max)) = (model.w_min, model.w_max) {
                property_row(ui, "Width envelope", &format!("{w_min:.6e}…{w_max:.6e} m"));
            }
            section_header(ui, "Actions", None);
            ui.horizontal_wrapped(|ui| {
                if ui.button("Model editor").clicked() {
                    crate::workbench::commands::vocabulary::Command::ModelEditor.execute(app);
                }
                if ui.button("Qualification").clicked() {
                    app.state.workbench.models_page =
                        crate::workbench::state::ModelsPage::Qualification;
                }
                if ui.button("Include graph").clicked() {
                    app.state.workbench.models_page = crate::workbench::state::ModelsPage::Include;
                    app.state.workbench.models_view.include_selected_source = model
                        .file_path
                        .as_ref()
                        .map(|path| path.to_string_lossy().into_owned());
                }
            });
        }
    }
    model_page_summary(ui, app);
}

fn model_page_summary(ui: &mut Ui, app: &RSpiceApp) {
    match app.state.workbench.models_page {
        crate::workbench::state::ModelsPage::Models => {}
        crate::workbench::state::ModelsPage::Symbols => symbol_page_summary(ui, app),
        crate::workbench::state::ModelsPage::Corners => corner_page_summary(ui, app),
        crate::workbench::state::ModelsPage::Bins => bin_page_summary(ui, app),
        crate::workbench::state::ModelsPage::Include => include_page_summary(ui, app),
        crate::workbench::state::ModelsPage::Qualification => {
            qualification_page_summary(ui, app);
        }
    }
}

fn symbol_page_summary(ui: &mut Ui, app: &RSpiceApp) {
    let symbol_count = app
        .state
        .library_manager
        .libraries_sorted()
        .iter()
        .flat_map(|library| library.cells_sorted())
        .flat_map(|cell| cell.views_sorted())
        .filter(|view| view.view_type == crate::state::ViewType::Symbol)
        .count();
    section_header(ui, "Symbol & CDF", Some(&symbol_count.to_string()));
    let Some(key) = app.state.workbench.models_view.selected_symbol.as_deref() else {
        property_row(ui, "Selection", "Choose a symbol row");
        return;
    };
    let mut parts = key.split('\u{1f}');
    let (Some(library_name), Some(cell_name), Some(view_name), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        property_row(ui, "Selection", "Stored symbol identity is invalid");
        return;
    };
    let Some(library) = app.state.library_manager.get_library(library_name) else {
        property_row(ui, "Selection", "Symbol library is no longer loaded");
        return;
    };
    let Some(view) = library
        .get_cell(cell_name)
        .and_then(|cell| cell.get_view(view_name))
        .filter(|view| view.view_type == crate::state::ViewType::Symbol)
    else {
        property_row(ui, "Selection", "Symbol view is no longer loaded");
        return;
    };
    property_row(
        ui,
        "Identity",
        &format!("{library_name}/{cell_name}/{view_name}"),
    );
    property_row(
        ui,
        "Ownership",
        if library.read_only {
            "Technology-owned / read-only"
        } else {
            "Project-editable"
        },
    );
    property_row(
        ui,
        "Source",
        &view.file_path.as_deref().map_or_else(
            || "Not declared".to_owned(),
            |path| path.display().to_string(),
        ),
    );
    property_row(
        ui,
        "Document",
        if view.modified { "Modified" } else { "Saved" },
    );
    match crate::state::ModelBoundSymbolDefinition::load_from_view(view) {
        Ok(Some(definition)) => {
            property_row(ui, "Pin contract", &definition.pins.len().to_string());
            property_row(
                ui,
                "Form fields",
                &definition.parameter_form.fields().count().to_string(),
            );
        }
        Ok(None) => {
            property_row(ui, "Typed contract", "Not declared");
        }
        Err(error) => {
            property_row(ui, "Typed contract", &format!("Invalid: {error}"));
        }
    }
}

fn corner_page_summary(ui: &mut Ui, app: &RSpiceApp) {
    let corner_count = app
        .state
        .model_library_manager
        .libraries_sorted()
        .iter()
        .map(|library| library.corners.len())
        .sum::<usize>();
    section_header(ui, "Section binding", Some(&corner_count.to_string()));
    let Some(key) = app.state.workbench.models_view.selected_corner.as_deref() else {
        property_row(ui, "Selection", "Choose a corner row");
        return;
    };
    let Some((library_name, corner_name)) = key.split_once('\u{1f}') else {
        property_row(ui, "Selection", "Stored corner identity is invalid");
        return;
    };
    let Some(library) = app.state.model_library_manager.get_library(library_name) else {
        property_row(ui, "Selection", "Corner library is no longer loaded");
        return;
    };
    let Some(corner) = library.corners.get(corner_name) else {
        property_row(ui, "Selection", "Corner section is no longer loaded");
        return;
    };
    property_row(ui, "Library", library_name);
    property_row(ui, "Section", &corner.name);
    property_row(ui, "NMOS", &corner.nmos_corner);
    property_row(ui, "PMOS", &corner.pmos_corner);
    property_row(
        ui,
        "Temperature",
        &format!("{:.3} deg C", corner.temperature),
    );
    property_row(ui, "Supply factor", &format!("{:.6}", corner.vdd_factor));
    property_row(
        ui,
        "Source",
        &corner
            .file_path
            .as_deref()
            .or(library.root_path.as_deref())
            .map_or_else(|| "Not bound".to_owned(), |path| path.display().to_string()),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct ModelGeometrySummary {
    cards: usize,
    complete_envelopes: usize,
    invalid_envelopes: usize,
    overlaps: usize,
    libraries: usize,
}

fn model_geometry_summary(app: &RSpiceApp, family: &str) -> ModelGeometrySummary {
    let cards = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| {
            library
                .models
                .values()
                .filter(move |model| {
                    model.model_type.display_name() == family
                        && (model.l_min.is_some()
                            || model.l_max.is_some()
                            || model.w_min.is_some()
                            || model.w_max.is_some())
                })
                .map(move |model| (library.name.as_str(), model))
        })
        .collect::<Vec<_>>();
    let complete_envelopes = cards
        .iter()
        .filter(|(_, model)| {
            model.l_min.is_some()
                && model.l_max.is_some()
                && model.w_min.is_some()
                && model.w_max.is_some()
        })
        .count();
    let invalid_envelopes = cards
        .iter()
        .filter(|(_, model)| model_geometry_is_invalid(model))
        .count();
    let mut overlaps = 0;
    for (index, (_, left)) in cards.iter().enumerate() {
        for (_, right) in cards.iter().skip(index + 1) {
            overlaps += usize::from(model_geometry_overlaps(left, right));
        }
    }
    let libraries = cards
        .iter()
        .map(|(library, _)| *library)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    ModelGeometrySummary {
        cards: cards.len(),
        complete_envelopes,
        invalid_envelopes,
        overlaps,
        libraries,
    }
}

fn model_geometry_is_invalid(model: &crate::state::model_library::DeviceModel) -> bool {
    model
        .l_min
        .zip(model.l_max)
        .is_some_and(|(min, max)| min >= max)
        || model
            .w_min
            .zip(model.w_max)
            .is_some_and(|(min, max)| min >= max)
}

fn model_geometry_overlaps(
    left: &crate::state::model_library::DeviceModel,
    right: &crate::state::model_library::DeviceModel,
) -> bool {
    let (Some(left_l0), Some(left_l1), Some(left_w0), Some(left_w1)) =
        (left.l_min, left.l_max, left.w_min, left.w_max)
    else {
        return false;
    };
    let (Some(right_l0), Some(right_l1), Some(right_w0), Some(right_w1)) =
        (right.l_min, right.l_max, right.w_min, right.w_max)
    else {
        return false;
    };
    left_l0 < right_l1 && right_l0 < left_l1 && left_w0 < right_w1 && right_w0 < left_w1
}

fn bin_page_summary(ui: &mut Ui, app: &RSpiceApp) {
    section_header(ui, "Geometry family", None);
    let Some(family) = app
        .state
        .workbench
        .models_view
        .selected_bin_family
        .as_deref()
    else {
        property_row(ui, "Selection", "Choose a geometry family");
        return;
    };
    let summary = model_geometry_summary(app, family);
    property_row(ui, "Family", family);
    property_row(ui, "Cards", &summary.cards.to_string());
    property_row(ui, "Libraries", &summary.libraries.to_string());
    property_row(
        ui,
        "Complete envelopes",
        &summary.complete_envelopes.to_string(),
    );
    property_row(
        ui,
        "Invalid envelopes",
        &summary.invalid_envelopes.to_string(),
    );
    property_row(ui, "Overlap pairs", &summary.overlaps.to_string());
}

fn include_page_summary(ui: &mut Ui, app: &RSpiceApp) {
    let file_count = app
        .state
        .model_library_manager
        .libraries_sorted()
        .iter()
        .map(|library| library.source_closure.len())
        .sum::<usize>();
    section_header(ui, "Include source", Some(&format!("{file_count} files")));
    let Some(selected_path) = app
        .state
        .workbench
        .models_view
        .include_selected_source
        .as_deref()
    else {
        property_row(ui, "Selection", "Choose a closure source");
        return;
    };
    for library in app.state.model_library_manager.libraries_sorted() {
        let Some((index, source)) = library
            .source_closure
            .iter()
            .enumerate()
            .find(|(_, source)| source.path.to_string_lossy() == selected_path)
        else {
            continue;
        };
        property_row(ui, "Library", &library.name);
        property_row(ui, "Closure order", &format!("{:02}", index + 1));
        property_row(ui, "Path", &source.path.display().to_string());
        property_row(ui, "Digest", &source.digest.to_string());
        property_row(
            ui,
            "Inbound edges",
            &library
                .source_edges
                .iter()
                .filter(|edge| edge.target == source.path)
                .count()
                .to_string(),
        );
        property_row(
            ui,
            "Outbound edges",
            &library
                .source_edges
                .iter()
                .filter(|edge| edge.owner == source.path)
                .count()
                .to_string(),
        );
        return;
    }
    property_row(
        ui,
        "Selection",
        "Source is no longer in the retained closure",
    );
}

fn qualification_page_summary(ui: &mut Ui, app: &RSpiceApp) {
    section_header(ui, "Qualification evidence", None);
    let Some(library) = app.state.model_library_manager.current_library() else {
        property_row(ui, "Selection", "Choose a model library");
        return;
    };
    let Some(model_name) = app.state.workbench.selected_model.as_deref() else {
        property_row(ui, "Selection", "Choose a model family");
        return;
    };
    if !library.models.contains_key(model_name) {
        property_row(
            ui,
            "Selection",
            "Model is no longer in the selected library",
        );
        return;
    }
    property_row(ui, "Model", model_name);
    if let Some(qualification) = library.model_qualification.get(model_name) {
        property_row(ui, "Suites", &qualification.suites.len().to_string());
        property_row(ui, "Evidence", &qualification.evidence.len().to_string());
        property_row(ui, "Releases", &qualification.releases.len().to_string());
    } else {
        property_row(ui, "Suites", "No retained qualification state");
    }
    if let Some(correlation) = library.model_correlation.get(model_name) {
        property_row(
            ui,
            "Correlation suites",
            &correlation.suites.len().to_string(),
        );
        property_row(
            ui,
            "Correlation evidence",
            &correlation.evidence.len().to_string(),
        );
    }
}

fn model_pack_inspector(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Shipped corpus", None);
    let Some(index) = state.model_library_manager.spice_packs() else {
        property_row(ui, "State", "Not installed");
        property_row(ui, "Recovery", "Set RSPICE_MODELS_DIR and rescan");
        return;
    };
    property_row(ui, "Packs", &index.packs().len().to_string());
    property_row(ui, "Parts", &index.part_count().to_string());
    property_row(ui, "Definitions", &index.definition_count().to_string());
    let selected = state
        .workbench
        .models_view
        .selected_pack
        .as_deref()
        .and_then(|id| index.pack(id))
        .cloned()
        .or_else(|| index.packs().first().cloned());
    let Some(pack) = selected else {
        return;
    };
    section_header(ui, "Selected pack", None);
    property_row(ui, "Name", &pack.name);
    property_row(ui, "ID", &pack.id);
    property_row(ui, "Origin", &pack.category);
    property_row(
        ui,
        "Parts",
        &(pack.models_top + pack.subcircuits_top).to_string(),
    );
    property_row(ui, "Files", &pack.files.to_string());
    property_row(ui, "License", &pack.spdx);
    property_row(ui, "Tier", pack.tier.display_name());
    property_row(
        ui,
        "Redistributable",
        if pack.redistributable { "Yes" } else { "No" },
    );
    property_row(
        ui,
        "Entry",
        pack.entry
            .as_deref()
            .and_then(std::path::Path::to_str)
            .unwrap_or("Not declared"),
    );
    section_header(ui, "Actions", None);
    if ui.button("Browse pack parts").clicked() {
        state.workbench.models_view.catalog_scope =
            crate::workbench::state::ModelsCatalogScope::RSpiceLibrary;
        state.workbench.models_view.selected_pack = Some(pack.id);
        state.workbench.models_view.catalog_query.clear();
    }
}

fn shipped_part_inspector(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "RSpice model library", None);
    let Some(index) = state.model_library_manager.spice_packs() else {
        property_row(ui, "State", "Not installed");
        return;
    };
    property_row(ui, "Addressable parts", &index.part_count().to_string());
    property_row(ui, "Packs", &index.packs().len().to_string());
    property_row(
        ui,
        "Redistributable packs",
        &index.redistributable_packs().count().to_string(),
    );
    let selected_key = state
        .workbench
        .models_view
        .selected_part
        .clone()
        .unwrap_or_default();
    let mut components = selected_key.split('\u{1f}');
    let pack_id = components.next().unwrap_or_default();
    let part_name = components.next().unwrap_or_default();
    if !part_name.is_empty() {
        section_header(ui, "Selected part", None);
        property_row(ui, "Name", part_name);
        property_row(ui, "Pack", pack_id);
        if let Some(pack) = index.pack(pack_id) {
            property_row(ui, "Pack title", &pack.name);
            property_row(ui, "Origin", &pack.category);
            property_row(ui, "License", &pack.spdx);
            property_row(
                ui,
                "Project eligibility",
                if pack.redistributable {
                    "Requires per-file check"
                } else {
                    "Blocked"
                },
            );
        }
        section_header(ui, "Actions", None);
        if ui.button("Show owning pack").clicked() {
            state.workbench.models_view.catalog_scope =
                crate::workbench::state::ModelsCatalogScope::InstalledPacks;
            state.workbench.models_view.selected_pack = Some(pack_id.to_owned());
        }
    } else {
        property_row(ui, "Selection", "Choose a catalog part");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn model_availability_never_promotes_unretained_external_sources() {
        use crate::state::model_library::{
            ModelLibrary, ModelSourceAuthority, ModelSourceContent, ModelSourcePin,
        };

        let mut external = ModelLibrary::new("vendor");
        external.source_authority = ModelSourceAuthority::External;
        assert_eq!(
            model_availability(&external),
            ModelAvailability::ExternalUnpinned
        );

        let path = std::path::PathBuf::from("C:/models/vendor.lib");
        external.source_closure.push(ModelSourcePin {
            path: path.clone(),
            digest: crate::product::ContentDigest::from_bytes([0x41; 32]),
        });
        external.source_contents.push(ModelSourceContent {
            path,
            bytes: b".model vendor_npn npn".to_vec(),
        });
        assert_eq!(
            model_availability(&external),
            ModelAvailability::ExternalPinned
        );

        let built_in = ModelLibrary::new("Bipolar");
        assert_eq!(model_availability(&built_in), ModelAvailability::BuiltIn);
    }

    #[test]
    fn geometry_summary_counts_only_real_family_envelopes_and_overlap_pairs() {
        use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

        let mut library = ModelLibrary::new("geometry");
        for (name, l0, l1, w0, w1) in [
            ("nch_a", 1.0, 3.0, 1.0, 3.0),
            ("nch_b", 2.0, 4.0, 2.0, 4.0),
            ("nch_invalid", 6.0, 5.0, 6.0, 7.0),
        ] {
            let mut model = DeviceModel::new(name, ModelType::Nmos);
            model.l_min = Some(l0);
            model.l_max = Some(l1);
            model.w_min = Some(w0);
            model.w_max = Some(w1);
            library.add_model(model);
        }
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.add_library(library);

        assert_eq!(
            model_geometry_summary(&app, "NMOS"),
            ModelGeometrySummary {
                cards: 3,
                complete_envelopes: 3,
                invalid_envelopes: 1,
                overlaps: 1,
                libraries: 1,
            }
        );
    }
}
