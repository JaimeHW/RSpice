//! Canonical Visualization Studio viewer metadata.
//!
//! The catalog mirrors the product manifest. Compatibility is deliberately
//! evaluated from an explicit caller-provided capability snapshot so catalog
//! discovery cannot silently assume that a result kind or external producer is
//! available.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ViewerGroup {
    TimeAndFrequency,
    Photonics,
    RfAndNetwork,
    StatisticalAndTabular,
    SerialLink,
    Specialized,
    FieldsAndPhysical,
}

impl ViewerGroup {
    pub const ALL: [Self; 7] = [
        Self::TimeAndFrequency,
        Self::Photonics,
        Self::RfAndNetwork,
        Self::StatisticalAndTabular,
        Self::SerialLink,
        Self::Specialized,
        Self::FieldsAndPhysical,
    ];

    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::TimeAndFrequency => "time-frequency",
            Self::Photonics => "photonics",
            Self::RfAndNetwork => "rf-network",
            Self::StatisticalAndTabular => "statistical-tabular",
            Self::SerialLink => "serial-link",
            Self::Specialized => "specialized",
            Self::FieldsAndPhysical => "fields-physical",
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::TimeAndFrequency => "Time & frequency",
            Self::Photonics => "Photonics",
            Self::RfAndNetwork => "RF & network",
            Self::StatisticalAndTabular => "Statistical & tabular",
            Self::SerialLink => "Serial link",
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
    PoleZero,
    Thermal,
    Mesh,
}

impl ViewerArt {
    #[must_use]
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
            Self::PoleZero => "pz",
            Self::Thermal => "thermal",
            Self::Mesh => "mesh",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickModeIcon {
    Waveform,
    Chart,
    Target,
    Cells,
    Verify,
}

impl QuickModeIcon {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Waveform => "waveform",
            Self::Chart => "chart",
            Self::Target => "target",
            Self::Cells => "cells",
            Self::Verify => "verify",
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewerQuickModeDefinition {
    pub id: &'static str,
    pub document_id: &'static str,
    pub icon: QuickModeIcon,
    pub label: &'static str,
    /// Analysis kinds accepted by this mode. A non-empty list is any-of.
    pub analysis_ids: &'static [&'static str],
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
    UnknownQuickMode,
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

const TRAN_DC: &[&str] = &["tran", "dc"];
const AC_STB_NOISE: &[&str] = &["ac", "stb", "noise"];
const FOURIER_TRAN_HB_PSS: &[&str] = &["fourier", "tran", "hb", "pss"];
const PNOISE_QPNOISE: &[&str] = &["pnoise", "qpnoise"];
const TRAN_FOURIER_HB_ENVELOPE: &[&str] = &["tran", "fourier", "hb", "envelope"];
const SP_HBSP_PSP: &[&str] = &["sp", "hbsp", "psp"];
const AC_SP_HB: &[&str] = &["ac", "sp", "hb"];
const SP_HBSP_PSP_HBNOISE: &[&str] = &["sp", "hbsp", "psp", "hbnoise"];
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
        domain: "TRAN · exact samples",
        x_axis: "Time · ms",
        y_axis: "Voltage · V",
        art: ViewerArt::Wave,
        analysis_ids: TRAN_DC,
        external_capability: None,
    },
    ViewerDocumentDefinition {
        id: "viewer-bode",
        group: ViewerGroup::TimeAndFrequency,
        title: "Bode / Nyquist",
        domain: "AC · complex response",
        x_axis: "Frequency · Hz",
        y_axis: "Magnitude · dB / phase · °",
        art: ViewerArt::Bode,
        analysis_ids: AC_STB_NOISE,
        external_capability: None,
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
    },
    ViewerDocumentDefinition {
        id: "viewer-network-quality",
        group: ViewerGroup::RfAndNetwork,
        title: "Noise figure / gain circles",
        domain: "SP · HBSP · PSP · HBNOISE",
        x_axis: "Frequency · Hz",
        y_axis: "Gain / NF / stability",
        art: ViewerArt::Bode,
        analysis_ids: SP_HBSP_PSP_HBNOISE,
        external_capability: None,
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
    },
    ViewerDocumentDefinition {
        id: "viewer-wafer-map",
        group: ViewerGroup::StatisticalAndTabular,
        title: "Wafer / spatial map",
        domain: "MC · die coordinates",
        x_axis: "Die X",
        y_axis: "Die Y",
        art: ViewerArt::Contour,
        analysis_ids: &["mc"],
        external_capability: None,
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
    },
];

pub const VIEWER_QUICK_MODES: &[ViewerQuickModeDefinition] = &[
    ViewerQuickModeDefinition {
        id: "waves",
        document_id: "viewer-waveform",
        icon: QuickModeIcon::Waveform,
        label: "Waves",
        analysis_ids: &["tran"],
    },
    ViewerQuickModeDefinition {
        id: "dc",
        document_id: "viewer-waveform",
        icon: QuickModeIcon::Chart,
        label: "DC Sweep",
        analysis_ids: &["dc"],
    },
    ViewerQuickModeDefinition {
        id: "bode",
        document_id: "viewer-bode",
        icon: QuickModeIcon::Chart,
        label: "Bode",
        analysis_ids: &["ac"],
    },
    ViewerQuickModeDefinition {
        id: "noise",
        document_id: "viewer-bode",
        icon: QuickModeIcon::Waveform,
        label: "Noise",
        analysis_ids: &["noise"],
    },
    ViewerQuickModeDefinition {
        id: "nyquist",
        document_id: "viewer-bode",
        icon: QuickModeIcon::Target,
        label: "Nyquist",
        analysis_ids: &["stb"],
    },
    ViewerQuickModeDefinition {
        id: "fft",
        document_id: "viewer-spectrum",
        icon: QuickModeIcon::Chart,
        label: "FFT",
        analysis_ids: &["fourier", "tran"],
    },
    ViewerQuickModeDefinition {
        id: "hb",
        document_id: "viewer-spectrum",
        icon: QuickModeIcon::Chart,
        label: "HB Tones",
        analysis_ids: &["hb"],
    },
    ViewerQuickModeDefinition {
        id: "phase-noise",
        document_id: "viewer-phase-noise",
        icon: QuickModeIcon::Waveform,
        label: "Phase Noise",
        analysis_ids: PNOISE_QPNOISE,
    },
    ViewerQuickModeDefinition {
        id: "smith",
        document_id: "viewer-smith",
        icon: QuickModeIcon::Target,
        label: "Smith",
        analysis_ids: &["sp"],
    },
    ViewerQuickModeDefinition {
        id: "sens",
        document_id: "viewer-table",
        icon: QuickModeIcon::Chart,
        label: "Sensitivity",
        analysis_ids: &["sens"],
    },
    ViewerQuickModeDefinition {
        id: "op",
        document_id: "viewer-table",
        icon: QuickModeIcon::Cells,
        label: "OP",
        analysis_ids: &["op"],
    },
    ViewerQuickModeDefinition {
        id: "specs",
        document_id: "viewer-table",
        icon: QuickModeIcon::Verify,
        label: "Specs",
        analysis_ids: &[],
    },
    ViewerQuickModeDefinition {
        id: "hist",
        document_id: "viewer-histogram",
        icon: QuickModeIcon::Chart,
        label: "Histogram",
        analysis_ids: MC_CORNER,
    },
    ViewerQuickModeDefinition {
        id: "eye",
        document_id: "eye-viewer",
        icon: QuickModeIcon::Target,
        label: "Eye",
        analysis_ids: &["tran"],
    },
    ViewerQuickModeDefinition {
        id: "pz",
        document_id: "viewer-pz",
        icon: QuickModeIcon::Target,
        label: "PZ",
        analysis_ids: &["pz"],
    },
];

#[must_use]
pub fn viewer_document(id: &str) -> Option<&'static ViewerDocumentDefinition> {
    VIEWER_DOCUMENTS.iter().find(|document| document.id == id)
}

#[must_use]
pub fn viewer_quick_mode(id: &str) -> Option<&'static ViewerQuickModeDefinition> {
    VIEWER_QUICK_MODES.iter().find(|mode| mode.id == id)
}

pub fn documents_in_group(
    group: ViewerGroup,
) -> impl Iterator<Item = &'static ViewerDocumentDefinition> {
    VIEWER_DOCUMENTS
        .iter()
        .filter(move |document| document.group == group)
}

pub fn quick_modes_for_document(
    document_id: &str,
) -> impl Iterator<Item = &'static ViewerQuickModeDefinition> + '_ {
    VIEWER_QUICK_MODES
        .iter()
        .filter(move |mode| mode.document_id == document_id)
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

#[must_use]
pub fn quick_mode_compatibility(
    mode_id: &str,
    capabilities: ViewerCapabilities<'_>,
) -> ViewerCompatibility {
    let Some(mode) = viewer_quick_mode(mode_id) else {
        return ViewerCompatibility::UnknownQuickMode;
    };

    analysis_compatibility(mode.analysis_ids, capabilities)
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
        "viewer-table",
        "viewer-histogram",
        "viewer-scatter",
        "viewer-contour",
        "viewer-box-violin",
        "viewer-wafer-map",
        "viewer-parallel-coordinates",
        "viewer-scatter-matrix",
        "viewer-contribution",
        "eye-viewer",
        "bathtub-viewer",
        "margin-viewer",
        "dynamic-droop-viewer",
        "viewer-pz",
        "field-viewer-3d",
        "field-viewer-current",
        "field-viewer-voltage",
        "field-viewer-thermal",
        "field-viewer-mesh",
        "field-viewer-probe",
    ];

    const QUICK_MODE_IDS: &[(&str, &str)] = &[
        ("waves", "viewer-waveform"),
        ("dc", "viewer-waveform"),
        ("bode", "viewer-bode"),
        ("noise", "viewer-bode"),
        ("nyquist", "viewer-bode"),
        ("fft", "viewer-spectrum"),
        ("hb", "viewer-spectrum"),
        ("phase-noise", "viewer-phase-noise"),
        ("smith", "viewer-smith"),
        ("sens", "viewer-table"),
        ("op", "viewer-table"),
        ("specs", "viewer-table"),
        ("hist", "viewer-histogram"),
        ("eye", "eye-viewer"),
        ("pz", "viewer-pz"),
    ];

    #[test]
    fn catalog_has_exact_manifest_document_ids_and_order() {
        assert_eq!(VIEWER_DOCUMENTS.len(), 37);
        assert_eq!(
            VIEWER_DOCUMENTS
                .iter()
                .map(|document| document.id)
                .collect::<Vec<_>>(),
            DOCUMENT_IDS
        );
    }

    #[test]
    fn catalog_has_exact_manifest_quick_mode_ids_and_owners() {
        assert_eq!(VIEWER_QUICK_MODES.len(), 15);
        assert_eq!(
            VIEWER_QUICK_MODES
                .iter()
                .map(|mode| (mode.id, mode.document_id))
                .collect::<Vec<_>>(),
            QUICK_MODE_IDS
        );
    }

    #[test]
    fn stable_ids_are_unique_and_lookups_are_total_for_catalog_entries() {
        let mut document_ids = HashSet::new();
        for document in VIEWER_DOCUMENTS {
            assert!(document_ids.insert(document.id), "{}", document.id);
            assert_eq!(viewer_document(document.id), Some(document));
        }

        let mut mode_ids = HashSet::new();
        for mode in VIEWER_QUICK_MODES {
            assert!(mode_ids.insert(mode.id), "{}", mode.id);
            assert_eq!(viewer_quick_mode(mode.id), Some(mode));
            assert!(viewer_document(mode.document_id).is_some(), "{}", mode.id);
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
                "Specialized",
                "Fields & physical",
            ]
        );
        assert_eq!(
            ViewerGroup::ALL.map(|group| documents_in_group(group).count()),
            [5, 3, 9, 9, 4, 1, 6]
        );
    }

    #[test]
    fn all_metadata_is_complete_and_requirement_ownership_is_unambiguous() {
        for document in VIEWER_DOCUMENTS {
            assert!(!document.id.is_empty());
            assert!(!document.title.is_empty());
            assert!(!document.domain.is_empty());
            assert!(!document.x_axis.is_empty());
            assert!(!document.y_axis.is_empty());
            assert!(!document.group.id().is_empty());
            assert!(!document.art.id().is_empty());
            assert!(
                document.analysis_ids.is_empty() || document.external_capability.is_none(),
                "{} mixes internal and external requirements",
                document.id
            );
        }

        for mode in VIEWER_QUICK_MODES {
            assert!(!mode.id.is_empty());
            assert!(!mode.label.is_empty());
            assert!(!mode.icon.id().is_empty());
        }
    }

    #[test]
    fn quick_mode_analysis_kinds_are_supported_by_their_document() {
        for mode in VIEWER_QUICK_MODES {
            let document = viewer_document(mode.document_id).expect("mode owner must exist");
            for analysis_id in mode.analysis_ids {
                assert!(
                    document.analysis_ids.contains(analysis_id) || document.analysis_ids.is_empty(),
                    "{} is outside {} compatibility",
                    mode.id,
                    document.id
                );
            }
        }
    }

    #[test]
    fn compatibility_is_any_of_for_analysis_backed_viewers() {
        let empty = ViewerCapabilities::default();
        assert_eq!(
            viewer_compatibility("viewer-bode", empty),
            ViewerCompatibility::MissingAnalysis {
                accepted_analysis_ids: AC_STB_NOISE,
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
    fn requirement_free_structured_views_remain_available() {
        let empty = ViewerCapabilities::default();
        assert!(viewer_compatibility("viewer-table", empty).is_compatible());
        assert!(quick_mode_compatibility("specs", empty).is_compatible());
    }

    #[test]
    fn quick_modes_are_checked_independently_from_their_parent_family() {
        let dc_only = ViewerCapabilities {
            analysis_ids: &["dc"],
            external_capabilities: &[],
        };
        assert!(quick_mode_compatibility("dc", dc_only).is_compatible());
        assert_eq!(
            quick_mode_compatibility("waves", dc_only),
            ViewerCompatibility::MissingAnalysis {
                accepted_analysis_ids: &["tran"],
            }
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
        assert_eq!(
            quick_mode_compatibility("mode-does-not-exist", all_plausible_capabilities),
            ViewerCompatibility::UnknownQuickMode
        );
    }

    #[test]
    fn document_and_mode_group_iterators_preserve_manifest_order() {
        assert_eq!(
            documents_in_group(ViewerGroup::Photonics)
                .map(|document| document.id)
                .collect::<Vec<_>>(),
            [
                "viewer-optical-spectrum",
                "viewer-optical-transfer",
                "viewer-optical-mode",
            ]
        );
        assert_eq!(
            quick_modes_for_document("viewer-bode")
                .map(|mode| mode.id)
                .collect::<Vec<_>>(),
            ["bode", "noise", "nyquist"]
        );
    }
}
