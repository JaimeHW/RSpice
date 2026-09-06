//! The evidence each Results sheet is shown, and the derivations that
//! produce a cache a sheet reads.
//!
//! Split out of `availability_tests` when that file crossed the line budget:
//! the seam is the one the tests already used — this module answers "what
//! does a sheet get", and its parent answers "what does the sheet do with
//! it". Nothing here asserts anything.

use super::*;
use crate::state::{SensitivityResultMode, SensitivityResultRow};
use crate::workbench::app_state::SpecializedViewerCacheProvenance;

pub(super) fn soa_analysis() -> AnalysisResult {
    use crate::state::{
        AnalysisResultFamilyMetadata, SoaEvaluationEvidence, SoaParameterEvidence,
        SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
    };
    let time = vec![0.0, 1.0e-9];
    AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time: time.clone() })
        .with_waveforms(vec![
            WaveformData::new(
                "SOA_VIOLATION_COUNT",
                time.clone(),
                vec![0.0, 0.0],
                "#ffbd2e",
            ),
            // The stress history the producer now retains, named exactly as
            // the sheet addresses it.
            WaveformData::new(
                crate::services::safety::soa_stress_waveform_name(
                    "M1",
                    crate::services::safety::SoAParameter::Vds,
                ),
                time,
                vec![2.0, 3.0],
                "#00aaff",
            ),
        ])
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations: vec![SoaEvaluationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.0,
                worst_time_s: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                // 3.0 V against a 3.3 V limit is inside the warning band,
                // which the payload validator derives rather than trusts.
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            // A non-passing rule must carry the exact event at its worst
            // point; the validator refuses a verdict with nothing behind it.
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.0,
                time_s: 1.0e-9,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        })
}

pub(super) fn reliability_analysis() -> AnalysisResult {
    use crate::state::{
        AnalysisResultFamilyMetadata, ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence,
        ReliabilityShiftEvidence, ReliabilityStressEvidence,
    };
    AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
        .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
            years: vec![1.0, 5.0],
        })
        .with_result_payload(AnalysisResultPayload::Reliability {
            devices: vec![ReliabilityDeviceEvidence {
                device_id: "M1".to_owned(),
                stress: ReliabilityStressEvidence {
                    average_gate_stress_v: 1.1,
                    average_drain_stress_v: 1.8,
                    average_temperature_k: 358.0,
                    duration_s: 1.0e-6,
                },
                checkpoints: vec![
                    ReliabilityCheckpointEvidence {
                        years: 1.0,
                        shift: ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 1.0e-3,
                            mobility_shift: -1.0e-4,
                            drain_source_resistance_shift: 2.0e-3,
                        },
                    },
                    ReliabilityCheckpointEvidence {
                        years: 5.0,
                        shift: ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 4.0e-3,
                            mobility_shift: -4.0e-4,
                            drain_source_resistance_shift: 8.0e-3,
                        },
                    },
                ],
            }],
        })
}

pub(super) fn optimization_analysis() -> AnalysisResult {
    use crate::state::AnalysisResultFamilyMetadata;
    AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            iterations: vec![0.0, 1.0],
            best_cost: 0.25,
            best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
            converged: true,
        })
        // The names the optimization runner emits: one OPT_COST trace over
        // the iteration axis, plus OPT_<variable> per design variable.
        .with_waveforms(vec![
            WaveformData::new("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25], "#ffbd2e"),
            WaveformData::new("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6], "#00aaff"),
        ])
}

/// A committed event history with all three kinds of row the Events sheet
/// draws: a scalar conductor, a real-valued event node, and two declared
/// buses over their own member traces.
///
/// The buses are here rather than in a second fixture because every gate that
/// reads this one is a gate about the sheet — the fit gate below measures the
/// radix bar and the per-row disclosures, which a history declaring no bus
/// does not paint at all.
pub(super) fn events_analysis() -> AnalysisResult {
    use crate::state::{
        DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
        DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
    };
    let trace = |name: &str, points: &[(f64, u8)]| DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: points
            .iter()
            .map(|(time_s, value_code)| DigitalEventPointEvidence {
                time_s: *time_s,
                value_code: *value_code,
            })
            .collect(),
    };
    let bus = |name: &str| DigitalBusEvidence {
        name: name.to_owned(),
        msb: 1,
        lsb: 0,
        members: vec![format!("{name}[1]"), format!("{name}[0]")],
        source: DigitalBusSourceEvidence::Import,
    };
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
        AnalysisResultPayload::TransientEvents {
            digital_traces: vec![
                trace("clk", &[(0.0, 0), (5.0e-10, 1), (1.0e-9, 12)]),
                trace("addr[1]", &[(0.0, 0), (1.0e-9, 1)]),
                trace("addr[0]", &[(0.0, 0), (5.0e-10, 1), (1.0e-9, 0)]),
                // One bit the run never states before its first change, so a
                // word the sheet cannot spell as a number is on screen too.
                trace("data[1]", &[(5.0e-10, 2)]),
                trace("data[0]", &[(0.0, 1), (1.0e-9, 12)]),
            ],
            real_traces: vec![RealEventTraceEvidence {
                node_name: "level".to_owned(),
                points: vec![RealEventPointEvidence {
                    time_s: 2.5e-10,
                    value: 0.75,
                }],
            }],
            digital_buses: vec![bus("addr"), bus("data")],
        },
    )
}

/// A dataset each sheet reports itself able to draw. Built from retained
/// evidence and, where a sheet reads a derived cache, through the same
/// derivation the controller runs after a completed analysis — never by
/// asserting a capability the data does not support.
pub(super) fn app_showing(viewer: ResultViewer) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    // The production viewer classifies retained data outside an open
    // project as `no-project`; this fixture represents an open Results
    // workspace and must establish that authority explicitly.
    app.state.project_lifecycle.project_open = true;
    let analysis = match viewer {
        ResultViewer::Waves
        | ResultViewer::Table
        | ResultViewer::Manifest
        | ResultViewer::Eye
        | ResultViewer::Fft => transient_analysis(),
        ResultViewer::DcSweep => AnalysisResult::new(1, AnalysisType::DcSweep, "DC")
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                vec![0.0, 0.5, 1.0, 1.5],
                vec![0.0, 0.4, 0.9, 1.2],
                "#00aaff",
            )]),
        ResultViewer::Bode | ResultViewer::Nyquist => ac_analysis(),
        ResultViewer::Smith | ResultViewer::Polar => sparameter_analysis(),
        ResultViewer::Scatter | ResultViewer::BoxViolin => {
            // The distribution sheets need a bounded measurement to normalize
            // against, and the box's default grouping draws nothing without
            // one.
            app.state.workspace.specs.push(crate::state::SpecEntry {
                measurement: "gain_dc".to_owned(),
                expression: String::new(),
                min: Some(39.5),
                max: None,
                unit: "dB".to_owned(),
                scope: crate::state::SpecPointScope::AllPoints,
            });
            monte_carlo_population_analysis()
        }
        ResultViewer::HarmonicBalance => {
            AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0e9, 2.0e9],
                    vec![1.0, 0.1, 0.01],
                    "#00aaff",
                )
                .with_complex_components(
                    "V(out)",
                    vec![1.0, 0.1, 0.01],
                    vec![0.0, 0.02, 0.001],
                ),
            ])
        }
        ResultViewer::PhaseNoise => AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE")
            .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::PeriodicNoise {
                output_quantity: crate::state::PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                carrier_frequency_hz: Some(2.4e9),
            })
            .with_waveforms(vec![WaveformData::new(
                "phase_noise",
                vec![1.0e3, 1.0e4, 1.0e5],
                vec![-80.0, -100.0, -120.0],
                "#00aaff",
            )]),
        ResultViewer::Hist => monte_carlo_analysis(),
        ResultViewer::Op => AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_device_op(
            rspice_core::circuit::DeviceOpReport {
                entries: vec![rspice_core::circuit::DeviceOpEntry {
                    name: "M1".to_owned(),
                    device_kind: "MOSFET",
                    region: Some("saturation"),
                    params: Vec::new(),
                }],
            },
        ),
        ResultViewer::NoiseContrib => AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
            .with_waveforms(vec![WaveformData::new(
                "inoise",
                vec![1.0, 10.0, 100.0],
                vec![1.0e-9, 2.0e-9, 4.0e-9],
                "#00aaff",
            )]),
        ResultViewer::Contribution => AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows: vec![
                    SensitivityResultRow {
                        parameter: "r1".to_owned(),
                        raw: 0.25,
                        normalized: 0.5,
                    },
                    SensitivityResultRow {
                        parameter: "r2".to_owned(),
                        raw: -0.125,
                        normalized: -0.25,
                    },
                ],
            }),
        ResultViewer::TransferFunction => transfer_function_analysis(),
        ResultViewer::Specs => {
            app.state.workspace.specs.push(crate::state::SpecEntry {
                measurement: "V(out)".to_owned(),
                expression: String::new(),
                min: Some(-2.0),
                max: Some(5.0),
                unit: "V".to_owned(),
                scope: crate::state::SpecPointScope::AllPoints,
            });
            transient_analysis()
        }
        ResultViewer::PoleZero => AnalysisResult::new(1, AnalysisType::PoleZero, "PZ")
            .with_result_payload(AnalysisResultPayload::PoleZero {
                poles: vec![crate::state::ComplexResultValue {
                    real: -1.0e3,
                    imaginary: 2.0e3,
                }],
                zeros: vec![crate::state::ComplexResultValue {
                    real: -5.0e3,
                    imaginary: 0.0,
                }],
                pole_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
                zero_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
                gain: Some(10.0),
            }),
        ResultViewer::Events => events_analysis(),
        ResultViewer::Soa => soa_analysis(),
        ResultViewer::Reliability => reliability_analysis(),
        ResultViewer::Optimization => optimization_analysis(),
    };

    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    run.lifecycle = crate::state::SimulationRunLifecycle::Completed;
    app.state.simulation.runs = vec![run];
    assert!(app.state.simulation.select_run(0));
    assert!(app.state.simulation.select_analysis(0));

    // Three sheets read a cache the controller fills after a run rather
    // than the retained analysis itself. Derive it exactly as the
    // controller's post-processing does, so the fixture cannot claim a
    // capability the data would not actually produce.
    match viewer {
        ResultViewer::Nyquist => derive_nyquist(&mut app),
        ResultViewer::Smith => {
            assert!(smith::synchronize_active_analysis(&mut app.state));
        }
        ResultViewer::Hist => derive_histogram(&mut app),
        _ => {}
    }

    if viewer == ResultViewer::Soa {
        // The stress trace is the sheet's only plot; a card that never
        // opens is a card this test never covers.
        app.state.ui.results.soa_stress_trace_open = true;
        app.state.ui.results.selected_soa_rule = Some(SoaRuleSelection {
            analysis: active_analysis_key(&app.state),
            device_id: "M1".to_owned(),
            parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
        });
    }
    app
}

pub(super) fn transient_analysis() -> AnalysisResult {
    let time: Vec<f64> = (0..64).map(|index| index as f64 * 1.0e-9).collect();
    let values: Vec<f64> = time
        .iter()
        .map(|t| (t * 1.0e9 * std::f64::consts::TAU / 8.0).sin())
        .collect();
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(vec![WaveformData::new("V(out)", time, values, "#00aaff")])
}

/// A resonant second-order response, swept densely enough to decimate.
///
/// Four points was not a sweep: it sat under every reduction threshold, so
/// the Bode, Nyquist and Smith fixtures exercised only the raw-stroke
/// path and never the one a real AC run takes. The resonance also makes
/// the complex locus genuinely non-monotone in its real part, so the
/// precondition assertion in `decimate_minmax` fires if either locus
/// sheet ever loses its `parametric` declaration.
pub(super) const AC_FIXTURE_POINTS: usize = 8_192;

pub(super) fn ac_analysis() -> AnalysisResult {
    let (mut frequency, mut real, mut imaginary) = (
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
    );
    let (mut magnitude, mut phase) = (
        Vec::with_capacity(AC_FIXTURE_POINTS),
        Vec::with_capacity(AC_FIXTURE_POINTS),
    );
    // H(jω) = 1 / (1 − (ω/ω0)² + j·ω/(Q·ω0)), log-swept through resonance.
    let (natural, quality) = (1.0e5_f64, 6.0_f64);
    for index in 0..AC_FIXTURE_POINTS {
        let decade = 3.0 + 4.0 * index as f64 / (AC_FIXTURE_POINTS - 1) as f64;
        let f = 10.0_f64.powf(decade);
        let ratio = f / natural;
        let (denominator_real, denominator_imaginary) = (1.0 - ratio * ratio, ratio / quality);
        let square =
            denominator_real * denominator_real + denominator_imaginary * denominator_imaginary;
        let (re, im) = (denominator_real / square, -denominator_imaginary / square);
        frequency.push(f);
        real.push(re);
        imaginary.push(im);
        magnitude.push(re.hypot(im));
        phase.push(im.atan2(re).to_degrees());
    }
    AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
        WaveformData::new("|V(out)|", frequency.clone(), magnitude, "#00aaff")
            .with_complex_components("V(out)", real, imaginary),
        WaveformData::new("phase(V(out))", frequency, phase, "#ffaa00"),
    ])
}

pub(super) fn sparameter_analysis() -> AnalysisResult {
    let mut analysis = ac_analysis();
    analysis.analysis_type = AnalysisType::SParameter;
    analysis.label = "SP".to_owned();
    analysis.waveforms.truncate(1);
    analysis.waveforms[0].name = "|S11|".to_owned();
    analysis.waveforms[0]
        .complex
        .as_mut()
        .expect("AC fixture complex components")
        .source_name = "S11".to_owned();
    analysis.with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm: vec![75.0, 100.0],
    })
}

/// A Monte Carlo that retained both halves of its evidence: the sampled
/// variable and what every trial measured, indexed the same way.
///
/// The histogram fixture beside this one deliberately retains neither — it
/// exercises the path where a distribution comes from the derived histogram
/// cache — so the correlation and distribution sheets need their own.
pub(super) fn monte_carlo_population_analysis() -> AnalysisResult {
    use crate::state::{
        FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
        MonteCarloVariableMetadata,
    };

    const TRIALS: usize = 101;
    let samples: Vec<f64> = (0..TRIALS)
        .map(|index| (index as f64 - 50.0) / 50.0)
        .collect();
    let members = samples
        .iter()
        .enumerate()
        .map(|(index, sample)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::MonteCarloTrial {
                    index,
                    seed: 0x73a4 + index as u64,
                },
                vec![
                    FamilyMeasurementEvidence {
                        name: "gain_dc".to_owned(),
                        value: Some(40.0 + 2.0 * sample),
                        passed: true,
                        error: None,
                    },
                    FamilyMeasurementEvidence {
                        name: "vos".to_owned(),
                        value: Some(60.0e-6 * sample),
                        passed: true,
                        error: None,
                    },
                ],
            )
        })
        .collect::<Vec<_>>();
    let mean = samples.iter().sum::<f64>() / TRIALS as f64;
    let variance = samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (TRIALS - 1) as f64;
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
        crate::state::AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 0x73a4,
            runs_requested: TRIALS,
            runs_completed: TRIALS,
            failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableMetadata {
                name: "XBRIDGE.dR".to_owned(),
                mean,
                std_dev: variance.sqrt(),
                min: -1.0,
                max: 1.0,
                samples,
            }],
            member_measurements: members,
        },
    )
}

pub(super) fn monte_carlo_analysis() -> AnalysisResult {
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_waveforms(vec![WaveformData::new(
        "V(out)",
        vec![0.0, 1.0, 2.0, 3.0],
        vec![0.95, 1.02, 0.98, 1.05],
        "#00aaff",
    )])
}

pub(super) fn transfer_function_analysis() -> AnalysisResult {
    use crate::state::{
        TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
        TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
    };
    AnalysisResult::new(1, AnalysisType::Tf, "XF").with_result_payload(
        AnalysisResultPayload::TransferFunction {
            input_source: "vin".to_owned(),
            output_expression: "V(out)".to_owned(),
            input_quantity: TransferFunctionQuantityEvidence::Voltage,
            output_quantity: TransferFunctionQuantityEvidence::Voltage,
            input_unit: "V".to_owned(),
            output_unit: "V".to_owned(),
            normalization: TransferFunctionNormalizationEvidence::None,
            accuracy: TransferFunctionAccuracyEvidence::Balanced,
            gain: Some(TransferFunctionScalarEvidence::Finite(-12.5)),
            input_resistance: Some(TransferFunctionScalarEvidence::Finite(1.0e6)),
            output_resistance: Some(TransferFunctionScalarEvidence::Finite(50.0)),
            nominal_input: None,
            nominal_output: None,
        },
    )
}

pub(super) fn in_flight_cache_provenance(app: &RSpiceApp) -> SpecializedViewerCacheProvenance {
    let run = app
        .state
        .simulation
        .active_run()
        .expect("the fixture retained one run");
    let dataset_id = run.dataset_id;
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .expect("the fixture selected one analysis");
    SpecializedViewerCacheProvenance::for_analysis(dataset_id, analysis)
}

pub(super) fn ac_complex_trace(app: &RSpiceApp) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .expect("the fixture selected one analysis");
    let waveform = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.complex.is_some())
        .expect("the AC fixture retains one complex trace");
    let complex = waveform
        .complex
        .as_ref()
        .expect("the trace was selected for its complex components");
    (
        waveform.x.to_vec(),
        complex.real.to_vec(),
        complex.imag.to_vec(),
    )
}

pub(super) fn derive_nyquist(app: &mut RSpiceApp) {
    let (frequency, real, imaginary) = ac_complex_trace(app);
    let provenance = in_flight_cache_provenance(app);
    app.state.analysis.nyquist_state.load_data(
        crate::analysis::nyquist::data::NyquistData::from_arrays(
            "V(out)", &frequency, &real, &imaginary,
        ),
    );
    app.state
        .bind_specialized_viewer_cache(ActiveViewer::Nyquist, provenance);
}

pub(super) fn derive_histogram(app: &mut RSpiceApp) {
    use crate::analysis::histogram::data::{Histogram, HistogramBin};

    let counts = [2_usize, 5, 9, 4];
    let edges = [0.90_f64, 0.95, 1.00, 1.05, 1.10];
    let bins: Vec<HistogramBin> = counts
        .iter()
        .enumerate()
        .map(|(index, count)| HistogramBin {
            lower: edges[index],
            upper: edges[index + 1],
            count: *count,
            weight: *count as f64,
        })
        .collect();
    let total_count: usize = counts.iter().sum();
    let provenance = in_flight_cache_provenance(app);
    app.state
        .analysis
        .histogram_state
        .load_histogram(Histogram {
            name: "V(out)".to_owned(),
            bins,
            total_count,
            total_weight: total_count as f64,
            underflow: 0,
            overflow: 0,
            data_min: edges[0],
            data_max: edges[edges.len() - 1],
        });
    app.state
        .bind_specialized_viewer_cache(ActiveViewer::Histogram, provenance);
}
