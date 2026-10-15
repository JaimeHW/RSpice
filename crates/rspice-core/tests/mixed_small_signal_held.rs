//! Small-signal analyses around a held discrete state (P1 design S1–S5, S11).
//!
//! The oracle is O5, held-digital equivalence: the same circuit with every
//! converter replaced by what its held state is — a built-in D/A converter by
//! its held level behind its 20 ohm output resistance (a noisy resistor, S11),
//! a stock XSPICE `dac_bridge` by an ideal source at its level (S3), an A/D
//! converter by nothing at all (S4) — solved analog-only. A mixed deck and its
//! twin are the same linear system at the bias, so every small-signal answer
//! must agree to rounding.
#![cfg(feature = "veriloga")]

use rspice_core::analysis::{AcSensitivityOutput, StbConfig};
use rspice_core::{Complex64, Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A source file the deck includes, removed when the test ends.
struct SourceFile(std::path::PathBuf);

impl SourceFile {
    fn new(extension: &str, source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_mixed_held_{}_{}.{extension}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, source).expect("write source file");
        Self(path)
    }

    fn path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for SourceFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// The corpus C5 trim register with a nonzero held code: its initial block
/// loads 1010 into both the shift register and the outputs. `csn` held high
/// reaches the module at time zero as an X-to-1 transition, which is a
/// `posedge` and latches the shift register into the outputs; that is the
/// held code because the shift register holds it too.
const TRIM_REGISTER: &str = "`timescale 1ns/1ps\n\
module trim_reg(sclk, sdi, csn, tr3, tr2, tr1, tr0);\n\
  input sclk, sdi, csn;\n\
  output tr3, tr2, tr1, tr0;\n\
  reg tr3, tr2, tr1, tr0;\n\
  reg [3:0] sh;\n\
  initial begin sh = 4'b1010; tr3 = 1'b1; tr2 = 1'b0; tr1 = 1'b1; tr0 = 1'b0; end\n\
  always @(posedge sclk) if (!csn) sh <= {sh[2:0], sdi};\n\
  always @(posedge csn) {tr3, tr2, tr1, tr0} <= sh;\n\
endmodule\n";

/// A runtime Verilog-A load with a charge and a noise source.
const NOISY_LOAD: &str = "`include \"disciplines.vams\"\n\
module noisy_load(p, n);\n\
  inout p, n; electrical p, n;\n\
  parameter real r = 5k;\n\
  parameter real c = 10p;\n\
  analog I(p, n) <+ V(p, n) / r + ddt(c * V(p, n)) + white_noise(1.0e-22, \"load\");\n\
endmodule\n";

/// The amplifier the trim drives: an inverting VCVS stage whose summing node
/// takes the input and the four trim legs, a loop probe in its feedback path,
/// a diode in its load for distortion, and the runtime Verilog-A load.
const AMPLIFIER: &str = "vin in 0 dc 0.2 ac 1 distof1 0.01\n\
r1 in sum 10k\n\
rt3 tr3 sum 10k\n\
rt2 tr2 sum 20k\n\
rt1 tr1 sum 40k\n\
rt0 tr0 sum 80k\n\
rf sum fb 20k\n\
vprobe out fb 0\n\
eamp out 0 0 sum 1e3\n\
d1 out dn dmod\n\
rd dn 0 5k\n\
.model dmod d is=1e-14 n=1.5\n\
rl out 0 2k\n";

/// The held code 1010 at vcc = 1.8 V.
const HELD: [(&str, f64); 4] = [("tr3", 1.8), ("tr2", 0.0), ("tr1", 1.8), ("tr0", 0.0)];

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

/// The trim register driving the amplifier through built-in converters.
fn verilog_deck(register: &SourceFile, load: &SourceFile) -> Netlist {
    parse(&format!(
        "* held trim register, Verilog\n\
         .param vcc=1.8\n\
         vcsn csn 0 1.8\n\
         vsdi sdi 0 0\n\
         vsclk sclk 0 0\n\
         xtrim sclk sdi csn tr3 tr2 tr1 tr0 trim_reg\n\
         {AMPLIFIER}\
         xload out 0 noisy_load\n\
         .hdl \"{}\"\n\
         .va \"{}\" noisy_load\n\
         .end\n",
        register.path(),
        load.path()
    ))
}

/// Its O5 twin: every built-in converter is its held level behind 20 ohm.
fn verilog_twin(load: &SourceFile) -> Netlist {
    let held = HELD
        .iter()
        .map(|(net, level)| format!("v{net} {net}_h 0 {level}\nr{net}_h {net}_h {net} 20\n"))
        .collect::<String>();
    parse(&format!(
        "* held trim register, O5 twin\n\
         {held}\
         {AMPLIFIER}\
         xload out 0 noisy_load\n\
         .va \"{}\" noisy_load\n\
         .end\n",
        load.path()
    ))
}

/// The same register in XSPICE: four flip-flops whose initial states are the
/// held code, never clocked, into a stock `dac_bridge`.
fn xspice_deck(load: &SourceFile) -> Netlist {
    parse(&format!(
        "* held trim register, XSPICE\n\
         vsdi sdi 0 0\n\
         vsclk sclk 0 0\n\
         aadc [sdi sclk] [sdid sclkd] adc\n\
         .model adc adc_bridge (in_low=0.9 in_high=0.9)\n\
         a3 sdid sclkd NULL NULL q3 nq3 ff1\n\
         a2 sdid sclkd NULL NULL q2 nq2 ff0\n\
         a1 sdid sclkd NULL NULL q1 nq1 ff1\n\
         a0 sdid sclkd NULL NULL q0 nq0 ff0\n\
         .model ff1 d_dff (ic=1)\n\
         .model ff0 d_dff (ic=0)\n\
         adac [q3 q2 q1 q0] [tr3 tr2 tr1 tr0] dac\n\
         .model dac dac_bridge (out_low=0 out_high=1.8)\n\
         {AMPLIFIER}\
         xload out 0 noisy_load\n\
         .va \"{}\" noisy_load\n\
         .end\n",
        load.path()
    ))
}

/// Its twin: a stock `dac_bridge` is an ideal source at its held level.
fn xspice_twin(load: &SourceFile) -> Netlist {
    let held = HELD
        .iter()
        .map(|(net, level)| format!("v{net} {net} 0 {level}\n"))
        .collect::<String>();
    parse(&format!(
        "* held XSPICE register, ideal-source twin\n\
         {held}\
         {AMPLIFIER}\
         xload out 0 noisy_load\n\
         .va \"{}\" noisy_load\n\
         .end\n",
        load.path()
    ))
}

fn node_index(engine: &Engine, netlist: &Netlist, name: &str) -> usize {
    engine
        .build_circuit(netlist)
        .expect("circuit builds")
        .get_node_by_name(name)
        .unwrap_or_else(|| panic!("node {name} exists"))
}

fn ac_voltage(point: &rspice_core::analysis::AcResult, node: &str) -> Complex64 {
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("AC result names {node}: {:?}", point.node_names));
    point.voltages[index]
}

fn assert_close(what: &str, actual: f64, expected: f64, abs: f64, rel: f64) {
    assert!(
        (actual - expected).abs() <= abs + rel * expected.abs(),
        "{what}: {actual:e} differs from the held twin's {expected:e}"
    );
}

fn assert_phasor_close(what: &str, actual: Complex64, expected: Complex64) {
    // A node a held source drives has no small-signal response at all, and
    // neither a decibel nor a phase compares two zeros.
    if expected.norm() <= 1.0e-15 {
        assert!(
            actual.norm() <= 1.0e-12,
            "{what}: {actual} where the twin's node is held at {expected}"
        );
        return;
    }
    let db = |value: Complex64| 20.0 * value.norm().log10();
    assert!(
        (db(actual) - db(expected)).abs() <= 1.0e-4,
        "{what}: {} dB against the twin's {} dB",
        db(actual),
        db(expected)
    );
    let phase = (actual.arg() - expected.arg()).to_degrees();
    assert!(
        phase.abs() <= 1.0e-3,
        "{what}: phase differs from the twin's by {phase} degrees"
    );
    assert_close(
        &format!("{what} re"),
        actual.re,
        expected.re,
        1.0e-5,
        1.0e-5,
    );
    assert_close(
        &format!("{what} im"),
        actual.im,
        expected.im,
        1.0e-5,
        1.0e-5,
    );
}

const FREQUENCIES: [f64; 3] = [1.0e3, 1.0e5, 1.0e7];

/// Every small-signal analysis of `mixed` equals the same analysis of `twin`.
fn assert_held_equivalence(mixed: &Netlist, twin: &Netlist, noise_tolerance: f64) {
    let engine = Engine::default();

    let op = engine
        .run_dc_op(mixed)
        .expect("the mixed operating point solves");
    let twin_op = engine
        .run_dc_op(twin)
        .expect("the twin operating point solves");
    for (net, level) in HELD {
        let held = op
            .try_voltage_named(net)
            .expect("the trim net is published");
        let reference = twin_op
            .try_voltage_named(net)
            .expect("the twin publishes it");
        assert_close(&format!("op v({net})"), held, reference, 1.0e-9, 1.0e-9);
        assert!(
            (held - level).abs() < 0.05,
            "v({net}) holds the code's level: {held}"
        );
    }

    let ac = engine
        .run_ac(mixed, &FREQUENCIES)
        .expect(".ac runs on the held state");
    let twin_ac = engine.run_ac(twin, &FREQUENCIES).expect("twin .ac");
    for (point, reference) in ac.iter().zip(&twin_ac) {
        for node in ["out", "sum", "tr3", "tr2"] {
            assert_phasor_close(
                &format!(".ac v({node}) at {} Hz", point.frequency),
                ac_voltage(point, node),
                ac_voltage(reference, node),
            );
        }
    }

    let tf = engine
        .run_transfer_function(mixed, "out", None, false, "vin")
        .expect(".tf runs on the held state");
    let twin_tf = engine
        .run_transfer_function(twin, "out", None, false, "vin")
        .expect("twin .tf");
    assert_close(".tf gain", tf.gain, twin_tf.gain, 1.0e-5, 1.0e-5);
    assert_close(
        ".tf input impedance",
        tf.input_impedance,
        twin_tf.input_impedance,
        1.0e-5,
        1.0e-5,
    );
    assert_close(
        ".tf output impedance",
        tf.output_impedance,
        twin_tf.output_impedance,
        1.0e-5,
        1.0e-5,
    );

    let stb_config = || {
        StbConfig::new()
            .with_sweep(1.0e3, 1.0e7, 9)
            .with_probe("vprobe")
    };
    let stb = engine.run_stb(mixed, stb_config()).expect(".stb runs");
    let twin_stb = engine.run_stb(twin, stb_config()).expect("twin .stb");
    for ((frequency, gain), reference) in stb
        .frequencies
        .iter()
        .zip(&stb.loop_gains)
        .zip(&twin_stb.loop_gains)
    {
        assert_phasor_close(
            &format!(".stb loop gain at {frequency} Hz"),
            *gain,
            *reference,
        );
    }

    let out = node_index(&engine, mixed, "out");
    let twin_out = node_index(&engine, twin, "out");
    let sens = engine
        .run_sensitivity_linearized(mixed, out, None)
        .expect("adjoint .sens runs on the held state");
    let twin_sens = engine
        .run_sensitivity_linearized(twin, twin_out, None)
        .expect("twin adjoint .sens");
    let mut compared = 0;
    for row in &sens.sensitivities {
        if let Some(reference) = twin_sens
            .sensitivities
            .iter()
            .find(|candidate| candidate.vector_name.eq_ignore_ascii_case(&row.vector_name))
        {
            assert_close(
                &format!("adjoint .sens {}", row.vector_name),
                row.absolute,
                reference.absolute,
                1.0e-5,
                1.0e-5,
            );
            compared += 1;
        }
    }
    assert!(compared >= 8, "the shared rows were compared: {compared}");

    let filters = ["r1", "rf", "rt3", "rt1", "rl", "vin"].map(String::from);
    let complete = engine
        .run_sensitivity_dc_complete(
            mixed,
            AcSensitivityOutput::Voltage {
                positive: out,
                negative: None,
            },
            &filters,
        )
        .expect("complete DC .sens runs on the held state");
    let twin_complete = engine
        .run_sensitivity_dc_complete(
            twin,
            AcSensitivityOutput::Voltage {
                positive: twin_out,
                negative: None,
            },
            &filters,
        )
        .expect("twin complete DC .sens");
    assert_eq!(complete.sensitivities.len(), filters.len());
    for row in &complete.sensitivities {
        let reference = twin_complete
            .sensitivities
            .iter()
            .find(|candidate| candidate.vector_name.eq_ignore_ascii_case(&row.vector_name))
            .unwrap_or_else(|| panic!("the twin reports {}", row.vector_name));
        assert_close(
            &format!("complete .sens {}", row.vector_name),
            row.absolute,
            reference.absolute,
            1.0e-5,
            1.0e-5,
        );
    }

    let disto = engine
        .run_distortion(mixed, &[1.0e3], None)
        .expect(".disto runs on the held state");
    let twin_disto = engine
        .run_distortion(twin, &[1.0e3], None)
        .expect("twin .disto");
    for (point, reference) in disto.points.iter().zip(&twin_disto.points) {
        assert_phasor_close(
            ".disto fundamental v(out)",
            ac_voltage(&point.fundamental_f1, "out"),
            ac_voltage(&reference.fundamental_f1, "out"),
        );
        for product in &point.products {
            let twin_product = reference
                .product(product.product)
                .expect("the twin reports every product");
            let actual = ac_voltage(&product.response, "out");
            let expected = ac_voltage(&twin_product.response, "out");
            assert_close(
                &format!(".disto {:?} re", product.product),
                actual.re,
                expected.re,
                1.0e-9,
                1.0e-5,
            );
            assert_close(
                &format!(".disto {:?} im", product.product),
                actual.im,
                expected.im,
                1.0e-9,
                1.0e-5,
            );
        }
    }

    let noise = engine
        .run_noise(mixed, out, &FREQUENCIES, 300.15)
        .expect(".noise runs on the held state");
    let twin_noise = engine
        .run_noise(twin, twin_out, &FREQUENCIES, 300.15)
        .expect("twin .noise");
    for (point, reference) in noise.iter().zip(&twin_noise) {
        assert_close(
            &format!(".noise output density at {} Hz", point.frequency),
            point.output_noise_density,
            reference.output_noise_density,
            0.0,
            noise_tolerance,
        );
    }
}

/// O5 equivalence of the Verilog register: the held code reaches the
/// amplifier through the built-in converters' levels and 20 ohm, which are
/// noisy (S11), and the runtime module's charge and noise are linearized the
/// same way in both decks.
#[test]
fn the_held_trim_register_matches_its_held_source_twin() {
    let register = SourceFile::new("v", TRIM_REGISTER);
    let load = SourceFile::new("va", NOISY_LOAD);
    assert_held_equivalence(
        &verilog_deck(&register, &load),
        &verilog_twin(&load),
        1.0e-4,
    );
}

/// The same deck with XSPICE flip-flops and a stock `dac_bridge`: the bridge
/// is an ideal source, so its twin holds ideal sources and adds no noise.
#[test]
fn the_xspice_trim_register_matches_its_held_source_twin() {
    let load = SourceFile::new("va", NOISY_LOAD);
    assert_held_equivalence(&xspice_deck(&load), &xspice_twin(&load), 1.0e-4);
}

/// S3 for the two held states that are not a level: an uninitialized `reg`
/// drives X, which is the rail midpoint behind 20 ohm, and a `1'bz` output
/// is open, so the node it reaches behaves as if no converter were there.
#[test]
fn an_unknown_output_is_the_midpoint_behind_its_resistance_and_z_is_open() {
    let module = SourceFile::new(
        "v",
        "`timescale 1ns/1ps\n\
         module xz(o, z);\n\
           output o, z;\n\
           reg o;\n\
           assign z = 1'bz;\n\
         endmodule\n",
    );
    let deck = parse(&format!(
        "* held X and Z\n\
         .param vcc=1.8\n\
         xm o z xz\n\
         io 0 o dc 0 ac 1\n\
         ro o 0 1e12\n\
         vs s 0 dc 0.3 ac 1\n\
         rs s z 1k\n\
         rz z 0 1k\n\
         .hdl \"{}\"\n\
         .end\n",
        module.path()
    ));
    let twin = parse(
        "* held X and Z, twin\n\
         vo oh 0 0.9\n\
         rho oh o 20\n\
         io 0 o dc 0 ac 1\n\
         ro o 0 1e12\n\
         vs s 0 dc 0.3 ac 1\n\
         rs s z 1k\n\
         rz z 0 1k\n\
         .end\n",
    );
    let engine = Engine::default();
    let op = engine
        .run_dc_op(&deck)
        .expect("the X/Z operating point solves");
    let level = op.try_voltage_named("o").expect("o is published");
    assert_close("X level", level, 0.9, 1.0e-9, 1.0e-9);
    let z = op.try_voltage_named("z").expect("z is published");
    assert_close("Z node", z, 0.15, 1.0e-9, 1.0e-9);

    let ac = engine.run_ac(&deck, &[1.0e3]).expect(".ac runs");
    let twin_ac = engine.run_ac(&twin, &[1.0e3]).expect("twin .ac");
    let impedance = ac_voltage(&ac[0], "o");
    assert_close("X output impedance", impedance.re, 20.0, 1.0e-9, 1.0e-6);
    assert_phasor_close("X node", impedance, ac_voltage(&twin_ac[0], "o"));
    assert_phasor_close(
        "transfer through the Z node",
        ac_voltage(&ac[0], "z"),
        ac_voltage(&twin_ac[0], "z"),
    );
    assert_close(
        "Z transfer",
        ac_voltage(&ac[0], "z").re,
        0.5,
        1.0e-12,
        1.0e-9,
    );
}

/// S5(a) and S5(b): the analog block reads a discrete variable as a
/// constant of the linearization, and the process that samples `V(a)` on a
/// 10 ns `timer` does not run during it, so every small-signal answer is the
/// twin's with the held code — whichever code the operating point sampled.
/// The timer is an analog event the digital process subscribes to through a
/// retained occurrence counter, which each analysis's switch of analysis
/// type must leave where the operating point put it.
#[test]
fn a_digital_read_of_an_analog_value_is_frozen_at_the_held_sample() {
    let module = SourceFile::new(
        "va",
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module xread(a, p);\n\
           input a; electrical a;\n\
           inout p; electrical p;\n\
           integer code;\n\
           initial code = 5;\n\
           always @(timer(0, 10n)) code = (V(a) > 0.5) ? 9 : 3;\n\
           analog I(p) <+ V(p) / 1k - code * 1.0e-4 * V(a);\n\
         endmodule\n",
    );
    let engine = Engine::default();
    for (bias, code) in [(0.7, 9.0), (0.3, 3.0)] {
        let deck = parse(&format!(
            "* digital read of an analog value\n\
             va a 0 dc {bias} ac 1\n\
             xr a p xread\n\
             vprobe p q 0\n\
             rp q 0 1k\n\
             .va \"{}\" xread\n\
             .end\n",
            module.path()
        ));
        // The module's `V(p)/1k` is a noiseless conductance, so the twin
        // spells it as a VCCS rather than a resistor.
        let twin = parse(&format!(
            "* held-code twin\n\
             va a 0 dc {bias} ac 1\n\
             gr p 0 p 0 1m\n\
             gx 0 p a 0 {}\n\
             vprobe p q 0\n\
             rp q 0 1k\n\
             .end\n",
            code * 1.0e-4
        ));
        let ac = engine.run_ac(&deck, &FREQUENCIES).expect(".ac runs");
        let twin_ac = engine.run_ac(&twin, &FREQUENCIES).expect("twin .ac");
        for (point, reference) in ac.iter().zip(&twin_ac) {
            assert_phasor_close(
                &format!("v(p) with the code held at {code}"),
                ac_voltage(point, "p"),
                ac_voltage(reference, "p"),
            );
        }
        let gain = ac_voltage(&ac[0], "p").re;
        assert_close("held-code transfer", gain, code * 0.05, 1.0e-12, 1.0e-9);

        let tf = engine
            .run_transfer_function(&deck, "p", None, false, "va")
            .expect(".tf runs");
        let twin_tf = engine
            .run_transfer_function(&twin, "p", None, false, "va")
            .expect("twin .tf");
        assert_close(".tf gain", tf.gain, code * 0.05, 1.0e-12, 1.0e-9);
        assert_close(".tf gain", tf.gain, twin_tf.gain, 1.0e-12, 1.0e-9);
        assert_close(
            ".tf output impedance",
            tf.output_impedance,
            twin_tf.output_impedance,
            1.0e-9,
            1.0e-9,
        );

        let stb_config = || {
            StbConfig::new()
                .with_sweep(1.0e3, 1.0e7, 5)
                .with_probe("vprobe")
        };
        let stb = engine.run_stb(&deck, stb_config()).expect(".stb runs");
        let twin_stb = engine.run_stb(&twin, stb_config()).expect("twin .stb");
        assert_eq!(stb.loop_gains.len(), twin_stb.loop_gains.len());
        for ((frequency, gain), reference) in stb
            .frequencies
            .iter()
            .zip(&stb.loop_gains)
            .zip(&twin_stb.loop_gains)
        {
            assert_phasor_close(
                &format!(".stb loop gain at {frequency} Hz"),
                *gain,
                *reference,
            );
        }

        let out = node_index(&engine, &deck, "p");
        let twin_out = node_index(&engine, &twin, "p");
        let noise = engine
            .run_noise(&deck, out, &FREQUENCIES, 300.15)
            .expect(".noise runs");
        let twin_noise = engine
            .run_noise(&twin, twin_out, &FREQUENCIES, 300.15)
            .expect("twin .noise");
        assert_eq!(noise.len(), twin_noise.len());
        for (point, reference) in noise.iter().zip(&twin_noise) {
            assert_close(
                &format!(".noise output density at {} Hz", point.frequency),
                point.output_noise_density,
                reference.output_noise_density,
                0.0,
                1.0e-9,
            );
        }
    }
}
