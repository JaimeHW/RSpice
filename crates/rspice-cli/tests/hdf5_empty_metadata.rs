mod common;

use std::process::Command;

#[test]
fn empty_optional_text_survives_hdf5_roundtrip() {
    let dir = common::test_dir("empty-hdf5-text");
    let source = dir.join("source.json");
    let encoded = dir.join("encoded.h5");
    let decoded = dir.join("decoded.json");
    let original = serde_json::json!({
        "analysis":"", "plot_name":"",
        "scale":{"name":"time", "type":"time", "values":[0.0,1.0]},
        "signals":[{"name":"V(out)","type":"voltage","values":[1.0,2.0]}]
    });
    std::fs::write(&source, original.to_string()).unwrap();
    for (input, output, format) in [(&source, &encoded, "hdf5"), (&encoded, &decoded, "json")] {
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(input)
            .arg(output)
            .args(["--to", format])
            .output()
            .unwrap();
        assert!(result.status.success(), "{result:?}");
    }
    let recovered: serde_json::Value =
        serde_json::from_slice(&std::fs::read(decoded).unwrap()).unwrap();
    assert_eq!(recovered, original);
}
