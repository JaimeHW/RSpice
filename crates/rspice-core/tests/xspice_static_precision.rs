//! Memoryless code-model arithmetic must retain representable derivatives.
use rspice_core::xspice::{CmContext, CodeModel, models::Multiplier};

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
