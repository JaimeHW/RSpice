//! Exact DC curve identities and traversal, independent of plotted axis order.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DcSweepDirection {
    Ascending,
    Descending,
}

impl DcSweepDirection {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ascending => "Ascending",
            Self::Descending => "Descending",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "kind",
    content = "name",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum DcSweepQuantity {
    NodeVoltage(String),
    BranchCurrent(String),
}

impl DcSweepQuantity {
    pub fn name(&self) -> &str {
        match self {
            Self::NodeVoltage(name) | Self::BranchCurrent(name) => name,
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            Self::NodeVoltage(_) => "V",
            Self::BranchCurrent(_) => "A",
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::NodeVoltage(name) => format!("V({name})"),
            Self::BranchCurrent(name) => format!("I({name})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DcSweepFamily {
    Single,
    Nested {
        source: String,
        /// Actual solved secondary coordinates, in traversal order.
        values: Vec<f64>,
    },
    Retraced,
}

/// One quantity basis and one optional secondary axis describe every curve.
/// Samples remain in the ordinary waveform owner, with ascending display axes.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DcSweepEvidence {
    pub source: String,
    pub direction: DcSweepDirection,
    pub quantities: Vec<DcSweepQuantity>,
    pub family: DcSweepFamily,
    /// Saved-output projection can keep a sparse subset of the solved grid.
    pub selection: DcCurveSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DcCurveIndex {
    pub quantity: usize,
    pub member: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "kind",
    content = "curves",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum DcCurveSelection {
    All,
    Saved(Vec<DcCurveIndex>),
}

/// Borrowed shape/quantity information at native, project, and worker boundaries.
pub(crate) struct DcTraceView<'a> {
    pub name: &'a str,
    pub unit: Option<&'a str>,
    pub x: &'a [f64],
    pub sample_count: usize,
    pub complex: bool,
}

impl DcSweepEvidence {
    pub(crate) fn curve_indices(&self) -> impl Iterator<Item = DcCurveIndex> + '_ {
        let members = if matches!(self.selection, DcCurveSelection::All) {
            self.member_count()
        } else {
            0
        };
        let saved = match &self.selection {
            DcCurveSelection::Saved(curves) => curves.as_slice(),
            DcCurveSelection::All => &[],
        };
        (0..members)
            .flat_map(move |member| {
                (0..self.quantities.len()).map(move |quantity| DcCurveIndex { quantity, member })
            })
            .chain(saved.iter().copied())
    }

    pub(crate) fn retain_curves(&mut self, names: &HashSet<String>) {
        let retained = self
            .curve_indices()
            .filter(|curve| {
                names.contains(&self.trace_name(&self.quantities[curve.quantity], curve.member))
            })
            .collect::<Vec<_>>();
        if self.quantities.len().checked_mul(self.member_count()) != Some(retained.len()) {
            self.selection = DcCurveSelection::Saved(retained);
        }
    }
    pub fn member_count(&self) -> usize {
        match &self.family {
            DcSweepFamily::Single => 1,
            DcSweepFamily::Nested { values, .. } => values.len(),
            DcSweepFamily::Retraced => 2,
        }
    }

    /// The ordinal keeps every nested member distinct even when a displayed
    /// coordinate is rounded by a consumer. Node and branch namespaces differ.
    pub fn trace_name(&self, quantity: &DcSweepQuantity, member: usize) -> String {
        let label = quantity.label();
        match &self.family {
            DcSweepFamily::Single => label,
            DcSweepFamily::Nested { source, values } => {
                format!(
                    "{label} [{source}={}; point {}]",
                    values[member],
                    member + 1
                )
            }
            DcSweepFamily::Retraced => {
                let branch = if member == 0 { "forward" } else { "reverse" };
                format!("{label} [{branch}]")
            }
        }
    }

    pub fn terminal_sample(&self, member: usize, sample_count: usize) -> Option<usize> {
        let descending = self.direction == DcSweepDirection::Descending;
        let reverse = matches!(self.family, DcSweepFamily::Retraced) && member == 1;
        if sample_count == 0 || member >= self.member_count() {
            None
        } else if descending != reverse {
            Some(0)
        } else {
            Some(sample_count - 1)
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.source.trim().is_empty() || self.source.trim() != self.source {
            return Err("DC sweep evidence has an invalid primary source".to_owned());
        }
        if self.quantities.is_empty() {
            return Err("DC sweep evidence has no solved quantities".to_owned());
        }
        let mut identities = HashSet::with_capacity(self.quantities.len());
        for quantity in &self.quantities {
            let name = quantity.name();
            if name.trim().is_empty()
                || name.trim() != name
                || !identities.insert((quantity.unit(), name.to_ascii_lowercase()))
                || (matches!(quantity, DcSweepQuantity::NodeVoltage(_))
                    && (name == "0" || name.eq_ignore_ascii_case("gnd")))
            {
                return Err(
                    "DC sweep evidence has an invalid or duplicate solved quantity".to_owned(),
                );
            }
        }
        if let DcSweepFamily::Nested { source, values } = &self.family {
            if source.trim().is_empty() || source.trim() != source || values.is_empty() {
                return Err(
                    "Nested DC evidence is missing its secondary source or points".to_owned(),
                );
            }
            if source.eq_ignore_ascii_case(&self.source) {
                return Err(
                    "Nested DC evidence must name distinct primary and secondary sources"
                        .to_owned(),
                );
            }
            if values.iter().any(|value| !value.is_finite()) {
                return Err("Nested DC evidence contains a non-finite coordinate".to_owned());
            }
            if values.len() > 1 {
                let descending = values[1] < values[0];
                if values.windows(2).any(|pair| {
                    if descending {
                        pair[1] >= pair[0]
                    } else {
                        pair[1] <= pair[0]
                    }
                }) {
                    return Err(
                        "Nested DC coordinates must follow one strict traversal order".to_owned(),
                    );
                }
            }
        }
        if let DcCurveSelection::Saved(curves) = &self.selection {
            let mut seen = HashSet::with_capacity(curves.len());
            if curves.iter().any(|curve| {
                curve.quantity >= self.quantities.len()
                    || curve.member >= self.member_count()
                    || !seen.insert(*curve)
            }) {
                return Err(
                    "DC saved-curve selection has an invalid or duplicate identity".to_owned(),
                );
            }
        }
        Ok(())
    }

    pub(crate) fn validate_traces<'a>(
        &self,
        primary_axis: &'a [f64],
        traces: impl IntoIterator<Item = DcTraceView<'a>>,
    ) -> Result<(), String> {
        self.validate_trace_coverage(traces, Some(primary_axis))
    }

    pub(crate) fn validate_retained_traces<'a>(
        &self,
        traces: impl IntoIterator<Item = DcTraceView<'a>>,
    ) -> Result<(), String> {
        self.validate_trace_coverage(traces, None)
    }

    fn validate_trace_coverage<'a>(
        &self,
        traces: impl IntoIterator<Item = DcTraceView<'a>>,
        primary_axis: Option<&'a [f64]>,
    ) -> Result<(), String> {
        self.validate()?;
        let mut by_name = HashMap::new();
        for trace in traces {
            if by_name.insert(trace.name, trace).is_some() {
                return Err("DC sweep contains duplicate curve names".to_owned());
            }
        }
        let count = match &self.selection {
            DcCurveSelection::All => self.quantities.len().checked_mul(self.member_count()),
            DcCurveSelection::Saved(curves) => Some(curves.len()),
        };
        if count.is_none_or(|count| count > by_name.len())
            || (primary_axis.is_some()
                && (count != Some(by_name.len())
                    || !matches!(self.selection, DcCurveSelection::All)))
        {
            return Err(
                "DC sweep curves do not cover the declared quantity/coordinate space".to_owned(),
            );
        }
        if let Some(axis) = primary_axis {
            validate_axis(axis)?;
        }
        let mut reference_axis = primary_axis;
        for curve in self.curve_indices() {
            let member = curve.member;
            let quantity = &self.quantities[curve.quantity];
            let name = self.trace_name(quantity, member);
            let trace = by_name
                .get(name.as_str())
                .ok_or_else(|| format!("DC sweep is missing the solved curve '{name}'"))?;
            if trace.unit != Some(quantity.unit()) || trace.complex {
                return Err(format!(
                    "DC sweep curve '{name}' has an inconsistent quantity"
                ));
            }
            if trace.x.is_empty() || trace.x.len() != trace.sample_count {
                return Err(format!(
                    "DC sweep curve '{name}' has an invalid sample shape"
                ));
            }
            if let Some(reference) = reference_axis {
                if !std::ptr::eq(reference, trace.x) && reference != trace.x {
                    return Err("DC sweep curves have different primary axes".to_owned());
                }
            } else {
                validate_axis(trace.x)?;
                reference_axis = Some(trace.x);
            }
        }
        Ok(())
    }
}

fn validate_axis(axis: &[f64]) -> Result<(), String> {
    if axis.is_empty()
        || axis.iter().any(|value| !value.is_finite())
        || axis.windows(2).any(|pair| pair[1] <= pair[0])
    {
        Err("DC sweep display axis must be finite and strictly ascending".to_owned())
    } else {
        Ok(())
    }
}
