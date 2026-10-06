mod common;
use std::path::Path;
use std::process::Command;

fn command(deck: &Path, output: Option<&Path>) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args(["--quiet", "run"]).arg(deck).args([
        "--corners",
        "tt-fast,tt_fast",
        "--jobs",
        "2",
        "-f",
        "csv",
    ]);
    if let Some(output) = output {
        command.arg("-o").arg(output);
    }
    command
}

#[test]
fn checkpoint_resume_names_do_not_depend_on_waveform_output() {
    for mode in ["single", "repeated", "control", "step", "alter"] {
        for save_output in [false, true] {
            let dir = common::test_dir("corner_resume");
            let deck = dir.join("deck.cir");
            let analysis = match mode {
                "repeated" => ".tran 100p 2n\n.tran 200p 2n\n",
                "control" => ".control\ntran 100p 2n\ntran 200p 2n\n.endc\n",
                "step" => ".step param r list 1k 2k\n.tran 100p 2n\n",
                "alter" => ".tran 100p 2n\n.alter hot\nR1 in out 2k\n",
                _ => ".tran 100p 2n\n",
            };
            std::fs::write(&deck, format!("Corner restart\n.param r=1k\nV1 in 0 1\nR1 in out {{r}}\nC1 out 0 1p\n{analysis}.end\n")).unwrap();
            let output = dir.join("result.csv");
            let checkpoint = dir.join("state.chk");
            let first = command(&deck, save_output.then_some(output.as_path()))
                .arg("--checkpoint")
                .arg(&checkpoint)
                .args(["--tran-stop", "1n"])
                .output()
                .unwrap();
            assert!(
                first.status.success(),
                "{mode}, output={save_output}: {first:?}"
            );
            let paths: Vec<_> = std::fs::read_dir(&dir)
                .unwrap()
                .map(Result::unwrap)
                .map(|entry| entry.path())
                .filter(|path| path.extension().is_some_and(|ext| ext == "chk"))
                .collect();
            assert_eq!(paths.len(), if mode == "single" { 2 } else { 4 });
            let resumed = command(&deck, (!save_output).then_some(output.as_path()))
                .arg("--resume")
                .arg(&checkpoint)
                .arg("--checkpoint")
                .arg(&checkpoint)
                .output()
                .unwrap();
            assert!(
                resumed.status.success(),
                "{mode}, output={save_output}: {resumed:?}"
            );
            for path in paths {
                let state = rspice_core::engine::TransientCheckpoint::load_with_abort(
                    &path,
                    &rspice_core::NoAbort,
                )
                .unwrap();
                assert!(
                    (state.time - 2e-9).abs() < 1e-18,
                    "{path:?}: {}",
                    state.time
                );
            }
        }
    }
}

#[test]
fn authored_restart_keeps_distinct_corner_files_with_or_without_waveforms() {
    for save_output in [false, true] {
        let dir = common::test_dir("corner_authored_restart");
        let first = dir.join("first.cir");
        let resumed = dir.join("resumed.cir");
        let circuit = "Corner scheduled restart\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1p\n";
        std::fs::write(
            &first,
            format!(
                "{circuit}.tran 100p 1n\n.options restart JOB=saved INITIAL_INTERVAL=1n\n.end\n"
            ),
        )
        .unwrap();
        std::fs::write(
            &resumed,
            format!("{circuit}.tran 100p 2n\n.options restart FILE=saved1e-09\n.end\n"),
        )
        .unwrap();
        let output = dir.join("result.csv");
        let result = command(&first, save_output.then_some(output.as_path()))
            .output()
            .unwrap();
        assert!(result.status.success(), "output={save_output}: {result:?}");
        for corner in ["tt-fast", "tt_fast"] {
            assert!(dir.join(format!("saved1e-09.{corner}")).is_file());
        }
        let result = command(&resumed, (!save_output).then_some(output.as_path()))
            .output()
            .unwrap();
        assert!(result.status.success(), "output={save_output}: {result:?}");
    }
}

#[test]
fn default_control_presentations_preserve_distinct_corner_spellings() {
    let dir = common::test_dir("corner_control_names");
    let deck = dir.join("deck.cir");
    std::fs::write(
        &deck,
        "Corner presentation\nV1 in 0 1\nR1 in 0 1k\n.control\nop\nprint v(in)\n.endc\n.end\n",
    )
    .unwrap();
    let result = command(&deck, None)
        .args(["--summary", "-"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let summary: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["outputs"].as_array().unwrap().len(), 2);
    for corner in ["tt-fast", "tt_fast"] {
        assert!(
            dir.join(format!("deck.{corner}.control-001.json"))
                .is_file()
        );
    }
}
