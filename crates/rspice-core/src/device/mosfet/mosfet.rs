//! MOSFET (Metal-Oxide-Semiconductor Field-Effect Transistor) device model
//!
//! Implements a Level 1 SPICE MOSFET model (Shichman-Hodges).
//! Supports NMOS and PMOS devices in cutoff, linear, and saturation regions.

use super::legacy_bsim::{LegacyBsimModel, LegacyBsimRegion, LegacyBsimSizedModel};
use super::smooth::{SMOOTH_VOLTAGE, smooth_max, smooth_min, smooth_positive, smooth_step};
use crate::constants::VT_REFERENCE;
use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

/// Separate smoothing width for Vds-dependent region transitions.
///
/// Keep this much smaller than threshold smoothing to avoid artificial channel
/// current at Vdsâ‰ˆ0 while retaining C1 continuity for Newton.
mod capacitance;
mod construction;
mod current;
mod junctions;
mod mos2;
mod nonlinear;
mod operating_point;
mod stamping;

const VDS_SMOOTHING: Value = SMOOTH_VOLTAGE * 1e-1;

/// MOSFET type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosType {
    Nmos,
    Pmos,
}

/// MOSFET operating region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosRegion {
    Cutoff,
    Linear,
    Saturation,
}

/// Pre-computed stamp indices for O(1) matrix access (4-terminal device, but only D and S rows)
/// Note: Gate draws no DC current so has no G row stamps
#[derive(Debug, Clone, Default)]
pub struct MosfetIndices {
    // Drain row (4 columns)
    pub dd: Option<CscIndex>,
    pub dg: Option<CscIndex>,
    pub ds: Option<CscIndex>,
    pub db: Option<CscIndex>,
    // Source row (4 columns)
    pub sd: Option<CscIndex>,
    pub sg: Option<CscIndex>,
    pub ss: Option<CscIndex>,
    pub sb: Option<CscIndex>,
}

/// MOSFET device supporting Level 1 (Shichman-Hodges) and Level 3 (BSIM3-like) models
///
/// Terminal connections:
/// - Drain (D)
/// - Gate (G)
/// - Source (S)
/// - Bulk/Body (B)
#[derive(Debug, Clone)]
pub struct Mosfet {
    pub name: String,
    pub mos_type: MosType,

    // Node connections
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    pub node_bulk: NodeId,

    // Geometry
    /// Channel length (L) in meters
    pub l: Value,
    /// Channel width (W) in meters
    pub w: Value,
    /// Lateral diffusion/channel shortening per side (LD) in meters
    pub ld: Value,

    // Model parameters (Level 1)
    /// Threshold voltage (VTO)
    pub vto: Value,
    /// Transconductance parameter (KP) in A/V^2
    pub kp: Value,
    /// Body effect coefficient (GAMMA)
    pub gamma: Value,
    /// Surface potential (PHI)
    pub phi: Value,
    /// Channel-length modulation (LAMBDA)
    pub lambda: Value,
    /// Bulk junction saturation current (IS)
    pub is_bulk: Value,
    /// Bulk junction saturation current density (JS)
    pub js_bulk: Value,
    /// Oxide capacitance per unit area (COX)
    pub cox: Value,
    /// Zero-bias bulk junction bottom capacitance density (CJ)
    pub cj: Value,
    /// Zero-bias bulk junction sidewall capacitance density (CJSW)
    pub cjsw: Value,
    /// Bulk junction built-in potential (PB)
    pub pb: Value,
    /// Bottom-junction grading coefficient (MJ)
    pub mj: Value,
    /// Sidewall grading coefficient (MJSW)
    pub mjsw: Value,
    /// Forward-bias depletion-capacitance coefficient (FC)
    pub fc: Value,
    /// Explicit zero-bias bulk-drain capacitance override (CBD/CAPBD)
    pub drain_bulk_cap_zero_bias: Option<Value>,
    /// Explicit zero-bias bulk-source capacitance override (CBS/CAPBS)
    pub source_bulk_cap_zero_bias: Option<Value>,
    /// Drain diffusion area (AD)
    pub drain_area: Value,
    /// Source diffusion area (AS)
    pub source_area: Value,
    /// Drain perimeter (PD)
    pub drain_perimeter: Value,
    /// Source perimeter (PS)
    pub source_perimeter: Value,
    /// Device-local junction GMIN used by ngspice-style bulk diode loading.
    pub junction_gmin: Value,
    /// Thermal voltage kT/q at the device operating temperature.
    /// Set by `set_temperature`; defaults to the 27C reference value.
    pub vt: Value,

    // BSIM3-like parameters for short-channel effects
    /// Model level (1 = Level 1, 3 = BSIM3-like)
    pub level: i32,
    /// Legacy BSIM1/BSIM2 model card for SPICE levels 4 and 5.
    legacy_bsim_model: Option<LegacyBsimModel>,
    /// Geometry-sized legacy BSIM instance data derived from W/L.
    legacy_bsim_sized: Option<LegacyBsimSizedModel>,
    /// Mobility at low field (U0) in cm^2/V*s
    pub u0: Value,
    /// U0 exactly as given on the model card, never temperature-scaled.
    /// ngspice's MOS2 velocity-saturation path reads the model mobility
    /// directly (mos2load.c uses MOS2surfaceMobility, not tSurfMob).
    pub u0_card: Value,
    /// First-order mobility degradation (UA)
    pub ua: Value,
    /// Second-order mobility degradation (UB)
    pub ub: Value,
    /// Velocity saturation (VSAT) in m/s
    pub vsat: Value,
    /// DIBL coefficient 1 (ETA0)
    pub eta0: Value,
    /// DIBL coefficient 2 (ETAB)
    pub etab: Value,
    /// Subthreshold swing coefficient (NFACTOR)
    pub nfactor: Value,
    /// Drain saturation voltage coefficient (PCLM)
    pub pclm: Value,
    /// Source/drain resistance (RDSW) in ohm*um
    pub rdsw: Value,

    // Berkeley MOS2 parameters
    /// Substrate doping (NSUB) in cm^-3.
    pub mos2_substrate_doping: Value,
    /// Narrow-channel factor (DELTA).
    pub mos2_narrow_factor: Value,
    /// Critical field exponent (UEXP).
    pub mos2_crit_field_exp: Value,
    /// Critical field (UCRIT) in V/cm.
    pub mos2_crit_field: Value,
    /// Maximum carrier drift velocity (VMAX).
    pub mos2_max_drift_vel: Value,
    /// Junction depth (XJ) in meters.
    pub mos2_junction_depth: Value,
    /// Effective channel charge coefficient (NEFF).
    pub mos2_channel_charge: Value,
    /// Fast surface state density (NFS) in cm^-2.
    pub mos2_fast_surface_state_density: Value,

    // BSIM4-specific parameters for enhanced short-channel modeling
    /// Short-channel Vth roll-off coefficient 0 (DVT0)
    pub dvt0: Value,
    /// Short-channel Vth roll-off coefficient 1 (DVT1)
    pub dvt1: Value,
    /// Short-channel Vth roll-off body-bias coefficient (DVT2)
    pub dvt2: Value,
    /// First body effect coefficient (K1)
    pub k1: Value,
    /// Second body effect coefficient (K2)
    pub k2: Value,
    /// Gate-source overlap capacitance per width (CGSO) in F/m
    pub cgso: Value,
    /// Gate-drain overlap capacitance per width (CGDO) in F/m
    pub cgdo: Value,
    /// Gate-bulk overlap capacitance per length (CGBO) in F/m
    pub cgbo: Value,
    /// Source/drain sheet resistance (RSH) in ohm/square
    pub rsh: Value,
    /// Drain ohmic resistance (RD); takes precedence over RSH·NRD
    pub rd_model: Value,
    /// Source ohmic resistance (RS); takes precedence over RSH·NRS
    pub rs_model: Value,
    /// Drain diffusion squares (NRD instance parameter)
    pub nrd: Value,
    /// Source diffusion squares (NRS instance parameter)
    pub nrs: Value,
    /// Thermal-noise temperature offset in kelvin (ngspice `dtemp`
    /// semantics for the channel and parasitic thermal sources).
    pub noise_temperature_offset: Value,
    /// Flicker noise coefficient (KF)
    pub kf: Value,
    /// Flicker noise current exponent (AF)
    pub af: Value,
    /// Flicker noise frequency exponent (EF)
    pub ef: Value,
    /// Channel thermal-noise coefficient (gamma)
    pub thermal_noise_gamma: Value,
    /// Noise model selector (NLEV); mos1set.c defaults it to 2.
    pub nlev: i32,
    /// NLEV=3 channel thermal-noise scale (GDSNOI)
    pub gdsnoi: Value,
    /// Instance multiplicity (M·NF). Geometry folds it into the width, but
    /// the SPICE noise laws keep per-finger factors, so it is retained.
    pub multiplicity: Value,

    // Level 6 (double-exponent/simplified) MOSFET parameters
    /// Current gain coefficient (KC) - drain current multiplier
    pub kc: Value,
    /// Current gain exponent (NC) - affects Vgs dependence
    pub nc: Value,
    /// Voltage clipping coefficient (KV) - saturation factor
    pub kv: Value,
    /// Voltage clipping exponent (NV) - affects Vds dependence  
    pub nv: Value,
    /// Secondary back-gate effect coefficient (GAMMA1)
    pub gamma1: Value,
    /// Drain-induced threshold modulation coefficient (SIGMA)
    pub sigma: Value,
    /// First-order channel length modulation (LAMBDA0)
    pub lambda0: Value,
    /// Second-order channel length modulation (LAMBDA1)
    pub lambda1: Value,

    // Operating point values
    vgs: Value,
    vds: Value,
    vbs: Value,
    eval_vgs: Value,
    eval_vds: Value,
    eval_vbs: Value,
    id: Value,
    gm: Value,
    gds: Value,
    gmb: Value,
    id_eq: Value,
    region: MosRegion,

    // Previous iteration values
    vgs_prev: Value,
    vds_prev: Value,
    vbs_prev: Value,
    eval_vgs_prev: Value,
    eval_vds_prev: Value,
    eval_vbs_prev: Value,
    id_prev: Value,
    gm_prev: Value,
    gds_prev: Value,
    gmb_prev: Value,
    ibs_prev: Value,
    gbs_prev: Value,
    ibd_prev: Value,
    gbd_prev: Value,
    has_branch_history: bool,

    /// Pre-computed matrix indices for O(1) stamping
    pub indices: MosfetIndices,
}

impl Mosfet {
    /// True when this instance runs the legacy BSIM1/BSIM2 equations rather
    /// than the classic level 1/2/3/6 models.
    pub fn uses_legacy_bsim(&self) -> bool {
        self.legacy_bsim_model.is_some()
    }
}
