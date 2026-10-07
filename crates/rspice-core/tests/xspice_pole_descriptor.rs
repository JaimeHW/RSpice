//! Code-model private states must remain visible to natural-pole extraction.
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
