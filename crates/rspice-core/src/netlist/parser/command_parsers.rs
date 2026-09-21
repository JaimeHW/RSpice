use super::analysis_card_scan::*;
use super::*;

pub(super) fn parse_device_initial_condition_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    origin: &NetlistSourceLocation,
    directive: &mut Option<DeviceInitialConditionDirective>,
) -> Result<(), ParseError> {
    if let Some(first) = directive {
        return Err(ParseError::DeviceInitialCondition(Box::new(
            DeviceInitialConditionError::DuplicateDirective {
                first: first.origin.clone(),
                duplicate: origin.clone(),
            },
        )));
    }

    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Err(ParseError::DeviceInitialCondition(Box::new(
            DeviceInitialConditionError::MissingInformation {
                origin: origin.clone(),
            },
        )));
    }

    if matches!(&stream.peek().kind, TokenKind::Ident(value) if value.eq_ignore_ascii_case("FILE"))
    {
        stream.advance();
        let requested_path = take_authored_initcond_path(stream).ok_or_else(|| {
            ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: "FILE requires one path".to_string(),
                },
            ))
        })?;
        if requested_path.trim().is_empty()
            || !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
        {
            return Err(ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: "FILE requires exactly one non-empty path".to_string(),
                },
            )));
        }
        *directive = Some(DeviceInitialConditionDirective {
            origin: origin.clone(),
            source: DeviceInitialConditionSource::File {
                requested_path,
                resolved_path: None,
                content_identity: None,
            },
            entries: Vec::new(),
        });
        return Ok(());
    }

    let entries = parse_device_initial_condition_entries(stream, line_num, params, origin)?;
    *directive = Some(DeviceInitialConditionDirective {
        origin: origin.clone(),
        source: DeviceInitialConditionSource::Inline,
        entries,
    });
    Ok(())
}

/// A quoted path as it was written, with its quotes taken off, or `None` when
/// the next token is not a quoted run.
///
/// Every directive that names a file reads its path through here, because a
/// path is literal. The lexer decodes a backslash inside a string literal as
/// an escape and drops it, which turns each Windows path into one that does
/// not exist: `"C:\meas\step.csv"` decodes to `C:measstep.csv`. What the
/// author wrote between the quotes is what the file is called.
pub(super) fn quoted_path_lexeme(stream: &mut TokenStream) -> Option<String> {
    let raw = stream.peek().lexeme.as_str();
    let mut characters = raw.chars();
    let opening = characters.next()?;
    let closing = characters.next_back()?;
    if !matches!(opening, '"' | '\'') || opening != closing {
        return None;
    }
    let path = raw[opening.len_utf8()..raw.len() - closing.len_utf8()].to_owned();
    stream.advance();
    Some(path)
}

fn take_authored_initcond_path(stream: &mut TokenStream) -> Option<String> {
    quoted_path_lexeme(stream).or_else(|| take_authored_initcond_token(stream))
}

fn take_authored_initcond_token(stream: &mut TokenStream) -> Option<String> {
    let first = stream.peek().clone();
    if matches!(first.kind, TokenKind::Newline | TokenKind::Eof) {
        return None;
    }
    let mut path = first.lexeme;
    let mut end = first.span.end;
    stream.advance();
    while stream.peek().span.start == end
        && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
    {
        let token = stream.peek().clone();
        path.push_str(&token.lexeme);
        end = token.span.end;
        stream.advance();
    }
    Some(path)
}

pub(super) fn parse_device_initial_condition_entries(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    origin: &NetlistSourceLocation,
) -> Result<Vec<DeviceInitialConditionEntry>, ParseError> {
    let mut entries = Vec::new();

    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let device = take_authored_initcond_token(stream).ok_or_else(|| {
            ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: "expected a fully qualified device name".to_string(),
                },
            ))
        })?;
        let keyword = expect_ident(stream, line_num).map_err(|_| {
            ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: format!("device '{device}' must be followed by IC=<value>"),
                },
            ))
        })?;
        if !keyword.eq_ignore_ascii_case("IC") || !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail: format!("device '{device}' must be followed by IC=<value>"),
                },
            )));
        }

        let mut values = Vec::new();
        loop {
            let value = expect_value(stream, line_num, params).map_err(|error| {
                ParseError::DeviceInitialCondition(Box::new(
                    DeviceInitialConditionError::MalformedDirective {
                        origin: origin.clone(),
                        detail: format!("device '{device}' has an invalid IC value: {error}"),
                    },
                ))
            })?;
            if !value.is_finite() {
                return Err(ParseError::DeviceInitialCondition(Box::new(
                    DeviceInitialConditionError::NonFiniteValue {
                        origin: origin.clone(),
                        device: device.clone(),
                        value_index: values.len() + 1,
                        value,
                    },
                )));
            }
            values.push(value);
            if !stream.consume(&TokenKind::Comma) {
                break;
            }
        }

        entries.push(DeviceInitialConditionEntry {
            device,
            values,
            origin: origin.clone(),
        });
    }

    if entries.is_empty() {
        return Err(ParseError::DeviceInitialCondition(Box::new(
            DeviceInitialConditionError::MissingInformation {
                origin: origin.clone(),
            },
        )));
    }
    Ok(entries)
}

pub(super) fn parse_step_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<StepCommand, ParseError> {
    skip_commas(stream);

    // Check for sweep type prefix
    let first = expect_ident(stream, line_num)?;
    let first_upper = first.to_uppercase();
    if first_upper == "DATA" {
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".STEP DATA requires DATA=<table-name>".to_string(),
            });
        }
        let table_name = expect_ident(stream, line_num)?;
        return Ok(StepCommand {
            target: StepTarget::Param,
            name: table_name.clone(),
            param_name: None,
            sweep: StepSweep::Data { table_name },
        });
    }

    let (sweep_prefix, target_str, mut name) = match first_upper.as_str() {
        "DEC" | "OCT" | "LIN" => {
            let target = expect_ident(stream, line_num)?;
            let target_upper = target.to_uppercase();
            match target_upper.as_str() {
                "PARAM" | "MODEL" => {
                    let name = expect_ident(stream, line_num)?;
                    (Some(first_upper), target_upper, name)
                }
                "TEMP" => (Some(first_upper), target_upper, "TEMP".to_string()),
                _ => (Some(first_upper), "DEVICE".to_string(), target),
            }
        }
        "PARAM" | "MODEL" => {
            let name = expect_ident(stream, line_num)?;
            (None, first_upper, name)
        }
        "TEMP" => (None, first_upper, "TEMP".to_string()),
        _ => {
            // Keep an unqualified name unresolved until the complete deck has
            // been parsed. A later .PARAM makes it a global sweep; otherwise
            // the name remains eligible for a natural device/model target.
            (None, "DEVICE".to_string(), first)
        }
    };

    let target = match target_str.as_str() {
        "PARAM" => StepTarget::Param,
        "MODEL" => StepTarget::Model,
        "TEMP" => StepTarget::Temp,
        _ => StepTarget::Device,
    };

    let mut param_name: Option<String> = None;
    if target == StepTarget::Device
        && let Some((device_name, device_param)) = name
            .rsplit_once(':')
            .map(|(device_name, device_param)| (device_name.to_string(), device_param.to_string()))
    {
        if device_name.is_empty()
            || device_param.is_empty()
            || device_name.split(':').any(str::is_empty)
        {
            return Err(ParseError::Syntax {
                line: line_num,
                message:
                    "Malformed .STEP device parameter target; expected device[:child...]:param"
                        .to_string(),
            });
        }
        name = device_name.to_string();
        param_name = Some(device_param.to_string());
    }

    match target {
        StepTarget::Model => {
            param_name = Some(expect_ident(stream, line_num)?);
        }
        StepTarget::Device => {
            // Optional device-parameter spec:
            // - .STEP R1(<param>) ...
            // - .STEP R1 <param> ...
            if param_name.is_none() && stream.consume(&TokenKind::LParen) {
                let pname = expect_ident(stream, line_num)?;
                if !stream.consume(&TokenKind::RParen) {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Expected ')' after .STEP device parameter name".to_string(),
                    });
                }
                param_name = Some(pname);
            } else if param_name.is_none()
                && let TokenKind::Ident(candidate) = &stream.peek().kind
            {
                let candidate_upper = candidate.to_ascii_uppercase();
                let reserved = ["LIST", "LIN", "DEC", "OCT", "PARAM", "MODEL", "TEMP"];
                let next_is_value_like = matches!(
                    stream.peek_n(1).kind,
                    TokenKind::Number(_)
                        | TokenKind::Expression(_)
                        | TokenKind::Plus
                        | TokenKind::Minus
                ) || matches!(
                    &stream.peek_n(1).kind,
                    TokenKind::Ident(next) if next.eq_ignore_ascii_case("LIST")
                );
                if !reserved.contains(&candidate_upper.as_str()) && next_is_value_like {
                    param_name = Some(candidate.clone());
                    stream.advance();
                }
            }
        }
        _ => {}
    }

    // Check for LIST keyword
    skip_commas(stream);
    let is_list = if let TokenKind::Ident(s) = &stream.peek().kind {
        if s.eq_ignore_ascii_case("LIST") {
            stream.advance();
            true
        } else {
            false
        }
    } else {
        false
    };

    let sweep = if is_list {
        // Parse list of values
        let mut values = Vec::new();
        while let Some(v) = try_signed_value(stream, params) {
            values.push(v);
        }
        if values.is_empty() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "LIST requires at least one value".to_string(),
            });
        }
        StepSweep::List(values)
    } else {
        // Parse start stop increment/points
        let start = expect_value(stream, line_num, params)?;
        let stop = expect_value(stream, line_num, params)?;
        let step_or_points = expect_value(stream, line_num, params)?;

        match sweep_prefix.as_deref() {
            Some("DEC") => StepSweep::Decade {
                points_per_decade: parse_step_points_per_interval(step_or_points, "DEC", line_num)?,
                start,
                stop,
            },
            Some("OCT") => StepSweep::Octave {
                points_per_octave: parse_step_points_per_interval(step_or_points, "OCT", line_num)?,
                start,
                stop,
            },
            _ => StepSweep::Linear {
                start,
                stop,
                step: step_or_points,
            },
        }
    };

    Ok(StepCommand {
        target,
        name,
        param_name,
        sweep,
    })
}

fn parse_step_points_per_interval(
    value: Value,
    sweep_type: &str,
    line_num: usize,
) -> Result<usize, ParseError> {
    // A usize contains values below 2^BITS. Expressing this exclusive bound in
    // floating point avoids the rounding of `usize::MAX as f64` on 64-bit hosts.
    let usize_upper_bound = 2.0_f64.powi(usize::BITS as i32);
    if !value.is_finite() || value < 1.0 || value.fract() != 0.0 || value >= usize_upper_bound {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".STEP {sweep_type} points per interval must be a positive integer representable as usize, found {value}"
            ),
        });
    }

    Ok(value as usize)
}

/// Parse .TEMP command: .TEMP t1 [t2 t3...]
///
/// ngspice also accepts the `.TEMP=t1` spelling, and sub-zero corners are
/// routine, so the value list reads signed magnitudes.
pub(super) fn parse_temp_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Vec<Value>, ParseError> {
    if matches!(stream.peek().kind, TokenKind::Equals) {
        stream.advance();
    }

    let mut temperatures = Vec::new();

    while let Some(v) = try_signed_value(stream, params) {
        temperatures.push(super::commands::parse_celsius_option("TEMP", v, line_num)?);
    }

    if temperatures.is_empty() {
        temperatures.push(27.0); // Default room temperature
    }

    Ok(temperatures)
}

/// One parsed `.FOUR` card: the spectrum it asks for and the window it asks
/// for that spectrum over.
pub(super) struct FourCard {
    pub(super) fundamental: Value,
    pub(super) num_harmonics: usize,
    pub(super) outputs: Vec<String>,
    pub(super) periods: usize,
    pub(super) window_from: Option<Value>,
    pub(super) window_to: Option<Value>,
}

/// Parse `.FOUR freq [num_harmonics] output1 [output2...]
/// [PERIODS=k] [FROM=t] [TO=t]`.
///
/// The positional form is unchanged and still valid alone. The window
/// keywords follow the outputs in any order; an output list ends at the first
/// `IDENT =` pair, which is what keeps the two forms disjoint (a probe is
/// never followed by `=`).
pub(super) fn parse_four_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<FourCard, ParseError> {
    const CARD: AnalysisCard = AnalysisCard::Four;

    let fundamental = expect_value(stream, line_num, params)?;
    if !fundamental.is_finite() || fundamental <= 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".FOUR fundamental frequency must be positive and finite, found {fundamental}"
            ),
        });
    }
    let num_harmonics = if at_keyword(stream) {
        9
    } else {
        match try_value(stream, params) {
            Some(value) => parse_four_harmonic_count(value, line_num)?,
            None => 9,
        }
    };

    let mut outputs = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if at_keyword(stream) {
            break;
        }
        if matches!(
            stream.peek().kind,
            TokenKind::Ident(_)
                | TokenKind::AtSign
                | TokenKind::Expression(_)
                | TokenKind::StringLit(_)
        ) {
            // Probe specs like V(out) span several tokens; reuse the .MEAS
            // signal parser for probes and authored expressions alike.
            outputs.push(super::commands::parse_meas_signal(
                stream, line_num, params,
            )?);
        } else {
            break;
        }
    }

    if outputs.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".FOUR requires at least one output".to_string(),
        });
    }

    let mut periods = None;
    let mut window_from = None;
    let mut window_to = None;
    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            return Err(card_error(
                CARD,
                line_num,
                AnalysisCardIssue::TrailingToken {
                    token: stream.peek().lexeme.clone(),
                },
            ));
        };
        match keyword.as_str() {
            "PERIODS" => bind_once(
                &mut periods,
                card_count(stream, line_num, params, CARD, "PERIODS", 1)?,
                CARD,
                line_num,
                "PERIODS",
            )?,
            "FROM" => bind_once(
                &mut window_from,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "FROM",
                    "a time in seconds >= 0",
                    |value| value >= 0.0,
                )?,
                CARD,
                line_num,
                "FROM",
            )?,
            "TO" => bind_once(
                &mut window_to,
                card_number(
                    stream,
                    line_num,
                    params,
                    CARD,
                    "TO",
                    "a positive time in seconds",
                    |value| value > 0.0,
                )?,
                CARD,
                line_num,
                "TO",
            )?,
            _ => {
                return Err(card_error(
                    CARD,
                    line_num,
                    AnalysisCardIssue::UnknownKeyword { keyword },
                ));
            }
        }
    }

    // A window that ends where it begins, or earlier, is not a window at all;
    // the two keywords would then be describing the same instant from both
    // sides. The engine's own refusal names one time, so name both here.
    if let (Some(from), Some(to)) = (window_from, window_to)
        && to <= from
    {
        return Err(card_error(
            CARD,
            line_num,
            AnalysisCardIssue::ConflictingFields {
                first: "TO",
                second: "FROM",
            },
        ));
    }

    Ok(FourCard {
        fundamental,
        num_harmonics,
        outputs,
        periods: periods.unwrap_or(1),
        window_from,
        window_to,
    })
}

fn parse_four_harmonic_count(value: Value, line_num: usize) -> Result<usize, ParseError> {
    let usize_upper_bound = 2.0_f64.powi(usize::BITS as i32);
    if !value.is_finite() || value < 1.0 || value.fract() != 0.0 || value >= usize_upper_bound {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                ".FOUR harmonic count must be a positive integer representable as usize, found {value}"
            ),
        });
    }
    Ok(value as usize)
}

#[cfg(test)]
mod four_command_tests {
    use crate::netlist::{AnalysisCommand, Netlist};

    #[test]
    fn four_accepts_an_optional_harmonic_count_and_preserves_the_default() {
        let explicit = Netlist::parse(
            "explicit Fourier harmonics\nV1 out 0 1\n.FOUR 60 15 I(V1) V(out)\n.END\n",
        )
        .expect("explicit harmonic count parses");
        let [
            AnalysisCommand::Four {
                fundamental,
                num_harmonics,
                outputs,
                ..
            },
        ] = explicit.analyses.as_slice()
        else {
            panic!("expected one Fourier analysis, got {:?}", explicit.analyses);
        };
        assert!((*fundamental - 60.0).abs() <= f64::EPSILON);
        assert_eq!(*num_harmonics, 15);
        assert_eq!(outputs.len(), 2);
        assert!(outputs[0].eq_ignore_ascii_case("I(V1)"), "{outputs:?}");
        assert!(outputs[1].eq_ignore_ascii_case("V(out)"), "{outputs:?}");

        let defaulted =
            Netlist::parse("default Fourier harmonics\nV1 out 0 1\n.FOUR 60 V(out)\n.END\n")
                .expect("default harmonic count parses");
        assert!(matches!(
            defaulted.analyses.as_slice(),
            [AnalysisCommand::Four {
                num_harmonics: 9,
                ..
            }]
        ));
    }

    #[test]
    fn four_rejects_non_integral_or_zero_harmonic_counts() {
        for count in ["0", "1.5"] {
            let error = Netlist::parse(&format!(
                "invalid Fourier harmonics\nV1 out 0 1\n.FOUR 60 {count} V(out)\n.END\n"
            ))
            .expect_err("invalid harmonic count must fail closed");
            assert!(error.to_string().contains("harmonic count"), "{error}");
        }
    }

    /// The window keywords follow the outputs, in any order, and the output
    /// list stops at the first `IDENT =` pair rather than swallowing it.
    #[test]
    fn four_reads_its_window_keywords_after_the_outputs() {
        for source in [
            ".FOUR 1k 5 V(out) I(V1) PERIODS=4 FROM=2m TO=6m",
            ".four 1K 5 v(out) i(v1) to=6m periods=4 from=2m",
        ] {
            let netlist =
                Netlist::parse(&format!("windowed Fourier\nV1 out 0 1\n{source}\n.END\n"))
                    .expect("a windowed .FOUR card parses");
            let [
                AnalysisCommand::Four {
                    fundamental,
                    num_harmonics,
                    outputs,
                    periods,
                    window_from,
                    window_to,
                },
            ] = netlist.analyses.as_slice()
            else {
                panic!("expected one Fourier analysis, got {:?}", netlist.analyses);
            };
            assert!((*fundamental - 1000.0).abs() <= f64::EPSILON * 1000.0);
            assert_eq!(*num_harmonics, 5);
            assert_eq!(outputs.len(), 2, "{outputs:?}");
            assert_eq!(*periods, 4);
            let from = window_from.expect("FROM is authored");
            let to = window_to.expect("TO is authored");
            assert!((from - 2e-3).abs() <= 1e-18, "{from}");
            assert!((to - 6e-3).abs() <= 1e-18, "{to}");
        }

        // The harmonic count is still optional, and a keyword pair in its
        // place is a keyword rather than a count.
        let netlist = Netlist::parse(
            "defaulted count beside a window\nV1 out 0 1\n.FOUR 1k V(out) PERIODS=2\n.END\n",
        )
        .expect("a window keyword may follow the outputs of a count-less card");
        assert!(matches!(
            netlist.analyses.as_slice(),
            [AnalysisCommand::Four {
                num_harmonics: 9,
                periods: 2,
                window_from: None,
                window_to: None,
                ..
            }]
        ));
    }

    /// A card that authors no keyword is the card it has always been: every
    /// deck already in existence keeps its spectrum.
    #[test]
    fn an_unwindowed_four_card_parses_as_it_always_has() {
        let netlist = Netlist::parse("plain Fourier\nV1 out 0 1\n.FOUR 60 15 V(out)\n.END\n")
            .expect("a keyword-less .FOUR card parses");
        assert!(matches!(
            netlist.analyses.as_slice(),
            [AnalysisCommand::Four {
                num_harmonics: 15,
                periods: 1,
                window_from: None,
                window_to: None,
                ..
            }]
        ));
    }

    #[test]
    fn a_four_card_refuses_an_unknown_repeated_or_contradictory_keyword() {
        let cases: [(&str, &str); 5] = [
            (
                "PERIODS=2 PERIODS=3",
                "keyword PERIODS authored more than once",
            ),
            ("TO=5m TO=6m", "keyword TO authored more than once"),
            ("WINDOW=hann", "unknown keyword 'WINDOW'"),
            ("PERIODS=0", "PERIODS must be a whole number >= 1, got 0"),
            (
                "FROM=6m TO=6m",
                "TO and FROM set the same quantity; author only one",
            ),
        ];
        for (tail, expected) in cases {
            let error = Netlist::parse(&format!(
                "refused Fourier window\nV1 out 0 1\n.TRAN 1u 10m\n.FOUR 1k V(out) {tail}\n.END\n"
            ))
            .expect_err("the card must fail closed");
            let rendered = error.to_string();
            assert!(
                rendered.contains(".FOUR at line 4: ") && rendered.contains(expected),
                "`{tail}` reported {rendered}"
            );
        }
    }
}

#[cfg(test)]
mod step_command_tests {
    use crate::config::ExpressionDialect;
    use crate::netlist::{AnalysisCommand, Netlist, NetlistParseOptions, StepSweep, StepTarget};

    fn parse_xyce(source: &str) -> Netlist {
        Netlist::parse_with_options(
            source,
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..NetlistParseOptions::default()
            },
        )
        .expect("Xyce .STEP deck parses")
    }

    #[test]
    fn step_list_accepts_signed_values_without_leading_zero() {
        let netlist = Netlist::parse(
            "signed step list\n\
             V1 out 0 0\n\
             .step V1 LIST -.05 +.5 -1.0\n\
             .end\n",
        )
        .expect("signed .STEP LIST values parse");

        let [AnalysisCommand::Step(command)] = netlist.analyses.as_slice() else {
            panic!("expected one .STEP analysis, got {:?}", netlist.analyses);
        };
        assert_eq!(command.name, "V1");
        assert_eq!(command.target, StepTarget::Device);
        assert!(matches!(
            &command.sweep,
            StepSweep::List(values)
                if values
                    .iter()
                    .zip([-0.05, 0.5, -1.0])
                    .all(|(actual, expected)| (actual - expected).abs() < 1e-15)
        ));
    }

    #[test]
    fn xyce_bare_step_parameter_resolution_is_independent_of_declaration_order() {
        for (source, expected_name) in [
            (
                "late parameter\nV1 out 0 0\n.step dtempParam list -10 0 10\n.param dtempParam=10\n.end\n",
                "dtempParam",
            ),
            (
                "early parameter\nV1 out 0 0\n.param dtempParam=10\n.step dtempParam list -10 0 10\n.end\n",
                "dtempParam",
            ),
            (
                "late global parameter\nV1 out 0 0\n.step LIN scale 1 3 1\n.global_param scale=1\n.end\n",
                "scale",
            ),
            (
                "early global parameter\nV1 out 0 0\n.global_param scale=1\n.step LIN scale 1 3 1\n.end\n",
                "scale",
            ),
        ] {
            let netlist = parse_xyce(source);
            let [AnalysisCommand::Step(command)] = netlist.analyses.as_slice() else {
                panic!("expected one .STEP analysis, got {:?}", netlist.analyses);
            };
            assert_eq!(command.target, StepTarget::Param);
            assert!(command.name.eq_ignore_ascii_case(expected_name));
            assert!(command.param_name.is_none());
        }
    }

    #[test]
    fn xyce_bare_step_name_prefers_a_global_over_a_natural_device_parameter() {
        let netlist = parse_xyce(
            "device and parameter collision\n\
             .param R1=9k\n\
             R1 out 0 1k\n\
             .step R1 list 1k 2k\n\
             .end\n",
        );
        let [AnalysisCommand::Step(command)] = netlist.analyses.as_slice() else {
            panic!("expected one .STEP analysis, got {:?}", netlist.analyses);
        };
        assert_eq!(command.target, StepTarget::Param);
        assert_eq!(command.name, "R1");
        assert!(command.param_name.is_none());
    }

    #[test]
    fn explicit_step_param_remains_global_when_a_device_has_the_same_name() {
        let netlist = parse_xyce(
            "explicit parameter target\n\
             .param R1=9k\n\
             R1 out 0 1k\n\
             .step PARAM R1 list 1k 2k\n\
             .end\n",
        );
        let [AnalysisCommand::Step(command)] = netlist.analyses.as_slice() else {
            panic!("expected one .STEP analysis, got {:?}", netlist.analyses);
        };
        assert_eq!(command.target, StepTarget::Param);
        assert_eq!(command.name, "R1");
        assert!(command.param_name.is_none());
    }

    #[test]
    fn explicit_step_device_parameter_remains_device_when_a_global_has_the_same_name() {
        for target in ["R1:R", "R1(R)"] {
            let netlist = parse_xyce(&format!(
                "explicit device target\n\
                 .param R1=9k\n\
                 R1 out 0 1k\n\
                 .step {target} list 1k 2k\n\
                 .end\n"
            ));
            let [AnalysisCommand::Step(command)] = netlist.analyses.as_slice() else {
                panic!("expected one .STEP analysis, got {:?}", netlist.analyses);
            };
            assert_eq!(command.target, StepTarget::Device);
            assert_eq!(command.name, "R1");
            assert_eq!(command.param_name.as_deref(), Some("R"));
        }
    }
}

/// Reference impedance an analysis port takes when its keyword omits one.
const SP_CARD_PORT_DEFAULT_Z0: Value = 50.0;

/// Parse .SP command:
/// `.SP DEC|LIN|OCT np fstart fstop [donoise] [PORT<k>=(<n+>[,<n->[,<z0>]]) ...]`
///
/// The noise flag stays positional, so it is read only when the next token is
/// not a `KEYWORD =` pair: `.SP DEC 10 1k 1MEG PORT1=(in)` asks for no noise,
/// and a flag written after a port keyword is a trailing token.
pub(super) fn parse_sp_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let var_str = expect_ident(stream, line_num)?;
    let variation = match var_str.to_uppercase().as_str() {
        "LIN" => FreqVariation::Lin,
        "OCT" => FreqVariation::Oct,
        "DEC" => FreqVariation::Dec,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unknown .SP frequency variation: {}", var_str),
            });
        }
    };

    let points = expect_value(stream, line_num, params)? as usize;
    let start_freq = expect_value(stream, line_num, params)?;
    let stop_freq = expect_value(stream, line_num, params)?;
    let do_noise = if at_card_end(stream) || at_keyword(stream) {
        false
    } else {
        match &stream.peek().kind {
            TokenKind::Ident(keyword) if keyword.eq_ignore_ascii_case("donoise") => {
                stream.advance();
                true
            }
            TokenKind::Ident(keyword)
                if keyword.eq_ignore_ascii_case("true")
                    || keyword.eq_ignore_ascii_case("false") =>
            {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(".SP noise option must be DONOISE, 0, or 1; found {keyword}"),
                });
            }
            _ => {
                let raw = expect_value(stream, line_num, params)?;
                match raw {
                    0.0 => false,
                    1.0 => true,
                    _ => {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                ".SP noise option must be DONOISE, 0, or 1; found {raw}"
                            ),
                        });
                    }
                }
            }
        }
    };
    let ports = parse_sp_card_ports(stream, line_num, params)?;

    Ok(AnalysisCommand::Sp {
        variation,
        points,
        start_freq,
        stop_freq,
        do_noise,
        ports,
    })
}

/// Read the `PORT<k>=` keywords that name a `.SP` run's reference planes.
///
/// Each value is `(<n+>[,<n->[,<z0>]])`: the parentheses are required, so the
/// grammar reads the same whether one node or three fields are written, and a
/// missing node cannot be silently taken from the next keyword. After the loop
/// the numbers must be exactly `1..=N` — the same density rule the deck's own
/// `portnum=` annotations are held to, checked here so the card is refused
/// where it was written rather than after elaboration.
fn parse_sp_card_ports(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
) -> Result<Vec<SpCardPort>, ParseError> {
    const CARD: AnalysisCard = AnalysisCard::Sp;

    let mut ports: Vec<SpCardPort> = Vec::new();
    loop {
        skip_commas(stream);
        if at_card_end(stream) {
            break;
        }
        let Some(keyword) = take_keyword(stream) else {
            return Err(card_error(
                CARD,
                line,
                AnalysisCardIssue::TrailingToken {
                    token: stream.peek().lexeme.clone(),
                },
            ));
        };
        let digits = keyword.strip_prefix("PORT").map(str::to_owned);
        let Some(number) = digits.and_then(|digits| digits.parse::<usize>().ok()) else {
            return Err(card_error(
                CARD,
                line,
                AnalysisCardIssue::UnknownKeyword { keyword },
            ));
        };
        if number == 0 {
            return Err(card_error(
                CARD,
                line,
                AnalysisCardIssue::InvalidName {
                    field: "PORT<k>",
                    value: keyword,
                },
            ));
        }
        if ports.iter().any(|port| port.number == number) {
            return Err(card_error(
                CARD,
                line,
                AnalysisCardIssue::DuplicateKeyword { keyword: "PORT<k>" },
            ));
        }
        ports.push(parse_sp_card_port(stream, line, params, number)?);
    }

    ports.sort_by_key(|port| port.number);
    for (position, port) in ports.iter().enumerate() {
        if port.number != position + 1 {
            return Err(card_error(
                CARD,
                line,
                AnalysisCardIssue::InvalidName {
                    field: "PORT<k>",
                    value: format!("PORT{}", port.number),
                },
            ));
        }
    }
    Ok(ports)
}

/// Read one `(<n+>[,<n->[,<z0>]])` reference-plane value.
fn parse_sp_card_port(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    number: usize,
) -> Result<SpCardPort, ParseError> {
    const CARD: AnalysisCard = AnalysisCard::Sp;

    if !stream.consume(&TokenKind::LParen) {
        return Err(card_error(
            CARD,
            line,
            AnalysisCardIssue::InvalidChoice {
                field: "PORT<k>",
                value: stream.peek().lexeme.clone(),
                expected: "(node+[,node-[,z0]])",
            },
        ));
    }
    let node_pos = card_name(stream, line, CARD, "PORT<k>")?.to_ascii_uppercase();
    let mut node_neg = "0".to_owned();
    let mut z0 = SP_CARD_PORT_DEFAULT_Z0;
    if stream.consume(&TokenKind::Comma) {
        node_neg = card_name(stream, line, CARD, "PORT<k>")?.to_ascii_uppercase();
        if stream.consume(&TokenKind::Comma) {
            z0 = card_number(
                stream,
                line,
                params,
                CARD,
                "PORT<k>",
                "a positive reference impedance in ohms",
                |value| value > 0.0,
            )?;
        }
    }
    if !stream.consume(&TokenKind::RParen) {
        return Err(card_error(
            CARD,
            line,
            AnalysisCardIssue::TrailingToken {
                token: stream.peek().lexeme.clone(),
            },
        ));
    }
    Ok(SpCardPort {
        number,
        node_pos,
        node_neg,
        z0,
    })
}

#[cfg(test)]
mod quoted_path_tests {
    use super::quoted_path_lexeme;
    use crate::netlist::Netlist;
    use crate::netlist::lexer::{TokenStream, tokenize};
    use crate::netlist::measure::MeasureType;

    fn taken(source: &str) -> (Option<String>, bool) {
        let mut stream = TokenStream::new(tokenize(source).expect("the source lexes"));
        let path = quoted_path_lexeme(&mut stream);
        (path, stream.is_eof())
    }

    /// A quoted path is the path that was written. Decoding it as a string
    /// literal drops every backslash, which is every Windows path there is.
    #[test]
    fn a_quoted_path_keeps_every_backslash_it_was_written_with() {
        for written in [
            r"C:\meas\step.csv",
            r"\\server\share\meas\step.csv",
            r"C:\new folder\tab\step.csv",
            "meas/step.csv",
            "",
        ] {
            assert_eq!(taken(&format!("\"{written}\"")).0.as_deref(), Some(written));
            assert_eq!(taken(&format!("'{written}'")).0.as_deref(), Some(written));
        }
    }

    /// An unquoted run is the caller's to read, and nothing is consumed here.
    #[test]
    fn an_unquoted_run_is_left_where_it_was() {
        for source in [r"C:\meas\step.csv", "step.csv"] {
            let (path, consumed_everything) = taken(source);
            assert_eq!(path, None, "{source}");
            assert!(!consumed_everything, "{source}");
        }
    }

    /// The directives that name a file all read their path this way.
    #[test]
    fn every_directive_that_names_a_file_keeps_its_path() {
        let measured = Netlist::parse(concat!(
            "measure file path\n",
            "V1 in 0 1\n",
            "R1 in out 1k\n",
            "C1 out 0 1u\n",
            ".TRAN 1u 1m\n",
            ".MEAS TRAN drift ERROR V(out) FILE=\"C:\\meas\\golden.csv\" DEPVARCOL=1\n",
            ".END\n",
        ))
        .expect(".MEAS ERROR with a quoted path parses");
        let file = measured
            .measurements
            .iter()
            .find_map(|measure| match &measure.measure_type {
                MeasureType::FileError { file, .. } => Some(file.path().to_owned()),
                _ => None,
            })
            .expect("one .MEAS ERROR");
        assert_eq!(file, r"C:\meas\golden.csv");
    }

    /// `.SPEF_INCLUDE` only resolves under a path-backed parse, so what this
    /// pins is the path the resolver was handed: the file is not there, and
    /// the refusal names it exactly as the deck wrote it.
    #[test]
    fn a_quoted_spef_include_keeps_its_path() {
        let error = Netlist::parse_with_path(
            concat!(
                "spef include path\n",
                "R1 in out 1k\n",
                ".SPEF_INCLUDE \"C:\\extract\\no-such-top.spef\"\n",
                ".END\n",
            ),
            std::path::Path::new("C:/decks/top.cir"),
        )
        .expect_err("the SPEF file is not there")
        .to_string();
        assert!(error.contains(r"C:\extract\no-such-top.spef"), "{error}");
    }
}

#[cfg(test)]
mod sp_command_tests {
    use crate::netlist::{AnalysisCommand, Netlist};

    fn parsed_noise_flag(option: &str) -> bool {
        let deck = format!(
            "SP noise parser\nV1 in 0 0 portnum=1 z0=50\n.SP DEC 10 1k 1meg {option}\n.END\n"
        );
        let netlist = Netlist::parse(&deck).expect(".SP option parses");
        let [AnalysisCommand::Sp { do_noise, .. }] = netlist.analyses.as_slice() else {
            panic!("expected one .SP analysis, got {:?}", netlist.analyses);
        };
        *do_noise
    }

    #[test]
    fn sp_accepts_keyword_and_deliberate_numeric_noise_forms() {
        assert!(parsed_noise_flag("donoise"));
        assert!(parsed_noise_flag("DoNoIsE"));
        assert!(parsed_noise_flag("1"));
        assert!(!parsed_noise_flag("0"));
    }

    #[test]
    fn sp_rejects_unknown_flags_and_trailing_tokens() {
        for option in ["2", "-1", "donoise extra", "0 extra", "true"] {
            let deck = format!(
                "bad SP noise option\nV1 in 0 0 portnum=1 z0=50\n.SP DEC 10 1k 1meg {option}\n.END\n"
            );
            let error = Netlist::parse(&deck).expect_err("malformed .SP option must fail");
            let message = error.to_string();
            assert!(
                message.contains(".SP") || message.contains("trailing token"),
                "unexpected error for '{option}': {message}"
            );
        }
    }

    /// A deck whose planes are named on the card instead of on its sources.
    fn ported_deck(tail: &str) -> String {
        format!(
            "SP card ports\n\
             V1 in 0 AC 1\n\
             R1 in out 50\n\
             R2 out 0 1meg\n\
             .SP DEC 10 1k 1meg {tail}\n\
             .END\n"
        )
    }

    #[test]
    fn sp_card_ports_parse_with_their_reference_impedances() {
        let deck = ported_deck("port1=(in) PORT2=(out,0,75)");
        let netlist = Netlist::parse(&deck).expect(".SP port keywords parse");
        let [
            AnalysisCommand::Sp {
                do_noise, ports, ..
            },
        ] = netlist.analyses.as_slice()
        else {
            panic!("expected one .SP analysis, got {:?}", netlist.analyses);
        };
        assert!(!*do_noise, "no positional noise flag was authored");
        assert_eq!(ports.len(), 2, "both planes survive parsing: {ports:?}");
        assert_eq!(ports[0].number, 1);
        assert_eq!(ports[0].node_pos, "IN");
        assert_eq!(ports[0].node_neg, "0", "an omitted negative node is ground");
        assert_eq!(
            ports[0].z0, 50.0,
            "an omitted reference impedance is 50 ohms"
        );
        assert_eq!(ports[1].number, 2);
        assert_eq!(ports[1].node_pos, "OUT");
        assert_eq!(ports[1].node_neg, "0");
        assert_eq!(
            ports[1].z0, 75.0,
            "an authored impedance is carried exactly"
        );
    }

    #[test]
    fn sp_card_ports_must_be_numbered_from_one_without_gaps() {
        for (tail, named) in [
            ("PORT1=(in) PORT3=(out)", "PORT3"),
            ("PORT2=(in) PORT3=(out)", "PORT2"),
            ("PORT1=(in) PORT1=(out)", "PORT<k>"),
            ("PORT0=(in)", "PORT0"),
        ] {
            let deck = ported_deck(tail);
            let error = Netlist::parse(&deck)
                .expect_err("a .SP port roster that is not dense from one must fail");
            let message = error.to_string();
            assert!(
                message.contains(".SP") && message.contains(named),
                "the refusal names the card and the keyword for '{tail}': {message}"
            );
        }
    }

    #[test]
    fn sp_keeps_its_positional_noise_flag_before_the_keywords() {
        for flag in ["1", "donoise"] {
            let deck = ported_deck(&format!("{flag} PORT1=(in) PORT2=(out)"));
            let netlist = Netlist::parse(&deck).expect("a flag before the keywords parses");
            let [
                AnalysisCommand::Sp {
                    do_noise, ports, ..
                },
            ] = netlist.analyses.as_slice()
            else {
                panic!("expected one .SP analysis, got {:?}", netlist.analyses);
            };
            assert!(*do_noise, "the positional flag still reads as noise");
            assert_eq!(ports.len(), 2);
        }
        let deck = ported_deck("PORT1=(in) 1");
        let error =
            Netlist::parse(&deck).expect_err("the noise flag is positional, so it cannot follow");
        let message = error.to_string();
        assert!(
            message.contains(".SP") && message.contains("trailing token"),
            "a flag written after a keyword is a trailing token: {message}"
        );
    }

    #[test]
    fn an_sp_card_written_without_keywords_parses_as_it_always_has() {
        for (tail, noise) in [("", false), ("donoise", true), ("0", false)] {
            let deck = ported_deck(tail);
            let netlist = Netlist::parse(&deck).expect("a keyword-free .SP card parses");
            let [
                AnalysisCommand::Sp {
                    variation,
                    points,
                    start_freq,
                    stop_freq,
                    do_noise,
                    ports,
                },
            ] = netlist.analyses.as_slice()
            else {
                panic!("expected one .SP analysis, got {:?}", netlist.analyses);
            };
            assert_eq!(*variation, crate::netlist::FreqVariation::Dec);
            assert_eq!(*points, 10);
            assert_eq!(*start_freq, 1.0e3);
            assert_eq!(*stop_freq, 1.0e6);
            assert_eq!(*do_noise, noise);
            assert!(
                ports.is_empty(),
                "a card with no keyword names no analysis plane: {ports:?}"
            );
        }
    }
}

/// Parse .DISTO command: .DISTO DEC|LIN|OCT np fstart fstop `[f2overf1]`
pub(super) fn parse_disto_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let var_str = expect_ident(stream, line_num)?;
    let variation = match var_str.to_uppercase().as_str() {
        "LIN" => FreqVariation::Lin,
        "OCT" => FreqVariation::Oct,
        "DEC" => FreqVariation::Dec,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Invalid .DISTO frequency variation '{}': expected LIN, OCT, or DEC",
                    var_str
                ),
            });
        }
    };

    let points = expect_value(stream, line_num, params)? as usize;
    let start_freq = expect_value(stream, line_num, params)?;
    let stop_freq = expect_value(stream, line_num, params)?;
    let f2_over_f1 = if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        None
    } else {
        Some(expect_value(stream, line_num, params)?)
    };

    Ok(AnalysisCommand::Disto {
        variation,
        points,
        start_freq,
        stop_freq,
        f2_over_f1,
    })
}

/// Parse .NOISE command: .NOISE V(out`[,ref]`) Vsource DEC|LIN|OCT np fstart fstop `[pts_per_summary]`
pub(super) fn parse_noise_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let (output_node, reference_node) = parse_voltage_output_reference(stream, line_num)?;

    // Input source
    let input_source = expect_ident(stream, line_num)?;

    // Frequency sweep type
    let var_str = expect_ident(stream, line_num)?;
    if var_str.eq_ignore_ascii_case("DATA") {
        if !stream.consume(&TokenKind::Equals) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".NOISE DATA requires DATA=<table-name>".to_string(),
            });
        }
        let table_name = expect_ident(stream, line_num)?;
        let _summary_interval = try_value(stream, params);
        return Ok(AnalysisCommand::NoiseData {
            output_node,
            reference_node,
            input_source,
            table_name,
        });
    }
    let variation = match var_str.to_uppercase().as_str() {
        "LIN" => FreqVariation::Lin,
        "OCT" => FreqVariation::Oct,
        "DEC" => FreqVariation::Dec,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Unknown frequency variation: {}", var_str),
            });
        }
    };

    let points = expect_value(stream, line_num, params)? as usize;
    let start_freq = expect_value(stream, line_num, params)?;
    let stop_freq = expect_value(stream, line_num, params)?;
    let _summary_interval = try_value(stream, params);

    Ok(AnalysisCommand::Noise {
        output_node,
        reference_node,
        input_source,
        variation,
        points,
        start_freq,
        stop_freq,
    })
}

/// Parse .TF command: .TF V(out`[,ref]`) insrc  |  .TF I(element) insrc
pub(super) fn parse_tf_command(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<AnalysisCommand, ParseError> {
    let is_current_probe = matches!(&stream.peek().kind, TokenKind::Ident(s) if {
        let upper = s.to_uppercase();
        upper == "I" || upper.starts_with("I(")
    });

    let (output_node, reference_node, output_is_current) = if is_current_probe {
        let ident = expect_ident(stream, line_num)?;
        let element = if ident.len() > 1 {
            // Merged token form `I(ELEM)`.
            parse_inline_current_probe(&ident, line_num)?
        } else {
            if !stream.consume(&TokenKind::LParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected '(' after I in .TF current probe".to_string(),
                });
            }
            let element = expect_ident(stream, line_num)?;
            if !stream.consume(&TokenKind::RParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected ')' in I(element) specification".to_string(),
                });
            }
            element
        };
        (element.to_uppercase(), None, true)
    } else {
        let (node, reference) = parse_voltage_output_reference(stream, line_num)?;
        (node, reference, false)
    };

    let input_source = expect_ident(stream, line_num)?.to_uppercase();

    Ok(AnalysisCommand::Tf {
        output_node,
        reference_node,
        output_is_current,
        input_source,
    })
}

/// Extract the element name from a merged `I(ELEM)` token.
fn parse_inline_current_probe(token: &str, line_num: usize) -> Result<String, ParseError> {
    let inner = token[1..]
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!("Invalid current probe `{token}` in .TF (expected I(element))"),
        })?;
    Ok(inner.to_uppercase())
}

/// Parse .SENS command:
/// .SENS V(out`[,ref]`)|I(vsource) [devspec ...] [AC DEC|LIN|OCT np fstart fstop]
pub(super) fn parse_sens_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<AnalysisCommand, ParseError> {
    let is_current_probe = matches!(&stream.peek().kind, TokenKind::Ident(s) if {
        let upper = s.to_uppercase();
        upper == "I" || upper.starts_with("I(")
    });
    let (output_node, reference_node, output_is_current) = if is_current_probe {
        let ident = expect_ident(stream, line_num)?;
        let element = if ident.len() > 1 {
            parse_inline_current_probe(&ident, line_num)?
        } else {
            if !stream.consume(&TokenKind::LParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected '(' after I in .SENS current probe".to_string(),
                });
            }
            let element = expect_ident(stream, line_num)?;
            if !stream.consume(&TokenKind::RParen) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Expected ')' in I(element) specification".to_string(),
                });
            }
            element
        };
        (element.to_uppercase(), None, true)
    } else {
        let (node, reference) = parse_voltage_output_reference(stream, line_num)?;
        (node, reference, false)
    };

    let mut filters = Vec::new();
    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let is_mode = matches!(
            &stream.peek().kind,
            TokenKind::Ident(mode)
                if mode.eq_ignore_ascii_case("AC") || mode.eq_ignore_ascii_case("DC")
        );
        if is_mode {
            break;
        }
        filters.push(consume_sensitivity_filter(stream));
    }

    let mut ac_sweep = None;

    if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let mode = expect_ident(stream, line_num)?;
        if mode.eq_ignore_ascii_case("DC") {
            if !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".SENS DC does not accept a frequency sweep".to_string(),
                });
            }
            return Ok(AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                output_is_current,
                filters,
                ac_sweep,
            });
        }
        if !mode.eq_ignore_ascii_case("AC") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Invalid .SENS mode '{}': expected AC, DC, a device filter, or end-of-line",
                    mode
                ),
            });
        }

        let var_str = expect_ident(stream, line_num)?;
        let variation = match var_str.to_uppercase().as_str() {
            "LIN" => FreqVariation::Lin,
            "OCT" => FreqVariation::Oct,
            "DEC" => FreqVariation::Dec,
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Invalid .SENS AC sweep variation '{}': expected LIN, OCT, or DEC",
                        var_str
                    ),
                });
            }
        };

        let points = expect_value(stream, line_num, params)? as usize;
        let start_freq = expect_value(stream, line_num, params)?;
        let stop_freq = expect_value(stream, line_num, params)?;

        ac_sweep = Some(SensitivityAcSweep {
            variation,
            points,
            start_freq,
            stop_freq,
        });
    }

    Ok(AnalysisCommand::Sensitivity {
        output_node,
        reference_node,
        output_is_current,
        filters,
        ac_sweep,
    })
}

/// Consume one whitespace-delimited `.SENS` device specification. Wildcards
/// are separate lexer tokens, so source spans are used to join adjacent pieces
/// (`M*`, `MOD:*`, `R?_TC1`) without merging the next whitespace-separated
/// filter.
fn consume_sensitivity_filter(stream: &mut TokenStream) -> String {
    let first = stream.advance().clone();
    let line = first.span.line;
    let mut end = first.span.end;
    let mut filter = first.lexeme;
    while !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
        && stream.peek().span.line == line
        && stream.peek().span.start == end
    {
        let token = stream.advance().clone();
        end = token.span.end;
        filter.push_str(&token.lexeme);
    }
    filter.to_ascii_uppercase()
}

/// Parse .PZ command: .PZ in+ in- out+ out- VOL|CUR PZ|POL|ZER
pub(super) fn parse_pz_command(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<AnalysisCommand, ParseError> {
    let input_pos = expect_node(stream, line_num)?;
    let input_neg = expect_node(stream, line_num)?;
    let output_pos = expect_node(stream, line_num)?;
    let output_neg = expect_node(stream, line_num)?;

    let transfer_type = expect_ident(stream, line_num)?;
    let transfer_type = match transfer_type.to_uppercase().as_str() {
        "VOL" => PoleZeroTransferType::Voltage,
        "CUR" => PoleZeroTransferType::Current,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Invalid .PZ transfer type '{}': expected VOL or CUR",
                    transfer_type
                ),
            });
        }
    };

    let analysis_type = expect_ident(stream, line_num)?;
    let analysis_type = match analysis_type.to_uppercase().as_str() {
        "PZ" => PoleZeroAnalysisType::PoleZero,
        "POL" => PoleZeroAnalysisType::PolesOnly,
        "ZER" => PoleZeroAnalysisType::ZerosOnly,
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Invalid .PZ analysis type '{}': expected PZ, POL, or ZER",
                    analysis_type
                ),
            });
        }
    };

    Ok(AnalysisCommand::PoleZero {
        input_pos,
        input_neg,
        output_pos,
        output_neg,
        transfer_type,
        analysis_type,
    })
}

/// Parse .MC command:
/// .MC runs [START zero_based_index] [SEED n] [DIST GAUSS|UNIFORM|WORSTCASE] [SPREAD rel]
/// [CONFIDENCE pct] [CI STUDENTT|BOOTSTRAP] [RESAMPLES n] [BOOTSEED n] [PARAMS p1 p2 ...]
///
/// Supported shorthand:
/// .MC runs GAUSS sigma
/// .MC runs UNIFORM tol
/// .MC runs WORSTCASE tol
pub(super) fn parse_mc_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    max_analysis_points: usize,
    report_spans: &mut Vec<std::ops::Range<usize>>,
) -> Result<MonteCarloCommand, ParseError> {
    let run_literal = monte_carlo_identity::literal_span(stream, stream.peek().span.start);
    let runs = expect_u64_value(stream, line_num, params, ".MC run count")?;
    monte_carlo_identity::retain_literal_span(report_spans, run_literal, stream);
    if runs == 0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".MC run count must be at least 1".to_owned(),
        });
    }
    let runs = usize::try_from(runs).map_err(|_| ParseError::Syntax {
        line: line_num,
        message: format!(".MC run count {runs} exceeds this platform's supported range"),
    })?;
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::AnalysisPoints,
        runs,
        max_analysis_points,
    )?;
    let mut command = MonteCarloCommand::new(runs);
    let mut bootstrap = false;
    let mut resamples = None;
    let mut bootstrap_seed = None;
    let mut confidence_seen = std::collections::HashSet::new();

    let parse_distribution = |s: &str| -> Option<MonteCarloDistribution> {
        match s.to_ascii_uppercase().as_str() {
            "GAUSS" | "GAUSSIAN" | "NORMAL" => Some(MonteCarloDistribution::Gaussian),
            "UNIFORM" | "UNIF" => Some(MonteCarloDistribution::Uniform),
            "WORST" | "WORSTCASE" | "WC" => Some(MonteCarloDistribution::WorstCase),
            _ => None,
        }
    };

    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        let field_start = stream.peek().span.start;
        let keyword = expect_ident(stream, line_num)?;
        if matches!(
            keyword.as_str(),
            "START" | "CONFIDENCE" | "CI" | "RESAMPLES" | "BOOTSEED"
        ) && !confidence_seen.insert(keyword.clone())
        {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("Duplicate .MC {keyword}"),
            });
        }
        match keyword.as_str() {
            "CONFIDENCE" => {
                stream.consume(&TokenKind::Equals);
                let literal = monte_carlo_identity::literal_span(stream, field_start);
                command.confidence_pct = expect_value(stream, line_num, params)?;
                monte_carlo_identity::retain_literal_span(report_spans, literal, stream);
            }
            "CI" => {
                stream.consume(&TokenKind::Equals);
                let method = expect_ident(stream, line_num)?;
                report_spans.push(field_start..stream.peek().span.start);
                bootstrap = match method.as_str() {
                    "STUDENTT" => false,
                    "BOOTSTRAP" => true,
                    _ => {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "Invalid .MC CI {method}: expected STUDENTT or BOOTSTRAP"
                            ),
                        });
                    }
                };
            }
            "RESAMPLES" => {
                stream.consume(&TokenKind::Equals);
                let literal = monte_carlo_identity::literal_span(stream, field_start);
                let count = expect_u64_value(stream, line_num, params, ".MC RESAMPLES")?;
                monte_carlo_identity::retain_literal_span(report_spans, literal, stream);
                resamples = Some(usize::try_from(count).map_err(|_| ParseError::Syntax {
                    line: line_num,
                    message: ".MC RESAMPLES exceeds the platform range".into(),
                })?);
            }
            "BOOTSEED" => {
                stream.consume(&TokenKind::Equals);
                let literal = monte_carlo_identity::literal_span(stream, field_start);
                bootstrap_seed = Some(expect_u64_value(stream, line_num, params, ".MC BOOTSEED")?);
                monte_carlo_identity::retain_literal_span(report_spans, literal, stream);
            }
            "START" => {
                stream.consume(&TokenKind::Equals);
                let literal = monte_carlo_identity::literal_span(stream, field_start);
                let first = expect_u64_value(stream, line_num, params, ".MC START")?;
                monte_carlo_identity::retain_literal_span(report_spans, literal, stream);
                command.first_trial = usize::try_from(first).map_err(|_| ParseError::Syntax {
                    line: line_num,
                    message: ".MC START exceeds this platform's supported index range".into(),
                })?;
            }
            "SEED" => {
                stream.consume(&TokenKind::Equals);
                command.seed = Some(expect_u64_value(stream, line_num, params, ".MC SEED")?);
            }
            "DIST" | "DISTRIBUTION" => {
                let dist = expect_ident(stream, line_num)?;
                command.distribution =
                    parse_distribution(&dist).ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: format!(
                            "Invalid .MC distribution '{}': expected GAUSS, UNIFORM, or WORSTCASE",
                            dist
                        ),
                    })?;

                if let Some(spread) = try_value(stream, params) {
                    command.relative_spread = spread;
                }
            }
            "GAUSS" | "GAUSSIAN" | "NORMAL" => {
                command.distribution = MonteCarloDistribution::Gaussian;
                if let Some(spread) = try_value(stream, params) {
                    command.relative_spread = spread;
                }
            }
            "UNIFORM" | "UNIF" => {
                command.distribution = MonteCarloDistribution::Uniform;
                if let Some(spread) = try_value(stream, params) {
                    command.relative_spread = spread;
                }
            }
            "WORST" | "WORSTCASE" | "WC" => {
                command.distribution = MonteCarloDistribution::WorstCase;
                if let Some(spread) = try_value(stream, params) {
                    command.relative_spread = spread;
                }
            }
            "SPREAD" | "SIGMA" | "TOL" | "TOLERANCE" => {
                command.relative_spread = expect_value(stream, line_num, params)?;
            }
            "PARAMS" | "PARAMETERS" => {
                while !stream.is_eof()
                    && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
                {
                    skip_commas(stream);
                    match &stream.peek().kind {
                        TokenKind::Ident(name) => {
                            let name = name.clone();
                            stream.advance();
                            if !command.params.iter().any(|p| p == &name) {
                                command.params.push(name);
                            }
                        }
                        other => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Invalid .MC parameter list token {:?}: expected identifier",
                                    other
                                ),
                            });
                        }
                    }
                }

                if command.params.is_empty() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "Invalid .MC PARAMS list: expected at least one parameter name"
                            .to_string(),
                    });
                }
            }
            _ => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Invalid .MC keyword '{}': expected START, SEED, DIST, SPREAD, CONFIDENCE, CI, RESAMPLES, BOOTSEED, or PARAMS",
                        keyword
                    ),
                });
            }
        }
    }

    if command.first_trial.checked_add(command.runs).is_none() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".MC trial range overflows the supported index range".into(),
        });
    }

    if !command.relative_spread.is_finite() || command.relative_spread < 0.0 {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Invalid .MC spread '{}': expected non-negative finite value",
                command.relative_spread
            ),
        });
    }

    if !command.confidence_pct.is_finite()
        || command.confidence_pct <= 0.0
        || command.confidence_pct >= 100.0
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".MC CONFIDENCE must be finite and strictly between 0 and 100 percent".into(),
        });
    }
    if bootstrap {
        let resamples = resamples.unwrap_or(10_000);
        if resamples < 2 {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".MC RESAMPLES must be at least two".into(),
            });
        }
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            resamples,
            max_analysis_points,
        )?;
        command.confidence_method =
            crate::netlist::MonteCarloMeanConfidenceMethod::PercentileBootstrap {
                resamples,
                seed: bootstrap_seed.unwrap_or(0),
            };
    } else if resamples.is_some() || bootstrap_seed.is_some() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: ".MC RESAMPLES and BOOTSEED require CI BOOTSTRAP".into(),
        });
    }
    Ok(command)
}

/// Parse voltage reference like V(out) or V(out,0)
pub(super) fn parse_voltage_reference(spec: &str) -> Result<(String, Option<String>), ParseError> {
    let spec_upper = spec.to_uppercase();

    if !spec_upper.starts_with("V(") {
        return Ok((spec.to_string(), None));
    }

    // Remove V( prefix and ) suffix
    let inner = spec
        .trim_start_matches(['V', 'v'])
        .trim_start_matches('(')
        .trim_end_matches(')');

    let parts: Vec<&str> = inner.split(',').collect();

    let node = parts[0].trim().to_string();
    let reference = if parts.len() > 1 {
        Some(parts[1].trim().to_string())
    } else {
        None
    };

    Ok((node, reference))
}

/// Parse voltage output specification from stream:
/// - `V(node)`
/// - `V(node,ref)`
/// - bare `node`
pub(super) fn parse_voltage_output_reference(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<(String, Option<String>), ParseError> {
    let first = expect_ident(stream, line_num)?;

    if first.to_uppercase() == "V" && matches!(stream.peek().kind, TokenKind::LParen) {
        stream.advance(); // (
        let node = expect_node(stream, line_num)?;
        let reference = if stream.consume(&TokenKind::Comma) {
            Some(expect_node(stream, line_num)?)
        } else {
            None
        };
        if !stream.consume(&TokenKind::RParen) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected ')' in V(node) specification".to_string(),
            });
        }
        return Ok((node, reference));
    }

    if first.to_uppercase().starts_with("V(") {
        return parse_voltage_reference(&first);
    }

    Ok((first, None))
}

/// Parse .NODESET command: .NODESET V(node1)=val V(node2)=val...
pub(super) fn parse_nodeset_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    node_sets: &mut Vec<NodeSet>,
    defer_values: bool,
) -> Result<Vec<(String, Option<String>)>, ParseError> {
    let mut authored_nodes = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let Some(target) = parse_voltage_hint_target(stream, line_num)? else {
            break;
        };

        require_voltage_hint_assignment(stream, line_num, ".NODESET", &target)?;
        let (voltage, voltage_expr) =
            parse_voltage_hint_value(stream, line_num, params, defer_values)?;
        validate_voltage_hint_value(voltage, defer_values, line_num, ".NODESET")?;
        node_sets.push(NodeSet {
            node: target.node,
            reference: target.reference,
            voltage,
            voltage_expr,
        });
        authored_nodes.push((target.authored_node, target.authored_reference));
    }

    Ok(authored_nodes)
}

/// Parse .IC command: .IC V(node1)=val V(node2)=val...
///
/// Initial conditions set the starting voltages for transient analysis.
/// Format: .IC V(node)=voltage [V(node2)=voltage2] ...
pub(super) fn parse_ic_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    initial_conditions: &mut Vec<InitialCondition>,
    defer_values: bool,
) -> Result<Vec<(String, Option<String>)>, ParseError> {
    let mut authored_nodes = Vec::new();
    while !stream.is_eof() && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let Some(target) = parse_voltage_hint_target(stream, line_num)? else {
            break;
        };

        require_voltage_hint_assignment(stream, line_num, ".IC", &target)?;
        let (voltage, voltage_expr) =
            parse_voltage_hint_value(stream, line_num, params, defer_values)?;
        validate_voltage_hint_value(voltage, defer_values, line_num, ".IC")?;
        initial_conditions.push(InitialCondition {
            node: target.node,
            reference: target.reference,
            voltage,
            voltage_expr,
        });
        authored_nodes.push((target.authored_node, target.authored_reference));
    }

    Ok(authored_nodes)
}

fn parse_voltage_hint_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    defer_values: bool,
) -> Result<(Value, Option<String>), ParseError> {
    if defer_values {
        // A subcircuit-scoped .IC/.NODESET expression must be evaluated in
        // each instance's effective parameter scope. Evaluating a formal
        // parameter here would freeze its definition-time default and discard
        // an X-line override before flattening. Preserve every scoped value
        // expression, while still validating it and retaining the definition
        // scope's value for parser diagnostics and introspection.
        let mut expression_stream = stream.clone();
        let expression = collect_voltage_hint_expression(&mut expression_stream, line_num)?;
        let mut value_stream = stream.clone();
        let voltage = match expect_value(&mut value_stream, line_num, params) {
            Ok(value) => value,
            Err(err) if parameter_error_can_defer(&err) => Value::NAN,
            Err(err) => return Err(err),
        };
        *stream = expression_stream;
        return Ok((voltage, Some(expression)));
    }

    let mut value_stream = stream.clone();
    match expect_value(&mut value_stream, line_num, params) {
        Ok(value) => {
            *stream = value_stream;
            Ok((value, None))
        }
        Err(err) => Err(err),
    }
}

fn collect_voltage_hint_expression(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    let mut expression = String::new();
    let mut saw_token = false;

    loop {
        if matches!(
            stream.peek().kind,
            TokenKind::Newline | TokenKind::Eof | TokenKind::Comma
        ) {
            break;
        }
        if saw_token && looks_like_voltage_hint_target(stream) {
            break;
        }

        let token = stream.peek().clone();
        let fragment = match &token.kind {
            TokenKind::Expression(expr) => expr.clone(),
            TokenKind::Ident(_)
            | TokenKind::Number(_)
            | TokenKind::Equals
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::LParen
            | TokenKind::RParen
            | TokenKind::AtSign
            | TokenKind::Tilde
            | TokenKind::Other(_) => token.lexeme.clone(),
            other => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Expected voltage expression, found {}", other),
                });
            }
        };
        if !expression.is_empty()
            && expression
                .chars()
                .last()
                .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
            && fragment
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            expression.push(' ');
        }
        expression.push_str(&fragment);
        saw_token = true;
        stream.advance();
    }

    if expression.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: "Expected voltage expression".to_string(),
        });
    }

    Ok(expression)
}

fn looks_like_voltage_hint_target(stream: &TokenStream) -> bool {
    matches!(&stream.peek().kind, TokenKind::Ident(ident) if ident.eq_ignore_ascii_case("V"))
        && matches!(stream.peek_n(1).kind, TokenKind::LParen)
}

struct VoltageHintTarget {
    node: String,
    reference: Option<String>,
    authored_node: String,
    authored_reference: Option<String>,
    wrapped: bool,
}

fn require_voltage_hint_assignment(
    stream: &mut TokenStream,
    line_num: usize,
    directive: &str,
    target: &VoltageHintTarget,
) -> Result<(), ParseError> {
    if stream.consume(&TokenKind::Equals) || !target.wrapped {
        return Ok(());
    }
    let rendered = match target.authored_reference.as_deref() {
        Some(reference) => format!("V({},{reference})", target.authored_node),
        None => format!("V({})", target.authored_node),
    };
    Err(ParseError::Syntax {
        line: line_num,
        message: format!(
            "{directive} voltage target {rendered} requires '=value' syntax; use the bare '<node> <value>' form when omitting '='"
        ),
    })
}

fn validate_voltage_hint_value(
    voltage: Value,
    deferred_value: bool,
    line_num: usize,
    directive: &str,
) -> Result<(), ParseError> {
    // A scoped unresolved parameter is represented internally by NaN until
    // each concrete instance's parameter scope is available. Every resolved
    // authored value, including infinities parsed from overflowed literals,
    // must fail here instead of disappearing from the engine hint vector.
    if voltage.is_finite() || deferred_value && voltage.is_nan() {
        return Ok(());
    }
    Err(ParseError::Syntax {
        line: line_num,
        message: format!("{directive} voltage value must be finite, found {voltage}"),
    })
}

fn parse_voltage_hint_target(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<Option<VoltageHintTarget>, ParseError> {
    skip_commas(stream);
    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return Ok(None);
    }

    if let TokenKind::Ident(ident) = &stream.peek().kind
        && ident.eq_ignore_ascii_case("V")
        && matches!(stream.peek_n(1).kind, TokenKind::LParen)
    {
        stream.advance(); // V
        stream.advance(); // (

        let (node, authored_node) = expect_node_with_authored_spelling(stream, line_num)?;
        let (reference, authored_reference) = if stream.consume(&TokenKind::Comma) {
            let (reference, authored_reference) =
                expect_node_with_authored_spelling(stream, line_num)?;
            (Some(reference), Some(authored_reference))
        } else {
            (None, None)
        };

        if !stream.consume(&TokenKind::RParen) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected ')' in voltage target specification".to_string(),
            });
        }
        return Ok(Some(VoltageHintTarget {
            node,
            reference,
            authored_node,
            authored_reference,
            wrapped: true,
        }));
    }

    let (node, authored_node) = expect_node_with_authored_spelling(stream, line_num)?;
    Ok(Some(VoltageHintTarget {
        node,
        reference: None,
        authored_node,
        authored_reference: None,
        wrapped: false,
    }))
}

pub(super) fn expect_node_with_authored_spelling(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<(String, String), ParseError> {
    let mut authored_stream = stream.clone();
    let node = expect_node(stream, line_num)?;
    let end = stream.peek().span.start;
    let mut authored = String::new();
    while authored_stream.peek().span.start < end {
        authored.push_str(&authored_stream.peek().lexeme);
        authored_stream.advance();
    }
    if authored.is_empty() {
        authored = node.clone();
    }
    Ok((node, authored))
}
