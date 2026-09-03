//! The one place an authored card becomes an engine call.
//!
//! Every surface reaches this match: `Engine.run` for a deck, and each
//! convenience method through the one-analysis plan it constructs. Adding a
//! second translation from a card to a runner anywhere else is how `.TRAN` on
//! one surface comes to mean something else on another.

use super::*;

/// Bind one produced result to the identity the canonical plan minted for the
/// card that produced it, at the coordinate it was solved at.
///
/// A deck may author several cards of one family, once per coordinate of a run
/// axis, and each result's shared document has to name its own card and place.
/// Without this every `.AC` result in a deck would publish under `ac-001` at no
/// coordinate.
fn identified<T: crate::results::CarriesDocumentEvidence>(
    result: T,
    context: Option<&ExecutionContext>,
) -> T {
    crate::results::bind_document_identity(
        result,
        context.and_then(|context| {
            context
                .analysis
                .map(|analysis| (analysis, context.result_coordinate.as_ref()))
        }),
    )
}

/// Run one directive, recording what it produced.
pub(super) fn execute(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    analysis: &AnalysisCommand,
    context: Option<&ExecutionContext>,
    out: &mut DirectiveOutcomes,
) -> PyResult<()> {
    let net = &netlist.inner;
    let max_analysis_points = || {
        py_engine
            .engine_for_netlist(net)
            .config()
            .resource_limits
            .max_analysis_points
    };

    match analysis {
        AnalysisCommand::Op => {
            let result = py_engine.dc_op_impl(py, netlist)?;
            let handle = Py::new(py, identified(result, context))?;
            out.op.push_with(handle, |handle| handle.clone_ref(py));
            out.records
                .push(PyAnalysisRecord::executed("op", ".op".to_string()));
        }
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2,
        } => {
            let engine = py_engine.engine_for_netlist(&netlist.inner);
            let primary = DcSweepSpec {
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            };
            let results = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_dc_sweep2_spec_with_report_and_abort(
                    &netlist.inner,
                    source,
                    &primary,
                    sweep2.as_ref(),
                    abort,
                )
            })?;
            let result = match sweep2 {
                Some(outer) => PyDcSweepResult::new_nested_with_reports(
                    results,
                    source,
                    &outer.source,
                    outer.spec().points(),
                )?,
                None => PyDcSweepResult::new_named_with_reports(results, source),
            };
            let handle = Py::new(py, identified(result, context))?;
            out.dc.push_with(handle, |handle| handle.clone_ref(py));
            let description = describe_analysis(analysis);
            out.records
                .push(PyAnalysisRecord::executed("dc", description));
        }
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } => {
            let tstart = start.unwrap_or(0.0);
            let resolved = rspice_core::execution::resolve_transient_maximum_step(
                *step, *stop, *start, *max_step,
            )
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
            let startup_mode =
                match context.map_or(TransientStartup::Card, |context| context.transient_startup) {
                    TransientStartup::Card => {
                        Some(rspice_core::engine::TransientStartupMode::from_uic(*uic))
                    }
                    // `run_tran` authors no card, so the deck's own `.TRAN` cards
                    // state the contract. Asking the engine keeps that rule in one
                    // place instead of restating it on this surface.
                    TransientStartup::DeckInferred => None,
                };
            let result = py_engine.tran_impl(py, netlist, *stop, resolved, tstart, startup_mode)?;
            let handle = Py::new(py, identified(result, context))?;
            out.tran.push_with(handle, |handle| handle.clone_ref(py));
            let mut detail = format!(".tran {step} {stop}");
            if tstart > 0.0 {
                detail.push_str(&format!(" (tstart={tstart})"));
            }
            out.records.push(PyAnalysisRecord::executed("tran", detail));
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                max_analysis_points(),
            )?;
            let result = py_engine.ac_impl(py, netlist, frequencies)?;
            let handle = Py::new(py, identified(result, context))?;
            out.ac.push_with(handle, |handle| handle.clone_ref(py));
            out.records.push(PyAnalysisRecord::executed(
                "ac",
                format!(
                    ".ac {} {points} {start_freq} {stop_freq}",
                    format!("{variation:?}").to_lowercase()
                ),
            ));
        }
        AnalysisCommand::AcData { table_name } => {
            let frequencies = ac_data_frequencies(net, table_name)?;
            let result = py_engine.ac_impl(py, netlist, frequencies)?;
            let handle = Py::new(py, identified(result, context))?;
            out.ac.push_with(handle, |handle| handle.clone_ref(py));
            out.records.push(PyAnalysisRecord::executed(
                "ac_data",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Hb { frequencies } => {
            // The default harmonic order, the multi-tone common basis, and
            // Xyce's explicit single-tone NUMFREQ collocation contract all
            // belong to `rspice-core`.
            let config = rspice_core::analysis::HbConfig::from_hb_card(
                frequencies,
                &net.options.hb_num_frequencies,
            )
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
            let engine = py_engine.engine_for_netlist(net);
            let result = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_hb_with_abort(net, config, abort)
            })?;
            out.hb
                .push(identified(PyHbResult::from_core(&result), context));
            // Retained under this instance's canonical identity so a bound
            // `.PAC`, `.PNOISE`, or `.ENVELOPE` consumes this exact carrier.
            if let Some(id) = context.and_then(|context| context.analysis_id.clone()) {
                out.periodic_operating_points.insert(
                    id,
                    PeriodicOperatingPoint::HarmonicBalance(Box::new(result.operating_point)),
                );
            }
            out.records.push(PyAnalysisRecord::executed(
                "hb",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => {
            let frequencies = sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                max_analysis_points(),
            )?;
            let result = py_engine.distortion_impl(py, netlist, frequencies, *f2_over_f1)?;
            let handle = Py::new(py, identified(result, context))?;
            out.distortion
                .push_with(handle, |handle| handle.clone_ref(py));
            out.records.push(PyAnalysisRecord::executed(
                "disto",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Sp {
            variation,
            points,
            start_freq,
            stop_freq,
            do_noise,
        } => {
            let frequencies = sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                max_analysis_points(),
            )?;
            out.s_parameters.push(identified(
                py_engine.sparameter_impl(py, netlist, frequencies, *do_noise)?,
                context,
            ));
            out.records.push(PyAnalysisRecord::executed(
                "sp",
                describe_analysis(analysis),
            ));
            if *do_noise {
                out.records.push(PyAnalysisRecord::executed(
                    "sp_noise",
                    describe_analysis(analysis),
                ));
            }
        }
        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let engine = py_engine.engine_for_netlist(net);
            let output = py_engine.resolve_node(
                py,
                &engine,
                net,
                &NodeIdentifier::Name(output_node.clone()),
                "noise output",
            )?;
            let output_neg = match reference_node {
                Some(reference) => Some(py_engine.resolve_node(
                    py,
                    &engine,
                    net,
                    &NodeIdentifier::Name(reference.clone()),
                    "noise reference",
                )?),
                None => None,
            };
            let frequencies = sweep_frequencies(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                max_analysis_points(),
            )?;
            let source = if input_source.is_empty() {
                None
            } else {
                Some(input_source.as_str())
            };
            let results = py_engine.noise_core_impl(
                py,
                netlist,
                output,
                output_neg,
                source,
                &frequencies,
                None,
            )?;
            let converted: Vec<PyNoiseResult> = PyNoiseResult::sweep_from_core(&results)
                .into_iter()
                .map(|point| identified(point, context))
                .collect();
            out.noise.push(converted);
            out.noise_core = Some(results);
            out.records.push(PyAnalysisRecord::executed(
                "noise",
                format!(".noise V({output_node}) {input_source}"),
            ));
        }
        AnalysisCommand::NoiseData {
            output_node,
            reference_node,
            input_source,
            table_name,
        } => {
            let engine = py_engine.engine_for_netlist(net);
            let (_, results) = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_noise_data_named_with_input_source_and_abort(
                    net,
                    output_node,
                    reference_node.as_deref(),
                    input_source,
                    table_name,
                    engine.config().temperature,
                    abort,
                )
            })?;
            let converted: Vec<PyNoiseResult> = PyNoiseResult::sweep_from_core(&results)
                .into_iter()
                .map(|point| identified(point, context))
                .collect();
            out.noise.push(converted);
            out.noise_core = Some(results);
            out.records.push(PyAnalysisRecord::executed(
                "noise_data",
                format!(".noise V({output_node}) {input_source} DATA={table_name}"),
            ));
        }
        AnalysisCommand::Tf {
            output_node,
            reference_node,
            output_is_current,
            input_source,
        } => {
            let result = py_engine.tf_impl(
                py,
                netlist,
                output_node,
                reference_node.as_deref(),
                *output_is_current,
                input_source,
            )?;
            out.tf.push(identified(result, context));
            out.records.push(PyAnalysisRecord::executed(
                "tf",
                format!(".tf {output_node} {input_source}"),
            ));
        }
        AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            probe,
        } => {
            let result = py_engine.stb_impl(
                py,
                netlist,
                probe,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
            )?;
            out.stb.push(identified(result, context));
            out.records.push(PyAnalysisRecord::executed(
                "stb",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::PoleZero {
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            transfer_type,
            analysis_type,
        } => {
            let (compute_poles, compute_zeros) = match analysis_type {
                PoleZeroAnalysisType::PoleZero => (true, true),
                PoleZeroAnalysisType::PolesOnly => (true, false),
                PoleZeroAnalysisType::ZerosOnly => (false, true),
            };
            let result = py_engine.pz_impl(
                py,
                netlist,
                &NodeIdentifier::Name(input_pos.clone()),
                Some(&NodeIdentifier::Name(input_neg.clone())),
                &NodeIdentifier::Name(output_pos.clone()),
                Some(&NodeIdentifier::Name(output_neg.clone())),
                matches!(transfer_type, PoleZeroTransferType::Current),
                compute_poles,
                compute_zeros,
            )?;
            out.pz.push(identified(result, context));
            out.records.push(PyAnalysisRecord::executed(
                "pz",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::MonteCarlo(command) => {
            let result = py_engine.monte_carlo_impl(
                py,
                netlist,
                command,
                context.and_then(|context| context.coordinate_id),
            )?;
            out.monte_carlo.push(identified(result, context));
            out.records.push(PyAnalysisRecord::executed(
                "mc",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. } => {
            return Err(crate::errors::SimulationError::new_err(
                "run-axis directives must be executed through the canonical deck materializer",
            ));
        }
        AnalysisCommand::Pss(card) => {
            let config = rspice_core::analysis::PssConfig::from(card.as_ref());
            let harmonics = config.num_harmonics;
            let engine = py_engine.engine_for_netlist(net);
            // The operating point carries the analysis result, so solving it
            // once yields both the published result and the exact orbit a
            // dependent `.PAC`/`.PNOISE` linearizes around.
            let operating_point = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_pss_operating_point_with_abort(net, config, abort)
            })?;
            out.pss.push(identified(
                PyPssResult::from_core(operating_point.analysis(), harmonics),
                context,
            ));
            if let Some(id) = context.and_then(|context| context.analysis_id.clone()) {
                out.periodic_operating_points.insert(
                    id,
                    PeriodicOperatingPoint::Shooting(Box::new(operating_point)),
                );
            }
            out.records.push(PyAnalysisRecord::executed(
                "pss",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Pac(card) => {
            let config = rspice_core::analysis::PacConfig::from(card.as_ref());
            config.validate().map_err(|message| {
                crate::errors::value_error(format!("invalid .PAC card: {message}"))
            })?;
            let engine = py_engine.engine_for_netlist(net);
            let result = match upstream_operating_point(out, context, ".PAC")? {
                PeriodicOperatingPoint::Shooting(point) => {
                    run_interruptible(py, &py_engine.active_runs, |abort| {
                        engine.run_pac_from_pss_with_abort(net, config, point, abort)
                    })?
                }
                PeriodicOperatingPoint::HarmonicBalance(point) => {
                    run_interruptible(py, &py_engine.active_runs, |abort| {
                        engine.run_pac_from_hb_with_abort(net, config, point, abort)
                    })?
                }
            };
            out.pac
                .push(identified(PyPacResult::from_core(&result), context));
            out.records.push(PyAnalysisRecord::executed(
                "pac",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Pnoise(card) => {
            // The offset grid is preflighted against this binding's own
            // analysis-point limit before the card runs; core reads the same
            // sweep off the card when it executes it, so the grid is not
            // decided twice.
            let offsets = sweep_frequencies(
                card.sweep.variation,
                card.sweep.points,
                card.sweep.start_freq,
                card.sweep.stop_freq,
                max_analysis_points(),
            )?;
            if offsets.iter().any(|offset| *offset <= 0.0) {
                return Err(crate::errors::value_error(
                    ".PNOISE offset frequencies must be strictly positive",
                ));
            }
            let engine = py_engine.engine_for_netlist(net);
            // Core's card runner decides what an authored `.PNOISE` means
            // around each carrier: an autonomous `.PSS` selects the
            // oscillator driver, whose result is a carrier-normalized
            // phase-noise spectrum plus the Demir diffusion evidence, and a
            // driven carrier selects the conversion-matrix analysis. Both
            // come back in the one type the shared document accepts.
            let result = match upstream_operating_point(out, context, ".PNOISE")? {
                PeriodicOperatingPoint::Shooting(point) => {
                    run_interruptible(py, &py_engine.active_runs, |abort| {
                        engine.run_pnoise_card_from_pss_with_abort(net, card, point, abort)
                    })?
                }
                PeriodicOperatingPoint::HarmonicBalance(point) => {
                    run_interruptible(py, &py_engine.active_runs, |abort| {
                        engine.run_pnoise_card_from_hb_with_abort(net, card, point, abort)
                    })?
                }
            };
            let kind = match result {
                rspice_core::engine::PeriodicNoiseResult::Driven { output, result } => {
                    out.pnoise.push(identified(
                        PyPeriodicNoiseResult::from_run(&output, &result),
                        context,
                    ));
                    "pnoise"
                }
                rspice_core::engine::PeriodicNoiseResult::Oscillator { output, result } => {
                    out.oscillator_noise.push(identified(
                        PyOscillatorNoiseResult::from_run(&output, &result),
                        context,
                    ));
                    "pnoise_oscillator"
                }
                rspice_core::engine::PeriodicNoiseResult::Spectral(_) => {
                    // The card runners return only the driven and autonomous
                    // shapes; a spectral assembly reaching here would mean the
                    // executed card is not the one that was authored.
                    return Err(crate::errors::SimulationError::new_err(
                        "the .PNOISE card runner returned a directly assembled spectral result, \
                         which no authored card produces",
                    ));
                }
            };
            out.records.push(PyAnalysisRecord::executed(
                kind,
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Envelope(card) => {
            // The envelope continues the carrier the bound `.HB` defined, so
            // its spectral configuration comes from that instance rather than
            // being re-derived from the deck.
            let config = match upstream_operating_point(out, context, ".ENVELOPE")? {
                PeriodicOperatingPoint::HarmonicBalance(point) => point.config().clone(),
                PeriodicOperatingPoint::Shooting(_) => {
                    return Err(crate::errors::value_error(
                        ".ENVELOPE continues a harmonic-balance carrier, but the analysis it is \
                         bound to is a shooting PSS",
                    ));
                }
            };
            let frozen = card.frozen_sources.clone();
            let engine = py_engine.engine_for_netlist(net);
            let result = run_interruptible(py, &py_engine.active_runs, |abort| {
                engine.run_envelope_with_abort(
                    net,
                    config,
                    &frozen,
                    card.duration,
                    card.max_step,
                    abort,
                )
            })?;
            let handle = Py::new(
                py,
                identified(PyEnvelopeResult::from_core(py, &result)?, context),
            )?;
            out.envelope
                .push_with(handle, |handle| handle.clone_ref(py));
            out.records.push(PyAnalysisRecord::executed(
                "envelope",
                describe_analysis(analysis),
            ));
        }
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            filters,
            ac_sweep,
        } => {
            if let Some(sweep) = ac_sweep {
                let frequencies = sweep_frequencies(
                    sweep.variation,
                    sweep.points,
                    sweep.start_freq,
                    sweep.stop_freq,
                    max_analysis_points(),
                )?;
                let output = NodeIdentifier::Name(output_node.clone());
                let reference = reference_node
                    .as_ref()
                    .map(|name| NodeIdentifier::Name(name.clone()));
                out.sensitivity_ac.push(identified(
                    py_engine.sensitivity_ac_complete_impl(
                        py,
                        netlist,
                        &output,
                        reference.as_ref(),
                        *output_is_current,
                        &frequencies,
                        filters,
                    )?,
                    context,
                ));
                out.records.push(PyAnalysisRecord::executed(
                    "sens_ac",
                    describe_analysis(analysis),
                ));
            } else {
                let output = NodeIdentifier::Name(output_node.clone());
                let reference = reference_node
                    .as_ref()
                    .map(|name| NodeIdentifier::Name(name.clone()));
                out.sensitivity.push(identified(
                    py_engine.sensitivity_dc_complete_impl(
                        py,
                        netlist,
                        &output,
                        reference.as_ref(),
                        *output_is_current,
                        filters,
                    )?,
                    context,
                ));
                out.records.push(PyAnalysisRecord::executed(
                    "sens",
                    describe_analysis(analysis),
                ));
            }
        }
        AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } => {
            let context = context.expect(".FOUR directives always receive a stable identity");
            out.pending_fourier.push(PendingFourier {
                fundamental: *fundamental,
                outputs: outputs.clone(),
                num_harmonics: *num_harmonics,
                analysis_id: context.analysis_id.clone().expect("checked above"),
                coordinate: context.coordinate.clone(),
            });
        }
    }
    Ok(())
}

/// The periodic operating point the deck plan bound this card to.
///
/// The binding is the plan's, not "whichever periodic analysis ran last": a
/// deck with two `.PSS` cards and a `.PAC` between them must linearize around
/// the one the planner selected, and a `.PAC` whose upstream failed must fail
/// rather than silently reuse an earlier orbit.
fn upstream_operating_point<'a>(
    out: &'a DirectiveOutcomes,
    context: Option<&ExecutionContext>,
    card: &'static str,
) -> PyResult<&'a PeriodicOperatingPoint> {
    let id = context
        .and_then(|context| context.upstream_analysis_id.clone())
        .ok_or_else(|| {
            crate::errors::SimulationError::new_err(format!(
                "{card} was executed without the upstream periodic analysis its deck plan bound it to"
            ))
        })?;
    out.periodic_operating_points.get(&id).ok_or_else(|| {
        crate::errors::SimulationError::new_err(format!(
            "{card} linearizes around {id}, which produced no periodic operating point in this run"
        ))
    })
}
