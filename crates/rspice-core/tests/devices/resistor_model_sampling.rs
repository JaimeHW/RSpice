use rspice_core::{Engine, Netlist};

fn seeded_values(count: usize) -> Vec<f64> {
    let mut context = rspice_core::netlist::ParamContext::new();
    context.set_random_seed(37);
    (0..count)
        .map(|_| rspice_core::netlist::expr::eval_expression("aunif(100,1)", &context).unwrap())
        .collect()
}

#[test]
fn each_resistor_consumes_one_model_sample_in_both_stamp_forms() {
    let expected = seeded_values(2);
    for tolerance in [0, 1000] {
        let source = format!(
            "* instance sampling\n.OPTIONS SEED=37\n.OPTIONS DEVICE ZERORESISTANCETOL={tolerance}\n\
             .MODEL rm R(RSH={{TEMP*0+aunif(100,1)}})\n\
             V1 a 0 1\nV2 b 0 1\nR1 a 0 rm L=1 W=1\nR2 b 0 rm L=1 W=1\n.END\n"
        );
        let result = Engine::default()
            .run_dc_op(&Netlist::parse(&source).unwrap())
            .unwrap();
        for (name, expected) in ["V1", "V2"].into_iter().zip(&expected) {
            let actual = -1.0 / result.branch_current_named(name).unwrap();
            assert!(
                (actual - expected).abs() < 1e-8,
                "branch tolerance {tolerance}, {name}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn flicker_and_electrical_parameters_share_the_same_model_sample() {
    let source = "* correlated model fields\n.OPTIONS SEED=37\n\
                  .MODEL rm R(RSH={TEMP*0+aunif(100,1)} KF={RSH} AF=1)\n\
                  R1 a 0 rm L=1 W=1\n.END\n";
    let circuit = Engine::default()
        .build_circuit(&Netlist::parse(source).unwrap())
        .unwrap();
    let flicker = circuit.resistor_storage().flicker[0].unwrap();
    let coefficient = flicker.coefficient * 2.0_f64.powi(flicker.binary_scale);
    assert_eq!(
        coefficient,
        circuit.resistor_storage().reported_resistances[0]
    );
}

#[test]
fn behavioral_resistor_policy_does_not_discard_a_model_sample() {
    let source = "* behavioral model sampling\n.OPTIONS SEED=37\nV1 out 0 1\n\
                  .MODEL rm R(R={TEMP*0+aunif(100,1)})\n\
                  R1 out 0 R={1+0*V(out)} rm\n.END\n";
    let result = Engine::default()
        .run_dc_op(&Netlist::parse(source).unwrap())
        .unwrap();
    let actual = -1.0 / result.branch_current_named("V1").unwrap();
    let expected = seeded_values(1)[0];
    assert!((actual - expected).abs() < 1e-8, "{actual} != {expected}");
}
