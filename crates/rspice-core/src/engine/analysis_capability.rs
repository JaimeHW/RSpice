//! The analysis capability matrix: circuit family × analysis.
//!
//! One table answers "may this analysis run on a circuit that contains this
//! family of instances", and it is the only place that answer is written. Every
//! analysis entry point asks it through [`Engine::ensure_analysis_capability`]
//! before solver work, and the same table is exported through
//! [`capability_matrix`] so a support table shown to a person is generated
//! from the code that enforces it rather than restated beside it.
//!
//! A cell is one of three answers:
//!
//! * [`CellSupport::Supported`]: every instance of the family takes part, and
//!   the cell names the tests that demonstrate it.
//! * [`CellSupport::Restricted`]: instances take part when an instance-level
//!   condition holds. The cell names the gate that decides it; the gate
//!   either runs here or defers to the analysis's own device-contract queries
//!   (`periodic_capability`, the native charge checks, the transient-noise
//!   render plan), which keep their own refusal tokens.
//! * [`CellSupport::Refused`]: no instance of the family takes part. The gate
//!   refuses on the family's first instance, in deck order, with the cell's
//!   token, and names the analysis, the instance, its module and the reason.
//!
//! The device-contract table beneath this one (`periodic_capability`) stays a
//! separate key space: it describes what a device family contributes to a
//! periodic or descriptor formulation, and a `Restricted` cell with the
//! [`InstanceGate::PeriodicContracts`] gate is answered by it.

use super::{Engine, SimulationError, UnsupportedCapabilityError};
use crate::CircuitData;
use crate::identity::AnalysisKind;

/// A family of instances a circuit can hold, in the order the gate checks
/// them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CapabilityFamily {
    /// Native SPICE devices. Always present.
    SpiceNative,
    /// XSPICE code models whose every port is analog.
    XspiceAnalog,
    /// Every other XSPICE code model, including the converters the builder
    /// inserts at a mixed boundary.
    XspiceEvent,
    /// Runtime-compiled analog Verilog-A instances.
    VerilogARuntime,
    /// Build-time generated Verilog-A compact models.
    VerilogAGenerated,
    /// Verilog-AMS modules with a discrete half, authored connect modules and
    /// pure-digital Verilog modules.
    VerilogAmsMixed,
    /// Real-valued event nets (wreal and real-valued XSPICE).
    RealNumber,
}

impl CapabilityFamily {
    pub const ALL: [Self; 7] = [
        Self::SpiceNative,
        Self::XspiceAnalog,
        Self::XspiceEvent,
        Self::VerilogARuntime,
        Self::VerilogAGenerated,
        Self::VerilogAmsMixed,
        Self::RealNumber,
    ];

    /// Stable identifier for exported tables.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::SpiceNative => "spice-native",
            Self::XspiceAnalog => "xspice-analog",
            Self::XspiceEvent => "xspice-event",
            Self::VerilogARuntime => "veriloga-runtime",
            Self::VerilogAGenerated => "veriloga-generated",
            Self::VerilogAmsMixed => "verilog-ams-mixed",
            Self::RealNumber => "real-number",
        }
    }

    /// How a refusal names an instance of the family.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SpiceNative => "SPICE device",
            Self::XspiceAnalog => "XSPICE analog code model",
            Self::XspiceEvent => "XSPICE event-driven code model",
            Self::VerilogARuntime => "Verilog-A instance",
            Self::VerilogAGenerated => "generated Verilog-A instance",
            Self::VerilogAmsMixed => "mixed Verilog-AMS instance",
            Self::RealNumber => "real-valued event net",
        }
    }
}

/// An analysis the matrix has a column for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CapabilityAnalysis {
    Op,
    Dc,
    Ac,
    Noise,
    Tf,
    Sp,
    Stb,
    Pz,
    Sens,
    Disto,
    Tran,
    /// A transient run with `NOISEFMAX=` device noise.
    TranNoise,
    Fourier,
    Fft,
    /// Monte Carlo and `.step` repetition of another analysis.
    Sweep,
    Pss,
    Hb,
    Pac,
    Pnoise,
    Pxf,
    Pstb,
    Envelope,
    QuasiPeriodic,
}

impl CapabilityAnalysis {
    pub const ALL: [Self; 23] = [
        Self::Op,
        Self::Dc,
        Self::Ac,
        Self::Noise,
        Self::Tf,
        Self::Sp,
        Self::Stb,
        Self::Pz,
        Self::Sens,
        Self::Disto,
        Self::Tran,
        Self::TranNoise,
        Self::Fourier,
        Self::Fft,
        Self::Sweep,
        Self::Pss,
        Self::Hb,
        Self::Pac,
        Self::Pnoise,
        Self::Pxf,
        Self::Pstb,
        Self::Envelope,
        Self::QuasiPeriodic,
    ];

    /// Stable identifier for exported tables.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Op => "op",
            Self::Dc => "dc",
            Self::Ac => "ac",
            Self::Noise => "noise",
            Self::Tf => "tf",
            Self::Sp => "sp",
            Self::Stb => "stb",
            Self::Pz => "pz",
            Self::Sens => "sens",
            Self::Disto => "disto",
            Self::Tran => "tran",
            Self::TranNoise => "trnoise",
            Self::Fourier => "four",
            Self::Fft => "fft",
            Self::Sweep => "sweep",
            Self::Pss => "pss",
            Self::Hb => "hb",
            Self::Pac => "pac",
            Self::Pnoise => "pnoise",
            Self::Pxf => "pxf",
            Self::Pstb => "pstb",
            Self::Envelope => "env",
            Self::QuasiPeriodic => "qp",
        }
    }

    /// The name a refusal's prose gives the analysis.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Op => "operating point",
            Self::Dc => "DC sweep",
            Self::Ac => "AC analysis",
            Self::Noise => "noise analysis",
            Self::Tf => "transfer function",
            Self::Sp => "S-parameter analysis",
            Self::Stb => "STB analysis",
            Self::Pz => "pole-zero analysis",
            Self::Sens => "sensitivity analysis",
            Self::Disto => "distortion analysis",
            Self::Tran => "transient analysis",
            Self::TranNoise => "transient noise",
            Self::Fourier => "Fourier analysis",
            Self::Fft => "FFT analysis",
            Self::Sweep => "parameter sweep",
            Self::Pss => "PSS analysis",
            Self::Hb => "harmonic-balance analysis",
            Self::Pac => "PAC analysis",
            Self::Pnoise => "pnoise analysis",
            Self::Pxf => "PXF analysis",
            Self::Pstb => "PSTB analysis",
            Self::Envelope => "envelope analysis",
            Self::QuasiPeriodic => "quasi-periodic analysis",
        }
    }

    /// The analysis kind a refusal records.
    pub const fn kind(self) -> AnalysisKind {
        match self {
            Self::Op => AnalysisKind::Op,
            Self::Dc => AnalysisKind::Dc,
            Self::Ac => AnalysisKind::Ac,
            Self::Noise => AnalysisKind::Noise,
            Self::Tf => AnalysisKind::TransferFunction,
            Self::Sp => AnalysisKind::Sp,
            Self::Stb => AnalysisKind::Stb,
            Self::Pz => AnalysisKind::PoleZero,
            Self::Sens => AnalysisKind::Sensitivity,
            Self::Disto => AnalysisKind::Distortion,
            Self::Tran | Self::TranNoise => AnalysisKind::Tran,
            Self::Fourier => AnalysisKind::Fourier,
            Self::Fft => AnalysisKind::Fft,
            Self::Sweep => AnalysisKind::MonteCarlo,
            Self::Pss => AnalysisKind::Pss,
            Self::Hb => AnalysisKind::HarmonicBalance,
            Self::Pac => AnalysisKind::Pac,
            Self::Pnoise => AnalysisKind::PNoise,
            Self::Pxf => AnalysisKind::Pxf,
            Self::Pstb => AnalysisKind::Pstb,
            Self::Envelope => AnalysisKind::Envelope,
            Self::QuasiPeriodic => AnalysisKind::Qpss,
        }
    }
}

/// The instance-level condition a [`CellSupport::Restricted`] cell evaluates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceGate {
    /// The analysis's device-contract queries in `periodic_capability`, which
    /// keep their own tokens (`analysis.hb.device`, `analysis.pz.device`, …).
    PeriodicContracts,
    /// A rational `G + sC` descriptor: every Verilog-A device and host
    /// analog half is free of `absdelay`, `zi_*` and `idtmod`.
    RationalDescriptor,
    /// The transient-noise render plan, which refuses its first process it
    /// cannot render.
    TransientNoiseRenderable,
    /// The native device checks every small-signal analysis runs during its
    /// bias preparation (unimplemented charge models, CPL lines, memristors).
    NativeDeviceChecks,
}

impl InstanceGate {
    /// Stable identifier for exported tables.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::PeriodicContracts => "periodic-contracts",
            Self::RationalDescriptor => "rational-descriptor",
            Self::TransientNoiseRenderable => "transient-noise-renderable",
            Self::NativeDeviceChecks => "native-device-checks",
        }
    }
}

/// One cell of the matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellSupport {
    /// Every instance of the family takes part. `evidence` names the tests,
    /// as `tests/<file>.rs::<fn>`, that demonstrate it.
    Supported { evidence: &'static [&'static str] },
    /// Instances take part when `gate` admits them; `condition` says what it
    /// admits and `token` is the refusal it raises when it does not.
    Restricted {
        gate: InstanceGate,
        token: &'static str,
        condition: &'static str,
        evidence: &'static [&'static str],
    },
    /// No instance of the family takes part. `reason` is what a person reads;
    /// `plan` is the roadmap item that closes the cell, internal metadata that
    /// never reaches refusal text — the `token` is how a refusal is tracked.
    Refused {
        token: &'static str,
        reason: &'static str,
        plan: &'static str,
    },
}

impl CellSupport {
    /// Stable identifier for exported tables.
    pub const fn status(self) -> &'static str {
        match self {
            Self::Supported { .. } => "supported",
            Self::Restricted { .. } => "restricted",
            Self::Refused { .. } => "refused",
        }
    }
}

/// The test that runs every supported cell on a fixture of its family.
const FIXTURES: &str =
    "tests/capability_refusals.rs::every_supported_cell_runs_on_its_family_fixture";
/// The tests that pin a native device refusal kept behind a restricted cell.
const NATIVE_REFUSALS: &str =
    "tests/capability_refusals.rs::noise_refuses_a_charge_model_it_does_not_implement";
const PERIODIC_REFUSALS: &str =
    "tests/capability_refusals.rs::harmonic_balance_refuses_a_device_it_cannot_stamp";
const PZ_REFUSALS: &str = "tests/capability_refusals.rs::pole_zero_refuses_transmission_lines";
const MIXED_HELD: &str =
    "tests/mixed_small_signal_held.rs::the_held_trim_register_matches_its_held_source_twin";
const MIXED_XSPICE_HELD: &str =
    "tests/mixed_small_signal_held.rs::the_xspice_trim_register_matches_its_held_source_twin";
const MIXED_DC: &str =
    "tests/mixed_dc_sweep_state.rs::a_sweep_carries_the_comparator_latch_in_sweep_order";
const XSPICE_DC: &str =
    "tests/mixed_dc_sweep_state.rs::the_xspice_latch_twin_reads_its_time_zero_state_at_every_point";
const TRANSIENT_NOISE_VERILOGA: &str =
    "tests/transient_noise_veriloga.rs::mixed_and_runtime_module_noise_reach_the_transient";

/// The reason periodic analyses refuse a running discrete state.
const DISCRETE_STATE_REASON: &str = "a periodic steady state with a running discrete state has no formulation yet: the digital half's event queue, process resumptions and drivers are not part of a period map";
const DISCRETE_STATE: &str = "analysis.mixed_signal.discrete_state";
const PERIODIC_VERILOGA: &str = "analysis.periodic.veriloga";

/// Every analysis with no family-specific restriction.
const fn supported() -> CellSupport {
    CellSupport::Supported {
        evidence: &[FIXTURES],
    }
}

/// Native devices keep their device-specific small-signal refusals.
const fn native_checks(token: &'static str) -> CellSupport {
    CellSupport::Restricted {
        gate: InstanceGate::NativeDeviceChecks,
        token,
        condition: "every native device model whose charge and small-signal equations this build implements",
        evidence: &[FIXTURES, NATIVE_REFUSALS],
    }
}

const fn periodic_contracts(token: &'static str, condition: &'static str) -> CellSupport {
    CellSupport::Restricted {
        gate: InstanceGate::PeriodicContracts,
        token,
        condition,
        evidence: &[FIXTURES, PERIODIC_REFUSALS],
    }
}

const fn renderable(evidence: &'static [&'static str]) -> CellSupport {
    CellSupport::Restricted {
        gate: InstanceGate::TransientNoiseRenderable,
        token: "analysis.tran.noise",
        // Shaped Verilog-A processes arrive with P1.5 (L1b).
        condition: "every noise mechanism the transient-noise render plan can render: white, flicker and tabulated sources (shaped Verilog-A noise processes are not rendered yet)",
        evidence,
    }
}

/// The answer of one cell. Exhaustive over both axes, with no catch-all arm,
/// so a new family or analysis does not compile until every cell of it is
/// decided.
pub const fn capability_cell(
    family: CapabilityFamily,
    analysis: CapabilityAnalysis,
) -> CellSupport {
    use CapabilityAnalysis as A;
    use CapabilityFamily as F;
    match analysis {
        A::Op | A::Tran | A::Fourier | A::Fft | A::Sweep => match family {
            F::SpiceNative
            | F::XspiceAnalog
            | F::XspiceEvent
            | F::VerilogARuntime
            | F::VerilogAGenerated
            | F::RealNumber => supported(),
            F::VerilogAmsMixed => CellSupport::Supported {
                evidence: &[FIXTURES, MIXED_HELD],
            },
        },
        A::Dc => match family {
            F::SpiceNative
            | F::XspiceAnalog
            | F::VerilogARuntime
            | F::VerilogAGenerated
            | F::RealNumber => supported(),
            F::XspiceEvent => CellSupport::Supported {
                evidence: &[FIXTURES, XSPICE_DC],
            },
            F::VerilogAmsMixed => CellSupport::Supported {
                evidence: &[FIXTURES, MIXED_DC],
            },
        },
        A::Ac | A::Noise | A::Tf | A::Sp | A::Stb | A::Sens | A::Disto => match family {
            F::SpiceNative => native_checks("analysis.small_signal.device"),
            F::XspiceAnalog | F::VerilogARuntime | F::VerilogAGenerated | F::RealNumber => {
                supported()
            }
            F::XspiceEvent => CellSupport::Supported {
                evidence: &[FIXTURES, MIXED_XSPICE_HELD],
            },
            F::VerilogAmsMixed => CellSupport::Supported {
                evidence: &[FIXTURES, MIXED_HELD],
            },
        },
        A::Pz => match family {
            F::SpiceNative => native_checks("analysis.pz.device"),
            // The real-number boundary models are XSPICE code models and
            // declare their descriptor the same way; the design's `S` for
            // them waits on the descriptor wiring of P1.6 (L2b).
            F::XspiceAnalog | F::XspiceEvent | F::VerilogARuntime | F::RealNumber => {
                CellSupport::Restricted {
                    gate: InstanceGate::PeriodicContracts,
                    token: "analysis.pz.device",
                    condition: "every instance whose dynamic state exports a complete rational G + sC descriptor",
                    evidence: &[FIXTURES, PZ_REFUSALS],
                }
            }
            F::VerilogAGenerated => supported(),
            F::VerilogAmsMixed => CellSupport::Refused {
                token: DISCRETE_STATE,
                reason: "the descriptor of a held discrete state is not wired into pole-zero extraction yet",
                plan: "P1.6 (L2b)",
            },
        },
        A::TranNoise => match family {
            F::SpiceNative => renderable(&[FIXTURES]),
            F::XspiceAnalog | F::XspiceEvent | F::RealNumber => supported(),
            F::VerilogARuntime | F::VerilogAGenerated | F::VerilogAmsMixed => {
                renderable(&[FIXTURES, TRANSIENT_NOISE_VERILOGA])
            }
        },
        A::Pss | A::Hb | A::Pac | A::Pnoise | A::Pxf | A::Pstb => match family {
            F::SpiceNative | F::XspiceAnalog => periodic_contracts(
                "analysis.periodic.device",
                "every instance whose declared periodic contracts admit it",
            ),
            F::XspiceEvent | F::RealNumber => CellSupport::Refused {
                token: DISCRETE_STATE,
                reason: DISCRETE_STATE_REASON,
                plan: "P7.2",
            },
            F::VerilogARuntime => CellSupport::Refused {
                token: PERIODIC_VERILOGA,
                reason: periodic_veriloga_reason(analysis, false),
                plan: "P7.1",
            },
            F::VerilogAGenerated => CellSupport::Refused {
                token: PERIODIC_VERILOGA,
                reason: periodic_veriloga_reason(analysis, true),
                plan: "P7.1",
            },
            F::VerilogAmsMixed => CellSupport::Refused {
                token: DISCRETE_STATE,
                reason: DISCRETE_STATE_REASON,
                plan: "P7.2/P7.3",
            },
        },
        A::Envelope | A::QuasiPeriodic => match family {
            F::SpiceNative | F::XspiceAnalog => periodic_contracts(
                "analysis.periodic.device",
                "every instance whose declared envelope and periodic contracts admit it",
            ),
            F::XspiceEvent | F::VerilogAmsMixed | F::RealNumber => CellSupport::Refused {
                token: DISCRETE_STATE,
                reason: DISCRETE_STATE_REASON,
                plan: "P7.3",
            },
            F::VerilogARuntime => CellSupport::Refused {
                token: PERIODIC_VERILOGA,
                reason: periodic_veriloga_reason(analysis, false),
                plan: "P7.1",
            },
            F::VerilogAGenerated => CellSupport::Refused {
                token: PERIODIC_VERILOGA,
                reason: periodic_veriloga_reason(analysis, true),
                plan: "P7.1",
            },
        },
    }
}

/// Why a periodic analysis refuses an analog Verilog-A instance, in the words
/// of the device-contract declaration it answers from.
const fn periodic_veriloga_reason(analysis: CapabilityAnalysis, generated: bool) -> &'static str {
    use CapabilityAnalysis as A;
    match (analysis, generated) {
        (A::Hb | A::QuasiPeriodic, false) => {
            "runtime Verilog-A devices without exact HB charge/noise capability metadata"
        }
        (A::Hb | A::QuasiPeriodic, true) => "generated Verilog-A compact-model equations",
        (A::Pss | A::Pstb, false) => "Verilog-A integration state is not part of a period map",
        (A::Pss | A::Pstb, true) => "generated Verilog-A integration state",
        (A::Pnoise, false) => "periodic Verilog-A noise sources are not declared",
        (A::Pnoise, true) => "periodic generated Verilog-A noise sources are not declared",
        (A::Envelope, false) => "Verilog-A integration state is not part of an envelope",
        (A::Envelope, true) => "generated Verilog-A integration state",
        (_, false) => "runtime Verilog-A equations have no periodic small-signal formulation",
        (_, true) => "generated Verilog-A compact-model equations",
    }
}

/// The whole matrix, family-major in [`CapabilityFamily::ALL`] order and then
/// [`CapabilityAnalysis::ALL`] order.
pub fn capability_matrix()
-> impl Iterator<Item = (CapabilityFamily, CapabilityAnalysis, CellSupport)> {
    CapabilityFamily::ALL.into_iter().flat_map(|family| {
        CapabilityAnalysis::ALL
            .into_iter()
            .map(move |analysis| (family, analysis, capability_cell(family, analysis)))
    })
}

/// The first instance of a family, as `(instance, module)`, in deck order.
fn first_instance(
    circuit: &CircuitData,
    family: CapabilityFamily,
) -> Option<(String, Option<String>)> {
    match family {
        CapabilityFamily::SpiceNative => None,
        CapabilityFamily::XspiceAnalog => circuit
            .xspice_instances
            .iter()
            .find(|instance| xspice_is_analog_only(instance))
            .map(|instance| {
                (
                    instance.name.clone(),
                    Some(instance.model_name().to_owned()),
                )
            }),
        CapabilityFamily::XspiceEvent => circuit
            .xspice_instances
            .iter()
            .find(|instance| !xspice_is_analog_only(instance))
            .map(|instance| {
                (
                    instance.name.clone(),
                    Some(instance.model_name().to_owned()),
                )
            }),
        CapabilityFamily::VerilogARuntime => {
            #[cfg(feature = "veriloga")]
            {
                circuit.veriloga_devices().iter().next().map(|device| {
                    (
                        device.name.to_string(),
                        Some(device.model_name().to_owned()),
                    )
                })
            }
            #[cfg(not(feature = "veriloga"))]
            {
                None
            }
        }
        CapabilityFamily::VerilogAGenerated => {
            #[cfg(feature = "veriloga-builtins-base")]
            {
                circuit
                    .generated_veriloga_devices()
                    .iter()
                    .next()
                    .map(|device| {
                        (
                            device.instance_name.clone(),
                            Some(device.model_name.to_owned()),
                        )
                    })
            }
            #[cfg(not(feature = "veriloga-builtins-base"))]
            {
                None
            }
        }
        CapabilityFamily::VerilogAmsMixed => {
            #[cfg(feature = "veriloga")]
            {
                circuit.mixed_signal_hosts.first().map(|host| {
                    (
                        host.instance_name().to_owned(),
                        Some(host.analog_device().model_name().to_owned()),
                    )
                })
            }
            #[cfg(not(feature = "veriloga"))]
            {
                None
            }
        }
        CapabilityFamily::RealNumber => {
            let node = circuit.net_kinds.discrete_nodes().find(|&node| {
                matches!(
                    circuit.net_kinds.kind(node),
                    crate::circuit::NetKind::Real | crate::circuit::NetKind::DigitalAndReal
                )
            })?;
            let name = circuit
                .node_names_sorted()
                .get(node - 1)
                .cloned()
                .unwrap_or_else(|| format!("node {node}"));
            Some((name, None))
        }
    }
}

/// Whether every port of a code model is analog — the trait default
/// `CodeModel::is_analog_only` reads, answered from the instance's port
/// contract.
fn xspice_is_analog_only(instance: &crate::xspice::XspiceInstance) -> bool {
    instance
        .ports()
        .iter()
        .all(|port| port.default_type.is_analog())
}

/// The refusal of one restricted cell's gate, evaluated here.
fn rational_descriptor_refusal(
    circuit: &CircuitData,
    analysis: CapabilityAnalysis,
) -> Option<UnsupportedCapabilityError> {
    #[cfg(feature = "veriloga")]
    {
        let devices = circuit
            .veriloga_devices()
            .iter()
            .map(|device| (device.name.to_string(), device));
        let hosts = circuit
            .mixed_signal_hosts
            .iter()
            .map(|host| (host.instance_name().to_owned(), host.analog_device()));
        for (instance, device) in devices.chain(hosts) {
            let blockers = device.small_signal_descriptor_blockers();
            if let Some(&operator) = blockers.first() {
                let module = device.model_name().to_owned();
                return Some(
                    UnsupportedCapabilityError::new(
                        "analysis.pz.non_rational_operator",
                        format!(
                            "{} does not support Verilog-A instance '{instance}' (module \
                             '{module}'): it uses {operator}, which has no finite rational \
                             G + sC descriptor",
                            analysis.label()
                        ),
                    )
                    .for_analysis_kind(analysis.kind())
                    .for_instance(instance)
                    .for_module(module),
                );
            }
        }
    }
    let _ = (circuit, analysis);
    None
}

/// The order the gate checks families in: the author's own constructs before
/// what elaboration inserted for them. A mixed module's built-in converters
/// are XSPICE event models the deck never names, so a refusal that reached
/// one of them first would name `__rspice_auto_dac_3` where the author wrote
/// `x1`; the mixed host is checked first, and the native devices last.
const REFUSAL_ORDER: [CapabilityFamily; 7] = [
    CapabilityFamily::VerilogAmsMixed,
    CapabilityFamily::RealNumber,
    CapabilityFamily::VerilogAGenerated,
    CapabilityFamily::VerilogARuntime,
    CapabilityFamily::XspiceEvent,
    CapabilityFamily::XspiceAnalog,
    CapabilityFamily::SpiceNative,
];

impl Engine {
    /// The dispatch gate every analysis asks before solver work.
    ///
    /// Families are checked in [`REFUSAL_ORDER`], and the first failing cell
    /// decides the refusal. A refusal always records the analysis kind, and
    /// the instance and module it declined.
    pub(in crate::engine) fn ensure_analysis_capability(
        circuit: &CircuitData,
        analysis: CapabilityAnalysis,
    ) -> Result<(), SimulationError> {
        for family in REFUSAL_ORDER {
            match capability_cell(family, analysis) {
                CellSupport::Supported { .. } => {}
                CellSupport::Restricted { gate, .. } => match gate {
                    // Answered by the analysis's own device-contract queries and
                    // native checks, which keep their tokens; and by the
                    // transient-noise render plan at install.
                    InstanceGate::PeriodicContracts
                    | InstanceGate::NativeDeviceChecks
                    | InstanceGate::TransientNoiseRenderable => {}
                    InstanceGate::RationalDescriptor => {
                        if let Some(refusal) = rational_descriptor_refusal(circuit, analysis) {
                            return Err(refusal.into());
                        }
                    }
                },
                // The roadmap item stays out of the text: the token tracks it.
                CellSupport::Refused {
                    token,
                    reason,
                    plan: _,
                } => {
                    let Some((instance, module)) = first_instance(circuit, family) else {
                        continue;
                    };
                    let module_text = module
                        .as_deref()
                        .map(|module| format!(" (module '{module}')"))
                        .unwrap_or_default();
                    let mut refusal = UnsupportedCapabilityError::new(
                        token,
                        format!(
                            "{} '{instance}'{module_text} cannot take part in {}: {reason}",
                            family.label(),
                            analysis.label(),
                        ),
                    )
                    .for_analysis_kind(analysis.kind())
                    .for_instance(instance);
                    if let Some(module) = module {
                        refusal = refusal.for_module(module);
                    }
                    return Err(refusal.into());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Whether text names a roadmap item (`P1.6`, `P7.2/P7.3`) or a lane
    /// (`L2b`, `lane L5`), which user-facing text must never do.
    fn names_a_plan_item(text: &str) -> bool {
        let bytes = text.as_bytes();
        let item = bytes.windows(4).any(|window| {
            window[0] == b'P'
                && window[1].is_ascii_digit()
                && window[2] == b'.'
                && window[3].is_ascii_digit()
        });
        let lane = bytes.windows(3).any(|window| {
            matches!(window[0], b'(' | b' ') && window[1] == b'L' && window[2].is_ascii_digit()
        });
        item || lane || text.contains("planned")
    }

    #[test]
    fn every_cell_is_decided_and_refusals_name_a_plan() {
        let cells = capability_matrix().collect::<Vec<_>>();
        assert_eq!(
            cells.len(),
            CapabilityFamily::ALL.len() * CapabilityAnalysis::ALL.len()
        );
        for (family, analysis, cell) in cells {
            match cell {
                CellSupport::Supported { evidence } => assert!(
                    !evidence.is_empty(),
                    "{family:?} × {analysis:?} claims support without evidence"
                ),
                CellSupport::Restricted {
                    token,
                    condition,
                    evidence,
                    ..
                } => {
                    assert!(token.starts_with("analysis."), "{family:?} × {analysis:?}");
                    assert!(!condition.is_empty(), "{family:?} × {analysis:?}");
                    assert!(!evidence.is_empty(), "{family:?} × {analysis:?}");
                    assert!(
                        !names_a_plan_item(condition),
                        "{family:?} × {analysis:?}: a person reads the condition: {condition}"
                    );
                }
                CellSupport::Refused {
                    token,
                    reason,
                    plan,
                } => {
                    assert!(token.starts_with("analysis."), "{family:?} × {analysis:?}");
                    assert!(!reason.is_empty() && !plan.is_empty());
                    assert!(
                        !names_a_plan_item(reason),
                        "{family:?} × {analysis:?}: a person reads the reason: {reason}"
                    );
                }
            }
        }
        for family in CapabilityFamily::ALL {
            assert!(
                matches!(
                    capability_cell(family, CapabilityAnalysis::Op),
                    CellSupport::Supported { .. }
                ),
                "every family has an operating point"
            );
        }
    }

    /// Every evidence path names a test file and a function in it that exists,
    /// so a claim cannot outlive the test it cites.
    #[test]
    fn every_cited_evidence_test_exists() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut cited = std::collections::BTreeSet::new();
        for (_, _, cell) in capability_matrix() {
            let evidence = match cell {
                CellSupport::Supported { evidence } | CellSupport::Restricted { evidence, .. } => {
                    evidence
                }
                CellSupport::Refused { .. } => &[][..],
            };
            cited.extend(evidence.iter().copied());
        }
        for path in cited {
            let (file, function) = path
                .split_once("::")
                .unwrap_or_else(|| panic!("evidence '{path}' is not <file>::<fn>"));
            assert!(
                file.starts_with("tests/") && file.ends_with(".rs"),
                "evidence '{path}' is not an integration test"
            );
            let source = std::fs::read_to_string(root.join(file))
                .unwrap_or_else(|error| panic!("evidence file '{file}' is unreadable: {error}"));
            assert!(
                source.contains(&format!("fn {function}(")),
                "evidence '{path}' names no function in {file}"
            );
        }
    }

    #[test]
    fn kinds_and_tags_are_distinct() {
        let tags = CapabilityAnalysis::ALL
            .iter()
            .map(|analysis| analysis.tag())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(tags.len(), CapabilityAnalysis::ALL.len());
        assert_eq!(CapabilityAnalysis::TranNoise.kind(), AnalysisKind::Tran);
        assert_eq!(CapabilityAnalysis::Sweep.kind(), AnalysisKind::MonteCarlo);
        assert_eq!(CapabilityAnalysis::QuasiPeriodic.kind(), AnalysisKind::Qpss);
        let mut order = REFUSAL_ORDER.map(CapabilityFamily::tag);
        let mut all = CapabilityFamily::ALL.map(CapabilityFamily::tag);
        order.sort_unstable();
        all.sort_unstable();
        assert_eq!(order, all, "the gate checks every family exactly once");
    }
}
