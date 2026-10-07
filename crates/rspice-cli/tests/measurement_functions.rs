//! Equivalent authored expressions and user functions must share measurement semantics.
mod common;

use serde_json::Value;
use std::process::Command;

#[test]
fn functions_resolve_axes_probes_and_nested_calls_in_scalar_and_continuous_measurements() {
    let dir = common::test_dir("measurement-functions");
    for (family, analysis, at) in [
        ("TRAN", ".TRAN 1n 3n", "1n"),
        ("DC", ".DC V1 0 2 1", "1"),
        ("AC", ".AC LIN 3 10 30", "20"),
        ("NOISE", ".NOISE V(out) V1 LIN 3 10 30", "20"),
    ] {
        let source = format!(
            "* function measurement equivalence\nV1 out 0 DC 2 AC 2\nR1 out 0 1k\n.FUNC axis_fn(x) {{x+TIME}}\n.FUNC voltage_scale(x) {{x*V(out)}}\n.FUNC nested(x) {{axis_fn(voltage_scale(x))}}\n.FUNC shadow(TIME) {{TIME+1}}\n.FUNC lazy(x) {{IF(x>0,voltage_scale(x),1/0)}}\n{analysis}\n\
            .MEAS {family} axis_inline FIND {{1+TIME}} AT={at}\n\
            .MEAS {family} axis_called FIND {{axis_fn(1)}} AT={at}\n\
            .MEAS {family} probe_inline FIND {{3*V(out)}} AT={at}\n\
            .MEAS {family} probe_called FIND {{voltage_scale(3)}} AT={at}\n\
            .MEAS {family} lazy_called FIND {{lazy(3)}} AT={at}\n\
            .MEAS {family} nested_inline FIND {{3*V(out)+TIME}} AT={at}\n\
            .MEAS {family} nested_called FIND {{nested(3)}} AT={at}\n\
            .MEAS {family} shadow_called FIND {{shadow(3)}} AT={at}\n\
            .MEAS {family} equation_inline EQN {{3*V(out)+TIME}}\n\
            .MEAS {family} equation_called EQN {{nested(3)}}\n\
            .MEAS {family}_CONT event FIND {{nested(3)}} AT={at}\n.END\n"
        );
        let deck = dir.join(format!("{family}.cir"));
        std::fs::write(&deck, source).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--summary", "-"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{family}: {}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
        let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
        let rows = summary["runs"][0]["measurements"].as_array().unwrap();
        assert_eq!(rows.len(), 11);
        let value = |name: &str| {
            let row = rows
                .iter()
                .find(|row| row["name"].as_str().unwrap().eq_ignore_ascii_case(name))
                .unwrap();
            assert_eq!(row["passed"], true, "{family}: {row}");
            row["value"].as_f64().unwrap()
        };
        for (called, inline) in [
            ("axis_called", "axis_inline"),
            ("probe_called", "probe_inline"),
            ("lazy_called", "probe_inline"),
            ("nested_called", "nested_inline"),
            ("equation_called", "equation_inline"),
            ("event", "nested_inline"),
        ] {
            assert!(
                (value(called) - value(inline)).abs() < 1e-12,
                "{family}: {called} differs from {inline}"
            );
        }
        assert_eq!(value("shadow_called"), 4.0);
    }
}

#[test]
fn timeout_interrupts_work_inside_one_measurement_expression() {
    use std::process::Stdio;
    use std::time::{Duration, Instant};

    let dir = common::test_dir("measurement-expression-timeout");
    for (family, analysis, at) in [
        ("TRAN", ".TRAN 1n 2n", "1n"),
        ("DC", ".DC V1 1 2 1", "1"),
        ("AC", ".AC LIN 2 10 20", "20"),
        ("NOISE", ".NOISE V(out) V1 LIN 2 10 20", "20"),
    ] {
        for (index, statement) in [
            format!(".MEAS {family} expensive FIND {{work(26)}} AT={at}"),
            format!(".MEAS {family} expensive EQN {{work(26)}}"),
            format!(".MEAS {family} expensive PARAM='work(26)'"),
            format!(".MEAS {family}_CONT expensive FIND {{work(26)}} AT={at}"),
        ]
        .iter()
        .enumerate()
        {
            let deck = dir.join(format!("timeout-{family}-{index}.cir"));
            let artifact_prefix = format!("cancelled-{family}-{index}");
            let artifact = dir.join(format!("{artifact_prefix}.json"));
            std::fs::write(&deck, format!(
            "* bounded expression cancellation\nV1 out 0 DC 1 AC 1\nR1 out 0 1k\n.FUNC work(x) {{IF(x<=0,1,work(x-1)+work(x-1))}}\n{analysis}\n{statement}\n.END\n"
        )).unwrap();
            let mut child = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["--timeout", "0.05", "--summary", "-"])
                .args(["--format", "json", "--output"])
                .arg(&artifact)
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
                        "{statement} ignored its 50 ms deadline: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            let output = child.wait_with_output().unwrap();
            assert_eq!(
                output.status.code(),
                Some(124),
                "{statement}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(summary["status"], "timed_out", "{statement}: {summary}");
            assert!(
                !std::fs::read_dir(&dir).unwrap().any(|entry| {
                    entry
                        .unwrap()
                        .file_name()
                        .to_string_lossy()
                        .starts_with(&artifact_prefix)
                }),
                "{statement} published an artifact after cancellation"
            );
        }
    }
}
