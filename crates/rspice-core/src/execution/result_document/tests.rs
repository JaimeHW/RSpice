//! Behavior tests for the shared typed result document.

use num_complex::Complex64;

use super::*;
use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};
use crate::analysis::ac::AcResult;
use crate::analysis::distortion::{
    DistortionAnalysisResult, DistortionPointResult, DistortionProduct, DistortionProductResult,
};
use crate::analysis::fourier::{FourierResult, HarmonicComponent};
use crate::analysis::harmonic_balance::{
    HbConfig, HbReactiveKind, HbReactiveSpectrum, HbResult, SpectralBranchCurrent, SpectralVoltage,
};
use crate::analysis::monte_carlo::{MonteCarloResult, VariableStatistics};
use crate::analysis::noise::{
    NoiseContribution, NoiseResult, NoiseSourceIdentity, NoiseSourceType,
    PortNoiseCorrelationResult,
};
use crate::analysis::pnoise::{NoiseContributor, PhaseNoisePoint, PnoiseResult};
use crate::analysis::pole_zero::{PoleZeroResult, RootSetEvidence, SpectrumCertificate};
use crate::analysis::pss::{PeriodicWaveform, PssResult};
use crate::analysis::s_param::{Port, SMatrix, SParameterResult};
use crate::analysis::sensitivity::{ElementType, Sensitivity, SensitivityResult};
use crate::analysis::stb::{BodePoint, NyquistPoint, StabilityMargins, StbResult};
use crate::analysis::transfer::TransferFunctionResult;
use crate::circuit::{DeviceOpEntry, DeviceOpReport, OpLabel};
use crate::engine::{Engine, PeriodicNoiseResult, SimulationConfig};
use crate::engine::{TransientDeviceOpTrace, TransientResult, TransientStoreTrace};
use crate::execution::plan::AnalysisKind;
use crate::netlist::Netlist;
use crate::solver::SimulationResult;

//=============================================================================
// Fixtures
//=============================================================================

fn instance(kind: AnalysisKind) -> AnalysisInstanceId {
    AnalysisInstanceId::new(kind, 0)
}

fn coordinate() -> ResultCoordinate {
    let netlist = Netlist::parse(
        "Stepped deck\n\
         .param rload=1k\n\
         R1 in out {rload}\n\
         V1 in 0 DC 1\n\
         .step param rload 1k 3k 1k\n\
         .op\n\
         .end\n",
    )
    .expect("stepped deck parses");
    let limits = crate::resource::ResourceLimits::default();
    let plan = crate::execution::DeckPlan::from_netlist(&netlist, &limits).expect("plan builds");
    let coordinates = plan
        .coordinates_with_abort(&limits, &NoAbort)
        .expect("coordinates materialize");
    ResultCoordinate::from_run_coordinate(coordinates.first().expect("at least one coordinate"))
}

fn operating_point_result() -> SimulationResult {
    let mut result = SimulationResult::new(2, 1);
    result.node_names = vec!["0".to_owned(), "in".to_owned(), "out".to_owned()];
    result.node_voltages = vec![0.0, 1.0, 0.5];
    result.branch_names = vec!["v1".to_owned()];
    result.branch_currents = vec![-0.5e-3];
    result.dc_observables = vec![("I(R1)".to_owned(), 0.5e-3)];
    result
}

fn device_report() -> DeviceOpReport {
    DeviceOpReport {
        entries: vec![DeviceOpEntry::new(
            "M1".to_owned(),
            OpLabel::MOSFET,
            Some(OpLabel::SATURATION),
            vec![(OpLabel::GM, 1.5e-3)],
        )],
    }
}

fn ac_points() -> Vec<AcResult> {
    vec![
        AcResult {
            frequency: 1.0e3,
            node_names: vec!["out".to_owned()],
            branch_names: vec!["v1".to_owned()],
            voltages: vec![Complex64::new(1.0, -0.5)],
            currents: vec![Complex64::new(-1.0e-3, 2.0e-4)],
        },
        AcResult {
            frequency: 1.0e4,
            node_names: vec!["out".to_owned()],
            branch_names: vec!["v1".to_owned()],
            voltages: vec![Complex64::new(0.25, -0.75)],
            currents: vec![Complex64::new(-2.0e-3, 5.0e-4)],
        },
    ]
}

fn transient_result() -> TransientResult {
    TransientResult {
        time: vec![0.0, 1.0e-6, 2.0e-6],
        step_sizes: vec![0.0, 1.0e-6, 1.0e-6],
        // The second node is deliberately unretained so the document has to
        // represent a whole missing series without inventing zeros.
        voltages: vec![vec![0.0, 0.5, 1.0], Vec::new()],
        branch_currents: vec![vec![0.0, -1.0e-3, -2.0e-3]],
        num_nodes: 2,
        node_names: vec!["out".to_owned(), "mid".to_owned()],
        branch_names: vec!["v1".to_owned()],
        digital_traces: Vec::new(),
        real_traces: Vec::new(),
        device_op_traces: vec![TransientDeviceOpTrace {
            device_name: "M1".to_owned(),
            parameter: "gm".to_owned(),
            values: vec![1.0e-3, 1.1e-3, 1.2e-3],
        }],
        store_traces: vec![TransientStoreTrace {
            name: "YMEMRISTOR!MR1:R".to_owned(),
            values: vec![100.0, 110.0, 120.0],
        }],
        fft_results: Vec::new(),
    }
}

fn noise_points() -> Vec<NoiseResult> {
    let contribution = |identity: NoiseSourceIdentity, output: f64| NoiseContribution {
        identity,
        noise_type: NoiseSourceType::Thermal,
        output_contribution: output,
        input_contribution: output * 2.0,
        percentage: 100.0,
    };
    vec![
        NoiseResult {
            frequency: 1.0e3,
            node_names: vec!["out".to_owned()],
            branch_names: vec!["v1".to_owned()],
            voltages: vec![Complex64::new(1.0, 0.0)],
            currents: vec![Complex64::new(-1.0e-3, 0.0)],
            output_noise_density: 4.0e-18,
            input_referred_density: 8.0e-18,
            input_gain_squared: 0.5,
            contribution_catalog: vec![NoiseSourceIdentity::device("r1")],
            mechanisms_unavailable: vec!["x1".to_owned()],
            contributions: vec![contribution(NoiseSourceIdentity::device("r1"), 4.0e-18)],
        },
        NoiseResult {
            frequency: 1.0e4,
            node_names: vec!["out".to_owned()],
            branch_names: vec!["v1".to_owned()],
            voltages: vec![Complex64::new(0.5, 0.0)],
            currents: vec![Complex64::new(-2.0e-3, 0.0)],
            output_noise_density: 2.0e-18,
            input_referred_density: 4.0e-18,
            input_gain_squared: 0.5,
            contribution_catalog: vec![NoiseSourceIdentity::device("r1")],
            mechanisms_unavailable: vec!["x1".to_owned()],
            // The contributor drops out at this frequency, so the document
            // must record absence rather than a zero contribution.
            contributions: Vec::new(),
        },
    ]
}

fn s_parameter_result() -> SParameterResult {
    let mut result = SParameterResult::new(50.0, vec![Port::single_ended(1, "in", 50.0)]);
    let mut matrix = SMatrix::new(1.0e9, 1);
    matrix.set(1, 1, Complex64::new(0.1, -0.2));
    result.add(matrix);
    result
}

fn port_noise_assembly() -> crate::analysis::s_param::PortNoiseAssembly {
    crate::analysis::s_param::PortNoiseAssembly {
        reference_temperature_kelvin: 300.15,
        points: vec![PortNoiseCorrelationResult {
            frequency: 1.0e9,
            current_correlation: vec![vec![Complex64::new(1.6e-20, 0.0)]],
        }],
        // A one-port network has no two-port noise figures.
        two_port: None,
    }
}

fn distortion_result() -> DistortionAnalysisResult {
    let response = |frequency: f64, value: Complex64| AcResult {
        frequency,
        node_names: vec!["out".to_owned()],
        branch_names: Vec::new(),
        voltages: vec![value],
        currents: Vec::new(),
    };
    DistortionAnalysisResult {
        f2_over_f1: Some(0.9),
        points: vec![DistortionPointResult {
            fundamental_f1: response(1.0e3, Complex64::new(1.0, 0.0)),
            fundamental_f2: Some(response(9.0e2, Complex64::new(0.9, 0.0))),
            products: vec![DistortionProductResult {
                product: DistortionProduct::ThirdOrderDifference,
                response: response(1.1e3, Complex64::new(1.0e-3, -2.0e-3)),
            }],
        }],
    }
}

fn stability_result() -> StbResult {
    let mut result = StbResult::new();
    result.bode_points = vec![
        BodePoint::from_loop_gain(1.0e3, Complex64::new(10.0, 0.0)),
        BodePoint::from_loop_gain(1.0e6, Complex64::new(0.5, -0.5)),
    ];
    result.nyquist_points = vec![NyquistPoint::from_loop_gain(
        Complex64::new(10.0, 0.0),
        1.0e3,
    )];
    result.margins = StabilityMargins {
        gain_margin_db: 12.0,
        gain_margin_freq: 2.0e6,
        phase_margin_deg: 60.0,
        phase_margin_freq: 1.0e6,
        dc_gain_db: 20.0,
        unity_gain_bandwidth: 1.0e6,
        conditionally_stable: false,
        num_crossovers: 1,
    };
    result.warnings = vec!["synthetic warning".to_owned()];
    result
}

fn sensitivity_result() -> SensitivityResult {
    let mut result = SensitivityResult::new("V(out)", 0.5);
    result.add(Sensitivity::new(
        "R1",
        ElementType::Resistor,
        "value",
        1.0e3,
        -2.5e-4,
        0.5,
    ));
    result
}

fn pole_zero_result() -> PoleZeroResult {
    let mut result = PoleZeroResult::new("V1", "V(out)");
    result.poles = vec![Complex64::new(-1.0e6, 0.0)];
    let certificate = SpectrumCertificate::exact(1, 0).expect("exact certificate");
    result.pole_evidence = RootSetEvidence::Qualified { certificate };
    result.zero_evidence = RootSetEvidence::NotRequested;
    result.dc_gain = Some(1.0);
    result
}

fn fourier_result() -> FourierResult {
    FourierResult {
        fundamental_freq: 1.0e3,
        dc_component: 0.01,
        harmonics: vec![
            HarmonicComponent {
                harmonic_number: 0,
                frequency: 0.0,
                magnitude: 0.01,
                phase: 0.0,
            },
            HarmonicComponent {
                harmonic_number: 1,
                frequency: 1.0e3,
                magnitude: 1.0,
                phase: -90.0,
            },
        ],
        thd: Some(1.25),
    }
}

fn monte_carlo_result() -> MonteCarloResult {
    let mut result = MonteCarloResult::new();
    result.num_runs = 3;
    result.num_failures = 0;
    result.all_converged = true;
    result.variables.insert(
        "V(out)".to_owned(),
        VariableStatistics::from_samples("V(out)", vec![0.9, 1.0, 1.1], 2),
    );
    result
}

fn pss_result() -> PssResult {
    let mut result = PssResult::new(1.0e-6, 1, 3);
    result.time = vec![0.0, 5.0e-7, 1.0e-6];
    result.node_names = vec!["out".to_owned()];
    result.waveforms = vec![PeriodicWaveform::from_values(vec![0.0, 1.0, 0.0])];
    result.iterations = 4;
    result.residual_norm = 1.0e-9;
    result
}

fn pnoise_result() -> PnoiseResult {
    let mut result = PnoiseResult::new(1.0e9, "out");
    result.add_point(PhaseNoisePoint::new(1.0e3, -100.0));
    result.add_point(PhaseNoisePoint::new(1.0e4, -120.0));
    let mut contributor = NoiseContributor::new("R1", "resistor");
    contributor.add_contribution(1.0e3, -105.0);
    result.add_contributor(contributor);
    result.set_jitter(1.0e-12, 1.0e-3, (1.0e3, 1.0e6));
    result.converged = true;
    result
}

fn harmonic_balance_result() -> HbResult {
    let mut result = HbResult::new(1.0e6, 1, 2);
    result.converged = true;
    result.iterations = 3;
    result.residual_norm = 1.0e-10;
    result.node_names = vec!["out".to_owned()];
    result.spectral_voltages = vec![SpectralVoltage {
        node_name: "out".to_owned(),
        coefficients: vec![
            Complex64::new(0.5, 0.0),
            Complex64::new(1.0, -0.25),
            Complex64::new(0.1, 0.05),
        ],
        frequencies: result.harmonic_frequencies.clone(),
    }];
    result.mna_branch_currents = vec![SpectralBranchCurrent {
        device_name: "v1".to_owned(),
        coefficients: vec![
            Complex64::new(-1.0e-3, 0.0),
            Complex64::new(-2.0e-3, 1.0e-4),
            Complex64::new(-1.0e-4, 0.0),
        ],
        frequencies: result.harmonic_frequencies.clone(),
    }];
    result.reactive_spectra = vec![HbReactiveSpectrum {
        device_name: "c1".to_owned(),
        kind: HbReactiveKind::Capacitor,
        voltage_coefficients: vec![
            Complex64::new(0.5, 0.0),
            Complex64::new(1.0, -0.25),
            Complex64::new(0.1, 0.05),
        ],
        current_coefficients: vec![
            Complex64::ZERO,
            Complex64::new(0.0, 1.0e-3),
            Complex64::new(0.0, 2.0e-4),
        ],
        dc_current_is_exact: true,
    }];
    result.tones = vec!["f0".to_owned()];
    result.solve_time_seconds = 0.25;
    result
}

fn envelope_deck() -> Netlist {
    Netlist::parse(
        "HB Envelope continuation\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 DC 0 AC 2 PULSE(0 1 250n 20n 20n 2u 10u)\n\
         Rcarrier carrier out 1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n",
    )
    .expect("envelope deck parses")
}

//=============================================================================
// One built document per family
//=============================================================================

fn document_for(kind: AnalysisResultKind) -> AnalysisResultDocument {
    let builder = match kind {
        AnalysisResultKind::OperatingPoint => AnalysisResultDocument::from_operating_point(
            instance(AnalysisKind::Op),
            &operating_point_result(),
            Some(&device_report()),
        ),
        AnalysisResultKind::DcSweep => {
            let engine = Engine::new(SimulationConfig::default());
            let netlist =
                Netlist::parse("DC sweep\nV1 in 0 DC 0\nR1 in 0 1k\n.dc V1 0 1 0.5\n.end\n")
                    .expect("DC deck parses");
            let points = engine
                .run_dc_sweep_with_report_and_abort(&netlist, "V1", 0.0, 1.0, 0.5, &NoAbort)
                .expect("DC sweep runs");
            AnalysisResultDocument::from_dc_sweep(
                instance(AnalysisKind::Dc),
                "V1",
                SignalUnit::Volt,
                &points,
            )
        }
        AnalysisResultKind::Ac => {
            AnalysisResultDocument::from_ac(instance(AnalysisKind::Ac), &ac_points())
        }
        AnalysisResultKind::Transient => AnalysisResultDocument::from_transient(
            instance(AnalysisKind::Tran),
            &transient_result(),
            None,
            Vec::new(),
        ),
        AnalysisResultKind::Noise => {
            AnalysisResultDocument::from_noise(instance(AnalysisKind::Noise), &noise_points())
        }
        AnalysisResultKind::SParameters => AnalysisResultDocument::from_s_parameters(
            instance(AnalysisKind::Sp),
            &s_parameter_result(),
        ),
        AnalysisResultKind::PortNoise => AnalysisResultDocument::from_port_noise(
            instance(AnalysisKind::Sp),
            &port_noise_assembly(),
        ),
        AnalysisResultKind::Distortion => AnalysisResultDocument::from_distortion(
            instance(AnalysisKind::Distortion),
            &distortion_result(),
        ),
        AnalysisResultKind::TransferFunction => AnalysisResultDocument::from_transfer_function(
            instance(AnalysisKind::TransferFunction),
            &TransferFunctionResult::new("V(out)", "V1", 0.5, 1.0e3, 50.0),
        ),
        AnalysisResultKind::Stability => {
            AnalysisResultDocument::from_stability(instance(AnalysisKind::Stb), &stability_result())
        }
        AnalysisResultKind::Sensitivity => AnalysisResultDocument::from_sensitivity(
            instance(AnalysisKind::Sensitivity),
            &sensitivity_result(),
        ),
        AnalysisResultKind::PoleZero => AnalysisResultDocument::from_pole_zero(
            instance(AnalysisKind::PoleZero),
            &pole_zero_result(),
        ),
        AnalysisResultKind::Fourier => AnalysisResultDocument::from_fourier(
            instance(AnalysisKind::Fourier),
            instance(AnalysisKind::Tran),
            "V(out)",
            SignalUnit::Volt,
            &fourier_result(),
        ),
        AnalysisResultKind::Fft => AnalysisResultDocument::from_transient_fft(
            instance(AnalysisKind::Fft),
            instance(AnalysisKind::Tran),
            SignalUnit::Volt,
            &transient_fft_result(),
        ),
        AnalysisResultKind::MonteCarlo => AnalysisResultDocument::from_monte_carlo(
            instance(AnalysisKind::MonteCarlo),
            &monte_carlo_result(),
        ),
        AnalysisResultKind::Pss => {
            AnalysisResultDocument::from_pss(instance(AnalysisKind::Pss), &pss_result())
        }
        AnalysisResultKind::Pac => {
            AnalysisResultDocument::from_pac(instance(AnalysisKind::Pac), &pac_result())
        }
        AnalysisResultKind::PNoise => AnalysisResultDocument::from_pnoise(
            instance(AnalysisKind::PNoise),
            &PeriodicNoiseResult::Spectral(pnoise_result()),
        ),
        AnalysisResultKind::HarmonicBalance => AnalysisResultDocument::from_harmonic_balance(
            instance(AnalysisKind::HarmonicBalance),
            &harmonic_balance_result(),
        ),
        AnalysisResultKind::Envelope => AnalysisResultDocument::from_envelope(
            instance(AnalysisKind::Envelope),
            &envelope_result(),
        ),
    };
    builder
        .unwrap_or_else(|error| panic!("{} projection failed: {error}", kind.tag()))
        .build()
        .unwrap_or_else(|error| panic!("{} document is invalid: {error}", kind.tag()))
}

fn transient_fft_result() -> crate::engine::TransientFftResult {
    use crate::engine::{TransientFftBin, TransientFftHarmonic, TransientFftMetrics};
    use crate::netlist::{FftFormat, FftOutput, FftWindow, XyceFftMode};

    crate::engine::TransientFftResult {
        output: FftOutput::Probe("V(out)".to_owned()),
        output_name: "V(out)".to_owned(),
        physical_type: "voltage",
        start_time: 0.0,
        stop_time: 1.0e-3,
        sample_interval: 1.0e-6,
        point_count: 1000,
        accurate_sampling: true,
        format: FftFormat::Unnormalized,
        mode: XyceFftMode::HspiceCompatible,
        window: FftWindow::Hann,
        window_name: "HANN".to_owned(),
        alpha: 3.0,
        coherent_gain: 0.5,
        frequency_resolution: 1.0e3,
        fundamental_bin: 1,
        minimum_metric_bin: 1,
        maximum_metric_bin: 3,
        bins: vec![
            TransientFftBin {
                index: 0,
                frequency: 0.0,
                real: 0.01,
                imaginary: 0.0,
                magnitude: 0.01,
                phase_degrees: 0.0,
            },
            TransientFftBin {
                index: 1,
                frequency: 1.0e3,
                real: 0.0,
                imaginary: -1.0,
                magnitude: 1.0,
                phase_degrees: -90.0,
            },
        ],
        metrics: Some(TransientFftMetrics {
            fundamental_magnitude: 1.0,
            thd_ratio: 0.01,
            thd_db: -40.0,
            sndr_db: 39.0,
            enob_bits: 6.2,
            snr_db: 41.0,
            sfdr_db: 45.0,
            sfdr_spur_bin: Some(1),
            sfdr_spur_frequency: Some(1.0e3),
            largest_harmonics: vec![TransientFftHarmonic {
                rank: 1,
                bin: 1,
                frequency: 1.0e3,
                magnitude: 1.0,
                magnitude_db: 0.0,
                phase_degrees: -90.0,
            }],
        }),
    }
}

fn pac_result() -> crate::analysis::PacResult {
    let mut result = crate::analysis::PacResult::new(
        1.0e6,
        vec![1.0e3, 1.0e4],
        -1,
        1,
        vec!["out".to_owned()],
        vec!["v1".to_owned()],
    )
    .expect("PAC result allocates");
    for frequency_index in 0..2 {
        for sideband in -1..=1 {
            let data = result
                .get_sideband_data_mut(frequency_index, sideband)
                .expect("sideband record exists");
            data.set_voltage(0, Complex64::new(0.5, -0.25))
                .expect("finite voltage");
            data.set_current(0, Complex64::new(-1.0e-3, 0.0))
                .expect("finite current");
        }
    }
    result.iterations = 2;
    result.residual = 1.0e-12;
    result
}

fn envelope_result() -> crate::engine::EnvelopeResult {
    Engine::new(SimulationConfig::default())
        .run_envelope_with_abort(
            &envelope_deck(),
            HbConfig::new(1.0e6).with_harmonics(4),
            &["Vmod".to_owned()],
            600.0e-9,
            10.0e-9,
            &NoAbort,
        )
        .expect("envelope run completes")
}

//=============================================================================
// Capability gate
//=============================================================================

/// Every registered result family must have a document mapping.
///
/// The match below is exhaustive on purpose: adding a variant to
/// `AnalysisResultKind` without giving it a payload and a projection stops this
/// test compiling.
#[test]
fn every_result_family_has_a_document_and_payload() {
    for kind in AnalysisResultKind::ALL {
        let document = document_for(kind);
        assert_eq!(
            document.result_kind(),
            kind,
            "{} document family",
            kind.tag()
        );
        assert_eq!(
            document.payload().result_kind(),
            kind,
            "{} payload family",
            kind.tag()
        );
        assert_eq!(document.schema(), ANALYSIS_RESULT_DOCUMENT_SCHEMA);
        assert_eq!(document.schema_version(), ANALYSIS_RESULT_DOCUMENT_VERSION);
        document
            .validate()
            .unwrap_or_else(|error| panic!("{} document is invalid: {error}", kind.tag()));
    }
}

#[test]
fn every_result_family_round_trips_through_json() {
    for kind in AnalysisResultKind::ALL {
        let document = document_for(kind);
        let json = document
            .to_json()
            .unwrap_or_else(|error| panic!("{} encode failed: {error}", kind.tag()));
        let decoded = AnalysisResultDocument::from_json(&json)
            .unwrap_or_else(|error| panic!("{} decode failed: {error}", kind.tag()));
        assert_eq!(decoded, document, "{} round trip", kind.tag());
    }
}

#[test]
fn coordinate_identity_and_topology_round_trip() {
    let fingerprint = crate::execution::TopologyFingerprint::from_materialized(
        ["0", "in", "out"],
        ["v(in)", "v(out)"],
        Vec::<String>::new(),
        [crate::execution::TopologyComponent::new(
            "resistor",
            "r1",
            "resistor",
            ["in", "out"],
            [(0, 0), (0, 1)],
        )
        .expect("component")],
    )
    .expect("fingerprint");

    let document = AnalysisResultDocument::from_ac(instance(AnalysisKind::Ac), &ac_points())
        .expect("AC projection")
        .coordinate(coordinate())
        .topology_fingerprint(fingerprint)
        .namespaces(ResultNamespaces {
            output: "run-001".to_owned(),
            checkpoint: "run-001/ac-001".to_owned(),
        })
        .build()
        .expect("document builds");

    let decoded =
        AnalysisResultDocument::from_json(&document.to_json().expect("encode")).expect("decode");
    assert_eq!(decoded, document);
    assert_eq!(decoded.topology_fingerprint(), Some(fingerprint));
    let restored = decoded.coordinate().expect("coordinate survives");
    assert_eq!(restored.id(), coordinate().id());
    assert_eq!(restored.assignments().len(), 1);
    assert_eq!(
        restored.assignments()[0].kind(),
        crate::execution::AxisKind::Step
    );
}

//=============================================================================
// Per-family fidelity
//=============================================================================

fn samples_of(document: &AnalysisResultDocument, canonical: &str) -> SeriesValues {
    document
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == canonical)
        .unwrap_or_else(|| panic!("signal {canonical} is present"))
        .values()
        .clone()
}

fn scalar_of(document: &AnalysisResultDocument, name: &str) -> ResultScalar {
    document
        .scalars()
        .iter()
        .find(|scalar| scalar.name() == name)
        .unwrap_or_else(|| panic!("scalar {name} is present"))
        .clone()
}

#[test]
fn operating_point_document_matches_its_source() {
    let source = operating_point_result();
    let document = document_for(AnalysisResultKind::OperatingPoint);
    assert_eq!(document.point_count(), 1);
    assert_eq!(
        samples_of(&document, "v(out)"),
        SeriesValues::Real {
            samples: vec![Some(0.5)]
        }
    );
    assert_eq!(
        samples_of(&document, "i(v1)"),
        SeriesValues::Real {
            samples: vec![Some(source.branch_currents[0])]
        }
    );
    let voltage = document
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == "v(out)")
        .expect("out voltage");
    assert_eq!(voltage.descriptor().unit(), &SignalUnit::Volt);
    assert_eq!(
        voltage.descriptor().owner(),
        &crate::execution::SignalOwner::Node("out".to_owned())
    );

    let ResultPayload::Op(payload) = document.payload() else {
        panic!("operating-point payload");
    };
    assert_eq!(payload.observables.len(), 1);
    assert_eq!(payload.observables[0].name, "I(R1)");
    assert_eq!(payload.observables[0].value, Some(0.5e-3));

    let state = document.device_states().first().expect("device state");
    assert_eq!(state.device_name(), "M1");
    assert_eq!(state.device_kind(), Some("MOSFET"));
    assert_eq!(state.regions(), [Some("saturation".to_owned())]);
    assert_eq!(state.parameters()[0].values, vec![Some(1.5e-3)]);
}

#[test]
fn ac_document_preserves_complex_values_units_and_names() {
    let source = ac_points();
    let document = document_for(AnalysisResultKind::Ac);
    assert_eq!(document.point_count(), 2);
    assert_eq!(document.axes().len(), 1);
    assert_eq!(document.axes()[0].unit(), &SignalUnit::Hertz);
    assert_eq!(
        document.axes()[0].values(),
        &AxisValues::Real {
            values: vec![1.0e3, 1.0e4]
        }
    );
    assert_eq!(
        samples_of(&document, "v(out)"),
        SeriesValues::Complex {
            samples: source
                .iter()
                .map(|point| Some(ComplexSample::from(point.voltages[0])))
                .collect()
        }
    );
    assert_eq!(
        samples_of(&document, "i(v1)"),
        SeriesValues::Complex {
            samples: source
                .iter()
                .map(|point| Some(ComplexSample::from(point.currents[0])))
                .collect()
        }
    );
}

#[test]
fn transient_document_keeps_unretained_series_as_explicit_absence() {
    let document = document_for(AnalysisResultKind::Transient);
    let mid = document
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == "v(mid)")
        .expect("the unretained node keeps its descriptor");
    assert_eq!(mid.availability(), SeriesAvailability::NotProjected);
    assert!(!mid.has_any_sample());
    assert_eq!(
        mid.values(),
        &SeriesValues::Real {
            samples: vec![None, None, None]
        }
    );
    assert_eq!(mid.descriptor().unit(), &SignalUnit::Volt);

    let ResultPayload::Tran(payload) = document.payload() else {
        panic!("transient payload");
    };
    assert_eq!(payload.step_sizes, vec![0.0, 1.0e-6, 1.0e-6]);
    assert_eq!(payload.store_traces[0].name, "YMEMRISTOR!MR1:R");
    assert_eq!(
        payload.store_traces[0].values,
        vec![Some(100.0), Some(110.0), Some(120.0)]
    );
    let state = document.device_states().first().expect("device trace");
    assert_eq!(state.device_name(), "M1");
    assert_eq!(state.parameters()[0].name, "gm");
    assert_eq!(
        state.parameters()[0].values,
        vec![Some(1.0e-3), Some(1.1e-3), Some(1.2e-3)]
    );
}

#[test]
fn noise_document_records_an_absent_contributor_without_zero_filling() {
    let document = document_for(AnalysisResultKind::Noise);
    let onoise = document
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == "onoise_spectrum")
        .expect("output noise density");
    assert_eq!(
        onoise.descriptor().unit(),
        &SignalUnit::Custom("V^2/Hz".to_owned())
    );
    assert_eq!(
        onoise.values(),
        &SeriesValues::Real {
            samples: vec![Some(4.0e-18), Some(2.0e-18)]
        }
    );

    let ResultPayload::Noise(payload) = document.payload() else {
        panic!("noise payload");
    };
    assert_eq!(payload.mechanisms_unavailable, ["x1"]);
    let contribution = &payload.contributions[0];
    assert_eq!(contribution.identity.device, "r1");
    assert_eq!(contribution.mechanism_kind, NoiseMechanismTag::Thermal);
    assert_eq!(
        contribution.output_contribution,
        vec![Some(4.0e-18), None],
        "a contributor that stops reporting must be absent, never zero"
    );
}

#[test]
fn s_parameter_and_port_noise_documents_carry_port_identity() {
    let document = document_for(AnalysisResultKind::SParameters);
    assert_eq!(
        samples_of(&document, "s(1,1)"),
        SeriesValues::Complex {
            samples: vec![Some(ComplexSample::new(0.1, -0.2))]
        }
    );
    let ResultPayload::Sp(payload) = document.payload() else {
        panic!("S-parameter payload");
    };
    assert_eq!(payload.reference_impedance, 50.0);
    assert_eq!(payload.ports[0].node_positive, "in");
    assert_eq!(payload.ports[0].node_negative, "0");

    let port_noise = document_for(AnalysisResultKind::PortNoise);
    assert_eq!(port_noise.analysis().kind(), AnalysisKind::Sp);
    let ResultPayload::PortNoise(payload) = port_noise.payload() else {
        panic!("port-noise payload");
    };
    assert_eq!(payload.port_count, 1);
    let entry = port_noise
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == "cy(1,1)")
        .expect("correlation entry");
    assert_eq!(
        entry.descriptor().unit(),
        &SignalUnit::Custom("A^2/Hz".to_owned())
    );
}

#[test]
fn distortion_document_keeps_product_identity_and_order() {
    let document = document_for(AnalysisResultKind::Distortion);
    let ResultPayload::Distortion(payload) = document.payload() else {
        panic!("distortion payload");
    };
    assert_eq!(payload.f2_over_f1, Some(0.9));
    assert_eq!(payload.products.len(), 1);
    assert_eq!(payload.products[0].product.label(), "2f1-f2");
    assert_eq!(payload.products[0].order, 3);
    assert_eq!(payload.products[0].frequencies, vec![1.1e3]);

    let qualifiers = document
        .signals()
        .iter()
        .filter_map(ResultSignal::qualifier)
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        qualifiers.contains(&SeriesQualifier::DistortionFundamental {
            tone: DistortionTone::F1
        })
    );
    assert!(
        qualifiers.contains(&SeriesQualifier::DistortionFundamental {
            tone: DistortionTone::F2
        })
    );
    assert!(qualifiers.contains(&SeriesQualifier::DistortionProduct {
        product: DistortionProductTag::ThirdOrderDifference
    }));
    // Same node name, three different responses: the qualifier is what keeps
    // them distinct.
    assert_eq!(
        document
            .signals()
            .iter()
            .filter(|signal| signal.descriptor().canonical_name() == "v(out)")
            .count(),
        3
    );
}

#[test]
fn stability_document_carries_typed_margins_and_nyquist() {
    let document = document_for(AnalysisResultKind::Stability);
    assert_eq!(
        scalar_of(&document, "gain_margin_db").unit(),
        Some(&SignalUnit::Custom("dB".to_owned()))
    );
    assert_eq!(
        scalar_of(&document, "gain_margin_db").value(),
        &ScalarValue::Real { value: Some(12.0) }
    );
    assert_eq!(
        scalar_of(&document, "phase_margin_degrees").unit(),
        Some(&SignalUnit::Degree)
    );
    assert_eq!(
        scalar_of(&document, "unity_gain_crossovers").value(),
        &ScalarValue::Count { value: 1 }
    );
    let ResultPayload::Stb(payload) = document.payload() else {
        panic!("stability payload");
    };
    assert_eq!(payload.warnings, ["synthetic warning"]);
    assert_eq!(payload.nyquist.len(), 1);
    assert_eq!(payload.nyquist[0].real, 10.0);
}

#[test]
fn sensitivity_transfer_and_pole_zero_documents_have_no_series() {
    let sensitivity = document_for(AnalysisResultKind::Sensitivity);
    assert_eq!(sensitivity.point_count(), 0);
    assert!(sensitivity.signals().is_empty());
    let ResultPayload::Sensitivity(payload) = sensitivity.payload() else {
        panic!("sensitivity payload");
    };
    assert_eq!(payload.entries[0].vector_name, "R1");
    assert_eq!(
        payload.entries[0].element_kind,
        SensitivityElementTag::Resistor
    );
    assert_eq!(payload.entries[0].absolute, -2.5e-4);

    let transfer = document_for(AnalysisResultKind::TransferFunction);
    assert_eq!(
        scalar_of(&transfer, "input_impedance").unit(),
        Some(&SignalUnit::Ohm)
    );

    let pole_zero = document_for(AnalysisResultKind::PoleZero);
    let ResultPayload::PoleZero(payload) = pole_zero.payload() else {
        panic!("pole-zero payload");
    };
    assert_eq!(payload.poles, vec![ComplexSample::new(-1.0e6, 0.0)]);
    assert!(payload.zeros.is_empty());
    assert!(payload.to_pole_evidence().is_qualified());
    assert_eq!(payload.zero_evidence, RootSetEvidenceDocument::NotRequested);
    assert_eq!(payload.dc_gain, Some(1.0));
}

#[test]
fn fourier_and_fft_documents_name_their_parent_transient() {
    let fourier = document_for(AnalysisResultKind::Fourier);
    assert_eq!(
        fourier.parent_analysis().map(AnalysisInstanceId::kind),
        Some(AnalysisKind::Tran)
    );
    assert_eq!(fourier.axes().len(), 2);
    assert_eq!(
        fourier.axes()[0].values(),
        &AxisValues::Integer { values: vec![0, 1] }
    );
    assert_eq!(
        scalar_of(&fourier, "total_harmonic_distortion").value(),
        &ScalarValue::Real { value: Some(1.25) }
    );

    let fft = document_for(AnalysisResultKind::Fft);
    assert_eq!(
        fft.parent_analysis().map(AnalysisInstanceId::kind),
        Some(AnalysisKind::Tran)
    );
    let ResultPayload::Fft(payload) = fft.payload() else {
        panic!("FFT payload");
    };
    assert_eq!(payload.window, FftWindowTag::Hann);
    assert_eq!(payload.window_name, "HANN");
    assert_eq!(
        payload.coefficient_format,
        FftCoefficientFormatTag::Unnormalized
    );
    assert_eq!(payload.sample_count, 1000);
    let metrics = payload.metrics.as_ref().expect("FFT metrics");
    assert_eq!(metrics.sfdr_spur_bin, Some(1));
    assert_eq!(metrics.largest_harmonics.len(), 1);
    assert_eq!(
        samples_of(&fft, "spectrum"),
        SeriesValues::Complex {
            samples: vec![
                Some(ComplexSample::new(0.01, 0.0)),
                Some(ComplexSample::new(0.0, -1.0)),
            ]
        }
    );
}

#[test]
fn monte_carlo_pss_pac_and_pnoise_documents_keep_their_typed_payloads() {
    let monte_carlo = document_for(AnalysisResultKind::MonteCarlo);
    let ResultPayload::MonteCarlo(payload) = monte_carlo.payload() else {
        panic!("Monte Carlo payload");
    };
    assert_eq!(payload.statistics[0].name, "V(out)");
    assert_eq!(
        payload.statistics[0].samples,
        vec![Some(0.9), Some(1.0), Some(1.1)]
    );
    assert_eq!(payload.statistics[0].minimum, Some(0.9));
    assert_eq!(
        scalar_of(&monte_carlo, "completed_runs").value(),
        &ScalarValue::Count { value: 3 }
    );

    let pss = document_for(AnalysisResultKind::Pss);
    assert_eq!(pss.axes()[0].unit(), &SignalUnit::Second);
    assert_eq!(
        scalar_of(&pss, "period").value(),
        &ScalarValue::Real {
            value: Some(1.0e-6)
        }
    );

    let pac = document_for(AnalysisResultKind::Pac);
    let ResultPayload::Pac(payload) = pac.payload() else {
        panic!("PAC payload");
    };
    assert_eq!(payload.sideband_minimum, -1);
    assert_eq!(payload.sideband_maximum, 1);
    assert_eq!(payload.sidebands.len(), 3);
    let sideband_qualifiers = pac
        .signals()
        .iter()
        .filter_map(ResultSignal::qualifier)
        .cloned()
        .collect::<Vec<_>>();
    assert!(sideband_qualifiers.contains(&SeriesQualifier::PacSideband { sideband: -1 }));
    assert!(sideband_qualifiers.contains(&SeriesQualifier::PacSideband { sideband: 1 }));

    let pnoise = document_for(AnalysisResultKind::PNoise);
    let ResultPayload::PNoise(payload) = pnoise.payload() else {
        panic!("PNoise payload");
    };
    assert_eq!(payload.output_node, "out");
    assert_eq!(payload.contributors[0].name, "R1");
    assert_eq!(
        payload.jitter_bandwidth,
        Some(PNoiseBandwidth {
            start: 1.0e3,
            stop: 1.0e6
        })
    );
    assert_eq!(
        scalar_of(&pnoise, "rms_jitter").unit(),
        Some(&SignalUnit::Second)
    );
}

#[test]
fn harmonic_balance_document_keeps_reactive_spectra_and_harmonic_axes() {
    let source = harmonic_balance_result();
    let document = document_for(AnalysisResultKind::HarmonicBalance);
    assert_eq!(document.point_count(), source.harmonic_frequencies.len());
    assert_eq!(
        document.axes()[1].values(),
        &AxisValues::Real {
            values: source.harmonic_frequencies.clone()
        }
    );
    assert_eq!(
        samples_of(&document, "v(out)"),
        SeriesValues::Complex {
            samples: source.spectral_voltages[0]
                .coefficients
                .iter()
                .map(|value| Some(ComplexSample::from(*value)))
                .collect()
        }
    );
    assert_eq!(
        samples_of(&document, "i(v1)"),
        SeriesValues::Complex {
            samples: source.mna_branch_currents[0]
                .coefficients
                .iter()
                .map(|value| Some(ComplexSample::from(*value)))
                .collect()
        }
    );
    let ResultPayload::Hb(payload) = document.payload() else {
        panic!("HB payload");
    };
    assert_eq!(payload.tones, ["f0"]);
    assert_eq!(payload.reactive_spectra[0].device_name, "c1");
    assert_eq!(
        payload.reactive_spectra[0].kind,
        HbReactiveKindTag::Capacitor
    );
    assert!(payload.reactive_spectra[0].dc_current_is_exact);
}

#[test]
fn envelope_document_carries_carrier_state_and_continued_transient() {
    let source = envelope_result();
    let document = document_for(AnalysisResultKind::Envelope);
    assert_eq!(document.result_kind(), AnalysisResultKind::Envelope);
    assert_eq!(
        document.point_count(),
        source.continued_transient().time.len()
    );
    let ResultPayload::Envelope(payload) = document.payload() else {
        panic!("envelope payload");
    };
    assert_eq!(
        payload.continuation.guarantee,
        EnvelopeGuaranteeTag::ExactLinearRcMnaV1
    );
    assert_eq!(payload.continuation.slow_time_duration, 600.0e-9);
    assert_eq!(payload.continuation.slow_time_max_step, 10.0e-9);
    assert_eq!(payload.continuation.carrier_harmonics, 4);
    assert_eq!(
        payload.continuation.canonical_frozen_sources,
        source.state().canonical_frozen_sources()
    );
    assert_eq!(
        payload.carrier.harmonic_frequencies,
        source.carrier().harmonic_frequencies
    );
    assert!(!payload.carrier.node_spectra.is_empty());
    assert_eq!(payload.transient.step_sizes.len(), document.point_count());
}

#[test]
fn envelope_runner_composes_the_two_authenticated_halves() {
    let netlist = envelope_deck();
    let engine = Engine::new(SimulationConfig::default());
    let config = HbConfig::new(1.0e6).with_harmonics(4);
    let composed = engine
        .run_envelope_with_abort(
            &netlist,
            config.clone(),
            &["Vmod".to_owned()],
            600.0e-9,
            10.0e-9,
            &NoAbort,
        )
        .expect("envelope run completes");
    let (carrier, state) = engine
        .run_hb_envelope_continuation_state(&netlist, config.clone(), &["Vmod".to_owned()])
        .expect("carrier solve");
    let (transient, _) = engine
        .run_tran_from_hb_envelope_state(
            &netlist,
            &config,
            &["Vmod".to_owned()],
            &state,
            600.0e-9,
            10.0e-9,
        )
        .expect("continuation");
    assert_eq!(
        composed.carrier().spectral_voltages.len(),
        carrier.result.spectral_voltages.len()
    );
    assert_eq!(composed.continued_transient().time, transient.time);
    assert_eq!(composed.time_origin(), state.time_origin());
}

//=============================================================================
// Version, shape, and encoding contracts
//=============================================================================

#[test]
fn a_forward_version_is_rejected_before_any_field_is_decoded() {
    let document = document_for(AnalysisResultKind::Ac);
    let json = document.to_json().expect("encode");
    let forward = json.replace("\"schemaVersion\":1", "\"schemaVersion\":2");
    assert_ne!(forward, json, "the version field must be present");
    // Also add a field this build has never seen, so a decoder that got past
    // the version check would fail with an unknown-field error instead.
    let forward = forward.replacen('{', "{\"futureField\":true,", 1);
    assert_eq!(
        AnalysisResultDocument::from_json(&forward),
        Err(ResultDocumentError::UnsupportedVersion {
            found: 2,
            current: ANALYSIS_RESULT_DOCUMENT_VERSION,
        })
    );

    let wrong_schema = json.replace(ANALYSIS_RESULT_DOCUMENT_SCHEMA, "rspice-something-else");
    assert!(matches!(
        AnalysisResultDocument::from_json(&wrong_schema),
        Err(ResultDocumentError::WrongSchema { .. })
    ));
}

#[test]
fn an_unknown_field_at_the_current_version_is_rejected() {
    let json = document_for(AnalysisResultKind::Ac)
        .to_json()
        .expect("encode");
    let tampered = json.replacen('{', "{\"unexpected\":1,", 1);
    assert!(matches!(
        AnalysisResultDocument::from_json(&tampered),
        Err(ResultDocumentError::Json(_))
    ));
}

fn real_signal(name: &str, samples: Vec<Option<f64>>) -> ResultSignal {
    ResultSignal::new(
        SignalDescriptor::new(
            format!("v({name})"),
            format!("V({name})"),
            crate::execution::SignalKind::Voltage,
            SignalUnit::Volt,
            SignalValueType::Real,
            crate::execution::SignalShape::Vector,
            crate::execution::SignalOwner::Node(name.to_owned()),
        )
        .expect("descriptor"),
        None,
        SeriesAvailability::Available,
        SeriesValues::Real { samples },
    )
    .expect("signal")
}

fn two_point_axis() -> ResultAxis {
    ResultAxis::new(
        "time",
        "Time",
        ResultAxisKind::Time,
        SignalUnit::Second,
        AxisValues::Real {
            values: vec![0.0, 1.0],
        },
    )
    .expect("axis")
}

#[test]
fn a_series_whose_length_disagrees_with_the_point_count_is_rejected() {
    let error = AnalysisResultDocument::builder(
        instance(AnalysisKind::Tran),
        ResultPayload::Tran(TransientPayload {
            step_sizes: vec![0.0, 1.0],
            store_traces: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            fft_children: Vec::new(),
            compression: None,
        }),
        2,
    )
    .axis(two_point_axis())
    .signal(real_signal("out", vec![Some(0.0)]))
    .build()
    .expect_err("a short series must be rejected");
    assert!(matches!(
        error,
        ResultDocumentError::SeriesLength {
            expected: 2,
            actual: 1,
            ..
        }
    ));
}

#[test]
fn duplicate_signal_identities_are_rejected() {
    let error = AnalysisResultDocument::builder(
        instance(AnalysisKind::Ac),
        ResultPayload::Ac(AcPayload {}),
        2,
    )
    .axis(two_point_axis())
    .signal(real_signal("out", vec![Some(0.0), Some(1.0)]))
    .signal(real_signal("OUT", vec![Some(0.0), Some(1.0)]))
    .build()
    .expect_err("one canonical identity may appear once");
    assert!(matches!(
        error,
        ResultDocumentError::DuplicateSeries {
            location: "signal",
            ..
        }
    ));
}

#[test]
fn a_series_that_declares_absence_may_not_carry_samples() {
    let descriptor = SignalDescriptor::new(
        "v(out)",
        "V(out)",
        crate::execution::SignalKind::Voltage,
        SignalUnit::Volt,
        SignalValueType::Real,
        crate::execution::SignalShape::Vector,
        crate::execution::SignalOwner::Node("out".to_owned()),
    )
    .expect("descriptor");
    let error = ResultSignal::new(
        descriptor,
        None,
        SeriesAvailability::AbsentAtCoordinate,
        SeriesValues::Real {
            samples: vec![Some(1.0)],
        },
    )
    .expect_err("absence and samples cannot coexist");
    assert!(matches!(error, ResultDocumentError::Malformed { .. }));
}

#[test]
fn a_descriptor_that_disagrees_with_its_encoding_is_rejected() {
    let descriptor = SignalDescriptor::new(
        "v(out)",
        "V(out)",
        crate::execution::SignalKind::Voltage,
        SignalUnit::Volt,
        SignalValueType::Complex,
        crate::execution::SignalShape::Vector,
        crate::execution::SignalOwner::Node("out".to_owned()),
    )
    .expect("descriptor");
    let error = ResultSignal::new(
        descriptor,
        None,
        SeriesAvailability::Available,
        SeriesValues::Real {
            samples: vec![Some(1.0)],
        },
    )
    .expect_err("a complex descriptor cannot carry real samples");
    assert!(matches!(
        error,
        ResultDocumentError::SignalValueType {
            declared: SignalValueType::Complex,
            encoded: SignalValueType::Real,
            ..
        }
    ));
}

#[test]
fn a_non_finite_source_sample_is_rejected_with_its_signal_named() {
    let mut result = transient_result();
    result.voltages[0][1] = f64::INFINITY;
    let error = AnalysisResultDocument::from_transient(
        instance(AnalysisKind::Tran),
        &result,
        None,
        Vec::new(),
    )
    .expect_err("JSON cannot encode an infinity, so the projection must refuse");
    let ResultDocumentError::SourceResult { detail, .. } = &error else {
        panic!("expected a source-result error, got {error}");
    };
    assert!(detail.contains("out"), "{detail}");
}

#[test]
fn a_post_process_family_must_name_a_transient_parent() {
    let missing = AnalysisResultDocument::builder(
        instance(AnalysisKind::Fourier),
        ResultPayload::Fourier(FourierPayload {
            output: "V(out)".to_owned(),
        }),
        0,
    )
    .build()
    .expect_err("Fourier results derive from a transient");
    assert!(matches!(
        missing,
        ResultDocumentError::MissingParentAnalysis {
            result_kind: AnalysisResultKind::Fourier
        }
    ));

    let wrong = AnalysisResultDocument::builder(
        instance(AnalysisKind::Fourier),
        ResultPayload::Fourier(FourierPayload {
            output: "V(out)".to_owned(),
        }),
        0,
    )
    .parent_analysis(instance(AnalysisKind::Ac))
    .build()
    .expect_err("an AC parent is not a transient");
    assert!(matches!(
        wrong,
        ResultDocumentError::WrongParentAnalysis {
            expected: [AnalysisKind::Tran],
            found: AnalysisKind::Ac,
            ..
        }
    ));

    let unexpected = AnalysisResultDocument::from_ac(instance(AnalysisKind::Ac), &ac_points())
        .expect("AC projection")
        .parent_analysis(instance(AnalysisKind::Tran))
        .build()
        .expect_err("AC is not a post-process");
    assert!(matches!(
        unexpected,
        ResultDocumentError::UnexpectedParentAnalysis { .. }
    ));
}

#[test]
fn a_payload_from_another_family_cannot_be_attached_to_an_analysis() {
    let error = AnalysisResultDocument::builder(
        instance(AnalysisKind::Ac),
        ResultPayload::Op(OperatingPointPayload {
            observables: Vec::new(),
        }),
        0,
    )
    .build()
    .expect_err("an AC card does not produce an operating point");
    assert!(matches!(
        error,
        ResultDocumentError::AnalysisFamilyMismatch { .. }
    ));
}

#[test]
fn encoding_stops_at_the_byte_limit_and_on_abort() {
    let document = document_for(AnalysisResultKind::Ac);
    let full = document.to_json().expect("encode");
    let limit = (full.len() / 2) as u64;
    assert_eq!(
        document.to_json_with_abort(&NoAbort, limit),
        Err(ResultDocumentError::ArtifactTooLarge { limit_bytes: limit })
    );
    assert_eq!(
        document.to_json_with_abort(&ImmediateAbort, u64::MAX),
        Err(ResultDocumentError::Aborted)
    );

    // An abort that fires only once serialization is under way is still
    // observed at the byte that crosses it.
    let abort = CountingAbort::new(1);
    assert_eq!(
        document.to_json_with_abort(&abort, u64::MAX),
        Err(ResultDocumentError::Aborted)
    );

    assert_eq!(
        AnalysisResultDocument::from_json_with_abort(&full, &NoAbort, limit),
        Err(ResultDocumentError::ArtifactTooLarge { limit_bytes: limit })
    );
    assert_eq!(
        AnalysisResultDocument::from_json_with_abort(&full, &ImmediateAbort, u64::MAX),
        Err(ResultDocumentError::Aborted)
    );
}

#[test]
fn windows_are_bounded_and_carry_an_explicit_validity_mask() {
    let document = document_for(AnalysisResultKind::Transient);
    assert_eq!(document.point_count(), 3);
    assert_eq!(
        document.window(2, 2),
        Err(ResultDocumentError::WindowOutOfBounds {
            start: 2,
            count: 2,
            point_count: 3,
        })
    );
    assert_eq!(
        document.window(4, 0),
        Err(ResultDocumentError::WindowOutOfBounds {
            start: 4,
            count: 0,
            point_count: 3,
        })
    );

    let window = document.window(1, 2).expect("window fits");
    assert_eq!(window.start, 1);
    assert_eq!(window.count, 2);
    assert_eq!(window.point_count, 3);
    assert_eq!(
        window.axes[0].values,
        AxisValues::Real {
            values: vec![1.0e-6, 2.0e-6]
        }
    );
    let retained = window
        .signals
        .iter()
        .find(|signal| signal.canonical_name == "v(out)")
        .expect("retained node");
    assert_eq!(
        retained.values,
        SeriesWindowValues::Real {
            values: vec![0.5, 1.0],
            validity: vec![1, 1],
        }
    );
    let absent = window
        .signals
        .iter()
        .find(|signal| signal.canonical_name == "v(mid)")
        .expect("unretained node");
    assert_eq!(
        absent.values,
        SeriesWindowValues::Real {
            values: vec![0.0, 0.0],
            validity: vec![0, 0],
        },
        "a placeholder zero must always be paired with a zero validity byte"
    );
}

#[test]
fn value_accounting_counts_every_retained_number() {
    let document = document_for(AnalysisResultKind::Ac);
    // One frequency axis plus two complex signals is five values per point.
    assert_eq!(document.values_per_point(), 5);
    assert_eq!(document.total_value_count(), 10);

    let transient = document_for(AnalysisResultKind::Transient);
    // Time, two node voltages, one branch current: four values per point.
    assert_eq!(transient.values_per_point(), 4);
    assert!(transient.total_value_count() >= 4 * transient.point_count());
}

//=============================================================================
// Quantities with no finite value
//=============================================================================

fn scalar_value_of(document: &AnalysisResultDocument, name: &str) -> ScalarValue {
    scalar_of(document, name).value().clone()
}

#[test]
fn an_ideal_source_publishes_unbounded_input_impedance_instead_of_failing() {
    let document = AnalysisResultDocument::from_transfer_function(
        instance(AnalysisKind::TransferFunction),
        &TransferFunctionResult::new("V(out)", "V1", 0.5, f64::INFINITY, 50.0),
    )
    .expect("an unbounded input impedance is a determination, not a projection failure")
    .build()
    .expect("document builds");
    assert_eq!(
        scalar_value_of(&document, "input_impedance"),
        ScalarValue::Unavailable {
            reason: ScalarUnavailability::PositiveInfinity
        }
    );
    assert_eq!(
        scalar_value_of(&document, "output_impedance"),
        ScalarValue::Real { value: Some(50.0) }
    );
}

#[test]
fn a_transfer_function_nan_is_still_a_projection_failure() {
    AnalysisResultDocument::from_transfer_function(
        instance(AnalysisKind::TransferFunction),
        &TransferFunctionResult::new("V(out)", "V1", f64::NAN, 1.0e3, 50.0),
    )
    .expect_err("NaN is a defect in the producing computation, not a determination");
}

#[test]
fn a_loop_with_no_crossover_records_the_absence_rather_than_zero_hertz() {
    let mut result = stability_result();
    result.margins = StabilityMargins {
        gain_margin_db: f64::INFINITY,
        gain_margin_freq: 0.0,
        phase_margin_deg: f64::INFINITY,
        phase_margin_freq: 0.0,
        dc_gain_db: -40.0,
        unity_gain_bandwidth: 0.0,
        conditionally_stable: false,
        num_crossovers: 0,
    };
    let document = AnalysisResultDocument::from_stability(instance(AnalysisKind::Stb), &result)
        .expect("an unconditionally stable loop must publish, not fail closed")
        .build()
        .expect("document builds");
    assert_eq!(
        scalar_value_of(&document, "gain_margin_db"),
        ScalarValue::Unavailable {
            reason: ScalarUnavailability::PositiveInfinity
        }
    );
    for name in [
        "gain_margin_frequency",
        "phase_margin_frequency",
        "unity_gain_bandwidth",
    ] {
        assert_eq!(
            scalar_value_of(&document, name),
            ScalarValue::Unavailable {
                reason: ScalarUnavailability::NoCrossover
            },
            "{name} must record the missing crossover rather than naming DC"
        );
    }
}

#[test]
fn a_loop_that_never_leaves_unity_gain_records_a_negative_divergence() {
    let mut result = stability_result();
    result.margins = StabilityMargins {
        gain_margin_db: f64::NEG_INFINITY,
        gain_margin_freq: 0.0,
        phase_margin_deg: f64::NEG_INFINITY,
        phase_margin_freq: 0.0,
        dc_gain_db: 60.0,
        unity_gain_bandwidth: 0.0,
        conditionally_stable: false,
        num_crossovers: 0,
    };
    let document = AnalysisResultDocument::from_stability(instance(AnalysisKind::Stb), &result)
        .expect("projection succeeds")
        .build()
        .expect("document builds");
    assert_eq!(
        scalar_value_of(&document, "phase_margin_degrees"),
        ScalarValue::Unavailable {
            reason: ScalarUnavailability::NegativeInfinity
        }
    );
}

#[test]
fn a_crossing_loop_still_publishes_its_finite_margins() {
    let document =
        AnalysisResultDocument::from_stability(instance(AnalysisKind::Stb), &stability_result())
            .expect("projection succeeds")
            .build()
            .expect("document builds");
    assert_eq!(
        scalar_value_of(&document, "gain_margin_db"),
        ScalarValue::Real { value: Some(12.0) }
    );
    assert_eq!(
        scalar_value_of(&document, "unity_gain_bandwidth"),
        ScalarValue::Real { value: Some(1.0e6) }
    );
}

//=============================================================================
// Periodic small-signal parents
//=============================================================================

#[test]
fn a_pac_result_accepts_either_periodic_large_signal_parent() {
    for parent in [AnalysisKind::Pss, AnalysisKind::HarmonicBalance] {
        AnalysisResultDocument::from_pac(instance(AnalysisKind::Pac), &pac_result())
            .expect("PAC projection")
            .parent_analysis(instance(parent))
            .build()
            .unwrap_or_else(|error| panic!("a {parent:?} carrier must be accepted: {error}"));
    }
}

#[test]
fn a_pac_result_still_refuses_a_transient_parent() {
    let error = AnalysisResultDocument::from_pac(instance(AnalysisKind::Pac), &pac_result())
        .expect("PAC projection")
        .parent_analysis(instance(AnalysisKind::Tran))
        .build()
        .expect_err("a transient is not a periodic carrier");
    assert!(
        error.to_string().contains("pss or hb"),
        "the refusal names both accepted carriers: {error}"
    );
}

//=============================================================================
// Nested DC sweeps
//=============================================================================

#[test]
fn a_nested_dc_sweep_publishes_every_authored_sweep_variable() {
    let engine = Engine::new(SimulationConfig::default());
    let netlist = Netlist::parse(
        "Nested DC sweep\n\
         V1 in 0 DC 0\n\
         V2 mid 0 DC 0\n\
         R1 in mid 1k\n\
         R2 mid 0 1k\n\
         .dc V1 0 1 0.5 V2 0 1 1\n\
         .end\n",
    )
    .expect("nested DC deck parses");
    let command = netlist
        .analyses
        .iter()
        .find(|command| matches!(command, crate::netlist::AnalysisCommand::Dc { .. }))
        .expect("the deck authors a .DC card");
    let crate::netlist::AnalysisCommand::Dc {
        source,
        start,
        stop,
        step,
        mode,
        sweep2: Some(outer),
    } = command
    else {
        panic!("the .DC card must carry an outer sweep");
    };
    let primary = crate::netlist::DcSweepSpec {
        start: *start,
        stop: *stop,
        step: *step,
        mode: mode.clone(),
    };
    let points = engine
        .run_dc_sweep2_spec_with_report_and_abort(&netlist, source, &primary, Some(outer), &NoAbort)
        .expect("nested DC sweep runs");
    let document = AnalysisResultDocument::from_nested_dc_sweep(
        instance(AnalysisKind::Dc),
        &[
            DcSweepAxisDocument {
                name: outer.source.trim().to_ascii_lowercase(),
                unit: SignalUnit::Volt,
                value_count: outer.spec().points().len(),
            },
            DcSweepAxisDocument {
                name: source.trim().to_ascii_lowercase(),
                unit: SignalUnit::Volt,
                value_count: primary.points().len(),
            },
        ],
        &points,
    )
    .expect("nested projection")
    .build()
    .expect("document builds");
    let ResultPayload::Dc(payload) = document.payload() else {
        panic!("a .DC card projects a DC payload");
    };
    assert_eq!(
        payload
            .sweep_variables
            .iter()
            .map(|axis| axis.name.as_str())
            .collect::<Vec<_>>(),
        vec!["v2", "v1"],
        "the outer source must survive beside the inner one"
    );
    assert_eq!(
        payload.primary_variable().map(|axis| axis.name.as_str()),
        Some("v1")
    );
}

#[test]
fn a_declared_dc_grid_that_does_not_match_the_result_is_refused() {
    let engine = Engine::new(SimulationConfig::default());
    let netlist = Netlist::parse("DC sweep\nV1 in 0 DC 0\nR1 in 0 1k\n.dc V1 0 1 0.5\n.end\n")
        .expect("DC deck parses");
    let points = engine
        .run_dc_sweep_with_report_and_abort(&netlist, "V1", 0.0, 1.0, 0.5, &NoAbort)
        .expect("DC sweep runs");
    AnalysisResultDocument::from_nested_dc_sweep(
        instance(AnalysisKind::Dc),
        &[DcSweepAxisDocument {
            name: "v1".to_owned(),
            unit: SignalUnit::Volt,
            value_count: points.len() + 1,
        }],
        &points,
    )
    .expect_err("a declared grid that disagrees with the result must be refused");
}

//=============================================================================
// Units
//=============================================================================

#[test]
fn an_unspecified_unit_survives_the_wire_round_trip_distinct_from_dimensionless() {
    use crate::execution::schema::{SignalKind, SignalOwner, SignalShape};

    let descriptor = SignalDescriptor::new(
        "param:rload",
        "rload",
        SignalKind::Scalar,
        SignalUnit::Unspecified,
        SignalValueType::Real,
        SignalShape::Scalar,
        SignalOwner::Analysis,
    )
    .expect("an unspecified unit is a valid scalar descriptor");
    let signal = ResultSignal::new(
        descriptor,
        None,
        SeriesAvailability::Available,
        SeriesValues::Real {
            samples: vec![Some(1.0e3)],
        },
    )
    .expect("signal builds");
    let encoded = serde_json::to_string(&signal).expect("signal serializes");
    let decoded: ResultSignal = serde_json::from_str(&encoded).expect("signal decodes");
    assert_eq!(decoded.descriptor().unit(), &SignalUnit::Unspecified);
    assert_ne!(decoded.descriptor().unit(), &SignalUnit::Dimensionless);
}
