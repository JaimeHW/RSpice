use rspice_core::{
    Netlist,
    circuit::ModelSafetyValue,
    engine::{Engine, SimulationConfig},
};

#[test]
fn model_safety_follows_selected_bins_and_scoped_parameter_elaboration() {
    let deck = "Selected model ratings\nVd d 0 1\nVg g 0 0.5\n\
        M1 d g 0 0 NM W=10u L=1u\nM2 d g 0 0 NM W=10u L=3u\n\
        .model NM.1 NMOS LEVEL=54 LMIN=0.5u LMAX=2u VTH0=0.4 VGS_MAX=1.1\n\
        .model NM.2 NMOS LEVEL=54 LMIN=2u LMAX=5u VTH0=0.4 VGS_MAX=2.2\n\
        .subckt CELL a params: rating=3\n.model DM D BV_MAX={rating}\nD1 a 0 DM\n.ends\n\
        X1 d CELL rating=4\nX2 d CELL rating=7\nD0 d 0 plain\n.model plain D IS=1e-14\n.end\n";
    let netlist = Netlist::parse(deck).unwrap();
    let circuit = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .unwrap();
    for (device, model, value) in [("m1", "NM.1", 1.1), ("m2", "NM.2", 2.2)] {
        let card = circuit.device_model_safety(device).unwrap();
        assert!(card.model_name.eq_ignore_ascii_case(model));
        assert_eq!(
            card.parameters.get("VGS_MAX"),
            Some(&ModelSafetyValue::Numeric(value))
        );
        assert!(!card.parameters.contains_key("VTH0"));
        assert!(!card.generated);
    }
    for (device, value) in [("X1.D1", 4.0), ("X2.D1", 7.0)] {
        let card = circuit.device_model_safety(device).unwrap();
        assert_eq!(
            card.parameters.get("BV_MAX"),
            Some(&ModelSafetyValue::Numeric(value))
        );
        assert!(!card.model_name.eq_ignore_ascii_case("DM"));
    }
    assert!(circuit.device_model_safety("D0").is_none());
}
