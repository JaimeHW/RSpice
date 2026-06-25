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
    in_hysteresis_band: bool,
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
            in_hysteresis_band: false,
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
    in_hysteresis_band: bool,
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
            in_hysteresis_band: false,
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
// Xyce Generic Expression-Controlled Switch
//=============================================================================

/// Xyce generic two-terminal switch:
/// `SW1 p n MODEL [ON|OFF] CONTROL={expr}`.
///
/// This first native slice supports time/constant control expressions. Control
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

    last_state: Value,
    hysteresis_rising: bool,
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
            ron: 1.0,
            roff: 1.0e6,
            on: 1.0,
            off: 0.0,
            onh: 1.0,
            offh: 0.0,
            hysteresis_enabled: false,
            last_state: 0.0,
            hysteresis_rising: true,
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
            | Expr::Time
            | Expr::Frequency
            | Expr::Temperature => {}
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
            | Expr::Frequency
            | Expr::Temperature
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
                self.hysteresis_rising = false;
                self.current_conductance = 1.0 / self.ron;
            }
            SwitchState::Off | SwitchState::Transitioning => {
                self.last_state = 0.0;
                self.hysteresis_rising = true;
                self.current_conductance = 1.0 / self.roff;
            }
        }
        self
    }

    /// Current small-signal conductance.
    pub fn conductance(&self) -> Value {
        self.current_conductance
    }

    fn evaluate_control(&mut self, time: Value) -> Value {
        let ctx = Context::transient(&[], &[], time);
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
            self.last_state = base_state;
            return self.interpolated_conductance(base_state);
        }

        if self.hysteresis_rising {
            let state = (control - self.offh) / Self::safe_delta(self.on - self.offh);
            if state <= 0.0 {
                self.last_state = 0.0;
                return 1.0 / self.roff;
            }
            if state >= 1.0 || base_state >= 1.0 {
                self.last_state = 1.0;
                self.hysteresis_rising = false;
                return 1.0 / self.ron;
            }
            self.last_state = state;
            return self.interpolated_conductance(state);
        }

        let latch_state = (control - self.off) / Self::safe_delta(self.onh - self.off);
        if latch_state >= 1.0 {
            self.last_state = 1.0;
            return 1.0 / self.ron;
        }
        if latch_state <= 0.0 || base_state <= 0.0 {
            self.last_state = 0.0;
            self.hysteresis_rising = true;
            return 1.0 / self.roff;
        }
        self.last_state = base_state;
        self.interpolated_conductance(base_state)
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

        assert!((switch.conductance_for_control(1.0) - 1.0).abs() < 1.0e-15);
        assert!((switch.conductance_for_control(0.562_116_094) - 1.0).abs() < 1.0e-15);

        let falling_g = switch.conductance_for_control(0.381_041_069);
        assert!((falling_g - 0.044_653_639_971).abs() < 1.0e-12);
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
