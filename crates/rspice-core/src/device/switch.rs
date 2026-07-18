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
use crate::expr::{
    BinaryOp, CompiledExpr, Context, Expr, Function, UnaryOp, Vm, compile, parse_expression_strict,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XyceSwitchHysteresisSide {
    Off,
    On,
}

#[derive(Debug, Clone, Copy)]
struct XyceSwitchCurve {
    on: Value,
    off: Value,
    onh: Value,
    offh: Value,
    hysteresis_enabled: bool,
    sticky_off_transition: bool,
    last_state: Value,
    transition_hold: Option<XyceSwitchHysteresisSide>,
    pending_side: Option<XyceSwitchHysteresisSide>,
    departing_on: bool,
    off_departure_bridge_count: u8,
    pending_departing_on: bool,
    pending_off_departure_bridge_count: u8,
}

#[derive(Debug, Clone, Copy)]
struct XyceSwitchEvaluation {
    conductance: Value,
    dconductance_dcontrol: Value,
    state: SwitchState,
}

impl XyceSwitchCurve {
    fn new(
        on: Value,
        off: Value,
        onh: Value,
        offh: Value,
        hysteresis_enabled: bool,
        sticky_off_transition: bool,
    ) -> Self {
        Self {
            on,
            off,
            onh,
            offh,
            hysteresis_enabled,
            sticky_off_transition,
            last_state: 0.0,
            transition_hold: Some(XyceSwitchHysteresisSide::Off),
            pending_side: Some(XyceSwitchHysteresisSide::Off),
            departing_on: false,
            off_departure_bridge_count: 0,
            pending_departing_on: false,
            pending_off_departure_bridge_count: 0,
        }
    }

    fn set_initial_state(&mut self, state: SwitchState) {
        match state {
            SwitchState::On => {
                self.last_state = 1.0;
                self.transition_hold = Some(XyceSwitchHysteresisSide::On);
                self.pending_side = Some(XyceSwitchHysteresisSide::On);
                self.departing_on = false;
                self.off_departure_bridge_count = 0;
                self.pending_departing_on = false;
                self.pending_off_departure_bridge_count = 0;
            }
            SwitchState::Off | SwitchState::Transitioning => {
                self.last_state = 0.0;
                self.transition_hold = Some(XyceSwitchHysteresisSide::Off);
                self.pending_side = Some(XyceSwitchHysteresisSide::Off);
                self.departing_on = false;
                self.off_departure_bridge_count = 0;
                self.pending_departing_on = false;
                self.pending_off_departure_bridge_count = 0;
            }
        }
    }

    fn commit_pending_side(&mut self) {
        if let Some(side) = self.pending_side {
            self.transition_hold = Some(side);
        }
        self.departing_on = self.pending_departing_on;
        self.off_departure_bridge_count = self.pending_off_departure_bridge_count;
    }

    #[inline]
    fn safe_delta(delta: Value) -> Value {
        if delta.abs() >= 1.0e-12 {
            delta
        } else if delta.is_sign_negative() {
            -1.0e-12
        } else {
            1.0e-12
        }
    }

    fn interpolated_sensitivity(
        ron: Value,
        roff: Value,
        normalized_state: Value,
        dstate_dcontrol: Value,
    ) -> (Value, Value) {
        let state = normalized_state.clamp(0.0, 1.0);
        if state >= 1.0 {
            return (1.0 / ron, 0.0);
        }
        if state <= 0.0 {
            return (1.0 / roff, 0.0);
        }

        let lm = (ron * roff).sqrt().ln();
        let lr = (ron / roff).ln();
        let x = 2.0 * state - 1.0;
        let conductance = (-lm - 0.75 * lr * x + 0.25 * lr * x * x * x).exp();
        let dconductance_dstate = conductance * 1.5 * lr * (x * x - 1.0);
        (conductance, dconductance_dstate * dstate_dcontrol)
    }

    fn evaluate(&mut self, control: Value, ron: Value, roff: Value) -> XyceSwitchEvaluation {
        let dstate_dcontrol = 1.0 / Self::safe_delta(self.on - self.off);
        let base_state = (control - self.off) * dstate_dcontrol;

        if !self.hysteresis_enabled {
            self.last_state = base_state;
            self.transition_hold = None;
            let (conductance, dconductance_dcontrol) =
                Self::interpolated_sensitivity(ron, roff, base_state, dstate_dcontrol);
            return XyceSwitchEvaluation {
                conductance,
                dconductance_dcontrol,
                state: switch_state_for_normalized_control(base_state),
            };
        }

        let previous_state = self.last_state;
        let hys_on_state = (control - self.off) / Self::safe_delta(self.onh - self.off);
        let hys_off_state = (control - self.offh) / Self::safe_delta(self.on - self.offh);

        match self.transition_hold.unwrap_or(if previous_state >= 1.0 {
            XyceSwitchHysteresisSide::On
        } else {
            XyceSwitchHysteresisSide::Off
        }) {
            XyceSwitchHysteresisSide::Off => {
                if base_state >= 1.0 || hys_off_state >= 1.0 {
                    self.last_state = hys_off_state;
                    self.pending_side = Some(XyceSwitchHysteresisSide::On);
                    self.pending_departing_on = false;
                    self.pending_off_departure_bridge_count = 0;
                    return XyceSwitchEvaluation {
                        conductance: 1.0 / ron,
                        dconductance_dcontrol: 0.0,
                        state: SwitchState::On,
                    };
                }
                let short_bridge_complete =
                    !self.sticky_off_transition && self.off_departure_bridge_count >= 2;
                if base_state <= 0.0 || (!short_bridge_complete && hys_off_state <= 0.0) {
                    self.last_state = hys_off_state;
                    self.pending_side = Some(XyceSwitchHysteresisSide::Off);
                    self.pending_departing_on = false;
                    self.pending_off_departure_bridge_count = 0;
                    return XyceSwitchEvaluation {
                        conductance: 1.0 / roff,
                        dconductance_dcontrol: 0.0,
                        state: SwitchState::Off,
                    };
                }

                let use_hysteresis_bridge = self.sticky_off_transition || !short_bridge_complete;
                let interpolation_state = if use_hysteresis_bridge {
                    hys_off_state
                } else {
                    base_state
                };
                self.last_state = interpolation_state;
                self.pending_side = Some(XyceSwitchHysteresisSide::Off);
                self.pending_departing_on = false;
                self.pending_off_departure_bridge_count = if self.sticky_off_transition {
                    0
                } else {
                    self.off_departure_bridge_count.saturating_add(1).min(2)
                };
                let (conductance, dconductance_dcontrol) =
                    Self::interpolated_sensitivity(ron, roff, interpolation_state, dstate_dcontrol);
                XyceSwitchEvaluation {
                    conductance,
                    dconductance_dcontrol,
                    state: SwitchState::Transitioning,
                }
            }
            XyceSwitchHysteresisSide::On => {
                if base_state >= 1.0 || hys_on_state >= 1.0 {
                    self.last_state = hys_on_state;
                    self.pending_side = Some(XyceSwitchHysteresisSide::On);
                    self.pending_departing_on = false;
                    self.pending_off_departure_bridge_count = 0;
                    return XyceSwitchEvaluation {
                        conductance: 1.0 / ron,
                        dconductance_dcontrol: 0.0,
                        state: SwitchState::On,
                    };
                }
                if base_state <= 0.0 || hys_on_state <= 0.0 {
                    self.last_state = hys_on_state;
                    self.pending_side = Some(XyceSwitchHysteresisSide::Off);
                    self.pending_departing_on = false;
                    self.pending_off_departure_bridge_count = 0;
                    return XyceSwitchEvaluation {
                        conductance: 1.0 / roff,
                        dconductance_dcontrol: 0.0,
                        state: SwitchState::Off,
                    };
                }

                self.last_state = hys_on_state;
                if self.departing_on {
                    self.pending_side = Some(XyceSwitchHysteresisSide::Off);
                    self.pending_departing_on = false;
                    self.pending_off_departure_bridge_count = 2;
                } else {
                    self.pending_side = Some(XyceSwitchHysteresisSide::On);
                    self.pending_departing_on = true;
                    self.pending_off_departure_bridge_count = 0;
                }
                let (conductance, dconductance_dcontrol) =
                    Self::interpolated_sensitivity(ron, roff, hys_on_state, dstate_dcontrol);
                XyceSwitchEvaluation {
                    conductance,
                    dconductance_dcontrol,
                    state: SwitchState::Transitioning,
                }
            }
        }
    }
}

#[inline]
fn switch_state_for_normalized_control(state: Value) -> SwitchState {
    if state >= 1.0 {
        SwitchState::On
    } else if state <= 0.0 {
        SwitchState::Off
    } else {
        SwitchState::Transitioning
    }
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
    xyce_curve: Option<XyceSwitchCurve>,

    // State
    state: SwitchState,
    prev_state: SwitchState,
    in_hysteresis_band: bool,
    current_resistance: Value,
    prev_resistance: Value,
    current_dg_dcontrol: Value,
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
            xyce_curve: None,
            state: SwitchState::Off,
            prev_state: SwitchState::Off,
            in_hysteresis_band: false,
            current_resistance: 1e6,
            prev_resistance: 1e6,
            current_dg_dcontrol: 0.0,
        }
    }

    /// Set model parameters
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        if let Some(&v) = params.get("VT") {
            self.vt = v;
        }
        if let Some(&v) = params.get("VH") {
            self.vh = v;
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
        if params.contains_key("VON")
            || params.contains_key("VOFF")
            || params.contains_key("VHON")
            || params.contains_key("VHOFF")
            || params.contains_key("ON")
            || params.contains_key("OFF")
            || params.contains_key("ONH")
            || params.contains_key("OFFH")
        {
            let on = params
                .get("ON")
                .or_else(|| params.get("VON"))
                .copied()
                .unwrap_or(1.0);
            let off = params
                .get("OFF")
                .or_else(|| params.get("VOFF"))
                .copied()
                .unwrap_or(0.0);
            let onh = params
                .get("ONH")
                .or_else(|| params.get("VHON"))
                .copied()
                .unwrap_or(on);
            let offh = params
                .get("OFFH")
                .or_else(|| params.get("VHOFF"))
                .copied()
                .unwrap_or(off);
            let hysteresis_enabled = params.contains_key("ONH")
                || params.contains_key("OFFH")
                || params.contains_key("VHON")
                || params.contains_key("VHOFF");
            self.xyce_curve = Some(XyceSwitchCurve::new(
                on,
                off,
                onh,
                offh,
                hysteresis_enabled,
                false,
            ));
        }
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.current_dg_dcontrol = 0.0;
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
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
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.current_dg_dcontrol = 0.0;
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
        self
    }

    /// Set initial hysteresis state.
    pub fn with_initial_state(mut self, state: SwitchState) -> Self {
        self.state = state;
        self.in_hysteresis_band = false;
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        if let Some(curve) = &mut self.xyce_curve {
            curve.set_initial_state(state);
        }
        self.current_dg_dcontrol = 0.0;
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

    /// Commit hysteresis-side state after an accepted transient timestep.
    pub fn commit_transient_hysteresis(&mut self) {
        if let Some(curve) = &mut self.xyce_curve {
            curve.commit_pending_side();
        }
    }

    /// Calculate resistance based on control voltage using smooth transition
    fn calculate_resistance(&self, vctrl: Value) -> Value {
        let (g, _) = self.control_sensitivity(vctrl);
        1.0 / g.max(1e-30)
    }

    #[inline]
    fn effective_threshold(&self) -> Value {
        if self.vh < 0.0 {
            match self.state {
                SwitchState::Off => self.vt - self.vh,
                SwitchState::On => self.vt + self.vh,
                SwitchState::Transitioning => self.vt,
            }
        } else {
            match self.state {
                SwitchState::Off => self.vt + self.vh,
                SwitchState::On => self.vt - self.vh,
                SwitchState::Transitioning => self.vt,
            }
        }
    }

    /// Evaluate main-branch conductance and its control derivative.
    ///
    /// Returns `(g, dg/dvctrl)` for the current hysteresis state.
    fn control_sensitivity(&self, vctrl: Value) -> (Value, Value) {
        if self.xyce_curve.is_some() {
            return (
                1.0 / self.current_resistance.max(1.0e-30),
                self.current_dg_dcontrol,
            );
        }

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
        if self.vh < 0.0 {
            let lower = self.vt + self.vh;
            let upper = self.vt - self.vh;
            if vctrl > upper {
                self.state = SwitchState::On;
                self.in_hysteresis_band = false;
            } else if vctrl < lower {
                self.state = SwitchState::Off;
                self.in_hysteresis_band = false;
            } else if !self.in_hysteresis_band {
                self.state = match self.state {
                    SwitchState::Off => SwitchState::On,
                    SwitchState::On => SwitchState::Off,
                    SwitchState::Transitioning => SwitchState::Transitioning,
                };
                self.in_hysteresis_band = true;
            }
            return;
        }

        self.in_hysteresis_band = false;
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
        if let Some(curve) = &mut self.xyce_curve {
            let evaluation = curve.evaluate(vctrl, self.ron, self.roff);
            self.state = evaluation.state;
            self.in_hysteresis_band = false;
            self.current_resistance = 1.0 / evaluation.conductance.max(1.0e-30);
            self.current_dg_dcontrol = evaluation.dconductance_dcontrol;
        } else {
            self.update_state(vctrl);
            self.current_resistance = self.calculate_resistance(vctrl);
            self.current_dg_dcontrol = 0.0;
        }
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
    /// Xyce-style full-on current control value from ION/ON.
    pub on_current: Option<Value>,
    /// Xyce-style full-off current control value from IOFF/OFF.
    pub off_current: Option<Value>,
    xyce_curve: Option<XyceSwitchCurve>,

    // State
    state: SwitchState,
    prev_state: SwitchState,
    in_hysteresis_band: bool,
    current_resistance: Value,
    prev_resistance: Value,
    current_dg_dictrl: Value,
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
            on_current: None,
            off_current: None,
            xyce_curve: None,
            state: SwitchState::Off,
            prev_state: SwitchState::Off,
            in_hysteresis_band: false,
            current_resistance: 1e6,
            prev_resistance: 1e6,
            current_dg_dictrl: 0.0,
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
            self.ih = v;
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
        if params.contains_key("ION")
            || params.contains_key("IOFF")
            || params.contains_key("IHON")
            || params.contains_key("IHOFF")
            || params.contains_key("ON")
            || params.contains_key("OFF")
            || params.contains_key("ONH")
            || params.contains_key("OFFH")
        {
            let on = params
                .get("ON")
                .or_else(|| params.get("ION"))
                .copied()
                .unwrap_or(1.0e-3);
            let off = params
                .get("OFF")
                .or_else(|| params.get("IOFF"))
                .copied()
                .unwrap_or(0.0);
            let onh = params
                .get("ONH")
                .or_else(|| params.get("IHON"))
                .copied()
                .unwrap_or(on);
            let offh = params
                .get("OFFH")
                .or_else(|| params.get("IHOFF"))
                .copied()
                .unwrap_or(off);
            let hysteresis_enabled = params.contains_key("ONH")
                || params.contains_key("OFFH")
                || params.contains_key("IHON")
                || params.contains_key("IHOFF");
            self.on_current = Some(on);
            self.off_current = Some(off);
            self.xyce_curve = Some(XyceSwitchCurve::new(
                on,
                off,
                onh,
                offh,
                hysteresis_enabled,
                true,
            ));
        }
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.current_dg_dictrl = 0.0;
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
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
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        self.current_dg_dictrl = 0.0;
        self.prev_resistance = self.current_resistance;
        self.prev_state = self.state;
        self
    }

    /// Set initial hysteresis state.
    pub fn with_initial_state(mut self, state: SwitchState) -> Self {
        self.state = state;
        self.in_hysteresis_band = false;
        self.current_resistance = match self.state {
            SwitchState::On => self.ron,
            SwitchState::Off => self.roff,
            SwitchState::Transitioning => (self.ron * self.roff).sqrt(),
        };
        if let Some(curve) = &mut self.xyce_curve {
            curve.set_initial_state(state);
        }
        self.current_dg_dictrl = 0.0;
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

    /// Commit hysteresis-side state after an accepted transient timestep.
    pub fn commit_transient_hysteresis(&mut self) {
        if let Some(curve) = &mut self.xyce_curve {
            curve.commit_pending_side();
        }
    }

    /// Calculate resistance based on control current
    fn calculate_resistance(&self, ictrl: Value) -> Value {
        let (g, _) = self.control_sensitivity(ictrl);
        1.0 / g.max(1e-30)
    }

    #[inline]
    fn effective_threshold(&self) -> Value {
        if self.ih < 0.0 {
            match self.state {
                SwitchState::Off => self.it - self.ih,
                SwitchState::On => self.it + self.ih,
                SwitchState::Transitioning => self.it,
            }
        } else {
            match self.state {
                SwitchState::Off => self.it + self.ih,
                SwitchState::On => self.it - self.ih,
                SwitchState::Transitioning => self.it,
            }
        }
    }

    /// Evaluate main-branch conductance and its control derivative.
    ///
    /// Returns `(g, dg/dictrl)` for the current hysteresis state.
    fn control_sensitivity(&self, ictrl: Value) -> (Value, Value) {
        if self.xyce_curve.is_some() {
            return (
                1.0 / self.current_resistance.max(1.0e-30),
                self.current_dg_dictrl,
            );
        }

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
        if self.ih < 0.0 {
            let lower = self.it + self.ih;
            let upper = self.it - self.ih;
            if ictrl > upper {
                self.state = SwitchState::On;
                self.in_hysteresis_band = false;
            } else if ictrl < lower {
                self.state = SwitchState::Off;
                self.in_hysteresis_band = false;
            } else if !self.in_hysteresis_band {
                self.state = match self.state {
                    SwitchState::Off => SwitchState::On,
                    SwitchState::On => SwitchState::Off,
                    SwitchState::Transitioning => SwitchState::Transitioning,
                };
                self.in_hysteresis_band = true;
            }
            return;
        }

        self.in_hysteresis_band = false;
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
        if let Some(curve) = &mut self.xyce_curve {
            let evaluation = curve.evaluate(ictrl, self.ron, self.roff);
            self.state = evaluation.state;
            self.in_hysteresis_band = false;
            self.current_resistance = 1.0 / evaluation.conductance.max(1.0e-30);
            self.current_dg_dictrl = evaluation.dconductance_dcontrol;
        } else {
            self.update_state(ictrl);
            self.current_resistance = self.calculate_resistance(ictrl);
            self.current_dg_dictrl = 0.0;
        }
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
// Xyce Generic Expression-Controlled Switch
//=============================================================================

/// Xyce generic two-terminal switch:
/// `SW1 p n MODEL [ON|OFF] CONTROL={expr}`.
///
/// Native evaluation supports time and fixed analysis-context scalars. Control
/// expressions that reference circuit unknowns require Jacobian coupling and are
/// rejected by the builder until that path is implemented.
#[derive(Debug, Clone)]
pub struct GenericSwitch {
    /// Instance name
    pub name: String,
    /// Positive terminal
    pub node_pos: NodeId,
    /// Negative terminal
    pub node_neg: NodeId,

    /// Compiled scalar control expression
    pub program: CompiledExpr,
    vm: Vm,
    time_breakpoints: Vec<Value>,
    temperature: Value,
    gmin: Value,
    expression_dialect: crate::netlist::ExpressionDialect,

    /// On resistance
    pub ron: Value,
    /// Off resistance
    pub roff: Value,
    /// Control value for fully on
    pub on: Value,
    /// Control value for fully off
    pub off: Value,
    /// Rising hysteresis on threshold
    pub onh: Value,
    /// Falling hysteresis off threshold
    pub offh: Value,
    /// Whether ONH/OFFH semantics are active
    pub hysteresis_enabled: bool,

    /// Xyce `lastStoVector` normalized control state. This is one accepted
    /// timepoint older than `current_state` and is the history level consulted
    /// by the generic-switch hysteresis law.
    last_state: Value,
    /// Xyce `currStoVector` normalized control state from the most recently
    /// accepted timepoint.
    current_state: Value,
    /// Xyce `nextStoVector` candidate produced by the current trial evaluation.
    /// Keeping all three history levels distinct prevents repeated Newton
    /// stamps and rejected steps from consuming hysteresis history.
    trial_state: Value,
    current_conductance: Value,
}

impl GenericSwitch {
    /// Create a generic switch with Xyce defaults.
    pub fn new(
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        control_expression: &str,
    ) -> Result<Self, String> {
        let ast = parse_expression_strict(control_expression).map_err(|err| {
            format!(
                "Generic switch '{}' has invalid CONTROL expression '{}': {}",
                name, control_expression, err
            )
        })?;
        let time_breakpoints = Self::collect_time_breakpoints(&ast);
        let program = compile(&ast);
        Ok(Self {
            name,
            node_pos,
            node_neg,
            program,
            vm: Vm::new(),
            time_breakpoints,
            temperature: crate::analysis::temperature::kelvin_to_celsius(
                crate::constants::TEMP_REFERENCE,
            ),
            gmin: crate::constants::GMIN,
            expression_dialect: crate::netlist::ExpressionDialect::Ngspice,
            ron: 1.0,
            roff: 1.0e6,
            on: 1.0,
            off: 0.0,
            onh: 1.0,
            offh: 0.0,
            hysteresis_enabled: false,
            last_state: 0.0,
            current_state: 0.0,
            trial_state: 0.0,
            current_conductance: 1.0e-6,
        })
    }

    /// Return true when the expression needs solution-vector Jacobian support.
    pub fn has_solution_references(&self) -> bool {
        !self.program.node_map.is_empty() || !self.program.branch_map.is_empty()
    }

    /// Time instants where the control expression can change discontinuously.
    pub fn time_breakpoints(&self) -> &[Value] {
        &self.time_breakpoints
    }

    /// Time instants where a time-only control expression crosses switch
    /// thresholds.
    pub fn threshold_breakpoints(&self, tstop: Value, scan_step: Value) -> Vec<Value> {
        if !(tstop.is_finite() && tstop > 0.0) {
            return Vec::new();
        }

        let mut thresholds = vec![self.off, self.on];
        if self.hysteresis_enabled {
            thresholds.push(self.offh);
            thresholds.push(self.onh);
        }
        thresholds.retain(|value| value.is_finite());
        thresholds.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        thresholds.dedup_by(|a, b| {
            let scale = a.abs().max(b.abs()).max(1.0);
            (*a - *b).abs() <= scale * 1.0e-12
        });
        if thresholds.is_empty() {
            return Vec::new();
        }

        let mut step = if scan_step.is_finite() && scan_step > 0.0 {
            scan_step
        } else {
            tstop / 1000.0
        };
        step = step.clamp(tstop / 1.0e6, tstop.max(f64::MIN_POSITIVE));
        let time_tolerance = (step.abs() * 1.0e-9)
            .max(tstop.abs() * 1.0e-14)
            .max(1.0e-18);

        let mut vm = Vm::new();
        let mut breakpoints = Vec::new();
        let mut t0 = 0.0;
        let mut y0 = self.evaluate_control_at(t0, &mut vm);

        while t0 < tstop {
            let t1 = (t0 + step).min(tstop);
            let y1 = self.evaluate_control_at(t1, &mut vm);
            for &threshold in &thresholds {
                self.push_threshold_crossing(
                    threshold,
                    t0,
                    y0,
                    t1,
                    y1,
                    time_tolerance,
                    &mut breakpoints,
                );
            }
            t0 = t1;
            y0 = y1;
        }

        breakpoints.retain(|time| time.is_finite() && *time >= 0.0 && *time <= tstop);
        breakpoints.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        breakpoints.dedup_by(|a, b| {
            let scale = a.abs().max(b.abs()).max(1.0);
            (*a - *b).abs() <= scale * 1.0e-12
        });
        breakpoints
    }

    fn push_threshold_crossing(
        &self,
        threshold: Value,
        t0: Value,
        y0: Value,
        t1: Value,
        y1: Value,
        time_tolerance: Value,
        breakpoints: &mut Vec<Value>,
    ) {
        let f0 = y0 - threshold;
        let f1 = y1 - threshold;
        if !f0.is_finite() || !f1.is_finite() {
            return;
        }
        let value_tolerance = threshold.abs().max(y0.abs()).max(y1.abs()).max(1.0) * 1.0e-12;
        if f0.abs() <= value_tolerance {
            breakpoints.push(t0);
            return;
        }
        if f1.abs() <= value_tolerance {
            breakpoints.push(t1);
            return;
        }
        if f0.signum() == f1.signum() {
            return;
        }

        let mut left = t0;
        let mut right = t1;
        let mut left_value = f0;
        let mut vm = Vm::new();
        for _ in 0..64 {
            let mid = 0.5 * (left + right);
            if (right - left).abs() <= time_tolerance {
                break;
            }
            let mid_value = self.evaluate_control_at(mid, &mut vm) - threshold;
            if !mid_value.is_finite() {
                return;
            }
            if mid_value.abs() <= value_tolerance {
                left = mid;
                right = mid;
                break;
            }
            if left_value.signum() == mid_value.signum() {
                left = mid;
                left_value = mid_value;
            } else {
                right = mid;
            }
        }
        breakpoints.push(0.5 * (left + right));
    }

    fn collect_time_breakpoints(expr: &Expr) -> Vec<Value> {
        let mut breakpoints = Vec::new();
        Self::collect_time_breakpoints_from_expr(expr, &mut breakpoints);
        breakpoints.retain(|time| time.is_finite() && *time >= 0.0);
        breakpoints.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        breakpoints.dedup_by(|a, b| {
            let scale = a.abs().max(b.abs()).max(1.0);
            (*a - *b).abs() <= scale * 1.0e-12
        });
        breakpoints
    }

    fn collect_time_breakpoints_from_expr(expr: &Expr, breakpoints: &mut Vec<Value>) {
        match expr {
            Expr::Binary { op, left, right } => {
                if matches!(
                    op,
                    BinaryOp::Lt
                        | BinaryOp::Le
                        | BinaryOp::Gt
                        | BinaryOp::Ge
                        | BinaryOp::Eq
                        | BinaryOp::Ne
                ) {
                    Self::push_affine_crossing(left, right, breakpoints);
                }
                Self::collect_time_breakpoints_from_expr(left, breakpoints);
                Self::collect_time_breakpoints_from_expr(right, breakpoints);
            }
            Expr::Unary { operand, .. } => {
                Self::collect_time_breakpoints_from_expr(operand, breakpoints);
            }
            Expr::Function { func, args } => {
                if matches!(
                    func,
                    Function::Stp
                        | Function::Gt0
                        | Function::Lt0
                        | Function::Ge0
                        | Function::Le0
                        | Function::Eq0
                        | Function::Ne0
                ) {
                    if let Some(arg) = args.first() {
                        Self::push_affine_zero_crossing(arg, breakpoints);
                    }
                }
                if matches!(func, Function::Table | Function::Pwl) {
                    Self::collect_table_time_breakpoints(args, breakpoints);
                }
                for arg in args {
                    Self::collect_time_breakpoints_from_expr(arg, breakpoints);
                }
            }
            Expr::Const(_)
            | Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::StringLiteral(_)
            | Expr::Time
            | Expr::Frequency
            | Expr::Temperature
            | Expr::ThermalVoltage
            | Expr::Gmin => {}
            Expr::LookupTable(table) => {
                if table.transient_breakpoints {
                    breakpoints.extend(table.points.iter().map(|(time, _)| *time));
                }
            }
        }
    }

    fn collect_table_time_breakpoints(args: &[Expr], breakpoints: &mut Vec<Value>) {
        let Some((time_scale, time_offset)) = args.first().and_then(Self::affine_time) else {
            return;
        };
        if time_scale.abs() < 1.0e-30 {
            return;
        }
        for pair in args[1..].chunks(2) {
            let Some(Expr::Const(knot)) = pair.first() else {
                continue;
            };
            breakpoints.push((*knot - time_offset) / time_scale);
        }
    }

    fn push_affine_crossing(left: &Expr, right: &Expr, breakpoints: &mut Vec<Value>) {
        let (Some((left_a, left_b)), Some((right_a, right_b))) =
            (Self::affine_time(left), Self::affine_time(right))
        else {
            return;
        };
        let a = left_a - right_a;
        let b = left_b - right_b;
        if a.abs() >= 1.0e-30 {
            breakpoints.push(-b / a);
        }
    }

    fn push_affine_zero_crossing(expr: &Expr, breakpoints: &mut Vec<Value>) {
        let Some((a, b)) = Self::affine_time(expr) else {
            return;
        };
        if a.abs() >= 1.0e-30 {
            breakpoints.push(-b / a);
        }
    }

    fn affine_time(expr: &Expr) -> Option<(Value, Value)> {
        match expr {
            Expr::Const(value) => Some((0.0, *value)),
            Expr::Time => Some((1.0, 0.0)),
            Expr::Unary {
                op: UnaryOp::Neg,
                operand,
            } => {
                let (a, b) = Self::affine_time(operand)?;
                Some((-a, -b))
            }
            Expr::Binary { op, left, right } => {
                let (left_a, left_b) = Self::affine_time(left)?;
                let (right_a, right_b) = Self::affine_time(right)?;
                match op {
                    BinaryOp::Add => Some((left_a + right_a, left_b + right_b)),
                    BinaryOp::Sub => Some((left_a - right_a, left_b - right_b)),
                    BinaryOp::Mul if left_a == 0.0 => Some((right_a * left_b, right_b * left_b)),
                    BinaryOp::Mul if right_a == 0.0 => Some((left_a * right_b, left_b * right_b)),
                    BinaryOp::Div if right_a == 0.0 && right_b.abs() >= 1.0e-30 => {
                        Some((left_a / right_b, left_b / right_b))
                    }
                    _ => None,
                }
            }
            Expr::Unary { .. }
            | Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::StringLiteral(_)
            | Expr::LookupTable(_)
            | Expr::Frequency
            | Expr::Temperature
            | Expr::ThermalVoltage
            | Expr::Gmin
            | Expr::Function { .. } => None,
        }
    }

    /// Set model parameters.
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        self.hysteresis_enabled = params.contains_key("ONH") || params.contains_key("OFFH");
        if let Some(&v) = params.get("RON") {
            self.ron = v.max(1.0e-12);
        }
        if let Some(&v) = params.get("ROFF") {
            self.roff = v.max(1.0e-12);
        }
        if let Some(&v) = params.get("ON") {
            self.on = v;
        }
        if let Some(&v) = params.get("OFF") {
            self.off = v;
        }
        self.onh = params.get("ONH").copied().unwrap_or(self.on);
        self.offh = params.get("OFFH").copied().unwrap_or(self.off);
        self.current_conductance = 1.0 / self.roff;
        self
    }

    /// Set initial ON/OFF state.
    pub fn with_initial_state(mut self, state: SwitchState) -> Self {
        match state {
            SwitchState::On => {
                self.last_state = 1.0;
                self.current_state = 1.0;
                self.trial_state = 1.0;
                self.current_conductance = 1.0 / self.ron;
            }
            SwitchState::Off | SwitchState::Transitioning => {
                self.last_state = 0.0;
                self.current_state = 0.0;
                self.trial_state = 0.0;
                self.current_conductance = 1.0 / self.roff;
            }
        }
        self
    }

    /// Set the fixed analysis context used by the retained CONTROL expression.
    ///
    /// `gmin` is the resolved device-option floor. It intentionally does not
    /// follow nonlinear continuation, whose temporary junction conductance is
    /// a solver aid rather than Xyce's expression-visible `GMIN` option.
    pub fn set_expression_context(
        &mut self,
        temperature: Value,
        gmin: Value,
        expression_dialect: crate::netlist::ExpressionDialect,
    ) {
        self.temperature = temperature;
        self.gmin = gmin;
        self.expression_dialect = expression_dialect;
    }

    /// Current small-signal conductance.
    pub fn conductance(&self) -> Value {
        self.current_conductance
    }

    fn evaluate_control_at(&self, time: Value, vm: &mut Vm) -> Value {
        let ctx = Context::transient(&[], &[], time)
            .with_temperature(self.temperature)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect);
        let value = vm.execute(&self.program, &ctx);
        if value.is_finite() { value } else { self.off }
    }

    fn evaluate_control(&mut self, time: Value) -> Value {
        let ctx = Context::transient(&[], &[], time)
            .with_temperature(self.temperature)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect);
        let value = self.vm.execute(&self.program, &ctx);
        if value.is_finite() { value } else { self.off }
    }

    #[inline]
    fn safe_delta(delta: Value) -> Value {
        if delta.abs() >= 1.0e-12 {
            delta
        } else if delta.is_sign_negative() {
            -1.0e-12
        } else {
            1.0e-12
        }
    }

    fn interpolated_conductance(&self, normalized_state: Value) -> Value {
        let state = normalized_state.clamp(0.0, 1.0);
        if state >= 1.0 {
            return 1.0 / self.ron;
        }
        if state <= 0.0 {
            return 1.0 / self.roff;
        }

        let lm = (self.ron * self.roff).sqrt().ln();
        let lr = (self.ron / self.roff).ln();
        let x = 2.0 * state - 1.0;
        (-lm - 0.75 * lr * x + 0.25 * lr * x * x * x).exp()
    }

    fn conductance_for_control(&mut self, control: Value) -> Value {
        let d_inv = 1.0 / Self::safe_delta(self.on - self.off);
        let base_state = (control - self.off) * d_inv;

        if !self.hysteresis_enabled {
            self.trial_state = base_state;
            return self.interpolated_conductance(base_state);
        }

        let previous_state = self.last_state;
        let hys_on_state = (control - self.off) / Self::safe_delta(self.onh - self.off);
        let hys_off_state = (control - self.offh) / Self::safe_delta(self.on - self.offh);

        if base_state >= 1.0 || (previous_state >= 1.0 && hys_on_state >= 1.0) {
            // Xyce writes the hysteretic normalization unconditionally in the
            // ON branch, including when the unhysterized control crossed 1.
            self.trial_state = hys_on_state;
            return 1.0 / self.ron;
        }

        if base_state <= 0.0 || (previous_state <= 0.0 && hys_off_state <= 0.0) {
            // Xyce likewise writes the hysteretic normalization
            // unconditionally in the OFF branch.
            self.trial_state = hys_off_state;
            return 1.0 / self.roff;
        }

        let interpolation_state = if previous_state <= 0.0 {
            hys_off_state
        } else if previous_state >= 1.0 {
            hys_on_state
        } else {
            base_state
        };
        // Xyce writes the unhysterized normalized control to its next store
        // vector in the interpolation branch, even when the conductance uses
        // a hysteresis-adjusted state for this accepted point.
        self.trial_state = base_state;
        self.interpolated_conductance(interpolation_state)
    }

    /// Advance Xyce's three-level store-vector history after an accepted
    /// transient timepoint. Rejected steps and repeated Newton stamps leave
    /// both accepted history levels untouched.
    pub(crate) fn accept_transient_step(&mut self) {
        self.last_state = self.current_state;
        self.current_state = self.trial_state;
    }

    /// Seed both accepted store-vector history levels from the operating-point
    /// trial, matching Xyce's `DataStore::setConstantHistory` handoff into
    /// transient analysis.
    pub(crate) fn initialize_transient_history(&mut self) {
        self.last_state = self.trial_state;
        self.current_state = self.trial_state;
    }

    /// Return the complete generic-switch transient store state. The first
    /// three entries correspond to Xyce's last/current/next store vectors; the
    /// fourth preserves the accepted-point conductance used by derived-current
    /// traces at a checkpoint seam.
    pub(crate) fn transient_store_snapshot(&self) -> [Value; 4] {
        [
            self.last_state,
            self.current_state,
            self.trial_state,
            self.current_conductance,
        ]
    }

    /// Restore a previously validated transient store snapshot.
    pub(crate) fn restore_transient_store_snapshot(&mut self, snapshot: [Value; 4]) {
        self.last_state = snapshot[0];
        self.current_state = snapshot[1];
        self.trial_state = snapshot[2];
        self.current_conductance = snapshot[3];
    }

    /// Stamp the switch conductance for a given analysis time.
    pub fn stamp_time_dependent(&mut self, time: Value, matrix: &mut impl MatrixStamper) {
        let control = self.evaluate_control(time);
        let g = self.conductance_for_control(control);
        self.current_conductance = g;
        self.stamp_conductance(g, matrix);
    }

    /// Stamp the current frozen conductance, used by small-signal analyses
    /// after the operating point has evaluated the switch at t=0.
    pub fn stamp_current_conductance(&self, matrix: &mut impl MatrixStamper) {
        self.stamp_conductance(self.current_conductance, matrix);
    }

    fn stamp_conductance(&self, g: Value, matrix: &mut impl MatrixStamper) {
        matrix.stamp(self.node_pos, self.node_pos, g);
        matrix.stamp(self.node_pos, self.node_neg, -g);
        matrix.stamp(self.node_neg, self.node_pos, -g);
        matrix.stamp(self.node_neg, self.node_neg, g);
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use crate::device::traits::NonlinearDevice;
    use crate::netlist::ExpressionDialect;

    use super::{CurrentSwitch, GenericSwitch, SwitchState, VoltageSwitch};

    #[test]
    fn generic_switch_extracts_time_control_breakpoints() {
        let switch = GenericSwitch::new("sw1".to_string(), 1, 0, "if(time>2u,1,stp(time-3u))")
            .expect("valid generic switch expression");

        assert_eq!(switch.time_breakpoints().len(), 2);
        assert!((switch.time_breakpoints()[0] - 2.0e-6).abs() < 1.0e-18);
        assert!((switch.time_breakpoints()[1] - 3.0e-6).abs() < 1.0e-18);
    }

    #[test]
    fn generic_switch_evaluates_runtime_specials_in_resolved_context() {
        let mut switch = GenericSwitch::new("sw1".to_string(), 1, 0, "TEMP + VT + GMIN")
            .expect("runtime-special CONTROL expression parses");
        switch.set_expression_context(80.0, 2.5e-8, ExpressionDialect::Xyce);

        let expected =
            80.0 + crate::constants::thermal_voltage(
                crate::analysis::temperature::celsius_to_kelvin(80.0),
            ) + 2.5e-8;
        let actual = switch.evaluate_control(0.0);
        assert!(
            (actual - expected).abs() < 1.0e-14,
            "{actual} != {expected}"
        );
    }

    #[test]
    fn generic_switch_hysteresis_keeps_xyce_branch_during_partial_transition() {
        let params = std::collections::HashMap::from([
            ("ON".to_string(), 1.0),
            ("ONH".to_string(), 0.55),
            ("OFF".to_string(), 0.0),
            ("OFFH".to_string(), 0.25),
            ("RON".to_string(), 1.0),
            ("ROFF".to_string(), 100.0),
        ]);
        let mut switch = GenericSwitch::new("sw1".to_string(), 1, 0, "1")
            .expect("valid generic switch expression")
            .with_params(&params)
            .with_initial_state(SwitchState::Off);

        let rising_g = switch.conductance_for_control(0.269_311_698);
        assert!((rising_g - 0.010_090_431_945).abs() < 1.0e-12);
        assert!((switch.conductance_for_control(0.269_311_698) - rising_g).abs() < 1.0e-15);
        switch.accept_transient_step();

        // Xyce's hysteresis decision reads lastStoVector, not currStoVector,
        // so the held-off curve remains active for one more accepted point.
        let second_rising_g = switch.conductance_for_control(0.274_889_451);
        assert!((second_rising_g - 0.010_149_897_256).abs() < 1.0e-12);
        switch.accept_transient_step();

        assert!((switch.conductance_for_control(1.0) - 1.0).abs() < 1.0e-15);
        switch.accept_transient_step();
        assert!((switch.conductance_for_control(1.0) - 1.0).abs() < 1.0e-15);
        switch.accept_transient_step();
        assert!((switch.conductance_for_control(0.562_116_094) - 1.0).abs() < 1.0e-15);
        switch.accept_transient_step();

        let falling_g = switch.conductance_for_control(0.381_041_069);
        assert!((falling_g - 0.354_599_436_328).abs() < 1.0e-12);

        let held_falling_g = switch.conductance_for_control(0.381_041_069);
        assert!((held_falling_g - 0.354_599_436_328).abs() < 1.0e-12);
        switch.accept_transient_step();

        let second_held_falling_g = switch.conductance_for_control(0.381_041_069);
        assert!((second_held_falling_g - 0.354_599_436_328).abs() < 1.0e-12);
        switch.accept_transient_step();

        let base_falling_g = switch.conductance_for_control(0.381_041_069);
        assert!((base_falling_g - 0.044_653_639_971).abs() < 1.0e-12);
    }

    #[test]
    fn voltage_switch_negative_vh_uses_ngspice_inverted_hysteresis_band() {
        let mut switch = VoltageSwitch::new("s1".to_string(), 1, 0, 2, 0)
            .with_thresholds(0.0, -0.5)
            .with_resistances(1.0, 1.0e6)
            .with_initial_state(SwitchState::Off);

        switch.update(&[0.0, -1.0]);
        assert_eq!(switch.state(), SwitchState::Off);

        switch.update(&[0.0, 0.0]);

        assert_eq!(switch.state(), SwitchState::On);
        assert!(
            switch.resistance() < 10.0,
            "negative VH should enter the on side of the inverted hysteresis band; resistance={}",
            switch.resistance()
        );

        switch.update(&[0.0, 0.0]);
        assert_eq!(switch.state(), SwitchState::On);
    }

    #[test]
    fn current_switch_negative_ih_uses_ngspice_inverted_hysteresis_band() {
        let mut switch = CurrentSwitch::new("w1".to_string(), 1, 0, "vctrl".to_string())
            .with_thresholds(0.0, -0.5)
            .with_resistances(1.0, 1.0e6)
            .with_initial_state(SwitchState::Off);
        switch.set_ctrl_branch(2);

        switch.update(&[0.0, -1.0]);
        assert_eq!(switch.state(), SwitchState::Off);

        switch.update(&[0.0, 0.0]);

        assert_eq!(switch.state(), SwitchState::On);
        assert!(
            switch.resistance() < 10.0,
            "negative IH should enter the on side of the inverted hysteresis band; resistance={}",
            switch.resistance()
        );

        switch.update(&[0.0, 0.0]);
        assert_eq!(switch.state(), SwitchState::On);
    }
}
