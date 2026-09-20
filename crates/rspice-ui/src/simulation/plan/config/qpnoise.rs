//! Lossless noise editor buffers with arbitrary tone counts and active-only parsing.
use super::*;
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep, QpnoiseControls};
use rspice_core::analysis::quasi_periodic::{QuasiPeriodicLinearConfig, QuasiPeriodicLinearMethod};
use rspice_core::engine::{
    QpnoiseFrequencyAxis, QpnoiseIntegrationMethod, QpnoiseLattices, QpnoiseNoiseFigure,
    QpnoiseObservation, QpnoiseOutput, QpnoiseSources,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QpnoiseLatticeSelection {
    AllRetained,
    Range,
    MaxOrders,
    Explicit,
}
fn legacy_lattice_selection() -> QpnoiseLatticeSelection {
    QpnoiseLatticeSelection::Range
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QpnoiseSourceSelection {
    All,
    Only,
    Except,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QpnoiseOutputDraft {
    pub current: bool,
    pub node: String,
    pub reference: String,
    pub branch: String,
    pub lattice: String,
}
impl Default for QpnoiseOutputDraft {
    fn default() -> Self {
        Self {
            current: false,
            node: "out".into(),
            reference: "0".into(),
            branch: "Vprobe".into(),
            lattice: "0,0".into(),
        }
    }
}
impl QpnoiseOutputDraft {
    fn resolve(&self) -> Result<QpnoiseOutput, String> {
        Ok(QpnoiseOutput {
            observation: if self.current {
                QpnoiseObservation::BranchCurrent {
                    branch: self.branch.trim().into(),
                }
            } else {
                QpnoiseObservation::Voltage {
                    positive: self.node.trim().into(),
                    negative: self.reference.trim().into(),
                }
            },
            lattice: parse_i32_tuple(&self.lattice, "QPNOISE output tuple")?,
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QuasiPeriodicNoiseDraft {
    pub sweep: FrequencySweepDraft,
    pub explicit_frequencies: String,
    pub frequency_axis: QpnoiseFrequencyAxis,
    pub output_node: String,
    pub output_ref: String,
    pub current_output: bool,
    pub output_branch: String,
    pub output_lattice: String,
    pub additional_outputs: Vec<QpnoiseOutputDraft>,
    pub input_source: String,
    pub input_referral: bool,
    pub input_lattice: String,
    /// An old saved draft activates its authored min:max bounds. New drafts
    /// default to the complete retained QPSS noise window.
    #[serde(default = "legacy_lattice_selection")]
    pub lattice_selection: QpnoiseLatticeSelection,
    pub lattice_products: String,
    pub max_orders: String,
    pub explicit_lattices: String,
    pub source_selection: QpnoiseSourceSelection,
    pub source_names: String,
    pub integrated_noise: bool,
    pub integration_method: QpnoiseIntegrationMethod,
    pub band_start: String,
    pub band_stop: String,
    pub contributor_ranking: bool,
    pub noise_figure: bool,
    pub source_resistor: String,
    pub reference_temperature: String,
    pub reference_lattices: String,
    pub linear_method: QuasiPeriodicLinearMethod,
    pub krylov_restart: String,
    pub krylov_cycles: String,
    pub linear_tolerance: String,
}
impl Default for QuasiPeriodicNoiseDraft {
    fn default() -> Self {
        Self {
            sweep: Default::default(),
            explicit_frequencies: String::new(),
            frequency_axis: QpnoiseFrequencyAxis::Output,
            output_node: "out".into(),
            output_ref: "0".into(),
            current_output: false,
            output_branch: "Vprobe".into(),
            output_lattice: "0,0".into(),
            additional_outputs: Vec::new(),
            input_source: "V1".into(),
            input_referral: true,
            input_lattice: "0,0".into(),
            lattice_selection: QpnoiseLatticeSelection::AllRetained,
            lattice_products: "-3:3, -3:3".into(),
            max_orders: "1,1".into(),
            explicit_lattices: "0,0".into(),
            source_selection: QpnoiseSourceSelection::All,
            source_names: String::new(),
            integrated_noise: true,
            integration_method: QpnoiseIntegrationMethod::Linear,
            band_start: String::new(),
            band_stop: String::new(),
            contributor_ranking: true,
            noise_figure: false,
            source_resistor: String::new(),
            reference_temperature: "290".into(),
            reference_lattices: String::new(),
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
fn tuples(text: &str, field: &str) -> Result<Vec<Vec<i32>>, String> {
    text.split([';', '\n'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| parse_i32_tuple(s, field))
        .collect()
}
impl QuasiPeriodicNoiseDraft {
    pub(crate) fn to_spec(&self) -> Result<AnalysisSpec, String> {
        let (start_freq, stop_freq, points_per_unit, sweep, explicit_frequencies) =
            if self.explicit_frequencies.trim().is_empty() {
                (
                    value(&self.sweep.start, "QPNOISE start frequency")?,
                    value(&self.sweep.stop, "QPNOISE stop frequency")?,
                    parse_positive_usize(&self.sweep.points, "QPNOISE frequency points")?,
                    match self.sweep.sweep {
                        0 => FrequencySweep::Decade,
                        1 => FrequencySweep::Octave,
                        2 => FrequencySweep::Linear,
                        _ => return Err("invalid QPNOISE frequency sweep".into()),
                    },
                    None,
                )
            } else {
                (
                    0.0,
                    0.0,
                    1,
                    FrequencySweep::Linear,
                    Some(
                        self.explicit_frequencies
                            .split(|c: char| c == ',' || c.is_whitespace())
                            .filter(|s| !s.is_empty())
                            .map(|s| value(s, "QPNOISE frequency"))
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                )
            };
        let (lattice_min, lattice_max, noise_lattices) = match self.lattice_selection {
            QpnoiseLatticeSelection::AllRetained => {
                (Vec::new(), Vec::new(), Some(QpnoiseLattices::AllRetained))
            }
            QpnoiseLatticeSelection::Explicit => (
                Vec::new(),
                Vec::new(),
                Some(QpnoiseLattices::Explicit {
                    tuples: tuples(&self.explicit_lattices, "QPNOISE noise tuple")?,
                }),
            ),
            QpnoiseLatticeSelection::MaxOrders => (
                Vec::new(),
                Vec::new(),
                Some(QpnoiseLattices::MaxOrders {
                    orders: self
                        .max_orders
                        .split(|c: char| c == ',' || c.is_whitespace())
                        .filter(|s| !s.is_empty())
                        .map(|s| {
                            s.parse::<usize>().map_err(|_| {
                                "maximum noise orders require nonnegative integers".to_owned()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                }),
            ),
            QpnoiseLatticeSelection::Range => {
                let pairs = self
                    .lattice_products
                    .split(',')
                    .map(|s| {
                        let pair = s
                            .split(':')
                            .map(|v| {
                                v.trim().parse::<i32>().map_err(|_| {
                                    "noise lattice bounds require signed integers".to_owned()
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        let [low, high]: [i32; 2] = pair
                            .try_into()
                            .map_err(|_| "each noise lattice bound requires min:max".to_owned())?;
                        Ok((low, high))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                let (minimum, maximum) = pairs.into_iter().unzip();
                (minimum, maximum, None)
            }
        };
        let source_names = || {
            self.source_names
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        };
        let sources = match self.source_selection {
            QpnoiseSourceSelection::All => QpnoiseSources::All,
            QpnoiseSourceSelection::Only => QpnoiseSources::Only(source_names()),
            QpnoiseSourceSelection::Except => QpnoiseSources::Except(source_names()),
        };
        let band = if !self.integrated_noise
            || (self.band_start.trim().is_empty() && self.band_stop.trim().is_empty())
        {
            None
        } else {
            Some([
                value(&self.band_start, "integration lower frequency")?,
                value(&self.band_stop, "integration upper frequency")?,
            ])
        };
        let defaults = QuasiPeriodicLinearConfig::default();
        let iterative = self.linear_method != QuasiPeriodicLinearMethod::Direct;
        let output_lattice = parse_i32_tuple(&self.output_lattice, "QPNOISE output tuple")?;
        let controls = QpnoiseControls {
            frequency_axis: self.frequency_axis,
            explicit_frequencies,
            input_referral: self.input_referral,
            input_lattice: if self.input_referral {
                parse_i32_tuple(&self.input_lattice, "QPNOISE input tuple")?
            } else {
                vec![0; output_lattice.len()]
            },
            output_lattice,
            branch_current: self
                .current_output
                .then(|| self.output_branch.trim().into()),
            additional_outputs: self
                .additional_outputs
                .iter()
                .map(QpnoiseOutputDraft::resolve)
                .collect::<Result<_, _>>()?,
            noise_lattices,
            sources,
            integration_band: band,
            integration_method: if self.integrated_noise {
                self.integration_method
            } else {
                QpnoiseIntegrationMethod::Linear
            },
            noise_figure: if self.noise_figure {
                Some(QpnoiseNoiseFigure {
                    source_resistor: self.source_resistor.trim().into(),
                    reference_temperature: value(
                        &self.reference_temperature,
                        "noise reference temperature",
                    )?,
                    reference_lattices: if self.reference_lattices.trim().is_empty() {
                        None
                    } else {
                        Some(tuples(
                            &self.reference_lattices,
                            "noise figure reference tuple",
                        )?)
                    },
                })
            } else {
                None
            },
            solver: QuasiPeriodicLinearConfig {
                method: self.linear_method,
                restart: if iterative {
                    parse_positive_usize(&self.krylov_restart, "Krylov restart")?
                } else {
                    defaults.restart
                },
                max_cycles: if iterative {
                    parse_positive_usize(&self.krylov_cycles, "Krylov cycles")?
                } else {
                    defaults.max_cycles
                },
                relative_tolerance: value(&self.linear_tolerance, "adjoint relative tolerance")?,
            },
        };
        let spec = AnalysisSpec::Qpnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
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
            input_source: if self.input_referral {
                self.input_source.trim().into()
            } else {
                String::new()
            },
            lattice_min,
            lattice_max,
            integrated_noise: self.integrated_noise,
            contributor_ranking: self.contributor_ranking,
            controls,
        };
        spec.validate()?;
        Ok(spec)
    }
    pub(crate) fn summary(&self) -> String {
        let output = if self.current_output {
            format!("I({})", self.output_branch)
        } else {
            format!("V({},{})", self.output_node, self.output_ref)
        };
        let sweep = if self.explicit_frequencies.trim().is_empty() {
            format!("{}…{} Hz", self.sweep.start, self.sweep.stop)
        } else {
            "explicit frequencies".into()
        };
        format!(
            "{output}, {} output(s), {sweep}",
            1 + self.additional_outputs.len()
        )
    }
}

#[cfg(test)]
mod tests;
