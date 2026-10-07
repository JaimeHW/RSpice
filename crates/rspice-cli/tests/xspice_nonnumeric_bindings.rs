mod common;

use std::process::Command;

#[test]
fn nonnumeric_field_reads_fail_with_json_diagnostics_and_preserve_results() {
    for fields in [
        "real={string} string=\"text\"",
        "real={dependent} string=\"text\"",
        "real={read_field()} string=\"text\"",
        "real_array=[{string}] string=\"text\"",
        "complex=<{string} 0> string=\"text\"",
        "complex_array=[<{string} 0>] string=\"text\"",
        "real={string_array} string_array={payload}",
    ] {
        for scoped in [false, true] {
            let directory = common::test_dir("nonnumeric_field_shadow");
            let deck = directory.join("deck.cir");
            let destination = directory.join("result.csv");
            let mut body = format!("A1 [in] alias {fields}");
            if scoped {
                body = format!(".SUBCKT cell in\n{body}\n.ENDS\nX1 in cell");
            }
            std::fs::write(&deck, format!("* nonnumeric shadow\nV1 in 0 1\n{body}\n.PARAM string=7 string_array=7 payload=\"[alpha]\"\n.GLOBAL_PARAM dependent={{string}}\n.FUNC read_field() {{string}}\n.MODEL alias print_param_types(real=1)\n.OP\n.END\n")).unwrap();
            for command in ["check", "run"] {
                std::fs::write(&destination, "existing result").unwrap();
                let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
                process
                    .args(["--quiet", "--error-format", "json", command])
                    .arg(&deck);
                if command == "run" {
                    process
                        .args(["--format", "csv", "--output"])
                        .arg(&destination);
                }
                let output = process.output().unwrap();
                assert!(
                    !output.status.success(),
                    "{fields}, scoped={scoped}, {command}"
                );
                let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
                let message = error["error"]["message"].as_str().unwrap();
                assert!(
                    message.contains("nonnumeric")
                        && message.to_ascii_uppercase().contains("STRING"),
                    "{error}"
                );
                assert_eq!(error["error"]["exit_code"], output.status.code().unwrap());
                assert_eq!(
                    std::fs::read_to_string(&destination).unwrap(),
                    "existing result"
                );
            }
        }
    }
}
