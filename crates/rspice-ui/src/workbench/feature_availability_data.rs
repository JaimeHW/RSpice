//! Immutable view data for the capability and platform matrix.
//!
//! This module is deliberately presentation-free. Its records are a direct
//! projection of the governed mockup contracts, while runtime route access is
//! resolved from the Rust route registry at the point of use. Catalog presence,
//! a design-fixture state, or an [`AnalysisKind`] mapping is never treated as
//! platform qualification, commercial release evidence, or sign-off approval.

mod availability_rows;

pub use availability_rows::*;

use crate::simulation::plan::AnalysisKind;

use super::{
    CapabilityWorkflowId,
    state::{EngineeringProfile, InteroperabilityDomain, InteroperabilitySupportLevel, Workspace},
};
use super::routing::availability::{SurfaceExecutionAvailability, surface_availability};
use super::routing::surface_catalog::{CanonicalTier, SurfaceId};

/// Deliberate state vocabulary shared by the versioned readiness fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadinessState {
    Current,
    Stale,
    Unavailable,
    Unknown,
}

impl ReadinessState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }

    /// Only `current` permits the fixture's unprotected design label. This
    /// still does not constitute product evidence or target qualification.
    #[must_use]
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Exact sign-off rule rendered for every platform row in the mockup matrix.
pub const PLATFORM_SIGN_OFF_RULE: &str = "exact current build, task, engine, model, device/browser, accessibility, and approval evidence";

/// One governed platform/task row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformAvailabilityRow {
    pub id: &'static str,
    pub label: &'static str,
    pub capability_mode_summary: &'static str,
    pub fixture_state: ReadinessState,
    pub fixture_qualification: &'static str,
    /// The deliberate fixture branch shown in the capability matrix.
    pub qualification_boundary: &'static str,
    /// The neutral design target boundary from `platform-task-contract.json`.
    pub design_qualification_boundary: &'static str,
    pub sign_off_rule: &'static str,
}

pub const PLATFORM_AVAILABILITY_ROWS: [PlatformAvailabilityRow; 4] = [
    PlatformAvailabilityRow {
        id: "desktop",
        label: "Native desktop",
        capability_mode_summary: "viewport: full \u{00b7} pointer: full \u{00b7} keyboard: full \u{00b7} stylus: adaptive \u{00b7} renderer: full \u{00b7} storage: full \u{00b7} filesystem: full \u{00b7} network: full \u{00b7} lifecycle: full \u{00b7} windowing: full \u{00b7} printing: full \u{00b7} assistive-technology: full",
        fixture_state: ReadinessState::Current,
        fixture_qualification: "unqualified-design-target",
        qualification_boundary: "The desktop composition record is current for the mockup contract; no native desktop build or operating-system matrix is qualified.",
        design_qualification_boundary: "Design target for Windows, macOS, and Linux native compositions; this record is not native-build qualification.",
        sign_off_rule: PLATFORM_SIGN_OFF_RULE,
    },
    PlatformAvailabilityRow {
        id: "browser",
        label: "Desktop or mobile browser",
        capability_mode_summary: "viewport: adaptive \u{00b7} pointer: full \u{00b7} keyboard: adaptive \u{00b7} stylus: adaptive \u{00b7} renderer: adaptive \u{00b7} storage: adaptive \u{00b7} filesystem: adaptive \u{00b7} network: adaptive \u{00b7} lifecycle: adaptive \u{00b7} windowing: adaptive \u{00b7} printing: adaptive \u{00b7} assistive-technology: adaptive",
        fixture_state: ReadinessState::Stale,
        fixture_qualification: "unqualified-design-target",
        qualification_boundary: "The browser branch intentionally represents stale platform evidence; viewport emulation is not browser qualification.",
        design_qualification_boundary: "Design target for supported browser engines and web APIs; viewport emulation is not browser or device qualification.",
        sign_off_rule: PLATFORM_SIGN_OFF_RULE,
    },
    PlatformAvailabilityRow {
        id: "tablet",
        label: "Tablet application or browser",
        capability_mode_summary: "viewport: adaptive \u{00b7} pointer: adaptive \u{00b7} keyboard: adaptive \u{00b7} stylus: full \u{00b7} renderer: adaptive \u{00b7} storage: adaptive \u{00b7} filesystem: adaptive \u{00b7} network: adaptive \u{00b7} lifecycle: adaptive \u{00b7} windowing: adaptive \u{00b7} printing: adaptive \u{00b7} assistive-technology: full",
        fixture_state: ReadinessState::Unavailable,
        fixture_qualification: "unqualified-design-target",
        qualification_boundary: "The unavailable branch contains no real-device tablet qualification and cannot be promoted by desktop or browser evidence.",
        design_qualification_boundary: "Design target for iPadOS and Android tablet portrait, landscape, split-screen, touch, stylus, and keyboard compositions; this is not real-device qualification.",
        sign_off_rule: PLATFORM_SIGN_OFF_RULE,
    },
    PlatformAvailabilityRow {
        id: "phone",
        label: "Phone application or browser",
        capability_mode_summary: "viewport: adaptive \u{00b7} pointer: adaptive \u{00b7} keyboard: adaptive \u{00b7} stylus: adaptive \u{00b7} renderer: adaptive \u{00b7} storage: adaptive \u{00b7} filesystem: adaptive \u{00b7} network: adaptive \u{00b7} lifecycle: adaptive \u{00b7} windowing: unavailable-with-reason \u{00b7} printing: adaptive \u{00b7} assistive-technology: full",
        fixture_state: ReadinessState::Unknown,
        fixture_qualification: "unqualified-design-target",
        qualification_boundary: "The phone branch intentionally represents unknown device evidence; responsive intent cannot substitute for device qualification.",
        design_qualification_boundary: "Design target for phone portrait and landscape task compositions. Release-scope authoring must be full or adaptive; this record is not real-device qualification.",
        sign_off_rule: PLATFORM_SIGN_OFF_RULE,
    },
];

#[must_use]
pub fn platform_availability_row(id: &str) -> Option<&'static PlatformAvailabilityRow> {
    PLATFORM_AVAILABILITY_ROWS.iter().find(|row| row.id == id)
}

pub fn platform_rows_matching(
    query: &str,
) -> impl Iterator<Item = &'static PlatformAvailabilityRow> {
    let query = normalized_query(query);
    PLATFORM_AVAILABILITY_ROWS
        .iter()
        .filter(move |row| matches_query(&query, &[row.id, row.label, row.capability_mode_summary]))
}

/// Roadmap/deferred status from the canonical planned design catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlannedWorkflowStatus {
    Roadmap,
    Deferred,
}

impl PlannedWorkflowStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Roadmap => "roadmap",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedWorkflowRow {
    pub workflow: CapabilityWorkflowId,
    pub id: &'static str,
    pub label: &'static str,
    pub group: &'static str,
    pub owner: &'static str,
    pub entry: &'static str,
    pub status: PlannedWorkflowStatus,
}

pub const PLANNED_WORKFLOW_ROWS: [PlannedWorkflowRow; 11] = [
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::GenericMultidimensionalRunsetController,
        id: "generic-multidimensional-runset-controller",
        label: "Generic multidimensional run-set controller",
        group: "Advanced analysis",
        owner: "Simulation plan",
        entry: "Simulate > Run-set dimensions",
        status: PlannedWorkflowStatus::Roadmap,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::SourceLoadPullAnalysis,
        id: "source-load-pull-analysis",
        label: "Source/load-pull analysis",
        group: "RF characterization",
        owner: "Simulation plan",
        entry: "Simulate > RF characterization > Source/load pull",
        status: PlannedWorkflowStatus::Roadmap,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::XParameterGeneration,
        id: "x-parameter-generation",
        label: "X-parameter generation",
        group: "RF characterization",
        owner: "Simulation plan",
        entry: "Simulate > RF characterization > X-parameters",
        status: PlannedWorkflowStatus::Roadmap,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::ModulatedSignalCharacterization,
        id: "modulated-signal-characterization",
        label: "Modulated-signal characterization",
        group: "RF characterization",
        owner: "Simulation plan",
        entry: "Simulate > RF characterization > Modulated signal",
        status: PlannedWorkflowStatus::Roadmap,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::CalibrationDeembeddingPlan,
        id: "calibration-deembedding-plan",
        label: "Calibration and de-embedding plan",
        group: "Measurement and results",
        owner: "Result document",
        entry: "Results > Transformations > Calibration and de-embedding",
        status: PlannedWorkflowStatus::Roadmap,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::StatisticalDistributionAuthoring,
        id: "statistical-distribution-authoring",
        label: "Statistical distribution authoring",
        group: "Variation and yield",
        owner: "Verification evidence",
        entry: "Verify > Variation models > Distribution set",
        status: PlannedWorkflowStatus::Roadmap,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::ErcPerc,
        id: "erc-perc",
        label: "ERC/PERC setup and review",
        group: "Physical verification",
        owner: "Verification evidence",
        entry: "Verify > Electrical rule checks",
        status: PlannedWorkflowStatus::Deferred,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::AntennaChecks,
        id: "antenna-checks",
        label: "Antenna-check setup and review",
        group: "Physical verification",
        owner: "Verification evidence",
        entry: "Verify > Antenna checks",
        status: PlannedWorkflowStatus::Deferred,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::DensityFillDfm,
        id: "density-fill-dfm",
        label: "Density, fill, and DFM",
        group: "Physical verification",
        owner: "Verification evidence",
        entry: "Verify > Manufacturability",
        status: PlannedWorkflowStatus::Deferred,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::GdsOasisStreamOut,
        id: "gds-oasis-stream-out",
        label: "GDSII/OASIS stream-out",
        group: "Physical handoff",
        owner: "Design",
        entry: "Design > Stream out",
        status: PlannedWorkflowStatus::Deferred,
    },
    PlannedWorkflowRow {
        workflow: CapabilityWorkflowId::TapeoutAssemblyHandoff,
        id: "tapeout-assembly-handoff",
        label: "Tapeout assembly and foundry handoff",
        group: "Physical handoff",
        owner: "Release candidate",
        entry: "More > Release > Tapeout handoff",
        status: PlannedWorkflowStatus::Deferred,
    },
];

#[must_use]
pub fn planned_workflow_row(id: &str) -> Option<&'static PlannedWorkflowRow> {
    PLANNED_WORKFLOW_ROWS.iter().find(|row| row.id == id)
}

pub fn planned_workflow_rows_matching(
    query: &str,
) -> impl Iterator<Item = &'static PlannedWorkflowRow> {
    let query = normalized_query(query);
    PLANNED_WORKFLOW_ROWS.iter().filter(move |row| {
        matches_query(
            &query,
            &[row.id, row.label, row.group, row.owner, row.entry],
        ) || planned_workflow_specification(row.id)
            .is_some_and(|specification| specification.matches_query(&query))
    })
}

/// Presentation intent carried by exact mockup specification text.
///
/// This is deliberately semantic rather than an egui color. A renderer maps
/// it through the active theme and therefore cannot accidentally turn the
/// design fixture into product-readiness evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlannedWorkflowTextStyle {
    Plain,
    Monospace,
    Success,
    Warning,
}

impl PlannedWorkflowTextStyle {
    #[must_use]
    pub const fn source_class(self) -> Option<&'static str> {
        match self {
            Self::Plain => None,
            Self::Monospace => Some("mono"),
            Self::Success => Some("ok-text"),
            Self::Warning => Some("warn-text"),
        }
    }
}

/// One exact cell in a planned-workflow specification table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedWorkflowTableCell {
    pub text: &'static str,
    pub style: PlannedWorkflowTextStyle,
}

impl PlannedWorkflowTableCell {
    #[must_use]
    pub const fn plain(text: &'static str) -> Self {
        Self {
            text,
            style: PlannedWorkflowTextStyle::Plain,
        }
    }

    #[must_use]
    pub const fn success(text: &'static str) -> Self {
        Self {
            text,
            style: PlannedWorkflowTextStyle::Success,
        }
    }

    #[must_use]
    pub const fn warning(text: &'static str) -> Self {
        Self {
            text,
            style: PlannedWorkflowTextStyle::Warning,
        }
    }
}

/// One immutable table in a planned-workflow specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedWorkflowTable {
    pub headers: &'static [&'static str],
    pub rows: &'static [&'static [PlannedWorkflowTableCell]],
}

/// One exact property-list entry in a planned-workflow specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedWorkflowProperty {
    pub label: &'static str,
    pub value: &'static str,
    pub style: PlannedWorkflowTextStyle,
}

impl PlannedWorkflowProperty {
    #[must_use]
    pub const fn plain(label: &'static str, value: &'static str) -> Self {
        Self {
            label,
            value,
            style: PlannedWorkflowTextStyle::Plain,
        }
    }

    #[must_use]
    pub const fn monospace(label: &'static str, value: &'static str) -> Self {
        Self {
            label,
            value,
            style: PlannedWorkflowTextStyle::Monospace,
        }
    }

    #[must_use]
    pub const fn warning(label: &'static str, value: &'static str) -> Self {
        Self {
            label,
            value,
            style: PlannedWorkflowTextStyle::Warning,
        }
    }
}

/// The exact two-column body composition specified by the mockup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlannedWorkflowContentLayout {
    /// All tables are stacked in the first column and the property list is in
    /// the second column.
    TableStackAndProperties,
    /// The first and second tables occupy the two columns respectively.
    SplitTables,
}

/// Optional exact chip collection below the primary split content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedWorkflowChipSection {
    pub title: &'static str,
    pub chips: &'static [&'static str],
}

/// Shared frame around every inspectable, non-executable planned design.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedWorkflowFrame {
    pub design_status: &'static str,
    pub executable: bool,
    pub unavailable_heading_prefix: &'static str,
    pub unavailable_heading_suffix: &'static str,
    pub unavailable_explanation: &'static str,
    pub ownership_section_title: &'static str,
    pub ownership_state: &'static str,
    pub ownership_field_labels: [&'static str; 3],
    pub task_sequence_section_title: &'static str,
    pub task_sequence_route_state: &'static str,
    pub task_steps: [&'static str; 4],
    pub validation_section_title: &'static str,
    pub outputs_section_title: &'static str,
    pub implementation_boundary: &'static str,
    pub dialog_title_suffix: &'static str,
    pub dialog_eyebrow_status: &'static str,
    pub primary_action: &'static str,
}

/// Exact shared frame from `plannedCapabilityFrame` in the governed mockup.
pub const PLANNED_WORKFLOW_FRAME: PlannedWorkflowFrame = PlannedWorkflowFrame {
    design_status: "complete",
    executable: false,
    unavailable_heading_prefix: "GUI design complete \u{00b7} ",
    unavailable_heading_suffix: " capability unavailable.",
    unavailable_explanation: "This is an inspectable implementation specification. It does not persist settings, invoke an engine or producer, create evidence, or expose an operational menu promise.",
    ownership_section_title: "Ownership and placement",
    ownership_state: "specified",
    ownership_field_labels: ["Canonical owner", "Intended entry point", "Workflow family"],
    task_sequence_section_title: "Intended task sequence",
    task_sequence_route_state: "future executable route",
    task_steps: [
        "1 \u{00b7} Configure",
        "2 \u{00b7} Validate",
        "3 \u{00b7} Preview",
        "4 \u{00b7} Execute / publish \u{00b7} disabled",
    ],
    validation_section_title: "Validation, failure, and recovery",
    outputs_section_title: "Outputs and provenance",
    implementation_boundary: "Implementation, numerical behavior, producer integration, and release qualification remain separate gates. When opened from the capability matrix, the primary action returns to that matrix without losing the review path.",
    dialog_title_suffix: " \u{00b7} GUI specification",
    dialog_eyebrow_status: "DESIGN COMPLETE",
    primary_action: "Close",
};

/// Complete read-only specification for one planned workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedWorkflowSpecification {
    pub id: &'static str,
    pub purpose: &'static str,
    pub content_section_title: &'static str,
    pub content_layout: PlannedWorkflowContentLayout,
    pub tables: &'static [PlannedWorkflowTable],
    pub property_section_title: Option<&'static str>,
    pub properties: &'static [PlannedWorkflowProperty],
    pub chip_section: Option<PlannedWorkflowChipSection>,
    pub validation_recovery: &'static str,
    pub outputs_provenance: &'static str,
}

impl PlannedWorkflowSpecification {
    fn matches_query(self, query: &str) -> bool {
        if matches_query(
            query,
            &[
                self.id,
                self.purpose,
                self.content_section_title,
                self.property_section_title.unwrap_or_default(),
                self.validation_recovery,
                self.outputs_provenance,
            ],
        ) {
            return true;
        }
        if self.tables.iter().any(|table| {
            table
                .headers
                .iter()
                .any(|header| matches_query(query, &[header]))
                || table
                    .rows
                    .iter()
                    .flat_map(|row| (*row).iter())
                    .any(|cell| matches_query(query, &[cell.text]))
        }) {
            return true;
        }
        if self
            .properties
            .iter()
            .any(|property| matches_query(query, &[property.label, property.value]))
        {
            return true;
        }
        self.chip_section.is_some_and(|section| {
            matches_query(query, &[section.title])
                || section
                    .chips
                    .iter()
                    .any(|chip| matches_query(query, &[chip]))
        })
    }
}

macro_rules! planned_row {
    ($($cell:literal),+ $(,)?) => {
        &[$(PlannedWorkflowTableCell::plain($cell)),+]
    };
}

const RUNSET_CONTROLLER_SPECIFICATION: PlannedWorkflowSpecification =
    PlannedWorkflowSpecification {
        id: "generic-multidimensional-runset-controller",
        purpose: "Compose arbitrary typed run dimensions into deterministic, reviewable point manifests without hiding inheritance, conditional branches, adaptive bounds, cost, storage, or rejected points.",
        content_section_title: "Dimension and composition editor",
        content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
        tables: &[PlannedWorkflowTable {
            headers: &["Dimension", "Type", "Values / source", "Dependency"],
            rows: &[
                planned_row![
                    "process",
                    "model corner",
                    "TT \u{00b7} FF \u{00b7} SS",
                    "root"
                ],
                planned_row![
                    "temperature",
                    "quantity list",
                    "\u{2212}40 \u{00b0}C \u{00b7} 27 \u{00b0}C \u{00b7} 125 \u{00b0}C",
                    "Cartesian with process"
                ],
                planned_row![
                    "vdd",
                    "quantity range",
                    "1.62 V \u{2192} 1.98 V \u{00b7} 0.18 V",
                    "zipped inside supply group"
                ],
                planned_row![
                    "mismatch",
                    "random sample",
                    "seeded \u{00b7} 200 samples",
                    "only when process \u{2260} TT"
                ],
            ],
        }],
        property_section_title: None,
        properties: &[
            PlannedWorkflowProperty::plain("Composition root", "Cartesian"),
            PlannedWorkflowProperty::plain("Nested groups", "supply \u{00b7} mismatch"),
            PlannedWorkflowProperty::monospace("Expanded points", "1,209 exact"),
            PlannedWorkflowProperty::plain(
                "Forecast",
                "03:48 \u{00b7} 2.6 GiB \u{00b7} 24 workers",
            ),
            PlannedWorkflowProperty::plain(
                "Invalid-value policy",
                "reject point and retain manifest",
            ),
        ],
        chip_section: Some(PlannedWorkflowChipSection {
            title: "Supported composition semantics",
            chips: &[
                "Cartesian",
                "Zipped",
                "Filtered",
                "Conditional",
                "Adaptive \u{00b7} bounded",
                "Nested",
            ],
        }),
        validation_recovery: "Block cycles, incompatible zip lengths, unit/type errors, unbounded adaptive branches, duplicate point identities, quota overflow, and persistence-version ambiguity before queueing. Preserve the exact draft, focus the failing node, and retain the last valid forecast.",
        outputs_provenance: "Versioned composition tree, immutable expanded point manifest, rejection manifest, cardinality/cost/storage forecast, seed and migration policy, and stable point-to-result identities.",
    };

const SOURCE_LOAD_PULL_SPECIFICATION: PlannedWorkflowSpecification = PlannedWorkflowSpecification {
    id: "source-load-pull-analysis",
    purpose: "Define source and load reflection-coefficient domains, tuner and fixture references, RF operating axes, objectives, exclusions, and contour-result intent as one canonical simulation plan.",
    content_section_title: "Impedance domains and operating axes",
    content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
    tables: &[
        PlannedWorkflowTable {
            headers: &["Domain", "Reference", "Sampling", "Exclusions"],
            rows: &[
                planned_row![
                    "Source \u{0393}",
                    "50 \u{03a9} \u{00b7} calibrated plane P1",
                    "polar grid \u{00b7} |\u{0393}| \u{2264} 0.65",
                    "unstable and tuner-unreachable"
                ],
                planned_row![
                    "Load \u{0393}",
                    "50 \u{03a9} \u{00b7} DUT plane P2",
                    "adaptive contour \u{00b7} |\u{0393}| \u{2264} 0.85",
                    "VSWR and SOA boundary"
                ],
            ],
        },
        PlannedWorkflowTable {
            headers: &["Axis", "Plan", "Composition"],
            rows: &[
                planned_row!["Frequency", "2.35, 2.40, 2.45 GHz", "outer"],
                planned_row![
                    "Available power",
                    "\u{2212}10 dBm \u{2192} 30 dBm \u{00b7} 2 dB",
                    "nested"
                ],
                planned_row!["Bias / temperature", "nominal + PVT subset", "conditional"],
            ],
        },
    ],
    property_section_title: Some("Objectives and bindings"),
    properties: &[
        PlannedWorkflowProperty::plain("Primary objective", "maximize PAE at Pout \u{2265} 28 dBm"),
        PlannedWorkflowProperty::plain(
            "Constraints",
            "gain \u{00b7} compression \u{00b7} harmonics \u{00b7} stability \u{00b7} SOA",
        ),
        PlannedWorkflowProperty::plain(
            "Tuner / fixture",
            "producer-bound \u{00b7} calibration required",
        ),
        PlannedWorkflowProperty::plain("Preview", "Smith-domain points, rejected zones, cost"),
        PlannedWorkflowProperty::plain(
            "Result document",
            "contours, optima, slices, exact point table",
        ),
    ],
    chip_section: None,
    validation_recovery: "Reject reference-plane, impedance, frequency, power, tuner-range, calibration, stability, or SOA inconsistencies before dispatch. Keep unreachable and excluded points distinct from failed simulations and allow return to the exact domain vertex.",
    outputs_provenance: "Immutable source/load impedance point manifest; fixture, tuner, calibration, and producer lineage; objective/constraint definitions; contour and optimum result intent; and rejected-point reasons.",
};

const X_PARAMETER_SPECIFICATION: PlannedWorkflowSpecification = PlannedWorkflowSpecification {
    id: "x-parameter-generation",
    purpose: "Specify nonlinear ports, fundamentals and mixing products, bias and environment axes, excitation power, normalization, validation vectors, and publication policy for producer-bound X-parameter generation.",
    content_section_title: "Nonlinear network definition",
    content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
    tables: &[PlannedWorkflowTable {
        headers: &["Port / tone", "Reference", "Role", "Plan"],
        rows: &[
            planned_row![
                "P1 \u{00b7} fundamental",
                "50 \u{03a9} \u{00b7} plane IN",
                "large-signal drive",
                "2.40 GHz \u{00b7} \u{2212}20 \u{2192} 30 dBm"
            ],
            planned_row![
                "P2 \u{00b7} response",
                "50 \u{03a9} \u{00b7} plane OUT",
                "scattered-wave output",
                "harmonics 1\u{2026}7"
            ],
            planned_row![
                "Auxiliary tones",
                "declared per port",
                "small-signal perturbation",
                "mixing order \u{2264} 5"
            ],
        ],
    }],
    property_section_title: None,
    properties: &[
        PlannedWorkflowProperty::plain(
            "Bias / environment",
            "VDD \u{00b7} IDQ \u{00b7} temperature \u{00b7} process",
        ),
        PlannedWorkflowProperty::plain(
            "Normalization",
            "power waves \u{00b7} declared complex convention",
        ),
        PlannedWorkflowProperty::plain("Compression guard", "continuation + stability checkpoint"),
        PlannedWorkflowProperty::plain(
            "Noise terms",
            "optional only when producer supports and qualifies",
        ),
        PlannedWorkflowProperty::plain(
            "Publication",
            "producer-native dataset; neutral exchange only if supported",
        ),
    ],
    chip_section: Some(PlannedWorkflowChipSection {
        title: "Required review views",
        chips: &[
            "Excitation grid",
            "Harmonic/mixing inventory",
            "Compression and stability",
            "Reciprocity/passivity applicability",
            "Validation-vector residuals",
            "Dataset lineage",
        ],
    }),
    validation_recovery: "Block undefined nonlinear ports, incompatible wave conventions, harmonic/mixing truncation ambiguity, discontinuous operating points, unsupported noise/export claims, or failed validation tolerances. Preserve partial producer artifacts as non-publishable diagnostics.",
    outputs_provenance: "Producer-bound nonlinear network dataset intent, exact excitation and operating-point manifest, normalization metadata, validation residuals, compatibility report, and immutable source/model/engine/platform lineage.",
};

const MODULATED_SIGNAL_SPECIFICATION: PlannedWorkflowSpecification = PlannedWorkflowSpecification {
    id: "modulated-signal-characterization",
    purpose: "Bind a versioned modulated waveform, carrier and power plan, PA/DPD state, calibration and triggering, measurement definitions, limits, and report intent into one repeatable RF characterization plan.",
    content_section_title: "Waveform, stimulus, and measurements",
    content_layout: PlannedWorkflowContentLayout::SplitTables,
    tables: &[
        PlannedWorkflowTable {
            headers: &["Category", "Selected design", "Required provenance"],
            rows: &[
                planned_row![
                    "Waveform",
                    "standard profile + revision \u{00b7} 100 MHz occupied BW",
                    "generator, sample rate, crest factor, seed"
                ],
                planned_row![
                    "Carrier / power",
                    "2.40 GHz \u{00b7} \u{2212}10 \u{2192} 30 dBm",
                    "reference plane and calibration"
                ],
                planned_row![
                    "PA / DPD",
                    "PA model \u{00b7} bypass or imported coefficient revision",
                    "model, memory depth, adaptation state"
                ],
                planned_row![
                    "Capture",
                    "settle, trigger, record length, averaging",
                    "clock and phase reference"
                ],
            ],
        },
        PlannedWorkflowTable {
            headers: &["Metric", "Evaluation", "Disposition"],
            rows: &[
                planned_row![
                    "EVM",
                    "RMS / peak \u{00b7} equalization disclosed",
                    "limit + uncertainty"
                ],
                planned_row![
                    "ACPR / ACLR",
                    "named adjacent bands and filters",
                    "per sideband"
                ],
                planned_row![
                    "CCDF",
                    "input and output power distributions",
                    "sample-confidence bound"
                ],
                planned_row![
                    "SEM / mask",
                    "standard revision or project mask",
                    "margin and violating bins"
                ],
                planned_row![
                    "Gain / PAE",
                    "time-aligned average and peak",
                    "operating-point slice"
                ],
            ],
        },
    ],
    property_section_title: None,
    properties: &[],
    chip_section: None,
    validation_recovery: "Reject missing standard revisions, incompatible sample rates, insufficient settle/capture duration, calibration or reference-plane mismatch, ambiguous filtering/equalization, clipped records, and inadequate sample confidence. Keep invalid and partial captures visibly non-compliant.",
    outputs_provenance: "Versioned characterization dataset intent; waveform, generator, model, calibration, and trigger lineage; EVM/ACPR/CCDF/mask measurement records; limit margins; and reproducible report definition.",
};

const CALIBRATION_DEEMBEDDING_SPECIFICATION: PlannedWorkflowSpecification =
    PlannedWorkflowSpecification {
        id: "calibration-deembedding-plan",
        purpose: "Build an ordered, reversible calibration and de-embedding transform chain with standards, reference planes, uncertainty, validity bands, compatibility checks, and immutable derived-dataset publication.",
        content_section_title: "Transform-chain editor",
        content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
        tables: &[PlannedWorkflowTable {
            headers: &[
                "Order",
                "Transform",
                "Source / method",
                "Reference-plane effect",
            ],
            rows: &[
                planned_row![
                    "1",
                    "Instrument calibration",
                    "SOLT \u{00b7} kit rev C",
                    "ports \u{2192} cable ends"
                ],
                planned_row![
                    "2",
                    "Fixture removal",
                    "TRL \u{00b7} coupon set 24Q2",
                    "cable ends \u{2192} DUT pads"
                ],
                planned_row![
                    "3",
                    "Port renormalization",
                    "frequency-dependent Zref",
                    "representation only"
                ],
                planned_row![
                    "4",
                    "Time-gate \u{00b7} optional",
                    "reviewed window",
                    "no plane change"
                ],
            ],
        }],
        property_section_title: None,
        properties: &[
            PlannedWorkflowProperty::plain(
                "Available methods",
                "SOLT \u{00b7} TRL \u{00b7} LRM \u{00b7} unknown-thru \u{00b7} fixture model",
            ),
            PlannedWorkflowProperty::plain("Validity band", "100 MHz \u{2192} 20 GHz"),
            PlannedWorkflowProperty::plain(
                "Uncertainty",
                "standard + repeatability + transform propagation",
            ),
            PlannedWorkflowProperty::plain(
                "Checks",
                "dimensions \u{00b7} reciprocity \u{00b7} passivity \u{00b7} causality \u{00b7} conditioning",
            ),
            PlannedWorkflowProperty::plain("Publication", "new immutable derived dataset"),
        ],
        chip_section: None,
        validation_recovery: "Block missing standards, expired calibration, port/order/dimension mismatch, invalid frequency overlap, ill-conditioned inversion, non-causal or non-passive results outside declared policy, and uncertainty gaps. Never overwrite source bytes; permit step disable, reorder, or rollback with previewed delta.",
        outputs_provenance: "Versioned transform graph, source-standard digests, validity and uncertainty envelope, before/after comparison, compatibility diagnostics, derived immutable dataset, and reversible lineage to every source sample.",
    };

const STATISTICAL_DISTRIBUTION_SPECIFICATION: PlannedWorkflowSpecification =
    PlannedWorkflowSpecification {
        id: "statistical-distribution-authoring",
        purpose: "Author reusable process, mismatch, global, and local variation variables with units, distributions, truncation, correlations, constraints, sampling and seed policy, validation, migration, and consumer bindings.",
        content_section_title: "Distribution-set editor",
        content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
        tables: &[PlannedWorkflowTable {
            headers: &["Variable", "Scope", "Distribution", "Parameters"],
            rows: &[
                planned_row![
                    "vth_n",
                    "global process",
                    "normal \u{00b7} truncated",
                    "\u{03bc} 0 V \u{00b7} \u{03c3} 18 mV \u{00b7} \u{00b1}4\u{03c3}"
                ],
                planned_row![
                    "tox_scale",
                    "lot/global",
                    "lognormal",
                    "median 1 \u{00b7} \u{03c3}log 0.025"
                ],
                planned_row![
                    "r_mismatch",
                    "instance/local",
                    "normal \u{00b7} area-scaled",
                    "Pelgrom coefficient + geometry"
                ],
                planned_row![
                    "package_bin",
                    "discrete",
                    "categorical",
                    "A .72 \u{00b7} B .23 \u{00b7} C .05"
                ],
            ],
        }],
        property_section_title: None,
        properties: &[
            PlannedWorkflowProperty::plain(
                "Supported families",
                "normal \u{00b7} lognormal \u{00b7} uniform \u{00b7} triangular \u{00b7} beta \u{00b7} discrete \u{00b7} empirical",
            ),
            PlannedWorkflowProperty::plain("Correlation", "named groups + PSD matrix validation"),
            PlannedWorkflowProperty::plain(
                "Constraints",
                "bounds \u{00b7} equations \u{00b7} conditional activation",
            ),
            PlannedWorkflowProperty::plain(
                "Sampling",
                "pseudo-random \u{00b7} Latin hypercube \u{00b7} quasi-random",
            ),
            PlannedWorkflowProperty::plain(
                "Seed policy",
                "project default + immutable run override",
            ),
        ],
        chip_section: Some(PlannedWorkflowChipSection {
            title: "Validation views",
            chips: &[
                "PDF / CDF",
                "Moments",
                "Correlation heatmap",
                "Eigenvalue / PSD check",
                "Constraint rejection rate",
                "Migration diff",
            ],
        }),
        validation_recovery: "Reject unit/domain mismatch, invalid parameters or truncation, non-positive-semidefinite correlation, impossible constraints, duplicate stochastic identity, unsupported sampling, and lossy schema migration. Preserve the prior version and identify every affected consumer.",
        outputs_provenance: "Versioned distribution set, correlation and constraint model, validation statistics and plots, seed/sampling policy, migration receipt, consumer impact list, and exact random-variable lineage for each run and sample.",
    };

const ERC_PERC_SPECIFICATION: PlannedWorkflowSpecification = PlannedWorkflowSpecification {
    id: "erc-perc",
    purpose: "Configure producer-bound electrical and programmable electrical rule checks, inspect topology and voltage-propagation results, cross-probe markers, repair or waive findings, and assemble current review evidence.",
    content_section_title: "Electrical-rule plan and result review",
    content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
    tables: &[PlannedWorkflowTable {
        headers: &["Check family", "Inputs", "Typical findings", "Review owner"],
        rows: &[
            planned_row![
                "Connectivity / topology",
                "netlist, hierarchy, devices",
                "floating gates \u{00b7} illegal paths \u{00b7} missing clamps",
                "circuit + reliability"
            ],
            planned_row![
                "Power domains",
                "domain intent, rails, states",
                "cross-domain devices \u{00b7} missing isolation",
                "power intent"
            ],
            planned_row![
                "Voltage propagation",
                "source ranges, device limits",
                "EOS/overvoltage \u{00b7} gate-oxide stress",
                "reliability"
            ],
            planned_row![
                "ESD / latch-up",
                "pads, clamps, wells, paths",
                "broken discharge path \u{00b7} resistance limit",
                "ESD specialist"
            ],
            planned_row![
                "Programmable query",
                "versioned rule API and parameters",
                "project/foundry topology conditions",
                "rule author + reviewer"
            ],
        ],
    }],
    property_section_title: None,
    properties: &[
        PlannedWorkflowProperty::warning("Producer binding", "required \u{00b7} unavailable"),
        PlannedWorkflowProperty::plain(
            "Rule package",
            "foundry / organization \u{00b7} exact version + digest",
        ),
        PlannedWorkflowProperty::plain(
            "Result browser",
            "hierarchy \u{00b7} rule \u{00b7} severity \u{00b7} domain \u{00b7} state",
        ),
        PlannedWorkflowProperty::plain(
            "Cross-probe",
            "schematic \u{00b7} layout \u{00b7} extracted path \u{00b7} rule source",
        ),
        PlannedWorkflowProperty::plain(
            "Disposition",
            "repair \u{00b7} explain \u{00b7} waive \u{00b7} reopen \u{00b7} supersede",
        ),
    ],
    chip_section: None,
    validation_recovery: "Block unresolved power intent, rule/API incompatibility, missing producer identity, stale layout/netlist/PDK inputs, incomplete voltage ranges, or invalid waiver authority. Distinguish not-run, partial, tool error, waived, and clean; preserve prior immutable results on rerun failure.",
    outputs_provenance: "Producer-bound ERC/PERC run manifest, rule and query inventory, hierarchy/path markers, voltage propagation and ESD-path evidence, repair deltas, waiver audit trail, currentness evaluation, and sign-off boundary.",
};

const ANTENNA_CHECKS_SPECIFICATION: PlannedWorkflowSpecification = PlannedWorkflowSpecification {
    id: "antenna-checks",
    purpose: "Bind foundry antenna rules and process sequence to nets, accumulate conductor and gate areas by layer, review violations, plan diode or routing repairs, cross-probe geometry, and govern waivers and evidence.",
    content_section_title: "Antenna rule setup and violation review",
    content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
    tables: &[PlannedWorkflowTable {
        headers: &["Rule scope", "Configured source", "Evaluation"],
        rows: &[
            planned_row![
                "Process sequence",
                "foundry deck \u{00b7} rev 12",
                "layer-by-layer exposure order"
            ],
            planned_row![
                "Gate area",
                "extracted device terminals",
                "oxide class + effective gate area"
            ],
            planned_row![
                "Conductor area",
                "routing polygons and vias",
                "partial and cumulative ratio"
            ],
            planned_row![
                "Protection",
                "diodes, jumpers, approved cells",
                "credit and connectivity validity"
            ],
        ],
    }],
    property_section_title: None,
    properties: &[
        PlannedWorkflowProperty::plain(
            "Marker filters",
            "rule \u{00b7} layer \u{00b7} cell \u{00b7} net \u{00b7} severity \u{00b7} repairability",
        ),
        PlannedWorkflowProperty::plain(
            "Geometry view",
            "contributing shapes + gate/protection path",
        ),
        PlannedWorkflowProperty::plain(
            "Repair strategies",
            "layer jump \u{00b7} diode \u{00b7} route split \u{00b7} approved waiver",
        ),
        PlannedWorkflowProperty::plain(
            "Incremental scope",
            "changed nets with full-dependency validation",
        ),
        PlannedWorkflowProperty::plain("Evidence", "before/after ratios + exact rule source"),
    ],
    chip_section: None,
    validation_recovery: "Reject missing process sequence, ambiguous gate/protection recognition, stale extraction, unit/layer-map mismatch, invalid incremental scope, or waiver without rule/net/layout revision and approval. A producer error or incomplete net inventory cannot be reported clean.",
    outputs_provenance: "Producer-bound rule/run manifest, per-net and per-layer area ledger, violation geometry, repair proposal and before/after ratio, incremental-rerun receipt, disposition history, and foundry-current evidence package.",
};

const DENSITY_FILL_DFM_SPECIFICATION: PlannedWorkflowSpecification = PlannedWorkflowSpecification {
    id: "density-fill-dfm",
    purpose: "Configure density windows and targets, fill generation and keepouts, DFM/hotspot checks, electrical-impact controls, revision comparison, marker disposition, and downstream invalidation as one manufacturability workflow.",
    content_section_title: "Density, fill, and manufacturability plan",
    content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
    tables: &[PlannedWorkflowTable {
        headers: &["Layer class", "Window / step", "Target", "Fill policy"],
        rows: &[
            planned_row![
                "Metal 1\u{2013}4",
                "100 \u{00b5}m / 50 \u{00b5}m",
                "30%\u{2013}70%",
                "grounded where qualified"
            ],
            planned_row![
                "Top metal",
                "200 \u{00b5}m / 100 \u{00b5}m",
                "20%\u{2013}65%",
                "floating or stitched by rule"
            ],
            planned_row![
                "Poly / diffusion",
                "foundry-defined",
                "deck-defined",
                "restricted cells only"
            ],
            planned_row![
                "Via arrays",
                "local pattern rules",
                "redundancy score",
                "approved templates"
            ],
        ],
    }],
    property_section_title: None,
    properties: &[
        PlannedWorkflowProperty::plain(
            "Keepouts",
            "analog/RF nets \u{00b7} inductors \u{00b7} ESD \u{00b7} seals \u{00b7} user regions",
        ),
        PlannedWorkflowProperty::plain(
            "DFM checks",
            "hotspots \u{00b7} lithography patterns \u{00b7} via redundancy \u{00b7} yield score",
        ),
        PlannedWorkflowProperty::plain(
            "Electrical impact",
            "coupling estimate + extraction threshold",
        ),
        PlannedWorkflowProperty::plain(
            "Revision policy",
            "new layout revision \u{00b7} never in-place sign-off mutation",
        ),
        PlannedWorkflowProperty::plain(
            "Downstream effect",
            "invalidate affected DRC, LVS, PEX, EM/IR evidence",
        ),
    ],
    chip_section: None,
    validation_recovery: "Block missing rule/fill producer, illegal keepout overlap, density target conflict, disconnected required fill, unbounded geometry growth, stale source layout, or unacceptable electrical-impact forecast. Roll back atomically to the prior layout revision on generation or verification failure.",
    outputs_provenance: "Density maps and window table, generated-fill change set, hotspot/DFM/yield markers, exclusions and waivers, electrical-impact estimate, before/after semantic and geometric diff, new layout revision, and downstream stale-evidence manifest.",
};

const GDS_OASIS_STREAM_OUT_SPECIFICATION: PlannedWorkflowSpecification =
    PlannedWorkflowSpecification {
        id: "gds-oasis-stream-out",
        purpose: "Define final GDSII/OASIS export ownership, hierarchy and naming policy, layer/datatype mapping, units and grid, protected-cell handling, integrity and round-trip checks, manifesting, and foundry delivery boundary.",
        content_section_title: "Stream-out package definition",
        content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
        tables: &[PlannedWorkflowTable {
            headers: &["Category", "Configured design", "Required check"],
            rows: &[
                planned_row![
                    "Source",
                    "immutable layout revision L42 \u{00b7} top /AFE",
                    "saved, current, hierarchy resolved"
                ],
                planned_row![
                    "Format",
                    "OASIS preferred \u{00b7} GDSII compatibility",
                    "writer/version and foundry acceptance"
                ],
                planned_row![
                    "Hierarchy",
                    "preserve \u{00b7} selective flatten exceptions",
                    "name/cell/reference limits"
                ],
                planned_row![
                    "Geometry",
                    "database unit 1 nm \u{00b7} exact grid policy",
                    "no rounding or overflow"
                ],
                planned_row![
                    "Layer map",
                    "PDK stream map rev 2.4.1",
                    "layer/purpose/datatype completeness"
                ],
                planned_row![
                    "Protected cells",
                    "preserve vendor policy",
                    "authorization and digest"
                ],
            ],
        }],
        property_section_title: None,
        properties: &[
            PlannedWorkflowProperty::plain("Compression", "format-qualified setting"),
            PlannedWorkflowProperty::plain(
                "Text / properties",
                "allowlist with encoding and length checks",
            ),
            PlannedWorkflowProperty::plain(
                "Integrity",
                "byte length \u{00b7} SHA-256 \u{00b7} structure inventory",
            ),
            PlannedWorkflowProperty::plain("Round trip", "read-back + semantic/layout comparison"),
            PlannedWorkflowProperty::plain(
                "Destination",
                "staged package \u{00b7} no direct overwrite",
            ),
        ],
        chip_section: None,
        validation_recovery: "Block unmapped or multiply mapped purposes, unsupported hierarchy/name/property limits, coordinate rounding, geometry overflow, protected-cell policy failure, corrupt write, read-back mismatch, or stale source/PDK. Preserve the source and prior valid package; quarantine an incomplete candidate.",
        outputs_provenance: "Staged GDSII/OASIS artifact, source/layout/PDK/writer manifest, layer-map and hierarchy reports, unmapped-object inventory, byte length and digest, round-trip comparison, warnings/dispositions, and foundry-profile compatibility statement.",
    };

const TAPEOUT_HANDOFF_SPECIFICATION: PlannedWorkflowSpecification = PlannedWorkflowSpecification {
    id: "tapeout-assembly-handoff",
    purpose: "Assemble a frozen tapeout release candidate, prove every required design and verification artifact current, collect independent approvals, package and protect deliverables, transfer them, record receipt and acceptance, and support supersession or recall.",
    content_section_title: "Release checklist and delivery ledger",
    content_layout: PlannedWorkflowContentLayout::TableStackAndProperties,
    tables: &[PlannedWorkflowTable {
        headers: &["Gate", "Required evidence", "State"],
        rows: &[
            &[
                PlannedWorkflowTableCell::plain("Design freeze"),
                PlannedWorkflowTableCell::plain("schematic, layout, hierarchy, source revisions"),
                PlannedWorkflowTableCell::success("design specified"),
            ],
            &[
                PlannedWorkflowTableCell::plain("Technology"),
                PlannedWorkflowTableCell::plain("PDK, rule packages, models, stream map, licenses"),
                PlannedWorkflowTableCell::warning("producer qualification required"),
            ],
            &[
                PlannedWorkflowTableCell::plain("Verification"),
                PlannedWorkflowTableCell::plain(
                    "DRC, LVS, ERC/PERC, antenna, density/DFM, extraction",
                ),
                PlannedWorkflowTableCell::warning("deferred producers block"),
            ],
            &[
                PlannedWorkflowTableCell::plain("Electrical sign-off"),
                PlannedWorkflowTableCell::plain(
                    "required simulations, measurements, reliability, waivers",
                ),
                PlannedWorkflowTableCell::warning("current evidence required"),
            ],
            &[
                PlannedWorkflowTableCell::plain("Deliverables"),
                PlannedWorkflowTableCell::plain(
                    "stream file, netlists, reports, manifests, checksums",
                ),
                PlannedWorkflowTableCell::warning("assembler unavailable"),
            ],
            &[
                PlannedWorkflowTableCell::plain("Approvals"),
                PlannedWorkflowTableCell::plain(
                    "design, verification, PDK, quality, release authority",
                ),
                PlannedWorkflowTableCell::warning("distinct signatures required"),
            ],
        ],
    }],
    property_section_title: None,
    properties: &[
        PlannedWorkflowProperty::plain("Package", "versioned inventory + content digests"),
        PlannedWorkflowProperty::plain(
            "Protection",
            "signing \u{00b7} encryption \u{00b7} access \u{00b7} retention",
        ),
        PlannedWorkflowProperty::plain(
            "Delivery",
            "approved foundry endpoint + resumable transfer",
        ),
        PlannedWorkflowProperty::plain(
            "Receipt",
            "submitted \u{00b7} received \u{00b7} validated \u{00b7} accepted / rejected",
        ),
        PlannedWorkflowProperty::plain(
            "Change control",
            "supersede \u{00b7} recall \u{00b7} reissue \u{00b7} rollback record",
        ),
    ],
    chip_section: None,
    validation_recovery: "Fail closed for any stale, missing, partial, waived-without-authority, digest-mismatched, producer-unknown, unqualified, unsigned, unauthorized, or rejected prerequisite. A successful upload is not foundry acceptance; interrupted transfer resumes only against the identical package digest.",
    outputs_provenance: "Frozen release-candidate manifest, prerequisite/currentness matrix, approved waiver set, deliverable inventory and digests, detached approval bindings, protected package, transfer and foundry receipts, acceptance state, and immutable supersession/recall lineage.",
};

/// Exact, complete, inspectable GUI specifications for all planned workflows.
pub const PLANNED_WORKFLOW_SPECIFICATIONS: [PlannedWorkflowSpecification; 11] = [
    RUNSET_CONTROLLER_SPECIFICATION,
    SOURCE_LOAD_PULL_SPECIFICATION,
    X_PARAMETER_SPECIFICATION,
    MODULATED_SIGNAL_SPECIFICATION,
    CALIBRATION_DEEMBEDDING_SPECIFICATION,
    STATISTICAL_DISTRIBUTION_SPECIFICATION,
    ERC_PERC_SPECIFICATION,
    ANTENNA_CHECKS_SPECIFICATION,
    DENSITY_FILL_DFM_SPECIFICATION,
    GDS_OASIS_STREAM_OUT_SPECIFICATION,
    TAPEOUT_HANDOFF_SPECIFICATION,
];

/// Finds the exact read-only GUI specification for a canonical planned ID.
/// Unknown IDs remain unknown; there is no generic or inferred fallback.
#[must_use]
pub fn planned_workflow_specification(id: &str) -> Option<&'static PlannedWorkflowSpecification> {
    PLANNED_WORKFLOW_SPECIFICATIONS
        .iter()
        .find(|specification| specification.id == id)
}

/// Semantic tone for governed capability-contract values. The renderer maps
/// this through theme tokens; the fixture never carries presentation colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractTone {
    Plain,
    Success,
    Warning,
    Error,
}

impl ContractTone {
    #[must_use]
    pub const fn source_class(self) -> Option<&'static str> {
        match self {
            Self::Plain => None,
            Self::Success => Some("ok-text"),
            Self::Warning => Some("warn-text"),
            Self::Error => Some("error-text"),
        }
    }
}

pub const INTEROPERABILITY_DIALOG_TITLE: &str = "Interoperability and format matrix";
pub const INTEROPERABILITY_DIALOG_EYEBROW: &str =
    "IMPORT · EXPORT · ROUND-TRIP · VERSIONED CONTRACT";
pub const INTEROPERABILITY_TABLIST_LABEL: &str = "Interoperability sections";

/// One exact row from the mockup's versioned format matrix, plus explicit
/// filter taxonomy. A row may intentionally belong to more than one support
/// projection when the release contract names both (for example native or a
/// licensed connector).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteroperabilityFormatRow {
    pub domain: InteroperabilityDomain,
    pub support_levels: &'static [InteroperabilitySupportLevel],
    pub domain_format: &'static str,
    pub version_dialect: &'static str,
    pub direction: &'static str,
    pub release_contract: &'static str,
    pub release_tone: ContractTone,
    pub round_trip_loss_policy: &'static str,
}

impl InteroperabilityFormatRow {
    #[must_use]
    pub fn matches(
        self,
        domain: InteroperabilityDomain,
        support_level: InteroperabilitySupportLevel,
    ) -> bool {
        (domain == InteroperabilityDomain::All || domain == self.domain)
            && (support_level == InteroperabilitySupportLevel::All
                || self.support_levels.contains(&support_level))
    }
}

const NATIVE: &[InteroperabilitySupportLevel] = &[InteroperabilitySupportLevel::Native];
const QUALIFIED: &[InteroperabilitySupportLevel] = &[InteroperabilitySupportLevel::Qualified];
const CONNECTOR_DEPENDENT: &[InteroperabilitySupportLevel] =
    &[InteroperabilitySupportLevel::ConnectorDependent];
const QUALIFIED_CONNECTOR: &[InteroperabilitySupportLevel] = &[
    InteroperabilitySupportLevel::Qualified,
    InteroperabilitySupportLevel::ConnectorDependent,
];
const NATIVE_CONNECTOR: &[InteroperabilitySupportLevel] = &[
    InteroperabilitySupportLevel::Native,
    InteroperabilitySupportLevel::ConnectorDependent,
];

pub const INTEROPERABILITY_FORMAT_ROWS: [InteroperabilityFormatRow; 12] = [
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::NetlistAndSimulation,
        support_levels: NATIVE,
        domain_format: "RSpice project + deck",
        version_dialect: "schema 1.x · native syntax",
        direction: "import · export",
        release_contract: "native",
        release_tone: ContractTone::Success,
        round_trip_loss_policy: "stable IDs + semantic digest",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::NetlistAndSimulation,
        support_levels: QUALIFIED,
        domain_format: "SPICE-family decks",
        version_dialect: "Spectre · HSPICE · PSpice · LTspice · ngspice · Xyce",
        direction: "import · reviewed export",
        release_contract: "qualified dialect profiles",
        release_tone: ContractTone::Success,
        round_trip_loss_policy: "unsupported constructs block or map explicitly",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::NetlistAndSimulation,
        support_levels: QUALIFIED,
        domain_format: "Behavioral / mixed signal",
        version_dialect: "Verilog-A · Verilog-AMS · SystemVerilog RNM",
        direction: "import · compile",
        release_contract: "runtime and license qualified",
        release_tone: ContractTone::Warning,
        round_trip_loss_policy: "source ownership + golden-vector report",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::NetlistAndSimulation,
        support_levels: CONNECTOR_DEPENDENT,
        domain_format: "Model / system exchange",
        version_dialect: "IBIS 7.x · IBIS-AMI · FMI 2/3 · SSP 1.x",
        direction: "import · co-sim",
        release_contract: "connector-dependent",
        release_tone: ContractTone::Warning,
        round_trip_loss_policy: "capability handshake; no silent downgrade",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::IcDesignAndLayout,
        support_levels: QUALIFIED_CONNECTOR,
        domain_format: "IC library / schematic",
        version_dialect: "OpenAccess 2.2.x · EDIF 2 0 0",
        direction: "import · governed export",
        release_contract: "qualified connector",
        release_tone: ContractTone::Warning,
        round_trip_loss_policy: "source object retained + stable-ID diff",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::IcDesignAndLayout,
        support_levels: QUALIFIED,
        domain_format: "IC layout / implementation",
        version_dialect: "GDSII · OASIS · LEF/DEF 5.8",
        direction: "import · export",
        release_contract: "qualified with layer map",
        release_tone: ContractTone::Success,
        round_trip_loss_policy: "DBU, origin, orientation, layer/purpose digest",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::PcbAndManufacturing,
        support_levels: QUALIFIED,
        domain_format: "PCB manufacturing",
        version_dialect: "ODB++ · IPC-2581C · Gerber X2 · Excellon 2",
        direction: "import review · export package",
        release_contract: "qualified by profile",
        release_tone: ContractTone::Warning,
        round_trip_loss_policy: "stackup, drill, aperture, polarity, checksum manifest",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::PcbAndManufacturing,
        support_levels: QUALIFIED,
        domain_format: "PCB connectivity / test",
        version_dialect: "IPC-D-356A · IPC-356 · IPC-2221 net classes",
        direction: "import · export",
        release_contract: "qualified subset",
        release_tone: ContractTone::Warning,
        round_trip_loss_policy: "net, pin, test-point and no-connect audit",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::MechanicalExchange,
        support_levels: CONNECTOR_DEPENDENT,
        domain_format: "ECAD / MCAD",
        version_dialect: "STEP AP242 · IDX / EDMD · IDF 3/4 · DXF",
        direction: "bidirectional candidate",
        release_contract: "connector-dependent",
        release_tone: ContractTone::Warning,
        round_trip_loss_policy: "coordinate frame + ownership of keepouts/cutouts",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::ResultsAndReports,
        support_levels: QUALIFIED,
        domain_format: "RF / measurement",
        version_dialect: "Touchstone 1/2 · CITIfile · MDIF · far-field",
        direction: "import · export",
        release_contract: "qualified",
        release_tone: ContractTone::Success,
        round_trip_loss_policy: "reference impedance, waves, units, causality, passivity",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::ResultsAndReports,
        support_levels: NATIVE_CONNECTOR,
        domain_format: "Results / events",
        version_dialect: "PSF · RAW · CSV · HDF5 · VCD · FST · FSDB",
        direction: "import · export by format",
        release_contract: "native or licensed connector",
        release_tone: ContractTone::Warning,
        round_trip_loss_policy: "axes, units, precision, sample/event digest",
    },
    InteroperabilityFormatRow {
        domain: InteroperabilityDomain::ResultsAndReports,
        support_levels: QUALIFIED,
        domain_format: "Reports / evidence",
        version_dialect: "PDF · SVG · PNG · CSV · XLSX · JSON",
        direction: "export",
        release_contract: "qualified render/data profiles",
        release_tone: ContractTone::Success,
        round_trip_loss_policy: "provenance footer + source manifest",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractNote {
    pub title: &'static str,
    pub body: &'static str,
}

pub const INTEROPERABILITY_ROUND_TRIP_NOTES: [ContractNote; 4] = [
    ContractNote {
        title: "Transactional candidate",
        body: "Imports open as candidates beside the untouched source archive. Replacement is blocked until mappings, validation, and loss disclosures are reviewed.",
    },
    ContractNote {
        title: "Coordinates and units",
        body: "Database units, scale, origin, orientation, handedness, temperature, impedance, complex convention, encoding, and locale are explicit.",
    },
    ContractNote {
        title: "Unsupported content",
        body: "Unsupported objects remain source-linked or quarantined. RSpice never fabricates an equivalent, drops an object silently, or marks a partial conversion complete.",
    },
    ContractNote {
        title: "Protected content",
        body: "Encrypted models, foundry decks, and licensed connectors retain entitlement, redaction, destination, and platform policy.",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoundTripGateRow {
    pub gate: &'static str,
    pub evidence: &'static str,
    pub failure_behavior: &'static str,
}

pub const INTEROPERABILITY_ROUND_TRIP_GATES: [RoundTripGateRow; 5] = [
    RoundTripGateRow {
        gate: "Parse",
        evidence: "source digest · version · encoding · object counts",
        failure_behavior: "quarantine invalid input",
    },
    RoundTripGateRow {
        gate: "Map",
        evidence: "explicit dialect, layer, unit, coordinate, and name rules",
        failure_behavior: "block unmapped required semantics",
    },
    RoundTripGateRow {
        gate: "Validate",
        evidence: "connectivity, geometry, numeric, and schema checks",
        failure_behavior: "retain candidate with actionable diagnostics",
    },
    RoundTripGateRow {
        gate: "Compare",
        evidence: "semantic diff and declared tolerances",
        failure_behavior: "disclose every accepted loss",
    },
    RoundTripGateRow {
        gate: "Commit",
        evidence: "transaction ID + source/candidate manifests",
        failure_behavior: "rollback to untouched predecessor",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteroperabilityQualificationRow {
    pub profile: &'static str,
    pub golden_corpus: &'static str,
    pub required_comparison: &'static str,
    pub platform_gate: &'static str,
    pub release_state: &'static str,
    pub release_tone: ContractTone,
}

pub const INTEROPERABILITY_QUALIFICATION_ROWS: [InteroperabilityQualificationRow; 4] = [
    InteroperabilityQualificationRow {
        profile: "SPICE deck profiles",
        golden_corpus: "syntax + semantics + failure corpus",
        required_comparison: "topology · values · analyses · numeric oracle",
        platform_gate: "desktop · web parser parity",
        release_state: "qualified subsets",
        release_tone: ContractTone::Success,
    },
    InteroperabilityQualificationRow {
        profile: "IC stream formats",
        golden_corpus: "hierarchy · arrays · text · properties · large coordinates",
        required_comparison: "geometry hash + layer map + connectivity",
        platform_gate: "native import/export workers",
        release_state: "qualified profiles",
        release_tone: ContractTone::Success,
    },
    InteroperabilityQualificationRow {
        profile: "PCB / MCAD packages",
        golden_corpus: "rigid-flex · blind/buried vias · cavities · variants",
        required_comparison: "stackup + drill + geometry + net ownership",
        platform_gate: "connector and version pinned",
        release_state: "profile dependent",
        release_tone: ContractTone::Warning,
    },
    InteroperabilityQualificationRow {
        profile: "Result data",
        golden_corpus: "real/complex · stepped · sparse · partial · corrupted",
        required_comparison: "axis, unit, precision, point and event digests",
        platform_gate: "memory and streaming bounds",
        release_state: "qualified formats",
        release_tone: ContractTone::Success,
    },
];

pub const INTEROPERABILITY_QUALIFICATION_BOUNDARY: &str = "“Supported” is never a single boolean. Direction, version, feature subset, connector/license, target platform, qualification corpus, known loss, and sign-off eligibility are versioned independently.";

pub const TOUCH_EDIT_GUIDE_DIALOG_TITLE: &str = "Touch schematic editing";
pub const TOUCH_EDIT_GUIDE_DIALOG_EYEBROW: &str = "SCHEMATIC · TOUCH · PRECISION INPUT";
pub const TOUCH_EDIT_GUIDE_ACTION_LABEL: &str = "Open touch guide";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TouchEditGuideStep {
    pub number: u8,
    pub title: &'static str,
    pub body: &'static str,
}

pub const TOUCH_EDIT_GUIDE_STEPS: [TouchEditGuideStep; 4] = [
    TouchEditGuideStep {
        number: 1,
        title: "Navigate without changing the design",
        body: "Drag with one finger to pan. Pinch with two fingers to zoom around the gesture center. Use Fit to restore the complete-sheet overview.",
    },
    TouchEditGuideStep {
        number: 2,
        title: "Select and inspect",
        body: "Tap an object to select it. Open Inspector for typed values, exact identity, operating-point data and validation state.",
    },
    TouchEditGuideStep {
        number: 3,
        title: "Place with snap preview",
        body: "Choose a drawing tool first, then tap the canvas. Wires and buses use two taps with an explicit snapped start point and a reviewed commit.",
    },
    TouchEditGuideStep {
        number: 4,
        title: "Use exact coordinates when touch is insufficient",
        body: "Open Object properties or the navigator to enter values and coordinates directly. Touch placement never changes solver precision or stored engineering values.",
    },
];

pub const TOUCH_EDIT_GUIDE_CONCEPT: &str = "Phone layouts preserve the complete supported task by presenting one precise step at a time. Tablet and desktop layouts expose more tools simultaneously; capability, validation, undo, and stored precision remain the same.";

pub const PLATFORM_LIFECYCLE_DIALOG_TITLE: &str = "Browser and mobile lifecycle contract";
pub const PLATFORM_LIFECYCLE_DIALOG_EYEBROW: &str = "PLATFORM · SUSPENSION · STORAGE · RECOVERY";
pub const PLATFORM_LIFECYCLE_WARNING: &str = "A backgrounded browser or mobile operating system may suspend or terminate execution without warning. RSpice never labels an interrupted run complete and never promotes partial results.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformLifecycleRow {
    pub platform_event: &'static str,
    pub protected_state: &'static str,
    pub user_visible_response: &'static str,
    pub recovery: &'static str,
    pub release_eligibility: &'static str,
    pub eligibility_tone: ContractTone,
}

pub const PLATFORM_LIFECYCLE_ROWS: [PlatformLifecycleRow; 6] = [
    PlatformLifecycleRow {
        platform_event: "Tab suspension / app background",
        protected_state: "manifest + periodic encrypted checkpoint",
        user_visible_response: "paused · last durable step",
        recovery: "resume exact task or restart",
        release_eligibility: "pending completion",
        eligibility_tone: ContractTone::Warning,
    },
    PlatformLifecycleRow {
        platform_event: "Storage quota eviction",
        protected_state: "server manifest + export receipt",
        user_visible_response: "local cache unavailable",
        recovery: "re-fetch verified artifacts",
        release_eligibility: "unchanged if digest matches",
        eligibility_tone: ContractTone::Plain,
    },
    PlatformLifecycleRow {
        platform_event: "Network handoff / offline",
        protected_state: "local edit journal + queued intent",
        user_visible_response: "offline banner · no silent dispatch",
        recovery: "review synchronization",
        release_eligibility: "remote run pending",
        eligibility_tone: ContractTone::Plain,
    },
    PlatformLifecycleRow {
        platform_event: "Permission or entitlement loss",
        protected_state: "project and non-protected results",
        user_visible_response: "read-only with exact cause",
        recovery: "reauthorize or choose eligible target",
        release_eligibility: "blocked",
        eligibility_tone: ContractTone::Error,
    },
    PlatformLifecycleRow {
        platform_event: "PWA or engine update",
        protected_state: "open document schema + old worker",
        user_visible_response: "update ready; no hot solver swap",
        recovery: "finish, checkpoint, then reload",
        release_eligibility: "build remains pinned",
        eligibility_tone: ContractTone::Plain,
    },
    PlatformLifecycleRow {
        platform_event: "Thermal / battery pressure",
        protected_state: "task checkpoint where supported",
        user_visible_response: "throttled or paused with estimate",
        recovery: "resume locally or transfer explicitly",
        release_eligibility: "numeric contract unchanged",
        eligibility_tone: ContractTone::Plain,
    },
];

pub const PLATFORM_LIFECYCLE_NOTES: [ContractNote; 2] = [
    ContractNote {
        title: "Touch editing",
        body: "44 px targets, gesture cancellation, snapped previews and exact-coordinate fallback prevent accidental imprecision.",
    },
    ContractNote {
        title: "Qualification owner",
        body: "Lifecycle behavior is tested per browser, OS and engine build before release eligibility is granted.",
    },
];

/// Exact customer-facing tier interpretation from the mockup analysis table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnalysisIntendedTier {
    ReleaseTarget,
    PreviewNonSignOff,
    CompatibilityPath,
}

impl AnalysisIntendedTier {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseTarget => "release target",
            Self::PreviewNonSignOff => "preview \u{00b7} non-sign-off",
            Self::CompatibilityPath => "compatibility path",
        }
    }

    #[must_use]
    pub const fn evidence_requirement(self) -> &'static str {
        match self {
            Self::ReleaseTarget => "engine, oracle, and platform qualification required",
            Self::PreviewNonSignOff => "setup and review only until separately qualified",
            Self::CompatibilityPath => "source-semantic comparison required",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalysisAvailabilityRow {
    pub id: &'static str,
    pub code: &'static str,
    pub title: &'static str,
    pub intended_tier: AnalysisIntendedTier,
    /// Present only when the current Rust plan domain has this exact stable ID.
    /// This mapping alone is not an engine/platform qualification claim.
    pub analysis_kind: Option<AnalysisKind>,
}

impl AnalysisAvailabilityRow {
    #[must_use]
    pub const fn evidence_requirement(self) -> &'static str {
        self.intended_tier.evidence_requirement()
    }
}

pub const CAPABILITY_FIXTURE_STATUS: &str = "design-fixture";
pub const CAPABILITY_FIXTURE_REVISION: &str = "2026-07-13.1";
pub const CAPABILITY_FIXTURE_AS_OF: &str = "2026-07-13T12:00:00Z";
pub const CAPABILITY_FIXTURE_BOUNDARY: &str = "This fixture does not prove that Rust, a simulator, an engine, a connector, a platform build, a model, or a commercial release has been implemented, qualified, approved, or made sign-off eligible.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntitlementState {
    Granted,
    Denied,
    Expired,
    Revoked,
    Unknown,
}

/// Coarse, display-only classification of the exact fixture binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceClassification {
    Bound,
    Missing,
    Unverified,
    Stale,
}

/// Customer projection of one product-mode resolver case. These rows are
/// deterministic design-fixture vectors, explicitly not product evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityClaimProjection {
    pub case_id: &'static str,
    pub original_label: &'static str,
    pub resolved_label: &'static str,
    pub subject_kind: &'static str,
    pub asserted_stage: &'static str,
    pub state: ReadinessState,
    pub label_allowed: bool,
    pub entitlement_state: EntitlementState,
    pub evidence_classification: EvidenceClassification,
    pub applicable_source_summary: &'static str,
    pub reason_codes: &'static [&'static str],
    pub fixture_revision: &'static str,
    pub as_of: &'static str,
    pub boundary: &'static str,
}

pub const CAPABILITY_CLAIM_PROJECTIONS: [CapabilityClaimProjection; 12] = [
    CapabilityClaimProjection {
        case_id: "current-design-contract",
        original_label: "Design complete",
        resolved_label: "Design complete",
        subject_kind: "design",
        asserted_stage: "design-complete",
        state: ReadinessState::Current,
        label_allowed: true,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Bound,
        applicable_source_summary: "evidence-design-current \u{00b7} design-contract \u{00b7} implementation/capability-readiness-fixture.json@2026-07-13.1 \u{00b7} current/verified",
        reason_codes: &[],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "stale-design-contract",
        original_label: "Design complete",
        resolved_label: "Claim blocked \u{00b7} Design complete",
        subject_kind: "design",
        asserted_stage: "design-complete",
        state: ReadinessState::Stale,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Stale,
        applicable_source_summary: "evidence-design-stale \u{00b7} design-contract \u{00b7} implementation/capability-readiness-fixture.json@2026-07-01.1 \u{00b7} stale/verified",
        reason_codes: &["evidence-stale"],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "unavailable-design-contract",
        original_label: "Design complete",
        resolved_label: "Claim blocked \u{00b7} Design complete",
        subject_kind: "design",
        asserted_stage: "design-complete",
        state: ReadinessState::Unavailable,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Bound,
        applicable_source_summary: "evidence-design-unavailable \u{00b7} design-contract \u{00b7} implementation/capability-readiness-fixture.json@unavailable-branch \u{00b7} unavailable/verified",
        reason_codes: &["evidence-unavailable", "evidence-not-applicable"],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "unknown-design-contract",
        original_label: "Design complete",
        resolved_label: "Claim blocked \u{00b7} Design complete",
        subject_kind: "design",
        asserted_stage: "design-complete",
        state: ReadinessState::Unknown,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Unverified,
        applicable_source_summary: "evidence-design-unknown \u{00b7} design-contract \u{00b7} implementation/capability-readiness-fixture.json@unknown-branch \u{00b7} unknown/unknown",
        reason_codes: &[
            "evidence-verification-unknown",
            "evidence-currentness-unknown",
            "evidence-applicability-unknown",
        ],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "release-label-without-evidence",
        original_label: "Production release ready",
        resolved_label: "Claim blocked \u{00b7} Production release ready",
        subject_kind: "release",
        asserted_stage: "platform-qualified",
        state: ReadinessState::Unknown,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Missing,
        applicable_source_summary: "No applicable evidence binding",
        reason_codes: &["missing-required-evidence"],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "qualified-label-with-stale-platform-evidence",
        original_label: "Platform qualified",
        resolved_label: "Claim blocked \u{00b7} Platform qualified",
        subject_kind: "platform",
        asserted_stage: "platform-qualified",
        state: ReadinessState::Stale,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Stale,
        applicable_source_summary: "evidence-platform-stale \u{00b7} release-evidence \u{00b7} fixture://stale-platform-qualification@stale-r1 \u{00b7} stale/verified",
        reason_codes: &["evidence-stale"],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "signoff-label-with-mismatched-build",
        original_label: "Sign-off eligible",
        resolved_label: "Claim blocked \u{00b7} Sign-off eligible",
        subject_kind: "sign-off",
        asserted_stage: "sign-off-eligible",
        state: ReadinessState::Stale,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Bound,
        applicable_source_summary: "evidence-signoff-wrong-build \u{00b7} release-evidence \u{00b7} fixture://mismatched-signoff@other-build-r1 \u{00b7} current/verified",
        reason_codes: &["evidence-target-mismatch"],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "release-label-with-unverified-evidence",
        original_label: "Release ready",
        resolved_label: "Claim blocked \u{00b7} Release ready",
        subject_kind: "release",
        asserted_stage: "platform-qualified",
        state: ReadinessState::Unknown,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Unverified,
        applicable_source_summary: "evidence-release-unverified \u{00b7} release-evidence \u{00b7} fixture://unverified-release@unverified-r1 \u{00b7} current/unverified",
        reason_codes: &["evidence-unverified"],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "qualified-label-with-expired-evidence",
        original_label: "Platform qualified",
        resolved_label: "Claim blocked \u{00b7} Platform qualified",
        subject_kind: "platform",
        asserted_stage: "platform-qualified",
        state: ReadinessState::Stale,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Stale,
        applicable_source_summary: "evidence-platform-expired \u{00b7} release-evidence \u{00b7} fixture://expired-platform-qualification@expired-r1 \u{00b7} current/verified",
        reason_codes: &["evidence-expired"],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "denied-entitlement-blocks-design-claim",
        original_label: "Design complete",
        resolved_label: "Claim blocked \u{00b7} Design complete",
        subject_kind: "design",
        asserted_stage: "design-complete",
        state: ReadinessState::Unavailable,
        label_allowed: false,
        entitlement_state: EntitlementState::Denied,
        evidence_classification: EvidenceClassification::Bound,
        applicable_source_summary: "evidence-design-current \u{00b7} design-contract \u{00b7} implementation/capability-readiness-fixture.json@2026-07-13.1 \u{00b7} current/verified",
        reason_codes: &[
            "entitlement-denied",
            "target-unavailable",
            "evidence-target-mismatch",
        ],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "test-vector-authority-rejected-for-product-claim",
        original_label: "Model qualified",
        resolved_label: "Claim blocked \u{00b7} Model qualified",
        subject_kind: "model",
        asserted_stage: "engine-ready",
        state: ReadinessState::Unknown,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Bound,
        applicable_source_summary: "evidence-model-qualified-current-test-vector \u{00b7} contract-test-vector \u{00b7} fixture://positive-model-qualification-vector@vector-r1 \u{00b7} current/verified",
        reason_codes: &["evidence-authority-ineligible"],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
    CapabilityClaimProjection {
        case_id: "preview-engine-cannot-be-release-ready",
        original_label: "Release ready",
        resolved_label: "Claim blocked \u{00b7} Release ready",
        subject_kind: "engine",
        asserted_stage: "platform-qualified",
        state: ReadinessState::Unavailable,
        label_allowed: false,
        entitlement_state: EntitlementState::Granted,
        evidence_classification: EvidenceClassification::Missing,
        applicable_source_summary: "No applicable evidence binding",
        reason_codes: &[
            "target-protected-claim-ineligible",
            "missing-required-evidence",
        ],
        fixture_revision: CAPABILITY_FIXTURE_REVISION,
        as_of: CAPABILITY_FIXTURE_AS_OF,
        boundary: CAPABILITY_FIXTURE_BOUNDARY,
    },
];

#[must_use]
pub fn capability_claim_projection(case_id: &str) -> Option<&'static CapabilityClaimProjection> {
    CAPABILITY_CLAIM_PROJECTIONS
        .iter()
        .find(|row| row.case_id == case_id)
}

pub fn capability_claim_rows_matching(
    query: &str,
) -> impl Iterator<Item = &'static CapabilityClaimProjection> {
    let query = normalized_query(query);
    CAPABILITY_CLAIM_PROJECTIONS.iter().filter(move |row| {
        matches_query(
            &query,
            &[
                row.case_id,
                row.original_label,
                row.resolved_label,
                row.subject_kind,
                row.asserted_stage,
                row.applicable_source_summary,
            ],
        )
    })
}

fn normalized_query(query: &str) -> String {
    query.trim().to_lowercase()
}

fn matches_query(query: &str, values: &[&str]) -> bool {
    query.is_empty()
        || values
            .iter()
            .any(|value| value.to_lowercase().contains(query))
}


#[cfg(test)]
mod tests;
