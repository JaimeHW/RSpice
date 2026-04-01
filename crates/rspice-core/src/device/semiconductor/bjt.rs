//! BJT (Bipolar Junction Transistor) device model
//!
//! Implements the Ebers-Moll model for NPN and PNP transistors.
//! Supports both large-signal DC and small-signal AC analysis.

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};
use std::cell::{Cell, RefCell};

/// BJT transistor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BjtType {
    Npn,
    Pnp,
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

#[derive(Debug, Clone, Copy)]
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
    ibc: BranchLinearization,
    iciei: BranchLinearization,
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
    linearized: BjtLinearization,
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
struct BranchLinearization {
    current: Value,
    d_internal: [Value; INTERNAL_DIM],
    d_external: [Value; EXTERNAL_DIM],
}

pub(crate) const BJT_DYNAMIC_CHARGE_COUNT: usize = 11;
pub(crate) const BJT_INTERNAL_STATE_DIM: usize = DYNAMIC_INTERNAL_DIM;
pub(crate) const BJT_EXTERNAL_STATE_DIM: usize = EXTERNAL_DIM;
pub(crate) const VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT: usize = 9;
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

#[derive(Debug, Clone, Copy)]
struct ExtendedOperatingPointState {
    vcx: Value,
    vci: Value,
    vbx: Value,
    vbi: Value,
    vei: Value,
    vbp: Value,
    vsi: Value,
    vrth: Value,
    ic: Value,
    ib: Value,
    ie: Value,
    isub: Value,
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
    charge_model: BjtChargeModel,

    // Node connections
    pub node_collector: NodeId,
    pub node_base: NodeId,
    pub node_emitter: NodeId,
    /// Optional substrate node (4-terminal BJT syntax)
    pub node_substrate: NodeId,

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
    /// Flicker noise coefficient (KF)
    pub kf: Value,
    /// Flicker noise current exponent (AF)
    pub af: Value,
    /// Flicker noise frequency exponent (EF)
    pub ef: Value,
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
    vc_ext: Value,
    vb_ext: Value,
    ve_ext: Value,
    vs_ext: Value,
    ic: Value,
    ib: Value,
    ie: Value,
    isub: Value,

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

    /// Pre-computed matrix indices for O(1) stamping
    pub indices: BjtIndices,
    reduced_linearization_cache: Cell<BjtReducedLinearization>,
    reduced_linearization_cache_valid: Cell<bool>,
    charge_snapshot_cache: Cell<BjtChargeSnapshot>,
    charge_snapshot_cache_valid: Cell<bool>,
    thermal_variant_cache: RefCell<Vec<(u64, Box<Bjt>)>>,
}

impl Bjt {
    const VBIC_CONVERGENCE_BRANCH_COUNT: usize = 9;
    const THERMAL_VARIANT_CACHE_CAPACITY: usize = 4;

    /// Create a new NPN BJT with default 2N2222 parameters
    pub fn new_npn(name: String, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self::new(name, BjtType::Npn, collector, base, emitter)
    }

    /// Create a new PNP BJT with default 2N2907 parameters
    pub fn new_pnp(name: String, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self::new(name, BjtType::Pnp, collector, base, emitter)
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
            charge_model: BjtChargeModel::LegacyGummelPoon,
            node_collector: collector,
            node_base: base,
            node_emitter: emitter,
            node_substrate: 0,

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
            cjcp: 0.0,    // C-S junction capacitance
            cjep: 0.0,
            cbeo: 0.0,
            cbco: 0.0,
            qco: 0.0,
            ccso: 0.0,
            ps: 0.75,
            ms: 0.33,
            ajs: -0.5,
            wbe: 1.0,
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
            kf: 0.0,
            af: 1.0,
            ef: 1.0,
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
            vc_ext: 0.0,
            vb_ext: 0.0,
            ve_ext: 0.0,
            vs_ext: 0.0,
            ic: 0.0,
            ib: 0.0,
            ie: 0.0,
            isub: 0.0,
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
            indices: BjtIndices::default(),
            reduced_linearization_cache: Cell::new(BjtReducedLinearization::default()),
            reduced_linearization_cache_valid: Cell::new(false),
            charge_snapshot_cache: Cell::new(BjtChargeSnapshot::default()),
            charge_snapshot_cache_valid: Cell::new(false),
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
        if params
            .get("LEVEL")
            .copied()
            .filter(|level| level.is_finite())
            .is_some_and(|level| level >= 4.0)
        {
            return true;
        }

        [
            "RCX", "RBI", "RCI", "RS", "RBP", "VO", "GAMM", "HRCF", "AVC1", "AVC2", "TAVC", "CJEP",
            "CJCP", "AJE", "AJC", "ISP", "WSP", "NFP", "IKP", "IBEIP", "IBENP", "IBCIP", "IBCNP",
            "NCIP", "NCNP", "XRE", "XRBI", "XRCI", "XRS", "XVO", "XRBP", "TNF", "XIKF", "XRCX",
            "XRBX", "EAIS", "EANS", "EAP", "CBEO", "CBCO", "QCO", "PS", "MS", "AJS", "WBE", "QTF",
            "XTF", "VTF", "ITF", "CCSO", "QBM", "NKF", "TD", "RTH", "CTH", "SELFT",
        ]
        .iter()
        .any(|key| params.contains_key(*key))
    }

    #[inline]
    fn apply_legacy_spice_model_defaults(&mut self) {
        self.is_nominal = 1e-16;
        self.is = self.is_nominal;
        self.bf = 100.0;
        self.br = 1.0;
        self.nf_nominal = 1.0;
        self.nr_nominal = 1.0;
        self.nf = 1.0;
        self.nr = 1.0;
        self.vaf = f64::INFINITY;
        self.var = f64::INFINITY;
        self.rb = 0.0;
        self.rbx = 0.0;
        self.rbi = 0.0;
        self.rbx_nominal = 0.0;
        self.rbi_nominal = 0.0;
        self.rc = 0.0;
        self.rcx = 0.0;
        self.rci = 0.0;
        self.rcx_nominal = 0.0;
        self.rci_nominal = 0.0;
        self.re = 0.0;
        self.re_nominal = 0.0;
        self.cje_nominal = 0.0;
        self.cjc_nominal = 0.0;
        self.cjep_nominal = 0.0;
        self.cbeo_nominal = 0.0;
        self.cbco_nominal = 0.0;
        self.qco_nominal = 0.0;
        self.ccso_nominal = 0.0;
        self.cje = 0.0;
        self.cjc = 0.0;
        self.cjep = 0.0;
        self.cbeo = 0.0;
        self.cbco = 0.0;
        self.qco = 0.0;
        self.ccso = 0.0;
        self.tf = 0.0;
        self.qtf = 0.0;
        self.xtf = 0.0;
        self.vtf = 0.0;
        self.itf = 0.0;
        self.tr = 0.0;
        self.td = 0.0;
        self.rth_nominal = 0.0;
        self.cth_nominal = 0.0;
        self.rth = 0.0;
        self.cth = 0.0;
        self.selft = 0.0;
        self.selft_given = false;
        self.ikf_nominal = 0.0;
        self.ikr_nominal = 0.0;
        self.ikf = 0.0;
        self.ikr = 0.0;
        self.qbm = 0.0;
        self.nkf = 0.5;
        self.ibei_nominal = 0.0;
        self.ibci_nominal = 0.0;
    }

    #[inline]
    fn apply_vbic_model_defaults(&mut self) {
        self.is_nominal = 1e-16;
        self.is = self.is_nominal;
        self.nf_nominal = 1.0;
        self.nr_nominal = 1.0;
        self.nf = 1.0;
        self.nr = 1.0;
        self.vaf = 0.0;
        self.var = 0.0;
        self.rb = 0.0;
        self.rc = 0.0;
        self.rbx = 0.0;
        self.rbi = 0.1;
        self.rcx = 0.0;
        self.rci = 0.1;
        self.re = 0.0;
        self.rs = 0.0;
        self.rbp = 0.1;
        self.rbx_nominal = self.rbx;
        self.rbi_nominal = self.rbi;
        self.rcx_nominal = self.rcx;
        self.rci_nominal = self.rci;
        self.re_nominal = self.re;
        self.rs_nominal = self.rs;
        self.rbp_nominal = self.rbp;
        self.fc = 0.9;
        self.cbeo_nominal = 0.0;
        self.cbco_nominal = 0.0;
        self.cbeo = 0.0;
        self.cbco = 0.0;
        self.cje_nominal = 0.0;
        self.cjc_nominal = 0.0;
        self.cjep_nominal = 0.0;
        self.cjcp_nominal = 0.0;
        self.cje = 0.0;
        self.cjc = 0.0;
        self.cjep = 0.0;
        self.cjcp = 0.0;
        self.vje = 0.75;
        self.vjc = 0.75;
        self.vje_nominal = self.vje;
        self.vjc_nominal = self.vjc;
        self.ps = 0.75;
        self.ps_nominal = self.ps;
        self.mje = 0.33;
        self.mjc = 0.33;
        self.ms = 0.33;
        self.aje = -0.5;
        self.ajc = -0.5;
        self.ajs = -0.5;
        self.qco_nominal = 0.0;
        self.qco = 0.0;
        self.ccso_nominal = 0.0;
        self.ccso = 0.0;
        self.ibei_nominal = 1e-18;
        self.iben_nominal = 0.0;
        self.ibci_nominal = 1e-16;
        self.ibcn_nominal = 0.0;
        self.ibei = self.ibei_nominal;
        self.iben = self.iben_nominal;
        self.ibci = self.ibci_nominal;
        self.ibcn = self.ibcn_nominal;
        self.wbe = 1.0;
        self.nei = 1.0;
        self.nen = 2.0;
        self.nci = 1.0;
        self.ncn = 2.0;
        self.avc1 = 0.0;
        self.avc2_nominal = 0.0;
        self.avc2 = 0.0;
        self.isp_nominal = 0.0;
        self.isp = 0.0;
        self.wsp = 1.0;
        self.nfp = 1.0;
        self.ibeip_nominal = 0.0;
        self.ibenp_nominal = 0.0;
        self.ibcip_nominal = 0.0;
        self.ibcnp_nominal = 0.0;
        self.ibeip = 0.0;
        self.ibenp = 0.0;
        self.ibcip = 0.0;
        self.ibcnp = 0.0;
        self.ncip = 1.0;
        self.ncnp = 2.0;
        self.vo_nominal = 0.0;
        self.vo = 0.0;
        self.gamm_nominal = 0.0;
        self.gamm = 0.0;
        self.hrcf = 1.0;
        self.ikf_nominal = 0.0;
        self.ikr_nominal = 0.0;
        self.ikf = 0.0;
        self.ikr = 0.0;
        self.ikp = 0.0;
        self.tf = 0.0;
        self.qtf = 0.0;
        self.xtf = 0.0;
        self.vtf = 0.0;
        self.itf = 0.0;
        self.tr = 0.0;
        self.td = 0.0;
        self.rth_nominal = 0.0;
        self.cth_nominal = 0.0;
        self.rth = 0.0;
        self.cth = 0.0;
        self.selft = 0.0;
        self.selft_given = false;
        self.kf = 0.0;
        self.af = 1.0;
        self.xre = 0.0;
        self.xrbi = 0.0;
        self.xrci = 0.0;
        self.xrs = 0.0;
        self.xvo = 0.0;
        self.xrbp = 0.0;
        self.ea = 1.12;
        self.eaie = 1.12;
        self.eaic = 1.12;
        self.eais = 1.12;
        self.eane = 1.12;
        self.eanc = 1.12;
        self.eans = 1.12;
        self.eap = 1.12;
        self.xis = 3.0;
        self.xii = 3.0;
        self.xin = 3.0;
        self.tnf = 0.0;
        self.tavc = 0.0;
        self.qbm = 0.0;
        self.nkf = 0.5;
        self.xikf = 0.0;
        self.xrcx = 0.0;
        self.xrbx = 0.0;
        self.isrr_nominal = 1.0;
        self.isrr = 1.0;
        self.xisr = 0.0;
        self.dear = 0.0;
    }

    #[inline]
    pub(crate) fn uses_vbic_dynamic_charges(&self) -> bool {
        self.charge_model == BjtChargeModel::Vbic
    }

    #[inline]
    pub(crate) fn has_vbic_self_heating(&self) -> bool {
        self.self_heating_enabled()
    }

    #[inline]
    fn vbic_temp_scaled_current(
        nominal: Value,
        r_t: Value,
        vtv: Value,
        temp_exponent: Value,
        activation_energy: Value,
        emission_coeff: Value,
    ) -> Value {
        if nominal <= 0.0 {
            return 0.0;
        }

        let emission = emission_coeff.max(1e-12);
        let ratio_term = r_t.max(1e-18).powf(temp_exponent);
        let energy_term = (-activation_energy * (1.0 - r_t) / vtv.max(1e-18)).clamp(-80.0, 80.0);
        let scaled = (ratio_term * energy_term.exp()).max(0.0);
        nominal * scaled.powf(1.0 / emission)
    }

    #[inline]
    fn vbic_temp_scaled_resistance(nominal: Value, r_t: Value, temp_exponent: Value) -> Value {
        if nominal <= 0.0 {
            return 0.0;
        }

        nominal * r_t.max(1e-18).powf(temp_exponent)
    }

    #[inline]
    fn vbic_log_exp_difference(x: Value) -> Value {
        if x > 40.0 {
            x + (1.0 - (-2.0 * x).exp()).ln()
        } else {
            ((x.exp() - (-x).exp()).max(1e-300)).ln()
        }
    }

    #[inline]
    fn vbic_temp_scaled_potential(
        nominal: Value,
        r_t: Value,
        vtv: Value,
        activation_energy: Value,
    ) -> Value {
        if nominal <= 0.0 {
            return nominal;
        }

        let vt_safe = vtv.max(1e-18);
        let ratio = r_t.max(1e-18);
        let arg = 0.5 * nominal * ratio / vt_safe;
        let psiio = 2.0 * (vt_safe / ratio) * Self::vbic_log_exp_difference(arg);
        let psiin = psiio * ratio - 3.0 * vt_safe * ratio.ln() - activation_energy * (ratio - 1.0);
        let expo = (-psiin / vt_safe).clamp(-80.0, 80.0).exp();
        let correction = 0.5 * (1.0 + (1.0 + 4.0 * expo).sqrt());
        (psiin + 2.0 * vt_safe * correction.ln()).max(1e-12)
    }

    fn refresh_operating_scaling(&mut self) {
        let temp = self.requested_temperature();
        self.refresh_operating_scaling_for(temp);
    }

    #[inline]
    fn clear_thermal_variant_cache(&self) {
        self.thermal_variant_cache.borrow_mut().clear();
    }

    fn clone_without_thermal_variant_cache(&self) -> Self {
        let saved_cache = {
            let mut cache = self.thermal_variant_cache.borrow_mut();
            std::mem::take(&mut *cache)
        };
        let clone = self.clone();
        *self.thermal_variant_cache.borrow_mut() = saved_cache;
        clone.thermal_variant_cache.borrow_mut().clear();
        clone
    }

    pub(crate) fn vbic_collector_substrate_charge_homotopy_variant(&self, lambda: Value) -> Self {
        let scale = lambda.clamp(0.0, 1.0);
        let mut variant = self.clone_without_thermal_variant_cache();
        variant.reduced_linearization_cache_valid.set(false);
        variant.charge_snapshot_cache_valid.set(false);

        if variant.charge_model != BjtChargeModel::Vbic {
            return variant;
        }

        variant.qco_nominal *= scale;
        variant.cjcp_nominal *= scale;
        variant.ccso_nominal *= scale;
        variant.qco *= scale;
        variant.cjcp *= scale;
        variant.ccso *= scale;
        variant
    }

    fn with_temperature_variant<R>(&self, thermal_rise: Value, f: impl FnOnce(&Self) -> R) -> R {
        if !self.self_heating_enabled() {
            return f(self);
        }

        let key = thermal_rise.to_bits();
        {
            let cache = self.thermal_variant_cache.borrow();
            if let Some((_, variant)) = cache.iter().find(|(cached_key, _)| *cached_key == key) {
                return f(variant.as_ref());
            }
        }

        let mut variant = self.clone_without_thermal_variant_cache();
        variant
            .refresh_operating_scaling_for((self.requested_temperature() + thermal_rise).max(1.0));
        let result = f(&variant);

        let mut cache = self.thermal_variant_cache.borrow_mut();
        if cache.len() >= Self::THERMAL_VARIANT_CACHE_CAPACITY {
            cache.remove(0);
        }
        cache.push((key, Box::new(variant)));
        result
    }

    fn refresh_operating_scaling_for(&mut self, temp: Value) {
        self.clear_thermal_variant_cache();
        let tnom = self.tnom.max(1.0);
        let vt = Self::thermal_voltage_at(temp);
        let ratio = (temp / tnom).max(1e-12);
        let delta_t = temp - tnom;
        let is_temp =
            Self::vbic_temp_scaled_current(self.is_nominal, ratio, vt, self.xis, self.ea, self.nf);
        let scale = self.instance_scale();
        let isrr_temp = Self::vbic_temp_scaled_current(
            self.isrr_nominal,
            ratio,
            vt,
            self.xisr,
            self.dear,
            self.nr,
        );
        let gamm_ratio_term = ratio.powf(self.xis);
        let gamm_energy_term = (-self.ea * (1.0 - ratio) / vt.max(1e-18)).clamp(-80.0, 80.0);
        let gamm_temp = self.gamm_nominal * gamm_ratio_term * gamm_energy_term.exp();
        let ibei_temp = Self::vbic_temp_scaled_current(
            self.ibei_nominal,
            ratio,
            vt,
            self.xii,
            self.eaie,
            self.nei,
        );
        let iben_temp = Self::vbic_temp_scaled_current(
            self.iben_nominal,
            ratio,
            vt,
            self.xin,
            self.eane,
            self.nen,
        );
        let ibci_temp = Self::vbic_temp_scaled_current(
            self.ibci_nominal,
            ratio,
            vt,
            self.xii,
            self.eaic,
            self.nci,
        );
        let ibcn_temp = Self::vbic_temp_scaled_current(
            self.ibcn_nominal,
            ratio,
            vt,
            self.xin,
            self.eanc,
            self.ncn,
        );
        let isp_temp = Self::vbic_temp_scaled_current(
            self.isp_nominal,
            ratio,
            vt,
            self.xis,
            self.eap,
            self.nfp,
        );
        let ibeip_temp = Self::vbic_temp_scaled_current(
            self.ibeip_nominal,
            ratio,
            vt,
            self.xii,
            self.eaic,
            self.nci,
        );
        let ibenp_temp = Self::vbic_temp_scaled_current(
            self.ibenp_nominal,
            ratio,
            vt,
            self.xin,
            self.eanc,
            self.ncn,
        );
        let ibcip_temp = Self::vbic_temp_scaled_current(
            self.ibcip_nominal,
            ratio,
            vt,
            self.xii,
            self.eais,
            self.ncip,
        );
        let ibcnp_temp = Self::vbic_temp_scaled_current(
            self.ibcnp_nominal,
            ratio,
            vt,
            self.xin,
            self.eans,
            self.ncnp,
        );
        let re_temp = Self::vbic_temp_scaled_resistance(self.re_nominal, ratio, self.xre);
        let rbx_temp = Self::vbic_temp_scaled_resistance(self.rbx_nominal, ratio, self.xrbx);
        let rbi_temp = Self::vbic_temp_scaled_resistance(self.rbi_nominal, ratio, self.xrbi);
        let rcx_temp = Self::vbic_temp_scaled_resistance(self.rcx_nominal, ratio, self.xrcx);
        let rci_temp = Self::vbic_temp_scaled_resistance(self.rci_nominal, ratio, self.xrci);
        let rs_temp = Self::vbic_temp_scaled_resistance(self.rs_nominal, ratio, self.xrs);
        let rbp_temp = Self::vbic_temp_scaled_resistance(self.rbp_nominal, ratio, self.xrbp);
        let vo_temp = if self.vo_nominal > 0.0 {
            self.vo_nominal * ratio.powf(self.xvo)
        } else {
            0.0
        };
        let vje_temp = Self::vbic_temp_scaled_potential(self.vje_nominal, ratio, vt, self.eaie);
        let vjc_temp = Self::vbic_temp_scaled_potential(self.vjc_nominal, ratio, vt, self.eaic);
        let ps_temp = Self::vbic_temp_scaled_potential(self.ps_nominal, ratio, vt, self.eais);
        let nf_temp = self.nf_nominal * (1.0 + delta_t * self.tnf);
        let nr_temp = self.nr_nominal * (1.0 + delta_t * self.tnf);
        let avc2_temp = self.avc2_nominal * (1.0 + (temp - self.tnom) * self.tavc);
        let ikf_temp = if self.ikf_nominal > 0.0 {
            self.ikf_nominal * ratio.powf(self.xikf)
        } else {
            0.0
        };

        self.vt = vt;
        self.temperature = temp;
        self.is = (is_temp * scale).max(1e-30);
        self.nf = nf_temp.max(1e-12);
        self.nr = nr_temp.max(1e-12);
        self.re = re_temp.max(0.0);
        self.rbx = rbx_temp.max(0.0);
        self.rbi = rbi_temp.max(0.0);
        self.rcx = rcx_temp.max(0.0);
        self.rci = rci_temp.max(0.0);
        self.vje = vje_temp;
        self.vjc = vjc_temp;
        self.ps = ps_temp;
        self.cje =
            (self.cje_nominal * (self.vje_nominal / vje_temp.max(1e-18)).powf(self.mje) * scale)
                .max(0.0);
        self.cjc =
            (self.cjc_nominal * (self.vjc_nominal / vjc_temp.max(1e-18)).powf(self.mjc) * scale)
                .max(0.0);
        self.cjcp =
            (self.cjcp_nominal * (self.ps_nominal / ps_temp.max(1e-18)).powf(self.ms) * scale)
                .max(0.0);
        self.cjep =
            (self.cjep_nominal * (self.vjc_nominal / vjc_temp.max(1e-18)).powf(self.mjc) * scale)
                .max(0.0);
        self.cbeo = (self.cbeo_nominal * scale).max(0.0);
        self.cbco = (self.cbco_nominal * scale).max(0.0);
        self.qco = (self.qco_nominal * scale).max(0.0);
        self.ccso = (self.ccso_nominal * scale).max(0.0);
        self.vo = vo_temp.max(0.0);
        self.gamm = gamm_temp.max(0.0);
        self.ikf = if ikf_temp > 0.0 {
            (ikf_temp * scale).max(1e-18)
        } else {
            0.0
        };
        self.ikr = if self.ikr_nominal > 0.0 {
            (self.ikr_nominal * scale).max(1e-18)
        } else {
            0.0
        };
        self.isrr = isrr_temp.max(0.0);
        self.ibei = (ibei_temp * scale).max(0.0);
        self.iben = (iben_temp * scale).max(0.0);
        self.ibci = (ibci_temp * scale).max(0.0);
        self.ibcn = (ibcn_temp * scale).max(0.0);
        self.isp = (isp_temp * scale).max(0.0);
        self.ibeip = (ibeip_temp * scale).max(0.0);
        self.ibenp = (ibenp_temp * scale).max(0.0);
        self.ibcip = (ibcip_temp * scale).max(0.0);
        self.ibcnp = (ibcnp_temp * scale).max(0.0);
        self.rs = rs_temp.max(0.0);
        self.rbp = rbp_temp.max(0.0);
        self.avc2 = if avc2_temp.is_finite() {
            avc2_temp.max(0.0)
        } else {
            self.avc2_nominal
        };
        self.rth = self.rth_nominal.max(0.0);
        self.cth = self.thermal_capacitance();
    }

    fn temperature_variant(&self, thermal_rise: Value) -> Self {
        let mut variant = self.clone_without_thermal_variant_cache();
        let active_temperature = (self.requested_temperature() + thermal_rise).max(1.0);
        variant.refresh_operating_scaling_for(active_temperature);
        variant
    }

    /// Set active device temperature (Kelvin).
    pub fn set_temperature(&mut self, temp_k: Value) {
        if temp_k.is_finite() && temp_k > 0.0 {
            self.ambient_temperature = temp_k;
            self.refresh_operating_scaling();
        }
    }

    /// Set optional substrate node (0 for ground/unconnected).
    pub fn set_substrate_node(&mut self, substrate: NodeId) {
        self.node_substrate = substrate;
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        let mut has_vaf = false;
        let mut has_var = false;
        let mut has_rb = false;
        let mut has_rc = false;
        let mut has_ibei = false;
        let mut has_ibci = false;
        let mut has_rth = false;
        self.charge_model = if Self::uses_vbic_charge_model(params) {
            BjtChargeModel::Vbic
        } else {
            BjtChargeModel::LegacyGummelPoon
        };
        match self.charge_model {
            BjtChargeModel::LegacyGummelPoon => self.apply_legacy_spice_model_defaults(),
            BjtChargeModel::Vbic => self.apply_vbic_model_defaults(),
        }

        // DC parameters
        if let Some(&v) = params.get("IS") {
            self.is_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("BF") {
            self.bf = v;
        }
        if let Some(&v) = params.get("BR") {
            self.br = v;
        }
        if let Some(&v) = params.get("NF") {
            self.nf_nominal = v;
            self.nf = v;
        }
        if let Some(&v) = params.get("NR") {
            self.nr_nominal = v;
            self.nr = v;
        }
        if let Some(&v) = params.get("VAF") {
            self.vaf = v;
            has_vaf = true;
        }
        if !has_vaf && let Some(&v) = params.get("VA") {
            self.vaf = v;
            has_vaf = true;
        }
        if let Some(&v) = params.get("VAR") {
            self.var = v;
            has_var = true;
        }
        if !has_var && let Some(&v) = params.get("VB") {
            self.var = v;
            has_var = true;
        }
        if let Some(&v) = params.get("RB") {
            self.rbx = v.max(0.0);
            self.rbx_nominal = self.rbx;
            self.rbi = 0.0;
            self.rbi_nominal = self.rbi;
            self.rb = self.rbx;
            has_rb = true;
        }
        if let Some(&v) = params.get("RC") {
            self.rcx = v.max(0.0);
            self.rcx_nominal = self.rcx;
            self.rci = 0.0;
            self.rci_nominal = self.rci;
            self.rc = self.rcx;
            has_rc = true;
        }
        if let Some(&v) = params.get("RE") {
            self.re = v;
            self.re_nominal = self.re.max(0.0);
        }
        if let Some(&v) = params.get("RS") {
            self.rs_nominal = v.max(0.0);
            self.rs = self.rs_nominal;
        }
        if let Some(&v) = params.get("RBP") {
            self.rbp_nominal = v.max(0.0);
            self.rbp = self.rbp_nominal;
        }
        if let Some(&v) = params.get("XTI")
            && v.is_finite()
            && v > 0.0
        {
            self.xti = v;
            self.xis = v;
            self.xii = v;
            self.xin = v;
        }
        if let Some(&v) = params.get("XIS")
            && v.is_finite()
        {
            self.xis = v;
        }
        if let Some(&v) = params.get("XII")
            && v.is_finite()
        {
            self.xii = v;
        }
        if let Some(&v) = params.get("XIN")
            && v.is_finite()
        {
            self.xin = v;
        }
        if let Some(&v) = params.get("XISR")
            && v.is_finite()
        {
            self.xisr = v;
        }
        if let Some(&v) = params.get("XRE")
            && v.is_finite()
        {
            self.xre = v;
        }
        if let Some(&v) = params.get("XRBI")
            && v.is_finite()
        {
            self.xrbi = v;
        }
        if let Some(&v) = params.get("XRCI")
            && v.is_finite()
        {
            self.xrci = v;
        }
        if let Some(&v) = params.get("XRS")
            && v.is_finite()
        {
            self.xrs = v;
        }
        if let Some(&v) = params.get("XVO")
            && v.is_finite()
        {
            self.xvo = v;
        }
        if let Some(&v) = params.get("XRBP")
            && v.is_finite()
        {
            self.xrbp = v;
        }
        if let Some(&v) = params.get("TNF")
            && v.is_finite()
        {
            self.tnf = v;
        }
        if let Some(&v) = params.get("XIKF")
            && v.is_finite()
        {
            self.xikf = v;
        }
        if let Some(&v) = params.get("XRCX")
            && v.is_finite()
        {
            self.xrcx = v;
        }
        if let Some(&v) = params.get("XRBX")
            && v.is_finite()
        {
            self.xrbx = v;
        }
        if let Some(&v) = params.get("EG")
            && v.is_finite()
            && v > 0.0
        {
            self.eg = v;
            self.ea = v;
            self.eaie = v;
            self.eaic = v;
            self.eais = v;
            self.eane = v;
            self.eanc = v;
            self.eans = v;
            self.eap = v;
        }
        if let Some(&v) = params.get("EA")
            && v.is_finite()
            && v > 0.0
        {
            self.ea = v;
        }
        if let Some(&v) = params.get("EAIE")
            && v.is_finite()
            && v > 0.0
        {
            self.eaie = v;
        }
        if let Some(&v) = params.get("EAIC")
            && v.is_finite()
            && v > 0.0
        {
            self.eaic = v;
        }
        if let Some(&v) = params.get("EANE")
            && v.is_finite()
            && v > 0.0
        {
            self.eane = v;
        }
        if let Some(&v) = params.get("EANC")
            && v.is_finite()
            && v > 0.0
        {
            self.eanc = v;
        }
        if let Some(&v) = params.get("EAIS")
            && v.is_finite()
            && v > 0.0
        {
            self.eais = v;
        }
        if let Some(&v) = params.get("EANS")
            && v.is_finite()
            && v > 0.0
        {
            self.eans = v;
        }
        if let Some(&v) = params.get("EAP")
            && v.is_finite()
            && v > 0.0
        {
            self.eap = v;
        }
        if let Some(&v) = params.get("DEAR")
            && v.is_finite()
        {
            self.dear = v;
        }
        if let Some(&v) = params.get("TNOM")
            && v.is_finite()
            && v > 0.0
        {
            self.tnom = if v > 200.0 { v } else { v + 273.15 };
        }
        if let Some(v) = params
            .get("KF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.kf = v;
        }
        if let Some(v) = params
            .get("AF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.af = v;
        }
        if let Some(v) = params
            .get("EF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.ef = v;
        }
        // VBIC aliases used in ngspice level=4 decks.
        if !has_vaf
            && let Some(&v) = params.get("VEF")
            && v.is_finite()
            && v > 0.0
        {
            self.vaf = v;
        }
        if !has_var
            && let Some(&v) = params.get("VER")
            && v.is_finite()
            && v > 0.0
        {
            self.var = v;
        }
        if !has_rb {
            let rbx = params
                .get("RBX")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            let rbi = params
                .get("RBI")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            if rbx > 0.0 || rbi > 0.0 {
                self.rbx = rbx;
                self.rbi = rbi;
                self.rbx_nominal = rbx;
                self.rbi_nominal = rbi;
                self.rb = (rbx + rbi).max(1e-12);
            }
        }
        if !has_rc {
            let rcx = params
                .get("RCX")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            let rci = params
                .get("RCI")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            if rcx > 0.0 || rci > 0.0 {
                self.rcx = rcx;
                self.rci = rci;
                self.rcx_nominal = rcx;
                self.rci_nominal = rci;
                self.rc = (rcx + rci).max(1e-12);
            }
        }
        if let Some(&v) = params.get("VO")
            && v.is_finite()
            && v >= 0.0
        {
            self.vo = v;
            self.vo_nominal = self.vo;
        }
        if let Some(&v) = params.get("GAMM")
            && v.is_finite()
            && v >= 0.0
        {
            self.gamm = v;
            self.gamm_nominal = self.gamm;
        }
        if let Some(&v) = params.get("HRCF")
            && v.is_finite()
            && v > 0.0
        {
            self.hrcf = v;
        }
        if let Some(&v) = params.get("AVC1")
            && v.is_finite()
        {
            self.avc1 = v.max(0.0);
        }
        if let Some(&v) = params.get("AVC2")
            && v.is_finite()
        {
            self.avc2_nominal = v.max(0.0);
            self.avc2 = self.avc2_nominal;
        }
        if let Some(&v) = params.get("TAVC")
            && v.is_finite()
        {
            self.tavc = v;
        }
        // Gummel-Poon charge parameters
        if let Some(&v) = params.get("CJE") {
            self.cje_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CJEP") {
            self.cjep_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("MJE") {
            self.mje = v;
        }
        if let Some(&v) = params.get("PE")
            && v.is_finite()
            && v > 0.0
        {
            self.vje = v;
            self.vje_nominal = v;
        }
        if let Some(&v) = params.get("VJE")
            && v.is_finite()
            && v > 0.0
        {
            self.vje = v;
            self.vje_nominal = v;
        }
        if let Some(&v) = params.get("AJE")
            && v.is_finite()
        {
            self.aje = v;
        }
        if let Some(&v) = params.get("CJC") {
            self.cjc_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CBEO")
            && v.is_finite()
        {
            self.cbeo_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CBCO")
            && v.is_finite()
        {
            self.cbco_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("QCO")
            && v.is_finite()
        {
            self.qco_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CJCP") {
            self.cjcp_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("MJC") {
            self.mjc = v;
        }
        if let Some(&v) = params.get("PS")
            && v.is_finite()
            && v > 0.0
        {
            self.ps = v;
            self.ps_nominal = v;
        }
        if let Some(&v) = params.get("MS")
            && v.is_finite()
        {
            self.ms = v;
        }
        if let Some(&v) = params.get("AJS")
            && v.is_finite()
        {
            self.ajs = v;
        }
        if let Some(&v) = params.get("PC")
            && v.is_finite()
            && v > 0.0
        {
            self.vjc = v;
            self.vjc_nominal = v;
        }
        if let Some(&v) = params.get("VJC")
            && v.is_finite()
            && v > 0.0
        {
            self.vjc = v;
            self.vjc_nominal = v;
        }
        if let Some(&v) = params.get("AJC")
            && v.is_finite()
        {
            self.ajc = v;
        }
        if let Some(&v) = params.get("WBE")
            && v.is_finite()
        {
            self.wbe = v.clamp(0.0, 1.0);
        }
        if let Some(&v) = params.get("FC")
            && v.is_finite()
        {
            self.fc = v.clamp(0.0, 0.999_999);
        }
        if let Some(&v) = params.get("TF") {
            self.tf = v;
        }
        if let Some(&v) = params.get("QTF")
            && v.is_finite()
        {
            self.qtf = v;
        }
        if let Some(&v) = params.get("XTF")
            && v.is_finite()
        {
            self.xtf = v;
        }
        if let Some(&v) = params.get("VTF")
            && v.is_finite()
        {
            self.vtf = v.max(0.0);
        }
        if let Some(&v) = params.get("ITF")
            && v.is_finite()
        {
            self.itf = v.max(0.0);
        }
        if let Some(&v) = params.get("TR") {
            self.tr = v;
        }
        if let Some(&v) = params.get("TD")
            && v.is_finite()
        {
            self.td = v.max(0.0);
        }
        if let Some(&v) = params.get("RTH")
            && v.is_finite()
        {
            self.rth_nominal = v.max(0.0);
            self.rth = self.rth_nominal;
            has_rth = true;
        }
        if let Some(&v) = params.get("CTH")
            && v.is_finite()
        {
            self.cth_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("SELFT")
            && v.is_finite()
        {
            self.selft = if v >= 0.5 { 1.0 } else { 0.0 };
            self.selft_given = true;
        }
        if let Some(&v) = params.get("IKF") {
            self.ikf_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IKR") {
            self.ikr_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("QBM")
            && v.is_finite()
        {
            self.qbm = v;
        }
        if let Some(&v) = params.get("NKF")
            && v.is_finite()
            && v > 0.0
        {
            self.nkf = v;
        }
        if let Some(v) = params
            .get("ISRR")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.isrr_nominal = v;
            self.isrr = v;
        }
        if let Some(&v) = params.get("ISP") {
            self.isp_nominal = v.max(0.0);
            self.isp = self.isp_nominal;
        }
        if let Some(&v) = params.get("WSP")
            && v.is_finite()
        {
            self.wsp = v;
        }
        if let Some(&v) = params.get("NFP")
            && v.is_finite()
            && v > 0.0
        {
            self.nfp = v;
        }
        if let Some(&v) = params.get("IKP") {
            self.ikp = v.max(0.0);
        }
        if let Some(&v) = params.get("IBEI") {
            self.ibei_nominal = v.max(0.0);
            has_ibei = true;
        }
        if let Some(&v) = params.get("IBEN") {
            self.iben_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBCI") {
            self.ibci_nominal = v.max(0.0);
            has_ibci = true;
        }
        if let Some(&v) = params.get("IBCN") {
            self.ibcn_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBEIP") {
            self.ibeip_nominal = v.max(0.0);
            self.ibeip = self.ibeip_nominal;
        }
        if let Some(&v) = params.get("IBENP") {
            self.ibenp_nominal = v.max(0.0);
            self.ibenp = self.ibenp_nominal;
        }
        if let Some(&v) = params.get("IBCIP") {
            self.ibcip_nominal = v.max(0.0);
            self.ibcip = self.ibcip_nominal;
        }
        if let Some(&v) = params.get("IBCNP") {
            self.ibcnp_nominal = v.max(0.0);
            self.ibcnp = self.ibcnp_nominal;
        }
        if let Some(&v) = params.get("NEI")
            && v.is_finite()
            && v > 0.0
        {
            self.nei = v;
        }
        if let Some(&v) = params.get("NEN")
            && v.is_finite()
            && v > 0.0
        {
            self.nen = v;
        }
        if let Some(&v) = params.get("NCI")
            && v.is_finite()
            && v > 0.0
        {
            self.nci = v;
        }
        if let Some(&v) = params.get("NCN")
            && v.is_finite()
            && v > 0.0
        {
            self.ncn = v;
        }
        if let Some(&v) = params.get("NCIP")
            && v.is_finite()
            && v > 0.0
        {
            self.ncip = v;
        }
        if let Some(&v) = params.get("NCNP")
            && v.is_finite()
            && v > 0.0
        {
            self.ncnp = v;
        }
        if let Some(&v) = params.get("CCSO")
            && v.is_finite()
        {
            self.ccso_nominal = v.max(0.0);
        }
        if !has_ibei && self.charge_model == BjtChargeModel::LegacyGummelPoon {
            self.ibei_nominal = self.is_nominal / self.bf.max(1e-18);
        }
        if !has_ibci && self.charge_model == BjtChargeModel::LegacyGummelPoon {
            self.ibci_nominal = self.is_nominal / self.br.max(1e-18);
        }
        if self.charge_model == BjtChargeModel::Vbic && has_rth {
            // ngspice VBIC setup semantics:
            // - If RTH is provided, clamp CTH to at least 1e-12.
            if self.cth_nominal < 1e-12 {
                self.cth_nominal = 1e-12;
            }
        }
        self.refresh_operating_scaling();
        self
    }

    /// Apply instance-level BJT scaling and thermal overrides.
    ///
    /// Supported keys:
    /// - `AREA`: area multiplier (default 1)
    /// - `M` / `MULT`: multiplicity (default 1)
    /// - `TEMP`: absolute device temperature in Celsius
    /// - `DTEMP`: temperature delta in Celsius
    pub fn with_instance_params(mut self, params: &[(String, Value)]) -> Self {
        for (name, value) in params {
            if !value.is_finite() {
                continue;
            }

            if name.eq_ignore_ascii_case("AREA") {
                if *value > 0.0 {
                    self.area = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT") {
                if *value > 0.0 {
                    self.m = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("TEMP") {
                self.instance_temp = Some(*value + 273.15);
                continue;
            }

            if name.eq_ignore_ascii_case("DTEMP") {
                self.instance_dtemp = *value;
            }
        }

        self.refresh_operating_scaling();
        self
    }

    /// Calculate base-emitter junction capacitance
    /// Cbe = CJE / (1 - Vbe/VJE)^MJE + gm * TF
    pub fn cbe(&self, vbe: Value, gm: Value) -> Value {
        let p = self.polarity();
        let (_, factor) = self.vbic_depletion_charge_and_derivative(
            p * vbe,
            self.vje,
            self.mje,
            self.fc,
            self.aje,
        );
        let cj = self.cje * factor;
        let cd = gm * self.tf; // Diffusion capacitance
        cj + cd
    }

    /// Calculate base-collector junction capacitance
    /// Cbc = CJC / (1 - Vbc/VJC)^MJC
    pub fn cbc(&self, vbc: Value) -> Value {
        let p = self.polarity();
        let (_, factor) = self.vbic_depletion_charge_and_derivative(
            p * vbc,
            self.vjc,
            self.mjc,
            self.fc,
            self.ajc,
        );
        self.cjc * factor
    }

    /// Calculate total capacitances for transient analysis
    /// Returns (Cbe, Cbc)
    pub fn junction_capacitances(&self, vbe: Value, vbc: Value) -> (Value, Value) {
        let gm = self.linearize_currents(vbe, vbc).dic_dvbe.max(1e-15);
        (self.cbe(vbe, gm), self.cbc(vbc))
    }

    /// Return cached collector, base, and emitter currents at the operating point.
    pub fn operating_point_currents(&self) -> (Value, Value, Value) {
        (self.ic, self.ib, self.ie)
    }

    /// Return the net current leaving a physical node, summing any tied BJT terminals.
    pub(crate) fn node_current(&self, node: NodeId) -> Value {
        let mut current = 0.0;
        if self.node_collector == node {
            current += self.ic;
        }
        if self.node_base == node {
            current += self.ib;
        }
        if self.node_emitter == node {
            current += self.ie;
        }
        if self.node_substrate == node {
            current += self.isub;
        }
        current
    }

    #[cfg(test)]
    fn operating_point_state(&self) -> ExtendedOperatingPointState {
        ExtendedOperatingPointState {
            vcx: self.vcx,
            vci: self.vci,
            vbx: self.vbx,
            vbi: self.vbi,
            vei: self.vei,
            vbp: self.vbp,
            vsi: self.vsi,
            vrth: self.vrth,
            ic: self.ic,
            ib: self.ib,
            ie: self.ie,
            isub: self.isub,
        }
    }

    /// Return the shot-noise branch currents referenced to the physical junctions.
    pub fn noise_branch_currents(&self) -> (Value, Value, Value) {
        let vp_be = self.polarity() * self.vbe;
        let vp_bc = self.polarity() * self.vbc;
        let ibe = self.diode_current_with_is(self.ibei, vp_be, self.nei)
            + self.diode_current_with_is(self.iben, vp_be, self.nen);
        let ibc = self.diode_current_with_is(self.ibci, vp_bc, self.nci)
            + self.diode_current_with_is(self.ibcn, vp_bc, self.ncn);
        (self.ic.abs(), ibe.abs(), ibc.abs())
    }

    /// Return flicker-noise coefficients, if enabled by the model card.
    pub fn flicker_noise_coefficients(&self) -> Option<(Value, Value, Value)> {
        if self.kf > 0.0 && self.kf.is_finite() {
            Some((self.kf, self.af.max(1e-12), self.ef.max(1e-12)))
        } else {
            None
        }
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let c = self.node_collector;
        let b = self.node_base;
        let e = self.node_emitter;

        // Collector row
        if c > 0 {
            self.indices.cc = matrix.get_index(c - 1, c - 1);
        }
        if c > 0 && b > 0 {
            self.indices.cb = matrix.get_index(c - 1, b - 1);
        }
        if c > 0 && e > 0 {
            self.indices.ce = matrix.get_index(c - 1, e - 1);
        }
        // Base row
        if b > 0 && c > 0 {
            self.indices.bc = matrix.get_index(b - 1, c - 1);
        }
        if b > 0 {
            self.indices.bb = matrix.get_index(b - 1, b - 1);
        }
        if b > 0 && e > 0 {
            self.indices.be = matrix.get_index(b - 1, e - 1);
        }
        // Emitter row
        if e > 0 && c > 0 {
            self.indices.ec = matrix.get_index(e - 1, c - 1);
        }
        if e > 0 && b > 0 {
            self.indices.eb = matrix.get_index(e - 1, b - 1);
        }
        if e > 0 {
            self.indices.ee = matrix.get_index(e - 1, e - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let rows = self.small_signal_row_coefficients(vc, vb, ve, vs);
        let nodes = self.external_terminal_nodes();
        let biases = [vc, vb, ve, vs];
        let currents = [self.ic, self.ib, self.ie, self.isub];

        let stamp_entry =
            |matrix: &mut StaticMatrix, row_idx: usize, col_idx: usize, value: Value| {
                let row = nodes[row_idx];
                let col = nodes[col_idx];
                if row == 0 || col == 0 {
                    return;
                }

                match (row_idx, col_idx) {
                    (EXT_C, EXT_C) => {
                        if let Some(idx) = self.indices.cc {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_C, EXT_B) => {
                        if let Some(idx) = self.indices.cb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_C, EXT_E) => {
                        if let Some(idx) = self.indices.ce {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_C) => {
                        if let Some(idx) = self.indices.bc {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_B) => {
                        if let Some(idx) = self.indices.bb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_B, EXT_E) => {
                        if let Some(idx) = self.indices.be {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_C) => {
                        if let Some(idx) = self.indices.ec {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_B) => {
                        if let Some(idx) = self.indices.eb {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    (EXT_E, EXT_E) => {
                        if let Some(idx) = self.indices.ee {
                            matrix.stamp_direct(idx, value);
                        } else {
                            matrix.add(row - 1, col - 1, value);
                        }
                    }
                    _ => matrix.add(row - 1, col - 1, value),
                }
            };

        for row_idx in 0..EXTERNAL_DIM {
            let ieq = currents[row_idx]
                - (0..EXTERNAL_DIM)
                    .map(|col_idx| rows[row_idx][col_idx] * biases[col_idx])
                    .sum::<Value>();
            for col_idx in 0..EXTERNAL_DIM {
                stamp_entry(matrix, row_idx, col_idx, rows[row_idx][col_idx]);
            }
            if nodes[row_idx] > 0 {
                rhs[nodes[row_idx] - 1] -= ieq;
            }
        }
    }

    /// Get polarity multiplier (+1 for NPN, -1 for PNP)
    fn polarity(&self) -> Value {
        match self.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        }
    }

    /// Diode current: I = Is * (exp(V / (n * Vt)) - 1)
    ///
    /// SPICE-style voltage limiting:
    /// - Forward: limit to 80*n*Vt to prevent exp overflow
    /// - Reverse: for V < -5*n*Vt, use linear extrapolation (negligible current)
    fn diode_current_with_is(&self, isat: Value, v: Value, n: Value) -> Value {
        let nvt = n * self.vt;
        let v_crit = 80.0 * nvt; // Forward limit
        let v_rev = -5.0 * nvt; // Reverse limit (around -0.13V at room temp)

        if v > v_crit {
            // Forward saturation - linear extrapolation
            let i_crit = isat * ((v_crit / nvt).exp() - 1.0);
            let g_crit = (isat / nvt) * (v_crit / nvt).exp();
            i_crit + g_crit * (v - v_crit)
        } else if v < v_rev {
            // Deep reverse bias - essentially just -Is (negligible)
            -isat
        } else {
            // Normal operating region
            isat * ((v / nvt).exp() - 1.0)
        }
    }

    #[inline]
    fn diode_current(&self, v: Value, n: Value) -> Value {
        self.diode_current_with_is(self.is, v, n)
    }

    /// Diode conductance: g = Is / (n * Vt) * exp(V / (n * Vt))
    ///
    /// SPICE-style limiting with minimum conductance floor for numerical stability
    fn diode_conductance_with_is(&self, isat: Value, v: Value, n: Value) -> Value {
        let nvt = n * self.vt;
        let v_crit = 80.0 * nvt;
        let v_rev = -5.0 * nvt;

        let g = if v > v_crit {
            // Forward saturation - constant high conductance
            (isat / nvt) * (v_crit / nvt).exp()
        } else if v < v_rev {
            // Deep reverse bias - minimum conductance
            1e-15
        } else {
            // Normal region
            (isat / nvt) * (v / nvt).exp()
        };

        // Apply minimum conductance floor
        g.max(1e-15)
    }

    #[inline]
    fn diode_conductance(&self, v: Value, n: Value) -> Value {
        self.diode_conductance_with_is(self.is, v, n)
    }

    #[inline]
    fn depletion_charge_base(potential: Value, grading: Value, scaled_voltage: Value) -> Value {
        let phi = potential.max(1e-12);
        let exponent = 1.0 - grading;
        let one_minus = (1.0 - scaled_voltage / phi).max(1e-18);
        if exponent.abs() < 1e-12 {
            -phi * one_minus.ln()
        } else {
            phi * (1.0 - one_minus.powf(exponent)) / exponent
        }
    }

    fn depletion_capacitance_factor(
        potential: Value,
        grading: Value,
        scaled_voltage: Value,
    ) -> Value {
        let phi = potential.max(1e-12);
        let one_minus = (1.0 - scaled_voltage / phi).max(1e-18);
        if (1.0 - grading).abs() < 1e-12 {
            1.0 / one_minus
        } else {
            one_minus.powf(-grading)
        }
    }

    fn vbic_depletion_charge_and_derivative(
        &self,
        junction_voltage_eff: Value,
        potential: Value,
        grading: Value,
        forward_coeff: Value,
        smoothing: Value,
    ) -> (Value, Value) {
        let phi = potential.max(1e-12);
        let fc = forward_coeff.clamp(0.0, 0.999_999);

        if smoothing > 0.0 {
            let dv0 = -phi * fc;
            let mv0 = (dv0 * dv0 + 4.0 * smoothing * smoothing).sqrt();
            let vl0 = -0.5 * (dv0 + mv0);
            let q0 = -Self::depletion_charge_base(phi, grading, vl0);

            let dv = junction_voltage_eff + dv0;
            let mv = (dv * dv + 4.0 * smoothing * smoothing).sqrt();
            let dmv_dv = dv / mv.max(1e-18);
            let vl = 0.5 * (dv - mv) - dv0;
            let dvl_dv = 0.5 * (1.0 - dmv_dv);

            let qlo = -Self::depletion_charge_base(phi, grading, vl);
            let dqlo_dvl = Self::depletion_capacitance_factor(phi, grading, vl);
            let linear_gain = (1.0 - fc).max(1e-18).powf(-grading);
            let charge = qlo + linear_gain * (junction_voltage_eff - vl + vl0) - q0;
            let derivative = dqlo_dvl * dvl_dv + linear_gain * (1.0 - dvl_dv);
            return (charge, derivative.max(0.0));
        }

        let dv0 = -phi * fc;
        let dvh = junction_voltage_eff + dv0;
        if dvh > 0.0 {
            let one_minus_fc = (1.0 - fc).max(1e-18);
            let pwq = one_minus_fc.powf(-1.0 - grading);
            let qlo = Self::depletion_charge_base(phi, grading, phi * fc);
            let charge = qlo + dvh * (one_minus_fc + 0.5 * grading * dvh / phi) * pwq;
            let derivative = pwq * (one_minus_fc + grading * dvh / phi);
            return (charge, derivative.max(0.0));
        }

        let charge = Self::depletion_charge_base(phi, grading, junction_voltage_eff);
        let derivative = Self::depletion_capacitance_factor(phi, grading, junction_voltage_eff);
        (charge, derivative.max(0.0))
    }

    fn legacy_transport_charge_state(
        &self,
        vbe_eff: Value,
        vbc_eff: Value,
    ) -> TransportChargeState {
        let ifi = self.diode_current(vbe_eff, self.nf).max(0.0);
        let iri = self
            .diode_current_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr)
            .max(0.0);
        let gfi = self.diode_conductance(vbe_eff, self.nf);
        let gri = self.diode_conductance_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr);

        let raw_q1_inv =
            1.0 - if self.var.is_finite() && self.var > 0.0 {
                vbe_eff / self.var
            } else {
                0.0
            } - if self.vaf.is_finite() && self.vaf > 0.0 {
                vbc_eff / self.vaf
            } else {
                0.0
            };
        let (q1_inv, dq1_inv_draw_q1_inv) = Self::smooth_positive_floor(raw_q1_inv, 1e-9);
        let q1 = 1.0 / q1_inv.max(1e-18);
        let dq1_dvbe_eff = if self.var.is_finite() && self.var > 0.0 {
            dq1_inv_draw_q1_inv / (self.var * q1_inv * q1_inv)
        } else {
            0.0
        };
        let dq1_dvbc_eff = if self.vaf.is_finite() && self.vaf > 0.0 {
            dq1_inv_draw_q1_inv / (self.vaf * q1_inv * q1_inv)
        } else {
            0.0
        };

        let inv_rolloff_f = if self.ikf > 0.0 { 1.0 / self.ikf } else { 0.0 };
        let inv_rolloff_r = if self.ikr > 0.0 { 1.0 / self.ikr } else { 0.0 };
        let (qb, dqb_dvbe_eff, dqb_dvbc_eff) = if inv_rolloff_f == 0.0 && inv_rolloff_r == 0.0 {
            (q1.max(1e-12), dq1_dvbe_eff, dq1_dvbc_eff)
        } else {
            let q2 = inv_rolloff_f * ifi + inv_rolloff_r * iri;
            let dq2_dvbe_eff = inv_rolloff_f * gfi;
            let dq2_dvbc_eff = inv_rolloff_r * gri;
            let sqrt_term = (1.0 + 4.0 * q2).sqrt().max(1e-18);
            (
                (0.5 * q1 * (1.0 + sqrt_term)).max(1e-12),
                0.5 * (1.0 + sqrt_term) * dq1_dvbe_eff + q1 * dq2_dvbe_eff / sqrt_term,
                0.5 * (1.0 + sqrt_term) * dq1_dvbc_eff + q1 * dq2_dvbc_eff / sqrt_term,
            )
        };

        let itzf = ifi / qb;
        let ditzf_dvbe_eff = gfi / qb - ifi * dqb_dvbe_eff / (qb * qb);
        let ditzf_dvbc_eff = -ifi * dqb_dvbc_eff / (qb * qb);
        let itzr = iri / qb;
        let ditzr_dvbe_eff = -iri * dqb_dvbe_eff / (qb * qb);
        let ditzr_dvbc_eff = gri / qb - iri * dqb_dvbc_eff / (qb * qb);

        TransportChargeState {
            q1,
            qb,
            ifi,
            iri,
            gfi,
            gri,
            dq1_dvbe_eff,
            dq1_dvbc_eff,
            itzf,
            itzr,
            dqb_dvbe_eff,
            dqb_dvbc_eff,
            ditzf_dvbe_eff,
            ditzf_dvbc_eff,
            ditzr_dvbe_eff,
            ditzr_dvbc_eff,
        }
    }

    fn vbic_transport_charge_state(&self, vbe_eff: Value, vbc_eff: Value) -> TransportChargeState {
        let ifi = self.diode_current(vbe_eff, self.nf).max(0.0);
        let iri = self
            .diode_current_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr)
            .max(0.0);
        let gfi = self.diode_conductance(vbe_eff, self.nf);
        let gri = self.diode_conductance_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr);

        let (qdbe, dqdbe_dvbe_eff) = self
            .vbic_depletion_charge_and_derivative(vbe_eff, self.vje, self.mje, self.fc, self.aje);
        let (qdbc, dqdbc_dvbc_eff) = self
            .vbic_depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc, self.fc, self.ajc);

        let q1z =
            1.0 + if self.var.is_finite() && self.var > 0.0 {
                qdbe / self.var
            } else {
                0.0
            } + if self.vaf.is_finite() && self.vaf > 0.0 {
                qdbc / self.vaf
            } else {
                0.0
            };
        let q1_shift = q1z - 1e-4;
        let q1_sqrt = (q1_shift * q1_shift + 1e-8).sqrt();
        let q1 = 0.5 * (q1_sqrt + q1_shift) + 1e-4;
        let dq1_dq1z = 0.5 * (q1_shift / q1_sqrt + 1.0);
        let dq1_dvbe_eff = dq1_dq1z
            * if self.var.is_finite() && self.var > 0.0 {
                dqdbe_dvbe_eff / self.var
            } else {
                0.0
            };
        let dq1_dvbc_eff = dq1_dq1z
            * if self.vaf.is_finite() && self.vaf > 0.0 {
                dqdbc_dvbc_eff / self.vaf
            } else {
                0.0
            };

        let inv_rolloff_f = if self.ikf > 0.0 { 1.0 / self.ikf } else { 0.0 };
        let inv_rolloff_r = if self.ikr > 0.0 { 1.0 / self.ikr } else { 0.0 };
        let q2 = inv_rolloff_f * ifi + inv_rolloff_r * iri;
        let dq2_dvbe_eff = inv_rolloff_f * gfi;
        let dq2_dvbc_eff = inv_rolloff_r * gri;
        let nkf = self.nkf.max(1e-12);
        let (qb, dqb_dvbe_eff, dqb_dvbc_eff) = if self.qbm < 0.5 {
            let inv_nkf = 1.0 / nkf;
            let xvar3 = q1.max(1e-18).powf(inv_nkf);
            let dxvar3_dvbe_eff = if q1 > 0.0 {
                xvar3 * inv_nkf * dq1_dvbe_eff / q1.max(1e-18)
            } else {
                0.0
            };
            let dxvar3_dvbc_eff = if q1 > 0.0 {
                xvar3 * inv_nkf * dq1_dvbc_eff / q1.max(1e-18)
            } else {
                0.0
            };
            let xvar1 = (xvar3 + 4.0 * q2).max(1e-18);
            let dxvar1_dvbe_eff = dxvar3_dvbe_eff + 4.0 * dq2_dvbe_eff;
            let dxvar1_dvbc_eff = dxvar3_dvbc_eff + 4.0 * dq2_dvbc_eff;
            let xvar4 = xvar1.powf(nkf);
            let dxvar4_dvbe_eff = xvar4 * nkf * dxvar1_dvbe_eff / xvar1;
            let dxvar4_dvbc_eff = xvar4 * nkf * dxvar1_dvbc_eff / xvar1;
            (
                (0.5 * (q1 + xvar4)).max(1e-12),
                0.5 * (dq1_dvbe_eff + dxvar4_dvbe_eff),
                0.5 * (dq1_dvbc_eff + dxvar4_dvbc_eff),
            )
        } else {
            let xvar1 = (1.0 + 4.0 * q2).max(1e-18);
            let dxvar1_dvbe_eff = 4.0 * dq2_dvbe_eff;
            let dxvar1_dvbc_eff = 4.0 * dq2_dvbc_eff;
            let xvar2 = xvar1.powf(nkf);
            let dxvar2_dvbe_eff = xvar2 * nkf * dxvar1_dvbe_eff / xvar1;
            let dxvar2_dvbc_eff = xvar2 * nkf * dxvar1_dvbc_eff / xvar1;
            (
                (0.5 * q1 * (1.0 + xvar2)).max(1e-12),
                0.5 * (1.0 + xvar2) * dq1_dvbe_eff + 0.5 * q1 * dxvar2_dvbe_eff,
                0.5 * (1.0 + xvar2) * dq1_dvbc_eff + 0.5 * q1 * dxvar2_dvbc_eff,
            )
        };

        let itzf = ifi / qb;
        let ditzf_dvbe_eff = gfi / qb - ifi * dqb_dvbe_eff / (qb * qb);
        let ditzf_dvbc_eff = -ifi * dqb_dvbc_eff / (qb * qb);

        let itzr = iri / qb;
        let ditzr_dvbe_eff = -iri * dqb_dvbe_eff / (qb * qb);
        let ditzr_dvbc_eff = gri / qb - iri * dqb_dvbc_eff / (qb * qb);

        TransportChargeState {
            q1,
            qb,
            ifi,
            iri,
            gfi,
            gri,
            dq1_dvbe_eff,
            dq1_dvbc_eff,
            itzf,
            itzr,
            dqb_dvbe_eff,
            dqb_dvbc_eff,
            ditzf_dvbe_eff,
            ditzf_dvbc_eff,
            ditzr_dvbe_eff,
            ditzr_dvbc_eff,
        }
    }

    fn transport_charge_state(&self, vbe_eff: Value, vbc_eff: Value) -> TransportChargeState {
        match self.charge_model {
            BjtChargeModel::LegacyGummelPoon => {
                self.legacy_transport_charge_state(vbe_eff, vbc_eff)
            }
            BjtChargeModel::Vbic => self.vbic_transport_charge_state(vbe_eff, vbc_eff),
        }
    }

    fn base_collector_current_state(
        &self,
        transport: TransportChargeState,
        vbc_eff: Value,
    ) -> BaseCollectorCurrentState {
        let ibcj = self.diode_current_with_is(self.ibci, vbc_eff, self.nci)
            + self.diode_current_with_is(self.ibcn, vbc_eff, self.ncn);
        let dibcj_dvbc_eff = self.diode_conductance_with_is(self.ibci, vbc_eff, self.nci)
            + self.diode_conductance_with_is(self.ibcn, vbc_eff, self.ncn);

        if self.avc1 <= 0.0 {
            return BaseCollectorCurrentState {
                ibc: ibcj,
                dibc_dvbe_eff: 0.0,
                dibc_dvbc_eff: dibcj_dvbc_eff,
            };
        }

        let vl_arg = self.vjc - vbc_eff;
        let vl_sqrt = (vl_arg * vl_arg + 0.01).sqrt().max(1e-18);
        let vl = 0.5 * (vl_sqrt + vl_arg);
        let dvl_dvbc_eff = 0.5 * (-vl_arg / vl_sqrt - 1.0);

        let power = self.mjc - 1.0;
        let vl_safe = vl.max(1e-18);
        let vl_power = vl_safe.powf(power);
        let d_vl_power_dvbc_eff = power * vl_safe.powf(power - 1.0) * dvl_dvbc_eff;

        let avalanche_arg = -self.avc2.max(0.0) * vl_power;
        let (avalanche_exp, d_avalanche_exp_darg) = Self::limited_exp(avalanche_arg);
        let d_avalanche_arg_dvbc_eff = -self.avc2.max(0.0) * d_vl_power_dvbc_eff;
        let avalf = self.avc1 * vl * avalanche_exp;
        let davalf_dvbc_eff = self.avc1
            * (dvl_dvbc_eff * avalanche_exp + vl * d_avalanche_exp_darg * d_avalanche_arg_dvbc_eff);

        let transport_minus_ibcj = transport.itzf - transport.itzr - ibcj;
        let d_transport_minus_ibcj_dvbe_eff = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let d_transport_minus_ibcj_dvbc_eff =
            transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff - dibcj_dvbc_eff;

        let igc = transport_minus_ibcj * avalf;
        let digc_dvbe_eff = d_transport_minus_ibcj_dvbe_eff * avalf;
        let digc_dvbc_eff =
            d_transport_minus_ibcj_dvbc_eff * avalf + transport_minus_ibcj * davalf_dvbc_eff;

        BaseCollectorCurrentState {
            ibc: ibcj - igc,
            dibc_dvbe_eff: -digc_dvbe_eff,
            dibc_dvbc_eff: dibcj_dvbc_eff - digc_dvbc_eff,
        }
    }

    fn linearize_currents_with_branches(
        &self,
        vbe: Value,
        vbc: Value,
    ) -> (BjtLinearization, BjtIntrinsicBranches) {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;
        let transport = self.transport_charge_state(vbe_eff, vbc_eff);
        let bc = self.base_collector_current_state(transport, vbc_eff);

        let ib_be = self.diode_current_with_is(self.ibei, vbe_eff, self.nei)
            + self.diode_current_with_is(self.iben, vbe_eff, self.nen);
        let dibe_dvbe = self.gbe(vbe);
        let iciei = transport.itzf - transport.itzr;
        let diciei_dvbe = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let diciei_dvbc = transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff;
        let ibe_branch = Self::branch_from_vbe_vbc(p * ib_be, dibe_dvbe, 0.0);
        let ibc_branch = Self::branch_from_vbe_vbc(p * bc.ibc, bc.dibc_dvbe_eff, bc.dibc_dvbc_eff);
        let iciei_branch = Self::branch_from_vbe_vbc(p * iciei, diciei_dvbe, diciei_dvbc);
        let linearized = BjtLinearization {
            // The intrinsic collector terminal sees both the transport branch
            // (collector to emitter) and the opposing B-C junction branch.
            ic: p * (iciei - bc.ibc),
            ib: p * (ib_be + bc.ibc),
            dic_dvbe: diciei_dvbe - bc.dibc_dvbe_eff,
            dic_dvbc: diciei_dvbc - bc.dibc_dvbc_eff,
            dic_dvrth: 0.0,
            dib_dvbe: dibe_dvbe + bc.dibc_dvbe_eff,
            dib_dvbc: bc.dibc_dvbc_eff,
            dib_dvrth: 0.0,
            qb: transport.qb,
            dqb_dvbe: p * transport.dqb_dvbe_eff,
            dqb_dvbc: p * transport.dqb_dvbc_eff,
            dqb_dvrth: 0.0,
        };

        (
            linearized,
            BjtIntrinsicBranches {
                ibe: ibe_branch,
                ibc: ibc_branch,
                iciei: iciei_branch,
            },
        )
    }

    fn linearize_currents(&self, vbe: Value, vbc: Value) -> BjtLinearization {
        self.linearize_currents_with_branches(vbe, vbc).0
    }

    #[inline]
    fn collector_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        [
            -linearized.dic_dvbc,
            linearized.dic_dvbe + linearized.dic_dvbc,
            -linearized.dic_dvbe,
            0.0,
        ]
    }

    #[inline]
    fn base_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        [
            -linearized.dib_dvbc,
            linearized.dib_dvbe + linearized.dib_dvbc,
            -linearized.dib_dvbe,
            0.0,
        ]
    }

    #[inline]
    fn emitter_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        let collector = self.collector_row_coefficients(linearized);
        let base = self.base_row_coefficients(linearized);
        let mut emitter = [0.0; EXTERNAL_DIM];
        for idx in 0..EXTERNAL_DIM {
            emitter[idx] = -(collector[idx] + base[idx]);
        }
        emitter
    }

    #[inline]
    fn series_active(resistance: Value) -> bool {
        resistance.is_finite() && resistance > 0.0
    }

    #[inline]
    fn limited_exp(arg: Value) -> (Value, Value) {
        let clamped = arg.clamp(-80.0, 80.0);
        let value = clamped.exp();
        let slope = if (arg - clamped).abs() < f64::EPSILON {
            value
        } else {
            0.0
        };
        (value, slope)
    }

    fn intrinsic_terminal_derivatives(
        &self,
        linearized: BjtLinearization,
    ) -> (
        [Value; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
    ) {
        let mut collector = [0.0; INTERNAL_DIM];
        collector[IDX_VCI] = -linearized.dic_dvbc;
        collector[IDX_VBI] = linearized.dic_dvbe + linearized.dic_dvbc;
        collector[IDX_VEI] = -linearized.dic_dvbe;
        collector[IDX_VRTH] = linearized.dic_dvrth;

        let mut base = [0.0; INTERNAL_DIM];
        base[IDX_VCI] = -linearized.dib_dvbc;
        base[IDX_VBI] = linearized.dib_dvbe + linearized.dib_dvbc;
        base[IDX_VEI] = -linearized.dib_dvbe;
        base[IDX_VRTH] = linearized.dib_dvrth;

        let mut emitter = [0.0; INTERNAL_DIM];
        for idx in 0..INTERNAL_DIM {
            emitter[idx] = -(collector[idx] + base[idx]);
        }

        (collector, base, emitter)
    }

    fn ircx_branch(&self, vc: Value, vcx: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rcx) {
            return branch;
        }

        let g = 1.0 / self.rcx.max(1e-12);
        branch.current = g * (vc - vcx);
        branch.d_internal[IDX_VCX] = -g;
        branch.d_external[0] = g;
        branch
    }

    fn irbx_branch(&self, vb: Value, vbx: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbx) {
            return branch;
        }

        let g = 1.0 / self.rbx.max(1e-12);
        branch.current = g * (vb - vbx);
        branch.d_internal[IDX_VBX] = -g;
        branch.d_external[1] = g;
        branch
    }

    fn ire_branch(&self, ve: Value, vei: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.re) {
            return branch;
        }

        let g = 1.0 / self.re.max(1e-12);
        branch.current = g * (ve - vei);
        branch.d_internal[IDX_VEI] = -g;
        branch.d_external[2] = g;
        branch
    }

    fn irbi_branch(
        &self,
        linearized: BjtLinearization,
        vbx: Value,
        vbi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbi) {
            return branch;
        }

        let rb = self.rbi.max(1e-12);
        let vrbi = vbx - vbi;
        let qb = linearized.qb.max(1e-12);
        let scale = vrbi / rb;
        let dqb_dvbi = linearized.dqb_dvbe + linearized.dqb_dvbc;
        let dqb_dvci = -linearized.dqb_dvbc;
        let dqb_dvei = -linearized.dqb_dvbe;

        branch.current = scale * qb;
        branch.d_internal[IDX_VBX] = qb / rb;
        branch.d_internal[IDX_VBI] = -qb / rb + scale * dqb_dvbi;
        branch.d_internal[IDX_VCI] = scale * dqb_dvci;
        branch.d_internal[IDX_VEI] = scale * dqb_dvei;
        branch
    }

    fn ibep_branch(&self, vbx: Value, vbp: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if self.ibeip <= 0.0 && self.ibenp <= 0.0 {
            return branch;
        }

        let p = self.polarity();
        let vbep_eff = p * (vbx - vbp);
        let ibeip = self.diode_current_with_is(self.ibeip, vbep_eff, self.nci.max(1e-12));
        let ibenp = self.diode_current_with_is(self.ibenp, vbep_eff, self.ncn.max(1e-12));
        let gbep = self.diode_conductance_with_is(self.ibeip, vbep_eff, self.nci.max(1e-12))
            + self.diode_conductance_with_is(self.ibenp, vbep_eff, self.ncn.max(1e-12));

        branch.current = p * (ibeip + ibenp);
        branch.d_internal[IDX_VBX] = gbep;
        branch.d_internal[IDX_VBP] = -gbep;
        branch
    }

    fn parasitic_transport_state(
        &self,
        vbx: Value,
        vbi: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> ParasiticTransportState {
        let mut state = ParasiticTransportState {
            qbp: 1.0,
            d_qbp: [0.0; INTERNAL_DIM],
            ifp: 0.0,
            d_ifp: [0.0; INTERNAL_DIM],
            irp: 0.0,
            d_irp: [0.0; INTERNAL_DIM],
        };

        if self.isp <= 0.0 {
            return state;
        }

        let p = self.polarity();
        let nfp_vt = (self.nfp.max(1e-12) * self.vt.max(1e-12)).max(1e-18);
        let vbep_eff = p * (vbx - vbp);
        let vbci_eff = p * (vbi - vci);
        let vbcp_eff = p * (vsi - vbp);

        let (exp_bep, dexp_bep_darg) = Self::limited_exp(vbep_eff / nfp_vt);
        let (exp_bci, dexp_bci_darg) = Self::limited_exp(vbci_eff / nfp_vt);
        let d_ifp_d_vbep_eff = self.isp * self.wsp * dexp_bep_darg / nfp_vt;
        let d_ifp_d_vbci_eff = self.isp * (1.0 - self.wsp) * dexp_bci_darg / nfp_vt;
        state.ifp = self.isp * (self.wsp * exp_bep + (1.0 - self.wsp) * exp_bci - 1.0);
        state.d_ifp[IDX_VBX] = d_ifp_d_vbep_eff * p;
        state.d_ifp[IDX_VBP] = -d_ifp_d_vbep_eff * p;
        state.d_ifp[IDX_VBI] = d_ifp_d_vbci_eff * p;
        state.d_ifp[IDX_VCI] = -d_ifp_d_vbci_eff * p;

        let iikp = if self.ikp.is_finite() && self.ikp > 0.0 {
            1.0 / self.ikp
        } else {
            0.0
        };
        let sqrt_term = (1.0 + 4.0 * state.ifp * iikp).max(1e-18).sqrt();
        state.qbp = (0.5 * (1.0 + sqrt_term)).max(1e-12);
        if iikp > 0.0 {
            let d_qbp_d_ifp = iikp / sqrt_term;
            for idx in 0..INTERNAL_DIM {
                state.d_qbp[idx] = d_qbp_d_ifp * state.d_ifp[idx];
            }
        }

        let (exp_bcp, dexp_bcp_darg) = Self::limited_exp(vbcp_eff / nfp_vt);
        let d_irp_d_vbcp_eff = self.isp * dexp_bcp_darg / nfp_vt;
        state.irp = self.isp * (exp_bcp - 1.0);
        state.d_irp[IDX_VSI] = d_irp_d_vbcp_eff * p;
        state.d_irp[IDX_VBP] = -d_irp_d_vbcp_eff * p;

        state
    }

    fn irbp_branch(
        &self,
        vbx: Value,
        vbi: Value,
        vcx: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbp) {
            return branch;
        }

        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let rbp = self.rbp.max(1e-12);
        let vrbp = vbp - vcx;
        let scale = vrbp / rbp;

        branch.current = scale * parasitic.qbp;
        branch.d_internal[IDX_VCX] = -parasitic.qbp / rbp;
        branch.d_internal[IDX_VBP] = parasitic.qbp / rbp + scale * parasitic.d_qbp[IDX_VBP];
        branch.d_internal[IDX_VBX] = scale * parasitic.d_qbp[IDX_VBX];
        branch.d_internal[IDX_VBI] = scale * parasitic.d_qbp[IDX_VBI];
        branch.d_internal[IDX_VCI] = scale * parasitic.d_qbp[IDX_VCI];
        branch.d_internal[IDX_VSI] = scale * parasitic.d_qbp[IDX_VSI];
        branch
    }

    fn ibcp_branch(&self, vbp: Value, vsi: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if self.ibcip <= 0.0 && self.ibcnp <= 0.0 {
            return branch;
        }

        let p = self.polarity();
        let vbcp_eff = p * (vsi - vbp);
        let ibcip = self.diode_current_with_is(self.ibcip, vbcp_eff, self.ncip.max(1e-12));
        let ibcnp = self.diode_current_with_is(self.ibcnp, vbcp_eff, self.ncnp.max(1e-12));
        let gbcp = self.diode_conductance_with_is(self.ibcip, vbcp_eff, self.ncip.max(1e-12))
            + self.diode_conductance_with_is(self.ibcnp, vbcp_eff, self.ncnp.max(1e-12));

        branch.current = p * (ibcip + ibcnp);
        branch.d_internal[IDX_VSI] = gbcp;
        branch.d_internal[IDX_VBP] = -gbcp;
        branch
    }

    fn iccp_branch(
        &self,
        vbx: Value,
        vbi: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if self.isp <= 0.0 {
            return branch;
        }

        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let p = self.polarity();
        let inv_qbp = 1.0 / parasitic.qbp.max(1e-12);
        let delta = parasitic.ifp - parasitic.irp;

        branch.current = p * delta * inv_qbp;
        for idx in 0..INTERNAL_DIM {
            branch.d_internal[idx] = p
                * ((parasitic.d_ifp[idx] - parasitic.d_irp[idx]) * inv_qbp
                    - delta * parasitic.d_qbp[idx] * inv_qbp * inv_qbp);
        }
        branch
    }

    fn irs_branch(&self, vs: Value, vsi: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rs) {
            return branch;
        }

        let g = 1.0 / self.rs.max(1e-12);
        branch.current = g * (vs - vsi);
        branch.d_internal[IDX_VSI] = -g;
        branch.d_external[EXT_S] = g;
        branch
    }

    fn irci_branch(&self, vcx: Value, vci: Value, vbi: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rci) {
            return branch;
        }

        let p = self.polarity();
        let vt = self.vt.max(1e-12);
        let rci = self.rci.max(1e-12);
        let gamm = self.gamm.max(0.0);
        let ivo = if self.vo.is_finite() && self.vo > 0.0 {
            1.0 / self.vo
        } else {
            0.0
        };
        let ihrcf = if self.hrcf.is_finite() && self.hrcf > 0.0 {
            1.0 / self.hrcf
        } else {
            0.0
        };

        let vrci_eff = p * (vcx - vci);
        let vbci_eff = p * (vbi - vci);
        let vbcx_eff = p * (vbi - vcx);

        let (exp_bci, dexp_bci_darg) = Self::limited_exp(vbci_eff / vt);
        let (exp_bcx, dexp_bcx_darg) = Self::limited_exp(vbcx_eff / vt);
        let d_exp_bci_dvbci_eff = dexp_bci_darg / vt;
        let d_exp_bcx_dvbcx_eff = dexp_bcx_darg / vt;

        let kbci = (1.0 + gamm * exp_bci).sqrt().max(1e-12);
        let kbcx = (1.0 + gamm * exp_bcx).sqrt().max(1e-12);
        let d_kbci_dvbci_eff = if gamm > 0.0 {
            gamm * d_exp_bci_dvbci_eff / (2.0 * kbci)
        } else {
            0.0
        };
        let d_kbcx_dvbcx_eff = if gamm > 0.0 {
            gamm * d_exp_bcx_dvbcx_eff / (2.0 * kbcx)
        } else {
            0.0
        };

        let ratio = ((kbci + 1.0) / (kbcx + 1.0)).max(1e-18);
        let log_ratio = ratio.ln();
        let d_ratio_dkbci = 1.0 / (kbcx + 1.0);
        let d_ratio_dkbcx = -(kbci + 1.0) / (kbcx + 1.0).powi(2);
        let d_log_ratio_dkbci = d_ratio_dkbci / ratio;
        let d_log_ratio_dkbcx = d_ratio_dkbcx / ratio;

        let iohm = (vrci_eff + vt * (kbci - kbcx - log_ratio)) / rci;
        let d_iohm_dvrci_eff = 1.0 / rci;
        let d_iohm_dvbci_eff = vt * d_kbci_dvbci_eff * (1.0 - d_log_ratio_dkbci) / rci;
        let d_iohm_dvbcx_eff = vt * d_kbcx_dvbcx_eff * (-1.0 - d_log_ratio_dkbcx) / rci;

        let sqrt_vrci = (vrci_eff * vrci_eff + 0.01).sqrt();
        let denom = 1.0 + 0.5 * ivo * ihrcf * sqrt_vrci;
        let d_denom_dvrci_eff = if ivo > 0.0 && ihrcf > 0.0 {
            0.5 * ivo * ihrcf * vrci_eff / sqrt_vrci
        } else {
            0.0
        };

        let derf_scale = ivo * rci;
        let derf = if derf_scale > 0.0 {
            derf_scale * iohm / denom
        } else {
            0.0
        };
        let d_derf_dvrci_eff = if derf_scale > 0.0 {
            derf_scale * (d_iohm_dvrci_eff / denom - iohm * d_denom_dvrci_eff / denom.powi(2))
        } else {
            0.0
        };
        let d_derf_dvbci_eff = if derf_scale > 0.0 {
            derf_scale * d_iohm_dvbci_eff / denom
        } else {
            0.0
        };
        let d_derf_dvbcx_eff = if derf_scale > 0.0 {
            derf_scale * d_iohm_dvbcx_eff / denom
        } else {
            0.0
        };

        let irci_scale = (1.0 + derf * derf).sqrt();
        let inv_irci_scale = 1.0 / irci_scale;
        let common = -iohm * derf / (irci_scale * irci_scale * irci_scale);
        let d_irci_eff_dvrci_eff = d_iohm_dvrci_eff * inv_irci_scale + common * d_derf_dvrci_eff;
        let d_irci_eff_dvbci_eff = d_iohm_dvbci_eff * inv_irci_scale + common * d_derf_dvbci_eff;
        let d_irci_eff_dvbcx_eff = d_iohm_dvbcx_eff * inv_irci_scale + common * d_derf_dvbcx_eff;
        let irci_eff = iohm * inv_irci_scale;

        branch.current = p * irci_eff;
        branch.d_internal[IDX_VCX] = d_irci_eff_dvrci_eff - d_irci_eff_dvbcx_eff;
        branch.d_internal[IDX_VCI] = -(d_irci_eff_dvrci_eff + d_irci_eff_dvbci_eff);
        branch.d_internal[IDX_VBI] = d_irci_eff_dvbci_eff + d_irci_eff_dvbcx_eff;
        branch
    }

    #[inline]
    fn thermal_derivative_step(&self, vrth: Value) -> Value {
        // Use a small relative perturbation to keep Vrth-derivative finite
        // differences accurate for strongly temperature-sensitive currents.
        ((self.requested_temperature() + vrth).abs().max(1.0) * 1e-6).clamp(1e-7, 1e-3)
    }

    #[inline]
    fn limit_logarithmic_step(vnew: Value, vold: Value, limit: Value) -> Value {
        let limit = limit.max(1e-18);
        if !vnew.is_finite() {
            return vold;
        }
        if !vold.is_finite() {
            return vnew;
        }

        if vnew > vold + limit {
            vold + limit + ((vnew - vold) / limit).log10()
        } else if vnew < vold - limit {
            vold - limit - ((vold - vnew) / limit).log10()
        } else {
            vnew
        }
    }

    #[inline]
    fn junction_critical_voltage(vt: Value, isat: Value) -> Value {
        let vt = vt.max(1e-18);
        let isat = isat.abs().max(1e-18);
        vt * (vt / (core::f64::consts::SQRT_2 * isat)).ln()
    }

    #[inline]
    fn vbic_limiting_parameters(&self, previous_vrth: Value) -> (Value, Value) {
        self.with_temperature_variant(previous_vrth, |model| {
            let vt = model.vt.max(1e-18);
            let vcrit = Self::junction_critical_voltage(vt, model.is);
            (vt, vcrit)
        })
    }

    #[inline]
    fn vbic_nonlinear_branch_voltages(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> VbicNonlinearBranchVoltages {
        let p = self.polarity();
        VbicNonlinearBranchVoltages {
            vbei: p * (internal[IDX_VBI] - internal[IDX_VEI]),
            vbex: p * (internal[IDX_VBX] - internal[IDX_VEI]),
            vbci: p * (internal[IDX_VBI] - internal[IDX_VCI]),
            vbcx: p * (internal[IDX_VBI] - internal[IDX_VCX]),
            vbep: p * (internal[IDX_VBX] - internal[IDX_VBP]),
            vbcp: p * (internal[IDX_VSI] - internal[IDX_VBP]),
            vrth: internal[IDX_VRTH],
        }
    }

    fn project_vbic_limited_branches_onto_internal_state(
        &self,
        raw: [Value; INTERNAL_DIM],
        limited: VbicNonlinearBranchVoltages,
    ) -> [Value; INTERNAL_DIM] {
        let p = self.polarity();
        let raw_nodes = [
            raw[IDX_VCX],
            raw[IDX_VCI],
            raw[IDX_VBX],
            raw[IDX_VBI],
            raw[IDX_VEI],
            raw[IDX_VBP],
            raw[IDX_VSI],
        ];
        let constraints = [
            [0.0, 0.0, 0.0, p, -p, 0.0, 0.0],
            [0.0, 0.0, p, 0.0, -p, 0.0, 0.0],
            [0.0, -p, 0.0, p, 0.0, 0.0, 0.0],
            [-p, 0.0, 0.0, p, 0.0, 0.0, 0.0],
            [0.0, 0.0, p, 0.0, 0.0, -p, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, -p, p],
        ];
        let targets = [
            limited.vbei,
            limited.vbex,
            limited.vbci,
            limited.vbcx,
            limited.vbep,
            limited.vbcp,
        ];

        let mut residual = [0.0; VBIC_LIMITED_BRANCH_DIM];
        for row in 0..VBIC_LIMITED_BRANCH_DIM {
            residual[row] = -targets[row];
            for col in 0..raw_nodes.len() {
                residual[row] += constraints[row][col] * raw_nodes[col];
            }
        }

        let mut gram = [[0.0; VBIC_LIMITED_BRANCH_DIM]; VBIC_LIMITED_BRANCH_DIM];
        for row in 0..VBIC_LIMITED_BRANCH_DIM {
            for col in 0..VBIC_LIMITED_BRANCH_DIM {
                gram[row][col] = (0..raw_nodes.len())
                    .map(|idx| constraints[row][idx] * constraints[col][idx])
                    .sum();
            }
        }

        let Some(lagrange) =
            Self::solve_small_dense_system(&gram, &residual, VBIC_LIMITED_BRANCH_DIM)
        else {
            let mut fallback = raw;
            fallback[IDX_VRTH] = limited.vrth;
            return fallback;
        };

        let mut projected = raw;
        for node_idx in 0..raw_nodes.len() {
            let correction = (0..VBIC_LIMITED_BRANCH_DIM)
                .map(|row| constraints[row][node_idx] * lagrange[row])
                .sum::<Value>();
            projected[node_idx] = raw_nodes[node_idx] - correction;
        }
        projected[IDX_VRTH] = limited.vrth;
        projected
    }

    fn limit_vbic_internal_state_to_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if self.charge_model != BjtChargeModel::Vbic {
            return raw;
        }

        let raw_branches = self.vbic_nonlinear_branch_voltages(raw);
        let previous_branches = self.vbic_nonlinear_branch_voltages(previous);
        let (vt, vcrit) = self.vbic_limiting_parameters(previous[IDX_VRTH]);
        let limited_branches = VbicNonlinearBranchVoltages {
            vbei: Self::limit_junction_voltage(
                raw_branches.vbei,
                previous_branches.vbei,
                vt,
                vcrit,
            ),
            vbex: Self::limit_junction_voltage(
                raw_branches.vbex,
                previous_branches.vbex,
                vt,
                vcrit,
            ),
            vbci: Self::limit_junction_voltage(
                raw_branches.vbci,
                previous_branches.vbci,
                vt,
                vcrit,
            ),
            vbcx: Self::limit_junction_voltage(
                raw_branches.vbcx,
                previous_branches.vbcx,
                vt,
                vcrit,
            ),
            vbep: Self::limit_junction_voltage(
                raw_branches.vbep,
                previous_branches.vbep,
                vt,
                vcrit,
            ),
            vbcp: Self::limit_junction_voltage(
                raw_branches.vbcp,
                previous_branches.vbcp,
                vt,
                vcrit,
            ),
            vrth: if self.self_heating_enabled() {
                Self::limit_logarithmic_step(raw_branches.vrth, previous_branches.vrth, 100.0)
                    .max(self.minimum_thermal_rise())
            } else {
                0.0
            },
        };

        let projected =
            self.project_vbic_limited_branches_onto_internal_state(raw, limited_branches);
        if projected.iter().all(|value| value.is_finite()) {
            projected
        } else {
            raw
        }
    }

    pub(crate) fn limit_vbic_dynamic_internal_state_to_previous(
        &self,
        raw: [Value; BJT_INTERNAL_STATE_DIM],
        previous: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        if self.charge_model != BjtChargeModel::Vbic {
            return raw;
        }

        let mut raw_static = [0.0; INTERNAL_DIM];
        raw_static.copy_from_slice(&raw[..INTERNAL_DIM]);
        let mut previous_static = [0.0; INTERNAL_DIM];
        previous_static.copy_from_slice(&previous[..INTERNAL_DIM]);

        let mut limited = raw;
        limited[..INTERNAL_DIM].copy_from_slice(
            &self.limit_vbic_internal_state_to_previous(raw_static, previous_static),
        );
        limited
    }

    #[inline]
    pub(crate) fn predict_vbic_dynamic_internal_state_from_previous_external_bias(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_dynamic: [Value; BJT_INTERNAL_STATE_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        if self.charge_model != BjtChargeModel::Vbic {
            return None;
        }

        let mut previous_static = [0.0; INTERNAL_DIM];
        previous_static.copy_from_slice(&previous_dynamic[..INTERNAL_DIM]);
        let predicted_static = self.predict_intrinsic_state_from_previous_external_bias(
            previous_external,
            previous_static,
            proposed_external,
        )?;

        let mut predicted_dynamic = previous_dynamic;
        predicted_dynamic[..INTERNAL_DIM].copy_from_slice(&predicted_static);
        Some(
            self.limit_vbic_dynamic_internal_state_to_previous(predicted_dynamic, previous_dynamic),
        )
    }

    #[inline]
    pub(crate) fn vbic_dynamic_internal_state_within_local_branch_envelope(
        &self,
        state: [Value; BJT_INTERNAL_STATE_DIM],
        reference: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> bool {
        let mut state_static = [0.0; INTERNAL_DIM];
        state_static.copy_from_slice(&state[..INTERNAL_DIM]);
        let mut reference_static = [0.0; INTERNAL_DIM];
        reference_static.copy_from_slice(&reference[..INTERNAL_DIM]);
        self.vbic_internal_state_within_local_branch_envelope(state_static, reference_static)
    }

    #[inline]
    fn limit_intrinsic_state_against_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if self.charge_model == BjtChargeModel::Vbic {
            self.limit_vbic_internal_state_to_previous(raw, previous)
        } else if self.self_heating_enabled() {
            let mut limited = raw;
            limited[IDX_VRTH] =
                Self::limit_logarithmic_step(raw[IDX_VRTH], previous[IDX_VRTH], 100.0)
                    .max(1.0 - self.requested_temperature());
            limited
        } else {
            raw
        }
    }

    fn predict_intrinsic_state_from_previous_external_bias_unlimited(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        let previous_state = self.intrinsic_state_from_internal_vector(previous_internal);
        let sensitivities = self.internal_voltage_sensitivities(
            previous_state,
            previous_external[EXT_C],
            previous_external[EXT_B],
            previous_external[EXT_E],
            previous_external[EXT_S],
        );
        let delta_external = [
            proposed_external[EXT_C] - previous_external[EXT_C],
            proposed_external[EXT_B] - previous_external[EXT_B],
            proposed_external[EXT_E] - previous_external[EXT_E],
            proposed_external[EXT_S] - previous_external[EXT_S],
        ];

        let mut predicted = previous_internal;
        for internal_idx in 0..INTERNAL_DIM {
            predicted[internal_idx] += sensitivities[internal_idx]
                .iter()
                .zip(delta_external.iter())
                .map(|(sensitivity, delta)| sensitivity * delta)
                .sum::<Value>();
        }

        predicted
            .iter()
            .all(|value| value.is_finite())
            .then_some(predicted)
    }

    fn predict_intrinsic_state_from_previous_external_bias(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        let predicted = self.predict_intrinsic_state_from_previous_external_bias_unlimited(
            previous_external,
            previous_internal,
            proposed_external,
        )?;
        Some(self.limit_intrinsic_state_against_previous(predicted, previous_internal))
    }

    #[inline]
    fn vbic_internal_state_within_local_branch_envelope(
        &self,
        state: [Value; INTERNAL_DIM],
        reference: [Value; INTERNAL_DIM],
    ) -> bool {
        if self.charge_model != BjtChargeModel::Vbic {
            return true;
        }

        let state_branches = self.vbic_nonlinear_branch_voltages(state);
        let reference_branches = self.vbic_nonlinear_branch_voltages(reference);
        let (vt, vcrit) = self.vbic_limiting_parameters(reference[IDX_VRTH]);
        let expected = VbicNonlinearBranchVoltages {
            vbei: Self::limit_junction_voltage(
                state_branches.vbei,
                reference_branches.vbei,
                vt,
                vcrit,
            ),
            vbex: Self::limit_junction_voltage(
                state_branches.vbex,
                reference_branches.vbex,
                vt,
                vcrit,
            ),
            vbci: Self::limit_junction_voltage(
                state_branches.vbci,
                reference_branches.vbci,
                vt,
                vcrit,
            ),
            vbcx: Self::limit_junction_voltage(
                state_branches.vbcx,
                reference_branches.vbcx,
                vt,
                vcrit,
            ),
            vbep: Self::limit_junction_voltage(
                state_branches.vbep,
                reference_branches.vbep,
                vt,
                vcrit,
            ),
            vbcp: Self::limit_junction_voltage(
                state_branches.vbcp,
                reference_branches.vbcp,
                vt,
                vcrit,
            ),
            vrth: if self.self_heating_enabled() {
                Self::limit_logarithmic_step(state_branches.vrth, reference_branches.vrth, 100.0)
                    .max(self.minimum_thermal_rise())
            } else {
                0.0
            },
        };

        [
            (state_branches.vbei, expected.vbei),
            (state_branches.vbex, expected.vbex),
            (state_branches.vbci, expected.vbci),
            (state_branches.vbcx, expected.vbcx),
            (state_branches.vbep, expected.vbep),
            (state_branches.vbcp, expected.vbcp),
            (state_branches.vrth, expected.vrth),
        ]
        .into_iter()
        .all(|(actual, limited)| (actual - limited).abs() <= 1e-12)
    }

    #[inline]
    fn vbic_max_local_branch_delta(
        &self,
        lhs: [Value; INTERNAL_DIM],
        rhs: [Value; INTERNAL_DIM],
    ) -> Value {
        if self.charge_model != BjtChargeModel::Vbic {
            return lhs
                .iter()
                .zip(rhs.iter())
                .map(|(lhs, rhs)| (lhs - rhs).abs())
                .fold(0.0, Value::max);
        }

        let lhs_branches = self.vbic_nonlinear_branch_voltages(lhs);
        let rhs_branches = self.vbic_nonlinear_branch_voltages(rhs);
        [
            (lhs_branches.vbei - rhs_branches.vbei).abs(),
            (lhs_branches.vbex - rhs_branches.vbex).abs(),
            (lhs_branches.vbci - rhs_branches.vbci).abs(),
            (lhs_branches.vbcx - rhs_branches.vbcx).abs(),
            (lhs_branches.vbep - rhs_branches.vbep).abs(),
            (lhs_branches.vbcp - rhs_branches.vbcp).abs(),
            (lhs_branches.vrth - rhs_branches.vrth).abs(),
        ]
        .into_iter()
        .fold(0.0, Value::max)
    }

    fn solve_intrinsic_state_with_external_continuation(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_state: [Value; INTERNAL_DIM],
        target_external: [Value; EXTERNAL_DIM],
    ) -> Option<([Value; INTERNAL_DIM], Value)> {
        let mut current_external = previous_external;
        let mut current_state = previous_state;
        let mut lambda: Value = 0.0;
        let mut step: Value = 1.0;

        while lambda < 1.0 - 1e-15 {
            let candidate_lambda = (lambda + step).min(1.0);
            let next_external = [
                previous_external[EXT_C]
                    + (target_external[EXT_C] - previous_external[EXT_C]) * candidate_lambda,
                previous_external[EXT_B]
                    + (target_external[EXT_B] - previous_external[EXT_B]) * candidate_lambda,
                previous_external[EXT_E]
                    + (target_external[EXT_E] - previous_external[EXT_E]) * candidate_lambda,
                previous_external[EXT_S]
                    + (target_external[EXT_S] - previous_external[EXT_S]) * candidate_lambda,
            ];

            let seed = self
                .predict_intrinsic_state_from_previous_external_bias(
                    current_external,
                    current_state,
                    next_external,
                )
                .unwrap_or(current_state);
            let (solved_state, solved_residual) = self.solve_intrinsic_state_from_seed(
                next_external[EXT_C],
                next_external[EXT_B],
                next_external[EXT_E],
                next_external[EXT_S],
                seed,
            );

            if solved_residual.is_finite()
                && self.vbic_max_local_branch_delta(solved_state, seed) <= 0.1
            {
                current_external = next_external;
                current_state = solved_state;
                lambda = candidate_lambda;
                step = (step * 2.0).min(1.0 - lambda).max(1e-6);
                continue;
            }

            if step <= 1.0 / 256.0 {
                return None;
            }
            step *= 0.5;
        }

        let residual = Self::intrinsic_state_residual_norm(
            &self
                .intrinsic_state_residual_jacobian(
                    target_external[EXT_C],
                    target_external[EXT_B],
                    target_external[EXT_E],
                    target_external[EXT_S],
                    current_state,
                )
                .0,
        );
        Some((current_state, residual))
    }

    #[inline]
    fn vbic_cached_external_matches(
        &self,
        external: [Value; EXTERNAL_DIM],
        voltage_abstol: Value,
        reltol: Value,
    ) -> bool {
        let cached = [self.vc_ext, self.vb_ext, self.ve_ext, self.vs_ext];
        cached
            .iter()
            .zip(external.iter())
            .all(|(cached, external)| {
                let diff = (cached - external).abs();
                let tol = reltol * cached.abs().max(external.abs()) + voltage_abstol;
                diff <= tol
            })
    }

    #[inline]
    fn vbic_branch_limit_scale(previous: Value, raw: Value, limited: Value) -> Option<Value> {
        let raw_delta = raw - previous;
        if !raw_delta.is_finite() || raw_delta.abs() <= 1e-18 {
            return None;
        }
        let limited_delta = limited - previous;
        if !limited_delta.is_finite() {
            return Some(0.0);
        }
        Some((limited_delta.abs() / raw_delta.abs()).clamp(0.0, 1.0))
    }

    pub(crate) fn vbic_external_step_limit_scale_from_state(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<Value> {
        if self.charge_model != BjtChargeModel::Vbic {
            return None;
        }

        let delta_external = [
            proposed_external[EXT_C] - previous_external[EXT_C],
            proposed_external[EXT_B] - previous_external[EXT_B],
            proposed_external[EXT_E] - previous_external[EXT_E],
            proposed_external[EXT_S] - previous_external[EXT_S],
        ];
        let max_delta = delta_external
            .iter()
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        if !max_delta.is_finite() || max_delta <= 1e-15 {
            return None;
        }

        let Some(raw_internal) = self
            .predict_intrinsic_state_from_previous_external_bias_unlimited(
                previous_external,
                previous_internal,
                proposed_external,
            )
        else {
            return Some(0.5);
        };
        if !raw_internal.iter().all(|value| value.is_finite()) {
            return Some(0.5);
        }

        let limited_internal =
            self.limit_intrinsic_state_against_previous(raw_internal, previous_internal);
        let previous_branches = self.vbic_nonlinear_branch_voltages(previous_internal);
        let raw_branches = self.vbic_nonlinear_branch_voltages(raw_internal);
        let limited_branches = self.vbic_nonlinear_branch_voltages(limited_internal);

        let mut scale: Value = 1.0;
        let mut engaged = false;
        for branch_scale in [
            Self::vbic_branch_limit_scale(
                previous_branches.vbei,
                raw_branches.vbei,
                limited_branches.vbei,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbex,
                raw_branches.vbex,
                limited_branches.vbex,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbci,
                raw_branches.vbci,
                limited_branches.vbci,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbcx,
                raw_branches.vbcx,
                limited_branches.vbcx,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbep,
                raw_branches.vbep,
                limited_branches.vbep,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbcp,
                raw_branches.vbcp,
                limited_branches.vbcp,
            ),
            if self.self_heating_enabled() {
                Self::vbic_branch_limit_scale(
                    previous_branches.vrth,
                    raw_branches.vrth,
                    limited_branches.vrth,
                )
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        {
            if branch_scale + 1e-15 < 1.0 {
                engaged = true;
            }
            scale = scale.min(branch_scale);
        }

        engaged.then_some(scale.max(0.0))
    }

    pub(crate) fn vbic_external_step_limit_scale_against_previous(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<Value> {
        let previous_internal = if self.vbic_cached_external_matches(previous_external, 1e-12, 1e-9)
        {
            self.internal_state_vector()
        } else {
            let solved_previous = self.solve_intrinsic_terminal_state(
                previous_external[EXT_C],
                previous_external[EXT_B],
                previous_external[EXT_E],
                previous_external[EXT_S],
            );
            [
                solved_previous.vcx,
                solved_previous.vci,
                solved_previous.vbx,
                solved_previous.vbi,
                solved_previous.vei,
                solved_previous.vbp,
                solved_previous.vsi,
                solved_previous.vrth,
            ]
        };

        self.vbic_external_step_limit_scale_from_state(
            previous_external,
            previous_internal,
            proposed_external,
        )
    }

    fn evaluate_state_fixed_temperature(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        vcx: Value,
        vci: Value,
        vbx: Value,
        vbi: Value,
        vei: Value,
        vbp: Value,
        vsi: Value,
    ) -> EvaluatedBjtState {
        let (linearized, intrinsic) = self.linearize_currents_with_branches(vbi - vei, vbi - vci);
        EvaluatedBjtState {
            linearized,
            ibe: intrinsic.ibe,
            ibc: intrinsic.ibc,
            iciei: intrinsic.iciei,
            ircx: self.ircx_branch(vc, vcx),
            irci: self.irci_branch(vcx, vci, vbi),
            irbx: self.irbx_branch(vb, vbx),
            irbi: self.irbi_branch(linearized, vbx, vbi),
            ire: self.ire_branch(ve, vei),
            ibep: self.ibep_branch(vbx, vbp),
            irbp: self.irbp_branch(vbx, vbi, vcx, vci, vbp, vsi),
            ibcp: self.ibcp_branch(vbp, vsi),
            iccp: self.iccp_branch(vbx, vbi, vci, vbp, vsi),
            irs: self.irs_branch(vs, vsi),
        }
    }

    fn apply_thermal_derivative(
        base: &mut BranchLinearization,
        plus: BranchLinearization,
        minus: BranchLinearization,
        denom: Value,
    ) {
        base.d_internal[IDX_VRTH] = (plus.current - minus.current) / denom;
    }

    fn evaluate_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        vcx: Value,
        vci: Value,
        vbx: Value,
        vbi: Value,
        vei: Value,
        vbp: Value,
        vsi: Value,
        vrth: Value,
    ) -> EvaluatedBjtState {
        let mut evaluated = self.with_temperature_variant(vrth, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });

        if !self.self_heating_enabled() {
            return evaluated;
        }

        let h = self.thermal_derivative_step(vrth);
        let plus = self.with_temperature_variant(vrth + h, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });
        let minus = self.with_temperature_variant(vrth - h, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });
        let denom = 2.0 * h;

        evaluated.linearized.dic_dvrth = (plus.linearized.ic - minus.linearized.ic) / denom;
        evaluated.linearized.dib_dvrth = (plus.linearized.ib - minus.linearized.ib) / denom;
        evaluated.linearized.dqb_dvrth = (plus.linearized.qb - minus.linearized.qb) / denom;
        Self::apply_thermal_derivative(&mut evaluated.ibe, plus.ibe, minus.ibe, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibc, plus.ibc, minus.ibc, denom);
        Self::apply_thermal_derivative(&mut evaluated.iciei, plus.iciei, minus.iciei, denom);
        Self::apply_thermal_derivative(&mut evaluated.ircx, plus.ircx, minus.ircx, denom);
        Self::apply_thermal_derivative(&mut evaluated.irci, plus.irci, minus.irci, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbx, plus.irbx, minus.irbx, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbi, plus.irbi, minus.irbi, denom);
        Self::apply_thermal_derivative(&mut evaluated.ire, plus.ire, minus.ire, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibep, plus.ibep, minus.ibep, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbp, plus.irbp, minus.irbp, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibcp, plus.ibcp, minus.ibcp, denom);
        Self::apply_thermal_derivative(&mut evaluated.iccp, plus.iccp, minus.iccp, denom);
        Self::apply_thermal_derivative(&mut evaluated.irs, plus.irs, minus.irs, denom);
        evaluated
    }

    fn intrinsic_state_for_biases(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> IntrinsicTerminalState {
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            let thermal_model = self
                .self_heating_enabled()
                .then(|| self.temperature_variant(self.vrth));
            let model = thermal_model.as_ref().unwrap_or(self);
            IntrinsicTerminalState {
                vcx: self.vcx,
                vci: self.vci,
                vbx: self.vbx,
                vbi: self.vbi,
                vei: self.vei,
                vbp: self.vbp,
                vsi: self.vsi,
                vrth: self.vrth,
                linearized: model.linearize_currents(self.vbe, self.vbc),
            }
        } else {
            self.solve_intrinsic_terminal_state(vc, vb, ve, vs)
        }
    }

    #[inline]
    fn intrinsic_state_residual_norm(residual: &[Value; INTERNAL_DIM]) -> Value {
        residual
            .iter()
            .fold(0.0, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    fn intrinsic_state_step_limit(iteration: usize, residual_norm: Value) -> Value {
        if residual_norm > 1e-2 {
            if iteration < 4 { 0.25 } else { 0.15 }
        } else if residual_norm > 1e-6 {
            0.1
        } else {
            0.05
        }
    }

    fn solve_intrinsic_state_from_seed_with_thermal_scale(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        thermal_scale: Value,
        mut state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        let mut best_state = state;
        let mut best_residual_norm = Value::INFINITY;

        for iteration in 0..32 {
            let (residual, jacobian) = self.intrinsic_state_residual_jacobian_with_thermal_scale(
                vc,
                vb,
                ve,
                vs,
                state,
                thermal_scale,
            );
            let residual_norm = Self::intrinsic_state_residual_norm(&residual);
            if residual_norm < best_residual_norm {
                best_residual_norm = residual_norm;
                best_state = state;
            }
            if !residual_norm.is_finite() || residual_norm < 1e-14 {
                break;
            }

            let rhs = residual.map(|value| -value);
            let Some(delta) = Self::solve_small_dense_system(&jacobian, &rhs, INTERNAL_DIM) else {
                break;
            };

            let max_raw_delta = delta
                .iter()
                .fold(0.0_f64, |max_delta, value| max_delta.max(value.abs()));
            if max_raw_delta < 1e-13 {
                break;
            }

            let base_limit = Self::intrinsic_state_step_limit(iteration, residual_norm);
            let mut alpha = if max_raw_delta > base_limit {
                base_limit / max_raw_delta
            } else {
                1.0
            };
            alpha = alpha.clamp(1e-3, 1.0);

            let mut accepted = false;
            let mut candidate = state;
            let mut candidate_residual_norm = residual_norm;
            let mut best_candidate = state;
            let mut best_candidate_residual_norm = residual_norm;
            for _ in 0..12 {
                for idx in 0..INTERNAL_DIM {
                    candidate[idx] = state[idx] + alpha * delta[idx];
                }
                candidate = self.limit_intrinsic_state_against_previous(candidate, state);
                let (candidate_residual, _) = self
                    .intrinsic_state_residual_jacobian_with_thermal_scale(
                        vc,
                        vb,
                        ve,
                        vs,
                        candidate,
                        thermal_scale,
                    );
                candidate_residual_norm = Self::intrinsic_state_residual_norm(&candidate_residual);
                if candidate_residual_norm.is_finite()
                    && candidate_residual_norm < best_candidate_residual_norm
                {
                    best_candidate = candidate;
                    best_candidate_residual_norm = candidate_residual_norm;
                }
                if candidate_residual_norm.is_finite() && candidate_residual_norm < residual_norm {
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if !accepted && best_candidate_residual_norm < residual_norm {
                candidate = best_candidate;
                candidate_residual_norm = best_candidate_residual_norm;
                accepted = true;
            }

            if !accepted {
                break;
            }

            state = candidate;
            if candidate_residual_norm < best_residual_norm {
                best_residual_norm = candidate_residual_norm;
                best_state = state;
            }
            if candidate_residual_norm < 1e-14 {
                break;
            }
        }

        (best_state, best_residual_norm)
    }

    fn solve_intrinsic_state_from_seed(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        self.solve_intrinsic_state_from_seed_with_thermal_scale(vc, vb, ve, vs, 1.0, state)
    }

    fn solve_intrinsic_state_with_self_heating_continuation(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        let (direct_state, direct_residual_norm) =
            self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, state);
        if !self.self_heating_enabled() {
            return (direct_state, direct_residual_norm);
        }

        let minimum_vrth = 0.0_f64.max(self.minimum_thermal_rise());
        let mut continuation_state = state;
        continuation_state[IDX_VRTH] = continuation_state[IDX_VRTH].max(minimum_vrth);
        for thermal_scale in [0.0, 0.05, 0.125, 0.25, 0.5, 0.75, 1.0] {
            if thermal_scale == 0.0 {
                continuation_state[IDX_VRTH] = minimum_vrth;
            }
            let (solved_state, _) = self.solve_intrinsic_state_from_seed_with_thermal_scale(
                vc,
                vb,
                ve,
                vs,
                thermal_scale,
                continuation_state,
            );
            continuation_state = solved_state;
        }

        let (continued_state, continued_residual_norm) =
            self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, continuation_state);
        if continued_residual_norm < direct_residual_norm {
            (continued_state, continued_residual_norm)
        } else {
            (direct_state, direct_residual_norm)
        }
    }

    fn rebalance_intrinsic_thermal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if !self.self_heating_enabled() {
            return state;
        }

        let mut current_state = state;
        let mut best_state = state;
        let mut best_residual = Value::INFINITY;
        let minimum_vrth = self.minimum_thermal_rise();

        for _ in 0..8 {
            let (residual, jacobian) =
                self.intrinsic_state_residual_jacobian(vc, vb, ve, vs, current_state);
            let thermal_residual = residual[IDX_VRTH];
            let thermal_residual_abs = thermal_residual.abs();
            if thermal_residual_abs.is_finite() && thermal_residual_abs < best_residual {
                best_residual = thermal_residual_abs;
                best_state = current_state;
            }
            let thermal_derivative = jacobian[IDX_VRTH][IDX_VRTH];
            if !thermal_residual.is_finite()
                || !thermal_derivative.is_finite()
                || thermal_derivative.abs() < 1e-18
                || thermal_residual_abs < 1e-12
            {
                break;
            }

            let current_vrth = current_state[IDX_VRTH];
            let max_step = (current_vrth - minimum_vrth + 10.0).max(1.0) * 0.5;
            let step = (-thermal_residual / thermal_derivative).clamp(-max_step, max_step);
            if step.abs() < 1e-12 {
                break;
            }

            let mut alpha = 1.0;
            let mut accepted = false;
            let mut best_candidate = current_state;
            let mut best_candidate_residual = thermal_residual_abs;
            for _ in 0..10 {
                let raw_vrth = current_vrth + alpha * step;
                let candidate_vrth =
                    Self::limit_logarithmic_step(raw_vrth, current_vrth, 100.0).max(minimum_vrth);
                if (candidate_vrth - current_vrth).abs() < 1e-12 {
                    break;
                }

                let mut candidate = current_state;
                candidate[IDX_VRTH] = candidate_vrth;
                let candidate_residual = self
                    .intrinsic_state_residual_jacobian(vc, vb, ve, vs, candidate)
                    .0[IDX_VRTH]
                    .abs();
                if candidate_residual.is_finite() && candidate_residual < best_candidate_residual {
                    best_candidate = candidate;
                    best_candidate_residual = candidate_residual;
                }
                if candidate_residual.is_finite() && candidate_residual < thermal_residual_abs {
                    current_state = candidate;
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if accepted {
                continue;
            }
            if best_candidate_residual + 1e-15 < thermal_residual_abs {
                current_state = best_candidate;
                continue;
            }
            break;
        }

        best_state
    }

    fn intrinsic_state_residual_jacobian_with_thermal_scale(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
        thermal_scale: Value,
    ) -> ([Value; INTERNAL_DIM], [[Value; INTERNAL_DIM]; INTERNAL_DIM]) {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let solve_vbp = Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0;

        let [
            mut vcx,
            mut vci,
            mut vbx,
            mut vbi,
            mut vei,
            mut vbp,
            mut vsi,
            mut vrth,
        ] = state;
        if !has_rcx {
            vcx = vc;
        }
        if !has_rci {
            vci = vcx;
        }
        if !has_rbx {
            vbx = vb;
        }
        if !has_rbi {
            vbi = vbx;
        }
        if !has_re {
            vei = ve;
        }
        if !has_rs {
            vsi = vs;
        }
        if !solve_vbp {
            vbp = vcx;
        }
        if !has_self_heat {
            vrth = 0.0;
        }

        let eval = self.evaluate_state(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi, vrth);
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);
        let thermal_sink = self.thermal_sink_branch(vrth);
        let thermal_power = Self::scale_branch(
            self.thermal_power_branch(eval, [vc, vb, ve, vs], state),
            thermal_scale,
        );

        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut residual = [0.0; INTERNAL_DIM];

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(eval.ircx, eval.irbp),
                if has_rci {
                    eval.irci
                } else {
                    collector_internal
                },
            );
            residual[IDX_VCX] = row.current;
            jacobian[IDX_VCX] = row.d_internal;
        } else {
            residual[IDX_VCX] = vcx - vc;
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            residual[IDX_VCI] = row.current;
            jacobian[IDX_VCI] = row.d_internal;
        } else {
            residual[IDX_VCI] = vci - vcx;
            jacobian[IDX_VCI][IDX_VCI] = 1.0;
            jacobian[IDX_VCI][IDX_VCX] = -1.0;
        }

        if has_rbx {
            let row = Self::sub_branches(
                Self::sub_branches(
                    Self::sub_branches(eval.irbx, if has_rbi { eval.irbi } else { base_internal }),
                    eval.ibep,
                ),
                eval.iccp,
            );
            residual[IDX_VBX] = row.current;
            jacobian[IDX_VBX] = row.d_internal;
        } else {
            residual[IDX_VBX] = vbx - vb;
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            residual[IDX_VBI] = row.current;
            jacobian[IDX_VBI] = row.d_internal;
        } else {
            residual[IDX_VBI] = vbi - vbx;
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(eval.ire, emitter_internal);
            residual[IDX_VEI] = row.current;
            jacobian[IDX_VEI] = row.d_internal;
        } else {
            residual[IDX_VEI] = vei - ve;
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            residual[IDX_VBP] = row.current;
            jacobian[IDX_VBP] = row.d_internal;
        } else {
            residual[IDX_VBP] = vbp - vcx;
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            residual[IDX_VSI] = row.current;
            jacobian[IDX_VSI] = row.d_internal;
        } else {
            residual[IDX_VSI] = vsi - vs;
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            residual[IDX_VRTH] = row.current;
            jacobian[IDX_VRTH] = row.d_internal;
        } else {
            residual[IDX_VRTH] = vrth;
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        (residual, jacobian)
    }

    fn intrinsic_state_residual_jacobian(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], [[Value; INTERNAL_DIM]; INTERNAL_DIM]) {
        self.intrinsic_state_residual_jacobian_with_thermal_scale(vc, vb, ve, vs, state, 1.0)
    }

    fn internal_kcl_linearization(
        &self,
        state: IntrinsicTerminalState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (
        EvaluatedBjtState,
        [[Value; INTERNAL_DIM]; INTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
    ) {
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let (jacobian, external_partials, _) =
            self.internal_kcl_linearization_from_eval(state, eval, vc, vb, ve, vs);
        (eval, jacobian, external_partials)
    }

    fn internal_kcl_linearization_from_eval(
        &self,
        state: IntrinsicTerminalState,
        eval: EvaluatedBjtState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (
        [[Value; INTERNAL_DIM]; INTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
    ) {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let solve_vbp = Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0;
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);
        let thermal_sink = self.thermal_sink_branch(state.vrth);
        let thermal_power = self.thermal_power_branch(
            eval,
            [vc, vb, ve, vs],
            [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
        );

        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut external_partials = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        let mut source = [0.0; INTERNAL_DIM];
        let internal = [
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth,
        ];
        let external = [vc, vb, ve, vs];
        let assign_row = |row_idx: usize,
                          row: BranchLinearization,
                          jacobian: &mut [[Value; INTERNAL_DIM]; INTERNAL_DIM],
                          external_partials: &mut [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
                          source: &mut [Value; INTERNAL_DIM]| {
            jacobian[row_idx] = row.d_internal;
            external_partials[row_idx] = row.d_external;
            source[row_idx] = row
                .d_internal
                .iter()
                .zip(internal.iter())
                .map(|(d, v)| d * v)
                .sum::<Value>()
                + row
                    .d_external
                    .iter()
                    .zip(external.iter())
                    .map(|(d, v)| d * v)
                    .sum::<Value>()
                - row.current;
        };

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(eval.ircx, eval.irbp),
                if has_rci {
                    eval.irci
                } else {
                    collector_internal
                },
            );
            assign_row(
                IDX_VCX,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
            external_partials[IDX_VCX][EXT_C] = -1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            assign_row(
                IDX_VCI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VCI][IDX_VCI] = 1.0;
            jacobian[IDX_VCI][IDX_VCX] = -1.0;
        }

        if has_rbx {
            let row = Self::sub_branches(
                Self::sub_branches(
                    Self::sub_branches(eval.irbx, if has_rbi { eval.irbi } else { base_internal }),
                    eval.ibep,
                ),
                eval.iccp,
            );
            assign_row(
                IDX_VBX,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
            external_partials[IDX_VBX][EXT_B] = -1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            assign_row(
                IDX_VBI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(eval.ire, emitter_internal);
            assign_row(
                IDX_VEI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
            external_partials[IDX_VEI][EXT_E] = -1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            assign_row(
                IDX_VBP,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            assign_row(
                IDX_VSI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
            external_partials[IDX_VSI][EXT_S] = -1.0;
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            assign_row(
                IDX_VRTH,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        (jacobian, external_partials, source)
    }

    fn reduced_linearization_from_state_and_eval(
        &self,
        state: IntrinsicTerminalState,
        eval: EvaluatedBjtState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        let (g_ii, g_ie, z_i_static) =
            self.internal_kcl_linearization_from_eval(state, eval, vc, vb, ve, vs);
        let terminal_currents = self.external_terminal_branches(eval);
        let (g_ei, g_ee, g_reduced) =
            Self::linearized_terminal_conductance_matrices(&g_ii, &g_ie, &terminal_currents);
        let internal = [
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth,
        ];
        let external = [vc, vb, ve, vs];
        let mut z_e_static = [0.0; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            z_e_static[row] = terminal_currents[row]
                .d_internal
                .iter()
                .zip(internal.iter())
                .map(|(d, v)| d * v)
                .sum::<Value>()
                + terminal_currents[row]
                    .d_external
                    .iter()
                    .zip(external.iter())
                    .map(|(d, v)| d * v)
                    .sum::<Value>()
                - terminal_currents[row].current;
        }
        let cached_dynamic_inputs = if self.uses_vbic_dynamic_charges() {
            let internal = [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth, 0.0, 0.0,
            ];
            Some(if self.self_heating_enabled() {
                self.with_temperature_variant(state.vrth, |model| {
                    model.dynamic_charge_inputs(external, internal)
                })
            } else {
                self.dynamic_charge_inputs(external, internal)
            })
        } else {
            None
        };

        BjtReducedLinearization {
            internal_voltages: [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
            external_voltages: [vc, vb, ve, vs],
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            g_reduced,
            z_i_static,
            z_e_static,
            cached_dynamic_inputs,
        }
    }

    #[inline]
    fn intrinsic_state_from_internal_vector(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> IntrinsicTerminalState {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = internal;
        let linearized = self
            .with_temperature_variant(vrth, |model| model.linearize_currents(vbi - vei, vbi - vci));

        IntrinsicTerminalState {
            vcx,
            vci,
            vbx,
            vbi,
            vei,
            vbp,
            vsi,
            vrth,
            linearized,
        }
    }

    fn compute_reduced_linearization(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        let state = self.intrinsic_state_for_biases(vc, vb, ve, vs);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        self.reduced_linearization_from_state_and_eval(state, eval, vc, vb, ve, vs)
    }

    pub(crate) fn reduced_linearization(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        if self.reduced_linearization_cache_valid.get()
            && self.cache_matches_external_biases(vc, vb, ve, vs)
        {
            return self.reduced_linearization_cache.get();
        }

        let reduced = self.compute_reduced_linearization(vc, vb, ve, vs);
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            self.reduced_linearization_cache.set(reduced);
            self.reduced_linearization_cache_valid.set(true);
        }
        reduced
    }

    fn linearized_terminal_conductance_matrices(
        g_ii: &[[Value; INTERNAL_DIM]; INTERNAL_DIM],
        g_ie: &[[Value; EXTERNAL_DIM]; INTERNAL_DIM],
        terminal_currents: &[BranchLinearization; EXTERNAL_DIM],
    ) -> (
        [[Value; INTERNAL_DIM]; EXTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; EXTERNAL_DIM],
        BjtConductanceMatrix,
    ) {
        let mut g_ei = [[0.0; INTERNAL_DIM]; EXTERNAL_DIM];
        let mut g_ee = [[0.0; EXTERNAL_DIM]; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            g_ei[row] = terminal_currents[row].d_internal;
            g_ee[row] = terminal_currents[row].d_external;
        }

        let mut sensitivities = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        for external in 0..EXTERNAL_DIM {
            let rhs = g_ie.map(|partials| -partials[external]);
            if let Some(solution) = Self::solve_small_dense_system(g_ii, &rhs, INTERNAL_DIM) {
                for idx in 0..INTERNAL_DIM {
                    sensitivities[idx][external] = solution[idx];
                }
            }
        }

        let mut g_reduced = [[0.0; EXTERNAL_DIM]; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            for col in 0..EXTERNAL_DIM {
                let mut value = g_ee[row][col];
                for internal in 0..INTERNAL_DIM {
                    value += g_ei[row][internal] * sensitivities[internal][col];
                }
                g_reduced[row][col] = value;
            }
        }

        (g_ei, g_ee, g_reduced)
    }

    fn epi_charge_state(&self, vcx: Value, vci: Value, vbi: Value) -> EpiChargeState {
        let mut state = EpiChargeState {
            kbci: 1.0,
            d_kbci: [0.0; INTERNAL_DIM],
            kbcx: 1.0,
            d_kbcx: [0.0; INTERNAL_DIM],
        };

        if self.gamm <= 0.0 {
            return state;
        }

        let p = self.polarity();
        let vt = self.vt.max(1e-12);
        let vbci_eff = p * (vbi - vci);
        let vbcx_eff = p * (vbi - vcx);

        let (exp_bci, dexp_bci_darg) = Self::limited_exp(vbci_eff / vt);
        let (exp_bcx, dexp_bcx_darg) = Self::limited_exp(vbcx_eff / vt);
        let d_exp_bci_dv = dexp_bci_darg / vt;
        let d_exp_bcx_dv = dexp_bcx_darg / vt;

        state.kbci = (1.0 + self.gamm * exp_bci).sqrt().max(1e-12);
        state.kbcx = (1.0 + self.gamm * exp_bcx).sqrt().max(1e-12);

        let d_kbci_dv = self.gamm * d_exp_bci_dv / (2.0 * state.kbci);
        let d_kbcx_dv = self.gamm * d_exp_bcx_dv / (2.0 * state.kbcx);

        state.d_kbci[IDX_VBI] = p * d_kbci_dv;
        state.d_kbci[IDX_VCI] = -p * d_kbci_dv;
        state.d_kbcx[IDX_VBI] = p * d_kbcx_dv;
        state.d_kbcx[IDX_VCX] = -p * d_kbcx_dv;
        state
    }

    fn dynamic_reduction_template(&self, base: BjtReducedLinearization) -> BjtDynamicReduction {
        let mut reduction = BjtDynamicReduction {
            external_voltages: base.external_voltages,
            g_ee: base.g_ee,
            g_reduced: base.g_reduced,
            ..Default::default()
        };

        for idx in 0..INTERNAL_DIM {
            reduction.internal_voltages[idx] = base.internal_voltages[idx];
        }
        for row in 0..INTERNAL_DIM {
            for col in 0..INTERNAL_DIM {
                reduction.g_ii[row][col] = base.g_ii[row][col];
            }
            reduction.g_ie[row] = base.g_ie[row];
            reduction.z_i_static[row] = base.z_i_static[row];
        }
        for row in 0..EXTERNAL_DIM {
            for col in 0..INTERNAL_DIM {
                reduction.g_ei[row][col] = base.g_ei[row][col];
            }
            reduction.z_e_static[row] = base.z_e_static[row];
        }

        // Default the excess-phase states to decoupled algebraic identities when
        // TD is not active so the dynamic reduction remains well-conditioned.
        reduction.g_ii[IDX_VXF1][IDX_VXF1] = 1.0;
        reduction.g_ii[IDX_VXF2][IDX_VXF2] = 1.0;
        reduction
    }

    fn dynamic_charge_inputs(
        &self,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> BjtDynamicChargeInputs {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, _vrth, _, _] = internal;
        let [vc, vb, ve, _vs] = external;
        let p = self.polarity();

        let vbe_eff = p * (vbi - vei);
        let vbex_eff = p * (vbx - vei);
        let vbc_eff = p * (vbi - vci);
        let vbep_eff = p * (vbx - vbp);
        let vbcp_eff = p * (vsi - vbp);
        let vbeo_eff = p * (vb - ve);
        let vbco_eff = p * (vb - vc);

        let transport = self.transport_charge_state(vbe_eff, vbc_eff);
        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let epi = self.epi_charge_state(vcx, vci, vbi);

        let (qdbe, dqdbe_dvbe_eff) = self
            .vbic_depletion_charge_and_derivative(vbe_eff, self.vje, self.mje, self.fc, self.aje);
        let (qdbex, dqdbex_dvbex_eff) = self
            .vbic_depletion_charge_and_derivative(vbex_eff, self.vje, self.mje, self.fc, self.aje);
        let (qdbc, dqdbc_dvbc_eff) = self
            .vbic_depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc, self.fc, self.ajc);
        let (qdbep, dqdbep_dvbep_eff) = self
            .vbic_depletion_charge_and_derivative(vbep_eff, self.vjc, self.mjc, self.fc, self.ajc);
        let (qdbcp, dqdbcp_dvbcp_eff) = self
            .vbic_depletion_charge_and_derivative(vbcp_eff, self.ps, self.ms, self.fc, self.ajs);

        let _ = (vbeo_eff, vbco_eff);

        BjtDynamicChargeInputs {
            transport,
            parasitic,
            epi,
            qdbe,
            dqdbe_dvbe_eff,
            qdbex,
            dqdbex_dvbex_eff,
            qdbc,
            dqdbc_dvbc_eff,
            qdbep,
            dqdbep_dvbep_eff,
            qdbcp,
            dqdbcp_dvbcp_eff,
        }
    }

    pub(crate) fn vbic_delay_static_branches(
        &self,
        reduction: &BjtDynamicReduction,
    ) -> [BjtCurrentBranch; 3] {
        let mut branches = [BjtCurrentBranch::default(); 3];
        if !self.uses_vbic_dynamic_charges() || self.td <= 0.0 {
            return branches;
        }

        let [_, _, _, _, _, _, _, _vrth, vxf1, vxf2] = reduction.internal_voltages;
        let p = self.polarity();
        let transport = reduction.vbic_transport;
        let d_itzf_actual_d_vbi = p * (transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff);
        let d_itzf_actual_d_vci = -p * transport.ditzf_dvbc_eff;
        let d_itzf_actual_d_vei = -p * transport.ditzf_dvbe_eff;
        let d_p_itzf_d_vbi = transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vci = -transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vei = -transport.ditzf_dvbe_eff;
        let d_itzf_d_vrth = reduction.vbic_d_itzf_d_vrth;

        let mut delta_iciei = BjtCurrentBranch {
            current: p * (vxf2 - transport.itzf),
            pos_internal: Some(IDX_VEI),
            neg_internal: Some(IDX_VCI),
            ..Default::default()
        };
        delta_iciei.d_internal[IDX_VBI] = -d_p_itzf_d_vbi;
        delta_iciei.d_internal[IDX_VCI] = -d_p_itzf_d_vci;
        delta_iciei.d_internal[IDX_VEI] = -d_p_itzf_d_vei;
        delta_iciei.d_internal[IDX_VRTH] = -p * d_itzf_d_vrth;
        delta_iciei.d_internal[IDX_VXF2] = p;
        branches[0] = delta_iciei;

        let mut ixf1 = BjtCurrentBranch {
            current: vxf2 - transport.itzf,
            pos_internal: Some(IDX_VXF1),
            ..Default::default()
        };
        ixf1.d_internal[IDX_VBI] = -d_itzf_actual_d_vbi;
        ixf1.d_internal[IDX_VCI] = -d_itzf_actual_d_vci;
        ixf1.d_internal[IDX_VEI] = -d_itzf_actual_d_vei;
        ixf1.d_internal[IDX_VRTH] = -d_itzf_d_vrth;
        ixf1.d_internal[IDX_VXF2] = 1.0;
        branches[1] = ixf1;

        let mut ixf2 = BjtCurrentBranch {
            current: vxf2 - vxf1,
            pos_internal: Some(IDX_VXF2),
            ..Default::default()
        };
        ixf2.d_internal[IDX_VXF1] = -1.0;
        ixf2.d_internal[IDX_VXF2] = 1.0;
        branches[2] = ixf2;

        branches
    }

    pub(crate) fn vbic_delay_static_thermal_branch(
        &self,
        reduction: &BjtDynamicReduction,
    ) -> BjtCurrentBranch {
        if !self.uses_vbic_dynamic_charges() || self.td <= 0.0 || !self.self_heating_enabled() {
            return BjtCurrentBranch::default();
        }

        let [_, vci, _, _, vei, _, _, _vrth, _, vxf2] = reduction.internal_voltages;
        let p = self.polarity();
        let transport = reduction.vbic_transport;
        let d_p_itzf_d_vbi = transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vci = -transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vei = -transport.ditzf_dvbe_eff;
        let d_itzf_d_vrth = reduction.vbic_d_itzf_d_vrth;

        let delta_current = p * (vxf2 - transport.itzf);
        let voltage = vci - vei;
        let mut branch = BjtCurrentBranch {
            current: -delta_current * voltage,
            pos_internal: Some(IDX_VRTH),
            ..Default::default()
        };
        branch.d_internal[IDX_VBI] = d_p_itzf_d_vbi * voltage;
        branch.d_internal[IDX_VCI] = d_p_itzf_d_vci * voltage - delta_current;
        branch.d_internal[IDX_VEI] = d_p_itzf_d_vei * voltage + delta_current;
        branch.d_internal[IDX_VRTH] = p * d_itzf_d_vrth * voltage;
        branch.d_internal[IDX_VXF2] = -p * voltage;
        branch
    }

    fn apply_vbic_excess_phase_transport(
        &self,
        mut reduction: BjtDynamicReduction,
        transport: TransportChargeState,
        d_itzf_d_vrth: Value,
    ) -> BjtDynamicReduction {
        if !self.uses_vbic_dynamic_charges() || self.td <= 0.0 {
            return reduction;
        }

        let p = self.polarity();

        // Direct forward transport already appears in the 7-state DC Jacobian.
        // Replace that static path with ngspice's excess-phase xf2-controlled path.
        let d_itzf_actual_d_vbi = p * (transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff);
        let d_itzf_actual_d_vci = -p * transport.ditzf_dvbc_eff;
        let d_itzf_actual_d_vei = -p * transport.ditzf_dvbe_eff;
        let d_p_itzf_d_vbi = transport.ditzf_dvbe_eff + transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vci = -transport.ditzf_dvbc_eff;
        let d_p_itzf_d_vei = -transport.ditzf_dvbe_eff;

        reduction.g_ii[IDX_VCI][IDX_VBI] += d_p_itzf_d_vbi;
        reduction.g_ii[IDX_VCI][IDX_VCI] += d_p_itzf_d_vci;
        reduction.g_ii[IDX_VCI][IDX_VEI] += d_p_itzf_d_vei;
        reduction.g_ii[IDX_VCI][IDX_VRTH] += p * d_itzf_d_vrth;
        reduction.g_ii[IDX_VCI][IDX_VXF2] -= p;

        reduction.g_ii[IDX_VEI][IDX_VBI] -= d_p_itzf_d_vbi;
        reduction.g_ii[IDX_VEI][IDX_VCI] -= d_p_itzf_d_vci;
        reduction.g_ii[IDX_VEI][IDX_VEI] -= d_p_itzf_d_vei;
        reduction.g_ii[IDX_VEI][IDX_VRTH] -= p * d_itzf_d_vrth;
        reduction.g_ii[IDX_VEI][IDX_VXF2] += p;

        reduction.g_ii[IDX_VXF1] = [0.0; BJT_INTERNAL_STATE_DIM];
        reduction.g_ii[IDX_VXF1][IDX_VBI] = -d_itzf_actual_d_vbi;
        reduction.g_ii[IDX_VXF1][IDX_VCI] = -d_itzf_actual_d_vci;
        reduction.g_ii[IDX_VXF1][IDX_VEI] = -d_itzf_actual_d_vei;
        reduction.g_ii[IDX_VXF1][IDX_VRTH] = -d_itzf_d_vrth;
        reduction.g_ii[IDX_VXF1][IDX_VXF2] = 1.0;

        reduction.g_ii[IDX_VXF2] = [0.0; BJT_INTERNAL_STATE_DIM];
        reduction.g_ii[IDX_VXF2][IDX_VXF1] = -1.0;
        reduction.g_ii[IDX_VXF2][IDX_VXF2] = 1.0;

        reduction
    }

    fn build_dynamic_reduction_from_transport(
        &self,
        mut reduction: BjtDynamicReduction,
        transport: TransportChargeState,
        d_itzf_d_vrth: Value,
    ) -> BjtDynamicReduction {
        reduction.vbic_transport = transport;
        reduction.vbic_d_itzf_d_vrth = d_itzf_d_vrth;
        if !self.uses_vbic_dynamic_charges() || self.td <= 0.0 {
            return reduction;
        }

        reduction.internal_voltages[IDX_VXF1] = transport.itzf;
        reduction.internal_voltages[IDX_VXF2] = transport.itzf;
        self.apply_vbic_excess_phase_transport(reduction, transport, d_itzf_d_vrth)
    }

    fn build_dynamic_reduction(&self, base: BjtReducedLinearization) -> BjtDynamicReduction {
        let reduction = self.dynamic_reduction_template(base);
        if !self.uses_vbic_dynamic_charges() {
            return reduction;
        }

        let vrth = reduction.internal_voltages[IDX_VRTH];
        let base_inputs = if self.self_heating_enabled() {
            self.temperature_variant(vrth)
                .dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages)
        } else {
            self.dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages)
        };
        let d_itzf_d_vrth = if self.self_heating_enabled() && self.td > 0.0 {
            let h = self.thermal_derivative_step(vrth);
            let mut plus_internal = reduction.internal_voltages;
            plus_internal[IDX_VRTH] = vrth + h;
            let mut minus_internal = reduction.internal_voltages;
            minus_internal[IDX_VRTH] = vrth - h;
            let plus = self
                .temperature_variant(vrth + h)
                .dynamic_charge_inputs(reduction.external_voltages, plus_internal);
            let minus = self
                .temperature_variant(vrth - h)
                .dynamic_charge_inputs(reduction.external_voltages, minus_internal);
            (plus.transport.itzf - minus.transport.itzf) / (2.0 * h)
        } else {
            0.0
        };

        self.build_dynamic_reduction_from_transport(reduction, base_inputs.transport, d_itzf_d_vrth)
    }

    fn dynamic_charge_branches_fixed_temperature(
        &self,
        reduction: &BjtDynamicReduction,
    ) -> [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT] {
        if !self.uses_vbic_dynamic_charges() {
            return [BjtChargeBranch::default(); BJT_DYNAMIC_CHARGE_COUNT];
        }

        let inputs =
            self.dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages);
        self.dynamic_charge_branches_from_inputs(reduction, inputs)
    }

    fn dynamic_charge_branches_from_inputs(
        &self,
        reduction: &BjtDynamicReduction,
        inputs: BjtDynamicChargeInputs,
    ) -> [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT] {
        let mut branches = [BjtChargeBranch::default(); BJT_DYNAMIC_CHARGE_COUNT];
        if !self.uses_vbic_dynamic_charges() {
            return branches;
        }

        let [_, vci, vbx, vbi, vei, vbp, vsi, vrth, vxf1, vxf2] = reduction.internal_voltages;
        let [vc, vb, ve, _vs] = reduction.external_voltages;
        let p = self.polarity();
        let wbe = self.wbe.clamp(0.0, 1.0);
        let transport = inputs.transport;
        let parasitic = inputs.parasitic;
        let epi = inputs.epi;
        let qdbe = inputs.qdbe;
        let dqdbe_dvbe_eff = inputs.dqdbe_dvbe_eff;
        let qdbex = inputs.qdbex;
        let dqdbex_dvbex_eff = inputs.dqdbex_dvbex_eff;
        let qdbc = inputs.qdbc;
        let dqdbc_dvbc_eff = inputs.dqdbc_dvbc_eff;
        let qdbep = inputs.qdbep;
        let dqdbep_dvbep_eff = inputs.dqdbep_dvbep_eff;
        let qdbcp = inputs.qdbcp;
        let dqdbcp_dvbcp_eff = inputs.dqdbcp_dvbcp_eff;
        let vbc_eff = p * (vbi - vci);
        let _vbex_eff = p * (vbx - vei);
        let _vbep_eff = p * (vbx - vbp);
        let vbcp_eff = p * (vsi - vbp);
        let vbeo_eff = p * (vb - ve);
        let vbco_eff = p * (vb - vc);

        let sg_if = if transport.ifi > 0.0 { 1.0 } else { 0.0 };
        let iitf = if self.itf > 0.0 { 1.0 / self.itf } else { 0.0 };
        let ivtf = if self.vtf > 0.0 { 1.0 / self.vtf } else { 0.0 };
        let sl_tf = if self.itf > 0.0 { 0.0 } else { 1.0 };
        let r_if = transport.ifi * sg_if * iitf;
        let dr_if_dvbe_eff = transport.gfi * sg_if * iitf;
        let m_if = r_if / (1.0 + r_if);
        let dm_if_dvbe_eff = dr_if_dvbe_eff / (1.0 + r_if).powi(2);
        let (bc_exp, bc_exp_slope) = Self::limited_exp(vbc_eff * ivtf / 1.44);
        let dbc_exp_dvbc_eff = bc_exp_slope * ivtf / 1.44;
        let tf_base = self.tf * (1.0 + self.qtf * transport.q1);
        let tf_mod = 1.0 + self.xtf * bc_exp * (sl_tf + m_if * m_if) * sg_if;
        let tff = tf_base * tf_mod;
        let dtff_dvbe_eff = self.tf * self.qtf * transport.dq1_dvbe_eff * tf_mod
            + tf_base * self.xtf * bc_exp * (2.0 * m_if * dm_if_dvbe_eff) * sg_if;
        let dtff_dvbc_eff = self.tf * self.qtf * transport.dq1_dvbc_eff * tf_mod
            + tf_base * self.xtf * dbc_exp_dvbc_eff * (sl_tf + m_if * m_if) * sg_if;

        let mut qbe = BjtChargeBranch {
            pos_internal: Some(IDX_VBI),
            neg_internal: Some(IDX_VEI),
            ..Default::default()
        };
        qbe.charge = self.cje * wbe * qdbe + tff * transport.ifi / transport.qb.max(1e-12);
        let qbe_tff = transport.ifi / transport.qb.max(1e-12);
        let qbe_ifi = tff / transport.qb.max(1e-12);
        let qbe_qb = -transport.ifi * tff / transport.qb.max(1e-12).powi(2);
        let dqbe_dvbe_eff = self.cje * wbe * dqdbe_dvbe_eff
            + qbe_tff * dtff_dvbe_eff
            + qbe_ifi * transport.gfi
            + qbe_qb * transport.dqb_dvbe_eff;
        let dqbe_dvbc_eff = qbe_tff * dtff_dvbc_eff + qbe_qb * transport.dqb_dvbc_eff;
        qbe.d_internal[IDX_VBI] = p * (dqbe_dvbe_eff + dqbe_dvbc_eff);
        qbe.d_internal[IDX_VEI] = -p * dqbe_dvbe_eff;
        qbe.d_internal[IDX_VCI] = -p * dqbe_dvbc_eff;
        branches[0] = qbe;

        if self.cje > 0.0 && wbe < 1.0 {
            let mut qbex = BjtChargeBranch {
                pos_internal: Some(IDX_VBX),
                neg_internal: Some(IDX_VEI),
                ..Default::default()
            };
            qbex.charge = self.cje * (1.0 - wbe) * qdbex;
            let dq_dvbex_eff = self.cje * (1.0 - wbe) * dqdbex_dvbex_eff;
            qbex.d_internal[IDX_VBX] = p * dq_dvbex_eff;
            qbex.d_internal[IDX_VEI] = -p * dq_dvbex_eff;
            branches[1] = qbex;
        }

        let mut qbc = BjtChargeBranch {
            pos_internal: Some(IDX_VBI),
            neg_internal: Some(IDX_VCI),
            ..Default::default()
        };
        qbc.charge = self.cjc * qdbc + self.tr * transport.iri + self.qco * epi.kbci;
        let dqbc_dvbc_eff = self.cjc * dqdbc_dvbc_eff + self.tr * transport.gri;
        qbc.d_internal[IDX_VBI] = p * dqbc_dvbc_eff + self.qco * epi.d_kbci[IDX_VBI];
        qbc.d_internal[IDX_VCI] = -p * dqbc_dvbc_eff + self.qco * epi.d_kbci[IDX_VCI];
        branches[2] = qbc;

        if self.qco > 0.0 {
            let mut qbcx = BjtChargeBranch {
                pos_internal: Some(IDX_VBI),
                neg_internal: Some(IDX_VCX),
                ..Default::default()
            };
            qbcx.charge = self.qco * epi.kbcx;
            for idx in 0..INTERNAL_DIM {
                qbcx.d_internal[idx] = self.qco * epi.d_kbcx[idx];
            }
            branches[3] = qbcx;
        }

        if self.cjep > 0.0 || self.tr != 0.0 {
            let mut qbep = BjtChargeBranch {
                pos_internal: Some(IDX_VBX),
                neg_internal: Some(IDX_VBP),
                ..Default::default()
            };
            qbep.charge = self.cjep * qdbep + self.tr * parasitic.ifp;
            let dq_dep = self.cjep * dqdbep_dvbep_eff;
            qbep.d_internal[IDX_VBX] = p * dq_dep + self.tr * parasitic.d_ifp[IDX_VBX];
            qbep.d_internal[IDX_VBP] = -p * dq_dep + self.tr * parasitic.d_ifp[IDX_VBP];
            qbep.d_internal[IDX_VBI] = self.tr * parasitic.d_ifp[IDX_VBI];
            qbep.d_internal[IDX_VCI] = self.tr * parasitic.d_ifp[IDX_VCI];
            branches[4] = qbep;
        }

        if self.cbeo > 0.0 {
            let mut qbeo = BjtChargeBranch {
                pos_external: Some(EXT_B),
                neg_external: Some(EXT_E),
                ..Default::default()
            };
            qbeo.charge = self.cbeo * vbeo_eff;
            qbeo.d_external[EXT_B] = p * self.cbeo;
            qbeo.d_external[EXT_E] = -p * self.cbeo;
            branches[5] = qbeo;
        }

        if self.cbco > 0.0 {
            let mut qbco = BjtChargeBranch {
                pos_external: Some(EXT_B),
                neg_external: Some(EXT_C),
                ..Default::default()
            };
            qbco.charge = self.cbco * vbco_eff;
            qbco.d_external[EXT_B] = p * self.cbco;
            qbco.d_external[EXT_C] = -p * self.cbco;
            branches[6] = qbco;
        }

        if self.cjcp > 0.0 || self.ccso > 0.0 {
            let mut qbcp = BjtChargeBranch {
                pos_internal: Some(IDX_VSI),
                neg_internal: Some(IDX_VBP),
                ..Default::default()
            };
            qbcp.charge = self.cjcp * qdbcp + self.ccso * vbcp_eff;
            let dq_dvbcp_eff = self.cjcp * dqdbcp_dvbcp_eff + self.ccso;
            qbcp.d_internal[IDX_VSI] = p * dq_dvbcp_eff;
            qbcp.d_internal[IDX_VBP] = -p * dq_dvbcp_eff;
            branches[7] = qbcp;
        }

        let cth = self.thermal_capacitance();
        if cth > 0.0 {
            let mut qcth = BjtChargeBranch {
                pos_internal: Some(IDX_VRTH),
                ..Default::default()
            };
            qcth.charge = cth * vrth;
            qcth.d_internal[IDX_VRTH] = cth;
            branches[IDX_QCTH] = qcth;
        }

        if self.td > 0.0 {
            let mut qxf1 = BjtChargeBranch {
                pos_internal: Some(IDX_VXF1),
                ..Default::default()
            };
            qxf1.charge = self.td * vxf1;
            qxf1.d_internal[IDX_VXF1] = self.td;
            branches[IDX_QXF1] = qxf1;

            let mut qxf2 = BjtChargeBranch {
                pos_internal: Some(IDX_VXF2),
                ..Default::default()
            };
            qxf2.charge = self.td * vxf2 / 3.0;
            qxf2.d_internal[IDX_VXF2] = self.td / 3.0;
            branches[IDX_QXF2] = qxf2;
        }

        branches
    }

    fn dynamic_charge_branches(
        &self,
        reduction: &BjtDynamicReduction,
    ) -> [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT] {
        let vrth = reduction.internal_voltages[IDX_VRTH];
        let mut branches = self.with_temperature_variant(vrth, |model| {
            model.dynamic_charge_branches_fixed_temperature(reduction)
        });

        if !self.self_heating_enabled() {
            return branches;
        }

        let h = self.thermal_derivative_step(vrth);
        let mut plus_reduction = *reduction;
        plus_reduction.internal_voltages[IDX_VRTH] = vrth + h;
        let mut minus_reduction = *reduction;
        minus_reduction.internal_voltages[IDX_VRTH] = vrth - h;
        let plus = self.with_temperature_variant(vrth + h, |model| {
            model.dynamic_charge_branches_fixed_temperature(&plus_reduction)
        });
        let minus = self.with_temperature_variant(vrth - h, |model| {
            model.dynamic_charge_branches_fixed_temperature(&minus_reduction)
        });
        let denom = 2.0 * h;

        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            branches[branch_idx].d_internal[IDX_VRTH] =
                (plus[branch_idx].charge - minus[branch_idx].charge) / denom;
        }

        branches
    }

    fn charge_snapshot_from_base(&self, base: BjtReducedLinearization) -> BjtChargeSnapshot {
        let template = self.dynamic_reduction_template(base);
        if !self.uses_vbic_dynamic_charges() {
            return BjtChargeSnapshot {
                reduction: template,
                branches: [BjtChargeBranch::default(); BJT_DYNAMIC_CHARGE_COUNT],
            };
        }

        let vrth = template.internal_voltages[IDX_VRTH];
        if !self.self_heating_enabled() {
            let inputs = base.cached_dynamic_inputs.unwrap_or_else(|| {
                self.dynamic_charge_inputs(template.external_voltages, template.internal_voltages)
            });
            let reduction =
                self.build_dynamic_reduction_from_transport(template, inputs.transport, 0.0);
            return BjtChargeSnapshot {
                reduction,
                branches: self.dynamic_charge_branches_from_inputs(&reduction, inputs),
            };
        }

        let h = self.thermal_derivative_step(vrth);
        let denom = 2.0 * h;

        let mut plus_internal = template.internal_voltages;
        plus_internal[IDX_VRTH] = vrth + h;
        let mut minus_internal = template.internal_voltages;
        minus_internal[IDX_VRTH] = vrth - h;

        let base_inputs = base.cached_dynamic_inputs.unwrap_or_else(|| {
            self.with_temperature_variant(vrth, |model| {
                model.dynamic_charge_inputs(template.external_voltages, template.internal_voltages)
            })
        });
        let plus_inputs = self.with_temperature_variant(vrth + h, |model| {
            model.dynamic_charge_inputs(template.external_voltages, plus_internal)
        });
        let minus_inputs = self.with_temperature_variant(vrth - h, |model| {
            model.dynamic_charge_inputs(template.external_voltages, minus_internal)
        });

        let d_itzf_d_vrth = if self.td > 0.0 {
            (plus_inputs.transport.itzf - minus_inputs.transport.itzf) / denom
        } else {
            0.0
        };
        let reduction = self.build_dynamic_reduction_from_transport(
            template,
            base_inputs.transport,
            d_itzf_d_vrth,
        );
        let mut branches = self.with_temperature_variant(vrth, |model| {
            model.dynamic_charge_branches_from_inputs(&reduction, base_inputs)
        });

        let mut plus_reduction = reduction;
        plus_reduction.internal_voltages[IDX_VRTH] = vrth + h;
        let mut minus_reduction = reduction;
        minus_reduction.internal_voltages[IDX_VRTH] = vrth - h;
        let plus_branches = self.with_temperature_variant(vrth + h, |model| {
            model.dynamic_charge_branches_from_inputs(&plus_reduction, plus_inputs)
        });
        let minus_branches = self.with_temperature_variant(vrth - h, |model| {
            model.dynamic_charge_branches_from_inputs(&minus_reduction, minus_inputs)
        });
        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            branches[branch_idx].d_internal[IDX_VRTH] =
                (plus_branches[branch_idx].charge - minus_branches[branch_idx].charge) / denom;
        }

        BjtChargeSnapshot {
            reduction,
            branches,
        }
    }

    fn dynamic_reduction_for_internal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> BjtDynamicReduction {
        let static_internal = [
            internal[IDX_VCX],
            internal[IDX_VCI],
            internal[IDX_VBX],
            internal[IDX_VBI],
            internal[IDX_VEI],
            internal[IDX_VBP],
            internal[IDX_VSI],
            internal[IDX_VRTH],
        ];
        let state = self.intrinsic_state_from_internal_vector(static_internal);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let base = self.reduced_linearization_from_state_and_eval(state, eval, vc, vb, ve, vs);
        let mut reduction = self.dynamic_reduction_template(base);
        reduction.internal_voltages = internal;

        if !self.uses_vbic_dynamic_charges() {
            return reduction;
        }

        let vrth = internal[IDX_VRTH];
        if !self.self_heating_enabled() {
            let inputs = self
                .dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages);
            let mut reduction =
                self.build_dynamic_reduction_from_transport(reduction, inputs.transport, 0.0);
            reduction.internal_voltages[IDX_VXF1] = internal[IDX_VXF1];
            reduction.internal_voltages[IDX_VXF2] = internal[IDX_VXF2];
            return reduction;
        }

        let h = self.thermal_derivative_step(vrth);
        let denom = 2.0 * h;

        let mut plus_internal = internal;
        plus_internal[IDX_VRTH] = vrth + h;
        let mut minus_internal = internal;
        minus_internal[IDX_VRTH] = vrth - h;

        let base_inputs = self.with_temperature_variant(vrth, |model| {
            model.dynamic_charge_inputs(reduction.external_voltages, internal)
        });
        let plus_inputs = self.with_temperature_variant(vrth + h, |model| {
            model.dynamic_charge_inputs(reduction.external_voltages, plus_internal)
        });
        let minus_inputs = self.with_temperature_variant(vrth - h, |model| {
            model.dynamic_charge_inputs(reduction.external_voltages, minus_internal)
        });

        let d_itzf_d_vrth = if self.td > 0.0 {
            (plus_inputs.transport.itzf - minus_inputs.transport.itzf) / denom
        } else {
            0.0
        };
        let mut reduction = self.build_dynamic_reduction_from_transport(
            reduction,
            base_inputs.transport,
            d_itzf_d_vrth,
        );
        reduction.internal_voltages[IDX_VXF1] = internal[IDX_VXF1];
        reduction.internal_voltages[IDX_VXF2] = internal[IDX_VXF2];
        reduction
    }

    pub(crate) fn charge_snapshot_for_dynamic_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> BjtChargeSnapshot {
        let reduction = self.dynamic_reduction_for_internal_state(vc, vb, ve, vs, internal);

        if !self.uses_vbic_dynamic_charges() {
            return BjtChargeSnapshot {
                reduction,
                branches: [BjtChargeBranch::default(); BJT_DYNAMIC_CHARGE_COUNT],
            };
        }

        if !self.self_heating_enabled() {
            let inputs = self
                .dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages);
            return BjtChargeSnapshot {
                reduction,
                branches: self.dynamic_charge_branches_from_inputs(&reduction, inputs),
            };
        }

        let vrth = internal[IDX_VRTH];
        let h = self.thermal_derivative_step(vrth);
        let denom = 2.0 * h;
        let mut branches = self.with_temperature_variant(vrth, |model| {
            let base_inputs = model
                .dynamic_charge_inputs(reduction.external_voltages, reduction.internal_voltages);
            model.dynamic_charge_branches_from_inputs(&reduction, base_inputs)
        });

        let mut plus_reduction = reduction;
        plus_reduction.internal_voltages[IDX_VRTH] = vrth + h;
        let mut minus_reduction = reduction;
        minus_reduction.internal_voltages[IDX_VRTH] = vrth - h;
        let plus_branches = self.with_temperature_variant(vrth + h, |model| {
            let plus_inputs = model.dynamic_charge_inputs(
                plus_reduction.external_voltages,
                plus_reduction.internal_voltages,
            );
            model.dynamic_charge_branches_from_inputs(&plus_reduction, plus_inputs)
        });
        let minus_branches = self.with_temperature_variant(vrth - h, |model| {
            let minus_inputs = model.dynamic_charge_inputs(
                minus_reduction.external_voltages,
                minus_reduction.internal_voltages,
            );
            model.dynamic_charge_branches_from_inputs(&minus_reduction, minus_inputs)
        });
        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            branches[branch_idx].d_internal[IDX_VRTH] =
                (plus_branches[branch_idx].charge - minus_branches[branch_idx].charge) / denom;
        }

        BjtChargeSnapshot {
            reduction,
            branches,
        }
    }

    pub(crate) fn charge_snapshot(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtChargeSnapshot {
        if self.charge_snapshot_cache_valid.get()
            && self.cache_matches_external_biases(vc, vb, ve, vs)
        {
            return self.charge_snapshot_cache.get();
        }

        let snapshot = self.charge_snapshot_from_base(self.reduced_linearization(vc, vb, ve, vs));
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            self.charge_snapshot_cache.set(snapshot);
            self.charge_snapshot_cache_valid.set(true);
        }
        snapshot
    }

    pub(crate) fn dynamic_internal_state_seed(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        if self.cache_matches_external_biases(vc, vb, ve, vs)
            && self.reduced_linearization_cache_valid.get()
        {
            let static_internal = self.internal_state_vector();
            let mut internal = [0.0; BJT_INTERNAL_STATE_DIM];
            internal[..INTERNAL_DIM].copy_from_slice(&static_internal);

            if self.uses_vbic_dynamic_charges() {
                let inputs = if self.self_heating_enabled() {
                    self.with_temperature_variant(static_internal[IDX_VRTH], |model| {
                        model.dynamic_charge_inputs([vc, vb, ve, vs], internal)
                    })
                } else {
                    self.dynamic_charge_inputs([vc, vb, ve, vs], internal)
                };
                internal[IDX_VXF1] = inputs.transport.itzf;
                internal[IDX_VXF2] = inputs.transport.itzf;
            }

            return internal;
        }

        self.charge_snapshot(vc, vb, ve, vs)
            .reduction
            .internal_voltages
    }

    pub(crate) fn external_terminal_currents_at_bias(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> [Value; EXTERNAL_DIM] {
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            return [self.ic, self.ib, self.ie, self.isub];
        }

        let state = self.intrinsic_state_for_biases(vc, vb, ve, vs);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let terminal = self.external_terminal_branches(eval);
        [
            terminal[EXT_C].current,
            terminal[EXT_B].current,
            terminal[EXT_E].current,
            terminal[EXT_S].current,
        ]
    }

    pub(crate) fn external_terminal_currents_for_internal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; INTERNAL_DIM],
    ) -> [Value; EXTERNAL_DIM] {
        let state = self.intrinsic_state_from_internal_vector(internal);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let terminal = self.external_terminal_branches(eval);
        [
            terminal[EXT_C].current,
            terminal[EXT_B].current,
            terminal[EXT_E].current,
            terminal[EXT_S].current,
        ]
    }

    fn solve_small_dense_system<const N: usize>(
        matrix: &[[Value; N]; N],
        rhs: &[Value; N],
        dim: usize,
    ) -> Option<[Value; N]> {
        if dim == 0 {
            return Some([0.0; N]);
        }

        let mut a = *matrix;
        let mut b = *rhs;

        for pivot in 0..dim {
            let mut best = pivot;
            let mut best_abs = a[pivot][pivot].abs();
            for row in (pivot + 1)..dim {
                let value = a[row][pivot].abs();
                if value > best_abs {
                    best = row;
                    best_abs = value;
                }
            }
            if best_abs < 1e-18 {
                return None;
            }
            if best != pivot {
                a.swap(pivot, best);
                b.swap(pivot, best);
            }

            let pivot_value = a[pivot][pivot];
            for row in (pivot + 1)..dim {
                let factor = a[row][pivot] / pivot_value;
                a[row][pivot] = 0.0;
                for col in (pivot + 1)..dim {
                    a[row][col] -= factor * a[pivot][col];
                }
                b[row] -= factor * b[pivot];
            }
        }

        let mut x = [0.0; N];
        for row in (0..dim).rev() {
            let mut sum = b[row];
            for col in (row + 1)..dim {
                sum -= a[row][col] * x[col];
            }
            let diag = a[row][row];
            if diag.abs() < 1e-18 {
                return None;
            }
            x[row] = sum / diag;
        }

        Some(x)
    }

    fn solve_intrinsic_terminal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> IntrinsicTerminalState {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let reuse_previous_state = self.reduced_linearization_cache_valid.get();
        let solve_vbp = Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0;

        let mut vcx = if reuse_previous_state {
            self.vcx
        } else if has_rcx {
            vc - self.ic * self.rcx.max(0.0)
        } else {
            vc
        };
        let mut vci = if reuse_previous_state {
            self.vci
        } else if has_rci {
            vcx - self.ic * self.rci.max(0.0)
        } else {
            vcx
        };
        let mut vbx = if reuse_previous_state {
            self.vbx
        } else if has_rbx {
            vb - self.ib * self.rbx.max(0.0)
        } else {
            vb
        };
        let mut vbi = if reuse_previous_state {
            self.vbi
        } else if has_rbi {
            vbx - self.ib * self.rbi.max(0.0)
        } else {
            vbx
        };
        let mut vei = if reuse_previous_state {
            self.vei
        } else if has_re {
            ve - self.ie * self.re.max(0.0)
        } else {
            ve
        };
        let mut vsi = if reuse_previous_state {
            self.vsi
        } else if has_rs {
            vs - self.isub * self.rs.max(0.0)
        } else {
            vs
        };
        let mut vbp = if reuse_previous_state {
            self.vbp
        } else if solve_vbp {
            vcx
        } else {
            vcx
        };
        let mut vrth = if reuse_previous_state {
            self.vrth
        } else if has_self_heat {
            let seed_internal = [vcx, vci, vbx, vbi, vei, vbp, vsi, 0.0];
            let first_guess =
                self.branchwise_thermal_rise_guess_from_internal(vc, vb, ve, vs, seed_internal);
            let second_guess = self.branchwise_thermal_rise_guess_from_internal(
                vc,
                vb,
                ve,
                vs,
                [vcx, vci, vbx, vbi, vei, vbp, vsi, first_guess],
            );
            if second_guess.is_finite() {
                second_guess
            } else if first_guess.is_finite() {
                first_guess
            } else {
                (vc * self.ic + vb * self.ib + ve * self.ie + vs * self.isub)
                    / self.thermal_conductance().max(1e-18)
            }
        } else {
            0.0
        };
        if !has_self_heat {
            vrth = 0.0;
        }

        let state = [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth];
        let previous_external = [self.vc_ext, self.vb_ext, self.ve_ext, self.vs_ext];
        let predicted_state = if reuse_previous_state {
            self.predict_intrinsic_state_from_previous_external_bias(
                previous_external,
                state,
                [vc, vb, ve, vs],
            )
        } else {
            None
        };
        let solve_from_seed = |seed: [Value; INTERNAL_DIM]| {
            if has_self_heat && !reuse_previous_state {
                self.solve_intrinsic_state_with_self_heating_continuation(vc, vb, ve, vs, seed)
            } else {
                self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, seed)
            }
        };

        let (mut best_state, mut best_residual_norm) =
            solve_from_seed(predicted_state.unwrap_or(state));
        if self.charge_model == BjtChargeModel::Vbic
            && reuse_previous_state
            && self.vbic_max_local_branch_delta(best_state, predicted_state.unwrap_or(state)) > 0.1
        {
            if let Some((continued_state, continued_residual_norm)) = self
                .solve_intrinsic_state_with_external_continuation(
                    previous_external,
                    state,
                    [vc, vb, ve, vs],
                )
            {
                if continued_residual_norm + 1e-15 < best_residual_norm
                    || self.vbic_max_local_branch_delta(
                        continued_state,
                        predicted_state.unwrap_or(state),
                    ) <= 0.1
                {
                    best_state = continued_state;
                    best_residual_norm = continued_residual_norm;
                }
            }
        }
        if predicted_state.is_some()
            && best_residual_norm > 1e-9
            && self.vbic_max_local_branch_delta(best_state, predicted_state.unwrap_or(state)) > 0.1
        {
            let (fallback_state, fallback_residual_norm) = solve_from_seed(state);
            if fallback_residual_norm + 1e-15 < best_residual_norm {
                best_state = fallback_state;
                best_residual_norm = fallback_residual_norm;
            }
        }
        if has_self_heat {
            for _ in 0..4 {
                let rebalanced_state =
                    self.rebalance_intrinsic_thermal_state(vc, vb, ve, vs, best_state);
                let (refined_state, refined_residual_norm) =
                    self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, rebalanced_state);
                if refined_residual_norm + 1e-15 < best_residual_norm {
                    best_state = refined_state;
                    best_residual_norm = refined_residual_norm;
                    continue;
                }
                break;
            }
        }

        [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = best_state;

        if !has_rcx {
            vcx = vc;
        }
        if !has_rci {
            vci = vcx;
        }
        if !has_rbx {
            vbx = vb;
        }
        if !has_rbi {
            vbi = vbx;
        }
        if !has_re {
            vei = ve;
        }
        if !has_rs {
            vsi = vs;
        }
        if !solve_vbp {
            vbp = vcx;
        }
        if !has_self_heat {
            vrth = 0.0;
        }
        let linearized = self
            .with_temperature_variant(vrth, |model| model.linearize_currents(vbi - vei, vbi - vci));

        IntrinsicTerminalState {
            vcx,
            vci,
            vbx,
            vbi,
            vei,
            vbp,
            vsi,
            vrth,
            linearized,
        }
    }

    fn internal_voltage_sensitivities(
        &self,
        state: IntrinsicTerminalState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> [[Value; EXTERNAL_DIM]; INTERNAL_DIM] {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let solve_vbp = Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0;

        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);
        let thermal_sink = self.thermal_sink_branch(state.vrth);
        let thermal_power = self.thermal_power_branch(
            eval,
            [vc, vb, ve, vs],
            [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
        );

        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut external_partials = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(eval.ircx, eval.irbp),
                if has_rci {
                    eval.irci
                } else {
                    collector_internal
                },
            );
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VCX][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VCX] = row.d_external;
        } else {
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
            external_partials[IDX_VCX][EXT_C] = -1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VCI][idx] = row.d_internal[idx];
            }
        } else {
            jacobian[IDX_VCI][IDX_VCI] = 1.0;
            jacobian[IDX_VCI][IDX_VCX] = -1.0;
        }

        if has_rbx {
            let row = Self::sub_branches(
                Self::sub_branches(
                    Self::sub_branches(eval.irbx, if has_rbi { eval.irbi } else { base_internal }),
                    eval.ibep,
                ),
                eval.iccp,
            );
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VBX][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VBX] = row.d_external;
        } else {
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
            external_partials[IDX_VBX][EXT_B] = -1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VBI][idx] = row.d_internal[idx];
            }
        } else {
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(eval.ire, emitter_internal);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VEI][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VEI] = row.d_external;
        } else {
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
            external_partials[IDX_VEI][EXT_E] = -1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VBP][idx] = row.d_internal[idx];
            }
        } else {
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VSI][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VSI] = row.d_external;
        } else {
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
            external_partials[IDX_VSI][EXT_S] = -1.0;
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VRTH][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VRTH] = row.d_external;
        } else {
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        let mut sensitivities = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        for external in 0..EXTERNAL_DIM {
            let rhs = external_partials.map(|partials| -partials[external]);
            if let Some(solution) = Self::solve_small_dense_system(&jacobian, &rhs, INTERNAL_DIM) {
                for idx in 0..INTERNAL_DIM {
                    sensitivities[idx][external] = solution[idx];
                }
            }
        }

        sensitivities
    }

    fn external_terminal_branches(
        &self,
        eval: EvaluatedBjtState,
    ) -> [BranchLinearization; EXTERNAL_DIM] {
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);

        let collector = if Self::series_active(self.rcx) {
            eval.ircx
        } else {
            Self::sub_branches(
                if Self::series_active(self.rci) {
                    eval.irci
                } else {
                    collector_internal
                },
                eval.irbp,
            )
        };
        let base = if Self::series_active(self.rbx) {
            eval.irbx
        } else {
            Self::add_branches(
                Self::add_branches(
                    if Self::series_active(self.rbi) {
                        eval.irbi
                    } else {
                        base_internal
                    },
                    eval.ibep,
                ),
                eval.iccp,
            )
        };
        let emitter = if Self::series_active(self.re) {
            eval.ire
        } else {
            emitter_internal
        };
        let substrate = if Self::series_active(self.rs) {
            eval.irs
        } else {
            Self::sub_branches(eval.ibcp, eval.iccp)
        };

        [collector, base, emitter, substrate]
    }

    fn intrinsic_branch_linearizations_for_model(
        model: &Self,
        vci: Value,
        vbi: Value,
        vei: Value,
    ) -> (
        BranchLinearization,
        BranchLinearization,
        BranchLinearization,
    ) {
        let p = model.polarity();
        let vbe = vbi - vei;
        let vbc = vbi - vci;
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;

        let ibe = model.diode_current_with_is(model.ibei, vbe_eff, model.nei)
            + model.diode_current_with_is(model.iben, vbe_eff, model.nen);
        let dibe_dvbe = model.diode_conductance_with_is(model.ibei, vbe_eff, model.nei)
            + model.diode_conductance_with_is(model.iben, vbe_eff, model.nen);
        let ibe_branch = Self::branch_from_vbe_vbc(p * ibe, dibe_dvbe, 0.0);

        let transport = model.transport_charge_state(vbe_eff, vbc_eff);
        let bc = model.base_collector_current_state(transport, vbc_eff);
        let ibc_branch = Self::branch_from_vbe_vbc(p * bc.ibc, bc.dibc_dvbe_eff, bc.dibc_dvbc_eff);

        let iciei = transport.itzf - transport.itzr;
        let diciei_dvbe = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let diciei_dvbc = transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff;
        let iciei_branch = Self::branch_from_vbe_vbc(p * iciei, diciei_dvbe, diciei_dvbc);

        (ibe_branch, ibc_branch, iciei_branch)
    }

    fn intrinsic_branch_linearizations(
        &self,
        vci: Value,
        vbi: Value,
        vei: Value,
        vrth: Value,
    ) -> (
        BranchLinearization,
        BranchLinearization,
        BranchLinearization,
    ) {
        let (mut ibe, mut ibc, mut iciei) = self.with_temperature_variant(vrth, |model| {
            Self::intrinsic_branch_linearizations_for_model(model, vci, vbi, vei)
        });

        if self.self_heating_enabled() {
            let h = self.thermal_derivative_step(vrth);
            let (ibe_plus, ibc_plus, iciei_plus) = self
                .with_temperature_variant(vrth + h, |model| {
                    Self::intrinsic_branch_linearizations_for_model(model, vci, vbi, vei)
                });
            let (ibe_minus, ibc_minus, iciei_minus) = self
                .with_temperature_variant(vrth - h, |model| {
                    Self::intrinsic_branch_linearizations_for_model(model, vci, vbi, vei)
                });
            let denom = 2.0 * h;

            ibe.d_internal[IDX_VRTH] = (ibe_plus.current - ibe_minus.current) / denom;
            ibc.d_internal[IDX_VRTH] = (ibc_plus.current - ibc_minus.current) / denom;
            iciei.d_internal[IDX_VRTH] = (iciei_plus.current - iciei_minus.current) / denom;
        }

        (ibe, ibc, iciei)
    }

    fn thermal_sink_branch(&self, vrth: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        let gth = self.thermal_conductance();
        if gth <= 0.0 {
            return branch;
        }
        branch.current = gth * vrth;
        branch.d_internal[IDX_VRTH] = gth;
        branch
    }

    fn thermal_power_branch(
        &self,
        eval: EvaluatedBjtState,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; INTERNAL_DIM],
    ) -> BranchLinearization {
        if !self.self_heating_enabled() {
            return BranchLinearization::default();
        }

        let [vcx, vci, vbx, vbi, vei, vbp, vsi, _vrth] = internal;
        let [vc, vb, ve, vs] = external;
        let zero_external = [0.0; EXTERNAL_DIM];
        let mut power = BranchLinearization::default();

        let add_power = |acc: &mut BranchLinearization,
                         current: BranchLinearization,
                         voltage: Value,
                         d_voltage_internal: [Value; INTERNAL_DIM],
                         d_voltage_external: [Value; EXTERNAL_DIM]| {
            *acc = Self::add_branches(
                *acc,
                Self::power_from_branch(current, voltage, d_voltage_internal, d_voltage_external),
            );
        };

        add_power(
            &mut power,
            eval.ibe,
            vbi - vei,
            [0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ibc,
            vbi - vci,
            [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.iciei,
            vci - vei,
            [0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ibep,
            vbx - vbp,
            [0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ibcp,
            vsi - vbp,
            [0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.iccp,
            vbx - vsi,
            [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ircx,
            vc - vcx,
            [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        );
        add_power(
            &mut power,
            eval.irci,
            vcx - vci,
            [1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.irbx,
            vb - vbx,
            [0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
        );
        add_power(
            &mut power,
            eval.irbi,
            vbx - vbi,
            [0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ire,
            ve - vei,
            [0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        );
        add_power(
            &mut power,
            eval.irbp,
            vbp - vcx,
            [-1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.irs,
            vs - vsi,
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );

        power
    }

    #[inline]
    fn branchwise_thermal_rise_guess_from_internal(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; INTERNAL_DIM],
    ) -> Value {
        let gth = self.thermal_conductance();
        if gth <= 0.0 {
            return 0.0;
        }

        let state = self.intrinsic_state_from_internal_vector(internal);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        self.thermal_power_branch(eval, [vc, vb, ve, vs], internal)
            .current
            / gth
    }

    #[inline]
    pub(crate) fn minimum_thermal_rise(&self) -> Value {
        1.0 - self.requested_temperature()
    }

    pub(crate) fn vbic_dynamic_thermal_residual_and_derivative(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> (Value, Value) {
        let static_internal = [
            internal[IDX_VCX],
            internal[IDX_VCI],
            internal[IDX_VBX],
            internal[IDX_VBI],
            internal[IDX_VEI],
            internal[IDX_VBP],
            internal[IDX_VSI],
            internal[IDX_VRTH],
        ];
        let state = self.intrinsic_state_from_internal_vector(static_internal);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let static_row = Self::sub_branches(
            self.thermal_sink_branch(state.vrth),
            self.thermal_power_branch(eval, [vc, vb, ve, vs], static_internal),
        );
        let mut residual = static_row.current;
        let mut derivative = static_row.d_internal[IDX_VRTH];

        let reduction = self.dynamic_reduction_for_internal_state(vc, vb, ve, vs, internal);
        let thermal_branch = self.vbic_delay_static_thermal_branch(&reduction);
        if thermal_branch.is_active() {
            residual += thermal_branch.current;
            derivative += thermal_branch.d_internal[IDX_VRTH];
        }

        (residual, derivative)
    }

    #[inline]
    fn internal_state_vector(&self) -> [Value; INTERNAL_DIM] {
        [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
        ]
    }

    #[inline]
    fn previous_internal_state_vector(&self) -> [Value; INTERNAL_DIM] {
        [
            self.vcx_prev,
            self.vci_prev,
            self.vbx_prev,
            self.vbi_prev,
            self.vei_prev,
            self.vbp_prev,
            self.vsi_prev,
            self.vrth_prev,
        ]
    }

    #[inline]
    fn vbic_convergence_voltage_vector_for_state(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> [Value; 9] {
        let p = self.polarity();
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, _vrth] = internal;
        [
            p * (vbi - vei),
            p * (vbx - vei),
            p * (vbi - vci),
            p * (vbi - vcx),
            p * (vbx - vbp),
            p * (vcx - vci),
            p * (vbx - vbi),
            p * (vbp - vcx),
            p * (vsi - vbp),
        ]
    }

    fn vbic_convergence_branches_for_state(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> [BranchLinearization; Self::VBIC_CONVERGENCE_BRANCH_COUNT] {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = internal;
        let eval = self.evaluate_state(0.0, 0.0, 0.0, 0.0, vcx, vci, vbx, vbi, vei, vbp, vsi, vrth);
        [
            eval.ibe, eval.ibep, eval.iciei, eval.ibc, eval.irci, eval.irbi, eval.irbp, eval.ibcp,
            eval.iccp,
        ]
    }

    pub(crate) fn vbic_transient_convergence_state_for_snapshot(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        snapshot: &BjtChargeSnapshot,
    ) -> VbicTransientConvergenceState {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth, _vxf1, _vxf2] =
            snapshot.reduction.internal_voltages;
        let eval = self.evaluate_state(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi, vrth);
        let delay_branches = self.vbic_delay_static_branches(&snapshot.reduction);

        let mut currents = [0.0; VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT];
        let mut d_currents_d_internal =
            [[0.0; BJT_INTERNAL_STATE_DIM]; VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT];

        let static_branches = [
            eval.ibe, eval.ibep, eval.iciei, eval.ibc, eval.irci, eval.irbi, eval.irbp, eval.ibcp,
            eval.iccp,
        ];
        for (branch_idx, branch) in static_branches.iter().enumerate() {
            currents[branch_idx] = branch.current;
            d_currents_d_internal[branch_idx][..INTERNAL_DIM].copy_from_slice(&branch.d_internal);
        }

        if self.uses_vbic_dynamic_charges() && self.td > 0.0 {
            currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX] += delay_branches[0].current;
            for idx in 0..BJT_INTERNAL_STATE_DIM {
                d_currents_d_internal[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX][idx] +=
                    delay_branches[0].d_internal[idx];
            }
        }

        let p = self.polarity();
        let voltages = [
            p * (vbi - vei),
            p * (vbx - vei),
            p * (vbi - vci),
            p * (vbi - vcx),
            p * (vbx - vbp),
            p * (vcx - vci),
            p * (vbx - vbi),
            p * (vbp - vcx),
            p * (vsi - vbp),
        ];

        VbicTransientConvergenceState {
            voltages,
            currents,
            d_currents_d_internal,
        }
    }

    pub(crate) fn vbic_transient_convergence_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> VbicTransientConvergenceState {
        let snapshot = self.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, internal);
        self.vbic_transient_convergence_state_for_snapshot(vc, vb, ve, vs, &snapshot)
    }

    #[inline]
    fn vbic_predicted_branch_current(
        previous: &BranchLinearization,
        delta_internal: &[Value; INTERNAL_DIM],
    ) -> Value {
        previous.current
            + previous
                .d_internal
                .iter()
                .zip(delta_internal.iter())
                .enumerate()
                .filter(|(idx, _)| *idx != IDX_VRTH)
                .map(|(_, (derivative, delta))| derivative * delta)
                .sum::<Value>()
    }

    fn vbic_is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();
        let current_tol = criteria.current_tolerance();
        let current_state = self.internal_state_vector();
        let previous_state = self.previous_internal_state_vector();
        let mut delta_internal = [0.0; INTERNAL_DIM];
        for idx in 0..INTERNAL_DIM {
            delta_internal[idx] = current_state[idx] - previous_state[idx];
        }

        let current_voltages = self.vbic_convergence_voltage_vector_for_state(current_state);
        let previous_voltages = self.vbic_convergence_voltage_vector_for_state(previous_state);
        let previous_branches = self.vbic_convergence_branches_for_state(previous_state);
        let current_branches = self.vbic_convergence_branches_for_state(current_state);
        let voltages_converged =
            current_voltages
                .iter()
                .zip(previous_voltages.iter())
                .all(|(current, previous)| {
                    let diff = (current - previous).abs();
                    let tol = reltol * current.abs().max(previous.abs()) + voltage_tol;
                    diff <= tol
                });

        let currents_converged = current_branches
            .iter()
            .zip(previous_branches.iter())
            .enumerate()
            .all(|(branch_idx, (current, previous))| {
                if self.uses_vbic_dynamic_charges()
                    && branch_idx == VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX
                {
                    return true;
                }
                let predicted = Self::vbic_predicted_branch_current(previous, &delta_internal);
                let actual = current.current;
                let tol = reltol * predicted.abs().max(actual.abs()) + current_tol;
                (predicted - actual).abs() <= tol
            });

        voltages_converged && currents_converged
    }

    fn solve_intrinsic_base_voltage(&self, vc: Value, vb: Value, ve: Value) -> Value {
        let rb = self.rb;
        if !rb.is_finite() || rb <= 0.0 {
            return vb;
        }

        let g_rb = 1.0 / rb.max(1e-12);
        let mut vbi = if self.vbi.is_finite() {
            self.vbi
        } else {
            vb - self.ib * rb
        };
        if !vbi.is_finite() {
            vbi = vb;
        }

        for _ in 0..12 {
            let linearized = self.linearize_currents(vbi - ve, vbi - vc);
            let f = linearized.ib - g_rb * (vb - vbi);
            let df = linearized.dib_dvbe + linearized.dib_dvbc + g_rb;
            if !df.is_finite() || df.abs() < 1e-18 {
                break;
            }

            let delta = (-f / df).clamp(-0.1, 0.1);
            vbi += delta;
            if delta.abs() < 1e-12 {
                break;
            }
        }

        vbi
    }

    fn small_signal_row_coefficients(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtConductanceMatrix {
        self.reduced_linearization(vc, vb, ve, vs).g_reduced
    }

    pub(crate) fn stamped_reduced_external_system(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (BjtConductanceMatrix, [Value; EXTERNAL_DIM]) {
        let rows = self.small_signal_row_coefficients(vc, vb, ve, vs);
        let biases = [vc, vb, ve, vs];
        let currents = self.external_terminal_currents_at_bias(vc, vb, ve, vs);
        let mut rhs = [0.0; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            rhs[row] = -currents[row];
            for col in 0..EXTERNAL_DIM {
                rhs[row] += rows[row][col] * biases[col];
            }
        }
        (rows, rhs)
    }

    pub(crate) fn stamp_small_signal_ac(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let rows = self.small_signal_row_coefficients(vc, vb, ve, vs);
        let nodes = self.external_terminal_nodes();
        for row_idx in 0..EXTERNAL_DIM {
            for col_idx in 0..EXTERNAL_DIM {
                matrix.stamp(nodes[row_idx], nodes[col_idx], rows[row_idx][col_idx]);
            }
        }
    }

    /// Calculate BJT currents using Ebers-Moll with Gummel-Poon enhancements
    ///
    /// Base model is Ebers-Moll for stability. Early voltage and high-injection
    /// effects are applied via go() output conductance and base charge modulation.
    #[allow(dead_code)]
    fn calculate_currents(&self, vbe: Value, vbc: Value) -> (Value, Value, Value) {
        let linearized = self.linearize_currents(vbe, vbc);
        let ie = -(linearized.ic + linearized.ib);
        (linearized.ic, linearized.ib, ie)
    }

    /// Get transconductance gm = dIc/dVbe with Gummel-Poon high-injection
    ///
    /// Includes the reduction in gm at high currents due to high-injection.
    fn gm(&self, vbe: Value) -> Value {
        let p = self.polarity();
        let vbe_eff = p * vbe;

        // Base diode conductance
        let g_diode = self.diode_conductance(vbe_eff, self.nf);
        let if_diode = self.diode_current(vbe_eff, self.nf);

        // High-injection correction factor.
        // IKF<=0 disables high-injection rolloff per SPICE semantics.
        let hf = if self.ikf > 0.0 {
            let ikf_ratio = if_diode.max(0.0) / self.ikf;
            1.0 / (1.0 + ikf_ratio)
        } else {
            1.0
        };

        // At low currents: gm ≈ g_diode
        // At high currents: gm ≈ g_diode * hf (reduced)
        // Apply minimum conductance floor for numerical stability
        (g_diode * hf).max(1e-15)
    }

    /// Get output conductance go = dIc/dVce (Early effect)
    #[allow(dead_code)]
    fn go(&self, ic: Value) -> Value {
        if self.vaf.is_finite() {
            ic.abs() / self.vaf
        } else {
            1e-12 // Minimum conductance
        }
    }

    /// Get base-emitter junction conductance
    /// Includes minimum conductance floor for numerical stability
    fn gbe(&self, vbe: Value) -> Value {
        let vp = self.polarity() * vbe;
        let g = self.diode_conductance_with_is(self.ibei, vp, self.nei)
            + self.diode_conductance_with_is(self.iben, vp, self.nen);
        g.max(1e-15) // Minimum floor prevents singular matrix
    }

    /// Get base-collector junction conductance
    /// Includes minimum conductance floor for numerical stability
    fn gbc(&self, vbc: Value) -> Value {
        let vp = self.polarity() * vbc;
        let g = self.diode_conductance_with_is(self.ibci, vp, self.nci)
            + self.diode_conductance_with_is(self.ibcn, vp, self.ncn);
        g.max(1e-15) // Minimum floor prevents singular matrix
    }

    /// Junction voltage limiting (Nagel's algorithm from SPICE)
    ///
    /// This is critical for Newton-Raphson convergence with BJTs. The exponential
    /// I-V characteristic means that large voltage changes can cause currents to
    /// blow up, diverging NR. This function limits how much a junction voltage
    /// can change between iterations.
    ///
    /// Algorithm from: L.W. Nagel, "SPICE2: A Computer Program to Simulate
    /// Semiconductor Circuits", UCB/ERL M520, 1975
    ///
    /// Used by commercial simulators: Spectre, HSPICE, PSpice, etc.
    fn limit_junction_voltage(vnew: Value, vold: Value, vt: Value, vcrit: Value) -> Value {
        let vt = vt.max(1e-18);
        if !vnew.is_finite() {
            return vold;
        }
        if !vold.is_finite() {
            return vnew;
        }

        if vnew > vcrit && (vnew - vold).abs() > 2.0 * vt {
            if vold > 0.0 {
                let arg = (vnew - vold) / vt;
                if arg > 0.0 {
                    vold + vt * (2.0 + (arg - 2.0).max(1e-18).ln())
                } else {
                    vold - vt * (2.0 + (2.0 - arg).max(1e-18).ln())
                }
            } else {
                vt * (vnew / vt).max(1e-18).ln()
            }
        } else if vnew < 0.0 {
            let arg = if vold > 0.0 {
                -vold - 1.0
            } else {
                2.0 * vold - 1.0
            };
            if vnew < arg { arg } else { vnew }
        } else {
            vnew
        }
    }
}

impl NonlinearDevice for Bjt {
    fn update(&mut self, voltages: &[Value]) {
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let previous_internal_state = self.internal_state_vector();

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

        let state = if self.charge_model == BjtChargeModel::Vbic {
            self.solve_intrinsic_terminal_state(vc, vb, ve, vs)
        } else {
            self.solve_intrinsic_terminal_state(vc, vb, ve, vs)
        };
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
        let biases = self.external_terminal_voltages(voltages);
        let rows = self.small_signal_row_coefficients(
            biases[EXT_C],
            biases[EXT_B],
            biases[EXT_E],
            biases[EXT_S],
        );
        let nodes = self.external_terminal_nodes();
        let currents = [self.ic, self.ib, self.ie, self.isub];

        for row_idx in 0..EXTERNAL_DIM {
            let ieq = currents[row_idx]
                - (0..EXTERNAL_DIM)
                    .map(|col_idx| rows[row_idx][col_idx] * biases[col_idx])
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

        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();
        let current_tol = criteria.current_tolerance();
        let voltages_converged = [
            (self.vbe, self.vbe_prev),
            (self.vbc, self.vbc_prev),
            (self.vcx, self.vcx_prev),
            (self.vci, self.vci_prev),
            (self.vbx, self.vbx_prev),
            (self.vbi, self.vbi_prev),
            (self.vei, self.vei_prev),
            (self.vbp, self.vbp_prev),
            (self.vsi, self.vsi_prev),
        ]
        .into_iter()
        .all(|(current, previous)| {
            let diff = (current - previous).abs();
            let tol = reltol * current.abs().max(previous.abs()) + voltage_tol;
            diff < tol
        });
        let currents_converged = [
            (self.ic, self.ic_prev),
            (self.ib, self.ib_prev),
            (self.ie, self.ie_prev),
            (self.isub, self.isub_prev),
        ]
        .into_iter()
        .all(|(current, previous)| {
            let diff = (current - previous).abs();
            let tol = reltol * current.abs().max(previous.abs()) + current_tol;
            diff < tol
        });

        voltages_converged && currents_converged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct CaptureMatrix {
        entries: HashMap<(NodeId, NodeId), Value>,
        rhs: HashMap<NodeId, Value>,
    }

    impl CaptureMatrix {
        fn g(&self, row: NodeId, col: NodeId) -> Value {
            *self.entries.get(&(row, col)).unwrap_or(&0.0)
        }

        fn i(&self, node: NodeId) -> Value {
            *self.rhs.get(&node).unwrap_or(&0.0)
        }
    }

    impl MatrixStamper for CaptureMatrix {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            *self.entries.entry((row, col)).or_insert(0.0) += value;
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            *self.rhs.entry(index).or_insert(0.0) += value;
        }
    }

    // =========================================================================
    // Creation and Configuration Tests
    // =========================================================================

    #[test]
    fn test_bjt_creation() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        assert_eq!(q.bjt_type, BjtType::Npn);
        assert_eq!(q.node_collector, 2);
        assert_eq!(q.node_base, 1);
        assert_eq!(q.node_emitter, 0);
    }

    #[test]
    fn test_pnp_polarity() {
        let npn = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        let pnp = Bjt::new_pnp("Q2".to_string(), 2, 1, 0);

        assert_eq!(npn.polarity(), 1.0);
        assert_eq!(pnp.polarity(), -1.0);
    }

    #[test]
    fn test_bjt_default_params() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // 2N2222-like defaults
        assert!((q.is - 1e-14).abs() < 1e-15);
        assert!((q.bf - 200.0).abs() < 1.0);
        assert!((q.br - 1.0).abs() < 0.1);
        assert!((q.vt - 0.02585).abs() < 0.001);
        assert!((q.vaf - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_bjt_default_tnom_matches_spice_nominal_temperature() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        assert!(
            (q.tnom - crate::analysis::temperature::T_NOMINAL).abs() < 1e-12,
            "default BJT TNOM should track SPICE 27C nominal temperature"
        );
    }

    #[test]
    fn test_bjt_gummel_poon_params() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("CJE".to_string(), 2e-12);
        params.insert("CJC".to_string(), 1e-12);
        params.insert("TF".to_string(), 1e-9);
        params.insert("TR".to_string(), 10e-9);
        params.insert("IKF".to_string(), 0.05);

        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);

        assert_eq!(q.cje, 2e-12);
        assert_eq!(q.cjc, 1e-12);
        assert_eq!(q.tf, 1e-9);
        assert_eq!(q.tr, 10e-9);
        assert_eq!(q.ikf, 0.05);
    }

    #[test]
    fn test_bjt_with_params_uses_ngspice_legacy_model_defaults() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&HashMap::new());

        assert_eq!(q.charge_model, BjtChargeModel::LegacyGummelPoon);
        assert!((q.is - 1e-16).abs() < 1e-24);
        assert_eq!(q.bf, 100.0);
        assert_eq!(q.rbx, 0.0);
        assert_eq!(q.rcx, 0.0);
        assert_eq!(q.re, 0.0);
        assert_eq!(q.cje, 0.0);
        assert_eq!(q.cjc, 0.0);
        assert_eq!(q.tf, 0.0);
        assert_eq!(q.tr, 0.0);
        assert_eq!(q.ikf, 0.0);
        assert_eq!(q.ikr, 0.0);
    }

    #[test]
    fn test_bjt_vbic_params_select_vbic_charge_model() {
        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&vbic_reference_params());
        assert_eq!(q.charge_model, BjtChargeModel::Vbic);
    }

    #[test]
    fn test_bjt_with_params_uses_ngspice_vbic_model_defaults() {
        let mut params = HashMap::new();
        params.insert("LEVEL".to_string(), 4.0);

        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);

        assert_eq!(q.charge_model, BjtChargeModel::Vbic);
        assert!((q.is - 1e-16).abs() < 1e-24);
        assert_eq!(q.rbx, 0.0);
        assert_eq!(q.rbi, 0.1);
        assert_eq!(q.rcx, 0.0);
        assert_eq!(q.rci, 0.1);
        assert_eq!(q.re, 0.0);
        assert_eq!(q.rbp, 0.1);
        assert_eq!(q.vaf, 0.0);
        assert_eq!(q.var, 0.0);
        assert_eq!(q.cje, 0.0);
        assert_eq!(q.cjc, 0.0);
        assert_eq!(q.tf, 0.0);
        assert_eq!(q.tr, 0.0);
        assert_eq!(q.ibei_nominal, 1e-18);
        assert_eq!(q.ibci_nominal, 1e-16);
        assert_eq!(q.isrr_nominal, 1.0);
        assert_eq!(q.selft, 0.0);
    }

    #[test]
    fn test_bjt_early_voltage_aliases_follow_spice_model_cards() {
        let mut params = HashMap::new();
        params.insert("VA".to_string(), 50.0);
        params.insert("VB".to_string(), 30.0);

        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);
        assert!((q.vaf - 50.0).abs() < 1e-12);
        assert!((q.var - 30.0).abs() < 1e-12);
    }

    #[test]
    fn test_bjt_noise_params_and_helpers() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("KF".to_string(), 2e-14);
        params.insert("AF".to_string(), 1.4);
        params.insert("EF".to_string(), 1.2);

        let mut q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);
        q.update(&[0.7, 5.0]);

        let (kf, af, ef) = q
            .flicker_noise_coefficients()
            .expect("KF should enable flicker noise");
        assert!((kf - 2e-14).abs() < 1e-24);
        assert!((af - 1.4).abs() < 1e-12);
        assert!((ef - 1.2).abs() < 1e-12);

        let (ic, ibe, ibc) = q.noise_branch_currents();
        assert!(ic > 0.0);
        assert!(ibe > 0.0);
        assert!(ibc >= 0.0);
    }

    #[test]
    fn test_bjt_vbic_extrinsic_caps_are_accumulated() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("CJE".to_string(), 1e-13);
        params.insert("CJEP".to_string(), 2e-13);
        params.insert("CJC".to_string(), 3e-13);
        params.insert("CJCP".to_string(), 4e-13);

        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);
        assert!((q.cje - 1e-13).abs() < 1e-20);
        assert!((q.cjc - 3e-13).abs() < 1e-20);
        assert!((q.cjcp - 4e-13).abs() < 1e-20);
    }

    #[test]
    fn test_bjt_vbic_base_recombination_terms_raise_base_current() {
        use std::collections::HashMap;

        let mut base = HashMap::new();
        base.insert("IS".to_string(), 1e-16);
        base.insert("BF".to_string(), 200.0);

        let mut vbic = base.clone();
        vbic.insert("IBEI".to_string(), 1e-18);
        vbic.insert("IBEN".to_string(), 5e-15);
        vbic.insert("NEN".to_string(), 2.0);

        let q_base = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&base);
        let q_vbic = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&vbic);

        let (_, ib_base, _) = q_base.calculate_currents(0.35, -0.5);
        let (_, ib_vbic, _) = q_vbic.calculate_currents(0.35, -0.5);

        assert!(ib_vbic > ib_base * 4.0);
    }

    fn vbic_reference_params() -> HashMap<String, Value> {
        let mut params = HashMap::new();
        params.insert("TNOM".to_string(), 300.15);
        params.insert("IS".to_string(), 1e-16);
        params.insert("IBEI".to_string(), 1e-18);
        params.insert("IBEN".to_string(), 5e-15);
        params.insert("IBCI".to_string(), 2e-17);
        params.insert("IBCN".to_string(), 5e-15);
        params.insert("ISP".to_string(), 1e-15);
        params.insert("RCX".to_string(), 10.0);
        params.insert("RCI".to_string(), 60.0);
        params.insert("RBX".to_string(), 10.0);
        params.insert("RBI".to_string(), 40.0);
        params.insert("RE".to_string(), 2.0);
        params.insert("RS".to_string(), 20.0);
        params.insert("RBP".to_string(), 40.0);
        params.insert("VEF".to_string(), 10.0);
        params.insert("VER".to_string(), 4.0);
        params.insert("IKF".to_string(), 2e-3);
        params.insert("IKR".to_string(), 2e-4);
        params.insert("IKP".to_string(), 2e-4);
        params.insert("CJE".to_string(), 1e-13);
        params.insert("CJC".to_string(), 2e-14);
        params.insert("CJEP".to_string(), 1e-13);
        params.insert("CJCP".to_string(), 4e-13);
        params.insert("VO".to_string(), 2.0);
        params.insert("GAMM".to_string(), 2e-11);
        params.insert("HRCF".to_string(), 2.0);
        params.insert("AVC1".to_string(), 2.0);
        params.insert("AVC2".to_string(), 15.0);
        params.insert("ITF".to_string(), 8e-2);
        params.insert("XTF".to_string(), 20.0);
        params.insert("QCO".to_string(), 1e-12);
        params.insert("TF".to_string(), 10e-12);
        params.insert("TR".to_string(), 100e-12);
        params.insert("TD".to_string(), 2e-11);
        params
    }

    fn assert_relative_within(actual: Value, expected: Value, rel_tol: Value, label: &str) {
        let scale = expected.abs().max(1e-18);
        let rel_err = (actual - expected).abs() / scale;
        assert!(
            rel_err <= rel_tol,
            "{label}: expected {expected:.12e}, got {actual:.12e}, rel_err={rel_err:.3e}"
        );
    }

    fn assert_relative_or_absolute_within(
        actual: Value,
        expected: Value,
        rel_tol: Value,
        abs_tol: Value,
        label: &str,
    ) {
        let abs_err = (actual - expected).abs();
        if abs_err <= abs_tol {
            return;
        }
        let scale = expected.abs().max(actual.abs()).max(abs_tol).max(1e-18);
        let rel_err = abs_err / scale;
        assert!(
            rel_err <= rel_tol,
            "{label}: expected {expected:.12e}, got {actual:.12e}, abs_err={abs_err:.3e}, rel_err={rel_err:.3e}"
        );
    }

    #[test]
    fn test_bjt_vbic_collector_avalanche_derivatives_match_finite_difference() {
        let params = vbic_reference_params();
        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let vbe_eff = 0.73;
        let vbc_eff = 0.18;
        let eps = 1e-6;

        let transport = q.transport_charge_state(vbe_eff, vbc_eff);
        let state = q.base_collector_current_state(transport, vbc_eff);

        let plus_vbe = q.base_collector_current_state(
            q.transport_charge_state(vbe_eff + eps, vbc_eff),
            vbc_eff,
        );
        let minus_vbe = q.base_collector_current_state(
            q.transport_charge_state(vbe_eff - eps, vbc_eff),
            vbc_eff,
        );
        let numerical_dvbe = (plus_vbe.ibc - minus_vbe.ibc) / (2.0 * eps);

        let plus_vbc = q.base_collector_current_state(
            q.transport_charge_state(vbe_eff, vbc_eff + eps),
            vbc_eff + eps,
        );
        let minus_vbc = q.base_collector_current_state(
            q.transport_charge_state(vbe_eff, vbc_eff - eps),
            vbc_eff - eps,
        );
        let numerical_dvbc = (plus_vbc.ibc - minus_vbc.ibc) / (2.0 * eps);

        assert_relative_within(
            state.dibc_dvbe_eff,
            numerical_dvbe,
            3e-3,
            "VBIC weak avalanche dIbc/dVbe",
        );
        assert_relative_within(
            state.dibc_dvbc_eff,
            numerical_dvbc,
            3e-3,
            "VBIC weak avalanche dIbc/dVbc",
        );
    }

    #[test]
    fn test_bjt_vbic_gummel_reference_points_track_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_pnp("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);

        q.update(&[0.3, 0.0, 0.0]);
        let ic_03 = q.node_current(q.node_collector);
        let ib_03 = q.node_current(q.node_base);
        assert_relative_within(ic_03.abs(), 1.007807e-11, 0.08, "FG ic @ 0.3V");
        assert_relative_within(ib_03.abs(), 1.814379e-12, 0.08, "FG ib @ 0.3V");

        q.update(&[0.7, 0.0, 0.0]);
        let ic_07 = q.node_current(q.node_collector);
        let ib_07 = q.node_current(q.node_base);
        assert_relative_within(ic_07.abs(), 4.491451e-05, 0.05, "FG ic @ 0.7V");
        assert_relative_within(ib_07.abs(), 5.682497e-07, 0.05, "FG ib @ 0.7V");
    }

    #[test]
    fn test_bjt_vbic_gummel_sensitive_base_current_window_tracks_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_pnp("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);

        for (ve, expected_ib) in [
            (0.51, 4.616036e-10),
            (0.52, 6.546910e-10),
            (0.53, 9.336370e-10),
        ] {
            q.update(&[ve, 0.0, 0.0]);
            let ib = q.node_current(q.node_base).abs();
            assert_relative_within(ib, expected_ib, 1e-2, &format!("FG ib @ {ve:.2}V"));
        }
    }

    #[test]
    fn test_bjt_vbic_output_reference_point_tracks_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);

        q.update(&[0.0, 0.7, 0.0]);
        let ic = q.node_current(q.node_collector);
        let ib = q.node_current(q.node_base);
        assert_relative_within(ic.abs(), 9.59413e-06, 0.08, "FO ic @ vb=0.7 vc=0.0");
        assert_relative_within(ib.abs(), 2.463096e-04, 0.05, "FO ib @ vb=0.7 vc=0.0");
    }

    #[test]
    fn test_bjt_vbic_isrr_temperature_scaling_uses_only_dear() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(423.15);

        assert_relative_within(
            q.isrr,
            1.0,
            1e-12,
            "default ISRR should remain nominal when DEAR and XISR are zero",
        );
    }

    #[test]
    fn test_bjt_vbic_output_sensitive_base_current_window_tracks_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);

        for (vc, expected_ib) in [
            (0.10, 1.218731e-05),
            (0.15, 2.335858e-06),
            (0.20, 8.262651e-07),
        ] {
            q.update(&[0.0, 0.7, vc]);
            let ib = q.node_current(q.node_base).abs();
            assert_relative_within(ib, expected_ib, 1e-2, &format!("FO ib @ vc={vc:.2}V"));
        }
    }

    #[test]
    fn test_bjt_vbic_output_high_voltage_base_current_zero_crossing_tracks_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);

        q.update(&[0.0, 0.8, 4.0]);
        let ib = q.node_current(q.node_base).abs();
        // Around the FO zero-crossing, tiny absolute current differences map to
        // large relative error. Enforce both relative and absolute envelopes.
        assert_relative_or_absolute_within(
            ib,
            7.82896e-09,
            6.0e-2,
            5.0e-10,
            "FO ib @ vb=0.8 vc=4.0",
        );
    }

    #[test]
    #[ignore]
    fn debug_bjt_vbic_output_high_voltage_zero_crossing_breakdown() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);
        q.update(&[0.0, 0.8, 4.0]);

        let state = q.intrinsic_state_for_biases(4.0, 0.8, 0.0, 0.0);
        let eval = q.evaluate_state(
            4.0, 0.8, 0.0, 0.0, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let terminal = q.external_terminal_branches(eval);
        let p = q.polarity();
        let vbe_eff = p * (state.vbi - state.vei);
        let vbc_eff = p * (state.vbi - state.vci);
        let transport = q.transport_charge_state(vbe_eff, vbc_eff);
        let bc = q.base_collector_current_state(transport, vbc_eff);

        eprintln!(
            "state vcx={:.12e} vci={:.12e} vbx={:.12e} vbi={:.12e} vei={:.12e} vbp={:.12e} vsi={:.12e}",
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi
        );
        eprintln!("vbe_eff={vbe_eff:.12e} vbc_eff={vbc_eff:.12e}");
        eprintln!(
            "transport ifi={:.12e} iri={:.12e} q1={:.12e} qb={:.12e} itzf={:.12e} itzr={:.12e}",
            transport.ifi,
            transport.iri,
            transport.q1,
            transport.qb,
            transport.itzf,
            transport.itzr
        );
        eprintln!(
            "bc ibc={:.12e} dibc_dvbe={:.12e} dibc_dvbc={:.12e}",
            bc.ibc, bc.dibc_dvbe_eff, bc.dibc_dvbc_eff
        );
        eprintln!(
            "linearized ic={:.12e} ib={:.12e}",
            eval.linearized.ic, eval.linearized.ib
        );
        eprintln!(
            "branches ircx={:.12e} irci={:.12e} irbx={:.12e} irbi={:.12e} ire={:.12e} ibep={:.12e} irbp={:.12e} ibcp={:.12e} iccp={:.12e} irs={:.12e}",
            eval.ircx.current,
            eval.irci.current,
            eval.irbx.current,
            eval.irbi.current,
            eval.ire.current,
            eval.ibep.current,
            eval.irbp.current,
            eval.ibcp.current,
            eval.iccp.current,
            eval.irs.current,
        );
        eprintln!(
            "terminal ic={:.12e} ib={:.12e} ie={:.12e} isub={:.12e}",
            terminal[EXT_C].current,
            terminal[EXT_B].current,
            terminal[EXT_E].current,
            terminal[EXT_S].current
        );
    }

    #[test]
    #[ignore]
    fn debug_bjt_vbic_output_high_voltage_reference_breakdown() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);
        q.update(&[0.0, 0.75, 4.1]);

        let state = q.intrinsic_state_for_biases(4.1, 0.75, 0.0, 0.0);
        let eval = q.evaluate_state(
            4.1, 0.75, 0.0, 0.0, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let terminal = q.external_terminal_branches(eval);
        let (ibe, ibc, iciei) =
            q.intrinsic_branch_linearizations(state.vci, state.vbi, state.vei, state.vrth);
        let thermal_power = q.thermal_power_branch(
            eval,
            [4.1, 0.75, 0.0, 0.0],
            [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
        );

        eprintln!(
            "state vcx={:.12e} vci={:.12e} vbx={:.12e} vbi={:.12e} vei={:.12e} vbp={:.12e} vsi={:.12e} vrth={:.12e}",
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth
        );
        eprintln!(
            "intrinsic branches ibe={:.12e} ibc={:.12e} iciei={:.12e}",
            ibe.current, ibc.current, iciei.current
        );
        eprintln!(
            "series/parasitic ircx={:.12e} irci={:.12e} irbx={:.12e} irbi={:.12e} ire={:.12e} ibep={:.12e} irbp={:.12e} ibcp={:.12e} iccp={:.12e} irs={:.12e}",
            eval.ircx.current,
            eval.irci.current,
            eval.irbx.current,
            eval.irbi.current,
            eval.ire.current,
            eval.ibep.current,
            eval.irbp.current,
            eval.ibcp.current,
            eval.iccp.current,
            eval.irs.current,
        );
        eprintln!(
            "terminal ic={:.12e} ib={:.12e} ie={:.12e} isub={:.12e}",
            terminal[EXT_C].current,
            terminal[EXT_B].current,
            terminal[EXT_E].current,
            terminal[EXT_S].current
        );
        eprintln!(
            "thermal power={:.12e} sink={:.12e} terminal_power={:.12e}",
            thermal_power.current,
            q.thermal_sink_branch(state.vrth).current,
            4.1 * terminal[EXT_C].current + 0.75 * terminal[EXT_B].current
        );

        let mut no_avalanche = q.clone();
        no_avalanche.avc1 = 0.0;
        no_avalanche.update(&[0.0, 0.75, 4.1]);
        eprintln!("collector current without AVC1={:.12e}", no_avalanche.ic);

        let mut no_rci = q.clone();
        no_rci.rci = 0.0;
        no_rci.update(&[0.0, 0.75, 4.1]);
        eprintln!("collector current without RCI={:.12e}", no_rci.ic);
    }

    #[test]
    fn test_bjt_vbic_circuit_output_reference_point_tracks_ngspice_without_explicit_tnom() {
        let netlist = crate::Netlist::parse(
            r#"VBIC Output Reference
V1 V1_P V1_N 0.0
VB V1_N 0 0.7
VC Q1_C 0 0.0
Q1 Q1_C V1_P 0 N1
.OP
.MODEL N1 NPN LEVEL=4
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300
.end
"#,
        )
        .expect("parse VBIC FO reference deck");
        let mut config = crate::engine::SimulationConfig::default();
        config.temperature = 300.15;
        let engine = crate::Engine::new(config);
        let result = engine.run_dc_op(&netlist).expect("dc op");

        let vc_idx = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("VC"))
            .expect("VC branch");
        let vb_idx = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("VB"))
            .expect("VB branch");
        let ic = result.branch_currents[vc_idx].abs();
        let ib = result.branch_currents[vb_idx].abs();

        assert_relative_within(ic, 9.59413e-06, 1e-2, "FO circuit ic @ vb=0.7 vc=0.0");
        assert_relative_within(ib, 2.463096e-04, 1e-2, "FO circuit ib @ vb=0.7 vc=0.0");
    }

    #[test]
    fn test_bjt_vbic_high_voltage_rth_without_selft_tracks_ngspice_reference() {
        let netlist = crate::Netlist::parse(
            r#"VBIC Output High-Voltage Reference
V1 V1_P V1_N 0.0
VB V1_N 0 0.75
VC Q1_C 0 4.1
Q1 Q1_C V1_P 0 N1
.OP
.MODEL N1 NPN LEVEL=4
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300
.end
"#,
        )
        .expect("parse VBIC FO high-voltage deck");
        let mut config = crate::engine::SimulationConfig::default();
        config.temperature = 300.15;
        let engine = crate::Engine::new(config);
        let result = engine.run_dc_op(&netlist).expect("dc op");

        let vc_idx = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("VC"))
            .expect("VC branch");
        let vb_idx = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("VB"))
            .expect("VB branch");
        let ic = result.branch_currents[vc_idx].abs();
        let ib = result.branch_currents[vb_idx].abs();

        assert_relative_within(ic, 3.302414e-04, 1e-2, "FO circuit ic @ vb=0.75 vc=4.1");
        assert_relative_within(ib, 3.72458e-06, 1e-2, "FO circuit ib @ vb=0.75 vc=4.1");
    }

    #[test]
    fn test_bjt_vbic_thermal_power_matches_branch_dissipation_not_terminal_power() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.update(&[0.0, 0.75, 4.1]);

        let state = q.intrinsic_state_for_biases(4.1, 0.75, 0.0, 0.0);
        let eval = q.evaluate_state(
            4.1, 0.75, 0.0, 0.0, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let power = q.thermal_power_branch(
            eval,
            [4.1, 0.75, 0.0, 0.0],
            [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
        );
        let sink = q.thermal_sink_branch(state.vrth);
        let terminal = q.external_terminal_branches(eval);
        let terminal_power = 4.1 * terminal[EXT_C].current + 0.75 * terminal[EXT_B].current;

        assert_relative_within(
            power.current,
            sink.current,
            1e-9,
            "branchwise thermal power should satisfy the thermal balance",
        );
        assert!(
            (power.current - terminal_power).abs() > 1e-7,
            "branchwise thermal power should differ materially from terminal-power shortcut: branchwise={:.12e} terminal={:.12e}",
            power.current,
            terminal_power
        );
    }

    #[test]
    fn test_bjt_vbic_temperature_reference_points_track_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(423.15);

        q.update(&[0.0, 0.3, 0.3]);
        let ic_03 = q.node_current(q.node_collector);
        let ib_03 = q.node_current(q.node_base);
        assert_relative_within(ic_03.abs(), 2.829459e-07, 0.03, "temp ic @ 0.3V");
        assert_relative_within(ib_03.abs(), 4.160429e-09, 0.05, "temp ib @ 0.3V");

        q.update(&[0.0, 0.7, 0.7]);
        let ic_07 = q.node_current(q.node_collector);
        let ib_07 = q.node_current(q.node_base);
        assert_relative_within(ic_07.abs(), 3.955907e-03, 0.03, "temp ic @ 0.7V");
        assert_relative_within(ib_07.abs(), 1.316270e-04, 0.05, "temp ib @ 0.7V");
    }

    #[test]
    fn test_bjt_vbic_temperature_sensitive_collector_window_tracks_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(423.15);

        for (vdrive, expected_ic) in [
            (0.52, 1.036773e-04),
            (0.53, 1.337890e-04),
            (0.54, 1.719999e-04),
        ] {
            q.update(&[0.0, vdrive, vdrive]);
            let ic = q.node_current(q.node_collector).abs();
            assert_relative_within(ic, expected_ic, 1e-2, &format!("temp ic @ {vdrive:.2}V"));
        }
    }

    fn reduced_dynamic_conductance(
        snapshot: &BjtChargeSnapshot,
    ) -> [[Value; EXTERNAL_DIM]; EXTERNAL_DIM] {
        let mut reduced = [[0.0; EXTERNAL_DIM]; EXTERNAL_DIM];
        for col in 0..EXTERNAL_DIM {
            let mut rhs = [0.0; BJT_INTERNAL_STATE_DIM];
            for row in 0..BJT_INTERNAL_STATE_DIM {
                rhs[row] = -snapshot.reduction.g_ie[row][col];
            }
            let solution = Bjt::solve_small_dense_system(
                &snapshot.reduction.g_ii,
                &rhs,
                BJT_INTERNAL_STATE_DIM,
            )
            .expect("dynamic reduction should remain solvable");
            for row in 0..EXTERNAL_DIM {
                let mut value = snapshot.reduction.g_ee[row][col];
                for internal in 0..BJT_INTERNAL_STATE_DIM {
                    value += snapshot.reduction.g_ei[row][internal] * solution[internal];
                }
                reduced[row][col] = value;
            }
        }
        reduced
    }

    #[test]
    fn test_bjt_vbic_dynamic_reduction_preserves_dc_conductance_when_td_is_active() {
        let params = vbic_reference_params();
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);
        let (vc, vb, ve, vs) = (1.2, 0.82, 0.0, 0.4);

        let base = q.reduced_linearization(vc, vb, ve, vs);
        let snapshot = q.charge_snapshot(vc, vb, ve, vs);
        let dynamic = reduced_dynamic_conductance(&snapshot);

        for row in 0..EXTERNAL_DIM {
            for col in 0..EXTERNAL_DIM {
                let scale = base.g_reduced[row][col].abs().max(1e-12);
                let rel_err = (dynamic[row][col] - base.g_reduced[row][col]).abs() / scale;
                assert!(
                    rel_err < 1e-9,
                    "dynamic DC reduction mismatch row={row} col={col}: dynamic={:.12e} base={:.12e} rel_err={rel_err:.3e}",
                    dynamic[row][col],
                    base.g_reduced[row][col]
                );
            }
        }
    }

    #[test]
    fn test_bjt_vbic_dynamic_reduction_preserves_dc_conductance_with_self_heating() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);
        let (vc, vb, ve, vs) = (1.2, 0.82, 0.0, 0.4);

        let base = q.reduced_linearization(vc, vb, ve, vs);
        let snapshot = q.charge_snapshot(vc, vb, ve, vs);
        let dynamic = reduced_dynamic_conductance(&snapshot);

        for row in 0..EXTERNAL_DIM {
            for col in 0..EXTERNAL_DIM {
                let scale = base.g_reduced[row][col].abs().max(1e-12);
                let rel_err = (dynamic[row][col] - base.g_reduced[row][col]).abs() / scale;
                assert!(
                    rel_err < 1e-4,
                    "self-heated dynamic DC reduction mismatch row={row} col={col}: dynamic={:.12e} base={:.12e} rel_err={rel_err:.3e}",
                    dynamic[row][col],
                    base.g_reduced[row][col]
                );
            }
        }
    }

    #[test]
    fn test_bjt_vbic_charge_snapshot_exposes_excess_phase_states_and_charges() {
        let params = vbic_reference_params();
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);
        let snapshot = q.charge_snapshot(1.2, 0.82, 0.0, 0.4);

        assert_eq!(snapshot.branches[IDX_QXF1].pos_internal, Some(IDX_VXF1));
        assert_eq!(snapshot.branches[IDX_QXF2].pos_internal, Some(IDX_VXF2));
        assert!((snapshot.branches[IDX_QXF1].d_internal[IDX_VXF1] - q.td).abs() < 1e-24);
        assert!((snapshot.branches[IDX_QXF2].d_internal[IDX_VXF2] - q.td / 3.0).abs() < 1e-24);
        assert!(snapshot.branches[IDX_QXF1].charge > 0.0);
        assert!(snapshot.branches[IDX_QXF2].charge > 0.0);

        assert!((snapshot.reduction.g_ii[IDX_VXF1][IDX_VXF2] - 1.0).abs() < 1e-12);
        assert!((snapshot.reduction.g_ii[IDX_VXF2][IDX_VXF1] + 1.0).abs() < 1e-12);
        assert!((snapshot.reduction.g_ii[IDX_VXF2][IDX_VXF2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_bjt_vbic_delay_static_collector_emitter_source_matches_internal_residual_polarity() {
        let params = vbic_reference_params();
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);
        let snapshot = q.charge_snapshot(1.2, 0.82, 0.0, 0.4);
        let branches = q.vbic_delay_static_branches(&snapshot.reduction);
        let delta_iciei = &branches[0];

        assert_eq!(delta_iciei.pos_internal, Some(IDX_VEI));
        assert_eq!(delta_iciei.neg_internal, Some(IDX_VCI));
    }

    #[test]
    fn test_bjt_vbic_excess_phase_reduction_maps_transport_derivatives_through_polarity() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let q = Bjt::new_pnp("Q1".to_string(), 4, 3, 2).with_params(&params);
        let snapshot = q.charge_snapshot(1.2, 2.48, 3.3, 1.6);
        let delay_branches = q.vbic_delay_static_branches(&snapshot.reduction);
        let ixf1 = delay_branches[1];
        let p = q.polarity();

        for idx in [IDX_VBI, IDX_VCI, IDX_VEI, IDX_VRTH, IDX_VXF2] {
            assert!(
                (snapshot.reduction.g_ii[IDX_VXF1][idx] - ixf1.d_internal[idx]).abs() < 1e-12,
                "expected xf1 row to preserve the excess-phase branch linearization at column {idx}"
            );
        }
        assert!((snapshot.reduction.g_ii[IDX_VCI][IDX_VXF2] + p).abs() < 1e-12);
        assert!((snapshot.reduction.g_ii[IDX_VEI][IDX_VXF2] - p).abs() < 1e-12);
        assert!(snapshot.reduction.g_ii[IDX_VCI][IDX_VRTH].is_finite());
        assert!(snapshot.reduction.g_ii[IDX_VEI][IDX_VRTH].is_finite());
    }

    #[test]
    fn test_bjt_vbic_charge_snapshot_without_td_keeps_delay_branches_inactive() {
        let mut params = vbic_reference_params();
        params.insert("TD".to_string(), 0.0);
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);
        let snapshot = q.charge_snapshot(1.2, 0.82, 0.0, 0.4);

        assert!(!snapshot.branches[IDX_QXF1].is_active());
        assert!(!snapshot.branches[IDX_QXF2].is_active());
        assert!((snapshot.reduction.g_ii[IDX_VXF1][IDX_VXF1] - 1.0).abs() < 1e-12);
        assert!((snapshot.reduction.g_ii[IDX_VXF2][IDX_VXF2] - 1.0).abs() < 1e-12);
        assert!(snapshot.reduction.internal_voltages[IDX_VXF1].abs() < 1e-18);
        assert!(snapshot.reduction.internal_voltages[IDX_VXF2].abs() < 1e-18);
    }

    #[test]
    fn test_bjt_vbic_rth_without_cth_uses_ngspice_minimum_thermal_capacitance() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);
        let snapshot = q.charge_snapshot(1.2, 0.82, 0.0, 0.4);

        assert!(q.self_heating_enabled());
        assert!((q.cth_nominal - 1e-12).abs() < 1e-24);
        assert!((q.cth - 1e-12).abs() < 1e-24);
        assert!(snapshot.branches[IDX_QCTH].is_active());
        assert!((snapshot.branches[IDX_QCTH].d_internal[IDX_VRTH] - q.cth).abs() < 1e-24);
        assert!(snapshot.reduction.g_ii[IDX_VRTH][IDX_VRTH].is_finite());
    }

    #[test]
    fn test_bjt_vbic_charge_snapshot_exposes_thermal_charge_when_cth_is_present() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        params.insert("CTH".to_string(), 2e-12);
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);
        let snapshot = q.charge_snapshot(1.2, 0.82, 0.0, 0.4);

        assert_eq!(snapshot.branches[IDX_QCTH].pos_internal, Some(IDX_VRTH));
        assert!(snapshot.branches[IDX_QCTH].is_active());
        assert!((snapshot.branches[IDX_QCTH].d_internal[IDX_VRTH] - q.cth).abs() < 1e-24);
        assert!(snapshot.reduction.g_ii[IDX_VRTH][IDX_VRTH].is_finite());
    }

    #[test]
    fn test_bjt_vbic_rth_without_selft_keeps_internal_thermal_state_disabled() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);

        assert_eq!(q.selft, 0.0);
        assert!((q.cth_nominal - 1e-12).abs() < 1e-24);
        assert!(!q.self_heating_enabled());
        assert_eq!(q.thermal_conductance(), 0.0);
        let snapshot = q.charge_snapshot(1.2, 0.82, 0.0, 0.4);
        assert!(!snapshot.branches[IDX_QCTH].is_active());
    }

    #[test]
    fn test_bjt_vbic_pnp_thermal_row_direct_collector_coupling_matches_finite_difference() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let q = Bjt::new_pnp("Q1".to_string(), 4, 3, 2).with_params(&params);
        let (vc, vb, ve, vs) = (1.575_451, 2.614_704, 1.94, 1.575_451);

        let state = q.intrinsic_state_for_biases(vc, vb, ve, vs);
        let eval = q.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let thermal_power = q.thermal_power_branch(
            eval,
            [vc, vb, ve, vs],
            [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
        );
        let thermal_row = Bjt::sub_branches(q.thermal_sink_branch(state.vrth), thermal_power);
        let eps = 1e-6;

        let thermal_row_residual = |vc: Value| {
            let eval = q.evaluate_state(
                vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
                state.vsi, state.vrth,
            );
            let row = Bjt::sub_branches(
                q.thermal_sink_branch(state.vrth),
                q.thermal_power_branch(
                    eval,
                    [vc, vb, ve, vs],
                    [
                        state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
                        state.vsi, state.vrth,
                    ],
                ),
            );
            row.current
        };
        let numerical =
            (thermal_row_residual(vc + eps) - thermal_row_residual(vc - eps)) / (2.0 * eps);
        let analytical = thermal_row.d_external[EXT_C];
        let scale = analytical.abs().max(numerical.abs()).max(1e-12);
        let rel_err = (analytical - numerical).abs() / scale;

        assert!(
            rel_err < 1e-4,
            "expected PNP thermal row collector derivative to match finite difference: analytical={analytical:.12e} numerical={numerical:.12e} rel_err={rel_err:.3e}"
        );
    }

    #[test]
    fn test_bjt_vbic_explicit_selft_zero_disables_internal_thermal_state() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 0.0);
        let q = Bjt::new_npn("Q1".to_string(), 4, 3, 2).with_params(&params);

        assert_eq!(q.selft, 0.0);
        assert!(!q.self_heating_enabled());
        assert_eq!(q.thermal_conductance(), 0.0);
        let snapshot = q.charge_snapshot(1.2, 0.82, 0.0, 0.4);
        assert!(!snapshot.branches[IDX_QCTH].is_active());
    }

    #[test]
    fn test_bjt_vbic_intrinsic_state_solver_reduces_fo_high_voltage_residual() {
        let params = vbic_reference_params();
        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let state = q.intrinsic_state_for_biases(4.0, 0.8, 0.0, 0.0);
        let residual = q
            .intrinsic_state_residual_jacobian(
                4.0,
                0.8,
                0.0,
                0.0,
                [
                    state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                    state.vrth,
                ],
            )
            .0;
        let residual_norm = Bjt::intrinsic_state_residual_norm(&residual);
        assert!(
            residual_norm < 1e-12,
            "FO high-voltage intrinsic residual too large: {residual_norm:.3e}"
        );
    }

    #[test]
    fn test_bjt_vbic_pnp_intrinsic_state_solver_reduces_diffamp_like_residual() {
        let params = vbic_reference_params();
        let q = Bjt::new_pnp("Q1".to_string(), 3, 2, 1).with_params(&params);
        let state = q.intrinsic_state_for_biases(1.5714, 1.9402, 2.6115, 1.5714);
        let residual = q
            .intrinsic_state_residual_jacobian(
                1.5714,
                1.9402,
                2.6115,
                1.5714,
                [
                    state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                    state.vrth,
                ],
            )
            .0;
        let residual_norm = Bjt::intrinsic_state_residual_norm(&residual);
        assert!(
            residual_norm < 1e-10,
            "PNP diffamp-like intrinsic residual too large: {residual_norm:.3e}"
        );
    }

    #[test]
    fn test_bjt_vbic_self_heating_intrinsic_state_solver_reduces_high_voltage_residual() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let state = q.intrinsic_state_for_biases(4.1, 0.75, 0.0, 0.0);
        let residual = q
            .intrinsic_state_residual_jacobian(
                4.1,
                0.75,
                0.0,
                0.0,
                [
                    state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                    state.vrth,
                ],
            )
            .0;
        let residual_norm = Bjt::intrinsic_state_residual_norm(&residual);
        assert!(
            residual_norm < 1e-10,
            "self-heated high-voltage intrinsic residual too large: {residual_norm:.3e}"
        );
        assert!(
            state.vrth > 0.0,
            "self-heated high-voltage operating point should produce a positive thermal rise"
        );
    }

    #[test]
    fn test_bjt_vbic_self_heating_repeated_update_preserves_warm_solution_quality() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let voltages = [0.0, 0.75, 4.1, 0.0];

        q.update(&voltages);
        let first_state = q.internal_state_vector();
        let first_residual_norm = Bjt::intrinsic_state_residual_norm(
            &q.intrinsic_state_residual_jacobian(4.1, 0.75, 0.0, 0.0, first_state)
                .0,
        );

        q.update(&voltages);
        let second_state = q.internal_state_vector();
        let second_residual_norm = Bjt::intrinsic_state_residual_norm(
            &q.intrinsic_state_residual_jacobian(4.1, 0.75, 0.0, 0.0, second_state)
                .0,
        );

        for idx in 0..INTERNAL_DIM {
            assert!(
                (second_state[idx] - first_state[idx]).abs() < 1e-12,
                "expected repeated warm-start update to preserve state at index {idx}"
            );
        }
        assert!(
            first_residual_norm < 1e-10,
            "expected first warm-startable self-heated residual to be small, got {first_residual_norm:.3e}"
        );
        assert!(
            second_residual_norm < 1e-10,
            "expected repeated self-heated update residual to remain small, got {second_residual_norm:.3e}"
        );
    }

    #[test]
    fn test_bjt_series_resistances_reduce_external_current_and_shift_internal_bias() {
        let mut ideal_params = HashMap::new();
        ideal_params.insert("RB".to_string(), 0.0);
        ideal_params.insert("RC".to_string(), 0.0);
        ideal_params.insert("RE".to_string(), 0.0);

        let mut series_params = HashMap::new();
        series_params.insert("RB".to_string(), 50.0);
        series_params.insert("RC".to_string(), 200.0);
        series_params.insert("RE".to_string(), 20.0);

        let mut ideal = Bjt::new_npn("Q0".to_string(), 3, 2, 1).with_params(&ideal_params);
        let mut with_series = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&series_params);
        let voltages = vec![0.0, 0.78, 3.3];

        ideal.update(&voltages);
        with_series.update(&voltages);

        assert!(with_series.vci < 3.3 - 1e-9);
        assert!(with_series.vei > 0.0 + 1e-9);
        assert!(with_series.vbe < ideal.vbe);
        assert!(with_series.ic.abs() < ideal.ic.abs());
    }

    #[test]
    fn test_bjt_series_resistance_small_signal_rows_preserve_kcl() {
        let mut params = HashMap::new();
        params.insert("RB".to_string(), 50.0);
        params.insert("RC".to_string(), 200.0);
        params.insert("RE".to_string(), 20.0);

        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let rows = q.small_signal_row_coefficients(3.3, 0.78, 0.0, 0.0);

        for col_idx in 0..EXTERNAL_DIM {
            let column_sum: Value = rows.iter().map(|row| row[col_idx]).sum();
            assert!(
                column_sum.abs() < 1e-9,
                "column {col_idx} violates KCL: {column_sum}"
            );
        }
    }

    #[test]
    fn test_bjt_vbic_small_signal_rows_preserve_four_terminal_kcl() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 4, 3, 2)
            .with_params(&params)
            .with_instance_params(&[("M".to_string(), 1.5)]);
        q.set_substrate_node(1);
        let biases = [1.2, 0.82, 0.0, 0.4];
        let mut voltages = vec![biases[EXT_S], biases[EXT_E], biases[EXT_B], biases[EXT_C]];
        q.update(&voltages);
        let rows = q.small_signal_row_coefficients(
            biases[EXT_C],
            biases[EXT_B],
            biases[EXT_E],
            biases[EXT_S],
        );

        for col_idx in 0..EXTERNAL_DIM {
            let column_sum: Value = rows.iter().map(|row| row[col_idx]).sum();
            assert!(
                column_sum.abs() < 1e-8,
                "column {col_idx} violates four-terminal KCL: {column_sum}"
            );
        }

        let currents = |device: &Bjt| {
            let state = device.operating_point_state();
            [state.ic, state.ib, state.ie, state.isub]
        };
        let eps = 1e-6;
        for col_idx in 0..EXTERNAL_DIM {
            let voltage_slot = match col_idx {
                EXT_S => 0,
                EXT_E => 1,
                EXT_B => 2,
                EXT_C => 3,
                _ => unreachable!(),
            };
            let mut plus = q.clone();
            voltages[voltage_slot] += eps;
            plus.update(&voltages);
            let plus_currents = currents(&plus);

            let mut minus = q.clone();
            voltages[voltage_slot] -= 2.0 * eps;
            minus.update(&voltages);
            let minus_currents = currents(&minus);
            voltages[voltage_slot] += eps;

            for row_idx in 0..EXTERNAL_DIM {
                let numerical = (plus_currents[row_idx] - minus_currents[row_idx]) / (2.0 * eps);
                let analytical = rows[row_idx][col_idx];
                let scale = numerical.abs().max(analytical.abs()).max(1e-10);
                let rel_err = (analytical - numerical).abs() / scale;
                assert!(
                    rel_err < 2e-2,
                    "four-terminal Jacobian mismatch row={row_idx} col={col_idx}: analytical={analytical:.12e} numerical={numerical:.12e} rel_err={rel_err:.3e}"
                );
            }
        }
    }

    #[test]
    fn test_bjt_vbic_pnp_small_signal_rows_match_finite_difference_at_diffamp_bias() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_pnp("Q1".to_string(), 4, 3, 2).with_params(&params);
        q.set_substrate_node(1);
        let biases = [1.575_451, 1.940_0, 2.614_704, 1.575_451];
        let mut voltages = vec![biases[EXT_S], biases[EXT_E], biases[EXT_B], biases[EXT_C]];
        q.update(&voltages);
        let rows = q.small_signal_row_coefficients(
            biases[EXT_C],
            biases[EXT_B],
            biases[EXT_E],
            biases[EXT_S],
        );

        let currents = |device: &Bjt| {
            let state = device.operating_point_state();
            [state.ic, state.ib, state.ie, state.isub]
        };
        let eps = 1e-6;
        for col_idx in 0..EXTERNAL_DIM {
            let voltage_slot = match col_idx {
                EXT_S => 0,
                EXT_E => 1,
                EXT_B => 2,
                EXT_C => 3,
                _ => unreachable!(),
            };
            let mut plus = q.clone();
            voltages[voltage_slot] += eps;
            plus.update(&voltages);
            let plus_currents = currents(&plus);

            let mut minus = q.clone();
            voltages[voltage_slot] -= 2.0 * eps;
            minus.update(&voltages);
            let minus_currents = currents(&minus);
            voltages[voltage_slot] += eps;

            for row_idx in 0..EXTERNAL_DIM {
                let numerical = (plus_currents[row_idx] - minus_currents[row_idx]) / (2.0 * eps);
                let analytical = rows[row_idx][col_idx];
                let scale = numerical.abs().max(analytical.abs()).max(1e-10);
                let rel_err = (analytical - numerical).abs() / scale;
                assert!(
                    rel_err < 2e-2,
                    "PNP four-terminal Jacobian mismatch row={row_idx} col={col_idx}: analytical={analytical:.12e} numerical={numerical:.12e} rel_err={rel_err:.3e}"
                );
            }
        }
    }

    #[test]
    fn test_bjt_node_current_aggregates_tied_substrate_terminal() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_substrate_node(q.node_collector);
        q.set_temperature(423.15);
        q.update(&[0.0, 0.3, 0.3]);

        let state = q.operating_point_state();
        assert!(state.isub.abs() > 0.0);
        assert_relative_within(
            q.node_current(q.node_collector).abs(),
            (state.ic + state.isub).abs(),
            1e-12,
            "collector node current should include tied substrate current",
        );
    }

    #[test]
    fn test_bjt_instance_multiplier_scales_collector_current() {
        let q1 = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        let q2 =
            Bjt::new_npn("Q2".to_string(), 2, 1, 0).with_instance_params(&[("M".to_string(), 2.0)]);

        // Stay in moderate current to avoid knee-current nonlinearity in this
        // pure multiplicity-scaling check.
        let (ic1, _, _) = q1.calculate_currents(0.35, -5.0);
        let (ic2, _, _) = q2.calculate_currents(0.35, -5.0);

        assert!(ic1 > 0.0 && ic2 > 0.0);
        let ratio = ic2 / ic1;
        assert!(
            (ratio - 2.0).abs() < 3e-2,
            "expected ~2x collector current from M=2, got ratio={ratio}"
        );
    }

    #[test]
    fn test_bjt_temperature_scaling_raises_low_bias_current() {
        let mut q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        let (ic_room, _, _) = q.calculate_currents(0.2, -0.2);

        q.set_temperature(423.15); // 150C
        let (ic_hot, _, _) = q.calculate_currents(0.2, -0.2);

        assert!(
            ic_hot > ic_room,
            "expected higher current at elevated temperature: room={ic_room} hot={ic_hot}"
        );
    }

    // =========================================================================
    // Operating Region Tests
    // =========================================================================

    #[test]
    fn test_bjt_forward_active() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Typical forward active: Vbe ~ 0.7V, Vbc < 0
        let (ic, ib, ie) = q.calculate_currents(0.7, -5.0);

        // Ic should be positive and >> Ib
        assert!(ic > 0.0);
        assert!(ib > 0.0);
        assert!(ic > ib * 10.0); // Beta > 10

        // KCL check
        assert!((ic + ib + ie).abs() < 1e-12);
    }

    #[test]
    fn test_bjt_cutoff() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Cutoff: Vbe << 0, Vbc << 0
        let (ic, ib, ie) = q.calculate_currents(-0.5, -5.0);

        // All currents should be essentially zero
        assert!(ic.abs() < 1e-12, "Ic should be ~0 in cutoff, got {}", ic);
        assert!(ib.abs() < 1e-12, "Ib should be ~0 in cutoff, got {}", ib);
        assert!(ie.abs() < 1e-12, "Ie should be ~0 in cutoff, got {}", ie);
    }

    #[test]
    fn test_bjt_saturation() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Saturation: Vbe > 0, Vbc > 0 (both junctions forward biased)
        let (ic, ib, _ie) = q.calculate_currents(0.75, 0.65);

        // In saturation, beta is reduced
        let beta_sat = ic.abs() / ib.abs();
        assert!(beta_sat < 50.0, "Beta should be reduced in saturation");
    }

    #[test]
    fn test_bjt_reverse_active() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Reverse active: Vbe < 0, Vbc > 0
        let (ic, ib, ie) = q.calculate_currents(-0.5, 0.7);

        // Current should flow, but Ic negative (emitter acts as collector)
        assert!(ic < 0.0, "Ic should be negative in reverse active");
        assert!((ic + ib + ie).abs() < 1e-12); // KCL
    }

    // =========================================================================
    // Small-Signal Parameter Tests
    // =========================================================================

    #[test]
    fn test_bjt_gm_positive() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        let gm = q.gm(0.7);
        assert!(gm > 0.0, "gm should be positive");

        // gm ≈ Ic / Vt for ideal BJT
        // At Vbe=0.7V, Ic is in mA range, so gm should be tens of mS
        assert!(gm > 1e-3, "gm should be > 1mS at Vbe=0.7V");
    }

    #[test]
    fn test_bjt_gm_increases_with_vbe() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        let gm1 = q.gm(0.6);
        let gm2 = q.gm(0.7);
        let gm3 = q.gm(0.8);

        assert!(gm2 > gm1, "gm should increase with Vbe");
        assert!(gm3 > gm2, "gm should increase with Vbe");
    }

    #[test]
    fn test_bjt_gm_ikf_zero_disables_high_injection_rolloff() {
        use std::collections::HashMap;

        let mut params_disabled = HashMap::new();
        params_disabled.insert("IKF".to_string(), 0.0);
        let q_disabled = Bjt::new_npn("Q_OFF".to_string(), 2, 1, 0).with_params(&params_disabled);

        let mut params_enabled = HashMap::new();
        params_enabled.insert("IKF".to_string(), 1e-3);
        let q_enabled = Bjt::new_npn("Q_ON".to_string(), 2, 1, 0).with_params(&params_enabled);

        // At strong forward bias, a finite IKF should reduce gm measurably.
        let gm_disabled = q_disabled.gm(0.85);
        let gm_enabled = q_enabled.gm(0.85);
        assert!(
            gm_disabled > gm_enabled * 10.0,
            "IKF=0 should keep gm much larger than finite-IKF rolloff: disabled={gm_disabled:.12e} enabled={gm_enabled:.12e}"
        );
    }

    #[test]
    fn test_bjt_gm_default_matches_explicit_ikf_zero() {
        use std::collections::HashMap;

        let q_default = Bjt::new_npn("Q_DEFAULT".to_string(), 2, 1, 0).with_params(&HashMap::new());
        let mut params = HashMap::new();
        params.insert("IKF".to_string(), 0.0);
        let q_ikf_zero = Bjt::new_npn("Q_IKF0".to_string(), 2, 1, 0).with_params(&params);

        let gm_default = q_default.gm(0.8);
        let gm_ikf_zero = q_ikf_zero.gm(0.8);
        let scale = gm_default.abs().max(1e-18);
        let rel_err = (gm_default - gm_ikf_zero).abs() / scale;
        assert!(
            rel_err < 1e-12,
            "default IKF should behave identically to explicit IKF=0: default={gm_default:.12e} ikf0={gm_ikf_zero:.12e} rel_err={rel_err:.3e}"
        );
    }

    #[test]
    fn test_bjt_go_early_effect() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // go = |Ic| / VAF
        let ic = 1e-3; // 1mA
        let go = q.go(ic);

        // With VAF=100V, go = 1mA/100V = 10μS
        assert!((go - 1e-5).abs() < 1e-6, "go should be ~10μS, got {}", go);
    }

    #[test]
    fn test_bjt_gbe_gbc() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        let gbe = q.gbe(0.7);
        let gbc = q.gbc(-5.0);

        // gbe should be gm / beta (approximately)
        assert!(gbe > 0.0);

        // gbc should be very small for reverse biased junction
        assert!(
            gbc < gbe * 0.01,
            "gbc << gbe for reverse biased BC junction"
        );
    }

    // =========================================================================
    // Capacitance Tests
    // =========================================================================

    #[test]
    fn test_bjt_junction_capacitances() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Forward active: Vbe=0.7V, Vbc=-5V
        let (cbe, cbc) = q.junction_capacitances(0.7, -5.0);

        // Cbe should be larger (forward biased + diffusion cap from TF*gm)
        assert!(cbe > 1e-12, "Expected Cbe > 1pF, got {:.2e}", cbe);

        // Cbc should be small (reverse biased)
        assert!(
            cbc > 0.1e-12 && cbc < 5e-12,
            "Expected Cbc ~0.5pF, got {:.2e}",
            cbc
        );

        // Cbe should be larger than Cbc in forward active
        assert!(cbe > cbc, "Expected Cbe > Cbc in forward active");
    }

    #[test]
    fn test_bjt_cbe_includes_diffusion() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Diffusion capacitance = gm * TF
        let gm = q.gm(0.7);
        let cbe = q.cbe(0.7, gm);

        // Cd = gm * TF, with TF=400ps and gm~40mS, Cd~16pF
        // Total Cbe should be >> CJE due to diffusion
        assert!(
            cbe > q.cje * 5.0,
            "Cbe should include significant diffusion cap"
        );
    }

    #[test]
    fn test_bjt_cbc_reverse_bias() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Reverse biased BC junction
        let cbc_rev = q.cbc(-10.0);
        let cbc_zero = q.cbc(0.0);

        // Capacitance should decrease with reverse bias
        assert!(cbc_rev < cbc_zero, "Cbc should decrease with reverse bias");
    }

    // =========================================================================
    // PNP Operation Tests
    // =========================================================================

    #[test]
    fn test_pnp_forward_active() {
        let q = Bjt::new_pnp("Q1".to_string(), 2, 1, 0);

        // PNP forward active: Vbe=-0.7V, Vbc=+5V
        let (ic, ib, ie) = q.calculate_currents(-0.7, 5.0);

        // Ic should be negative (current flows out of collector)
        assert!(ic < 0.0, "PNP Ic should be negative, got {}", ic);
        assert!(ib < 0.0, "PNP Ib should be negative, got {}", ib);

        // KCL check
        assert!((ic + ib + ie).abs() < 1e-12);
    }

    #[test]
    fn test_pnp_gm() {
        let q = Bjt::new_pnp("Q1".to_string(), 2, 1, 0);

        // gm should still be positive (magnitude of transconductance)
        let gm = q.gm(-0.7);
        assert!(gm > 0.0, "gm magnitude should be positive for PNP");
    }

    // =========================================================================
    // High-Injection Tests
    // =========================================================================

    #[test]
    fn test_bjt_high_injection_beta_reduction() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("IKF".to_string(), 0.01); // Low knee current for easy testing

        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);

        // At low current
        let (ic_low, ib_low, _) = q.calculate_currents(0.6, -5.0);
        let beta_low = ic_low / ib_low;

        // At high current (above IKF)
        let (ic_high, ib_high, _) = q.calculate_currents(0.85, -5.0);
        let beta_high = ic_high / ib_high;

        // Beta should be lower at high current due to high-injection effects
        assert!(
            beta_high < beta_low,
            "Beta should decrease at high injection"
        );
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_bjt_zero_voltages() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        let (ic, ib, ie) = q.calculate_currents(0.0, 0.0);

        // All currents should be near zero
        assert!(ic.abs() < 1e-12);
        assert!(ib.abs() < 1e-12);
        assert!(ie.abs() < 1e-12);
    }

    #[test]
    fn test_bjt_large_reverse_bias() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Large reverse biases should not cause overflow
        let (ic, ib, ie) = q.calculate_currents(-10.0, -100.0);

        assert!(ic.is_finite());
        assert!(ib.is_finite());
        assert!(ie.is_finite());
    }

    #[test]
    fn test_bjt_convergence_check() {
        let loose = NonlinearConvergenceCriteria::voltage_only(0.01);
        let tight = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Simulate convergence
        q.vbe = 0.7;
        q.vbc = -5.0;
        q.vbe_prev = 0.7001;
        q.vbc_prev = -5.001;

        // Should be converged within 1mV tolerance
        // With RELTOL=1e-3: tol_vbe = 0.001*0.7 + 0.01 ≈ 0.0107, Δ=0.0001 < 0.0107 ✓
        assert!(q.is_converged(loose));

        // Even with very tight VNTOL, RELTOL provides adequate tolerance for
        // junction voltages. This is correct SPICE behavior.
        // tol_vbe = 0.001*0.7 + 1e-6 ≈ 0.0007, Δ=0.0001 < 0.0007 ✓
        assert!(q.is_converged(tight));

        // To fail convergence, delta must exceed RELTOL*|V| + VNTOL
        q.vbe_prev = 0.71; // Δ=0.01, tol=0.001*0.71+1e-6≈0.00071, 0.01 > 0.00071
        assert!(!q.is_converged(tight));
    }

    #[test]
    fn test_bjt_vbic_predicted_branch_current_ignores_thermal_delta() {
        let mut branch = BranchLinearization::default();
        branch.current = 1.5e-6;
        branch.d_internal[IDX_VBI] = 2.0e-3;
        branch.d_internal[IDX_VCI] = -1.0e-3;
        branch.d_internal[IDX_VRTH] = 7.5e-3;

        let mut delta = [0.0; INTERNAL_DIM];
        delta[IDX_VBI] = 2.0e-3;
        delta[IDX_VCI] = -3.0e-3;
        delta[IDX_VRTH] = 5.0;

        let predicted = Bjt::vbic_predicted_branch_current(&branch, &delta);
        let expected = branch.current
            + branch.d_internal[IDX_VBI] * delta[IDX_VBI]
            + branch.d_internal[IDX_VCI] * delta[IDX_VCI];
        assert!((predicted - expected).abs() < 1e-24);
    }

    #[test]
    fn test_bjt_vbic_convergence_branches_are_finite_at_reference_bias() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.update(&[0.0, 0.75, 4.1]);

        let branches = q.vbic_convergence_branches_for_state(q.internal_state_vector());
        for branch in branches {
            assert!(branch.current.is_finite());
            assert!(branch.d_internal.iter().all(|value| value.is_finite()));
        }
    }

    #[test]
    fn test_bjt_reduced_linearization_cache_matches_fresh_rebuild() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let [vc, vb, ve, vs] = [0.0, 0.75, 4.1, 0.0];
        q.update(&[vc, vb, ve, vs]);

        let cached = q.reduced_linearization(vc, vb, ve, vs);
        let fresh = q.compute_reduced_linearization(vc, vb, ve, vs);
        for idx in 0..INTERNAL_DIM {
            assert!((cached.internal_voltages[idx] - fresh.internal_voltages[idx]).abs() < 1e-18);
            for col in 0..INTERNAL_DIM {
                assert!((cached.g_ii[idx][col] - fresh.g_ii[idx][col]).abs() < 1e-15);
            }
            for col in 0..EXTERNAL_DIM {
                assert!((cached.g_ie[idx][col] - fresh.g_ie[idx][col]).abs() < 1e-15);
            }
        }
        for row in 0..EXTERNAL_DIM {
            for col in 0..INTERNAL_DIM {
                assert!((cached.g_ei[row][col] - fresh.g_ei[row][col]).abs() < 1e-15);
            }
            for col in 0..EXTERNAL_DIM {
                assert!((cached.g_ee[row][col] - fresh.g_ee[row][col]).abs() < 1e-15);
                assert!((cached.g_reduced[row][col] - fresh.g_reduced[row][col]).abs() < 1e-15);
            }
        }
    }

    #[test]
    fn test_bjt_vbic_reduced_linearization_stays_continuous_for_small_off_cache_npn_diffamp_step() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_substrate_node(0);

        let [vc, vb, ve, vs] = [2.614_704, 1.65, 1.011_054, 0.0];
        q.update(&[ve, vb, vc]);

        let base = q.reduced_linearization(vc, vb, ve, vs);
        let previous_internal = q.internal_state_vector();
        let previous_external = [vc, vb, ve, vs];
        let eps = 1e-6;
        let plus = q.compute_reduced_linearization(vc + eps, vb, ve, vs);
        let minus = q.compute_reduced_linearization(vc - eps, vb, ve, vs);
        let base_branches = q.vbic_nonlinear_branch_voltages(base.internal_voltages);

        for (label, solved, external_vc) in [("plus", plus, vc + eps), ("minus", minus, vc - eps)] {
            let predicted = q
                .predict_intrinsic_state_from_previous_external_bias(
                    previous_external,
                    previous_internal,
                    [external_vc, vb, ve, vs],
                )
                .expect("predict off-cache NPN diffamp state");
            let predicted_branches = q.vbic_nonlinear_branch_voltages(predicted);
            let predicted_residual = q
                .intrinsic_state_residual_jacobian(external_vc, vb, ve, vs, predicted)
                .0;
            let predicted_residual_norm = Bjt::intrinsic_state_residual_norm(&predicted_residual);
            let solved_branches = q.vbic_nonlinear_branch_voltages(solved.internal_voltages);
            for (branch_name, solved_value, base_value) in [
                ("vbei", solved_branches.vbei, base_branches.vbei),
                ("vbex", solved_branches.vbex, base_branches.vbex),
                ("vbci", solved_branches.vbci, base_branches.vbci),
                ("vbcx", solved_branches.vbcx, base_branches.vbcx),
                ("vbep", solved_branches.vbep, base_branches.vbep),
                ("vbcp", solved_branches.vbcp, base_branches.vbcp),
            ] {
                assert!(
                    (solved_value - base_value).abs() < 1e-3,
                    "expected {label} off-cache VBIC branch {branch_name} to remain continuous near the cached NPN diffamp bias: base={base_value:.12e} predicted={:.12e} solved={solved_value:.12e} predicted_residual={predicted_residual_norm:.12e}",
                    match branch_name {
                        "vbei" => predicted_branches.vbei,
                        "vbex" => predicted_branches.vbex,
                        "vbci" => predicted_branches.vbci,
                        "vbcx" => predicted_branches.vbcx,
                        "vbep" => predicted_branches.vbep,
                        "vbcp" => predicted_branches.vbcp,
                        _ => unreachable!(),
                    }
                );
            }
            let residual = q
                .intrinsic_state_residual_jacobian(
                    external_vc,
                    vb,
                    ve,
                    vs,
                    solved.internal_voltages,
                )
                .0;
            assert!(
                predicted_residual_norm < 1e-5,
                "expected {label} predicted off-cache VBIC state to stay near the cached NPN diffamp solution"
            );
            assert!(
                Bjt::intrinsic_state_residual_norm(&residual) < 1e-8,
                "expected {label} off-cache VBIC solve to remain converged near the cached NPN diffamp bias"
            );
        }
    }

    #[test]
    fn test_bjt_stamped_reduced_external_system_rebuilds_rhs_for_requested_bias() {
        let params = vbic_reference_params();
        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let [vc, vb, ve, vs] = [0.0, 0.75, 4.1, 0.0];
        let expected = q.external_terminal_currents_at_bias(vc, vb, ve, vs);
        let (rows, rhs) = q.stamped_reduced_external_system(vc, vb, ve, vs);

        for row in 0..EXTERNAL_DIM {
            let stamped_current = -rhs[row]
                + rows[row]
                    .iter()
                    .zip([vc, vb, ve, vs].iter())
                    .map(|(g, v)| g * v)
                    .sum::<Value>();
            assert!(
                (stamped_current - expected[row]).abs() < 1e-12,
                "expected reduced stamped current to match the requested-bias terminal current at row {row}, stamped={:.16e}, expected={:.16e}",
                stamped_current,
                expected[row]
            );
        }
    }

    #[test]
    fn test_bjt_charge_snapshot_cache_matches_fresh_rebuild() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let [vc, vb, ve, vs] = [0.0, 0.75, 4.1, 0.0];
        q.update(&[vc, vb, ve, vs]);

        let cached = q.charge_snapshot(vc, vb, ve, vs);
        let reduction = q.build_dynamic_reduction(q.compute_reduced_linearization(vc, vb, ve, vs));
        let fresh = BjtChargeSnapshot {
            reduction,
            branches: q.dynamic_charge_branches(&reduction),
        };

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (cached.reduction.internal_voltages[idx] - fresh.reduction.internal_voltages[idx])
                    .abs()
                    < 1e-18
            );
        }
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                assert!(
                    (cached.reduction.g_ii[row][col] - fresh.reduction.g_ii[row][col]).abs()
                        < 1e-15
                );
            }
        }
        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            assert!(
                (cached.branches[branch_idx].charge - fresh.branches[branch_idx].charge).abs()
                    < 1e-18
            );
            for idx in 0..BJT_INTERNAL_STATE_DIM {
                assert!(
                    (cached.branches[branch_idx].d_internal[idx]
                        - fresh.branches[branch_idx].d_internal[idx])
                        .abs()
                        < 1e-15
                );
            }
        }
    }

    #[test]
    fn test_bjt_charge_snapshot_for_dynamic_state_matches_cached_snapshot_at_operating_point() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let [vc, vb, ve, vs] = [0.0, 0.75, 4.1, 0.0];
        q.update(&[vc, vb, ve, vs]);

        let cached = q.charge_snapshot(vc, vb, ve, vs);
        let rebuilt =
            q.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, cached.reduction.internal_voltages);

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (cached.reduction.internal_voltages[idx]
                    - rebuilt.reduction.internal_voltages[idx])
                    .abs()
                    < 1e-18
            );
            for col in 0..BJT_INTERNAL_STATE_DIM {
                assert!(
                    (cached.reduction.g_ii[idx][col] - rebuilt.reduction.g_ii[idx][col]).abs()
                        < 1e-15
                );
            }
        }
        for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
            assert!(
                (cached.branches[branch_idx].charge - rebuilt.branches[branch_idx].charge).abs()
                    < 1e-18
            );
            for idx in 0..BJT_INTERNAL_STATE_DIM {
                assert!(
                    (cached.branches[branch_idx].d_internal[idx]
                        - rebuilt.branches[branch_idx].d_internal[idx])
                        .abs()
                        < 1e-15
                );
            }
        }
    }

    #[test]
    fn test_bjt_charge_snapshot_for_dynamic_state_preserves_explicit_delay_nodes() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let [vc, vb, ve, vs] = [0.0, 0.75, 4.1, 0.0];
        q.update(&[vc, vb, ve, vs]);

        let cached = q.charge_snapshot(vc, vb, ve, vs);
        let mut internal = cached.reduction.internal_voltages;
        internal[IDX_VXF1] = 1.75 * internal[IDX_VXF1] + 1.0e-6;
        internal[IDX_VXF2] = 0.65 * internal[IDX_VXF2] - 2.0e-6;

        let rebuilt = q.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, internal);

        assert!((rebuilt.reduction.internal_voltages[IDX_VXF1] - internal[IDX_VXF1]).abs() < 1e-18);
        assert!((rebuilt.reduction.internal_voltages[IDX_VXF2] - internal[IDX_VXF2]).abs() < 1e-18);
        assert!((rebuilt.branches[IDX_QXF1].charge - q.td * internal[IDX_VXF1]).abs() < 1e-18);
        assert!(
            (rebuilt.branches[IDX_QXF2].charge - q.td * internal[IDX_VXF2] / 3.0).abs() < 1e-18
        );
    }

    #[test]
    fn test_bjt_charge_snapshot_for_dynamic_state_preserves_explicit_delay_nodes_with_self_heating()
    {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let [vc, vb, ve, vs] = [0.0, 0.75, 4.1, 0.0];
        q.update(&[vc, vb, ve, vs]);

        let cached = q.charge_snapshot(vc, vb, ve, vs);
        let mut internal = cached.reduction.internal_voltages;
        internal[IDX_VXF1] = 1.75 * internal[IDX_VXF1] + 1.0e-6;
        internal[IDX_VXF2] = 0.65 * internal[IDX_VXF2] - 2.0e-6;

        let rebuilt = q.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, internal);

        assert!((rebuilt.reduction.internal_voltages[IDX_VXF1] - internal[IDX_VXF1]).abs() < 1e-18);
        assert!((rebuilt.reduction.internal_voltages[IDX_VXF2] - internal[IDX_VXF2]).abs() < 1e-18);
        assert!((rebuilt.branches[IDX_QXF1].charge - q.td * internal[IDX_VXF1]).abs() < 1e-18);
        assert!(
            (rebuilt.branches[IDX_QXF2].charge - q.td * internal[IDX_VXF2] / 3.0).abs() < 1e-18
        );
    }

    #[test]
    fn test_bjt_dynamic_internal_state_seed_matches_cached_operating_point_snapshot() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let [vc, vb, ve, vs] = [0.0, 0.75, 4.1, 0.0];
        q.update(&[vc, vb, ve, vs]);

        let cached = q.charge_snapshot(vc, vb, ve, vs);
        let seed = q.dynamic_internal_state_seed(vc, vb, ve, vs);

        for idx in 0..BJT_INTERNAL_STATE_DIM {
            assert!(
                (seed[idx] - cached.reduction.internal_voltages[idx]).abs() < 1e-18,
                "expected operating-point dynamic seed to match cached snapshot at index {idx}"
            );
        }
    }

    #[test]
    fn test_bjt_vbic_convergence_matches_manual_ngspice_branch_current_test() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.update(&[0.0, 0.75, 4.1]);
        q.update(&[0.0, 0.75001, 4.10002]);

        let criteria = NonlinearConvergenceCriteria::new(1e-6, 1e-12, 1e-3);
        let current_state = q.internal_state_vector();
        let previous_state = q.previous_internal_state_vector();
        let current_voltages = q.vbic_convergence_voltage_vector_for_state(current_state);
        let previous_voltages = q.vbic_convergence_voltage_vector_for_state(previous_state);
        let current_branches = q.vbic_convergence_branches_for_state(current_state);
        let previous_branches = q.vbic_convergence_branches_for_state(previous_state);
        let mut delta_internal = [0.0; INTERNAL_DIM];
        for idx in 0..INTERNAL_DIM {
            delta_internal[idx] = current_state[idx] - previous_state[idx];
        }

        let voltages_converged =
            current_voltages
                .iter()
                .zip(previous_voltages.iter())
                .all(|(current, previous)| {
                    let diff = (current - previous).abs();
                    let tol = criteria.relative_tolerance() * current.abs().max(previous.abs())
                        + criteria.voltage_tolerance();
                    diff <= tol
                });
        let currents_converged = current_branches
            .iter()
            .zip(previous_branches.iter())
            .enumerate()
            .all(|(branch_idx, (current, previous))| {
                if q.uses_vbic_dynamic_charges()
                    && branch_idx == VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX
                {
                    return true;
                }
                let predicted = previous.current
                    + previous
                        .d_internal
                        .iter()
                        .zip(delta_internal.iter())
                        .enumerate()
                        .filter(|(idx, _)| *idx != IDX_VRTH)
                        .map(|(_, (derivative, delta))| derivative * delta)
                        .sum::<Value>();
                let actual = current.current;
                let tol = criteria.relative_tolerance() * predicted.abs().max(actual.abs())
                    + criteria.current_tolerance();
                (predicted - actual).abs() <= tol
            });
        let expected = voltages_converged && currents_converged;

        assert_eq!(q.vbic_is_converged(criteria), expected);
    }

    #[test]
    fn test_vbic_transient_convergence_state_includes_delayed_iciei_vxf2_term() {
        let params = vbic_reference_params();
        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let [vc, vb, ve, vs] = [0.0, 0.75, 4.1, 0.0];
        let base_snapshot = q.charge_snapshot(vc, vb, ve, vs);
        let base_state =
            q.vbic_transient_convergence_state_for_snapshot(vc, vb, ve, vs, &base_snapshot);

        let mut shifted_internal = base_snapshot.reduction.internal_voltages;
        shifted_internal[IDX_VXF2] += 1.0e-6;
        let shifted_state = q.vbic_transient_convergence_state(vc, vb, ve, vs, shifted_internal);

        assert!(
            (shifted_state.currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX]
                - base_state.currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX]
                - 1.0e-6)
                .abs()
                < 1e-12
        );
        assert!(
            (base_state.d_currents_d_internal[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX][IDX_VXF2]
                - 1.0)
                .abs()
                < 1e-12
        );
        assert_eq!(base_state.voltages.len(), VBIC_TRANSIENT_CONVERGENCE_VOLTAGE_COUNT);
    }

    #[test]
    fn test_bjt_limit_logarithmic_step_matches_ngspice_behavior() {
        let within = Bjt::limit_logarithmic_step(12.0, 10.0, 5.0);
        assert!((within - 12.0).abs() < 1e-12);

        let limited_up = Bjt::limit_logarithmic_step(140.0, 10.0, 100.0);
        let expected_up = 10.0 + 100.0 + ((140.0_f64 - 10.0) / 100.0).log10();
        assert!((limited_up - expected_up).abs() < 1e-12);

        let limited_down = Bjt::limit_logarithmic_step(-130.0, 10.0, 100.0);
        let expected_down = 10.0 - 100.0 - ((10.0_f64 - (-130.0)) / 100.0).log10();
        assert!((limited_down - expected_down).abs() < 1e-12);
    }

    #[test]
    fn test_bjt_limit_junction_voltage_matches_ngspice_pnjlim_behavior() {
        let vt = 0.025851999786;
        let vcrit = Bjt::junction_critical_voltage(vt, 1e-16);

        let forward_limited = Bjt::limit_junction_voltage(1.25, 0.65, vt, vcrit);
        let expected_forward = 0.65 + vt * (2.0 + (((1.25 - 0.65) / vt) - 2.0).ln());
        assert!((forward_limited - expected_forward).abs() < 1e-12);

        let reverse_to_forward = Bjt::limit_junction_voltage(1.2, -0.1, vt, vcrit);
        let expected_reverse_to_forward = vt * (1.2 / vt).ln();
        assert!((reverse_to_forward - expected_reverse_to_forward).abs() < 1e-12);

        let reverse_clamped_from_forward = Bjt::limit_junction_voltage(-3.0, 0.4, vt, vcrit);
        assert!((reverse_clamped_from_forward - (-1.4)).abs() < 1e-12);

        let reverse_clamped_from_reverse = Bjt::limit_junction_voltage(-3.0, -0.4, vt, vcrit);
        assert!((reverse_clamped_from_reverse - (-1.8)).abs() < 1e-12);
    }

    #[test]
    fn test_vbic_internal_branch_limiter_matches_ngspice_branch_targets() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.update(&[0.0, 0.75, 4.1]);

        let previous = q.internal_state_vector();
        let previous_branches = q.vbic_nonlinear_branch_voltages(previous);
        let raw_branches = VbicNonlinearBranchVoltages {
            vbei: previous_branches.vbei + 1.10,
            vbex: previous_branches.vbex + 1.00,
            vbci: previous_branches.vbci + 1.30,
            vbcx: previous_branches.vbcx + 1.20,
            vbep: previous_branches.vbep + 0.90,
            vbcp: previous_branches.vbcp + 1.10,
            vrth: previous_branches.vrth + 250.0,
        };
        let vei = previous[IDX_VEI] - 0.2;
        let raw = [
            vei + raw_branches.vbei - raw_branches.vbcx,
            vei + raw_branches.vbei - raw_branches.vbci,
            vei + raw_branches.vbex,
            vei + raw_branches.vbei,
            vei,
            vei + raw_branches.vbex - raw_branches.vbep,
            vei + raw_branches.vbex - raw_branches.vbep + raw_branches.vbcp,
            raw_branches.vrth,
        ];

        let limited = q.limit_vbic_internal_state_to_previous(raw, previous);
        let limited_branches = q.vbic_nonlinear_branch_voltages(limited);
        let (vt, vcrit) = q.vbic_limiting_parameters(previous[IDX_VRTH]);

        assert_relative_or_absolute_within(
            limited_branches.vbei,
            Bjt::limit_junction_voltage(raw_branches.vbei, previous_branches.vbei, vt, vcrit),
            1e-12,
            1e-12,
            "limited Vbei",
        );
        assert_relative_or_absolute_within(
            limited_branches.vbex,
            Bjt::limit_junction_voltage(raw_branches.vbex, previous_branches.vbex, vt, vcrit),
            1e-12,
            1e-12,
            "limited Vbex",
        );
        assert_relative_or_absolute_within(
            limited_branches.vbci,
            Bjt::limit_junction_voltage(raw_branches.vbci, previous_branches.vbci, vt, vcrit),
            1e-12,
            1e-12,
            "limited Vbci",
        );
        assert_relative_or_absolute_within(
            limited_branches.vbcx,
            Bjt::limit_junction_voltage(raw_branches.vbcx, previous_branches.vbcx, vt, vcrit),
            1e-12,
            1e-12,
            "limited Vbcx",
        );
        assert_relative_or_absolute_within(
            limited_branches.vbep,
            Bjt::limit_junction_voltage(raw_branches.vbep, previous_branches.vbep, vt, vcrit),
            1e-12,
            1e-12,
            "limited Vbep",
        );
        assert_relative_or_absolute_within(
            limited_branches.vbcp,
            Bjt::limit_junction_voltage(raw_branches.vbcp, previous_branches.vbcp, vt, vcrit),
            1e-12,
            1e-12,
            "limited Vbcp",
        );
        assert_relative_or_absolute_within(
            limited_branches.vrth,
            Bjt::limit_logarithmic_step(raw_branches.vrth, previous_branches.vrth, 100.0)
                .max(q.minimum_thermal_rise()),
            1e-12,
            1e-12,
            "limited Vrth",
        );
    }

    #[test]
    fn test_vbic_update_preserves_converged_intrinsic_state_after_large_step_limiting() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_substrate_node(4);
        q.update(&[0.0, 0.75, 4.1, 0.0]);

        let previous = q.internal_state_vector();
        let [vc, vb, ve, vs] = [0.2, 1.65, -0.05, 0.15];
        let raw_state = q.solve_intrinsic_terminal_state(vc, vb, ve, vs);
        let raw_internal = [
            raw_state.vcx,
            raw_state.vci,
            raw_state.vbx,
            raw_state.vbi,
            raw_state.vei,
            raw_state.vbp,
            raw_state.vsi,
            raw_state.vrth,
        ];
        let limited = q.limit_vbic_internal_state_to_previous(
            [
                raw_state.vcx,
                raw_state.vci,
                raw_state.vbx,
                raw_state.vbi,
                raw_state.vei,
                raw_state.vbp,
                raw_state.vsi,
                raw_state.vrth,
            ],
            previous,
        );
        assert!(
            q.vbic_max_local_branch_delta(raw_internal, limited) > 1e-6,
            "expected large-step VBIC case to engage branch limiting before the final solve"
        );

        q.update(&[ve, vb, vc, vs]);
        let actual = q.internal_state_vector();

        for idx in 0..INTERNAL_DIM {
            assert_relative_or_absolute_within(
                actual[idx],
                raw_internal[idx],
                1e-12,
                1e-12,
                "converged update state",
            );
        }
    }

    #[test]
    fn test_vbic_dynamic_internal_state_limiter_preserves_delay_nodes() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.update(&[0.0, 0.75, 4.1]);

        let [vc, vb, ve, vs] = [4.1, 0.75, 0.0, 0.0];
        let previous = q
            .charge_snapshot(vc, vb, ve, vs)
            .reduction
            .internal_voltages;
        let raw_static = [
            previous[IDX_VCX] + 0.90,
            previous[IDX_VCI] + 0.95,
            previous[IDX_VBX] + 0.60,
            previous[IDX_VBI] + 0.85,
            previous[IDX_VEI] - 0.25,
            previous[IDX_VBP] - 0.35,
            previous[IDX_VSI] + 0.40,
            previous[IDX_VRTH] + 200.0,
        ];
        let mut dynamic_internal = previous;
        dynamic_internal[..INTERNAL_DIM].copy_from_slice(&raw_static);
        dynamic_internal[IDX_VXF1] += 1.0e-6;
        dynamic_internal[IDX_VXF2] -= 2.0e-6;

        let mut previous_static = [0.0; INTERNAL_DIM];
        previous_static.copy_from_slice(&previous[..INTERNAL_DIM]);
        let expected_static = q.limit_vbic_internal_state_to_previous(raw_static, previous_static);
        let limited = q.limit_vbic_dynamic_internal_state_to_previous(dynamic_internal, previous);

        for idx in 0..INTERNAL_DIM {
            assert_relative_or_absolute_within(
                limited[idx],
                expected_static[idx],
                1e-12,
                1e-12,
                "limited dynamic static state",
            );
        }
        assert!((limited[IDX_VXF1] - dynamic_internal[IDX_VXF1]).abs() < 1e-18);
        assert!((limited[IDX_VXF2] - dynamic_internal[IDX_VXF2]).abs() < 1e-18);
    }

    #[test]
    fn test_vbic_collector_substrate_charge_homotopy_variant_scales_nominal_and_thermal_params() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let scale = 0.375;

        let variant = q.vbic_collector_substrate_charge_homotopy_variant(scale);
        assert_relative_or_absolute_within(variant.qco, q.qco * scale, 1e-18, 1e-12, "scaled qco");
        assert_relative_or_absolute_within(
            variant.cjcp,
            q.cjcp * scale,
            1e-18,
            1e-12,
            "scaled cjcp",
        );
        assert_relative_or_absolute_within(
            variant.ccso,
            q.ccso * scale,
            1e-18,
            1e-12,
            "scaled ccso",
        );

        let heated = q.temperature_variant(18.0);
        let heated_variant = variant.temperature_variant(18.0);
        assert_relative_or_absolute_within(
            heated_variant.qco,
            heated.qco * scale,
            1e-18,
            1e-12,
            "scaled heated qco",
        );
        assert_relative_or_absolute_within(
            heated_variant.cjcp,
            heated.cjcp * scale,
            1e-18,
            1e-12,
            "scaled heated cjcp",
        );
        assert_relative_or_absolute_within(
            heated_variant.ccso,
            heated.ccso * scale,
            1e-18,
            1e-12,
            "scaled heated ccso",
        );
    }

    #[test]
    fn test_vbic_external_step_limit_scale_engages_for_large_forward_step() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let previous_external = [4.1, 0.75, 0.0, 0.0];
        q.update(&[
            previous_external[EXT_E],
            previous_external[EXT_B],
            previous_external[EXT_C],
            previous_external[EXT_S],
        ]);

        let proposed_external = [3.6, 1.55, -0.1, 0.2];
        let scale = q
            .vbic_external_step_limit_scale_against_previous(previous_external, proposed_external)
            .expect("expected reduced VBIC limiter to engage");
        assert!(
            scale > 0.0 && scale < 1.0,
            "expected large forward step to be damped, got scale {scale:.6e}"
        );

        let mut scaled_external = proposed_external;
        for _ in 0..3 {
            let Some(pass_scale) = q.vbic_external_step_limit_scale_against_previous(
                previous_external,
                scaled_external,
            ) else {
                break;
            };
            if pass_scale >= 1.0 {
                break;
            }
            scaled_external = [
                previous_external[EXT_C]
                    + pass_scale * (scaled_external[EXT_C] - previous_external[EXT_C]),
                previous_external[EXT_B]
                    + pass_scale * (scaled_external[EXT_B] - previous_external[EXT_B]),
                previous_external[EXT_E]
                    + pass_scale * (scaled_external[EXT_E] - previous_external[EXT_E]),
                previous_external[EXT_S]
                    + pass_scale * (scaled_external[EXT_S] - previous_external[EXT_S]),
            ];
        }
        let residual_scale = q
            .vbic_external_step_limit_scale_against_previous(previous_external, scaled_external)
            .unwrap_or(1.0);
        assert!(
            residual_scale > 0.95,
            "expected scaled proposal to land back inside the VBIC branch limiter envelope, got residual scale {residual_scale:.6e}"
        );
    }

    #[test]
    fn test_vbic_external_step_limit_scale_from_state_matches_cached_helper() {
        let mut params = vbic_reference_params();
        params.insert("RTH".to_string(), 300.0);
        params.insert("SELFT".to_string(), 1.0);
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let previous_external = [4.1, 0.75, 0.0, 0.0];
        q.update(&[
            previous_external[EXT_E],
            previous_external[EXT_B],
            previous_external[EXT_C],
            previous_external[EXT_S],
        ]);

        let proposed_external = [3.6, 1.55, -0.1, 0.2];
        let expected_scale = q
            .vbic_external_step_limit_scale_against_previous(previous_external, proposed_external)
            .expect("expected cached VBIC limiter to engage");
        let state_scale = q
            .vbic_external_step_limit_scale_from_state(
                previous_external,
                q.internal_state_vector(),
                proposed_external,
            )
            .expect("expected state-based VBIC limiter to engage");

        assert_relative_or_absolute_within(
            state_scale,
            expected_scale,
            1e-18,
            1e-12,
            "state-based VBIC continuation scale",
        );
    }

    #[test]
    fn test_vbic_external_step_limit_scale_skips_small_step() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let previous_external = [4.1, 0.75, 0.0, 0.0];
        q.update(&[
            previous_external[EXT_E],
            previous_external[EXT_B],
            previous_external[EXT_C],
        ]);

        let proposed_external = [4.09999, 0.75002, -1.0e-5, 0.0];
        let scale =
            q.vbic_external_step_limit_scale_against_previous(previous_external, proposed_external);
        assert!(
            scale.is_none(),
            "expected small local VBIC step to pass without extra damping, got {scale:?}"
        );
    }

    #[test]
    fn test_bjt_stamp_preserves_kcl_in_jacobian_and_rhs() {
        // Nodes are 1-based in stamps (0 is ground): C=3, B=2, E=1
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1);
        let voltages = vec![0.2, 0.9, 5.0]; // E, B, C
        q.update(&voltages);

        let mut matrix = CaptureMatrix::default();
        let mut rhs = vec![0.0; 3];
        q.stamp_nonlinear(&voltages, &mut matrix, &mut rhs);

        let cc = matrix.g(3, 3);
        let cb = matrix.g(3, 2);
        let ce = matrix.g(3, 1);
        let bc = matrix.g(2, 3);
        let bb = matrix.g(2, 2);
        let be = matrix.g(2, 1);
        let ec = matrix.g(1, 3);
        let eb = matrix.g(1, 2);
        let ee = matrix.g(1, 1);

        // Emitter row must be the negative of collector+base rows (charge conservation)
        assert!((ec + cc + bc).abs() < 1e-12);
        assert!((eb + cb + bb).abs() < 1e-12);
        assert!((ee + ce + be).abs() < 1e-12);

        // Current-source linearization must also satisfy KCL
        let i_sum = matrix.i(3) + matrix.i(2) + matrix.i(1);
        assert!(i_sum.abs() < 1e-12);
    }

    #[test]
    fn test_bjt_linearization_matches_finite_difference_current_derivatives() {
        let mut params = HashMap::new();
        params.insert("IS".to_string(), 1e-15);
        params.insert("BF".to_string(), 120.0);
        params.insert("BR".to_string(), 3.0);
        params.insert("VAF".to_string(), 55.0);
        params.insert("VAR".to_string(), 35.0);
        params.insert("IKF".to_string(), 0.02);
        params.insert("IKR".to_string(), 0.01);

        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let vbe = 0.67;
        let vbc = -0.18;
        let linearized = q.linearize_currents(vbe, vbc);
        let h = 1e-7;

        let (ic_p_vbe, ib_p_vbe, _) = q.calculate_currents(vbe + h, vbc);
        let (ic_m_vbe, ib_m_vbe, _) = q.calculate_currents(vbe - h, vbc);
        let (ic_p_vbc, ib_p_vbc, _) = q.calculate_currents(vbe, vbc + h);
        let (ic_m_vbc, ib_m_vbc, _) = q.calculate_currents(vbe, vbc - h);

        let dic_dvbe_fd = (ic_p_vbe - ic_m_vbe) / (2.0 * h);
        let dic_dvbc_fd = (ic_p_vbc - ic_m_vbc) / (2.0 * h);
        let dib_dvbe_fd = (ib_p_vbe - ib_m_vbe) / (2.0 * h);
        let dib_dvbc_fd = (ib_p_vbc - ib_m_vbc) / (2.0 * h);

        assert!(
            (linearized.dic_dvbe - dic_dvbe_fd).abs() <= dic_dvbe_fd.abs().max(1.0) * 2e-5,
            "collector dIc/dVbe mismatch: analytical={} fd={}",
            linearized.dic_dvbe,
            dic_dvbe_fd
        );
        assert!(
            (linearized.dic_dvbc - dic_dvbc_fd).abs() <= dic_dvbc_fd.abs().max(1.0) * 2e-5,
            "collector dIc/dVbc mismatch: analytical={} fd={}",
            linearized.dic_dvbc,
            dic_dvbc_fd
        );
        assert!(
            (linearized.dib_dvbe - dib_dvbe_fd).abs() <= dib_dvbe_fd.abs().max(1.0) * 2e-5,
            "base dIb/dVbe mismatch: analytical={} fd={}",
            linearized.dib_dvbe,
            dib_dvbe_fd
        );
        assert!(
            (linearized.dib_dvbc - dib_dvbc_fd).abs() <= dib_dvbc_fd.abs().max(1.0) * 2e-5,
            "base dIb/dVbc mismatch: analytical={} fd={}",
            linearized.dib_dvbc,
            dib_dvbc_fd
        );
    }
}
