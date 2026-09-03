//! The compression error certificate.
//!
//! Every compressed result carries the algorithm, the sample domain, the
//! applied tolerance policy and the worst observed reconstruction error. The
//! rebuild refuses a certificate from a schema version it does not implement
//! rather than presenting an unverified waveform as a bounded one.

use super::*;

pub(crate) type CompressionErrorPersistenceState =
    (String, String, usize, f64, f64, f64, Option<f64>, f64, f64);
pub(crate) type CompressionReportPersistenceState = (
    u32,
    String,
    String,
    bool,
    f64,
    f64,
    f64,
    usize,
    usize,
    Option<CompressionErrorPersistenceState>,
);

pub(crate) fn compression_report_persistence_state(
    report: &rspice_core::engine::TransientCompressionReport,
) -> CompressionReportPersistenceState {
    let worst = report.worst_observed.as_ref().map(|observation| {
        (
            observation.signal.kind.as_str().to_string(),
            observation.signal.canonical_name.clone(),
            observation.input_sample_index,
            observation.time,
            observation.actual_value,
            observation.absolute_error,
            observation.relative_error,
            observation.allowed_tolerance,
            observation.tolerance_utilization,
        )
    });
    (
        report.schema_version,
        report.algorithm.as_str().to_string(),
        report.sample_domain.as_str().to_string(),
        report.applied_policy.enabled,
        report.applied_policy.absolute_tolerance,
        report.applied_policy.relative_tolerance,
        report.applied_policy.maximum_retained_interval,
        report.input_points,
        report.retained_points,
        worst,
    )
}

pub(crate) fn rebuild_compression_report(
    state: CompressionReportPersistenceState,
) -> PyResult<rspice_core::engine::TransientCompressionReport> {
    let (
        schema_version,
        algorithm,
        sample_domain,
        enabled,
        absolute_tolerance,
        relative_tolerance,
        maximum_retained_interval,
        input_points,
        retained_points,
        worst,
    ) = state;
    if schema_version != rspice_core::engine::TRANSIENT_COMPRESSION_REPORT_VERSION {
        return Err(crate::errors::value_error(format!(
            "unsupported compressed-transient compression-report version {schema_version}"
        )));
    }
    let algorithm = match algorithm.as_str() {
        "multi-channel-rdp-linear-v1" => {
            rspice_core::engine::TransientCompressionAlgorithm::MultiChannelRdpLinearV1
        }
        _ => {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient compression algorithm '{algorithm}'"
            )));
        }
    };
    let sample_domain = match sample_domain.as_str() {
        "accepted-input-samples" => {
            rspice_core::engine::TransientCompressionSampleDomain::AcceptedInputSamples
        }
        _ => {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient compression sample domain '{sample_domain}'"
            )));
        }
    };
    let worst_observed = worst
        .map(
            |(
                signal_kind,
                canonical_name,
                input_sample_index,
                time,
                actual_value,
                absolute_error,
                relative_error,
                allowed_tolerance,
                tolerance_utilization,
            )| {
                let Some(kind) =
                    rspice_core::engine::TransientCompressionSignalKind::from_tag(&signal_kind)
                else {
                    return Err(crate::errors::value_error(format!(
                        "unsupported compressed-transient compression signal kind '{signal_kind}'"
                    )));
                };
                Ok(rspice_core::engine::TransientCompressionErrorObservation {
                    signal: rspice_core::engine::TransientCompressionSignal::new(
                        kind,
                        canonical_name,
                    )
                    .map_err(crate::errors::value_error)?,
                    input_sample_index,
                    time,
                    actual_value,
                    absolute_error,
                    relative_error,
                    allowed_tolerance,
                    tolerance_utilization,
                })
            },
        )
        .transpose()?;
    Ok(rspice_core::engine::TransientCompressionReport {
        schema_version,
        algorithm,
        sample_domain,
        applied_policy: rspice_core::engine::TransientCompressionPolicy {
            enabled,
            absolute_tolerance,
            relative_tolerance,
            maximum_retained_interval,
        },
        input_points,
        retained_points,
        worst_observed,
    })
}
