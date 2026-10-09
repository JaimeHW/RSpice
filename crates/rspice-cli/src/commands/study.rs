//! Study hosts prepare the same immutable task graph used by application runs.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rspice_core::AbortSignal;
use rspice_simulation::execution::{
    HeadlessPreparationError, HeadlessSourceResolver, PreparedRunSnapshot, prepare_headless_run,
};
use rspice_simulation::study_document::{StudyDocument, StudyDocumentError};
use serde_json::{Value, json};

use crate::cli::{CliError, Config, NetlistOptions, StudyArgs, StudyCommands, StudyInspectArgs};

pub fn execute(args: StudyArgs, config: &Config, quiet: bool) -> Result<(), CliError> {
    match args.command {
        StudyCommands::Check(args) => inspect(args, config, quiet, false),
        StudyCommands::Plan(args) => inspect(args, config, quiet, true),
    }
}

fn inspect(
    args: StudyInspectArgs,
    config: &Config,
    quiet: bool,
    detailed: bool,
) -> Result<(), CliError> {
    crate::abort::install_interrupt_handler();
    let timeout = args.timeout.map(crate::abort::arm_timeout).transpose()?;
    let prepared = prepare(&args.input, config).and_then(|prepared| {
        check_abort(args.timeout)?;
        Ok(prepared)
    });
    drop(timeout);
    let schema = if detailed {
        "rspice.study.plan"
    } else {
        "rspice.study.check"
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(error) => {
            // A timer may have fired during a bounded read or source expansion.
            let error = if crate::abort::ProcessAbort.is_aborted() {
                super::run::cancellation_cli_error(args.timeout)
            } else {
                error
            };
            if args.json {
                crate::console::line(format_args!(
                    "{}",
                    crate::observability::envelope(
                        schema,
                        json!({
                            "valid": false,
                            "study": args.input,
                            "errors": [{"message": error.to_string(), "details": error.details()}],
                        })
                    )
                ))?;
            }
            return Err(error);
        }
    };
    let report = prepared.report()?;
    if args.json {
        let mut report = report;
        if !detailed {
            report
                .as_object_mut()
                .expect("report object")
                .remove("tasks");
        }
        return crate::console::line(format_args!(
            "{}",
            crate::observability::envelope(schema, report)
        ));
    }
    if quiet {
        return Ok(());
    }
    crate::console::line(format_args!(
        "Valid study: {} ({} tasks)",
        prepared.document.id, report["task_count"]
    ))?;
    if detailed {
        for (index, task) in report["tasks"]
            .as_array()
            .expect("task array")
            .iter()
            .enumerate()
        {
            crate::console::line(format_args!(
                "  {}. {}: {} [{}]",
                index + 1,
                task["id"].as_str().unwrap_or_default(),
                task["label"].as_str().unwrap_or_default(),
                task["instance_id"].as_str().unwrap_or_default(),
            ))?;
        }
        crate::console::line(format_args!("Snapshot: {}", prepared.snapshot.digest()))?;
    }
    for advisory in prepared.snapshot.metadata().advisories {
        crate::observability::diagnostic("study.advisory", None, advisory);
    }
    Ok(())
}

struct PreparedStudy {
    document: StudyDocument,
    path: PathBuf,
    circuit_path: PathBuf,
    snapshot: PreparedRunSnapshot,
    // Indexed by UUID strings, never by array order after topological sorting.
    names: HashMap<String, usize>,
}

impl PreparedStudy {
    fn report(&self) -> Result<Value, CliError> {
        let receipt = self
            .snapshot
            .prepared_run_receipt()
            .map_err(|error| invalid(&self.path, error.to_string(), None))?;
        let tasks = receipt
            .tasks()
            .iter()
            .map(|task| {
                let authored = &self.document.tasks[self.names[&task.instance_id().to_string()]];
                json!({
                    "id": authored.id,
                    "label": authored.label.as_ref().unwrap_or(&authored.id),
                    "instance_id": task.instance_id(),
                    "depends_on": authored.depends_on,
                    "dependency_ids": task.dependencies(),
                    "analysis": authored.analysis,
                    "analysis_tag": task.analysis_kind_tag(),
                    "configuration_digest": task.config_digest(),
                    "qualification": task.canonical_kind().availability().label(),
                })
            })
            .collect::<Vec<_>>();
        let metadata = self.snapshot.metadata();
        Ok(json!({
            "valid": true,
            "study": self.path,
            "study_id": self.document.id,
            "study_schema_version": self.document.schema_version,
            "circuit": self.circuit_path,
            "task_count": tasks.len(),
            "tasks": tasks,
            "snapshot_digest": metadata.snapshot_digest,
            "source_digest": metadata.source_digest,
            "target": metadata.target,
            "advisories": metadata.advisories,
            "qualification_blocker": receipt.sign_off_blocker(),
            "validation_scope": "Source and task preparation; no numerical analysis has been executed.",
        }))
    }
}

fn prepare(path: &Path, config: &Config) -> Result<PreparedStudy, CliError> {
    if super::is_stdin(path) {
        return Err(CliError::InvalidArgument {
            message: "study input must be a file so relative circuit paths have an explicit base"
                .into(),
            suggestion: None,
        });
    }
    let path = std::path::absolute(path).map_err(|source| CliError::InputReadError {
        path: path.into(),
        source,
    })?;
    let source = super::read_netlist_input(&path, &NetlistOptions::default(), config)?;
    let limits = config.resources.limits();
    let document = StudyDocument::from_json(
        &source,
        limits.max_netlist_bytes,
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| map_document_error(error, &path))?;
    let circuit_path = path
        .parent()
        .expect("absolute file has parent")
        .join(&document.circuit.path);
    let circuit = super::read_netlist_input(&circuit_path, &NetlistOptions::default(), config)?;
    let mut include_search_paths = document.circuit.include_search_paths.clone();
    include_search_paths.extend(config.paths.include_paths.iter().cloned());
    include_search_paths.extend(config.paths.library_paths.iter().cloned());
    let input = document
        .run_input(
            &circuit,
            &circuit_path,
            HeadlessSourceResolver::Native {
                include_search_paths,
            },
            limits,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_document_error(error, &path))?;
    let names = input
        .tasks
        .iter()
        .enumerate()
        .map(|(index, task)| (task.instance_id.to_string(), index))
        .collect();
    let snapshot = prepare_headless_run(input, &crate::abort::ProcessAbort)
        .map_err(|error| map_preparation_error(error, &path))?;
    Ok(PreparedStudy {
        document,
        path,
        circuit_path,
        snapshot,
        names,
    })
}

fn map_document_error(error: StudyDocumentError, path: &Path) -> CliError {
    match error {
        StudyDocumentError::Aborted => super::run::cancellation_cli_error(None),
        StudyDocumentError::ResourceLimit(source) => CliError::ResourceLimit {
            path: path.into(),
            source,
        },
        StudyDocumentError::InputTooLarge { bytes, limit } => CliError::ResourceLimit {
            path: path.into(),
            source: rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::NetlistBytes,
                requested: bytes,
                limit,
            },
        },
        StudyDocumentError::Json(error) => invalid(path, error.to_string(), Some(error.line())),
        error => invalid(path, error.to_string(), None),
    }
}

fn map_preparation_error(error: HeadlessPreparationError, path: &Path) -> CliError {
    match error {
        HeadlessPreparationError::Aborted => super::run::cancellation_cli_error(None),
        HeadlessPreparationError::ResourceLimit(source) => CliError::ResourceLimit {
            path: path.into(),
            source,
        },
        HeadlessPreparationError::Parse(error) => {
            // Keep the original included file's location in structured diagnostics.
            let location = error.source_location();
            let mapped = super::map_parse_error(error);
            let mut details = mapped.details();
            if let Some(location) = location {
                details.line = Some(location.line);
                details.path = location.path.map(|path| path.display().to_string());
            }
            CliError::Reported {
                message: mapped.to_string(),
                category: mapped.category(),
                details: Box::new(details),
            }
        }
        error => invalid(path, error.to_string(), None),
    }
}

fn invalid(path: &Path, message: String, line: Option<usize>) -> CliError {
    CliError::StudyInput {
        path: path.into(),
        message,
        line,
    }
}

fn check_abort(timeout: Option<f64>) -> Result<(), CliError> {
    if crate::abort::ProcessAbort.is_aborted() {
        Err(super::run::cancellation_cli_error(timeout))
    } else {
        Ok(())
    }
}
