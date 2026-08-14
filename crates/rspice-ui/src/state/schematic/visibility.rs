//! Device-local schematic display policy.
//!
//! These are the seven canvas display controls plus the grid style. They are
//! persisted presentation choices about a design, with no dependency on the
//! chrome that renders the controls, so the schematic model can consult them
//! directly when deciding what a component's labels say.

use serde::{Deserialize, Serialize};

/// Canvas grid rendering style selected by the toolbar cycle and the richer
/// canvas-settings transaction. Snapping remains independently configurable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GridStyle {
    /// One dot per snap point (default).
    #[default]
    Dots,
    /// Hairline rules per snap point.
    Lines,
    /// No grid.
    Off,
}

/// Device-local construction layers drawn around the permanent schematic
/// paper boundary.
///
/// The paper edge itself is intentionally absent: it is authored document
/// geometry and cannot be hidden. These switches only suppress supporting
/// editing detail and never affect project data, print, or export policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DrawingSheetLayerVisibility {
    pub margins: bool,
    pub zones: bool,
    pub title_block: bool,
    pub registration_marks: bool,
    pub dimensions_at_detail_zoom: bool,
}

impl Default for DrawingSheetLayerVisibility {
    fn default() -> Self {
        Self {
            margins: true,
            zones: true,
            title_block: true,
            registration_marks: true,
            dimensions_at_detail_zoom: true,
        }
    }
}

/// Device-local hierarchy context shown around the active schematic.
///
/// This is presentation state only. It never changes the cell/view binding
/// or serializes into project-owned design data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicHierarchyVisibility {
    #[default]
    ActiveAndParent,
    ActiveOnly,
    FullVisibleHierarchy,
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

/// Instance parameter-label detail rendered on the canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicParameterLabelVisibility {
    ValuesOnly,
    #[default]
    NamesAndValues,
    Hidden,
}

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
    #[default]
    SelectedAcrossHierarchy,
    NetClassColors,
    Off,
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
/// Keeping all seven mockup controls in one value gives Apply/Cancel true
/// transactional semantics and prevents partial frame-by-frame publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SchematicVisibilityPolicy {
    pub hierarchy: SchematicHierarchyVisibility,
    pub annotations: SchematicAnnotationVisibility,
    pub back_annotation: SchematicBackAnnotationContent,
    pub parameter_labels: SchematicParameterLabelVisibility,
    pub wire_routing: SchematicWireRoutingStyle,
    pub net_highlighting: SchematicNetHighlighting,
    pub review_markers: SchematicReviewMarkerVisibility,
}
