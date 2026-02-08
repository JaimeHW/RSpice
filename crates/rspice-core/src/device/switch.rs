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

use super::traits::{MatrixStamper, NonlinearDevice};
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

    fn is_converged(&self, tolerance: Value) -> bool {
        if self.state != self.prev_state {
            return false;
        }

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

    fn is_converged(&self, tolerance: Value) -> bool {
        if self.state != self.prev_state {
            return false;
        }

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

#[cfg(test)]
mod tests {
    use super::super::traits::MatrixStamper;
    use super::*;
    use crate::circuit::NodeId;
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

        fn row_current(&self, row: NodeId, variables: &[Value]) -> Value {
            let mut sum = 0.0;
            for (&(r, c), &v) in &self.entries {
                if r == row && c > 0 && c <= variables.len() {
                    sum += v * variables[c - 1];
                }
            }
            sum - self.i(row)
        }
    }

    impl MatrixStamper for CaptureMatrix {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            if row == 0 || col == 0 {
                return;
            }
            *self.entries.entry((row, col)).or_insert(0.0) += value;
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            if index == 0 {
                return;
            }
            *self.rhs.entry(index).or_insert(0.0) += value;
        }
    }

    #[test]
    fn test_vswitch_creation() {
        let sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0);
        assert_eq!(sw.name, "S1");
        assert_eq!(sw.state(), SwitchState::Off);
    }

    #[test]
    fn test_vswitch_resistance() {
        let sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0)
            .with_resistances(1.0, 1e6)
            .with_thresholds(2.5, 0.5);

        // Below threshold -> high resistance
        let r_off = sw.calculate_resistance(0.0);
        assert!(r_off > 1e4);

        // Above threshold -> low resistance
        let r_on = sw.calculate_resistance(5.0);
        assert!(r_on < 100.0);
    }

    #[test]
    fn test_vswitch_hysteresis() {
        let mut sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0).with_thresholds(2.5, 0.5);

        // Start off
        assert_eq!(sw.state(), SwitchState::Off);

        // Below turn-on threshold (2.5 + 0.5 = 3.0) -> still off
        sw.update_state(2.8);
        assert_eq!(sw.state(), SwitchState::Off);

        // Above turn-on threshold -> on
        sw.update_state(3.5);
        assert_eq!(sw.state(), SwitchState::On);

        // Above turn-off threshold (2.5 - 0.5 = 2.0) -> still on
        sw.update_state(2.2);
        assert_eq!(sw.state(), SwitchState::On);

        // Below turn-off threshold -> off
        sw.update_state(1.5);
        assert_eq!(sw.state(), SwitchState::Off);
    }

    #[test]
    fn test_cswitch_creation() {
        let sw = CurrentSwitch::new("W1".to_string(), 1, 2, "Vsense".to_string());
        assert_eq!(sw.name, "W1");
        assert_eq!(sw.ctrl_source, "Vsense");
    }

    #[test]
    fn test_cswitch_resistance() {
        let sw = CurrentSwitch::new("W1".to_string(), 1, 2, "Vsense".to_string())
            .with_resistances(1.0, 1e6)
            .with_thresholds(0.001, 0.0); // 1mA threshold

        // Below threshold -> high resistance
        let r_off = sw.calculate_resistance(0.0);
        assert!(r_off > 1e4);

        // Above threshold -> low resistance
        let r_on = sw.calculate_resistance(0.01);
        assert!(r_on < 100.0);
    }

    #[test]
    fn test_smooth_transition() {
        let sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0)
            .with_resistances(1.0, 1e6)
            .with_thresholds(2.5, 0.0);

        // At threshold, resistance should be geometric mean
        let r_mid = sw.calculate_resistance(2.5);
        let geo_mean = (1.0_f64 * 1e6).sqrt();

        // Should be within factor of 2 of geometric mean
        assert!(r_mid > geo_mean / 2.0 && r_mid < geo_mean * 2.0);
    }

    #[test]
    fn test_switch_params() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("VT".to_string(), 3.3);
        params.insert("VH".to_string(), 0.3);
        params.insert("RON".to_string(), 0.1);
        params.insert("ROFF".to_string(), 1e9);

        let sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0).with_params(&params);

        assert_eq!(sw.vt, 3.3);
        assert_eq!(sw.vh, 0.3);
        assert_eq!(sw.ron, 0.1);
        assert_eq!(sw.roff, 1e9);
    }

    #[test]
    fn test_vswitch_initial_state_shifts_hysteresis_window() {
        let sw_off = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0)
            .with_resistances(1.0, 1e9)
            .with_thresholds(1.0, 0.2)
            .with_initial_state(SwitchState::Off);
        let sw_on = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0)
            .with_resistances(1.0, 1e9)
            .with_thresholds(1.0, 0.2)
            .with_initial_state(SwitchState::On);

        let r_off = sw_off.calculate_resistance(1.0);
        let r_on = sw_on.calculate_resistance(1.0);

        assert!(
            r_off > 1e6,
            "OFF initial state in hysteresis window should remain high-R, got {}",
            r_off
        );
        assert!(
            r_on < 1e3,
            "ON initial state in hysteresis window should remain low-R, got {}",
            r_on
        );
    }

    #[test]
    fn test_cswitch_initial_state_shifts_hysteresis_window() {
        let sw_off = CurrentSwitch::new("W1".to_string(), 1, 2, "VSENSE".to_string())
            .with_resistances(1.0, 1e9)
            .with_thresholds(1e-3, 2e-4)
            .with_initial_state(SwitchState::Off);
        let sw_on = CurrentSwitch::new("W1".to_string(), 1, 2, "VSENSE".to_string())
            .with_resistances(1.0, 1e9)
            .with_thresholds(1e-3, 2e-4)
            .with_initial_state(SwitchState::On);

        let r_off = sw_off.calculate_resistance(1e-3);
        let r_on = sw_on.calculate_resistance(1e-3);

        assert!(
            r_off > 1e4,
            "OFF initial state in hysteresis window should remain high-R, got {}",
            r_off
        );
        assert!(
            r_on < 1e4,
            "ON initial state in hysteresis window should remain low-R, got {}",
            r_on
        );
        assert!(
            r_off > r_on * 50.0,
            "initial state should create clear hysteretic resistance separation: off={} on={}",
            r_off,
            r_on
        );
    }

    #[test]
    fn test_vswitch_stamp_includes_control_jacobian_and_linearized_rhs() {
        let sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 4)
            .with_resistances(10.0, 1e6)
            .with_thresholds(0.0, 0.0)
            .with_initial_state(SwitchState::Transitioning);

        let x0 = vec![0.6, 0.1, 0.05, -0.02];
        let mut matrix = CaptureMatrix::default();
        sw.stamp_nonlinear(&x0, &mut matrix, &mut []);

        assert!(
            matrix.g(1, 3).abs() > 1e-12 && matrix.g(1, 4).abs() > 1e-12,
            "control-node Jacobian terms must be present"
        );
        assert!(
            (matrix.g(1, 3) + matrix.g(2, 3)).abs() < 1e-12
                && (matrix.g(1, 4) + matrix.g(2, 4)).abs() < 1e-12,
            "switch branch Jacobian rows should be antisymmetric"
        );
        assert!(
            (matrix.i(1) + matrix.i(2)).abs() < 1e-12,
            "KCL requires equal-and-opposite RHS injections"
        );

        // Validate first-order linearization against the nonlinear current.
        let x1 = vec![x0[0] + 2e-4, x0[1] - 1e-4, x0[2] + 1e-4, x0[3] - 8e-5];
        let i_lin = matrix.row_current(1, &x1);
        let vmain = x1[0] - x1[1];
        let vctrl = x1[2] - x1[3];
        let i_nl = vmain / sw.calculate_resistance(vctrl);
        let rel_err = (i_lin - i_nl).abs() / i_nl.abs().max(1e-12);
        assert!(
            rel_err < 5e-3,
            "vswitch linearization mismatch: i_lin={} i_nl={} rel_err={}",
            i_lin,
            i_nl,
            rel_err
        );
    }

    #[test]
    fn test_iswitch_stamp_includes_control_branch_jacobian_and_linearized_rhs() {
        let mut sw = CurrentSwitch::new("W1".to_string(), 1, 2, "VSENSE".to_string())
            .with_resistances(5.0, 1e6)
            .with_thresholds(1e-3, 0.0)
            .with_initial_state(SwitchState::Transitioning);
        sw.set_ctrl_branch(3);

        let x0 = vec![0.45, 0.2, 1.1e-3];
        let mut matrix = CaptureMatrix::default();
        sw.stamp_nonlinear(&x0, &mut matrix, &mut []);

        assert!(
            matrix.g(1, 3).abs() > 1e-12,
            "control-branch Jacobian term must be present"
        );
        assert!(
            (matrix.g(1, 3) + matrix.g(2, 3)).abs() < 1e-12,
            "iswitch branch Jacobian rows should be antisymmetric"
        );
        assert!(
            (matrix.i(1) + matrix.i(2)).abs() < 1e-12,
            "KCL requires equal-and-opposite RHS injections"
        );

        let x1 = vec![x0[0] + 1e-4, x0[1] - 6e-5, x0[2] + 7e-6];
        let i_lin = matrix.row_current(1, &x1);
        let vmain = x1[0] - x1[1];
        let ictrl = x1[2];
        let i_nl = vmain / sw.calculate_resistance(ictrl);
        let rel_err = (i_lin - i_nl).abs() / i_nl.abs().max(1e-12);
        assert!(
            rel_err < 5e-3,
            "iswitch linearization mismatch: i_lin={} i_nl={} rel_err={}",
            i_lin,
            i_nl,
            rel_err
        );
    }

    #[test]
    fn test_switch_convergence_requires_state_and_resistance_settling() {
        let mut sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0)
            .with_resistances(1.0, 1e6)
            .with_thresholds(1.0, 0.2);

        assert!(sw.is_converged(1e-9), "fresh switch should start converged");

        // First update crosses ON threshold and should be marked non-converged.
        sw.update(&[0.0, 0.0, 2.0]);
        assert!(
            !sw.is_converged(1e-9),
            "state transition must require another Newton iteration"
        );

        // Re-applying the same operating point should settle convergence.
        sw.update(&[0.0, 0.0, 2.0]);
        assert!(
            sw.is_converged(1e-9),
            "stable state/resistance should report converged"
        );
    }
}
