//! Pole-zero over held event state and Verilog-A devices whose dynamic
//! operators are rational.
//!
//! Each Verilog-A instance hands the extraction its exact `G + sC` descriptor: `ddt`
//! charge, every `laplace_*` filter's realization and every `idt` become
//! explicit states, so the poles are the circuit's and not a two-frequency
//! fit. Expected values are closed forms or analog-only twins (O5), never an
//! earlier RSpice answer.
#![cfg(feature = "veriloga")]

use rspice_core::{Complex64, Engine, Netlist, SimulationError};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A source file the deck includes, removed when the test ends.
struct SourceFile(std::path::PathBuf);

impl SourceFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_pz_rational_{}_{}.va",
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

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("deck parses")
}

fn node(netlist: &Netlist, name: &str) -> usize {
    Engine::default()
        .build_circuit(netlist)
        .expect("circuit builds")
        .get_node_by_name(name)
        .unwrap_or_else(|| panic!("node {name} exists"))
}

/// Sort roots by real part, then imaginary part, for a pairwise comparison.
fn sorted(mut roots: Vec<Complex64>) -> Vec<Complex64> {
    roots.sort_by(|a, b| a.re.total_cmp(&b.re).then(a.im.total_cmp(&b.im)));
    roots
}

fn assert_roots(what: &str, actual: &[Complex64], expected: &[Complex64], rel: f64) {
    let actual = sorted(actual.to_vec());
    let expected = sorted(expected.to_vec());
    assert_eq!(
        actual.len(),
        expected.len(),
        "{what}: {actual:?} against {expected:?}"
    );
    for (root, reference) in actual.iter().zip(&expected) {
        assert!(
            (root - reference).norm() <= rel * reference.norm().max(1.0),
            "{what}: {root} against {reference} (all: {actual:?})"
        );
    }
}

fn real(values: &[f64]) -> Vec<Complex64> {
    values.iter().map(|&re| Complex64::new(re, 0.0)).collect()
}

/// `V(o) = laplace_nd(V(a), {1}, {1, tau})` behind the module's own 1k, on
/// an RC input node: poles at `-1/RC` and `-1/tau`, and no zero.
#[test]
fn an_rc_with_a_laplace_filter_has_both_closed_form_poles() {
    let module = SourceFile::new(
        "`include \"disciplines.vams\"\n\
         module lpf(a, o);\n\
           inout a, o; electrical a, o;\n\
           analog I(o) <+ 1.0e-3 * V(o) - 1.0e-3 * laplace_nd(V(a), '{1.0}, '{1.0, 1.0e-7});\n\
         endmodule\n",
    );
    let deck = parse(&format!(
        "* RC into a laplace filter\n\
         i1 0 a dc 0 ac 1\n\
         r1 a 0 1k\n\
         c1 a 0 1n\n\
         x1 a o lpf\n\
         .va \"{}\" lpf\n\
         .end\n",
        module.path()
    ));
    let engine = Engine::default();
    let result = engine
        .run_pz(&deck, node(&deck, "a"), node(&deck, "o"))
        .expect("a laplace filter exports its realization");
    assert_roots("poles", &result.poles, &real(&[-1.0e6, -1.0e7]), 1.0e-9);
    assert!(
        result.zeros.is_empty(),
        "no finite zero: {:?}",
        result.zeros
    );
    let gain = result.dc_gain.expect("finite DC gain");
    assert!((gain.abs() - 1.0e3).abs() <= 1.0e-9 * 1.0e3, "{gain}");
    let spectrum = engine
        .run_pole_spectrum(&deck)
        .expect("natural poles of the same circuit");
    assert_roots(
        "natural poles",
        &spectrum.poles,
        &real(&[-1.0e6, -1.0e7]),
        1.0e-9,
    );
}

/// `V(o) = -idt(V(x))/tau` with `x` the midpoint of two equal resistors
/// from a grounded input to `o`: `Vo = -Vo/(2 s tau)`, a pole at
/// `-1/(2 tau)`. Without an initial condition the integrator is an internal
/// node of the module; with one it is an explicit descriptor state.
#[test]
fn an_integrator_in_resistive_feedback_has_its_analytic_pole() {
    for (form, integral) in [
        ("implicit", "idt(V(x))"),
        ("initial condition", "idt(V(x), 0.0)"),
    ] {
        let module = SourceFile::new(&format!(
            "`include \"disciplines.vams\"\n\
             module integ(x, o);\n\
               inout x, o; electrical x, o;\n\
               analog V(o) <+ -{integral} / 1.0e-6;\n\
             endmodule\n"
        ));
        let deck = parse(&format!(
            "* integrator in resistive feedback\n\
             vin in 0 dc 0 ac 1\n\
             r1 in x 1k\n\
             r2 x o 1k\n\
             x1 x o integ\n\
             .va \"{}\" integ\n\
             .end\n",
            module.path()
        ));
        let spectrum = Engine::default()
            .run_pole_spectrum(&deck)
            .unwrap_or_else(|error| panic!("{form}: the integrator is rational: {error}"));
        assert_roots(form, &spectrum.poles, &real(&[-5.0e5]), 1.0e-9);
    }
}

/// A module that integrates only charge is the native R and C it spells:
/// the same matrix entries in the same order, so the same roots and gain to
/// the last bit.
#[test]
fn a_ddt_only_module_gives_its_native_twins_poles_bit_for_bit() {
    let module = SourceFile::new(
        "`include \"disciplines.vams\"\n\
         module rc_load(p, n);\n\
           inout p, n; electrical p, n;\n\
           analog I(p, n) <+ 1.0e-3 * V(p, n) + ddt(1.0e-12 * V(p, n));\n\
         endmodule\n",
    );
    let deck = parse(&format!(
        "* ddt-only load\n\
         i1 in 0 dc 0 ac 1\n\
         r1 in out 1k\n\
         rl out 0 1k\n\
         x1 out 0 rc_load\n\
         .va \"{}\" rc_load\n\
         .end\n",
        module.path()
    ));
    let twin = parse(
        "* native twin\n\
         i1 in 0 dc 0 ac 1\n\
         r1 in out 1k\n\
         rl out 0 1k\n\
         rx out 0 1k\n\
         cx out 0 1p\n\
         .end\n",
    );
    let engine = Engine::default();
    let ports = |netlist: &Netlist| (node(netlist, "in"), node(netlist, "out"));
    let (input, output) = ports(&deck);
    let result = engine.run_pz(&deck, input, output).expect(".pz runs");
    let (twin_input, twin_output) = ports(&twin);
    let reference = engine
        .run_pz(&twin, twin_input, twin_output)
        .expect("twin .pz");
    let bits = |roots: &[Complex64]| {
        roots
            .iter()
            .map(|root| (root.re.to_bits(), root.im.to_bits()))
            .collect::<Vec<_>>()
    };
    assert_eq!(bits(&result.poles), bits(&reference.poles));
    assert_eq!(bits(&result.zeros), bits(&reference.zeros));
    assert_eq!(
        result.dc_gain.map(f64::to_bits),
        reference.dc_gain.map(f64::to_bits)
    );
    assert_roots("poles", &result.poles, &real(&[-2.0e9]), 1.0e-9);
    let spectrum = engine.run_pole_spectrum(&deck).expect("natural poles");
    let twin_spectrum = engine.run_pole_spectrum(&twin).expect("twin natural poles");
    assert_eq!(bits(&spectrum.poles), bits(&twin_spectrum.poles));
}

/// A mixed module whose held trim code scales a laplace filter, against the
/// analog-only twin of its held state: a VCCS of the held gain into the
/// filter's RC.
#[test]
fn a_mixed_host_with_laplace_and_trim_matches_its_twin() {
    let module = SourceFile::new(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module trimmed_lpf(a, o);\n\
           inout a, o; electrical a, o;\n\
           integer trim;\n\
           initial trim = 10;\n\
           analog I(o) <+ 1.0e-3 * V(o) - 1.0e-4 * trim * laplace_nd(V(a), '{1.0}, '{1.0, 1.0e-7});\n\
         endmodule\n",
    );
    let deck = parse(&format!(
        "* mixed laplace with a held trim\n\
         i1 0 a dc 0 ac 1\n\
         r1 a 0 1k\n\
         c1 a 0 1n\n\
         x1 a o trimmed_lpf\n\
         rl o 0 10k\n\
         .va \"{}\" trimmed_lpf\n\
         .end\n",
        module.path()
    ));
    let twin = parse(
        "* held-trim twin\n\
         i1 0 a dc 0 ac 1\n\
         r1 a 0 1k\n\
         c1 a 0 1n\n\
         gf 0 f a 0 1m\n\
         rf f 0 1k\n\
         cf f 0 100p\n\
         go 0 o f 0 1m\n\
         ro o 0 1k\n\
         rl o 0 10k\n\
         .end\n",
    );
    let engine = Engine::default();
    let result = engine
        .run_pz(&deck, node(&deck, "a"), node(&deck, "o"))
        .expect("pole-zero runs on the held mixed module");
    let reference = engine
        .run_pz(&twin, node(&twin, "a"), node(&twin, "o"))
        .expect("twin .pz");
    assert_roots("poles", &result.poles, &reference.poles, 1.0e-9);
    assert_roots("poles", &result.poles, &real(&[-1.0e6, -1.0e7]), 1.0e-9);
    assert_roots("zeros", &result.zeros, &reference.zeros, 1.0e-9);
    let (gain, twin_gain) = (
        result.dc_gain.expect("finite DC gain"),
        reference.dc_gain.expect("twin DC gain"),
    );
    // 1k into the filter, the held trim's 1e-4*10 into 1k || 10k.
    assert!(
        (gain - twin_gain).abs() <= 1.0e-9 * twin_gain.abs(),
        "{gain} {twin_gain}"
    );
    assert!(
        (gain - 1.0e3 * 1.0e-3 * (1.0e4 / 11.0)).abs() <= 1.0e-9 * gain.abs(),
        "{gain}"
    );
}

/// `absdelay` has no finite rational descriptor: the extraction refuses,
/// naming the instance and its module as typed fields.
#[test]
fn a_transport_delay_refuses_by_instance_and_module() {
    let module = SourceFile::new(
        "`include \"disciplines.vams\"\n\
         module delayed(p, n);\n\
           inout p, n; electrical p, n;\n\
           analog I(p, n) <+ 1.0e-3 * absdelay(V(p, n), 1.0e-9);\n\
         endmodule\n",
    );
    let deck = parse(&format!(
        "* transport delay\n\
         i1 in 0 dc 0 ac 1\n\
         r1 in out 1k\n\
         rl out 0 1k\n\
         x1 out 0 delayed\n\
         .va \"{}\" delayed\n\
         .end\n",
        module.path()
    ));
    let engine = Engine::default();
    for error in [
        engine
            .run_pz(&deck, node(&deck, "in"), node(&deck, "out"))
            .expect_err("exp(-s td) is not rational"),
        engine
            .run_pole_spectrum(&deck)
            .expect_err("nor are its natural poles finite"),
    ] {
        let SimulationError::UnsupportedCapability(refusal) = &error else {
            panic!("a capability refusal: {error}");
        };
        assert_eq!(refusal.capability, "analysis.pz.non_rational_operator");
        assert_eq!(
            refusal.analysis_kind,
            Some(rspice_core::identity::AnalysisKind::PoleZero)
        );
        assert!(
            refusal
                .instance
                .as_deref()
                .is_some_and(|instance| instance.eq_ignore_ascii_case("x1")),
            "{error}"
        );
        assert_eq!(refusal.module.as_deref(), Some("delayed"), "{error}");
        assert!(error.to_string().contains("absdelay"), "{error}");
    }
}

/// Held converters and real-number boundary models stamp their held level
/// and nothing dynamic: the natural poles are those of the analog network
/// with each driven node replaced by an ideal source at its level.
#[test]
fn held_converters_take_part_as_their_held_stamps() {
    let deck = parse(
        "* held converters\n\
         i1 0 in dc 0 ac 1\n\
         r1 in 0 1k\n\
         c1 in 0 1n\n\
         aadc [in] [d] adcm\n\
         .model adcm adc_bridge (in_low=0.25 in_high=0.25)\n\
         ainv d dn invm\n\
         .model invm d_inverter\n\
         adac [dn] [xq] dacm\n\
         .model dacm dac_bridge (out_low=0 out_high=1)\n\
         rq xq o 1k\n\
         ro o 0 1k\n\
         co o 0 1n\n\
         aobs in rv obs\n\
         .model obs v_to_real (gain=1)\n\
         arv rv xr drv\n\
         .model drv real_to_v\n\
         rr xr r 1k\n\
         cr r 0 1n\n\
         .end\n",
    );
    let twin = parse(
        "* held-source twin\n\
         i1 0 in dc 0 ac 1\n\
         r1 in 0 1k\n\
         c1 in 0 1n\n\
         vq xq 0 dc 1\n\
         rq xq o 1k\n\
         ro o 0 1k\n\
         co o 0 1n\n\
         vr xr 0 dc 0\n\
         rr xr r 1k\n\
         cr r 0 1n\n\
         .end\n",
    );
    let engine = Engine::default();
    let spectrum = engine
        .run_pole_spectrum(&deck)
        .expect("held converters take part in pole-zero");
    let reference = engine.run_pole_spectrum(&twin).expect("twin natural poles");
    assert_roots("natural poles", &spectrum.poles, &reference.poles, 1.0e-9);
    assert_roots(
        "natural poles",
        &spectrum.poles,
        &real(&[-1.0e6, -1.0e6, -2.0e6]),
        1.0e-9,
    );
}
