//! Expression evaluator for `.PARAM` statements and behavioral sources.
//!
//! Evaluation is complex-valued throughout so the same expressions serve AC
//! analysis; real-only callers use the `eval_simple*` entry points.
//!
//! Supports arithmetic (`+ - * / **`), comparison and boolean operators,
//! parenthesized grouping, parameter substitution from a [`ParamContext`],
//! and roughly fifty built-in functions: transcendental and hyperbolic
//! functions and their inverses, rounding (`FLOOR`, `CEIL`, `ROUND`, `INT`,
//! `NINT`), complex accessors (`R`/`RE`, `IMG`, `PH`, `M`/`MAG`, `DB`),
//! conditionals (`IF`, `LIMIT`, `TABLE`), sign and step helpers (`SGN`,
//! `URAMP`, `U`, `U2`, `EQ0`, `GT0`, and the rest of the comparison family),
//! and the statistical generators `GAUSS`, `UNIF`, and `RAND`. Most accept
//! several spellings, so `ABS`, `M`, and `MAG` are one function.
//!
//! Two dialect-sensitive cases are worth knowing, since they change results
//! rather than raise errors:
//!
//! - `LOG` is the natural logarithm, except under
//!   [`crate::config::ExpressionDialect::Xyce`] where it is base 10. `LN` and
//!   `LOG10` are unambiguous and always mean what they say.
//! - `PH`/`PHASE` returns degrees, not radians.
//!
//! Submodules: `parser` and `types` build the AST, `eval` walks it, `context`
//! holds parameter scopes and the statistical generators' state, `behavioral`
//! adds the circuit probes and runtime quantities that behavioral sources
//! reference, and `api` is the public entry surface.

use crate::Value;
use std::collections::HashMap;

mod api;
mod behavioral;
mod context;
mod error;
mod eval;
mod model;
mod parameter_direction;
mod parser;
mod resolution;
mod scalar_direction;
mod types;

#[cfg(test)]
mod tests;

use parser::ExprParser;

pub use crate::ComplexValue;
#[doc(hidden)]
pub use api::{
    ParseExpressionWithAbortError, eval_expression_complex_with_abort,
    parse_control_expression_prefix_with_abort, parse_control_expression_with_abort,
    parse_expression_with_abort,
};
pub use api::{eval_expression, eval_expression_complex, eval_simple, parse_expression};
pub(crate) use api::{eval_expression_complex_with_probe_and_abort, needs_forward_reference_probe};
#[doc(hidden)]
pub use behavioral::{
    BehavioralPreparationError, expand_output_user_functions_with_abort,
    materialize_available_parameter_expressions_with_abort,
    prepare_behavioral_expression_with_abort,
};
pub use behavioral::{
    ParameterCircuitProbe, ParameterCircuitProbeKind, RuntimeSpecialQuantity,
    behavioral_expression_references_frequency, behavioral_expression_references_runtime_quantity,
    behavioral_expression_references_unbound_frequency, expand_output_user_functions,
    finalize_parameter_expressions, materialize_available_parameter_expressions,
    parameter_expression_circuit_probe, prepare_behavioral_expression, runtime_special_quantity,
    validate_global_parameter_expressions, validate_parameter_expressions,
};
pub(crate) use behavioral::{
    finalize_parameter_expressions_with_abort,
    prepare_behavioral_expression_preserving_parameters_with_abort,
    prepare_behavioral_expression_preserving_spelling_with_abort,
    validate_prepared_behavioral_runtime_expression,
};
pub(crate) use context::{CapturedStatisticalParameter, MODEL_TEMPERATURE_PARAMETERS};
pub use context::{
    DEFAULT_RANDOM_SEED, FunctionDef, ParamContext, ParameterRedefinitionDiagnosticPolicy,
    ParameterRedefinitionPolicy, RandomState, StatisticalParamMode,
};
pub use error::ExprError;
pub(crate) use eval::PreparedProgress;
#[doc(hidden)]
pub use eval::{
    ExpressionEvaluationError, PreparedExpression, evaluate_complex_raw,
    evaluate_complex_with_functions_and_abort, normalize_xyce_expression_component,
    normalize_xyce_expression_result,
};
pub use eval::{evaluate, evaluate_complex};
pub(crate) use model::evaluate_instance_expression;
#[doc(hidden)]
pub use model::{
    ModelEvaluationContext, ModelNominalTemperature, resolve_real_instance_expressions,
    set_model_temperature_scalars,
};
#[doc(hidden)]
pub use parameter_direction::ComplexDirection;
#[doc(hidden)]
pub use resolution::ParameterResolutionError;
pub(crate) use resolution::{ParameterEnvironment, ParameterResolver};
pub(crate) use types::require_real;
pub use types::{BinOpKind, Expr, UnaryOpKind, is_real};
