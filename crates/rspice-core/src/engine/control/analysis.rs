//! Non-executing command admission shared by static checks and execution.
use super::*;
use crate::ResourceLimits;
use crate::netlist::{NetlistParseOptions, ParseError, ParseWithAbortError};

impl ControlCircuit {
    /// Direct control command for an analysis identity, if this host executes it.
    /// This describes the control host only, not declarative deck execution.
    pub const fn analysis_command_name(
        kind: crate::identity::AnalysisKind,
    ) -> Option<&'static str> {
        kind.control_command()
    }

    /// Check host availability without executing a command or its arguments.
    pub fn validate_command_name(command: &ControlCommand) -> Result<(), ControlError> {
        CommandKind::parse(command).map(|_| ())
    }

    /// Whether this command names an explicit electrical analysis. This does
    /// not parse its arguments or classify the orchestration command `run`.
    pub fn is_analysis_command(command: &ControlCommand) -> bool {
        matches!(CommandKind::parse(command), Ok(CommandKind::Analysis))
    }

    /// Check that this analysis has a control execution handler, without
    /// validating its parameters, circuit capabilities or resource needs.
    pub fn validate_analysis_support(
        analysis: &AnalysisCommand,
        line: usize,
    ) -> Result<(), ControlError> {
        identity(analysis, line).map(|_| ())
    }

    /// Parse a supported analysis request using the execution grammar and limits.
    /// Arguments must already be literal or substituted. No analysis is run.
    pub fn parse_analysis_command(
        command: &ControlCommand,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisCommand, ControlExecutionError> {
        let line = command.line;
        if !matches!(CommandKind::parse(command)?, CommandKind::Analysis) {
            return Err(
                command_error(line, "expected a supported electrical analysis command").into(),
            );
        }
        if command.arguments.contains(['\r', '\n']) {
            return Err(command_error(
                line,
                "a control command cannot contain another physical line",
            )
            .into());
        }
        if command.name == "op" && !command.arguments.trim().is_empty() {
            return Err(command_error(line, "op does not accept arguments").into());
        }
        let source = format!(
            "control analysis\n.{} {}\n.end\n",
            command.name, command.arguments
        );
        let parsed = Netlist::parse_with_options_and_abort(
            &source,
            NetlistParseOptions {
                resource_limits,
                ..NetlistParseOptions::default()
            },
            abort,
        )
        .map_err(|error| match error {
            ParseWithAbortError::Aborted => simulation_error(line, SimulationError::Aborted),
            ParseWithAbortError::Parse(ParseError::ResourceLimit(error)) => {
                simulation_error(line, error.into())
            }
            error => ControlError::new(line, ControlErrorKind::Syntax, error.to_string()).into(),
        })?;
        let mut analyses = parsed.analyses.into_iter();
        let analysis = analyses
            .next()
            .ok_or_else(|| command_error(line, "analysis command did not produce an analysis"))?;
        if analyses.next().is_some() {
            return Err(
                command_error(line, "analysis command produced more than one analysis").into(),
            );
        }
        Ok(analysis)
    }
}

pub(super) fn identity(
    analysis: &AnalysisCommand,
    line: usize,
) -> Result<(&'static str, crate::identity::AnalysisKind), ControlError> {
    use crate::identity::AnalysisKind;
    let kind = match analysis {
        AnalysisCommand::Op => AnalysisKind::Op,
        AnalysisCommand::Dc { .. } => AnalysisKind::Dc,
        AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => AnalysisKind::Ac,
        AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => AnalysisKind::Noise,
        AnalysisCommand::Tran { .. } => AnalysisKind::Tran,
        AnalysisCommand::Tf { .. } => AnalysisKind::TransferFunction,
        AnalysisCommand::PoleZero { .. } => AnalysisKind::PoleZero,
        AnalysisCommand::Sensitivity { .. } => AnalysisKind::Sensitivity,
        AnalysisCommand::Disto { .. } => AnalysisKind::Distortion,
        AnalysisCommand::Stb { .. } => AnalysisKind::Stb,
        AnalysisCommand::DcMatch(_) => AnalysisKind::DcMatch,
        _ => {
            return Err(command_error(
                line,
                "this analysis has no control-host execution handler",
            ));
        }
    };
    ControlCircuit::analysis_command_name(kind)
        .map(|name| (name, kind))
        .ok_or_else(|| command_error(line, "this analysis has no control-host execution handler"))
}

// Preserve resource failures and cooperative cancellation during grid construction.
pub(super) fn frequency_error(
    line: usize,
    error: crate::analysis::FrequencyGridError,
) -> ControlExecutionError {
    use crate::analysis::FrequencyGridError;
    match error {
        error @ (FrequencyGridError::Aborted
        | FrequencyGridError::LimitExceeded { .. }
        | FrequencyGridError::Allocation { .. }) => simulation_error(line, error.into()),
        other => command_error(line, other.to_string()).into(),
    }
}

/// Keep frequency setup/retention outside the generic control dispatch frame.
/// These result families use the same bounded, cancellable physical grid.
pub(super) fn frequency(
    engine: &Engine,
    netlist: &Netlist,
    command: &AnalysisCommand,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<(ControlAnalysisResult, usize), ControlExecutionError> {
    let (variation, points, start_freq, stop_freq) = match command {
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        }
        | AnalysisCommand::Noise {
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        }
        | AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        } => (*variation, *points, *start_freq, *stop_freq),
        _ => {
            return Err(
                command_error(line, "expected AC, NOISE or DISTO frequency analysis").into(),
            );
        }
    };
    let frequencies = crate::analysis::ac::try_ac_sweep_frequencies_bounded_with_abort(
        variation,
        points,
        start_freq,
        stop_freq,
        engine.config().resource_limits.max_analysis_points,
        abort,
    )
    .map_err(|error| frequency_error(line, error))?;
    if let AnalysisCommand::Disto { f2_over_f1, .. } = command {
        let result = engine
            .run_distortion_with_abort(netlist, &frequencies, *f2_over_f1, abort)
            .map_err(|error| simulation_error(line, error))?;
        let count = result.retained_value_count();
        return Ok((ControlAnalysisResult::Distortion(Box::new(result)), count));
    }
    if let AnalysisCommand::Noise {
        output_node,
        reference_node,
        input_source,
        ..
    } = command
    {
        let result = engine
            .run_noise_named_with_input_source_and_abort(
                netlist,
                output_node,
                reference_node.as_deref(),
                input_source,
                &frequencies,
                engine.config().temperature,
                abort,
            )
            .map_err(|error| simulation_error(line, error))?;
        let count = result
            .iter()
            .fold(0usize, |n, p| n.saturating_add(p.retained_value_count()));
        Ok((ControlAnalysisResult::Noise(result), count))
    } else {
        let result = engine
            .run_ac_with_abort(netlist, &frequencies, abort)
            .map_err(|error| simulation_error(line, error))?;
        let count = result
            .iter()
            .map(|point| {
                point
                    .voltages
                    .len()
                    .saturating_add(point.currents.len())
                    .saturating_mul(2)
                    .saturating_add(1)
            })
            .fold(0usize, usize::saturating_add);
        Ok((ControlAnalysisResult::Ac(result), count))
    }
}

pub(super) fn frequency_table(
    engine: &Engine,
    netlist: &Netlist,
    command: &AnalysisCommand,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<(ControlAnalysisResult, usize), ControlExecutionError> {
    match command {
        AnalysisCommand::AcData { table_name } => {
            let result = engine
                .run_ac_table_with_abort(netlist, table_name, abort)
                .map_err(|error| simulation_error(line, error))?;
            let count = result.retained_value_count();
            Ok((ControlAnalysisResult::AcTable(result), count))
        }
        AnalysisCommand::NoiseData {
            output_node,
            reference_node,
            input_source,
            table_name,
        } => {
            let result = engine
                .run_noise_table_named_with_input_source_and_abort(
                    netlist,
                    output_node,
                    reference_node.as_deref(),
                    input_source,
                    table_name,
                    engine.config().temperature,
                    abort,
                )
                .map_err(|error| simulation_error(line, error))?;
            let count = result.retained_value_count();
            Ok((ControlAnalysisResult::NoiseTable(result), count))
        }
        _ => Err(command_error(line, "expected an AC or NOISE table").into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advertised_control_analyses_parse_and_resolve_to_their_identity() {
        use crate::identity::AnalysisKind;
        let fixtures = [
            (AnalysisKind::Op, ""),
            (AnalysisKind::Dc, "V1 0 1 0.5"),
            (AnalysisKind::Ac, "lin 3 1 10"),
            (AnalysisKind::Noise, "V(out) V1 lin 3 1 10"),
            (AnalysisKind::Tran, "1n 10n"),
            (AnalysisKind::TransferFunction, "V(out) V1"),
            (AnalysisKind::PoleZero, "in 0 out 0 vol pz"),
            (AnalysisKind::Sensitivity, "V(out) R1"),
            (AnalysisKind::Distortion, "lin 3 1 10"),
            (AnalysisKind::Stb, "lin 3 1 10 probe=Vprobe"),
            (AnalysisKind::DcMatch, "OUT=V(out)"),
        ];
        for (kind, arguments) in fixtures {
            let name = ControlCircuit::analysis_command_name(kind).expect("advertised handler");
            let command = ControlCommand {
                line: 7,
                name: name.into(),
                arguments: arguments.into(),
            };
            let analysis = ControlCircuit::parse_analysis_command(
                &command,
                ResourceLimits::default(),
                &crate::NoAbort,
            )
            .expect("command grammar admits advertised analysis");
            assert_eq!(
                identity(&analysis, 7).expect("host admits analysis"),
                (name, kind)
            );
        }
        let advertised: std::collections::BTreeSet<_> = AnalysisKind::ALL
            .into_iter()
            .filter(|kind| ControlCircuit::analysis_command_name(*kind).is_some())
            .collect();
        assert_eq!(
            advertised,
            fixtures.into_iter().map(|(kind, _)| kind).collect()
        );
    }
    use crate::analysis::FrequencyGridError;
    use crate::{SimulationErrorCategory, SimulationErrorCode};
    use std::error::Error;

    #[test]
    fn frequency_grid_allocation_failure_keeps_control_line_and_allocator_cause() {
        let source = Vec::<f64>::new().try_reserve_exact(usize::MAX).unwrap_err();
        let error = frequency_error(
            17,
            FrequencyGridError::Allocation {
                requested: usize::MAX,
                source: source.clone(),
            },
        );
        let ControlExecutionError::Simulation {
            line,
            source: simulation,
        } = error
        else {
            panic!("allocation failure was reported as a command error");
        };
        assert_eq!(line, 17);
        assert_eq!(
            simulation.descriptor().category,
            SimulationErrorCategory::ResourceLimit
        );
        assert_eq!(
            simulation.descriptor().code,
            SimulationErrorCode::AllocationFailed
        );
        assert_eq!(
            simulation
                .source()
                .unwrap()
                .downcast_ref::<std::collections::TryReserveError>(),
            Some(&source)
        );
        assert!(matches!(
            frequency_error(17, FrequencyGridError::InvalidStartFrequency),
            ControlExecutionError::Command(_)
        ));
        assert!(matches!(
            frequency_error(17, FrequencyGridError::Aborted),
            ControlExecutionError::Simulation {
                line: 17,
                source: SimulationError::Aborted
            }
        ));
    }

    fn variant(command: &AnalysisCommand) -> &'static str {
        use AnalysisCommand as A;
        match command {
            A::Op => "Op",
            A::Dc { .. } => "Dc",
            A::Ac { .. } => "Ac",
            A::AcData { .. } => "AcData",
            A::Hb(_) => "Hb",
            A::Qpss(_) => "Qpss",
            A::Qpac(_) => "Qpac",
            A::Qpxf(_) => "Qpxf",
            A::Qpnoise(_) => "Qpnoise",
            A::Sp { .. } => "Sp",
            A::Stb { .. } => "Stb",
            A::Disto { .. } => "Disto",
            A::Tran { .. } => "Tran",
            A::Noise { .. } => "Noise",
            A::NoiseData { .. } => "NoiseData",
            A::PoleZero { .. } => "PoleZero",
            A::Sensitivity { .. } => "Sensitivity",
            A::Tf { .. } => "Tf",
            A::Four { .. } => "Four",
            A::MonteCarlo(_) => "MonteCarlo",
            A::Step(_) => "Step",
            A::Temp { .. } => "Temp",
            A::Pss(_) => "Pss",
            A::Pac(_) => "Pac",
            A::Pxf(_) => "Pxf",
            A::Pnoise(_) => "Pnoise",
            A::Pstb(_) => "Pstb",
            A::Envelope(_) => "Envelope",
            A::DcMatch(_) => "DcMatch",
        }
    }

    /// `run` executes exactly the analyses static admission accepts, each into
    /// one dataset, and refuses every other analysis before producing one. A
    /// new `AnalysisCommand` variant needs a row here.
    #[test]
    fn run_executes_exactly_the_analyses_static_admission_accepts() {
        // (variant, card, whether the control host executes it)
        const ANALYSES: [(&str, &str, bool); 29] = [
            ("Op", ".op", true),
            ("Dc", ".dc V1 0 1 .5", true),
            ("Ac", ".ac lin 2 1 10", true),
            ("AcData", ".ac data=grid", true),
            ("Hb", ".hb 1k", false),
            ("Qpss", ".qpss 1k 1.4142135623730951k", false),
            (
                "Qpac",
                ".qpac lin 2 1 10 source=V1 out=V(out) inlattice=(0,0) outlattice=(0,0)",
                false,
            ),
            ("Qpxf", ".qpxf list=(1) out=V(out) outlattice=(0,0)", false),
            (
                "Qpnoise",
                ".qpnoise list=(1) out=V(out) outlattice=(0,0) minlattice=(0,0) maxlattice=(0,0)",
                false,
            ),
            ("Sp", ".sp lin 2 1 10", false),
            ("Stb", ".stb lin 2 1 10 probe=V1", true),
            ("Disto", ".disto lin 3 1 10 .9", true),
            ("Tran", ".tran 1u 1m", true),
            ("Noise", ".noise V(out) V1 lin 2 1 10", true),
            ("NoiseData", ".noise V(out) V1 data=grid", true),
            ("PoleZero", ".pz in 0 out 0 vol pz", true),
            ("Sensitivity", ".sens V(out)", true),
            ("Tf", ".tf V(out) V1", true),
            ("Four", ".four 1k V(out)", false),
            ("MonteCarlo", ".mc 2 uniform .1 seed 37", false),
            ("Step", ".step param r list 1k 2k", false),
            ("Temp", ".temp 0 27 100", false),
            ("Pss", ".pss fund=1k", false),
            ("Pac", ".pac lin 2 1 10 input=V1 out=V(out)", false),
            ("Pxf", ".pxf lin 2 1 10 input=V1 out=V(out)", false),
            ("Pnoise", ".pnoise lin 2 1 10 out=V(out)", false),
            ("Pstb", ".pstb probe=V1 maxharm=3", false),
            ("Envelope", ".envelope tstop=1m maxstep=1u", false),
            ("DcMatch", ".dcmatch out=V(out)", true),
        ];
        let mut variants = std::collections::BTreeSet::new();
        for (name, card, executed) in ANALYSES {
            assert!(variants.insert(name), "duplicate analysis {name}");
            let source = if name == "Stb" {
                format!(
                    "Control stability\nE1 out 0 sense 0 -10\nV1 out drive 0\nR1 drive sense 1k\nC1 sense 0 1u\n{card}\n.end\n"
                )
            } else {
                format!(
                    "Control analyses\n.param r=1k\nV1 in 0 1 AC 1 DISTOF1 1m DISTOF2 1m\nR1 in out {{r}}\nR2 out 0 1k\n.data grid freq\n1\n10\n.enddata\n{card}\n.end\n"
                )
            };
            let mut netlist =
                crate::Netlist::parse(&source).unwrap_or_else(|error| panic!("{name}: {error}"));
            if name == "DcMatch" {
                netlist
                    .spectre_statistics
                    .variations
                    .push(crate::netlist::SpectreVariation {
                        line: 1,
                        scope: crate::netlist::SpectreVariationScope::Mismatch,
                        parameter: "r".into(),
                        distribution: crate::netlist::SpectreDistribution::Gaussian,
                        spread: crate::netlist::SpectreSpread::StandardDeviation("10".into()),
                        percent: false,
                        bounds: None,
                    });
            }
            let [command] = netlist.analyses.as_slice() else {
                panic!("{name}: the card must name exactly one command");
            };
            assert_eq!(variant(command), name);
            assert_eq!(
                ControlCircuit::validate_analysis_support(command, 1).is_ok(),
                executed,
                "{name}: static admission must match execution"
            );
            // `run` selects standalone producers. Fourier and sweep or
            // temperature cards need a producer, so alone they leave `run`
            // nothing to execute.
            let needs_producer = matches!(
                command,
                AnalysisCommand::Four { .. }
                    | AnalysisCommand::Step(_)
                    | AnalysisCommand::Temp { .. }
            );
            let mut host = ControlCircuit::new(netlist).unwrap();
            let outcome = host.execute(
                &Engine::default(),
                &ControlCommand {
                    name: "run".into(),
                    arguments: String::new(),
                    line: 1,
                },
                &ParamContext::new(),
                &crate::NoAbort,
            );
            if executed {
                outcome.unwrap_or_else(|error| panic!("{name}: {error}"));
                assert_eq!(host.datasets().len(), 1);
            } else {
                let error = outcome
                    .expect_err("an analysis without a control handler is refused")
                    .to_string();
                let expected = if needs_producer {
                    "run has no declarative analysis to execute"
                } else {
                    "no control-host execution handler"
                };
                assert!(error.contains(expected), "{name}: {error}");
                assert!(host.datasets().is_empty());
            }
        }
    }
}
