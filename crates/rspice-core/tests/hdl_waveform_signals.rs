#![cfg(feature = "veriloga")]
//! HDL signals in a run's waveforms, on the C12 testbench (`full`): the
//! internal variables `.ESAVE` names, published as ordinary traces under
//! their instance paths.
//!
//! The testbench loads codes 0 to 15, one every 100 us, and 99 us into each
//! period reads V(vout) into `vk[k]`, then `mv = vk[k]`; `meas` is driven by
//! `assign meas = mv`, and `pass` is set to 1 at time zero and only cleared
//! by a step outside its window, which this design never takes.

use rspice_core::engine::TransientResult;
use rspice_core::xspice::DigitalState;
use rspice_core::{Engine, Netlist};

const STOP: f64 = 1.65e-3;
const STEP: f64 = 0.5e-6;

/// The C12 `full` deck with its `.ESAVE` list replaced by `esave`, parsed
/// where the deck is so that its includes resolve.
fn c12_full(esave: &str) -> Netlist {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus/c12-ldo-tb/deck/full.tran.cir");
    let text = std::fs::read_to_string(&path).expect("the deck reads");
    assert!(text.contains(".esave xtb.ok__event_0\n"), "the deck's list");
    let text = text.replace(".esave xtb.ok__event_0\n", &format!("{esave}\n"));
    Netlist::parse_with_path(&text, &path).unwrap_or_else(|error| panic!("parses: {error}"))
}

fn run(esave: &str) -> TransientResult {
    Engine::default()
        .run_tran(&c12_full(esave), STOP, STEP)
        .unwrap_or_else(|error| panic!("{esave}: {error}"))
}

fn logic(result: &TransientResult, name: &str) -> Vec<(f64, DigitalState)> {
    result
        .digital_trace_named(name)
        .unwrap_or_else(|| panic!("no digital trace {name}"))
        .iter()
        .map(|point| (point.time, point.value.state))
        .collect()
}

fn real(result: &TransientResult, name: &str) -> Vec<(f64, f64)> {
    result
        .real_trace_named(name)
        .unwrap_or_else(|| panic!("no real trace {name}"))
        .iter()
        .map(|point| (point.time, point.value))
        .collect()
}

/// The bit a logic trace holds at `time`: its last point at or before it.
fn level_at(trace: &[(f64, DigitalState)], time: f64) -> DigitalState {
    trace
        .iter()
        .rev()
        .find(|(at, _)| *at <= time)
        .map(|(_, state)| *state)
        .expect("a value by then")
}

/// `.esave xtb.pass xtb.mv` publishes the testbench's verdict register and
/// its real variable as traces named by instance path. `D(xtb.pass)` is 1
/// from time zero on; `E(xtb.mv)` steps to each code's reading 99 us into
/// its period, as `meas`, which `assign meas = mv` drives, does point for
/// point. A vector variable traces as an engine-declared bus over its bits,
/// and `xtrim.sh` holds code k half way through period k.
#[test]
fn esave_publishes_internal_hdl_variables_as_traces() {
    let result = run(".esave xtb.ok__event_0 xtb.pass xtb.mv meas xtrim.sh");

    assert_eq!(logic(&result, "xtb.pass"), [(0.0, DigitalState::One)]);

    let mv = real(&result, "xtb.mv");
    let meas = real(&result, "meas");
    assert_eq!(
        mv.len(),
        17,
        "the value at time zero, then one per code: {mv:?}"
    );
    assert_eq!(mv[0], (0.0, 0.0));
    for (k, (time, _)) in mv.iter().skip(1).enumerate() {
        let expected = k as f64 * 100e-6 + 99e-6;
        assert!(
            (time - expected).abs() <= 1e-15,
            "code {k} is read at {expected:e} s, the trace steps at {time:e} s"
        );
    }
    assert!(
        mv.windows(2).all(|pair| pair[0].1 != pair[1].1),
        "every point is a change: {mv:?}"
    );
    let bits = |trace: &[(f64, f64)]| -> Vec<(u64, u64)> {
        trace
            .iter()
            .map(|(time, value)| (time.to_bits(), value.to_bits()))
            .collect()
    };
    assert_eq!(bits(&mv), bits(&meas), "mv and meas step together, exactly");

    let bus = result
        .digital_buses
        .iter()
        .find(|bus| bus.name == "xtrim.sh")
        .expect("the vector is a bus");
    assert_eq!((bus.msb, bus.lsb), (3, 0));
    assert_eq!(
        bus.members,
        ["xtrim.sh[3]", "xtrim.sh[2]", "xtrim.sh[1]", "xtrim.sh[0]"]
    );
    assert_eq!(bus.source, rspice_core::engine::DigitalBusSource::Engine);
    let members: Vec<_> = bus
        .members
        .iter()
        .map(|member| logic(&result, member))
        .collect();
    for k in 0..16u32 {
        let time = f64::from(k) * 100e-6 + 50e-6;
        let word = members.iter().fold(0, |word, member| {
            word << 1 | u32::from(level_at(member, time) == DigitalState::One)
        });
        assert_eq!(word, k, "xtrim.sh at {time:e} s");
    }
}

/// A name that is neither an event net nor a net or variable of an HDL
/// instance is refused by name, as `.ESAVE` refuses a node that is not an
/// event node.
#[test]
fn esave_refuses_a_name_no_instance_declares() {
    let error = Engine::default()
        .run_tran(&c12_full(".esave xtb.ok__event_0 xtb.pas"), STOP, STEP)
        .expect_err("a misspelt variable is refused")
        .to_string()
        .to_ascii_lowercase();
    assert!(
        error.contains("'xtb.pas' is not an event node or an hdl variable"),
        "{error}"
    );
}
