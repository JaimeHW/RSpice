//! Loop gain must retain a finite result even near unity return difference.
use rspice_core::analysis::stb::{StbConfig, StbSweepType};
use rspice_core::{ComplexValue, Engine, Netlist};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn high_gain_single_pole_preserves_small_return_difference_in_both_orientations() {
    let mut failures = Vec::new();
    for gain in [1e-40, 1e-12, 1e6, 1e12, 1e20, 1e40, 1e200] {
        for probe in ["eo x", "x eo"] {
            let source = format!(
                "High loop gain\nE1 eo 0 ctrl 0 -{gain:e}\nVprobe {probe} 0\nR1 x ctrl 1k\nC1 ctrl 0 159.154943091895n\n.end\n"
            );
            let netlist = Netlist::parse(&source).unwrap();
            let config = StbConfig::new()
                .with_sweep(10.0, 1000.0, 3)
                .with_sweep_type(StbSweepType::Linear)
                .with_probe("Vprobe");
            match Engine::default().run_stb(&netlist, config) {
                Ok(result) => {
                    for (&frequency, &actual) in result.frequencies.iter().zip(&result.loop_gains) {
                        let expected = gain / ComplexValue::new(1.0, frequency / 1000.0);
                        let expected_scaled = expected / gain;
                        let error =
                            (actual / gain - expected_scaled).norm() / expected_scaled.norm();
                        if !error.is_finite() || error > 2e-8 {
                            failures.push(format!("gain={gain:e} probe={probe} f={frequency}: {actual}, expected {expected}, error={error:e}"));
                        }
                    }
                }
                Err(error) => failures.push(format!("gain={gain:e} probe={probe}: {error}")),
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn high_gain_loaded_break_preserves_the_analytic_return_ratio() {
    for gain in [1e6, 1e12, 1e20, 1e40] {
        for probe in ["out fb", "fb out"] {
            let source = format!(
                "Loaded loop\nE1 eo 0 ctrl 0 -{gain:e}\nRO eo out 10k\nVP {probe} 0\nRF fb ctrl 10k\nRG ctrl 0 10k\nCG ctrl 0 10n\n.end\n"
            );
            let netlist = Netlist::parse(&source).unwrap();
            let config = StbConfig::new()
                .with_sweep(10.0, 1e6, 5)
                .with_sweep_type(StbSweepType::Linear)
                .with_probe("VP");
            let result = Engine::default().run_stb(&netlist, config).unwrap();
            for (&frequency, &actual) in result.frequencies.iter().zip(&result.loop_gains) {
                let zg = 1e4 / ComplexValue::new(1.0, std::f64::consts::TAU * frequency * 1e-4);
                let expected = gain * zg / (2e4 + zg);
                let expected_scaled = expected / gain;
                let error = (actual / gain - expected_scaled).norm() / expected_scaled.norm();
                assert!(
                    error < 2e-8,
                    "gain={gain:e} probe={probe} f={frequency}: {actual}, expected {expected}, error={error:e}"
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn probe_controlled_source_terms_remain_in_the_kcl_complement() {
    // F1 adds 3*i_probe to the same KCL row as the probe itself. For the two
    // Tian experiments, a=z/(A+z), d=1/4 and c=0, hence T=(A+3z)/(3A+z).
    for gain in [1e-6, 1.0, 1e6] {
        let source = format!(
            "Probe current control\nE1 eo 0 ctrl 0 -{gain:e}\nVP x eo 0\nF1 x 0 VP 3\nR1 x ctrl 1k\nC1 ctrl 0 159.154943091895n\n.end\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        let config = StbConfig::new()
            .with_sweep(10.0, 1000.0, 3)
            .with_sweep_type(StbSweepType::Linear)
            .with_probe("VP");
        let result = Engine::default().run_stb(&netlist, config).unwrap();
        for (&frequency, &actual) in result.frequencies.iter().zip(&result.loop_gains) {
            let z = ComplexValue::new(1.0, frequency / 1000.0);
            let expected = (gain + 3.0 * z) / (3.0 * gain + z);
            let error = (actual - expected).norm() / expected.norm();
            assert!(
                error < 2e-8,
                "A={gain}, f={frequency}: {actual} != {expected}, error={error}"
            );
        }
    }
}
