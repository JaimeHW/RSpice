//! Verification and launch support for RSpice-owned native Python runtimes.
//!
//! This crate deliberately has no system-interpreter discovery. A caller
//! supplies an application-local runtime directory and a release trust store;
//! every executable input is signed, enumerated, and content verified before
//! the Python worker can be launched.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufReader, Read},
    path::{Component, Path, PathBuf},
    process::{Child, ChildStderr, ChildStdin, Command, Stdio},
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, RecvTimeoutError, SyncSender, TryRecvError},
    },
    thread,
    time::Duration,
};

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use rspice_automation_protocol::{
    EventEnvelope, PROTOCOL_VERSION, RequestEnvelope, ResourceLimits, RuntimeIdentity,
    RuntimePlatform, native_codec,
};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

pub const MANIFEST_FILE: &str = "runtime-manifest.json";
pub const SIGNATURE_FILE: &str = "runtime-manifest.ed25519.json";
const MANIFEST_LIMIT: u64 = 2 * 1024 * 1024;
const SIGNATURE_LIMIT: u64 = 16 * 1024;
const STDERR_CAPTURE_LIMIT: usize = 256 * 1024;
const RUNTIME_DIGEST_DOMAIN: &[u8] = b"rspice-managed-python-runtime/v1\0";
const MANAGED_PYTHON_MAJOR: u64 = 3;
const MANAGED_PYTHON_MINOR: u64 = 14;
const MANAGED_PYTHON_ABI: &str = "cp314";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagedRuntimeManifest {
    pub schema: String,
    pub runtime_build: String,
    pub target_triple: String,
    pub architecture: String,
    pub python_version: String,
    pub python_abi: String,
    pub rspice_api_version: String,
    /// Content-addressed Python environments installed in this signed
    /// runtime. A project lock must name one of these exact digests.
    pub environment_digests_sha256: Vec<String>,
    pub protocol_major: u16,
    pub protocol_minor: u16,
    pub python_executable: String,
    pub worker_bootstrap: String,
    pub runtime_digest_sha256: String,
    pub files: Vec<RuntimeFile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFile {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
    pub executable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestSignature {
    key_id: String,
    signature_hex: String,
}

#[derive(Clone, Debug)]
pub struct RuntimeRequirement {
    pub target_triple: String,
    pub architecture: String,
    pub python: VersionReq,
    pub rspice_api: VersionReq,
}

#[derive(Clone, Default)]
pub struct RuntimeTrustStore {
    keys: BTreeMap<String, VerifyingKey>,
}

impl RuntimeTrustStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_key(
        &mut self,
        key_id: impl Into<String>,
        key: VerifyingKey,
    ) -> Result<(), RuntimeError> {
        let key_id = key_id.into();
        validate_short_text("trust key ID", &key_id)?;
        if self.keys.insert(key_id.clone(), key).is_some() {
            return Err(RuntimeError::DuplicateTrustKey { key_id });
        }
        Ok(())
    }

    pub fn add_key_bytes(
        &mut self,
        key_id: impl Into<String>,
        key: [u8; 32],
    ) -> Result<(), RuntimeError> {
        let key =
            VerifyingKey::from_bytes(&key).map_err(|_| RuntimeError::InvalidTrustKeyEncoding)?;
        self.add_key(key_id, key)
    }

    pub fn verify(
        &self,
        runtime_root: impl AsRef<Path>,
        requirement: &RuntimeRequirement,
    ) -> Result<VerifiedRuntime, RuntimeError> {
        let runtime_root = runtime_root.as_ref();
        let root_metadata =
            fs::symlink_metadata(runtime_root).map_err(|source| RuntimeError::OpenRuntimeRoot {
                path: runtime_root.to_path_buf(),
                source,
            })?;
        if !root_metadata.is_dir() || root_metadata.file_type().is_symlink() {
            return Err(RuntimeError::UnsafeRuntimeRoot {
                path: runtime_root.to_path_buf(),
            });
        }
        let canonical_root =
            fs::canonicalize(runtime_root).map_err(|source| RuntimeError::OpenRuntimeRoot {
                path: runtime_root.to_path_buf(),
                source,
            })?;
        let manifest_path = canonical_root.join(MANIFEST_FILE);
        let signature_path = canonical_root.join(SIGNATURE_FILE);
        let manifest_bytes = read_bounded(&manifest_path, MANIFEST_LIMIT)?;
        let signature_bytes = read_bounded(&signature_path, SIGNATURE_LIMIT)?;
        let signature: ManifestSignature = serde_json::from_slice(&signature_bytes)
            .map_err(RuntimeError::InvalidSignatureDocument)?;
        validate_short_text("runtime signing key ID", &signature.key_id)?;
        let key =
            self.keys
                .get(&signature.key_id)
                .ok_or_else(|| RuntimeError::UntrustedSigningKey {
                    key_id: signature.key_id.clone(),
                })?;
        let signature_raw = decode_hex_exact::<64>(&signature.signature_hex)
            .map_err(|reason| RuntimeError::InvalidSignatureEncoding { reason })?;
        let signature = Signature::from_bytes(&signature_raw);
        key.verify(&manifest_bytes, &signature).map_err(|_| {
            RuntimeError::InvalidManifestSignature {
                key_id: signature_key_id(&signature_bytes).unwrap_or_default(),
            }
        })?;

        let manifest: ManagedRuntimeManifest =
            serde_json::from_slice(&manifest_bytes).map_err(RuntimeError::InvalidManifest)?;
        validate_manifest(&manifest, requirement)?;
        let expected_digest = decode_hex_exact::<32>(&manifest.runtime_digest_sha256)
            .map_err(|reason| RuntimeError::InvalidRuntimeDigest { reason })?;
        let (actual_digest, listed_paths) = verify_files(&canonical_root, &manifest)?;
        if actual_digest != expected_digest {
            return Err(RuntimeError::RuntimeDigestMismatch {
                expected: manifest.runtime_digest_sha256.clone(),
                actual: encode_hex(&actual_digest),
            });
        }
        reject_unlisted_files(&canonical_root, &listed_paths)?;

        let python_executable =
            verified_member_path(&canonical_root, &manifest.python_executable, &listed_paths)?;
        let worker_bootstrap =
            verified_member_path(&canonical_root, &manifest.worker_bootstrap, &listed_paths)?;
        let platform = match std::env::consts::OS {
            "windows" => RuntimePlatform::NativeWindows,
            "macos" => RuntimePlatform::NativeMacOs,
            _ => RuntimePlatform::NativeLinux,
        };
        let identity = RuntimeIdentity {
            managed: true,
            platform,
            architecture: manifest.architecture.clone(),
            runtime_build: manifest.runtime_build.clone(),
            runtime_digest: rspice_automation_protocol::Digest(actual_digest),
            python_version: manifest.python_version.clone(),
            python_abi: manifest.python_abi.clone(),
            rspice_api_version: manifest.rspice_api_version.clone(),
            protocol: PROTOCOL_VERSION,
        };
        identity
            .validate()
            .map_err(|error| RuntimeError::InvalidIdentity(error.to_string()))?;

        Ok(VerifiedRuntime {
            root: canonical_root,
            python_executable,
            worker_bootstrap,
            manifest,
            identity,
        })
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedRuntime {
    root: PathBuf,
    python_executable: PathBuf,
    worker_bootstrap: PathBuf,
    manifest: ManagedRuntimeManifest,
    identity: RuntimeIdentity,
}

impl VerifiedRuntime {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn manifest(&self) -> &ManagedRuntimeManifest {
        &self.manifest
    }

    pub fn identity(&self) -> &RuntimeIdentity {
        &self.identity
    }

    pub fn supports_environment(&self, environment: rspice_automation_protocol::Digest) -> bool {
        let encoded = encode_hex(&environment.0);
        self.manifest
            .environment_digests_sha256
            .iter()
            .any(|candidate| candidate == &encoded)
    }

    /// Launches only the verified application-local executable. No PATH
    /// lookup, shell, user site, startup hook, or ambient working directory is
    /// involved.
    pub fn launch(
        &self,
        scratch_directory: impl AsRef<Path>,
    ) -> Result<NativeWorker, RuntimeError> {
        let scratch = prepare_scratch(scratch_directory.as_ref())?;
        let mut command = Command::new(&self.python_executable);
        command
            .arg("-I")
            .arg("-S")
            .arg("-B")
            .arg("-u")
            .arg(&self.worker_bootstrap)
            .arg("--rspice-protocol-stdio")
            .current_dir(&scratch)
            .env_clear()
            .env("PYTHONUTF8", "1")
            .env("PYTHONIOENCODING", "utf-8")
            .env("RSPICE_RUNTIME_ROOT", &self.root)
            .env(
                "RSPICE_RUNTIME_DIGEST",
                &self.manifest.runtime_digest_sha256,
            )
            .env("RSPICE_RUNTIME_ARCH", &self.manifest.architecture)
            .env("RSPICE_RUNTIME_BUILD", &self.manifest.runtime_build)
            .env("RSPICE_API_VERSION", &self.manifest.rspice_api_version)
            .env("TMP", &scratch)
            .env("TEMP", &scratch)
            .env("TMPDIR", &scratch)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt as _;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            command.creation_flags(CREATE_NO_WINDOW);
        }
        let mut child = command.spawn().map_err(|source| RuntimeError::Launch {
            executable: self.python_executable.clone(),
            source,
        })?;
        let stdin = child
            .stdin
            .take()
            .ok_or(RuntimeError::MissingWorkerPipe("stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or(RuntimeError::MissingWorkerPipe("stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or(RuntimeError::MissingWorkerPipe("stderr"))?;
        let (events_tx, events_rx) = mpsc::sync_channel(256);
        let event_reader = thread::Builder::new()
            .name("rspice-python-events".to_owned())
            .spawn(move || {
                let mut reader = BufReader::new(stdout);
                loop {
                    match native_codec::read_frame::<_, EventEnvelope>(&mut reader) {
                        Ok(Some(event)) => {
                            if events_tx.send(Ok(event)).is_err() {
                                break;
                            }
                        }
                        Ok(None) => break,
                        Err(error) => {
                            let _ = events_tx.send(Err(error.to_string()));
                            break;
                        }
                    }
                }
            })
            .map_err(RuntimeError::ReaderThread)?;
        let (stderr_tx, stderr_rx) = mpsc::sync_channel(1);
        let stderr_reader = spawn_stderr_reader(stderr, stderr_tx)?;
        Ok(NativeWorker {
            child: Arc::new(Mutex::new(child)),
            stdin,
            events: events_rx,
            stderr: stderr_rx,
            event_reader: Some(event_reader),
            stderr_reader: Some(stderr_reader),
            wall_limit: None,
            resource_limits_applied: false,
            #[cfg(windows)]
            job: None,
        })
    }
}

pub struct NativeWorker {
    child: Arc<Mutex<Child>>,
    stdin: ChildStdin,
    events: Receiver<Result<EventEnvelope, String>>,
    stderr: Receiver<String>,
    event_reader: Option<thread::JoinHandle<()>>,
    stderr_reader: Option<thread::JoinHandle<()>>,
    wall_limit: Option<(SyncSender<()>, thread::JoinHandle<()>)>,
    resource_limits_applied: bool,
    #[cfg(windows)]
    job: Option<WindowsJob>,
}

#[cfg(windows)]
struct WindowsJob(windows_sys::Win32::Foundation::HANDLE);

#[cfg(windows)]
impl WindowsJob {
    fn assign(child: &Arc<Mutex<Child>>, limits: ResourceLimits) -> Result<Self, RuntimeError> {
        use std::{ffi::c_void, mem::size_of, os::windows::io::AsRawHandle as _};
        use windows_sys::Win32::{
            Foundation::{CloseHandle, GetLastError},
            System::JobObjects::{
                AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_ACTIVE_PROCESS,
                JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOB_OBJECT_LIMIT_PROCESS_MEMORY,
                JOB_OBJECT_LIMIT_PROCESS_TIME, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
                JobObjectExtendedLimitInformation, SetInformationJobObject,
            },
        };

        let handle = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if handle.is_null() {
            return Err(RuntimeError::WindowsJob {
                operation: "create",
                code: unsafe { GetLastError() },
            });
        }
        let mut information = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        information.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
            | JOB_OBJECT_LIMIT_PROCESS_MEMORY
            | JOB_OBJECT_LIMIT_PROCESS_TIME
            | JOB_OBJECT_LIMIT_ACTIVE_PROCESS;
        // Host-side task concurrency is brokered; the Python sandbox itself
        // never receives authority to create a child process.
        information.BasicLimitInformation.ActiveProcessLimit = 1;
        information.BasicLimitInformation.PerProcessUserTimeLimit =
            i64::try_from(limits.cpu_time_ms.saturating_mul(10_000)).unwrap_or(i64::MAX);
        information.ProcessMemoryLimit = usize::try_from(limits.memory_bytes).unwrap_or(usize::MAX);
        let information_size = match u32::try_from(size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>())
        {
            Ok(bytes) => bytes,
            Err(_) => {
                unsafe { CloseHandle(handle) };
                return Err(RuntimeError::WindowsJobInformationSize {
                    bytes: size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>(),
                });
            }
        };
        let configured = unsafe {
            SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                (&raw const information).cast::<c_void>(),
                information_size,
            )
        };
        if configured == 0 {
            let code = unsafe { GetLastError() };
            unsafe { CloseHandle(handle) };
            return Err(RuntimeError::WindowsJob {
                operation: "configure",
                code,
            });
        }
        let process = child.lock().map_err(|_| RuntimeError::WorkerLockPoisoned)?;
        let assigned = unsafe { AssignProcessToJobObject(handle, process.as_raw_handle()) };
        drop(process);
        if assigned == 0 {
            let code = unsafe { GetLastError() };
            unsafe { CloseHandle(handle) };
            return Err(RuntimeError::WindowsJob {
                operation: "assign",
                code,
            });
        }
        Ok(Self(handle))
    }
}

#[cfg(windows)]
impl Drop for WindowsJob {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.0);
        }
    }
}

impl NativeWorker {
    pub fn process_id(&self) -> u32 {
        self.child.lock().map_or(0, |child| child.id())
    }

    /// Arms a process-owning watchdog for one launch request. Unlike UI-frame
    /// polling, this remains effective if user Python loops forever or the UI
    /// is not repainting. A new launch replaces the prior deadline.
    pub fn arm_wall_time_limit(&mut self, wall_time_ms: u64) -> Result<(), RuntimeError> {
        self.clear_wall_time_limit();
        let child = Arc::clone(&self.child);
        let (cancel_tx, cancel_rx) = mpsc::sync_channel(1);
        let watchdog = thread::Builder::new()
            .name("rspice-python-wall-limit".to_owned())
            .spawn(move || {
                if matches!(
                    cancel_rx.recv_timeout(Duration::from_millis(wall_time_ms)),
                    Err(RecvTimeoutError::Timeout)
                ) && let Ok(mut child) = child.lock()
                    && child.try_wait().ok().flatten().is_none()
                {
                    let _ = child.kill();
                }
            })
            .map_err(RuntimeError::WatchdogThread)?;
        self.wall_limit = Some((cancel_tx, watchdog));
        Ok(())
    }

    /// Applies OS-enforced CPU, memory, process-count, and kill-on-close limits
    /// before any untrusted source is sent to the worker. POSIX workers apply
    /// the equivalent rlimits inside their trusted bootstrap.
    pub fn apply_resource_limits(&mut self, limits: ResourceLimits) -> Result<(), RuntimeError> {
        if self.resource_limits_applied {
            return Err(RuntimeError::ResourceLimitsAlreadyApplied);
        }
        #[cfg(windows)]
        {
            self.job = Some(WindowsJob::assign(&self.child, limits)?);
        }
        #[cfg(not(windows))]
        {
            let _ = limits;
        }
        self.resource_limits_applied = true;
        Ok(())
    }

    pub fn clear_wall_time_limit(&mut self) {
        if let Some((cancel, watchdog)) = self.wall_limit.take() {
            let _ = cancel.try_send(());
            let _ = watchdog.join();
        }
    }

    pub fn send(&mut self, request: &RequestEnvelope) -> Result<(), RuntimeError> {
        request
            .validate()
            .map_err(|error| RuntimeError::InvalidRequest(error.to_string()))?;
        native_codec::write_frame(&mut self.stdin, request).map_err(RuntimeError::Transport)
    }

    pub fn try_event(&self) -> Result<Option<EventEnvelope>, RuntimeError> {
        match self.events.try_recv() {
            Ok(Ok(event)) => {
                event
                    .validate()
                    .map_err(|error| RuntimeError::InvalidEvent(error.to_string()))?;
                Ok(Some(event))
            }
            Ok(Err(message)) => Err(RuntimeError::WorkerProtocol { message }),
            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => Ok(None),
        }
    }

    pub fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>, RuntimeError> {
        self.child
            .lock()
            .map_err(|_| RuntimeError::WorkerLockPoisoned)?
            .try_wait()
            .map_err(RuntimeError::Wait)
    }

    pub fn captured_stderr(&self) -> Option<String> {
        self.stderr.try_recv().ok()
    }

    pub fn terminate(&mut self) -> Result<(), RuntimeError> {
        self.clear_wall_time_limit();
        let mut child = self
            .child
            .lock()
            .map_err(|_| RuntimeError::WorkerLockPoisoned)?;
        if child.try_wait().map_err(RuntimeError::Wait)?.is_none() {
            child.kill().map_err(RuntimeError::Terminate)?;
        }
        Ok(())
    }
}

impl Drop for NativeWorker {
    fn drop(&mut self) {
        let _ = self.terminate();
        if let Ok(mut child) = self.child.lock() {
            let _ = child.wait();
        }
        if let Some(handle) = self.event_reader.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.stderr_reader.take() {
            let _ = handle.join();
        }
    }
}

fn validate_manifest(
    manifest: &ManagedRuntimeManifest,
    requirement: &RuntimeRequirement,
) -> Result<(), RuntimeError> {
    if manifest.schema != "rspice.managed-python-runtime/v2" {
        return Err(RuntimeError::UnsupportedManifestSchema {
            schema: manifest.schema.clone(),
        });
    }
    for (field, value) in [
        ("runtime build", manifest.runtime_build.as_str()),
        ("target triple", manifest.target_triple.as_str()),
        ("architecture", manifest.architecture.as_str()),
        ("Python ABI", manifest.python_abi.as_str()),
    ] {
        validate_short_text(field, value)?;
    }
    if manifest.protocol_major != PROTOCOL_VERSION.major
        || manifest.protocol_minor > PROTOCOL_VERSION.minor
    {
        return Err(RuntimeError::IncompatibleProtocol {
            major: manifest.protocol_major,
            minor: manifest.protocol_minor,
        });
    }
    if manifest.target_triple != requirement.target_triple
        || manifest.architecture != requirement.architecture
    {
        return Err(RuntimeError::WrongPlatform {
            expected: format!(
                "{} ({})",
                requirement.target_triple, requirement.architecture
            ),
            actual: format!("{} ({})", manifest.target_triple, manifest.architecture),
        });
    }
    let python = Version::parse(&manifest.python_version).map_err(|source| {
        RuntimeError::InvalidVersion {
            field: "Python",
            source,
        }
    })?;
    if python.major != MANAGED_PYTHON_MAJOR
        || python.minor != MANAGED_PYTHON_MINOR
        || !requirement.python.matches(&python)
    {
        return Err(RuntimeError::IncompatibleVersion {
            field: "Python",
            required: requirement.python.to_string(),
            actual: python.to_string(),
        });
    }
    if manifest.python_abi != MANAGED_PYTHON_ABI {
        return Err(RuntimeError::IncompatiblePythonAbi {
            expected: MANAGED_PYTHON_ABI,
            actual: manifest.python_abi.clone(),
        });
    }
    let api = Version::parse(&manifest.rspice_api_version).map_err(|source| {
        RuntimeError::InvalidVersion {
            field: "RSpice API",
            source,
        }
    })?;
    if !requirement.rspice_api.matches(&api) {
        return Err(RuntimeError::IncompatibleVersion {
            field: "RSpice API",
            required: requirement.rspice_api.to_string(),
            actual: api.to_string(),
        });
    }
    if manifest.environment_digests_sha256.is_empty()
        || manifest.environment_digests_sha256.len() > 4_096
    {
        return Err(RuntimeError::InvalidEnvironmentCount {
            actual: manifest.environment_digests_sha256.len(),
        });
    }
    let mut environments = BTreeSet::new();
    for environment in &manifest.environment_digests_sha256 {
        decode_hex_exact::<32>(environment).map_err(|reason| {
            RuntimeError::InvalidEnvironmentDigest {
                digest: environment.clone(),
                reason,
            }
        })?;
        if !environments.insert(environment) {
            return Err(RuntimeError::DuplicateEnvironmentDigest {
                digest: environment.clone(),
            });
        }
    }
    if manifest.files.is_empty() || manifest.files.len() > 100_000 {
        return Err(RuntimeError::InvalidFileCount {
            actual: manifest.files.len(),
        });
    }
    validate_portable_path(&manifest.python_executable)?;
    validate_portable_path(&manifest.worker_bootstrap)?;
    Ok(())
}

fn verify_files(
    root: &Path,
    manifest: &ManagedRuntimeManifest,
) -> Result<([u8; 32], BTreeSet<String>), RuntimeError> {
    let mut files = manifest.files.clone();
    files.sort_by(|left, right| left.path.cmp(&right.path));
    let mut paths = BTreeSet::new();
    for file in &files {
        validate_portable_path(&file.path)?;
        if !paths.insert(file.path.clone()) {
            return Err(RuntimeError::DuplicateRuntimePath {
                path: file.path.clone(),
            });
        }
        let declared_digest = decode_hex_exact::<32>(&file.sha256).map_err(|reason| {
            RuntimeError::InvalidFileDigest {
                path: file.path.clone(),
                reason,
            }
        })?;
        let path = root.join(&file.path);
        let metadata =
            fs::symlink_metadata(&path).map_err(|source| RuntimeError::OpenRuntimeFile {
                path: file.path.clone(),
                source,
            })?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(RuntimeError::UnsafeRuntimeMember {
                path: file.path.clone(),
            });
        }
        let canonical =
            fs::canonicalize(&path).map_err(|source| RuntimeError::OpenRuntimeFile {
                path: file.path.clone(),
                source,
            })?;
        if !canonical.starts_with(root) {
            return Err(RuntimeError::UnsafeRuntimeMember {
                path: file.path.clone(),
            });
        }
        if metadata.len() != file.bytes {
            return Err(RuntimeError::RuntimeFileSizeMismatch {
                path: file.path.clone(),
                expected: file.bytes,
                actual: metadata.len(),
            });
        }
        let actual_digest = hash_file(&canonical)?;
        if actual_digest != declared_digest {
            return Err(RuntimeError::RuntimeFileDigestMismatch {
                path: file.path.clone(),
                expected: file.sha256.clone(),
                actual: encode_hex(&actual_digest),
            });
        }
    }
    Ok((runtime_inventory_digest(manifest)?, paths))
}

/// Calculate the content identity signed into a managed-runtime manifest.
///
/// Release assembly uses this exact implementation, so the packager and the
/// installed verifier cannot silently drift to different framing, ordering,
/// or executable-bit semantics.
pub fn runtime_inventory_digest(
    manifest: &ManagedRuntimeManifest,
) -> Result<[u8; 32], RuntimeError> {
    let mut digest = Sha256::new();
    digest.update(RUNTIME_DIGEST_DOMAIN);
    digest.update(manifest.runtime_build.as_bytes());
    digest.update([0]);
    digest.update(manifest.target_triple.as_bytes());
    digest.update([0]);
    digest.update(manifest.python_version.as_bytes());
    digest.update([0]);
    digest.update(manifest.rspice_api_version.as_bytes());
    digest.update([0]);

    let mut environments = manifest.environment_digests_sha256.clone();
    environments.sort();
    for environment in environments {
        let value = decode_hex_exact::<32>(&environment).map_err(|reason| {
            RuntimeError::InvalidEnvironmentDigest {
                digest: environment,
                reason,
            }
        })?;
        digest.update(value);
    }

    let mut paths = BTreeSet::new();
    let mut files = manifest.files.clone();
    files.sort_by(|left, right| left.path.cmp(&right.path));
    for file in files {
        validate_portable_path(&file.path)?;
        if !paths.insert(file.path.clone()) {
            return Err(RuntimeError::DuplicateRuntimePath { path: file.path });
        }
        let file_digest = decode_hex_exact::<32>(&file.sha256).map_err(|reason| {
            RuntimeError::InvalidFileDigest {
                path: file.path.clone(),
                reason,
            }
        })?;
        digest.update((file.path.len() as u64).to_be_bytes());
        digest.update(file.path.as_bytes());
        digest.update(file.bytes.to_be_bytes());
        digest.update([u8::from(file.executable)]);
        digest.update(file_digest);
    }
    Ok(digest.finalize().into())
}

fn reject_unlisted_files(root: &Path, listed: &BTreeSet<String>) -> Result<(), RuntimeError> {
    let mut pending = vec![(root.to_path_buf(), String::new())];
    let mut discovered = BTreeSet::new();
    while let Some((directory, prefix)) = pending.pop() {
        for entry in fs::read_dir(&directory).map_err(|source| RuntimeError::EnumerateRuntime {
            path: directory.clone(),
            source,
        })? {
            let entry = entry.map_err(|source| RuntimeError::EnumerateRuntime {
                path: directory.clone(),
                source,
            })?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| RuntimeError::NonUnicodeRuntimePath)?;
            let logical = if prefix.is_empty() {
                name
            } else {
                format!("{prefix}/{name}")
            };
            let metadata = fs::symlink_metadata(entry.path()).map_err(|source| {
                RuntimeError::EnumerateRuntime {
                    path: entry.path(),
                    source,
                }
            })?;
            if metadata.file_type().is_symlink() {
                return Err(RuntimeError::UnsafeRuntimeMember { path: logical });
            }
            if metadata.is_dir() {
                pending.push((entry.path(), logical));
            } else if metadata.is_file()
                && !matches!(logical.as_str(), MANIFEST_FILE | SIGNATURE_FILE)
            {
                discovered.insert(logical);
            } else if !metadata.is_file() {
                return Err(RuntimeError::UnsafeRuntimeMember { path: logical });
            }
        }
    }
    if discovered != *listed {
        let path = discovered
            .symmetric_difference(listed)
            .next()
            .cloned()
            .unwrap_or_else(|| "<unknown>".to_owned());
        return Err(RuntimeError::RuntimeInventoryMismatch { path });
    }
    Ok(())
}

fn verified_member_path(
    root: &Path,
    logical: &str,
    listed: &BTreeSet<String>,
) -> Result<PathBuf, RuntimeError> {
    if !listed.contains(logical) {
        return Err(RuntimeError::EntrypointNotListed {
            path: logical.to_owned(),
        });
    }
    let path =
        fs::canonicalize(root.join(logical)).map_err(|source| RuntimeError::OpenRuntimeFile {
            path: logical.to_owned(),
            source,
        })?;
    if !path.starts_with(root) {
        return Err(RuntimeError::UnsafeRuntimeMember {
            path: logical.to_owned(),
        });
    }
    Ok(path)
}

fn prepare_scratch(path: &Path) -> Result<PathBuf, RuntimeError> {
    fs::create_dir_all(path).map_err(|source| RuntimeError::PrepareScratch {
        path: path.to_path_buf(),
        source,
    })?;
    let metadata = fs::symlink_metadata(path).map_err(|source| RuntimeError::PrepareScratch {
        path: path.to_path_buf(),
        source,
    })?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(RuntimeError::UnsafeScratch {
            path: path.to_path_buf(),
        });
    }
    fs::canonicalize(path).map_err(|source| RuntimeError::PrepareScratch {
        path: path.to_path_buf(),
        source,
    })
}

fn spawn_stderr_reader(
    stderr: ChildStderr,
    sender: mpsc::SyncSender<String>,
) -> Result<thread::JoinHandle<()>, RuntimeError> {
    thread::Builder::new()
        .name("rspice-python-stderr".to_owned())
        .spawn(move || {
            let mut reader = stderr.take((STDERR_CAPTURE_LIMIT + 1) as u64);
            let mut bytes = Vec::new();
            let _ = reader.read_to_end(&mut bytes);
            let truncated = bytes.len() > STDERR_CAPTURE_LIMIT;
            bytes.truncate(STDERR_CAPTURE_LIMIT);
            let mut text = String::from_utf8_lossy(&bytes).into_owned();
            if truncated {
                text.push_str("\n[RSpice truncated worker stderr]");
            }
            let _ = sender.send(text);
        })
        .map_err(RuntimeError::ReaderThread)
}

fn read_bounded(path: &Path, limit: u64) -> Result<Vec<u8>, RuntimeError> {
    let metadata = fs::symlink_metadata(path).map_err(|source| RuntimeError::ReadMetadata {
        path: path.to_path_buf(),
        source,
    })?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > limit {
        return Err(RuntimeError::InvalidControlFile {
            path: path.to_path_buf(),
            bytes: metadata.len(),
            maximum: limit,
        });
    }
    fs::read(path).map_err(|source| RuntimeError::ReadControlFile {
        path: path.to_path_buf(),
        source,
    })
}

fn hash_file(path: &Path) -> Result<[u8; 32], RuntimeError> {
    let mut file = fs::File::open(path).map_err(|source| RuntimeError::ReadRuntimeFile {
        path: path.to_path_buf(),
        source,
    })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|source| RuntimeError::ReadRuntimeFile {
                path: path.to_path_buf(),
                source,
            })?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hasher.finalize().into())
}

fn validate_portable_path(path: &str) -> Result<(), RuntimeError> {
    let as_path = Path::new(path);
    if path.is_empty()
        || path.len() > 1024
        || path.contains('\\')
        || path != path.trim()
        || as_path.is_absolute()
        || as_path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(RuntimeError::InvalidRuntimePath {
            path: path.to_owned(),
        });
    }
    Ok(())
}

fn validate_short_text(field: &'static str, text: &str) -> Result<(), RuntimeError> {
    if text.trim().is_empty() || text.len() > 256 || text.as_bytes().contains(&0) {
        return Err(RuntimeError::InvalidText { field });
    }
    Ok(())
}

fn decode_hex_exact<const N: usize>(text: &str) -> Result<[u8; N], String> {
    if text.len() != N * 2 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("expected exactly {} hexadecimal characters", N * 2));
    }
    let mut bytes = [0_u8; N];
    for (index, pair) in text.as_bytes().chunks_exact(2).enumerate() {
        let pair = std::str::from_utf8(pair).map_err(|_| "hex is not UTF-8".to_owned())?;
        bytes[index] = u8::from_str_radix(pair, 16).map_err(|_| "invalid hex digit".to_owned())?;
    }
    Ok(bytes)
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn signature_key_id(bytes: &[u8]) -> Option<String> {
    serde_json::from_slice::<ManifestSignature>(bytes)
        .ok()
        .map(|value| value.key_id)
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("could not open managed runtime root '{path}': {source}")]
    OpenRuntimeRoot {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("managed runtime root '{path}' is not a safe local directory")]
    UnsafeRuntimeRoot { path: PathBuf },
    #[error("could not read metadata for runtime control file '{path}': {source}")]
    ReadMetadata {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error(
        "runtime control file '{path}' has {bytes} bytes or an unsafe type; maximum is {maximum}"
    )]
    InvalidControlFile {
        path: PathBuf,
        bytes: u64,
        maximum: u64,
    },
    #[error("could not read runtime control file '{path}': {source}")]
    ReadControlFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("runtime signature document is invalid: {0}")]
    InvalidSignatureDocument(serde_json::Error),
    #[error("runtime manifest is invalid: {0}")]
    InvalidManifest(serde_json::Error),
    #[error("runtime signing key '{key_id}' is not trusted by this RSpice build")]
    UntrustedSigningKey { key_id: String },
    #[error("runtime manifest signature from key '{key_id}' is invalid")]
    InvalidManifestSignature { key_id: String },
    #[error("runtime signature encoding is invalid: {reason}")]
    InvalidSignatureEncoding { reason: String },
    #[error("runtime digest is invalid: {reason}")]
    InvalidRuntimeDigest { reason: String },
    #[error("runtime file digest for '{path}' is invalid: {reason}")]
    InvalidFileDigest { path: String, reason: String },
    #[error("runtime environment digest '{digest}' is invalid: {reason}")]
    InvalidEnvironmentDigest { digest: String, reason: String },
    #[error("runtime manifest schema '{schema}' is unsupported")]
    UnsupportedManifestSchema { schema: String },
    #[error("runtime protocol {major}.{minor} is incompatible")]
    IncompatibleProtocol { major: u16, minor: u16 },
    #[error("runtime is for {actual}; this application requires {expected}")]
    WrongPlatform { expected: String, actual: String },
    #[error("runtime {field} version '{actual}' does not satisfy '{required}'")]
    IncompatibleVersion {
        field: &'static str,
        required: String,
        actual: String,
    },
    #[error("runtime {field} version is invalid: {source}")]
    InvalidVersion {
        field: &'static str,
        source: semver::Error,
    },
    #[error("runtime Python ABI '{actual}' is incompatible; expected '{expected}'")]
    IncompatiblePythonAbi {
        expected: &'static str,
        actual: String,
    },
    #[error("runtime manifest contains {actual} files; expected 1..=100000")]
    InvalidFileCount { actual: usize },
    #[error("runtime manifest contains {actual} environments; expected 1..=4096")]
    InvalidEnvironmentCount { actual: usize },
    #[error("{field} is empty or invalid")]
    InvalidText { field: &'static str },
    #[error("runtime path '{path}' is not normalized and portable")]
    InvalidRuntimePath { path: String },
    #[error("runtime path '{path}' appears more than once")]
    DuplicateRuntimePath { path: String },
    #[error("runtime environment digest '{digest}' appears more than once")]
    DuplicateEnvironmentDigest { digest: String },
    #[error("could not open runtime file '{path}': {source}")]
    OpenRuntimeFile {
        path: String,
        source: std::io::Error,
    },
    #[error("runtime member '{path}' is a symlink, special file, or escapes the signed root")]
    UnsafeRuntimeMember { path: String },
    #[error("runtime file '{path}' has {actual} bytes; manifest declares {expected}")]
    RuntimeFileSizeMismatch {
        path: String,
        expected: u64,
        actual: u64,
    },
    #[error("could not read runtime file '{path}': {source}")]
    ReadRuntimeFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("runtime file '{path}' digest is {actual}; manifest declares {expected}")]
    RuntimeFileDigestMismatch {
        path: String,
        expected: String,
        actual: String,
    },
    #[error("runtime aggregate digest is {actual}; manifest declares {expected}")]
    RuntimeDigestMismatch { expected: String, actual: String },
    #[error("could not enumerate runtime directory '{path}': {source}")]
    EnumerateRuntime {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("runtime contains a non-Unicode path")]
    NonUnicodeRuntimePath,
    #[error("runtime signed inventory and on-disk inventory differ at '{path}'")]
    RuntimeInventoryMismatch { path: String },
    #[error("runtime entrypoint '{path}' is not part of the signed inventory")]
    EntrypointNotListed { path: String },
    #[error("runtime identity is invalid: {0}")]
    InvalidIdentity(String),
    #[error("trust store already contains key '{key_id}'")]
    DuplicateTrustKey { key_id: String },
    #[error("managed-runtime trust key is not a valid Ed25519 public key")]
    InvalidTrustKeyEncoding,
    #[error("could not prepare Automation scratch directory '{path}': {source}")]
    PrepareScratch {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Automation scratch path '{path}' is not a safe local directory")]
    UnsafeScratch { path: PathBuf },
    #[error("could not launch verified Python worker '{executable}': {source}")]
    Launch {
        executable: PathBuf,
        source: std::io::Error,
    },
    #[error("Python worker did not provide its {0} pipe")]
    MissingWorkerPipe(&'static str),
    #[error("could not start Python worker reader thread: {0}")]
    ReaderThread(std::io::Error),
    #[error("could not start Python worker resource watchdog: {0}")]
    WatchdogThread(std::io::Error),
    #[error("Python worker resource limits are process-lifetime state and were already applied")]
    ResourceLimitsAlreadyApplied,
    #[error("Python worker process ownership lock was poisoned")]
    WorkerLockPoisoned,
    #[cfg(windows)]
    #[error("could not {operation} the mandatory Windows Automation job object (Win32 {code})")]
    WindowsJob { operation: &'static str, code: u32 },
    #[cfg(windows)]
    #[error("Windows Automation job-object information has unsupported size {bytes}")]
    WindowsJobInformationSize { bytes: usize },
    #[error("invalid Automation request: {0}")]
    InvalidRequest(String),
    #[error("invalid Automation event: {0}")]
    InvalidEvent(String),
    #[error("Python worker protocol failed: {message}")]
    WorkerProtocol { message: String },
    #[error("Python worker transport failed: {0}")]
    Transport(#[from] native_codec::CodecError),
    #[error("could not query Python worker status: {0}")]
    Wait(std::io::Error),
    #[error("could not terminate Python worker: {0}")]
    Terminate(std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer as _, SigningKey};
    use rand::rngs::OsRng;

    fn requirement() -> RuntimeRequirement {
        RuntimeRequirement {
            target_triple: "x86_64-pc-windows-msvc".to_owned(),
            architecture: "x86_64".to_owned(),
            python: VersionReq::parse(">=3.14.0,<3.15.0").unwrap(),
            rspice_api: VersionReq::parse(">=1.0.0,<2.0.0").unwrap(),
        }
    }

    fn create_signed_runtime() -> (tempfile::TempDir, RuntimeTrustStore) {
        create_signed_runtime_for("3.14.6", "cp314")
    }

    fn create_signed_runtime_for(
        python_version: &str,
        python_abi: &str,
    ) -> (tempfile::TempDir, RuntimeTrustStore) {
        let temporary = tempfile::tempdir().unwrap();
        fs::create_dir_all(temporary.path().join("worker")).unwrap();
        fs::write(temporary.path().join("python.exe"), b"managed-python").unwrap();
        fs::write(
            temporary.path().join("worker/bootstrap.py"),
            b"# governed worker\n",
        )
        .unwrap();
        let mut files = vec![
            runtime_file(temporary.path(), "python.exe", true),
            runtime_file(temporary.path(), "worker/bootstrap.py", false),
        ];
        files.sort_by(|left, right| left.path.cmp(&right.path));
        let mut manifest = ManagedRuntimeManifest {
            schema: "rspice.managed-python-runtime/v2".to_owned(),
            runtime_build: format!("rspice-python-{python_version}+test.1"),
            target_triple: "x86_64-pc-windows-msvc".to_owned(),
            architecture: "x86_64".to_owned(),
            python_version: python_version.to_owned(),
            python_abi: python_abi.to_owned(),
            rspice_api_version: "1.0.0".to_owned(),
            environment_digests_sha256: vec![
                "d445b1443965be4e6b1b191ee023176dbd35430ac3cd00603458384ea03b8518".to_owned(),
            ],
            protocol_major: PROTOCOL_VERSION.major,
            protocol_minor: PROTOCOL_VERSION.minor,
            python_executable: "python.exe".to_owned(),
            worker_bootstrap: "worker/bootstrap.py".to_owned(),
            runtime_digest_sha256: String::new(),
            files,
        };
        manifest.runtime_digest_sha256 = encode_hex(&runtime_inventory_digest(&manifest).unwrap());
        let manifest_bytes = serde_json::to_vec_pretty(&manifest).unwrap();
        let signing = SigningKey::generate(&mut OsRng);
        let signature = signing.sign(&manifest_bytes);
        let signature_document = ManifestSignature {
            key_id: "test-release-key".to_owned(),
            signature_hex: encode_hex(&signature.to_bytes()),
        };
        fs::write(temporary.path().join(MANIFEST_FILE), manifest_bytes).unwrap();
        fs::write(
            temporary.path().join(SIGNATURE_FILE),
            serde_json::to_vec_pretty(&signature_document).unwrap(),
        )
        .unwrap();
        let mut trust = RuntimeTrustStore::new();
        trust
            .add_key("test-release-key", signing.verifying_key())
            .unwrap();
        (temporary, trust)
    }

    fn runtime_file(root: &Path, path: &str, executable: bool) -> RuntimeFile {
        let bytes = fs::read(root.join(path)).unwrap();
        RuntimeFile {
            path: path.to_owned(),
            bytes: bytes.len() as u64,
            sha256: encode_hex(&Sha256::digest(&bytes)),
            executable,
        }
    }

    #[test]
    fn signed_complete_app_local_runtime_is_accepted() {
        let (runtime, trust) = create_signed_runtime();
        let verified = trust.verify(runtime.path(), &requirement()).unwrap();
        assert!(verified.identity().managed);
        assert_eq!(verified.identity().python_version, "3.14.6");
        assert_eq!(verified.root(), fs::canonicalize(runtime.path()).unwrap());
    }

    #[test]
    fn prior_python_abi_line_is_rejected_even_by_a_broad_project_range() {
        let (runtime, trust) = create_signed_runtime_for("3.12.9", "cp312");
        let mut broad = requirement();
        broad.python = VersionReq::parse(">=3.12.0,<3.15.0").unwrap();

        assert!(matches!(
            trust.verify(runtime.path(), &broad),
            Err(RuntimeError::IncompatibleVersion {
                field: "Python",
                actual,
                ..
            }) if actual == "3.12.9"
        ));
    }

    #[test]
    fn mismatched_python_abi_tag_is_rejected() {
        let (runtime, trust) = create_signed_runtime_for("3.14.6", "cp312");

        assert!(matches!(
            trust.verify(runtime.path(), &requirement()),
            Err(RuntimeError::IncompatiblePythonAbi {
                expected: "cp314",
                actual,
            }) if actual == "cp312"
        ));
    }

    #[test]
    fn file_tampering_and_unsigned_extra_code_fail_closed() {
        let (runtime, trust) = create_signed_runtime();
        fs::write(runtime.path().join("worker/bootstrap.py"), b"# modified\n").unwrap();
        assert!(matches!(
            trust.verify(runtime.path(), &requirement()),
            Err(RuntimeError::RuntimeFileSizeMismatch { .. })
                | Err(RuntimeError::RuntimeFileDigestMismatch { .. })
        ));

        let (runtime, trust) = create_signed_runtime();
        fs::write(runtime.path().join("worker/injected.py"), b"pass\n").unwrap();
        assert!(matches!(
            trust.verify(runtime.path(), &requirement()),
            Err(RuntimeError::RuntimeInventoryMismatch { .. })
        ));
    }

    #[test]
    fn manifest_signature_and_platform_mismatch_fail_closed() {
        let (runtime, trust) = create_signed_runtime();
        let manifest_path = runtime.path().join(MANIFEST_FILE);
        let mut bytes = fs::read(&manifest_path).unwrap();
        bytes.push(b' ');
        fs::write(&manifest_path, bytes).unwrap();
        assert!(matches!(
            trust.verify(runtime.path(), &requirement()),
            Err(RuntimeError::InvalidManifestSignature { .. })
        ));

        let (runtime, trust) = create_signed_runtime();
        let mut wrong = requirement();
        wrong.architecture = "aarch64".to_owned();
        assert!(matches!(
            trust.verify(runtime.path(), &wrong),
            Err(RuntimeError::WrongPlatform { .. })
        ));
    }
}
