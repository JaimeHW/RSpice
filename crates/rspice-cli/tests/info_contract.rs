//! Inspection flags and machine output must describe the same parsed deck.
mod common;

use std::io::Write;
use std::process::{Command, Stdio};

fn inspect(deck: &str, flags: &[&str]) -> String {
    let directory = common::test_dir("info");
    let path = directory.join("deck.cir");
    std::fs::write(&path, deck).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "info"])
        .arg(path)
        .args(flags)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn detailed_json_reports_connections_sources_and_scoped_expressions() {
    let deck = "Detailed inspection\nV1 in 0 DC 1 AC 2 45 PULSE(0 5 1n 2n 3n 4n 10n)\nR1 in out 1k\nC1 out 0 2n IC=0.3\nB1 sense 0 V={V(out)*2}\n.subckt amp p n params: r=2k\nRload p n {r}\nE1 drive n p n {r/1000}\nVlocal p n PULSE(0 {r} 0 1n 1n 2n 4n)\n.ends amp\nX1 out 0 amp r=3k\n.end\n";
    let json: serde_json::Value =
        serde_json::from_str(&inspect(deck, &["--detailed", "--hierarchy", "--json"])).unwrap();
    let elements = json["element_details"].as_array().unwrap();
    assert_eq!(elements.len(), 5);
    assert_eq!(elements[0]["name"], "V1");
    assert_eq!(elements[0]["nodes"], serde_json::json!(["IN", "0"]));
    let source = &elements[0]["specification"]["source"];
    assert_eq!(source["kind"], "dc_ac_transient");
    assert_eq!(source["dc_value"], 1.0);
    assert_eq!(source["ac_magnitude"], 2.0);
    assert_eq!(source["transient"]["kind"], "pulse");
    assert_eq!(source["transient"]["v2"], 5.0);
    assert_eq!(source["transient"]["period"], 1e-8);
    assert_eq!(elements[1]["specification"]["value"], 1000.0);
    assert_eq!(elements[2]["specification"]["initial_voltage"], 0.3);
    assert_eq!(elements[3]["specification"]["kind"], "behavioral_voltage");
    assert!(
        elements[3]["specification"]["expression"]
            .as_str()
            .unwrap()
            .contains('*')
    );
    let local = &json["hierarchy"]["definitions"][0]["element_details"];
    assert_eq!(local[0]["name"], "RLOAD");
    assert!(local[0]["specification"]["value"].is_null());
    assert_eq!(local[0]["specification"]["value_expr"], "r");
    assert_eq!(
        local[1]["specification"]["control_nodes"],
        serde_json::json!(["P", "N"])
    );
    assert_eq!(local[1]["specification"]["gain_expr"], "r/1000");
    assert_eq!(local[2]["specification"]["kind"], "voltage_source");
    assert!(
        local[2]["specification"]["source_expression"]
            .as_str()
            .unwrap()
            .contains("{r}")
    );
    let text = inspect(deck, &["--detailed", "--hierarchy"]);
    assert!(text.contains("R1 (resistor): IN OUT"), "{text}");
    assert!(text.contains("value: 1000.0"), "{text}");
    assert!(text.contains("RLOAD (resistor): P N"), "{text}");
    assert!(!text.contains("Some("), "{text}");
    let summary: serde_json::Value =
        serde_json::from_str(&inspect(deck, &["--hierarchy", "--json"])).unwrap();
    assert!(summary["element_details"].is_null());
    assert!(summary["hierarchy"]["definitions"][0]["element_details"].is_null());
}

#[test]
fn detailed_mixed_signal_ports_preserve_vector_inversion() {
    let deck = "Digital inspection\n.model gate d_and(rise_delay=1n fall_delay=2n)\nA1 [din ~enable] dout gate\nA2 [din enable] out2 gate\n.end\n";
    let json: serde_json::Value =
        serde_json::from_str(&inspect(deck, &["--detailed", "--json"])).unwrap();
    let elements = &json["element_details"];
    assert_eq!(elements[0]["specification"]["kind"], "xspice");
    assert_eq!(elements[0]["specification"]["model"], "GATE");
    let port = &elements[0]["specification"]["ports"][0];
    assert_eq!(port["kind"], "digital_vector");
    assert_eq!(port["nodes"][0]["inverted"], false);
    assert_eq!(port["nodes"][1]["inverted"], true);
    assert_eq!(port["nodes"][1]["name"], "ENABLE");
    let ordinary = &elements[1]["specification"]["ports"][0];
    assert_eq!(ordinary["kind"], "digital_vector");
    assert_eq!(ordinary["nodes"][1]["inverted"], false);
    assert_eq!(ordinary["nodes"][1]["name"], "ENABLE");
}

#[test]
fn detailed_waveform_defaults_are_distinct_from_zero_and_optional_fields() {
    let deck = "Default waveform\nV1 in 0 PULSE(0 1)\nV2 out 0 SIN(0 1)\n.end\n";
    let json: serde_json::Value =
        serde_json::from_str(&inspect(deck, &["--detailed", "--json"])).unwrap();
    let pulse = &json["element_details"][0]["specification"]["source"];
    assert_eq!(pulse["delay"], 0.0);
    assert_eq!(pulse["rise"], "default");
    assert_eq!(pulse["width"], "default");
    assert_eq!(pulse["period"], "default");
    assert_eq!(
        json["element_details"][1]["specification"]["source"]["frequency"],
        "default"
    );
    let text = inspect(deck, &["--detailed"]);
    assert!(text.contains("default"), "{text}");
    assert!(!text.contains("NaN"), "{text}");
}

#[test]
fn parameter_inspection_preserves_strings_complex_values_and_runtime_expressions() {
    let deck = "Parameter inspection\n.param z={2+3j} label=\"run A\" live={time+1} gain=2\n.global_param global_only={time*2} mask={time+4}\n.param mask=\"local\"\n.end\n";
    let json: serde_json::Value = serde_json::from_str(&inspect(
        deck,
        &["--params", "--json", "--spice-dialect", "xyce"],
    ))
    .unwrap();
    let definitions = json["parameter_definitions"].as_array().unwrap();
    let names: Vec<_> = definitions
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["GAIN", "GLOBAL_ONLY", "LABEL", "LIVE", "MASK", "Z"]);
    assert_eq!(definitions[1]["kind"], "expression");
    assert_eq!(definitions[1]["value"], "time*2");
    assert_eq!(definitions[2]["kind"], "string");
    assert_eq!(definitions[2]["value"], "run A");
    assert_eq!(definitions[3]["kind"], "expression");
    assert_eq!(definitions[3]["value"], "time+1");
    assert_eq!(definitions[4]["kind"], "string");
    assert_eq!(definitions[4]["value"], "local");
    assert_eq!(definitions[5]["kind"], "complex");
    assert_eq!(
        definitions[5]["value"],
        serde_json::json!({"real":2.0,"imaginary":3.0})
    );
    assert_eq!(
        json["params"],
        serde_json::json!([{"name":"GAIN", "value":2.0}])
    );
    let text = inspect(deck, &["--params", "--spice-dialect", "xyce"]);
    assert!(text.contains("Parameters (6):"), "{text}");
    assert!(text.contains("LABEL = \"run A\""), "{text}");
    assert!(text.contains("Z = 2 +3j"), "{text}");
    assert!(text.contains("LIVE = {time+1}"), "{text}");
    let plain: serde_json::Value =
        serde_json::from_str(&inspect(deck, &["--json", "--spice-dialect", "xyce"])).unwrap();
    assert!(plain["parameter_definitions"].is_null());
}

#[test]
fn model_definitions_include_typed_parameters_and_preserve_summary_fields() {
    let deck = "Model inspection\n.model diode D(IS=2p N=1.2)\n.model curve pwl(x_array=[0 1] y_array=[0 2])\n.model event d_source(input_file=\"events.txt\")\nD1 in 0 diode\n.end\n";
    let json: serde_json::Value =
        serde_json::from_str(&inspect(deck, &["--models", "--json"])).unwrap();
    assert_eq!(json["schema"], "rspice.info");
    assert_eq!(json["schema_version"], 1);
    assert_eq!(
        json["models"],
        serde_json::json!(["DIODE", "CURVE", "EVENT"])
    );
    let models = json["model_definitions"].as_array().unwrap();
    assert_eq!(models[0]["model_type"], "D");
    let parameter = |model: usize, name: &str| {
        models[model]["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["name"].as_str().unwrap().eq_ignore_ascii_case(name))
            .unwrap()
    };
    assert_eq!(parameter(0, "IS")["kind"], "real");
    assert_eq!(parameter(0, "IS")["value"], 2e-12);
    assert_eq!(parameter(1, "x_array")["kind"], "real_vector");
    assert_eq!(
        parameter(1, "x_array")["value"],
        serde_json::json!([0.0, 1.0])
    );
    assert_eq!(parameter(2, "input_file")["kind"], "string");
    assert!(
        std::path::Path::new(parameter(2, "input_file")["value"].as_str().unwrap())
            .ends_with("events.txt")
    );
    let text = inspect(deck, &["--models"]);
    assert!(
        text.to_lowercase().contains("is = 0.000000000002"),
        "{text}"
    );
    assert!(text.contains("events.txt"), "{text}");
    let summary: serde_json::Value = serde_json::from_str(&inspect(deck, &["--json"])).unwrap();
    assert!(summary["model_definitions"].is_null());
}

#[test]
fn hierarchy_retains_nested_scopes_repeated_instances_and_recursive_references() {
    let deck = "Hierarchy inspection\n.subckt outer p n params: r=1k\n.param twice={2*r}\n.subckt inner a b\nR1 a b 2k\n.ends inner\nXlocal p n inner\nXrecursive p n outer r={twice}\n.ends outer\n.subckt inner a b\nRglobal a b 3k\n.ends inner\nXone in 0 outer r=4k\nXtwo out 0 outer\n.end\n";
    let json: serde_json::Value =
        serde_json::from_str(&inspect(deck, &["--hierarchy", "--json"])).unwrap();
    assert_eq!(
        json["subcircuits"],
        serde_json::json!(["outer.inner", "outer", "inner"])
    );
    let hierarchy = &json["hierarchy"];
    assert_eq!(hierarchy["instances"].as_array().unwrap().len(), 2);
    assert_eq!(hierarchy["instances"][0]["name"], "Xone");
    assert_eq!(hierarchy["instances"][0]["subcircuit"], "outer");
    assert_eq!(
        hierarchy["instances"][0]["nodes"],
        serde_json::json!(["IN", "0"])
    );
    assert_eq!(hierarchy["instances"][0]["parameters"][0]["value"], 4000.0);
    assert_eq!(hierarchy["definitions"].as_array().unwrap().len(), 2);
    let outer = &hierarchy["definitions"][0];
    assert_eq!(outer["ports"], serde_json::json!(["P", "N"]));
    assert_eq!(outer["element_count"], 2);
    assert_eq!(outer["definitions"][0]["name"], "outer.inner");
    assert_eq!(outer["instances"][1]["subcircuit"], "outer");
    assert_eq!(outer["instances"][1]["parameters"][0]["kind"], "expression");
    assert_eq!(outer["parameters"][0]["value"], 1000.0);
    assert_eq!(outer["body_parameters"][0]["kind"], "expression");
    let text = inspect(deck, &["--hierarchy"]);
    assert!(text.contains("Xone -> outer (IN 0)"), "{text}");
    assert!(text.contains("    .subckt outer.inner"), "{text}");
    assert!(text.contains("Xrecursive -> outer"), "{text}");
}

#[test]
fn info_reports_closed_stdout_as_io_failure_in_both_formats() {
    for json in [false, true] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
        command.args(["--quiet", "--error-format", "json", "info", "-"]);
        if json {
            command.arg("--json");
        }
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        // Close the reader before delivering the deck, so the command cannot
        // race ahead and write successfully before its pipe closes.
        drop(child.stdout.take());
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"Closed pipe\nV1 in 0 1\nR1 in 0 1k\n.end\n")
            .unwrap();
        let result = child.wait_with_output().unwrap();
        assert_eq!(result.status.code(), Some(74), "json={json}: {result:?}");
        let diagnostic: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
        assert_eq!(diagnostic["error"]["category"], "io");
    }
}
