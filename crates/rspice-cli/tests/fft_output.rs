//! Typed CLI publication contract for source-authored transient `.FFT`.

use serde_json::Value;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

struct TestDirectory(PathBuf);

impl std::ops::Deref for TestDirectory {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn test_dir(tag: &str) -> TestDirectory {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let serial = NEXT.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "rspice_fft_output_{}_{}_{}",
        std::process::id(),
        tag,
        serial
    ));
    std::fs::create_dir_all(&path).expect("create FFT output test directory");
    TestDirectory(path)
}

fn run(deck: &Path, output: &Path, format: &str, extra: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args([
        "--quiet",
        "run",
        deck.to_str().expect("UTF-8 deck path"),
        "--output",
        output.to_str().expect("UTF-8 output path"),
        "--format",
        format,
    ]);
    command.args(extra);
    command.output().expect("run FFT CLI fixture")
}

fn write_deck(directory: &Path, name: &str, source: &str) -> PathBuf {
    let path = directory.join(name);
    std::fs::write(&path, source).expect("write FFT CLI fixture");
    path
}

/// Every per-coordinate FFT bundle the CLI published next to `results.json`,
/// in directory order. The names carry the planner's stable coordinate tag,
/// so the test discovers them instead of re-deriving the naming rule.
fn coordinate_fft_artifacts(directory: &Path) -> Vec<PathBuf> {
    let mut artifacts = std::fs::read_dir(directory)
        .expect("read FFT output directory")
        .map(|entry| entry.expect("read FFT output entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("results.run_") && name.ends_with(".fft.json"))
        })
        .collect::<Vec<_>>();
    artifacts.sort();
    artifacts
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).expect("read JSON artifact"))
        .expect("parse JSON artifact")
}

fn assert_no_staging_file(directory: &Path) {
    let entries = std::fs::read_dir(directory)
        .expect("read FFT output directory")
        .map(|entry| entry.expect("read FFT output entry").file_name())
        .collect::<Vec<_>>();
    assert!(
        entries.iter().all(|name| !name
            .to_string_lossy()
            .contains(rspice_output::STAGING_MARKER)),
        "atomic FFT staging file remained: {entries:?}"
    );
    assert!(
        entries.iter().all(|name| {
            let name = name.to_string_lossy();
            !name.contains(".rspice-pair-backup-v1-") && !name.contains(".rspice-pair-rollback-v1-")
        }),
        "grouped FFT recovery sidecar remained: {entries:?}"
    );
}

#[test]
fn json_bundle_preserves_ordered_complex_spectra_metrics_units_and_parent_identity() {
    let directory = test_dir("json");
    let deck = write_deck(
        &directory,
        "multiple_fft.cir",
        "typed FFT JSON bundle\n\
         V1 out 0 SIN(0 1 1k)\n\
         R1 out 0 1k\n\
         .options fft fftout=1\n\
         .tran 1u 1m\n\
         .fft v(out) np=8 format=unorm window=rect freq=1k\n\
         .fft i(V1) np=16 window=hann freq=1k\n\
         .end\n",
    );
    let requested = directory.join("results.json");
    let output = run(&deck, &requested, "json", &[]);
    assert!(
        output.status.success(),
        "FFT JSON run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!requested.exists(), "base artifact path was overwritten");
    assert!(directory.join("results.tran-001.json").exists());
    let artifact = directory.join("results.fft.json");
    let document = read_json(&artifact);
    assert_eq!(document["schema_version"], 2);
    assert_eq!(document["analysis"], "fft");
    assert_eq!(document["parent_analysis_id"], "tran-001");
    assert!(document["coordinate"].is_null());
    assert_eq!(document["result_count"], 2);
    assert_eq!(
        document["format_policy"]["supported"],
        serde_json::json!(["json", "csv", "tsv", "raw", "ascii", "hdf5"])
    );
    let results = document["results"].as_array().expect("FFT result array");
    assert_eq!(results[0]["analysis_id"], "fft-001");
    assert_eq!(results[1]["analysis_id"], "fft-002");
    assert_eq!(results[0]["parent_analysis_id"], "tran-001");
    assert_eq!(results[0]["signal"]["physical_type"], "voltage");
    assert_eq!(results[0]["signal"]["unit"], "V");
    assert_eq!(results[1]["signal"]["physical_type"], "current");
    assert_eq!(results[1]["signal"]["unit"], "1");
    assert_eq!(results[0]["transform"]["format"], "unnormalized");
    assert_eq!(results[1]["transform"]["format"], "normalized");
    assert_eq!(
        results[0]["spectrum"]["complex_representation"],
        "cartesian"
    );
    assert_eq!(results[0]["spectrum"]["frequency_unit"], "Hz");
    assert_eq!(results[0]["spectrum"]["value_unit"], "V");
    assert_eq!(results[1]["spectrum"]["value_unit"], "1");
    assert_eq!(results[0]["metrics"]["units"]["fundamental_magnitude"], "V");
    assert_eq!(results[1]["metrics"]["units"]["fundamental_magnitude"], "1");
    let first_bins = results[0]["spectrum"]["bins"]
        .as_array()
        .expect("first FFT bins");
    let second_bins = results[1]["spectrum"]["bins"]
        .as_array()
        .expect("second FFT bins");
    assert_eq!(first_bins.len(), 5);
    assert_eq!(second_bins.len(), 9);
    assert!(first_bins[1]["value"]["real"].is_number());
    assert!(first_bins[1]["value"]["imaginary"].is_number());
    assert!(first_bins[1]["magnitude"].is_number());
    assert!(first_bins[1]["phase_degrees"].is_number());
    for result in results {
        let metrics = &result["metrics"];
        assert!(metrics["fundamental_magnitude"].is_number());
        assert!(metrics["thd_ratio"].is_number());
        assert!(metrics["sndr_db"].is_number());
        assert!(metrics["largest_harmonics"].is_array());
    }
    assert_no_staging_file(&directory);
}

#[test]
fn csv_and_tsv_are_lossless_flattened_bin_and_metric_record_tables() {
    let directory = test_dir("delimited");
    let deck = write_deck(
        &directory,
        "fft_delimited.cir",
        "typed FFT delimited bundle\n\
         V1 out 0 SIN(0 1 1k)\n\
         R1 out 0 1k\n\
         .options fft fftout=1\n\
         .tran 1u 1m\n\
         .fft v(out) np=8 window=hann freq=1k\n\
         .end\n",
    );
    for (format, separator) in [("csv", ','), ("tsv", '\t')] {
        let requested = directory.join(format!("results.{format}"));
        let output = run(&deck, &requested, format, &[]);
        assert!(
            output.status.success(),
            "FFT {format} run failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let artifact = directory.join(format!("results.fft.{format}"));
        let text = std::fs::read_to_string(&artifact).expect("read FFT delimited artifact");
        let mut lines = text.lines();
        let header = lines.next().expect("FFT delimited header");
        assert_eq!(header.split(separator).count(), 54);
        let columns = header.split(separator).collect::<Vec<_>>();
        for required in [
            "analysis_id",
            "parent_analysis_id",
            "coordinate_id",
            "real",
            "imaginary",
            "magnitude",
            "phase_degrees",
            "thd_ratio",
            "harmonic_rank",
        ] {
            assert!(header.split(separator).any(|field| field == required));
        }
        let records = lines.collect::<Vec<_>>();
        let first_record = records
            .first()
            .expect("FFT delimited artifact has a bin record")
            .split(separator)
            .collect::<Vec<_>>();
        let physical_type = columns
            .iter()
            .position(|column| *column == "physical_type")
            .expect("physical_type column");
        let value_unit = columns
            .iter()
            .position(|column| *column == "value_unit")
            .expect("value_unit column");
        assert_eq!(first_record[physical_type], "voltage");
        assert_eq!(first_record[value_unit], "1");
        assert!(records.iter().any(|line| line.contains("bin")));
        assert!(records.iter().any(|line| line.contains("largest_harmonic")));
        assert!(
            records
                .iter()
                .all(|line| line.split(separator).count() == 54)
        );
    }
    assert_no_staging_file(&directory);
}

#[test]
fn step_and_temperature_fft_artifacts_retain_unique_canonical_coordinates() {
    let directory = test_dir("axes");
    let deck = write_deck(
        &directory,
        "fft_axes.cir",
        "typed FFT run axes\n\
         .param amp=1\n\
         V1 out 0 SIN(0 {amp} 1k)\n\
         R1 out 0 1k\n\
         .step param amp list 1 2\n\
         .temp 25 75\n\
         .tran 1u 1m\n\
         .fft v(out) np=8 window=rect\n\
         .end\n",
    );
    let requested = directory.join("results.json");
    let output = run(&deck, &requested, "json", &[]);
    assert!(
        output.status.success(),
        "FFT axis run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let artifacts = coordinate_fft_artifacts(&directory);
    assert_eq!(
        artifacts.len(),
        4,
        "one FFT bundle per STEP x TEMP coordinate"
    );
    let mut coordinate_ids = HashSet::new();
    let mut ordinals = HashSet::new();
    for artifact in &artifacts {
        let document = read_json(artifact);
        assert_eq!(document["parent_analysis_id"], "tran-001");
        let metadata = document["coordinate"].as_object().expect("FFT coordinate");
        let ordinal = metadata["ordinal"].as_u64().expect("coordinate ordinal");
        assert!(ordinals.insert(ordinal));
        let id = metadata["coordinate_id"]
            .as_str()
            .expect("stable coordinate ID");
        assert!(coordinate_ids.insert(id.to_string()));
        let tag = metadata["tag"].as_str().expect("coordinate tag");
        assert!(tag.starts_with("run-"));
        let file_name = artifact
            .file_name()
            .and_then(|name| name.to_str())
            .expect("UTF-8 artifact name");
        assert!(
            file_name.contains(&tag.replace('-', "_")),
            "artifact {file_name} must be namespaced by its coordinate tag {tag}"
        );
        let assignment = metadata["assignment"]
            .as_str()
            .expect("coordinate assignment");
        assert!(assignment.contains("PARAM amp"));
        assert!(assignment.contains("TEMP"));
    }
    assert_eq!(coordinate_ids.len(), 4);
    assert_eq!(ordinals, (1..=4).collect::<HashSet<_>>());
    assert_no_staging_file(&directory);
}

#[test]
fn repeated_transients_publish_separate_fft_bundles_with_stable_parent_ids() {
    let directory = test_dir("repeated_tran");
    let deck = write_deck(
        &directory,
        "repeated_tran_fft.cir",
        "typed FFT repeated transients\n\
         V1 out 0 SIN(0 1 1k)\n\
         R1 out 0 1k\n\
         .tran 1u 1m\n\
         .tran 2u 2m\n\
         .fft v(out) np=8 window=rect\n\
         .fft i(V1) np=16 window=hann\n\
         .end\n",
    );
    let requested = directory.join("results.json");
    let output = run(&deck, &requested, "json", &[]);
    assert!(
        output.status.success(),
        "repeated-transient FFT run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    for ordinal in 1..=2 {
        let parent_analysis_id = format!("tran-{ordinal:03}");
        assert!(
            directory
                .join(format!("results.{parent_analysis_id}.json"))
                .exists()
        );
        let document = read_json(&directory.join(format!("results.{parent_analysis_id}.fft.json")));
        assert_eq!(document["parent_analysis_id"], parent_analysis_id);
        assert!(document["coordinate"].is_null());
        let results = document["results"].as_array().expect("FFT result array");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["analysis_id"], "fft-001");
        assert_eq!(results[1]["analysis_id"], "fft-002");
        assert_eq!(results[0]["parent_analysis_id"], parent_analysis_id);
        assert_eq!(results[1]["parent_analysis_id"], parent_analysis_id);
    }
    assert_no_staging_file(&directory);
}

#[test]
fn fft_failure_and_timeout_publish_no_fft_or_staging_artifact() {
    let directory = test_dir("failure");
    let invalid = write_deck(
        &directory,
        "invalid_fft.cir",
        "invalid FFT runtime window\n\
         V1 out 0 1\n\
         R1 out 0 1k\n\
         .tran 1u 1m\n\
         .fft v(out) np=8 start=900u stop=800u\n\
         .end\n",
    );
    let invalid_output = directory.join("invalid.json");
    let failure = run(&invalid, &invalid_output, "json", &[]);
    assert!(!failure.status.success(), "invalid FFT window was accepted");

    let long = write_deck(
        &directory,
        "timeout_fft.cir",
        "cancelled FFT run\n\
         V1 out 0 SIN(0 1 1k)\n\
         R1 out 0 1k\n\
         .tran 1n 1\n\
         .fft v(out) np=1024\n\
         .end\n",
    );
    let timeout_output = directory.join("timeout.json");
    let timeout = run(&long, &timeout_output, "json", &["--timeout", "0.000001"]);
    assert!(
        !timeout.status.success(),
        "timed FFT run unexpectedly completed"
    );

    let published_fft = std::fs::read_dir(&*directory)
        .expect("read failed FFT directory")
        .map(|entry| entry.expect("read failed FFT entry").file_name())
        .any(|name| name.to_string_lossy().contains(".fft."));
    assert!(
        !published_fft,
        "failed or cancelled run published FFT output"
    );
    assert_no_staging_file(&directory);
}

#[test]
fn second_sibling_commit_failure_restores_existing_pair_or_leaves_both_absent() {
    let directory = test_dir("pair_rollback");
    let deck = write_deck(
        &directory,
        "pair_rollback.cir",
        "typed FFT pair rollback\n\
         V1 out 0 SIN(0 1 1k)\n\
         R1 out 0 1k\n\
         .tran 1u 1m\n\
         .fft v(out) np=8 window=rect\n\
         .end\n",
    );
    let requested = directory.join("results.json");
    let transient = directory.join("results.tran-001.json");
    let fft = directory.join("results.fft.json");
    std::fs::create_dir(&fft).expect("create conflicting FFT destination directory");

    let predecessor = b"preexisting transient bytes\n";
    std::fs::write(&transient, predecessor).expect("write transient predecessor");
    let output = run(&deck, &requested, "json", &[]);
    assert!(
        !output.status.success(),
        "conflicting FFT destination was replaced"
    );
    assert_eq!(
        std::fs::read(&transient).expect("read restored transient predecessor"),
        predecessor
    );
    assert!(
        fft.is_dir(),
        "the conflicting predecessor directory changed"
    );
    assert_no_staging_file(&directory);

    std::fs::remove_file(&transient).expect("remove transient predecessor");
    let output = run(&deck, &requested, "json", &[]);
    assert!(
        !output.status.success(),
        "conflicting FFT destination was replaced"
    );
    assert!(
        !transient.exists(),
        "failed pair publication left a newly committed transient sibling"
    );
    assert!(
        fft.is_dir(),
        "the conflicting predecessor directory changed"
    );
    assert_no_staging_file(&directory);
}
