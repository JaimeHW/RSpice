//! Frontend compatibility is admitted before any solver or artifact work.
use super::*;

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
    Ok(())
}
