//! Lossless QPAC editor buffers and one validated specification boundary.
use crate::analysis_spec::{AnalysisSpec, QpacControls};
use crate::config::FrequencySweep;
use crate::drafts::FrequencySweepDraft;
use crate::drafts::parse::{parse_i32_tuple, parse_positive_usize};
use rspice_core::analysis::quasi_periodic::{
    QuasiPeriodicAcConfig, QuasiPeriodicLinearConfig, QuasiPeriodicLinearMethod,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QuasiPeriodicAcDraft {
    pub sweep: FrequencySweepDraft,
    pub explicit_offsets: String,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: String,
    pub input_lattice: String,
    pub output_lattice: String,
    pub magnitude: String,
    pub phase_degrees: String,
    pub linear_method: QuasiPeriodicLinearMethod,
    pub krylov_restart: String,
    pub krylov_cycles: String,
    pub linear_tolerance: String,
    pub current_absolute_tolerance: String,
    pub voltage_absolute_tolerance: String,
}

impl Default for QuasiPeriodicAcDraft {
    fn default() -> Self {
        Self {
            sweep: FrequencySweepDraft::default(),
            explicit_offsets: String::new(),
            input_source: "V1".into(),
            output_node: "out".into(),
            output_ref: "0".into(),
            input_lattice: "0, 0".into(),
            output_lattice: "0, 0".into(),
            magnitude: "1".into(),
            phase_degrees: "0".into(),
            linear_method: QuasiPeriodicLinearMethod::Auto,
            krylov_restart: "32".into(),
            krylov_cycles: "20".into(),
            linear_tolerance: "1e-10".into(),
            current_absolute_tolerance: "1e-12".into(),
            voltage_absolute_tolerance: "1e-9".into(),
        }
    }
}

fn value(text: &str, field: &str) -> Result<f64, String> {
    let value =
        crate::options::parse_si_value(text).map_err(|e| format!("invalid {field}: {e}"))?;
    if !value.is_finite() {
        return Err(format!("{field} must be finite"));
    }
    Ok(value)
}

impl QuasiPeriodicAcDraft {
    pub fn to_spec(&self) -> Result<AnalysisSpec, String> {
        let (start_freq, stop_freq, points_per_unit, sweep, explicit_offsets) =
            if self.explicit_offsets.trim().is_empty() {
                (
                    value(&self.sweep.start, "QPAC start frequency")?,
                    value(&self.sweep.stop, "QPAC stop frequency")?,
                    parse_positive_usize(&self.sweep.points, "QPAC frequency points")?,
                    match self.sweep.sweep {
                        0 => FrequencySweep::Decade,
                        1 => FrequencySweep::Octave,
                        2 => FrequencySweep::Linear,
                        _ => return Err("QPAC frequency sweep mode is invalid".into()),
                    },
                    None,
                )
            } else {
                let offsets = self
                    .explicit_offsets
                    .split(|c: char| c == ',' || c.is_whitespace())
                    .filter(|v| !v.is_empty())
                    .map(|v| value(v, "QPAC probe offset"))
                    .collect::<Result<Vec<_>, _>>()?;
                (0.0, 0.0, 1, FrequencySweep::Linear, Some(offsets))
            };
        let defaults = QuasiPeriodicLinearConfig::default();
        let iterative = self.linear_method != QuasiPeriodicLinearMethod::Direct;
        let controls = QpacControls {
            magnitude: value(&self.magnitude, "QPAC magnitude")?,
            phase_degrees: value(&self.phase_degrees, "QPAC phase")?,
            explicit_offsets,
            solver: QuasiPeriodicAcConfig {
                linear: QuasiPeriodicLinearConfig {
                    method: self.linear_method,
                    restart: if iterative {
                        parse_positive_usize(&self.krylov_restart, "QPAC Krylov restart")?
                    } else {
                        defaults.restart
                    },
                    max_cycles: if iterative {
                        parse_positive_usize(&self.krylov_cycles, "QPAC Krylov cycles")?
                    } else {
                        defaults.max_cycles
                    },
                    relative_tolerance: value(&self.linear_tolerance, "QPAC relative tolerance")?,
                },
                current_absolute_tolerance: value(
                    &self.current_absolute_tolerance,
                    "QPAC current tolerance",
                )?,
                voltage_absolute_tolerance: value(
                    &self.voltage_absolute_tolerance,
                    "QPAC voltage tolerance",
                )?,
            },
        };
        let spec = AnalysisSpec::Qpac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            input_source: self.input_source.trim().into(),
            output_node: self.output_node.trim().into(),
            output_ref: self.output_ref.trim().into(),
            input_lattice: parse_i32_tuple(&self.input_lattice, "QPAC input tuple")?,
            output_lattice: parse_i32_tuple(&self.output_lattice, "QPAC output tuple")?,
            controls,
        };
        spec.validate()?;
        Ok(spec)
    }
}
