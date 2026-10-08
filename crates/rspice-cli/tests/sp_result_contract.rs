mod common;

use common::{read_json, test_dir};
use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json"])
        .args(args)
        .output()
        .unwrap()
}

fn run(deck: &Path, output: &Path, format: &str) {
    let result = cli(&[
        "run",
        deck.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
        "-f",
        format,
    ]);
    assert!(result.status.success(), "{result:?}");
}

fn convert(input: &Path, output: &Path, format: &str) {
    let result = cli(&[
        "convert",
        input.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        format,
    ]);
    assert!(result.status.success(), "{result:?}");
}

fn write_deck(path: &Path) {
    std::fs::write(path, "* mixed reference impedances\nV1 in 0 DC 0 AC 1 PORTNUM 1 Z0 50\nV2 out 0 DC 0 AC 0 PORTNUM 2 Z0 75\nR1 in out 50\n.SP LIN 3 1k 3k\n.END\n").unwrap();
}

fn signal<'a>(table: &'a Value, name: &str) -> &'a Value {
    table["signals"]
        .as_array()
        .unwrap()
        .iter()
        .find(|signal| signal["name"] == name)
        .unwrap_or_else(|| panic!("missing {name}: {table}"))
}

fn assert_references(table: &Value, units: bool) {
    for (port, impedance) in [(1, 50.0), (2, 75.0)] {
        let reference = signal(table, &format!("Z0({port})"));
        // Rawfiles carry all columns as complex when any signal is complex.
        let values = reference
            .get("values")
            .or_else(|| reference.get("real"))
            .unwrap();
        assert_eq!(
            values,
            &serde_json::json!([impedance, impedance, impedance])
        );
        if units {
            assert_eq!(reference["unit"], "ohm");
        }
    }
}

#[test]
fn typed_sp_comparison_detects_different_port_normalizations() {
    let dir = test_dir("sp_compare_references");
    let deck = dir.join("network.cir");
    let left = dir.join("left.json");
    let right = dir.join("right.json");
    write_deck(&deck);
    run(&deck, &left, "json");
    let mut changed = read_json(&left);
    changed["payload"]["ports"][1]["referenceImpedance"] = 100.0.into();
    std::fs::write(&right, serde_json::to_vec(&changed).unwrap()).unwrap();
    let result = cli(&[
        "compare",
        left.to_str().unwrap(),
        right.to_str().unwrap(),
        "--json",
    ]);
    assert_eq!(result.status.code(), Some(3), "{result:?}");
}

#[test]
fn typed_sp_conversion_preserves_per_port_reference_impedances() {
    let dir = test_dir("sp_convert_references");
    let deck = dir.join("network.cir");
    let input = dir.join("typed.json");
    write_deck(&deck);
    run(&deck, &input, "json");
    for format in ["csv", "tsv", "ascii", "raw", "hdf5", "json"] {
        let converted = dir.join(format!("converted.{format}"));
        let table = dir.join(format!("table-{format}.json"));
        convert(&input, &converted, format);
        convert(&converted, &table, "json");
        assert_references(&read_json(&table), !matches!(format, "csv" | "tsv"));
    }
}

#[test]
fn expanded_port_references_obey_both_table_value_limits() {
    let dir = test_dir("sp_reference_budget");
    let deck = dir.join("network.cir");
    let input = dir.join("typed.json");
    let output = dir.join("table.json");
    let config = dir.join("limits.toml");
    write_deck(&deck);
    run(&deck, &input, "json");
    // The document retains 31 numeric values; its expanded table needs 33.
    for limit in ["max_external_data_values", "max_result_values"] {
        std::fs::write(&config, format!("[resources]\n{limit}=31\n")).unwrap();
        std::fs::write(&output, "predecessor").unwrap();
        let result = cli(&[
            "--config",
            config.to_str().unwrap(),
            "convert",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
            "--to",
            "json",
        ]);
        assert_eq!(result.status.code(), Some(75), "{limit}: {result:?}");
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
    }
}

#[test]
fn flat_sp_runs_retain_mixed_port_references_in_every_format() {
    let dir = test_dir("sp_run_references");
    let deck = dir.join("network.cir");
    write_deck(&deck);
    for format in ["csv", "tsv", "ascii", "raw", "hdf5"] {
        let output = dir.join(format!("network.{format}"));
        let table = dir.join(format!("table-{format}.json"));
        run(&deck, &output, format);
        convert(&output, &table, "json");
        assert_references(&read_json(&table), !matches!(format, "csv" | "tsv"));
    }
}

#[test]
fn flat_sp_noise_exports_state_the_physical_units() {
    let dir = test_dir("sp_noise_units");
    let deck = dir.join("network.cir");
    write_deck(&deck);
    let source = std::fs::read_to_string(&deck)
        .unwrap()
        .replace("Z0 75", "Z0 50")
        .replace("3k\n", "3k DONOISE\n");
    std::fs::write(&deck, source).unwrap();
    for format in ["ascii", "raw", "hdf5"] {
        let output = dir.join(format!("network.{format}"));
        let table_path = dir.join(format!("table-{format}.json"));
        run(&deck, &output, format);
        convert(&output, &table_path, "json");
        let table = read_json(&table_path);
        for (name, unit) in [
            ("S_1_1", "1"),
            ("S_2_1", "1"),
            ("CY_A2_per_Hz_1_1", "A^2/Hz"),
            ("noise_reference_temperature_K", "K"),
            ("noise_normalization_4kT_J", "J"),
            ("noise_resistance_ohm", "ohm"),
            ("noise_factor_linear", "1"),
            ("minimum_noise_factor_linear", "1"),
            ("optimum_source_reflection", "1"),
        ] {
            assert_eq!(signal(&table, name)["unit"], unit, "{format}: {name}");
        }
    }
}

#[test]
fn configured_sparam_retains_its_impedance_override_and_dimensionless_units() {
    let dir = test_dir("sparam_units");
    let deck = dir.join("network.cir");
    std::fs::write(
        &deck,
        "* configured ports\nR1 in out 50\n.AC LIN 3 1k 3k\n.END\n",
    )
    .unwrap();
    for format in ["ascii", "hdf5"] {
        let output = dir.join(format!("network.{format}"));
        let table_path = dir.join(format!("table-{format}.json"));
        let result = cli(&[
            "run",
            deck.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "-f",
            format,
            "--sparam",
            "in,0,out,0",
            "--sparam-z0",
            "75",
        ]);
        assert!(result.status.success(), "{result:?}");
        convert(&output, &table_path, "json");
        let table = read_json(&table_path);
        for name in ["S11", "S21", "S12", "S22"] {
            assert_eq!(signal(&table, name)["unit"], "1", "{format}");
        }
        for name in ["Z0(1)", "Z0(2)"] {
            let reference = signal(&table, name);
            assert_eq!(reference["unit"], "ohm");
            let values = reference
                .get("values")
                .or_else(|| reference.get("real"))
                .unwrap();
            assert_eq!(values, &serde_json::json!([75.0, 75.0, 75.0]));
        }
    }
}
