//! Release qualification for one signed, app-local managed CPython runtime.
//!
//! This executable is intentionally separate from the GUI. Native release
//! assembly runs it against the exact staged and signed runtime before the
//! runtime can be packaged with RSpice. It proves signature verification,
//! protocol identity, authoritative compilation, governed execution, bounded
//! output, clean shutdown, hard cancellation, and watchdog termination.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitStatus,
    thread,
    time::{Duration, Instant},
};

use rspice_automation_protocol::{
    Digest, DocumentRole, EventEnvelope, ExceptionPolicy, LaunchMode, PROTOCOL_VERSION,
    RequestEnvelope, ResourceLimits, RuntimeEvent, RuntimeRequest, RuntimeState, SourceDocument,
    SourceSnapshot,
};
use rspice_automation_runtime::{
    NativeWorker, RuntimeError, RuntimeRequirement, RuntimeTrustStore, VerifiedRuntime,
};
use semver::VersionReq;
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

const EVENT_POLL_INTERVAL: Duration = Duration::from_millis(5);
const STARTUP_TIMEOUT: Duration = Duration::from_secs(20);
const OPERATION_TIMEOUT: Duration = Duration::from_secs(20);
const TERMINATION_TIMEOUT: Duration = Duration::from_secs(5);
const QUALIFIED_OUTPUT: &str = "RSPICE_MANAGED_RUNTIME_QUALIFIED_π\n";

#[derive(Debug)]
struct Options {
    runtime_root: PathBuf,
    key_id: String,
    public_key: [u8; 32],
    target: String,
    architecture: String,
    python_version: String,
    api_version: String,
    environment_digest: Digest,
}

struct ScratchDirectory(PathBuf);

impl ScratchDirectory {
    fn new(label: &str) -> Result<Self, String> {
        let path = env::temp_dir().join(format!(
            "rspice-managed-runtime-qualification-{label}-{}",
            Uuid::new_v4()
        ));
        fs::create_dir(&path).map_err(|error| {
            format!(
                "could not create qualification scratch '{}': {error}",
                path.display()
            )
        })?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ScratchDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("managed-runtime qualification failed: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let options = parse_options(env::args().skip(1))?;
    let runtime = verify_runtime(&options)?;
    let validation_ms = qualify_completed_launch(
        &runtime,
        &options,
        LaunchMode::Validate,
        "value = 6 * 7\n",
        None,
        true,
    )?;
    let execution_ms = qualify_completed_launch(
        &runtime,
        &options,
        LaunchMode::Run,
        &format!("print({QUALIFIED_OUTPUT:?}, end=\"\")\n"),
        Some(QUALIFIED_OUTPUT),
        false,
    )?;
    let hard_cancel_ms = qualify_hard_cancellation(&runtime, &options)?;
    let watchdog_ms = qualify_watchdog_termination(&runtime, &options)?;

    let evidence = serde_json::json!({
        "schema": "rspice.managed-runtime-qualification/v1",
        "runtime_build": runtime.identity().runtime_build,
        "runtime_digest": hex(&runtime.identity().runtime_digest.0),
        "python_version": runtime.identity().python_version,
        "python_abi": runtime.identity().python_abi,
        "rspice_api_version": runtime.identity().rspice_api_version,
        "target": options.target,
        "architecture": options.architecture,
        "validation_ms": validation_ms,
        "execution_ms": execution_ms,
        "hard_cancel_ms": hard_cancel_ms,
        "watchdog_ms": watchdog_ms,
        "system_python_path_used": false,
        "passed": true,
    });
    println!(
        "{}",
        serde_json::to_string(&evidence)
            .map_err(|error| format!("could not serialize qualification evidence: {error}"))?
    );
    Ok(())
}

fn parse_options(arguments: impl Iterator<Item = String>) -> Result<Options, String> {
    let mut arguments = arguments;
    let mut runtime_root = None;
    let mut key_id = None;
    let mut public_key = None;
    let mut target = None;
    let mut architecture = None;
    let mut python_version = None;
    let mut api_version = None;
    let mut environment_digest = None;
    while let Some(argument) = arguments.next() {
        let value = arguments
            .next()
            .ok_or_else(|| format!("{argument} requires a value"))?;
        match argument.as_str() {
            "--runtime-root" => set_once(&mut runtime_root, PathBuf::from(value), &argument)?,
            "--key-id" => set_once(&mut key_id, value, &argument)?,
            "--public-key-hex" => set_once(
                &mut public_key,
                decode_hex_32(&value, "public key")?,
                &argument,
            )?,
            "--target" => set_once(&mut target, value, &argument)?,
            "--architecture" => set_once(&mut architecture, value, &argument)?,
            "--python-version" => set_once(&mut python_version, value, &argument)?,
            "--api-version" => set_once(&mut api_version, value, &argument)?,
            "--environment-digest" => set_once(
                &mut environment_digest,
                Digest(decode_hex_32(&value, "environment digest")?),
                &argument,
            )?,
            _ => return Err(format!("unknown argument {argument}")),
        }
    }
    let options = Options {
        runtime_root: required(runtime_root, "--runtime-root")?,
        key_id: required(key_id, "--key-id")?,
        public_key: required(public_key, "--public-key-hex")?,
        target: required(target, "--target")?,
        architecture: required(architecture, "--architecture")?,
        python_version: required(python_version, "--python-version")?,
        api_version: required(api_version, "--api-version")?,
        environment_digest: required(environment_digest, "--environment-digest")?,
    };
    for (label, value) in [
        ("key ID", options.key_id.as_str()),
        ("target", options.target.as_str()),
        ("architecture", options.architecture.as_str()),
        ("Python version", options.python_version.as_str()),
        ("API version", options.api_version.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(format!("{label} must not be empty"));
        }
    }
    Ok(options)
}

fn set_once<T>(slot: &mut Option<T>, value: T, argument: &str) -> Result<(), String> {
    if slot.replace(value).is_some() {
        return Err(format!("{argument} was supplied more than once"));
    }
    Ok(())
}

fn required<T>(value: Option<T>, argument: &str) -> Result<T, String> {
    value.ok_or_else(|| format!("missing required argument {argument}"))
}

fn decode_hex_32(value: &str, label: &str) -> Result<[u8; 32], String> {
    let value = value.strip_prefix("sha256:").unwrap_or(value);
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!(
            "{label} must contain exactly 64 hexadecimal digits"
        ));
    }
    let mut decoded = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let pair = std::str::from_utf8(pair)
            .map_err(|error| format!("{label} is not valid UTF-8: {error}"))?;
        decoded[index] = u8::from_str_radix(pair, 16)
            .map_err(|error| format!("{label} contains invalid hexadecimal: {error}"))?;
    }
    Ok(decoded)
}

fn verify_runtime(options: &Options) -> Result<VerifiedRuntime, String> {
    let mut trust = RuntimeTrustStore::new();
    trust
        .add_key_bytes(options.key_id.clone(), options.public_key)
        .map_err(|error| error.to_string())?;
    let requirement = RuntimeRequirement {
        target_triple: options.target.clone(),
        architecture: options.architecture.clone(),
        python: exact_requirement(&options.python_version, "Python")?,
        rspice_api: exact_requirement(&options.api_version, "RSpice API")?,
    };
    let runtime = trust
        .verify(&options.runtime_root, &requirement)
        .map_err(|error| error.to_string())?;
    if !runtime.supports_environment(options.environment_digest) {
        return Err(
            "signed runtime does not contain the release Automation environment".to_owned(),
        );
    }
    if runtime.identity().architecture != options.architecture
        || runtime.identity().python_version != options.python_version
        || runtime.identity().rspice_api_version != options.api_version
    {
        return Err("verified runtime identity does not match release inputs".to_owned());
    }
    Ok(runtime)
}

fn exact_requirement(value: &str, label: &str) -> Result<VersionReq, String> {
    VersionReq::parse(&format!("={value}"))
        .map_err(|error| format!("{label} version is invalid: {error}"))
}

fn standard_limits(wall_time_ms: u64) -> ResourceLimits {
    ResourceLimits {
        wall_time_ms,
        cpu_time_ms: 30_000,
        memory_bytes: 512 * 1024 * 1024,
        output_bytes: 1024 * 1024,
        artifact_bytes: 1024 * 1024,
        max_tasks: 1,
        max_stack_depth: 2_000,
    }
}

fn source_snapshot(options: &Options, source: &str) -> SourceSnapshot {
    let entry_id = Uuid::new_v4();
    let lock_id = Uuid::new_v4();
    let permission_id = Uuid::new_v4();
    let lock = format!(
        "schema = \"rspice.environment-lock/v3\"\npython = \"={}\"\n",
        options.python_version
    );
    let permissions = "schema = \"rspice.permissions/v1\"\ncapabilities = []\n";
    let documents = vec![
        SourceDocument {
            document_id: entry_id,
            logical_path: "qualification/entry.py".to_owned(),
            revision: 1,
            role: DocumentRole::PythonEntry,
            read_only: false,
            source: source.to_owned(),
        },
        SourceDocument {
            document_id: lock_id,
            logical_path: "locks/exact.snapshot".to_owned(),
            revision: 1,
            role: DocumentRole::EnvironmentLock,
            read_only: true,
            source: lock,
        },
        SourceDocument {
            document_id: permission_id,
            logical_path: "policy/closed.rules".to_owned(),
            revision: 1,
            role: DocumentRole::PermissionManifest,
            read_only: true,
            source: permissions.to_owned(),
        },
    ];
    let mut closure = Sha256::new();
    for document in &documents {
        closure.update(document.logical_path.as_bytes());
        closure.update([0]);
        closure.update(document.source.as_bytes());
        closure.update([0xff]);
    }
    SourceSnapshot {
        project_id: Uuid::new_v4(),
        workspace_id: Uuid::new_v4(),
        workspace_revision: 1,
        closure_digest: Digest(closure.finalize().into()),
        environment_digest: options.environment_digest,
        permission_digest: Digest(Sha256::digest(permissions.as_bytes()).into()),
        entry_document_id: entry_id,
        selected_run_plan_document_id: None,
        python_requirement: format!("={}", options.python_version),
        api_requirement: format!("={}", options.api_version),
        browser_runtime_requirement: None,
        documents,
        capabilities: Vec::new(),
    }
}

fn launch_request(
    options: &Options,
    mode: LaunchMode,
    source: &str,
    limits: ResourceLimits,
) -> RequestEnvelope {
    RequestEnvelope {
        protocol: PROTOCOL_VERSION,
        request_id: 1,
        request: RuntimeRequest::Launch {
            mode,
            snapshot: Box::new(source_snapshot(options, source)),
            limits,
            breakpoints: Vec::new(),
            exception_policy: ExceptionPolicy::Uncaught,
        },
    }
}

fn qualified_worker(
    runtime: &VerifiedRuntime,
    label: &str,
) -> Result<(NativeWorker, ScratchDirectory, u64), String> {
    let scratch = ScratchDirectory::new(label)?;
    let mut worker = runtime
        .launch(scratch.path())
        .map_err(|error| error.to_string())?;
    let mut sequence = 0;
    let hello = next_event(&mut worker, &mut sequence, Instant::now() + STARTUP_TIMEOUT)?;
    match hello.event {
        RuntimeEvent::Hello { identity } if identity == *runtime.identity() => {}
        RuntimeEvent::Hello { .. } => {
            return Err("worker hello identity does not match its verified manifest".to_owned());
        }
        event => {
            return Err(format!(
                "worker emitted {event:?} before authenticated hello"
            ));
        }
    }
    if hello.request_id.is_some() || hello.session_id.is_some() {
        return Err(
            "initial worker hello unexpectedly claimed request/session identity".to_owned(),
        );
    }
    Ok((worker, scratch, sequence))
}

fn qualify_completed_launch(
    runtime: &VerifiedRuntime,
    options: &Options,
    mode: LaunchMode,
    source: &str,
    expected_output: Option<&str>,
    prove_single_limit_application: bool,
) -> Result<u128, String> {
    let (mut worker, _scratch, mut sequence) = qualified_worker(runtime, "completed")?;
    let limits = standard_limits(15_000);
    worker
        .apply_resource_limits(limits)
        .map_err(|error| error.to_string())?;
    if prove_single_limit_application {
        match worker.apply_resource_limits(limits) {
            Err(RuntimeError::ResourceLimitsAlreadyApplied) => {}
            Err(error) => return Err(format!("repeated resource limits returned {error}")),
            Ok(()) => return Err("worker accepted a second process-lifetime limit set".to_owned()),
        }
    }
    worker
        .arm_wall_time_limit(limits.wall_time_ms)
        .map_err(|error| error.to_string())?;
    let request = launch_request(options, mode, source, limits);
    let started = Instant::now();
    worker.send(&request).map_err(|error| error.to_string())?;

    let mut validating = false;
    let mut running = false;
    let mut output = String::new();
    loop {
        let event = next_event(
            &mut worker,
            &mut sequence,
            Instant::now() + OPERATION_TIMEOUT,
        )?;
        if event.request_id != Some(request.request_id) {
            return Err("launch event has the wrong request identity".to_owned());
        }
        match event.event {
            RuntimeEvent::State {
                state: RuntimeState::Validating,
                ..
            } => validating = true,
            RuntimeEvent::State {
                state: RuntimeState::Running,
                ..
            } => running = true,
            RuntimeEvent::Output { text, .. } => output.push_str(&text),
            RuntimeEvent::State {
                state: RuntimeState::Completed,
                ..
            } => break,
            RuntimeEvent::Diagnostic { diagnostic } => {
                return Err(format!(
                    "qualified source produced diagnostic {}: {}",
                    diagnostic.code, diagnostic.message
                ));
            }
            RuntimeEvent::WorkerFailed { code, message, .. } => {
                return Err(format!("worker failed {code}: {message}"));
            }
            RuntimeEvent::State {
                state: RuntimeState::Failed | RuntimeState::Cancelled | RuntimeState::Terminated,
                detail,
            } => return Err(format!("launch ended unexpectedly: {detail}")),
            _ => {}
        }
    }
    let elapsed = started.elapsed().as_millis();
    worker.clear_wall_time_limit();
    if !validating {
        return Err("worker launch never entered authoritative validation".to_owned());
    }
    if mode == LaunchMode::Run && !running {
        return Err("worker execution never entered the running state".to_owned());
    }
    if expected_output.is_some_and(|expected| output != expected) {
        return Err(format!(
            "worker output mismatch: expected {expected_output:?}, observed {output:?}"
        ));
    }
    shutdown_worker(&mut worker, &mut sequence, 2)?;
    Ok(elapsed)
}

fn qualify_hard_cancellation(runtime: &VerifiedRuntime, options: &Options) -> Result<u128, String> {
    let (mut worker, _scratch, mut sequence) = qualified_worker(runtime, "hard-cancel")?;
    let limits = standard_limits(30_000);
    worker
        .apply_resource_limits(limits)
        .map_err(|error| error.to_string())?;
    worker
        .arm_wall_time_limit(limits.wall_time_ms)
        .map_err(|error| error.to_string())?;
    let request = launch_request(options, LaunchMode::Run, "while True:\n    pass\n", limits);
    worker.send(&request).map_err(|error| error.to_string())?;
    wait_for_running(&mut worker, &mut sequence, request.request_id)?;

    let started = Instant::now();
    worker.terminate().map_err(|error| error.to_string())?;
    let status = wait_for_exit(&mut worker, Instant::now() + TERMINATION_TIMEOUT)?;
    let elapsed = started.elapsed().as_millis();
    if status.success() {
        return Err(
            "hard-cancelled Python worker exited successfully instead of being killed".to_owned(),
        );
    }
    if elapsed > TERMINATION_TIMEOUT.as_millis() {
        return Err(format!("hard cancellation took {elapsed} ms"));
    }
    Ok(elapsed)
}

fn qualify_watchdog_termination(
    runtime: &VerifiedRuntime,
    options: &Options,
) -> Result<u128, String> {
    let (mut worker, _scratch, mut sequence) = qualified_worker(runtime, "watchdog")?;
    let limits = standard_limits(750);
    worker
        .apply_resource_limits(limits)
        .map_err(|error| error.to_string())?;
    worker
        .arm_wall_time_limit(limits.wall_time_ms)
        .map_err(|error| error.to_string())?;
    let request = launch_request(options, LaunchMode::Run, "while True:\n    pass\n", limits);
    let started = Instant::now();
    worker.send(&request).map_err(|error| error.to_string())?;
    wait_for_running(&mut worker, &mut sequence, request.request_id)?;
    let status = wait_for_exit(&mut worker, Instant::now() + TERMINATION_TIMEOUT)?;
    let elapsed = started.elapsed().as_millis();
    worker.clear_wall_time_limit();
    if status.success() {
        return Err(
            "wall-limited Python worker exited successfully instead of being killed".to_owned(),
        );
    }
    if elapsed > TERMINATION_TIMEOUT.as_millis() {
        return Err(format!("wall watchdog termination took {elapsed} ms"));
    }
    Ok(elapsed)
}

fn wait_for_running(
    worker: &mut NativeWorker,
    sequence: &mut u64,
    request_id: u64,
) -> Result<(), String> {
    let deadline = Instant::now() + OPERATION_TIMEOUT;
    loop {
        let event = next_event(worker, sequence, deadline)?;
        if event.request_id != Some(request_id) {
            return Err("infinite-loop launch event has the wrong request identity".to_owned());
        }
        match event.event {
            RuntimeEvent::State {
                state: RuntimeState::Running,
                ..
            } => return Ok(()),
            RuntimeEvent::WorkerFailed { code, message, .. } => {
                return Err(format!(
                    "worker failed before cancellation {code}: {message}"
                ));
            }
            RuntimeEvent::State {
                state: RuntimeState::Failed | RuntimeState::Completed,
                detail,
            } => {
                return Err(format!(
                    "infinite-loop source ended before cancellation: {detail}"
                ));
            }
            _ => {}
        }
    }
}

fn shutdown_worker(
    worker: &mut NativeWorker,
    sequence: &mut u64,
    request_id: u64,
) -> Result<(), String> {
    worker
        .send(&RequestEnvelope {
            protocol: PROTOCOL_VERSION,
            request_id,
            request: RuntimeRequest::Shutdown,
        })
        .map_err(|error| error.to_string())?;
    let deadline = Instant::now() + TERMINATION_TIMEOUT;
    loop {
        let event = next_event(worker, sequence, deadline)?;
        if event.request_id == Some(request_id)
            && matches!(event.event, RuntimeEvent::Terminated { .. })
        {
            break;
        }
    }
    let status = wait_for_exit(worker, deadline)?;
    if !status.success() {
        return Err(format!("worker shutdown exited with {status}"));
    }
    Ok(())
}

fn next_event(
    worker: &mut NativeWorker,
    sequence: &mut u64,
    deadline: Instant,
) -> Result<EventEnvelope, String> {
    loop {
        if let Some(event) = worker.try_event().map_err(|error| error.to_string())? {
            if event.sequence <= *sequence {
                return Err(format!(
                    "worker event sequence regressed from {} to {}",
                    *sequence, event.sequence
                ));
            }
            *sequence = event.sequence;
            return Ok(event);
        }
        if let Some(status) = worker.try_wait().map_err(|error| error.to_string())? {
            let stderr = worker.captured_stderr().unwrap_or_default();
            return Err(format!(
                "worker exited with {status} before the required event{}",
                if stderr.trim().is_empty() {
                    String::new()
                } else {
                    format!(": {}", stderr.trim())
                }
            ));
        }
        if Instant::now() >= deadline {
            return Err("timed out waiting for a managed-runtime event".to_owned());
        }
        thread::sleep(EVENT_POLL_INTERVAL);
    }
}

fn wait_for_exit(worker: &mut NativeWorker, deadline: Instant) -> Result<ExitStatus, String> {
    loop {
        if let Some(status) = worker.try_wait().map_err(|error| error.to_string())? {
            return Ok(status);
        }
        if Instant::now() >= deadline {
            return Err("timed out waiting for managed Python to exit".to_owned());
        }
        thread::sleep(EVENT_POLL_INTERVAL);
    }
}

fn hex(bytes: &[u8; 32]) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(64);
    for byte in bytes {
        let _ = write!(encoded, "{byte:02x}");
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arguments() -> Vec<String> {
        vec![
            "--runtime-root",
            "qualified-runtime",
            "--key-id",
            "qualification-key",
            "--public-key-hex",
            &"ab".repeat(32),
            "--target",
            "x86_64-pc-windows-msvc",
            "--architecture",
            "x86_64",
            "--python-version",
            "3.14.6",
            "--api-version",
            "1.0.0",
            "--environment-digest",
            &"51".repeat(32),
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }

    #[test]
    fn release_options_are_exact_and_duplicate_or_unknown_inputs_fail_closed() {
        let options = parse_options(arguments().into_iter()).expect("qualified release options");
        assert_eq!(options.python_version, "3.14.6");
        assert_eq!(options.public_key, [0xab; 32]);
        assert_eq!(options.environment_digest, Digest([0x51; 32]));

        let mut duplicate = arguments();
        duplicate.extend(["--target".to_owned(), "other".to_owned()]);
        assert!(parse_options(duplicate.into_iter()).is_err());
        assert!(parse_options(["--unknown".to_owned(), "value".to_owned()].into_iter()).is_err());
    }

    #[test]
    fn qualification_snapshot_is_protocol_valid_and_not_demo_filename_bound() {
        let options = parse_options(arguments().into_iter()).expect("qualified release options");
        let snapshot = source_snapshot(&options, "print('qualification')\n");
        snapshot.validate().expect("valid qualification snapshot");
        let paths = snapshot
            .documents
            .iter()
            .map(|document| document.logical_path.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            paths,
            [
                "qualification/entry.py",
                "locks/exact.snapshot",
                "policy/closed.rules"
            ]
        );
        assert!(!paths.contains(&"characterize.py"));
        assert!(!paths.contains(&"requirements.lock"));
        assert!(!paths.contains(&"permissions.toml"));
    }
}
