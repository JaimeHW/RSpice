//! FFT conversion must preserve provenance, metrics, and unavailable requests.
mod common;

use common::{read_json, test_dir};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .unwrap()
}

fn convert(source: &Path, destination: &Path, from: &str, to: &str, extra: &[&str]) -> Output {
    let mut args = vec![
        "convert",
        source.to_str().unwrap(),
        destination.to_str().unwrap(),
        "--from",
        from,
        "--to",
        to,
    ];
    args.extend(extra);
    cli(&args)
}

fn source(directory: &Path) -> PathBuf {
    let deck = directory.join("fft.cir");
    let requested = directory.join("waveform.json");
    std::fs::write(&deck, "FFT conversion fixture\n.param amplitude=1\nV1 out 0 SIN(0 {amplitude} 1k)\nR1 out 0 1k\n.options fft fftout=1\n.tran 1u 1m\n.step param amplitude list 1 2\n.fft v(out) np=8 format=unorm window=rect freq=1k\n.fft i(V1) np=16 window=hann freq=2k fmin=1k\n.fft v(out) np=16 window=gauss alfa=3\n.fft v(out) np=16 window=kaiser alfa=3\n.end\n").unwrap();
    let output = cli(&[
        "run",
        deck.to_str().unwrap(),
        "-o",
        requested.to_str().unwrap(),
        "-f",
        "json",
    ]);
    assert!(output.status.success(), "{output:?}");
    let mut paths: Vec<_> = std::fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.to_string_lossy().ends_with(".fft.json"))
        .collect();
    paths.sort();
    assert_eq!(paths.len(), 2);
    paths.remove(0)
}

fn roundtrip_all_formats(directory: &Path, source: &Path, expected: &serde_json::Value, tag: &str) {
    for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5"] {
        let intermediate = directory.join(format!("{tag}.{format}"));
        let recovered = directory.join(format!("{tag}.{format}.json"));
        let output = convert(source, &intermediate, "json", format, &[]);
        assert!(output.status.success(), "write {format}: {output:?}");
        let output = convert(&intermediate, &recovered, format, "json", &[]);
        assert!(output.status.success(), "read {format}: {output:?}");
        assert_eq!(
            &read_json(&recovered),
            expected,
            "{format} changed FFT document contents"
        );
    }
}

#[test]
fn all_six_formats_preserve_ragged_spectra_metrics_windows_and_run_identity() {
    let directory = test_dir("complete");
    let source = source(&directory);
    let expected = read_json(&source);
    assert!(expected["coordinate"].is_object());
    assert!(expected["results"][0]["metrics"].is_object());
    assert_ne!(
        expected["results"][0]["sampling"]["point_count"],
        expected["results"][1]["sampling"]["point_count"]
    );
    roundtrip_all_formats(&directory, &source, &expected, "complete");
}

#[test]
fn incomplete_requests_survive_mixed_and_empty_spectrum_bundles() {
    let directory = test_dir("incomplete");
    let source = source(&directory);
    let mut document = read_json(&source);
    for (tag, count) in [("mixed", 1), ("empty", 4)] {
        for result in document["results"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .take(count)
        {
            result["status"] = serde_json::json!({"kind":"incomplete-history","availableStart":0.0,"availableStop":0.0002});
            result["metrics"] = serde_json::Value::Null;
            result["spectrum"]["bins"] = serde_json::json!([]);
        }
        let input = directory.join(format!("{tag}-input.json"));
        std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
        roundtrip_all_formats(&directory, &input, &document, tag);
    }
}

#[test]
fn invalid_metadata_bins_and_partial_transform_requests_preserve_existing_output() {
    let directory = test_dir("invalid");
    let source = source(&directory);
    let original = read_json(&source);
    let destination = directory.join("protected.json");
    std::fs::write(&destination, "predecessor").unwrap();
    for pointer in [
        "/results/3/spectrum/bins/2/magnitude",
        "/results/3/transform/fundamental_bin",
        "/results/3/metrics/thd_ratio",
    ] {
        let mut document = original.clone();
        *document.pointer_mut(pointer).unwrap() = serde_json::json!(999999);
        let input = directory.join("invalid.json");
        std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
        let output = convert(&input, &destination, "json", "json", &[]);
        assert_eq!(output.status.code(), Some(1), "{pointer}: {output:?}");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "predecessor"
        );
    }
    for flags in [
        vec!["--variables", "V(out)"],
        vec!["--start", "1k"],
        vec!["--stop", "2k"],
    ] {
        let output = convert(&source, &destination, "json", "json", &flags);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "predecessor"
        );
    }
    let csv = directory.join("invalid.csv");
    assert!(convert(&source, &csv, "json", "csv", &[]).status.success());
    let text = std::fs::read_to_string(&csv).unwrap();
    let (head, tail) = text.split_once('\n').unwrap();
    let (row, rest) = tail.split_once('\n').unwrap();
    let broken = row.replacen("tran-001", "tran-002", 1);
    std::fs::write(&csv, format!("{head}\n{row}\n{broken}\n{rest}")).unwrap();
    let output = convert(&csv, &destination, "csv", "json", &[]);
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert_eq!(std::fs::read_to_string(destination).unwrap(), "predecessor");
}

#[test]
fn all_fft_readers_enforce_numeric_admission_before_publication() {
    let directory = test_dir("budget");
    let source = source(&directory);
    let config = directory.join("limited.toml");
    std::fs::write(&config, "[resources]\nmax_external_data_values=1\n").unwrap();
    let destination = directory.join("protected.json");
    std::fs::write(&destination, "predecessor").unwrap();
    for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5"] {
        let input = directory.join(format!("budget.{format}"));
        assert!(
            convert(&source, &input, "json", format, &[])
                .status
                .success()
        );
        let output = convert(
            &input,
            &destination,
            format,
            "json",
            &["--config", config.to_str().unwrap()],
        );
        assert_eq!(output.status.code(), Some(75), "{format}: {output:?}");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "predecessor"
        );
    }
}

#[test]
fn selecting_a_raw_plot_still_validates_other_fft_plot_metadata() {
    let directory = test_dir("raw_container");
    let source = source(&directory);
    let fft = directory.join("spectrum.raw");
    assert!(
        convert(&source, &fft, "json", "ascii", &[])
            .status
            .success()
    );
    let text = std::fs::read_to_string(fft).unwrap().replacen(
        "\"analysis_id\":\"fft-004\"",
        "\"analysis_id\":\"fft-999\"",
        1,
    );
    let multi = directory.join("container.raw");
    std::fs::write(&multi, format!("Title: Simple\nPlotname: Transient Analysis\nFlags: real double\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0 1\n{text}")).unwrap();
    let output = convert(
        &multi,
        &directory.join("selected.json"),
        "ascii",
        "json",
        &["--section", "1"],
    );
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("FFT"),
        "{output:?}"
    );
}

#[test]
fn typed_fft_comparison_retains_transform_contracts_across_formats() {
    let directory = test_dir("comparison");
    let source = source(&directory);
    for (format, extension) in [
        ("json", "json"),
        ("csv", "csv"),
        ("tsv", "tsv"),
        ("raw", "raw"),
        ("ascii", "raw"),
        ("hdf5", "h5"),
    ] {
        let destination = directory.join(format!("compare-{format}.{extension}"));
        assert!(
            convert(&source, &destination, "json", format, &[])
                .status
                .success()
        );
        let compared = cli(&[
            "compare",
            destination.to_str().unwrap(),
            source.to_str().unwrap(),
            "--json",
        ]);
        assert!(compared.status.success(), "{format}: {compared:?}");
        let report: serde_json::Value = serde_json::from_slice(&compared.stdout).unwrap();
        assert_eq!(report["comparison_passed"], true);
        assert_eq!(report["num_variables"], 4);
    }
    let mut altered = read_json(&source);
    altered["results"][0]["sampling"]["accurate_sampling"] = serde_json::json!(false);
    let changed = directory.join("different-policy.json");
    std::fs::write(&changed, serde_json::to_vec(&altered).unwrap()).unwrap();
    let compared = cli(&[
        "compare",
        changed.to_str().unwrap(),
        source.to_str().unwrap(),
        "--json",
    ]);
    assert!(!compared.status.success(), "{compared:?}");
    assert!(String::from_utf8_lossy(&compared.stdout).contains("sampling contract differs"));
    let selected = cli(&[
        "compare",
        changed.to_str().unwrap(),
        source.to_str().unwrap(),
        "--variables",
        "I(V1)",
        "--json",
    ]);
    assert!(selected.status.success(), "{selected:?}");
    let report: serde_json::Value = serde_json::from_slice(&selected.stdout).unwrap();
    assert_eq!(report["num_variables"], 1);
    for result in altered["results"].as_array_mut().unwrap() {
        result["status"] = serde_json::json!({"kind":"incomplete-history","availableStart":0.0,"availableStop":0.0002});
        result["metrics"] = serde_json::Value::Null;
        result["spectrum"]["bins"] = serde_json::json!([]);
    }
    std::fs::write(&changed, serde_json::to_vec(&altered).unwrap()).unwrap();
    let compared = cli(&[
        "compare",
        changed.to_str().unwrap(),
        changed.to_str().unwrap(),
        "--json",
    ]);
    assert!(
        !compared.status.success(),
        "unavailable spectra must not pass: {compared:?}"
    );
}
