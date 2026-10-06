//! Stage analysis cards and their effects before changing parser-owned state.
use super::*;

#[derive(Clone, Copy)]
pub(super) struct AnalysisCardContext<'a> {
    pub(super) line_num: usize,
    pub(super) logical_line: &'a str,
    pub(super) params: &'a ParamContext,
    pub(super) max_analysis_points: usize,
    pub(super) origin: &'a NetlistSourceLocation,
    pub(super) lin_exists: bool,
    pub(super) current_noise: Option<TransientNoiseConfig>,
}

pub(super) struct AnalysisCardSink<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
    pub(super) monte_carlo_source_cards: &'a mut Vec<monte_carlo_identity::SourceCard>,
    pub(super) lin_analysis: &'a mut Option<LinAnalysis>,
    pub(super) fft_analyses: &'a mut Vec<FftAnalysis>,
    pub(super) output_requests: &'a mut Vec<OutputRequest>,
    pub(super) diagnostics: &'a mut Vec<ParseDiagnostic>,
    pub(super) options: &'a mut SimulationOptions,
}

pub(super) struct ParsedAnalysisCard {
    analysis: Option<AnalysisCommand>,
    lin_analysis: Option<LinAnalysis>,
    fft_analysis: Option<FftAnalysis>,
    monte_carlo_source_card: Option<monte_carlo_identity::SourceCard>,
    output_request: Option<OutputRequest>,
    transient_noise: Option<TransientNoiseConfig>,
    diagnostics: Vec<ParseDiagnostic>,
}

#[derive(Clone, Copy)]
enum AnalysisHead {
    Op,
    Dc,
    Ac,
    Lin,
    Qpnoise,
    Qpxf,
    Qpac,
    Qpss,
    Hb,
    Pss,
    Pac,
    Pxf,
    Pnoise,
    Pstb,
    Envelope,
    Sp,
    Stb,
    Disto,
    Tran,
    Step,
    MonteCarlo,
    Temp,
    Four,
    Fft,
    Noise,
    Sensitivity,
    PoleZero,
    Transfer,
    DcMatch,
}

impl AnalysisHead {
    fn parse(command: &str) -> Option<Self> {
        Some(match command {
            ".OP" => Self::Op,
            ".DC" => Self::Dc,
            ".AC" => Self::Ac,
            ".LIN" => Self::Lin,
            ".QPNOISE" => Self::Qpnoise,
            ".QPXF" => Self::Qpxf,
            ".QPAC" => Self::Qpac,
            ".QPSS" => Self::Qpss,
            ".HB" => Self::Hb,
            ".PSS" => Self::Pss,
            ".PAC" => Self::Pac,
            ".PXF" => Self::Pxf,
            ".PNOISE" => Self::Pnoise,
            ".PSTB" => Self::Pstb,
            ".ENVELOPE" => Self::Envelope,
            ".SP" => Self::Sp,
            ".STB" => Self::Stb,
            ".DISTO" => Self::Disto,
            ".TRAN" | ".TR" => Self::Tran,
            ".STEP" => Self::Step,
            ".MC" => Self::MonteCarlo,
            ".TEMP" => Self::Temp,
            ".FOUR" | ".FOURIER" => Self::Four,
            ".FFT" => Self::Fft,
            ".NOISE" => Self::Noise,
            ".SENS" => Self::Sensitivity,
            ".PZ" => Self::PoleZero,
            ".TF" => Self::Transfer,
            ".DCMATCH" => Self::DcMatch,
            _ => return None,
        })
    }
}

impl ParsedAnalysisCard {
    pub(super) fn parse(
        command: &str,
        stream: &mut TokenStream,
        context: AnalysisCardContext<'_>,
    ) -> Result<Option<Self>, ParseError> {
        let Some(head) = AnalysisHead::parse(command) else {
            return Ok(None);
        };
        // The full card, including trailing fields and cross-card constraints,
        // must succeed before any live statistical draws or effects are kept.
        if card_values_may_sample(stream) {
            let isolated = context.params.isolated_random_clone();
            Self::stage(
                head,
                command,
                &mut stream.clone(),
                AnalysisCardContext {
                    params: &isolated,
                    ..context
                },
            )?;
        }
        Self::stage(head, command, stream, context).map(Some)
    }

    fn stage(
        head: AnalysisHead,
        command: &str,
        stream: &mut TokenStream,
        context: AnalysisCardContext<'_>,
    ) -> Result<Self, ParseError> {
        let AnalysisCardContext {
            line_num,
            logical_line,
            params,
            max_analysis_points,
            origin,
            lin_exists,
            current_noise,
        } = context;
        let mut analysis = None;
        let mut lin_analysis = None;
        let mut fft_analysis = None;
        let mut monte_carlo_source_card = None;
        let mut output_request = None;
        let mut transient_noise = None;
        let mut collected_diagnostics = Vec::new();
        let diagnostics = &mut collected_diagnostics;
        match head {
            AnalysisHead::Op => {
                analysis = Some(AnalysisCommand::Op);
            }
            AnalysisHead::Dc => {
                let (source, spec) = parse_dc_sweep_spec(stream, line_num, params)?;

                push_xyce_inconsistent_dc_sweep_warning(
                    params,
                    diagnostics,
                    origin,
                    &source,
                    &spec,
                );

                // Optional second (outer) source: .DC V1 a b s V2 a2 b2 s2
                skip_commas(stream);
                let sweep2 = if matches!(stream.peek().kind, TokenKind::Ident(_)) {
                    let (source2, spec2) = parse_dc_sweep_spec(stream, line_num, params)?;
                    push_xyce_inconsistent_dc_sweep_warning(
                        params,
                        diagnostics,
                        origin,
                        &source2,
                        &spec2,
                    );
                    Some(crate::netlist::DcSecondSweep {
                        source: source2,
                        start: spec2.start,
                        stop: spec2.stop,
                        step: spec2.step,
                        mode: spec2.mode,
                    })
                } else {
                    None
                };

                analysis = Some(AnalysisCommand::Dc {
                    source,
                    start: spec.start,
                    stop: spec.stop,
                    step: spec.step,
                    mode: spec.mode,
                    sweep2,
                });
            }
            AnalysisHead::Ac => {
                let var_str = expect_ident(stream, line_num)?;
                if var_str.eq_ignore_ascii_case("DATA") {
                    if !stream.consume(&TokenKind::Equals) {
                        return Err(ParseError::Syntax {
                            line: line_num,
                            message: ".AC DATA requires DATA=<table-name>".to_string(),
                        });
                    }
                    let table_name = expect_ident(stream, line_num)?;
                    analysis = Some(AnalysisCommand::AcData { table_name });
                } else {
                    let variation = match var_str.as_str() {
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

                    analysis = Some(AnalysisCommand::Ac {
                        variation,
                        points,
                        start_freq,
                        stop_freq,
                    });
                }
            }
            AnalysisHead::Lin => {
                lin_analysis = Some(parse_lin_command(stream, line_num, params)?);
            }
            AnalysisHead::Qpnoise => {
                analysis = Some(qpnoise_card::parse(stream, line_num, params)?);
            }
            AnalysisHead::Qpxf => {
                analysis = Some(qpxf_card::parse(stream, line_num, params)?);
            }
            AnalysisHead::Qpac => {
                analysis = Some(qpac_card::parse(stream, line_num, params)?);
            }
            AnalysisHead::Qpss => {
                analysis = Some(qpss_card::parse(stream, line_num, params)?);
            }
            AnalysisHead::Hb => {
                analysis = Some(hb_card::parse_hb_command(stream, line_num, params)?);
            }
            AnalysisHead::Pss => {
                analysis = Some(periodic_cards::parse_pss_command(stream, line_num, params)?);
            }
            AnalysisHead::Pac => {
                analysis = Some(periodic_cards::parse_pac_command(stream, line_num, params)?);
            }
            AnalysisHead::Pxf => {
                analysis = Some(periodic_cards::parse_pxf_command(stream, line_num, params)?);
            }
            AnalysisHead::Pnoise => {
                analysis = Some(periodic_cards::parse_pnoise_command(
                    stream, line_num, params,
                )?);
            }
            AnalysisHead::Pstb => {
                analysis = Some(periodic_cards::parse_pstb_command(
                    stream, line_num, params,
                )?);
            }
            AnalysisHead::Envelope => {
                analysis = Some(periodic_cards::parse_envelope_command(
                    stream, line_num, params,
                )?);
            }
            AnalysisHead::Sp => {
                let sp = parse_sp_command(stream, line_num, params)?;
                analysis = Some(sp);
            }
            AnalysisHead::Stb => {
                analysis = Some(parse_stb_command(stream, line_num, params)?);
            }
            AnalysisHead::Disto => {
                let disto = parse_disto_command(stream, line_num, params)?;
                analysis = Some(disto);
            }
            AnalysisHead::Tran => {
                let step = expect_value(stream, line_num, params)?;
                if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
                    return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".TRAN line has an unexpected number of fields\nUnrecognized dot line will be ignored"
                        .to_string(),
                });
                }
                let stop = expect_value(stream, line_num, params)?;
                // A `KEY=VALUE` tail is never a positional field, so the optional
                // positionals stop at the first keyword pair rather than letting
                // a keyword whose name also parses as a value be eaten as tstart.
                let start = try_positional_transient_value(stream, params);
                let max_step = try_positional_transient_value(stream, params);
                let mut uic = consume_uic_keyword(stream);
                transient_noise =
                    parse_transient_noise_keywords(stream, line_num, params, &mut uic)?;

                analysis = Some(AnalysisCommand::Tran {
                    step,
                    stop,
                    start,
                    max_step,
                    uic,
                });
            }
            AnalysisHead::Step => {
                let step_cmd = parse_step_command(stream, line_num, params)?;
                analysis = Some(AnalysisCommand::Step(step_cmd));
            }
            AnalysisHead::MonteCarlo => {
                let mut report_spans = Vec::new();
                let mc_cmd = parse_mc_command(
                    stream,
                    line_num,
                    params,
                    max_analysis_points,
                    &mut report_spans,
                )?;
                monte_carlo_source_card = Some(monte_carlo_identity::SourceCard::new(
                    origin,
                    logical_line,
                    &report_spans,
                ));
                analysis = Some(AnalysisCommand::MonteCarlo(mc_cmd));
            }
            AnalysisHead::Temp => {
                let temperatures = parse_temp_command(stream, line_num, params)?;
                analysis = Some(AnalysisCommand::Temp { temperatures });
            }
            AnalysisHead::Four => {
                let authored_source = remaining_command_source(stream);
                let card = parse_four_command(stream, line_num, params)?;
                output_request = Some(OutputRequest::from_four(
                    card.outputs.as_slice(),
                    origin.clone(),
                    &authored_source,
                ));
                analysis = Some(AnalysisCommand::Four {
                    fundamental: card.fundamental,
                    outputs: card.outputs,
                    num_harmonics: card.num_harmonics,
                    periods: card.periods,
                    window_from: card.window_from,
                    window_to: card.window_to,
                });
            }
            AnalysisHead::Fft => {
                let analysis = parse_fft_command(stream, line_num, params, diagnostics)?;
                output_request = Some(OutputRequest::from_fft(&analysis, origin.clone()));
                fft_analysis = Some(analysis);
            }
            AnalysisHead::Noise => {
                let noise = parse_noise_command(stream, line_num, params)?;
                analysis = Some(noise);
            }
            AnalysisHead::Sensitivity => {
                let sens = parse_sens_command(stream, line_num, params)?;
                analysis = Some(sens);
            }
            AnalysisHead::PoleZero => {
                let pz = parse_pz_command(stream, line_num)?;
                analysis = Some(pz);
            }
            AnalysisHead::Transfer => {
                analysis = Some(parse_tf_command(stream, line_num)?);
            }
            AnalysisHead::DcMatch => {
                analysis = Some(dcmatch_card::parse_dcmatch_command(
                    stream, line_num, params,
                )?);
            }
        }
        reject_unconsumed_command_tokens(stream, line_num, command)?;
        if lin_exists && lin_analysis.is_some() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".LIN may appear only once in a netlist".into(),
            });
        }
        if let (Some(existing), Some(selected)) = (current_noise, transient_noise)
            && existing != selected
        {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "this deck's .TRAN cards request different transient-noise settings; one deck plays one noise realization".into(),
            });
        }
        Ok(Self {
            analysis,
            lin_analysis,
            fft_analysis,
            monte_carlo_source_card,
            output_request,
            transient_noise,
            diagnostics: collected_diagnostics,
        })
    }

    pub(super) fn publish(self, sink: AnalysisCardSink<'_>) {
        if let Some(analysis) = self.analysis {
            sink.analyses.push(analysis);
        }
        if let Some(lin) = self.lin_analysis {
            *sink.lin_analysis = Some(lin);
        }
        if let Some(fft) = self.fft_analysis {
            sink.fft_analyses.push(fft);
        }
        if let Some(card) = self.monte_carlo_source_card {
            sink.monte_carlo_source_cards.push(card);
        }
        if let Some(request) = self.output_request {
            sink.output_requests.push(request);
        }
        if let Some(noise) = self.transient_noise {
            sink.options.transient_noise = Some(noise);
        }
        sink.diagnostics.extend(self.diagnostics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(state: &mut ParseState, line: &str) -> Result<(), ParseError> {
        process_line(line, 7, &NetlistSourceLocation::in_memory(7), state)
    }

    fn reference() -> ParamContext {
        let mut params = ParamContext::new();
        params.set_random_seed(37);
        params
    }

    #[test]
    fn late_card_failures_publish_no_analysis_or_auxiliary_effects() {
        for card in [
            ".OP extra",
            ".AC DATA=grid extra",
            ".DC V1 1 0 1 V2 0 {missing} 1",
            ".TRAN 1n 1u NOISEFMAX=1meg extra",
            ".FFT v(out) START=-1 NP=32 extra",
            ".MC 2 uniform .1 seed 37 extra",
        ] {
            let mut state = ParseState::new();
            state.params.set_expression_dialect(ExpressionDialect::Xyce);
            assert!(parse(&mut state, card).is_err(), "{card}");
            assert!(state.analyses.is_empty(), "{card}");
            assert!(state.diagnostics.is_empty(), "{card}");
            assert!(state.fft_analyses.is_empty(), "{card}");
            assert!(state.monte_carlo_source_cards.is_empty(), "{card}");
            assert!(state.output_requests.is_empty(), "{card}");
            assert!(state.lin_analysis.is_none(), "{card}");
            assert!(state.options.transient_noise.is_none(), "{card}");
        }
    }

    #[test]
    fn a_failed_sampled_card_retries_without_advancing_the_live_stream() {
        for expression in ["aunif(0,1)", "limit(0,1)", "sample(0)"] {
            let mut state = ParseState::new();
            state.params = reference();
            state
                .params
                .define_function("sample", vec!["X".into()], "x+aunif(0,1)");
            let expected = reference();
            let source = format!(".DC V1 {{{expression}}} {{stop}} 1");
            assert!(parse(&mut state, &source).is_err());
            assert!(state.analyses.is_empty());
            state.params.set("stop", 2.0);
            parse(&mut state, &source).unwrap();
            let sampled = eval_expression(
                if expression == "sample(0)" {
                    "aunif(0,1)"
                } else {
                    expression
                },
                &expected,
            )
            .unwrap();
            let AnalysisCommand::Dc { start, .. } = state.analyses[0] else {
                panic!("DC card")
            };
            assert_eq!(start, sampled);
            assert_eq!(
                eval_expression("aunif(0,1)", &state.params).unwrap(),
                eval_expression("aunif(0,1)", &expected).unwrap()
            );
        }
    }

    #[test]
    fn rejected_cross_card_noise_settings_keep_prior_state_and_random_position() {
        let mut state = ParseState::new();
        state.params = reference();
        parse(&mut state, ".TRAN 1n 1u NOISEFMAX=1meg").unwrap();
        let original = state.options.transient_noise;
        assert!(parse(&mut state, ".TR 1n 1u NOISEFMAX={aunif(2e6,1)}").is_err());
        assert_eq!(state.analyses.len(), 1);
        assert_eq!(state.options.transient_noise, original);
        assert_eq!(
            eval_expression("aunif(0,1)", &state.params).unwrap(),
            eval_expression("aunif(0,1)", &reference()).unwrap()
        );
    }

    #[test]
    fn duplicate_lin_failure_keeps_the_first_card() {
        let mut state = ParseState::new();
        parse(&mut state, ".LIN SPARCALC=0").unwrap();
        let error = parse(&mut state, ".LIN SPARCALC=0").unwrap_err();
        assert!(error.to_string().contains("only once"), "{error}");
        assert!(matches!(state.lin_analysis, Some(LinAnalysis::AcOnly)));
        assert!(state.analyses.is_empty());
    }

    #[test]
    fn completed_cards_keep_diagnostic_and_output_owner_order() {
        let mut state = ParseState::new();
        state.params.set_expression_dialect(ExpressionDialect::Xyce);
        parse(&mut state, ".DC V1 1 0 1 V2 2 0 1").unwrap();
        parse(&mut state, ".FOURIER 1k V(out)").unwrap();
        parse(&mut state, ".FFT V(out) START=-1 NP=32").unwrap();
        parse(&mut state, ".MC 2 uniform .1 seed 37").unwrap();
        assert_eq!(state.analyses.len(), 3);
        assert_eq!(state.monte_carlo_source_cards.len(), 1);
        assert_eq!(state.fft_analyses.len(), 1);
        assert_eq!(state.output_requests.len(), 2);
        assert_eq!(state.diagnostics.len(), 3);
        assert!(state.diagnostics[0].message.contains("V1"));
        assert!(state.diagnostics[1].message.contains("V2"));
        assert!(state.diagnostics[2].message.contains("START"));
    }
}
