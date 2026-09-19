//! Authored DC axis modes shared by forms, workers and study bases.
use rspice_core::netlist::{DcSweepMode, DcSweepSpec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DcAxisMode {
    #[default]
    Linear,
    List {
        values: Vec<f64>,
    },
    Decade {
        points_per_decade: usize,
    },
    Octave {
        points_per_octave: usize,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DcSweepModes {
    #[serde(default)]
    pub primary: DcAxisMode,
    #[serde(default)]
    pub secondary: DcAxisMode,
}

impl From<&DcSweepMode> for DcAxisMode {
    fn from(mode: &DcSweepMode) -> Self {
        match mode {
            DcSweepMode::Linear => Self::Linear,
            DcSweepMode::List(values) => Self::List {
                values: values.clone(),
            },
            DcSweepMode::Decade { points_per_decade } => Self::Decade {
                points_per_decade: *points_per_decade,
            },
            DcSweepMode::Octave { points_per_octave } => Self::Octave {
                points_per_octave: *points_per_octave,
            },
        }
    }
}

impl DcAxisMode {
    pub fn spec(&self, start: f64, stop: f64, step: f64) -> DcSweepSpec {
        match self {
            Self::Linear => DcSweepSpec::linear(start, stop, step),
            Self::List { values } => DcSweepSpec::list(values.clone()),
            Self::Decade { points_per_decade } => {
                DcSweepSpec::decade(start, stop, *points_per_decade)
            }
            Self::Octave { points_per_octave } => {
                DcSweepSpec::octave(start, stop, *points_per_octave)
            }
        }
    }

    pub fn validate(&self, start: f64, stop: f64, step: f64) -> Result<(), String> {
        match self {
            Self::Linear => {
                if [start, stop, step].iter().any(|x| !x.is_finite()) {
                    return Err("Sweep start, stop, and step must be finite".into());
                }
                if step == 0.0 {
                    return Err("Step size cannot be zero".into());
                }
                if start != stop && (stop - start).signum() != step.signum() {
                    return Err("Step direction must match sweep direction".into());
                }
            }
            Self::List { values } => {
                if values.is_empty() || values.iter().any(|x| !x.is_finite()) {
                    return Err(
                        "A DC list needs at least one finite value; order and repeats are retained"
                            .into(),
                    );
                }
            }
            Self::Decade {
                points_per_decade: count,
            }
            | Self::Octave {
                points_per_octave: count,
            } => {
                if start > stop {
                    return Err("DEC/OCT require ascending endpoints; use LIST for descending logarithmic coordinates".into());
                }
                if *count == 0
                    || !start.is_finite()
                    || !stop.is_finite()
                    || start <= 0.0
                    || stop <= 0.0
                {
                    return Err("Logarithmic DC sweeps need positive finite endpoints and at least one point per interval".into());
                }
            }
        }
        let points = self
            .spec(start, stop, step)
            .points_bounded_with_abort(2_000_000, &rspice_core::abort_signal::NoAbort)
            .map_err(|error| error.to_string())?;
        if points.is_empty() {
            return Err("DC sweep generated no representable points".into());
        }
        Ok(())
    }

    pub fn card_axis(&self, source: &str, start: f64, stop: f64, step: f64) -> String {
        match self {
            Self::Linear => format!("{source} {start} {stop} {step}"),
            Self::List { values } => format!(
                "{source} LIST {}",
                values
                    .iter()
                    .map(f64::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::Decade { points_per_decade } => {
                format!("{source} DEC {points_per_decade} {start} {stop}")
            }
            Self::Octave { points_per_octave } => {
                format!("{source} OCT {points_per_octave} {start} {stop}")
            }
        }
    }
}
