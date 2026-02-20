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
        /// Optional model name for model-based resistor instances.
        model: Option<String>,
        /// Optional instance parameters (e.g., `L`, `W`, `M`, `R`).
        instance_params: Vec<(String, Value)>,
    },

    /// Capacitor: value in Farads
    Capacitor {
        value: Value,
        initial_voltage: Option<Value>,
    },

    /// Inductor: value in Henries
    Inductor {
        value: Value,
        initial_current: Option<Value>,
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
    Diode { model: String },

    /// BJT (NPN or PNP)
    Bjt {
        model: String,
        bjt_type: BjtType,
        /// Instance parameters (e.g. `AREA`, `M`, `TEMP`).
        instance_params: Vec<(String, Value)>,
    },

    /// MOSFET
    Mosfet {
        model: String,
        mos_type: MosType,
        /// Instance parameters (e.g. `W`, `L`, `M`, `NF`).
        instance_params: Vec<(String, Value)>,
    },

    /// JFET (NJF or PJF)
    Jfet {
        model: String,
        jfet_type: JfetType,
        /// Instance parameters (e.g. `AREA`, `M`, `W`, `L`).
        instance_params: Vec<(String, Value)>,
    },

    /// MESFET (GaAs FET: NMF or PMF)
    Mesfet {
        model: String,
        mesfet_type: MesfetType,
        /// Instance parameters (e.g. `AREA`, `M`, `W`, `L`).
        instance_params: Vec<(String, Value)>,
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
    BehavioralVoltage { expression: String },

    /// Behavioral current source: B1 n+ n- I=expr
    BehavioralCurrent { expression: String },

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
        params: Vec<(String, Value)>,
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
}

//=============================================================================
// Analysis Commands
//=============================================================================

/// Analysis command from netlist
#[derive(Debug, Clone)]
pub enum AnalysisCommand {
    /// DC operating point: .OP
    Op,

    /// DC sweep: .DC source start stop step
    Dc {
        source: String,
        start: Value,
        stop: Value,
        step: Value,
    },

    /// AC analysis: .AC DEC|LIN|OCT np fstart fstop
    Ac {
        variation: FreqVariation,
        points: usize,
        start_freq: Value,
        stop_freq: Value,
    },

    /// Distortion analysis: .DISTO DEC|LIN|OCT np fstart fstop [f2overf1]
    Disto {
        variation: FreqVariation,
        points: usize,
        start_freq: Value,
        stop_freq: Value,
        f2_over_f1: Option<Value>,
    },

    /// Transient analysis: .TRAN tstep tstop [tstart [tmaxstep]]
    Tran {
        step: Value,
        stop: Value,
        start: Option<Value>,
        max_step: Option<Value>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let r = Element {
            name: "R1".to_string(),
            kind: ElementKind::Resistor {
                value: 1000.0,
                model: None,
                instance_params: Vec::new(),
            },
            nodes: vec!["1".to_string(), "0".to_string()],
        };
        assert_eq!(r.name, "R1");
        assert_eq!(r.nodes.len(), 2);
    }

    #[test]
    fn test_switch_element() {
        let sw = Element {
            name: "S1".to_string(),
            kind: ElementKind::VSwitch {
                control_pos: "ctrl".to_string(),
                control_neg: "0".to_string(),
                model: "SW1".to_string(),
                initial_state: Some(SwitchState::Off),
            },
            nodes: vec!["out".to_string(), "0".to_string()],
        };
        assert_eq!(sw.name, "S1");
    }

    #[test]
    fn test_transmission_line() {
        let tl = Element {
            name: "T1".to_string(),
            kind: ElementKind::TransmissionLine {
                z0: Some(50.0),
                td: Some(1e-9),
                freq: None,
                nl: None,
                model: None,
            },
            nodes: vec![
                "in".to_string(),
                "0".to_string(),
                "out".to_string(),
                "0".to_string(),
            ],
        };
        assert_eq!(tl.nodes.len(), 4);
    }

    #[test]
    fn test_coupling() {
        let k = Element {
            name: "K1".to_string(),
            kind: ElementKind::Coupling {
                inductors: vec!["L1".to_string(), "L2".to_string()],
                coefficient: 0.99,
            },
            nodes: vec![], // Coupling doesn't have nodes
        };
        if let ElementKind::Coupling { coefficient, .. } = k.kind {
            assert!((coefficient - 0.99).abs() < 1e-10);
        }
    }

    #[test]
    fn test_step_command() {
        let step = StepCommand {
            target: StepTarget::Param,
            name: "RL".to_string(),
            param_name: None,
            sweep: StepSweep::Linear {
                start: 100.0,
                stop: 1000.0,
                step: 100.0,
            },
        };
        assert_eq!(step.name, "RL");
    }

    #[test]
    fn test_noise_analysis() {
        let noise = AnalysisCommand::Noise {
            output_node: "out".to_string(),
            reference_node: Some("0".to_string()),
            input_source: "V1".to_string(),
            variation: FreqVariation::Dec,
            points: 10,
            start_freq: 1.0,
            stop_freq: 1e6,
        };
        if let AnalysisCommand::Noise { points, .. } = noise {
            assert_eq!(points, 10);
        }
    }

    #[test]
    fn test_disto_analysis() {
        let disto = AnalysisCommand::Disto {
            variation: FreqVariation::Dec,
            points: 12,
            start_freq: 10.0,
            stop_freq: 1e6,
            f2_over_f1: Some(1.5),
        };
        if let AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } = disto
        {
            assert_eq!(variation, FreqVariation::Dec);
            assert_eq!(points, 12);
            assert!((start_freq - 10.0).abs() < 1e-12);
            assert!((stop_freq - 1e6).abs() < 1e-6);
            assert_eq!(f2_over_f1, Some(1.5));
        }
    }

    #[test]
    fn test_pole_zero_analysis() {
        let pz = AnalysisCommand::PoleZero {
            input_pos: "in".to_string(),
            input_neg: "0".to_string(),
            output_pos: "out".to_string(),
            output_neg: "0".to_string(),
            transfer_type: PoleZeroTransferType::Voltage,
            analysis_type: PoleZeroAnalysisType::PoleZero,
        };
        if let AnalysisCommand::PoleZero {
            transfer_type,
            analysis_type,
            ..
        } = pz
        {
            assert_eq!(transfer_type, PoleZeroTransferType::Voltage);
            assert_eq!(analysis_type, PoleZeroAnalysisType::PoleZero);
        }
    }

    #[test]
    fn test_sensitivity_analysis() {
        let sens = AnalysisCommand::Sensitivity {
            output_node: "out".to_string(),
            reference_node: Some("0".to_string()),
            ac_sweep: Some(SensitivityAcSweep {
                variation: FreqVariation::Dec,
                points: 10,
                start_freq: 1.0,
                stop_freq: 1e6,
            }),
        };
        if let AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            ac_sweep,
        } = sens
        {
            assert_eq!(output_node, "out");
            assert_eq!(reference_node.as_deref(), Some("0"));
            let ac = ac_sweep.expect("expected AC sweep");
            assert_eq!(ac.variation, FreqVariation::Dec);
            assert_eq!(ac.points, 10);
        }
    }

    #[test]
    fn test_monte_carlo_analysis_command() {
        let mut mc = MonteCarloCommand::new(256);
        mc.seed = Some(42);
        mc.distribution = MonteCarloDistribution::Uniform;
        mc.relative_spread = 0.05;
        mc.params = vec!["RVAL".to_string(), "CVAL".to_string()];

        let cmd = AnalysisCommand::MonteCarlo(mc.clone());
        if let AnalysisCommand::MonteCarlo(parsed) = cmd {
            assert_eq!(parsed.runs, 256);
            assert_eq!(parsed.seed, Some(42));
            assert_eq!(parsed.distribution, MonteCarloDistribution::Uniform);
            assert!((parsed.relative_spread - 0.05).abs() < 1e-12);
            assert_eq!(parsed.params, vec!["RVAL".to_string(), "CVAL".to_string()]);
        } else {
            panic!("expected MonteCarlo command");
        }
    }

    #[test]
    fn test_monte_carlo_analysis_worst_case_distribution() {
        let mut mc = MonteCarloCommand::new(32);
        mc.distribution = MonteCarloDistribution::WorstCase;
        mc.relative_spread = 0.03;

        let cmd = AnalysisCommand::MonteCarlo(mc);
        if let AnalysisCommand::MonteCarlo(parsed) = cmd {
            assert_eq!(parsed.distribution, MonteCarloDistribution::WorstCase);
            assert!((parsed.relative_spread - 0.03).abs() < 1e-12);
        } else {
            panic!("expected MonteCarlo command");
        }
    }
}
