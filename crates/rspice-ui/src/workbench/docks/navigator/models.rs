//! Models-source navigator backed by the retained library and include closure.

use egui::{ScrollArea, Sense, Ui};

use crate::ui::tokens::{self, Tokens};
use crate::workbench::AppState;
use crate::workbench::RSpiceApp;
use crate::workbench::design_system::{WorkbenchIcon, property_row, section_header};
use crate::workbench::state::ModelsPage;

use super::{muted, nav_row_indented_styled, nav_row_indented_styled_with_metrics};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ModelSourceGroup {
    Project,
    Pdk,
    Library,
    BuiltIn,
}

impl ModelSourceGroup {
    const fn label(self) -> &'static str {
        match self {
            Self::Project => "Project models",
            Self::Pdk => "PDK libraries",
            Self::Library => "Imported libraries",
            Self::BuiltIn => "RSpice built-ins",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModelNavigatorSource {
    label: String,
    path: String,
    closure_order: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModelNavigatorLibrary {
    name: String,
    group: ModelSourceGroup,
    model_count: usize,
    sources: Vec<ModelNavigatorSource>,
    retained_source_count: usize,
    edge_count: usize,
    corner_count: usize,
    bin_count: usize,
}

impl ModelNavigatorLibrary {
    fn from_library(library: &crate::state::model_library::ModelLibrary) -> Self {
        let group = match library.source_authority {
            crate::state::model_library::ModelSourceAuthority::ProjectOwned { .. } => {
                ModelSourceGroup::Project
            }
            crate::state::model_library::ModelSourceAuthority::BuiltIn => ModelSourceGroup::BuiltIn,
            crate::state::model_library::ModelSourceAuthority::External
                if !library.pdk_name.trim().is_empty()
                    || !library.technology_node.trim().is_empty() =>
            {
                ModelSourceGroup::Pdk
            }
            crate::state::model_library::ModelSourceAuthority::External => {
                ModelSourceGroup::Library
            }
        };
        Self {
            name: library.name.clone(),
            group,
            model_count: library.model_count(),
            sources: library
                .source_closure
                .iter()
                .enumerate()
                .map(|(index, source)| ModelNavigatorSource {
                    label: source
                        .path
                        .file_name()
                        .and_then(|value| value.to_str())
                        .unwrap_or("source")
                        .to_owned(),
                    path: source.path.to_string_lossy().into_owned(),
                    closure_order: index + 1,
                })
                .collect(),
            retained_source_count: library.source_contents.len(),
            edge_count: library.source_edges.len(),
            corner_count: library.corners.len(),
            bin_count: library
                .models
                .values()
                .filter(|model| {
                    model.l_min.is_some()
                        || model.l_max.is_some()
                        || model.w_min.is_some()
                        || model.w_max.is_some()
                })
                .count(),
        }
    }

    fn closure_state(&self) -> &'static str {
        if self.group == ModelSourceGroup::BuiltIn {
            "Catalog metadata"
        } else if self.sources.is_empty() {
            "No pinned closure"
        } else if self.sources.len() == self.retained_source_count {
            "Pinned bytes retained"
        } else {
            "Retained bytes incomplete"
        }
    }
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .id_salt("workbench.models.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            models_content(ui, &mut app.state);
        });
}

fn models_content(ui: &mut Ui, state: &mut AppState) {
    let mut libraries = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(ModelNavigatorLibrary::from_library)
        .collect::<Vec<_>>();
    libraries.sort_by(|left, right| {
        left.group
            .cmp(&right.group)
            .then_with(|| left.name.cmp(&right.name))
    });
    section_header(ui, "Model sources", Some(&libraries.len().to_string()));
    if libraries.is_empty() {
        muted(ui, "Attach a model source to populate the project closure.");
    }
    let mut active_group = None;
    for library in &libraries {
        if active_group != Some(library.group) {
            active_group = Some(library.group);
            let group_model_count = libraries
                .iter()
                .filter(|candidate| candidate.group == library.group)
                .map(|candidate| candidate.model_count)
                .sum::<usize>();
            nav_row_indented_styled_with_metrics(
                ui,
                WorkbenchIcon::Folder,
                library.group.label(),
                false,
                Some(&group_model_count.to_string()),
                0,
                false,
                Tokens::get(ui.ctx()).metrics.row_h,
                tokens::FS_0,
                tokens::FS_0,
                true,
                false,
                Sense::hover(),
            );
        }
        let selected =
            state.model_library_manager.selected_library.as_deref() == Some(library.name.as_str());
        if nav_row_indented_styled(
            ui,
            WorkbenchIcon::Models,
            &library.name,
            selected,
            Some(&library.model_count.to_string()),
            1,
            true,
        )
        .clicked()
        {
            state.model_library_manager.select_library(&library.name);
            state.workbench.models_page = ModelsPage::Models;
            state.workbench.models_view.catalog_scope =
                crate::workbench::state::ModelsCatalogScope::Project;
            state.workbench.selected_model = None;
        }
        if selected {
            for source in library.sources.iter().take(12) {
                let response = nav_row_indented_styled(
                    ui,
                    WorkbenchIcon::Code,
                    &source.label,
                    state
                        .workbench
                        .models_view
                        .include_selected_source
                        .as_deref()
                        == Some(source.path.as_str()),
                    Some(&format!("{:02}", source.closure_order)),
                    2,
                    true,
                );
                let clicked = response.clicked();
                response.on_hover_text(&source.path);
                if clicked {
                    state.workbench.models_page = ModelsPage::Include;
                    state.workbench.models_view.include_selected_source = Some(source.path.clone());
                }
            }
            if library.corner_count > 0
                && nav_row_indented_styled(
                    ui,
                    WorkbenchIcon::Models,
                    "Corners & sections",
                    state.workbench.models_page == ModelsPage::Corners,
                    Some(&library.corner_count.to_string()),
                    2,
                    false,
                )
                .clicked()
            {
                state.workbench.models_page = ModelsPage::Corners;
            }
            if library.bin_count > 0
                && nav_row_indented_styled(
                    ui,
                    WorkbenchIcon::Models,
                    "Bins & geometry",
                    state.workbench.models_page == ModelsPage::Bins,
                    Some(&library.bin_count.to_string()),
                    2,
                    false,
                )
                .clicked()
            {
                state.workbench.models_page = ModelsPage::Bins;
            }
        }
    }
    if let Some(index) = state.model_library_manager.spice_packs() {
        section_header(ui, "Shipped packs", Some(&index.part_count().to_string()));
        let attached = index
            .packs()
            .iter()
            .filter(|pack| {
                let directory = index.root().join(&pack.path);
                state
                    .model_library_manager
                    .libraries_sorted()
                    .iter()
                    .any(|library| {
                        library
                            .root_path
                            .as_deref()
                            .is_some_and(|path| path.starts_with(&directory))
                    })
            })
            .count();
        if nav_row_indented_styled(
            ui,
            WorkbenchIcon::Models,
            "Installed packs",
            state.workbench.models_page == ModelsPage::Models
                && state.workbench.models_view.catalog_scope
                    == crate::workbench::state::ModelsCatalogScope::InstalledPacks,
            Some(&format!("{} · {attached} attached", index.packs().len())),
            0,
            false,
        )
        .clicked()
        {
            state.workbench.models_page = ModelsPage::Models;
            state.workbench.models_view.catalog_scope =
                crate::workbench::state::ModelsCatalogScope::InstalledPacks;
            state.workbench.models_view.catalog_query.clear();
        }
        if nav_row_indented_styled(
            ui,
            WorkbenchIcon::Search,
            "Search all parts",
            state.workbench.models_page == ModelsPage::Models
                && state.workbench.models_view.catalog_scope
                    == crate::workbench::state::ModelsCatalogScope::RSpiceLibrary,
            Some(&index.part_count().to_string()),
            0,
            false,
        )
        .clicked()
        {
            state.workbench.models_page = ModelsPage::Models;
            state.workbench.models_view.catalog_scope =
                crate::workbench::state::ModelsCatalogScope::RSpiceLibrary;
            state.workbench.models_view.selected_pack = None;
            state.workbench.models_view.catalog_query.clear();
        }
    } else {
        section_header(ui, "Shipped packs", None);
        if nav_row_indented_styled(
            ui,
            WorkbenchIcon::Models,
            "Installed packs",
            state.workbench.models_page == ModelsPage::Models
                && state.workbench.models_view.catalog_scope
                    == crate::workbench::state::ModelsCatalogScope::InstalledPacks,
            Some("not found"),
            0,
            false,
        )
        .clicked()
        {
            state.workbench.models_page = ModelsPage::Models;
            state.workbench.models_view.catalog_scope =
                crate::workbench::state::ModelsCatalogScope::InstalledPacks;
        }
        if nav_row_indented_styled(
            ui,
            WorkbenchIcon::Search,
            "Search all parts",
            state.workbench.models_page == ModelsPage::Models
                && state.workbench.models_view.catalog_scope
                    == crate::workbench::state::ModelsCatalogScope::RSpiceLibrary,
            Some("unavailable"),
            0,
            false,
        )
        .clicked()
        {
            state.workbench.models_page = ModelsPage::Models;
            state.workbench.models_view.catalog_scope =
                crate::workbench::state::ModelsCatalogScope::RSpiceLibrary;
            state.workbench.models_view.selected_pack = None;
        }
    }

    let selected = state
        .model_library_manager
        .selected_library
        .as_deref()
        .and_then(|name| libraries.iter().find(|library| library.name == name));
    let closure_meta = selected.map(|library| format!("{} files", library.sources.len()));
    section_header(ui, "Include closure", closure_meta.as_deref());
    if let Some(library) = selected {
        property_row(ui, "Library", &library.name);
        property_row(
            ui,
            "Closure",
            &format!("{} edges / {}", library.edge_count, library.closure_state()),
        );
    } else {
        property_row(ui, "Selection", "Choose a model source");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
    #[test]
    fn model_source_projection_uses_real_authority_and_retained_closure_order() {
        use crate::state::model_library::{
            ModelLibrary, ModelSourceAuthority, ModelSourceContent, ModelSourcePin,
        };

        let mut project = ModelLibrary::new("project_models");
        project.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id: ModelSourceId::new(),
            revision: ObjectRevision::INITIAL,
            digest: ContentDigest::from_bytes([0x21; 32]),
        };
        assert_eq!(
            ModelNavigatorLibrary::from_library(&project).group,
            ModelSourceGroup::Project
        );

        let mut pdk = ModelLibrary::new("demo180").with_technology("demo180", "180nm");
        pdk.source_authority = ModelSourceAuthority::External;
        let base = std::path::PathBuf::from("C:/pdk/models/base.lib");
        let corner = std::path::PathBuf::from("C:/pdk/models/corners.lib");
        pdk.source_closure = vec![
            ModelSourcePin {
                path: base.clone(),
                digest: ContentDigest::from_bytes([0x31; 32]),
            },
            ModelSourcePin {
                path: corner.clone(),
                digest: ContentDigest::from_bytes([0x32; 32]),
            },
        ];
        pdk.source_contents = vec![
            ModelSourceContent {
                path: base,
                bytes: b".include corners.lib".to_vec(),
            },
            ModelSourceContent {
                path: corner,
                bytes: b".lib TT".to_vec(),
            },
        ];
        let projection = ModelNavigatorLibrary::from_library(&pdk);
        assert_eq!(projection.group, ModelSourceGroup::Pdk);
        assert_eq!(projection.sources[0].label, "base.lib");
        assert_eq!(projection.sources[0].closure_order, 1);
        assert_eq!(projection.sources[1].label, "corners.lib");
        assert_eq!(projection.sources[1].closure_order, 2);
        assert_eq!(projection.closure_state(), "Pinned bytes retained");

        let built_in = ModelLibrary::new("MOSFET");
        assert_eq!(
            ModelNavigatorLibrary::from_library(&built_in).group,
            ModelSourceGroup::BuiltIn
        );
    }
}
