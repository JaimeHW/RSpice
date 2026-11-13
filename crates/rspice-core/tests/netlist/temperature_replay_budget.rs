//! Failed discovery preserves the budget for successful temperature confirmation.
use rspice_core::Netlist;
use rspice_core::netlist::expr::{ParamContext, eval_expression};

/// Parses a `seed=37` deck whose TEMP is `1/(TNOM-27)` plus the first draw and
/// whose TNOM is 55 plus the second, then checks the next draw keeps the phase.
fn assert_seed37_temperature_draws(deck: &str) -> Netlist {
    let netlist = Netlist::parse(deck).unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let operand = eval_expression("aunif(0,1)", &expected).unwrap();
    let nominal = 55.0 + eval_expression("aunif(0,1)", &expected).unwrap();
    assert_eq!(netlist.options.temp, Some(1.0 / (nominal - 27.0) + operand));
    assert_eq!(netlist.options.tnom, Some(nominal));
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &expected).unwrap()
    );
    netlist
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn newly_reachable_random_operands_reconcile_before_publication() {
    assert_seed37_temperature_draws(
        "Newly reachable draw\n.options seed=37 temp={scale/(TNOM-27)+aunif(0,1)} tnom={nominal+aunif(0,1)}\n.param scale=1 nominal=55\n.end\n",
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn recovered_child_draws_keep_the_phase_of_later_root_temperature_options() {
    assert_seed37_temperature_draws(
        "Scoped reachable draw\n.options seed=37\n.subckt child p\n.options temp={scale/(TNOM-27)+aunif(0,1)}\n.param scale=1\n.ends\n.options tnom={nominal+aunif(0,1)}\n.param nominal=55\n.end\n",
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn eager_parameter_recovery_also_gets_a_complete_confirmation_budget() {
    let netlist = assert_seed37_temperature_draws(
        "Eager reachable draw\n.options seed=37\n.param scale={1/(TNOM-27)+aunif(0,1)} nominal={55+aunif(0,1)}\n.options temp={scale} tnom={nominal}\n.end\n",
    );
    assert_eq!(netlist.params.get("scale"), netlist.options.temp);
}
