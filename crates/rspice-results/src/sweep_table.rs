//! A sweep, corner run or temperature step read as one table: a row per
//! point, a column for each quantity the points were swept over and one for
//! each value they measured, so any measured value plots against any swept
//! quantity.
//!
//! This is what ADE's Plot Across Design Points and Plot Across Corners
//! draw, and what ADS indexes out of a swept dataset (`IC.i[::,5]`, a value
//! against the outer sweep variable): the swept quantity on X, the measured
//! value on Y, and every further swept quantity a family of curves.
//!
//! The table comes from either shape a family is retained in:
//!
//! - an expanded family: a `.STEP` point, a corner or a temperature solved as
//!   its own analysis. Each member names its point (its `.STEP` coordinate, or
//!   the PVT point it was solved at) and keeps its own `.MEAS` results and
//!   waveforms, so a row's measured values are its `.MEAS` results, and a
//!   signal's value is read from the member's own samples at an X the reader
//!   names;
//! - an in-analysis family (`.STEP` over an operating point, a corner sweep):
//!   one analysis whose waveforms hold one sample per point, so a row is one
//!   sample of each waveform.
//!
//! Only a quantity whose value differs between points is a swept axis: a
//! corner run at one temperature does not sweep the temperature.

use crate::analysis_result::AnalysisResult;
use crate::family_metadata::AnalysisResultFamilyMetadata;
use crate::number_format::{spice_coordinate, swept_device_unit};
use crate::provenance::{AnalysisResultFamilyMember, AnalysisResultPvtPoint};
use crate::sampling::SampleInterpolation;
use crate::waveform::RetainedWaveform;

/// One point's value of a swept quantity.
#[derive(Debug, Clone, PartialEq)]
pub enum SweptValue {
    Number(f64),
    /// A quantity that is a name, not a number: a process corner.
    Text(String),
}

/// One quantity the points were swept over, with its value at every row.
#[derive(Debug, Clone, PartialEq)]
pub struct SweptAxis {
    /// The quantity as the deck names it: `r`, `R1`, `temp`, `corner`.
    pub name: String,
    /// Its unit where the swept thing names one; empty otherwise.
    pub unit: String,
    pub values: Vec<SweptValue>,
}

impl SweptAxis {
    /// Whether every row's value is a number, so the quantity can be an X
    /// axis.
    #[must_use]
    pub fn is_numeric(&self) -> bool {
        self.values
            .iter()
            .all(|value| matches!(value, SweptValue::Number(_)))
    }

    /// Row `row`'s value as a number, when it is one.
    #[must_use]
    pub fn number(&self, row: usize) -> Option<f64> {
        match self.values.get(row)? {
            SweptValue::Number(value) => Some(*value),
            SweptValue::Text(_) => None,
        }
    }

    /// Row `row`'s value as the netlist states it: `2k`, `125°C`, `FF`.
    #[must_use]
    pub fn spelled(&self, row: usize) -> String {
        match self.values.get(row) {
            Some(SweptValue::Number(value)) => spice_coordinate(*value, &self.unit),
            Some(SweptValue::Text(text)) => text.clone(),
            None => String::new(),
        }
    }

    /// Row `row` named by this quantity: `r=2k`, or a corner's own name.
    #[must_use]
    pub fn assignment(&self, row: usize) -> String {
        match self.values.get(row) {
            Some(SweptValue::Text(text)) => text.clone(),
            _ => format!("{}={}", self.name, self.spelled(row)),
        }
    }

    /// The distinct values, in the order they first appear.
    #[must_use]
    pub fn levels(&self) -> Vec<SweptValue> {
        let mut levels: Vec<SweptValue> = Vec::new();
        for value in &self.values {
            if !levels.iter().any(|level| same_value(level, value)) {
                levels.push(value.clone());
            }
        }
        levels
    }

    /// Which of [`Self::levels`] row `row` takes.
    #[must_use]
    pub fn level_of(&self, row: usize) -> Option<usize> {
        let value = self.values.get(row)?;
        self.levels()
            .iter()
            .position(|level| same_value(level, value))
    }
}

fn same_value(left: &SweptValue, right: &SweptValue) -> bool {
    match (left, right) {
        (SweptValue::Number(a), SweptValue::Number(b)) => a.to_bits() == b.to_bits(),
        (SweptValue::Text(a), SweptValue::Text(b)) => a == b,
        _ => false,
    }
}

/// What one measured column holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputKind {
    /// A `.MEAS` result each point evaluated.
    Measurement,
    /// One sample of a waveform per point: an in-analysis family's value.
    Sample,
    /// A signal each point retained whole, read at an X the reader names.
    Signal,
}

/// One measured value of every row.
#[derive(Debug, Clone, PartialEq)]
pub struct SweepOutput {
    pub name: String,
    pub unit: String,
    pub kind: OutputKind,
    /// The row's finite value, or `None` where the point has none. Empty for
    /// a [`OutputKind::Signal`], which is read on request.
    pub values: Vec<Option<f64>>,
}

/// A family as rows of swept quantities and measured values.
#[derive(Debug, Clone, PartialEq)]
pub struct SweepTable {
    /// Each row's name, as a legend names the member: `r=2k`, `FF · 125°C`.
    pub rows: Vec<String>,
    /// The analysis each row's values came from: the member itself for an
    /// expanded family, the family's one analysis for an in-analysis one.
    pub analyses: Vec<usize>,
    /// The quantities that differ between rows, in the order they were
    /// declared.
    pub axes: Vec<SweptAxis>,
    pub outputs: Vec<SweepOutput>,
    /// The X axis an expanded member's signals are drawn against: `time`
    /// with its unit `s`. `None` for an in-analysis family.
    pub signal_axis: Option<(String, String)>,
}

impl SweepTable {
    /// The output named `name`, matched as SPICE matches names: in any case.
    #[must_use]
    pub fn output(&self, name: &str) -> Option<&SweepOutput> {
        self.outputs
            .iter()
            .find(|output| output.name.eq_ignore_ascii_case(name))
    }

    /// The axis named `name`, in any case.
    #[must_use]
    pub fn axis(&self, name: &str) -> Option<&SweptAxis> {
        self.axes
            .iter()
            .find(|axis| axis.name.eq_ignore_ascii_case(name))
    }

    /// The quantities that can stand on X: those whose every value is a
    /// number.
    pub fn numeric_axes(&self) -> impl Iterator<Item = &SweptAxis> {
        self.axes.iter().filter(|axis| axis.is_numeric())
    }

    /// Row `row` named by every axis but `x`: what tells the curves of a
    /// family drawn against `x` apart, `c=10p`, or empty when nothing else
    /// was swept.
    #[must_use]
    pub fn curve_label(&self, row: usize, x: &str) -> String {
        self.axes
            .iter()
            .filter(|axis| axis.name != x)
            .map(|axis| axis.assignment(row))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The rows grouped into the curves a plot against `x` draws: one curve
    /// per combination of the other axes' values, each curve's rows in
    /// ascending `x`. Rows without a numeric `x` are left out.
    #[must_use]
    pub fn curves(&self, x: &str) -> Vec<SweepCurve> {
        let Some(axis) = self.axis(x) else {
            return Vec::new();
        };
        let mut curves: Vec<SweepCurve> = Vec::new();
        for row in 0..self.rows.len() {
            if axis.number(row).is_none() {
                continue;
            }
            let label = self.curve_label(row, x);
            match curves.iter_mut().find(|curve| curve.label == label) {
                Some(curve) => curve.rows.push(row),
                None => curves.push(SweepCurve {
                    levels: self
                        .axes
                        .iter()
                        .filter(|other| other.name != x)
                        .map(|other| other.level_of(row).unwrap_or(0))
                        .collect(),
                    label,
                    rows: vec![row],
                }),
            }
        }
        for curve in &mut curves {
            curve.rows.sort_by(|a, b| {
                axis.number(*a)
                    .unwrap_or(f64::NAN)
                    .total_cmp(&axis.number(*b).unwrap_or(f64::NAN))
            });
        }
        curves
    }
}

/// One curve of a value plotted against a swept quantity.
#[derive(Debug, Clone, PartialEq)]
pub struct SweepCurve {
    /// What the other swept quantities hold on this curve, `c=10p`; empty
    /// when the family swept one quantity.
    pub label: String,
    /// Which level of each other axis, in axis order, the curve holds: the
    /// first picks its colour, the second its dash.
    pub levels: Vec<usize>,
    /// The rows it joins, in ascending X.
    pub rows: Vec<usize>,
}

/// The family `analyses[index]` belongs to, as a table; `None` when it
/// belongs to none or its points cannot be told apart.
#[must_use]
pub fn sweep_table<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    analyses: &[A],
    index: usize,
) -> Option<SweepTable> {
    let analysis = analyses.get(index)?.as_ref();
    if let Some(table) = in_analysis_table(analysis, index) {
        return Some(table);
    }
    expanded_table(analyses, index)
}

/// Whether `analyses[index]` has a sweep table, without building one.
#[must_use]
pub fn has_sweep_table<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    analyses: &[A],
    index: usize,
) -> bool {
    let Some(analysis) = analyses.get(index).map(AsRef::as_ref) else {
        return false;
    };
    if matches!(
        analysis.family_metadata,
        Some(
            AnalysisResultFamilyMetadata::Parametric { .. }
                | AnalysisResultFamilyMetadata::Corner { .. }
        )
    ) {
        return in_analysis_table(analysis, index).is_some();
    }
    expanded_members(analyses, index).len() >= 2
}

/// Whether `analysis` is a point of a sweep, alone: a sweep held in one
/// analysis, a `.STEP` point, or a point solved at a PVT corner. Whether a
/// point's siblings make a sweep of it is [`has_sweep_table`]'s question.
#[must_use]
pub fn is_sweep_point<W: AsRef<RetainedWaveform>>(analysis: &AnalysisResult<W>) -> bool {
    if !analysis.success {
        return false;
    }
    match &analysis.family_metadata {
        Some(
            AnalysisResultFamilyMetadata::Parametric { .. }
            | AnalysisResultFamilyMetadata::Corner { .. },
        ) => !analysis.waveforms.is_empty(),
        _ => match analysis.family_member() {
            Some(member) => matches!(member, AnalysisResultFamilyMember::StepPoint { .. }),
            None => analysis
                .provenance()
                .is_some_and(|provenance| provenance.pvt_point().is_some()),
        },
    }
}

/// The swept quantity a `.STEP` family result names in its target: `PARAM
/// rload` sweeps `rload`, whose unit the deck never states; `DEVICE R1`
/// sweeps a resistor in ohms; `TEMP` the temperature.
#[must_use]
pub fn step_target_axis(target: &str) -> (String, String) {
    let trimmed = target.trim();
    let (kind, rest) = trimmed
        .split_once(char::is_whitespace)
        .map_or((trimmed, ""), |(kind, rest)| (kind, rest.trim()));
    match kind.to_ascii_uppercase().as_str() {
        "PARAM" if !rest.is_empty() => (rest.to_owned(), String::new()),
        "DEVICE" if !rest.is_empty() => {
            let unit = if rest.contains('.') {
                ""
            } else {
                swept_device_unit(rest)
            };
            (rest.to_owned(), unit.to_owned())
        }
        "TEMP" => ("temp".to_owned(), "°C".to_owned()),
        "MODEL" | "DATA" if !rest.is_empty() => (rest.to_owned(), String::new()),
        _ => (trimmed.to_owned(), String::new()),
    }
}

fn in_analysis_table<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    index: usize,
) -> Option<SweepTable> {
    if !analysis.success {
        return None;
    }
    let metadata = analysis.family_metadata.as_ref()?;
    metadata.validate_for(analysis.analysis_type).ok()?;
    let (rows, axes) = match metadata {
        AnalysisResultFamilyMetadata::Parametric {
            target,
            sweep_values,
            ..
        } => {
            let (name, unit) = step_target_axis(target);
            let axis = SweptAxis {
                name,
                unit,
                values: sweep_values
                    .iter()
                    .map(|value| SweptValue::Number(*value))
                    .collect(),
            };
            let rows = (0..sweep_values.len())
                .map(|row| axis.assignment(row))
                .collect();
            (rows, vec![axis])
        }
        AnalysisResultFamilyMetadata::Corner {
            x_values,
            x_label,
            x_unit,
            temperatures_c,
            corner_labels,
            ..
        } => {
            let axes = varying(vec![
                SweptAxis {
                    name: "corner".to_owned(),
                    unit: String::new(),
                    values: corner_labels
                        .iter()
                        .map(|label| SweptValue::Text(label.clone()))
                        .collect(),
                },
                SweptAxis {
                    name: "temp".to_owned(),
                    unit: "°C".to_owned(),
                    values: temperatures_c
                        .iter()
                        .map(|value| SweptValue::Number(*value))
                        .collect(),
                },
                SweptAxis {
                    name: x_label.clone(),
                    unit: unit_text(x_unit),
                    values: x_values
                        .iter()
                        .map(|value| SweptValue::Number(*value))
                        .collect(),
                },
            ]);
            let rows = (0..x_values.len())
                .map(|row| point_name(&axes, row))
                .collect();
            (rows, axes)
        }
        _ => return None,
    };
    let count = axes.first().map_or(0, |axis| axis.values.len());
    let mut outputs: Vec<SweepOutput> = Vec::new();
    let members = metadata.member_measurements();
    if members.len() == count {
        for (row, member) in members.iter().enumerate() {
            for evidence in &member.measurements {
                let position = output_slot(&mut outputs, &evidence.name, count, || {
                    evidence
                        .unit
                        .as_ref()
                        .and_then(|unit| unit.symbol())
                        .map(unit_text)
                        .unwrap_or_default()
                });
                outputs[position].values[row] = evidence
                    .value
                    .filter(|value| value.is_finite() && evidence.passed);
            }
        }
    }
    outputs.extend(
        analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .filter(|waveform| waveform.y.len() == count)
            .map(|waveform| SweepOutput {
                name: waveform.name.clone(),
                unit: waveform.unit.clone().unwrap_or_default(),
                kind: OutputKind::Sample,
                values: (0..count)
                    .map(|row| waveform.y.sample(row).filter(|value| value.is_finite()))
                    .collect(),
            }),
    );
    (count >= 2 && !outputs.is_empty() && !axes.is_empty()).then(|| SweepTable {
        rows,
        analyses: vec![index; count],
        axes,
        outputs,
        signal_axis: None,
    })
}

/// The siblings of `analyses[index]` solved as points of one family, in the
/// family's order: every successful analysis of the same kind that one
/// authored analysis expanded into, or that names a member of the same
/// kind of `.STEP` family. Monte Carlo trials are a population, read by the
/// statistics sheets, not a sweep.
pub fn expanded_members<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    analyses: &[A],
    index: usize,
) -> Vec<usize> {
    let Some(analysis) = analyses.get(index).map(AsRef::as_ref) else {
        return Vec::new();
    };
    let step = analysis
        .family_member()
        .filter(|member| matches!(member, AnalysisResultFamilyMember::StepPoint { .. }));
    let authored = analysis
        .provenance()
        .filter(|provenance| provenance.pvt_point().is_some())
        .map(|provenance| provenance.authored_source_instance_id());
    if step.is_none() && authored.is_none() {
        return Vec::new();
    }
    let mut members: Vec<usize> = analyses
        .iter()
        .map(AsRef::as_ref)
        .enumerate()
        .filter(|(_, sibling)| sibling.success && sibling.analysis_type == analysis.analysis_type)
        .filter(|(_, sibling)| match step {
            Some(_) => matches!(
                sibling.family_member(),
                Some(AnalysisResultFamilyMember::StepPoint { .. })
            ),
            None => {
                sibling.family_member().is_none()
                    && sibling.provenance().is_some_and(|provenance| {
                        provenance.pvt_point().is_some()
                            && Some(provenance.authored_source_instance_id()) == authored
                    })
            }
        })
        .map(|(position, _)| position)
        .collect();
    if step.is_some() {
        members.sort_by_key(|position| {
            analyses[*position]
                .as_ref()
                .family_member()
                .map_or(usize::MAX, AnalysisResultFamilyMember::index)
        });
    }
    members
}

fn expanded_table<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    analyses: &[A],
    index: usize,
) -> Option<SweepTable> {
    let members = expanded_members(analyses, index);
    if members.len() < 2 {
        return None;
    }
    let points: Vec<&AnalysisResult<W>> = members
        .iter()
        .map(|&member| analyses[member].as_ref())
        .collect();
    let axes = point_axes(&points);
    if axes.is_empty() {
        return None;
    }
    let rows = (0..members.len())
        .map(|row| point_name(&axes, row))
        .collect();
    let mut outputs: Vec<SweepOutput> = Vec::new();
    for (row, &member) in members.iter().enumerate() {
        for measurement in &analyses[member].as_ref().measurements {
            let position = output_slot(&mut outputs, &measurement.name, members.len(), || {
                measurement
                    .units
                    .as_ref()
                    .and_then(|units| units.value.symbol())
                    .map(unit_text)
                    .unwrap_or_default()
            });
            outputs[position].values[row] = measurement
                .value
                .filter(|value| value.is_finite() && measurement.error.is_none());
        }
    }
    let first = analyses[members[0]].as_ref();
    for waveform in first.waveforms.iter().map(AsRef::as_ref) {
        if outputs.iter().any(|output| output.name == waveform.name) {
            continue;
        }
        outputs.push(SweepOutput {
            name: waveform.name.clone(),
            unit: waveform.unit.clone().unwrap_or_default(),
            kind: OutputKind::Signal,
            values: Vec::new(),
        });
    }
    let (x_label, x_unit, _, _) = first.analysis_type.axis_info();
    Some(SweepTable {
        rows,
        analyses: members,
        axes,
        outputs,
        signal_axis: Some((x_label.to_lowercase(), x_unit.to_owned())),
    })
}

/// Each row's value of signal `name` at `at` on the member's own X axis, or
/// at the end of its record when `at` is `None`: `V(out)` at 10 ms of each
/// stepped transient. A row whose member retained no such signal, or has no
/// sample there, reads `None`.
#[must_use]
pub fn signal_values<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    analyses: &[A],
    table: &SweepTable,
    name: &str,
    at: Option<f64>,
) -> Vec<Option<f64>> {
    table
        .analyses
        .iter()
        .map(|&member| {
            let waveform = analyses
                .get(member)?
                .as_ref()
                .waveforms
                .iter()
                .map(AsRef::as_ref)
                .find(|waveform| waveform.name.eq_ignore_ascii_case(name))?;
            let value = match at {
                Some(at) => waveform
                    .y
                    .sample_at(&waveform.x, at, SampleInterpolation::Linear),
                None => waveform.y.sample(waveform.y.len().checked_sub(1)?)?,
            };
            value.is_finite().then_some(value)
        })
        .collect()
}

/// The quantities that differ between the points of an expanded family, in
/// the family's order: each `.STEP` coordinate, or a PVT point's process,
/// temperature, supply and swept design values. One axis per quantity, one
/// value per point.
#[must_use]
pub fn point_axes<W: AsRef<RetainedWaveform>>(points: &[&AnalysisResult<W>]) -> Vec<SweptAxis> {
    varying(
        match points.first().and_then(|point| point.family_member()) {
            Some(AnalysisResultFamilyMember::StepPoint { .. }) => step_axes(points),
            _ => pvt_axes(points),
        },
    )
}

/// A `.STEP` family's axes, one per coordinate name in the order the first
/// member states them.
fn step_axes<W: AsRef<RetainedWaveform>>(points: &[&AnalysisResult<W>]) -> Vec<SweptAxis> {
    let mut axes: Vec<SweptAxis> = Vec::new();
    for (row, point) in points.iter().enumerate() {
        let Some(AnalysisResultFamilyMember::StepPoint { coordinate, .. }) = point.family_member()
        else {
            continue;
        };
        for value in coordinate {
            let position = match axes.iter().position(|axis| axis.name == value.name) {
                Some(position) => position,
                None => {
                    axes.push(SweptAxis {
                        name: value.name.clone(),
                        unit: value.unit.clone(),
                        values: vec![SweptValue::Number(f64::NAN); points.len()],
                    });
                    axes.len() - 1
                }
            };
            axes[position].values[row] = SweptValue::Number(value.value);
        }
    }
    axes
}

/// A PVT family's axes: the process corner, the temperature, the supply and
/// each swept design value.
fn pvt_axes<W: AsRef<RetainedWaveform>>(members: &[&AnalysisResult<W>]) -> Vec<SweptAxis> {
    let points: Vec<Option<&AnalysisResultPvtPoint>> = members
        .iter()
        .map(|member| {
            member
                .provenance()
                .and_then(|provenance| provenance.pvt_point())
        })
        .collect();
    let mut axes = vec![
        SweptAxis {
            name: "corner".to_owned(),
            unit: String::new(),
            values: points
                .iter()
                .map(|point| {
                    SweptValue::Text(point.map_or_else(String::new, |p| p.process().to_owned()))
                })
                .collect(),
        },
        SweptAxis {
            name: "temp".to_owned(),
            unit: "°C".to_owned(),
            values: points
                .iter()
                .map(|point| {
                    SweptValue::Number(
                        point.map_or(f64::NAN, AnalysisResultPvtPoint::temperature_celsius),
                    )
                })
                .collect(),
        },
    ];
    if points
        .iter()
        .all(|point| point.is_some_and(|point| point.supply_voltage().is_some()))
    {
        axes.push(SweptAxis {
            name: "supply".to_owned(),
            unit: "V".to_owned(),
            values: points
                .iter()
                .map(|point| {
                    SweptValue::Number(
                        point
                            .and_then(AnalysisResultPvtPoint::supply_voltage)
                            .unwrap_or(f64::NAN),
                    )
                })
                .collect(),
        });
    }
    let names: Vec<String> = points
        .iter()
        .flatten()
        .flat_map(|point| point.swept_values().iter().map(|value| value.name.clone()))
        .fold(Vec::new(), |mut names, name| {
            if !names.contains(&name) {
                names.push(name);
            }
            names
        });
    for name in names {
        let texts: Vec<String> = points
            .iter()
            .map(|point| {
                point
                    .and_then(|point| point.swept_values().iter().find(|value| value.name == name))
                    .map_or_else(String::new, |value| value.value.clone())
            })
            .collect();
        let numbers: Option<Vec<f64>> = texts
            .iter()
            .map(|text| rspice_vocabulary::quantity::parse_engineering_value(text).ok())
            .collect();
        axes.push(SweptAxis {
            name,
            unit: String::new(),
            values: match numbers {
                Some(numbers) => numbers.into_iter().map(SweptValue::Number).collect(),
                None => texts.into_iter().map(SweptValue::Text).collect(),
            },
        });
    }
    axes
}

/// The measurement column named `name`, matched case-insensitively as every
/// specification join matches a `.MEAS` name, added empty when no row has
/// stated it yet.
fn output_slot(
    outputs: &mut Vec<SweepOutput>,
    name: &str,
    rows: usize,
    unit: impl FnOnce() -> String,
) -> usize {
    if let Some(position) = outputs
        .iter()
        .position(|output| output.name.eq_ignore_ascii_case(name))
    {
        return position;
    }
    outputs.push(SweepOutput {
        name: name.to_owned(),
        unit: unit(),
        kind: OutputKind::Measurement,
        values: vec![None; rows],
    });
    outputs.len() - 1
}

/// Keep only the axes whose value differs between rows.
fn varying(axes: Vec<SweptAxis>) -> Vec<SweptAxis> {
    axes.into_iter()
        .filter(|axis| axis.levels().len() > 1)
        .collect()
}

/// A point named by every axis that varies, as a legend names it:
/// `r=2k, C1=10pF`, `ff, temp=125°C`.
#[must_use]
pub fn point_name(axes: &[SweptAxis], row: usize) -> String {
    axes.iter()
        .map(|axis| axis.assignment(row))
        .collect::<Vec<_>>()
        .join(", ")
}

/// A unit as a column head writes it: a temperature in degrees Celsius,
/// whatever the producer spelled it.
fn unit_text(unit: &str) -> String {
    match unit.trim() {
        "C" | "degC" | "celsius" => "°C".to_owned(),
        other => other.to_owned(),
    }
}

#[cfg(test)]
mod tests;
