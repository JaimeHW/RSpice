//! Frontend compatibility is admitted before any solver or artifact work.
use super::*;

pub(super) fn projection_refusal(analysis: &AnalysisCommand) -> Option<CliError> {
    let (name, capability) = match analysis {
        AnalysisCommand::Qpss(_) => ("QPSS", "analysis.qpss.cli_result_document"),
        AnalysisCommand::Qpac(_) => ("QPAC", "analysis.qpac.cli_result_document"),
        AnalysisCommand::Qpxf(_) => ("QPXF", "analysis.qpxf.cli_result_document"),
        AnalysisCommand::Qpnoise(_) => ("QPNOISE", "analysis.qpnoise.cli_result_document"),
        _ => return None,
    };
    Some(rspice_core::SimulationError::unsupported_capability(capability,
        format!("the CLI has no {name} result-document projection; see the CLI supported-feature matrix")
    ).into())
}

pub(super) fn validate(netlist: &Netlist, args: &RunArgs, config: &Config) -> Result<(), CliError> {
    if netlist.control_script.is_some() {
        if args.checkpoint.is_some()
            || args.resume.is_some()
            || args.tran_stop.is_some()
            || args.compress
            || config.simulation.compress_waveforms
            || netlist.options.restart.is_some()
        {
            return Err(CliError::InvalidArgument {
                message: "control-script checkpoint, segmented-restart and compression execution is not yet implemented".into(),
                suggestion: None,
            });
        }
        if !netlist.fft_analyses.is_empty()
            || netlist
                .analyses
                .iter()
                .any(|analysis| matches!(analysis, AnalysisCommand::Four { .. }))
        {
            return Err(CliError::InvalidArgument {
                message: "declarative Fourier/FFT post-processing inside a control-script run is not yet implemented".into(),
                suggestion: None,
            });
        }
        if netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. }
            )
        }) {
            return Err(CliError::InvalidArgument {
                message:
                    "control scripts combined with declarative run axes are not yet executable"
                        .into(),
                suggestion: None,
            });
        }
        return Ok(());
    }
    let mode = requested_mode_name(args);
    let runs_cards = mode.is_none_or(|mode| mode == "--corners");
    if runs_cards {
        for analysis in &netlist.analyses {
            if let Some(error) = projection_refusal(analysis) {
                return Err(error);
            }
        }
    }
    Ok(())
}
