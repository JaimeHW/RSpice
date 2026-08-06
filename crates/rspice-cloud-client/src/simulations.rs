use std::collections::HashSet;

use rspice_cloud_contract::{
    API_VERSION, CURRENT_SIMULATION_REQUEST_DIGEST_VERSION, CreateSimulationRunRequest,
    SimulationRun, SimulationRunStatus, Uuid, is_valid_simulation_execution_manifest,
};
use serde_json::Value;

use crate::validation::{decode_lower_hex_sha256, parse_timestamp_text, valid_timestamp_text};

const MAX_ANALYSIS_BYTES: usize = 128 * 1024;
const MAX_MANIFEST_BYTES: usize = 128 * 1024;
const MAX_RESULT_ARTIFACTS: usize = 100;
const MAX_FAILURE_CODE_BYTES: usize = 120;
const MAX_FAILURE_DETAIL_CHARACTERS: usize = 1_024;

pub(crate) fn valid_simulation_run(run: &SimulationRun) -> bool {
    if run.id.is_nil()
        || run.circuit_id.is_nil()
        || run.revision_id.is_nil()
        || run.request_digest_version != CURRENT_SIMULATION_REQUEST_DIGEST_VERSION
        || decode_lower_hex_sha256(&run.request_sha256).is_none()
        || !valid_json_object(&run.analysis, MAX_ANALYSIS_BYTES)
        || !valid_timestamp_text(&run.queued_at)
        || !valid_timestamp_text(&run.created_at)
        || !valid_timestamp_text(&run.updated_at)
        || !run.started_at.as_deref().is_none_or(valid_timestamp_text)
        || !run.completed_at.as_deref().is_none_or(valid_timestamp_text)
        || !run
            .cancellation_requested_at
            .as_deref()
            .is_none_or(valid_timestamp_text)
        || !run
            .execution_manifest
            .as_ref()
            .is_none_or(is_valid_simulation_execution_manifest)
        || !run
            .result_manifest
            .as_ref()
            .is_none_or(|manifest| valid_json_object(manifest, MAX_MANIFEST_BYTES))
        || !valid_result_artifact_ids(&run.result_artifact_ids)
    {
        return false;
    }

    valid_event_order(run) && valid_lifecycle(run)
}

pub(crate) fn simulation_run_matches_request(
    run: &SimulationRun,
    circuit_id: Uuid,
    request: &CreateSimulationRunRequest,
    replayed: bool,
) -> bool {
    !circuit_id.is_nil()
        && request
            .revision_id
            .is_none_or(|revision_id| !revision_id.is_nil())
        && valid_simulation_run(run)
        && run.circuit_id == circuit_id
        && request
            .revision_id
            .is_none_or(|revision_id| run.revision_id == revision_id)
        && run.analysis == request.analysis
        && (replayed || run.status == SimulationRunStatus::Queued)
}

pub(crate) fn valid_simulation_run_list(runs: &[SimulationRun], circuit_id: Uuid) -> bool {
    if circuit_id.is_nil()
        || runs
            .iter()
            .any(|run| run.circuit_id != circuit_id || !valid_simulation_run(run))
        || runs.iter().map(|run| run.id).collect::<HashSet<_>>().len() != runs.len()
    {
        return false;
    }

    runs.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].created_at)
            .zip(parse_timestamp_text(&pair[1].created_at))
            .is_some_and(|(newer, older)| (newer, pair[0].id) > (older, pair[1].id))
    })
}

pub(crate) fn simulation_run_api_path(run_id: Uuid) -> String {
    format!("/api/{API_VERSION}/simulation-runs/{run_id}")
}

fn valid_lifecycle(run: &SimulationRun) -> bool {
    let no_failure = run.failure_code.is_none() && run.failure_detail.is_none();
    let no_result = run.result_manifest.is_none() && run.result_artifact_ids.is_empty();

    match run.status {
        SimulationRunStatus::Queued => {
            run.started_at.is_none()
                && run.completed_at.is_none()
                && run.cancellation_requested_at.is_none()
                && run.execution_manifest.is_none()
                && no_result
                && no_failure
        }
        SimulationRunStatus::Running => {
            run.started_at.is_some()
                && run.completed_at.is_none()
                && run.execution_manifest.is_some()
                && no_result
                && no_failure
        }
        SimulationRunStatus::Succeeded => {
            run.started_at.is_some()
                && run.completed_at.is_some()
                && run.cancellation_requested_at.is_none()
                && run.execution_manifest.is_some()
                && run.result_manifest.is_some()
                && no_failure
        }
        SimulationRunStatus::Failed => {
            run.started_at.is_some()
                && run.completed_at.is_some()
                && run.cancellation_requested_at.is_none()
                && no_result
                && run.failure_code.as_deref().is_some_and(valid_failure_code)
                && run
                    .failure_detail
                    .as_deref()
                    .is_some_and(valid_failure_detail)
        }
        SimulationRunStatus::Cancelled => {
            run.completed_at.is_some()
                && no_result
                && no_failure
                && matches!(
                    (&run.started_at, &run.execution_manifest),
                    (None, None) | (Some(_), Some(_))
                )
        }
    }
}

fn valid_event_order(run: &SimulationRun) -> bool {
    let Some(queued_at) = parse_timestamp_text(&run.queued_at) else {
        return false;
    };
    let Some(created_at) = parse_timestamp_text(&run.created_at) else {
        return false;
    };
    let Some(updated_at) = parse_timestamp_text(&run.updated_at) else {
        return false;
    };
    let started_at = run.started_at.as_deref().and_then(parse_timestamp_text);
    let completed_at = run.completed_at.as_deref().and_then(parse_timestamp_text);
    let cancellation_requested_at = run
        .cancellation_requested_at
        .as_deref()
        .and_then(parse_timestamp_text);

    queued_at >= created_at
        && updated_at >= created_at
        && started_at.is_none_or(|timestamp| timestamp >= queued_at)
        && completed_at.is_none_or(|timestamp| timestamp >= started_at.unwrap_or(queued_at))
        && cancellation_requested_at.is_none_or(|timestamp| timestamp >= queued_at)
        && cancellation_requested_at
            .zip(completed_at)
            .is_none_or(|(requested, completed)| requested <= completed)
}

fn valid_json_object(value: &Value, max_bytes: usize) -> bool {
    value.is_object() && serde_json::to_vec(value).is_ok_and(|encoded| encoded.len() <= max_bytes)
}

fn valid_result_artifact_ids(ids: &[Uuid]) -> bool {
    ids.len() <= MAX_RESULT_ARTIFACTS
        && ids.iter().all(|id| !id.is_nil())
        && ids.iter().copied().collect::<HashSet<_>>().len() == ids.len()
}

fn valid_failure_code(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_FAILURE_CODE_BYTES
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

fn valid_failure_detail(value: &str) -> bool {
    !value.is_empty()
        && value.chars().count() <= MAX_FAILURE_DETAIL_CHARACTERS
        && !value.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn execution_manifest() -> Value {
        json!({
            "protocol_version": 3,
            "engine_protocol_version": 3,
            "attempt": 1,
            "worker_class": "shared",
            "engine": {
                "name": "rspice-engine",
                "build": "2026.07.20",
                "runtime_mode": "self_contained",
                "adapter_sha256": "ab".repeat(32),
                "solver_sha256": null,
                "model_library_sha256": null,
            },
            "revision": {"content_digest_version": 2},
            "request": {"digest_version": 1},
            "artifacts": [],
        })
    }

    fn queued_run() -> SimulationRun {
        SimulationRun {
            id: Uuid::from_u128(1),
            circuit_id: Uuid::from_u128(2),
            revision_id: Uuid::from_u128(3),
            status: SimulationRunStatus::Queued,
            analysis: json!({"kind": "transient", "stop": "1ms"}),
            request_digest_version: CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
            request_sha256: "0a".repeat(32),
            queued_at: "2026-07-19T00:00:00Z".to_owned(),
            started_at: None,
            completed_at: None,
            cancellation_requested_at: None,
            execution_manifest: None,
            result_manifest: None,
            result_artifact_ids: Vec::new(),
            failure_code: None,
            failure_detail: None,
            created_at: "2026-07-19T00:00:00Z".to_owned(),
            updated_at: "2026-07-19T00:00:00Z".to_owned(),
        }
    }

    #[test]
    fn simulation_responses_are_request_bound_and_lifecycle_consistent() {
        let request = CreateSimulationRunRequest {
            revision_id: Some(Uuid::from_u128(3)),
            analysis: json!({"kind": "transient", "stop": "1ms"}),
        };
        let mut run = queued_run();
        assert!(simulation_run_matches_request(
            &run,
            Uuid::from_u128(2),
            &request,
            false,
        ));

        run.circuit_id = Uuid::from_u128(4);
        assert!(!simulation_run_matches_request(
            &run,
            Uuid::from_u128(2),
            &request,
            false,
        ));

        run = queued_run();
        run.request_sha256 = "0A".repeat(32);
        assert!(!valid_simulation_run(&run));

        run = queued_run();
        run.status = SimulationRunStatus::Running;
        run.started_at = Some("2026-07-18T23:59:59Z".to_owned());
        run.execution_manifest = Some(execution_manifest());
        assert!(!valid_simulation_run(&run));

        run = queued_run();
        run.status = SimulationRunStatus::Running;
        run.started_at = Some("2026-07-19T00:00:01Z".to_owned());
        let mut malformed_provenance = execution_manifest();
        malformed_provenance["engine"]["adapter_sha256"] = json!("AB".repeat(32));
        run.execution_manifest = Some(malformed_provenance);
        assert!(!valid_simulation_run(&run));

        run = queued_run();
        run.status = SimulationRunStatus::Running;
        run.started_at = Some("2026-07-19T00:00:01Z".to_owned());
        let mut unbound_solver = execution_manifest();
        unbound_solver["engine"]["runtime_mode"] = json!("delegating");
        run.execution_manifest = Some(unbound_solver);
        assert!(!valid_simulation_run(&run));

        run = queued_run();
        run.status = SimulationRunStatus::Succeeded;
        run.started_at = Some("2026-07-19T00:00:01Z".to_owned());
        run.completed_at = Some("2026-07-19T00:00:02Z".to_owned());
        run.execution_manifest = Some(execution_manifest());
        assert!(!valid_simulation_run(&run));

        run.result_manifest = Some(json!({"format": "rspice-result-v1"}));
        run.result_artifact_ids = vec![Uuid::from_u128(5)];
        assert!(valid_simulation_run(&run));
        assert!(!simulation_run_matches_request(
            &run,
            Uuid::from_u128(2),
            &request,
            false,
        ));
        assert!(simulation_run_matches_request(
            &run,
            Uuid::from_u128(2),
            &request,
            true,
        ));

        run.result_artifact_ids.push(Uuid::from_u128(5));
        assert!(!valid_simulation_run(&run));

        run = queued_run();
        run.created_at = "2026-07-19T00:00:01Z".to_owned();
        assert!(!valid_simulation_run(&run));
    }

    #[test]
    fn simulation_pages_are_unique_scope_bound_and_strictly_newest_first() {
        let circuit_id = Uuid::from_u128(2);
        let older = queued_run();
        let mut newer = queued_run();
        newer.id = Uuid::from_u128(10);
        newer.queued_at = "2026-07-19T00:00:01Z".to_owned();
        newer.created_at = "2026-07-19T00:00:01Z".to_owned();
        newer.updated_at = "2026-07-19T00:00:01Z".to_owned();

        assert!(valid_simulation_run_list(
            &[newer.clone(), older.clone()],
            circuit_id
        ));
        assert!(!valid_simulation_run_list(
            &[older.clone(), newer.clone()],
            circuit_id
        ));
        assert!(!valid_simulation_run_list(
            &[newer.clone(), newer],
            circuit_id
        ));

        let mut wrong_scope = older;
        wrong_scope.circuit_id = Uuid::from_u128(20);
        assert!(!valid_simulation_run_list(&[wrong_scope], circuit_id));
        assert_eq!(
            simulation_run_api_path(Uuid::from_u128(10)),
            "/api/v1/simulation-runs/00000000-0000-0000-0000-00000000000a"
        );
    }
}
