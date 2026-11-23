//! Ordered option fields, with failed-pass temperature discovery.
use super::*;
use crate::netlist::parser::option_names::{
    OptionId, option_name, option_package as package_named,
};

mod recovery;

#[allow(clippy::too_many_arguments)]
pub(in crate::netlist::parser) fn parse_options_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    options: &mut super::SimulationOptions,
    unknown_warned: &mut std::collections::HashSet<String>,
    diagnostics: &mut Vec<ParseDiagnostic>,
    mut parameter_direction: Option<&mut ParameterDirectionCapture>,
    control_command: bool,
    mut temperature_options: Option<temperature::TemperatureOptionSink<'_>>,
) -> Result<(), ParseError> {
    let mut option_package: Option<String> = None;
    if control_command {
        stream.require_real_numeric_values();
    }

    while !stream.is_eof() {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        // Xyce's RESTART and OUTPUT packages use deliberately irregular tail
        // grammars: after an interval scalar, bare values occur in
        // `<time> <interval>` pairs. RESTART may reach the tail after other
        // package fields; OUTPUT consumes its tail in the INITIAL_INTERVAL arm
        // so later named OUTPUT fields on the same line remain available.
        if option_package.as_deref() == Some("RESTART")
            && restart_interval_schedule_starts(stream, params)
        {
            if let Err(error) = parse_restart_interval_schedule(stream, line_num, params, options) {
                let Some(sink) = temperature_options.as_mut() else {
                    return Err(error);
                };
                sink.plan.retain_card_error(error, line_num, sink.origin);
                // RESTART's positional schedule owns the rest of this card.
                // Its error remains fatal unless a fresh pass validates it.
                stream.skip_to_eol();
            }
            break;
        }

        let (key, key_end) = expect_option_key(stream, line_num)?;
        let key_upper = key.to_uppercase();
        if control_command
            && !matches!(
                key_upper.as_str(),
                "RELTOL"
                    | "ABSTOL"
                    | "VNTOL"
                    | "GMIN"
                    | "CHGTOL"
                    | "EVENTFLUXTOL"
                    | "TRTOL"
                    | "XMU"
                    | "METHOD"
                    | "ITL1"
                    | "ITL2"
                    | "ITL4"
                    | "TEMP"
                    | "TNOM"
            )
        {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!("control option '{key}' has no runtime handler"),
            });
        }
        let has_equals = stream.consume(&TokenKind::Equals);

        // A value accepted without `=` must still be separated from its key.
        // Xyce treats a fused spelling such as `ABSTOL-1e-6` as one unknown
        // option token; it does not reinterpret the adjacent minus sign as an
        // assigned negative tolerance. Preserve that lexical boundary so a
        // malformed key is diagnosed and ignored instead of becoming a fatal
        // value error for an otherwise valid deck.
        if !has_equals
            && stream.peek().span.start == key_end
            && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
        {
            let fused_key = format!("{key_upper}{}", stream.peek().lexeme.to_uppercase());
            stream.advance();
            let warning_key = option_package
                .as_deref()
                .map_or(fused_key.clone(), |package| {
                    format!("{package}.{fused_key}")
                });
            ignore_unknown_option(
                stream,
                line_num,
                params,
                false,
                &warning_key,
                unknown_warned,
                diagnostics,
            );
            continue;
        }

        // A package that does not open alone opens before one of its own
        // names: `LINSOL TR_PARTITION`.
        let opens_before_next = package_named(&key_upper).is_some_and(|package| {
            !package.opens_alone
                && matches!(&stream.peek().kind, TokenKind::Ident(next)
                    if package.lists(&next.to_ascii_uppercase()))
        });
        if !has_equals && (option_package_key_is_known(&key_upper) || opens_before_next) {
            if key_upper == "RESTART" {
                options.restart.get_or_insert_default();
            }
            option_package = Some(key_upper);
            continue;
        }

        let scoped_key = option_package
            .as_deref()
            .map(|package| format!("{package}.{key_upper}"));

        // The table says which names are read where; one it has no row for
        // here is an unknown option, diagnosed and passed over.
        let Some(row) = option_name(option_package.as_deref(), &key_upper) else {
            let warning_key = scoped_key.as_deref().unwrap_or(&key_upper);
            ignore_unknown_option(
                stream,
                line_num,
                params,
                has_equals,
                warning_key,
                unknown_warned,
                diagnostics,
            );
            continue;
        };
        let value_start = stream.checkpoint();
        let result = (|| -> Result<(), ParseError> {
            match row.id {
                OptionId::RspiceStatistics => {
                    let known = matches!(&stream.peek().kind, TokenKind::Ident(name)
                        if name.eq_ignore_ascii_case("SAMPLE")
                            || name.eq_ignore_ascii_case("NOMINAL"));
                    if !known {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "RSPICE_STATISTICS expects SAMPLE or NOMINAL".to_owned(),
                        });
                    }
                    stream.advance();
                }
                OptionId::RspiceExpRepeat => {
                    let known = matches!(&stream.peek().kind, TokenKind::Ident(name)
                        if name.eq_ignore_ascii_case("QSPICE")
                            || name.eq_ignore_ascii_case("LTSPICE"));
                    if !known {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "RSPICE_EXP_REPEAT expects QSPICE or LTSPICE".to_owned(),
                        });
                    }
                    stream.advance();
                }
                OptionId::RspiceSource => {
                    let simulator = match &stream.peek().kind {
                        TokenKind::Ident(name) if name.eq_ignore_ascii_case("QSPICE") => {
                            crate::netlist::SourceSimulator::Qspice
                        }
                        TokenKind::Ident(name) if name.eq_ignore_ascii_case("LTSPICE") => {
                            crate::netlist::SourceSimulator::Ltspice
                        }
                        _ => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: "RSPICE_SOURCE expects QSPICE or LTSPICE".to_owned(),
                            });
                        }
                    };
                    options.source_simulator = Some(simulator);
                    options.vdmos_subthres_is_ltspice =
                        simulator == crate::netlist::SourceSimulator::Ltspice;
                    options.undeclared_subcircuit_m_is_ignored = true;
                    stream.advance();
                }
                OptionId::RspiceDialect => {
                    let dialect = match &stream.peek().kind {
                        TokenKind::Ident(name) => match name.to_ascii_uppercase().as_str() {
                            "BEST_AVAILABLE" => Some(crate::config::SpiceDialect::BestAvailable),
                            "NGSPICE" => Some(crate::config::SpiceDialect::Ngspice),
                            "XYCE" => Some(crate::config::SpiceDialect::Xyce),
                            _ => None,
                        },
                        _ => None,
                    }
                    .ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: "RSPICE_DIALECT expects BEST_AVAILABLE, NGSPICE, or XYCE"
                            .to_owned(),
                    })?;
                    stream.advance();
                    options.spice_dialect = Some(dialect);
                }
                OptionId::Seed => {
                    // The parse pre-scan applies the seed before any parameter
                    // evaluation; this arm validates and records it for
                    // downstream drivers (e.g. per-run Monte-Carlo streams).
                    if let TokenKind::Ident(word) = &stream.peek().kind
                        && word.eq_ignore_ascii_case("random")
                    {
                        stream.advance();
                        log::warn!(
                            "line {line_num}: '.options seed=random' is not supported; \
                             the existing deterministic seed is retained (set an explicit \
                             integer seed to vary the stream)"
                        );
                        return Ok(());
                    }
                    options.seed = Some(expect_seed_option(stream, line_num)?);
                }
                OptionId::FftMode => {
                    let value = expect_value(stream, line_num, params)?;
                    let mode = parse_usize_option("FFT.FFT_MODE", value, line_num)?;
                    options.fft_mode = Some(match mode {
                        0 => crate::netlist::XyceFftMode::HspiceCompatible,
                        1 => crate::netlist::XyceFftMode::SpectreCompatible,
                        _ => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "FFT.FFT_MODE must be either 0 or 1, found {mode}"
                                ),
                            });
                        }
                    });
                }
                OptionId::FftAccurate => {
                    options.fft_accurate = Some(parse_fft_boolean_option(
                        stream,
                        line_num,
                        params,
                        has_equals,
                        "FFT_ACCURATE",
                    )?);
                }
                OptionId::FftOut => {
                    options.fft_output_metrics = Some(parse_fft_boolean_option(
                        stream, line_num, params, has_equals, "FFTOUT",
                    )?);
                }
                OptionId::MeasureFail => {
                    let value = expect_value(stream, line_num, params)?;
                    if !value.is_finite() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!("MEASURE.MEASFAIL must be finite, found {value}"),
                        });
                    }
                    let integer_value = value.trunc();
                    options.measure_fail_output = Some(match integer_value {
                        0.0 => false,
                        1.0 => true,
                        _ => {
                            let message = format!(
                                "MEASURE.MEASFAIL expects 0 or 1; defaulting invalid value {value} to 1"
                            );
                            diagnostics.push(ParseDiagnostic::warning(
                                line_num,
                                "invalid-option-defaulted",
                                message,
                            ));
                            true
                        }
                    });
                }
                OptionId::MeasureDefaultValue => {
                    let value = expect_value(stream, line_num, params)?;
                    if !value.is_finite() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!("MEASURE.DEFAULT_VAL must be finite, found {value}"),
                        });
                    }
                    options.measure_default_value = Some(value);
                }
                OptionId::MeasureUseContFiles => {
                    let value = expect_value(stream, line_num, params)?;
                    if !value.is_finite() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "MEASURE.USE_CONT_FILES must be finite, found {value}"
                            ),
                        });
                    }
                    options.measure_use_cont_files = Some(value.trunc() != 0.0);
                }
                OptionId::MeasureUseLttm => {
                    let value = expect_value(stream, line_num, params)?;
                    if !value.is_finite() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!("MEASURE.USE_LTTM must be finite, found {value}"),
                        });
                    }
                    options.measure_use_lttm = Some(match value.trunc() {
                        0.0 => false,
                        1.0 => true,
                        _ => {
                            let message = format!(
                                "MEASURE.USE_LTTM expects 0 or 1; defaulting invalid value {value} to 1"
                            );
                            diagnostics.push(ParseDiagnostic::warning(
                                line_num,
                                "invalid-option-defaulted",
                                message,
                            ));
                            true
                        }
                    });
                }
                OptionId::NonlinContinuation => {
                    options.nonlinear_continuation = Some(parse_nonlinear_continuation_option(
                        stream, line_num, params,
                    )?);
                }
                OptionId::LocaConparam => {
                    let TokenKind::Ident(name) = &stream.peek().kind else {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "LOCA.CONPARAM expects a parameter name".to_string(),
                        });
                    };
                    options.loca_continuation_parameters.push(name.clone());
                    stream.advance();
                }
                OptionId::HbNumFreq => {
                    // Xyce lists one count per tone, `numfreq=11,7`, or
                    // spells them `numfreq=11 numfreq2=7`.
                    loop {
                        let value = expect_value(stream, line_num, params)?;
                        let count = parse_usize_option("HBINT.NUMFREQ", value, line_num)?;
                        if count == 0 {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: "HBINT.NUMFREQ must be a positive integer, found 0"
                                    .to_string(),
                            });
                        }
                        options.hb_num_frequencies.push(count);
                        if !matches!(stream.peek().kind, TokenKind::Comma)
                            || !matches!(stream.peek_n(1).kind, TokenKind::Number(_))
                        {
                            break;
                        }
                        stream.advance();
                    }
                }
                OptionId::HbSaveIcData => {
                    options.hb_save_ic_data =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::HbTahb => {
                    let value = expect_value(stream, line_num, params)?;
                    let mode = parse_usize_option("HBINT.TAHB", value, line_num)?;
                    options.hb_time_domain_mode = Some(match mode {
                        0 => XyceHbTimeDomainMode::Direct,
                        1 => XyceHbTimeDomainMode::TransientAssisted,
                        2 => XyceHbTimeDomainMode::DcOperatingPoint,
                        _ => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "HBINT.TAHB must be one of 0, 1, or 2, found {mode}"
                                ),
                            });
                        }
                    });
                }
                OptionId::LinsolHbPrecType => {
                    options.linsol_hb_preconditioner =
                        Some(parse_xyce_hb_preconditioner_option(stream, line_num)?);
                }
                OptionId::TopologySupernode => {
                    options.topology_supernode =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::DeviceZeroResistanceTol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_zero_resistance_tol = Some(parse_non_negative_real_option(
                        "DEVICE.ZERORESISTANCETOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::DeviceMinRes => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_min_resistance = Some(parse_non_negative_real_option(
                        "DEVICE.MINRES",
                        value,
                        line_num,
                    )?);
                }
                OptionId::DeviceMinCap => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_min_capacitance = Some(parse_non_negative_real_option(
                        "DEVICE.MINCAP",
                        value,
                        line_num,
                    )?);
                }
                OptionId::Pnjmaxi => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_pnjmaxi =
                        Some(parse_positive_real_option("PNJMAXI", value, line_num)?);
                }
                OptionId::DeviceB3soiGminScaling => {
                    options.b3soi_gmin_scaling =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::DeviceVoltLim => {
                    options.device_voltage_limiting =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::DeviceDebugLevel => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_debug_level =
                        Some(parse_i64_option("DEVICE.DEBUGLEVEL", value, line_num)?);
                }
                OptionId::DeviceSeparateLoad => {
                    options.device_separate_load =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::TimeintDebugLevel => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_debug_level = Some(parse_xyce_i32_option(
                        "TIMEINT.DEBUGLEVEL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::DeviceTryToCompact => {
                    options.device_try_to_compact =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::GlobalTopologySupernode => {
                    options.topology_supernode =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::GlobalZeroResistanceTol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_zero_resistance_tol = Some(parse_non_negative_real_option(
                        "ZERORESISTANCETOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::GlobalMinRes => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_min_resistance =
                        Some(parse_non_negative_real_option("MINRES", value, line_num)?);
                }
                OptionId::GlobalMinCap => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_min_capacitance =
                        Some(parse_non_negative_real_option("MINCAP", value, line_num)?);
                }
                OptionId::GlobalB3soiGminScaling => {
                    options.b3soi_gmin_scaling =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::GlobalTryToCompact => {
                    options.device_try_to_compact =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::LinsolTrPartition => {
                    options.linsol_tr_partition =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::ConnectRules => {
                    options.connect_rules = Some(parse_restart_string_option(
                        stream,
                        line_num,
                        "CONNECTRULES",
                    )?);
                }
                // The HDL random root seed.
                OptionId::HdlSeed => {
                    let value = expect_u64_literal(stream, line_num, "HDLSEED")?;
                    options.hdl_seed =
                        Some(u32::try_from(value).map_err(|_| ParseError::Syntax {
                            line: line_num,
                            message: format!(
                                "HDLSEED must be an unsigned 32-bit integer, found {value}"
                            ),
                        })?);
                }
                // Section 4.3: plusargs, after the command line's.
                OptionId::HdlPlusargs => {
                    let value = parse_restart_string_option(stream, line_num, "HDL_PLUSARGS")?;
                    options.hdl_plusargs =
                        Some(value.split_whitespace().map(str::to_string).collect());
                }
                OptionId::HdlRundir => {
                    options.hdl_rundir =
                        Some(parse_restart_string_option(stream, line_num, "HDL_RUNDIR")?);
                }
                OptionId::HdlDelaySelect => {
                    let selection = match &stream.peek().kind {
                        TokenKind::Ident(word) => crate::config::HdlDelaySelect::parse(word),
                        _ => None,
                    }
                    .ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: "HDL_DELAY_SELECT expects MIN, TYP or MAX".to_owned(),
                    })?;
                    stream.advance();
                    options.hdl_delay_select = Some(selection);
                }
                OptionId::HdlPathDelays => {
                    options.hdl_path_delays =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::HdlTimingChecks => {
                    options.hdl_timing_checks =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::HdlLibmap => {
                    options.hdl_libmap =
                        Some(parse_restart_string_option(stream, line_num, "HDL_LIBMAP")?);
                }
                OptionId::ConnectRulesSource => {
                    options.connect_rules_source = Some(parse_restart_string_option(
                        stream,
                        line_num,
                        "CONNECTRULES_SOURCE",
                    )?);
                }
                OptionId::WrealResolution => {
                    let value = parse_restart_string_option(stream, line_num, "WREAL_RESOLUTION")?;
                    use crate::netlist::WrealResolution;
                    options.wreal_resolution = Some(match value.to_ascii_lowercase().as_str() {
                        "single" => WrealResolution::Single,
                        "sum" => WrealResolution::Sum,
                        "avg" => WrealResolution::Average,
                        "min" => WrealResolution::Minimum,
                        "max" => WrealResolution::Maximum,
                        "4state" => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: "WREAL_RESOLUTION=4state resolves real nets with X and \
                                          Z states, which real-valued nets here do not carry; \
                                          choose single, sum, avg, min or max"
                                    .to_owned(),
                            });
                        }
                        _ => {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "WREAL_RESOLUTION must be single, sum, avg, min or max, \
                                     found '{value}'"
                                ),
                            });
                        }
                    });
                }
                OptionId::AutoBridge => {
                    let (enabled, show_generated) =
                        parse_auto_bridge_option(stream, line_num, params, has_equals)?;
                    options.auto_bridge = Some(enabled);
                    options.auto_bridge_show_generated = Some(show_generated);
                }
                OptionId::NonlinTranReltol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.nonlin_transient_reltol = Some(parse_positive_real_option(
                        "NONLIN-TRAN.RELTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::NonlinTranAbstol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.nonlin_transient_abstol = Some(parse_positive_real_option(
                        "NONLIN-TRAN.ABSTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::NonlinTranDeltaxtol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.nonlin_transient_deltaxtol = Some(parse_positive_real_option(
                        "NONLIN-TRAN.DELTAXTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::NonlinTranRhstol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.nonlin_transient_rhstol = Some(parse_positive_real_option(
                        "NONLIN-TRAN.RHSTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::NonlinTranMaxstep => {
                    let value = expect_value(stream, line_num, params)?;
                    let value = parse_usize_option("NONLIN-TRAN.MAXSTEP", value, line_num)?;
                    if value == 0 {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "NONLIN-TRAN.MAXSTEP must be at least 1".to_string(),
                        });
                    }
                    options.nonlin_transient_maxstep = Some(value);
                }
                OptionId::NonlinHbMaxstep => {
                    let value = expect_value(stream, line_num, params)?;
                    let value = parse_usize_option("NONLIN-HB.MAXSTEP", value, line_num)?;
                    if value == 0 {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "NONLIN-HB.MAXSTEP must be at least 1".to_string(),
                        });
                    }
                    options.nonlin_hb_maxstep = Some(value);
                }
                OptionId::NonlinTranEnforceDeviceConv => {
                    options.nonlin_transient_enforce_device_convergence =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::NonlinTranNox => {
                    options.nonlin_transient_nox =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::TimeintReltol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_reltol = Some(parse_positive_real_option(
                        "TIMEINT.RELTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::TimeintAbstol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_abstol = Some(parse_positive_real_option(
                        "TIMEINT.ABSTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::TimeintDelmax => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_delmax = Some(parse_positive_real_option(
                        "TIMEINT.DELMAX",
                        value,
                        line_num,
                    )?);
                }
                OptionId::TimeintMinTimestep => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_min_timestep = Some(parse_positive_real_option(
                        "TIMEINT.MINTIMESTEP",
                        value,
                        line_num,
                    )?);
                }
                OptionId::TimeintErroption => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_error_control.is_none() {
                        options.timeint_error_control =
                            Some(parse_transient_error_control_option(value, line_num)?);
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.ERROPTION", line_num, diagnostics);
                    }
                }
                OptionId::TimeintMinTimestepsBp => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_min_steps_between_breakpoints.is_none() {
                        options.timeint_min_steps_between_breakpoints =
                            Some(parse_exact_xyce_nonnegative_integer_option(
                                "TIMEINT.MINTIMESTEPSBP",
                                value,
                                line_num,
                            )?);
                    } else {
                        warn_duplicate_packaged_option(
                            "TIMEINT.MINTIMESTEPSBP",
                            line_num,
                            diagnostics,
                        );
                    }
                }
                OptionId::TimeintNlmin => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_nlmin.is_none() {
                        options.timeint_nlmin = Some(parse_exact_xyce_nonnegative_integer_option(
                            "TIMEINT.NLMIN",
                            value,
                            line_num,
                        )?);
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.NLMIN", line_num, diagnostics);
                    }
                }
                OptionId::TimeintNlmax => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_nlmax.is_none() {
                        options.timeint_nlmax = Some(parse_exact_xyce_nonnegative_integer_option(
                            "TIMEINT.NLMAX",
                            value,
                            line_num,
                        )?);
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.NLMAX", line_num, diagnostics);
                    }
                }
                OptionId::TimeintTimestepsReversal => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_timesteps_reversal.is_none() {
                        options.timeint_timesteps_reversal = Some(parse_binary_integer_option(
                            "TIMESTEPSREVERSAL",
                            value,
                            line_num,
                        )?);
                    } else {
                        warn_duplicate_packaged_option(
                            "TIMEINT.TIMESTEPSREVERSAL",
                            line_num,
                            diagnostics,
                        );
                    }
                }
                // N_NLS_Manager.C:277-290 maps any nonzero integer to the
                // positive return code, after Param::getImmutableValue<int>.
                OptionId::TimeintNlNearConv => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_nl_near_convergence.is_none() {
                        options.timeint_nl_near_convergence = Some(
                            parse_xyce_i32_option("TIMEINT.NLNEARCONV", value, line_num)? != 0,
                        );
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.NLNEARCONV", line_num, diagnostics);
                    }
                }
                OptionId::TimeintNlSmallUpdate => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_nl_small_update.is_none() {
                        options.timeint_nl_small_update = Some(
                            parse_xyce_i32_option("TIMEINT.NLSMALLUPDATE", value, line_num)? != 0,
                        );
                    } else {
                        warn_duplicate_packaged_option(
                            "TIMEINT.NLSMALLUPDATE",
                            line_num,
                            diagnostics,
                        );
                    }
                }
                // Which derivative columns a `.SENS` table prints; Xyce reads
                // each as a boolean (N_IO_OutputMgr.C:2845-2870).
                OptionId::SensitivityColumn => {
                    let name = format!("SENSITIVITY.{key_upper}");
                    let value = expect_value(stream, line_num, params)?;
                    let enabled = parse_xyce_i32_option(&name, value, line_num)? != 0;
                    let slot = match key_upper.as_str() {
                        "DIRECT" => &mut options.sensitivity_direct,
                        "ADJOINT" => &mut options.sensitivity_adjoint,
                        "OUTPUTSCALED" => &mut options.sensitivity_output_scaled,
                        _ => &mut options.sensitivity_output_unscaled,
                    };
                    if slot.is_none() {
                        *slot = Some(enabled);
                    } else {
                        warn_duplicate_packaged_option(&name, line_num, diagnostics);
                    }
                }
                OptionId::TimeintMinord => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_min_order.is_none() {
                        options.timeint_min_order = Some(parse_xyce_transient_order_option(
                            "TIMEINT.MINORD",
                            value,
                            line_num,
                            false,
                        )?);
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.MINORD", line_num, diagnostics);
                    }
                }
                OptionId::TimeintMaxord => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_max_order.is_none() {
                        options.timeint_max_order = Some(parse_xyce_transient_order_option(
                            "TIMEINT.MAXORD",
                            value,
                            line_num,
                            true,
                        )?);
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.MAXORD", line_num, diagnostics);
                    }
                }
                OptionId::TimeintBreakpoints => {
                    let values = parse_time_point_vector_option(
                        stream,
                        line_num,
                        params,
                        "TIMEINT.BREAKPOINTS",
                    )?;
                    append_canonical_time_points(&mut options.timeint_breakpoints, values);
                }
                // The run's step ceiling is deliberately outside `TIMEINT`: that
                // package's own ceiling is `DELMAX`, and a second key inside it
                // meaning the same thing would make a deck line ambiguous. Being
                // unscoped also means a misplaced `TIMEINT MAXTIMESTEP` is
                // reported by the package's unknown-key arm instead of silently
                // taking effect.
                OptionId::MaxTimestep => {
                    let value = expect_value(stream, line_num, params)?;
                    options.max_timestep =
                        Some(parse_positive_real_option("MAXTIMESTEP", value, line_num)?);
                }
                OptionId::TimeintUseDeviceMax => {
                    options.timeint_use_device_max_timestep =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::TimeintNewLte => {
                    let value = expect_value(stream, line_num, params)?;
                    options.transient_lte_reference =
                        Some(parse_transient_lte_reference_option(value, line_num)?);
                }
                OptionId::TimeintNewBpStepping => {
                    let value = expect_value(stream, line_num, params)?;
                    options.transient_new_bp_stepping =
                        Some(parse_new_breakpoint_stepping_option(value, line_num)?);
                }
                OptionId::TimeintMethod => {
                    options.method = Some(parse_method_option(stream, line_num, params)?);
                }
                OptionId::RestartPack => {
                    let value = parse_restart_boolean_option(
                        stream,
                        line_num,
                        params,
                        has_equals,
                        "RESTART.PACK",
                    )?;
                    options.restart.get_or_insert_default().pack = Some(value);
                }
                OptionId::RestartPrintTimeintOptions => {
                    let value = parse_restart_boolean_option(
                        stream,
                        line_num,
                        params,
                        has_equals,
                        "RESTART.PRINT_TIMEINT_OPTIONS",
                    )?;
                    options
                        .restart
                        .get_or_insert_default()
                        .print_timeint_options = Some(value);
                }
                OptionId::RestartJob => {
                    let value = parse_restart_string_option(stream, line_num, "RESTART.JOB")?;
                    options.restart.get_or_insert_default().job = Some(value);
                }
                OptionId::RestartStartTime => {
                    let value = expect_value(stream, line_num, params)?;
                    let value =
                        parse_non_negative_real_option("RESTART.START_TIME", value, line_num)?;
                    options.restart.get_or_insert_default().start_time = Some(value);
                }
                OptionId::RestartFile => {
                    let value = parse_restart_string_option(stream, line_num, "RESTART.FILE")?;
                    options.restart.get_or_insert_default().file = Some(value);
                }
                OptionId::RestartInitialInterval => {
                    let value = expect_value(stream, line_num, params)?;
                    let value =
                        parse_positive_real_option("RESTART.INITIAL_INTERVAL", value, line_num)?;
                    options.restart.get_or_insert_default().initial_interval = Some(value);
                }
                // Spelled unscoped, as ngspice spells `bypass`. A `BYPASS` package
                // would have to enumerate every key it accepts before an
                // unenumerated one leaked back out to the global namespace, and
                // three keys do not need a namespace to stay apart.
                OptionId::Bypass => {
                    options.bypass =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::BypassReltol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.bypass_reltol = Some(parse_non_negative_real_option(
                        "BYPASSRELTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::BypassAbstol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.bypass_abstol = Some(parse_non_negative_real_option(
                        "BYPASSABSTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::Reltol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.reltol = Some(parse_positive_real_option("RELTOL", value, line_num)?);
                }
                OptionId::Abstol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.abstol = Some(parse_positive_real_option("ABSTOL", value, line_num)?);
                }
                OptionId::Vntol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.vntol = Some(parse_positive_real_option("VNTOL", value, line_num)?);
                }
                OptionId::Iabstol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.iabstol = Some(parse_positive_real_option("IABSTOL", value, line_num)?);
                }
                OptionId::ResidualReltol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.residual_reltol = Some(parse_positive_real_option(
                        "RESIDUAL_RELTOL",
                        value,
                        line_num,
                    )?);
                }
                OptionId::Gmin => {
                    let value = expect_value(stream, line_num, params)?;
                    options.gmin = Some(parse_non_negative_real_option("GMIN", value, line_num)?);
                }
                OptionId::Rshunt => {
                    let value = expect_value_with_direction(
                        stream,
                        line_num,
                        params,
                        parameter_direction
                            .as_deref_mut()
                            .map(|capture| capture.rshunt.insert(0.0.into())),
                    )?;
                    options.rshunt = Some(parse_positive_real_option("RSHUNT", value, line_num)?);
                }
                OptionId::Gshunt => {
                    let value = expect_value(stream, line_num, params)?;
                    options.gshunt =
                        Some(parse_non_negative_real_option("GSHUNT", value, line_num)?);
                }
                OptionId::Cshunt => {
                    let value = expect_value_with_direction(
                        stream,
                        line_num,
                        params,
                        parameter_direction
                            .as_deref_mut()
                            .map(|capture| capture.cshunt.insert(0.0.into())),
                    )?;
                    options.cshunt = Some(parse_positive_real_option("CSHUNT", value, line_num)?);
                }
                OptionId::Xmu
                    if params.expression_dialect() != crate::config::ExpressionDialect::Xyce =>
                {
                    let value = expect_value(stream, line_num, params)?;
                    options.xmu = Some(parse_xmu_option(value, line_num)?);
                }
                OptionId::Trtol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.trtol = Some(parse_positive_real_option("TRTOL", value, line_num)?);
                }
                OptionId::Ramptime => {
                    let value = expect_value(stream, line_num, params)?;
                    options.ramptime =
                        Some(parse_non_negative_real_option("RAMPTIME", value, line_num)?);
                }
                OptionId::MaxEvtIter => {
                    let value = expect_value(stream, line_num, params)?;
                    let passes = parse_usize_option("MAXEVTITER", value, line_num)?;
                    // ngspice's EVTiter runs `passes < max_event_passes`
                    // (xspice/evt/evtiter.c:146), so under 0 it runs no pass
                    // and refuses every settle as too many passes.
                    options.max_event_passes = Some(
                        u32::try_from(passes)
                            .ok()
                            .filter(|passes| *passes >= 1)
                            .ok_or_else(|| ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "MAXEVTITER must be from 1 to {}, found {passes}: it is \
                                     the passes an event settle may take",
                                    u32::MAX
                                ),
                            })?,
                    );
                }
                OptionId::DigitalDelayType => {
                    let value = expect_value(stream, line_num, params)?;
                    options.digital_delay_type =
                        Some(parse_digital_delay_type_option(value, line_num)?);
                }
                OptionId::EventSave => {
                    options.xspice_event_trace_save = Some(
                        if parse_boolean_option(stream, line_num, params, has_equals)? {
                            crate::netlist::EventSaveSelection::All
                        } else {
                            crate::netlist::EventSaveSelection::None
                        },
                    );
                }
                OptionId::Chgtol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.chgtol = Some(parse_positive_real_option("CHGTOL", value, line_num)?);
                }
                OptionId::EventFluxTol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.eventfluxtol =
                        Some(parse_positive_real_option("EVENTFLUXTOL", value, line_num)?);
                }
                OptionId::Pivtol => {
                    let value = expect_value(stream, line_num, params)?;
                    options.pivtol = Some(parse_positive_real_option("PIVTOL", value, line_num)?);
                }
                OptionId::Pivrel => {
                    let value = expect_value(stream, line_num, params)?;
                    options.pivrel = Some(parse_positive_real_option("PIVREL", value, line_num)?);
                }
                OptionId::Temp => {
                    if let Some(sink) = temperature_options.as_mut() {
                        sink.parse(
                            temperature::TemperatureOption::Temp,
                            stream,
                            line_num,
                            params,
                            options,
                        )?;
                    } else {
                        let value = expect_value(stream, line_num, params)?;
                        options.temp = Some(parse_celsius_option("TEMP", value, line_num)?);
                    }
                }
                OptionId::Tnom => {
                    if let Some(sink) = temperature_options.as_mut() {
                        sink.parse(
                            temperature::TemperatureOption::Tnom,
                            stream,
                            line_num,
                            params,
                            options,
                        )?;
                    } else {
                        let value = expect_value(stream, line_num, params)?;
                        options.tnom = Some(parse_celsius_option("TNOM", value, line_num)?);
                    }
                }
                OptionId::Scale => {
                    // Element geometry scale factor (`.options scale`, Xyce's
                    // `.options parser scale`). The circuit builder multiplies
                    // the drawn instance geometry by it once flattened.
                    let value = expect_value(stream, line_num, params)?;
                    options.scale = Some(parse_positive_real_option("SCALE", value, line_num)?);
                }
                OptionId::Itl1 => {
                    let value = expect_value(stream, line_num, params)?;
                    options.itl1 = Some(parse_usize_option("ITL1", value, line_num)?);
                }
                OptionId::Itl2 => {
                    let value = expect_value(stream, line_num, params)?;
                    options.itl2 = Some(parse_usize_option("ITL2", value, line_num)?);
                }
                OptionId::Itl4 => {
                    let value = expect_value(stream, line_num, params)?;
                    options.itl4 = Some(parse_usize_option("ITL4", value, line_num)?);
                }
                OptionId::Itl6 => {
                    let value = expect_value(stream, line_num, params)?;
                    options.itl6 = Some(parse_usize_option("ITL6", value, line_num)?);
                }
                OptionId::Method => {
                    options.method = Some(parse_method_option(stream, line_num, params)?);
                }
                // The continuation ladder's rungs are switched one at a time
                // rather than through `NONLIN CONTINUATION`, which selects a
                // single algorithm to run in place of the ladder.
                OptionId::GminStepping => {
                    options.gmin_stepping =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::SourceStepping => {
                    options.source_stepping =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::PseudoTransient => {
                    options.pseudo_transient =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::ArcLength => {
                    options.arc_length =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::Damping => {
                    options.damping_strategy =
                        Some(parse_damping_option(stream, line_num, params)?);
                }
                OptionId::Solver => {
                    options.matrix_solver = Some(parse_matrix_solver_option(stream, line_num)?);
                }
                OptionId::OutputInitialInterval => {
                    if !options.output_time_points.is_empty() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "Cannot specify both .OPTIONS OUTPUT INITIAL_INTERVAL and OUTPUTTIMEPOINTS".to_string(),
                        });
                    }
                    let value = expect_value(stream, line_num, params)?;
                    let initial_interval =
                        parse_positive_real_option("OUTPUT.INITIAL_INTERVAL", value, line_num)?;
                    let intervals = parse_output_interval_schedule(stream, line_num, params)?;
                    options.output_interval_schedule =
                        Some(crate::netlist::XyceOutputIntervalSchedule {
                            initial_interval,
                            intervals,
                        });
                }
                OptionId::OutputTimePoints => {
                    if options.output_interval_schedule.is_some() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "Cannot specify both .OPTIONS OUTPUT INITIAL_INTERVAL and OUTPUTTIMEPOINTS".to_string(),
                        });
                    }
                    let values = parse_time_point_vector_option(
                        stream,
                        line_num,
                        params,
                        "OUTPUT.OUTPUTTIMEPOINTS",
                    )?;
                    append_canonical_time_points(&mut options.output_time_points, values);
                }
                OptionId::OutputSnapshots => {
                    options.output_snapshots =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::OutputPrintHeader => {
                    options.output_print_header =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::OutputPrintFooter => {
                    options.output_print_footer =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::OutputPhaseRadians => {
                    options.output_phase_radians =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::Interp => {
                    options.interp =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                OptionId::Noacct => {
                    // Ngspice compatibility flag: NOACCT suppresses accounting
                    // output and does not change the solved circuit state.
                    if has_equals {
                        let _ = expect_value(stream, line_num, params)?;
                    }
                }
                OptionId::AllowSimplifiedMos => {
                    // Bare flag enables; an explicit value of 0 disables.
                    let enabled = if has_equals {
                        expect_value(stream, line_num, params)? != 0.0
                    } else {
                        true
                    };
                    options.allow_simplified_mos = Some(enabled);
                }
                // Under Xyce expressions XMU is no option: Xyce has none.
                OptionId::Xmu => {
                    let warning_key = scoped_key.as_deref().unwrap_or(&key_upper);
                    ignore_unknown_option(
                        stream,
                        line_num,
                        params,
                        has_equals,
                        warning_key,
                        unknown_warned,
                        diagnostics,
                    );
                }
            }
            Ok(())
        })();
        if let Err(error) = result {
            if matches!(error, ParseError::ResourceLimit(_)) {
                return Err(error);
            }
            let Some(sink) = temperature_options.as_mut() else {
                return Err(error);
            };
            sink.plan.retain_card_error(error, line_num, sink.origin);
            // This pass is already invalid. Consume the failed field through
            // its own grammar boundary so later assignments can select TEMP
            // or TNOM. A fresh pass must validate every field before success.
            recovery::consume_failed_field(stream, value_start, row.id, params);
        }
    }

    Ok(())
}
