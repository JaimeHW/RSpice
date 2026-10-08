//! Decode the structured FFT projection, retaining its metadata contract.
use super::*;
use crate::commands::waveform_io::{conversion_error, enforce_resource_limit};
use rspice_core::io::json::NumericJsonDocument;

#[derive(serde::Serialize, serde::Deserialize)]
struct Document {
    schema_version: u32,
    analysis: String,
    parent_analysis_id: String,
    coordinate: Option<FftRawCoordinate>,
    result_count: usize,
    results: Vec<ResultRecord>,
    format_policy: Policy,
}

impl NumericJsonDocument for Document {}

#[derive(serde::Serialize, serde::Deserialize)]
struct Policy {
    selected: String,
    representation: String,
    supported: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ResultRecord {
    #[serde(flatten)]
    metadata: FftRawMetadataResult,
    spectrum: Spectrum,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Spectrum {
    frequency_unit: String,
    value_unit: Option<String>,
    phase_unit: String,
    complex_representation: String,
    bins: Vec<Bin>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Bin {
    index: usize,
    frequency_hz: f64,
    value: Complex,
    magnitude: f64,
    phase_degrees: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Complex {
    real: f64,
    imaginary: f64,
}

// The JSON reader admits numbers during decoding. Keep this independent check
// at the typed projection boundary too, before allocating spectra and metadata.
fn numeric_count(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Number(_) => 1,
        serde_json::Value::Array(values) => values.iter().fold(0usize, |count, value| {
            count.saturating_add(numeric_count(value))
        }),
        serde_json::Value::Object(values) => values.values().fold(0usize, |count, value| {
            count.saturating_add(numeric_count(value))
        }),
        _ => 0,
    }
}

impl FftBundle {
    pub(crate) fn from_json(
        path: &Path,
        content: &str,
        value: serde_json::Value,
        limits: rspice_core::ResourceLimits,
    ) -> Result<Self, CliError> {
        let numeric_values = numeric_count(&value);
        enforce_resource_limit(
            path,
            rspice_core::ResourceKind::ExternalDataValues,
            numeric_values,
            limits.max_external_data_values,
        )?;
        enforce_resource_limit(
            path,
            rspice_core::ResourceKind::ResultValues,
            numeric_values,
            limits.max_result_values,
        )?;
        // Release the admitted untyped tree before allocating the typed one.
        // Decode from source so integers beyond u64 still have their authored
        // digits available to the shared precision check.
        drop(value);
        let document = Document::decode_numeric_json(content, &crate::abort::ProcessAbort)
            .map_err(|error| match error {
                rspice_core::io::json::JsonDecodeError::Aborted => CliError::Interrupted,
                error => conversion_error(path, error),
            })?;
        if document.schema_version != FFT_ARTIFACT_SCHEMA_VERSION
            || document.analysis != "fft"
            || document.result_count != document.results.len()
            || document.format_policy.selected != "json"
            || document.format_policy.representation != "structured"
            || document
                .format_policy
                .supported
                .iter()
                .map(String::as_str)
                .ne(["json", "csv", "tsv", "raw", "ascii", "hdf5"])
        {
            return Err(conversion_error(path, "invalid FFT JSON document envelope"));
        }
        let mut results = Vec::with_capacity(document.results.len());
        let mut bins = Vec::new();
        for result in document.results {
            if result.spectrum.frequency_unit != "Hz"
                || result.spectrum.phase_unit != "degree"
                || result.spectrum.complex_representation != "cartesian"
                || result.spectrum.value_unit != result.metadata.signal.unit
            {
                return Err(conversion_error(
                    path,
                    "invalid FFT spectrum units or representation",
                ));
            }
            bins.extend(
                result
                    .spectrum
                    .bins
                    .into_iter()
                    .map(|bin| DecodedFftRawBin {
                        analysis_id: result.metadata.analysis_id.clone(),
                        index: bin.index,
                        frequency_hz: bin.frequency_hz,
                        real: bin.value.real,
                        imaginary: bin.value.imaginary,
                        magnitude: bin.magnitude,
                        phase_degrees: bin.phase_degrees,
                    }),
            );
            results.push(result.metadata);
        }
        Self::from_metadata(
            metadata_envelope(document.parent_analysis_id, document.coordinate, results),
            bins,
        )
        .map_err(|error| conversion_error(path, error))
    }
}

/// The common provenance validator predates the other decoders; supply its
/// fixed RAW transport fields only after validating each source's own envelope.
pub(super) fn metadata_envelope(
    parent_analysis_id: String,
    coordinate: Option<FftRawCoordinate>,
    results: Vec<FftRawMetadataResult>,
) -> FftRawMetadata {
    FftRawMetadata {
        schema_version: FFT_ARTIFACT_SCHEMA_VERSION,
        analysis: "fft".into(),
        parent_analysis_id,
        coordinate,
        result_count: results.len(),
        results,
        data_columns: [
            "frequency_hz",
            "real",
            "imaginary",
            "magnitude",
            "phase_degrees",
            "fft_ordinal",
            "bin_index",
        ]
        .map(str::to_owned),
        frequency_unit: "Hz".into(),
        phase_unit: "degree".into(),
        complex_representation: "cartesian".into(),
        selected_format: "raw".into(),
    }
}
