use rspice_core::{Engine, Netlist};

#[test]
fn scalar_model_nominal_temperature_is_not_folded_during_parsing() {
    for declarations in [
        ".MODEL device D(IS={TNOM*1u} TNOM=57)",
        ".MODEL device D(TNOM=57 IS={TNOM*1u})",
    ] {
        let netlist = Netlist::parse(&format!(
            "* nominal model temperature\n{declarations}\n.END\n"
        ))
        .unwrap();
        let model = &netlist.models[0];
        assert!(
            !model
                .params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("IS"))
        );
        assert!(
            model
                .expr_params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("IS"))
        );
    }
}

#[test]
fn vector_model_temperature_is_not_frozen_at_the_parser_default() {
    for declarations in [
        ".MODEL lut pwl(X_ARRAY=[0 1] Y_ARRAY=[0 {TEMP}])\n.TEMP 47",
        ".TEMP 47\n.MODEL lut pwl(X_ARRAY=[0 1] Y_ARRAY=[0 {TEMP}])",
    ] {
        let netlist = Netlist::parse(&format!(
            "* vector model temperature\nV1 input 0 .2\nA1 input out lut\n{declarations}\n.END\n"
        ))
        .unwrap();
        assert!(
            netlist.models[0]
                .real_vector_expr_params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("Y_ARRAY"))
        );
        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let index = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!(
            (result.node_voltages[index] - 9.4).abs() < 1e-9,
            "{declarations}: {:?}",
            result.node_voltages
        );
    }
}
