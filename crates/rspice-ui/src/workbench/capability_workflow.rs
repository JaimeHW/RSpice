//! Stable identities for deep-linked Feature Availability workflows.
//!
//! These action-level identities are deliberately separate from [`SurfaceId`]:
//! they are nested workflows owned by the canonical Feature Availability
//! manager, not additional entries in the governed 63-surface registry.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::surface_catalog::SurfaceId;

/// Immutable metadata for one deep-linked capability workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityWorkflowMetadata {
    pub id: CapabilityWorkflowId,
    pub stable_id: &'static str,
    pub label: &'static str,
}

macro_rules! define_capability_workflows {
    (
        $(
            $variant:ident => {
                id: $id:literal,
                label: $label:literal
            }
        ),+ $(,)?
    ) => {
        /// Stable identity for every workflow deep-linked from Feature Availability.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum CapabilityWorkflowId {
            $($variant),+
        }

        impl CapabilityWorkflowId {
            /// Every workflow in exact mockup order: eleven planned capability
            /// designs followed by the interoperability, touch-editing, and
            /// platform-lifecycle inspection documents.
            pub const ALL: [Self; 14] = [$(Self::$variant),+];

            #[must_use]
            pub const fn metadata(self) -> CapabilityWorkflowMetadata {
                match self {
                    $(
                        Self::$variant => CapabilityWorkflowMetadata {
                            id: Self::$variant,
                            stable_id: $id,
                            label: $label,
                        }
                    ),+
                }
            }

            #[must_use]
            pub const fn as_str(self) -> &'static str {
                self.metadata().stable_id
            }

            #[must_use]
            pub const fn label(self) -> &'static str {
                self.metadata().label
            }

            /// Canonical surface that owns and renders this nested workflow.
            #[must_use]
            pub const fn owner_surface(self) -> SurfaceId {
                SurfaceId::FeatureAvailability
            }

            /// Exact browser route spelling declared by the mockup.
            #[must_use]
            pub const fn deep_link(self) -> &'static str {
                match self {
                    $(Self::$variant => concat!("?surface=", $id)),+
                }
            }
        }

        impl FromStr for CapabilityWorkflowId {
            type Err = CapabilityWorkflowIdParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($id => Ok(Self::$variant),)+
                    _ => Err(CapabilityWorkflowIdParseError::Unknown(value.to_owned())),
                }
            }
        }
    };
}

define_capability_workflows! {
    GenericMultidimensionalRunsetController => {
        id: "generic-multidimensional-runset-controller",
        label: "Generic multidimensional run-set controller"
    },
    SourceLoadPullAnalysis => {
        id: "source-load-pull-analysis",
        label: "Source/load-pull analysis"
    },
    XParameterGeneration => {
        id: "x-parameter-generation",
        label: "X-parameter generation"
    },
    ModulatedSignalCharacterization => {
        id: "modulated-signal-characterization",
        label: "Modulated-signal characterization"
    },
    CalibrationDeembeddingPlan => {
        id: "calibration-deembedding-plan",
        label: "Calibration and de-embedding plan"
    },
    StatisticalDistributionAuthoring => {
        id: "statistical-distribution-authoring",
        label: "Statistical distribution authoring"
    },
    ErcPerc => {
        id: "erc-perc",
        label: "ERC/PERC setup and review"
    },
    AntennaChecks => {
        id: "antenna-checks",
        label: "Antenna-check setup and review"
    },
    DensityFillDfm => {
        id: "density-fill-dfm",
        label: "Density, fill, and DFM"
    },
    GdsOasisStreamOut => {
        id: "gds-oasis-stream-out",
        label: "GDSII/OASIS stream-out"
    },
    TapeoutAssemblyHandoff => {
        id: "tapeout-assembly-handoff",
        label: "Tapeout assembly and foundry handoff"
    },
    InteroperabilityMatrix => {
        id: "interoperability-matrix",
        label: "Interoperability and format matrix"
    },
    TouchEditGuide => {
        id: "touch-edit-guide",
        label: "Touch schematic editing"
    },
    PlatformLifecycle => {
        id: "platform-lifecycle",
        label: "Browser and mobile lifecycle contract"
    },
}

impl fmt::Display for CapabilityWorkflowId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Serialize for CapabilityWorkflowId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CapabilityWorkflowId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CapabilityWorkflowIdParseError {
    #[error("unknown Feature Availability workflow ID `{0}`")]
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn catalog_is_exact_unique_and_owned_by_feature_availability() {
        let expected_ids = [
            "generic-multidimensional-runset-controller",
            "source-load-pull-analysis",
            "x-parameter-generation",
            "modulated-signal-characterization",
            "calibration-deembedding-plan",
            "statistical-distribution-authoring",
            "erc-perc",
            "antenna-checks",
            "density-fill-dfm",
            "gds-oasis-stream-out",
            "tapeout-assembly-handoff",
            "interoperability-matrix",
            "touch-edit-guide",
            "platform-lifecycle",
        ];

        assert_eq!(CapabilityWorkflowId::ALL.len(), 14);
        assert_eq!(
            CapabilityWorkflowId::ALL.map(CapabilityWorkflowId::as_str),
            expected_ids
        );
        assert_eq!(
            CapabilityWorkflowId::ALL
                .iter()
                .map(|workflow| workflow.as_str())
                .collect::<HashSet<_>>()
                .len(),
            14
        );
        assert_eq!(
            CapabilityWorkflowId::ALL
                .iter()
                .map(|workflow| workflow.deep_link())
                .collect::<HashSet<_>>()
                .len(),
            14
        );
        assert!(
            CapabilityWorkflowId::ALL
                .into_iter()
                .all(|workflow| workflow.owner_surface() == SurfaceId::FeatureAvailability)
        );
        assert_eq!(SurfaceId::ALL.len(), 63);
        assert!(
            CapabilityWorkflowId::ALL
                .into_iter()
                .all(|workflow| workflow.as_str().parse::<SurfaceId>().is_err())
        );
    }

    #[test]
    fn exact_text_and_serde_roundtrip_fail_closed() {
        assert_eq!(
            CapabilityWorkflowId::TouchEditGuide.deep_link(),
            "?surface=touch-edit-guide"
        );
        assert_eq!(
            CapabilityWorkflowId::TouchEditGuide.label(),
            "Touch schematic editing"
        );
        for workflow in CapabilityWorkflowId::ALL {
            let stable_id = workflow.as_str();
            assert_eq!(workflow.to_string(), stable_id);
            assert_eq!(stable_id.parse::<CapabilityWorkflowId>(), Ok(workflow));
            assert_eq!(workflow.deep_link(), format!("?surface={stable_id}"));

            let encoded = serde_json::to_string(&workflow).expect("workflow ID serializes");
            assert_eq!(encoded, format!("\"{stable_id}\""));
            assert_eq!(
                serde_json::from_str::<CapabilityWorkflowId>(&encoded)
                    .expect("workflow ID deserializes"),
                workflow
            );
        }

        for invalid in [
            "",
            "Interoperability-Matrix",
            "interoperability_matrix",
            "interoperability-matrix ",
            "touch-editing-guide",
            "source-load-pull",
            "feature-availability",
        ] {
            assert!(invalid.parse::<CapabilityWorkflowId>().is_err());
            assert!(
                serde_json::from_str::<CapabilityWorkflowId>(&format!("\"{invalid}\"")).is_err()
            );
        }
    }
}
