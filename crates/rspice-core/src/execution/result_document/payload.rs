//! Per-family typed payloads for [`AnalysisResultDocument`].
//!
//! Everything a core result carries that is not "an axis plus signals plus
//! scalars" lives here: S-matrix ports, distortion product identity, pole and
//! zero sets with their numerical evidence, noise contributor tables, Monte
//! Carlo statistics, PSS Floquet provenance, PAC sideband and conversion data,
//! HB reactive spectra, FFT transform settings and ranked harmonics, transient
//! step sizes, event traces and the compression certificate, and the envelope
//! continuation identity.
//!
//! [`AnalysisResultDocument`]: super::AnalysisResultDocument

use serde::{Deserialize, Serialize};

use super::{ComplexSample, ResultDocumentError, finite, finite_optional, finite_slice};
use crate::analysis::distortion::DistortionProduct;
use crate::analysis::floquet::{FloquetOrbitKind, FloquetSpectrumEvidence};
use crate::analysis::harmonic_balance::{HbContinuationLimitation, HbReactiveKind};
use crate::analysis::noise::NoiseSourceType;
use crate::analysis::pole_zero::{RootSetEvidence, SpectrumCertificate};
use crate::analysis::sensitivity::ElementType;
use crate::engine::HbEnvelopeStateGuarantee;
use crate::engine::waveform::{
    TransientCompressionAlgorithm, TransientCompressionErrorObservation,
    TransientCompressionPolicy, TransientCompressionReport, TransientCompressionSampleDomain,
    TransientCompressionSignal, TransientCompressionSignalKind,
};
use crate::execution::capability::AnalysisResultKind;
use crate::execution::plan::AnalysisInstanceId;
use crate::execution::schema::SignalUnit;
use crate::netlist::{FftFormat, FftOutput, FftWindow, XyceFftMode};
use crate::xspice::{DigitalState, DigitalStrength};

/// Data that does not fit the shared axis/signal/scalar shape, one variant per
/// [`AnalysisResultKind`].
///
/// The exhaustive [`ResultPayload::result_kind`] match is the compile-time half
/// of the capability gate: a new result family cannot be added to
/// `AnalysisResultKind` without also being given a payload here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "family", rename_all = "kebab-case")]
pub enum ResultPayload {
    Op(OperatingPointPayload),
    Dc(DcSweepPayload),
    Ac(AcPayload),
    Tran(TransientPayload),
    Noise(NoisePayload),
    Sp(SParameterPayload),
    PortNoise(PortNoisePayload),
    Distortion(DistortionPayload),
    Tf(TransferFunctionPayload),
    Stb(StabilityPayload),
    Sensitivity(SensitivityPayload),
    PoleZero(PoleZeroPayload),
    Fourier(FourierPayload),
    Fft(FftPayload),
    MonteCarlo(MonteCarloPayload),
    Pss(PssPayload),
    Pac(PacPayload),
    PNoise(PNoisePayload),
    Hb(HarmonicBalancePayload),
    Envelope(EnvelopePayload),
}

impl ResultPayload {
    /// The result family this payload belongs to.
    pub const fn result_kind(&self) -> AnalysisResultKind {
        match self {
            Self::Op(_) => AnalysisResultKind::OperatingPoint,
            Self::Dc(_) => AnalysisResultKind::DcSweep,
            Self::Ac(_) => AnalysisResultKind::Ac,
            Self::Tran(_) => AnalysisResultKind::Transient,
            Self::Noise(_) => AnalysisResultKind::Noise,
            Self::Sp(_) => AnalysisResultKind::SParameters,
            Self::PortNoise(_) => AnalysisResultKind::PortNoise,
            Self::Distortion(_) => AnalysisResultKind::Distortion,
            Self::Tf(_) => AnalysisResultKind::TransferFunction,
            Self::Stb(_) => AnalysisResultKind::Stability,
            Self::Sensitivity(_) => AnalysisResultKind::Sensitivity,
            Self::PoleZero(_) => AnalysisResultKind::PoleZero,
            Self::Fourier(_) => AnalysisResultKind::Fourier,
            Self::Fft(_) => AnalysisResultKind::Fft,
            Self::MonteCarlo(_) => AnalysisResultKind::MonteCarlo,
            Self::Pss(_) => AnalysisResultKind::Pss,
            Self::Pac(_) => AnalysisResultKind::Pac,
            Self::PNoise(_) => AnalysisResultKind::PNoise,
            Self::Hb(_) => AnalysisResultKind::HarmonicBalance,
            Self::Envelope(_) => AnalysisResultKind::Envelope,
        }
    }

    /// Numerical values retained by this payload, for resource accounting.
    pub fn value_count(&self) -> usize {
        match self {
            Self::Op(payload) => payload.observables.len(),
            Self::Dc(payload) => payload
                .observables
                .iter()
                .map(|series| series.values.len())
                .fold(0, usize::saturating_add),
            Self::Ac(_) => 0,
            Self::Tran(payload) => payload.value_count(),
            Self::Noise(payload) => payload
                .contributions
                .iter()
                .map(NoiseContributionSeries::value_count)
                .fold(0, usize::saturating_add),
            Self::Sp(payload) => payload.ports.len().saturating_mul(2),
            Self::PortNoise(_) => 0,
            Self::Distortion(payload) => payload
                .products
                .iter()
                .map(|product| product.frequencies.len())
                .fold(0, usize::saturating_add),
            Self::Tf(_) => 0,
            Self::Stb(payload) => payload.nyquist.len().saturating_mul(3),
            Self::Sensitivity(payload) => payload.entries.len().saturating_mul(3),
            Self::PoleZero(payload) => payload
                .poles
                .len()
                .saturating_add(payload.zeros.len())
                .saturating_mul(2),
            Self::Fourier(_) => 0,
            Self::Fft(payload) => payload.metrics.as_ref().map_or(0, |metrics| {
                metrics.largest_harmonics.len().saturating_mul(4)
            }),
            Self::MonteCarlo(payload) => payload
                .statistics
                .iter()
                .map(MonteCarloVariableStatistics::value_count)
                .fold(0, usize::saturating_add),
            Self::Pss(payload) => payload.floquet_multipliers.len().saturating_mul(2),
            Self::Pac(payload) => payload.value_count(),
            Self::PNoise(payload) => payload
                .contributors
                .iter()
                .map(|contributor| contributor.contributions.len().saturating_mul(2))
                .fold(0, usize::saturating_add),
            Self::Hb(payload) => payload
                .reactive_spectra
                .iter()
                .map(HbReactiveSpectrumDocument::value_count)
                .fold(0, usize::saturating_add),
            Self::Envelope(payload) => payload
                .carrier
                .value_count()
                .saturating_add(payload.transient.value_count()),
        }
    }

    pub(super) fn validate(&self) -> Result<(), ResultDocumentError> {
        match self {
            Self::Op(payload) => payload.validate(),
            Self::Dc(payload) => payload.validate(),
            Self::Ac(_) | Self::PortNoise(_) => Ok(()),
            Self::Tran(payload) => payload.validate(),
            Self::Noise(payload) => payload.validate(),
            Self::Sp(payload) => payload.validate(),
            Self::Distortion(payload) => payload.validate(),
            Self::Tf(payload) => payload.validate(),
            Self::Stb(payload) => payload.validate(),
            Self::Sensitivity(payload) => payload.validate(),
            Self::PoleZero(payload) => payload.validate(),
            Self::Fourier(payload) => payload.validate(),
            Self::Fft(payload) => payload.validate(),
            Self::MonteCarlo(payload) => payload.validate(),
            Self::Pss(payload) => payload.validate(),
            Self::Pac(payload) => payload.validate(),
            Self::PNoise(payload) => payload.validate(),
            Self::Hb(payload) => payload.validate(),
            Self::Envelope(payload) => payload.validate(),
        }
    }
}

//=============================================================================
// Shared named observables
//=============================================================================

/// One named quantity a result carries under its own probe spelling.
///
/// `unit` is `None` when the producing result declares no unit for the probe.
/// The document does not infer one from the name.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamedObservable {
    pub name: String,
    #[serde(with = "super::wire::optional_signal_unit")]
    pub unit: Option<SignalUnit>,
    pub value: Option<f64>,
}

/// One named quantity sampled once per point of the document's axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamedObservableSeries {
    pub name: String,
    #[serde(with = "super::wire::optional_signal_unit")]
    pub unit: Option<SignalUnit>,
    pub values: Vec<Option<f64>>,
}

//=============================================================================
// Operating point and DC sweep
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperatingPointPayload {
    /// Converged DC observables in their canonical SPICE probe spelling.
    pub observables: Vec<NamedObservable>,
}

impl OperatingPointPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        for observable in &self.observables {
            super::require_name("operating-point observable", &observable.name)?;
            finite_optional("operating-point observable", observable.value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DcSweepPayload {
    /// Authored sweep source or parameter driving the primary axis.
    pub sweep_variable: String,
    /// Per-point converged DC observables.
    pub observables: Vec<NamedObservableSeries>,
}

impl DcSweepPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        super::require_name("DC sweep variable", &self.sweep_variable)?;
        for observable in &self.observables {
            super::require_name("DC sweep observable", &observable.name)?;
            for value in &observable.values {
                finite_optional("DC sweep observable", *value)?;
            }
        }
        Ok(())
    }
}

/// AC carries no data outside its frequency axis and complex signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcPayload {}

//=============================================================================
// Transient
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransientPayload {
    /// Accepted integration interval for each time point. The first entry is
    /// zero; every later entry is the exact timestep that produced its sample.
    pub step_sizes: Vec<f64>,
    /// Typed non-solution device store waveforms, such as a compact-model
    /// internal resistance, in their canonical Xyce store spelling.
    pub store_traces: Vec<NamedObservableSeries>,
    /// XSPICE digital event histories. These have their own event times and
    /// are deliberately not resampled onto the transient axis.
    pub digital_traces: Vec<DigitalEventTrace>,
    /// XSPICE real-valued event histories.
    pub real_traces: Vec<RealEventTrace>,
    /// Identity of each `.FFT` post-process this transient produced. The
    /// spectra themselves are separate documents that name this analysis as
    /// their parent.
    pub fft_children: Vec<FftChildReference>,
    /// Compression certificate, when the published result was decimated.
    pub compression: Option<CompressionReportDocument>,
}

impl TransientPayload {
    pub(super) fn value_count(&self) -> usize {
        let stores = self
            .store_traces
            .iter()
            .map(|trace| trace.values.len())
            .fold(0, usize::saturating_add);
        let digital = self
            .digital_traces
            .iter()
            .map(|trace| trace.points.len())
            .fold(0, usize::saturating_add);
        let real = self
            .real_traces
            .iter()
            .map(|trace| trace.points.len().saturating_mul(2))
            .fold(0, usize::saturating_add);
        self.step_sizes
            .len()
            .saturating_add(stores)
            .saturating_add(digital)
            .saturating_add(real)
    }

    fn validate(&self) -> Result<(), ResultDocumentError> {
        finite_slice("transient step size", &self.step_sizes)?;
        if let Some(first) = self.step_sizes.first()
            && *first != 0.0
        {
            return Err(ResultDocumentError::Malformed {
                location: "transient step sizes",
                detail: "the first accepted sample must have a zero integration interval"
                    .to_owned(),
            });
        }
        if self.step_sizes.iter().skip(1).any(|step| *step <= 0.0) {
            return Err(ResultDocumentError::Malformed {
                location: "transient step sizes",
                detail: "every accepted integration interval after the first must be positive"
                    .to_owned(),
            });
        }
        for trace in &self.store_traces {
            super::require_name("transient store trace", &trace.name)?;
            for value in &trace.values {
                finite_optional("transient store trace", *value)?;
            }
        }
        for trace in &self.digital_traces {
            super::require_name("digital trace node", &trace.node_name)?;
            for point in &trace.points {
                finite("digital event time", point.time)?;
            }
        }
        for trace in &self.real_traces {
            super::require_name("real event trace node", &trace.node_name)?;
            for point in &trace.points {
                finite("real event time", point.time)?;
                finite("real event value", point.value)?;
            }
        }
        if let Some(report) = &self.compression {
            report.validate()?;
        }
        Ok(())
    }
}

/// Reference from a transient result to one `.FFT` document it produced.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FftChildReference {
    #[serde(with = "super::wire::analysis_instance_id")]
    pub analysis: AnalysisInstanceId,
    /// Display spelling of the resolved scalar column the spectrum was taken
    /// from, so a reader can pair children with sources without loading them.
    pub output_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DigitalEventTrace {
    pub node_name: String,
    pub points: Vec<DigitalEventPoint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DigitalEventPoint {
    pub time: f64,
    pub state: DigitalStateTag,
    pub strength: DigitalStrengthTag,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RealEventTrace {
    pub node_name: String,
    pub points: Vec<RealEventPoint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RealEventPoint {
    pub time: f64,
    pub value: f64,
}

/// Wire spelling of one XSPICE digital logic state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigitalStateTag {
    Zero,
    One,
    Unknown,
    ZeroResistive,
    OneResistive,
    UnknownResistive,
    ZeroHighZ,
    OneHighZ,
    UnknownHighZ,
    HighZ,
}

impl From<DigitalState> for DigitalStateTag {
    fn from(state: DigitalState) -> Self {
        match state {
            DigitalState::Zero => Self::Zero,
            DigitalState::One => Self::One,
            DigitalState::Unknown => Self::Unknown,
            DigitalState::ZeroR => Self::ZeroResistive,
            DigitalState::OneR => Self::OneResistive,
            DigitalState::UnknownR => Self::UnknownResistive,
            DigitalState::ZeroZ => Self::ZeroHighZ,
            DigitalState::OneZ => Self::OneHighZ,
            DigitalState::UnknownZ => Self::UnknownHighZ,
            DigitalState::HighZ => Self::HighZ,
        }
    }
}

impl From<DigitalStateTag> for DigitalState {
    fn from(state: DigitalStateTag) -> Self {
        match state {
            DigitalStateTag::Zero => Self::Zero,
            DigitalStateTag::One => Self::One,
            DigitalStateTag::Unknown => Self::Unknown,
            DigitalStateTag::ZeroResistive => Self::ZeroR,
            DigitalStateTag::OneResistive => Self::OneR,
            DigitalStateTag::UnknownResistive => Self::UnknownR,
            DigitalStateTag::ZeroHighZ => Self::ZeroZ,
            DigitalStateTag::OneHighZ => Self::OneZ,
            DigitalStateTag::UnknownHighZ => Self::UnknownZ,
            DigitalStateTag::HighZ => Self::HighZ,
        }
    }
}

/// Wire spelling of one XSPICE digital drive strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigitalStrengthTag {
    Undetermined,
    HighZ,
    Resistive,
    Strong,
}

impl From<DigitalStrength> for DigitalStrengthTag {
    fn from(strength: DigitalStrength) -> Self {
        match strength {
            DigitalStrength::Undetermined => Self::Undetermined,
            DigitalStrength::HighZ => Self::HighZ,
            DigitalStrength::Resistive => Self::Resistive,
            DigitalStrength::Strong => Self::Strong,
        }
    }
}

impl From<DigitalStrengthTag> for DigitalStrength {
    fn from(strength: DigitalStrengthTag) -> Self {
        match strength {
            DigitalStrengthTag::Undetermined => Self::Undetermined,
            DigitalStrengthTag::HighZ => Self::HighZ,
            DigitalStrengthTag::Resistive => Self::Resistive,
            DigitalStrengthTag::Strong => Self::Strong,
        }
    }
}

//=============================================================================
// Transient compression certificate
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompressionReportDocument {
    pub report_version: u32,
    pub algorithm: CompressionAlgorithmTag,
    pub sample_domain: CompressionSampleDomainTag,
    pub policy: CompressionPolicyDocument,
    pub input_points: usize,
    pub retained_points: usize,
    /// `None` means no input sample was approximated, never that the error was
    /// not measured.
    pub worst_observed: Option<CompressionObservationDocument>,
}

impl CompressionReportDocument {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        finite(
            "compression absolute tolerance",
            self.policy.absolute_tolerance,
        )?;
        finite(
            "compression relative tolerance",
            self.policy.relative_tolerance,
        )?;
        finite(
            "compression maximum retained interval",
            self.policy.maximum_retained_interval,
        )?;
        if self.retained_points > self.input_points {
            return Err(ResultDocumentError::Malformed {
                location: "transient compression report",
                detail: "retained points cannot exceed input points".to_owned(),
            });
        }
        if let Some(observation) = &self.worst_observed {
            super::require_name(
                "compression observation signal",
                &observation.signal_canonical_name,
            )?;
            finite("compression observation time", observation.time)?;
            finite("compression observation value", observation.actual_value)?;
            finite("compression absolute error", observation.absolute_error)?;
            finite_optional("compression relative error", observation.relative_error)?;
            finite(
                "compression allowed tolerance",
                observation.allowed_tolerance,
            )?;
            finite(
                "compression tolerance utilization",
                observation.tolerance_utilization,
            )?;
        }
        Ok(())
    }
}

impl From<&TransientCompressionReport> for CompressionReportDocument {
    fn from(report: &TransientCompressionReport) -> Self {
        Self {
            report_version: report.schema_version,
            algorithm: report.algorithm.into(),
            sample_domain: report.sample_domain.into(),
            policy: CompressionPolicyDocument::from(&report.applied_policy),
            input_points: report.input_points,
            retained_points: report.retained_points,
            worst_observed: report
                .worst_observed
                .as_ref()
                .map(CompressionObservationDocument::from),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompressionPolicyDocument {
    pub enabled: bool,
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
    pub maximum_retained_interval: f64,
}

impl From<&TransientCompressionPolicy> for CompressionPolicyDocument {
    fn from(policy: &TransientCompressionPolicy) -> Self {
        Self {
            enabled: policy.enabled,
            absolute_tolerance: policy.absolute_tolerance,
            relative_tolerance: policy.relative_tolerance,
            maximum_retained_interval: policy.maximum_retained_interval,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompressionObservationDocument {
    pub signal_kind: CompressionSignalKindTag,
    pub signal_canonical_name: String,
    pub input_sample_index: usize,
    pub time: f64,
    pub actual_value: f64,
    pub absolute_error: f64,
    pub relative_error: Option<f64>,
    pub allowed_tolerance: f64,
    pub tolerance_utilization: f64,
}

impl From<&TransientCompressionErrorObservation> for CompressionObservationDocument {
    fn from(observation: &TransientCompressionErrorObservation) -> Self {
        Self {
            signal_kind: observation.signal.kind.into(),
            signal_canonical_name: observation.signal.canonical_name.clone(),
            input_sample_index: observation.input_sample_index,
            time: observation.time,
            actual_value: observation.actual_value,
            absolute_error: observation.absolute_error,
            relative_error: observation.relative_error,
            allowed_tolerance: observation.allowed_tolerance,
            tolerance_utilization: observation.tolerance_utilization,
        }
    }
}

impl CompressionObservationDocument {
    /// Rebuild the core observation identity this document projected.
    pub fn to_signal(&self) -> Result<TransientCompressionSignal, String> {
        TransientCompressionSignal::new(self.signal_kind.into(), self.signal_canonical_name.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompressionAlgorithmTag {
    MultiChannelRdpLinearV1,
}

impl From<TransientCompressionAlgorithm> for CompressionAlgorithmTag {
    fn from(algorithm: TransientCompressionAlgorithm) -> Self {
        match algorithm {
            TransientCompressionAlgorithm::MultiChannelRdpLinearV1 => Self::MultiChannelRdpLinearV1,
        }
    }
}

impl From<CompressionAlgorithmTag> for TransientCompressionAlgorithm {
    fn from(algorithm: CompressionAlgorithmTag) -> Self {
        match algorithm {
            CompressionAlgorithmTag::MultiChannelRdpLinearV1 => Self::MultiChannelRdpLinearV1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompressionSampleDomainTag {
    AcceptedInputSamples,
}

impl From<TransientCompressionSampleDomain> for CompressionSampleDomainTag {
    fn from(domain: TransientCompressionSampleDomain) -> Self {
        match domain {
            TransientCompressionSampleDomain::AcceptedInputSamples => Self::AcceptedInputSamples,
        }
    }
}

impl From<CompressionSampleDomainTag> for TransientCompressionSampleDomain {
    fn from(domain: CompressionSampleDomainTag) -> Self {
        match domain {
            CompressionSampleDomainTag::AcceptedInputSamples => Self::AcceptedInputSamples,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompressionSignalKindTag {
    Voltage,
    BranchCurrent,
    DeviceObservable,
    DeviceStore,
}

impl From<TransientCompressionSignalKind> for CompressionSignalKindTag {
    fn from(kind: TransientCompressionSignalKind) -> Self {
        match kind {
            TransientCompressionSignalKind::Voltage => Self::Voltage,
            TransientCompressionSignalKind::BranchCurrent => Self::BranchCurrent,
            TransientCompressionSignalKind::DeviceObservable => Self::DeviceObservable,
            TransientCompressionSignalKind::DeviceStore => Self::DeviceStore,
        }
    }
}

impl From<CompressionSignalKindTag> for TransientCompressionSignalKind {
    fn from(kind: CompressionSignalKindTag) -> Self {
        match kind {
            CompressionSignalKindTag::Voltage => Self::Voltage,
            CompressionSignalKindTag::BranchCurrent => Self::BranchCurrent,
            CompressionSignalKindTag::DeviceObservable => Self::DeviceObservable,
            CompressionSignalKindTag::DeviceStore => Self::DeviceStore,
        }
    }
}

//=============================================================================
// Noise
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoisePayload {
    /// Every mechanism identity the device models export, independent of
    /// whether it is active at this bias.
    pub contribution_catalog: Vec<NoiseSourceIdentityDocument>,
    /// Instances whose model was compiled without noise schedules. They are
    /// distinguishable from devices the deck does not contain.
    pub mechanisms_unavailable: Vec<String>,
    /// Per-mechanism contribution spectra aligned with the frequency axis.
    pub contributions: Vec<NoiseContributionSeries>,
}

impl NoisePayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        for identity in &self.contribution_catalog {
            super::require_name("noise catalog device", &identity.device)?;
        }
        for name in &self.mechanisms_unavailable {
            super::require_name("noise mechanism-unavailable device", name)?;
        }
        for contribution in &self.contributions {
            super::require_name("noise contribution device", &contribution.identity.device)?;
            for value in contribution
                .output_contribution
                .iter()
                .chain(&contribution.input_contribution)
                .chain(&contribution.percentage)
            {
                finite_optional("noise contribution", *value)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoiseSourceIdentityDocument {
    pub device: String,
    /// `None` denotes a model that exposes only a whole-device contribution.
    pub mechanism: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoiseContributionSeries {
    pub identity: NoiseSourceIdentityDocument,
    pub mechanism_kind: NoiseMechanismTag,
    /// Output-referred spectral density in V^2/Hz at each frequency.
    pub output_contribution: Vec<Option<f64>>,
    /// Input-referred spectral density in V^2/Hz at each frequency.
    pub input_contribution: Vec<Option<f64>>,
    /// Share of the total output noise, in percent, at each frequency.
    pub percentage: Vec<Option<f64>>,
}

impl NoiseContributionSeries {
    fn value_count(&self) -> usize {
        self.output_contribution
            .len()
            .saturating_add(self.input_contribution.len())
            .saturating_add(self.percentage.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NoiseMechanismTag {
    Thermal,
    Shot,
    Flicker,
    Burst,
    White,
    Table,
    Bsim4Flicker,
    Bsim3Flicker,
    Bsim4CorrelatedThermal,
}

impl From<NoiseSourceType> for NoiseMechanismTag {
    fn from(kind: NoiseSourceType) -> Self {
        match kind {
            NoiseSourceType::Thermal => Self::Thermal,
            NoiseSourceType::Shot => Self::Shot,
            NoiseSourceType::Flicker => Self::Flicker,
            NoiseSourceType::Burst => Self::Burst,
            NoiseSourceType::White => Self::White,
            NoiseSourceType::Table => Self::Table,
            NoiseSourceType::Bsim4Flicker => Self::Bsim4Flicker,
            NoiseSourceType::Bsim3Flicker => Self::Bsim3Flicker,
            NoiseSourceType::Bsim4CorrelatedThermal => Self::Bsim4CorrelatedThermal,
        }
    }
}

impl From<NoiseMechanismTag> for NoiseSourceType {
    fn from(kind: NoiseMechanismTag) -> Self {
        match kind {
            NoiseMechanismTag::Thermal => Self::Thermal,
            NoiseMechanismTag::Shot => Self::Shot,
            NoiseMechanismTag::Flicker => Self::Flicker,
            NoiseMechanismTag::Burst => Self::Burst,
            NoiseMechanismTag::White => Self::White,
            NoiseMechanismTag::Table => Self::Table,
            NoiseMechanismTag::Bsim4Flicker => Self::Bsim4Flicker,
            NoiseMechanismTag::Bsim3Flicker => Self::Bsim3Flicker,
            NoiseMechanismTag::Bsim4CorrelatedThermal => Self::Bsim4CorrelatedThermal,
        }
    }
}

//=============================================================================
// S-parameters and port noise
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SParameterPayload {
    /// Analysis reference impedance in ohms.
    pub reference_impedance: f64,
    pub ports: Vec<PortDocument>,
    /// Angular frequency in rad/s at each analyzed point, exactly as the
    /// S-matrix retained it.
    pub angular_frequencies: Vec<f64>,
}

impl SParameterPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        finite("S-parameter reference impedance", self.reference_impedance)?;
        finite_slice("S-parameter angular frequency", &self.angular_frequencies)?;
        for port in &self.ports {
            super::require_name("S-parameter port node", &port.node_positive)?;
            super::require_name("S-parameter port reference node", &port.node_negative)?;
            finite("S-parameter port impedance", port.reference_impedance)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortDocument {
    /// One-based port number.
    pub number: usize,
    pub node_positive: String,
    pub node_negative: String,
    pub reference_impedance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortNoisePayload {
    /// Dimension of the Hermitian current-noise covariance matrix.
    pub port_count: usize,
}

//=============================================================================
// Distortion
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DistortionPayload {
    /// `None` for harmonic mode; otherwise the fixed `F2/F1` ratio.
    pub f2_over_f1: Option<f64>,
    /// One entry per computed Volterra product, in authored order.
    pub products: Vec<DistortionProductSeries>,
}

impl DistortionPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        finite_optional("distortion F2/F1 ratio", self.f2_over_f1)?;
        for product in &self.products {
            finite_slice("distortion product frequency", &product.frequencies)?;
            if product.order != product.product.order() {
                return Err(ResultDocumentError::Malformed {
                    location: "distortion product",
                    detail: format!(
                        "product {} has Volterra order {}",
                        product.product.label(),
                        product.product.order()
                    ),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DistortionProductSeries {
    pub product: DistortionProductTag,
    /// Volterra order, redundant with `product` and checked on decode.
    pub order: usize,
    /// Physical product frequency at each swept F1 point.
    pub frequencies: Vec<f64>,
}

/// Wire spelling of one `.DISTO` spectral product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DistortionProductTag {
    /// `2*F1`.
    SecondHarmonic,
    /// `3*F1`.
    ThirdHarmonic,
    /// `F1+F2`.
    Sum,
    /// `F1-F2`.
    Difference,
    /// `2*F1-F2`.
    ThirdOrderDifference,
}

impl DistortionProductTag {
    /// Stable SPICE-oriented product label (`2f1`, `3f1`, `f1+f2`, `f1-f2`,
    /// `2f1-f2`).
    pub const fn label(self) -> &'static str {
        self.to_core().label()
    }

    /// Volterra order of this product.
    pub const fn order(self) -> usize {
        self.to_core().order()
    }

    /// The core spectral-product identity this tag names.
    pub const fn to_core(self) -> DistortionProduct {
        match self {
            Self::SecondHarmonic => DistortionProduct::SecondHarmonic,
            Self::ThirdHarmonic => DistortionProduct::ThirdHarmonic,
            Self::Sum => DistortionProduct::Sum,
            Self::Difference => DistortionProduct::Difference,
            Self::ThirdOrderDifference => DistortionProduct::ThirdOrderDifference,
        }
    }
}

impl From<DistortionProduct> for DistortionProductTag {
    fn from(product: DistortionProduct) -> Self {
        match product {
            DistortionProduct::SecondHarmonic => Self::SecondHarmonic,
            DistortionProduct::ThirdHarmonic => Self::ThirdHarmonic,
            DistortionProduct::Sum => Self::Sum,
            DistortionProduct::Difference => Self::Difference,
            DistortionProduct::ThirdOrderDifference => Self::ThirdOrderDifference,
        }
    }
}

impl From<DistortionProductTag> for DistortionProduct {
    fn from(product: DistortionProductTag) -> Self {
        product.to_core()
    }
}

/// Which first-order tone a distortion fundamental response belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DistortionTone {
    F1,
    F2,
}

//=============================================================================
// Transfer function, stability, sensitivity, pole-zero
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransferFunctionPayload {
    /// Authored output variable, for example `V(out)`.
    pub output: String,
    /// Authored input source, for example `Vin`.
    pub input: String,
}

impl TransferFunctionPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        super::require_name("transfer-function output", &self.output)?;
        super::require_name("transfer-function input", &self.input)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StabilityPayload {
    /// Whether the loop-gain projection completed.
    pub success: bool,
    pub warnings: Vec<String>,
    /// Nyquist contour, retained separately because it is optional and carries
    /// its own frequency for each sample.
    pub nyquist: Vec<NyquistSample>,
}

impl StabilityPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        for sample in &self.nyquist {
            finite("Nyquist frequency", sample.frequency)?;
            finite("Nyquist real part", sample.real)?;
            finite("Nyquist imaginary part", sample.imaginary)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NyquistSample {
    pub frequency: f64,
    pub real: f64,
    pub imaginary: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SensitivityPayload {
    /// Authored output variable.
    pub output: String,
    pub entries: Vec<SensitivityEntry>,
}

impl SensitivityPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        super::require_name("sensitivity output", &self.output)?;
        for entry in &self.entries {
            super::require_name("sensitivity vector name", &entry.vector_name)?;
            super::require_name("sensitivity element", &entry.element)?;
            finite("sensitivity nominal value", entry.nominal_value)?;
            finite("absolute sensitivity", entry.absolute)?;
            finite("normalized sensitivity", entry.normalized)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SensitivityEntry {
    /// Stable SPICE-compatible vector name (`R1`, `M1_W`, `MOD:VTO`).
    pub vector_name: String,
    pub element: String,
    pub element_kind: SensitivityElementTag,
    pub parameter: String,
    pub nominal_value: f64,
    /// `d(output)/d(parameter)`.
    pub absolute: f64,
    /// `(parameter/output) * d(output)/d(parameter)`.
    pub normalized: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SensitivityElementTag {
    Resistor,
    Capacitor,
    Inductor,
    VoltageSource,
    CurrentSource,
    Transconductance,
    Transresistance,
    Diode,
    Bjt,
    Mosfet,
    Jfet,
    Mesfet,
    BehavioralSource,
    Switch,
    TransmissionLine,
    Coupling,
    Xspice,
    Model,
    Other,
}

impl From<ElementType> for SensitivityElementTag {
    fn from(kind: ElementType) -> Self {
        match kind {
            ElementType::Resistor => Self::Resistor,
            ElementType::Capacitor => Self::Capacitor,
            ElementType::Inductor => Self::Inductor,
            ElementType::VoltageSource => Self::VoltageSource,
            ElementType::CurrentSource => Self::CurrentSource,
            ElementType::Transconductance => Self::Transconductance,
            ElementType::Transresistance => Self::Transresistance,
            ElementType::Diode => Self::Diode,
            ElementType::Bjt => Self::Bjt,
            ElementType::Mosfet => Self::Mosfet,
            ElementType::Jfet => Self::Jfet,
            ElementType::Mesfet => Self::Mesfet,
            ElementType::BehavioralSource => Self::BehavioralSource,
            ElementType::Switch => Self::Switch,
            ElementType::TransmissionLine => Self::TransmissionLine,
            ElementType::Coupling => Self::Coupling,
            ElementType::Xspice => Self::Xspice,
            ElementType::Model => Self::Model,
            ElementType::Other => Self::Other,
        }
    }
}

impl From<SensitivityElementTag> for ElementType {
    fn from(kind: SensitivityElementTag) -> Self {
        match kind {
            SensitivityElementTag::Resistor => Self::Resistor,
            SensitivityElementTag::Capacitor => Self::Capacitor,
            SensitivityElementTag::Inductor => Self::Inductor,
            SensitivityElementTag::VoltageSource => Self::VoltageSource,
            SensitivityElementTag::CurrentSource => Self::CurrentSource,
            SensitivityElementTag::Transconductance => Self::Transconductance,
            SensitivityElementTag::Transresistance => Self::Transresistance,
            SensitivityElementTag::Diode => Self::Diode,
            SensitivityElementTag::Bjt => Self::Bjt,
            SensitivityElementTag::Mosfet => Self::Mosfet,
            SensitivityElementTag::Jfet => Self::Jfet,
            SensitivityElementTag::Mesfet => Self::Mesfet,
            SensitivityElementTag::BehavioralSource => Self::BehavioralSource,
            SensitivityElementTag::Switch => Self::Switch,
            SensitivityElementTag::TransmissionLine => Self::TransmissionLine,
            SensitivityElementTag::Coupling => Self::Coupling,
            SensitivityElementTag::Xspice => Self::Xspice,
            SensitivityElementTag::Model => Self::Model,
            SensitivityElementTag::Other => Self::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PoleZeroPayload {
    pub input: String,
    pub output: String,
    pub poles: Vec<ComplexSample>,
    pub zeros: Vec<ComplexSample>,
    pub pole_evidence: RootSetEvidenceDocument,
    pub zero_evidence: RootSetEvidenceDocument,
    /// `H(0)`, when the transfer function has a finite DC value.
    pub dc_gain: Option<f64>,
    /// `H(inf)`, when it is finite.
    pub high_frequency_gain: Option<f64>,
}

impl PoleZeroPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        super::require_name("pole-zero input", &self.input)?;
        super::require_name("pole-zero output", &self.output)?;
        for root in self.poles.iter().chain(&self.zeros) {
            finite("pole-zero root real part", root.real)?;
            finite("pole-zero root imaginary part", root.imaginary)?;
        }
        finite_optional("pole-zero DC gain", self.dc_gain)?;
        finite_optional("pole-zero high-frequency gain", self.high_frequency_gain)?;
        self.pole_evidence.validate("pole")?;
        self.zero_evidence.validate("zero")?;
        let poles = self.to_pole_evidence();
        let zeros = self.to_zero_evidence();
        let pole_roots = complex_roots(&self.poles);
        let zero_roots = complex_roots(&self.zeros);
        if !poles.is_consistent_with(&pole_roots) || !zeros.is_consistent_with(&zero_roots) {
            return Err(ResultDocumentError::Malformed {
                location: "pole-zero evidence",
                detail: "root-set evidence disagrees with the retained root vector".to_owned(),
            });
        }
        Ok(())
    }

    /// Rebuild the core evidence for [`Self::poles`].
    pub fn to_pole_evidence(&self) -> RootSetEvidence {
        self.pole_evidence.to_core()
    }

    /// Rebuild the core evidence for [`Self::zeros`].
    pub fn to_zero_evidence(&self) -> RootSetEvidence {
        self.zero_evidence.to_core()
    }
}

fn complex_roots(samples: &[ComplexSample]) -> Vec<num_complex::Complex64> {
    samples
        .iter()
        .map(|sample| num_complex::Complex64::new(sample.real, sample.imaginary))
        .collect()
}

/// Completeness and numerical evidence for a retained root vector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "evidence", rename_all = "kebab-case")]
pub enum RootSetEvidenceDocument {
    /// The quantity was not requested by the analysis configuration.
    NotRequested,
    /// The calculation proved there are no finite roots.
    QualifiedEmpty {
        certificate: SpectrumCertificateDocument,
    },
    /// Every root belongs to a complete, strictly qualified spectrum.
    Qualified {
        certificate: SpectrumCertificateDocument,
    },
    /// Complete and usable, but the residual exceeds the strict threshold.
    Approximate {
        certificate: SpectrumCertificateDocument,
    },
    /// Roots restored from an older result that carried no evidence.
    LegacyUnknown,
}

impl RootSetEvidenceDocument {
    fn validate(&self, role: &'static str) -> Result<(), ResultDocumentError> {
        let certificate = match self {
            Self::NotRequested | Self::LegacyUnknown => return Ok(()),
            Self::QualifiedEmpty { certificate }
            | Self::Qualified { certificate }
            | Self::Approximate { certificate } => certificate,
        };
        finite("root-set backward error", certificate.max_backward_error)?;
        finite(
            "root-set qualification tolerance",
            certificate.qualification_tolerance,
        )?;
        if certificate.to_core().is_none() {
            return Err(ResultDocumentError::Malformed {
                location: "root-set certificate",
                detail: format!("{role} spectrum certificate fails its own validity contract"),
            });
        }
        Ok(())
    }

    fn to_core(&self) -> RootSetEvidence {
        match self {
            Self::NotRequested => RootSetEvidence::NotRequested,
            Self::LegacyUnknown => RootSetEvidence::LegacyUnknown,
            Self::QualifiedEmpty { certificate } => certificate
                .to_core()
                .map_or(RootSetEvidence::LegacyUnknown, |certificate| {
                    RootSetEvidence::QualifiedEmpty { certificate }
                }),
            Self::Qualified { certificate } => certificate
                .to_core()
                .map_or(RootSetEvidence::LegacyUnknown, |certificate| {
                    RootSetEvidence::Qualified { certificate }
                }),
            Self::Approximate { certificate } => certificate
                .to_core()
                .map_or(RootSetEvidence::LegacyUnknown, |certificate| {
                    RootSetEvidence::Approximate { certificate }
                }),
        }
    }
}

impl From<&RootSetEvidence> for RootSetEvidenceDocument {
    fn from(evidence: &RootSetEvidence) -> Self {
        match evidence {
            RootSetEvidence::NotRequested => Self::NotRequested,
            RootSetEvidence::LegacyUnknown => Self::LegacyUnknown,
            RootSetEvidence::QualifiedEmpty { certificate } => Self::QualifiedEmpty {
                certificate: SpectrumCertificateDocument::from(*certificate),
            },
            RootSetEvidence::Qualified { certificate } => Self::Qualified {
                certificate: SpectrumCertificateDocument::from(*certificate),
            },
            RootSetEvidence::Approximate { certificate } => Self::Approximate {
                certificate: SpectrumCertificateDocument::from(*certificate),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpectrumCertificateDocument {
    pub problem_order: usize,
    pub infinite_count: usize,
    pub max_backward_error: f64,
    pub qualification_tolerance: f64,
}

impl SpectrumCertificateDocument {
    /// Rebuild the core certificate, or `None` when the retained numbers do
    /// not satisfy its validity contract.
    pub fn to_core(self) -> Option<SpectrumCertificate> {
        SpectrumCertificate::new(
            self.problem_order,
            self.infinite_count,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }
}

impl From<SpectrumCertificate> for SpectrumCertificateDocument {
    fn from(certificate: SpectrumCertificate) -> Self {
        Self {
            problem_order: certificate.problem_order,
            infinite_count: certificate.infinite_count,
            max_backward_error: certificate.max_backward_error,
            qualification_tolerance: certificate.qualification_tolerance,
        }
    }
}

//=============================================================================
// Fourier and FFT
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FourierPayload {
    /// Authored output the harmonic series was taken from.
    pub output: String,
}

impl FourierPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        super::require_name("Fourier output", &self.output)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FftPayload {
    /// Authored probe or braced expression.
    pub source: FftSourceDocument,
    /// Display spelling of the resolved scalar column.
    pub output_name: String,
    /// Physical quantity class declared by the transient post-process.
    pub physical_type: String,
    /// Inclusive beginning of the sampled record, in seconds.
    pub start_time: f64,
    /// Exclusive end of the sampled record, in seconds.
    pub stop_time: f64,
    /// Uniform sample spacing in seconds.
    pub sample_interval: f64,
    /// Number of uniformly resampled real input points.
    pub sample_count: usize,
    /// Whether the solver was forced onto every sample time.
    pub accurate_sampling: bool,
    pub coefficient_format: FftCoefficientFormatTag,
    pub compatibility_mode: FftCompatibilityModeTag,
    pub window: FftWindowTag,
    /// Canonical source spelling of the window retained by the parser.
    pub window_name: String,
    /// HSPICE-compatible `ALFA` value.
    pub alpha: f64,
    /// Mean window coefficient used for coherent-gain compensation.
    pub coherent_gain: f64,
    /// DFT bin width in hertz.
    pub frequency_resolution: f64,
    pub fundamental_bin: usize,
    pub minimum_metric_bin: usize,
    pub maximum_metric_bin: usize,
    /// Xyce `FFTOUT=1` figures and the ranked harmonic list.
    pub metrics: Option<FftMetricsDocument>,
}

impl FftPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        super::require_name("FFT output name", &self.output_name)?;
        super::require_name("FFT physical type", &self.physical_type)?;
        super::require_name("FFT window name", &self.window_name)?;
        finite("FFT start time", self.start_time)?;
        finite("FFT stop time", self.stop_time)?;
        finite("FFT sample interval", self.sample_interval)?;
        finite("FFT alpha", self.alpha)?;
        finite("FFT coherent gain", self.coherent_gain)?;
        finite("FFT frequency resolution", self.frequency_resolution)?;
        if self.stop_time <= self.start_time {
            return Err(ResultDocumentError::Malformed {
                location: "FFT record window",
                detail: "the sampled record must end after it starts".to_owned(),
            });
        }
        if self.minimum_metric_bin > self.maximum_metric_bin {
            return Err(ResultDocumentError::Malformed {
                location: "FFT metric bins",
                detail: "the lower metric bin cannot exceed the upper metric bin".to_owned(),
            });
        }
        if let Some(metrics) = &self.metrics {
            metrics.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum FftSourceDocument {
    /// A canonical probe such as `V(OUT)` or `I(V1)`.
    Probe { text: String },
    /// A braced expression evaluated at each transient sample.
    Expression { text: String },
}

impl From<&FftOutput> for FftSourceDocument {
    fn from(output: &FftOutput) -> Self {
        match output {
            FftOutput::Probe(text) => Self::Probe { text: text.clone() },
            FftOutput::Expression(text) => Self::Expression { text: text.clone() },
        }
    }
}

impl From<FftSourceDocument> for FftOutput {
    fn from(source: FftSourceDocument) -> Self {
        match source {
            FftSourceDocument::Probe { text } => Self::Probe(text),
            FftSourceDocument::Expression { text } => Self::Expression(text),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FftCoefficientFormatTag {
    Normalized,
    Unnormalized,
}

impl From<FftFormat> for FftCoefficientFormatTag {
    fn from(format: FftFormat) -> Self {
        match format {
            FftFormat::Normalized => Self::Normalized,
            FftFormat::Unnormalized => Self::Unnormalized,
        }
    }
}

impl From<FftCoefficientFormatTag> for FftFormat {
    fn from(format: FftCoefficientFormatTag) -> Self {
        match format {
            FftCoefficientFormatTag::Normalized => Self::Normalized,
            FftCoefficientFormatTag::Unnormalized => Self::Unnormalized,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FftCompatibilityModeTag {
    HspiceCompatible,
    SpectreCompatible,
}

impl From<XyceFftMode> for FftCompatibilityModeTag {
    fn from(mode: XyceFftMode) -> Self {
        match mode {
            XyceFftMode::HspiceCompatible => Self::HspiceCompatible,
            XyceFftMode::SpectreCompatible => Self::SpectreCompatible,
        }
    }
}

impl From<FftCompatibilityModeTag> for XyceFftMode {
    fn from(mode: FftCompatibilityModeTag) -> Self {
        match mode {
            FftCompatibilityModeTag::HspiceCompatible => Self::HspiceCompatible,
            FftCompatibilityModeTag::SpectreCompatible => Self::SpectreCompatible,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FftWindowTag {
    Rectangular,
    Bartlett,
    BartlettHann,
    Hamming,
    Hann,
    Blackman67Db,
    Blackman,
    BlackmanHarris,
    Nuttall,
    HalfCycleSine,
    HalfCycleSine3,
    HalfCycleSine6,
    Cosine2,
    Cosine4,
}

impl From<FftWindow> for FftWindowTag {
    fn from(window: FftWindow) -> Self {
        match window {
            FftWindow::Rectangular => Self::Rectangular,
            FftWindow::Bartlett => Self::Bartlett,
            FftWindow::BartlettHann => Self::BartlettHann,
            FftWindow::Hamming => Self::Hamming,
            FftWindow::Hann => Self::Hann,
            FftWindow::Blackman67Db => Self::Blackman67Db,
            FftWindow::Blackman => Self::Blackman,
            FftWindow::BlackmanHarris => Self::BlackmanHarris,
            FftWindow::Nuttall => Self::Nuttall,
            FftWindow::HalfCycleSine => Self::HalfCycleSine,
            FftWindow::HalfCycleSine3 => Self::HalfCycleSine3,
            FftWindow::HalfCycleSine6 => Self::HalfCycleSine6,
            FftWindow::Cosine2 => Self::Cosine2,
            FftWindow::Cosine4 => Self::Cosine4,
        }
    }
}

impl From<FftWindowTag> for FftWindow {
    fn from(window: FftWindowTag) -> Self {
        match window {
            FftWindowTag::Rectangular => Self::Rectangular,
            FftWindowTag::Bartlett => Self::Bartlett,
            FftWindowTag::BartlettHann => Self::BartlettHann,
            FftWindowTag::Hamming => Self::Hamming,
            FftWindowTag::Hann => Self::Hann,
            FftWindowTag::Blackman67Db => Self::Blackman67Db,
            FftWindowTag::Blackman => Self::Blackman,
            FftWindowTag::BlackmanHarris => Self::BlackmanHarris,
            FftWindowTag::Nuttall => Self::Nuttall,
            FftWindowTag::HalfCycleSine => Self::HalfCycleSine,
            FftWindowTag::HalfCycleSine3 => Self::HalfCycleSine3,
            FftWindowTag::HalfCycleSine6 => Self::HalfCycleSine6,
            FftWindowTag::Cosine2 => Self::Cosine2,
            FftWindowTag::Cosine4 => Self::Cosine4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FftMetricsDocument {
    pub fundamental_magnitude: f64,
    pub thd_ratio: f64,
    pub thd_db: f64,
    pub sndr_db: f64,
    pub enob_bits: f64,
    pub snr_db: f64,
    pub sfdr_db: f64,
    pub sfdr_spur_bin: Option<usize>,
    pub sfdr_spur_frequency: Option<f64>,
    /// Non-DC bins ranked by descending magnitude. The list is deliberately
    /// ragged: it is shorter than the spectrum and shorter still for narrow
    /// records.
    pub largest_harmonics: Vec<FftHarmonicDocument>,
}

impl FftMetricsDocument {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        finite("FFT fundamental magnitude", self.fundamental_magnitude)?;
        finite("FFT THD ratio", self.thd_ratio)?;
        finite("FFT THD dB", self.thd_db)?;
        finite("FFT SNDR dB", self.sndr_db)?;
        finite("FFT ENOB", self.enob_bits)?;
        finite("FFT SNR dB", self.snr_db)?;
        finite("FFT SFDR dB", self.sfdr_db)?;
        finite_optional("FFT SFDR spur frequency", self.sfdr_spur_frequency)?;
        if self.sfdr_spur_bin.is_some() != self.sfdr_spur_frequency.is_some() {
            return Err(ResultDocumentError::Malformed {
                location: "FFT metrics",
                detail: "an SFDR spur must carry both its bin and its frequency".to_owned(),
            });
        }
        for (index, harmonic) in self.largest_harmonics.iter().enumerate() {
            if harmonic.rank != index + 1 {
                return Err(ResultDocumentError::Malformed {
                    location: "FFT ranked harmonics",
                    detail: "ranked harmonics must be one-based and contiguous".to_owned(),
                });
            }
            finite("FFT harmonic frequency", harmonic.frequency)?;
            finite("FFT harmonic magnitude", harmonic.magnitude)?;
            finite("FFT harmonic magnitude dB", harmonic.magnitude_db)?;
            finite("FFT harmonic phase", harmonic.phase_degrees)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FftHarmonicDocument {
    /// One-based position in descending-magnitude order.
    pub rank: usize,
    /// DFT bin index.
    pub bin: usize,
    pub frequency: f64,
    pub magnitude: f64,
    pub magnitude_db: f64,
    pub phase_degrees: f64,
}

//=============================================================================
// Monte Carlo
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MonteCarloPayload {
    /// Per-variable statistics, ordered by canonical variable name so the
    /// document is independent of the producing hash map's iteration order.
    pub statistics: Vec<MonteCarloVariableStatistics>,
}

impl MonteCarloPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        for statistics in &self.statistics {
            super::require_name("Monte Carlo variable", &statistics.name)?;
            for sample in &statistics.samples {
                finite_optional("Monte Carlo sample", *sample)?;
            }
            finite_optional("Monte Carlo mean", statistics.mean)?;
            finite_optional(
                "Monte Carlo standard deviation",
                statistics.standard_deviation,
            )?;
            finite_optional("Monte Carlo minimum", statistics.minimum)?;
            finite_optional("Monte Carlo maximum", statistics.maximum)?;
            finite_slice("Monte Carlo histogram bin edge", &statistics.bin_edges)?;
            if !statistics.histogram.is_empty()
                && statistics.bin_edges.len() != statistics.histogram.len() + 1
            {
                return Err(ResultDocumentError::Malformed {
                    location: "Monte Carlo histogram",
                    detail: "a histogram with n bins needs exactly n+1 edges".to_owned(),
                });
            }
        }
        Ok(())
    }
}

/// Statistics for one Monte Carlo output variable.
///
/// The producing result reports `NaN` for an undefined statistic (no samples,
/// or a non-finite sample). This document represents that as `None`, which is
/// the same information without a value a reader can accidentally use.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MonteCarloVariableStatistics {
    pub name: String,
    /// One sample per completed trial. These stay in the payload because the
    /// producing result declares no unit for an output variable, and the
    /// document does not infer one from a probe name.
    pub samples: Vec<Option<f64>>,
    pub mean: Option<f64>,
    pub standard_deviation: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub histogram: Vec<usize>,
    pub bin_edges: Vec<f64>,
}

impl MonteCarloVariableStatistics {
    fn value_count(&self) -> usize {
        self.samples
            .len()
            .saturating_add(self.bin_edges.len())
            .saturating_add(4)
    }
}

//=============================================================================
// PSS, PAC, PNoise
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PssPayload {
    /// Monodromy eigenvalues used for stability analysis and PNoise.
    pub floquet_multipliers: Vec<ComplexSample>,
    pub floquet_evidence: FloquetEvidenceDocument,
    pub floquet_orbit_kind: FloquetOrbitTag,
    /// Autonomous phase mode, when one was qualified inside the unit-circle
    /// uncertainty band.
    pub trivial_floquet_multiplier_index: Option<usize>,
}

impl PssPayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        for multiplier in &self.floquet_multipliers {
            finite("Floquet multiplier real part", multiplier.real)?;
            finite("Floquet multiplier imaginary part", multiplier.imaginary)?;
        }
        if let FloquetEvidenceDocument::Qualified { certificate } = &self.floquet_evidence {
            finite("Floquet backward error", certificate.max_backward_error)?;
            finite(
                "Floquet qualification tolerance",
                certificate.qualification_tolerance,
            )?;
        }
        if let Some(index) = self.trivial_floquet_multiplier_index
            && index >= self.floquet_multipliers.len()
        {
            return Err(ResultDocumentError::Malformed {
                location: "PSS Floquet spectrum",
                detail: "the selected phase mode is outside the multiplier vector".to_owned(),
            });
        }
        Ok(())
    }
}

/// Provenance for a retained Floquet multiplier vector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "evidence", rename_all = "kebab-case")]
pub enum FloquetEvidenceDocument {
    /// Stability post-processing was not performed.
    NotComputed,
    /// The periodic map has no dynamic state, so it has no multipliers.
    NoDynamicModes,
    /// Every multiplier belongs to a complete, strictly qualified spectrum.
    Qualified {
        certificate: FloquetCertificateDocument,
    },
    /// Multipliers restored without a numerical certificate.
    LegacyUnknown,
}

impl From<&FloquetSpectrumEvidence> for FloquetEvidenceDocument {
    fn from(evidence: &FloquetSpectrumEvidence) -> Self {
        match evidence {
            FloquetSpectrumEvidence::NotComputed => Self::NotComputed,
            FloquetSpectrumEvidence::NoDynamicModes => Self::NoDynamicModes,
            FloquetSpectrumEvidence::LegacyUnknown => Self::LegacyUnknown,
            FloquetSpectrumEvidence::Qualified { certificate } => Self::Qualified {
                certificate: FloquetCertificateDocument {
                    problem_order: certificate.problem_order,
                    max_backward_error: certificate.max_backward_error,
                    qualification_tolerance: certificate.qualification_tolerance,
                },
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FloquetCertificateDocument {
    pub problem_order: usize,
    pub max_backward_error: f64,
    pub qualification_tolerance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FloquetOrbitTag {
    Driven,
    Autonomous,
}

impl From<FloquetOrbitKind> for FloquetOrbitTag {
    fn from(kind: FloquetOrbitKind) -> Self {
        match kind {
            FloquetOrbitKind::Driven => Self::Driven,
            FloquetOrbitKind::Autonomous => Self::Autonomous,
        }
    }
}

impl From<FloquetOrbitTag> for FloquetOrbitKind {
    fn from(kind: FloquetOrbitTag) -> Self {
        match kind {
            FloquetOrbitTag::Driven => Self::Driven,
            FloquetOrbitTag::Autonomous => Self::Autonomous,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PacPayload {
    /// PSS fundamental frequency in hertz.
    pub fundamental_frequency: f64,
    pub sideband_minimum: i32,
    pub sideband_maximum: i32,
    pub input_source: Option<String>,
    pub output_node: Option<String>,
    pub iterations: usize,
    pub residual: f64,
    /// One entry per analyzed sideband, in ascending sideband order.
    pub sidebands: Vec<PacSidebandDescriptor>,
    /// Output conversion matrix. `None` for a PAC run with no output node,
    /// which deliberately retains node spectra only.
    pub conversion_matrix: Option<PacConversionMatrixDocument>,
}

impl PacPayload {
    fn value_count(&self) -> usize {
        let sidebands = self
            .sidebands
            .iter()
            .map(|sideband| sideband.absolute_frequencies.len().saturating_mul(2))
            .fold(0, usize::saturating_add);
        let conversion = self
            .conversion_matrix
            .as_ref()
            .map_or(0, |matrix| matrix.entries.len().saturating_mul(2));
        sidebands.saturating_add(conversion)
    }

    fn validate(&self) -> Result<(), ResultDocumentError> {
        finite("PAC fundamental frequency", self.fundamental_frequency)?;
        finite("PAC residual", self.residual)?;
        if self.sideband_minimum > self.sideband_maximum {
            return Err(ResultDocumentError::Malformed {
                location: "PAC sideband range",
                detail: "the lowest sideband cannot exceed the highest".to_owned(),
            });
        }
        for sideband in &self.sidebands {
            finite_slice("PAC absolute frequency", &sideband.absolute_frequencies)?;
            finite_slice("PAC frequency offset", &sideband.frequency_offsets)?;
            if sideband.absolute_frequencies.len() != sideband.frequency_offsets.len() {
                return Err(ResultDocumentError::SeriesLength {
                    location: "PAC sideband frequency columns".to_owned(),
                    expected: sideband.absolute_frequencies.len(),
                    actual: sideband.frequency_offsets.len(),
                });
            }
            if sideband.sideband < self.sideband_minimum
                || sideband.sideband > self.sideband_maximum
            {
                return Err(ResultDocumentError::Malformed {
                    location: "PAC sideband range",
                    detail: format!(
                        "sideband {} is outside the declared range",
                        sideband.sideband
                    ),
                });
            }
        }
        if let Some(matrix) = &self.conversion_matrix {
            for entry in &matrix.entries {
                finite("PAC conversion element real part", entry.value.real)?;
                finite(
                    "PAC conversion element imaginary part",
                    entry.value.imaginary,
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PacSidebandDescriptor {
    pub sideband: i32,
    /// Offset from the fundamental at each analyzed point, in hertz.
    pub frequency_offsets: Vec<f64>,
    /// `sideband * f0 + offset` at each analyzed point, in hertz.
    pub absolute_frequencies: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PacConversionMatrixDocument {
    pub entries: Vec<PacConversionEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PacConversionEntry {
    pub frequency_index: usize,
    pub output_sideband: i32,
    pub input_sideband: i32,
    pub value: ComplexSample,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PNoisePayload {
    /// Node the phase noise was referred to.
    pub output_node: String,
    /// Integration band used for the jitter figures, in hertz.
    pub jitter_bandwidth: Option<PNoiseBandwidth>,
    pub contributors: Vec<PNoiseContributor>,
}

impl PNoisePayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        super::require_name("PNoise output node", &self.output_node)?;
        if let Some(bandwidth) = self.jitter_bandwidth {
            finite("PNoise jitter bandwidth start", bandwidth.start)?;
            finite("PNoise jitter bandwidth stop", bandwidth.stop)?;
        }
        for contributor in &self.contributors {
            super::require_name("PNoise contributor", &contributor.name)?;
            finite_optional("PNoise contributor percentage", contributor.percentage)?;
            for point in &contributor.contributions {
                finite(
                    "PNoise contributor offset frequency",
                    point.offset_frequency,
                )?;
                finite("PNoise contributor density", point.contribution_dbc_per_hz)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PNoiseBandwidth {
    pub start: f64,
    pub stop: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PNoiseContributor {
    pub name: String,
    pub device_type: String,
    /// The contributor's own offset-frequency grid, which is deliberately not
    /// assumed to match the document axis.
    pub contributions: Vec<PNoiseContribution>,
    pub percentage: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PNoiseContribution {
    pub offset_frequency: f64,
    pub contribution_dbc_per_hz: f64,
}

//=============================================================================
// Harmonic balance and envelope
//=============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HarmonicBalancePayload {
    /// Multi-tone identifiers, when the run was multi-tone.
    pub tones: Vec<String>,
    /// Named reactive-element spectra retained from the periodic state.
    pub reactive_spectra: Vec<HbReactiveSpectrumDocument>,
    /// Reasons a projected state is not a complete physical continuation
    /// state.
    pub continuation_limitations: Vec<HbContinuationLimitationTag>,
}

impl HarmonicBalancePayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        for spectrum in &self.reactive_spectra {
            super::require_name("HB reactive device", &spectrum.device_name)?;
            if spectrum.voltage_coefficients.len() != spectrum.current_coefficients.len() {
                return Err(ResultDocumentError::SeriesLength {
                    location: format!(
                        "HB reactive spectrum '{}' current coefficients",
                        spectrum.device_name
                    ),
                    expected: spectrum.voltage_coefficients.len(),
                    actual: spectrum.current_coefficients.len(),
                });
            }
            for coefficient in spectrum
                .voltage_coefficients
                .iter()
                .chain(&spectrum.current_coefficients)
            {
                finite("HB reactive coefficient real part", coefficient.real)?;
                finite(
                    "HB reactive coefficient imaginary part",
                    coefficient.imaginary,
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HbReactiveSpectrumDocument {
    pub device_name: String,
    pub kind: HbReactiveKindTag,
    /// Positive-to-negative terminal-voltage phasors, `[DC, H1, ..., Hn]`.
    pub voltage_coefficients: Vec<ComplexSample>,
    /// Positive-to-negative branch-current phasors, `[DC, H1, ..., Hn]`.
    pub current_coefficients: Vec<ComplexSample>,
    /// Whether harmonic zero is a physically exact branch current.
    pub dc_current_is_exact: bool,
}

impl HbReactiveSpectrumDocument {
    fn value_count(&self) -> usize {
        self.voltage_coefficients
            .len()
            .saturating_add(self.current_coefficients.len())
            .saturating_mul(2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HbReactiveKindTag {
    Capacitor,
    Inductor,
}

impl From<HbReactiveKind> for HbReactiveKindTag {
    fn from(kind: HbReactiveKind) -> Self {
        match kind {
            HbReactiveKind::Capacitor => Self::Capacitor,
            HbReactiveKind::Inductor => Self::Inductor,
        }
    }
}

impl From<HbReactiveKindTag> for HbReactiveKind {
    fn from(kind: HbReactiveKindTag) -> Self {
        match kind {
            HbReactiveKindTag::Capacitor => Self::Capacitor,
            HbReactiveKindTag::Inductor => Self::Inductor,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HbContinuationLimitationTag {
    NonlinearVoltageSourcesUseNortonEquivalent,
    InductorDcCurrentUsesShortSurrogate,
    VerilogAInternalStateNotRetained,
}

impl From<&HbContinuationLimitation> for HbContinuationLimitationTag {
    fn from(limitation: &HbContinuationLimitation) -> Self {
        match limitation {
            HbContinuationLimitation::NonlinearVoltageSourcesUseNortonEquivalent => {
                Self::NonlinearVoltageSourcesUseNortonEquivalent
            }
            HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate => {
                Self::InductorDcCurrentUsesShortSurrogate
            }
            HbContinuationLimitation::VerilogAInternalStateNotRetained => {
                Self::VerilogAInternalStateNotRetained
            }
        }
    }
}

impl From<HbContinuationLimitationTag> for HbContinuationLimitation {
    fn from(limitation: HbContinuationLimitationTag) -> Self {
        match limitation {
            HbContinuationLimitationTag::NonlinearVoltageSourcesUseNortonEquivalent => {
                Self::NonlinearVoltageSourcesUseNortonEquivalent
            }
            HbContinuationLimitationTag::InductorDcCurrentUsesShortSurrogate => {
                Self::InductorDcCurrentUsesShortSurrogate
            }
            HbContinuationLimitationTag::VerilogAInternalStateNotRetained => {
                Self::VerilogAInternalStateNotRetained
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvelopePayload {
    /// Identity and completeness contract of the authenticated carrier state.
    pub continuation: EnvelopeContinuationDocument,
    /// Carrier harmonic spectra the envelope was started from.
    pub carrier: EnvelopeCarrierDocument,
    /// Everything the continued transient carries beyond its axis, signals,
    /// and scalars.
    pub transient: TransientPayload,
}

impl EnvelopePayload {
    fn validate(&self) -> Result<(), ResultDocumentError> {
        super::require_name(
            "envelope HB configuration identity",
            &self.continuation.hb_config_identity,
        )?;
        super::require_name(
            "envelope original netlist identity",
            &self.continuation.original_netlist_identity,
        )?;
        super::require_name(
            "envelope resolved simulation identity",
            &self.continuation.resolved_simulation_identity,
        )?;
        for source in &self.continuation.canonical_frozen_sources {
            super::require_name("envelope frozen source", source)?;
        }
        finite(
            "envelope carrier fundamental frequency",
            self.continuation.carrier_fundamental_frequency,
        )?;
        finite("envelope history step", self.continuation.history_step)?;
        finite("envelope time origin", self.continuation.time_origin)?;
        finite(
            "envelope slow-time duration",
            self.continuation.slow_time_duration,
        )?;
        finite(
            "envelope slow-time maximum step",
            self.continuation.slow_time_max_step,
        )?;
        if self.continuation.slow_time_duration <= 0.0 {
            return Err(ResultDocumentError::Malformed {
                location: "envelope continuation",
                detail: "slow-time duration must be positive".to_owned(),
            });
        }
        self.carrier.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvelopeContinuationDocument {
    pub guarantee: EnvelopeGuaranteeTag,
    pub carrier_fundamental_frequency: f64,
    pub carrier_harmonics: usize,
    /// Digest binding this artifact to one resolved HB configuration.
    pub hb_config_identity: String,
    /// Canonical names of the sources frozen for the carrier solve.
    pub canonical_frozen_sources: Vec<String>,
    pub original_netlist_identity: String,
    pub resolved_simulation_identity: String,
    /// History interval the checkpoint was built with, in seconds.
    pub history_step: f64,
    /// Slow-time origin the continuation restarted from, in seconds.
    pub time_origin: f64,
    /// Slow-time interval the continuation integrated, in seconds.
    pub slow_time_duration: f64,
    /// Maximum slow-time step the continuation was allowed, in seconds.
    pub slow_time_max_step: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EnvelopeGuaranteeTag {
    /// Exact phase projection for ordinary linear R/C elements, independent
    /// current sources, and ideal voltage-source MNA branches.
    ExactLinearRcMnaV1,
}

impl From<HbEnvelopeStateGuarantee> for EnvelopeGuaranteeTag {
    fn from(guarantee: HbEnvelopeStateGuarantee) -> Self {
        match guarantee {
            HbEnvelopeStateGuarantee::ExactLinearRcMnaV1 => Self::ExactLinearRcMnaV1,
        }
    }
}

impl From<EnvelopeGuaranteeTag> for HbEnvelopeStateGuarantee {
    fn from(guarantee: EnvelopeGuaranteeTag) -> Self {
        match guarantee {
            EnvelopeGuaranteeTag::ExactLinearRcMnaV1 => Self::ExactLinearRcMnaV1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvelopeCarrierDocument {
    pub converged: bool,
    pub iterations: usize,
    pub residual_norm: f64,
    pub fundamental_frequency: f64,
    /// Harmonic frequencies in hertz, `[0, f0, 2*f0, ...]`.
    pub harmonic_frequencies: Vec<f64>,
    /// One-sided node-voltage phasors per node, aligned with
    /// `harmonic_frequencies`.
    pub node_spectra: Vec<EnvelopeNodeSpectrum>,
}

impl EnvelopeCarrierDocument {
    fn value_count(&self) -> usize {
        let spectra = self
            .node_spectra
            .iter()
            .map(|spectrum| spectrum.coefficients.len().saturating_mul(2))
            .fold(0, usize::saturating_add);
        self.harmonic_frequencies.len().saturating_add(spectra)
    }

    fn validate(&self) -> Result<(), ResultDocumentError> {
        finite("envelope carrier residual norm", self.residual_norm)?;
        finite("envelope carrier fundamental", self.fundamental_frequency)?;
        finite_slice(
            "envelope carrier harmonic frequency",
            &self.harmonic_frequencies,
        )?;
        for spectrum in &self.node_spectra {
            super::require_name("envelope carrier node", &spectrum.node_name)?;
            if spectrum.coefficients.len() != self.harmonic_frequencies.len() {
                return Err(ResultDocumentError::SeriesLength {
                    location: format!("envelope carrier node '{}'", spectrum.node_name),
                    expected: self.harmonic_frequencies.len(),
                    actual: spectrum.coefficients.len(),
                });
            }
            for coefficient in &spectrum.coefficients {
                finite("envelope carrier coefficient real part", coefficient.real)?;
                finite(
                    "envelope carrier coefficient imaginary part",
                    coefficient.imaginary,
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvelopeNodeSpectrum {
    pub node_name: String,
    pub coefficients: Vec<ComplexSample>,
}
