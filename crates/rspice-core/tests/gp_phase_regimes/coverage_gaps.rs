use super::*;

/// Smooth prescribed forcing has numerical coverage in behavioral.rs.
/// Switched, branch-current-dependent and stateful providers remain C03d work.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_behavioral_dynamic_and_switched_event_gaps_remain_explicit() {
    for expression in [
        ".7+1u*if(time>1n,1,0)",
        ".7+1u*abs(time/1n-1)",
        ".7+1u*i(VC)",
        ".7+sdt(1u)",
    ] {
        let source = Netlist::parse(
            &include_str!("../testdata/qualification/gp-behavioral-event-gap.cir")
                .replace(".7+1u*sin(2*pi*1e9*time)", expression),
        )
        .unwrap();
        let engine = Engine::new(SimulationConfig {
            gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
            ..SimulationConfig::default()
        });
        let error = engine.run_tran(&source, 2e-9, 4e-12).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("requires a smooth prescribed physical time equation"),
            "{expression}: {error}"
        );
    }
}
