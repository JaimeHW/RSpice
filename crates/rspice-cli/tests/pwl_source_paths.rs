mod common;
use std::process::Command;

#[test]
fn pwl_sources_use_the_deck_directory_in_wrappers_and_nested_subcircuits() {
    let dir = common::test_dir("pwl_path_base");
    let decks = dir.join("deck directory");
    let working = dir.join("working");
    std::fs::create_dir(&decks).unwrap();
    std::fs::create_dir(&working).unwrap();
    std::fs::write(decks.join("wave.dat"), "0 1\n1 1\n").unwrap();
    // A successful solve of the wrong file is more dangerous than a missing
    // file error. Make both locations readable, with unmistakable answers.
    std::fs::write(working.join("wave.dat"), "0 9\n1 9\n").unwrap();
    let cases = [
        ("plain", "V1 in 0 PWL FILE=\"wave.dat\"", 1.0),
        ("distortion", "V1 in 0 PWL FILE=\"wave.dat\" DISTOF1 1", 1.0),
        (
            "hierarchy",
            ".subckt driver p\nV1 p 0 PWL FILE=wave.dat\n.ends\nX1 in driver",
            1.0,
        ),
        (
            "nested",
            ".subckt outer p\n.subckt inner q params: gain=2\nV1 q 0 AC 1 PWL(FILE='wave.dat' VSCALE={gain}) DISTOF1 1\n.ends inner\nX1 p inner\n.ends outer\nX1 in outer",
            2.0,
        ),
    ];
    for (name, body, expected) in cases {
        let deck = decks.join(format!("{name}.cir"));
        let output = dir.join(format!("{name}.csv"));
        std::fs::write(&deck, format!("paths\n{body}\nR1 in 0 1k\n.op\n.end\n")).unwrap();
        for cwd in [&working, &decks] {
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["-f", "csv", "-o"])
                .arg(&output)
                .current_dir(cwd)
                .output()
                .unwrap();
            assert!(result.status.success(), "{name}: {result:?}");
            let csv = std::fs::read_to_string(&output).unwrap();
            let voltage = csv
                .lines()
                .find_map(|line| line.strip_prefix("V(IN),"))
                .unwrap()
                .parse::<f64>()
                .unwrap();
            assert!(
                (voltage - expected).abs() < 1e-10,
                "{name} from {}: {csv}",
                cwd.display()
            );
        }
    }
}

#[test]
fn nested_pwl_data_remains_protected_from_output_outside_the_deck_directory() {
    let dir = common::test_dir("nested_pwl_source_protection");
    let deck = dir.join("deck.cir");
    let data = dir.join("wave.dat");
    let working = dir.join("elsewhere");
    std::fs::create_dir(&working).unwrap();
    std::fs::write(&data, "0 1\n1 1\n").unwrap();
    std::fs::write(working.join("wave.dat"), "0 9\n1 9\n").unwrap();
    std::fs::write(&deck, "nested\n.subckt driver p\nV1 p 0 PWL FILE=wave.dat\n.ends\nX1 in driver\nR1 in 0 1k\n.op\n.end\n").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["-f", "csv", "-o"])
        .arg(&data)
        .current_dir(working)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("source"),
        "{result:?}"
    );
    assert_eq!(std::fs::read_to_string(&data).unwrap(), "0 1\n1 1\n");
}
