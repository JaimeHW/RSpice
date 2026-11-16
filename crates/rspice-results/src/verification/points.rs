//! The points one run solved, named the way a verification sheet heads them.
//!
//! A point is where evidence was solved, not an analysis: an AC and a
//! transient solved at the same point are one point. Two sources name one. A
//! task the run set expanded to a point carries that point's record; a corner
//! or sweep family solved inside one analysis names each member instead.
//! Evidence from an analysis that was attributed to no point was solved at the
//! deck's own conditions, and those are one point too.

use std::collections::HashMap;

use rspice_vocabulary::product::ContentDigest;

use crate::analysis_result::AnalysisResult;
use crate::family_measurements::FamilyMemberId;
use crate::family_metadata::AnalysisResultFamilyMetadata;
use crate::provenance::{
    AnalysisResultPvtPoint, AnalysisResultSweptAxis, AnalysisResultSweptValue,
};
use crate::waveform::RetainedWaveform;

/// Where a point's identity comes from.
#[derive(Debug, Clone, PartialEq)]
pub enum PointSource {
    /// The deck's own conditions: evidence attributed to no point.
    Deck,
    /// A point the run set expanded a task to, as its results record it.
    Attributed(AnalysisResultPvtPoint),
    /// A member of a corner or sweep family solved inside one analysis.
    Member(FamilyMemberId),
}

/// One point the run solved.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    /// The process corner, when the point names one.
    pub process: Option<String>,
    /// The supply, when the run swept one.
    pub supply_voltage: Option<f64>,
    /// The temperature, when the point names one.
    pub temperature_celsius: Option<f64>,
    /// The design-variable and source values, in declaration order.
    pub swept_values: Vec<AnalysisResultSweptValue>,
    /// Whether this is the run's own reference point.
    pub nominal: bool,
    /// The point named by what varies across the run: `SS 125 °C`.
    pub label: String,
    /// Every coordinate the point has: `SS · 1.62 V · 125 °C`.
    pub detail: String,
    pub source: PointSource,
}

/// A coordinate that varies across a run's points, so a view heads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointAxis {
    Process,
    Supply,
    Temperature,
    /// A design variable or source the run set swept, or the target of a
    /// sweep family.
    Swept {
        axis: AnalysisResultSweptAxis,
        name: String,
    },
}

/// The points of one run, in run order with the nominal point first, and
/// where each analysis's evidence sits among them.
#[derive(Debug, Clone, Default)]
pub(crate) struct RunPoints {
    pub(crate) points: Vec<Point>,
    /// The point of each analysis's own evidence, by its position in the run;
    /// `None` for an analysis the run's points were not collected from.
    own: Vec<Option<usize>>,
    /// The point of each family member, by analysis position and member.
    members: HashMap<(usize, MemberKey), usize>,
    /// The analyses whose evidence sits at each point, by position.
    analyses_at: Vec<Vec<usize>>,
}

impl RunPoints {
    /// Collect the points of every analysis `admits` accepts.
    pub(crate) fn collect<'a, W: AsRef<RetainedWaveform> + 'a>(
        analyses: impl IntoIterator<Item = &'a AnalysisResult<W>>,
        admits: impl Fn(&AnalysisResult<W>) -> bool,
    ) -> Self {
        let mut index: HashMap<PointKey, usize> = HashMap::new();
        let mut points: Vec<Point> = Vec::new();
        let mut own = Vec::new();
        let mut members = HashMap::new();
        let mut place = |key: PointKey, point: &dyn Fn() -> Point| -> usize {
            *index.entry(key).or_insert_with(|| {
                points.push(point());
                points.len() - 1
            })
        };
        for (position, analysis) in analyses.into_iter().enumerate() {
            if !admits(analysis) {
                own.push(None);
                continue;
            }
            let attributed = analysis
                .provenance()
                .and_then(|provenance| provenance.pvt_point());
            let family = family_points(analysis);
            // A family whose members are its points holds evidence of its own
            // only when it measured something as a whole.
            let own_point = (family.is_empty() || !analysis.scalar_evidence_names().is_empty())
                .then(|| match attributed {
                    Some(record) => {
                        place(PointKey::attributed(record), &|| attributed_point(record))
                    }
                    None => place(PointKey::Deck, &deck_point),
                });
            own.push(own_point);
            for (member, key, point) in family {
                let at = place(key, &|| point.clone());
                members.insert((position, member), at);
            }
        }
        let mut run = Self {
            points,
            own,
            members,
            analyses_at: Vec::new(),
        };
        run.order_nominal_first();
        run.name_points();
        run.index_analyses();
        run
    }

    /// The analyses whose evidence sits at one point, in run order.
    pub(crate) fn analyses_at(&self, point: usize) -> &[usize] {
        self.analyses_at.get(point).map_or(&[], Vec::as_slice)
    }

    /// The point an analysis's own evidence sits at.
    pub(crate) fn of_analysis(&self, analysis: usize) -> Option<usize> {
        self.own.get(analysis).copied().flatten()
    }

    /// The point a candidate sits at: its family member's, when that member is
    /// a point, otherwise its analysis's own.
    pub(crate) fn of_candidate(
        &self,
        analysis: usize,
        member: Option<&FamilyMemberId>,
    ) -> Option<usize> {
        member
            .and_then(MemberKey::of)
            .and_then(|key| self.members.get(&(analysis, key)).copied())
            .or_else(|| self.of_analysis(analysis))
    }

    /// The coordinates that vary across these points, in heading order.
    pub(crate) fn varying_axes(&self) -> Vec<PointAxis> {
        varying_axes(&self.points)
    }

    fn order_nominal_first(&mut self) {
        let mut order = (0..self.points.len()).collect::<Vec<_>>();
        order.sort_by_key(|index| !self.points[*index].nominal);
        let mut moved_to = vec![0; order.len()];
        for (to, from) in order.iter().enumerate() {
            moved_to[*from] = to;
        }
        let mut points = std::mem::take(&mut self.points)
            .into_iter()
            .map(Some)
            .collect::<Vec<_>>();
        self.points = order
            .iter()
            .filter_map(|from| points[*from].take())
            .collect();
        for slot in self.own.iter_mut().flatten() {
            *slot = moved_to[*slot];
        }
        for slot in self.members.values_mut() {
            *slot = moved_to[*slot];
        }
    }

    fn index_analyses(&mut self) {
        let mut analyses_at = vec![Vec::new(); self.points.len()];
        let own = self
            .own
            .iter()
            .enumerate()
            .filter_map(|(position, point)| Some((position, (*point)?)));
        let members = self
            .members
            .iter()
            .map(|((position, _), point)| (*position, *point));
        for (position, point) in own.chain(members) {
            analyses_at[point].push(position);
        }
        for analyses in &mut analyses_at {
            analyses.sort_unstable();
            analyses.dedup();
        }
        self.analyses_at = analyses_at;
    }

    fn name_points(&mut self) {
        let axes = varying_axes(&self.points);
        for point in &mut self.points {
            point.label = short_label(point, &axes);
        }
    }
}

/// Which family member a candidate is, as a point: a corner by its label, a
/// sweep point by its index. Monte Carlo trials are not points.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MemberKey {
    Corner(usize),
    Sweep(usize),
}

impl MemberKey {
    fn of(member: &FamilyMemberId) -> Option<Self> {
        match member {
            FamilyMemberId::Corner { index, .. } => Some(Self::Corner(*index)),
            FamilyMemberId::SweepPoint { index, .. } => Some(Self::Sweep(*index)),
            FamilyMemberId::MonteCarloTrial { .. }
            | FamilyMemberId::MonteCarloSequenceTrial { .. } => None,
        }
    }
}

/// The identity two analyses share when they solved the same point.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PointKey {
    Deck,
    Attributed {
        process: String,
        supply_voltage: Option<u64>,
        temperature_celsius: u64,
        corner_contract: Option<ContentDigest>,
        nominal: bool,
        swept_values: Vec<AnalysisResultSweptValue>,
    },
    Corner(String),
    Sweep {
        target: String,
        value: u64,
    },
}

impl PointKey {
    /// Exactly the record's own equality, bit for bit.
    fn attributed(record: &AnalysisResultPvtPoint) -> Self {
        Self::Attributed {
            process: record.process().to_owned(),
            supply_voltage: record.supply_voltage().map(f64::to_bits),
            temperature_celsius: record.temperature_celsius().to_bits(),
            corner_contract: record.corner_contract(),
            nominal: record.is_nominal(),
            swept_values: record.swept_values().to_vec(),
        }
    }
}

/// The target a sweep family names a temperature sweep by.
const TEMPERATURE_TARGET: &str = "TEMP";

/// The members of a corner or sweep family solved inside one analysis, each
/// with the point it names.
fn family_points<W>(analysis: &AnalysisResult<W>) -> Vec<(MemberKey, PointKey, Point)> {
    let Some(metadata) = &analysis.family_metadata else {
        return Vec::new();
    };
    let target = match metadata {
        AnalysisResultFamilyMetadata::Parametric { target, .. } => Some(target.as_str()),
        _ => None,
    };
    let temperatures = match metadata {
        AnalysisResultFamilyMetadata::Corner { temperatures_c, .. } => temperatures_c.as_slice(),
        _ => &[],
    };
    metadata
        .member_measurements()
        .iter()
        .filter_map(|member| match &member.member {
            FamilyMemberId::Corner { index, label } => Some((
                MemberKey::Corner(*index),
                PointKey::Corner(label.clone()),
                corner_point(&member.member, label, temperatures.get(*index).copied()),
            )),
            FamilyMemberId::SweepPoint { index, value } => {
                let target = target.unwrap_or_default();
                Some((
                    MemberKey::Sweep(*index),
                    PointKey::Sweep {
                        target: target.to_ascii_uppercase(),
                        value: value.to_bits(),
                    },
                    sweep_point(&member.member, target, *value),
                ))
            }
            FamilyMemberId::MonteCarloTrial { .. }
            | FamilyMemberId::MonteCarloSequenceTrial { .. } => None,
        })
        .collect()
}

fn deck_point() -> Point {
    Point {
        process: None,
        supply_voltage: None,
        temperature_celsius: None,
        swept_values: Vec::new(),
        nominal: false,
        label: String::new(),
        detail: "The deck's own conditions".to_owned(),
        source: PointSource::Deck,
    }
}

fn attributed_point(record: &AnalysisResultPvtPoint) -> Point {
    let mut point = Point {
        process: Some(record.process().to_owned()),
        supply_voltage: record.supply_voltage(),
        temperature_celsius: Some(record.temperature_celsius()),
        swept_values: record.swept_values().to_vec(),
        nominal: record.is_nominal(),
        label: String::new(),
        detail: String::new(),
        source: PointSource::Attributed(record.clone()),
    };
    point.detail = full_label(&point);
    point
}

/// A corner member, read from the label the executor writes,
/// `{process}_{volts}V_{celsius}C`; a label in any other shape names the
/// point by itself, with the family's temperature for it.
fn corner_point(member: &FamilyMemberId, label: &str, temperature: Option<f64>) -> Point {
    let parsed = parse_corner_label(label);
    let mut point = Point {
        process: parsed.as_ref().map(|corner| corner.0.clone()),
        supply_voltage: parsed.as_ref().map(|corner| corner.1),
        temperature_celsius: parsed.as_ref().map(|corner| corner.2).or(temperature),
        swept_values: Vec::new(),
        nominal: false,
        label: String::new(),
        detail: String::new(),
        source: PointSource::Member(member.clone()),
    };
    point.detail = if parsed.is_some() {
        full_label(&point)
    } else {
        label.to_owned()
    };
    point
}

fn parse_corner_label(label: &str) -> Option<(String, f64, f64)> {
    let mut parts = label.rsplitn(3, '_');
    let temperature = parts.next()?.strip_suffix('C')?.parse::<f64>().ok()?;
    let voltage = parts.next()?.strip_suffix('V')?.parse::<f64>().ok()?;
    let process = parts.next()?.trim();
    (!process.is_empty() && temperature.is_finite() && voltage.is_finite())
        .then(|| (process.to_owned(), voltage, temperature))
}

/// A sweep member: a temperature when the family swept `TEMP`, otherwise the
/// value its target took.
fn sweep_point(member: &FamilyMemberId, target: &str, value: f64) -> Point {
    let temperature = target.eq_ignore_ascii_case(TEMPERATURE_TARGET);
    let mut point = Point {
        process: None,
        supply_voltage: None,
        temperature_celsius: temperature.then_some(value),
        swept_values: if temperature {
            Vec::new()
        } else {
            vec![AnalysisResultSweptValue {
                axis: AnalysisResultSweptAxis::Parameter,
                name: target.to_owned(),
                value: number(value),
            }]
        },
        nominal: false,
        label: String::new(),
        detail: String::new(),
        source: PointSource::Member(member.clone()),
    };
    point.detail = full_label(&point);
    point
}

/// A coordinate as few digits as it needs, with a true minus sign.
fn number(value: f64) -> String {
    let rounded = (value * 1.0e6).round() / 1.0e6;
    format!("{rounded}").replace('-', "\u{2212}")
}

fn process_text(point: &Point) -> Option<String> {
    point.process.clone()
}

fn supply_text(point: &Point) -> Option<String> {
    point
        .supply_voltage
        .map(|volts| format!("{} V", number(volts)))
}

fn temperature_text(point: &Point) -> Option<String> {
    point
        .temperature_celsius
        .map(|celsius| format!("{} \u{00b0}C", number(celsius)))
}

fn swept_text(value: &AnalysisResultSweptValue) -> String {
    format!("{} = {}", value.name, value.value)
}

fn full_label(point: &Point) -> String {
    let mut parts = [
        process_text(point),
        supply_text(point),
        temperature_text(point),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    parts.extend(point.swept_values.iter().map(swept_text));
    parts.join(" \u{00b7} ")
}

/// A point's head: only the coordinates that vary across the run, and the
/// process alone when nothing does.
fn short_label(point: &Point, axes: &[PointAxis]) -> String {
    if point.source == PointSource::Deck {
        return "Deck".to_owned();
    }
    if let PointSource::Member(FamilyMemberId::Corner { label, .. }) = &point.source
        && point.process.is_none()
    {
        return label.clone();
    }
    let parts = axes
        .iter()
        .filter_map(|axis| match axis {
            PointAxis::Process => process_text(point),
            PointAxis::Supply => supply_text(point),
            PointAxis::Temperature => temperature_text(point),
            PointAxis::Swept { axis, name } => point
                .swept_values
                .iter()
                .find(|value| value.axis == *axis && value.name == *name)
                .map(swept_text),
        })
        .collect::<Vec<_>>();
    if parts.is_empty() {
        process_text(point).unwrap_or_else(|| point.detail.clone())
    } else {
        parts.join(" ")
    }
}

fn varying_axes(points: &[Point]) -> Vec<PointAxis> {
    fn varies<T: PartialEq>(values: impl Iterator<Item = T>) -> bool {
        let mut seen: Vec<T> = Vec::new();
        for value in values {
            if !seen.contains(&value) {
                seen.push(value);
            }
            if seen.len() > 1 {
                return true;
            }
        }
        false
    }
    let mut axes = Vec::new();
    if varies(points.iter().map(|point| point.process.clone())) {
        axes.push(PointAxis::Process);
    }
    if varies(
        points
            .iter()
            .map(|point| point.supply_voltage.map(f64::to_bits)),
    ) {
        axes.push(PointAxis::Supply);
    }
    if varies(
        points
            .iter()
            .map(|point| point.temperature_celsius.map(f64::to_bits)),
    ) {
        axes.push(PointAxis::Temperature);
    }
    let mut swept: Vec<(AnalysisResultSweptAxis, String)> = Vec::new();
    for value in points.iter().flat_map(|point| &point.swept_values) {
        if !swept
            .iter()
            .any(|(axis, name)| *axis == value.axis && *name == value.name)
        {
            swept.push((value.axis, value.name.clone()));
        }
    }
    for (axis, name) in swept {
        let values = points.iter().map(|point| {
            point
                .swept_values
                .iter()
                .find(|value| value.axis == axis && value.name == name)
                .map(|value| value.value.clone())
        });
        if varies(values) {
            axes.push(PointAxis::Swept { axis, name });
        }
    }
    axes
}
