mod common;

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn timeout_interrupts_control_assignments_conditions_and_command_arguments() {
    let functions = work_functions();
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
        "option abstol=1n reltol={work(26)}",
        "pz in 0 out 0 vol pz\nprint pole(work(26))",
        "pz in 0 out 0 vol pz\nsettype frequency pole(work(26))",
        "pz in 0 out 0 vol pz\nplot pole(1) xlimit 0 work(26)",
    ]
    .iter()
    .enumerate()
    {
        let deck = dir.join(format!("control-{index}.cir"));
        std::fs::write(&deck, format!(
            "* control deadline\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1u\n{functions}.control\n{script}\n.endc\n.end\n"
        )).unwrap();
        assert_times_out(&deck, script, true);
    }
}

#[test]
fn timeout_interrupts_parameter_evaluation_during_parsing() {
    let functions = work_functions();
    let dir = common::test_dir("parameter-expression-timeout");
    for (index, declaration) in [
        ".PARAM expensive={work(26)}",
        ".PARAM expensive='work(26)'",
        ".PARAM expensive=work(26)+1",
        ".GLOBAL_PARAM expensive={work(26)}",
        ".SUBCKT cell a\n.PARAM expensive={work(26)}\nR1 a 0 {expensive}\n.ENDS cell\nX1 out cell",
    ]
    .iter()
    .enumerate()
    {
        let deck = dir.join(format!("parameter-{index}.cir"));
        std::fs::write(&deck, format!(
            "* parameter deadline\n{functions}{declaration}\nV1 out 0 1\nR1 out 0 1k\n.OP\n.END\n"
        )).unwrap();
        assert_times_out(&deck, declaration, false);
    }
}

#[test]
fn timeout_interrupts_numeric_cards_and_forward_reference_retries() {
    let functions = work_functions();
    let dir = common::test_dir("numeric-card-expression-timeout");
    for (index, card) in [
        ".OPTIONS reltol={work(26)}\n.OP",
        ".TRAN {work(26)} 1",
        ".TRAN 1 {work(26)}",
        ".TRAN 1 100 {work(26)}",
        ".AC LIN {work(26)} 1 100",
        ".DC V1 0 {work(26)} 1",
        ".IC V(out)={work(26)}\n.OP",
        ".NODESET V(out)={work(26)}\n.OP",
        ".TRAN {later+work(26)} 1\n.PARAM later=1",
        ".IC V(out)={later+work(26)}\n.PARAM later=1\n.OP",
        ".OPTIONS TEMP={later+work(26)}\n.PARAM later=1\n.OP",
    ]
    .iter()
    .enumerate()
    {
        let deck = dir.join(format!("numeric-{index}.cir"));
        std::fs::write(
            &deck,
            format!("* numeric card deadline\n{functions}V1 out 0 1\nR1 out 0 1k\n{card}\n.END\n"),
        )
        .unwrap();
        assert_times_out(&deck, card, false);
    }
}

#[test]
fn timeout_interrupts_conditionals_and_eager_element_expressions() {
    let functions = work_functions();
    let dir = common::test_dir("eager-element-expression-timeout");
    for (index, body) in [
        ".IF {work(26)}\nR2 out 0 1k\n.ENDIF",
        ".IF 0\n.ELSEIF {work(26)}\nR2 out 0 1k\n.ENDIF",
        ".SUBCKT cell a\n.IF {work(26)}\nR2 a 0 1k\n.ENDIF\n.ENDS\nX1 out cell",
        ".MODEL dd D(IS={work(26)})\nD1 out 0 dd",
        ".MODEL buffer d_buffer(rise_delay=work(26)+0)",
        "R2 out 0 1k TEMP={work(26)}",
        "R2 out 0 1k TEMP=work(26)+0",
        "C1 out 0 {work(26)}",
        "C1 out 0 1u {work(26)}",
        "V2 a 0 SIN(0 1 1 {work(26)})\nR2 a 0 1k",
        "I2 out 0 PULSE(0 1 {work(26)})",
        ".SUBCKT cell a PARAMS: value={work(26)}\nR2 a 0 {value}\n.ENDS\nX1 out cell",
        ".SUBCKT cell a PARAMS: value={later+work(26)} later=1\nR2 a 0 {value}\n.ENDS\nX1 out cell",
    ]
    .iter()
    .enumerate()
    {
        let deck = dir.join(format!("eager-{index}.cir"));
        std::fs::write(&deck, format!(
            "* eager expression deadline\n{functions}V1 out 0 1\nR1 out 0 1k\n{body}\n.OP\n.END\n"
        )).unwrap();
        assert_times_out(&deck, body, false);
    }
}

fn assert_times_out(deck: &std::path::Path, source: &str, execution_started: bool) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(deck)
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
                "{source} ignored its 50 ms deadline: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(124),
        "{source}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["error"]["category"], "timeout", "{source}: {error}");
    assert_eq!(error["error"]["exit_code"], 124);
    if execution_started {
        let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(summary["status"], "timed_out", "{source}: {summary}");
    } else {
        // Parsing has not produced a run to summarize; the process-level JSON
        // error is the machine-readable contract for preflight failures.
        assert!(output.stdout.is_empty(), "{source}: {output:?}");
    }
}

fn work_functions() -> String {
    let mut source = String::from(".FUNC f0() {1}\n");
    for index in 1..=26 {
        source.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    source.push_str(".FUNC work(x) {f26()}\n");
    source
}
