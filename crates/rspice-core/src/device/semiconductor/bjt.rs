//! BJT (Bipolar Junction Transistor) device model
//!
//! Implements the Ebers-Moll model for NPN and PNP transistors.
//! Supports both large-signal DC and small-signal AC analysis.

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

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
    dib_dvbe: Value,
    dib_dvbc: Value,
    qb: Value,
    dqb_dvbe: Value,
    dqb_dvbc: Value,
}

#[derive(Debug, Clone, Copy)]
struct TransportChargeState {
    qb: Value,
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
struct IntrinsicTerminalState {
    vcx: Value,
    vci: Value,
    vbx: Value,
    vbi: Value,
    vei: Value,
    vbp: Value,
    vsi: Value,
    linearized: BjtLinearization,
}

#[derive(Debug, Clone, Copy, Default)]
struct BranchLinearization {
    current: Value,
    d_internal: [Value; 5],
    d_external: [Value; 3],
}

#[derive(Debug, Clone, Copy)]
struct EvaluatedBjtState {
    linearized: BjtLinearization,
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
    ic: Value,
    ib: Value,
    ie: Value,
    isub: Value,
}

type BjtRowCoefficients = (Value, Value, Value);

const INTERNAL_DIM: usize = 5;
const EXTERNAL_DIM: usize = 3;
const IDX_VCX: usize = 0;
const IDX_VCI: usize = 1;
const IDX_VBX: usize = 2;
const IDX_VBI: usize = 3;
const IDX_VEI: usize = 4;
const IDX_VBP: usize = 5;
const IDX_VSI: usize = 6;
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
    /// Temperature exponent of IS (XIS)
    pub xis: Value,
    /// Temperature exponent of IBEI/IBCI (XII)
    pub xii: Value,
    /// Temperature exponent of IBEN/IBCN (XIN)
    pub xin: Value,
    /// Temperature exponent of ISRR (XISR)
    pub xisr: Value,
    /// Activation energy for IS (EA)
    pub ea: Value,
    /// Activation energy for IBEI (EAIE)
    pub eaie: Value,
    /// Activation energy for IBCI (EAIC)
    pub eaic: Value,
    /// Activation energy for IBEN (EANE)
    pub eane: Value,
    /// Activation energy for IBCN (EANC)
    pub eanc: Value,
    /// Delta activation energy for ISRR (DEAR)
    pub dear: Value,

    // Gummel-Poon charge model parameters
    /// Zero-bias B-E junction capacitance (CJE)
    pub cje: Value,
    /// B-E built-in potential (VJE)
    pub mje: Value,
    /// Zero-bias B-C junction capacitance (CJC)
    pub cjc: Value,
    /// Zero-bias collector-substrate capacitance (CJCP)
    pub cjcp: Value,
    /// B-C grading coefficient (MJC)
    pub mjc: Value,
    /// Forward transit time (TF)
    pub tf: Value,
    /// Reverse transit time (TR)
    pub tr: Value,
    /// Knee current for high-level injection (IKF)
    pub ikf: Value,
    /// Reverse knee current (IKR)
    pub ikr: Value,
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

    /// Nominal saturation current before area/multiplicity and temp scaling
    is_nominal: Value,
    /// Nominal zero-bias B-E capacitance before area/multiplicity scaling
    cje_nominal: Value,
    /// Nominal zero-bias B-C capacitance before area/multiplicity scaling
    cjc_nominal: Value,
    /// Nominal zero-bias collector-substrate capacitance before scaling
    cjcp_nominal: Value,
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

    /// Pre-computed matrix indices for O(1) stamping
    pub indices: BjtIndices,
}

impl Bjt {
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
            tnom: 300.0,
            temperature: 300.0,
            xti: 3.0,
            eg: 1.11,
            vje: 0.75,          // B-E built-in potential
            vjc: 0.75,          // B-C built-in potential
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
            xis: 3.0,
            xii: 3.0,
            xin: 3.0,
            xisr: 0.0,
            ea: 1.12,
            eaie: 1.12,
            eaic: 1.12,
            eane: 1.12,
            eanc: 1.12,
            dear: 0.0,

            // Gummel-Poon parameters
            cje: 1e-12,   // B-E junction capacitance
            mje: 0.33,    // B-E grading coefficient
            cjc: 0.5e-12, // B-C junction capacitance
            cjcp: 0.0,    // C-S junction capacitance
            mjc: 0.33,    // B-C grading coefficient
            tf: 4e-10,    // Forward transit time (400ps)
            tr: 5e-9,     // Reverse transit time (5ns)
            ikf: 0.1,     // Knee current (100mA)
            ikr: 0.01,    // Reverse knee
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
            is_nominal: 1e-14,
            cje_nominal: 1e-12,
            cjc_nominal: 0.5e-12,
            cjcp_nominal: 0.0,
            ikf_nominal: 0.1,
            ikr_nominal: 0.01,
            isrr_nominal: 1.0,
            ibei_nominal: 5e-17,
            iben_nominal: 0.0,
            ibci_nominal: 1e-14,
            ibcn_nominal: 0.0,
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
            indices: BjtIndices::default(),
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
    fn effective_temperature(&self) -> Value {
        let base = self.instance_temp.unwrap_or(self.temperature);
        (base + self.instance_dtemp).max(1.0)
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

    fn refresh_operating_scaling(&mut self) {
        let temp = self.effective_temperature();
        let tnom = self.tnom.max(1.0);
        let vt = Self::thermal_voltage_at(temp);
        let ratio = (temp / tnom).max(1e-12);
        let is_temp =
            Self::vbic_temp_scaled_current(self.is_nominal, ratio, vt, self.xis, self.ea, self.nf);
        let scale = self.instance_scale();
        let isrr_temp = Self::vbic_temp_scaled_current(
            self.isrr_nominal,
            ratio,
            vt,
            self.xisr,
            self.ea + self.dear,
            self.nr,
        );
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

        self.vt = vt;
        self.temperature = temp;
        self.is = (is_temp * scale).max(1e-30);
        self.cje = (self.cje_nominal * scale).max(0.0);
        self.cjc = (self.cjc_nominal * scale).max(0.0);
        self.cjcp = (self.cjcp_nominal * scale).max(0.0);
        self.ikf = (self.ikf_nominal * scale).max(1e-18);
        self.ikr = (self.ikr_nominal * scale).max(1e-18);
        self.isrr = isrr_temp.max(0.0);
        self.ibei = (ibei_temp * scale).max(0.0);
        self.iben = (iben_temp * scale).max(0.0);
        self.ibci = (ibci_temp * scale).max(0.0);
        self.ibcn = (ibcn_temp * scale).max(0.0);
    }

    /// Set active device temperature (Kelvin).
    pub fn set_temperature(&mut self, temp_k: Value) {
        if temp_k.is_finite() && temp_k > 0.0 {
            self.temperature = temp_k;
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
            self.nf = v;
        }
        if let Some(&v) = params.get("NR") {
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
            self.rbi = 0.0;
            self.rb = self.rbx;
            has_rb = true;
        }
        if let Some(&v) = params.get("RC") {
            self.rcx = v.max(0.0);
            self.rci = 0.0;
            self.rc = self.rcx;
            has_rc = true;
        }
        if let Some(&v) = params.get("RE") {
            self.re = v;
        }
        if let Some(&v) = params.get("RS") {
            self.rs = v.max(0.0);
        }
        if let Some(&v) = params.get("RBP") {
            self.rbp = v.max(0.0);
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
        if let Some(&v) = params.get("EG")
            && v.is_finite()
            && v > 0.0
        {
            self.eg = v;
            self.ea = v;
            self.eaie = v;
            self.eaic = v;
            self.eane = v;
            self.eanc = v;
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
                self.rc = (rcx + rci).max(1e-12);
            }
        }
        if let Some(&v) = params.get("VO")
            && v.is_finite()
            && v >= 0.0
        {
            self.vo = v;
        }
        if let Some(&v) = params.get("GAMM")
            && v.is_finite()
            && v >= 0.0
        {
            self.gamm = v;
        }
        if let Some(&v) = params.get("HRCF")
            && v.is_finite()
            && v > 0.0
        {
            self.hrcf = v;
        }
        // Gummel-Poon charge parameters
        if let Some(&v) = params.get("CJE") {
            self.cje_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CJEP") {
            // VBIC peripheral BE capacitance is mainly tied to internal/peripheral
            // nodes that are not explicitly represented in this 3-terminal model.
            self.cje_nominal += 0.0 * v.max(0.0);
        }
        if let Some(&v) = params.get("MJE") {
            self.mje = v;
        }
        if let Some(&v) = params.get("CJC") {
            self.cjc_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CJCP") {
            // Collector-periphery capacitance is mostly substrate-coupled and is
            // omitted in this reduced 3-terminal abstraction.
            self.cjcp_nominal = 0.0 * v.max(0.0);
        }
        if let Some(&v) = params.get("MJC") {
            self.mjc = v;
        }
        if let Some(&v) = params.get("TF") {
            self.tf = v;
        }
        if let Some(&v) = params.get("TR") {
            self.tr = v;
        }
        if let Some(&v) = params.get("IKF") {
            self.ikf_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IKR") {
            self.ikr_nominal = v.max(0.0);
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
            self.isp = v.max(0.0);
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
            self.ibeip = v.max(0.0);
        }
        if let Some(&v) = params.get("IBENP") {
            self.ibenp = v.max(0.0);
        }
        if let Some(&v) = params.get("IBCIP") {
            self.ibcip = v.max(0.0);
        }
        if let Some(&v) = params.get("IBCNP") {
            self.ibcnp = v.max(0.0);
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
        if !has_ibei {
            self.ibei_nominal = self.is_nominal / self.bf.max(1e-18);
        }
        if !has_ibci {
            self.ibci_nominal = self.is_nominal / self.br.max(1e-18);
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
        let v = (p * vbe).min(0.9 * self.vje); // Clamp to avoid singularity
        let cj = self.cje / (1.0 - v / self.vje).powf(self.mje);
        let cd = gm * self.tf; // Diffusion capacitance
        cj + cd
    }

    /// Calculate base-collector junction capacitance
    /// Cbc = CJC / (1 - Vbc/VJC)^MJC
    pub fn cbc(&self, vbc: Value) -> Value {
        let p = self.polarity();
        let v = (p * vbc).min(0.9 * self.vjc); // Clamp to avoid singularity
        self.cjc / (1.0 - v / self.vjc).powf(self.mjc)
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
        let vc = if self.node_collector == 0 {
            0.0
        } else {
            voltages[self.node_collector - 1]
        };
        let vb = if self.node_base == 0 {
            0.0
        } else {
            voltages[self.node_base - 1]
        };
        let ve = if self.node_emitter == 0 {
            0.0
        } else {
            voltages[self.node_emitter - 1]
        };

        let (collector, base, emitter) = self.small_signal_row_coefficients(vc, vb, ve);

        // Equivalent current sources for the linearized node-current equations.
        let ic_eq = self.ic - (collector.0 * vc + collector.1 * vb + collector.2 * ve);
        let ib_eq = self.ib - (base.0 * vc + base.1 * vb + base.2 * ve);

        // Debug logging for Newton convergence analysis (commented for performance)
        // log::trace!(
        //     "BJT {}: Vc={:.3} Vb={:.3} Ve={:.3} | VBE={:.3} VBC={:.3} | Ic={:.2e} Ib={:.2e} | gm={:.2e} go={:.2e} | ic_eq={:.2e} ib_eq={:.2e}",
        //     self.name, vc, vb, ve, vbe, vbc, ic, ib, gm, go, ic_eq, ib_eq
        // );

        // Stamp matrix using direct indexing
        // Collector row
        if let Some(idx) = self.indices.cc {
            matrix.stamp_direct(idx, collector.0);
        }
        if let Some(idx) = self.indices.cb {
            matrix.stamp_direct(idx, collector.1);
        }
        if let Some(idx) = self.indices.ce {
            matrix.stamp_direct(idx, collector.2);
        }
        // Base row
        if let Some(idx) = self.indices.bc {
            matrix.stamp_direct(idx, base.0);
        }
        if let Some(idx) = self.indices.bb {
            matrix.stamp_direct(idx, base.1);
        }
        if let Some(idx) = self.indices.be {
            matrix.stamp_direct(idx, base.2);
        }
        // Emitter row
        if let Some(idx) = self.indices.ec {
            matrix.stamp_direct(idx, emitter.0);
        }
        if let Some(idx) = self.indices.eb {
            matrix.stamp_direct(idx, emitter.1);
        }
        if let Some(idx) = self.indices.ee {
            matrix.stamp_direct(idx, emitter.2);
        }

        // Stamp RHS - current flowing OUT of node is positive in the equation
        // BJT collector current flows from collector to emitter
        if self.node_collector > 0 {
            rhs[self.node_collector - 1] -= ic_eq;
        }
        if self.node_base > 0 {
            rhs[self.node_base - 1] -= ib_eq;
        }
        if self.node_emitter > 0 {
            rhs[self.node_emitter - 1] += ic_eq + ib_eq;
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
    fn depletion_charge_and_derivative(
        &self,
        junction_voltage_eff: Value,
        potential: Value,
        grading: Value,
    ) -> (Value, Value) {
        let phi = potential.max(1e-12);
        let v = junction_voltage_eff.min(0.9 * phi);
        let one_minus = (1.0 - v / phi).max(1e-12);

        if (1.0 - grading).abs() < 1e-12 {
            (-phi * one_minus.ln(), 1.0 / one_minus)
        } else {
            let exponent = 1.0 - grading;
            let pow = one_minus.powf(exponent);
            (
                phi * (1.0 - pow) / exponent,
                one_minus.powf(-grading),
            )
        }
    }

    fn transport_charge_state(&self, vbe_eff: Value, vbc_eff: Value) -> TransportChargeState {
        let ifi = self.diode_current(vbe_eff, self.nf).max(0.0);
        let iri = self
            .diode_current_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr)
            .max(0.0);
        let gfi = self.diode_conductance(vbe_eff, self.nf);
        let gri = self.diode_conductance_with_is(self.is * self.isrr.max(0.0), vbc_eff, self.nr);

        let (qdbe, dqdbe_dvbe_eff) =
            self.depletion_charge_and_derivative(vbe_eff, self.vje, self.mje);
        let (qdbc, dqdbc_dvbc_eff) =
            self.depletion_charge_and_derivative(vbc_eff, self.vjc, self.mjc);

        let q1z = 1.0
            + if self.var.is_finite() && self.var > 0.0 {
                qdbe / self.var
            } else {
                0.0
            }
            + if self.vaf.is_finite() && self.vaf > 0.0 {
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

        let q2 = ifi / self.ikf.max(1e-18) + iri / self.ikr.max(1e-18);
        let dq2_dvbe_eff = gfi / self.ikf.max(1e-18);
        let dq2_dvbc_eff = gri / self.ikr.max(1e-18);

        let qb_sqrt = (q1 * q1 + 4.0 * q2).sqrt().max(1e-18);
        let qb = (0.5 * (q1 + qb_sqrt)).max(1e-12);
        let dqb_dvbe_eff =
            0.5 * dq1_dvbe_eff + 0.5 * (q1 * dq1_dvbe_eff + 2.0 * dq2_dvbe_eff) / qb_sqrt;
        let dqb_dvbc_eff =
            0.5 * dq1_dvbc_eff + 0.5 * (q1 * dq1_dvbc_eff + 2.0 * dq2_dvbc_eff) / qb_sqrt;

        let itzf = ifi / qb;
        let ditzf_dvbe_eff = gfi / qb - ifi * dqb_dvbe_eff / (qb * qb);
        let ditzf_dvbc_eff = -ifi * dqb_dvbc_eff / (qb * qb);

        let itzr = iri / qb;
        let ditzr_dvbe_eff = -iri * dqb_dvbe_eff / (qb * qb);
        let ditzr_dvbc_eff = gri / qb - iri * dqb_dvbc_eff / (qb * qb);

        TransportChargeState {
            qb,
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

    fn linearize_currents(&self, vbe: Value, vbc: Value) -> BjtLinearization {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;
        let transport = self.transport_charge_state(vbe_eff, vbc_eff);

        let ib_be = self.diode_current_with_is(self.ibei, vbe_eff, self.nei)
            + self.diode_current_with_is(self.iben, vbe_eff, self.nen);
        let ib_bc = self.diode_current_with_is(self.ibci, vbc_eff, self.nci)
            + self.diode_current_with_is(self.ibcn, vbc_eff, self.ncn);
        let dibe_dvbe = self.gbe(vbe);
        let dibc_dvbc = self.gbc(vbc);
        let iciei = transport.itzf - transport.itzr;
        let diciei_dvbe = transport.ditzf_dvbe_eff - transport.ditzr_dvbe_eff;
        let diciei_dvbc = transport.ditzf_dvbc_eff - transport.ditzr_dvbc_eff;

        BjtLinearization {
            // The intrinsic collector terminal sees both the transport branch
            // (collector to emitter) and the opposing B-C junction branch.
            ic: p * (iciei - ib_bc),
            ib: p * (ib_be + ib_bc),
            dic_dvbe: diciei_dvbe,
            dic_dvbc: diciei_dvbc - dibc_dvbc,
            dib_dvbe: dibe_dvbe,
            dib_dvbc: dibc_dvbc,
            qb: transport.qb,
            dqb_dvbe: p * transport.dqb_dvbe_eff,
            dqb_dvbc: p * transport.dqb_dvbc_eff,
        }
    }

    #[inline]
    fn collector_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        (
            -linearized.dic_dvbc,
            linearized.dic_dvbe + linearized.dic_dvbc,
            -linearized.dic_dvbe,
        )
    }

    #[inline]
    fn base_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        (
            -linearized.dib_dvbc,
            linearized.dib_dvbe + linearized.dib_dvbc,
            -linearized.dib_dvbe,
        )
    }

    #[inline]
    fn emitter_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        let (cc, cb, ce) = self.collector_row_coefficients(linearized);
        let (bc, bb, be) = self.base_row_coefficients(linearized);
        (-(cc + bc), -(cb + bb), -(ce + be))
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
    ) -> ([Value; 5], [Value; 5], [Value; 5]) {
        let mut collector = [0.0; 5];
        collector[IDX_VCI] = -linearized.dic_dvbc;
        collector[IDX_VBI] = linearized.dic_dvbe + linearized.dic_dvbc;
        collector[IDX_VEI] = -linearized.dic_dvbe;

        let mut base = [0.0; 5];
        base[IDX_VCI] = -linearized.dib_dvbc;
        base[IDX_VBI] = linearized.dib_dvbe + linearized.dib_dvbc;
        base[IDX_VEI] = -linearized.dib_dvbe;

        let mut emitter = [0.0; 5];
        for idx in 0..5 {
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
        let d_iohm_dvbci_eff =
            vt * d_kbci_dvbci_eff * (1.0 - d_log_ratio_dkbci) / rci;
        let d_iohm_dvbcx_eff =
            vt * d_kbcx_dvbcx_eff * (-1.0 - d_log_ratio_dkbcx) / rci;

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

    fn evaluate_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vcx: Value,
        vci: Value,
        vbx: Value,
        vbi: Value,
        vei: Value,
    ) -> EvaluatedBjtState {
        let linearized = self.linearize_currents(vbi - vei, vbi - vci);
        EvaluatedBjtState {
            linearized,
            ircx: self.ircx_branch(vc, vcx),
            irci: self.irci_branch(vcx, vci, vbi),
            irbx: self.irbx_branch(vb, vbx),
            irbi: self.irbi_branch(linearized, vbx, vbi),
            ire: self.ire_branch(ve, vei),
            ibep: BranchLinearization::default(),
            irbp: BranchLinearization::default(),
            ibcp: BranchLinearization::default(),
            iccp: BranchLinearization::default(),
            irs: BranchLinearization::default(),
        }
    }

    fn solve_small_dense_system(
        matrix: &[[Value; 5]; 5],
        rhs: &[Value; 5],
        dim: usize,
    ) -> Option<[Value; 5]> {
        if dim == 0 {
            return Some([0.0; 5]);
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

        let mut x = [0.0; 5];
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
    ) -> IntrinsicTerminalState {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);

        let mut vcx = if self.vcx.is_finite() {
            self.vcx
        } else if has_rcx {
            vc - self.ic * self.rcx.max(0.0)
        } else {
            vc
        };
        let mut vci = if self.vci.is_finite() {
            self.vci
        } else if has_rci {
            vcx - self.ic * self.rci.max(0.0)
        } else {
            vcx
        };
        let mut vbx = if self.vbx.is_finite() {
            self.vbx
        } else if has_rbx {
            vb - self.ib * self.rbx.max(0.0)
        } else {
            vb
        };
        let mut vbi = if self.vbi.is_finite() {
            self.vbi
        } else if has_rbi {
            vbx - self.ib * self.rbi.max(0.0)
        } else {
            vbx
        };
        let mut vei = if self.vei.is_finite() {
            self.vei
        } else if has_re {
            ve - self.ie * self.re.max(0.0)
        } else {
            ve
        };

        for _ in 0..16 {
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

            let eval = self.evaluate_state(vc, vb, ve, vcx, vci, vbx, vbi, vei);
            let (collector_d, base_d, emitter_d) =
                self.intrinsic_terminal_derivatives(eval.linearized);
            let ie_intrinsic = -(eval.linearized.ic + eval.linearized.ib);

            let mut jacobian = [[0.0; 5]; 5];
            let mut residual = [0.0; 5];

            if has_rcx {
                residual[IDX_VCX] = eval.ircx.current
                    - if has_rci {
                        eval.irci.current
                    } else {
                        eval.linearized.ic
                    };
                for idx in 0..5 {
                    jacobian[IDX_VCX][idx] = eval.ircx.d_internal[idx]
                        - if has_rci {
                            eval.irci.d_internal[idx]
                        } else {
                            collector_d[idx]
                        };
                }
            } else {
                residual[IDX_VCX] = vcx - vc;
                jacobian[IDX_VCX][IDX_VCX] = 1.0;
            }

            if has_rci {
                residual[IDX_VCI] = eval.irci.current - eval.linearized.ic;
                for idx in 0..5 {
                    jacobian[IDX_VCI][idx] = eval.irci.d_internal[idx] - collector_d[idx];
                }
            } else {
                residual[IDX_VCI] = vci - vcx;
                jacobian[IDX_VCI][IDX_VCI] = 1.0;
                jacobian[IDX_VCI][IDX_VCX] = -1.0;
            }

            if has_rbx {
                residual[IDX_VBX] = eval.irbx.current
                    - if has_rbi {
                        eval.irbi.current
                    } else {
                        eval.linearized.ib
                    };
                for idx in 0..5 {
                    jacobian[IDX_VBX][idx] = eval.irbx.d_internal[idx]
                        - if has_rbi {
                            eval.irbi.d_internal[idx]
                        } else {
                            base_d[idx]
                        };
                }
            } else {
                residual[IDX_VBX] = vbx - vb;
                jacobian[IDX_VBX][IDX_VBX] = 1.0;
            }

            if has_rbi {
                residual[IDX_VBI] = eval.irbi.current - eval.linearized.ib;
                for idx in 0..5 {
                    jacobian[IDX_VBI][idx] = eval.irbi.d_internal[idx] - base_d[idx];
                }
            } else {
                residual[IDX_VBI] = vbi - vbx;
                jacobian[IDX_VBI][IDX_VBI] = 1.0;
                jacobian[IDX_VBI][IDX_VBX] = -1.0;
            }

            if has_re {
                residual[IDX_VEI] = eval.ire.current - ie_intrinsic;
                for idx in 0..5 {
                    jacobian[IDX_VEI][idx] = eval.ire.d_internal[idx] - emitter_d[idx];
                }
            } else {
                residual[IDX_VEI] = vei - ve;
                jacobian[IDX_VEI][IDX_VEI] = 1.0;
            }

            let rhs = [
                -residual[IDX_VCX],
                -residual[IDX_VCI],
                -residual[IDX_VBX],
                -residual[IDX_VBI],
                -residual[IDX_VEI],
            ];
            let Some(delta) = Self::solve_small_dense_system(&jacobian, &rhs, 5) else {
                break;
            };

            let mut max_delta: Value = 0.0;
            for (value, idx) in [
                (&mut vcx, IDX_VCX),
                (&mut vci, IDX_VCI),
                (&mut vbx, IDX_VBX),
                (&mut vbi, IDX_VBI),
                (&mut vei, IDX_VEI),
            ] {
                let step = delta[idx].clamp(-0.1, 0.1);
                *value += step;
                max_delta = max_delta.max(step.abs());
            }

            if max_delta < 1e-12 {
                break;
            }
        }

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
        let linearized = self.linearize_currents(vbi - vei, vbi - vci);

        IntrinsicTerminalState {
            vcx,
            vci,
            vbx,
            vbi,
            vei,
            vbp: self.vbp,
            vsi: self.vsi,
            linearized,
        }
    }

    fn internal_voltage_sensitivities(
        &self,
        state: IntrinsicTerminalState,
        vc: Value,
        vb: Value,
        ve: Value,
    ) -> [[Value; 3]; 5] {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);

        let eval = self.evaluate_state(
            vc, vb, ve, state.vcx, state.vci, state.vbx, state.vbi, state.vei,
        );
        let (collector_d, base_d, emitter_d) =
            self.intrinsic_terminal_derivatives(eval.linearized);
        let ie_intrinsic = -(eval.linearized.ic + eval.linearized.ib);

        let mut jacobian = [[0.0; 5]; 5];
        let mut external_partials = [[0.0; 3]; 5];

        if has_rcx {
            for idx in 0..5 {
                jacobian[IDX_VCX][idx] = eval.ircx.d_internal[idx]
                    - if has_rci {
                        eval.irci.d_internal[idx]
                    } else {
                        collector_d[idx]
                    };
            }
            external_partials[IDX_VCX] = eval.ircx.d_external;
        } else {
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
            external_partials[IDX_VCX][0] = -1.0;
        }

        if has_rci {
            for idx in 0..5 {
                jacobian[IDX_VCI][idx] = eval.irci.d_internal[idx] - collector_d[idx];
            }
        } else {
            jacobian[IDX_VCI][IDX_VCI] = 1.0;
            jacobian[IDX_VCI][IDX_VCX] = -1.0;
        }

        if has_rbx {
            for idx in 0..5 {
                jacobian[IDX_VBX][idx] = eval.irbx.d_internal[idx]
                    - if has_rbi {
                        eval.irbi.d_internal[idx]
                    } else {
                        base_d[idx]
                    };
            }
            external_partials[IDX_VBX] = eval.irbx.d_external;
        } else {
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
            external_partials[IDX_VBX][1] = -1.0;
        }

        if has_rbi {
            for idx in 0..5 {
                jacobian[IDX_VBI][idx] = eval.irbi.d_internal[idx] - base_d[idx];
            }
        } else {
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            for idx in 0..5 {
                jacobian[IDX_VEI][idx] = eval.ire.d_internal[idx] - emitter_d[idx];
            }
            external_partials[IDX_VEI] = eval.ire.d_external;
        } else {
            let _ = ie_intrinsic;
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
            external_partials[IDX_VEI][2] = -1.0;
        }

        let mut sensitivities = [[0.0; 3]; 5];
        for external in 0..3 {
            let rhs = [
                -external_partials[IDX_VCX][external],
                -external_partials[IDX_VCI][external],
                -external_partials[IDX_VBX][external],
                -external_partials[IDX_VBI][external],
                -external_partials[IDX_VEI][external],
            ];
            if let Some(solution) = Self::solve_small_dense_system(&jacobian, &rhs, 5) {
                for idx in 0..5 {
                    sensitivities[idx][external] = solution[idx];
                }
            }
        }

        sensitivities
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
    ) -> (BjtRowCoefficients, BjtRowCoefficients, BjtRowCoefficients) {
        let same_cached_bias = |now: Value, cached: Value| {
            (now - cached).abs() <= f64::EPSILON * now.abs().max(cached.abs()).max(1.0)
        };
        let state = if same_cached_bias(vc, self.vc_ext)
            && same_cached_bias(vb, self.vb_ext)
            && same_cached_bias(ve, self.ve_ext)
        {
            IntrinsicTerminalState {
                vcx: self.vcx,
                vci: self.vci,
                vbx: self.vbx,
                vbi: self.vbi,
                vei: self.vei,
                vbp: self.vbp,
                vsi: self.vsi,
                linearized: self.linearize_currents(self.vbe, self.vbc),
            }
        } else {
            self.solve_intrinsic_terminal_state(vc, vb, ve)
        };
        let sensitivities = self.internal_voltage_sensitivities(state, vc, vb, ve);
        let eval = self.evaluate_state(
            vc, vb, ve, state.vcx, state.vci, state.vbx, state.vbi, state.vei,
        );
        let (collector_internal, base_internal, emitter_internal) =
            self.intrinsic_terminal_derivatives(eval.linearized);

        let collector_current = if Self::series_active(self.rcx) {
            eval.ircx
        } else if Self::series_active(self.rci) {
            eval.irci
        } else {
            BranchLinearization {
                current: eval.linearized.ic,
                d_internal: collector_internal,
                d_external: [0.0; 3],
            }
        };
        let base_current = if Self::series_active(self.rbx) {
            eval.irbx
        } else if Self::series_active(self.rbi) {
            eval.irbi
        } else {
            BranchLinearization {
                current: eval.linearized.ib,
                d_internal: base_internal,
                d_external: [0.0; 3],
            }
        };
        let emitter_current = if Self::series_active(self.re) {
            eval.ire
        } else {
            BranchLinearization {
                current: -(eval.linearized.ic + eval.linearized.ib),
                d_internal: emitter_internal,
                d_external: [0.0; 3],
            }
        };

        let current_row = |branch: BranchLinearization| {
            let mut row = [0.0; 3];
            for external in 0..3 {
                row[external] = branch.d_external[external];
                for internal in 0..5 {
                    row[external] +=
                        branch.d_internal[internal] * sensitivities[internal][external];
                }
            }
            (row[0], row[1], row[2])
        };

        let collector = current_row(collector_current);
        let base = current_row(base_current);
        let emitter = current_row(emitter_current);

        (collector, base, emitter)
    }

    pub(crate) fn stamp_small_signal_ac(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let vc = if self.node_collector == 0 {
            0.0
        } else {
            voltages[self.node_collector - 1]
        };
        let vb = if self.node_base == 0 {
            0.0
        } else {
            voltages[self.node_base - 1]
        };
        let ve = if self.node_emitter == 0 {
            0.0
        } else {
            voltages[self.node_emitter - 1]
        };

        let (collector, base, emitter) = self.small_signal_row_coefficients(vc, vb, ve);
        matrix.stamp(self.node_collector, self.node_collector, collector.0);
        matrix.stamp(self.node_collector, self.node_base, collector.1);
        matrix.stamp(self.node_collector, self.node_emitter, collector.2);

        matrix.stamp(self.node_base, self.node_collector, base.0);
        matrix.stamp(self.node_base, self.node_base, base.1);
        matrix.stamp(self.node_base, self.node_emitter, base.2);

        matrix.stamp(self.node_emitter, self.node_collector, emitter.0);
        matrix.stamp(self.node_emitter, self.node_base, emitter.1);
        matrix.stamp(self.node_emitter, self.node_emitter, emitter.2);
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

        // High-injection correction factor
        let ikf_ratio = if_diode.max(0.0) / self.ikf.max(1e-6);
        let hf = 1.0 / (1.0 + ikf_ratio);

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
    #[allow(dead_code)]
    fn limit_junction_voltage(vnew: Value, vold: Value, vt: Value) -> Value {
        // Critical voltage: above this, exponential becomes problematic
        let vcrit = vt * (vt / (core::f64::consts::SQRT_2 * 1e-14)).ln();

        // If new voltage is below critical, accept it (reverse bias is stable)
        if vnew < vcrit {
            return vnew;
        }

        // For forward bias above critical voltage, use logarithmic limiting
        // This prevents huge jumps that would cause exp() overflow
        let delta = vnew - vold;

        if delta.abs() <= 2.0 * vt {
            // Small change - accept as-is
            vnew
        } else if vold >= 0.0 {
            // Forward bias case: limit using logarithmic function
            // New voltage = old + Vt * (1 + ln((delta/Vt - 1).max(1e-10)))
            let arg = (delta / vt - 1.0).abs().max(1e-10);
            if delta > 0.0 {
                vold + vt * (1.0 + arg.ln())
            } else {
                vold - vt * (1.0 + arg.ln())
            }
        } else {
            // Transition from reverse to forward - be conservative
            // Limit to 2*Vt step toward forward bias
            if vnew > 0.0 {
                vt.min(vnew) // Don't exceed Vt on first forward step
            } else {
                vnew.max(vold - 2.0 * vt) // Limit reverse-to-less-reverse
            }
        }
    }
}

impl NonlinearDevice for Bjt {
    fn update(&mut self, voltages: &[Value]) {
        let vc = if self.node_collector == 0 {
            0.0
        } else {
            voltages[self.node_collector - 1]
        };
        let vb = if self.node_base == 0 {
            0.0
        } else {
            voltages[self.node_base - 1]
        };
        let ve = if self.node_emitter == 0 {
            0.0
        } else {
            voltages[self.node_emitter - 1]
        };

        self.vbe_prev = self.vbe;
        self.vbc_prev = self.vbc;
        self.vcx_prev = self.vcx;
        self.vbi_prev = self.vbi;
        self.vci_prev = self.vci;
        self.vbx_prev = self.vbx;
        self.vei_prev = self.vei;

        let state = self.solve_intrinsic_terminal_state(vc, vb, ve);
        let eval = self.evaluate_state(
            vc, vb, ve, state.vcx, state.vci, state.vbx, state.vbi, state.vei,
        );
        self.vc_ext = vc;
        self.vb_ext = vb;
        self.ve_ext = ve;
        self.vcx = state.vcx;
        self.vci = state.vci;
        self.vbx = state.vbx;
        self.vbi = state.vbi;
        self.vei = state.vei;
        self.vbe = self.vbi - self.vei;
        self.vbc = self.vbi - self.vci;

        self.ic = if Self::series_active(self.rcx) {
            eval.ircx.current
        } else if Self::series_active(self.rci) {
            eval.irci.current
        } else {
            eval.linearized.ic
        };
        self.ib = if Self::series_active(self.rbx) {
            eval.irbx.current
        } else if Self::series_active(self.rbi) {
            eval.irbi.current
        } else {
            eval.linearized.ib
        };
        self.ie = if Self::series_active(self.re) {
            eval.ire.current
        } else {
            -(eval.linearized.ic + eval.linearized.ib)
        };
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let vc = if self.node_collector == 0 {
            0.0
        } else {
            voltages[self.node_collector - 1]
        };
        let vb = if self.node_base == 0 {
            0.0
        } else {
            voltages[self.node_base - 1]
        };
        let ve = if self.node_emitter == 0 {
            0.0
        } else {
            voltages[self.node_emitter - 1]
        };

        let (collector, base, emitter) = self.small_signal_row_coefficients(vc, vb, ve);

        // Equivalent currents for the linearized node-current equations.
        let ic_eq = self.ic - (collector.0 * vc + collector.1 * vb + collector.2 * ve);
        let ib_eq = self.ib - (base.0 * vc + base.1 * vb + base.2 * ve);

        // Stamp the linearized model
        // Collector node equation
        matrix.stamp(self.node_collector, self.node_collector, collector.0);
        matrix.stamp(self.node_collector, self.node_base, collector.1);
        matrix.stamp(self.node_collector, self.node_emitter, collector.2);

        // Base node equation
        matrix.stamp(self.node_base, self.node_collector, base.0);
        matrix.stamp(self.node_base, self.node_base, base.1);
        matrix.stamp(self.node_base, self.node_emitter, base.2);

        // Emitter node equation
        matrix.stamp(self.node_emitter, self.node_collector, emitter.0);
        matrix.stamp(self.node_emitter, self.node_base, emitter.1);
        matrix.stamp(self.node_emitter, self.node_emitter, emitter.2);

        // Stamp equivalent current sources
        matrix.stamp_rhs(self.node_collector, -ic_eq);
        matrix.stamp_rhs(self.node_base, -ib_eq);
        matrix.stamp_rhs(self.node_emitter, ic_eq + ib_eq);
    }

    /// Check Newton-Raphson convergence using SPICE-style voltage criteria.
    ///
    /// Uses the standard SPICE convergence test:
    ///   |delta(V)| < RELTOL * max(|V_new|, |V_old|) + VNTOL
    ///
    /// `tolerance` is VNTOL from solver configuration.
    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        // Industry-standard SPICE convergence parameter
        const RELTOL: Value = 1e-3; // 0.1% relative tolerance
        let tolerance = criteria.voltage_tolerance();

        let vbe_diff = (self.vbe - self.vbe_prev).abs();
        let vbc_diff = (self.vbc - self.vbc_prev).abs();

        // SPICE criterion: |delta(V)| < RELTOL * max(|V_new|, |V_old|) + VNTOL
        let vbe_tol = RELTOL * self.vbe.abs().max(self.vbe_prev.abs()) + tolerance;
        let vbc_tol = RELTOL * self.vbc.abs().max(self.vbc_prev.abs()) + tolerance;

        vbe_diff < vbe_tol && vbc_diff < vbc_tol
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
        assert!((q.cjcp - 0.0).abs() < 1e-20);
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

    #[test]
    fn test_bjt_vbic_gummel_reference_points_track_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_pnp("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);

        q.update(&[0.3, 0.0, 0.0]);
        let (ic_03, ib_03, _) = q.operating_point_currents();
        assert_relative_within(ic_03.abs(), 1.007807e-11, 0.08, "FG ic @ 0.3V");
        assert_relative_within(ib_03.abs(), 1.814379e-12, 0.08, "FG ib @ 0.3V");

        q.update(&[0.7, 0.0, 0.0]);
        let (ic_07, ib_07, _) = q.operating_point_currents();
        assert_relative_within(ic_07.abs(), 4.491451e-05, 0.05, "FG ic @ 0.7V");
        assert_relative_within(ib_07.abs(), 5.682497e-07, 0.05, "FG ib @ 0.7V");
    }

    #[test]
    fn test_bjt_vbic_output_reference_point_tracks_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(300.15);

        q.update(&[0.0, 0.7, 0.0]);
        let (ic, ib, _) = q.operating_point_currents();
        assert_relative_within(ic.abs(), 9.59413e-06, 0.08, "FO ic @ vb=0.7 vc=0.0");
        assert_relative_within(ib.abs(), 5.682510e-07, 0.08, "FO ib @ vb=0.7 vc=0.0");
    }

    #[test]
    fn test_bjt_vbic_temperature_reference_points_track_ngspice() {
        let params = vbic_reference_params();
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        q.set_temperature(423.15);

        q.update(&[0.0, 0.3, 0.3]);
        let (ic_03, ib_03, _) = q.operating_point_currents();
        assert_relative_within(ic_03.abs(), 2.829459e-07, 0.03, "temp ic @ 0.3V");
        assert_relative_within(ib_03.abs(), 4.160429e-09, 0.05, "temp ib @ 0.3V");

        q.update(&[0.0, 0.7, 0.7]);
        let (ic_07, ib_07, _) = q.operating_point_currents();
        assert_relative_within(ic_07.abs(), 3.955907e-03, 0.03, "temp ic @ 0.7V");
        assert_relative_within(ib_07.abs(), 1.316270e-04, 0.05, "temp ib @ 0.7V");
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
        let (collector, base, emitter) = q.small_signal_row_coefficients(3.3, 0.78, 0.0);

        assert!((collector.0 + base.0 + emitter.0).abs() < 1e-9);
        assert!((collector.1 + base.1 + emitter.1).abs() < 1e-9);
        assert!((collector.2 + base.2 + emitter.2).abs() < 1e-9);
    }

    #[test]
    fn test_bjt_instance_multiplier_scales_collector_current() {
        let q1 = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        let q2 =
            Bjt::new_npn("Q2".to_string(), 2, 1, 0).with_instance_params(&[("M".to_string(), 2.0)]);

        let (ic1, _, _) = q1.calculate_currents(0.7, -5.0);
        let (ic2, _, _) = q2.calculate_currents(0.7, -5.0);

        assert!(ic1 > 0.0 && ic2 > 0.0);
        let ratio = ic2 / ic1;
        assert!(
            (ratio - 2.0).abs() < 1e-4,
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
