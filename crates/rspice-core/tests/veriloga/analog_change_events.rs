//! `@(d)` in an analog block on a discrete signal: Verilog-AMS LRM 2.4
//! section 5.10.5's state change event. Section 7.3.4 puts the operand of a
//! discrete event in the discrete context, so the event is IEEE 1364-2005
//! section 9.7.1's: any change of the value, into or out of `x` or `z`
//! included, and never an analog read of the value that could find `x`.
use rspice_core::engine::{TransientCheckpoint, TransientResult, TransientStartupMode};
use rspice_core::{Engine, Netlist, SimulationConfig};

struct Source(std::path::PathBuf);
impl Source {
    fn new(text: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-analog-change-events-{}-{id}.vams",
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

/// A driver stepping 0, x, z, 1, 1 again and 0, ten nanoseconds apart: a
/// change into `x`, one from `x` to `z`, one out of `z`, an assignment that
/// changes nothing, and a known change.
const DRIVER: &str = r#"
`timescale 1ns/1ps
discipline ddiscrete; domain discrete; enddiscipline
`default_discipline ddiscrete
module drv(o); output o; reg o;
  initial begin
    o = 1'b0; #10 o = 1'bx; #10 o = 1'bz; #10 o = 1'b1; #10 o = 1'b1; #10 o = 1'b0;
  end
endmodule
`default_discipline
"#;

/// The driver's port converted by `body`, a connect module from `ddiscrete`.
fn deck(source: &Source) -> String {
    format!(
        "* analog change events\nXD y drv\nRY y 0 1k\n.va \"{}\" drv module=drv\n.end\n",
        source.path()
    )
}

fn converter(body: &str) -> String {
    format!(
        r#"{DRIVER}
connectmodule a2d(d, a); input d; ddiscrete d; output a; electrical a;
{body}
endmodule
connectrules ev; connect a2d input ddiscrete, output electrical; endconnectrules
"#
    )
}

/// A connect module counting its input's changes, as its output level.
fn counter() -> String {
    converter(
        "  real n;
  analog begin
    @(d) n = n + 1;
    V(a) <+ n;
  end",
    )
}

/// Each change fires once. The driver's register starts at x, so its first
/// assignment, 0 at time 0, is a change already counted at the operating
/// point; then 0 to x, x to z, z to 1 and 1 to 0 each add one, and the
/// assignment of 1 over 1 adds nothing. The run reads `d` only as the
/// event, so the x and z it holds are no error.
#[test]
fn each_change_of_value_fires_once_x_and_z_included() {
    let source = Source::new(&counter());
    let netlist = Netlist::parse(&deck(&source)).unwrap();
    let result = Engine::default().run_tran(&netlist, 60e-9, 0.1e-9).unwrap();
    for (time, changes) in [
        (0.0, 1.0),
        (5e-9, 1.0),
        (15e-9, 2.0),
        (25e-9, 3.0),
        (35e-9, 4.0),
        (45e-9, 4.0),
        (55e-9, 5.0),
    ] {
        let actual = voltage(&result, "y", time);
        assert!(
            (actual - changes).abs() < 1e-9,
            "changes by {time}: {actual} != {changes}"
        );
    }
}

/// The body runs because of the change and reads the value it settled, as
/// every discrete read in an event body does: x maps to 1, z to 2.5, 1 to 5
/// and 0 to 0 from the change itself. Read at the left limit instead, each
/// level would trail the driver by one change: 0 at 15 ns, 1 at 25 ns, 2.5
/// at 35 ns and 5 at 55 ns.
#[test]
fn the_body_reads_the_value_the_change_settled() {
    let source = Source::new(&converter(
        "  real lv;
  analog begin
    @(d) begin
      if (d === 1'bx) lv = 1;
      else if (d === 1'bz) lv = 2.5;
      else if (d === 1'b1) lv = 5;
      else lv = 0;
    end
    V(a) <+ lv;
  end",
    ));
    let netlist = Netlist::parse(&deck(&source)).unwrap();
    let result = Engine::default().run_tran(&netlist, 60e-9, 0.1e-9).unwrap();
    for (time, expected) in [
        (5e-9, 0.0),
        (15e-9, 1.0),
        (25e-9, 2.5),
        (35e-9, 5.0),
        (45e-9, 5.0),
        (55e-9, 0.0),
    ] {
        let actual = voltage(&result, "y", time);
        assert!(
            (actual - expected).abs() < 1e-9,
            "v(y)@{time}: {actual} != {expected}"
        );
    }
}

/// A checkpoint taken while the net is z resumes onto the unbroken run's
/// accepted points bit for bit: the count crosses the seam, and the changes
/// after it fire once each in the resumed run as in the unbroken one.
#[test]
fn a_checkpoint_between_changes_resumes_exactly() {
    let source = Source::new(&counter());
    let netlist = Netlist::parse(&deck(&source)).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let (baseline, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
            &netlist,
            60e-9,
            0.1e-9,
            TransientStartupMode::OperatingPoint,
            &[25e-9],
            &rspice_core::NoAbort,
        )
        .unwrap();
    let saved = &scheduled[0].checkpoint;
    let bytes = saved
        .to_bytes(rspice_core::engine::TransientCheckpointEncoding::Packed)
        .unwrap();
    let restored = TransientCheckpoint::from_bytes(&bytes).unwrap();
    let (resumed, _) = engine
        .run_tran_resume_with_abort(&netlist, &restored, 60e-9, 0.1e-9, &rspice_core::NoAbort)
        .unwrap();
    let after = |result: &TransientResult| {
        let node = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case("y"))
            .unwrap();
        let values = result.try_voltage_waveform(node + 1).unwrap();
        result
            .time
            .iter()
            .zip(values)
            .filter(|(time, _)| **time > saved.time)
            .map(|(time, value)| (time.to_bits(), value.to_bits()))
            .collect::<Vec<_>>()
    };
    let expected = after(&baseline);
    assert!(expected.len() > 10, "the run continues after the seam");
    assert_eq!(after(&resumed), expected);
    assert!((voltage(&resumed, "y", 55e-9) - 5.0).abs() < 1e-9);
}

/// A real variable's value changes and a named event's triggers are state
/// change events too (LRM sections 3.7 and 5.10.4): the real fires at 10 and
/// 30 ns but not at 20, where it is assigned the value it holds, and the
/// event fires at each trigger, 15 and 25 ns. Counted from 5 ns, after the
/// real's own first assignment at time 0.
#[test]
fn a_real_value_change_and_a_named_event_each_fire() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module watch(o); output o; electrical o;
  real level;
  event ev;
  initial begin level = 0.5; #10 level = 1.5; #10 level = 1.5; #10 level = 2.5; end
  initial begin #15 -> ev; #10 -> ev; end
  real n, m;
  analog begin
    @(level) n = n + 1;
    @(ev) m = m + 1;
    V(o) <+ n + 100 * m;
  end
endmodule
"#,
    );
    let netlist = Netlist::parse(&format!(
        "* real and event changes\nXW y watch\nRY y 0 1k\n.va \"{}\" watch module=watch\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&netlist, 40e-9, 0.1e-9).unwrap();
    let start = voltage(&result, "y", 5e-9);
    for (time, expected) in [
        (12e-9, 1.0),
        (17e-9, 101.0),
        (22e-9, 101.0),
        (27e-9, 201.0),
        (35e-9, 202.0),
    ] {
        let actual = voltage(&result, "y", time) - start;
        assert!(
            (actual - expected).abs() < 1e-9,
            "v(y)@{time} - v(y)@5ns: {actual} != {expected}"
        );
    }
}
