//! Project-document identities, dirty comparison and canonical fingerprints.

mod canonical;
mod fingerprints;
#[cfg(any(test, feature = "document-fingerprint-observation"))]
pub(crate) use fingerprints::observe;
#[cfg(any(test, feature = "document-fingerprint-observation"))]
pub use fingerprints::{DOCUMENT_DIGESTS, FINGERPRINT_PASSES, PROJECT_CAPTURES};
pub use fingerprints::{
    content_digest, document_digests, document_fingerprints,
    document_fingerprints_with_results_cache, unobserved,
};

pub mod result_fingerprint;
pub use result_fingerprint::ResultFingerprintCache;

use rspice_design::schematic::document::SchematicDocument;
use rspice_design_model::cell_view::CellViewRef;
use rspice_vocabulary::product::ContentDigest;
use serde::Serialize;
use sha2::{Digest as _, Sha256};
use std::collections::HashMap;

/// Stable identity of every project-owned document that participates in
/// Save, Save all, Revert, and dirty-state decisions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectDocumentId {
    ProjectConfiguration,
    CellView(CellViewRef),
    SimulationPlan,
    ResultHistory,
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
    /// In stable-key order.
    records: Vec<DocumentRecord>,
    /// Each record's place in `records`.
    index: HashMap<ProjectDocumentId, usize>,
    /// How many records are dirty.
    dirty: usize,
    comparison_failed: bool,
}

impl DocumentRegistry {
    pub fn records(&self) -> &[DocumentRecord] {
        &self.records
    }

    pub fn is_dirty(&self, id: &ProjectDocumentId) -> bool {
        self.comparison_failed || self.index.get(id).is_some_and(|&at| self.records[at].dirty)
    }

    /// Whether any document differs from the accepted one.
    pub fn any_dirty(&self) -> bool {
        self.dirty > 0
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
        self.dirty = self.records.len();
    }

    pub fn rebuild_from_fingerprints(
        &mut self,
        current: &DocumentFingerprints,
        accepted: Option<&DocumentFingerprints>,
    ) {
        let empty = HashMap::new();
        self.rebuild(
            &current.documents,
            accepted.map_or(&empty, |fingerprints| &fingerprints.documents),
        );
    }

    /// Every document of `current` and `accepted`, dirty where the two differ.
    fn rebuild(
        &mut self,
        current: &HashMap<ProjectDocumentId, ContentDigest>,
        accepted: &HashMap<ProjectDocumentId, ContentDigest>,
    ) {
        let mut ids = current
            .keys()
            .chain(accepted.keys().filter(|id| !current.contains_key(*id)))
            .cloned()
            .collect::<Vec<_>>();
        ids.sort_by_cached_key(ProjectDocumentId::stable_key);
        self.records = ids
            .into_iter()
            .map(|id| DocumentRecord {
                dirty: current.get(&id) != accepted.get(&id),
                id,
            })
            .collect();
        self.reindex();
        self.comparison_failed = false;
    }

    fn reindex(&mut self) {
        self.index = self
            .records
            .iter()
            .enumerate()
            .map(|(at, record)| (record.id.clone(), at))
            .collect();
        self.dirty = self.records.iter().filter(|record| record.dirty).count();
    }
}

/// The identity of a whole project from its documents' identities.
fn project_content_digest(documents: &HashMap<ProjectDocumentId, ContentDigest>) -> ContentDigest {
    // Each key is built once: a comparison that built its own two keys
    // cost a project of 5,000 cells about 50 ms per sort.
    let mut ordered = documents
        .iter()
        .map(|(id, digest)| (id.stable_key(), digest))
        .collect::<Vec<_>>();
    ordered.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-project-content-digest\0v1\0");
    hasher.update((ordered.len() as u64).to_be_bytes());
    for (key, digest) in ordered {
        hasher.update((key.len() as u64).to_be_bytes());
        hasher.update(key.as_bytes());
        hasher.update(digest.as_bytes());
    }
    ContentDigest::from_bytes(hasher.finalize().into())
}

/// Cached identities of one complete, immutable project snapshot.
#[derive(Debug)]
pub struct DocumentFingerprints {
    documents: HashMap<ProjectDocumentId, ContentDigest>,
    content: ContentDigest,
}

impl DocumentFingerprints {
    pub fn from_documents(documents: HashMap<ProjectDocumentId, ContentDigest>) -> Self {
        let content = project_content_digest(&documents);
        Self { documents, content }
    }

    pub fn content_digest(&self) -> ContentDigest {
        self.content
    }

    /// Whether the snapshot holds the document at all.
    pub fn contains(&self, id: &ProjectDocumentId) -> bool {
        self.documents.contains_key(id)
    }

    /// The identity of one document, when the snapshot holds it.
    pub fn get(&self, id: &ProjectDocumentId) -> Option<&ContentDigest> {
        self.documents.get(id)
    }

    /// The documents whose identity differs between `self` and `other`,
    /// including any present in only one, in stable-key order.
    pub fn differing_documents(&self, other: &Self) -> Vec<ProjectDocumentId> {
        differing_documents(&self.documents, &other.documents)
    }
}

/// The documents whose identity differs between two projects, including any
/// present in only one, in stable-key order.
fn differing_documents(
    left: &HashMap<ProjectDocumentId, ContentDigest>,
    right: &HashMap<ProjectDocumentId, ContentDigest>,
) -> Vec<ProjectDocumentId> {
    let mut differing = left
        .iter()
        .filter(|(id, digest)| right.get(*id) != Some(*digest))
        .map(|(id, _)| id.clone())
        .chain(right.keys().filter(|id| !left.contains_key(*id)).cloned())
        .collect::<Vec<_>>();
    differing.sort_by_cached_key(ProjectDocumentId::stable_key);
    differing
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

/// The project descriptor's share of the configuration document. Its path
/// is a persistence binding and its revision a concurrency coordinate:
/// neither is engineering content.
pub(crate) fn project_descriptor_digest(
    project: &crate::ProjectDescriptor,
) -> Result<ContentDigest, String> {
    let mut value = serde_json::to_value(project).map_err(|error| error.to_string())?;
    if let Some(path) = value.pointer_mut("/path") {
        *path = serde_json::Value::Null;
    }
    if let Some(project) = value.as_object_mut() {
        project.remove("revision");
    }
    digest(&value)
}

/// The library catalog's share of the configuration document: libraries
/// and cells. Every view entry, including its existence and metadata, is
/// owned by its cell view's document; otherwise saving the configuration
/// could silently accept an unrelated unsaved view, or reverting it discard
/// one. Folder paths are persistence bindings, and the browser's selection
/// and the catalog revision are presentation and coordination.
pub(crate) fn library_catalog_digest(libraries: &impl Serialize) -> Result<ContentDigest, String> {
    let mut value = serde_json::to_value(libraries).map_err(|error| error.to_string())?;
    if let Some(manager) = value.as_object_mut() {
        for key in [
            "selected_library",
            "selected_cell",
            "selected_view",
            "revision",
        ] {
            manager.remove(key);
        }
    }
    if let Some(libraries) = value
        .pointer_mut("/libraries")
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
    digest(&value)
}

/// The configuration document from its four shares: the project
/// descriptor, the library catalog, the configuration sets and design
/// management. Each share is digested on its own, so a change to one leaves
/// the others' digests standing.
pub(crate) fn configuration_digest(
    descriptor: ContentDigest,
    catalog: ContentDigest,
    configuration_sets: ContentDigest,
    design_management: ContentDigest,
) -> ContentDigest {
    composed(
        b"rspice-project-configuration\0v2\0",
        &[descriptor, catalog, configuration_sets, design_management],
    )
}

/// A cell view document from its four shares: the drawing, the physical
/// layout, the view entry and the project source it owns, each digested as
/// the value or `null`. An edit to the drawing leaves the other three
/// standing, and an edit to the library leaves the drawing's.
pub(crate) fn cell_view_digest(parts: &[ContentDigest; 4]) -> ContentDigest {
    composed(b"rspice-cell-view-document\0v2\0", parts)
}

fn composed(tag: &[u8], parts: &[ContentDigest]) -> ContentDigest {
    let mut hasher = Sha256::new();
    hasher.update(tag);
    hasher.update((parts.len() as u64).to_be_bytes());
    for part in parts {
        hasher.update(part.as_bytes());
    }
    ContentDigest::from_bytes(hasher.finalize().into())
}

pub fn reference_from_key(key: &str) -> Option<CellViewRef> {
    let mut segments = key.split('/');
    let reference = CellViewRef::new(segments.next()?, segments.next()?, segments.next()?);
    segments.next().is_none().then_some(reference)
}

thread_local! {
    /// The buffer the last digest on this thread encoded into, kept so a
    /// drawing of thousands of parts is not grown from nothing every time.
    static ENCODING: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// The identity of `value`'s canonical JSON encoding: its keys in byte
/// order, no whitespace, numbers and strings as `serde_json` writes them.
pub fn digest(value: &impl Serialize) -> Result<ContentDigest, String> {
    let mut canonical = ENCODING.with(|buffer| std::mem::take(&mut *buffer.borrow_mut()));
    canonical.clear();
    let written = canonical::write(value, &mut canonical).map_err(|error| error.to_string());
    let digest = written.map(|()| {
        let mut hasher = Sha256::new();
        hasher.update(b"rspice-canonical-json-digest\0v1\0");
        hasher.update((canonical.len() as u64).to_be_bytes());
        hasher.update(&canonical);
        ContentDigest::from_bytes(hasher.finalize().into())
    });
    ENCODING.with(|buffer| *buffer.borrow_mut() = canonical);
    digest
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

    /// The encoding written straight from a value is byte for byte the one
    /// its JSON value tree writes, so no saved file's digest moves.
    #[test]
    fn canonical_encoding_matches_the_json_value_tree() {
        use serde::ser::{SerializeMap as _, Serializer};
        use std::collections::BTreeMap;

        #[derive(Serialize)]
        struct Unit;
        #[derive(Serialize)]
        struct Wrapped(u8);
        #[derive(Serialize)]
        struct Pair(i32, &'static str);
        #[derive(Serialize)]
        enum Shape {
            Point,
            Circle(f64),
            Line(i64, i64),
            Box { width: f32, height: f32 },
        }
        #[derive(Serialize)]
        struct Inner {
            zeta: Option<u32>,
            alpha: Vec<Shape>,
        }
        #[derive(Serialize)]
        struct Outer {
            name: String,
            #[serde(flatten)]
            inner: Inner,
            by_number: BTreeMap<i64, char>,
            by_flag: BTreeMap<bool, Unit>,
            wrapped: Wrapped,
            pair: Pair,
            wide: (i128, u128),
            nothing: (),
            floats: Vec<f64>,
            narrow: f32,
        }
        struct Bytes;
        impl Serialize for Bytes {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_bytes(&[0, 7, 255])
            }
        }
        /// A map that names one key twice and keys others by floats.
        struct Repeated;
        impl Serialize for Repeated {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("b", &1)?;
                map.serialize_entry("a", &2)?;
                map.serialize_entry("b", &3)?;
                map.serialize_entry(&1.5_f64, &4)?;
                map.serialize_entry(&0.1_f32, &5)?;
                map.end()
            }
        }

        let outer = Outer {
            name: "quote \" slash \\ line\n tab\t bell\u{7} \u{e9} \u{1f600}".to_owned(),
            inner: Inner {
                zeta: None,
                alpha: vec![
                    Shape::Point,
                    Shape::Circle(0.1),
                    Shape::Line(-3, i64::MAX),
                    Shape::Box {
                        width: 0.1,
                        height: 2.0,
                    },
                ],
            },
            by_number: BTreeMap::from([(-1, 'x'), (10, '"'), (2, '\u{0}')]),
            by_flag: BTreeMap::from([(true, Unit), (false, Unit)]),
            wrapped: Wrapped(9),
            pair: Pair(-7, ""),
            wide: (i128::from(i64::MIN), u128::from(u64::MAX)),
            nothing: (),
            floats: vec![
                1.0,
                -0.0,
                1e300,
                1e-7,
                123456789.125,
                f64::NAN,
                f64::INFINITY,
            ],
            narrow: 16_777_217.0,
        };
        let oracle = |value: &dyn erased::Value| {
            let tree = value.tree()?;
            let mut bytes = Vec::new();
            write_canonical_json(&tree, &mut bytes)?;
            Ok::<_, String>(bytes)
        };
        let streamed = |value: &dyn erased::Value| value.stream();
        let values: [&dyn erased::Value; 6] = [
            &outer,
            &Bytes,
            &Repeated,
            &Shape::Point,
            &vec![BTreeMap::<String, u8>::new()],
            &Some(Wrapped(1)),
        ];
        for value in values {
            let expected = oracle(value).unwrap();
            assert_eq!(
                String::from_utf8(streamed(value).unwrap()).unwrap(),
                String::from_utf8(expected).unwrap(),
            );
        }

        // What JSON cannot hold, neither can encode.
        let failing: [&dyn erased::Value; 3] = [
            &BTreeMap::from([(vec![1_u8], 1_u8)]),
            &(u128::from(u64::MAX) + 1),
            &BTreeMap::from([(Some(1_u8), 1_u8)]),
        ];
        for value in failing {
            assert!(oracle(value).is_err());
            assert!(streamed(value).is_err());
        }
    }

    mod erased {
        use serde::Serialize;

        pub(super) trait Value {
            fn tree(&self) -> Result<serde_json::Value, String>;
            fn stream(&self) -> Result<Vec<u8>, String>;
        }

        impl<T: Serialize> Value for T {
            fn tree(&self) -> Result<serde_json::Value, String> {
                serde_json::to_value(self).map_err(|error| error.to_string())
            }

            fn stream(&self) -> Result<Vec<u8>, String> {
                let mut bytes = Vec::new();
                super::super::canonical::write(self, &mut bytes)
                    .map_err(|error| error.to_string())?;
                Ok(bytes)
            }
        }
    }
}
