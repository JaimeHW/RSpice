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

#[test]
fn model_safety_retains_vbic_aliases_and_actual_native_bjt_family() {
    use rspice_core::circuit::BjtModelSafetyFamily;
    let deck = "BJT rating metadata\nQG c b 0 GP\nQI c b 0 s inferred\nQV c b 0 s VB\n\
        .model GP LPNP VBE_MAX=1 RCX=2\n\
        .model inferred NPN RCI=1 BVBE=2 BVBC=3 BVCE=4 BVSUB=5 VSUBFWD=0.4\n\
        .model VB PNP LEVEL=12 VBE_MAX=6 VSUB_MAX=7\n.end\n";
    // Pin the GP family even though RCX normally infers VBIC without LEVEL.
    let deck = deck.replace("GP LPNP", "GP LPNP LEVEL=1");
    let circuit = Engine::new(SimulationConfig::default())
        .build_circuit(&Netlist::parse(&deck).unwrap())
        .unwrap();
    assert_eq!(
        circuit.device_model_safety("QG").unwrap().bjt_family,
        Some(BjtModelSafetyFamily::GummelPoon)
    );
    for device in ["QI", "QV"] {
        assert_eq!(
            circuit.device_model_safety(device).unwrap().bjt_family,
            Some(BjtModelSafetyFamily::Vbic)
        );
    }
    let inferred = circuit.device_model_safety("QI").unwrap();
    assert!(!inferred.parameters.contains_key("LEVEL"));
    assert!(!inferred.parameters.contains_key("RCI"));
    for (key, value) in [
        ("BVBE", 2.0),
        ("BVBC", 3.0),
        ("BVCE", 4.0),
        ("BVSUB", 5.0),
        ("VSUBFWD", 0.4),
    ] {
        assert_eq!(
            inferred.parameters.get(key),
            Some(&ModelSafetyValue::Numeric(value))
        );
    }
}

#[test]
fn model_safety_follows_selected_native_mos_family_and_polarity() {
    use rspice_core::{
        circuit::{MosModelSafety, MosModelSafetyFamily::*},
        engine::SpiceDialect::{BestAvailable, Ngspice, Xyce},
    };
    for (dialect, model, expected) in [
        (BestAvailable, "NMOS LEVEL=9 DVT0=2.2", Some((Bsim3, false))),
        (BestAvailable, "PMOS LEVEL=9 DVT0=2.2", Some((Bsim3, true))),
        (BestAvailable, "NMOS LEVEL=9 VTO=0.4 KP=1m", None),
        (Ngspice, "NMOS LEVEL=9 DVT0=2.2", None),
        (Xyce, "NMOS LEVEL=9", Some((Bsim3, false))),
        (BestAvailable, "NMOS LEVEL=8", Some((Bsim3, false))),
        (BestAvailable, "PMOS LEVEL=49", Some((Bsim3, true))),
        (BestAvailable, "NMOS LEVEL=14", Some((Bsim4, false))),
        (BestAvailable, "PMOS LEVEL=54", Some((Bsim4, true))),
        (BestAvailable, "NMOS LEVEL=18", Some((Vdmos, false))),
        (BestAvailable, "PMOS LEVEL=18", Some((Vdmos, true))),
        (BestAvailable, "VDMOS PCHAN=1", Some((Vdmos, true))),
        (BestAvailable, "NVDMOS PCHAN=1", Some((Vdmos, false))),
        (BestAvailable, "PVDMOS PCHAN=0", Some((Vdmos, true))),
    ] {
        let deck = format!(
            "MOS rating routing\nM1 d g 0 0 MM W=10u L=1u\n.model MM {model} VGS_MAX=1 VGSR_MAX=0.5\n.end\n"
        );
        let circuit = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            ..Default::default()
        })
        .build_circuit(&Netlist::parse(&deck).unwrap())
        .unwrap_or_else(|error| panic!("{dialect:?}: {model}: {error}"));
        let card = circuit.device_model_safety("M1").unwrap();
        assert_eq!(
            card.mos,
            expected.map(|(family, p_channel)| MosModelSafety { family, p_channel }),
            "{dialect:?}: {model}"
        );
        assert!(!card.generated);
        assert_eq!(
            card.parameters.get("VGS_MAX"),
            Some(&ModelSafetyValue::Numeric(1.0))
        );
    }
}
