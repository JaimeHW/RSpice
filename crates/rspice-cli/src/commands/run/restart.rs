//! The authored `.OPTIONS RESTART` / Xyce `-r` segmented transient path.
//!
//! A restart schedule names its state files by logical name inside a
//! namespace directory the deck's own path establishes. Every name is checked
//! to be a single portable component before it is joined, so an authored deck
//! can never write outside that namespace, and the read and write paths are
//! resolved separately: a resumed run reads a file it did not create.

use super::RunContext;
use crate::cli::CliError;
use std::path::{Component, Path, PathBuf};

pub(super) fn run_authored_restart(
    ctx: &RunContext<'_>,
    restart: &rspice_core::netlist::XyceRestartOptions,
    tstop: f64,
    max_step: f64,
    startup_mode: rspice_core::engine::TransientStartupMode,
    progress: &indicatif::ProgressBar,
) -> Result<rspice_core::engine::TransientResult, CliError> {
    validate_supported_restart_options(restart)?;
    let abort = crate::abort::ProgressAbort::new(progress);

    match (restart.job.as_deref(), restart.file.as_deref()) {
        (Some(_), Some(_)) => Err(restart_cli_error(
            ".OPTIONS RESTART cannot specify both JOB and FILE in one run",
        )),
        (Some(job), None) => {
            let interval = restart.initial_interval.ok_or_else(|| {
                restart_cli_error(
                    ".OPTIONS RESTART JOB requires a positive INITIAL_INTERVAL checkpoint cadence",
                )
            })?;
            let parent = restart_namespace_parent(&ctx.args.input)?;
            validate_restart_logical_name(job, "JOB")?;
            let available_points = ctx
                .engine
                .config()
                .resource_limits
                .max_analysis_points
                .saturating_sub(ctx.netlist.options.output_time_points.len())
                .saturating_sub(
                    ctx.netlist
                        .options
                        .output_interval_schedule
                        .as_ref()
                        .map_or(0, |schedule| schedule.intervals.len()),
                )
                .saturating_sub(ctx.netlist.options.timeint_breakpoints.len())
                .saturating_sub(restart.intervals.len());
            // Construction validates the complete nominal namespace before
            // the expensive simulation. Xyce may intentionally skip nominal
            // files when one accepted step crosses several cadence points.
            let plan = rspice_core::engine::XyceRestartJobPlan::new(
                job,
                interval,
                &restart.intervals,
                tstop,
                restart.pack,
                available_points,
            )
            .map_err(|error| restart_cli_error(error.to_string()))?;
            let (result, checkpoints) = ctx
                .engine
                .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                    ctx.netlist,
                    tstop,
                    max_step,
                    startup_mode,
                    plan.nominal_times(),
                    &abort,
                )
                .map_err(|error| map_restart_simulation_error(ctx, error))?;
            let mut previous_nominal = None;
            for scheduled in &checkpoints {
                let nominal_time = scheduled.nominal_time;
                if plan
                    .nominal_times()
                    .binary_search_by(|time| time.total_cmp(&nominal_time))
                    .is_err()
                    || previous_nominal.is_some_and(|previous| nominal_time <= previous)
                {
                    return Err(CliError::InternalError {
                        message: format!(
                            "transient restart scheduler returned unexpected nominal time {nominal_time:.17e}s"
                        ),
                    });
                }
                previous_nominal = Some(nominal_time);
                let name = plan.logical_name(nominal_time).ok_or_else(|| {
                    CliError::InternalError {
                        message: format!(
                            "transient restart scheduler returned unnamed nominal time {nominal_time:.17e}s"
                        ),
                    }
                })?;
                let path = safe_restart_write_path(&parent, &name)?;
                scheduled
                    .checkpoint
                    .save_with_encoding_and_abort(&path, plan.encoding(), &abort)
                    .map_err(|source| CliError::CoreSimulationError {
                        source,
                        analysis: Some(format!(
                            ".OPTIONS RESTART checkpoint save ({})",
                            path.display()
                        )),
                    })?;
                if !ctx.quiet {
                    println!(
                        "  Restart checkpoint saved (nominal t={nominal_time:.6e}s, accepted t={:.6e}s): {}",
                        scheduled.checkpoint.time,
                        path.display()
                    );
                }
            }
            Ok(result)
        }
        (None, Some(file)) => {
            if restart.initial_interval.is_some() || !restart.intervals.is_empty() {
                return Err(restart_cli_error(
                    ".OPTIONS RESTART FILE cannot also specify a checkpoint interval schedule; use a separate JOB run to write checkpoints",
                ));
            }
            let parent = restart_namespace_parent(&ctx.args.input)?;
            let path = safe_restart_read_path(&parent, file)?;
            let checkpoint_limit = ctx.engine.config().resource_limits.max_external_data_bytes;
            let checkpoint = rspice_core::engine::TransientCheckpoint::load_with_limit_and_abort(
                &path,
                checkpoint_limit,
                checkpoint_limit,
                &abort,
            )
            .map_err(|source| CliError::CoreSimulationError {
                source,
                analysis: Some(format!(
                    ".OPTIONS RESTART checkpoint load ({})",
                    path.display()
                )),
            })?;
            ctx.engine
                .run_tran_restart_resume_with_abort(
                    ctx.netlist,
                    &checkpoint,
                    tstop,
                    max_step,
                    &abort,
                )
                .map(|(result, _)| result)
                .map_err(|error| map_restart_simulation_error(ctx, error))
        }
        (None, None) => Err(restart_cli_error(
            ".OPTIONS RESTART requires either JOB for checkpoint output or FILE for restart input",
        )),
    }
}

fn validate_supported_restart_options(
    restart: &rspice_core::netlist::XyceRestartOptions,
) -> Result<(), CliError> {
    if restart.print_timeint_options.is_some() {
        return Err(restart_cli_error(
            ".OPTIONS RESTART PRINT_TIMEINT_OPTIONS is not supported; RSpice validates the saved integration configuration instead of importing Xyce time-integrator options",
        ));
    }
    if restart.start_time.is_some() {
        return Err(restart_cli_error(
            ".OPTIONS RESTART START_TIME filename inference is not supported; specify the exact checkpoint logical name with FILE",
        ));
    }
    Ok(())
}

fn restart_namespace_parent(input: &Path) -> Result<PathBuf, CliError> {
    let parent = input
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::canonicalize(parent).map_err(|error| {
        restart_cli_error(format!(
            "cannot resolve the input deck directory {} for .OPTIONS RESTART: {error}",
            parent.display()
        ))
    })
}

fn validate_restart_logical_name(name: &str, option: &str) -> Result<(), CliError> {
    if name.is_empty() || name.trim() != name || name == "." || name == ".." {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART {option} must be a non-empty logical filename without surrounding whitespace"
        )));
    }
    if name.contains(['/', '\\'])
        || name
            .chars()
            .any(|character| character.is_control() || "<>:\"|?*".contains(character))
        || name.ends_with(['.', ' '])
    {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART {option} must be one portable filename, without separators or reserved filename characters"
        )));
    }
    let path = Path::new(name);
    if path.is_absolute()
        || !matches!(
            path.components().collect::<Vec<_>>().as_slice(),
            [Component::Normal(_)]
        )
    {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART {option} must name exactly one file in the input deck directory"
        )));
    }
    let device_stem = name
        .split('.')
        .next()
        .unwrap_or_default()
        .trim_end_matches(['.', ' '])
        .to_ascii_uppercase();
    let reserved = matches!(device_stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (device_stem.len() == 4
            && matches!(&device_stem[..3], "COM" | "LPT")
            && matches!(device_stem.as_bytes()[3], b'1'..=b'9'));
    if reserved {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART {option} uses reserved device filename '{name}'"
        )));
    }
    Ok(())
}

fn safe_restart_write_path(parent: &Path, name: &str) -> Result<PathBuf, CliError> {
    validate_restart_logical_name(name, "JOB output")?;
    let path = parent.join(name);
    match std::fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(restart_cli_error(format!(
            "refusing to replace .OPTIONS RESTART symlink {}",
            path.display()
        ))),
        Ok(metadata) if metadata.is_dir() => Err(restart_cli_error(format!(
            ".OPTIONS RESTART destination {} is a directory",
            path.display()
        ))),
        Ok(_) => Ok(path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(path),
        Err(error) => Err(restart_cli_error(format!(
            "cannot inspect .OPTIONS RESTART destination {}: {error}",
            path.display()
        ))),
    }
}

fn safe_restart_read_path(parent: &Path, name: &str) -> Result<PathBuf, CliError> {
    validate_restart_logical_name(name, "FILE")?;
    let path = parent.join(name);
    let metadata = std::fs::symlink_metadata(&path).map_err(|error| {
        restart_cli_error(format!(
            "cannot inspect .OPTIONS RESTART FILE {}: {error}",
            path.display()
        ))
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART FILE must be a regular, non-symlink file in the input deck directory: {}",
            path.display()
        )));
    }
    let resolved = std::fs::canonicalize(&path).map_err(|error| {
        restart_cli_error(format!(
            "cannot resolve .OPTIONS RESTART FILE {}: {error}",
            path.display()
        ))
    })?;
    if resolved.parent() != Some(parent) {
        return Err(restart_cli_error(format!(
            ".OPTIONS RESTART FILE escapes the input deck directory: {}",
            path.display()
        )));
    }
    Ok(resolved)
}

pub(super) fn restart_cli_error(message: impl Into<String>) -> CliError {
    CliError::SimulationError {
        message: message.into(),
        analysis: Some("Transient restart".to_string()),
    }
}

pub(super) fn map_restart_simulation_error(
    ctx: &RunContext<'_>,
    error: rspice_core::SimulationError,
) -> CliError {
    if matches!(error, rspice_core::SimulationError::Aborted) {
        super::cancellation_cli_error(ctx.args.timeout)
    } else {
        CliError::CoreSimulationError {
            source: error,
            analysis: Some("Transient restart".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restart_logical_names_are_single_portable_components() {
        validate_restart_logical_name("trans_test2e-08", "FILE").unwrap();
        for unsafe_name in [
            "",
            ".",
            "..",
            "../state",
            "sub/state",
            "sub\\state",
            "C:state",
        ] {
            assert!(validate_restart_logical_name(unsafe_name, "FILE").is_err());
        }
    }
}
