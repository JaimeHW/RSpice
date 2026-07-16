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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SchematicDocumentPolicy {
    pub grid_pitch: SchematicGridPitch,
    pub wire_junctions: WireJunctionPolicy,
    pub selection_crossing: SelectionCrossingPolicy,
    pub net_naming: NetNamingPolicy,
    pub property_commit: PropertyCommitPolicy,
    pub operating_point_annotations: OperatingPointAnnotationPolicy,
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
        };
        let json = serde_json::to_string(&policy).unwrap();
        assert!(json.contains("orthogonal-manual"));
        assert_eq!(
            serde_json::from_str::<SchematicDocumentPolicy>(&json).unwrap(),
            policy
        );
    }

    #[test]
    fn directional_selection_matches_physical_editor_convention() {
        assert!(SelectionCrossingPolicy::Directional.enclosed_only(true));
        assert!(!SelectionCrossingPolicy::Directional.enclosed_only(false));
    }
}
