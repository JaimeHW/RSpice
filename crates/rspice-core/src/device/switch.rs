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

use crate::{circuit::NodeId, Value};
use super::traits::{NonlinearDevice, MatrixStamper};

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
    current_resistance: Value,
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
            current_resistance: 1e6,
        }
    }

    /// Set model parameters
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        if let Some(&v) = params.get("VT") { self.vt = v; }
        if let Some(&v) = params.get("VH") { self.vh = v; }
        if let Some(&v) = params.get("RON") { self.ron = v.max(1e-6); }
        if let Some(&v) = params.get("ROFF") { self.roff = v.max(1e-6); }
        if let Some(&v) = params.get("SMOOTH") { self.smooth = v.max(1e-6); }
        self.current_resistance = self.roff;
        self
    }

    /// Set thresholds
    pub fn with_thresholds(mut self, vt: Value, vh: Value) -> Self {
        self.vt = vt;
        self.vh = vh;
        self
    }

    /// Set on/off resistances
    pub fn with_resistances(mut self, ron: Value, roff: Value) -> Self {
        self.ron = ron.max(1e-6);
        self.roff = roff.max(1e-6);
        self.current_resistance = self.roff;
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
        // Smooth step function using tanh
        // x = (vctrl - vt) / smooth
        // f(x) = 0.5 * (1 - tanh(x))  -> 1 when off, 0 when on
        
        let x = (vctrl - self.vt) / self.smooth.max(1e-6);
        let f = 0.5 * (1.0 - x.tanh());
        
        // Interpolate between RON and ROFF
        let log_ron = self.ron.ln();
        let log_roff = self.roff.ln();
        let log_r = log_ron + (log_roff - log_ron) * f;
        
        log_r.exp()
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
        let vctrl_pos = if self.ctrl_pos > 0 { voltages[self.ctrl_pos - 1] } else { 0.0 };
        let vctrl_neg = if self.ctrl_neg > 0 { voltages[self.ctrl_neg - 1] } else { 0.0 };
        let vctrl = vctrl_pos - vctrl_neg;
        
        self.update_state(vctrl);
        self.current_resistance = self.calculate_resistance(vctrl);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let vctrl_pos = if self.ctrl_pos > 0 { voltages[self.ctrl_pos - 1] } else { 0.0 };
        let vctrl_neg = if self.ctrl_neg > 0 { voltages[self.ctrl_neg - 1] } else { 0.0 };
        let vctrl = vctrl_pos - vctrl_neg;
        
        let r = self.calculate_resistance(vctrl);
        let g = 1.0 / r;
        
        // Stamp as a conductance between nodes
        matrix.stamp(self.node_pos, self.node_pos, g);
        matrix.stamp(self.node_pos, self.node_neg, -g);
        matrix.stamp(self.node_neg, self.node_pos, -g);
        matrix.stamp(self.node_neg, self.node_neg, g);
    }

    fn is_converged(&self, _tolerance: Value) -> bool {
        true
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
    current_resistance: Value,
}

impl CurrentSwitch {
    /// Create a new current-controlled switch
    pub fn new(
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        ctrl_source: String,
    ) -> Self {
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
            smooth: 0.001,  // 1mA smooth region
            state: SwitchState::Off,
            current_resistance: 1e6,
        }
    }

    /// Set the controlling branch index
    pub fn set_ctrl_branch(&mut self, branch: NodeId) {
        self.ctrl_branch = Some(branch);
    }

    /// Set model parameters
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        if let Some(&v) = params.get("IT") { self.it = v; }
        if let Some(&v) = params.get("IH") { self.ih = v; }
        if let Some(&v) = params.get("RON") { self.ron = v.max(1e-6); }
        if let Some(&v) = params.get("ROFF") { self.roff = v.max(1e-6); }
        if let Some(&v) = params.get("SMOOTH") { self.smooth = v.max(1e-9); }
        self.current_resistance = self.roff;
        self
    }

    /// Set thresholds
    pub fn with_thresholds(mut self, it: Value, ih: Value) -> Self {
        self.it = it;
        self.ih = ih;
        self
    }

    /// Set on/off resistances
    pub fn with_resistances(mut self, ron: Value, roff: Value) -> Self {
        self.ron = ron.max(1e-6);
        self.roff = roff.max(1e-6);
        self.current_resistance = self.roff;
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
        let x = (ictrl - self.it) / self.smooth.max(1e-9);
        let f = 0.5 * (1.0 - x.tanh());
        
        let log_ron = self.ron.ln();
        let log_roff = self.roff.ln();
        let log_r = log_ron + (log_roff - log_ron) * f;
        
        log_r.exp()
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
        
        self.update_state(ictrl);
        self.current_resistance = self.calculate_resistance(ictrl);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let ictrl = if let Some(branch) = self.ctrl_branch {
            if branch > 0 && branch <= voltages.len() {
                voltages[branch - 1]
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        let r = self.calculate_resistance(ictrl);
        let g = 1.0 / r;
        
        matrix.stamp(self.node_pos, self.node_pos, g);
        matrix.stamp(self.node_pos, self.node_neg, -g);
        matrix.stamp(self.node_neg, self.node_pos, -g);
        matrix.stamp(self.node_neg, self.node_neg, g);
    }

    fn is_converged(&self, _tolerance: Value) -> bool {
        true
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0)
            .with_thresholds(2.5, 0.5);
        
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
        
        let sw = VoltageSwitch::new("S1".to_string(), 1, 2, 3, 0)
            .with_params(&params);
        
        assert_eq!(sw.vt, 3.3);
        assert_eq!(sw.vh, 0.3);
        assert_eq!(sw.ron, 0.1);
        assert_eq!(sw.roff, 1e9);
    }
}
