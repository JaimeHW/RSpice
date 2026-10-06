use super::*;

/// C03d remains open: the exact-delay physical-event owner must acquire real
/// behavioral F/Q and sided time equations before this refusal can become a
/// numerical comparison. Weil's ordinary transient path already runs it.
/// This disposition test prevents a missing stamp from turning into silent
/// success; it is not evidence that behavioral exact-delay support is done.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_behavioral_physical_event_gap_is_explicit() {
    let source = Netlist::parse(include_str!(
        "../testdata/qualification/gp-behavioral-event-gap.cir"
    ))
    .unwrap();
    for model in [
        GpTransientPhaseModel::ExactDelay,
        GpTransientPhaseModel::NgspiceWeil,
    ] {
        let engine = Engine::new(SimulationConfig {
            gp_transient_phase_model: model,
            ..SimulationConfig::default()
        });
        let result = engine.run_tran(&source, 2e-9, 4e-12);
        if model == GpTransientPhaseModel::ExactDelay {
            let error = result.unwrap_err();
            assert!(matches!(error, rspice_core::SimulationError::Circuit(_)));
            assert!(
                error
                    .to_string()
                    .contains("behavioral sources require a prepared physical event sampler"),
                "{error}"
            );
        } else {
            let result = result.unwrap();
            assert_eq!(result.time.last(), Some(&2e-9));
            assert!(
                result
                    .voltages
                    .iter()
                    .chain(&result.branch_currents)
                    .flatten()
                    .all(|value| value.is_finite())
            );
        }
    }
}
