//! Physical analysis operands must not silently project complex expressions.
use rspice_core::config::ExpressionDialect;
use rspice_core::engine::ControlCircuit;
use rspice_core::execution::control::ControlCommand;
use rspice_core::netlist::{AnalysisCommand, NetlistParseOptions};
use rspice_core::{Netlist, NoAbort, ResourceLimits};

fn deck(cards: &str) -> String {
    format!("numeric fields\nV1 in 0 1 AC 1\nR1 in 0 1k\n{cards}\n.end\n")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn required_analysis_fields_reject_complex_literals_and_expressions() {
    for template in [
        ".AC LIN 3 1 VALUE",
        ".DC V1 0 VALUE .1",
        ".TRAN 1u VALUE",
        ".NOISE V(in) V1 LIN 3 1 VALUE",
        ".SP LIN 3 1 VALUE PORT1=(in)",
        ".DISTO LIN 3 1 VALUE",
        ".SENS V(in) AC LIN 3 1 VALUE",
    ] {
        Netlist::parse(&deck(&template.replace("VALUE", "2"))).expect(template);
        for value in ["{2+1j}", "{2+1e-300j}", "2j", "2.0j"] {
            let card = template.replace("VALUE", value);
            Netlist::parse(&deck(&card)).expect_err(&card);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn optional_analysis_fields_reject_complex_values_instead_of_omitting_them() {
    for template in [
        ".TRAN 1u 1m VALUE",
        ".TRAN 1u 1m 0 VALUE",
        ".NOISE V(in) V1 LIN 3 1 1000 VALUE",
        ".DISTO LIN 3 1 1000 VALUE",
        ".SP LIN 3 1 1000 PORT1=(in,0,VALUE)",
        ".TEMP 27 VALUE",
        ".STEP PARAM gain LIST 1 VALUE",
    ] {
        Netlist::parse(&deck(&template.replace("VALUE", "1u"))).expect(template);
        let card = template.replace("VALUE", "{1u+1j}");
        Netlist::parse(&deck(&card)).expect_err(&card);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn known_and_deferred_bindings_cannot_hide_an_imaginary_component() {
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        for declaration in [".param stop={2+1j}", ".global_param stop={2+1j}"] {
            for operand in ["stop", "{stop}", "+stop"] {
                for cards in [
                    format!("{declaration}\n.AC LIN 3 1 {operand}"),
                    format!(".AC LIN 3 1 {operand}\n{declaration}"),
                    format!(".AC LIN 3 1 {{later}}\n.param later={{stop}}\n{declaration}"),
                    format!(".TRAN 1u 1m {operand}\n{declaration}"),
                ] {
                    Netlist::parse_with_options(
                        &deck(&cards),
                        NetlistParseOptions {
                            expression_dialect: dialect,
                            ..Default::default()
                        },
                    )
                    .expect_err(&cards);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn control_and_direct_numeric_admission_agree() {
    for (name, arguments) in [
        ("ac", "lin 3 1 {2+1j}"),
        ("dc", "V1 0 {2+1j} 1"),
        ("tran", "1u 1m {1u+1j}"),
        ("noise", "V(in) V1 lin 3 1 {2+1j}"),
    ] {
        let command = ControlCommand {
            name: name.into(),
            arguments: arguments.into(),
            line: 9,
        };
        let error =
            ControlCircuit::parse_analysis_command(&command, ResourceLimits::default(), &NoAbort)
                .expect_err(arguments);
        assert!(error.to_string().contains("control line 9"), "{error}");
        Netlist::parse(&deck(&format!(".{name} {arguments}"))).expect_err(arguments);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_projections_lazy_branches_and_imaginary_looking_names_still_work() {
    let source = "complex names\nV1 2j 0 1\nR1 2j 0 1k\n\
        .data jtable freq\n1\n2\n.enddata\n\
        .TF V(2j) V1\n.AC DATA=jtable\n\
        .AC LIN 3 1 {real(2+1j)}\n\
        .DC V1 0 {if(1,imag(2+3j),unknown)} .5\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    assert_eq!(netlist.analyses.len(), 4);
    assert!(matches!(
        netlist.analyses[2],
        AnalysisCommand::Ac { stop_freq: 2.0, .. }
    ));
    assert!(matches!(
        netlist.analyses[3],
        AnalysisCommand::Dc { stop: 3.0, .. }
    ));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn integer_literal_fast_paths_preserve_exact_seeds_and_reject_imaginary_suffixes() {
    for card in [
        ".MC 2j uniform .1",
        ".MC 2.0j uniform .1",
        ".MC {2+1j} uniform .1",
        ".MC 2 uniform .1 SEED 37j",
        ".MC 2 uniform .1 START 1j",
    ] {
        assert!(Netlist::parse(&deck(card)).is_err(), "{card}");
    }
    let netlist = Netlist::parse(&deck(".MC {1+1} uniform .1 SEED 18446744073709551615")).unwrap();
    let AnalysisCommand::MonteCarlo(command) = &netlist.analyses[0] else {
        panic!("MC")
    };
    assert_eq!(command.runs, 2);
    assert_eq!(command.seed, Some(u64::MAX));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn scoped_resumption_keeps_complex_values_until_numeric_admission() {
    for template in [
        ".subckt child p\n.AC LIN 3 1 stop\n.param stop={later}\n.param later=VALUE\n.ends",
        ".subckt parent p\n.subckt child q\n.TRAN 1u 1m {stop}\n.ends\n.param stop={later}\n.param later=VALUE\n.ends",
    ] {
        Netlist::parse(&deck(&template.replace("VALUE", "1u"))).unwrap();
        let source = deck(&template.replace("VALUE", "{1u+1j}"));
        assert!(Netlist::parse(&source).is_err(), "{source}");
    }
}
