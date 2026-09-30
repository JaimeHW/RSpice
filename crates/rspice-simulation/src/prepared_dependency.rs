//! Required producer kinds and compatibility of frozen analysis requests.
use crate::execution_options::SpecExecutionOptions;
use rspice_app_types::product::{AnalysisInstanceId, ContentDigest};
use rspice_simulation_contract::analysis_spec::{AnalysisSpec, PssMethod};
use rspice_simulation_contract::dependency_contract::{
    FourierTransientRequirement, PeriodicStateCapability, TransientCapability,
    validate_fourier_transient_contract, validate_harmonic_balance_carrier_contract,
    validate_periodic_state_contract,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionArtifactKind {
    TransientTrajectory,
    PeriodicState,
    HbState,
    QpssState,
    DcOperatingPointSeed,
}

impl ExecutionArtifactKind {
    /// What a refusal calls the producer of this artifact.
    pub const fn producer_label(self) -> &'static str {
        match self {
            Self::TransientTrajectory => "Transient",
            Self::PeriodicState => "shooting PSS",
            Self::HbState => "Harmonic Balance",
            Self::QpssState => "QPSS",
            Self::DcOperatingPointSeed => "operating point",
        }
    }
}

/// Which typed artifact kinds one prepared task may bind, in the order the
/// plan prefers them. Empty for a task that binds none.
///
/// One table, asked by every stage that has an opinion about a task's
/// dependencies — queue binding, snapshot preparation, resolution at dispatch.
/// It used to be written out at each of them, which is how the periodic
/// small-signal family came to be listed as a shooting-PSS consumer in four
/// places while the engine had accepted either carrier all along.
///
/// The three carrier-bearing kinds read their family off the request: `FROM=`
/// names one, and its absence names the nearest preceding periodic solve of
/// either family, which is why that position lists both. Which of the two the
/// task actually binds is settled by the plan, whose dependency edge names an
/// exact instance.
pub fn required_artifact_kinds(
    spec: &AnalysisSpec,
    options: &SpecExecutionOptions,
) -> &'static [ExecutionArtifactKind] {
    use rspice_simulation_contract::periodic_carrier::PeriodicCarrier;

    const NONE: &[ExecutionArtifactKind] = &[];
    const TRANSIENT: &[ExecutionArtifactKind] = &[ExecutionArtifactKind::TransientTrajectory];
    const PERIODIC: &[ExecutionArtifactKind] = &[ExecutionArtifactKind::PeriodicState];
    const HB: &[ExecutionArtifactKind] = &[ExecutionArtifactKind::HbState];
    const QPSS: &[ExecutionArtifactKind] = &[ExecutionArtifactKind::QpssState];
    const DC_SEED: &[ExecutionArtifactKind] = &[ExecutionArtifactKind::DcOperatingPointSeed];
    const EITHER_PERIODIC: &[ExecutionArtifactKind] = &[
        ExecutionArtifactKind::PeriodicState,
        ExecutionArtifactKind::HbState,
    ];

    fn for_carrier(carrier: Option<PeriodicCarrier>) -> &'static [ExecutionArtifactKind] {
        match carrier {
            // A request whose options were never built states no carrier, and
            // the default position is the one that admits either family.
            None | Some(PeriodicCarrier::Preceding) => EITHER_PERIODIC,
            Some(PeriodicCarrier::Pss) => PERIODIC,
            Some(PeriodicCarrier::Hb) => HB,
        }
    }

    match spec {
        // A recorded FFT reads the spectrum its transient already solved, so
        // it binds the same trajectory a Fourier analysis does.
        AnalysisSpec::Fourier { .. } | AnalysisSpec::Fft { .. } => TRANSIENT,
        AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. } => HB,
        AnalysisSpec::Qpac { .. } | AnalysisSpec::Qpnoise { .. } | AnalysisSpec::Qpxf { .. } => {
            QPSS
        }
        AnalysisSpec::Pss {
            method: PssMethod::Shooting,
            ..
        }
        | AnalysisSpec::Qpss { .. }
        | AnalysisSpec::HarmonicBalance { .. } => DC_SEED,
        AnalysisSpec::Pac => for_carrier(options.pac.as_ref().map(|config| config.carrier)),
        AnalysisSpec::Pxf => for_carrier(options.pxf.as_ref().map(|config| config.carrier)),
        AnalysisSpec::Pnoise => for_carrier(options.pnoise.as_ref().map(|config| config.carrier)),
        // `.PSTB` reads a monodromy matrix and only a shooting solve produces
        // one, and the spectrum is a reading of the steady state its siblings
        // consume. Neither has a carrier to choose.
        AnalysisSpec::Pstb | AnalysisSpec::Psp { .. } | AnalysisSpec::PssSpectrum { .. } => {
            PERIODIC
        }
        _ => NONE,
    }
}

pub fn validate_prepared_dependency_contract_with_options(
    consumer: &AnalysisSpec,
    consumer_options: &SpecExecutionOptions,
    producer: &AnalysisSpec,
) -> Result<(), ExecutionArtifactError> {
    if matches!(
        consumer,
        AnalysisSpec::Qpac { .. } | AnalysisSpec::Qpnoise { .. } | AnalysisSpec::Qpxf { .. }
    ) {
        return producer.qpss_config().map(|_| ()).map_err(|error| {
            ExecutionArtifactError::ContractMismatch(format!(
                "{} requires a compatible QPSS producer: {error}",
                consumer.run_type().display_name()
            ))
        });
    }
    if matches!(
        consumer,
        AnalysisSpec::Pss {
            method: PssMethod::Shooting,
            ..
        } | AnalysisSpec::Qpss { .. }
            | AnalysisSpec::HarmonicBalance { .. }
    ) {
        return match producer {
            AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. } => Ok(()),
            _ => Err(ExecutionArtifactError::ContractMismatch(format!(
                "periodic analysis cannot consume a DC operating-point seed produced by {}",
                producer.run_type().display_name()
            ))),
        };
    }
    if matches!(consumer, AnalysisSpec::Pss { .. }) {
        return Err(ExecutionArtifactError::ContractMismatch(
            "legacy HB-PSS is not executable and cannot bind a DC operating-point seed".to_owned(),
        ));
    }
    if matches!(
        consumer,
        AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. }
    ) {
        return match producer {
            AnalysisSpec::HarmonicBalance { .. } => Ok(()),
            _ => Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} cannot consume an HB state produced by {}",
                consumer.run_type().display_name(),
                producer.run_type().display_name()
            ))),
        };
    }
    if matches!(
        consumer,
        AnalysisSpec::PssSpectrum { .. }
            | AnalysisSpec::Pac
            | AnalysisSpec::Pxf
            | AnalysisSpec::Pnoise
            | AnalysisSpec::Pstb
            | AnalysisSpec::Psp { .. }
    ) {
        let require_autonomous = matches!(consumer, AnalysisSpec::Pnoise)
            && consumer_options
                .pnoise
                .as_ref()
                .is_some_and(|config| config.noise_ref == crate::periodic::PnoiseReference::Phase);
        // The carrier family this request named. A harmonic-balance producer
        // is admitted only where the request's `FROM=` admits it, so a sealed
        // specification cannot be linearized about a solution other than the
        // one it reported.
        if matches!(producer, AnalysisSpec::HarmonicBalance { .. }) {
            let accepts_hb = required_artifact_kinds(consumer, consumer_options)
                .contains(&ExecutionArtifactKind::HbState);
            if !accepts_hb {
                return Err(ExecutionArtifactError::ContractMismatch(format!(
                    "{} names a shooting-PSS carrier and cannot consume a harmonic-balance state",
                    consumer.run_type().display_name()
                )));
            }
            return validate_harmonic_balance_carrier_contract(
                consumer.run_type().display_name(),
                require_autonomous,
            )
            .map_err(ExecutionArtifactError::ContractMismatch);
        }
        return match producer {
            AnalysisSpec::Pss {
                method: PssMethod::Shooting,
                oscillator_mode,
                ..
            } => validate_periodic_state_contract(
                consumer.run_type().display_name(),
                PeriodicStateCapability {
                    shooting: true,
                    autonomous: *oscillator_mode,
                },
                require_autonomous,
            )
            .map_err(ExecutionArtifactError::ContractMismatch),
            AnalysisSpec::Pss {
                oscillator_mode, ..
            } => validate_periodic_state_contract(
                consumer.run_type().display_name(),
                PeriodicStateCapability {
                    shooting: false,
                    autonomous: *oscillator_mode,
                },
                require_autonomous,
            )
            .map_err(ExecutionArtifactError::ContractMismatch),
            _ => Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} cannot consume a typed artifact produced by {}",
                consumer.run_type().display_name(),
                producer.run_type().display_name()
            ))),
        };
    }

    // A recorded FFT's only requirement of its producer is the engine's own:
    // a card whose STOP is past the transient's stop time fails that
    // transient, so the pair is refused before the run rather than during it.
    // An unauthored STOP takes the transient's stop and can never exceed it.
    if let AnalysisSpec::Fft { request } = consumer {
        return match producer {
            AnalysisSpec::Transient {
                stop_time: transient_stop,
                ..
            } => {
                let stop = request.stop.unwrap_or(*transient_stop);
                if stop > *transient_stop {
                    Err(ExecutionArtifactError::ContractMismatch(format!(
                        "STOP {stop} exceeds transient stop time {transient_stop}"
                    )))
                } else {
                    Ok(())
                }
            }
            _ => Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} cannot consume a typed artifact produced by {}",
                consumer.run_type().display_name(),
                producer.run_type().display_name()
            ))),
        };
    }

    let (
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            start_time,
            stop_time,
            ..
        },
        AnalysisSpec::Transient {
            stop_time: transient_stop,
            step_time,
            start_time: transient_start,
            max_timestep,
            ..
        },
    ) = (consumer, producer)
    else {
        return Err(ExecutionArtifactError::ContractMismatch(format!(
            "{} cannot consume a typed artifact produced by {}",
            consumer.run_type().display_name(),
            producer.run_type().display_name()
        )));
    };

    let num_harmonics = u32::try_from(*num_harmonics).map_err(|_| {
        ExecutionArtifactError::ContractMismatch(
            "Fourier harmonic count exceeds the supported dependency contract".to_owned(),
        )
    })?;
    validate_fourier_transient_contract(
        FourierTransientRequirement {
            start_time: *start_time,
            stop_time: *stop_time,
            fundamental_freq: *fundamental_freq,
            num_harmonics,
        },
        TransientCapability {
            start_time: *transient_start,
            stop_time: *transient_stop,
            step_time: *step_time,
            max_timestep: *max_timestep,
        },
    )
    .map_err(ExecutionArtifactError::ContractMismatch)
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ExecutionArtifactError {
    #[error("missing {kind:?} artifact from bound prerequisite {producer}")]
    Missing {
        producer: AnalysisInstanceId,
        kind: ExecutionArtifactKind,
    },
    #[error("dependency artifact belongs to producer {actual}, expected {expected}")]
    ProducerMismatch {
        expected: AnalysisInstanceId,
        actual: AnalysisInstanceId,
    },
    #[error("dependency artifact belongs to stale snapshot {actual}, expected {expected}")]
    StaleSnapshot {
        expected: ContentDigest,
        actual: ContentDigest,
    },
    #[error("dependency artifact payload digest is {actual}, expected {expected}")]
    PayloadDigestMismatch {
        expected: ContentDigest,
        actual: ContentDigest,
    },
    #[error("invalid dependency artifact payload: {0}")]
    InvalidPayload(String),
    #[error("invalid typed dependency contract: {0}")]
    ContractMismatch(String),
    #[error("invalid dependency artifact transfer: {0}")]
    Transport(String),
}
