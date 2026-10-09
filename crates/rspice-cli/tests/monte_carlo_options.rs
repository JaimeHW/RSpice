//! CLI overrides must preserve authored studies and be consumed by every run.
mod common;

use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Output};

const STUDY: &str = "Seed override\n.param rtop=1k supply=5\nV1 in 0 {supply}\nR1 in out {rtop}\nR2 out 0 1k\n.mc 4 uniform .15 START 3 SEED 11 CONFIDENCE 90 CI BOOTSTRAP RESAMPLES 64 BOOTSEED 9 PARAMS rtop\n.end\n";

fn run(deck: &Path, output: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(deck)
        .args(["-f", "json", "-o"])
        .arg(output)
        .args(extra)
        .output()
        .unwrap()
}

fn scalar<'a>(document: &'a Value, name: &str) -> &'a Value {
    &document["scalars"]
        .as_array()
        .unwrap()
        .iter()
        .find(|scalar| scalar["name"] == name)
        .unwrap()["value"]["value"]
}

fn documents(directory: &Path, stem: &str) -> BTreeMap<String, Value> {
    std::fs::read_dir(directory)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_str().unwrap();
            let suffix = name.strip_prefix(stem)?.strip_suffix(".json")?;
            let document = common::read_json(&path);
            document["payload"]["statistics"]
                .is_array()
                .then(|| (suffix.to_string(), document))
        })
        .collect()
}

#[test]
fn seed_override_preserves_the_authored_monte_carlo_study() {
    let dir = common::test_dir("mc_authored_seed");
    let authored = dir.join("authored.sp");
    std::fs::write(&authored, STUDY).unwrap();
    for seed in ["0", "17", "18446744073709551615"] {
        let reference = dir.join("reference.sp");
        std::fs::write(
            &reference,
            STUDY.replace("SEED 11", &format!("SEED {seed}")),
        )
        .unwrap();
        let expected_path = dir.join("expected.json");
        let actual_path = dir.join("actual.json");
        let expected = run(&reference, &expected_path, &[]);
        assert!(expected.status.success(), "{expected:?}");
        let actual = run(&authored, &actual_path, &["--seed", seed]);
        assert!(actual.status.success(), "{seed}: {actual:?}");
        let expected = common::read_json(&expected_path);
        let actual = common::read_json(&actual_path);
        assert_eq!(actual["payload"], expected["payload"]);
        assert_eq!(actual["scalars"], expected["scalars"]);
        assert_eq!(
            scalar(&actual, "sampling_seed"),
            &json!(seed.parse::<u64>().unwrap())
        );
        assert_eq!(scalar(&actual, "completed_runs"), &json!(4));
        assert_eq!(scalar(&actual, "first_trial"), &json!(3));
        assert_eq!(scalar(&actual, "mean_confidence_level_pct"), &json!(90.0));
        assert_eq!(
            scalar(&actual, "mean_confidence_bootstrap_resamples"),
            &json!(64)
        );
        assert_eq!(
            actual["payload"]["successfulTrialIndices"],
            json!([3, 4, 5, 6])
        );
    }
    // An explicit mode still replaces the authored study as documented.
    std::fs::write(
        &authored,
        STUDY.replace(".end\n", ".control\nop\n.endc\n.end\n"),
    )
    .unwrap();
    let destination = dir.join("mode.json");
    let output = run(
        &authored,
        &destination,
        &["--monte-carlo", "2", "--seed", "17"],
    );
    assert!(output.status.success(), "{output:?}");
    let mut results = documents(&dir, "mode");
    assert_eq!(results.len(), 1);
    let (_, actual) = results.pop_first().unwrap();
    assert_eq!(scalar(&actual, "sampling_seed"), &json!(17));
    assert_eq!(scalar(&actual, "completed_runs"), &json!(2));
    assert_eq!(actual["payload"]["successfulTrialIndices"], json!([0, 1]));
}

#[test]
fn seed_overrides_replay_authored_seeds_across_axes_alters_and_corners() {
    for (case, tail, flags) in [
        ("step", ".step param supply list 4 5\n", vec![]),
        ("temperature", ".temp -10 40\n", vec![]),
        (
            "alter",
            ".alter second\n.param rtop=1.2k\n",
            vec!["--jobs", "2"],
        ),
        ("corners", "", vec!["--corners", "tt,ss", "--jobs", "2"]),
    ] {
        let dir = common::test_dir(case);
        let source = STUDY.replace(".end\n", &format!("{tail}.end\n"));
        let authored = dir.join("authored.sp");
        let reference = dir.join("reference.sp");
        std::fs::write(&authored, &source).unwrap();
        std::fs::write(&reference, source.replace("SEED 11", "SEED 17")).unwrap();
        let expected = run(&reference, &dir.join("expected.json"), &flags);
        assert!(expected.status.success(), "{case}: {expected:?}");
        let mut overrides = flags;
        overrides.extend(["--seed", "17"]);
        let actual = run(&authored, &dir.join("actual.json"), &overrides);
        assert!(actual.status.success(), "{case}: {actual:?}");
        let expected = documents(&dir, "expected");
        let actual = documents(&dir, "actual");
        assert_eq!(expected.len(), 2, "{case}: {expected:?}");
        assert_eq!(
            actual.keys().collect::<Vec<_>>(),
            expected.keys().collect::<Vec<_>>()
        );
        for (identity, document) in actual {
            assert_eq!(
                document["payload"], expected[&identity]["payload"],
                "{case}, {identity}"
            );
            assert_eq!(
                document["scalars"], expected[&identity]["scalars"],
                "{case}, {identity}"
            );
        }
    }
}

#[test]
fn unused_seeds_are_rejected_before_artifact_publication() {
    let conditional_study = STUDY
        .replace("supply=5", "supply=5 run_mc=1")
        .replace(".mc 4", ".if run_mc\n.mc 4")
        .replace(
            ".end\n",
            ".else\n.op\n.endif\n.alter no_mc\n.param run_mc=0\n.end\n",
        );
    let conditional_control = STUDY.replace("supply=5", "supply=5 mode=0").replace(
        ".end\n",
        ".if (mode==1)\n.control\nop\n.endc\n.endif\n.step param mode list 0 1\n.end\n",
    );
    for (case, source, extra) in [
        ("op", STUDY.replace(".mc 4 uniform .15 START 3 SEED 11 CONFIDENCE 90 CI BOOTSTRAP RESAMPLES 64 BOOTSEED 9 PARAMS rtop", ".op"), vec![]),
        ("control", STUDY.replace(".end\n", ".control\nop\n.endc\n.end\n"), vec![]),
        ("overridden", STUDY.to_string(), vec!["--sens-output", "out", "--sens-param", "rtop"]),
        ("later-alter", conditional_study, vec!["--jobs", "2"]),
        ("later-step-control", conditional_control, vec![]),
    ] {
        let dir = common::test_dir(case);
        let deck = dir.join("source.sp");
        std::fs::write(&deck, source).unwrap();
        let destination = dir.join("result.json");
        let summary = dir.join("summary.json");
        std::fs::write(&destination, "previous result").unwrap();
        std::fs::write(&summary, "previous summary").unwrap();
        let mut flags = extra;
        flags.extend(["--seed", "17", "--summary", summary.to_str().unwrap()]);
        let output = run(&deck, &destination, &flags);
        assert_eq!(output.status.code(), Some(2), "{case}: {output:?}");
        let error: Value = serde_json::from_slice(&output.stderr).unwrap();
        let message = error["error"]["message"].as_str().unwrap();
        assert!(message.contains("--seed") && message.contains("executed Monte Carlo"), "{case}: {error}");
        assert_eq!(std::fs::read_to_string(destination).unwrap(), "previous result");
        assert_eq!(std::fs::read_to_string(summary).unwrap(), "previous summary");
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 3, "{case}: unexpected publication");
    }
}
