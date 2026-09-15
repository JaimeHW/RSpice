//! End-to-end control execution through the ordinary public `run` command.

mod common;

use common::test_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn run(deck: &Path, output: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(output)
        .args(["-f", "csv", "--convergence", "robust"])
        .args(extra)
        .output()
        .expect("execute public CLI")
}

fn passed(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn document(path: &Path) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(path).expect("published control data")).unwrap()
}

#[test]
fn original_bjt_control_loop_publishes_six_datasets_and_its_plot() {
    let dir = test_dir("control-bjt");
    let deck = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/paranoia/control_structs/foreach_bjt_ft.sp");
    passed(&run(
        &deck,
        &dir.join("result.csv"),
        &[
            "--reltol",
            "1e-10",
            "--abstol",
            "1e-18",
            "--voltage-abstol",
            "1e-12",
            "--gmin",
            "0",
        ],
    ));
    for ordinal in 1..=6 {
        let data =
            std::fs::read_to_string(dir.join(format!("result.ac-{ordinal:03}.csv"))).unwrap();
        assert!(data.starts_with("frequency,"));
        assert_eq!(data.lines().count(), 58);
    }
    let plot = document(&dir.join("result.control-001.json"));
    assert_eq!(plot["schema"], "rspice.control-presentation");
    assert_eq!(plot["traces"].as_array().unwrap().len(), 6);
    assert_eq!(plot["x_logarithmic"], true);
    assert_eq!(plot["y_limits"], serde_json::json!([0.1, 100.0]));
    for (index, trace) in plot["traces"].as_array().unwrap().iter().enumerate() {
        assert_eq!(trace["y"]["dataset"], format!("ac{}", index + 1));
        assert_eq!(trace["y"]["samples"].as_array().unwrap().len(), 57);
        assert_eq!(trace["y"]["unit"], "A");
    }
    let svg = std::fs::read_to_string(dir.join("result.control-001.svg")).unwrap();
    assert!(
        svg.contains("<svg")
            && svg.contains("ac1.vgain#branch")
            && svg.contains("ac6.vgain#branch")
    );
}

#[test]
fn original_memristor_control_script_publishes_each_time_grid_and_xy_plot() {
    let dir = test_dir("control-memristor");
    let deck = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/paranoia/memristor/memristor.sp");
    passed(&run(&deck, &dir.join("result.csv"), &[]));
    for ordinal in 1..=3 {
        assert!(dir.join(format!("result.tran-{ordinal:03}.csv")).is_file());
    }
    for ordinal in [1, 3, 4, 5] {
        assert!(
            dir.join(format!("result.control-{ordinal:03}.svg"))
                .is_file()
        );
    }
    let resistance = document(&dir.join("result.control-003.json"));
    let current = document(&dir.join("result.control-005.json"));
    for (index, frequency) in [1.4e8, 1e8, 1.1e8].iter().enumerate() {
        let trace = &resistance["traces"][index];
        assert_eq!(trace["y"]["unit"], "ohm");
        let last = trace["x"]["samples"].as_array().unwrap().last().unwrap()[0]
            .as_f64()
            .unwrap();
        assert!((last * frequency - 1.0).abs() < 1e-12);
        let trace = &current["traces"][index];
        assert_eq!(trace["x"]["dataset"], trace["y"]["dataset"]);
        assert_eq!(trace["x"]["unit"], "V");
        assert_eq!(
            trace["y"]["current_sources"][0]["owner"]["branchName"],
            "v1"
        );
        assert!(trace["y"]["current_sources"][0].get("impulses").is_some());
    }
}

#[test]
fn included_control_commands_keep_source_locations_and_publish_atomically() {
    let dir = test_dir("control-include");
    let search = dir.join("models");
    std::fs::create_dir(&search).unwrap();
    let deck = dir.join("root.sp");
    let child = search.join("commands.inc");
    std::fs::write(
        &deck,
        "included control\nV1 out 0 0\nR1 out 0 1k\n.include commands.inc\n.end\n",
    )
    .unwrap();
    let script =
        ".control\nforeach bias 1 2\nalter v1 $bias\nop\nend\nprint op1.v(out) op2.v(out)\n.endc\n";
    std::fs::write(&child, script).unwrap();
    let success = run(
        &deck,
        &dir.join("first.csv"),
        &["-I", search.to_str().unwrap()],
    );
    passed(&success);
    let stdout = String::from_utf8(success.stdout).unwrap();
    assert!(stdout.contains("1.00000000000000000e0") && stdout.contains("2.00000000000000000e0"));
    assert!(dir.join("first.op-001.csv").is_file() && dir.join("first.op-002.csv").is_file());

    std::fs::write(
        &child,
        script.replace("print op1.v(out) op2.v(out)", "plot v(missing)"),
    )
    .unwrap();
    let existing = dir.join("failed.op-001.csv");
    std::fs::write(&existing, "previous result\n").unwrap();
    let failed = run(
        &deck,
        &dir.join("failed.csv"),
        &["-I", search.to_str().unwrap()],
    );
    assert!(!failed.status.success());
    let diagnostic = String::from_utf8_lossy(&failed.stderr);
    assert!(diagnostic.contains("commands.inc:6"), "{diagnostic}");
    assert_eq!(
        std::fs::read_to_string(existing).unwrap(),
        "previous result\n"
    );
    assert!(!dir.join("failed.op-002.csv").exists());
}
