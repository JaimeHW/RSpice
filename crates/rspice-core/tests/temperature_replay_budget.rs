//! Failed discovery preserves the budget for successful temperature confirmation.
use rspice_core::Netlist;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn newly_reachable_random_operands_reconcile_before_publication() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse(
        "Newly reachable draw\n.options seed=37 temp={scale/(TNOM-27)+aunif(0,1)} tnom={nominal+aunif(0,1)}\n.param scale=1 nominal=55\n.end\n",
    ).unwrap();
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
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn recovered_child_draws_keep_the_phase_of_later_root_temperature_options() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse(
        "Scoped reachable draw\n.options seed=37\n.subckt child p\n.options temp={scale/(TNOM-27)+aunif(0,1)}\n.param scale=1\n.ends\n.options tnom={nominal+aunif(0,1)}\n.param nominal=55\n.end\n",
    ).unwrap();
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
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn eager_parameter_recovery_also_gets_a_complete_confirmation_budget() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    let netlist = Netlist::parse(
        "Eager reachable draw\n.options seed=37\n.param scale={1/(TNOM-27)+aunif(0,1)} nominal={55+aunif(0,1)}\n.options temp={scale} tnom={nominal}\n.end\n",
    ).unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let operand = eval_expression("aunif(0,1)", &expected).unwrap();
    let nominal = 55.0 + eval_expression("aunif(0,1)", &expected).unwrap();
    assert_eq!(netlist.options.temp, Some(1.0 / (nominal - 27.0) + operand));
    assert_eq!(netlist.options.tnom, Some(nominal));
    assert_eq!(netlist.params.get("scale"), netlist.options.temp);
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        eval_expression("aunif(0,1)", &expected).unwrap()
    );
}
