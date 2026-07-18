//! Abstract Syntax Tree for netlist representation
//!
//! Provides a complete representation of SPICE netlist elements including:
//! - All standard circuit components (R, L, C, sources)
//! - Semiconductor devices (diodes, BJTs, MOSFETs)
//! - Controlled sources (VCVS, VCCS, CCVS, CCCS)
//! - Advanced components (switches, transmission lines, coupled inductors)
//! - Analysis commands (DC, AC, transient, parametric, noise)

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};

use super::expr::FunctionDef;

/// Internal instance marker for Xyce resistor lines whose value field is
/// omitted. Xyce parses those as zero and then applies a 1000 ohm default
/// during resistor setup; explicit zero-ohm resistors must not use this path.
pub(crate) const XYCE_DEFAULT_RESISTOR_VALUE_MARKER: &str = "__RSPICE_XYCE_DEFAULT_RESISTOR_VALUE";

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
    /// A resolved string value.
    String(String),
    /// A string parameter reference to be evaluated with a parameter context.
    StringExpression(String),
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
            ParametricValue::Expression(_)
            | ParametricValue::String(_)
            | ParametricValue::StringExpression(_) => None,
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
            ParametricValue::String(value) => Err(format!(
                "Unable to resolve string value '{}' as a numeric expression",
                value
            )),
            ParametricValue::StringExpression(expr) => Err(format!(
                "Unable to resolve string expression '{}' as a numeric expression",
                expr
            )),
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
    /// Internal construction provenance used to preserve semantic ownership
    /// across hierarchy flattening. Authored elements always use the default.
    pub provenance: ElementProvenance,
}

/// Construction provenance for an element.
///
/// This is explicit metadata rather than a naming convention: valid authored
/// names may contain the same suffixes used for parser-generated elements.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ElementProvenance {
    #[default]
    Authored,
    GeneratedPassiveHelper {
        /// Name of the authored passive that owns this helper in the same
        /// hierarchy scope.
        owner: String,
        role: GeneratedPassiveHelperRole,
    },
    /// Resistor explicitly generated by Xyce ADDRESISTORS materialization.
    GeneratedXyceAddResistor { mode: XyceAddResistorMode },
}

/// Role of a parser-generated passive parasitic element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedPassiveHelperRole {
    SeriesResistance,
    ParallelResistance,
    ParallelCapacitance,
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

    /// Voltage source captured inside a subcircuit body before instance
    /// parameter scope is known.
    VoltageSourceDeferred(String),

    /// Current source
    CurrentSource(SourceSpec),

    /// Current source captured inside a subcircuit body before instance
    /// parameter scope is known.
    CurrentSourceDeferred(String),

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

    /// Xyce memristor lexical form (`YMEMRISTOR`).
    ///
    /// The model level selects the concrete family when the circuit is built,
    /// so model-card scoping and subcircuit flattening remain independent of
    /// declaration order. Assignments are retained syntactically for the
    /// selected device builder's fail-closed validation.
    XyceMemristor {
        model: String,
        /// Instance parameters resolved while parsing the current scope.
        instance_params: Vec<(String, Value)>,
        /// Instance parameters captured inside subcircuit bodies and resolved
        /// against the concrete instance scope during flattening.
        deferred_params: Vec<(String, String)>,
    },

    //-------------------------------------------------------------------------
    // Controlled Sources
    //-------------------------------------------------------------------------
    /// Voltage-controlled voltage source: E1 n+ n- nc+ nc- gain
    Vcvs {
        gain: Value,
        /// Gain expression captured inside subcircuits and resolved when the
        /// instance parameter scope is known.
        gain_expr: Option<String>,
        control_nodes: (String, String),
    },

    /// Current-controlled current source: F1 n+ n- Vname gain
    Cccs {
        gain: Value,
        /// Gain expression captured inside subcircuits and resolved when the
        /// instance parameter scope is known.
        gain_expr: Option<String>,
        control_element: String,
    },

    /// Voltage-controlled current source: G1 n+ n- nc+ nc- gm
    Vccs {
        transconductance: Value,
        /// Transconductance expression captured inside subcircuits and resolved
        /// when the instance parameter scope is known.
        transconductance_expr: Option<String>,
        control_nodes: (String, String),
    },

    /// Current-controlled voltage source: H1 n+ n- Vname rm
    Ccvs {
        transresistance: Value,
        /// Transresistance expression captured inside subcircuits and resolved
        /// when the instance parameter scope is known.
        transresistance_expr: Option<String>,
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
        /// Internal metadata for PSpice U-device frontend lowering. This is
        /// consumed by parser normalization before simulation.
        pspice_u_timing: Option<PspiceUTiming>,
        /// Port connections with type information
        ports: Vec<XspicePort>,
        /// Instance parameter overrides
        params: Vec<(String, Value)>,
        /// Instance parameter overrides captured as expressions inside
        /// subcircuit bodies; resolved against the instance scope during
        /// flattening and merged over `params`.
        expr_params: Vec<(String, String)>,
        /// String instance parameter overrides.
        string_params: Vec<(String, String)>,
        /// String instance parameter overrides captured as string parameter
        /// references inside subcircuit bodies.
        string_expr_params: Vec<(String, String)>,
        /// String-vector instance parameter overrides.
        string_vector_params: Vec<(String, Vec<String>)>,
        /// String-vector instance parameter overrides captured as string
        /// parameter references inside subcircuit bodies.
        string_vector_expr_params: Vec<(String, String)>,
        /// Real-vector instance parameter overrides.
        real_vector_params: Vec<(String, Vec<Value>)>,
        /// Real-vector instance parameter overrides captured as expressions
        /// inside subcircuit bodies.
        real_vector_expr_params: Vec<(String, Vec<String>)>,
    },
}

/// Internal metadata attached to XSPICE elements lowered from PSpice U devices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PspiceUTiming {
    pub timing_model: String,
    pub delay_mode: PspiceUTimingMode,
}

/// PSpice min/typ/max timing selection carried by MNTYMXDLY.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PspiceUTimingMode {
    Min,
    Typ,
    Max,
}

impl Default for PspiceUTimingMode {
    fn default() -> Self {
        Self::Typ
    }
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

    /// Canonicalize node-zero aliases in every selected voltage probe.
    ///
    /// Frontends that replace the parsed output selection (for example a CLI
    /// `--save` override) must apply the netlist's effective
    /// [`GroundPolicy`](super::GroundPolicy)
    /// before execution so their synthetic selection has the same semantics
    /// as source-authored `.SAVE`/`.PRINT` cards.
    pub fn apply_ground_policy(&mut self, policy: super::GroundPolicy) {
        fn replace(node: &mut String, policy: super::GroundPolicy) {
            let canonical = policy.canonical_node(node);
            if canonical != node {
                *node = canonical.to_string();
            }
        }
        for signal in &mut self.signals {
            match signal {
                SaveSignal::Voltage(node) => replace(node, policy),
                SaveSignal::Raw(raw) => {
                    *raw = super::apply_ground_policy_to_probe_references(raw, policy);
                    replace(raw, policy);
                }
                SaveSignal::VoltageDiff(pos, neg) => {
                    replace(pos, policy);
                    replace(neg, policy);
                }
                SaveSignal::All | SaveSignal::Current(_) | SaveSignal::DeviceParam { .. } => {}
            }
        }
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

    /// Whether a bare/raw output vector name is selected by a raw save entry.
    ///
    /// This intentionally ignores typed probes such as `v(out)` and `i(v1)`.
    /// It is useful for non-voltage/non-current vectors that have their own
    /// type, such as XSPICE digital traces.
    pub fn selects_raw_name(&self, variable: &str) -> bool {
        if self.keeps_everything() {
            return true;
        }

        let var = variable.trim().to_ascii_lowercase();
        self.signals.iter().any(|signal| match signal {
            SaveSignal::All => true,
            SaveSignal::Raw(name) => pattern_selects(&name.to_ascii_lowercase(), &var),
            _ => false,
        })
    }
}

/// Match a save selection against a vector name, honoring `*` wildcards.
///
/// Without a `*` this is plain equality. With one, `*` matches any run of
/// characters except the `.` hierarchy separator (Spectre `save` semantics),
/// so `x1.*` covers `x1.ntail` but not `x1.xb.nref`. Inputs are expected
/// pre-lowercased by the caller.
fn pattern_selects(pattern: &str, text: &str) -> bool {
    // Xyce prints hierarchical nodes with ':', while RSpice's flattened
    // solution vectors use '.'. Treat the two separators as aliases at the
    // output-selection boundary so a deck's own .PRINT/.SAVE directives keep
    // selecting the canonical flattened signal.
    let pattern = canonical_hierarchy_separators(pattern);
    let text = canonical_hierarchy_separators(text);
    let pattern = pattern.as_ref();
    let text = text.as_ref();
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

fn canonical_hierarchy_separators(name: &str) -> std::borrow::Cow<'_, str> {
    if name.contains(':') {
        std::borrow::Cow::Owned(name.replace(':', "."))
    } else {
        std::borrow::Cow::Borrowed(name)
    }
}

#[cfg(test)]
mod save_set_tests {
    use super::*;

    #[test]
    fn hierarchical_colon_and_dot_names_select_each_other() {
        let saves = SaveSet {
            signals: vec![SaveSignal::Voltage("xtest:2".to_string())],
        };
        assert!(saves.selects("V(XTEST.2)"));
        assert!(saves.selects("V(XTEST:2)"));
    }

    #[test]
    fn hierarchical_wildcard_does_not_cross_colon_separator() {
        assert!(pattern_selects("x1:*", "x1:out"));
        assert!(!pattern_selects("x1:*", "x1:inner:out"));
    }

    #[test]
    fn frontend_save_overrides_apply_the_selected_ground_policy() {
        let mut saves = SaveSet {
            signals: vec![
                SaveSignal::Voltage("GND!".into()),
                SaveSignal::VoltageDiff("out".into(), "GROUND".into()),
                SaveSignal::Raw("V(GND)+V(GROUND)".into()),
            ],
        };

        saves.apply_ground_policy(super::super::GroundPolicy::XyceReplace);

        assert_eq!(saves.signals[0], SaveSignal::Voltage("0".into()));
        assert_eq!(
            saves.signals[1],
            SaveSignal::VoltageDiff("out".into(), "0".into())
        );
        assert_eq!(saves.signals[2], SaveSignal::Raw("V(0)+V(0)".into()));
    }
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
/// A5 %g out model             ; single-ended conductance terminal
/// A6 %gd[n+ n-] out model     ; differential conductance terminal
/// A7 %vnam vsrc out model     ; named voltage-source branch-current input
/// A8 %i out model             ; single-ended current terminal
/// A9 %h[winding] model        ; single-ended hybrid terminal
/// A10 %hd[core+ core-] model  ; differential hybrid terminal
/// A11 null out d_source       ; null = unconnected
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XspiceDigitalNode {
    pub name: String,
    pub inverted: bool,
}

impl XspiceDigitalNode {
    pub fn new(name: impl Into<String>, inverted: bool) -> Self {
        Self {
            name: name.into(),
            inverted,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum XspicePort {
    /// Single analog node (voltage or current)
    /// Syntax: `nodename`
    Analog(String),

    /// Single digital node (12-state logic)
    /// Syntax: `[nodename]`
    Digital(String),

    /// Explicitly typed single digital node.
    /// Syntax: `%d nodename`
    ExplicitDigital(String),

    /// Single inverted digital node
    /// Syntax: `[~nodename]`
    DigitalInverted(String),

    /// Vector of analog nodes
    /// Syntax: `(n1 n2 n3)` (uncommon)
    AnalogVector(Vec<String>),

    /// Vector of digital nodes
    /// Syntax: `[n1 n2 n3]`
    DigitalVector(Vec<String>),

    /// Vector of digital nodes with one or more inverted entries.
    /// Syntax: `[n1 ~n2 n3]`
    DigitalVectorMixed(Vec<XspiceDigitalNode>),

    /// Single-ended conductance terminal
    /// Syntax: `%g node` or `%g(node)`
    Conductance(String),

    /// Single-ended current terminal.
    ///
    /// On input ports, ngspice interprets `%i name` as a named branch current.
    /// On output ports, it interprets `name` as a node with ground as the
    /// reference terminal.
    /// Syntax: `%i name` or `%i(name)`
    Current(String),

    /// Named voltage-source current input
    /// Syntax: `%vnam vsource_name`
    VoltageName(String),

    /// Differential voltage input/output
    /// Syntax: `%vd[n+ n-]` or `%vd(n+ n-)`
    DifferentialVoltage { pos: String, neg: String },

    /// Differential current input/output
    /// Syntax: `%id[n+ n-]` or `%id(n+ n-)`
    DifferentialCurrent { pos: String, neg: String },

    /// Differential conductance terminal pair
    /// Syntax: `%gd[n+ n-]`, `%gd(n+ n-)`, or `%gd n+ n-`
    DifferentialConductance { pos: String, neg: String },

    /// Single-ended hybrid terminal.
    /// Syntax: `%h node` or `%h(node)`
    Hybrid(String),

    /// Differential hybrid terminal pair.
    /// Syntax: `%hd[n+ n-]`, `%hd(n+ n-)`, or `%hd n+ n-`
    DifferentialHybrid { pos: String, neg: String },

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

    /// Create an explicitly typed single digital port
    pub fn explicit_digital(node: impl Into<String>) -> Self {
        XspicePort::ExplicitDigital(node.into())
    }

    /// Create a single inverted digital port
    pub fn digital_inverted(node: impl Into<String>) -> Self {
        XspicePort::DigitalInverted(node.into())
    }

    /// Create a digital vector from node names
    pub fn digital_vector(nodes: Vec<String>) -> Self {
        XspicePort::DigitalVector(nodes)
    }

    /// Create a digital vector with explicit per-node inversion flags
    pub fn digital_vector_mixed(nodes: Vec<XspiceDigitalNode>) -> Self {
        XspicePort::DigitalVectorMixed(nodes)
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
                | XspicePort::Conductance(_)
                | XspicePort::Current(_)
                | XspicePort::VoltageName(_)
                | XspicePort::DifferentialVoltage { .. }
                | XspicePort::DifferentialCurrent { .. }
                | XspicePort::DifferentialConductance { .. }
                | XspicePort::Hybrid(_)
                | XspicePort::DifferentialHybrid { .. }
        )
    }

    /// Check if this is a digital port
    pub fn is_digital(&self) -> bool {
        matches!(
            self,
            XspicePort::Digital(_)
                | XspicePort::ExplicitDigital(_)
                | XspicePort::DigitalInverted(_)
                | XspicePort::DigitalVector(_)
                | XspicePort::DigitalVectorMixed(_)
        )
    }

    /// Check if this is a null connection
    pub fn is_null(&self) -> bool {
        matches!(self, XspicePort::Null)
    }

    /// Get all node names referenced by this port
    pub fn node_names(&self) -> Vec<&str> {
        match self {
            XspicePort::Analog(n)
            | XspicePort::Digital(n)
            | XspicePort::ExplicitDigital(n)
            | XspicePort::DigitalInverted(n)
            | XspicePort::Conductance(n)
            | XspicePort::Current(n)
            | XspicePort::Hybrid(n) => vec![n.as_str()],
            XspicePort::VoltageName(_) => vec![],
            XspicePort::AnalogVector(v) | XspicePort::DigitalVector(v) => {
                v.iter().map(|s| s.as_str()).collect()
            }
            XspicePort::DigitalVectorMixed(v) => v.iter().map(|node| node.name.as_str()).collect(),
            XspicePort::DifferentialVoltage { pos, neg }
            | XspicePort::DifferentialCurrent { pos, neg }
            | XspicePort::DifferentialConductance { pos, neg }
            | XspicePort::DifferentialHybrid { pos, neg } => {
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
    pub mode: DcSweepMode,
}

impl DcSecondSweep {
    pub fn linear(source: String, start: Value, stop: Value, step: Value) -> Self {
        Self {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
        }
    }

    pub fn spec(&self) -> DcSweepSpec {
        DcSweepSpec {
            start: self.start,
            stop: self.stop,
            step: self.step,
            mode: self.mode.clone(),
        }
    }
}

/// DC sweep value generation mode.
#[derive(Debug, Clone, PartialEq)]
pub enum DcSweepMode {
    /// Classic SPICE linear sweep: start, stop, increment.
    Linear,
    /// Explicit value list, in source order.
    List(Vec<Value>),
    /// Logarithmic decade sweep with N points per decade.
    Decade { points_per_decade: usize },
    /// Logarithmic octave sweep with N points per octave.
    Octave { points_per_octave: usize },
}

/// Value specification for one `.DC` sweep variable.
#[derive(Debug, Clone, PartialEq)]
pub struct DcSweepSpec {
    pub start: Value,
    pub stop: Value,
    pub step: Value,
    pub mode: DcSweepMode,
}

impl DcSweepSpec {
    pub fn linear(start: Value, stop: Value, step: Value) -> Self {
        Self {
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
        }
    }

    pub fn list(values: Vec<Value>) -> Self {
        let start = values.first().copied().unwrap_or(0.0);
        let stop = values.last().copied().unwrap_or(start);
        Self {
            start,
            stop,
            step: 0.0,
            mode: DcSweepMode::List(values),
        }
    }

    pub fn decade(start: Value, stop: Value, points_per_decade: usize) -> Self {
        Self {
            start,
            stop,
            step: points_per_decade as Value,
            mode: DcSweepMode::Decade { points_per_decade },
        }
    }

    pub fn octave(start: Value, stop: Value, points_per_octave: usize) -> Self {
        Self {
            start,
            stop,
            step: points_per_octave as Value,
            mode: DcSweepMode::Octave { points_per_octave },
        }
    }

    pub fn points(&self) -> Vec<Value> {
        if let DcSweepMode::List(values) = &self.mode {
            return if values.iter().all(|value| value.is_finite()) {
                values.clone()
            } else {
                Vec::new()
            };
        }
        self.points_controlled(2_000_000, false, &NoAbort)
            .unwrap_or_default()
    }

    pub(crate) fn points_bounded_with_abort(
        &self,
        max_points: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SweepPointGenerationError> {
        self.points_controlled(max_points, true, abort)
    }

    fn points_controlled(
        &self,
        max_points: usize,
        reject_limit: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SweepPointGenerationError> {
        match &self.mode {
            DcSweepMode::Linear => linear_sweep_points_controlled(
                self.start,
                self.stop,
                self.step,
                max_points,
                reject_limit,
                abort,
            ),
            DcSweepMode::List(values) => {
                copy_sweep_values_controlled(values, max_points, reject_limit, abort)
            }
            DcSweepMode::Decade { points_per_decade } => logarithmic_sweep_points_controlled(
                self.start,
                self.stop,
                *points_per_decade,
                10.0,
                max_points,
                reject_limit,
                abort,
            ),
            DcSweepMode::Octave { points_per_octave } => logarithmic_sweep_points_controlled(
                self.start,
                self.stop,
                *points_per_octave,
                2.0,
                max_points,
                reject_limit,
                abort,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SweepPointGenerationError {
    Aborted,
    LimitExceeded { requested: usize, limit: usize },
}

fn poll_sweep_abort(
    abort: &dyn AbortSignal,
    index: usize,
) -> Result<(), SweepPointGenerationError> {
    if index.is_multiple_of(64) && abort.is_aborted() {
        Err(SweepPointGenerationError::Aborted)
    } else {
        Ok(())
    }
}

fn copy_sweep_values_controlled(
    values: &[Value],
    max_values: usize,
    reject_limit: bool,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SweepPointGenerationError> {
    if reject_limit && values.len() > max_values {
        return Err(SweepPointGenerationError::LimitExceeded {
            requested: values.len(),
            limit: max_values,
        });
    }
    let retained = values.len().min(max_values);
    let mut result = Vec::with_capacity(retained);
    for (index, value) in values.iter().take(retained).copied().enumerate() {
        poll_sweep_abort(abort, index)?;
        if !value.is_finite() {
            return Ok(Vec::new());
        }
        result.push(value);
    }
    Ok(result)
}

fn linear_sweep_points_controlled(
    start: Value,
    stop: Value,
    step: Value,
    max_points: usize,
    reject_limit: bool,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SweepPointGenerationError> {
    if !start.is_finite() || !stop.is_finite() || !step.is_finite() || step == 0.0 {
        return Ok(Vec::new());
    }
    if (stop > start && step < 0.0) || (stop < start && step > 0.0) {
        return Ok(Vec::new());
    }

    let mut points = Vec::new();
    let eps = (step.abs() * 1e-9).max(1e-18);
    let mut point_index = 0usize;

    let done = |x: Value| -> bool {
        if step > 0.0 {
            x > stop + eps
        } else {
            x < stop - eps
        }
    };

    loop {
        poll_sweep_abort(abort, point_index)?;
        let value = start + step * point_index as Value;
        if done(value) {
            break;
        }

        let snapped_to_stop = (value - stop).abs() <= eps;
        if point_index >= max_points {
            if reject_limit {
                return Err(SweepPointGenerationError::LimitExceeded {
                    requested: point_index.saturating_add(1),
                    limit: max_points,
                });
            }
            break;
        }
        points.push(if snapped_to_stop { stop } else { value });
        point_index += 1;
        if snapped_to_stop {
            break;
        }
    }

    if points.is_empty() {
        points.push(start);
    }

    Ok(points)
}

fn logarithmic_sweep_points_controlled(
    start: Value,
    stop: Value,
    points_per_interval: usize,
    base: Value,
    max_points: usize,
    reject_limit: bool,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SweepPointGenerationError> {
    if !start.is_finite()
        || !stop.is_finite()
        || start <= 0.0
        || stop <= 0.0
        || points_per_interval == 0
    {
        return Ok(Vec::new());
    }

    if start > stop {
        return if max_points == 0 && reject_limit {
            Err(SweepPointGenerationError::LimitExceeded {
                requested: 1,
                limit: 0,
            })
        } else {
            Ok(vec![start])
        };
    }

    let multiplier = base.powf(1.0 / points_per_interval as Value);
    let span = if (base - 10.0).abs() <= Value::EPSILON {
        (stop.log10() - start.log10()).abs()
    } else {
        (stop.ln() - start.ln()).abs() / base.ln()
    };
    let raw_count = span * points_per_interval as Value + 1.0;
    let boundary_epsilon = 64.0 * Value::EPSILON * raw_count.abs().max(1.0);
    let requested = (raw_count + boundary_epsilon).floor() as usize;
    let requested = requested.max(1);
    if reject_limit && requested > max_points {
        return Err(SweepPointGenerationError::LimitExceeded {
            requested,
            limit: max_points,
        });
    }
    let count = requested.min(max_points);
    let mut points = Vec::with_capacity(count);
    for index in 0..count {
        poll_sweep_abort(abort, index)?;
        points.push(start * multiplier.powi(index as i32));
    }
    Ok(points)
}

//=============================================================================
// Source Specifications
//=============================================================================

/// RF port metadata attached to a voltage source for ngspice-compatible
/// S-parameter analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceRfPort {
    /// Port number as specified by `portnum`/`port`, 1-indexed.
    pub portnum: usize,
    /// Reference impedance in ohms. Ngspice defaults this to 50 ohms when
    /// `portnum` is present and `z0` is omitted.
    pub z0: Value,
    /// Optional RF power annotation used by ngspice's RF source path.
    pub power: Option<Value>,
    /// Optional RF source frequency annotation.
    pub frequency: Option<Value>,
    /// Optional RF source phase annotation in degrees.
    pub phase: Option<Value>,
}

/// One independent-source excitation used by small-signal Volterra
/// distortion analysis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SourceDistortionTone {
    /// Sinusoidal peak magnitude in volts or amperes.
    pub magnitude: Value,
    /// Sinusoidal phase in radians.
    pub phase: Value,
}

/// Source specification (DC, AC, or transient waveforms)
#[derive(Debug, Clone)]
pub enum SourceSpec {
    /// A source with one or both ngspice-compatible `DISTOF1`/`DISTOF2`
    /// small-signal distortion excitations. The wrapped source retains its
    /// independent DC, AC, transient, and RF-port behavior.
    Distortion {
        inner: Box<SourceSpec>,
        f1: Option<SourceDistortionTone>,
        f2: Option<SourceDistortionTone>,
    },

    /// A source with ngspice RF-port annotations. The wrapped source keeps
    /// ordinary DC/AC/transient electrical behavior.
    RfPort {
        inner: Box<SourceSpec>,
        port: SourceRfPort,
    },

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

    /// Pulse source: PULSE(v1 v2 td tr tf pw per phase)
    Pulse {
        v1: Value,
        v2: Value,
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
        /// XSPICE pulse phase, in degrees.
        phase: Value,
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

    /// Piecewise linear source: PWL(t1 v1 t2 v2 ...) [TD=delay] [R=repeat_time]
    Pwl {
        points: Vec<(Value, Value)>,
        /// Delay before the waveform starts. Before this time the source is zero.
        delay: Value,
        /// Optional source-time knot where repetition starts after the last
        /// point. `R=0` repeats the whole waveform.
        repeat_from: Option<Value>,
    },

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
        /// Delay before the waveform starts. Before this time the source is zero.
        delay: Value,
        /// Optional source-time knot where repetition starts after the last
        /// point. `R=0` repeats the whole waveform.
        repeat_from: Option<Value>,
    },

    /// Xyce/HSPICE digital pattern source: PAT(VHI VLO TD TR TF TSAMPLE DATA [R=n]).
    Pat {
        /// High output level.
        vhi: Value,
        /// Low output level.
        vlo: Value,
        /// Time delay. Xyce permits negative values.
        delay: Value,
        /// Rising edge duration.
        rise: Value,
        /// Falling edge duration.
        fall: Value,
        /// Bit sample interval.
        sample: Value,
        /// Pattern bits including the leading B marker, e.g. B1010.
        data: String,
        /// Number of repeated cycles after the first pattern. -1 repeats forever.
        repeat_count: i32,
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
    /// Return RF-port metadata when this source has ngspice `portnum`
    /// annotations.
    pub fn rf_port(&self) -> Option<&SourceRfPort> {
        match self {
            SourceSpec::RfPort { port, .. } => Some(port),
            SourceSpec::Distortion { inner, .. } => inner.rf_port(),
            _ => None,
        }
    }

    /// Return the `DISTOF1` excitation when one was explicitly present.
    pub fn distortion_f1(&self) -> Option<SourceDistortionTone> {
        match self {
            SourceSpec::Distortion { f1, .. } => *f1,
            SourceSpec::RfPort { inner, .. } => inner.distortion_f1(),
            _ => None,
        }
    }

    /// Return the `DISTOF2` excitation when one was explicitly present.
    pub fn distortion_f2(&self) -> Option<SourceDistortionTone> {
        match self {
            SourceSpec::Distortion { f2, .. } => *f2,
            SourceSpec::RfPort { inner, .. } => inner.distortion_f2(),
            _ => None,
        }
    }

    /// Return this specification with its AC excitation replaced by the
    /// given magnitude and phase, preserving DC and transient content.
    ///
    /// Pure waveform specs gain a `DcAcTransient` wrapper with a zero DC
    /// value, matching how SPICE treats `AC` annotations on such sources.
    pub fn with_ac(self, magnitude: Value, phase: Value) -> Self {
        match self {
            SourceSpec::Distortion { inner, f1, f2 } => SourceSpec::Distortion {
                inner: Box::new(inner.with_ac(magnitude, phase)),
                f1,
                f2,
            },
            SourceSpec::RfPort { inner, port } => SourceSpec::RfPort {
                inner: Box::new(inner.with_ac(magnitude, phase)),
                port,
            },
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
            SourceSpec::Distortion { inner, f1, f2 } => SourceSpec::Distortion {
                inner: Box::new(inner.with_dc_value(value)),
                f1,
                f2,
            },
            SourceSpec::RfPort { inner, port } => SourceSpec::RfPort {
                inner: Box::new(inner.with_dc_value(value)),
                port,
            },
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

/// Signal sampled by an HSPICE/Xyce-style `.FFT` post-processing directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FftOutput {
    /// A canonical probe such as `V(OUT)`, `V(P,N)`, or `I(V1)`.
    Probe(String),
    /// A braced expression evaluated at each transient sample.
    Expression(String),
}

/// Coefficient normalization selected by `.FFT FORMAT=`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FftFormat {
    /// Normalize coefficients by the largest magnitude, matching Xyce's default.
    #[default]
    Normalized,
    /// Preserve unnormalized coefficient magnitudes.
    Unnormalized,
}

/// Window function selected by `.FFT WINDOW=`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FftWindow {
    #[default]
    Rectangular,
    Bartlett,
    BartlettHann,
    Hamming,
    Hann,
    Blackman67Db,
    Blackman,
    BlackmanHarris,
    Nuttall,
    HalfCycleSine,
    HalfCycleSine3,
    HalfCycleSine6,
    Cosine2,
    Cosine4,
}

/// Typed `.FFT` post-processing request.
///
/// The directive is parsed and validated for every deck, but is active only
/// when transient analysis is selected. Xyce uses the same analysis gating.
#[derive(Debug, Clone, PartialEq)]
pub struct FftAnalysis {
    pub output: FftOutput,
    pub start: Option<Value>,
    pub stop: Option<Value>,
    pub points: usize,
    /// Explicit format. `None` preserves Xyce's `.OPTIONS FFT FFT_MODE`
    /// dependent default selection.
    pub format: Option<FftFormat>,
    pub window: FftWindow,
    /// Uppercase spelling retained for byte-exact Xyce-compatible headers.
    pub window_name: String,
    pub alpha: Value,
    pub fundamental_frequency: Option<Value>,
    pub minimum_frequency: Option<Value>,
    pub maximum_frequency: Option<Value>,
}

impl FftAnalysis {
    pub const DEFAULT_POINTS: usize = 1024;
    pub const DEFAULT_ALPHA: Value = 3.0;
}

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
        mode: DcSweepMode,
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

    /// Xyce table-driven AC analysis: `.AC DATA=<table>`, where the table
    /// supplies an explicit `FREQ` column.
    AcData { table_name: String },

    /// Harmonic-balance analysis: `.HB f1 [f2 ...]`.
    ///
    /// Frequencies are the independent large-signal tones. A single entry
    /// is ordinary one-tone HB; multiple entries use their common spectral
    /// basis in the engine.
    Hb { frequencies: Vec<Value> },

    /// S-parameter analysis: .SP DEC|LIN|OCT np fstart fstop [donoise]
    Sp {
        variation: FreqVariation,
        points: usize,
        start_freq: Value,
        stop_freq: Value,
        /// Ngspice RFSPICE accepts an optional integer noise flag. RSpice
        /// parses and carries it so decks round-trip even though the CLI
        /// currently exports the S-matrix only.
        do_noise: bool,
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

    /// Sensitivity analysis:
    /// `.SENS V(out[,ref])|I(vsource) [devspec ...] [AC DEC|LIN|OCT np fstart fstop]`
    Sensitivity {
        /// Output node for voltage probes, branch-owning element for current probes.
        output_node: String,
        /// Optional reference node for differential voltage probes.
        reference_node: Option<String>,
        /// True for an `I(element)` output probe.
        output_is_current: bool,
        /// Optional case-insensitive device/parameter glob filters.
        filters: Vec<String>,
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

/// Named table captured from a `.DATA ... .ENDDATA` block.
#[derive(Debug, Clone)]
pub struct DataTable {
    /// Table name referenced by `DATA=<name>`.
    pub name: String,
    /// Parameter columns updated simultaneously for each table row.
    pub params: Vec<String>,
    /// Row-major numeric values. Each row length must match `params.len()`.
    pub rows: Vec<Vec<Value>>,
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
    /// Row-wise parameter table reference: DATA=<table-name>
    Data { table_name: String },
}

impl StepSweep {
    pub fn values(&self) -> Vec<Value> {
        match self {
            StepSweep::Linear { start, stop, step } => {
                DcSweepSpec::linear(*start, *stop, *step).points()
            }
            StepSweep::Decade {
                points_per_decade,
                start,
                stop,
            } => DcSweepSpec::decade(*start, *stop, *points_per_decade).points(),
            StepSweep::Octave {
                points_per_octave,
                start,
                stop,
            } => DcSweepSpec::octave(*start, *stop, *points_per_octave).points(),
            StepSweep::List(values) => {
                if values.iter().all(|value| value.is_finite()) {
                    values.clone()
                } else {
                    Vec::new()
                }
            }
            StepSweep::Data { .. } => Vec::new(),
        }
    }

    pub(crate) fn values_bounded_with_abort(
        &self,
        max_values: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SweepPointGenerationError> {
        match self {
            StepSweep::Linear { start, stop, step } => DcSweepSpec::linear(*start, *stop, *step)
                .points_bounded_with_abort(max_values, abort),
            StepSweep::Decade {
                points_per_decade,
                start,
                stop,
            } => DcSweepSpec::decade(*start, *stop, *points_per_decade)
                .points_bounded_with_abort(max_values, abort),
            StepSweep::Octave {
                points_per_octave,
                start,
                stop,
            } => DcSweepSpec::octave(*start, *stop, *points_per_octave)
                .points_bounded_with_abort(max_values, abort),
            StepSweep::List(values) => {
                copy_sweep_values_controlled(values, max_values, true, abort)
            }
            StepSweep::Data { .. } => Ok(Vec::new()),
        }
    }
}

#[cfg(test)]
mod controlled_step_sweep_tests {
    use super::*;

    #[test]
    fn bounded_step_grids_exactly_match_compatibility_generation() {
        let sweeps = [
            StepSweep::Linear {
                start: -1.0,
                stop: 1.0,
                step: 0.25,
            },
            StepSweep::Decade {
                points_per_decade: 5,
                start: 1.0,
                stop: 10_001.0,
            },
            StepSweep::Octave {
                points_per_octave: 3,
                start: 1.0,
                stop: 8.0,
            },
            StepSweep::List(vec![3.0, 1.0, 4.0, 1.5]),
        ];

        for sweep in sweeps {
            let expected = sweep.values();
            let bounded = sweep
                .values_bounded_with_abort(expected.len(), &NoAbort)
                .expect("exact compatibility grid fits its bound");
            assert_eq!(bounded, expected, "grid changed for {sweep:?}");

            let error = sweep
                .values_bounded_with_abort(expected.len() - 1, &NoAbort)
                .expect_err("too-small bound must reject the whole grid");
            assert!(matches!(
                error,
                SweepPointGenerationError::LimitExceeded { .. }
            ));
        }

        let decade = StepSweep::Decade {
            points_per_decade: 5,
            start: 1.0,
            stop: 10_001.0,
        }
        .values();
        assert_eq!(decade.len(), 21);
        assert!((decade[20] - 10_000.0).abs() <= 1.0e-10);
    }

    #[test]
    fn list_compatibility_is_unbounded_while_controlled_generation_rejects() {
        let sweep = DcSweepSpec::list(vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(sweep.points(), vec![1.0, 2.0, 3.0, 4.0]);
        assert!(matches!(
            sweep.points_bounded_with_abort(3, &NoAbort),
            Err(SweepPointGenerationError::LimitExceeded {
                requested: 4,
                limit: 3
            })
        ));
    }

    #[test]
    fn bounded_step_generation_honors_delayed_cancellation() {
        let sweep = StepSweep::List((0..128).map(|value| value as Value).collect());
        let abort = crate::abort_signal::CountingAbort::new(1);
        assert!(matches!(
            sweep.values_bounded_with_abort(128, &abort),
            Err(SweepPointGenerationError::Aborted)
        ));
        assert!(abort.count() >= 2);
    }

    #[test]
    fn bounded_log_grid_honors_the_callers_resource_ceiling() {
        let sweep = StepSweep::Decade {
            points_per_decade: 2_000_001,
            start: 1.0,
            stop: 10.0,
        };
        let values = sweep
            .values_bounded_with_abort(2_000_002, &NoAbort)
            .expect("explicit ceiling above the default permits the requested grid");
        assert_eq!(values.len(), 2_000_002);
    }
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

/// Startup directive that supplied a node-voltage seed or hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartupDirectiveKind {
    Ic,
    NodeSet,
}

impl StartupDirectiveKind {
    pub fn as_spice_directive(self) -> &'static str {
        match self {
            Self::Ic => ".IC",
            Self::NodeSet => ".NODESET",
        }
    }
}

/// Semantic stage at which a startup diagnostic is established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartupDiagnosticStage {
    Parse,
    StartupTopology,
}

/// Stable, typed startup-directive diagnostic code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartupDiagnosticCode {
    EmptyDirective,
    UndefinedNode,
    ScopedGlobalNode,
}

impl StartupDiagnosticCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyDirective => "startup-empty-directive",
            Self::UndefinedNode => "startup-undefined-node",
            Self::ScopedGlobalNode => "startup-scoped-global-node",
        }
    }

    pub fn stage(self) -> StartupDiagnosticStage {
        match self {
            Self::EmptyDirective | Self::ScopedGlobalNode => StartupDiagnosticStage::Parse,
            Self::UndefinedNode => StartupDiagnosticStage::StartupTopology,
        }
    }
}

/// Source scope that owns a startup directive card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupDirectiveScope {
    TopLevel,
    Subcircuit {
        /// Qualified definition name (nested definitions use `PARENT:CHILD`).
        qualified_definition: String,
        /// Deterministic concrete instance paths after hierarchy expansion.
        qualified_instances: Vec<String>,
    },
}

/// Whether a startup card or one of its entries affects simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartupDirectiveDisposition {
    Applied,
    PartiallyApplied,
    Ignored(StartupDiagnosticCode),
}

/// One ordered `V(node)=value` assignment retained for diagnostics.
#[derive(Debug, Clone, PartialEq)]
pub struct StartupDirectiveEntry {
    /// Node spelling exactly as parsed from the authored card.
    pub(crate) authored_node: String,
    /// Parser-normalized node spelling used by the existing numeric startup
    /// representation. Kept distinct so diagnostics never alter execution.
    pub(crate) execution_node: String,
    /// Case- and hierarchy-separator-normalized node identity.
    pub(crate) canonical_node: String,
    /// Concrete execution node after hierarchy expansion, when applicable.
    pub(crate) qualified_nodes: Vec<String>,
    pub(crate) disposition: StartupDirectiveDisposition,
    /// Immutable parsed payload used to rebuild the effective execution
    /// vectors transactionally during semantic revalidation.
    pub(crate) voltage: Value,
    pub(crate) voltage_expr: Option<String>,
}

impl StartupDirectiveEntry {
    pub fn authored_node(&self) -> &str {
        &self.authored_node
    }

    pub fn execution_node(&self) -> &str {
        &self.execution_node
    }

    pub fn canonical_node(&self) -> &str {
        &self.canonical_node
    }

    pub fn qualified_nodes(&self) -> &[String] {
        &self.qualified_nodes
    }

    pub fn disposition(&self) -> StartupDirectiveDisposition {
        self.disposition
    }

    pub fn voltage(&self) -> Value {
        self.voltage
    }

    pub fn voltage_expression(&self) -> Option<&str> {
        self.voltage_expr.as_deref()
    }
}

/// One physical `.IC` or `.NODESET` card with its ordered assignments.
///
/// Empty cards are retained with an empty `entries` vector. This sidecar owns
/// diagnostic provenance only; execution continues to use
/// [`InitialCondition`] and [`NodeSet`].
#[derive(Debug, Clone, PartialEq)]
pub struct StartupDirectiveRecord {
    pub(crate) kind: StartupDirectiveKind,
    pub(crate) origin: super::NetlistSourceLocation,
    pub(crate) scope: StartupDirectiveScope,
    pub(crate) entries: Vec<StartupDirectiveEntry>,
    pub(crate) disposition: StartupDirectiveDisposition,
}

impl StartupDirectiveRecord {
    pub fn kind(&self) -> StartupDirectiveKind {
        self.kind
    }

    pub fn origin(&self) -> &super::NetlistSourceLocation {
        &self.origin
    }

    pub fn scope(&self) -> &StartupDirectiveScope {
        &self.scope
    }

    pub fn entries(&self) -> &[StartupDirectiveEntry] {
        &self.entries
    }

    pub fn disposition(&self) -> StartupDirectiveDisposition {
        self.disposition
    }
}

/// Typed warning projection derived deterministically from startup records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupDiagnostic {
    pub code: StartupDiagnosticCode,
    pub stage: StartupDiagnosticStage,
    pub kind: StartupDirectiveKind,
    pub origins: Vec<super::NetlistSourceLocation>,
    pub scopes: Vec<StartupDirectiveScope>,
    /// Canonical affected nodes, sorted and deduplicated. Empty-directive
    /// warnings naturally retain an empty vector.
    pub canonical_nodes: Vec<String>,
}

/// Initial condition specification
#[derive(Debug, Clone)]
pub struct InitialCondition {
    /// Node name
    pub node: String,
    /// Initial voltage
    pub voltage: Value,
    /// Deferred voltage expression for subcircuit-scoped startup directives.
    pub voltage_expr: Option<String>,
}

/// One device-targeted initial-condition override from Xyce's `.INITCOND`
/// directive.
///
/// Device names are retained exactly as authored for diagnostics. Matching is
/// case-insensitive and treats Xyce's `:` hierarchy separator as equivalent to
/// RSpice's canonical `.` separator when the hierarchy is flattened.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceInitialConditionEntry {
    /// Fully qualified device name, for example `XINV1:MN1`.
    pub device: String,
    /// Device-specific `IC=` vector in authored order.
    pub values: Vec<Value>,
    /// Exact source record that supplied this entry.
    pub origin: super::NetlistSourceLocation,
}

/// Origin of a device-targeted `.INITCOND` data set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceInitialConditionSource {
    /// Values were authored directly on the `.INITCOND` card.
    Inline,
    /// Values were loaded from an external text resource.
    File {
        /// Path exactly as authored on the `.INITCOND FILE` card.
        requested_path: String,
        /// Canonical provider identity after successful resolution.
        resolved_path: Option<std::path::PathBuf>,
        /// BLAKE3 identity of the exact decoded source text consumed by the
        /// parser.
        content_identity: Option<String>,
    },
}

/// Typed representation of one netlist-wide `.INITCOND` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceInitialConditionDirective {
    /// Exact source location of the directive itself.
    pub origin: super::NetlistSourceLocation,
    /// Inline or external-file provenance.
    pub source: DeviceInitialConditionSource,
    /// Resolved entries. External-file directives remain empty until a source
    /// provider resolves them.
    pub entries: Vec<DeviceInitialConditionEntry>,
}

impl DeviceInitialConditionDirective {
    /// Whether an external-file directive still requires source resolution.
    pub fn requires_source_resolution(&self) -> bool {
        matches!(
            self.source,
            DeviceInitialConditionSource::File {
                resolved_path: None,
                ..
            }
        )
    }
}

/// Nodeset hint for operating point
#[derive(Debug, Clone)]
pub struct NodeSet {
    /// Node name
    pub node: String,
    /// Suggested voltage
    pub voltage: Value,
    /// Deferred voltage expression for subcircuit-scoped startup directives.
    pub voltage_expr: Option<String>,
}

//=============================================================================
// Simulation Options
//=============================================================================

/// Device designator selected by Xyce `.PREPROCESS REMOVEUNUSED`.
///
/// The variants are intentionally limited to the eight device families Xyce
/// 7.10 accepts. Keeping this typed prevents an unknown designator from
/// becoming an inert string that silently changes preprocessing behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RemoveUnusedDeviceType {
    Capacitor,
    Diode,
    CurrentSource,
    Inductor,
    Mosfet,
    Bjt,
    Resistor,
    VoltageSource,
}

impl RemoveUnusedDeviceType {
    /// Parse one canonical one-letter Xyce selector.
    pub fn from_xyce_selector(selector: &str) -> Option<Self> {
        match selector.to_ascii_uppercase().as_str() {
            "C" => Some(Self::Capacitor),
            "D" => Some(Self::Diode),
            "I" => Some(Self::CurrentSource),
            "L" => Some(Self::Inductor),
            "M" => Some(Self::Mosfet),
            "Q" => Some(Self::Bjt),
            "R" => Some(Self::Resistor),
            "V" => Some(Self::VoltageSource),
            _ => None,
        }
    }

    /// Canonical Xyce spelling used in diagnostics and serialization.
    pub fn xyce_selector(self) -> &'static str {
        match self {
            Self::Capacitor => "C",
            Self::Diode => "D",
            Self::CurrentSource => "I",
            Self::Inductor => "L",
            Self::Mosfet => "M",
            Self::Bjt => "Q",
            Self::Resistor => "R",
            Self::VoltageSource => "V",
        }
    }
}

/// Normalized device-family selection for `.PREPROCESS REMOVEUNUSED`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RemoveUnusedPolicy {
    /// Selected families. A set makes repeated authored selectors idempotent
    /// and keeps semantic/checkpoint identity deterministic.
    pub device_types: std::collections::BTreeSet<RemoveUnusedDeviceType>,
}

/// Xyce `.PREPROCESS ADDRESISTORS` connectivity category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum XyceAddResistorMode {
    OneTerminal,
    NoDcPath,
}

impl XyceAddResistorMode {
    pub fn xyce_keyword(self) -> &'static str {
        match self {
            Self::OneTerminal => "ONETERMINAL",
            Self::NoDcPath => "NODCPATH",
        }
    }
}

/// One root-authored ADDRESISTORS mode and its unevaluated resistance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XyceAddResistorSpec {
    /// Exact logical token retained from the root netlist.
    pub raw_resistance: String,
    /// Root physical line that selected this mode.
    pub source_line: usize,
}

/// Typed root-wide Xyce ADDRESISTORS selection.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct XyceAddResistorsPolicy {
    pub one_terminal: Option<XyceAddResistorSpec>,
    pub no_dc_path: Option<XyceAddResistorSpec>,
}

impl XyceAddResistorsPolicy {
    pub fn spec(&self, mode: XyceAddResistorMode) -> Option<&XyceAddResistorSpec> {
        match mode {
            XyceAddResistorMode::OneTerminal => self.one_terminal.as_ref(),
            XyceAddResistorMode::NoDcPath => self.no_dc_path.as_ref(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.one_terminal.is_none() && self.no_dc_path.is_none()
    }
}

impl RemoveUnusedPolicy {
    pub fn contains(&self, device_type: RemoveUnusedDeviceType) -> bool {
        self.device_types.contains(&device_type)
    }

    pub fn is_empty(&self) -> bool {
        self.device_types.is_empty()
    }
}

/// Ngspice-style XSPICE auto-bridge template from `set auto_bridge_* = (...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XspiceAutoBridgeTemplate {
    pub key: String,
    pub setup_card: String,
    pub device_card: String,
    pub max_nodes: Option<usize>,
}

/// Ngspice-style XSPICE auto-bridge parameter selector from
/// `set auto_bridge_parm_* = <param>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XspiceAutoBridgeParamName {
    pub node_type: String,
    pub param_name: String,
}

/// Reference magnitude used to normalize transient local-truncation error.
///
/// Four policies correspond exactly to Xyce's `.OPTIONS TIMEINT NEWLTE`
/// selectors; `PredictorLocal` preserves RSpice's non-Xyce adaptive default.
/// Historical modes retain only accepted solution magnitudes, so a rejected
/// candidate cannot relax future error weights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransientLteReference {
    /// RSpice legacy policy: scale by the larger candidate/predictor magnitude.
    #[default]
    PredictorLocal,
    /// Scale each state by its magnitude at the prior accepted point (`NEWLTE=0`).
    PointLocal,
    /// Scale every state by the prior accepted point's infinity norm (`NEWLTE=1`).
    PointGlobal,
    /// Scale every state by the largest magnitude seen over the transient (`NEWLTE=2`).
    SignalGlobal,
    /// Scale each state by its own largest magnitude seen over the transient (`NEWLTE=3`).
    SignalLocal,
}

impl TransientLteReference {
    /// Convert a Xyce `NEWLTE` selector to its reference policy.
    pub fn from_xyce_selector(selector: u8) -> Option<Self> {
        match selector {
            0 => Some(Self::PointLocal),
            1 => Some(Self::PointGlobal),
            2 => Some(Self::SignalGlobal),
            3 => Some(Self::SignalLocal),
            _ => None,
        }
    }

    /// Return the corresponding Xyce `NEWLTE` selector.
    pub fn xyce_selector(self) -> Option<u8> {
        match self {
            Self::PredictorLocal => None,
            Self::PointLocal => Some(0),
            Self::PointGlobal => Some(1),
            Self::SignalGlobal => Some(2),
            Self::SignalLocal => Some(3),
        }
    }
}

/// Xyce nonlinear solve/continuation policy selected by
/// `.OPTIONS NONLIN CONTINUATION=...`.
///
/// The variants preserve the canonical Xyce selectors so configuration
/// resolution never silently turns an unsupported continuation algorithm into
/// a different solve. Execution sites must explicitly match the modes they
/// implement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonlinearContinuationMode {
    Standard,
    Natural,
    Mosfet,
    Gmin,
    PseudoTransient,
    SimultaneousSourceStep,
    SequentialSourceStep,
}

impl NonlinearContinuationMode {
    /// Canonical numeric selector accepted by Xyce 7.10.
    pub fn xyce_selector(self) -> i64 {
        match self {
            Self::Standard => 0,
            Self::Natural => 1,
            Self::Mosfet => 2,
            Self::Gmin => 3,
            Self::PseudoTransient => 9,
            Self::SimultaneousSourceStep => 34,
            Self::SequentialSourceStep => 35,
        }
    }

    pub fn from_xyce_selector(selector: i64) -> Option<Self> {
        match selector {
            0 => Some(Self::Standard),
            1 => Some(Self::Natural),
            2 => Some(Self::Mosfet),
            3 => Some(Self::Gmin),
            9 => Some(Self::PseudoTransient),
            34 => Some(Self::SimultaneousSourceStep),
            35 => Some(Self::SequentialSourceStep),
            _ => None,
        }
    }
}

/// Simulation options from .OPTIONS command
///
/// Controls numerical parameters for simulation accuracy and convergence.
/// All fields are optional - unspecified values use engine defaults.
#[derive(Debug, Clone, Default)]
pub struct SimulationOptions {
    /// Xyce `.PREPROCESS REPLACEGROUND TRUE|FALSE`. When enabled, the exact
    /// case-insensitive fields `GND`, `GND!`, and `GROUND` are node-zero
    /// aliases throughout circuit elaboration and output expressions.
    pub replace_ground: Option<bool>,
    /// Xyce `.PREPROCESS REMOVEUNUSED` device-family selection. `None` is the
    /// semantic default and leaves every authored device in the circuit.
    pub remove_unused: Option<RemoveUnusedPolicy>,
    /// Xyce `.PREPROCESS ADDRESISTORS` generation policy. Parsing records the
    /// request but never mutates the first-run circuit; callers opt into the
    /// generated semantic copy through `Netlist::materialize_xyce_add_resistors`.
    pub add_resistors: Option<XyceAddResistorsPolicy>,
    /// Xyce `.OPTIONS MEASURE MEASFAIL`: emit `FAILED` rather than the
    /// calculation default value in machine-readable measurement files.
    /// Xyce defaults this to enabled.
    pub measure_fail_output: Option<bool>,
    /// Xyce `.OPTIONS MEASURE DEFAULT_VAL`: global measurement initialization
    /// and failure value. It overrides a per-equation default and defaults to
    /// zero when `MEASFAIL=0` serializes an unevaluated measurement.
    pub measure_default_value: Option<Value>,
    /// Xyce `.OPTIONS MEASURE USE_CONT_FILES`: route continuous measurement
    /// records to per-measure files instead of the aggregate measurement
    /// file. Xyce enables this by default.
    pub measure_use_cont_files: Option<bool>,
    /// Xyce `.OPTIONS HBINT NUMFREQ[<n>]=...` harmonic orders.
    /// Each order produces a bilateral `2*N+1` collocation grid.
    pub hb_num_frequencies: Vec<usize>,
    /// Explicit Xyce nonlinear continuation policy.
    pub nonlinear_continuation: Option<NonlinearContinuationMode>,
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
    /// Xyce `.OPTIONS TIMEINT RELTOL` for transient LTE weighting.
    pub timeint_reltol: Option<Value>,
    /// Xyce `.OPTIONS TIMEINT ABSTOL` for transient LTE weighting.
    pub timeint_abstol: Option<Value>,
    /// Xyce `.OPTIONS TIMEINT DELMAX` transient timestep ceiling.
    pub timeint_delmax: Option<Value>,
    /// Xyce `.OPTIONS TIMEINT USEDEVICEMAX` device-provided timestep policy.
    /// Xyce enables this policy by default.
    pub timeint_use_device_max_timestep: Option<bool>,
    /// Xyce `.OPTIONS NONLIN-TRAN RELTOL` for transient Newton update weights.
    pub nonlin_transient_reltol: Option<Value>,
    /// Xyce `.OPTIONS NONLIN-TRAN ABSTOL` for transient Newton update weights.
    pub nonlin_transient_abstol: Option<Value>,
    /// Xyce `.OPTIONS NONLIN-TRAN DELTAXTOL` normalized update threshold.
    pub nonlin_transient_deltaxtol: Option<Value>,
    /// Xyce `.OPTIONS NONLIN-TRAN RHSTOL` raw nonlinear-residual threshold.
    pub nonlin_transient_rhstol: Option<Value>,
    /// Xyce `.OPTIONS NONLIN-TRAN MAXSTEP` nonlinear iteration budget.
    pub nonlin_transient_maxstep: Option<usize>,
    /// Xyce `.OPTIONS NONLIN-TRAN ENFORCEDEVICECONV` device-local
    /// convergence policy.
    pub nonlin_transient_enforce_device_convergence: Option<bool>,
    /// Xyce transient LTE error-weight reference policy (`NEWLTE=0..3`).
    pub transient_lte_reference: Option<TransientLteReference>,
    /// Xyce breakpoint-step LTE policy (`NEWBPSTEPPING=0|1`).
    ///
    /// When disabled, the first integration step after a breakpoint is
    /// accepted based on Newton convergence without testing its local
    /// truncation error. Xyce enables the newer policy by default.
    pub transient_new_bp_stepping: Option<bool>,
    /// Transient source/code-model ramping time in seconds (default: disabled)
    pub ramptime: Option<Value>,
    /// Ngspice XSPICE digital delay policy:
    /// 0 = default transport, 1 = default inertial,
    /// 2 = force transport, 3 = force inertial.
    pub digital_delay_type: Option<i64>,
    /// Ngspice `esave` event-output policy for XSPICE digital traces.
    /// `None` keeps the default of recording event traces.
    pub xspice_event_trace_save: Option<bool>,
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
    /// Ngspice-compatible XSPICE automatic analog/digital bridge insertion.
    /// Enabled by default; set `.options auto_bridge=0` to disable.
    pub auto_bridge: Option<bool>,
    /// Ngspice-compatible `.options auto_bridge=2` generated bridge diagnostics.
    pub auto_bridge_show_generated: Option<bool>,
    /// Ngspice-compatible `set no_auto_bridge_family` control variable.
    /// Enabled by default; set to false to skip family-specific bridge lookup.
    pub auto_bridge_family: Option<bool>,
    /// Ngspice-compatible XSPICE custom automatic bridge templates promoted
    /// from `.control` `set auto_bridge_* = (...)` variables.
    pub auto_bridge_templates: Vec<XspiceAutoBridgeTemplate>,
    /// Ngspice-compatible custom parameter names used to derive generated
    /// auto-bridge threshold/output levels, keyed by XSPICE node type.
    pub auto_bridge_param_names: Vec<XspiceAutoBridgeParamName>,
    /// Xyce `.options topology supernode=...`: collapse nodes connected by
    /// explicit zero/near-zero resistors before device construction.
    pub topology_supernode: Option<bool>,
    /// Xyce `.options device zeroresistancetol=...`: resistance threshold
    /// used by topology supernode reduction.
    pub device_zero_resistance_tol: Option<Value>,
    /// Xyce `.options device b3soigminscaling=...`: when enabled, BSIMSOI3
    /// receives `GMIN * 1e-6` in its terminal GMIN branches. Xyce enables this
    /// by default and decks may set it to zero to request the full GMIN.
    pub b3soi_gmin_scaling: Option<bool>,
}

impl SimulationOptions {
    /// Create empty options (all defaults)
    pub fn new() -> Self {
        Self::default()
    }

    /// Effective Xyce continuous-measurement file-routing policy.
    pub fn measure_use_cont_files(&self) -> bool {
        self.measure_use_cont_files.unwrap_or(true)
    }

    /// Merge another options set, preferring values from `other`
    pub fn merge(&mut self, other: &SimulationOptions) {
        if other.replace_ground.is_some() {
            self.replace_ground = other.replace_ground;
        }
        if other.remove_unused.is_some() {
            self.remove_unused = other.remove_unused.clone();
        }
        if other.add_resistors.is_some() {
            self.add_resistors = other.add_resistors.clone();
        }
        if other.measure_fail_output.is_some() {
            self.measure_fail_output = other.measure_fail_output;
        }
        if other.measure_default_value.is_some() {
            self.measure_default_value = other.measure_default_value;
        }
        if other.measure_use_cont_files.is_some() {
            self.measure_use_cont_files = other.measure_use_cont_files;
        }
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
        if other.timeint_reltol.is_some() {
            self.timeint_reltol = other.timeint_reltol;
        }
        if other.timeint_abstol.is_some() {
            self.timeint_abstol = other.timeint_abstol;
        }
        if other.timeint_delmax.is_some() {
            self.timeint_delmax = other.timeint_delmax;
        }
        if other.timeint_use_device_max_timestep.is_some() {
            self.timeint_use_device_max_timestep = other.timeint_use_device_max_timestep;
        }
        if other.nonlin_transient_reltol.is_some() {
            self.nonlin_transient_reltol = other.nonlin_transient_reltol;
        }
        if other.nonlin_transient_abstol.is_some() {
            self.nonlin_transient_abstol = other.nonlin_transient_abstol;
        }
        if other.nonlin_transient_deltaxtol.is_some() {
            self.nonlin_transient_deltaxtol = other.nonlin_transient_deltaxtol;
        }
        if other.nonlin_transient_rhstol.is_some() {
            self.nonlin_transient_rhstol = other.nonlin_transient_rhstol;
        }
        if other.nonlin_transient_maxstep.is_some() {
            self.nonlin_transient_maxstep = other.nonlin_transient_maxstep;
        }
        if other.nonlin_transient_enforce_device_convergence.is_some() {
            self.nonlin_transient_enforce_device_convergence =
                other.nonlin_transient_enforce_device_convergence;
        }
        if other.transient_lte_reference.is_some() {
            self.transient_lte_reference = other.transient_lte_reference;
        }
        if other.transient_new_bp_stepping.is_some() {
            self.transient_new_bp_stepping = other.transient_new_bp_stepping;
        }
        if other.ramptime.is_some() {
            self.ramptime = other.ramptime;
        }
        if other.digital_delay_type.is_some() {
            self.digital_delay_type = other.digital_delay_type;
        }
        if other.xspice_event_trace_save.is_some() {
            self.xspice_event_trace_save = other.xspice_event_trace_save;
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
        if other.auto_bridge.is_some() {
            self.auto_bridge = other.auto_bridge;
        }
        if other.auto_bridge_show_generated.is_some() {
            self.auto_bridge_show_generated = other.auto_bridge_show_generated;
        }
        if other.auto_bridge_family.is_some() {
            self.auto_bridge_family = other.auto_bridge_family;
        }
        for template in &other.auto_bridge_templates {
            self.set_auto_bridge_template(template.clone());
        }
        for param_name in &other.auto_bridge_param_names {
            self.set_auto_bridge_param_name(param_name.clone());
        }
        if other.topology_supernode.is_some() {
            self.topology_supernode = other.topology_supernode;
        }
        if other.device_zero_resistance_tol.is_some() {
            self.device_zero_resistance_tol = other.device_zero_resistance_tol;
        }
        if other.b3soi_gmin_scaling.is_some() {
            self.b3soi_gmin_scaling = other.b3soi_gmin_scaling;
        }
        if other.nonlinear_continuation.is_some() {
            self.nonlinear_continuation = other.nonlinear_continuation;
        }
        if !other.hb_num_frequencies.is_empty() {
            self.hb_num_frequencies = other.hb_num_frequencies.clone();
        }
    }

    pub fn set_auto_bridge_template(&mut self, template: XspiceAutoBridgeTemplate) {
        self.auto_bridge_templates
            .retain(|existing| !existing.key.eq_ignore_ascii_case(&template.key));
        self.auto_bridge_templates.push(template);
    }

    pub fn set_auto_bridge_param_name(&mut self, param_name: XspiceAutoBridgeParamName) {
        self.auto_bridge_param_names.retain(|existing| {
            !existing
                .node_type
                .eq_ignore_ascii_case(&param_name.node_type)
        });
        self.auto_bridge_param_names.push(param_name);
    }

    pub fn auto_bridge_param_name(&self, node_type: &str) -> Option<&str> {
        self.auto_bridge_param_names
            .iter()
            .rev()
            .find(|param_name| param_name.node_type.eq_ignore_ascii_case(node_type))
            .map(|param_name| param_name.param_name.as_str())
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
    pub string_vector_params: Vec<(String, Vec<String>)>,
    pub real_vector_params: Vec<(String, Vec<Value>)>,
    pub real_vector_expr_params: Vec<(String, Vec<String>)>,
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
    /// Initial conditions declared inside the subcircuit body.
    pub initial_conditions: Vec<InitialCondition>,
    /// Nodeset hints declared inside the subcircuit body.
    pub node_sets: Vec<NodeSet>,
    /// Default parameter values (can be overridden at instance)
    pub params: Vec<(String, Value)>,
    /// Default numeric parameter expressions resolved when the subcircuit is instantiated.
    pub expr_params: Vec<(String, String)>,
    /// Default string parameter values (can be overridden at instance)
    pub string_params: Vec<(String, String)>,
    /// Parameter definitions declared inside the subcircuit body with `.PARAM`.
    pub body_params: Vec<(String, Value)>,
    /// Deferred numeric `.PARAM` expressions declared inside the subcircuit body.
    pub body_expr_params: Vec<(String, String)>,
    /// String parameter definitions declared inside the subcircuit body.
    pub body_string_params: Vec<(String, String)>,
    /// User-defined functions declared inside the subcircuit body with `.FUNC`.
    pub body_functions: Vec<FunctionDef>,
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
