//! Driver-receiver segregation (Verilog-AMS LRM 2.4 sections 7.8.4 and
//! 9.22.5): on a net a bidirectional connect module joins, the digital
//! drivers resolve together and feed the connect module's digital port, and
//! the connect module's own drive is all the receivers see.
use rspice_core::engine::{TransientCheckpoint, TransientResult, TransientStartupMode};
use rspice_core::{Engine, Netlist, SimulationConfig};

struct Source(std::path::PathBuf);
impl Source {
    fn new(text: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-segregation-{}-{id}.vams",
            std::process::id()
        ));
        std::fs::write(&path, text).unwrap();
        Self(path)
    }
    fn path(&self) -> String {
        self.0.to_string_lossy().replace('\\', "/")
    }
}
impl Drop for Source {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn voltage(result: &TransientResult, name: &str, time: f64) -> f64 {
    let node = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("node {name} missing from {:?}", result.node_names));
    let sample = result.time.iter().position(|value| *value >= time).unwrap();
    result
        .try_voltage_waveform(node + 1)
        .expect("node index in range")[sample]
}

/// An inout port drives 1 from 10 ns and 0 from 20 ns into a node a 10 ohm
/// source holds at 0 V until it steps to 1.8 V at 25 ns, and releases at
/// 30 ns. The module copies what it receives on the port to an output.
const PORT: &str = r#"
`timescale 1ns/1ps
module seg(p, o); inout p; output o; wire p; reg drv;
  assign p = drv;
  assign o = p;
  initial begin drv = 1'bz; #10 drv = 1'b1; #10 drv = 1'b0; #10 drv = 1'bz; end
endmodule
"#;

/// An authored bidirectional connect module: the port's driver sets 1.8 V or
/// 0 V behind 20 ohm, or releases on x and z, and the receivers get the
/// analog node against a 0.9 V threshold (section 9.22.5's `assign d = out`).
const AUTHORED: &str = r#"
connectmodule bid(d, a); inout d; logic d; inout a; electrical a;
  reg r; real g, lvl;
  assign d = r;
  initial begin r = 1'bz; g = 0.0; lvl = 0.0; end
  always @(above(V(a) - 0.9) or above(0.9 - V(a))) r = V(a) > 0.9;
  always @(d) begin
    g = (d === 1'b1 || d === 1'b0) ? 1.0 / 20.0 : 0.0;
    lvl = (d === 1'b1) ? 1.8 : 0.0;
  end
  analog I(a) <+ g * (V(a) - lvl);
endmodule
connectrules r; connect bid; endconnectrules
"#;

fn deck(source: &Source, options: &str) -> String {
    format!(
        "* driver-receiver segregation\n\
         .hdl \"{}\" module=seg\n\
         {options}\
         .param vcc=1.8\n\
         xs p o seg\n\
         rs p m 10\n\
         vm m 0 pwl(0 0 25n 0 25.1n 1.8)\n\
         ro o 0 1k\n\
         .end\n",
        source.path()
    )
}

/// The states the event trace of the port's net records, `xs.p__event_0`
/// (the converter's digital side of `p`), and of `o`, which the VCD export
/// writes: the receivers' value, never the contention's `x`.
fn assert_traced_as_received(result: &TransientResult) {
    use rspice_core::xspice::DigitalState;
    for name in ["xs.p__event_0", "o"] {
        let trace: Vec<_> = result
            .digital_trace_named(name)
            .unwrap_or_else(|| panic!("{name} has no digital trace"))
            .iter()
            .map(|point| (point.time, point.value.state))
            .collect();
        assert!(
            trace
                .iter()
                .all(|(_, state)| matches!(state, DigitalState::Zero | DigitalState::One)),
            "{name}: {trace:?}"
        );
        assert_eq!(
            trace.first().map(|(_, state)| *state),
            Some(DigitalState::Zero),
            "{name}: {trace:?}"
        );
        assert_eq!(
            trace.last().map(|(_, state)| *state),
            Some(DigitalState::One),
            "{name}: {trace:?}"
        );
    }
    let document = rspice_core::execution::event_vcd_document(
        "tran",
        &result.digital_traces,
        &result.real_traces,
        &result.digital_buses,
    )
    .unwrap();
    let mut vcd = Vec::new();
    rspice_core::io::write_vcd(&mut vcd, &document).unwrap();
    let vcd = String::from_utf8(vcd).unwrap();
    assert!(
        !vcd.lines().any(|line| line.starts_with('x')),
        "the dump records no x:\n{vcd}"
    );
}

/// What the receivers see, through the output's built-in 20 ohm D/A into
/// 1 kohm: 0 V for a 0 and 1.8 * 1000 / 1020 V for a 1. An `x` would read
/// 0.9 * 1000 / 1020 V.
fn received(result: &TransientResult, time: f64) -> f64 {
    voltage(result, "o", time) / (1.8 * 1000.0 / 1020.0)
}

/// The built-in bidirectional converter: the port's 1 against the node's
/// 0 V is 0.6 V on the node, which the receivers see as the A/D half's 0,
/// not the `x` the port's own 1 resolved with it gave; at 27 ns the port's
/// 0 against 1.8 V is 1.2 V, a 1.
#[test]
fn the_built_in_bidirectional_converter_alone_drives_the_receivers() {
    let source = Source::new(PORT);
    let netlist = Netlist::parse(&deck(&source, "")).unwrap();
    let result = Engine::default().run_tran(&netlist, 40e-9, 0.1e-9).unwrap();
    for (time, node, receives) in [
        (5e-9, 0.0, 0.0),
        (15e-9, 0.6, 0.0),
        (22e-9, 0.0, 0.0),
        (27e-9, 1.2, 1.0),
        (35e-9, 1.8, 1.0),
    ] {
        let actual = voltage(&result, "p", time);
        assert!(
            (actual - node).abs() < 1e-3,
            "V(p)@{time}: {actual} != {node}"
        );
        let seen = received(&result, time);
        assert!(
            (seen - receives).abs() < 1e-6,
            "received@{time}: {seen} != {receives}"
        );
    }
    assert_traced_as_received(&result);
}

/// An authored bidirectional connect module's digital port reads the port's
/// driver alone, without its own assignment: 1.8 V behind 20 ohm against
/// 0 V behind 10 ohm is 0.6 V at 15 ns, and 0 V behind 20 ohm against 1.8 V
/// behind 10 ohm is 1.2 V at 27 ns. Read with its own assignment it saw `x`
/// in both and released the node, to 0 V and 1.8 V. The receivers see the
/// module's assignment alone.
#[test]
fn an_authored_bidirectional_module_reads_the_drivers_and_drives_the_receivers() {
    let source = Source::new(&format!("{PORT}{AUTHORED}"));
    let netlist = Netlist::parse(&deck(&source, ".options connectrules=r\n")).unwrap();
    let result = Engine::default().run_tran(&netlist, 40e-9, 0.1e-9).unwrap();
    for (time, node, receives) in [
        (5e-9, 0.0, 0.0),
        (15e-9, 0.6, 0.0),
        (22e-9, 0.0, 0.0),
        (27e-9, 1.2, 1.0),
        (35e-9, 1.8, 1.0),
    ] {
        let actual = voltage(&result, "p", time);
        assert!(
            (actual - node).abs() < 1e-3,
            "V(p)@{time}: {actual} != {node}"
        );
        let seen = received(&result, time);
        assert!(
            (seen - receives).abs() < 1e-6,
            "received@{time}: {seen} != {receives}"
        );
    }
    assert_traced_as_received(&result);
}

/// Every driver of the net feeds the converter, a code model's among them:
/// with the port released, an XSPICE buffer driving the segment 1 from
/// 10 ns and 0 from 20 ns is converted onto the node as the port's own
/// drive is, and the receivers still see the converter's A/D half alone.
#[test]
fn a_code_model_on_the_segment_feeds_the_converter_and_not_the_receivers() {
    let source = Source::new(&PORT.replace(
        "initial begin drv = 1'bz; #10 drv = 1'b1; #10 drv = 1'b0; #10 drv = 1'bz; end",
        "initial drv = 1'bz;",
    ));
    let deck = deck(&source, "").replace(
        "rs p m 10\n",
        "vin in 0 pwl(0 0 10n 0 10.001n 1.8 20n 1.8 20.001n 0)\n\
         aadc [in] [din] adc\n\
         .model adc adc_bridge(in_low=0.9 in_high=0.9 rise_delay=1p fall_delay=1p)\n\
         abuf din p buf\n\
         .model buf d_buffer(rise_delay=1p fall_delay=1p)\n\
         rs p m 10\n",
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let result = Engine::default().run_tran(&netlist, 40e-9, 0.1e-9).unwrap();
    for (time, node, receives) in [
        (5e-9, 0.0, 0.0),
        (15e-9, 0.6, 0.0),
        (22e-9, 0.0, 0.0),
        (27e-9, 1.2, 1.0),
        (35e-9, 1.2, 1.0),
    ] {
        let actual = voltage(&result, "p", time);
        assert!(
            (actual - node).abs() < 1e-3,
            "V(p)@{time}: {actual} != {node}"
        );
        let seen = received(&result, time);
        assert!(
            (seen - receives).abs() < 1e-6,
            "received@{time}: {seen} != {receives}"
        );
    }
    assert_traced_as_received(&result);
}

/// [`deck`] with a buffer reading `p` into `q`.
fn deck_with_buffer(source: &Source, options: &str) -> String {
    deck(source, options).replace(
        "ro o 0 1k\n",
        "ro o 0 1k\n\
         abuf p q buf\n\
         .model buf d_buffer(rise_delay=1p fall_delay=1p)\n",
    )
}

/// A code model's input on the segment is a receiver too (section 7.8.4
/// rule 2): a buffer reading `p` sees what the design's receivers see,
/// through both contentions, for both converters, and never the `x` the
/// port's drive resolved with the converter's would give.
#[test]
fn a_code_model_input_on_the_segment_reads_what_the_design_receives() {
    use rspice_core::xspice::DigitalState;
    for options in ["", ".options connectrules=r\n"] {
        let source = Source::new(&format!("{PORT}{AUTHORED}"));
        let netlist = Netlist::parse(&deck_with_buffer(&source, options)).unwrap();
        let result = Engine::default().run_tran(&netlist, 40e-9, 0.1e-9).unwrap();
        let trace: Vec<_> = result
            .digital_trace_named("q")
            .unwrap_or_else(|| panic!("{options}: q has no digital trace"))
            .iter()
            .map(|point| (point.time, point.value.state))
            .collect();
        assert!(
            trace
                .iter()
                .all(|(_, state)| matches!(state, DigitalState::Zero | DigitalState::One)),
            "{options}: {trace:?}"
        );
        for (time, receives) in [
            (5e-9, 0.0),
            (15e-9, 0.0),
            (22e-9, 0.0),
            (27e-9, 1.0),
            (35e-9, 1.0),
        ] {
            let state = trace
                .iter()
                .rev()
                .find(|(at, _)| *at <= time)
                .map(|(_, state)| *state);
            let expected = if receives == 1.0 {
                DigitalState::One
            } else {
                DigitalState::Zero
            };
            assert_eq!(state, Some(expected), "{options} q@{time}: {trace:?}");
            let seen = received(&result, time);
            assert!(
                (seen - receives).abs() < 1e-6,
                "{options} received@{time}: {seen} != {receives}"
            );
        }
    }
}

/// A checkpoint taken while the port's driver and the converter disagree
/// resumes onto the unbroken run's accepted points exactly, for both
/// converters, and the buffer reading the segment onto the unbroken run's
/// events.
#[test]
fn a_segregated_net_resumes_from_a_checkpoint_in_contention_exactly() {
    for options in ["", ".options connectrules=r\n"] {
        let source = Source::new(&format!("{PORT}{AUTHORED}"));
        let netlist = Netlist::parse(&deck_with_buffer(&source, options)).unwrap();
        let engine = Engine::new(SimulationConfig::default());
        let (baseline, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                &netlist,
                40e-9,
                0.1e-9,
                TransientStartupMode::OperatingPoint,
                &[15e-9],
                &rspice_core::NoAbort,
            )
            .unwrap();
        let saved = &scheduled[0].checkpoint;
        let bytes = saved
            .to_bytes(rspice_core::engine::TransientCheckpointEncoding::Unpacked)
            .unwrap();
        let restored = TransientCheckpoint::from_bytes(&bytes).unwrap();
        let (resumed, _) = engine
            .run_tran_resume_with_abort(&netlist, &restored, 40e-9, 0.1e-9, &rspice_core::NoAbort)
            .unwrap();
        let after = |result: &TransientResult| {
            let nodes: Vec<_> = ["p", "o"]
                .iter()
                .map(|name| {
                    let node = result
                        .node_names
                        .iter()
                        .position(|node| node.eq_ignore_ascii_case(name))
                        .unwrap();
                    result.try_voltage_waveform(node + 1).unwrap().to_vec()
                })
                .collect();
            result
                .time
                .iter()
                .enumerate()
                .filter(|(_, time)| **time > saved.time)
                .map(|(index, time)| {
                    (
                        time.to_bits(),
                        nodes[0][index].to_bits(),
                        nodes[1][index].to_bits(),
                    )
                })
                .collect::<Vec<_>>()
        };
        let events = |result: &TransientResult| {
            result
                .digital_trace_named("q")
                .expect("q has a digital trace")
                .iter()
                .filter(|point| point.time > saved.time)
                .map(|point| (point.time.to_bits(), point.value))
                .collect::<Vec<_>>()
        };
        let expected = after(&baseline);
        assert!(
            expected.len() > 10,
            "{options}: the run continues after the seam"
        );
        assert_eq!(after(&resumed), expected, "{options}");
        let expected = events(&baseline);
        assert!(
            !expected.is_empty(),
            "{options}: the buffer's output moves after the seam"
        );
        assert_eq!(events(&resumed), expected, "{options}");
    }
}
