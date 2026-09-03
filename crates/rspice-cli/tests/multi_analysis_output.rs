//! With several analysis cards in one deck, `-o` must produce one file per
//! analysis instead of silently overwriting the same path.

mod common;

use common::test_dir;

use std::process::Command;

#[test]
fn each_analysis_gets_its_own_output_file() {
    let dir = test_dir("multi");
    let deck = dir.join("multi.sp");
    std::fs::write(
        &deck,
        "* op + transient in one deck\n\
         v1 in 0 sin(0 1 1k)\n\
         r1 in out 1k\n\
         c1 out 0 1u\n\
         .op\n\
         .tran 10u 100u\n\
         .end\n",
    )
    .expect("write deck");

    let out = dir.join("results.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice");
    assert!(
        output.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Each artifact is namespaced by the canonical analysis identity the
    // planner minted for its card, so a second `.OP` would be `op-002` rather
    // than overwriting the first.
    let op = dir.join("results.op-001.csv");
    let tran = dir.join("results.tran-001.csv");
    assert!(
        op.exists(),
        "missing per-analysis OP output {}",
        op.display()
    );
    assert!(
        tran.exists(),
        "missing per-analysis transient output {}",
        tran.display()
    );
    assert!(
        !out.exists(),
        "untagged output should not exist when several analyses run"
    );

    let op_text = std::fs::read_to_string(&op).expect("read op csv");
    assert!(op_text.starts_with("signal,"), "op file has op schema");
    let tran_text = std::fs::read_to_string(&tran).expect("read tran csv");
    assert!(
        tran_text.starts_with("time,"),
        "tran file has waveform schema"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn single_analysis_keeps_exact_output_path() {
    let dir = test_dir("single");
    let deck = dir.join("single.sp");
    std::fs::write(
        &deck,
        "* single analysis\nv1 in 0 dc 5\nr1 in out 1k\nr2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");

    let out = dir.join("exact.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice");
    assert!(output.status.success());
    assert!(
        out.exists(),
        "single-analysis output must keep the exact path"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// A scalar deck takes its artifact namespaces from the canonical plan, so
/// repeated cards are `ac-001`/`ac-002` for the same reason a `.STEP`
/// coordinate's are — one identity scheme, not two.
#[test]
fn repeated_cards_use_the_canonical_plan_identity_without_axes() {
    let dir = test_dir("repeated");
    let deck = dir.join("repeated.sp");
    std::fs::write(
        &deck,
        "* repeated AC cards with no run axis\n\
         v1 in 0 dc 0 ac 1\n\
         r1 in out 1k\n\
         c1 out 0 1u\n\
         .ac lin 3 1 10\n\
         .tran 10u 50u\n\
         .ac dec 3 100 1000\n\
         .end\n",
    )
    .expect("write deck");

    let out = dir.join("results.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice");
    assert!(
        output.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let source = std::fs::read_to_string(&deck).expect("read planned deck");
    let netlist = rspice_core::Netlist::parse(&source).expect("parse planned deck");
    let plan = rspice_core::execution::DeckPlan::from_netlist_with_abort(
        &netlist,
        &rspice_core::ResourceLimits::default(),
        &rspice_core::NoAbort,
    )
    .expect("plan the scalar deck");
    let planned = plan
        .analyses()
        .iter()
        .map(|analysis| dir.join(format!("results.{}.csv", analysis.id().tag())))
        .collect::<Vec<_>>();
    assert_eq!(planned.len(), 3);

    let mut actual = std::fs::read_dir(&dir)
        .expect("list scalar outputs")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "csv"))
        .collect::<Vec<_>>();
    actual.sort();
    let mut expected = planned;
    expected.sort();
    assert_eq!(
        actual, expected,
        "scalar artifacts must be namespaced by the planner's analysis identities"
    );

    // The two `.AC` cards are distinct sweeps, so their artifacts must differ.
    let first = std::fs::read_to_string(dir.join("results.ac-001.csv")).expect("read ac-001");
    let second = std::fs::read_to_string(dir.join("results.ac-002.csv")).expect("read ac-002");
    assert_ne!(first, second);

    let _ = std::fs::remove_dir_all(&dir);
}
