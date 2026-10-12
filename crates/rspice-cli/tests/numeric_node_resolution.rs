//! Renaming nodes cannot change the circuit measured by an analysis.
mod common;

use common::test_dir;
use std::process::Command;

fn simulate(input: &str, output: &str, card: &str, flags: &[&str]) -> String {
    let dir = test_dir("numeric_nodes");
    let deck = dir.join("deck.cir");
    let result = dir.join("result.csv");
    std::fs::write(
        &deck,
        format!("Node labels\nV1 {input} 0 1\nR1 {input} {output} 1k\nC1 {output} 0 1u\nR2 {output} 0 1k\n{card}\n.end\n"),
    )
    .unwrap();
    let process = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["--format", "csv", "--output"])
        .arg(&result)
        .args(flags)
        .output()
        .unwrap();
    assert!(
        process.status.success(),
        "{}",
        String::from_utf8_lossy(&process.stderr)
    );
    std::fs::read_to_string(result).unwrap()
}

#[test]
fn sensitivity_measures_the_authored_numeric_node() {
    for (input, output) in [("in", "out"), ("2", "1"), ("10", "20")] {
        let csv = simulate(input, output, &format!(".sens V({output})"), &[]);
        // The `sens:parameter(..)` identity columns carry commas inside quoted
        // headings, so the table is read as CSV rather than split on commas.
        let mut table =
            csv::Reader::from_reader(common::delimited_data_text(&csv, b',').as_bytes());
        let headings = table.headers().unwrap().clone();
        let values: Vec<f64> = table
            .records()
            .next()
            .unwrap()
            .unwrap()
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let source = headings
            .iter()
            .position(|name| name.ends_with("/d(V1)"))
            .unwrap();
        let resistor = headings
            .iter()
            .position(|name| name.ends_with("/d(R1)"))
            .unwrap();
        assert!((values[source] - 0.5).abs() < 1e-9, "{csv}");
        assert!((values[resistor] + 0.00025).abs() < 1e-10, "{csv}");
    }
}

#[test]
fn numeric_pole_zero_card_and_flags_match_named_ports() {
    let named = simulate("in", "out", ".pz in 0 out 0 vol pz", &[]);
    let numeric = simulate("10", "20", ".pz 10 0 20 0 vol pz", &[]);
    // The table names the transfer's ports as authored, so those identities
    // are the only text a rename may change; every measured value is not.
    for (csv, input, output) in [(&named, "IN", "OUT"), (&numeric, "10", "20")] {
        assert!(
            csv.contains(&format!("\"pz:input(V({input},0))\""))
                && csv.contains(&format!("\"pz:output(V({output},0))\"")),
            "{csv}"
        );
    }
    assert_eq!(
        numeric
            .replace("V(10,0)", "V(IN,0)")
            .replace("V(20,0)", "V(OUT,0)"),
        named
    );
    let flagged = simulate("10", "20", "", &["--pz-input", "10", "--pz-output", "20"]);
    assert_eq!(flagged, numeric);
}
