//! Publication of the shared core result document as one engine artifact.
//!
//! Every analysis family this executor runs publishes exactly one
//! [`AnalysisResultDocument`]. The adapter does not define a result schema of
//! its own: the content type, the schema identifier, and the schema version
//! are all read off the core document, so a core schema revision changes the
//! wire declaration without an adapter edit.
//!
//! Two things are derived here rather than decided per family: the manifest
//! measurement set, which is projected generically from the document's axes,
//! signals, and scalars, and the result schema signature, which is what the
//! coordinate loop compares when a run axis could conditionally change a
//! result's shape.

use std::sync::LazyLock;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::execution::result_document::{
    AxisValues, ScalarValue, SeriesQualifier, SeriesValues,
};
use rspice_core::execution::{
    ANALYSIS_RESULT_DOCUMENT_SCHEMA, ANALYSIS_RESULT_DOCUMENT_VERSION, AnalysisResultDocument,
    SignalUnit,
};

use crate::failure::{DirectiveFailure, check_abort, map_measurement_error};
use crate::measure::{Measurement, measurement_name};
use crate::wire::{MAX_ENGINE_ARTIFACT_BYTES, MAX_ENGINE_RETAINED_RESULT_BYTES};

/// Smallest number of encoded JSON bytes one retained numerical value can
/// occupy, including its separator.
///
/// Nothing in the document encodes a value in less than `0` plus one
/// delimiter. Multiplying the document's own `total_value_count` by this is
/// therefore a sound lower bound on the encoded size, which is exactly what a
/// preflight needs: a family whose value count alone cannot fit is refused
/// with a typed resource outcome before a single byte is serialized.
const MINIMUM_ENCODED_BYTES_PER_VALUE: u64 = 2;

/// MIME type declared for every typed result artifact, derived from the core
/// document's own schema identity so the two cannot drift.
static RESULT_DOCUMENT_CONTENT_TYPE: LazyLock<String> = LazyLock::new(|| {
    format!(
        "application/vnd.{ANALYSIS_RESULT_DOCUMENT_SCHEMA}+json;version={ANALYSIS_RESULT_DOCUMENT_VERSION}"
    )
});

/// The declared content type of a typed result artifact.
pub fn result_document_content_type() -> &'static str {
    &RESULT_DOCUMENT_CONTENT_TYPE
}

/// A results file staged in memory. Files are only written after every
/// directive has succeeded, so a failed run leaves `results/` empty and the
/// response is the single source of truth about declared outputs.
pub struct PendingArtifact {
    pub file_name: String,
    pub content_type: String,
    /// Result family tag the document inside declares.
    pub result_kind: String,
    pub content: String,
}

/// Structural identity of one result document, without its values.
///
/// Two coordinates of the same run axis must publish the same signal schema.
/// Comparing this is how a conditional `.STEP` topology that silently drops a
/// node is caught before the result set is published.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct ResultSchemaSignature(Vec<String>);

impl ResultSchemaSignature {
    pub(crate) fn from_document(document: &AnalysisResultDocument) -> Self {
        let mut entries = vec![format!("kind={}", document.result_kind().tag())];
        for axis in document.axes() {
            entries.push(format!(
                "axis={};kind={:?};unit={}",
                axis.name(),
                axis.kind(),
                axis.unit().symbol()
            ));
        }
        for signal in document.signals() {
            let descriptor = signal.descriptor();
            entries.push(format!(
                "signal={};qualifier={};kind={:?};owner={:?};unit={};value={:?};shape={:?}",
                descriptor.canonical_name(),
                signal.qualifier().map_or_else(String::new, qualifier_tag),
                descriptor.kind(),
                descriptor.owner(),
                descriptor.unit().symbol(),
                descriptor.value_type(),
                descriptor.shape(),
            ));
        }
        for scalar in document.scalars() {
            entries.push(format!(
                "scalar={};unit={}",
                scalar.name(),
                scalar.unit().map_or_else(String::new, SignalUnit::symbol),
            ));
        }
        for state in document.device_states() {
            entries.push(format!(
                "device={};kind={}",
                state.device_name(),
                state.device_kind().unwrap_or_default()
            ));
            for parameter in state.parameters() {
                entries.push(format!(
                    "device_parameter={}.{};unit={}",
                    state.device_name(),
                    parameter.name,
                    parameter
                        .unit
                        .as_ref()
                        .map_or_else(String::new, SignalUnit::symbol),
                ));
            }
        }
        Self(entries)
    }
}

/// Validate, budget, and encode one document as a staged result artifact.
///
/// The value-count preflight runs first so a Monte Carlo trial table, an HB
/// spectrum set, or a PNoise sideband set that cannot possibly fit is refused
/// as `resource.result_set_bytes` instead of being allocated and then found
/// to be too large.
pub(crate) fn encode_result_artifact(
    file_name: String,
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
    byte_limit: u64,
) -> Result<PendingArtifact, DirectiveFailure> {
    let limit = byte_limit
        .min(MAX_ENGINE_ARTIFACT_BYTES)
        .min(MAX_ENGINE_RETAINED_RESULT_BYTES);
    preflight_document_values(document, limit)?;
    let content = document
        .to_json_with_abort(abort, limit)
        .map_err(crate::failure::map_result_document_error)?;
    Ok(PendingArtifact {
        file_name,
        content_type: result_document_content_type().to_owned(),
        result_kind: document.result_kind().tag().to_owned(),
        content,
    })
}

/// Refuse a document whose retained value count alone cannot fit the budget.
pub(crate) fn preflight_document_values(
    document: &AnalysisResultDocument,
    byte_limit: u64,
) -> Result<(), DirectiveFailure> {
    let values = u64::try_from(document.total_value_count())
        .map_err(|_| DirectiveFailure::ResultSetBytes)?;
    let minimum_bytes = values
        .checked_mul(MINIMUM_ENCODED_BYTES_PER_VALUE)
        .ok_or(DirectiveFailure::ResultSetBytes)?;
    if minimum_bytes > byte_limit {
        return Err(DirectiveFailure::ResultSetBytes);
    }
    Ok(())
}

/// Project one document's series and scalars into manifest measurements.
///
/// This is deliberately one generic projection rather than a per-family
/// choice: the typed artifact is the evidence, and the manifest is a
/// name-keyed digest of it. A series whose samples are all absent contributes
/// no measurement, because its absence is already recorded in the document
/// with its descriptor, unit, and availability; inventing a placeholder scalar
/// for it is exactly the missingness lie the document exists to prevent.
pub(crate) fn measurements_from_document(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<Vec<Measurement>, DirectiveFailure> {
    let mut measurements = Vec::new();
    for axis in document.axes() {
        check_abort(abort)?;
        let samples: Vec<Option<f64>> = match axis.values() {
            AxisValues::Real { values } => values.iter().copied().map(Some).collect(),
            // An integer axis (trial index, harmonic index, sideband) is an
            // exact coordinate; widening it to f64 is lossless for every
            // magnitude an axis of a retained result can reach, and the exact
            // values stay in the typed artifact regardless.
            AxisValues::Integer { values } => {
                values.iter().map(|value| Some(*value as f64)).collect()
            }
        };
        push_series(
            &mut measurements,
            measurement_name("axis", axis.name()),
            axis.unit().symbol(),
            &samples,
            abort,
        )?;
    }

    for signal in document.signals() {
        check_abort(abort)?;
        let descriptor = signal.descriptor();
        let label = match signal.qualifier() {
            Some(qualifier) => format!(
                "{}#{}",
                descriptor.canonical_name(),
                qualifier_tag(qualifier)
            ),
            None => descriptor.canonical_name().to_owned(),
        };
        let unit = descriptor.unit().symbol();
        match signal.values() {
            SeriesValues::Real { samples } => push_series(
                &mut measurements,
                measurement_name("signal", &label),
                unit,
                samples,
                abort,
            )?,
            SeriesValues::Complex { samples } => {
                let real: Vec<Option<f64>> = samples.iter().map(|s| s.map(|s| s.real)).collect();
                let imaginary: Vec<Option<f64>> =
                    samples.iter().map(|s| s.map(|s| s.imaginary)).collect();
                push_series(
                    &mut measurements,
                    measurement_name("signal", &format!("{label}.re")),
                    unit.clone(),
                    &real,
                    abort,
                )?;
                push_series(
                    &mut measurements,
                    measurement_name("signal", &format!("{label}.im")),
                    unit,
                    &imaginary,
                    abort,
                )?;
            }
            // Logic samples are a state/strength pair, not a number. They are
            // retained verbatim in the typed artifact; folding them onto a
            // numeric axis here would invent an encoding the document
            // deliberately does not define.
            SeriesValues::Logic { .. } => {}
        }
    }

    for scalar in document.scalars() {
        check_abort(abort)?;
        let unit = scalar
            .unit()
            .map_or_else(|| "1".to_owned(), SignalUnit::symbol);
        let name = measurement_name("scalar", scalar.name());
        let value = match scalar.value() {
            ScalarValue::Real { value } => *value,
            ScalarValue::Integer { value } => Some(*value as f64),
            ScalarValue::Count { value } => Some(*value as f64),
            ScalarValue::Boolean { value } => Some(f64::from(u8::from(*value))),
            ScalarValue::Complex { value } => {
                if let Some(sample) = value {
                    push_scalar(
                        &mut measurements,
                        measurement_name("scalar", &format!("{}.re", scalar.name())),
                        unit.clone(),
                        sample.real,
                    )?;
                    push_scalar(
                        &mut measurements,
                        measurement_name("scalar", &format!("{}.im", scalar.name())),
                        unit,
                        sample.imaginary,
                    )?;
                }
                continue;
            }
            // Text carries no numeric evidence; it stays in the document.
            ScalarValue::Text { .. } => continue,
            // A quantity the analysis proved has no finite value is not a
            // measurement. Naming it here with any number at all would report
            // a margin or an impedance the circuit does not have; the typed
            // determination stays in the document, where it can be read.
            ScalarValue::Unavailable { .. } => continue,
        };
        if let Some(value) = value {
            push_scalar(&mut measurements, name, unit, value)?;
        }
    }

    Ok(measurements)
}

fn push_scalar(
    measurements: &mut Vec<Measurement>,
    name: String,
    unit: String,
    value: f64,
) -> Result<(), DirectiveFailure> {
    measurements.push(Measurement::scalar(name, unit, value).ok_or(DirectiveFailure::NonFinite)?);
    Ok(())
}

fn push_series(
    measurements: &mut Vec<Measurement>,
    name: String,
    unit: String,
    samples: &[Option<f64>],
    abort: &dyn AbortSignal,
) -> Result<(), DirectiveFailure> {
    if let Some(measurement) =
        Measurement::series_with_abort(name, unit, samples, abort).map_err(map_measurement_error)?
    {
        measurements.push(measurement);
    }
    Ok(())
}

fn qualifier_tag(qualifier: &SeriesQualifier) -> String {
    match qualifier {
        SeriesQualifier::DistortionFundamental { tone } => format!("fundamental-{tone:?}"),
        SeriesQualifier::DistortionProduct { product } => format!("product-{}", product.label()),
        SeriesQualifier::PacSideband { sideband } => format!("sideband-{sideband}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::execution::DeckPlan;
    use rspice_core::resource::ResourceLimits;
    use rspice_core::{Engine, Netlist};

    /// One real operating-point document, projected exactly as the executor
    /// projects it, so the budget and measurement contracts are exercised
    /// against a document the engine actually produced.
    fn divider_document() -> AnalysisResultDocument {
        let netlist = Netlist::parse_validated(
            "budget fixture\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
        )
        .expect("budget fixture parses");
        let plan =
            DeckPlan::from_netlist_with_abort(&netlist, &ResourceLimits::unlimited(), &NoAbort)
                .expect("budget fixture plans");
        let analysis = plan.analyses()[0].id();
        let (result, report) = Engine::default()
            .run_dc_op_with_report_and_abort(&netlist, &NoAbort)
            .expect("budget fixture solves");
        AnalysisResultDocument::from_operating_point(analysis, &result, Some(&report))
            .expect("budget fixture projects")
            .build()
            .expect("budget fixture validates")
    }

    #[test]
    fn a_document_that_cannot_fit_is_refused_before_it_is_serialized() {
        let document = divider_document();
        let values = document.total_value_count();
        assert!(values > 0, "the fixture must retain values to budget");
        assert!(preflight_document_values(&document, u64::MAX).is_ok());

        // The lower bound is the value count times the smallest encoding one
        // value can have, so a budget one byte under that is provably
        // unreachable and is refused without allocating the document's JSON.
        let lower_bound = values as u64 * MINIMUM_ENCODED_BYTES_PER_VALUE;
        assert!(matches!(
            preflight_document_values(&document, lower_bound - 1),
            Err(DirectiveFailure::ResultSetBytes)
        ));
        assert!(matches!(
            encode_result_artifact("op-001.result.json".to_owned(), &document, &NoAbort, 1),
            Err(DirectiveFailure::ResultSetBytes)
        ));
    }

    #[test]
    fn an_encoded_artifact_declares_the_documents_own_family_and_content_type() {
        let document = divider_document();
        let artifact = encode_result_artifact(
            "op-001.result.json".to_owned(),
            &document,
            &NoAbort,
            u64::MAX,
        )
        .expect("the fixture fits an unbounded budget");
        assert_eq!(artifact.result_kind, document.result_kind().tag());
        assert_eq!(artifact.content_type, result_document_content_type());
        assert_eq!(
            AnalysisResultDocument::from_json(&artifact.content).expect("artifact round-trips"),
            document
        );
    }

    #[test]
    fn measurements_name_every_axis_signal_and_scalar_of_the_document() {
        let document = divider_document();
        let measurements =
            measurements_from_document(&document, &NoAbort).expect("projection succeeds");
        let names: Vec<&str> = measurements
            .iter()
            .map(|measurement| measurement.name.as_str())
            .collect();
        assert!(names.contains(&"signal:v(out)"), "{names:?}");
        assert!(names.contains(&"signal:i(v1)"), "{names:?}");
        assert!(
            measurements.iter().all(|measurement| {
                !measurement.unit.trim().is_empty() && measurement.sample_count > 0
            }),
            "every measurement declares a unit and a shape"
        );
    }

    #[test]
    fn the_declared_content_type_is_read_off_the_core_document() {
        let content_type = result_document_content_type();
        assert!(
            content_type.contains(ANALYSIS_RESULT_DOCUMENT_SCHEMA),
            "{content_type}"
        );
        assert!(
            content_type.ends_with(&format!("version={ANALYSIS_RESULT_DOCUMENT_VERSION}")),
            "{content_type}"
        );
    }
}
