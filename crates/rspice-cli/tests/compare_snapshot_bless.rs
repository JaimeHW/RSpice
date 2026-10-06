//! Golden promotion must preserve the validated source in every supported format.
mod common;

use common::test_dir;
use std::path::Path;
use std::process::Command;

fn bless_and_replay(source: &Path, golden: &Path) {
    let expected = std::fs::read(source).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(source)
        .arg(golden)
        .args(["--bless", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}: {output:?}", source.display());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["blessed"], true);
    assert_eq!(std::fs::read(golden).unwrap(), expected);
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(source)
        .arg(golden)
        .args(["--abstol", "0", "--reltol", "0"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}: {output:?}", golden.display());
}

#[test]
fn all_waveform_formats_bless_and_replay_exactly() {
    let dir = test_dir("snapshot_formats");
    let input = dir.join("input.csv");
    std::fs::write(&input, "time,V(out)\n0,1\n0.000001,2\n").unwrap();
    for format in ["csv", "tsv", "json", "raw", "ascii", "hdf5"] {
        let source = dir.join(format!("source.{format}"));
        let golden = dir.join(format!("golden.{format}"));
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(&input)
            .arg(&source)
            .args(["--to", format])
            .output()
            .unwrap();
        assert!(output.status.success(), "{format}: {output:?}");
        bless_and_replay(&source, &golden);
    }
    let source = dir.join("source.vcd");
    std::fs::write(&source, "$timescale 1 ns $end\n$scope module test $end\n$var wire 2 ! bus [1:0] $end\n$upscope $end\n$enddefinitions $end\n#0\nb01 !\n#10\nb10 !\n").unwrap();
    bless_and_replay(&source, &dir.join("golden.vcd"));
    let source = dir.join("source.s2p");
    std::fs::write(
        &source,
        "! preserve this comment\n# Hz S RI R 50\n1 0 0 2 0 0 3 0 0\n",
    )
    .unwrap();
    bless_and_replay(&source, &dir.join("golden.s2p"));
}

#[test]
fn typed_fft_formats_bless_and_replay_exactly() {
    let dir = test_dir("snapshot_fft");
    let deck = dir.join("fft.cir");
    let waveform = dir.join("waveform.json");
    std::fs::write(&deck, "FFT snapshot\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.options fft\n.tran 1u 1m\n.fft v(out) np=8 window=rect\n.end\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .arg("-o")
        .arg(&waveform)
        .args(["-f", "json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let input = dir.join("waveform.fft.json");
    for format in ["csv", "tsv", "json", "raw", "ascii", "hdf5"] {
        let source = dir.join(format!("source.{format}"));
        let golden = dir.join(format!("golden.{format}"));
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(&input)
            .arg(&source)
            .args(["--to", format])
            .output()
            .unwrap();
        assert!(output.status.success(), "{format}: {output:?}");
        bless_and_replay(&source, &golden);
    }
}

#[test]
fn bless_admits_source_bytes_before_decoding_or_touching_the_golden() {
    let dir = test_dir("snapshot_byte_limit");
    let config = dir.join("limits.toml");
    std::fs::write(&config, "[resources]\nmax_external_data_bytes=1\n").unwrap();
    for extension in ["csv", "tsv", "json", "raw", "h5", "vcd", "s2p"] {
        for existing in [false, true] {
            let source = dir.join(format!("source.{extension}"));
            let golden = dir.join(format!("golden-{existing}.{extension}"));
            std::fs::write(&source, [0xff; 2]).unwrap();
            if existing {
                std::fs::write(&golden, "preserve this reference").unwrap();
            }
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .arg("--config")
                .arg(&config)
                .args(["--quiet", "compare"])
                .arg(&source)
                .arg(&golden)
                .arg("--bless")
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(75), "{extension}: {output:?}");
            if existing {
                assert_eq!(
                    std::fs::read_to_string(&golden).unwrap(),
                    "preserve this reference"
                );
            } else {
                assert!(!golden.exists());
            }
        }
    }
}
