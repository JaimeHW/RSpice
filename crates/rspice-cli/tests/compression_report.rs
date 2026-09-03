//! A compressed transient reports what it approximated, not just how much it
//! dropped.
//!
//! A compression ratio says how many samples were discarded. It says nothing
//! about whether discarding them was allowed, which is the only question that
//! decides whether the stored waveform is still evidence. The run therefore
//! has to print the worst error the compressor actually observed — which
//! signal, at what time, in absolute and relative terms, and how much of the
//! declared tolerance it consumed — alongside the ratio.

mod common;

use common::{TestDirectory, test_dir};

use std::process::Command;

/// An RC low-pass driven well above its corner, sampled far more finely than
/// the compressor's tolerance requires, so samples are genuinely discarded and
/// the certificate names a real approximated sample rather than reporting
/// "none".
const SMOOTH_DECK: &str = "* compression certificate fixture\n\
     V1 in 0 SIN(0 1 1k)\n\
     R1 in out 1k\n\
     C1 out 0 1u\n\
     .tran 1u 2m\n\
     .end\n";

fn run_compressed(directory: &TestDirectory, tolerance: &str) -> (String, String) {
    let deck = directory.path().join("smooth.sp");
    let output = directory.path().join("smooth.csv");
    std::fs::write(&deck, SMOOTH_DECK).expect("write compression fixture");

    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--compress",
            "--compress-tol",
            tolerance,
            "--output",
            output.to_str().expect("UTF-8 output path"),
            "--format",
            "csv",
        ])
        .output()
        .expect("run compressed transient");

    let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
    assert!(
        result.status.success(),
        "compressed run failed:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    (stdout, stderr)
}

#[test]
fn a_compressed_run_reports_its_worst_error_beside_the_ratio() {
    let directory = test_dir("certificate");
    let (stdout, stderr) = run_compressed(&directory, "1e-4");

    assert!(
        stdout.contains("compression ratio:"),
        "the ratio is still reported:\n{stdout}\n{stderr}"
    );

    let line = stdout
        .lines()
        .find(|line| line.contains("Worst compression error:"))
        .unwrap_or_else(|| {
            panic!("no worst-error certificate beside the ratio:\n{stdout}\n{stderr}")
        });

    for expected in [
        "voltage",
        "at t=",
        "absolute",
        "relative",
        "tolerance",
        "% used",
    ] {
        assert!(
            line.contains(expected),
            "the certificate omits {expected:?}: {line}"
        );
    }
    assert!(
        line.contains("v("),
        "the certificate must name the signal it measured: {line}"
    );
}

/// A deck whose transient carries every post-process family, so one run
/// exercises `.MEASURE`, `.FOUR`, and `.FFT` against the same trajectory.
///
/// The `.tran` step is far finer than the tolerance requires, so `--compress`
/// genuinely discards samples: a post-process evaluated on the published
/// waveform instead of the accepted one would move.
const POST_PROCESS_DECK: &str = "* compressed post-process equivalence fixture\n\
     V1 in 0 SIN(0 1 1k)\n\
     R1 in out 1k\n\
     C1 out 0 100n\n\
     .tran 200n 3m\n\
     .four 1k v(out)\n\
     .fft v(out)\n\
     .meas tran vmax MAX v(out)\n\
     .meas tran vmin MIN v(out)\n\
     .meas tran vavg AVG v(out)\n\
     .end\n";

/// Run `POST_PROCESS_DECK` in its own subdirectory and return every published
/// artifact keyed by the suffix that follows the base name.
///
/// Both runs use identical deck and output file names, so the only thing that
/// can legitimately differ between them is the absolute directory an artifact
/// records as its own provenance. That prefix is normalized out; every other
/// byte is compared as published.
fn run_post_process_deck(
    directory: &TestDirectory,
    tag: &str,
    extra_args: &[&str],
) -> std::collections::BTreeMap<String, Vec<u8>> {
    let run_directory = directory.path().join(tag);
    std::fs::create_dir_all(&run_directory).expect("create post-process run directory");
    let deck = run_directory.join("deck.sp");
    let output = run_directory.join("out.json");
    std::fs::write(&deck, POST_PROCESS_DECK).expect("write post-process fixture");

    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args([
        "--quiet",
        "run",
        deck.to_str().expect("UTF-8 deck path"),
        "--output",
        output.to_str().expect("UTF-8 output path"),
        "--format",
        "json",
        "--meas-file",
        run_directory
            .join("out.meas.csv")
            .to_str()
            .expect("UTF-8 measurement path"),
        "--meas-format",
        "csv",
    ]);
    command.args(extra_args);
    let result = command.output().expect("run post-process deck");
    assert!(
        result.status.success(),
        "post-process run {tag} failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );

    let own_directory = run_directory
        .to_str()
        .expect("UTF-8 run directory")
        .to_string();
    std::fs::read_dir(&run_directory)
        .expect("list post-process artifacts")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("out."))
        })
        .map(|path| {
            let suffix = path
                .file_name()
                .and_then(|name| name.to_str())
                .expect("UTF-8 artifact name")
                .trim_start_matches("out.")
                .to_string();
            let text = std::fs::read_to_string(&path)
                .expect("post-process artifacts are text")
                .replace(&own_directory, "<run>");
            (suffix, text.into_bytes())
        })
        .collect()
}

/// `.MEASURE`, `.FOUR`, and `.FFT` are evaluated on the accepted trajectory,
/// so `--compress` — which only decimates the published waveform — must not
/// move a single digit of them.
#[test]
fn post_process_artifacts_are_byte_identical_with_and_without_compression() {
    let directory = test_dir("post_process_equivalence");
    let plain = run_post_process_deck(&directory, "plain", &[]);
    let compressed = run_post_process_deck(
        &directory,
        "compressed",
        &["--compress", "--compress-tol", "1e-4"],
    );

    assert_eq!(
        plain.keys().collect::<Vec<_>>(),
        compressed.keys().collect::<Vec<_>>(),
        "a compressed run published a different artifact set"
    );

    // The waveform itself is allowed to differ: that is what compression does,
    // and the run reports the error it accepted. Everything derived from the
    // accepted trajectory is not.
    let waveform = "tran-001.json";
    assert!(
        plain.contains_key(waveform),
        "no transient waveform artifact was published: {:?}",
        plain.keys().collect::<Vec<_>>()
    );
    assert_ne!(
        plain[waveform], compressed[waveform],
        "the compressed run retained every sample, so this fixture proves nothing"
    );

    for (suffix, plain_bytes) in &plain {
        if suffix == waveform {
            continue;
        }
        assert_eq!(
            plain_bytes, &compressed[suffix],
            "compression moved the post-process artifact '{suffix}'"
        );
    }

    // Name the three families explicitly so a fixture that silently stops
    // publishing one cannot make this test vacuous.
    for expected in ["four-001.json", "fft.json", "meas.csv"] {
        assert!(
            plain.contains_key(expected),
            "fixture published no '{expected}': {:?}",
            plain.keys().collect::<Vec<_>>()
        );
    }
}

#[test]
fn a_run_that_discarded_nothing_says_so_rather_than_omitting_the_certificate() {
    let directory = test_dir("lossless");
    // A tolerance far below the solver's own accuracy leaves every accepted
    // sample in place. Silence would be indistinguishable from a report that
    // was never computed, so the line still has to appear.
    let (stdout, stderr) = run_compressed(&directory, "1e-30");

    assert!(
        stdout.contains("Worst compression error: none"),
        "a lossless compression must still state that nothing was approximated:\n\
         {stdout}\n{stderr}"
    );
}
