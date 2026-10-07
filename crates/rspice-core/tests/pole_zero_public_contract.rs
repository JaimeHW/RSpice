//! Public pole-zero completion and physical port identity, on both solver paths.
use rspice_core::engine::NodeResolver;
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, ResultPayload, SignalUnit,
};
use rspice_core::{AbortSignal, Engine, Netlist, NoAbort, SimulationError};
use std::sync::atomic::{AtomicBool, Ordering};

const RESISTOR: &str = "Static PZ\nR1 out 0 1k\n.pz out 0 out 0 cur pz\n.end\n";
const RC: &str = "Dynamic PZ\nR1 out 0 1k\nC1 out 0 1u\n.pz out 0 out 0 cur pz\n.end\n";

fn document(source: &str, expected_gain_unit: SignalUnit) -> AnalysisResultDocument {
    let netlist = Netlist::parse(source).unwrap();
    let result = Engine::default()
        .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
        .unwrap();
    assert_eq!(result.gain_unit, expected_gain_unit);
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
    assert_eq!(payload.root_unit, Some(SignalUnit::RadianPerSecond));
    assert_eq!(payload.gain_unit, Some(expected_gain_unit));
    document
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_and_voltage_gains_keep_their_units_without_rescaling_roots() {
    for source in [RESISTOR, RC] {
        let result = document(source, SignalUnit::Ohm);
        let ResultPayload::PoleZero(payload) = result.payload() else {
            panic!("PZ");
        };
        assert!((payload.dc_gain.unwrap() - 1000.0).abs() < 1e-8);
        if source == RC {
            assert_eq!(payload.poles.len(), 1);
            assert!((payload.poles[0].real + 1000.0).abs() < 1e-8);
        }
    }
    for reference in ["0", "ref"] {
        let shunt = if reference == "0" {
            ""
        } else {
            "Rref ref 0 1k\n"
        };
        let source = format!(
            "Voltage RC\nV1 in {reference} 1\nR1 in out 1k\nC1 out {reference} 1u\n{shunt}.pz in {reference} out {reference} vol pz\n.end\n"
        );
        let result = document(&source, SignalUnit::Dimensionless);
        let ResultPayload::PoleZero(payload) = result.payload() else {
            panic!("PZ");
        };
        assert!((payload.dc_gain.unwrap() - 1.0).abs() < 1e-12);
        assert_eq!(payload.poles.len(), 1);
        assert!((payload.poles[0].real + 1000.0).abs() < 1e-8);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn legacy_pole_zero_documents_preserve_absent_units_and_numeric_values() {
    let original = document(RC, SignalUnit::Ohm);
    let mut encoded = serde_json::to_value(&original).unwrap();
    encoded["payload"]
        .as_object_mut()
        .unwrap()
        .remove("rootUnit");
    encoded["payload"]
        .as_object_mut()
        .unwrap()
        .remove("gainUnit");
    for version in 1..=11 {
        encoded["schemaVersion"] = version.into();
        let decoded = AnalysisResultDocument::from_json(&encoded.to_string()).unwrap();
        let ResultPayload::PoleZero(payload) = decoded.payload() else {
            panic!("PZ");
        };
        assert_eq!(payload.root_unit, None);
        assert_eq!(payload.gain_unit, None);
        assert_eq!(payload.dc_gain, Some(1000.0));
        assert!((payload.poles[0].real + 1000.0).abs() < 1e-8);
        let restored = serde_json::to_value(&decoded).unwrap();
        assert!(restored["payload"].get("rootUnit").is_none());
        assert!(restored["payload"].get("gainUnit").is_none());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn pole_zero_unit_metadata_cannot_lie_about_its_version_or_quantity() {
    let original = serde_json::to_value(document(RC, SignalUnit::Ohm)).unwrap();
    assert_eq!(original["payload"]["rootUnit"]["unit"], "radian_per_second");
    assert_eq!(original["payload"]["gainUnit"]["unit"], "ohm");
    for (field, value) in [("rootUnit", "hertz"), ("gainUnit", "volt")] {
        let mut encoded = original.clone();
        encoded["payload"][field]["unit"] = value.into();
        assert!(AnalysisResultDocument::from_json(&encoded.to_string()).is_err());
    }
    for field in ["rootUnit", "gainUnit"] {
        let mut encoded = original.clone();
        encoded["payload"].as_object_mut().unwrap().remove(field);
        assert!(AnalysisResultDocument::from_json(&encoded.to_string()).is_err());
    }
    let mut encoded = original;
    encoded["schemaVersion"] = 11.into();
    assert!(AnalysisResultDocument::from_json(&encoded.to_string()).is_err());
}

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
