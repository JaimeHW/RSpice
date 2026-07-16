//! Durable, validated preferences that have concrete runtime consumers.
//!
//! Mockup settings are added here only after their owning subsystem consumes
//! the value. This prevents a persisted form value from being mistaken for an
//! implemented engineering policy.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::quantity::{QuantityPresentationPolicy, UnitsPreferences};

use super::shortcuts::{ShortcutPreferences, ShortcutProfileLibrary, ShortcutProfileLibraryError};

/// Mockup-defined workspace composition applied by the workbench owner.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspacePreset {
    #[default]
    Engineering,
    Canvas,
    Diagnostics,
}

impl WorkspacePreset {
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Engineering => 0,
            Self::Canvas => 1,
            Self::Diagnostics => 2,
        }
    }

    pub fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::Engineering),
            1 => Ok(Self::Canvas),
            2 => Ok(Self::Diagnostics),
            _ => Err("workspace preset index is outside its domain"),
        }
    }
}

/// Default console visibility applied once when a workbench session launches.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConsoleLaunchBehavior {
    #[default]
    Collapsed,
    Open,
}

impl ConsoleLaunchBehavior {
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Collapsed => 0,
            Self::Open => 1,
        }
    }

    pub fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::Collapsed),
            1 => Ok(Self::Open),
            _ => Err("console launch behavior index is outside its domain"),
        }
    }

    #[must_use]
    pub const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }
}

/// Amount of retained background activity allowed to request user attention.
///
/// RSpice never raises or focuses a host window from this policy. The runtime
/// consumer only controls the in-application activity stream and badge.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackgroundTaskAttention {
    #[default]
    BadgeAndNotify,
    NotifyOnFailureOnly,
    Silent,
}

impl BackgroundTaskAttention {
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::BadgeAndNotify => 0,
            Self::NotifyOnFailureOnly => 1,
            Self::Silent => 2,
        }
    }

    pub fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::BadgeAndNotify),
            1 => Ok(Self::NotifyOnFailureOnly),
            2 => Ok(Self::Silent),
            _ => Err("background task attention index is outside its domain"),
        }
    }

    #[must_use]
    pub const fn retains(self, failure: bool) -> bool {
        match self {
            Self::BadgeAndNotify => true,
            Self::NotifyOnFailureOnly => failure,
            Self::Silent => false,
        }
    }
}

/// Durable workspace preferences whose values have concrete workbench
/// consumers. Unknown fields round-trip for forward compatibility.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WorkspacePreferences {
    preset: WorkspacePreset,
    console_on_launch: ConsoleLaunchBehavior,
    background_task_attention: BackgroundTaskAttention,
    #[serde(flatten)]
    unknown: BTreeMap<String, Value>,
}

impl WorkspacePreferences {
    #[must_use]
    pub const fn preset(&self) -> WorkspacePreset {
        self.preset
    }

    pub fn set_preset(&mut self, preset: WorkspacePreset) {
        self.preset = preset;
    }

    #[must_use]
    pub const fn console_on_launch(&self) -> ConsoleLaunchBehavior {
        self.console_on_launch
    }

    pub fn set_console_on_launch(&mut self, behavior: ConsoleLaunchBehavior) {
        self.console_on_launch = behavior;
    }

    #[must_use]
    pub const fn background_task_attention(&self) -> BackgroundTaskAttention {
        self.background_task_attention
    }

    pub fn set_background_task_attention(&mut self, attention: BackgroundTaskAttention) {
        self.background_task_attention = attention;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum WorkspacePreferencesStorage {
    Current(WorkspacePreferences),
    Future(Value),
}

impl Default for WorkspacePreferencesStorage {
    fn default() -> Self {
        Self::Current(WorkspacePreferences::default())
    }
}

impl WorkspacePreferencesStorage {
    fn current(&self) -> Option<&WorkspacePreferences> {
        match self {
            Self::Current(preferences) => Some(preferences),
            Self::Future(_) => None,
        }
    }

    fn current_mut(&mut self) -> Result<&mut WorkspacePreferences, &'static str> {
        match self {
            Self::Current(preferences) => Ok(preferences),
            Self::Future(_) => Err("workspace preferences were written by an incompatible build"),
        }
    }

    fn is_default(&self) -> bool {
        matches!(self, Self::Current(preferences) if preferences == &WorkspacePreferences::default())
    }
}

/// Validated number of significant digits used by result presentation.
///
/// This never changes a stored waveform sample or an engineering export. It
/// controls only human-facing labels and readouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct DisplayedSignificantDigits(u8);

impl DisplayedSignificantDigits {
    pub const MIN: u8 = 3;
    pub const MAX: u8 = 17;
    pub const DEFAULT: u8 = 7;

    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl Default for DisplayedSignificantDigits {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl TryFrom<u8> for DisplayedSignificantDigits {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err("displayed significant digits must be between 3 and 17")
        }
    }
}

impl<'de> Deserialize<'de> for DisplayedSignificantDigits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CursorInterpolation {
    #[default]
    MonotoneCubicWhereValid,
    Linear,
    NearestAcceptedPoint,
}

impl CursorInterpolation {
    const fn index(self) -> usize {
        match self {
            Self::MonotoneCubicWhereValid => 0,
            Self::Linear => 1,
            Self::NearestAcceptedPoint => 2,
        }
    }

    fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::MonotoneCubicWhereValid),
            1 => Ok(Self::Linear),
            2 => Ok(Self::NearestAcceptedPoint),
            _ => Err("cursor interpolation index is outside its domain"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComplexNumberDisplay {
    #[default]
    MagnitudePhaseDegrees,
    RealImaginary,
    MagnitudePhaseRadians,
}

impl ComplexNumberDisplay {
    const fn index(self) -> usize {
        match self {
            Self::MagnitudePhaseDegrees => 0,
            Self::RealImaginary => 1,
            Self::MagnitudePhaseRadians => 2,
        }
    }

    fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::MagnitudePhaseDegrees),
            1 => Ok(Self::RealImaginary),
            2 => Ok(Self::MagnitudePhaseRadians),
            _ => Err("complex-number display index is outside its domain"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LargeDatasetDisplay {
    #[default]
    EnvelopeExtrema,
    UniformDisplaySampling,
    NoDisplayDecimation,
}

impl LargeDatasetDisplay {
    const fn index(self) -> usize {
        match self {
            Self::EnvelopeExtrema => 0,
            Self::UniformDisplaySampling => 1,
            Self::NoDisplayDecimation => 2,
        }
    }

    fn from_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::EnvelopeExtrema),
            1 => Ok(Self::UniformDisplaySampling),
            2 => Ok(Self::NoDisplayDecimation),
            _ => Err("large-dataset display index is outside its domain"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringExportFormat {
    #[default]
    Csv,
    TouchstoneWhereCompatible,
    /// Reserved by the approved mockup. The UI must not offer this until the
    /// UI crate has a verified cross-platform HDF5 publication backend.
    Hdf5EngineeringDataset,
}

impl EngineeringExportFormat {
    const fn index(self) -> usize {
        match self {
            Self::Csv => 0,
            Self::TouchstoneWhereCompatible => 1,
            Self::Hdf5EngineeringDataset => 2,
        }
    }

    fn from_runtime_index(index: usize) -> Result<Self, &'static str> {
        match index {
            0 => Ok(Self::Csv),
            1 => Ok(Self::TouchstoneWhereCompatible),
            2 => Err("HDF5 engineering export is not available in this build"),
            _ => Err("engineering export index is outside its domain"),
        }
    }

    #[must_use]
    pub const fn is_runtime_supported(self) -> bool {
        !matches!(self, Self::Hdf5EngineeringDataset)
    }
}

/// Durable user defaults for result presentation with actual runtime owners.
/// Unknown fields are retained so a newer build can round-trip through this
/// version without losing additions to the Results domain.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ResultsPreferences {
    displayed_significant_digits: DisplayedSignificantDigits,
    cursor_interpolation: CursorInterpolation,
    complex_number_display: ComplexNumberDisplay,
    large_dataset_display: LargeDatasetDisplay,
    engineering_export: EngineeringExportFormat,
    #[serde(flatten)]
    unknown: BTreeMap<String, Value>,
}

impl ResultsPreferences {
    #[must_use]
    pub const fn displayed_significant_digits(&self) -> DisplayedSignificantDigits {
        self.displayed_significant_digits
    }

    #[must_use]
    pub const fn cursor_interpolation(&self) -> CursorInterpolation {
        self.cursor_interpolation
    }

    #[must_use]
    pub const fn complex_number_display(&self) -> ComplexNumberDisplay {
        self.complex_number_display
    }

    #[must_use]
    pub const fn large_dataset_display(&self) -> LargeDatasetDisplay {
        self.large_dataset_display
    }

    #[must_use]
    pub const fn engineering_export(&self) -> EngineeringExportFormat {
        self.engineering_export
    }

    fn presentation_policy(&self) -> ResultPresentationPolicy {
        ResultPresentationPolicy {
            displayed_significant_digits: self.displayed_significant_digits,
            cursor_interpolation: self.cursor_interpolation,
            complex_number_display: self.complex_number_display,
            large_dataset_display: self.large_dataset_display,
            engineering_export: if self.engineering_export.is_runtime_supported() {
                self.engineering_export
            } else {
                EngineeringExportFormat::default()
            },
        }
    }
}

/// Fully resolved result policy consumed by viewers and export dispatch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ResultPresentationPolicy {
    displayed_significant_digits: DisplayedSignificantDigits,
    cursor_interpolation: CursorInterpolation,
    complex_number_display: ComplexNumberDisplay,
    large_dataset_display: LargeDatasetDisplay,
    engineering_export: EngineeringExportFormat,
}

impl ResultPresentationPolicy {
    #[must_use]
    pub const fn displayed_significant_digits(self) -> DisplayedSignificantDigits {
        self.displayed_significant_digits
    }

    #[must_use]
    pub const fn cursor_interpolation(self) -> CursorInterpolation {
        self.cursor_interpolation
    }

    #[must_use]
    pub const fn complex_number_display(self) -> ComplexNumberDisplay {
        self.complex_number_display
    }

    #[must_use]
    pub const fn large_dataset_display(self) -> LargeDatasetDisplay {
        self.large_dataset_display
    }

    #[must_use]
    pub const fn engineering_export(self) -> EngineeringExportFormat {
        self.engineering_export
    }
}

/// Forward-compatible storage for the complete Results domain. If a future
/// schema changes the root shape, retain it verbatim and run current defaults
/// rather than invalidating the complete recoverable application session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum ResultsPreferencesStorage {
    Current(ResultsPreferences),
    Future(Value),
}

impl Default for ResultsPreferencesStorage {
    fn default() -> Self {
        Self::Current(ResultsPreferences::default())
    }
}

impl ResultsPreferencesStorage {
    fn current(&self) -> Option<&ResultsPreferences> {
        match self {
            Self::Current(preferences) => Some(preferences),
            Self::Future(_) => None,
        }
    }

    fn current_mut(&mut self) -> Result<&mut ResultsPreferences, &'static str> {
        match self {
            Self::Current(preferences) => Ok(preferences),
            Self::Future(_) => Err("results preferences were written by an incompatible build"),
        }
    }

    fn is_default(&self) -> bool {
        matches!(self, Self::Current(preferences) if preferences == &ResultsPreferences::default())
    }
}

/// Forward-compatible root wrapper. Future non-object or otherwise
/// incompatible unit domains are retained verbatim and resolve to safe
/// defaults instead of invalidating the entire recoverable session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum UnitsPreferencesStorage {
    Current(UnitsPreferences),
    Future(Value),
}

impl Default for UnitsPreferencesStorage {
    fn default() -> Self {
        Self::Current(UnitsPreferences::default())
    }
}

impl UnitsPreferencesStorage {
    fn current(&self) -> Option<&UnitsPreferences> {
        match self {
            Self::Current(preferences) => Some(preferences),
            Self::Future(_) => None,
        }
    }

    fn current_mut(&mut self) -> Result<&mut UnitsPreferences, &'static str> {
        match self {
            Self::Current(preferences) => Ok(preferences),
            Self::Future(_) => Err("units preferences were written by an incompatible build"),
        }
    }

    fn is_default(&self) -> bool {
        matches!(self, Self::Current(preferences) if preferences.is_empty())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChoicePreference {
    /// Retired persisted key from the first Preferences implementation.
    /// Layout presets were not a single source of truth for dock commands,
    /// so restored values are accepted for migration and then discarded.
    #[serde(rename = "workspace-preset")]
    LegacyWorkspacePreset,
    /// Retired persisted key. Console visibility is owned by the workbench
    /// session until a distinct launch-policy owner exists.
    #[serde(rename = "console-on-launch")]
    LegacyConsoleOnLaunch,
    /// Retired persisted key. Schematic grid size is currently document
    /// presentation state, not a globally enforced new-document policy.
    #[serde(rename = "schematic-grid")]
    LegacySchematicGrid,
    InterfaceScale,
    MinimumTouchTarget,
    UnitSystem,
    EngineeringSuffixes,
    FrequencyDisplay,
    TemperatureDisplay,
    CopiedValueFormat,
    AngleDisplay,
    LayoutCoordinateDisplay,
    TimeFrequencyInput,
    DecimalSeparatorInput,
    #[serde(rename = "schematic-grid-default")]
    SchematicGrid,
    OperatingPointAnnotation,
    WireJunctionBehavior,
    SelectionCrossingPolicy,
    NetNamingPolicy,
    PropertyCommitPolicy,
    DefaultSolverPreset,
    ConvergenceFailurePolicy,
    CheckpointPolicy,
    EngineSelection,
    RandomSeedPolicy,
    IncrementalResultReuse,
    LicenseWaitBehavior,
    LiveResultStreaming,
    DiagnosticArtifact,
    CursorInterpolation,
    ComplexNumberDisplay,
    FamilyTraceLabeling,
    DefaultAxisPolicy,
    LargeDatasetDisplay,
    MeasurementEvaluation,
    EngineeringExport,
    DefaultComputeTarget,
    QueuePriority,
    RuntimeEnvironment,
    TransferPolicy,
    PreemptionTimeout,
    UnavailableTarget,
    CertificateTrust,
    NetworkProxy,
    LocalProjectEncryption,
    AutomationSandbox,
    DataResidency,
    ExtensionPolicy,
    ExtensionUpdates,
}

impl ChoicePreference {
    const ALL: [Self; 49] = [
        Self::LegacyWorkspacePreset,
        Self::LegacyConsoleOnLaunch,
        Self::LegacySchematicGrid,
        Self::InterfaceScale,
        Self::MinimumTouchTarget,
        Self::UnitSystem,
        Self::EngineeringSuffixes,
        Self::FrequencyDisplay,
        Self::TemperatureDisplay,
        Self::CopiedValueFormat,
        Self::AngleDisplay,
        Self::LayoutCoordinateDisplay,
        Self::TimeFrequencyInput,
        Self::DecimalSeparatorInput,
        Self::SchematicGrid,
        Self::OperatingPointAnnotation,
        Self::WireJunctionBehavior,
        Self::SelectionCrossingPolicy,
        Self::NetNamingPolicy,
        Self::PropertyCommitPolicy,
        Self::DefaultSolverPreset,
        Self::ConvergenceFailurePolicy,
        Self::CheckpointPolicy,
        Self::EngineSelection,
        Self::RandomSeedPolicy,
        Self::IncrementalResultReuse,
        Self::LicenseWaitBehavior,
        Self::LiveResultStreaming,
        Self::DiagnosticArtifact,
        Self::CursorInterpolation,
        Self::ComplexNumberDisplay,
        Self::FamilyTraceLabeling,
        Self::DefaultAxisPolicy,
        Self::LargeDatasetDisplay,
        Self::MeasurementEvaluation,
        Self::EngineeringExport,
        Self::DefaultComputeTarget,
        Self::QueuePriority,
        Self::RuntimeEnvironment,
        Self::TransferPolicy,
        Self::PreemptionTimeout,
        Self::UnavailableTarget,
        Self::CertificateTrust,
        Self::NetworkProxy,
        Self::LocalProjectEncryption,
        Self::AutomationSandbox,
        Self::DataResidency,
        Self::ExtensionPolicy,
        Self::ExtensionUpdates,
    ];

    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::LegacyWorkspacePreset => "workspace-preset",
            Self::LegacyConsoleOnLaunch => "console-on-launch",
            Self::LegacySchematicGrid => "schematic-grid",
            Self::InterfaceScale => "interface-scale",
            Self::MinimumTouchTarget => "minimum-touch-target",
            Self::UnitSystem => "unit-system",
            Self::EngineeringSuffixes => "engineering-suffixes",
            Self::FrequencyDisplay => "frequency-display",
            Self::TemperatureDisplay => "temperature-display",
            Self::CopiedValueFormat => "copied-value-format",
            Self::AngleDisplay => "angle-display",
            Self::LayoutCoordinateDisplay => "layout-coordinate-display",
            Self::TimeFrequencyInput => "time-frequency-input",
            Self::DecimalSeparatorInput => "decimal-separator-input",
            Self::SchematicGrid => "schematic-grid-default",
            Self::OperatingPointAnnotation => "operating-point-annotation",
            Self::WireJunctionBehavior => "wire-junction-behavior",
            Self::SelectionCrossingPolicy => "selection-crossing-policy",
            Self::NetNamingPolicy => "net-naming-policy",
            Self::PropertyCommitPolicy => "property-commit-policy",
            Self::DefaultSolverPreset => "default-solver-preset",
            Self::ConvergenceFailurePolicy => "convergence-failure-policy",
            Self::CheckpointPolicy => "checkpoint-policy",
            Self::EngineSelection => "engine-selection",
            Self::RandomSeedPolicy => "random-seed-policy",
            Self::IncrementalResultReuse => "incremental-result-reuse",
            Self::LicenseWaitBehavior => "license-wait-behavior",
            Self::LiveResultStreaming => "live-result-streaming",
            Self::DiagnosticArtifact => "diagnostic-artifact",
            Self::CursorInterpolation => "cursor-interpolation",
            Self::ComplexNumberDisplay => "complex-number-display",
            Self::FamilyTraceLabeling => "family-trace-labeling",
            Self::DefaultAxisPolicy => "default-axis-policy",
            Self::LargeDatasetDisplay => "large-dataset-display",
            Self::MeasurementEvaluation => "measurement-evaluation",
            Self::EngineeringExport => "engineering-export",
            Self::DefaultComputeTarget => "default-compute-target",
            Self::QueuePriority => "queue-priority",
            Self::RuntimeEnvironment => "runtime-environment",
            Self::TransferPolicy => "transfer-policy",
            Self::PreemptionTimeout => "preemption-timeout",
            Self::UnavailableTarget => "unavailable-target",
            Self::CertificateTrust => "certificate-trust",
            Self::NetworkProxy => "network-proxy",
            Self::LocalProjectEncryption => "local-project-encryption",
            Self::AutomationSandbox => "automation-sandbox",
            Self::DataResidency => "data-residency",
            Self::ExtensionPolicy => "extension-policy",
            Self::ExtensionUpdates => "extension-updates",
        }
    }

    const fn max_value(self) -> u8 {
        match self {
            Self::LegacyWorkspacePreset
            | Self::LegacySchematicGrid
            | Self::UnitSystem
            | Self::TemperatureDisplay
            | Self::LayoutCoordinateDisplay
            | Self::SchematicGrid
            | Self::OperatingPointAnnotation
            | Self::WireJunctionBehavior
            | Self::SelectionCrossingPolicy
            | Self::ConvergenceFailurePolicy
            | Self::CheckpointPolicy
            | Self::EngineSelection
            | Self::RandomSeedPolicy
            | Self::LicenseWaitBehavior
            | Self::DiagnosticArtifact
            | Self::CursorInterpolation
            | Self::ComplexNumberDisplay
            | Self::FamilyTraceLabeling
            | Self::DefaultAxisPolicy
            | Self::LargeDatasetDisplay
            | Self::EngineeringExport
            | Self::DefaultComputeTarget
            | Self::QueuePriority
            | Self::UnavailableTarget
            | Self::NetworkProxy
            | Self::AutomationSandbox
            | Self::ExtensionPolicy
            | Self::ExtensionUpdates => 2,
            Self::LegacyConsoleOnLaunch
            | Self::MinimumTouchTarget
            | Self::EngineeringSuffixes
            | Self::FrequencyDisplay
            | Self::CopiedValueFormat
            | Self::AngleDisplay
            | Self::TimeFrequencyInput
            | Self::DecimalSeparatorInput
            | Self::NetNamingPolicy
            | Self::PropertyCommitPolicy
            | Self::IncrementalResultReuse
            | Self::LiveResultStreaming
            | Self::MeasurementEvaluation
            | Self::RuntimeEnvironment
            | Self::TransferPolicy
            | Self::PreemptionTimeout
            | Self::CertificateTrust
            | Self::LocalProjectEncryption
            | Self::DataResidency => 1,
            Self::InterfaceScale | Self::DefaultSolverPreset => 3,
        }
    }

    const fn is_runtime_consumed(self) -> bool {
        !matches!(
            self,
            Self::LegacyWorkspacePreset
                | Self::LegacyConsoleOnLaunch
                | Self::LegacySchematicGrid
                | Self::FamilyTraceLabeling
                | Self::DefaultAxisPolicy
                | Self::MeasurementEvaluation
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TogglePreference {
    ReducedMotion,
    HierarchicalEditInPlace,
    CrossProbeBehavior,
    IncrementalConnectivityChecks,
    RestorePlotDocuments,
    SynchronizeResultFamilyCrossProbes,
    AutomaticDiagnostics,
    PythonAutomationApi,
}

impl TogglePreference {
    const ALL: [Self; 8] = [
        Self::ReducedMotion,
        Self::HierarchicalEditInPlace,
        Self::CrossProbeBehavior,
        Self::IncrementalConnectivityChecks,
        Self::RestorePlotDocuments,
        Self::SynchronizeResultFamilyCrossProbes,
        Self::AutomaticDiagnostics,
        Self::PythonAutomationApi,
    ];

    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::ReducedMotion => "reduced-motion",
            Self::HierarchicalEditInPlace => "hierarchical-edit-in-place",
            Self::CrossProbeBehavior => "cross-probe-behavior",
            Self::IncrementalConnectivityChecks => "incremental-connectivity-checks",
            Self::RestorePlotDocuments => "restore-plot-documents",
            Self::SynchronizeResultFamilyCrossProbes => "synchronize-result-family-cross-probes",
            Self::AutomaticDiagnostics => "automatic-diagnostics",
            Self::PythonAutomationApi => "python-automation-api",
        }
    }

    const fn default_value(self) -> bool {
        !matches!(self, Self::ReducedMotion | Self::AutomaticDiagnostics)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScalarPreference {
    LocalParallelSlots,
    DisplayedSignificantDigits,
    ParallelTaskCeiling,
}

impl ScalarPreference {
    const ALL: [Self; 3] = [
        Self::LocalParallelSlots,
        Self::DisplayedSignificantDigits,
        Self::ParallelTaskCeiling,
    ];

    const fn stable_id(self) -> &'static str {
        match self {
            Self::LocalParallelSlots => "local-parallel-slots",
            Self::DisplayedSignificantDigits => "displayed-significant-digits",
            Self::ParallelTaskCeiling => "parallel-task-ceiling",
        }
    }

    const fn default_value(self) -> u32 {
        match self {
            Self::LocalParallelSlots => 12,
            Self::DisplayedSignificantDigits => 7,
            Self::ParallelTaskCeiling => 64,
        }
    }

    const fn bounds(self) -> (u32, u32) {
        match self {
            Self::LocalParallelSlots => (1, 1024),
            Self::DisplayedSignificantDigits => (3, 17),
            Self::ParallelTaskCeiling => (1, 4096),
        }
    }
}

/// User/device overrides consumed by the workbench at runtime. Missing values
/// read as the reviewed zero/false defaults, keeping legacy sessions valid.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPreferences {
    /// String-keyed wire storage keeps a newer preference from invalidating
    /// the complete recoverable application session in an older build.
    /// Known keys are validated through the typed accessors below; unknown
    /// keys are retained byte-semantically for a later compatible build.
    choices: BTreeMap<String, Value>,
    toggles: BTreeMap<String, Value>,
    scalars: BTreeMap<String, Value>,
    #[serde(skip_serializing_if = "WorkspacePreferencesStorage::is_default")]
    workspace: WorkspacePreferencesStorage,
    #[serde(skip_serializing_if = "ResultsPreferencesStorage::is_default")]
    results: ResultsPreferencesStorage,
    #[serde(skip_serializing_if = "UnitsPreferencesStorage::is_default")]
    units: UnitsPreferencesStorage,
    shortcuts: ShortcutProfileLibrary,
    /// Forward-compatible typed domains that this build does not understand.
    #[serde(flatten)]
    unknown_domains: BTreeMap<String, Value>,
}

impl UserPreferences {
    /// Typed Workspace preferences when the stored domain is compatible with
    /// this build. A future incompatible root is retained without being
    /// mistaken for an enforceable runtime policy.
    #[must_use]
    pub fn workspace(&self) -> Option<&WorkspacePreferences> {
        self.workspace.current()
    }

    pub fn workspace_mut(&mut self) -> Result<&mut WorkspacePreferences, &'static str> {
        self.workspace.current_mut()
    }

    /// Typed Units preferences when the stored domain is compatible with this
    /// build. An incompatible future root remains preserved on the wire.
    #[must_use]
    pub fn units(&self) -> Option<&UnitsPreferences> {
        self.units.current()
    }

    pub fn units_mut(&mut self) -> Result<&mut UnitsPreferences, &'static str> {
        self.units.current_mut()
    }

    /// Immutable display/input policy for one UI transaction.
    #[must_use]
    pub fn quantity_presentation_policy(&self) -> QuantityPresentationPolicy {
        self.units.current().map_or_else(
            QuantityPresentationPolicy::default,
            UnitsPreferences::presentation_policy,
        )
    }

    /// Typed Results preferences when the stored domain is compatible with
    /// this build. An incompatible future root is retained and resolves to
    /// safe current defaults instead of invalidating the session.
    #[must_use]
    pub fn results(&self) -> Option<&ResultsPreferences> {
        self.results.current()
    }

    #[must_use]
    pub fn result_presentation_policy(&self) -> ResultPresentationPolicy {
        self.results.current().map_or_else(
            ResultPresentationPolicy::default,
            ResultsPreferences::presentation_policy,
        )
    }

    #[must_use]
    pub const fn shortcuts(&self) -> &ShortcutPreferences {
        self.shortcuts.active()
    }

    pub fn shortcuts_mut(
        &mut self,
    ) -> Result<&mut ShortcutPreferences, ShortcutProfileLibraryError> {
        self.shortcuts.active_mut()
    }

    #[must_use]
    pub const fn shortcut_profiles(&self) -> &ShortcutProfileLibrary {
        &self.shortcuts
    }

    pub fn shortcut_profiles_mut(&mut self) -> &mut ShortcutProfileLibrary {
        &mut self.shortcuts
    }

    #[must_use]
    pub fn choice(&self, key: ChoicePreference) -> usize {
        if is_units_choice(key) {
            return self
                .units
                .current()
                .and_then(|units| units.compatibility_index(key.stable_id()))
                .unwrap_or_default();
        }
        let results = self.result_presentation_policy();
        let value = match key {
            ChoicePreference::CursorInterpolation => results.cursor_interpolation.index(),
            ChoicePreference::ComplexNumberDisplay => results.complex_number_display.index(),
            ChoicePreference::LargeDatasetDisplay => results.large_dataset_display.index(),
            ChoicePreference::EngineeringExport => results.engineering_export.index(),
            _ => usize::MAX,
        };
        if value != usize::MAX {
            return value;
        }
        if !key.is_runtime_consumed() {
            return 0;
        }
        usize::from(
            self.choices
                .get(key.stable_id())
                .and_then(Value::as_u64)
                .and_then(|value| u8::try_from(value).ok())
                .filter(|value| *value <= key.max_value())
                .unwrap_or_default(),
        )
    }

    pub fn set_choice(&mut self, key: ChoicePreference, value: usize) -> Result<(), &'static str> {
        if is_units_choice(key) {
            return self
                .units
                .current_mut()?
                .set_compatibility_index(key.stable_id(), value)
                .expect("the units choice set is kept in sync with its compatibility adapter");
        }
        match key {
            ChoicePreference::CursorInterpolation => {
                self.results.current_mut()?.cursor_interpolation =
                    CursorInterpolation::from_index(value)?;
                return Ok(());
            }
            ChoicePreference::ComplexNumberDisplay => {
                self.results.current_mut()?.complex_number_display =
                    ComplexNumberDisplay::from_index(value)?;
                return Ok(());
            }
            ChoicePreference::LargeDatasetDisplay => {
                self.results.current_mut()?.large_dataset_display =
                    LargeDatasetDisplay::from_index(value)?;
                return Ok(());
            }
            ChoicePreference::EngineeringExport => {
                self.results.current_mut()?.engineering_export =
                    EngineeringExportFormat::from_runtime_index(value)?;
                return Ok(());
            }
            _ => {}
        }
        if !key.is_runtime_consumed() {
            return Err("preference no longer has a runtime owner");
        }
        let value = u8::try_from(value).map_err(|_| "choice index is not representable")?;
        if value > key.max_value() {
            return Err("choice index is outside the preference domain");
        }
        if value == 0 {
            self.choices.remove(key.stable_id());
        } else {
            self.choices
                .insert(key.stable_id().to_owned(), Value::from(value));
        }
        Ok(())
    }

    #[must_use]
    pub fn toggle(&self, key: TogglePreference) -> bool {
        self.toggles
            .get(key.stable_id())
            .and_then(Value::as_bool)
            .unwrap_or_else(|| key.default_value())
    }

    pub fn set_toggle(&mut self, key: TogglePreference, value: bool) {
        if value == key.default_value() {
            self.toggles.remove(key.stable_id());
        } else {
            self.toggles
                .insert(key.stable_id().to_owned(), Value::Bool(value));
        }
    }

    #[must_use]
    pub fn scalar(&self, key: ScalarPreference) -> u32 {
        if key == ScalarPreference::DisplayedSignificantDigits {
            return u32::from(
                self.result_presentation_policy()
                    .displayed_significant_digits()
                    .get(),
            );
        }
        let (minimum, maximum) = key.bounds();
        self.scalars
            .get(key.stable_id())
            .and_then(Value::as_u64)
            .and_then(|value| u32::try_from(value).ok())
            .filter(|value| (minimum..=maximum).contains(value))
            .unwrap_or_else(|| key.default_value())
    }

    pub fn set_scalar(&mut self, key: ScalarPreference, value: u32) -> Result<(), &'static str> {
        if key == ScalarPreference::DisplayedSignificantDigits {
            let value = u8::try_from(value)
                .map_err(|_| "displayed significant digits are not representable by this build")?;
            self.results.current_mut()?.displayed_significant_digits =
                DisplayedSignificantDigits::try_from(value)?;
            return Ok(());
        }
        let (minimum, maximum) = key.bounds();
        if !(minimum..=maximum).contains(&value) {
            return Err("numeric preference is outside its validated domain");
        }
        if value == key.default_value() {
            self.scalars.remove(key.stable_id());
        } else {
            self.scalars
                .insert(key.stable_id().to_owned(), Value::from(value));
        }
        Ok(())
    }

    pub(crate) fn normalize(&mut self) {
        self.migrate_legacy_workspace_storage();
        self.migrate_legacy_units_storage();
        self.migrate_legacy_results_storage();
        for key in ChoicePreference::ALL {
            if !key.is_runtime_consumed()
                || self
                    .choices
                    .get(key.stable_id())
                    .and_then(Value::as_u64)
                    .is_none_or(|value| value > u64::from(key.max_value()))
            {
                self.choices.remove(key.stable_id());
            }
        }
        for key in TogglePreference::ALL {
            if self
                .toggles
                .get(key.stable_id())
                .is_some_and(|value| !value.is_boolean())
            {
                self.toggles.remove(key.stable_id());
            }
        }
        for key in ScalarPreference::ALL {
            let (minimum, maximum) = key.bounds();
            if self
                .scalars
                .get(key.stable_id())
                .and_then(Value::as_u64)
                .and_then(|value| u32::try_from(value).ok())
                .is_none_or(|value| !(minimum..=maximum).contains(&value))
            {
                self.scalars.remove(key.stable_id());
            }
        }
    }

    fn migrate_legacy_workspace_storage(&mut self) {
        let Ok(workspace) = self.workspace.current_mut() else {
            return;
        };
        if let Some(index) =
            take_legacy_choice(&mut self.choices, ChoicePreference::LegacyWorkspacePreset)
            && let Ok(value) = WorkspacePreset::from_index(index)
        {
            workspace.preset = value;
        }
        if let Some(index) =
            take_legacy_choice(&mut self.choices, ChoicePreference::LegacyConsoleOnLaunch)
            && let Ok(value) = ConsoleLaunchBehavior::from_index(index)
        {
            workspace.console_on_launch = value;
        }
    }

    fn migrate_legacy_units_storage(&mut self) {
        let Ok(units) = self.units.current_mut() else {
            return;
        };
        for key in UNIT_CHOICE_KEYS {
            let stable_id = key.stable_id();
            let legacy = self.choices.remove(stable_id);
            if units.contains_wire_key(stable_id) {
                continue;
            }
            if let Some(index) = legacy
                .and_then(|value| value.as_u64())
                .and_then(|value| usize::try_from(value).ok())
            {
                let _ = units.set_compatibility_index(stable_id, index);
            }
        }
    }

    fn migrate_legacy_results_storage(&mut self) {
        let Ok(results) = self.results.current_mut() else {
            return;
        };
        if let Some(value) = self
            .scalars
            .remove(ScalarPreference::DisplayedSignificantDigits.stable_id())
            .and_then(|value| value.as_u64())
            .and_then(|value| u8::try_from(value).ok())
            .and_then(|value| DisplayedSignificantDigits::try_from(value).ok())
        {
            results.displayed_significant_digits = value;
        }
        if let Some(index) =
            take_legacy_choice(&mut self.choices, ChoicePreference::CursorInterpolation)
            && let Ok(value) = CursorInterpolation::from_index(index)
        {
            results.cursor_interpolation = value;
        }
        if let Some(index) =
            take_legacy_choice(&mut self.choices, ChoicePreference::ComplexNumberDisplay)
            && let Ok(value) = ComplexNumberDisplay::from_index(index)
        {
            results.complex_number_display = value;
        }
        if let Some(index) =
            take_legacy_choice(&mut self.choices, ChoicePreference::LargeDatasetDisplay)
            && let Ok(value) = LargeDatasetDisplay::from_index(index)
        {
            results.large_dataset_display = value;
        }
        if let Some(index) =
            take_legacy_choice(&mut self.choices, ChoicePreference::EngineeringExport)
            && let Ok(value) = EngineeringExportFormat::from_runtime_index(index)
        {
            results.engineering_export = value;
        }
    }

    #[must_use]
    pub fn interface_scale(&self) -> f32 {
        [1.0, 1.1, 1.25, 1.5][self.choice(ChoicePreference::InterfaceScale)]
    }
}

const UNIT_CHOICE_KEYS: [ChoicePreference; 9] = [
    ChoicePreference::UnitSystem,
    ChoicePreference::EngineeringSuffixes,
    ChoicePreference::FrequencyDisplay,
    ChoicePreference::TemperatureDisplay,
    ChoicePreference::CopiedValueFormat,
    ChoicePreference::AngleDisplay,
    ChoicePreference::LayoutCoordinateDisplay,
    ChoicePreference::TimeFrequencyInput,
    ChoicePreference::DecimalSeparatorInput,
];

const fn is_units_choice(key: ChoicePreference) -> bool {
    matches!(
        key,
        ChoicePreference::UnitSystem
            | ChoicePreference::EngineeringSuffixes
            | ChoicePreference::FrequencyDisplay
            | ChoicePreference::TemperatureDisplay
            | ChoicePreference::CopiedValueFormat
            | ChoicePreference::AngleDisplay
            | ChoicePreference::LayoutCoordinateDisplay
            | ChoicePreference::TimeFrequencyInput
            | ChoicePreference::DecimalSeparatorInput
    )
}

fn take_legacy_choice(
    choices: &mut BTreeMap<String, Value>,
    key: ChoicePreference,
) -> Option<usize> {
    choices
        .remove(key.stable_id())
        .and_then(|value| value.as_u64())
        .and_then(|value| usize::try_from(value).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumed_overrides_round_trip() {
        let mut preferences = UserPreferences::default();
        preferences
            .set_choice(ChoicePreference::InterfaceScale, 2)
            .unwrap();
        preferences.set_toggle(TogglePreference::ReducedMotion, true);
        let json = serde_json::to_string(&preferences).unwrap();
        let restored: UserPreferences = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.choice(ChoicePreference::InterfaceScale), 2);
        assert!(restored.toggle(TogglePreference::ReducedMotion));
        assert_eq!(restored.interface_scale(), 1.25);
    }

    #[test]
    fn units_compatibility_controls_write_the_typed_domain() {
        let mut preferences = UserPreferences::default();
        preferences
            .set_choice(ChoicePreference::FrequencyDisplay, 1)
            .unwrap();
        preferences
            .set_choice(ChoicePreference::TemperatureDisplay, 2)
            .unwrap();
        preferences
            .set_choice(ChoicePreference::AngleDisplay, 1)
            .unwrap();

        let units = preferences.units().unwrap();
        assert_eq!(
            units.frequency_display(),
            crate::quantity::FrequencyDisplay::RadiansPerSecond
        );
        assert_eq!(
            units.temperature_display(),
            crate::quantity::TemperatureDisplay::Fahrenheit
        );
        assert_eq!(
            units.angle_display(),
            crate::quantity::AngleDisplay::Radians
        );
        let encoded = serde_json::to_value(preferences).unwrap();
        assert_eq!(encoded["units"]["frequency-display"], "radians-per-second");
        assert!(encoded["choices"].get("frequency-display").is_none());
    }

    #[test]
    fn legacy_unit_indices_migrate_without_overwriting_a_typed_value() {
        let mut preferences: UserPreferences = serde_json::from_str(
            r#"{
                "choices":{"frequency-display":1,"temperature-display":2},
                "units":{"frequency-display":"hertz-engineering"}
            }"#,
        )
        .unwrap();
        preferences.normalize();

        assert_eq!(preferences.choice(ChoicePreference::FrequencyDisplay), 0);
        assert_eq!(preferences.choice(ChoicePreference::TemperatureDisplay), 2);
        let encoded = serde_json::to_value(preferences).unwrap();
        assert!(encoded["choices"].get("frequency-display").is_none());
        assert!(encoded["choices"].get("temperature-display").is_none());
    }

    #[test]
    fn incompatible_future_units_root_is_retained_and_fail_closed() {
        let mut preferences: UserPreferences =
            serde_json::from_str(r#"{"units":17,"choices":{"frequency-display":1}}"#).unwrap();
        preferences.normalize();

        assert!(preferences.units().is_none());
        assert_eq!(preferences.choice(ChoicePreference::FrequencyDisplay), 0);
        assert!(
            preferences
                .set_choice(ChoicePreference::FrequencyDisplay, 1)
                .is_err()
        );
        assert_eq!(serde_json::to_value(preferences).unwrap()["units"], 17);
    }

    #[test]
    fn results_controls_write_a_typed_domain_and_round_trip_unknown_fields() {
        let mut preferences: UserPreferences =
            serde_json::from_str(r#"{"results":{"future-render-contract":{"version":2}}}"#)
                .unwrap();
        preferences
            .set_scalar(ScalarPreference::DisplayedSignificantDigits, 11)
            .unwrap();
        preferences
            .set_choice(ChoicePreference::CursorInterpolation, 1)
            .unwrap();
        preferences
            .set_choice(ChoicePreference::ComplexNumberDisplay, 1)
            .unwrap();
        preferences
            .set_choice(ChoicePreference::LargeDatasetDisplay, 2)
            .unwrap();
        preferences
            .set_choice(ChoicePreference::EngineeringExport, 1)
            .unwrap();

        let policy = preferences.result_presentation_policy();
        assert_eq!(policy.displayed_significant_digits().get(), 11);
        assert_eq!(policy.cursor_interpolation(), CursorInterpolation::Linear);
        assert_eq!(
            policy.complex_number_display(),
            ComplexNumberDisplay::RealImaginary
        );
        assert_eq!(
            policy.large_dataset_display(),
            LargeDatasetDisplay::NoDisplayDecimation
        );
        assert_eq!(
            policy.engineering_export(),
            EngineeringExportFormat::TouchstoneWhereCompatible
        );

        let encoded = serde_json::to_value(preferences).unwrap();
        assert_eq!(encoded["results"]["displayed-significant-digits"], 11);
        assert_eq!(encoded["results"]["cursor-interpolation"], "linear");
        assert_eq!(
            encoded["results"]["complex-number-display"],
            "real-imaginary"
        );
        assert_eq!(
            encoded["results"]["large-dataset-display"],
            "no-display-decimation"
        );
        assert_eq!(
            encoded["results"]["engineering-export"],
            "touchstone-where-compatible"
        );
        assert_eq!(encoded["results"]["future-render-contract"]["version"], 2);
        assert!(encoded["choices"].get("complex-number-display").is_none());
        assert!(
            encoded["scalars"]
                .get("displayed-significant-digits")
                .is_none()
        );
    }

    #[test]
    fn legacy_result_indices_migrate_into_typed_storage() {
        let mut preferences: UserPreferences = serde_json::from_str(
            r#"{
                "choices":{
                    "cursor-interpolation":2,
                    "complex-number-display":1,
                    "large-dataset-display":2,
                    "engineering-export":1
                },
                "scalars":{"displayed-significant-digits":13}
            }"#,
        )
        .unwrap();
        preferences.normalize();

        assert_eq!(
            preferences.scalar(ScalarPreference::DisplayedSignificantDigits),
            13
        );
        assert_eq!(preferences.choice(ChoicePreference::CursorInterpolation), 2);
        assert_eq!(
            preferences.choice(ChoicePreference::ComplexNumberDisplay),
            1
        );
        assert_eq!(preferences.choice(ChoicePreference::LargeDatasetDisplay), 2);
        assert_eq!(preferences.choice(ChoicePreference::EngineeringExport), 1);
        let encoded = serde_json::to_value(preferences).unwrap();
        assert!(encoded["choices"].get("cursor-interpolation").is_none());
        assert!(
            encoded["scalars"]
                .get("displayed-significant-digits")
                .is_none()
        );
    }

    #[test]
    fn incompatible_future_results_root_is_retained_and_fail_closed() {
        let mut preferences: UserPreferences =
            serde_json::from_str(r#"{"results":17,"choices":{"complex-number-display":1}}"#)
                .unwrap();
        preferences.normalize();

        assert!(preferences.results().is_none());
        assert_eq!(
            preferences.choice(ChoicePreference::ComplexNumberDisplay),
            0
        );
        assert!(
            preferences
                .set_choice(ChoicePreference::ComplexNumberDisplay, 1)
                .is_err()
        );
        assert_eq!(serde_json::to_value(preferences).unwrap()["results"], 17);
    }

    #[test]
    fn reserved_hdf5_value_is_preserved_but_never_exposed_as_runtime_policy() {
        let mut preferences: UserPreferences = serde_json::from_str(
            r#"{"results":{"engineering-export":"hdf5-engineering-dataset"}}"#,
        )
        .unwrap();

        assert_eq!(preferences.choice(ChoicePreference::EngineeringExport), 0);
        assert_eq!(
            preferences
                .result_presentation_policy()
                .engineering_export(),
            EngineeringExportFormat::Csv
        );
        assert!(
            preferences
                .set_choice(ChoicePreference::EngineeringExport, 2)
                .is_err()
        );
        assert_eq!(
            serde_json::to_value(preferences).unwrap()["results"]["engineering-export"],
            "hdf5-engineering-dataset"
        );
    }

    #[test]
    fn setters_reject_out_of_domain_values() {
        let mut preferences = UserPreferences::default();
        assert!(
            preferences
                .set_choice(ChoicePreference::MinimumTouchTarget, 2)
                .is_err()
        );
    }

    #[test]
    fn legacy_workspace_indices_migrate_into_the_typed_workspace_domain() {
        let mut preferences: UserPreferences = serde_json::from_str(
            r#"{"choices":{"workspace-preset":2,"console-on-launch":1,"schematic-grid":1},"toggles":{}}"#,
        )
        .unwrap();

        assert_eq!(
            preferences.choice(ChoicePreference::LegacyWorkspacePreset),
            0
        );
        preferences.normalize();

        let workspace = preferences.workspace().unwrap();
        assert_eq!(workspace.preset(), WorkspacePreset::Diagnostics);
        assert_eq!(workspace.console_on_launch(), ConsoleLaunchBehavior::Open);
        assert_eq!(preferences.choice(ChoicePreference::LegacySchematicGrid), 0);
        let encoded = serde_json::to_value(&preferences).unwrap();
        assert_eq!(encoded["workspace"]["preset"], "diagnostics");
        assert_eq!(encoded["workspace"]["console-on-launch"], "open");
        assert!(encoded["choices"].get("workspace-preset").is_none());
        assert!(
            preferences
                .set_choice(ChoicePreference::LegacyWorkspacePreset, 1)
                .is_err()
        );
    }

    #[test]
    fn workspace_preferences_are_typed_and_preserve_future_fields() {
        let mut preferences: UserPreferences =
            serde_json::from_str(r#"{"workspace":{"future-layout-contract":{"version":2}}}"#)
                .unwrap();
        let workspace = preferences.workspace_mut().unwrap();
        workspace.set_preset(WorkspacePreset::Canvas);
        workspace.set_console_on_launch(ConsoleLaunchBehavior::Open);
        workspace.set_background_task_attention(BackgroundTaskAttention::Silent);

        let encoded = serde_json::to_value(&preferences).unwrap();
        assert_eq!(encoded["workspace"]["preset"], "canvas");
        assert_eq!(encoded["workspace"]["console-on-launch"], "open");
        assert_eq!(encoded["workspace"]["background-task-attention"], "silent");
        assert_eq!(encoded["workspace"]["future-layout-contract"]["version"], 2);
    }

    #[test]
    fn background_attention_policy_has_no_focus_escalation_state() {
        assert!(BackgroundTaskAttention::BadgeAndNotify.retains(false));
        assert!(BackgroundTaskAttention::BadgeAndNotify.retains(true));
        assert!(!BackgroundTaskAttention::NotifyOnFailureOnly.retains(false));
        assert!(BackgroundTaskAttention::NotifyOnFailureOnly.retains(true));
        assert!(!BackgroundTaskAttention::Silent.retains(false));
        assert!(!BackgroundTaskAttention::Silent.retains(true));
    }

    #[test]
    fn incompatible_future_workspace_root_is_retained_and_fail_closed() {
        let mut preferences: UserPreferences =
            serde_json::from_str(r#"{"workspace":17,"choices":{"workspace-preset":2}}"#).unwrap();
        preferences.normalize();

        assert!(preferences.workspace().is_none());
        assert!(preferences.workspace_mut().is_err());
        assert_eq!(serde_json::to_value(preferences).unwrap()["workspace"], 17);
    }

    #[test]
    fn unknown_preference_keys_and_domains_do_not_invalidate_or_disappear() {
        let source = r#"{
            "choices":{"interface-scale":2,"future-density-mode":7},
            "toggles":{"reduced-motion":true,"future-motion-policy":{"mode":"quiet"}},
            "units":{},
            "future-results-policy":{"digits":11,"mode":"exact"}
        }"#;
        let mut restored: UserPreferences = serde_json::from_str(source).unwrap();
        restored.normalize();

        assert_eq!(restored.choice(ChoicePreference::InterfaceScale), 2);
        assert!(restored.toggle(TogglePreference::ReducedMotion));
        let encoded = serde_json::to_value(&restored).unwrap();
        assert_eq!(encoded["choices"]["future-density-mode"], 7);
        assert_eq!(encoded["toggles"]["future-motion-policy"]["mode"], "quiet");
        assert_eq!(encoded["future-results-policy"]["digits"], 11);
    }

    #[test]
    fn malformed_known_values_are_isolated_without_touching_unknown_values() {
        let mut restored: UserPreferences = serde_json::from_str(
            r#"{
                "choices":{"interface-scale":"large","future-choice":9},
                "toggles":{"reduced-motion":17,"future-toggle":false}
            }"#,
        )
        .unwrap();
        restored.normalize();

        assert_eq!(restored.choice(ChoicePreference::InterfaceScale), 0);
        assert!(!restored.toggle(TogglePreference::ReducedMotion));
        let encoded = serde_json::to_value(restored).unwrap();
        assert_eq!(encoded["choices"]["future-choice"], 9);
        assert_eq!(encoded["toggles"]["future-toggle"], false);
        assert!(encoded["choices"].get("interface-scale").is_none());
        assert!(encoded["toggles"].get("reduced-motion").is_none());
    }

    #[test]
    fn shortcut_profile_round_trips_inside_user_preferences() {
        let mut preferences = UserPreferences::default();
        preferences
            .shortcuts_mut()
            .unwrap()
            .set_binding(
                crate::workbench::commands::Command::Save,
                crate::workbench::shortcuts::ShortcutBindingSlot::Primary,
                crate::workbench::commands::CommandPlatform::ALL.to_vec(),
                Some(crate::workbench::shortcuts::ShortcutSequence::single(
                    crate::workbench::shortcuts::ShortcutStroke::new(
                        egui::Key::F6,
                        false,
                        false,
                        false,
                    ),
                )),
            )
            .unwrap();
        let encoded = serde_json::to_string(&preferences).unwrap();
        let restored: UserPreferences = serde_json::from_str(&encoded).unwrap();
        assert_eq!(
            restored
                .shortcuts()
                .resolved_bindings(crate::workbench::commands::Command::Save)
                .into_iter()
                .find(|binding| {
                    binding.slot() == crate::workbench::shortcuts::ShortcutBindingSlot::Primary
                })
                .unwrap()
                .display_label(),
            "F6"
        );
    }

    #[test]
    fn future_incompatible_shortcut_root_does_not_invalidate_the_session() {
        let mut preferences: UserPreferences = serde_json::from_str(r#"{"shortcuts":17}"#).unwrap();
        assert_eq!(serde_json::to_value(&preferences).unwrap()["shortcuts"], 17);
        assert!(!preferences.shortcuts().audit().is_valid());

        assert!(matches!(
            preferences.shortcuts_mut(),
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        ));
        assert_eq!(serde_json::to_value(&preferences).unwrap()["shortcuts"], 17);
    }
}
