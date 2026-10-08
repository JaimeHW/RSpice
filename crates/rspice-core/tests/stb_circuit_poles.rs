//! Circuit stability is evidence about every natural mode, including hidden modes.
use rspice_core::analysis::pole_zero::StabilityVerdict;
use rspice_core::analysis::stb::{
    CircuitPoleEvidence, CircuitPoleFailure, StbConfig, StbSweepType,
};
use rspice_core::{Engine, Netlist, NoAbort, ResourceLimits};

const LOOP: &str = "E1 eo 0 ctrl 0 -10\nVPROBE eo x 0\nR1 x ctrl 1k\nC1 ctrl 0 1u\n";

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stable_loop_margins_do_not_hide_a_lossless_natural_mode() {
    let extra = "Lhidden1 a 0 1\nLhidden2 b 0 3\nChidden1 a 0 3\nChidden2 b 0 5\nCcouple a b 4\n";
    let result = Engine::default()
        .run_stb(&netlist(extra), config())
        .unwrap();
    assert_eq!(
        result.result.circuit_poles.spectrum().unwrap().poles.len(),
        5
    );
    assert_eq!(
        result.result.stability_verdict(),
        StabilityVerdict::Unstable
    );
}
fn config() -> StbConfig {
    StbConfig::new()
        .with_sweep(10.0, 10000.0, 3)
        .with_sweep_type(StbSweepType::Linear)
        .with_probe("VPROBE")
        .with_nyquist(true)
}
fn netlist(extra: &str) -> Netlist {
    Netlist::parse(&format!("Loop\n{LOOP}{extra}\n.end\n")).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stb_reports_closed_loop_modes_and_detects_a_hidden_unstable_state() {
    let stable = Engine::default().run_stb(&netlist(""), config()).unwrap();
    let spectrum = stable.result.circuit_poles.spectrum().unwrap();
    assert_eq!(spectrum.poles.len(), 1);
    assert!((spectrum.poles[0].re + 11000.0).abs() < 1e-7);
    assert_eq!(stable.result.stability_verdict(), StabilityVerdict::Stable);
    let hidden = Engine::default()
        .run_stb(
            &netlist("Ghidden hidden 0 hidden 0 -1\nChidden hidden 0 1"),
            config(),
        )
        .unwrap();
    assert_eq!(hidden.loop_gains, stable.loop_gains);
    assert_eq!(hidden.result.margins, stable.result.margins);
    assert_eq!(
        hidden.result.stability_verdict(),
        StabilityVerdict::Unstable
    );
    assert_eq!(
        hidden.result.circuit_poles.spectrum().unwrap().poles.len(),
        2
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stb_retains_unsupported_and_resource_limited_pole_evidence() {
    let unsupported = Engine::default()
        .run_stb(&netlist("B1 ctrl 0 I={FREQ*1p*V(ctrl)}"), config())
        .unwrap();
    assert!(matches!(
        unsupported.result.circuit_poles,
        CircuitPoleEvidence::Unavailable {
            cause: CircuitPoleFailure::Unsupported { .. }
        }
    ));
    assert_eq!(
        unsupported.result.stability_verdict(),
        StabilityVerdict::Indeterminate
    );
    assert_eq!(unsupported.loop_gains.len(), 3);
    let mut engine_config = rspice_core::engine::SimulationConfig::default();
    engine_config.resource_limits.max_result_values = 3 * 12 + 10;
    let limited = Engine::new(engine_config)
        .run_stb(&netlist(""), config())
        .unwrap();
    let CircuitPoleEvidence::Unavailable {
        cause:
            CircuitPoleFailure::ResourceLimit {
                resource,
                requested,
                limit,
            },
    } = &limited.result.circuit_poles
    else {
        panic!("{:?}", limited.result.circuit_poles)
    };
    assert_eq!(resource, "result_values");
    assert_eq!(*limit, 46);
    assert!(*requested > *limit);
    assert_eq!(
        limited.result.stability_verdict(),
        StabilityVerdict::Indeterminate
    );
    limited
        .result
        .validate_with_abort(&ResourceLimits::default(), &NoAbort)
        .unwrap();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stb_diagnostic_byte_limit_keeps_its_typed_resource_error() {
    let mut engine_config = rspice_core::engine::SimulationConfig::default();
    engine_config.resource_limits.max_external_data_bytes = 0;
    let error = Engine::new(engine_config)
        .run_stb(&netlist("B1 ctrl 0 I={FREQ*1p*V(ctrl)}"), config())
        .unwrap_err();
    assert!(
        matches!(error, rspice_core::SimulationError::ResourceLimit(limit)
        if limit.resource == rspice_core::ResourceKind::ExternalDataBytes
            && limit.limit == 0 && limit.requested > 0)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stb_uses_the_accepted_nonlinear_capacitance_bias() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::engine::{SimulationConfig, SpiceDialect};
    use rspice_core::netlist::NetlistParseOptions;
    let source = format!(
        "Biased loop\n{}I1 0 ctrl 11m\n.end\n",
        LOOP.replace("C1 ctrl 0 1u", "C1 ctrl 0 C={1u*(1+V(ctrl))}")
    );
    let netlist = Netlist::parse_with_options(
        &source,
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap();
    let result = Engine::new_with_resolved_config(
        SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce),
    )
    .run_stb(&netlist, config())
    .unwrap();
    let poles = &result.result.circuit_poles.spectrum().unwrap().poles;
    assert_eq!(poles.len(), 1);
    assert!((poles[0].re + 5500.0).abs() < 1e-6, "{poles:?}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn retained_stb_spectra_validate_and_legacy_results_stay_indeterminate() {
    let result = Engine::default()
        .run_stb(&netlist(""), config())
        .unwrap()
        .result;
    let mut json = serde_json::to_value(&result).unwrap();
    let roundtrip: rspice_core::analysis::stb::StbResult =
        serde_json::from_value(json.clone()).unwrap();
    assert_eq!(roundtrip, result);
    roundtrip
        .validate_with_abort(&ResourceLimits::default(), &NoAbort)
        .unwrap();
    use rspice_core::execution::{
        AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, ResultPayload,
    };
    let document = AnalysisResultDocument::from_stability(
        AnalysisInstanceId::new(AnalysisKind::Stb, 0),
        &result,
    )
    .unwrap()
    .build()
    .unwrap();
    let decoded = AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap();
    let ResultPayload::Stb(payload) = decoded.payload() else {
        panic!()
    };
    assert_eq!(payload.circuit_poles, result.circuit_poles);
    let mut old_document = serde_json::to_value(&document).unwrap();
    old_document["schemaVersion"] = serde_json::json!(15);
    assert!(
        AnalysisResultDocument::from_json(&serde_json::to_string(&old_document).unwrap()).is_err()
    );
    old_document["payload"]
        .as_object_mut()
        .unwrap()
        .remove("circuitPoles");
    let old_document =
        AnalysisResultDocument::from_json(&serde_json::to_string(&old_document).unwrap()).unwrap();
    let ResultPayload::Stb(old_payload) = old_document.payload() else {
        panic!()
    };
    assert!(old_payload.circuit_poles.is_not_computed());
    json.as_object_mut().unwrap().remove("circuit_poles");
    let legacy: rspice_core::analysis::stb::StbResult = serde_json::from_value(json).unwrap();
    assert_eq!(legacy.stability_verdict(), StabilityVerdict::Indeterminate);
    let mut corrupted = result;
    let CircuitPoleEvidence::Available { spectrum } = &mut corrupted.circuit_poles else {
        panic!()
    };
    spectrum.poles.clear();
    assert!(
        corrupted
            .validate_with_abort(&ResourceLimits::default(), &NoAbort)
            .is_err()
    );
}
