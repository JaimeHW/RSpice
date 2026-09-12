//! Parameter scope and evaluation policy.
//!
//! [`ParamContext`] resolves parameter names, holds user-defined
//! [`FunctionDef`]s, and carries the settings that change how expressions
//! evaluate: the [`crate::config::ExpressionDialect`], the
//! [`ParameterRedefinitionPolicy`] applied when a name is defined twice,
//! [`ParameterRedefinitionDiagnosticPolicy`] controlling whether that event is
//! silent, warned, or fatal, and
//! the [`StatisticalParamMode`] governing `GAUSS`/`UNIF`/`RAND`.
//!
//! Those generators draw from a seeded [`RandomState`] rather than thread
//! RNG, so a Monte Carlo or corner run reproduces exactly given the same
//! seed ([`DEFAULT_RANDOM_SEED`] unless overridden).

use super::*;
use crate::config::ExpressionDialect;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

//=============================================================================
// Expression Evaluator
//=============================================================================

/// Default seed for the statistical expression functions.
///
/// A fixed default keeps every run reproducible out of the box; decks that
/// want a different stream set `.options seed=<n>`.
pub const DEFAULT_RANDOM_SEED: u64 = 1;

const DEFAULT_TEMPERATURE_C: Value = 27.0;
const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// Finalizer from SplitMix64 (Steele, Lea, Flood 2014).
#[inline]
fn splitmix64_mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Deterministic random stream backing the statistical expression functions
/// (`gauss`/`agauss`/`unif`/`aunif`/2-argument `limit`).
///
/// Each draw is a pure function of `(seed, draw_index)` via counter-based
/// SplitMix64, so a given seed reproduces the identical sequence on every
/// platform and run. Clones share the underlying draw counter: every context
/// derived from one netlist pulls from a single netlist-wide sequence, which
/// is what gives per-instance mismatch draws (each device evaluation advances
/// the stream) while staying reproducible under the deterministic
/// parse/flatten order. Reseeding installs a fresh, decoupled stream —
/// Monte-Carlo drivers give each run its own seed to obtain independent,
/// individually reproducible streams.
#[derive(Debug, Clone)]
pub struct RandomState {
    seed: u64,
    counter: Arc<AtomicU64>,
}

impl RandomState {
    /// Create a stream for the given seed, positioned at the first draw.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// The seed this stream was created with.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Rewind the stream to draw zero, keeping the seed.
    ///
    /// Elaborating a netlist consumes draws, so a second elaboration of the
    /// *same* netlist would otherwise continue the sequence and produce a
    /// different circuit — the same deck simulated twice in one process would
    /// disagree with itself. Rewinding at each build scopes the sequence to
    /// one elaboration, which is also what makes several analyses in one deck
    /// see the same per-instance mismatch offsets instead of redrawing them
    /// between the `.op` and the `.tran`.
    ///
    /// Takes `&self` because the counter is shared: every context derived from
    /// the netlist has to see the rewind.
    pub(crate) fn restart(&self) {
        self.counter.store(0, Ordering::Relaxed);
    }

    #[inline]
    fn next_bits(&self) -> u64 {
        // Relaxed is sufficient: draws are pure functions of the index, and
        // ordering across threads is never relied upon (parallel drivers
        // reseed per run instead of sharing one stream).
        let index = self.counter.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
        splitmix64_mix(self.seed.wrapping_add(index.wrapping_mul(GOLDEN_GAMMA)))
    }

    /// Uniform draw in `[0, 1)` with 53-bit resolution.
    #[inline]
    pub fn next_uniform(&self) -> Value {
        // Top 53 bits scaled by 2^-53: the standard full-precision mapping.
        (self.next_bits() >> 11) as Value * (1.0 / 9_007_199_254_740_992.0)
    }

    /// Uniform draw in `[-1, 1)`.
    #[inline]
    pub fn next_symmetric(&self) -> Value {
        2.0 * self.next_uniform() - 1.0
    }

    /// Standard normal draw via Box-Muller (two uniforms per draw).
    #[inline]
    pub fn next_standard_normal(&self) -> Value {
        // 1 - u maps [0,1) onto (0,1], keeping the logarithm finite.
        let u1 = 1.0 - self.next_uniform();
        let u2 = self.next_uniform();
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }
}

impl Default for RandomState {
    fn default() -> Self {
        Self::new(DEFAULT_RANDOM_SEED)
    }
}

/// Evaluation policy for statistical parameter functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatisticalParamMode {
    /// Evaluate `gauss`/`agauss`/`unif`/`aunif`/2-arg `limit` as seeded draws.
    #[default]
    Sample,
    /// Evaluate statistical operators at their nominal value, matching Xyce's
    /// non-UQ expression semantics before a sampling engine injects values.
    Nominal,
}

/// Value-selection policy for repeated parameter definitions within one
/// netlist scope.
///
/// SPICE dialects disagree here: traditional Xyce keeps the first definition,
/// while HSPICE-compatible and many other flows keep the last. Keeping this
/// policy explicit prevents source rewriting and makes the selected semantics
/// part of the parsed parameter environment. Warning/error handling for a
/// duplicate is a separate diagnostic policy and is intentionally not encoded
/// by this value-selection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParameterRedefinitionPolicy {
    /// Retain the first definition encountered in the current scope.
    UseFirst,
    /// Replace an earlier definition with the last one encountered.
    #[default]
    UseLast,
}

/// Diagnostic behavior for repeated parameter definitions within one scope.
///
/// This is deliberately independent from [`ParameterRedefinitionPolicy`]:
/// Xyce exposes modes that select either the first or last value while also
/// choosing whether the duplicate is silent, warned, or fatal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParameterRedefinitionDiagnosticPolicy {
    /// Accept the configured value-selection behavior without a diagnostic.
    #[default]
    Silent,
    /// Accept the configured value-selection behavior and emit a warning for
    /// every definition after the first.
    Warning,
    /// Reject the duplicate with a structured parse error.
    Error,
}

#[derive(Debug, Clone)]
pub(crate) struct ParameterDefinitionAcceptance {
    pub(crate) authoritative: bool,
    pub(crate) first_origin: Option<crate::netlist::NetlistSourceLocation>,
}

/// User-defined function definition
#[derive(Debug, Clone)]
pub struct FunctionDef {
    /// Function name
    pub name: String,
    /// Argument names
    pub args: Vec<String>,
    /// Function body expression string
    pub body: String,
}

impl FunctionDef {
    /// Create a new function definition
    pub fn new(name: &str, args: Vec<String>, body: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            args: args.into_iter().map(|a| a.to_uppercase()).collect(),
            body: body.to_string(),
        }
    }
}

/// Compiled expression and immutable bindings from its definition site.
pub(crate) struct CapturedStatisticalParameter {
    program: PreparedExpression,
    bindings: std::collections::BTreeMap<String, CapturedStatisticalBinding>,
    coordinates: BTreeSet<String>,
    depth: usize,
    random_seed: u64,
    random_counter: u64,
    statistical_mode: StatisticalParamMode,
    expression_dialect: ExpressionDialect,
}

impl std::fmt::Debug for CapturedStatisticalParameter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Shared alias graphs can have exponentially many paths. Debug output
        // must not recursively expand each path; checkpoint snapshots deduplicate
        // nodes separately when the complete semantic representation is needed.
        formatter
            .debug_struct("CapturedStatisticalParameter")
            .field("coordinates", &self.coordinates)
            .field("binding_count", &self.bindings.len())
            .field("depth", &self.depth)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
enum CapturedStatisticalBinding {
    Value(Option<ComplexValue>),
    Parameter(Arc<CapturedStatisticalParameter>),
    Coordinate,
}

impl CapturedStatisticalParameter {
    fn evaluate(
        &self,
        coordinates: &ParamContext,
        memo: &mut HashMap<usize, ComplexValue>,
    ) -> Result<ComplexValue, ExprError> {
        let identity = std::ptr::from_ref(self) as usize;
        if let Some(value) = memo.get(&identity) {
            return Ok(*value);
        }
        let context = ParamContext {
            random: RandomState {
                seed: self.random_seed,
                counter: Arc::new(AtomicU64::new(self.random_counter)),
            },
            statistical_mode: self.statistical_mode,
            expression_dialect: self.expression_dialect,
            ..ParamContext::default()
        };
        let value = self.program.clone().evaluate_with(&context, &mut |name| {
            let value = match self.bindings.get(name) {
                Some(CapturedStatisticalBinding::Value(value)) => *value,
                Some(CapturedStatisticalBinding::Parameter(parameter)) => {
                    Some(parameter.evaluate(coordinates, memo)?)
                }
                Some(CapturedStatisticalBinding::Coordinate) => coordinates.get_complex(name),
                None => None,
            };
            value
                .map(Some)
                .ok_or_else(|| ExprError::UndefinedParam(name.to_owned()))
        })?;
        let value = if self.expression_dialect == ExpressionDialect::Xyce {
            normalize_xyce_expression_result(value)
        } else {
            value
        };
        memo.insert(identity, value);
        Ok(value)
    }

    fn snapshot(&self, seen: &mut HashMap<usize, usize>, output: &mut String) -> usize {
        use std::fmt::Write;
        let identity = std::ptr::from_ref(self) as usize;
        if let Some(index) = seen.get(&identity) {
            return *index;
        }
        let index = seen.len();
        seen.insert(identity, index);
        writeln!(
            output,
            "capture {index}: seed={},counter={},mode={:?},dialect={:?},program={}",
            self.random_seed,
            self.random_counter,
            self.statistical_mode,
            self.expression_dialect,
            self.program.checkpoint_semantic_snapshot()
        )
        .unwrap();
        for (name, binding) in &self.bindings {
            let binding = match binding {
                CapturedStatisticalBinding::Value(value) => format!(
                    "value {:?}",
                    value.map(|value| (value.re.to_bits(), value.im.to_bits()))
                ),
                CapturedStatisticalBinding::Coordinate => "coordinate".to_owned(),
                CapturedStatisticalBinding::Parameter(parameter) => {
                    format!("capture {}", parameter.snapshot(seen, output))
                }
            };
            writeln!(output, "capture {index} binding {name:?}: {binding}").unwrap();
        }
        index
    }
}

#[derive(Debug, Clone, Default)]
struct ParameterDirections {
    ordinary: HashMap<String, Result<ComplexDirection, ExprError>>,
    global: HashMap<String, Result<ComplexDirection, ExprError>>,
}

/// Context for parameter substitution during evaluation.
#[derive(Debug, Clone, Default)]
pub struct ParamContext {
    params: HashMap<String, Value>,
    complex_params: HashMap<String, ComplexValue>,
    string_params: HashMap<String, String>,
    /// Retained ordinary `.PARAM`/`.CSPARAM` expressions whose value depends
    /// on an active analysis quantity such as `TIME` or `FREQ`. Ordinary
    /// expressions remain scoped and settable; a numeric or string binding in
    /// a child scope removes the inherited symbolic definition.
    parameter_expressions: HashMap<String, String>,
    /// Definition-time bindings for ordinary root parameters that depend on a
    /// statistical coordinate. Immutable captures share prior alias versions.
    statistical_captures: HashMap<String, Arc<CapturedStatisticalParameter>>,
    /// Allocated only for an explicitly requested parameter direction.
    parameter_directions: Option<Box<ParameterDirections>>,
    /// Numeric projections owned by the independent `.GLOBAL_PARAM`
    /// namespace. Xyce resolves an ordinary binding first, regardless of
    /// directive order, and consults this namespace only when no ordinary
    /// binding of any type exists.
    global_params: HashMap<String, Value>,
    global_complex_params: HashMap<String, ComplexValue>,
    global_string_params: HashMap<String, String>,
    /// Retained top-level `.GLOBAL_PARAM` expressions. Statically evaluable
    /// definitions may also have a numeric projection in `params`, while
    /// runtime-dependent definitions remain symbolic until device binding.
    global_expressions: HashMap<String, String>,
    /// User-defined functions (.FUNC)
    functions: HashMap<String, FunctionDef>,
    /// Stream for the statistical functions; deterministic by default.
    random: RandomState,
    /// Statistical-function evaluation policy for this parse/evaluation scope.
    statistical_mode: StatisticalParamMode,
    /// Parameter identities whose values are supplied by a native Spectre
    /// process/mismatch plan during elaboration.  Parsers consult this set to
    /// retain dependent expressions instead of freezing them at nominal.
    spectre_statistical_parameters: BTreeSet<String>,
    /// Dialect-specific expression-function behavior.
    expression_dialect: ExpressionDialect,
    /// Repeated-definition behavior selected for this parse/elaboration.
    parameter_redefinition_policy: ParameterRedefinitionPolicy,
    /// Diagnostic behavior for a repeated definition in this parse scope.
    parameter_redefinition_diagnostic_policy: ParameterRedefinitionDiagnosticPolicy,
    /// First source location of each `.PARAM`/`.CSPARAM` definition in the
    /// current parser scope.
    scope_parameter_definitions: HashMap<String, crate::netlist::NetlistSourceLocation>,
    /// First source location of each `.GLOBAL_PARAM` definition in the current
    /// parser scope.
    scope_global_parameter_definitions: HashMap<String, crate::netlist::NetlistSourceLocation>,
    /// First source location of each parameter-function definition in the
    /// current parser scope.
    scope_function_definitions: HashMap<String, crate::netlist::NetlistSourceLocation>,
}

impl ParamContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a parameter value
    pub fn set(&mut self, name: &str, value: Value) {
        let key = name.to_uppercase();
        self.params.insert(key.clone(), value);
        self.complex_params.remove(&key);
        self.string_params.remove(&key);
        self.parameter_expressions.remove(&key);
        self.statistical_captures.remove(&key);
        if let Some(directions) = &mut self.parameter_directions {
            directions.ordinary.remove(&key);
        }
    }

    /// Set a parameter value while preserving its imaginary component.
    pub fn set_complex(&mut self, name: &str, value: ComplexValue) {
        let key = name.to_uppercase();
        self.params.insert(key.clone(), value.re);
        self.string_params.remove(&key);
        self.parameter_expressions.remove(&key);
        self.statistical_captures.remove(&key);
        if let Some(directions) = &mut self.parameter_directions {
            directions.ordinary.remove(&key);
        }
        if is_real(value) {
            self.complex_params.remove(&key);
        } else {
            self.complex_params.insert(key, value);
        }
    }

    /// Set a string parameter value.
    pub fn set_string(&mut self, name: &str, value: impl Into<String>) {
        let key = name.to_uppercase();
        self.string_params.insert(key.clone(), value.into());
        self.params.remove(&key);
        self.complex_params.remove(&key);
        self.parameter_expressions.remove(&key);
        self.statistical_captures.remove(&key);
        if let Some(directions) = &mut self.parameter_directions {
            directions.ordinary.remove(&key);
        }
    }

    /// Define an ordinary scoped parameter expression with an optional static
    /// projection. This is distinct from `.GLOBAL_PARAM`: ordinary parameters
    /// retain lexical shadowing and can be replaced by `.STEP`/instance scope
    /// bindings through the normal numeric setters.
    pub fn define_parameter_expression(
        &mut self,
        name: &str,
        expression: impl Into<String>,
        static_value: Option<ComplexValue>,
    ) {
        let key = name.to_uppercase();
        self.statistical_captures.remove(&key);
        if let Some(directions) = &mut self.parameter_directions {
            directions.ordinary.remove(&key);
        }
        self.params.remove(&key);
        self.complex_params.remove(&key);
        self.string_params.remove(&key);
        if let Some(value) = static_value {
            self.params.insert(key.clone(), value.re);
            if !is_real(value) {
                self.complex_params.insert(key.clone(), value);
            }
        }
        self.parameter_expressions.insert(key, expression.into());
    }

    /// Return a retained ordinary scoped parameter expression.
    pub fn get_parameter_expression(&self, name: &str) -> Option<&str> {
        self.parameter_expressions
            .get(&name.to_uppercase())
            .map(String::as_str)
    }

    /// Return all retained ordinary parameter expressions for deterministic
    /// validation and scope propagation.
    pub fn all_parameter_expressions(&self) -> Vec<(String, String)> {
        self.parameter_expressions
            .iter()
            .map(|(name, expression)| (name.clone(), expression.clone()))
            .collect()
    }

    /// Define a top-level `.GLOBAL_PARAM` expression with an optional static
    /// projection. The expression remains authoritative so dependencies that
    /// contain `TIME` or `FREQ` cannot be accidentally frozen at parse time.
    pub fn define_global_expression(
        &mut self,
        name: &str,
        expression: impl Into<String>,
        static_value: Option<ComplexValue>,
    ) {
        let key = name.to_uppercase();
        if let Some(directions) = &mut self.parameter_directions {
            directions.global.remove(&key);
        }
        self.global_params.remove(&key);
        self.global_complex_params.remove(&key);
        self.global_string_params.remove(&key);
        if let Some(value) = static_value {
            self.global_params.insert(key.clone(), value.re);
            if !is_real(value) {
                self.global_complex_params.insert(key.clone(), value);
            }
        }
        self.global_expressions.insert(key, expression.into());
    }

    /// Set a numeric `.GLOBAL_PARAM` value without disturbing a same-name
    /// ordinary `.PARAM` binding.
    pub fn set_global(&mut self, name: &str, value: Value) {
        let key = name.to_uppercase();
        if let Some(directions) = &mut self.parameter_directions {
            directions.global.remove(&key);
        }
        self.global_params.insert(key.clone(), value);
        self.global_complex_params.remove(&key);
        self.global_string_params.remove(&key);
        self.global_expressions.remove(&key);
    }

    /// Set a complex `.GLOBAL_PARAM` value without crossing namespaces.
    pub fn set_global_complex(&mut self, name: &str, value: ComplexValue) {
        let key = name.to_uppercase();
        if let Some(directions) = &mut self.parameter_directions {
            directions.global.remove(&key);
        }
        self.global_params.insert(key.clone(), value.re);
        self.global_string_params.remove(&key);
        self.global_expressions.remove(&key);
        if is_real(value) {
            self.global_complex_params.remove(&key);
        } else {
            self.global_complex_params.insert(key, value);
        }
    }

    /// Set a string `.GLOBAL_PARAM` value without crossing namespaces.
    pub fn set_global_string(&mut self, name: &str, value: impl Into<String>) {
        let key = name.to_uppercase();
        if let Some(directions) = &mut self.parameter_directions {
            directions.global.remove(&key);
        }
        self.global_string_params.insert(key.clone(), value.into());
        self.global_params.remove(&key);
        self.global_complex_params.remove(&key);
        self.global_expressions.remove(&key);
    }

    /// Return the retained `.GLOBAL_PARAM` expression, if this name denotes
    /// an expression-valued global definition.
    pub fn get_global_expression(&self, name: &str) -> Option<&str> {
        self.global_expressions
            .get(&name.to_uppercase())
            .map(String::as_str)
    }

    /// Return all retained global expressions for deterministic inspection
    /// and propagation into derived parameter scopes.
    pub fn all_global_expressions(&self) -> Vec<(String, String)> {
        self.global_expressions
            .iter()
            .map(|(name, expression)| (name.clone(), expression.clone()))
            .collect()
    }

    /// Whether expression expansion is needed to resolve symbolic bindings.
    pub(crate) fn has_retained_parameter_expressions(&self) -> bool {
        !self.parameter_expressions.is_empty() || !self.global_expressions.is_empty()
    }

    /// Whether the ordinary namespace contains this name in any value class.
    pub fn has_parameter_binding(&self, name: &str) -> bool {
        let key = name.to_uppercase();
        self.params.contains_key(&key)
            || self.complex_params.contains_key(&key)
            || self.string_params.contains_key(&key)
            || self.parameter_expressions.contains_key(&key)
    }

    /// Whether either the ordinary `.PARAM` namespace or the
    /// `.GLOBAL_PARAM` namespace contains this name in any value class.
    pub fn has_any_parameter_binding(&self, name: &str) -> bool {
        let key = name.to_uppercase();
        self.has_parameter_binding(&key) || self.has_global_binding_key(&key)
    }

    fn has_global_binding_key(&self, key: &str) -> bool {
        self.global_params.contains_key(key)
            || self.global_complex_params.contains_key(key)
            || self.global_string_params.contains_key(key)
            || self.global_expressions.contains_key(key)
    }

    fn effective_numeric_without_builtin(&self, key: &str) -> Option<Value> {
        if self.has_parameter_binding(key) {
            return self
                .complex_params
                .get(key)
                .map(|value| value.re)
                .or_else(|| self.params.get(key).copied());
        }
        self.global_complex_params
            .get(key)
            .map(|value| value.re)
            .or_else(|| self.global_params.get(key).copied())
    }

    /// Get a parameter value
    pub fn get(&self, name: &str) -> Option<Value> {
        let key = name.to_uppercase();
        if self.has_any_parameter_binding(&key) {
            return self.effective_numeric_without_builtin(&key);
        }

        let temp_c = self
            .effective_numeric_without_builtin("TEMP")
            .or_else(|| self.effective_numeric_without_builtin("TEMPER"))
            .unwrap_or(DEFAULT_TEMPERATURE_C);
        builtin_numeric_param(&key, temp_c)
    }

    /// Get a parameter value, preserving any imaginary component.
    pub fn get_complex(&self, name: &str) -> Option<ComplexValue> {
        let key = name.to_uppercase();
        if self.has_parameter_binding(&key) {
            return self
                .complex_params
                .get(&key)
                .copied()
                .or_else(|| self.params.get(&key).copied().map(ComplexValue::from));
        }
        if self.has_global_binding_key(&key) {
            return self.global_complex_params.get(&key).copied().or_else(|| {
                self.global_params
                    .get(&key)
                    .copied()
                    .map(ComplexValue::from)
            });
        }
        self.get(&key).map(ComplexValue::from)
    }

    pub(crate) fn seed_parameter_direction(&mut self, name: &str, global: bool) {
        self.parameter_directions
            .get_or_insert_with(Default::default);
        self.retain_parameter_direction(name, global, Some(Ok(1.0.into())));
    }

    pub(crate) fn parameter_direction(&self, name: &str) -> Result<ComplexDirection, ExprError> {
        let Some(directions) = &self.parameter_directions else {
            return Ok(ComplexDirection::zero());
        };
        let key = name.to_ascii_uppercase();
        if !self.has_any_parameter_binding(&key) && matches!(key.as_str(), "TEMP" | "TEMPER" | "VT")
        {
            let temperature = if self.effective_numeric_without_builtin("TEMP").is_some() {
                self.parameter_direction("TEMP")?
            } else if self.effective_numeric_without_builtin("TEMPER").is_some() {
                self.parameter_direction("TEMPER")?
            } else {
                ComplexDirection::zero()
            };
            // Builtins use the real effective temperature, even for a complex binding.
            let direction = ComplexDirection::from(temperature.re);
            return Ok(if key == "VT" {
                direction * crate::constants::thermal_voltage(1.0)
            } else {
                direction
            });
        }
        let bindings = if self.has_parameter_binding(&key) {
            &directions.ordinary
        } else {
            &directions.global
        };
        bindings
            .get(&key)
            .cloned()
            .unwrap_or_else(|| Ok(ComplexDirection::zero()))
    }

    pub(crate) fn retain_parameter_direction(
        &mut self,
        name: &str,
        global: bool,
        direction: Option<Result<ComplexDirection, ExprError>>,
    ) {
        let (Some(directions), Some(direction)) = (&mut self.parameter_directions, direction)
        else {
            return;
        };
        let bindings = if global {
            &mut directions.global
        } else {
            &mut directions.ordinary
        };
        if !matches!(&direction, Ok(direction) if direction.is_zero()) {
            bindings.insert(name.to_ascii_uppercase(), direction);
        } else {
            bindings.remove(&name.to_ascii_uppercase());
        }
    }

    /// Evaluate once at the actual binding site. Both ordinary/global shadowing
    /// and random draws are owned by this context, not reconstructed later.
    pub(crate) fn evaluate_parameter_binding(
        &self,
        expression: &str,
    ) -> Result<(ComplexValue, Option<Result<ComplexDirection, ExprError>>), ExprError> {
        if self.parameter_directions.is_none() {
            return eval_expression_complex(expression, self).map(|value| (value, None));
        }
        let expression = parse_expression(expression)?;
        let mut prepared = PreparedExpression::compile(&expression, self)?;
        let mut dependency_error = None;
        let (value, direction) = prepared.evaluate_parameter_direction_with(self, &mut |name| {
            Ok(self.get_complex(name).map(|value| {
                let direction = self.parameter_direction(name).unwrap_or_else(|error| {
                    dependency_error.get_or_insert(error);
                    ComplexDirection::undefined()
                });
                (value, direction)
            }))
        })?;
        // A dependency's undefined direction enters as the tangent
        // `ComplexDirection::undefined` documents: this expression reports the
        // dependency's own error only where that tangent survives to its
        // value or is consumed by a condition selecting it.
        let direction = match direction {
            Ok(tangent) if !tangent.is_undefined() => Ok(tangent),
            outcome => dependency_error.map_or(outcome, Err),
        };
        let value = if self.expression_dialect == ExpressionDialect::Xyce {
            normalize_xyce_expression_result(value)
        } else {
            value
        };
        Ok((value, Some(direction)))
    }

    /// Get a string parameter value.
    pub fn get_string(&self, name: &str) -> Option<&str> {
        let key = name.to_uppercase();
        if self.has_parameter_binding(&key) {
            return self.string_params.get(&key).map(String::as_str);
        }
        self.global_string_params.get(&key).map(String::as_str)
    }

    /// Merge another context into this one
    ///
    /// Parameters and functions are imported; this context keeps its own
    /// random stream so merging scopes never perturbs draw sequences.
    pub fn merge(&mut self, other: &ParamContext) {
        for (k, v) in &other.params {
            self.set(k, *v);
        }
        for (k, v) in &other.complex_params {
            self.set_complex(k, *v);
        }
        for (k, v) in &other.string_params {
            self.set_string(k, v.clone());
        }
        for (k, expression) in &other.parameter_expressions {
            let value = other
                .complex_params
                .get(k)
                .copied()
                .or_else(|| other.params.get(k).copied().map(ComplexValue::from));
            self.define_parameter_expression(k, expression.clone(), value);
        }
        for (k, v) in &other.global_params {
            self.set_global(k, *v);
        }
        for (k, v) in &other.global_complex_params {
            self.set_global_complex(k, *v);
        }
        for (k, v) in &other.global_string_params {
            self.set_global_string(k, v.clone());
        }
        for (k, expression) in &other.global_expressions {
            let value = other
                .global_complex_params
                .get(k)
                .copied()
                .or_else(|| other.global_params.get(k).copied().map(ComplexValue::from));
            self.define_global_expression(k, expression.clone(), value);
        }
        for (k, v) in &other.functions {
            self.functions.insert(k.clone(), v.clone());
        }
        if let Some(incoming) = &other.parameter_directions {
            let directions = self
                .parameter_directions
                .get_or_insert_with(Default::default);
            directions.ordinary.extend(
                incoming
                    .ordinary
                    .iter()
                    .map(|(name, value)| (name.clone(), value.clone())),
            );
            directions.global.extend(
                incoming
                    .global
                    .iter()
                    .map(|(name, value)| (name.clone(), value.clone())),
            );
        }
        self.statistical_captures.extend(
            other
                .statistical_captures
                .iter()
                .map(|(name, capture)| (name.clone(), capture.clone())),
        );
    }

    /// Reseed the statistical-function stream, restarting it from draw zero.
    ///
    /// Installs a fresh stream decoupled from any previously shared one;
    /// reseed before deriving child contexts that should follow it.
    pub fn set_random_seed(&mut self, seed: u64) {
        self.random = RandomState::new(seed);
    }

    /// Adopt another context's random stream (shared draw counter), so this
    /// context's statistical draws continue the same netlist-wide sequence.
    pub fn adopt_random(&mut self, source: &RandomState) {
        self.random = source.clone();
    }

    /// The stream used by `gauss`/`agauss`/`unif`/`aunif`/2-arg `limit`.
    pub fn random(&self) -> &RandomState {
        &self.random
    }

    /// Rewind the statistical stream to draw zero. See [`RandomState::restart`].
    pub(crate) fn restart_statistical_stream(&self) {
        self.random.restart();
    }

    /// Set the statistical-function evaluation policy.
    pub fn set_statistical_mode(&mut self, mode: StatisticalParamMode) {
        self.statistical_mode = mode;
    }

    /// Current statistical-function evaluation policy.
    pub fn statistical_mode(&self) -> StatisticalParamMode {
        self.statistical_mode
    }

    pub(crate) fn set_spectre_statistical_parameters(
        &mut self,
        parameters: impl IntoIterator<Item = String>,
    ) {
        self.spectre_statistical_parameters = parameters
            .into_iter()
            .map(|parameter| parameter.to_ascii_uppercase())
            .collect();
    }

    pub(crate) fn mark_spectre_statistical_parameter(&mut self, parameter: &str) {
        self.spectre_statistical_parameters
            .insert(parameter.to_ascii_uppercase());
    }

    /// A local definition replaces the inherited binding, including its
    /// statistical identity. Captured aliases keep their own dependencies.
    pub(crate) fn shadow_spectre_statistical_parameter(&mut self, parameter: &str) {
        self.spectre_statistical_parameters
            .remove(&parameter.to_ascii_uppercase());
    }

    pub(crate) fn spectre_statistical_parameter_names(&self) -> Vec<String> {
        self.spectre_statistical_parameters
            .iter()
            .chain(self.statistical_captures.keys())
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    pub(crate) fn expression_references_spectre_statistics(&self, expression: &str) -> bool {
        (!self.spectre_statistical_parameters.is_empty() || !self.statistical_captures.is_empty())
            && self.expression_references_parameters(expression, |identifier| {
                let identifier = identifier.to_ascii_uppercase();
                self.spectre_statistical_parameters.contains(&identifier)
                    || self.statistical_captures.contains_key(&identifier)
            })
    }

    /// Conservative dependency check for expressions and source lines. Follow
    /// each reachable function once without evaluating it or consuming random
    /// draws. Formals shadow parameter leaves in their own function body, but
    /// do not shadow a called function's free parameters.
    pub(crate) fn expression_references_parameters(
        &self,
        expression: &str,
        matches_parameter: impl Fn(&str) -> bool,
    ) -> bool {
        let mut pending = vec![(expression, &[] as &[String])];
        let mut visited = std::collections::HashSet::new();
        while let Some((expression, formals)) = pending.pop() {
            let found = super::behavioral::any_expression_identifier_with_abort(
                expression,
                &crate::abort_signal::NoAbort,
                |name, is_call| {
                    if is_call {
                        if let Some(function) = self.get_function(name)
                            && visited.insert(function.name.as_str())
                        {
                            pending.push((function.body.as_str(), function.args.as_slice()));
                        }
                        false
                    } else {
                        !formals
                            .iter()
                            .any(|formal| formal.eq_ignore_ascii_case(name))
                            && (matches_parameter(name)
                                || self
                                    .statistical_captures
                                    .get(&name.to_ascii_uppercase())
                                    .is_some_and(|capture| {
                                        capture
                                            .coordinates
                                            .iter()
                                            .any(|name| matches_parameter(name))
                                    }))
                    }
                },
            )
            .expect("NoAbort cannot cancel parameter dependency inspection");
            if found {
                return true;
            }
        }
        false
    }

    pub(crate) fn capture_statistical_parameter_expression(
        &self,
        expression: &str,
    ) -> Result<Option<Arc<CapturedStatisticalParameter>>, ExprError> {
        if !self.expression_references_spectre_statistics(expression) {
            return Ok(None);
        }
        let program = PreparedExpression::compile(&parse_expression(expression)?, self)?;
        let mut bindings = std::collections::BTreeMap::new();
        let mut coordinates = BTreeSet::new();
        let mut depth = 1;
        program.visit_runtime_parameters(|name| {
            let binding = if self.spectre_statistical_parameters.contains(name) {
                coordinates.insert(name.to_owned());
                CapturedStatisticalBinding::Coordinate
            } else if let Some(parameter) = self.statistical_captures.get(name) {
                depth = depth.max(parameter.depth + 1);
                coordinates.extend(parameter.coordinates.iter().cloned());
                CapturedStatisticalBinding::Parameter(parameter.clone())
            } else {
                CapturedStatisticalBinding::Value(self.get_complex(name))
            };
            bindings.insert(name.to_owned(), binding);
        });
        if depth > crate::resource::MAX_EXPRESSION_TREE_DEPTH {
            return Err(ExprError::InvalidArgument(
                "Statistical parameter dependency depth exceeds the expression stack safety limit"
                    .to_owned(),
            ));
        }
        Ok(Some(Arc::new(CapturedStatisticalParameter {
            program,
            bindings,
            coordinates,
            depth,
            random_seed: self.random.seed,
            random_counter: self.random.counter.load(Ordering::Relaxed),
            statistical_mode: self.statistical_mode,
            expression_dialect: self.expression_dialect,
        })))
    }

    pub(crate) fn retain_statistical_parameter_capture(
        &mut self,
        name: &str,
        capture: Option<Arc<CapturedStatisticalParameter>>,
    ) {
        if let Some(capture) = capture {
            self.statistical_captures
                .insert(name.to_ascii_uppercase(), capture);
        }
    }

    pub(crate) fn materialize_statistical_parameter_captures(&mut self) -> Result<(), ExprError> {
        let mut memo = HashMap::new();
        let mut names = self
            .statistical_captures
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        names.sort_unstable();
        // Evaluate against the original coordinate environment, then publish
        // values together. A later alias cannot rebind an earlier capture.
        let values = names
            .into_iter()
            .map(|name| {
                let value = self.statistical_captures[&name].evaluate(self, &mut memo)?;
                Ok((name, value))
            })
            .collect::<Result<Vec<_>, ExprError>>()?;
        for (name, value) in values {
            self.params.insert(name.clone(), value.re);
            if is_real(value) {
                self.complex_params.remove(&name);
            } else {
                self.complex_params.insert(name, value);
            }
        }
        Ok(())
    }

    /// Set dialect-specific expression-function semantics.
    pub fn set_expression_dialect(&mut self, dialect: ExpressionDialect) {
        self.expression_dialect = dialect;
    }

    /// Current dialect-specific expression-function semantics.
    pub fn expression_dialect(&self) -> ExpressionDialect {
        self.expression_dialect
    }

    /// Select repeated parameter-definition behavior for subsequent parsing.
    pub fn set_parameter_redefinition_policy(&mut self, policy: ParameterRedefinitionPolicy) {
        self.parameter_redefinition_policy = policy;
    }

    /// Repeated parameter-definition behavior retained by this context.
    pub fn parameter_redefinition_policy(&self) -> ParameterRedefinitionPolicy {
        self.parameter_redefinition_policy
    }

    /// Select duplicate-definition diagnostic behavior for subsequent parsing.
    pub fn set_parameter_redefinition_diagnostic_policy(
        &mut self,
        policy: ParameterRedefinitionDiagnosticPolicy,
    ) {
        self.parameter_redefinition_diagnostic_policy = policy;
    }

    /// Duplicate-definition diagnostic behavior retained by this context.
    pub fn parameter_redefinition_diagnostic_policy(
        &self,
    ) -> ParameterRedefinitionDiagnosticPolicy {
        self.parameter_redefinition_diagnostic_policy
    }

    /// Begin a `.PARAM`/`.CSPARAM` or `.GLOBAL_PARAM` definition.
    ///
    /// Returns whether this definition is authoritative under the configured
    /// policy. The parser still consumes and validates ignored definitions.
    pub(crate) fn accepts_parameter_definition(
        &mut self,
        name: &str,
        global: bool,
        origin: &crate::netlist::NetlistSourceLocation,
    ) -> ParameterDefinitionAcceptance {
        let definitions = if global {
            &mut self.scope_global_parameter_definitions
        } else {
            &mut self.scope_parameter_definitions
        };
        let key = name.to_ascii_uppercase();
        let first_origin = definitions.get(&key).cloned();
        definitions.entry(key).or_insert_with(|| origin.clone());
        ParameterDefinitionAcceptance {
            authoritative: first_origin.is_none()
                || self.parameter_redefinition_policy == ParameterRedefinitionPolicy::UseLast,
            first_origin,
        }
    }

    /// Begin a parameter-function definition in the current parser scope.
    pub(crate) fn accepts_parameter_function_definition(
        &mut self,
        name: &str,
        origin: &crate::netlist::NetlistSourceLocation,
    ) -> ParameterDefinitionAcceptance {
        let key = name.to_ascii_uppercase();
        let first_origin = self.scope_function_definitions.get(&key).cloned();
        self.scope_function_definitions
            .entry(key)
            .or_insert_with(|| origin.clone());
        ParameterDefinitionAcceptance {
            authoritative: first_origin.is_none()
                || self.parameter_redefinition_policy == ParameterRedefinitionPolicy::UseLast,
            first_origin,
        }
    }

    /// Reset parse-time definition tracking when entering a child scope.
    ///
    /// Values and policy remain inherited; only same-scope redefinition
    /// history is cleared so a local definition can shadow its caller.
    pub(crate) fn begin_child_definition_scope(&mut self) {
        self.scope_parameter_definitions.clear();
        self.scope_global_parameter_definitions.clear();
        self.scope_function_definitions.clear();
    }

    /// Get all parameters as a vector of (name, value) tuples
    pub fn all_params(&self) -> Vec<(String, Value)> {
        let mut values = self.global_params.clone();
        for name in self
            .string_params
            .keys()
            .chain(self.parameter_expressions.keys())
        {
            values.remove(name);
        }
        values.extend(
            self.params
                .iter()
                .map(|(name, value)| (name.clone(), *value)),
        );
        values.into_iter().collect()
    }

    /// Get all string parameters as a vector of (name, value) tuples.
    pub fn all_string_params(&self) -> Vec<(String, String)> {
        let mut values = self.global_string_params.clone();
        for name in self
            .params
            .keys()
            .chain(self.complex_params.keys())
            .chain(self.parameter_expressions.keys())
        {
            values.remove(name);
        }
        values.extend(
            self.string_params
                .iter()
                .map(|(name, value)| (name.clone(), value.clone())),
        );
        values.into_iter().collect()
    }

    /// Get all user-defined functions as owned definitions.
    pub fn all_functions(&self) -> Vec<FunctionDef> {
        self.functions.values().cloned().collect()
    }

    /// Define a user function
    ///
    /// # Example
    /// ```ignore
    /// ctx.define_function("SQUARE", vec!["X".to_string()], "X*X");
    /// ctx.define_function("RPAR", vec!["R1".to_string(), "R2".to_string()], "R1*R2/(R1+R2)");
    /// ```
    pub fn define_function(&mut self, name: &str, args: Vec<String>, body: &str) {
        let func = FunctionDef::new(name, args, body);
        self.functions.insert(func.name.clone(), func);
    }

    /// Import an already parsed user-defined function.
    pub fn import_function(&mut self, func: FunctionDef) {
        self.functions.insert(func.name.clone(), func);
    }

    /// Check if a user function is defined
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(&name.to_uppercase())
    }

    /// Get a user function definition
    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(&name.to_uppercase())
    }

    /// Return the numeric parameter bindings in deterministic name order.
    ///
    /// This is used by analyses whose parameter context can vary at every
    /// accepted point, such as Xyce `.DC DATA` table rows.
    pub fn numeric_parameters(&self) -> Vec<(String, Value)> {
        let mut values = self.all_params();
        values.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        values
    }

    /// Deterministic semantic snapshot used to bind transient checkpoints to
    /// the elaborated parameter environment. Map iteration order and runtime
    /// allocation addresses are deliberately excluded.
    pub(crate) fn checkpoint_semantic_snapshot(&self) -> String {
        let mut numeric = self
            .params
            .iter()
            .map(|(name, value)| (name.clone(), value.to_bits()))
            .collect::<Vec<_>>();
        numeric.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut complex = self
            .complex_params
            .iter()
            .map(|(name, value)| (name.clone(), value.re.to_bits(), value.im.to_bits()))
            .collect::<Vec<_>>();
        complex.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut strings = self
            .string_params
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<Vec<_>>();
        strings.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut global_numeric = self
            .global_params
            .iter()
            .map(|(name, value)| (name.clone(), value.to_bits()))
            .collect::<Vec<_>>();
        global_numeric.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut global_complex = self
            .global_complex_params
            .iter()
            .map(|(name, value)| (name.clone(), value.re.to_bits(), value.im.to_bits()))
            .collect::<Vec<_>>();
        global_complex.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut global_strings = self
            .global_string_params
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<Vec<_>>();
        global_strings.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut globals = self
            .global_expressions
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<Vec<_>>();
        globals.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut parameter_expressions = self
            .parameter_expressions
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<Vec<_>>();
        parameter_expressions.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut functions = self.functions.values().cloned().collect::<Vec<_>>();
        functions.sort_unstable_by(|left, right| left.name.cmp(&right.name));
        let mut snapshot = format!(
            "numeric={numeric:?}\ncomplex={complex:?}\nstrings={strings:?}\nparameter_expressions={parameter_expressions:?}\nglobal_numeric={global_numeric:?}\nglobal_complex={global_complex:?}\nglobal_strings={global_strings:?}\nglobal_expressions={globals:?}\nfunctions={functions:?}\nrandom_seed={}\nstatistical_mode={:?}\nspectre_statistical_parameters={:?}\nexpression_dialect={:?}\nparameter_redefinition_policy={:?}\n",
            self.random.seed,
            self.statistical_mode,
            self.spectre_statistical_parameters,
            self.expression_dialect,
            self.parameter_redefinition_policy,
        );
        let mut names = self.statistical_captures.keys().collect::<Vec<_>>();
        names.sort_unstable();
        let mut seen = HashMap::new();
        for name in names {
            use std::fmt::Write;
            let index = self.statistical_captures[name].snapshot(&mut seen, &mut snapshot);
            writeln!(snapshot, "statistical parameter {name:?}: capture {index}").unwrap();
        }
        snapshot
    }

    /// Clone this parameter environment without sharing its mutable random
    /// draw counter. Checkpoint provenance may elaborate a circuit to discover
    /// dependencies, but that read-only operation must not perturb the live
    /// simulation's deterministic statistical stream.
    pub(crate) fn isolated_random_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.random = RandomState {
            seed: self.random.seed,
            counter: Arc::new(AtomicU64::new(self.random.counter.load(Ordering::Relaxed))),
        };
        cloned
    }

    /// Clone this context for checkpoint provenance without sharing the live
    /// statistical draw counter.
    pub(crate) fn checkpoint_isolated_clone(&self) -> Self {
        self.isolated_random_clone()
    }

    /// Number of user-defined functions in this scope. Behavioral lowering
    /// uses this to enable eager constant folding for large function graphs,
    /// preventing repeated argument substitution from multiplying static ASTs.
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }
}

fn builtin_numeric_param(name: &str, temp_c: Value) -> Option<Value> {
    match name {
        "TEMP" | "TEMPER" => Some(temp_c),
        "VT" => Some(crate::constants::thermal_voltage(
            crate::constants::celsius_to_kelvin(temp_c),
        )),
        _ => None,
    }
}
