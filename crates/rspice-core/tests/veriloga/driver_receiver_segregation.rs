//! Driver-receiver segregation (Verilog-AMS LRM 2.4 sections 7.8.4 and
//! 9.22.5): on a net a bidirectional connect module joins, the digital
//! drivers resolve together and feed the connect module's digital port, and
//! the connect module's own drive is all the receivers see.
use rspice_core::engine::{TransientCheckpoint, TransientResult, TransientStartupMode};
use rspice_core::{Engine, Netlist, SimulationConfig};

struct Source(std::path::PathBuf);
impl Source {
    fn new(text: &str) -> Self {
        Self::with_extension(text, "vams")
    }
    fn with_extension(text: &str, extension: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-segregation-{}-{id}.{extension}",
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

/// The node's voltage at `time`, interpolated between the accepted points
/// around it, so two runs on different time grids compare at one instant.
fn voltage_at(result: &TransientResult, name: &str, time: f64) -> f64 {
    let node = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("node {name} missing from {:?}", result.node_names));
    let values = result
        .try_voltage_waveform(node + 1)
        .expect("node index in range");
    let after = result.time.iter().position(|value| *value >= time).unwrap();
    if after == 0 || result.time[after] == time {
        return values[after];
    }
    let (t0, t1) = (result.time[after - 1], result.time[after]);
    values[after - 1] + (values[after] - values[after - 1]) * (time - t0) / (t1 - t0)
}

/// The converters a deck's build inserts, as `check --converters` lists
/// them: boundary, port, direction, kind and module.
fn converter_rows(netlist: &Netlist) -> Vec<String> {
    let circuit = Engine::default()
        .build_circuit(netlist)
        .unwrap_or_else(|error| panic!("deck builds: {error}"));
    let mut rows: Vec<String> = circuit
        .converter_insertions()
        .iter()
        .map(|record| {
            format!(
                "{} {:?} {:?} {:?} {}",
                record.boundary, record.port, record.direction, record.kind, record.module
            )
        })
        .collect();
    rows.sort();
    rows
}

/// The rows `with` lists beyond `without`, after checking it lists every
/// row of `without`.
fn added_rows(without: &[String], with: &[String]) -> Vec<String> {
    for row in without {
        assert!(
            with.contains(row),
            "{row} is gone with the receiver: {with:#?}"
        );
    }
    with.iter()
        .filter(|row| !without.contains(row))
        .cloned()
        .collect()
}

/// Two modules drive one bus through ports of their own: 1 from 10 to 30 ns
/// and 0 from 20 to 40 ns, against a 2.2 kohm pull-up to 1.8 V.
const BUS: &str = r#"
`timescale 1ns/1ps
module drv(p); inout p; wire p; reg d;
  parameter real t_on = 10;
  parameter real t_off = 30;
  parameter integer level = 1;
  assign p = d;
  initial begin d = 1'bz; #(t_on) d = level ? 1'b1 : 1'b0; #(t_off - t_on) d = 1'bz; end
endmodule
"#;

fn bus_deck(source: &Source, receiver: bool) -> String {
    format!(
        "* two drivers on one bus\n\
         .hdl \"{}\" module=drv\n\
         .param vcc=1.8\n\
         xa n drv t_on=10 t_off=30 level=1\n\
         xb n drv t_on=20 t_off=40 level=0\n\
         vdd vdd 0 1.8\n\
         rp n vdd 2.2k\n\
         cb n 0 0.1p\n\
         {}.end\n",
        source.path(),
        if receiver {
            "abuf n q buf\n.model buf d_buffer(rise_delay=1p fall_delay=1p)\n"
        } else {
            ""
        }
    )
}

/// A code model that only reads a net never changes the net's analog value.
/// The modules' ports keep the converters they have without it, inout and
/// output ports alike, and the code model reads the node through an A/D of
/// its own that drives nothing: the bus holds the same voltage, the drivers'
/// contention included, and the code model reads it against vcc/2.
#[test]
fn a_code_model_reading_the_bus_never_changes_it() {
    use rspice_core::xspice::DigitalState;
    for direction in ["inout", "output"] {
        let source = Source::new(&BUS.replace("inout p;", &format!("{direction} p;")));
        let [without, with] =
            [false, true].map(|receiver| Netlist::parse(&bus_deck(&source, receiver)).unwrap());
        assert_eq!(
            added_rows(&converter_rows(&without), &converter_rows(&with)),
            vec!["N Some(\"ABUF.in\") In AdcBridge adc_bridge".to_string()],
            "{direction}"
        );
        let run = |netlist: &Netlist| Engine::default().run_tran(netlist, 50e-9, 0.1e-9).unwrap();
        let (alone, read) = (run(&without), run(&with));
        // Each driver converts through its own 20 ohm: the 1 to 1.8 V
        // against the 0 to ground, with the pull-up. The converters' sensing
        // inputs load the bus by megohms, a few microvolts.
        let contention = 1.8 * (1.0 / 20.0 + 1.0 / 2200.0) / (2.0 / 20.0 + 1.0 / 2200.0);
        let seen = voltage_at(&alone, "n", 25e-9);
        assert!(
            (seen - contention).abs() < 1e-5,
            "{direction} v(n)@25n: {seen} != {contention}"
        );
        let trace: Vec<_> = read
            .digital_trace_named("q")
            .unwrap_or_else(|| panic!("{direction}: q has no digital trace"))
            .iter()
            .map(|point| (point.time, point.value.state))
            .collect();
        for (time, expected) in [
            (5e-9, DigitalState::One),
            (15e-9, DigitalState::One),
            (25e-9, DigitalState::One),
            (35e-9, DigitalState::Zero),
            (45e-9, DigitalState::One),
        ] {
            let (bare, loaded) = (voltage_at(&alone, "n", time), voltage_at(&read, "n", time));
            assert!(
                (bare - loaded).abs() < 1e-6,
                "{direction} v(n)@{time}: {loaded} with the reader, {bare} without"
            );
            let state = trace
                .iter()
                .rev()
                .find(|(at, _)| *at <= time)
                .map(|(_, state)| *state);
            assert_eq!(state, Some(expected), "{direction} q@{time}: {trace:?}");
        }
    }
}

/// Corpus design C7's contention deck, with or without the d_buffer that
/// reads SDA, parsed from a copy whose includes name their files absolutely.
fn i2c_contention_deck(receiver: bool) -> Netlist {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus/c07-i2c/deck")
        .canonicalize()
        .expect("the C7 deck directory exists");
    let net = dir.join("../net").canonicalize().unwrap();
    let net = net.to_string_lossy().replace('\\', "/");
    let text = std::fs::read_to_string(dir.join("cont.tran.cir")).unwrap();
    let mut deck = String::new();
    for line in text.lines() {
        let reads_sda = ["abufrx ", ".model dbufrx", "adacrx ", ".model dacrx"]
            .iter()
            .any(|card| line.starts_with(card));
        if reads_sda && !receiver {
            continue;
        }
        deck.push_str(&line.replace("../net", &net));
        deck.push('\n');
    }
    assert!(deck.contains("abufrx sda sda_rx") == receiver, "{deck}");
    let copy = Source::with_extension(&deck, "cir");
    Netlist::parse_file(&copy.0).unwrap_or_else(|error| panic!("the C7 deck parses: {error}"))
}

/// The I2C bus's contention, the controller sinking SDA through 20 ohm
/// against i2c_dev51 driving it high, reads the same with a receiver on SDA
/// as without one, and the receiver reads 0, 1 and 0 across it: the
/// converters' A/D of V(sda) against vcc/2, as the modules' receivers read
/// it.
#[test]
fn the_i2c_bus_reads_the_same_with_a_receiver_on_sda() {
    let [without, with] = [false, true].map(i2c_contention_deck);
    let added = added_rows(&converter_rows(&without), &converter_rows(&with));
    assert_eq!(
        added,
        vec!["SDA Some(\"ABUFRX.in\") In AdcBridge adc_bridge".to_string()]
    );
    let run = |netlist: &Netlist| Engine::default().run_tran(netlist, 320e-6, 100e-9).unwrap();
    let (alone, read) = (run(&without), run(&with));
    let contention = 3.3 * (1.0 / 20.0 + 1.0 / 2200.0) / (2.0 / 20.0 + 1.0 / 2200.0);
    let seen = voltage_at(&alone, "sda", 305e-6);
    assert!(
        (seen - contention).abs() < 1e-5,
        "v(sda)@305u: {seen} != {contention}"
    );
    for (time, receives) in [(297e-6, 0.0), (305e-6, 1.0), (313e-6, 0.0), (319e-6, 1.0)] {
        let (bare, loaded) = (
            voltage_at(&alone, "sda", time),
            voltage_at(&read, "sda", time),
        );
        assert!(
            (bare - loaded).abs() < 1e-6,
            "v(sda)@{time}: {loaded} with the receiver, {bare} without"
        );
        let rx = voltage_at(&read, "rx", time);
        assert!(
            (rx - receives).abs() < 1e-6,
            "v(rx)@{time}: {rx} != {receives}"
        );
    }
}
