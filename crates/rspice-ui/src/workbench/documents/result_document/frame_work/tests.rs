//! The idle-frame gate: steady-state Results frames must be O(visible).
//!
//! Every surface here is driven through its real `show` on a deliberately
//! large retained dataset, for ten consecutive frames with identical input
//! and no state change in between. What the gate asserts is not a duration —
//! wall clock on CI is noise — but a count: how many times those ten frames
//! walked the complete dataset. The answer has to be zero. The first frame is
//! allowed to build whatever it needs; frames two onward are the product's
//! steady state, and a reader who is not touching anything is entitled to
//! have nothing recomputed.

use std::collections::BTreeMap;

use super::{DatasetWalk, FrameSampleRead, WorkCounts};
// Through the module under measurement rather than around it: this drives the
// Results workspace's own surfaces with the session type that workspace
// already holds, and does not reach for the session aggregate on its own.
use super::super::{
    AppState, ResultViewer, bode, events, eye, fft, manifest, noise_contrib, nyquist, op_inspector,
    optimization, sensitivity, soa, table, waves,
};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType, DcOpResult,
    DigitalEventPointEvidence, DigitalEventTraceEvidence, OperatingPointValue, SimulationRun,
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence, WaveformData,
};

/// Retained samples per transient waveform.
const TRANSIENT_SAMPLES: usize = 250_000;
/// Transient node-voltage waveforms; the product of the two is the headline
/// dataset size.
const TRANSIENT_TRACES: usize = 4;
/// Transient branch-current waveforms.
///
/// Volts and amps do not share a Y scale, so a strip carrying both is a
/// two-pane strip — the ordinary shape of a probed circuit, and the shape a
/// single-slot per-strip memo cannot serve.
const TRANSIENT_CURRENT_TRACES: usize = 2;
/// Retained points in the frequency response.
const AC_POINTS: usize = 40_000;
/// Retained points in each ordinary-noise spectrum.
const NOISE_POINTS: usize = 20_000;
/// Retained noise spectra: input-referred, output-referred, and per-device
/// contributors.
const NOISE_TRACES: usize = 6;
/// Committed XSPICE event nodes on the transient.
const EVENT_NODES: usize = 8;
/// Committed events on each event node.
const EVENTS_PER_NODE: usize = 2_000;
/// Time-domain samples the fixture's spectrum is transformed from.
const FFT_SAMPLES: usize = 16_384;
/// Retained points on the loop-gain locus.
const NYQUIST_POINTS: usize = 20_000;
/// Retained points on each distortion curve.
const DISTORTION_POINTS: usize = 40_000;
/// Retained coefficients in the harmonic-balance spectrum.
const HARMONIC_COEFFICIENTS: usize = 8_000;
/// Retained offsets in the phase-noise spectrum.
const PHASE_NOISE_POINTS: usize = 20_000;
/// Retained frequencies in the S-parameter sweep.
const SPARAMETER_POINTS: usize = 20_000;
/// Retained node-voltage rows on the operating point.
const OP_NODES: usize = 20_000;
/// Retained per-device rows on the operating point.
const OP_DEVICES: usize = 20_000;
/// Evaluated SOA rules, each with its own retained stress history.
const SOA_RULES: usize = 400;
/// Retained samples in each SOA stress history.
const SOA_SAMPLES: usize = 2_500;
/// Folded eye acquisitions.
const EYE_TRACES: usize = 500;
/// Samples per folded acquisition.
const EYE_SAMPLES: usize = 400;

/// Ranked sensitivity parameters.
const SENSITIVITY_PARAMETERS: usize = 5_000;
/// Retained optimizer candidates.
const OPTIMIZATION_ITERATIONS: usize = 5_000;

/// Frames measured after the first. Ten is enough that a per-frame walk
/// cannot hide behind an every-other-frame schedule.
const IDLE_FRAMES: usize = 10;

fn transient_analysis() -> AnalysisResult {
    let x: Vec<f64> = (0..TRANSIENT_SAMPLES)
        .map(|index| index as f64 * 1.0e-9)
        .collect();
    let mut waveforms: Vec<WaveformData> = (0..TRANSIENT_TRACES)
        .map(|trace| {
            let y: Vec<f64> = (0..TRANSIENT_SAMPLES)
                .map(|index| ((index + trace) as f64 * 1.0e-3).sin())
                .collect();
            WaveformData::new(format!("V(n{trace})"), x.clone(), y, "#00aaff")
        })
        .collect();
    waveforms.extend((0..TRANSIENT_CURRENT_TRACES).map(|trace| {
        let y: Vec<f64> = (0..TRANSIENT_SAMPLES)
            .map(|index| ((index + trace) as f64 * 1.0e-3).cos() * 1.0e-3)
            .collect();
        WaveformData::new(format!("I(R{trace})"), x.clone(), y, "#ffbd2e")
    }));

    // The event history is retained evidence of its own — the sparse
    // schedule the event solver accepted, not the analog timestep grid — so
    // the EVENTS sheet reads it off the same transient the waves do.
    let digital_traces = (0..EVENT_NODES)
        .map(|node| DigitalEventTraceEvidence {
            node_name: format!("d{node}"),
            points: (0..EVENTS_PER_NODE)
                .map(|index| DigitalEventPointEvidence {
                    time_s: index as f64 * 1.0e-7,
                    value_code: ((index + node) % 2) as u8,
                })
                .collect(),
        })
        .collect();

    AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_waveforms(waveforms)
        .with_result_payload(AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces: Vec::new(),
        })
}

/// A decade-swept frequency response with a magnitude/phase pair.
///
/// The Bode sheet, its stability card and the Bode tab's own availability
/// gate all read this; the gate reads it for every analysis in the run, on
/// every frame, whichever sheet is open.
fn ac_analysis() -> AnalysisResult {
    let frequency: Vec<f64> = (0..AC_POINTS)
        .map(|index| 10f64.powf(index as f64 * 7.0 / AC_POINTS as f64))
        .collect();
    // A two-pole loop: 60 dB at DC, poles at 10 Hz and 100 kHz.
    let magnitude: Vec<f64> = frequency
        .iter()
        .map(|f| 1_000.0 / ((1.0 + (f / 10.0).powi(2)).sqrt() * (1.0 + (f / 1.0e5).powi(2)).sqrt()))
        .collect();
    let phase: Vec<f64> = frequency
        .iter()
        .map(|f| -(f / 10.0).atan().to_degrees() - (f / 1.0e5).atan().to_degrees())
        .collect();
    AnalysisResult::new(6, AnalysisType::Ac, "AC").with_waveforms(vec![
        WaveformData::new("|V(out)|", frequency.clone(), magnitude, "#00aaff"),
        WaveformData::new("phase(V(out))", frequency, phase, "#ffbd2e"),
    ])
}

/// A distortion result: plain frequency curves with no magnitude/phase
/// pairing to resolve.
///
/// The Bode tab offers this family too, and the only way to know whether one
/// of its curves is drawable is to check every retained sample. Placed before
/// the AC result so the tab strip's `any` reaches it first — which is exactly
/// how the reader would meet it, and exactly what a gate that walks would
/// charge them for.
fn distortion_analysis() -> AnalysisResult {
    let frequency: Vec<f64> = (0..DISTORTION_POINTS)
        .map(|index| 10f64.powf(1.0 + index as f64 * 6.0 / DISTORTION_POINTS as f64))
        .collect();
    let curve = |scale: f64| -> Vec<f64> {
        frequency
            .iter()
            .map(|f| scale / (1.0 + (f / 1.0e4).powi(2)))
            .collect()
    };
    let (second_harmonic, third_harmonic) = (curve(1.0e-3), curve(1.0e-4));
    AnalysisResult::new(8, AnalysisType::Disto, "DISTO").with_waveforms(vec![
        WaveformData::new("HD2", frequency.clone(), second_harmonic, "#00aaff"),
        WaveformData::new("HD3", frequency, third_harmonic, "#ffbd2e"),
    ])
}

/// A discrete complex coefficient spectrum.
fn harmonic_balance_analysis() -> AnalysisResult {
    let frequency: Vec<f64> = (0..HARMONIC_COEFFICIENTS)
        .map(|index| (index + 1) as f64 * 1.0e6)
        .collect();
    let real: Vec<f64> = (0..HARMONIC_COEFFICIENTS)
        .map(|index| 1.0 / (index as f64 + 1.0))
        .collect();
    let imag: Vec<f64> = (0..HARMONIC_COEFFICIENTS)
        .map(|index| -0.5 / (index as f64 + 1.0))
        .collect();
    let magnitude: Vec<f64> = real
        .iter()
        .zip(imag.iter())
        .map(|(re, im)| re.hypot(*im))
        .collect();
    AnalysisResult::new(9, AnalysisType::HarmonicBalance, "HB").with_waveforms(vec![
        WaveformData::new("|V(out)|", frequency, magnitude, "#00aaff")
            .with_complex_components("V(out)", real, imag),
    ])
}

/// An explicitly-labelled phase-noise spectrum with its retained carrier.
fn phase_noise_analysis() -> AnalysisResult {
    let offset: Vec<f64> = (0..PHASE_NOISE_POINTS)
        .map(|index| 10f64.powf(1.0 + index as f64 * 6.0 / PHASE_NOISE_POINTS as f64))
        .collect();
    let level: Vec<f64> = offset
        .iter()
        .map(|f| -40.0 - 30.0 * f.log10() - 20.0)
        .collect();
    AnalysisResult::new(10, AnalysisType::Pnoise, "PNOISE")
        .with_family_metadata(AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: crate::state::PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(2.4e9),
        })
        .with_waveforms(vec![WaveformData::new(
            "phase_noise",
            offset,
            level,
            "#00aaff",
        )])
}

/// A two-port S-parameter sweep with its per-port reference impedances.
fn sparameter_analysis() -> AnalysisResult {
    let frequency: Vec<f64> = (0..SPARAMETER_POINTS)
        .map(|index| 10f64.powf(6.0 + index as f64 * 4.0 / SPARAMETER_POINTS as f64))
        .collect();
    let waveforms = ["S11", "S21"]
        .into_iter()
        .enumerate()
        .map(|(port, name)| {
            let real: Vec<f64> = frequency
                .iter()
                .map(|f| (f.log10() + port as f64).cos() * 0.4)
                .collect();
            let imag: Vec<f64> = frequency
                .iter()
                .map(|f| (f.log10() + port as f64).sin() * 0.4)
                .collect();
            let magnitude: Vec<f64> = real
                .iter()
                .zip(imag.iter())
                .map(|(re, im)| re.hypot(*im))
                .collect();
            WaveformData::new(name, frequency.clone(), magnitude, "#00aaff")
                .with_complex_components(name, real, imag)
        })
        .collect();
    AnalysisResult::new(11, AnalysisType::SParameter, "SP")
        .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm: vec![50.0, 50.0],
        })
        .with_waveforms(waveforms)
}

/// Input- and output-referred noise densities plus per-device contributors.
///
/// Every one of them is verified sample by sample before the noise sheet or
/// the noise tab will speak for it.
fn noise_analysis() -> AnalysisResult {
    let frequency: Vec<f64> = (0..NOISE_POINTS)
        .map(|index| 10f64.powf(index as f64 * 6.0 / NOISE_POINTS as f64))
        .collect();
    let density = |scale: f64| -> Vec<f64> {
        frequency
            .iter()
            .map(|f| scale * (1.0e-18 + 1.0e-15 / f))
            .collect()
    };
    let mut waveforms = vec![
        WaveformData::new(
            "inoise_spectrum",
            frequency.clone(),
            density(0.5),
            "#00aaff",
        ),
        WaveformData::new(
            "onoise_spectrum",
            frequency.clone(),
            density(1.0),
            "#ffbd2e",
        ),
    ];
    waveforms.extend((0..NOISE_TRACES - 2).map(|device| {
        WaveformData::new(
            format!("noise(r{device})"),
            frequency.clone(),
            density(0.1 * (device as f64 + 1.0)),
            "#7f8c98",
        )
    }));
    AnalysisResult::new(7, AnalysisType::Noise, "NOISE").with_waveforms(waveforms)
}

fn operating_point_analysis() -> AnalysisResult {
    let dc = DcOpResult {
        node_voltages: (0..OP_NODES)
            .map(|index| OperatingPointValue {
                name: format!("V(x{}.n{index})", index % 32),
                value: index as f64 * 1.0e-3,
                unit: "V".to_owned(),
            })
            .collect(),
        ..DcOpResult::default()
    };
    let report = rspice_core::circuit::DeviceOpReport {
        entries: (0..OP_DEVICES)
            .map(|index| rspice_core::circuit::DeviceOpEntry {
                name: format!("x{}.m{index}", index % 32),
                device_kind: "MOSFET",
                region: Some("saturation"),
                params: vec![
                    ("id", index as f64 * 1.0e-6),
                    ("vgs", 1.2),
                    ("vds", 1.8),
                    ("gm", 1.0e-3),
                ],
            })
            .collect(),
    };
    AnalysisResult::new(2, AnalysisType::DcOp, "OP")
        .with_dc_op(dc)
        .with_device_op(report)
}

fn soa_analysis() -> AnalysisResult {
    let time: Vec<f64> = (0..SOA_SAMPLES)
        .map(|index| index as f64 * 1.0e-12)
        .collect();
    let mut waveforms = Vec::with_capacity(SOA_RULES);
    let mut evaluations = Vec::with_capacity(SOA_RULES);
    let mut violations = Vec::with_capacity(SOA_RULES);
    for rule in 0..SOA_RULES {
        let device_id = format!("M{rule:04}");
        // The worst sample is the last one, and every other sample sits below
        // it, which is what the stress-history verifier checks for.
        let y: Vec<f64> = (0..SOA_SAMPLES)
            .map(|index| 3.0 * (index as f64 + 1.0) / SOA_SAMPLES as f64)
            .collect();
        let worst_actual_value = y[SOA_SAMPLES - 1];
        let worst_time_s = time[SOA_SAMPLES - 1];
        waveforms.push(WaveformData::new(
            crate::services::safety::soa_stress_waveform_name(
                &device_id,
                crate::services::safety::SoAParameter::Vds,
            ),
            time.clone(),
            y,
            "#00aaff",
        ));
        evaluations.push(SoaEvaluationEvidence {
            device_id: device_id.clone(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            worst_actual_value,
            worst_time_s,
            sample_count: SOA_SAMPLES as u64,
            unit: "V".to_owned(),
            description: "Maximum drain-source voltage".to_owned(),
            verdict: SoaRuleVerdictEvidence::Warning,
        });
        violations.push(SoaViolationEvidence {
            device_id,
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            actual_value: worst_actual_value,
            time_s: worst_time_s,
            severity: SoaViolationSeverityEvidence::Warning,
        });
    }
    AnalysisResult::new(3, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time })
        .with_waveforms(waveforms)
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations,
            violations,
        })
}

fn sensitivity_analysis() -> AnalysisResult {
    AnalysisResult::new(4, AnalysisType::Sensitivity, "SENS").with_result_payload(
        AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: crate::state::SensitivityResultMode::Dc,
            rows: (0..SENSITIVITY_PARAMETERS)
                .map(|index| crate::state::SensitivityResultRow {
                    parameter: format!("p{index:06}"),
                    raw: index as f64,
                    normalized: (index as f64).sin(),
                })
                .collect(),
        },
    )
}

fn optimization_analysis() -> AnalysisResult {
    let axis: Vec<f64> = (0..OPTIMIZATION_ITERATIONS)
        .map(|index| index as f64)
        .collect();
    let cost: Vec<f64> = (0..OPTIMIZATION_ITERATIONS)
        .map(|index| 1.0 / (index as f64 + 1.0))
        .collect();
    let gain: Vec<f64> = (0..OPTIMIZATION_ITERATIONS)
        .map(|index| index as f64 * 0.5)
        .collect();
    AnalysisResult::new(5, AnalysisType::Optimization, "OPT")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            iterations: axis.clone(),
            best_cost: cost[OPTIMIZATION_ITERATIONS - 1],
            best_variables: std::collections::BTreeMap::from([(
                "GAIN".to_owned(),
                gain[OPTIMIZATION_ITERATIONS - 1],
            )]),
            converged: true,
        })
        .with_waveforms(vec![
            WaveformData::new("OPT_COST", axis.clone(), cost, "#0af"),
            WaveformData::new("OPT_GAIN", axis, gain, "#fa0"),
        ])
}

/// One run carrying every dataset the measured surfaces read.
fn large_state() -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(transient_analysis());
    run.add_analysis(operating_point_analysis());
    run.add_analysis(soa_analysis());
    run.add_analysis(sensitivity_analysis());
    run.add_analysis(optimization_analysis());
    run.add_analysis(distortion_analysis());
    run.add_analysis(ac_analysis());
    run.add_analysis(noise_analysis());
    run.add_analysis(harmonic_balance_analysis());
    run.add_analysis(phase_noise_analysis());
    run.add_analysis(sparameter_analysis());

    state.simulation.runs = vec![run];
    assert!(state.simulation.select_run(0));

    let mut eye = crate::analysis::eye_diagram::EyeData::new(1.0e-9, 2);
    for trace in 0..EYE_TRACES {
        let time: Vec<f64> = (0..EYE_SAMPLES)
            .map(|index| index as f64 * 2.0 / EYE_SAMPLES as f64)
            .collect();
        let amplitude: Vec<f64> = (0..EYE_SAMPLES)
            .map(|index| ((index + trace) as f64 * 0.05).sin())
            .collect();
        eye.add_trace(crate::analysis::eye_diagram::EyeTrace::new(time, amplitude));
    }
    state.analysis.eye_diagram_state.load_data(eye);

    let samples: Vec<f64> = (0..FFT_SAMPLES)
        .map(|index| {
            let t = index as f64 / FFT_SAMPLES as f64;
            (2.0 * std::f64::consts::PI * 64.0 * t).sin()
                + 0.1 * (2.0 * std::f64::consts::PI * 192.0 * t).sin()
        })
        .collect();
    state.analysis.fft_state.load_data(
        crate::analysis::fft::FftData::from_time_domain(
            "V(n0)",
            &samples,
            FFT_SAMPLES as f64,
            crate::analysis::fft::WindowFunction::Hanning,
        )
        .expect("finite qualified FFT frame fixture"),
    );

    let (frequency, real, imaginary) = (0..NYQUIST_POINTS).fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut frequency, mut real, mut imaginary), index| {
            let f = 10f64.powf(index as f64 * 6.0 / NYQUIST_POINTS as f64);
            let (magnitude, phase) = (
                1_000.0 / (1.0 + (f / 10.0).powi(2)).sqrt(),
                -(f / 10.0).atan(),
            );
            frequency.push(f);
            real.push(magnitude * phase.cos());
            imaginary.push(magnitude * phase.sin());
            (frequency, real, imaginary)
        },
    );
    state
        .analysis
        .nyquist_state
        .load_data(crate::analysis::nyquist::NyquistData::from_arrays(
            "Loop gain",
            &frequency,
            &real,
            &imaginary,
        ));

    // The reader has the envelope on: the drawn panes build it, and so does
    // the sheet bar's own gate before them.
    state.ui.results.show_family_envelope = true;
    state
}

/// Select the analysis a surface speaks for, by retained analysis type.
fn select_analysis(state: &mut AppState, analysis_type: AnalysisType) {
    let index = state
        .simulation
        .active_run()
        .expect("the fixture selects a run")
        .analyses
        .iter()
        .position(|analysis| analysis.analysis_type == analysis_type)
        .expect("the fixture retains this analysis type");
    assert!(state.simulation.select_analysis(index));
}

/// Run one frame of `surface` against an unchanged state and identical input.
fn frame(
    ctx: &egui::Context,
    state: &mut AppState,
    surface: &mut dyn FnMut(&mut egui::Ui, &mut AppState),
) {
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_680.0, 1_020.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| surface(ui, state));
        },
    );
}

/// Work counted across [`IDLE_FRAMES`] frames that follow a first frame.
fn steady_state_work(
    state: &mut AppState,
    mut surface: impl FnMut(&mut egui::Ui, &mut AppState),
) -> WorkCounts {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    // The first frame is allowed to build the memos the rest read.
    frame(&ctx, state, &mut surface);
    let baseline = WorkCounts::reset();
    for _ in 0..IDLE_FRAMES {
        frame(&ctx, state, &mut surface);
    }
    baseline.since()
}

/// Every measured surface, named for the failure message.
fn surfaces() -> Vec<(&'static str, ResultViewer)> {
    vec![
        ("Manifest", ResultViewer::Manifest),
        ("OP inspector", ResultViewer::Op),
        ("Table", ResultViewer::Table),
        ("SOA", ResultViewer::Soa),
        ("Waves", ResultViewer::Waves),
        ("Eye", ResultViewer::Eye),
        ("Sensitivity", ResultViewer::Contribution),
        ("Optimization", ResultViewer::Optimization),
        ("Bode", ResultViewer::Bode),
        ("Noise", ResultViewer::NoiseContrib),
        ("Nyquist", ResultViewer::Nyquist),
        ("FFT", ResultViewer::Fft),
        ("Events", ResultViewer::Events),
    ]
}

fn show_surface(viewer: ResultViewer, ui: &mut egui::Ui, state: &mut AppState) {
    state.ui.results.viewer = viewer;
    // The document bar states the sheet's purpose on every frame, including
    // whether the retained evidence validated, and the tab strip decides on
    // every frame which sheets to offer. Both are part of the frame, and both
    // used to be where the dataset walks were.
    let _ = super::super::sheet_purpose(state);
    for candidate in ResultViewer::PRIMARY
        .into_iter()
        .chain(ResultViewer::DATASET_NATIVE)
    {
        let _ = super::super::viewer_availability(state, candidate);
    }
    // The wave instrument's sheet bar decides on every frame whether to offer
    // the family-envelope control, and deciding means building the envelope
    // for the active pane. It runs before the panes do, and asks the same
    // memo they are about to ask with a different key.
    if matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    ) {
        let tokens = super::super::Tokens::get(ui.ctx());
        let _ = waves::family_envelope_available(state, &tokens);
    }
    match viewer {
        ResultViewer::Manifest => manifest::show(ui, state),
        ResultViewer::Op => op_inspector::show(ui, state),
        ResultViewer::Table => table::show(ui, state),
        ResultViewer::Soa => soa::show(ui, state),
        ResultViewer::Waves => waves::show(ui, state),
        ResultViewer::Eye => eye::show(ui, state),
        ResultViewer::Contribution => {
            sensitivity::show(ui, state);
            sensitivity::right_panel(ui, state);
        }
        ResultViewer::Optimization => {
            optimization::show(ui, state);
            optimization::right_panel(ui, state);
        }
        ResultViewer::Bode => {
            waves::show_bode(ui, state);
            bode::right_panel(ui, state);
        }
        ResultViewer::NoiseContrib => {
            waves::show_noise(ui, state);
            noise_contrib::right_panel(ui, state);
        }
        ResultViewer::Nyquist => {
            nyquist::show(ui, state);
            nyquist::right_panel(ui, state);
        }
        ResultViewer::Fft => {
            fft::show(ui, state);
            fft::right_panel(ui, state);
        }
        ResultViewer::Events => {
            events::show(ui, state);
            events::right_panel(ui, state);
        }
        other => panic!("{other:?} is not a measured surface"),
    }
}

fn state_for(viewer: ResultViewer) -> AppState {
    let mut state = large_state();
    match viewer {
        ResultViewer::Op => select_analysis(&mut state, AnalysisType::DcOp),
        ResultViewer::Soa => select_analysis(&mut state, AnalysisType::Soa),
        ResultViewer::Contribution => select_analysis(&mut state, AnalysisType::Sensitivity),
        ResultViewer::Optimization => select_analysis(&mut state, AnalysisType::Optimization),
        ResultViewer::Bode | ResultViewer::Nyquist => {
            select_analysis(&mut state, AnalysisType::Ac);
        }
        ResultViewer::NoiseContrib => select_analysis(&mut state, AnalysisType::Noise),
        _ => select_analysis(&mut state, AnalysisType::Transient),
    }
    if viewer == ResultViewer::Waves {
        add_transient_expression(&mut state);
    }
    state
}

/// Put one visible expression on the transient strip.
///
/// An expression is a second retained series the strip has to fit its axis to
/// on every frame, and resolving a strip happens twice per frame — so it is
/// where a per-frame full scan hides most easily. Without one on the fixture,
/// every gate here is measuring a Waves sheet the product rarely shows.
fn add_transient_expression(state: &mut AppState) {
    let run = state
        .simulation
        .active_run()
        .expect("the fixture selects a run");
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == AnalysisType::Transient)
        .expect("the fixture retains a transient analysis");
    let key = super::super::AnalysisPresentationKey::new(run.dataset_id, analysis);
    state.ui.results.analysis_exprs.insert(
        key,
        vec![super::super::ExprTrace {
            text: "V(n0) * 2".to_owned(),
            visible: true,
        }],
    );
}

/// The permanent gate.
///
/// Ten consecutive frames with identical input and no state change between
/// them, on a dataset large enough that any whole-dataset work would be
/// unmissable. A surface that recomputes anything dataset-sized on an idle
/// frame fails here, named, with the class of work it repeated and how many
/// times it repeated it.
#[test]
fn idle_frames_do_no_whole_dataset_work_on_any_results_surface() {
    let mut offenders = BTreeMap::new();
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let work = steady_state_work(&mut state, |ui, state| show_surface(viewer, ui, state));
        if work.total() > 0 {
            offenders.insert(name, work.nonzero());
        }
    }
    assert!(
        offenders.is_empty(),
        "these Results surfaces repeated whole-dataset work across {IDLE_FRAMES} idle frames \
         (surface -> [(work, times)]):\n{offenders:#?}"
    );
}

/// The measurement is only meaningful if the counters can see the work
/// at all. Frame one is the proof: it has no memos to read, so it must count
/// the walks the steady state then refuses to repeat.
#[test]
fn the_first_frame_counts_the_work_the_idle_frames_must_not_repeat() {
    let mut seen = BTreeMap::new();
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let baseline = WorkCounts::reset();
        frame(&ctx, &mut state, &mut |ui, state| {
            show_surface(viewer, ui, state);
        });
        seen.insert(name, baseline.since().total());
    }
    for (name, count) in &seen {
        assert!(
            *count > 0,
            "the first {name} frame counted no whole-dataset work at all, so the idle-frame \
             gate for it is vacuous: {seen:#?}"
        );
    }
}

/// A memo that can serve stale data is worse than the cost it saved. Every
/// surface must show the new truth after the dataset behind it changes.
#[test]
fn a_new_data_version_rebuilds_every_memo() {
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        frame(&ctx, &mut state, &mut |ui, state| {
            show_surface(viewer, ui, state);
        });

        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);
        let baseline = WorkCounts::reset();
        frame(&ctx, &mut state, &mut |ui, state| {
            show_surface(viewer, ui, state);
        });
        assert!(
            baseline.since().total() > 0,
            "{name} served its memo across a data-version bump without recomputing anything"
        );
    }
}

/// Around-cursor mode maps two cursors and a window centre onto the retained
/// grid on every frame. On a million-sample transient that was three scans of
/// the whole grid per frame, for a cursor the reader had already placed.
#[test]
fn the_table_maps_a_cursor_without_scanning_the_grid() {
    let mut state = state_for(ResultViewer::Table);
    state.ui.results.table.around_cursor = true;
    state.ui.results.cursors.place(1.0e-4);
    state.ui.results.cursors.place(2.0e-4);
    state.ui.results.cursor_strip = Some(0);

    let work = steady_state_work(&mut state, |ui, state| {
        show_surface(ResultViewer::Table, ui, state);
    });
    assert_eq!(
        work.get(DatasetWalk::TableCursorScan),
        0,
        "the table scanned the retained grid to place a cursor that had not moved"
    );
}

/// The retained-evidence verdict is a memo, not a snapshot.
///
/// Corrupting the evidence and declaring a new dataset generation has to
/// close every gate that reads it — the sheet bar's caution and the SOA
/// availability gate both come from the one memo, so both must move.
#[test]
fn corrupted_evidence_closes_the_gates_that_read_the_memo() {
    let mut state = state_for(ResultViewer::Soa);
    state.ui.results.viewer = ResultViewer::Soa;
    assert!(soa::active_payload_is_valid(&state));
    let purpose = super::super::sheet_purpose(&state);
    assert!(
        !purpose.contains("failed validation"),
        "the fixture starts out valid, but the bar says: {purpose}"
    );

    // A retained waveform with more coordinates than values is exactly what
    // `validate_retained_evidence` exists to refuse.
    let analysis = state.simulation.runs[0]
        .analyses
        .iter_mut()
        .find(|analysis| analysis.analysis_type == AnalysisType::Soa)
        .expect("the fixture retains an SOA analysis");
    let mut shortened = analysis.waveforms[0].y.as_ref().clone();
    shortened.pop();
    analysis.waveforms[0].y = std::sync::Arc::new(shortened);
    state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

    assert!(
        !soa::active_payload_is_valid(&state),
        "the SOA gate served a verdict from the previous dataset generation"
    );
    let purpose = super::super::sheet_purpose(&state);
    assert!(
        purpose.contains("the retained evidence failed validation"),
        "the sheet bar kept the old verdict: {purpose}"
    );
}

/// Report the counts rather than asserting on them, for tuning the fixes.
/// Not a gate: `--ignored` keeps it out of the ordinary run.
///
/// ```text
/// cargo test -p rspice-ui --lib report_idle_frame_work -- --ignored --nocapture
/// ```
// The crate denies `print_stdout` because the desktop build detaches from its
// console and the browser build has no stderr, so a stray diagnostic reaches
// nobody. This is the same exception `services::license` already carries: an
// `#[ignore]`d operator entry point whose entire output *is* the printed
// table, read through `--nocapture`. Routing it through `log` would put the
// table behind a logger no test installs. The allow is per-function so the
// rule keeps holding for every other test in the crate.
#[test]
#[allow(clippy::print_stdout)]
#[ignore = "measurement harness"]
fn report_idle_frame_work() {
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let first = WorkCounts::reset();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        frame(&ctx, &mut state, &mut |ui, state| {
            show_surface(viewer, ui, state);
        });
        let first = first.since();
        let steady = steady_state_work(&mut state, |ui, state| show_surface(viewer, ui, state));
        println!(
            "{name:14} first frame {:5} | {IDLE_FRAMES} idle frames {:6} | idle detail {:?}",
            first.total(),
            steady.total(),
            steady.nonzero()
        );
    }
}

/// Retained samples one surface may read across [`IDLE_FRAMES`] steady frames.
///
/// What is drawn sets it. The only surface that legitimately reads the dataset
/// on a steady frame is the overview lane under a Waves strip, which draws a
/// fixed 160 points (plus the partial last stride) per strip; this fixture has
/// two time-domain strips, so 3 280 over ten frames. Four thousand leaves room
/// for a third strip and is still three orders of magnitude under the
/// 5 050 000 an unmemoized lane read.
const STEADY_SAMPLE_CEILING_PER_SURFACE: u64 = 4_000;

/// The second gate: a steady frame may read the dataset, but only as much of
/// it as it draws.
///
/// The walk gate above cannot see this. Nothing here walks a *complete*
/// dataset — an overview lane reads one trace of several, an axis fit reads
/// one series — so it counted zero while the Waves sheet spent most of its
/// frame reading a quarter of a million samples to place a hundred and sixty
/// points. What makes the difference visible is counting the samples.
#[test]
fn no_steady_frame_reads_more_of_the_dataset_than_it_draws() {
    let mut reads = BTreeMap::new();
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let work = steady_state_work(&mut state, |ui, state| show_surface(viewer, ui, state));
        if work.total_samples() > STEADY_SAMPLE_CEILING_PER_SURFACE {
            reads.insert(name, (work.total_samples(), work.nonzero_samples()));
        }
    }
    assert!(
        reads.is_empty(),
        "over {IDLE_FRAMES} idle frames these surfaces read more than \
         {STEADY_SAMPLE_CEILING_PER_SURFACE} retained samples, which is more than they draw \
         (surface -> (samples, [(class, samples)])):\n{reads:#?}"
    );
}

/// The sample counter is only worth having if it can see the work. The first
/// frame proves it: with no memo to read, it must count the reads the steady
/// frames then refuse to repeat at that scale.
#[test]
fn the_first_frame_counts_the_samples_the_steady_frames_must_not_reread() {
    let mut state = state_for(ResultViewer::Waves);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let baseline = WorkCounts::reset();
    frame(&ctx, &mut state, &mut |ui, state| {
        show_surface(ResultViewer::Waves, ui, state);
    });
    let first = baseline.since();
    assert!(
        first.samples(FrameSampleRead::TraceExtremes) >= TRANSIENT_SAMPLES as u64,
        "the first Waves frame read {} samples for trace extremes, fewer than the \
         {TRANSIENT_SAMPLES} one retained waveform holds, so the sample gate is vacuous",
        first.samples(FrameSampleRead::TraceExtremes)
    );
}

/// The S-parameter gate is scoped to the *active* analysis, so the surfaces
/// above never reach its walk — and an instrument nothing exercises is an
/// instrument that passes forever.
///
/// This drives the gate the way the tab strip does, with the S-parameter
/// result selected: once to build, then not again.
#[test]
fn the_smith_gate_verifies_its_traces_once_per_dataset_generation() {
    let mut state = large_state();
    select_analysis(&mut state, AnalysisType::SParameter);

    let first = WorkCounts::reset();
    let offered = super::super::viewer_availability(&state, ResultViewer::Smith).available;
    let first = first.since();
    assert!(
        offered,
        "the fixture must retain an S-parameter result the Smith sheet accepts"
    );
    assert!(
        first.get(DatasetWalk::SParameterTraceScan) > 0,
        "the gate answered without verifying a single retained trace, so this measures nothing"
    );

    let baseline = WorkCounts::reset();
    for _ in 0..IDLE_FRAMES {
        assert!(super::super::viewer_availability(&state, ResultViewer::Smith).available);
    }
    assert_eq!(
        baseline.since().get(DatasetWalk::SParameterTraceScan),
        0,
        "the Smith tab re-verified every retained complex coefficient on an idle frame"
    );
}

#[test]
fn the_counter_reports_exactly_the_walks_that_were_noted() {
    let baseline = WorkCounts::reset();
    super::note(DatasetWalk::WaveXRange);
    super::note(DatasetWalk::WaveXRange);
    super::note(DatasetWalk::EyeRaster);

    let delta = baseline.since();
    assert_eq!(delta.get(DatasetWalk::WaveXRange), 2);
    assert_eq!(delta.get(DatasetWalk::EyeRaster), 1);
    assert_eq!(delta.get(DatasetWalk::OpPlan), 0);
    assert_eq!(delta.total(), 3);
    assert_eq!(delta.total_samples(), 0);
    assert_eq!(
        delta.nonzero(),
        vec![(DatasetWalk::WaveXRange, 2), (DatasetWalk::EyeRaster, 1)]
    );
}

#[test]
fn the_counter_reports_exactly_the_samples_that_were_read() {
    let baseline = WorkCounts::reset();
    super::note_samples(FrameSampleRead::StripOverview, 160);
    super::note_samples(FrameSampleRead::StripOverview, 1);
    super::note_samples(FrameSampleRead::TraceExtremes, 250_000);

    let delta = baseline.since();
    assert_eq!(delta.samples(FrameSampleRead::StripOverview), 161);
    assert_eq!(delta.samples(FrameSampleRead::TraceExtremes), 250_000);
    assert_eq!(delta.total_samples(), 250_161);
    assert_eq!(delta.total(), 0);
    assert_eq!(
        delta.nonzero_samples(),
        vec![
            (FrameSampleRead::StripOverview, 161),
            (FrameSampleRead::TraceExtremes, 250_000)
        ]
    );
}

#[test]
fn a_reset_snapshot_measures_only_work_that_follows_it() {
    super::note(DatasetWalk::OpPlan);
    super::note_samples(FrameSampleRead::StripOverview, 999);
    let baseline = WorkCounts::reset();
    assert_eq!(baseline.total(), 0);
    assert_eq!(baseline.total_samples(), 0);
    assert_eq!(baseline.since().total(), 0);
    assert_eq!(baseline.since().total_samples(), 0);
}
