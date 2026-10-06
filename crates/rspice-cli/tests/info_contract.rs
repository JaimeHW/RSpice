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
