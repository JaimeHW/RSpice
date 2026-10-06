mod common;
use std::process::Command;

#[test]
fn conditional_step_inputs_are_reserved_before_any_run_publishes() {
    let dir = common::test_dir("step_external_source_ownership");
    let deck = dir.join("deck.cir");
    let later = dir.join("later.dat");
    std::fs::write(&later, "0 2\n1 2\n").unwrap();
    std::fs::write(dir.join("first.dat"), "0 1\n1 1\n").unwrap();
    std::fs::write(&deck, "conditional\n.param choice=0\n.step param choice list 0 1\n.if (choice == 0)\nV1 in 0 PWL FILE=\"first.dat\"\n.else\nV1 in 0 PWL FILE=\"later.dat\"\n.endif\nR1 in 0 1k\n.op\n.end\n").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["-f", "csv", "-o"])
        .arg(&later)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("source"),
        "{result:?}"
    );
    assert_eq!(std::fs::read_to_string(&later).unwrap(), "0 2\n1 2\n");
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 3);
}

#[test]
fn source_discovery_does_not_compile_models_for_non_simulating_control_scripts() {
    let dir = common::test_dir("control_source_discovery");
    let deck = dir.join("deck.cir");
    std::fs::write(
        dir.join("unused.va"),
        "this is not a valid Verilog-A module\n",
    )
    .unwrap();
    std::fs::write(
        &deck,
        "control\n.va unused.va\nV1 in 0 1\nR1 in 0 1k\n.control\noption reltol=1e-5\n.endc\n.end\n",
    )
    .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
}

#[test]
fn ordinary_outputs_can_reuse_paths_named_only_in_inactive_verilog_branches() {
    let dir = common::test_dir("inactive_source_discovery");
    let deck = dir.join("deck.cir");
    let output = dir.join("result.csv");
    std::fs::write(&output, "previous output\n").unwrap();
    std::fs::write(dir.join("source.dat"), "0 1\n1 1\n").unwrap();
    std::fs::write(dir.join("resistor.va"), "`include \"disciplines.vams\"\n`ifdef UNUSED_OUTPUT\n`include \"result.csv\"\n`endif\nmodule resistor(p,n);\ninout p,n; electrical p,n;\nanalog I(p,n) <+ V(p,n)/1000;\nendmodule\n").unwrap();
    std::fs::write(
        &deck,
        "ordinary\n.va resistor.va\nV1 in 0 PWL FILE=\"source.dat\"\nX1 in 0 resistor\n.op\n.end\n",
    )
    .unwrap();
    for _ in 0..2 {
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["-f", "csv", "-o"])
            .arg(&output)
            .output()
            .unwrap();
        assert!(result.status.success(), "{result:?}");
        let csv = std::fs::read_to_string(&output).unwrap();
        assert!(csv.to_ascii_lowercase().contains("in"), "{csv}");
        assert!(csv.lines().count() >= 2, "{csv}");
    }
    assert_eq!(
        std::fs::read_to_string(dir.join("source.dat")).unwrap(),
        "0 1\n1 1\n"
    );
}
