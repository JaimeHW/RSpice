//! Public pole-zero completion and physical port identity, on both solver paths.
use rspice_core::engine::NodeResolver;
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, ResultPayload,
};
use rspice_core::{AbortSignal, Engine, Netlist, NoAbort, SimulationError};
use std::sync::atomic::{AtomicBool, Ordering};

const RESISTOR: &str = "Static PZ\nR1 out 0 1k\n.pz out 0 out 0 cur pz\n.end\n";
const RC: &str = "Dynamic PZ\nR1 out 0 1k\nC1 out 0 1u\n.pz out 0 out 0 cur pz\n.end\n";

struct StopAtCompletion(AtomicBool);
impl AbortSignal for StopAtCompletion {
    fn is_aborted(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    fn observe_progress(&self, fraction: f64) {
        if fraction == 1.0 {
            self.0.store(true, Ordering::Relaxed);
        }
    }
}

fn final_cancel(source: &str) {
    let netlist = Netlist::parse(source).unwrap();
    let abort = StopAtCompletion(AtomicBool::new(false));
    let result =
        Engine::default().run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &abort);
    assert!(abort.is_aborted(), "the solver reached completion");
    assert!(
        matches!(result, Err(SimulationError::Aborted)),
        "{result:?}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sparse_pole_zero_obeys_final_progress_cancellation() {
    final_cancel(RC);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn dense_pole_zero_obeys_final_progress_cancellation() {
    // No dynamic rows: sparse state-space reduction deliberately declines.
    final_cancel(RESISTOR);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_port_identity_survives_sparse_dense_and_document_projection() {
    for (source, count) in [(RESISTOR, 0), (RC, 1)] {
        let netlist = Netlist::parse(source).unwrap();
        let result = Engine::default()
            .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
            .unwrap();
        assert_eq!(result.input, "I(OUT,0)");
        assert_eq!(result.output, "V(OUT,0)");
        assert_eq!(result.poles.len(), count);
        assert!(result.zeros.is_empty());
        assert!(result.has_consistent_root_evidence());
        assert!((result.dc_gain.unwrap() - 1000.0).abs() < 1e-8);
        if let Some(pole) = result.poles.first() {
            assert!((pole.re + 1000.0).abs() < 1e-8 && pole.im == 0.0);
        }
        let document = AnalysisResultDocument::from_pole_zero(
            AnalysisInstanceId::new(AnalysisKind::PoleZero, 0),
            &result,
        )
        .unwrap()
        .build()
        .unwrap();
        let ResultPayload::PoleZero(payload) = document.payload() else {
            panic!("PZ");
        };
        assert_eq!(payload.input, "I(OUT,0)");
        assert_eq!(payload.output, "V(OUT,0)");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn numeric_and_hierarchical_differential_ports_keep_their_circuit_names() {
    for (source, names, labels) in [
        (
            "Numeric\nV1 7 0 1\nR1 7 9 1k\nC1 9 0 1u\n.pz 7 0 9 0 vol pz\n.end\n",
            ["7", "0", "9", "0"],
            ["V(7,0)", "V(9,0)"],
        ),
        (
            "Hierarchy\n.subckt filter p n\nR1 p mid 1k\nC1 mid n 1u\n.ends\nX1 in ref filter\nV1 in ref 1\nRref ref 0 1k\n.pz in ref x1.mid ref vol pz\n.end\n",
            ["in", "ref", "x1.mid", "ref"],
            ["V(IN,REF)", "V(X1.MID,REF)"],
        ),
    ] {
        let netlist = Netlist::parse(source).unwrap();
        let engine = Engine::default();
        let resolver = NodeResolver::build_with_abort(&engine, &netlist, &NoAbort).unwrap();
        let ports = names.map(|name| resolver.resolve(name, "test port").unwrap());
        let by_index = engine
            .run_pz_ports_with_abort(
                &netlist,
                ports[0],
                Some(ports[1]),
                ports[2],
                Some(ports[3]),
                false,
                true,
                true,
                &NoAbort,
            )
            .unwrap();
        let by_card = engine
            .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
            .unwrap();
        for result in [by_index, by_card] {
            assert_eq!(result.input, labels[0]);
            assert_eq!(result.output, labels[1]);
        }
    }
}
