//! Abstract Syntax Tree for netlist representation
//!
//! Provides a complete representation of SPICE netlist elements including:
//! - All standard circuit components (R, L, C, sources)
//! - Semiconductor devices (diodes, BJTs, MOSFETs)
//! - Controlled sources (VCVS, VCCS, CCVS, CCCS)
//! - Advanced components (switches, transmission lines, coupled inductors)
//! - Analysis commands (DC, AC, transient, parametric, noise)

use crate::Value;

//=============================================================================
// Parametric Values
//=============================================================================

/// A value that can be either resolved or a parameter expression.
///
/// Used for subcircuit instance parameters where the value may reference
/// parameters from the instance or parent scope.
///
/// # Examples
/// ```ignore
/// // Resolved numeric value
/// ParametricValue::Resolved(1000.0) // 1k ohms
///
/// // Expression to be evaluated later
/// ParametricValue::Expression("2*W".to_string()) // 2 * W parameter
/// ```
#[derive(Debug, Clone)]
pub enum ParametricValue {
    /// A resolved numeric value  
    Resolved(Value),
    /// An expression string to be evaluated with a parameter context
    Expression(String),
}

impl ParametricValue {
    /// Create from a resolved value
    pub fn from_value(v: Value) -> Self {
        ParametricValue::Resolved(v)
    }

    /// Create from an expression string
    pub fn from_expr(s: &str) -> Self {
        ParametricValue::Expression(s.to_string())
    }

    /// Try to get the resolved value
    pub fn as_value(&self) -> Option<Value> {
        match self {
            ParametricValue::Resolved(v) => Some(*v),
            ParametricValue::Expression(_) => None,
        }
    }

    /// Check if this is a resolved value
    pub fn is_resolved(&self) -> bool {
        matches!(self, ParametricValue::Resolved(_))
    }

    /// Resolve this value using a parameter lookup function.
    /// Returns the resolved value or an error message.
    pub fn resolve<F>(&self, lookup: F) -> Result<Value, String>
    where
        F: Fn(&str) -> Option<Value>,
    {
        match self {
            ParametricValue::Resolved(v) => Ok(*v),
            ParametricValue::Expression(expr) => {
                // Simple parameter lookup (single identifier)
                if let Some(v) = lookup(expr) {
                    return Ok(v);
                }
                // Try parsing as a number
                if let Ok(v) = expr.parse::<f64>() {
                    return Ok(v);
                }
                Err(format!("Unable to resolve expression: {}", expr))
            }
        }
    }
}

impl Default for ParametricValue {
    fn default() -> Self {
        ParametricValue::Resolved(0.0)
    }
}

impl From<Value> for ParametricValue {
    fn from(v: Value) -> Self {
        ParametricValue::Resolved(v)
    }
}

//=============================================================================
// Circuit Elements
//=============================================================================

/// A circuit element (component instance)
#[derive(Debug, Clone)]
pub struct Element {
    /// Element name (e.g., "R1", "C1", "M1")
    pub name: String,
    /// Element type and parameters
    pub kind: ElementKind,
    /// Connected nodes
    pub nodes: Vec<String>,
}

/// Types of circuit elements
#[derive(Debug, Clone)]
pub enum ElementKind {
    //-------------------------------------------------------------------------
    // Passive Components
    //-------------------------------------------------------------------------
    /// Resistor.
    ///
    /// Supports both numeric-value form (`R1 n+ n- 1k`) and model-based form
    /// (`R1 n+ n- RMOD L=10u W=2u`).
    Resistor {
        /// Explicit resistance value in Ohms when provided directly.
        value: Value,
        /// Optional deferred expression for parameterized resistor values.
        value_expr: Option<String>,
        /// Optional model name for model-based resistor instances.
        model: Option<String>,
        /// Optional instance parameters (e.g., `L`, `W`, `M`, `R`).
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured as expressions inside subcircuit
        /// bodies; resolved against the instance scope during flattening
        /// and merged over `instance_params`.
        deferred_params: Vec<(String, String)>,
    },

    /// Capacitor.
    ///
    /// Supports both numeric-value form (`C1 n+ n- 1u`) and model-based form
    /// (`C1 n+ n- CMOD W=10u L=20u`).
    Capacitor {
        /// Explicit capacitance value in Farads when provided directly.
        /// `NAN` when the value must be resolved from a model card.
        value: Value,
        /// Optional deferred expression for parameterized capacitance values.
        value_expr: Option<String>,
        initial_voltage: Option<Value>,
        /// Optional model name for model-based capacitor instances.
        model: Option<String>,
        /// Optional instance parameters (e.g. `W`, `L`, `M`, `SCALE`, `TC1`).
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured as expressions inside subcircuit
        /// bodies; resolved against the instance scope during flattening
        /// and merged over `instance_params`.
        deferred_params: Vec<(String, String)>,
    },

    /// Inductor.
    ///
    /// Supports both numeric-value form (`L1 n+ n- 1m`) and model-based form
    /// (`L1 n+ n- LMOD`); magnetic-core (Jiles-Atherton) models are detected
    /// from the referenced model card's type at circuit-build time.
    Inductor {
        /// Explicit inductance value in Henries when provided directly.
        /// `NAN` when the value must be resolved from a model card.
        value: Value,
        /// Optional deferred expression for parameterized inductance values.
        value_expr: Option<String>,
        initial_current: Option<Value>,
        /// Optional model name for model-based inductor instances.
        model: Option<String>,
        /// Optional instance parameters (e.g. `M`, `SCALE`, `TC1`, `TC2`).
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured as expressions inside subcircuit
        /// bodies; resolved against the instance scope during flattening
        /// and merged over `instance_params`.
        deferred_params: Vec<(String, String)>,
    },

    /// Jiles-Atherton hysteresis inductor: magnetic core with nonlinear B-H curve
    /// Syntax: L1 n+ n- value MODEL=JA_MODEL
    JilesAthertonInductor {
        /// Base inductance value (Henries)
        value: Value,
        /// Model name referencing a JA model definition
        model: String,
        /// Initial current (Amps)
        initial_current: Option<Value>,
    },

    //-------------------------------------------------------------------------
    // Sources
    //-------------------------------------------------------------------------
    /// Voltage source
    VoltageSource(SourceSpec),

    /// Current source
    CurrentSource(SourceSpec),

    //-------------------------------------------------------------------------
    // Semiconductor Devices
    //-------------------------------------------------------------------------
    /// Diode
    Diode {
        model: String,
        /// Instance parameters (e.g. `AREA`, `M`, `TEMP`, `DTEMP`, `OFF`).
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured as expressions inside subcircuit
        /// bodies; resolved against the instance scope during flattening
        /// and merged over `instance_params`.
        deferred_params: Vec<(String, String)>,
    },

    /// BJT (NPN or PNP)
    Bjt {
        model: String,
        bjt_type: BjtType,
        /// Instance parameters (e.g. `AREA`, `M`, `TEMP`).
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured as expressions inside subcircuit
        /// bodies; resolved against the instance scope during flattening
        /// and merged over `instance_params`.
        deferred_params: Vec<(String, String)>,
    },

    /// MOSFET
    Mosfet {
        model: String,
        mos_type: MosType,
        /// Parsed from compact `Mname D G S model` syntax. This is accepted
        /// only for VDMOS-compatible models; ordinary MOS devices require an
        /// explicit bulk node even when it is tied to source.
        compact_syntax: bool,
        /// Instance parameters (e.g. `W`, `L`, `M`, `NF`).
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured as expressions inside subcircuit
        /// bodies; resolved against the instance scope during flattening
        /// and merged over `instance_params`.
        deferred_params: Vec<(String, String)>,
    },

    /// JFET (NJF or PJF)
    Jfet {
        model: String,
        jfet_type: JfetType,
        /// Instance parameters (e.g. `AREA`, `M`, `W`, `L`).
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured as expressions inside subcircuit
        /// bodies; resolved against the instance scope during flattening
        /// and merged over `instance_params`.
        deferred_params: Vec<(String, String)>,
    },

    /// MESFET (GaAs FET: NMF or PMF)
    Mesfet {
        model: String,
        mesfet_type: MesfetType,
        /// Instance parameters (e.g. `AREA`, `M`, `W`, `L`).
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured as expressions inside subcircuit
        /// bodies; resolved against the instance scope during flattening
        /// and merged over `instance_params`.
        deferred_params: Vec<(String, String)>,
    },

    //-------------------------------------------------------------------------
    // Controlled Sources
    //-------------------------------------------------------------------------
    /// Voltage-controlled voltage source: E1 n+ n- nc+ nc- gain
    Vcvs {
        gain: Value,
        control_nodes: (String, String),
    },

    /// Current-controlled current source: F1 n+ n- Vname gain
    Cccs {
        gain: Value,
        control_element: String,
    },

    /// Voltage-controlled current source: G1 n+ n- nc+ nc- gm
    Vccs {
        transconductance: Value,
        control_nodes: (String, String),
    },

    /// Current-controlled voltage source: H1 n+ n- Vname rm
    Ccvs {
        transresistance: Value,
        control_element: String,
    },

    //-------------------------------------------------------------------------
    // Behavioral Sources
    //-------------------------------------------------------------------------
    /// Behavioral voltage source: B1 n+ n- V=expr
    BehavioralVoltage {
        expression: String,
        tc1: Value,
        tc2: Value,
    },

    /// Behavioral current source: B1 n+ n- I=expr
    BehavioralCurrent {
        expression: String,
        tc1: Value,
        tc2: Value,
    },

    //-------------------------------------------------------------------------
    // Switches
    //-------------------------------------------------------------------------
    /// Voltage-controlled switch: S1 n+ n- nc+ nc- MODEL [ON|OFF]
    VSwitch {
        /// Control node positive
        control_pos: String,
        /// Control node negative
        control_neg: String,
        /// Switch model name
        model: String,
        /// Initial state
        initial_state: Option<SwitchState>,
    },

    /// Current-controlled switch: W1 n+ n- Vname MODEL [ON|OFF]
    ISwitch {
        /// Control element (voltage source for sensing)
        control_element: String,
        /// Switch model name
        model: String,
        /// Initial state
        initial_state: Option<SwitchState>,
    },

    /// Xyce generic expression-controlled switch:
    /// SW1 n+ n- MODEL [ON|OFF] CONTROL={expr}
    GenericSwitch {
        /// Switch model name
        model: String,
        /// Scalar control expression
        control_expression: String,
        /// Initial state
        initial_state: Option<SwitchState>,
    },

    //-------------------------------------------------------------------------
    // Transmission Lines
    //-------------------------------------------------------------------------
    /// Transmission line:
    /// - Inline form: T1 p1+ p1- p2+ p2- Z0=val TD=val
    /// - Model form: O1 p1+ p1- p2+ p2- MODELNAME
    TransmissionLine {
        /// Characteristic impedance (Ohms), optional when provided by model card
        z0: Option<Value>,
        /// Propagation delay (seconds) - specify TD or (F, NL)
        td: Option<Value>,
        /// Frequency for NL specification (Hz)
        freq: Option<Value>,
        /// Normalized electrical length at freq (wavelengths)
        nl: Option<Value>,
        /// Optional model name for O/Y/P-style transmission lines
        model: Option<String>,
    },

    //-------------------------------------------------------------------------
    // Coupled Inductors
    //-------------------------------------------------------------------------
    /// Coupling coefficient: K1 L1 L2 [L3...] coefficient
    Coupling {
        /// Names of coupled inductors
        inductors: Vec<String>,
        /// Coupling coefficient (0 < k ≤ 1)
        coefficient: Value,
    },

    //-------------------------------------------------------------------------
    // Subcircuits
    //-------------------------------------------------------------------------
    /// Subcircuit instance: X1 node1 node2... SUBCKTNAME [PARAM=val...]
    Subcircuit {
        subckt_name: String,
        params: Vec<(String, ParametricValue)>,
    },

    //-------------------------------------------------------------------------
    // XSPICE Code Models
    //-------------------------------------------------------------------------
    /// XSPICE code model instance: A1 [in] out model_name [PARAM=val...]
    ///
    /// XSPICE provides mixed-signal simulation capability through code models.
    /// Port connections use bracket syntax to distinguish port types:
    /// - `node` - Analog node (voltage/current)
    /// - `[node]` - Digital node (12-state logic)
    /// - `[n1 n2 n3]` - Vector of digital nodes
    /// - `%vd[n+ n-]` - Differential voltage input
    /// - `%id[n+ n-]` - Differential current input
    /// - `null` - Unconnected port
    ///
    /// Example: `A1 [clk] [d] [q] [qbar] d_dff rise_delay=10n`
    Xspice {
        /// Code model type name (e.g., "gain", "d_and", "adc_bridge")
        model: String,
        /// Port connections with type information
        ports: Vec<XspicePort>,
        /// Instance parameter overrides
        params: Vec<(String, Value)>,
    },
}

/// Switch initial state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchState {
    On,
    Off,
}

//=============================================================================
// Output selection (.save / .probe / .print / .plot)
//=============================================================================

/// One requested output signal from a `.save`/`.probe`/`.print`/`.plot` card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveSignal {
    /// `all` — keep every computed vector.
    All,
    /// `v(node)` — a node voltage.
    Voltage(String),
    /// `v(a,b)` — a differential voltage probe.
    VoltageDiff(String, String),
    /// `i(elem)` — a branch/element current.
    Current(String),
    /// `@dev[param]` — a device-parameter probe.
    DeviceParam { device: String, param: String },
    /// A bare vector name (`out` is shorthand for `v(out)` in ngspice).
    Raw(String),
}

/// Accumulated output selection for a netlist.
///
/// Empty means "no directive given", in which case simulators keep every
/// vector. A non-empty set restricts stored/exported vectors to the selected
/// signals; analysis scale vectors (time, frequency, sweep) are always kept.
#[derive(Debug, Clone, Default)]
pub struct SaveSet {
    /// Selected signals in netlist order (duplicates are harmless).
    pub signals: Vec<SaveSignal>,
}

impl SaveSet {
    /// `true` when no `.save`-family directive was given.
    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }

    /// `true` when every vector should be kept (no directive, or `all`).
    pub fn keeps_everything(&self) -> bool {
        self.is_empty() || self.signals.iter().any(|s| matches!(s, SaveSignal::All))
    }

    /// Whether an output vector named `variable` is selected.
    ///
    /// `variable` follows raw-file conventions: `v(out)`, `V(OUT)`, `i(v1)`,
    /// `vd#branch`, `time`, `frequency`. Matching is case-insensitive and
    /// scale vectors are always selected.
    ///
    /// Voltage, current and raw selections may carry `*` wildcards with
    /// Spectre `save` semantics: `*` matches within one hierarchy level and
    /// never crosses a `.` separator, so `v(x1.*)` selects every net directly
    /// inside `X1` while leaving `x1.xb.nref` to `v(x1.*.*)` or an explicit
    /// probe.
    pub fn selects(&self, variable: &str) -> bool {
        if self.keeps_everything() {
            return true;
        }

        let var = variable.trim().to_ascii_lowercase();
        if matches!(
            var.as_str(),
            "time" | "frequency" | "freq" | "v-sweep" | "v(v-sweep)" | "sweep" | "temp-sweep"
        ) {
            return true;
        }

        let inner_v = var.strip_prefix("v(").and_then(|s| s.strip_suffix(')'));
        let inner_i = var.strip_prefix("i(").and_then(|s| s.strip_suffix(')'));
        let branch = var.strip_suffix("#branch");

        for signal in &self.signals {
            match signal {
                SaveSignal::All => return true,
                SaveSignal::Voltage(node) => {
                    let node = node.to_ascii_lowercase();
                    if pattern_selects(&node, inner_v.unwrap_or(var.as_str())) {
                        return true;
                    }
                }
                SaveSignal::VoltageDiff(a, b) => {
                    let probe = format!("v({},{})", a.to_ascii_lowercase(), b.to_ascii_lowercase());
                    if var.replace(' ', "") == probe {
                        return true;
                    }
                }
                SaveSignal::Current(elem) => {
                    let elem = elem.to_ascii_lowercase();
                    if inner_i
                        .or(branch)
                        .is_some_and(|t| pattern_selects(&elem, t))
                    {
                        return true;
                    }
                }
                SaveSignal::DeviceParam { device, param } => {
                    let bracket_probe = format!(
                        "@{}[{}]",
                        device.to_ascii_lowercase(),
                        param.to_ascii_lowercase()
                    );
                    let xyce_probe = format!(
                        "n({}:{})",
                        device.to_ascii_lowercase(),
                        param.to_ascii_lowercase()
                    );
                    if var == bracket_probe || var == xyce_probe {
                        return true;
                    }
                }
                SaveSignal::Raw(name) => {
                    let name = name.to_ascii_lowercase();
                    if pattern_selects(&name, &var)
                        || inner_v.is_some_and(|t| pattern_selects(&name, t))
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}

/// Match a save selection against a vector name, honoring `*` wildcards.
///
/// Without a `*` this is plain equality. With one, `*` matches any run of
/// characters except the `.` hierarchy separator (Spectre `save` semantics),
/// so `x1.*` covers `x1.ntail` but not `x1.xb.nref`. Inputs are expected
/// pre-lowercased by the caller.
fn pattern_selects(pattern: &str, text: &str) -> bool {
    if !pattern.contains('*') {
        return pattern == text;
    }

    // Iterative glob with single-star backtracking; the star is forbidden
    // from swallowing '.' so wildcards stay within one hierarchy level.
    let (pattern, text) = (pattern.as_bytes(), text.as_bytes());
    let (mut p, mut t) = (0usize, 0usize);
    let mut star: Option<(usize, usize)> = None;
    while t < text.len() {
        if p < pattern.len() && (pattern[p] == text[t] && pattern[p] != b'*') {
            p += 1;
            t += 1;
        } else if p < pattern.len() && pattern[p] == b'*' {
            star = Some((p, t));
            p += 1;
        } else if let Some((star_p, star_t)) = star {
            if text[star_t] == b'.' {
                return false;
            }
            star = Some((star_p, star_t + 1));
            p = star_p + 1;
            t = star_t + 1;
        } else {
            return false;
        }
    }
    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}

//=============================================================================
// XSPICE Port Types
//=============================================================================

/// XSPICE port connection specification
///
/// XSPICE uses bracket syntax to distinguish port types. This enum captures
/// the various connection types supported by XSPICE code models.
///
/// # Examples
/// ```text
/// A1 in out gain              ; 'in' and 'out' are analog nodes
/// A2 [clk] [d] [q] d_dff      ; [clk], [d], [q] are digital ports
/// A3 [a b c] [y] d_and        ; [a b c] is a digital vector
/// A4 %vd[n+ n-] out gain      ; differential voltage input
/// A5 null out d_source        ; null = unconnected
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum XspicePort {
    /// Single analog node (voltage or current)
    /// Syntax: `nodename`
    Analog(String),

    /// Single digital node (12-state logic)
    /// Syntax: `[nodename]`
    Digital(String),

    /// Vector of analog nodes
    /// Syntax: `(n1 n2 n3)` (uncommon)
    AnalogVector(Vec<String>),

    /// Vector of digital nodes
    /// Syntax: `[n1 n2 n3]`
    DigitalVector(Vec<String>),

    /// Differential voltage input/output
    /// Syntax: `%vd[n+ n-]` or `%vd(n+ n-)`
    DifferentialVoltage { pos: String, neg: String },

    /// Differential current input/output
    /// Syntax: `%id[n+ n-]` or `%id(n+ n-)`
    DifferentialCurrent { pos: String, neg: String },

    /// Null connection (unconnected port)
    /// Syntax: `null` or `[]`
    Null,
}

impl XspicePort {
    /// Create a single analog port
    pub fn analog(node: impl Into<String>) -> Self {
        XspicePort::Analog(node.into())
    }

    /// Create a single digital port
    pub fn digital(node: impl Into<String>) -> Self {
        XspicePort::Digital(node.into())
    }

    /// Create a digital vector from node names
    pub fn digital_vector(nodes: Vec<String>) -> Self {
        XspicePort::DigitalVector(nodes)
    }

    /// Create a differential voltage port
    pub fn diff_voltage(pos: impl Into<String>, neg: impl Into<String>) -> Self {
        XspicePort::DifferentialVoltage {
            pos: pos.into(),
            neg: neg.into(),
        }
    }

    /// Check if this is an analog port
    pub fn is_analog(&self) -> bool {
        matches!(
            self,
            XspicePort::Analog(_)
                | XspicePort::AnalogVector(_)
                | XspicePort::DifferentialVoltage { .. }
                | XspicePort::DifferentialCurrent { .. }
        )
    }

    /// Check if this is a digital port
    pub fn is_digital(&self) -> bool {
        matches!(self, XspicePort::Digital(_) | XspicePort::DigitalVector(_))
    }

    /// Check if this is a null connection
    pub fn is_null(&self) -> bool {
        matches!(self, XspicePort::Null)
    }

    /// Get all node names referenced by this port
    pub fn node_names(&self) -> Vec<&str> {
        match self {
            XspicePort::Analog(n) | XspicePort::Digital(n) => vec![n.as_str()],
            XspicePort::AnalogVector(v) | XspicePort::DigitalVector(v) => {
                v.iter().map(|s| s.as_str()).collect()
            }
            XspicePort::DifferentialVoltage { pos, neg }
            | XspicePort::DifferentialCurrent { pos, neg } => {
                vec![pos.as_str(), neg.as_str()]
            }
            XspicePort::Null => vec![],
        }
    }
}

//=============================================================================
// Transistor Types
//=============================================================================

/// BJT transistor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BjtType {
    Npn,
    Pnp,
}

/// MOSFET type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosType {
    Nmos,
    Pmos,
}

/// JFET type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JfetType {
    Njf,
    Pjf,
}

/// MESFET transistor type (GaAs)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MesfetType {
    #[default]
    Nmf, // N-channel MESFET
    Pmf, // P-channel MESFET
}

/// Second (outer) source of a two-source `.DC` sweep.
#[derive(Debug, Clone)]
pub struct DcSecondSweep {
    pub source: String,
    pub start: Value,
    pub stop: Value,
    pub step: Value,
}

//=============================================================================
// Source Specifications
//=============================================================================

/// Source specification (DC, AC, or transient waveforms)
#[derive(Debug, Clone)]
pub enum SourceSpec {
    /// DC value
    Dc(Value),

    /// AC magnitude and phase
    Ac { magnitude: Value, phase: Value },

    /// Combined DC and AC specification (common SPICE syntax: DC x AC y [phase])
    DcAc {
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
    },

    /// Combined DC operating value and transient waveform specification
    /// (common SPICE syntax: `DC x PULSE(...)`, `DC x PWL(...)`, etc).
    ///
    /// The explicit `dc_value` is used for operating point / DC analyses,
    /// while `transient` drives time-domain evaluation.
    DcTransient {
        dc_value: Value,
        transient: Box<SourceSpec>,
    },

    /// Combined DC, AC, and transient specification.
    ///
    /// Common in ngspice decks where one source drives:
    /// - DC operating point (`dc_value`)
    /// - AC small-signal excitation (`ac_magnitude`, `ac_phase`)
    /// - Time-domain excitation (`transient`)
    DcAcTransient {
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        transient: Box<SourceSpec>,
    },

    /// Pulse source: PULSE(v1 v2 td tr tf pw per)
    Pulse {
        v1: Value,
        v2: Value,
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
        width_defaults_to_zero: bool,
    },

    /// Sinusoidal source: SIN(vo va freq td theta phase)
    Sin {
        offset: Value,
        amplitude: Value,
        frequency: Value,
        delay: Value,
        damping: Value,
        phase: Value,
    },

    /// Piecewise linear source: PWL(t1 v1 t2 v2 ...)
    Pwl { points: Vec<(Value, Value)> },

    /// Piecewise linear source from external file: PWL FILE="filename"
    /// Supports CSV (time,value columns) and WAV audio files
    PwlFile {
        /// Path to the data file (CSV or WAV)
        path: String,
        /// Time scaling factor (default 1.0)
        time_scale: Value,
        /// Value scaling factor (default 1.0)  
        value_scale: Value,
        /// Time offset (default 0.0)
        time_offset: Value,
        /// Value offset (default 0.0)
        value_offset: Value,
    },

    /// Exponential source: EXP(v1 v2 td1 tau1 td2 tau2)
    Exp {
        v1: Value,
        v2: Value,
        td1: Value,
        tau1: Value,
        td2: Value,
        tau2: Value,
    },

    /// Single-frequency FM source: SFFM(VO VA FC MDI FM TD PHASEM PHASEC)
    ///
    /// `v(t) = VO + VA*sin(2*pi*FC*(t-TD) + PHASEC + MDI*sin(2*pi*FM*(t-TD) + PHASEM))`
    /// for `t > TD`, and exactly 0 before (ngspice vsrcload.c semantics).
    /// Omitted `carrier_freq`/`signal_freq` are stored as NaN and resolved
    /// against the active transient's stop time at evaluation.
    Sffm {
        /// VO: output offset
        offset: Value,
        /// VA: carrier amplitude
        amplitude: Value,
        /// FC: carrier frequency in Hz (NaN = ngspice default 5/tstop)
        carrier_freq: Value,
        /// MDI: modulation index (clamped to `[0, FC/FM]` like ngspice)
        modulation_index: Value,
        /// FM: signal (modulating) frequency in Hz (NaN/0 = 500/tstop)
        signal_freq: Value,
        /// TD: delay before the waveform starts
        delay: Value,
        /// PHASEM: modulation phase in degrees
        phase_modulation: Value,
        /// PHASEC: carrier phase in degrees
        phase_carrier: Value,
    },

    /// Amplitude-modulation source: AM(VO VMO VMA FM FC TD PHASEM PHASEC)
    ///
    /// `v(t) = VO + (VMO + VMA*sin(2*pi*FM*(t-TD) + PHASEM)) * sin(2*pi*FC*(t-TD) + PHASEC)`
    /// for `t > TD`, and exactly 0 before (ngspice vsrcload.c semantics).
    /// Omitted `modulating_freq`/`carrier_freq` are stored as NaN and
    /// resolved against the active transient's stop time at evaluation.
    Am {
        /// VO: overall output offset
        offset: Value,
        /// VMO: modulation offset
        modulation_offset: Value,
        /// VMA: modulation amplitude (ngspice default 1)
        modulation_amplitude: Value,
        /// FM: modulating frequency in Hz (NaN = ngspice default 5/tstop)
        modulating_freq: Value,
        /// FC: carrier frequency in Hz (NaN = ngspice default 500/tstop)
        carrier_freq: Value,
        /// TD: delay before the waveform starts
        delay: Value,
        /// PHASEM: modulation phase in degrees
        phase_modulation: Value,
        /// PHASEC: carrier phase in degrees
        phase_carrier: Value,
    },

    /// Transient noise source: TRNOISE(NA NT NALPHA NAMP)
    ///
    /// Gaussian white noise of RMS amplitude `NA` sampled every `NT`
    /// seconds, plus 1/f^NALPHA noise of amplitude `NAMP` (Kasdin
    /// fractional-integration filter, matching ngspice's trnoise). The
    /// transient front end expands this spec into a seeded, deterministic
    /// PWL sample train before circuit construction; the DC operating
    /// point sees exactly 0 (zero-mean noise, ngspice semantics).
    TrNoise {
        /// NA: white-noise RMS amplitude (V or A). 0 disables.
        na: Value,
        /// NT: sample interval (s). Must be > 0 when any amplitude is set.
        nt: Value,
        /// NALPHA: 1/f exponent, 0 < alpha < 2. 0 disables flicker.
        nalpha: Value,
        /// NAMP: 1/f^alpha amplitude. 0 disables flicker.
        namp: Value,
    },
}

impl SourceSpec {
    /// Return this specification with its AC excitation replaced by the
    /// given magnitude and phase, preserving DC and transient content.
    ///
    /// Pure waveform specs gain a `DcAcTransient` wrapper with a zero DC
    /// value, matching how SPICE treats `AC` annotations on such sources.
    pub fn with_ac(self, magnitude: Value, phase: Value) -> Self {
        match self {
            SourceSpec::Dc(dc_value) => SourceSpec::DcAc {
                dc_value,
                ac_magnitude: magnitude,
                ac_phase: phase,
            },
            SourceSpec::Ac { .. } => SourceSpec::Ac { magnitude, phase },
            SourceSpec::DcAc { dc_value, .. } => SourceSpec::DcAc {
                dc_value,
                ac_magnitude: magnitude,
                ac_phase: phase,
            },
            SourceSpec::DcTransient {
                dc_value,
                transient,
            }
            | SourceSpec::DcAcTransient {
                dc_value,
                transient,
                ..
            } => SourceSpec::DcAcTransient {
                dc_value,
                ac_magnitude: magnitude,
                ac_phase: phase,
                transient,
            },
            transient => SourceSpec::DcAcTransient {
                dc_value: 0.0,
                ac_magnitude: magnitude,
                ac_phase: phase,
                transient: Box::new(transient),
            },
        }
    }

    /// Replace the DC operating value, preserving AC and transient parts.
    /// A purely AC or transient spec gains an explicit DC component.
    pub fn with_dc_value(self, value: Value) -> SourceSpec {
        match self {
            SourceSpec::Dc(_) => SourceSpec::Dc(value),
            SourceSpec::Ac { magnitude, phase } => SourceSpec::DcAc {
                dc_value: value,
                ac_magnitude: magnitude,
                ac_phase: phase,
            },
            SourceSpec::DcAc {
                ac_magnitude,
                ac_phase,
                ..
            } => SourceSpec::DcAc {
                dc_value: value,
                ac_magnitude,
                ac_phase,
            },
            SourceSpec::DcTransient { transient, .. } => SourceSpec::DcTransient {
                dc_value: value,
                transient,
            },
            SourceSpec::DcAcTransient {
                ac_magnitude,
                ac_phase,
                transient,
                ..
            } => SourceSpec::DcAcTransient {
                dc_value: value,
                ac_magnitude,
                ac_phase,
                transient,
            },
            transient => SourceSpec::DcTransient {
                dc_value: value,
                transient: Box::new(transient),
            },
        }
    }
}

//=============================================================================
// Analysis Commands
//=============================================================================

/// Analysis command from netlist
#[derive(Debug, Clone)]
pub enum AnalysisCommand {
    /// DC operating point: .OP
    Op,

    /// DC sweep: .DC source start stop step [source2 start2 stop2 step2]
    Dc {
        source: String,
        start: Value,
        stop: Value,
        step: Value,
        /// Optional second (outer) sweep source: the first source sweeps
        /// fully at every value of this one, ngspice-style.
        sweep2: Option<DcSecondSweep>,
    },

    /// AC analysis: .AC DEC|LIN|OCT np fstart fstop
    Ac {
        variation: FreqVariation,
        points: usize,
        start_freq: Value,
        stop_freq: Value,
    },

    /// Loop-stability analysis: .STB DEC|LIN|OCT np fstart fstop PROBE=vname
    ///
    /// Tian double-injection loop gain at a designated 0 V voltage source
    /// placed in series with the feedback path (the Spectre probe
    /// convention). The probe name also parses as a bare trailing token.
    Stb {
        variation: FreqVariation,
        points: usize,
        start_freq: Value,
        stop_freq: Value,
        /// Name of the 0 V voltage source serving as the loop probe.
        probe: String,
    },

    /// Distortion analysis: .DISTO DEC|LIN|OCT np fstart fstop [f2overf1]
    Disto {
        variation: FreqVariation,
        points: usize,
        start_freq: Value,
        stop_freq: Value,
        f2_over_f1: Option<Value>,
    },

    /// Transient analysis: .TRAN tstep tstop [tstart [tmaxstep]] [UIC]
    Tran {
        step: Value,
        stop: Value,
        start: Option<Value>,
        max_step: Option<Value>,
        /// Skip the operating point and integrate from user initial
        /// conditions (.IC node voltages, per-element IC= values).
        uic: bool,
    },

    /// Noise analysis: .NOISE V(out) Vsource DEC|LIN|OCT np fstart fstop
    Noise {
        output_node: String,
        reference_node: Option<String>,
        input_source: String,
        variation: FreqVariation,
        points: usize,
        start_freq: Value,
        stop_freq: Value,
    },

    /// Pole-zero analysis: .PZ in+ in- out+ out- VOL|CUR PZ|POL|ZER
    PoleZero {
        input_pos: String,
        input_neg: String,
        output_pos: String,
        output_neg: String,
        transfer_type: PoleZeroTransferType,
        analysis_type: PoleZeroAnalysisType,
    },

    /// Sensitivity analysis: .SENS V(out[,ref]) [AC DEC|LIN|OCT np fstart fstop]
    Sensitivity {
        output_node: String,
        reference_node: Option<String>,
        ac_sweep: Option<SensitivityAcSweep>,
    },

    /// DC small-signal transfer function: .TF V(out[,ref]) insrc
    /// or .TF I(element) insrc — gain, input resistance, output resistance.
    Tf {
        /// Output node for `V(...)` probes, element name for `I(...)`.
        output_node: String,
        /// Reference node of a differential `V(out,ref)` probe.
        reference_node: Option<String>,
        /// True when the probe is a branch current `I(element)`.
        output_is_current: bool,
        /// Independent source the transfer function is taken from.
        input_source: String,
    },

    /// Fourier analysis: .FOUR freq output1 [output2...]
    Four {
        fundamental: Value,
        outputs: Vec<String>,
        num_harmonics: usize,
    },

    /// Monte Carlo analysis:
    /// .MC runs [SEED n] [DIST GAUSS|UNIFORM] [SPREAD rel] [PARAMS p1 p2 ...]
    MonteCarlo(MonteCarloCommand),

    /// Parametric sweep: .STEP PARAM name start stop increment
    Step(StepCommand),

    /// Temperature sweep: .TEMP t1 [t2 t3...]
    Temp { temperatures: Vec<Value> },
}

/// Monte Carlo command configuration
#[derive(Debug, Clone)]
pub struct MonteCarloCommand {
    /// Number of Monte Carlo runs
    pub runs: usize,
    /// Optional RNG seed (deterministic when set)
    pub seed: Option<u64>,
    /// Statistical distribution used for parameter perturbation
    pub distribution: MonteCarloDistribution,
    /// Relative spread:
    /// - Gaussian: sigma (e.g. 0.01 = 1% sigma)
    /// - Uniform: half-width tolerance (e.g. 0.05 = +/-5%)
    pub relative_spread: Value,
    /// Optional explicit parameter list (empty = all eligible parameters)
    pub params: Vec<String>,
}

impl MonteCarloCommand {
    /// Create a Monte Carlo command with Spectre-like defaults:
    /// Gaussian 1% variation, no explicit parameter filter.
    pub fn new(runs: usize) -> Self {
        Self {
            runs,
            seed: None,
            distribution: MonteCarloDistribution::Gaussian,
            relative_spread: 0.01,
            params: Vec::new(),
        }
    }
}

/// Monte Carlo distribution choice
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonteCarloDistribution {
    Gaussian,
    Uniform,
    /// Two-point worst-case spread (nominal ± relative_spread).
    WorstCase,
}

/// Parametric sweep specification
#[derive(Debug, Clone)]
pub struct StepCommand {
    /// Type of parameter being swept
    pub target: StepTarget,
    /// Name of parameter/device/model
    pub name: String,
    /// Parameter name (for device/model params)
    pub param_name: Option<String>,
    /// Sweep type and values
    pub sweep: StepSweep,
}

/// What is being swept in a .STEP command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepTarget {
    /// .STEP PARAM name - global parameter
    Param,
    /// .STEP name - device instance parameter
    Device,
    /// .STEP MODEL name - model parameter
    Model,
    /// .STEP TEMP - temperature (special case)
    Temp,
}

/// Sweep specification for .STEP command
#[derive(Debug, Clone)]
pub enum StepSweep {
    /// Linear sweep with increment: start stop increment
    Linear {
        start: Value,
        stop: Value,
        step: Value,
    },
    /// Decade sweep: DEC np start stop
    Decade {
        points_per_decade: usize,
        start: Value,
        stop: Value,
    },
    /// Octave sweep: OCT np start stop
    Octave {
        points_per_octave: usize,
        start: Value,
        stop: Value,
    },
    /// List of specific values: LIST v1 v2 v3...
    List(Vec<Value>),
}

/// Frequency variation type for AC/noise analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreqVariation {
    /// Linear sweep
    Lin,
    /// Octave sweep
    Oct,
    /// Decade sweep
    Dec,
}

/// Transfer type for .PZ analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoleZeroTransferType {
    /// Voltage transfer function
    Voltage,
    /// Current transfer function
    Current,
}

/// Analysis mode for .PZ analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoleZeroAnalysisType {
    /// Compute both poles and zeros
    PoleZero,
    /// Compute poles only
    PolesOnly,
    /// Compute zeros only
    ZerosOnly,
}

/// AC sweep configuration for sensitivity analysis
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensitivityAcSweep {
    pub variation: FreqVariation,
    pub points: usize,
    pub start_freq: Value,
    pub stop_freq: Value,
}

//=============================================================================
// Initial Conditions
//=============================================================================

/// Initial condition specification
#[derive(Debug, Clone)]
pub struct InitialCondition {
    /// Node name
    pub node: String,
    /// Initial voltage
    pub voltage: Value,
}

/// Nodeset hint for operating point
#[derive(Debug, Clone)]
pub struct NodeSet {
    /// Node name
    pub node: String,
    /// Suggested voltage
    pub voltage: Value,
}

//=============================================================================
// Simulation Options
//=============================================================================

/// Simulation options from .OPTIONS command
///
/// Controls numerical parameters for simulation accuracy and convergence.
/// All fields are optional - unspecified values use engine defaults.
#[derive(Debug, Clone, Default)]
pub struct SimulationOptions {
    /// Relative tolerance for convergence (default: 1e-3)
    pub reltol: Option<Value>,
    /// Absolute current tolerance (default: 1e-12 A)
    pub abstol: Option<Value>,
    /// Absolute voltage tolerance (default: 1e-6 V)
    pub vntol: Option<Value>,
    /// Absolute current tolerance for residual checks (default: 1e-12 A)
    pub iabstol: Option<Value>,
    /// Relative residual tolerance for equation convergence checks (default: RELTOL)
    pub residual_reltol: Option<Value>,
    /// Minimum conductance (default: 1e-12 S)
    pub gmin: Option<Value>,
    /// Integration method: "TRAP", "GEAR", "TRAPGEAR"
    pub method: Option<String>,
    /// Transient error tolerance factor (default: 7.0)
    pub trtol: Option<Value>,
    /// Maximum Newton-Raphson iterations (default: 150)
    pub itl1: Option<usize>,
    /// DC transfer curve iterations (default: 50)
    pub itl2: Option<usize>,
    /// Transient analysis iterations (default: 10)
    pub itl4: Option<usize>,
    /// Source stepping max iterations (default: 500)
    pub itl6: Option<usize>,
    /// Charge tolerance for capacitors (default: 1e-14 C)
    pub chgtol: Option<Value>,
    /// Pivot tolerance for matrix operations (default: 1e-13)
    pub pivtol: Option<Value>,
    /// Temperature in Celsius (default: 27)
    pub temp: Option<Value>,
    /// Nominal temperature in Celsius (default: 27)
    pub tnom: Option<Value>,
    /// Seed for the statistical expression functions
    /// (`gauss`/`agauss`/`unif`/`aunif`/2-arg `limit`). Applied netlist-wide
    /// before parameter evaluation regardless of where the `.options` line
    /// appears; Monte-Carlo drivers derive per-run streams from it.
    pub seed: Option<u64>,
    /// Accept MOS model levels with no native implementation (e.g. BSIM3
    /// LEVEL=8/49, BSIM4 LEVEL=14/54) and run them on the simplified
    /// short-channel approximation instead of rejecting the deck. The
    /// approximation ignores nearly the entire BSIM parameter set, so this
    /// is opt-in; results will not match the named model.
    pub allow_simplified_mos: Option<bool>,
}

impl SimulationOptions {
    /// Create empty options (all defaults)
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge another options set, preferring values from `other`
    pub fn merge(&mut self, other: &SimulationOptions) {
        if other.reltol.is_some() {
            self.reltol = other.reltol;
        }
        if other.abstol.is_some() {
            self.abstol = other.abstol;
        }
        if other.vntol.is_some() {
            self.vntol = other.vntol;
        }
        if other.iabstol.is_some() {
            self.iabstol = other.iabstol;
        }
        if other.residual_reltol.is_some() {
            self.residual_reltol = other.residual_reltol;
        }
        if other.gmin.is_some() {
            self.gmin = other.gmin;
        }
        if other.method.is_some() {
            self.method = other.method.clone();
        }
        if other.trtol.is_some() {
            self.trtol = other.trtol;
        }
        if other.itl1.is_some() {
            self.itl1 = other.itl1;
        }
        if other.itl2.is_some() {
            self.itl2 = other.itl2;
        }
        if other.itl4.is_some() {
            self.itl4 = other.itl4;
        }
        if other.itl6.is_some() {
            self.itl6 = other.itl6;
        }
        if other.chgtol.is_some() {
            self.chgtol = other.chgtol;
        }
        if other.pivtol.is_some() {
            self.pivtol = other.pivtol;
        }
        if other.temp.is_some() {
            self.temp = other.temp;
        }
        if other.tnom.is_some() {
            self.tnom = other.tnom;
        }
        if other.seed.is_some() {
            self.seed = other.seed;
        }
        if other.allow_simplified_mos.is_some() {
            self.allow_simplified_mos = other.allow_simplified_mos;
        }
    }
}

//=============================================================================
// Model and Subcircuit Definitions
//=============================================================================

/// Model definition: .MODEL name type (params)
#[derive(Debug, Clone)]
pub struct ModelDef {
    pub name: String,
    pub model_type: String,
    pub params: Vec<(String, Value)>,
    pub expr_params: Vec<(String, String)>,
    pub string_params: Vec<(String, String)>,
    pub real_vector_params: Vec<(String, Vec<Value>)>,
    pub integer_vector_params: Vec<(String, Vec<i64>)>,
}

/// Subcircuit definition: .SUBCKT name ports [PARAMS: ...]
///
/// Represents a reusable circuit block that can be instantiated with X elements.
/// Follows standard conventions for parameter scoping and local options.
#[derive(Debug, Clone)]
pub struct SubcircuitDef {
    /// Subcircuit name
    pub name: String,
    /// Port (terminal) names in connection order
    pub ports: Vec<String>,
    /// Internal elements
    pub elements: Vec<Element>,
    /// Default parameter values (can be overridden at instance)
    pub params: Vec<(String, Value)>,
    /// Local simulation options scoped to this subcircuit
    /// (temp, scale, reltol, etc.)
    pub local_options: std::collections::HashMap<String, Value>,
    /// Optional parent library reference for cross-linking
    pub library_ref: Option<String>,
    /// Nested subcircuit definitions (for hierarchical parsing)
    pub nested_subcircuits: Vec<SubcircuitDef>,
}

//=============================================================================
// Include/Lib Directives
//=============================================================================

/// Include directive: .INCLUDE "filename"
#[derive(Debug, Clone)]
pub struct IncludeDirective {
    /// File path (relative or absolute)
    pub path: String,
}

/// Library directive: .LIB "filename" [section]
#[derive(Debug, Clone)]
pub struct LibDirective {
    /// File path
    pub path: String,
    /// Optional section name within the library
    pub section: Option<String>,
}

//=============================================================================
// Tests
//=============================================================================
