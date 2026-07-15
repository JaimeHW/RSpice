//! Stable project-document registry and content fingerprints.

use std::collections::{HashMap, HashSet};

use serde::Serialize;
use sha2::{Digest as _, Sha256};

use crate::io::ProjectFile;
use crate::product::ContentDigest;
use crate::state::CellViewRef;
use crate::workbench::state::Workspace;

/// Stable identity of every project-owned document that participates in
/// Save, Save all, Revert, and dirty-state decisions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ProjectDocumentId {
    ProjectConfiguration,
    CellView(CellViewRef),
    SimulationPlan,
    ResultHistory,
    VerificationSpecifications,
    ModelCatalog,
    NetlistSource,
}

impl ProjectDocumentId {
    pub(crate) fn stable_key(&self) -> String {
        match self {
            Self::ProjectConfiguration => "project/configuration".to_owned(),
            Self::CellView(reference) => format!("design/{}", reference.key()),
            Self::SimulationPlan => "simulation/plan".to_owned(),
            Self::ResultHistory => "results/history".to_owned(),
            Self::VerificationSpecifications => "verification/specifications".to_owned(),
            Self::ModelCatalog => "models/catalog".to_owned(),
            Self::NetlistSource => "netlist/source".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DocumentRecord {
    pub(crate) id: ProjectDocumentId,
    pub(crate) dirty: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DocumentRegistry {
    records: Vec<DocumentRecord>,
}

impl DocumentRegistry {
    pub(crate) fn records(&self) -> &[DocumentRecord] {
        &self.records
    }

    pub(crate) fn is_dirty(&self, id: &ProjectDocumentId) -> bool {
        self.records
            .iter()
            .find(|record| &record.id == id)
            .is_some_and(|record| record.dirty)
    }

    pub(crate) fn rebuild(
        &mut self,
        current: &ProjectFile,
        accepted: Option<&ProjectFile>,
    ) -> Result<(), String> {
        let current = document_digests(current)?;
        let accepted = match accepted {
            Some(project) => document_digests(project)?,
            None => HashMap::new(),
        };
        let mut ids = current
            .keys()
            .chain(accepted.keys())
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        ids.sort_by_key(ProjectDocumentId::stable_key);
        self.records = ids
            .into_iter()
            .map(|id| DocumentRecord {
                dirty: current.get(&id) != accepted.get(&id),
                id,
            })
            .collect();
        Ok(())
    }
}

pub(crate) fn active_document(
    workspace: Workspace,
    active_view: &CellViewRef,
) -> ProjectDocumentId {
    match workspace {
        Workspace::Project => ProjectDocumentId::ProjectConfiguration,
        Workspace::Design => ProjectDocumentId::CellView(active_view.clone()),
        Workspace::Simulate => ProjectDocumentId::SimulationPlan,
        Workspace::Results => ProjectDocumentId::ResultHistory,
        Workspace::Verify => ProjectDocumentId::VerificationSpecifications,
        Workspace::Models => ProjectDocumentId::ModelCatalog,
        Workspace::Netlist => ProjectDocumentId::NetlistSource,
    }
}

pub(crate) fn content_digest(project: &ProjectFile) -> Result<ContentDigest, String> {
    let digests = document_digests(project)?;
    let mut ordered = digests.into_iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(id, _)| id.stable_key());
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-project-content-digest\0v1\0");
    hasher.update((ordered.len() as u64).to_be_bytes());
    for (id, digest) in ordered {
        let key = id.stable_key();
        hasher.update((key.len() as u64).to_be_bytes());
        hasher.update(key.as_bytes());
        hasher.update(digest.as_bytes());
    }
    Ok(ContentDigest::from_bytes(hasher.finalize().into()))
}

fn document_digests(
    project: &ProjectFile,
) -> Result<HashMap<ProjectDocumentId, ContentDigest>, String> {
    let mut documents = HashMap::new();

    documents.insert(
        ProjectDocumentId::ProjectConfiguration,
        digest(&project_configuration_value(project)?)?,
    );
    documents.insert(
        ProjectDocumentId::SimulationPlan,
        digest(
            &project
                .execution_context
                .as_ref()
                .map(|context| &context.simulation_plan),
        )?,
    );
    documents.insert(
        ProjectDocumentId::ModelCatalog,
        digest(
            &project
                .execution_context
                .as_ref()
                .map(|context| &context.model_libraries),
        )?,
    );
    documents.insert(
        ProjectDocumentId::ResultHistory,
        digest(&project.simulation_results)?,
    );
    documents.insert(
        ProjectDocumentId::VerificationSpecifications,
        digest(&project.workspace.specs)?,
    );
    documents.insert(
        ProjectDocumentId::NetlistSource,
        digest(&(
            &project.workspace.netlist_source,
            &project.workspace.netlist_source_path,
        ))?,
    );

    let mut references = HashSet::new();
    for key in project.workspace.schematic_buffers.keys() {
        if let Some(reference) = reference_from_key(key) {
            references.insert(reference);
        }
    }
    for (library_key, library) in project.libraries.libraries_by_key() {
        for (cell_key, cell) in &library.cells {
            for view_key in cell.views.keys() {
                references.insert(CellViewRef::new(library_key, cell_key, view_key));
            }
        }
    }
    for reference in references {
        let schematic = project
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .map(SchematicDocumentContent::from);
        let view = project
            .libraries
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(ViewDocumentContent::from);
        documents.insert(
            ProjectDocumentId::CellView(reference),
            digest(&(schematic, view))?,
        );
    }

    Ok(documents)
}

/// Explicit lifecycle projection of a schematic document. Runtime interaction
/// state (selection, wire preview, clipboard, pan/zoom, caches, derived
/// terminal connections, and undo history) is deliberately impossible to
/// serialize through this type.
#[derive(Serialize)]
struct SchematicDocumentContent<'a> {
    schema_version: u16,
    components: &'a [crate::state::Component],
    wires: &'a [crate::state::Wire],
    net_labels: &'a [crate::state::NetLabel],
    junctions: &'a [crate::state::Junction],
}

impl<'a> From<&'a crate::state::SchematicState> for SchematicDocumentContent<'a> {
    fn from(schematic: &'a crate::state::SchematicState) -> Self {
        Self {
            schema_version: 1,
            components: &schematic.components,
            wires: &schematic.wires,
            net_labels: &schematic.net_labels,
            junctions: &schematic.junctions,
        }
    }
}

/// Engineering portion of a library view. View metadata owns document
/// content; browser/file bindings, open state, timestamps, and dirty state do
/// not.
#[derive(Serialize)]
struct ViewDocumentContent<'a> {
    schema_version: u16,
    name: &'a str,
    view_type: crate::state::ViewType,
    metadata: &'a HashMap<String, String>,
}

impl<'a> From<&'a crate::state::View> for ViewDocumentContent<'a> {
    fn from(view: &'a crate::state::View) -> Self {
        Self {
            schema_version: 1,
            name: &view.name,
            view_type: view.view_type,
            metadata: &view.metadata,
        }
    }
}

fn project_configuration_value(project: &ProjectFile) -> Result<serde_json::Value, String> {
    let mut value = serde_json::to_value((&project.workspace.project, &project.libraries))
        .map_err(|error| error.to_string())?;
    // Paths are persistence bindings and browser/tree expansion is
    // presentation state.  Neither makes engineering content dirty.
    if let Some(project) = value.pointer_mut("/0/path") {
        *project = serde_json::Value::Null;
    }
    if let Some(project) = value.get_mut(0).and_then(serde_json::Value::as_object_mut) {
        project.remove("revision");
    }
    scrub_library_presentation(&mut value);
    // Cell-view metadata is owned by the corresponding CellView document,
    // not by project configuration.
    if let Some(libraries) = value
        .pointer_mut("/1/libraries")
        .and_then(|v| v.as_object_mut())
    {
        for library in libraries.values_mut() {
            if let Some(object) = library.as_object_mut() {
                object.remove("path");
            }
            if let Some(cells) = library.get_mut("cells").and_then(|v| v.as_object_mut()) {
                for cell in cells.values_mut() {
                    if let Some(views) = cell.get_mut("views").and_then(|v| v.as_object_mut()) {
                        for view in views.values_mut() {
                            if let Some(object) = view.as_object_mut() {
                                object.remove("metadata");
                                object.remove("modified");
                                object.remove("is_open");
                                object.remove("modified_time");
                                object.remove("file_path");
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(value)
}

fn scrub_library_presentation(value: &mut serde_json::Value) {
    let Some(manager) = value.get_mut(1).and_then(serde_json::Value::as_object_mut) else {
        return;
    };
    for key in [
        "selected_library",
        "selected_cell",
        "selected_view",
        "filter_text",
        "show_read_only",
    ] {
        manager.remove(key);
    }
    if let Some(libraries) = manager.get_mut("libraries").and_then(|v| v.as_object_mut()) {
        for library in libraries.values_mut() {
            if let Some(object) = library.as_object_mut() {
                object.remove("expanded");
                if let Some(cells) = object.get_mut("cells").and_then(|v| v.as_object_mut()) {
                    for cell in cells.values_mut() {
                        if let Some(object) = cell.as_object_mut() {
                            object.remove("expanded");
                        }
                    }
                }
            }
        }
    }
}

fn reference_from_key(key: &str) -> Option<CellViewRef> {
    let mut segments = key.split('/');
    let reference = CellViewRef::new(segments.next()?, segments.next()?, segments.next()?);
    segments.next().is_none().then_some(reference)
}

fn digest(value: &impl Serialize) -> Result<ContentDigest, String> {
    let value = serde_json::to_value(value).map_err(|error| error.to_string())?;
    let mut canonical = Vec::new();
    write_canonical_json(&value, &mut canonical)?;
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-canonical-json-digest\0v1\0");
    hasher.update((canonical.len() as u64).to_be_bytes());
    hasher.update(canonical);
    Ok(ContentDigest::from_bytes(hasher.finalize().into()))
}

fn write_canonical_json(value: &serde_json::Value, out: &mut Vec<u8>) -> Result<(), String> {
    match value {
        serde_json::Value::Null => out.extend_from_slice(b"null"),
        serde_json::Value::Bool(value) => {
            out.extend_from_slice(if *value { b"true" } else { b"false" })
        }
        serde_json::Value::Number(number) => out.extend_from_slice(number.to_string().as_bytes()),
        serde_json::Value::String(string) => {
            serde_json::to_writer(&mut *out, string).map_err(|error| error.to_string())?;
        }
        serde_json::Value::Array(values) => {
            out.push(b'[');
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    out.push(b',');
                }
                write_canonical_json(value, out)?;
            }
            out.push(b']');
        }
        serde_json::Value::Object(values) => {
            out.push(b'{');
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
            for (index, (key, value)) in entries.into_iter().enumerate() {
                if index != 0 {
                    out.push(b',');
                }
                serde_json::to_writer(&mut *out, key).map_err(|error| error.to_string())?;
                out.push(b':');
                write_canonical_json(value, out)?;
            }
            out.push(b'}');
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::app::AppState;
    use crate::state::{ComponentType, Point};

    #[test]
    fn canonical_digest_is_independent_of_map_insertion_order() {
        let mut first = HashMap::new();
        first.insert("alpha", 1_u32);
        first.insert("beta", 2_u32);
        let mut second = HashMap::new();
        second.insert("beta", 2_u32);
        second.insert("alpha", 1_u32);

        assert_eq!(digest(&first).unwrap(), digest(&second).unwrap());
    }

    #[test]
    fn presentation_and_interaction_state_never_marks_engineering_documents_dirty() {
        let mut state = AppState::default();
        let component = state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(4, 6));
        let baseline = super::super::snapshot(&state).expect("baseline snapshot");
        let active = state.workspace.active_view.clone();

        state.schematic.selection.select_component(component);
        state.schematic.copy_selection();
        state.schematic.pan = (125.0, -40.0);
        state.schematic.zoom = 2.25;
        state.schematic.grid_size = 25;
        state.schematic.current_file = Some(std::path::PathBuf::from("presentation.rsch"));
        state.workspace.open_views[0].dirty = true;
        state.library_manager.filter_text = "presentation filter".to_owned();
        state.library_manager.show_read_only = !state.library_manager.show_read_only;
        state
            .library_manager
            .select_view(&active.library, &active.cell, &active.view);
        let view = state
            .library_manager
            .get_library_mut(&active.library)
            .and_then(|library| library.get_cell_mut(&active.cell))
            .and_then(|cell| cell.get_view_mut(&active.view))
            .expect("active view metadata");
        view.is_open = true;
        view.modified = true;
        view.file_path = Some(std::path::PathBuf::from("presentation-only.sch"));
        view.modified_time = Some(42);

        let current = super::super::snapshot(&state).expect("current snapshot");
        let mut registry = DocumentRegistry::default();
        registry
            .rebuild(&current, Some(&baseline))
            .expect("rebuild registry");
        assert!(
            registry.records().iter().all(|record| !record.dirty),
            "selection, clipboard, viewport, grid, open-state, and browser presentation are not engineering edits"
        );

        state
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(12, 9));
        let edited = super::super::snapshot(&state).expect("edited snapshot");
        registry
            .rebuild(&edited, Some(&baseline))
            .expect("rebuild edited registry");
        assert!(registry.is_dirty(&ProjectDocumentId::CellView(active)));
        assert!(!registry.is_dirty(&ProjectDocumentId::ProjectConfiguration));
    }
}
