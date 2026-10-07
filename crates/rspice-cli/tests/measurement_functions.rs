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
