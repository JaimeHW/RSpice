//! Canonical Visualization Studio viewer metadata.
//!
//! The catalog mirrors the product manifest. Compatibility is deliberately
//! evaluated from an explicit caller-provided capability snapshot so catalog
//! discovery cannot silently assume that a result kind or external producer is
//! available.

use std::borrow::Cow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ViewerGroup {
    TimeAndFrequency,
    Photonics,
    RfAndNetwork,
    StatisticalAndTabular,
    SerialLink,
    DigitalAndAms,
    VerificationAndOptimization,
    Specialized,
    FieldsAndPhysical,
}

impl ViewerGroup {
    pub const ALL: [Self; 9] = [
        Self::TimeAndFrequency,
        Self::Photonics,
        Self::RfAndNetwork,
        Self::StatisticalAndTabular,
        Self::SerialLink,
        Self::DigitalAndAms,
        Self::VerificationAndOptimization,
        Self::Specialized,
        Self::FieldsAndPhysical,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::TimeAndFrequency => "Time & frequency",
            Self::Photonics => "Photonics",
            Self::RfAndNetwork => "RF & network",
            Self::StatisticalAndTabular => "Statistical & tabular",
            Self::SerialLink => "Serial link",
            Self::DigitalAndAms => "Digital & AMS",
            Self::VerificationAndOptimization => "Verification & optimization",
            Self::Specialized => "Specialized",
            Self::FieldsAndPhysical => "Fields & physical",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ViewerArt {
    Wave,
    Bode,
    Spectrum,
    Phase,
    Field,
    Smith,
    Polar,
    Contour,
    Wireless,
    Table,
    Histogram,
    Scatter,
    Eye,
    Bathtub,
    Margin,
    DigitalEvents,
    Soa,
    Reliability,
    Optimization,
    PoleZero,
    Thermal,
    Mesh,
}

impl ViewerArt {
    #[must_use]
    #[cfg(test)]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Wave => "wave",
            Self::Bode => "bode",
            Self::Spectrum => "spectrum",
            Self::Phase => "phase",
            Self::Field => "field",
            Self::Smith => "smith",
            Self::Polar => "polar",
            Self::Contour => "contour",
            Self::Wireless => "wireless",
            Self::Table => "table",
            Self::Histogram => "histogram",
            Self::Scatter => "scatter",
            Self::Eye => "eye",
            Self::Bathtub => "bathtub",
            Self::Margin => "margin",
            Self::DigitalEvents => "digital-events",
            Self::Soa => "soa",
            Self::Reliability => "reliability",
            Self::Optimization => "optimization",
            Self::PoleZero => "pz",
            Self::Thermal => "thermal",
            Self::Mesh => "mesh",
        }
    }
}

/// What the product manifest promises about a viewer for *this* release.
///
/// The catalog publishes the whole designed set, most of which no build draws
/// yet. Without this, a view waiting on its own release and a view waiting on
/// the user's data are one undifferentiated "unavailable", and a reader cannot
/// tell which of the two they are looking at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ViewerReleaseClass {
    /// In scope for this release. [`VIEWER_DOCUMENTS`] entries carrying this
    /// must have a renderer — a workspace sheet answers for every one of them.
    ReleaseTarget,
    /// Designed and published, scheduled after this release.
    ReleasePlanned,
    /// Designed for setup and review before it is separately qualified.
    Preview,
    /// Designed, and explicitly out of release scope until reactivated.
    Deferred,
    /// Satisfied first by a qualified external producer, not by this engine.
    QualifiedExternalFirst,
}

impl ViewerReleaseClass {
    /// Why a view with this classification cannot be opened, in the voice the
    /// rest of the product uses for a capability the binary does not carry.
    /// Never phrased as something the reader could correct by producing data.
    #[must_use]
    pub const fn unavailable_reason(self) -> &'static str {
        match self {
            // Reached only if a release-target view lost its renderer, which
            // `every_release_target_viewer_document_ships_a_renderer` forbids.
            Self::ReleaseTarget => "This view is unavailable in this build",
            Self::ReleasePlanned => "Planned for a release after this one",
            Self::Preview => "Designed for preview and not yet qualified to draw",
            Self::Deferred => "Deferred: out of release scope until reactivated",
            Self::QualifiedExternalFirst => {
                "Drawn from a qualified external producer this build does not carry"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewerDocumentDefinition {
    pub id: &'static str,
    pub group: ViewerGroup,
    pub title: &'static str,
    pub domain: &'static str,
    pub x_axis: &'static str,
    pub y_axis: &'static str,
    pub art: ViewerArt,
    /// Analysis kinds accepted by this viewer. A non-empty list is any-of.
    pub analysis_ids: &'static [&'static str],
    /// Qualified producer capability required by externally owned viewers.
    pub external_capability: Option<&'static str>,
    /// Release scope from the product manifest's capability record.
    pub release: ViewerReleaseClass,
    /// What has to exist before a deferred view is worth building. A
    /// deferral without one is an indefinite one; carrying the trigger
    /// on the row is what stops "deferred" from meaning "abandoned".
    pub deferral_trigger: Option<&'static str>,
}

impl ViewerDocumentDefinition {
    /// Why this view cannot be opened, said in the row the reader is
    /// looking at. A deferred view that names its trigger says what would
    /// bring it back; every other row falls through to the sentence its
    /// release class owns.
    #[must_use]
    pub fn unavailable_reason(&self) -> Cow<'static, str> {
        match self.deferral_trigger {
            Some(trigger) => Cow::Owned(format!("Deferred until {trigger}")),
            None => Cow::Borrowed(self.release.unavailable_reason()),
        }
    }
}

/// Canonical result-document creation family generated with the viewer
/// catalog. Keeping membership beside the viewer inventory prevents the
/// creation dialog from maintaining a second, drifting classification table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResultCreationFamilyDefinition {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_viewer_id: Option<&'static str>,
    pub viewer_ids: &'static [&'static str],
    pub quick_mode_ids: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ViewerCapabilities<'a> {
    pub analysis_ids: &'a [&'a str],
    pub external_capabilities: &'a [&'a str],
}

impl ViewerCapabilities<'_> {
    #[must_use]
    pub fn supports_analysis(self, analysis_id: &str) -> bool {
        self.analysis_ids.contains(&analysis_id)
    }

    #[must_use]
    pub fn supports_external_capability(self, capability_id: &str) -> bool {
        self.external_capabilities.contains(&capability_id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewerCompatibility {
    Compatible,
    UnknownDocument,
    MissingAnalysis {
        accepted_analysis_ids: &'static [&'static str],
    },
    MissingExternalCapability {
        capability_id: &'static str,
    },
}

impl ViewerCompatibility {
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }
}

include!(concat!(env!("OUT_DIR"), "/results_contract.rs"));

const _: [(); CANONICAL_VIEWER_COUNT] = [(); VIEWER_DOCUMENTS.len()];
const _: [(); CANONICAL_CREATION_FAMILY_COUNT] = [(); RESULT_CREATION_FAMILIES.len()];

#[must_use]
pub fn result_creation_family(id: &str) -> Option<&'static ResultCreationFamilyDefinition> {
    RESULT_CREATION_FAMILIES
        .iter()
        .find(|family| family.id == id)
}

#[must_use]
pub fn viewer_document(id: &str) -> Option<&'static ViewerDocumentDefinition> {
    VIEWER_DOCUMENTS.iter().find(|document| document.id == id)
}

#[must_use]
pub fn viewer_compatibility(
    document_id: &str,
    capabilities: ViewerCapabilities<'_>,
) -> ViewerCompatibility {
    let Some(document) = viewer_document(document_id) else {
        return ViewerCompatibility::UnknownDocument;
    };

    if let Some(capability_id) = document.external_capability {
        return if capabilities.supports_external_capability(capability_id) {
            ViewerCompatibility::Compatible
        } else {
            ViewerCompatibility::MissingExternalCapability { capability_id }
        };
    }

    analysis_compatibility(document.analysis_ids, capabilities)
}

fn analysis_compatibility(
    accepted_analysis_ids: &'static [&'static str],
    capabilities: ViewerCapabilities<'_>,
) -> ViewerCompatibility {
    if accepted_analysis_ids.is_empty()
        || accepted_analysis_ids
            .iter()
            .any(|analysis_id| capabilities.supports_analysis(analysis_id))
    {
        ViewerCompatibility::Compatible
    } else {
        ViewerCompatibility::MissingAnalysis {
            accepted_analysis_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const DOCUMENT_IDS: &[&str] = &[
        "viewer-waveform",
        "viewer-bode",
        "viewer-spectrum",
        "viewer-phase-noise",
        "viewer-spectrogram",
        "viewer-optical-spectrum",
        "viewer-optical-transfer",
        "viewer-optical-mode",
        "viewer-smith",
        "viewer-polar",
        "viewer-load-pull",
        "viewer-wireless",
        "viewer-network-quality",
        "viewer-mixed-mode-network",
        "viewer-constellation",
        "viewer-ccdf",
        "viewer-tdr",
        "viewer-transfer-function",
        "viewer-table",
        "viewer-histogram",
        "viewer-scatter",
        "viewer-contour",
        "viewer-box-violin",
        "viewer-parallel-coordinates",
        "viewer-scatter-matrix",
        "viewer-contribution",
        "eye-viewer",
        "bathtub-viewer",
        "margin-viewer",
        "dynamic-droop-viewer",
        "viewer-digital-events",
        "viewer-soa",
        "viewer-reliability",
        "viewer-optimization",
        "viewer-pz",
        "field-viewer-3d",
        "field-viewer-current",
        "field-viewer-voltage",
        "field-viewer-thermal",
        "field-viewer-mesh",
        "field-viewer-probe",
    ];

    #[test]
    fn catalog_has_exact_manifest_document_ids_and_order() {
        assert_eq!(VIEWER_DOCUMENTS.len(), CANONICAL_VIEWER_COUNT);
        assert_eq!(CANONICAL_VIEWER_COUNT, 41);
        assert_eq!(
            VIEWER_DOCUMENTS
                .iter()
                .map(|document| document.id)
                .collect::<Vec<_>>(),
            DOCUMENT_IDS
        );
    }

    #[test]
    fn stable_ids_are_unique_and_lookups_are_total_for_catalog_entries() {
        let mut document_ids = HashSet::new();
        for document in VIEWER_DOCUMENTS {
            assert!(document_ids.insert(document.id), "{}", document.id);
            assert_eq!(viewer_document(document.id), Some(document));
        }
    }

    #[test]
    fn groups_have_exact_manifest_labels_order_and_document_counts() {
        assert_eq!(
            ViewerGroup::ALL.map(ViewerGroup::label),
            [
                "Time & frequency",
                "Photonics",
                "RF & network",
                "Statistical & tabular",
                "Serial link",
                "Digital & AMS",
                "Verification & optimization",
                "Specialized",
                "Fields & physical",
            ]
        );
        assert_eq!(
            ViewerGroup::ALL.map(|group| VIEWER_DOCUMENTS
                .iter()
                .filter(|document| document.group == group)
                .count()),
            [5, 3, 9, 9, 4, 1, 3, 1, 6]
        );
    }

    #[test]
    fn creation_families_are_generated_from_the_same_contract() {
        assert_eq!(
            RESULT_CREATION_FAMILIES.len(),
            CANONICAL_CREATION_FAMILY_COUNT
        );
        assert_eq!(CANONICAL_CREATION_FAMILY_COUNT, 9);
        assert_eq!(
            RESULT_CREATION_FAMILIES
                .iter()
                .map(|family| family.id)
                .collect::<Vec<_>>(),
            [
                "waveform-worksheet",
                "frequency-stability",
                "rf-network",
                "statistics-yield",
                "digital-ams-events",
                "verification-optimization",
                "fields-physical",
                "photonics",
                "report-page",
            ]
        );
        for family in RESULT_CREATION_FAMILIES {
            assert!(!family.label.is_empty());
            assert!(!family.description.is_empty());
            for viewer_id in family.viewer_ids {
                assert!(
                    viewer_document(viewer_id).is_some(),
                    "{} references unknown viewer {viewer_id}",
                    family.id
                );
            }
        }
    }

    #[test]
    fn all_metadata_is_complete_and_requirement_ownership_is_unambiguous() {
        for document in VIEWER_DOCUMENTS {
            assert!(!document.id.is_empty());
            assert!(!document.title.is_empty());
            assert!(!document.domain.is_empty());
            assert!(!document.x_axis.is_empty());
            assert!(!document.y_axis.is_empty());
            assert!(!document.group.label().is_empty());
            assert!(!document.art.id().is_empty());
            assert!(
                document.analysis_ids.is_empty() || document.external_capability.is_none(),
                "{} mixes internal and external requirements",
                document.id
            );
        }
    }

    #[test]
    fn compatibility_is_any_of_for_analysis_backed_viewers() {
        let empty = ViewerCapabilities::default();
        assert_eq!(
            viewer_compatibility("viewer-bode", empty),
            ViewerCompatibility::MissingAnalysis {
                accepted_analysis_ids: &["ac", "stb", "noise", "pac", "pstb", "qpac"],
            }
        );

        let stb = ViewerCapabilities {
            analysis_ids: &["stb"],
            external_capabilities: &[],
        };
        assert_eq!(
            viewer_compatibility("viewer-bode", stb),
            ViewerCompatibility::Compatible
        );
    }

    #[test]
    fn external_viewers_require_the_exact_qualified_capability() {
        let wrong = ViewerCapabilities {
            analysis_ids: &[],
            external_capabilities: &["em"],
        };
        assert_eq!(
            viewer_compatibility("viewer-optical-mode", wrong),
            ViewerCompatibility::MissingExternalCapability {
                capability_id: "photonics",
            }
        );

        let photonics = ViewerCapabilities {
            analysis_ids: &[],
            external_capabilities: &["photonics"],
        };
        assert!(viewer_compatibility("viewer-optical-mode", photonics).is_compatible());
    }

    #[test]
    fn table_requires_a_canonical_typed_tabular_result() {
        let empty = ViewerCapabilities::default();
        assert_eq!(
            viewer_compatibility("viewer-table", empty),
            ViewerCompatibility::MissingAnalysis {
                accepted_analysis_ids: &["op", "temp", "pss", "pstb"],
            }
        );
        assert!(
            viewer_compatibility(
                "viewer-table",
                ViewerCapabilities {
                    analysis_ids: &["op"],
                    external_capabilities: &[],
                },
            )
            .is_compatible()
        );
        assert!(
            viewer_compatibility(
                "viewer-table",
                ViewerCapabilities {
                    analysis_ids: &["pstb"],
                    external_capabilities: &[],
                },
            )
            .is_compatible()
        );
    }

    #[test]
    fn unknown_ids_fail_closed() {
        let all_plausible_capabilities = ViewerCapabilities {
            analysis_ids: &[
                "tran", "dc", "ac", "stb", "noise", "fourier", "hb", "pss", "pnoise", "qpnoise",
                "envelope", "sp", "hbsp", "psp", "hbnoise", "mc", "corner", "sens", "dcmatch",
                "op", "pz",
            ],
            external_capabilities: &["photonics", "em", "em-ir", "electrothermal"],
        };
        assert_eq!(
            viewer_compatibility("viewer-does-not-exist", all_plausible_capabilities),
            ViewerCompatibility::UnknownDocument
        );
    }

    #[test]
    fn document_group_iteration_preserves_manifest_order() {
        assert_eq!(
            VIEWER_DOCUMENTS
                .iter()
                .filter(|document| document.group == ViewerGroup::Photonics)
                .map(|document| document.id)
                .collect::<Vec<_>>(),
            [
                "viewer-optical-spectrum",
                "viewer-optical-transfer",
                "viewer-optical-mode",
            ]
        );
    }
}
