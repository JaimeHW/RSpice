//! Natural circuit modes do not depend on a transfer port or its gain range.
use rspice_core::analysis::pole_zero::StabilityVerdict;
use rspice_core::{AbortSignal, Engine, Netlist, SimulationError};
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn natural_circuit_spectrum_retains_an_unobserved_unstable_mode() {
    let netlist = Netlist::parse(
        "Hidden mode\nR1 out 0 .5\nC1 out 0 1\nG1 hidden 0 hidden 0 -1\nC2 hidden 0 1\n.end\n",
    )
    .unwrap();
    let spectrum = Engine::default().run_pole_spectrum(&netlist).unwrap();
    assert_eq!(spectrum.stability_verdict(), StabilityVerdict::Unstable);
    assert_eq!(spectrum.poles.len(), 2);
    assert!((spectrum.poles[0].re - 1.0).abs() < 1e-12);
    assert!((spectrum.poles[1].re + 2.0).abs() < 1e-12);
    assert!(spectrum.evidence.is_qualified());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn natural_circuit_spectrum_needs_neither_a_transfer_gain_nor_an_excitation() {
    let netlist = Netlist::parse("Tiny static conductance\nG1 out 0 out 0 1e-310\n.end\n").unwrap();
    let mut config = rspice_core::engine::SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    let engine = Engine::new(config);
    let spectrum = engine.run_pole_spectrum(&netlist).unwrap();
    assert_eq!(spectrum.stability_verdict(), StabilityVerdict::Stable);
    assert!(spectrum.poles.is_empty());
    assert_eq!(spectrum.evidence.certificate().unwrap().infinite_count, 1);
    assert!(
        engine.run_pz(&netlist, 1, 1).is_err(),
        "1/G cannot be represented as a binary64 gain"
    );

    let clamped =
        Netlist::parse("Clamped capacitor\nV1 out 0 0\nR1 out 0 1k\nC1 out 0 1u\n.end\n").unwrap();
    let spectrum = Engine::default().run_pole_spectrum(&clamped).unwrap();
    assert!(spectrum.poles.is_empty());
    let certificate = spectrum.evidence.certificate().unwrap();
    assert_eq!(
        (certificate.problem_order, certificate.infinite_count),
        (2, 2)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn natural_circuit_spectrum_uses_the_accepted_capacitor_bias() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::engine::{SimulationConfig, SpiceDialect};
    use rspice_core::netlist::NetlistParseOptions;
    let netlist = Netlist::parse_with_options(
        "Accepted bias\nI1 0 out 1m\nR1 out 0 1k\nC1 out 0 C={1u*(1+V(out))}\n.end\n",
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap();
    let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
    config.convergence_config.gmin_target = 0.0;
    let spectrum = Engine::new_with_resolved_config(config)
        .run_pole_spectrum(&netlist)
        .unwrap();
    assert_eq!(spectrum.poles.len(), 1);
    assert!((spectrum.poles[0].re + 500.0).abs() < 1e-8);
    assert_eq!(spectrum.stability_verdict(), StabilityVerdict::Stable);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn natural_circuit_spectrum_shares_vbic_charge_expansion() {
    for kind in ["NPN", "PNP"] {
        let netlist = Netlist::parse(&format!("VBIC overlap\nR1 out 0 1k\nQ1 0 out 0 qmod\n.model qmod {kind} LEVEL=4 IS=1e-30 IBEI=0 IBEN=0 IBCI=0 IBCN=0 ISP=0 CBEO=1u\n.end\n")).unwrap();
        // The passive -1/(R*C) oracle excludes artificial junction gmin.
        let mut config = rspice_core::engine::SimulationConfig::default();
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        let spectrum = engine.run_pole_spectrum(&netlist).unwrap();
        let transfer = engine.run_pz(&netlist, 1, 1).unwrap();
        assert_eq!(spectrum.poles.len(), 1);
        assert!(
            (spectrum.poles[0].re + 1000.0).abs() < 1e-7,
            "{kind}: {spectrum:?}; transfer: {transfer:?}"
        );
        assert_eq!(spectrum.poles, transfer.poles);
        assert_eq!(spectrum.evidence, transfer.pole_evidence);
    }
}

struct CancelAtCompletion(AtomicBool);
impl AbortSignal for CancelAtCompletion {
    fn is_aborted(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    fn observe_progress(&self, fraction: f64) {
        if fraction == 1.0 {
            self.0.store(true, Ordering::Relaxed);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn natural_circuit_spectrum_preserves_typed_cancellation_limits_and_admission() {
    let netlist = Netlist::parse("RC\nR1 out 0 1k\nC1 out 0 1u\n.end\n").unwrap();
    let engine = Engine::default();
    let abort = rspice_core::abort_signal::CountingAbort::new(0);
    assert!(matches!(
        engine.run_pole_spectrum_with_abort(&netlist, &abort),
        Err(SimulationError::Aborted)
    ));
    let abort = CancelAtCompletion(AtomicBool::new(false));
    assert!(matches!(
        engine.run_pole_spectrum_with_abort(&netlist, &abort),
        Err(SimulationError::Aborted)
    ));
    assert!(abort.0.load(Ordering::Relaxed));
    let mut config = rspice_core::engine::SimulationConfig::default();
    config.resource_limits.max_result_values = 8;
    assert!(matches!(
        Engine::new(config).run_pole_spectrum(&netlist),
        Err(SimulationError::ResourceLimit(_))
    ));
    let stateful =
        Netlist::parse("Integral\nR1 out 0 1k\nB1 out 0 I={SDT(V(out))}\n.end\n").unwrap();
    assert!(matches!(
        engine.run_pole_spectrum(&stateful),
        Err(SimulationError::UnsupportedCapability(_))
    ));
}
