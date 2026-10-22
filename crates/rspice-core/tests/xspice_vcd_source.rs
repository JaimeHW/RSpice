//! The `d_vcd` digital stimulus: a VCD file replayed onto event nets.
//!
//! Every expected value is closed form: event times are the VCD reader's own
//! tick conversion, compared bit for bit, and the analog levels are the D/A's
//! drive through its 20 Ω into 1 MΩ.

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::io::parse_vcd_reader;
use rspice_core::netlist::Netlist;
use rspice_core::xspice::DigitalValue;
use std::path::PathBuf;

struct Scratch(PathBuf);

impl Scratch {
    fn new(label: &str) -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock follows Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-dvcd-{label}-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).expect("create isolated test directory");
        Self(path)
    }

    fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, contents).expect("write the test file");
        path
    }

    /// The deck, parsed from a file in this directory so that `file=` reads
    /// beside it.
    fn deck(&self, text: &str) -> Netlist {
        let path = self.write("deck.cir", text);
        Netlist::parse_with_path(text, &path).expect("deck parses")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn events(result: &TransientResult, node: &str) -> Vec<(f64, DigitalValue)> {
    result
        .digital_trace_named(node)
        .unwrap_or_else(|| panic!("no trace of {node}: {:?}", result.node_names))
        .iter()
        .map(|point| (point.time, point.value))
        .collect()
}

/// `node`'s value in force at `time`.
fn value_in_force(result: &TransientResult, node: &str, time: f64) -> DigitalValue {
    events(result, node)
        .iter()
        .rev()
        .find(|(at, _)| *at <= time)
        .map(|(_, value)| *value)
        .unwrap_or_else(|| panic!("{node} has no value at {time}"))
}

fn bits(times: &[f64]) -> Vec<u64> {
    times.iter().map(|time| time.to_bits()).collect()
}

/// `clk` in scope `tb`: 0 at #0, then toggling every #5000 ps, 100 times.
fn clock_vcd() -> String {
    let mut text = String::from(
        "$timescale 1ps $end\n$scope module tb $end\n$var wire 1 ! clk $end\n$upscope $end\n\
         $enddefinitions $end\n#0\n$dumpvars\n0!\n$end\n",
    );
    for k in 1..=100u64 {
        text.push_str(&format!("#{}\n{}!\n", k * 5000, k % 2));
    }
    text
}

/// Every change of `clk` is an event at exactly the time the reader gives its
/// tick, and `toffset=1n` moves every one by 1 ns, leaving X before it.
#[test]
fn a_clock_changes_at_exactly_the_ticks_the_reader_places() {
    let scratch = Scratch::new("clock");
    let vcd = clock_vcd();
    scratch.write("clk.vcd", &vcd);
    let document = parse_vcd_reader(vcd.as_bytes()).expect("the dump reads");
    let changes = &document.signals[0].changes;
    let expected: Vec<f64> = changes
        .iter()
        .map(|change| document.timescale.seconds_at_tick(change.tick).unwrap())
        .collect();
    let levels: Vec<DigitalValue> = (0..=100u64)
        .map(|k| {
            if k % 2 == 1 {
                DigitalValue::one()
            } else {
                DigitalValue::zero()
            }
        })
        .collect();
    let run = |offset: &str| {
        let netlist = scratch.deck(&format!(
            "* d_vcd clock\na1 [clk] vstim\n\
             .model vstim d_vcd (file=\"clk.vcd\" signals=\"tb.clk\"{offset})\n.end\n"
        ));
        Engine::default()
            .run_tran(&netlist, 510e-9, 1e-9)
            .expect("the transient runs")
    };

    let plain = events(&run(""), "clk");
    let times: Vec<f64> = plain.iter().map(|(time, _)| *time).collect();
    assert_eq!(bits(&times), bits(&expected), "{plain:?}");
    let values: Vec<DigitalValue> = plain.iter().map(|(_, value)| *value).collect();
    assert_eq!(values, levels);
    assert_eq!(expected[1], 5000.0 * 1e-12);
    assert_eq!(expected[100], 500_000.0 * 1e-12);

    let moved = events(&run(" toffset=1n"), "clk");
    assert_eq!(
        moved[0],
        (0.0, DigitalValue::unknown()),
        "X before the first change"
    );
    let moved_times: Vec<f64> = moved[1..].iter().map(|(time, _)| *time).collect();
    let shifted: Vec<f64> = expected.iter().map(|time| time + 1e-9).collect();
    assert_eq!(bits(&moved_times), bits(&shifted), "{moved:?}");
    let moved_values: Vec<DigitalValue> = moved[1..].iter().map(|(_, value)| *value).collect();
    assert_eq!(moved_values, levels);
}

/// A change no queued delay can date still lands on its tick: from 2 ns, every
/// `2 ns + delay` near 7 ns is a tie that rounds past it.
#[test]
fn a_change_no_delay_reaches_still_lands_on_its_tick() {
    let scratch = Scratch::new("tie");
    let vcd = "$timescale 1ns $end\n$scope module tb $end\n$var wire 1 ! clk $end\n$upscope $end\n\
               $enddefinitions $end\n#0\n$dumpvars\n0!\n$end\n#2\n1!\n#7\n0!\n";
    scratch.write("clk.vcd", vcd);
    let document = parse_vcd_reader(vcd.as_bytes()).expect("the dump reads");
    let expected: Vec<f64> = document.signals[0]
        .changes
        .iter()
        .map(|change| document.timescale.seconds_at_tick(change.tick).unwrap())
        .collect();
    let (from, at) = (expected[1], expected[2]);
    let mut delay = at - from;
    for _ in 0..8 {
        delay = delay.next_down();
    }
    for _ in 0..16 {
        assert_ne!(from + delay, at, "no delay from {from:e} reaches {at:e}");
        delay = delay.next_up();
    }

    let netlist = scratch.deck(
        "* d_vcd tie\na1 [clk] vstim\n\
         .model vstim d_vcd (file=\"clk.vcd\" signals=\"tb.clk\")\n.end\n",
    );
    let result = Engine::default()
        .run_tran(&netlist, 10e-9, 1e-9)
        .expect("the transient runs");
    let clk = events(&result, "clk");
    let times: Vec<f64> = clk.iter().map(|(time, _)| *time).collect();
    assert_eq!(bits(&times), bits(&expected), "{clk:?}");
}

/// `b[3:0]`, `b0101` at #0 and `bx01z` at #10000.
const BUS_VCD: &str = "$timescale 1ps $end\n$scope module tb $end\n$var wire 4 # b [3:0] $end\n\
                       $upscope $end\n$enddefinitions $end\n#0\n$dumpvars\nb0101 #\n$end\n\
                       #10000\nbx01z #\n";

/// The bus lands most significant bit first: 0, 1, 0, 1 and then X, 0, 1, Z
/// on b3..b0, the change at exactly 10 ns.
#[test]
fn a_vector_lands_most_significant_bit_first_in_four_states() {
    let scratch = Scratch::new("bus");
    scratch.write("bus.vcd", BUS_VCD);
    let netlist = scratch.deck(
        "* d_vcd bus\na1 [b3 b2 b1 b0] vstim\n\
         .model vstim d_vcd (file=\"bus.vcd\" signals=\"tb.b[3:0]\")\n.end\n",
    );
    let result = Engine::default()
        .run_tran(&netlist, 20e-9, 1e-9)
        .expect("the transient runs");
    let before = ["b3", "b2", "b1", "b0"].map(|node| value_in_force(&result, node, 5e-9));
    assert_eq!(
        before,
        [
            DigitalValue::zero(),
            DigitalValue::one(),
            DigitalValue::zero(),
            DigitalValue::one()
        ]
    );
    let after = ["b3", "b2", "b1", "b0"].map(|node| value_in_force(&result, node, 15e-9));
    assert_eq!(
        after,
        [
            DigitalValue::unknown(),
            DigitalValue::zero(),
            DigitalValue::one(),
            DigitalValue::high_z()
        ]
    );
    // b2 and b1 both change at the second tick, and nowhere else.
    for node in ["b2", "b1"] {
        let times: Vec<f64> = events(&result, node)
            .iter()
            .map(|(time, _)| *time)
            .collect();
        assert_eq!(bits(&times), bits(&[0.0, 10000.0 * 1e-12]), "{node}");
    }
}

/// A `$dumpoff` with no `$dumpon` after it is refused, naming its ticks.
#[test]
fn a_dump_switched_off_is_refused_naming_its_ticks() {
    let scratch = Scratch::new("dumpoff");
    scratch.write(
        "off.vcd",
        "$timescale 1ps $end\n$scope module tb $end\n$var wire 1 ! clk $end\n$upscope $end\n\
         $enddefinitions $end\n#0\n$dumpvars\n0!\n$end\n#10000\n1!\n#20000\n$dumpoff\nx!\n$end\n",
    );
    let netlist = scratch.deck(
        "* d_vcd dumpoff\na1 [clk] vstim\n\
         .model vstim d_vcd (file=\"off.vcd\" signals=\"tb.clk\")\n.end\n",
    );
    let error = Engine::default()
        .run_tran(&netlist, 30e-9, 1e-9)
        .expect_err("a dump that was switched off is refused")
        .to_string();
    assert!(
        error.contains("off.vcd")
            && error.contains("from tick 20000 to the end of the file")
            && error.contains("not recorded"),
        "{error}"
    );
}

/// A file with a 4-bit `b` and a real `v`, for the refusals.
const MIXED_VCD: &str = "$timescale 1ns $end\n$scope module tb $end\n$var wire 4 # b [3:0] $end\n\
                         $var real 64 % v $end\n$upscope $end\n$enddefinitions $end\n#0\n\
                         $dumpvars\nb0000 #\nr0.25 %\n$end\n#5\nb1111 #\nr1.5 %\n";

fn refusal(scratch: &Scratch, ports: &str, signals: &str) -> String {
    let netlist = scratch.deck(&format!(
        "* d_vcd refusal\na1 {ports} vstim\n\
         .model vstim d_vcd (file=\"mixed.vcd\" signals=\"{signals}\")\n.end\n"
    ));
    Engine::default()
        .run_tran(&netlist, 10e-9, 1e-9)
        .expect_err("the mapping is refused")
        .to_string()
}

/// A width that disagrees with the file, and a real variable on a digital
/// port, are refused naming the signal; a real variable drives `real_out`.
#[test]
fn a_mapping_the_file_does_not_fit_is_refused_naming_the_signal() {
    let scratch = Scratch::new("refusals");
    scratch.write("mixed.vcd", MIXED_VCD);

    let width = refusal(&scratch, "[d7 d6 d5 d4 d3 d2 d1 d0]", "tb.b[7:0]");
    assert!(
        width.contains("'tb.b[7:0]' is 8 bits wide, but") && width.contains("declares it 4 bits"),
        "{width}"
    );

    let real = refusal(&scratch, "[d3 d2 d1 d0 r]", "tb.b tb.v");
    assert!(
        real.contains("'tb.v' is a real variable, but it would drive out[4], a digital port"),
        "{real}"
    );

    let short = refusal(&scratch, "[d1 d0]", "tb.b");
    assert!(
        short.contains("'tb.b' is 4 bits wide, but out has 2 of its 2 ports left for it"),
        "{short}"
    );

    let netlist = scratch.deck(
        "* d_vcd real\na1 [d3 d2 d1 d0] [r] vstim\n\
         .model vstim d_vcd (file=\"mixed.vcd\" signals=\"tb.b tb.v\")\n.end\n",
    );
    let result = Engine::default()
        .run_tran(&netlist, 10e-9, 1e-9)
        .expect("a real variable drives real_out");
    let real: Vec<(u64, f64)> = result
        .real_trace_named("r")
        .expect("a real trace of r")
        .iter()
        .map(|point| (point.time.to_bits(), point.value))
        .collect();
    assert_eq!(
        real,
        vec![(0.0f64.to_bits(), 0.25), ((5.0f64 * 1e-9).to_bits(), 1.5)]
    );
}

#[cfg(feature = "veriloga")]
mod mixed {
    use super::*;
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    use rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES;

    /// The built-in connect modules under a 1 V D/A that ramps over 1 ns,
    /// after `module`.
    fn with_rules(module: &str) -> String {
        let mut source = module.to_string();
        for (_, connect) in BUILTIN_CONNECT_MODULES {
            source.push_str(connect);
        }
        source.push_str(
            "connectrules deck;\n    connect d2a #(.vsup(1), .trise(1n), .tfall(1n));\n\
             endconnectrules\n",
        );
        source
    }

    const BUFFER: &str = "`include \"disciplines.vams\"\n`timescale 1ns/1ps\n\
                          module buffer(i, o);\n  input i; output o;\n  assign o = i;\nendmodule\n";

    fn series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
        let index = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
        &result.voltages[index]
    }

    /// `node` at `time`, linearly interpolated.
    fn value_at(result: &TransientResult, node: &str, time: f64) -> f64 {
        let values = series(result, node);
        let after = result
            .time
            .iter()
            .position(|t| *t >= time)
            .unwrap_or_else(|| panic!("{time} is inside the run"));
        if after == 0 || result.time[after] == time {
            return values[after];
        }
        let (t0, t1) = (result.time[after - 1], result.time[after]);
        values[after - 1] + (values[after] - values[after - 1]) * (time - t0) / (t1 - t0)
    }

    /// The bus of the four-state test, b3 and b0 into the D/A and 1 MΩ: a 1
    /// is 1 V through the D/A's 20 Ω, an X half the supply, a Z nothing.
    #[test]
    fn x_and_z_reach_an_electrical_net_through_the_d_to_a() {
        let scratch = Scratch::new("d2a");
        scratch.write("bus.vcd", BUS_VCD);
        let rules = scratch.write("rules.vams", &with_rules(BUFFER));
        let netlist = scratch.deck(&format!(
            "* d_vcd into the D/A\n.hdl \"{}\" module=buffer\n.options connectrules=deck\n\
             a1 [b3 b2 b1 b0] vstim\n\
             .model vstim d_vcd (file=\"bus.vcd\" signals=\"tb.b[3:0]\")\n\
             r3 b3 0 1meg\nr0 b0 0 1meg\n.end\n",
            rules.display().to_string().replace('\\', "/")
        ));
        let result = Engine::default()
            .run_tran(&netlist, 20e-9, 0.1e-9)
            .expect("the transient runs");
        let divider = 1e6 / (1e6 + 20.0);
        let close = |node: &str, time: f64, expected: f64| {
            let actual = value_at(&result, node, time);
            assert!(
                (actual - expected).abs() <= 1e-12,
                "V({node}) at {time:e}: {actual:e}, expected {expected:e}"
            );
        };
        close("b3", 8e-9, 0.0);
        close("b0", 8e-9, divider);
        close("b3", 18e-9, 0.5 * divider);
        close("b0", 18e-9, 0.0);
    }

    /// `a` in scope `tb`, 1 ns ticks: forty changes about 100 ns apart.
    fn stimulus(flip: bool) -> String {
        let mut text = String::from(
            "$timescale 1ns $end\n$scope module tb $end\n$var wire 1 ! a $end\n$upscope $end\n\
             $enddefinitions $end\n#0\n$dumpvars\n0!\n$end\n",
        );
        for k in 1..=40u64 {
            let level = (k % 2 == 1) != (flip && k == 30);
            text.push_str(&format!(
                "#{}\n{}!\n",
                k * 100 + (k % 3) * 7,
                u8::from(level)
            ));
        }
        text
    }

    fn buffered_deck(scratch: &Scratch) -> Netlist {
        let source = scratch.write("buffer.vams", &with_rules(BUFFER));
        scratch.deck(&format!(
            "* d_vcd through an HDL buffer\n.hdl \"{}\" module=buffer\n\
             .options connectrules=deck\na1 [a] vstim\n\
             .model vstim d_vcd (file=\"stim.vcd\" signals=\"tb.a\")\n\
             x1 a q buffer\nr1 q 0 1meg\n.end\n",
            source.display().to_string().replace('\\', "/")
        ))
    }

    /// Every event after `stop` at the same instant with the same value, and
    /// `q` within 1 nV at every point either run accepted after it.
    fn assert_resumes_onto(whole: &TransientResult, resumed: &TransientResult, stop: f64) {
        for trace in &whole.digital_traces {
            let after = |points: &[rspice_core::engine::DigitalTracePoint]| {
                points
                    .iter()
                    .filter(|point| point.time > stop)
                    .copied()
                    .collect::<Vec<_>>()
            };
            let other = resumed
                .digital_traces
                .iter()
                .find(|other| other.node_name == trace.node_name)
                .unwrap_or_else(|| panic!("the resumed run does not trace {}", trace.node_name));
            assert_eq!(
                after(&other.points),
                after(&trace.points),
                "events on {} resumed from {stop:e}",
                trace.node_name
            );
        }
        let end = *whole.time.last().expect("a run");
        for time in whole
            .time
            .iter()
            .chain(&resumed.time)
            .copied()
            .filter(|time| *time > stop && *time <= end)
        {
            let (a, b) = (value_at(whole, "q", time), value_at(resumed, "q", time));
            assert!(
                (a - b).abs() <= 1e-9,
                "q resumed from {stop:e} at {time:e}: {b} against {a}"
            );
        }
    }

    /// A checkpoint just before a change, between two, and late in the file
    /// resumes onto the uninterrupted run: the same events at the same
    /// instants, and the same analog output.
    #[test]
    fn a_checkpoint_mid_file_resumes_onto_the_uninterrupted_run() {
        let scratch = Scratch::new("resume");
        scratch.write("stim.vcd", &stimulus(false));
        let netlist = buffered_deck(&scratch);
        let engine = Engine::default();
        let whole = engine
            .run_tran(&netlist, 4.5e-6, 10e-9)
            .expect("the uninterrupted run");
        let a = events(&whole, "a");
        assert_eq!(a.len(), 41, "{a:?}");
        assert_eq!(a[10].0.to_bits(), (1007.0f64 * 1e-9).to_bits());
        for stop in [1.0e-6, 1.5035e-6, 2.6e-6] {
            let (_, checkpoint) = engine
                .run_tran_checkpointed(&netlist, stop, 10e-9)
                .expect("checkpoints");
            let (resumed, _) = engine
                .run_tran_resume(&netlist, &checkpoint, 4.5e-6, 10e-9)
                .expect("resumes");
            assert_resumes_onto(&whole, &resumed, stop);
        }
    }

    /// A checkpoint resumed after its VCD file was edited is refused, naming
    /// the file and the instance, through a saved image as well as in memory.
    #[test]
    fn a_resume_against_an_edited_file_is_refused_naming_the_file() {
        let scratch = Scratch::new("edited");
        scratch.write("stim.vcd", &stimulus(false));
        let netlist = buffered_deck(&scratch);
        let engine = Engine::default();
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, 1.5e-6, 10e-9)
            .expect("checkpoints");
        let saved = checkpoint
            .to_bytes(TransientCheckpointEncoding::Unpacked)
            .expect("serializes");
        let text = String::from_utf8(saved.clone()).expect("canonical text");
        assert!(text.starts_with("RSPICE-CHECKPOINT 62\n"));
        let bound: Vec<&str> = text
            .lines()
            .filter(|line| line.starts_with("vcd_file"))
            .collect();
        assert!(
            matches!(bound.as_slice(), ["vcd_files 1", line] if line.starts_with("vcd_file A1 ")
                && line.ends_with("stim.vcd\"")),
            "the image binds the file: {bound:?}"
        );

        scratch.write("stim.vcd", &stimulus(true));
        let reloaded = TransientCheckpoint::from_bytes(&saved).expect("the image reads back");
        for image in [&checkpoint, &reloaded] {
            let error = engine
                .run_tran_resume(&netlist, image, 4.5e-6, 10e-9)
                .expect_err("an edited stimulus is refused")
                .to_string();
            assert!(
                error.contains("stim.vcd")
                    && error.contains("d_vcd instance A1")
                    && error.contains("has changed since the checkpoint was taken"),
                "{error}"
            );
        }

        // The original bytes resume.
        scratch.write("stim.vcd", &stimulus(false));
        engine
            .run_tran_resume(&netlist, &reloaded, 4.5e-6, 10e-9)
            .expect("the unedited file resumes");
    }

    /// The image of a deck that replays no VCD file carries an empty section.
    #[test]
    fn a_deck_without_a_vcd_stimulus_records_no_file() {
        let netlist = Netlist::parse("* resistor\nv1 a 0 1\nr1 a 0 1k\n.end\n").expect("parses");
        let (_, checkpoint) = Engine::default()
            .run_tran_checkpointed(&netlist, 1e-6, 0.1e-6)
            .expect("checkpoints");
        assert!(checkpoint.to_text().ends_with("\nvcd_files 0\n"));
    }
}
