//! Code-model arithmetic must retain representable responses and derivatives.
use rspice_core::xspice::{
    CmContext, CodeModel,
    models::{Divider, Multiplier, SXfer, Spice2Poly},
};

fn rational_context(gain: f64, numerator: &[f64], denominator: &[f64]) -> CmContext {
    let mut context = CmContext::new();
    context.set_param("gain", gain);
    context.set_real_vector_param("num_coeff", numerator.to_vec());
    context.set_real_vector_param("den_coeff", denominator.to_vec());
    SXfer.init(&mut context).unwrap();
    context
}

fn coefficient_list(values: &[f64]) -> String {
    values
        .iter()
        .map(|value| format!("{value:e}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn relative_component(actual: f64, expected: f64) {
    // A complex norm would itself underflow for some of these finite gains.
    assert!(
        actual.is_finite() && (actual - expected).abs() <= 5e-14 * expected.abs(),
        "{actual:e} != {expected:e}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn constant_rational_transfer_retains_common_coefficient_scales() {
    use rspice_core::xspice::XspiceSmallSignalDescriptor;
    use rspice_core::{Engine, Netlist};

    for scale in [1e-100, 1e-310, f64::from_bits(1), 1e300] {
        for sign in [-1.0, 1.0] {
            let mut context = rational_context(1.0, &[2.0 * scale], &[sign * scale]);
            context.set_input_analog("in", 10.0);
            SXfer.evaluate(&mut context).unwrap();
            relative_component(context.output("out"), sign * 20.0);
            relative_component(context.partial("out"), sign * 2.0);
            relative_component(SXfer.ac_gain(&context)[0], sign * 2.0);
            assert!(matches!(
                SXfer.small_signal_descriptor(&context).unwrap(),
                XspiceSmallSignalDescriptor::AffineAc
            ));

            let netlist = Netlist::parse(&format!(
                "Constant rational scale\nV1 in 0 dc 10 ac 1\nA1 in out filt\n.model filt s_xfer(num_coeff=[{:e}] den_coeff=[{:e}])\nRload out 0 1\nRpole out rc 1\nCpole rc 0 1\n.end\n",
                2.0 * scale, sign * scale
            )).unwrap();
            let engine = Engine::default();
            let op = engine.run_dc_op(&netlist).unwrap();
            let output = op
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            relative_component(op.node_voltages[output], sign * 20.0);
            for point in engine.run_ac(&netlist, &[0.0, 1.0, 1e200]).unwrap() {
                let output = point
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                relative_component(point.voltages[output].re, sign * 2.0);
                assert!(point.voltages[output].im.abs() < 1e-12);
            }
            let spectrum = engine.run_pole_spectrum(&netlist).unwrap();
            assert!(spectrum.evidence.is_qualified());
            assert_eq!(spectrum.poles.len(), 1);
            relative_component(spectrum.poles[0].re, -1.0);
            relative_component(spectrum.poles[0].im, 0.0);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn constant_rational_output_rounds_after_all_factors_and_the_input_offset() {
    use rspice_core::xspice::AnalysisType;
    let huge = 2.0f64.powi(1023);
    let tiny = 2.0f64.powi(-600);
    for (gain, numerator, denominator, input, offset, output, partial) in [
        (1e308, 1e-308, 1.0, 10.0, 0.0, 10.0, 1.0),
        (1e-300, 1e300, 1.0, 1e-100, 0.0, 1e-100, 1.0),
        (tiny, tiny, 1.0, 1.0 / tiny, 0.0, tiny, 0.0),
        (0.5, 1.0, 1.0, huge, huge, huge, 0.5),
        (1.0, 1.0, 2.0, huge, huge, huge, 0.5),
    ] {
        for analysis in [AnalysisType::DcOp, AnalysisType::Transient] {
            let mut context = rational_context(gain, &[numerator], &[denominator]);
            context.analysis = analysis;
            context.set_input_analog("in", input);
            context.set_param("in_offset", offset);
            SXfer.evaluate(&mut context).unwrap();
            relative_component(context.output("out"), output);
            relative_component(context.partial("out"), partial);
            relative_component(SXfer.output_input_partials(&context, "out")[0].1, partial);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_denormalization_preserves_finite_canonical_coefficients_and_initial_states() {
    use rspice_core::xspice::{AnalysisType, XspiceSmallSignalDescriptor};
    use rspice_core::{Engine, Netlist};

    let tiny = f64::from_bits(1);
    for (frequency, leading, linear, constant, gain) in [
        (
            2.0f64.powi(600),
            2.0f64.powi(1000),
            2.0f64.powi(400),
            2.0f64.powi(-200),
            2.0f64.powi(-200),
        ),
        (
            2.0f64.powi(-600),
            2.0f64.powi(-1000),
            2.0f64.powi(-400),
            2.0f64.powi(200),
            2.0f64.powi(200),
        ),
        (1.0, tiny, tiny, tiny, tiny),
        (1.0, -tiny, -tiny, -tiny, -tiny),
    ] {
        for sign in [-1.0, 1.0] {
            let denominator = [leading, sign * linear, constant];
            let mut context = CmContext::new();
            context.analysis = AnalysisType::Transient;
            context.set_param("gain", gain);
            context.set_param("denormalized_freq", sign * frequency);
            context.set_real_vector_param("num_coeff", vec![1.0]);
            context.set_real_vector_param("den_coeff", denominator.to_vec());
            context.set_real_vector_param("int_ic", vec![0.2, 0.4]);
            SXfer.init(&mut context).unwrap();
            let XspiceSmallSignalDescriptor::Rational { coefficients, .. } =
                SXfer.small_signal_descriptor(&context).unwrap()
            else {
                panic!("dynamic transfer must retain its rational descriptor");
            };
            assert_eq!(coefficients.numerator, [1.0]);
            assert_eq!(coefficients.denominator, [1.0, 1.0, 1.0]);
            assert_eq!(coefficients.gain, 1.0);
            assert_eq!(
                [context.state(0), context.state(1), context.state(2)],
                [0.4, 0.2, 0.0]
            );
            context.set_input_analog("in", 1.0);
            context.timestep = 0.5;
            SXfer.evaluate(&mut context).unwrap();
            for (index, expected) in [0.6, 0.4, 0.4].into_iter().enumerate() {
                relative_component(context.state(index), expected);
            }
            relative_component(context.output("out"), 0.6);
            relative_component(context.partial("out"), 0.25);

            let netlist = Netlist::parse(&format!(
                "Normalized rational states\nV1 in 0 dc 0 ac 1\nA1 in out filt\n.model filt s_xfer(gain={gain:e} denormalized_freq={:e} num_coeff=[1] den_coeff=[{}])\nRload out 0 1\n.end\n",
                sign * frequency, coefficient_list(&denominator)
            )).unwrap();
            let engine = Engine::default();
            for point in engine
                .run_ac(&netlist, &[0.0, 1.0 / std::f64::consts::TAU])
                .unwrap()
            {
                let output = point
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                let expected = if point.frequency == 0.0 {
                    (1.0, 0.0)
                } else {
                    (0.0, -1.0)
                };
                assert!((point.voltages[output].re - expected.0).abs() < 1e-12);
                assert!((point.voltages[output].im - expected.1).abs() < 1e-12);
            }
            let spectrum = engine.run_pole_spectrum(&netlist).unwrap();
            assert!(spectrum.evidence.is_qualified());
            assert_eq!(spectrum.poles.len(), 2);
            for pole in spectrum.poles {
                assert!((pole.re + 0.5).abs() < 1e-10);
                assert!((pole.im.abs() - 3.0f64.sqrt() / 2.0).abs() < 1e-10);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_denormalization_retains_numerator_terms_across_frequency_power_range() {
    use rspice_core::xspice::XspiceSmallSignalDescriptor;
    for exponent in [-1, 1] {
        let frequency = 2.0f64.powi(600 * exponent);
        let coefficient = 2.0f64.powi(-200 * exponent);
        let authored = vec![
            2.0f64.powi(1000 * exponent),
            2.0f64.powi(400 * exponent),
            coefficient,
        ];
        let mut context = CmContext::new();
        context.set_param("gain", 1.0);
        context.set_param("denormalized_freq", frequency);
        context.set_real_vector_param("num_coeff", authored.clone());
        context.set_real_vector_param("den_coeff", authored);
        SXfer.init(&mut context).unwrap();
        let XspiceSmallSignalDescriptor::Rational { coefficients, .. } =
            SXfer.small_signal_descriptor(&context).unwrap()
        else {
            panic!("canceled modes still require a rational descriptor");
        };
        assert_eq!(coefficients.numerator, [coefficient; 3]);
        assert_eq!(coefficients.denominator, [1.0; 3]);
        assert_eq!(coefficients.gain, 1.0 / coefficient);
        for frequency in [0.0, 1.0 / std::f64::consts::TAU, 1e200] {
            let gain = SXfer.output_input_ac_partials(&context, "out", frequency)[0].1;
            relative_component(gain.re, 1.0);
            relative_component(gain.im, 0.0);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_denormalization_reports_coefficients_lost_by_the_realization() {
    for (gain, numerator, denominator, frequency, parameter, element) in [
        (
            1.0,
            vec![1.0],
            vec![1e300, 1e-300],
            1.0,
            "den_coeff",
            Some(1),
        ),
        (
            1.0,
            vec![1e-300, 0.0],
            vec![1.0, 1.0],
            1e100,
            "num_coeff",
            Some(0),
        ),
        (1e-300, vec![1.0], vec![1e300, 1e300], 1.0, "gain", None),
    ] {
        let mut context = CmContext::new();
        context.set_param("gain", gain);
        context.set_param("denormalized_freq", frequency);
        context.set_real_vector_param("num_coeff", numerator);
        context.set_real_vector_param("den_coeff", denominator);
        let error = SXfer
            .init(&mut context)
            .expect_err("normalization must not erase a nonzero coefficient")
            .to_string();
        for expected in [parameter, "underflow", "realization"] {
            assert!(error.contains(expected), "{error}");
        }
        if let Some(index) = element {
            assert!(error.contains(&format!("element {index}")), "{error}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_transient_partials_match_analytic_and_finite_difference_derivatives() {
    use rspice_core::xspice::AnalysisType;
    for dt in [0.0, 0.125, 0.5, -0.5, f64::NAN, f64::INFINITY] {
        let h = if dt.is_finite() && dt > 0.0 { dt } else { 0.0 };
        for (numerator, denominator, initial, derivative) in [
            (vec![2.0], vec![1.0, 3.0], vec![0.25], 2.0 * h),
            (vec![2.0, -3.0], vec![1.0, 3.0], vec![0.25], 2.0 - 3.0 * h),
            (
                vec![4.0, -3.0, 2.0],
                vec![1.0, 3.0, 2.0],
                vec![0.25, -0.5],
                4.0 - 3.0 * h + 2.0 * h * h,
            ),
            (
                vec![2.0, -1.0],
                vec![1.0, 6.0, 11.0, 6.0],
                vec![0.25, -0.5, 0.75],
                2.0 * h * h - h * h * h,
            ),
        ] {
            for gain in [-2.0, 0.0, 0.5] {
                let mut context = rational_context(gain, &numerator, &denominator);
                context.set_real_vector_param("int_ic", initial.clone());
                SXfer.init(&mut context).unwrap();
                context.analysis = AnalysisType::Transient;
                context.timestep = dt;
                let states: Vec<_> = (0..denominator.len())
                    .map(|index| context.state(index))
                    .collect();
                for input in [-1.5, 0.0, 2.25] {
                    for offset in [0.0, 0.75] {
                        context.set_param("in_offset", offset);
                        context.set_input_analog("in", input);
                        let expected = gain * derivative;
                        let before: Vec<_> = (0..denominator.len())
                            .map(|index| context.state(index))
                            .collect();
                        let previous_output = context.output("out");
                        relative_component(
                            SXfer.output_input_partials(&context, "out")[0].1,
                            expected,
                        );
                        assert_eq!(context.output("out"), previous_output);
                        for (index, &value) in before.iter().enumerate() {
                            assert_eq!(context.state(index), value);
                        }
                        SXfer.evaluate(&mut context).unwrap();
                        relative_component(context.partial("out"), expected);
                        let step = 2.0f64.powi(-18);
                        context.set_input_analog("in", input + step);
                        SXfer.evaluate(&mut context).unwrap();
                        let plus = context.output("out");
                        context.set_input_analog("in", input - step);
                        SXfer.evaluate(&mut context).unwrap();
                        let minus = context.output("out");
                        let difference = (plus - minus) / (2.0 * step);
                        assert!(
                            (difference - expected).abs() < 1e-10,
                            "dt={dt}, gain={gain}, input={input}, offset={offset}, finite difference={difference}, expected={expected}"
                        );
                        for (index, &state) in states.iter().enumerate() {
                            assert_eq!(context.state_prev(index), state);
                        }
                    }
                }
                SXfer.evaluate(&mut context).unwrap();
                let output = context.output("out");
                SXfer.evaluate(&mut context).unwrap();
                assert_eq!(context.output("out"), output);
                context.advance_state();
                SXfer.evaluate(&mut context).unwrap();
                relative_component(context.partial("out"), gain * derivative);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_transient_partials_retain_compensating_gain_and_timestep_factors() {
    use rspice_core::xspice::AnalysisType;
    let huge = 2.0f64.powi(600);
    for (gain, numerator, order, dt, expected) in [
        (1e-300, vec![1.0], 2, 1e200, 1e100),
        (1e300, vec![1.0], 2, 1e-200, 1e-100),
        (1e-300, vec![1e-300], 1, 1e300, 1e-300),
        (1e300, vec![1e300], 1, 1e-300, 1e300),
        (1.0, vec![1.0, -huge, 1.0], 2, huge, 1.0),
        (1.0, vec![f64::from_bits(1)], 1, 1.0, f64::from_bits(1)),
        (1e-300, vec![1e-300], 1, 1.0, 0.0),
    ] {
        let mut denominator = vec![0.0; order + 1];
        denominator[0] = 1.0;
        for sign in [-1.0, 1.0] {
            let mut context = rational_context(sign * gain, &numerator, &denominator);
            context.analysis = AnalysisType::Transient;
            context.timestep = dt;
            context.set_input_analog("in", 0.0);
            SXfer.evaluate(&mut context).unwrap();
            assert_eq!(context.output("out"), 0.0);
            relative_component(context.partial("out"), sign * expected);
            relative_component(
                SXfer.output_input_partials(&context, "out")[0].1,
                sign * expected,
            );
        }
    }
    let mut context = rational_context(1e308, &[10.0], &[1.0, 0.0]);
    context.analysis = AnalysisType::Transient;
    context.timestep = 1.0;
    SXfer.evaluate(&mut context).unwrap();
    assert_eq!(context.output("out"), 0.0);
    assert_eq!(context.partial("out"), f64::INFINITY);
    assert_eq!(
        SXfer.output_input_partials(&context, "out")[0].1,
        f64::INFINITY
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_transient_feedback_has_no_spurious_dc_algebraic_loop() {
    use rspice_core::{Engine, Netlist};
    let netlist = Netlist::parse(
        "Rational feedback derivative\nVdrive drive 0 pulse(-1 1 0.01 1n 1n 1 2)\nEfeedback in drive out 0 0.5\nA1 in out filt\n.model filt s_xfer(num_coeff=[1] den_coeff=[1 1] int_ic=[-2])\nRload out 0 1\n.end\n"
    ).unwrap();
    let engine = Engine::default();
    let op = engine.run_dc_op(&netlist).unwrap();
    for name in ["in", "out"] {
        let node = op
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap();
        assert!((op.node_voltages[node] + 2.0).abs() < 1e-10);
    }
    // dx/dt = drive - 0.5*x. Start at equilibrium x=-2, then drive steps
    // from -1 to +1 at 10 ms. The 1 ns rise is negligible at this tolerance.
    let result = engine.run_tran(&netlist, 0.2, 0.001).unwrap();
    let output = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("out"))
        .unwrap();
    for (&time, &value) in result.time.iter().zip(&result.voltages[output]) {
        let expected = 2.0 - 4.0 * (-0.5 * (time - 0.01).max(0.0)).exp();
        assert!(
            (value - expected).abs() < 0.002,
            "time={time}, output={value}, expected={expected}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_ac_retains_finite_quotients_beyond_intermediate_float_range() {
    use rspice_core::{Complex64, Engine, Netlist};
    let large_frequency = 1e200;
    let inverse_omega = 1.0 / (std::f64::consts::TAU * large_frequency);
    let corner_ratio = std::f64::consts::TAU * 0.1;
    let lowpass = Complex64::new(1.0, -corner_ratio) / (1.0 + corner_ratio * corner_ratio);
    let highpass = Complex64::new(corner_ratio * corner_ratio, corner_ratio)
        / (1.0 + corner_ratio * corner_ratio);
    for (gain, numerator, denominator, frequency, expected) in [
        (
            1.0,
            vec![1.0],
            vec![1.0, 1.0],
            large_frequency,
            Complex64::new(0.0, -inverse_omega),
        ),
        (
            1.0,
            vec![1.0, 0.0, 0.0],
            vec![1.0, 0.0, 1.0],
            large_frequency,
            Complex64::new(1.0, 0.0),
        ),
        (
            1.0,
            vec![1e-300],
            vec![1.0, 1e-300],
            0.0,
            Complex64::new(1.0, 0.0),
        ),
        (1.0, vec![1e-300], vec![1.0, 1e-300], 1e-301, lowpass),
        (
            1e-308,
            vec![1e308, 0.0],
            vec![1.0, 1.0],
            large_frequency,
            Complex64::new(1.0, inverse_omega),
        ),
        (
            1e308,
            vec![1e-308, 0.0],
            vec![1.0, 1e-300],
            1e-301,
            highpass,
        ),
        (
            1e-300,
            vec![1e-300],
            vec![1.0, 1e-300],
            0.0,
            Complex64::new(1e-300, 0.0),
        ),
        (
            1e300,
            vec![1e300],
            vec![1.0, 1e300],
            0.0,
            Complex64::new(1e300, 0.0),
        ),
        (
            1.0,
            vec![f64::from_bits(2)],
            vec![1.0, f64::from_bits(1)],
            0.0,
            Complex64::new(2.0, 0.0),
        ),
        (
            1.0,
            vec![f64::MAX],
            vec![1.0, f64::MAX],
            0.0,
            Complex64::new(1.0, 0.0),
        ),
        (
            f64::MAX,
            vec![1e300],
            vec![1.0, 1e300],
            0.0,
            Complex64::new(f64::MAX, 0.0),
        ),
    ] {
        for sign in [-1.0, 1.0] {
            let context = rational_context(sign * gain, &numerator, &denominator);
            let actual = SXfer.output_input_ac_partials(&context, "out", frequency)[0].1;
            relative_component(actual.re, sign * expected.re);
            relative_component(actual.im, sign * expected.im);

            let netlist = Netlist::parse(&format!(
                "Scaled rational AC\nV1 in 0 dc 0 ac 1\nA1 in out filt\n.model filt s_xfer(gain={:e} num_coeff=[{}] den_coeff=[{}])\nRload out 0 1\n.end\n",
                sign * gain, coefficient_list(&numerator), coefficient_list(&denominator)
            )).unwrap();
            let point = Engine::default()
                .run_ac(&netlist, &[frequency])
                .unwrap()
                .remove(0);
            let output = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            relative_component(point.voltages[output].re, sign * expected.re);
            relative_component(point.voltages[output].im, sign * expected.im);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_dc_gain_preserves_tiny_denominators_and_compensating_factors() {
    for (gain, numerator, denominator, expected) in [
        (1.0, 1e-300, 1e-300, 1.0),
        (1e-300, 1e-300, 1e-300, 1e-300),
        (1e300, 1e300, 1e300, 1e300),
        (1.0, f64::from_bits(2), f64::from_bits(1), 2.0),
        (f64::MAX, 1e300, 1e300, f64::MAX),
    ] {
        let context = rational_context(gain, &[numerator], &[1.0, denominator]);
        relative_component(SXfer.ac_gain(&context)[0], expected);
    }
    // The legacy real-gain callback keeps the exact-integrator DC convention.
    let context = rational_context(1.0, &[1.0], &[1.0, 0.0]);
    assert_eq!(SXfer.ac_gain(&context), [0.0]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_singular_ac_refuses_true_poles_instead_of_reporting_zero() {
    for (numerator, denominator, frequency, pole_count, imaginary_magnitude) in [
        ("1", "1 0", 0.0, 1, 0.0),
        ("1", "1 0 1", 1.0 / std::f64::consts::TAU, 2, 1.0),
    ] {
        let netlist = rspice_core::Netlist::parse(&format!(
            "Rational singular frequency\nV1 in 0 dc 0 ac 1\nA1 in out filt\n.model filt s_xfer(num_coeff=[{numerator}] den_coeff=[{denominator}])\nRload out 0 1\n.end\n"
        )).unwrap();
        let engine = rspice_core::Engine::default();
        let error = engine
            .run_ac(&netlist, &[frequency])
            .expect_err("a singular transfer is not zero")
            .to_string();
        for expected in ["XSPICE", "A1", "out", "non-finite"] {
            assert!(error.contains(expected), "{error}");
        }
        let spectrum = engine.run_pole_spectrum(&netlist).unwrap();
        assert_eq!(spectrum.poles.len(), pole_count);
        for pole in &spectrum.poles {
            assert!(pole.re.abs() < 1e-12);
            assert!((pole.im.abs() - imaginary_magnitude).abs() < 1e-12);
        }
        assert!(spectrum.evidence.is_qualified());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_origin_cancellation_and_zero_gain_keep_their_internal_modes() {
    use rspice_core::{Complex64, Engine, Netlist};
    for (gain, numerator, denominator, dc, poles) in [
        (
            1.0,
            vec![1.0, 0.0],
            vec![1.0, 0.0],
            1.0,
            vec![Complex64::new(0.0, 0.0)],
        ),
        (
            2.0,
            vec![3.0, 0.0, 0.0],
            vec![1.0, 2.0, 0.0, 0.0],
            3.0,
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(-2.0, 0.0),
            ],
        ),
        (
            0.0,
            vec![1.0],
            vec![1.0, 0.0],
            0.0,
            vec![Complex64::new(0.0, 0.0)],
        ),
        (
            1.0,
            vec![0.0],
            vec![1.0, 0.0, 1.0],
            0.0,
            vec![Complex64::new(0.0, 1.0), Complex64::new(0.0, -1.0)],
        ),
    ] {
        let context = rational_context(gain, &numerator, &denominator);
        relative_component(SXfer.ac_gain(&context)[0], dc);
        let netlist = Netlist::parse(&format!(
            "Removable transfer origins\nV1 in 0 dc 0 ac 1\nA1 in out filt\n.model filt s_xfer(gain={gain} num_coeff=[{}] den_coeff=[{}])\nRload out 0 1\n.end\n",
            coefficient_list(&numerator), coefficient_list(&denominator)
        )).unwrap();
        let engine = Engine::default();
        let point = engine.run_ac(&netlist, &[0.0]).unwrap().remove(0);
        let output = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        relative_component(point.voltages[output].re, dc);
        relative_component(point.voltages[output].im, 0.0);
        if dc == 0.0 {
            let point = engine
                .run_ac(&netlist, &[1.0 / std::f64::consts::TAU])
                .unwrap()
                .remove(0);
            assert_eq!(point.voltages[output], Complex64::new(0.0, 0.0));
        }
        let spectrum = engine.run_pole_spectrum(&netlist).unwrap();
        assert!(spectrum.evidence.is_qualified());
        let mut remaining = spectrum.poles;
        assert_eq!(remaining.len(), poles.len());
        let repeated_origin = poles
            .iter()
            .filter(|pole| pole.re == 0.0 && pole.im == 0.0)
            .count()
            > 1;
        for expected in poles {
            // The double integrator is a defective zero eigenvalue: a
            // perturbation of size epsilon can split it by sqrt(epsilon).
            // Check both retained modes; AC response checks stay much tighter.
            let tolerance = if repeated_origin && expected == Complex64::new(0.0, 0.0) {
                2.0 * f64::EPSILON.sqrt()
            } else {
                1e-10
            };
            let index = remaining
                .iter()
                .position(|pole| (*pole - expected).norm() < tolerance)
                .unwrap_or_else(|| panic!("gain={gain}, numerator={numerator:?}, denominator={denominator:?}: missing {expected}, actual {remaining:?}"));
            remaining.remove(index);
        }
    }
}

fn multiplier(inputs: &[f64], gains: &[f64], offset: f64) -> (f64, Vec<f64>) {
    let mut context = CmContext::new();
    context.set_port_width("in", inputs.len());
    context.set_input_analog_vector("in", inputs).unwrap();
    context.set_real_vector_param("in_gain", gains.to_vec());
    context.mark_param_provided("in_gain");
    context.set_param("out_gain", 1.0);
    context.set_param("out_offset", offset);
    Multiplier.init(&mut context).unwrap();
    Multiplier.evaluate(&mut context).unwrap();
    let partials = Multiplier.output_input_vector_partials(&context, "out");
    (
        context.output("out"),
        partials.into_iter().map(|(_, _, value)| value).collect(),
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn multiplier_retains_derivatives_when_its_output_underflows() {
    let tiny = 2.0f64.powi(-600);
    let (output, partials) = multiplier(&[tiny, tiny], &[1.0, 1.0], 0.0);
    assert_eq!(output, 0.0);
    assert_eq!(partials, [tiny, tiny]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn multiplier_rounds_after_all_input_and_gain_factors() {
    let tiny = 2.0f64.powi(-600);
    let huge = 2.0f64.powi(600);
    for inputs in [[tiny, tiny, huge], [tiny, huge, tiny], [huge, tiny, tiny]] {
        let (output, partials) = multiplier(&inputs, &[1.0; 3], 0.0);
        assert_eq!(output, tiny);
        for (partial, input) in partials.iter().zip(inputs) {
            assert_eq!(*partial, if input == huge { 0.0 } else { 1.0 });
        }
    }
    let (output, partials) = multiplier(&[1.0, 2.0, 3.0, 4.0], &[huge, huge, tiny, tiny], 0.0);
    assert_eq!(output, 24.0);
    assert_eq!(partials, [24.0, 12.0, 8.0, 6.0]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn multiplier_retains_zero_input_derivatives_and_offset_cancellation() {
    let tiny = 2.0f64.powi(-600);
    let huge = 2.0f64.powi(600);
    let (output, partials) = multiplier(&[0.0, huge, huge], &[tiny, 1.0, 1.0], 0.0);
    assert_eq!(output, 0.0);
    assert_eq!(partials, [huge, 0.0, 0.0]);
    let largest_power = 2.0f64.powi(1023);
    let (output, partials) = multiplier(&[largest_power, 2.0], &[1.0, 1.0], -largest_power);
    assert_eq!(output, largest_power);
    assert_eq!(partials, [2.0, largest_power]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn multiplier_preserves_subnormals_signs_and_reports_true_overflow() {
    let subnormal = f64::from_bits(1);
    let huge = 2.0f64.powi(1023);
    let (output, partials) = multiplier(&[-subnormal, huge], &[1.0, 1.0], 0.0);
    assert_eq!(output, -2.0f64.powi(-51));
    assert_eq!(partials, [huge, -subnormal]);
    let (output, partials) = multiplier(&[0.0, 0.0, huge], &[1.0; 3], 3.0);
    assert_eq!(output, 3.0);
    assert_eq!(partials, [0.0; 3]);
    let (output, partials) = multiplier(&[0.0, huge, huge], &[1.0; 3], 0.0);
    assert_eq!(output, 0.0);
    // The existing circuit stamper rejects non-finite trial derivatives.
    // They must remain visible instead of being replaced with zero.
    assert_eq!(partials[0], f64::INFINITY);
    let (output, _) = multiplier(&[0.0, 0.0, f64::NAN], &[1.0; 3], 0.0);
    assert!(output.is_nan());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn multiplier_ac_retains_a_derivative_below_the_dc_output_range() {
    let tiny = 2.0f64.powi(-600);
    let netlist = rspice_core::Netlist::parse(&format!(
        "Tiny multiplier\nV1 in 0 dc {tiny:e} ac 1\nV2 bias 0 dc {tiny:e}\nA1 [in bias] out mult\n.model mult mult\nR1 out 0 1\n.end\n"
    )).unwrap();
    let points = rspice_core::Engine::default()
        .run_ac(&netlist, &[1.0])
        .unwrap();
    let out = points[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    assert_eq!(points[0].voltages[out].re, tiny);
    assert_eq!(points[0].voltages[out].im, 0.0);
}

fn divider(num: f64, den: f64, num_gain: f64, den_gain: f64, out_gain: f64) -> (f64, Vec<f64>) {
    let mut context = CmContext::new();
    context.set_input_analog("num", num);
    context.set_input_analog("den", den);
    for (name, value) in [
        ("num_gain", num_gain),
        ("den_gain", den_gain),
        ("out_gain", out_gain),
        ("den_lower_limit", 1e-10),
    ] {
        context.set_param(name, value);
    }
    Divider.init(&mut context).unwrap();
    Divider.evaluate(&mut context).unwrap();
    (
        context.output("out"),
        Divider
            .output_input_partials(&context, "out")
            .into_iter()
            .map(|(_, value)| value)
            .collect(),
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn divider_preserves_scaled_quotients_and_both_partials() {
    let huge = 2.0f64.powi(600);
    let tiny = 2.0f64.powi(-600);
    for gain in [1.0, huge] {
        let (output, partials) = divider(huge, huge, gain, gain, 1.0);
        assert_eq!(output, 1.0);
        assert_eq!(partials, [tiny, -tiny]);
    }
    let (output, partials) = divider(tiny, 2.0, tiny, 1.0, huge);
    assert_eq!(output, tiny / 2.0);
    assert_eq!(partials, [0.5, -tiny / 4.0]);
}

fn polynomial(inputs: &[f64], coefficients: &[f64]) -> (f64, Vec<f64>) {
    let mut context = CmContext::new();
    context.set_port_width("in", inputs.len());
    context.set_input_analog_vector("in", inputs).unwrap();
    context.set_real_vector_param("coef", coefficients.to_vec());
    context.mark_param_provided("coef");
    context.set_param("m", 1.0);
    Spice2Poly.init(&mut context).unwrap();
    Spice2Poly.evaluate(&mut context).unwrap();
    let mut partials = vec![0.0; inputs.len()];
    for (_, index, value) in Spice2Poly.output_input_vector_partials(&context, "out") {
        partials[index] = value;
    }
    (context.output("out"), partials)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn polynomial_retains_derivatives_and_coefficient_scale_before_rounding() {
    let tiny = 2.0f64.powi(-600);
    let huge = 2.0f64.powi(600);
    assert_eq!(
        polynomial(&[tiny], &[0.0, 0.0, 1.0]),
        (0.0, vec![2.0 * tiny])
    );
    assert_eq!(
        polynomial(&[tiny], &[0.0, 0.0, 2.0f64.powi(1000)]),
        (2.0f64.powi(-200), vec![2.0f64.powi(401)])
    );
    assert_eq!(
        polynomial(&[huge, huge], &[0.0, 0.0, 0.0, 0.0, tiny]),
        (huge, vec![1.0, 1.0])
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn polynomial_keeps_cancellation_between_out_of_range_terms() {
    let huge = 2.0f64.powi(1023);
    assert_eq!(polynomial(&[huge], &[-huge, 2.0]), (huge, vec![2.0]));
    assert_eq!(
        polynomial(&[huge, huge], &[0.0, 2.0, -2.0]),
        (0.0, vec![2.0, -2.0])
    );
    let huge = 2.0f64.powi(600);
    assert_eq!(
        polynomial(&[huge, huge], &[0.0, 0.0, 0.0, 0.5, 0.0, -0.5]),
        (0.0, vec![huge, -huge])
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn small_signal_analyses_reject_unrepresentable_transfer_gain_instead_of_zero() {
    for output in ["out", "%id[out 0]"] {
        let netlist = rspice_core::Netlist::parse(&format!(
            "Unrepresentable AC gain\nV1 in 0 dc 0 ac 1\nA1 in {output} filt\n.model filt s_xfer(gain=100 num_coeff=[1e308] den_coeff=[1 1])\nR1 out 0 1\nP1 rf 0 portnum=1 z0=50\nRport rf 0 50\n.end\n"
        )).unwrap();
        // The zero operating point is finite; only the AC derivative overflows.
        let engine = rspice_core::Engine::default();
        engine.run_dc_op(&netlist).unwrap();
        for (analysis, result) in [
            ("AC", engine.run_ac(&netlist, &[1.0]).map(|_| ())),
            (
                "noise",
                engine
                    .run_noise_named_with_input_source(&netlist, "out", None, "V1", &[1.0], 300.15)
                    .map(|_| ()),
            ),
            (
                "SP",
                engine
                    .run_sp_over_grid_with_abort(&netlist, &[1.0], true, &rspice_core::NoAbort)
                    .map(|_| ()),
            ),
        ] {
            let error = result.expect_err("an invalid code-model gain must not become zero");
            let message = error.to_string();
            for expected in ["XSPICE", "A1", "out", "non-finite", "Hz"] {
                assert!(
                    message.contains(expected),
                    "{analysis}, {output}: {message}"
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rational_dc_rejects_unrepresentable_feedthrough_even_at_zero_input() {
    for output in ["out", "%id[out 0]"] {
        for gain in [-10.0, 10.0] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "Unrepresentable feedthrough\nV1 in 0 dc 0\nA1 in {output} filt\n.model filt s_xfer(gain={gain} num_coeff=[1e308 0] den_coeff=[1 1])\nRload out 0 1\n.end\n"
            )).unwrap();
            let error = rspice_core::Engine::default()
                .run_dc_op(&netlist)
                .expect_err("zero output does not make an infinite derivative valid")
                .to_string();
            for expected in ["XSPICE", "A1", "out", "conductance", "inf"] {
                assert!(error.contains(expected), "{output}, gain={gain}: {error}");
            }
        }
    }
}
