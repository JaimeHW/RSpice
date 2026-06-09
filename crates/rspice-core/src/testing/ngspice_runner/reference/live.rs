use super::*;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const LIVE_REFERENCES_ENV: &str = "RSPICE_NGSPICE_LIVE_REFERENCES";
const SOURCE_ROOT_ENV: &str = "NGSPICE_SOURCE_ROOT";
const NGSPICE_EXE_ENV: &str = "NGSPICE_EXE";
const REFERENCE_TIMEOUT_ENV: &str = "RSPICE_NGSPICE_REFERENCE_TIMEOUT_MS";
const DEFAULT_REFERENCE_TIMEOUT_MS: u128 = 30_000;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn live_ngspice_reference_config_from_env()
    -> Result<Option<LiveNgspiceReferenceConfig>, String> {
        if !Self::live_ngspice_references_requested()? {
            return Ok(None);
        }

        let source_root = Self::path_env(SOURCE_ROOT_ENV)
            .ok_or_else(|| format!("{LIVE_REFERENCES_ENV}=1 requires {SOURCE_ROOT_ENV}"))?
            .canonicalize()
            .map_err(|err| format!("failed to resolve {SOURCE_ROOT_ENV}: {err}"))?;
        let ngspice_exe = Self::path_env(NGSPICE_EXE_ENV)
            .ok_or_else(|| format!("{LIVE_REFERENCES_ENV}=1 requires {NGSPICE_EXE_ENV}"))?
            .canonicalize()
            .map_err(|err| format!("failed to resolve {NGSPICE_EXE_ENV}: {err}"))?;

        if !source_root.join("tests").is_dir() {
            return Err(format!(
                "{SOURCE_ROOT_ENV} must point at a local ngspice source tree with tests/: {}",
                source_root.display()
            ));
        }
        if !ngspice_exe.is_file() {
            return Err(format!(
                "{NGSPICE_EXE_ENV} must point at a local ngspice executable: {}",
                ngspice_exe.display()
            ));
        }

        Ok(Some(LiveNgspiceReferenceConfig {
            source_root,
            ngspice_exe,
            timeout_ms: Self::reference_timeout_ms_from_env()?,
        }))
    }

    pub(in crate::testing::ngspice_runner) fn authoritative_circuit_path(
        &self,
        cir_path: &Path,
    ) -> Result<PathBuf, String> {
        let Some(config) = self.live_reference_config()? else {
            return Ok(cir_path.to_path_buf());
        };
        self.source_circuit_path_for(config, cir_path)
    }

    pub(in crate::testing::ngspice_runner) fn load_reference_output(
        &self,
        cir_path: &Path,
    ) -> Result<Option<ReferenceOutput>, String> {
        let Some(config) = self.live_reference_config()? else {
            let out_path = cir_path.with_extension("out");
            if !out_path.exists() {
                return Ok(None);
            }
            let content = fs::read_to_string(&out_path).map_err(|err| {
                format!(
                    "Failed to read reference output '{}': {err}",
                    out_path.display()
                )
            })?;
            return Ok(Some(ReferenceOutput {
                content,
                description: format!("checked-in reference output '{}'", out_path.display()),
            }));
        };

        let source_cir = self.source_circuit_path_for(config, cir_path)?;
        let content = self.live_ngspice_output(config, &source_cir)?;
        Ok(Some(ReferenceOutput {
            content,
            description: format!("live ngspice output from '{}'", source_cir.display()),
        }))
    }

    fn live_reference_config(&self) -> Result<Option<&LiveNgspiceReferenceConfig>, String> {
        match &self.live_reference_config {
            Ok(Some(config)) => Ok(Some(config)),
            Ok(None) => Ok(None),
            Err(err) => Err(err.clone()),
        }
    }

    fn live_ngspice_references_requested() -> Result<bool, String> {
        let Some(value) = std::env::var_os(LIVE_REFERENCES_ENV) else {
            return Ok(false);
        };
        let value = value.to_string_lossy();
        match value.trim().to_ascii_lowercase().as_str() {
            "" | "0" | "false" | "no" | "off" => Ok(false),
            "1" | "true" | "yes" | "on" => Ok(true),
            other => Err(format!(
                "{LIVE_REFERENCES_ENV} must be one of 1/0, true/false, yes/no, or on/off; got '{other}'"
            )),
        }
    }

    fn path_env(name: &str) -> Option<PathBuf> {
        std::env::var_os(name)
            .filter(|value| !value.as_os_str().is_empty())
            .map(PathBuf::from)
    }

    fn reference_timeout_ms_from_env() -> Result<u128, String> {
        let Some(value) = std::env::var_os(REFERENCE_TIMEOUT_ENV) else {
            return Ok(DEFAULT_REFERENCE_TIMEOUT_MS);
        };
        let value = value.to_string_lossy();
        let timeout_ms = value
            .trim()
            .parse::<u128>()
            .map_err(|err| format!("invalid {REFERENCE_TIMEOUT_ENV}: {err}"))?;
        if timeout_ms == 0 {
            return Err(format!("{REFERENCE_TIMEOUT_ENV} must be greater than zero"));
        }
        Ok(timeout_ms)
    }

    fn source_circuit_path_for(
        &self,
        config: &LiveNgspiceReferenceConfig,
        cir_path: &Path,
    ) -> Result<PathBuf, String> {
        let source_tests_dir = config.source_root.join("tests");
        let normalized_cir_path = cir_path
            .canonicalize()
            .unwrap_or_else(|_| cir_path.to_path_buf());
        let normalized_source_tests_dir = source_tests_dir
            .canonicalize()
            .unwrap_or_else(|_| source_tests_dir.clone());

        if normalized_cir_path.starts_with(&normalized_source_tests_dir) {
            if normalized_cir_path.is_file() {
                return Ok(normalized_cir_path);
            }
            return Err(format!(
                "authoritative ngspice source deck is missing: {}",
                normalized_cir_path.display()
            ));
        }

        let rel = normalized_cir_path
            .strip_prefix(&self.test_dir)
            .or_else(|_| cir_path.strip_prefix(&self.test_dir))
            .map_err(|_| {
                format!(
                    "{LIVE_REFERENCES_ENV}=1 requires '{}' to live under test directory '{}' so it can be mapped into the local ngspice source tree",
                    cir_path.display(),
                    self.test_dir.display()
                )
            })?;
        let source_cir = source_tests_dir.join(rel);
        if !source_cir.is_file() {
            return Err(format!(
                "{LIVE_REFERENCES_ENV}=1 mapped '{}' to '{}', but the authoritative local ngspice source deck is missing",
                cir_path.display(),
                source_cir.display()
            ));
        }
        Ok(source_cir)
    }

    fn live_ngspice_output(
        &self,
        config: &LiveNgspiceReferenceConfig,
        source_cir: &Path,
    ) -> Result<String, String> {
        let cache_key = source_cir
            .canonicalize()
            .unwrap_or_else(|_| source_cir.to_path_buf());

        if let Some(cached) = self
            .live_reference_cache
            .lock()
            .map_err(|_| "live ngspice reference cache was poisoned".to_string())?
            .get(&cache_key)
            .cloned()
        {
            return cached;
        }

        let result = config.run(source_cir);
        self.live_reference_cache
            .lock()
            .map_err(|_| "live ngspice reference cache was poisoned".to_string())?
            .insert(cache_key, result.clone());
        result
    }
}

impl LiveNgspiceReferenceConfig {
    fn run(&self, source_cir: &Path) -> Result<String, String> {
        let start = Instant::now();
        let stdout_path = unique_temp_path(source_cir, "stdout");
        let stderr_path = unique_temp_path(source_cir, "stderr");
        let stdout_file = fs::File::create(&stdout_path).map_err(|err| {
            format!(
                "failed to create temporary stdout file '{}': {err}",
                stdout_path.display()
            )
        })?;
        let stderr_file = fs::File::create(&stderr_path).map_err(|err| {
            format!(
                "failed to create temporary stderr file '{}': {err}",
                stderr_path.display()
            )
        })?;

        let source_parent = source_cir.parent().unwrap_or(&self.source_root);
        let mut child = Command::new(&self.ngspice_exe)
            .arg("--batch")
            .arg(source_cir)
            .current_dir(source_parent)
            .env("SPICE_SCRIPTS", self.source_root.join("tests").join("bin"))
            .env("ngspice_vpath", source_parent)
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file))
            .spawn()
            .map_err(|err| {
                format!(
                    "failed to spawn local ngspice '{}': {err}",
                    self.ngspice_exe.display()
                )
            })?;

        let timeout = Duration::from_millis(self.timeout_ms.min(u64::MAX as u128) as u64);
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) if start.elapsed() >= timeout => {
                    let _ = child.kill();
                    let _ = child.wait();
                    let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
                    let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
                    let _ = fs::remove_file(&stdout_path);
                    let _ = fs::remove_file(&stderr_path);
                    return Err(format!(
                        "local ngspice exceeded live reference timeout ({}ms) for '{}'; stdout tail: {}; stderr tail: {}",
                        self.timeout_ms,
                        source_cir.display(),
                        tail(&stdout),
                        tail(&stderr)
                    ));
                }
                Ok(None) => thread::sleep(Duration::from_millis(50)),
                Err(err) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    let _ = fs::remove_file(&stdout_path);
                    let _ = fs::remove_file(&stderr_path);
                    return Err(format!("failed to poll local ngspice: {err}"));
                }
            }
        };

        let stdout = fs::read_to_string(&stdout_path).map_err(|err| {
            format!(
                "failed to read temporary stdout file '{}': {err}",
                stdout_path.display()
            )
        })?;
        let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
        let _ = fs::remove_file(&stdout_path);
        let _ = fs::remove_file(&stderr_path);

        if !status.success() && stdout.trim().is_empty() {
            return Err(format!(
                "local ngspice exited with status {status} for '{}'; stdout tail: {}; stderr tail: {}",
                source_cir.display(),
                tail(&stdout),
                tail(&stderr)
            ));
        }
        if !status.success() {
            log::warn!(
                "Using captured stdout from local ngspice despite status {status} for '{}'; stderr tail: {}",
                source_cir.display(),
                tail(&stderr)
            );
        }

        Ok(stdout)
    }
}

fn unique_temp_path(cir_path: &Path, suffix: &str) -> PathBuf {
    let stem = cir_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();

    std::env::temp_dir().join(format!(
        "rspice-ngspice-live-reference-{stem}-{suffix}-{}-{}.txt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos()
    ))
}

fn tail(content: &str) -> String {
    let lines = content
        .lines()
        .rev()
        .take(3)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(" | ");
    truncate(lines.trim(), 240)
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut iter = value.chars();
    let truncated = iter.by_ref().take(max_chars).collect::<String>();
    if iter.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}
