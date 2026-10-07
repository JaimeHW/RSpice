//! Temperature-sensitive model values must survive parsing and hierarchy binding.
mod common;

use std::process::Command;

#[test]
fn equivalent_temperature_expressions_produce_the_same_resistance() {
    for (body, expected) in [
        (".MODEL rm R(RSH=TEMP)\nR1 out 0 rm L=1 W=1 TEMP=47", 47.0),
        (".MODEL rm R(RSH=+ TEMP)\nR1 out 0 rm L=1 W=1 TEMP=47", 47.0),
        (
            ".MODEL rm R(RSH=- TEMP)\nR1 out 0 rm L=1 W=1 TEMP=-10",
            10.0,
        ),
        (".MODEL rm R(RSH=TNOM TNOM=47)\nR1 out 0 rm L=1 W=1", 47.0),
        (
            ".FUNC thermal() {TEMP}\n.MODEL rm R(RSH={thermal()})\nR1 out 0 rm L=1 W=1 TEMP=47",
            47.0,
        ),
        (
            ".MODEL rm R(RSH={thermal()})\n.FUNC thermal() {TEMP}\nR1 out 0 rm L=1 W=1 TEMP=47",
            47.0,
        ),
        (
            ".SUBCKT cell a\n.MODEL rm R(RSH={TEMP})\nR1 a 0 rm L=1 W=1 TEMP=47\n.ENDS\nX1 out cell",
            47.0,
        ),
        (
            ".MODEL rm R(RSH={TEMP})\n.SUBCKT cell a\nR1 a 0 rm L=1 W=1 TEMP=47\n.ENDS\nX1 out cell",
            47.0,
        ),
        (
            ".SUBCKT cell a PARAMS: scale=2\n.FUNC thermal() {TEMP*scale}\n.MODEL rm R(RSH={thermal()})\nR1 a 0 rm L=1 W=1 TEMP=47\n.ENDS\nX1 out cell scale=3",
            141.0,
        ),
    ] {
        let directory = common::test_dir("model_temperature_dependency");
        let deck = directory.join("deck.cir");
        let result = directory.join("result.csv");
        std::fs::write(
            &deck,
            format!("* model temperature dependency\nV1 out 0 1\n{body}\n.OP\n.END\n"),
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
            "{body}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let csv = std::fs::read_to_string(result).unwrap();
        let current: f64 = csv
            .lines()
            .filter_map(|line| line.split_once(','))
            .find(|(name, _)| name.eq_ignore_ascii_case("I(V1)"))
            .unwrap_or_else(|| panic!("{csv}"))
            .1
            .parse()
            .unwrap();
        let actual = -1.0 / current;
        assert!(
            (actual - expected).abs() < expected * 1e-9,
            "{body}: expected {expected}, got {actual}"
        );
    }
}
