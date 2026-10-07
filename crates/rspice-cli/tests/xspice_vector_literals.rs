mod common;

use std::{
    path::Path,
    process::{Command, Output},
};

fn invoke(command: &str, deck: &Path, destination: &Path) -> Output {
    let mut process = Command::new(env!("CARGO_BIN_EXE_rspice"));
    process.args(["--quiet", command]).arg(deck);
    if command == "run" {
        process
            .args(["--format", "csv", "--output"])
            .arg(destination);
    }
    process.output().unwrap()
}

#[test]
fn malformed_quoted_vectors_fail_without_replacing_results() {
    for field in [
        "real_array=\"[1 2]junk\"",
        "string_array=\"[alpha beta]junk\"",
        "complex_array=\"[<1 2>]junk\"",
        "string_array={payload}",
    ] {
        let directory = common::test_dir("quoted_vector_trailing_input");
        let deck = directory.join("deck.cir");
        let result = directory.join("result.csv");
        std::fs::write(
            &deck,
            format!(
                "* invalid vector\n.PARAM payload=\"[alpha beta]junk\"\nV1 in 0 1\n\
             A1 [in] print_param_types {field}\n.OP\n.END\n"
            ),
        )
        .unwrap();
        for command in ["check", "run"] {
            std::fs::write(&result, "existing result").unwrap();
            let output = invoke(command, &deck, &result);
            assert!(!output.status.success(), "{command}: {field}");
            let diagnostic = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(diagnostic.contains("Unexpected token"), "{diagnostic}");
            assert_eq!(std::fs::read_to_string(&result).unwrap(), "existing result");
        }
    }
}

#[test]
fn complex_forward_bindings_work_in_check_and_run() {
    let directory = common::test_dir("complex_forward_bindings");
    let deck = directory.join("deck.cir");
    let result = directory.join("result.csv");
    std::fs::write(
        &deck,
        "* forward complex fields\nV1 in 0 1\n\
         A1 [in] print_param_types complex=<{RE(z)} later(2)> complex_array=[<-r+4 {IMG(z)}>]\n\
         A2 [in] alias real={r} real_array=[{real}] complex=<{real} later(2)> complex_array=[<{real} later(2)>]\n\
         .PARAM z={2+3j} r=2\n.FUNC later(x) {x+1}\n.MODEL alias print_param_types(real=1)\n.OP\n.END\n",
    )
    .unwrap();
    for command in ["check", "run"] {
        let output = invoke(command, &deck, &result);
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert!(std::fs::read_to_string(&result).unwrap().contains("V(IN)"));
}

#[test]
fn literal_string_vector_does_not_change_another_instances_random_gain() {
    let directory = common::test_dir("literal_string_sampling");
    let deck = directory.join("deck.cir");
    let result = directory.join("result.csv");
    std::fs::write(
        &deck,
        "* literal string sample\n.OPTIONS SEED=37\n.FUNC sample() {aunif(100,1)}\n\
         V1 in 0 1\nA1 [in] print_param_types string_array=[<sample()>]\n\
         .PARAM marker={aunif(100,1)}\nA2 in out gain gain={marker}\n.OP\n.END\n",
    )
    .unwrap();
    let output = invoke("run", &deck, &result);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut reference = rspice_core::netlist::ParamContext::new();
    reference.set_random_seed(37);
    let expected = rspice_core::netlist::expr::eval_expression("aunif(100,1)", &reference).unwrap();
    let csv = std::fs::read_to_string(&result).unwrap();
    let actual: f64 = csv
        .lines()
        .filter_map(|line| line.split_once(','))
        .find(|(name, _)| name.eq_ignore_ascii_case("V(OUT)"))
        .unwrap()
        .1
        .parse()
        .unwrap();
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}
