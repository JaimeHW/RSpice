//! `===`, `!==` and `case` on a discrete net in analog code: Verilog-AMS LRM
//! 2.4 section 7.3.2's way for the continuous domain to tell `x` and `z`
//! apart, and so the way an authored D/A connect module maps them to levels.
//!
//! The section makes every other analog read of an `x` or `z` bit an error,
//! which the last test pins: the comparison is the exception, not a value.
use rspice_core::engine::{TransientCheckpoint, TransientResult, TransientStartupMode};
use rspice_core::{Engine, Netlist, SimulationConfig};

struct Source(std::path::PathBuf);
impl Source {
    fn new(text: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-analog-case-equality-{}-{id}.vams",
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

/// A driver whose port declares no discipline, under section 10.2's own
/// example directive, stepping 0, 1, x, z, x and 0 ten nanoseconds apart.
const DRIVER: &str = r#"
`timescale 1ns/1ps
discipline ddiscrete; domain discrete; enddiscipline
`default_discipline ddiscrete
module drv(o); output o; reg o;
  initial begin
    o = 1'b0; #10 o = 1'b1; #10 o = 1'bx; #10 o = 1'bz; #10 o = 1'bx; #10 o = 1'b0;
  end
endmodule
`default_discipline
"#;

/// Section 7.3.2's example as written, made the connect module the
/// `ddiscrete` driver's port selects.
const LRM_7_3_2: &str = r#"
connectmodule a2d(dnet, anet);
input dnet;
output anet;
wire dnet;
ddiscrete dnet;
electrical anet;
real avar;
analog begin
if (dnet === 1'b1)
avar = 5;
else if (dnet === 1'bx)
avar = avar; // hold value
else if (dnet === 1'b0)
avar = 0;
else if (dnet === 1'bz)
avar = 2.5; // high impedance - float value
V(anet) <+ avar;
end
endmodule
connectrules lrm; connect a2d input ddiscrete, output electrical; endconnectrules
"#;

fn lrm_deck(source: &Source) -> String {
    format!(
        "* LRM 7.3.2\nXD y drv\nRY y 0 1k\n.va \"{}\" drv module=drv\n.end\n",
        source.path()
    )
}

/// The section's stated result: 5 for a 1, 0 for a 0, 2.5 for a z, and an x
/// holds the level before it, 5 after the 1 and 2.5 after the z.
#[test]
fn the_lrm_example_maps_each_state_to_its_level_and_holds_through_x() {
    let source = Source::new(&format!("{DRIVER}{LRM_7_3_2}"));
    let netlist = Netlist::parse(&lrm_deck(&source)).unwrap();
    let result = Engine::default().run_tran(&netlist, 60e-9, 0.1e-9).unwrap();
    for (time, expected) in [
        (5e-9, 0.0),
        (15e-9, 5.0),
        (25e-9, 5.0),
        (35e-9, 2.5),
        (45e-9, 2.5),
        (55e-9, 0.0),
    ] {
        let actual = voltage(&result, "y", time);
        assert!(
            (actual - expected).abs() < 1e-9,
            "v(y)@{time}: {actual} != {expected}"
        );
    }
}

/// A resistive D/A that maps each state to its own level and drive through
/// `case`, `===` and `!==`: 0 and 1 drive 0 and `vdd` behind `ron`, x drives
/// `vdd/2` behind `rx`, and z releases the node. Into a 1 kohm load returned
/// to a 1 V reference, the node is `(lv*g + vref/rl) / (g + 1/rl)`.
#[test]
fn an_authored_converter_drives_its_x_and_z_levels() {
    let source = Source::new(&format!(
        r#"{DRIVER}
connectmodule d2a_xz(d, a);
  input d; ddiscrete d; output a; electrical a;
  parameter real vdd = 1.8, ron = 100, rx = 1k;
  real lv, g;
  analog begin
    case (d)
      1'b0: lv = 0.0;
      1'b1: lv = vdd;
      1'bx: lv = vdd / 2;
      default: lv = 0.0;
    endcase
    if (d === 1'bx) g = 1 / rx;
    else if (d !== 1'bz) g = 1 / ron;
    else g = 0;
    I(a) <+ g * (V(a) - lv);
  end
endmodule
connectrules xz; connect d2a_xz input ddiscrete, output electrical; endconnectrules
"#
    ));
    let netlist = Netlist::parse(&format!(
        "* X and Z levels\nXD y drv\nRL y r 1k\nVR r 0 1\n.va \"{}\" drv module=drv\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&netlist, 60e-9, 0.1e-9).unwrap();
    let (vdd, ron, rx, rl, vref) = (1.8, 100.0, 1e3, 1e3, 1.0);
    let node = |lv: f64, g: f64| (lv * g + vref / rl) / (g + 1.0 / rl);
    let (zero, one, x, z) = (
        node(0.0, 1.0 / ron),
        node(vdd, 1.0 / ron),
        node(vdd / 2.0, 1.0 / rx),
        node(0.0, 0.0),
    );
    for (time, expected) in [
        (5e-9, zero),
        (15e-9, one),
        (25e-9, x),
        (35e-9, z),
        (45e-9, x),
        (55e-9, zero),
    ] {
        let actual = voltage(&result, "y", time);
        assert!(
            (actual - expected).abs() < 1e-9,
            "v(y)@{time}: {actual} != {expected}"
        );
    }
}

/// Section 5.1.8's comparison at the wider width, on a bus and its selects:
/// `4'b10xz === bus` holds only while the bus holds exactly those bits, a
/// bit-select compares one bit, `2'b0z` zero-extends against a one-bit
/// select, a known constant wider than the bus holds only zeros above it,
/// and an integer parameter or variable matches a select only while every
/// bit of it is known. Each match adds its own power of ten to the output.
#[test]
fn a_bus_and_its_selects_compare_bit_for_bit_at_the_wider_width() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module bus(o); output o; electrical o;
  parameter integer two = 2;
  reg [3:0] b;
  initial begin b = 4'b0000; #10 b = 4'b10xz; #10 b = 4'b1x0z; end
  real sum;
  integer k;
  analog begin
    sum = 0;
    k = 2;
    if (b === 4'b10xz) sum = sum + 1;
    if (b[1] === 1'bx) sum = sum + 10;
    if (b[0] === 2'b0z) sum = sum + 100;
    if (b[3:2] !== 2'b10) sum = sum + 1000;
    if (b === 8'h00) sum = sum + 10000;
    if (b === 4'bxxxx) sum = sum + 100000;
    if (b[3:2] === two) sum = sum + 1000000;
    if (k === b[3:2]) sum = sum + 10000000;
    V(o) <+ sum;
  end
endmodule
"#,
    );
    let netlist = Netlist::parse(&format!(
        "* bus compare\nXB y bus\nRY y 0 1k\n.va \"{}\" bus module=bus\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&netlist, 30e-9, 0.1e-9).unwrap();
    // 0000: only the 8'h00 compare (and the [3:2] mismatch with 10).
    // 10xz: the whole bus, b[1] is x, b[0] is z, and [3:2] is two.
    // 1x0z: b[0] is z, and [3:2] is 1x, which is neither 10 nor two.
    for (time, expected) in [(5e-9, 11000.0), (15e-9, 11000111.0), (25e-9, 1100.0)] {
        let actual = voltage(&result, "y", time);
        assert!(
            (actual - expected).abs() < 1e-9,
            "v(y)@{time}: {actual} != {expected}"
        );
    }
}

/// In an analog event body a comparison reads the bits the activation
/// settled, as every discrete read there does. The driver steps 0, 1, z, 0,
/// x and 1, each an IEEE 1364-2005 section 9.7.2 edge, so `@(posedge d or
/// negedge d)` fires at every step and maps the new state there. Read at the
/// left limit instead, the level would trail the driver by one step: 0 at
/// 15 ns, 5 at 25 ns, 2.5 at 35 ns and 0 at 55 ns.
#[test]
fn an_event_body_compares_the_bits_its_activation_settled() {
    let driver = DRIVER.replace(
        "o = 1'b0; #10 o = 1'b1; #10 o = 1'bx; #10 o = 1'bz; #10 o = 1'bx; #10 o = 1'b0;",
        "o = 1'b0; #10 o = 1'b1; #10 o = 1'bz; #10 o = 1'b0; #10 o = 1'bx; #10 o = 1'b1;",
    );
    assert_ne!(driver, DRIVER);
    let source = Source::new(&format!(
        r#"{driver}
connectmodule a2d_event(d, a); input d; ddiscrete d; output a; electrical a;
  real lv;
  analog begin
    @(posedge d or negedge d) begin
      if (d === 1'b1) lv = 5;
      else if (d === 1'b0) lv = 0;
      else if (d === 1'bz) lv = 2.5;
    end
    V(a) <+ lv;
  end
endmodule
connectrules ev; connect a2d_event input ddiscrete, output electrical; endconnectrules
"#
    ));
    let netlist = Netlist::parse(&lrm_deck(&source)).unwrap();
    let result = Engine::default().run_tran(&netlist, 60e-9, 0.1e-9).unwrap();
    for (time, expected) in [
        (15e-9, 5.0),
        (25e-9, 2.5),
        (35e-9, 0.0),
        (45e-9, 0.0),
        (55e-9, 5.0),
    ] {
        let actual = voltage(&result, "y", time);
        assert!(
            (actual - expected).abs() < 1e-9,
            "v(y)@{time}: {actual} != {expected}"
        );
    }
}

/// A checkpoint while the net is x resumes onto the unbroken run's accepted
/// points bit for bit: the held level and the sampled chunks are state.
#[test]
fn a_checkpoint_taken_at_x_resumes_exactly() {
    let source = Source::new(&format!("{DRIVER}{LRM_7_3_2}"));
    let netlist = Netlist::parse(&lrm_deck(&source)).unwrap();
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
    // The held level crosses the seam: 5 V through the x after the 1.
    assert!((voltage(&resumed, "y", 28e-9) - 5.0).abs() < 1e-9);
}

/// Every other analog read of an `x` or `z` bit is an error (section 7.3.2:
/// "It will be an error if the value of the digital variable being accessed
/// in the analog context goes either to x or z"), named with the value and
/// the line that reads it, and a real operand of `===` is refused: section
/// 4.2.1 makes `===` and `!==` illegal on a real.
#[test]
fn any_other_analog_read_of_x_or_z_is_an_error() {
    let read = Source::new(&format!(
        r#"{DRIVER}
connectmodule plain(d, a); input d; ddiscrete d; output a; electrical a;
  analog V(a) <+ (d == 1'b1) ? 1.0 : 0.0;
endmodule
connectrules p; connect plain input ddiscrete, output electrical; endconnectrules
"#
    ));
    let netlist = Netlist::parse(&lrm_deck(&read)).unwrap();
    let error = Engine::default()
        .run_tran(&netlist, 60e-9, 0.1e-9)
        .expect_err("the read of x is an error")
        .to_string();
    assert!(
        error.contains("`d` = x") && error.contains("section 7.3.2"),
        "{error}"
    );

    let continuous = Source::new(&format!(
        r#"{DRIVER}
connectmodule cont(d, a); input d; ddiscrete d; output a; electrical a;
  real r;
  analog begin r = V(a); V(a) <+ (r === 1'bx) ? 1.0 : 0.0; end
endmodule
connectrules p; connect cont input ddiscrete, output electrical; endconnectrules
"#
    ));
    let netlist = Netlist::parse(&lrm_deck(&continuous)).unwrap();
    let error = Engine::default()
        .run_tran(&netlist, 60e-9, 0.1e-9)
        .expect_err("a continuous operand is refused")
        .to_string();
    assert!(
        error.contains("`===` in analog code compares the bits") && error.contains("is real"),
        "{error}"
    );
}
