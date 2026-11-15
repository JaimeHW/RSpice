//! Public entry points for parsing and evaluating expressions.
//!
//! `parse_expression` yields an AST that can be evaluated repeatedly, which
//! is what callers inside the Newton loop want. The `eval_*` helpers parse
//! and evaluate in one call for one-shot use. Each comes in a real-valued
//! form and a `_complex` form; the real form takes the real component.

use super::*;

//=============================================================================
// Public API
//=============================================================================

/// Parse a SPICE expression string into an AST
pub fn parse_expression(input: &str) -> Result<Expr, ExprError> {
    if let Some(parsed) = ParseMemo::recall(input) {
        return parsed;
    }
    let mut parser = ExprParser::new(input);
    let parsed = parser.parse();
    ParseMemo::retain(input, &parsed);
    parsed
}

/// The parse of every distinct expression text read while a [`ParseMemo`]
/// scope is open on this thread, for each way of reading number suffixes.
type ParsedTexts = [rustc_hash::FxHashMap<Box<str>, Result<Expr, ExprError>>; 2];

thread_local! {
    static PARSED_TEXTS: std::cell::RefCell<Option<ParsedTexts>> =
        const { std::cell::RefCell::new(None) };
}

/// Keeps the parse of each expression text for as long as it lives.
///
/// Expanding a hierarchy evaluates one subcircuit body's value expressions
/// once per instance; parsing the same text again for each instance made the
/// parse, not the evaluation, most of a large hierarchical deck's expansion.
/// A parse depends only on the text and on how this thread reads number
/// suffixes, so a remembered parse is the parse. The memo is this thread's
/// alone and is dropped with the outermost scope.
pub(crate) struct ParseMemo {
    outermost: bool,
}

impl ParseMemo {
    /// Open a scope; an enclosing scope's memo stays in use.
    pub(crate) fn enter() -> Self {
        let outermost = PARSED_TEXTS.with(|texts| {
            let mut texts = texts.borrow_mut();
            let outermost = texts.is_none();
            if outermost {
                *texts = Some(Default::default());
            }
            outermost
        });
        Self { outermost }
    }

    fn recall(input: &str) -> Option<Result<Expr, ExprError>> {
        let suffixes = usize::from(crate::spice_number::reading_numbers_as_ngspice());
        PARSED_TEXTS.with(|texts| {
            texts
                .borrow()
                .as_ref()
                .and_then(|texts| texts[suffixes].get(input).cloned())
        })
    }

    fn retain(input: &str, parsed: &Result<Expr, ExprError>) {
        let suffixes = usize::from(crate::spice_number::reading_numbers_as_ngspice());
        PARSED_TEXTS.with(|texts| {
            if let Some(texts) = texts.borrow_mut().as_mut() {
                texts[suffixes].insert(input.into(), parsed.clone());
            }
        });
    }
}

impl Drop for ParseMemo {
    fn drop(&mut self) {
        if self.outermost {
            PARSED_TEXTS.with(|texts| texts.borrow_mut().take());
        }
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub enum ParseExpressionWithAbortError {
    Aborted,
    Parse(ExprError),
}

#[doc(hidden)]
pub fn parse_expression_with_abort(
    input: &str,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<Expr, ParseExpressionWithAbortError> {
    if let Some(parsed) = ParseMemo::recall(input) {
        return parsed.map_err(ParseExpressionWithAbortError::Parse);
    }
    let mut parser = ExprParser::with_abort(input, abort);
    let result = parser.parse();
    if parser.was_aborted() {
        Err(ParseExpressionWithAbortError::Aborted)
    } else {
        ParseMemo::retain(input, &result);
        result.map_err(ParseExpressionWithAbortError::Parse)
    }
}

/// Parse an expression under Xyce's lexer, which reads `:` inside a name as
/// part of it: `{X1:R1:R*2}` names the device parameter `X1:R1:R`.
#[doc(hidden)]
pub fn parse_xyce_expression_with_abort(
    input: &str,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<Expr, ParseExpressionWithAbortError> {
    let mut parser = ExprParser::with_abort(input, abort).with_colon_identifiers();
    let result = parser.parse();
    if parser.was_aborted() {
        Err(ParseExpressionWithAbortError::Aborted)
    } else {
        result.map_err(ParseExpressionWithAbortError::Parse)
    }
}

#[doc(hidden)]
pub fn parse_control_expression_prefix_with_abort(
    input: &str,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<(Expr, usize), ParseExpressionWithAbortError> {
    let mut parser = ExprParser::with_abort(input, abort);
    let result = parser.parse_control_prefix();
    if parser.was_aborted() {
        Err(ParseExpressionWithAbortError::Aborted)
    } else {
        result.map_err(ParseExpressionWithAbortError::Parse)
    }
}

#[doc(hidden)]
pub fn parse_control_expression_with_abort(
    input: &str,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<Expr, ParseExpressionWithAbortError> {
    let (expression, consumed) = parse_control_expression_prefix_with_abort(input, abort)?;
    if consumed == input.len() {
        Ok(expression)
    } else {
        Err(ParseExpressionWithAbortError::Parse(
            ExprError::TrailingInput(input[consumed..].to_owned()),
        ))
    }
}

/// Parse and evaluate a SPICE expression with the given context
pub fn eval_expression(input: &str, ctx: &ParamContext) -> Result<Value, ExprError> {
    let expr = parse_expression(input)?;
    evaluate(&expr, ctx)
}

/// Parse and evaluate a SPICE expression, preserving complex values.
pub fn eval_expression_complex(input: &str, ctx: &ParamContext) -> Result<ComplexValue, ExprError> {
    let expr = parse_expression(input)?;
    evaluate_complex(&expr, ctx)
}

impl From<ParseExpressionWithAbortError> for ExpressionEvaluationError {
    fn from(error: ParseExpressionWithAbortError) -> Self {
        match error {
            ParseExpressionWithAbortError::Aborted => Self::Aborted,
            ParseExpressionWithAbortError::Parse(error) => Self::Expression(error),
        }
    }
}

#[doc(hidden)]
pub fn eval_expression_complex_with_abort(
    input: &str,
    ctx: &ParamContext,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<ComplexValue, ExpressionEvaluationError> {
    let expr = parse_expression_with_abort(input, abort)?;
    evaluate_complex_with_functions_and_abort(
        &expr,
        ctx,
        &mut |_| Ok(None),
        &mut |_, _| Ok(None),
        abort,
    )
}

/// Probe forward bindings without consuming the live statistical stream. Reuse
/// the parsed expression and perform the selected evaluation only once.
pub(crate) fn eval_expression_complex_with_probe_and_abort(
    input: &str,
    ctx: &ParamContext,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<ComplexValue, ExpressionEvaluationError> {
    eval_expression_complex_with_probe_and_resolver(input, ctx, &mut |_| Ok(None), abort)
}

/// The resolver is a side-effect-free parameter lookup/filter: both the
/// isolated probe and the selected live evaluation consult the same bindings.
pub(crate) fn eval_expression_complex_with_probe_and_resolver(
    input: &str,
    ctx: &ParamContext,
    resolver: &mut impl FnMut(&str) -> Result<Option<ComplexValue>, ExprError>,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<ComplexValue, ExpressionEvaluationError> {
    let expr = parse_expression_with_abort(input, abort)?;
    // A function not yet defined is also a forward reference. Conservatively
    // probe calls (and grouping) before a prefix can draw from the live stream.
    if input.contains('(') || expression_needs_forward_reference_probe(&expr, ctx) {
        evaluate_complex_with_functions_and_abort(
            &expr,
            &ctx.isolated_random_clone(),
            resolver,
            &mut |_, _| Ok(None),
            abort,
        )?;
    }
    evaluate_complex_with_functions_and_abort(&expr, ctx, resolver, &mut |_, _| Ok(None), abort)
}

/// Known scalar bindings and builtin functions cannot introduce a missing name.
/// Keep their common path free of full parameter-environment snapshots. User
/// functions and retained bindings are conservatively probed because their
/// bodies can hide both forward references and statistical calls.
pub(crate) fn needs_forward_reference_probe(expression: &str, params: &ParamContext) -> bool {
    let Ok(parsed) = parse_expression(expression) else {
        return false;
    };
    expression_needs_forward_reference_probe(&parsed, params)
}

fn expression_needs_forward_reference_probe(expression: &Expr, params: &ParamContext) -> bool {
    let mut pending = vec![expression];
    while let Some(node) = pending.pop() {
        match node {
            Expr::Param(name)
                if params.get_complex(name).is_none()
                    || params.get_parameter_expression(name).is_some()
                    || params.get_global_expression(name).is_some() =>
            {
                return true;
            }
            Expr::FnCall { name, .. } if params.get_function(name).is_some() => return true,
            Expr::FnCall { args, .. } => pending.extend(args.iter()),
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            _ => {}
        }
    }
    false
}

#[cfg(test)]
mod parse_memo_tests {
    use super::*;

    fn value(expression: &Result<Expr, ExprError>) -> String {
        format!("{expression:?}")
    }

    #[test]
    fn a_remembered_parse_is_the_parse_for_each_suffix_reading() {
        let texts = ["2x*r/2", "1x", "{oops", "a+(b"];
        let plain = texts.map(|text| value(&parse_expression(text)));
        let ngspice = {
            let _reading = crate::spice_number::read_suffixes_as_ngspice(true);
            texts.map(|text| value(&parse_expression(text)))
        };
        assert_ne!(plain[1], ngspice[1], "the readings differ on `x`");
        let _memo = ParseMemo::enter();
        for _ in 0..2 {
            assert_eq!(texts.map(|text| value(&parse_expression(text))), plain);
            let _reading = crate::spice_number::read_suffixes_as_ngspice(true);
            assert_eq!(
                texts.map(|text| value(
                    &parse_expression_with_abort(text, &crate::abort_signal::NoAbort).map_err(
                        |error| match error {
                            ParseExpressionWithAbortError::Parse(error) => error,
                            ParseExpressionWithAbortError::Aborted => unreachable!(),
                        }
                    )
                )),
                ngspice
            );
        }
    }

    #[test]
    fn the_memo_lives_as_long_as_its_outermost_scope() {
        let outer = ParseMemo::enter();
        {
            let _inner = ParseMemo::enter();
            parse_expression("r*2").expect("parses");
        }
        assert!(ParseMemo::recall("r*2").is_some());
        drop(outer);
        assert!(ParseMemo::recall("r*2").is_none());
    }
}

#[cfg(test)]
mod cancellation_tests {
    use super::*;

    #[test]
    fn failed_forward_probes_leave_the_live_statistical_stream_untouched() {
        for source in ["aunif(0,1)+missing", "aunif(0,1)+late()"] {
            let mut params = ParamContext::new();
            params.set_random_seed(73);
            let reference = params.isolated_random_clone();
            assert!(
                eval_expression_complex_with_probe_and_abort(
                    source,
                    &params,
                    &crate::abort_signal::NoAbort
                )
                .is_err()
            );
            assert_eq!(
                eval_expression("aunif(0,1)", &params).unwrap(),
                eval_expression("aunif(0,1)", &reference).unwrap()
            );
        }
    }

    #[test]
    fn grouping_and_numeric_suffixes_cancel_at_every_parser_boundary() {
        for source in ["(1+2)", "{1+2}", "10kOhm", "2j", "'1/(27-28)'"] {
            let mut completed = false;
            for limit in 0..256 {
                let abort = crate::abort_signal::CountingAbort::new(limit);
                match parse_expression_with_abort(source, &abort) {
                    Err(ParseExpressionWithAbortError::Aborted) => {}
                    Ok(_) => completed = true,
                    result => panic!("{source}, poll limit {limit}: {result:?}"),
                }
                assert_eq!(abort.polls_after_abort(), 0, "{source}, poll limit {limit}");
                if completed {
                    break;
                }
            }
            assert!(completed, "{source} never completed");
        }
    }
}
