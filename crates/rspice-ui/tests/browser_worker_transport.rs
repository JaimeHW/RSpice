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
        "protocolVersion": 17,
        "request": {
            "request": {
                "id": 1,
                "request": {"Spec": {"spec": {"MonteCarlo": {"variation_source": source}}, "options": {}}},
                "netlist": netlist,
                "source_path": null,
                "project_veriloga_runtimes": {"runtimes": [], "connections": []}
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
    assert_eq!(response["protocolVersion"], 27);
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
        let members = first["member_measurements"].as_array().unwrap();
        assert_eq!(members.len(), 4);
        for (index, member) in members.iter().enumerate() {
            assert_eq!(member["member"]["member"], "monte_carlo_sequence_trial");
            assert_eq!(member["member"]["index"].as_u64(), Some(index as u64));
            assert_eq!(member["member"]["seed"].as_u64(), Some(seed));
            assert_eq!(
                member["member"]["policy"],
                "parameter-xoroshiro128plus-2018-v1"
            );
        }
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
        "protocolVersion": 17,
        "request": {
            "request": {
                "id": 2,
                "request": {"Spec": {"spec": {"Transient": {
                    "stop_time": 1.0, "step_time": 0.01, "start_time": 0.5,
                    "max_timestep": null, "uic": false
                }}, "options": {}}},
                "netlist": "Worker convergence\nV1 out 0 1\nR1 out 0 1k\n.end\n",
                "source_path": null,
                "project_veriloga_runtimes": {"runtimes": [], "connections": []}
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
        Some(25.0)
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
        "protocolVersion": 17,
        "request": {
            "request": {"id":3,"request":{"Spec":{"spec":{"DcSweep":{
                "source_name":"V1","start":1.0,"stop":0.0,"step":-0.5,
                "source2":"V2","start2":1e-7,"stop2":3e-7,"step2":1e-7,"hysteresis":false
            }},"options":{}}},
            "netlist":"Nested worker DC\nV1 in 0 0\nV2 out 0 0\nR1 in out 1k\n.end\n","source_path":null,
            "project_veriloga_runtimes":{"runtimes":[],"connections":[]}},
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
        Some(25.0)
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

#[wasm_bindgen_test]
fn scoped_soa_current_rules_reach_the_solver_and_transfer_complete_evidence() {
    let request = serde_json::json!({
        "protocolVersion": 17,
        "request": {
            "request": {
                "id": 4,
                "request": {"Spec": {"spec": {"Soa": {
                    "stop_time": 1e-8, "step_time": 1e-9,
                    "observation": {"start_time": 2e-9, "max_step": 5e-10, "devices": ["M1"]},
                    "rules": [{"parameter": "Id", "max_value": 0.004, "models": ["NM"]}],
                    "check_vgs_max": false, "max_vgs": 1.8,
                    "check_vds_max": false, "max_vds": 3.3,
                    "check_vbe_max": false, "max_vbe": 0.9,
                    "check_vce_max": false, "max_vce": 5.0
                }}, "options": {}}},
                "netlist": "Browser SOA\nVd d 0 3\nVg g 0 2\nM1 d g 0 0 NM W=10u L=1u\n.model NM NMOS LEVEL=1 VTO=1 KP=1m LAMBDA=0\n.save V(d)\n.end\n",
                "source_path": null,
                "project_veriloga_runtimes": {"runtimes": [], "connections": []}
            },
            "dependency_metadata": "{\"snapshot_digest\":null,\"bindings\":[],\"artifacts\":[]}",
            "dependency_buffer_count": 0
        }, "buffers": []
    });
    let response =
        rspice_ui::run_rspice_ui_worker_request(js_sys::JSON::parse(&request.to_string()).unwrap())
            .unwrap();
    let response = structured_clone(&response);
    let buffers = js_sys::Reflect::get(&response, &"buffers".into())
        .unwrap()
        .dyn_into::<js_sys::Array>()
        .unwrap();
    let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
        js_sys::Reflect::get(&response, &"response".into()).unwrap(),
    )
    .unwrap();
    let soa = &metadata["outcome"]["Success"]["Soa"];
    let time = transferred_series(&buffers, &soa["time"]).to_vec();
    assert!(time[0] >= 2e-9);
    assert!(time.windows(2).all(|pair| pair[1] - pair[0] <= 5.00001e-10));
    let evaluations = soa["evaluations"]
        .as_array()
        .expect("SOA has rule evidence");
    assert_eq!(evaluations.len(), 1);
    assert_eq!(evaluations[0]["parameter"], "Id");
    assert_eq!(evaluations[0]["unit"], "A");
    assert_eq!(
        evaluations[0]["sample_count"].as_u64(),
        Some(time.len() as u64)
    );
    assert!((evaluations[0]["worst_actual_value"].as_f64().unwrap() - 0.005).abs() < 1e-8);
    let stress = soa["waveforms"]
        .as_array()
        .unwrap()
        .iter()
        .find(|trace| trace["name"] == "SOA_ID(M1)")
        .unwrap();
    let values = transferred_series(&buffers, &stress["y_values"]).to_vec();
    assert_eq!(values.len(), time.len());
    assert!(values.iter().all(|value| (*value - 0.005).abs() < 1e-8));
}

#[wasm_bindgen_test]
fn optimization_expression_reaches_a_current_target_through_the_worker() {
    let request = serde_json::json!({
        "protocolVersion": 17,
        "request": {
            "request": {
                "id": 6,
                "request": {"Spec": {"spec": {"Optimization": {
                    "variables": [{"name": "RLOAD", "min": 500.0, "max": 5000.0, "initial": 4000.0}],
                    "objective_expression": "I(V1)",
                    "objective_node": "", "objective_ref": "",
                    "goal": "Target", "target": -0.001, "algorithm": "PatternSearch",
                    "max_iterations": 180, "cost_tolerance": 1e-16,
                    "fd_step": 1e-4, "initial_step": 0.1, "min_step": 1e-8
                }}, "options": {}}},
                "netlist": "Worker current target\n.param RLOAD=2k\nV1 in 0 1\nR1 in 0 {RLOAD}\n.save V(in)\n.end\n",
                "source_path": null,
                "project_veriloga_runtimes": {"runtimes": [], "connections": []}
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
    let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
        js_sys::Reflect::get(&response, &"response".into()).unwrap(),
    )
    .unwrap();
    let result = &metadata["outcome"]["Success"]["Optimization"];
    assert_eq!(result["converged"], true, "{metadata}");
    assert!(result["best_cost"].as_f64().unwrap() <= 1e-16);
    assert!((result["best_variables"]["RLOAD"].as_f64().unwrap() - 1000.0).abs() < 0.1);
}

#[wasm_bindgen_test]
fn envelope_initializer_settings_execute_and_pss_budget_survives_worker_transport() {
    let run = |method: &str, iterations: u32| {
        let request = serde_json::json!({
            "protocolVersion": 17,
            "request": {
                "request": {
                    "id": 7,
                    "request": {"Spec": {"spec": {"Envelope": {
                        "initialization": {"max_iterations": iterations, "damping": 0.5,
                            "pss_stabilization_periods": 0, "pss_points_per_period": 64,
                            "hb_collocation_points": 17, "hb_oversample": 4,
                            "hb_use_krylov": true, "hb_gmres_restart": 12},
                        "fundamental_freq": 1e6, "additional_carrier_tones": [], "stop_time": 4e-6,
                        "num_harmonics": 3, "envelope_step": 0.5e-6,
                        "modulation_sources": ["Vmod"], "initial_periodic_solve": method,
                        "adaptive_mode": "fixed_envelope_step", "extraction_path": "projection"
                    }}, "options": {}}},
                    "netlist": "Initializer controls\nV1 in mod SIN(0 1 1Meg)\nVmod mod 0 PWL(0 0 10u 0)\nR1 in out 1k\nC1 out 0 1n\n.end\n",
                    "source_path": null,
                    "project_veriloga_runtimes": {"runtimes": [], "connections": []}
                },
                "dependency_metadata": "{\"snapshot_digest\":null,\"bindings\":[],\"artifacts\":[]}",
                "dependency_buffer_count": 0
            },
            "buffers": []
        });
        let response = rspice_ui::run_rspice_ui_worker_request(
            js_sys::JSON::parse(&request.to_string()).unwrap(),
        )
        .unwrap();
        let response = structured_clone(&response);
        serde_wasm_bindgen::from_value::<serde_json::Value>(
            js_sys::Reflect::get(&response, &"response".into()).unwrap(),
        )
        .unwrap()
    };
    for (method, expected) in [
        ("harmonic_balance", "harmonic_balance"),
        ("periodic_steady_state", "shooting"),
    ] {
        let response = run(method, 100);
        let initialization = &response["outcome"]["Success"]["Transient"]["convergence"]["metadata"]
            ["initialization"];
        assert_eq!(initialization["method"], expected, "{response}");
        assert!(
            initialization["final_residual"]
                .as_f64()
                .unwrap()
                .is_finite()
        );
        if expected == "shooting" {
            assert!(
                initialization["solver_iterations"]
                    .as_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
                    > 1
            );
        }
    }
    let failed = run("periodic_steady_state", 1);
    assert!(failed["outcome"].get("Failure").is_some(), "{failed}");
}

#[wasm_bindgen_test]
fn optimization_rejects_failed_and_overflowed_costs_through_the_worker() {
    let run = |expression: &str, goal: &str, target: f64| {
        let request = serde_json::json!({
            "protocolVersion": 17,
            "request": {
                "request": {
                    "id": 8,
                    "request": {"Spec": {"spec": {"Optimization": {
                        "variables": [{"name": "X", "min": 0.0, "max": 1.0, "initial": 0.51}],
                        "objective_expression": expression, "objective_node": "", "objective_ref": "",
                        "goal": goal, "target": target, "algorithm": "PatternSearch",
                        "max_iterations": 40, "cost_tolerance": 1e-8,
                        "fd_step": 0.1, "initial_step": 0.1, "min_step": 1e-8
                    }}, "options": {}}},
                    "netlist": "Worker objective validity\n.param X=0.51\nV1 in 0 {X}\nR1 in 0 1k\n.end\n",
                    "source_path": null,
                    "project_veriloga_runtimes": {"runtimes": [], "connections": []}
                },
                "dependency_metadata": "{\"snapshot_digest\":null,\"bindings\":[],\"artifacts\":[]}",
                "dependency_buffer_count": 0
            }, "buffers": []
        });
        let response = rspice_ui::run_rspice_ui_worker_request(
            js_sys::JSON::parse(&request.to_string()).unwrap(),
        )
        .unwrap();
        let response = structured_clone(&response);
        serde_wasm_bindgen::from_value::<serde_json::Value>(
            js_sys::Reflect::get(&response, &"response".into()).unwrap(),
        )
        .unwrap()
    };
    let valid = run("1e40*(1+V(in))/(V(in)>=0.5)", "Minimize", 0.0);
    let result = &valid["outcome"]["Success"]["Optimization"];
    let x = result["best_variables"]["X"].as_f64().unwrap();
    assert!((0.5..=0.51).contains(&x), "{valid}");
    assert!((result["best_cost"].as_f64().unwrap() / (1e40 * (1.0 + x)) - 1.0).abs() < 1e-12);
    let overflow = run("1e200*V(in)", "Target", -1e200);
    assert!(overflow["outcome"].get("Failure").is_some(), "{overflow}");
}
