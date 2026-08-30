//! Authenticating a result document against the encoding that sealed it.
//!
//! Every result schema from v8 onward sealed its runs with a digest over the
//! fields that schema had. Checking a file written under one of them against
//! today's encoding would reject a valid project; checking it against no
//! encoding would accept a tampered one. So each schema keeps its own
//! validator, and migration runs the one that matches the file it was handed
//! before it admits any later field or reseals anything.
//!
//! These are deliberately not shared. A single parameterised validator would
//! have to be edited every time a field is added, and the edit that got it
//! wrong would silently re-authenticate history under the wrong rule.

use super::*;

/// Authenticate a schema-v8 run with the exact digest encoding that wrote it.
/// This must run before any v9 fields are introduced or digests are resealed.
pub(super) fn validate_v8_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, CONTENT_DIGEST_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_waveform_units(run, CONTENT_DIGEST_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        if analysis.result_payload.is_present() {
            return Err(format!(
                "schema-v8 analysis {} contains a typed result payload introduced by schema v9",
                analysis.id
            ));
        }
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v8 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v1_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v8 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v8 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v1_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v8 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v9 run with the exact typed-payload digest encoding
/// that wrote it before schema-v10 Reliability/SOA evidence is admitted.
pub(super) fn validate_v9_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_operating_point_evidence(run, TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_waveform_units(run, TYPED_PAYLOAD_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        if matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::Reliability { .. })
                | Some(AnalysisResultPayload::Soa { .. })
        ) {
            return Err(format!(
                "schema-v9 analysis {} contains Reliability/SOA evidence introduced by schema v10",
                analysis.id
            ));
        }
        if matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransferFunction { .. })
        ) {
            return Err(format!(
                "schema-v9 analysis {} contains transfer-function evidence introduced by schema v11",
                analysis.id
            ));
        }
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v9 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v2_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v9 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v9 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v2_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v9 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v10 run with the exact Reliability/SOA-capable
/// digest encoding that wrote it. Typed TF evidence is a schema-v11 field and
/// must be rejected before the authenticated v10 document is resealed.
pub(super) fn validate_v10_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, RELIABILITY_SOA_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_operating_point_evidence(run, RELIABILITY_SOA_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_waveform_units(run, RELIABILITY_SOA_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        if matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransferFunction { .. })
        ) {
            return Err(format!(
                "schema-v10 analysis {} contains transfer-function evidence introduced by schema v11",
                analysis.id
            ));
        }
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v10 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v3_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v10 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v10 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v3_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v10 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v11 run before admitting schema-v12 operating-point
/// payloads and optional/input-referred integrated noise evidence.
pub(super) fn validate_v11_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    validate_legacy_noise_summary_shape(run, TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_operating_point_evidence(run, TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION)?;
    reject_legacy_waveform_units(run, TRANSFER_FUNCTION_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v11 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v4_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v11 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v11 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v4_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v11 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v12 run with the exact digest encoding that wrote it,
/// before per-waveform units are admitted and the run is resealed.
pub(super) fn validate_v12_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    reject_legacy_waveform_units(run, OPERATING_POINT_RESULTS_SCHEMA_VERSION)?;
    for analysis in &run.analyses {
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v12 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v5_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v12 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v12 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v5_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v12 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate schema-v13 through schema-v15 runs with the exact digest
/// encoding that required every pole-zero payload to contain a numeric gain.
/// This runs before optional gain is admitted and the document is resealed.
pub(super) fn validate_v13_to_v15_result_digests(
    run: &ProjectSimulationRun,
    source_schema: u32,
) -> Result<(), String> {
    debug_assert!(
        (WAVEFORM_UNIT_RESULTS_SCHEMA_VERSION..=EXECUTED_DECK_RESULTS_SCHEMA_VERSION)
            .contains(&source_schema)
    );
    for analysis in &run.analyses {
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v{source_schema} analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v6_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v{source_schema} analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v{source_schema} simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v6_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v{source_schema} simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v16 run with the exact digest encoding that wrote it
/// before durable Floquet payloads are admitted and the document is resealed.
pub(super) fn validate_v16_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    for analysis in &run.analyses {
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v16 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v7_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v16 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v16 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v7_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v16 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}

/// Authenticate a schema-v17 run with the exact digest encoding that wrote it
/// before measurement FAILVALUE verification evidence is admitted and the
/// document is resealed.
pub(super) fn validate_v17_result_digests(run: &ProjectSimulationRun) -> Result<(), String> {
    for analysis in &run.analyses {
        let retained = analysis
            .result_data_digest
            .as_ref()
            .copied()
            .ok_or_else(|| {
                format!(
                    "schema-v17 analysis {} is missing its result data digest",
                    analysis.id
                )
            })?;
        let computed = analysis
            .clone()
            .into_analysis()?
            .legacy_v8_result_data_digest();
        if retained != computed {
            return Err(format!(
                "schema-v17 analysis {} result data digest does not match retained content",
                analysis.id
            ));
        }
    }

    let retained = run
        .dataset_content_digest
        .as_ref()
        .copied()
        .ok_or_else(|| {
            format!(
                "schema-v17 simulation run {} is missing its dataset content digest",
                run.id
            )
        })?;
    let computed = run.clone().into_run()?.legacy_v8_dataset_content_digest();
    if retained != computed {
        return Err(format!(
            "schema-v17 simulation run {} dataset content digest does not match retained content",
            run.id
        ));
    }
    Ok(())
}
