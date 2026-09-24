//! Persisted waveform, device-report, and measurement values.

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectWaveformData {
    pub name: String,
    /// Exact samples share storage with immutable snapshots. A writer must
    /// detach through copy-on-write before changing a draft's samples.
    pub x: crate::state::SharedWaveformValues,
    pub y: crate::state::SharedWaveformValues,
    pub color: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    /// The unit the samples are measured in, introduced by schema v13.
    /// Absent in every earlier project, and absent in a current one whose
    /// producer stated no unit — the two are the same fact, so both read
    /// back as unstated rather than as a fabricated default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<ProjectComplexWaveformComponents>,
}

impl ProjectWaveformData {
    pub(super) fn into_waveform(self) -> WaveformData {
        let mut waveform = WaveformData::new(self.name, self.x, self.y, self.color);
        waveform.visible = self.visible;
        waveform.unit = self.unit;
        if let Some(complex) = self.complex {
            waveform =
                waveform.with_complex_components(complex.source_name, complex.real, complex.imag);
        }
        waveform
    }

    pub(super) fn validate(&self, prefix: &str) -> Result<(), String> {
        if self.x.len() != self.y.len() {
            return Err(format!(
                "{prefix} has mismatched x/y sample counts ({} vs {})",
                self.x.len(),
                self.y.len()
            ));
        }
        require_finite_values(&self.x, &format!("{prefix}.x"))?;
        require_monotonic_non_decreasing(&self.x, &format!("{prefix}.x"))?;
        require_finite_values(&self.y, &format!("{prefix}.y"))?;
        if let Some(complex) = &self.complex {
            complex.validate(prefix, self.y.len())?;
        }
        Ok(())
    }
}

impl From<&WaveformData> for ProjectWaveformData {
    fn from(waveform: &WaveformData) -> Self {
        Self {
            name: waveform.name.clone(),
            x: waveform.x.clone(),
            y: waveform.y.clone(),
            color: waveform.color.clone(),
            visible: waveform.visible,
            unit: waveform.unit.clone(),
            complex: waveform
                .complex
                .as_ref()
                .map(|complex| ProjectComplexWaveformComponents {
                    source_name: complex.source_name.clone(),
                    real: complex.real.clone(),
                    imag: complex.imag.clone(),
                }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectComplexWaveformComponents {
    pub source_name: String,
    pub real: crate::state::SharedWaveformValues,
    pub imag: crate::state::SharedWaveformValues,
}

impl ProjectComplexWaveformComponents {
    fn validate(&self, prefix: &str, expected_len: usize) -> Result<(), String> {
        if self.real.len() != self.imag.len() || self.real.len() != expected_len {
            return Err(format!(
                "{prefix}.complex has mismatched real/imag/display sample counts"
            ));
        }
        require_finite_values(&self.real, &format!("{prefix}.complex.real"))?;
        require_finite_values(&self.imag, &format!("{prefix}.complex.imag"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDcOpResult {
    #[serde(default)]
    pub node_voltages: Vec<ProjectOperatingPointValue>,
    #[serde(default)]
    pub branch_currents: Vec<ProjectOperatingPointValue>,
    #[serde(default)]
    pub power_dissipation: Vec<ProjectOperatingPointValue>,
}

impl ProjectDcOpResult {
    pub(super) fn into_dc_op(self) -> DcOpResult {
        DcOpResult {
            node_voltages: self
                .node_voltages
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
            branch_currents: self
                .branch_currents
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
            power_dissipation: self
                .power_dissipation
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
        }
    }

    pub(super) fn validate(&self, prefix: &str) -> Result<(), String> {
        for (idx, value) in self.node_voltages.iter().enumerate() {
            value.validate(&format!("{prefix}.node_voltages[{idx}]"))?;
        }
        for (idx, value) in self.branch_currents.iter().enumerate() {
            value.validate(&format!("{prefix}.branch_currents[{idx}]"))?;
        }
        for (idx, value) in self.power_dissipation.iter().enumerate() {
            value.validate(&format!("{prefix}.power_dissipation[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&DcOpResult> for ProjectDcOpResult {
    fn from(dc_op: &DcOpResult) -> Self {
        Self {
            node_voltages: dc_op
                .node_voltages
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
            branch_currents: dc_op
                .branch_currents
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
            power_dissipation: dc_op
                .power_dissipation
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectOperatingPointValue {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

impl ProjectOperatingPointValue {
    fn into_op_value(self) -> OperatingPointValue {
        OperatingPointValue {
            name: self.name,
            value: self.value,
            unit: self.unit,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_finite(self.value, &format!("{prefix}.value"))
    }
}

impl From<&OperatingPointValue> for ProjectOperatingPointValue {
    fn from(value: &OperatingPointValue) -> Self {
        Self {
            name: value.name.clone(),
            value: value.value,
            unit: value.unit.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNoiseSummary {
    #[serde(default)]
    pub input_quantity: Option<rspice_core::analysis::noise::NoiseInputQuantity>,
    #[serde(default)]
    pub conversion: Option<crate::state::PeriodicNoiseConversionEvidence>,
    #[serde(default)]
    pub noise_figure: Option<std::sync::Arc<crate::state::NoiseFigureEvidence>>,
    #[serde(default)]
    pub rows: Vec<ProjectNoiseContributorRow>,
    #[serde(default)]
    pub total_rms: Option<f64>,
    #[serde(default)]
    pub input_rms: Option<f64>,
    pub band: (f64, f64),
}

impl ProjectNoiseSummary {
    pub(in crate::io::project_io) fn into_noise_summary(self) -> NoiseSummary {
        NoiseSummary {
            input_quantity: self.input_quantity,
            conversion: self.conversion,
            noise_figure: self.noise_figure,
            rows: self
                .rows
                .into_iter()
                .map(ProjectNoiseContributorRow::into_row)
                .collect(),
            total_rms: self.total_rms,
            input_rms: self.input_rms,
            band: self.band,
        }
    }

    pub(super) fn validate(&self, prefix: &str) -> Result<(), String> {
        if let Some(conversion) = &self.conversion {
            conversion.validate(self.band)?;
        }
        if let Some(figure) = &self.noise_figure {
            figure.validate()?;
        }
        if let Some(total_rms) = self.total_rms {
            require_finite(total_rms, &format!("{prefix}.total_rms"))?;
        }
        if let Some(input_rms) = self.input_rms {
            require_finite(input_rms, &format!("{prefix}.input_rms"))?;
        }
        require_finite(self.band.0, &format!("{prefix}.band.0"))?;
        require_finite(self.band.1, &format!("{prefix}.band.1"))?;
        for (idx, row) in self.rows.iter().enumerate() {
            row.validate(&format!("{prefix}.rows[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&NoiseSummary> for ProjectNoiseSummary {
    fn from(summary: &NoiseSummary) -> Self {
        Self {
            input_quantity: summary.input_quantity,
            conversion: summary.conversion.clone(),
            noise_figure: summary.noise_figure.clone(),
            rows: summary
                .rows
                .iter()
                .map(ProjectNoiseContributorRow::from)
                .collect(),
            total_rms: summary.total_rms,
            input_rms: summary.input_rms,
            band: summary.band,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNoiseContributorRow {
    pub device: String,
    pub mechanism: String,
    pub power: f64,
    pub share_pct: f64,
}

impl ProjectNoiseContributorRow {
    fn into_row(self) -> NoiseContributorRow {
        NoiseContributorRow {
            device: self.device,
            mechanism: self.mechanism,
            power: self.power,
            share_pct: self.share_pct,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_noise_mechanism(&self.mechanism, &format!("{prefix}.mechanism"))?;
        require_finite(self.power, &format!("{prefix}.power"))?;
        require_finite(self.share_pct, &format!("{prefix}.share_pct"))
    }
}

impl From<&NoiseContributorRow> for ProjectNoiseContributorRow {
    fn from(row: &NoiseContributorRow) -> Self {
        Self {
            device: row.device.clone(),
            mechanism: row.mechanism.to_string(),
            power: row.power,
            share_pct: row.share_pct,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeviceOpReport {
    #[serde(default)]
    pub entries: Vec<ProjectDeviceOpEntry>,
}

impl ProjectDeviceOpReport {
    pub(super) fn into_report(self) -> rspice_core::circuit::DeviceOpReport {
        rspice_core::circuit::DeviceOpReport {
            entries: self
                .entries
                .into_iter()
                .map(ProjectDeviceOpEntry::into_entry)
                .collect(),
        }
    }

    pub(super) fn validate(&self, prefix: &str) -> Result<(), String> {
        for (idx, entry) in self.entries.iter().enumerate() {
            entry.validate(&format!("{prefix}.entries[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&rspice_core::circuit::DeviceOpReport> for ProjectDeviceOpReport {
    fn from(report: &rspice_core::circuit::DeviceOpReport) -> Self {
        Self {
            entries: report
                .entries
                .iter()
                .map(ProjectDeviceOpEntry::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeviceOpEntry {
    pub name: String,
    pub device_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    pub params: Vec<ProjectNamedValue>,
}

impl ProjectDeviceOpEntry {
    fn into_entry(self) -> rspice_core::circuit::DeviceOpEntry {
        rspice_core::circuit::DeviceOpEntry {
            name: self.name,
            device_kind: intern_static_label(self.device_kind),
            region: self.region.map(intern_static_label),
            params: self
                .params
                .into_iter()
                .map(|param| (intern_static_label(param.name), param.value))
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.device_kind, &format!("{prefix}.device_kind"))?;
        if let Some(region) = &self.region {
            require_static_label(region, &format!("{prefix}.region"))?;
        }
        for (idx, param) in self.params.iter().enumerate() {
            param.validate(&format!("{prefix}.params[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&rspice_core::circuit::DeviceOpEntry> for ProjectDeviceOpEntry {
    fn from(entry: &rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: entry.name.clone(),
            device_kind: entry.device_kind.to_string(),
            region: entry.region.map(str::to_string),
            params: entry
                .params
                .iter()
                .map(|(name, value)| ProjectNamedValue {
                    name: (*name).to_string(),
                    value: *value,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNamedValue {
    pub name: String,
    pub value: f64,
}

impl ProjectNamedValue {
    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.name, &format!("{prefix}.name"))?;
        require_finite(self.value, &format!("{prefix}.value"))
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeasurement {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub failure_limit_exceeded: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_axis: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<rspice_core::analysis::MeasurementUnits>,
}

impl ProjectMeasurement {
    pub(super) fn into_measurement(self) -> rspice_core::MeasureResult {
        rspice_core::MeasureResult {
            name: self.name,
            value: self.value,
            raw_value: self.raw_value,
            error: self.error,
            passed: self.passed,
            expected: self.expected,
            tolerance: self.tolerance,
            failure_limit: self.failure_limit,
            failure_limit_exceeded: self.failure_limit_exceeded,
            event_axis: self.event_axis,
            units: self.units,
        }
    }

    pub(super) fn validate(&self, prefix: &str) -> Result<(), String> {
        if let Some(units) = &self.units {
            units
                .validate()
                .map_err(|error| format!("{prefix}.units: {error}"))?;
        }
        require_optional_finite(self.value, &format!("{prefix}.value"))?;
        require_optional_finite(self.raw_value, &format!("{prefix}.raw_value"))?;
        if self.value.is_some() != self.raw_value.is_some() {
            return Err(format!(
                "{prefix}.raw_value must be present exactly when value is present"
            ));
        }
        require_optional_finite(self.expected, &format!("{prefix}.expected"))?;
        require_optional_finite(self.tolerance, &format!("{prefix}.tolerance"))?;
        require_optional_finite(self.failure_limit, &format!("{prefix}.failure_limit"))?;
        let expected_exceeded = match (self.raw_value, self.failure_limit) {
            (Some(raw_value), Some(limit)) => raw_value.abs() >= limit,
            _ => false,
        };
        if self.failure_limit_exceeded != expected_exceeded {
            return Err(format!(
                "{prefix}.failure_limit_exceeded does not match abs(raw_value) >= failure_limit"
            ));
        }
        if self.failure_limit_exceeded && self.passed {
            return Err(format!(
                "{prefix} cannot pass after its FAILVALUE limit was reached"
            ));
        }
        require_optional_finite(self.event_axis, &format!("{prefix}.event_axis"))?;
        Ok(())
    }
}

impl From<&rspice_core::MeasureResult> for ProjectMeasurement {
    fn from(measurement: &rspice_core::MeasureResult) -> Self {
        Self {
            name: measurement.name.clone(),
            value: measurement.value,
            raw_value: measurement.raw_value,
            error: measurement.error.clone(),
            passed: measurement.passed,
            expected: measurement.expected,
            tolerance: measurement.tolerance,
            failure_limit: measurement.failure_limit,
            failure_limit_exceeded: measurement.failure_limit_exceeded,
            event_axis: measurement.event_axis,
            units: measurement.units.clone(),
        }
    }
}
