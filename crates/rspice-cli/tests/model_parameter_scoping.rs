//! Deferred model cards must use their own parameter context through execution.
mod common;

use std::process::Command;

#[test]
fn unrelated_model_order_cannot_change_the_operating_point() {
    for models in [
        ".MODEL unused R(R={later})\n.MODEL active R(RSH={R})",
        ".MODEL active R(RSH={R})\n.MODEL unused R(R={later})",
    ] {
        for parameters_first in [false, true] {
            let directory = common::test_dir("model_scope");
            let deck = directory.join("deck.cir");
            let result = directory.join("result.csv");
            let parameters = ".PARAM R=200 later=100";
            let declarations = if parameters_first {
                format!("{parameters}\n{models}")
            } else {
                format!("{models}\n{parameters}")
            };
            std::fs::write(
                &deck,
                format!("* model scopes\n{declarations}\nI1 0 out 1\nR1 out 0 active L=1 W=1\n.OP\n.END\n"),
            )
            .unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["--format", "csv", "--output"])
                .arg(&result)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let csv = std::fs::read_to_string(result).unwrap();
            let mut lines = csv.lines();
            assert_eq!(lines.next(), Some("signal,value"));
            let voltage: f64 = lines
                .filter_map(|line| line.split_once(','))
                .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
                .unwrap_or_else(|| panic!("{csv}"))
                .1
                .parse()
                .unwrap();
            assert!((voltage - 200.0).abs() < 1e-6, "{declarations}: {csv}");
        }
    }
}
