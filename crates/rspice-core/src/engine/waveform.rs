//! Compressed transient result container.
//!
//! A compressed transient is a complete result container, not a reduced copy
//! of one: it keeps the retained time grid, the exact accepted step size at
//! every retained point, every analog channel under its own descriptor, the
//! event-driven digital and real traces exactly as recorded, the typed
//! post-process products, the producing run's identity, and the versioned
//! evidence describing how the decimation was performed and how far it moved
//! any discarded sample.
//!
//! # Decimation
//!
//! Analog channels share one retained grid, chosen by multi-channel
//! Ramer-Douglas-Peucker decimation and read back by piecewise-linear
//! interpolation. A sample is discarded only when every channel's linear
//! reconstruction at that time stays inside
//! `absolute_tolerance + relative_tolerance * |actual|`.
//!
//! Event traces are never decimated. Interpolation is undefined for an event
//! channel, so digital and real traces are carried verbatim as opaque typed
//! channels.
//!
//! # Validity and non-finite policy
//!
//! A channel sample is either a number or explicitly absent with a reason.
//! Compression never publishes a fabricated number: a non-finite solver value
//! becomes [`TransientSampleAbsence::NonFinite`] and an unrecorded value (for
//! example a device operating-point parameter that did not exist yet at that
//! sample) becomes [`TransientSampleAbsence::NotRecorded`]. Absent samples do
//! not participate in the decimation error budget or in its certificate: a
//! retained segment whose endpoints are absent cannot be reconstructed, so
//! error is only measured where both endpoints and the discarded sample are
//! present.
//!
//! # Layering note
//!
//! The typed execution vocabulary (`SignalDescriptor`, `AnalysisInstanceId`,
//! `RunCoordinateId`, `TopologyFingerprint`) lives in `crate::execution`,
//! which sits *above* the engine in this crate's layer order. The engine
//! therefore names its own descriptor and identity vocabulary here, and
//! `crate::execution` maps it into the shared result document. The mapping is
//! total and tested; it is not a second decision about what a volt is.

use crate::Value;
use crate::engine::result::{DigitalTrace, RealTrace, TransientPostResults};

//=============================================================================
// Configuration
//=============================================================================

/// Configuration for waveform compression
#[derive(Debug, Clone, PartialEq)]
pub struct CompressionConfig {
    /// Absolute tolerance in each channel's native units.
    /// Points within this absolute error of the interpolated value are skipped.
    pub abs_tol: Value,

    /// Relative tolerance for storing points (fraction)
    /// Points within this relative error are skipped
    pub rel_tol: Value,

    /// Whether compression is enabled
    /// When disabled, all points are stored (useful for debugging)
    pub enabled: bool,

    /// Maximum time between retained points. Set to `0.0` to impose no
    /// time-axis gap limit.
    pub maximum_retained_interval: Value,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            abs_tol: 1e-6, // 1 microvolt
            rel_tol: 1e-3, // 0.1%
            enabled: true,
            maximum_retained_interval: 0.0,
        }
    }
}

impl CompressionConfig {
    /// No compression (store all points)
    pub fn none() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    /// Aggressive compression (good for long simulations)
    pub fn aggressive() -> Self {
        Self {
            abs_tol: 1e-5,
            rel_tol: 1e-2, // 1%
            enabled: true,
            maximum_retained_interval: 0.0,
        }
    }
}

/// Schema version for the persisted transient-compression certificate.
pub const TRANSIENT_COMPRESSION_REPORT_VERSION: u32 = 1;

/// Algorithm that produced a compressed transient waveform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransientCompressionAlgorithm {
    /// Shared-grid, multi-channel Ramer-Douglas-Peucker decimation with
    /// piecewise-linear reconstruction.
    MultiChannelRdpLinearV1,
}

impl TransientCompressionAlgorithm {
    /// Stable wire spelling for adapters and persistence layers.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MultiChannelRdpLinearV1 => "multi-channel-rdp-linear-v1",
        }
    }
}

/// Sample domain over which the declared compression error was measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransientCompressionSampleDomain {
    /// Every discarded sample from the original accepted solver grid.
    AcceptedInputSamples,
}

impl TransientCompressionSampleDomain {
    /// Stable wire spelling for adapters and persistence layers.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcceptedInputSamples => "accepted-input-samples",
        }
    }
}

/// Exact compression policy applied to one published result.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientCompressionPolicy {
    /// Whether decimation was enabled.
    pub enabled: bool,
    /// Absolute interpolation tolerance in each signal's native unit.
    pub absolute_tolerance: Value,
    /// Relative interpolation tolerance as a fraction of the actual sample.
    pub relative_tolerance: Value,
    /// Maximum permitted gap between retained time points. Zero disables it.
    pub maximum_retained_interval: Value,
}

impl From<&CompressionConfig> for TransientCompressionPolicy {
    fn from(config: &CompressionConfig) -> Self {
        Self {
            enabled: config.enabled,
            absolute_tolerance: config.abs_tol,
            relative_tolerance: config.rel_tol,
            maximum_retained_interval: config.maximum_retained_interval,
        }
    }
}

/// Stable identity class for a compressed analog signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransientCompressionSignalKind {
    Voltage,
    BranchCurrent,
    DeviceObservable,
    DeviceStore,
}

impl TransientCompressionSignalKind {
    /// Stable wire spelling for adapters and persistence layers.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Voltage => "voltage",
            Self::BranchCurrent => "branch-current",
            Self::DeviceObservable => "device-observable",
            Self::DeviceStore => "device-store",
        }
    }

    /// Parse a wire spelling produced by [`Self::as_str`].
    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "voltage" => Some(Self::Voltage),
            "branch-current" => Some(Self::BranchCurrent),
            "device-observable" => Some(Self::DeviceObservable),
            "device-store" => Some(Self::DeviceStore),
            _ => None,
        }
    }
}

/// Stable signal identity attached to a compression-error observation.
///
/// `canonical_name` is independent of any positional result array, so it is a
/// direct reference into the descriptor-indexed channel list of
/// [`TransientResultCompressed`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransientCompressionSignal {
    pub kind: TransientCompressionSignalKind,
    pub canonical_name: String,
}

impl TransientCompressionSignal {
    /// Construct a canonical signal identity.
    pub fn new(
        kind: TransientCompressionSignalKind,
        canonical_name: impl Into<String>,
    ) -> Result<Self, String> {
        let canonical_name = canonical_name.into();
        let canonical_name = canonical_name.trim();
        if canonical_name.is_empty() {
            return Err("compression signal canonical name cannot be empty".to_string());
        }
        Ok(Self {
            kind,
            canonical_name: canonical_name.to_ascii_lowercase(),
        })
    }

    pub(crate) fn voltage(name: &str) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("compression voltage identity requires a node name".to_string());
        }
        Self::new(
            TransientCompressionSignalKind::Voltage,
            format!("v({})", name.trim()),
        )
    }

    pub(crate) fn branch_current(name: &str) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("compression branch-current identity requires a branch name".to_string());
        }
        Self::new(
            TransientCompressionSignalKind::BranchCurrent,
            format!("i({})", name.trim()),
        )
    }

    pub(crate) fn device_observable(device: &str, parameter: &str) -> Result<Self, String> {
        if device.trim().is_empty() || parameter.trim().is_empty() {
            return Err(
                "compression device-observable identity requires device and parameter names"
                    .to_string(),
            );
        }
        Self::new(
            TransientCompressionSignalKind::DeviceObservable,
            format!("@{}[{}]", device.trim(), parameter.trim()),
        )
    }

    pub(crate) fn device_store(name: &str) -> Result<Self, String> {
        Self::new(TransientCompressionSignalKind::DeviceStore, name)
    }
}

/// Worst final-grid reconstruction error observed at one discarded sample.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientCompressionErrorObservation {
    /// Stable identity of the signal that consumed the largest fraction of
    /// its declared tolerance.
    pub signal: TransientCompressionSignal,
    /// Index of the sample in the original accepted input grid.
    pub input_sample_index: usize,
    /// Original sample time in seconds.
    pub time: Value,
    /// Original signal value at the measured sample. This makes the reported
    /// relative error and absolute-plus-relative allowance independently
    /// checkable after the full accepted grid has been released.
    pub actual_value: Value,
    /// Absolute reconstruction error in the signal's native unit.
    pub absolute_error: Value,
    /// Absolute error divided by `|actual|`; absent at zero or when the ratio
    /// cannot be represented as a finite `Value`.
    pub relative_error: Option<Value>,
    /// Applied `absolute + relative * |actual|` tolerance in the signal's
    /// native unit.
    pub allowed_tolerance: Value,
    /// Unitless `absolute_error / allowed_tolerance`. A zero tolerance with
    /// zero error has utilization zero.
    pub tolerance_utilization: Value,
}

/// Versioned evidence describing how a compressed transient was produced and
/// the worst error of its final published retained grid.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientCompressionReport {
    pub schema_version: u32,
    pub algorithm: TransientCompressionAlgorithm,
    pub sample_domain: TransientCompressionSampleDomain,
    pub applied_policy: TransientCompressionPolicy,
    pub input_points: usize,
    pub retained_points: usize,
    /// `None` means no input sample was approximated (or there were no
    /// compressible analog samples), never that error measurement was skipped.
    pub worst_observed: Option<TransientCompressionErrorObservation>,
}

impl TransientCompressionReport {
    pub(crate) fn new(
        config: &CompressionConfig,
        input_points: usize,
        retained_points: usize,
        worst_observed: Option<TransientCompressionErrorObservation>,
    ) -> Self {
        Self {
            schema_version: TRANSIENT_COMPRESSION_REPORT_VERSION,
            algorithm: TransientCompressionAlgorithm::MultiChannelRdpLinearV1,
            sample_domain: TransientCompressionSampleDomain::AcceptedInputSamples,
            applied_policy: config.into(),
            input_points,
            retained_points,
            worst_observed,
        }
    }
}

//=============================================================================
// Channel descriptors
//=============================================================================

/// Physical unit of one compressed transient channel.
///
/// This mirrors the shared execution-layer unit vocabulary; see the layering
/// note in the module documentation. [`Self::Unspecified`] is the honest
/// answer for a device observable or store whose producing model does not
/// declare a unit, and is deliberately not the same claim as
/// [`Self::Dimensionless`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransientChannelUnit {
    Volt,
    Ampere,
    Ohm,
    Siemens,
    Watt,
    Hertz,
    Second,
    Degree,
    Radian,
    Dimensionless,
    Unspecified,
    Custom(String),
}

impl TransientChannelUnit {
    /// Stable wire spelling for adapters and persistence layers.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Volt => "volt",
            Self::Ampere => "ampere",
            Self::Ohm => "ohm",
            Self::Siemens => "siemens",
            Self::Watt => "watt",
            Self::Hertz => "hertz",
            Self::Second => "second",
            Self::Degree => "degree",
            Self::Radian => "radian",
            Self::Dimensionless => "dimensionless",
            Self::Unspecified => "unspecified",
            Self::Custom(symbol) => symbol,
        }
    }

    /// Parse a wire spelling produced by [`Self::as_str`].
    ///
    /// An unknown spelling is a custom unit symbol rather than an error: the
    /// producing model owns its own symbols.
    pub fn from_tag(tag: &str) -> Result<Self, String> {
        Ok(match tag {
            "volt" => Self::Volt,
            "ampere" => Self::Ampere,
            "ohm" => Self::Ohm,
            "siemens" => Self::Siemens,
            "watt" => Self::Watt,
            "hertz" => Self::Hertz,
            "second" => Self::Second,
            "degree" => Self::Degree,
            "radian" => Self::Radian,
            "dimensionless" => Self::Dimensionless,
            "unspecified" => Self::Unspecified,
            symbol if symbol.trim().is_empty() => {
                return Err("compressed transient channel unit symbol cannot be empty".to_string());
            }
            symbol => Self::Custom(symbol.to_string()),
        })
    }
}

/// What a compressed transient channel belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransientChannelOwner {
    /// A circuit node, named as the netlist spells it.
    Node(String),
    /// A branch whose current the solver carries as an unknown.
    Branch(String),
    /// A device instance.
    Device(String),
}

/// Which result family one compressed channel came from.
///
/// The role carries the identifying names, so a channel never depends on its
/// position in a parallel name vector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransientChannelRole {
    /// Solution voltage of the node with this zero-based index; the solver
    /// node id is `node_index + 1` because index 0 is ground.
    NodeVoltage { node_index: usize, node: String },
    /// Current through a named branch, positive flowing into the branch's
    /// first terminal, which is the engine's sign convention everywhere.
    BranchCurrent { branch: String },
    /// Device operating-point observable `@device[parameter]`.
    DeviceObservable { device: String, parameter: String },
    /// Typed non-solution device store waveform, such as a compact-model
    /// internal resistance.
    DeviceStore { store: String },
}

impl TransientChannelRole {
    /// Identity class of a channel with this role.
    pub const fn kind(&self) -> TransientCompressionSignalKind {
        match self {
            Self::NodeVoltage { .. } => TransientCompressionSignalKind::Voltage,
            Self::BranchCurrent { .. } => TransientCompressionSignalKind::BranchCurrent,
            Self::DeviceObservable { .. } => TransientCompressionSignalKind::DeviceObservable,
            Self::DeviceStore { .. } => TransientCompressionSignalKind::DeviceStore,
        }
    }

    fn display_name(&self) -> String {
        match self {
            Self::NodeVoltage { node, .. } => format!("V({node})"),
            Self::BranchCurrent { branch } => format!("I({branch})"),
            Self::DeviceObservable { device, parameter } => format!("@{device}[{parameter}]"),
            Self::DeviceStore { store } => store.clone(),
        }
    }

    fn signal(&self) -> Result<TransientCompressionSignal, String> {
        match self {
            Self::NodeVoltage { node, .. } => TransientCompressionSignal::voltage(node),
            Self::BranchCurrent { branch } => TransientCompressionSignal::branch_current(branch),
            Self::DeviceObservable { device, parameter } => {
                TransientCompressionSignal::device_observable(device, parameter)
            }
            Self::DeviceStore { store } => TransientCompressionSignal::device_store(store),
        }
    }

    fn owner(&self) -> TransientChannelOwner {
        match self {
            Self::NodeVoltage { node, .. } => TransientChannelOwner::Node(node.clone()),
            Self::BranchCurrent { branch } => TransientChannelOwner::Branch(branch.clone()),
            Self::DeviceObservable { device, .. } => TransientChannelOwner::Device(device.clone()),
            Self::DeviceStore { store } => TransientChannelOwner::Device(store.clone()),
        }
    }

    fn default_unit(&self) -> TransientChannelUnit {
        match self {
            Self::NodeVoltage { .. } => TransientChannelUnit::Volt,
            Self::BranchCurrent { .. } => TransientChannelUnit::Ampere,
            // No compact model declares a unit for its operating-point or
            // store outputs today. Claiming "dimensionless" would be a
            // fabricated physical claim, so the descriptor says so.
            Self::DeviceObservable { .. } | Self::DeviceStore { .. } => {
                TransientChannelUnit::Unspecified
            }
        }
    }
}

/// Descriptor that keys one compressed transient channel.
///
/// Every compressed transient channel is a real scalar sampled on the shared
/// retained grid, so [`Self::value_type`] and [`Self::shape`] are declared
/// invariants rather than free fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransientChannelDescriptor {
    role: TransientChannelRole,
    canonical_name: String,
    display_name: String,
    unit: TransientChannelUnit,
}

impl TransientChannelDescriptor {
    /// Build a descriptor for one role with its model-declared unit.
    pub fn new(role: TransientChannelRole, unit: TransientChannelUnit) -> Result<Self, String> {
        if let TransientChannelUnit::Custom(symbol) = &unit
            && symbol.trim().is_empty()
        {
            return Err("compressed transient channel unit symbol cannot be empty".to_string());
        }
        let signal = role.signal()?;
        let display_name = role.display_name();
        if display_name.trim().is_empty() {
            return Err("compressed transient channel display name cannot be empty".to_string());
        }
        Ok(Self {
            canonical_name: signal.canonical_name,
            display_name,
            unit,
            role,
        })
    }

    /// Build a descriptor with the unit implied by its role.
    pub fn for_role(role: TransientChannelRole) -> Result<Self, String> {
        let unit = role.default_unit();
        Self::new(role, unit)
    }

    /// Which result family this channel came from.
    pub const fn role(&self) -> &TransientChannelRole {
        &self.role
    }

    /// Identity class of this channel.
    pub const fn kind(&self) -> TransientCompressionSignalKind {
        self.role.kind()
    }

    /// Lower-case canonical name, unique within one compressed result.
    pub fn canonical_name(&self) -> &str {
        &self.canonical_name
    }

    /// Display spelling, preserving the authored case.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Physical unit of this channel's samples.
    pub const fn unit(&self) -> &TransientChannelUnit {
        &self.unit
    }

    /// What this channel belongs to.
    pub fn owner(&self) -> TransientChannelOwner {
        self.role.owner()
    }

    /// Value type of every sample. Compressed transient channels are real.
    pub const fn value_type(&self) -> &'static str {
        "real"
    }

    /// Shape of every sample. Compressed transient channels are scalars.
    pub const fn shape(&self) -> &'static str {
        "scalar"
    }

    /// Stable identity of this channel, as the compression certificate spells
    /// it.
    pub fn signal(&self) -> TransientCompressionSignal {
        TransientCompressionSignal {
            kind: self.role.kind(),
            canonical_name: self.canonical_name.clone(),
        }
    }
}

//=============================================================================
// Samples and validity
//=============================================================================

/// Why one compressed channel sample carries no number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransientSampleAbsence {
    /// The producing run computed a non-finite value here. A run started with
    /// a non-finite tolerance policy may publish these; the number itself is
    /// deliberately not republished as if it were a measurement.
    NonFinite,
    /// The producing run recorded no value for this channel at this accepted
    /// sample, for example a device operating-point parameter that the device
    /// did not report yet.
    NotRecorded,
}

impl TransientSampleAbsence {
    /// Stable wire spelling for adapters and persistence layers.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonFinite => "non-finite",
            Self::NotRecorded => "not-recorded",
        }
    }

    /// Parse a wire spelling produced by [`Self::as_str`].
    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "non-finite" => Some(Self::NonFinite),
            "not-recorded" => Some(Self::NotRecorded),
            _ => None,
        }
    }
}

/// One compressed channel sample: a number, or an explicit absence.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransientChannelSample {
    /// A finite retained value.
    Value(Value),
    /// No value exists here, and this is why.
    Absent(TransientSampleAbsence),
}

impl TransientChannelSample {
    /// The number, when this sample has one.
    pub const fn value(self) -> Option<Value> {
        match self {
            Self::Value(value) => Some(value),
            Self::Absent(_) => None,
        }
    }

    /// Why this sample has no number, when it has none.
    pub const fn absence(self) -> Option<TransientSampleAbsence> {
        match self {
            Self::Value(_) => None,
            Self::Absent(reason) => Some(reason),
        }
    }

    /// Whether this sample carries a number.
    pub const fn is_present(self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// Classify one raw solver value, never publishing a non-finite number.
    pub fn from_solver_value(value: Value) -> Self {
        if value.is_finite() {
            Self::Value(value)
        } else {
            Self::Absent(TransientSampleAbsence::NonFinite)
        }
    }
}

/// Whether one channel was retained at all, and if not, why.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransientChannelAvailability {
    /// The channel was retained; it has one sample per retained time point.
    Available,
    /// The authored output projection deliberately did not retain this
    /// channel. Its descriptor, unit, and owner remain evidence that it
    /// exists, and it carries no samples.
    NotProjected,
}

impl TransientChannelAvailability {
    /// Stable wire spelling for adapters and persistence layers.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::NotProjected => "not-projected",
        }
    }

    /// Parse a wire spelling produced by [`Self::as_str`].
    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "available" => Some(Self::Available),
            "not-projected" => Some(Self::NotProjected),
            _ => None,
        }
    }
}

/// One descriptor-keyed analog channel on the retained grid.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientCompressedChannel {
    /// Name, kind, unit, owner, value type, and shape of this channel.
    pub descriptor: TransientChannelDescriptor,
    /// Whether this channel was retained.
    pub availability: TransientChannelAvailability,
    /// One sample per retained time point, or empty when the channel was not
    /// projected.
    pub samples: Vec<TransientChannelSample>,
}

impl TransientCompressedChannel {
    /// Present values at every retained point, or `None` when any sample is
    /// absent or the channel was not projected.
    pub fn dense_values(&self) -> Option<Vec<Value>> {
        if self.availability != TransientChannelAvailability::Available {
            return None;
        }
        self.samples
            .iter()
            .map(|sample| sample.value())
            .collect::<Option<Vec<_>>>()
    }

    /// Per-sample validity mask aligned with the retained grid.
    pub fn validity(&self) -> Vec<bool> {
        self.samples
            .iter()
            .map(|sample| sample.is_present())
            .collect()
    }
}

//=============================================================================
// Parent identity
//=============================================================================

/// Identity of the authored analysis card a compressed result came from.
///
/// The parts are stored rather than the execution-layer `AnalysisInstanceId`
/// itself; see the layering note in the module documentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransientAnalysisIdentity {
    /// Stable analysis-kind tag, for example `tran`.
    pub kind_tag: String,
    /// Zero-based ordinal among the authored cards of that kind.
    pub ordinal: u32,
}

impl TransientAnalysisIdentity {
    /// Build an identity, rejecting an empty kind tag.
    pub fn new(kind_tag: impl Into<String>, ordinal: u32) -> Result<Self, String> {
        let kind_tag = kind_tag.into();
        if kind_tag.trim().is_empty() {
            return Err("compressed transient analysis kind tag cannot be empty".to_string());
        }
        Ok(Self {
            kind_tag: kind_tag.trim().to_ascii_lowercase(),
            ordinal,
        })
    }

    /// Stable `<kind>-<ordinal+1, 3 digits>` tag.
    pub fn tag(&self) -> String {
        format!("{}-{:03}", self.kind_tag, u64::from(self.ordinal) + 1)
    }
}

/// Identity of the shared-deck coordinate a compressed result was produced at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransientCoordinateIdentity {
    /// Semantic digest of the coordinate's axis assignments.
    pub semantic: [u8; 16],
    /// Zero-based occurrence of that digest in the planned order.
    pub occurrence: u32,
    /// Zero-based position in the Cartesian coordinate order.
    pub ordinal: usize,
    /// Display label for this coordinate.
    pub label: String,
}

impl TransientCoordinateIdentity {
    /// Build a coordinate identity, rejecting an empty label.
    pub fn new(
        semantic: [u8; 16],
        occurrence: u32,
        ordinal: usize,
        label: impl Into<String>,
    ) -> Result<Self, String> {
        let label = label.into();
        if label.trim().is_empty() {
            return Err("compressed transient coordinate label cannot be empty".to_string());
        }
        Ok(Self {
            semantic,
            occurrence,
            ordinal,
            label: label.trim().to_string(),
        })
    }
}

/// Optional parent identity that makes a compressed result self-describing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TransientResultIdentity {
    /// Authored analysis card this result came from.
    pub analysis: Option<TransientAnalysisIdentity>,
    /// Shared-deck coordinate this result was produced at.
    pub coordinate: Option<TransientCoordinateIdentity>,
    /// Structural identity of the elaborated topology that was solved.
    pub topology_fingerprint: Option<[u8; 32]>,
}

impl TransientResultIdentity {
    /// Whether no identity component is recorded.
    pub fn is_empty(&self) -> bool {
        self.analysis.is_none() && self.coordinate.is_none() && self.topology_fingerprint.is_none()
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(analysis) = &self.analysis
            && analysis.kind_tag.trim().is_empty()
        {
            return Err("compressed transient analysis kind tag cannot be empty".to_string());
        }
        if let Some(coordinate) = &self.coordinate
            && coordinate.label.trim().is_empty()
        {
            return Err("compressed transient coordinate label cannot be empty".to_string());
        }
        Ok(())
    }
}

//=============================================================================
// Result container
//=============================================================================

/// Compressed transient result: a complete, self-describing result container.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientResultCompressed {
    /// Retained time points, a subset of the accepted solver grid with their
    /// exact IEEE-754 values.
    pub time: Vec<Value>,

    /// Exact accepted integration interval associated with each retained time
    /// point.
    pub step_sizes: Vec<Value>,

    /// Descriptor-keyed analog channels, in node, branch, device-observable,
    /// then device-store order.
    pub channels: Vec<TransientCompressedChannel>,

    /// XSPICE digital event traces, carried exactly. Event traces keep every
    /// sample: interpolation is undefined for them, so they are never
    /// decimated.
    pub digital_traces: Vec<DigitalTrace>,

    /// XSPICE real-valued event traces, carried exactly, for the same reason.
    pub real_traces: Vec<RealTrace>,

    /// Typed `.FFT`, `.FOUR`, and `.MEASURE` products computed on the exact
    /// accepted trajectory, before any decimation.
    pub post_results: TransientPostResults,

    /// Analysis instance, coordinate, and topology this result came from.
    pub identity: TransientResultIdentity,

    /// Compression ratio achieved
    pub compression_ratio: Value,

    /// Total number of simulation points before compression
    pub input_points: usize,

    /// Versioned applied-policy and final-grid reconstruction-error evidence.
    pub compression_report: TransientCompressionReport,
}

impl TransientResultCompressed {
    /// Number of solution nodes, excluding ground.
    pub fn num_nodes(&self) -> usize {
        self.channels
            .iter()
            .filter(|channel| {
                matches!(
                    channel.descriptor.role(),
                    TransientChannelRole::NodeVoltage { .. }
                )
            })
            .count()
    }

    /// Node names in solver order, as the netlist spells them.
    pub fn node_names(&self) -> Vec<String> {
        self.node_channels()
            .map(|channel| channel.1.descriptor.owner_name().to_string())
            .collect()
    }

    /// Branch names aligned with the retained branch-current channels.
    pub fn branch_names(&self) -> Vec<String> {
        self.channels
            .iter()
            .filter_map(|channel| match channel.descriptor.role() {
                TransientChannelRole::BranchCurrent { branch } => Some(branch.clone()),
                _ => None,
            })
            .collect()
    }

    /// Node-voltage channels paired with their zero-based node index.
    fn node_channels(&self) -> impl Iterator<Item = (usize, &TransientCompressedChannel)> {
        self.channels
            .iter()
            .filter_map(|channel| match channel.descriptor.role() {
                TransientChannelRole::NodeVoltage { node_index, .. } => {
                    Some((*node_index, channel))
                }
                _ => None,
            })
    }

    /// Look one channel up by its canonical identity.
    pub fn channel(
        &self,
        signal: &TransientCompressionSignal,
    ) -> Option<&TransientCompressedChannel> {
        self.channels.iter().find(|channel| {
            channel.descriptor.kind() == signal.kind
                && channel.descriptor.canonical_name() == signal.canonical_name
        })
    }

    /// Look one channel up by canonical name, ignoring case.
    pub fn channel_named(&self, canonical_name: &str) -> Option<&TransientCompressedChannel> {
        self.channels.iter().find(|channel| {
            channel
                .descriptor
                .canonical_name()
                .eq_ignore_ascii_case(canonical_name)
        })
    }

    /// The node-voltage channel for a zero-based node index.
    pub fn node_voltage_channel(&self, node_index: usize) -> Option<&TransientCompressedChannel> {
        self.node_channels()
            .find(|(index, _)| *index == node_index)
            .map(|(_, channel)| channel)
    }

    /// The branch-current channel for a canonical branch name.
    pub fn branch_current_channel(&self, name: &str) -> Option<&TransientCompressedChannel> {
        self.channels
            .iter()
            .find(|channel| match channel.descriptor.role() {
                TransientChannelRole::BranchCurrent { branch } => branch.eq_ignore_ascii_case(name),
                _ => false,
            })
    }

    /// The device operating-point channel for one device and parameter.
    pub fn device_op_channel(
        &self,
        device_name: &str,
        parameter: &str,
    ) -> Option<&TransientCompressedChannel> {
        self.channels
            .iter()
            .find(|channel| match channel.descriptor.role() {
                TransientChannelRole::DeviceObservable {
                    device,
                    parameter: candidate,
                } => {
                    device.eq_ignore_ascii_case(device_name)
                        && candidate.eq_ignore_ascii_case(parameter)
                }
                _ => false,
            })
    }

    /// The typed device-store channel for a canonical store name.
    pub fn store_channel(&self, name: &str) -> Option<&TransientCompressedChannel> {
        self.channels
            .iter()
            .find(|channel| match channel.descriptor.role() {
                TransientChannelRole::DeviceStore { store } => store.eq_ignore_ascii_case(name),
                _ => false,
            })
    }

    /// Validate the complete inventory and retained-grid alignment before
    /// exposing or expanding this result.
    pub fn validate(&self) -> Result<(), String> {
        let point_count = self.time.len();
        if self.step_sizes.len() != point_count {
            return Err(format!(
                "compressed transient has {} step sizes for {point_count} time points",
                self.step_sizes.len()
            ));
        }
        if self.input_points < point_count {
            return Err(format!(
                "compressed transient retains {point_count} points from an impossible {}-point input",
                self.input_points
            ));
        }
        self.identity.validate()?;
        self.validate_report_policy(point_count)?;
        self.validate_grid(point_count)?;
        self.validate_channels(point_count)?;
        self.validate_event_traces()?;
        self.validate_worst_observation()
    }

    fn validate_report_policy(&self, point_count: usize) -> Result<(), String> {
        let report = &self.compression_report;
        if report.schema_version != TRANSIENT_COMPRESSION_REPORT_VERSION {
            return Err(format!(
                "compressed transient has unsupported compression-report version {}",
                report.schema_version
            ));
        }
        if report.input_points != self.input_points || report.retained_points != point_count {
            return Err(format!(
                "compressed transient report counts {}/{} do not match result counts {}/{}",
                report.retained_points, report.input_points, point_count, self.input_points
            ));
        }
        let policy = &report.applied_policy;
        for (name, value) in [
            ("absolute tolerance", policy.absolute_tolerance),
            ("relative tolerance", policy.relative_tolerance),
            (
                "maximum retained interval",
                policy.maximum_retained_interval,
            ),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(format!(
                    "compressed transient report has invalid {name} {value}"
                ));
            }
        }
        if !policy.enabled && report.input_points != report.retained_points {
            return Err(
                "compressed transient report claims disabled compression discarded input points"
                    .to_string(),
            );
        }
        if !self.compression_ratio.is_finite() || self.compression_ratio < 1.0 {
            return Err(format!(
                "compressed transient has invalid compression ratio {}",
                self.compression_ratio
            ));
        }
        let expected_ratio = if point_count == 0 {
            1.0
        } else {
            self.input_points as Value / point_count as Value
        };
        let ratio_tolerance = 16.0 * Value::EPSILON * expected_ratio.max(1.0);
        if (self.compression_ratio - expected_ratio).abs() > ratio_tolerance {
            return Err(format!(
                "compressed transient ratio {} is inconsistent with {} retained points from a {}-point input (expected {expected_ratio})",
                self.compression_ratio, point_count, self.input_points
            ));
        }
        Ok(())
    }

    fn validate_grid(&self, _point_count: usize) -> Result<(), String> {
        if self
            .time
            .windows(2)
            .any(|window| !window[0].is_finite() || window[1] <= window[0])
            || self.time.last().is_some_and(|time| !time.is_finite())
        {
            return Err(
                "compressed transient time points must be finite and strictly increasing"
                    .to_string(),
            );
        }
        let policy = &self.compression_report.applied_policy;
        if policy.enabled
            && policy.maximum_retained_interval > 0.0
            && self.time.windows(2).any(|window| {
                retained_gap_exceeds(window[0], window[1], policy.maximum_retained_interval)
            })
        {
            return Err(format!(
                "compressed transient retained grid exceeds the declared maximum interval {}",
                policy.maximum_retained_interval
            ));
        }
        if self
            .step_sizes
            .iter()
            .any(|step| !step.is_finite() || *step < 0.0)
        {
            return Err(
                "compressed transient step sizes must be finite and non-negative".to_string(),
            );
        }
        Ok(())
    }

    fn validate_channels(&self, point_count: usize) -> Result<(), String> {
        let mut seen = std::collections::BTreeSet::new();
        let mut expected_node_index = 0usize;
        for channel in &self.channels {
            let descriptor = &channel.descriptor;
            let rebuilt = TransientChannelDescriptor::new(
                descriptor.role().clone(),
                descriptor.unit().clone(),
            )?;
            if rebuilt.canonical_name() != descriptor.canonical_name()
                || rebuilt.display_name() != descriptor.display_name()
            {
                return Err(format!(
                    "compressed transient channel '{}' has names inconsistent with its role",
                    descriptor.canonical_name()
                ));
            }
            if !seen.insert((
                descriptor.kind().as_str(),
                descriptor.canonical_name().to_string(),
            )) {
                return Err(format!(
                    "compressed transient has duplicate channel '{}:{}'",
                    descriptor.kind().as_str(),
                    descriptor.canonical_name()
                ));
            }
            if let TransientChannelRole::NodeVoltage { node_index, .. } = descriptor.role() {
                if *node_index != expected_node_index {
                    return Err(format!(
                        "compressed transient node channel '{}' has index {node_index} but solver order expects {expected_node_index}",
                        descriptor.canonical_name()
                    ));
                }
                expected_node_index += 1;
            }
            match channel.availability {
                TransientChannelAvailability::Available => {
                    if channel.samples.len() != point_count {
                        return Err(format!(
                            "compressed transient channel '{}' has {} samples for {point_count} time points",
                            descriptor.canonical_name(),
                            channel.samples.len()
                        ));
                    }
                }
                TransientChannelAvailability::NotProjected => {
                    if !channel.samples.is_empty() {
                        return Err(format!(
                            "compressed transient channel '{}' declares it was not projected but carries {} samples",
                            descriptor.canonical_name(),
                            channel.samples.len()
                        ));
                    }
                }
            }
            if channel
                .samples
                .iter()
                .any(|sample| sample.value().is_some_and(|value| !value.is_finite()))
            {
                return Err(format!(
                    "compressed transient channel '{}' published a non-finite value instead of an absence",
                    descriptor.canonical_name()
                ));
            }
        }
        Ok(())
    }

    fn validate_event_traces(&self) -> Result<(), String> {
        for trace in &self.digital_traces {
            if trace.node_name.trim().is_empty() {
                return Err("compressed transient digital trace has an empty node name".to_string());
            }
            if trace
                .points
                .windows(2)
                .any(|window| !window[0].time.is_finite() || window[1].time < window[0].time)
                || trace
                    .points
                    .last()
                    .is_some_and(|point| !point.time.is_finite())
            {
                return Err(format!(
                    "compressed transient digital trace '{}' has non-finite or unordered event times",
                    trace.node_name
                ));
            }
        }
        for trace in &self.real_traces {
            if trace.node_name.trim().is_empty() {
                return Err("compressed transient real trace has an empty node name".to_string());
            }
            if trace
                .points
                .windows(2)
                .any(|window| !window[0].time.is_finite() || window[1].time < window[0].time)
                || trace
                    .points
                    .last()
                    .is_some_and(|point| !point.time.is_finite())
            {
                return Err(format!(
                    "compressed transient real trace '{}' has non-finite or unordered event times",
                    trace.node_name
                ));
            }
        }
        Ok(())
    }

    fn validate_worst_observation(&self) -> Result<(), String> {
        let report = &self.compression_report;
        let policy = &report.applied_policy;
        let has_comparable_sample = self.channels.iter().any(|channel| {
            channel.availability == TransientChannelAvailability::Available
                && channel.samples.iter().any(|sample| sample.is_present())
        });
        let has_approximated_signal =
            report.input_points > report.retained_points && has_comparable_sample;
        let Some(observation) = &report.worst_observed else {
            if has_approximated_signal {
                return Err(
                    "compressed transient report omitted the worst approximated analog sample"
                        .to_string(),
                );
            }
            return Ok(());
        };
        if report.input_points == report.retained_points {
            return Err(
                "compressed transient report records an error when no sample was approximated"
                    .to_string(),
            );
        }
        if observation.input_sample_index >= report.input_points {
            return Err(format!(
                "compressed transient worst-error sample index {} is outside the {}-point input grid",
                observation.input_sample_index, report.input_points
            ));
        }
        if !observation.time.is_finite()
            || !self
                .time
                .first()
                .is_some_and(|start| observation.time >= *start)
            || !self
                .time
                .last()
                .is_some_and(|stop| observation.time <= *stop)
        {
            return Err(format!(
                "compressed transient worst-error time {} is outside the result interval",
                observation.time
            ));
        }
        if self
            .time
            .binary_search_by(|time| time.total_cmp(&observation.time))
            .is_ok()
        {
            return Err(
                "compressed transient worst-error observation names a retained sample".to_string(),
            );
        }
        let channel = self
            .channel(&observation.signal)
            .filter(|channel| channel.availability == TransientChannelAvailability::Available)
            .ok_or_else(|| {
                format!(
                    "compressed transient worst-error signal '{}:{}' does not exist in the result",
                    observation.signal.kind.as_str(),
                    observation.signal.canonical_name
                )
            })?;
        for (name, value) in [
            ("actual value", observation.actual_value),
            ("absolute error", observation.absolute_error),
            ("allowed tolerance", observation.allowed_tolerance),
            ("tolerance utilization", observation.tolerance_utilization),
        ] {
            if !value.is_finite() || (name != "actual value" && value < 0.0) {
                return Err(format!(
                    "compressed transient worst-error report has invalid {name} {value}"
                ));
            }
        }
        let reconstructed = self
            .interpolate_channel(channel, observation.time)
            .ok_or_else(|| {
                "compressed transient worst-error signal cannot be reconstructed".to_string()
            })?;
        let expected_absolute_error = (observation.actual_value - reconstructed).abs();
        if !expected_absolute_error.is_finite()
            || !certificate_value_matches(observation.absolute_error, expected_absolute_error)
        {
            return Err(format!(
                "compressed transient worst-error absolute error {} is inconsistent with actual value {} and reconstructed value {reconstructed} (expected {expected_absolute_error})",
                observation.absolute_error, observation.actual_value
            ));
        }
        let expected_relative_error = if observation.actual_value == 0.0 {
            None
        } else {
            let relative = observation.absolute_error / observation.actual_value.abs();
            relative.is_finite().then_some(relative)
        };
        if !optional_certificate_value_matches(observation.relative_error, expected_relative_error)
        {
            return Err(format!(
                "compressed transient worst-error relative error {:?} is inconsistent with error {} and actual value {}",
                observation.relative_error, observation.absolute_error, observation.actual_value
            ));
        }
        let expected_tolerance =
            policy.absolute_tolerance + policy.relative_tolerance * observation.actual_value.abs();
        if !expected_tolerance.is_finite()
            || !certificate_value_matches(observation.allowed_tolerance, expected_tolerance)
        {
            return Err(format!(
                "compressed transient worst-error tolerance {} is inconsistent with policy and actual value {} (expected {expected_tolerance})",
                observation.allowed_tolerance, observation.actual_value
            ));
        }
        let expected_utilization = if observation.allowed_tolerance > 0.0 {
            observation.absolute_error / observation.allowed_tolerance
        } else if observation.absolute_error == 0.0 {
            0.0
        } else {
            Value::INFINITY
        };
        let utilization_slack = 64.0
            * Value::EPSILON
            * expected_utilization
                .abs()
                .max(observation.tolerance_utilization.abs())
                .max(1.0);
        if !expected_utilization.is_finite()
            || (observation.tolerance_utilization - expected_utilization).abs() > utilization_slack
            || observation.tolerance_utilization > 1.0 + 64.0 * Value::EPSILON
        {
            return Err(format!(
                "compressed transient worst-error utilization {} is inconsistent with error {} and tolerance {}",
                observation.tolerance_utilization,
                observation.absolute_error,
                observation.allowed_tolerance
            ));
        }
        Ok(())
    }

    fn interpolate_channel(
        &self,
        channel: &TransientCompressedChannel,
        time: Value,
    ) -> Option<Value> {
        if channel.availability != TransientChannelAvailability::Available
            || channel.samples.len() != self.time.len()
            || self.time.is_empty()
            || !time.is_finite()
        {
            return None;
        }
        let times = &self.time;
        let samples = &channel.samples;
        if time <= times[0] {
            return samples[0].value();
        }
        if time >= *times.last()? {
            return samples.last()?.value();
        }
        let index = match times.binary_search_by(|candidate| candidate.total_cmp(&time)) {
            Ok(index) => return samples.get(index)?.value(),
            Err(index) => index.checked_sub(1)?,
        };
        let start = samples[index].value()?;
        let end = samples[index + 1].value()?;
        let t0 = times[index];
        let t1 = times[index + 1];
        let fraction = (time - t0) / (t1 - t0);
        Some(start + fraction * (end - start))
    }

    /// Get value at arbitrary time via linear interpolation
    ///
    /// This is how compressed waveforms are read - the stored points
    /// are the control points for piecewise linear interpolation.
    pub fn interpolate(&self, node: usize, time: Value) -> Option<Value> {
        self.interpolate_channel(self.node_voltage_channel(node)?, time)
    }

    /// Get the retained branch-current waveform for a canonical branch name.
    ///
    /// Returns `None` when the branch was not projected or any retained sample
    /// is absent; the samples themselves are then read through
    /// [`Self::branch_current_channel`].
    pub fn try_branch_current_waveform_named(&self, name: &str) -> Option<Vec<Value>> {
        self.branch_current_channel(name)?.dense_values()
    }

    /// Interpolate a retained branch-current waveform by canonical name.
    pub fn interpolate_branch_current_named(&self, name: &str, time: Value) -> Option<Value> {
        self.interpolate_channel(self.branch_current_channel(name)?, time)
    }

    /// Get a retained device operating-point waveform by device and parameter.
    pub fn try_device_op_waveform_named(
        &self,
        device_name: &str,
        parameter: &str,
    ) -> Option<Vec<Value>> {
        self.device_op_channel(device_name, parameter)?
            .dense_values()
    }

    /// Interpolate a retained device operating-point waveform.
    pub fn interpolate_device_op_named(
        &self,
        device_name: &str,
        parameter: &str,
        time: Value,
    ) -> Option<Value> {
        self.interpolate_channel(self.device_op_channel(device_name, parameter)?, time)
    }

    /// Get a retained typed device-store waveform by canonical name.
    pub fn try_store_waveform_named(&self, name: &str) -> Option<Vec<Value>> {
        self.store_channel(name)?.dense_values()
    }

    /// Interpolate a retained typed device-store waveform by canonical name.
    pub fn interpolate_store_named(&self, name: &str, time: Value) -> Option<Value> {
        self.interpolate_channel(self.store_channel(name)?, time)
    }

    /// Get the retained voltage waveform for a zero-based node index.
    pub fn try_voltage_waveform(&self, node: usize) -> Option<Vec<Value>> {
        self.node_voltage_channel(node)?.dense_values()
    }

    /// Sample waveform at uniform intervals
    ///
    /// Useful for FFT or other analysis that requires uniform sampling.
    pub fn resample(&self, node: usize, num_points: usize) -> Option<(Vec<Value>, Vec<Value>)> {
        if self.time.is_empty() || num_points < 2 {
            return None;
        }
        let channel = self.node_voltage_channel(node)?;

        let t_start = self.time[0];
        let t_end = *self.time.last()?;
        let dt = (t_end - t_start) / (num_points - 1) as Value;

        let times: Vec<_> = (0..num_points).map(|i| t_start + i as Value * dt).collect();

        let values = times
            .iter()
            .map(|&time| self.interpolate_channel(channel, time))
            .collect::<Option<Vec<_>>>()?;

        Some((times, values))
    }
}

impl TransientChannelDescriptor {
    /// Netlist spelling of this channel's owner.
    pub fn owner_name(&self) -> &str {
        match &self.role {
            TransientChannelRole::NodeVoltage { node, .. } => node,
            TransientChannelRole::BranchCurrent { branch } => branch,
            TransientChannelRole::DeviceObservable { device, .. } => device,
            TransientChannelRole::DeviceStore { store } => store,
        }
    }
}

fn certificate_value_matches(actual: Value, expected: Value) -> bool {
    let scale = actual.abs().max(expected.abs()).max(Value::MIN_POSITIVE);
    (actual - expected).abs() <= 64.0 * Value::EPSILON * scale
}

fn optional_certificate_value_matches(actual: Option<Value>, expected: Option<Value>) -> bool {
    match (actual, expected) {
        (Some(actual), Some(expected)) => {
            actual.is_finite() && actual >= 0.0 && certificate_value_matches(actual, expected)
        }
        (None, None) => true,
        _ => false,
    }
}

fn retained_gap_exceeds(start: Value, stop: Value, maximum_interval: Value) -> bool {
    let gap = stop - start;
    let scale = start
        .abs()
        .max(stop.abs())
        .max(maximum_interval.abs())
        .max(Value::MIN_POSITIVE);
    gap > maximum_interval + 64.0 * Value::EPSILON * scale
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn node_channel(
        node_index: usize,
        name: &str,
        samples: Vec<TransientChannelSample>,
    ) -> TransientCompressedChannel {
        let availability = if samples.is_empty() {
            TransientChannelAvailability::NotProjected
        } else {
            TransientChannelAvailability::Available
        };
        TransientCompressedChannel {
            descriptor: TransientChannelDescriptor::for_role(TransientChannelRole::NodeVoltage {
                node_index,
                node: name.to_string(),
            })
            .expect("node descriptor is well formed"),
            availability,
            samples,
        }
    }

    fn values(samples: &[Value]) -> Vec<TransientChannelSample> {
        samples
            .iter()
            .map(|value| TransientChannelSample::Value(*value))
            .collect()
    }

    fn malformed_compressed_result(
        channels: Vec<TransientCompressedChannel>,
    ) -> TransientResultCompressed {
        let config = CompressionConfig::none();
        TransientResultCompressed {
            time: vec![0.0, 1.0, 2.0],
            step_sizes: vec![0.0, 1.0, 1.0],
            channels,
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            post_results: TransientPostResults::default(),
            identity: TransientResultIdentity::default(),
            compression_ratio: 1.0,
            input_points: 3,
            compression_report: TransientCompressionReport::new(&config, 3, 3, None),
        }
    }

    #[test]
    fn compressed_result_rejects_misaligned_voltage_channels() {
        let missing_channel = malformed_compressed_result(Vec::new());
        assert_eq!(missing_channel.interpolate(0, 0.5), None);
        assert_eq!(missing_channel.resample(0, 3), None);
        missing_channel
            .validate()
            .expect("a result with no channels is structurally valid");

        let short_channel =
            malformed_compressed_result(vec![node_channel(0, "out", values(&[0.0]))]);
        assert_eq!(short_channel.interpolate(0, 0.5), None);
        assert_eq!(short_channel.resample(0, 3), None);
        assert!(short_channel.validate().is_err());
    }

    #[test]
    fn compressed_result_validates_projected_out_channels() {
        let mut projected = malformed_compressed_result(vec![node_channel(0, "out", Vec::new())]);
        projected.channels.push(TransientCompressedChannel {
            descriptor: TransientChannelDescriptor::for_role(TransientChannelRole::BranchCurrent {
                branch: "V1".to_string(),
            })
            .expect("branch descriptor is well formed"),
            availability: TransientChannelAvailability::NotProjected,
            samples: Vec::new(),
        });

        projected
            .validate()
            .expect("empty projected-out channels are typed missingness");
        assert_eq!(projected.interpolate(0, 0.5), None);
        assert_eq!(projected.try_branch_current_waveform_named("v1"), None);
        assert_eq!(projected.node_names(), vec!["out".to_string()]);
        assert_eq!(projected.branch_names(), vec!["V1".to_string()]);
    }

    #[test]
    fn compressed_result_rejects_a_published_non_finite_sample() {
        let mut fabricated =
            malformed_compressed_result(vec![node_channel(0, "out", values(&[0.0, 1.0, 2.0]))]);
        fabricated.channels[0].samples[1] = TransientChannelSample::Value(Value::NAN);
        assert!(
            fabricated
                .validate()
                .expect_err("a non-finite number must be an absence instead")
                .contains("instead of an absence")
        );

        fabricated.channels[0].samples[1] =
            TransientChannelSample::Absent(TransientSampleAbsence::NonFinite);
        fabricated
            .validate()
            .expect("an explicit absence is the supported representation");
        assert_eq!(fabricated.channels[0].dense_values(), None);
        assert_eq!(
            fabricated.channels[0].validity(),
            vec![true, false, true],
            "validity marks the absent sample"
        );
    }

    #[test]
    fn compressed_result_rejects_duplicate_and_misordered_channels() {
        let duplicate = malformed_compressed_result(vec![
            node_channel(0, "out", values(&[0.0, 1.0, 2.0])),
            node_channel(1, "OUT", values(&[0.0, 1.0, 2.0])),
        ]);
        assert!(
            duplicate
                .validate()
                .expect_err("canonical channel names are unique")
                .contains("duplicate channel")
        );

        let misordered = malformed_compressed_result(vec![
            node_channel(1, "out", values(&[0.0, 1.0, 2.0])),
            node_channel(0, "in", values(&[0.0, 1.0, 2.0])),
        ]);
        assert!(
            misordered
                .validate()
                .expect_err("node channels stay in solver order")
                .contains("solver order")
        );
    }

    #[test]
    fn compressed_result_rejects_inconsistent_ratio_metadata() {
        let mut malformed =
            malformed_compressed_result(vec![node_channel(0, "out", values(&[0.0, 1.0, 2.0]))]);
        malformed.input_points = 6;
        malformed.compression_ratio = 1.5;
        malformed.compression_report.input_points = 6;
        malformed.compression_report.applied_policy.enabled = true;

        let error = malformed
            .validate()
            .expect_err("ratio metadata must agree with retained and input point counts");
        assert!(error.contains("inconsistent"), "unexpected error: {error}");
    }

    #[test]
    fn compressed_result_rejects_malformed_error_certificate() {
        let mut certified =
            malformed_compressed_result(vec![node_channel(0, "out", values(&[0.0, 1.0, 2.0]))]);
        certified.input_points = 4;
        certified.compression_ratio = 4.0 / 3.0;
        let observed_actual = 0.500_000_5_f64;
        let observed_error = (observed_actual - 0.5).abs();
        let observed_tolerance = 1.0e-6 + 1.0e-3 * observed_actual.abs();
        certified.compression_report = TransientCompressionReport {
            schema_version: TRANSIENT_COMPRESSION_REPORT_VERSION,
            algorithm: TransientCompressionAlgorithm::MultiChannelRdpLinearV1,
            sample_domain: TransientCompressionSampleDomain::AcceptedInputSamples,
            applied_policy: (&CompressionConfig::default()).into(),
            input_points: 4,
            retained_points: 3,
            worst_observed: Some(TransientCompressionErrorObservation {
                signal: TransientCompressionSignal::voltage("out").unwrap(),
                input_sample_index: 1,
                time: 0.5,
                actual_value: observed_actual,
                absolute_error: observed_error,
                relative_error: Some(observed_error / observed_actual),
                allowed_tolerance: observed_tolerance,
                tolerance_utilization: observed_error / observed_tolerance,
            }),
        };
        certified
            .validate()
            .expect("baseline certificate validates");

        let mut missing = certified.clone();
        missing.compression_report.worst_observed = None;
        assert!(
            missing
                .validate()
                .expect_err("discarded analog samples require an observation")
                .contains("omitted")
        );

        let mut future = certified.clone();
        future.compression_report.schema_version += 1;
        assert!(
            future
                .validate()
                .expect_err("future report versions fail closed")
                .contains("unsupported")
        );

        let mut unknown_signal = certified.clone();
        unknown_signal
            .compression_report
            .worst_observed
            .as_mut()
            .unwrap()
            .signal = TransientCompressionSignal::voltage("missing").unwrap();
        assert!(
            unknown_signal
                .validate()
                .expect_err("the reported signal must exist")
                .contains("does not exist")
        );

        let mut impossible_tolerance = certified.clone();
        let observation = impossible_tolerance
            .compression_report
            .worst_observed
            .as_mut()
            .unwrap();
        observation.allowed_tolerance = 1.0e-6;
        observation.tolerance_utilization = observation.absolute_error / 1.0e-6;
        assert!(
            impossible_tolerance
                .validate()
                .expect_err("policy algebra must be independently checkable")
                .contains("inconsistent with policy")
        );

        let mut impossible_relative = certified.clone();
        impossible_relative
            .compression_report
            .worst_observed
            .as_mut()
            .unwrap()
            .relative_error = Some(2.0e-6);
        assert!(
            impossible_relative
                .validate()
                .expect_err("relative-error algebra must be independently checkable")
                .contains("relative error")
        );

        let mut impossible_interval = certified.clone();
        impossible_interval
            .compression_report
            .applied_policy
            .maximum_retained_interval = 0.5;
        assert!(
            impossible_interval
                .validate()
                .expect_err("the retained grid must honor its declared interval")
                .contains("maximum interval")
        );

        let mut retained_sample = certified.clone();
        retained_sample
            .compression_report
            .worst_observed
            .as_mut()
            .unwrap()
            .time = 1.0;
        assert!(
            retained_sample
                .validate()
                .expect_err("a retained sample was not reconstructed")
                .contains("retained sample")
        );

        let mut projected_out = certified.clone();
        projected_out.channels[0].samples.clear();
        projected_out.channels[0].availability = TransientChannelAvailability::NotProjected;
        assert!(
            projected_out
                .validate()
                .expect_err("a projected-out channel cannot support an error observation")
                .contains("does not exist")
        );

        let mut over_budget = certified;
        let observation = over_budget
            .compression_report
            .worst_observed
            .as_mut()
            .unwrap();
        observation.actual_value = 0.501;
        observation.absolute_error = (observation.actual_value - 0.5).abs();
        observation.relative_error = Some(observation.absolute_error / observation.actual_value);
        observation.allowed_tolerance = 1.0e-6 + 1.0e-3 * observation.actual_value;
        observation.tolerance_utilization =
            observation.absolute_error / observation.allowed_tolerance;
        assert!(
            over_budget
                .validate()
                .expect_err("a certificate cannot claim an over-budget result")
                .contains("utilization")
        );
    }

    #[test]
    fn channel_unit_wire_spellings_round_trip() {
        for unit in [
            TransientChannelUnit::Volt,
            TransientChannelUnit::Ampere,
            TransientChannelUnit::Ohm,
            TransientChannelUnit::Siemens,
            TransientChannelUnit::Watt,
            TransientChannelUnit::Hertz,
            TransientChannelUnit::Second,
            TransientChannelUnit::Degree,
            TransientChannelUnit::Radian,
            TransientChannelUnit::Dimensionless,
            TransientChannelUnit::Unspecified,
            TransientChannelUnit::Custom("V/K".to_string()),
        ] {
            let tag = unit.as_str().to_string();
            assert_eq!(
                TransientChannelUnit::from_tag(&tag).expect("known unit spelling parses"),
                unit
            );
        }
        assert!(TransientChannelUnit::from_tag("   ").is_err());
    }

    #[test]
    fn descriptor_carries_unit_owner_and_shape_for_every_role() {
        let voltage = TransientChannelDescriptor::for_role(TransientChannelRole::NodeVoltage {
            node_index: 0,
            node: "OUT".to_string(),
        })
        .expect("voltage descriptor");
        assert_eq!(voltage.canonical_name(), "v(out)");
        assert_eq!(voltage.display_name(), "V(OUT)");
        assert_eq!(*voltage.unit(), TransientChannelUnit::Volt);
        assert_eq!(voltage.owner(), TransientChannelOwner::Node("OUT".into()));
        assert_eq!(voltage.value_type(), "real");
        assert_eq!(voltage.shape(), "scalar");

        let current = TransientChannelDescriptor::for_role(TransientChannelRole::BranchCurrent {
            branch: "V1".to_string(),
        })
        .expect("current descriptor");
        assert_eq!(current.canonical_name(), "i(v1)");
        assert_eq!(*current.unit(), TransientChannelUnit::Ampere);
        assert_eq!(current.owner(), TransientChannelOwner::Branch("V1".into()));

        let observable =
            TransientChannelDescriptor::for_role(TransientChannelRole::DeviceObservable {
                device: "M1".to_string(),
                parameter: "gm".to_string(),
            })
            .expect("observable descriptor");
        assert_eq!(observable.canonical_name(), "@m1[gm]");
        assert_eq!(*observable.unit(), TransientChannelUnit::Unspecified);
        assert_eq!(
            observable.owner(),
            TransientChannelOwner::Device("M1".into())
        );

        let store = TransientChannelDescriptor::for_role(TransientChannelRole::DeviceStore {
            store: "YMEM!R1:R".to_string(),
        })
        .expect("store descriptor");
        assert_eq!(store.canonical_name(), "ymem!r1:r");
        assert!(
            TransientChannelDescriptor::new(
                TransientChannelRole::DeviceStore {
                    store: "YMEM!R1:R".to_string()
                },
                TransientChannelUnit::Custom(" ".to_string())
            )
            .expect_err("an empty custom unit symbol is refused")
            .contains("unit symbol")
        );
    }
}
