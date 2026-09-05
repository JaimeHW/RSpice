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

// Retained temporarily as a disabled reference while the generated catalog is
// reviewed. The build consumes the neutral contract below; this block cannot
// become an independent runtime authority.
#[cfg(any())]
mod legacy_catalog {
    use super::*;

    const TIME_DC: &[&str] = &["tran", "pss", "envelope", "soa", "dc"];
    const FREQUENCY_RESPONSE_NOISE: &[&str] =
        &["ac", "pac", "pxf", "stb", "disto", "noise", "hbnoise"];
    const FOURIER_TRAN_HB_PSS: &[&str] = &["fourier", "tran", "hb", "pss"];
    const PNOISE_QPNOISE: &[&str] = &["pnoise", "qpnoise"];
    const TRAN_FOURIER_HB_ENVELOPE: &[&str] = &["tran", "fourier", "hb", "envelope"];
    const SP_HBSP_PSP: &[&str] = &["sp", "hbsp", "psp"];
    const AC_SP_HB: &[&str] = &["ac", "sp", "hb"];
    const ENVELOPE_TRAN: &[&str] = &["envelope", "tran"];
    const SP_TRAN: &[&str] = &["sp", "tran"];
    const MC_CORNER: &[&str] = &["mc", "corner"];
    const DC_MC_CORNER: &[&str] = &["dc", "mc", "corner"];
    const SENS_DCMATCH: &[&str] = &["sens", "dcmatch"];
    const TRAN_CORNER: &[&str] = &["tran", "corner"];

    pub const VIEWER_DOCUMENTS: &[ViewerDocumentDefinition] = &[
        ViewerDocumentDefinition {
            id: "viewer-waveform",
            group: ViewerGroup::TimeAndFrequency,
            title: "Waveform",
            domain: "TRAN · PSS · ENVELOPE · exact samples",
            x_axis: "Time · ms",
            y_axis: "Voltage · V",
            art: ViewerArt::Wave,
            analysis_ids: TIME_DC,
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "viewer-bode",
            group: ViewerGroup::TimeAndFrequency,
            title: "Frequency response / noise",
            domain: "AC · PAC · PXF · STB · DISTO · NOISE · HBNOISE",
            x_axis: "Frequency · Hz",
            y_axis: "Magnitude · dB / phase · ° / noise density",
            art: ViewerArt::Bode,
            analysis_ids: FREQUENCY_RESPONSE_NOISE,
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "viewer-spectrum",
            group: ViewerGroup::TimeAndFrequency,
            title: "Spectrum / sidebands",
            domain: "FOUR · HB · PSS",
            x_axis: "Frequency · Hz",
            y_axis: "Amplitude · dBc",
            art: ViewerArt::Spectrum,
            analysis_ids: FOURIER_TRAN_HB_PSS,
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "viewer-phase-noise",
            group: ViewerGroup::TimeAndFrequency,
            title: "Phase noise / jitter",
            domain: "PNOISE · oscillator",
            x_axis: "Offset frequency · Hz",
            y_axis: "L(f) · dBc/Hz",
            art: ViewerArt::Phase,
            analysis_ids: PNOISE_QPNOISE,
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "viewer-spectrogram",
            group: ViewerGroup::TimeAndFrequency,
            title: "Spectrogram / waterfall",
            domain: "TRAN · FOUR · HB · envelope",
            x_axis: "Time / sweep family",
            y_axis: "Frequency · Hz",
            art: ViewerArt::Spectrum,
            analysis_ids: TRAN_FOURIER_HB_ENVELOPE,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-optical-spectrum",
            group: ViewerGroup::Photonics,
            title: "Optical spectrum",
            domain: "Wavelength sweep · optical power",
            x_axis: "Wavelength · nm",
            y_axis: "Power · dBm",
            art: ViewerArt::Spectrum,
            analysis_ids: &[],
            external_capability: Some("photonics"),
            release: ViewerReleaseClass::Deferred,
        },
        ViewerDocumentDefinition {
            id: "viewer-optical-transfer",
            group: ViewerGroup::Photonics,
            title: "Electro-optical transfer",
            domain: "Electrical drive · optical response",
            x_axis: "Frequency · Hz",
            y_axis: "Magnitude / phase",
            art: ViewerArt::Bode,
            analysis_ids: &[],
            external_capability: Some("photonics"),
            release: ViewerReleaseClass::Deferred,
        },
        ViewerDocumentDefinition {
            id: "viewer-optical-mode",
            group: ViewerGroup::Photonics,
            title: "Optical mode profile",
            domain: "Waveguide eigenmode · field distribution",
            x_axis: "Cross-section X",
            y_axis: "Cross-section Y",
            art: ViewerArt::Field,
            analysis_ids: &[],
            external_capability: Some("photonics"),
            release: ViewerReleaseClass::Deferred,
        },
        ViewerDocumentDefinition {
            id: "viewer-smith",
            group: ViewerGroup::RfAndNetwork,
            title: "Smith chart",
            domain: "SP · HBSP · PSP",
            x_axis: "Normalized resistance",
            y_axis: "Normalized reactance",
            art: ViewerArt::Smith,
            analysis_ids: SP_HBSP_PSP,
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "viewer-polar",
            group: ViewerGroup::RfAndNetwork,
            title: "Polar response",
            domain: "Complex result family",
            x_axis: "Real",
            y_axis: "Imaginary",
            art: ViewerArt::Polar,
            analysis_ids: AC_SP_HB,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-load-pull",
            group: ViewerGroup::RfAndNetwork,
            title: "Load-pull contours",
            domain: "HB · ΓL sweep",
            x_axis: "Γ real",
            y_axis: "Γ imaginary",
            art: ViewerArt::Contour,
            analysis_ids: &["hb"],
            external_capability: None,
            release: ViewerReleaseClass::Preview,
        },
        ViewerDocumentDefinition {
            id: "viewer-wireless",
            group: ViewerGroup::RfAndNetwork,
            title: "Wireless quality",
            domain: "Envelope · modulated",
            x_axis: "Time / symbol",
            y_axis: "EVM · ACPR · mask",
            art: ViewerArt::Wireless,
            analysis_ids: &["envelope"],
            external_capability: None,
            release: ViewerReleaseClass::Preview,
        },
        ViewerDocumentDefinition {
            id: "viewer-network-quality",
            group: ViewerGroup::RfAndNetwork,
            title: "Noise figure / gain circles",
            domain: "SP · HBSP · PSP",
            x_axis: "Frequency · Hz",
            y_axis: "Gain / NF / stability",
            art: ViewerArt::Bode,
            analysis_ids: SP_HBSP_PSP,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-mixed-mode-network",
            group: ViewerGroup::RfAndNetwork,
            title: "Mixed-mode network matrix",
            domain: "Single-ended and mode-converted S-parameters",
            x_axis: "Frequency · Hz",
            y_axis: "SDD / SDC / SCD / SCC",
            art: ViewerArt::Table,
            analysis_ids: SP_HBSP_PSP,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-constellation",
            group: ViewerGroup::RfAndNetwork,
            title: "Constellation / EVM",
            domain: "Envelope · demodulated symbols",
            x_axis: "In-phase",
            y_axis: "Quadrature",
            art: ViewerArt::Wireless,
            analysis_ids: &["envelope"],
            external_capability: None,
            release: ViewerReleaseClass::Preview,
        },
        ViewerDocumentDefinition {
            id: "viewer-ccdf",
            group: ViewerGroup::RfAndNetwork,
            title: "CCDF / crest factor",
            domain: "Envelope or transient amplitude statistics",
            x_axis: "Power above average · dB",
            y_axis: "Probability",
            art: ViewerArt::Histogram,
            analysis_ids: ENVELOPE_TRAN,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-tdr",
            group: ViewerGroup::RfAndNetwork,
            title: "TDR / TDT",
            domain: "SP transform or transient step response",
            x_axis: "Time / distance",
            y_axis: "Impedance / transmission",
            art: ViewerArt::Wave,
            analysis_ids: SP_TRAN,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-transfer-function",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Transfer function",
            domain: "XF · DC operating point",
            x_axis: "Declared source / output",
            y_axis: "Gain and resistance",
            art: ViewerArt::Table,
            analysis_ids: &["xf"],
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "viewer-table",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Tabular datasheet",
            domain: "Measurements · specifications",
            x_axis: "Declared dimensions",
            y_axis: "Typed values",
            art: ViewerArt::Table,
            analysis_ids: &[],
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "viewer-histogram",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Histogram / CDF",
            domain: "MC · samples",
            x_axis: "Measured value",
            y_axis: "Count / probability",
            art: ViewerArt::Histogram,
            analysis_ids: MC_CORNER,
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "viewer-scatter",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Scatter / correlation",
            domain: "MC · paired measures",
            x_axis: "Input measure",
            y_axis: "Output measure",
            art: ViewerArt::Scatter,
            analysis_ids: &["mc"],
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-contour",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Contour / surface",
            domain: "2D sweep · scalar result",
            x_axis: "Sweep axis 1",
            y_axis: "Sweep axis 2",
            art: ViewerArt::Contour,
            analysis_ids: DC_MC_CORNER,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-box-violin",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Box / violin distribution",
            domain: "MC · corner families",
            x_axis: "Family / category",
            y_axis: "Measured value",
            art: ViewerArt::Histogram,
            analysis_ids: MC_CORNER,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-parallel-coordinates",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Parallel coordinates",
            domain: "MC · multivariate samples",
            x_axis: "Parameter / measurement axis",
            y_axis: "Normalized value",
            art: ViewerArt::Scatter,
            analysis_ids: &["mc"],
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-scatter-matrix",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Scatter matrix",
            domain: "MC · paired variables",
            x_axis: "Selected variables",
            y_axis: "Selected variables",
            art: ViewerArt::Scatter,
            analysis_ids: &["mc"],
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-contribution",
            group: ViewerGroup::StatisticalAndTabular,
            title: "Sensitivity / mismatch contribution",
            domain: "SENS · DCMATCH",
            x_axis: "Parameter",
            y_axis: "Normalized contribution",
            art: ViewerArt::Histogram,
            analysis_ids: SENS_DCMATCH,
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "eye-viewer",
            group: ViewerGroup::SerialLink,
            title: "Eye diagram",
            domain: "Bit-aligned waveform",
            x_axis: "Unit interval",
            y_axis: "Voltage · V",
            art: ViewerArt::Eye,
            analysis_ids: &["tran"],
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "bathtub-viewer",
            group: ViewerGroup::SerialLink,
            title: "Bathtub / BER",
            domain: "Statistical timing",
            x_axis: "Sampling phase · UI",
            y_axis: "BER",
            art: ViewerArt::Bathtub,
            analysis_ids: &["tran"],
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "margin-viewer",
            group: ViewerGroup::SerialLink,
            title: "Timing / voltage margin",
            domain: "Eye mask · corners",
            x_axis: "Sampling phase · UI",
            y_axis: "Voltage · V",
            art: ViewerArt::Margin,
            analysis_ids: TRAN_CORNER,
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "dynamic-droop-viewer",
            group: ViewerGroup::SerialLink,
            title: "Dynamic supply droop",
            domain: "Transient PDN",
            x_axis: "Time · ns",
            y_axis: "Supply · mV",
            art: ViewerArt::Wave,
            analysis_ids: &["tran"],
            external_capability: None,
            release: ViewerReleaseClass::ReleasePlanned,
        },
        ViewerDocumentDefinition {
            id: "viewer-pz",
            group: ViewerGroup::Specialized,
            title: "Pole-zero map",
            domain: "PZ · complex plane",
            x_axis: "Real frequency",
            y_axis: "Imaginary frequency",
            art: ViewerArt::PoleZero,
            analysis_ids: &["pz"],
            external_capability: None,
            release: ViewerReleaseClass::ReleaseTarget,
        },
        ViewerDocumentDefinition {
            id: "field-viewer-3d",
            group: ViewerGroup::FieldsAndPhysical,
            title: "3D electromagnetic field",
            domain: "EM · vector field",
            x_axis: "Geometry X",
            y_axis: "Geometry Y / Z",
            art: ViewerArt::Field,
            analysis_ids: &[],
            external_capability: Some("em"),
            release: ViewerReleaseClass::QualifiedExternalFirst,
        },
        ViewerDocumentDefinition {
            id: "field-viewer-current",
            group: ViewerGroup::FieldsAndPhysical,
            title: "Current density",
            domain: "EM / IR · conductor",
            x_axis: "Layout X",
            y_axis: "Layout Y",
            art: ViewerArt::Field,
            analysis_ids: &[],
            external_capability: Some("em-ir"),
            release: ViewerReleaseClass::QualifiedExternalFirst,
        },
        ViewerDocumentDefinition {
            id: "field-viewer-voltage",
            group: ViewerGroup::FieldsAndPhysical,
            title: "Voltage drop",
            domain: "IR · scalar field",
            x_axis: "Layout X",
            y_axis: "Layout Y",
            art: ViewerArt::Field,
            analysis_ids: &[],
            external_capability: Some("em-ir"),
            release: ViewerReleaseClass::QualifiedExternalFirst,
        },
        ViewerDocumentDefinition {
            id: "field-viewer-thermal",
            group: ViewerGroup::FieldsAndPhysical,
            title: "Thermal distribution",
            domain: "Electrothermal · scalar field",
            x_axis: "Geometry X",
            y_axis: "Geometry Y / Z",
            art: ViewerArt::Thermal,
            analysis_ids: &[],
            external_capability: Some("electrothermal"),
            release: ViewerReleaseClass::QualifiedExternalFirst,
        },
        ViewerDocumentDefinition {
            id: "field-viewer-mesh",
            group: ViewerGroup::FieldsAndPhysical,
            title: "Mesh quality",
            domain: "EM · finite elements",
            x_axis: "Geometry X",
            y_axis: "Geometry Y / Z",
            art: ViewerArt::Mesh,
            analysis_ids: &[],
            external_capability: Some("em"),
            release: ViewerReleaseClass::QualifiedExternalFirst,
        },
        ViewerDocumentDefinition {
            id: "field-viewer-probe",
            group: ViewerGroup::FieldsAndPhysical,
            title: "Field probe",
            domain: "EM · sampled path",
            x_axis: "Distance · µm",
            y_axis: "Field magnitude",
            art: ViewerArt::Wave,
            analysis_ids: &[],
            external_capability: Some("em"),
            release: ViewerReleaseClass::QualifiedExternalFirst,
        },
    ];
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
