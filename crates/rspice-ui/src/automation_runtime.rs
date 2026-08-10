//! Native managed-runtime ownership for the Automation workspace.
//!
//! Runtime verification is performed off the EGUI thread. The only discovery
//! path is relative to the installed RSpice executable and anchored by the
//! Ed25519 release key compiled into this application build.

use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
};

use rspice_automation_protocol::{EventEnvelope, RequestEnvelope};
use rspice_automation_runtime::{
    NativeWorker, RuntimeRequirement, RuntimeTrustStore, VerifiedRuntime,
};
use semver::{Version, VersionReq};

const RUNTIME_RELATIVE_PATH: &str = "runtimes/python";

pub(crate) struct NativeAutomationRuntime {
    discovery: RuntimeDiscovery,
    worker: Option<NativeWorker>,
    next_request_id: u64,
    last_event_sequence: u64,
    active_limits: Option<ActiveRuntimeLimits>,
}

#[derive(Clone, Copy)]
struct ActiveRuntimeLimits {
    request_id: u64,
    output_bytes: u64,
    artifact_bytes: u64,
    observed_output_bytes: u64,
    observed_artifact_bytes: u64,
}

enum RuntimeDiscovery {
    Pending(Receiver<Result<VerifiedRuntime, String>>),
    Available(Box<VerifiedRuntime>),
    Unavailable(String),
}

impl NativeAutomationRuntime {
    pub(crate) fn discover() -> Self {
        let key_id = env!("RSPICE_AUTOMATION_RUNTIME_KEY_ID");
        let key_hex = env!("RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX");
        if key_id.is_empty() || key_hex.is_empty() {
            return Self {
                discovery: RuntimeDiscovery::Unavailable(
                    "this application build has no managed-runtime release trust key".to_owned(),
                ),
                worker: None,
                next_request_id: 1,
                last_event_sequence: 0,
                active_limits: None,
            };
        }
        let key = match decode_key(key_hex) {
            Ok(key) => key,
            Err(error) => {
                return Self {
                    discovery: RuntimeDiscovery::Unavailable(error),
                    worker: None,
                    next_request_id: 1,
                    last_event_sequence: 0,
                    active_limits: None,
                };
            }
        };
        let root = match installed_runtime_root() {
            Ok(root) => root,
            Err(error) => {
                return Self {
                    discovery: RuntimeDiscovery::Unavailable(error),
                    worker: None,
                    next_request_id: 1,
                    last_event_sequence: 0,
                    active_limits: None,
                };
            }
        };
        let key_id = key_id.to_owned();
        let (sender, receiver) = mpsc::sync_channel(1);
        let spawn = thread::Builder::new()
            .name("rspice-python-runtime-verifier".to_owned())
            .spawn(move || {
                let mut trust = RuntimeTrustStore::new();
                let result = runtime_requirement().and_then(|requirement| {
                    trust
                        .add_key_bytes(key_id, key)
                        .map_err(|error| error.to_string())?;
                    trust
                        .verify(root, &requirement)
                        .map_err(|error| error.to_string())
                });
                let _ = sender.send(result);
            });
        let discovery = match spawn {
            Ok(_) => RuntimeDiscovery::Pending(receiver),
            Err(error) => RuntimeDiscovery::Unavailable(format!(
                "could not start managed-runtime verification: {error}"
            )),
        };
        Self {
            discovery,
            worker: None,
            next_request_id: 1,
            last_event_sequence: 0,
            active_limits: None,
        }
    }

    pub(crate) fn poll_discovery(&mut self) {
        let RuntimeDiscovery::Pending(receiver) = &self.discovery else {
            return;
        };
        let update = match receiver.try_recv() {
            Ok(Ok(runtime)) => Some(RuntimeDiscovery::Available(Box::new(runtime))),
            Ok(Err(error)) => Some(RuntimeDiscovery::Unavailable(error)),
            Err(TryRecvError::Disconnected) => Some(RuntimeDiscovery::Unavailable(
                "managed-runtime verification ended without a result".to_owned(),
            )),
            Err(TryRecvError::Empty) => None,
        };
        if let Some(update) = update {
            self.discovery = update;
        }
    }

    pub(crate) fn availability_reason(&self) -> Option<&str> {
        match &self.discovery {
            RuntimeDiscovery::Pending(_) => {
                Some("managed Python runtime verification is in progress")
            }
            RuntimeDiscovery::Available(_) => None,
            RuntimeDiscovery::Unavailable(reason) => Some(reason),
        }
    }

    pub(crate) fn ensure_worker(&mut self, scratch: PathBuf) -> Result<(), String> {
        self.poll_discovery();
        if self.worker.is_some() {
            return Ok(());
        }
        let RuntimeDiscovery::Available(runtime) = &self.discovery else {
            return Err(self
                .availability_reason()
                .unwrap_or("managed Python runtime is unavailable")
                .to_owned());
        };
        self.worker = Some(runtime.launch(scratch).map_err(|error| error.to_string())?);
        self.last_event_sequence = 0;
        Ok(())
    }

    pub(crate) fn send(&mut self, request: &RequestEnvelope) -> Result<(), String> {
        let worker = self
            .worker
            .as_mut()
            .ok_or_else(|| "managed Python worker is not running".to_owned())?;
        worker.send(request).map_err(|error| error.to_string())
    }

    pub(crate) fn send_request(
        &mut self,
        request: rspice_automation_protocol::RuntimeRequest,
    ) -> Result<u64, String> {
        if let rspice_automation_protocol::RuntimeRequest::Launch { snapshot, .. } = &request {
            self.validate_launch_snapshot(snapshot)?;
            if self.active_limits.is_some() {
                return Err("a managed Python launch is already active".to_owned());
            }
        }
        let limits = match &request {
            rspice_automation_protocol::RuntimeRequest::Launch { limits, .. } => Some(*limits),
            _ => None,
        };
        let request_id = self.next_request_id;
        let next_request_id = self
            .next_request_id
            .checked_add(1)
            .ok_or_else(|| "managed Python request identity space is exhausted".to_owned())?;
        let envelope = RequestEnvelope {
            protocol: rspice_automation_protocol::PROTOCOL_VERSION,
            request_id,
            request,
        };
        envelope.validate().map_err(|error| error.to_string())?;
        if let Some(limits) = limits {
            let configured = self
                .worker
                .as_mut()
                .ok_or_else(|| "managed Python worker is not running".to_owned())
                .and_then(|worker| {
                    worker
                        .apply_resource_limits(limits)
                        .map_err(|error| error.to_string())?;
                    worker
                        .arm_wall_time_limit(limits.wall_time_ms)
                        .map_err(|error| error.to_string())
                });
            if let Err(error) = configured {
                // Never retain a worker whose OS resource boundary was only
                // partially installed. A new launch must start in a fresh,
                // verified process and configure the complete limit set.
                let _ = self.terminate();
                return Err(error);
            }
            self.active_limits = Some(ActiveRuntimeLimits {
                request_id,
                output_bytes: limits.output_bytes,
                artifact_bytes: limits.artifact_bytes,
                observed_output_bytes: 0,
                observed_artifact_bytes: 0,
            });
        }
        if let Err(error) = self.send(&envelope) {
            // A failed framed write leaves the worker transport state
            // uncertain. Dispose of the whole isolated process so a failed
            // control request cannot weaken the active launch boundary.
            let _ = self.terminate();
            return Err(error);
        }
        self.next_request_id = next_request_id;
        Ok(request_id)
    }

    fn validate_launch_snapshot(
        &self,
        snapshot: &rspice_automation_protocol::SourceSnapshot,
    ) -> Result<(), String> {
        snapshot.validate().map_err(|error| error.to_string())?;
        let RuntimeDiscovery::Available(runtime) = &self.discovery else {
            return Err("the signed managed Python runtime is unavailable".to_owned());
        };
        require_version(
            "Python",
            &runtime.identity().python_version,
            &snapshot.python_requirement,
        )?;
        require_version(
            "RSpice API",
            &runtime.identity().rspice_api_version,
            &snapshot.api_requirement,
        )?;
        if !runtime.supports_environment(snapshot.environment_digest) {
            return Err(
                "the selected Python environment is not installed in this signed RSpice runtime"
                    .to_owned(),
            );
        }
        Ok(())
    }

    pub(crate) fn poll_events(&mut self) -> Vec<Result<EventEnvelope, String>> {
        let mut events = Vec::new();
        let Some(worker) = self.worker.as_mut() else {
            return events;
        };
        for _ in 0..256 {
            match worker.try_event() {
                Ok(Some(event)) => {
                    if event.sequence <= self.last_event_sequence {
                        events.push(Err(format!(
                            "managed Python worker event sequence regressed from {} to {}",
                            self.last_event_sequence, event.sequence
                        )));
                        break;
                    }
                    self.last_event_sequence = event.sequence;
                    if let rspice_automation_protocol::RuntimeEvent::Hello { identity } =
                        &event.event
                    {
                        let expected = match &self.discovery {
                            RuntimeDiscovery::Available(runtime) => runtime.identity(),
                            _ => {
                                events.push(Err(
                                    "managed Python worker identified itself without a verified runtime"
                                        .to_owned(),
                                ));
                                break;
                            }
                        };
                        if identity != expected {
                            events.push(Err(
                                "managed Python worker identity does not match the signed runtime manifest"
                                    .to_owned(),
                            ));
                            break;
                        }
                    }
                    if let Some(limits) = self
                        .active_limits
                        .as_mut()
                        .filter(|limits| event.request_id == Some(limits.request_id))
                    {
                        match &event.event {
                            rspice_automation_protocol::RuntimeEvent::Output { text, .. } => {
                                limits.observed_output_bytes = limits
                                    .observed_output_bytes
                                    .saturating_add(text.len() as u64);
                                if limits.observed_output_bytes > limits.output_bytes {
                                    events.push(Err(format!(
                                        "managed Python exceeded its {}-byte output limit",
                                        limits.output_bytes
                                    )));
                                    break;
                                }
                            }
                            rspice_automation_protocol::RuntimeEvent::ArtifactPublished {
                                bytes,
                                ..
                            } => {
                                limits.observed_artifact_bytes =
                                    limits.observed_artifact_bytes.saturating_add(*bytes);
                                if limits.observed_artifact_bytes > limits.artifact_bytes {
                                    events.push(Err(format!(
                                        "managed Python exceeded its {}-byte artifact limit",
                                        limits.artifact_bytes
                                    )));
                                    break;
                                }
                            }
                            rspice_automation_protocol::RuntimeEvent::State {
                                state:
                                    rspice_automation_protocol::RuntimeState::Cancelled
                                    | rspice_automation_protocol::RuntimeState::Completed
                                    | rspice_automation_protocol::RuntimeState::Failed
                                    | rspice_automation_protocol::RuntimeState::Terminated,
                                ..
                            } => {
                                worker.clear_wall_time_limit();
                                self.active_limits = None;
                            }
                            _ => {}
                        }
                    }
                    events.push(Ok(event));
                }
                Ok(None) => break,
                Err(error) => {
                    events.push(Err(error.to_string()));
                    break;
                }
            }
        }
        match worker.try_wait() {
            Ok(Some(status)) => {
                let stderr = worker.captured_stderr().unwrap_or_default();
                events.push(Err(format!(
                    "managed Python worker exited with {status}{}",
                    if stderr.trim().is_empty() {
                        String::new()
                    } else {
                        format!(": {}", stderr.trim())
                    }
                )));
                self.worker = None;
                self.active_limits = None;
            }
            Ok(None) => {}
            Err(error) => events.push(Err(error.to_string())),
        }
        events
    }

    pub(crate) fn terminate(&mut self) -> Result<(), String> {
        let result = self.worker.take().map_or(Ok(()), |mut worker| {
            worker.terminate().map_err(|error| error.to_string())
        });
        self.active_limits = None;
        result
    }
}

fn installed_runtime_root() -> Result<PathBuf, String> {
    let executable = std::env::current_exe()
        .map_err(|error| format!("could not resolve the RSpice executable: {error}"))?;
    let directory = executable
        .parent()
        .ok_or_else(|| "the RSpice executable has no installation directory".to_owned())?;
    Ok(directory.join(RUNTIME_RELATIVE_PATH))
}

fn runtime_requirement() -> Result<RuntimeRequirement, String> {
    Ok(RuntimeRequirement {
        target_triple: env!("RSPICE_BUILD_TARGET").to_owned(),
        architecture: env!("RSPICE_BUILD_ARCH").to_owned(),
        python: VersionReq::parse(">=3.14.0,<3.15.0")
            .map_err(|error| format!("the compiled Python requirement is invalid: {error}"))?,
        rspice_api: VersionReq::parse(">=1.0.0,<2.0.0")
            .map_err(|error| format!("the compiled RSpice API requirement is invalid: {error}"))?,
    })
}

fn require_version(label: &str, actual: &str, requirement: &str) -> Result<(), String> {
    let actual = Version::parse(actual)
        .map_err(|error| format!("the signed {label} version is invalid: {error}"))?;
    let requirement = VersionReq::parse(requirement)
        .map_err(|error| format!("the governed {label} requirement is invalid: {error}"))?;
    if !requirement.matches(&actual) {
        return Err(format!(
            "signed {label} {actual} does not satisfy governed requirement {requirement}"
        ));
    }
    Ok(())
}

fn decode_key(value: &str) -> Result<[u8; 32], String> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("the compiled managed-runtime trust key is malformed".to_owned());
    }
    let mut key = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let pair = std::str::from_utf8(pair).map_err(|_| "trust key is not UTF-8".to_owned())?;
        key[index] = u8::from_str_radix(pair, 16)
            .map_err(|_| "trust key contains invalid hexadecimal".to_owned())?;
    }
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiled_key_decoder_is_exact_and_never_path_discovers_python() {
        assert_eq!(decode_key(&"ab".repeat(32)).unwrap(), [0xab; 32]);
        assert!(decode_key("python").is_err());
        assert_eq!(RUNTIME_RELATIVE_PATH, "runtimes/python");
    }

    #[test]
    fn compiled_runtime_requirement_is_valid_and_production_has_no_panic_shortcuts() {
        let requirement = runtime_requirement().expect("compiled runtime requirement");
        assert!(requirement.python.matches(&Version::new(3, 14, 0)));
        assert!(requirement.rspice_api.matches(&Version::new(1, 0, 0)));

        let production = include_str!("automation_runtime.rs")
            .split("\n#[cfg(test)]\nmod tests {")
            .next()
            .expect("production source");
        for shortcut in [
            ".expect(",
            ".unwrap(",
            "panic!(",
            "unreachable!(",
            "todo!(",
            "unimplemented!(",
        ] {
            assert!(
                !production.contains(shortcut),
                "production code must not contain {shortcut}"
            );
        }
        for required in [
            "a managed Python launch is already active",
            "envelope.validate()",
            "partially installed",
            "let _ = self.terminate();",
            "self.active_limits = None;",
        ] {
            assert!(
                production.contains(required),
                "managed runtime lifecycle must retain {required}"
            );
        }
    }
}
