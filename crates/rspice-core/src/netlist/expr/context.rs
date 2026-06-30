use super::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

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
    string_params: HashMap<String, String>,
    /// User-defined functions (.FUNC)
    functions: HashMap<String, FunctionDef>,
    /// Stream for the statistical functions; deterministic by default.
    random: RandomState,
}

impl ParamContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a parameter value
    pub fn set(&mut self, name: &str, value: Value) {
        self.params.insert(name.to_uppercase(), value);
    }

    /// Set a string parameter value.
    pub fn set_string(&mut self, name: &str, value: impl Into<String>) {
        self.string_params.insert(name.to_uppercase(), value.into());
    }

    /// Get a parameter value
    pub fn get(&self, name: &str) -> Option<Value> {
        let key = name.to_uppercase();
        self.params
            .get(&key)
            .copied()
            .or_else(|| builtin_numeric_param(&key))
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
        }
        for (k, v) in &other.string_params {
            self.string_params.insert(k.clone(), v.clone());
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

    /// Check if a user function is defined
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(&name.to_uppercase())
    }

    /// Get a user function definition
    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(&name.to_uppercase())
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

fn builtin_numeric_param(name: &str) -> Option<Value> {
    match name {
        "TEMP" | "TEMPER" => Some(DEFAULT_TEMPERATURE_C),
        _ => None,
    }
}
