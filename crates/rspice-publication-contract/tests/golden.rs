//! Golden-fixture conformance for the publication interchange format.
//!
//! The two snapshot fixtures are the reference corpus every downstream
//! consumer builds against: the client snapshot builder, the `rspice-publish`
//! renderer, and the `rspice-viewer` hydration runtime. The checked-in JSON
//! is byte-exact: any serialization change, field rename, or enum-encoding
//! drift fails these tests and must ship as a deliberate schema-version bump
//! with regenerated fixtures.
//!
//! Fixture values are literals throughout. Computed transcendentals would
//! tie golden bytes to platform `libm` rounding.
//!
//! To regenerate after an intentional schema change:
//! `cargo test -p rspice-publication-contract --test golden regenerate_fixtures -- --ignored`

use rspice_publication_contract::{
    AnalysisRecord, AxisScale, ContentLicense, Dataset, Disclosure, FIGURE_MANIFEST_SCHEMA_VERSION,
    Figure, FigureContent, FigureManifest, GroupTag, ManifestEntry, ManifestFigureKind,
    Measurement, NetlistSection, PUBLICATION_SNAPSHOT_SCHEMA_VERSION, Paint, PaintRole,
    PathPrimitive, PathSegment, PayloadRef, PlotFigure, PlotHydration, PlotTraceBinding, Point,
    Primitive, PrimitiveGroup, PublicationMetadata, PublicationSnapshot, ResultsSection, Scene,
    SchematicSection, SheetScene, Stroke, StrokePattern, SweepAxis, TextAnchor, TextFont,
    TextPrimitive, Trace, TraceTransform, TraceValues, Validate,
};

const RC_LOWPASS_GOLDEN: &str = include_str!("fixtures/rc-lowpass.json");
const MULTI_ANALYSIS_GOLDEN: &str = include_str!("fixtures/multi-analysis.json");
const FIGURE_MANIFEST_GOLDEN: &str = include_str!("fixtures/figure-manifest.json");

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|v| v.to_bits()).collect()
}

fn stroke(width_um: u64, role: PaintRole) -> Option<Stroke> {
    Some(Stroke {
        width_um,
        paint: Paint::Role(role),
        pattern: StrokePattern::Solid,
    })
}

fn move_to(x_um: i64, y_um: i64) -> PathSegment {
    PathSegment::MoveTo {
        to: Point { x_um, y_um },
    }
}

fn line_to(x_um: i64, y_um: i64) -> PathSegment {
    PathSegment::LineTo {
        to: Point { x_um, y_um },
    }
}

fn label(x_um: i64, y_um: i64, text: &str, font: TextFont, role: PaintRole) -> Primitive {
    Primitive::Text(TextPrimitive {
        origin: Point { x_um, y_um },
        text: text.to_string(),
        height_um: 2_540,
        font,
        anchor: TextAnchor::Start,
        rotation_millideg: 0,
        paint: Paint::Role(role),
    })
}

/// A letter-landscape schematic sheet exercising the full scene vocabulary:
/// tagged groups, arcs, dashed strokes, fills, rotated text, and an explicit
/// RGBA annotation color.
fn rc_lowpass_sheet() -> Scene {
    Scene {
        width_um: 279_400,
        height_um: 215_900,
        groups: vec![
            PrimitiveGroup {
                tag: Some(GroupTag::SheetFrame),
                primitives: vec![Primitive::Path(PathPrimitive {
                    segments: vec![
                        move_to(6_350, 6_350),
                        line_to(273_050, 6_350),
                        line_to(273_050, 209_550),
                        line_to(6_350, 209_550),
                        PathSegment::Close,
                    ],
                    stroke: stroke(254, PaintRole::Foreground),
                    fill: None,
                })],
            },
            PrimitiveGroup {
                tag: Some(GroupTag::Instance {
                    reference: "V1".to_string(),
                }),
                primitives: vec![
                    Primitive::Path(PathPrimitive {
                        segments: vec![PathSegment::Arc {
                            center: Point {
                                x_um: 50_800,
                                y_um: 101_600,
                            },
                            radius_um: 6_350,
                            start_millideg: 0,
                            sweep_millideg: 360_000,
                        }],
                        stroke: stroke(381, PaintRole::Foreground),
                        fill: None,
                    }),
                    label(
                        59_690,
                        96_520,
                        "V1",
                        TextFont::SansSemibold,
                        PaintRole::Foreground,
                    ),
                    label(
                        59_690,
                        100_330,
                        "PULSE",
                        TextFont::Sans,
                        PaintRole::Secondary,
                    ),
                ],
            },
            PrimitiveGroup {
                tag: Some(GroupTag::Instance {
                    reference: "R1".to_string(),
                }),
                primitives: vec![
                    Primitive::Path(PathPrimitive {
                        segments: vec![
                            move_to(88_900, 76_200),
                            line_to(88_900, 71_120),
                            line_to(114_300, 71_120),
                            line_to(114_300, 76_200),
                            PathSegment::Close,
                        ],
                        stroke: stroke(381, PaintRole::Foreground),
                        fill: None,
                    }),
                    label(
                        95_250,
                        66_040,
                        "R1",
                        TextFont::SansSemibold,
                        PaintRole::Foreground,
                    ),
                    label(95_250, 80_010, "1k", TextFont::Sans, PaintRole::Secondary),
                ],
            },
            PrimitiveGroup {
                tag: Some(GroupTag::Instance {
                    reference: "C1".to_string(),
                }),
                primitives: vec![
                    Primitive::Path(PathPrimitive {
                        segments: vec![
                            move_to(146_050, 96_520),
                            line_to(156_210, 96_520),
                            move_to(146_050, 99_060),
                            line_to(156_210, 99_060),
                        ],
                        stroke: stroke(381, PaintRole::Foreground),
                        fill: None,
                    }),
                    Primitive::Text(TextPrimitive {
                        origin: Point {
                            x_um: 160_020,
                            y_um: 97_790,
                        },
                        text: "1µ".to_string(),
                        height_um: 2_540,
                        font: TextFont::Sans,
                        anchor: TextAnchor::Start,
                        rotation_millideg: -90_000,
                        paint: Paint::Role(PaintRole::Secondary),
                    }),
                ],
            },
            PrimitiveGroup {
                tag: Some(GroupTag::Net {
                    name: "out".to_string(),
                }),
                primitives: vec![
                    Primitive::Path(PathPrimitive {
                        segments: vec![move_to(114_300, 73_660), line_to(151_130, 73_660)],
                        stroke: stroke(254, PaintRole::Foreground),
                        fill: None,
                    }),
                    label(
                        129_540,
                        69_850,
                        "out",
                        TextFont::SansSemibold,
                        PaintRole::Foreground,
                    ),
                ],
            },
            PrimitiveGroup {
                tag: Some(GroupTag::Annotation),
                primitives: vec![
                    Primitive::Path(PathPrimitive {
                        segments: vec![
                            move_to(190_500, 63_500),
                            line_to(241_300, 63_500),
                            line_to(241_300, 83_820),
                            line_to(190_500, 83_820),
                            PathSegment::Close,
                        ],
                        stroke: Some(Stroke {
                            width_um: 254,
                            paint: Paint::Rgba([214, 138, 42, 255]),
                            pattern: StrokePattern::Dashed,
                        }),
                        fill: Some(Paint::Rgba([214, 138, 42, 24])),
                    }),
                    Primitive::Text(TextPrimitive {
                        origin: Point {
                            x_um: 193_040,
                            y_um: 71_120,
                        },
                        text: "τ = 1 ms".to_string(),
                        height_um: 2_540,
                        font: TextFont::Sans,
                        anchor: TextAnchor::Start,
                        rotation_millideg: 0,
                        paint: Paint::Rgba([214, 138, 42, 255]),
                    }),
                ],
            },
        ],
    }
}

/// A compact plot scene: frame, one gridline, axis captions, one trace path.
fn plot_scene(x_caption: &str, y_caption: &str, series: u8) -> Scene {
    Scene {
        width_um: 152_400,
        height_um: 101_600,
        groups: vec![
            PrimitiveGroup {
                tag: None,
                primitives: vec![
                    Primitive::Path(PathPrimitive {
                        segments: vec![
                            move_to(15_240, 7_620),
                            line_to(147_320, 7_620),
                            line_to(147_320, 86_360),
                            line_to(15_240, 86_360),
                            PathSegment::Close,
                        ],
                        stroke: stroke(254, PaintRole::Secondary),
                        fill: None,
                    }),
                    Primitive::Path(PathPrimitive {
                        segments: vec![move_to(15_240, 46_990), line_to(147_320, 46_990)],
                        stroke: Some(Stroke {
                            width_um: 127,
                            paint: Paint::Role(PaintRole::Grid),
                            pattern: StrokePattern::Dotted,
                        }),
                        fill: None,
                    }),
                    Primitive::Text(TextPrimitive {
                        origin: Point {
                            x_um: 81_280,
                            y_um: 95_250,
                        },
                        text: x_caption.to_string(),
                        height_um: 2_286,
                        font: TextFont::Monospace,
                        anchor: TextAnchor::Middle,
                        rotation_millideg: 0,
                        paint: Paint::Role(PaintRole::Secondary),
                    }),
                    Primitive::Text(TextPrimitive {
                        origin: Point {
                            x_um: 5_080,
                            y_um: 46_990,
                        },
                        text: y_caption.to_string(),
                        height_um: 2_286,
                        font: TextFont::Monospace,
                        anchor: TextAnchor::Middle,
                        rotation_millideg: 90_000,
                        paint: Paint::Role(PaintRole::Secondary),
                    }),
                ],
            },
            PrimitiveGroup {
                tag: None,
                primitives: vec![Primitive::Path(PathPrimitive {
                    segments: vec![
                        move_to(15_240, 83_820),
                        line_to(48_260, 40_640),
                        line_to(81_280, 21_590),
                        line_to(114_300, 12_700),
                        line_to(147_320, 10_160),
                    ],
                    stroke: Some(Stroke {
                        width_um: 381,
                        paint: Paint::Role(PaintRole::TraceSeries(series)),
                        pattern: StrokePattern::Solid,
                    }),
                    fill: None,
                })],
            },
        ],
    }
}

/// Fixture 1: the smallest complete publication — one sheet, one transient
/// analysis, full disclosure.
fn rc_lowpass_snapshot() -> PublicationSnapshot {
    PublicationSnapshot {
        schema_version: PUBLICATION_SNAPSHOT_SCHEMA_VERSION,
        metadata: PublicationMetadata {
            title: "RC low-pass step response".to_string(),
            description: "First-order RC low-pass driven by a 1 V pulse.\nDemonstrates the 1 ms time constant against the published rise-time spec.".to_string(),
            author_display: "James Whitfield".to_string(),
            app_version: "0.1.0".to_string(),
            created_utc: "2026-08-05T21:00:00Z".to_string(),
            license: ContentLicense::CernOhlP2,
        },
        disclosure: Disclosure {
            schematic: true,
            netlist: true,
            results: true,
            archive: true,
        },
        schematic: Some(SchematicSection {
            sheets: vec![SheetScene {
                name: "top".to_string(),
                page_label: Some("1".to_string()),
                scene: rc_lowpass_sheet(),
            }],
        }),
        netlist: Some(NetlistSection {
            deck: "* RSpice Netlist\n* Components: 3\n* Nets: 2\n\nV1 in 0 PULSE(0 1 0 1u 1u 1m 2m)\nR1 in out 1k\nC1 out 0 1u\n\n* Analysis commands\n.tran 10u 5m\n\n.end".to_string(),
        }),
        results: Some(ResultsSection {
            analyses: vec![AnalysisRecord {
                id: 1,
                label: "Transient".to_string(),
                card: ".tran 10u 5m".to_string(),
            }],
            datasets: vec![Dataset {
                id: 1,
                analysis_id: 1,
                name: "tran1".to_string(),
                variant: None,
                sweep: SweepAxis {
                    label: "time".to_string(),
                    unit: "s".to_string(),
                    values_bits: bits(&[
                        0.0, 0.5e-3, 1.0e-3, 1.5e-3, 2.0e-3, 2.5e-3, 3.0e-3, 3.5e-3, 4.0e-3,
                        4.5e-3, 5.0e-3,
                    ]),
                },
                traces: vec![
                    Trace {
                        label: "V(in)".to_string(),
                        unit: "V".to_string(),
                        values: TraceValues::Real {
                            bits: bits(&[0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0]),
                        },
                    },
                    Trace {
                        label: "V(out)".to_string(),
                        unit: "V".to_string(),
                        values: TraceValues::Real {
                            bits: bits(&[
                                0.0, 0.3935, 0.6321, 0.3834, 0.2325, 0.5347, 0.7178, 0.4353,
                                0.2640, 0.5537, 0.7294,
                            ]),
                        },
                    },
                ],
            }],
            measurements: vec![Measurement {
                analysis_id: 1,
                name: "rise_time".to_string(),
                value_bits: Some(2.2e-3_f64.to_bits()),
                display: "2.20 ms".to_string(),
                spec_display: Some("≤ 2.5 ms".to_string()),
                passed: Some(true),
            }],
        }),
        figures: vec![
            Figure {
                id: 1,
                title: "Top sheet".to_string(),
                content: FigureContent::SchematicSheet { sheet_index: 0 },
            },
            Figure {
                id: 2,
                title: "Step response".to_string(),
                content: FigureContent::Plot(PlotFigure {
                    scene: plot_scene("time (s)", "V", 0),
                    hydration: Some(PlotHydration {
                        x_scale: AxisScale::Linear,
                        y_scale: AxisScale::Linear,
                        x_label: "time (s)".to_string(),
                        y_label: "V".to_string(),
                        bindings: vec![
                            PlotTraceBinding {
                                dataset_id: 1,
                                trace_index: 0,
                                transform: TraceTransform::Identity,
                            },
                            PlotTraceBinding {
                                dataset_id: 1,
                                trace_index: 1,
                                transform: TraceTransform::Identity,
                            },
                        ],
                    }),
                }),
            },
        ],
    }
}

/// Fixture 2: multi-analysis coverage — complex AC data with dB/phase
/// bindings, per-corner dataset variants, a withheld schematic, failing and
/// unevaluated measurements, and a static-only figure.
fn multi_analysis_snapshot() -> PublicationSnapshot {
    PublicationSnapshot {
        schema_version: PUBLICATION_SNAPSHOT_SCHEMA_VERSION,
        metadata: PublicationMetadata {
            title: "Two-stage amplifier verification".to_string(),
            description: "AC response, DC transfer at temperature corners, and load-current transient.\nSchematic withheld; netlist and results disclosed.".to_string(),
            author_display: "James Whitfield".to_string(),
            app_version: "0.1.0".to_string(),
            created_utc: "2026-08-05T21:30:00Z".to_string(),
            license: ContentLicense::AllRightsReserved,
        },
        disclosure: Disclosure {
            schematic: false,
            netlist: true,
            results: true,
            archive: false,
        },
        schematic: None,
        netlist: Some(NetlistSection {
            deck: "* RSpice Netlist\n* Components: 9\n* Nets: 7\n\nVIN in 0 DC 1.2 AC 1\nVDD vdd 0 5\nM1 n1 in 0 0 NFET W=10u L=1u\nRD1 vdd n1 10k\nM2 out n1 0 0 NFET W=20u L=1u\nRD2 vdd out 5k\nCL out 0 2p\nRL out 0 100k\nCC n1 out 1p\n\n.MODEL NFET NMOS (LEVEL=1 VTO=0.7 KP=110u)\n\n* Analysis commands\n.ac dec 1 1 1e6\n.dc VIN 0 2.5 0.5\n.tran 1u 5u\n\n.end".to_string(),
        }),
        results: Some(ResultsSection {
            analyses: vec![
                AnalysisRecord {
                    id: 1,
                    label: "AC response".to_string(),
                    card: ".ac dec 1 1 1e6".to_string(),
                },
                AnalysisRecord {
                    id: 2,
                    label: "DC sweep".to_string(),
                    card: ".dc VIN 0 2.5 0.5".to_string(),
                },
                AnalysisRecord {
                    id: 3,
                    label: "Transient".to_string(),
                    card: ".tran 1u 5u".to_string(),
                },
            ],
            datasets: vec![
                Dataset {
                    id: 1,
                    analysis_id: 1,
                    name: "ac1".to_string(),
                    variant: None,
                    sweep: SweepAxis {
                        label: "frequency".to_string(),
                        unit: "Hz".to_string(),
                        values_bits: bits(&[1.0, 10.0, 100.0, 1.0e3, 1.0e4, 1.0e5, 1.0e6]),
                    },
                    traces: vec![Trace {
                        label: "V(out)".to_string(),
                        unit: "V".to_string(),
                        values: TraceValues::Complex {
                            real_bits: bits(&[
                                39.8, 39.75, 39.2, 35.4, 21.0, 4.7, 0.42,
                            ]),
                            imaginary_bits: bits(&[
                                -0.25, -1.55, -6.1, -16.8, -20.5, -9.6, -1.9,
                            ]),
                        },
                    }],
                },
                Dataset {
                    id: 2,
                    analysis_id: 2,
                    name: "dc1".to_string(),
                    variant: Some("27 °C".to_string()),
                    sweep: SweepAxis {
                        label: "VIN".to_string(),
                        unit: "V".to_string(),
                        values_bits: bits(&[0.0, 0.5, 1.0, 1.5, 2.0, 2.5]),
                    },
                    traces: vec![Trace {
                        label: "V(out)".to_string(),
                        unit: "V".to_string(),
                        values: TraceValues::Real {
                            bits: bits(&[4.98, 4.91, 3.62, 1.08, 0.21, 0.08]),
                        },
                    }],
                },
                Dataset {
                    id: 3,
                    analysis_id: 2,
                    name: "dc1".to_string(),
                    variant: Some("85 °C".to_string()),
                    sweep: SweepAxis {
                        label: "VIN".to_string(),
                        unit: "V".to_string(),
                        values_bits: bits(&[0.0, 0.5, 1.0, 1.5, 2.0, 2.5]),
                    },
                    traces: vec![Trace {
                        label: "V(out)".to_string(),
                        unit: "V".to_string(),
                        values: TraceValues::Real {
                            bits: bits(&[4.95, 4.77, 3.18, 0.86, 0.17, 0.07]),
                        },
                    }],
                },
                Dataset {
                    id: 4,
                    analysis_id: 3,
                    name: "tran1".to_string(),
                    variant: None,
                    sweep: SweepAxis {
                        label: "time".to_string(),
                        unit: "s".to_string(),
                        values_bits: bits(&[0.0, 1.0e-6, 2.0e-6, 3.0e-6, 4.0e-6, 5.0e-6]),
                    },
                    traces: vec![
                        Trace {
                            label: "V(out)".to_string(),
                            unit: "V".to_string(),
                            values: TraceValues::Real {
                                bits: bits(&[2.51, 3.94, 2.48, 1.02, 2.53, 3.96]),
                            },
                        },
                        Trace {
                            label: "I(RL)".to_string(),
                            unit: "A".to_string(),
                            values: TraceValues::Real {
                                bits: bits(&[
                                    2.51e-5, 3.94e-5, 2.48e-5, 1.02e-5, 2.53e-5, 3.96e-5,
                                ]),
                            },
                        },
                    ],
                },
            ],
            measurements: vec![
                Measurement {
                    analysis_id: 1,
                    name: "gbw".to_string(),
                    value_bits: Some(3.2e6_f64.to_bits()),
                    display: "3.2 MHz".to_string(),
                    spec_display: Some("≥ 1 MHz".to_string()),
                    passed: Some(true),
                },
                Measurement {
                    analysis_id: 2,
                    name: "offset".to_string(),
                    value_bits: Some(0.012_f64.to_bits()),
                    display: "12 mV".to_string(),
                    spec_display: Some("≤ 5 mV".to_string()),
                    passed: Some(false),
                },
                Measurement {
                    analysis_id: 3,
                    name: "settling".to_string(),
                    value_bits: None,
                    display: "not computed".to_string(),
                    spec_display: None,
                    passed: None,
                },
            ],
        }),
        figures: vec![
            Figure {
                id: 1,
                title: "Bode magnitude".to_string(),
                content: FigureContent::Plot(PlotFigure {
                    scene: plot_scene("frequency (Hz)", "dB", 0),
                    hydration: Some(PlotHydration {
                        x_scale: AxisScale::Logarithmic,
                        y_scale: AxisScale::Linear,
                        x_label: "frequency (Hz)".to_string(),
                        y_label: "dB".to_string(),
                        bindings: vec![PlotTraceBinding {
                            dataset_id: 1,
                            trace_index: 0,
                            transform: TraceTransform::MagnitudeDb,
                        }],
                    }),
                }),
            },
            Figure {
                id: 2,
                title: "Bode phase".to_string(),
                content: FigureContent::Plot(PlotFigure {
                    scene: plot_scene("frequency (Hz)", "°", 1),
                    hydration: Some(PlotHydration {
                        x_scale: AxisScale::Logarithmic,
                        y_scale: AxisScale::Linear,
                        x_label: "frequency (Hz)".to_string(),
                        y_label: "°".to_string(),
                        bindings: vec![PlotTraceBinding {
                            dataset_id: 1,
                            trace_index: 0,
                            transform: TraceTransform::PhaseDegrees,
                        }],
                    }),
                }),
            },
            Figure {
                id: 3,
                title: "DC transfer at corners".to_string(),
                content: FigureContent::Plot(PlotFigure {
                    scene: plot_scene("VIN (V)", "V(out)", 2),
                    hydration: Some(PlotHydration {
                        x_scale: AxisScale::Linear,
                        y_scale: AxisScale::Linear,
                        x_label: "VIN (V)".to_string(),
                        y_label: "V(out)".to_string(),
                        bindings: vec![
                            PlotTraceBinding {
                                dataset_id: 2,
                                trace_index: 0,
                                transform: TraceTransform::Identity,
                            },
                            PlotTraceBinding {
                                dataset_id: 3,
                                trace_index: 0,
                                transform: TraceTransform::Identity,
                            },
                        ],
                    }),
                }),
            },
            Figure {
                id: 4,
                title: "Load current".to_string(),
                content: FigureContent::Plot(PlotFigure {
                    scene: plot_scene("time (s)", "A", 3),
                    hydration: None,
                }),
            },
        ],
    }
}

/// The manifest `rspice-publish` would emit for the RC fixture's figures.
fn rc_lowpass_manifest() -> FigureManifest {
    FigureManifest {
        schema_version: FIGURE_MANIFEST_SCHEMA_VERSION,
        figures: vec![
            ManifestEntry {
                figure_id: 1,
                dom_id: "figure-1".to_string(),
                kind: ManifestFigureKind::SchematicSheet,
                payload: PayloadRef {
                    path: "figures/1.json".to_string(),
                    sha256_hex: "3f3b6f0f2f6a4d8c9e1b7a5c0d2e4f6a8b0c2d4e6f8a0b2c4d6e8f0a1b3c5d7e"
                        .to_string(),
                    byte_len: 18_432,
                },
            },
            ManifestEntry {
                figure_id: 2,
                dom_id: "figure-2".to_string(),
                kind: ManifestFigureKind::Plot,
                payload: PayloadRef {
                    path: "figures/2.json".to_string(),
                    sha256_hex: "9c8d7e6f5a4b3c2d1e0f9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d"
                        .to_string(),
                    byte_len: 4_096,
                },
            },
        ],
    }
}

fn pretty(value: &impl serde::Serialize) -> String {
    let mut text = serde_json::to_string_pretty(value).expect("pretty serialization");
    text.push('\n');
    text
}

#[test]
fn rc_lowpass_fixture_matches_golden_bytes() {
    let snapshot = rc_lowpass_snapshot();
    snapshot.validate().expect("fixture must validate");
    assert_eq!(pretty(&snapshot), RC_LOWPASS_GOLDEN);
}

#[test]
fn multi_analysis_fixture_matches_golden_bytes() {
    let snapshot = multi_analysis_snapshot();
    snapshot.validate().expect("fixture must validate");
    assert_eq!(pretty(&snapshot), MULTI_ANALYSIS_GOLDEN);
}

#[test]
fn figure_manifest_fixture_matches_golden_bytes() {
    let manifest = rc_lowpass_manifest();
    manifest.validate().expect("fixture must validate");
    assert_eq!(pretty(&manifest), FIGURE_MANIFEST_GOLDEN);
}

#[test]
fn golden_snapshots_round_trip_through_canonical_bytes() {
    for (name, golden, snapshot) in [
        ("rc-lowpass", RC_LOWPASS_GOLDEN, rc_lowpass_snapshot()),
        (
            "multi-analysis",
            MULTI_ANALYSIS_GOLDEN,
            multi_analysis_snapshot(),
        ),
    ] {
        let from_golden = PublicationSnapshot::from_canonical_bytes(golden.as_bytes())
            .unwrap_or_else(|error| panic!("{name} golden must parse: {error}"));
        assert_eq!(
            from_golden, snapshot,
            "{name} golden must equal its builder"
        );

        let canonical = snapshot.canonical_bytes().expect("canonical bytes");
        let reparsed = PublicationSnapshot::from_canonical_bytes(&canonical).expect("reparse");
        assert_eq!(reparsed, snapshot, "{name} must round-trip");
        assert_eq!(
            canonical,
            reparsed.canonical_bytes().expect("second serialization"),
            "{name} canonical bytes must be deterministic"
        );
    }
}

#[test]
fn golden_manifest_round_trips_through_canonical_bytes() {
    let manifest = rc_lowpass_manifest();
    let from_golden = FigureManifest::from_canonical_bytes(FIGURE_MANIFEST_GOLDEN.as_bytes())
        .expect("manifest golden must parse");
    assert_eq!(from_golden, manifest);
    let canonical = manifest.canonical_bytes().expect("canonical bytes");
    assert_eq!(
        FigureManifest::from_canonical_bytes(&canonical).expect("reparse"),
        manifest
    );
}

/// Rewrites the golden files from the builders. Run only after a deliberate
/// schema change, then review the diff like any other contract change:
/// `cargo test -p rspice-publication-contract --test golden regenerate_fixtures -- --ignored`
#[test]
#[ignore = "regenerates golden fixtures; run explicitly after a deliberate schema change"]
fn regenerate_fixtures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    std::fs::create_dir_all(&root).expect("fixture directory");
    std::fs::write(root.join("rc-lowpass.json"), pretty(&rc_lowpass_snapshot()))
        .expect("write rc-lowpass fixture");
    std::fs::write(
        root.join("multi-analysis.json"),
        pretty(&multi_analysis_snapshot()),
    )
    .expect("write multi-analysis fixture");
    std::fs::write(
        root.join("figure-manifest.json"),
        pretty(&rc_lowpass_manifest()),
    )
    .expect("write figure-manifest fixture");
}
