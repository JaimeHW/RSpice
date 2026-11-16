//! A transient writes its rows to a results store, leaves the store beside its output, and
//! writes the output from it.

mod common;

use common::{run_rspice, test_dir};

const RC: &str = "rc step
v1 in 0 pulse(0 1 10n 1n 1n 200n 400n)
r1 in out 1k
c1 out 0 100p
.tran 5n 2u
.end
";

/// The same circuit with a measurement, whose run is read back from its store into memory
/// before its outputs are written.
const RC_MEASURED: &str = "rc step
v1 in 0 pulse(0 1 10n 1n 1n 200n 400n)
r1 in out 1k
c1 out 0 100p
.tran 5n 2u
.meas tran vmax max v(out)
.end
";

fn run(args: &[&str]) {
    let output = run_rspice(args);
    assert!(
        output.status.success(),
        "rspice {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn a_streamed_output_is_byte_identical_to_the_same_run_read_back_into_memory() {
    let dir = test_dir("results-store-streamed");
    let streamed_deck = dir.join("rc.cir");
    let measured_deck = dir.join("rc_measured.cir");
    std::fs::write(&streamed_deck, RC).unwrap();
    std::fs::write(&measured_deck, RC_MEASURED).unwrap();
    for format in ["raw", "ascii", "csv", "tsv"] {
        let streamed = dir.join(format!("streamed-{format}.out"));
        let read_back = dir.join(format!("read-back-{format}.out"));
        for (deck, output) in [(&streamed_deck, &streamed), (&measured_deck, &read_back)] {
            run(&[
                "--quiet",
                "run",
                deck.to_str().unwrap(),
                "-f",
                format,
                "-o",
                output.to_str().unwrap(),
            ]);
        }
        let streamed_bytes = std::fs::read(&streamed).unwrap();
        assert!(streamed_bytes.len() > 1000, "{format}: a whole table");
        assert_eq!(
            streamed_bytes,
            std::fs::read(&read_back).unwrap(),
            "{format}: the streamed table differs from the table read back"
        );
    }
}

#[test]
fn a_run_leaves_its_store_beside_its_output_and_the_summary_names_it() {
    let dir = test_dir("results-store-kept");
    let deck = dir.join("rc.cir");
    std::fs::write(&deck, RC).unwrap();
    let output = dir.join("out.raw");
    let summary = dir.join("summary.json");
    for _ in 0..2 {
        // The second run replaces the first one's store.
        run(&[
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "--summary",
            summary.to_str().unwrap(),
        ]);
    }
    let store = dir.join("out.rsr");
    assert!(store.join("manifest.rsm").is_file());
    assert!(store.join("runs/000000.rsr").is_file());
    let leftovers = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(".rspice-store")
        })
        .count();
    assert_eq!(leftovers, 0, "no store is left under its private name");

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).unwrap()).unwrap();
    assert_eq!(json["schema_version"], 5);
    let stores = json["stores"].as_array().unwrap();
    assert_eq!(stores.len(), 1, "{json}");
    assert_eq!(stores[0]["path"], store.display().to_string());
    assert!(stores[0]["rows"].as_u64().unwrap() > 400);
    assert_eq!(stores[0]["bytes"].as_u64().unwrap(), {
        std::fs::metadata(store.join("runs/000000.rsr"))
            .unwrap()
            .len()
    });
    assert_eq!(stores[0]["digest"].as_str().unwrap().len(), 64);
}

#[test]
fn no_store_deletes_the_store_once_the_output_is_written() {
    let dir = test_dir("results-store-deleted");
    let deck = dir.join("rc.cir");
    std::fs::write(&deck, RC).unwrap();
    let output = dir.join("out.csv");
    let summary = dir.join("summary.json");
    run(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-f",
        "csv",
        "-o",
        output.to_str().unwrap(),
        "--no-store",
        "--summary",
        summary.to_str().unwrap(),
    ]);
    assert!(output.is_file());
    let entries = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.contains("rsr"))
        .collect::<Vec<_>>();
    assert!(entries.is_empty(), "{entries:?}");
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).unwrap()).unwrap();
    assert_eq!(json["stores"], serde_json::json!([]));
}

#[test]
fn a_directory_that_is_not_a_store_is_never_replaced() {
    let dir = test_dir("results-store-foreign");
    let deck = dir.join("rc.cir");
    std::fs::write(&deck, RC).unwrap();
    let foreign = dir.join("out.rsr");
    std::fs::create_dir_all(&foreign).unwrap();
    std::fs::write(foreign.join("notes.txt"), "mine").unwrap();
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        dir.join("out.raw").to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("not a results store"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(foreign.join("notes.txt")).unwrap(),
        "mine"
    );
}
