//! Exercise the shipping simulation entry point and structured-clone boundary
//! in a JavaScript runtime, where JSON-only round trips cannot qualify u64s.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = structuredClone)]
    fn structured_clone(value: &JsValue) -> JsValue;
}

/// The request revision this worker accepts and the response revision it
/// answers with. The worker-contract tests in `rspice-simulation` pin both to
/// the Rust transport constants, so a protocol bump fails natively first.
const REQUEST_PROTOCOL: u8 = 48;
const RESPONSE_PROTOCOL: u8 = 38;

/// One analysis request as the browser host transfers it: the shipping
/// default execution resource policy, no dependencies and no buffers.
fn worker_request(id: u64, spec: serde_json::Value, netlist: &str) -> JsValue {
    js_sys::JSON::parse(&worker_request_json(id, spec, netlist).to_string()).unwrap()
}

fn worker_request_json(id: u64, spec: serde_json::Value, netlist: &str) -> serde_json::Value {
    serde_json::json!({
        "protocolVersion": REQUEST_PROTOCOL,
        "request": {
            "request": {
                "execution_limits": rspice_core::ResourceLimits::default(),
                "id": id,
                "request": {"Spec": {"spec": spec, "options": {}}},
                "netlist": netlist,
                "source_path": null,
                "project_veriloga_runtimes": {"runtimes": [], "connections": []}
            },
            "dependency_metadata": "{\"snapshot_digest\":null,\"bindings\":[],\"artifacts\":[]}",
            "dependency_buffer_count": 0
        },
        "buffers": [], "byteBuffers": []
    })
}

fn response_protocol(response: &JsValue) -> Option<f64> {
    js_sys::Reflect::get(response, &"protocolVersion".into())
        .unwrap()
        .as_f64()
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
    let value = worker_request(
        1,
        serde_json::json!({"MonteCarlo": {"variation_source": source}}),
        &netlist,
    );
    let response = rspice_worker::run_rspice_ui_worker_request(value)
        .expect("worker must return valid Monte Carlo results");
    let response = structured_clone(&response);
    let response: serde_json::Value = serde_wasm_bindgen::from_value(response).unwrap();
    assert_eq!(response["protocolVersion"], RESPONSE_PROTOCOL);
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
fn a_host_on_another_revision_is_refused_as_a_protocol_mismatch() {
    // A host from before revision 44 sends no `execution_limits`. The
    // refusal must name the revision, not the first field the host omitted.
    let stale = worker_request(
        5,
        serde_json::json!({"Transient": {
            "stop_time": 1.0, "step_time": 0.1, "start_time": 0.0,
            "max_timestep": null, "uic": false
        }}),
        "Stale host\nV1 out 0 1\nR1 out 0 1k\n.end\n",
    );
    js_sys::Reflect::set(
        &stale,
        &"protocolVersion".into(),
        &JsValue::from(REQUEST_PROTOCOL - 1),
    )
    .unwrap();
    let metadata = js_sys::Reflect::get(&stale, &"request".into()).unwrap();
    let request = js_sys::Reflect::get(&metadata, &"request".into()).unwrap();
    assert!(
        js_sys::Reflect::delete_property(
            request.unchecked_ref::<js_sys::Object>(),
            &"execution_limits".into()
        )
        .unwrap()
    );
    let error = rspice_worker::run_rspice_ui_worker_request(stale)
        .expect_err("a stale request revision must be refused")
        .as_string()
        .unwrap();
    assert_eq!(
        error,
        format!(
            "unsupported worker request transport protocol {}",
            REQUEST_PROTOCOL - 1
        )
    );
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
    let request = worker_request(
        2,
        serde_json::json!({"Transient": {
            "stop_time": 1.0, "step_time": 0.01, "start_time": 0.5,
            "max_timestep": null, "uic": false
        }}),
        "Worker convergence\nV1 out 0 1\nR1 out 0 1k\n.end\n",
    );
    let response = rspice_worker::run_rspice_ui_worker_request(request).unwrap();
    let response = structured_clone(&response);
    assert_eq!(
        response_protocol(&response),
        Some(f64::from(RESPONSE_PROTOCOL))
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
    let request = worker_request(
        3,
        serde_json::json!({"DcSweep": {
            "source_name": "V1", "start": 1.0, "stop": 0.0, "step": -0.5,
            "source2": "V2", "start2": 1e-7, "stop2": 3e-7, "step2": 1e-7, "hysteresis": false
        }}),
        "Nested worker DC\nV1 in 0 0\nV2 out 0 0\nR1 in out 1k\n.end\n",
    );
    let response = rspice_worker::run_rspice_ui_worker_request(request).unwrap();
    let response = structured_clone(&response);
    assert_eq!(
        response_protocol(&response),
        Some(f64::from(RESPONSE_PROTOCOL))
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
    let request = worker_request(
        4,
        serde_json::json!({"Soa": {
            "stop_time": 1e-8, "step_time": 1e-9,
            "observation": {"start_time": 2e-9, "max_step": 5e-10, "devices": ["M1"]},
            "rules": [{"parameter": "Id", "max_value": 0.004, "models": ["NM"]}],
            "check_vgs_max": false, "max_vgs": 1.8,
            "check_vds_max": false, "max_vds": 3.3,
            "check_vbe_max": false, "max_vbe": 0.9,
            "check_vce_max": false, "max_vce": 5.0
        }}),
        "Browser SOA\nVd d 0 3\nVg g 0 2\nM1 d g 0 0 NM W=10u L=1u\n.model NM NMOS LEVEL=1 VTO=1 KP=1m LAMBDA=0\n.save V(d)\n.end\n",
    );
    let response = rspice_worker::run_rspice_ui_worker_request(request).unwrap();
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
    let request = worker_request(
        6,
        serde_json::json!({"Optimization": {
            "variables": [{"name": "RLOAD", "min": 500.0, "max": 5000.0, "initial": 4000.0}],
            "objective_expression": "I(V1)",
            "objective_node": "", "objective_ref": "",
            "goal": "Target", "target": -0.001, "algorithm": "PatternSearch",
            "max_iterations": 180, "cost_tolerance": 1e-16,
            "fd_step": 1e-4, "initial_step": 0.1, "min_step": 1e-8
        }}),
        "Worker current target\n.param RLOAD=2k\nV1 in 0 1\nR1 in 0 {RLOAD}\n.save V(in)\n.end\n",
    );
    let response = rspice_worker::run_rspice_ui_worker_request(request).unwrap();
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
        let request = worker_request(
            7,
            serde_json::json!({"Envelope": {
                "initialization": {"max_iterations": iterations, "damping": 0.5,
                    "pss_stabilization_periods": 0, "pss_points_per_period": 64,
                    "hb_collocation_points": 17, "hb_oversample": 4,
                    "hb_use_krylov": true, "hb_gmres_restart": 12},
                "fundamental_freq": 1e6, "additional_carrier_tones": [], "stop_time": 4e-6,
                "num_harmonics": 3, "envelope_step": 0.5e-6,
                "modulation_sources": ["Vmod"], "initial_periodic_solve": method,
                "adaptive_mode": "fixed_envelope_step", "extraction_path": "projection"
            }}),
            "Initializer controls\nV1 in mod SIN(0 1 1Meg)\nVmod mod 0 PWL(0 0 10u 0)\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        );
        let response = rspice_worker::run_rspice_ui_worker_request(request).unwrap();
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
        let request = worker_request(
            8,
            serde_json::json!({"Optimization": {
                "variables": [{"name": "X", "min": 0.0, "max": 1.0, "initial": 0.51}],
                "objective_expression": expression, "objective_node": "", "objective_ref": "",
                "goal": goal, "target": target, "algorithm": "PatternSearch",
                "max_iterations": 40, "cost_tolerance": 1e-8,
                "fd_step": 0.1, "initial_step": 0.1, "min_step": 1e-8
            }}),
            "Worker objective validity\n.param X=0.51\nV1 in 0 {X}\nR1 in 0 1k\n.end\n",
        );
        let response = rspice_worker::run_rspice_ui_worker_request(request).unwrap();
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

/// The worker has no file system, so the dump a `d_vcd` replays travels in
/// the request, sealed as preparation read it: the model replays its edges at
/// exactly the times the VCD reader gives their ticks. Without the bytes the
/// worker has nothing to read the file from.
#[wasm_bindgen_test]
fn a_sealed_vcd_dump_travels_with_the_request_and_replays_in_the_worker() {
    const VCD: &str = "$timescale 1ps $end\n$scope module tb $end\n$var wire 1 ! clk $end\n\
                       $upscope $end\n$enddefinitions $end\n#0\n$dumpvars\n0!\n$end\n\
                       #500\n1!\n#1000\n0!\n#1500\n1!\n";
    let mut request = worker_request_json(
        9,
        serde_json::json!({"Transient": {
            "stop_time": 2e-9, "step_time": 2e-11, "start_time": 0.0,
            "max_timestep": 2e-11, "uic": false
        }}),
        "Worker VCD\nV1 in 0 1\nR1 in 0 1k\na_stim [clk] vstim\n\
         .model vstim d_vcd (file=\"clk.vcd\" signals=\"tb.clk\")\n.end\n",
    );
    request["request"]["request"]["data_files"] = serde_json::json!({"files": [{
        "path": "clk.vcd",
        "contents": VCD,
        "digest": rspice_vocabulary::canonical::content_digest(
            "rspice.code-model-data-file/v1",
            VCD.as_bytes(),
        ),
    }]});
    let run = |request: &serde_json::Value| {
        let response = rspice_worker::run_rspice_ui_worker_request(
            js_sys::JSON::parse(&request.to_string()).unwrap(),
        )
        .expect("a well-formed request is answered");
        let response = structured_clone(&response);
        serde_wasm_bindgen::from_value::<serde_json::Value>(
            js_sys::Reflect::get(&response, &"response".into()).unwrap(),
        )
        .unwrap()
    };

    let replayed = run(&request);
    let digital = replayed["outcome"]["Success"]["Transient"]["events"]["digital"]
        .as_array()
        .unwrap_or_else(|| panic!("{replayed}"));
    let clk = digital
        .iter()
        .find(|trace| {
            trace["node_name"]
                .as_str()
                .is_some_and(|name| name.eq_ignore_ascii_case("clk"))
        })
        .unwrap_or_else(|| panic!("{replayed}"));
    let document =
        rspice_core::io::parse_vcd_bytes_with_limits(VCD.as_bytes(), Default::default()).unwrap();
    let expected: Vec<(u64, u64)> = document.signals[0]
        .changes
        .iter()
        .enumerate()
        .map(|(index, change)| {
            let time = document.timescale.seconds_at_tick(change.tick).unwrap();
            (time.to_bits(), u64::from(index % 2 == 1))
        })
        .collect();
    let actual: Vec<(u64, u64)> = clk["points"]
        .as_array()
        .unwrap()
        .iter()
        .map(|point| {
            (
                point["time_s"].as_f64().unwrap().to_bits(),
                point["value_code"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(actual, expected);

    request["request"]["request"]
        .as_object_mut()
        .unwrap()
        .remove("data_files");
    let unsealed = run(&request);
    let failure = unsealed["outcome"]["Failure"].to_string();
    assert!(failure.contains("clk.vcd"), "{unsealed}");
}

/// A `.VEC` card's vector file travels the same way: sealed in the request,
/// it drives its input and checks its expected output in the worker. Input a
/// rises over 10..11 ns and output b follows it, so the row at 15 ns that
/// expects b high holds only if the input drove; the row at 18 ns (line 11)
/// expects b low and is the one violation.
#[wasm_bindgen_test]
fn a_sealed_vector_file_drives_and_checks_in_the_worker() {
    const VEC: &str = "radix 1 1\nvname a b\nio i o\ntunit ns\nvih 1.8\nvil 0\ntrise 1\n\
                       0 0 0\n10 1 0\n15 1 1\n18 1 0\n";
    let mut request = worker_request_json(
        11,
        serde_json::json!({"Transient": {
            "stop_time": 2e-8, "step_time": 2e-10, "start_time": 0.0,
            "max_timestep": 2e-10, "uic": false
        }}),
        "Worker VEC\nR1 a b 1\nR2 b 0 1meg\n.vec 'drive.vec'\n.end\n",
    );
    request["request"]["request"]["data_files"] = serde_json::json!({"files": [{
        "path": "drive.vec",
        "contents": VEC,
        "digest": rspice_vocabulary::canonical::content_digest(
            "rspice.code-model-data-file/v1",
            VEC.as_bytes(),
        ),
    }]});
    let run = |request: &serde_json::Value| {
        let response = rspice_worker::run_rspice_ui_worker_request(
            js_sys::JSON::parse(&request.to_string()).unwrap(),
        )
        .expect("a well-formed request is answered");
        let response = structured_clone(&response);
        serde_wasm_bindgen::from_value::<serde_json::Value>(
            js_sys::Reflect::get(&response, &"response".into()).unwrap(),
        )
        .unwrap()
    };

    let checked = run(&request);
    let reports = checked["outcome"]["Success"]["Transient"]["checks"]["assertions"]
        .as_array()
        .unwrap_or_else(|| panic!("{checked}"));
    let [report] = reports.as_slice() else {
        panic!("one report, the mismatch: {checked}");
    };
    assert_eq!(report["name"], "drive.vec");
    assert_eq!(report["target"], "V(b)");
    assert_eq!(report["line"].as_u64(), Some(11));
    let [violation] = report["violations"].as_array().unwrap().as_slice() else {
        panic!("one violation: {report}");
    };
    assert_eq!(
        violation["start"].as_f64().map(f64::to_bits),
        Some(18e-9f64.to_bits())
    );

    request["request"]["request"]
        .as_object_mut()
        .unwrap()
        .remove("data_files");
    let unsealed = run(&request);
    let failure = unsealed["outcome"]["Failure"].to_string();
    assert!(failure.contains("drive.vec"), "{unsealed}");
}

/// A transient's `.ASSERT` report crosses the worker exactly: the report the
/// core run publishes in this runtime, field for field and bit for bit. RC,
/// τ = 1 µs: `V(out)` rises through 0.5 V at τ·ln 2 and stays above it.
#[wasm_bindgen_test]
fn a_transient_assertion_report_crosses_the_worker_exactly() {
    const RC: &str = "* rc\nV1 in 0 PWL(0 0 1p 1)\nR1 in out 1k\nC1 out 0 1n\n\
                      .options reltol=1e-6\n.tran 10n 2u\n\
                      .ASSERT a EXPR='V(out)' MAX=0.5 LEVEL=ERROR MESSAGE='settled too high'\n.end\n";
    let request = worker_request(
        10,
        serde_json::json!({"Transient": {
            "stop_time": 2e-6, "step_time": 10e-9, "start_time": 0.0,
            "max_timestep": null, "uic": false
        }}),
        RC,
    );
    let response = rspice_worker::run_rspice_ui_worker_request(request).unwrap();
    let response = structured_clone(&response);
    assert_eq!(
        response_protocol(&response),
        Some(f64::from(RESPONSE_PROTOCOL))
    );
    let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
        js_sys::Reflect::get(&response, &"response".into()).unwrap(),
    )
    .unwrap();
    let crossed = metadata["outcome"]["Success"]["Transient"]["checks"]["assertions"]
        .as_array()
        .unwrap_or_else(|| panic!("{metadata}"));

    let netlist = rspice_core::Netlist::parse(RC).unwrap();
    // The worker resolves the window by the deck's dialect, so the native
    // run it is checked against must too.
    let max_step = rspice_core::execution::resolve_dialect_transient_maximum_step(
        netlist.parse_dialect(),
        10e-9,
        2e-6,
        None,
        None,
    )
    .unwrap();
    let result = rspice_core::Engine::default()
        .resolved_for_netlist(&netlist)
        .run_tran_with_abort(&netlist, 2e-6, max_step, &rspice_core::NoAbort)
        .unwrap();
    let expected =
        rspice_core::analysis::evaluate_tran_assertions(&netlist, &result, &rspice_core::NoAbort)
            .unwrap();
    let ([crossed], [report]) = (crossed.as_slice(), expected.as_slice()) else {
        panic!("one report each: {crossed:?} {expected:?}");
    };
    assert_eq!(crossed["name"], report.name.as_str());
    assert_eq!(crossed["analysis"], "tran");
    assert_eq!(crossed["target"], report.target.as_str());
    assert_eq!(crossed["level"], "error");
    assert_eq!(crossed["message"], "settled too high");
    assert_eq!(crossed["line"].as_u64(), Some(report.origin.line as u64));
    assert_eq!(crossed["unknown_stretches"], 0);
    assert!(crossed.get("instance").is_none() && crossed.get("error").is_none());
    let [violation] = report.violations.as_slice() else {
        panic!("one violation: {report:?}");
    };
    let bits = |value: &serde_json::Value| value.as_f64().map(f64::to_bits);
    let [moved] = crossed["violations"].as_array().unwrap().as_slice() else {
        panic!("one crossed violation: {crossed}");
    };
    assert_eq!(bits(&moved["start"]), violation.start.map(f64::to_bits));
    assert_eq!(bits(&moved["extreme"]), violation.extreme.map(f64::to_bits));
    assert_eq!(bits(&moved["at"]), violation.at.map(f64::to_bits));
    assert!(
        moved["end"].is_null() && violation.end.is_none(),
        "open at the stop time"
    );
    let start = violation.start.unwrap();
    assert!(
        (start - 1e-6 * std::f64::consts::LN_2).abs() < 0.05e-9,
        "{start:e}"
    );
}

// The raw capability exports a JIT-compiled model module imports. The worker
// binary exports them (`src/main.rs`); this test is a module of its own, so it
// exports the same four over the same library functions.

#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub extern "C" fn rspice_ui_wasm_jit_eval_op_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand0: f64,
    operand1: f64,
    operand2: f64,
    operand3: f64,
    operand4: f64,
) -> f64 {
    rspice_worker::rspice_ui_wasm_jit_eval_op_v1(
        frame_offset,
        opcode,
        aux0,
        aux1,
        aux2,
        operand0,
        operand1,
        operand2,
        operand3,
        operand4,
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn rspice_ui_wasm_jit_eval_op_slice_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand_count: i32,
) -> f64 {
    rspice_worker::rspice_ui_wasm_jit_eval_op_slice_v1(
        frame_offset,
        opcode,
        aux0,
        aux1,
        aux2,
        operand_count,
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn rspice_ui_wasm_jit_math1_v1(opcode: i32, value: f64) -> f64 {
    rspice_worker::rspice_ui_wasm_jit_math1_v1(opcode, value)
}

#[unsafe(no_mangle)]
pub extern "C" fn rspice_ui_wasm_jit_math2_v1(opcode: i32, left: f64, right: f64) -> f64 {
    rspice_worker::rspice_ui_wasm_jit_math2_v1(opcode, left, right)
}

fn property(object: &JsValue, key: &str) -> JsValue {
    js_sys::Reflect::get(object, &key.into()).unwrap()
}

/// Prepare `request` for the browser JIT and install what it needs, as
/// `simulation-worker.js` does: each model module instantiated against this
/// module's memory and raw capability exports, and a dispatcher that calls
/// an installed module's export. Returns the dispatch token to run it with.
fn install_wasm_jit_models(request: &serde_json::Value) -> u32 {
    let preparation = rspice_worker::prepare_rspice_ui_wasm_jit_request(
        js_sys::JSON::parse(&request.to_string()).unwrap(),
    )
    .expect("the request prepares for the JIT");
    let errors = property(&preparation, "errors")
        .dyn_into::<js_sys::Array>()
        .unwrap();
    assert_eq!(errors.length(), 0, "{:?}", errors.to_vec());
    let exports = wasm_bindgen::exports();
    let capabilities = js_sys::Object::new();
    for (import, export) in [
        ("memory", "memory"),
        ("eval_op_v1", "rspice_ui_wasm_jit_eval_op_v1"),
        ("eval_op_slice_v1", "rspice_ui_wasm_jit_eval_op_slice_v1"),
        ("math1_v1", "rspice_ui_wasm_jit_math1_v1"),
        ("math2_v1", "rspice_ui_wasm_jit_math2_v1"),
    ] {
        let value = property(&exports, export);
        assert!(!value.is_undefined(), "the test module exports {export}");
        js_sys::Reflect::set(&capabilities, &import.into(), &value).unwrap();
    }
    let imports = js_sys::Object::new();
    js_sys::Reflect::set(&imports, &"rspice_jit".into(), &capabilities).unwrap();
    let installed = js_sys::Map::new();
    let artifacts = property(&preparation, "artifacts")
        .dyn_into::<js_sys::Array>()
        .unwrap();
    assert!(
        artifacts.length() > 0,
        "a device runtime has a model module"
    );
    for artifact in artifacts.iter() {
        let bytes = js_sys::Uint8Array::new(&property(&artifact, "moduleBytes"));
        let module = js_sys::WebAssembly::Module::new(&bytes).expect("the model module compiles");
        let instance = js_sys::WebAssembly::Instance::new(&module, &imports)
            .expect("the model module instantiates");
        installed.set(&property(&artifact, "cacheKey"), &instance.exports());
    }
    let dispatcher = Closure::<dyn Fn(String, String, u32) -> i32>::new(
        move |cache_key: String, export: String, frame_offset: u32| {
            let entry = property(&installed.get(&cache_key.into()), &export)
                .dyn_into::<js_sys::Function>()
                .expect("the dispatched export is installed");
            let status = entry
                .call1(&JsValue::UNDEFINED, &frame_offset.into())
                .expect("the model export returns")
                .as_f64()
                .expect("a numeric status");
            status as i32
        },
    );
    rspice_worker::install_rspice_ui_wasm_jit_dispatcher(
        dispatcher
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone(),
    );
    dispatcher.forget();
    property(&preparation, "dispatchToken").as_f64().unwrap() as u32
}

/// The Verilog-A sources of a manual deck's `.hdl` lines, sealed as the
/// prepared manual-deck route seals them (`seal_deck_hdl_sources`): `roots`
/// compiled as one unit with the unified runtime options against the deck's
/// cells and selection, `module` keyed by `deck_source_key`, and named on
/// its line by that key. The browser has no file system to read the files
/// from, so the test seals the text the route would have read, headers
/// included. Returns the deck and the serialized runtime set.
fn deck_with_sealed_module(
    title: &str,
    files: &[(&str, &str)],
    roots: &[&str],
    module: &str,
) -> (String, serde_json::Value) {
    let (key, digest) = deck_key(files, roots, module);
    let deck = format!("{title}\n.veriloga \"{key}\" {module}\nX1 q {module}\nR1 q 0 1k\n.end\n");
    let netlist = rspice_core::Netlist::parse(&deck).expect("the deck parses");
    let runtime = sealed_runtime(&netlist, key, digest, files, roots, module);
    let set = rspice_simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime])
        .expect("one runtime is a set");
    (deck, serde_json::to_value(&set).unwrap())
}

/// The key and digest the prepared manual-deck route seals `module` of
/// `files` under.
fn deck_key(
    files: &[(&str, &str)],
    roots: &[&str],
    module: &str,
) -> (String, rspice_vocabulary::product::ContentDigest) {
    let text: String = files
        .iter()
        .map(|(name, text)| format!("{name}\n{text}"))
        .collect();
    let digest = rspice_vocabulary::canonical::content_digest("rspice.test/unit", text.as_bytes());
    let file_name = roots[0].rsplit('/').next().unwrap();
    let key =
        rspice_simulation::netlist_preparation::deck_source_key(digest, module, module, file_name);
    (key, digest)
}

/// `module` of `files` compiled with the unified runtime options against
/// `netlist`'s cells and selection, and sealed under `key`.
fn sealed_runtime(
    netlist: &rspice_core::Netlist,
    key: String,
    digest: rspice_vocabulary::product::ContentDigest,
    files: &[(&str, &str)],
    roots: &[&str],
    module: &str,
) -> rspice_simulation::veriloga::PreparedVerilogARuntime {
    let cells = rspice_core::engine::hdl_external_cells(netlist);
    let delay_select =
        rspice_core::engine::hdl_delay_select(&rspice_core::resolve_simulation_config(
            &rspice_core::SimulationConfig::default(),
            Some(&netlist.options),
            &rspice_core::SimulationConfigOverrides::default(),
        ));
    let bundle = rspice_veriloga::VirtualSourceBundle::new(
        roots[0].to_owned(),
        files
            .iter()
            .map(|(name, text)| rspice_veriloga::VirtualSourceFile::new((*name).to_owned(), *text)),
    )
    .unwrap();
    let compiler = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
        external_cells: Some(cells.clone()),
        delay_select,
        wreal_resolution: rspice_core::engine::hdl_wreal_resolution(netlist),
        ..rspice_simulation::compilation::unified_runtime_compiler_options()
    });
    let limits = rspice_simulation::project_veriloga::project_virtual_compile_limits();
    let prepared = if roots.len() > 1 {
        compiler.prepare_virtual_unit_runtime_source(&bundle, roots, limits)
    } else {
        compiler.prepare_virtual_runtime_source(&bundle, limits)
    };
    let compilation = prepared
        .expect("the sources prepare")
        .compile_runtime(module)
        .expect("the module compiles");
    rspice_simulation::veriloga::PreparedVerilogARuntime::try_from_virtual_compilation_against(
        key,
        digest,
        module.to_owned(),
        &compilation,
        Some(&cells),
        delay_select,
    )
    .expect("the runtime seals")
}

const CLOCK: &str = "`timescale 1ns/1ps\nmodule clk(q);\n  output q;\n  reg q;\n  \
                     initial q = 1'b0;\n  always #5 q = ~q;\nendmodule\n";

/// A manual deck's sealed HDL runtime reaches the worker as a project
/// runtime does, in the request's runtime set, and runs there.
#[wasm_bindgen_test]
fn a_manual_decks_sealed_hdl_runs_in_the_worker() {
    let (deck, runtimes) =
        deck_with_sealed_module("Worker HDL clock", &[("clk.v", CLOCK)], &["clk.v"], "clk");
    assert_sealed_clock_runs_in_the_worker(11, &deck, runtimes);
}

/// Two files compiled as one unit, one reading its half period from a local
/// header, run in the worker from the sealed bytes alone: `top` in one file
/// instantiates `leaf` from another, whose `` `include "half.vh" `` the
/// seal carried, and `q` toggles every 5 ns as it does on the desktop.
#[wasm_bindgen_test]
fn a_two_file_unit_with_a_local_header_runs_in_the_worker() {
    const TOP: &str = "`timescale 1ns/1ps\nmodule top(q);\n  output q;\n  wire q;\n  \
                       leaf u1(.y(q));\nendmodule\n";
    const LEAF: &str = "`include \"half.vh\"\n`timescale 1ns/1ps\nmodule leaf(y);\n  \
                        output y;\n  reg y;\n  initial y = 1'b0;\n  always #`HALF y = ~y;\n\
                        endmodule\n";
    let (deck, runtimes) = deck_with_sealed_module(
        "Worker HDL unit",
        &[
            ("rtl/top.v", TOP),
            ("lib/leaf.v", LEAF),
            ("lib/half.vh", "`define HALF 5\n"),
        ],
        &["rtl/top.v", "lib/leaf.v"],
        "top",
    );
    assert_sealed_clock_runs_in_the_worker(13, &deck, runtimes);
}

/// A project's own Verilog-A device runs in the worker beside an HDL
/// process: the project's `vres`, a 1 kΩ resistor as `I(p, n) <+ V(p, n) / r`
/// sealed under its project key, loads the clock's `q` where the other
/// decks put a deck resistor, and `V(q)` steps between the same levels.
#[wasm_bindgen_test]
fn a_project_verilog_a_device_loads_an_hdl_process_in_the_worker() {
    const VRES: &str = "`include \"disciplines.vams\"\nmodule vres(p, n);\n  inout p, n;\n  \
                        electrical p, n;\n  parameter real r = 1000.0;\n  \
                        analog I(p, n) <+ V(p, n) / r;\nendmodule\n";
    let clock = [("clk.v", CLOCK)];
    let (clock_key, clock_digest) = deck_key(&clock, &["clk.v"], "clk");
    let device = [("vres.va", VRES)];
    let device_key = "__rspice_project__/fixture/vres.va".to_owned();
    let device_digest =
        rspice_vocabulary::canonical::content_digest("rspice.test/project", VRES.as_bytes());
    let deck = format!(
        "Worker Verilog-A beside HDL\n.veriloga \"{clock_key}\" clk\n\
         .veriloga \"{device_key}\" vres\nX1 q clk\nX2 q 0 vres\n.end\n"
    );
    let netlist = rspice_core::Netlist::parse(&deck).expect("the deck parses");
    let runtimes = vec![
        sealed_runtime(&netlist, clock_key, clock_digest, &clock, &["clk.v"], "clk"),
        sealed_runtime(
            &netlist,
            device_key,
            device_digest,
            &device,
            &["vres.va"],
            "vres",
        ),
    ];
    let set = rspice_simulation::veriloga::PreparedVerilogARuntimeSet::try_new(runtimes)
        .expect("two runtimes are a set");
    assert_sealed_clock_runs_in_the_worker(21, &deck, serde_json::to_value(&set).unwrap());
}

/// An instance whose parameter reshapes its module compiles anew when the
/// circuit is built, to a JIT model no sealed runtime has. The worker's
/// preparation builds the circuit once to find each such model, so both
/// instances, one width each, run on models the browser installed first.
#[wasm_bindgen_test]
fn instances_a_parameter_recompiles_run_on_models_the_worker_prepared() {
    const TICK: &str = "`include \"disciplines.vams\"\n`timescale 1ns/1ps\nmodule tick(p);\n  \
                        inout p;\n  electrical p;\n  parameter integer width = 1;\n  \
                        reg [width-1:0] count;\n  initial count = 0;\n  \
                        always #5 count = count + 1;\n  analog I(p) <+ V(p) / 1000.0;\nendmodule\n";
    let files = [("tick.vams", TICK)];
    let (key, digest) = deck_key(&files, &["tick.vams"], "tick");
    let deck = format!(
        "Worker specialized instances\n.veriloga \"{key}\" tick\nV1 a 0 1\n\
         X1 a tick width=2\nX2 a tick width=3\n.end\n"
    );
    let netlist = rspice_core::Netlist::parse(&deck).expect("the deck parses");
    let runtime = sealed_runtime(&netlist, key, digest, &files, &["tick.vams"], "tick");
    let set = rspice_simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime])
        .expect("one runtime is a set");
    let mut request = worker_request_json(
        23,
        serde_json::json!({"Transient": {
            "stop_time": 20e-9, "step_time": 1e-10, "start_time": 0.0,
            "max_timestep": 1e-10, "uic": false
        }}),
        &deck,
    );
    request["request"]["request"]["project_veriloga_runtimes"] =
        serde_json::to_value(&set).unwrap();
    // The sealed runtime's model, and one per width the build compiles.
    let preparation = rspice_worker::prepare_rspice_ui_wasm_jit_request(
        js_sys::JSON::parse(&request.to_string()).unwrap(),
    )
    .expect("the request prepares for the JIT");
    let artifacts = property(&preparation, "artifacts")
        .dyn_into::<js_sys::Array>()
        .unwrap();
    assert_eq!(artifacts.length(), 3);
    rspice_worker::cancel_prepared_rspice_ui_wasm_jit_request(
        property(&preparation, "dispatchToken").as_f64().unwrap() as u32,
    )
    .unwrap();
    let dispatch_token = install_wasm_jit_models(&request);
    let response = rspice_worker::run_prepared_rspice_ui_wasm_jit_request(dispatch_token)
        .expect("a prepared request is answered");
    let response = structured_clone(&response);
    let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
        js_sys::Reflect::get(&response, &"response".into()).unwrap(),
    )
    .unwrap();
    assert!(
        metadata["outcome"]["Success"]["Transient"].is_object(),
        "{metadata}"
    );
}

/// A file the HDL reads that the run did not seal is a located warning in
/// the browser, for `$readmemh` and for a read-mode `$fopen` alike: the
/// browser has no file system, so the message names the cause, and each
/// record carries the call's file and line to the run's checks.
#[wasm_bindgen_test]
fn an_unsealed_file_the_hdl_reads_is_a_located_warning_in_the_worker() {
    const ROM: &str = "`timescale 1ns/1ps\nmodule rom(q);\n  output q;\n  reg q;\n  \
                       reg [7:0] mem [0:3];\n  integer fd;\n  initial begin\n    q = 1'b0;\n    \
                       $readmemh(\"rom.hex\", mem);\n    fd = $fopen(\"rom.hex\", \"r\");\n    \
                       #5 q = 1'b1;\n  end\nendmodule\n";
    let (deck, runtimes) =
        deck_with_sealed_module("Worker unsealed file", &[("rom.v", ROM)], &["rom.v"], "rom");
    let mut request = worker_request_json(
        22,
        serde_json::json!({"Transient": {
            "stop_time": 10e-9, "step_time": 1e-10, "start_time": 0.0,
            "max_timestep": 1e-10, "uic": false
        }}),
        &deck,
    );
    request["request"]["request"]["project_veriloga_runtimes"] = runtimes;
    let dispatch_token = install_wasm_jit_models(&request);
    let response = rspice_worker::run_prepared_rspice_ui_wasm_jit_request(dispatch_token)
        .expect("a prepared request is answered");
    let response = structured_clone(&response);
    let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
        js_sys::Reflect::get(&response, &"response".into()).unwrap(),
    )
    .unwrap();
    let records = metadata["outcome"]["Success"]["Transient"]["checks"]["hdl"]
        .as_array()
        .unwrap_or_else(|| panic!("the run completes with its checks: {metadata}"));
    let missing = "`rom.hex` is not among the run's sealed files";
    let located: Vec<(String, u64, String)> = records
        .iter()
        .map(|record| {
            (
                record["text"].as_str().unwrap_or_default().to_owned(),
                record["origin"]["line"].as_u64().unwrap_or_default(),
                record["origin"]["file"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            )
        })
        .collect();
    for (task, line) in [("$readmemh", 9), ("$fopen", 10)] {
        let record = records
            .iter()
            .zip(&located)
            .find(|(_, (text, _, _))| text.starts_with(task))
            .unwrap_or_else(|| panic!("no {task} record: {located:?}"));
        let (text, at, file) = record.1;
        assert_eq!(text, &format!("{task}: {missing}"), "{located:?}");
        assert_eq!(*at, line, "{located:?}");
        assert!(file.ends_with("rom.v"), "{located:?}");
        assert!(
            record.0["stream"].to_string().contains("Warning"),
            "a warning: {}",
            record.0
        );
    }
}

/// The worker runs `deck` with its sealed runtime set: `q` toggles every
/// 5 ns, and the default `d2a` drives the 1 kΩ load through its 20 Ω
/// `rhi`/`rlo` from 3.3 V or 0 V.
fn assert_sealed_clock_runs_in_the_worker(id: u64, deck: &str, runtimes: serde_json::Value) {
    let mut request = worker_request_json(
        id,
        serde_json::json!({"Transient": {
            "stop_time": 20e-9, "step_time": 1e-10, "start_time": 0.0,
            "max_timestep": 1e-10, "uic": false
        }}),
        deck,
    );
    request["request"]["request"]["project_veriloga_runtimes"] = runtimes;
    let dispatch_token = install_wasm_jit_models(&request);
    let response = rspice_worker::run_prepared_rspice_ui_wasm_jit_request(dispatch_token)
        .expect("a prepared request is answered");
    let response = structured_clone(&response);
    let buffers = js_sys::Reflect::get(&response, &"buffers".into())
        .unwrap()
        .dyn_into::<js_sys::Array>()
        .unwrap();
    let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
        js_sys::Reflect::get(&response, &"response".into()).unwrap(),
    )
    .unwrap();
    let transient = &metadata["outcome"]["Success"]["Transient"];
    assert!(transient.is_object(), "{metadata}");

    let digital = transient["events"]["digital"]
        .as_array()
        .unwrap_or_else(|| panic!("{metadata}"));
    let q = digital
        .iter()
        .find(|trace| {
            trace["node_name"]
                .as_str()
                .is_some_and(|name| name.eq_ignore_ascii_case("q"))
        })
        .unwrap_or_else(|| panic!("no digital q: {metadata}"));
    let events: Vec<(f64, u64)> = q["points"]
        .as_array()
        .unwrap()
        .iter()
        .map(|point| {
            (
                point["time_s"].as_f64().unwrap(),
                point["value_code"].as_u64().unwrap(),
            )
        })
        .collect();
    let expected = [(0.0, 0), (5e-9, 1), (10e-9, 0), (15e-9, 1), (20e-9, 0)];
    assert!(events.len() >= 4, "{events:?}");
    for ((time, value), (want_time, want_value)) in events.iter().zip(expected) {
        assert!((time - want_time).abs() < 1e-15, "{events:?}");
        assert_eq!(*value, want_value, "{events:?}");
    }

    let time = transferred_series(&buffers, &transient["time"]).to_vec();
    let waveforms = transient["waveforms"]
        .as_array()
        .unwrap_or_else(|| panic!("{metadata}"));
    let v_q = waveforms
        .iter()
        .find(|trace| {
            trace["y_unit"] == "V"
                && trace["name"]
                    .as_str()
                    .is_some_and(|name| name.eq_ignore_ascii_case("q"))
        })
        .unwrap_or_else(|| panic!("no node voltage q: {metadata}"));
    let v_q = transferred_series(&buffers, &v_q["y_values"]).to_vec();
    assert_eq!(v_q.len(), time.len());
    let at = |t: f64| {
        let index = time
            .iter()
            .position(|sample| *sample >= t)
            .unwrap_or_else(|| panic!("no sample at {t:e}"));
        v_q[index]
    };
    let high = 3.3 * 1000.0 / 1020.0;
    for (t, want) in [
        (2.5e-9, 0.0),
        (7.5e-9, high),
        (12.5e-9, 0.0),
        (17.5e-9, high),
    ] {
        let got = at(t);
        assert!(
            (got - want).abs() < 1e-6,
            "V(q) at {t:e}: {got}, want {want}"
        );
    }
}

#[wasm_bindgen_test]
fn a_monte_carlo_trials_transient_runs_in_the_worker() {
    // Trial 2 of the deck's own study: the worker draws the resistance that
    // trial draws and returns the trial's whole transient.
    let deck = "MC trial transient\n.param r={aunif(1000,200)}\nV1 in 0 PULSE(0 1 1u 0.1u 0.1u 5u 10u)\nR1 in out {r}\nC1 out 0 1n\n.tran 0.1u 10u\n.mc 4 seed 9\n.end\n";
    let trial = |index: u64| {
        let mut request = worker_request_json(
            12 + index,
            serde_json::json!({"Transient": {
                "stop_time": 10e-6, "step_time": 1e-7, "start_time": 0.0,
                "max_timestep": null, "uic": false
            }}),
            deck,
        );
        request["request"]["request"]["request"]["Spec"]["options"]["member"] = serde_json::json!({
            "member": "monte_carlo_trial", "index": index, "variation_source": "deck_statistics"
        });
        let response = rspice_worker::run_rspice_ui_worker_request(
            js_sys::JSON::parse(&request.to_string()).unwrap(),
        )
        .expect("a Monte Carlo trial's transient is answered");
        let response = structured_clone(&response);
        let buffers = js_sys::Reflect::get(&response, &"buffers".into())
            .unwrap()
            .dyn_into::<js_sys::Array>()
            .unwrap();
        let metadata: serde_json::Value = serde_wasm_bindgen::from_value(
            js_sys::Reflect::get(&response, &"response".into()).unwrap(),
        )
        .unwrap();
        let transient = &metadata["outcome"]["Success"]["Transient"];
        assert!(transient.is_object(), "{metadata}");
        let time = transferred_series(&buffers, &transient["time"]).to_vec();
        let out = transient["waveforms"]
            .as_array()
            .unwrap_or_else(|| panic!("{metadata}"))
            .iter()
            .find(|trace| {
                trace["name"]
                    .as_str()
                    .is_some_and(|name| name.eq_ignore_ascii_case("out"))
            })
            .unwrap_or_else(|| panic!("no node voltage out: {metadata}"));
        let out = transferred_series(&buffers, &out["y_values"]).to_vec();
        assert_eq!(out.len(), time.len());
        assert!(time.len() > 20, "a whole waveform: {} points", time.len());
        (time, out)
    };
    let (time, two) = trial(2);
    let (_, again) = trial(2);
    let (_, three) = trial(3);
    // The same trial draws the same resistance; another trial draws another,
    // so its RC charges along another curve.
    assert_eq!(two, again);
    let during_rise = time.iter().position(|t| *t >= 2e-6).unwrap();
    assert_ne!(two[during_rise], three[during_rise]);
    assert!(two[during_rise] > 0.0 && two[during_rise] < 1.0);
}
