//! Typed projection and filtering for retained simulation families.
//!
//! This module deliberately operates on immutable [`AnalysisResult`] source
//! evidence. It never derives dimensions from display labels and never
//! manufactures coordinates for failed points whose exact coordinates were
//! not retained by the solver.

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use crate::product::DatasetId;
use crate::results::visualization_document::{
    FamilyComparisonOperator, FamilyDimension as DocumentFamilyDimension, FamilyFilterExpression,
    FamilyPredicate, TypedValue, ValueType,
};
use crate::state::{AnalysisResult, AnalysisResultFamilyMetadata};

/// Presentation-only selection of exact source sample rows for one immutable
/// analysis. The dataset and analysis identities prevent a selection from
/// being applied to a later run merely because its arrays have the same size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceSampleSelection {
    pub dataset_id: DatasetId,
    pub analysis_sequence: u64,
    pub source_indices: Vec<usize>,
}

impl SourceSampleSelection {
    pub fn new(
        dataset_id: DatasetId,
        analysis_sequence: u64,
        source_indices: Vec<usize>,
    ) -> Result<Self, String> {
        if source_indices.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err("selected family sample indices must be unique and ascending".to_owned());
        }
        Ok(Self {
            dataset_id,
            analysis_sequence,
            source_indices,
        })
    }

    pub fn fingerprint(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.dataset_id.hash(&mut hasher);
        self.analysis_sequence.hash(&mut hasher);
        self.source_indices.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FamilyValueKind {
    Number,
    Integer,
    Text,
    Status,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FamilyValue {
    Number(f64),
    Integer(u64),
    Text(String),
    Status(FamilyPointStatus),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FamilyPointStatus {
    Retained,
    NotRun,
}

impl FamilyPointStatus {
    const fn query_name(self) -> &'static str {
        match self {
            Self::Retained => "retained",
            Self::NotRun => "not-run",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FamilyDimension {
    pub id: String,
    pub label: String,
    pub unit: Option<String>,
    pub kind: FamilyValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FamilyPoint {
    /// Exact index in every compatible source waveform.
    pub source_index: usize,
    pub values: BTreeMap<String, FamilyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FamilyManifest {
    pub dimensions: Vec<FamilyDimension>,
    pub points: Vec<FamilyPoint>,
    /// Count of failed/unavailable points retained by source metadata without
    /// exact coordinates. These are omissions, never synthetic points.
    pub omitted_points: usize,
}

impl FamilyManifest {
    pub fn from_analysis(analysis: &AnalysisResult) -> Result<Option<Self>, String> {
        let Some(metadata) = analysis.family_metadata.as_ref() else {
            return Ok(None);
        };
        metadata.validate_for(analysis.analysis_type)?;

        let mut dimensions = vec![dimension(
            "sample",
            "Sample",
            None,
            FamilyValueKind::Integer,
        )];
        let (mut points, omitted_points) = match metadata {
            AnalysisResultFamilyMetadata::Parametric {
                target,
                sweep_values,
                failed_points,
            } => {
                dimensions.insert(0, dimension(target, target, None, FamilyValueKind::Number));
                (indexed_numeric_points(target, sweep_values), *failed_points)
            }
            AnalysisResultFamilyMetadata::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                failed_corners,
            } => {
                dimensions.splice(
                    0..0,
                    [
                        dimension("process", "Process", None, FamilyValueKind::Text),
                        dimension(
                            "temperature",
                            "Temperature",
                            Some("°C"),
                            FamilyValueKind::Number,
                        ),
                        dimension(
                            x_label,
                            x_label,
                            (!x_unit.trim().is_empty()).then_some(x_unit.as_str()),
                            FamilyValueKind::Number,
                        ),
                    ],
                );
                let points = x_values
                    .iter()
                    .zip(temperatures_c)
                    .zip(corner_labels)
                    .enumerate()
                    .map(|(index, ((x, temperature), process))| {
                        point(
                            index,
                            [
                                ("process", FamilyValue::Text(process.clone())),
                                ("temperature", FamilyValue::Number(*temperature)),
                                (x_label.as_str(), FamilyValue::Number(*x)),
                            ],
                        )
                    })
                    .collect();
                (points, *failed_corners)
            }
            AnalysisResultFamilyMetadata::MonteCarlo {
                runs_completed,
                failures,
                variables,
                ..
            } => {
                for variable in variables {
                    if variable.samples.len() != *runs_completed {
                        return Err(format!(
                            "Monte Carlo variable '{}' has {} retained samples for {runs_completed} completed runs",
                            variable.name,
                            variable.samples.len()
                        ));
                    }
                    dimensions.insert(
                        dimensions.len() - 1,
                        dimension(
                            &variable.name,
                            &variable.name,
                            None,
                            FamilyValueKind::Number,
                        ),
                    );
                }
                let points = (0..*runs_completed)
                    .map(|index| {
                        let values = variables.iter().map(|variable| {
                            (
                                variable.name.as_str(),
                                FamilyValue::Number(variable.samples[index]),
                            )
                        });
                        point(index, values)
                    })
                    .collect();
                (points, *failures)
            }
            AnalysisResultFamilyMetadata::Reliability { years } => {
                dimensions.insert(
                    0,
                    dimension("years", "Years", Some("yr"), FamilyValueKind::Number),
                );
                (indexed_numeric_points("years", years), 0)
            }
            AnalysisResultFamilyMetadata::Optimization { iterations, .. } => {
                dimensions.insert(
                    0,
                    dimension("iteration", "Iteration", None, FamilyValueKind::Number),
                );
                (indexed_numeric_points("iteration", iterations), 0)
            }
            AnalysisResultFamilyMetadata::Soa { time } => {
                dimensions.insert(
                    0,
                    dimension("time", "Time", Some("s"), FamilyValueKind::Number),
                );
                (indexed_numeric_points("time", time), 0)
            }
        };

        dimensions.push(dimension("status", "Status", None, FamilyValueKind::Status));
        for family_point in &mut points {
            family_point.values.insert(
                "sample".to_owned(),
                FamilyValue::Integer(family_point.source_index as u64 + 1),
            );
            family_point.values.insert(
                "status".to_owned(),
                FamilyValue::Status(FamilyPointStatus::Retained),
            );
        }
        Ok(Some(Self {
            dimensions,
            points,
            omitted_points,
        }))
    }

    pub fn dimension(&self, id: &str) -> Option<&FamilyDimension> {
        self.dimensions
            .iter()
            .find(|dimension| dimension.id.eq_ignore_ascii_case(id.trim()))
    }

    pub fn matching_source_indices(&self, query: &str) -> Result<Vec<usize>, String> {
        let predicates = parse_query(self, query)?;
        Ok(self
            .points
            .iter()
            .filter(|point| predicates.iter().all(|predicate| predicate.matches(point)))
            .map(|point| point.source_index)
            .collect())
    }

    pub fn compile_filter(&self, query: &str) -> Result<Option<FamilyFilterExpression>, String> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(None);
        }
        let predicates = parse_query(self, query)?;
        let mut compiled = predicates
            .into_iter()
            .map(|predicate| predicate.to_document(self))
            .collect::<Result<Vec<_>, _>>()?;
        let predicate = if compiled.len() == 1 {
            compiled.pop().expect("one compiled family predicate")
        } else {
            FamilyPredicate::All {
                predicates: compiled,
            }
        };
        Ok(Some(FamilyFilterExpression {
            source: query.to_owned(),
            predicate,
        }))
    }

    pub fn compatible_waveform_len(&self, sample_count: usize) -> Result<(), String> {
        if sample_count == self.points.len() {
            Ok(())
        } else {
            Err(format!(
                "family contains {} retained points but the source waveform contains {sample_count} samples",
                self.points.len()
            ))
        }
    }
}

fn dimension(id: &str, label: &str, unit: Option<&str>, kind: FamilyValueKind) -> FamilyDimension {
    FamilyDimension {
        id: id.trim().to_owned(),
        label: label.trim().to_owned(),
        unit: unit.map(str::to_owned),
        kind,
    }
}

fn indexed_numeric_points(id: &str, values: &[f64]) -> Vec<FamilyPoint> {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| point(index, [(id, FamilyValue::Number(*value))]))
        .collect()
}

fn point<'a>(
    source_index: usize,
    values: impl IntoIterator<Item = (&'a str, FamilyValue)>,
) -> FamilyPoint {
    FamilyPoint {
        source_index,
        values: values
            .into_iter()
            .map(|(id, value)| (id.to_owned(), value))
            .collect(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonOperator {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
enum Predicate {
    Compare {
        dimension: String,
        operator: ComparisonOperator,
        value: FamilyValue,
    },
    In {
        dimension: String,
        values: Vec<FamilyValue>,
    },
}

impl Predicate {
    fn matches(&self, point: &FamilyPoint) -> bool {
        match self {
            Self::Compare {
                dimension,
                operator,
                value,
            } => point
                .values
                .get(dimension)
                .is_some_and(|candidate| compare_values(candidate, value, *operator)),
            Self::In { dimension, values } => {
                point.values.get(dimension).is_some_and(|candidate| {
                    values
                        .iter()
                        .any(|value| compare_values(candidate, value, ComparisonOperator::Equal))
                })
            }
        }
    }

    fn to_document(&self, manifest: &FamilyManifest) -> Result<FamilyPredicate, String> {
        match self {
            Self::Compare {
                dimension,
                operator,
                value,
            } => Ok(FamilyPredicate::Compare {
                dimension: document_dimension(manifest, dimension)?,
                operator: match operator {
                    ComparisonOperator::Equal => FamilyComparisonOperator::Equal,
                    ComparisonOperator::NotEqual => FamilyComparisonOperator::NotEqual,
                    ComparisonOperator::Greater => FamilyComparisonOperator::GreaterThan,
                    ComparisonOperator::GreaterOrEqual => {
                        FamilyComparisonOperator::GreaterThanOrEqual
                    }
                    ComparisonOperator::Less => FamilyComparisonOperator::LessThan,
                    ComparisonOperator::LessOrEqual => FamilyComparisonOperator::LessThanOrEqual,
                },
                value: document_value(value)?,
            }),
            Self::In { dimension, values } => Ok(FamilyPredicate::In {
                dimension: document_dimension(manifest, dimension)?,
                values: values
                    .iter()
                    .map(document_value)
                    .collect::<Result<_, _>>()?,
            }),
        }
    }
}

fn document_dimension(
    manifest: &FamilyManifest,
    id: &str,
) -> Result<DocumentFamilyDimension, String> {
    let dimension = manifest
        .dimension(id)
        .ok_or_else(|| format!("unknown family dimension '{id}'"))?;
    DocumentFamilyDimension::new(
        dimension.id.clone(),
        match dimension.kind {
            FamilyValueKind::Number => ValueType::Real,
            FamilyValueKind::Integer => ValueType::Integer,
            FamilyValueKind::Text | FamilyValueKind::Status => ValueType::Text,
        },
    )
    .map_err(|error| error.to_string())
}

fn document_value(value: &FamilyValue) -> Result<TypedValue, String> {
    match value {
        FamilyValue::Number(value) => Ok(TypedValue::Real(*value)),
        FamilyValue::Integer(value) => i64::try_from(*value)
            .map(TypedValue::Integer)
            .map_err(|_| format!("family integer {value} exceeds the supported signed range")),
        FamilyValue::Text(value) => Ok(TypedValue::Text(value.clone())),
        FamilyValue::Status(value) => Ok(TypedValue::Text(value.query_name().to_owned())),
    }
}

fn compare_values(left: &FamilyValue, right: &FamilyValue, operator: ComparisonOperator) -> bool {
    let ordering = match (left, right) {
        (FamilyValue::Number(left), FamilyValue::Number(right)) => left.partial_cmp(right),
        (FamilyValue::Integer(left), FamilyValue::Integer(right)) => Some(left.cmp(right)),
        (FamilyValue::Text(left), FamilyValue::Text(right)) => {
            Some(left.to_ascii_lowercase().cmp(&right.to_ascii_lowercase()))
        }
        (FamilyValue::Status(left), FamilyValue::Status(right)) => {
            Some(left.query_name().cmp(right.query_name()))
        }
        _ => None,
    };
    match operator {
        ComparisonOperator::Equal => ordering == Some(std::cmp::Ordering::Equal),
        ComparisonOperator::NotEqual => ordering != Some(std::cmp::Ordering::Equal),
        ComparisonOperator::Greater => ordering == Some(std::cmp::Ordering::Greater),
        ComparisonOperator::GreaterOrEqual => matches!(
            ordering,
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        ),
        ComparisonOperator::Less => ordering == Some(std::cmp::Ordering::Less),
        ComparisonOperator::LessOrEqual => matches!(
            ordering,
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        ),
    }
}

fn parse_query(manifest: &FamilyManifest, query: &str) -> Result<Vec<Predicate>, String> {
    let query = query.trim();
    if query.is_empty() {
        return Ok(Vec::new());
    }
    split_conjunction(query)
        .into_iter()
        .map(|clause| parse_clause(manifest, clause))
        .collect()
}

fn split_conjunction(query: &str) -> Vec<&str> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut start = 0usize;
    let mut clauses = Vec::new();
    for (index, ch) in query.char_indices() {
        if matches!(ch, '\'' | '"') {
            if quote == Some(ch) {
                quote = None;
            } else if quote.is_none() {
                quote = Some(ch);
            }
        } else if quote.is_none() {
            match ch {
                '{' => depth += 1,
                '}' => depth = depth.saturating_sub(1),
                '·' if depth == 0 => {
                    clauses.push(query[start..index].trim());
                    start = index + ch.len_utf8();
                }
                _ if depth == 0
                    && query[index..]
                        .get(..3)
                        .is_some_and(|token| token.eq_ignore_ascii_case("and"))
                    && query[..index]
                        .chars()
                        .next_back()
                        .is_none_or(char::is_whitespace)
                    && query
                        .get(index + 3..)
                        .and_then(|remaining| remaining.chars().next())
                        .is_none_or(char::is_whitespace) =>
                {
                    clauses.push(query[start..index].trim());
                    start = index + 3;
                }
                _ => {}
            }
        }
    }
    clauses.push(query[start..].trim());
    clauses
}

fn parse_clause(manifest: &FamilyManifest, clause: &str) -> Result<Predicate, String> {
    if clause.is_empty() {
        return Err("family filter contains an empty clause".to_owned());
    }
    if let Some((left, right)) = split_keyword(clause, "in") {
        let dimension = canonical_dimension(manifest, left)?;
        let right = right.trim();
        if !(right.starts_with('{') && right.ends_with('}')) {
            return Err(format!(
                "set membership for '{}' must use braces",
                dimension.id
            ));
        }
        let values = right[1..right.len() - 1]
            .split(',')
            .map(|value| parse_value(dimension, value))
            .collect::<Result<Vec<_>, _>>()?;
        if values.is_empty() {
            return Err(format!("filter set for '{}' is empty", dimension.id));
        }
        return Ok(Predicate::In {
            dimension: dimension.id.clone(),
            values,
        });
    }

    for (symbol, operator) in [
        (">=", ComparisonOperator::GreaterOrEqual),
        ("<=", ComparisonOperator::LessOrEqual),
        ("!=", ComparisonOperator::NotEqual),
        ("=", ComparisonOperator::Equal),
        (">", ComparisonOperator::Greater),
        ("<", ComparisonOperator::Less),
    ] {
        if let Some((left, right)) = clause.split_once(symbol) {
            let dimension = canonical_dimension(manifest, left)?;
            return Ok(Predicate::Compare {
                dimension: dimension.id.clone(),
                operator,
                value: parse_value(dimension, right)?,
            });
        }
    }
    Err(format!("unsupported family filter clause '{clause}'"))
}

fn split_keyword<'a>(value: &'a str, keyword: &str) -> Option<(&'a str, &'a str)> {
    value.char_indices().find_map(|(index, _)| {
        let end = index + keyword.len();
        (end <= value.len()
            && value
                .get(index..end)
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(keyword))
            && index > 0
            && value[..index]
                .chars()
                .next_back()
                .is_some_and(char::is_whitespace)
            && value
                .get(end..)
                .and_then(|remaining| remaining.chars().next())
                .is_some_and(char::is_whitespace))
        .then(|| (&value[..index], &value[end..]))
    })
}

fn canonical_dimension<'a>(
    manifest: &'a FamilyManifest,
    raw: &str,
) -> Result<&'a FamilyDimension, String> {
    manifest
        .dimension(raw)
        .ok_or_else(|| format!("unknown family dimension '{}'", raw.trim()))
}

fn parse_value(dimension: &FamilyDimension, raw: &str) -> Result<FamilyValue, String> {
    let raw = raw.trim().trim_matches(['\'', '"']);
    if raw.is_empty() {
        return Err(format!("filter value for '{}' is empty", dimension.id));
    }
    match dimension.kind {
        FamilyValueKind::Number => {
            let number = strip_expected_unit(raw, dimension.unit.as_deref())?
                .parse::<f64>()
                .map_err(|_| format!("'{raw}' is not a number for '{}'", dimension.id))?;
            if !number.is_finite() {
                return Err(format!("'{raw}' is not finite for '{}'", dimension.id));
            }
            Ok(FamilyValue::Number(number))
        }
        FamilyValueKind::Integer => raw
            .parse::<u64>()
            .map(FamilyValue::Integer)
            .map_err(|_| format!("'{raw}' is not an integer for '{}'", dimension.id)),
        FamilyValueKind::Text => Ok(FamilyValue::Text(raw.to_owned())),
        FamilyValueKind::Status => match raw.to_ascii_lowercase().as_str() {
            "retained" | "complete" | "completed" => {
                Ok(FamilyValue::Status(FamilyPointStatus::Retained))
            }
            "not-run" | "not_run" | "missing" => Ok(FamilyValue::Status(FamilyPointStatus::NotRun)),
            _ => Err(format!("unknown family status '{raw}'")),
        },
    }
}

fn strip_expected_unit<'a>(raw: &'a str, unit: Option<&str>) -> Result<&'a str, String> {
    let Some(unit) = unit else {
        return Ok(raw);
    };
    let raw = raw.trim();
    if let Some(value) = raw.strip_suffix(unit) {
        return Ok(value.trim());
    }
    if raw.chars().any(|ch| ch.is_alphabetic() || ch == '°') {
        Err(format!("'{raw}' does not use expected unit {unit}"))
    } else {
        Ok(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisType, MonteCarloVariableMetadata};

    fn corner_result() -> AnalysisResult {
        AnalysisResult::new(7, AnalysisType::Corner, "PVT").with_family_metadata(
            AnalysisResultFamilyMetadata::Corner {
                x_values: vec![1.0, 2.0, 3.0],
                x_label: "RGAIN".to_owned(),
                x_unit: "kΩ".to_owned(),
                temperatures_c: vec![-40.0, 27.0, 125.0],
                corner_labels: vec!["SS".to_owned(), "TT".to_owned(), "FF".to_owned()],
                failed_corners: 2,
            },
        )
    }

    #[test]
    fn corner_projection_is_exact_and_records_omissions() {
        let manifest = FamilyManifest::from_analysis(&corner_result())
            .expect("valid metadata")
            .expect("family");
        assert_eq!(manifest.points.len(), 3);
        assert_eq!(manifest.omitted_points, 2);
        assert_eq!(
            manifest.points[0].values.get("process"),
            Some(&FamilyValue::Text("SS".to_owned()))
        );
        assert_eq!(
            manifest.points[1].values.get("temperature"),
            Some(&FamilyValue::Number(27.0))
        );
        assert_eq!(
            manifest.points[1].values.get("sample"),
            Some(&FamilyValue::Integer(2))
        );
    }

    #[test]
    fn exact_mockup_query_filters_typed_dimensions() {
        let manifest = FamilyManifest::from_analysis(&corner_result())
            .unwrap()
            .unwrap();
        assert_eq!(
            manifest
                .matching_source_indices(
                    "process in {TT,SS} and temperature >= 27°C and status != not-run"
                )
                .unwrap(),
            [1]
        );
    }

    #[test]
    fn exact_mockup_slice_separator_filters_typed_dimensions() {
        let manifest = FamilyManifest::from_analysis(&corner_result())
            .unwrap()
            .unwrap();
        assert_eq!(
            manifest
                .matching_source_indices("temperature in {27,125} · status != not-run")
                .unwrap(),
            [1, 2]
        );
    }

    #[test]
    fn invalid_dimension_and_unit_fail_closed() {
        let manifest = FamilyManifest::from_analysis(&corner_result())
            .unwrap()
            .unwrap();
        assert!(
            manifest
                .matching_source_indices("voltage >= 1")
                .unwrap_err()
                .contains("unknown family dimension")
        );
        assert!(
            manifest
                .matching_source_indices("temperature >= 27K")
                .unwrap_err()
                .contains("expected unit °C")
        );
    }

    #[test]
    fn monte_carlo_requires_one_sample_per_completed_run() {
        let analysis = AnalysisResult::new(8, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 2,
                runs_requested: 3,
                runs_completed: 2,
                failures: 1,
                all_converged: false,
                variables: vec![MonteCarloVariableMetadata {
                    name: "gain".to_owned(),
                    samples: vec![1.0],
                    mean: 1.0,
                    std_dev: 0.0,
                    min: 1.0,
                    max: 1.0,
                }],
            },
        );
        assert!(
            FamilyManifest::from_analysis(&analysis)
                .unwrap_err()
                .contains("1 retained samples for 2 completed runs")
        );
    }

    #[test]
    fn waveform_compatibility_prevents_index_invention() {
        let manifest = FamilyManifest::from_analysis(&corner_result())
            .unwrap()
            .unwrap();
        assert!(manifest.compatible_waveform_len(3).is_ok());
        assert!(manifest.compatible_waveform_len(12).is_err());
    }
}
