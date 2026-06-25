//! BJT (Bipolar Junction Transistor) device model
//!
//! Implements the Ebers-Moll model for NPN and PNP transistors.
//! Supports both large-signal DC and small-signal AC analysis.

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};
use std::cell::{Cell, RefCell};

mod dynamic;
mod intrinsic;
mod mna;
mod params;
mod solve;

/// BJT transistor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BjtType {
    Npn,
    Pnp,
}

/// Ngspice legacy BJT substrate-charge topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BjtSubstrateTopology {
    Vertical,
    Lateral,
}

impl BjtSubstrateTopology {
    #[inline]
    pub(crate) fn default_for_type(bjt_type: BjtType) -> Self {
        match bjt_type {
            BjtType::Npn => Self::Vertical,
            BjtType::Pnp => Self::Lateral,
        }
    }

    #[inline]
    pub(crate) fn from_ngspice_subs(value: Value, bjt_type: BjtType) -> Self {
        if (value.round() as i32) == -1 {
            Self::Lateral
        } else if (value.round() as i32) == 1 {
            Self::Vertical
        } else {
            Self::default_for_type(bjt_type)
        }
    }

    #[inline]
    pub(crate) fn ngspice_sign(self) -> Value {
        match self {
            Self::Vertical => 1.0,
            Self::Lateral => -1.0,
        }
    }
}

/// Pre-computed stamp indices for O(1) matrix access (3-terminal device)
/// Layout: [row][col] where row/col are C, B, E
#[derive(Debug, Clone, Default)]
pub struct BjtIndices {
    // Collector row
    pub cc: Option<CscIndex>,
    pub cb: Option<CscIndex>,
    pub ce: Option<CscIndex>,
    // Base row
    pub bc: Option<CscIndex>,
    pub bb: Option<CscIndex>,
    pub be: Option<CscIndex>,
    // Emitter row
    pub ec: Option<CscIndex>,
    pub eb: Option<CscIndex>,
    pub ee: Option<CscIndex>,
}

#[derive(Debug, Clone, Copy, Default)]
struct BjtLinearization {
    ic: Value,
    ib: Value,
    dic_dvbe: Value,
    dic_dvbc: Value,
    dic_dvrth: Value,
    dib_dvbe: Value,
    dib_dvbc: Value,
    dib_dvrth: Value,
    qb: Value,
    dqb_dvbe: Value,
    dqb_dvbc: Value,
    dqb_dvrth: Value,
}

#[derive(Debug, Clone, Copy, Default)]
struct TransportChargeState {
    q1: Value,
    qb: Value,
    ifi: Value,
    iri: Value,
    gfi: Value,
    gri: Value,
    dq1_dvbe_eff: Value,
    dq1_dvbc_eff: Value,
    itzf: Value,
    itzr: Value,
    dqb_dvbe_eff: Value,
    dqb_dvbc_eff: Value,
    ditzf_dvbe_eff: Value,
    ditzf_dvbc_eff: Value,
    ditzr_dvbe_eff: Value,
    ditzr_dvbc_eff: Value,
}

#[derive(Debug, Clone, Copy)]
struct BaseCollectorCurrentState {
    ibc: Value,
    dibc_dvbe_eff: Value,
    dibc_dvbc_eff: Value,
}

#[derive(Debug, Clone, Copy)]
struct BjtIntrinsicBranches {
    ibe: BranchLinearization,
    ibex: BranchLinearization,
    ibc: BranchLinearization,
    iciei: BranchLinearization,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct LegacyTransientChargeState {
    pub qbe: Value,
    pub capbe: Value,
    pub capbe_vbc: Value,
    pub qbc: Value,
    pub capbc: Value,
    pub qbx: Value,
    pub capbx: Value,
    pub qcs: Value,
    pub capcs: Value,
}

#[derive(Debug, Clone, Copy)]
struct IntrinsicTerminalState {
    vcx: Value,
    vci: Value,
    vbx: Value,
    vbi: Value,
    vei: Value,
    vbp: Value,
    vsi: Value,
    vrth: Value,
}

#[derive(Debug, Clone, Copy, Default)]
struct VbicNonlinearBranchVoltages {
    vbei: Value,
    vbex: Value,
    vbci: Value,
    vbcx: Value,
    vbep: Value,
    vbcp: Value,
    vrth: Value,
}

#[derive(Debug, Clone, Copy, Default)]
struct LegacyNonlinearBranchVoltages {
    vbe: Value,
    vbc: Value,
    vsub: Value,
}

#[derive(Debug, Clone, Copy, Default)]
struct BranchLinearization {
    current: Value,
    d_internal: [Value; INTERNAL_DIM],
    d_external: [Value; EXTERNAL_DIM],
}

pub(crate) const BJT_DYNAMIC_CHARGE_COUNT: usize = 11;
pub(crate) const BJT_INTERNAL_STATE_DIM: usize = DYNAMIC_INTERNAL_DIM;
pub(crate) const BJT_EXTERNAL_STATE_DIM: usize = EXTERNAL_DIM;
pub(crate) const VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT: usize = 10;
pub(crate) const VBIC_TRANSIENT_CONVERGENCE_VOLTAGE_COUNT: usize = 9;
pub(crate) const VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX: usize = 2;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct VbicTransientConvergenceState {
    pub voltages: [Value; VBIC_TRANSIENT_CONVERGENCE_VOLTAGE_COUNT],
    pub currents: [Value; VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT],
    pub d_currents_d_internal:
        [[Value; BJT_INTERNAL_STATE_DIM]; VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT],
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct BjtChargeBranch {
    pub charge: Value,
    pub d_internal: [Value; BJT_INTERNAL_STATE_DIM],
    pub d_external: [Value; EXTERNAL_DIM],
    pub pos_internal: Option<usize>,
    pub neg_internal: Option<usize>,
    pub pos_external: Option<usize>,
    pub neg_external: Option<usize>,
}

impl BjtChargeBranch {
    pub(crate) fn is_active(&self) -> bool {
        self.pos_internal.is_some()
            || self.neg_internal.is_some()
            || self.pos_external.is_some()
            || self.neg_external.is_some()
    }

    pub(crate) fn linearization_dot(
        &self,
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
        external_voltages: &[Value; EXTERNAL_DIM],
    ) -> Value {
        let internal = self
            .d_internal
            .iter()
            .zip(internal_voltages.iter())
            .map(|(d, v)| d * v)
            .sum::<Value>();
        let external = self
            .d_external
            .iter()
            .zip(external_voltages.iter())
            .map(|(d, v)| d * v)
            .sum::<Value>();
        internal + external
    }

    pub(crate) fn accumulate_derivatives(
        &self,
        c_ii: &mut [[Value; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
        c_ie: &mut [[Value; EXTERNAL_DIM]; BJT_INTERNAL_STATE_DIM],
        c_ei: &mut [[Value; BJT_INTERNAL_STATE_DIM]; EXTERNAL_DIM],
        c_ee: &mut [[Value; EXTERNAL_DIM]; EXTERNAL_DIM],
    ) {
        for (sign, row) in [(1.0, self.pos_internal), (-1.0, self.neg_internal)] {
            let Some(row) = row else {
                continue;
            };
            for col in 0..BJT_INTERNAL_STATE_DIM {
                c_ii[row][col] += sign * self.d_internal[col];
            }
            for col in 0..EXTERNAL_DIM {
                c_ie[row][col] += sign * self.d_external[col];
            }
        }

        for (sign, row) in [(1.0, self.pos_external), (-1.0, self.neg_external)] {
            let Some(row) = row else {
                continue;
            };
            for col in 0..BJT_INTERNAL_STATE_DIM {
                c_ei[row][col] += sign * self.d_internal[col];
            }
            for col in 0..EXTERNAL_DIM {
                c_ee[row][col] += sign * self.d_external[col];
            }
        }
    }

    pub(crate) fn accumulate_source(
        &self,
        current: Value,
        z_i: &mut [Value; BJT_INTERNAL_STATE_DIM],
        z_e: &mut [Value; EXTERNAL_DIM],
    ) {
        for (sign, row) in [(1.0, self.pos_internal), (-1.0, self.neg_internal)] {
            if let Some(row) = row {
                z_i[row] += sign * current;
            }
        }
        for (sign, row) in [(1.0, self.pos_external), (-1.0, self.neg_external)] {
            if let Some(row) = row {
                z_e[row] += sign * current;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct BjtCurrentBranch {
    pub current: Value,
    pub d_internal: [Value; BJT_INTERNAL_STATE_DIM],
    pub d_external: [Value; EXTERNAL_DIM],
    pub pos_internal: Option<usize>,
    pub neg_internal: Option<usize>,
    pub pos_external: Option<usize>,
    pub neg_external: Option<usize>,
}

impl BjtCurrentBranch {
    pub(crate) fn is_active(&self) -> bool {
        self.pos_internal.is_some()
            || self.neg_internal.is_some()
            || self.pos_external.is_some()
            || self.neg_external.is_some()
    }

    pub(crate) fn linearization_dot(
        &self,
        internal_voltages: &[Value; BJT_INTERNAL_STATE_DIM],
        external_voltages: &[Value; EXTERNAL_DIM],
    ) -> Value {
        let internal = self
            .d_internal
            .iter()
            .zip(internal_voltages.iter())
            .map(|(d, v)| d * v)
            .sum::<Value>();
        let external = self
            .d_external
            .iter()
            .zip(external_voltages.iter())
            .map(|(d, v)| d * v)
            .sum::<Value>();
        internal + external
    }

    pub(crate) fn accumulate_source(
        &self,
        current: Value,
        z_i: &mut [Value; BJT_INTERNAL_STATE_DIM],
        z_e: &mut [Value; BJT_EXTERNAL_STATE_DIM],
    ) {
        for (sign, row) in [(1.0, self.pos_internal), (-1.0, self.neg_internal)] {
            if let Some(row) = row {
                z_i[row] += sign * current;
            }
        }
        for (sign, row) in [(1.0, self.pos_external), (-1.0, self.neg_external)] {
            if let Some(row) = row {
                z_e[row] += sign * current;
            }
        }
    }

    pub(crate) fn accumulate_derivatives(
        &self,
        g_ii: &mut [[Value; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
        g_ie: &mut [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
        g_ei: &mut [[Value; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
        g_ee: &mut [[Value; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM],
    ) {
        for (sign, row) in [(1.0, self.pos_internal), (-1.0, self.neg_internal)] {
            let Some(row) = row else {
                continue;
            };
            for col in 0..BJT_INTERNAL_STATE_DIM {
                g_ii[row][col] += sign * self.d_internal[col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                g_ie[row][col] += sign * self.d_external[col];
            }
        }

        for (sign, row) in [(1.0, self.pos_external), (-1.0, self.neg_external)] {
            let Some(row) = row else {
                continue;
            };
            for col in 0..BJT_INTERNAL_STATE_DIM {
                g_ei[row][col] += sign * self.d_internal[col];
            }
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                g_ee[row][col] += sign * self.d_external[col];
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct BjtDynamicReduction {
    pub internal_voltages: [Value; BJT_INTERNAL_STATE_DIM],
    pub external_voltages: [Value; EXTERNAL_DIM],
    pub g_ii: [[Value; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM],
    pub g_ie: [[Value; EXTERNAL_DIM]; BJT_INTERNAL_STATE_DIM],
    pub g_ei: [[Value; BJT_INTERNAL_STATE_DIM]; EXTERNAL_DIM],
    pub g_ee: [[Value; EXTERNAL_DIM]; EXTERNAL_DIM],
    pub g_reduced: [[Value; EXTERNAL_DIM]; EXTERNAL_DIM],
    pub z_i_static: [Value; BJT_INTERNAL_STATE_DIM],
    pub z_e_static: [Value; EXTERNAL_DIM],
    vbic_transport: TransportChargeState,
    vbic_d_itzf_d_vrth: Value,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct BjtReducedLinearization {
    pub internal_voltages: [Value; INTERNAL_DIM],
    pub external_voltages: [Value; EXTERNAL_DIM],
    pub g_ii: [[Value; INTERNAL_DIM]; INTERNAL_DIM],
    pub g_ie: [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
    pub g_ei: [[Value; INTERNAL_DIM]; EXTERNAL_DIM],
    pub g_ee: [[Value; EXTERNAL_DIM]; EXTERNAL_DIM],
    pub g_reduced: [[Value; EXTERNAL_DIM]; EXTERNAL_DIM],
    pub z_i_static: [Value; INTERNAL_DIM],
    pub z_e_static: [Value; EXTERNAL_DIM],
    cached_dynamic_inputs: Option<BjtDynamicChargeInputs>,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct BjtChargeSnapshot {
    pub reduction: BjtDynamicReduction,
    pub branches: [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT],
}

#[derive(Debug, Clone, Copy)]
struct EvaluatedBjtState {
    linearized: BjtLinearization,
    ibe: BranchLinearization,
    ibex: BranchLinearization,
    ibc: BranchLinearization,
    iciei: BranchLinearization,
    ircx: BranchLinearization,
    irci: BranchLinearization,
    irbx: BranchLinearization,
    irbi: BranchLinearization,
    ire: BranchLinearization,
    ibep: BranchLinearization,
    irbp: BranchLinearization,
    ibcp: BranchLinearization,
    iccp: BranchLinearization,
    irs: BranchLinearization,
}

type BjtRowCoefficients = [Value; EXTERNAL_DIM];
type BjtConductanceMatrix = [BjtRowCoefficients; EXTERNAL_DIM];

#[derive(Debug, Clone, Copy, Default)]
struct ParasiticTransportState {
    qbp: Value,
    d_qbp: [Value; INTERNAL_DIM],
    ifp: Value,
    d_ifp: [Value; INTERNAL_DIM],
    irp: Value,
    d_irp: [Value; INTERNAL_DIM],
}

#[derive(Debug, Clone, Copy, Default)]
struct EpiChargeState {
    kbci: Value,
    d_kbci: [Value; INTERNAL_DIM],
    kbcx: Value,
    d_kbcx: [Value; INTERNAL_DIM],
}

#[derive(Debug, Clone, Copy, Default)]
struct BjtDynamicChargeInputs {
    transport: TransportChargeState,
    parasitic: ParasiticTransportState,
    epi: EpiChargeState,
    qdbe: Value,
    dqdbe_dvbe_eff: Value,
    qdbex: Value,
    dqdbex_dvbex_eff: Value,
    qdbc: Value,
    dqdbc_dvbc_eff: Value,
    qdbep: Value,
    dqdbep_dvbep_eff: Value,
    qdbcp: Value,
    dqdbcp_dvbcp_eff: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BjtChargeModel {
    LegacyGummelPoon,
    Vbic,
}

const INTERNAL_DIM: usize = 8;
const EXTERNAL_DIM: usize = 4;
const IDX_VCX: usize = 0;
const IDX_VCI: usize = 1;
const IDX_VBX: usize = 2;
const IDX_VBI: usize = 3;
const IDX_VEI: usize = 4;
const IDX_VBP: usize = 5;
const IDX_VSI: usize = 6;
const IDX_VRTH: usize = 7;
const DYNAMIC_INTERNAL_DIM: usize = INTERNAL_DIM + 2;
const VBIC_LIMITED_BRANCH_DIM: usize = 6;
const LEGACY_LIMITED_BRANCH_DIM: usize = 3;
const IDX_VXF1: usize = INTERNAL_DIM;
const IDX_VXF2: usize = INTERNAL_DIM + 1;
const IDX_QCTH: usize = BJT_DYNAMIC_CHARGE_COUNT - 3;
const IDX_QXF1: usize = BJT_DYNAMIC_CHARGE_COUNT - 2;
const IDX_QXF2: usize = BJT_DYNAMIC_CHARGE_COUNT - 1;
const EXT_C: usize = 0;
const EXT_B: usize = 1;
const EXT_E: usize = 2;
const EXT_S: usize = 3;

/// BJT device using the Ebers-Moll model
///
/// Terminal connections:
/// - Collector (C)
/// - Base (B)
/// - Emitter (E)
#[derive(Debug, Clone)]
pub struct Bjt {
    pub name: String,
    pub bjt_type: BjtType,
    pub substrate_topology: BjtSubstrateTopology,
    charge_model: BjtChargeModel,

    // Node connections
    pub node_collector: NodeId,
    pub node_base: NodeId,
    pub node_emitter: NodeId,
    /// Optional substrate node (4-terminal BJT syntax)
    pub node_substrate: NodeId,

    // MNA-promoted VBIC internal nodes (ngspice vbicsetup.c topology). The
    // circuit builder assigns these through `assign_vbic_internal_nodes`,
    // aliasing collapsed states onto their parent nodes (`cx` onto the
    // collector when RCX is zero, `ci` onto `cx` when RCI is zero, and so on)
    // so every state keeps exactly one matrix column. `node_rth` and the
    // excess-phase pair stay zero (ground, dropped by stampers) when
    // self-heating or TD is disabled.
    pub node_cx: NodeId,
    pub node_ci: NodeId,
    pub node_bx: NodeId,
    pub node_bi: NodeId,
    pub node_ei: NodeId,
    pub node_bp: NodeId,
    pub node_si: NodeId,
    pub node_rth: NodeId,
    pub node_xf1: NodeId,
    pub node_xf2: NodeId,
    /// True once the builder has promoted this VBIC instance's internal
    /// states to MNA unknowns; legacy Gummel-Poon instances stay on the
    /// reduced 4-terminal Schur stamp.
    vbic_mna_promoted: bool,

    // Model parameters (Ebers-Moll)
    /// Saturation current (IS)
    pub is: Value,
    /// Forward current gain (BF)
    pub bf: Value,
    /// Reverse current gain (BR)
    pub br: Value,
    /// Forward emission coefficient (NF)
    pub nf: Value,
    /// Reverse emission coefficient (NR)
    pub nr: Value,
    /// Thermal voltage (VT = kT/q, ~26mV at 300K)
    pub vt: Value,
    /// Nominal model temperature (K)
    pub tnom: Value,
    /// Requested ambient/device temperature before self-heating (K)
    ambient_temperature: Value,
    /// Active device temperature (K)
    pub temperature: Value,
    /// Saturation-current temperature exponent (XTI)
    pub xti: Value,
    /// Bandgap used for IS temperature scaling (EG, eV)
    pub eg: Value,
    /// Base-emitter built-in potential
    pub vje: Value,
    /// Base-collector built-in potential
    pub vjc: Value,
    /// Forward-bias depletion-cap smoothing coefficient (FC)
    pub fc: Value,
    /// Forward Early voltage (VAF)
    pub vaf: Value,
    /// Reverse Early voltage (VAR)
    pub var: Value,
    /// Legacy aggregate base resistance
    pub rb: Value,
    /// Legacy aggregate collector resistance
    pub rc: Value,
    /// Emitter resistance
    pub re: Value,
    /// Extrinsic base resistance (RBX)
    pub rbx: Value,
    /// Intrinsic base resistance (RBI)
    pub rbi: Value,
    /// Extrinsic collector resistance (RCX)
    pub rcx: Value,
    /// Intrinsic collector resistance (RCI)
    pub rci: Value,
    /// Substrate resistance (RS)
    pub rs: Value,
    /// Parasitic base resistance (RBP)
    pub rbp: Value,
    /// Epi drift saturation voltage (VO)
    pub vo: Value,
    /// Epi doping parameter (GAMM)
    pub gamm: Value,
    /// High-current collector resistance factor (HRCF)
    pub hrcf: Value,
    /// B-C weak avalanche parameter 1 (AVC1)
    pub avc1: Value,
    /// B-C weak avalanche parameter 2 (AVC2)
    pub avc2: Value,
    /// Temperature exponent of AVC2 (TAVC)
    pub tavc: Value,
    /// Temperature exponent of IS (XIS)
    pub xis: Value,
    /// Temperature exponent of IBEI/IBCI (XII)
    pub xii: Value,
    /// Temperature exponent of IBEN/IBCN (XIN)
    pub xin: Value,
    /// Temperature exponent of ISRR (XISR)
    pub xisr: Value,
    /// Temperature exponent of emitter resistance (XRE)
    pub xre: Value,
    /// Temperature exponent of intrinsic base resistance (XRBI)
    pub xrbi: Value,
    /// Temperature exponent of intrinsic collector resistance (XRCI)
    pub xrci: Value,
    /// Temperature exponent of substrate resistance (XRS)
    pub xrs: Value,
    /// Temperature exponent of epi drift saturation voltage (XVO)
    pub xvo: Value,
    /// Temperature exponent of parasitic base resistance (XRBP)
    pub xrbp: Value,
    /// Temperature exponent of forward emission coefficients NF/NR (TNF)
    pub tnf: Value,
    /// Temperature exponent of forward knee current IKF (XIKF)
    pub xikf: Value,
    /// Temperature exponent of extrinsic collector resistance (XRCX)
    pub xrcx: Value,
    /// Temperature exponent of extrinsic base resistance (XRBX)
    pub xrbx: Value,
    /// Activation energy for IS (EA)
    pub ea: Value,
    /// Activation energy for IBEI (EAIE)
    pub eaie: Value,
    /// Activation energy for IBCI (EAIC)
    pub eaic: Value,
    /// Activation energy for IBCIP (EAIS)
    pub eais: Value,
    /// Activation energy for IBEN (EANE)
    pub eane: Value,
    /// Activation energy for IBCN (EANC)
    pub eanc: Value,
    /// Activation energy for IBCNP (EANS)
    pub eans: Value,
    /// Delta activation energy for ISRR (DEAR)
    pub dear: Value,
    /// Activation energy for ISP (EAP)
    pub eap: Value,

    // Gummel-Poon charge model parameters
    /// Zero-bias B-E junction capacitance (CJE)
    pub cje: Value,
    /// Forward-bias smoothing coefficient for intrinsic B-E depletion charge (AJE)
    pub aje: Value,
    /// B-E built-in potential (VJE)
    pub mje: Value,
    /// Zero-bias B-C junction capacitance (CJC)
    pub cjc: Value,
    /// Fraction of CJC assigned to the internal B-C junction (XCJC)
    pub xcjc: Value,
    /// Zero-bias collector-substrate capacitance (CJCP)
    pub cjcp: Value,
    /// Zero-bias extrinsic B-C capacitance (CJEP)
    pub cjep: Value,
    /// Extrinsic B-E overlap capacitance (CBEO)
    pub cbeo: Value,
    /// Extrinsic B-C overlap capacitance (CBCO)
    pub cbco: Value,
    /// Epi charge parameter (QCO)
    pub qco: Value,
    /// Fixed collector-substrate capacitance (CCSO)
    pub ccso: Value,
    /// Collector-substrate built-in potential (PS)
    pub ps: Value,
    /// Collector-substrate grading coefficient (MS)
    pub ms: Value,
    /// Collector-substrate smoothing coefficient (AJS)
    pub ajs: Value,
    /// Portion of intrinsic B-E depletion charge assigned to Vbei (WBE)
    pub wbe: Value,
    /// VBIC13 reverse B-E breakdown voltage at active temperature (VBBE_T).
    pub vbbe: Value,
    /// VBIC13 reverse B-E breakdown emission coefficient at active temperature (NBBE_T).
    pub nbbe: Value,
    /// Active VBIC13 reverse B-E breakdown current after instance scaling (IBBE).
    pub ibbe: Value,
    /// VBIC13 reverse B-E breakdown equilibrium exponential, recomputed from VBBE/NBBE/Vt.
    pub ebbe: Value,
    /// First-order temperature coefficient for VBBE (TVBBE1).
    pub tvbbe1: Value,
    /// Second-order temperature coefficient for VBBE (TVBBE2).
    pub tvbbe2: Value,
    /// Temperature coefficient for NBBE (TNBBE).
    pub tnbbe: Value,
    /// Circuit-level junction GMIN (ngspice `CKTgmin`): every nonlinear
    /// branch carries a `gmin` parallel in ngspice's bjt/vbic loads, and
    /// gmin stepping ramps this value through the device equations, not
    /// just the matrix diagonal.
    pub junction_gmin: Value,
    /// Forward-bias smoothing coefficient for B-C depletion charge (AJC)
    pub ajc: Value,
    /// B-C grading coefficient (MJC)
    pub mjc: Value,
    /// Forward transit time (TF)
    pub tf: Value,
    /// Base-width modulation factor for TF (QTF)
    pub qtf: Value,
    /// Bias dependence coefficient for TF (XTF)
    pub xtf: Value,
    /// Voltage giving VBC dependence of TF (VTF)
    pub vtf: Value,
    /// High-current dependence of TF (ITF)
    pub itf: Value,
    /// Reverse transit time (TR)
    pub tr: Value,
    /// Forward excess-phase delay time (TD)
    pub td: Value,
    /// Thermal resistance (RTH)
    pub rth: Value,
    /// Thermal capacitance (CTH)
    pub cth: Value,
    /// VBIC self-heating selector (SELFT): 0 disables internal thermal state, 1 enables it.
    pub selft: Value,
    /// Whether SELFT was explicitly provided in model parameters.
    selft_given: bool,
    /// Knee current for high-level injection (IKF)
    pub ikf: Value,
    /// Reverse knee current (IKR)
    pub ikr: Value,
    /// Selects the SGP high-current qb formulation (QBM)
    pub qbm: Value,
    /// High-current beta roll-off exponent (NKF)
    pub nkf: Value,
    /// Whether NK/NKF was explicitly provided on the model card.
    nkf_given: bool,
    /// Reverse transport scale factor (ISRR in VBIC)
    pub isrr: Value,
    /// Parasitic transport saturation current (ISP)
    pub isp: Value,
    /// Portion of parasitic transport current driven by Vbep (WSP)
    pub wsp: Value,
    /// Parasitic forward emission coefficient (NFP)
    pub nfp: Value,
    /// Parasitic knee current (IKP)
    pub ikp: Value,
    /// Ideal parasitic B-E saturation current (IBEIP)
    pub ibeip: Value,
    /// Non-ideal parasitic B-E saturation current (IBENP)
    pub ibenp: Value,
    /// Ideal parasitic B-C saturation current (IBCIP)
    pub ibcip: Value,
    /// Non-ideal parasitic B-C saturation current (IBCNP)
    pub ibcnp: Value,
    /// Ideal parasitic B-C emission coefficient (NCIP)
    pub ncip: Value,
    /// Non-ideal parasitic B-C emission coefficient (NCNP)
    pub ncnp: Value,
    /// Instance area factor
    pub area: Value,
    /// Instance multiplicity factor
    pub m: Value,
    /// Instance OFF flag used for operating-point startup seeding.
    initial_off: bool,
    /// Flicker noise coefficient (KF)
    pub kf: Value,
    /// Flicker noise current exponent (AF)
    pub af: Value,
    /// Flicker noise frequency exponent (EF)
    pub ef: Value,
    /// Thermal-noise temperature offset in kelvin (ngspice `dtemp`
    /// semantics for the parasitic-resistance thermal sources).
    pub noise_temperature_offset: Value,
    /// VBIC base-emitter flicker noise coefficient (KFN)
    pub kfn: Value,
    /// VBIC base-emitter flicker noise current exponent (AFN)
    pub afn: Value,
    /// VBIC base-emitter flicker noise frequency exponent (BFN)
    pub bfn: Value,
    /// Active ideal base-emitter saturation current.
    ibei: Value,
    /// Active non-ideal base-emitter saturation current.
    iben: Value,
    /// Active ideal base-collector saturation current.
    ibci: Value,
    /// Active non-ideal base-collector saturation current.
    ibcn: Value,
    /// Emission coefficient for ideal BE base current branch.
    nei: Value,
    /// Emission coefficient for non-ideal BE base current branch.
    nen: Value,
    /// Emission coefficient for ideal BC base current branch.
    nci: Value,
    /// Emission coefficient for non-ideal BC base current branch.
    ncn: Value,
    /// Nominal forward emission coefficient before temperature scaling.
    nf_nominal: Value,
    /// Nominal reverse emission coefficient before temperature scaling.
    nr_nominal: Value,

    /// Nominal saturation current before area/multiplicity and temp scaling
    is_nominal: Value,
    /// Nominal B-E built-in potential before temperature scaling.
    vje_nominal: Value,
    /// Nominal B-C built-in potential before temperature scaling.
    vjc_nominal: Value,
    /// Nominal emitter resistance before temperature scaling.
    re_nominal: Value,
    /// Nominal extrinsic base resistance before temperature scaling.
    rbx_nominal: Value,
    /// Nominal intrinsic base resistance before temperature scaling.
    rbi_nominal: Value,
    /// Nominal extrinsic collector resistance before temperature scaling.
    rcx_nominal: Value,
    /// Nominal intrinsic collector resistance before temperature scaling.
    rci_nominal: Value,
    /// Nominal epi drift saturation voltage before temperature scaling.
    vo_nominal: Value,
    /// Nominal epi doping parameter before temperature scaling.
    gamm_nominal: Value,
    /// Nominal zero-bias B-E capacitance before area/multiplicity scaling
    cje_nominal: Value,
    /// Nominal zero-bias B-C capacitance before area/multiplicity scaling
    cjc_nominal: Value,
    /// Nominal zero-bias collector-substrate capacitance before scaling
    cjcp_nominal: Value,
    /// Nominal zero-bias extrinsic B-C capacitance before scaling.
    cjep_nominal: Value,
    /// Nominal extrinsic B-E overlap capacitance before scaling.
    cbeo_nominal: Value,
    /// Nominal extrinsic B-C overlap capacitance before scaling.
    cbco_nominal: Value,
    /// Nominal epi charge parameter before scaling.
    qco_nominal: Value,
    /// Nominal fixed collector-substrate capacitance before scaling.
    ccso_nominal: Value,
    /// Nominal collector-substrate built-in potential before temperature scaling.
    ps_nominal: Value,
    /// Nominal forward high-injection knee current before scaling
    ikf_nominal: Value,
    /// Nominal reverse high-injection knee current before scaling
    ikr_nominal: Value,
    /// Nominal reverse transport scale factor before temperature scaling.
    isrr_nominal: Value,
    /// Nominal ideal base-emitter saturation current before scaling.
    ibei_nominal: Value,
    /// Nominal non-ideal base-emitter saturation current before scaling.
    iben_nominal: Value,
    /// Nominal ideal base-collector saturation current before scaling.
    ibci_nominal: Value,
    /// Nominal non-ideal base-collector saturation current before scaling.
    ibcn_nominal: Value,
    /// Nominal VBIC13 reverse B-E breakdown voltage before temperature scaling.
    vbbe_nominal: Value,
    /// Nominal VBIC13 reverse B-E breakdown emission coefficient before temperature scaling.
    nbbe_nominal: Value,
    /// Nominal VBIC13 reverse B-E breakdown current before instance scaling.
    ibbe_nominal: Value,
    /// Nominal parasitic transport saturation current before scaling.
    isp_nominal: Value,
    /// Nominal ideal parasitic B-E saturation current before scaling.
    ibeip_nominal: Value,
    /// Nominal non-ideal parasitic B-E saturation current before scaling.
    ibenp_nominal: Value,
    /// Nominal ideal parasitic B-C saturation current before scaling.
    ibcip_nominal: Value,
    /// Nominal non-ideal parasitic B-C saturation current before scaling.
    ibcnp_nominal: Value,
    /// Nominal substrate resistance before temperature scaling.
    rs_nominal: Value,
    /// Nominal parasitic base resistance before temperature scaling.
    rbp_nominal: Value,
    /// Nominal weak avalanche parameter 2 before temperature scaling.
    avc2_nominal: Value,
    /// Nominal thermal resistance before multiplicity scaling.
    rth_nominal: Value,
    /// Nominal thermal capacitance before multiplicity scaling.
    cth_nominal: Value,
    /// Optional per-instance absolute temperature override (K)
    instance_temp: Option<Value>,
    /// Optional per-instance temperature delta (K)
    instance_dtemp: Value,

    // Operating point values (for linearization)
    vbe: Value,
    vbc: Value,
    vcx: Value,
    vbi: Value,
    vci: Value,
    vbx: Value,
    vei: Value,
    vbp: Value,
    vsi: Value,
    vrth: Value,
    /// Excess-phase network states read from the MNA solution (TD > 0 only).
    vxf1: Value,
    vxf2: Value,
    vc_ext: Value,
    vb_ext: Value,
    ve_ext: Value,
    vs_ext: Value,
    /// External-terminal voltages consistent with the state the cached
    /// linearization and currents were evaluated at. Equal to the raw
    /// solution biases except on iterates where the legacy per-iterate
    /// junction limiter engaged: the companion then anchors at the limited
    /// point (ngspice's `ceqbe` uses the limited junction voltages).
    eval_anchor: [Value; EXTERNAL_DIM],
    /// The last legacy update's pnjlim materially replaced a junction
    /// voltage. ngspice sets `CKTnoncon` for this, forcing another Newton
    /// iteration: the limited companion intentionally disagrees with the
    /// raw solution, so convergence and residual acceptance must wait until
    /// the limiter disengages.
    legacy_junction_limited: bool,
    ic: Value,
    ib: Value,
    ie: Value,
    isub: Value,
    intrinsic_linearization: BjtLinearization,

    // Previous iteration values (for convergence)
    vbe_prev: Value,
    vbc_prev: Value,
    vcx_prev: Value,
    vbi_prev: Value,
    vci_prev: Value,
    vbx_prev: Value,
    vei_prev: Value,
    vbp_prev: Value,
    vsi_prev: Value,
    vrth_prev: Value,
    ic_prev: Value,
    ib_prev: Value,
    ie_prev: Value,
    isub_prev: Value,
    intrinsic_linearization_prev: BjtLinearization,

    /// Pre-computed matrix indices for O(1) stamping
    pub indices: BjtIndices,
    reduced_linearization_cache: Cell<BjtReducedLinearization>,
    reduced_linearization_cache_valid: Cell<bool>,
    previous_reduced_linearization: BjtReducedLinearization,
    previous_reduced_linearization_valid: bool,
    charge_snapshot_cache: Cell<BjtChargeSnapshot>,
    charge_snapshot_cache_valid: Cell<bool>,
    /// Static linearization at the limited MNA bias, written by
    /// `update_vbic_mna` and consumed by the promoted stamp paths.
    mna_eval: Option<EvaluatedBjtState>,
    /// Excess-phase algebraic rows (delta-iciei, ixf1, ixf2) at the limited
    /// MNA bias (TD > 0 only).
    mna_delay_branches: [BjtCurrentBranch; 3],
    /// Excess-phase correction to the thermal power row (TD > 0 with
    /// self-heating only).
    mna_delay_thermal: BjtCurrentBranch,
    /// Dynamic charge branches at the limited MNA bias, computed on demand
    /// for the transient companion and AC passes.
    mna_charge_cache: Cell<[BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT]>,
    mna_charge_cache_valid: Cell<bool>,
    thermal_variant_cache: RefCell<Vec<(u64, Box<Bjt>)>>,
}

impl Bjt {
    const VBIC_CONVERGENCE_BRANCH_COUNT: usize = 10;
    const THERMAL_VARIANT_CACHE_CAPACITY: usize = 4;

    /// Create a new NPN BJT with default 2N2222 parameters
    pub fn new_npn(name: String, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self::new(name, BjtType::Npn, collector, base, emitter)
    }

    /// Create a new PNP BJT with default 2N2907 parameters
    pub fn new_pnp(name: String, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self::new(name, BjtType::Pnp, collector, base, emitter)
    }

    /// Set the circuit-level junction GMIN (ngspice `CKTgmin`) carried in
    /// parallel with every nonlinear branch of the bjt/vbic loads.
    #[inline]
    pub fn set_junction_gmin(&mut self, gmin: Value) {
        self.junction_gmin = gmin.max(0.0);
    }

    fn new(
        name: String,
        bjt_type: BjtType,
        collector: NodeId,
        base: NodeId,
        emitter: NodeId,
    ) -> Self {
        Self {
            name,
            bjt_type,
            substrate_topology: BjtSubstrateTopology::default_for_type(bjt_type),
            charge_model: BjtChargeModel::LegacyGummelPoon,
            node_collector: collector,
            node_base: base,
            node_emitter: emitter,
            node_substrate: 0,
            node_cx: 0,
            node_ci: 0,
            node_bx: 0,
            node_bi: 0,
            node_ei: 0,
            node_bp: 0,
            node_si: 0,
            node_rth: 0,
            node_xf1: 0,
            node_xf2: 0,
            vbic_mna_promoted: false,

            // Default parameters (2N2222-like for NPN)
            is: 1e-14,          // Saturation current
            bf: 200.0,          // Forward current gain
            br: 1.0,            // Reverse current gain
            nf: 1.0,            // Forward emission coefficient
            nr: 1.0,            // Reverse emission coefficient
            vt: 0.025851999786, // Thermal voltage at 300K
            tnom: crate::analysis::temperature::T_NOMINAL,
            ambient_temperature: crate::analysis::temperature::T_NOMINAL,
            temperature: crate::analysis::temperature::T_NOMINAL,
            xti: 3.0,
            eg: 1.11,
            vje: 0.75, // B-E built-in potential
            vjc: 0.75, // B-C built-in potential
            fc: 0.9,
            vaf: 100.0,         // Forward Early voltage
            var: f64::INFINITY, // Reverse Early voltage
            rb: 10.0,           // Base resistance
            rc: 1.0,            // Collector resistance
            re: 0.1,            // Emitter resistance
            rbx: 10.0,          // Preserve legacy constant RB via RBX
            rbi: 0.0,
            rcx: 1.0, // Preserve legacy constant RC via RCX
            rci: 0.0,
            rs: 0.0,
            rbp: 0.1,
            vo: 0.0,
            gamm: 0.0,
            hrcf: 1.0,
            avc1: 0.0,
            avc2: 0.0,
            tavc: 0.0,
            xis: 3.0,
            xii: 3.0,
            xin: 3.0,
            xisr: 0.0,
            xre: 0.0,
            xrbi: 0.0,
            xrci: 0.0,
            xrs: 0.0,
            xvo: 0.0,
            xrbp: 0.0,
            tnf: 0.0,
            xikf: 0.0,
            xrcx: 0.0,
            xrbx: 0.0,
            ea: 1.12,
            eaie: 1.12,
            eaic: 1.12,
            eais: 1.12,
            eane: 1.12,
            eanc: 1.12,
            eans: 1.12,
            dear: 0.0,
            eap: 1.12,

            // Gummel-Poon parameters
            cje: 1e-12, // B-E junction capacitance
            aje: -0.5,
            mje: 0.33,    // B-E grading coefficient
            cjc: 0.5e-12, // B-C junction capacitance
            xcjc: 1.0,
            cjcp: 0.0, // C-S junction capacitance
            cjep: 0.0,
            cbeo: 0.0,
            cbco: 0.0,
            qco: 0.0,
            ccso: 0.0,
            ps: 0.75,
            ms: 0.33,
            ajs: -0.5,
            wbe: 1.0,
            vbbe: 0.0,
            nbbe: 1.0,
            ibbe: 1e-6,
            ebbe: 1.0,
            tvbbe1: 0.0,
            tvbbe2: 0.0,
            tnbbe: 0.0,
            junction_gmin: 0.0,
            ajc: -0.5,
            mjc: 0.33, // B-C grading coefficient
            tf: 4e-10, // Forward transit time (400ps)
            qtf: 0.0,
            xtf: 0.0,
            vtf: 0.0,
            itf: 0.0,
            tr: 5e-9, // Reverse transit time (5ns)
            td: 0.0,
            rth: 0.0,
            cth: 0.0,
            selft: 0.0,
            selft_given: false,
            ikf: 0.0,
            ikr: 0.0,
            qbm: 0.0,
            nkf: 0.5,
            nkf_given: false,
            isrr: 1.0,
            isp: 0.0,
            wsp: 1.0,
            nfp: 1.0,
            ikp: 0.0,
            ibeip: 0.0,
            ibenp: 0.0,
            ibcip: 0.0,
            ibcnp: 0.0,
            ncip: 1.0,
            ncnp: 2.0,
            area: 1.0,
            m: 1.0,
            initial_off: false,
            kf: 0.0,
            af: 1.0,
            ef: 1.0,
            noise_temperature_offset: 0.0,
            kfn: 0.0,
            afn: 1.0,
            bfn: 1.0,
            ibei: 5e-17, // Derived from IS/BF defaults
            iben: 0.0,
            ibci: 1e-14, // Derived from IS/BR defaults
            ibcn: 0.0,
            nei: 1.0,
            nen: 2.0,
            nci: 1.0,
            ncn: 2.0,
            nf_nominal: 1.0,
            nr_nominal: 1.0,
            is_nominal: 1e-14,
            vje_nominal: 0.75,
            vjc_nominal: 0.75,
            re_nominal: 0.1,
            rbx_nominal: 10.0,
            rbi_nominal: 0.0,
            rcx_nominal: 1.0,
            rci_nominal: 0.0,
            vo_nominal: 0.0,
            gamm_nominal: 0.0,
            cje_nominal: 1e-12,
            cjc_nominal: 0.5e-12,
            cjcp_nominal: 0.0,
            cjep_nominal: 0.0,
            cbeo_nominal: 0.0,
            cbco_nominal: 0.0,
            qco_nominal: 0.0,
            ccso_nominal: 0.0,
            ps_nominal: 0.75,
            ikf_nominal: 0.0,
            ikr_nominal: 0.0,
            isrr_nominal: 1.0,
            ibei_nominal: 5e-17,
            iben_nominal: 0.0,
            ibci_nominal: 1e-14,
            ibcn_nominal: 0.0,
            vbbe_nominal: 0.0,
            nbbe_nominal: 1.0,
            ibbe_nominal: 1e-6,
            isp_nominal: 0.0,
            ibeip_nominal: 0.0,
            ibenp_nominal: 0.0,
            ibcip_nominal: 0.0,
            ibcnp_nominal: 0.0,
            rs_nominal: 0.0,
            rbp_nominal: 0.1,
            avc2_nominal: 0.0,
            rth_nominal: 0.0,
            cth_nominal: 0.0,
            instance_temp: None,
            instance_dtemp: 0.0,

            vbe: 0.0,
            vbc: 0.0,
            vcx: 0.0,
            vbi: 0.0,
            vci: 0.0,
            vbx: 0.0,
            vei: 0.0,
            vbp: 0.0,
            vsi: 0.0,
            vrth: 0.0,
            vxf1: 0.0,
            vxf2: 0.0,
            vc_ext: 0.0,
            vb_ext: 0.0,
            ve_ext: 0.0,
            vs_ext: 0.0,
            eval_anchor: [0.0; EXTERNAL_DIM],
            legacy_junction_limited: false,
            ic: 0.0,
            ib: 0.0,
            ie: 0.0,
            isub: 0.0,
            intrinsic_linearization: BjtLinearization::default(),
            vbe_prev: 0.0,
            vbc_prev: 0.0,
            vcx_prev: 0.0,
            vbi_prev: 0.0,
            vci_prev: 0.0,
            vbx_prev: 0.0,
            vei_prev: 0.0,
            vbp_prev: 0.0,
            vsi_prev: 0.0,
            vrth_prev: 0.0,
            ic_prev: 0.0,
            ib_prev: 0.0,
            ie_prev: 0.0,
            isub_prev: 0.0,
            intrinsic_linearization_prev: BjtLinearization::default(),
            indices: BjtIndices::default(),
            reduced_linearization_cache: Cell::new(BjtReducedLinearization::default()),
            reduced_linearization_cache_valid: Cell::new(false),
            previous_reduced_linearization: BjtReducedLinearization::default(),
            previous_reduced_linearization_valid: false,
            charge_snapshot_cache: Cell::new(BjtChargeSnapshot::default()),
            charge_snapshot_cache_valid: Cell::new(false),
            mna_eval: None,
            mna_delay_branches: [BjtCurrentBranch::default(); 3],
            mna_delay_thermal: BjtCurrentBranch::default(),
            mna_charge_cache: Cell::new([BjtChargeBranch::default(); BJT_DYNAMIC_CHARGE_COUNT]),
            mna_charge_cache_valid: Cell::new(false),
            thermal_variant_cache: RefCell::new(Vec::new()),
        }
    }

    #[inline]
    fn same_cached_bias(now: Value, cached: Value) -> bool {
        (now - cached).abs() <= f64::EPSILON * now.abs().max(cached.abs()).max(1.0)
    }

    #[inline]
    fn cache_matches_external_biases(&self, vc: Value, vb: Value, ve: Value, vs: Value) -> bool {
        Self::same_cached_bias(vc, self.vc_ext)
            && Self::same_cached_bias(vb, self.vb_ext)
            && Self::same_cached_bias(ve, self.ve_ext)
            && Self::same_cached_bias(vs, self.vs_ext)
    }

    /// External voltages the companion equivalent current anchors at. When
    /// the call matches the cached evaluation, the cached anchor carries the
    /// per-iterate junction-limited point (raw biases otherwise); off-cache
    /// callers evaluate fresh at the raw biases, so those anchor raw.
    #[inline]
    fn companion_anchor(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> [Value; EXTERNAL_DIM] {
        if self.reduced_linearization_cache_valid.get()
            && self.cache_matches_external_biases(vc, vb, ve, vs)
        {
            self.eval_anchor
        } else {
            [vc, vb, ve, vs]
        }
    }

    #[inline]
    fn thermal_voltage_at(temp_k: Value) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;
        K_BOLTZMANN * temp_k.max(1.0) / Q_ELECTRON
    }

    #[inline]
    fn instance_scale(&self) -> Value {
        (self.area * self.m).max(1e-18)
    }

    #[inline]
    pub(crate) fn is_initially_off(&self) -> bool {
        self.initial_off
    }

    #[inline]
    fn requested_temperature(&self) -> Value {
        let base = self.instance_temp.unwrap_or(self.ambient_temperature);
        (base + self.instance_dtemp).max(1.0)
    }

    #[inline]
    fn self_heating_enabled(&self) -> bool {
        self.charge_model == BjtChargeModel::Vbic
            && self.rth_nominal > 0.0
            // The shipped ngspice VBIC regression decks include `RTH` on several
            // level-4 models whose reference outputs match the non-self-heated
            // solution unless `SELFT` is explicitly enabled.
            && self.selft_given
            && self.selft >= 0.5
    }

    #[inline]
    fn thermal_conductance(&self) -> Value {
        if !self.self_heating_enabled() {
            return 0.0;
        }
        self.instance_scale() / self.rth_nominal.max(1e-18)
    }

    #[inline]
    pub(crate) fn thermal_capacitance(&self) -> Value {
        if !self.self_heating_enabled() {
            return 0.0;
        }
        self.cth_nominal.max(0.0) * self.instance_scale()
    }

    #[inline]
    fn node_voltage(voltages: &[Value], node: NodeId) -> Value {
        if node == 0 { 0.0 } else { voltages[node - 1] }
    }

    #[inline]
    fn external_terminal_nodes(&self) -> [NodeId; EXTERNAL_DIM] {
        [
            self.node_collector,
            self.node_base,
            self.node_emitter,
            self.node_substrate,
        ]
    }

    #[inline]
    fn external_terminal_voltages(&self, voltages: &[Value]) -> [Value; EXTERNAL_DIM] {
        self.external_terminal_nodes()
            .map(|node| Self::node_voltage(voltages, node))
    }

    #[inline]
    fn branch_from_internal(
        current: Value,
        d_internal: [Value; INTERNAL_DIM],
    ) -> BranchLinearization {
        BranchLinearization {
            current,
            d_internal,
            d_external: [0.0; EXTERNAL_DIM],
        }
    }

    #[inline]
    fn add_branches(lhs: BranchLinearization, rhs: BranchLinearization) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        branch.current = lhs.current + rhs.current;
        for idx in 0..INTERNAL_DIM {
            branch.d_internal[idx] = lhs.d_internal[idx] + rhs.d_internal[idx];
        }
        for idx in 0..EXTERNAL_DIM {
            branch.d_external[idx] = lhs.d_external[idx] + rhs.d_external[idx];
        }
        branch
    }

    #[inline]
    fn sub_branches(lhs: BranchLinearization, rhs: BranchLinearization) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        branch.current = lhs.current - rhs.current;
        for idx in 0..INTERNAL_DIM {
            branch.d_internal[idx] = lhs.d_internal[idx] - rhs.d_internal[idx];
        }
        for idx in 0..EXTERNAL_DIM {
            branch.d_external[idx] = lhs.d_external[idx] - rhs.d_external[idx];
        }
        branch
    }

    #[inline]
    fn scale_branch(branch: BranchLinearization, factor: Value) -> BranchLinearization {
        let mut scaled = BranchLinearization::default();
        scaled.current = branch.current * factor;
        for idx in 0..INTERNAL_DIM {
            scaled.d_internal[idx] = branch.d_internal[idx] * factor;
        }
        for idx in 0..EXTERNAL_DIM {
            scaled.d_external[idx] = branch.d_external[idx] * factor;
        }
        scaled
    }

    #[inline]
    fn branch_from_vbe_vbc(current: Value, d_dvbe: Value, d_dvbc: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        branch.current = current;
        branch.d_internal[IDX_VBI] = d_dvbe + d_dvbc;
        branch.d_internal[IDX_VCI] = -d_dvbc;
        branch.d_internal[IDX_VEI] = -d_dvbe;
        branch
    }

    #[inline]
    fn power_from_branch(
        current_branch: BranchLinearization,
        voltage: Value,
        d_voltage_internal: [Value; INTERNAL_DIM],
        d_voltage_external: [Value; EXTERNAL_DIM],
    ) -> BranchLinearization {
        let mut power = BranchLinearization::default();
        power.current = current_branch.current * voltage;
        for idx in 0..INTERNAL_DIM {
            power.d_internal[idx] = current_branch.d_internal[idx] * voltage
                + current_branch.current * d_voltage_internal[idx];
        }
        for idx in 0..EXTERNAL_DIM {
            power.d_external[idx] = current_branch.d_external[idx] * voltage
                + current_branch.current * d_voltage_external[idx];
        }
        power
    }

    #[inline]
    fn smooth_positive_floor(value: Value, floor: Value) -> (Value, Value) {
        let floor = floor.max(1e-18);
        let shifted = value - floor;
        let radius = (shifted * shifted + floor * floor).sqrt().max(1e-18);
        (
            0.5 * (shifted + radius) + floor,
            0.5 * (1.0 + shifted / radius),
        )
    }

    #[inline]
    fn uses_vbic_charge_model(params: &std::collections::HashMap<String, Value>) -> bool {
        if let Some(level) = params
            .get("LEVEL")
            .copied()
            .filter(|level| level.is_finite())
        {
            return [4.0, 9.0, 11.0, 12.0, 13.0]
                .iter()
                .any(|expected| (level - expected).abs() <= 1e-9);
        }

        [
            "RCX", "RBI", "RCI", "RS", "RBP", "VO", "GAMM", "HRCF", "AVC1", "AVC2", "TAVC", "CJEP",
            "CJCP", "AJE", "AJC", "ISP", "WSP", "NFP", "IKP", "IBEIP", "IBENP", "IBCIP", "IBCNP",
            "NCIP", "NCNP", "XRE", "XRBI", "XRCI", "XRS", "XVO", "XRBP", "TNF", "XIKF", "XRCX",
            "XRBX", "EAIS", "EANS", "EAP", "CBEO", "CBCO", "QCO", "PS", "MS", "AJS", "WBE", "QTF",
            "VBBE", "NBBE", "IBBE", "TVBBE1", "TVBBE2", "TNBBE", "EBBE", "XTF", "VTF", "ITF",
            "CCSO", "QBM", "TD", "RTH", "CTH", "SELFT",
        ]
        .iter()
        .any(|key| params.contains_key(*key))
    }
}

impl NonlinearDevice for Bjt {
    fn update(&mut self, voltages: &[Value]) {
        if self.vbic_mna_promoted {
            self.update_vbic_mna(voltages);
            return;
        }
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let previous_linearization_available = self.reduced_linearization_cache_valid.get()
            && self.cache_matches_external_biases(
                self.vc_ext,
                self.vb_ext,
                self.ve_ext,
                self.vs_ext,
            );
        self.previous_reduced_linearization = if previous_linearization_available {
            self.reduced_linearization_cache.get()
        } else {
            BjtReducedLinearization::default()
        };
        self.previous_reduced_linearization_valid = previous_linearization_available;

        self.vbe_prev = self.vbe;
        self.vbc_prev = self.vbc;
        self.vcx_prev = self.vcx;
        self.vbi_prev = self.vbi;
        self.vci_prev = self.vci;
        self.vbx_prev = self.vbx;
        self.vei_prev = self.vei;
        self.vbp_prev = self.vbp;
        self.vsi_prev = self.vsi;
        self.vrth_prev = self.vrth;
        self.ic_prev = self.ic;
        self.ib_prev = self.ib;
        self.ie_prev = self.ie;
        self.isub_prev = self.isub;
        self.intrinsic_linearization_prev = self.intrinsic_linearization;

        let mut state = self.solve_intrinsic_terminal_state(vc, vb, ve, vs);
        // ngspice bjtload.c: each NIiter iteration limits the junction
        // voltages via pnjlim against the previous iterate's limited values
        // and evaluates the model there. With the constant series
        // resistances externalized the junctions are external matrix
        // unknowns, so this per-iterate replacement is the junction step
        // control (the engine-side external scale clamp skips GP devices).
        // The companion anchor follows the limited point: ngspice's ceqbe
        // subtracts conductance times the LIMITED junction voltages, so on
        // terminals whose internal node is identity-collapsed the anchor is
        // the limited state value rather than the raw solution bias.
        let mut anchor = [vc, vb, ve, vs];
        self.legacy_junction_limited = false;
        if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            let raw_vbe = state.vbi - state.vei;
            let raw_vbc = state.vbi - state.vci;
            state = self.limit_legacy_terminal_state_against_iterate(
                state,
                self.reduced_linearization_cache_valid.get(),
            );
            self.legacy_junction_limited = (state.vbi - state.vei - raw_vbe).abs() > 1e-12
                || (state.vbi - state.vci - raw_vbc).abs() > 1e-12;
            if !Self::series_active(self.rcx) && !Self::series_active(self.rci) {
                anchor[EXT_C] = state.vci;
            }
            if !Self::series_active(self.rbx) && !Self::series_active(self.rbi) {
                anchor[EXT_B] = state.vbi;
            }
            if !Self::series_active(self.re) {
                anchor[EXT_E] = state.vei;
            }
            if !Self::series_active(self.rs) {
                anchor[EXT_S] = state.vsi;
            }
        }
        self.eval_anchor = anchor;
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let reduced = self.reduced_linearization_from_state_and_eval(state, eval, vc, vb, ve, vs);
        let terminal_currents = self.external_terminal_branches(eval);
        self.vc_ext = vc;
        self.vb_ext = vb;
        self.ve_ext = ve;
        self.vs_ext = vs;
        self.vcx = state.vcx;
        self.vci = state.vci;
        self.vbx = state.vbx;
        self.vbi = state.vbi;
        self.vei = state.vei;
        self.vbp = state.vbp;
        self.vsi = state.vsi;
        self.vrth = state.vrth;
        self.vbe = self.vbi - self.vei;
        self.vbc = self.vbi - self.vci;
        self.ic = terminal_currents[EXT_C].current;
        self.ib = terminal_currents[EXT_B].current;
        self.ie = terminal_currents[EXT_E].current;
        self.isub = terminal_currents[EXT_S].current;
        self.intrinsic_linearization = eval.linearized;
        self.reduced_linearization_cache.set(reduced);
        self.reduced_linearization_cache_valid.set(true);
        self.charge_snapshot_cache_valid.set(false);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        if self.vbic_mna_promoted {
            // The promoted stamp is linearized at the limited bias cached by
            // `update`, exactly like ngspice's limited-Vbei companion load.
            self.stamp_vbic_mna(matrix);
            return;
        }
        let biases = self.external_terminal_voltages(voltages);
        let rows = self.small_signal_row_coefficients(
            biases[EXT_C],
            biases[EXT_B],
            biases[EXT_E],
            biases[EXT_S],
        );
        let anchor =
            self.companion_anchor(biases[EXT_C], biases[EXT_B], biases[EXT_E], biases[EXT_S]);
        let nodes = self.external_terminal_nodes();
        let currents = [self.ic, self.ib, self.ie, self.isub];

        for row_idx in 0..EXTERNAL_DIM {
            let ieq = currents[row_idx]
                - (0..EXTERNAL_DIM)
                    .map(|col_idx| rows[row_idx][col_idx] * anchor[col_idx])
                    .sum::<Value>();
            for col_idx in 0..EXTERNAL_DIM {
                matrix.stamp(nodes[row_idx], nodes[col_idx], rows[row_idx][col_idx]);
            }
            matrix.stamp_rhs(nodes[row_idx], -ieq);
        }
    }

    /// Check Newton-Raphson convergence using SPICE-style voltage criteria.
    ///
    /// Uses the standard SPICE convergence test:
    ///   |delta(V)| < RELTOL * max(|V_new|, |V_old|) + VNTOL
    ///
    /// `tolerance` is VNTOL from solver configuration.
    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if self.charge_model == BjtChargeModel::Vbic {
            return self.vbic_is_converged(criteria);
        }

        self.legacy_bjt_is_converged(criteria)
    }
}

impl Bjt {
    /// Diagnostic accessor for DC iteration tracing.
    pub(crate) fn legacy_junction_limited_for_trace(&self) -> bool {
        self.legacy_junction_limited
    }

    /// Whether this device runs the legacy Gummel-Poon iterate-replacement
    /// junction limiting (ngspice bjtload.c pnjlim discipline). Newton must
    /// take full node steps for these devices: the per-iterate junction
    /// replacement is the globalization, and merit-based step shrinking
    /// fights it (junction turn-on transiently raises the raw residual).
    pub(crate) fn uses_legacy_gummel_poon(&self) -> bool {
        self.charge_model == BjtChargeModel::LegacyGummelPoon
    }
}
