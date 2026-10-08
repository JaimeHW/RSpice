//! A stepped event dump's manifest must describe the event declarations.
mod common;

use common::{read_json, test_dir};
use rspice_core::io::{VcdSignalKind, parse_vcd_file};
use std::collections::BTreeSet;
use std::process::Command;

fn check(circuit: &str, expected: &[&str]) {
    let dir = test_dir("vcd_step_schema");
    let deck = dir.join("network.cir");
    let output = dir.join("events.vcd");
    std::fs::write(
        &deck,
        format!("{circuit}\n.step param r list 1k 2k\n.tran 1n 20n\n.end\n"),
    )
    .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .arg("-o")
        .arg(&output)
        .args(["-f", "vcd"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let manifest = read_json(&dir.join("events.step_schema.json"));
    let entry = &manifest["analyses"][0];
    let schema = entry["union_schema"].as_array().unwrap();
    let names = schema
        .iter()
        .map(|descriptor| {
            descriptor["display_name"]
                .as_str()
                .unwrap()
                .to_ascii_lowercase()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        names,
        expected
            .iter()
            .map(|name| name.to_ascii_lowercase())
            .collect()
    );
    let coordinates = entry["coordinates"].as_array().unwrap();
    assert_eq!(coordinates.len(), 2);
    for coordinate in coordinates {
        assert!(coordinate["source_signal_indices"].is_null());
        assert_eq!(
            coordinate["validity"],
            serde_json::json!(vec![true; schema.len()])
        );
        let dump = parse_vcd_file(&dir.join(coordinate["artifact"].as_str().unwrap())).unwrap();
        assert_eq!(dump.signals.len(), schema.len());
        for signal in dump.signals {
            let (prefix, kind, value_type, unit) = match signal.kind {
                VcdSignalKind::Logic => ("D", "digital", "logic", "logic"),
                VcdSignalKind::Real => ("E", "scalar", "real", "unspecified"),
            };
            let name = format!("{prefix}({})", signal.variables[0].name);
            let descriptor = schema
                .iter()
                .find(|descriptor| descriptor["display_name"] == name)
                .unwrap();
            assert_eq!(descriptor["kind"], kind);
            assert_eq!(descriptor["value_type"], value_type);
            assert_eq!(descriptor["unit"], unit);
        }
    }
}

#[test]
fn event_manifest_lists_real_and_digital_traces_instead_of_analog_samples() {
    check(
        "* mixed event output\n.param r=1k\nV1 in 0 pulse(0 5 0 1n 1n 5n 10n)\nAbridge [in] [d] adc\nAdac [d] [out] dac\nAobs out rnode obs\nRout out 0 {r}\n.model adc adc_bridge(in_low=1 in_high=4)\n.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)\n.model obs v_to_real(gain=2)",
        &["D(d)", "E(rnode)"],
    );
}

#[test]
fn analog_only_dump_has_an_empty_manifest_schema() {
    check("* no events\n.param r=1k\nV1 out 0 1\nR1 out 0 {r}", &[]);
}
