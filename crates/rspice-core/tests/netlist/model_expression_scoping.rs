use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::{Netlist, NetlistParseOptions};

fn parameter(netlist: &Netlist, model: &str, name: &str) -> f64 {
    netlist
        .models
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case(model))
        .unwrap()
        .params
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .unwrap()
        .1
}

#[test]
fn forward_model_resolution_cannot_shadow_another_models_deck_parameters() {
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        for models in [
            ".MODEL first D(IS={later})\n.MODEL second D(N={IS})",
            ".MODEL second D(N={IS})\n.MODEL first D(IS={later})",
        ] {
            for source in [
                format!("* model scopes\n.PARAM IS=2 later=1p\n{models}\n.END\n"),
                format!("* model scopes\n{models}\n.PARAM IS=2 later=1p\n.END\n"),
            ] {
                let netlist = Netlist::parse_with_options(
                    &source,
                    NetlistParseOptions {
                        expression_dialect: dialect,
                        ..Default::default()
                    },
                )
                .unwrap();
                assert_eq!(
                    parameter(&netlist, "second", "N"),
                    2.0,
                    "{dialect:?}: {source}"
                );
                assert_eq!(netlist.params.get("IS"), Some(2.0));
            }
        }
    }
}

#[test]
fn unrelated_model_fields_cannot_make_an_undefined_bare_parameter_valid() {
    let source = "* isolated model bindings\n.MODEL first D(IS={later})\n.MODEL second D(N=IS)\n.PARAM later=1p\n.END\n";
    let error = Netlist::parse(source).expect_err("IS is not a deck parameter");
    assert!(
        error
            .to_string()
            .contains("Expected value for model parameter 'N', found IS"),
        "{error}"
    );
}

#[test]
fn a_bare_reference_resolved_within_its_own_model_stays_valid() {
    let netlist = Netlist::parse("* local model fields\n.MODEL first D(IS={later} N=IS)\n.MODEL second D(IS={other})\n.PARAM later=1p other=2p\n.END\n").unwrap();
    assert_eq!(parameter(&netlist, "first", "N"), 1e-12);
    assert_eq!(parameter(&netlist, "second", "IS"), 2e-12);
}

#[test]
fn an_inherited_model_field_cannot_hide_an_unresolved_replacement() {
    let source =
        "* inherited fields\n.MODEL base D(N=1)\n.MODEL child AKO:base D(N=missing)\n.END\n";
    let error = Netlist::parse(source).expect_err("missing is not a deck parameter");
    assert!(
        error
            .to_string()
            .contains("Expected value for model parameter 'N', found MISSING"),
        "{error}"
    );
}
