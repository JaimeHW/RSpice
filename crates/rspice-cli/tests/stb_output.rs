//! `.STB` end to end through the CLI: the card runs, the margins print,
//! and the export carries the loop-gain sweep with magnitude and phase.

mod common;

use common::test_dir;

use std::process::Command;

/// Single-pole inverting loop: T(f) = 1000/(1 + j f/1kHz). DC gain 60 dB,
/// unity crossover at ~1 MHz, phase margin ~90.06 degrees.
const SINGLE_POLE_STB: &str = "* single-pole loop with .stb card
e1 eo 0 ctrl 0 -1000
vprobe eo x 0
r1 x ctrl 1k
c1 ctrl 0 159.154943091895n
.stb dec 20 10 10meg probe=vprobe
.end
";

#[test]
fn gain_margin_is_reported_without_a_unity_gain_crossover() {
    // Three buffered RC sections give T(s)=1000/(1+s/(2*pi*1000))^3.
    // At f=sqrt(3)*1000 Hz the phase is -180 degrees and T=-125,
    // while the entire authored band remains above unity.
    let dir = test_dir("stb_phase_crossover");
    let deck = dir.join("stb.sp");
    std::fs::write(
        &deck,
        "* three-pole loop\n\
E1 eo 0 n3 0 -1000\nVP eo x 0\n\
R1 x n1 1k\nC1 n1 0 159.154943091895n\n\
E2 b1 0 n1 0 1\nR2 b1 n2 1k\nC2 n2 0 159.154943091895n\n\
E3 b2 0 n2 0 1\nR3 b2 n3 1k\nC3 n3 0 159.154943091895n\n\
.stb dec 100 100 3000 probe=VP\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["run", deck.to_str().unwrap()])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{stdout}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("No unity-gain crossover"), "{stdout}");
    let line = stdout
        .lines()
        .find(|line| line.contains("Gain margin:"))
        .expect("the measured phase crossing must report a gain margin");
    let words = line.split_whitespace().collect::<Vec<_>>();
    let margin: f64 = words[2].parse().unwrap();
    let frequency: f64 = words[5].parse().unwrap();
    assert!((margin + 20.0 * 125f64.log10()).abs() < 0.02, "{line}");
    assert!(
        (frequency / (3f64.sqrt() * 1000.0) - 1.0).abs() < 0.001,
        "{line}"
    );
}

#[test]
fn stb_card_runs_and_exports_loop_gain() {
    let dir = test_dir("stb");
    let deck = dir.join("stb_loop.sp");
    std::fs::write(&deck, SINGLE_POLE_STB).expect("write deck");

    let out = dir.join("loop.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ])
        .output()
        .expect("rspice runs");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "exit ok\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    // Margins reach the console with the closed-form values.
    assert!(
        stdout.contains("Phase margin:"),
        "phase margin printed:\n{stdout}"
    );
    let pm_line = stdout
        .lines()
        .find(|line| line.contains("Phase margin:"))
        .expect("phase margin line");
    let pm: f64 = pm_line
        .split_whitespace()
        .nth(2)
        .expect("phase margin value")
        .parse()
        .expect("phase margin parses");
    assert!(stdout.contains("no gain margin to report"), "{stdout}");
    assert!(!stdout.contains("Gain margin: inf"), "{stdout}");
    // Closed form: 180 - atan(fu/fp) with fu = 1kHz*sqrt(1000^2-1) ~ 1 MHz.
    assert!(
        (pm - 90.057).abs() < 0.5,
        "phase margin ~90.06 deg, got {pm}"
    );

    // The export exists, is tagged stb, and carries the sweep columns.
    let exported = std::fs::read_dir(&dir)
        .expect("read test dir")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.contains("stb") && name.ends_with(".csv"))
        })
        .unwrap_or(out);
    let content = std::fs::read_to_string(&exported).expect("export readable");
    assert!(
        content.contains("loopgain_mag_db") && content.contains("loopgain_phase_deg"),
        "export carries magnitude and phase columns:\n{}",
        content.lines().take(3).collect::<Vec<_>>().join("\n")
    );

    // First sweep row sits at 10 Hz where |T| ~ 60 dB and phase ~ -0.57 deg.
    let header_idx = content
        .lines()
        .position(|line| line.contains("loopgain_mag_db"))
        .expect("header row");
    let first_data = content.lines().nth(header_idx + 1).expect("first data row");
    let fields: Vec<&str> = first_data.split(',').map(str::trim).collect();
    let header_fields: Vec<&str> = content
        .lines()
        .nth(header_idx)
        .unwrap()
        .split(',')
        .map(str::trim)
        .collect();
    let mag_col = header_fields
        .iter()
        .position(|name| *name == "loopgain_mag_db")
        .expect("mag column");
    let mag: f64 = fields[mag_col].parse().expect("magnitude parses");
    assert!(
        (mag - 60.0).abs() < 0.1,
        "DC-ish magnitude ~60 dB, got {mag}"
    );
}

#[test]
fn stb_missing_probe_source_fails_with_guidance() {
    let dir = test_dir("stb");
    let deck = dir.join("stb_bad_probe.sp");
    std::fs::write(
        &deck,
        "* probe names a source that does not exist
e1 eo 0 ctrl 0 -1000
r1 eo ctrl 1k
c1 ctrl 0 1u
.stb dec 10 10 1meg probe=vnone
.end
",
    )
    .expect("write deck");

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["run", deck.to_str().unwrap()])
        .output()
        .expect("rspice runs");

    assert!(!output.status.success(), "missing probe must fail the run");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{stdout}{stderr}");
    assert!(
        combined.contains("vnone") || combined.to_ascii_lowercase().contains("probe"),
        "error names the probe problem:\n{combined}"
    );
}

#[test]
fn stb_high_start_exports_an_independent_dc_return_ratio() {
    let dir = test_dir("stb_dc");
    let deck = dir.join("high_start.sp");
    let output_path = dir.join("result.json");
    std::fs::write(
        &deck,
        SINGLE_POLE_STB.replace("dec 20 10 10meg", "lin 3 100k 10meg"),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "run",
            deck.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "-f",
            "json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("DC loop gain: 60.00 dB"));
    let document: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output_path).unwrap()).unwrap();
    let scalar = |name: &str| {
        document["scalars"]
            .as_array()
            .unwrap()
            .iter()
            .find(|scalar| scalar["name"] == name)
            .unwrap()["value"]
            .clone()
    };
    assert!((scalar("dc_loop_gain")["value"]["real"].as_f64().unwrap() - 1000.0).abs() < 1e-8);
    assert!((scalar("dc_loop_gain_db")["value"].as_f64().unwrap() - 60.0).abs() < 1e-10);
    assert_eq!(document["pointCount"], 3);
}

#[test]
fn single_point_and_truncated_sweeps_export_unobserved_margins() {
    for points in [1, 3] {
        let dir = test_dir("stb_unobserved");
        let deck = dir.join("loop.sp");
        let output_path = dir.join("result.json");
        std::fs::write(
            &deck,
            SINGLE_POLE_STB.replace("dec 20 10 10meg", &format!("lin {points} 10 1000")),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "run",
                deck.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
                "-f",
                "json",
            ])
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success(),
            "{stdout}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(stdout.contains("No unity-gain crossover"), "{stdout}");
        assert!(stdout.contains("no gain margin to report"), "{stdout}");
        let document: serde_json::Value =
            serde_json::from_slice(&std::fs::read(output_path).unwrap()).unwrap();
        assert_eq!(document["pointCount"], points);
        for name in [
            "gain_margin_db",
            "gain_margin_frequency",
            "phase_margin_degrees",
            "phase_margin_frequency",
            "unity_gain_bandwidth",
        ] {
            let scalar = document["scalars"]
                .as_array()
                .unwrap()
                .iter()
                .find(|scalar| scalar["name"] == name)
                .unwrap();
            assert_eq!(scalar["value"]["representation"], "unavailable", "{name}");
            assert_eq!(scalar["value"]["reason"], "no_crossover", "{name}");
        }
    }
}

#[test]
fn stb_exports_core_bode_samples_without_clipping_or_rewrapping() {
    let tiny = SINGLE_POLE_STB
        .replace("-1000", "-1e-310")
        .replace("dec 20 10 10meg", "lin 1 10 1000");
    let three_pole = "* three-pole loop\n\
E1 eo 0 n3 0 -1000\nVP eo x 0\n\
R1 x n1 1k\nC1 n1 0 159.154943091895n\n\
E2 b1 0 n1 0 1\nR2 b1 n2 1k\nC2 n2 0 159.154943091895n\n\
E3 b2 0 n2 0 1\nR3 b2 n3 1k\nC3 n3 0 159.154943091895n\n\
.stb dec 20 100 100k probe=VP\n.end\n"
        .to_owned();
    for (deck_text, gain, poles) in [(tiny, 1e-310_f64, 1.0), (three_pole, 1000.0, 3.0)] {
        let dir = test_dir("stb_bode_export");
        let deck = dir.join("loop.sp");
        let output_path = dir.join("result.csv");
        std::fs::write(&deck, deck_text).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "run",
                deck.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
                "-f",
                "csv",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let content = std::fs::read_to_string(output_path).unwrap();
        let start = content
            .lines()
            .position(|line| line.contains("loopgain_mag_db"))
            .unwrap();
        let mut lines = content.lines().skip(start);
        let headers = lines
            .next()
            .unwrap()
            .split(',')
            .map(str::trim)
            .collect::<Vec<_>>();
        let column = |name| headers.iter().position(|header| *header == name).unwrap();
        let [frequency, magnitude, phase] =
            ["frequency", "loopgain_mag_db", "loopgain_phase_deg"].map(column);
        let mut rows = 0;
        for row in lines {
            let row = row.split(',').map(str::trim).collect::<Vec<_>>();
            let f = row[frequency].parse::<f64>().unwrap();
            let actual_db = row[magnitude].parse::<f64>().unwrap();
            let actual_phase = row[phase].parse::<f64>().unwrap();
            // Independent T= gain/(1+j*f/1000)^poles oracle.
            let ratio = f / 1000.0;
            let expected_db = 20.0 * gain.log10() - 10.0 * poles * (1.0 + ratio * ratio).log10();
            let expected_phase = -poles * ratio.atan().to_degrees();
            assert!(
                (actual_db - expected_db).abs() < 1e-8,
                "{actual_db} versus {expected_db} at {f}Hz"
            );
            assert!(
                (actual_phase - expected_phase).abs() < 1e-8,
                "{actual_phase} versus {expected_phase} at {f}Hz"
            );
            rows += 1;
        }
        assert_eq!(rows, if poles == 1.0 { 1 } else { 61 });
    }
}
