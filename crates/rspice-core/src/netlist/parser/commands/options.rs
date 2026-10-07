//! Ordered option fields, with failed-pass temperature discovery.
use super::*;

mod recovery;

#[allow(clippy::too_many_arguments)]
pub(in crate::netlist::parser) fn parse_options_command(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    options: &mut super::SimulationOptions,
    max_analysis_points: usize,
    unknown_warned: &mut std::collections::HashSet<String>,
    diagnostics: &mut Vec<ParseDiagnostic>,
    mut parameter_direction: Option<&mut ParameterDirectionCapture>,
    control_command: bool,
    mut temperature_options: Option<temperature::TemperatureOptionSink<'_>>,
) -> Result<(), ParseError> {
    let mut option_package: Option<String> = None;
    let expect_value = |stream: &mut TokenStream, line_num, params: &ParamContext| {
        if control_command {
            expect_control_real_value(stream, line_num, params)
        } else {
            super::expect_value(stream, line_num, params)
        }
    };

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
            if let Err(error) = parse_restart_interval_schedule(
                stream,
                line_num,
                params,
                options,
                max_analysis_points,
            ) {
                if matches!(error, ParseError::ResourceLimit(_)) {
                    return Err(error);
                }
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

        let is_supported_linsol_package = key_upper == "LINSOL"
            && matches!(&stream.peek().kind, TokenKind::Ident(next)
                if next.eq_ignore_ascii_case("TR_PARTITION")
                    || next.eq_ignore_ascii_case("TRPARTITION"));
        if !has_equals && (option_package_key_is_known(&key_upper) || is_supported_linsol_package) {
            if key_upper == "RESTART" {
                options.restart.get_or_insert_default();
            }
            option_package = Some(key_upper);
            continue;
        }

        let scoped_key = option_package
            .as_deref()
            .map(|package| format!("{package}.{key_upper}"));

        let value_start = stream.checkpoint();
        let result = (|| -> Result<(), ParseError> {
            match (option_package.as_deref(), key_upper.as_str()) {
                (None, "RSPICE_DIALECT") => {
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
                (package, "SEED" | "RNDSEED") if seed_option_applies_to_package(package) => {
                    // The parse pre-scan applies the seed before any parameter
                    // evaluation; this arm validates and records it for
                    // downstream drivers (e.g. per-run Monte-Carlo streams).
                    if let TokenKind::Ident(word) = &stream.peek().kind
                        && word.eq_ignore_ascii_case("random")
                    {
                        stream.advance();
                        log::warn!(
                            "line {line_num}: `.options seed=random` is not supported; \
                             the existing deterministic seed is retained (set an explicit \
                             integer seed to vary the stream)"
                        );
                        return Ok(());
                    }
                    options.seed = Some(expect_seed_option(stream, line_num)?);
                }
                (Some("FFT"), "FFT_MODE") => {
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
                (Some("FFT"), "FFT_ACCURATE") => {
                    options.fft_accurate = Some(parse_fft_boolean_option(
                        stream,
                        line_num,
                        params,
                        has_equals,
                        "FFT_ACCURATE",
                    )?);
                }
                (Some("FFT"), "FFTOUT") => {
                    options.fft_output_metrics = Some(parse_fft_boolean_option(
                        stream, line_num, params, has_equals, "FFTOUT",
                    )?);
                }
                (Some("FFT"), _) => {
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
                (Some("MEASURE"), "MEASFAIL") => {
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
                            log::warn!("line {line_num}: {message}");
                            diagnostics.push(ParseDiagnostic::warning(
                                line_num,
                                "invalid-option-defaulted",
                                message,
                            ));
                            true
                        }
                    });
                }
                (Some("MEASURE"), "DEFAULT_VAL") => {
                    let value = expect_value(stream, line_num, params)?;
                    if !value.is_finite() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: format!("MEASURE.DEFAULT_VAL must be finite, found {value}"),
                        });
                    }
                    options.measure_default_value = Some(value);
                }
                (Some("MEASURE"), "USE_CONT_FILES") => {
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
                (Some("MEASURE"), "USE_LTTM") => {
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
                            log::warn!("line {line_num}: {message}");
                            diagnostics.push(ParseDiagnostic::warning(
                                line_num,
                                "invalid-option-defaulted",
                                message,
                            ));
                            true
                        }
                    });
                }
                (Some("MEASURE"), _) => {
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
                (Some("NONLIN"), "CONTINUATION")
                | (Some("NONLIN-CONTINUATION"), "CONTINUATION") => {
                    options.nonlinear_continuation = Some(parse_nonlinear_continuation_option(
                        stream, line_num, params,
                    )?);
                }
                (Some("HBINT"), key) if key.starts_with("NUMFREQ") => {
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
                }
                (Some("HBINT"), "SAVEICDATA") => {
                    options.hb_save_ic_data =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("HBINT"), "TAHB") => {
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
                (Some("HBINT"), _) => {
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
                (Some("LINSOL-HB"), "PREC_TYPE") => {
                    options.linsol_hb_preconditioner =
                        Some(parse_xyce_hb_preconditioner_option(stream, line_num)?);
                }
                (Some("LINSOL-HB"), _) => {
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
                (Some("TOPOLOGY"), "SUPERNODE") => {
                    options.topology_supernode =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("DEVICE"), "ZERORESISTANCETOL" | "ZERO_RESISTANCE_TOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_zero_resistance_tol = Some(parse_non_negative_real_option(
                        "DEVICE.ZERORESISTANCETOL",
                        value,
                        line_num,
                    )?);
                }
                (Some("DEVICE"), "MINRES" | "MIN_RES") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_min_resistance = Some(parse_non_negative_real_option(
                        "DEVICE.MINRES",
                        value,
                        line_num,
                    )?);
                }
                (Some("DEVICE"), "MINCAP" | "MIN_CAP") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_min_capacitance = Some(parse_non_negative_real_option(
                        "DEVICE.MINCAP",
                        value,
                        line_num,
                    )?);
                }
                (Some("DEVICE") | None, "PNJMAXI") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_pnjmaxi =
                        Some(parse_positive_real_option("PNJMAXI", value, line_num)?);
                }
                (Some("DEVICE"), "B3SOIGMINSCALING" | "B3SOI_GMIN_SCALING") => {
                    options.b3soi_gmin_scaling =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("DEVICE"), "VOLTLIM" | "VOLT_LIM") => {
                    options.device_voltage_limiting =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("DEVICE"), "DEBUGLEVEL" | "DEBUG_LEVEL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_debug_level =
                        Some(parse_i64_option("DEVICE.DEBUGLEVEL", value, line_num)?);
                }
                (Some("DEVICE"), "SEPARATELOAD" | "SEPARATE_LOAD") => {
                    options.device_separate_load =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("TIMEINT"), "DEBUGLEVEL" | "DEBUG_LEVEL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_debug_level = Some(parse_xyce_i32_option(
                        "TIMEINT.DEBUGLEVEL",
                        value,
                        line_num,
                    )?);
                }
                (Some("DEVICE"), "TRYTOCOMPACT" | "TRY_TO_COMPACT") => {
                    options.device_try_to_compact =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (None, "TOPOLOGY_SUPERNODE" | "TOPOLOGYSUPERNODE") => {
                    options.topology_supernode =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (
                    None,
                    "DEVICE_ZERORESISTANCETOL"
                    | "DEVICEZERORESISTANCETOL"
                    | "ZERORESISTANCETOL"
                    | "ZERO_RESISTANCE_TOL",
                ) => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_zero_resistance_tol = Some(parse_non_negative_real_option(
                        "ZERORESISTANCETOL",
                        value,
                        line_num,
                    )?);
                }
                (None, "MINRES" | "MIN_RES" | "DEVICE_MINRES" | "DEVICEMINRES") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_min_resistance =
                        Some(parse_non_negative_real_option("MINRES", value, line_num)?);
                }
                (None, "MINCAP" | "MIN_CAP" | "DEVICE_MINCAP" | "DEVICE_MIN_CAP") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.device_min_capacitance =
                        Some(parse_non_negative_real_option("MINCAP", value, line_num)?);
                }
                (None, "B3SOIGMINSCALING" | "B3SOI_GMIN_SCALING" | "DEVICE_B3SOIGMINSCALING") => {
                    options.b3soi_gmin_scaling =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (None, "TRYTOCOMPACT" | "TRY_TO_COMPACT" | "DEVICE_TRYTOCOMPACT") => {
                    options.device_try_to_compact =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("LINSOL"), "TR_PARTITION" | "TRPARTITION") => {
                    options.linsol_tr_partition =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (None, "CONNECTRULES") => {
                    options.connect_rules = Some(parse_restart_string_option(
                        stream,
                        line_num,
                        "CONNECTRULES",
                    )?);
                }
                (None, "CONNECTRULES_SOURCE") => {
                    options.connect_rules_source = Some(parse_restart_string_option(
                        stream,
                        line_num,
                        "CONNECTRULES_SOURCE",
                    )?);
                }
                (Some("XSPICE"), "AUTO_BRIDGE" | "AUTOBRIDGE")
                | (None, "AUTO_BRIDGE" | "AUTOBRIDGE" | "XSPICE_AUTO_BRIDGE") => {
                    let (enabled, show_generated) =
                        parse_auto_bridge_option(stream, line_num, params, has_equals)?;
                    options.auto_bridge = Some(enabled);
                    options.auto_bridge_show_generated = Some(show_generated);
                }
                (Some("NONLIN-TRAN"), "RELTOL") | (Some("NONLIN-TRANSIENT"), "RELTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.nonlin_transient_reltol = Some(parse_positive_real_option(
                        "NONLIN-TRAN.RELTOL",
                        value,
                        line_num,
                    )?);
                }
                (Some("NONLIN-TRAN"), "ABSTOL") | (Some("NONLIN-TRANSIENT"), "ABSTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.nonlin_transient_abstol = Some(parse_positive_real_option(
                        "NONLIN-TRAN.ABSTOL",
                        value,
                        line_num,
                    )?);
                }
                (Some("NONLIN-TRAN"), "DELTAXTOL") | (Some("NONLIN-TRANSIENT"), "DELTAXTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.nonlin_transient_deltaxtol = Some(parse_positive_real_option(
                        "NONLIN-TRAN.DELTAXTOL",
                        value,
                        line_num,
                    )?);
                }
                (Some("NONLIN-TRAN"), "RHSTOL") | (Some("NONLIN-TRANSIENT"), "RHSTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.nonlin_transient_rhstol = Some(parse_positive_real_option(
                        "NONLIN-TRAN.RHSTOL",
                        value,
                        line_num,
                    )?);
                }
                (Some("NONLIN-TRAN"), "MAXSTEP") | (Some("NONLIN-TRANSIENT"), "MAXSTEP") => {
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
                (Some("NONLIN-HB"), "MAXSTEP") => {
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
                (Some("NONLIN-HB"), _) => {
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
                (Some("NONLIN-TRAN"), "ENFORCEDEVICECONV" | "ENFORCE_DEVICE_CONV")
                | (Some("NONLIN-TRANSIENT"), "ENFORCEDEVICECONV" | "ENFORCE_DEVICE_CONV") => {
                    options.nonlin_transient_enforce_device_convergence =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("NONLIN-TRAN"), "NOX") | (Some("NONLIN-TRANSIENT"), "NOX") => {
                    options.nonlin_transient_nox =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("NONLIN-TRAN"), _) | (Some("NONLIN-TRANSIENT"), _) => {
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
                (Some("TIMEINT"), "RELTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_reltol = Some(parse_positive_real_option(
                        "TIMEINT.RELTOL",
                        value,
                        line_num,
                    )?);
                }
                (Some("TIMEINT"), "ABSTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_abstol = Some(parse_positive_real_option(
                        "TIMEINT.ABSTOL",
                        value,
                        line_num,
                    )?);
                }
                (Some("TIMEINT"), "DELMAX") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_delmax = Some(parse_positive_real_option(
                        "TIMEINT.DELMAX",
                        value,
                        line_num,
                    )?);
                }
                (Some("TIMEINT"), "MINTIMESTEP" | "MIN_TIMESTEP") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.timeint_min_timestep = Some(parse_positive_real_option(
                        "TIMEINT.MINTIMESTEP",
                        value,
                        line_num,
                    )?);
                }
                (Some("TIMEINT"), "ERROPTION") => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_error_control.is_none() {
                        options.timeint_error_control =
                            Some(parse_transient_error_control_option(value, line_num)?);
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.ERROPTION", line_num, diagnostics);
                    }
                }
                (Some("TIMEINT"), "MINTIMESTEPSBP") => {
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
                (Some("TIMEINT"), "NLMIN") => {
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
                (Some("TIMEINT"), "NLMAX") => {
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
                (Some("TIMEINT"), "TIMESTEPSREVERSAL") => {
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
                (Some("TIMEINT"), "MINORD") => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_min_order.is_none() {
                        options.timeint_min_order = Some(parse_xyce_transient_order_option(
                            "TIMEINT.MINORD",
                            value,
                            line_num,
                        )?);
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.MINORD", line_num, diagnostics);
                    }
                }
                (Some("TIMEINT"), "MAXORD") => {
                    let value = expect_value(stream, line_num, params)?;
                    if options.timeint_max_order.is_none() {
                        options.timeint_max_order = Some(parse_xyce_transient_order_option(
                            "TIMEINT.MAXORD",
                            value,
                            line_num,
                        )?);
                    } else {
                        warn_duplicate_packaged_option("TIMEINT.MAXORD", line_num, diagnostics);
                    }
                }
                (Some("TIMEINT"), "BREAKPOINTS") => {
                    let values = parse_time_point_vector_option(
                        stream,
                        line_num,
                        params,
                        "TIMEINT.BREAKPOINTS",
                        options
                            .timeint_breakpoints
                            .len()
                            .saturating_add(options.output_time_points.len())
                            .saturating_add(output_interval_count(options))
                            .saturating_add(restart_interval_count(options)),
                        max_analysis_points,
                    )?;
                    append_canonical_time_points(
                        &mut options.timeint_breakpoints,
                        values,
                        &options.output_time_points,
                        max_analysis_points,
                    )?;
                }
                // The run's step ceiling is deliberately outside `TIMEINT`: that
                // package's own ceiling is `DELMAX`, and a second key inside it
                // meaning the same thing would make a deck line ambiguous. Being
                // unscoped also means a misplaced `TIMEINT MAXTIMESTEP` is
                // reported by the package's unknown-key arm instead of silently
                // taking effect.
                (None, "MAXTIMESTEP" | "MAX_TIMESTEP") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.max_timestep =
                        Some(parse_positive_real_option("MAXTIMESTEP", value, line_num)?);
                }
                (Some("TIMEINT"), "USEDEVICEMAX" | "USE_DEVICE_MAX") => {
                    options.timeint_use_device_max_timestep =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("TIMEINT"), "NEWLTE") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.transient_lte_reference =
                        Some(parse_transient_lte_reference_option(value, line_num)?);
                }
                (Some("TIMEINT"), "NEWBPSTEPPING") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.transient_new_bp_stepping =
                        Some(parse_new_breakpoint_stepping_option(value, line_num)?);
                }
                (Some("TIMEINT"), "METHOD") => {
                    options.method = Some(parse_method_option(stream, line_num, params)?);
                }
                (Some("TIMEINT"), _) => {
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
                (Some("RESTART"), "PACK") => {
                    let value = parse_restart_boolean_option(
                        stream,
                        line_num,
                        params,
                        has_equals,
                        "RESTART.PACK",
                    )?;
                    options.restart.get_or_insert_default().pack = Some(value);
                }
                (Some("RESTART"), "PRINT_TIMEINT_OPTIONS" | "PRINTTIMEINTOPTIONS") => {
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
                (Some("RESTART"), "JOB") => {
                    let value = parse_restart_string_option(stream, line_num, "RESTART.JOB")?;
                    options.restart.get_or_insert_default().job = Some(value);
                }
                (Some("RESTART"), "START_TIME" | "STARTTIME") => {
                    let value = expect_value(stream, line_num, params)?;
                    let value =
                        parse_non_negative_real_option("RESTART.START_TIME", value, line_num)?;
                    options.restart.get_or_insert_default().start_time = Some(value);
                }
                (Some("RESTART"), "FILE") => {
                    let value = parse_restart_string_option(stream, line_num, "RESTART.FILE")?;
                    options.restart.get_or_insert_default().file = Some(value);
                }
                (Some("RESTART"), "INITIAL_INTERVAL" | "INITIALINTERVAL") => {
                    let value = expect_value(stream, line_num, params)?;
                    let value =
                        parse_positive_real_option("RESTART.INITIAL_INTERVAL", value, line_num)?;
                    options.restart.get_or_insert_default().initial_interval = Some(value);
                }
                (Some("RESTART"), _) => {
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
                // Spelled unscoped, as ngspice spells `bypass`. A `BYPASS` package
                // would have to enumerate every key it accepts before an
                // unenumerated one leaked back out to the global namespace, and
                // three keys do not need a namespace to stay apart.
                (_, "BYPASS") => {
                    options.bypass =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (_, "BYPASSRELTOL" | "BYPASS_RELTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.bypass_reltol = Some(parse_non_negative_real_option(
                        "BYPASSRELTOL",
                        value,
                        line_num,
                    )?);
                }
                (_, "BYPASSABSTOL" | "BYPASS_ABSTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.bypass_abstol = Some(parse_non_negative_real_option(
                        "BYPASSABSTOL",
                        value,
                        line_num,
                    )?);
                }
                (_, "RELTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.reltol = Some(parse_positive_real_option("RELTOL", value, line_num)?);
                }
                (_, "ABSTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.abstol = Some(parse_positive_real_option("ABSTOL", value, line_num)?);
                }
                (_, "VNTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.vntol = Some(parse_positive_real_option("VNTOL", value, line_num)?);
                }
                (_, "IABSTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.iabstol = Some(parse_positive_real_option("IABSTOL", value, line_num)?);
                }
                (_, "RESIDUAL_RELTOL" | "RESRELTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.residual_reltol = Some(parse_positive_real_option(
                        "RESIDUAL_RELTOL",
                        value,
                        line_num,
                    )?);
                }
                (_, "GMIN") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.gmin = Some(parse_non_negative_real_option("GMIN", value, line_num)?);
                }
                (_, "RSHUNT") => {
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
                (_, "CSHUNT") => {
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
                (None, "XMU")
                    if params.expression_dialect() != crate::config::ExpressionDialect::Xyce =>
                {
                    let value = expect_value(stream, line_num, params)?;
                    options.xmu = Some(parse_xmu_option(value, line_num)?);
                }
                (_, "TRTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.trtol = Some(parse_positive_real_option("TRTOL", value, line_num)?);
                }
                (_, "RAMPTIME") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.ramptime =
                        Some(parse_non_negative_real_option("RAMPTIME", value, line_num)?);
                }
                (Some("XSPICE"), "DIGITAL_DELAY_TYPE" | "DIGITALDELAYTYPE" | "DIGITAL_DELAY")
                | (None, "DIGITAL_DELAY_TYPE" | "DIGITALDELAYTYPE" | "XSPICE_DIGITAL_DELAY_TYPE") =>
                {
                    let value = expect_value(stream, line_num, params)?;
                    options.digital_delay_type =
                        Some(parse_digital_delay_type_option(value, line_num)?);
                }
                (Some("XSPICE"), "ESAVE" | "EVENT_SAVE" | "EVENTSAVE")
                | (None, "XSPICE_ESAVE" | "XSPICE_EVENT_SAVE" | "XSPICE_EVENTSAVE") => {
                    options.xspice_event_trace_save =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (_, "CHGTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.chgtol = Some(parse_positive_real_option("CHGTOL", value, line_num)?);
                }
                (None, "EVENTFLUXTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.eventfluxtol =
                        Some(parse_positive_real_option("EVENTFLUXTOL", value, line_num)?);
                }
                (_, "PIVTOL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.pivtol = Some(parse_positive_real_option("PIVTOL", value, line_num)?);
                }
                (_, "PIVREL") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.pivrel = Some(parse_positive_real_option("PIVREL", value, line_num)?);
                }
                (None | Some("DEVICE"), "TEMP") => {
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
                (None | Some("DEVICE"), "TNOM") => {
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
                (_, "SCALE") => {
                    // Element geometry scale factor. ngspice exposes it as the
                    // `scale` shell variable and reads it in device setup; the
                    // geometric LEVEL=3 diode is the first RSpice device to derive
                    // dimensions from it.
                    let value = expect_value(stream, line_num, params)?;
                    options.scale = Some(parse_positive_real_option("SCALE", value, line_num)?);
                }
                (_, "ITL1") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.itl1 = Some(parse_usize_option("ITL1", value, line_num)?);
                }
                (_, "ITL2") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.itl2 = Some(parse_usize_option("ITL2", value, line_num)?);
                }
                (_, "ITL4") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.itl4 = Some(parse_usize_option("ITL4", value, line_num)?);
                }
                (_, "ITL6") => {
                    let value = expect_value(stream, line_num, params)?;
                    options.itl6 = Some(parse_usize_option("ITL6", value, line_num)?);
                }
                (_, "METHOD") => {
                    options.method = Some(parse_method_option(stream, line_num, params)?);
                }
                // The continuation ladder's rungs are switched one at a time
                // rather than through `NONLIN CONTINUATION`, which selects a
                // single algorithm to run in place of the ladder.
                (_, "GMINSTEPPING" | "GMIN_STEPPING") => {
                    options.gmin_stepping =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (_, "SOURCESTEPPING" | "SOURCE_STEPPING" | "SRCSTEPPING") => {
                    options.source_stepping =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (_, "PSEUDOTRANSIENT" | "PSEUDO_TRANSIENT") => {
                    options.pseudo_transient =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (_, "ARCLENGTH" | "ARC_LENGTH") => {
                    options.arc_length =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (_, "DAMPING") => {
                    options.damping_strategy =
                        Some(parse_damping_option(stream, line_num, params)?);
                }
                (_, "SOLVER") => {
                    options.matrix_solver = Some(parse_matrix_solver_option(stream, line_num)?);
                }
                (Some("OUTPUT"), "INITIAL_INTERVAL" | "INITIALINTERVAL") => {
                    if !options.output_time_points.is_empty() {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: "Cannot specify both .OPTIONS OUTPUT INITIAL_INTERVAL and OUTPUTTIMEPOINTS".to_string(),
                        });
                    }
                    let value = expect_value(stream, line_num, params)?;
                    let initial_interval =
                        parse_positive_real_option("OUTPUT.INITIAL_INTERVAL", value, line_num)?;
                    let intervals = parse_output_interval_schedule(
                        stream,
                        line_num,
                        params,
                        options,
                        max_analysis_points,
                    )?;
                    options.output_interval_schedule =
                        Some(crate::netlist::XyceOutputIntervalSchedule {
                            initial_interval,
                            intervals,
                        });
                }
                (Some("OUTPUT"), "OUTPUTTIMEPOINTS") => {
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
                        options
                            .output_time_points
                            .len()
                            .saturating_add(options.timeint_breakpoints.len())
                            .saturating_add(output_interval_count(options))
                            .saturating_add(restart_interval_count(options)),
                        max_analysis_points,
                    )?;
                    append_canonical_time_points(
                        &mut options.output_time_points,
                        values,
                        &options.timeint_breakpoints,
                        max_analysis_points,
                    )?;
                }
                (Some("OUTPUT"), "SNAPSHOTS") => {
                    options.output_snapshots =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("OUTPUT"), "PRINTHEADER") => {
                    options.output_print_header =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (Some("OUTPUT"), "PRINTFOOTER") => {
                    options.output_print_footer =
                        Some(parse_boolean_option(stream, line_num, params, has_equals)?);
                }
                (_, "INTERP" | "NOACCT") => {
                    // Ngspice compatibility flags. INTERP affects rawfile storage
                    // density and NOACCT suppresses accounting output; neither
                    // changes the solved circuit state in RSpice today.
                    if has_equals {
                        let _ = expect_value(stream, line_num, params)?;
                    }
                }
                (_, "ALLOW_SIMPLIFIED_MOS" | "ALLOWSIMPLIFIEDMOS") => {
                    // Bare flag enables; an explicit value of 0 disables.
                    let enabled = if has_equals {
                        expect_value(stream, line_num, params)? != 0.0
                    } else {
                        true
                    };
                    options.allow_simplified_mos = Some(enabled);
                }
                _ => {
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
            recovery::consume_failed_field(
                stream,
                value_start,
                option_package.as_deref(),
                &key_upper,
                params,
            );
        }
    }

    Ok(())
}
