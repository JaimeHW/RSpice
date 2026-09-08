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
    assert_eq!(response["protocolVersion"], 17);
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
