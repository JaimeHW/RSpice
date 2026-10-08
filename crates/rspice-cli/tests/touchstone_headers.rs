//! Header interpretation must preserve the meaning of imported RF samples.
mod common;

use common::{read_json, test_dir};
use std::path::Path;
use std::process::{Command, Output};

fn convert(input: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "convert"])
        .arg(input)
        .arg(output)
        .args(["--to", "json"])
        .output()
        .unwrap()
}

fn signal<'a>(table: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    table["signals"]
        .as_array()
        .unwrap()
        .iter()
        .find(|signal| signal["name"] == name)
        .unwrap_or_else(|| panic!("missing {name} in {table}"))
}

#[test]
fn later_option_lines_cannot_reinterpret_network_samples() {
    let dir = test_dir("touchstone_first_options");
    let input = dir.join("source.s1p");
    let output = dir.join("decoded.json");
    for source in [
        "# Hz S RI R 75\n# GHz S MA R 50\n1 0.2 0.3\n",
        "# Hz S RI R 75\n1 0.2 0.3\n# GHz S MA R 50\n",
        "# Hz S RI R 75\n# ignored additional option line\n1 0.2 0.3\n",
    ] {
        std::fs::write(&input, source).unwrap();
        let result = convert(&input, &output);
        assert!(result.status.success(), "{source}: {result:?}");
        let table = read_json(&output);
        assert_eq!(table["scale"]["values"], serde_json::json!([1.0]));
        assert_eq!(signal(&table, "S11")["real"], serde_json::json!([0.2]));
        assert_eq!(signal(&table, "S11")["imag"], serde_json::json!([0.3]));
        assert_eq!(signal(&table, "Z0(1)")["values"], serde_json::json!([75.0]));
    }
}

#[test]
fn default_and_reordered_options_preserve_units_values_and_references() {
    let dir = test_dir("touchstone_option_defaults");
    let input = dir.join("source.s1p");
    let output = dir.join("decoded.json");
    // Exercise every independently omitted field. Defaults are GHz, S, MA, 50 ohm.
    for mask in 0..16 {
        let mut fields = Vec::new();
        for (index, field) in ["R 75", "RI", "S", "MHz"].iter().enumerate() {
            if mask & (1 << index) != 0 {
                fields.push(*field);
            }
        }
        let source = format!("# {}\n1 0.25 90\n", fields.join(" "));
        std::fs::write(&input, &source).unwrap();
        let result = convert(&input, &output);
        assert!(result.status.success(), "{source}: {result:?}");
        let table = read_json(&output);
        let frequency = if mask & 8 != 0 { 1e6 } else { 1e9 };
        let reference = if mask & 1 != 0 { 75.0 } else { 50.0 };
        assert_eq!(table["scale"]["values"], serde_json::json!([frequency]));
        let (real, imag) = if mask & 2 != 0 {
            (0.25, 90.0)
        } else {
            (0.0, 0.25)
        };
        assert!((signal(&table, "S11")["real"][0].as_f64().unwrap() - real).abs() < 1e-14);
        assert!((signal(&table, "S11")["imag"][0].as_f64().unwrap() - imag).abs() < 1e-14);
        assert_eq!(
            signal(&table, "Z0(1)")["values"],
            serde_json::json!([reference])
        );
    }
    for options in ["RI R 75 Hz S", "s ri hz r 75", "R 75 S Hz RI"] {
        std::fs::write(&input, format!("# {options}\n1 0.2 0.3\n")).unwrap();
        let result = convert(&input, &output);
        assert!(result.status.success(), "{options}: {result:?}");
        let table = read_json(&output);
        assert_eq!(table["scale"]["values"], serde_json::json!([1.0]));
        assert_eq!(signal(&table, "S11")["imag"], serde_json::json!([0.3]));
    }
}

#[test]
fn conflicting_or_unknown_option_fields_preserve_the_destination() {
    let dir = test_dir("touchstone_invalid_options");
    let input = dir.join("source.s1p");
    let output = dir.join("decoded.json");
    for options in [
        "Hz GHz S RI",
        "Hz S RI MA",
        "Hz S S RI",
        "Hz S RI R 50 R 75",
        "Hz S RI R",
        "Hz S RI R 0",
        "Hz Y RI",
        "Hz S RI unknown",
    ] {
        std::fs::write(&input, format!("# {options}\n1 0.2 0.3\n")).unwrap();
        std::fs::write(&output, "previous result").unwrap();
        let result = convert(&input, &output);
        assert!(!result.status.success(), "{options}: {result:?}");
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "previous result");
    }
}

const TWO_PORT: &str = "[Version] 2.0\n# Hz S RI R 50\n[Number of Ports] 2\n[Number of Frequencies] 1\n[Two-Port Data Order] 21_12\n[Reference] 50 75\n[Network Data]\n1 0.1 0 0.2 0 0.3 0 0.4 0\n[End]\n";

fn assert_refused(sources: &[String], tag: &str) {
    let dir = test_dir(tag);
    let input = dir.join("source.s2p");
    let output = dir.join("decoded.json");
    for source in sources {
        std::fs::write(&input, source).unwrap();
        std::fs::write(&output, "previous result").unwrap();
        let result = convert(&input, &output);
        assert!(!result.status.success(), "{source}: {result:?}");
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "previous result");
    }
}

#[test]
fn a_partial_reference_list_cannot_invent_another_ports_impedance() {
    assert_refused(
        &[TWO_PORT.replace("[Reference] 50 75", "[Reference] 75")],
        "touchstone_incomplete_reference",
    );
}

#[test]
fn version_two_requires_declared_dimensions_and_ordering() {
    let sources = [
        "[Version] 2.0\n",
        "# Hz S RI R 50\n",
        "[Number of Ports] 2\n",
        "[Number of Frequencies] 1\n",
        "[Two-Port Data Order] 21_12\n",
    ]
    .map(|declaration| TWO_PORT.replace(declaration, ""));
    assert_refused(&sources, "touchstone_missing_declarations");
}

#[test]
fn conflicting_metadata_cannot_override_the_first_declaration() {
    let sources = [
        TWO_PORT.replace("[Version] 2.0", "[Version] 2.1\n[Version] 2.0"),
        TWO_PORT.replace(
            "[Number of Ports] 2",
            "[Number of Ports] 1\n[Number of Ports] 2",
        ),
        TWO_PORT.replace(
            "[Number of Frequencies] 1",
            "[Number of Frequencies] 2\n[Number of Frequencies] 1",
        ),
        TWO_PORT.replace(
            "[Network Data]",
            "[Matrix Format] Lower\n[Matrix Format] Full\n[Network Data]",
        ),
    ];
    assert_refused(&sources, "touchstone_conflicting_declarations");
}

#[test]
fn metadata_after_version_one_samples_cannot_transpose_the_network() {
    assert_refused(
        &["# Hz S RI R 50\n1 0.1 0 0.2 0 0.3 0 0.4 0\n[Two-Port Data Order] 12_21\n".to_string()],
        "touchstone_late_declarations",
    );
}

#[test]
fn section_markers_cannot_hide_trailing_data() {
    assert_refused(
        &[
            TWO_PORT.replace("[Network Data]", "[Network Data] 1 0.1 0"),
            TWO_PORT.replace(
                "[Network Data]",
                "[Begin Information] ignored\n[End Information]\n[Network Data]",
            ),
        ],
        "touchstone_section_arguments",
    );
}

#[test]
fn continued_port_references_keep_their_individual_values() {
    let dir = test_dir("touchstone_continued_reference");
    let input = dir.join("source.ts");
    let output = dir.join("decoded.json");
    for version in ["2.0", "2.1"] {
        let source = TWO_PORT
            .replace("[Version] 2.0", &format!("[Version] {version}"))
            .replace(
                "[Reference] 50 75",
                "[Reference]\n50\n# ignored additional option line\n75",
            );
        std::fs::write(&input, source).unwrap();
        let result = convert(&input, &output);
        assert!(result.status.success(), "{result:?}");
        let table = read_json(&output);
        assert_eq!(signal(&table, "Z0(1)")["values"], serde_json::json!([50.0]));
        assert_eq!(signal(&table, "Z0(2)")["values"], serde_json::json!([75.0]));
    }
}

#[test]
fn version_two_port_declarations_override_filename_hints() {
    let dir = test_dir("touchstone_explicit_dimensions");
    let output = dir.join("decoded.json");
    for extension in ["s1p", "s0p", "s99999999999999999999999999999999999p", "ts"] {
        let input = dir.join(format!("source.{extension}"));
        std::fs::write(&input, TWO_PORT).unwrap();
        let result = convert(&input, &output);
        assert!(result.status.success(), "{extension}: {result:?}");
        let table = read_json(&output);
        assert_eq!(signal(&table, "S21")["real"], serde_json::json!([0.2]));
        assert_eq!(signal(&table, "S12")["real"], serde_json::json!([0.3]));
        assert_eq!(signal(&table, "Z0(2)")["values"], serde_json::json!([75.0]));
    }
}

#[test]
fn triangular_two_port_networks_accept_both_declared_orders() {
    let dir = test_dir("touchstone_triangular_order");
    let input = dir.join("source.ts");
    let output = dir.join("decoded.json");
    for matrix in ["Lower", "Upper"] {
        for order in ["21_12", "12_21"] {
            let source = TWO_PORT
                .replace(
                    "[Two-Port Data Order] 21_12",
                    &format!("[Two-Port Data Order] {order}"),
                )
                .replace(
                    "[Network Data]",
                    &format!("[Matrix Format] {matrix}\n[Network Data]"),
                )
                .replace("0.2 0 0.3 0", "0.25 0");
            std::fs::write(&input, source).unwrap();
            let result = convert(&input, &output);
            assert!(result.status.success(), "{matrix}, {order}: {result:?}");
            let table = read_json(&output);
            for (name, expected) in [("S11", 0.1), ("S12", 0.25), ("S21", 0.25), ("S22", 0.4)] {
                assert_eq!(signal(&table, name)["real"], serde_json::json!([expected]));
            }
        }
    }
}

#[test]
fn every_standard_line_ending_preserves_comments_and_samples() {
    let dir = test_dir("touchstone_line_endings");
    let output = dir.join("decoded.json");
    for (extension, source) in [
        (
            "s1p",
            "! network\n# Hz S RI R 50 ! options\n1 0.25 0 ! sample\n",
        ),
        ("ts", TWO_PORT),
    ] {
        let input = dir.join(format!("source.{extension}"));
        let mut baseline = None;
        for ending in ["\n", "\r\n", "\r"] {
            std::fs::write(&input, source.replace('\n', ending)).unwrap();
            let result = convert(&input, &output);
            assert!(
                result.status.success(),
                "{extension}, {ending:?}: {result:?}"
            );
            let table = read_json(&output);
            if let Some(expected) = &baseline {
                assert_eq!(&table, expected);
            } else {
                baseline = Some(table);
            }
        }
    }
}
