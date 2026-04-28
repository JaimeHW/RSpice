//! Switch Device Models
//!
//! Implements voltage-controlled and current-controlled switches.
//!
//! # SPICE Syntax
//! ```text
//! S<name> n+ n- nc+ nc- <model>     ; Voltage-controlled switch
//! W<name> n+ n- Vname <model>        ; Current-controlled switch
//! .MODEL <mname> VSWITCH [params]
//! .MODEL <mname> ISWITCH [params]
//! ```
//!
//! # Model Parameters
//! ## VSWITCH (Voltage-controlled)
//! | Parameter | Description | Default |
//! |-----------|-------------|---------|
//! | VT | Threshold voltage | 0.0V |
//! | VH | Hysteresis voltage | 0.0V |
//! | RON | On resistance | 1Ω |
//! | ROFF | Off resistance | 1MΩ |
//!
//! ## ISWITCH (Current-controlled)
//! | Parameter | Description | Default |
//! |-----------|-------------|---------|
//! | IT | Threshold current | 0.0A |
//! | IH | Hysteresis current | 0.0A |
//! | RON | On resistance | 1Ω |
//! | ROFF | Off resistance | 1MΩ |
//!
//! # Implementation
//! Uses a smooth transition function to avoid discontinuities:
//! ```text
//! R = RON + (ROFF - RON) * f(x)
//! ```
//! where f(x) is a smooth step function.

use super::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Value, circuit::NodeId};

//=============================================================================
// Switch State
//=============================================================================

/// Switch state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchState {
    Off,
    On,
    /// Transitioning (used for hysteresis)
    Transitioning,
}

//=============================================================================
// Voltage-Controlled Switch
//=============================================================================

/// Voltage-Controlled Switch (SPICE S element)
#[derive(Debug, Clone)]
pub struct VoltageSwitch {
    /// Instance name
    pub name: String,
    /// Positive terminal
    pub node_pos: NodeId,
    /// Negative terminal
    pub node_neg: NodeId,
    /// Positive control node
    pub ctrl_pos: NodeId,
    /// Negative control node
    pub ctrl_neg: NodeId,

    // Model parameters
    /// Threshold voltage
    pub vt: Value,
    /// Hysteresis voltage
    pub vh: Value,
    /// On resistance
    pub ron: Value,
    /// Off resistance
    pub roff: Value,
    /// Smoothness factor (controls transition steepness)
    pub smooth: Value,

    // State
    state: SwitchState,
    prev_state: SwitchState,
    current_resistance: Value,
    prev_resistance: Value,
}

impl VoltageSwitch {
    /// Create a new voltage-controlled switch
    pub fn new(
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        ctrl_pos: NodeId,
        ctrl_neg: NodeId,
    ) -> Self {
        Self {
            name,
            node_pos,
            node_neg,
            ctrl_pos,
            ctrl_neg,
            vt: 0.0,
            vh: 0.0,
            ron: 1.0,
            roff: 1e6,
            smooth: 0.1,
            state: SwitchState::Off,
            prev_state: SwitchState::Off,
            current_resistance: 1e6,
            prev_resistance: 1e6,
        }
    }

    /// Set model parameters
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        if let Some(&v) = params.get("VT") {
            self.vt = v;
        }
        if let Some(&v) = params.get("VH") {
            self.vh = v.abs();
        }
        if let Some(&v) = params.get("RON") {
            self.ron = v.max(1e-6);
        }
        if let Some(&v) = params.get("ROFF") {
            self.roff = v.max(1e-6);
        }
        if let Some(&v) = params.get("SMOOTH") {
            self.smooth = v.max(1e-6);
        }
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
        self
    }

    /// Set thresholds
    pub fn with_thresholds(mut self, vt: Value, vh: Value) -> Self {
        self.vt = vt;
        self.vh = vh.abs();
        self
    }

    /// Set on/off resistances
    pub fn with_resistances(mut self, ron: Value, roff: Value) -> Self {
        self.ron = ron.max(1e-6);
        self.roff = roff.max(1e-6);
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
        self
    }

    /// Set initial hysteresis state.
    pub fn with_initial_state(mut self, state: SwitchState) -> Self {
        self.state = state;
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
        self
    }

    /// Get current state
    pub fn state(&self) -> SwitchState {
        self.state
    }

    /// Get current resistance
    pub fn resistance(&self) -> Value {
        self.current_resistance
    }

    /// Calculate resistance based on control voltage using smooth transition
    fn calculate_resistance(&self, vctrl: Value) -> Value {
        let (g, _) = self.control_sensitivity(vctrl);
        1.0 / g.max(1e-30)
    }

    #[inline]
    fn effective_threshold(&self) -> Value {
        match self.state {
            SwitchState::Off => self.vt + self.vh,
            SwitchState::On => self.vt - self.vh,
            SwitchState::Transitioning => self.vt,
        }
    }

    /// Evaluate main-branch conductance and its control derivative.
    ///
    /// Returns `(g, dg/dvctrl)` for the current hysteresis state.
    fn control_sensitivity(&self, vctrl: Value) -> (Value, Value) {
        let vt_eff = self.effective_threshold();
        let smooth = self.smooth.max(1e-6);
        let x = (vctrl - vt_eff) / smooth;
        let tanh_x = x.tanh();
        let f = 0.5 * (1.0 - tanh_x);

        // Interpolate in log-R domain (SPICE-compatible smooth transition).
        let log_ron = self.ron.ln();
        let log_roff = self.roff.ln();
        let dlog_r = log_roff - log_ron;
        let log_r = log_ron + dlog_r * f;
        let g = (-log_r).exp();

        // d/dx tanh(x) = sech^2(x) = 1 - tanh^2(x)
        let sech2 = 1.0 - tanh_x * tanh_x;
        let df_dvctrl = -0.5 * sech2 / smooth;
        let dlogr_dvctrl = dlog_r * df_dvctrl;
        let dg_dvctrl = -g * dlogr_dvctrl;

        (g, dg_dvctrl)
    }

    /// Update state based on control voltage (with hysteresis)
    fn update_state(&mut self, vctrl: Value) {
        match self.state {
            SwitchState::Off => {
                if vctrl > self.vt + self.vh {
                    self.state = SwitchState::On;
                }
            }
            SwitchState::On => {
                if vctrl < self.vt - self.vh {
                    self.state = SwitchState::Off;
                }
            }
            SwitchState::Transitioning => {
                if vctrl > self.vt + self.vh {
                    self.state = SwitchState::On;
                } else if vctrl < self.vt - self.vh {
                    self.state = SwitchState::Off;
                }
            }
        }
    }
}

impl NonlinearDevice for VoltageSwitch {
    fn update(&mut self, voltages: &[Value]) {
        let vctrl_pos = if self.ctrl_pos > 0 {
            voltages[self.ctrl_pos - 1]
        } else {
            0.0
        };
        let vctrl_neg = if self.ctrl_neg > 0 {
            voltages[self.ctrl_neg - 1]
        } else {
            0.0
        };
        let vctrl = vctrl_pos - vctrl_neg;

        self.prev_state = self.state;
        self.prev_resistance = self.current_resistance;
        self.update_state(vctrl);
        self.current_resistance = self.calculate_resistance(vctrl);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let vp = if self.node_pos > 0 {
            voltages[self.node_pos - 1]
        } else {
            0.0
        };
        let vn = if self.node_neg > 0 {
            voltages[self.node_neg - 1]
        } else {
            0.0
        };
        let vctrl_pos = if self.ctrl_pos > 0 {
            voltages[self.ctrl_pos - 1]
        } else {
            0.0
        };
        let vctrl_neg = if self.ctrl_neg > 0 {
            voltages[self.ctrl_neg - 1]
        } else {
            0.0
        };
        let vctrl = vctrl_pos - vctrl_neg;
        let vmain = vp - vn;
        let (g, dg_dvctrl) = self.control_sensitivity(vctrl);
        let gm_ctrl = dg_dvctrl * vmain;

        // I(p->n) = g(vctrl) * (vp - vn)
        let i = g * vmain;
        // Linearization: I ≈ Σ J_k * x_k + Ieq
        let ieq = i - (g * vp) - (-g * vn) - (gm_ctrl * vctrl_pos) - (-gm_ctrl * vctrl_neg);

        // Main branch Jacobian terms.
        matrix.stamp(self.node_pos, self.node_pos, g);
        matrix.stamp(self.node_pos, self.node_neg, -g);
        matrix.stamp(self.node_neg, self.node_pos, -g);
        matrix.stamp(self.node_neg, self.node_neg, g);

        // Control Jacobian terms (row coupling to control nodes).
        matrix.stamp(self.node_pos, self.ctrl_pos, gm_ctrl);
        matrix.stamp(self.node_pos, self.ctrl_neg, -gm_ctrl);
        matrix.stamp(self.node_neg, self.ctrl_pos, -gm_ctrl);
        matrix.stamp(self.node_neg, self.ctrl_neg, gm_ctrl);

        // Equivalent current source for linearized residual.
        matrix.stamp_rhs(self.node_pos, -ieq);
        matrix.stamp_rhs(self.node_neg, ieq);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if self.state != self.prev_state {
            return false;
        }

        let tolerance = criteria.voltage_tolerance();
        let denom = self
            .current_resistance
            .abs()
            .max(self.prev_resistance.abs())
            .max(1e-12);
        let rel = (self.current_resistance - self.prev_resistance).abs() / denom;
        rel < tolerance.max(1e-3)
    }
}

//=============================================================================
// Current-Controlled Switch
//=============================================================================

/// Current-Controlled Switch (SPICE W element)
#[derive(Debug, Clone)]
pub struct CurrentSwitch {
    /// Instance name
    pub name: String,
    /// Positive terminal
    pub node_pos: NodeId,
    /// Negative terminal
    pub node_neg: NodeId,
    /// Branch index of the controlling current source
    pub ctrl_branch: Option<NodeId>,
    /// Name of controlling voltage source (for reference)
    pub ctrl_source: String,

    // Model parameters
    /// Threshold current
    pub it: Value,
    /// Hysteresis current
    pub ih: Value,
    /// On resistance
    pub ron: Value,
    /// Off resistance
    pub roff: Value,
    /// Smoothness factor
    pub smooth: Value,

    // State
    state: SwitchState,
    prev_state: SwitchState,
    current_resistance: Value,
    prev_resistance: Value,
}

impl CurrentSwitch {
    /// Create a new current-controlled switch
    pub fn new(name: String, node_pos: NodeId, node_neg: NodeId, ctrl_source: String) -> Self {
        Self {
            name,
            node_pos,
            node_neg,
            ctrl_branch: None,
            ctrl_source,
            it: 0.0,
            ih: 0.0,
            ron: 1.0,
            roff: 1e6,
            smooth: 0.001, // 1mA smooth region
            state: SwitchState::Off,
            prev_state: SwitchState::Off,
            current_resistance: 1e6,
            prev_resistance: 1e6,
        }
    }

    /// Set the controlling branch index
    pub fn set_ctrl_branch(&mut self, branch: NodeId) {
        self.ctrl_branch = Some(branch);
    }

    /// Set model parameters
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        if let Some(&v) = params.get("IT") {
            self.it = v;
        }
        if let Some(&v) = params.get("IH") {
            self.ih = v.abs();
        }
        if let Some(&v) = params.get("RON") {
            self.ron = v.max(1e-6);
        }
        if let Some(&v) = params.get("ROFF") {
            self.roff = v.max(1e-6);
        }
        if let Some(&v) = params.get("SMOOTH") {
            self.smooth = v.max(1e-9);
        }
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
        self
    }

    /// Set thresholds
    pub fn with_thresholds(mut self, it: Value, ih: Value) -> Self {
        self.it = it;
        self.ih = ih.abs();
        self
    }

    /// Set on/off resistances
    pub fn with_resistances(mut self, ron: Value, roff: Value) -> Self {
        self.ron = ron.max(1e-6);
        self.roff = roff.max(1e-6);
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
        self
    }

    /// Set initial hysteresis state.
    pub fn with_initial_state(mut self, state: SwitchState) -> Self {
        self.state = state;
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
        self
    }

    /// Get current state
    pub fn state(&self) -> SwitchState {
        self.state
    }

    /// Get current resistance
    pub fn resistance(&self) -> Value {
        self.current_resistance
    }

    /// Calculate resistance based on control current
    fn calculate_resistance(&self, ictrl: Value) -> Value {
        let (g, _) = self.control_sensitivity(ictrl);
        1.0 / g.max(1e-30)
    }

    #[inline]
    fn effective_threshold(&self) -> Value {
        match self.state {
            SwitchState::Off => self.it + self.ih,
            SwitchState::On => self.it - self.ih,
            SwitchState::Transitioning => self.it,
        }
    }

    /// Evaluate main-branch conductance and its control derivative.
    ///
    /// Returns `(g, dg/dictrl)` for the current hysteresis state.
    fn control_sensitivity(&self, ictrl: Value) -> (Value, Value) {
        let it_eff = self.effective_threshold();
        let smooth = self.smooth.max(1e-9);
        let x = (ictrl - it_eff) / smooth;
        let tanh_x = x.tanh();
        let f = 0.5 * (1.0 - tanh_x);

        let log_ron = self.ron.ln();
        let log_roff = self.roff.ln();
        let dlog_r = log_roff - log_ron;
        let log_r = log_ron + dlog_r * f;
        let g = (-log_r).exp();

        let sech2 = 1.0 - tanh_x * tanh_x;
        let df_dictrl = -0.5 * sech2 / smooth;
        let dlogr_dictrl = dlog_r * df_dictrl;
        let dg_dictrl = -g * dlogr_dictrl;

        (g, dg_dictrl)
    }

    /// Update state with hysteresis
    fn update_state(&mut self, ictrl: Value) {
        match self.state {
            SwitchState::Off => {
                if ictrl > self.it + self.ih {
                    self.state = SwitchState::On;
                }
            }
            SwitchState::On => {
                if ictrl < self.it - self.ih {
                    self.state = SwitchState::Off;
                }
            }
            SwitchState::Transitioning => {
                if ictrl > self.it + self.ih {
                    self.state = SwitchState::On;
                } else if ictrl < self.it - self.ih {
                    self.state = SwitchState::Off;
                }
            }
        }
    }
}

impl NonlinearDevice for CurrentSwitch {
    fn update(&mut self, voltages: &[Value]) {
        let ictrl = if let Some(branch) = self.ctrl_branch {
            if branch > 0 && branch <= voltages.len() {
                voltages[branch - 1]
            } else {
                0.0
            }
        } else {
            0.0
        };

        self.prev_state = self.state;
        self.prev_resistance = self.current_resistance;
        self.update_state(ictrl);
        self.current_resistance = self.calculate_resistance(ictrl);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let vp = if self.node_pos > 0 {
            voltages[self.node_pos - 1]
        } else {
            0.0
        };
        let vn = if self.node_neg > 0 {
            voltages[self.node_neg - 1]
        } else {
            0.0
        };
        let ictrl = if let Some(branch) = self.ctrl_branch {
            if branch > 0 && branch <= voltages.len() {
                voltages[branch - 1]
            } else {
                0.0
            }
        } else {
            0.0
        };
        let vmain = vp - vn;
        let (g, dg_dictrl) = self.control_sensitivity(ictrl);
        let g_ctrl = dg_dictrl * vmain;

        let i = g * vmain;
        let ieq = i - (g * vp) - (-g * vn) - (g_ctrl * ictrl);

        matrix.stamp(self.node_pos, self.node_pos, g);
        matrix.stamp(self.node_pos, self.node_neg, -g);
        matrix.stamp(self.node_neg, self.node_pos, -g);
        matrix.stamp(self.node_neg, self.node_neg, g);

        if let Some(branch) = self.ctrl_branch {
            matrix.stamp(self.node_pos, branch, g_ctrl);
            matrix.stamp(self.node_neg, branch, -g_ctrl);
        }

        matrix.stamp_rhs(self.node_pos, -ieq);
        matrix.stamp_rhs(self.node_neg, ieq);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if self.state != self.prev_state {
            return false;
        }

        let tolerance = criteria.voltage_tolerance();
        let denom = self
            .current_resistance
            .abs()
            .max(self.prev_resistance.abs())
            .max(1e-12);
        let rel = (self.current_resistance - self.prev_resistance).abs() / denom;
        rel < tolerance.max(1e-3)
    }
}

//=============================================================================
// Tests
//=============================================================================
