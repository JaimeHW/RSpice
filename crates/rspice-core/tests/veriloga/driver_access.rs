//! A connect module's driver access functions and `driver_update`
//! (Verilog-AMS LRM 2.4 section 9.22): the connect module reads each driver
//! of the net its digital port joins, every driver but a connect module's
//! own, and wakes whenever one of them takes a new value.
use rspice_core::engine::{TransientCheckpoint, TransientResult, TransientStartupMode};
use rspice_core::{Engine, Netlist, SimulationConfig};

struct Source(std::path::PathBuf);
impl Source {
    fn new(text: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-driver-access-{}-{id}.vams",
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

/// The driver half of the section 9.22.6 example: a connect module that
/// pulls its node up through 100 ohm for each driver at 1 and down through
/// 120 ohm for each driver at 0, off a 5 V supply. Three drivers each switch
/// from 0 to 1, at 10, 20 and 30 ns. The node has no receiver, so the
/// example's receiver half, which drives `d` from thresholds on the node,
/// is left out.
const LRM_EXAMPLE: &str = r#"
`include "disciplines.vams"
`timescale 1ns/1ps
module drv(o); output o; ddiscrete o; reg o; parameter real t1 = 10;
  initial begin o = 1'b0; #(t1) o = 1'b1; end
endmodule
connectmodule c2e(d, a);
  input d; output a; ddiscrete d;
  electrical a, rail, gnd;
  ground gnd;
  branch (rail, a) pull_up;
  branch (a, gnd) pull_down;
  branch (rail, gnd) power;
  parameter real impedance0 = 120.0;
  parameter real impedance1 = 100.0;
  parameter real impedanceOff = 1e6;
  parameter real supply = 5.0;
  integer i, num_ones, num_zeros;
  initial begin num_ones = 0; num_zeros = 0; end
  always @(driver_update(d)) begin
    num_ones = 0;
    num_zeros = 0;
    for (i = 0; i < $driver_count(d); i = i + 1)
      if ($driver_state(d, i) == 1) num_ones = num_ones + 1;
      else num_zeros = num_zeros + 1;
  end
  analog begin
    V(pull_up) <+ 1/((1/impedance1)*num_ones + (1/impedanceOff)) * I(pull_up);
    V(pull_down) <+ 1/((1/impedance0)*num_zeros + (1/impedanceOff)) * I(pull_down);
    V(power) <+ supply;
  end
endmodule
connectrules lrm; connect c2e; endconnectrules
"#;

fn lrm_deck(source: &Source) -> String {
    format!(
        "* section 9.22.6 driver access\n\
         .hdl \"{}\" module=drv\n\
         .options connectrules=lrm\n\
         x1 n1 drv t1=10\n\
         x2 n1 drv t1=20\n\
         x3 n1 drv t1=30\n\
         c1 n1 0 1p\n\
         .end\n",
        source.path()
    )
}

/// The node of the section 9.22.6 example with `ones` drivers at 1 and
/// `zeros` at 0: the supply across the two pulls in series.
fn lrm_node(ones: f64, zeros: f64) -> f64 {
    let up = 1.0 / (ones / 100.0 + 1.0 / 1e6);
    let down = 1.0 / (zeros / 120.0 + 1.0 / 1e6);
    5.0 * down / (up + down)
}

#[test]
fn the_lrm_connect_module_pulls_its_node_by_each_drivers_state() {
    let source = Source::new(LRM_EXAMPLE);
    let netlist = Netlist::parse(&lrm_deck(&source)).unwrap();
    let result = Engine::default().run_tran(&netlist, 40e-9, 0.1e-9).unwrap();
    for (time, ones, zeros) in [
        (5e-9, 0.0, 3.0),
        (15e-9, 1.0, 2.0),
        (25e-9, 2.0, 1.0),
        (35e-9, 3.0, 0.0),
    ] {
        let node = voltage(&result, "n1", time);
        let expected = lrm_node(ones, zeros);
        assert!(
            (node - expected).abs() < 1e-6,
            "v(n1)@{time}: {node} != {expected}"
        );
    }
}

/// A checkpoint between two driver updates resumes onto the unbroken run's
/// accepted points exactly: the connect module's view of its drivers, and
/// the update count it waits on, are part of the state.
#[test]
fn the_driver_view_resumes_from_a_checkpoint_exactly() {
    let source = Source::new(LRM_EXAMPLE);
    let netlist = Netlist::parse(&lrm_deck(&source)).unwrap();
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
        let node = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case("n1"))
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
}

/// Four drivers, one each at `We0` then `We1`, `Pu1`, `StX` and `HiZ`, and a
/// connect module that sets its node to a checksum of what it reads of
/// them: per driver 1, 10, 100 or 1000 for a state of 0, 1, x or z, plus
/// 10^4 times its strength (section 9.22.3) and 10^7 times its type.
const STRENGTHS: &str = r#"
`timescale 1ns/1ps
module w(o); output o; wire o; reg v;
  assign (weak0, weak1) o = v;
  initial begin v = 1'b0; #10 v = 1'b1; end
endmodule
module p(o); output o; wire o; assign (pull0, pull1) o = 1'b1; endmodule
module sx(o); output o; wire o; assign o = 1'bx; endmodule
module sz(o); output o; wire o; assign o = 1'bz; endmodule
connectmodule probe(d, a);
  input d; output a; logic d; electrical a;
  integer i;
  real code;
  initial code = 0;
  always @(driver_update(d)) begin
    code = 0;
    for (i = 0; i < $driver_count(d); i = i + 1) begin
      case ($driver_state(d, i))
        1'b0: code = code + 1;
        1'b1: code = code + 10;
        1'bx: code = code + 100;
        1'bz: code = code + 1000;
      endcase
      code = code + 1e4 * $driver_strength(d, i) + 1e7 * $driver_type(d, i);
    end
  end
  analog V(a) <+ code;
endmodule
connectrules probing; connect probe; endconnectrules
"#;

#[test]
fn each_driver_reports_its_state_and_strength() {
    let source = Source::new(STRENGTHS);
    let deck = format!(
        "* driver states and strengths\n\
         .hdl \"{0}\" module=w\n\
         .hdl \"{0}\" module=p\n\
         .hdl \"{0}\" module=sx\n\
         .hdl \"{0}\" module=sz\n\
         .options connectrules=probing\n\
         xw n1 w\n\
         xp n1 p\n\
         xx n1 sx\n\
         xz n1 sz\n\
         ro n1 0 1k\n\
         .end\n",
        source.path()
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let result = Engine::default().run_tran(&netlist, 20e-9, 0.1e-9).unwrap();
    // Strength codes, strength0 in bits 5-3 and strength1 in bits 2-0: `We`
    // is 3, `Pu` 5 and `St` 6, a known value carries its one strength in
    // both, and `HiZ` is 0. Every type is `DRIVER_UNKNOWN`, 0.
    let strengths = f64::from((3 << 3 | 3) + (5 << 3 | 5) + (6 << 3 | 6));
    for (time, states) in [
        (5e-9, 1.0 + 10.0 + 100.0 + 1000.0),
        (15e-9, 10.0 + 10.0 + 100.0 + 1000.0),
    ] {
        let node = voltage(&result, "n1", time);
        let expected = states + 1e4 * strengths;
        assert!(
            (node - expected).abs() < 1e-6,
            "v(n1)@{time}: {node} != {expected}"
        );
    }
}

fn refusal(source: &str, deck: impl FnOnce(&Source) -> String) -> String {
    let source = Source::new(source);
    let netlist = Netlist::parse(&deck(&source)).unwrap();
    match Engine::default().run_tran(&netlist, 20e-9, 0.1e-9) {
        Ok(_) => panic!("the deck runs"),
        Err(error) => error.to_string(),
    }
}

#[test]
fn a_driver_access_function_outside_a_connect_module_is_refused() {
    let error = refusal(
        r#"
`timescale 1ns/1ps
module m(o); output o; wire o; reg r; integer n;
  assign o = r;
  initial begin r = 1'b0; n = $driver_count(o); end
endmodule
"#,
        |source| {
            format!(
                "* driver access outside a connect module\n.hdl \"{}\" module=m\nx1 n1 m\nro n1 0 1k\n.end\n",
                source.path()
            )
        },
    );
    assert!(
        error.contains("$driver_count") && error.contains("only a connect module"),
        "{error}"
    );
}

#[test]
fn a_pending_transaction_function_is_refused_by_name() {
    let error = refusal(
        &STRENGTHS.replace("$driver_type(d, i)", "$driver_next_state(d, i)"),
        |source| {
            format!(
                "* pending transaction\n.hdl \"{}\" module=w\n.options connectrules=probing\n\
                 xw n1 w\nro n1 0 1k\n.end\n",
                source.path()
            )
        },
    );
    assert!(
        error.contains("$driver_next_state") && error.contains("9.23.2"),
        "{error}"
    );
}

/// A driver whose value waits out a delay has a pending value ahead of it,
/// and `driver_update` is the moment that value is scheduled; with no view
/// of pending values the run is refused rather than woken late.
#[test]
fn driver_update_on_a_net_with_a_delayed_driver_is_refused() {
    let error = refusal(
        &STRENGTHS.replace(
            "assign (weak0, weak1) o = v;",
            "assign (weak0, weak1) #2 o = v;",
        ),
        |source| {
            format!(
                "* delayed driver\n.hdl \"{}\" module=w\n.options connectrules=probing\n\
                 xw n1 w\nro n1 0 1k\n.end\n",
                source.path()
            )
        },
    );
    assert!(
        error.contains("driver_update") && error.contains("delayed driver"),
        "{error}"
    );
}

/// A code model's output is scheduled ahead of its value, as a gate's is;
/// with no view of that pending value the run is refused.
#[test]
fn driver_update_on_a_net_a_code_model_drives_is_refused() {
    let error = refusal(STRENGTHS, |source| {
        format!(
            "* code model driver\n.hdl \"{}\" module=w\n.options connectrules=probing\n\
             xw n1 w\n\
             vin in 0 pwl(0 0 10n 0 10.001n 1.8)\n\
             aadc [in] [din] adc\n\
             .model adc adc_bridge(in_low=0.9 in_high=0.9 rise_delay=1p fall_delay=1p)\n\
             abuf din n1 buf\n\
             .model buf d_buffer(rise_delay=1p fall_delay=1p)\n\
             ro n1 0 1k\n.end\n",
            source.path()
        )
    });
    assert!(
        error.contains("driver_update") && error.to_ascii_lowercase().contains("code model `abuf`"),
        "{error}"
    );
}
