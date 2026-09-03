//! Single-deck execution and contract adjudication.

use super::*;
use crate::suites::ngspice::{TestRunner as NgspiceOracleRunner, TestRunnerConfig};
use rspice_core::SimulationError;
use rspice_core::analysis::FrequencyGridError;
use rspice_core::analysis::ac::try_ac_sweep_frequencies_with_abort;
use rspice_core::analysis::s_param;

impl ExecutionRunner {
    /// Run every discovered deck, honouring the extended-cost gate.
    pub fn run_corpus(&self) -> Vec<ExecutionResult> {
        self.discover()
            .into_iter()
            .filter(|key| !(self.config.skip_extended && self.is_extended(key)))
            .map(|key| {
                let result = self.run_deck(&key);
                if self.config.verbose {
                    eprintln!(
                        "  {:<7} {key} ({} ms) — {}",
                        if result.passed { "ok" } else { "FAILED" },
                        result.duration_ms,
                        result.outcome.summary()
                    );
                }
                result
            })
            .collect()
    }

    /// Run one deck and judge it against its contract.
    pub fn run_deck(&self, key: &str) -> ExecutionResult {
        let start = Instant::now();
        let contract = self.contract_for(key);
        let (outcome, analyses, oracle_compared) = self.execute(key, start);
        let failure = adjudicate(contract, &outcome);

        ExecutionResult {
            name: key.to_string(),
            passed: failure.is_none(),
            contract,
            outcome,
            analyses,
            duration_ms: start.elapsed().as_millis(),
            oracle_compared,
            failure,
        }
    }

    /// Load, build, and run a deck, reporting how far it got.
    fn execute(&self, key: &str, start: Instant) -> (ExecutionOutcome, Vec<String>, bool) {
        let path = self.deck_path(key);

        let source = match std::fs::read_to_string(&path) {
            Ok(source) => source,
            Err(err) => {
                return (
                    rejected(format!("unreadable deck: {err}")),
                    Vec::new(),
                    false,
                );
            }
        };

        // Includes resolve relative to the deck, which is why both corpora
        // keep their upstream directory shape: ISCAS85 decks reach up two
        // levels for a shared technology file, and the paranoia decks sit
        // beside the model cards they include.
        let expanded = match Netlist::preprocess_includes(&source, &path) {
            Ok(expanded) => expanded,
            Err(err) => {
                return (
                    rejected(format!("include expansion: {err}")),
                    Vec::new(),
                    false,
                );
            }
        };

        // Parsed with `.control` blocks intact, deliberately. The parser
        // promotes the analysis and output commands out of the script, and
        // these corpora need that: ISCAS85's annotated decks carry no bare
        // `.tran` card at all — their only analysis request is a `tran`
        // inside an `if $?batchmode` guard — so stripping the block first
        // leaves a deck that appears to ask for nothing and would be scored
        // as vacuous coverage rather than run.
        let netlist = match Netlist::parse_with_path(&expanded, &path) {
            Ok(netlist) => netlist,
            Err(err) => {
                return (rejected(format!("parse: {err}")), Vec::new(), false);
            }
        };

        let analyses: Vec<String> = netlist.analyses.iter().map(describe).collect();

        // A deck that requests nothing has not been exercised, and reporting
        // it as a pass would be counting a file we never simulated. The
        // corpora do contain such files — include-only fragments that happen
        // to carry a deck extension — and they belong in the manifest under
        // `library_fragment` rather than in the pass column as coverage.
        if netlist.analyses.is_empty() {
            return (ExecutionOutcome::NoAnalysis, analyses, false);
        }

        let engine = self.engine(None);
        let abort = DeadlineAbort::new(start, self.config.max_time_per_deck_ms);
        let oracle = path.with_extension("oracle.out").is_file().then(|| {
            let config = TestRunnerConfig {
                relative_tolerance: 0.05,
                absolute_tolerance: 1.0e-6,
                max_mismatches: 10,
                ..TestRunnerConfig::default()
            };
            NgspiceOracleRunner::new_checked_in_oracle(&self.root, config)
        });
        let mut oracle_compared = false;

        // A measures gate is declared in two places on purpose: the manifest
        // marker says the deck is judged that way, the sidecar says on what.
        // Either alone is a mistake — an undeclared sidecar would silently
        // replace a pointwise gate, and a marker without gates would judge
        // nothing — so both halves have to agree before anything runs.
        let measures = self.is_measures(key);
        let has_gates_sidecar = path.with_extension("gates.tsv").is_file();
        if measures != has_gates_sidecar {
            let diagnostic = if measures {
                "manifest marks the deck !measures but no <deck>.gates.tsv sidecar declares the gates"
            } else {
                "a <deck>.gates.tsv sidecar is present but the manifest does not mark the deck !measures"
            };
            return (
                ExecutionOutcome::ReferenceMismatch {
                    diagnostic: diagnostic.to_string(),
                },
                analyses,
                false,
            );
        }

        for (analysis, label) in netlist.analyses.iter().zip(&analyses) {
            let (analysis_outcome, compared) = self.run_analysis(
                &engine,
                &netlist,
                &path,
                analysis,
                &abort,
                oracle.as_ref(),
                measures,
            );
            oracle_compared |= compared;
            match analysis_outcome {
                AnalysisOutcome::Completed => {}
                AnalysisOutcome::Refused(diagnostic) => {
                    return (
                        ExecutionOutcome::Refused {
                            analysis: label.clone(),
                            diagnostic,
                        },
                        analyses,
                        oracle_compared,
                    );
                }
                AnalysisOutcome::TimedOut => {
                    return (
                        ExecutionOutcome::TimedOut {
                            analysis: label.clone(),
                        },
                        analyses,
                        oracle_compared,
                    );
                }
                AnalysisOutcome::NotExecuted => {
                    return (
                        ExecutionOutcome::Unsupported {
                            directive: label.clone(),
                        },
                        analyses,
                        oracle_compared,
                    );
                }
                AnalysisOutcome::ReferenceMismatch(diagnostic) => {
                    return (
                        ExecutionOutcome::ReferenceMismatch { diagnostic },
                        analyses,
                        oracle_compared,
                    );
                }
            }
        }

        (ExecutionOutcome::Completed, analyses, oracle_compared)
    }

    /// Execute one analysis directive.
    ///
    /// The set covered here is the set these corpora actually request in
    /// bulk. Anything else reports [`AnalysisOutcome::NotExecuted`] and
    /// surfaces as a named unsupported directive, which is the honest
    /// outcome: silently skipping a directive would let a deck count as
    /// passing coverage it never received.
    #[allow(clippy::too_many_arguments)]
    fn run_analysis(
        &self,
        engine: &Engine,
        netlist: &Netlist,
        deck_path: &Path,
        analysis: &AnalysisCommand,
        abort: &DeadlineAbort,
        oracle: Option<&NgspiceOracleRunner>,
        measures: bool,
    ) -> (AnalysisOutcome, bool) {
        match analysis {
            AnalysisCommand::Op => match engine.run_dc_op_with_abort(netlist, abort) {
                Ok(result)
                    if finite(result.node_voltages.iter().chain(&result.branch_currents)) =>
                {
                    oracle.map_or((AnalysisOutcome::Completed, false), |oracle| {
                        compare_oracle(oracle.compare_dc_op_reference(deck_path, &result))
                    })
                }
                Ok(_) => (
                    AnalysisOutcome::Refused("non-finite values in results".to_string()),
                    false,
                ),
                Err(error) => (classify(Err(error)), false),
            },
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                sweep2,
                ..
            } => {
                let swept = match sweep2 {
                    None => {
                        engine.run_dc_sweep_with_abort(netlist, source, *start, *stop, *step, abort)
                    }
                    Some(outer) => engine.run_dc_sweep2_with_abort(
                        netlist,
                        source,
                        *start,
                        *stop,
                        *step,
                        Some(outer),
                        abort,
                    ),
                };
                match swept {
                    Ok(points)
                        if points.iter().all(|(sweep, result)| {
                            sweep.is_finite()
                                && finite(
                                    result.node_voltages.iter().chain(&result.branch_currents),
                                )
                        }) =>
                    {
                        oracle.map_or((AnalysisOutcome::Completed, false), |oracle| {
                            compare_oracle(
                                oracle.compare_dc_sweep_reference(deck_path, netlist, &points),
                            )
                        })
                    }
                    Ok(_) => (
                        AnalysisOutcome::Refused("non-finite values in results".to_string()),
                        false,
                    ),
                    Err(error) => (classify(Err(error)), false),
                }
            }
            AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            } => {
                let frequencies = match checked_frequency_sweep(
                    *variation,
                    *points,
                    *start_freq,
                    *stop_freq,
                    abort,
                ) {
                    Ok(frequencies) => frequencies,
                    Err(outcome) => return (outcome, false),
                };
                match engine.run_ac_with_abort(netlist, &frequencies, abort) {
                    Ok(results)
                        if results.iter().all(|point| {
                            point.frequency.is_finite()
                                && point
                                    .voltages
                                    .iter()
                                    .chain(&point.currents)
                                    .all(|value| value.re.is_finite() && value.im.is_finite())
                        }) =>
                    {
                        oracle.map_or((AnalysisOutcome::Completed, false), |oracle| {
                            compare_oracle(
                                oracle.compare_ac_reference(deck_path, netlist, &results),
                            )
                        })
                    }
                    Ok(_) => (
                        AnalysisOutcome::Refused("non-finite values in results".to_string()),
                        false,
                    ),
                    Err(error) => (classify(Err(error)), false),
                }
            }
            AnalysisCommand::Sp {
                variation,
                points,
                start_freq,
                stop_freq,
                ..
            } => {
                let frequencies = match checked_frequency_sweep(
                    *variation,
                    *points,
                    *start_freq,
                    *stop_freq,
                    abort,
                ) {
                    Ok(frequencies) => frequencies,
                    Err(outcome) => return (outcome, false),
                };
                let ports = match s_param::collect_ports(netlist) {
                    Ok(ports) => ports,
                    Err(error) => {
                        return (AnalysisOutcome::Refused(error.to_string()), false);
                    }
                };
                match s_param::extract_s_matrix_with_abort(
                    netlist,
                    &ports,
                    &frequencies,
                    |driven| {
                        engine
                            .run_ac_with_abort(driven, &frequencies, abort)
                            .map_err(|error| error.to_string())
                    },
                    abort,
                ) {
                    Ok(parameters)
                        if parameters
                            .iter()
                            .flatten()
                            .flatten()
                            .all(|value| value.re.is_finite() && value.im.is_finite()) =>
                    {
                        oracle.map_or((AnalysisOutcome::Completed, false), |oracle| {
                            compare_oracle(oracle.compare_s_parameter_reference(
                                deck_path,
                                &frequencies,
                                &parameters,
                            ))
                        })
                    }
                    Ok(_) => (
                        AnalysisOutcome::Refused("non-finite values in results".to_string()),
                        false,
                    ),
                    Err(_) if abort.is_aborted() => (AnalysisOutcome::TimedOut, false),
                    Err(error) => (AnalysisOutcome::Refused(error.to_string()), false),
                }
            }
            AnalysisCommand::Tran {
                step,
                stop,
                max_step,
                uic,
                ..
            } => {
                let ceiling = max_step.unwrap_or_else(|| default_max_step(*step, *stop));
                // Measure gates are grid-stable engineering quantities and
                // are compared free-running, as the ngspice suite runs its
                // `measures` decks; only the pointwise comparison replays the
                // reference grid.
                let locked_time_grid = match oracle
                    .filter(|_| !measures)
                    .map(|oracle| oracle.transient_reference_grid(deck_path))
                    .transpose()
                {
                    Ok(grid) => grid.flatten(),
                    Err(error) => return (AnalysisOutcome::ReferenceMismatch(error), true),
                };
                let locked_engine = locked_time_grid.map(|grid| self.engine(Some(grid)));
                let transient_engine = locked_engine.as_ref().unwrap_or(engine);
                match transient_engine.run_tran_with_startup_mode_and_abort(
                    netlist,
                    *stop,
                    ceiling,
                    rspice_core::engine::TransientStartupMode::from_uic(*uic),
                    abort,
                ) {
                    Ok(result)
                        if finite(result.time.iter())
                            && result
                                .voltages
                                .iter()
                                .chain(&result.branch_currents)
                                .all(|waveform| finite(waveform.iter())) =>
                    {
                        oracle.map_or((AnalysisOutcome::Completed, false), |oracle| {
                            compare_oracle(if measures {
                                oracle.compare_transient_measures(deck_path, &result)
                            } else {
                                oracle.compare_transient_reference(deck_path, netlist, &result)
                            })
                        })
                    }
                    Ok(_) => (
                        AnalysisOutcome::Refused("non-finite values in results".to_string()),
                        false,
                    ),
                    Err(error) => (classify(Err(error)), false),
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
                let frequencies = match checked_frequency_sweep(
                    *variation,
                    *points,
                    *start_freq,
                    *stop_freq,
                    abort,
                ) {
                    Ok(frequencies) => frequencies,
                    Err(outcome) => return (outcome, false),
                };
                match engine.run_noise_named_with_input_source_and_abort(
                    netlist,
                    output_node,
                    reference_node.as_deref(),
                    input_source,
                    &frequencies,
                    engine.config().temperature,
                    abort,
                ) {
                    Ok(results)
                        if results.iter().all(|point| {
                            point.frequency.is_finite() && point.output_noise_density.is_finite()
                        }) =>
                    {
                        oracle.map_or((AnalysisOutcome::Completed, false), |oracle| {
                            compare_oracle(oracle.compare_noise_reference(deck_path, &results))
                        })
                    }
                    Ok(_) => (
                        AnalysisOutcome::Refused("non-finite values in results".to_string()),
                        false,
                    ),
                    Err(error) => (classify(Err(error)), false),
                }
            }
            // Not analyses. `.temp` sets the temperature list the other
            // directives run at, and `.four` post-processes a transient that
            // has already run. Both arrive in `analyses` because that is
            // where the parser files directives, but treating them as
            // unexecutable would report a deck as gap-blocked over a setting
            // the engine already honoured.
            AnalysisCommand::Temp { .. } | AnalysisCommand::Four { .. } => {
                (AnalysisOutcome::Completed, false)
            }
            _ => (AnalysisOutcome::NotExecuted, false),
        }
    }
}

/// Outcome of a single analysis directive.
enum AnalysisOutcome {
    Completed,
    Refused(String),
    TimedOut,
    NotExecuted,
    ReferenceMismatch(String),
}

fn compare_oracle(
    comparison: Result<Vec<crate::suites::ngspice::ValueMismatch>, String>,
) -> (AnalysisOutcome, bool) {
    match comparison {
        Ok(mismatches) if mismatches.is_empty() => (AnalysisOutcome::Completed, true),
        Ok(mismatches) => {
            let diagnostic = mismatches
                .iter()
                .take(3)
                .map(|mismatch| {
                    if mismatch.expected.is_finite() && mismatch.actual.is_finite() {
                        format!(
                            "{} at {:.6e}: expected {:.6e}, got {:.6e}",
                            mismatch.node, mismatch.x_value, mismatch.expected, mismatch.actual
                        )
                    } else {
                        format!("{} at {:.6e}", mismatch.node, mismatch.x_value)
                    }
                })
                .collect::<Vec<_>>()
                .join("; ");
            (AnalysisOutcome::ReferenceMismatch(diagnostic), true)
        }
        Err(error) => (AnalysisOutcome::ReferenceMismatch(error), true),
    }
}

/// Map an engine result onto an outcome.
///
/// `Ok(false)` — a completed run carrying non-finite values — is treated as a
/// refusal rather than a completion. A NaN that propagates into results is
/// the failure mode this corpus exists to catch, and calling it success
/// because no error was returned would make the whole suite vacuous.
fn classify(result: Result<bool, SimulationError>) -> AnalysisOutcome {
    match result {
        Ok(true) => AnalysisOutcome::Completed,
        Ok(false) => AnalysisOutcome::Refused("non-finite values in results".to_string()),
        Err(SimulationError::Aborted) => AnalysisOutcome::TimedOut,
        Err(err) => AnalysisOutcome::Refused(err.to_string()),
    }
}

fn checked_frequency_sweep(
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start: f64,
    stop: f64,
    abort: &DeadlineAbort,
) -> Result<Vec<f64>, AnalysisOutcome> {
    try_ac_sweep_frequencies_with_abort(variation, points, start, stop, abort).map_err(|error| {
        match error {
            FrequencyGridError::Aborted => AnalysisOutcome::TimedOut,
            _ => AnalysisOutcome::Refused(format!("invalid frequency sweep: {error}")),
        }
    })
}

fn finite<'a>(mut values: impl Iterator<Item = &'a f64>) -> bool {
    values.all(|value| value.is_finite())
}

/// ngspice's default ceiling when a deck gives no `tmax`.
fn default_max_step(step: f64, stop: f64) -> f64 {
    let candidate = if step > 0.0 { step } else { stop / 50.0 };
    candidate.min(stop / 50.0).max(f64::MIN_POSITIVE)
}

fn rejected(diagnostic: String) -> ExecutionOutcome {
    ExecutionOutcome::Rejected { diagnostic }
}

/// Judge an outcome against its contract.
fn adjudicate(contract: ExecutionContract, outcome: &ExecutionOutcome) -> Option<ExecutionFailure> {
    match contract {
        ExecutionContract::Executes => match outcome {
            ExecutionOutcome::Completed => None,
            _ => Some(ExecutionFailure::Unmet {
                expected: "every requested analysis completes with finite results",
            }),
        },
        ExecutionContract::ExpectedUnsupported => match outcome {
            // A rejection or a named unsupported directive is the deck
            // failing the way the manifest says it fails.
            ExecutionOutcome::Rejected { .. } | ExecutionOutcome::Unsupported { .. } => None,
            // Completing is the interesting failure: the gap closed.
            ExecutionOutcome::Completed => Some(ExecutionFailure::ContractStale {
                expected: "deck to fail on an unimplemented feature, but it ran to completion",
            }),
            // A refusal or timeout is a *different* failure from the one
            // recorded, so the entry no longer describes what happens.
            _ => Some(ExecutionFailure::ContractStale {
                expected: "deck to fail on an unimplemented feature, not to refuse numerically",
            }),
        },
        ExecutionContract::ExpectedRefusal => match outcome {
            ExecutionOutcome::Refused { .. } => None,
            ExecutionOutcome::Completed => Some(ExecutionFailure::ContractStale {
                expected: "a clean numerical refusal, but the deck completed",
            }),
            _ => Some(ExecutionFailure::Unmet {
                expected: "a clean numerical refusal",
            }),
        },
        ExecutionContract::LibraryFragment => match outcome {
            ExecutionOutcome::NoAnalysis => None,
            _ => Some(ExecutionFailure::ContractStale {
                expected: "an include fragment that parses and requests nothing",
            }),
        },
    }
}

/// A stable label for an analysis directive.
fn describe(analysis: &AnalysisCommand) -> String {
    match analysis {
        AnalysisCommand::Op => ".op".to_string(),
        AnalysisCommand::Dc { sweep2: None, .. } => ".dc".to_string(),
        AnalysisCommand::Dc { .. } => ".dc (2-D)".to_string(),
        AnalysisCommand::Ac { .. } => ".ac".to_string(),
        AnalysisCommand::Sp { .. } => ".sp".to_string(),
        AnalysisCommand::Tran { .. } => ".tran".to_string(),
        other => format!("{other:?}")
            .split_whitespace()
            .next()
            .unwrap_or("unknown")
            .trim_end_matches('{')
            .to_ascii_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_contract_accepts_only_completion() {
        assert!(adjudicate(ExecutionContract::Executes, &ExecutionOutcome::Completed).is_none());
        for outcome in [
            rejected("parse".to_string()),
            ExecutionOutcome::Refused {
                analysis: ".tran".to_string(),
                diagnostic: "no convergence".to_string(),
            },
            ExecutionOutcome::TimedOut {
                analysis: ".tran".to_string(),
            },
        ] {
            assert!(
                matches!(
                    adjudicate(ExecutionContract::Executes, &outcome),
                    Some(ExecutionFailure::Unmet { .. })
                ),
                "{outcome:?}"
            );
        }
    }

    /// The property that keeps the manifest from rotting into a list of
    /// things that used to be broken.
    #[test]
    fn a_deck_that_outperforms_its_contract_fails_as_stale() {
        assert!(matches!(
            adjudicate(
                ExecutionContract::ExpectedUnsupported,
                &ExecutionOutcome::Completed
            ),
            Some(ExecutionFailure::ContractStale { .. })
        ));
        assert!(matches!(
            adjudicate(
                ExecutionContract::ExpectedRefusal,
                &ExecutionOutcome::Completed
            ),
            Some(ExecutionFailure::ContractStale { .. })
        ));
    }

    #[test]
    fn expected_unsupported_accepts_rejection_and_named_directives() {
        for outcome in [
            rejected("Unknown measurement type: MAX_AT".to_string()),
            ExecutionOutcome::Unsupported {
                directive: ".pz".to_string(),
            },
        ] {
            assert!(
                adjudicate(ExecutionContract::ExpectedUnsupported, &outcome).is_none(),
                "{outcome:?}"
            );
        }
    }

    /// A completed run holding NaN is a failure, not a pass.
    #[test]
    fn non_finite_results_classify_as_a_refusal() {
        assert!(matches!(
            classify(Ok(false)),
            AnalysisOutcome::Refused(diagnostic) if diagnostic.contains("non-finite")
        ));
    }

    #[test]
    fn aborted_runs_classify_as_a_timeout() {
        assert!(matches!(
            classify(Err(SimulationError::Aborted)),
            AnalysisOutcome::TimedOut
        ));
    }
}
