//! TRNOISE end-to-end: transient noise sources through the full engine.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

#[test]
fn large_poisson_current_sources_preserve_their_mean_through_a_shunt() {
    let engine = Engine::default();
    for lambda in [64.0_f64, 1e6, 1e18, 1e20, 1e100] {
        let netlist = Netlist::parse(&format!(
            "Poisson shunt\nI1 0 out TRRANDOM(4 1n 0 {lambda} 0)\nR1 out 0 {}\n.end\n",
            1.0 / lambda
        ))
        .unwrap();
        let result = engine
            .run_tran(&netlist, 4e-9, 1e-9)
            .unwrap_or_else(|error| panic!("lambda={lambda}: {error}"));
        let output = result.try_voltage_waveform_named("out").unwrap();
        assert_eq!(output[0], 0.0);
        for value in output.iter().skip(1) {
            assert!(
                (value - 1.0).abs() < 12.0 / lambda.sqrt() + 1e-12,
                "lambda={lambda}, normalized sample={value}"
            );
        }
        assert_eq!(result.time.last().copied(), Some(4e-9));
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }
}

#[test]
fn centered_poisson_noise_preserves_fluctuations_through_large_offset_cancellation() {
    use std::sync::Arc;
    let grid: Vec<_> = (0..=256).map(|index| f64::from(index) * 1e-9).collect();
    let engine = Engine::new(SimulationConfig {
        locked_time_grid: Some(Arc::new(grid.clone())),
        ..Default::default()
    });
    for lambda in [1e20_f64, 1e40, 1e100] {
        let mut reference = None;
        for (bias, dc) in [
            (-lambda, 0.0),
            (0.0, -lambda),
            (-0.5 * lambda, -0.5 * lambda),
        ] {
            let netlist = Netlist::parse(&format!(
                "centered Poisson shunt\nI1 0 out DC {dc} TRRANDOM(4 1n 0 {lambda} {bias}) AC 1 DISTOF1 1\nR1 out 0 {}\n.end\n", 1.0/lambda.sqrt()
            )).unwrap();
            let result = engine.run_tran(&netlist, grid[256], 1e-9).unwrap();
            assert_eq!(result.time, grid);
            let output = result.try_voltage_waveform_named("out").unwrap().to_vec();
            let samples = &output[1..];
            let mean = samples.iter().sum::<f64>() / samples.len() as f64;
            let variance =
                samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / samples.len() as f64;
            assert!(
                mean.abs() < 0.5 && (variance - 1.0).abs() < 0.6,
                "lambda={lambda}: mean={mean}, variance={variance}"
            );
            if let Some(expected) = &reference {
                assert_eq!(
                    &output, expected,
                    "offset spelling must preserve every sample"
                );
            } else {
                reference = Some(output);
            }
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

#[test]
fn trnoise_startup_matches_zero_origin_and_explicit_dc_bias() {
    let engine = Engine::default();
    for waveform in [
        "TRNOISE(1 1n 0 0)",
        "TRNOISE(0 1n 1 1)",
        "TRNOISE(1 1n 1 1)",
    ] {
        for source in ["V1 out 0", "I1 0 out"] {
            for dc in ["", "DC .25"] {
                let netlist = Netlist::parse(&format!(
                    "noise startup\n{source} {dc} {waveform} AC 2 DISTOF1 1\nR1 out 0 1\n.end\n"
                ))
                .unwrap();
                let result = engine.run_tran(&netlist, 10e-9, 1e-9).unwrap();
                let output = result
                    .node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case("out"))
                    .unwrap();
                assert_eq!(result.time[0], 0.0);
                let expected = if dc.is_empty() { 0.0 } else { 0.25 };
                assert!(
                    (result.voltages[output][0] - expected).abs() < 1e-14,
                    "{source} {dc} {waveform}: {}",
                    result.voltages[output][0]
                );
                assert!(
                    result.voltages[output]
                        .iter()
                        .skip(1)
                        .any(|value| value.abs() > 0.1)
                );
            }
        }
    }
}

#[test]
fn explicit_dc_offsets_the_complete_random_waveform() {
    use std::sync::Arc;
    let grid: Vec<_> = (0..=32).map(|index| f64::from(index) * 1e-9).collect();
    let engine = Engine::new(SimulationConfig {
        locked_time_grid: Some(Arc::new(grid.clone())),
        ..SimulationConfig::default()
    });
    for waveform in ["TRNOISE(1 1n 1 1 1 .7n .9n)", "TRRANDOM(2 1n .3n 1 0)"] {
        for source in ["V1 out 0", "I1 0 out"] {
            let run = |dc: &str| {
                let deck = Netlist::parse(&format!(
                    "random DC offset\n{source} {dc} {waveform} AC 2 DISTOF1 1\nR1 out 0 1\n.end\n"
                ))
                .unwrap();
                engine.run_tran(&deck, grid[32], 1e-9).unwrap()
            };
            let baseline = run("");
            let biased = run("DC .25");
            assert_eq!(baseline.time, biased.time);
            let base = baseline.try_voltage_waveform_named("out").unwrap();
            let offset = biased.try_voltage_waveform_named("out").unwrap();
            for (a, b) in base.iter().zip(offset) {
                assert!(
                    (b - a - 0.25).abs() < 2e-14,
                    "{source} {waveform}: {a} -> {b}"
                );
            }
        }
    }
}

#[test]
fn disabled_flicker_is_zero_without_allocating_a_noise_sample_train() {
    let netlist =
        Netlist::parse("disabled flicker\nV1 out 0 TRNOISE(0 0 0 1)\nR1 out 0 1\n.end\n").unwrap();
    let result = Engine::default().run_tran(&netlist, 10e-9, 1e-9).unwrap();
    assert!(result.voltages.iter().flatten().all(|value| *value == 0.0));
}

#[test]
fn extending_noise_horizons_preserves_every_locked_sample() {
    use std::sync::Arc;
    let grid: Vec<_> = (0..=129).map(|index| f64::from(index) * 0.3e-9).collect();
    let engine = Engine::new(SimulationConfig {
        locked_time_grid: Some(Arc::new(grid.clone())),
        ..SimulationConfig::default()
    });
    for waveform in [
        "TRNOISE(0 1n 1 1)",
        "TRNOISE(1 1n 1 1 1 .7n .9n)",
        "TRNOISE(0 0 0 0 1 .7n .9n)",
        "TRRANDOM(2 1n .3n 1 0)",
        "TRRANDOM(4 1n .3n 64 0)",
        "TRRANDOM(4 1n .3n 1e100 -1e100)",
    ] {
        let netlist = Netlist::parse(&format!(
            "noise horizon\nV1 out 0 {waveform}\nR1 out 0 1\n.end\n"
        ))
        .unwrap();
        let full = engine.run_tran(&netlist, grid[129], 1e-9).unwrap();
        let short = engine.run_tran(&netlist, grid[57], 1e-9).unwrap();
        assert_eq!(short.time, full.time[..short.time.len()]);
        for (actual, expected) in short
            .voltages
            .iter()
            .zip(&full.voltages)
            .chain(short.branch_currents.iter().zip(&full.branch_currents))
        {
            for (a, b) in actual.iter().zip(expected) {
                assert_eq!(a.to_bits(), b.to_bits(), "{waveform}");
            }
        }
    }
}

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
