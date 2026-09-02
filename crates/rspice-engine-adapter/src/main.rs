//! Self-contained RSpice engine executor for the cloud worker protocol.
//!
//! The worker launches this binary with a cleared environment, writes exactly
//! one protocol-3 JSON request to standard input, and reads exactly one JSON
//! response from standard output. Standard error carries private diagnostics
//! only. A zero exit status means a well-formed response was emitted —
//! including canonical `status: failed` outcomes for customer circuits the
//! solver rejects. A non-zero exit status is reserved for launch-contract and
//! sandbox-authority violations, which the worker treats as controller
//! faults rather than customer results.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use rspice_engine_adapter::document::{CircuitContent, IncludeSources, interpret_document};
use rspice_engine_adapter::execute::{self, Execution};
use rspice_engine_adapter::wire::{
    self, EngineResponse, MAX_ENGINE_REQUEST_BYTES, MAX_ENGINE_RESPONSE_BYTES,
    MAX_ENGINE_RESULT_MANIFEST_BYTES,
};

/// The launch environment did not match the reviewed self-contained contract.
const EXIT_LAUNCH_CONTRACT: u8 = 10;
/// Standard input could not be read to end-of-file.
const EXIT_REQUEST_IO: u8 = 11;
/// The request violated the wire contract the worker also enforces.
const EXIT_REQUEST_INVALID: u8 = 12;
/// A manifested artifact or configured component failed re-verification.
const EXIT_SANDBOX_AUTHORITY: u8 = 13;
/// The response could not be serialized or written.
const EXIT_RESPONSE_IO: u8 = 14;

/// Engine build identity: the workspace version plus the exact source SHA
/// stamped by the component release lane. This is the `engine_build` value
/// the producing workflow asserts in the signed component predicate.
fn engine_build() -> String {
    format!(
        "{}+{}",
        env!("CARGO_PKG_VERSION"),
        option_env!("RSPICE_BUILD_SHA").unwrap_or("development"),
    )
}

fn main() -> ExitCode {
    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        None => serve(),
        Some("component-info") if arguments.next().is_none() => {
            println!(
                "{}",
                serde_json::json!({
                    "component": "rspice-engine-adapter",
                    "engine_name": "rspice",
                    "engine_build": engine_build(),
                    "runtime_mode": "self_contained",
                    "protocol_versions": [3],
                    "document_schemas": ["rspice-circuit-v1"],
                    "result_schemas": [
                        "rspice-analog-result-v1",
                        "rspice-transient-fft-result-v1",
                        "rspice-axis-execution-v1"
                    ],
                })
            );
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("unknown argument {other:?}; this executor serves one stdin request");
            ExitCode::from(EXIT_LAUNCH_CONTRACT)
        }
    }
}

fn serve() -> ExitCode {
    let model_library_path = match validate_launch_environment() {
        Ok(path) => path,
        Err(violation) => {
            eprintln!("launch contract violation: {violation}");
            return ExitCode::from(EXIT_LAUNCH_CONTRACT);
        }
    };

    // The engine parallelizes device evaluation through rayon. Repetition
    // deterministic replay compares series hashes bit for bit, and work-stealing
    // reduction order is not a determinism guarantee we currently claim, so
    // this executor pins the pool to one thread. Lifting the pin requires a
    // deterministic-reduction audit of every parallel solver path.
    if let Err(error) = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
    {
        eprintln!("failed to pin the compute pool: {error}");
        return ExitCode::from(EXIT_LAUNCH_CONTRACT);
    }

    let mut request_bytes = Vec::new();
    if let Err(error) = std::io::stdin()
        .lock()
        .take(MAX_ENGINE_REQUEST_BYTES as u64 + 1)
        .read_to_end(&mut request_bytes)
    {
        eprintln!("failed to read the request: {error}");
        return ExitCode::from(EXIT_REQUEST_IO);
    }

    let request = match wire::parse_engine_request(&request_bytes) {
        Ok(request) => request,
        Err(error) => {
            // The worker validated this same envelope before launch, so a
            // disagreement here is contract drift, not customer content.
            eprintln!("request contract violation: {error}");
            return ExitCode::from(EXIT_REQUEST_INVALID);
        }
    };

    let sources =
        match IncludeSources::bind(&request.revision.artifacts, model_library_path.as_deref()) {
            Ok(sources) => sources,
            Err(fault) => {
                // The worker byte-verified every artifact and component before
                // launch; failing re-verification means the sandbox lied.
                eprintln!("sandbox authority violation: {fault}");
                return ExitCode::from(EXIT_SANDBOX_AUTHORITY);
            }
        };

    let execution = match interpret_document(&request.revision.document, &sources) {
        Ok(content) => run(&request.analysis, &content),
        Err(rejection) => Execution {
            response: EngineResponse::failed(rejection.failure_code, &rejection.failure_detail),
            artifacts: Vec::new(),
        },
    };

    if !execution.artifacts.is_empty()
        && let Err(fault) = execute::write_artifacts(Path::new("results"), &execution.artifacts)
    {
        eprintln!("sandbox authority violation: {fault}");
        return ExitCode::from(EXIT_SANDBOX_AUTHORITY);
    }

    emit(&execution.response)
}

fn run(analysis: &serde_json::Value, content: &CircuitContent) -> Execution {
    let execution = execute::execute(analysis, content, &engine_build());
    // The response must fit the wire budget with its manifest and declared
    // artifacts. An oversized manifest is a bounded customer outcome: the
    // waveform data still exists conceptually, but this run cannot represent
    // it within the reviewed protocol, and saying so beats truncating.
    if let EngineResponse::Succeeded {
        result_manifest, ..
    } = &execution.response
    {
        match serde_json::to_vec(result_manifest) {
            Ok(bytes) if result_manifest_size_allowed(bytes.len()) => {}
            Ok(_) => {
                return Execution {
                    response: EngineResponse::failed(
                        "results.manifest_too_large",
                        "The result manifest exceeds its protocol byte limit; reduce the number of saved signals in the deck.",
                    ),
                    artifacts: Vec::new(),
                };
            }
            Err(_) => {
                return Execution {
                    response: EngineResponse::failed(
                        "results.manifest_invalid",
                        "The result manifest could not be serialized.",
                    ),
                    artifacts: Vec::new(),
                };
            }
        }
    }
    match serde_json::to_vec(&execution.response) {
        Ok(bytes) if bytes.len() <= MAX_ENGINE_RESPONSE_BYTES => execution,
        Ok(_) => Execution {
            response: EngineResponse::failed(
                "results.manifest_too_large",
                "The result manifest exceeds the response budget; reduce the number of \
                 saved signals in the deck.",
            ),
            artifacts: Vec::new(),
        },
        Err(_) => Execution {
            response: EngineResponse::failed(
                "results.manifest_invalid",
                "The result manifest could not be serialized.",
            ),
            artifacts: Vec::new(),
        },
    }
}

const fn result_manifest_size_allowed(serialized_bytes: usize) -> bool {
    serialized_bytes <= MAX_ENGINE_RESULT_MANIFEST_BYTES
}

fn emit(response: &EngineResponse) -> ExitCode {
    let bytes = match serde_json::to_vec(response) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("failed to serialize the response: {error}");
            return ExitCode::from(EXIT_RESPONSE_IO);
        }
    };
    debug_assert!(bytes.len() <= MAX_ENGINE_RESPONSE_BYTES);
    let mut stdout = std::io::stdout().lock();
    if let Err(error) = stdout.write_all(&bytes).and_then(|()| stdout.flush()) {
        eprintln!("failed to write the response: {error}");
        return ExitCode::from(EXIT_RESPONSE_IO);
    }
    ExitCode::SUCCESS
}

/// Enforces the reviewed self-contained launch contract: the protocol flags
/// the worker always sets, no delegated solver, and an optional byte-bound
/// model bundle. Anything else means this binary is running under a
/// deployment it was not reviewed for.
fn validate_launch_environment() -> Result<Option<PathBuf>, String> {
    expect_env("RSPICE_ENGINE_PROTOCOL_VERSION", "3")?;
    expect_env("RSPICE_ENGINE_INPUT", "stdin-json")?;
    expect_env("RSPICE_ENGINE_OUTPUT", "stdout-json")?;
    if std::env::var_os("RSPICE_ENGINE_SOLVER_PATH").is_some() {
        return Err(
            "RSPICE_ENGINE_SOLVER_PATH is set, but this executor is the reviewed \
             self-contained engine and never launches a delegated solver"
                .to_owned(),
        );
    }
    match std::env::var_os("RSPICE_ENGINE_MODEL_LIBRARY_PATH") {
        None => Ok(None),
        Some(value) => {
            let path = PathBuf::from(value);
            if path.is_file() {
                Ok(Some(path))
            } else {
                Err("RSPICE_ENGINE_MODEL_LIBRARY_PATH does not name a readable file".to_owned())
            }
        }
    }
}

fn expect_env(name: &str, expected: &str) -> Result<(), String> {
    match std::env::var(name) {
        Ok(value) if value == expected => Ok(()),
        Ok(value) => Err(format!(
            "{name} is {value:?}; this executor requires {expected:?}"
        )),
        Err(_) => Err(format!(
            "{name} is unset; this executor requires {expected:?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_manifest_byte_limit_accepts_the_boundary_only() {
        assert!(result_manifest_size_allowed(
            MAX_ENGINE_RESULT_MANIFEST_BYTES
        ));
        assert!(!result_manifest_size_allowed(
            MAX_ENGINE_RESULT_MANIFEST_BYTES + 1
        ));
    }
}
