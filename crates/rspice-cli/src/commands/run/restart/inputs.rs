//! Reserve checkpoint inputs for every run before any artifact is published.
use super::*;
use crate::commands::run::{RunArgs, naming};
use rspice_core::AbortSignal;
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind};

/// Shared by preflight and execution so neither can invent a different state path.
pub(crate) struct CheckpointNamespace<'a> {
    label: Option<&'a str>,
    coordinate: bool,
    repeated: bool,
    control: bool,
}

impl<'a> CheckpointNamespace<'a> {
    pub fn new(label: Option<&'a str>, coordinate: bool, repeated: bool, control: bool) -> Self {
        Self {
            label,
            coordinate,
            repeated,
            control,
        }
    }

    fn cli_base(&self, path: &Path) -> PathBuf {
        self.label.map_or_else(
            || path.to_path_buf(),
            |label| naming::tag_output_path(path, &naming::sanitize_run_tag(label)),
        )
    }

    pub fn cli_path(&self, path: &Path, analysis: &str) -> PathBuf {
        let base = self.cli_base(path);
        if self.coordinate || self.repeated || self.control {
            naming::tag_output_path(&base, analysis)
        } else {
            base
        }
    }

    fn restart_base(&self, name: &str) -> String {
        self.label.map_or_else(
            || name.to_string(),
            |label| format!("{name}.{}", naming::sanitize_run_tag(label)),
        )
    }

    pub fn restart_qualifies_analysis(&self) -> bool {
        self.control || self.repeated
    }

    pub fn restart_name(&self, name: &str, analysis: &str) -> String {
        let base = self.restart_base(name);
        if self.restart_qualifies_analysis() {
            format!("{base}.{analysis}")
        } else {
            base
        }
    }
}

pub(crate) fn protect_planned_inputs(
    netlist: &rspice_core::Netlist,
    args: &RunArgs,
    label: Option<&str>,
    coordinate: bool,
    transients: impl Iterator<Item = AnalysisInstanceId>,
    limits: rspice_core::ResourceLimits,
) -> Result<(), CliError> {
    let restart_file = netlist
        .options
        .restart
        .as_ref()
        .and_then(|restart| restart.file.as_deref());
    if args.resume.is_none() && restart_file.is_none() {
        return Ok(());
    }
    let repeated = netlist
        .analyses
        .iter()
        .filter(|analysis| matches!(analysis, rspice_core::netlist::AnalysisCommand::Tran { .. }))
        .count()
        > 1;
    let namespace = CheckpointNamespace::new(
        label,
        coordinate,
        repeated,
        netlist.control_script.is_some(),
    );
    let restart_parent = restart_file
        .map(|_| restart_namespace_parent(&args.input))
        .transpose()?;
    if namespace.control {
        // Control flow can choose the number of transients at run time. Reserve
        // existing snapshots in its canonical namespace without reading state.
        // The concrete read is protected again at dispatch, including new files.
        if let Some(resume) = &args.resume {
            let base = namespace.cli_base(resume);
            let renewal = args
                .checkpoint
                .as_deref()
                .map(|path| namespace.cli_base(path));
            protect_control_namespace(&base, renewal.as_deref(), true, args, limits)?;
        }
        if let (Some(file), Some(parent)) = (restart_file, restart_parent) {
            validate_restart_logical_name(file, "FILE")?;
            let base = parent.join(namespace.restart_base(file));
            protect_control_namespace(&base, None, false, args, limits)?;
        }
        return Ok(());
    }
    for id in transients {
        if crate::abort::ProcessAbort.is_aborted() {
            return Err(crate::commands::run::cancellation_cli_error(args.timeout));
        }
        let tag = id.tag();
        if let Some(resume) = &args.resume {
            let path = namespace.cli_path(resume, &tag);
            let renewal = args
                .checkpoint
                .as_deref()
                .map(|path| namespace.cli_path(path, &tag));
            protect_input(&path, renewal.as_deref())?;
        }
        if let (Some(file), Some(parent)) = (restart_file, &restart_parent) {
            let name = namespace.restart_name(file, &tag);
            validate_restart_logical_name(&name, "FILE")?;
            protect_input(&parent.join(name), None)?;
        }
    }
    Ok(())
}

fn protect_control_namespace(
    base: &Path,
    renewal_base: Option<&Path>,
    before_extension: bool,
    args: &RunArgs,
    limits: rspice_core::ResourceLimits,
) -> Result<(), CliError> {
    let parent = base
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut prefix = if before_extension {
        base.file_stem()
    } else {
        base.file_name()
    }
    .unwrap_or_default()
    .to_os_string();
    prefix.push(".tran-");
    let mut suffix = std::ffi::OsString::new();
    if before_extension && let Some(extension) = base.extension() {
        suffix.push(".");
        suffix.push(extension);
    }
    let prefix = namespace_key(&prefix);
    let suffix = namespace_key(&suffix);
    let entries = match std::fs::read_dir(parent) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(source) => {
            return Err(CliError::InputReadError {
                path: parent.to_path_buf(),
                source,
            });
        }
    };
    let mut count = 0usize;
    for entry in entries {
        if crate::abort::ProcessAbort.is_aborted() {
            return Err(crate::commands::run::cancellation_cli_error(args.timeout));
        }
        let entry = entry.map_err(|source| CliError::InputReadError {
            path: parent.to_path_buf(),
            source,
        })?;
        let candidate = namespace_key(&entry.file_name());
        if candidate.len() < prefix.len() + suffix.len() {
            continue;
        }
        let (head, rest) = candidate.split_at(prefix.len());
        let (digits, tail) = rest.split_at(rest.len() - suffix.len());
        if head != prefix || tail != suffix {
            continue;
        }
        let Some(ordinal) = std::str::from_utf8(digits)
            .ok()
            .and_then(|text| text.parse::<u64>().ok())
            .filter(|ordinal| (1..=u64::from(u32::MAX) + 1).contains(ordinal))
        else {
            continue;
        };
        let id = AnalysisInstanceId::new(AnalysisKind::Tran, (ordinal - 1) as u32);
        if id.tag().as_bytes().strip_prefix(b"tran-") != Some(digits) {
            continue;
        }
        count += 1;
        if count > limits.max_batch_runs {
            return Err(CliError::InvalidArgument {
                message: format!(
                    "checkpoint input namespace '{}' exceeds the {}-file limit from resources.max_batch_runs",
                    base.display(),
                    limits.max_batch_runs
                ),
                suggestion: Some(
                    "use a dedicated checkpoint basename or increase the configured limit".into(),
                ),
            });
        }
        let renewal = renewal_base.map(|base| naming::tag_output_path(base, &id.tag()));
        protect_input(&entry.path(), renewal.as_deref())?;
    }
    Ok(())
}

// Match the destination registry's Windows case folding, including Unicode.
// On Unix retain the original bytes so non-Unicode paths remain usable.
fn namespace_key(name: &std::ffi::OsStr) -> Vec<u8> {
    #[cfg(windows)]
    {
        name.to_string_lossy().to_lowercase().into_bytes()
    }
    #[cfg(not(windows))]
    {
        name.as_encoded_bytes().to_vec()
    }
}
