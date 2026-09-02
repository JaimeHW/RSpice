//! Authoritative non-UI result and signal-surface capability registry.
//!
//! This registry describes adapters, not solver qualification. `Mapped` means
//! a surface has an explicit representation for the complete public result
//! family. `Partial` means an execution/export path exists but loses result
//! structure, metadata, or orchestration semantics. `Unsupported` means the
//! surface deliberately has no adapter. The generated matrix therefore makes
//! gaps visible without advertising them as implemented.

use std::fmt::Write as _;

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

    const fn label(self) -> &'static str {
        match self {
            Self::Mapped => "mapped",
            Self::Partial(_) => "partial",
            Self::Unsupported(_) => "unsupported",
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

const CLI_ARTIFACT: &str = "CSV/text artifact exists, but no shared typed result document";
const CLI_AXIS: &str =
    "shared deck axes execute, but the CLI artifact does not retain a typed coordinate document";
const CLI_AXIS_UNAVAILABLE: &str = "CLI has no authored deck-axis route for this analysis family";
const PY_AXIS_UNAVAILABLE: &str =
    "typed direct API exists, but Engine.run has no authored axis route";
const WASM_UNAVAILABLE: &str = "browser API has no result adapter for this family";
const WASM_AXIS_UNAVAILABLE: &str = "browser API does not consume DeckPlan axes";
const ADAPTER_UNAVAILABLE: &str = "protocol-3 adapter has no result mapping for this family";
const ADAPTER_AXIS_UNAVAILABLE: &str = "protocol-3 adapter does not consume DeckPlan axes";

const fn cli_artifact_axes() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Partial(CLI_ARTIFACT),
        MappingStatus::Partial(CLI_AXIS),
        MappingStatus::Partial(CLI_AXIS),
    )
}

const fn cli_artifact_scalar_only() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Partial(CLI_ARTIFACT),
        MappingStatus::Unsupported(CLI_AXIS_UNAVAILABLE),
        MappingStatus::Unsupported(CLI_AXIS_UNAVAILABLE),
    )
}

const fn python_mapped_axes() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Mapped,
        MappingStatus::Mapped,
        MappingStatus::Mapped,
    )
}

const fn python_direct_only() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Mapped,
        MappingStatus::Unsupported(PY_AXIS_UNAVAILABLE),
        MappingStatus::Unsupported(PY_AXIS_UNAVAILABLE),
    )
}

const fn wasm_direct_only() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Mapped,
        MappingStatus::Unsupported(WASM_AXIS_UNAVAILABLE),
        MappingStatus::Unsupported(WASM_AXIS_UNAVAILABLE),
    )
}

const fn adapter_typed_scalar_only() -> SurfaceCapability {
    SurfaceCapability::new(
        MappingStatus::Mapped,
        MappingStatus::Unsupported(ADAPTER_AXIS_UNAVAILABLE),
        MappingStatus::Unsupported(ADAPTER_AXIS_UNAVAILABLE),
    )
}

/// One authoritative row per core result family.
///
/// Every constructor is deliberately visible in source: unsupported cells are
/// declarations, never a wildcard/default inferred by the renderer.
pub const ANALYSIS_CAPABILITY_MATRIX: &[AnalysisResultCapability] = &[
    AnalysisResultCapability {
        result: AnalysisResultKind::OperatingPoint,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: wasm_direct_only(),
        engine_adapter: adapter_typed_scalar_only(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::DcSweep,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: wasm_direct_only(),
        engine_adapter: adapter_typed_scalar_only(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Ac,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: wasm_direct_only(),
        engine_adapter: adapter_typed_scalar_only(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Transient,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: wasm_direct_only(),
        engine_adapter: adapter_typed_scalar_only(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Noise,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: wasm_direct_only(),
        engine_adapter: adapter_typed_scalar_only(),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::SParameters,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::PortNoise,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Distortion,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::TransferFunction,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Stability,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Sensitivity,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::PoleZero,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Fourier,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Fft,
        cli: SurfaceCapability::new(
            MappingStatus::Mapped,
            MappingStatus::Mapped,
            MappingStatus::Mapped,
        ),
        python: python_mapped_axes(),
        wasm: wasm_direct_only(),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::MonteCarlo,
        cli: cli_artifact_scalar_only(),
        python: SurfaceCapability::new(
            MappingStatus::Mapped,
            MappingStatus::Partial(
                "nested STEP executes, but coordinate-derived Monte Carlo seed semantics are undefined",
            ),
            MappingStatus::Partial(
                "nested TEMP executes, but coordinate-derived Monte Carlo seed semantics are undefined",
            ),
        ),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Pss,
        cli: cli_artifact_scalar_only(),
        python: python_direct_only(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Pac,
        cli: SurfaceCapability::unsupported("CLI has no PAC execution or result adapter"),
        python: python_direct_only(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::PNoise,
        cli: SurfaceCapability::unsupported("CLI has no PNoise execution or result adapter"),
        python: python_direct_only(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::HarmonicBalance,
        cli: cli_artifact_axes(),
        python: python_mapped_axes(),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
    },
    AnalysisResultCapability {
        result: AnalysisResultKind::Envelope,
        cli: SurfaceCapability::unsupported("CLI has no envelope result adapter"),
        python: SurfaceCapability::new(
            MappingStatus::Partial(
                "HB continuation state and continued transient are exposed, but no envelope result document exists",
            ),
            MappingStatus::Unsupported(PY_AXIS_UNAVAILABLE),
            MappingStatus::Unsupported(PY_AXIS_UNAVAILABLE),
        ),
        wasm: SurfaceCapability::unsupported(WASM_UNAVAILABLE),
        engine_adapter: SurfaceCapability::unsupported(ADAPTER_UNAVAILABLE),
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

const SIGNAL_ARTIFACT: &str = "export exists, but it is not a shared SignalDescriptor document";
const ADVANCED_SIGNAL_SUBSET: &str = "mapped for a subset of result families only";
const DIGITAL_OUT_OF_SCOPE: &str =
    "digital/AMS surface work is owned by the separate digital effort";

/// Signal-descriptor adapter coverage, kept beside result coverage so adding a
/// `SignalKind` cannot silently inherit a frontend default.
pub const SIGNAL_CAPABILITY_MATRIX: &[SignalCapability] = &[
    SignalCapability {
        signal: SignalKind::Voltage,
        cli: MappingStatus::Partial(SIGNAL_ARTIFACT),
        python: MappingStatus::Mapped,
        wasm: MappingStatus::Partial(ADVANCED_SIGNAL_SUBSET),
        engine_adapter: MappingStatus::Mapped,
    },
    SignalCapability {
        signal: SignalKind::Current,
        cli: MappingStatus::Partial(SIGNAL_ARTIFACT),
        python: MappingStatus::Mapped,
        wasm: MappingStatus::Partial(ADVANCED_SIGNAL_SUBSET),
        engine_adapter: MappingStatus::Mapped,
    },
    SignalCapability {
        signal: SignalKind::DeviceObservable,
        cli: MappingStatus::Partial(ADVANCED_SIGNAL_SUBSET),
        python: MappingStatus::Partial(ADVANCED_SIGNAL_SUBSET),
        wasm: MappingStatus::Partial("mapped for OP, DC, transient, and noise result documents"),
        engine_adapter: MappingStatus::Mapped,
    },
    SignalCapability {
        signal: SignalKind::Scalar,
        cli: MappingStatus::Partial(SIGNAL_ARTIFACT),
        python: MappingStatus::Partial(ADVANCED_SIGNAL_SUBSET),
        wasm: MappingStatus::Partial(
            "mapped for transient integration and noise scalars; other result families remain unavailable",
        ),
        engine_adapter: MappingStatus::Mapped,
    },
    SignalCapability {
        signal: SignalKind::Digital,
        cli: MappingStatus::Unsupported(DIGITAL_OUT_OF_SCOPE),
        python: MappingStatus::Unsupported(DIGITAL_OUT_OF_SCOPE),
        wasm: MappingStatus::Unsupported(DIGITAL_OUT_OF_SCOPE),
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

const fn signal_tag(kind: SignalKind) -> &'static str {
    match kind {
        SignalKind::Voltage => "voltage",
        SignalKind::Current => "current",
        SignalKind::DeviceObservable => "device-observable",
        SignalKind::Scalar => "scalar",
        SignalKind::Digital => "digital",
    }
}

fn render_status(status: MappingStatus) -> String {
    match status.note() {
        Some(note) => format!("{} — {}", status.label(), note),
        None => status.label().to_string(),
    }
}

fn render_surface(capability: SurfaceCapability) -> String {
    format!(
        "{} / {} / {}",
        capability.scalar.label(),
        capability.stepped.label(),
        capability.temperature.label()
    )
}

fn render_analysis_boundaries(output: &mut String) {
    output.push_str(
        "\n## Declared analysis boundaries\n\n\
         Repeated forms with the same declaration are grouped. Every non-mapped declaration remains here with its code-owned reason.\n\n\
         | Result | Surface | Form | Status | Reason |\n\
         |---|---|---|---|---|\n",
    );
    for row in ANALYSIS_CAPABILITY_MATRIX {
        for surface in NonUiSurface::ALL {
            let capability = row.surface(surface);
            let forms = [
                ("scalar", capability.scalar),
                ("STEP", capability.stepped),
                ("TEMP", capability.temperature),
            ];
            let mut emitted = [false; 3];
            for index in 0..forms.len() {
                if emitted[index] || forms[index].1 == MappingStatus::Mapped {
                    continue;
                }
                emitted[index] = true;
                let mut form_names = forms[index].0.to_string();
                for candidate in (index + 1)..forms.len() {
                    if forms[candidate].1 == forms[index].1 {
                        emitted[candidate] = true;
                        form_names.push_str(", ");
                        form_names.push_str(forms[candidate].0);
                    }
                }
                let status = forms[index].1;
                writeln!(
                    output,
                    "| `{}` | {} | {} | {} | {} |",
                    row.result.tag(),
                    surface.heading(),
                    form_names,
                    status.label(),
                    status.note().expect("non-mapped declaration has a reason"),
                )
                .expect("writing to String cannot fail");
            }
        }
    }
}

/// Generate the checked-in, human-readable matrix from the code registry.
#[must_use]
pub fn render_non_ui_capability_matrix() -> String {
    let mut output = String::from(
        "# RSpice non-UI capability matrix\n\n\
         <!-- Generated by rspice_core::execution::render_non_ui_capability_matrix. -->\n\n\
         This is an adapter inventory, not a claim of numerical qualification. `mapped` means an explicit typed/structured adapter exists for the public result shape. `partial` means an execution/export path exists but loses structure, metadata, or orchestration semantics. `unsupported` is an intentional fail-closed boundary. STEP and TEMP describe authored shared-deck-axis orchestration, not a caller manually editing and rerunning a scalar deck. Analysis cells are ordered **scalar / STEP / TEMP**; every non-mapped reason is listed below the compact table.\n\n\
         ## Analysis result families\n\n\
         | Result | CLI | Python | WASM | Engine adapter |\n\
         |---|---|---|---|---|\n",
    );
    for row in ANALYSIS_CAPABILITY_MATRIX {
        writeln!(
            output,
            "| `{}` | {} | {} | {} | {} |",
            row.result.tag(),
            render_surface(row.cli),
            render_surface(row.python),
            render_surface(row.wasm),
            render_surface(row.engine_adapter),
        )
        .expect("writing to String cannot fail");
    }

    render_analysis_boundaries(&mut output);

    output.push_str(
        "\n## Signal descriptor families\n\n\
         | Signal | CLI | Python | WASM | Engine adapter |\n\
         |---|---|---|---|---|\n",
    );
    for row in SIGNAL_CAPABILITY_MATRIX {
        writeln!(
            output,
            "| `{}` | {} | {} | {} | {} |",
            signal_tag(row.signal),
            render_status(row.cli),
            render_status(row.python),
            render_status(row.wasm),
            render_status(row.engine_adapter),
        )
        .expect("writing to String cannot fail");
    }

    output.push_str(
        "\n## Declared signal boundaries\n\n\
         | Signal | Surface | Status | Reason |\n\
         |---|---|---|---|\n",
    );
    for row in SIGNAL_CAPABILITY_MATRIX {
        for surface in NonUiSurface::ALL {
            let status = row.surface(surface);
            if let Some(note) = status.note() {
                writeln!(
                    output,
                    "| `{}` | {} | {} | {} |",
                    signal_tag(row.signal),
                    surface.heading(),
                    status.label(),
                    note,
                )
                .expect("writing to String cannot fail");
            }
        }
    }
    output
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

    #[test]
    fn checked_in_matrix_is_generated_from_registry() {
        const CHECKED_IN: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../NON_UI_CAPABILITY_MATRIX.md"
        ));
        assert_eq!(CHECKED_IN, render_non_ui_capability_matrix());
    }
}
