//! Measurements must observe the parameter environment that produced each row.
mod common;

use serde_json::Value;
use std::process::Command;

#[test]
fn table_measurements_use_resolved_row_parameters_in_every_execution_route() {
    let dir = common::test_dir("frequency-table-measurements");
    for (family, command) in [
        ("AC", "ac data=points"),
        ("NOISE", "noise V(out) V1 data=points"),
    ] {
        for (route, cards) in [
            ("direct", format!(".{command}")),
            ("control", format!(".control\n{command}\n.endc")),
            ("run", format!(".{command}\n.control\nrun\n.endc")),
        ] {
            let deck = dir.join(format!("{family}-{route}.cir"));
            std::fs::write(
                &deck,
                format!(
                    "* resolved table parameters\n\
                .PARAM P=1 Q={{2*P}} C={{P*1j}}\n\
                 .FUNC row_value(x) {{x+Q+imag(C)}}\n\
                V1 out 0 DC 1 AC 1\n\
                R1 out 0 {{P*1k}}\n\
                .DATA points FREQ P\n10 3\n20 5\n10 7\n.ENDDATA\n\
                {cards}\n\
                .MEAS {family} direct FIND {{P}} AT=20\n\
                .MEAS {family} dependent FIND {{Q}} AT=20\n\
                .MEAS {family} imaginary FIND {{imag(C)}} AT=20\n\
                .MEAS {family} equation EQN {{Q}}\n\
                .MEAS {family} post PARAM='row_value(2)+direct'\n\
                .MEAS {family} function FIND {{row_value(2)}} AT=20\n\
                .MEAS {family} function_equation EQN {{row_value(2)}}\n\
                .MEAS {family}_CONT continuous FIND {{Q}} AT=20\n\
                .MEAS {family}_CONT continuous_imaginary FIND {{imag(C)}} AT=20\n\
                .MEAS {family}_CONT continuous_function FIND {{row_value(2)}} AT=20\n\
                .END\n"
                ),
            )
            .unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .args(["--summary", "-"])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{family} {route}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
            let rows = summary["runs"][0]["measurements"].as_array().unwrap();
            assert_eq!(rows.len(), 10);
            for (name, expected) in [
                ("direct", 5.0),
                ("dependent", 10.0),
                ("imaginary", 5.0),
                ("equation", 14.0),
                ("post", 28.0),
                ("function", 17.0),
                ("function_equation", 23.0),
                ("continuous_function", 17.0),
                ("continuous", 10.0),
                ("continuous_imaginary", 5.0),
            ] {
                let row = rows
                    .iter()
                    .find(|row| row["name"].as_str().unwrap().eq_ignore_ascii_case(name))
                    .unwrap();
                assert_eq!(row["value"], expected, "{family} {route} {name}: {row}");
                assert_eq!(row["passed"], true);
            }
        }
    }
}
