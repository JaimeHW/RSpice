//! TRNOISE end-to-end: transient noise sources through the full engine.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

#[test]
fn hierarchical_random_sources_match_scoped_flat_instances() {
    let engine = Engine::default();
    for random in [false, true] {
        for current in [false, true] {
            let waveform = |amplitude: &str, interval: &str| {
                if random {
                    format!("TRRANDOM(2 {interval} 0 {amplitude} 0)")
                } else {
                    format!("TRNOISE({amplitude} {interval} 0 0)")
                }
            };
            let source = if current { "I1 0 p" } else { "V1 p 0" };
            let hierarchical = Netlist::parse(&format!(
                "scoped random sources\n.subckt cell p params:gain=1 sample=1n\n{source} {} AC 2 DISTOF1 1\n.ends cell\n.subckt pair left right params:amp=1\nXleft left cell gain={{amp}} sample=1n\nXright right cell gain={{2*amp}} sample=2n\n.ends pair\nXtop out1 out2 pair amp=.01\nR1 out1 0 1\nR2 out2 0 1\n.options seed=42\n.end\n",
                waveform("{gain}", "{sample}"),
            )).unwrap();
            let names = engine.transient_source_names(&hierarchical).unwrap();
            assert_eq!(names.len(), 2);
            assert!(names[0].contains("Xleft") && names[1].contains("Xright"));
            let (left, right) = if current {
                ("Ileft 0 out1", "Iright 0 out2")
            } else {
                ("Vleft out1 0", "Vright out2 0")
            };
            let mut flat = Netlist::parse(&format!(
                "flat random instances\n{left} {} AC 2 DISTOF1 1\n{right} {} AC 2 DISTOF1 1\nR1 out1 0 1\nR2 out2 0 1\n.options seed=42\n.end\n",
                waveform(".01", "1n"), waveform(".02", "2n"),
            )).unwrap();
            // Preserve canonical identities so the independent reference draws
            // each instance's stream rather than a differently named source's.
            flat.elements[0].name.clone_from(&names[0]);
            flat.elements[1].name.clone_from(&names[1]);
            let expected = engine.run_tran(&flat, 10e-9, 1e-9).unwrap();
            let actual = engine.run_tran(&hierarchical, 10e-9, 1e-9).unwrap();
            assert_eq!(actual.node_names, expected.node_names);
            assert_eq!(actual.time, expected.time);
            assert_eq!(actual.voltages, expected.voltages);
            assert_eq!(actual.branch_currents, expected.branch_currents);
            for name in ["out1", "out2"] {
                let column = actual
                    .node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name))
                    .unwrap();
                assert!(
                    actual.voltages[column]
                        .iter()
                        .any(|value| value.abs() > 0.003)
                );
            }
            for name in &names {
                let selection = [name.clone()];
                assert_eq!(
                    engine
                        .transient_source_event_times(&hierarchical, 10e-9, 1e-9, &selection)
                        .unwrap(),
                    engine
                        .transient_source_event_times(&flat, 10e-9, 1e-9, &selection)
                        .unwrap(),
                );
            }
        }
    }
}

#[test]
fn uninstantiated_random_sources_do_not_expand_or_fail_the_run() {
    let netlist = Netlist::parse(
        "unused noise\nV1 out 0 1\nR1 out 0 1\n.subckt unused p\nVbad p 0 TRNOISE(1 0 0 0)\n.ends unused\n.end\n"
    ).unwrap();
    let result = Engine::default().run_tran(&netlist, 10e-9, 1e-9).unwrap();
    let output = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(result.voltages[output].iter().all(|value| *value == 1.0));
}

#[test]
fn distortion_annotations_preserve_seeded_transient_noise() {
    let engine = Engine::default();
    for waveform in ["TRNOISE(1 1n 0 0)", "TRRANDOM(2 1n 0 1 0)"] {
        for source in ["V1 out 0", "I1 0 out"] {
            for terms in ["", "AC 2", "DC .25 AC 2"] {
                let run = |annotation: &str| {
                    let netlist = Netlist::parse(&format!(
                        "annotated noise\n{source} {waveform} {terms} {annotation}\nR1 out 0 1\n.options seed=42\n.end\n"
                    )).unwrap();
                    engine.run_tran(&netlist, 10e-9, 1e-9).unwrap()
                };
                let expected = run("");
                let index = expected
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                assert!(
                    expected.voltages[index]
                        .iter()
                        .any(|value| value.abs() > 0.3),
                    "fixture must contain noise"
                );
                for annotation in ["DISTOF1 1", "DISTOF2 .5 90", "DISTOF1 1 DISTOF2 .5 90"] {
                    let actual = run(annotation);
                    assert_eq!(
                        actual.time, expected.time,
                        "{waveform} {terms} {annotation}"
                    );
                    assert_eq!(
                        actual.voltages, expected.voltages,
                        "{source} {waveform} {terms} {annotation}"
                    );
                }
            }
        }
    }
}

fn run_noise_deck(seed_line: &str) -> (Vec<f64>, Vec<f64>) {
    let deck = format!(
        "\
* trnoise bench
{seed_line}
v1 in 0 trnoise(1m 1n 0 0)
r1 in out 1k
r2 out 0 1k
.tran 1n 2u
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&netlist, 2e-6, 1e-9)
        .expect("transient with noise source runs");
    let out_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present");
    (result.time.clone(), result.voltages[out_idx].clone())
}

#[test]
fn white_noise_reaches_the_output_with_expected_statistics() {
    let (time, v_out) = run_noise_deck(".options seed=11");

    // The divider halves the 1 mV RMS source noise. Points *between* noise
    // samples are linear interpolations of two Gaussians (reduced variance
    // by construction), so the sharp invariant is the variance measured at
    // the NT sample grid itself. Solver timepoints land on the grid because
    // every PWL vertex is a breakpoint.
    const NT: f64 = 1e-9;
    let on_grid: Vec<f64> = time
        .iter()
        .zip(&v_out)
        .filter(|(t, _)| {
            let k = (**t / NT).round();
            (**t - k * NT).abs() < 1e-15
        })
        .map(|(_, v)| *v)
        .collect();

    assert!(
        on_grid.len() >= 500,
        "solver must land on the noise sample grid (breakpoints honored); \
         only {} of {} points were on-grid",
        on_grid.len(),
        time.len()
    );

    let n = on_grid.len() as f64;
    let mean = on_grid.iter().sum::<f64>() / n;
    let var = on_grid.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let expected = (0.5e-3f64).powi(2);

    assert!(
        mean.abs() < 5e-5,
        "noise is zero-mean at the output, got mean {mean}"
    );
    assert!(
        (var - expected).abs() / expected < 0.12,
        "on-grid output variance within 12% of (NA/2)^2: got {var}, want {expected}"
    );
}

#[test]
fn runs_are_bit_identical_for_a_fixed_seed() {
    let (t1, v1) = run_noise_deck(".options seed=42");
    let (t2, v2) = run_noise_deck(".options seed=42");
    assert_eq!(t1.len(), t2.len(), "identical time grids");
    assert!(
        t1.iter().zip(&t2).all(|(a, b)| a == b) && v1.iter().zip(&v2).all(|(a, b)| a == b),
        "fixed seed must reproduce the run bit-identically"
    );
}

#[test]
fn different_seeds_give_different_sample_paths() {
    let (_, v1) = run_noise_deck(".options seed=1");
    let (_, v2) = run_noise_deck(".options seed=2");
    assert!(
        v1.iter().zip(&v2).any(|(a, b)| a != b),
        "different seeds must give different noise"
    );
}

#[test]
fn operating_point_sees_zero() {
    let deck = "\
* trnoise op
v1 in 0 trnoise(10m 1n 0 0)
r1 in out 1k
r2 out 0 1k
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("op solves");
    for (name, idx) in op.node_names.iter().zip(0..) {
        if name.eq_ignore_ascii_case("in") || name.eq_ignore_ascii_case("out") {
            assert!(
                op.node_voltages[idx].abs() < 1e-12,
                "OP must see 0 from a zero-mean noise source at {name}"
            );
        }
    }
}

#[test]
fn rts_tail_is_accepted_and_requires_positive_dwell_times() {
    let deck = "\
* trnoise rts
v1 in 0 trnoise(1m 1n 0 0 5m 10u 20u)
r1 in 0 1k
.tran 1n 1u
.end
";
    Netlist::parse(deck).expect("RTS capture and emission mean times are supported");

    // A dwell time is the mean of an exponential draw, so zero or negative
    // means there is no distribution to sample. Only the all-zero tail means
    // "no RTS"; a half-specified one must still fail closed.
    let half_specified = "\
* trnoise rts
v1 in 0 trnoise(1m 1n 0 0 5m 0 20u)
r1 in 0 1k
.tran 1n 1u
.end
";
    let err = Netlist::parse(half_specified).expect_err("a zero RTS dwell time must be rejected");
    let msg = format!("{err}");
    assert!(
        msg.contains("RTS"),
        "diagnostic names the rejected feature: {msg}"
    );
}
