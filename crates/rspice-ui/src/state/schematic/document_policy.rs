//! Explicit project-portable schematic editor policy.
//!
//! User Preferences seed this policy for a newly created schematic. Once the
//! document exists these resolved values travel with it, so changing a user
//! default cannot silently reinterpret connectivity or naming in saved work.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicGridPitch {
    #[default]
    Mil50,
    Mil25,
    Metric,
}

impl SchematicGridPitch {
    #[must_use]
    pub const fn canvas_grid_size(self) -> i32 {
        match self {
            Self::Mil50 => 10,
            Self::Mil25 => 5,
            Self::Metric => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum WireJunctionPolicy {
    #[default]
    OrthogonalAutomatic,
    OrthogonalManual,
    AnyAngle,
}

impl WireJunctionPolicy {
    #[must_use]
    pub const fn automatic_junctions(self) -> bool {
        !matches!(self, Self::OrthogonalManual)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionCrossingPolicy {
    #[default]
    Directional,
    EnclosedOnly,
    Intersecting,
}

impl SelectionCrossingPolicy {
    #[must_use]
    pub const fn enclosed_only(self, left_to_right: bool) -> bool {
        match self {
            Self::Directional => left_to_right,
            Self::EnclosedOnly => true,
            Self::Intersecting => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NetNamingPolicy {
    #[default]
    StrictCaseSensitive,
    SpiceCompatibleRelaxed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PropertyCommitPolicy {
    #[default]
    Atomic,
    ApplyValidFields,
}

/// Default schematic annotation detail after a compatible operating-point run.
///
/// This belongs to the document policy rather than live user preferences so
/// opening an existing design cannot silently change its review presentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum OperatingPointAnnotationPolicy {
    #[default]
    VoltagesAndSelectedCurrents,
    VoltagesOnly,
    Hidden,
}

/// Physical drawing-sheet format retained with the schematic document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicPageSize {
    #[default]
    A4,
    A3,
    UsLetter,
    UsLedger,
}

impl SchematicPageSize {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::A4 => "A4",
            Self::A3 => "A3",
            Self::UsLetter => "US Letter",
            Self::UsLedger => "US Ledger",
        }
    }

    /// Nominal portrait dimensions in tenths of a millimetre.
    #[must_use]
    pub const fn portrait_dimensions_tenth_mm(self) -> (u32, u32) {
        match self {
            Self::A4 => (2_100, 2_970),
            Self::A3 => (2_970, 4_200),
            Self::UsLetter => (2_159, 2_794),
            Self::UsLedger => (2_794, 4_318),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SchematicPageOrientation {
    Portrait,
    #[default]
    Landscape,
}

impl SchematicPageOrientation {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Portrait => "portrait",
            Self::Landscape => "landscape",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SchematicDocumentPolicy {
    pub grid_pitch: SchematicGridPitch,
    pub wire_junctions: WireJunctionPolicy,
    pub selection_crossing: SelectionCrossingPolicy,
    pub net_naming: NetNamingPolicy,
    pub property_commit: PropertyCommitPolicy,
    pub operating_point_annotations: OperatingPointAnnotationPolicy,
    pub page_size: SchematicPageSize,
    pub page_orientation: SchematicPageOrientation,
}

impl SchematicDocumentPolicy {
    #[must_use]
    pub fn page_size_display(self) -> String {
        let (portrait_width, portrait_height) = self.page_size.portrait_dimensions_tenth_mm();
        let (width, height) = match self.page_orientation {
            SchematicPageOrientation::Portrait => (portrait_width, portrait_height),
            SchematicPageOrientation::Landscape => (portrait_height, portrait_width),
        };
        format!(
            "{} {} · {} × {} mm",
            self.page_size.label(),
            self.page_orientation.label(),
            format_tenth_mm(width),
            format_tenth_mm(height)
        )
    }
}

fn format_tenth_mm(value: u32) -> String {
    if value % 10 == 0 {
        (value / 10).to_string()
    } else {
        format!("{}.{:01}", value / 10, value % 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_policy_round_trips_with_stable_names() {
        let policy = SchematicDocumentPolicy {
            grid_pitch: SchematicGridPitch::Metric,
            wire_junctions: WireJunctionPolicy::OrthogonalManual,
            selection_crossing: SelectionCrossingPolicy::Intersecting,
            net_naming: NetNamingPolicy::SpiceCompatibleRelaxed,
            property_commit: PropertyCommitPolicy::ApplyValidFields,
            operating_point_annotations: OperatingPointAnnotationPolicy::VoltagesOnly,
            page_size: SchematicPageSize::A3,
            page_orientation: SchematicPageOrientation::Portrait,
        };
        let json = serde_json::to_string(&policy).unwrap();
        assert!(json.contains("orthogonal-manual"));
        assert_eq!(
            serde_json::from_str::<SchematicDocumentPolicy>(&json).unwrap(),
            policy
        );
    }

    #[test]
    fn legacy_document_policy_defaults_page_authority() {
        let policy = serde_json::from_str::<SchematicDocumentPolicy>("{}").unwrap();
        assert_eq!(policy.page_size, SchematicPageSize::A4);
        assert_eq!(policy.page_orientation, SchematicPageOrientation::Landscape);
        assert_eq!(policy.page_size_display(), "A4 landscape · 297 × 210 mm");
    }

    #[test]
    fn directional_selection_matches_physical_editor_convention() {
        assert!(SelectionCrossingPolicy::Directional.enclosed_only(true));
        assert!(!SelectionCrossingPolicy::Directional.enclosed_only(false));
    }
}
