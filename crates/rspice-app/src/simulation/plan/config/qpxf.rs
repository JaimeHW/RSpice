//! Lossless QPXF editor buffers with active-only parsing and complete native controls.
use super::*;
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep, QpxfControls};
use rspice_core::analysis::quasi_periodic::{QuasiPeriodicLinearConfig, QuasiPeriodicLinearMethod};
use rspice_core::engine::{QpxfFrequencyAxis, QpxfInputLattices, QpxfSources};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum QpxfSourceSelection {
    #[default]
    Single,
    Named,
    AllIndependent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum QpxfSidebandSelection {
    #[default]
    Single,
    Explicit,
    MaxOrders,
    AllRetained,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QuasiPeriodicTransferDraft {
    pub sweep: FrequencySweepDraft,
    pub explicit_frequencies: String,
    pub frequency_axis: QpxfFrequencyAxis,
    pub source_selection: QpxfSourceSelection,
    pub input_source: String,
    /// One complete name per line; spaces and hierarchy punctuation are preserved.
    pub input_sources: String,
    pub current_output: bool,
    pub output_node: String,
    pub output_ref: String,
    pub output_branch: String,
    pub sideband_selection: QpxfSidebandSelection,
    pub input_lattice: String,
    pub input_lattices: String,
    pub max_orders: String,
    pub output_lattice: String,
    pub group_delay: bool,
    pub group_delay_magnitude_floor: String,
    pub linear_method: QuasiPeriodicLinearMethod,
    pub krylov_restart: String,
    pub krylov_cycles: String,
    pub linear_tolerance: String,
}
impl Default for QuasiPeriodicTransferDraft {
    fn default() -> Self {
        Self {
            sweep: FrequencySweepDraft::default(),
            explicit_frequencies: String::new(),
            frequency_axis: QpxfFrequencyAxis::Output,
            source_selection: QpxfSourceSelection::Single,
            input_source: "V1".into(),
            input_sources: "V1".into(),
            current_output: false,
            output_node: "out".into(),
            output_ref: "0".into(),
            output_branch: "Vprobe".into(),
            sideband_selection: QpxfSidebandSelection::Single,
            input_lattice: "0, 0".into(),
            input_lattices: "0, 0".into(),
            max_orders: "1, 1".into(),
            output_lattice: "0, 0".into(),
            group_delay: false,
            group_delay_magnitude_floor: "0".into(),
            linear_method: QuasiPeriodicLinearMethod::Auto,
            krylov_restart: "32".into(),
            krylov_cycles: "20".into(),
            linear_tolerance: "1e-10".into(),
        }
    }
}
fn value(text: &str, field: &str) -> Result<f64, String> {
    let value = crate::simulation::dialog::options::parse_si_value(text)
        .map_err(|e| format!("invalid {field}: {e}"))?;
    if !value.is_finite() {
        return Err(format!("{field} must be finite"));
    }
    Ok(value)
}
impl QuasiPeriodicTransferDraft {
    pub(crate) fn to_spec(&self) -> Result<AnalysisSpec, String> {
        let (start_freq, stop_freq, points_per_unit, sweep, explicit_frequencies) =
            if self.explicit_frequencies.trim().is_empty() {
                (
                    value(&self.sweep.start, "QPXF start frequency")?,
                    value(&self.sweep.stop, "QPXF stop frequency")?,
                    parse_positive_usize(&self.sweep.points, "QPXF frequency points")?,
                    match self.sweep.sweep {
                        0 => FrequencySweep::Decade,
                        1 => FrequencySweep::Octave,
                        2 => FrequencySweep::Linear,
                        _ => return Err("QPXF frequency sweep mode is invalid".into()),
                    },
                    None,
                )
            } else {
                let values = self
                    .explicit_frequencies
                    .split(|c: char| c == ',' || c.is_whitespace())
                    .filter(|v| !v.is_empty())
                    .map(|v| value(v, "QPXF frequency"))
                    .collect::<Result<Vec<_>, _>>()?;
                (0.0, 0.0, 1, FrequencySweep::Linear, Some(values))
            };
        let (input_source, input_sources) = match self.source_selection {
            QpxfSourceSelection::Single => (self.input_source.trim().into(), None),
            QpxfSourceSelection::AllIndependent => {
                (String::new(), Some(QpxfSources::AllIndependent))
            }
            QpxfSourceSelection::Named => {
                let mut names: Vec<String> = self
                    .input_sources
                    .lines()
                    .map(str::trim)
                    .filter(|v| !v.is_empty())
                    .map(String::from)
                    .collect();
                if names.len() == 1 {
                    (names.remove(0), None)
                } else {
                    (String::new(), Some(QpxfSources::Named(names)))
                }
            }
        };
        let (input_lattice, input_lattices) = match self.sideband_selection {
            QpxfSidebandSelection::Single => (
                parse_i32_tuple(&self.input_lattice, "QPXF input sideband")?,
                None,
            ),
            QpxfSidebandSelection::AllRetained => {
                (Vec::new(), Some(QpxfInputLattices::AllRetained))
            }
            QpxfSidebandSelection::MaxOrders => {
                let orders = self
                    .max_orders
                    .split(|c: char| c == ',' || c.is_whitespace())
                    .filter(|v| !v.is_empty())
                    .map(|v| {
                        v.parse::<usize>().map_err(|_| {
                            "QPXF maximum orders must be nonnegative integers".to_owned()
                        })
                    })
                    .collect::<Result<_, _>>()?;
                (Vec::new(), Some(QpxfInputLattices::MaxOrders(orders)))
            }
            QpxfSidebandSelection::Explicit => {
                let mut tuples = self
                    .input_lattices
                    .split([';', '\n'])
                    .map(str::trim)
                    .filter(|v| !v.is_empty())
                    .map(|v| parse_i32_tuple(v, "QPXF input sideband"))
                    .collect::<Result<Vec<_>, _>>()?;
                if tuples.len() == 1 {
                    (tuples.remove(0), None)
                } else {
                    (Vec::new(), Some(QpxfInputLattices::Explicit(tuples)))
                }
            }
        };
        let defaults = QuasiPeriodicLinearConfig::default();
        let iterative = self.linear_method != QuasiPeriodicLinearMethod::Direct;
        let controls = QpxfControls {
            frequency_axis: self.frequency_axis,
            explicit_frequencies,
            input_sources,
            input_lattices,
            branch_current: self
                .current_output
                .then(|| self.output_branch.trim().into()),
            solver: QuasiPeriodicLinearConfig {
                method: self.linear_method,
                restart: if iterative {
                    parse_positive_usize(&self.krylov_restart, "QPXF Krylov restart")?
                } else {
                    defaults.restart
                },
                max_cycles: if iterative {
                    parse_positive_usize(&self.krylov_cycles, "QPXF Krylov cycles")?
                } else {
                    defaults.max_cycles
                },
                relative_tolerance: value(
                    &self.linear_tolerance,
                    "QPXF relative adjoint tolerance",
                )?,
            },
            group_delay_magnitude_floor: if self.group_delay {
                value(
                    &self.group_delay_magnitude_floor,
                    "QPXF group-delay magnitude floor",
                )?
            } else {
                0.0
            },
        };
        let spec = AnalysisSpec::Qpxf {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            input_source,
            output_node: if self.current_output {
                String::new()
            } else {
                self.output_node.trim().into()
            },
            output_ref: if self.current_output {
                String::new()
            } else {
                self.output_ref.trim().into()
            },
            input_lattice,
            output_lattice: parse_i32_tuple(&self.output_lattice, "QPXF output sideband")?,
            group_delay: self.group_delay,
            controls,
        };
        spec.validate()?;
        Ok(spec)
    }
    pub(crate) fn summary(&self) -> String {
        let source = match self.source_selection {
            QpxfSourceSelection::Single => self.input_source.as_str(),
            QpxfSourceSelection::Named => "Selected sources",
            QpxfSourceSelection::AllIndependent => "All sources",
        };
        let output = if self.current_output {
            format!("I({})", self.output_branch)
        } else {
            format!("V({},{})", self.output_node, self.output_ref)
        };
        let axis = match self.frequency_axis {
            QpxfFrequencyAxis::Output => "output Hz",
            QpxfFrequencyAxis::Offset => "offset Hz",
        };
        let sweep = if self.explicit_frequencies.trim().is_empty() {
            format!("{}…{}", self.sweep.start, self.sweep.stop)
        } else {
            self.explicit_frequencies.clone()
        };
        format!(
            "{source} → {output} · {sweep} {axis} · [{}]",
            self.output_lattice
        )
    }
}
