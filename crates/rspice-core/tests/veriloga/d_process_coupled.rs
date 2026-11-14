//! A `d_process` peer beside a Verilog module, in a coupled circuit trial.

use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;
use std::fs;
use std::path::PathBuf;

use crate::toggle_peer::{
    ProcessExchangeLog, ProcessFactoryGuard, TOGGLE_PEER_DECK, assert_toggle_peer_matches_ngspice,
};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

/// A `d_process` whose input a Verilog module drives from its output takes
/// part in every coupled circuit trial. The process protocol has no undo, so
/// each exchange a trial makes is journaled; the circuit rolls the trial back
/// and the acceptance at the same time, with the same inputs, replays the
/// answer. The peer is still asked exactly once per rising edge, and `d`
/// matches ngspice-46 (the module as an XSPICE inverter) at every event, as
/// the uncoupled deck does. The transient used to stop at the first edge with
/// "d_process cannot participate in a circuit trial: its external process has
/// no rollback protocol".
#[test]
fn d_process_beside_a_verilog_module_replays_rolled_back_trials_from_its_journal() {
    let dir = unique_temp_dir("rspice-d-process-coupled");
    fs::create_dir_all(&dir).expect("create the module's directory");
    let module = dir.join("inv.v");
    fs::write(
        &module,
        "`timescale 1ns/1ps\n\
         module inv(d, q);\n  input d; output q; reg q;\n  always @(d) q = ~d;\nendmodule\n",
    )
    .expect("write the module");
    let deck = TOGGLE_PEER_DECK.replace("aproc [dosc]", "aproc [q]");
    let netlist = Netlist::parse(&format!(
        "d_process beside a Verilog module\n.hdl \"{}\"\nxinv d q inv\n{deck}.end\n",
        module.display().to_string().replace('\\', "/")
    ))
    .expect("coupled d_process deck parses");

    let exchanges = ProcessExchangeLog::default();
    let _factory = ProcessFactoryGuard::toggle_peer(&exchanges);
    let engine = Engine::default();
    let result = engine
        .run_tran(&netlist, 20.0e-6, 400.0e-9)
        .expect("the coupled d_process transient runs to its stop time");
    let _ = fs::remove_dir_all(dir);

    // The module inverts d, so the process reads the complement of the bit it
    // last answered; d is unknown before the first edge, which the process
    // protocol packs as its seeded pseudo-random bit.
    let first = exchanges.lock().expect("lock the exchange log")[0]
        .1
        .clone();
    assert_toggle_peer_matches_ngspice(&result, &exchanges, |edge| {
        if edge == 0 {
            first.clone()
        } else {
            vec![u8::from(edge % 2 == 0)]
        }
    });
}

/// A Verilog observer of `d` reads no node voltage, so the coupled deck is
/// stepped as the deck without it is, but for the landings of its coupled
/// events.
///
/// Any mixed module used to keep a deck with code models on the generic
/// node-voltage estimator, which only a module that dates a crossing of a
/// node voltage needs. Over 20 us the deck took 2,166 points with the
/// observer and 593 without it; over 2 ms, 217,986 and 58,013, where
/// ngspice-46 takes 46,642 with the module as an XSPICE inverter. The
/// observer now leaves it 738.
#[test]
fn a_verilog_observer_of_an_event_net_leaves_the_d_process_deck_its_steps() {
    let dir = unique_temp_dir("rspice-d-process-observer");
    fs::create_dir_all(&dir).expect("create the module's directory");
    let module = dir.join("obs.v");
    fs::write(
        &module,
        "`timescale 1ns/1ps\n\
         module obs(d, q);\n  input d; output q; reg q;\n  always @(d) q = d;\nendmodule\n",
    )
    .expect("write the module");
    let observed = Netlist::parse(&format!(
        "d_process with a Verilog observer\n.hdl \"{}\"\nxobs d q obs\n{TOGGLE_PEER_DECK}.end\n",
        module.display().to_string().replace('\\', "/")
    ))
    .expect("observed d_process deck parses");
    let alone = Netlist::parse(&format!("d_process alone\n{TOGGLE_PEER_DECK}.end\n"))
        .expect("d_process deck parses");

    let engine = Engine::default();
    let mut points = Vec::new();
    for netlist in [&alone, &observed] {
        let exchanges = ProcessExchangeLog::default();
        let _factory = ProcessFactoryGuard::toggle_peer(&exchanges);
        let result = engine
            .run_tran(netlist, 20.0e-6, 400.0e-9)
            .expect("the d_process transient runs to its stop time");
        // The process reads the clock, high at every edge it is asked at.
        assert_toggle_peer_matches_ngspice(&result, &exchanges, |_| vec![1]);
        points.push(result.time.len());
    }
    let _ = fs::remove_dir_all(dir);
    assert!(
        points[1] as f64 <= 1.5 * points[0] as f64,
        "the observed deck took {} points, the deck alone {}",
        points[1],
        points[0]
    );
}
