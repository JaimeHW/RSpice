//! Code-model poles must include private states and accepted-bias derivatives.
use rspice_core::analysis::pole_zero::StabilityVerdict;
use rspice_core::{Engine, Netlist};

fn transfer(model: &str) -> Netlist {
    Netlist::parse(&format!(
        "Transfer states\nV1 in 0 0\nA1 in out filt\n.model filt {model}\nR1 out 0 1k\n.end\n"
    ))
    .unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_transfer_retains_its_pole_and_hidden_unstable_state() {
    for (numerator, denominator, pole) in [("1", "1 2", -2.0), ("1 -2", "1 -2", 2.0)] {
        let netlist = transfer(&format!(
            "s_xfer(num_coeff=[{numerator}] den_coeff=[{denominator}])"
        ));
        let spectrum = Engine::default().run_pole_spectrum(&netlist).unwrap();
        assert_eq!(spectrum.poles.len(), 1, "{spectrum:?}");
        assert!((spectrum.poles[0].re - pole).abs() < 1e-10);
        assert!(spectrum.poles[0].im.abs() < 1e-10);
        assert!(spectrum.evidence.is_qualified());
        assert_eq!(
            spectrum.stability_verdict(),
            if pole < 0.0 {
                StabilityVerdict::Stable
            } else {
                StabilityVerdict::Unstable
            }
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn arbitrary_frequency_table_cannot_certify_an_empty_spectrum() {
    let error = Engine::default()
        .run_pole_spectrum(&transfer("xfer(table=[1 1 0 10 0.5 -45])"))
        .unwrap_err();
    assert!(error.to_string().contains("analysis.pz.device"), "{error}");
    assert!(error.to_string().contains("XSPICE"), "{error}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_transfer_preserves_order_scaling_and_integrator_modes() {
    for (numerator, denominator, frequency, expected) in [
        ("1", "1 3 2", 1.0, vec![-1.0, -2.0]),
        ("1", "1 2", 1e-6, vec![-2e-6]),
        ("1", "1 2", 1e6, vec![-2e6]),
        ("1", "1 2", -3.0, vec![6.0]),
        ("1", "1 0", 1.0, vec![0.0]),
        ("2", "4", 1.0, vec![]),
        ("0", "1 -2", 1.0, vec![2.0]),
    ] {
        let netlist = transfer(&format!(
            "s_xfer(num_coeff=[{numerator}] den_coeff=[{denominator}] denormalized_freq={frequency})"
        ));
        let spectrum = Engine::default().run_pole_spectrum(&netlist).unwrap();
        assert_eq!(spectrum.poles.len(), expected.len(), "{spectrum:?}");
        for (actual, expected) in spectrum.poles.iter().zip(expected) {
            assert!(
                (actual.re - expected).abs() <= 1e-9 * expected.abs().max(1e-6),
                "{spectrum:?}"
            );
            assert!(actual.im.abs() < 1e-12);
        }
        assert!(spectrum.evidence.is_qualified());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_transfer_pz_preserves_port_polarity_and_feedthrough() {
    for (connections, extra, scale) in [
        ("in out", "", 1.0),
        ("%vd[0 in] out", "", -1.0),
        ("in %vd[0 out]", "", -1.0),
        ("in %id[out 0]", "", -5.0),
        ("%id[sense 0] out", "Rs in sense 2\n", 0.5),
        ("%id[0 sense] %id[out 0]", "Rs in sense 2\n", 2.5),
        ("%i(sense) out", "Rs in sense 2\n", 0.5),
    ] {
        for (numerator, denominator, dc, hf, zeros) in [
            ("1", "1 2", 1.5, 0.0, vec![]),
            ("1 0", "1 2", 0.0, 3.0, vec![0.0]),
            ("2", "4", 1.5, 1.5, vec![]),
        ] {
            let netlist = Netlist::parse(&format!(
                "Port realization\nV1 in 0 dc 0 ac 1\n{extra}A1 {connections} filt\n.model filt s_xfer(gain=3 num_coeff=[{numerator}] den_coeff=[{denominator}])\nRload out 0 5\n.pz in 0 out 0 vol pz\n.end\n"
            )).unwrap();
            let result = Engine::default()
                .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &rspice_core::NoAbort)
                .unwrap_or_else(|error| {
                    panic!("{connections}, {numerator}/{denominator}: {error}")
                });
            assert!(
                (result.dc_gain.unwrap() - dc * scale).abs() < 1e-8,
                "{connections}: {result:?}"
            );
            assert!(
                (result.hf_gain.unwrap() - hf * scale).abs() < 1e-8,
                "{connections}: {result:?}"
            );
            assert_eq!(result.zeros.len(), zeros.len(), "{connections}: {result:?}");
            for (actual, expected) in result.zeros.iter().zip(zeros) {
                assert!((actual.re - expected).abs() < 1e-9 && actual.im.abs() < 1e-9);
            }
            let spectrum = Engine::default().run_pole_spectrum(&netlist).unwrap();
            assert_eq!(result.poles, spectrum.poles);
            assert_eq!(result.pole_evidence, spectrum.evidence);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stb_feedback_moves_the_rational_pole_and_retains_hidden_instability() {
    use rspice_core::analysis::stb::{StbConfig, StbSweepType};
    let config = StbConfig::new()
        .with_probe("VPROBE")
        .with_sweep(1.0, 10.0, 3)
        .with_sweep_type(StbSweepType::Linear);
    for hidden in [false, true] {
        let extra = if hidden {
            "Ahidden 0 hidden unstable\n.model unstable s_xfer(num_coeff=[1 -2] den_coeff=[1 -2])\nRh hidden 0 1\n"
        } else {
            ""
        };
        let netlist = Netlist::parse(&format!(
            "Rational feedback\nA1 ctrl eo filt\n.model filt s_xfer(gain=-10 num_coeff=[1] den_coeff=[1 1])\nVPROBE eo ctrl 0\nR1 ctrl 0 1k\n{extra}.end\n"
        )).unwrap();
        let result = Engine::default().run_stb(&netlist, config.clone()).unwrap();
        let spectrum = result.result.circuit_poles.spectrum().unwrap();
        assert_eq!(spectrum.poles.len(), if hidden { 2 } else { 1 });
        assert!(
            (spectrum.poles.last().unwrap().re + 11.0).abs() < 1e-8,
            "{spectrum:?}"
        );
        assert_eq!(
            result.result.stability_verdict(),
            if hidden {
                StabilityVerdict::Unstable
            } else {
                StabilityVerdict::Stable
            }
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_private_states_are_included_in_resource_limits() {
    use rspice_core::{ResourceKind, SimulationError};
    let netlist = transfer("s_xfer(num_coeff=[1] den_coeff=[1 3 2])");
    let mut config = rspice_core::engine::SimulationConfig::default();
    config.resource_limits.max_matrix_unknowns = 4;
    let error = Engine::new(config).run_pole_spectrum(&netlist).unwrap_err();
    assert!(
        matches!(error, SimulationError::ResourceLimit(error) if error.resource == ResourceKind::MatrixUnknowns && error.requested == 6 && error.limit == 4),
        "{error}"
    );
    let mut config = rspice_core::engine::SimulationConfig::default();
    config.resource_limits.max_result_values = 4 * (8 * 4 + 1);
    let error = Engine::new(config).run_pole_spectrum(&netlist).unwrap_err();
    assert!(
        matches!(error, SimulationError::ResourceLimit(error) if error.resource == ResourceKind::ResultValues && error.requested == 6 * (8 * 6 + 1)),
        "{error}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn static_gain_remains_admitted_and_stb_retains_table_unavailability() {
    use rspice_core::analysis::stb::{
        CircuitPoleEvidence, CircuitPoleFailure, StbConfig, StbSweepType,
    };
    let spectrum = Engine::default()
        .run_pole_spectrum(&transfer("gain(gain=3)"))
        .unwrap();
    assert!(spectrum.poles.is_empty() && spectrum.evidence.is_qualified());
    let netlist = Netlist::parse("Table feedback\nA1 ctrl eo filt\n.model filt xfer(table=[1 -10 0 10 -1 0] r_i=true)\nVPROBE eo ctrl 0\nR1 ctrl 0 1k\n.end\n").unwrap();
    let config = StbConfig::new()
        .with_probe("VPROBE")
        .with_sweep(1.0, 10.0, 3)
        .with_sweep_type(StbSweepType::Linear);
    let result = Engine::default().run_stb(&netlist, config).unwrap();
    assert_eq!(result.loop_gains.len(), 3);
    assert_eq!(
        result.result.stability_verdict(),
        StabilityVerdict::Indeterminate
    );
    assert!(matches!(result.result.circuit_poles,
        CircuitPoleEvidence::Unavailable { cause: CircuitPoleFailure::Unsupported { ref detail, .. } }
        if detail.contains("frequency table")));
}

fn voltage_feedback(model: &str, ports: &str, extra: &str, bias_current: f64) -> Netlist {
    Netlist::parse(&format!(
        "Nonlinear feedback\nA1 {ports} cm\n.model cm {model}\nVPROBE drive source 0\nR1 source x 1\nRshunt x 0 1\nC1 x 0 1\nIBIAS 0 x {bias_current}\n{extra}.pz x 0 x 0 cur pz\n.end\n"
    )).unwrap()
}

fn assert_feedback_pole(netlist: &Netlist, bias: f64, pole: f64) {
    use rspice_core::analysis::stb::{StbConfig, StbSweepType};

    let engine = Engine::default();
    let op = engine.run_dc_op(netlist).unwrap();
    let x = op
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("x"))
        .unwrap();
    assert!((op.node_voltages[x] - bias).abs() < 1e-8, "{op:?}");

    let spectrum = engine.run_pole_spectrum(netlist).unwrap();
    assert!(spectrum.evidence.is_qualified(), "{spectrum:?}");
    assert_eq!(spectrum.poles.len(), 1, "{spectrum:?}");
    assert!((spectrum.poles[0].re - pole).abs() < 1e-7, "{spectrum:?}");
    assert!(spectrum.poles[0].im.abs() < 1e-9, "{spectrum:?}");

    let pz = engine
        .run_pz_from_card_with_abort(netlist, &netlist.analyses[0], &rspice_core::NoAbort)
        .unwrap();
    assert_eq!(pz.poles.len(), 1, "{pz:?}");
    assert!((pz.poles[0] - spectrum.poles[0]).norm() < 1e-8, "{pz:?}");
    assert!(pz.pole_evidence.is_qualified(), "{pz:?}");
    let certificate = spectrum.evidence.certificate().unwrap();
    let pz_certificate = pz.pole_evidence.certificate().unwrap();
    // PZ may take an analytic path with a different residual estimate.
    assert_eq!(pz_certificate.problem_order, certificate.problem_order);
    assert_eq!(pz_certificate.infinite_count, certificate.infinite_count);
    assert!(pz.zeros.is_empty(), "{pz:?}");
    // Every fixture has C=1, so its current-to-voltage transfer is 1/(s-pole).
    assert!((pz.dc_gain.unwrap() + 1.0 / pole).abs() < 1e-8, "{pz:?}");
    assert!(pz.hf_gain.unwrap().abs() < 1e-9, "{pz:?}");

    let config = StbConfig::new()
        .with_probe("VPROBE")
        .with_sweep(0.1, 10.0, 3)
        .with_sweep_type(StbSweepType::Linear);
    let stb = engine.run_stb(netlist, config).unwrap();
    assert_eq!(stb.loop_gains.len(), 3);
    let stb_spectrum = stb.result.circuit_poles.spectrum().unwrap();
    assert_eq!(stb_spectrum.poles.len(), 1, "{stb_spectrum:?}");
    assert!((stb_spectrum.poles[0] - spectrum.poles[0]).norm() < 1e-8);
    assert!(stb_spectrum.evidence.is_qualified(), "{stb_spectrum:?}");
    let stb_certificate = stb_spectrum.evidence.certificate().unwrap();
    assert_eq!(stb_certificate.problem_order, certificate.problem_order);
    assert_eq!(stb_certificate.infinite_count, certificate.infinite_count);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nonlinear_product_and_polynomial_poles_follow_the_solved_bias() {
    // f(x)=-x^2, I=2*b+b^2, and the closed-circuit pole is -2-2*b.
    for (model, ports) in [
        ("mult(out_gain=-1)", "[x x] drive"),
        ("spice2poly(coef=[0 0 -1])", "[x] drive"),
        ("icm_spice2poly(coef=[0 0 -1])", "[x] drive"),
    ] {
        for bias in [1.0, 2.0] {
            assert_feedback_pole(
                &voltage_feedback(model, ports, "", 2.0 * bias + bias * bias),
                bias,
                -2.0 - 2.0 * bias,
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn divider_and_alias_poles_include_numerator_and_denominator_controls() {
    for model in ["divider", "divide"] {
        for (ports, gain, current) in [("x ref drive", -1.0, 5.0), ("ref x drive", 1.0, 3.0)] {
            assert_feedback_pole(
                &voltage_feedback(
                    &format!("{model}(out_gain={gain})"),
                    ports,
                    "VREF ref 0 2\n",
                    current,
                ),
                2.0,
                -2.5,
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn limiter_poles_follow_linear_saturated_and_smoothed_regions() {
    for (bias, current, range, pole) in [
        (2.0, 8.0, 0.0, -4.0),
        (10.0, 30.0, 0.0, -2.0),
        (5.0, 19.95, 0.2, -3.0),
    ] {
        assert_feedback_pole(
            &voltage_feedback(
                &format!(
                    "limit(gain=-2 out_lower_limit=-10 out_upper_limit=10 limit_range={range})"
                ),
                "x drive",
                "",
                current,
            ),
            bias,
            pole,
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn controlled_limiter_poles_include_both_limit_inputs() {
    for (ports, extra, gain, current, pole) in [
        (
            "x upper lower drive",
            "VU upper 0 10\nVL lower 0 -10\n",
            -2,
            8.0,
            -4.0,
        ),
        (
            "fixed upper x drive",
            "VF fixed 0 -10\nVU upper 0 10\n",
            1,
            2.0,
            -1.0,
        ),
        (
            "fixed x lower drive",
            "VF fixed 0 10\nVL lower 0 -10\n",
            1,
            2.0,
            -1.0,
        ),
    ] {
        assert_feedback_pole(
            &voltage_feedback(
                &format!("climit(gain={gain} limit_range=0)"),
                ports,
                extra,
                current,
            ),
            2.0,
            pole,
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn lookup_poles_use_the_active_segment_and_selected_input() {
    assert_feedback_pole(
        &voltage_feedback(
            "pwl(x_array=[-4 -2 0 2 4] y_array=[8 4 0 -8 -20] input_domain=0)",
            "x drive",
            "",
            6.0,
        ),
        1.0,
        -6.0,
    );
    // Swapping vector order must still differentiate the selected minimum.
    for ports in ["[x ref] drive", "[ref x] drive"] {
        assert_feedback_pole(
            &voltage_feedback(
                "multi_input_pwl(x=[0 4] y=[0 -8] model=\"and\")",
                ports,
                "VREF ref 0 3\n",
                4.0,
            ),
            1.0,
            -4.0,
        );
    }
}

struct TableFile(&'static str);

impl Drop for TableFile {
    fn drop(&mut self) {
        let _ = rspice_core::xspice::unregister_data_file(self.0);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn table_poles_include_all_controls_and_selected_output_types() {
    for (dimensions, path, ports) in [
        (2, "virtual://pole-descriptor/table2d", "x x"),
        (3, "virtual://pole-descriptor/table3d", "x x x"),
    ] {
        // Multilinear interpolation reproduces -x*y and -x*y*z exactly.
        let mut data = "4\n".repeat(dimensions) + &"0 1 2 3\n".repeat(dimensions);
        for z in 0..if dimensions == 3 { 4 } else { 1 } {
            for y in 0..4 {
                for x in 0..4 {
                    let value = -x * y * if dimensions == 3 { z } else { 1 };
                    data.push_str(&format!("{value} "));
                }
                data.push('\n');
            }
        }
        rspice_core::xspice::register_data_file(path, data).unwrap();
        let _file = TableFile(path);
        let bias = 1.25_f64;
        let product = bias.powi(dimensions as i32);
        let slope = dimensions as f64 * bias.powi(dimensions as i32 - 1);
        for (output, gain) in [("%v(drive)", 1), ("%vd[drive 0]", 1), ("%vd[0 drive]", -1)] {
            assert_feedback_pole(
                &voltage_feedback(
                    &format!("table{dimensions}d(file=\"{path}\" order=2 gain={gain})"),
                    &format!("{ports} {output}"),
                    "",
                    2.0 * bias + product,
                ),
                bias,
                -2.0 - slope,
            );
        }
        // A positive product drawn through the default current output adds
        // conductance to the shunt. The series probe preserves that polarity.
        let netlist = Netlist::parse(&format!(
            "Table current feedback\nA1 {ports} drive cm\n.model cm table{dimensions}d(file=\"{path}\" order=2 gain=-1)\nVPROBE drive x 0\nRshunt x 0 1\nC1 x 0 1\nIBIAS 0 x {}\n.pz x 0 x 0 cur pz\n.end\n",
            bias + product
        )).unwrap();
        assert_feedback_pole(&netlist, bias, -1.0 - slope);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn differential_vector_feedback_poles_include_the_negative_control_terminal() {
    for (ports, gain) in [("[%vd[ref x]] drive", 2), ("[%vd[x ref]] drive", -2)] {
        assert_feedback_pole(
            &voltage_feedback(
                &format!("spice2poly(coef=[0 {gain}])"),
                ports,
                "VREF ref 0 3\n",
                2.0,
            ),
            2.0,
            -4.0,
        );
    }
}
