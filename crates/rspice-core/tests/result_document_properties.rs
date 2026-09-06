//! Round-trip laws for the shared typed result document.
//!
//! The law under test is that JSON is a *lossless* representation of an
//! [`AnalysisResultDocument`]: encoding a valid document and decoding the
//! result reproduces it exactly, for every result family, including explicit
//! missingness (`None` samples), availability provenance, units, device state
//! and payload metadata. Rule 7 of the engineering contract — results are
//! immutable evidence — is exactly this law, so it is stated over generated
//! documents rather than over one hand-written example per family.
//!
//! Two more laws ride along, because they are properties of the same
//! encode/decode pair: a document whose declared byte budget is smaller than
//! its encoding fails with the typed `ArtifactTooLarge` rather than a truncated
//! artifact, and a decoder rejects a foreign schema or version before it
//! decodes any field.
//!
//! Generation is seeded and failure persistence is off, so every case is
//! reproducible from this source alone.

use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

use rspice_core::abort_signal::NoAbort;
use rspice_core::execution::result_document::{
    ANALYSIS_RESULT_DOCUMENT_VERSION, AcPayload, AnalysisResultDocument, AxisValues, ComplexSample,
    DcSweepAxisDocument, DcSweepPayload, DeviceParameterSeries, DeviceStateSeries, DigitalStateTag,
    DigitalStrengthTag, DistortionPayload, DistortionProductSeries, DistortionProductTag,
    EnvelopeCarrierDocument, EnvelopeContinuationDocument, EnvelopeGuaranteeTag,
    EnvelopeNodeSpectrum, EnvelopePayload, FftCoefficientFormatTag, FftCompatibilityModeTag,
    FftHarmonicDocument, FftMetricsDocument, FftPayload, FftSourceDocument, FftWindowTag,
    FloquetEvidenceDocument, FloquetOrbitTag, FourierPayload, HarmonicBalancePayload,
    HbReactiveKindTag, HbReactiveSpectrumDocument, LogicSample, MonteCarloPayload,
    MonteCarloVariableStatistics, NamedObservable, NamedObservableSeries, NoiseContributionSeries,
    NoiseMechanismTag, NoisePayload, NoiseSourceIdentityDocument, NyquistSample,
    OperatingPointPayload, PNoiseBandwidth, PNoiseContribution, PNoiseContributor, PNoisePayload,
    PacPayload, PacSidebandDescriptor, PoleZeroPayload, PortDocument,
    PortNoiseCovarianceNormalization, PortNoisePayload, PssPayload, ResultAxis, ResultAxisKind,
    ResultDocumentError, ResultNamespaces, ResultPayload, ResultScalar, ResultSignal,
    RootSetEvidenceDocument, SParameterPayload, ScalarValue, SensitivityElementTag,
    SensitivityEntry, SensitivityPayload, SeriesAvailability, SeriesValues, StabilityPayload,
    TransferFunctionPayload, TransientPayload,
};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisRequest, DeckPlan, SignalDescriptor, SignalKind,
    SignalOwner, SignalShape, SignalUnit, SignalValueType,
};

fn runner(seed: u64, cases: u32) -> TestRunner {
    let mut entropy = [0_u8; 32];
    entropy[..8].copy_from_slice(&seed.to_le_bytes());
    TestRunner::new_with_rng(
        Config {
            cases,
            failure_persistence: None,
            ..Config::default()
        },
        TestRng::from_seed(RngAlgorithm::ChaCha, &entropy),
    )
}

/// Mint a real planner analysis identity.
///
/// `AnalysisInstanceId` has no public constructor: only the planner names an
/// analysis. A document fixture therefore asks the planner for the identity
/// instead of inventing a parallel naming scheme.
fn analysis_id(kind: AnalysisKind) -> AnalysisInstanceId {
    let requests = if kind == AnalysisKind::ImplicitOp {
        Vec::new()
    } else {
        vec![AnalysisRequest::new(kind)]
    };
    DeckPlan::new(Vec::new(), requests)
        .expect("a plan with no axes and one analysis is valid")
        .analyses()
        .first()
        .expect("a plan always has at least one planned analysis")
        .id()
}

/// The generated content one document carries, independent of its family.
#[derive(Debug, Clone)]
struct Shape {
    point_count: usize,
    /// Sample presence mask, one entry per point, shared by every series so
    /// missingness is exercised in the same places.
    present: Vec<bool>,
    magnitudes: Vec<f64>,
    include_device_state: bool,
    include_namespaces: bool,
}

fn shape() -> impl Strategy<Value = Shape> {
    (1_usize..4, any::<bool>(), any::<bool>()).prop_flat_map(
        |(point_count, include_device_state, include_namespaces)| {
            (
                prop::collection::vec(any::<bool>(), point_count..=point_count),
                prop::collection::vec(-1e6_f64..1e6, point_count..=point_count),
            )
                .prop_map(move |(present, magnitudes)| Shape {
                    point_count,
                    present,
                    magnitudes,
                    include_device_state,
                    include_namespaces,
                })
        },
    )
}

impl Shape {
    fn reals(&self) -> Vec<Option<f64>> {
        self.present
            .iter()
            .zip(&self.magnitudes)
            .map(|(present, value)| present.then_some(*value))
            .collect()
    }

    fn finite_reals(&self) -> Vec<f64> {
        self.magnitudes.clone()
    }

    fn complexes(&self) -> Vec<Option<ComplexSample>> {
        self.present
            .iter()
            .zip(&self.magnitudes)
            .map(|(present, value)| present.then(|| ComplexSample::new(*value, -*value * 0.5)))
            .collect()
    }

    fn logics(&self) -> Vec<Option<LogicSample>> {
        self.present
            .iter()
            .map(|present| {
                present.then_some(LogicSample {
                    state: DigitalStateTag::OneResistive,
                    strength: DigitalStrengthTag::Resistive,
                })
            })
            .collect()
    }

    /// Strictly increasing, strictly positive axis coordinates.
    fn axis_values(&self) -> Vec<f64> {
        (0..self.point_count)
            .map(|index| index as f64 + 1.0)
            .collect()
    }

    fn complex_vector(&self) -> Vec<ComplexSample> {
        self.magnitudes
            .iter()
            .map(|value| ComplexSample::new(*value, *value * 0.25))
            .collect()
    }
}

fn voltage_descriptor(name: &str, value_type: SignalValueType) -> SignalDescriptor {
    SignalDescriptor::new(
        format!("v({name})"),
        format!("V({name})"),
        SignalKind::Voltage,
        SignalUnit::Volt,
        value_type,
        SignalShape::Vector,
        SignalOwner::Node(name.to_owned()),
    )
    .expect("a node voltage descriptor is structurally valid")
}

fn digital_descriptor(name: &str) -> SignalDescriptor {
    SignalDescriptor::new(
        format!("d({name})"),
        format!("D({name})"),
        SignalKind::Digital,
        SignalUnit::Logic,
        SignalValueType::Logic,
        SignalShape::Vector,
        SignalOwner::Node(name.to_owned()),
    )
    .expect("a digital node descriptor is structurally valid")
}

/// Every series shape the document can carry, including a deliberately
/// unretained one whose descriptor is still evidence that the signal exists.
fn signals(shape: &Shape) -> Vec<ResultSignal> {
    vec![
        ResultSignal::new(
            voltage_descriptor("out", SignalValueType::Real),
            None,
            SeriesAvailability::Available,
            SeriesValues::Real {
                samples: shape.reals(),
            },
        )
        .expect("a real voltage series is valid"),
        ResultSignal::new(
            voltage_descriptor("mid", SignalValueType::Complex),
            None,
            SeriesAvailability::Available,
            SeriesValues::Complex {
                samples: shape.complexes(),
            },
        )
        .expect("a complex voltage series is valid"),
        ResultSignal::new(
            digital_descriptor("clk"),
            None,
            SeriesAvailability::Available,
            SeriesValues::Logic {
                samples: shape.logics(),
            },
        )
        .expect("a logic series is valid"),
        ResultSignal::new(
            voltage_descriptor("dropped", SignalValueType::Real),
            None,
            SeriesAvailability::NotProjected,
            SeriesValues::Real {
                samples: vec![None; shape.point_count],
            },
        )
        .expect("an unretained series carries no samples"),
        ResultSignal::new(
            voltage_descriptor("gone", SignalValueType::Real),
            None,
            SeriesAvailability::AbsentAtCoordinate,
            SeriesValues::Real {
                samples: vec![None; shape.point_count],
            },
        )
        .expect("a coordinate-absent series carries no samples"),
    ]
}

/// One scalar per representation, so no encoding branch is untested.
fn scalars(shape: &Shape) -> Vec<ResultScalar> {
    let magnitude = shape.magnitudes[0];
    vec![
        ResultScalar::new(
            "residual",
            "Residual",
            Some(SignalUnit::Dimensionless),
            ScalarValue::Real {
                value: Some(magnitude),
            },
        )
        .expect("a real scalar is valid"),
        ResultScalar::new(
            "undefined",
            "Undefined",
            None,
            ScalarValue::Real { value: None },
        )
        .expect("an absent real scalar is valid"),
        ResultScalar::new(
            "gain",
            "Gain",
            Some(SignalUnit::Custom("V/V".to_owned())),
            ScalarValue::Complex {
                value: Some(ComplexSample::new(magnitude, -magnitude)),
            },
        )
        .expect("a complex scalar is valid"),
        ResultScalar::new("order", "Order", None, ScalarValue::Integer { value: -7 })
            .expect("an integer scalar is valid"),
        ResultScalar::new(
            "iterations",
            "Iterations",
            None,
            ScalarValue::Count { value: 42 },
        )
        .expect("a count scalar is valid"),
        ResultScalar::new(
            "converged",
            "Converged",
            None,
            ScalarValue::Boolean { value: true },
        )
        .expect("a boolean scalar is valid"),
        ResultScalar::new(
            "method",
            "Method",
            None,
            ScalarValue::Text {
                value: "gear2".to_owned(),
            },
        )
        .expect("a text scalar is valid"),
    ]
}

fn device_states(shape: &Shape) -> Vec<DeviceStateSeries> {
    vec![
        DeviceStateSeries::new(
            "m1",
            Some("nmos".to_owned()),
            shape
                .present
                .iter()
                .map(|present| present.then(|| "saturation".to_owned()))
                .collect(),
            vec![DeviceParameterSeries {
                name: "gm".to_owned(),
                unit: Some(SignalUnit::Siemens),
                values: shape.reals(),
            }],
        )
        .expect("a device state history is valid"),
    ]
}

fn transient_payload(shape: &Shape) -> TransientPayload {
    let mut step_sizes = vec![0.0];
    step_sizes.extend((1..shape.point_count).map(|index| index as f64 * 1e-9 + 1e-9));
    TransientPayload {
        step_sizes,
        store_traces: vec![NamedObservableSeries {
            name: "r1:r".to_owned(),
            unit: Some(SignalUnit::Ohm),
            values: shape.reals(),
        }],
        digital_traces: Vec::new(),
        digital_buses: Vec::new(),
        real_traces: Vec::new(),
        fft_children: Vec::new(),
        compression: None,
    }
}

/// A valid document for one result family, filled with the generated content.
fn document(family: usize, shape: &Shape) -> AnalysisResultDocument {
    let axis_values = AxisValues::Real {
        values: shape.axis_values(),
    };
    let integer_axis = AxisValues::Integer {
        values: (0..shape.point_count as i64).collect(),
    };

    let (analysis, parent, axis_kind, axis, payload) = match family {
        0 => (
            AnalysisKind::Op,
            None,
            ResultAxisKind::Index,
            integer_axis.clone(),
            ResultPayload::Op(OperatingPointPayload {
                observables: vec![NamedObservable {
                    name: "v(out)".to_owned(),
                    unit: Some(SignalUnit::Volt),
                    value: Some(shape.magnitudes[0]),
                }],
            }),
        ),
        1 => (
            AnalysisKind::Dc,
            None,
            ResultAxisKind::SweepValue,
            axis_values.clone(),
            ResultPayload::Dc(DcSweepPayload {
                sweep_variables: vec![DcSweepAxisDocument {
                    name: "v1".to_owned(),
                    unit: SignalUnit::Volt,
                    value_count: axis_values.len(),
                }],
                observables: vec![NamedObservableSeries {
                    name: "v(out)".to_owned(),
                    unit: Some(SignalUnit::Volt),
                    values: shape.reals(),
                }],
            }),
        ),
        2 => (
            AnalysisKind::Ac,
            None,
            ResultAxisKind::Frequency,
            axis_values.clone(),
            ResultPayload::Ac(AcPayload {}),
        ),
        3 => (
            AnalysisKind::Tran,
            None,
            ResultAxisKind::Time,
            axis_values.clone(),
            ResultPayload::Tran(transient_payload(shape)),
        ),
        4 => (
            AnalysisKind::Noise,
            None,
            ResultAxisKind::Frequency,
            axis_values.clone(),
            ResultPayload::Noise(NoisePayload {
                contribution_catalog: vec![NoiseSourceIdentityDocument {
                    device: "r1".to_owned(),
                    mechanism: Some("thermal".to_owned()),
                }],
                mechanisms_unavailable: vec!["q1".to_owned()],
                contributions: vec![NoiseContributionSeries {
                    identity: NoiseSourceIdentityDocument {
                        device: "r1".to_owned(),
                        mechanism: Some("thermal".to_owned()),
                    },
                    mechanism_kind: NoiseMechanismTag::Thermal,
                    output_contribution: shape.reals(),
                    input_contribution: shape.reals(),
                    percentage: shape.reals(),
                }],
            }),
        ),
        5 => (
            AnalysisKind::Sp,
            None,
            ResultAxisKind::Frequency,
            axis_values.clone(),
            ResultPayload::Sp(SParameterPayload {
                reference_impedance: 50.0,
                ports: vec![PortDocument {
                    number: 1,
                    node_positive: "in".to_owned(),
                    node_negative: "0".to_owned(),
                    reference_impedance: 50.0,
                }],
                angular_frequencies: shape.finite_reals(),
            }),
        ),
        6 => (
            AnalysisKind::Sp,
            None,
            ResultAxisKind::PortIndex,
            integer_axis.clone(),
            ResultPayload::PortNoise(PortNoisePayload {
                port_count: 2,
                reference_temperature_kelvin: 300.15,
                covariance_normalization: PortNoiseCovarianceNormalization::AmpereSquaredPerHertz,
                thermal_normalization_joule: 4.0 * 1.380_649e-23 * 300.15,
                two_port: Vec::new(),
            }),
        ),
        7 => (
            AnalysisKind::Distortion,
            None,
            ResultAxisKind::Frequency,
            axis_values.clone(),
            ResultPayload::Distortion(DistortionPayload {
                f2_over_f1: Some(0.9),
                products: vec![DistortionProductSeries {
                    product: DistortionProductTag::ThirdOrderDifference,
                    order: DistortionProductTag::ThirdOrderDifference.order(),
                    frequencies: shape.finite_reals(),
                }],
            }),
        ),
        8 => (
            AnalysisKind::TransferFunction,
            None,
            ResultAxisKind::Index,
            integer_axis.clone(),
            ResultPayload::Tf(TransferFunctionPayload {
                output: "v(out)".to_owned(),
                input: "v1".to_owned(),
            }),
        ),
        9 => (
            AnalysisKind::Stb,
            None,
            ResultAxisKind::Frequency,
            axis_values.clone(),
            ResultPayload::Stb(StabilityPayload {
                success: true,
                warnings: vec!["probe orientation assumed".to_owned()],
                nyquist: shape
                    .magnitudes
                    .iter()
                    .enumerate()
                    .map(|(index, value)| NyquistSample {
                        frequency: index as f64 + 1.0,
                        real: *value,
                        imaginary: -*value,
                    })
                    .collect(),
            }),
        ),
        10 => (
            AnalysisKind::Sensitivity,
            None,
            ResultAxisKind::Index,
            integer_axis.clone(),
            ResultPayload::Sensitivity(SensitivityPayload {
                output: "v(out)".to_owned(),
                ac_entries: Vec::new(),
                entries: vec![SensitivityEntry {
                    vector_name: "v(out)".to_owned(),
                    element: "r1".to_owned(),
                    element_kind: SensitivityElementTag::Resistor,
                    parameter: "r".to_owned(),
                    nominal_value: 1000.0,
                    absolute: shape.magnitudes[0],
                    normalized: shape.magnitudes[0] * 0.001,
                }],
            }),
        ),
        11 => (
            AnalysisKind::PoleZero,
            None,
            ResultAxisKind::Index,
            integer_axis.clone(),
            ResultPayload::PoleZero(PoleZeroPayload {
                input: "v1".to_owned(),
                output: "v(out)".to_owned(),
                poles: Vec::new(),
                zeros: Vec::new(),
                pole_evidence: RootSetEvidenceDocument::NotRequested,
                zero_evidence: RootSetEvidenceDocument::NotRequested,
                dc_gain: Some(shape.magnitudes[0]),
                high_frequency_gain: None,
            }),
        ),
        12 => (
            AnalysisKind::Fourier,
            Some(analysis_id(AnalysisKind::Tran)),
            ResultAxisKind::HarmonicIndex,
            integer_axis.clone(),
            ResultPayload::Fourier(FourierPayload {
                output: "v(out)".to_owned(),
            }),
        ),
        13 => (
            AnalysisKind::Fft,
            Some(analysis_id(AnalysisKind::Tran)),
            ResultAxisKind::BinIndex,
            integer_axis.clone(),
            ResultPayload::Fft(FftPayload {
                source: FftSourceDocument::Probe {
                    text: "V(out)".to_owned(),
                },
                output_name: "V(out)".to_owned(),
                physical_type: "voltage".to_owned(),
                start_time: 0.0,
                stop_time: 1e-3,
                sample_interval: 1e-6,
                sample_count: 1024,
                accurate_sampling: true,
                coefficient_format: FftCoefficientFormatTag::Normalized,
                compatibility_mode: FftCompatibilityModeTag::HspiceCompatible,
                window: FftWindowTag::Hann,
                window_name: "hann".to_owned(),
                alpha: 3.0,
                coherent_gain: 0.5,
                frequency_resolution: 1000.0,
                fundamental_bin: 1,
                minimum_metric_bin: 1,
                maximum_metric_bin: 512,
                metrics: Some(FftMetricsDocument {
                    fundamental_magnitude: shape.magnitudes[0].abs(),
                    thd_ratio: 0.01,
                    thd_db: -40.0,
                    sndr_db: 60.0,
                    enob_bits: 9.7,
                    snr_db: 61.0,
                    sfdr_db: 55.0,
                    sfdr_spur_bin: Some(3),
                    sfdr_spur_frequency: Some(3000.0),
                    largest_harmonics: vec![FftHarmonicDocument {
                        rank: 1,
                        bin: 2,
                        frequency: 2000.0,
                        magnitude: 0.01,
                        magnitude_db: -40.0,
                        phase_degrees: 12.5,
                    }],
                }),
            }),
        ),
        14 => (
            AnalysisKind::MonteCarlo,
            None,
            ResultAxisKind::TrialIndex,
            integer_axis.clone(),
            ResultPayload::MonteCarlo(MonteCarloPayload {
                statistics: vec![MonteCarloVariableStatistics {
                    name: "v(out)".to_owned(),
                    samples: shape.reals(),
                    mean: Some(shape.magnitudes[0]),
                    standard_deviation: None,
                    minimum: Some(shape.magnitudes[0]),
                    maximum: Some(shape.magnitudes[0]),
                    histogram: vec![1, 0],
                    bin_edges: vec![0.0, 1.0, 2.0],
                }],
            }),
        ),
        15 => (
            AnalysisKind::Pss,
            None,
            ResultAxisKind::HarmonicIndex,
            integer_axis.clone(),
            ResultPayload::Pss(PssPayload {
                floquet_multipliers: shape.complex_vector(),
                floquet_evidence: FloquetEvidenceDocument::NotComputed,
                floquet_orbit_kind: FloquetOrbitTag::Driven,
                trivial_floquet_multiplier_index: Some(0),
            }),
        ),
        16 => (
            AnalysisKind::Pac,
            Some(analysis_id(AnalysisKind::Pss)),
            ResultAxisKind::Sideband,
            integer_axis.clone(),
            ResultPayload::Pac(PacPayload {
                fundamental_frequency: 1e9,
                sideband_minimum: -1,
                sideband_maximum: 1,
                input_source: Some("v1".to_owned()),
                output_node: Some("out".to_owned()),
                iterations: 7,
                residual: 1e-12,
                sidebands: vec![PacSidebandDescriptor {
                    sideband: 1,
                    frequency_offsets: shape.finite_reals(),
                    absolute_frequencies: shape.finite_reals(),
                }],
                conversion_matrix: None,
            }),
        ),
        17 => (
            AnalysisKind::PNoise,
            Some(analysis_id(AnalysisKind::Pss)),
            ResultAxisKind::OffsetFrequency,
            axis_values.clone(),
            ResultPayload::PNoise(PNoisePayload {
                output_node: "out".to_owned(),
                oscillator: None,
                jitter_bandwidth: Some(PNoiseBandwidth {
                    start: 1.0,
                    stop: 1e6,
                }),
                contributors: vec![PNoiseContributor {
                    name: "m1".to_owned(),
                    device_type: "mosfet".to_owned(),
                    contributions: vec![PNoiseContribution {
                        offset_frequency: 1e3,
                        contribution_dbc_per_hz: -120.0,
                    }],
                    percentage: Some(42.0),
                }],
            }),
        ),
        18 => (
            AnalysisKind::HarmonicBalance,
            None,
            ResultAxisKind::HarmonicIndex,
            integer_axis.clone(),
            ResultPayload::Hb(HarmonicBalancePayload {
                tones: vec!["1e9".to_owned()],
                reactive_spectra: vec![HbReactiveSpectrumDocument {
                    device_name: "c1".to_owned(),
                    kind: HbReactiveKindTag::Capacitor,
                    voltage_coefficients: shape.complex_vector(),
                    current_coefficients: shape.complex_vector(),
                    dc_current_is_exact: true,
                }],
                continuation_limitations: Vec::new(),
            }),
        ),
        _ => (
            AnalysisKind::Envelope,
            Some(analysis_id(AnalysisKind::HarmonicBalance)),
            ResultAxisKind::Time,
            axis_values.clone(),
            ResultPayload::Envelope(EnvelopePayload {
                continuation: EnvelopeContinuationDocument {
                    guarantee: EnvelopeGuaranteeTag::ExactLinearRcMnaV1,
                    carrier_fundamental_frequency: 1e9,
                    carrier_harmonics: 3,
                    hb_config_identity: "hb-1".to_owned(),
                    canonical_frozen_sources: vec!["v1".to_owned()],
                    original_netlist_identity: "deck-1".to_owned(),
                    resolved_simulation_identity: "sim-1".to_owned(),
                    history_step: 1e-9,
                    time_origin: 0.0,
                    slow_time_duration: 1e-6,
                    slow_time_max_step: 1e-8,
                },
                carrier: EnvelopeCarrierDocument {
                    converged: true,
                    iterations: 11,
                    residual_norm: 1e-10,
                    fundamental_frequency: 1e9,
                    harmonic_frequencies: shape.axis_values(),
                    node_spectra: vec![EnvelopeNodeSpectrum {
                        node_name: "out".to_owned(),
                        coefficients: shape.complex_vector(),
                    }],
                },
                transient: transient_payload(shape),
            }),
        ),
    };

    let mut builder =
        AnalysisResultDocument::builder(analysis_id(analysis), payload, shape.point_count)
            .axis(
                ResultAxis::new("x", "X", axis_kind, SignalUnit::Dimensionless, axis)
                    .expect("a generated axis is valid"),
            )
            .signals(signals(shape))
            .scalars(scalars(shape));
    if let Some(parent) = parent {
        builder = builder.parent_analysis(parent);
    }
    if shape.include_device_state {
        builder = builder.device_states(device_states(shape));
    }
    if shape.include_namespaces {
        builder = builder.namespaces(ResultNamespaces {
            output: "run-1/tran-001".to_owned(),
            checkpoint: "run-1/ckpt".to_owned(),
        });
    }
    builder.build().expect("a generated document is valid")
}

/// Number of families the fixture covers; one per [`ResultPayload`] variant.
const FAMILY_COUNT: usize = 20;

#[test]
fn law_every_result_family_round_trips_through_json_exactly() {
    runner(0x0D0C_0001, 48)
        .run(&shape(), |shape| {
            for family in 0..FAMILY_COUNT {
                let document = document(family, &shape);
                let json = document.to_json().expect("a valid document encodes");
                let decoded = AnalysisResultDocument::from_json(&json)
                    .expect("a document this build wrote decodes");
                prop_assert_eq!(&decoded, &document, "family {} lost information", family);
                let reencoded = decoded.to_json().expect("a decoded document re-encodes");
                prop_assert_eq!(reencoded, json, "family {} is not byte-stable", family);
            }
            Ok(())
        })
        .expect("JSON is a lossless representation of every result family");
}

#[test]
fn law_missingness_survives_the_round_trip_as_absence() {
    runner(0x0D0C_0002, 48)
        .run(&shape(), |shape| {
            let document = document(3, &shape);
            let decoded = AnalysisResultDocument::from_json(
                &document.to_json().expect("a valid document encodes"),
            )
            .expect("a document this build wrote decodes");
            for (original, decoded) in document.signals().iter().zip(decoded.signals()) {
                prop_assert_eq!(original.availability(), decoded.availability());
                match (original.values(), decoded.values()) {
                    (
                        SeriesValues::Real { samples: left },
                        SeriesValues::Real { samples: right },
                    ) => {
                        for (left, right) in left.iter().zip(right) {
                            // A missing sample must never come back as 0.0.
                            prop_assert_eq!(left.is_none(), right.is_none());
                            prop_assert_eq!(left, right);
                        }
                    }
                    (left, right) => prop_assert_eq!(left, right),
                }
            }
            Ok(())
        })
        .expect("an absent sample decodes as absent, never as a value");
}

#[test]
fn law_a_budget_smaller_than_the_encoding_is_a_typed_refusal() {
    runner(0x0D0C_0003, 32)
        .run(&shape(), |shape| {
            let document = document(0, &shape);
            let json = document.to_json().expect("a valid document encodes");
            let budget = (json.len() - 1) as u64;
            let refused = document.to_json_with_abort(&NoAbort, budget);
            prop_assert!(
                matches!(refused, Err(ResultDocumentError::ArtifactTooLarge { .. })),
                "encoding past the budget must be a typed refusal"
            );
            // The same budget refuses the decode before any field is parsed.
            let refused = AnalysisResultDocument::from_json_with_abort(&json, &NoAbort, budget);
            prop_assert!(
                matches!(refused, Err(ResultDocumentError::ArtifactTooLarge { .. })),
                "decoding past the budget must be a typed refusal"
            );
            Ok(())
        })
        .expect("an artifact that does not fit its budget is refused, never truncated");
}

#[test]
fn law_a_foreign_schema_version_is_refused_before_any_field_is_decoded() {
    runner(0x0D0C_0004, 32)
        .run(&(shape(), 1_u32..64), |(shape, bump)| {
            let json = document(2, &shape)
                .to_json()
                .expect("a valid document encodes");
            let future = json.replacen(
                &format!("\"schemaVersion\":{ANALYSIS_RESULT_DOCUMENT_VERSION}"),
                &format!(
                    "\"schemaVersion\":{}",
                    ANALYSIS_RESULT_DOCUMENT_VERSION + bump
                ),
                1,
            );
            prop_assert_ne!(
                &future,
                &json,
                "the fixture must actually change the version"
            );
            prop_assert!(
                matches!(
                    AnalysisResultDocument::from_json(&future),
                    Err(ResultDocumentError::UnsupportedVersion { .. })
                ),
                "a future schema version must be refused by version, not by field"
            );

            let foreign = json.replacen(
                "\"schema\":\"rspice-analysis-result\"",
                "\"schema\":\"some-other-tool\"",
                1,
            );
            prop_assert!(
                matches!(
                    AnalysisResultDocument::from_json(&foreign),
                    Err(ResultDocumentError::WrongSchema { .. })
                ),
                "a foreign schema identity must be refused by identity"
            );
            Ok(())
        })
        .expect("schema identity and version gate the decoder");
}
