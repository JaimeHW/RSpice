//! Device-local schematic display policy.
//!
//! These are the six canvas display controls plus the grid style. They are
//! persisted presentation choices about a design, with no dependency on the
//! chrome that renders the controls, so the schematic model can consult them
//! directly when deciding what a component's labels say.

use serde::{Deserialize, Serialize};

pub use crate::view::grid::GridStyle;

/// Device-local construction layers drawn around the permanent schematic
/// paper boundary.
///
/// The paper edge itself is intentionally absent: it is authored document
/// geometry and cannot be hidden. These switches only suppress supporting
/// editing detail and never affect project data, print, or export policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DrawingSheetLayerVisibility {
    pub zones: bool,
    pub title_block: bool,
    /// ISO 5457 centring marks and ASME Y14.1 microfilm arrows. A session
    /// saved when the layer also held corner registration and fold marks
    /// keeps its choice.
    #[serde(alias = "registration_marks")]
    pub centring_marks: bool,
    pub dimensions_at_detail_zoom: bool,
}

impl Default for DrawingSheetLayerVisibility {
    fn default() -> Self {
        Self {
            zones: true,
            title_block: true,
            centring_marks: true,
            dimensions_at_detail_zoom: true,
        }
    }
}

/// Mutually exclusive canvas annotation layer selected by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicAnnotationVisibility {
    #[default]
    OperatingPoint,
    ViolationsOnly,
    Hidden,
}

/// Quantities rendered from the explicitly selected retained result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicBackAnnotationContent {
    #[default]
    NetVoltagesAndDeviceCurrents,
    VoltagesOnly,
    VoltagesCurrentsAndPower,
}

pub use rspice_design::schematic::component_display::SchematicParameterLabelVisibility;

/// Interactive wire and bus routing policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicWireRoutingStyle {
    #[default]
    Orthogonal,
    FortyFiveDegree,
    FreeAngle,
}

/// Temporary net-highlighting presentation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicNetHighlighting {
    /// The selected net's conductors in the open view. Each view keeps its
    /// own highlight; nothing carries it through an instance terminal into a
    /// child or back up to the parent.
    #[default]
    // The policy's earlier name promised a highlight across hierarchy levels;
    // a preference saved under it still reads.
    #[serde(alias = "selected-across-hierarchy")]
    SelectedNet,
    /// The selected net, carried through the instance terminal it reaches:
    /// descending into an instance lights the child's port net it drives,
    /// and ascending lights the parent net the port reaches. Each occurrence
    /// holds its own highlight, so two instances of one master never share
    /// one.
    SelectedNetAcrossHierarchy,
    NetClassColors,
    Off,
}

impl SchematicNetHighlighting {
    /// Whether the selected net's conductors are lit.
    pub const fn lights_selected_net(self) -> bool {
        matches!(self, Self::SelectedNet | Self::SelectedNetAcrossHierarchy)
    }
}

/// Governed design-review records rendered on the canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicReviewMarkerVisibility {
    #[default]
    OpenAndAssigned,
    All,
    Hidden,
}

/// Complete device-local schematic display policy.
///
/// Keeping all six controls in one value gives Apply/Cancel true
/// transactional semantics and prevents partial frame-by-frame publication.
///
/// Earlier builds also saved a `hierarchy` choice for a dimmed parent drawn
/// under an edit-in-place child. That overlay is gone (a child sheet's
/// coordinates have no defined map onto its instance footprint), and so is the
/// choice; a policy saved with it still loads, because unknown fields are
/// ignored here rather than refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SchematicVisibilityPolicy {
    pub annotations: SchematicAnnotationVisibility,
    pub back_annotation: SchematicBackAnnotationContent,
    pub parameter_labels: SchematicParameterLabelVisibility,
    pub wire_routing: SchematicWireRoutingStyle,
    pub net_highlighting: SchematicNetHighlighting,
    pub review_markers: SchematicReviewMarkerVisibility,
}
