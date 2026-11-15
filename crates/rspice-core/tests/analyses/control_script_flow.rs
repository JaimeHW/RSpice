//! The control commands ngspice's scripted regression decks rely on, each
//! checked against what ngspice-46 prints for the same script: `goto` and
//! `label`, `@device[parameter]` in an expression, a vector `alter` value,
//! `remcirc`, and `stop` with `resume`.

use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlPresentation,
    ControlPresentationKind,
};
use rspice_core::execution::control::{ControlLimits, ControlProgram};
use rspice_core::{Engine, Netlist, NoAbort, SimulationConfig};

/// Run `source`'s script to its end or first error, keeping what it asked
/// to present.
fn run(source: &str) -> (ControlCircuit, Vec<ControlPresentation>, Option<String>) {
    let engine = Engine::new(SimulationConfig::default());
    let program =
        ControlProgram::parse_deck_with_abort(source, ControlLimits::default(), &NoAbort).unwrap();
    let netlist = Netlist::parse(program.declarative_source()).unwrap();
    let mut session = program.start(netlist.params.clone());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let mut presented = Vec::new();
    loop {
        let command = match session.next_command(&mut circuit.evaluator(&engine), &NoAbort) {
            Ok(Some(command)) => command,
            Ok(None) => return (circuit, presented, None),
            Err(error) => return (circuit, presented, Some(error.to_string())),
        };
        match circuit.execute(&engine, &command, session.variables(), &NoAbort) {
            Ok(ControlCommandEffect::Presentation(request)) => presented.push(request),
            Ok(_) => {}
            Err(error) => return (circuit, presented, Some(error.to_string())),
        }
    }
}

/// Every printed column, by expression, in print order.
fn printed(presented: &[ControlPresentation]) -> Vec<(String, Vec<f64>)> {
    presented
        .iter()
        .filter_map(|request| match &request.kind {
            ControlPresentationKind::Print(traces) => Some(traces),
            _ => None,
        })
        .flatten()
        .map(|trace| {
            let samples = trace.y.samples.iter().map(|sample| sample.re).collect();
            (trace.y.expression.clone(), samples)
        })
        .collect()
}

fn close(actual: &[f64], expected: &[f64], tolerance: f64) -> bool {
    actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| (actual - expected).abs() <= tolerance)
}

/// ngspice-46 prints i = 3, j = 10 and n = 2: the first `goto` jumps back to
/// a label the script has passed, and the second leaves the `while` for a
/// label before it, which ends that loop and enters it anew. (`op` gives
/// `print` a plot to print from.)
#[test]
fn goto_jumps_to_its_label_and_ends_the_loops_it_leaves() {
    let (_, presented, error) = run(
        "goto\nV1 1 0 1\nR1 1 0 1k\n.control\nop\nlet i = 0\nlabel again\nlet i = i + 1\n\
         if i < 3\ngoto again\nend\nlet j = 0\nlet n = 0\nlabel top\nlet n = n + 1\n\
         while j < 10\nlet j = j + 1\nif j = 4\ngoto top\nend\nend\nprint i j n\n.endc\n.end\n",
    );
    assert_eq!(error, None);
    let values = printed(&presented)
        .into_iter()
        .map(|(_, samples)| samples)
        .collect::<Vec<_>>();
    assert_eq!(values, [vec![3.0], vec![10.0], vec![2.0]]);
}

/// ngspice-46 prints @v1[pulse] as the seven coefficients the deck gave,
/// @dmod[is] = 2e-14 from the model card, and a saved @v1[dc] of a DC sweep
/// of v1 as the sweep's values.
#[test]
fn an_expression_reads_device_and_model_parameters() {
    let (_, presented, error) = run(
        "at reads\nv1 1 0 dc 0 pulse(0 1 1n 2n 3n 10n 20n)\nr1 1 0 1k\nd1 1 0 dmod\n\
         .model dmod d is=2e-14 n=1.5\n.control\nop\nlet p = @v1[pulse]\nprint p\n\
         let s = @dmod[is]\nprint s\nsave @v1[dc]\ndc v1 0 1 0.25\nlet foo = @v1[dc]\n\
         print foo\n.endc\n.end\n",
    );
    assert_eq!(error, None);
    let values = printed(&presented);
    assert_eq!(values.len(), 3, "{values:?}");
    assert!(
        close(
            &values[0].1,
            &[0.0, 1.0, 1e-9, 2e-9, 3e-9, 10e-9, 20e-9],
            1e-21
        ),
        "{values:?}"
    );
    assert!(close(&values[1].1, &[2e-14], 1e-26), "{values:?}");
    assert!(
        close(&values[2].1, &[0.0, 0.25, 0.5, 0.75, 1.0], 1e-12),
        "{values:?}"
    );
}

/// A bracketed `alter` value gives the source the waveform its parameter
/// names. ngspice-46 prints the new coefficients and its transient reaches
/// 2 V at 2 ns, the new pulse's top.
#[test]
fn a_vector_alter_value_replaces_the_source_waveform() {
    let (circuit, presented, error) = run(
        "vector alter\nv1 1 0 dc 0 pulse(0 1 1n 2n 3n 10n 20n)\nr1 1 0 1k\n.control\nop\n\
         alter @v1[pulse] = [ 0 2 1n 1n 1n 5n 10n ]\nlet p = @v1[pulse]\nprint p\n\
         tran 0.5n 10n\n.endc\n.end\n",
    );
    assert_eq!(error, None);
    let values = printed(&presented);
    assert!(
        close(
            &values[0].1,
            &[0.0, 2.0, 1e-9, 1e-9, 1e-9, 5e-9, 10e-9],
            1e-21
        ),
        "{values:?}"
    );
    let ControlAnalysisResult::Transient(result) = &circuit.datasets()[1].result else {
        panic!("expected a transient");
    };
    let node = result
        .node_names
        .iter()
        .position(|name| name == "1")
        .unwrap();
    for (time, voltage) in result.time.iter().zip(&result.voltages[node]) {
        if (2e-9..=7e-9).contains(time) {
            assert!((voltage - 2.0).abs() < 1e-9, "v(1) = {voltage} at {time}");
        }
    }
}

/// ngspice-46 prints v(1) = 2 after `remcirc`, ignores a second `remcirc`
/// with a warning, and refuses `op` with no circuit loaded.
#[test]
fn remcirc_keeps_the_plots_and_removes_the_circuit() {
    let (_, presented, error) = run(
        "remcirc\nV1 1 0 2\nR1 1 0 1k\n.control\nop\nremcirc\nprint v(1)\nremcirc\nop\n\
         .endc\n.end\n",
    );
    assert!(close(&printed(&presented)[0].1, &[2.0], 1e-12));
    assert!(
        presented
            .iter()
            .any(|request| matches!(request.kind, ControlPresentationKind::Ignored(_)))
    );
    let error = error.expect("op with no circuit is refused");
    assert!(error.contains("op: no circuit loaded"), "{error}");
}

/// Two stops interrupt one transient, and each `resume` continues it into
/// the same plot with what `alter` changed. ngspice-46 prints the plot
/// ending at 0.5 ms, 1 ms and 2 ms, and v(2) = 1.967358 at the first stop
/// and -2.43686 at the end: an RC charged from 5 V for 0.5 ms (tau 1 ms),
/// driven to -5 V for 0.5 ms, then through 2k (tau 2 ms) for 1 ms.
#[test]
fn stop_interrupts_a_transient_and_resume_continues_it_altered() {
    let (circuit, presented, error) = run(
        "stop resume\nv1 1 0 dc 5\nr1 1 2 1k\nc2 2 0 1u\n.control\nstop when time = 0.5ms\n\
         stop when time=1ms\ntran 10u 2ms uic\nprint time[length(time)-1] v(2)[length(time)-1]\n\
         alter v1=-5\nresume\nprint time[length(time)-1]\nalter r1=2k\nresume\n.endc\n.end\n",
    );
    assert_eq!(error, None);
    let values = printed(&presented);
    assert!(close(&values[0].1, &[0.5e-3], 1e-15), "{values:?}");
    assert!(close(&values[1].1, &[1.967358], 1e-4), "{values:?}");
    assert!(close(&values[2].1, &[1e-3], 1e-15), "{values:?}");
    assert_eq!(circuit.datasets().len(), 1);
    assert!(!circuit.is_paused("tran1"));
    let ControlAnalysisResult::Transient(result) = &circuit.datasets()[0].result else {
        panic!("expected a transient");
    };
    assert!(result.time.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(result.time.contains(&0.5e-3) && result.time.contains(&1e-3));
    assert_eq!(result.time.last(), Some(&2e-3));
    let node = result
        .node_names
        .iter()
        .position(|name| name == "2")
        .unwrap();
    let end = *result.voltages[node].last().unwrap();
    assert!((end + 2.43686).abs() < 1e-4, "v(2) = {end} at 2 ms");
}

/// ngspice's `resume` with no run in progress starts one, as `run` does
/// (frontend/runcoms2.c:86).
#[test]
fn resume_with_no_interrupted_run_runs_the_deck() {
    let (circuit, _, error) = run(
        "resume\nv1 1 0 dc 1\nr1 1 2 1k\nc2 2 0 1u\n.tran 10u 1ms\n.control\nresume\n.endc\n.end\n",
    );
    assert_eq!(error, None);
    assert_eq!(circuit.datasets().len(), 1);
    assert_eq!(circuit.datasets()[0].name, "tran1");
}
