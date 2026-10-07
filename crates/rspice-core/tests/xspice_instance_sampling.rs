use rspice_core::Netlist;
use rspice_core::netlist::{ParamContext, expr::eval_expression};

#[test]
fn failed_forward_instance_probes_do_not_advance_the_parameter_stream() {
    let mut reference = ParamContext::new();
    reference.set_random_seed(37);
    let expected = eval_expression("aunif(100,1)", &reference).unwrap();
    for (instance, declaration) in [
        ("A1 in out gain gain={aunif(100,1)+later}", ".PARAM later=0"),
        ("A1 in out gain gain=aunif(100,1)+later", ".PARAM later=0"),
        (
            "A1 in out gain gain={aunif(100,1)+later()}",
            ".FUNC later() {0}",
        ),
        (
            "A1 [in] print_param_types real_array=[0 {aunif(100,1)+later}]",
            ".PARAM later=0",
        ),
    ] {
        let source = format!(
            "* forward instance sample\n.OPTIONS SEED=37\n{instance}\n{declaration}\n\
             .PARAM marker={{aunif(100,1)}}\n.END\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        assert_eq!(
            netlist.params.get("marker").unwrap(),
            expected,
            "{instance}"
        );
    }
}
