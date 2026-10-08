//! Natural circuit modes do not depend on a transfer port or its gain range.
use rspice_core::analysis::pole_zero::StabilityVerdict;
use rspice_core::{AbortSignal, Engine, Netlist, SimulationError};
use std::sync::atomic::{AtomicBool, Ordering};

const LOSSLESS_LC: &str = "L1 a 0 1\nL2 b 0 3\nC1 a 0 3\nC2 b 0 5\nCc a b 4\n";

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn exact_lossless_and_weakly_damped_circuits_have_physical_stability_verdicts() {
    let engine = Engine::default();
    for damping in [0.0, 2.0f64.powi(-60), -2.0f64.powi(-60), 2.0f64.powi(-10)] {
        let deck = format!(
            "LC damping\n{LOSSLESS_LC}G1 a 0 a 0 {:.17e}\nG2 b 0 b 0 {:.17e}\nGc a b a b {:.17e}\n.pz a 0 a 0 cur pol\n.end\n",
            6.0 * damping,
            10.0 * damping,
            8.0 * damping
        );
        let netlist = Netlist::parse(&deck).unwrap();
        let spectrum = engine.run_pole_spectrum(&netlist).unwrap();
        let expected = if damping > 0.0 {
            StabilityVerdict::Stable
        } else {
            StabilityVerdict::Unstable
        };
        assert_eq!(
            spectrum.stability_verdict(),
            expected,
            "damping={damping}: {spectrum:?}"
        );
        assert_eq!(spectrum.poles.len(), 4);
        // det(s^2*C + L^-1) is (141*s^4+34*s^2+1)/3. Adding
        // G=2*damping*C gives -damping +/- j*sqrt(omega^2-damping^2).
        for (pair, sign) in spectrum.poles.chunks_exact(2).zip([-1.0, 1.0]) {
            let frequency =
                ((17.0 + sign * 2.0 * 37.0f64.sqrt()) / 141.0 - damping * damping).sqrt();
            for pole in pair {
                assert!((pole.im.abs() - frequency).abs() < 1e-12);
            }
        }
        let transfer = engine
            .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &rspice_core::NoAbort)
            .unwrap();
        assert_eq!(transfer.stability_verdict(), expected);
        assert_eq!(
            transfer
                .pole_evidence
                .certificate()
                .unwrap()
                .asymptotically_stable,
            Some(damping > 0.0)
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn lossless_lc_parameter_grid_never_acquires_asymptotic_stability_from_roundoff() {
    let engine = Engine::default();
    for l1 in 1..=5 {
        for l2 in 1..=5 {
            for coupling in 1..=5 {
                let netlist = Netlist::parse(&format!("Conserved LC energy\nL1 a 0 {l1}\nL2 b 0 {l2}\nC1 a 0 3\nC2 b 0 5\nCc a b {coupling}\n.end\n")).unwrap();
                let spectrum = engine.run_pole_spectrum(&netlist).unwrap();
                assert_eq!(spectrum.poles.len(), 4);
                assert_eq!(
                    spectrum.stability_verdict(),
                    StabilityVerdict::Unstable,
                    "L1={l1} L2={l2} Cc={coupling}: {spectrum:?}"
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn backward_error_only_legacy_spectra_do_not_recreate_a_sign_certificate() {
    use rspice_core::analysis::pole_zero::PoleSpectrum;
    let netlist = Netlist::parse(&format!("Lossless\n{LOSSLESS_LC}.end\n")).unwrap();
    let spectrum = Engine::default().run_pole_spectrum(&netlist).unwrap();
    let mut stored = serde_json::to_value(&spectrum).unwrap();
    let restored: PoleSpectrum = serde_json::from_value(stored.clone()).unwrap();
    assert_eq!(restored.stability_verdict(), StabilityVerdict::Unstable);
    stored["evidence"]["certificate"]
        .as_object_mut()
        .unwrap()
        .remove("asymptoticallyStable");
    let legacy: PoleSpectrum = serde_json::from_value(stored).unwrap();
    assert!(legacy.evidence.is_qualified());
    assert_eq!(legacy.stability_verdict(), StabilityVerdict::Indeterminate);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stability_evidence_survives_physical_scaling_and_result_documents() {
    use rspice_core::execution::{
        AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, ResultPayload,
    };
    for exponent in [-20, 0, 20] {
        let scale = 2.0f64.powi(exponent);
        for damping in [0.0, 2.0f64.powi(-60), -2.0f64.powi(-60)] {
            let netlist = Netlist::parse(&format!(
                "Scaled LC\nL1 a 0 {:.17e}\nL2 b 0 {:.17e}\nC1 a 0 {:.17e}\nC2 b 0 {:.17e}\nCc a b {:.17e}\nG1 a 0 a 0 {:.17e}\nG2 b 0 b 0 {:.17e}\nGc a b a b {:.17e}\n.pz a 0 a 0 cur pol\n.end\n",
                1.0/scale, 3.0/scale, 3.0*scale, 5.0*scale, 4.0*scale,
                6.0*damping*scale, 10.0*damping*scale, 8.0*damping*scale,
            )).unwrap();
            let result = Engine::default()
                .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &rspice_core::NoAbort)
                .unwrap();
            let expected = Some(damping > 0.0);
            assert_eq!(
                result
                    .pole_evidence
                    .certificate()
                    .unwrap()
                    .asymptotically_stable,
                expected
            );
            let document = AnalysisResultDocument::from_pole_zero(
                AnalysisInstanceId::new(AnalysisKind::PoleZero, 0),
                &result,
            )
            .unwrap()
            .build()
            .unwrap();
            let json = serde_json::to_string(&document).unwrap();
            let restored: AnalysisResultDocument = serde_json::from_str(&json).unwrap();
            let ResultPayload::PoleZero(payload) = restored.payload() else {
                panic!("PZ payload");
            };
            assert_eq!(payload.to_pole_evidence(), result.pole_evidence);
        }
    }
}

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
