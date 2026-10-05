//! Distinct outer runs must never share a result or restart destination.
mod common;

use common::test_dir;
use std::collections::HashSet;
use std::process::Command;

#[test]
fn colliding_labels_keep_every_result_and_checkpoint_in_serial_and_parallel_runs() {
    for jobs in ["1", "2"] {
        let dir = test_dir("alter_identity");
        let deck = dir.join("deck.cir");
        std::fs::write(&deck, "Alter artifacts\nV1 in 0 10\nR1 in out 1k\nR2 out 0 1k\n.tran 1u 5u\n.alter hot-fast\nR2 out 0 2k\n.alter hot_fast\nR2 out 0 3k\n.alter HOT_FAST\nR2 out 0 4k\n.alter base\nR2 out 0 5k\n.alter hot_fast_run_2\nR2 out 0 6k\n.end\n").unwrap();
        let summary = dir.join("summary.json");
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(deck)
            .args(["-f", "csv", "-o"])
            .arg(dir.join("result.csv"))
            .arg("--checkpoint")
            .arg(dir.join("state.chk"))
            .arg("--summary")
            .arg(&summary)
            .args(["--jobs", jobs])
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(summary).unwrap()).unwrap();
        assert_eq!(json["counts"]["passed_runs"], 6);
        let outputs = json["outputs"].as_array().unwrap();
        assert_eq!(outputs.len(), 6);
        let unique: HashSet<_> = outputs
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect();
        assert_eq!(unique.len(), 6);
        let mut voltages: Vec<_> = unique
            .iter()
            .map(|path| {
                let content = std::fs::read_to_string(path).unwrap();
                let mut lines = content.lines();
                let column = lines
                    .next()
                    .unwrap()
                    .split(',')
                    .position(|name| name.eq_ignore_ascii_case("V(OUT)"))
                    .unwrap();
                lines
                    .last()
                    .unwrap()
                    .split(',')
                    .nth(column)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()
            })
            .collect();
        voltages.sort_by(f64::total_cmp);
        for (index, voltage) in voltages.iter().enumerate() {
            let ratio = (index + 1) as f64;
            assert!((voltage - 10.0 * ratio / (ratio + 1.0)).abs() < 1e-8);
        }
        let checkpoints = std::fs::read_dir(&*dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "chk"))
            .count();
        assert_eq!(checkpoints, 6);
    }
}
