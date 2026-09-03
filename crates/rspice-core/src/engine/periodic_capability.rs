//! Native periodic-analysis device capability descriptors.
//!
//! Harmonic balance, PAC, periodic noise, PSS continuation, HB envelope
//! continuation and pole-zero descriptor extraction each need to know, before
//! any solver work, whether every device in the circuit can take part. That
//! question used to be answered by four hand-maintained family lists spread
//! across `engine::hb::drive`, `engine::hb::state` and `engine::pss`, one of
//! which carried a comment asking the next reader to keep it aligned by hand
//! with the transient residual and state-commit code. A family added to the
//! circuit store but forgotten in one of those lists silently acquired support
//! it did not have.
//!
//! This module replaces the lists with one declaration table. Every native
//! device/element family in [`CircuitData`] appears exactly once in
//! [`PeriodicDeviceFamily`], and [`periodic_capability_descriptor`] answers
//! for it with an exhaustive `match`, so a new family cannot compile without a
//! declaration. Each descriptor states, for the six contracts of
//! [`PeriodicCapability`], whether the family's contribution is complete,
//! restricted to instances satisfying a documented condition, absent, or
//! inapplicable because the family has nothing to contribute.
//!
//! The declarations record *today's implementation status*, not an aspiration.
//! A family whose exact periodic residual only covers LEVEL=1 instances says
//! so, and the corresponding gap query reports the missing capability by name
//! instead of naming the device family alone.
//!
//! # Extension point
//!
//! Verilog-A capability metadata is owned by a separate effort. This module
//! must not depend on it, so it exposes a registry
//! ([`register_external_capability_provider`]) of
//! [`ExternalPeriodicCapabilityProvider`] trait objects keyed by family. When
//! a provider declares a contract complete for a family, the gap queries below
//! stop reporting it. Nothing in this crate registers one yet: the hook exists
//! so that Verilog-A capability metadata never has to reopen this table.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::circuit::CircuitData;

/// The six native periodic-analysis device contracts.
///
/// Each is queried by the analyses that actually need it, and a family may
/// satisfy one and not another: a diode's exact periodic residual covers only
/// its LEVEL=1 form, while a lossless delay line has an exact periodic
/// descriptor but no finite explicit dynamic state for pole-zero extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum PeriodicCapability {
    /// Exact periodic residual and analytic Jacobian for a nonlinear device
    /// family, evaluated over the harmonic state rather than a single bias.
    PeriodicResidualJacobian,
    /// Charge and dynamic-state contribution as an explicit finite state in a
    /// rational `G + sC` descriptor. Pole-zero extraction needs this; an
    /// irrational (delay) or hidden-state contribution cannot supply it.
    DynamicStateDescriptor,
    /// Small-signal/PAC descriptor contribution to the exact periodic MNA
    /// system: the branch equations the harmonic solver stamps directly.
    PeriodicSmallSignalDescriptor,
    /// Stationary and cyclostationary noise sources and correlations,
    /// including sideband folding of a periodically modulated source.
    NoiseSources,
    /// Capture and restore of the family's state across the PSS period map,
    /// so a converged shooting solution can be continued in the time domain.
    PssStateMap,
    /// Initialization and continuation of the family's state for HB envelope
    /// analysis.
    EnvelopeContinuation,
}

impl PeriodicCapability {
    /// Every contract, for the tests that must cover all six.
    #[cfg(test)]
    pub(crate) const ALL: [Self; 6] = [
        Self::PeriodicResidualJacobian,
        Self::DynamicStateDescriptor,
        Self::PeriodicSmallSignalDescriptor,
        Self::NoiseSources,
        Self::PssStateMap,
        Self::EnvelopeContinuation,
    ];
}

/// Every native device/element family the circuit store can hold.
///
/// The list is exhaustive by construction: [`periodic_capability_descriptor`]
/// matches on it without a wildcard arm, so adding a variant fails to compile
/// until its six declarations exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum PeriodicDeviceFamily {
    Resistor,
    ResistorBranch,
    Capacitor,
    Inductor,
    VoltageSource,
    CurrentSource,
    Vcvs,
    Vccs,
    Cccs,
    Ccvs,
    Diode,
    Bjt,
    Mosfet,
    Bsim3v3,
    Bsim4v8,
    B3SoiDd,
    B3SoiFd,
    B3SoiPd,
    Ekv26,
    Ekv3,
    Vdmos,
    Jfet,
    XyceMemristor,
    VoltageSwitch,
    CurrentSwitch,
    GenericSwitch,
    TransmissionLine,
    CoupledTransmissionLine,
    InductorCoupling,
    CoupledInductorPair,
    MultiWindingTransformer,
    JilesAthertonInductor,
    XyceCoreGroup,
    BehavioralSource,
    XspiceInstance,
    RuntimeVerilogA,
    GeneratedVerilogA,
}

impl PeriodicDeviceFamily {
    pub(crate) const ALL: [Self; 37] = [
        Self::Resistor,
        Self::ResistorBranch,
        Self::Capacitor,
        Self::Inductor,
        Self::VoltageSource,
        Self::CurrentSource,
        Self::Vcvs,
        Self::Vccs,
        Self::Cccs,
        Self::Ccvs,
        Self::Diode,
        Self::Bjt,
        Self::Mosfet,
        Self::Bsim3v3,
        Self::Bsim4v8,
        Self::B3SoiDd,
        Self::B3SoiFd,
        Self::B3SoiPd,
        Self::Ekv26,
        Self::Ekv3,
        Self::Vdmos,
        Self::Jfet,
        Self::XyceMemristor,
        Self::VoltageSwitch,
        Self::CurrentSwitch,
        Self::GenericSwitch,
        Self::TransmissionLine,
        Self::CoupledTransmissionLine,
        Self::InductorCoupling,
        Self::CoupledInductorPair,
        Self::MultiWindingTransformer,
        Self::JilesAthertonInductor,
        Self::XyceCoreGroup,
        Self::BehavioralSource,
        Self::XspiceInstance,
        Self::RuntimeVerilogA,
        Self::GeneratedVerilogA,
    ];

    /// Human-readable family name used in rejection messages.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Resistor => "resistors",
            Self::ResistorBranch => "zero-resistance MNA branches",
            Self::Capacitor => "capacitors",
            Self::Inductor => "inductors",
            Self::VoltageSource => "independent voltage sources",
            Self::CurrentSource => "independent current sources",
            Self::Vcvs => "voltage-controlled voltage sources",
            Self::Vccs => "voltage-controlled current sources",
            Self::Cccs => "current-controlled current sources",
            Self::Ccvs => "current-controlled voltage sources",
            Self::Diode => "diodes",
            Self::Bjt => "native BJT/VBIC devices",
            Self::Mosfet => "classic MOS devices",
            Self::Bsim3v3 => "native BSIM3v3 devices",
            Self::Bsim4v8 => "native BSIM4 devices",
            Self::B3SoiDd => "native B3SOI DD devices",
            Self::B3SoiFd => "native B3SOI FD devices",
            Self::B3SoiPd => "native B3SOI PD devices",
            Self::Ekv26 => "native EKV 2.6 devices",
            Self::Ekv3 => "native EKV3 devices",
            Self::Vdmos => "native VDMOS devices",
            Self::Jfet => "JFET devices",
            Self::XyceMemristor => "native Xyce memristors",
            Self::VoltageSwitch => "voltage-controlled switches",
            Self::CurrentSwitch => "current-controlled switches",
            Self::GenericSwitch => "generic SWITCH CONTROL devices",
            Self::TransmissionLine => "scalar transmission lines",
            Self::CoupledTransmissionLine => "coupled transmission lines",
            Self::InductorCoupling => "coupled inductors",
            Self::CoupledInductorPair => "coupled inductor pairs",
            Self::MultiWindingTransformer => "multi-winding transformers",
            Self::JilesAthertonInductor => "Jiles-Atherton magnetic cores",
            Self::XyceCoreGroup => "shared Xyce magnetic cores",
            Self::BehavioralSource => "behavioral sources",
            Self::XspiceInstance => "XSPICE devices",
            Self::RuntimeVerilogA => "runtime Verilog-A devices",
            Self::GeneratedVerilogA => "generated Verilog-A devices",
        }
    }

    /// Number of instances of this family in a circuit.
    ///
    /// Feature-gated families report zero when their feature is off, so the
    /// enum stays exhaustive in every build configuration.
    pub(crate) fn instance_count(self, circuit: &CircuitData) -> usize {
        match self {
            Self::Resistor => circuit.resistors.len(),
            Self::ResistorBranch => circuit.resistor_branches.len(),
            Self::Capacitor => circuit.capacitors.len(),
            Self::Inductor => circuit.inductors.len(),
            Self::VoltageSource => circuit.voltage_sources.len(),
            Self::CurrentSource => circuit.current_sources.len(),
            Self::Vcvs => circuit.vcvs.len(),
            Self::Vccs => circuit.vccs.len(),
            Self::Cccs => circuit.cccs.len(),
            Self::Ccvs => circuit.ccvs.len(),
            Self::Diode => circuit.diodes.len(),
            Self::Bjt => circuit.bjts.len(),
            Self::Mosfet => circuit.mosfets.len(),
            Self::Bsim3v3 => circuit.bsim3v3.len(),
            Self::Bsim4v8 => circuit.bsim4v8.len(),
            Self::B3SoiDd => circuit.b3soi.len(),
            Self::B3SoiFd => circuit.b3soi_fd.len(),
            Self::B3SoiPd => circuit.b3soi_pd.len(),
            Self::Ekv26 => circuit.ekv26s.len(),
            Self::Ekv3 => circuit.ekv3s.len(),
            Self::Vdmos => circuit.vdmoses.len(),
            Self::Jfet => circuit.jfets.len(),
            Self::XyceMemristor => circuit.xyce_memristors.len(),
            Self::VoltageSwitch => circuit.vswitches.len(),
            Self::CurrentSwitch => circuit.iswitches.len(),
            Self::GenericSwitch => circuit.generic_switches.len(),
            Self::TransmissionLine => circuit.tlines.len(),
            Self::CoupledTransmissionLine => circuit.coupled_tlines.len(),
            Self::InductorCoupling => circuit.couplings.len(),
            Self::CoupledInductorPair => circuit.coupled_inductor_pairs.len(),
            Self::MultiWindingTransformer => circuit.multi_winding_transformers.len(),
            Self::JilesAthertonInductor => circuit.jiles_atherton_inductors.len(),
            Self::XyceCoreGroup => circuit.xyce_core_groups.len(),
            Self::BehavioralSource => circuit.behavioral_sources.len(),
            Self::XspiceInstance => circuit.xspice_instances.len(),
            Self::RuntimeVerilogA => {
                #[cfg(feature = "veriloga")]
                {
                    circuit.veriloga_devices().len()
                }
                #[cfg(not(feature = "veriloga"))]
                {
                    let _ = circuit;
                    0
                }
            }
            Self::GeneratedVerilogA => {
                #[cfg(feature = "veriloga-builtins-base")]
                {
                    circuit.generated_veriloga_devices.len()
                }
                #[cfg(not(feature = "veriloga-builtins-base"))]
                {
                    let _ = circuit;
                    0
                }
            }
        }
    }
}

/// What a family declares about one contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CapabilitySupport {
    /// The family has nothing to contribute to this contract, so its absence
    /// can never block an analysis. A resistor has no charge state; a
    /// capacitor has no noise source.
    Inapplicable,
    /// Implemented exactly for every instance of the family.
    Complete,
    /// Implemented for instances satisfying the stated condition. Instances
    /// outside it are reported as gaps naming the missing capability.
    Restricted(&'static str),
    /// Not implemented. Every instance is a gap, reported with this text.
    Absent(&'static str),
}

impl CapabilitySupport {
    /// Whether every instance of this family is admitted unconditionally.
    pub(crate) const fn admits_every_instance(self) -> bool {
        matches!(self, Self::Inapplicable | Self::Complete)
    }

    /// Whether an instance can be admitted at all, subject to its condition.
    pub(crate) const fn admits_some_instance(self) -> bool {
        !matches!(self, Self::Absent(_))
    }
}

/// One family's declarations for all six contracts.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PeriodicCapabilityDescriptor {
    residual_jacobian: CapabilitySupport,
    dynamic_state: CapabilitySupport,
    small_signal: CapabilitySupport,
    noise: CapabilitySupport,
    pss_state: CapabilitySupport,
    envelope: CapabilitySupport,
}

impl PeriodicCapabilityDescriptor {
    pub(crate) const fn support(&self, capability: PeriodicCapability) -> CapabilitySupport {
        match capability {
            PeriodicCapability::PeriodicResidualJacobian => self.residual_jacobian,
            PeriodicCapability::DynamicStateDescriptor => self.dynamic_state,
            PeriodicCapability::PeriodicSmallSignalDescriptor => self.small_signal,
            PeriodicCapability::NoiseSources => self.noise,
            PeriodicCapability::PssStateMap => self.pss_state,
            PeriodicCapability::EnvelopeContinuation => self.envelope,
        }
    }
}

use CapabilitySupport::{Absent, Complete, Inapplicable, Restricted};

/// Shared phrase for the envelope initializer's linear subset. The gap query
/// reports the offending family by name; this states why it is a gap.
const ENVELOPE_LINEAR_SUBSET: &str = "the exact envelope initializer supports only ordinary R/C elements and independent \
     voltage/current sources";
const CYCLOSTATIONARY_FLICKER: &str = "stationary thermal/shot noise is exact; a nonzero flicker coefficient needs cyclostationary \
     colored-noise folding rather than a DC-bias substitution";

/// The declaration table.
///
/// Exhaustive on purpose: there is no default arm, so a new family in
/// [`CircuitData`] cannot reach an advanced analysis without someone stating
/// what it can and cannot do.
pub(crate) const fn periodic_capability_descriptor(
    family: PeriodicDeviceFamily,
) -> PeriodicCapabilityDescriptor {
    use PeriodicDeviceFamily as F;
    match family {
        F::Resistor => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Inapplicable,
            small_signal: Complete,
            noise: Restricted(CYCLOSTATIONARY_FLICKER),
            pss_state: Restricted(
                "a resistor without an accepted thermal-state temperature; the Xyce LEVEL=2 \
                 thermal resistor state is not advanced by the shooting period map",
            ),
            envelope: Complete,
        },
        F::ResistorBranch => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Inapplicable,
            small_signal: Complete,
            noise: Restricted(CYCLOSTATIONARY_FLICKER),
            pss_state: Complete,
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Capacitor => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Restricted(
                "a constant capacitance; a solution-dependent capacitor charge linearization is \
                 not represented by the exact periodic MNA descriptor",
            ),
            noise: Inapplicable,
            pss_state: Restricted(
                "a constant capacitance; solution-dependent capacitor charge/expression history \
                 is not advanced by the shooting period map",
            ),
            envelope: Restricted(
                "an ordinary two-terminal capacitor with a constant value and no internal or \
                 IC-constrained branch",
            ),
        },
        F::Inductor => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Complete,
            noise: Inapplicable,
            pss_state: Complete,
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::VoltageSource | F::CurrentSource => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Inapplicable,
            small_signal: Complete,
            noise: Inapplicable,
            pss_state: Complete,
            envelope: Complete,
        },
        F::Vcvs | F::Vccs | F::Cccs | F::Ccvs => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Inapplicable,
            small_signal: Complete,
            noise: Inapplicable,
            pss_state: Complete,
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Diode => PeriodicCapabilityDescriptor {
            residual_jacobian: Restricted(
                "LEVEL=1 junction equations with representable exact-HB parameters, without \
                 high-injection, recombination, sidewall, tunneling or overlap terms",
            ),
            dynamic_state: Complete,
            small_signal: Complete,
            noise: Restricted(CYCLOSTATIONARY_FLICKER),
            pss_state: Absent("diode junction/diffusion charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Bjt => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent(
                "native BJT/VBIC models whose complete Gummel-Poon/VBIC equations are not \
                 represented by exact HB",
            ),
            dynamic_state: Complete,
            small_signal: Inapplicable,
            noise: Absent("periodic BJT noise sources need the exact periodic BJT residual"),
            pss_state: Absent("BJT/VBIC charge and internal-state history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Mosfet => PeriodicCapabilityDescriptor {
            residual_jacobian: Restricted(
                "LEVEL=1 devices with representable physical parameters, the ngspice \
                 reverse-clamp bulk junction, and no sidewall charge on a nonzero perimeter",
            ),
            dynamic_state: Complete,
            small_signal: Complete,
            noise: Restricted(CYCLOSTATIONARY_FLICKER),
            pss_state: Absent("classic MOSFET charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Bsim3v3 => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent("native BSIM3v3"),
            dynamic_state: Restricted(
                "AC-NQS is a rational charge-deficit effect and needs a hidden charge-deficit \
                 state instead of G+sC descriptor extraction, so only ACNQSMOD=0 has a finite \
                 explicit descriptor state",
            ),
            small_signal: Inapplicable,
            noise: Absent("periodic BSIM3 noise sources need the exact periodic BSIM3 residual"),
            pss_state: Absent("BSIM3 charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Bsim4v8 => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent("native BSIM4"),
            dynamic_state: Restricted(
                "AC-NQS is a rational charge-deficit effect and needs a hidden charge-deficit \
                 state instead of G+sC descriptor extraction, so only ACNQSMOD=0 has a finite \
                 explicit descriptor state",
            ),
            small_signal: Inapplicable,
            noise: Absent("periodic BSIM4 noise sources need the exact periodic BSIM4 residual"),
            pss_state: Absent("BSIM4 charge and NQS history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::B3SoiDd => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent("native B3SOI DD"),
            dynamic_state: Complete,
            small_signal: Inapplicable,
            noise: Absent("periodic BSIMSOI noise sources need the exact periodic SOI residual"),
            pss_state: Absent("BSIMSOI charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::B3SoiFd => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent("native B3SOI FD"),
            dynamic_state: Complete,
            small_signal: Inapplicable,
            noise: Absent("periodic BSIMSOI noise sources need the exact periodic SOI residual"),
            pss_state: Absent("BSIMSOI charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::B3SoiPd => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent("native B3SOI PD"),
            dynamic_state: Complete,
            small_signal: Inapplicable,
            noise: Absent("periodic BSIMSOI noise sources need the exact periodic SOI residual"),
            pss_state: Absent("BSIMSOI charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        // EKV and VDMOS have neither a periodic residual nor a periodic MNA
        // stamp. Their gap is reported against the descriptor contract, which
        // is the one the harmonic solver would have to stamp them into.
        F::Ekv26 => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Absent("unstamped advanced semiconductor models"),
            noise: Absent("periodic EKV 2.6 noise sources need an exact periodic residual"),
            pss_state: Absent("EKV 2.6 charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Ekv3 => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Absent("unstamped advanced semiconductor models"),
            noise: Absent("periodic EKV3 noise sources need an exact periodic residual"),
            pss_state: Absent("EKV3 charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Vdmos => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Absent("unstamped advanced semiconductor models"),
            noise: Absent("periodic VDMOS noise sources need an exact periodic residual"),
            pss_state: Absent("VDMOS charge history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::Jfet => PeriodicCapabilityDescriptor {
            residual_jacobian: Restricted(
                "Shichman-Hodges channel equations at M=1, AREA=1, FC=0.5, N=1 and TEMP=TNOM \
                 with representable physical parameters",
            ),
            dynamic_state: Complete,
            small_signal: Complete,
            noise: Restricted(CYCLOSTATIONARY_FLICKER),
            pss_state: Absent("JFET/MESFET charge and trap history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::XyceMemristor => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent("native Xyce memristor"),
            dynamic_state: Absent(
                "the memristive state variable has no validated small-signal dynamic-state \
                 linearization",
            ),
            small_signal: Inapplicable,
            noise: Absent("periodic memristor noise sources need an exact periodic residual"),
            pss_state: Absent("native memristor resistance state"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::VoltageSwitch => PeriodicCapabilityDescriptor {
            residual_jacobian: Restricted(
                "non-hysteretic switches (VH=0) with representable physical parameters and \
                 without Xyce ON/OFF curve semantics",
            ),
            dynamic_state: Inapplicable,
            small_signal: Complete,
            noise: Inapplicable,
            pss_state: Absent("switch hysteresis state"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::CurrentSwitch => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent(
                "current-controlled switches requiring exact control-branch current spectra",
            ),
            dynamic_state: Inapplicable,
            small_signal: Inapplicable,
            noise: Inapplicable,
            pss_state: Absent("switch hysteresis state"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::GenericSwitch => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent(
                "generic SWITCH CONTROL devices whose expression-controlled equations have no \
                 exact periodic residual",
            ),
            dynamic_state: Inapplicable,
            small_signal: Inapplicable,
            noise: Inapplicable,
            pss_state: Absent("switch hysteresis state"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::TransmissionLine => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Restricted(
                "a memoryless line contributes no dynamic state at all; every line with \
                 propagation delay has an irrational descriptor and no finite explicit state",
            ),
            small_signal: Restricted(
                "lines with exact frequency-domain data: memoryless RG and LEN=0 through \
                 connections, LTRA RLC/RC lines, and lossless delay lines",
            ),
            noise: Inapplicable,
            pss_state: Restricted(
                "a memoryless line (finite-length RG, or the LEN=0 ideal through connection) has \
                 no propagation history to carry; every other line's delay history is not \
                 captured",
            ),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::CoupledTransmissionLine => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Restricted("CPL lines that retain full physical lossless RLGC matrices"),
            noise: Inapplicable,
            pss_state: Absent("coupled transmission-line convolution history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::InductorCoupling | F::CoupledInductorPair => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Complete,
            noise: Inapplicable,
            pss_state: Absent("coupled-inductor mutual history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::MultiWindingTransformer => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Complete,
            noise: Inapplicable,
            pss_state: Absent("multi-winding transformer history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::JilesAthertonInductor => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Absent("nonlinear magnetic-core branch equations"),
            noise: Inapplicable,
            pss_state: Absent("Jiles-Atherton hysteretic magnetic history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::XyceCoreGroup => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Absent("nonlinear magnetic-core branch equations"),
            noise: Inapplicable,
            pss_state: Absent("shared Xyce Core hysteretic magnetic history"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::BehavioralSource => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Absent("behavioral-source equations"),
            noise: Inapplicable,
            pss_state: Restricted(
                "behavioral sources without `sdt` integrals; accepted-step memory is not \
                 captured",
            ),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::XspiceInstance => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Absent("XSPICE code-model equations"),
            noise: Inapplicable,
            pss_state: Absent("XSPICE accepted-step and event state"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::RuntimeVerilogA => PeriodicCapabilityDescriptor {
            residual_jacobian: Absent(
                "runtime Verilog-A devices without exact HB charge/noise capability metadata",
            ),
            dynamic_state: Complete,
            small_signal: Inapplicable,
            noise: Absent("periodic Verilog-A noise sources are not declared"),
            pss_state: Absent("Verilog-A integration state"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
        F::GeneratedVerilogA => PeriodicCapabilityDescriptor {
            residual_jacobian: Inapplicable,
            dynamic_state: Complete,
            small_signal: Absent("generated Verilog-A compact-model equations"),
            noise: Absent("periodic generated Verilog-A noise sources are not declared"),
            pss_state: Absent("generated Verilog-A integration state"),
            envelope: Absent(ENVELOPE_LINEAR_SUBSET),
        },
    }
}

// ---------------------------------------------------------------------------
// Instance-level conditions behind the `Restricted` declarations
// ---------------------------------------------------------------------------

/// Whether a diode's authored equations reduce exactly to the LEVEL=1 junction
/// form the exact periodic residual implements.
fn diode_has_exact_periodic_residual_form(diode: &crate::device::Diode) -> bool {
    diode.level == crate::device::DiodeLevel::Legacy
        && diode.forward_knee_current <= 0.0
        && diode.reverse_knee_current <= 0.0
        && diode.recombination_saturation_current == 0.0
        && diode.sidewall_saturation_current == 0.0
        && diode.sidewall_cj0 == 0.0
        && !diode.tunneling.bottom_given
        && !diode.tunneling.sidewall_given
        && diode.tunneling.bottom == 0.0
        && diode.tunneling.sidewall == 0.0
        && diode.overlap_capacitance == 0.0
}

/// Whether a LEVEL=1 diode's junction parameters are representable by the
/// exact periodic residual's closed-form charge and current expressions.
fn diode_has_representable_exact_periodic_parameters(diode: &crate::device::Diode) -> bool {
    diode.is.is_finite()
        && diode.is >= 0.0
        && diode.n.is_finite()
        && diode.n > 0.0
        && diode.vt.is_finite()
        && diode.vt > 0.0
        && (diode.n * diode.vt).is_finite()
        && diode.cj0.is_finite()
        && diode.cj0 >= 0.0
        && diode.vj.is_finite()
        && diode.vj > 0.0
        && diode.m.is_finite()
        && (0.0..=1.0).contains(&diode.m)
        && diode.fc.is_finite()
        && (0.0..1.0).contains(&diode.fc)
        && diode.tt.is_finite()
        && diode.tt >= 0.0
        && diode.exact_hb_breakdown_parameter_error().is_none()
}

/// Whether a classic MOS device's authored equations reduce exactly to the
/// LEVEL=1 form the exact periodic residual implements.
fn mos_has_exact_periodic_residual_form(mos: &crate::device::Mosfet) -> bool {
    mos.level == 1
        && mos.body_junction_model == crate::device::MosBodyJunctionModel::NgspiceReverseClamp
        && !(mos.cjsw != 0.0 && (mos.source_perimeter != 0.0 || mos.drain_perimeter != 0.0))
}

/// Whether a JFET reduces exactly to the unscaled Shichman-Hodges form.
fn jfet_has_exact_periodic_residual_form(jfet: &crate::device::Jfet) -> bool {
    jfet.params.channel_model == crate::device::JfetChannelModel::ShichmanHodges
        && jfet.m == 1.0
        && jfet.area == 1.0
        && jfet.params.fc == 0.5
        && jfet.params.n == 1.0
        && jfet.resolved_instance_temperature() == jfet.params.tnom
}

/// Whether a JFET's physical parameters are representable by the exact
/// periodic residual.
fn jfet_has_representable_exact_periodic_parameters(jfet: &crate::device::Jfet) -> bool {
    let params = &jfet.params;
    params.vto.is_finite()
        && params.beta.is_finite()
        && params.beta >= 0.0
        && params.lambda.is_finite()
        && params.lambda >= 0.0
        && params.is.is_finite()
        && params.is >= 0.0
        && params.cgs.is_finite()
        && params.cgs >= 0.0
        && params.cgd.is_finite()
        && params.cgd >= 0.0
        && params.pb.is_finite()
        && params.pb > 0.0
        && params.m.is_finite()
        && (0.0..=1.0).contains(&params.m)
}

/// Whether a scalar line carries exact frequency-domain data the periodic
/// descriptor can stamp, rather than a retained lossy approximation.
fn tline_has_exact_periodic_descriptor(line: &crate::device::TransmissionLine) -> bool {
    if line.has_txl_runtime() {
        return false;
    }
    if line.is_memoryless_two_port() || line.ltra_ac_total_rlc().is_some() {
        return true;
    }
    line.attenuation() == 1.0
        && line.loss_time_constant() == 0.0
        && line.dc_series_resistance() == 0.0
        && !line.has_distributed_rlgc()
}

/// The MNA branch ordinals a scalar line owns in the exact periodic system.
fn tline_periodic_branch_ordinals(
    line: &crate::device::TransmissionLine,
) -> Option<(usize, usize)> {
    line.zero_length_branch_ordinals()
        .or_else(|| line.rg_branch_ordinals())
        .or_else(|| line.ltra_branch_ordinals())
        .or_else(|| line.txl_branch_ordinals())
}

/// One family's failure to satisfy one contract, with the instances involved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CapabilityGap {
    pub(crate) family: PeriodicDeviceFamily,
    /// Prose naming the missing capability and, where the restriction is
    /// per-instance, how many instances fall outside it.
    pub(crate) detail: String,
}

impl CapabilityGap {
    fn new(family: PeriodicDeviceFamily, detail: impl Into<String>) -> Self {
        Self {
            family,
            detail: detail.into(),
        }
    }
}

fn count_noun(count: usize) -> &'static str {
    if count == 1 { "device" } else { "devices" }
}

/// Join gap details into one summary clause, preserving query order.
pub(crate) fn summarize(gaps: &[CapabilityGap]) -> Option<String> {
    if gaps.is_empty() {
        return None;
    }
    Some(
        gaps.iter()
            .map(|gap| gap.detail.as_str())
            .collect::<Vec<_>>()
            .join(", "),
    )
}

/// Sort by text and drop repeats, so a summary reads deterministically and
/// never names the same missing capability twice.
fn normalize(mut gaps: Vec<CapabilityGap>) -> Vec<CapabilityGap> {
    gaps.sort_by(|left, right| {
        left.detail
            .cmp(&right.detail)
            .then_with(|| left.family.cmp(&right.family))
    });
    gaps.dedup_by(|left, right| left.detail == right.detail);
    gaps
}

// ---------------------------------------------------------------------------
// Verilog-A extension point
// ---------------------------------------------------------------------------

/// Capability metadata supplied by a device effort this module does not own.
///
/// The Verilog-A/AMS program owns what its compiled and runtime modules can
/// contribute to a periodic analysis. Registering a provider for a family lets
/// those declarations replace the conservative defaults above without this
/// module depending on any `rspice-veriloga*` crate.
pub(crate) trait ExternalPeriodicCapabilityProvider: Send + Sync + std::fmt::Debug {
    /// What the provider declares for one contract.
    fn support(&self, capability: PeriodicCapability) -> CapabilitySupport;
}

/// Refusal to register a second provider for a family.
// The extension point has no in-crate registrant yet: the Verilog-A capability
// effort is its first caller, and the hook must exist before that effort lands.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DuplicateCapabilityProvider {
    pub(crate) family: PeriodicDeviceFamily,
}

impl std::fmt::Display for DuplicateCapabilityProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a periodic capability provider is already registered for {}",
            self.family.label()
        )
    }
}

type ProviderMap = BTreeMap<PeriodicDeviceFamily, Arc<dyn ExternalPeriodicCapabilityProvider>>;

fn providers() -> &'static RwLock<ProviderMap> {
    static PROVIDERS: std::sync::OnceLock<RwLock<ProviderMap>> = std::sync::OnceLock::new();
    PROVIDERS.get_or_init(|| RwLock::new(BTreeMap::new()))
}

/// Register capability metadata for one family.
///
/// Fails closed on a second registration for the same family: silently
/// replacing one would make the effective declaration depend on link order.
// The extension point has no in-crate registrant yet: the Verilog-A capability
// effort is its first caller, and the hook must exist before that effort lands.
#[allow(dead_code)]
pub(crate) fn register_external_capability_provider(
    family: PeriodicDeviceFamily,
    provider: Arc<dyn ExternalPeriodicCapabilityProvider>,
) -> Result<(), DuplicateCapabilityProvider> {
    let mut registry = providers()
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if registry.contains_key(&family) {
        return Err(DuplicateCapabilityProvider { family });
    }
    registry.insert(family, provider);
    Ok(())
}

/// Effective declaration for a family and contract: the registered provider's
/// if there is one, otherwise the native table's.
pub(crate) fn capability_support(
    family: PeriodicDeviceFamily,
    capability: PeriodicCapability,
) -> CapabilitySupport {
    let registered = providers()
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&family)
        .map(|provider| provider.support(capability));
    registered.unwrap_or_else(|| periodic_capability_descriptor(family).support(capability))
}

/// The text a family's `Absent` declaration reports, after the registry.
fn absent_detail(
    family: PeriodicDeviceFamily,
    capability: PeriodicCapability,
) -> Option<&'static str> {
    match capability_support(family, capability) {
        Absent(missing) => Some(missing),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

/// Whether the exact periodic residual has any nonlinear device to stamp.
///
/// The harmonic solver only builds its nonlinear Newton state when a family
/// with an implemented exact residual is present.
pub(in crate::engine) fn has_exact_periodic_nonlinear_devices(circuit: &CircuitData) -> bool {
    PeriodicDeviceFamily::ALL.iter().any(|&family| {
        family.instance_count(circuit) > 0
            && matches!(
                capability_support(family, PeriodicCapability::PeriodicResidualJacobian),
                Complete | Restricted(_)
            )
    })
}

/// Gaps in the exact periodic residual/Jacobian contract.
///
/// This is the nonlinear-device half of the HB/PAC/PNoise admission gate.
pub(in crate::engine) fn periodic_residual_gaps(circuit: &CircuitData) -> Vec<CapabilityGap> {
    use PeriodicCapability::PeriodicResidualJacobian as Cap;
    use PeriodicDeviceFamily as F;
    let mut gaps = Vec::new();

    let describe = |family: PeriodicDeviceFamily, count: usize, what: &str| {
        CapabilityGap::new(family, format!("{what} ({count} {})", count_noun(count)))
    };

    // Diodes: reduced equations first, then nonrepresentable parameters, so a
    // deck hitting both learns about the model reduction it authored.
    if F::Diode.instance_count(circuit) > 0
        && capability_support(F::Diode, Cap).admits_some_instance()
    {
        let reduced = circuit
            .diodes
            .devices
            .iter()
            .filter(|diode| !diode_has_exact_periodic_residual_form(diode))
            .count();
        if reduced > 0 {
            gaps.push(describe(
                F::Diode,
                reduced,
                "diodes requiring high-injection, recombination, sidewall, tunneling, overlap, or non-LEVEL=1 equations not represented by exact HB",
            ));
        }
        let invalid = circuit
            .diodes
            .devices
            .iter()
            .filter(|diode| !diode_has_representable_exact_periodic_parameters(diode))
            .count();
        if invalid > 0 {
            gaps.push(describe(
                F::Diode,
                invalid,
                "LEVEL=1 diodes with invalid or nonrepresentable exact-HB junction parameters",
            ));
        }
    }

    if F::Mosfet.instance_count(circuit) > 0
        && capability_support(F::Mosfet, Cap).admits_some_instance()
    {
        let invalid = circuit
            .mosfets
            .devices
            .iter()
            .filter(|mos| mos.level == 1 && mos.level1_physical_parameter_error().is_some())
            .count();
        if invalid > 0 {
            gaps.push(describe(
                F::Mosfet,
                invalid,
                "LEVEL=1 MOS devices with invalid or nonrepresentable physical parameters",
            ));
        }
        let reduced = circuit
            .mosfets
            .devices
            .iter()
            .filter(|mos| !mos_has_exact_periodic_residual_form(mos))
            .count();
        if reduced > 0 {
            gaps.push(describe(
                F::Mosfet,
                reduced,
                "classic MOS devices requiring non-LEVEL=1, non-ngspice bulk-junction, or sidewall-charge equations not represented by exact HB",
            ));
        }
    }

    if F::Jfet.instance_count(circuit) > 0
        && capability_support(F::Jfet, Cap).admits_some_instance()
    {
        let reduced = circuit
            .jfets
            .iter()
            .filter(|jfet| !jfet_has_exact_periodic_residual_form(jfet))
            .count();
        if reduced > 0 {
            gaps.push(describe(
                F::Jfet,
                reduced,
                "JFET devices requiring non-Shichman-Hodges, geometry-scaled, temperature-scaled, or non-default junction equations not represented by exact HB",
            ));
        }
        let invalid = circuit
            .jfets
            .iter()
            .filter(|jfet| !jfet_has_representable_exact_periodic_parameters(jfet))
            .count();
        if invalid > 0 {
            gaps.push(describe(
                F::Jfet,
                invalid,
                "JFET devices with invalid physical parameters",
            ));
        }
    }

    if F::VoltageSwitch.instance_count(circuit) > 0
        && capability_support(F::VoltageSwitch, Cap).admits_some_instance()
    {
        let invalid = circuit
            .vswitches
            .iter()
            .filter(|switch| switch.physical_parameter_error().is_some())
            .count();
        if invalid > 0 {
            gaps.push(describe(
                F::VoltageSwitch,
                invalid,
                "voltage-controlled switches with invalid or nonrepresentable physical parameters",
            ));
        }
        let unsupported = circuit
            .vswitches
            .iter()
            .filter(|switch| switch.vh != 0.0 || switch.uses_xyce_curve_semantics())
            .count();
        if unsupported > 0 {
            gaps.push(describe(
                F::VoltageSwitch,
                unsupported,
                "voltage-controlled switches requiring hysteresis or Xyce ON/OFF curve semantics not represented by exact HB",
            ));
        }
    }

    // Families whose exact periodic residual is absent outright, reported in
    // declaration order so the summary is stable.
    for family in PeriodicDeviceFamily::ALL {
        let count = family.instance_count(circuit);
        if count == 0 {
            continue;
        }
        let Some(missing) = absent_detail(family, Cap) else {
            continue;
        };
        gaps.push(describe(family, count, missing));
    }

    gaps
}

/// Gaps in the exact periodic MNA / PAC descriptor contract.
pub(in crate::engine) fn periodic_descriptor_gaps(circuit: &CircuitData) -> Vec<CapabilityGap> {
    use PeriodicCapability::PeriodicSmallSignalDescriptor as Cap;
    use PeriodicDeviceFamily as F;
    let mut gaps = Vec::new();

    for family in PeriodicDeviceFamily::ALL {
        if family.instance_count(circuit) == 0 {
            continue;
        }
        match capability_support(family, Cap) {
            Inapplicable | Complete => {}
            Absent(missing) => gaps.push(CapabilityGap::new(family, missing)),
            Restricted(_) => match family {
                F::Capacitor => {
                    if circuit.capacitors.has_solution_dependent_values() {
                        gaps.push(CapabilityGap::new(
                            family,
                            "solution-dependent capacitor charge linearizations",
                        ));
                    }
                }
                F::TransmissionLine => {
                    if circuit.tlines.iter().any(|line| line.has_txl_runtime()) {
                        gaps.push(CapabilityGap::new(
                            family,
                            "native TXL lines without retained exact physical frequency-domain RLGC data",
                        ));
                    }
                    if circuit
                        .tlines
                        .iter()
                        .any(|line| !tline_has_exact_periodic_descriptor(line))
                    {
                        gaps.push(CapabilityGap::new(
                            family,
                            "scalar delay lines with retained lossy approximation state",
                        ));
                    }
                }
                F::CoupledTransmissionLine => {
                    if circuit
                        .coupled_tlines
                        .iter()
                        .any(|line| line.lossless_frequency_data().is_none())
                    {
                        gaps.push(CapabilityGap::new(
                            family,
                            "lossy CPL lines without retained full physical RLGC matrices",
                        ));
                    }
                }
                _ => {}
            },
        }
    }

    if !every_branch_has_a_periodic_owner(circuit) {
        gaps.push(CapabilityGap::new(
            F::ResistorBranch,
            "unrepresented MNA branch families",
        ));
    }

    normalize(gaps)
}

/// Whether every MNA branch ordinal is owned by a family with an exact
/// periodic descriptor.
///
/// This is the structural backstop: it fails closed for a branch-owning family
/// that reaches the circuit store without a declaration here.
fn every_branch_has_a_periodic_owner(circuit: &CircuitData) -> bool {
    let mut represented = vec![false; circuit.num_branches()];
    let mark = |ordinal: usize, represented: &mut Vec<bool>| {
        if let Some(slot) = ordinal
            .checked_sub(1)
            .and_then(|index| represented.get_mut(index))
        {
            *slot = true;
        }
    };

    for &ordinal in circuit
        .voltage_sources
        .branch_indices
        .iter()
        .chain(&circuit.inductors.branch_indices)
        .chain(&circuit.resistor_branches.branch_indices)
        .chain(&circuit.vcvs.branch_indices)
        .chain(&circuit.ccvs.branch_indices)
        .chain(
            circuit
                .multi_winding_transformers
                .iter()
                .flat_map(|binding| binding.branch_ordinals.iter()),
        )
    {
        mark(ordinal, &mut represented);
    }
    for line in &circuit.tlines {
        if let Some((branch1, branch2)) = tline_periodic_branch_ordinals(line) {
            mark(branch1, &mut represented);
            mark(branch2, &mut represented);
        }
    }
    for line in &circuit.coupled_tlines {
        if let Some(branches) = line.native_branch_ordinals() {
            for conductor in 0..line.conductors() {
                if let Some((near, far)) = branches.conductor(conductor) {
                    mark(near, &mut represented);
                    mark(far, &mut represented);
                }
            }
        }
    }

    represented.iter().all(|slot| *slot)
}

/// Gaps in the PSS period-map state contract.
pub(in crate::engine) fn pss_state_gaps(circuit: &CircuitData) -> Vec<CapabilityGap> {
    use PeriodicCapability::PssStateMap as Cap;
    use PeriodicDeviceFamily as F;
    let mut gaps = Vec::new();

    for family in PeriodicDeviceFamily::ALL {
        if family.instance_count(circuit) == 0 {
            continue;
        }
        match capability_support(family, Cap) {
            Inapplicable | Complete => {}
            Absent(missing) => gaps.push(CapabilityGap::new(family, missing)),
            Restricted(_) => match family {
                F::Resistor => {
                    if circuit.resistors.thermal.iter().any(Option::is_some) {
                        gaps.push(CapabilityGap::new(
                            family,
                            "thermal resistor accepted temperature state",
                        ));
                    }
                }
                F::Capacitor => {
                    if circuit.capacitors.has_solution_dependent_values() {
                        gaps.push(CapabilityGap::new(
                            family,
                            "solution-dependent capacitor charge/expression history",
                        ));
                    }
                }
                F::BehavioralSource => {
                    let has_integral = circuit
                        .behavioral_sources
                        .voltage_sources
                        .iter()
                        .any(|source| source.program.sdt_count != 0)
                        || circuit
                            .behavioral_sources
                            .current_sources
                            .iter()
                            .any(|source| source.program.sdt_count != 0);
                    if has_integral {
                        gaps.push(CapabilityGap::new(
                            family,
                            "behavioral-source accepted-step memory",
                        ));
                    }
                }
                F::TransmissionLine => {
                    if circuit
                        .tlines
                        .iter()
                        .any(|line| !line.is_memoryless_two_port())
                    {
                        gaps.push(CapabilityGap::new(
                            family,
                            "transmission-line delay history",
                        ));
                    }
                }
                _ => {}
            },
        }
    }

    normalize(gaps)
}

/// Gaps in the envelope initialization/continuation contract.
///
/// Every family outside the initializer's linear subset is named, so a
/// rejection says which element of the deck is outside it.
pub(in crate::engine) fn envelope_gaps(circuit: &CircuitData) -> Vec<CapabilityGap> {
    use PeriodicCapability::EnvelopeContinuation as Cap;
    use PeriodicDeviceFamily as F;
    let mut gaps = Vec::new();

    for family in PeriodicDeviceFamily::ALL {
        if family.instance_count(circuit) == 0 {
            continue;
        }
        match capability_support(family, Cap) {
            Inapplicable | Complete => {}
            Absent(_) => gaps.push(CapabilityGap::new(family, family.label())),
            Restricted(_) => {
                if family == F::Capacitor {
                    if circuit.capacitors.has_solution_dependent_values() {
                        gaps.push(CapabilityGap::new(
                            family,
                            "solution-dependent capacitor values",
                        ));
                    }
                    if circuit.capacitors.internal.iter().any(|internal| *internal)
                        || circuit
                            .capacitors
                            .ic_branch_indices
                            .iter()
                            .any(Option::is_some)
                    {
                        gaps.push(CapabilityGap::new(
                            family,
                            "internal or IC-constrained capacitor branches",
                        ));
                    }
                }
            }
        }
    }

    // Structural backstop: the exact initializer owns only voltage-source
    // branches, so any other branch means a family it cannot initialize.
    if circuit.num_branches() != circuit.voltage_sources.len() {
        gaps.push(CapabilityGap::new(
            F::ResistorBranch,
            "unrecognized MNA branch families",
        ));
    }

    normalize(gaps)
}

/// Gaps in the charge/dynamic-state descriptor contract.
///
/// Pole-zero extraction is the analysis that needs every dynamic state as an
/// explicit finite state in a rational `G + sC` descriptor, so it is this
/// contract's consumer.
pub(in crate::engine) fn dynamic_state_descriptor_gaps(
    circuit: &CircuitData,
) -> Vec<CapabilityGap> {
    use PeriodicCapability::DynamicStateDescriptor as Cap;
    use PeriodicDeviceFamily as F;
    let mut gaps = Vec::new();

    for family in PeriodicDeviceFamily::ALL {
        if family.instance_count(circuit) == 0 {
            continue;
        }
        match capability_support(family, Cap) {
            Inapplicable | Complete => {}
            Absent(missing) => gaps.push(CapabilityGap::new(
                family,
                format!("{}: {missing}", family.label()),
            )),
            Restricted(condition) => match family {
                F::Bsim3v3 => {
                    for dev in &circuit.bsim3v3.devices {
                        if dev.core.model.acnqs_mod != 0 {
                            gaps.push(CapabilityGap::new(
                                family,
                                format!("BSIM3 '{}' with ACNQSMOD=1: {condition}", dev.name),
                            ));
                        }
                    }
                }
                F::Bsim4v8 => {
                    for dev in &circuit.bsim4v8.devices {
                        if dev.core.model.acnqs_mod != 0 {
                            gaps.push(CapabilityGap::new(
                                family,
                                format!("BSIM4 '{}' with ACNQSMOD=1: {condition}", dev.name),
                            ));
                        }
                    }
                }
                F::TransmissionLine => {
                    for line in &circuit.tlines {
                        if !line.is_memoryless_two_port() {
                            gaps.push(CapabilityGap::new(
                                family,
                                format!("transmission line '{}': {condition}", line.name),
                            ));
                        }
                    }
                }
                _ => {}
            },
        }
    }

    gaps
}

/// Instances whose colored-noise control needs cyclostationary folding.
///
/// The low-level periodic solver has an exact stationary colored-source
/// contract; these controls modulate with the periodic device current and need
/// cyclostationary correlation rather than a DC-bias substitution.
pub(in crate::engine) fn cyclostationary_noise_gaps(circuit: &CircuitData) -> Vec<CapabilityGap> {
    use PeriodicCapability::NoiseSources as Cap;
    use PeriodicDeviceFamily as F;
    let mut gaps = Vec::new();

    if !capability_support(F::Resistor, Cap).admits_every_instance() {
        for (index, flicker) in circuit.resistors.flicker.iter().enumerate() {
            if !circuit.resistors.noisy[index] {
                continue;
            }
            if let Some((coefficient, af, ef)) = flicker
                && *coefficient != 0.0
            {
                let name = circuit
                    .resistors
                    .names
                    .get(index)
                    .map(String::as_str)
                    .unwrap_or("<unnamed resistor>");
                gaps.push(CapabilityGap::new(
                    F::Resistor,
                    format!(
                        "resistor '{name}' flicker noise (coefficient={coefficient}, AF={af}, EF={ef})"
                    ),
                ));
            }
        }
    }
    if !capability_support(F::ResistorBranch, Cap).admits_every_instance() {
        for (index, flicker) in circuit.resistor_branches.flicker.iter().enumerate() {
            if !circuit.resistor_branches.noisy[index] {
                continue;
            }
            if let Some((coefficient, af, ef)) = flicker
                && *coefficient != 0.0
            {
                let name = circuit
                    .resistor_branches
                    .names
                    .get(index)
                    .map(String::as_str)
                    .unwrap_or("<unnamed branch-form resistor>");
                gaps.push(CapabilityGap::new(
                    F::ResistorBranch,
                    format!(
                        "branch-form resistor '{name}' cyclostationary flicker noise (coefficient={coefficient}, AF={af}, EF={ef})"
                    ),
                ));
            }
        }
    }
    if !capability_support(F::Diode, Cap).admits_every_instance() {
        for diode in &circuit.diodes.devices {
            if diode.kf != 0.0 {
                gaps.push(CapabilityGap::new(
                    F::Diode,
                    format!(
                        "diode '{}' flicker noise (KF={}, AF={})",
                        diode.name, diode.kf, diode.af
                    ),
                ));
            }
        }
    }
    if !capability_support(F::Mosfet, Cap).admits_every_instance() {
        for mos in &circuit.mosfets.devices {
            if mos.kf != 0.0 {
                gaps.push(CapabilityGap::new(
                    F::Mosfet,
                    format!(
                        "MOSFET '{}' flicker noise (KF={}, AF={}, EF={})",
                        mos.name, mos.kf, mos.af, mos.ef
                    ),
                ));
            }
        }
    }
    if !capability_support(F::Jfet, Cap).admits_every_instance() {
        for jfet in &circuit.jfets {
            if jfet.params.kf != 0.0 {
                gaps.push(CapabilityGap::new(
                    F::Jfet,
                    format!(
                        "JFET '{}' flicker noise (KF={}, AF={}, EF={})",
                        jfet.name, jfet.params.kf, jfet.params.af, jfet.params.ef
                    ),
                ));
            }
        }
    }

    gaps
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Which kind of declaration a family made, ignoring its prose.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Declared {
        Inapplicable,
        Complete,
        Restricted,
        Absent,
    }

    const fn declared(support: CapabilitySupport) -> Declared {
        match support {
            Inapplicable => Declared::Inapplicable,
            Complete => Declared::Complete,
            Restricted(_) => Declared::Restricted,
            Absent(_) => Declared::Absent,
        }
    }

    /// The support set the four hand-maintained family lists encoded before
    /// they were deleted, in the order of [`PeriodicCapability::ALL`].
    ///
    /// This is the frozen "before" picture: `hb_unsupported_nonlinear_device_summary`
    /// plus `hb_has_supported_nonlinear_devices` (residual/Jacobian),
    /// `hb_periodic_mna_unsupported_summary` (periodic descriptor),
    /// `ensure_pss_continuation_state_supported` (PSS state map),
    /// `ensure_hb_envelope_linear_subset` (envelope), the pole-zero
    /// transmission-line and BSIM AC-NQS checks (dynamic state), and the
    /// pnoise colored-noise gate (noise sources). The match is exhaustive, so
    /// a new family cannot reach the circuit store without stating what it can
    /// do here as well as in the table itself.
    const fn expected_declarations(family: PeriodicDeviceFamily) -> [Declared; 6] {
        use Declared::{Absent as A, Complete as C, Inapplicable as I, Restricted as R};
        use PeriodicDeviceFamily as F;
        match family {
            F::Resistor => [I, I, C, R, R, C],
            F::ResistorBranch => [I, I, C, R, C, A],
            F::Capacitor => [I, C, R, I, R, R],
            F::Inductor => [I, C, C, I, C, A],
            F::VoltageSource | F::CurrentSource => [I, I, C, I, C, C],
            F::Vcvs | F::Vccs | F::Cccs | F::Ccvs => [I, I, C, I, C, A],
            F::Diode => [R, C, C, R, A, A],
            F::Bjt => [A, C, I, A, A, A],
            F::Mosfet => [R, C, C, R, A, A],
            F::Bsim3v3 | F::Bsim4v8 => [A, R, I, A, A, A],
            F::B3SoiDd | F::B3SoiFd | F::B3SoiPd => [A, C, I, A, A, A],
            F::Ekv26 | F::Ekv3 | F::Vdmos => [I, C, A, A, A, A],
            F::Jfet => [R, C, C, R, A, A],
            F::XyceMemristor => [A, A, I, A, A, A],
            F::VoltageSwitch => [R, I, C, I, A, A],
            F::CurrentSwitch | F::GenericSwitch => [A, I, I, I, A, A],
            // The dynamic-state and PSS contracts became instance conditional
            // when the memoryless RG line gained native execution stamps: an
            // RG line carries neither an irrational descriptor nor delay
            // history, so it is admitted where a linear resistor is.
            F::TransmissionLine => [I, R, R, I, R, A],
            F::CoupledTransmissionLine => [I, C, R, I, A, A],
            F::InductorCoupling | F::CoupledInductorPair | F::MultiWindingTransformer => {
                [I, C, C, I, A, A]
            }
            F::JilesAthertonInductor | F::XyceCoreGroup => [I, C, A, I, A, A],
            F::BehavioralSource => [I, C, A, I, R, A],
            F::XspiceInstance => [I, C, A, I, A, A],
            F::RuntimeVerilogA => [A, C, I, A, A, A],
            F::GeneratedVerilogA => [I, C, A, A, A, A],
        }
    }

    #[test]
    fn every_family_declares_the_support_the_deleted_hand_lists_encoded() {
        for family in PeriodicDeviceFamily::ALL {
            let expected = expected_declarations(family);
            let descriptor = periodic_capability_descriptor(family);
            for (index, capability) in PeriodicCapability::ALL.into_iter().enumerate() {
                assert_eq!(
                    declared(descriptor.support(capability)),
                    expected[index],
                    "{family:?} changed its declared support for {capability:?}; \
                     a device may not gain or lose advanced-analysis support silently"
                );
            }
        }
    }

    #[test]
    fn every_family_declaration_is_explicit_and_gives_a_reason() {
        let mut labels = std::collections::BTreeSet::new();
        for family in PeriodicDeviceFamily::ALL {
            assert!(
                labels.insert(family.label()),
                "{family:?} shares its label with another family, so a rejection \
                 could not say which one it means"
            );
            let descriptor = periodic_capability_descriptor(family);
            for capability in PeriodicCapability::ALL {
                match descriptor.support(capability) {
                    Inapplicable | Complete => {}
                    Restricted(condition) => assert!(
                        !condition.trim().is_empty(),
                        "{family:?} restricts {capability:?} without saying to what"
                    ),
                    Absent(missing) => assert!(
                        !missing.trim().is_empty(),
                        "{family:?} refuses {capability:?} without naming what is missing"
                    ),
                }
            }
        }
        assert_eq!(
            labels.len(),
            PeriodicDeviceFamily::ALL.len(),
            "the family list and its labels must stay one-to-one"
        );
    }

    /// An empty circuit contains no family, so no contract can report a gap.
    /// This is the floor the whole table rests on: a gap always names an
    /// instance that is actually present.
    #[test]
    fn an_empty_circuit_has_no_capability_gap() {
        let circuit = CircuitData::new();
        assert!(periodic_residual_gaps(&circuit).is_empty());
        assert!(periodic_descriptor_gaps(&circuit).is_empty());
        assert!(pss_state_gaps(&circuit).is_empty());
        assert!(envelope_gaps(&circuit).is_empty());
        assert!(dynamic_state_descriptor_gaps(&circuit).is_empty());
        assert!(cyclostationary_noise_gaps(&circuit).is_empty());
        assert!(!has_exact_periodic_nonlinear_devices(&circuit));
    }

    #[derive(Debug)]
    struct StubProvider(CapabilitySupport);

    impl ExternalPeriodicCapabilityProvider for StubProvider {
        fn support(&self, _capability: PeriodicCapability) -> CapabilitySupport {
            self.0
        }
    }

    /// The extension point the Verilog-A capability effort will use.
    ///
    /// It is a registry of trait objects keyed by family: this module names no
    /// `rspice-veriloga*` type, and a registrant replaces the conservative
    /// native declaration without editing the table. A second registration for
    /// the same family is refused rather than silently overwriting, because
    /// the effective declaration must not depend on registration order.
    #[test]
    fn an_external_provider_replaces_a_declaration_and_refuses_a_duplicate() {
        // The generated Verilog-A family is chosen because no other test in
        // this binary builds a circuit containing one.
        let family = PeriodicDeviceFamily::GeneratedVerilogA;
        let capability = PeriodicCapability::PeriodicSmallSignalDescriptor;
        assert!(
            matches!(
                periodic_capability_descriptor(family).support(capability),
                Absent(_)
            ),
            "the native table must refuse this contract before a provider registers"
        );

        register_external_capability_provider(family, Arc::new(StubProvider(Complete)))
            .expect("the first registration for a family is accepted");
        assert_eq!(capability_support(family, capability), Complete);

        let duplicate =
            register_external_capability_provider(family, Arc::new(StubProvider(Inapplicable)))
                .expect_err("a second provider for the same family must be refused");
        assert_eq!(duplicate.family, family);
        assert!(
            duplicate.to_string().contains(family.label()),
            "the refusal must name the family it protects: {duplicate}"
        );
        assert_eq!(
            capability_support(family, capability),
            Complete,
            "a refused registration must not disturb the accepted one"
        );
    }
}
