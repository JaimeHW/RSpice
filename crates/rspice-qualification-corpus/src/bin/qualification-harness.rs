//! The qualification harness driver.
//!
//! `plan` expands published policy and corpus bytes into per-execution
//! protocol-3 engine requests for the released worker adapter, in the
//! exact corpus execution order the evidence contract joins against.
//! Further stages (assembly of the retained evidence artifacts) build on
//! the plan index this stage writes.

use std::path::Path;
use std::process::ExitCode;

use rspice_qualification_corpus::harness;
use uuid::Uuid;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("qualification harness failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.first().map(String::as_str) {
        Some("plan") => {
            let [_, policy_path, corpus_path, suite_run_id, output] = arguments.as_slice() else {
                return Err(
                    "usage: qualification-harness plan <policy.json> <corpus.json> \
                     <suite-run-id> <output-directory>"
                        .to_owned(),
                );
            };
            let policy_bytes = std::fs::read(policy_path)
                .map_err(|error| format!("cannot read {policy_path}: {error}"))?;
            let corpus_bytes = std::fs::read(corpus_path)
                .map_err(|error| format!("cannot read {corpus_path}: {error}"))?;
            let suite_run_id: Uuid = suite_run_id
                .parse()
                .map_err(|error| format!("suite run id: {error}"))?;
            let executions = harness::plan(&policy_bytes, &corpus_bytes, suite_run_id)?;
            harness::write_plan(&executions, suite_run_id, Path::new(output))?;
            println!(
                "planned {} executions for suite run {suite_run_id}",
                executions.len()
            );
            Ok(())
        }
        Some("assemble") => {
            let [
                _,
                policy_path,
                corpus_path,
                suite_run_id,
                responses,
                binding_path,
                log_path,
                output,
            ] = arguments.as_slice()
            else {
                return Err(
                    "usage: qualification-harness assemble <policy.json> <corpus.json> \
                     <suite-run-id> <responses-directory> <binding.json> <log-file> \
                     <output-directory>"
                        .to_owned(),
                );
            };
            let policy_bytes = std::fs::read(policy_path)
                .map_err(|error| format!("cannot read {policy_path}: {error}"))?;
            let corpus_bytes = std::fs::read(corpus_path)
                .map_err(|error| format!("cannot read {corpus_path}: {error}"))?;
            let suite_run_id: uuid::Uuid = suite_run_id
                .parse()
                .map_err(|error| format!("suite run id: {error}"))?;
            let binding_bytes = std::fs::read(binding_path)
                .map_err(|error| format!("cannot read {binding_path}: {error}"))?;
            let binding: harness::ReleaseBinding = serde_json::from_slice(&binding_bytes)
                .map_err(|error| format!("{binding_path}: {error}"))?;
            let log_bytes = std::fs::read(log_path)
                .map_err(|error| format!("cannot read {log_path}: {error}"))?;
            let summary = harness::assemble(
                &policy_bytes,
                &corpus_bytes,
                suite_run_id,
                Path::new(responses),
                &binding,
                &log_bytes,
                Path::new(output),
            )?;
            println!(
                "assembled {} executions, case set sha256 {}",
                summary.executed_case_count, summary.case_set_sha256
            );
            Ok(())
        }
        _ => Err("usage: qualification-harness <plan|assemble> ...".to_owned()),
    }
}
