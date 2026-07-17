//! Canonical content identity for immutable retained result data.
//!
//! This encoding is deliberately independent of serde, `Debug`, collection
//! allocation, and host word size. Presentation-only waveform attributes are
//! excluded: a color, visibility, or display-cache change does not create a
//! different source dataset. Authoritative f64/complex128 values are encoded
//! in IEEE-754 binary64 form after normalizing signed zero and NaN payloads.

use sha2::{Digest as _, Sha256};

use super::*;
use crate::product::ContentDigest;
use crate::state::{
    SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
};

const RESULT_DIGEST_MAGIC: &[u8] = b"RSPICE-RESULT-DATA";
const RESULT_DIGEST_ENCODING_VERSION: u16 = 1;
const CANONICAL_NAN_BITS: u64 = 0x7ff8_0000_0000_0000;

struct ResultDigestWriter {
    hasher: Sha256,
}

impl ResultDigestWriter {
    fn new(domain: &str) -> Self {
        let mut writer = Self {
            hasher: Sha256::new(),
        };
        writer.raw(RESULT_DIGEST_MAGIC);
        writer.raw(&RESULT_DIGEST_ENCODING_VERSION.to_be_bytes());
        writer.string(domain);
        writer
    }

    fn bool(&mut self, value: bool) {
        self.raw(&[0x01, u8::from(value)]);
    }

    fn u8(&mut self, value: u8) {
        self.raw(&[0x02, value]);
    }

    fn u64(&mut self, value: u64) {
        self.raw(&[0x03]);
        self.raw(&value.to_be_bytes());
    }

    fn usize(&mut self, value: usize) {
        self.u64(u64::try_from(value).expect("supported Rust targets use at most 64-bit usize"));
    }

    fn f64(&mut self, value: f64) {
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
        self.raw(value.as_bytes());
    }

    fn digest(&mut self, value: ContentDigest) {
        self.raw(&[0x06]);
        self.raw(value.as_bytes());
    }

    fn uuid(&mut self, value: uuid::Uuid) {
        self.raw(&[0x07]);
        self.raw(value.as_bytes());
    }

    fn sequence(&mut self, len: usize) {
        self.raw(&[0x08]);
        self.usize(len);
    }

    fn option<T: ?Sized>(&mut self, value: Option<&T>, encode: impl FnOnce(&mut Self, &T)) {
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
        let mut writer = ResultDigestWriter::new("rspice.analysis-result-data/v1");
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
        }

        writer.option(self.dc_op.as_ref(), encode_dc_op);
        writer.option(self.device_op.as_ref(), encode_device_op);
        writer.option(self.noise_summary.as_ref(), encode_noise_summary);
        writer.option(self.family_metadata.as_ref(), encode_family_metadata);

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

        writer.finish()
    }
}

impl SimulationRun {
    /// SHA-256 identity of the ordered immutable analysis dataset retained by
    /// this run. Stable run/dataset IDs and wall-clock metadata are excluded;
    /// they address the dataset but do not define its sample content.
    #[must_use]
    pub fn dataset_content_digest(&self) -> ContentDigest {
        let mut writer = ResultDigestWriter::new("rspice.simulation-dataset-data/v1");
        writer.sequence(self.analyses.len());
        for analysis in &self.analyses {
            writer.u64(analysis.id);
            writer.digest(analysis.result_data_digest());
        }
        writer.finish()
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

fn encode_noise_summary(writer: &mut ResultDigestWriter, summary: &NoiseSummary) {
    writer.f64(summary.total_rms);
    writer.f64(summary.band.0);
    writer.f64(summary.band.1);
    writer.sequence(summary.rows.len());
    for row in &summary.rows {
        writer.string(&row.device);
        writer.string(row.mechanism);
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
    fn family_samples_and_dataset_order_are_content_identity() {
        let mut first = analysis(AnalysisType::MonteCarlo).with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
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
}
