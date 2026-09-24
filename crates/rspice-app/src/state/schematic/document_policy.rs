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

// Page size and orientation are named by the persisted design-management
// schema and by the signed drawing-sheet package, so they are defined in
// `rspice-design-model` and re-exported from the module that owned them.
pub use rspice_design_model::{SchematicPageOrientation, SchematicPageSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicCustomPageSize {
    pub portrait_width_um: u64,
    pub portrait_height_um: u64,
}

impl<'de> Deserialize<'de> for SchematicCustomPageSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            portrait_width_um: u64,
            portrait_height_um: u64,
        }
        let wire = Wire::deserialize(deserializer)?;
        if !(10_000..=5_000_000).contains(&wire.portrait_width_um)
            || !(10_000..=5_000_000).contains(&wire.portrait_height_um)
            || wire.portrait_width_um > wire.portrait_height_um
        {
            return Err(serde::de::Error::custom(
                "custom schematic page dimensions must be canonical portrait dimensions from 10 mm through 5 m",
            ));
        }
        Ok(Self {
            portrait_width_um: wire.portrait_width_um,
            portrait_height_um: wire.portrait_height_um,
        })
    }
}

impl SchematicCustomPageSize {
    #[must_use]
    pub const fn normalized(width_um: u64, height_um: u64) -> Self {
        if width_um <= height_um {
            Self {
                portrait_width_um: width_um,
                portrait_height_um: height_um,
            }
        } else {
            Self {
                portrait_width_um: height_um,
                portrait_height_um: width_um,
            }
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
    /// Exact custom dimensions for legacy single-sheet documents. Governed
    /// multi-sheet documents retain this authority per `DesignSheet`.
    pub custom_page_size: Option<SchematicCustomPageSize>,
    pub page_orientation: SchematicPageOrientation,
}

impl SchematicDocumentPolicy {
    #[must_use]
    pub fn page_size_display(self) -> String {
        let (name, portrait_width_um, portrait_height_um) =
            if let Some(custom) = self.custom_page_size {
                (
                    "Custom",
                    custom.portrait_width_um,
                    custom.portrait_height_um,
                )
            } else {
                let (width, height) = self.page_size.portrait_dimensions_tenth_mm();
                (
                    self.page_size.label(),
                    u64::from(width) * 100,
                    u64::from(height) * 100,
                )
            };
        let (width, height) = match self.page_orientation {
            SchematicPageOrientation::Portrait => (portrait_width_um, portrait_height_um),
            SchematicPageOrientation::Landscape => (portrait_height_um, portrait_width_um),
        };
        format!(
            "{} {} · {} × {} mm",
            name,
            self.page_orientation.label(),
            format_micrometres_as_mm(width),
            format_micrometres_as_mm(height)
        )
    }
}

fn format_micrometres_as_mm(value: u64) -> String {
    let whole = value / 1_000;
    let fraction = value % 1_000;
    if fraction == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{fraction:03}")
            .trim_end_matches('0')
            .to_owned()
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
            custom_page_size: None,
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
        assert_eq!(policy.custom_page_size, None);
        assert_eq!(policy.page_orientation, SchematicPageOrientation::Landscape);
        assert_eq!(policy.page_size_display(), "A4 landscape · 297 × 210 mm");
    }

    #[test]
    fn directional_selection_matches_physical_editor_convention() {
        assert!(SelectionCrossingPolicy::Directional.enclosed_only(true));
        assert!(!SelectionCrossingPolicy::Directional.enclosed_only(false));
    }
}
