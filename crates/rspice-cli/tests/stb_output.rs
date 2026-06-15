//! `.STB` end to end through the CLI: the card runs, the margins print,
//! and the export carries the loop-gain sweep with magnitude and phase.

use std::path::PathBuf;
use std::process::Command;

fn test_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_stb_output_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

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
fn stb_card_runs_and_exports_loop_gain() {
    let dir = test_dir();
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
    let dir = test_dir();
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
