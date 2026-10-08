//! Canonical content identity for immutable retained result data.
//!
//! This encoding is deliberately independent of serde, `Debug`, collection
//! allocation, and host word size. Presentation-only waveform attributes are
//! excluded: a color, visibility, or display-cache change does not create a
//! different source dataset. Authoritative f64/complex128 values are encoded
//! in IEEE-754 binary64 form after normalizing signed zero and NaN payloads.

use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_type::AnalysisType;
use crate::convergence_quality::TransientConvergenceEvidence;
use crate::dc_mismatch::DcMismatchScopeEvidence;
use crate::events::DigitalBusSourceEvidence;
use crate::family_metadata::{AnalysisResultFamilyMetadata, PeriodicNoiseOutputQuantity};
use crate::fft::spectrum::{FftSpectrumEvidence, FftSpectrumStatusEvidence};
use crate::floquet::{
    FloquetOrbitKindEvidence, FloquetSpectrumCertificateEvidence, FloquetSpectrumEvidence,
    FloquetStabilityVerdictEvidence, PstbStabilityClassificationEvidence,
};
use crate::monte_carlo::MonteCarloMeanConfidence;
use crate::monte_carlo_checkpoint::MonteCarloCheckpointEvidence;
use crate::noise::NoiseSummary;
use crate::operating_point::{
    DcOpResult, OperatingPointAccuracyEvidence, OperatingPointAnnotationEvidence,
    OperatingPointDeviceDetailEvidence, OperatingPointHomotopyEvidence,
    OperatingPointInitialGuessEvidence, OperatingPointNodeInitializationEvidence,
    OperatingPointProcessEvidence, OperatingPointSaveDeviceEvidence,
    OperatingPointTemperatureEvidence, OperatingPointValue,
};
use crate::pole_zero::{PoleZeroRootSetEvidence, PoleZeroSpectrumCertificate};
use crate::result_import::ResultImportSource;
use crate::saved_output::{
    ComplexExpressionPolicy, SavedOutputKind, SavedOutputMaterializationStatus, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputReceipt, SavedOutputStreaming,
};
use crate::sensitivity::{
    SensitivityBasisEvidence, SensitivityResultMode, SensitivityStudyEvidence,
};
use crate::simulation_values::ComplexResultValue;
use crate::soa_evidence::{
    SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationSeverityEvidence,
};
use crate::transfer_function::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
use crate::waveform::RetainedWaveform;
use rspice_app_types::product::ContentDigest;
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;

const RESULT_DIGEST_MAGIC: &[u8] = b"RSPICE-RESULT-DATA";
/// Supported canonical encodings, including authenticated project migrations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum ResultDigestEncoding {
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    /// 2026-09-06: the digital bus table declared over a transient's retained
    /// event traces. A declaration is data, not presentation — the same members
    /// read as one 8-bit word rather than as eight conductors is a different
    /// claim about the run — so a result that gained one is a different result.
    V10 = 10,
    /// v11 binds external source attribution to its retained event-code interpretation.
    V11 = 11,
    /// v12 binds numerical quality and its original transient time basis.
    V12 = 12,
    // Physical source bindings for durable saved-output evaluation.
    V13 = 13,
    // Arithmetic interpretation is part of an immutable saved-output recipe.
    V14 = 14,
    // Explicit availability for sensitivity quantities.
    V15 = 15,
    // Exact current charge observations and frontend delivery coverage.
    V16 = 16,
}

impl ResultDigestEncoding {
    /// Encoding used for newly retained results and datasets.
    pub const CURRENT: Self = Self::V16;
}

const CANONICAL_NAN_BITS: u64 = 0x7ff8_0000_0000_0000;

struct ResultDigestWriter {
    hasher: Sha256,
    retained_bytes: u64,
}

impl crate::convergence_quality::ConvergenceEncoder for ResultDigestWriter {
    fn u64(&mut self, value: u64) {
        Self::u64(self, value);
    }
    fn f64(&mut self, value: f64) {
        Self::f64(self, value);
    }
    fn string(&mut self, value: &str) {
        Self::string(self, value);
    }
    fn tag(&mut self, value: u8) {
        Self::u8(self, value);
    }
}

impl ResultDigestWriter {
    fn new(domain: &str, encoding_version: ResultDigestEncoding) -> Self {
        let mut writer = Self {
            hasher: Sha256::new(),
            retained_bytes: 0,
        };
        writer.raw(RESULT_DIGEST_MAGIC);
        writer.raw(&(encoding_version as u16).to_be_bytes());
        writer.string(domain);
        writer
    }

    fn bool(&mut self, value: bool) {
        self.retained_bytes = self.retained_bytes.saturating_add(1);
        self.raw(&[0x01, u8::from(value)]);
    }

    fn u8(&mut self, value: u8) {
        self.retained_bytes = self.retained_bytes.saturating_add(1);
        self.raw(&[0x02, value]);
    }

    fn u64(&mut self, value: u64) {
        self.retained_bytes = self.retained_bytes.saturating_add(8);
        self.raw(&[0x03]);
        self.raw(&value.to_be_bytes());
    }

    fn usize(&mut self, value: usize) {
        self.u64(u64::try_from(value).expect("supported Rust targets use at most 64-bit usize"));
    }

    /// A signed index, encoded in the unsigned tag with its two's-complement
    /// bits. A declared bus range is stated exactly as it was declared, and a
    /// descending range's indices are negative in the general case.
    fn i64(&mut self, value: i64) {
        self.u64(u64::from_be_bytes(value.to_be_bytes()));
    }

    fn f64(&mut self, value: f64) {
        self.retained_bytes = self.retained_bytes.saturating_add(8);
        let bits = if value == 0.0 {
            0
        } else if value.is_nan() {
            CANONICAL_NAN_BITS
        } else {
            value.to_bits()
        };
        self.raw(&[0x04]);
        self.raw(&bits.to_be_bytes());
    }

    fn string(&mut self, value: &str) {
        self.raw(&[0x05]);
        self.usize(value.len());
        self.retained_bytes = self
            .retained_bytes
            .saturating_add(u64::try_from(value.len()).unwrap_or(u64::MAX));
        self.raw(value.as_bytes());
    }

    fn digest(&mut self, value: ContentDigest) {
        self.retained_bytes = self.retained_bytes.saturating_add(32);
        self.raw(&[0x06]);
        self.raw(value.as_bytes());
    }

    fn uuid(&mut self, value: uuid::Uuid) {
        self.retained_bytes = self.retained_bytes.saturating_add(16);
        self.raw(&[0x07]);
        self.raw(value.as_bytes());
    }

    fn sequence(&mut self, len: usize) {
        self.raw(&[0x08]);
        self.usize(len);
    }

    fn option<T: ?Sized>(&mut self, value: Option<&T>, encode: impl FnOnce(&mut Self, &T)) {
        self.retained_bytes = self.retained_bytes.saturating_add(1);
        match value {
            Some(value) => {
                self.raw(&[0x09, 1]);
                encode(self, value);
            }
            None => self.raw(&[0x09, 0]),
        }
    }

    fn f64_slice(&mut self, values: &[f64]) {
        self.sequence(values.len());
        for value in values {
            self.f64(*value);
        }
    }

    fn finish(self) -> ContentDigest {
        ContentDigest::from_bytes(self.hasher.finalize().into())
    }

    fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    fn raw(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }
}

/// Borrowed authoritative evidence for canonical identity and retention accounting.
/// Waveforms are visited in their retained order, without copying sample arrays
/// or exposing presentation attributes to the encoder.
pub struct AnalysisResultDataRef<'a, W> {
    pub analysis_type: AnalysisType,
    pub success: bool,
    pub error_message: Option<&'a str>,
    pub import_source: Option<&'a ResultImportSource>,
    pub convergence: Option<&'a TransientConvergenceEvidence>,
    pub waveforms: W,
    pub dc_op: Option<&'a DcOpResult>,
    pub device_op: Option<&'a rspice_core::circuit::DeviceOpReport>,
    pub noise_summary: Option<&'a NoiseSummary>,
    pub family_metadata: Option<&'a AnalysisResultFamilyMetadata>,
    pub result_payload: Option<&'a AnalysisResultPayload>,
    pub measurements: &'a [rspice_core::MeasureResult],
    pub saved_output_receipts: &'a [SavedOutputReceipt],
    pub monte_carlo_checkpoint: Option<&'a MonteCarloCheckpointEvidence>,
    pub native_scalar_units: Option<&'a BTreeMap<String, rspice_core::analysis::MeasurementUnit>>,
}

impl<'a, W> AnalysisResultDataRef<'a, W>
where
    W: ExactSizeIterator<Item = &'a RetainedWaveform> + Clone,
{
    /// SHA-256 identity of authoritative data in the selected supported encoding.
    #[must_use]
    pub fn digest(&self, encoding: ResultDigestEncoding) -> ContentDigest {
        self.result_data_writer_with_encoding(encoding).finish()
    }

    /// Logical retained bytes from the complete current digest field walk.
    /// The presentation owner accounts for its derived caches separately.
    #[must_use]
    pub fn retained_data_bytes(&self) -> u64 {
        self.result_data_writer_with_encoding(ResultDigestEncoding::CURRENT)
            .retained_bytes()
    }

    fn result_data_writer_with_encoding(
        &self,
        version: ResultDigestEncoding,
    ) -> ResultDigestWriter {
        let domain = match version {
            ResultDigestEncoding::V1 => "rspice.analysis-result-data/v1",
            ResultDigestEncoding::V2 => "rspice.analysis-result-data/v2",
            ResultDigestEncoding::V3 => "rspice.analysis-result-data/v3",
            ResultDigestEncoding::V4 => "rspice.analysis-result-data/v4",
            ResultDigestEncoding::V5 => "rspice.analysis-result-data/v5",
            ResultDigestEncoding::V6 => "rspice.analysis-result-data/v6",
            ResultDigestEncoding::V7 => "rspice.analysis-result-data/v7",
            ResultDigestEncoding::V8 => "rspice.analysis-result-data/v8",
            ResultDigestEncoding::V9 => "rspice.analysis-result-data/v9",
            ResultDigestEncoding::V10 => "rspice.analysis-result-data/v10",
            ResultDigestEncoding::V11 => "rspice.analysis-result-data/v11",
            ResultDigestEncoding::V12 => "rspice.analysis-result-data/v12",
            ResultDigestEncoding::V13 => "rspice.analysis-result-data/v13",
            ResultDigestEncoding::V14 => "rspice.analysis-result-data/v14",
            ResultDigestEncoding::V15 => "rspice.analysis-result-data/v15",
            ResultDigestEncoding::V16 => "rspice.analysis-result-data/v16",
        };
        let mut writer = ResultDigestWriter::new(domain, version);
        writer.u8(analysis_type_tag(self.analysis_type));
        writer.bool(self.success);
        writer.option(self.error_message, |writer, value| {
            writer.string(value);
        });

        if version >= ResultDigestEncoding::V11 {
            writer.option(self.import_source, |writer, source| {
                writer.string(&source.source_name);
                writer.string(source.format.canonical_id());
            });
        }
        if version >= ResultDigestEncoding::V12 {
            writer.option(self.convergence, |writer, quality| quality.encode(writer));
        }
        writer.sequence(self.waveforms.len());
        for waveform in self.waveforms.clone() {
            writer.string(&waveform.name);
            writer.f64_slice(&waveform.x);
            writer.f64_slice(&waveform.y);
            writer.option(waveform.complex.as_ref(), |writer, complex| {
                writer.string(&complex.source_name);
                writer.f64_slice(&complex.real);
                writer.f64_slice(&complex.imag);
            });
            // The unit is not presentation: the same samples read as amps
            // rather than volts are different data, and a dataset that
            // restated one must not keep the identity of the one it replaced.
            if version >= ResultDigestEncoding::V6 {
                writer.option(waveform.unit.as_deref(), |writer, unit| {
                    writer.string(unit);
                });
            }
        }

        writer.option(self.dc_op, encode_dc_op);
        writer.option(self.device_op, encode_device_op);
        writer.option(self.noise_summary, |writer, summary| {
            encode_noise_summary(writer, summary, version);
        });
        writer.option(self.family_metadata, encode_family_metadata);
        if version >= ResultDigestEncoding::V2 {
            writer.option(self.result_payload, |writer, payload| {
                encode_result_payload(writer, payload, version);
            });
        }

        writer.sequence(self.measurements.len());
        for measurement in self.measurements {
            writer.string(&measurement.name);
            writer.option(measurement.value.as_ref(), |writer, value| {
                writer.f64(*value);
            });
            writer.option(measurement.error.as_deref(), |writer, error| {
                writer.string(error);
            });
            writer.bool(measurement.passed);
            writer.option(measurement.expected.as_ref(), |writer, value| {
                writer.f64(*value);
            });
            writer.option(measurement.tolerance.as_ref(), |writer, value| {
                writer.f64(*value);
            });
            if version >= ResultDigestEncoding::V9 {
                writer.option(measurement.raw_value.as_ref(), |writer, value| {
                    writer.f64(*value);
                });
                writer.option(measurement.failure_limit.as_ref(), |writer, value| {
                    writer.f64(*value);
                });
                writer.bool(measurement.failure_limit_exceeded);
            }
        }

        writer.sequence(self.saved_output_receipts.len());
        for receipt in self.saved_output_receipts {
            writer.uuid(receipt.output_id.as_uuid());
            writer.u64(receipt.output_revision.get());
            writer.uuid(receipt.analysis_id.as_uuid());
            writer.digest(receipt.contract_digest);
            writer.string(&receipt.name);
            writer.string(&receipt.source_expression);
            if version >= ResultDigestEncoding::V14 {
                writer.u8(match receipt.complex_policy {
                    ComplexExpressionPolicy::LegacyMagnitude => 0,
                    ComplexExpressionPolicy::Rectangular => 1,
                });
            }
            writer.u8(saved_output_kind_tag(receipt.output_kind));
            writer.u8(saved_output_policy_tag(receipt.save_policy));
            writer.u8(saved_output_precision_tag(receipt.stored_precision));
            writer.u8(saved_output_streaming_tag(receipt.streaming));
            encode_saved_output_status(&mut writer, &receipt.status);
            if version >= ResultDigestEncoding::V13 {
                writer.option(receipt.source_bindings.as_ref(), |writer, bindings| {
                    use crate::saved_output::{SavedOutputAxis, SavedOutputBoundSource};
                    match &bindings.axis {
                        SavedOutputAxis::Missing => writer.u8(0),
                        SavedOutputAxis::OperatingPoint => writer.u8(1),
                        SavedOutputAxis::Waveform { name } => {
                            writer.u8(2);
                            writer.string(name);
                        }
                        SavedOutputAxis::DcFamily => writer.u8(3),
                    }
                    writer.sequence(bindings.references.len());
                    for (reference, source) in &bindings.references {
                        writer.string(reference);
                        match source {
                            SavedOutputBoundSource::Missing => writer.u8(0),
                            SavedOutputBoundSource::Ground => writer.u8(1),
                            SavedOutputBoundSource::Waveform { name } => {
                                writer.u8(2);
                                writer.string(name);
                            }
                            SavedOutputBoundSource::DcQuantity { quantity } => {
                                writer.u8(3);
                                writer.u64(*quantity as u64);
                            }
                        }
                    }
                });
            }
        }

        // Optional extension: historical results without a journal keep their
        // existing identities. Bind exact binary bits, including signed zero,
        // through the validated content digest and account for the full blob.
        if version >= ResultDigestEncoding::V16
            && let Some(checkpoint) = self.monte_carlo_checkpoint
        {
            writer.string("monte-carlo-checkpoint/v1");
            writer.digest(checkpoint.digest());
            writer.usize(checkpoint.bytes().len());
            writer.retained_bytes = writer
                .retained_bytes
                .saturating_add(checkpoint.bytes().len() as u64);
        }

        let members = self
            .family_metadata
            .map_or(&[][..], |metadata| metadata.member_measurements());
        if version >= ResultDigestEncoding::V16
            && (self
                .measurements
                .iter()
                .any(|measurement| measurement.units.is_some())
                || members.iter().any(|member| {
                    member
                        .measurements
                        .iter()
                        .any(|measurement| measurement.unit.is_some())
                }))
        {
            writer.string("measurement-units/v1");
            writer.sequence(self.measurements.len());
            for measurement in self.measurements {
                writer.option(measurement.units.as_ref(), |writer, units| {
                    encode_measurement_unit(writer, &units.value);
                    encode_measurement_unit(writer, &units.raw_value);
                    encode_measurement_unit(writer, &units.axis);
                });
            }
            writer.sequence(members.len());
            for member in members {
                writer.sequence(member.measurements.len());
                for measurement in &member.measurements {
                    writer.option(measurement.unit.as_ref(), encode_measurement_unit);
                }
            }
        }
        // The absent extension leaves all historical result identities intact.
        if version >= ResultDigestEncoding::V16
            && let Some(units) = self.native_scalar_units
        {
            writer.string("native-scalar-units/v1");
            writer.sequence(units.len());
            for (name, unit) in units {
                writer.string(name);
                encode_measurement_unit(&mut writer, unit);
            }
        }
        writer
    }
}
fn encode_measurement_unit(
    writer: &mut ResultDigestWriter,
    unit: &rspice_core::analysis::MeasurementUnit,
) {
    match unit {
        rspice_core::analysis::MeasurementUnit::Unknown => writer.u8(0),
        rspice_core::analysis::MeasurementUnit::Known(symbol) => {
            writer.u8(1);
            writer.string(symbol);
        }
    }
}

/// Identity of ordered analysis data, excluding run addresses and wall-clock metadata.
/// The same encoding is used for the dataset and every member analysis.
#[must_use]
pub fn dataset_content_digest<'a, W>(
    analyses: impl ExactSizeIterator<Item = (u64, AnalysisResultDataRef<'a, W>)>,
    version: ResultDigestEncoding,
) -> ContentDigest
where
    W: ExactSizeIterator<Item = &'a RetainedWaveform> + Clone,
{
    let domain = match version {
        ResultDigestEncoding::V1 => "rspice.simulation-dataset-data/v1",
        ResultDigestEncoding::V2 => "rspice.simulation-dataset-data/v2",
        ResultDigestEncoding::V3 => "rspice.simulation-dataset-data/v3",
        ResultDigestEncoding::V4 => "rspice.simulation-dataset-data/v4",
        ResultDigestEncoding::V5 => "rspice.simulation-dataset-data/v5",
        ResultDigestEncoding::V6 => "rspice.simulation-dataset-data/v6",
        ResultDigestEncoding::V7 => "rspice.simulation-dataset-data/v7",
        ResultDigestEncoding::V8 => "rspice.simulation-dataset-data/v8",
        ResultDigestEncoding::V9 => "rspice.simulation-dataset-data/v9",
        ResultDigestEncoding::V10 => "rspice.simulation-dataset-data/v10",
        ResultDigestEncoding::V11 => "rspice.simulation-dataset-data/v11",
        ResultDigestEncoding::V12 => "rspice.simulation-dataset-data/v12",
        ResultDigestEncoding::V13 => "rspice.simulation-dataset-data/v13",
        ResultDigestEncoding::V14 => "rspice.simulation-dataset-data/v14",
        ResultDigestEncoding::V15 => "rspice.simulation-dataset-data/v15",
        ResultDigestEncoding::V16 => "rspice.simulation-dataset-data/v16",
    };
    let mut writer = ResultDigestWriter::new(domain, version);
    writer.sequence(analyses.len());
    for (id, analysis) in analyses {
        writer.u64(id);
        writer.digest(analysis.digest(version));
    }
    writer.finish()
}
fn encode_result_payload(
    writer: &mut ResultDigestWriter,
    payload: &AnalysisResultPayload,
    encoding_version: ResultDigestEncoding,
) {
    match payload {
        AnalysisResultPayload::DcSweep { evidence } => {
            use crate::dc_sweep::{
                DcCurveSelection, DcSweepDirection, DcSweepFamily, DcSweepQuantity,
            };
            writer.u8(10);
            writer.string(&evidence.source);
            writer.u8(match evidence.direction {
                DcSweepDirection::Ascending => 0,
                DcSweepDirection::Descending => 1,
                DcSweepDirection::AsAuthored => 2,
            });
            writer.sequence(evidence.quantities.len());
            for quantity in &evidence.quantities {
                writer.u8(match quantity {
                    DcSweepQuantity::NodeVoltage(_) => 0,
                    DcSweepQuantity::BranchCurrent(_) => 1,
                });
                writer.string(quantity.name());
            }
            match &evidence.family {
                DcSweepFamily::Single => writer.u8(0),
                DcSweepFamily::Nested { source, values } => {
                    writer.u8(1);
                    writer.string(source);
                    writer.sequence(values.len());
                    for value in values {
                        writer.f64(*value);
                    }
                }
                DcSweepFamily::Retraced => writer.u8(2),
            }
            match &evidence.selection {
                DcCurveSelection::All => writer.u8(0),
                DcCurveSelection::Saved(curves) => {
                    writer.u8(1);
                    writer.sequence(curves.len());
                    for curve in curves {
                        writer.u64(curve.quantity as u64);
                        writer.u64(curve.member as u64);
                    }
                }
            }
        }
        AnalysisResultPayload::OperatingPoint {
            temperature_mode,
            temperature_celsius,
            initial_guess,
            node_initialization,
            homotopy,
            annotation,
            device_detail,
            save_device_op,
            accuracy,
            selected_devices,
            violation_devices,
            violation_source_content_digest,
            validated_startup_directives,
            mna_node_names,
            mna_branch_names,
            mna_solution,
            effective_source_content_digest,
            previous_state,
            run_point_index,
            run_point_count,
            run_point_process,
            run_point_supply_voltage,
            run_point_nominal_supply_voltage,
        } => {
            writer.u8(6);
            writer.u8(match temperature_mode {
                OperatingPointTemperatureEvidence::PvtRunSet => 0,
                OperatingPointTemperatureEvidence::Nominal27C => 1,
                OperatingPointTemperatureEvidence::Explicit => 2,
                OperatingPointTemperatureEvidence::ActiveRunSetAxis => 3,
            });
            writer.f64(*temperature_celsius);
            writer.u8(match initial_guess {
                OperatingPointInitialGuessEvidence::Automatic => 0,
                OperatingPointInitialGuessEvidence::PreviousConverged => 1,
                OperatingPointInitialGuessEvidence::UserNodeVoltages => 2,
                OperatingPointInitialGuessEvidence::ZeroState => 3,
                OperatingPointInitialGuessEvidence::PreviousCompatible => 4,
            });
            writer.u8(match node_initialization {
                OperatingPointNodeInitializationEvidence::UseIcAndNodeset => 0,
                OperatingPointNodeInitializationEvidence::IgnoreIcAndNodeset => 1,
                OperatingPointNodeInitializationEvidence::ForceIcValues => 2,
                OperatingPointNodeInitializationEvidence::ValidateOnly => 3,
            });
            writer.u8(match homotopy {
                OperatingPointHomotopyEvidence::Adaptive => 0,
                OperatingPointHomotopyEvidence::SourceStepping => 1,
                OperatingPointHomotopyEvidence::GminStepping => 2,
                OperatingPointHomotopyEvidence::PseudoTransient => 3,
                OperatingPointHomotopyEvidence::None => 4,
            });
            writer.u8(match annotation {
                OperatingPointAnnotationEvidence::VoltagesAndCurrents => 0,
                OperatingPointAnnotationEvidence::VoltagesOnly => 1,
                OperatingPointAnnotationEvidence::VoltagesAndDeviceOp => 2,
                OperatingPointAnnotationEvidence::None => 3,
            });
            writer.u8(match device_detail {
                OperatingPointDeviceDetailEvidence::SelectedAndViolations => 0,
                OperatingPointDeviceDetailEvidence::AllDevices => 1,
                OperatingPointDeviceDetailEvidence::ViolationsOnly => 2,
                OperatingPointDeviceDetailEvidence::None => 3,
            });
            writer.u8(match save_device_op {
                OperatingPointSaveDeviceEvidence::Enabled => 0,
                OperatingPointSaveDeviceEvidence::Disabled => 1,
                OperatingPointSaveDeviceEvidence::FinalPointOnly => 2,
            });
            writer.u8(match accuracy {
                OperatingPointAccuracyEvidence::Fast => 0,
                OperatingPointAccuracyEvidence::Balanced => 1,
                OperatingPointAccuracyEvidence::Accurate => 2,
                OperatingPointAccuracyEvidence::Robust => 3,
            });
            writer.sequence(selected_devices.len());
            for device in selected_devices {
                writer.string(device);
            }
            writer.sequence(violation_devices.len());
            for device in violation_devices {
                writer.string(device);
            }
            writer.option(
                violation_source_content_digest.as_ref(),
                |writer, digest| writer.digest(*digest),
            );
            writer.u64(*validated_startup_directives);
            writer.sequence(mna_node_names.len());
            for name in mna_node_names {
                writer.string(name);
            }
            writer.sequence(mna_branch_names.len());
            for name in mna_branch_names {
                writer.string(name);
            }
            writer.f64_slice(mna_solution);
            writer.u64(*run_point_index);
            writer.u64(*run_point_count);
            // Keep the common nominal point compact while retaining exact
            // process/supply evidence whenever a real corner is present.
            if *run_point_process != OperatingPointProcessEvidence::TT
                || run_point_supply_voltage.is_some()
                || run_point_nominal_supply_voltage.is_some()
            {
                writer.u8(0xa5);
                writer.u8(match run_point_process {
                    OperatingPointProcessEvidence::TT => 0,
                    OperatingPointProcessEvidence::SS => 1,
                    OperatingPointProcessEvidence::FF => 2,
                    OperatingPointProcessEvidence::SF => 3,
                    OperatingPointProcessEvidence::FS => 4,
                });
                writer.option(run_point_supply_voltage.as_ref(), |writer, voltage| {
                    writer.f64(*voltage);
                });
                writer.option(
                    run_point_nominal_supply_voltage.as_ref(),
                    |writer, voltage| writer.f64(*voltage),
                );
            }
            if let Some(digest) = effective_source_content_digest {
                writer.u8(0xa6);
                writer.digest(*digest);
            }
            if let Some(previous) = previous_state {
                writer.u8(0xa7);
                writer.digest(previous.source_content_digest);
                writer.digest(previous.producer_snapshot_digest);
                writer.digest(previous.producer_result_digest);
            }
        }
        AnalysisResultPayload::PoleZero {
            poles,
            zeros,
            pole_evidence,
            zero_evidence,
            gain,
        } => {
            writer.u8(0);
            encode_complex_result_values(writer, poles);
            encode_complex_result_values(writer, zeros);
            if encoding_version >= ResultDigestEncoding::V7 {
                writer.option(gain.as_ref(), |writer, gain| writer.f64(*gain));
                encode_pole_zero_root_evidence(writer, pole_evidence);
                encode_pole_zero_root_evidence(writer, zero_evidence);
            } else if let Some(gain) = gain {
                // Preserve the byte-for-byte schema-v9 through schema-v15
                // encoding for authenticated legacy migration.
                writer.f64(*gain);
            } else {
                // No authentic legacy payload could omit gain. Keep this
                // deterministic and distinct so validation reports a digest
                // mismatch instead of panicking on a tampered document.
                writer.u8(u8::MAX);
            }
        }
        AnalysisResultPayload::PssFloquet {
            period_s,
            fundamental_frequency_hz,
            iterations,
            residual_norm,
            multipliers,
            floquet_evidence,
            orbit_kind,
            trivial_multiplier_index,
            stability_verdict,
        } => {
            if encoding_version < ResultDigestEncoding::V8 {
                // No authentic schema-v16 result could contain this payload.
                // Migration rejects it before invoking the legacy encoder;
                // the sentinel keeps digest helpers total for adversarial data.
                writer.u8(u8::MAX);
                return;
            }
            writer.u8(8);
            writer.option(period_s.as_ref(), |writer, value| writer.f64(*value));
            writer.option(fundamental_frequency_hz.as_ref(), |writer, value| {
                writer.f64(*value)
            });
            writer.option(iterations.as_ref(), |writer, value| writer.u64(*value));
            writer.option(residual_norm.as_ref(), |writer, value| writer.f64(*value));
            writer.sequence(multipliers.len());
            for multiplier in multipliers {
                encode_complex_result_value(writer, multiplier.multiplier);
            }
            encode_floquet_spectrum_evidence(writer, floquet_evidence);
            writer.u8(floquet_orbit_kind_tag(*orbit_kind));
            writer.option(trivial_multiplier_index.as_ref(), |writer, value| {
                writer.u64(*value)
            });
            writer.u8(floquet_stability_verdict_tag(*stability_verdict));
        }
        AnalysisResultPayload::Pstb {
            period_s,
            fundamental_frequency_hz,
            stability_threshold,
            probe_instance,
            detect_subharmonics,
            modes,
            floquet_evidence,
            orbit_kind,
            trivial_multiplier_index,
            stability_verdict,
            stability_classification,
            min_stability_margin_db,
            max_multiplier_magnitude,
            num_unstable,
            subharmonics,
            converged,
            iterations,
        } => {
            if encoding_version < ResultDigestEncoding::V8 {
                writer.u8(u8::MAX - 1);
                return;
            }
            writer.u8(9);
            writer.option(period_s.as_ref(), |writer, value| writer.f64(*value));
            writer.option(fundamental_frequency_hz.as_ref(), |writer, value| {
                writer.f64(*value)
            });
            writer.option(stability_threshold.as_ref(), |writer, value| {
                writer.f64(*value)
            });
            writer.option(probe_instance.as_deref(), |writer, value| {
                writer.string(value)
            });
            writer.option(detect_subharmonics.as_ref(), |writer, value| {
                writer.bool(*value)
            });
            writer.sequence(modes.len());
            for mode in modes {
                encode_complex_result_value(writer, mode.multiplier);
                encode_complex_result_value(writer, mode.exponent);
                writer.f64(mode.probe_participation);
                writer.bool(mode.is_unstable);
                writer.bool(mode.is_trivial);
                writer.option(mode.subharmonic_order.as_ref(), |writer, value| {
                    writer.u64(*value)
                });
            }
            encode_floquet_spectrum_evidence(writer, floquet_evidence);
            writer.u8(floquet_orbit_kind_tag(*orbit_kind));
            writer.option(trivial_multiplier_index.as_ref(), |writer, value| {
                writer.u64(*value)
            });
            writer.u8(floquet_stability_verdict_tag(*stability_verdict));
            writer.u8(pstb_stability_classification_tag(*stability_classification));
            writer.option(min_stability_margin_db.as_ref(), |writer, value| {
                writer.f64(*value)
            });
            writer.option(max_multiplier_magnitude.as_ref(), |writer, value| {
                writer.f64(*value)
            });
            writer.option(num_unstable.as_ref(), |writer, value| writer.u64(*value));
            writer.sequence(subharmonics.len());
            for order in subharmonics {
                writer.u64(*order);
            }
            writer.option(converged.as_ref(), |writer, value| writer.bool(*value));
            writer.option(iterations.as_ref(), |writer, value| writer.u64(*value));
        }
        AnalysisResultPayload::Sensitivity {
            output,
            result_mode,
            rows,
        } => {
            writer.u8(1);
            writer.string(output);
            match result_mode {
                SensitivityResultMode::Dc => writer.u8(0),
                SensitivityResultMode::Ac { frequency_hz } => {
                    writer.u8(1);
                    writer.f64(*frequency_hz);
                }
            }
            writer.sequence(rows.len());
            for row in rows {
                writer.string(&row.parameter);
                for value in [row.raw, row.normalized] {
                    match value {
                        rspice_core::analysis::sensitivity::SensitivityValue::Available(value) => {
                            if encoding_version >= ResultDigestEncoding::V15 {
                                writer.u8(0);
                            }
                            writer.f64(value);
                        }
                        rspice_core::analysis::sensitivity::SensitivityValue::Unavailable {
                            unavailable,
                        } => {
                            // Legacy imports reject this shape before checking their digest.
                            writer.u8(1);
                            writer.string(unavailable.as_str());
                        }
                    }
                }
            }
        }
        // Written unconditionally of `encoding_version`. The per-era rules
        // above replay an older encoding over a payload that era could
        // contain; no build before this one could write a DC mismatch
        // payload, so there is no older encoding of it to replay.
        AnalysisResultPayload::DcMismatch { evidence } => {
            writer.u8(11);
            writer.string(&evidence.output);
            writer.string(&evidence.output_unit);
            writer.f64(evidence.nominal_value);
            writer.f64(evidence.sigma_multiplier);
            writer.f64(evidence.sigma_total);
            writer.f64(evidence.sigma_mismatch);
            writer.f64(evidence.sigma_process);
            writer.bool(evidence.include_mismatch);
            writer.bool(evidence.include_process);
            writer.u64(evidence.contributor_limit);
            writer.f64(evidence.threshold);
            writer.bool(evidence.normalized_contributions);
            writer.u64(evidence.applied_correlations_mismatch);
            writer.u64(evidence.applied_correlations_process);
            writer.u64(evidence.evaluated_contributors);
            writer.sequence(evidence.contributors.len());
            for row in &evidence.contributors {
                writer.string(&row.instance);
                writer.string(&row.parameter);
                writer.u8(match row.scope {
                    DcMismatchScopeEvidence::Mismatch => 0,
                    DcMismatchScopeEvidence::Process => 1,
                });
                writer.f64(row.sigma_parameter);
                writer.f64(row.sensitivity);
                writer.f64(row.contribution);
                writer.f64(row.share);
            }
        }
        AnalysisResultPayload::ScalarMeasurements { values } => {
            writer.u8(2);
            writer.sequence(values.len());
            for (name, value) in values {
                writer.string(name);
                writer.f64(*value);
            }
        }
        AnalysisResultPayload::TransferFunction {
            input_source,
            output_expression,
            input_quantity,
            output_quantity,
            input_unit,
            output_unit,
            normalization,
            accuracy,
            gain,
            input_resistance,
            output_resistance,
            nominal_input,
            nominal_output,
        } => {
            writer.u8(5);
            writer.string(input_source);
            writer.string(output_expression);
            writer.u8(transfer_function_quantity_tag(*input_quantity));
            writer.u8(transfer_function_quantity_tag(*output_quantity));
            writer.string(input_unit);
            writer.string(output_unit);
            writer.u8(transfer_function_normalization_tag(*normalization));
            writer.u8(transfer_function_accuracy_tag(*accuracy));
            writer.option(gain.as_ref(), encode_transfer_function_scalar);
            writer.option(input_resistance.as_ref(), encode_transfer_function_scalar);
            writer.option(output_resistance.as_ref(), encode_transfer_function_scalar);
            writer.option(nominal_input.as_ref(), |writer, value| writer.f64(*value));
            writer.option(nominal_output.as_ref(), |writer, value| writer.f64(*value));
        }
        AnalysisResultPayload::Soa {
            source_history,
            evaluations,
            violations,
        } => {
            writer.u8(4);
            if let Some(source) = source_history {
                writer.string("soa-complete-observation-history-v1");
                writer.f64_slice(&source.time);
                writer.sequence(source.waveforms.len());
                for wave in &source.waveforms {
                    writer.string(&wave.name);
                    writer.string(&wave.unit);
                    writer.f64_slice(&wave.values);
                }
            }
            writer.sequence(evaluations.len());
            for evaluation in evaluations {
                writer.string(&evaluation.device_id);
                writer.u8(soa_parameter_tag(evaluation.parameter));
                writer.f64(evaluation.limit_value);
                writer.f64(evaluation.worst_actual_value);
                writer.f64(evaluation.worst_time_s);
                writer.u64(evaluation.sample_count);
                writer.string(&evaluation.unit);
                writer.string(&evaluation.description);
                writer.u8(soa_rule_verdict_tag(evaluation.verdict));
            }
            writer.sequence(violations.len());
            for violation in violations {
                writer.string(&violation.device_id);
                writer.u8(soa_parameter_tag(violation.parameter));
                writer.f64(violation.limit_value);
                writer.f64(violation.actual_value);
                writer.f64(violation.time_s);
                writer.u8(soa_violation_severity_tag(violation.severity));
            }
            if evaluations
                .iter()
                .any(|evaluation| evaluation.derating.is_some())
            {
                writer.string("soa-power-derating-evidence-v1");
                writer.sequence(evaluations.len());
                for evaluation in evaluations {
                    writer.option(evaluation.derating.as_ref(), |writer, derating| {
                        writer.f64(derating.rated_power_w);
                        writer.f64(derating.curve.reference_temperature_kelvin);
                        writer.f64(derating.curve.watts_per_kelvin);
                    });
                }
            }
            if evaluations.iter().any(|e| e.envelope.is_some()) {
                writer.string("soa-current-voltage-evidence-v1");
                writer.sequence(evaluations.len());
                for evaluation in evaluations {
                    writer.option(evaluation.envelope.as_ref(), |writer, envelope| {
                        writer.f64(envelope.maximum_current_a);
                        let curve = &envelope.curve;
                        writer.string(&curve.source);
                        writer.string(&curve.conditions);
                        writer.f64_slice(&curve.voltages_v);
                        writer.option(curve.dc_currents_a.as_ref(), |writer, row| {
                            writer.f64_slice(row)
                        });
                        writer.option(curve.pulse_width_s.as_ref(), |writer, width| {
                            writer.f64(*width)
                        });
                        writer.bool(
                            curve.voltage_interpolation
                                == crate::safety::SoaVoltageInterpolation::Logarithmic,
                        );
                        writer.bool(
                            curve.pulse_interpolation
                                == crate::safety::SoaPulseInterpolation::Logarithmic,
                        );
                        writer.sequence(curve.pulses.len());
                        for pulse in &curve.pulses {
                            writer.f64(pulse.duration_s);
                            writer.f64_slice(&pulse.currents_a);
                        }
                    });
                }
            }
            if evaluations
                .iter()
                .any(|evaluation| !evaluation.thresholds.is_default())
            {
                writer.string("soa-severity-thresholds-evidence-v1");
                writer.sequence(evaluations.len());
                for evaluation in evaluations {
                    writer.option(
                        evaluation.thresholds.warning_fraction.as_ref(),
                        |writer, value| writer.f64(*value),
                    );
                    writer.option(
                        evaluation.thresholds.critical_fraction.as_ref(),
                        |writer, value| writer.f64(*value),
                    );
                }
            }
            if evaluations
                .iter()
                .any(|evaluation| evaluation.duration.is_some())
            {
                writer.string("soa-excursion-duration-evidence-v1");
                writer.sequence(evaluations.len());
                for evaluation in evaluations {
                    writer.option(evaluation.duration.as_ref(), |writer, duration| {
                        writer.f64(duration.minimum_duration_s);
                        writer.f64(duration.total_exceedance_s);
                        writer.f64(duration.longest_excursion_s);
                        writer.u64(duration.qualified_excursions);
                        writer.u64(duration.rejected_excursions);
                        writer.u64(duration.clipped_excursions);
                    });
                }
            }
            if evaluations.iter().any(|evaluation| {
                evaluation
                    .duration
                    .and_then(|duration| duration.cumulative)
                    .is_some()
            }) {
                writer.string("soa-cumulative-duration-evidence-v1");
                writer.sequence(evaluations.len());
                for evaluation in evaluations {
                    writer.option(
                        evaluation
                            .duration
                            .and_then(|duration| duration.cumulative)
                            .as_ref(),
                        |writer, cumulative| {
                            writer.option(cumulative.recovery_time_s.as_ref(), |writer, value| {
                                writer.f64(*value)
                            });
                            writer.f64(cumulative.peak_exposure_s);
                            writer.f64(cumulative.final_exposure_s);
                        },
                    );
                }
            }
        }
        // Tag 12: DC mismatch evidence holds 11. Appended, never reused: these
        // bytes identify every retained result already on disk, and two
        // payloads under one tag would let a spectrum and a contributor table
        // with equal leading bytes digest alike
        // (`every_typed_payload_digests_under_its_own_tag`).
        AnalysisResultPayload::FftSpectrum { spectrum } => {
            writer.u8(12);
            encode_fft_spectrum_evidence(writer, spectrum);
        }
        AnalysisResultPayload::Stb { response } if !response.circuit_poles.is_not_computed() => {
            writer.u8(19);
            encode_stb_response(writer, response);
        }
        AnalysisResultPayload::Stb { response } => {
            // Preserve published tag-18 hashes for legacy/sweep-only evidence.
            writer.u8(18);
            encode_stb_response(writer, response);
        }
        AnalysisResultPayload::Qpnoise { response } => {
            writer.u8(17);
            writer.string(&response.metadata.retained_identity);
            writer.retained_bytes = writer
                .retained_bytes
                .saturating_add(AnalysisResultPayload::qpnoise_response_bytes(response) as u64);
        }
        AnalysisResultPayload::Qpxf { response } => {
            writer.u8(16);
            writer.string(response.metadata.retained_identity());
            let bytes = response
                .solutions
                .iter()
                .flat_map(|s| &s.sensitivities)
                .chain(response.transfers.iter().map(|t| &t.values))
                .fold(0u64, |n, row| {
                    n.saturating_add((row.len() as u64).saturating_mul(16))
                });
            writer.retained_bytes = writer.retained_bytes.saturating_add(bytes);
        }
        AnalysisResultPayload::Qpac { response } => {
            // Appended tag: QPSS retains tag 14.
            writer.u8(15);
            writer.string(response.metadata.retained_identity());
            writer.retained_bytes = writer.retained_bytes.saturating_add(
                response
                    .unit_solutions
                    .iter()
                    .flat_map(|s| &s.spectra)
                    .map(|row| row.len() as u64 * 16)
                    .sum::<u64>()
                    + (response.output_transfer.len() + response.output_response.len()) as u64 * 16,
            );
        }
        AnalysisResultPayload::Qpss { operating_point } => {
            writer.u8(14);
            // This identity binds all config, producer metadata and signed
            // MNA coefficients. Count retained numerical storage separately.
            writer.string(operating_point.retained_identity());
            writer.retained_bytes = writer.retained_bytes.saturating_add(
                operating_point
                    .complete_spectra()
                    .iter()
                    .map(|row| row.len() as u64 * 16)
                    .sum::<u64>(),
            );
        }
        // Tag 13: the recorded FFT spectrum holds 12. Written unconditionally
        // of `encoding_version` for the same reason the DC mismatch arm above
        // is: no build before this one could write a sensitivity study, so
        // there is no older encoding of one to replay. The frozen
        // `Sensitivity` arm keeps tag 1 and is untouched.
        AnalysisResultPayload::SensitivityStudy { evidence } => {
            writer.u8(13);
            encode_sensitivity_study_evidence(writer, evidence);
        }
        AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            digital_buses,
            current_impulses,
            voltage_impulses,
        } => {
            writer.u8(7);
            if encoding_version >= ResultDigestEncoding::V16 {
                writer.option(current_impulses.as_ref(), |writer, history| {
                    writer.f64(history.start_time_s);
                    writer.f64(history.stop_time_s);
                    writer.bool(history.delivery_complete);
                    writer.sequence(history.traces.len());
                    for trace in &history.traces {
                        match &trace.owner {
                            rspice_core::CurrentImpulseOwner::Branch { branch_name } => {
                                writer.u8(0);
                                writer.string(branch_name);
                            }
                            rspice_core::CurrentImpulseOwner::DeviceLead {
                                device_name,
                                parameter,
                            } => {
                                writer.u8(1);
                                writer.string(device_name);
                                writer.string(parameter);
                            }
                        }
                        writer.bool(trace.complete);
                        writer.sequence(trace.points.len());
                        for point in &trace.points {
                            writer.f64(point.time);
                            writer.f64(point.charge_coulombs);
                        }
                        // Typed extension preserves every pre-derivative digest.
                        // The string tag cannot alias the following owner/sequence tag.
                        if !trace.derivatives.is_empty() {
                            writer.string("current-impulse-derivatives-v1");
                            writer.sequence(trace.derivatives.len());
                            for point in &trace.derivatives {
                                writer.f64(point.time);
                                writer.u64(u64::from(point.order));
                                writer.f64(point.coefficient);
                            }
                        }
                    }
                });
            }
            if let Some(history) = voltage_impulses {
                writer.string("voltage-impulse-history-v1");
                writer.f64(history.start_time_s);
                writer.f64(history.stop_time_s);
                writer.bool(history.delivery_complete);
                writer.sequence(history.traces.len());
                for trace in &history.traces {
                    writer.string(&trace.node_name);
                    writer.bool(trace.complete);
                    writer.sequence(trace.points.len());
                    for point in &trace.points {
                        writer.f64(point.time);
                        writer.f64(point.volt_seconds);
                    }
                    writer.sequence(trace.derivatives.len());
                    for point in &trace.derivatives {
                        writer.f64(point.time);
                        writer.u64(u64::from(point.order));
                        writer.f64(point.coefficient);
                    }
                }
            }
            writer.sequence(digital_traces.len());
            for trace in digital_traces {
                writer.string(&trace.node_name);
                writer.sequence(trace.points.len());
                for point in &trace.points {
                    writer.f64(point.time_s);
                    writer.u8(point.value_code);
                }
            }
            writer.sequence(real_traces.len());
            for trace in real_traces {
                writer.string(&trace.node_name);
                writer.sequence(trace.points.len());
                for point in &trace.points {
                    writer.f64(point.time_s);
                    writer.f64(point.value);
                }
            }
            // A declaration is authoritative data, not presentation: the same
            // eight traces read as one byte are a different claim about the
            // run than eight independent conductors, and a result that gained
            // or lost one must not keep the identity of the one it replaced.
            if encoding_version >= ResultDigestEncoding::V10 {
                writer.sequence(digital_buses.len());
                for bus in digital_buses {
                    writer.string(&bus.name);
                    writer.i64(bus.msb);
                    writer.i64(bus.lsb);
                    writer.sequence(bus.members.len());
                    for member in &bus.members {
                        writer.string(member);
                    }
                    writer.u8(digital_bus_source_tag(bus.source));
                }
            }
        }
    }
}

const fn digital_bus_source_tag(source: DigitalBusSourceEvidence) -> u8 {
    match source {
        DigitalBusSourceEvidence::Engine => 0,
        DigitalBusSourceEvidence::Schematic => 1,
        DigitalBusSourceEvidence::Import => 2,
    }
}

fn encode_stb_response(
    writer: &mut ResultDigestWriter,
    response: &rspice_core::analysis::stb::StbResult,
) {
    writer.bool(response.success);
    writer.sequence(response.bode_points.len());
    for p in &response.bode_points {
        writer.f64(p.frequency);
        writer.f64(p.loop_gain.re);
        writer.f64(p.loop_gain.im);
        for value in [p.magnitude, p.magnitude_db, p.phase_deg] {
            writer.option(value.as_ref(), |w, v| w.f64(*v));
        }
    }
    writer.sequence(response.nyquist_points.len());
    for p in &response.nyquist_points {
        writer.f64(p.frequency);
        writer.f64(p.real);
        writer.f64(p.imag);
    }
    for margin in [response.margins.gain_margin, response.margins.phase_margin] {
        writer.option(margin.as_ref(), |w, m| {
            w.f64(m.value);
            w.f64(m.frequency);
        });
    }
    writer.option(response.margins.dc_loop_gain.as_ref(), |w, v| {
        w.f64(v.re);
        w.f64(v.im);
    });
    writer.usize(response.margins.num_crossovers);
    writer.sequence(response.warnings.len());
    for warning in &response.warnings {
        writer.string(warning);
    }
    if !response.circuit_poles.is_not_computed() {
        use rspice_core::analysis::stb::{CircuitPoleEvidence as E, CircuitPoleFailure as F};
        match &response.circuit_poles {
            E::Available { spectrum } => {
                writer.u8(0);
                writer.sequence(spectrum.poles.len());
                for pole in &spectrum.poles {
                    writer.f64(pole.re);
                    writer.f64(pole.im);
                }
                // Available evidence is validated before retaining a result.
                use rspice_core::analysis::pole_zero::RootSetEvidence as R;
                let evidence_tag = match &spectrum.evidence {
                    R::NotRequested => 0,
                    R::QualifiedEmpty { .. } => 1,
                    R::Qualified { .. } => 2,
                    R::Approximate { .. } => 3,
                    R::LegacyUnknown => 4,
                    _ => 255,
                };
                let certificate = spectrum.evidence.certificate();
                writer.u8(
                    if certificate.is_some_and(|c| c.asymptotically_stable.is_some()) {
                        evidence_tag + 4
                    } else {
                        evidence_tag
                    },
                );
                writer.option(certificate, |w, c| {
                    w.usize(c.problem_order);
                    w.usize(c.infinite_count);
                    w.f64(c.max_backward_error);
                    w.f64(c.qualification_tolerance);
                    if let Some(stable) = c.asymptotically_stable {
                        w.bool(stable);
                    }
                });
            }
            E::Unavailable { cause } => match cause {
                F::Unsupported { capability, detail } => {
                    writer.u8(1);
                    writer.string(capability);
                    writer.string(detail);
                }
                F::Numerical { detail } => {
                    writer.u8(2);
                    writer.string(detail);
                }
                F::ResourceLimit {
                    resource,
                    requested,
                    limit,
                } => {
                    writer.u8(3);
                    writer.string(resource);
                    writer.usize(*requested);
                    writer.usize(*limit);
                }
            },
            E::NotComputed => unreachable!(),
        }
    }
}

fn encode_transfer_function_scalar(
    writer: &mut ResultDigestWriter,
    scalar: &TransferFunctionScalarEvidence,
) {
    match scalar {
        TransferFunctionScalarEvidence::Finite(value) => {
            writer.u8(0);
            writer.f64(*value);
        }
        TransferFunctionScalarEvidence::PositiveInfinity => writer.u8(1),
        TransferFunctionScalarEvidence::NegativeInfinity => writer.u8(2),
    }
}

const fn transfer_function_quantity_tag(quantity: TransferFunctionQuantityEvidence) -> u8 {
    match quantity {
        TransferFunctionQuantityEvidence::Voltage => 0,
        TransferFunctionQuantityEvidence::Current => 1,
    }
}

const fn transfer_function_normalization_tag(
    normalization: TransferFunctionNormalizationEvidence,
) -> u8 {
    match normalization {
        TransferFunctionNormalizationEvidence::None => 0,
        TransferFunctionNormalizationEvidence::RelativeToNominal => 1,
        TransferFunctionNormalizationEvidence::PerSourceUnit => 2,
    }
}

const fn transfer_function_accuracy_tag(accuracy: TransferFunctionAccuracyEvidence) -> u8 {
    match accuracy {
        TransferFunctionAccuracyEvidence::Fast => 0,
        TransferFunctionAccuracyEvidence::Balanced => 1,
        TransferFunctionAccuracyEvidence::Accurate => 2,
        TransferFunctionAccuracyEvidence::Robust => 3,
    }
}

const fn soa_parameter_tag(parameter: SoaParameterEvidence) -> u8 {
    match parameter {
        SoaParameterEvidence::GateSourceVoltage => 0,
        SoaParameterEvidence::DrainSourceVoltage => 1,
        SoaParameterEvidence::GateDrainVoltage => 2,
        SoaParameterEvidence::BaseEmitterVoltage => 3,
        SoaParameterEvidence::CollectorEmitterVoltage => 4,
        SoaParameterEvidence::BaseCollectorVoltage => 5,
        SoaParameterEvidence::DrainCurrent => 6,
        SoaParameterEvidence::CollectorCurrent => 7,
        SoaParameterEvidence::PowerDissipation => 8,
        SoaParameterEvidence::Temperature => 9,
        SoaParameterEvidence::GateSourceVoltagePositive => 10,
        SoaParameterEvidence::GateSourceVoltageNegative => 11,
        SoaParameterEvidence::DrainSourceVoltagePositive => 12,
        SoaParameterEvidence::DrainSourceVoltageNegative => 13,
        SoaParameterEvidence::GateDrainVoltagePositive => 14,
        SoaParameterEvidence::GateDrainVoltageNegative => 15,
        SoaParameterEvidence::BaseEmitterVoltagePositive => 16,
        SoaParameterEvidence::BaseEmitterVoltageNegative => 17,
        SoaParameterEvidence::CollectorEmitterVoltagePositive => 18,
        SoaParameterEvidence::CollectorEmitterVoltageNegative => 19,
        SoaParameterEvidence::BaseCollectorVoltagePositive => 20,
        SoaParameterEvidence::BaseCollectorVoltageNegative => 21,
        SoaParameterEvidence::DrainCurrentPositive => 22,
        SoaParameterEvidence::DrainCurrentNegative => 23,
        SoaParameterEvidence::CollectorCurrentPositive => 24,
        SoaParameterEvidence::CollectorCurrentNegative => 25,
        SoaParameterEvidence::GateCurrent => 26,
        SoaParameterEvidence::GateCurrentPositive => 27,
        SoaParameterEvidence::GateCurrentNegative => 28,
        SoaParameterEvidence::SourceCurrent => 29,
        SoaParameterEvidence::SourceCurrentPositive => 30,
        SoaParameterEvidence::SourceCurrentNegative => 31,
        SoaParameterEvidence::BaseCurrent => 32,
        SoaParameterEvidence::BaseCurrentPositive => 33,
        SoaParameterEvidence::BaseCurrentNegative => 34,
        SoaParameterEvidence::EmitterCurrent => 35,
        SoaParameterEvidence::EmitterCurrentPositive => 36,
        SoaParameterEvidence::EmitterCurrentNegative => 37,
        SoaParameterEvidence::BodySourceVoltage => 38,
        SoaParameterEvidence::BodySourceVoltagePositive => 39,
        SoaParameterEvidence::BodySourceVoltageNegative => 40,
        SoaParameterEvidence::BodyDrainVoltage => 41,
        SoaParameterEvidence::BodyDrainVoltagePositive => 42,
        SoaParameterEvidence::BodyDrainVoltageNegative => 43,
        SoaParameterEvidence::GateBodyVoltage => 44,
        SoaParameterEvidence::GateBodyVoltagePositive => 45,
        SoaParameterEvidence::GateBodyVoltageNegative => 46,
        SoaParameterEvidence::BulkCurrent => 47,
        SoaParameterEvidence::BulkCurrentPositive => 48,
        SoaParameterEvidence::BulkCurrentNegative => 49,
        SoaParameterEvidence::BackgateSourceVoltage => 50,
        SoaParameterEvidence::BackgateSourceVoltagePositive => 51,
        SoaParameterEvidence::BackgateSourceVoltageNegative => 52,
        SoaParameterEvidence::BackgateDrainVoltage => 53,
        SoaParameterEvidence::BackgateDrainVoltagePositive => 54,
        SoaParameterEvidence::BackgateDrainVoltageNegative => 55,
        SoaParameterEvidence::GateBackgateVoltage => 56,
        SoaParameterEvidence::GateBackgateVoltagePositive => 57,
        SoaParameterEvidence::GateBackgateVoltageNegative => 58,
        SoaParameterEvidence::BackgateCurrent => 59,
        SoaParameterEvidence::BackgateCurrentPositive => 60,
        SoaParameterEvidence::BackgateCurrentNegative => 61,
        SoaParameterEvidence::BodyBackgateVoltage => 62,
        SoaParameterEvidence::BodyBackgateVoltagePositive => 63,
        SoaParameterEvidence::BodyBackgateVoltageNegative => 64,
        SoaParameterEvidence::CollectorSubstrateVoltage => 65,
        SoaParameterEvidence::CollectorSubstrateVoltagePositive => 66,
        SoaParameterEvidence::CollectorSubstrateVoltageNegative => 67,
        SoaParameterEvidence::BaseSubstrateVoltage => 68,
        SoaParameterEvidence::BaseSubstrateVoltagePositive => 69,
        SoaParameterEvidence::BaseSubstrateVoltageNegative => 70,
        SoaParameterEvidence::EmitterSubstrateVoltage => 71,
        SoaParameterEvidence::EmitterSubstrateVoltagePositive => 72,
        SoaParameterEvidence::EmitterSubstrateVoltageNegative => 73,
        SoaParameterEvidence::SubstrateCurrent => 74,
        SoaParameterEvidence::SubstrateCurrentPositive => 75,
        SoaParameterEvidence::SubstrateCurrentNegative => 76,
        SoaParameterEvidence::AnodeCathodeVoltage => 77,
        SoaParameterEvidence::AnodeCathodeVoltagePositive => 78,
        SoaParameterEvidence::AnodeCathodeVoltageNegative => 79,
        SoaParameterEvidence::AnodeCurrent => 80,
        SoaParameterEvidence::AnodeCurrentPositive => 81,
        SoaParameterEvidence::AnodeCurrentNegative => 82,
    }
}

const fn soa_rule_verdict_tag(verdict: SoaRuleVerdictEvidence) -> u8 {
    match verdict {
        SoaRuleVerdictEvidence::Pass => 0,
        SoaRuleVerdictEvidence::Warning => 1,
        SoaRuleVerdictEvidence::Violation => 2,
        SoaRuleVerdictEvidence::Critical => 3,
    }
}

const fn soa_violation_severity_tag(severity: SoaViolationSeverityEvidence) -> u8 {
    match severity {
        SoaViolationSeverityEvidence::Warning => 0,
        SoaViolationSeverityEvidence::Violation => 1,
        SoaViolationSeverityEvidence::Critical => 2,
    }
}

/// Every field of a sensitivity study, in declaration order, unconditionally.
///
/// Nothing here is skipped when it is empty or defaulted: the filter that
/// selected the variables and the grid they were solved on are what make two
/// studies of one deck different runs, and a digest that dropped either would
/// present one as the other.
fn encode_sensitivity_study_evidence(
    writer: &mut ResultDigestWriter,
    evidence: &SensitivityStudyEvidence,
) {
    writer.string(&evidence.output);
    writer.string(&evidence.filter);
    match &evidence.basis {
        SensitivityBasisEvidence::Dc { output } => {
            writer.u8(0);
            writer.f64(*output);
        }
        SensitivityBasisEvidence::Ac {
            frequencies_hz,
            output,
        } => {
            writer.u8(1);
            writer.sequence(frequencies_hz.len());
            for frequency in frequencies_hz {
                writer.f64(*frequency);
            }
            writer.sequence(output.len());
            for value in output {
                writer.f64(value.real);
                writer.f64(value.imaginary);
            }
        }
    }
    writer.sequence(evidence.rows.len());
    for row in &evidence.rows {
        writer.string(&row.parameter);
        writer.f64(row.nominal_value);
        for column in [&row.raw, &row.normalized, &row.phase] {
            writer.sequence(column.len());
            for value in column {
                match value {
                    rspice_core::analysis::sensitivity::SensitivityValue::Available(value) => {
                        writer.u8(0);
                        writer.f64(*value);
                    }
                    rspice_core::analysis::sensitivity::SensitivityValue::Unavailable {
                        unavailable,
                    } => {
                        writer.u8(1);
                        writer.string(unavailable.as_str());
                    }
                }
            }
        }
    }
}

fn encode_fft_spectrum_evidence(writer: &mut ResultDigestWriter, spectrum: &FftSpectrumEvidence) {
    match spectrum.status {
        FftSpectrumStatusEvidence::Complete => writer.u8(0),
        FftSpectrumStatusEvidence::IncompleteHistory {
            available_start_s,
            available_stop_s,
        } => {
            writer.u8(1);
            writer.f64(available_start_s);
            writer.f64(available_stop_s);
        }
    }
    writer.string(&spectrum.output);
    writer.string(&spectrum.physical_type);
    writer.f64(spectrum.start_time_s);
    writer.f64(spectrum.stop_time_s);
    writer.f64(spectrum.sample_interval_s);
    writer.usize(spectrum.point_count);
    writer.bool(spectrum.accurate_sampling);
    writer.string(spectrum.format.keyword());
    writer.string(spectrum.mode.label());
    writer.string(&spectrum.window);
    writer.f64(spectrum.alpha);
    writer.f64(spectrum.coherent_gain);
    writer.f64(spectrum.frequency_resolution_hz);
    writer.usize(spectrum.fundamental_bin);
    writer.usize(spectrum.minimum_metric_bin);
    writer.usize(spectrum.maximum_metric_bin);
    writer.option(spectrum.metrics.as_ref(), |writer, metrics| {
        for value in [
            metrics.fundamental_magnitude,
            metrics.thd_ratio,
            metrics.thd_db,
            metrics.sndr_db,
            metrics.enob_bits,
            metrics.snr_db,
            metrics.sfdr_db,
        ] {
            writer.f64(value);
        }
        writer.option(metrics.sfdr_spur_bin.as_ref(), |writer, bin| {
            writer.usize(*bin);
        });
        writer.option(
            metrics.sfdr_spur_frequency_hz.as_ref(),
            |writer, frequency| writer.f64(*frequency),
        );
        writer.sequence(metrics.largest_harmonics.len());
        for harmonic in &metrics.largest_harmonics {
            writer.usize(harmonic.rank);
            writer.usize(harmonic.bin);
            writer.f64(harmonic.frequency_hz);
            writer.f64(harmonic.magnitude);
            writer.f64(harmonic.magnitude_db);
            writer.f64(harmonic.phase_degrees);
        }
    });
}

fn encode_complex_result_values(writer: &mut ResultDigestWriter, values: &[ComplexResultValue]) {
    writer.sequence(values.len());
    for value in values {
        encode_complex_result_value(writer, *value);
    }
}

fn encode_complex_result_value(writer: &mut ResultDigestWriter, value: ComplexResultValue) {
    writer.f64(value.real);
    writer.f64(value.imaginary);
}

fn encode_floquet_spectrum_evidence(
    writer: &mut ResultDigestWriter,
    evidence: &FloquetSpectrumEvidence,
) {
    match evidence {
        FloquetSpectrumEvidence::NotComputed => writer.u8(0),
        FloquetSpectrumEvidence::NoDynamicModes => writer.u8(1),
        FloquetSpectrumEvidence::Qualified { certificate } => {
            writer.u8(2);
            encode_floquet_spectrum_certificate(writer, *certificate);
        }
        FloquetSpectrumEvidence::LegacyUnknown => writer.u8(3),
    }
}

fn encode_floquet_spectrum_certificate(
    writer: &mut ResultDigestWriter,
    certificate: FloquetSpectrumCertificateEvidence,
) {
    writer.u64(certificate.problem_order);
    writer.f64(certificate.max_backward_error);
    writer.f64(certificate.qualification_tolerance);
}

const fn floquet_orbit_kind_tag(orbit_kind: FloquetOrbitKindEvidence) -> u8 {
    match orbit_kind {
        FloquetOrbitKindEvidence::Driven => 0,
        FloquetOrbitKindEvidence::Autonomous => 1,
        FloquetOrbitKindEvidence::LegacyUnknown => 2,
    }
}

const fn floquet_stability_verdict_tag(verdict: FloquetStabilityVerdictEvidence) -> u8 {
    match verdict {
        FloquetStabilityVerdictEvidence::Stable => 0,
        FloquetStabilityVerdictEvidence::Unstable => 1,
        FloquetStabilityVerdictEvidence::Marginal => 2,
        FloquetStabilityVerdictEvidence::Indeterminate => 3,
    }
}

const fn pstb_stability_classification_tag(
    classification: PstbStabilityClassificationEvidence,
) -> u8 {
    match classification {
        PstbStabilityClassificationEvidence::Stable => 0,
        PstbStabilityClassificationEvidence::UnstableReal => 1,
        PstbStabilityClassificationEvidence::UnstableComplex => 2,
        PstbStabilityClassificationEvidence::PeriodDoubling => 3,
        PstbStabilityClassificationEvidence::NeimarkSacker => 4,
        PstbStabilityClassificationEvidence::SaddleNode => 5,
        PstbStabilityClassificationEvidence::Marginal => 6,
        PstbStabilityClassificationEvidence::Indeterminate => 7,
    }
}

fn encode_pole_zero_root_evidence(
    writer: &mut ResultDigestWriter,
    evidence: &PoleZeroRootSetEvidence,
) {
    match evidence {
        PoleZeroRootSetEvidence::NotRequested => writer.u8(0),
        PoleZeroRootSetEvidence::QualifiedEmpty { certificate } => {
            writer.u8(if certificate.asymptotically_stable.is_some() {
                5
            } else {
                1
            });
            encode_pole_zero_certificate(writer, *certificate);
        }
        PoleZeroRootSetEvidence::Qualified { certificate } => {
            writer.u8(if certificate.asymptotically_stable.is_some() {
                6
            } else {
                2
            });
            encode_pole_zero_certificate(writer, *certificate);
        }
        PoleZeroRootSetEvidence::Approximate { certificate } => {
            writer.u8(if certificate.asymptotically_stable.is_some() {
                7
            } else {
                3
            });
            encode_pole_zero_certificate(writer, *certificate);
        }
        PoleZeroRootSetEvidence::LegacyUnknown => writer.u8(4),
    }
}

fn encode_pole_zero_certificate(
    writer: &mut ResultDigestWriter,
    certificate: PoleZeroSpectrumCertificate,
) {
    writer.u64(certificate.problem_order);
    writer.u64(certificate.infinite_count);
    writer.f64(certificate.max_backward_error);
    writer.f64(certificate.qualification_tolerance);
    // New evidence tags distinguish this extension; absent evidence preserves
    // the published legacy digest bytes exactly.
    if let Some(stable) = certificate.asymptotically_stable {
        writer.bool(stable);
    }
}

fn encode_dc_op(writer: &mut ResultDigestWriter, result: &DcOpResult) {
    encode_operating_point_values(writer, &result.node_voltages);
    encode_operating_point_values(writer, &result.branch_currents);
    encode_operating_point_values(writer, &result.power_dissipation);
}

fn encode_operating_point_values(writer: &mut ResultDigestWriter, values: &[OperatingPointValue]) {
    writer.sequence(values.len());
    for value in values {
        writer.string(&value.name);
        writer.f64(value.value);
        writer.string(&value.unit);
    }
}

fn encode_device_op(
    writer: &mut ResultDigestWriter,
    report: &rspice_core::circuit::DeviceOpReport,
) {
    writer.sequence(report.entries.len());
    for entry in &report.entries {
        writer.string(&entry.name);
        writer.string(entry.device_kind);
        writer.option(entry.region, |writer, region| writer.string(region));
        writer.sequence(entry.params.len());
        for (name, value) in &entry.params {
            writer.string(name);
            writer.f64(*value);
        }
    }
}

fn encode_noise_summary(
    writer: &mut ResultDigestWriter,
    summary: &NoiseSummary,
    encoding_version: ResultDigestEncoding,
) {
    if encoding_version <= ResultDigestEncoding::V4 {
        // Schemas v1-v11 required this scalar and encoded it directly. The
        // migration boundary rejects absence before invoking this legacy
        // encoder; NaN keeps this helper total for digest-focused unit tests.
        writer.f64(summary.total_rms.unwrap_or(f64::NAN));
    } else {
        writer.option(summary.total_rms.as_ref(), |writer, value| {
            writer.f64(*value)
        });
        writer.option(summary.input_rms.as_ref(), |writer, value| {
            writer.f64(*value)
        });
    }
    writer.f64(summary.band.0);
    writer.f64(summary.band.1);
    writer.sequence(summary.rows.len());
    for row in &summary.rows {
        writer.string(&row.device);
        writer.string(&row.mechanism);
        writer.f64(row.power);
        writer.f64(row.share_pct);
    }
    if let Some(quantity) = summary.input_quantity {
        writer.string("noise-input-quantity-v1");
        writer.string(quantity.density_unit());
    }
    if let Some(conversion) = &summary.conversion {
        writer.string("periodic-noise-conversion-v1");
        writer.string(&conversion.input_source);
        writer.f64(conversion.carrier_hz);
        writer.i64(i64::from(conversion.input_sideband));
        writer.i64(i64::from(conversion.output_sideband));
        writer.i64(i64::from(conversion.max_sideband));
        if let Some(sampling) = &conversion.sampling {
            writer.string("pnoise-sampling-evidence-v1");
            writer.f64(sampling.carrier_frequency_hz);
            encode_pnoise_sampling(writer, &sampling.request);
            encode_pnoise_sample_point(writer, &sampling.output);
            writer.option(sampling.reference.as_ref(), encode_pnoise_sample_point);
            writer.option(sampling.nominal_delay_seconds.as_ref(), |writer, value| {
                writer.f64(*value)
            });
        }
    }
    if let Some(figure) = &summary.noise_figure {
        writer.string("source-referenced-ssb-noise-figure-v1");
        writer.string(&figure.input_source);
        writer.string(&figure.source_resistor);
        writer.f64(figure.source_resistance_ohm);
        writer.f64(figure.source_temperature_kelvin);
        writer.f64(figure.reference_temperature_kelvin);
        writer.f64_slice(&figure.frequencies);
        writer.f64_slice(&figure.decibels);
    }
}

fn encode_family_metadata(
    writer: &mut ResultDigestWriter,
    metadata: &AnalysisResultFamilyMetadata,
) {
    match metadata {
        AnalysisResultFamilyMetadata::Parametric {
            target,
            sweep_values,
            failed_points,
            ..
        } => {
            writer.u8(0);
            writer.string(target);
            writer.f64_slice(sweep_values);
            writer.usize(*failed_points);
        }
        AnalysisResultFamilyMetadata::Corner {
            x_values,
            x_label,
            x_unit,
            temperatures_c,
            corner_labels,
            failed_corners,
            ..
        } => {
            writer.u8(1);
            writer.f64_slice(x_values);
            writer.string(x_label);
            writer.string(x_unit);
            writer.f64_slice(temperatures_c);
            writer.sequence(corner_labels.len());
            for label in corner_labels {
                writer.string(label);
            }
            writer.usize(*failed_corners);
        }
        AnalysisResultFamilyMetadata::MonteCarlo {
            seed,
            runs_requested,
            runs_completed,
            failures,
            all_converged,
            variables,
            ..
        } => {
            let has_confidence = variables
                .iter()
                .any(|variable| variable.mean_confidence.is_some());
            // Tag 2 is frozen for historic results with no mean confidence.
            writer.u8(if has_confidence { 9 } else { 2 });
            writer.u64(*seed);
            writer.usize(*runs_requested);
            writer.usize(*runs_completed);
            writer.usize(*failures);
            writer.bool(*all_converged);
            writer.sequence(variables.len());
            for variable in variables {
                writer.string(&variable.name);
                writer.f64_slice(&variable.samples);
                writer.f64(variable.mean);
                writer.f64(variable.std_dev);
                writer.f64(variable.min);
                writer.f64(variable.max);
                if has_confidence {
                    writer.option(
                        variable.mean_confidence.as_ref(),
                        encode_monte_carlo_mean_confidence,
                    );
                }
            }
        }
        AnalysisResultFamilyMetadata::Optimization {
            iterations,
            best_cost,
            best_variables,
            best_objectives,
            best_constraints,
            converged,
        } => {
            writer.u8(4);
            writer.f64_slice(iterations);
            writer.f64(*best_cost);
            writer.sequence(best_variables.len());
            for (name, value) in best_variables {
                writer.string(name);
                writer.f64(*value);
            }
            writer.bool(*converged);
            if !best_constraints.is_empty() {
                writer.string("optimization-constraints/v1");
                writer.sequence(best_constraints.len());
                for observation in best_constraints {
                    let term = &observation.constraint;
                    writer.string(&term.measurement);
                    if !term.unit.is_empty() {
                        writer.string("optimization-measurement-unit/v1");
                        writer.string(&term.unit);
                    }
                    writer.bool(term.lower.is_some());
                    if let Some(value) = term.lower {
                        writer.f64(value);
                    }
                    writer.bool(term.upper.is_some());
                    if let Some(value) = term.upper {
                        writer.f64(value);
                    }
                    writer.f64(term.tolerance);
                    writer.f64(term.scale);
                    writer.f64(observation.value);
                    writer.f64(observation.violation);
                }
            }
            if !best_objectives.is_empty() {
                writer.string("weighted-optimization-objectives/v1");
                writer.sequence(best_objectives.len());
                for observation in best_objectives {
                    let term = &observation.objective;
                    writer.string(&term.measurement);
                    if !term.unit.is_empty() {
                        writer.string("optimization-measurement-unit/v1");
                        writer.string(&term.unit);
                    }
                    writer.u8(match term.goal {
                        crate::optimization::OptimizationObjectiveGoal::Minimize => 0,
                        crate::optimization::OptimizationObjectiveGoal::Maximize => 1,
                        crate::optimization::OptimizationObjectiveGoal::Target => 2,
                    });
                    writer.bool(term.target.is_some());
                    if let Some(target) = term.target {
                        writer.f64(target);
                    }
                    writer.f64(term.scale);
                    writer.f64(term.weight);
                    writer.f64(observation.value);
                    writer.f64(observation.contribution);
                }
            }
        }
        AnalysisResultFamilyMetadata::Soa { time } => {
            writer.u8(5);
            writer.f64_slice(time);
        }
        AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity,
            carrier_frequency_hz,
        } => {
            writer.u8(6);
            writer.u8(match output_quantity {
                PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity => 0,
                PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz => 1,
                PeriodicNoiseOutputQuantity::TimingNoisePowerSpectralDensity => 2,
            });
            writer.option(carrier_frequency_hz.as_ref(), |writer, frequency| {
                writer.f64(*frequency)
            });
        }
        AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
            noise_reference_temperature_kelvin,
        } => {
            // Preserve the encoding of existing scattering-only results.
            writer.u8(if noise_reference_temperature_kelvin.is_some() {
                8
            } else {
                7
            });
            writer.f64_slice(reference_impedances_ohm);
            if let Some(temperature) = noise_reference_temperature_kelvin {
                writer.f64(*temperature);
            }
        }
    }
    encode_member_measurements(writer, metadata.member_measurements());
}

/// A family's per-member evidence, appended to the family's own encoding.
///
/// Written only when the family has members. A family that measured none
/// encodes exactly the bytes it always did, so every digest taken before
/// families retained member evidence still matches the result it was taken
/// from — the empty carriage *is* the old content, and giving it new bytes
/// would restate every historical result as changed.
fn encode_member_measurements(
    writer: &mut ResultDigestWriter,
    members: &[crate::family_measurements::FamilyMemberMeasurements],
) {
    use crate::family_measurements::FamilyMemberId;

    if members.is_empty() {
        return;
    }
    writer.sequence(members.len());
    for member in members {
        match &member.member {
            FamilyMemberId::MonteCarloTrial { index, seed } => {
                writer.u8(0);
                writer.usize(*index);
                writer.u64(*seed);
            }
            FamilyMemberId::MonteCarloSequenceTrial {
                index,
                seed,
                policy,
            } => {
                writer.u8(3);
                writer.usize(*index);
                writer.u64(*seed);
                writer.string(policy);
            }
            FamilyMemberId::SweepPoint { index, value } => {
                writer.u8(1);
                writer.usize(*index);
                writer.f64(*value);
            }
            FamilyMemberId::Corner { index, label } => {
                writer.u8(2);
                writer.usize(*index);
                writer.string(label);
            }
        }
        writer.sequence(member.measurements.len());
        for measurement in &member.measurements {
            writer.string(&measurement.name);
            writer.option(measurement.value.as_ref(), |writer, value| {
                writer.f64(*value);
            });
            writer.bool(measurement.passed);
            writer.option(measurement.error.as_deref(), |writer, error| {
                writer.string(error);
            });
        }
    }
}

fn encode_saved_output_status(
    writer: &mut ResultDigestWriter,
    status: &SavedOutputMaterializationStatus,
) {
    match status {
        SavedOutputMaterializationStatus::Materialized {
            waveform_name,
            sample_count,
        } => {
            writer.u8(0);
            writer.string(waveform_name);
            writer.u64(*sample_count);
        }
        SavedOutputMaterializationStatus::Deferred => writer.u8(1),
        SavedOutputMaterializationStatus::SuppressedOnSuccess => writer.u8(2),
        SavedOutputMaterializationStatus::Unavailable { reason } => {
            writer.u8(3);
            writer.string(reason);
        }
        SavedOutputMaterializationStatus::MaterializedDcFamily { members } => {
            writer.u8(4);
            writer.sequence(members.len());
            for member in members {
                writer.usize(member.member);
                writer.string(&member.waveform_name);
                writer.u64(member.sample_count);
            }
        }
    }
}

const fn analysis_type_tag(analysis_type: AnalysisType) -> u8 {
    match analysis_type {
        AnalysisType::DcOp => 0,
        AnalysisType::DcSweep => 1,
        AnalysisType::Ac => 2,
        AnalysisType::Disto => 3,
        AnalysisType::Transient => 4,
        AnalysisType::Noise => 5,
        AnalysisType::PoleZero => 6,
        AnalysisType::Tf => 7,
        AnalysisType::Sensitivity => 8,
        AnalysisType::Pac => 9,
        AnalysisType::Pnoise => 10,
        AnalysisType::Pxf => 11,
        AnalysisType::Pstb => 12,
        AnalysisType::Stb => 13,
        AnalysisType::MonteCarlo => 14,
        AnalysisType::Parametric => 15,
        AnalysisType::Corner => 16,
        AnalysisType::Optimization => 18,
        AnalysisType::Soa => 19,
        AnalysisType::SParameter => 20,
        AnalysisType::Envelope => 21,
        AnalysisType::Fourier => 22,
        AnalysisType::HarmonicBalance => 23,
        AnalysisType::Pss => 24,
        AnalysisType::Qpss => 25,
        AnalysisType::Hbsp => 26,
        AnalysisType::Hbnoise => 27,
        AnalysisType::Psp => 28,
        AnalysisType::Qpac => 29,
        AnalysisType::Qpnoise => 30,
        AnalysisType::Qpxf => 31,
        AnalysisType::TransientNoise => 32,
        AnalysisType::DcMismatch => 33,
    }
}

const fn saved_output_kind_tag(kind: SavedOutputKind) -> u8 {
    match kind {
        SavedOutputKind::RawVoltageOrCurrent => 0,
        SavedOutputKind::DerivedExpression => 1,
        SavedOutputKind::DeviceOperatingPointQuantity => 2,
        SavedOutputKind::NoiseContributor => 3,
        SavedOutputKind::RfPortQuantity => 4,
    }
}

const fn saved_output_policy_tag(policy: SavedOutputPolicy) -> u8 {
    match policy {
        SavedOutputPolicy::EveryAcceptedPoint => 0,
        SavedOutputPolicy::SelectedAndFinalPoints => 1,
        SavedOutputPolicy::OnDemandFromRetainedState => 2,
        SavedOutputPolicy::FailureDiagnosticsOnly => 3,
    }
}

const fn saved_output_precision_tag(precision: SavedOutputPrecision) -> u8 {
    match precision {
        SavedOutputPrecision::FullSourcePrecision => 0,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision => 1,
    }
}

const fn saved_output_streaming_tag(streaming: SavedOutputStreaming) -> u8 {
    match streaming {
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation => 0,
        SavedOutputStreaming::StoreOnly => 1,
    }
}

fn encode_monte_carlo_mean_confidence(
    writer: &mut ResultDigestWriter,
    evidence: &MonteCarloMeanConfidence,
) {
    use crate::monte_carlo::{MonteCarloMeanInterval, MonteCarloMeanMethod};
    writer.f64(evidence.level_pct);
    match evidence.method {
        MonteCarloMeanMethod::StudentT => writer.u8(0),
        MonteCarloMeanMethod::PercentileBootstrap { resamples, seed } => {
            writer.u8(1);
            writer.usize(resamples);
            writer.u64(seed);
        }
    }
    writer.usize(evidence.successful_samples);
    writer.bool(evidence.conditional_on_successful_trials);
    match evidence.interval {
        MonteCarloMeanInterval::Available { lower, upper } => {
            writer.u8(0);
            writer.f64(lower);
            writer.f64(upper);
        }
        MonteCarloMeanInterval::InsufficientSamples => writer.u8(1),
        MonteCarloMeanInterval::Unrepresentable => writer.u8(2),
    }
}

fn encode_pnoise_sampling(
    writer: &mut ResultDigestWriter,
    sampling: &rspice_core::analysis::pnoise::PeriodicNoiseSampling,
) {
    use rspice_core::analysis::pnoise::{
        PeriodicNoiseEdge, PeriodicNoiseEdgeDirection, PeriodicNoiseSampling,
    };
    fn edge(writer: &mut ResultDigestWriter, edge: &PeriodicNoiseEdge) {
        writer.f64(edge.threshold_volts);
        writer.u8(match edge.direction {
            PeriodicNoiseEdgeDirection::Rising => 0,
            PeriodicNoiseEdgeDirection::Falling => 1,
            PeriodicNoiseEdgeDirection::Either => 2,
        });
        writer.usize(edge.occurrence);
        writer.f64(edge.phase_tolerance_degrees);
        writer.f64(edge.minimum_slew_volts_per_second);
    }
    match sampling {
        PeriodicNoiseSampling::Phase { phase_degrees } => {
            writer.u8(0);
            writer.f64(*phase_degrees);
        }
        PeriodicNoiseSampling::Edge { edge: output } => {
            writer.u8(1);
            edge(writer, output);
        }
        PeriodicNoiseSampling::Delay {
            edge: output,
            reference_node,
            reference_ref,
            reference_edge,
            periods,
        } => {
            writer.u8(2);
            edge(writer, output);
            writer.string(reference_node);
            writer.option(reference_ref.as_ref(), |writer, name| writer.string(name));
            edge(writer, reference_edge);
            writer.u64(u64::from(*periods));
        }
    }
}

fn encode_pnoise_sample_point(
    writer: &mut ResultDigestWriter,
    point: &rspice_core::analysis::pnoise::PeriodicNoiseSamplePoint,
) {
    writer.string(&point.node);
    writer.option(point.reference.as_ref(), |writer, name| writer.string(name));
    writer.f64(point.phase_degrees);
    writer.f64(point.voltage);
    writer.f64(point.slew_volts_per_second);
}

#[cfg(test)]
mod tests;
