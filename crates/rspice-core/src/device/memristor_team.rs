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
    pub window: Value,
    /// Kinetic term before multiplication by the window.
    pub state_drive: Value,
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
}

impl XyceTeamMemristor {
    pub fn new(
        model: XyceTeamModelParams,
        instance: XyceTeamInstanceParams,
    ) -> Result<Self, XyceTeamMemristorError> {
        model.validate()?;
        instance.validate()?;
        Ok(Self { model, instance })
    }

    #[inline]
    pub fn model(&self) -> &XyceTeamModelParams {
        &self.model
    }

    #[inline]
    pub fn instance(&self) -> &XyceTeamInstanceParams {
        &self.instance
    }

    /// Evaluate effective resistance and its derivative with respect to the
    /// scaled solver state.
    pub fn resistance(&self, x_scaled: Value) -> Result<(Value, Value), XyceTeamMemristorError> {
        finite_input("x", x_scaled)?;
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

    /// Evaluate `F`, `Q`, and their exact analytic Jacobians.
    pub fn evaluate(
        &self,
        v_pos: Value,
        v_neg: Value,
        x_scaled: Value,
    ) -> Result<XyceTeamMemristorCache, XyceTeamMemristorError> {
        finite_input("v_pos", v_pos)?;
        finite_input("v_neg", v_neg)?;
        finite_input("x", x_scaled)?;

        let voltage = v_pos - v_neg;
        let (resistance, dresistance_dx) = self.resistance(x_scaled)?;
        let conductance = resistance.recip();
        let current = voltage * conductance;
        let dcurrent_dv_pos = conductance;
        let dcurrent_dv_neg = -conductance;
        let dcurrent_dx = -voltage * dresistance_dx / (resistance * resistance);

        let (state_drive, dstate_drive_di) = self.state_drive(current);
        let (window, dwindow_dx) = self.window(x_scaled, current)?;
        let state_residual = state_drive * window;
        let dstate_di = dstate_drive_di * window;
        let dstate_dx = dstate_di * dcurrent_dx + state_drive * dwindow_dx;

        let residual = [current, -current, state_residual];
        let jacobian = [
            [dcurrent_dv_pos, dcurrent_dv_neg, dcurrent_dx],
            [-dcurrent_dv_pos, -dcurrent_dv_neg, -dcurrent_dx],
            [
                dstate_di * dcurrent_dv_pos,
                dstate_di * dcurrent_dv_neg,
                dstate_dx,
            ],
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
            window,
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
        assert_close(cache.state_drive, expected, 1e-14, 1e-14);
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
        assert_eq!(cache.state_drive, 0.0);
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
}
