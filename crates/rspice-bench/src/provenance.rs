//! Reproducibility metadata shared by benchmark reports.

use crate::error::BenchError;
use crate::workspace_root;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::Read as _;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_VERSION_OUTPUT: usize = 16 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostInfo {
    pub os: String,
    pub arch: String,
    pub os_version: String,
    pub cpu_model: String,
    pub cpu_count: usize,
    /// Stable digest of the normalized host fields above, not a hostname.
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolProvenance {
    pub name: &'static str,
    pub version: &'static str,
    pub profile: &'static str,
    pub target: String,
    pub rustc: String,
    pub git_commit: Option<String>,
    pub git_dirty: Option<bool>,
    pub cargo_lock_blake3: Option<String>,
    pub executable_name: Option<String>,
    pub executable_blake3: Option<String>,
}

pub fn unix_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

pub fn host() -> HostInfo {
    let os = env::consts::OS.to_string();
    let arch = env::consts::ARCH.to_string();
    let cpu_count = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    let cpu_model = cpu_model();
    let os_version = os_version();
    let normalized = format!("{os}\n{arch}\n{os_version}\n{cpu_model}\n{cpu_count}");
    let fingerprint = blake3::hash(normalized.as_bytes()).to_hex().to_string();
    HostInfo {
        os,
        arch,
        os_version,
        cpu_model,
        cpu_count,
        fingerprint,
    }
}

pub fn tool() -> ToolProvenance {
    let root = workspace_root();
    let current_exe = env::current_exe().ok();
    ToolProvenance {
        name: "rspice-bench",
        version: env!("CARGO_PKG_VERSION"),
        profile: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
        target: format!("{}-{}", env::consts::ARCH, env::consts::OS),
        rustc: command_text(Command::new("rustc").arg("--version").arg("--verbose")),
        git_commit: git_text(&root, &["rev-parse", "HEAD"]),
        git_dirty: git_dirty(&root),
        cargo_lock_blake3: hash_file(&root.join("Cargo.lock")),
        executable_name: current_exe
            .as_deref()
            .and_then(Path::file_name)
            .map(|name| name.to_string_lossy().into_owned()),
        executable_blake3: current_exe.as_deref().and_then(hash_file),
    }
}

pub fn require_release(tool: &ToolProvenance) -> Result<(), BenchError> {
    if tool.profile != "release" {
        return Err(BenchError::BenchmarkPolicy {
            message: "trusted benchmark runs require a release build; pass --exploratory to run an ungated debug measurement".to_string(),
        });
    }
    Ok(())
}

fn hash_file(path: &Path) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).ok()?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Some(hasher.finalize().to_hex().to_string())
}

fn git_text(root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_dirty(root: &Path) -> Option<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=normal"])
        .current_dir(root)
        .output()
        .ok()?;
    output.status.success().then_some(!output.stdout.is_empty())
}

fn command_text(command: &mut Command) -> String {
    command_result(command).1
}

fn command_result(command: &mut Command) -> (bool, String) {
    match command.output() {
        Ok(output) => {
            let mut bytes = output.stdout;
            bytes.extend_from_slice(&output.stderr);
            bytes.truncate(MAX_VERSION_OUTPUT);
            let text = String::from_utf8_lossy(&bytes).trim().to_string();
            if output.status.success() && !text.is_empty() {
                (true, text)
            } else if text.is_empty() {
                (false, format!("exit status {}", output.status))
            } else {
                (false, format!("exit status {}: {text}", output.status))
            }
        }
        Err(error) => (false, format!("unavailable: {error}")),
    }
}

fn cpu_model() -> String {
    if let Ok(value) = env::var("PROCESSOR_IDENTIFIER")
        && !value.trim().is_empty()
    {
        return value.trim().to_string();
    }
    if cfg!(target_os = "linux")
        && let Ok(text) = fs::read_to_string("/proc/cpuinfo")
        && let Some(value) = text.lines().find_map(|line| {
            line.split_once(':')
                .filter(|(key, _)| key.trim() == "model name")
                .map(|(_, value)| value.trim().to_string())
        })
    {
        return value;
    }
    if cfg!(target_os = "macos") {
        let value = command_text(Command::new("sysctl").args(["-n", "machdep.cpu.brand_string"]));
        if !value.starts_with("unavailable:") {
            return value;
        }
    }
    "unknown".to_string()
}

fn os_version() -> String {
    if cfg!(windows) {
        return command_text(Command::new("cmd").args(["/c", "ver"]));
    }
    command_text(Command::new("uname").arg("-srv"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_fingerprint_is_complete_and_stable_within_a_run() {
        let first = host();
        let second = host();
        assert_eq!(first, second);
        assert_eq!(first.fingerprint.len(), 64);
        assert!(!first.os.is_empty());
        assert!(!first.arch.is_empty());
        assert!(first.cpu_count > 0);
    }
}
