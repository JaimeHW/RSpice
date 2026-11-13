use rspice_core::Netlist;

#[test]
fn static_model_retries_do_not_consume_speculative_samples() {
    let mut reference = rspice_core::netlist::ParamContext::new();
    reference.set_random_seed(37);
    let expected = rspice_core::netlist::expr::eval_expression("aunif(100,1)", &reference).unwrap();
    for fields in [
        "RSH={aunif(100,1)+0*DEFW} DEFW={later}",
        "DEFW={later} RSH={aunif(100,1)+0*DEFW}",
    ] {
        let source = format!(
            "* static model samples\n.OPTIONS SEED=37\n.MODEL rm R({fields})\n.PARAM later=2\n.END\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        let value = netlist.models[0]
            .params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("RSH"))
            .unwrap()
            .1;
        assert_eq!(value, expected, "{fields}");
        let mut next = rspice_core::netlist::ParamContext::new();
        next.set_random_seed(37);
        rspice_core::netlist::expr::eval_expression("aunif(100,1)", &next).unwrap();
        assert_eq!(
            rspice_core::netlist::expr::eval_expression("aunif(100,1)", &netlist.params).unwrap(),
            rspice_core::netlist::expr::eval_expression("aunif(100,1)", &next).unwrap(),
            "{fields}"
        );
    }
}

#[test]
fn forward_function_probes_do_not_consume_model_samples() {
    let source = "* forward function\n.OPTIONS SEED=37\n.MODEL rm R(RSH={aunif(100,1)+0*later()})\n.FUNC later() {2}\n.END\n";
    let netlist = Netlist::parse(source).unwrap();
    let value = netlist.models[0]
        .params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("RSH"))
        .unwrap()
        .1;
    let mut reference = rspice_core::netlist::ParamContext::new();
    reference.set_random_seed(37);
    assert_eq!(
        value,
        rspice_core::netlist::expr::eval_expression("aunif(100,1)", &reference).unwrap()
    );
}
