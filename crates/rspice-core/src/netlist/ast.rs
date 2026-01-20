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
    /// Resistor: value in Ohms
    Resistor { value: Value },

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
    Bjt { model: String, bjt_type: BjtType },

    /// MOSFET
    Mosfet { model: String, mos_type: MosType },

    /// JFET (NJF or PJF)
    Jfet { model: String, jfet_type: JfetType },

    /// MESFET (GaAs FET: NMF or PMF)
    Mesfet {
        model: String,
        mesfet_type: MesfetType,
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
    /// Lossless transmission line: T1 port1+ port1- port2+ port2- Z0=val TD=val
    TransmissionLine {
        /// Characteristic impedance (Ohms)
        z0: Value,
        /// Propagation delay (seconds) - specify TD or (F, NL)
        td: Option<Value>,
        /// Frequency for NL specification (Hz)
        freq: Option<Value>,
        /// Normalized electrical length at freq (wavelengths)
        nl: Option<Value>,
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

    /// Fourier analysis: .FOUR freq output1 [output2...]
    Four {
        fundamental: Value,
        outputs: Vec<String>,
        num_harmonics: usize,
    },

    /// Parametric sweep: .STEP PARAM name start stop increment
    Step(StepCommand),

    /// Temperature sweep: .TEMP t1 [t2 t3...]
    Temp { temperatures: Vec<Value> },
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
#[derive(Debug, Clone)]
pub struct SubcircuitDef {
    pub name: String,
    pub ports: Vec<String>,
    pub elements: Vec<Element>,
    /// Default parameter values
    pub params: Vec<(String, Value)>,
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
            kind: ElementKind::Resistor { value: 1000.0 },
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
                z0: 50.0,
                td: Some(1e-9),
                freq: None,
                nl: None,
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
}
