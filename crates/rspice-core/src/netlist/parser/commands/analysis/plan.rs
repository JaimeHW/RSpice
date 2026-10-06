//! Ordered analysis bindings. Ready cards keep their eager values; failed
//! cards bind in their authored scope after declarations are complete.
use super::*;

mod scoped;
use crate::netlist::parser::scopes::LexicalScopes;

#[derive(Debug, Default)]
pub(in crate::netlist::parser) struct AnalysisCardPlan {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    card: Card,
    line: usize,
    origin: NetlistSourceLocation,
    output_position: usize,
    diagnostic_position: usize,
}

#[derive(Debug)]
enum Card {
    Ready(Box<ParsedAnalysisCard>),
    Pending(Box<PendingCard>),
}

#[derive(Debug)]
struct PendingCard {
    scope: usize,
    command: String,
    stream: TokenStream,
    logical_line: String,
    max_analysis_points: usize,
    values: Vec<(String, crate::ComplexValue)>,
    strings: Vec<(String, String)>,
    functions: Vec<crate::netlist::expr::FunctionDef>,
}

impl PendingCard {
    fn capture(
        scope: usize,
        command: &str,
        stream: TokenStream,
        context: AnalysisCardContext<'_>,
    ) -> Self {
        let mut values = context
            .params
            .all_params()
            .into_iter()
            .filter_map(|(name, _)| context.params.get_complex(&name).map(|value| (name, value)))
            .collect::<Vec<_>>();
        // Builtins need not have stored bindings, but were already readable at
        // this card. Temperature reconciliation may replay the whole parser.
        for name in ["TEMP", "TEMPER", "TNOM", "VT"] {
            if let Some(value) = context.params.get_complex(name) {
                values.push((name.to_owned(), value));
            }
        }
        Self {
            scope,
            command: command.to_owned(),
            stream,
            logical_line: context.logical_line.to_owned(),
            max_analysis_points: context.max_analysis_points,
            values,
            strings: context.params.all_string_params(),
            functions: context.params.all_functions(),
        }
    }

    fn context(&self, completed: &ParamContext) -> ParamContext {
        // Copy only resolved authored bindings over the completed scope. An
        // old unresolved definition must not overwrite its selected final value.
        let mut params = completed.clone();
        for (name, value) in &self.values {
            params.set_complex(name, *value);
        }
        for (name, value) in &self.strings {
            params.set_string(name, value.clone());
        }
        for function in &self.functions {
            params.import_function(function.clone());
        }
        params
    }
}

impl AnalysisCardPlan {
    pub(in crate::netlist::parser) fn parse(
        &mut self,
        scopes: &mut LexicalScopes,
        command: &str,
        stream: &mut TokenStream,
        context: AnalysisCardContext<'_>,
        sink: AnalysisCardSink<'_>,
    ) -> Result<bool, ParseError> {
        if AnalysisHead::parse(command).is_none() {
            return Ok(false);
        }
        let queued = !self.entries.is_empty();
        let mut probe = stream.clone();
        let parsing = AnalysisCardContext {
            lin_exists: !queued && context.lin_exists,
            current_noise: if queued { None } else { context.current_noise },
            ..context
        };
        let card = match ParsedAnalysisCard::parse(command, &mut probe, parsing) {
            Ok(Some(card)) if !queued => {
                *stream = probe;
                card.publish(sink);
                return Ok(true);
            }
            Ok(Some(card)) => Card::Ready(Box::new(card)),
            Ok(None) => unreachable!("recognized analysis head"),
            // Grammar and value errors can be indistinguishable until binding
            // completes (notably optional values and typed periodic fields).
            // Retain the original tokens and run the authoritative grammar
            // again, instead of classifying human-readable error strings.
            Err(error) if !matches!(error, ParseError::ResourceLimit(_)) => {
                Card::Pending(Box::new(PendingCard::capture(
                    scopes.retain(),
                    command,
                    stream.clone(),
                    context,
                )))
            }
            Err(error) => return Err(error),
        };
        self.entries.push(Entry {
            card,
            line: context.line_num,
            origin: context.origin.clone(),
            output_position: sink.output_requests.len(),
            diagnostic_position: sink.diagnostics.len(),
        });
        stream.skip_to_eol();
        Ok(true)
    }

    pub(in crate::netlist::parser) fn complete(
        &mut self,
        scopes: &mut LexicalScopes,
        params: &ParamContext,
        sink: AnalysisCardSink<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        if self.entries.is_empty() {
            return Ok(());
        }
        let mut lin_exists = sink.lin_analysis.is_some();
        let mut current_noise = sink.options.transient_noise;
        let mut ready = Vec::new();
        for entry in std::mem::take(&mut self.entries) {
            ensure_parse_not_aborted(abort)?;
            let card = match entry.card {
                Card::Ready(card) => {
                    card.validate_constraints(entry.line, lin_exists, current_noise)
                        .map_err(|error| located_error(error, entry.line, &entry.origin))?;
                    *card
                }
                Card::Pending(pending) if pending.scope != 0 => scoped::bind(
                    scopes,
                    &pending,
                    AnalysisCardContext {
                        line_num: entry.line,
                        logical_line: &pending.logical_line,
                        params,
                        max_analysis_points: pending.max_analysis_points,
                        origin: &entry.origin,
                        lin_exists,
                        current_noise,
                    },
                    abort,
                )?,
                Card::Pending(pending) => {
                    let bound = pending.context(params);
                    ParsedAnalysisCard::parse(
                        &pending.command,
                        &mut pending.stream.clone(),
                        AnalysisCardContext {
                            line_num: entry.line,
                            logical_line: &pending.logical_line,
                            params: &bound,
                            max_analysis_points: pending.max_analysis_points,
                            origin: &entry.origin,
                            lin_exists,
                            current_noise,
                        },
                    )
                    .map_err(|error| located_error(error, entry.line, &entry.origin))?
                    .expect("saved analysis head")
                }
            };
            lin_exists |= card.lin_analysis.is_some();
            current_noise = card.transient_noise.or(current_noise);
            ready.push((entry.output_position, entry.diagnostic_position, card));
        }
        ensure_parse_not_aborted(abort)?;
        // Non-analysis directives have continued to append outputs/warnings.
        // Merge once at their saved boundaries; repeated Vec::insert would be
        // quadratic and scope-close order must not become publication order.
        let mut outputs = std::mem::take(sink.output_requests).into_iter();
        let mut diagnostics = std::mem::take(sink.diagnostics).into_iter();
        let mut output_cursor = 0;
        let mut diagnostic_cursor = 0;
        for (output_position, diagnostic_position, card) in ready {
            ensure_parse_not_aborted(abort)?;
            sink.output_requests
                .extend(outputs.by_ref().take(output_position - output_cursor));
            sink.diagnostics.extend(
                diagnostics
                    .by_ref()
                    .take(diagnostic_position - diagnostic_cursor),
            );
            output_cursor = output_position;
            diagnostic_cursor = diagnostic_position;
            card.publish(AnalysisCardSink {
                analyses: sink.analyses,
                monte_carlo_source_cards: sink.monte_carlo_source_cards,
                lin_analysis: sink.lin_analysis,
                fft_analyses: sink.fft_analyses,
                output_requests: sink.output_requests,
                diagnostics: sink.diagnostics,
                options: sink.options,
            });
        }
        sink.output_requests.extend(outputs);
        sink.diagnostics.extend(diagnostics);
        Ok(())
    }
}

fn located_error(error: ParseError, line: usize, origin: &NetlistSourceLocation) -> ParseError {
    match error {
        ParseError::InvalidValue(message) => {
            ParseError::InvalidValue(format!("{origin}: {message}"))
        }
        error => source_map_logical_line_error(error, line, origin, true),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_scoped_binding_does_not_draw_or_publish_partial_effects() {
        for cancel in [false, true] {
            let mut state = ParseState::new();
            state.params.set_random_seed(73);
            for (index, line) in [
                ".subckt child p",
                ".DC V1 {aunif(0,1)+stop} 100 1 invalid",
                ".param stop={base+aunif(0,1)}",
                ".param base=5",
                ".ends",
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
            let cancelled = crate::abort_signal::CountingAbort::new(5);
            let abort: &dyn AbortSignal = if cancel { &cancelled } else { &NoAbort };
            let result = state.analysis_cards.complete(
                &mut state.scopes,
                &state.params,
                AnalysisCardSink {
                    analyses: &mut state.analyses,
                    monte_carlo_source_cards: &mut state.monte_carlo_source_cards,
                    lin_analysis: &mut state.lin_analysis,
                    fft_analyses: &mut state.fft_analyses,
                    output_requests: &mut state.output_requests,
                    diagnostics: &mut state.diagnostics,
                    options: &mut state.options,
                },
                abort,
            );
            assert!(result.is_err());
            assert_eq!(matches!(result, Err(ParseWithAbortError::Aborted)), cancel);
            assert_eq!(cancelled.polls_after_abort(), 0);
            assert!(state.analyses.is_empty());
            assert!(state.output_requests.is_empty());
            assert_eq!(
                eval_expression("aunif(0,1)", &state.params).unwrap(),
                eval_expression("aunif(0,1)", &expected).unwrap()
            );
        }
    }

    #[test]
    fn cancellation_precedes_deferred_binding_and_publication() {
        let mut state = ParseState::new();
        state.params.set_random_seed(37);
        process_line(
            ".DC V1 {aunif(0,1)} {stop} 1",
            2,
            &NetlistSourceLocation::in_memory(2),
            &mut state,
        )
        .unwrap();
        process_line(
            ".FFT V(out) NP=32",
            3,
            &NetlistSourceLocation::in_memory(3),
            &mut state,
        )
        .unwrap();
        state.params.set("stop", 2.0);
        let expected = state.params.isolated_random_clone();
        let abort = crate::abort_signal::CountingAbort::new(0);
        let result = state.analysis_cards.complete(
            &mut state.scopes,
            &state.params,
            AnalysisCardSink {
                analyses: &mut state.analyses,
                monte_carlo_source_cards: &mut state.monte_carlo_source_cards,
                lin_analysis: &mut state.lin_analysis,
                fft_analyses: &mut state.fft_analyses,
                output_requests: &mut state.output_requests,
                diagnostics: &mut state.diagnostics,
                options: &mut state.options,
            },
            &abort,
        );
        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert_eq!(abort.polls_after_abort(), 0);
        assert!(state.analyses.is_empty());
        assert!(state.output_requests.is_empty());
        assert!(state.fft_analyses.is_empty());
        assert_eq!(
            eval_expression("aunif(0,1)", &state.params).unwrap(),
            eval_expression("aunif(0,1)", &expected).unwrap()
        );
    }
}
