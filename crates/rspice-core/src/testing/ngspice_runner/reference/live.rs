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

#[derive(Clone, Debug)]
struct RawReferencePlot {
    plotname: String,
    variables: Vec<String>,
    is_complex: bool,
    data: Vec<Vec<num_complex::Complex64>>,
}

#[derive(Clone, Debug, Default)]
struct RawPrintRequest {
    include_all: bool,
    variables: Vec<String>,
}

#[derive(Clone, Debug)]
struct RawHeader {
    plotname: String,
    variables: Vec<String>,
    no_points: usize,
    is_binary: bool,
    is_complex: bool,
}

#[derive(Clone, Copy)]
enum RawBinaryEncoding {
    RealAllF64,
    RealMixedAxisF64RestF32,
    ComplexAllF64,
    ComplexMixedAxisF64RestF32,
}

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
        let ngspice_exe = prefer_windows_console_ngspice_exe(ngspice_exe)?;

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

    pub(in crate::testing::ngspice_runner) fn load_live_raw_reference_tables(
        &self,
        cir_path: &Path,
    ) -> Result<Option<Vec<ReferenceTable>>, String> {
        let Some(config) = self.live_reference_config()? else {
            return Ok(None);
        };
        let source_cir = self.source_circuit_path_for(config, cir_path)?;
        let cache_key = source_cir
            .canonicalize()
            .unwrap_or_else(|_| source_cir.to_path_buf());

        if let Some(cached) = self
            .live_raw_reference_cache
            .lock()
            .map_err(|_| "live ngspice raw reference cache was poisoned".to_string())?
            .get(&cache_key)
            .cloned()
        {
            return cached.map(Some);
        }

        let result = config.run_raw_reference_tables(&source_cir, self.config.absolute_tolerance);
        self.live_raw_reference_cache
            .lock()
            .map_err(|_| "live ngspice raw reference cache was poisoned".to_string())?
            .insert(cache_key, result.clone());
        result.map(Some)
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

    fn run_raw_reference_tables(
        &self,
        source_cir: &Path,
        absolute_tolerance: f64,
    ) -> Result<Vec<ReferenceTable>, String> {
        let source = fs::read_to_string(source_cir).map_err(|err| {
            format!(
                "failed to read local ngspice source deck '{}' for raw references: {err}",
                source_cir.display()
            )
        })?;
        let source = Netlist::preprocess_includes(&source, source_cir).unwrap_or(source);
        let requests = raw_print_requests_by_axis(&source);
        if requests.is_empty() {
            return Ok(Vec::new());
        }

        let start = Instant::now();
        let raw_path = unique_temp_path(source_cir, "raw");
        let stdout_path = unique_temp_path(source_cir, "raw-stdout");
        let stderr_path = unique_temp_path(source_cir, "raw-stderr");
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
            .arg("-r")
            .arg(&raw_path)
            .arg(source_cir)
            .current_dir(source_parent)
            .env("SPICE_SCRIPTS", self.source_root.join("tests").join("bin"))
            .env("ngspice_vpath", source_parent)
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file))
            .spawn()
            .map_err(|err| {
                format!(
                    "failed to spawn local ngspice '{}' for raw references: {err}",
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
                    let _ = fs::remove_file(&raw_path);
                    let _ = fs::remove_file(&stdout_path);
                    let _ = fs::remove_file(&stderr_path);
                    return Err(format!(
                        "local ngspice exceeded live raw reference timeout ({}ms) for '{}'; stdout tail: {}; stderr tail: {}",
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
                    let _ = fs::remove_file(&raw_path);
                    let _ = fs::remove_file(&stdout_path);
                    let _ = fs::remove_file(&stderr_path);
                    return Err(format!(
                        "failed to poll local ngspice raw reference run: {err}"
                    ));
                }
            }
        };

        let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
        let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
        let raw = fs::read(&raw_path).map_err(|err| {
            let _ = fs::remove_file(&raw_path);
            let _ = fs::remove_file(&stdout_path);
            let _ = fs::remove_file(&stderr_path);
            format!(
                "local ngspice did not produce readable raw reference file '{}' for '{}': {err}; stdout tail: {}; stderr tail: {}",
                raw_path.display(),
                source_cir.display(),
                tail(&stdout),
                tail(&stderr)
            )
        })?;
        let _ = fs::remove_file(&raw_path);
        let _ = fs::remove_file(&stdout_path);
        let _ = fs::remove_file(&stderr_path);

        if !status.success() {
            return Err(format!(
                "local ngspice raw reference run exited with status {status} for '{}'; stdout tail: {}; stderr tail: {}",
                source_cir.display(),
                tail(&stdout),
                tail(&stderr)
            ));
        }
        if raw.is_empty() {
            return Err(format!(
                "local ngspice produced an empty raw reference file for '{}'; stdout tail: {}; stderr tail: {}",
                source_cir.display(),
                tail(&stdout),
                tail(&stderr)
            ));
        }

        let plots = parse_ngspice_raw_plots(&raw).map_err(|err| {
            format!(
                "failed to parse local ngspice raw reference for '{}': {err}; stdout tail: {}; stderr tail: {}",
                source_cir.display(),
                tail(&stdout),
                tail(&stderr)
            )
        })?;
        Ok(raw_reference_tables_from_plots(
            &plots,
            &requests,
            absolute_tolerance,
        ))
    }
}

impl RawBinaryEncoding {
    fn row_size_bytes(self, num_vars: usize) -> usize {
        match self {
            Self::RealAllF64 => num_vars * 8,
            Self::RealMixedAxisF64RestF32 => 8 + num_vars.saturating_sub(1) * 4,
            Self::ComplexAllF64 => num_vars * 16,
            Self::ComplexMixedAxisF64RestF32 => 16 + num_vars.saturating_sub(1) * 8,
        }
    }

    fn is_all_f64(self) -> bool {
        matches!(self, Self::RealAllF64 | Self::ComplexAllF64)
    }
}

fn raw_print_requests_by_axis(source: &str) -> BTreeMap<String, RawPrintRequest> {
    let mut requests = BTreeMap::new();
    for line in folded_netlist_lines(source) {
        let trimmed = TestRunner::strip_netlist_comment(&line).trim();
        let parts = trimmed.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 3 || !parts[0].eq_ignore_ascii_case(".print") {
            continue;
        }
        let Some(axis) = raw_axis_name_for_print_analysis(parts[1]) else {
            continue;
        };
        let request = requests
            .entry(axis)
            .or_insert_with(RawPrintRequest::default);
        for var in &parts[2..] {
            if var.eq_ignore_ascii_case("all") {
                request.include_all = true;
            } else if !request.variables.iter().any(|existing| {
                TestRunner::normalize_variable_name(existing)
                    == TestRunner::normalize_variable_name(var)
            }) {
                request.variables.push((*var).to_string());
            }
        }
    }
    requests
}

fn folded_netlist_lines(source: &str) -> Vec<String> {
    let mut folded: Vec<String> = Vec::new();
    for raw_line in source.lines() {
        let trimmed = TestRunner::strip_netlist_comment(raw_line).trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(continuation) = trimmed.strip_prefix('+') {
            if let Some(last) = folded.last_mut() {
                last.push(' ');
                last.push_str(continuation.trim());
            }
        } else {
            folded.push(trimmed.to_string());
        }
    }
    folded
}

fn raw_axis_name_for_print_analysis(analysis: &str) -> Option<String> {
    match analysis.to_ascii_lowercase().as_str() {
        "dc" => Some("v-sweep".to_string()),
        "tran" => Some("time".to_string()),
        "ac" | "noise" => Some("frequency".to_string()),
        _ => None,
    }
}

fn raw_reference_tables_from_plots(
    plots: &[RawReferencePlot],
    requests: &BTreeMap<String, RawPrintRequest>,
    absolute_tolerance: f64,
) -> Vec<ReferenceTable> {
    let mut tables = Vec::new();
    for plot in plots {
        let axis_name = raw_axis_name_for_plot(plot);
        let target = TestRunner::normalize_variable_name(&axis_name);
        let Some(request) = requests.get(&target) else {
            continue;
        };
        let Some(axis_data) = plot.data.first() else {
            continue;
        };
        let x = axis_data.iter().map(|value| value.re).collect::<Vec<_>>();
        if x.is_empty() {
            continue;
        }

        let requested_vars = if request.include_all {
            plot.variables.iter().skip(1).cloned().collect::<Vec<_>>()
        } else {
            request.variables.clone()
        };
        let mut table = ReferenceTable {
            x_name: axis_name,
            variables: BTreeMap::new(),
        };

        for var in requested_vars {
            if TestRunner::normalize_variable_name(&var)
                == TestRunner::normalize_variable_name(&table.x_name)
            {
                continue;
            }
            let Some(var_idx) = raw_variable_index_for_request(plot, &var) else {
                continue;
            };
            let Some(values) = plot.data.get(var_idx) else {
                continue;
            };
            let y = values
                .iter()
                .map(|value| {
                    if plot.is_complex {
                        TestRunner::evaluate_reference_complex_output(
                            &var,
                            *value,
                            absolute_tolerance,
                        )
                    } else {
                        value.re
                    }
                })
                .collect::<Vec<_>>();
            if y.len() == x.len() {
                table
                    .variables
                    .insert(var, ReferenceSeries { x: x.clone(), y });
            }
        }

        if !table.variables.is_empty() {
            tables.push(table);
        }
    }
    tables
}

fn raw_axis_name_for_plot(plot: &RawReferencePlot) -> String {
    let plotname = plot.plotname.to_ascii_lowercase();
    if plotname.contains("transient") {
        return "time".to_string();
    }
    if plotname.contains("dc transfer") {
        return "v-sweep".to_string();
    }
    if plotname.contains("ac analysis") || plotname.contains("noise") {
        return "frequency".to_string();
    }

    let raw_axis = plot.variables.first().cloned().unwrap_or_default();
    let normalized = TestRunner::normalize_variable_name(&raw_axis);
    normalized
        .strip_prefix("v(")
        .and_then(|inner| inner.strip_suffix(')'))
        .unwrap_or(&raw_axis)
        .to_string()
}

fn raw_variable_index_for_request(plot: &RawReferencePlot, request: &str) -> Option<usize> {
    let normalized_request = TestRunner::normalize_variable_name(request);
    if let Some(idx) = raw_variable_index(plot, &normalized_request) {
        return Some(idx);
    }

    match TestRunner::parse_ac_probe(&normalized_request) {
        Some(AcProbe::Voltage {
            node_pos, node_neg, ..
        }) => {
            let base = if let Some(node_neg) = node_neg {
                format!("v({node_pos},{node_neg})")
            } else {
                format!("v({node_pos})")
            };
            raw_variable_index(plot, &base)
        }
        Some(AcProbe::Current { branch, .. }) => raw_variable_index(plot, &format!("i({branch})")),
        None => None,
    }
}

fn raw_variable_index(plot: &RawReferencePlot, normalized_name: &str) -> Option<usize> {
    plot.variables.iter().position(|candidate| {
        TestRunner::normalize_variable_name(candidate)
            == TestRunner::normalize_variable_name(normalized_name)
    })
}

fn parse_ngspice_raw_plots(content: &[u8]) -> Result<Vec<RawReferencePlot>, String> {
    let mut pos = 0usize;
    let mut plots = Vec::new();
    while skip_raw_whitespace(content, &mut pos) {
        let header = parse_raw_header(content, &mut pos)?;
        let data = if header.is_binary {
            parse_raw_binary_data(content, &header, &mut pos)?
        } else {
            parse_raw_ascii_data(content, &header, &mut pos)?
        };
        plots.push(RawReferencePlot {
            plotname: header.plotname,
            variables: header.variables,
            is_complex: header.is_complex,
            data,
        });
    }
    Ok(plots)
}

fn parse_raw_header(content: &[u8], pos: &mut usize) -> Result<RawHeader, String> {
    let mut plotname = String::new();
    let mut variables = Vec::new();
    let mut no_variables = 0usize;
    let mut no_points = 0usize;
    let mut is_complex = false;
    let mut in_variables = false;

    let is_binary = loop {
        let Some(line) = read_raw_line(content, pos) else {
            return Err("unexpected end of rawfile while reading header".to_string());
        };
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("Binary:") {
            break true;
        }
        if trimmed.eq_ignore_ascii_case("Values:") {
            break false;
        }

        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key.trim().to_ascii_lowercase();
            let value = value.trim();
            match key.as_str() {
                "plotname" => plotname = value.to_string(),
                "flags" => {
                    is_complex = value
                        .split_whitespace()
                        .any(|flag| flag.eq_ignore_ascii_case("complex"));
                }
                "no. variables" => {
                    no_variables = value.parse::<usize>().map_err(|err| {
                        format!("invalid rawfile No. Variables value '{value}': {err}")
                    })?;
                }
                "no. points" => {
                    no_points = value.parse::<usize>().map_err(|err| {
                        format!("invalid rawfile No. Points value '{value}': {err}")
                    })?;
                }
                "variables" => in_variables = true,
                _ => {}
            }
            continue;
        }

        if in_variables && !trimmed.is_empty() {
            let parts = trimmed.split_whitespace().collect::<Vec<_>>();
            if parts.len() >= 3 && parts[0].parse::<usize>().is_ok() {
                variables.push(parts[1].to_string());
            }
        }
    };

    if no_variables == 0 {
        return Err("rawfile plot is missing No. Variables".to_string());
    }
    if no_points == 0 {
        return Err("rawfile plot is missing No. Points".to_string());
    }
    if variables.len() != no_variables {
        return Err(format!(
            "rawfile plot declared {no_variables} variable(s) but listed {}",
            variables.len()
        ));
    }
    Ok(RawHeader {
        plotname,
        variables,
        no_points,
        is_binary,
        is_complex,
    })
}

fn parse_raw_binary_data(
    content: &[u8],
    header: &RawHeader,
    pos: &mut usize,
) -> Result<Vec<Vec<num_complex::Complex64>>, String> {
    let encoding = choose_raw_binary_encoding(content, header, *pos)?;
    let payload_len = encoding.row_size_bytes(header.variables.len()) * header.no_points;
    if content.len().saturating_sub(*pos) < payload_len {
        return Err("rawfile binary payload is shorter than declared data".to_string());
    }

    let mut cursor = *pos;
    let mut data = vec![Vec::with_capacity(header.no_points); header.variables.len()];
    for _ in 0..header.no_points {
        for var_idx in 0..header.variables.len() {
            let real = match encoding {
                RawBinaryEncoding::RealAllF64 | RawBinaryEncoding::ComplexAllF64 => {
                    read_raw_f64(content, &mut cursor)?
                }
                RawBinaryEncoding::RealMixedAxisF64RestF32
                | RawBinaryEncoding::ComplexMixedAxisF64RestF32 => {
                    if var_idx == 0 {
                        read_raw_f64(content, &mut cursor)?
                    } else {
                        read_raw_f32(content, &mut cursor)? as f64
                    }
                }
            };
            let imag = match encoding {
                RawBinaryEncoding::ComplexAllF64 => read_raw_f64(content, &mut cursor)?,
                RawBinaryEncoding::ComplexMixedAxisF64RestF32 => {
                    if var_idx == 0 {
                        read_raw_f64(content, &mut cursor)?
                    } else {
                        read_raw_f32(content, &mut cursor)? as f64
                    }
                }
                RawBinaryEncoding::RealAllF64 | RawBinaryEncoding::RealMixedAxisF64RestF32 => 0.0,
            };
            data[var_idx].push(num_complex::Complex64::new(real, imag));
        }
    }
    *pos += payload_len;
    Ok(data)
}

fn choose_raw_binary_encoding(
    content: &[u8],
    header: &RawHeader,
    pos: usize,
) -> Result<RawBinaryEncoding, String> {
    let candidates = if header.is_complex {
        vec![
            RawBinaryEncoding::ComplexAllF64,
            RawBinaryEncoding::ComplexMixedAxisF64RestF32,
        ]
    } else {
        vec![
            RawBinaryEncoding::RealAllF64,
            RawBinaryEncoding::RealMixedAxisF64RestF32,
        ]
    };

    let mut in_bounds = Vec::new();
    for encoding in candidates {
        let payload_len = encoding.row_size_bytes(header.variables.len()) * header.no_points;
        let end = pos.saturating_add(payload_len);
        if end <= content.len() {
            in_bounds.push(encoding);
            if raw_payload_end_looks_valid(content, end) && encoding.is_all_f64() {
                return Ok(encoding);
            }
        }
    }

    if let Some(encoding) = in_bounds
        .iter()
        .copied()
        .find(|encoding| encoding.is_all_f64())
    {
        return Ok(encoding);
    }
    in_bounds
        .first()
        .copied()
        .ok_or_else(|| "rawfile binary payload is shorter than declared data".to_string())
}

fn raw_payload_end_looks_valid(content: &[u8], mut pos: usize) -> bool {
    while pos < content.len() && matches!(content[pos], b'\r' | b'\n' | b'\t' | b' ') {
        pos += 1;
    }
    pos == content.len() || content[pos..].starts_with(b"Title:")
}

fn parse_raw_ascii_data(
    content: &[u8],
    header: &RawHeader,
    pos: &mut usize,
) -> Result<Vec<Vec<num_complex::Complex64>>, String> {
    let mut data = vec![Vec::with_capacity(header.no_points); header.variables.len()];
    for _ in 0..header.no_points {
        let Some(index_line) = read_raw_line(content, pos) else {
            return Err("unexpected end of rawfile while reading ASCII values".to_string());
        };
        let first = index_line.split_whitespace().collect::<Vec<_>>();
        if first.len() >= header.variables.len() + 1 {
            for var_idx in 0..header.variables.len() {
                let value = first[var_idx + 1].parse::<f64>().map_err(|err| {
                    format!(
                        "invalid rawfile ASCII value '{}': {err}",
                        first[var_idx + 1]
                    )
                })?;
                data[var_idx].push(num_complex::Complex64::new(value, 0.0));
            }
            continue;
        }

        let value = first
            .get(1)
            .or_else(|| first.first())
            .ok_or_else(|| "rawfile ASCII point is missing axis value".to_string())?
            .parse::<f64>()
            .map_err(|err| format!("invalid rawfile ASCII axis value: {err}"))?;
        data[0].push(num_complex::Complex64::new(value, 0.0));
        for series in data.iter_mut().skip(1) {
            let Some(value_line) = read_raw_line(content, pos) else {
                return Err("unexpected end of rawfile while reading ASCII values".to_string());
            };
            let value = value_line
                .split_whitespace()
                .next()
                .ok_or_else(|| "rawfile ASCII value line is empty".to_string())?
                .parse::<f64>()
                .map_err(|err| format!("invalid rawfile ASCII value: {err}"))?;
            series.push(num_complex::Complex64::new(value, 0.0));
        }
    }
    Ok(data)
}

fn skip_raw_whitespace(content: &[u8], pos: &mut usize) -> bool {
    while *pos < content.len() && matches!(content[*pos], b'\r' | b'\n' | b'\t' | b' ') {
        *pos += 1;
    }
    *pos < content.len()
}

fn read_raw_line(content: &[u8], pos: &mut usize) -> Option<String> {
    if *pos >= content.len() {
        return None;
    }
    let start = *pos;
    while *pos < content.len() && content[*pos] != b'\n' {
        *pos += 1;
    }
    let mut end = *pos;
    if *pos < content.len() && content[*pos] == b'\n' {
        *pos += 1;
    }
    if end > start && content[end - 1] == b'\r' {
        end -= 1;
    }
    Some(String::from_utf8_lossy(&content[start..end]).into_owned())
}

fn read_raw_f64(content: &[u8], pos: &mut usize) -> Result<f64, String> {
    let end = pos.saturating_add(8);
    if end > content.len() {
        return Err("unexpected end of rawfile while reading f64".to_string());
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&content[*pos..end]);
    *pos = end;
    Ok(f64::from_le_bytes(bytes))
}

fn read_raw_f32(content: &[u8], pos: &mut usize) -> Result<f32, String> {
    let end = pos.saturating_add(4);
    if end > content.len() {
        return Err("unexpected end of rawfile while reading f32".to_string());
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&content[*pos..end]);
    *pos = end;
    Ok(f32::from_le_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn windows_gui_ngspice_exe_prefers_sibling_console_binary() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("rspice_ngspice_console_pick_{unique}"));
        fs::create_dir_all(&dir).expect("create temp dir");
        let gui = dir.join("ngspice.exe");
        let console = dir.join("ngspice_con.exe");
        fs::write(&gui, b"gui").expect("write gui marker");
        fs::write(&console, b"console").expect("write console marker");

        let resolved = prefer_windows_console_ngspice_exe(gui.clone()).expect("console exists");

        let _ = fs::remove_dir_all(&dir);
        assert_eq!(resolved, console);
    }

    #[cfg(windows)]
    #[test]
    fn windows_gui_ngspice_exe_without_sibling_console_is_rejected() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("rspice_ngspice_console_missing_{unique}"));
        fs::create_dir_all(&dir).expect("create temp dir");
        let gui = dir.join("ngspice.exe");
        fs::write(&gui, b"gui").expect("write gui marker");

        let err = prefer_windows_console_ngspice_exe(gui.clone())
            .expect_err("GUI ngspice.exe must not be used without ngspice_con.exe");

        let _ = fs::remove_dir_all(&dir);
        assert!(err.contains("ngspice_con.exe"), "{err}");
        assert!(err.contains(&gui.display().to_string()), "{err}");
    }

    #[test]
    fn rawfile_fallback_builds_tables_from_printed_variables_only() {
        let mut raw = Vec::new();
        push_binary_plot(
            &mut raw,
            "DC transfer characteristic",
            &[("v(v-sweep)", "voltage"), ("v(3)", "voltage")],
            &[vec![0.0, 4.0], vec![1.0, 2.0]],
        );
        push_binary_plot(
            &mut raw,
            "Transient Analysis",
            &[
                ("time", "time"),
                ("v(3)", "voltage"),
                ("v(5)", "voltage"),
                ("v(1)", "voltage"),
            ],
            &[vec![0.0, 4.0, 0.2, 0.0], vec![1.0e-9, 0.3, 4.8, 5.0]],
        );

        let requests =
            raw_print_requests_by_axis(".print dc v(3)\n.print tran v(3) v(5)\n.plot tran v(1)\n");
        assert!(requests.contains_key("v-sweep"));
        assert!(requests.contains_key("time"));
        let plots = parse_ngspice_raw_plots(&raw).expect("parse synthetic rawfile");
        assert_eq!(plots.len(), 2);
        let tables = raw_reference_tables_from_plots(&plots, &requests, 1.0e-12);
        let table_axes = tables
            .iter()
            .map(|table| table.x_name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(table_axes, vec!["v-sweep", "time"]);
        let transient = tables
            .iter()
            .find(|table| table.x_name == "time")
            .expect("transient table");
        assert!(transient.variables.contains_key("v(3)"));
        assert!(transient.variables.contains_key("v(5)"));
        assert!(!transient.variables.contains_key("v(1)"));
        assert_eq!(transient.variables["v(3)"].x, vec![0.0, 1.0e-9]);
        assert_eq!(transient.variables["v(5)"].y, vec![0.2, 4.8]);
    }

    fn push_binary_plot(
        raw: &mut Vec<u8>,
        plotname: &str,
        variables: &[(&str, &str)],
        rows: &[Vec<f64>],
    ) {
        raw.extend_from_slice(
            format!(
                "Title: synthetic\nDate: today\nPlotname: {plotname}\nFlags: real\nNo. Variables: {}\nNo. Points: {}\nVariables:\n",
                variables.len(),
                rows.len()
            )
            .as_bytes(),
        );
        for (idx, (name, kind)) in variables.iter().enumerate() {
            raw.extend_from_slice(format!("\t{idx}\t{name}\t{kind}\n").as_bytes());
        }
        raw.extend_from_slice(b"Binary:\n");
        for row in rows {
            assert_eq!(row.len(), variables.len());
            for value in row {
                raw.extend_from_slice(&value.to_le_bytes());
            }
        }
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

fn prefer_windows_console_ngspice_exe(ngspice_exe: PathBuf) -> Result<PathBuf, String> {
    #[cfg(windows)]
    {
        let is_windows_gui_binary = ngspice_exe
            .file_name()
            .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case("ngspice.exe"));
        if is_windows_gui_binary {
            let console_exe = ngspice_exe.with_file_name("ngspice_con.exe");
            if console_exe.is_file() {
                log::warn!(
                    "Using Windows console ngspice '{}' instead of GUI binary '{}'",
                    console_exe.display(),
                    ngspice_exe.display()
                );
                return Ok(console_exe);
            }
            return Err(format!(
                "{NGSPICE_EXE_ENV} points to Windows GUI ngspice.exe '{}', but sibling console binary '{}' is missing; set {NGSPICE_EXE_ENV} to ngspice_con.exe to avoid GUI popup dialogs",
                ngspice_exe.display(),
                console_exe.display()
            ));
        }
    }
    Ok(ngspice_exe)
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
