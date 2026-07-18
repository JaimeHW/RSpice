//! Backend liveness and readiness probes.

use std::time::Instant;

use crate::cli::{CliError, Config, HealthArgs, HealthMode};
use rspice_core::{Engine, EngineHealthReport};

pub fn execute(
    args: HealthArgs,
    config: &Config,
    _verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    let started = Instant::now();
    let engine = match Engine::try_new(config.core_simulation_config()) {
        Ok(engine) => engine,
        Err(error) => {
            let error = CliError::from(error);
            emit_failure(&args, started.elapsed().as_secs_f64(), &error);
            return Err(error);
        }
    };

    let report = match args.mode {
        HealthMode::Liveness => None,
        HealthMode::Readiness => match engine.health_check() {
            Ok(report) => Some(report),
            Err(error) => {
                let error = CliError::from(error);
                emit_failure(&args, started.elapsed().as_secs_f64(), &error);
                return Err(error);
            }
        },
    };

    let duration_secs = started.elapsed().as_secs_f64();
    if args.json {
        println!("{}", success_payload(&args, duration_secs, report.as_ref()));
    } else if !quiet {
        match report {
            Some(report) => println!(
                "RSpice backend ready ({}, {:.3} ms; parser={} elements, solver=V(out)={:.6})",
                args.mode.as_str(),
                duration_secs * 1_000.0,
                report.element_count,
                report.output_voltage,
            ),
            None => println!(
                "RSpice backend alive ({}, {:.3} ms)",
                args.mode.as_str(),
                duration_secs * 1_000.0,
            ),
        }
    }
    Ok(())
}

fn success_payload(
    args: &HealthArgs,
    duration_secs: f64,
    report: Option<&EngineHealthReport>,
) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "status": "ready",
        "ready": true,
        "mode": args.mode.as_str(),
        "duration_secs": duration_secs,
        "run_id": crate::observability::run_id(),
        "tool": tool_identity(),
        "runtime": runtime_identity(),
        "checks": {
            "configuration": {"status": "pass"},
            "parser": {
                "status": if report.is_some() { "pass" } else { "skipped" },
                "element_count": report.map(|report| report.element_count),
            },
            "solver": {
                "status": if report.is_some() { "pass" } else { "skipped" },
                "node_count": report.map(|report| report.node_count),
                "branch_count": report.map(|report| report.branch_count),
                "output_voltage": report.map(|report| report.output_voltage),
                "duration_secs": report.map(|report| report.elapsed.as_secs_f64()),
            },
        },
    })
}

fn emit_failure(args: &HealthArgs, duration_secs: f64, error: &CliError) {
    if !args.json {
        return;
    }
    let details = error.details();
    println!(
        "{}",
        serde_json::json!({
            "schema_version": 1,
            "status": "not_ready",
            "ready": false,
            "mode": args.mode.as_str(),
            "duration_secs": duration_secs,
            "run_id": crate::observability::run_id(),
            "tool": tool_identity(),
            "runtime": runtime_identity(),
            "error": {
                "message": error.to_string(),
                "code": details.code,
                "category": details.category,
                "retryable": details.retryable,
                "resource": details.resource,
                "requested": details.requested,
                "limit": details.limit,
            },
        })
    );
}

fn tool_identity() -> serde_json::Value {
    serde_json::json!({
        "name": "rspice",
        "version": env!("CARGO_PKG_VERSION"),
        "target": env!("RSPICE_BUILD_TARGET"),
        "profile": env!("RSPICE_BUILD_PROFILE"),
        "commit": env!("RSPICE_BUILD_COMMIT"),
    })
}

fn runtime_identity() -> serde_json::Value {
    serde_json::json!({
        "os": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "pointer_width": usize::BITS,
        "available_parallelism": std::thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1),
        "process_id": std::process::id(),
    })
}
