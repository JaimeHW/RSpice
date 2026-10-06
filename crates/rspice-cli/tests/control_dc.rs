//! Public CLI publication must retain the core DC grid and selected mode.
mod common;
use rspice_core::execution::result_document::AxisValues;
use rspice_core::execution::{AnalysisResultDocument, SignalUnit};
use std::process::Command;

#[test]
fn direct_and_control_dc_publish_list_modes_and_both_axis_units() {
    let dir = common::test_dir("control-dc-grids");
    let mut documents = Vec::new();
    for (name, cards) in [
        ("direct", ".dc I1 list 1m 0 2m V2 list 1 3"),
        (
            "explicit",
            ".control\ndc I1 list 1m 0 2m V2 list 1 3\n.endc",
        ),
        (
            "run",
            ".dc I1 list 1m 0 2m V2 list 1 3\n.control\nrun\n.endc",
        ),
    ] {
        let input = dir.join(format!("{name}.cir"));
        let output = dir.join(format!("{name}.json"));
        std::fs::write(
            &input,
            format!("DC\nI1 0 out 0\nR1 out bias 1k\nV2 bias 0 0\n{cards}\n.end\n"),
        )
        .unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&input)
            .arg("-o")
            .arg(&output)
            .args(["-f", "json"])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let published = if name == "direct" {
            output
        } else {
            dir.join(format!("{name}.dc-001.json"))
        };
        let document: AnalysisResultDocument =
            serde_json::from_slice(&std::fs::read(published).unwrap()).unwrap();
        let inner = document
            .axes()
            .iter()
            .find(|a| a.name() == "sweep:i1")
            .unwrap();
        let outer = document
            .axes()
            .iter()
            .find(|a| a.name() == "sweep:v2")
            .unwrap();
        assert_eq!(inner.unit(), &SignalUnit::Ampere);
        assert_eq!(outer.unit(), &SignalUnit::Volt);
        let AxisValues::Real { values } = inner.values() else {
            panic!("real axis")
        };
        assert_eq!(*values, vec![1e-3, 0.0, 2e-3, 1e-3, 0.0, 2e-3]);
        documents.push(document);
    }
    for document in &documents[1..] {
        assert_eq!(document.axes(), documents[0].axes());
        assert_eq!(document.signals(), documents[0].signals());
        assert_eq!(document.device_states(), documents[0].device_states());
    }
}
