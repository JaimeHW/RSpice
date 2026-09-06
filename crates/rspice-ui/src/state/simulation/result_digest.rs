//! Canonical content identity for immutable retained result data.
//!
//! This encoding is deliberately independent of serde, `Debug`, collection
//! allocation, and host word size. Presentation-only waveform attributes are
//! excluded: a color, visibility, or display-cache change does not create a
//! different source dataset. Authoritative f64/complex128 values are encoded
//! in IEEE-754 binary64 form after normalizing signed zero and NaN payloads.

use sha2::{Digest as _, Sha256};

use super::analysis_result::{
    FloquetOrbitKindEvidence, FloquetSpectrumCertificateEvidence, FloquetSpectrumEvidence,
    FloquetStabilityVerdictEvidence, PstbStabilityClassificationEvidence,
};
use super::*;
use crate::product::ContentDigest;
use crate::state::{
    SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
};

const RESULT_DIGEST_MAGIC: &[u8] = b"RSPICE-RESULT-DATA";
const RESULT_DIGEST_ENCODING_VERSION_V1: u16 = 1;
const RESULT_DIGEST_ENCODING_VERSION_V2: u16 = 2;
const RESULT_DIGEST_ENCODING_VERSION_V3: u16 = 3;
const RESULT_DIGEST_ENCODING_VERSION_V4: u16 = 4;
const RESULT_DIGEST_ENCODING_VERSION_V5: u16 = 5;
const RESULT_DIGEST_ENCODING_VERSION_V6: u16 = 6;
const RESULT_DIGEST_ENCODING_VERSION_V7: u16 = 7;
const RESULT_DIGEST_ENCODING_VERSION_V8: u16 = 8;
const RESULT_DIGEST_ENCODING_VERSION_V9: u16 = 9;
/// 2026-09-06: the digital bus table declared over a transient's retained
/// event traces. A declaration is data, not presentation — the same members
/// read as one 8-bit word rather than as eight conductors is a different
/// claim about the run — so a result that gained one is a different result.
const RESULT_DIGEST_ENCODING_VERSION_V10: u16 = 10;
const CANONICAL_NAN_BITS: u64 = 0x7ff8_0000_0000_0000;

struct ResultDigestWriter {
    hasher: Sha256,
    retained_bytes: u64,
}

impl ResultDigestWriter {
    fn new(domain: &str, encoding_version: u16) -> Self {
        let mut writer = Self {
            hasher: Sha256::new(),
            retained_bytes: 0,
        };
        writer.raw(RESULT_DIGEST_MAGIC);
        writer.raw(&encoding_version.to_be_bytes());
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

impl AnalysisResult {
    /// SHA-256 identity of the authoritative retained data and evidence for
    /// this analysis. Display labels, timestamps, colors, visibility, and
    /// derived display caches are intentionally not part of the identity.
    #[must_use]
    pub fn result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V10)
    }

    /// Logical bytes occupied by all authoritative retained result evidence.
    ///
    /// This deliberately follows the same complete field walk as the result
    /// digest, so adding an authenticated payload, measurement, waveform, or
    /// saved-output receipt cannot silently escape runtime retention budgets.
    /// Presentation caches are added separately because they are intentionally
    /// excluded from immutable content identity.
    #[must_use]
    pub fn retained_storage_bytes(&self) -> u64 {
        let writer = self.result_data_writer_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V10);
        let cache_bytes = self.waveforms.iter().fold(0_u64, |total, waveform| {
            let bytes = waveform.display_cache.as_ref().map_or(0_u64, |cache| {
                u64::try_from(cache.x.len())
                    .unwrap_or(u64::MAX)
                    .saturating_add(u64::try_from(cache.y.len()).unwrap_or(u64::MAX))
                    .saturating_mul(std::mem::size_of::<f32>() as u64)
                    .saturating_add(std::mem::size_of::<usize>() as u64)
            });
            total.saturating_add(bytes)
        });
        writer.retained_bytes().saturating_add(cache_bytes)
    }

    /// Schema-v8 digest retained solely for authenticated migration. New
    /// result documents must use [`Self::result_data_digest`].
    #[must_use]
    pub(crate) fn legacy_v1_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V1)
    }

    /// Schema-v9 digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v2_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V2)
    }

    /// Schema-v10 digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v3_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V3)
    }

    /// Schema-v11 digest retained solely for authenticated migration. Its
    /// noise-summary encoding predates optional output noise and
    /// input-referred integrated noise evidence.
    #[must_use]
    pub(crate) fn legacy_v4_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V4)
    }

    /// Schema-v12 digest retained solely for authenticated migration. Its
    /// waveform encoding predates the per-waveform retained unit.
    #[must_use]
    pub(crate) fn legacy_v5_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V5)
    }

    /// Schema-v13 through schema-v15 digest retained solely for authenticated
    /// migration. Its pole-zero payload required a numeric DC gain.
    #[must_use]
    pub(crate) fn legacy_v6_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V6)
    }

    /// Schema-v16 digest retained solely for authenticated migration. It
    /// predates durable PSS/PSTB Floquet payloads.
    #[must_use]
    pub(crate) fn legacy_v7_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V7)
    }

    /// Schema-v17 digest retained solely for authenticated migration. It
    /// predates durable measurement FAILVALUE verification evidence.
    #[must_use]
    pub(crate) fn legacy_v8_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V8)
    }

    /// Schema-v18 digest retained solely for authenticated migration. It
    /// predates the digital bus table declared over retained event traces.
    #[must_use]
    pub(crate) fn legacy_v9_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V9)
    }

    fn result_data_digest_with_encoding(&self, version: u16) -> ContentDigest {
        self.result_data_writer_with_encoding(version).finish()
    }

    fn result_data_writer_with_encoding(&self, version: u16) -> ResultDigestWriter {
        let domain = match version {
            RESULT_DIGEST_ENCODING_VERSION_V1 => "rspice.analysis-result-data/v1",
            RESULT_DIGEST_ENCODING_VERSION_V2 => "rspice.analysis-result-data/v2",
            RESULT_DIGEST_ENCODING_VERSION_V3 => "rspice.analysis-result-data/v3",
            RESULT_DIGEST_ENCODING_VERSION_V4 => "rspice.analysis-result-data/v4",
            RESULT_DIGEST_ENCODING_VERSION_V5 => "rspice.analysis-result-data/v5",
            RESULT_DIGEST_ENCODING_VERSION_V6 => "rspice.analysis-result-data/v6",
            RESULT_DIGEST_ENCODING_VERSION_V7 => "rspice.analysis-result-data/v7",
            RESULT_DIGEST_ENCODING_VERSION_V8 => "rspice.analysis-result-data/v8",
            RESULT_DIGEST_ENCODING_VERSION_V9 => "rspice.analysis-result-data/v9",
            RESULT_DIGEST_ENCODING_VERSION_V10 => "rspice.analysis-result-data/v10",
            _ => unreachable!("supported result digest encoding"),
        };
        let mut writer = ResultDigestWriter::new(domain, version);
        writer.u8(analysis_type_tag(self.analysis_type));
        writer.bool(self.success);
        writer.option(self.error_message.as_deref(), |writer, value| {
            writer.string(value);
        });

        writer.sequence(self.waveforms.len());
        for waveform in &self.waveforms {
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
            if version >= RESULT_DIGEST_ENCODING_VERSION_V6 {
                writer.option(waveform.unit.as_deref(), |writer, unit| {
                    writer.string(unit);
                });
            }
        }

        writer.option(self.dc_op.as_ref(), encode_dc_op);
        writer.option(self.device_op.as_ref(), encode_device_op);
        writer.option(self.noise_summary.as_ref(), |writer, summary| {
            encode_noise_summary(writer, summary, version);
        });
        writer.option(self.family_metadata.as_ref(), encode_family_metadata);
        if version >= RESULT_DIGEST_ENCODING_VERSION_V2 {
            writer.option(self.result_payload.as_ref(), |writer, payload| {
                encode_result_payload(writer, payload, version);
            });
        }

        writer.sequence(self.measurements.len());
        for measurement in &self.measurements {
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
            if version >= RESULT_DIGEST_ENCODING_VERSION_V9 {
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
        for receipt in &self.saved_output_receipts {
            writer.uuid(receipt.output_id.as_uuid());
            writer.u64(receipt.output_revision.get());
            writer.uuid(receipt.analysis_id.as_uuid());
            writer.digest(receipt.contract_digest);
            writer.string(&receipt.name);
            writer.string(&receipt.source_expression);
            writer.u8(saved_output_kind_tag(receipt.output_kind));
            writer.u8(saved_output_policy_tag(receipt.save_policy));
            writer.u8(saved_output_precision_tag(receipt.stored_precision));
            writer.u8(saved_output_streaming_tag(receipt.streaming));
            encode_saved_output_status(&mut writer, &receipt.status);
        }

        writer
    }
}

impl SimulationRun {
    /// SHA-256 identity of the ordered immutable analysis dataset retained by
    /// this run. Stable run/dataset IDs and wall-clock metadata are excluded;
    /// they address the dataset but do not define its sample content.
    #[must_use]
    pub fn dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V10)
    }

    /// Schema-v8 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v1_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V1)
    }

    /// Schema-v9 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v2_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V2)
    }

    /// Schema-v10 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v3_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V3)
    }

    /// Schema-v11 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v4_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V4)
    }

    /// Schema-v12 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v5_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V5)
    }

    /// Schema-v13 through schema-v15 dataset digest retained solely for
    /// authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v6_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V6)
    }

    /// Schema-v16 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v7_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V7)
    }

    /// Schema-v17 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v8_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V8)
    }

    /// Schema-v18 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v9_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(RESULT_DIGEST_ENCODING_VERSION_V9)
    }

    fn dataset_content_digest_with_encoding(&self, version: u16) -> ContentDigest {
        let domain = match version {
            RESULT_DIGEST_ENCODING_VERSION_V1 => "rspice.simulation-dataset-data/v1",
            RESULT_DIGEST_ENCODING_VERSION_V2 => "rspice.simulation-dataset-data/v2",
            RESULT_DIGEST_ENCODING_VERSION_V3 => "rspice.simulation-dataset-data/v3",
            RESULT_DIGEST_ENCODING_VERSION_V4 => "rspice.simulation-dataset-data/v4",
            RESULT_DIGEST_ENCODING_VERSION_V5 => "rspice.simulation-dataset-data/v5",
            RESULT_DIGEST_ENCODING_VERSION_V6 => "rspice.simulation-dataset-data/v6",
            RESULT_DIGEST_ENCODING_VERSION_V7 => "rspice.simulation-dataset-data/v7",
            RESULT_DIGEST_ENCODING_VERSION_V8 => "rspice.simulation-dataset-data/v8",
            RESULT_DIGEST_ENCODING_VERSION_V9 => "rspice.simulation-dataset-data/v9",
            RESULT_DIGEST_ENCODING_VERSION_V10 => "rspice.simulation-dataset-data/v10",
            _ => unreachable!("supported dataset digest encoding"),
        };
        let mut writer = ResultDigestWriter::new(domain, version);
        writer.sequence(self.analyses.len());
        for analysis in &self.analyses {
            writer.u64(analysis.id);
            writer.digest(match version {
                RESULT_DIGEST_ENCODING_VERSION_V1 => analysis.legacy_v1_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V2 => analysis.legacy_v2_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V3 => analysis.legacy_v3_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V4 => analysis.legacy_v4_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V5 => analysis.legacy_v5_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V6 => analysis.legacy_v6_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V7 => analysis.legacy_v7_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V8 => analysis.legacy_v8_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V9 => analysis.legacy_v9_result_data_digest(),
                RESULT_DIGEST_ENCODING_VERSION_V10 => analysis.result_data_digest(),
                _ => unreachable!("supported dataset digest encoding"),
            });
        }
        writer.finish()
    }
}

fn encode_result_payload(
    writer: &mut ResultDigestWriter,
    payload: &AnalysisResultPayload,
    encoding_version: u16,
) {
    match payload {
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
            if encoding_version >= RESULT_DIGEST_ENCODING_VERSION_V7 {
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
            if encoding_version < RESULT_DIGEST_ENCODING_VERSION_V8 {
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
            if encoding_version < RESULT_DIGEST_ENCODING_VERSION_V8 {
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
                writer.f64(row.raw);
                writer.f64(row.normalized);
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
        AnalysisResultPayload::Reliability { devices } => {
            writer.u8(3);
            writer.sequence(devices.len());
            for device in devices {
                writer.string(&device.device_id);
                writer.f64(device.stress.average_gate_stress_v);
                writer.f64(device.stress.average_drain_stress_v);
                writer.f64(device.stress.average_temperature_k);
                writer.f64(device.stress.duration_s);
                writer.sequence(device.checkpoints.len());
                for checkpoint in &device.checkpoints {
                    writer.f64(checkpoint.years);
                    let shift = &checkpoint.shift;
                    writer.f64(shift.threshold_voltage_shift_v);
                    writer.f64(shift.mobility_shift);
                    writer.f64(shift.drain_source_resistance_shift);
                }
            }
        }
        AnalysisResultPayload::Soa {
            evaluations,
            violations,
        } => {
            writer.u8(4);
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
        }
        AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            digital_buses,
        } => {
            writer.u8(7);
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
            if encoding_version >= RESULT_DIGEST_ENCODING_VERSION_V10 {
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
            writer.u8(1);
            encode_pole_zero_certificate(writer, *certificate);
        }
        PoleZeroRootSetEvidence::Qualified { certificate } => {
            writer.u8(2);
            encode_pole_zero_certificate(writer, *certificate);
        }
        PoleZeroRootSetEvidence::Approximate { certificate } => {
            writer.u8(3);
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
    encoding_version: u16,
) {
    if encoding_version <= RESULT_DIGEST_ENCODING_VERSION_V4 {
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
            writer.u8(2);
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
            }
        }
        AnalysisResultFamilyMetadata::Reliability { years } => {
            writer.u8(3);
            writer.f64_slice(years);
        }
        AnalysisResultFamilyMetadata::Optimization {
            iterations,
            best_cost,
            best_variables,
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
            });
            writer.option(carrier_frequency_hz.as_ref(), |writer, frequency| {
                writer.f64(*frequency)
            });
        }
        AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
        } => {
            writer.u8(7);
            writer.f64_slice(reference_impedances_ohm);
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
    members: &[crate::state::FamilyMemberMeasurements],
) {
    use crate::state::FamilyMemberId;

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
        AnalysisType::Reliability => 17,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::simulation::analysis_result::{
        PssFloquetMultiplierEvidence, PstbFloquetModeEvidence, PstbStabilityClassificationEvidence,
    };

    fn stable_pstb_result() -> AnalysisResult {
        let certificate = FloquetSpectrumCertificateEvidence {
            problem_order: 1,
            max_backward_error: 0.0,
            qualification_tolerance:
                FloquetSpectrumCertificateEvidence::canonical_qualification_tolerance(1).unwrap(),
        };
        let multiplier = ComplexResultValue {
            real: 0.5,
            imaginary: 0.0,
        };
        AnalysisResult::new(1, AnalysisType::Pstb, "PSTB").with_result_payload(
            AnalysisResultPayload::Pstb {
                period_s: Some(1.0),
                fundamental_frequency_hz: Some(1.0),
                stability_threshold: Some(1.0),
                probe_instance: Some("LPROBE".to_owned()),
                detect_subharmonics: Some(false),
                modes: vec![PstbFloquetModeEvidence {
                    multiplier,
                    exponent: ComplexResultValue {
                        real: 0.5_f64.ln(),
                        imaginary: 0.0,
                    },
                    probe_participation: 0.25,
                    is_unstable: false,
                    is_trivial: false,
                    subharmonic_order: None,
                }],
                floquet_evidence: FloquetSpectrumEvidence::Qualified { certificate },
                orbit_kind: FloquetOrbitKindEvidence::Driven,
                trivial_multiplier_index: None,
                stability_verdict: FloquetStabilityVerdictEvidence::Stable,
                stability_classification: PstbStabilityClassificationEvidence::Stable,
                min_stability_margin_db: Some(-20.0 * 0.5_f64.log10()),
                max_multiplier_magnitude: Some(0.5),
                num_unstable: Some(0),
                subharmonics: Vec::new(),
                converged: Some(true),
                iterations: Some(0),
            },
        )
    }

    #[test]
    fn periodic_payload_fields_are_v8_identity_while_v7_stays_legacy() {
        let pss =
            AnalysisResult::new(1, AnalysisType::Pss, "PSS")
                .with_result_payload(AnalysisResultPayload::PssFloquet {
                period_s: Some(1.0),
                fundamental_frequency_hz: Some(1.0),
                iterations: Some(2),
                residual_norm: Some(1.0e-12),
                multipliers: vec![PssFloquetMultiplierEvidence {
                    multiplier: ComplexResultValue {
                        real: 0.5,
                        imaginary: 0.0,
                    },
                }],
                floquet_evidence: FloquetSpectrumEvidence::Qualified {
                    certificate: FloquetSpectrumCertificateEvidence {
                        problem_order: 1,
                        max_backward_error: 0.0,
                        qualification_tolerance:
                            FloquetSpectrumCertificateEvidence::canonical_qualification_tolerance(1)
                                .unwrap(),
                    },
                },
                orbit_kind: FloquetOrbitKindEvidence::Driven,
                trivial_multiplier_index: None,
                stability_verdict: FloquetStabilityVerdictEvidence::Stable,
            });
        let mut changed_pss = pss.clone();
        let Some(AnalysisResultPayload::PssFloquet { multipliers, .. }) =
            changed_pss.result_payload.as_mut()
        else {
            unreachable!()
        };
        multipliers[0].multiplier.real = 0.25;
        assert_ne!(pss.result_data_digest(), changed_pss.result_data_digest());
        assert_eq!(
            pss.legacy_v7_result_data_digest(),
            changed_pss.legacy_v7_result_data_digest()
        );

        let source = stable_pstb_result();
        assert!(source.validate_retained_evidence().is_ok());
        let baseline = source.result_data_digest();
        let legacy = source.legacy_v7_result_data_digest();

        macro_rules! assert_mutation_is_identity {
            ($pattern:pat, $mutation:expr) => {{
                let mut changed = source.clone();
                let Some($pattern) = changed.result_payload.as_mut() else {
                    unreachable!()
                };
                $mutation;
                assert_ne!(baseline, changed.result_data_digest());
                assert_eq!(
                    legacy,
                    changed.legacy_v7_result_data_digest(),
                    "schema-v16 encoded no periodic payload semantics"
                );
            }};
        }

        assert_mutation_is_identity!(
            AnalysisResultPayload::Pstb { period_s, .. },
            *period_s = Some(2.0)
        );
        assert_mutation_is_identity!(
            AnalysisResultPayload::Pstb {
                stability_threshold,
                ..
            },
            *stability_threshold = Some(1.1)
        );
        assert_mutation_is_identity!(
            AnalysisResultPayload::Pstb { probe_instance, .. },
            *probe_instance = Some("LALT".to_owned())
        );
        assert_mutation_is_identity!(
            AnalysisResultPayload::Pstb { modes, .. },
            modes[0].probe_participation = 0.5
        );
        assert_mutation_is_identity!(
            AnalysisResultPayload::Pstb {
                floquet_evidence,
                ..
            },
            *floquet_evidence = FloquetSpectrumEvidence::NotComputed
        );
        assert_mutation_is_identity!(
            AnalysisResultPayload::Pstb {
                stability_classification,
                ..
            },
            *stability_classification = PstbStabilityClassificationEvidence::Marginal
        );
        assert_mutation_is_identity!(
            AnalysisResultPayload::Pstb { iterations, .. },
            *iterations = Some(1)
        );

        let mut first_run = SimulationRun::new(1);
        first_run.analyses = vec![source.clone()];
        let mut second_run = first_run.clone();
        let Some(AnalysisResultPayload::Pstb { modes, .. }) =
            second_run.analyses[0].result_payload.as_mut()
        else {
            unreachable!()
        };
        modes[0].probe_participation = 0.75;
        assert_ne!(
            first_run.dataset_content_digest(),
            second_run.dataset_content_digest(),
            "a full mode hidden from presentation remains dataset identity"
        );
        assert_eq!(
            first_run.legacy_v7_dataset_content_digest(),
            second_run.legacy_v7_dataset_content_digest()
        );
    }

    #[test]
    fn measurement_verification_fields_are_v9_identity_while_v8_stays_legacy() {
        let mut measurement = rspice_core::MeasureResult::success("peak", 12.0);
        measurement.failure_limit = Some(10.0);
        measurement.failure_limit_exceeded = true;
        measurement.passed = false;
        measurement.error = Some("FAILVALUE limit exceeded".to_owned());
        let source = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_measurements(vec![measurement]);
        let current = source.result_data_digest();
        let legacy = source.legacy_v8_result_data_digest();

        let mut raw_changed = source.clone();
        raw_changed.measurements[0].raw_value = Some(13.0);
        assert_ne!(current, raw_changed.result_data_digest());
        assert_eq!(legacy, raw_changed.legacy_v8_result_data_digest());

        let mut limit_changed = source.clone();
        limit_changed.measurements[0].failure_limit = Some(11.0);
        assert_ne!(current, limit_changed.result_data_digest());
        assert_eq!(legacy, limit_changed.legacy_v8_result_data_digest());

        let mut verdict_changed = source.clone();
        verdict_changed.measurements[0].failure_limit_exceeded = false;
        assert_ne!(current, verdict_changed.result_data_digest());
        assert_eq!(legacy, verdict_changed.legacy_v8_result_data_digest());

        let mut first_run = SimulationRun::new(1);
        first_run.analyses = vec![source];
        let mut second_run = first_run.clone();
        second_run.analyses[0].measurements[0].raw_value = Some(13.0);
        assert_ne!(
            first_run.dataset_content_digest(),
            second_run.dataset_content_digest()
        );
        assert_eq!(
            first_run.legacy_v8_dataset_content_digest(),
            second_run.legacy_v8_dataset_content_digest()
        );
    }

    fn operating_point_result() -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_result_payload(
            AnalysisResultPayload::OperatingPoint {
                temperature_mode: OperatingPointTemperatureEvidence::PvtRunSet,
                temperature_celsius: 27.0,
                initial_guess: OperatingPointInitialGuessEvidence::Automatic,
                node_initialization: OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
                homotopy: OperatingPointHomotopyEvidence::Adaptive,
                annotation: OperatingPointAnnotationEvidence::VoltagesAndCurrents,
                device_detail: OperatingPointDeviceDetailEvidence::SelectedAndViolations,
                save_device_op: OperatingPointSaveDeviceEvidence::Enabled,
                accuracy: OperatingPointAccuracyEvidence::Balanced,
                selected_devices: vec!["M1".to_owned()],
                violation_devices: vec!["M2".to_owned()],
                violation_source_content_digest: Some(ContentDigest::from_bytes([7; 32])),
                validated_startup_directives: 2,
                mna_node_names: vec!["in".to_owned(), "out".to_owned()],
                mna_branch_names: vec!["V1".to_owned()],
                mna_solution: vec![1.0, 0.5, -0.5e-3],
                effective_source_content_digest: Some(ContentDigest::from_bytes([8; 32])),
                run_point_index: 1,
                run_point_count: 2,
                run_point_process: OperatingPointProcessEvidence::TT,
                run_point_supply_voltage: None,
                run_point_nominal_supply_voltage: None,
            },
        )
    }

    #[test]
    fn operating_point_mna_and_context_are_field_sensitive_content_identity() {
        let source = operating_point_result();
        let baseline = source.result_data_digest();
        let mut changed = source.clone();
        let Some(AnalysisResultPayload::OperatingPoint { mna_solution, .. }) =
            changed.result_payload.as_mut()
        else {
            panic!("OP payload")
        };
        mna_solution[1] = 0.500_000_000_000_000_1;
        assert_ne!(baseline, changed.result_data_digest());

        let mut changed = source.clone();
        let Some(AnalysisResultPayload::OperatingPoint {
            violation_devices, ..
        }) = changed.result_payload.as_mut()
        else {
            panic!("OP payload")
        };
        violation_devices.push("M3".to_owned());
        assert_ne!(baseline, changed.result_data_digest());

        let mut changed = source.clone();
        let Some(AnalysisResultPayload::OperatingPoint {
            run_point_index, ..
        }) = changed.result_payload.as_mut()
        else {
            panic!("OP payload")
        };
        *run_point_index = 0;
        assert_ne!(baseline, changed.result_data_digest());

        let mut changed = source.clone();
        let Some(AnalysisResultPayload::OperatingPoint {
            run_point_process, ..
        }) = changed.result_payload.as_mut()
        else {
            panic!("OP payload")
        };
        *run_point_process = OperatingPointProcessEvidence::SS;
        assert_ne!(baseline, changed.result_data_digest());

        let mut changed = source.clone();
        let Some(AnalysisResultPayload::OperatingPoint {
            run_point_supply_voltage,
            run_point_nominal_supply_voltage,
            ..
        }) = changed.result_payload.as_mut()
        else {
            panic!("OP payload")
        };
        *run_point_supply_voltage = Some(0.9);
        *run_point_nominal_supply_voltage = Some(1.0);
        assert_ne!(baseline, changed.result_data_digest());
    }
    use std::collections::BTreeMap;
    use std::sync::Arc;

    fn analysis(kind: AnalysisType) -> AnalysisResult {
        AnalysisResult::new(1, kind, "presentation label").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.25, 0.5, 0.75],
                "#00aaff",
            )
            .with_complex_components(
                "V(out)",
                vec![0.25, 0.5, 0.75],
                vec![-0.5, 0.0, 0.5],
            ),
        ])
    }

    #[test]
    fn analysis_digest_is_deterministic_and_ignores_presentation_cache() {
        let source = analysis(AnalysisType::Ac);
        let mut presentation_changed = source.clone();
        presentation_changed.label = "renamed".to_owned();
        presentation_changed.timestamp += 100.0;
        presentation_changed.waveforms[0].color = "#ff0000".to_owned();
        presentation_changed.waveforms[0].visible = false;
        presentation_changed.waveforms[0].rebuild_display_cache(2);

        assert_eq!(
            source.result_data_digest(),
            source.clone().result_data_digest()
        );
        assert_eq!(
            source.result_data_digest(),
            presentation_changed.result_data_digest()
        );
    }

    #[test]
    fn analysis_digest_detects_real_imaginary_and_kind_changes() {
        let source = analysis(AnalysisType::Ac);
        let mut real_changed = source.clone();
        Arc::make_mut(&mut real_changed.waveforms[0].y)[1] = 0.500_000_000_000_000_1;
        let mut imaginary_changed = source.clone();
        Arc::make_mut(
            &mut imaginary_changed.waveforms[0]
                .complex
                .as_mut()
                .expect("complex evidence")
                .imag,
        )[1] = 1.0e-30;
        let kind_changed = analysis(AnalysisType::Transient);

        assert_ne!(
            source.result_data_digest(),
            real_changed.result_data_digest()
        );
        assert_ne!(
            source.result_data_digest(),
            imaginary_changed.result_data_digest()
        );
        assert_ne!(
            source.result_data_digest(),
            kind_changed.result_data_digest()
        );
    }

    #[test]
    fn retained_waveform_unit_is_content_identity_without_rewriting_v12_history() {
        let unstated = analysis(AnalysisType::Transient);
        let volts = {
            let mut result = unstated.clone();
            result.waveforms[0].unit = Some("V".to_owned());
            result
        };
        let amps = {
            let mut result = unstated.clone();
            result.waveforms[0].unit = Some("A".to_owned());
            result
        };

        assert_ne!(unstated.result_data_digest(), volts.result_data_digest());
        assert_ne!(volts.result_data_digest(), amps.result_data_digest());
        assert_eq!(
            volts.legacy_v5_result_data_digest(),
            amps.legacy_v5_result_data_digest(),
            "schema-v12 never contained waveform unit bytes"
        );

        let mut stated_run = SimulationRun::new(1);
        stated_run.analyses = vec![volts];
        let mut restated_run = stated_run.clone();
        restated_run.analyses = vec![amps];
        assert_ne!(
            stated_run.dataset_content_digest(),
            restated_run.dataset_content_digest()
        );
        assert_eq!(
            stated_run.legacy_v5_dataset_content_digest(),
            restated_run.legacy_v5_dataset_content_digest()
        );
    }

    #[test]
    fn canonical_float_encoding_normalizes_signed_zero_and_nan_payloads() {
        let mut positive_zero = analysis(AnalysisType::Ac);
        let mut negative_zero = positive_zero.clone();
        Arc::make_mut(&mut positive_zero.waveforms[0].y)[0] = 0.0;
        Arc::make_mut(&mut negative_zero.waveforms[0].y)[0] = -0.0;
        assert_eq!(
            positive_zero.result_data_digest(),
            negative_zero.result_data_digest()
        );

        let mut first_nan = positive_zero.clone();
        let mut second_nan = positive_zero;
        Arc::make_mut(&mut first_nan.waveforms[0].y)[0] = f64::from_bits(0x7ff8_0000_0000_0001);
        Arc::make_mut(&mut second_nan.waveforms[0].y)[0] = f64::from_bits(0x7fff_ffff_ffff_ffff);
        assert_eq!(
            first_nan.result_data_digest(),
            second_nan.result_data_digest()
        );
    }

    #[test]
    fn retained_storage_count_covers_authoritative_samples_and_display_cache() {
        let without_cache = analysis(AnalysisType::Transient);
        let mut with_cache = without_cache.clone();
        with_cache.waveforms[0].rebuild_display_cache(2);

        let base = without_cache.retained_storage_bytes();
        let cached = with_cache.retained_storage_bytes();

        assert!(base >= 4 * std::mem::size_of::<f64>() as u64);
        assert_eq!(
            cached - base,
            4 * std::mem::size_of::<f32>() as u64 + std::mem::size_of::<usize>() as u64
        );
        assert_eq!(
            without_cache.result_data_digest(),
            with_cache.result_data_digest(),
            "presentation caches remain outside immutable content identity"
        );
    }

    #[test]
    fn family_samples_and_dataset_order_are_content_identity() {
        let mut first = analysis(AnalysisType::MonteCarlo).with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                member_measurements: Vec::new(),
                seed: 9,
                runs_requested: 2,
                runs_completed: 2,
                failures: 0,
                all_converged: true,
                variables: vec![MonteCarloVariableMetadata {
                    name: "V(out)".to_owned(),
                    samples: vec![0.99, 1.01],
                    mean: 1.0,
                    std_dev: 0.01,
                    min: 0.99,
                    max: 1.01,
                }],
            },
        );
        first.id = 1;
        let mut changed_family = first.clone();
        let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) =
            changed_family.family_metadata.as_mut()
        else {
            panic!("Monte Carlo family metadata")
        };
        variables[0].samples[1] = 1.02;
        assert_ne!(
            first.result_data_digest(),
            changed_family.result_data_digest()
        );

        let mut second = analysis(AnalysisType::Ac);
        second.id = 2;
        let mut run = SimulationRun::new(1);
        run.analyses = vec![first.clone(), second.clone()];
        let digest = run.dataset_content_digest();
        run.analyses.swap(0, 1);
        assert_ne!(digest, run.dataset_content_digest());
        run.analyses = vec![first, changed_family];
        assert_ne!(digest, run.dataset_content_digest());
    }

    #[test]
    fn periodic_noise_quantity_and_carrier_are_content_identity() {
        let output_psd = analysis(AnalysisType::Pnoise).with_family_metadata(
            AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
                carrier_frequency_hz: Some(2.4e9),
            },
        );
        let phase_noise = analysis(AnalysisType::Pnoise).with_family_metadata(
            AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(2.4e9),
            },
        );
        let other_carrier = analysis(AnalysisType::Pnoise).with_family_metadata(
            AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(5.0e9),
            },
        );

        assert_ne!(
            output_psd.result_data_digest(),
            phase_noise.result_data_digest()
        );
        assert_ne!(
            phase_noise.result_data_digest(),
            other_carrier.result_data_digest()
        );
    }

    #[test]
    fn typed_payload_is_current_content_identity_without_rewriting_v1_history() {
        let pole_zero = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
            AnalysisResultPayload::PoleZero {
                poles: vec![ComplexResultValue {
                    real: -1.0,
                    imaginary: 2.0,
                }],
                zeros: vec![ComplexResultValue {
                    real: -3.0,
                    imaginary: 0.0,
                }],
                pole_evidence: PoleZeroRootSetEvidence::Qualified {
                    certificate: PoleZeroSpectrumCertificate {
                        problem_order: 1,
                        infinite_count: 0,
                        max_backward_error: 1.0e-14,
                        qualification_tolerance:
                            PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                                .unwrap(),
                    },
                },
                zero_evidence: PoleZeroRootSetEvidence::Qualified {
                    certificate: PoleZeroSpectrumCertificate {
                        problem_order: 1,
                        infinite_count: 0,
                        max_backward_error: 2.0e-14,
                        qualification_tolerance:
                            PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                                .unwrap(),
                    },
                },
                gain: Some(4.0),
            },
        );
        let mut changed_root = pole_zero.clone();
        let Some(AnalysisResultPayload::PoleZero { poles, .. }) =
            changed_root.result_payload.as_mut()
        else {
            panic!("pole-zero payload")
        };
        poles[0].imaginary = 2.5;

        let mut unavailable_gain = pole_zero.clone();
        let Some(AnalysisResultPayload::PoleZero { gain, .. }) =
            unavailable_gain.result_payload.as_mut()
        else {
            panic!("pole-zero payload")
        };
        *gain = None;

        let mut approximate_evidence = pole_zero.clone();
        let Some(AnalysisResultPayload::PoleZero { pole_evidence, .. }) =
            approximate_evidence.result_payload.as_mut()
        else {
            panic!("pole-zero payload")
        };
        *pole_evidence = PoleZeroRootSetEvidence::Approximate {
            certificate: PoleZeroSpectrumCertificate {
                problem_order: 1,
                infinite_count: 0,
                max_backward_error: 1.0e-9,
                qualification_tolerance:
                    PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1).unwrap(),
            },
        };

        assert_ne!(
            pole_zero.result_data_digest(),
            changed_root.result_data_digest()
        );
        assert_ne!(
            pole_zero.result_data_digest(),
            unavailable_gain.result_data_digest(),
            "gain availability is authenticated result evidence"
        );
        assert_ne!(
            pole_zero.result_data_digest(),
            approximate_evidence.result_data_digest(),
            "root-set qualification is authenticated result evidence"
        );
        assert_eq!(
            pole_zero.legacy_v1_result_data_digest(),
            changed_root.legacy_v1_result_data_digest(),
            "schema-v8 never contained typed payload bytes"
        );

        let mut first_run = SimulationRun::new(1);
        first_run.analyses = vec![pole_zero];
        let mut second_run = first_run.clone();
        second_run.analyses = vec![changed_root];
        assert_ne!(
            first_run.dataset_content_digest(),
            second_run.dataset_content_digest()
        );
        assert_eq!(
            first_run.legacy_v1_dataset_content_digest(),
            second_run.legacy_v1_dataset_content_digest()
        );
    }

    #[test]
    fn sensitivity_and_scalar_payload_digests_are_deterministic_and_field_sensitive() {
        let sensitivity = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Ac {
                    frequency_hz: 1_000.0,
                },
                rows: vec![SensitivityResultRow {
                    parameter: "gain".to_owned(),
                    raw: 2.0,
                    normalized: 0.5,
                }],
            });
        let mut changed = sensitivity.clone();
        let Some(AnalysisResultPayload::Sensitivity { rows, .. }) = changed.result_payload.as_mut()
        else {
            panic!("sensitivity payload")
        };
        rows[0].normalized = 0.500_000_000_000_000_1;
        assert_ne!(
            sensitivity.result_data_digest(),
            changed.result_data_digest()
        );

        let scalar = AnalysisResult::new(1, AnalysisType::Disto, "DISTO").with_result_payload(
            AnalysisResultPayload::ScalarMeasurements {
                values: BTreeMap::from([
                    ("gain".to_owned(), 10.0),
                    ("resistance".to_owned(), 50.0),
                ]),
            },
        );
        assert_eq!(
            scalar.result_data_digest(),
            scalar.clone().result_data_digest()
        );

        let positive_zero = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ")
            .with_result_payload(AnalysisResultPayload::PoleZero {
                poles: vec![ComplexResultValue {
                    real: 0.0,
                    imaginary: 0.0,
                }],
                zeros: Vec::new(),
                pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
                zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
                gain: Some(1.0),
            });
        let negative_zero = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ")
            .with_result_payload(AnalysisResultPayload::PoleZero {
                poles: vec![ComplexResultValue {
                    real: -0.0,
                    imaginary: -0.0,
                }],
                zeros: Vec::new(),
                pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
                zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
                gain: Some(1.0),
            });
        assert_eq!(
            positive_zero.result_data_digest(),
            negative_zero.result_data_digest()
        );
    }

    #[test]
    fn transfer_function_evidence_is_field_sensitive_v4_content_identity() {
        let source = AnalysisResult::new(1, AnalysisType::Tf, "TF").with_result_payload(
            AnalysisResultPayload::TransferFunction {
                input_source: "VIN".to_owned(),
                output_expression: "V(OUT)".to_owned(),
                input_quantity: TransferFunctionQuantityEvidence::Voltage,
                output_quantity: TransferFunctionQuantityEvidence::Voltage,
                input_unit: "V".to_owned(),
                output_unit: "V".to_owned(),
                normalization: TransferFunctionNormalizationEvidence::RelativeToNominal,
                accuracy: TransferFunctionAccuracyEvidence::Accurate,
                gain: Some(TransferFunctionScalarEvidence::Finite(0.5)),
                input_resistance: Some(TransferFunctionScalarEvidence::PositiveInfinity),
                output_resistance: Some(TransferFunctionScalarEvidence::Finite(50.0)),
                nominal_input: Some(1.0),
                nominal_output: Some(0.5),
            },
        );
        assert_eq!(
            source.result_data_digest(),
            source.clone().result_data_digest()
        );

        let mutate = |mutation: fn(&mut AnalysisResultPayload)| {
            let mut changed = source.clone();
            mutation(changed.result_payload.as_mut().expect("TF payload"));
            assert_ne!(source.result_data_digest(), changed.result_data_digest());
        };
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { input_source, .. } = payload else {
                unreachable!()
            };
            *input_source = "IIN".to_owned();
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction {
                output_expression, ..
            } = payload
            else {
                unreachable!()
            };
            *output_expression = "V(ALT)".to_owned();
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { input_quantity, .. } = payload else {
                unreachable!()
            };
            *input_quantity = TransferFunctionQuantityEvidence::Current;
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction {
                output_quantity, ..
            } = payload
            else {
                unreachable!()
            };
            *output_quantity = TransferFunctionQuantityEvidence::Current;
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { input_unit, .. } = payload else {
                unreachable!()
            };
            *input_unit = "mV".to_owned();
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { output_unit, .. } = payload else {
                unreachable!()
            };
            *output_unit = "mV".to_owned();
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { normalization, .. } = payload else {
                unreachable!()
            };
            *normalization = TransferFunctionNormalizationEvidence::None;
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { accuracy, .. } = payload else {
                unreachable!()
            };
            *accuracy = TransferFunctionAccuracyEvidence::Robust;
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { gain, .. } = payload else {
                unreachable!()
            };
            *gain = Some(TransferFunctionScalarEvidence::Finite(
                0.500_000_000_000_000_1,
            ));
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction {
                input_resistance, ..
            } = payload
            else {
                unreachable!()
            };
            *input_resistance = Some(TransferFunctionScalarEvidence::NegativeInfinity);
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction {
                output_resistance, ..
            } = payload
            else {
                unreachable!()
            };
            *output_resistance = None;
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { nominal_input, .. } = payload else {
                unreachable!()
            };
            *nominal_input = Some(2.0);
        });
        mutate(|payload| {
            let AnalysisResultPayload::TransferFunction { nominal_output, .. } = payload else {
                unreachable!()
            };
            *nominal_output = Some(0.25);
        });

        assert_ne!(
            source.result_data_digest(),
            source.legacy_v3_result_data_digest(),
            "current results must be sealed in the v6 domain"
        );
    }

    #[test]
    fn reliability_and_soa_evidence_are_field_sensitive_v4_content_identity() {
        let reliability = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![ReliabilityDeviceEvidence {
                    device_id: "M1".to_owned(),
                    stress: ReliabilityStressEvidence {
                        average_gate_stress_v: 1.2,
                        average_drain_stress_v: 1.8,
                        average_temperature_k: 358.15,
                        duration_s: 3_600.0,
                    },
                    checkpoints: vec![ReliabilityCheckpointEvidence {
                        years: 10.0,
                        shift: ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 0.03,
                            mobility_shift: -0.004,
                            drain_source_resistance_shift: 0.0015,
                        },
                    }],
                }],
            });
        let mut changed_reliability = reliability.clone();
        let Some(AnalysisResultPayload::Reliability { devices }) =
            changed_reliability.result_payload.as_mut()
        else {
            panic!("reliability payload")
        };
        devices[0].stress.duration_s = 3_601.0;
        assert_ne!(
            reliability.result_data_digest(),
            changed_reliability.result_data_digest()
        );

        let soa = AnalysisResult::new(1, AnalysisType::Soa, "SOA").with_result_payload(
            AnalysisResultPayload::Soa {
                evaluations: vec![SoaEvaluationEvidence {
                    device_id: "M1".to_owned(),
                    parameter: SoaParameterEvidence::DrainSourceVoltage,
                    limit_value: 3.3,
                    worst_actual_value: 3.2,
                    worst_time_s: 1.0e-6,
                    sample_count: 1_001,
                    unit: "V".to_owned(),
                    description: "Maximum drain-source voltage".to_owned(),
                    verdict: SoaRuleVerdictEvidence::Warning,
                }],
                violations: vec![SoaViolationEvidence {
                    device_id: "M1".to_owned(),
                    parameter: SoaParameterEvidence::DrainSourceVoltage,
                    limit_value: 3.3,
                    actual_value: 3.2,
                    time_s: 1.0e-6,
                    severity: SoaViolationSeverityEvidence::Warning,
                }],
            },
        );
        let mut changed_soa = soa.clone();
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) =
            changed_soa.result_payload.as_mut()
        else {
            panic!("SOA payload")
        };
        evaluations[0].sample_count = 1_002;
        assert_ne!(soa.result_data_digest(), changed_soa.result_data_digest());
        assert_eq!(soa.result_data_digest(), soa.clone().result_data_digest());
    }
    /// Two results whose event histories are identical, one of which says
    /// eight of the conductors are one word.
    fn events_with_and_without_a_bus() -> (AnalysisResult, AnalysisResult) {
        use crate::state::simulation::analysis_result::{
            DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
            DigitalEventTraceEvidence,
        };

        let trace = |name: &str| DigitalEventTraceEvidence {
            node_name: name.to_owned(),
            points: vec![
                DigitalEventPointEvidence {
                    time_s: 0.0,
                    value_code: 0,
                },
                DigitalEventPointEvidence {
                    time_s: 5.0e-9,
                    value_code: 1,
                },
            ],
        };
        let payload = |digital_buses| AnalysisResultPayload::TransientEvents {
            digital_traces: vec![trace("count#1"), trace("count#0")],
            real_traces: Vec::new(),
            digital_buses,
        };
        let analysis = |digital_buses| {
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_result_payload(payload(digital_buses))
        };
        (
            analysis(Vec::new()),
            analysis(vec![DigitalBusEvidence {
                name: "count".to_owned(),
                msb: 1,
                lsb: 0,
                members: vec!["count#1".to_owned(), "count#0".to_owned()],
                source: DigitalBusSourceEvidence::Engine,
            }]),
        )
    }

    #[test]
    fn declaring_a_bus_over_the_same_traces_is_a_different_result() {
        let (plain, bussed) = events_with_and_without_a_bus();
        assert_ne!(plain.result_data_digest(), bussed.result_data_digest());
        assert_eq!(bussed.result_data_digest(), bussed.result_data_digest());
    }

    /// The V9 encoding is what schema-v18 files were sealed with. It has to
    /// keep answering the same way for a table it never knew about, or every
    /// such file fails authentication the moment one is added.
    #[test]
    fn the_schema_v18_encoding_cannot_see_a_bus_table() {
        let (plain, bussed) = events_with_and_without_a_bus();
        assert_eq!(
            plain.legacy_v9_result_data_digest(),
            bussed.legacy_v9_result_data_digest()
        );
        assert_ne!(
            plain.legacy_v9_result_data_digest(),
            plain.result_data_digest(),
            "V10 states the table, even an empty one, so it is a different domain"
        );
    }
}
