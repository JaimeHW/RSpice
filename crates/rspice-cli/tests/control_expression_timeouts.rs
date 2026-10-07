mod common;

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn timeout_interrupts_control_assignments_conditions_and_command_arguments() {
    let dir = common::test_dir("control-expression-timeout");
    for (index, script) in [
        "let expensive=work(26)",
        "if work(26)\nend",
        "while work(26)\nend",
        "dowhile work(26)\nend",
        "repeat work(26)\nend",
        "alter R1 work(26)",
        "alter @V1[sin] [ 0 work(26) 1 ]",
        "set num_threads=work(26)",
        "pz in 0 out 0 vol pz\nprint pole(work(26))",
        "pz in 0 out 0 vol pz\nsettype frequency pole(work(26))",
        "pz in 0 out 0 vol pz\nplot pole(1) xlimit 0 work(26)",
    ]
    .iter()
    .enumerate()
    {
        let deck = dir.join(format!("control-{index}.cir"));
        std::fs::write(&deck, format!(
            "* control deadline\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1u\n.FUNC work(x) {{IF(x<=0,1,work(x-1)+work(x-1))}}\n.control\n{script}\n.endc\n.end\n"
        )).unwrap();
        let mut child = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--timeout", "0.05", "--summary", "-"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let started = Instant::now();
        while child.try_wait().unwrap().is_none() {
            if started.elapsed() >= Duration::from_secs(10) {
                let _ = child.kill();
                let output = child.wait_with_output().unwrap();
                panic!(
                    "{script} ignored its 50 ms deadline: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let output = child.wait_with_output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(124),
            "{script}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(summary["status"], "timed_out", "{script}: {summary}");
    }
}
