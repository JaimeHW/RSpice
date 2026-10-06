mod common;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn command(deck: &Path, output: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .args(["-f", "csv", "-o"])
        .arg(output)
        .args(extra)
        .output()
        .unwrap()
}

fn checkpoint_files(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(Result::unwrap)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "chk"))
        .map(|path| {
            let bytes = std::fs::read(&path).unwrap();
            (path, bytes)
        })
        .collect();
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

fn deck_body(mode: &str) -> String {
    let analyses = match mode {
        "control" => ".control\ntran 100p 2n\ntran 100p 2n\n.endc\n",
        "step" => ".step param r list 1k 2k\n.tran 100p 2n\n",
        "repeated" => ".tran 100p 2n\n.tran 200p 2n\n",
        "alter" => ".tran 100p 2n\n.alter hot\nR1 in out 2k\n",
        _ => ".tran 100p 2n\n",
    };
    format!(
        "Resume ownership\n.param r=1k\nV1 in 0 1\nR1 in out {{r}}\nC1 out 0 1p\n{analyses}.end\n"
    )
}

#[test]
fn resume_inputs_survive_results_and_reports_across_all_namespaces() {
    for mode in ["single", "repeated", "control", "step", "alter", "corners"] {
        let dir = common::test_dir("resume_ownership");
        let deck = dir.join("deck.cir");
        std::fs::write(&deck, deck_body(mode)).unwrap();
        let base = dir.join("state.chk");
        let mut options = vec!["--checkpoint", base.to_str().unwrap(), "--tran-stop", "1n"];
        if mode == "corners" {
            options.extend(["--corners", "tt,ss", "-j", "2"]);
        }
        let first = command(&deck, &dir.join("first.csv"), &options);
        assert!(first.status.success(), "{mode}: {first:?}");
        let originals = checkpoint_files(&dir);
        assert!(!originals.is_empty());
        let mut resume = vec!["--resume", base.to_str().unwrap()];
        if mode == "corners" {
            resume.extend(["--corners", "tt,ss", "-j", "2"]);
        }
        // Generated waveform names collide with the corresponding generated state names.
        let collision = command(&deck, &base, &resume);
        assert_eq!(collision.status.code(), Some(2), "{mode}: {collision:?}");
        assert!(
            String::from_utf8_lossy(&collision.stderr).contains("source"),
            "{mode}: {collision:?}"
        );
        for flags in [
            vec!["--summary"],
            vec!["--meas-file"],
            vec!["--report-format", "junit", "--report-file"],
        ] {
            let mut options = resume.clone();
            options.extend(flags);
            options.push(originals.last().unwrap().0.to_str().unwrap());
            let collision = command(&deck, &dir.join("later.csv"), &options);
            assert_eq!(collision.status.code(), Some(2), "{mode}: {collision:?}");
            assert!(
                String::from_utf8_lossy(&collision.stderr).contains("source"),
                "{mode}: {collision:?}"
            );
        }
        for (path, bytes) in &originals {
            assert_eq!(&std::fs::read(path).unwrap(), bytes, "{mode}: {path:?}");
        }
        // An explicit renewal is a checkpoint publication, not a generic output.
        resume.extend(["--checkpoint", base.to_str().unwrap()]);
        let renewed = command(&deck, &dir.join("renewed.csv"), &resume);
        assert!(renewed.status.success(), "{mode}: {renewed:?}");
        for (path, bytes) in &originals {
            assert_ne!(
                &std::fs::read(path).unwrap(),
                bytes,
                "{mode}: state was not renewed"
            );
        }
    }
}

#[test]
fn unreadable_resume_state_cannot_be_replaced_by_failure_reports() {
    for control in [false, true] {
        let dir = common::test_dir("bad_resume_ownership");
        let deck = dir.join("deck.cir");
        std::fs::write(&deck, deck_body(if control { "control" } else { "single" })).unwrap();
        let base = dir.join("state.chk");
        let source = if control {
            dir.join("state.tran-001.chk")
        } else {
            base.clone()
        };
        std::fs::write(&source, "corrupt checkpoint, preserve for diagnosis\n").unwrap();
        let result = command(
            &deck,
            &dir.join("result.csv"),
            &[
                "--resume",
                base.to_str().unwrap(),
                "--report-format",
                "junit",
                "--report-file",
                source.to_str().unwrap(),
            ],
        );
        assert_eq!(result.status.code(), Some(2), "{result:?}");
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("source"),
            "{result:?}"
        );
        assert_eq!(
            std::fs::read_to_string(&source).unwrap(),
            "corrupt checkpoint, preserve for diagnosis\n"
        );
    }
}

#[test]
fn authored_restart_inputs_are_protected_before_any_analysis_publishes() {
    let dir = common::test_dir("authored_restart_ownership");
    let source = dir.join("saved");
    // A malformed snapshot is still input. Preflight must reserve it before
    // the preceding OP or the failure-report path can replace its contents.
    std::fs::write(&source, "existing restart state\n").unwrap();
    let deck = dir.join("deck.cir");
    std::fs::write(&deck, "Restart input\nV1 in 0 1\nR1 in 0 1k\n.op\n.tran 100p 2n\n.options restart FILE=saved\n.end\n").unwrap();
    for flags in [
        vec!["--summary"],
        vec!["--report-format", "junit", "--report-file"],
    ] {
        let mut options = flags;
        options.push(source.to_str().unwrap());
        let result = command(&deck, &dir.join("result.csv"), &options);
        assert_eq!(result.status.code(), Some(2), "{result:?}");
        assert!(
            String::from_utf8_lossy(&result.stderr).contains("source"),
            "{result:?}"
        );
        assert_eq!(
            std::fs::read_to_string(&source).unwrap(),
            "existing restart state\n"
        );
        assert!(!dir.join("result.op-001.csv").exists());
    }
}

#[test]
fn a_failed_control_run_cannot_publish_over_a_later_resume_input() {
    let dir = common::test_dir("later_control_resume");
    let deck = dir.join("deck.cir");
    std::fs::write(&deck, deck_body("control")).unwrap();
    // Windows namespace matching must agree with filesystem case folding,
    // including non-ASCII names. Unix keeps the exact filename bytes.
    let base = dir.join("état.chk");
    let stem = if cfg!(windows) { "ÉTAT" } else { "état" };
    for id in ["001", "002"] {
        std::fs::write(
            dir.join(format!("{stem}.tran-{id}.chk")),
            "unreadable state\n",
        )
        .unwrap();
    }
    let later = dir.join(format!("{stem}.tran-002.chk"));
    let result = command(
        &deck,
        &dir.join("result.csv"),
        &[
            "--resume",
            base.to_str().unwrap(),
            "--summary",
            later.to_str().unwrap(),
        ],
    );
    assert_eq!(result.status.code(), Some(2), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("source"),
        "{result:?}"
    );
    assert_eq!(
        std::fs::read_to_string(later).unwrap(),
        "unreadable state\n"
    );
}
