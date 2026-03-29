use super::*;

fn closest_pole_real(poles: &[crate::analysis::pole_zero::Complex], expected: Value) -> Value {
    poles
        .iter()
        .min_by(|a, b| {
            (a.re - expected)
                .abs()
                .partial_cmp(&(b.re - expected).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|p| p.re)
        .expect("expected at least one pole")
}

fn vbic_ceamp_netlist(with_td: bool) -> crate::Netlist {
    let deck_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/vbic/CEamp.cir");
    let source = std::fs::read_to_string(deck_path).expect("read CEamp deck");
    let deck = if with_td {
        source
    } else {
        source.replace(" TD=2e-11", " TD=0")
    };
    crate::Netlist::parse(&deck).expect("parse CEamp deck")
}

fn vbic_ceamp_ports(engine: &Engine, netlist: &crate::Netlist) -> (usize, usize) {
    let circuit = engine
        .build_circuit(netlist)
        .expect("CEamp circuit should build");
    let input_pos = circuit
        .get_node_by_name("1")
        .expect("CEamp input node 1 should resolve");
    let output_pos = circuit
        .get_node_by_name("4")
        .expect("CEamp output node 4 should resolve");
    (input_pos, output_pos)
}

#[test]
fn test_run_pz_parallel_rl_includes_inductor_dynamics() {
    let netlist = crate::netlist::parse_netlist("* Parallel RL\nR1 out 0 1k\nL1 out 0 1m\n.end\n")
        .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_pz(&netlist, 1, 1)
        .expect("PZ analysis should succeed");

    let expected = -1e6; // -R/L
    let closest = closest_pole_real(&result.poles, expected);
    assert!(
        (closest - expected).abs() < 2e4,
        "expected pole near {}, got {}",
        expected,
        closest
    );
}

#[test]
fn test_run_pz_rc_with_ideal_source_still_has_rc_pole() {
    let netlist = crate::netlist::parse_netlist(
        "* RC with source\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_pz(&netlist, 1, 2)
        .expect("PZ analysis should succeed");

    let expected = -1e6; // -1/(RC)
    let closest = closest_pole_real(&result.poles, expected);
    assert!(
        (closest - expected).abs() < 2e4,
        "expected pole near {}, got {}",
        expected,
        closest
    );
    assert!(result.dc_gain.is_finite());
}

#[test]
fn test_run_pz_ports_supports_differential_references() {
    let netlist = crate::netlist::parse_netlist(
        "* Diff PZ\nR1 in out 1k\nR2 out ref 500\nC1 out ref 1n\nR3 ref 0 1k\n.end\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let diff = engine
        .run_pz_ports(&netlist, 1, Some(3), 2, Some(3), true, true, true)
        .expect("differential PZ should succeed");

    let h11 = engine
        .run_pz(&netlist, 1, 2)
        .expect("h11 should succeed")
        .dc_gain;
    let h12 = engine
        .run_pz(&netlist, 3, 2)
        .expect("h12 should succeed")
        .dc_gain;
    let h21 = engine
        .run_pz(&netlist, 1, 3)
        .expect("h21 should succeed")
        .dc_gain;
    let h22 = engine
        .run_pz(&netlist, 3, 3)
        .expect("h22 should succeed")
        .dc_gain;
    let expected = h11 - h12 - h21 + h22;

    assert!((diff.dc_gain - expected).abs() < 1e-9);
    assert!(!diff.poles.is_empty());
}

#[test]
fn test_run_pz_ports_respects_analysis_mode_flags() {
    let netlist = crate::netlist::parse_netlist("* RC\nR1 in out 1k\nC1 out 0 1n\n.end\n")
        .expect("netlist should parse");
    let engine = Engine::default();

    let poles_only = engine
        .run_pz_ports(&netlist, 1, None, 2, None, true, true, false)
        .expect("poles-only PZ should succeed");
    assert!(!poles_only.poles.is_empty());
    assert!(poles_only.zeros.is_empty());

    let zeros_only = engine
        .run_pz_ports(&netlist, 1, None, 2, None, true, false, true)
        .expect("zeros-only PZ should succeed");
    assert!(zeros_only.poles.is_empty());
}

#[test]
fn test_run_pz_ports_voltage_mode_dc_gain() {
    let netlist = crate::netlist::parse_netlist("* Divider\nR1 in out 1k\nR2 out 0 1k\n.end\n")
        .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_pz_ports(&netlist, 1, None, 2, None, false, false, false)
        .expect("voltage-mode PZ should succeed");

    assert!((result.dc_gain - 0.5).abs() < 1e-9);
}

#[test]
fn test_run_pz_ports_voltage_mode_highpass_zero() {
    let netlist = crate::netlist::parse_netlist("* High-pass\nC1 in out 1n\nR1 out 0 1k\n.end\n")
        .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_pz_ports(&netlist, 1, None, 2, None, false, false, true)
        .expect("voltage-mode zero analysis should succeed");

    assert!(
        result.zeros.iter().any(|z| z.magnitude() < 1e-2),
        "expected zero near origin, got {:?}",
        result.zeros
    );
}

#[test]
fn test_run_pz_ports_voltage_mode_unity_transfer_has_no_zeros() {
    let netlist = crate::netlist::parse_netlist("* Any circuit\nR1 in out 1k\nC1 out 0 1n\n.end\n")
        .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_pz_ports(&netlist, 1, None, 1, None, false, false, true)
        .expect("voltage-mode zero analysis should succeed");

    assert!(
        result.zeros.is_empty(),
        "expected no zeros for unity transfer, got {:?}",
        result.zeros
    );
}

#[test]
fn test_run_pz_includes_nonlinear_small_signal_jacobian() {
    let netlist = crate::netlist::parse_netlist(
        "* Diode-loaded pole\nVDD vdd 0 1\nR1 vdd out 1k\nD1 out 0 dmod\nC1 out 0 1n\n.model dmod d(is=1e-14)\n.end\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_pz_ports(&netlist, 2, None, 2, None, true, true, false)
        .expect("PZ analysis should succeed");
    let dominant = result
        .dominant_pole()
        .expect("expected a dominant pole for the diode-loaded node");

    assert!(
        dominant.re < -1.2e6,
        "nonlinear diode conductance should shift the pole beyond the passive -1e6 RC estimate, got {}",
        dominant.re
    );
}

#[test]
fn test_run_pz_vbic_ceamp_with_td_matches_reference_root_order() {
    let netlist = vbic_ceamp_netlist(true);
    let engine = Engine::default();
    let (input_pos, output_pos) = vbic_ceamp_ports(&engine, &netlist);
    let result = engine
        .run_pz_ports(
            &netlist, input_pos, None, output_pos, None, true, true, true,
        )
        .expect("VBIC CEamp PZ should succeed");

    assert_eq!(result.poles.len(), 5, "expected five finite CEamp poles");
    assert_eq!(result.zeros.len(), 4, "expected four finite CEamp zeros");
    assert!(
        result
            .poles
            .iter()
            .all(|p| p.re.is_finite() && p.im.is_finite())
    );
    assert!(
        result
            .zeros
            .iter()
            .all(|z| z.re.is_finite() && z.im.is_finite())
    );
    assert!(
        result.poles.iter().all(|p| p.re < 0.0),
        "expected stable CEamp poles, got {:?}",
        result.poles
    );
}

#[test]
fn test_run_pz_vbic_ceamp_td_expands_dynamic_order_vs_td_zero() {
    let with_td = vbic_ceamp_netlist(true);
    let without_td = vbic_ceamp_netlist(false);
    let engine = Engine::default();
    let (input_pos_with_td, output_pos_with_td) = vbic_ceamp_ports(&engine, &with_td);
    let (input_pos_without_td, output_pos_without_td) = vbic_ceamp_ports(&engine, &without_td);

    let with_td_result = engine
        .run_pz_ports(
            &with_td,
            input_pos_with_td,
            None,
            output_pos_with_td,
            None,
            true,
            true,
            true,
        )
        .expect("VBIC CEamp PZ with TD should succeed");
    let without_td_result = engine
        .run_pz_ports(
            &without_td,
            input_pos_without_td,
            None,
            output_pos_without_td,
            None,
            true,
            true,
            true,
        )
        .expect("VBIC CEamp PZ without TD should succeed");

    assert!(
        with_td_result.poles.len() >= without_td_result.poles.len(),
        "TD-enabled VBIC PZ should not reduce finite pole count"
    );
    assert!(
        with_td_result.zeros.len() >= without_td_result.zeros.len(),
        "TD-enabled VBIC PZ should not reduce finite zero count"
    );
}

#[test]
fn test_run_sensitivity_ac_returns_sweep_sized_results() {
    let netlist = crate::netlist::parse_netlist(
        "* RC low-pass\n.PARAM RVAL=1k\nV1 in 0 AC 1\nR1 in out {RVAL}\nC1 out 0 1n\n.end\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let frequencies = vec![1e3, 1e4, 1e5, 1e6];

    let sens = engine
        .run_sensitivity_ac(&netlist, 2, "RVAL", 1e3, &frequencies, None)
        .expect("AC sensitivity should succeed");

    assert_eq!(sens.len(), frequencies.len());
    assert!(sens.iter().all(|v| v.is_finite()));
}

#[test]
fn test_run_sensitivity_ac_detects_frequency_behavior() {
    let netlist = crate::netlist::parse_netlist(
        "* RC low-pass\n.PARAM RVAL=1k\nV1 in 0 AC 1\nR1 in out {RVAL}\nC1 out 0 1n\n.end\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let frequencies = vec![10.0, 1e6];

    let sens = engine
        .run_sensitivity_ac(&netlist, 2, "RVAL", 1e3, &frequencies, None)
        .expect("AC sensitivity should succeed");

    // Sensitivity should vary with frequency for a reactive transfer function.
    assert!(sens[1].abs() > sens[0].abs() * 1e3);
}

#[test]
fn test_create_perturbed_netlist_rebuilds_only_bound_elements() {
    let netlist = crate::netlist::parse_netlist(
        "* Mixed resistor values\n.PARAM RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.end\n",
    )
    .expect("netlist should parse");

    let (perturbed, rebuilt) = Engine::create_perturbed_netlist(&netlist, "RVAL", 2e3)
        .expect("perturbed netlist should build");
    assert_eq!(rebuilt, 1);

    let mut r1 = None;
    let mut r2 = None;
    for element in &perturbed.elements {
        if let crate::netlist::ElementKind::Resistor { value, .. } = element.kind {
            if element.name.eq_ignore_ascii_case("R1") {
                r1 = Some(value);
            } else if element.name.eq_ignore_ascii_case("R2") {
                r2 = Some(value);
            }
        }
    }

    assert!((r1.expect("R1 should exist") - 2e3).abs() < 1e-9);
    assert!((r2.expect("R2 should exist") - 1e3).abs() < 1e-9);
}

#[test]
fn test_create_perturbed_netlist_preserves_source_relative_model_paths() {
    let temp_dir = std::env::temp_dir().join(format!(
        "rspice_perturbed_model_paths_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let netlist_path = temp_dir.join("deck.cir");
    let stimulus_path = temp_dir.join("stimulus.txt");
    std::fs::write(&stimulus_path, "* digital stimulus placeholder").expect("stimulus file");

    let netlist = Netlist::parse_with_path(
        "\
* Perturbed model path preservation
.PARAM RVAL=1k
.model dsrc d_source (input_file=\"stimulus.txt\")
A1 [out] dsrc
R1 out 0 {RVAL}
.end
",
        &netlist_path,
    )
    .expect("netlist should parse with source path");

    let (perturbed, rebuilt) = Engine::create_perturbed_netlist(&netlist, "RVAL", 2e3)
        .expect("perturbed netlist should build");
    assert_eq!(rebuilt, 1);
    assert_eq!(
        perturbed.source_path.as_deref(),
        Some(netlist_path.as_path())
    );

    let model = perturbed
        .models
        .iter()
        .find(|model| model.name.eq_ignore_ascii_case("dsrc"))
        .expect("model should be preserved");
    let input_file = model
        .string_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("input_file"))
        .map(|(_, value)| value.clone())
        .expect("input_file string param should be preserved");
    assert_eq!(std::path::PathBuf::from(input_file), stimulus_path);
}

#[test]
fn test_run_sensitivity_dc_matches_expected_divider_derivative() {
    let netlist = crate::netlist::parse_netlist(
        "* Divider sensitivity\n.PARAM RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.end\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let sensitivity = engine
        .run_sensitivity(&netlist, 2, "RVAL", 1e3, Some(1.0))
        .expect("DC sensitivity should succeed");

    // Vout = R2/(R1+R2), dVout/dR1 at R1=R2=1k is -1/(4*R) = -2.5e-4.
    assert!((sensitivity + 2.5e-4).abs() < 5e-6);
}

#[test]
fn test_run_sensitivity_supports_subcircuit_param_references() {
    let netlist = crate::netlist::parse_netlist(
        "* Subckt sensitivity\n\
             .PARAM RVAL=1k\n\
             .SUBCKT PASS IN OUT\n\
             R1 IN OUT {RVAL}\n\
             .ENDS PASS\n\
             V1 IN 0 1\n\
             X1 IN OUT PASS\n\
             R2 OUT 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let sensitivity = engine
        .run_sensitivity(&netlist, 2, "RVAL", 1e3, Some(1.0))
        .expect("subcircuit sensitivity should succeed");

    // Vout = R2/(R1+R2), dVout/dR1 at R1=R2=1k is -1/(4*R) = -2.5e-4.
    assert!((sensitivity + 2.5e-4).abs() < 5e-6);
}

#[test]
fn test_run_sensitivity_linearized_reports_element_and_source_derivatives() {
    let netlist = crate::netlist::parse_netlist(
        "* Linearized sensitivity\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let circuit = engine
        .build_circuit(&netlist)
        .expect("circuit should build");
    let out = circuit
        .get_node_by_name("out")
        .expect("output node should resolve");

    let result = engine
        .run_sensitivity_linearized(&netlist, out, None)
        .expect("linearized sensitivity should succeed");

    let sens_v1 = result.get("V1").expect("V1 sensitivity should be present");
    let sens_r1 = result.get("R1").expect("R1 sensitivity should be present");
    let sens_r2 = result.get("R2").expect("R2 sensitivity should be present");

    assert!((result.output_value - 0.5).abs() < 1e-9);
    assert!(
        (sens_v1.absolute - 0.5).abs() < 1e-9,
        "expected dVout/dV1 = 0.5, got {}",
        sens_v1.absolute
    );
    assert!(
        (sens_r1.absolute + 2.5e-4).abs() < 5e-8,
        "expected dVout/dR1 = -2.5e-4, got {}",
        sens_r1.absolute
    );
    assert!(
        (sens_r2.absolute - 2.5e-4).abs() < 5e-8,
        "expected dVout/dR2 = 2.5e-4, got {}",
        sens_r2.absolute
    );
}

#[test]
fn test_run_sensitivity_overrides_redefined_param_cards() {
    let netlist = crate::netlist::parse_netlist(
        "* Redefined param sensitivity\n\
             .PARAM RVAL=2k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             .PARAM RVAL=5k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let sensitivity = engine
        .run_sensitivity(&netlist, 2, "RVAL", 1e3, Some(1.0))
        .expect("redefined parameter sensitivity should succeed");

    // Sensitivity should match evaluation at overridden RVAL=1k.
    assert!((sensitivity + 2.5e-4).abs() < 5e-6);
}

#[test]
fn test_run_step_applies_parameterized_element_values() {
    let netlist = crate::netlist::parse_netlist(
        "* Step divider\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let values = vec![1e3, 2e3, 4e3];

    let results = engine
        .run_step(&netlist, "RVAL", &values)
        .expect("step sweep should succeed");

    assert_eq!(results.len(), values.len());
    for ((value, result), expected_r) in results.iter().zip(values.iter()) {
        assert!((*value - *expected_r).abs() < 1e-12);
        let expected_vout = 1e3 / (expected_r + 1e3);
        assert!((result.voltage(2) - expected_vout).abs() < 1e-6);
    }
}

#[test]
fn test_run_step_supports_subcircuit_parameter_references() {
    let netlist = crate::netlist::parse_netlist(
        "* Step subckt divider\n\
             .PARAM RVAL=1k\n\
             .SUBCKT PASS IN OUT\n\
             R1 IN OUT {RVAL}\n\
             .ENDS PASS\n\
             V1 IN 0 1\n\
             X1 IN OUT PASS\n\
             R2 OUT 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let values = vec![1e3, 2e3, 4e3];

    let results = engine
        .run_step(&netlist, "RVAL", &values)
        .expect("subcircuit step sweep should succeed");

    assert_eq!(results.len(), values.len());
    for ((value, result), expected_r) in results.iter().zip(values.iter()) {
        assert!((*value - *expected_r).abs() < 1e-12);
        let expected_vout = 1e3 / (expected_r + 1e3);
        assert!((result.voltage(2) - expected_vout).abs() < 1e-6);
    }
}

#[test]
fn test_run_step_errors_for_unbound_parameter() {
    let netlist = crate::netlist::parse_netlist(
        "* Unbound parameter step\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let err = engine
        .run_step(&netlist, "RVAL", &[1e3, 2e3])
        .expect_err("unbound step parameter should fail");

    match err {
        SimulationError::Circuit(msg) => assert!(msg.contains("not bound")),
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_step_command_device_resistor_value() {
    let netlist = crate::netlist::parse_netlist(
        "* Step device resistor\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let values = vec![1e3, 2e3, 4e3];
    let step_cmd = StepCommand {
        target: StepTarget::Device,
        name: "R1".to_string(),
        param_name: Some("VALUE".to_string()),
        sweep: crate::netlist::StepSweep::List(values.clone()),
    };

    let results = engine
        .run_step_command(&netlist, &step_cmd, &values)
        .expect("device step should succeed");

    assert_eq!(results.len(), values.len());
    for ((stepped, result), r1) in results.iter().zip(values.iter()) {
        assert!((*stepped - *r1).abs() < 1e-12);
        let expected = 1e3 / (r1 + 1e3);
        assert!((result.voltage(2) - expected).abs() < 1e-6);
    }
}

#[test]
fn test_run_step_command_device_model_resistor_value_override() {
    let netlist = crate::netlist::parse_netlist(
        "* Step device model-based resistor\n\
             V1 in 0 1\n\
             R1 in out RMOD L=10u W=2u\n\
             R2 out 0 1k\n\
             .MODEL RMOD R (RSH=100)\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let values = vec![500.0, 1000.0];
    let step_cmd = StepCommand {
        target: StepTarget::Device,
        name: "R1".to_string(),
        param_name: Some("VALUE".to_string()),
        sweep: crate::netlist::StepSweep::List(values.clone()),
    };

    let results = engine
        .run_step_command(&netlist, &step_cmd, &values)
        .expect("device step should succeed");

    assert_eq!(results.len(), values.len());
    for ((stepped, result), r1) in results.iter().zip(values.iter()) {
        assert!((*stepped - *r1).abs() < 1e-9);
        let expected = 1e3 / (r1 + 1e3);
        assert!((result.voltage(2) - expected).abs() < 1e-6);
    }
}

#[test]
fn test_run_step_command_model_diode_parameter() {
    let netlist = crate::netlist::parse_netlist(
        "* Step model diode\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             D1 out 0 DMOD\n\
             .MODEL DMOD D (IS=1e-12 N=1)\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let values = vec![1e-12, 1e-8];
    let step_cmd = StepCommand {
        target: StepTarget::Model,
        name: "DMOD".to_string(),
        param_name: Some("IS".to_string()),
        sweep: crate::netlist::StepSweep::List(values.clone()),
    };

    let results = engine
        .run_step_command(&netlist, &step_cmd, &values)
        .expect("model step should succeed");

    assert_eq!(results.len(), values.len());
    // Larger IS should reduce forward voltage in this bias setup.
    assert!(results[1].1.voltage(2) < results[0].1.voltage(2));
}

#[test]
fn test_run_step_command_model_requires_param_name() {
    let netlist = crate::netlist::parse_netlist(
        "* Step model missing param\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             D1 out 0 DMOD\n\
             .MODEL DMOD D (IS=1e-12 N=1)\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let step_cmd = StepCommand {
        target: StepTarget::Model,
        name: "DMOD".to_string(),
        param_name: None,
        sweep: crate::netlist::StepSweep::List(vec![1e-12, 1e-10]),
    };

    let err = engine
        .run_step_command(&netlist, &step_cmd, &[1e-12, 1e-10])
        .expect_err("missing model parameter should fail");
    match err {
        SimulationError::Circuit(msg) => {
            assert!(msg.contains("requires an explicit parameter"))
        }
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_step_command_device_rejects_unsupported_parameter() {
    let netlist = crate::netlist::parse_netlist(
        "* Step device unsupported parameter\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let step_cmd = StepCommand {
        target: StepTarget::Device,
        name: "R1".to_string(),
        param_name: Some("FOO".to_string()),
        sweep: crate::netlist::StepSweep::List(vec![1e3]),
    };

    let err = engine
        .run_step_command(&netlist, &step_cmd, &[1e3])
        .expect_err("unsupported device parameter should fail");
    match err {
        SimulationError::Circuit(msg) => {
            assert!(msg.contains("Unsupported resistor step parameter"))
        }
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_apply_device_step_value_transmission_line_parameters() {
    let mut kind = ElementKind::TransmissionLine {
        z0: Some(50.0),
        td: Some(1e-9),
        freq: None,
        nl: None,
        model: Some("LLINE".to_string()),
    };

    Engine::apply_device_step_value(&mut kind, Some("TD"), 2e-9).expect("TD step should succeed");
    Engine::apply_device_step_value(&mut kind, Some("FREQ"), 1e9)
        .expect("FREQ step should succeed");
    Engine::apply_device_step_value(&mut kind, Some("NL"), 0.25).expect("NL step should succeed");
    Engine::apply_device_step_value(&mut kind, None, 75.0)
        .expect("default tline step should map to Z0");

    match kind {
        ElementKind::TransmissionLine {
            z0,
            td,
            freq,
            nl,
            model,
        } => {
            assert_eq!(z0, Some(75.0));
            assert_eq!(td, Some(2e-9));
            assert_eq!(freq, Some(1e9));
            assert_eq!(nl, Some(0.25));
            assert_eq!(model.as_deref(), Some("LLINE"));
        }
        other => panic!("unexpected element kind: {:?}", other),
    }
}

#[test]
fn test_run_noise_ports_ground_reference_matches_single_ended_api() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise API equivalence\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let frequencies = vec![1.0, 1e3, 1e6];

    let single = engine
        .run_noise(&netlist, 2, &frequencies, 300.0)
        .expect("single-ended noise should succeed");
    let diff_ground = engine
        .run_noise_ports(&netlist, 2, Some(0), &frequencies, 300.0)
        .expect("ground-referenced differential noise should succeed");

    assert_eq!(single.len(), diff_ground.len());
    for (s, d) in single.iter().zip(diff_ground.iter()) {
        let tol = 1e-24 + s.output_noise_density.abs() * 1e-12;
        assert!(
            (s.output_noise_density - d.output_noise_density).abs() <= tol,
            "expected equivalent densities at {} Hz: single={}, diff={}",
            s.frequency,
            s.output_noise_density,
            d.output_noise_density
        );
    }
}

#[test]
fn test_run_noise_ports_is_symmetric_for_differential_measurement() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise differential symmetry\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let frequencies = vec![10.0, 1e4];

    let out_minus_in = engine
        .run_noise_ports(&netlist, 2, Some(1), &frequencies, 300.0)
        .expect("V(out,in) noise should succeed");
    let in_minus_out = engine
        .run_noise_ports(&netlist, 1, Some(2), &frequencies, 300.0)
        .expect("V(in,out) noise should succeed");

    assert_eq!(out_minus_in.len(), in_minus_out.len());
    for (a, b) in out_minus_in.iter().zip(in_minus_out.iter()) {
        let tol = 1e-24 + a.output_noise_density.abs() * 1e-12;
        assert!(
            (a.output_noise_density - b.output_noise_density).abs() <= tol,
            "expected symmetric differential noise at {} Hz: a={}, b={}",
            a.frequency,
            a.output_noise_density,
            b.output_noise_density
        );
    }
}

#[test]
fn test_run_noise_ports_rejects_identical_output_nodes() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise invalid output\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let err = engine
        .run_noise_ports(&netlist, 2, Some(2), &[1e3], 300.0)
        .expect_err("identical output nodes should be rejected");

    match err {
        SimulationError::Circuit(msg) => assert!(msg.contains("cannot be the same")),
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_noise_with_input_source_computes_divider_referred_density() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise input-referred divider\n\
             V1 in 0 DC 1 AC 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let results = engine
        .run_noise_with_input_source(&netlist, 2, None, "V1", &[1e3], 300.0)
        .expect("noise with input source should succeed");

    let r = &results[0];
    // Divider gain is 0.5 -> input-referred should be 4x output-referred.
    let ratio = r.input_referred_density / r.output_noise_density;
    assert!(
        (ratio - 4.0).abs() < 1e-3,
        "expected ratio ~4, got {}",
        ratio
    );
}

#[test]
fn test_run_noise_with_current_input_source_uses_transimpedance() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise current-input transimpedance\n\
             I1 in 0 DC 1 AC 1\n\
             R1 in 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let results = engine
        .run_noise_with_input_source(&netlist, 1, None, "I1", &[1e3], 300.0)
        .expect("noise with current input source should succeed");

    let r = &results[0];
    // |V/I| = R = 1k -> gain^2 = 1e6, so input-referred = output/1e6.
    let ratio = r.input_referred_density / r.output_noise_density;
    assert!(
        (ratio - 1e-6).abs() < 1e-9,
        "expected transimpedance-referred ratio ~1e-6, got {}",
        ratio
    );
}

#[test]
fn test_run_noise_with_input_source_rejects_unknown_source() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise unknown input source\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let err = engine
        .run_noise_with_input_source(&netlist, 2, None, "VMISS", &[1e3], 300.0)
        .expect_err("unknown noise input source should fail");
    match err {
        SimulationError::Circuit(msg) => assert!(msg.contains("not found")),
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_noise_with_input_source_rejects_zero_transfer_gain() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise input source zero gain\n\
             V1 in 0 DC 1 AC 1\n\
             C1 in out 1u\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let err = engine
        .run_noise_with_input_source(&netlist, 2, None, "V1", &[0.0], 300.0)
        .expect_err("undefined input-referred gain should fail");

    match err {
        SimulationError::Circuit(msg) => assert!(msg.contains("undefined")),
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_noise_ports_rejects_invalid_output_node() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise invalid node\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let err = engine
        .run_noise_ports(&netlist, 99, None, &[1e3], 300.0)
        .expect_err("invalid output node should be rejected");

    match err {
        SimulationError::Circuit(msg) => {
            assert!(msg.contains("Invalid node for noise analysis"))
        }
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_noise_contributions_use_resistor_instance_names() {
    let netlist = crate::netlist::parse_netlist(
        "* Noise resistor names\n\
             V1 in 0 1\n\
             RFEED in out 1k\n\
             RLOAD out 0 2k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();
    let results = engine
        .run_noise(&netlist, 2, &[1e3], 300.0)
        .expect("noise analysis should succeed");
    let first = results
        .first()
        .expect("noise result should contain one point");
    let names: std::collections::HashSet<&str> = first
        .contributions
        .iter()
        .map(|contrib| contrib.device_name.as_str())
        .collect();
    assert!(names.contains("RFEED"));
    assert!(names.contains("RLOAD"));
}

#[test]
fn test_run_noise_collects_bjt_shot_and_flicker_sources() {
    let netlist = crate::netlist::parse_netlist(
        "* BJT noise\n\
             VCC vcc 0 5\n\
             VB base 0 0.6\n\
             RC vcc out 1k\n\
             Q1 out base 0 QB\n\
             .MODEL QB NPN (IS=1e-14 BF=200 BR=1 NF=1 NR=1 VAF=1e12 CJE=0 CJC=0 TF=0 TR=0 IKF=1e9 IKR=1e9 KF=1e-14 AF=1.2 EF=1.1)\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_noise(&netlist, 3, &[1e3], 300.0)
        .expect("BJT noise analysis should succeed");
    let contribs = &result[0].contributions;

    assert!(
        contribs.iter().any(|c| c.device_name == "Q1:IC"),
        "expected collector shot noise contribution"
    );
    assert!(
        contribs.iter().any(|c| c.device_name == "Q1:flicker"),
        "expected BJT flicker noise contribution"
    );
}

#[test]
fn test_run_noise_collects_mosfet_thermal_and_flicker_sources() {
    let netlist = crate::netlist::parse_netlist(
        "* MOS noise\n\
             VDD vdd 0 5\n\
             VG gate 0 2\n\
             RD vdd out 1k\n\
             M1 out gate 0 0 NM1 W=20u L=1u\n\
             .MODEL NM1 NMOS (KP=200u VTO=0.7 KF=1e-24 AF=1.1 EF=1.3)\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_noise(&netlist, 3, &[1e3], 300.0)
        .expect("MOS noise analysis should succeed");
    let contribs = &result[0].contributions;

    assert!(
        contribs.iter().any(|c| c.device_name == "M1:thermal"),
        "expected MOS thermal noise contribution"
    );
    assert!(
        contribs.iter().any(|c| c.device_name == "M1:flicker"),
        "expected MOS flicker noise contribution"
    );
}

#[test]
fn test_run_noise_collects_jfet_noise_sources() {
    let netlist = crate::netlist::parse_netlist(
        "* JFET noise\n\
             VDD vdd 0 5\n\
             VG gate 0 -1\n\
             RD vdd out 1k\n\
             J1 out gate 0 JN\n\
             .MODEL JN NJF (VTO=-2 BETA=1e-3 KF=1e-18 AF=1.0 EF=1.0)\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_noise(&netlist, 3, &[1e3], 300.0)
        .expect("JFET noise analysis should succeed");
    let contribs = &result[0].contributions;

    assert!(
        contribs.iter().any(|c| c.device_name == "J1:thermal"),
        "expected JFET thermal noise contribution"
    );
    assert!(
        contribs.iter().any(|c| c.device_name == "J1:flicker"),
        "expected JFET flicker noise contribution"
    );
}

#[test]
fn test_run_monte_carlo_applies_parameter_variation() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo divider\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_monte_carlo(&netlist, 128, 12345)
        .expect("monte carlo should succeed");

    assert_eq!(result.num_runs, 128);
    let out = result.variables.get("V(2)").expect("V(2) statistics");
    assert!(out.std_dev > 0.0, "expected non-zero variation at output");
    assert!(out.mean > 0.45 && out.mean < 0.55, "unexpected output mean");
}

#[test]
fn test_run_monte_carlo_supports_uniform_distribution() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo uniform\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_monte_carlo_with_options(
            &netlist,
            256,
            12,
            Distribution::Uniform { tolerance: 0.05 },
            None,
        )
        .expect("uniform monte carlo should succeed");

    let out = result.variables.get("V(2)").expect("V(2) statistics");
    let min_expected = 1e3 / (1e3 * 1.05 + 1e3);
    let max_expected = 1e3 / (1e3 * 0.95 + 1e3);
    assert!(out.std_dev > 0.0);
    assert!(
        out.min >= min_expected - 1e-6,
        "uniform min outside expected range"
    );
    assert!(
        out.max <= max_expected + 1e-6,
        "uniform max outside expected range"
    );
}

#[test]
fn test_run_monte_carlo_supports_worst_case_distribution() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo worst case\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_monte_carlo_with_options(
            &netlist,
            256,
            21,
            Distribution::WorstCase { tolerance: 0.05 },
            None,
        )
        .expect("worst-case monte carlo should succeed");

    let out = result.variables.get("V(2)").expect("V(2) statistics");
    let min_expected = 1e3 / (1e3 * 1.05 + 1e3);
    let max_expected = 1e3 / (1e3 * 0.95 + 1e3);
    assert!(out.std_dev > 0.0);
    assert!(
        out.min >= min_expected - 1e-6,
        "worst-case min outside expected range"
    );
    assert!(
        out.max <= max_expected + 1e-6,
        "worst-case max outside expected range"
    );
}

#[test]
fn test_run_monte_carlo_parameter_filter_is_respected() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo parameter filter\n\
             .PARAM RMAIN=1k RISO=1k\n\
             V1 in 0 1\n\
             R1 in out {RMAIN}\n\
             R2 out 0 1k\n\
             V2 aux 0 1\n\
             R3 aux aux2 {RISO}\n\
             R4 aux2 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let filter_iso = vec!["RISO".to_string()];
    let result_iso = engine
        .run_monte_carlo_with_options(
            &netlist,
            128,
            5,
            Distribution::Gaussian { sigma: 0.02 },
            Some(&filter_iso),
        )
        .expect("filtered monte carlo should succeed");
    let out_iso = result_iso.variables.get("V(2)").expect("V(2) stats");
    assert!(
        out_iso.std_dev < 1e-12,
        "output should not vary when only isolated parameter is varied"
    );

    let filter_main = vec!["RMAIN".to_string()];
    let result_main = engine
        .run_monte_carlo_with_options(
            &netlist,
            128,
            5,
            Distribution::Gaussian { sigma: 0.02 },
            Some(&filter_main),
        )
        .expect("filtered monte carlo should succeed");
    let out_main = result_main.variables.get("V(2)").expect("V(2) stats");
    assert!(out_main.std_dev > 0.0);
}

#[test]
fn test_run_monte_carlo_with_options_errors_for_unknown_filtered_parameter() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo unknown parameter\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let filter = vec!["MISSING".to_string()];
    let err = engine
        .run_monte_carlo_with_options(
            &netlist,
            32,
            9,
            Distribution::Gaussian { sigma: 0.01 },
            Some(&filter),
        )
        .expect_err("unknown filtered parameter should fail");
    match err {
        SimulationError::Circuit(msg) => assert!(msg.contains("not defined")),
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_monte_carlo_with_options_errors_for_unbound_filtered_parameter() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo unbound filtered parameter\n\
             .PARAM RVAL=1k RUNUSED=2k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let filter = vec!["RUNUSED".to_string()];
    let err = engine
        .run_monte_carlo_with_options(
            &netlist,
            32,
            9,
            Distribution::Gaussian { sigma: 0.01 },
            Some(&filter),
        )
        .expect_err("unbound filtered parameter should fail");
    match err {
        SimulationError::Circuit(msg) => assert!(msg.contains("not bound")),
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_monte_carlo_with_options_rejects_negative_spread() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo invalid spread\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let err = engine
        .run_monte_carlo_with_options(
            &netlist,
            16,
            1,
            Distribution::Gaussian { sigma: -0.5 },
            None,
        )
        .expect_err("negative spread should fail");
    match err {
        SimulationError::Circuit(msg) => assert!(msg.contains("non-negative")),
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_monte_carlo_reports_non_ground_node_indices() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo node indexing\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_monte_carlo(&netlist, 64, 321)
        .expect("monte carlo should succeed");

    assert!(
        !result.variables.contains_key("V(0)"),
        "ground should not be reported as a Monte Carlo variable"
    );
    assert!(
        !result.variables.contains_key("V(3)"),
        "unexpected extra node statistic indicates indexing mismatch"
    );

    let vin = result.variables.get("V(1)").expect("V(1) statistics");
    let vout = result.variables.get("V(2)").expect("V(2) statistics");

    assert!((vin.mean - 1.0).abs() < 1e-12);
    assert!(
        vin.std_dev < 1e-12,
        "ideal source node should not vary across Monte Carlo samples"
    );
    assert!(vout.std_dev > 0.0, "output node should vary with RVAL");
}

#[test]
fn test_run_monte_carlo_reports_named_node_aliases() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo named aliases\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_monte_carlo(&netlist, 64, 111)
        .expect("monte carlo should succeed");

    let find_case_insensitive = |target: &str| {
        result
            .variables
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(target))
            .map(|(_, stats)| stats)
            .expect("named alias should exist")
    };

    let vin_num = result.variables.get("V(1)").expect("numeric Vin stats");
    let vout_num = result.variables.get("V(2)").expect("numeric Vout stats");
    let vin_named = find_case_insensitive("V(in)");
    let vout_named = find_case_insensitive("V(out)");

    assert!((vin_num.mean - vin_named.mean).abs() < 1e-15);
    assert!((vin_num.std_dev - vin_named.std_dev).abs() < 1e-15);
    assert!((vout_num.mean - vout_named.mean).abs() < 1e-15);
    assert!((vout_num.std_dev - vout_named.std_dev).abs() < 1e-15);
}

#[test]
fn test_run_monte_carlo_reports_all_nodes_beyond_ten() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo long ladder\n\
             .PARAM RVAR=1k\n\
             V1 n1 0 1\n\
             R1 n1 n2 {RVAR}\n\
             R2 n2 n3 1k\n\
             R3 n3 n4 1k\n\
             R4 n4 n5 1k\n\
             R5 n5 n6 1k\n\
             R6 n6 n7 1k\n\
             R7 n7 n8 1k\n\
             R8 n8 n9 1k\n\
             R9 n9 n10 1k\n\
             R10 n10 n11 1k\n\
             R11 n11 n12 1k\n\
             R12 n12 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_monte_carlo(&netlist, 64, 91)
        .expect("monte carlo should succeed");

    assert!(
        result.variables.contains_key("V(12)"),
        "highest-index node statistic should not be truncated"
    );
    assert!(
        result
            .variables
            .keys()
            .any(|name| name.eq_ignore_ascii_case("V(n12)")),
        "named alias for the last node should be preserved"
    );
}

#[test]
fn test_run_monte_carlo_is_deterministic_for_seed() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo deterministic\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out {RVAL}\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let a = engine
        .run_monte_carlo(&netlist, 64, 77)
        .expect("run A should succeed");
    let b = engine
        .run_monte_carlo(&netlist, 64, 77)
        .expect("run B should succeed");

    let a_out = a.variables.get("V(2)").expect("A V(2) stats");
    let b_out = b.variables.get("V(2)").expect("B V(2) stats");
    assert!((a_out.mean - b_out.mean).abs() < 1e-15);
    assert!((a_out.std_dev - b_out.std_dev).abs() < 1e-15);
}

#[test]
fn test_run_monte_carlo_supports_subcircuit_parameter_references() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo subckt\n\
             .PARAM RVAL=1k\n\
             .SUBCKT PASS IN OUT\n\
             R1 IN OUT {RVAL}\n\
             .ENDS PASS\n\
             V1 IN 0 1\n\
             X1 IN OUT PASS\n\
             R2 OUT 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let result = engine
        .run_monte_carlo(&netlist, 64, 123)
        .expect("subcircuit monte carlo should succeed");
    let out = result.variables.get("V(2)").expect("V(2) statistics");
    assert!(out.std_dev > 0.0);
}

#[test]
fn test_run_monte_carlo_errors_for_unbound_parameter_set() {
    let netlist = crate::netlist::parse_netlist(
        "* Monte Carlo unbound\n\
             .PARAM RVAL=1k\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .END\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let err = engine
        .run_monte_carlo(&netlist, 16, 9)
        .expect_err("unbound monte carlo parameter set should fail");
    match err {
        SimulationError::Circuit(msg) => assert!(msg.contains("not bound")),
        other => panic!("expected Circuit error, got {:?}", other),
    }
}

#[test]
fn test_run_sensitivity_errors_for_unbound_parameter() {
    let netlist = crate::netlist::parse_netlist(
        "* Unbound parameter\n.PARAM RVAL=1k\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n",
    )
    .expect("netlist should parse");
    let engine = Engine::default();

    let err = engine
        .run_sensitivity(&netlist, 2, "RVAL", 1e3, None)
        .expect_err("unbound parameter should fail");

    match err {
        SimulationError::Circuit(msg) => {
            assert!(msg.contains("not bound"));
        }
        other => panic!("expected Circuit error, got {:?}", other),
    }
}
