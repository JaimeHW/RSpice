use rspice_core::abort_signal::CountingAbort;
use rspice_core::netlist::{Netlist, ParseWithAbortError};

#[test]
fn model_device_source_and_subcircuit_expressions_honor_cancellation() {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=18 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    for body in [
        ".MODEL dd D(IS={f18()})\nD1 out 0 dd",
        ".MODEL buffer d_buffer(rise_delay=f18()+0)",
        "A1 out aux gain gain={f18()}",
        "A1 out aux gain gain=f18()+0",
        "A1 [out] print_param_types real_array=[0 {f18()}]",
        "A1 [out] print_param_types complex=<f18() 1>",
        "A1 [out] print_param_types complex=<1 f18()>",
        "A1 [out] print_param_types complex_array=[<1 f18()>]",
        "R2 out 0 1k TEMP={f18()}",
        "R2 out 0 1k TEMP=f18()+0",
        "C1 out 0 {f18()}",
        "C1 out 0 1u {f18()}",
        "V2 a 0 SIN(0 1 1 {f18()})\nR2 a 0 1k",
        "I2 out 0 PULSE(0 1 {f18()})",
        ".SUBCKT cell a PARAMS: value={f18()}\nR2 a 0 {value}\n.ENDS\nX1 out cell",
        ".SUBCKT cell a PARAMS: value={later+f18()} later=1\nR2 a 0 {value}\n.ENDS\nX1 out cell",
    ] {
        let source = format!(
            "* eager expression cancellation\n{functions}V1 out 0 1\nR1 out 0 1k\n{body}\n.OP\n.END\n"
        );
        let abort = CountingAbort::new(1024);
        let result = Netlist::parse_with_abort(&source, &abort);
        assert!(
            matches!(result, Err(ParseWithAbortError::Aborted)),
            "{body}: {} polls; {:?}",
            abort.count(),
            result.as_ref().err()
        );
        assert_eq!(abort.count(), 1025, "{body}");
        assert_eq!(abort.polls_after_abort(), 0, "{body}");
    }
}
