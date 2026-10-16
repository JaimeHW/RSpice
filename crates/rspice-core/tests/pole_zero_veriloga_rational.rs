//! Pole-zero over held event state and Verilog-A devices whose dynamic
//! operators are rational.
//!
//! Expected values are closed forms or analog-only twins (O5), never an
//! earlier RSpice answer.
#![cfg(feature = "veriloga")]

use rspice_core::{Complex64, Engine, Netlist};

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("deck parses")
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
