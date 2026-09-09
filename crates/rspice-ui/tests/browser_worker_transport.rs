//! Exercise the shipping simulation entry point and structured-clone boundary
//! in a JavaScript runtime, where JSON-only round trips cannot qualify u64s.

#![cfg(all(target_arch = "wasm32", feature = "browser-worker"))]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = structuredClone)]
    fn structured_clone(value: &JsValue) -> JsValue;
}

fn transferred_series(
    buffers: &js_sys::Array,
    reference: &serde_json::Value,
) -> js_sys::Float64Array {
    let reference = &reference["Buffer"];
    let index = u32::try_from(reference["buffer"].as_u64().unwrap()).unwrap();
    assert!(index < buffers.length());
    let values = buffers
        .get(index)
        .dyn_into::<js_sys::Float64Array>()
        .expect("structuredClone preserves the actual Float64Array");
    assert_eq!(Some(u64::from(values.length())), reference["len"].as_u64());
    values
}

fn run(seed: u64, source: &str) -> serde_json::Value {
    let resistance = if source == "deck_statistics" {
        "{aunif(1000,50)}"
    } else {
        "1000"
    };
    let netlist = format!(
        "MC worker boundary\n.param rval={resistance}\nV1 in 0 1\nR1 in out {{rval}}\nR2 out 0 1k\n.mc 4 seed {seed} DIST UNIFORM SPREAD .05\n.end ; done\n"
    );
    let request = serde_json::json!({
        "protocolVersion": 9,
        "request": {
            "request": {
                "id": 1,
                "request": {"Spec": {"spec": {"MonteCarlo": {"variation_source": source}}, "options": {}}},
                "netlist": netlist,
                "source_path": null
            },
            "dependency_metadata": "{\"snapshot_digest\":null,\"bindings\":[],\"artifacts\":[]}",
            "dependency_buffer_count": 0
        },
        "buffers": []
    });
    let value = js_sys::JSON::parse(&request.to_string()).unwrap();
    let response = rspice_ui::run_rspice_ui_worker_request(value)
        .expect("worker must return valid Monte Carlo results");
    let response = structured_clone(&response);
    let response: serde_json::Value = serde_wasm_bindgen::from_value(response).unwrap();
    assert_eq!(response["protocolVersion"], 19);
    assert_eq!(response["response"]["id"].as_u64(), Some(1));
    let result = response["response"]["outcome"]["Success"]["Inline"]["MonteCarlo"].clone();
    assert_eq!(result["seed"].as_u64(), Some(seed), "{response}");
    assert_eq!(result["runs_completed"], 4);
    assert_eq!(result["num_failures"], 0);
    result
}

#[wasm_bindgen_test]
fn complete_seed_range_survives_the_worker_and_structured_clone() {
    for seed in [0, (1_u64 << 53) + 1, u64::MAX] {
        let first = run(seed, "parameter_tolerance");
        assert_eq!(first, run(seed, "parameter_tolerance"));
    }
}

#[wasm_bindgen_test]
fn deck_statistics_retain_full_width_trial_identities() {
    let result = run(7, "deck_statistics");
    let members = result["member_measurements"].as_array().unwrap();
    assert_eq!(members.len(), 4);
    assert_eq!(members[0]["member"]["member"], "monte_carlo_trial");
    assert_eq!(
        members[0]["member"]["seed"].as_u64(),
        Some(1_346_066_267_577_507_604)
    );
    for (index, member) in members.iter().enumerate() {
        assert_eq!(member["member"]["index"].as_u64(), Some(index as u64));
    }
    assert_eq!(result, run(7, "deck_statistics"));
}

#[wasm_bindgen_test]
fn transient_quality_survives_the_worker_and_structured_clone_before_output_cropping() {
    let request = serde_json::json!({
        "protocolVersion": 9,
        "request": {
            "request": {
                "id": 2,
                "request": {"Spec": {"spec": {"Transient": {
                    "stop_time": 1.0, "step_time": 0.01, "start_time": 0.5,
                    "max_timestep": null, "uic": false
                }}, "options": {}}},
                "netlist": "Worker convergence\nV1 out 0 1\nR1 out 0 1k\n.end\n",
                "source_path": null
            },
            "dependency_metadata": "{\"snapshot_digest\":null,\"bindings\":[],\"artifacts\":[]}",
            "dependency_buffer_count": 0
        },
        "buffers": []
    });
    let response =
        rspice_ui::run_rspice_ui_worker_request(js_sys::JSON::parse(&request.to_string()).unwrap())
            .unwrap();
    let response = structured_clone(&response);
    assert_eq!(
        js_sys::Reflect::get(&response, &"protocolVersion".into())
            .unwrap()
            .as_f64(),
        Some(19.0)
    );
    let buffers = js_sys::Reflect::get(&response, &"buffers".into())
        .unwrap()
        .dyn_into::<js_sys::Array>()
        .expect("the worker retains a separate numeric buffer array");
    let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
        js_sys::Reflect::get(&response, &"response".into()).unwrap(),
    )
    .unwrap();
    assert_eq!(metadata["id"], 2);
    let series = |reference: &serde_json::Value| transferred_series(&buffers, reference);
    let transient = &metadata["outcome"]["Success"]["Transient"];
    let quality = &transient["convergence"]["metadata"]["transient"];
    let basis = &quality["time_basis"];
    assert_eq!(basis["start_s"], 0.0);
    assert_eq!(basis["stop_s"], 1.0);
    let source_samples: u64 = basis["sample_count"].as_str().unwrap().parse().unwrap();
    let time = series(&transient["time"]);
    assert!(source_samples > u64::from(time.length()));
    assert_eq!(time.get_index(0), 0.5);
    assert_eq!(time.get_index(time.length() - 1), 1.0);
    assert_eq!(quality["force_accepted_points"], "0");
    for field in ["transient_indices", "transient_times"] {
        assert_eq!(series(&transient["convergence"][field]).length(), 0);
    }
}

#[wasm_bindgen_test]
fn nested_dc_curves_and_exact_traversal_survive_the_worker_and_structured_clone() {
    let request = serde_json::json!({
        "protocolVersion": 9,
        "request": {
            "request": {"id":3,"request":{"Spec":{"spec":{"DcSweep":{
                "source_name":"V1","start":1.0,"stop":0.0,"step":-0.5,
                "source2":"V2","start2":1e-7,"stop2":3e-7,"step2":1e-7,"hysteresis":false
            }},"options":{}}},
            "netlist":"Nested worker DC\nV1 in 0 0\nV2 out 0 0\nR1 in out 1k\n.end\n","source_path":null},
            "dependency_metadata":"{\"snapshot_digest\":null,\"bindings\":[],\"artifacts\":[]}",
            "dependency_buffer_count":0
        },"buffers":[]
    });
    let response =
        rspice_ui::run_rspice_ui_worker_request(js_sys::JSON::parse(&request.to_string()).unwrap())
            .unwrap();
    let response = structured_clone(&response);
    assert_eq!(
        js_sys::Reflect::get(&response, &"protocolVersion".into())
            .unwrap()
            .as_f64(),
        Some(19.0)
    );
    let buffers = js_sys::Reflect::get(&response, &"buffers".into())
        .unwrap()
        .dyn_into::<js_sys::Array>()
        .unwrap();
    let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
        js_sys::Reflect::get(&response, &"response".into()).unwrap(),
    )
    .unwrap();
    assert_eq!(metadata["id"], 3);
    let dc = &metadata["outcome"]["Success"]["DcSweep"];
    assert_eq!(dc["evidence"]["direction"], "descending");
    assert_eq!(dc["evidence"]["source"], "V1");
    assert_eq!(dc["evidence"]["family"]["source"], "V2");
    assert_eq!(
        transferred_series(&buffers, &dc["evidence"]["family"]["values"]).to_vec(),
        [1e-7, 2e-7, 3e-7]
    );
    assert_eq!(
        transferred_series(&buffers, &dc["sweep_values"]).to_vec(),
        [0.0, 0.5, 1.0]
    );
    let curves = dc["waveforms"]
        .as_array()
        .expect("a successful family retains curves");
    assert_eq!(curves.len(), 12);
    let mut names = std::collections::HashSet::new();
    let mut secondary = Vec::new();
    for curve in curves {
        let name = curve["name"].as_str().unwrap();
        assert!(names.insert(name));
        let current = name.starts_with("I(");
        assert_eq!(curve["y_unit"], if current { "A" } else { "V" });
        assert_eq!(
            transferred_series(&buffers, &curve["x_values"]).to_vec(),
            [0.0, 0.5, 1.0]
        );
        let y = transferred_series(&buffers, &curve["y_values"]).to_vec();
        if name.starts_with("V(OUT)") {
            assert!(y.iter().all(|v| *v == y[0]));
            secondary.push(y[0]);
        } else if name.starts_with("V(IN)") {
            assert_eq!(y, [0.0, 0.5, 1.0]);
        }
    }
    secondary.sort_by(f64::total_cmp);
    assert_eq!(secondary.len(), 3);
    for (actual, expected) in secondary.into_iter().zip([1e-7, 2e-7, 3e-7]) {
        assert!((actual - expected).abs() < 1e-20);
    }
}
