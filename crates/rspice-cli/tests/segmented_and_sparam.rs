//! Checkpoint/resume segmentation and two-port S-parameter extraction.

use std::path::PathBuf;
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_seg_sp_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

#[test]
fn unusable_explicit_checkpoint_is_refused_before_write_and_preserves_destination() {
    let dir = test_dir("checkpoint_preflight_preserves_destination");
    let deck = dir.join("stateful.cir");
    std::fs::write(
        &deck,
        "checkpoint preflight blocker\n\
         V1 in 0 1\n\
         B1 out 0 V={SDT(V(in))}\n\
         R1 out 0 1k\n\
         .TRAN 1n 10n\n\
         .END\n",
    )
    .expect("write blocked checkpoint deck");
    let checkpoint = dir.join("state.chk");
    let original = b"last known good checkpoint bytes";
    std::fs::write(&checkpoint, original).expect("seed existing checkpoint");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--checkpoint",
        checkpoint.to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("checkpoint capability preflight failed")
            && stderr.contains("behavioral-source accepted SDT state is not checkpointed"),
        "CLI must surface core checkpoint capability failure: {stderr}"
    );
    assert_eq!(
        std::fs::read(&checkpoint).expect("read preserved checkpoint"),
        original,
        "preflight refusal must preserve the previous destination byte-for-byte"
    );
    assert!(
        std::fs::read_dir(&dir).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".rspice-checkpoint.tmp.")),
        "preflight refusal must not create an atomic-write temporary"
    );
    std::fs::remove_dir_all(dir).expect("remove checkpoint preflight fixture");
}

fn last_vout(path: &std::path::Path) -> (f64, f64) {
    let text = std::fs::read_to_string(path).expect("read csv");
    let header: Vec<&str> = text.lines().next().expect("header").split(',').collect();
    let vout_col = header
        .iter()
        .position(|c| c.eq_ignore_ascii_case("V(OUT)"))
        .expect("V(OUT) column");
    let last = text.lines().last().expect("data");
    let fields: Vec<&str> = last.split(',').collect();
    (
        fields[0].parse().expect("time"),
        fields[vout_col].parse().expect("vout"),
    )
}

fn csv_times(path: &std::path::Path) -> Vec<f64> {
    std::fs::read_to_string(path)
        .expect("read csv")
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .next()
                .expect("time column")
                .parse()
                .expect("numeric time")
        })
        .collect()
}

fn assert_retained_gaps_at_most(path: &std::path::Path, maximum_interval: f64) {
    let times = csv_times(path);
    assert!(
        times.len() >= 2,
        "{} must contain a time series",
        path.display()
    );
    for window in times.windows(2) {
        let gap = window[1] - window[0];
        let scale = window[0]
            .abs()
            .max(window[1].abs())
            .max(maximum_interval.abs())
            .max(f64::MIN_POSITIVE);
        let tolerance = 64.0 * f64::EPSILON * scale;
        assert!(
            gap <= maximum_interval + tolerance,
            "{} retained gap {gap:.17e}s from {:.17e}s to {:.17e}s exceeds {maximum_interval:.17e}s (64-ULP-scale tolerance {tolerance:.17e}s)",
            path.display(),
            window[0],
            window[1]
        );
    }
}

const BUG_1284_FIRST: &str =
    include_str!("../../../tests/xyce/Netlists/Certification_Tests/BUG_1284/bug_1284_first.cir");
const BUG_1284_RESTARTED: &str = include_str!(
    "../../../tests/xyce/Netlists/Certification_Tests/BUG_1284/bug_1284_restarted.cir"
);

#[test]
fn xyce_bug_1284_job_writes_20ns_checkpoint_and_file_resumes_to_50ns() {
    let dir = test_dir("bug1284_restart");
    let first = dir.join("bug_1284_first.cir");
    let restarted = dir.join("bug_1284_restarted.cir");
    std::fs::write(&first, BUG_1284_FIRST).expect("write first-stage BUG_1284 deck");
    std::fs::write(&restarted, BUG_1284_RESTARTED).expect("write restart BUG_1284 deck");

    let output = run_rspice(&["--quiet", "run", first.to_str().unwrap()]);
    assert!(
        output.status.success(),
        "BUG_1284 checkpoint writer failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    for name in [
        "trans_test0",
        "trans_test5e-09",
        "trans_test1e-08",
        "trans_test1.5e-08",
        "trans_test2e-08",
    ] {
        assert!(
            dir.join(name).is_file(),
            "missing restart checkpoint {name}"
        );
    }
    assert!(
        std::fs::read(dir.join("trans_test2e-08"))
            .expect("read default-encoded authored checkpoint")
            .starts_with(b"RSPICE-CPACK\0\0\0\0"),
        "authored RESTART JOB must default to packed persistence"
    );

    let resumed_csv = dir.join("resumed.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        restarted.to_str().unwrap(),
        "-o",
        resumed_csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "BUG_1284 checkpoint reader failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let final_time = std::fs::read_to_string(&resumed_csv)
        .expect("read resumed transient output")
        .lines()
        .last()
        .expect("resumed transient has samples")
        .split(',')
        .next()
        .expect("resumed transient has a time column")
        .parse::<f64>()
        .expect("resumed final time is numeric");
    assert!(
        (final_time - 50e-9).abs() <= 1e-18,
        "restart must reach the authored extended stop time, got {final_time:.17e}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn xyce_restart_rejects_namespace_escape_and_cli_checkpoint_conflict() {
    let dir = test_dir("restart_safety");
    let escaping = dir.join("escaping.cir");
    std::fs::write(
        &escaping,
        "* unsafe restart namespace\nV1 n 0 1\nR1 n 0 1k\n.TRAN 1n 2n\n.OPTIONS RESTART JOB=../escape INITIAL_INTERVAL=1n\n.END\n",
    )
    .expect("write unsafe restart deck");
    let output = run_rspice(&["--quiet", "run", escaping.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("one portable filename") || stderr.contains("exactly one file"),
        "restart path refusal should be precise: {stderr}"
    );
    assert!(!dir.parent().unwrap().join("escape0").exists());

    let safe = dir.join("safe.cir");
    std::fs::write(
        &safe,
        "* conflicting restart controls\nV1 n 0 1\nR1 n 0 1k\n.TRAN 1n 2n\n.OPTIONS RESTART JOB=safe INITIAL_INTERVAL=1n\n.END\n",
    )
    .expect("write restart conflict deck");
    let explicit = dir.join("explicit.ckpt");
    let output = run_rspice(&[
        "--quiet",
        "run",
        safe.to_str().unwrap(),
        "--checkpoint",
        explicit.to_str().unwrap(),
    ]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be combined with --checkpoint or --resume"),
        "restart control-plane conflict should be explicit: {stderr}"
    );
    assert!(!explicit.exists());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn xyce_restart_pack_selects_distinct_encodings_and_file_auto_detects_both() {
    let dir = test_dir("restart_pack");
    let packed_deck = dir.join("packed.cir");
    std::fs::write(
        &packed_deck,
        "restart encoding contract\n\
         V1 in 0 PULSE(0 1 0 100p 100p 2n 5n)\n\
         R1 in out 1k\n\
         C1 out 0 1p\n\
         .TRAN 50p 2n\n\
         .OPTIONS RESTART JOB=packed INITIAL_INTERVAL=2n PACK=1\n\
         .END\n",
    )
    .expect("write packed restart deck");
    let unpacked_deck = dir.join("unpacked.cir");
    std::fs::write(
        &unpacked_deck,
        "restart encoding contract\n\
         V1 in 0 PULSE(0 1 0 100p 100p 2n 5n)\n\
         R1 in out 1k\n\
         C1 out 0 1p\n\
         .TRAN 50p 2n\n\
         .OPTIONS RESTART JOB=unpacked INITIAL_INTERVAL=2n PACK=0\n\
         .END\n",
    )
    .expect("write unpacked restart deck");

    for deck in [&packed_deck, &unpacked_deck] {
        let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
        assert!(
            output.status.success(),
            "checkpoint writer failed for {}: {}",
            deck.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let packed_path = dir.join("packed2e-09");
    let unpacked_path = dir.join("unpacked2e-09");
    let packed_bytes = std::fs::read(&packed_path).expect("read packed checkpoint");
    let unpacked_bytes = std::fs::read(&unpacked_path).expect("read unpacked checkpoint");
    assert!(packed_bytes.starts_with(b"RSPICE-CPACK\0\0\0\0"));
    assert!(unpacked_bytes.starts_with(b"RSPICE-CHECKPOINT "));
    assert_ne!(packed_bytes, unpacked_bytes);
    assert!(
        std::fs::read_dir(&dir).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".rspice-checkpoint.tmp.")),
        "atomic checkpoint temporaries must not remain after successful writes"
    );

    // FILE always auto-detects the envelope. Deliberately author the opposite
    // PACK value to prove it is not used as a decoder selector.
    let packed_resume = dir.join("packed_resume.cir");
    std::fs::write(
        &packed_resume,
        "restart encoding contract\n\
         V1 in 0 PULSE(0 1 0 100p 100p 2n 5n)\n\
         R1 in out 1k\n\
         C1 out 0 1p\n\
         .TRAN 50p 4n\n\
         .OPTIONS RESTART FILE=packed2e-09 PACK=0\n\
         .END\n",
    )
    .expect("write packed resume deck");
    let unpacked_resume = dir.join("unpacked_resume.cir");
    std::fs::write(
        &unpacked_resume,
        "restart encoding contract\n\
         V1 in 0 PULSE(0 1 0 100p 100p 2n 5n)\n\
         R1 in out 1k\n\
         C1 out 0 1p\n\
         .TRAN 50p 4n\n\
         .OPTIONS RESTART FILE=unpacked2e-09 PACK=1\n\
         .END\n",
    )
    .expect("write unpacked resume deck");

    let packed_csv = dir.join("packed.csv");
    let unpacked_csv = dir.join("unpacked.csv");
    for (deck, csv) in [
        (&packed_resume, &packed_csv),
        (&unpacked_resume, &unpacked_csv),
    ] {
        let output = run_rspice(&[
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            csv.to_str().unwrap(),
            "-f",
            "csv",
        ]);
        assert!(
            output.status.success(),
            "checkpoint reader failed for {}: {}",
            deck.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let packed_final = last_vout(&packed_csv);
    let unpacked_final = last_vout(&unpacked_csv);
    assert_eq!(packed_final.0.to_bits(), unpacked_final.0.to_bits());
    assert_eq!(packed_final.1.to_bits(), unpacked_final.1.to_bits());

    let mut corrupt = packed_bytes;
    corrupt[40] ^= 0x01; // BLAKE3 seal, outside the compressed payload.
    std::fs::write(&packed_path, corrupt).expect("corrupt packed integrity seal");
    let output = run_rspice(&["--quiet", "run", packed_resume.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("BLAKE3 integrity check failed"),
        "corrupt authored checkpoint must fail closed: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// A run segmented through --checkpoint/--resume must land on the same
/// final state as one uninterrupted run; resuming against a different deck
/// must be refused.
#[test]
fn checkpoint_resume_matches_uninterrupted_run() {
    let dir = test_dir("ckpt");
    let deck = dir.join("rc.sp");
    std::fs::write(
        &deck,
        "* RC driven by a sine\n\
         V1 in 0 SIN(0 1 100k)\n\
         R1 in out 1k\n\
         C1 out 0 100n\n\
         .tran 10n 20u\n\
         .end\n",
    )
    .expect("write deck");

    let full = dir.join("full.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        full.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(output.status.success());

    let state = dir.join("state.ckpt");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--tran-stop",
        "10u",
        "--checkpoint",
        state.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(state.exists(), "checkpoint file must be written");

    let checkpoint_bytes = usize::try_from(
        std::fs::metadata(&state)
            .expect("checkpoint metadata")
            .len(),
    )
    .expect("checkpoint length fits usize");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .env(
            "RSPICE_MAX_EXTERNAL_DATA_BYTES",
            checkpoint_bytes.saturating_sub(1).to_string(),
        )
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "--resume",
            state.to_str().unwrap(),
        ])
        .output()
        .expect("run resource-limited checkpoint resume");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("configured encoded limit")
            && stderr.contains(&checkpoint_bytes.to_string()),
        "--resume must enforce configured external-data bytes: {stderr}"
    );

    let resumed = dir.join("resumed.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--resume",
        state.to_str().unwrap(),
        "-o",
        resumed.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let (t_full, v_full) = last_vout(&full);
    let (t_seg, v_seg) = last_vout(&resumed);
    assert!((t_full - 2e-5).abs() < 1e-9 && (t_seg - 2e-5).abs() < 1e-9);
    assert!(
        (v_full - v_seg).abs() < 1e-6,
        "segmented run must match the full run: full={v_full}, resumed={v_seg}"
    );

    // Compression is a result-storage consumer, not a solver-state consumer:
    // it must leave the exact checkpoint image and resumed trajectory intact.
    let compressed_state = dir.join("compressed-state.ckpt");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--tran-stop",
        "10u",
        "--checkpoint",
        compressed_state.to_str().unwrap(),
        "--compress",
    ]);
    assert!(
        output.status.success(),
        "compressed checkpoint stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read(&compressed_state).expect("read compressed-run checkpoint"),
        std::fs::read(&state).expect("read ordinary checkpoint"),
        "result compression must not alter the exact checkpoint state"
    );

    let compressed_resumed = dir.join("compressed-resumed.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--resume",
        compressed_state.to_str().unwrap(),
        "--compress",
        "-o",
        compressed_resumed.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "compressed resume stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (t_compressed, v_compressed) = last_vout(&compressed_resumed);
    assert!((t_compressed - t_full).abs() < 1e-9);
    assert!(
        (v_full - v_compressed).abs() < 1e-6,
        "compressed segmented run must match the full run: full={v_full}, resumed={v_compressed}"
    );

    // Fingerprint safety: a different deck cannot consume this state.
    let other = dir.join("other.sp");
    std::fs::write(
        &other,
        "* different circuit\nV1 in 0 SIN(0 1 100k)\nR1 in out 2k\nC1 out 0 100n\n.tran 10n 20u\n.end\n",
    )
    .expect("write deck");
    let output = run_rspice(&[
        "--quiet",
        "run",
        other.to_str().unwrap(),
        "--resume",
        state.to_str().unwrap(),
    ]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "mismatched checkpoint must be refused"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("different netlist"),
        "refusal should explain the fingerprint mismatch: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn compression_uses_static_solver_ceiling_across_widened_checkpoint_resume() {
    let dir = test_dir("compression_interval_ceiling");
    let deck = dir.join("rc.sp");
    std::fs::write(
        &deck,
        "compression interval ceiling\n\
         V1 in 0 SIN(0 1 100meg)\n\
         R1 in out 1k\n\
         C1 out 0 1p\n\
         .tran 10n 20n\n\
         .end\n",
    )
    .expect("write compression interval deck");

    let first_csv = dir.join("first.csv");
    let first_checkpoint = dir.join("first.ckpt");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--tran-stop",
        "10n",
        "--max-step",
        "2n",
        "--checkpoint",
        first_checkpoint.to_str().unwrap(),
        "-o",
        first_csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "ordinary first segment failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let compressed_first_csv = dir.join("compressed-first.csv");
    let compressed_first_checkpoint = dir.join("compressed-first.ckpt");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--tran-stop",
        "10n",
        "--max-step",
        "2n",
        "--checkpoint",
        compressed_first_checkpoint.to_str().unwrap(),
        "--compress",
        "-o",
        compressed_first_csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "compressed first segment failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read(&compressed_first_checkpoint).expect("read compressed first checkpoint"),
        std::fs::read(&first_checkpoint).expect("read ordinary first checkpoint"),
        "compression must preserve byte-exact solver state at the 2ns ceiling"
    );
    assert_retained_gaps_at_most(&first_csv, 2.0e-9);
    assert_retained_gaps_at_most(&compressed_first_csv, 2.0e-9);

    let resumed_csv = dir.join("resumed.csv");
    let resumed_checkpoint = dir.join("resumed.ckpt");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--resume",
        first_checkpoint.to_str().unwrap(),
        "--max-step",
        "4n",
        "--checkpoint",
        resumed_checkpoint.to_str().unwrap(),
        "-o",
        resumed_csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "ordinary widened resume failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let compressed_resumed_csv = dir.join("compressed-resumed.csv");
    let compressed_resumed_checkpoint = dir.join("compressed-resumed.ckpt");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--resume",
        compressed_first_checkpoint.to_str().unwrap(),
        "--max-step",
        "4n",
        "--checkpoint",
        compressed_resumed_checkpoint.to_str().unwrap(),
        "--compress",
        "-o",
        compressed_resumed_csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "compressed widened resume failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read(&compressed_resumed_checkpoint).expect("read compressed resumed checkpoint"),
        std::fs::read(&resumed_checkpoint).expect("read ordinary resumed checkpoint"),
        "compression must preserve byte-exact solver state after widening the ceiling to 4ns"
    );
    let ordinary_final = last_vout(&resumed_csv);
    let compressed_final = last_vout(&compressed_resumed_csv);
    assert_eq!(ordinary_final.0.to_bits(), compressed_final.0.to_bits());
    assert_eq!(ordinary_final.1.to_bits(), compressed_final.1.to_bits());
    assert_retained_gaps_at_most(&resumed_csv, 4.0e-9);
    assert_retained_gaps_at_most(&compressed_resumed_csv, 4.0e-9);

    let _ = std::fs::remove_dir_all(&dir);
}

/// Series 50Ω two-port at Z0 = 50Ω: S11 = 1/3 and S21 = 2/3, exactly and
/// at every frequency.
#[test]
fn sparam_matches_series_resistor_analytics() {
    let dir = test_dir("sparam");
    let deck = dir.join("twoport.sp");
    std::fs::write(
        &deck,
        "* series 50 ohm two-port\nR1 p1 p2 50\n.ac lin 3 1k 100k\n.end\n",
    )
    .expect("write deck");

    let s2p = dir.join("out.s2p");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--sparam",
        "p1,0,p2,0",
        "-o",
        s2p.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let text = std::fs::read_to_string(&s2p).expect("touchstone file");
    assert!(text.contains("# HZ S RI R 50"), "touchstone header: {text}");
    for line in text.lines().filter(|l| !l.starts_with(['!', '#'])) {
        let fields: Vec<f64> = line
            .split_whitespace()
            .map(|f| f.parse().expect("numeric"))
            .collect();
        assert_eq!(fields.len(), 9, "freq + 4 complex pairs: {line}");
        let (s11re, s21re, s12re, s22re) = (fields[1], fields[3], fields[5], fields[7]);
        assert!((s11re - 1.0 / 3.0).abs() < 1e-9, "S11: {line}");
        assert!((s21re - 2.0 / 3.0).abs() < 1e-9, "S21: {line}");
        assert!((s12re - s21re).abs() < 1e-12, "reciprocity: {line}");
        assert!((s22re - s11re).abs() < 1e-12, "symmetry: {line}");
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn native_sp_card_matches_series_resistor_analytics() {
    let dir = test_dir("native_sp");
    let deck = dir.join("twoport_native_sp.cir");
    std::fs::write(
        &deck,
        "* native .sp two-port\n\
         V1 p1 0 dc 0 ac 1 portnum 1 z0 50\n\
         R1 p1 p2 50\n\
         V2 p2 0 dc 0 ac 0 portnum 2 z0 50\n\
         .sp lin 3 1k 100k\n\
         .end\n",
    )
    .expect("write deck");

    let csv = dir.join("out.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let text = std::fs::read_to_string(&csv).expect("csv output");
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().expect("header").split(',').collect();
    let idx = |name: &str| {
        header
            .iter()
            .position(|column| *column == name)
            .unwrap_or_else(|| panic!("missing {name} in header {header:?}"))
    };
    let s11_re = idx("Re(S_1_1)");
    let s21_re = idx("Re(S_2_1)");
    let s12_re = idx("Re(S_1_2)");
    let s22_re = idx("Re(S_2_2)");

    for line in lines {
        let fields: Vec<f64> = line
            .split(',')
            .map(|field| field.parse().expect("numeric csv field"))
            .collect();
        assert!((fields[s11_re] - 1.0 / 3.0).abs() < 1e-9, "S11: {line}");
        assert!((fields[s21_re] - 2.0 / 3.0).abs() < 1e-9, "S21: {line}");
        assert!(
            (fields[s12_re] - fields[s21_re]).abs() < 1e-12,
            "reciprocity: {line}"
        );
        assert!(
            (fields[s22_re] - fields[s11_re]).abs() < 1e-12,
            "symmetry: {line}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}
