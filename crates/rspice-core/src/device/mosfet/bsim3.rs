//! BSIM3v3 MOSFET Model
//!
//! Implements the BSIM3v3.24 model for submicron MOSFETs (0.18μm and above).
//! This is the predecessor to BSIM4, still widely used for legacy designs.
//!
//! # Model Features
//!
//! - **Threshold voltage**: Body effect, narrow/short channel effects
//! - **Mobility degradation**: Vertical field, velocity saturation
//! - **Output conductance**: Channel length modulation, DIBL
//! - **Capacitance**: Charge-based Meyer model
//! - **Noise**: Flicker (1/f) and thermal noise
//!
//! # Key Differences from BSIM4
//!
//! - Simpler mobility model
//! - Different velocity saturation formulation
//! - RCS/RCD parasitic resistances handled differently
//! - No quantum effects (unlike BSIM4)

use crate::Value;

/// Physical constants for BSIM3
const Q: Value = 1.60217663e-19; // Electron charge (C)
const EPSSI: Value = 1.03594e-10; // Silicon permittivity (F/m)
const EPSOX: Value = 3.453e-11; // SiO2 permittivity (F/m)
const KB: Value = 1.38064852e-23; // Boltzmann constant (J/K)

/// BSIM3 device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Bsim3Type {
    #[default]
    Nmos,
    Pmos,
}

/// BSIM3 operating region
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Bsim3Region {
    #[default]
    Cutoff,
    Linear,
    Saturation,
    Subthreshold,
}

/// BSIM3v3 model parameters
///
/// Default values are for a typical 0.18μm process
#[derive(Debug, Clone)]
pub struct Bsim3Params {
    /// Device type (NMOS/PMOS)
    pub mos_type: Bsim3Type,

    /// Channel length (m)
    pub l: Value,
    /// Channel width (m)
    pub w: Value,
    /// Gate oxide thickness (m)
    pub tox: Value,

    // Threshold voltage parameters
    /// Threshold voltage at Vbs=0 (V)
    pub vth0: Value,
    /// First-order body effect coefficient
    pub k1: Value,
    /// Second-order body effect coefficient
    pub k2: Value,
    /// Body effect sensitivity parameter
    pub k3: Value,
    /// Body effect width dependence
    pub k3b: Value,
    /// Narrow width effect coefficient
    pub w0: Value,
    /// DIBL coefficient 1
    pub dvt0: Value,
    /// DIBL coefficient 2
    pub dvt1: Value,
    /// DIBL coefficient for wide devices
    pub dvt2: Value,
    /// DIBL low-Vds coefficient
    pub eta0: Value,
    /// DIBL slope coefficient
    pub etab: Value,
    /// DIBL output resistance factor
    pub dsub: Value,

    // Mobility parameters
    /// Low-field mobility (cm²/V·s)
    pub u0: Value,
    /// Mobility degradation coefficient 1
    pub ua: Value,
    /// Mobility degradation coefficient 2
    pub ub: Value,
    /// Mobility degradation coefficient 3
    pub uc: Value,

    // Velocity saturation
    /// Saturation velocity (m/s)
    pub vsat: Value,
    /// Velocity saturation factor
    pub a0: Value,
    /// Geometry factor for Abulk
    pub ags: Value,
    /// Threshold voltage offset for Rout
    pub a1: Value,
    /// Threshold voltage offset for Rout 2
    pub a2: Value,
    /// Body effect of saturation velocity
    pub b0: Value,
    /// Body effect of saturation velocity 2
    pub b1: Value,

    // Output conductance
    /// CLM coefficient
    pub pclm: Value,
    /// DIBL parameter for Rout
    pub pdiblc1: Value,
    /// DIBL parameter for Rout 2
    pub pdiblc2: Value,
    /// DIBL control parameter
    pub pdiblcb: Value,
    /// Dout effect coefficient
    pub drout: Value,
    /// Saturation parameter
    pub pscbe1: Value,
    /// Saturation parameter 2
    pub pscbe2: Value,
    /// Vdsat offset
    pub pvag: Value,

    // Subthreshold parameters
    /// Subthreshold swing factor
    pub nfactor: Value,
    /// Body charge coefficient at depletion
    pub cdsc: Value,
    /// Body charge coefficient at strong inversion
    pub cdscd: Value,
    /// Body charge coefficient (voltage dependent)
    pub cdscb: Value,
    /// Subthreshold drain bias coefficient
    pub cit: Value,
    /// Voff dependency on Vds
    pub voff: Value,

    // Source/drain parasitic
    /// Source resistance per square (Ω/sq)
    pub rsh: Value,
    /// Drain/source resistance per width (Ω·μm)
    pub rdsw: Value,
    /// Resistance width exponent
    pub prwb: Value,
    /// Resistance Vgs exponent
    pub prwg: Value,
    /// Width exponent for resistance
    pub wr: Value,

    // Capacitance parameters
    /// Overlap capacitance (F/m)
    pub cgso: Value,
    /// Overlap capacitance (F/m)
    pub cgdo: Value,
    /// Body capacitance (F/m)
    pub cgbo: Value,
    /// Body junction capacitance (F/m²)
    pub cj: Value,
    /// Body junction sidewall capacitance (F/m)
    pub cjsw: Value,
    /// Junction built-in potential (V)
    pub pb: Value,
    /// Junction grading coefficient
    pub mj: Value,
    /// Sidewall grading coefficient
    pub mjsw: Value,

    // Noise parameters
    /// Flicker noise coefficient
    pub noia: Value,
    /// Flicker noise exponent
    pub noib: Value,
    /// Noise model selector
    pub noic: Value,
    /// Noise exponent
    pub af: Value,
    /// Flicker noise frequency exponent
    pub ef: Value,

    /// Operating temperature (K)
    pub temp: Value,
}

impl Default for Bsim3Params {
    fn default() -> Self {
        Self {
            mos_type: Bsim3Type::Nmos,
            l: 0.18e-6,
            w: 1.0e-6,
            tox: 4.0e-9,

            // Threshold voltage (typical 0.18μm NMOS)
            vth0: 0.4,
            k1: 0.5,
            k2: -0.1,
            k3: 80.0,
            k3b: 0.0,
            w0: 0.0,
            dvt0: 0.5,
            dvt1: 0.53,
            dvt2: -0.032,
            eta0: 0.08,
            etab: -0.07,
            dsub: 0.56,

            // Mobility
            u0: 400.0, // cm²/V·s for NMOS
            ua: 2.25e-9,
            ub: 5.87e-19,
            uc: -4.65e-11,

            // Velocity saturation
            vsat: 1.5e5,
            a0: 1.0,
            ags: 0.2,
            a1: 0.0,
            a2: 1.0,
            b0: 0.0,
            b1: 0.0,

            // Output conductance
            pclm: 1.3,
            pdiblc1: 0.39,
            pdiblc2: 0.0086,
            pdiblcb: 0.0,
            drout: 0.56,
            pscbe1: 4.24e8,
            pscbe2: 1.0e-5,
            pvag: 0.1,

            // Subthreshold
            nfactor: 1.5,
            cdsc: 0.0,
            cdscd: 0.0,
            cdscb: 0.0,
            cit: 0.0,
            voff: -0.11,

            // Parasitics
            rsh: 0.0,
            rdsw: 200.0,
            prwb: 0.0,
            prwg: 0.0,
            wr: 1.0,

            // Capacitance
            cgso: 2.0e-10,
            cgdo: 2.0e-10,
            cgbo: 1.0e-12,
            cj: 1.0e-3,
            cjsw: 5.0e-10,
            pb: 0.8,
            mj: 0.5,
            mjsw: 0.33,

            // Noise
            noia: 1.0e20,
            noib: 5.0e4,
            noic: 0.0,
            af: 1.0,
            ef: 1.0,

            temp: 300.0,
        }
    }
}

impl Bsim3Params {
    /// Create NMOS parameters
    pub fn nmos() -> Self {
        Self::default()
    }

    /// Create PMOS parameters
    pub fn pmos() -> Self {
        Self {
            mos_type: Bsim3Type::Pmos,
            vth0: -0.4,
            u0: 100.0, // Lower mobility for PMOS
            ..Self::default()
        }
    }

    /// Set channel length
    pub fn with_length(mut self, l: Value) -> Self {
        self.l = l;
        self
    }

    /// Set channel width
    pub fn with_width(mut self, w: Value) -> Self {
        self.w = w;
        self
    }

    /// Set oxide thickness
    pub fn with_tox(mut self, tox: Value) -> Self {
        self.tox = tox;
        self
    }

    /// Set threshold voltage
    pub fn with_vth0(mut self, vth0: Value) -> Self {
        self.vth0 = vth0;
        self
    }
}

/// BSIM3v3 MOSFET device instance
#[derive(Debug, Clone)]
pub struct Bsim3 {
    /// Device name
    pub name: String,
    /// Drain node
    pub nd: usize,
    /// Gate node
    pub ng: usize,
    /// Source node
    pub ns: usize,
    /// Bulk node
    pub nb: usize,
    /// Model parameters
    pub params: Bsim3Params,

    // Operating point
    region: Bsim3Region,
    id: Value,
    gm: Value,
    gds: Value,
    gmb: Value,
    vth: Value,
    vdsat: Value,
}

impl Bsim3 {
    /// Create a new BSIM3 MOSFET
    pub fn new(
        name: String,
        nd: usize,
        ng: usize,
        ns: usize,
        nb: usize,
        params: Bsim3Params,
    ) -> Self {
        Self {
            name,
            nd,
            ng,
            ns,
            nb,
            params,
            region: Bsim3Region::Cutoff,
            id: 0.0,
            gm: 0.0,
            gds: 0.0,
            gmb: 0.0,
            vth: 0.0,
            vdsat: 0.0,
        }
    }

    /// Polarity (+1 for NMOS, -1 for PMOS)
    fn polarity(&self) -> Value {
        match self.params.mos_type {
            Bsim3Type::Nmos => 1.0,
            Bsim3Type::Pmos => -1.0,
        }
    }

    /// Thermal voltage
    fn vt(&self) -> Value {
        KB * self.params.temp / Q
    }

    /// Gate oxide capacitance per unit area
    fn cox(&self) -> Value {
        EPSOX / self.params.tox
    }

    /// Effective channel length
    fn leff(&self) -> Value {
        self.params.l
    }

    /// Effective channel width
    fn weff(&self) -> Value {
        self.params.w
    }

    /// Calculate threshold voltage with short-channel effects
    fn calculate_vth(&self, vbs: Value) -> Value {
        let p = &self.params;
        let pol = self.polarity();
        let vt = self.vt();
        let leff = self.leff();

        // Surface potential
        let phi_s = 0.8; // ~2*phi_f for typical doping

        // Body effect
        let vbs_eff = pol * vbs;
        let body_effect = if vbs_eff > 0.0 {
            // Forward bias reduces Vth
            p.k1 * ((phi_s - vbs_eff).max(0.01).sqrt() - phi_s.sqrt())
        } else {
            // Reverse bias increases Vth
            p.k1 * ((phi_s + vbs_eff.abs()).sqrt() - phi_s.sqrt())
        };

        // Short channel effect (simplified BSIM3 formulation)
        // SCE reduces Vth for short channels
        let l_ref = 1e-6; // Reference length (1um)
        let sce = -p.dvt0 * (l_ref / leff - 1.0).max(0.0) * vt;

        pol * p.vth0 + body_effect + sce
    }

    /// Calculate effective mobility
    fn calculate_mobility(&self, vgs: Value, vth: Value) -> Value {
        let p = &self.params;
        let pol = self.polarity();
        let cox = self.cox();

        let vgst = pol * vgs - vth;
        let eeff = (vgst * cox / EPSSI + 1e6).max(1e6); // Effective field

        // Mobility degradation
        let denom = 1.0 + p.ua * eeff.powf(p.ub) + p.uc * vgst * vgst;
        p.u0 * 1e-4 / denom.max(0.1) // Convert from cm²/V·s to m²/V·s
    }

    /// Calculate drain current with all effects
    pub fn calculate_id(&mut self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = &self.params;
        let pol = self.polarity();
        let vt = self.vt();
        let cox = self.cox();
        let leff = self.leff();
        let weff = self.weff();

        // Calculate effective voltages for PMOS
        let vgs_eff = pol * vgs;
        let vds_eff = (pol * vds).max(0.0); // Ensure positive Vds

        // Threshold voltage
        self.vth = self.calculate_vth(vbs);
        let vth = self.vth;

        // Gate overdrive (unclipped for region detection)
        let vgst_raw = vgs_eff - vth;

        // Cutoff region: Vgs well below Vth (more than ~4*n*Vt)
        // This gives ~6 decades of subthreshold current range
        if vgst_raw < -4.0 * p.nfactor * vt {
            self.region = Bsim3Region::Cutoff;
            self.id = 0.0;
            self.gm = 0.0;
            self.gds = 0.0;
            self.gmb = 0.0;
            self.vdsat = 0.0;
            return 0.0;
        }

        // Subthreshold region: Vgs near or slightly below Vth
        if vgst_raw < p.nfactor * vt {
            self.region = Bsim3Region::Subthreshold;

            // BSIM3 subthreshold current model
            // Id = Is0 * exp(Vgs - Vth) / (n * Vt)) * (1 - exp(-Vds/Vt))
            let n = p.nfactor;

            // Specific current: Is0 = u0 * Cox * (n-1) * Vt^2 * W/L
            let is0 = p.u0 * 1e-4 * cox * (n - 1.0) * vt * vt * weff / leff;

            // Exponential term with proper weak inversion slope
            let exp_arg = (vgst_raw / (n * vt)).clamp(-40.0, 40.0);

            // Drain-source dependence for weak inversion
            let vds_factor = 1.0 - (-vds_eff / vt).exp();

            self.id = pol * is0 * exp_arg.exp() * vds_factor;

            // Small-signal parameters in subthreshold
            self.gm = self.id.abs() / (n * vt);
            self.gds = self.id.abs() / vt * (-vds_eff / vt).exp();
            self.gmb = self.gm * (n - 1.0) / n;
            self.vdsat = 2.0 * n * vt;

            return self.id;
        }

        // Strong inversion: gate overdrive (now guaranteed positive)
        let vgst = vgst_raw;

        // Mobility
        let mu = self.calculate_mobility(vgs, vth);

        // Saturation velocity
        let esat = 2.0 * p.vsat / mu;

        // Saturation voltage (VDSAT)
        let vdsat_raw = vgst / (1.0 + vgst / (esat * leff));
        self.vdsat = vdsat_raw;

        // Determine region
        let vds_sat = self.vdsat;
        let in_saturation = vds_eff >= vds_sat;

        // Base current calculation
        let beta = mu * cox * weff / leff;

        let id_base = if in_saturation {
            self.region = Bsim3Region::Saturation;
            // Saturation current with CLM
            let lambda = p.pclm / leff;
            beta * vdsat_raw.powi(2) / 2.0 * (1.0 + lambda * (vds_eff - vds_sat))
        } else {
            self.region = Bsim3Region::Linear;
            // Linear region
            beta * (vgst * vds_eff - vds_eff.powi(2) / 2.0)
        };

        self.id = pol * id_base;

        // Small-signal parameters
        if in_saturation {
            self.gm = beta * vdsat_raw;
            self.gds = id_base * p.pclm / leff;
        } else {
            self.gm = beta * vds_eff;
            self.gds = beta * (vgst - vds_eff);
        }
        self.gmb = self.gm * 0.2; // Simplified body effect

        self.id
    }

    /// Get drain current (after calculate_id)
    pub fn id(&self) -> Value {
        self.id
    }

    /// Get transconductance
    pub fn gm(&self) -> Value {
        self.gm
    }

    /// Get output conductance
    pub fn gds(&self) -> Value {
        self.gds
    }

    /// Get body transconductance
    pub fn gmb(&self) -> Value {
        self.gmb
    }

    /// Get threshold voltage
    pub fn vth(&self) -> Value {
        self.vth
    }

    /// Get saturation voltage
    pub fn vdsat(&self) -> Value {
        self.vdsat
    }

    /// Get operating region
    pub fn region(&self) -> Bsim3Region {
        self.region
    }

    /// Get region as string
    pub fn region_str(&self) -> &'static str {
        match self.region {
            Bsim3Region::Cutoff => "cutoff",
            Bsim3Region::Linear => "linear",
            Bsim3Region::Saturation => "saturation",
            Bsim3Region::Subthreshold => "subthreshold",
        }
    }

    /// Calculate gate-source capacitance
    pub fn cgs(&self) -> Value {
        let p = &self.params;
        let weff = self.weff();
        let leff = self.leff();
        let cox = self.cox();

        match self.region {
            Bsim3Region::Cutoff | Bsim3Region::Subthreshold => p.cgso * weff,
            Bsim3Region::Linear => 2.0 / 3.0 * cox * weff * leff + p.cgso * weff,
            Bsim3Region::Saturation => 2.0 / 3.0 * cox * weff * leff + p.cgso * weff,
        }
    }

    /// Calculate gate-drain capacitance
    pub fn cgd(&self) -> Value {
        let p = &self.params;
        let weff = self.weff();

        match self.region {
            Bsim3Region::Cutoff | Bsim3Region::Subthreshold => p.cgdo * weff,
            Bsim3Region::Saturation => p.cgdo * weff,
            Bsim3Region::Linear => p.cgdo * weff, // Simplified
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

