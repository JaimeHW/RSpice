//! Native Xyce level-2 TEAM memristor equations.
//!
//! The device contributes three DAE equations for terminal voltages `v+`, `v-`
//! and the internal state `x`:
//!
//! ```text
//! F = [ i, -i, f_team(i, x) * window(i, x) ]
//! Q = [ 0,  0, x ]
//! ```
//!
//! `x` is the solver coordinate and equals the physical state in metres times
//! `XSCALING`.  Keeping that convention at this boundary avoids poorly scaled
//! nanometre-sized unknowns while preserving the model-card units.  Evaluation
//! deliberately does not clamp `x`: limiting a Newton iterate would change the
//! equations and invalidate their analytic Jacobian.
//!
//! # Operating point
//!
//! Setting `dx/dt = 0` gives the DC state row directly, and it is the physical
//! equation whenever it has a nondegenerate root, so that is what this kernel
//! solves by default.  The active branch of `f_team` is `(i/I* - 1)^ALPHA*`,
//! whose root at the threshold current has multiplicity `ALPHA*`.  Only a unit
//! exponent leaves a nonzero derivative there; for any larger exponent both the
//! residual and its Jacobian vanish as the iterate approaches the root, so
//! Newton cannot converge on it.  The row is likewise identically zero
//! throughout the `[ION, IOFF]` deadband, where no bias-determined state exists
//! at all.
//!
//! Xyce 7.10 loads the dynamic row unconditionally at its operating point and
//! inherits both failures, aborting such decks with "Numerically singular
//! matrix found by Amesos".  Because a memristor's state is programmed by
//! history rather than by bias, this kernel instead gauges those rank-deficient
//! cases to the device's initial state with the well-posed row `x - XON`,
//! matching how the PEM family and commercial simulators treat an internal
//! state that only appears in `Q`.  That reproduces Xyce's own converged TEAM
//! operating point (`x = 0`, `R = RON`) on the decks it can start, whose `XON`
//! is the default zero, and additionally solves the constant-bias decks Xyce
//! cannot start without `UIC`.
//!
//! Compatibility policy: model metadata, residuals, charges, and observable
//! outputs follow Xyce 7.10. The Newton Jacobian is the exact derivative of
//! those equations with respect to RSpice's scaled solver coordinate. It does
//! not reproduce Xyce 7.10's historical TEAM load bugs (its terminal-state
//! derivative uses `R^2` instead of `1/R^2`, and its state row omits the
//! `dF/di * di/dx` chain). The non-unit-XSCALING finite-difference regression
//! below makes this intentional corrected-model boundary explicit.

use crate::Value;
use std::error::Error;
use std::fmt;

/// Xyce model level implemented by this module.
pub const XYCE_TEAM_MEMRISTOR_LEVEL: u8 = 2;

/// Wire/runtime version for accepted TEAM resistance-RTN state.
pub const XYCE_TEAM_RESISTANCE_NOISE_STATE_VERSION: u32 = 1;

/// Xyce TEAM random-telegraph resistance-noise parameters.
///
/// Xyce 7.10 draws an exponentially distributed dwell interval with mean
/// `RESLAMBDA * RESTD`, then alternates the common `RON`/`ROFF` multiplier
/// between `1 +/- RESDELTA * RESDELTAGRAD / 2`. `RESEPTD` is documented as
/// the minimum update interval; honoring that floor also prevents a malformed
/// zero-dwell model from consuming an unbounded number of random values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyceTeamResistanceNoiseParams {
    pub enabled: bool,
    pub seed: u32,
    pub lambda: Value,
    pub update_time: Value,
    pub epsilon_update_time: Value,
    pub delta: Value,
    pub delta_gradient: Value,
}

impl Default for XyceTeamResistanceNoiseParams {
    fn default() -> Self {
        Self {
            enabled: false,
            seed: 0,
            lambda: 0.0,
            update_time: 0.0,
            epsilon_update_time: 1.0e-10,
            delta: 0.0,
            delta_gradient: 0.0,
        }
    }
}

impl XyceTeamResistanceNoiseParams {
    pub fn validate(&self) -> Result<(), XyceTeamMemristorError> {
        for (name, value) in [
            ("RESLAMBDA", self.lambda),
            ("RESTD", self.update_time),
            ("RESEPTD", self.epsilon_update_time),
            ("RESDELTA", self.delta),
            ("RESDELTAGRAD", self.delta_gradient),
        ] {
            if !value.is_finite() {
                return Err(XyceTeamMemristorError::InvalidParameter {
                    name,
                    reason: "must be finite",
                });
            }
        }
        if !self.enabled {
            return Ok(());
        }
        require(
            self.lambda > 0.0,
            "RESLAMBDA",
            "must be greater than zero when RESNOISE is enabled",
        )?;
        require(
            self.update_time > 0.0,
            "RESTD",
            "must be greater than zero when RESNOISE is enabled",
        )?;
        require(
            self.epsilon_update_time > 0.0,
            "RESEPTD",
            "must be greater than zero when RESNOISE is enabled",
        )?;
        let excursion = 0.5 * self.delta * self.delta_gradient;
        require(
            excursion.is_finite() && excursion.abs() < 1.0,
            "RESDELTA",
            "RESDELTA*RESDELTAGRAD/2 must have magnitude less than one",
        )?;
        Ok(())
    }

    /// Collision-resistant-enough runtime provenance for checkpoint matching.
    /// The semantic netlist identity remains the primary restart authority;
    /// this independent tag prevents accepting a structurally misrouted row.
    pub fn provenance(&self, instance_name: &str) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut feed = |bytes: &[u8]| {
            for byte in bytes {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
        };
        feed(instance_name.to_ascii_uppercase().as_bytes());
        feed(&[u8::from(self.enabled)]);
        feed(&self.seed.to_le_bytes());
        for value in [
            self.lambda,
            self.update_time,
            self.epsilon_update_time,
            self.delta,
            self.delta_gradient,
        ] {
            feed(&value.to_bits().to_le_bytes());
        }
        hash
    }
}

/// Accepted-boundary TEAM RTN state stored in transient checkpoints.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct XyceTeamResistanceNoiseCheckpoint {
    pub version: u32,
    pub instance_name: String,
    pub provenance: u64,
    pub initialized: bool,
    pub rng_state: u64,
    pub last_update_time: Value,
    pub next_update_interval: Value,
    pub high_state: bool,
    pub resistance_factor: Value,
    pub last_trial_time: Value,
}

/// Trial/accepted runtime for one TEAM instance.
///
/// The transient driver snapshots this value before an attempted step. A
/// proposed state is generated at most once for that candidate time and is
/// retained only if the step is accepted; every rejection restores the RNG
/// word, dwell interval, level, and factor through the ordinary nonlinear
/// circuit rollback image.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct XyceTeamResistanceNoiseRuntime {
    params: XyceTeamResistanceNoiseParams,
    provenance: u64,
    initialized: bool,
    rng_state: u64,
    last_update_time: Value,
    next_update_interval: Value,
    high_state: bool,
    resistance_factor: Value,
    last_trial_time: Value,
}

impl XyceTeamResistanceNoiseRuntime {
    pub(crate) fn new(params: XyceTeamResistanceNoiseParams, instance_name: &str) -> Self {
        let provenance = params.provenance(instance_name);
        // Stable per-instance streams avoid construction-order coupling while
        // preserving authored RESSEED reproducibility.
        let rng_state = provenance ^ u64::from(params.seed) ^ 0x5253_5049_4345_5254_u64;
        Self {
            params,
            provenance,
            initialized: false,
            rng_state,
            last_update_time: 0.0,
            next_update_interval: 0.0,
            high_state: false,
            resistance_factor: 1.0,
            last_trial_time: 0.0,
        }
    }

    #[inline]
    pub(crate) fn enabled(&self) -> bool {
        self.params.enabled
    }

    #[inline]
    pub(crate) fn resistance_factor(&self) -> Value {
        self.resistance_factor
    }

    pub(crate) fn initialize_accepted_boundary(
        &mut self,
        time: Value,
    ) -> Result<(), XyceTeamMemristorError> {
        finite_input("resistance-noise accepted time", time)?;
        if !self.params.enabled || self.initialized {
            return Ok(());
        }
        self.last_update_time = time;
        self.last_trial_time = time;
        self.next_update_interval = self.draw_interval();
        self.initialized = true;
        Ok(())
    }

    pub(crate) fn prepare_trial(&mut self, time: Value) -> Result<(), XyceTeamMemristorError> {
        finite_input("resistance-noise trial time", time)?;
        if !self.params.enabled {
            return Ok(());
        }
        if !self.initialized {
            return Err(XyceTeamMemristorError::InvalidParameter {
                name: "RESNOISE",
                reason: "transient runtime was not initialized at an accepted boundary",
            });
        }
        if self.last_trial_time.to_bits() == time.to_bits() {
            return Ok(());
        }
        if time < self.last_update_time {
            return Err(XyceTeamMemristorError::InvalidParameter {
                name: "RESNOISE",
                reason: "trial time precedes the accepted resistance-noise state",
            });
        }
        self.last_trial_time = time;
        if time - self.last_update_time > self.next_update_interval {
            self.last_update_time = time;
            self.next_update_interval = self.draw_interval();
            self.high_state = !self.high_state;
            let excursion = 0.5 * self.params.delta * self.params.delta_gradient;
            self.resistance_factor = if self.high_state {
                1.0 + excursion
            } else {
                1.0 - excursion
            };
        }
        Ok(())
    }

    fn draw_interval(&mut self) -> Value {
        self.rng_state = self.rng_state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut word = self.rng_state;
        word = (word ^ (word >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        word = (word ^ (word >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        word ^= word >> 31;
        let uniform = (((word >> 11) as Value) + 1.0) / 9_007_199_254_740_992.0;
        (-uniform.ln() * self.params.lambda * self.params.update_time)
            .max(self.params.epsilon_update_time)
    }

    pub(crate) fn checkpoint(&self, instance_name: &str) -> XyceTeamResistanceNoiseCheckpoint {
        XyceTeamResistanceNoiseCheckpoint {
            version: XYCE_TEAM_RESISTANCE_NOISE_STATE_VERSION,
            instance_name: instance_name.to_string(),
            provenance: self.provenance,
            initialized: self.initialized,
            rng_state: self.rng_state,
            last_update_time: self.last_update_time,
            next_update_interval: self.next_update_interval,
            high_state: self.high_state,
            resistance_factor: self.resistance_factor,
            last_trial_time: self.last_trial_time,
        }
    }

    pub(crate) fn validate_checkpoint(
        &self,
        instance_name: &str,
        checkpoint: &XyceTeamResistanceNoiseCheckpoint,
        accepted_time: Value,
    ) -> Result<(), String> {
        if checkpoint.version != XYCE_TEAM_RESISTANCE_NOISE_STATE_VERSION {
            return Err(format!(
                "TEAM resistance-noise checkpoint for '{}' has unsupported state version {}; runtime requires {}",
                instance_name, checkpoint.version, XYCE_TEAM_RESISTANCE_NOISE_STATE_VERSION
            ));
        }
        if checkpoint.instance_name != instance_name {
            return Err(format!(
                "TEAM resistance-noise checkpoint instance '{}' does not match target '{}'",
                checkpoint.instance_name, instance_name
            ));
        }
        if checkpoint.provenance != self.provenance {
            return Err(format!(
                "TEAM resistance-noise checkpoint provenance mismatch for '{instance_name}'"
            ));
        }
        if !checkpoint.last_update_time.is_finite()
            || !checkpoint.next_update_interval.is_finite()
            || !checkpoint.resistance_factor.is_finite()
            || !checkpoint.last_trial_time.is_finite()
        {
            return Err(format!(
                "TEAM resistance-noise checkpoint for '{instance_name}' contains non-finite state"
            ));
        }
        if checkpoint.initialized != self.params.enabled {
            return Err(format!(
                "TEAM resistance-noise checkpoint initialization state for '{instance_name}' does not match RESNOISE"
            ));
        }
        if self.params.enabled {
            if checkpoint.next_update_interval < self.params.epsilon_update_time {
                return Err(format!(
                    "TEAM resistance-noise checkpoint dwell interval for '{instance_name}' is below RESEPTD"
                ));
            }
            if checkpoint.last_update_time > accepted_time
                || checkpoint.last_trial_time.to_bits() != accepted_time.to_bits()
            {
                return Err(format!(
                    "TEAM resistance-noise checkpoint time provenance for '{instance_name}' does not match accepted time {accepted_time}"
                ));
            }
            let excursion = 0.5 * self.params.delta * self.params.delta_gradient;
            let expected_factor = if checkpoint.high_state {
                1.0 + excursion
            } else {
                1.0 - excursion
            };
            if checkpoint.resistance_factor.to_bits() != expected_factor.to_bits() {
                return Err(format!(
                    "TEAM resistance-noise checkpoint factor for '{instance_name}' is inconsistent with its RTN state"
                ));
            }
        } else if checkpoint.rng_state != self.rng_state
            || checkpoint.last_update_time.to_bits() != 0.0f64.to_bits()
            || checkpoint.next_update_interval.to_bits() != 0.0f64.to_bits()
            || checkpoint.high_state
            || checkpoint.resistance_factor.to_bits() != 1.0f64.to_bits()
        {
            return Err(format!(
                "disabled TEAM resistance-noise checkpoint for '{instance_name}' contains active state"
            ));
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint(
        &mut self,
        instance_name: &str,
        checkpoint: &XyceTeamResistanceNoiseCheckpoint,
        accepted_time: Value,
    ) -> Result<(), String> {
        self.validate_checkpoint(instance_name, checkpoint, accepted_time)?;
        self.initialized = checkpoint.initialized;
        self.rng_state = checkpoint.rng_state;
        self.last_update_time = checkpoint.last_update_time;
        self.next_update_interval = checkpoint.next_update_interval;
        self.high_state = checkpoint.high_state;
        self.resistance_factor = checkpoint.resistance_factor;
        self.last_trial_time = checkpoint.last_trial_time;
        Ok(())
    }
}

/// Parameters on a Xyce `.MODEL ... MEMRISTOR LEVEL=2` card.
///
/// Defaults match Xyce 7.10's `N_DEV_MemristorTEAM` metadata.  The stochastic
/// resistance extension is intentionally outside this deterministic equation
/// kernel; it belongs in the transient device wrapper that owns time and RNG
/// state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyceTeamModelParams {
    pub k_on: Value,
    pub k_off: Value,
    pub alpha_on: Value,
    pub alpha_off: Value,
    pub x_on: Value,
    pub x_off: Value,
    pub r_on: Value,
    pub r_off: Value,
    pub i_on: Value,
    pub i_off: Value,
    pub x_scaling: Value,
    /// `D`, used by window types 1 and 2.
    pub d: Value,
    /// `P`, used by window types 1 through 3.
    pub p: Value,
    /// `J`, used by window type 3.
    pub j: Value,
    pub a_on: Value,
    pub a_off: Value,
    pub wc: Value,
    /// Window selector: 0 none, 1 Joglekar, 2 Biolek, 3 Prodromakis, 4 TEAM.
    pub window_type: i32,
}

impl Default for XyceTeamModelParams {
    fn default() -> Self {
        Self {
            k_on: -8.0e-13,
            k_off: 8.0e-13,
            alpha_on: 3.0,
            alpha_off: 3.0,
            x_on: 0.0,
            x_off: 3.0e-9,
            r_on: 50.0,
            r_off: 1.0e3,
            i_on: 8.9e-6,
            i_off: 1.15e-4,
            x_scaling: 1.0,
            d: 1.15e-4,
            p: 1.15e-4,
            j: 1.15e-4,
            a_on: 0.0,
            a_off: 3.0e-9,
            wc: 1.07e-12,
            window_type: 0,
        }
    }
}

impl XyceTeamModelParams {
    /// Validate parameter-domain requirements needed by the equations and
    /// their derivatives.
    pub fn validate(&self) -> Result<(), XyceTeamMemristorError> {
        let finite = [
            ("KON", self.k_on),
            ("KOFF", self.k_off),
            ("ALPHAON", self.alpha_on),
            ("ALPHAOFF", self.alpha_off),
            ("XON", self.x_on),
            ("XOFF", self.x_off),
            ("RON", self.r_on),
            ("ROFF", self.r_off),
            ("ION", self.i_on),
            ("IOFF", self.i_off),
            ("XSCALING", self.x_scaling),
            ("D", self.d),
            ("P", self.p),
            ("J", self.j),
            ("AON", self.a_on),
            ("AOFF", self.a_off),
            ("WC", self.wc),
        ];
        for (name, value) in finite {
            if !value.is_finite() {
                return Err(XyceTeamMemristorError::InvalidParameter {
                    name,
                    reason: "must be finite",
                });
            }
        }

        require(self.r_on > 0.0, "RON", "must be greater than zero")?;
        require(self.r_off > 0.0, "ROFF", "must be greater than zero")?;
        require(
            self.x_scaling > 0.0,
            "XSCALING",
            "must be greater than zero",
        )?;
        require(self.x_on != self.x_off, "XOFF", "must differ from XON")?;
        require(self.i_on != 0.0, "ION", "must be nonzero")?;
        require(self.i_off != 0.0, "IOFF", "must be nonzero")?;
        require(self.i_on < self.i_off, "ION", "must be less than IOFF")?;
        require(self.alpha_on >= 1.0, "ALPHAON", "must be at least one")?;
        require(self.alpha_off >= 1.0, "ALPHAOFF", "must be at least one")?;

        // With the conventional signs, each active-branch power has a
        // non-negative base.  Xyce's historical positive ION default is also
        // supported because its integer default exponent remains real-valued.
        if self.i_on > 0.0 {
            integer_exponent(self.alpha_on).ok_or(XyceTeamMemristorError::InvalidParameter {
                name: "ALPHAON",
                reason: "must be an integer when ION is positive",
            })?;
        }
        if self.i_off < 0.0 {
            integer_exponent(self.alpha_off).ok_or(XyceTeamMemristorError::InvalidParameter {
                name: "ALPHAOFF",
                reason: "must be an integer when IOFF is negative",
            })?;
        }

        match self.window_type {
            0 => {}
            1 | 2 => {
                require(self.d != 0.0, "D", "must be nonzero")?;
                require(self.p > 0.0, "P", "must be greater than zero")?;
            }
            3 => {
                require(self.p > 0.0, "P", "must be greater than zero")?;
            }
            4 => require(self.wc > 0.0, "WC", "must be greater than zero")?,
            _ => {
                return Err(XyceTeamMemristorError::InvalidWindowType(self.window_type));
            }
        }
        Ok(())
    }

    /// Physical `XON` expressed in the solver's scaled state coordinate.
    #[inline]
    pub fn scaled_x_on(&self) -> Value {
        self.x_on * self.x_scaling
    }

    /// Physical `XOFF` expressed in the solver's scaled state coordinate.
    #[inline]
    pub fn scaled_x_off(&self) -> Value {
        self.x_off * self.x_scaling
    }
}

/// Parameters attached to one `YMEMRISTOR` instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct XyceTeamInstanceParams {
    /// I/V relation selector: 0 linear resistance, 1 exponential resistance.
    pub iv_relation: i32,
}

impl XyceTeamInstanceParams {
    pub fn validate(&self) -> Result<(), XyceTeamMemristorError> {
        match self.iv_relation {
            0 | 1 => Ok(()),
            value => Err(XyceTeamMemristorError::InvalidIvRelation(value)),
        }
    }
}

/// Construction or evaluation failure for the TEAM equation kernel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XyceTeamMemristorError {
    InvalidParameter {
        name: &'static str,
        reason: &'static str,
    },
    InvalidIvRelation(i32),
    InvalidWindowType(i32),
    NonFiniteInput {
        name: &'static str,
    },
    NonFiniteEvaluation,
}

impl fmt::Display for XyceTeamMemristorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter { name, reason } => {
                write!(f, "invalid TEAM memristor parameter {name}: {reason}")
            }
            Self::InvalidIvRelation(value) => {
                write!(
                    f,
                    "invalid TEAM memristor IVRELATION {value}; expected 0 or 1"
                )
            }
            Self::InvalidWindowType(value) => {
                write!(f, "invalid TEAM memristor WT {value}; expected 0 through 4")
            }
            Self::NonFiniteInput { name } => {
                write!(f, "TEAM memristor input {name} must be finite")
            }
            Self::NonFiniteEvaluation => write!(f, "TEAM memristor evaluation was not finite"),
        }
    }
}

impl Error for XyceTeamMemristorError {}

/// Which form of the state row an evaluation should build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XyceTeamEvaluationMode {
    /// Steady state `f_team * window = 0` where that root is nondegenerate,
    /// otherwise the `x - XON` gauge. See the module docs.
    DcOperatingPoint,
    /// `f_team(i, x) * window(i, x)`, the integrated kinetic law.
    Dynamic,
}

/// Kinetic state-row data, absent for the DC `x - XON` gauge equation.
///
/// The gauge does not reference the kinetics, so the DC path never evaluates
/// them.  That also keeps the operating point defined for window types whose
/// fractional powers are only real-valued over part of the state range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyceTeamStateDrive {
    pub window: Value,
    /// Kinetic term before multiplication by the window.
    pub drive: Value,
}

/// Complete equation cache for one Newton evaluation.
///
/// Rows and columns of `jacobian` are ordered `(v_pos, v_neg, x_scaled)`.
/// `charge_jacobian` is the DAE `dQ/dz` matrix and has only `(2, 2) = 1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyceTeamMemristorCache {
    pub resistance: Value,
    /// `dR/dx_scaled`, useful when exposing resistance as an auxiliary node.
    pub resistance_derivative: Value,
    pub conductance: Value,
    pub current: Value,
    /// Kinetics behind the state row, `None` at the DC operating point.
    pub state_drive: Option<XyceTeamStateDrive>,
    pub residual: [Value; 3],
    pub charge: [Value; 3],
    pub jacobian: [[Value; 3]; 3],
    pub charge_jacobian: [[Value; 3]; 3],
}

/// Validated, solver-independent native TEAM memristor equation kernel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyceTeamMemristor {
    model: XyceTeamModelParams,
    instance: XyceTeamInstanceParams,
    resistance_noise: XyceTeamResistanceNoiseParams,
}

impl XyceTeamMemristor {
    pub fn new(
        model: XyceTeamModelParams,
        instance: XyceTeamInstanceParams,
    ) -> Result<Self, XyceTeamMemristorError> {
        model.validate()?;
        instance.validate()?;
        Ok(Self {
            model,
            instance,
            resistance_noise: XyceTeamResistanceNoiseParams::default(),
        })
    }

    pub fn with_resistance_noise(
        mut self,
        resistance_noise: XyceTeamResistanceNoiseParams,
    ) -> Result<Self, XyceTeamMemristorError> {
        resistance_noise.validate()?;
        self.resistance_noise = resistance_noise;
        Ok(self)
    }

    #[inline]
    pub fn model(&self) -> &XyceTeamModelParams {
        &self.model
    }

    #[inline]
    pub fn instance(&self) -> &XyceTeamInstanceParams {
        &self.instance
    }

    #[inline]
    pub fn resistance_noise(&self) -> &XyceTeamResistanceNoiseParams {
        &self.resistance_noise
    }

    /// Evaluate effective resistance and its derivative with respect to the
    /// scaled solver state.
    pub fn resistance(&self, x_scaled: Value) -> Result<(Value, Value), XyceTeamMemristorError> {
        self.resistance_with_factor(x_scaled, 1.0)
    }

    pub(crate) fn resistance_with_factor(
        &self,
        x_scaled: Value,
        resistance_factor: Value,
    ) -> Result<(Value, Value), XyceTeamMemristorError> {
        finite_input("x", x_scaled)?;
        finite_input("resistance_factor", resistance_factor)?;
        if resistance_factor <= 0.0 {
            return Err(XyceTeamMemristorError::InvalidParameter {
                name: "resistance_factor",
                reason: "must be greater than zero",
            });
        }
        let x_on = self.model.scaled_x_on();
        let span = self.model.scaled_x_off() - x_on;
        let (resistance, derivative) = match self.instance.iv_relation {
            0 => {
                let derivative = (self.model.r_off - self.model.r_on) / span;
                (self.model.r_on + (x_scaled - x_on) * derivative, derivative)
            }
            1 => {
                let slope = (self.model.r_off / self.model.r_on).ln() / span;
                let resistance = self.model.r_on * ((x_scaled - x_on) * slope).exp();
                (resistance, resistance * slope)
            }
            _ => unreachable!("instance parameters are validated at construction"),
        };
        let resistance = resistance * resistance_factor;
        let derivative = derivative * resistance_factor;
        if !resistance.is_finite() || !derivative.is_finite() {
            return Err(XyceTeamMemristorError::NonFiniteEvaluation);
        }
        Ok((resistance, derivative))
    }

    /// Evaluate the selected window and `dwindow/dx_scaled`.
    pub fn window(
        &self,
        x_scaled: Value,
        current: Value,
    ) -> Result<(Value, Value), XyceTeamMemristorError> {
        finite_input("x", x_scaled)?;
        finite_input("current", current)?;
        let (value, derivative) = match self.model.window_type {
            0 => (1.0, 0.0),
            1 => {
                let exponent = 2.0 * self.model.p;
                let base = 2.0 * x_scaled / self.model.d - 1.0;
                (
                    1.0 - base.powf(exponent),
                    -powf_derivative(base, exponent) * 2.0 / self.model.d,
                )
            }
            2 => {
                let exponent = 2.0 * self.model.p;
                let step = if current < 0.0 { 0.0 } else { 1.0 };
                let base = x_scaled / self.model.d - step;
                (
                    1.0 - base.powf(exponent),
                    -powf_derivative(base, exponent) / self.model.d,
                )
            }
            3 => {
                let exponent = self.model.p;
                let shifted = x_scaled - 0.5;
                let base = shifted * shifted - 0.75;
                (
                    self.model.j * (1.0 - base.powf(exponent)),
                    -self.model.j * powf_derivative(base, exponent) * 2.0 * shifted,
                )
            }
            4 => {
                let wc = self.model.wc * self.model.x_scaling;
                let (z, dz_dx) = if current >= 0.0 {
                    (
                        (x_scaled - self.model.a_off * self.model.x_scaling) / wc,
                        1.0 / wc,
                    )
                } else {
                    (
                        -(x_scaled - self.model.a_on * self.model.x_scaling) / wc,
                        -1.0 / wc,
                    )
                };
                let inner = z.exp();
                if inner.is_infinite() {
                    (0.0, 0.0)
                } else {
                    let value = (-inner).exp();
                    (value, -value * inner * dz_dx)
                }
            }
            _ => unreachable!("model parameters are validated at construction"),
        };
        if !value.is_finite() || !derivative.is_finite() {
            return Err(XyceTeamMemristorError::NonFiniteEvaluation);
        }
        Ok((value, derivative))
    }

    /// State the operating point gauges `x` to, in the scaled coordinate.
    #[inline]
    pub fn dc_state_target(&self) -> Value {
        self.model.scaled_x_on()
    }

    /// Whether the steady-state row `f_team * window = 0` has a root Newton can
    /// converge on.
    ///
    /// The active branch is `(i/I* - 1)^ALPHA*`, so its threshold root carries
    /// multiplicity `ALPHA*` and only a unit exponent leaves a nonzero
    /// derivative there. Both branches must qualify because the branch that
    /// ends up active depends on the solved bias.
    #[inline]
    pub fn has_nondegenerate_dc_state_root(&self) -> bool {
        self.model.alpha_on == 1.0 && self.model.alpha_off == 1.0
    }

    /// Evaluate the transient `F`, `Q`, and their exact analytic Jacobians.
    pub fn evaluate(
        &self,
        v_pos: Value,
        v_neg: Value,
        x_scaled: Value,
    ) -> Result<XyceTeamMemristorCache, XyceTeamMemristorError> {
        self.evaluate_with_mode(v_pos, v_neg, x_scaled, XyceTeamEvaluationMode::Dynamic)
    }

    /// Evaluate `F`, `Q`, and their exact analytic Jacobians for one mode.
    pub fn evaluate_with_mode(
        &self,
        v_pos: Value,
        v_neg: Value,
        x_scaled: Value,
        mode: XyceTeamEvaluationMode,
    ) -> Result<XyceTeamMemristorCache, XyceTeamMemristorError> {
        self.evaluate_with_mode_and_resistance_factor(v_pos, v_neg, x_scaled, mode, 1.0)
    }

    pub(crate) fn evaluate_with_mode_and_resistance_factor(
        &self,
        v_pos: Value,
        v_neg: Value,
        x_scaled: Value,
        mode: XyceTeamEvaluationMode,
        resistance_factor: Value,
    ) -> Result<XyceTeamMemristorCache, XyceTeamMemristorError> {
        finite_input("v_pos", v_pos)?;
        finite_input("v_neg", v_neg)?;
        finite_input("x", x_scaled)?;

        let voltage = v_pos - v_neg;
        let (resistance, dresistance_dx) =
            self.resistance_with_factor(x_scaled, resistance_factor)?;
        let conductance = resistance.recip();
        let current = voltage * conductance;
        let dcurrent_dv_pos = conductance;
        let dcurrent_dv_neg = -conductance;
        let dcurrent_dx = -voltage * dresistance_dx / (resistance * resistance);

        let gauge = (None, x_scaled - self.dc_state_target(), [0.0, 0.0, 1.0]);
        let kinetic_row = || -> Result<_, XyceTeamMemristorError> {
            let (drive, ddrive_di) = self.state_drive(current);
            let (window, dwindow_dx) = self.window(x_scaled, current)?;
            let dstate_di = ddrive_di * window;
            Ok((
                Some(XyceTeamStateDrive { window, drive }),
                drive * window,
                [
                    dstate_di * dcurrent_dv_pos,
                    dstate_di * dcurrent_dv_neg,
                    dstate_di * dcurrent_dx + drive * dwindow_dx,
                ],
            ))
        };

        let (state_drive, state_residual, state_jacobian) = match mode {
            XyceTeamEvaluationMode::DcOperatingPoint => {
                if self.has_nondegenerate_dc_state_root() {
                    let (drive, residual, jacobian) = kinetic_row()?;
                    // A unit exponent still leaves the row identically zero
                    // inside the threshold deadband, and a saturated window can
                    // annihilate it anywhere. Gauge those iterates too rather
                    // than hand the solver an empty equation.
                    if residual == 0.0 && jacobian.iter().all(|derivative| *derivative == 0.0) {
                        gauge
                    } else {
                        (drive, residual, jacobian)
                    }
                } else {
                    gauge
                }
            }
            XyceTeamEvaluationMode::Dynamic => kinetic_row()?,
        };

        let residual = [current, -current, state_residual];
        let jacobian = [
            [dcurrent_dv_pos, dcurrent_dv_neg, dcurrent_dx],
            [-dcurrent_dv_pos, -dcurrent_dv_neg, -dcurrent_dx],
            state_jacobian,
        ];
        let charge = [0.0, 0.0, x_scaled];
        let charge_jacobian = [[0.0; 3], [0.0; 3], [0.0, 0.0, 1.0]];

        if !conductance.is_finite()
            || !current.is_finite()
            || residual.iter().any(|value| !value.is_finite())
            || jacobian.iter().flatten().any(|value| !value.is_finite())
        {
            return Err(XyceTeamMemristorError::NonFiniteEvaluation);
        }

        Ok(XyceTeamMemristorCache {
            resistance,
            resistance_derivative: dresistance_dx,
            conductance,
            current,
            state_drive,
            residual,
            charge,
            jacobian,
            charge_jacobian,
        })
    }

    fn state_drive(&self, current: Value) -> (Value, Value) {
        if current >= self.model.i_off {
            let base = current / self.model.i_off - 1.0;
            let power = if base < 0.0 {
                base.powi(
                    integer_exponent(self.model.alpha_off)
                        .expect("negative IOFF requires an integer ALPHAOFF"),
                )
            } else {
                base.powf(self.model.alpha_off)
            };
            let power_derivative = if base == 0.0 && self.model.alpha_off == 1.0 {
                1.0
            } else if base < 0.0 {
                powi_derivative(
                    base,
                    integer_exponent(self.model.alpha_off)
                        .expect("negative IOFF requires an integer ALPHAOFF"),
                )
            } else {
                self.model.alpha_off * base.powf(self.model.alpha_off - 1.0)
            };
            let scaled_k = self.model.k_off * self.model.x_scaling;
            (
                -scaled_k * power,
                -scaled_k * power_derivative / self.model.i_off,
            )
        } else if current <= self.model.i_on {
            let base = current / self.model.i_on - 1.0;
            let power = if base < 0.0 {
                base.powi(
                    integer_exponent(self.model.alpha_on)
                        .expect("positive ION requires an integer ALPHAON"),
                )
            } else {
                base.powf(self.model.alpha_on)
            };
            let power_derivative = if base == 0.0 && self.model.alpha_on == 1.0 {
                1.0
            } else if base < 0.0 {
                powi_derivative(
                    base,
                    integer_exponent(self.model.alpha_on)
                        .expect("positive ION requires an integer ALPHAON"),
                )
            } else {
                self.model.alpha_on * base.powf(self.model.alpha_on - 1.0)
            };
            let scaled_k = self.model.k_on * self.model.x_scaling;
            (
                -scaled_k * power,
                -scaled_k * power_derivative / self.model.i_on,
            )
        } else {
            (0.0, 0.0)
        }
    }
}

fn require(
    condition: bool,
    name: &'static str,
    reason: &'static str,
) -> Result<(), XyceTeamMemristorError> {
    if condition {
        Ok(())
    } else {
        Err(XyceTeamMemristorError::InvalidParameter { name, reason })
    }
}

fn finite_input(name: &'static str, value: Value) -> Result<(), XyceTeamMemristorError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(XyceTeamMemristorError::NonFiniteInput { name })
    }
}

fn integer_exponent(value: Value) -> Option<i32> {
    if !value.is_finite() || value <= 0.0 || value > i32::MAX as Value {
        return None;
    }
    let rounded = value.round();
    if (value - rounded).abs() <= 16.0 * Value::EPSILON * value.abs().max(1.0) {
        Some(rounded as i32)
    } else {
        None
    }
}

#[inline]
fn powi_derivative(base: Value, exponent: i32) -> Value {
    if exponent == 1 {
        1.0
    } else {
        exponent as Value * base.powi(exponent - 1)
    }
}

#[inline]
fn powf_derivative(base: Value, exponent: Value) -> Value {
    if base == 0.0 && exponent == 1.0 {
        1.0
    } else {
        exponent * base.powf(exponent - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_model(iv_relation: i32, window_type: i32) -> XyceTeamMemristor {
        let model = XyceTeamModelParams {
            k_on: -4.68e-12,
            k_off: 1.46e-12,
            alpha_on: 3.0,
            alpha_off: 4.0,
            x_on: 0.0,
            x_off: 3.0e-9,
            r_on: 50.0,
            r_off: 1000.0,
            i_on: -8.9e-6,
            i_off: 115.0e-6,
            x_scaling: 1.0e9,
            d: 3.0,
            p: 2.0,
            j: 0.8,
            a_on: 0.0,
            a_off: 3.0e-9,
            wc: 1.0e-9,
            window_type,
        };
        XyceTeamMemristor::new(model, XyceTeamInstanceParams { iv_relation }).unwrap()
    }

    fn assert_close(actual: Value, expected: Value, relative: Value, absolute: Value) {
        let tolerance = absolute + relative * actual.abs().max(expected.abs());
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual:.16e}, expected={expected:.16e}, tolerance={tolerance:.3e}"
        );
    }

    #[test]
    fn defaults_match_xyce_7_10_level_two_metadata() {
        let model = XyceTeamModelParams::default();
        assert_eq!(XYCE_TEAM_MEMRISTOR_LEVEL, 2);
        assert_eq!(model.k_on, -8.0e-13);
        assert_eq!(model.k_off, 8.0e-13);
        assert_eq!(model.alpha_on, 3.0);
        assert_eq!(model.alpha_off, 3.0);
        assert_eq!(model.x_on, 0.0);
        assert_eq!(model.x_off, 3.0e-9);
        assert_eq!(model.r_on, 50.0);
        assert_eq!(model.r_off, 1.0e3);
        assert_eq!(model.i_on, 8.9e-6);
        assert_eq!(model.i_off, 1.15e-4);
        assert_eq!(model.x_scaling, 1.0);
        assert_eq!(model.d, 1.15e-4);
        assert_eq!(model.p, 1.15e-4);
        assert_eq!(model.j, 1.15e-4);
        assert_eq!(model.a_on, 0.0);
        assert_eq!(model.a_off, 3.0e-9);
        assert_eq!(model.wc, 1.07e-12);
        assert_eq!(model.window_type, 0);
        assert_eq!(XyceTeamInstanceParams::default().iv_relation, 0);
        XyceTeamMemristor::new(model, XyceTeamInstanceParams::default()).unwrap();
    }

    #[test]
    fn validation_rejects_undefined_parameter_domains() {
        let mut model = XyceTeamModelParams::default();
        model.x_scaling = 0.0;
        assert!(matches!(
            model.validate(),
            Err(XyceTeamMemristorError::InvalidParameter {
                name: "XSCALING",
                ..
            })
        ));

        model = XyceTeamModelParams::default();
        model.window_type = 5;
        assert_eq!(
            model.validate(),
            Err(XyceTeamMemristorError::InvalidWindowType(5))
        );
        assert_eq!(
            XyceTeamInstanceParams { iv_relation: 2 }.validate(),
            Err(XyceTeamMemristorError::InvalidIvRelation(2))
        );
    }

    #[test]
    fn xscaling_maps_physical_endpoints_without_changing_endpoint_resistance() {
        for iv_relation in 0..=1 {
            let device = test_model(iv_relation, 0);
            let (at_on, _) = device.resistance(device.model().scaled_x_on()).unwrap();
            let (at_off, _) = device.resistance(device.model().scaled_x_off()).unwrap();
            assert_close(at_on, device.model().r_on, 1e-14, 1e-14);
            assert_close(at_off, device.model().r_off, 1e-14, 1e-14);
        }
    }

    #[test]
    fn state_is_not_hard_clamped_to_model_endpoints() {
        let device = test_model(0, 0);
        let outside = device.model().scaled_x_off() + 0.25;
        let (resistance, _) = device.resistance(outside).unwrap();
        assert!(resistance > device.model().r_off);
        let cache = device.evaluate(0.1, 0.0, outside).unwrap();
        assert_eq!(cache.charge[2], outside);
    }

    #[test]
    fn exact_dae_charge_structure_is_exposed() {
        let cache = test_model(1, 4).evaluate(0.2, 0.0, 1.25).unwrap();
        assert_eq!(cache.charge, [0.0, 0.0, 1.25]);
        assert_eq!(cache.charge_jacobian, [[0.0; 3], [0.0; 3], [0.0, 0.0, 1.0]]);
        assert_eq!(cache.residual[0], -cache.residual[1]);
    }

    #[test]
    fn state_drive_is_transformed_into_the_scaled_solver_coordinate() {
        let device = test_model(0, 0);
        let cache = device.evaluate(0.20, 0.0, 1.15).unwrap();
        let base = cache.current / device.model().i_off - 1.0;
        let expected =
            -device.model().k_off * device.model().x_scaling * base.powf(device.model().alpha_off);
        assert_close(
            cache
                .state_drive
                .expect("dynamic mode reports kinetics")
                .drive,
            expected,
            1e-14,
            1e-14,
        );
    }

    #[test]
    fn dc_operating_point_gauges_the_state_to_scaled_xon() {
        // Every bias below is singular under the kinetic row: the first sits in
        // the threshold deadband, the second is driven well past IOFF.
        for (v_pos, x) in [(0.0, 1.2), (0.20, 1.15), (-0.20, 0.4)] {
            for iv_relation in 0..=1 {
                for window_type in 0..=4 {
                    let device = test_model(iv_relation, window_type);
                    let cache = device
                        .evaluate_with_mode(v_pos, 0.0, x, XyceTeamEvaluationMode::DcOperatingPoint)
                        .unwrap();
                    assert_eq!(cache.state_drive, None);
                    assert_eq!(cache.jacobian[2], [0.0, 0.0, 1.0]);
                    assert_close(
                        cache.residual[2],
                        x - device.dc_state_target(),
                        1e-14,
                        1e-14,
                    );
                    // The gauge row is exactly satisfied at the target, so one
                    // Newton step from any iterate lands on it.
                    let solved = x - cache.residual[2] / cache.jacobian[2][2];
                    assert_close(solved, device.model().scaled_x_on(), 1e-14, 1e-14);
                    // Terminal rows keep the physical resistive law.
                    assert_eq!(cache.residual[0], -cache.residual[1]);
                    assert_close(
                        cache.residual[0],
                        v_pos / device.resistance(x).unwrap().0,
                        1e-14,
                        1e-14,
                    );
                }
            }
        }
    }

    #[test]
    fn dc_gauge_target_tracks_a_nonzero_xon() {
        let mut model = XyceTeamModelParams::default();
        model.x_on = 0.5e-9;
        model.x_scaling = 1.0e9;
        let device = XyceTeamMemristor::new(model, XyceTeamInstanceParams::default()).unwrap();
        assert_close(device.dc_state_target(), 0.5, 1e-14, 1e-14);
        let cache = device
            .evaluate_with_mode(0.2, 0.0, 2.0, XyceTeamEvaluationMode::DcOperatingPoint)
            .unwrap();
        assert_close(cache.residual[2], 1.5, 1e-14, 1e-14);
        // Gauging to XON puts the operating point at RON, as Xyce's own
        // converged TEAM operating point does.
        let (resistance, _) = device.resistance(device.dc_state_target()).unwrap();
        assert_close(resistance, device.model().r_on, 1e-12, 1e-12);
    }

    #[test]
    fn unit_exponents_keep_the_physical_dc_state_root() {
        // ALPHA*=1 makes the threshold root simple, so the operating point must
        // solve the physical steady state rather than gauge it away.
        let mut model = XyceTeamModelParams::default();
        model.r_on = 50.0;
        model.r_off = 150.0;
        model.x_on = 0.0;
        model.x_off = 1.0;
        model.i_on = -1.0e-3;
        model.i_off = 1.0e-3;
        model.k_on = -1.0;
        model.k_off = 1.0;
        model.alpha_on = 1.0;
        model.alpha_off = 1.0;
        let device = XyceTeamMemristor::new(model, XyceTeamInstanceParams::default()).unwrap();
        assert!(device.has_nondegenerate_dc_state_root());

        // R=100 puts the current exactly at IOFF, which is the steady state.
        let cache = device
            .evaluate_with_mode(0.1, 0.0, 0.5, XyceTeamEvaluationMode::DcOperatingPoint)
            .unwrap();
        assert!(cache.state_drive.is_some(), "physical row must be retained");
        assert_close(cache.resistance, 100.0, 1e-12, 1e-12);
        assert_close(cache.current, 1.0e-3, 1e-12, 1e-15);
        assert_close(cache.residual[2], 0.0, 1e-12, 1e-18);
        // The root is simple, so the state derivative is usable.
        assert!(
            cache.jacobian[2][2].abs() > 1.0e-6,
            "simple root must expose a nonzero state derivative, got {:.6e}",
            cache.jacobian[2][2]
        );

        // The deadband still has no bias-determined state, so it gauges.
        let deadband = device
            .evaluate_with_mode(0.0, 0.0, 0.5, XyceTeamEvaluationMode::DcOperatingPoint)
            .unwrap();
        assert_eq!(deadband.state_drive, None);
        assert_eq!(deadband.jacobian[2], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn dc_state_root_is_degenerate_for_every_exponent_above_one() {
        for alpha in [2.0, 3.0, 4.0, 10.0] {
            let mut model = XyceTeamModelParams::default();
            model.alpha_on = alpha;
            model.alpha_off = alpha;
            let device = XyceTeamMemristor::new(model, XyceTeamInstanceParams::default()).unwrap();
            assert!(
                !device.has_nondegenerate_dc_state_root(),
                "ALPHA={alpha} root has multiplicity {alpha}"
            );
        }
    }

    #[test]
    fn degenerate_dynamic_state_row_is_singular_at_every_dc_bias() {
        // This is the property that makes the gauge necessary rather than a
        // convenience: Xyce loads the row below at its operating point and
        // reports a numerically singular matrix for the same decks.
        let device = test_model(1, 0);
        assert!(!device.has_nondegenerate_dc_state_root());
        // Inside the threshold deadband the row vanishes identically.
        let deadband = device.evaluate(0.0, 0.0, 1.2).unwrap();
        assert_eq!(deadband.residual[2], 0.0);
        assert_eq!(deadband.jacobian[2], [0.0; 3]);

        // Outside it, the only root of the row is the threshold itself, where
        // the derivative vanishes because ALPHAOFF >= 1.
        let model = device.model();
        let x_at_threshold = {
            let slope =
                (model.r_off / model.r_on).ln() / (model.scaled_x_off() - model.scaled_x_on());
            model.scaled_x_on() + (0.20 / model.i_off / model.r_on).ln() / slope
        };
        let root = device.evaluate(0.20, 0.0, x_at_threshold).unwrap();
        assert_close(root.current, model.i_off, 1e-12, 1e-15);
        assert_close(root.residual[2], 0.0, 1e-12, 1e-18);
        for derivative in root.jacobian[2] {
            assert_close(derivative, 0.0, 1e-12, 1e-12);
        }
    }

    #[test]
    fn corrected_analytic_jacobian_matches_nonunit_xscaling_finite_differences() {
        let states = [
            // Both points are safely beyond their current threshold and away
            // from every window discontinuity.
            (0.20, 0.0, 1.15),
            (-0.20, 0.0, 1.35),
        ];
        for iv_relation in 0..=1 {
            for window_type in 0..=4 {
                let device = test_model(iv_relation, window_type);
                for (v_pos, v_neg, x) in states {
                    let point = [v_pos, v_neg, x];
                    let cache = device.evaluate(point[0], point[1], point[2]).unwrap();
                    for column in 0..3 {
                        let step = if column == 2 { 2.0e-6 } else { 2.0e-7 };
                        let mut plus = point;
                        let mut minus = point;
                        plus[column] += step;
                        minus[column] -= step;
                        let f_plus = device.evaluate(plus[0], plus[1], plus[2]).unwrap();
                        let f_minus = device.evaluate(minus[0], minus[1], minus[2]).unwrap();
                        for row in 0..3 {
                            let finite_difference =
                                (f_plus.residual[row] - f_minus.residual[row]) / (2.0 * step);
                            assert_close(
                                cache.jacobian[row][column],
                                finite_difference,
                                2.0e-5,
                                2.0e-11,
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn threshold_deadband_has_zero_state_residual_and_derivatives() {
        let device = test_model(0, 4);
        let cache = device.evaluate(0.0, 0.0, 1.2).unwrap();
        assert_eq!(cache.current, 0.0);
        assert_eq!(
            cache
                .state_drive
                .expect("dynamic mode reports kinetics")
                .drive,
            0.0
        );
        assert_eq!(cache.residual[2], 0.0);
        assert_eq!(cache.jacobian[2], [0.0; 3]);
    }

    #[test]
    fn team_window_uses_scaled_a_and_wc_parameters() {
        let device = test_model(0, 4);
        let x = 2.0;
        let (positive, positive_dx) = device.window(x, 1.0e-3).unwrap();
        let expected_inner = ((x - 3.0) / 1.0).exp();
        assert_close(positive, (-expected_inner).exp(), 1e-14, 1e-14);
        assert_close(
            positive_dx,
            -(-expected_inner).exp() * expected_inner,
            1e-14,
            1e-14,
        );

        let (negative, negative_dx) = device.window(x, -1.0e-3).unwrap();
        let expected_inner = (-x).exp();
        assert_close(negative, (-expected_inner).exp(), 1e-14, 1e-14);
        assert_close(
            negative_dx,
            (-expected_inner).exp() * expected_inner,
            1e-14,
            1e-14,
        );
    }

    fn noise_params(seed: u32) -> XyceTeamResistanceNoiseParams {
        XyceTeamResistanceNoiseParams {
            enabled: true,
            seed,
            lambda: 1.0,
            update_time: 1.0e-6,
            epsilon_update_time: 1.0e-12,
            delta: 2.0,
            delta_gradient: 0.2,
        }
    }

    #[test]
    fn resistance_noise_seed_is_reproducible_and_distinguishes_streams() {
        let mut first = XyceTeamResistanceNoiseRuntime::new(noise_params(17), "YMEMRISTOR!MR1");
        let mut repeat = XyceTeamResistanceNoiseRuntime::new(noise_params(17), "YMEMRISTOR!MR1");
        let mut different = XyceTeamResistanceNoiseRuntime::new(noise_params(18), "YMEMRISTOR!MR1");
        first.initialize_accepted_boundary(0.0).unwrap();
        repeat.initialize_accepted_boundary(0.0).unwrap();
        different.initialize_accepted_boundary(0.0).unwrap();

        assert_eq!(first, repeat);
        assert_ne!(
            first
                .checkpoint("YMEMRISTOR!MR1")
                .next_update_interval
                .to_bits(),
            different
                .checkpoint("YMEMRISTOR!MR1")
                .next_update_interval
                .to_bits()
        );
    }

    #[test]
    fn rejected_noise_trial_restores_rng_and_retry_bit_exactly() {
        let mut origin = XyceTeamResistanceNoiseRuntime::new(noise_params(23), "YMEMRISTOR!MR1");
        origin.initialize_accepted_boundary(0.0).unwrap();

        let accepted = origin.clone();
        let mut rejected = origin;
        rejected.prepare_trial(1.0).unwrap();
        assert_ne!(rejected, accepted, "fixture must advance the RTN trial");

        // Circuit rejection assigns the cloned nonlinear snapshot back to the
        // binding. Reproduce that exact operation before retrying.
        rejected = accepted.clone();
        rejected.prepare_trial(2.0).unwrap();
        let once = rejected.checkpoint("YMEMRISTOR!MR1");
        rejected.prepare_trial(2.0).unwrap();
        assert_eq!(
            rejected.checkpoint("YMEMRISTOR!MR1"),
            once,
            "Newton reevaluations at one trial time must not redraw"
        );

        let mut direct = accepted;
        direct.prepare_trial(2.0).unwrap();
        assert_eq!(rejected, direct, "reject/retry must be bit-exact");
    }

    #[test]
    fn resistance_noise_validation_fails_closed_and_disabled_is_equation_neutral() {
        let mut malformed = noise_params(1);
        malformed.lambda = Value::NAN;
        assert!(malformed.validate().is_err());
        malformed = noise_params(1);
        malformed.update_time = 0.0;
        assert!(malformed.validate().is_err());
        malformed = noise_params(1);
        malformed.delta = 20.0;
        assert!(malformed.validate().is_err());

        let deterministic = test_model(1, 0);
        let disabled = deterministic
            .with_resistance_noise(XyceTeamResistanceNoiseParams {
                enabled: false,
                seed: 99,
                lambda: 7.0,
                update_time: 3.0,
                epsilon_update_time: 2.0,
                delta: 4.0,
                delta_gradient: 0.1,
            })
            .unwrap();
        assert_eq!(
            deterministic.evaluate(0.25, 0.0, 1.1).unwrap(),
            disabled.evaluate(0.25, 0.0, 1.1).unwrap(),
            "RESNOISE=0 must preserve the deterministic equation bit-for-bit"
        );
    }
}
