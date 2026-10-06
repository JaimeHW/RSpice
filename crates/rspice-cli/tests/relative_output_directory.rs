mod common;
use std::process::Command;

#[test]
fn configured_relative_directory_is_resolved_once_for_composed_runs() {
    for (name, cards, corners) in [
        ("scalar", ".op", false),
        ("corners", ".op", true),
        ("control", ".control\nop\n.endc", true),
        ("steps", ".step param r list 1k 2k\n.op", true),
    ] {
        let dir = common::test_dir(name);
        std::fs::write(
            dir.join("config.toml"),
            "[output]\noutput_directory=\"results\"\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("deck.cir"),
            format!("output destination\n.param r=1k\nV1 in 0 1\nR1 in 0 {{r}}\n{cards}\n.end\n"),
        )
        .unwrap();
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_rspice"));
        cmd.current_dir(&dir).args([
            "--quiet",
            "--config",
            "config.toml",
            "run",
            "deck.cir",
            "-f",
            "csv",
            "-o",
            "out.csv",
        ]);
        if corners {
            cmd.args(["--corners", "tt,ss"]);
        }
        let out = cmd.output().unwrap();
        assert!(out.status.success(), "{name}: {out:?}");
        let files = std::fs::read_dir(dir.join("results"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        assert!(!files.is_empty());
        assert!(
            files.iter().all(|path| path.is_file()
                && path
                    .extension()
                    .is_some_and(|ext| ext == "csv" || ext == "json")),
            "{files:?}"
        );
        assert!(!dir.join("results/results").exists());
    }
}
