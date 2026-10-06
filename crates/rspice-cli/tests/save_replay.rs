mod common;
use std::process::Command;

#[test]
fn explicit_probe_selection_survives_every_coordinate_reparse() {
    for (name, axis, analysis) in [
        ("step-op", ".step param r list 1k 2k", ".op"),
        ("step-implicit", ".step param r list 1k 2k", ""),
        ("temperature", ".temp -40 85", ".op"),
        (
            "control",
            ".step param r list 1k 2k",
            ".control\nop\nalter R1=3000\nop\n.endc",
        ),
    ] {
        let dir = common::test_dir(name);
        let deck = dir.join("deck.cir");
        std::fs::write(&deck, format!("saved probes\n.param r=1k\nV1 in 0 1\nR1 in out {{r}}\nR2 out 0 1k\n.save V(in)\n{axis}\n{analysis}\n.end\n")).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--save", "V(out)", "-f", "csv", "-o"])
            .arg(dir.join("results.csv"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{name}: {output:?}");
        let mut checked = 0;
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|ext| ext == "csv") {
                let text = std::fs::read_to_string(path).unwrap().to_ascii_lowercase();
                assert!(text.contains("v(out)"), "{name}: {text}");
                assert!(
                    !text.contains("v(in)") && !text.contains("i(v1)"),
                    "{name}: {text}"
                );
                checked += 1;
            }
        }
        assert!(checked > 0);
    }
}
