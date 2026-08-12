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
//!   [`ExpressionDialect::Xyce`] where it is base 10. `LN` and `LOG10` are
//!   unambiguous and always mean what they say.
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
mod parser;
mod types;

#[cfg(test)]
mod tests;

use parser::ExprParser;

pub(crate) use api::{ParseExpressionWithAbortError, parse_expression_with_abort};
pub use api::{
    eval_expression, eval_expression_complex, eval_simple, eval_simple_complex, parse_expression,
};
pub(crate) use behavioral::{
    BehavioralPreparationError, expand_output_user_functions_with_abort,
    prepare_behavioral_expression_with_abort, validate_prepared_behavioral_runtime_expression,
};
pub use behavioral::{
    ParameterCircuitProbe, ParameterCircuitProbeKind, RuntimeSpecialQuantity,
    behavioral_expression_references_frequency, behavioral_expression_references_runtime_quantity,
    behavioral_expression_references_unbound_frequency, expand_output_user_functions,
    finalize_parameter_expressions, materialize_available_parameter_expressions,
    parameter_expression_circuit_probe, prepare_behavioral_expression,
    prepare_behavioral_expression_preserving_spelling, runtime_special_quantity,
    validate_global_parameter_expressions, validate_parameter_expressions,
};
pub use context::{
    DEFAULT_RANDOM_SEED, FunctionDef, ParamContext, ParameterRedefinitionPolicy, RandomState,
    StatisticalParamMode,
};
pub use error::ExprError;
pub(crate) use eval::{
    PreparedExpression, evaluate_complex_raw, normalize_xyce_expression_component,
    normalize_xyce_expression_result,
};
pub use eval::{evaluate, evaluate_complex};
pub use types::{BinOpKind, ComplexValue, Expr, UnaryOpKind};
