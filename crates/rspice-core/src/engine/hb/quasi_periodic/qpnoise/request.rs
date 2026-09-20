//! Authored noise windows, measurement channels, source selection and integration.
use super::*;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QpnoiseFrequencyAxis {
    /// Frequencies are physical frequencies at the first output's tone tuple.
    Output,
    /// Frequencies are translated offsets at the zero tuple.
    Offset,
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum QpnoiseLattices {
    AllRetained,
    Explicit {
        tuples: Vec<Vec<i32>>,
    },
    MaxOrders {
        orders: Vec<usize>,
    },
    /// Asymmetric per-tone bounds intersected with the retained circuit lattice.
    Range {
        minimum: Vec<i32>,
        maximum: Vec<i32>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "kind",
    content = "names",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum QpnoiseSources {
    All,
    Only(Vec<String>),
    Except(Vec<String>),
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseOutput {
    pub observation: QpnoiseObservation,
    pub lattice: Vec<i32>,
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseInput {
    pub source: String,
    pub lattice: Vec<i32>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QpnoiseIntegrationMethod {
    Linear,
    LogLog,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseIntegration {
    /// Physical output-frequency band; None uses each output's complete sweep.
    pub band_hz: Option<[Value; 2]>,
    pub method: QpnoiseIntegrationMethod,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseNoiseFigure {
    /// Fixed thermal resistor in series with the input's ideal voltage source.
    pub source_resistor: String,
    pub reference_temperature: Value,
    /// None gives SSB using the input signal tuple. An explicit list sums the
    /// independent reference-channel powers (e.g. signal and image for DSB).
    pub reference_lattices: Option<Vec<Vec<i32>>>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpnoiseRequest {
    pub frequencies_hz: Vec<Value>,
    pub frequency_axis: QpnoiseFrequencyAxis,
    pub outputs: Vec<QpnoiseOutput>,
    pub input: Option<QpnoiseInput>,
    pub input_lattices: QpnoiseLattices,
    pub sources: QpnoiseSources,
    pub integration: Option<QpnoiseIntegration>,
    pub contributor_ranking: bool,
    pub noise_figure: Option<QpnoiseNoiseFigure>,
    pub linear: QuasiPeriodicLinearConfig,
}
fn name(value: &str) -> bool {
    !value.trim().is_empty() && !value.contains(['\r', '\n'])
}
fn tuples_valid(tuples: &[Vec<i32>], dimensions: usize) -> bool {
    let mut seen = BTreeSet::new();
    !tuples.is_empty()
        && tuples
            .iter()
            .all(|t| t.len() == dimensions && seen.insert(t))
}
impl QpnoiseRequest {
    pub fn validate(&self) -> Result<(), SimulationError> {
        self.linear.validate().map_err(numerical_error)?;
        if self.frequencies_hz.is_empty()
            || self.frequencies_hz.iter().any(|f| !f.is_finite())
            || self.frequencies_hz.windows(2).any(|p| p[0] >= p[1])
        {
            return Err(qpnoise_error(
                "frequencies must be finite, nonempty and strictly increasing",
            ));
        }
        let dimensions = self.outputs.first().map_or(0, |o| o.lattice.len());
        if dimensions < 2 {
            return Err(qpnoise_error(
                "at least one output with two or more tone coordinates is required",
            ));
        }
        let mut outputs = BTreeSet::new();
        for output in &self.outputs {
            let identity = match &output.observation {
                QpnoiseObservation::Voltage { positive, negative } => {
                    if !name(positive)
                        || !name(negative)
                        || positive.trim().eq_ignore_ascii_case(negative.trim())
                    {
                        return Err(qpnoise_error(
                            "output voltage requires distinct named nodes",
                        ));
                    }
                    format!(
                        "v:{}:{}",
                        positive.trim().to_ascii_lowercase(),
                        negative.trim().to_ascii_lowercase()
                    )
                }
                QpnoiseObservation::BranchCurrent { branch } => {
                    if !name(branch) {
                        return Err(qpnoise_error(
                            "output branch must have a nonempty single-line name",
                        ));
                    }
                    format!("i:{}", branch.trim().to_ascii_lowercase())
                }
            };
            if output.lattice.len() != dimensions || !outputs.insert((identity, &output.lattice)) {
                return Err(qpnoise_error(
                    "outputs must be unique and use the same tone dimensions",
                ));
            }
        }
        if let Some(input) = &self.input
            && (!name(&input.source) || input.lattice.len() != dimensions)
        {
            return Err(qpnoise_error(
                "input referral requires a named independent source and matching tone tuple",
            ));
        }
        self.input_lattices.validate(dimensions)?;
        if let QpnoiseSources::Only(names) | QpnoiseSources::Except(names) = &self.sources {
            let mut seen = BTreeSet::new();
            if names.is_empty()
                || names
                    .iter()
                    .any(|s| !name(s) || !seen.insert(s.trim().to_ascii_lowercase()))
            {
                return Err(qpnoise_error(
                    "noise mechanism selection requires unique nonempty names",
                ));
            }
        }
        if let Some(integration) = &self.integration
            && let Some([low, high]) = integration.band_hz
            && (!low.is_finite() || !high.is_finite() || low < 0.0 || high <= low)
        {
            return Err(qpnoise_error(
                "integration band must be finite, nonnegative and increasing",
            ));
        }
        if let Some(figure) = &self.noise_figure {
            if self.input.is_none()
                || !name(&figure.source_resistor)
                || !figure.reference_temperature.is_finite()
                || figure.reference_temperature <= 0.0
            {
                return Err(qpnoise_error(
                    "noise figure requires input referral, a named series resistor and positive reference temperature",
                ));
            }
            if let Some(tuples) = &figure.reference_lattices
                && !tuples_valid(tuples, dimensions)
            {
                return Err(qpnoise_error(
                    "noise-figure reference tuples must be unique and match the tone dimensions",
                ));
            }
        }
        Ok(())
    }
    pub fn frequency_anchor(&self) -> Vec<i32> {
        match self.frequency_axis {
            QpnoiseFrequencyAxis::Output => self
                .outputs
                .first()
                .map_or_else(Vec::new, |o| o.lattice.clone()),
            QpnoiseFrequencyAxis::Offset => {
                vec![0; self.outputs.first().map_or(0, |o| o.lattice.len())]
            }
        }
    }
}
impl QpnoiseLattices {
    fn validate(&self, dimensions: usize) -> Result<(), SimulationError> {
        let valid = match self {
            Self::AllRetained => true,
            Self::Explicit { tuples } => tuples_valid(tuples, dimensions),
            Self::MaxOrders { orders } => {
                orders.len() == dimensions && orders.iter().all(|n| *n <= i32::MAX as usize)
            }
            Self::Range { minimum, maximum } => {
                minimum.len() == dimensions
                    && maximum.len() == dimensions
                    && minimum.iter().zip(maximum).all(|(a, b)| a <= b)
            }
        };
        if valid {
            Ok(())
        } else {
            Err(qpnoise_error(
                "noise-input window has invalid dimensions, duplicate tuples or reversed bounds",
            ))
        }
    }
    pub(super) fn resolve(
        &self,
        grid: &QuasiPeriodicGrid,
    ) -> Result<Vec<Vec<i32>>, SimulationError> {
        self.validate(grid.dimensions().len())?;
        let bounds = grid
            .config()
            .harmonics
            .iter()
            .map(|n| *n as i32)
            .collect::<Vec<_>>();
        match self {
            Self::Explicit { tuples } if tuples.iter().any(|t| grid.index_of(t).is_none()) => {
                return Err(qpnoise_error(
                    "an explicit noise-input tuple is outside retained QPSS",
                ));
            }
            Self::MaxOrders { orders }
                if orders
                    .iter()
                    .zip(&grid.config().harmonics)
                    .any(|(a, b)| a > b) =>
            {
                return Err(qpnoise_error("noise orders exceed retained QPSS"));
            }
            Self::Range { minimum, maximum }
                if minimum
                    .iter()
                    .zip(maximum)
                    .zip(&bounds)
                    .any(|((a, b), h)| *a < -*h || b > h) =>
            {
                return Err(qpnoise_error("noise range exceeds retained QPSS"));
            }
            _ => {}
        }
        let selected = match self {
            Self::AllRetained => grid.indices().to_vec(),
            Self::Explicit { tuples } => tuples.clone(),
            Self::MaxOrders { orders } => grid
                .indices()
                .iter()
                .filter(|t| {
                    t.iter()
                        .zip(orders)
                        .all(|(k, h)| k.unsigned_abs() as usize <= *h)
                })
                .cloned()
                .collect(),
            Self::Range { minimum, maximum } => grid
                .indices()
                .iter()
                .filter(|t| {
                    t.iter()
                        .zip(minimum)
                        .zip(maximum)
                        .all(|((k, a), b)| a <= k && k <= b)
                })
                .cloned()
                .collect(),
        };
        if selected.is_empty() {
            return Err(qpnoise_error(
                "noise-input window contains no retained tuples",
            ));
        }
        Ok(selected)
    }
}
impl QpnoiseSources {
    pub(super) fn select(
        &self,
        sources: Vec<QuasiPeriodicNoiseSource>,
    ) -> Result<Vec<QuasiPeriodicNoiseSource>, SimulationError> {
        let (names, include) = match self {
            Self::All => return Ok(sources),
            Self::Only(names) => (names, true),
            Self::Except(names) => (names, false),
        };
        for name in names {
            if !sources
                .iter()
                .any(|s| s.name.eq_ignore_ascii_case(name.trim()))
            {
                return Err(qpnoise_error(format!(
                    "noise mechanism '{name}' is absent from the physical source catalog"
                )));
            }
        }
        Ok(sources
            .into_iter()
            .filter(|s| names.iter().any(|n| s.name.eq_ignore_ascii_case(n.trim())) == include)
            .collect())
    }
}
