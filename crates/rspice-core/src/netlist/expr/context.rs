use super::*;
use std::cell::Cell;
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
const MAX_FUNCTION_CALL_DEPTH: usize = 4096;
const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

thread_local! {
    static FUNCTION_CALL_DEPTH: Cell<usize> = const { Cell::new(0) };
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatisticalParamMode {
    /// Evaluate `gauss`/`agauss`/`unif`/`aunif`/2-arg `limit` as seeded draws.
    Sample,
    /// Evaluate statistical operators at their nominal value, matching Xyce's
    /// non-UQ expression semantics before a sampling engine injects values.
    Nominal,
}

impl Default for StatisticalParamMode {
    fn default() -> Self {
        Self::Sample
    }
}

/// Dialect-specific expression-function semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionDialect {
    /// ngspice-compatible expression functions: `log(x)` is natural log.
    Ngspice,
    /// Xyce-compatible expression functions: `log(x)` is base-10 log.
    Xyce,
}

impl Default for ExpressionDialect {
    fn default() -> Self {
        Self::Ngspice
    }
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

/// Context for parameter substitution during evaluation
#[derive(Debug, Clone, Default)]
pub struct ParamContext {
    params: HashMap<String, Value>,
    complex_params: HashMap<String, ComplexValue>,
    string_params: HashMap<String, String>,
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
    /// Dialect-specific expression-function behavior.
    expression_dialect: ExpressionDialect,
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
        self.global_expressions.remove(&key);
    }

    /// Set a parameter value while preserving its imaginary component.
    pub fn set_complex(&mut self, name: &str, value: ComplexValue) {
        let key = name.to_uppercase();
        self.params.insert(key.clone(), value.real_projection());
        self.global_expressions.remove(&key);
        if value.is_real() {
            self.complex_params.remove(&key);
        } else {
            self.complex_params.insert(key, value);
        }
    }

    /// Set a string parameter value.
    pub fn set_string(&mut self, name: &str, value: impl Into<String>) {
        let key = name.to_uppercase();
        self.string_params.insert(key.clone(), value.into());
        self.global_expressions.remove(&key);
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
        self.params.remove(&key);
        self.complex_params.remove(&key);
        self.string_params.remove(&key);
        if let Some(value) = static_value {
            self.params.insert(key.clone(), value.real_projection());
            if !value.is_real() {
                self.complex_params.insert(key.clone(), value);
            }
        }
        self.global_expressions.insert(key, expression.into());
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

    /// Get a parameter value
    pub fn get(&self, name: &str) -> Option<Value> {
        let key = name.to_uppercase();
        if let Some(value) = self.complex_params.get(&key) {
            return Some(value.real_projection());
        }
        if let Some(value) = self.params.get(&key) {
            return Some(*value);
        }

        let temp_c = self
            .params
            .get("TEMP")
            .or_else(|| self.params.get("TEMPER"))
            .copied()
            .unwrap_or(DEFAULT_TEMPERATURE_C);
        builtin_numeric_param(&key, temp_c)
    }

    /// Get a parameter value, preserving any imaginary component.
    pub fn get_complex(&self, name: &str) -> Option<ComplexValue> {
        let key = name.to_uppercase();
        if let Some(value) = self.complex_params.get(&key) {
            return Some(*value);
        }
        self.get(&key).map(ComplexValue::real)
    }

    /// Get a string parameter value.
    pub fn get_string(&self, name: &str) -> Option<&str> {
        self.string_params
            .get(&name.to_uppercase())
            .map(String::as_str)
    }

    /// Merge another context into this one
    ///
    /// Parameters and functions are imported; this context keeps its own
    /// random stream so merging scopes never perturbs draw sequences.
    pub fn merge(&mut self, other: &ParamContext) {
        for (k, v) in &other.params {
            self.params.insert(k.clone(), *v);
            if !other.complex_params.contains_key(k) {
                self.complex_params.remove(k);
            }
            if !other.global_expressions.contains_key(k) {
                self.global_expressions.remove(k);
            }
        }
        for (k, v) in &other.complex_params {
            self.complex_params.insert(k.clone(), *v);
            if !other.global_expressions.contains_key(k) {
                self.global_expressions.remove(k);
            }
        }
        for (k, v) in &other.string_params {
            self.string_params.insert(k.clone(), v.clone());
            self.global_expressions.remove(k);
        }
        for (k, expression) in &other.global_expressions {
            if !other.params.contains_key(k) {
                self.params.remove(k);
                self.complex_params.remove(k);
            }
            self.string_params.remove(k);
            self.global_expressions
                .insert(k.clone(), expression.clone());
        }
        for (k, v) in &other.functions {
            self.functions.insert(k.clone(), v.clone());
        }
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

    /// Set the statistical-function evaluation policy.
    pub fn set_statistical_mode(&mut self, mode: StatisticalParamMode) {
        self.statistical_mode = mode;
    }

    /// Current statistical-function evaluation policy.
    pub fn statistical_mode(&self) -> StatisticalParamMode {
        self.statistical_mode
    }

    /// Set dialect-specific expression-function semantics.
    pub fn set_expression_dialect(&mut self, dialect: ExpressionDialect) {
        self.expression_dialect = dialect;
    }

    /// Current dialect-specific expression-function semantics.
    pub fn expression_dialect(&self) -> ExpressionDialect {
        self.expression_dialect
    }

    /// Get all parameters as a vector of (name, value) tuples
    pub fn all_params(&self) -> Vec<(String, Value)> {
        self.params.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }

    /// Get all string parameters as a vector of (name, value) tuples.
    pub fn all_string_params(&self) -> Vec<(String, String)> {
        self.string_params
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
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
        let mut values = self
            .params
            .iter()
            .map(|(name, value)| (name.clone(), *value))
            .collect::<Vec<_>>();
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
        let mut globals = self
            .global_expressions
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<Vec<_>>();
        globals.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        let mut functions = self.functions.values().cloned().collect::<Vec<_>>();
        functions.sort_unstable_by(|left, right| left.name.cmp(&right.name));
        format!(
            "numeric={numeric:?}\ncomplex={complex:?}\nstrings={strings:?}\nglobals={globals:?}\nfunctions={functions:?}\nrandom_seed={}\nstatistical_mode={:?}\nexpression_dialect={:?}\n",
            self.random.seed, self.statistical_mode, self.expression_dialect,
        )
    }

    /// Clone this parameter environment without sharing its mutable random
    /// draw counter. Checkpoint provenance may elaborate a circuit to discover
    /// dependencies, but that read-only operation must not perturb the live
    /// simulation's deterministic statistical stream.
    pub(crate) fn checkpoint_isolated_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.random = RandomState {
            seed: self.random.seed,
            counter: Arc::new(AtomicU64::new(self.random.counter.load(Ordering::Relaxed))),
        };
        cloned
    }

    /// Number of user-defined functions in this scope. Behavioral lowering
    /// uses this to enable eager constant folding for large function graphs,
    /// preventing repeated argument substitution from multiplying static ASTs.
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    /// Call a user-defined function with the given argument values
    ///
    /// Creates a temporary context with the arguments bound to their values
    /// and evaluates the function body.
    pub fn call_function(&self, name: &str, arg_values: &[Value]) -> Result<Value, ExprError> {
        let func = self
            .get_function(name)
            .ok_or_else(|| ExprError::UnknownFunction(name.to_string()))?;

        if arg_values.len() != func.args.len() {
            return Err(ExprError::WrongArgCount(name.to_string()));
        }

        let _depth_guard = FunctionCallDepthGuard::enter(name)?;

        // Create temporary context with arguments bound
        let mut temp_ctx = self.clone();
        for (arg_name, &arg_value) in func.args.iter().zip(arg_values.iter()) {
            temp_ctx.set(arg_name, arg_value);
        }

        // Parse and evaluate the function body
        let expr = parse_expression(&func.body)?;
        evaluate(&expr, &temp_ctx)
    }
}

struct FunctionCallDepthGuard;

impl FunctionCallDepthGuard {
    fn enter(name: &str) -> Result<Self, ExprError> {
        FUNCTION_CALL_DEPTH.with(|depth| {
            let current = depth.get();
            if current >= MAX_FUNCTION_CALL_DEPTH {
                return Err(ExprError::InvalidArgument(format!(
                    "function nesting exceeds maximum depth of {} while calling {}",
                    MAX_FUNCTION_CALL_DEPTH, name
                )));
            }
            depth.set(current + 1);
            Ok(Self)
        })
    }
}

impl Drop for FunctionCallDepthGuard {
    fn drop(&mut self) {
        FUNCTION_CALL_DEPTH.with(|depth| {
            depth.set(depth.get().saturating_sub(1));
        });
    }
}

fn builtin_numeric_param(name: &str, temp_c: Value) -> Option<Value> {
    match name {
        "TEMP" | "TEMPER" => Some(temp_c),
        "VT" => Some(crate::constants::thermal_voltage(
            crate::analysis::temperature::celsius_to_kelvin(temp_c),
        )),
        _ => None,
    }
}
