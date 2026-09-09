//! Real DC solves through the numerical transfer boundary, including refusals.

use super::*;
use crate::simulation::engine_bridge::nested_dc_tests::{nested_config, solve};

fn response() -> WorkerResponse {
    WorkerResponse::from_result_for_transfer(53, Ok(solve(nested_config())))
}

fn with_metadata(
    original: &WorkerResponseTransport,
    value: serde_json::Value,
) -> WorkerResponseTransport {
    WorkerResponseTransport {
        protocol: original.protocol,
        response: serde_json::from_value(value).unwrap(),
        buffers: original.buffers.clone(),
    }
}

#[test]
fn dc_worker_transfer_preserves_exact_coordinate_quantity_and_traversal_evidence() {
    let expected = response();
    let transport = WorkerResponseTransport::from_response(expected.clone()).unwrap();
    let metadata = serde_json::to_value(&transport.response).unwrap();
    let dc = &metadata["outcome"]["Success"]["DcSweep"];
    assert_eq!(dc["waveforms"].as_array().unwrap().len(), 12);
    let reference = &dc["evidence"]["family"]["values"]["Buffer"];
    let index = reference["buffer"].as_u64().unwrap() as usize;
    assert_eq!(reference["len"], 3);
    assert_eq!(transport.buffers[index], vec![1e-7, 2e-7, 3e-7]);
    assert_eq!(dc["evidence"]["direction"], "ascending");
    assert_eq!(dc["evidence"]["quantities"].as_array().unwrap().len(), 4);
    assert_eq!(transport.into_response().unwrap(), expected);
}

#[test]
fn dc_worker_refuses_missing_or_inconsistent_curve_evidence() {
    let transport = WorkerResponseTransport::from_response(response()).unwrap();
    let original = serde_json::to_value(&transport.response).unwrap();
    let base = "/outcome/Success/DcSweep";
    for (field, value) in [
        ("/evidence", serde_json::Value::Null),
        (
            "/evidence/family/values",
            serde_json::json!({"Inline":[1e-7,2e-7,3e-7]}),
        ),
        (
            "/evidence/family/values/Buffer/buffer",
            serde_json::json!(u32::MAX),
        ),
        ("/evidence/family/values/Buffer/len", serde_json::json!(2)),
        ("/evidence/source", serde_json::json!("V9")),
        ("/evidence/quantities/0/name", serde_json::json!("renamed")),
        ("/waveforms/0/y_unit", serde_json::json!("invalid")),
        ("/waveforms/0/x_values/Buffer/len", serde_json::json!(1)),
    ] {
        let mut modified = original.clone();
        *modified.pointer_mut(&format!("{base}{field}")).unwrap() = value;
        let modified = with_metadata(&transport, modified);
        assert!(modified.into_response().is_err(), "worker accepted {field}");
    }
    let mut stale = transport;
    stale.protocol -= 1;
    assert!(stale.into_response().unwrap_err().contains("protocol"));

    let mut missing = response();
    let WorkerOutcome::Success(result) = &mut missing.outcome else {
        unreachable!()
    };
    let WorkerSimulationResult::DcSweep { evidence, .. } = result.as_mut() else {
        unreachable!()
    };
    *evidence = None;
    assert!(
        WorkerResponseTransport::from_response(missing)
            .unwrap_err()
            .contains("evidence")
    );
}

#[test]
fn dc_worker_rejects_coordinate_aliasing_and_expansion_before_copying_buffers() {
    let original = WorkerResponseTransport::from_response(response()).unwrap();
    let mut alias = serde_json::to_value(&original.response).unwrap();
    let dc = &mut alias["outcome"]["Success"]["DcSweep"];
    dc["evidence"]["family"]["values"] = dc["sweep_values"].clone();
    let alias = with_metadata(&original, alias);
    assert!(alias.into_response().is_err());

    let mut oversized = serde_json::to_value(&original.response).unwrap();
    let dc = &mut oversized["outcome"]["Success"]["DcSweep"];
    for trace in dc["waveforms"].as_array_mut().unwrap() {
        trace["y_values"] =
            serde_json::json!({"Buffer":{"buffer":0,"len":MAX_WORKER_F64_VALUES / 6}});
    }
    let oversized = with_metadata(&original, oversized);
    assert!(
        oversized
            .into_response()
            .unwrap_err()
            .contains("numeric limit")
    );
}
