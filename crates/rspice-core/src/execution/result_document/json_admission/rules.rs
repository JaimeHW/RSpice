//! Wire rules for the document's retained-value accounting policy.
use super::Rule::{self, Array, Impulse, Items, Numbers, Object, Payload, Series, Skip};
use crate::execution::AnalysisResultKind;

pub(super) const REAL_SERIES: Rule = Object(0, &[("samples", Items(1))]);
pub(super) const COMPLEX_SERIES: Rule = Object(0, &[("samples", Items(2))]);
const VALUES: Rule = Object(0, &[("values", Items(1))]);

pub(super) const DOCUMENT: Rule = Object(
    0,
    &[
        ("axes", Array(&Object(0, &[("values", VALUES)]))),
        ("signals", Array(&Object(0, &[("values", Series)]))),
        ("scalars", Items(1)),
        (
            "deviceStates",
            Array(&Object(0, &[("parameters", Array(&VALUES))])),
        ),
        (
            "frequencyTable",
            Object(1, &[("finish", Object(2, &[("point", Numbers)]))]),
        ),
        ("payload", Payload),
    ],
);

const TRANSIENT: Rule = Object(
    0,
    &[
        ("stepSizes", Items(1)),
        ("storeTraces", Array(&VALUES)),
        ("digitalTraces", Array(&Object(0, &[("points", Items(1))]))),
        ("realTraces", Array(&Object(0, &[("points", Items(2))]))),
        ("digitalBuses", Array(&Object(0, &[("members", Items(1))]))),
        ("currentImpulses", Array(&Impulse)),
        ("voltageImpulses", Array(&Impulse)),
    ],
);

pub(super) fn payload(tag: &str) -> Result<Rule, &'static str> {
    // The payload enum's PNoise spelling predates the result-kind tag.
    let tag = if tag == "p-noise" { "pnoise" } else { tag };
    let kind = AnalysisResultKind::ALL
        .into_iter()
        .find(|kind| kind.tag() == tag)
        .ok_or("unknown result payload family")?;
    Ok(family(kind))
}

fn family(kind: AnalysisResultKind) -> Rule {
    // Exhaustive: adding a result family requires deciding its admission rule.
    match kind {
        AnalysisResultKind::OperatingPoint => Object(0, &[("observables", Items(1))]),
        AnalysisResultKind::DcSweep => Object(0, &[("observables", Array(&VALUES))]),
        AnalysisResultKind::Ac
        | AnalysisResultKind::TransferFunction
        | AnalysisResultKind::Fourier => Skip,
        AnalysisResultKind::Transient => TRANSIENT,
        AnalysisResultKind::Noise => Object(
            0,
            &[(
                "contributions",
                Array(&Object(
                    0,
                    &[
                        ("outputContribution", Items(1)),
                        ("inputContribution", Items(1)),
                        ("percentage", Items(1)),
                    ],
                )),
            )],
        ),
        AnalysisResultKind::SParameters => {
            Object(0, &[("ports", Items(2)), ("angularFrequencies", Items(1))])
        }
        AnalysisResultKind::PortNoise => Object(0, &[("twoPort", Items(6))]),
        AnalysisResultKind::Distortion => Object(
            0,
            &[("products", Array(&Object(0, &[("frequencies", Items(1))])))],
        ),
        AnalysisResultKind::Stability => Object(
            4,
            &[
                ("nyquist", Items(3)),
                (
                    "circuitPoles",
                    Object(0, &[("spectrum", Object(1, &[("poles", Items(2))]))]),
                ),
            ],
        ),
        AnalysisResultKind::Sensitivity => Object(
            0,
            &[
                ("entries", Items(3)),
                (
                    "acEntries",
                    Array(&Object(
                        1,
                        &[
                            ("absolute", Items(2)),
                            ("normalized", Items(2)),
                            ("magnitude", Items(1)),
                            ("phase", Items(1)),
                        ],
                    )),
                ),
            ],
        ),
        AnalysisResultKind::PoleZero => Object(0, &[("poles", Items(2)), ("zeros", Items(2))]),
        AnalysisResultKind::Fft => Object(
            0,
            &[
                ("metrics", Object(0, &[("largestHarmonics", Items(4))])),
                ("status", Numbers),
            ],
        ),
        AnalysisResultKind::MonteCarlo => Object(
            0,
            &[
                ("successfulTrialIndices", Items(1)),
                (
                    "statistics",
                    Array(&Object(
                        4,
                        &[
                            ("samples", Items(1)),
                            ("binEdges", Items(1)),
                            ("histogram", Items(1)),
                        ],
                    )),
                ),
            ],
        ),
        AnalysisResultKind::Pss => Object(0, &[("floquetMultipliers", Items(2))]),
        AnalysisResultKind::Pac => Object(
            0,
            &[
                (
                    "sidebands",
                    Array(&Object(
                        0,
                        &[
                            ("absoluteFrequencies", Items(1)),
                            ("frequencyOffsets", Items(1)),
                        ],
                    )),
                ),
                ("conversionMatrix", Object(0, &[("entries", Items(2))])),
            ],
        ),
        AnalysisResultKind::Pxf => Object(0, &[("groupDelay", Items(2))]),
        AnalysisResultKind::PNoise => Object(
            0,
            &[(
                "contributors",
                Array(&Object(0, &[("contributions", Items(2))])),
            )],
        ),
        AnalysisResultKind::Pstb => Object(
            0,
            &[
                ("modes", Items(5)),
                ("subharmonics", Items(1)),
                ("probeStateProjection", Items(2)),
            ],
        ),
        AnalysisResultKind::HarmonicBalance => Object(
            0,
            &[(
                "reactiveSpectra",
                Array(&Object(
                    0,
                    &[
                        ("voltageCoefficients", Items(2)),
                        ("currentCoefficients", Items(2)),
                    ],
                )),
            )],
        ),
        AnalysisResultKind::Envelope => Object(
            0,
            &[
                (
                    "carrier",
                    Object(
                        0,
                        &[
                            ("harmonicFrequencies", Items(1)),
                            (
                                "nodeSpectra",
                                Array(&Object(0, &[("coefficients", Items(2))])),
                            ),
                        ],
                    ),
                ),
                ("transient", TRANSIENT),
            ],
        ),
        AnalysisResultKind::DcMatch => Object(4, &[("contributors", Items(4))]),
        AnalysisResultKind::Qpss
        | AnalysisResultKind::Qpac
        | AnalysisResultKind::Qpxf
        | AnalysisResultKind::Qpnoise => Numbers,
    }
}
