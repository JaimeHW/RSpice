//! Project-document identities, dirty comparison and canonical fingerprints.

pub mod result_fingerprint;
pub use result_fingerprint::ResultFingerprintCache;

use rspice_app_types::product::ContentDigest;
use rspice_design::schematic::document::SchematicDocument;
use rspice_design_model::cell_view::CellViewRef;
use serde::Serialize;
use sha2::{Digest as _, Sha256};
use std::collections::{HashMap, HashSet};

/// Stable identity of every project-owned document that participates in
/// Save, Save all, Revert, and dirty-state decisions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectDocumentId {
    ProjectConfiguration,
    CellView(CellViewRef),
    SimulationPlan,
    ResultHistory,
    VerificationSpecifications,
    ModelCatalog,
    NetlistSource,
    StimulusLibrary,
}

impl ProjectDocumentId {
    pub fn stable_key(&self) -> String {
        match self {
            Self::ProjectConfiguration => "project/configuration".to_owned(),
            Self::CellView(reference) => format!("design/{}", reference.key()),
            Self::SimulationPlan => "simulation/plan".to_owned(),
            Self::ResultHistory => "results/history".to_owned(),
            Self::VerificationSpecifications => "verification/specifications".to_owned(),
            Self::ModelCatalog => "models/catalog".to_owned(),
            Self::NetlistSource => "netlist/source".to_owned(),
            Self::StimulusLibrary => "stimulus/library".to_owned(),
        }
    }

    /// The document as a reader names it in a save, close, or revert prompt.
    pub fn label(&self) -> String {
        match self {
            Self::ProjectConfiguration => "Project configuration".to_owned(),
            // Cell and view as the reader knows them, not the persisted
            // library/cell/view key.
            Self::CellView(reference) => format!("{} \u{b7} {}", reference.cell, reference.view),
            Self::SimulationPlan => "Simulation plan".to_owned(),
            Self::ResultHistory => "Result history".to_owned(),
            Self::VerificationSpecifications => "Verification specifications".to_owned(),
            Self::ModelCatalog => "Model catalog".to_owned(),
            Self::NetlistSource => "Netlist source".to_owned(),
            Self::StimulusLibrary => "Stimulus library".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocumentRecord {
    pub id: ProjectDocumentId,
    pub dirty: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DocumentRegistry {
    records: Vec<DocumentRecord>,
    comparison_failed: bool,
}

impl DocumentRegistry {
    pub fn records(&self) -> &[DocumentRecord] {
        &self.records
    }

    pub fn is_dirty(&self, id: &ProjectDocumentId) -> bool {
        self.comparison_failed
            || self
                .records
                .iter()
                .find(|record| &record.id == id)
                .is_some_and(|record| record.dirty)
    }

    pub fn comparison_failed(&self) -> bool {
        self.comparison_failed
    }

    /// A failed comparison cannot provide evidence that any document is clean,
    /// including documents created since the previous successful comparison.
    pub fn invalidate(&mut self) {
        self.comparison_failed = true;
        for record in &mut self.records {
            record.dirty = true;
        }
    }

    pub fn rebuild_from_fingerprints(
        &mut self,
        current: &DocumentFingerprints,
        accepted: Option<&DocumentFingerprints>,
    ) {
        let current = &current.documents;
        let empty = HashMap::new();
        let accepted = accepted.map_or(&empty, |fingerprints| &fingerprints.documents);
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
        self.comparison_failed = false;
    }
}

/// Cached identities of one complete, immutable project snapshot.
#[derive(Debug)]
pub struct DocumentFingerprints {
    documents: HashMap<ProjectDocumentId, ContentDigest>,
    content: ContentDigest,
}

impl DocumentFingerprints {
    pub fn from_documents(documents: HashMap<ProjectDocumentId, ContentDigest>) -> Self {
        let mut ordered = documents.iter().collect::<Vec<_>>();
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
        let content = ContentDigest::from_bytes(hasher.finalize().into());
        Self { documents, content }
    }

    pub fn content_digest(&self) -> ContentDigest {
        self.content
    }
}

/// Explicit lifecycle projection of a schematic document. Runtime interaction
/// state (selection, wire preview, clipboard, pan/zoom, caches, derived
/// terminal connections, and undo history) is deliberately impossible to
/// serialize through this type.
#[derive(Serialize)]
pub struct SchematicDocumentContent<'a> {
    schema_version: u16,
    grid_size: i32,
    document_policy: rspice_design::schematic::document_policy::SchematicDocumentPolicy,
    components: &'a [rspice_design::schematic::component::Component],
    wires: &'a [rspice_design::schematic::wire::Wire],
    buses: &'a [rspice_design::schematic::bus::Bus],
    bus_taps: &'a [rspice_design::schematic::bus::BusTap],
    net_labels: &'a [rspice_design::schematic::net_label::NetLabel],
    design_notes: &'a [rspice_design::schematic::design_note::DesignNote],
    documentation_shapes: &'a [rspice_design::schematic::documentation_shape::DocumentationShape],
    junctions: &'a [rspice_design::schematic::net_label::Junction],
    probes: &'a [rspice_design::schematic::probe::SchematicProbe],
}

impl<'a> From<&'a SchematicDocument> for SchematicDocumentContent<'a> {
    fn from(schematic: &'a SchematicDocument) -> Self {
        Self {
            schema_version: 5,
            grid_size: schematic.grid_size,
            document_policy: schematic.document_policy,
            components: &schematic.components,
            wires: &schematic.wires,
            buses: &schematic.buses,
            bus_taps: &schematic.bus_taps,
            net_labels: &schematic.net_labels,
            design_notes: &schematic.design_notes,
            documentation_shapes: &schematic.documentation_shapes,
            junctions: &schematic.junctions,
            probes: &schematic.probes,
        }
    }
}

/// Engineering portion of a library view. View metadata owns document
/// content; browser/file bindings, open state, timestamps, and dirty state do
/// not.
#[derive(Serialize)]
pub struct ViewDocumentContent<'a> {
    schema_version: u16,
    name: &'a str,
    view_type: rspice_design::library::ViewType,
    metadata: &'a HashMap<String, String>,
}

impl<'a> From<&'a rspice_design::library::View> for ViewDocumentContent<'a> {
    fn from(view: &'a rspice_design::library::View) -> Self {
        Self {
            schema_version: 1,
            name: &view.name,
            view_type: view.view_type,
            metadata: &view.metadata,
        }
    }
}

pub fn project_configuration_value(
    project: &crate::ProjectDescriptor,
    libraries: &impl Serialize,
    configuration_sets: &rspice_design::configuration_set::ConfigurationSetCatalog,
    design_management: &rspice_design_model::design_management::DesignManagementCatalog,
    pdk_callback_receipts: &[rspice_model_library::pdk::callback::ProjectPdkCallbackReceipt],
) -> Result<serde_json::Value, String> {
    let mut value = serde_json::to_value((
        project,
        libraries,
        configuration_sets,
        design_management,
        pdk_callback_receipts,
    ))
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
    // Every view entry, including its existence and metadata, is owned by the
    // corresponding CellView document. Project configuration owns the
    // library/cell catalog only; otherwise saving configuration could silently
    // accept an unrelated unsaved view or reverting it could discard one.
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
                        views.clear();
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
        // The revision is a concurrency/audit coordinate, not independent
        // engineering configuration. The catalog structure below is the
        // content authority, while each view document owns its view payload.
        "revision",
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

pub fn reference_from_key(key: &str) -> Option<CellViewRef> {
    let mut segments = key.split('/');
    let reference = CellViewRef::new(segments.next()?, segments.next()?, segments.next()?);
    segments.next().is_none().then_some(reference)
}

pub fn digest(value: &impl Serialize) -> Result<ContentDigest, String> {
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
}
