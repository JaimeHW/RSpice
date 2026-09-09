//! Field availability and explicit unknowns in historical result schemas.
//!
//! These rules reject evidence that an older format could not authenticate.
//! Migration restores only the documented legacy defaults after authentication;
//! the digest encoders remain with their schema-specific validators.

use super::*;

pub(super) fn reject_pole_zero_evidence_before_schema_v16(
    results: &ProjectSimulationResultsData,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= POLE_ZERO_EVIDENCE_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for run in &results.runs {
        for analysis in &run.analyses {
            let PersistedField::Value(AnalysisResultPayload::PoleZero {
                pole_evidence,
                zero_evidence,
                ..
            }) = &analysis.result_payload
            else {
                continue;
            };
            if !matches!(
                pole_evidence,
                crate::state::PoleZeroRootSetEvidence::LegacyUnknown
            ) || !matches!(
                zero_evidence,
                crate::state::PoleZeroRootSetEvidence::LegacyUnknown
            ) {
                return Err(format!(
                    "schema-v{source_schema} analysis {} contains pole-zero root evidence introduced by schema v16",
                    analysis.id
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn reject_periodic_stability_payload_before_schema_v17(
    results: &ProjectSimulationResultsData,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= PERIODIC_STABILITY_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for run in &results.runs {
        for analysis in &run.analyses {
            if matches!(
                analysis.result_payload.as_ref(),
                Some(AnalysisResultPayload::PssFloquet { .. })
                    | Some(AnalysisResultPayload::Pstb { .. })
            ) {
                return Err(format!(
                    "schema-v{source_schema} analysis {} contains periodic stability evidence introduced by schema v17",
                    analysis.id
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn synthesize_legacy_periodic_markers(
    run: &mut ProjectSimulationRun,
) -> Result<(), String> {
    for analysis in &mut run.analyses {
        if !analysis.success || !analysis.result_payload.is_missing() {
            continue;
        }
        let analysis_type = analysis_type_from_key(&analysis.analysis_type).ok_or_else(|| {
            format!(
                "legacy analysis {} has unknown analysis type '{}'",
                analysis.id, analysis.analysis_type
            )
        })?;
        if let Some(marker) = AnalysisResultPayload::legacy_periodic_marker(analysis_type) {
            analysis.result_payload = PersistedField::Value(marker);
        }
    }
    Ok(())
}

pub(super) fn require_legacy_result_digest_absence(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    if run.dataset_content_digest.is_present() {
        return Err(format!(
            "schema-v{source_schema} simulation run {} contains a dataset content digest introduced by schema v8",
            run.id
        ));
    }
    if let Some(analysis) = run
        .analyses
        .iter()
        .find(|analysis| analysis.result_data_digest.is_present())
    {
        return Err(format!(
            "schema-v{source_schema} analysis {} contains a result data digest introduced by schema v8",
            analysis.id
        ));
    }
    Ok(())
}

pub(in crate::io::project_io) fn validate_result_fields_for_source_schema(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, source_schema)?;
    reject_legacy_waveform_units(run, source_schema)?;
    for analysis in &run.analyses {
        if source_schema < FAMILY_METADATA_RESULTS_SCHEMA_VERSION
            && analysis.family_metadata.is_some()
        {
            return Err(format!(
                "schema-v{source_schema} analysis {} contains family metadata introduced by schema v7",
                analysis.id
            ));
        }
        if source_schema < TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION
            && analysis.result_payload.is_present()
        {
            return Err(format!(
                "schema-v{source_schema} analysis {} contains a typed result payload introduced by schema v9",
                analysis.id
            ));
        }
    }
    Ok(())
}

/// A unit on a waveform written before schema v13 is content the digest that
/// sealed the file never covered. Admitting it would let an edited unit ride
/// into a resealed document under an authentic older digest.
pub(super) fn reject_legacy_waveform_units(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    for analysis in &run.analyses {
        if analysis
            .waveforms
            .iter()
            .any(|waveform| waveform.unit.is_some())
        {
            return Err(format!(
                "schema-v{source_schema} analysis {} contains a waveform unit introduced by schema v13",
                analysis.id
            ));
        }
    }
    Ok(())
}

pub(super) fn reject_legacy_operating_point_evidence(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    if let Some(analysis) = run.analyses.iter().find(|analysis| {
        matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::OperatingPoint { .. })
        )
    }) {
        return Err(format!(
            "schema-v{source_schema} analysis {} contains operating-point evidence introduced by schema v12",
            analysis.id
        ));
    }
    Ok(())
}

pub(super) fn validate_legacy_noise_summary_shape(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    for analysis in &run.analyses {
        let Some(summary) = analysis.noise_summary.as_ref() else {
            continue;
        };
        if summary.total_rms.is_none() {
            return Err(format!(
                "schema-v{source_schema} analysis {} is missing required noise_summary.total_rms",
                analysis.id
            ));
        }
        if summary.input_rms.is_some() {
            return Err(format!(
                "schema-v{source_schema} analysis {} contains noise_summary.input_rms introduced by schema v12",
                analysis.id
            ));
        }
    }
    Ok(())
}

pub(super) fn reject_measurement_verification_before_schema_v18(
    results: &ProjectSimulationResultsData,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= MEASUREMENT_VERIFICATION_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for run in &results.runs {
        for analysis in &run.analyses {
            for measurement in &analysis.measurements {
                if measurement.raw_value.is_some()
                    || measurement.failure_limit.is_some()
                    || measurement.failure_limit_exceeded
                {
                    return Err(format!(
                        "schema-v{source_schema} analysis {} measurement '{}' contains measurement verification evidence introduced by schema v18",
                        analysis.id, measurement.name
                    ));
                }
            }
        }
    }
    Ok(())
}

/// A digital bus table is schema-v19 evidence. A file that says it was
/// written under an earlier schema cannot have had one, so one that carries a
/// bus is not an old file — it is a file whose stated version disagrees with
/// its content, and admitting it would authenticate a table under a digest
/// encoding that never covered it.
pub(super) fn reject_digital_buses_before_schema_v19(
    results: &ProjectSimulationResultsData,
    source_schema: u32,
) -> Result<(), String> {
    if source_schema >= DIGITAL_BUS_RESULTS_SCHEMA_VERSION {
        return Ok(());
    }
    for run in &results.runs {
        for analysis in &run.analyses {
            if let Some(AnalysisResultPayload::TransientEvents { digital_buses, .. }) =
                analysis.result_payload.as_ref()
                && !digital_buses.is_empty()
            {
                return Err(format!(
                    "schema-v{source_schema} analysis {} declares digital bus '{}', which was introduced by schema v19",
                    analysis.id, digital_buses[0].name
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn restore_legacy_measurement_verification(run: &mut ProjectSimulationRun) {
    for analysis in &mut run.analyses {
        for measurement in &mut analysis.measurements {
            measurement.raw_value = measurement.value;
            measurement.failure_limit = None;
            measurement.failure_limit_exceeded = false;
        }
    }
}
