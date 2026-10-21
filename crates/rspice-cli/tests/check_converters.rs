//! `check --converters` lists the converters a build inserts on mixed
//! boundaries, one dense row each, and `--json` emits their records.
mod common;
use common::test_dir;
use std::process::Command;

/// An XSPICE inverter between two analog nets at `.param vcc=5`: its input
/// `din` reads through ngspice's stock `adc_bridge` at vcc/2, its output
/// `dout` drives through `dac_bridge` at vcc. No connect rule reaches either.
const DECK: &str = "* converters\n\
                    .param vcc=5\n\
                    vin din 0 dc 1\n\
                    rin din 0 1k\n\
                    ainv din dout inv\n\
                    .model inv d_inverter\n\
                    rload dout 0 1k\n\
                    .op\n\
                    .end\n";

fn check(arguments: &[&str]) -> std::process::Output {
    let dir = test_dir("check_converters");
    let deck = dir.join("deck.cir");
    std::fs::write(&deck, DECK).unwrap();
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("check")
        .arg(deck)
        .args(arguments)
        .output()
        .unwrap()
}

#[test]
fn each_converter_is_one_row_with_its_port_module_and_supply() {
    let output = check(&["--converters"]);
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Converters: 2"), "{stdout}");
    let row = |node: &str| {
        stdout
            .lines()
            .find(|line| {
                line.split_whitespace()
                    .next()
                    .is_some_and(|first| first.eq_ignore_ascii_case(node))
            })
            .unwrap_or_else(|| panic!("a row for {node}: {stdout}"))
            .to_ascii_lowercase()
    };
    let header = row("NODE");
    for column in ["port", "dir", "module", "instance", "supply", "parameters"] {
        assert!(header.contains(column), "{header}");
    }
    let input = row("din");
    for cell in ["ainv.in", "input", "adc_bridge", "5 v"] {
        assert!(input.contains(cell), "{cell:?} in {input}");
    }
    let output = row("dout");
    for cell in ["ainv.out", "output", "dac_bridge", "5 v"] {
        assert!(output.contains(cell), "{cell:?} in {output}");
    }
}

#[test]
fn json_carries_every_record_with_its_sources() {
    let output = check(&["--converters", "--json"]);
    assert!(output.status.success(), "{output:?}");
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["schema"], "rspice.check");
    let converters = json["converters"].as_array().expect("a converters array");
    assert_eq!(converters.len(), 2, "{json:#}");
    for converter in converters {
        for (field, kind) in [
            ("boundary", "string"),
            ("port", "string"),
            ("direction", "string"),
            ("kind", "string"),
            ("module", "string"),
            ("instance", "string"),
            ("rules", "string"),
            ("location", "null"),
            ("supply", "object"),
            ("parameters", "array"),
            ("deck", "string"),
        ] {
            let value = &converter[field];
            let actual = match value {
                serde_json::Value::Null => "null",
                serde_json::Value::String(_) => "string",
                serde_json::Value::Object(_) => "object",
                serde_json::Value::Array(_) => "array",
                _ => "other",
            };
            assert_eq!(actual, kind, "{field} in {converter:#}");
        }
        assert_eq!(converter["rules"], "auto_bridge");
        assert_eq!(converter["supply"]["level"], 5.0);
        assert_eq!(converter["supply"]["net"], serde_json::Value::Null);
        assert_eq!(converter["supply"]["source"]["parameter"], "vcc");
        for parameter in converter["parameters"].as_array().unwrap() {
            assert_eq!(parameter["source"], "auto_bridge", "{parameter:#}");
            assert_eq!(parameter["unit"], "V");
            assert_eq!(parameter["tracks"], false);
        }
    }
    let on = |node: &str| {
        converters
            .iter()
            .find(|converter| {
                converter["boundary"]
                    .as_str()
                    .is_some_and(|boundary| boundary.eq_ignore_ascii_case(node))
            })
            .unwrap_or_else(|| panic!("a record for {node}: {json:#}"))
    };
    let parameter = |converter: &serde_json::Value, name: &str| {
        converter["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .find(|parameter| parameter["name"] == name)
            .unwrap_or_else(|| panic!("{name} in {converter:#}"))
            .clone()
    };
    let input = on("din");
    assert_eq!(input["kind"], "adc_bridge");
    assert_eq!(input["module"], "adc_bridge");
    assert_eq!(input["direction"], "In");
    assert_eq!(parameter(input, "in_low")["value"], 2.5);
    assert_eq!(parameter(input, "in_low")["rule"], "vcc/2");
    let output = on("dout");
    assert_eq!(output["kind"], "dac_bridge");
    assert_eq!(output["module"], "dac_bridge");
    assert_eq!(output["direction"], "Out");
    assert_eq!(parameter(output, "out_high")["value"], 5.0);
    assert_eq!(parameter(output, "out_low")["value"], 0.0);
}
