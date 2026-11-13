use rspice_core::abort_signal::CountingAbort;
use rspice_core::engine::SimulationError;
use rspice_core::{Engine, Netlist};

#[test]
fn passive_and_xspice_model_construction_observes_abort() {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=16 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    for body in [
        ".MODEL rm R(RSH={TEMP+f16()})\nR2 out 0 rm L=1 W=1",
        ".MODEL cm C(CJ={TEMP+f16()})\nC1 out 0 cm L=1 W=1",
        ".MODEL lm L(L={TEMP+f16()})\nL1 aux 0 1m lm\nR2 out aux 1k",
        ".MODEL amp gain(GAIN={TEMP+f16()})\nA1 out aux amp",
        ".MODEL lut pwl(X_ARRAY=[0 1] Y_ARRAY=[0 {TEMP+f16()}])\nA1 out aux lut",
    ] {
        let netlist = Netlist::parse(&format!(
            "* cancelled construction\n{functions}V1 out 0 .2\nR1 out 0 1k\n{body}\n.END\n"
        ))
        .unwrap();
        let abort = CountingAbort::new(256);
        let result = Engine::default().build_circuit_with_abort(&netlist, &abort);
        assert!(
            matches!(result, Err(SimulationError::Aborted)),
            "{body}: result {:?}, {} polls",
            result.as_ref().err(),
            abort.count()
        );
        assert_eq!(abort.polls_after_abort(), 0, "{body}");
    }
}
