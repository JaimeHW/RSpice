//! QPXF source, observation, signed tuple and sweep-axis contracts.
use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicLinearConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QpxfFrequencyAxis {
    /// Authored frequencies are the physical output frequencies.
    Output,
    /// Physical output frequency = offset + output_tuple·tones.
    Offset,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "kind",
    content = "names",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum QpxfSources {
    AllIndependent,
    Named(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "kind",
    content = "values",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum QpxfInputLattices {
    AllRetained,
    Explicit(Vec<Vec<i32>>),
    /// Keep retained tuples within each authored per-tone absolute order.
    MaxOrders(Vec<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum QpxfOutput {
    Voltage { positive: String, negative: String },
    BranchCurrent { branch: String },
}
impl QpxfOutput {
    pub fn quantity(&self) -> QpxfQuantity {
        match self {
            Self::Voltage { .. } => QpxfQuantity::Voltage,
            Self::BranchCurrent { .. } => QpxfQuantity::Current,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpxfRequest {
    pub frequencies_hz: Vec<Value>,
    pub frequency_axis: QpxfFrequencyAxis,
    pub input_sources: QpxfSources,
    pub input_lattices: QpxfInputLattices,
    pub output: QpxfOutput,
    pub output_lattice: Vec<i32>,
    pub linear: QuasiPeriodicLinearConfig,
    pub group_delay: bool,
    pub group_delay_magnitude_floor: Value,
}

impl QpxfRequest {
    pub fn validate(&self) -> Result<(), SimulationError> {
        self.linear.validate().map_err(numerical_error)?;
        if !self.group_delay_magnitude_floor.is_finite() || self.group_delay_magnitude_floor < 0.0 {
            return Err(qpxf_error(
                "group-delay magnitude floor must be nonnegative and finite",
            ));
        }
        if self.frequencies_hz.is_empty()
            || self.frequencies_hz.iter().any(|f| !f.is_finite())
            || self.frequencies_hz.windows(2).any(|f| f[0] >= f[1])
        {
            return Err(qpxf_error(
                "frequencies must be finite, nonempty and strictly increasing",
            ));
        }
        let dimensions = self.output_lattice.len();
        if dimensions < 2 {
            return Err(qpxf_error("output tuple needs at least two coordinates"));
        }
        let valid_name = |name: &str| !name.trim().is_empty() && !name.contains(['\r', '\n']);
        let valid_output = match &self.output {
            QpxfOutput::Voltage { positive, negative } => {
                valid_name(positive)
                    && valid_name(negative)
                    && !positive.trim().eq_ignore_ascii_case(negative.trim())
            }
            QpxfOutput::BranchCurrent { branch } => valid_name(branch),
        };
        if !valid_output {
            return Err(qpxf_error(
                "a named branch current or distinct named output/reference nodes are required",
            ));
        }
        if let QpxfSources::Named(names) = &self.input_sources {
            let mut seen = BTreeSet::new();
            if names.is_empty()
                || names
                    .iter()
                    .any(|name| !valid_name(name) || !seen.insert(name.trim().to_ascii_lowercase()))
            {
                return Err(qpxf_error(
                    "input source selection must contain unique nonempty single-line names",
                ));
            }
        }
        match &self.input_lattices {
            QpxfInputLattices::AllRetained => {}
            QpxfInputLattices::Explicit(tuples) => {
                let mut seen = BTreeSet::new();
                if tuples.is_empty()
                    || tuples
                        .iter()
                        .any(|tuple| tuple.len() != dimensions || !seen.insert(tuple))
                {
                    return Err(qpxf_error(
                        "input tuples must be unique, nonempty and match the output tuple dimensions",
                    ));
                }
            }
            QpxfInputLattices::MaxOrders(orders) => {
                if orders.len() != dimensions || orders.iter().any(|n| *n > i32::MAX as usize) {
                    return Err(qpxf_error(
                        "maximum input orders need one nonnegative signed-index bound per tone",
                    ));
                }
            }
        }
        Ok(())
    }
}

impl QpxfInputLattices {
    pub(super) fn resolve(
        &self,
        grid: &QuasiPeriodicGrid,
    ) -> Result<Vec<Vec<i32>>, SimulationError> {
        match self {
            Self::AllRetained => Ok(grid.indices().to_vec()),
            Self::Explicit(tuples) => {
                if tuples.iter().any(|tuple| grid.index_of(tuple).is_none()) {
                    return Err(qpxf_error(
                        "an input tuple is absent from the retained QPSS lattice",
                    ));
                }
                Ok(tuples.clone())
            }
            Self::MaxOrders(orders) => {
                if orders.len() != grid.dimensions().len()
                    || orders
                        .iter()
                        .zip(&grid.config().harmonics)
                        .any(|(order, retained)| order > retained)
                {
                    return Err(qpxf_error(
                        "input order bounds exceed the retained QPSS lattice",
                    ));
                }
                Ok(grid
                    .indices()
                    .iter()
                    .filter(|tuple| {
                        tuple
                            .iter()
                            .zip(orders)
                            .all(|(k, n)| k.unsigned_abs() as usize <= *n)
                    })
                    .cloned()
                    .collect())
            }
        }
    }
}
