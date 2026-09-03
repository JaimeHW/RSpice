//! Authoritative non-UI result and signal-surface capability registry.
//!
//! This registry describes adapters, not solver qualification. `Mapped` means
//! a surface has an explicit representation for the complete public result
//! family. `Partial` means an execution/export path exists but loses result
//! structure, metadata, or orchestration semantics. `Unsupported` means the
//! surface deliberately has no adapter. Keeping every declaration in code, with
//! its reason, makes each gap visible without advertising it as implemented.

use super::{AnalysisKind, SignalKind};

/// Result families that must be mapped intentionally by every non-UI surface.
///
/// Keep this enum exhaustive. In particular, do not add `#[non_exhaustive]`:
/// the exhaustive matches in this module are the compile-time part of the
/// capability gate. A new core result family cannot compile until it has a
/// stable tag and a row with four explicit surface declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnalysisResultKind {
    OperatingPoint,
    DcSweep,
    Ac,
    Transient,
    Noise,
    SParameters,
    PortNoise,
    Distortion,
    TransferFunction,
    Stability,
    Sensitivity,
    PoleZero,
    Fourier,
    Fft,
    MonteCarlo,
    Pss,
    Pac,
    PNoise,
    HarmonicBalance,
    Envelope,
}

impl AnalysisResultKind {
    pub const ALL: [Self; 20] = [
        Self::OperatingPoint,
        Self::DcSweep,
        Self::Ac,
        Self::Transient,
        Self::Noise,
        Self::SParameters,
        Self::PortNoise,
        Self::Distortion,
        Self::TransferFunction,
        Self::Stability,
        Self::Sensitivity,
        Self::PoleZero,
        Self::Fourier,
        Self::Fft,
        Self::MonteCarlo,
        Self::Pss,
        Self::Pac,
        Self::PNoise,
        Self::HarmonicBalance,
        Self::Envelope,
    ];

    pub const fn tag(self) -> &'static str {
        match self {
            Self::OperatingPoint => "op",
            Self::DcSweep => "dc",
            Self::Ac => "ac",
            Self::Transient => "tran",
            Self::Noise => "noise",
            Self::SParameters => "sp",
            Self::PortNoise => "port-noise",
            Self::Distortion => "distortion",
            Self::TransferFunction => "tf",
            Self::Stability => "stb",
            Self::Sensitivity => "sensitivity",
            Self::PoleZero => "pole-zero",
            Self::Fourier => "fourier",
            Self::Fft => "fft",
            Self::MonteCarlo => "monte-carlo",
            Self::Pss => "pss",
            Self::Pac => "pac",
            Self::PNoise => "pnoise",
            Self::HarmonicBalance => "hb",
            Self::Envelope => "envelope",
        }
    }
}

/// Convert every core analysis identity to the result family it produces.
///
/// Port-noise is an optional second result of `SP`, so it has a registry row
/// but no distinct `AnalysisKind`. Implicit and authored OP share one schema.
pub const fn analysis_result_kind(kind: AnalysisKind) -> AnalysisResultKind {
    match kind {
        AnalysisKind::ImplicitOp | AnalysisKind::Op => AnalysisResultKind::OperatingPoint,
        AnalysisKind::Dc => AnalysisResultKind::DcSweep,
        AnalysisKind::Ac => AnalysisResultKind::Ac,
        AnalysisKind::Tran => AnalysisResultKind::Transient,
        AnalysisKind::Noise => AnalysisResultKind::Noise,
        AnalysisKind::Sp => AnalysisResultKind::SParameters,
        AnalysisKind::Stb => AnalysisResultKind::Stability,
        AnalysisKind::Distortion => AnalysisResultKind::Distortion,
        AnalysisKind::PoleZero => AnalysisResultKind::PoleZero,
        AnalysisKind::Sensitivity => AnalysisResultKind::Sensitivity,
        AnalysisKind::TransferFunction => AnalysisResultKind::TransferFunction,
        AnalysisKind::Pss => AnalysisResultKind::Pss,
        AnalysisKind::Pac => AnalysisResultKind::Pac,
        AnalysisKind::PNoise => AnalysisResultKind::PNoise,
        AnalysisKind::HarmonicBalance => AnalysisResultKind::HarmonicBalance,
        AnalysisKind::Envelope => AnalysisResultKind::Envelope,
        AnalysisKind::MonteCarlo => AnalysisResultKind::MonteCarlo,
        AnalysisKind::Fourier => AnalysisResultKind::Fourier,
        AnalysisKind::Fft => AnalysisResultKind::Fft,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonUiSurface {
    Cli,
    Python,
    Wasm,
    EngineAdapter,
}

impl NonUiSurface {
    pub const ALL: [Self; 4] = [Self::Cli, Self::Python, Self::Wasm, Self::EngineAdapter];

    pub const fn heading(self) -> &'static str {
        match self {
            Self::Cli => "CLI",
            Self::Python => "Python",
            Self::Wasm => "WASM",
            Self::EngineAdapter => "Engine adapter",
        }
    }
}

/// Fidelity of one explicit surface mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingStatus {
    /// The surface has a typed/structured adapter for the public result shape.
    Mapped,
    /// Some path exists, but it is not a complete typed result mapping.
    Partial(&'static str),
    /// No mapping exists; the reason is intentional and visible.
    Unsupported(&'static str),
}

impl MappingStatus {
    pub const fn note(self) -> Option<&'static str> {
        match self {
            Self::Mapped => None,
            Self::Partial(note) | Self::Unsupported(note) => Some(note),
        }
    }
}

/// Scalar and shared-deck-axis declarations for one surface/result pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceCapability {
    pub scalar: MappingStatus,
    pub stepped: MappingStatus,
    pub temperature: MappingStatus,
}

impl SurfaceCapability {
    pub const fn new(
        scalar: MappingStatus,
        stepped: MappingStatus,
        temperature: MappingStatus,
    ) -> Self {
        Self {
            scalar,
            stepped,
            temperature,
        }
    }

    pub const fn unsupported(reason: &'static str) -> Self {
        Self::new(
            MappingStatus::Unsupported(reason),
            MappingStatus::Unsupported(reason),
            MappingStatus::Unsupported(reason),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalysisResultCapability {
    pub result: AnalysisResultKind,
    pub cli: SurfaceCapability,
    pub python: SurfaceCapability,
    pub wasm: SurfaceCapability,
    pub engine_adapter: SurfaceCapability,
}

impl AnalysisResultCapability {
    pub const fn surface(self, surface: NonUiSurface) -> SurfaceCapability {
        match surface {
            NonUiSurface::Cli => self.cli,
            NonUiSurface::Python => self.python,
            NonUiSurface::Wasm => self.wasm,
            NonUiSurface::EngineAdapter => self.engine_adapter,
        }
    }
}

const PY_PNOISE_DRIVEN_AXIS_ONLY: &str = "deck-axis .PNOISE executes around a driven carrier only; an autonomous PSS carrier's      oscillator phase noise has no run-report field";

/// The CLI publishes the shared typed result document for this family, under
/// its canonical analysis identity, for a scalar deck and for every coordinate
/// of a `.STEP` or `.TEMP` axis.
const fn cli_mapped_axes() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Mapped,
        MappingStatus::Mapped,
        MappingStatus::Mapped,
    )
}

const fn python_mapped_axes() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Mapped,
        MappingStatus::Mapped,
        MappingStatus::Mapped,
    )
}

const fn wasm_mapped_axes() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Mapped,
        MappingStatus::Mapped,
        MappingStatus::Mapped,
    )
}

const fn adapter_typed_axes() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Mapped,
        MappingStatus::Mapped,
        MappingStatus::Mapped,
    )
}

const ADAPTER_FFT_ATTACHED: &str = "a complete typed FFT bundle is published beside its parent transient, but it is the adapter's \
     own schema rather than the shared fft result document";

const fn adapter_attached_fft() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Partial(ADAPTER_FFT_ATTACHED),
        MappingStatus::Partial(ADAPTER_FFT_ATTACHED),
        MappingStatus::Partial(ADAPTER_FFT_ATTACHED),
    )
}

/// One authoritative row per core result family.
///
/// Every constructor is deliberately visible in source: unsupported cells are
/// declarations, never a wildcard/default inferred by the renderer.
pub const ANALYSIS_CAPABILITY_MATRIX: &[AnalysisResultCapability] = &[
    AnalysisResultCapability {
        result: AnalysisResultKind::OperatingPoint,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::DcSweep,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Ac,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Transient,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Noise,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::SParameters,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::PortNoise,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Distortion,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::TransferFunction,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Stability,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Sensitivity,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::PoleZero,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Fourier,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Fft,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_attached_fft(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::MonteCarlo,
        cli: cli_mapped_axes(),
        python: SurfaceCapability::new(
            MappingStatus::Mapped,
            MappingStatus::Partial(
                "nested STEP executes, but coordinate-derived Monte Carlo seed semantics are undefined",
            ),
            MappingStatus::Partial(
                "nested TEMP executes, but coordinate-derived Monte Carlo seed semantics are undefined",
            ),
        ),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Pss,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Pac,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::PNoise,
        cli: cli_mapped_axes(),
        python: SurfaceCapability::new(
            MappingStatus::Mapped,
            MappingStatus::Partial(PY_PNOISE_DRIVEN_AXIS_ONLY),
            MappingStatus::Partial(PY_PNOISE_DRIVEN_AXIS_ONLY),
        ),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::HarmonicBalance,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Envelope,
        cli: cli_mapped_axes(),
        python: python_mapped_axes(),
        wasm: wasm_mapped_axes(),
        engine_adapter: adapter_typed_axes(),
    },
];

/// Exhaustive result-to-row lookup. A new `AnalysisResultKind` variant makes
/// this match fail compilation until its surface declarations are added.
pub const fn analysis_result_capability(
    kind: AnalysisResultKind,
) -> &'static AnalysisResultCapability {
    match kind {
        AnalysisResultKind::OperatingPoint => &ANALYSIS_CAPABILITY_MATRIX[0],
        AnalysisResultKind::DcSweep => &ANALYSIS_CAPABILITY_MATRIX[1],
        AnalysisResultKind::Ac => &ANALYSIS_CAPABILITY_MATRIX[2],
        AnalysisResultKind::Transient => &ANALYSIS_CAPABILITY_MATRIX[3],
        AnalysisResultKind::Noise => &ANALYSIS_CAPABILITY_MATRIX[4],
        AnalysisResultKind::SParameters => &ANALYSIS_CAPABILITY_MATRIX[5],
        AnalysisResultKind::PortNoise => &ANALYSIS_CAPABILITY_MATRIX[6],
        AnalysisResultKind::Distortion => &ANALYSIS_CAPABILITY_MATRIX[7],
        AnalysisResultKind::TransferFunction => &ANALYSIS_CAPABILITY_MATRIX[8],
        AnalysisResultKind::Stability => &ANALYSIS_CAPABILITY_MATRIX[9],
        AnalysisResultKind::Sensitivity => &ANALYSIS_CAPABILITY_MATRIX[10],
        AnalysisResultKind::PoleZero => &ANALYSIS_CAPABILITY_MATRIX[11],
        AnalysisResultKind::Fourier => &ANALYSIS_CAPABILITY_MATRIX[12],
        AnalysisResultKind::Fft => &ANALYSIS_CAPABILITY_MATRIX[13],
        AnalysisResultKind::MonteCarlo => &ANALYSIS_CAPABILITY_MATRIX[14],
        AnalysisResultKind::Pss => &ANALYSIS_CAPABILITY_MATRIX[15],
        AnalysisResultKind::Pac => &ANALYSIS_CAPABILITY_MATRIX[16],
        AnalysisResultKind::PNoise => &ANALYSIS_CAPABILITY_MATRIX[17],
        AnalysisResultKind::HarmonicBalance => &ANALYSIS_CAPABILITY_MATRIX[18],
        AnalysisResultKind::Envelope => &ANALYSIS_CAPABILITY_MATRIX[19],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalCapability {
    pub signal: SignalKind,
    pub cli: MappingStatus,
    pub python: MappingStatus,
    pub wasm: MappingStatus,
    pub engine_adapter: MappingStatus,
}

impl SignalCapability {
    pub const fn surface(self, surface: NonUiSurface) -> MappingStatus {
        match surface {
            NonUiSurface::Cli => self.cli,
            NonUiSurface::Python => self.python,
            NonUiSurface::Wasm => self.wasm,
            NonUiSurface::EngineAdapter => self.engine_adapter,
        }
    }
}

/// The Python device-observable and analysis-scalar gaps, stated by family.
///
/// These are deliberately specific rather than "a subset of result families":
/// the missing cells are missing because the engine computes no such signal
/// for those families, not because the binding declined to expose one, and a
/// reader deciding whether to wait for the adapter needs to know which it is.
const PY_DEVICE_OBSERVABLE_SUBSET: &str = "mapped for operating-point, DC-sweep and transient results, which are the families whose \
     solvers capture per-device observables; the frequency-domain and periodic families compute \
     none to expose";
const PY_SCALAR_SUBSET: &str = "mapped as named accessors on the noise, transfer-function, stability, pole-zero, \
     distortion, S-parameter, harmonic-balance, PSS, Monte Carlo and envelope results, not as a \
     shared SignalDescriptor lookup; AC and operating-point results publish no analysis-owned \
     scalar";
const DIGITAL_OUT_OF_SCOPE: &str =
    "digital/AMS surface work is owned by the separate digital effort";
const WASM_LOGIC_SAMPLES: &str = "descriptors, state/strength samples and validity masks cross the browser boundary with the shared document, but no digital-specific browser surface exists; that work is owned by the separate digital effort";

/// Signal-descriptor adapter coverage, kept beside result coverage so adding a
/// `SignalKind` cannot silently inherit a frontend default.
pub const SIGNAL_CAPABILITY_MATRIX: &[SignalCapability] = &[
    SignalCapability {
        signal: SignalKind::Voltage,
        cli: MappingStatus::Mapped,
        python: MappingStatus::Mapped,
        wasm: MappingStatus::Mapped,
        engine_adapter: MappingStatus::Mapped,
    },
    SignalCapability {
        signal: SignalKind::Current,
        cli: MappingStatus::Mapped,
        python: MappingStatus::Mapped,
        wasm: MappingStatus::Mapped,
        engine_adapter: MappingStatus::Mapped,
    },
    SignalCapability {
        signal: SignalKind::DeviceObservable,
        cli: MappingStatus::Mapped,
        python: MappingStatus::Partial(PY_DEVICE_OBSERVABLE_SUBSET),
        wasm: MappingStatus::Mapped,
        engine_adapter: MappingStatus::Mapped,
    },
    SignalCapability {
        signal: SignalKind::Scalar,
        cli: MappingStatus::Mapped,
        python: MappingStatus::Partial(PY_SCALAR_SUBSET),
        wasm: MappingStatus::Mapped,
        engine_adapter: MappingStatus::Mapped,
    },
    SignalCapability {
        signal: SignalKind::Digital,
        cli: MappingStatus::Unsupported(DIGITAL_OUT_OF_SCOPE),
        python: MappingStatus::Unsupported(DIGITAL_OUT_OF_SCOPE),
        wasm: MappingStatus::Partial(WASM_LOGIC_SAMPLES),
        engine_adapter: MappingStatus::Unsupported(DIGITAL_OUT_OF_SCOPE),
    },
];

/// Exhaustive signal-to-row lookup. This is the signal-schema counterpart of
/// `analysis_result_capability`.
pub const fn signal_capability(kind: SignalKind) -> &'static SignalCapability {
    match kind {
        SignalKind::Voltage => &SIGNAL_CAPABILITY_MATRIX[0],
        SignalKind::Current => &SIGNAL_CAPABILITY_MATRIX[1],
        SignalKind::DeviceObservable => &SIGNAL_CAPABILITY_MATRIX[2],
        SignalKind::Scalar => &SIGNAL_CAPABILITY_MATRIX[3],
        SignalKind::Digital => &SIGNAL_CAPABILITY_MATRIX[4],
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    fn assert_status_is_deliberate(status: MappingStatus) {
        if let Some(note) = status.note() {
            assert!(!note.trim().is_empty(), "non-mapped status needs a reason");
        }
    }

    #[test]
    fn every_result_kind_has_exactly_one_explicit_row() {
        assert_eq!(
            ANALYSIS_CAPABILITY_MATRIX.len(),
            AnalysisResultKind::ALL.len()
        );
        let mut seen = BTreeSet::new();
        for kind in AnalysisResultKind::ALL {
            let row = analysis_result_capability(kind);
            assert_eq!(row.result, kind);
            assert!(seen.insert(row.result), "duplicate row for {kind:?}");
            for surface in NonUiSurface::ALL {
                let declaration = row.surface(surface);
                for status in [
                    declaration.scalar,
                    declaration.stepped,
                    declaration.temperature,
                ] {
                    assert_status_is_deliberate(status);
                }
            }
        }
    }

    #[test]
    fn every_signal_kind_has_exactly_one_explicit_row() {
        assert_eq!(SIGNAL_CAPABILITY_MATRIX.len(), 5);
        let expected = [
            SignalKind::Voltage,
            SignalKind::Current,
            SignalKind::DeviceObservable,
            SignalKind::Scalar,
            SignalKind::Digital,
        ];
        let mut seen = BTreeSet::new();
        for kind in expected {
            let row = signal_capability(kind);
            assert_eq!(row.signal, kind);
            assert!(seen.insert(row.signal), "duplicate row for {kind:?}");
            for surface in NonUiSurface::ALL {
                assert_status_is_deliberate(row.surface(surface));
            }
        }
    }

    #[test]
    fn every_core_analysis_identity_selects_its_registered_result() {
        let kinds = [
            AnalysisKind::ImplicitOp,
            AnalysisKind::Op,
            AnalysisKind::Dc,
            AnalysisKind::Ac,
            AnalysisKind::Tran,
            AnalysisKind::Noise,
            AnalysisKind::Sp,
            AnalysisKind::Stb,
            AnalysisKind::Distortion,
            AnalysisKind::PoleZero,
            AnalysisKind::Sensitivity,
            AnalysisKind::TransferFunction,
            AnalysisKind::Pss,
            AnalysisKind::Pac,
            AnalysisKind::PNoise,
            AnalysisKind::HarmonicBalance,
            AnalysisKind::Envelope,
            AnalysisKind::MonteCarlo,
            AnalysisKind::Fourier,
            AnalysisKind::Fft,
        ];
        for kind in kinds {
            let result = analysis_result_kind(kind);
            assert_eq!(analysis_result_capability(result).result, result);
        }
    }
}
