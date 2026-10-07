//! Reconcile lexical temperature hints with the ordinary scoped parser.
//!
//! Hints avoid replay for ordinary literal options. Only the full parser can
//! decide which source/conditional cards apply and evaluate parameter values.
//! A replay starts from immutable input and a fresh statistical stream; it
//! never evaluates declarations on a cloned, shared random counter.

use super::*;

mod options;
pub(super) use options::{TemperatureOption, TemperatureOptionPlan, TemperatureOptionSink};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct ParserTemperatures {
    temp: Option<Value>,
    tnom: Option<Value>,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum TemperatureDirective {
    Single(Value),
    Sweep,
}

impl TemperatureDirective {
    pub(super) fn from_values(values: &[Value]) -> Self {
        match values {
            [single] => Self::Single(*single),
            _ => Self::Sweep,
        }
    }
}

/// A failed completion can still discover an authoritative temperature. The
/// public error survives unless a fresh parse confirms a different setting.
#[derive(Debug)]
pub(super) struct TemperaturePassError {
    error: Box<ParseWithAbortError>,
    selected: Option<ParserTemperatures>,
}

impl From<ParseWithAbortError> for TemperaturePassError {
    fn from(error: ParseWithAbortError) -> Self {
        Self {
            error: Box::new(error),
            selected: None,
        }
    }
}

impl From<ParseError> for TemperaturePassError {
    fn from(error: ParseError) -> Self {
        ParseWithAbortError::from(error).into()
    }
}

impl TemperaturePassError {
    pub(super) fn after_completion(
        error: ParseWithAbortError,
        used: ParserTemperatures,
        mut resolved: ParserTemperatures,
        last_directive: Option<TemperatureDirective>,
        overrides: &[ParameterOverride],
    ) -> Self {
        if matches!(
            &error,
            ParseWithAbortError::Aborted | ParseWithAbortError::Parse(ParseError::ResourceLimit(_))
        ) {
            return error.into();
        }
        if let Some(TemperatureDirective::Single(single)) = last_directive {
            resolved.temp = Some(single);
        }
        // Overrides were validated before parsing; retain a validation error
        // if that contract ever changes instead of choosing another value.
        if let Err(error) = resolved.apply_override(overrides) {
            return error.into();
        }
        Self {
            error: Box::new(error),
            selected: (used != resolved).then_some(resolved),
        }
    }
}

impl ParserTemperatures {
    pub(super) fn apply_override(
        &mut self,
        overrides: &[ParameterOverride],
    ) -> Result<(), ParseError> {
        if let Some(temp) = replayed_temperature(overrides)? {
            self.temp = Some(temp);
        }
        Ok(())
    }

    fn resolved(netlist: &Netlist) -> Self {
        Self {
            temp: netlist.options.temp,
            tnom: netlist.options.tnom,
        }
    }

    pub(super) fn install(self, params: &mut ParamContext) {
        if let Some(temp) = self.temp {
            install_temperature(params, temp);
        }
        if let Some(tnom) = self.tnom {
            params.set("TNOM", tnom);
        }
    }
}

/// Resolve both global temperatures before publishing any eagerly parsed
/// values. An acyclic dependence between TEMP and TNOM needs at most three
/// passes (discovery, dependency propagation, confirmation). A changing or
/// cyclic selection is an error, never a silently accepted provisional deck.
pub(super) fn parse_with_consistent_temperatures(
    mut parse: impl FnMut(
        Option<ParserTemperatures>,
    ) -> Result<(Netlist, ParserTemperatures), TemperaturePassError>,
) -> Result<Netlist, ParseWithAbortError> {
    let mut selected = None;
    for _ in 0..3 {
        let (netlist, used) = match parse(selected) {
            Ok(result) => result,
            Err(TemperaturePassError {
                selected: Some(temperatures),
                ..
            }) => {
                selected = Some(temperatures);
                continue;
            }
            Err(error) => return Err(*error.error),
        };
        let resolved = ParserTemperatures::resolved(&netlist);
        if used == resolved {
            return Ok(netlist);
        }
        selected = Some(resolved);
    }
    Err(ParseError::InvalidValue(
        "TEMP/TNOM selection changes when temperature-dependent expressions are reevaluated; remove circular temperature options or temperature-dependent option selection".into(),
    ).into())
}

pub(super) fn replayed_temperature(
    overrides: &[ParameterOverride],
) -> Result<Option<Value>, ParseError> {
    overrides
        .iter()
        .rev()
        .find(|parameter| !parameter.global && parameter.name.eq_ignore_ascii_case("TEMP"))
        .map(|parameter| parse_celsius_option("TEMP", parameter.value, 0))
        .transpose()
}

/// Physical study coordinates take precedence over authored options and .TEMP.
pub(super) fn apply_replayed_temperature(state: &mut ParseState) -> Result<(), ParseError> {
    if let Some(temperature) = replayed_temperature(&state.parameter_overrides)? {
        state.options.temp = Some(temperature);
        install_temperature(&mut state.params, temperature);
    }
    Ok(())
}

fn install_temperature(params: &mut ParamContext, temperature: Value) {
    params.set("TEMP", temperature);
    params.set("TEMPER", temperature);
    params.set(
        "VT",
        crate::constants::thermal_voltage(crate::constants::celsius_to_kelvin(temperature)),
    );
}

/// A lexical scan supplies hints only. It must not evaluate an expression,
/// consume a random draw, report a suppressed card's error, or author options.
pub(super) fn prescan_temperature_options_with_abort(
    lines: &[&str],
    body_start: usize,
    allow_non_semicolon_comments: bool,
    xyce_syntax: bool,
    abort: &dyn AbortSignal,
) -> Result<ParserTemperatures, ParseWithAbortError> {
    let mut hints = ParserTemperatures::default();
    for_each_options_line_with_abort(
        lines,
        body_start,
        allow_non_semicolon_comments,
        xyce_syntax,
        abort,
        |line, line_num| {
            scan_temperature_option_line(line, line_num, &mut hints);
            Ok(())
        },
    )?;
    Ok(hints)
}

fn scan_temperature_option_line(line: &str, line_num: usize, hints: &mut ParserTemperatures) {
    let Ok(tokens) = tokenize(line) else {
        return;
    };
    let mut stream = TokenStream::new(tokens);
    stream.advance();
    let mut option_package: Option<String> = None;
    while !stream.is_eof() {
        skip_commas(&mut stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }
        let TokenKind::Ident(key) = &stream.peek().kind else {
            stream.advance();
            continue;
        };
        let key = key.to_ascii_uppercase();
        stream.advance();
        let has_equals = stream.consume(&TokenKind::Equals);
        if !has_equals && option_package_key_is_known(&key) {
            option_package = Some(key);
            continue;
        }
        let literal = match (&stream.peek().kind, &stream.peek_n(1).kind) {
            (TokenKind::Number(value), _) => Some((*value, 1)),
            (TokenKind::Plus, TokenKind::Number(value)) => Some((*value, 2)),
            (TokenKind::Minus, TokenKind::Number(value)) => Some((-value, 2)),
            _ => None,
        };
        if matches!(key.as_str(), "TEMP" | "TNOM")
            && matches!(option_package.as_deref(), None | Some("DEVICE"))
            && let Some((value, width)) = literal
            && let Ok(value) = parse_celsius_option(&key, value, line_num)
        {
            if key == "TEMP" {
                hints.temp = Some(value);
            } else {
                hints.tnom = Some(value);
            }
            for _ in 0..width {
                stream.advance();
            }
            continue;
        }
        if has_equals && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            stream.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eager_parameter_recovery_stops_at_every_cancellation_boundary() {
        let source = "Cancellation during discovery\n.param scale={1/(TEMP-27)}\n.temp {ambient}\n.param ambient=85\n.end\n";
        for limit in 0..512 {
            let abort = crate::abort_signal::CountingAbort::new(limit);
            let result = super::super::parse_netlist_with_options_and_abort(
                source,
                NetlistParseOptions::default(),
                &abort,
            );
            assert_eq!(abort.polls_after_abort(), 0, "poll limit {limit}");
            match result {
                Err(ParseWithAbortError::Aborted) => {}
                Ok(netlist) => {
                    assert!(limit > 20, "both discovery and replay must be exercised");
                    assert_eq!(netlist.params.get("scale"), Some(1.0 / 58.0));
                    return;
                }
                error => panic!("unexpected result at poll limit {limit}: {error:?}"),
            }
        }
        panic!("cancellation coverage never reached successful completion");
    }

    #[test]
    fn option_recovery_stops_at_every_cancellation_boundary() {
        let source = "Option cancellation\n.options OUTPUT INITIAL_INTERVAL={1/(TEMP-27)} 10 2 SNAPSHOTS=1 DEVICE TEMP={ambient}\n.param ambient=85\n.end\n";
        for limit in 0..512 {
            let abort = crate::abort_signal::CountingAbort::new(limit);
            let result = super::super::parse_netlist_with_options_and_abort(
                source,
                NetlistParseOptions::default(),
                &abort,
            );
            assert_eq!(abort.polls_after_abort(), 0, "poll limit {limit}");
            match result {
                Err(ParseWithAbortError::Aborted) => {}
                Ok(netlist) => {
                    assert!(limit > 20);
                    assert_eq!(netlist.options.temp, Some(85.0));
                    assert_eq!(netlist.options.output_snapshots, Some(true));
                    return;
                }
                error => panic!("unexpected result at poll limit {limit}: {error:?}"),
            }
        }
        panic!("cancellation coverage never reached successful completion");
    }

    #[test]
    fn pending_group_discovery_stops_at_every_cancellation_boundary() {
        for source in [
            "Root group\n.options temp={scale/(TNOM-27)} tnom={nominal}\n.param scale=1 nominal=55\n.end\n",
            "Delayed group\n.subckt parent p\n.subckt child q\n.options temp={scale/(TNOM-27)} tnom={nominal}\n.param scale=1\n.ends\n.param nominal=55\n.ends\n.end\n",
        ] {
            let mut completed = false;
            for limit in 0..1024 {
                let abort = crate::abort_signal::CountingAbort::new(limit);
                let result = super::super::parse_netlist_with_options_and_abort(
                    source,
                    NetlistParseOptions::default(),
                    &abort,
                );
                assert_eq!(abort.polls_after_abort(), 0, "poll limit {limit}");
                match result {
                    Err(ParseWithAbortError::Aborted) => {}
                    Ok(netlist) => {
                        assert!(limit > 20);
                        assert_eq!(netlist.options.temp, Some(1.0 / 28.0));
                        assert_eq!(netlist.options.tnom, Some(55.0));
                        completed = true;
                        break;
                    }
                    error => panic!("unexpected result at poll limit {limit}: {error:?}"),
                }
            }
            assert!(completed, "cancellation coverage never completed: {source}");
        }
    }

    #[test]
    fn physical_override_takes_precedence_over_a_failed_groups_candidate() {
        let netlist = parse_netlist_with_parameter_overrides_and_abort(
            "Physical pending group\n.options temp={scale/(TNOM-27)} tnom={nominal}\n.param scale=1 nominal=55 observed={TEMP}\n.end\n",
            NetlistParseOptions::default(),
            &[ParameterOverride { name: "TEMP".into(), value: 85.0, global: false, direction: false }],
            &NoAbort,
        ).unwrap();
        assert_eq!(netlist.options.temp, Some(85.0));
        assert_eq!(netlist.options.tnom, Some(55.0));
        assert_eq!(netlist.params.get("observed"), Some(85.0));
    }

    #[test]
    fn physical_temperature_override_survives_failed_analysis_completion() {
        let source = "Physical replay\n.ac lin 1 {1/(TNOM-27)} 100\n.temp 27\n.param nominal=55\n.options tnom={nominal}\n.end\n";
        let netlist = parse_netlist_with_parameter_overrides_and_abort(
            source,
            NetlistParseOptions::default(),
            &[ParameterOverride {
                name: "TEMP".into(),
                value: 85.0,
                global: false,
                direction: false,
            }],
            &NoAbort,
        )
        .unwrap();
        assert_eq!(netlist.options.temp, Some(85.0));
        assert_eq!(netlist.options.tnom, Some(55.0));
        assert_eq!(netlist.params.get("TEMP"), Some(85.0));
        let AnalysisCommand::Ac { start_freq, .. } = netlist.analyses[0] else {
            panic!("AC")
        };
        assert_eq!(start_freq, 1.0 / 28.0);
    }

    #[test]
    fn completion_cancellation_and_resource_errors_never_request_replay() {
        let limit = crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            2,
            1,
        )
        .unwrap_err();
        for error in [ParseWithAbortError::Aborted, ParseError::from(limit).into()] {
            let mut failure = Some(error);
            let mut calls = 0;
            let result = parse_with_consistent_temperatures(|_| {
                calls += 1;
                Err(TemperaturePassError::after_completion(
                    failure.take().expect("terminal errors must not replay"),
                    ParserTemperatures::default(),
                    ParserTemperatures {
                        temp: Some(85.0),
                        ..Default::default()
                    },
                    None,
                    &[],
                ))
            });
            assert_eq!(calls, 1);
            assert!(matches!(
                result,
                Err(ParseWithAbortError::Aborted
                    | ParseWithAbortError::Parse(ParseError::ResourceLimit(_)))
            ));
        }
    }

    #[test]
    fn replay_after_a_provisional_analysis_error_remains_cancellable() {
        let source = "Provisional analysis\n.ac lin 1 {1/(TEMP-27)} {1/(TEMP-27)}\n.param ambient=85\n.options temp={ambient}\n.end\n";
        let counter = crate::abort_signal::CountingAbort::new(usize::MAX);
        let failure = parse_netlist_impl(
            source,
            NetlistParseOptions::default(),
            None,
            None,
            ParserReplay::default(),
            &[],
            &counter,
        )
        .unwrap_err();
        assert_eq!(failure.selected.unwrap().temp, Some(85.0));
        let abort = crate::abort_signal::CountingAbort::new(counter.count());
        assert!(matches!(
            parse_netlist_with_options_and_abort(source, NetlistParseOptions::default(), &abort),
            Err(ParseWithAbortError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
    }

    #[test]
    fn literal_fast_path_and_parameter_dependency_pass_counts_are_bounded() {
        for (body, expected) in [
            (".options temp=85 tnom=35\n", 1),
            (".param ambient=85\n.options temp={ambient}\n", 2),
            (
                ".param nominal=55\n.options temp={TNOM+10} tnom={nominal}\n",
                3,
            ),
        ] {
            let source = format!("Temperature pass count\n{body}.end\n");
            let mut passes = 0;
            parse_with_consistent_temperatures(|temperatures| {
                passes += 1;
                parse_netlist_impl(
                    &source,
                    NetlistParseOptions::default(),
                    None,
                    None,
                    ParserReplay {
                        seed: None,
                        temperatures,
                    },
                    &[],
                    &NoAbort,
                )
            })
            .unwrap();
            assert_eq!(passes, expected, "{body}");
        }
    }

    #[test]
    fn temperature_reconciliation_replay_remains_cancellable() {
        let source = "Temperature replay\n.param ambient=85\n.options temp={ambient}\n.end\n";
        let counter = crate::abort_signal::CountingAbort::new(usize::MAX);
        let (candidate, used) = parse_netlist_impl(
            source,
            NetlistParseOptions::default(),
            None,
            None,
            ParserReplay::default(),
            &[],
            &counter,
        )
        .unwrap();
        assert_ne!(used, ParserTemperatures::resolved(&candidate));
        let abort = crate::abort_signal::CountingAbort::new(counter.count());
        assert!(matches!(
            parse_netlist_with_options_and_abort(source, NetlistParseOptions::default(), &abort),
            Err(ParseWithAbortError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
    }

    #[test]
    fn absent_nominal_option_preserves_a_root_parameter_override() {
        let source = "Nominal parameter override\n.param nominal_read={TNOM}\n.end\n";
        let netlist = parse_netlist_with_parameter_overrides_and_abort(
            source,
            NetlistParseOptions::default(),
            &[ParameterOverride {
                name: "TNOM".into(),
                value: 55.0,
                global: false,
                direction: false,
            }],
            &NoAbort,
        )
        .unwrap();
        assert_eq!(netlist.params.get("nominal_read"), Some(55.0));
        assert_eq!(netlist.options.tnom, None);
    }
}
