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

/// A Verilog observer of `d` reads no node voltage and drives nothing the
/// analog side reads, so the coupled deck is stepped as the deck without it
/// is: the same accepted points, at the same times and with the same `V(out)`
/// to within the instants the coupled queue keys its events on. A coupled run
/// keys a code model's output on the HDL's tick lattice, so `d` moves at
/// exactly 501 ns where the deck alone dates it at the sum 500 ns + 1 ns,
/// 5.009999999999999e-7; the step sequence carries that ulp on, and nowhere
/// does it reach the femtosecond the events are checked against ngspice to.
///
/// Any mixed module used to keep a deck with code models on the generic
/// node-voltage estimator, which only a module that dates a crossing of a
/// node voltage needs: over 20 us the deck took 2,166 points with an observer
/// and 593 without it. Then each clock edge, which the stepper lands on
/// because a coupled queue's events are exact landings, also restarted the
/// integrator, though the clock reaches only the process and nothing analog
/// steps there. A coupled event whose readers drive no analog output now
/// lands without a restart, as the uncoupled queue's does.
///
/// An observer that drives its own output node `q` gets a converter there,
/// whose zero-duration edges `q` steps on: one completion per `d` edge, and
/// the controller's resume after it, is all it adds.
#[test]
fn a_verilog_observer_of_an_event_net_leaves_the_d_process_deck_its_steps() {
    let dir = unique_temp_dir("rspice-d-process-observer");
    fs::create_dir_all(&dir).expect("create the modules' directory");
    let observer = dir.join("obs.v");
    fs::write(
        &observer,
        "`timescale 1ns/1ps\n\
         module obs(d);\n  input d; reg q;\n  always @(d) q = d;\nendmodule\n",
    )
    .expect("write the observer");
    let follower = dir.join("follow.v");
    fs::write(
        &follower,
        "`timescale 1ns/1ps\n\
         module follow(d, q);\n  input d; output q; reg q;\n  always @(d) q = d;\nendmodule\n",
    )
    .expect("write the follower");
    let deck = |title: &str, module: Option<(&PathBuf, &str)>| {
        let hdl = module.map_or_else(String::new, |(path, card)| {
            format!(
                ".hdl \"{}\"\n{card}\n",
                path.display().to_string().replace('\\', "/")
            )
        });
        Netlist::parse(&format!("{title}\n{hdl}{TOGGLE_PEER_DECK}.end\n"))
            .expect("the d_process deck parses")
    };
    let decks = [
        deck("d_process alone", None),
        deck("d_process observed", Some((&observer, "xobs d obs"))),
        deck("d_process followed", Some((&follower, "xfol d q follow"))),
    ];

    let engine = Engine::default();
    let mut runs = Vec::new();
    for netlist in &decks {
        let exchanges = ProcessExchangeLog::default();
        let _factory = ProcessFactoryGuard::toggle_peer(&exchanges);
        let result = engine
            .run_tran(netlist, 20.0e-6, 400.0e-9)
            .expect("the d_process transient runs to its stop time");
        // The process reads the clock, high at every edge it is asked at.
        assert_toggle_peer_matches_ngspice(&result, &exchanges, |_| vec![1]);
        let out = result.node_index_named("out").expect("V(out) is recorded");
        let volts = result
            .try_voltage_waveform(out)
            .expect("node index in range")
            .to_vec();
        let d_edges = result.digital_trace_named("d").map_or(0, <[_]>::len);
        runs.push((result.time.clone(), volts, d_edges));
    }
    let _ = fs::remove_dir_all(dir);
    let [
        (alone_times, alone_volts, d_edges),
        (observed_times, observed_volts, _),
        (followed_times, ..),
    ] = &runs[..]
    else {
        unreachable!("three runs");
    };
    assert_eq!(
        observed_times.len(),
        alone_times.len(),
        "the observed deck took {} points, the deck alone {}",
        observed_times.len(),
        alone_times.len()
    );
    for (index, (observed, alone)) in observed_times.iter().zip(alone_times).enumerate() {
        assert!(
            (observed - alone).abs() < 1.0e-15,
            "point {index}: observed at {observed:e} s, alone at {alone:e} s"
        );
        let (observed_v, alone_v) = (observed_volts[index], alone_volts[index]);
        assert!(
            (observed_v - alone_v).abs() < 1.0e-9,
            "V(out) at point {index}: observed {observed_v}, alone {alone_v}"
        );
    }
    assert!(
        followed_times.len() <= alone_times.len() + 3 * d_edges,
        "the followed deck took {} points, the deck alone {} over {d_edges} edges of d",
        followed_times.len(),
        alone_times.len()
    );
}
