//! Forward temperature option bindings, owned by the parser's lexical scopes.

use super::*;
use crate::netlist::expr::{
    ExpressionEvaluationError, ParameterEnvironment, ParameterResolutionError, ParameterResolver,
    PreparedExpression, PreparedProgress,
};
use crate::netlist::parser::scopes::{LexicalScopes, ScopeEnvironment};

mod binding;
use binding::ScopeBinding;

#[derive(Clone, Copy, Debug)]
pub(in super::super) enum TemperatureOption {
    Temp,
    Tnom,
}

impl TemperatureOption {
    fn name(self) -> &'static str {
        match self {
            Self::Temp => "TEMP",
            Self::Tnom => "TNOM",
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Temp => 0,
            Self::Tnom => 1,
        }
    }

    fn install(self, options: &mut SimulationOptions, value: Value) {
        match self {
            Self::Temp => options.temp = Some(value),
            Self::Tnom => options.tnom = Some(value),
        }
    }
}

#[derive(Debug)]
struct PendingTemperatureOption {
    option: TemperatureOption,
    ordinal: usize,
    expression: PreparedExpression,
    bound_values: HashMap<String, crate::ComplexValue>,
    sign: Value,
    origin: NetlistSourceLocation,
}

#[derive(Debug)]
struct PendingScope {
    id: usize,
    entries: Vec<PendingTemperatureOption>,
}

#[derive(Debug, Default)]
pub(in super::super) struct TemperatureOptionPlan {
    ordinal: usize,
    last: [usize; 2],
    scopes: Vec<Vec<PendingTemperatureOption>>,
    delayed: Vec<PendingScope>,
    // An eager declaration or option can fail before temperature is selected.
    // Keep its first error independently of subsequent redefinitions. A pass
    // containing one of these errors must never publish a netlist.
    provisional_error: Option<ParseError>,
    // Successful isolated operands from a failed group are replay candidates,
    // never published options. Ordinals prevent later cards being overwritten.
    recovered: [Option<(usize, Value)>; 2],
}

pub(in super::super) struct TemperatureOptionSink<'a> {
    pub(in super::super) plan: &'a mut TemperatureOptionPlan,
    pub(in super::super) depth: usize,
    pub(in super::super) origin: &'a NetlistSourceLocation,
}

impl TemperatureOptionSink<'_> {
    pub(in super::super) fn parse(
        &mut self,
        option: TemperatureOption,
        stream: &mut TokenStream,
        line: usize,
        params: &ParamContext,
        options: &mut SimulationOptions,
    ) -> Result<(), ParseError> {
        skip_commas(stream);
        let sign_width = usize::from(matches!(
            stream.peek().kind,
            TokenKind::Plus | TokenKind::Minus
        ));
        let expression = match &stream.peek_n(sign_width).kind {
            TokenKind::Expression(expression) => Some(expression.clone()),
            TokenKind::Ident(name)
                if params.get(name).is_none()
                    && parse_boolean_literal(name).is_none()
                    && crate::netlist::lexer::parse_spice_value(name).is_err() =>
            {
                Some(name.clone())
            }
            _ => None,
        };
        let deferred = if let Some(expression) = expression.filter(|expression| {
            crate::netlist::expr::needs_forward_reference_probe(expression, params)
        }) {
            // A failed evaluation can already have sampled a random function.
            // Probe on an isolated stream; immediate values are evaluated on
            // the live stream exactly once. Forward expressions draw only when
            // all required declaration scopes close, in their authored order.
            match stream.numeric_expression(&expression, &params.isolated_random_clone()) {
                Err(crate::netlist::expr::ExprError::UndefinedParam(_)) => {
                    let prepared = stream
                        .prepare_numeric_expression(&expression, params)
                        .map_err(|error| ParseError::InvalidValue(error.to_string()))?;
                    let mut bound_values = HashMap::new();
                    prepared.visit_runtime_parameters(|name| {
                        if let Some(value) = params.get_complex(name) {
                            bound_values.insert(name.to_owned(), value);
                        }
                    });
                    let sign = if matches!(stream.peek().kind, TokenKind::Minus) {
                        -1.0
                    } else {
                        1.0
                    };
                    for _ in 0..=sign_width {
                        stream.advance();
                    }
                    Some((prepared, bound_values, sign))
                }
                // Preserve the ordinary evaluator's lazy branches and errors.
                _ => None,
            }
        } else {
            None
        };
        self.plan.ordinal += 1;
        self.plan.last[option.index()] = self.plan.ordinal;
        if let Some((expression, bound_values, sign)) = deferred {
            self.plan
                .scopes
                .resize_with(self.plan.scopes.len().max(self.depth + 1), Vec::new);
            self.plan.scopes[self.depth].push(PendingTemperatureOption {
                option,
                ordinal: self.plan.ordinal,
                expression,
                bound_values,
                sign,
                origin: self.origin.clone(),
            });
        } else {
            let value = expect_value(stream, line, params)?;
            option.install(options, parse_celsius_option(option.name(), value, line)?);
        }
        Ok(())
    }
}

impl TemperatureOptionPlan {
    pub(in super::super) fn selected(&self, options: &SimulationOptions) -> ParserTemperatures {
        let recovered = |index: usize| {
            self.recovered[index]
                .filter(|(ordinal, _)| self.last[index] == *ordinal)
                .map(|(_, value)| value)
        };
        ParserTemperatures {
            temp: recovered(0).or(options.temp),
            tnom: recovered(1).or(options.tnom),
        }
    }

    pub(in super::super) fn retain_scope_error(
        &mut self,
        error: ParseWithAbortError,
    ) -> Result<(), ParseWithAbortError> {
        match error {
            error @ (ParseWithAbortError::Aborted
            | ParseWithAbortError::Parse(ParseError::ResourceLimit(_))) => Err(error),
            ParseWithAbortError::Parse(error) => {
                self.provisional_error.get_or_insert(error);
                Ok(())
            }
        }
    }

    pub(in super::super) fn retain_parameter_error(
        &mut self,
        error: crate::netlist::expr::ExprError,
        origin: &NetlistSourceLocation,
    ) {
        self.provisional_error
            .get_or_insert_with(|| ParseError::InvalidValue(format!("{origin}: {error}")));
    }

    pub(in super::super) fn retain_card_error(
        &mut self,
        error: ParseError,
        line: usize,
        origin: &NetlistSourceLocation,
    ) {
        self.provisional_error.get_or_insert_with(|| match error {
            ParseError::InvalidValue(message) => {
                ParseError::InvalidValue(format!("{origin}: {message}"))
            }
            error => source_map_logical_line_error(error, line, origin, true),
        });
    }

    pub(in super::super) fn take_error(&mut self) -> Option<ParseError> {
        self.provisional_error.take()
    }

    pub(in super::super) fn has_error(&self) -> bool {
        self.provisional_error.is_some()
    }

    pub(in super::super) fn prefer_error(
        &mut self,
        error: ParseWithAbortError,
    ) -> ParseWithAbortError {
        match error {
            error @ (ParseWithAbortError::Aborted
            | ParseWithAbortError::Parse(ParseError::ResourceLimit(_))) => error,
            error => self
                .take_error()
                .map(ParseWithAbortError::from)
                .unwrap_or(error),
        }
    }

    pub(in super::super) fn resolve_scope(
        &mut self,
        scopes: &mut LexicalScopes,
        root: &mut ParamContext,
        frames: &mut [SubcktFrame],
        options: &mut SimulationOptions,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let closing = scopes.current();
        let mut candidates = std::mem::take(&mut self.delayed);
        if let Some(entries) = self.scopes.get_mut(frames.len())
            && !entries.is_empty()
        {
            candidates.push(PendingScope {
                id: closing,
                entries: std::mem::take(entries),
            });
        }
        let mut first_error = None;
        for pending in candidates {
            ensure_parse_not_aborted(abort)?;
            // A sibling closing cannot complete a declaration owned by this
            // scope. Retrying there would also move statistical evaluations.
            if pending.id != closing && !scopes.is_descendant(pending.id, closing) {
                self.delayed.push(pending);
                continue;
            }
            match pending.bind(scopes, root, frames, abort)? {
                ScopeBinding::Complete(values) => {
                    ensure_parse_not_aborted(abort)?;
                    for (entry, value) in pending.entries.iter().zip(values) {
                        // Superseded active assignments are still evaluated and
                        // validated, but only the latest assignment is published.
                        if self.last[entry.option.index()] == entry.ordinal {
                            entry.option.install(options, value);
                        }
                    }
                }
                ScopeBinding::Failed { error, selected } => {
                    first_error.get_or_insert(error);
                    for (index, value) in selected.into_iter().enumerate() {
                        if let Some((ordinal, _)) = value
                            && self.last[index] == ordinal
                        {
                            self.recovered[index] = value;
                        }
                    }
                }
                ScopeBinding::Incomplete => {
                    scopes.retain();
                    self.delayed.push(pending);
                }
            }
        }
        first_error.map_or(Ok(()), |error| Err(error.into()))
    }
}

impl PendingTemperatureOption {
    fn error(&self, message: String) -> ParseError {
        ParseError::Syntax {
            line: self.origin.line,
            message: format!("{}: {}: {message}", self.origin, self.option.name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_group_candidates_never_publish_bindings_or_draw_from_live_streams() {
        let mut state = ParseState::new();
        state.params.set_random_seed(37);
        for (index, line) in [
            ".options temp={scale/(TNOM-27)} tnom={nominal+aunif(0,1)}",
            ".param scale={base+aunif(0,1)} nominal=55",
            ".param base=1",
        ]
        .into_iter()
        .enumerate()
        {
            process_line(
                line,
                index + 2,
                &NetlistSourceLocation::in_memory(index + 2),
                &mut state,
            )
            .unwrap();
        }
        let expected = state.params.isolated_random_clone();
        let probe = expected.isolated_random_clone();
        let _scale = eval_expression("aunif(0,1)", &probe).unwrap();
        let nominal = 55.0 + eval_expression("aunif(0,1)", &probe).unwrap();
        let error = state
            .temperature_options
            .resolve_scope(
                &mut state.scopes,
                &mut state.params,
                &mut state.subckt_stack,
                &mut state.options,
                &NoAbort,
            )
            .unwrap_err();
        assert!(error.to_string().contains("Division by zero"), "{error}");
        assert_eq!(state.options.temp, None);
        assert_eq!(state.options.tnom, None);
        assert_eq!(state.params.get("scale"), None);
        assert_eq!(
            state.temperature_options.selected(&state.options),
            ParserTemperatures {
                temp: None,
                tnom: Some(nominal)
            }
        );
        assert_eq!(
            eval_expression("aunif(0,1)", &state.params).unwrap(),
            eval_expression("aunif(0,1)", &expected).unwrap()
        );
    }

    #[test]
    fn incomplete_failed_and_cancelled_scope_probes_do_not_draw_or_publish() {
        for cancel in [false, true] {
            let mut state = ParseState::new();
            state.params.set_random_seed(37);
            for (index, line) in [
                ".subckt parent p",
                ".param shared={base+aunif(0,1)}",
                ".subckt child q",
                ".options temp={aunif(0,1)+shared} tnom={bad}",
            ]
            .into_iter()
            .enumerate()
            {
                process_line(
                    line,
                    index + 2,
                    &NetlistSourceLocation::in_memory(index + 2),
                    &mut state,
                )
                .unwrap();
            }
            let reference = state.params.isolated_random_clone();
            state
                .temperature_options
                .resolve_scope(
                    &mut state.scopes,
                    &mut state.params,
                    &mut state.subckt_stack,
                    &mut state.options,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(state.temperature_options.delayed.len(), 1);
            assert_eq!(state.options.temp, None);
            assert_eq!(state.options.tnom, None);
            assert_eq!(
                eval_expression("aunif(0,1)", &state.params).unwrap(),
                eval_expression("aunif(0,1)", &reference).unwrap()
            );
            for (index, line) in [".ends", ".param base=85 bad=-274"].into_iter().enumerate() {
                process_line(
                    line,
                    index + 6,
                    &NetlistSourceLocation::in_memory(index + 6),
                    &mut state,
                )
                .unwrap();
            }
            let cancelled = crate::abort_signal::CountingAbort::new(5);
            let abort: &dyn AbortSignal = if cancel { &cancelled } else { &NoAbort };
            let result = state.temperature_options.resolve_scope(
                &mut state.scopes,
                &mut state.params,
                &mut state.subckt_stack,
                &mut state.options,
                abort,
            );
            assert!(result.is_err());
            assert_eq!(matches!(result, Err(ParseWithAbortError::Aborted)), cancel);
            assert_eq!(cancelled.polls_after_abort(), 0);
            assert_eq!(state.options.temp, None);
            assert_eq!(state.options.tnom, None);
            assert_eq!(state.subckt_stack[0].local_params.get("shared"), None);
            assert_eq!(
                eval_expression("aunif(0,1)", &state.params).unwrap(),
                eval_expression("aunif(0,1)", &reference).unwrap()
            );
        }
    }

    #[test]
    fn scope_resolution_stops_at_the_first_cancellation_poll() {
        let mut plan = TemperatureOptionPlan::default();
        let mut params = ParamContext::new();
        let mut options = SimulationOptions::default();
        let origin = NetlistSourceLocation::in_memory(2);
        for _ in 0..3 {
            let mut stream = TokenStream::new(tokenize("{ambient}").unwrap());
            TemperatureOptionSink {
                plan: &mut plan,
                depth: 0,
                origin: &origin,
            }
            .parse(
                TemperatureOption::Temp,
                &mut stream,
                2,
                &params,
                &mut options,
            )
            .unwrap();
        }
        params.set("ambient", 85.0);
        let abort = crate::abort_signal::CountingAbort::new(1);
        assert!(matches!(
            plan.resolve_scope(
                &mut LexicalScopes::default(),
                &mut params,
                &mut [],
                &mut options,
                &abort
            ),
            Err(ParseWithAbortError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
        assert_eq!(options.temp, None);
    }
}
