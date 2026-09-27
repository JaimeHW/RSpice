//! Persisted schematic content, independent of editor and session state.

use serde::{Deserialize, Serialize};

use super::bus::{Bus, BusTap};
use super::component::Component;
use super::design_note::DesignNote;
use super::document_policy::SchematicDocumentPolicy;
use super::documentation_shape::DocumentationShape;
use super::net_label::{Junction, NetLabel};
use super::probe::SchematicProbe;
use super::validated_revision::ValidatedRevisionJournal;
use super::wire::{Wire, WireConnection};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "SchematicState")]
pub struct SchematicDocument {
    /// All placed components
    pub components: Vec<Component>,

    /// All wires
    pub wires: Vec<Wire>,

    /// Durable multi-conductor bus polylines.
    #[serde(default)]
    pub buses: Vec<Bus>,

    /// Typed scalar and slice taps attached to declared buses.
    #[serde(default)]
    pub bus_taps: Vec<BusTap>,

    /// Durable non-electrical schematic documentation objects.
    #[serde(default)]
    pub design_notes: Vec<DesignNote>,

    /// Durable non-electrical lines, boundaries, arcs, polygons, and callouts.
    #[serde(default)]
    pub documentation_shapes: Vec<DocumentationShape>,

    /// Durable crosshair flags created by the schematic Probe tool.
    #[serde(default)]
    pub probes: Vec<SchematicProbe>,

    /// Grid size in pixels
    pub grid_size: i32,

    /// Project-portable editor semantics resolved when this document was
    /// created. Legacy schematics receive the reviewed default policy.
    #[serde(default)]
    pub document_policy: SchematicDocumentPolicy,

    /// Net labels for naming nodes
    #[serde(default)]
    pub net_labels: Vec<NetLabel>,

    /// Explicit wire junctions for connecting crossing wires
    /// Only wires sharing an endpoint OR joined by an explicit junction are connected
    pub junctions: Vec<Junction>,

    /// Wire-to-terminal connections (for rubber-banding)
    pub connections: Vec<WireConnection>,

    /// Durable, append-only evidence for validated schematic saves.
    ///
    /// Legacy schematic documents predate this journal and deserialize with
    /// an empty history. Records own exact design snapshots; transient editor
    /// state and undo history are intentionally excluded.
    #[serde(default)]
    pub validated_revisions: ValidatedRevisionJournal,
}

impl Default for SchematicDocument {
    fn default() -> Self {
        let document_policy = SchematicDocumentPolicy::default();
        let grid_size = document_policy.grid_pitch.canvas_grid_size();
        Self {
            components: Vec::new(),
            wires: Vec::new(),
            buses: Vec::new(),
            bus_taps: Vec::new(),
            design_notes: Vec::new(),
            documentation_shapes: Vec::new(),
            probes: Vec::new(),
            grid_size,
            document_policy,
            net_labels: Vec::new(),
            junctions: Vec::new(),
            connections: Vec::new(),
            validated_revisions: ValidatedRevisionJournal::default(),
        }
    }
}
