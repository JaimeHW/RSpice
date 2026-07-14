use rspice_core::Value;

pub(crate) const REFERENCE_MODEL_BINDING_BEGIN: &str = "* RSPICE REFERENCE MODEL BINDING BEGIN";
pub(crate) const REFERENCE_MODEL_BINDING_END: &str = "* RSPICE REFERENCE MODEL BINDING END";

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
    pub(super) fn validate(&self) -> Result<(), String> {
        if self.temperatures_c.is_empty() {
            return Err("Temperature sweep requires at least one temperature point".to_string());
        }
        if self.temperatures_c.iter().any(|t| !t.is_finite()) {
            return Err("Temperature sweep points must be finite values".to_string());
        }
        validate_base_mode("Temperature sweep", &self.base_mode)
    }
}

/// Process-corner designation for UI corner sweeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CornerProcess {
    TT,
    SS,
    FF,
    SF,
    FS,
}

/// One explicit foundry/library model binding for a process point.
///
/// `section = Some(..)` represents `.lib <path> <section>`; `None`
/// represents a process-independent `.include <path>`. Paths remain data and
/// are validated before being rendered into a SPICE directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CornerModelBinding {
    pub process: CornerProcess,
    pub library_path: String,
    pub section: Option<String>,
}

impl CornerModelBinding {
    pub(crate) fn validate(&self) -> Result<(), String> {
        let path = self.library_path.trim();
        if path.is_empty() {
            return Err("Corner model binding requires a library path".to_owned());
        }
        if path.chars().any(|character| {
            character == '"' || character == '\0' || character == '\r' || character == '\n'
        }) {
            return Err(format!(
                "Corner model library path contains a character that cannot be represented safely: {path}"
            ));
        }
        if let Some(section) = self.section.as_deref() {
            let section = section.trim();
            if section.is_empty() {
                return Err("Corner model section cannot be empty".to_owned());
            }
            if section.chars().any(|character| {
                character.is_whitespace()
                    || character == '"'
                    || character == '\''
                    || character.is_control()
            }) {
                return Err(format!(
                    "Corner model section contains an unsupported character: {section}"
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn spice_directive(&self) -> String {
        match self.section.as_deref() {
            Some(section) => format!(".lib \"{}\" {section}", self.library_path.trim()),
            None => format!(".include \"{}\"", self.library_path.trim()),
        }
    }
}

impl CornerProcess {
    pub(crate) fn as_keyword(self) -> &'static str {
        match self {
            Self::TT => "TT",
            Self::SS => "SS",
            Self::FF => "FF",
            Self::SF => "SF",
            Self::FS => "FS",
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
    /// Explicit model sources keyed to process. Non-typical corners must be
    /// backed by a real library section; RSpice never fabricates a foundry
    /// corner by scaling model parameters heuristically.
    pub model_bindings: Vec<CornerModelBinding>,
}

/// Frequency sweep type used by corner AC base analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl CornerFrequencySweep {
    pub(super) fn as_keyword(self) -> &'static str {
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
    pub(super) fn metric_label(&self) -> CornerMetricLabel {
        match self {
            Self::Ac { .. } => CornerMetricLabel::AcMagnitude,
            _ => CornerMetricLabel::Voltage,
        }
    }

    pub(super) fn display_name(&self) -> &'static str {
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
            model_bindings: Vec::new(),
        }
    }
}

impl CornerRunConfig {
    pub(super) fn validate(&self) -> Result<(), String> {
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
        for binding in &self.model_bindings {
            binding.validate()?;
        }
        for process in &self.process_corners {
            if *process != CornerProcess::TT
                && !self
                    .model_bindings
                    .iter()
                    .any(|binding| binding.process == *process && binding.section.is_some())
            {
                return Err(format!(
                    "{} corner requires an explicit PDK model-library section",
                    process.as_keyword()
                ));
            }
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
pub(super) struct CornerPoint {
    pub(super) process: CornerProcess,
    pub(super) voltage: Value,
    pub(super) temperature_c: Value,
}

impl CornerPoint {
    pub(super) fn label(&self) -> String {
        format!(
            "{}_{:.6}V_{:.6}C",
            self.process.as_keyword(),
            self.voltage,
            self.temperature_c
        )
    }
}

#[derive(Debug, Clone)]
pub(super) struct SweepPointResult {
    pub(super) node_names: Vec<String>,
    pub(super) node_values: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CornerMetricLabel {
    Voltage,
    AcMagnitude,
}

impl CornerMetricLabel {
    pub(super) fn format_trace_name(self, node_name: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_typical_corner_requires_real_model_section() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::FF],
            ..CornerRunConfig::default()
        };

        let error = config
            .validate()
            .expect_err("synthetic process scaling must not be accepted");

        assert!(error.contains("explicit PDK model-library section"));
    }

    #[test]
    fn binding_rejects_directive_injection_characters() {
        let binding = CornerModelBinding {
            process: CornerProcess::FF,
            library_path: "models.lib\n.end".to_owned(),
            section: Some("ff".to_owned()),
        };

        assert!(binding.validate().is_err());
    }
}
