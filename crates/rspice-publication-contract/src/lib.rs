//! Versioned interchange schema for sealed RSpice circuit publications.
//!
//! A publication snapshot is the complete, self-contained description of a
//! published circuit page: document metadata, disclosure decisions, resolved
//! schematic scenes, the netlist deck, analysis records, exact result
//! datasets, and the figures the author curated. The RSpice client builds
//! one at publish time; the `rspice-publish` renderer turns it into the
//! immutable page bundle; the `rspice-viewer` runtime hydrates individual
//! figures from per-figure payloads carved out of the same types.
//!
//! Design rules this crate is built around:
//!
//! - **Typed data, never markup.** Every string is plain text that consumers
//!   escape at their own boundary. Nothing in a snapshot can smuggle HTML,
//!   SVG, or script into a rendered page.
//! - **Resolved geometry, not authored objects.** Schematic and plot content
//!   is carried as a display list in integral micrometres — the same
//!   physical-unit convention as the hardcopy pipeline — so the renderer and
//!   viewer reproduce exactly what the author saw without compiling the
//!   editor's document model. Version skew between the producing client and
//!   the rendering binary cannot change what a sealed page looks like.
//! - **Exact numerics.** Result samples are IEEE-754 bit patterns, not
//!   decimal strings, so a sealed snapshot never loses source precision to
//!   formatting and serialization is bit-reproducible.
//! - **Strict envelopes.** Every type rejects unknown fields, roots carry an
//!   exact schema version, and [`PublicationSnapshot::from_canonical_bytes`]
//!   enforces a hard size cap before parsing. Additive evolution happens by
//!   bumping [`PUBLICATION_SNAPSHOT_SCHEMA_VERSION`], never by silently
//!   tolerating unrecognized content.
//! - **Deterministic bytes.** No unordered collections appear anywhere in
//!   the schema; canonical serialization of equal values is byte-identical.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Legacy snapshot schema retained for installed clients and immutable pages.
pub const PUBLICATION_SNAPSHOT_V2_SCHEMA_VERSION: u32 = 2;
/// Current snapshot schema. Version 3 adds typed page presentation,
/// component/net identity, explicit signal bindings, and simulation
/// provenance while preserving strict v2 decoding.
pub const PUBLICATION_SNAPSHOT_SCHEMA_VERSION: u32 = 3;
/// Exact schema version a conforming figure manifest must declare.
pub const FIGURE_MANIFEST_SCHEMA_VERSION: u32 = 1;
/// Hard upper bound on canonical snapshot bytes, matching the hardcopy
/// worker-snapshot transport cap.
pub const MAX_PUBLICATION_SNAPSHOT_BYTES: usize = 64 * 1024 * 1024;
/// Hard upper bound on one figure hydration payload.
pub const MAX_FIGURE_PAYLOAD_BYTES: usize = 32 * 1024 * 1024;

/// Longest accepted publication title, in Unicode scalar values.
pub const MAX_TITLE_CHARS: usize = 200;
/// Longest accepted publication description, in Unicode scalar values.
pub const MAX_DESCRIPTION_CHARS: usize = 4_000;
/// Longest accepted author display name, in Unicode scalar values.
pub const MAX_AUTHOR_CHARS: usize = 120;
/// Longest accepted label (sheet names, trace labels, analysis labels,
/// measurement names, unit strings), in Unicode scalar values.
pub const MAX_LABEL_CHARS: usize = 200;
/// Longest accepted netlist deck, in bytes.
pub const MAX_DECK_BYTES: usize = 4 * 1024 * 1024;
/// Longest accepted scene text primitive, in Unicode scalar values.
pub const MAX_TEXT_CHARS: usize = 1_024;
/// Longest accepted unit string, in Unicode scalar values.
pub const MAX_UNIT_CHARS: usize = 40;
/// Longest accepted producing-client version string.
pub const MAX_APP_VERSION_CHARS: usize = 64;
/// Longest accepted RFC 3339 creation instant.
pub const MAX_TIMESTAMP_CHARS: usize = 40;
/// Longest accepted figure mount-point DOM id.
pub const MAX_DOM_ID_CHARS: usize = 64;
/// Longest accepted bundle-relative payload path.
pub const MAX_PAYLOAD_PATH_CHARS: usize = 256;

/// Most schematic sheets one snapshot may carry.
pub const MAX_SHEETS: usize = 64;
/// Most figures one snapshot or manifest may carry.
pub const MAX_FIGURES: usize = 64;
/// Most analysis records one snapshot may carry.
pub const MAX_ANALYSES: usize = 256;
/// Most datasets one snapshot or figure payload may carry.
pub const MAX_DATASETS: usize = 64;
/// Most traces one dataset may carry.
pub const MAX_TRACES_PER_DATASET: usize = 128;
/// Most sample values (sweep plus every trace component) one snapshot or
/// payload may carry in total, matching the visualization document's
/// source-cell ceiling.
pub const MAX_TOTAL_SAMPLE_VALUES: usize = 8_000_000;
/// Most measurements one snapshot may carry.
pub const MAX_MEASUREMENTS: usize = 1_024;
/// Most primitive groups one scene may carry.
pub const MAX_GROUPS_PER_SCENE: usize = 100_000;
/// Most primitives one scene may carry across all groups.
pub const MAX_PRIMITIVES_PER_SCENE: usize = 1_000_000;
/// Most segments one path primitive may carry.
pub const MAX_SEGMENTS_PER_PATH: usize = 10_000;
/// Longest v3 overview narrative or figure accessibility summary.
pub const MAX_NARRATIVE_CHARS: usize = 16_000;
/// Bounds for v3 typed engineering metadata.
pub const MAX_SPECIFICATIONS: usize = 256;
pub const MAX_COMPONENTS: usize = 10_000;
pub const MAX_NETS: usize = 10_000;
pub const MAX_PINS_PER_COMPONENT: usize = 256;
pub const MAX_NET_CONNECTIONS: usize = 100_000;
pub const MAX_SIGNAL_IDENTITIES: usize = 8_192;
pub const MAX_SIMULATION_WARNINGS: usize = 256;

/// Schema violations detected by [`Validate::validate`] or the canonical
/// byte codecs. Every variant names the offending element so a producer can
/// fix its builder and a renderer can log a precise rejection.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ContractError {
    #[error("unsupported schema version {found}, expected {expected}")]
    UnsupportedSchemaVersion { expected: u32, found: u32 },
    #[error("canonical payload is {found} bytes, above the {limit}-byte cap")]
    PayloadTooLarge { limit: usize, found: usize },
    #[error("canonical payload is not valid JSON for this schema: {0}")]
    Malformed(String),
    #[error("{field} is empty")]
    EmptyField { field: &'static str },
    #[error("{field} exceeds {limit} characters")]
    FieldTooLong { field: &'static str, limit: usize },
    #[error("{field} contains a control character")]
    ControlCharacter { field: &'static str },
    #[error("netlist deck exceeds {MAX_DECK_BYTES} bytes")]
    DeckTooLarge,
    #[error("disclosure declares {section} withheld but the section is present")]
    UndisclosedSectionPresent { section: &'static str },
    #[error("disclosure declares {section} disclosed but the section is absent")]
    DisclosedSectionMissing { section: &'static str },
    #[error("duplicate {kind} identifier {id}")]
    DuplicateId { kind: &'static str, id: u64 },
    #[error("figure {figure_id} references unknown {kind} {reference}")]
    DanglingReference {
        figure_id: u64,
        kind: &'static str,
        reference: u64,
    },
    #[error(
        "dataset {dataset_id} trace {trace_index} has {trace_len} samples but the sweep has {sweep_len}"
    )]
    TraceLengthMismatch {
        dataset_id: u64,
        trace_index: usize,
        trace_len: usize,
        sweep_len: usize,
    },
    #[error("dataset {dataset_id} sweep is empty")]
    EmptySweep { dataset_id: u64 },
    #[error("scene has a zero-area extent")]
    EmptyScene,
    #[error(
        "scene path is malformed: it must begin with a move or arc, use positive stroke widths, radii, and text heights"
    )]
    MalformedPath,
    #[error("scene path declares neither stroke nor fill")]
    UnpaintedPath,
    #[error("measurement {name} references unknown analysis {analysis_id}")]
    MeasurementDanglingAnalysis { name: String, analysis_id: u64 },
    #[error("dataset {dataset_id} references unknown analysis {analysis_id}")]
    DatasetDanglingAnalysis { dataset_id: u64, analysis_id: u64 },
    #[error("payload reference digest must be 64 lowercase hexadecimal characters")]
    MalformedDigest,
    #[error("figure manifest entry declares an empty payload path")]
    EmptyPayloadPath,
    #[error("{field} is not a valid identifier for its role")]
    MalformedIdentifier { field: &'static str },
    #[error("{kind} count exceeds the {limit}-element bound")]
    TooMany { kind: &'static str, limit: usize },
    #[error("dataset {dataset_id} carries a non-finite sample")]
    NonFiniteSample { dataset_id: u64 },
    #[error("measurement {name} carries a non-finite value")]
    NonFiniteMeasurement { name: String },
    #[error("figure payload byte length must be positive and at most {MAX_FIGURE_PAYLOAD_BYTES}")]
    PayloadLengthOutOfRange,
    #[error("schema v2 snapshot cannot carry the v3 field {field}")]
    V3FieldInLegacySnapshot { field: &'static str },
    #[error("publication section {section} is duplicated")]
    DuplicateSection { section: &'static str },
    #[error("default publication section is not present in section_order")]
    DefaultSectionMissing,
    #[error("v3 {kind} named {value} is duplicated")]
    DuplicateNamedIdentity { kind: &'static str, value: String },
    #[error("v3 {kind} references unknown {target_kind} {target}")]
    DanglingNamedReference {
        kind: &'static str,
        target_kind: &'static str,
        target: String,
    },
    #[error("signal identity duplicates dataset {dataset_id} trace {trace_index}")]
    DuplicateSignalIdentity { dataset_id: u64, trace_index: u32 },
    #[error("v3 field {field} carries non-finite IEEE-754 bits")]
    NonFiniteV3Value { field: &'static str },
}

/// Self-validation implemented by every root type in this contract.
pub trait Validate {
    /// Check every structural invariant this schema promises its consumers.
    ///
    /// A value that fails validation must never be serialized, stored, or
    /// rendered.
    fn validate(&self) -> Result<(), ContractError>;
}

// ---------------------------------------------------------------------------
// Shared text validation
// ---------------------------------------------------------------------------

/// Reject control characters. Multi-line fields allow `\n`; nothing else
/// below U+0020 is ever legitimate in publication text.
fn validate_text(
    value: &str,
    field: &'static str,
    limit: usize,
    allow_newlines: bool,
) -> Result<(), ContractError> {
    if value.chars().count() > limit {
        return Err(ContractError::FieldTooLong { field, limit });
    }
    let clean = value
        .chars()
        .all(|c| !c.is_control() || (allow_newlines && (c == '\n' || c == '\t')));
    if !clean {
        return Err(ContractError::ControlCharacter { field });
    }
    Ok(())
}

fn validate_required_text(
    value: &str,
    field: &'static str,
    limit: usize,
) -> Result<(), ContractError> {
    if value.trim().is_empty() {
        return Err(ContractError::EmptyField { field });
    }
    validate_text(value, field, limit, false)
}

fn validate_required_multiline(
    value: &str,
    field: &'static str,
    limit: usize,
) -> Result<(), ContractError> {
    if value.trim().is_empty() {
        return Err(ContractError::EmptyField { field });
    }
    validate_text(value, field, limit, true)
}

// ---------------------------------------------------------------------------
// Envelope
// ---------------------------------------------------------------------------

/// Root of the sealed publication interchange format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationSnapshot {
    /// Version 2 and [`PUBLICATION_SNAPSHOT_SCHEMA_VERSION`] are accepted.
    pub schema_version: u32,
    pub metadata: PublicationMetadata,
    pub disclosure: Disclosure,
    /// Present exactly when [`Disclosure::schematic`] is true.
    pub schematic: Option<SchematicSection>,
    /// Present exactly when [`Disclosure::netlist`] is true.
    pub netlist: Option<NetlistSection>,
    /// Present exactly when [`Disclosure::results`] is true.
    pub results: Option<ResultsSection>,
    /// Author-curated page figures, in page order.
    pub figures: Vec<Figure>,
    /// Schema-v3 page information architecture and authored summaries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation: Option<PublicationPresentation>,
    /// Schema-v3 typed engineering identity and simulation provenance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engineering: Option<EngineeringPublication>,
}

/// Document-level facts rendered into the page head, provenance footer, and
/// unfurl metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationMetadata {
    pub title: String,
    /// May be empty; multi-line text is allowed.
    pub description: String,
    pub author_display: String,
    /// Producing client version string, e.g. `0.1.0`.
    pub app_version: String,
    /// RFC 3339 UTC creation instant supplied by the producing client.
    pub created_utc: String,
    /// The author's chosen terms for the published content.
    pub license: ContentLicense,
}

/// The author's chosen content license, sealed into the snapshot so every
/// copy of the bytes carries its terms. A closed set: the page renders the
/// exact name, so free-form license text can never smuggle markup.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContentLicense {
    /// CERN Open Hardware Licence Version 2 — Permissive.
    CernOhlP2,
    /// Creative Commons Attribution 4.0 International.
    CcBy40,
    /// No reuse terms granted; the page is view-only.
    #[default]
    AllRightsReserved,
}

impl ContentLicense {
    /// Presentation name rendered on the published page.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::CernOhlP2 => "CERN-OHL-P-2.0",
            Self::CcBy40 => "CC-BY-4.0",
            Self::AllRightsReserved => "All rights reserved",
        }
    }
}

/// What the author chose to reveal. Sections withheld here are absent from
/// the snapshot entirely — disclosure is subtraction at build time, and
/// [`Validate::validate`] rejects any snapshot where flags and content
/// disagree in either direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Disclosure {
    pub schematic: bool,
    pub netlist: bool,
    pub results: bool,
    /// Whether the sealed project archive is published alongside the page.
    /// The archive bytes travel as a sibling artifact, never inside the
    /// snapshot.
    pub archive: bool,
}

// ---------------------------------------------------------------------------
// Schema v3 presentation and engineering identity
// ---------------------------------------------------------------------------

/// Semantic page sections. The renderer chooses responsive layout; the
/// snapshot controls only authored order and the initial section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PublicationSection {
    Overview,
    Schematic,
    Results,
    Components,
    Files,
    Details,
}

impl PublicationSection {
    const fn name(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Schematic => "schematic",
            Self::Results => "results",
            Self::Components => "components",
            Self::Files => "files",
            Self::Details => "details",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationPresentation {
    pub overview: Option<PublicationOverview>,
    pub section_order: Vec<PublicationSection>,
    pub default_section: PublicationSection,
    pub featured_figure_id: Option<u64>,
    pub figure_details: Vec<FigurePresentation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationOverview {
    /// Authored design intent, constraints, and conclusions.
    pub narrative: String,
    pub specifications: Vec<Specification>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Specification {
    pub label: String,
    pub value: String,
    pub unit: Option<String>,
}

/// Optional authored presentation attached to a figure by immutable id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FigurePresentation {
    pub figure_id: u64,
    pub caption: Option<String>,
    /// Plain-text alternative explaining the engineering conclusion.
    pub accessible_summary: String,
    pub default_interactive: bool,
}

/// Typed records used by component tables, inspectors, cross-probing, and
/// reproducibility details. No local path or model source bytes may appear.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineeringPublication {
    pub components: Vec<ComponentRecord>,
    pub nets: Vec<NetRecord>,
    pub signals: Vec<SignalIdentity>,
    pub simulation: Option<SimulationProvenance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentRecord {
    pub reference: String,
    pub value: String,
    pub device: String,
    pub model: Option<ModelReference>,
    pub pins: Vec<ComponentPin>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentPin {
    pub name: String,
    pub number: Option<String>,
    pub net: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelReference {
    pub name: String,
    pub device_class: String,
    /// Public library/package label only, never a local filesystem path.
    pub library: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetRecord {
    pub name: String,
    pub connections: Vec<NetConnection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetConnection {
    pub component_reference: String,
    pub pin_name: String,
}

/// Explicit identity for one dataset trace. Consumers never infer identity
/// by parsing display labels such as `V(out)` or `I(R1)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalIdentity {
    pub dataset_id: u64,
    pub trace_index: u32,
    pub target: SignalTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignalTarget {
    NetVoltage { net: String },
    DeviceCurrent { reference: String },
    Expression { label: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationProvenance {
    pub engine: String,
    pub engine_version: String,
    pub temperature_c_bits: Option<u64>,
    pub corner: Option<String>,
    pub settings: Vec<SimulationSetting>,
    pub warnings: Vec<SimulationWarning>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationSetting {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationWarning {
    pub severity: WarningSeverity,
    pub message: String,
    pub analysis_id: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WarningSeverity {
    Information,
    Warning,
    Error,
}

// ---------------------------------------------------------------------------
// Resolved scenes (display list)
// ---------------------------------------------------------------------------

/// One exact point in integral micrometres.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
    pub x_um: i64,
    pub y_um: i64,
}

/// Semantic paint role, mirroring the display-semantic color classes the
/// RSpice scene pipeline resolves every drawn object into. Renderers map
/// roles onto their palette so a page can present light and dark themes from
/// one sealed scene; explicit RGBA is reserved for author-picked colors that
/// must survive verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaintRole {
    /// Primary drawing ink: symbol bodies, wires, frames, labels.
    Foreground,
    /// De-emphasized ink: annotations, documentation, secondary chrome.
    Secondary,
    /// Grid and minor guide lines.
    Grid,
    /// Highlighted engineering accents: pins, markers.
    Accent,
    /// Failure and violation emphasis.
    Warning,
    /// Success and pass emphasis.
    Success,
    /// Zero-based series index into the renderer's trace palette.
    TraceSeries(u8),
}

/// Paint selection for strokes, fills, and text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Paint {
    Role(PaintRole),
    Rgba([u8; 4]),
}

/// Stroke pattern vocabulary. Dash geometry is a renderer decision keyed off
/// the pattern, keeping sealed scenes independent of output styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StrokePattern {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stroke {
    pub width_um: u64,
    pub paint: Paint,
    pub pattern: StrokePattern,
}

/// One path step. Arc angles are integral millidegrees, counter-clockwise
/// positive, measured from the positive x-axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PathSegment {
    MoveTo {
        to: Point,
    },
    LineTo {
        to: Point,
    },
    Arc {
        center: Point,
        radius_um: u64,
        start_millideg: i32,
        sweep_millideg: i32,
    },
    Close,
}

/// Horizontal text anchoring relative to the origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

/// Typeface class for scene text, mirroring the scene pipeline's font
/// vocabulary. Concrete font files are a renderer decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextFont {
    Sans,
    SansSemibold,
    Monospace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathPrimitive {
    pub segments: Vec<PathSegment>,
    pub stroke: Option<Stroke>,
    pub fill: Option<Paint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextPrimitive {
    pub origin: Point,
    pub text: String,
    pub height_um: u64,
    pub font: TextFont,
    pub anchor: TextAnchor,
    /// Integral millidegrees, counter-clockwise positive.
    pub rotation_millideg: i32,
    pub paint: Paint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Primitive {
    Path(PathPrimitive),
    Text(TextPrimitive),
}

/// Optional semantic identity for a primitive group, used for hover and
/// cross-probe affordances in hydrated figures and for stable SVG grouping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GroupTag {
    Instance { reference: String },
    Net { name: String },
    SheetFrame,
    Annotation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrimitiveGroup {
    pub tag: Option<GroupTag>,
    pub primitives: Vec<Primitive>,
}

/// A complete resolved drawing in physical units. `width_um`/`height_um`
/// define the viewport; primitive coordinates live in the same space with
/// the origin at the top-left corner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub width_um: u64,
    pub height_um: u64,
    pub groups: Vec<PrimitiveGroup>,
}

// ---------------------------------------------------------------------------
// Content sections
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetScene {
    pub name: String,
    /// Print-set page label when the project assigns one.
    pub page_label: Option<String>,
    pub scene: Scene,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicSection {
    pub sheets: Vec<SheetScene>,
}

/// The exact deck text the run consumed. The first line is the SPICE title
/// line by definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetlistSection {
    pub deck: String,
}

impl NetlistSection {
    /// The SPICE title: the deck's first line.
    #[must_use]
    pub fn title(&self) -> &str {
        self.deck.lines().next().unwrap_or_default()
    }
}

/// One analysis the author ran, in run order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisRecord {
    pub id: u64,
    /// Human-readable label, e.g. `Transient · 0–1 ms`.
    pub label: String,
    /// The SPICE control card, e.g. `.tran 1u 1m`.
    pub card: String,
}

/// Sample vector encodings. Values are IEEE-754 binary64 bit patterns so
/// serialization is exact and reproducible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TraceValues {
    Real {
        bits: Vec<u64>,
    },
    Complex {
        real_bits: Vec<u64>,
        imaginary_bits: Vec<u64>,
    },
}

impl TraceValues {
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Real { bits } => bits.len(),
            Self::Complex { real_bits, .. } => real_bits.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// The independent variable of a dataset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SweepAxis {
    /// e.g. `time`, `frequency`, `v-sweep`.
    pub label: String,
    /// Plain-text unit, e.g. `s`, `Hz`, `V`. May be empty.
    pub unit: String,
    /// IEEE-754 binary64 bit patterns.
    pub values_bits: Vec<u64>,
}

/// One dependent vector of a dataset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trace {
    /// e.g. `V(out)`, `I(V1)`.
    pub label: String,
    /// Plain-text unit. May be empty.
    pub unit: String,
    pub values: TraceValues,
}

/// One rectangular result table: a sweep axis plus aligned traces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dataset {
    pub id: u64,
    pub analysis_id: u64,
    pub name: String,
    /// Distinguishes stepped/corner variants of one analysis. `None` for a
    /// single-run dataset.
    pub variant: Option<String>,
    pub sweep: SweepAxis,
    pub traces: Vec<Trace>,
}

/// One scalar measurement the author published, as the measurement strip
/// presented it: value, declared specification, and pass state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Measurement {
    pub analysis_id: u64,
    pub name: String,
    /// IEEE-754 binary64 bit pattern of the value, when numeric.
    pub value_bits: Option<u64>,
    /// Exact display string the author saw, e.g. `3.162 µs`.
    pub display: String,
    /// Declared specification in display form, e.g. `≥ 40 dB`.
    pub spec_display: Option<String>,
    /// Pass state against the declared specification; `None` when no
    /// specification was declared.
    pub passed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultsSection {
    pub analyses: Vec<AnalysisRecord>,
    pub datasets: Vec<Dataset>,
    pub measurements: Vec<Measurement>,
}

// ---------------------------------------------------------------------------
// Figures
// ---------------------------------------------------------------------------

/// Axis mapping the viewer must reproduce when hydrating a plot figure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AxisScale {
    Linear,
    Logarithmic,
}

/// How a bound trace's stored values become plotted ordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TraceTransform {
    Identity,
    Magnitude,
    MagnitudeDb,
    PhaseDegrees,
    RealPart,
    ImaginaryPart,
}

/// One trace binding inside a hydratable plot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlotTraceBinding {
    pub dataset_id: u64,
    /// Index into the dataset's `traces`.
    pub trace_index: u32,
    pub transform: TraceTransform,
}

/// Everything the viewer needs to rebuild a live plot from datasets alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlotHydration {
    pub x_scale: AxisScale,
    pub y_scale: AxisScale,
    pub x_label: String,
    pub y_label: String,
    pub bindings: Vec<PlotTraceBinding>,
}

/// A plot figure: the exact scene the author saw, plus optional hydration
/// bindings when the page's datasets can drive a live instrument.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlotFigure {
    pub scene: Scene,
    pub hydration: Option<PlotHydration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FigureContent {
    /// Index into [`SchematicSection::sheets`].
    SchematicSheet {
        sheet_index: u32,
    },
    Plot(PlotFigure),
}

/// One author-curated page figure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Figure {
    pub id: u64,
    pub title: String,
    pub content: FigureContent,
}

// ---------------------------------------------------------------------------
// Figure manifest and hydration payloads (renderer → viewer)
// ---------------------------------------------------------------------------

/// Reference to one sealed sibling asset inside the page bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PayloadRef {
    /// Bundle-relative path, e.g. `figures/3.json`.
    pub path: String,
    /// Lowercase hex SHA-256 of the payload bytes.
    pub sha256_hex: String,
    pub byte_len: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ManifestFigureKind {
    SchematicSheet,
    Plot,
}

/// One hydratable figure advertised to the viewer runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestEntry {
    pub figure_id: u64,
    /// The DOM id of the figure's mount point in the rendered page.
    pub dom_id: String,
    pub kind: ManifestFigureKind,
    pub payload: PayloadRef,
}

/// Written by `rspice-publish` into the page bundle; read by the viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FigureManifest {
    /// Must equal [`FIGURE_MANIFEST_SCHEMA_VERSION`] exactly.
    pub schema_version: u32,
    pub figures: Vec<ManifestEntry>,
}

/// The self-contained per-figure asset the viewer fetches on activation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FigurePayload {
    /// Must equal [`FIGURE_MANIFEST_SCHEMA_VERSION`] exactly; the payload
    /// format is versioned together with the manifest that references it.
    pub schema_version: u32,
    pub figure_id: u64,
    pub scene: Scene,
    /// Datasets backing the hydration bindings. Empty for schematic sheets.
    pub datasets: Vec<Dataset>,
    pub hydration: Option<PlotHydration>,
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn require_finite_bits(bits: u64, dataset_id: u64) -> Result<(), ContractError> {
    if f64::from_bits(bits).is_finite() {
        Ok(())
    } else {
        Err(ContractError::NonFiniteSample { dataset_id })
    }
}

fn require_bound(count: usize, limit: usize, kind: &'static str) -> Result<(), ContractError> {
    if count > limit {
        return Err(ContractError::TooMany { kind, limit });
    }
    Ok(())
}

fn require_unique_ids<'a>(
    ids: impl Iterator<Item = &'a u64>,
    kind: &'static str,
) -> Result<(), ContractError> {
    let mut seen = Vec::new();
    for &id in ids {
        if seen.contains(&id) {
            return Err(ContractError::DuplicateId { kind, id });
        }
        seen.push(id);
    }
    Ok(())
}

/// Identifiers embedded into DOM ids must never need escaping.
fn validate_dom_id(value: &str, field: &'static str) -> Result<(), ContractError> {
    if value.is_empty() || value.chars().count() > MAX_DOM_ID_CHARS {
        return Err(ContractError::MalformedIdentifier { field });
    }
    let mut chars = value.chars();
    let leading_alphabetic = chars.next().is_some_and(|c| c.is_ascii_alphabetic());
    if !leading_alphabetic
        || !value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(ContractError::MalformedIdentifier { field });
    }
    Ok(())
}

/// Bundle-relative paths must stay inside the bundle: no absolute paths, no
/// parent traversal, no backslashes, and a conservative character set.
fn validate_payload_path(value: &str) -> Result<(), ContractError> {
    if value.is_empty() {
        return Err(ContractError::EmptyPayloadPath);
    }
    if value.chars().count() > MAX_PAYLOAD_PATH_CHARS {
        return Err(ContractError::MalformedIdentifier {
            field: "payload path",
        });
    }
    let charset_ok = value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '/'));
    let segments_ok = !value.starts_with('/')
        && !value.ends_with('/')
        && value.split('/').all(|segment| {
            !segment.is_empty() && segment != "." && segment != ".." && !segment.starts_with('.')
        });
    if !charset_ok || !segments_ok {
        return Err(ContractError::MalformedIdentifier {
            field: "payload path",
        });
    }
    Ok(())
}

impl Validate for Scene {
    fn validate(&self) -> Result<(), ContractError> {
        if self.width_um == 0 || self.height_um == 0 {
            return Err(ContractError::EmptyScene);
        }
        require_bound(self.groups.len(), MAX_GROUPS_PER_SCENE, "scene groups")?;
        let primitive_count: usize = self.groups.iter().map(|g| g.primitives.len()).sum();
        require_bound(
            primitive_count,
            MAX_PRIMITIVES_PER_SCENE,
            "scene primitives",
        )?;
        for group in &self.groups {
            if let Some(tag) = &group.tag {
                match tag {
                    GroupTag::Instance { reference } => {
                        validate_required_text(
                            reference,
                            "group instance reference",
                            MAX_LABEL_CHARS,
                        )?;
                    }
                    GroupTag::Net { name } => {
                        validate_required_text(name, "group net name", MAX_LABEL_CHARS)?;
                    }
                    GroupTag::SheetFrame | GroupTag::Annotation => {}
                }
            }
            for primitive in &group.primitives {
                match primitive {
                    Primitive::Path(path) => {
                        require_bound(path.segments.len(), MAX_SEGMENTS_PER_PATH, "path segments")?;
                        let starts_positioned = matches!(
                            path.segments.first(),
                            Some(PathSegment::MoveTo { .. } | PathSegment::Arc { .. })
                        );
                        if !starts_positioned {
                            return Err(ContractError::MalformedPath);
                        }
                        if path.stroke.is_none() && path.fill.is_none() {
                            return Err(ContractError::UnpaintedPath);
                        }
                        if let Some(stroke) = &path.stroke
                            && stroke.width_um == 0
                        {
                            return Err(ContractError::MalformedPath);
                        }
                        for segment in &path.segments {
                            if let PathSegment::Arc { radius_um, .. } = segment
                                && *radius_um == 0
                            {
                                return Err(ContractError::MalformedPath);
                            }
                        }
                    }
                    Primitive::Text(text) => {
                        validate_required_text(&text.text, "scene text", MAX_TEXT_CHARS)?;
                        if text.height_um == 0 {
                            return Err(ContractError::MalformedPath);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn validate_analyses(analyses: &[AnalysisRecord]) -> Result<(), ContractError> {
    require_bound(analyses.len(), MAX_ANALYSES, "analyses")?;
    require_unique_ids(analyses.iter().map(|a| &a.id), "analysis")?;
    for analysis in analyses {
        validate_required_text(&analysis.label, "analysis label", MAX_LABEL_CHARS)?;
        validate_required_text(&analysis.card, "analysis card", MAX_LABEL_CHARS)?;
    }
    Ok(())
}

/// Validate datasets, checking analysis references only when the owning
/// container carries analysis records (figure payloads do not).
fn validate_datasets(
    datasets: &[Dataset],
    analyses: Option<&[AnalysisRecord]>,
) -> Result<(), ContractError> {
    require_bound(datasets.len(), MAX_DATASETS, "datasets")?;
    require_unique_ids(datasets.iter().map(|d| &d.id), "dataset")?;
    let mut total_values: usize = 0;
    for dataset in datasets {
        if let Some(analyses) = analyses
            && !analyses.iter().any(|a| a.id == dataset.analysis_id)
        {
            return Err(ContractError::DatasetDanglingAnalysis {
                dataset_id: dataset.id,
                analysis_id: dataset.analysis_id,
            });
        }
        validate_required_text(&dataset.name, "dataset name", MAX_LABEL_CHARS)?;
        if let Some(variant) = &dataset.variant {
            validate_required_text(variant, "dataset variant", MAX_LABEL_CHARS)?;
        }
        validate_required_text(&dataset.sweep.label, "sweep label", MAX_LABEL_CHARS)?;
        validate_text(&dataset.sweep.unit, "sweep unit", MAX_UNIT_CHARS, false)?;
        let sweep_len = dataset.sweep.values_bits.len();
        if sweep_len == 0 {
            return Err(ContractError::EmptySweep {
                dataset_id: dataset.id,
            });
        }
        for &bits in &dataset.sweep.values_bits {
            require_finite_bits(bits, dataset.id)?;
        }
        total_values = total_values.saturating_add(sweep_len);
        require_bound(
            dataset.traces.len(),
            MAX_TRACES_PER_DATASET,
            "dataset traces",
        )?;
        for (trace_index, trace) in dataset.traces.iter().enumerate() {
            validate_required_text(&trace.label, "trace label", MAX_LABEL_CHARS)?;
            validate_text(&trace.unit, "trace unit", MAX_UNIT_CHARS, false)?;
            let trace_len = trace.values.len();
            if trace_len != sweep_len {
                return Err(ContractError::TraceLengthMismatch {
                    dataset_id: dataset.id,
                    trace_index,
                    trace_len,
                    sweep_len,
                });
            }
            match &trace.values {
                TraceValues::Real { bits } => {
                    for &b in bits {
                        require_finite_bits(b, dataset.id)?;
                    }
                    total_values = total_values.saturating_add(bits.len());
                }
                TraceValues::Complex {
                    real_bits,
                    imaginary_bits,
                } => {
                    if imaginary_bits.len() != real_bits.len() {
                        return Err(ContractError::TraceLengthMismatch {
                            dataset_id: dataset.id,
                            trace_index,
                            trace_len: imaginary_bits.len(),
                            sweep_len,
                        });
                    }
                    for &b in real_bits.iter().chain(imaginary_bits) {
                        require_finite_bits(b, dataset.id)?;
                    }
                    total_values = total_values
                        .saturating_add(real_bits.len())
                        .saturating_add(imaginary_bits.len());
                }
            }
        }
    }
    require_bound(total_values, MAX_TOTAL_SAMPLE_VALUES, "sample values")?;
    Ok(())
}

fn validate_hydration(
    hydration: &PlotHydration,
    datasets: &[Dataset],
    figure_id: u64,
) -> Result<(), ContractError> {
    validate_text(
        &hydration.x_label,
        "hydration x label",
        MAX_LABEL_CHARS,
        false,
    )?;
    validate_text(
        &hydration.y_label,
        "hydration y label",
        MAX_LABEL_CHARS,
        false,
    )?;
    for binding in &hydration.bindings {
        let Some(dataset) = datasets.iter().find(|d| d.id == binding.dataset_id) else {
            return Err(ContractError::DanglingReference {
                figure_id,
                kind: "dataset",
                reference: binding.dataset_id,
            });
        };
        if usize::try_from(binding.trace_index)
            .ok()
            .is_none_or(|index| index >= dataset.traces.len())
        {
            return Err(ContractError::DanglingReference {
                figure_id,
                kind: "trace",
                reference: u64::from(binding.trace_index),
            });
        }
    }
    Ok(())
}

fn validate_specifications(specifications: &[Specification]) -> Result<(), ContractError> {
    require_bound(
        specifications.len(),
        MAX_SPECIFICATIONS,
        "overview specifications",
    )?;
    for specification in specifications {
        validate_required_text(&specification.label, "specification label", MAX_LABEL_CHARS)?;
        validate_required_text(&specification.value, "specification value", MAX_TEXT_CHARS)?;
        if let Some(unit) = &specification.unit {
            validate_required_text(unit, "specification unit", MAX_UNIT_CHARS)?;
        }
    }
    Ok(())
}

fn validate_presentation(
    snapshot: &PublicationSnapshot,
    presentation: &PublicationPresentation,
) -> Result<(), ContractError> {
    if let Some(overview) = &presentation.overview {
        validate_required_multiline(
            &overview.narrative,
            "overview narrative",
            MAX_NARRATIVE_CHARS,
        )?;
        validate_specifications(&overview.specifications)?;
    }
    if presentation.section_order.is_empty() {
        return Err(ContractError::EmptyField {
            field: "section order",
        });
    }
    require_bound(presentation.section_order.len(), 6, "publication sections")?;
    for (index, section) in presentation.section_order.iter().enumerate() {
        if presentation.section_order[..index].contains(section) {
            return Err(ContractError::DuplicateSection {
                section: section.name(),
            });
        }
        let available = match section {
            PublicationSection::Overview | PublicationSection::Details => true,
            PublicationSection::Schematic => snapshot.disclosure.schematic,
            PublicationSection::Results => snapshot.disclosure.results,
            PublicationSection::Components => snapshot
                .engineering
                .as_ref()
                .is_some_and(|engineering| !engineering.components.is_empty()),
            PublicationSection::Files => {
                snapshot.disclosure.netlist
                    || snapshot.disclosure.results
                    || snapshot.disclosure.archive
            }
        };
        if !available {
            return Err(ContractError::DanglingNamedReference {
                kind: "section order",
                target_kind: "disclosed section",
                target: section.name().to_string(),
            });
        }
    }
    if !presentation
        .section_order
        .contains(&presentation.default_section)
    {
        return Err(ContractError::DefaultSectionMissing);
    }
    if let Some(featured) = presentation.featured_figure_id
        && !snapshot.figures.iter().any(|figure| figure.id == featured)
    {
        return Err(ContractError::DanglingReference {
            figure_id: featured,
            kind: "featured figure",
            reference: featured,
        });
    }
    require_bound(
        presentation.figure_details.len(),
        MAX_FIGURES,
        "figure presentation records",
    )?;
    require_unique_ids(
        presentation
            .figure_details
            .iter()
            .map(|detail| &detail.figure_id),
        "figure presentation",
    )?;
    for detail in &presentation.figure_details {
        if !snapshot
            .figures
            .iter()
            .any(|figure| figure.id == detail.figure_id)
        {
            return Err(ContractError::DanglingReference {
                figure_id: detail.figure_id,
                kind: "figure presentation",
                reference: detail.figure_id,
            });
        }
        if let Some(caption) = &detail.caption {
            validate_required_multiline(caption, "figure caption", MAX_NARRATIVE_CHARS)?;
        }
        validate_required_multiline(
            &detail.accessible_summary,
            "figure accessible summary",
            MAX_NARRATIVE_CHARS,
        )?;
    }
    Ok(())
}

fn validate_engineering(
    snapshot: &PublicationSnapshot,
    engineering: &EngineeringPublication,
) -> Result<(), ContractError> {
    require_bound(engineering.components.len(), MAX_COMPONENTS, "components")?;
    require_bound(engineering.nets.len(), MAX_NETS, "nets")?;
    require_bound(
        engineering.signals.len(),
        MAX_SIGNAL_IDENTITIES,
        "signal identities",
    )?;

    for (index, component) in engineering.components.iter().enumerate() {
        validate_required_text(&component.reference, "component reference", MAX_LABEL_CHARS)?;
        if engineering.components[..index]
            .iter()
            .any(|seen| seen.reference == component.reference)
        {
            return Err(ContractError::DuplicateNamedIdentity {
                kind: "component",
                value: component.reference.clone(),
            });
        }
        validate_text(&component.value, "component value", MAX_LABEL_CHARS, false)?;
        validate_required_text(&component.device, "component device", MAX_LABEL_CHARS)?;
        require_bound(
            component.pins.len(),
            MAX_PINS_PER_COMPONENT,
            "component pins",
        )?;
        for (pin_index, pin) in component.pins.iter().enumerate() {
            validate_required_text(&pin.name, "component pin name", MAX_LABEL_CHARS)?;
            if component.pins[..pin_index]
                .iter()
                .any(|seen| seen.name == pin.name)
            {
                return Err(ContractError::DuplicateNamedIdentity {
                    kind: "component pin",
                    value: format!("{}.{}", component.reference, pin.name),
                });
            }
            if let Some(number) = &pin.number {
                validate_required_text(number, "component pin number", MAX_LABEL_CHARS)?;
            }
            if let Some(net) = &pin.net {
                validate_required_text(net, "component pin net", MAX_LABEL_CHARS)?;
            }
        }
        if let Some(model) = &component.model {
            validate_required_text(&model.name, "model name", MAX_LABEL_CHARS)?;
            validate_required_text(&model.device_class, "model device class", MAX_LABEL_CHARS)?;
            if let Some(library) = &model.library {
                validate_required_text(library, "model library", MAX_LABEL_CHARS)?;
                if library.contains('/') || library.contains('\\') || library.contains(':') {
                    return Err(ContractError::MalformedIdentifier {
                        field: "model library",
                    });
                }
            }
        }
    }

    let mut connection_count = 0usize;
    for (index, net) in engineering.nets.iter().enumerate() {
        validate_required_text(&net.name, "net name", MAX_LABEL_CHARS)?;
        if engineering.nets[..index]
            .iter()
            .any(|seen| seen.name == net.name)
        {
            return Err(ContractError::DuplicateNamedIdentity {
                kind: "net",
                value: net.name.clone(),
            });
        }
        connection_count = connection_count.saturating_add(net.connections.len());
        for connection in &net.connections {
            validate_required_text(
                &connection.component_reference,
                "net component reference",
                MAX_LABEL_CHARS,
            )?;
            validate_required_text(&connection.pin_name, "net pin name", MAX_LABEL_CHARS)?;
            let Some(component) = engineering
                .components
                .iter()
                .find(|component| component.reference == connection.component_reference)
            else {
                return Err(ContractError::DanglingNamedReference {
                    kind: "net connection",
                    target_kind: "component",
                    target: connection.component_reference.clone(),
                });
            };
            if !component
                .pins
                .iter()
                .any(|pin| pin.name == connection.pin_name)
            {
                return Err(ContractError::DanglingNamedReference {
                    kind: "net connection",
                    target_kind: "component pin",
                    target: format!("{}.{}", connection.component_reference, connection.pin_name),
                });
            }
        }
    }
    require_bound(connection_count, MAX_NET_CONNECTIONS, "net connections")?;
    if !engineering.nets.is_empty() {
        for component in &engineering.components {
            for pin in &component.pins {
                if let Some(net) = &pin.net
                    && !engineering
                        .nets
                        .iter()
                        .any(|candidate| candidate.name == *net)
                {
                    return Err(ContractError::DanglingNamedReference {
                        kind: "component pin",
                        target_kind: "net",
                        target: net.clone(),
                    });
                }
            }
        }
    }

    for (index, signal) in engineering.signals.iter().enumerate() {
        if engineering.signals[..index].iter().any(|seen| {
            seen.dataset_id == signal.dataset_id && seen.trace_index == signal.trace_index
        }) {
            return Err(ContractError::DuplicateSignalIdentity {
                dataset_id: signal.dataset_id,
                trace_index: signal.trace_index,
            });
        }
        let Some(dataset) = snapshot.results.as_ref().and_then(|results| {
            results
                .datasets
                .iter()
                .find(|dataset| dataset.id == signal.dataset_id)
        }) else {
            return Err(ContractError::DanglingReference {
                figure_id: 0,
                kind: "signal dataset",
                reference: signal.dataset_id,
            });
        };
        if usize::try_from(signal.trace_index)
            .ok()
            .is_none_or(|trace_index| trace_index >= dataset.traces.len())
        {
            return Err(ContractError::DanglingReference {
                figure_id: 0,
                kind: "signal trace",
                reference: u64::from(signal.trace_index),
            });
        }
        match &signal.target {
            SignalTarget::NetVoltage { net } => {
                validate_required_text(net, "signal net", MAX_LABEL_CHARS)?;
                if !engineering
                    .nets
                    .iter()
                    .any(|candidate| candidate.name == *net)
                {
                    return Err(ContractError::DanglingNamedReference {
                        kind: "signal identity",
                        target_kind: "net",
                        target: net.clone(),
                    });
                }
            }
            SignalTarget::DeviceCurrent { reference } => {
                validate_required_text(reference, "signal component", MAX_LABEL_CHARS)?;
                if !engineering
                    .components
                    .iter()
                    .any(|component| component.reference == *reference)
                {
                    return Err(ContractError::DanglingNamedReference {
                        kind: "signal identity",
                        target_kind: "component",
                        target: reference.clone(),
                    });
                }
            }
            SignalTarget::Expression { label } => {
                validate_required_text(label, "signal expression", MAX_LABEL_CHARS)?;
            }
        }
    }

    if let Some(simulation) = &engineering.simulation {
        validate_required_text(&simulation.engine, "simulation engine", MAX_LABEL_CHARS)?;
        validate_required_text(
            &simulation.engine_version,
            "simulation engine version",
            MAX_APP_VERSION_CHARS,
        )?;
        if let Some(bits) = simulation.temperature_c_bits
            && !f64::from_bits(bits).is_finite()
        {
            return Err(ContractError::NonFiniteV3Value {
                field: "simulation temperature",
            });
        }
        if let Some(corner) = &simulation.corner {
            validate_required_text(corner, "simulation corner", MAX_LABEL_CHARS)?;
        }
        require_bound(
            simulation.settings.len(),
            MAX_SPECIFICATIONS,
            "simulation settings",
        )?;
        for setting in &simulation.settings {
            validate_required_text(&setting.name, "simulation setting name", MAX_LABEL_CHARS)?;
            validate_required_text(&setting.value, "simulation setting value", MAX_TEXT_CHARS)?;
        }
        require_bound(
            simulation.warnings.len(),
            MAX_SIMULATION_WARNINGS,
            "simulation warnings",
        )?;
        for warning in &simulation.warnings {
            validate_required_multiline(
                &warning.message,
                "simulation warning",
                MAX_NARRATIVE_CHARS,
            )?;
            if let Some(analysis_id) = warning.analysis_id
                && snapshot.results.as_ref().is_none_or(|results| {
                    !results
                        .analyses
                        .iter()
                        .any(|analysis| analysis.id == analysis_id)
                })
            {
                return Err(ContractError::DanglingReference {
                    figure_id: 0,
                    kind: "warning analysis",
                    reference: analysis_id,
                });
            }
        }
    }
    Ok(())
}

impl Validate for PublicationSnapshot {
    fn validate(&self) -> Result<(), ContractError> {
        if ![
            PUBLICATION_SNAPSHOT_V2_SCHEMA_VERSION,
            PUBLICATION_SNAPSHOT_SCHEMA_VERSION,
        ]
        .contains(&self.schema_version)
        {
            return Err(ContractError::UnsupportedSchemaVersion {
                expected: PUBLICATION_SNAPSHOT_SCHEMA_VERSION,
                found: self.schema_version,
            });
        }
        if self.schema_version == PUBLICATION_SNAPSHOT_V2_SCHEMA_VERSION {
            if self.presentation.is_some() {
                return Err(ContractError::V3FieldInLegacySnapshot {
                    field: "presentation",
                });
            }
            if self.engineering.is_some() {
                return Err(ContractError::V3FieldInLegacySnapshot {
                    field: "engineering",
                });
            }
        }

        validate_required_text(&self.metadata.title, "title", MAX_TITLE_CHARS)?;
        validate_text(
            &self.metadata.description,
            "description",
            MAX_DESCRIPTION_CHARS,
            true,
        )?;
        validate_required_text(&self.metadata.author_display, "author", MAX_AUTHOR_CHARS)?;
        validate_required_text(
            &self.metadata.app_version,
            "app version",
            MAX_APP_VERSION_CHARS,
        )?;
        validate_required_text(
            &self.metadata.created_utc,
            "creation instant",
            MAX_TIMESTAMP_CHARS,
        )?;

        match (self.disclosure.schematic, &self.schematic) {
            (false, Some(_)) => {
                return Err(ContractError::UndisclosedSectionPresent {
                    section: "schematic",
                });
            }
            (true, None) => {
                return Err(ContractError::DisclosedSectionMissing {
                    section: "schematic",
                });
            }
            _ => {}
        }
        match (self.disclosure.netlist, &self.netlist) {
            (false, Some(_)) => {
                return Err(ContractError::UndisclosedSectionPresent { section: "netlist" });
            }
            (true, None) => {
                return Err(ContractError::DisclosedSectionMissing { section: "netlist" });
            }
            _ => {}
        }
        match (self.disclosure.results, &self.results) {
            (false, Some(_)) => {
                return Err(ContractError::UndisclosedSectionPresent { section: "results" });
            }
            (true, None) => {
                return Err(ContractError::DisclosedSectionMissing { section: "results" });
            }
            _ => {}
        }

        if let Some(schematic) = &self.schematic {
            require_bound(schematic.sheets.len(), MAX_SHEETS, "sheets")?;
            for sheet in &schematic.sheets {
                validate_required_text(&sheet.name, "sheet name", MAX_LABEL_CHARS)?;
                if let Some(page_label) = &sheet.page_label {
                    validate_required_text(page_label, "sheet page label", MAX_LABEL_CHARS)?;
                }
                sheet.scene.validate()?;
            }
        }

        if let Some(netlist) = &self.netlist {
            if netlist.deck.trim().is_empty() {
                return Err(ContractError::EmptyField { field: "deck" });
            }
            if netlist.deck.len() > MAX_DECK_BYTES {
                return Err(ContractError::DeckTooLarge);
            }
            validate_text(&netlist.deck, "deck", MAX_DECK_BYTES, true)?;
        }

        if let Some(results) = &self.results {
            validate_analyses(&results.analyses)?;
            validate_datasets(&results.datasets, Some(&results.analyses))?;
            require_bound(results.measurements.len(), MAX_MEASUREMENTS, "measurements")?;
            for measurement in &results.measurements {
                if !results
                    .analyses
                    .iter()
                    .any(|a| a.id == measurement.analysis_id)
                {
                    return Err(ContractError::MeasurementDanglingAnalysis {
                        name: measurement.name.clone(),
                        analysis_id: measurement.analysis_id,
                    });
                }
                validate_required_text(&measurement.name, "measurement name", MAX_LABEL_CHARS)?;
                validate_required_text(
                    &measurement.display,
                    "measurement display",
                    MAX_LABEL_CHARS,
                )?;
                if let Some(spec) = &measurement.spec_display {
                    validate_required_text(spec, "measurement spec", MAX_LABEL_CHARS)?;
                }
                if let Some(bits) = measurement.value_bits
                    && !f64::from_bits(bits).is_finite()
                {
                    return Err(ContractError::NonFiniteMeasurement {
                        name: measurement.name.clone(),
                    });
                }
            }
        }

        require_bound(self.figures.len(), MAX_FIGURES, "figures")?;
        require_unique_ids(self.figures.iter().map(|f| &f.id), "figure")?;
        for figure in &self.figures {
            validate_required_text(&figure.title, "figure title", MAX_LABEL_CHARS)?;
            match &figure.content {
                FigureContent::SchematicSheet { sheet_index } => {
                    let sheet_count = self
                        .schematic
                        .as_ref()
                        .map_or(0, |section| section.sheets.len());
                    if usize::try_from(*sheet_index)
                        .ok()
                        .is_none_or(|index| index >= sheet_count)
                    {
                        return Err(ContractError::DanglingReference {
                            figure_id: figure.id,
                            kind: "sheet",
                            reference: u64::from(*sheet_index),
                        });
                    }
                }
                FigureContent::Plot(plot) => {
                    plot.scene.validate()?;
                    if let Some(hydration) = &plot.hydration {
                        let datasets = self
                            .results
                            .as_ref()
                            .map_or(&[][..], |results| &results.datasets[..]);
                        validate_hydration(hydration, datasets, figure.id)?;
                    }
                }
            }
        }

        if let Some(engineering) = &self.engineering {
            validate_engineering(self, engineering)?;
        }
        if let Some(presentation) = &self.presentation {
            validate_presentation(self, presentation)?;
        }

        Ok(())
    }
}

fn validate_sha256_hex(value: &str) -> Result<(), ContractError> {
    if value.len() != 64
        || !value
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        return Err(ContractError::MalformedDigest);
    }
    Ok(())
}

impl Validate for FigureManifest {
    fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != FIGURE_MANIFEST_SCHEMA_VERSION {
            return Err(ContractError::UnsupportedSchemaVersion {
                expected: FIGURE_MANIFEST_SCHEMA_VERSION,
                found: self.schema_version,
            });
        }
        require_bound(self.figures.len(), MAX_FIGURES, "manifest entries")?;
        require_unique_ids(self.figures.iter().map(|f| &f.figure_id), "manifest figure")?;
        for entry in &self.figures {
            validate_dom_id(&entry.dom_id, "figure dom id")?;
            validate_payload_path(&entry.payload.path)?;
            validate_sha256_hex(&entry.payload.sha256_hex)?;
            if entry.payload.byte_len == 0
                || usize::try_from(entry.payload.byte_len)
                    .ok()
                    .is_none_or(|len| len > MAX_FIGURE_PAYLOAD_BYTES)
            {
                return Err(ContractError::PayloadLengthOutOfRange);
            }
        }
        Ok(())
    }
}

impl Validate for FigurePayload {
    fn validate(&self) -> Result<(), ContractError> {
        if self.schema_version != FIGURE_MANIFEST_SCHEMA_VERSION {
            return Err(ContractError::UnsupportedSchemaVersion {
                expected: FIGURE_MANIFEST_SCHEMA_VERSION,
                found: self.schema_version,
            });
        }
        self.scene.validate()?;
        validate_datasets(&self.datasets, None)?;
        if let Some(hydration) = &self.hydration {
            validate_hydration(hydration, &self.datasets, self.figure_id)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Canonical bytes
// ---------------------------------------------------------------------------

fn canonical_bytes_of<T: Serialize + Validate>(
    value: &T,
    limit: usize,
) -> Result<Vec<u8>, ContractError> {
    value.validate()?;
    let bytes =
        serde_json::to_vec(value).map_err(|error| ContractError::Malformed(error.to_string()))?;
    if bytes.len() > limit {
        return Err(ContractError::PayloadTooLarge {
            limit,
            found: bytes.len(),
        });
    }
    Ok(bytes)
}

fn from_canonical_bytes_of<T: serde::de::DeserializeOwned + Validate>(
    bytes: &[u8],
    limit: usize,
) -> Result<T, ContractError> {
    if bytes.len() > limit {
        return Err(ContractError::PayloadTooLarge {
            limit,
            found: bytes.len(),
        });
    }
    let value: T = serde_json::from_slice(bytes)
        .map_err(|error| ContractError::Malformed(error.to_string()))?;
    value.validate()?;
    Ok(value)
}

impl PublicationSnapshot {
    /// Validate, then serialize to the canonical interchange encoding.
    /// Equal values always produce identical bytes.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes_of(self, MAX_PUBLICATION_SNAPSHOT_BYTES)
    }

    /// Enforce the size cap, parse strictly, then validate.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        from_canonical_bytes_of(bytes, MAX_PUBLICATION_SNAPSHOT_BYTES)
    }
}

impl FigureManifest {
    /// Validate, then serialize to the canonical interchange encoding.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes_of(self, MAX_FIGURE_PAYLOAD_BYTES)
    }

    /// Enforce the size cap, parse strictly, then validate.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        from_canonical_bytes_of(bytes, MAX_FIGURE_PAYLOAD_BYTES)
    }
}

impl FigurePayload {
    /// Validate, then serialize to the canonical interchange encoding.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes_of(self, MAX_FIGURE_PAYLOAD_BYTES)
    }

    /// Enforce the size cap, parse strictly, then validate.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        from_canonical_bytes_of(bytes, MAX_FIGURE_PAYLOAD_BYTES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(values: &[f64]) -> Vec<u64> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    fn minimal_scene() -> Scene {
        Scene {
            width_um: 279_400,
            height_um: 215_900,
            groups: vec![PrimitiveGroup {
                tag: Some(GroupTag::SheetFrame),
                primitives: vec![Primitive::Path(PathPrimitive {
                    segments: vec![
                        PathSegment::MoveTo {
                            to: Point { x_um: 0, y_um: 0 },
                        },
                        PathSegment::LineTo {
                            to: Point {
                                x_um: 279_400,
                                y_um: 0,
                            },
                        },
                        PathSegment::Close,
                    ],
                    stroke: Some(Stroke {
                        width_um: 254,
                        paint: Paint::Role(PaintRole::Foreground),
                        pattern: StrokePattern::Solid,
                    }),
                    fill: None,
                })],
            }],
        }
    }

    fn minimal_snapshot() -> PublicationSnapshot {
        PublicationSnapshot {
            schema_version: PUBLICATION_SNAPSHOT_SCHEMA_VERSION,
            metadata: PublicationMetadata {
                title: "RC low-pass".to_string(),
                description: String::new(),
                author_display: "Test Author".to_string(),
                app_version: "0.1.0".to_string(),
                created_utc: "2026-08-05T21:00:00Z".to_string(),
                license: ContentLicense::AllRightsReserved,
            },
            disclosure: Disclosure {
                schematic: true,
                netlist: true,
                results: true,
                archive: false,
            },
            schematic: Some(SchematicSection {
                sheets: vec![SheetScene {
                    name: "top".to_string(),
                    page_label: None,
                    scene: minimal_scene(),
                }],
            }),
            netlist: Some(NetlistSection {
                deck: "* RSpice Netlist\nR1 in out 1k\n.end".to_string(),
            }),
            results: Some(ResultsSection {
                analyses: vec![AnalysisRecord {
                    id: 1,
                    label: "Transient".to_string(),
                    card: ".tran 10u 5m".to_string(),
                }],
                datasets: vec![Dataset {
                    id: 1,
                    analysis_id: 1,
                    name: "tran1".to_string(),
                    variant: None,
                    sweep: SweepAxis {
                        label: "time".to_string(),
                        unit: "s".to_string(),
                        values_bits: bits(&[0.0, 0.5e-3, 1.0e-3]),
                    },
                    traces: vec![Trace {
                        label: "V(out)".to_string(),
                        unit: "V".to_string(),
                        values: TraceValues::Real {
                            bits: bits(&[0.0, 0.25, 0.5]),
                        },
                    }],
                }],
                measurements: vec![],
            }),
            figures: vec![Figure {
                id: 1,
                title: "Step response".to_string(),
                content: FigureContent::Plot(PlotFigure {
                    scene: minimal_scene(),
                    hydration: Some(PlotHydration {
                        x_scale: AxisScale::Linear,
                        y_scale: AxisScale::Linear,
                        x_label: "time".to_string(),
                        y_label: "V".to_string(),
                        bindings: vec![PlotTraceBinding {
                            dataset_id: 1,
                            trace_index: 0,
                            transform: TraceTransform::Identity,
                        }],
                    }),
                }),
            }],
            presentation: None,
            engineering: None,
        }
    }

    fn add_v3_metadata(snapshot: &mut PublicationSnapshot) {
        snapshot.presentation = Some(PublicationPresentation {
            overview: Some(PublicationOverview {
                narrative: "A published RC filter with explicit design intent.".to_string(),
                specifications: vec![Specification {
                    label: "Time constant".to_string(),
                    value: "1.00".to_string(),
                    unit: Some("ms".to_string()),
                }],
            }),
            section_order: vec![
                PublicationSection::Overview,
                PublicationSection::Results,
                PublicationSection::Components,
                PublicationSection::Files,
                PublicationSection::Details,
            ],
            default_section: PublicationSection::Results,
            featured_figure_id: Some(1),
            figure_details: vec![FigurePresentation {
                figure_id: 1,
                caption: Some("The output approaches its final value exponentially.".to_string()),
                accessible_summary:
                    "Transient voltage rises from zero toward the steady-state output.".to_string(),
                default_interactive: false,
            }],
        });
        snapshot.engineering = Some(EngineeringPublication {
            components: vec![ComponentRecord {
                reference: "R1".to_string(),
                value: "1k".to_string(),
                device: "Resistor".to_string(),
                model: None,
                pins: vec![
                    ComponentPin {
                        name: "1".to_string(),
                        number: Some("1".to_string()),
                        net: Some("in".to_string()),
                    },
                    ComponentPin {
                        name: "2".to_string(),
                        number: Some("2".to_string()),
                        net: Some("out".to_string()),
                    },
                ],
            }],
            nets: vec![
                NetRecord {
                    name: "in".to_string(),
                    connections: vec![NetConnection {
                        component_reference: "R1".to_string(),
                        pin_name: "1".to_string(),
                    }],
                },
                NetRecord {
                    name: "out".to_string(),
                    connections: vec![NetConnection {
                        component_reference: "R1".to_string(),
                        pin_name: "2".to_string(),
                    }],
                },
            ],
            signals: vec![SignalIdentity {
                dataset_id: 1,
                trace_index: 0,
                target: SignalTarget::NetVoltage {
                    net: "out".to_string(),
                },
            }],
            simulation: Some(SimulationProvenance {
                engine: "RSpice".to_string(),
                engine_version: "0.1.0".to_string(),
                temperature_c_bits: Some(27.0f64.to_bits()),
                corner: Some("typical".to_string()),
                settings: vec![SimulationSetting {
                    name: "Relative tolerance".to_string(),
                    value: "1e-3".to_string(),
                }],
                warnings: vec![SimulationWarning {
                    severity: WarningSeverity::Information,
                    message: "Operating point converged.".to_string(),
                    analysis_id: Some(1),
                }],
            }),
        });
    }

    #[test]
    fn minimal_snapshot_round_trips_canonically() {
        let snapshot = minimal_snapshot();
        let bytes = snapshot.canonical_bytes().expect("canonical bytes");
        let reparsed = PublicationSnapshot::from_canonical_bytes(&bytes).expect("reparse");
        assert_eq!(snapshot, reparsed);
        let second = reparsed.canonical_bytes().expect("second serialization");
        assert_eq!(bytes, second, "canonical bytes must be deterministic");
    }

    #[test]
    fn unsupported_future_schema_versions_are_rejected() {
        let mut snapshot = minimal_snapshot();
        snapshot.schema_version = PUBLICATION_SNAPSHOT_SCHEMA_VERSION + 1;
        assert_eq!(
            snapshot.validate(),
            Err(ContractError::UnsupportedSchemaVersion {
                expected: PUBLICATION_SNAPSHOT_SCHEMA_VERSION,
                found: PUBLICATION_SNAPSHOT_SCHEMA_VERSION + 1,
            })
        );
    }

    #[test]
    fn legacy_v2_snapshots_remain_byte_compatible_and_cannot_smuggle_v3_fields() {
        let mut snapshot = minimal_snapshot();
        snapshot.schema_version = PUBLICATION_SNAPSHOT_V2_SCHEMA_VERSION;
        snapshot.validate().expect("plain v2 remains supported");
        let bytes = snapshot.canonical_bytes().expect("v2 canonical bytes");
        let text = std::str::from_utf8(&bytes).expect("snapshot JSON");
        assert!(!text.contains("presentation"));
        assert!(!text.contains("engineering"));

        add_v3_metadata(&mut snapshot);
        assert_eq!(
            snapshot.validate(),
            Err(ContractError::V3FieldInLegacySnapshot {
                field: "presentation"
            })
        );
    }

    #[test]
    fn v3_metadata_round_trips_with_explicit_engineering_identity() {
        let mut snapshot = minimal_snapshot();
        add_v3_metadata(&mut snapshot);
        snapshot.validate().expect("complete v3 metadata");
        let bytes = snapshot.canonical_bytes().expect("canonical v3 bytes");
        let reparsed = PublicationSnapshot::from_canonical_bytes(&bytes).expect("parse v3");
        assert_eq!(reparsed, snapshot);
        assert!(
            bytes
                .windows(b"signal".len())
                .any(|window| window == b"signal")
        );
    }

    #[test]
    fn v3_named_and_numeric_references_fail_closed() {
        let mut snapshot = minimal_snapshot();
        add_v3_metadata(&mut snapshot);
        snapshot.engineering.as_mut().expect("engineering").signals[0].target =
            SignalTarget::NetVoltage {
                net: "missing".to_string(),
            };
        assert!(matches!(
            snapshot.validate(),
            Err(ContractError::DanglingNamedReference {
                target_kind: "net",
                ..
            })
        ));

        let mut snapshot = minimal_snapshot();
        add_v3_metadata(&mut snapshot);
        snapshot
            .engineering
            .as_mut()
            .expect("engineering")
            .simulation
            .as_mut()
            .expect("simulation")
            .temperature_c_bits = Some(f64::NAN.to_bits());
        assert_eq!(
            snapshot.validate(),
            Err(ContractError::NonFiniteV3Value {
                field: "simulation temperature"
            })
        );
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let mut value = serde_json::to_value(minimal_snapshot()).expect("to value");
        value
            .as_object_mut()
            .expect("object")
            .insert("surprise".to_string(), serde_json::Value::Bool(true));
        let bytes = serde_json::to_vec(&value).expect("bytes");
        assert!(matches!(
            PublicationSnapshot::from_canonical_bytes(&bytes),
            Err(ContractError::Malformed(_))
        ));
    }

    #[test]
    fn disclosure_must_match_content_in_both_directions() {
        let mut withheld_but_present = minimal_snapshot();
        withheld_but_present.disclosure.netlist = false;
        assert_eq!(
            withheld_but_present.validate(),
            Err(ContractError::UndisclosedSectionPresent { section: "netlist" })
        );

        let mut disclosed_but_absent = minimal_snapshot();
        disclosed_but_absent.schematic = None;
        assert_eq!(
            disclosed_but_absent.validate(),
            Err(ContractError::DisclosedSectionMissing {
                section: "schematic"
            })
        );
    }

    #[test]
    fn dangling_references_are_rejected() {
        let mut bad_sheet = minimal_snapshot();
        bad_sheet.figures.push(Figure {
            id: 2,
            title: "Ghost sheet".to_string(),
            content: FigureContent::SchematicSheet { sheet_index: 5 },
        });
        assert_eq!(
            bad_sheet.validate(),
            Err(ContractError::DanglingReference {
                figure_id: 2,
                kind: "sheet",
                reference: 5,
            })
        );

        let mut bad_binding = minimal_snapshot();
        if let FigureContent::Plot(plot) = &mut bad_binding.figures[0].content {
            plot.hydration.as_mut().expect("hydration").bindings[0].dataset_id = 99;
        }
        assert_eq!(
            bad_binding.validate(),
            Err(ContractError::DanglingReference {
                figure_id: 1,
                kind: "dataset",
                reference: 99,
            })
        );

        let mut bad_trace = minimal_snapshot();
        if let FigureContent::Plot(plot) = &mut bad_trace.figures[0].content {
            plot.hydration.as_mut().expect("hydration").bindings[0].trace_index = 7;
        }
        assert_eq!(
            bad_trace.validate(),
            Err(ContractError::DanglingReference {
                figure_id: 1,
                kind: "trace",
                reference: 7,
            })
        );
    }

    #[test]
    fn trace_lengths_must_match_the_sweep() {
        let mut snapshot = minimal_snapshot();
        let results = snapshot.results.as_mut().expect("results");
        results.datasets[0].traces[0].values = TraceValues::Real {
            bits: bits(&[0.0, 0.25]),
        };
        assert_eq!(
            snapshot.validate(),
            Err(ContractError::TraceLengthMismatch {
                dataset_id: 1,
                trace_index: 0,
                trace_len: 2,
                sweep_len: 3,
            })
        );

        let mut complex = minimal_snapshot();
        let results = complex.results.as_mut().expect("results");
        results.datasets[0].traces[0].values = TraceValues::Complex {
            real_bits: bits(&[0.0, 0.25, 0.5]),
            imaginary_bits: bits(&[0.0, 0.25]),
        };
        assert!(matches!(
            complex.validate(),
            Err(ContractError::TraceLengthMismatch { .. })
        ));
    }

    #[test]
    fn non_finite_samples_are_rejected() {
        let mut snapshot = minimal_snapshot();
        let results = snapshot.results.as_mut().expect("results");
        results.datasets[0].traces[0].values = TraceValues::Real {
            bits: vec![0, f64::NAN.to_bits(), 0],
        };
        assert_eq!(
            snapshot.validate(),
            Err(ContractError::NonFiniteSample { dataset_id: 1 })
        );

        let mut measurement = minimal_snapshot();
        let results = measurement.results.as_mut().expect("results");
        results.measurements.push(Measurement {
            analysis_id: 1,
            name: "gain".to_string(),
            value_bits: Some(f64::INFINITY.to_bits()),
            display: "∞".to_string(),
            spec_display: None,
            passed: None,
        });
        assert!(matches!(
            measurement.validate(),
            Err(ContractError::NonFiniteMeasurement { .. })
        ));
    }

    #[test]
    fn duplicate_identifiers_are_rejected() {
        let mut snapshot = minimal_snapshot();
        let results = snapshot.results.as_mut().expect("results");
        let clone = results.datasets[0].clone();
        results.datasets.push(clone);
        assert_eq!(
            snapshot.validate(),
            Err(ContractError::DuplicateId {
                kind: "dataset",
                id: 1
            })
        );
    }

    #[test]
    fn control_characters_are_rejected_in_text() {
        let mut snapshot = minimal_snapshot();
        snapshot.metadata.title = "bad\u{0007}title".to_string();
        assert_eq!(
            snapshot.validate(),
            Err(ContractError::ControlCharacter { field: "title" })
        );

        let mut deck = minimal_snapshot();
        deck.netlist = Some(NetlistSection {
            deck: "* RSpice Netlist\r\n.end".to_string(),
        });
        assert_eq!(
            deck.validate(),
            Err(ContractError::ControlCharacter { field: "deck" }),
            "carriage returns are not canonical deck bytes"
        );
    }

    #[test]
    fn malformed_scenes_are_rejected() {
        let mut no_move = minimal_snapshot();
        let sheets = &mut no_move.schematic.as_mut().expect("schematic").sheets;
        sheets[0].scene.groups[0].primitives = vec![Primitive::Path(PathPrimitive {
            segments: vec![PathSegment::LineTo {
                to: Point { x_um: 1, y_um: 1 },
            }],
            stroke: Some(Stroke {
                width_um: 254,
                paint: Paint::Role(PaintRole::Foreground),
                pattern: StrokePattern::Solid,
            }),
            fill: None,
        })];
        assert_eq!(no_move.validate(), Err(ContractError::MalformedPath));

        let mut unpainted = minimal_snapshot();
        let sheets = &mut unpainted.schematic.as_mut().expect("schematic").sheets;
        sheets[0].scene.groups[0].primitives = vec![Primitive::Path(PathPrimitive {
            segments: vec![PathSegment::MoveTo {
                to: Point { x_um: 0, y_um: 0 },
            }],
            stroke: None,
            fill: None,
        })];
        assert_eq!(unpainted.validate(), Err(ContractError::UnpaintedPath));

        let mut flat = minimal_snapshot();
        flat.schematic.as_mut().expect("schematic").sheets[0]
            .scene
            .height_um = 0;
        assert_eq!(flat.validate(), Err(ContractError::EmptyScene));
    }

    #[test]
    fn manifest_identifiers_are_strict() {
        let manifest = |dom_id: &str, path: &str, digest: &str| FigureManifest {
            schema_version: FIGURE_MANIFEST_SCHEMA_VERSION,
            figures: vec![ManifestEntry {
                figure_id: 1,
                dom_id: dom_id.to_string(),
                kind: ManifestFigureKind::Plot,
                payload: PayloadRef {
                    path: path.to_string(),
                    sha256_hex: digest.to_string(),
                    byte_len: 128,
                },
            }],
        };
        let good_digest = "a".repeat(64);

        assert!(
            manifest("fig-1", "figures/1.json", &good_digest)
                .validate()
                .is_ok()
        );
        assert_eq!(
            manifest("1fig", "figures/1.json", &good_digest).validate(),
            Err(ContractError::MalformedIdentifier {
                field: "figure dom id"
            })
        );
        assert_eq!(
            manifest("fig one", "figures/1.json", &good_digest).validate(),
            Err(ContractError::MalformedIdentifier {
                field: "figure dom id"
            })
        );
        for bad_path in [
            "/figures/1.json",
            "figures/../1.json",
            "figures\\1.json",
            "figures/.hidden",
            "figures//1.json",
            "figures/1.json/",
        ] {
            assert!(
                manifest("fig-1", bad_path, &good_digest)
                    .validate()
                    .is_err(),
                "path {bad_path:?} must be rejected"
            );
        }
        assert_eq!(
            manifest("fig-1", "figures/1.json", &"A".repeat(64)).validate(),
            Err(ContractError::MalformedDigest)
        );
        assert_eq!(
            manifest("fig-1", "figures/1.json", "abc123").validate(),
            Err(ContractError::MalformedDigest)
        );
    }

    #[test]
    fn payload_length_bounds_are_enforced() {
        let mut manifest = FigureManifest {
            schema_version: FIGURE_MANIFEST_SCHEMA_VERSION,
            figures: vec![ManifestEntry {
                figure_id: 1,
                dom_id: "fig-1".to_string(),
                kind: ManifestFigureKind::SchematicSheet,
                payload: PayloadRef {
                    path: "figures/1.json".to_string(),
                    sha256_hex: "b".repeat(64),
                    byte_len: 0,
                },
            }],
        };
        assert_eq!(
            manifest.validate(),
            Err(ContractError::PayloadLengthOutOfRange)
        );
        manifest.figures[0].payload.byte_len = MAX_FIGURE_PAYLOAD_BYTES as u64 + 1;
        assert_eq!(
            manifest.validate(),
            Err(ContractError::PayloadLengthOutOfRange)
        );
    }

    #[test]
    fn oversized_canonical_payloads_are_rejected_before_parsing() {
        let oversized = vec![b' '; MAX_FIGURE_PAYLOAD_BYTES + 1];
        assert_eq!(
            FigureManifest::from_canonical_bytes(&oversized),
            Err(ContractError::PayloadTooLarge {
                limit: MAX_FIGURE_PAYLOAD_BYTES,
                found: MAX_FIGURE_PAYLOAD_BYTES + 1,
            })
        );
    }

    #[test]
    fn figure_payload_validates_bindings_against_its_own_datasets() {
        let snapshot = minimal_snapshot();
        let results = snapshot.results.as_ref().expect("results");
        let payload = FigurePayload {
            schema_version: FIGURE_MANIFEST_SCHEMA_VERSION,
            figure_id: 1,
            scene: minimal_scene(),
            datasets: results.datasets.clone(),
            hydration: Some(PlotHydration {
                x_scale: AxisScale::Linear,
                y_scale: AxisScale::Linear,
                x_label: "time".to_string(),
                y_label: "V".to_string(),
                bindings: vec![PlotTraceBinding {
                    dataset_id: 1,
                    trace_index: 0,
                    transform: TraceTransform::Identity,
                }],
            }),
        };
        assert!(payload.validate().is_ok());
        let bytes = payload.canonical_bytes().expect("payload bytes");
        assert_eq!(
            FigurePayload::from_canonical_bytes(&bytes).expect("reparse"),
            payload
        );
    }

    #[test]
    fn netlist_title_is_the_first_deck_line() {
        let netlist = NetlistSection {
            deck: "* RSpice Netlist\nR1 a b 1k\n.end".to_string(),
        };
        assert_eq!(netlist.title(), "* RSpice Netlist");
    }
}
