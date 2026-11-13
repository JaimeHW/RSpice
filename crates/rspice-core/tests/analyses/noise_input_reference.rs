//! Independent unit-source noise referral; ordinary AC observations remain intact.
use rspice_core::analysis::noise::{NoiseInputQuantity, NoisePhysicalConstants, NoiseResult};
use rspice_core::{Engine, Netlist, SimulationConfig, SpiceDialect};

fn relative(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs() * 1e-9,
        "{actual:e} != {expected:e}"
    );
}

fn engine(dialect: SpiceDialect) -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: dialect,
        ..SimulationConfig::default()
    })
}

fn noise(engine: &Engine, deck: &str, source: &str) -> Vec<NoiseResult> {
    let netlist = Netlist::parse(deck).unwrap();
    let frequencies = [10.0, 1e3, 1e5];
    let ac = engine.run_ac(&netlist, &frequencies).unwrap();
    let result = engine
        .run_noise_named_with_input_source(
            &netlist,
            "out",
            Some("ref"),
            source,
            &frequencies,
            300.15,
        )
        .unwrap();
    for (point, ac) in result.iter().zip(ac) {
        assert_eq!(point.node_names, ac.node_names);
        assert_eq!(point.branch_names, ac.branch_names);
        for (actual, expected) in point
            .voltages
            .iter()
            .chain(&point.currents)
            .zip(ac.voltages.iter().chain(&ac.currents))
        {
            assert!(
                (*actual - *expected).norm() <= 1e-12 * expected.norm().max(1e-20),
                "retained AC {actual:?} != {expected:?}"
            );
        }
    }
    result
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unit_voltage_referral_ignores_authored_amplitudes_phases_and_other_drives() {
    for (dialect, boltzmann) in [
        (SpiceDialect::BestAvailable, 1.380649e-23),
        (
            SpiceDialect::Ngspice,
            NoisePhysicalConstants::NGSPICE_46.boltzmann,
        ),
    ] {
        for (amplitude, phase, other) in [(0.0, 0.0, 0.0), (2.0, -40.0, 7e-4), (2e-6, 170.0, 3e-4)]
        {
            let deck = format!(
                "Unit voltage noise\nVIN in ref DC 0 AC {amplitude} {phase}\nVREF ref 0 DC 0 AC 0.3 60\nITRIM out ref DC 0 AC {other} 37\nR1 in out 1k\nC1 out ref 1u\n.end\n"
            );
            for point in noise(&engine(dialect), &deck, "vin") {
                let gain = 1.0 / (1.0 + (std::f64::consts::TAU * point.frequency * 1e-3).powi(2));
                assert_eq!(point.input_quantity, Some(NoiseInputQuantity::Voltage));
                relative(point.input_gain_squared, gain);
                relative(
                    point.output_noise_density,
                    4.0 * boltzmann * 300.15 * 1e3 * gain,
                );
                relative(point.input_referred_density, 4.0 * boltzmann * 300.15 * 1e3);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unit_current_referral_uses_both_terminals_and_preserves_ampere_density() {
    for (dialect, boltzmann) in [
        (SpiceDialect::BestAvailable, 1.380649e-23),
        (
            SpiceDialect::Ngspice,
            NoisePhysicalConstants::NGSPICE_46.boltzmann,
        ),
    ] {
        for terminals in ["ref out", "out ref"] {
            for (amplitude, phase) in [(0.0, 0.0), (2e-6, 83.0), (7.0, -170.0)] {
                let deck = format!(
                    "Unit current noise\nIIN {terminals} DC 0 AC {amplitude} {phase}\nVREF ref 0 DC 0 AC 0.3 60\nITRIM out ref DC 0 AC 0.2u 37\nR1 out ref 1k\nC1 out ref 1u\n.end\n"
                );
                for point in noise(&engine(dialect), &deck, "iin") {
                    let gain =
                        1e6 / (1.0 + (std::f64::consts::TAU * point.frequency * 1e-3).powi(2));
                    assert_eq!(point.input_quantity, Some(NoiseInputQuantity::Current));
                    relative(point.input_gain_squared, gain);
                    relative(point.input_referred_density, 4.0 * boltzmann * 300.15 / 1e3);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn hierarchical_inputs_use_unit_transfer_even_when_full_ac_output_cancels() {
    for (source, devices, gain) in [
        (
            "x1.vin",
            "VIN in ref DC 0 AC 2\nR1 in out 1k\nITRIM out ref DC 0 AC 2m",
            1.0,
        ),
        (
            "x1.iin",
            "IIN ref out DC 0 AC 2m\nR1 out ref 1k\nITRIM out ref DC 0 AC 2m",
            1e6,
        ),
    ] {
        let deck = format!(
            "Cancelled ordinary AC\nX1 out ref input\nVREF ref 0 0\n.subckt input out ref\n{devices}\n.ends\n.end\n"
        );
        for dialect in [SpiceDialect::BestAvailable, SpiceDialect::Ngspice] {
            for point in noise(&engine(dialect), &deck, source) {
                relative(point.input_gain_squared, gain);
                assert!(point.input_referred_density > 0.0);
                let out = point
                    .node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case("out"))
                    .unwrap();
                assert!(point.voltages[out].norm() < 1e-12);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_referral_includes_two_unclamped_input_terminals() {
    let deck = "Floating current input\nIIN ref out DC 0 AC 2m 30\nITRIM 0 out DC 0 AC 0.5m\nR1 out 0 1k\nR2 ref 0 2k\n.end\n";
    for point in noise(&engine(SpiceDialect::BestAvailable), deck, "IIN") {
        // The differential port sees R1+R2, and the two independent thermal
        // voltage spectra add. Both adjoint injection coordinates are nonzero.
        relative(point.input_gain_squared, 9e6);
        relative(
            point.output_noise_density,
            4.0 * 1.380649e-23 * 300.15 * 3e3,
        );
        relative(
            point.input_referred_density,
            4.0 * 1.380649e-23 * 300.15 / 3e3,
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn xyce_referral_keeps_full_differential_ac_response() {
    let deck = "Xyce differential noise\nVIN in ref DC 0 AC 2 30\nVREF ref 0 DC 0 AC 0.3 60\nITRIM out ref DC 0 AC 700u -40\nR1 in out 1k\nC1 out ref 1u\n.end\n";
    for source in ["VIN", "ITRIM"] {
        for point in noise(&engine(SpiceDialect::Xyce), deck, source) {
            let index = |name: &str| {
                point
                    .node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .unwrap()
            };
            let gain = (point.voltages[index("out")] - point.voltages[index("ref")]).norm_sqr();
            relative(point.input_gain_squared, gain);
            relative(
                point.input_referred_density,
                point.output_noise_density / gain,
            );
        }
    }
}
