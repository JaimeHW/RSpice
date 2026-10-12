//! A discrete event scheduled at the stop time is the stop time.
//!
//! `.tran 1n 100n` parses its stop time as `100 * 1e-9`, one ulp above the
//! nearest double to 100 ns, while a digital tick or a stimulus file's
//! `1e-7` converts to that nearest double. The two instants are the same; the
//! run must land the event on `tstop`, process it there, and end there, rather
//! than accept a point one ulp short and refuse the sliver after it.

use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::{AnalysisCommand, Netlist};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A scratch directory removed on drop.
struct Scratch(PathBuf);

impl Scratch {
    fn new(prefix: &str) -> Self {
        let dir = std::env::temp_dir().join(format!(
            "rspice_{prefix}_{}_{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create scratch directory");
        Self(dir)
    }

    fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, contents).expect("write scratch file");
        path
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The deck's `.tran` stop time and solver ceiling, as a frontend resolves
/// them.
fn tran_window(netlist: &Netlist) -> (f64, f64) {
    netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                ..
            } => Some((
                *stop,
                rspice_core::execution::resolve_transient_maximum_step(
                    *step, *stop, *start, *max_step,
                )
                .expect("valid .tran"),
            )),
            _ => None,
        })
        .expect("deck has a .tran card")
}

fn run(netlist: &Netlist) -> TransientResult {
    let (stop, max_step) = tran_window(netlist);
    Engine::new(SimulationConfig::default())
        .run_tran(netlist, stop, max_step)
        .unwrap_or_else(|error| panic!("transient reaches its stop time: {error}"))
}

fn assert_ends_at_stop(result: &TransientResult, stop: f64) {
    assert_eq!(
        result.time.last().copied(),
        Some(stop),
        "the stop time is the last accepted point"
    );
    assert!(
        result.time.windows(2).all(|pair| pair[1] > pair[0]),
        "accepted points strictly increase"
    );
}

/// The digital trace of `node`.
fn digital_trace(result: &TransientResult, node: &str) -> Vec<(f64, String)> {
    result
        .digital_trace_named(node)
        .unwrap_or_else(|| {
            panic!(
                "digital trace {node} in {:?}",
                result
                    .digital_traces
                    .iter()
                    .map(|trace| &trace.node_name)
                    .collect::<Vec<_>>()
            )
        })
        .iter()
        .map(|point| (point.time, point.value.to_ngspice_token()))
        .collect()
}

/// The module's digital output, read on its own port (its built-in
/// converter's event endpoint) rather than on the converted voltage.
#[cfg(feature = "veriloga")]
const Q_PORT: &str = "x1.q__event_0";

/// The corpus probe P06b module: a 10 ns `always` clock, so the tenth tick is
/// the stop time. `q` rises at the tick `c` reaches `rise`.
#[cfg(feature = "veriloga")]
fn counter(rise: u32) -> String {
    format!(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module cnt(p, n, q); inout p, n; electrical p, n; output q; reg q; integer c;\n\
         \x20 initial begin q = 0; c = 0; end\n\
         \x20 always #10 begin c = c + 1; q = (c == {rise}); end\n\
         \x20 analog I(p, n) <+ V(p, n)/1000.0;\n\
         endmodule\n"
    )
}

#[cfg(feature = "veriloga")]
fn p06b_deck(module: &std::path::Path, cards: &str) -> Netlist {
    Netlist::parse(&format!(
        "* P0.6b\n.hdl \"{}\"\nvp p 0 1\nx1 p 0 q cnt\nrp p 0 {{r}}\nrq q 0 10k\n{cards}\n.tran 1n 100n\n.end\n",
        module.display().to_string().replace('\\', "/")
    ))
    .expect("deck parses")
}

#[cfg(feature = "veriloga")]
#[test]
fn the_p06b_deck_lands_its_tenth_tick_on_the_stop_time() {
    let scratch = Scratch::new("p06b_tran");
    let module = scratch.write("cnt.vams", &counter(3));
    let netlist = p06b_deck(&module, ".param r=1k");
    let (stop, _) = tran_window(&netlist);
    // The tick and the stop time are one ulp apart: the case under test.
    assert_ne!(stop, 1.0e-7);
    assert_eq!(stop, 1.0e-7_f64.next_up());

    let result = run(&netlist);
    assert_ends_at_stop(&result, stop);
    let q = digital_trace(&result, Q_PORT);
    let rise = q
        .iter()
        .find(|(_, value)| value.starts_with('1'))
        .unwrap_or_else(|| panic!("q rises: {q:?}"));
    assert!(
        (rise.0 - 30.0e-9).abs() <= 1.0e-12,
        "q rises at the third tick: {q:?}"
    );
}

#[cfg(feature = "veriloga")]
#[test]
fn an_event_at_the_stop_time_is_processed_there() {
    let scratch = Scratch::new("p06b_last_tick");
    let module = scratch.write("cnt.vams", &counter(10));
    let netlist = p06b_deck(&module, ".param r=1k");
    let (stop, _) = tran_window(&netlist);

    let result = run(&netlist);
    assert_ends_at_stop(&result, stop);
    let q = digital_trace(&result, Q_PORT);
    let (time, value) = q.last().expect("q has samples");
    assert!(
        value.starts_with('1'),
        "the tenth tick sets q, at the stop time: {q:?}"
    );
    assert!(
        (time - stop).abs() <= 1.0e-12 && *time <= stop,
        "q's last event is the stop-time tick: {q:?}"
    );
}

#[cfg(feature = "veriloga")]
#[test]
fn every_step_point_of_the_p06b_deck_reaches_the_stop_time() {
    let scratch = Scratch::new("p06b_step");
    let module = scratch.write("cnt.vams", &counter(3));
    let netlist = p06b_deck(&module, ".param r=1k\n.step param r list 1k 2k 3k");
    let (stop, max_step) = tran_window(&netlist);
    let engine = Engine::new(SimulationConfig::default());
    let plan =
        rspice_core::execution::DeckPlan::from_netlist(&netlist, &engine.config().resource_limits)
            .expect("deck plan");
    let materializer = engine
        .prepare_deck_plan_materializer_with_abort(&netlist, &plan, &rspice_core::NoAbort)
        .expect("materializer");
    assert_eq!(materializer.len(), 3);
    for (index, r) in [1.0e3, 2.0e3, 3.0e3].into_iter().enumerate() {
        let (_, point, _, _) = materializer
            .materialize_run_with_abort(index, &rspice_core::NoAbort)
            .expect("materialize point")
            .into_parts();
        let result = engine
            .run_tran(&point, stop, max_step)
            .unwrap_or_else(|error| panic!("point {index} reaches its stop time: {error}"));
        assert_ends_at_stop(&result, stop);
        // 1 mA through the module plus 1 V / r through rp.
        let current = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("vp"))
            .map(|branch| *result.branch_currents[branch].last().expect("samples"))
            .expect("i(vp)");
        assert!(
            (current + 1.0e-3 + 1.0 / r).abs() <= 1.0e-9,
            "point {index}: i(vp) = {current}"
        );
    }
}

#[test]
fn an_xspice_event_at_the_stop_time_is_processed_there() {
    let scratch = Scratch::new("xspice_stop_event");
    scratch.write("stim.stim", "0 0s\n1e-7 1s\n");
    let deck = scratch.write(
        "deck.cir",
        "* d_source edge at the stop time\n\
         a_src [d] src\n\
         a_dac [d] [out] dac\n\
         .model src d_source (input_file=\"stim.stim\")\n\
         .model dac dac_bridge (out_low=0 out_high=1 out_undef=0.5 t_rise=1p t_fall=1p)\n\
         rload out 0 1k\n\
         .tran 1n 100n\n\
         .end\n",
    );
    let netlist = Netlist::parse_with_path(&fs::read_to_string(&deck).expect("deck"), &deck)
        .expect("deck parses");
    let (stop, _) = tran_window(&netlist);
    assert_eq!(stop, 1.0e-7_f64.next_up());

    let result = run(&netlist);
    assert_ends_at_stop(&result, stop);
    let d = digital_trace(&result, "d");
    let (time, value) = d.last().expect("d has samples");
    assert_eq!(value, "1s", "the stop-time edge is delivered: {d:?}");
    assert!(
        (time - stop).abs() <= 1.0e-12,
        "d's last event is the stop-time edge: {d:?}"
    );
}

#[test]
fn a_pulse_corner_at_the_stop_time_is_reached_exactly() {
    // A 10 ns period: the tenth rising edge's corner is the stop time,
    // computed by the waveform's own arithmetic.
    let netlist = Netlist::parse(
        "* PULSE corner at the stop time\n\
         v1 a 0 pulse(0 1 0 1n 1n 4n 10n)\n\
         v2 b 0 pulse(0 1 99n 1n 1n 1u 2u)\n\
         ra a 0 1k\n\
         rb b 0 1k\n\
         .tran 1n 100n\n\
         .end\n",
    )
    .expect("deck parses");
    let (stop, _) = tran_window(&netlist);

    let result = run(&netlist);
    assert_ends_at_stop(&result, stop);
    let value = |node: &str| {
        let index = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("{node} in {:?}", result.node_names));
        *result.voltages[index].last().expect("samples")
    };
    // The tenth period starts at the stop time: a sits at its initial value.
    assert!(
        value("a").abs() <= 1.0e-6,
        "v(a) at the stop time: {}",
        value("a")
    );
    // b completes its 1 ns rise exactly at the stop time.
    assert!(
        (value("b") - 1.0).abs() <= 1.0e-6,
        "v(b) at the stop time: {}",
        value("b")
    );
}
