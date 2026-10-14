//! A small-signal card that names a transient state (`ACTIMES=` or
//! `CHECKPOINT=`) publishes one typed result per state: a dataset named
//! `<card>@t=<time>` whose document names the `.TRAN` analysis as its parent
//! and carries the state's time.

mod common;

use common::{read_json, test_dir};

use serde_json::Value;
use std::path::Path;
use std::process::Command;

/// A diode charging a capacitor through a resistor: the diode's current, and
/// with it its conductance, falls as the capacitor charges (time constant
/// about 10 us), so each state linearizes differently.
fn deck(cards: &str) -> String {
    format!(
        "* diode-RC charging\n\
         v1 in 0 pulse(0 1 0 100n 100n 1 2) ac 1\n\
         r1 in a 10k\n\
         d1 a out dmod\n\
         c1 out 0 1n\n\
         r2 out 0 1meg\n\
         .model dmod d is=1e-14 n=1.2 cjo=5p\n\
         .tran 0.1u 20u 0 0.1u\n\
         {cards}\
         .end\n"
    )
}

fn run(deck: &Path, output: &Path) {
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--output",
            output.to_str().expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run rspice");
    assert!(
        result.status.success(),
        "rspice run failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

/// The document's `state_time` scalar, in seconds.
fn state_time(document: &Value) -> f64 {
    let scalar = document["scalars"]
        .as_array()
        .expect("scalars array")
        .iter()
        .find(|scalar| scalar["name"].as_str() == Some("state_time"))
        .unwrap_or_else(|| panic!("no state_time scalar in {:#}", document["scalars"]));
    scalar["value"]["value"]
        .as_f64()
        .expect("a real state time")
}

/// Every signal's samples, keyed by its descriptor, flattened to numbers.
fn signal_samples(document: &Value) -> Vec<(String, Vec<f64>)> {
    document["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .map(|signal| {
            let name = signal["descriptor"].to_string();
            let mut numbers = Vec::new();
            collect_numbers(&signal["values"], &mut numbers);
            (name, numbers)
        })
        .collect()
}

fn collect_numbers(value: &Value, out: &mut Vec<f64>) {
    match value {
        Value::Number(number) => out.push(number.as_f64().expect("finite sample")),
        Value::Array(items) => items.iter().for_each(|item| collect_numbers(item, out)),
        Value::Object(fields) => fields.values().for_each(|item| collect_numbers(item, out)),
        _ => {}
    }
}

#[test]
fn each_state_publishes_its_own_dataset_named_by_the_card_and_the_time() {
    let directory = test_dir("small_signal_actimes");
    let input = directory.join("actimes.cir");
    std::fs::write(&input, deck(".ac dec 2 1k 1meg actimes=10u,20u\n")).expect("write deck");
    let requested = directory.join("results.json");
    run(&input, &requested);

    let transient = read_json(&directory.join("results.tran-001.json"));
    assert_eq!(transient["analysis"]["tag"].as_str(), Some("tran-001"));
    let mut previous: Option<Vec<(String, Vec<f64>)>> = None;
    for (dataset, time) in [("ac-001@t=1e-5", 1.0e-5), ("ac-001@t=2e-5", 2.0e-5)] {
        let path = directory.join(format!("results.{dataset}.json"));
        assert!(path.exists(), "{} is missing", path.display());
        let document = read_json(&path);
        assert_eq!(
            document["analysis"]["tag"].as_str(),
            Some("ac-001"),
            "{dataset}"
        );
        assert_eq!(
            document["parentAnalysis"]["tag"].as_str(),
            Some("tran-001"),
            "{dataset}"
        );
        assert_eq!(
            document["namespaces"]["output"].as_str(),
            Some(dataset),
            "{dataset}"
        );
        assert!(
            (state_time(&document) - time).abs() <= 1.0e-12 * time,
            "{dataset}: state_time {}",
            state_time(&document)
        );
        let samples = signal_samples(&document);
        if let Some(previous) = &previous {
            assert_ne!(
                previous, &samples,
                "the two states linearize the diode at different biases"
            );
        }
        previous = Some(samples);
    }
    assert!(
        !directory.join("results.ac-001.json").exists(),
        "a card at transient states publishes no operating-point result"
    );
}

/// A control script's `run` solves each card at the operating point, so it
/// refuses a card that names a transient state instead of answering it there.
#[test]
fn a_control_script_refuses_to_run_a_card_at_a_transient_state() {
    let directory = test_dir("small_signal_control_run");
    let input = directory.join("control.cir");
    std::fs::write(
        &input,
        deck(".ac dec 2 1k 1meg actimes=10u\n.control\nrun\n.endc\n"),
    )
    .expect("write deck");
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            input.to_str().expect("UTF-8 deck path"),
            "--output",
            directory
                .join("results.json")
                .to_str()
                .expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run rspice");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !result.status.success(),
        "the run must be refused: {stderr}"
    );
    assert!(
        stderr.contains(".ac card") && stderr.contains("ACTIMES="),
        "the refusal names the card and its keyword: {stderr}"
    );
    assert!(
        !directory.join("results.ac-001.json").exists(),
        "no operating-point result is published for the card"
    );
}

#[test]
fn a_checkpoint_file_publishes_the_state_it_holds() {
    let directory = test_dir("small_signal_checkpoint");
    // The run that writes the state at 20 us.
    let writer = directory.join("writer.cir");
    std::fs::write(
        &writer,
        deck(".options restart job=diode_state initial_interval=20u\n"),
    )
    .expect("write the writer deck");
    run(&writer, &directory.join("writer.json"));
    // The schedule writes the state at t=0 as well; take the one at 20 us.
    let checkpoint = std::fs::read_dir(directory.path())
        .expect("list the deck directory")
        .map(|entry| entry.expect("directory entry").file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .find(|name| name.starts_with("diode_state") && name != "diode_state0")
        .expect("the restart schedule wrote the checkpoint at 20 us");

    // The same state, linearized live and from the file.
    let live = directory.join("live.cir");
    std::fs::write(&live, deck(".ac dec 2 1k 1meg actimes=20u\n")).expect("write deck");
    run(&live, &directory.join("live.json"));
    let reader = directory.join("reader.cir");
    std::fs::write(
        &reader,
        deck(&format!(".ac dec 2 1k 1meg checkpoint=\"{checkpoint}\"\n")),
    )
    .expect("write the reader deck");
    run(&reader, &directory.join("reader.json"));

    let live = read_json(&directory.join("live.ac-001@t=2e-5.json"));
    let read = read_json(&directory.join("reader.ac-001@t=2e-5.json"));
    assert_eq!(read["parentAnalysis"]["tag"].as_str(), Some("tran-001"));
    assert_eq!(state_time(&read), state_time(&live));
    let (live, read) = (signal_samples(&live), signal_samples(&read));
    assert_eq!(live.len(), read.len());
    for ((name, expected), (read_name, actual)) in live.iter().zip(&read) {
        assert_eq!(name, read_name);
        assert_eq!(expected.len(), actual.len(), "{name}");
        for (expected, actual) in expected.iter().zip(actual) {
            assert!(
                (expected - actual).abs() <= 1.0e-9 * expected.abs().max(1.0e-12),
                "{name}: {actual} from the checkpoint against {expected} live"
            );
        }
    }
}
