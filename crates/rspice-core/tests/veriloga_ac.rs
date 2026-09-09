//! AC small-signal pins for Verilog-A reactive elements.
//!
//! ddt() charges previously contributed nothing to AC analysis (only the
//! resistive Jacobian was stamped), so any Verilog-A capacitor or
//! inductor was invisible to .ac. These tests pin the reactive stamping
//! (jw * dQ/dx) against closed-form single-pole responses.
#![cfg(feature = "veriloga")]

use rspice_core::engine::SimulationConfig;
use rspice_core::{Engine, ModelFinishPoint, Netlist, NoAbort, SimulationOutcome};
use std::io::Write;
use std::path::PathBuf;

fn write_model(name: &str, source: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("rspice_ac_{}_{}.va", name, std::process::id()));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path
}

fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

#[test]
fn sp_finish_before_frequency_results_preserves_completion_metadata() {
    let model = write_model(
        "sp_finish_before_frequency",
        r#"module sp_finish_before_frequency(p,n);
inout p,n; electrical p,n;
parameter integer finish_initial=0;
analog initial if (finish_initial) $finish(1);
analog begin
    @(initial_step("ac")) if (!finish_initial) $finish(2);
    I(p,n)<+1e-3*V(p,n);
end
endmodule"#,
    );
    for finish_initial in [false, true] {
        let netlist = Netlist::parse(&format!(
            "* SP normal completion\nV1 in 0 AC 1 portnum=1 z0=50\nX1 in 0 sp_finish_before_frequency finish_initial={}\n.va \"{}\" sp_finish_before_frequency\n.end\n",
            usize::from(finish_initial), deck_path(&model),
        )).unwrap();
        let outcome = Engine::default()
            .run_with_outcome(&NoAbort, |engine, signal| {
                engine.run_sp_over_grid_with_abort(&netlist, &[10.0, 20.0], false, signal)
            })
            .expect("an accepted model finish is normal SP completion");
        let SimulationOutcome::Finished { result, finish } = outcome else {
            panic!("SP must retain model completion from its AC bias solve");
        };
        assert!(result.is_none());
        assert_eq!(finish.model, "sp_finish_before_frequency");
        assert!(finish.instance.eq_ignore_ascii_case("X1"));
        assert_eq!(finish.diagnostic_level, if finish_initial { 1 } else { 2 });
        assert_eq!(
            finish.point,
            if finish_initial {
                ModelFinishPoint::Initialization
            } else {
                ModelFinishPoint::OperatingPoint
            },
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn sp_frequency_finish_retains_every_port_and_noise_at_the_final_state() {
    let frequencies = (1..=13)
        .map(|index| f64::from(index) * 10.0)
        .collect::<Vec<_>>();
    let model = write_model(
        "sp_frequency_finish",
        r#"module sp_frequency_finish(p,n);
inout p,n; electrical p,n;
parameter integer finish_phase=1, finish_early=1;
real count;
analog begin
    @(initial_step) count=1;
    @(final_step) begin
        count=count+1;
        if ((finish_phase==1 && analysis("ac")) ||
            (finish_phase==3 && analysis("noise"))) $finish(2);
    end
    if (finish_early && !analysis("static") && count==1 &&
        ((finish_phase==1 && analysis("ac")) ||
         (finish_phase==3 && analysis("noise")))) $finish(1);
    I(p,n)<+count*0.02*V(p,n);
    I(p,n)<+white_noise(4*1.380649e-23*$temperature*count*0.02, "thermal");
end
endmodule"#,
    );
    for (phase, do_noise, source) in [
        (0, false, "V"),
        (1, false, "V"),
        (0, true, "V"),
        (1, true, "V"),
        (3, true, "V"),
        (0, true, "P"),
        (1, true, "P"),
        (3, true, "P"),
    ] {
        for early in [false, true] {
            let netlist = Netlist::parse(&format!(
                "* Atomic SP frequency point\n{source}1 p1 0 AC 1 portnum=1 z0=50\n{source}2 p2 0 AC 0 portnum=2 z0=50\nX1 p1 p2 sp_frequency_finish finish_phase={phase} finish_early={}\n.va \"{}\" sp_frequency_finish\n.end\n",
                usize::from(early), deck_path(&model),
            )).unwrap();
            let mut config = SimulationConfig::default();
            config.resource_limits.max_parallel_workers = 4;
            let outcome = Engine::new(config)
                .run_with_outcome(&NoAbort, |engine, signal| {
                    engine.run_sp_over_grid_with_abort(&netlist, &frequencies, do_noise, signal)
                })
                .unwrap_or_else(|error| {
                    panic!("phase={phase}, early={early}, noise={do_noise}: {error}")
                });
            let requested_finish = phase != 0;
            let expected_points = if requested_finish && early {
                1
            } else {
                frequencies.len()
            };
            let run = match outcome {
                SimulationOutcome::Completed(run) => {
                    assert!(!requested_finish);
                    run
                }
                SimulationOutcome::Finished { result, finish } => {
                    assert!(requested_finish);
                    assert_eq!(finish.diagnostic_level, if early { 1 } else { 2 });
                    assert_eq!(
                        finish.point,
                        ModelFinishPoint::Frequency {
                            frequency: if early { 10.0 } else { 130.0 },
                        }
                    );
                    result.expect("a completed SP frequency contains every excitation")
                }
            };
            assert_eq!(run.scattering.data.len(), expected_points);
            assert_eq!(run.port_noise.is_some(), do_noise);
            for (index, matrix) in run.scattering.data.iter().enumerate() {
                let final_point = index + 1 == expected_points;
                let resistance = if final_point { 25.0 } else { 50.0 };
                let reflection = resistance / (100.0 + resistance);
                let transmission = 100.0 / (100.0 + resistance);
                for (actual, expected) in [
                    (matrix.s11(), reflection),
                    (matrix.s22(), reflection),
                    (matrix.s12(), transmission),
                    (matrix.s21(), transmission),
                ] {
                    assert!(
                        (actual.re - expected).abs() < 1e-10 && actual.im.abs() < 1e-10,
                        "phase={phase}, early={early}, noise={do_noise}, point={index}: {actual} versus {expected}"
                    );
                }
                if let Some(noise) = &run.port_noise {
                    assert_eq!(noise.points.len(), expected_points);
                    let point = &noise.points[index];
                    assert_eq!(point.frequency, matrix.frequency);
                    let expected_psd = 4.0 * 1.380649e-23 * 300.15 / resistance;
                    for row in 0..2 {
                        for column in 0..2 {
                            let sign = if row == column { 1.0 } else { -1.0 };
                            let actual = point.current_correlation[row][column];
                            assert!(
                                (actual.re / expected_psd - sign).abs() < 1e-10
                                    && actual.im.abs() < 1e-30,
                                "phase={phase}, early={early}, point={index}, Cy({row},{column})={actual}, PSD={expected_psd}"
                            );
                        }
                    }
                }
            }
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn sp_scattering_and_noise_share_the_nonlinear_port_bias() {
    let model = write_model(
        "sp_quadratic_bias",
        r#"module sp_quadratic_bias(p,n);
inout p,n; electrical p,n;
analog begin
    I(p,n) <+ 0.02*V(p,n)*V(p,n);
    I(p,n) <+ white_noise(1e-20*V(p,n)*V(p,n), "bias");
end
endmodule"#,
    );
    // KCL: (1-v)/50 = .02 v^2, hence v=(sqrt(5)-1)/2.
    let bias = (5.0_f64.sqrt() - 1.0) / 2.0;
    let conductance = 0.04 * bias;
    let expected_s = (1.0 - 50.0 * conductance) / (1.0 + 50.0 * conductance);
    for source in ["V", "P"] {
        let netlist = Netlist::parse(&format!(
            "* Shared nonlinear RF bias\n{source}1 p 0 DC 1 portnum=1 z0=50\nX1 p 0 sp_quadratic_bias\n.va \"{}\" sp_quadratic_bias\n.end\n", deck_path(&model),
        )).unwrap();
        let result = Engine::default()
            .run_sp_over_grid_with_abort(&netlist, &[10.0, 20.0], true, &NoAbort)
            .unwrap();
        for (matrix, point) in result
            .scattering
            .data
            .iter()
            .zip(result.port_noise.unwrap().points)
        {
            assert!((matrix.s11().re - expected_s).abs() < 1e-10);
            assert!(
                (point.current_correlation[0][0].re / (1e-20 * bias * bias) - 1.0).abs() < 1e-10,
                "{source}: {}",
                point.current_correlation[0][0]
            );
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn sp_physical_port_noise_retains_bias_and_uses_the_noise_operator() {
    let model = write_model(
        "sp_biased_noise",
        r#"module sp_biased_noise(p,n);
inout p,n; electrical p,n;
analog begin
    I(p,n) <+ ((analysis("noise") && !analysis("static")) ? 0.04 : 0.02)*V(p,n);
    I(p,n) <+ white_noise(V(p,n)*V(p,n)*1e-20, "biased");
end
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* Preserve physical port bias\nP1 p 0 DC 1 PORT=1 Z0=50\nX1 p 0 sp_biased_noise\n.va \"{}\" sp_biased_noise\n.end\n",
        deck_path(&model),
    )).unwrap();
    let result = Engine::default()
        .run_sp_over_grid_with_abort(&netlist, &[10.0, 20.0], true, &NoAbort)
        .unwrap();
    for point in result.port_noise.unwrap().points {
        // The 1 V generator and two equal DC resistances put the DUT at 0.5 V.
        // Its Norton PSD is Vbias^2 * 1e-20 even though the noise-phase
        // conductance (0.04 S) differs from both the DC and AC value (0.02 S).
        let actual = point.current_correlation[0][0];
        assert!((actual.re / 2.5e-21 - 1.0).abs() < 1e-10, "{actual}");
        assert_eq!(actual.im, 0.0);
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn sp_noise_failure_does_not_publish_a_pending_ac_finish() {
    let model = write_model(
        "sp_failed_final_noise",
        r#"module sp_failed_final_noise(p,n);
inout p,n; electrical p,n;
parameter integer bad_task=0, early=1;
real psd;
analog begin
    @(initial_step) psd=1e-22;
    @(final_step) begin
        if (analysis("ac")) $finish(2);
        if (analysis("noise")) begin
            if (bad_task) $finish(99);
            else psd=-1;
        end
    end
    if (early && analysis("ac") && !analysis("static")) $finish(1);
    I(p,n)<+0.02*V(p,n);
    I(p,n)<+white_noise(psd, "thermal");
end
endmodule"#,
    );
    for bad_task in [false, true] {
        for early in [false, true] {
            let netlist = Netlist::parse(&format!(
                "* Failed SP transaction\nV1 p1 0 AC 1 portnum=1 z0=50\nV2 p2 0 AC 0 portnum=2 z0=50\nX1 p1 p2 sp_failed_final_noise bad_task={} early={}\n.va \"{}\" sp_failed_final_noise\n.end\n",
                usize::from(bad_task), usize::from(early), deck_path(&model),
            )).unwrap();
            let outcome = Engine::default().run_with_outcome(&NoAbort, |engine, signal| {
                let result =
                    engine.run_sp_over_grid_with_abort(&netlist, &[10.0, 20.0], true, signal);
                assert!(signal.model_control().unwrap().finish().is_none());
                result
            });
            assert!(
                matches!(outcome, Err(rspice_core::SimulationError::Circuit(_))),
                "bad_task={bad_task}, early={early}: {outcome:?}"
            );
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn ac_portless_finish_runs_the_analysis_lifecycle() {
    let model = write_model(
        "portless_finish",
        r#"
module portless_finish;
parameter integer at_bias=0, bad_final=0;
real count;
analog begin
    @(initial_step("ac")) begin count=1; if (at_bias) $finish(1); end
    @(final_step("ac")) begin count=count+1; if (bad_final) $finish(99); else $finish(2); end
    if (!analysis("static") && count==1) $finish(1);
end
endmodule"#,
    );
    for at_bias in [false, true] {
        for bad_final in [false, true] {
            let netlist = Netlist::parse(&format!(
                "* Portless lifecycle\nX1 portless_finish at_bias={} bad_final={}\n.va \"{}\" portless_finish\n.end\n",
                usize::from(at_bias), usize::from(bad_final), deck_path(&model),
            )).unwrap();
            let outcome = Engine::default().run_with_outcome(&NoAbort, |engine, signal| {
                engine.run_ac_with_abort(&netlist, &[10.0, 20.0], signal)
            });
            if bad_final {
                assert!(
                    outcome.is_err(),
                    "final-step failure must not publish successful finish: {outcome:?}"
                );
                continue;
            }
            let SimulationOutcome::Finished { result, finish } = outcome.unwrap() else {
                panic!("portless model did not finish");
            };
            assert_eq!(finish.diagnostic_level, 1);
            if at_bias {
                assert!(result.is_none());
                assert_eq!(finish.point, ModelFinishPoint::OperatingPoint);
            } else {
                let results = result.unwrap();
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].frequency, 10.0);
                assert!(results[0].voltages.is_empty() && results[0].currents.is_empty());
                assert_eq!(
                    finish.point,
                    ModelFinishPoint::Frequency { frequency: 10.0 }
                );
            }
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn sp_task_free_parallel_points_share_the_bias_and_global_final_boundary() {
    let model = write_model(
        "sp_passive_parallel",
        r#"module sp_passive_parallel(p,n);
inout p,n; electrical p,n;
real conductance;
analog begin
    @(initial_step) conductance=0.02;
    @(final_step) conductance=0.04;
    I(p,n)<+conductance*V(p,n);
    I(p,n)<+white_noise(4*1.380649e-23*$temperature*conductance, "thermal");
end
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* Parallel SP with independent columns\nP1 p1 0 AC 7 portnum=1 z0=50\nP2 p2 0 AC 9 portnum=2 z0=50\nIextra p1 p2 AC 11\nVextra aux 0 AC 100\nRextra aux 0 1k\nC1 p1 p2 1u\nX1 p1 p2 sp_passive_parallel\n.va \"{}\" sp_passive_parallel\n.end\n",
        deck_path(&model),
    )).unwrap();
    let frequencies = (1..=25)
        .map(|index| f64::from(index) * 10.0)
        .collect::<Vec<_>>();
    for do_noise in [false, true] {
        let runs = [1, 4].map(|workers| {
            let mut config = SimulationConfig::default();
            config.resource_limits.max_parallel_workers = workers;
            Engine::new(config)
                .run_sp_over_grid_with_abort(&netlist, &frequencies, do_noise, &NoAbort)
                .unwrap()
        });
        for (index, &frequency) in frequencies.iter().enumerate() {
            let conductance = if index + 1 == frequencies.len() {
                0.04
            } else {
                0.02
            };
            let impedance = rspice_core::Complex64::new(1.0, 0.0)
                / rspice_core::Complex64::new(
                    conductance,
                    2.0 * std::f64::consts::PI * frequency * 1e-6,
                );
            let denominator = rspice_core::Complex64::new(100.0, 0.0) + impedance;
            let reflection = impedance / denominator;
            let transmission = rspice_core::Complex64::new(100.0, 0.0) / denominator;
            for run in &runs {
                assert_eq!(run.scattering.data.len(), frequencies.len());
                let matrix = &run.scattering.data[index];
                assert_eq!(matrix.frequency, frequency);
                for (actual, expected) in [
                    (matrix.s11(), reflection),
                    (matrix.s22(), reflection),
                    (matrix.s12(), transmission),
                    (matrix.s21(), transmission),
                ] {
                    assert!(
                        (actual - expected).norm() < 1e-10,
                        "point={index}: {actual} vs {expected}"
                    );
                }
                if do_noise {
                    let noise = &run.port_noise.as_ref().unwrap().points;
                    assert_eq!(noise.len(), frequencies.len());
                    let expected = 4.0 * 1.380649e-23 * 300.15 * conductance;
                    assert!(
                        (noise[index].current_correlation[0][0].re / expected - 1.0).abs() < 1e-10
                    );
                }
            }
            for row in 1..=2 {
                for column in 1..=2 {
                    assert_eq!(
                        runs[0].scattering.data[index].get(row, column),
                        runs[1].scattering.data[index].get(row, column)
                    );
                }
            }
            if do_noise {
                assert_eq!(
                    runs[0].port_noise.as_ref().unwrap().points[index].current_correlation,
                    runs[1].port_noise.as_ref().unwrap().points[index].current_correlation
                );
            }
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn ac_portless_final_step_errors_are_not_skipped_without_system_tasks() {
    let model = write_model(
        "portless_final_loop",
        r#"
module portless_final_loop;
integer count;
analog @(final_step("ac")) while ($temperature > 0) count=count+1;
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* Portless final step\nX1 portless_final_loop\n.va \"{}\" portless_final_loop\n.end\n",
        deck_path(&model),
    ))
    .unwrap();
    let error = Engine::default()
        .run_ac(&netlist, &[10.0, 20.0])
        .expect_err("the final event must run even without a task instruction");
    let rspice_core::SimulationError::Circuit(message) = &error else {
        panic!("the loop must fail during the analysis, not compilation: {error}");
    };
    assert!(
        message.contains("frequency candidate failed")
            || message.contains("equilibrium analysis-step setup failed"),
        "{error}"
    );
    assert!(
        error.to_string().to_ascii_lowercase().contains("loop"),
        "{error}"
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn frequency_data_finish_retains_completed_rows_for_ac_and_noise() {
    let model = write_model(
        "data_finish",
        r#"
module data_finish(p,n);
inout p,n; electrical p,n;
parameter integer stop_now=0, at_bias=0;
analog begin
    if (stop_now < 0) $finish(99);
    if (stop_now && (at_bias || !analysis("static"))) $finish(1);
    I(p,n)<+1e-3*V(p,n);
    I(p,n)<+white_noise(1e-18,"data_noise");
end
endmodule"#,
    );
    for at_bias in [false, true] {
        let netlist = Netlist::parse(&format!(
            "* Ordered table finish\n.param stop_now=0\nVREF in 0 DC 0 AC 1\nR1 in out 1k\nX1 out 0 data_finish stop_now={{stop_now}} at_bias={}\n.va \"{}\" data_finish\n.data finish_rows FREQ stop_now\n10 0\n20 1\n30 -1\n.enddata\n.end\n",
            usize::from(at_bias), deck_path(&model),
        )).unwrap();
        let engine = Engine::default();
        let ac = engine
            .run_with_outcome(&NoAbort, |engine, signal| {
                engine.run_ac_data_with_abort(&netlist, "finish_rows", signal)
            })
            .unwrap();
        let noise = engine
            .run_with_outcome(&NoAbort, |engine, signal| {
                engine.run_noise_data_named_with_input_source_and_abort(
                    &netlist,
                    "out",
                    None,
                    "VREF",
                    "finish_rows",
                    300.15,
                    signal,
                )
            })
            .unwrap();
        let SimulationOutcome::Finished {
            result: Some((ac_rows, ac_results)),
            finish,
        } = ac
        else {
            panic!("AC table lost its prefix");
        };
        let SimulationOutcome::Finished {
            result: Some((noise_rows, noise_results)),
            finish: noise_finish,
        } = noise
        else {
            panic!("noise table lost its prefix");
        };
        assert_eq!(finish, noise_finish);
        let count = if at_bias { 1 } else { 2 };
        assert_eq!(ac_rows.len(), count);
        assert_eq!(noise_rows.len(), count);
        assert_eq!(ac_results.len(), count);
        assert_eq!(noise_results.len(), count);
        assert_eq!(
            finish.point,
            if at_bias {
                ModelFinishPoint::OperatingPoint
            } else {
                ModelFinishPoint::Frequency { frequency: 20.0 }
            }
        );
        for (index, (ac, noise)) in ac_results.iter().zip(&noise_results).enumerate() {
            assert_eq!(ac.frequency, [10.0, 20.0][index]);
            assert_eq!(noise.frequency, ac.frequency);
        }
        assert_eq!(
            engine.run_ac_data(&netlist, "finish_rows").unwrap().1.len(),
            count
        );
        assert_eq!(
            engine
                .run_noise_data_named_with_input_source(
                    &netlist,
                    "out",
                    None,
                    "VREF",
                    "finish_rows",
                    300.15
                )
                .unwrap()
                .1
                .len(),
            count
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn ac_finish_in_the_operating_point_prevents_frequency_results() {
    let model = write_model(
        "finish_operating_point",
        r#"module finish_operating_point(p,n);
inout p,n; electrical p,n;
analog begin
    @(initial_step("ac")) $finish(1);
    I(p,n)<+1e-3*V(p,n);
end
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* Finish in the AC bias point\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nX1 out 0 finish_operating_point\n.va \"{}\" finish_operating_point\n.end\n",
        deck_path(&model),
    )).unwrap();
    let outcome = Engine::default()
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_ac_with_abort(&netlist, &[1e3, 1e4], signal)
        })
        .expect("accepted finish is normal completion");
    let SimulationOutcome::Finished { result, finish } = outcome else {
        panic!("AC must honor the accepted operating-point finish");
    };
    assert!(result.is_none(), "no frequency point was solved");
    assert_eq!(finish.point, ModelFinishPoint::OperatingPoint);
    assert_eq!(finish.diagnostic_level, 1);
    let _ = std::fs::remove_file(model);
}

#[test]
fn ac_finish_retains_the_point_and_runs_final_step_in_sweep_order() {
    let model = write_model(
        "finish_frequency",
        r#"module finish_frequency(p,n);
inout p,n; electrical p,n;
parameter integer finish_early=1;
real count;
analog begin
    @(initial_step("ac")) count=1;
    @(final_step("ac")) begin count=count+1; $finish(2); end
    if (finish_early && analysis("ac") && !analysis("static") && count==1) $finish(1);
    I(p,n)<+count*1e-3*V(p,n);
end
endmodule"#,
    );
    let frequencies = [
        10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1e3, 2e3, 5e3, 1e4, 1e5, 5e5, 1e6,
    ];
    for finish_early in [true, false] {
        let netlist = Netlist::parse(&format!(
            "* Finish at an accepted AC point\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nX1 out 0 finish_frequency finish_early={}\n.va \"{}\" finish_frequency\n.end\n",
            usize::from(finish_early), deck_path(&model),
        )).unwrap();
        for workers in [1, 4] {
            let mut config = SimulationConfig::default();
            config.resource_limits.max_parallel_workers = workers;
            let outcome = Engine::new(config)
                .run_with_outcome(&NoAbort, |engine, signal| {
                    engine.run_ac_with_abort(&netlist, &frequencies, signal)
                })
                .expect("accepted finish preserves the solved frequency point");
            let SimulationOutcome::Finished {
                result: Some(results),
                finish,
            } = outcome
            else {
                panic!("AC must publish the accepted frequency finish");
            };
            let expected_count = if finish_early { 1 } else { frequencies.len() };
            assert_eq!(results.len(), expected_count);
            assert_eq!(finish.diagnostic_level, if finish_early { 1 } else { 2 });
            assert_eq!(
                finish.point,
                ModelFinishPoint::Frequency {
                    frequency: frequencies[expected_count - 1]
                }
            );
            for (index, result) in results.iter().enumerate() {
                assert_eq!(result.frequency, frequencies[index]);
                let output = result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                let expected = if index + 1 == expected_count {
                    1.0 / 3.0
                } else {
                    0.5
                };
                assert!((result.voltages[output].re - expected).abs() < 1e-12);
                assert!(result.voltages[output].im.abs() < 1e-12);
            }
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn ac_equilibrium_reports_static_analysis_to_initial_step() {
    let model = write_model(
        "static_operating_point",
        r#"module static_operating_point(p,n);
inout p,n; electrical p,n;
real conductance;
analog begin
    @(initial_step("ac")) conductance=analysis("static") ? 1e-3 : 3e-3;
    I(p,n)<+(conductance+(analysis("static") ? 2e-3 : 0))*V(p,n);
end
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* AC operating-point analysis query\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nX1 out 0 static_operating_point\n.va \"{}\" static_operating_point\n.end\n",
        deck_path(&model),
    )).unwrap();
    let results = Engine::default().run_ac(&netlist, &[1e3, 1e4]).unwrap();
    assert_eq!(results.len(), 2);
    for result in results {
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let voltage = result.voltages[output];
        assert!(
            (voltage.re - 0.5).abs() < 1e-12 && voltage.im.abs() < 1e-12,
            "expected a 1 mS conductance initialized during the static operating point, got {voltage}"
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn ac_initial_step_initializes_every_frequency_from_the_committed_operating_point() {
    let model = write_model(
        "initial_step",
        r#"
`include "disciplines.vams"
module va_ac_initial_step(p, n);
    inout p, n;
    electrical p, n;
    real conductance;
    analog begin
        @(initial_step("ac")) conductance = conductance + 1.0e-3;
        I(p, n) <+ conductance * V(p, n);
    end
endmodule
"#,
    );

    let deck = format!(
        "* AC initial-step lifecycle\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 1k\n\
         X1 out 0 va_ac_initial_step\n\
         .va \"{}\" va_ac_initial_step\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse AC lifecycle deck");
    let frequencies = [
        10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1.0e3, 1.0e4, 1.0e5, 1.0e6,
    ];
    let results = Engine::default()
        .run_ac(&netlist, &frequencies)
        .expect("AC lifecycle run");
    assert_eq!(results.len(), frequencies.len());
    let output = results[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node");
    for result in &results {
        let voltage = result.voltages[output];
        assert!(
            (voltage.re - 0.5).abs() < 1.0e-12 && voltage.im.abs() < 1.0e-12,
            "AC initial_step state was not retained at {} Hz: {voltage}",
            result.frequency
        );
    }

    let _ = std::fs::remove_file(model);
}

#[test]
fn ac_final_step_marks_only_the_global_final_frequency() {
    let model = write_model(
        "final_step",
        r#"
`include "disciplines.vams"
module va_ac_final_step(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("ac")) count = count + 1.0;
        @(final_step("ac")) count = count + 1.0;
        I(p, n) <+ count * 1.0e-3 * V(p, n);
        I(p, n) <+ ddt(count * 1.0e-9 * V(p, n));
    end
endmodule
"#,
    );
    let deck = format!(
        "* AC final-step lifecycle\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 1k\n\
         X1 out 0 va_ac_final_step\n\
         .va \"{}\" va_ac_final_step\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse AC final-step deck");
    let frequencies = [
        10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1.0e3, 2.0e3, 5.0e3, 1.0e4, 1.0e5, 5.0e5, 1.0e6,
    ];

    let run = |workers| {
        let mut config = SimulationConfig::default();
        config.resource_limits.max_parallel_workers = workers;
        Engine::new(config)
            .run_ac(&netlist, &frequencies)
            .expect("AC final-step sweep runs")
    };
    let serial = run(1);
    let chunk_parallel = run(4);
    assert_eq!(serial.len(), frequencies.len());
    assert_eq!(chunk_parallel.len(), frequencies.len());
    let output = serial[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node");

    for (index, (serial_point, parallel_point)) in serial.iter().zip(&chunk_parallel).enumerate() {
        let serial_voltage = serial_point.voltages[output];
        let parallel_voltage = parallel_point.voltages[output];
        assert_eq!(serial_voltage.re.to_bits(), parallel_voltage.re.to_bits());
        assert_eq!(serial_voltage.im.to_bits(), parallel_voltage.im.to_bits());

        let count = if index + 1 == frequencies.len() {
            2.0
        } else {
            1.0
        };
        let real = 1.0 + count;
        let imag = 2.0 * std::f64::consts::PI * frequencies[index] * count * 1.0e-6;
        let denominator = real * real + imag * imag;
        let expected_re = real / denominator;
        let expected_im = -imag / denominator;
        assert!(
            (serial_voltage.re - expected_re).abs() <= 1.0e-11 * expected_re.abs().max(1.0)
                && (serial_voltage.im - expected_im).abs() <= 1.0e-11 * expected_im.abs().max(1.0),
            "AC final_step count at index {index} was not exact: actual={serial_voltage}, expected={expected_re}+j{expected_im}"
        );
    }

    let _ = std::fs::remove_file(model);
}

/// RC lowpass with a Verilog-A capacitor: H(s) = 1/(1 + sRC).
/// R = 1k, C = 1u -> fc = 1/(2 pi RC) ~= 159.155 Hz.
#[test]
fn veriloga_capacitor_rc_lowpass_ac() {
    let model = write_model(
        "cap",
        r#"
`include "disciplines.vams"
module va_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-6 from (0:inf);
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#,
    );

    let deck = format!(
        "* veriloga RC lowpass\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 1k\n\
         XC1 out 0 va_cap c=1u\n\
         .va \"{}\" va_cap\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let fc = 1.0 / (2.0 * std::f64::consts::PI * 1e3 * 1e-6);
    let freqs = [fc / 100.0, fc, 100.0 * fc];
    let results = Engine::default().run_ac(&netlist, &freqs).expect("ac runs");

    let out_idx = results[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out node");

    // Far below fc: |H| ~ 1, phase ~ 0
    let h_low = results[0].voltages[out_idx];
    assert!(
        (h_low.norm() - 1.0).abs() < 2e-4,
        "low-frequency magnitude, got {}",
        h_low.norm()
    );

    // At fc: |H| = 1/sqrt(2), phase = -45 degrees
    let h_fc = results[1].voltages[out_idx];
    assert!(
        (h_fc.norm() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6,
        "corner magnitude, got {}",
        h_fc.norm()
    );
    let phase_deg = h_fc.arg().to_degrees();
    assert!(
        (phase_deg + 45.0).abs() < 1e-3,
        "corner phase, got {phase_deg}"
    );

    // Far above fc: |H| ~ fc/f
    let h_high = results[2].voltages[out_idx];
    assert!(
        (h_high.norm() - 0.01).abs() < 1e-4,
        "high-frequency rolloff, got {}",
        h_high.norm()
    );

    let _ = std::fs::remove_file(model);
}

/// RL highpass with a Verilog-A inductor written in flux form
/// (V <+ ddt(L*I)): H(s) = sL/R / (1 + sL/R).
/// R = 100, L = 10m -> fc = R/(2 pi L) ~= 1591.55 Hz.
#[test]
fn veriloga_inductor_rl_highpass_ac() {
    let model = write_model(
        "ind",
        r#"
`include "disciplines.vams"
module va_ind(p, n);
    inout p, n;
    electrical p, n;
    parameter real l = 10e-3 from (0:inf);
    analog V(p, n) <+ ddt(l * I(p, n));
endmodule
"#,
    );

    let deck = format!(
        "* veriloga RL highpass\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 100\n\
         XL1 out 0 va_ind l=10m\n\
         .va \"{}\" va_ind\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let fc = 100.0 / (2.0 * std::f64::consts::PI * 10e-3);
    let freqs = [fc / 100.0, fc, 100.0 * fc];
    let results = Engine::default().run_ac(&netlist, &freqs).expect("ac runs");

    let out_idx = results[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out node");

    // Far below fc the inductor shorts the output: |H| ~ f/fc
    let h_low = results[0].voltages[out_idx];
    assert!(
        (h_low.norm() - 0.01).abs() < 1e-4,
        "low-frequency shorting, got {}",
        h_low.norm()
    );

    // At fc: |H| = 1/sqrt(2), phase = +45 degrees
    let h_fc = results[1].voltages[out_idx];
    assert!(
        (h_fc.norm() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6,
        "corner magnitude, got {}",
        h_fc.norm()
    );
    let phase_deg = h_fc.arg().to_degrees();
    assert!(
        (phase_deg - 45.0).abs() < 1e-3,
        "corner phase, got {phase_deg}"
    );

    // Far above fc the inductor is open: |H| ~ 1
    let h_high = results[2].voltages[out_idx];
    assert!(
        (h_high.norm() - 1.0).abs() < 2e-4,
        "high-frequency passband, got {}",
        h_high.norm()
    );

    let _ = std::fs::remove_file(model);
}

#[test]
fn veriloga_ac_runtime_stamp_errors_are_simulation_errors_not_panics() {
    let model = write_model(
        "ac_oob",
        r#"
`include "disciplines.vams"
module va_ac_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = analysis("ac") ? 5 : 1;
        w[i] = 1.0e-6;
        I(p, n) <+ w[i] * V(p, n);
    end
endmodule
"#,
    );

    let deck = format!(
        "* veriloga AC runtime diagnostic\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in 0 1k\n\
         XBAD in 0 va_ac_oob\n\
         .va \"{}\" va_ac_oob\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = std::panic::catch_unwind(|| Engine::default().run_ac(&netlist, &[1.0e3]));

    let _ = std::fs::remove_file(model);

    let result = result.expect("Verilog-A AC runtime stamp errors must not panic");
    let err = result.expect_err("AC runtime stamp error must be reported to the caller");
    let text = err.to_string();
    assert!(
        text.contains("Verilog-A") && (text.contains("Array index 5") || text.contains("[1:4]")),
        "diagnostic should identify the Verilog-A AC array bounds error, got: {text}"
    );
}

/// Bias-dependent charge: Q = 0.5*k*V^2 gives C(V) = kV, so the corner
/// frequency moves with the DC operating point.
#[test]
fn veriloga_nonlinear_charge_linearizes_at_bias() {
    let model = write_model(
        "varactor",
        r#"
`include "disciplines.vams"
module va_varactor(p, n);
    inout p, n;
    electrical p, n;
    parameter real k = 1e-6 from (0:inf);
    real q;
    analog begin
        q = 0.5 * k * V(p, n) * V(p, n);
        I(p, n) <+ ddt(q);
    end
endmodule
"#,
    );

    // DC bias 2 V through a large feed resistor sets C = k*Vdc = 2 uF;
    // the AC divider R=1k, C=2u has fc = 1/(2 pi R C) ~= 79.58 Hz
    let deck = format!(
        "* veriloga varactor bias-dependent corner\n\
         V1 in 0 DC 2 AC 1\n\
         R1 in out 1k\n\
         XQ1 out 0 va_varactor k=1u\n\
         .va \"{}\" va_varactor\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    // DC operating point: no DC path to ground through the cap, so
    // V(out) settles to 2 V and C(Vdc) = 2 uF
    let c_bias = 1e-6 * 2.0;
    let fc = 1.0 / (2.0 * std::f64::consts::PI * 1e3 * c_bias);
    let results = Engine::default().run_ac(&netlist, &[fc]).expect("ac runs");

    let out_idx = results[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out node");
    let h = results[0].voltages[out_idx];
    assert!(
        (h.norm() - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-3,
        "bias-dependent corner: |H(fc)| = {} (C must linearize at Vdc=2)",
        h.norm()
    );

    let _ = std::fs::remove_file(model);
}

#[test]
fn nested_ddx_laplace_model_solves_through_the_engine() {
    for (operator, numerator, denominator) in [
        ("laplace_nd", "'{1.0,0.5}", "'{1.0,0.25}"),
        ("laplace_zp", "'{-2.0,0.0}", "'{-4.0,0.0}"),
        ("laplace_zd", "'{-2.0,0.0}", "'{1.0,0.25}"),
        ("laplace_np", "'{1.0,0.5}", "'{-4.0,0.0}"),
    ] {
        let model = write_model(
            &format!("curvature_{operator}"),
            &format!(
                "module curvature(p,n); inout p,n; electrical p,n; real y;
                analog begin
                    y=ddx(ddx({operator}(V(p,n)*V(p,n)*V(p,n),{numerator},{denominator}),V(p)),V(p));
                    I(p,n)<+y;
                end endmodule"
            ),
        );
        let netlist = Netlist::parse(&format!(
            "* Laplace curvature\nI1 0 out DC 6 AC 1\nX1 out 0 curvature\n.va \"{}\" curvature\n.end\n",
            deck_path(&model)
        )).unwrap();
        let engine = Engine::default();
        // At DC, I = d^2(V^3)/dV^2 = 6V and dI/dV = 6.
        let bias = engine.run_dc_op(&netlist).expect("curvature bias solves");
        assert!(
            (bias.try_voltage_named("out").unwrap() - 1.0).abs() < 1e-10,
            "{operator}"
        );
        let points = engine
            .run_ac(&netlist, &[0.0])
            .expect("curvature AC solves");
        let out = points[0]
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let voltage = points[0].voltages[out];
        assert!(
            (voltage.re - 1.0 / 6.0).abs() < 1e-10 && voltage.im.abs() < 1e-10,
            "{operator}: {voltage}"
        );
        std::fs::remove_file(model).expect("remove curvature model");
    }
}
