mod common;
use std::process::Command;

#[test]
fn explicit_temperature_wins_in_scalar_control_corner_and_parameter_sweep_runs() {
    for (name, analysis, extra) in [
        ("scalar", ".op\n", vec![]),
        ("control", ".control\nop\n.endc\n", vec![]),
        ("corner", ".op\n", vec!["--corners", "tt,ss"]),
        ("step", ".step param r list 1k 2k\n.op\n", vec![]),
        ("implicit", ".step param r list 1k 2k\n", vec![]),
    ] {
        let dir = common::test_dir(name);
        let deck = dir.join("deck.cir");
        std::fs::write(&deck, format!("temperature override\n.param r=1k\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k TC1=0.01\n.options temp=125\n{analysis}.end\n")).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--temp", "27", "-f", "csv", "-o"])
            .arg(dir.join("result.csv"))
            .args(extra)
            .output()
            .unwrap();
        assert!(output.status.success(), "{name}: {output:?}");
        let mut checked = 0;
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if !path.extension().is_some_and(|ext| ext == "csv") {
                continue;
            }
            let text = std::fs::read_to_string(path).unwrap();
            if name == "implicit" {
                let index = text
                    .lines()
                    .next()
                    .unwrap()
                    .split(',')
                    .position(|col| col.eq_ignore_ascii_case("V(OUT)"))
                    .unwrap();
                for row in text.lines().skip(1) {
                    let value: f64 = row.split(',').nth(index).unwrap().parse().unwrap();
                    assert!((value - 0.5).abs() < 1e-9, "{name}: {text}");
                    checked += 1;
                }
            } else {
                let row = text
                    .lines()
                    .find(|row| row.to_ascii_lowercase().starts_with("v(out),"))
                    .unwrap();
                let value: f64 = row.split_once(',').unwrap().1.parse().unwrap();
                assert!((value - 0.5).abs() < 1e-9, "{name}: {text}");
                checked += 1;
            }
        }
        assert!(checked > 0, "no results for {name}");
    }
}

#[test]
fn temperature_override_cannot_mislabel_an_authored_temperature_axis() {
    let dir = common::test_dir("temperature-conflict");
    let deck = dir.join("deck.cir");
    for axis in [".temp -40 125", ".step temp list -40 125"] {
        std::fs::write(
            &deck,
            format!("temperature axis\nV1 n 0 1\nR1 n 0 1k\n{axis}\n.op\n.end\n"),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "run"])
            .arg(&deck)
            .args(["--temp", "27"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(String::from_utf8_lossy(&output.stderr).contains("conflicts"));
    }
}
