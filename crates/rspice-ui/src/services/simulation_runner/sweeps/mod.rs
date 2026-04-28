//! Parametric and corner sweep runners.

#![allow(clippy::needless_range_loop, clippy::type_complexity)]

use super::{build_engine_config, generate_freq_points, parse_runner_netlist};
use rspice_core::Value;
use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::{AnalysisCommand, ElementKind, SourceSpec, StepCommand, StepTarget};
use rspice_core::solver::SimulationResult as CoreSimulationResult;
use std::path::Path;

mod sweep_points;
use sweep_points::{expand_step_sweep_values, extract_temp_points};

/// Parametric sweep data.
#[derive(Debug, Clone)]
pub struct ParametricData {
    pub target: String,
    pub sweep_values: Vec<Value>,
    pub voltages: Vec<(String, Vec<Value>)>,
    pub num_points: usize,
    pub num_failures: usize,
}

/// Explicit configuration for temperature sweep execution.
#[derive(Debug, Clone)]
pub struct TempRunConfig {
    pub temperatures_c: Vec<Value>,
    pub base_mode: CornerBaseMode,
}

impl Default for TempRunConfig {
    fn default() -> Self {
        Self {
            temperatures_c: vec![25.0],
            base_mode: CornerBaseMode::Op,
        }
    }
}

impl TempRunConfig {
    fn validate(&self) -> Result<(), String> {
        if self.temperatures_c.is_empty() {
            return Err("Temperature sweep requires at least one temperature point".to_string());
        }
        if self.temperatures_c.iter().any(|t| !t.is_finite()) {
            return Err("Temperature sweep points must be finite values".to_string());
        }
        validate_base_mode("Temperature sweep", &self.base_mode)
    }
}

/// Run parametric analysis by executing the first `.STEP` command in the netlist.
pub fn run_parametric_analysis(netlist_text: &str) -> Result<ParametricData, String> {
    run_parametric_analysis_with_source_path(netlist_text, None)
}

/// Run parametric analysis by executing the first `.STEP` command in the
/// netlist, resolving relative includes from the source path when provided.
pub fn run_parametric_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<ParametricData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;

    let step_cmd = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Step(step) => Some(step),
            _ => None,
        })
        .ok_or_else(|| "Parametric analysis requires a .STEP command in the netlist".to_string())?;

    let values = expand_step_sweep_values(&step_cmd.sweep).map_err(|err| err.to_string())?;
    if values.is_empty() {
        return Err("Parametric analysis has no sweep points to execute".to_string());
    }

    let results = if step_cmd.target == StepTarget::Temp {
        let cfg = TempRunConfig {
            temperatures_c: values.clone(),
            base_mode: CornerBaseMode::Op,
        };
        return run_parametric_analysis_with_netlist_and_config(&netlist, &cfg, "TEMP");
    } else {
        let engine = Engine::new(build_engine_config(&netlist, None));
        engine
            .run_step_command(&netlist, step_cmd, &values)
            .map_err(|e| format!("Parametric analysis error: {}", e))?
    };

    if results.is_empty() {
        return Err("Parametric analysis produced no converged sweep points".to_string());
    }

    let num_failures = values.len().saturating_sub(results.len());
    let (sweep_values, voltages) = map_dc_sweep_results(&results);

    Ok(ParametricData {
        target: describe_step_target(step_cmd),
        num_points: sweep_values.len(),
        sweep_values,
        voltages,
        num_failures,
    })
}

/// Run temperature sweep analysis with explicit base-mode configuration.
pub fn run_parametric_analysis_with_config(
    netlist_text: &str,
    config: &TempRunConfig,
) -> Result<ParametricData, String> {
    run_parametric_analysis_with_config_and_source_path(netlist_text, config, None)
}

/// Run temperature sweep analysis with explicit base-mode configuration and a
/// source path used to resolve relative includes and model file references.
pub fn run_parametric_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &TempRunConfig,
    source_path: Option<&Path>,
) -> Result<ParametricData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;
    run_parametric_analysis_with_netlist_and_config(&netlist, config, "TEMP")
}

fn run_parametric_analysis_with_netlist_and_config(
    netlist: &rspice_core::Netlist,
    config: &TempRunConfig,
    target: &str,
) -> Result<ParametricData, String> {
    config.validate()?;

    let results = run_temperature_sweep(netlist, &config.temperatures_c, &config.base_mode)?;
    if results.is_empty() {
        return Err("Parametric analysis produced no converged sweep points".to_string());
    }

    let num_failures = config.temperatures_c.len().saturating_sub(results.len());
    let metric_label = config.base_mode.metric_label();
    let (sweep_values, voltages) = map_temperature_results(&results, metric_label);

    Ok(ParametricData {
        target: target.to_string(),
        num_points: sweep_values.len(),
        sweep_values,
        voltages,
        num_failures,
    })
}

// =============================================================================
// Corner Analysis
// =============================================================================

/// Process-corner designation for UI corner sweeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerProcess {
    TT,
    SS,
    FF,
    SF,
    FS,
}

impl CornerProcess {
    fn as_keyword(self) -> &'static str {
        match self {
            Self::TT => "TT",
            Self::SS => "SS",
            Self::FF => "FF",
            Self::SF => "SF",
            Self::FS => "FS",
        }
    }

    fn nmos_factor(self) -> Value {
        match self {
            Self::TT => 1.0,
            Self::SS | Self::SF => 0.9,
            Self::FF | Self::FS => 1.1,
        }
    }

    fn pmos_factor(self) -> Value {
        match self {
            Self::TT => 1.0,
            Self::SS | Self::FS => 0.9,
            Self::FF | Self::SF => 1.1,
        }
    }
}

/// Explicit configuration for corner sweep execution.
#[derive(Debug, Clone)]
pub struct CornerRunConfig {
    pub process_corners: Vec<CornerProcess>,
    pub voltages: Vec<Value>,
    pub temperatures_c: Vec<Value>,
    pub full_matrix: bool,
    pub nominal_voltage: Option<Value>,
    pub base_mode: CornerBaseMode,
}

/// Frequency sweep type used by corner AC base analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl CornerFrequencySweep {
    fn as_keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Base analysis executed at each corner point.
#[derive(Debug, Clone, Default)]
pub enum CornerBaseMode {
    /// Run DC operating point directly at each corner.
    #[default]
    Op,
    /// Run DC sweep and record the final converged point at each corner.
    DcSweep {
        source_name: String,
        start: Value,
        stop: Value,
        step: Value,
    },
    /// Run transient analysis and record the terminal sample at each corner.
    Transient { stop_time: Value, step_time: Value },
    /// Run AC analysis and record terminal-frequency magnitude at each corner.
    Ac {
        start_freq: Value,
        stop_freq: Value,
        points_per_unit: usize,
        sweep: CornerFrequencySweep,
    },
}

impl CornerBaseMode {
    fn metric_label(&self) -> CornerMetricLabel {
        match self {
            Self::Ac { .. } => CornerMetricLabel::AcMagnitude,
            _ => CornerMetricLabel::Voltage,
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Self::Op => "OP",
            Self::DcSweep { .. } => "DC",
            Self::Transient { .. } => "TRAN",
            Self::Ac { .. } => "AC",
        }
    }
}

impl Default for CornerRunConfig {
    fn default() -> Self {
        Self {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![1.0],
            temperatures_c: vec![25.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::default(),
        }
    }
}

impl CornerRunConfig {
    fn validate(&self) -> Result<(), String> {
        if self.process_corners.is_empty() {
            return Err("Corner analysis requires at least one process corner".to_string());
        }
        if self.voltages.is_empty() {
            return Err("Corner analysis requires at least one voltage corner".to_string());
        }
        if self.voltages.iter().any(|v| !v.is_finite() || *v <= 0.0) {
            return Err(
                "Corner analysis voltage corners must be positive finite values".to_string(),
            );
        }
        if self.temperatures_c.is_empty() {
            return Err("Corner analysis requires at least one temperature corner".to_string());
        }
        if self.temperatures_c.iter().any(|t| !t.is_finite()) {
            return Err("Corner analysis temperature corners must be finite values".to_string());
        }
        if let Some(vnom) = self.nominal_voltage
            && (!vnom.is_finite() || vnom <= 0.0)
        {
            return Err(
                "Corner analysis nominal voltage must be a positive finite value".to_string(),
            );
        }
        validate_base_mode("Corner", &self.base_mode)?;
        Ok(())
    }
}

fn validate_base_mode(context: &str, base_mode: &CornerBaseMode) -> Result<(), String> {
    match base_mode {
        CornerBaseMode::Op => {}
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            if source_name.trim().is_empty() {
                return Err(format!(
                    "{} DC sweep base mode requires a non-empty source name",
                    context
                ));
            }
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err(format!(
                    "{} DC sweep base mode requires finite start/stop/step values",
                    context
                ));
            }
            if *step == 0.0 {
                return Err(format!(
                    "{} DC sweep base mode step cannot be zero",
                    context
                ));
            }
            if (stop - start).abs() > 0.0 && (stop - start).signum() != step.signum() {
                return Err(format!(
                    "{} DC sweep base mode step direction must match start/stop range",
                    context
                ));
            }
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            if !stop_time.is_finite() || *stop_time <= 0.0 {
                return Err(format!(
                    "{} transient base mode stop_time must be a positive finite value",
                    context
                ));
            }
            if !step_time.is_finite() || *step_time <= 0.0 {
                return Err(format!(
                    "{} transient base mode step_time must be a positive finite value",
                    context
                ));
            }
            if step_time > stop_time {
                return Err(format!(
                    "{} transient base mode step_time must be <= stop_time",
                    context
                ));
            }
        }
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            ..
        } => {
            if !start_freq.is_finite() || !stop_freq.is_finite() {
                return Err(format!(
                    "{} AC base mode requires finite start/stop frequencies",
                    context
                ));
            }
            if *start_freq <= 0.0 || *stop_freq <= 0.0 {
                return Err(format!(
                    "{} AC base mode frequencies must be positive values",
                    context
                ));
            }
            if stop_freq < start_freq {
                return Err(format!(
                    "{} AC base mode stop frequency must be >= start frequency",
                    context
                ));
            }
            if *points_per_unit == 0 {
                return Err(format!(
                    "{} AC base mode points_per_unit must be greater than zero",
                    context
                ));
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct CornerPoint {
    process: CornerProcess,
    voltage: Value,
    temperature_c: Value,
}

impl CornerPoint {
    fn label(&self) -> String {
        format!(
            "{}_{:.6}V_{:.6}C",
            self.process.as_keyword(),
            self.voltage,
            self.temperature_c
        )
    }
}

#[derive(Debug, Clone)]
struct SweepPointResult {
    node_names: Vec<String>,
    node_values: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CornerMetricLabel {
    Voltage,
    AcMagnitude,
}

impl CornerMetricLabel {
    fn format_trace_name(self, node_name: &str) -> String {
        match self {
            Self::Voltage => format!("V({})", node_name),
            Self::AcMagnitude => format!("|V({})|", node_name),
        }
    }
}

/// Temperature/process/voltage corner sweep data.
#[derive(Debug, Clone)]
pub struct CornerData {
    /// X-axis values for each executed corner point.
    pub x_values: Vec<Value>,
    /// X-axis label for corner traces.
    pub x_label: String,
    /// X-axis unit for corner traces.
    pub x_unit: String,
    /// Temperature for each executed corner point.
    pub temperatures_c: Vec<Value>,
    /// Human-readable corner labels in execution order.
    pub corner_labels: Vec<String>,
    /// Per-node values for each corner point.
    pub voltages: Vec<(String, Vec<Value>)>,
    pub num_points: usize,
    pub num_failures: usize,
}

/// Run corner analysis from `.TEMP` commands in the netlist.
///
/// This compatibility entry point executes temperature-only TT/nominal sweeps.
pub fn run_corner_analysis(netlist_text: &str) -> Result<CornerData, String> {
    run_corner_analysis_with_source_path(netlist_text, None)
}

/// Run corner analysis from `.TEMP` commands in the netlist, resolving relative
/// includes from the source path when provided.
pub fn run_corner_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;
    let temperatures = extract_temp_points(&netlist);

    if temperatures.is_empty() {
        return Err("Corner analysis requires at least one .TEMP command".to_string());
    }

    let config = CornerRunConfig {
        temperatures_c: temperatures,
        ..Default::default()
    };
    run_corner_analysis_with_netlist(&netlist, &config)
}

/// Run corner analysis with explicit process/voltage/temperature configuration.
pub fn run_corner_analysis_with_config(
    netlist_text: &str,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    run_corner_analysis_with_config_and_source_path(netlist_text, config, None)
}

/// Run corner analysis with explicit process/voltage/temperature configuration
/// and a source path used to resolve relative includes and model file
/// references.
pub fn run_corner_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &CornerRunConfig,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;
    run_corner_analysis_with_netlist(&netlist, config)
}

fn run_corner_analysis_with_netlist(
    netlist: &rspice_core::Netlist,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    config.validate()?;
    let points = expand_corner_points(config);
    if points.is_empty() {
        return Err("Corner analysis produced no corner points".to_string());
    }

    let nominal_voltage = config
        .nominal_voltage
        .or_else(|| infer_nominal_supply_voltage(netlist))
        .unwrap_or(1.0);
    let results = run_corner_sweep(netlist, &points, config, nominal_voltage)?;
    if results.is_empty() {
        return Err("Corner analysis produced no converged corner points".to_string());
    }

    let num_failures = points.len().saturating_sub(results.len());
    let metric = config.base_mode.metric_label();
    let (x_values, x_label, x_unit, temperatures_c, corner_labels, voltages) =
        map_corner_results(&results, metric);

    Ok(CornerData {
        x_values,
        x_label,
        x_unit,
        num_points: temperatures_c.len(),
        temperatures_c,
        corner_labels,
        voltages,
        num_failures,
    })
}

// =============================================================================
// Helper functions
// =============================================================================

fn describe_step_target(step_cmd: &StepCommand) -> String {
    match step_cmd.target {
        StepTarget::Param => format!("PARAM {}", step_cmd.name),
        StepTarget::Device => match step_cmd.param_name.as_deref() {
            Some(param) => format!("DEVICE {}.{}", step_cmd.name, param),
            None => format!("DEVICE {}", step_cmd.name),
        },
        StepTarget::Model => {
            let param = step_cmd.param_name.as_deref().unwrap_or("PARAM");
            format!("MODEL {}.{}", step_cmd.name, param)
        }
        StepTarget::Temp => "TEMP".to_string(),
    }
}

fn map_dc_sweep_results(
    results: &[(Value, CoreSimulationResult)],
) -> (Vec<Value>, Vec<(String, Vec<Value>)>) {
    let sweep_values: Vec<Value> = results.iter().map(|(value, _)| *value).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_voltages.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_voltages.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((format!("V({})", node_name), values));
        }
    }

    (sweep_values, voltages)
}

fn map_temperature_results(
    results: &[(Value, SweepPointResult)],
    metric_label: CornerMetricLabel,
) -> (Vec<Value>, Vec<(String, Vec<Value>)>) {
    let sweep_values: Vec<Value> = results.iter().map(|(temp_c, _)| *temp_c).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_values.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_values.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    (sweep_values, voltages)
}

fn map_corner_results(
    results: &[(CornerPoint, SweepPointResult)],
    metric_label: CornerMetricLabel,
) -> (
    Vec<Value>,
    String,
    String,
    Vec<Value>,
    Vec<String>,
    Vec<(String, Vec<Value>)>,
) {
    let temperatures_c: Vec<Value> = results
        .iter()
        .map(|(point, _)| point.temperature_c)
        .collect();
    let (x_values, x_label, x_unit) = corner_axis_from_points(results, &temperatures_c);
    let corner_labels: Vec<String> = results.iter().map(|(point, _)| point.label()).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_values.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_values.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    (
        x_values,
        x_label,
        x_unit,
        temperatures_c,
        corner_labels,
        voltages,
    )
}

fn corner_axis_from_points(
    results: &[(CornerPoint, SweepPointResult)],
    temperatures_c: &[Value],
) -> (Vec<Value>, String, String) {
    if results.is_empty() {
        return (Vec::new(), "Corner Index".to_string(), String::new());
    }

    let first_point = &results[0].0;
    let single_process = results
        .iter()
        .all(|(point, _)| point.process == first_point.process);
    let single_voltage = results
        .iter()
        .all(|(point, _)| (point.voltage - first_point.voltage).abs() < 1e-15);

    let mut seen_temps = std::collections::HashSet::with_capacity(temperatures_c.len());
    let has_duplicate_temp = temperatures_c
        .iter()
        .any(|temperature| !seen_temps.insert(temperature.to_bits()));

    if single_process && single_voltage && !has_duplicate_temp {
        return (
            temperatures_c.to_vec(),
            "Temperature".to_string(),
            "C".to_string(),
        );
    }

    (
        (0..results.len()).map(|index| index as Value).collect(),
        "Corner Index".to_string(),
        String::new(),
    )
}

fn expand_corner_points(config: &CornerRunConfig) -> Vec<CornerPoint> {
    if config.full_matrix {
        let mut points = Vec::with_capacity(
            config.process_corners.len() * config.voltages.len() * config.temperatures_c.len(),
        );
        for process in &config.process_corners {
            for &voltage in &config.voltages {
                for &temperature_c in &config.temperatures_c {
                    points.push(CornerPoint {
                        process: *process,
                        voltage,
                        temperature_c,
                    });
                }
            }
        }
        return points;
    }

    let n = config
        .process_corners
        .len()
        .max(config.voltages.len())
        .max(config.temperatures_c.len());
    let mut points = Vec::with_capacity(n);
    for idx in 0..n {
        points.push(CornerPoint {
            process: config.process_corners[idx % config.process_corners.len()],
            voltage: config.voltages[idx % config.voltages.len()],
            temperature_c: config.temperatures_c[idx % config.temperatures_c.len()],
        });
    }
    points
}

fn run_corner_sweep(
    netlist: &rspice_core::Netlist,
    points: &[CornerPoint],
    config: &CornerRunConfig,
    nominal_voltage: Value,
) -> Result<Vec<(CornerPoint, SweepPointResult)>, String> {
    if !nominal_voltage.is_finite() || nominal_voltage <= 0.0 {
        return Err("Corner analysis nominal voltage must be a positive finite value".to_string());
    }

    let mut results = Vec::with_capacity(points.len());

    for point in points {
        if !point.voltage.is_finite() || point.voltage <= 0.0 {
            return Err(format!(
                "Corner voltage must be positive and finite (got {})",
                point.voltage
            ));
        }
        if !point.temperature_c.is_finite() {
            return Err(format!(
                "Corner temperature must be finite (got {})",
                point.temperature_c
            ));
        }

        let mut corner_netlist = netlist.clone();
        apply_process_corner(&mut corner_netlist, point.process);
        apply_voltage_corner(&mut corner_netlist, point.voltage, nominal_voltage)?;

        let mut sim_config = build_engine_config(&corner_netlist, None);
        sim_config.temperature = point.temperature_c + 273.15;
        let engine = Engine::new(sim_config);

        match run_base_mode_point(&engine, &corner_netlist, &config.base_mode) {
            Ok(result) => results.push((point.clone(), result)),
            Err(e) => {
                log::warn!(
                    "Corner {} ({}) failed: {}",
                    point.label(),
                    config.base_mode.display_name(),
                    e
                );
            }
        }
    }

    Ok(results)
}

fn run_base_mode_point(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    base_mode: &CornerBaseMode,
) -> Result<SweepPointResult, String> {
    match base_mode {
        CornerBaseMode::Op => engine
            .run_dc_op(netlist)
            .map(|dc| sweep_point_result_from_dc(&dc))
            .map_err(|e| format!("DC operating point error: {}", e)),
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            let results = engine
                .run_dc_sweep(netlist, source_name, *start, *stop, *step)
                .map_err(|e| format!("DC sweep error: {}", e))?;
            let (_, terminal) = results
                .last()
                .ok_or_else(|| "DC sweep produced no points".to_string())?;
            Ok(sweep_point_result_from_dc(terminal))
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            let result = engine
                .run_tran(netlist, *stop_time, *step_time)
                .map_err(|e| format!("Transient analysis error: {}", e))?;
            sweep_point_result_from_transient(result)
        }
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => run_base_mode_ac_point(
            engine,
            netlist,
            *start_freq,
            *stop_freq,
            *points_per_unit,
            *sweep,
        ),
    }
}

fn sweep_point_result_from_dc(result: &CoreSimulationResult) -> SweepPointResult {
    SweepPointResult {
        node_names: result.node_names.clone(),
        node_values: result.node_voltages.clone(),
    }
}

fn sweep_point_result_from_transient(result: TransientResult) -> Result<SweepPointResult, String> {
    if result.time.is_empty() {
        return Err("Transient analysis produced no time points".to_string());
    }
    if result.node_names.is_empty() {
        return Err("Transient analysis returned no node names".to_string());
    }

    let mut node_values = Vec::with_capacity(result.node_names.len());
    for (idx, node_name) in result.node_names.iter().enumerate() {
        let Some(waveform) = result.voltages.get(idx) else {
            return Err(format!(
                "Transient result missing waveform for node '{}'",
                node_name
            ));
        };
        let Some(value) = waveform.last().copied() else {
            return Err(format!(
                "Transient waveform for node '{}' contains no samples",
                node_name
            ));
        };
        node_values.push(value);
    }

    Ok(SweepPointResult {
        node_names: result.node_names,
        node_values,
    })
}

fn run_base_mode_ac_point(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    start_freq: Value,
    stop_freq: Value,
    points_per_unit: usize,
    sweep: CornerFrequencySweep,
) -> Result<SweepPointResult, String> {
    let frequencies =
        generate_freq_points(start_freq, stop_freq, points_per_unit, sweep.as_keyword());
    if frequencies.is_empty() {
        return Err("Corner AC base mode generated no frequency points".to_string());
    }

    let dc_result = engine
        .run_dc_op(netlist)
        .map_err(|e| format!("DC OP error (required for AC): {}", e))?;
    let node_names = dc_result.node_names;

    let ac_results = engine
        .run_ac(netlist, &frequencies)
        .map_err(|e| format!("AC analysis error: {}", e))?;
    let terminal = ac_results
        .last()
        .ok_or_else(|| "AC analysis produced no points".to_string())?;

    let mut node_values = vec![0.0; node_names.len()];
    for node_idx in 1..node_names.len() {
        let ac_idx = node_idx.saturating_sub(1);
        node_values[node_idx] = terminal
            .voltages
            .get(ac_idx)
            .map(|value| value.norm())
            .unwrap_or(0.0);
    }

    Ok(SweepPointResult {
        node_names,
        node_values,
    })
}

fn apply_process_corner(netlist: &mut rspice_core::Netlist, process: CornerProcess) {
    let nmos_factor = process.nmos_factor();
    let pmos_factor = process.pmos_factor();

    for model in &mut netlist.models {
        let factor = process_factor_for_model_type(&model.model_type, nmos_factor, pmos_factor);
        if (factor - 1.0).abs() < 1e-15 {
            continue;
        }
        for (param_name, param_value) in &mut model.params {
            if is_mobility_like_model_param(param_name) {
                *param_value *= factor;
            }
        }
    }
}

fn process_factor_for_model_type(
    model_type: &str,
    nmos_factor: Value,
    pmos_factor: Value,
) -> Value {
    let ty = model_type.trim().to_ascii_uppercase();
    if ty.contains("PMOS") || ty.contains("PJF") || ty.contains("PMF") || ty.contains("PNP") {
        pmos_factor
    } else if ty.contains("NMOS") || ty.contains("NJF") || ty.contains("NMF") || ty.contains("NPN")
    {
        nmos_factor
    } else {
        (nmos_factor + pmos_factor) * 0.5
    }
}

fn is_mobility_like_model_param(param_name: &str) -> bool {
    let upper = param_name.trim().to_ascii_uppercase();
    matches!(
        upper.as_str(),
        "KP" | "BETA" | "U0" | "UO" | "MU" | "MOBILITY" | "KP0" | "KP1"
    )
}

fn apply_voltage_corner(
    netlist: &mut rspice_core::Netlist,
    corner_voltage: Value,
    nominal_voltage: Value,
) -> Result<(), String> {
    if !corner_voltage.is_finite() || corner_voltage <= 0.0 {
        return Err("Corner voltage must be a positive finite value".to_string());
    }
    if !nominal_voltage.is_finite() || nominal_voltage <= 0.0 {
        return Err("Corner nominal voltage must be a positive finite value".to_string());
    }
    let scale = corner_voltage / nominal_voltage;

    let mut candidate_indices = Vec::new();
    for (idx, element) in netlist.elements.iter().enumerate() {
        let Some(neg) = element.nodes.get(1) else {
            continue;
        };
        if !is_ground_node(neg) {
            continue;
        }
        if let ElementKind::VoltageSource(spec) = &element.kind
            && dc_value_from_source(spec).is_some()
        {
            candidate_indices.push(idx);
        }
    }

    if candidate_indices.is_empty() {
        for (idx, element) in netlist.elements.iter().enumerate() {
            if let ElementKind::VoltageSource(spec) = &element.kind
                && dc_value_from_source(spec).is_some()
            {
                candidate_indices.push(idx);
            }
        }
    }

    for idx in candidate_indices {
        let Some(element) = netlist.elements.get_mut(idx) else {
            continue;
        };
        if let ElementKind::VoltageSource(spec) = &mut element.kind
            && let Some(dc) = dc_value_from_source(spec)
        {
            let _ = set_dc_value_for_source(spec, dc * scale);
        }
    }

    Ok(())
}

fn infer_nominal_supply_voltage(netlist: &rspice_core::Netlist) -> Option<Value> {
    let mut ground_referenced = Vec::new();
    let mut all_sources = Vec::new();

    for element in &netlist.elements {
        if let ElementKind::VoltageSource(spec) = &element.kind
            && let Some(dc) = dc_value_from_source(spec)
        {
            let abs_dc = dc.abs();
            if abs_dc <= 1e-15 {
                continue;
            }
            all_sources.push(abs_dc);
            if element
                .nodes
                .get(1)
                .map(|name| is_ground_node(name))
                .unwrap_or(false)
            {
                ground_referenced.push(abs_dc);
            }
        }
    }

    if !ground_referenced.is_empty() {
        return ground_referenced.into_iter().max_by(|a, b| a.total_cmp(b));
    }
    all_sources.into_iter().max_by(|a, b| a.total_cmp(b))
}

fn is_ground_node(node: &str) -> bool {
    let n = node.trim();
    n == "0" || n.eq_ignore_ascii_case("gnd") || n.eq_ignore_ascii_case("ground")
}

fn dc_value_from_source(spec: &SourceSpec) -> Option<Value> {
    match spec {
        SourceSpec::Dc(v) => Some(*v),
        SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}

fn set_dc_value_for_source(spec: &mut SourceSpec, value: Value) -> bool {
    match spec {
        SourceSpec::Dc(v) => {
            *v = value;
            true
        }
        SourceSpec::DcAc { dc_value, .. } => {
            *dc_value = value;
            true
        }
        _ => false,
    }
}

fn run_temperature_sweep(
    netlist: &rspice_core::Netlist,
    temperatures_c: &[Value],
    base_mode: &CornerBaseMode,
) -> Result<Vec<(Value, SweepPointResult)>, String> {
    let mut results = Vec::with_capacity(temperatures_c.len());

    for &temp_c in temperatures_c {
        if !temp_c.is_finite() {
            return Err("Temperature sweep contains non-finite value".to_string());
        }

        let mut config = build_engine_config(netlist, None);
        config.temperature = temp_c + 273.15;
        let engine = Engine::new(config);

        match run_base_mode_point(&engine, netlist, base_mode) {
            Ok(point_result) => results.push((temp_c, point_result)),
            Err(e) => {
                log::warn!(
                    "Temperature corner {}C ({}) failed: {}",
                    temp_c,
                    base_mode.display_name(),
                    e
                );
            }
        }
    }

    Ok(results)
}
