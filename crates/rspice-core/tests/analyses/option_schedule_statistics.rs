//! Schedule look-ahead must classify operands without sampling them.
use rspice_core::Netlist;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn schedule_lookahead_never_samples_operands() {
    use rspice_core::netlist::expr::{ParamContext, eval_expression};
    for package in ["OUTPUT", "RESTART"] {
        let netlist = Netlist::parse(&format!("Schedule draws\n.options seed=37\n.options {package} INITIAL_INTERVAL=.1 {{aunif(10,1)}} {{aunif(1,.1)}}\n.param next={{aunif(0,1)}}\n.end\n")).unwrap();
        let mut expected = ParamContext::new();
        expected.set_random_seed(37);
        let time = eval_expression("aunif(10,1)", &expected).unwrap();
        let interval = eval_expression("aunif(1,.1)", &expected).unwrap();
        let actual = if package == "OUTPUT" {
            let point = &netlist
                .options
                .output_interval_schedule
                .as_ref()
                .unwrap()
                .intervals[0];
            (point.time, point.interval)
        } else {
            let point = &netlist.options.restart.as_ref().unwrap().intervals[0];
            (point.time, point.interval)
        };
        assert_eq!(actual, (time, interval));
        assert_eq!(
            netlist.params.get("next"),
            Some(eval_expression("aunif(0,1)", &expected).unwrap())
        );
        assert_eq!(
            eval_expression("aunif(0,1)", &netlist.params).unwrap(),
            eval_expression("aunif(0,1)", &expected).unwrap()
        );
    }
}
