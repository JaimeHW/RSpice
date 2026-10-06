//! Control noise must publish the same result as declarative execution.
mod common;
use rspice_core::execution::AnalysisResultDocument;
use std::process::Command;

#[test]
fn control_noise_publication_matches_direct_and_preserves_current_density_units() {
    let dir = common::test_dir("control-noise");
    let mut documents = Vec::new();
    for (name, cards) in [
        ("direct", ".noise V(out) I1 lin 3 10 100"),
        (
            "explicit",
            ".control\nnoise V(out) I1 lin 3 10 100\nprint inoise_spectrum dni(R1)\n.endc",
        ),
        ("run", ".noise V(out) I1 lin 3 10 100\n.control\nrun\n.endc"),
    ] {
        let input = dir.join(format!("{name}.cir"));
        let output = dir.join(format!("{name}.json"));
        std::fs::write(
            &input,
            format!("Noise\nI1 0 out DC 0 AC 1\nR1 out 0 1k\n{cards}\n.end\n"),
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
            dir.join(format!("{name}.noise-001.json"))
        };
        let document: AnalysisResultDocument =
            serde_json::from_slice(&std::fs::read(published).unwrap()).unwrap();
        documents.push(document);
    }
    for document in &documents[1..] {
        assert_eq!(document.axes(), documents[0].axes());
        assert_eq!(document.signals(), documents[0].signals());
        assert_eq!(document.payload(), documents[0].payload());
    }
    let presentation: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.join("explicit.control-001.json")).unwrap())
            .unwrap();
    assert_eq!(presentation["traces"][0]["y"]["unit"], "A/sqrt(Hz)");
    assert_eq!(presentation["traces"][1]["y"]["unit"], "A^2/Hz");

    // RAW preserves its traditional current variable type.
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(dir.join("explicit.cir"))
        .arg("-o")
        .arg(dir.join("explicit.raw"))
        .args(["-f", "ascii"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let raw = std::fs::read_to_string(dir.join("explicit.noise-001.raw")).unwrap();
    assert!(
        raw.lines()
            .any(|line| line.contains("inoise_spectrum") && line.contains("current")),
        "{raw}"
    );

    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(dir.join("run.cir"))
        .arg("-o")
        .arg(dir.join("run.h5"))
        .args(["-f", "hdf5"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let file = rustyhdf5::File::open(dir.join("run.noise-001.h5")).unwrap();
    let attrs = file.group("noise-001").unwrap().attrs().unwrap();
    for (index, name, unit) in [
        (0, "onoise_spectrum", "V/sqrt(Hz)"),
        (1, "inoise_spectrum", "A/sqrt(Hz)"),
    ] {
        let prefix = format!("signal_{index:04}");
        assert!(
            matches!(attrs.get(&format!("{prefix}_name")), Some(rustyhdf5::AttrValue::String(value)) if value == name)
        );
        assert!(
            matches!(attrs.get(&format!("{prefix}_unit")), Some(rustyhdf5::AttrValue::String(value)) if value == unit)
        );
    }
}
