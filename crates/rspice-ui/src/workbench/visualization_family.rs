//! Typed projection and filtering for retained simulation families.
//!
//! This module deliberately operates on immutable [`AnalysisResult`] source
//! evidence. It never derives dimensions from display labels and never
//! manufactures coordinates for failed points whose exact coordinates were
//! not retained by the solver.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};

use crate::product::DatasetId;
use crate::results::visualization_document::{
    AccessibleColorPalette, FamilyAggregationMethod, FamilyComparisonOperator,
    FamilyDimension as DocumentFamilyDimension, FamilyEncodingMap, FamilyFilterExpression,
    FamilyPredicate, FamilyPresentationPolicy, FamilyXOrdering, TypedValue, ValueType,
};
use crate::state::{AnalysisResult, AnalysisResultFamilyMetadata};

/// Presentation-only selection of exact source sample rows for one immutable
/// analysis. The dataset and analysis identities prevent a selection from
/// being applied to a later run merely because its arrays have the same size.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SourceSampleSelection {
    pub dataset_id: DatasetId,
    pub analysis_sequence: u64,
    pub source_indices: Vec<usize>,
    family_render_plan: Option<FamilyRenderPlan>,
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
            family_render_plan: None,
        })
    }

    /// Attach the renderer projection compiled from the already-validated
    /// pane policy. The immutable source rows remain identified by their
    /// original indices; this plan only groups and styles those rows.
    pub fn with_family_presentation(
        mut self,
        manifest: &FamilyManifest,
        policy: &FamilyPresentationPolicy,
    ) -> Result<Self, String> {
        self.family_render_plan = Some(FamilyRenderPlan::compile(
            manifest,
            policy,
            &self.source_indices,
        )?);
        Ok(self)
    }

    pub fn family_render_plan(&self) -> Option<&FamilyRenderPlan> {
        self.family_render_plan.as_ref()
    }

    /// Resolve an overlay analysis onto the active family policy and exact X
    /// domain. The caller must suppress the overlay when this returns an
    /// error; falling back to the overlay waveform's native X would mix
    /// incompatible domains in one pane.
    pub fn overlay_render_plan(
        &self,
        analysis: &AnalysisResult,
    ) -> Result<Option<FamilyRenderPlan>, String> {
        let Some(active_plan) = &self.family_render_plan else {
            return Ok(None);
        };
        let manifest = FamilyManifest::from_analysis(analysis)?
            .ok_or_else(|| "overlay analysis has no retained family manifest".to_owned())?;
        active_plan.project_overlay(&manifest).map(Some)
    }

    pub fn fingerprint(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.dataset_id.hash(&mut hasher);
        self.analysis_sequence.hash(&mut hasher);
        self.source_indices.hash(&mut hasher);
        self.family_render_plan
            .as_ref()
            .map(FamilyRenderPlan::fingerprint)
            .hash(&mut hasher);
        hasher.finish()
    }
}

/// Renderer-neutral, exact-row projection of a family policy. The plan is
/// compiled once at the Visualization Studio boundary so the waveform view
/// never reinterprets display labels or solver metadata.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FamilyRenderPlan {
    policy: FamilyPresentationPolicy,
    x_axis: FamilyXAxis,
    groups: Vec<FamilyRenderGroup>,
}

impl FamilyRenderPlan {
    fn compile(
        manifest: &FamilyManifest,
        policy: &FamilyPresentationPolicy,
        selected_indices: &[usize],
    ) -> Result<Self, String> {
        policy.validate().map_err(|error| error.to_string())?;
        if policy.aggregation.method != FamilyAggregationMethod::None {
            return Err(
                "aggregated family rendering is not supported by the waveform renderer".to_owned(),
            );
        }
        if policy.x_dimension.ordering != FamilyXOrdering::Source {
            return Err(
                "the waveform renderer currently requires exact source ordering for family X values"
                    .to_owned(),
            );
        }
        if policy
            .encodings
            .iter()
            .any(|encoding| matches!(encoding, FamilyEncodingMap::Facet { .. }))
        {
            return Err(
                "faceted family policies require a faceted renderer and cannot be flattened into one waveform pane"
                    .to_owned(),
            );
        }

        require_manifest_dimension(manifest, &policy.x_dimension.dimension)?;
        for dimension in &policy.family_dimensions {
            require_manifest_dimension(manifest, dimension)?;
        }
        let x_dimension = manifest
            .dimension(&policy.x_dimension.dimension.key)
            .ok_or_else(|| {
                format!(
                    "family X dimension '{}' is unavailable",
                    policy.x_dimension.dimension.key
                )
            })?;
        if !matches!(
            x_dimension.kind,
            FamilyValueKind::Number | FamilyValueKind::Integer
        ) {
            return Err(format!(
                "family X dimension '{}' must contain exact numeric values",
                x_dimension.id
            ));
        }
        let x_axis = FamilyXAxis {
            dimension_key: x_dimension.id.clone(),
            label: x_dimension.label.clone(),
            unit: x_dimension.unit.clone().unwrap_or_default(),
        };

        let selected: BTreeSet<usize> = selected_indices.iter().copied().collect();
        if selected.len() != selected_indices.len() {
            return Err("selected family sample indices must be unique".to_owned());
        }
        let available: BTreeSet<usize> = manifest
            .points
            .iter()
            .map(|point| point.source_index)
            .collect();
        if !selected.is_subset(&available) {
            return Err(
                "selected family rows are not present in the immutable manifest".to_owned(),
            );
        }

        let category_tables = policy
            .encodings
            .iter()
            .map(|encoding| {
                let dimension = encoding.dimension();
                let values = manifest
                    .points
                    .iter()
                    .filter_map(|point| point.values.get(&dimension.key))
                    .map(CanonicalFamilyValue::from)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();
                (dimension.key.clone(), values)
            })
            .collect::<BTreeMap<_, _>>();

        let points_by_index = manifest
            .points
            .iter()
            .map(|point| (point.source_index, point))
            .collect::<BTreeMap<_, _>>();
        let mut grouped = BTreeMap::<Vec<CanonicalFamilyValue>, Vec<usize>>::new();
        for source_index in selected_indices {
            let point = points_by_index
                .get(source_index)
                .ok_or_else(|| "selected family row disappeared from the manifest".to_owned())?;
            let key = policy
                .family_dimensions
                .iter()
                .map(|dimension| {
                    point
                        .values
                        .get(&dimension.key)
                        .map(CanonicalFamilyValue::from)
                        .ok_or_else(|| {
                            format!(
                                "family row {source_index} is missing dimension '{}'",
                                dimension.key
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            grouped.entry(key).or_default().push(*source_index);
        }

        let mut groups = Vec::with_capacity(grouped.len());
        for (key, source_indices) in grouped {
            let first_index = *source_indices
                .first()
                .ok_or_else(|| "family render group is empty".to_owned())?;
            let point = points_by_index
                .get(&first_index)
                .ok_or_else(|| "family render group lost its source row".to_owned())?;
            let mut style = FamilyTraceStyle::default();
            let mut explicit_labels = BTreeMap::new();
            for encoding in &policy.encodings {
                let dimension = encoding.dimension();
                let value = point.values.get(&dimension.key).ok_or_else(|| {
                    format!(
                        "family row {first_index} is missing encoded dimension '{}'",
                        dimension.key
                    )
                })?;
                let canonical = CanonicalFamilyValue::from(value);
                let categories = category_tables.get(&dimension.key).ok_or_else(|| {
                    format!("family category table '{}' is unavailable", dimension.key)
                })?;
                let ordinal = categories
                    .binary_search(&canonical)
                    .map_err(|_| "family category is absent from its stable table".to_owned())?;
                match encoding {
                    FamilyEncodingMap::Color { palette, .. } => {
                        style.color = Some(FamilyColorStyle {
                            palette: *palette,
                            ordinal,
                            category_count: categories.len(),
                        });
                    }
                    FamilyEncodingMap::Dash { .. } => style.dash_ordinal = Some(ordinal),
                    FamilyEncodingMap::Marker { .. } => style.marker_ordinal = Some(ordinal),
                    FamilyEncodingMap::Thickness {
                        minimum_points,
                        maximum_points,
                        ..
                    } => {
                        style.width_points = Some(interpolate_width(
                            value,
                            categories,
                            *minimum_points,
                            *maximum_points,
                        )?);
                    }
                    FamilyEncodingMap::Label { prefix, .. } => {
                        let value = display_family_value(value, manifest.dimension(&dimension.key));
                        explicit_labels.insert(
                            dimension.key.clone(),
                            match prefix {
                                Some(prefix) => format!("{prefix}{value}"),
                                None => value,
                            },
                        );
                    }
                    FamilyEncodingMap::Facet { .. } => unreachable!("facets rejected above"),
                }
            }
            let label = policy
                .family_dimensions
                .iter()
                .zip(&key)
                .map(|(dimension, value)| {
                    let manifest_dimension = manifest.dimension(&dimension.key);
                    let value = explicit_labels
                        .get(&dimension.key)
                        .cloned()
                        .unwrap_or_else(|| {
                            display_family_value(&value.to_family_value(), manifest_dimension)
                        });
                    format!(
                        "{}={value}",
                        manifest_dimension
                            .map_or(dimension.key.as_str(), |item| item.label.as_str()),
                    )
                })
                .collect::<Vec<_>>()
                .join(" · ");
            let x_values = source_indices
                .iter()
                .map(|source_index| {
                    let point = points_by_index.get(source_index).ok_or_else(|| {
                        format!("family X row {source_index} disappeared from the manifest")
                    })?;
                    let value = point.values.get(&x_axis.dimension_key).ok_or_else(|| {
                        format!(
                            "family row {source_index} is missing X dimension '{}'",
                            x_axis.dimension_key
                        )
                    })?;
                    exact_numeric_x(value, &x_axis.dimension_key, *source_index)
                })
                .collect::<Result<Vec<_>, _>>()?;
            if x_values.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err(format!(
                    "family X dimension '{}' must be strictly increasing in source order within every render group",
                    x_axis.dimension_key
                ));
            }
            groups.push(FamilyRenderGroup {
                ordinal: groups.len(),
                stable_key: stable_group_key(&policy.family_dimensions, &key),
                label,
                source_indices,
                x_values,
                style,
                identity_values: key,
            });
        }
        Ok(Self {
            policy: policy.clone(),
            x_axis,
            groups,
        })
    }

    pub fn groups(&self) -> &[FamilyRenderGroup] {
        &self.groups
    }

    pub fn x_axis(&self) -> &FamilyXAxis {
        &self.x_axis
    }

    /// Compile the same exact policy against an overlay manifest, then map
    /// each active group/X coordinate to one and only one overlay source row.
    /// Any semantic or coordinate mismatch fails closed before a trace can be
    /// drawn on the active domain.
    fn project_overlay(&self, manifest: &FamilyManifest) -> Result<Self, String> {
        let filtered_indices =
            manifest.matching_source_indices_for_filter(self.policy.filter.as_ref())?;
        let candidate = Self::compile(manifest, &self.policy, &filtered_indices)?;
        if candidate.x_axis != self.x_axis {
            return Err(format!(
                "overlay family X axis differs: expected '{}' [{}], found '{}' [{}]",
                self.x_axis.label, self.x_axis.unit, candidate.x_axis.label, candidate.x_axis.unit
            ));
        }

        let mut groups = Vec::with_capacity(self.groups.len());
        for active in &self.groups {
            let overlay = candidate
                .groups
                .iter()
                .find(|group| group.identity_values == active.identity_values)
                .ok_or_else(|| {
                    format!(
                        "overlay family is missing presentation group '{}'",
                        active.label
                    )
                })?;
            let mut source_indices = Vec::with_capacity(active.x_values.len());
            for active_x in &active.x_values {
                let mut matches = overlay
                    .x_values
                    .iter()
                    .enumerate()
                    .filter(|(_, overlay_x)| overlay_x.to_bits() == active_x.to_bits());
                let (position, _) = matches.next().ok_or_else(|| {
                    format!(
                        "overlay group '{}' is missing exact X coordinate {active_x}",
                        active.label
                    )
                })?;
                if matches.next().is_some() {
                    return Err(format!(
                        "overlay group '{}' contains duplicate exact X coordinate {active_x}",
                        active.label
                    ));
                }
                source_indices.push(overlay.source_indices[position]);
            }
            groups.push(FamilyRenderGroup {
                ordinal: active.ordinal,
                stable_key: active.stable_key,
                label: active.label.clone(),
                source_indices,
                x_values: active.x_values.clone(),
                style: active.style,
                identity_values: active.identity_values.clone(),
            });
        }
        Ok(Self {
            policy: self.policy.clone(),
            x_axis: self.x_axis.clone(),
            groups,
        })
    }

    fn fingerprint(&self) -> u64 {
        let mut hash = StableHash::default();
        hash.bytes(self.x_axis.dimension_key.as_bytes());
        hash.bytes(self.x_axis.label.as_bytes());
        hash.bytes(self.x_axis.unit.as_bytes());
        for group in &self.groups {
            hash.bytes(&group.ordinal.to_le_bytes());
            hash.bytes(&group.stable_key.to_le_bytes());
            hash.bytes(group.label.as_bytes());
            for source_index in &group.source_indices {
                hash.bytes(&source_index.to_le_bytes());
            }
            for x in &group.x_values {
                hash.bytes(&x.to_bits().to_le_bytes());
            }
            group.style.hash_into(&mut hash);
        }
        hash.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FamilyXAxis {
    pub dimension_key: String,
    pub label: String,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FamilyRenderGroup {
    /// Exact position in the compiled active plan. Overlay plans preserve
    /// this ordinal after identity/X matching, avoiding hash-only joins.
    pub ordinal: usize,
    pub stable_key: u64,
    pub label: String,
    pub source_indices: Vec<usize>,
    /// Exact numeric X coordinates projected from the immutable family
    /// manifest, in one-to-one order with `source_indices`.
    pub x_values: Vec<f64>,
    pub style: FamilyTraceStyle,
    identity_values: Vec<CanonicalFamilyValue>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct FamilyTraceStyle {
    pub color: Option<FamilyColorStyle>,
    pub dash_ordinal: Option<usize>,
    pub marker_ordinal: Option<usize>,
    pub width_points: Option<f32>,
}

impl FamilyTraceStyle {
    fn hash_into(self, hash: &mut StableHash) {
        if let Some(color) = self.color {
            hash.byte(1);
            hash.byte(palette_tag(color.palette));
            hash.bytes(&color.ordinal.to_le_bytes());
            hash.bytes(&color.category_count.to_le_bytes());
        } else {
            hash.byte(0);
        }
        for ordinal in [self.dash_ordinal, self.marker_ordinal] {
            if let Some(ordinal) = ordinal {
                hash.byte(1);
                hash.bytes(&ordinal.to_le_bytes());
            } else {
                hash.byte(0);
            }
        }
        match self.width_points {
            Some(width) => {
                hash.byte(1);
                hash.bytes(&width.to_bits().to_le_bytes());
            }
            None => hash.byte(0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FamilyColorStyle {
    pub palette: AccessibleColorPalette,
    pub ordinal: usize,
    pub category_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CanonicalFamilyValue {
    Number(u64),
    Integer(u64),
    Text(String),
    Status(FamilyPointStatus),
}

fn exact_numeric_x(
    value: &FamilyValue,
    dimension: &str,
    source_index: usize,
) -> Result<f64, String> {
    match value {
        FamilyValue::Number(value) if value.is_finite() => Ok(*value),
        FamilyValue::Number(_) => Err(format!(
            "family row {source_index} has a non-finite X value for dimension '{dimension}'"
        )),
        FamilyValue::Integer(value) => {
            let converted = *value as f64;
            if converted.is_finite() && converted as u128 == u128::from(*value) {
                Ok(converted)
            } else {
                Err(format!(
                    "family row {source_index} integer X value {value} cannot be represented losslessly as f64"
                ))
            }
        }
        FamilyValue::Text(_) | FamilyValue::Status(_) => Err(format!(
            "family row {source_index} has a non-numeric X value for dimension '{dimension}'"
        )),
    }
}

impl From<&FamilyValue> for CanonicalFamilyValue {
    fn from(value: &FamilyValue) -> Self {
        match value {
            FamilyValue::Number(value) => Self::Number(value.to_bits()),
            FamilyValue::Integer(value) => Self::Integer(*value),
            FamilyValue::Text(value) => Self::Text(value.clone()),
            FamilyValue::Status(value) => Self::Status(*value),
        }
    }
}

impl CanonicalFamilyValue {
    fn to_family_value(&self) -> FamilyValue {
        match self {
            Self::Number(value) => FamilyValue::Number(f64::from_bits(*value)),
            Self::Integer(value) => FamilyValue::Integer(*value),
            Self::Text(value) => FamilyValue::Text(value.clone()),
            Self::Status(value) => FamilyValue::Status(*value),
        }
    }
}

impl Ord for CanonicalFamilyValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => {
                f64::from_bits(*left).total_cmp(&f64::from_bits(*right))
            }
            (Self::Integer(left), Self::Integer(right)) => left.cmp(right),
            (Self::Text(left), Self::Text(right)) => left.cmp(right),
            (Self::Status(left), Self::Status(right)) => left.query_name().cmp(right.query_name()),
            _ => canonical_value_tag(self).cmp(&canonical_value_tag(other)),
        }
    }
}

impl PartialOrd for CanonicalFamilyValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const fn canonical_value_tag(value: &CanonicalFamilyValue) -> u8 {
    match value {
        CanonicalFamilyValue::Number(_) => 0,
        CanonicalFamilyValue::Integer(_) => 1,
        CanonicalFamilyValue::Text(_) => 2,
        CanonicalFamilyValue::Status(_) => 3,
    }
}

fn require_manifest_dimension(
    manifest: &FamilyManifest,
    dimension: &DocumentFamilyDimension,
) -> Result<(), String> {
    let resolved = manifest
        .dimension(&dimension.key)
        .ok_or_else(|| format!("family dimension '{}' is unavailable", dimension.key))?;
    let value_type = match resolved.kind {
        FamilyValueKind::Number => ValueType::Real,
        FamilyValueKind::Integer => ValueType::Integer,
        FamilyValueKind::Text | FamilyValueKind::Status => ValueType::Text,
    };
    if value_type != dimension.value_type {
        return Err(format!(
            "family dimension '{}' changed from {:?} to {:?}",
            dimension.key, dimension.value_type, value_type
        ));
    }
    Ok(())
}

fn interpolate_width(
    value: &FamilyValue,
    categories: &[CanonicalFamilyValue],
    minimum: f32,
    maximum: f32,
) -> Result<f32, String> {
    let ratio = match value {
        FamilyValue::Number(current) => {
            let values = categories.iter().filter_map(|value| match value {
                CanonicalFamilyValue::Number(value) => Some(f64::from_bits(*value)),
                _ => None,
            });
            normalized_real(*current, values)?
        }
        FamilyValue::Integer(current) => {
            let mut values = categories.iter().filter_map(|value| match value {
                CanonicalFamilyValue::Integer(value) => Some(*value),
                _ => None,
            });
            let first = values
                .next()
                .ok_or_else(|| "thickness encoding has no integer family values".to_owned())?;
            let (mut low, mut high) = (first, first);
            for value in values {
                low = low.min(value);
                high = high.max(value);
            }
            if high == low {
                0.5
            } else {
                let numerator = u128::from(current.saturating_sub(low));
                let denominator = u128::from(high - low);
                (numerator as f64 / denominator as f64).clamp(0.0, 1.0)
            }
        }
        _ => return Err("thickness encoding requires a numeric family value".to_owned()),
    } as f32;
    Ok(minimum + ratio * (maximum - minimum))
}

fn normalized_real(current: f64, mut values: impl Iterator<Item = f64>) -> Result<f64, String> {
    let first = values
        .next()
        .ok_or_else(|| "thickness encoding has no real family values".to_owned())?;
    let (mut low, mut high) = (first, first);
    for value in values {
        low = low.min(value);
        high = high.max(value);
    }
    if high == low {
        Ok(0.5)
    } else {
        Ok(((current - low) / (high - low)).clamp(0.0, 1.0))
    }
}

fn display_family_value(value: &FamilyValue, dimension: Option<&FamilyDimension>) -> String {
    let raw = match value {
        FamilyValue::Number(value) => format!("{value:.6}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned(),
        FamilyValue::Integer(value) => value.to_string(),
        FamilyValue::Text(value) => value.clone(),
        FamilyValue::Status(value) => value.query_name().to_owned(),
    };
    match dimension.and_then(|dimension| dimension.unit.as_deref()) {
        Some("°C") => format!("{raw} °C"),
        Some(unit) if !unit.is_empty() => format!("{raw} {unit}"),
        _ => raw,
    }
}

fn stable_group_key(
    dimensions: &[DocumentFamilyDimension],
    values: &[CanonicalFamilyValue],
) -> u64 {
    let mut hash = StableHash::default();
    for (dimension, value) in dimensions.iter().zip(values) {
        hash.bytes(dimension.key.as_bytes());
        hash.byte(canonical_value_tag(value));
        match value {
            CanonicalFamilyValue::Number(value) | CanonicalFamilyValue::Integer(value) => {
                hash.bytes(&value.to_le_bytes());
            }
            CanonicalFamilyValue::Text(value) => hash.bytes(value.as_bytes()),
            CanonicalFamilyValue::Status(value) => hash.bytes(value.query_name().as_bytes()),
        }
    }
    hash.finish()
}

const fn palette_tag(palette: AccessibleColorPalette) -> u8 {
    match palette {
        AccessibleColorPalette::OkabeItoCategorical => 0,
        AccessibleColorPalette::TolBrightCategorical => 1,
        AccessibleColorPalette::CividisSequential => 2,
        AccessibleColorPalette::ViridisSequential => 3,
    }
}

struct StableHash(u64);

impl Default for StableHash {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl StableHash {
    fn byte(&mut self, byte: u8) {
        self.0 ^= u64::from(byte);
        self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
    }

    fn bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.byte(*byte);
        }
        self.byte(0xff);
    }

    const fn finish(self) -> u64 {
        self.0
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

    /// Evaluate the persisted typed filter AST against this exact manifest.
    /// `FamilyFilterExpression::source` is deliberately ignored: it is UI
    /// round-trip text, while the typed predicate is the authoritative
    /// execution contract.
    pub fn matching_source_indices_for_filter(
        &self,
        filter: Option<&FamilyFilterExpression>,
    ) -> Result<Vec<usize>, String> {
        let Some(filter) = filter else {
            return Ok(self.points.iter().map(|point| point.source_index).collect());
        };
        validate_manifest_predicate(self, &filter.predicate)?;
        self.points
            .iter()
            .filter_map(|point| {
                evaluate_manifest_predicate(self, point, &filter.predicate)
                    .map(|matches| matches.then_some(point.source_index))
                    .transpose()
            })
            .collect()
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

fn validate_manifest_predicate(
    manifest: &FamilyManifest,
    predicate: &FamilyPredicate,
) -> Result<(), String> {
    match predicate {
        FamilyPredicate::Constant { .. } => Ok(()),
        FamilyPredicate::Compare {
            dimension, value, ..
        } => {
            require_manifest_dimension(manifest, dimension)?;
            require_typed_filter_value(dimension, value)
        }
        FamilyPredicate::In { dimension, values } => {
            require_manifest_dimension(manifest, dimension)?;
            for value in values {
                require_typed_filter_value(dimension, value)?;
            }
            Ok(())
        }
        FamilyPredicate::Between {
            dimension,
            lower,
            upper,
            ..
        } => {
            require_manifest_dimension(manifest, dimension)?;
            require_typed_filter_value(dimension, lower)?;
            require_typed_filter_value(dimension, upper)
        }
        FamilyPredicate::All { predicates } | FamilyPredicate::Any { predicates } => {
            for predicate in predicates {
                validate_manifest_predicate(manifest, predicate)?;
            }
            Ok(())
        }
        FamilyPredicate::Not { predicate } => validate_manifest_predicate(manifest, predicate),
    }
}

fn require_typed_filter_value(
    dimension: &DocumentFamilyDimension,
    value: &TypedValue,
) -> Result<(), String> {
    if value.value_type() == dimension.value_type {
        Ok(())
    } else {
        Err(format!(
            "family filter value for '{}' has type {:?}, expected {:?}",
            dimension.key,
            value.value_type(),
            dimension.value_type
        ))
    }
}

fn evaluate_manifest_predicate(
    manifest: &FamilyManifest,
    point: &FamilyPoint,
    predicate: &FamilyPredicate,
) -> Result<bool, String> {
    match predicate {
        FamilyPredicate::Constant { value } => Ok(*value),
        FamilyPredicate::Compare {
            dimension,
            operator,
            value,
        } => {
            let candidate = manifest_typed_value(manifest, point, dimension)?;
            compare_typed_values(&candidate, value, *operator)
        }
        FamilyPredicate::In { dimension, values } => {
            let candidate = manifest_typed_value(manifest, point, dimension)?;
            Ok(values
                .iter()
                .any(|value| exact_typed_value_eq(&candidate, value)))
        }
        FamilyPredicate::Between {
            dimension,
            lower,
            upper,
            include_lower,
            include_upper,
        } => {
            let candidate = manifest_typed_value(manifest, point, dimension)?;
            let lower_cmp = typed_value_ordering(&candidate, lower).ok_or_else(|| {
                format!("family filter range '{}' is not comparable", dimension.key)
            })?;
            let upper_cmp = typed_value_ordering(&candidate, upper).ok_or_else(|| {
                format!("family filter range '{}' is not comparable", dimension.key)
            })?;
            Ok(matches!(
                lower_cmp,
                Ordering::Greater | Ordering::Equal if *include_lower || lower_cmp != Ordering::Equal
            ) && matches!(
                upper_cmp,
                Ordering::Less | Ordering::Equal if *include_upper || upper_cmp != Ordering::Equal
            ))
        }
        FamilyPredicate::All { predicates } => {
            for predicate in predicates {
                if !evaluate_manifest_predicate(manifest, point, predicate)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        FamilyPredicate::Any { predicates } => {
            for predicate in predicates {
                if evaluate_manifest_predicate(manifest, point, predicate)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        FamilyPredicate::Not { predicate } => {
            evaluate_manifest_predicate(manifest, point, predicate).map(|value| !value)
        }
    }
}

fn manifest_typed_value(
    manifest: &FamilyManifest,
    point: &FamilyPoint,
    dimension: &DocumentFamilyDimension,
) -> Result<TypedValue, String> {
    require_manifest_dimension(manifest, dimension)?;
    let value = point.values.get(&dimension.key).ok_or_else(|| {
        format!(
            "family row {} is missing filter dimension '{}'",
            point.source_index, dimension.key
        )
    })?;
    match value {
        FamilyValue::Number(value) if value.is_finite() => Ok(TypedValue::Real(*value)),
        FamilyValue::Number(_) => Err(format!(
            "family row {} has non-finite filter value '{}'",
            point.source_index, dimension.key
        )),
        FamilyValue::Integer(value) => {
            i64::try_from(*value).map(TypedValue::Integer).map_err(|_| {
                format!(
                    "family row {} integer filter value for '{}' exceeds i64",
                    point.source_index, dimension.key
                )
            })
        }
        FamilyValue::Text(value) => Ok(TypedValue::Text(value.clone())),
        FamilyValue::Status(value) => Ok(TypedValue::Text(value.query_name().to_owned())),
    }
}

fn exact_typed_value_eq(left: &TypedValue, right: &TypedValue) -> bool {
    match (left, right) {
        (TypedValue::Real(left), TypedValue::Real(right)) => left.to_bits() == right.to_bits(),
        (TypedValue::Integer(left), TypedValue::Integer(right)) => left == right,
        (TypedValue::Boolean(left), TypedValue::Boolean(right)) => left == right,
        (TypedValue::Text(left), TypedValue::Text(right)) => left == right,
        _ => false,
    }
}

fn typed_value_ordering(left: &TypedValue, right: &TypedValue) -> Option<Ordering> {
    match (left, right) {
        (TypedValue::Real(left), TypedValue::Real(right)) => left.partial_cmp(right),
        (TypedValue::Integer(left), TypedValue::Integer(right)) => Some(left.cmp(right)),
        (TypedValue::Boolean(left), TypedValue::Boolean(right)) => Some(left.cmp(right)),
        (TypedValue::Text(left), TypedValue::Text(right)) => Some(left.cmp(right)),
        _ => None,
    }
}

fn compare_typed_values(
    left: &TypedValue,
    right: &TypedValue,
    operator: FamilyComparisonOperator,
) -> Result<bool, String> {
    match operator {
        FamilyComparisonOperator::Equal => Ok(exact_typed_value_eq(left, right)),
        FamilyComparisonOperator::NotEqual => Ok(!exact_typed_value_eq(left, right)),
        FamilyComparisonOperator::Contains
        | FamilyComparisonOperator::StartsWith
        | FamilyComparisonOperator::EndsWith => {
            let (TypedValue::Text(left), TypedValue::Text(right)) = (left, right) else {
                return Err("text family filter operator received non-text values".to_owned());
            };
            Ok(match operator {
                FamilyComparisonOperator::Contains => left.contains(right),
                FamilyComparisonOperator::StartsWith => left.starts_with(right),
                FamilyComparisonOperator::EndsWith => left.ends_with(right),
                _ => unreachable!(),
            })
        }
        FamilyComparisonOperator::LessThan
        | FamilyComparisonOperator::LessThanOrEqual
        | FamilyComparisonOperator::GreaterThan
        | FamilyComparisonOperator::GreaterThanOrEqual => {
            let ordering = typed_value_ordering(left, right)
                .ok_or_else(|| "family filter values are not comparable".to_owned())?;
            Ok(match operator {
                FamilyComparisonOperator::LessThan => ordering == Ordering::Less,
                FamilyComparisonOperator::LessThanOrEqual => ordering != Ordering::Greater,
                FamilyComparisonOperator::GreaterThan => ordering == Ordering::Greater,
                FamilyComparisonOperator::GreaterThanOrEqual => ordering != Ordering::Less,
                _ => unreachable!(),
            })
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
    use crate::results::visualization_document::{
        FamilyAggregationPolicy, FamilyDimension as DocumentFamilyDimension, FamilyEncodingMap,
        FamilyXDimension, MissingPointPolicy,
    };
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

    fn process_policy() -> FamilyPresentationPolicy {
        let process = DocumentFamilyDimension::new("process", ValueType::Text).unwrap();
        FamilyPresentationPolicy {
            x_dimension: FamilyXDimension {
                dimension: DocumentFamilyDimension::new("RGAIN", ValueType::Real).unwrap(),
                ordering: FamilyXOrdering::Source,
            },
            family_dimensions: vec![process.clone()],
            facet_layout: None,
            aggregation: FamilyAggregationPolicy {
                method: FamilyAggregationMethod::None,
                over_dimensions: Vec::new(),
            },
            filter: None,
            missing_points: MissingPointPolicy::ExcludeWithOmissionRecord,
            encodings: vec![
                FamilyEncodingMap::Color {
                    dimension: process.clone(),
                    palette: AccessibleColorPalette::OkabeItoCategorical,
                },
                FamilyEncodingMap::Dash { dimension: process },
            ],
        }
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

    #[test]
    fn family_styles_are_deterministic_across_filter_subsets() {
        let manifest = FamilyManifest::from_analysis(&corner_result())
            .unwrap()
            .unwrap();
        let policy = process_policy();
        let dataset = DatasetId::new();
        let full = SourceSampleSelection::new(dataset, 7, vec![0, 1, 2])
            .unwrap()
            .with_family_presentation(&manifest, &policy)
            .unwrap();
        let tt_only = SourceSampleSelection::new(dataset, 7, vec![1])
            .unwrap()
            .with_family_presentation(&manifest, &policy)
            .unwrap();

        let full_tt = full
            .family_render_plan()
            .unwrap()
            .groups()
            .iter()
            .find(|group| group.label.contains("TT"))
            .unwrap();
        let filtered_tt = &tt_only.family_render_plan().unwrap().groups()[0];
        assert_eq!(full_tt.style, filtered_tt.style);
        assert_eq!(full_tt.stable_key, filtered_tt.stable_key);
        assert_eq!(full_tt.source_indices, [1]);
        assert_eq!(full.fingerprint(), full.clone().fingerprint());
    }

    #[test]
    fn unsupported_family_flattening_fails_closed() {
        let manifest = FamilyManifest::from_analysis(&corner_result())
            .unwrap()
            .unwrap();
        let mut policy = process_policy();
        policy.x_dimension.ordering = FamilyXOrdering::Descending;
        let error = SourceSampleSelection::new(DatasetId::new(), 7, vec![0, 1, 2])
            .unwrap()
            .with_family_presentation(&manifest, &policy)
            .unwrap_err();
        assert!(error.contains("exact source ordering"));
    }

    #[test]
    fn typed_filter_ast_is_authoritative_and_covers_every_predicate_form() {
        let manifest = FamilyManifest::from_analysis(&corner_result())
            .unwrap()
            .unwrap();
        let process = DocumentFamilyDimension::new("process", ValueType::Text).unwrap();
        let x = DocumentFamilyDimension::new("RGAIN", ValueType::Real).unwrap();
        let temperature = DocumentFamilyDimension::new("temperature", ValueType::Real).unwrap();
        let status = DocumentFamilyDimension::new("status", ValueType::Text).unwrap();
        let filter = FamilyFilterExpression {
            source: "this source text is intentionally not executable".to_owned(),
            predicate: FamilyPredicate::All {
                predicates: vec![
                    FamilyPredicate::Constant { value: true },
                    FamilyPredicate::In {
                        dimension: process,
                        values: vec![
                            TypedValue::Text("TT".to_owned()),
                            TypedValue::Text("FF".to_owned()),
                        ],
                    },
                    FamilyPredicate::Between {
                        dimension: x,
                        lower: TypedValue::Real(2.0),
                        upper: TypedValue::Real(3.0),
                        include_lower: true,
                        include_upper: true,
                    },
                    FamilyPredicate::Any {
                        predicates: vec![
                            FamilyPredicate::Compare {
                                dimension: temperature,
                                operator: FamilyComparisonOperator::GreaterThanOrEqual,
                                value: TypedValue::Real(27.0),
                            },
                            FamilyPredicate::Constant { value: false },
                        ],
                    },
                    FamilyPredicate::Not {
                        predicate: Box::new(FamilyPredicate::Compare {
                            dimension: status,
                            operator: FamilyComparisonOperator::Equal,
                            value: TypedValue::Text("not-run".to_owned()),
                        }),
                    },
                ],
            },
        };

        assert_eq!(
            manifest
                .matching_source_indices_for_filter(Some(&filter))
                .unwrap(),
            [1, 2]
        );
    }

    #[test]
    fn family_x_requires_finite_lossless_numeric_projection() {
        let group = DocumentFamilyDimension::new("group", ValueType::Text).unwrap();
        let policy = |x_type| FamilyPresentationPolicy {
            x_dimension: FamilyXDimension {
                dimension: DocumentFamilyDimension::new("x", x_type).unwrap(),
                ordering: FamilyXOrdering::Source,
            },
            family_dimensions: vec![group.clone()],
            facet_layout: None,
            aggregation: FamilyAggregationPolicy {
                method: FamilyAggregationMethod::None,
                over_dimensions: Vec::new(),
            },
            filter: None,
            missing_points: MissingPointPolicy::ExcludeWithOmissionRecord,
            encodings: vec![FamilyEncodingMap::Label {
                dimension: group.clone(),
                prefix: None,
            }],
        };
        let manifest = |kind, value| FamilyManifest {
            dimensions: vec![
                dimension("x", "Exact X", Some("u"), kind),
                dimension("group", "Group", None, FamilyValueKind::Text),
            ],
            points: vec![point(
                0,
                [("x", value), ("group", FamilyValue::Text("A".to_owned()))],
            )],
            omitted_points: 0,
        };

        let non_finite = manifest(FamilyValueKind::Number, FamilyValue::Number(f64::NAN));
        let error = SourceSampleSelection::new(DatasetId::new(), 1, vec![0])
            .unwrap()
            .with_family_presentation(&non_finite, &policy(ValueType::Real))
            .unwrap_err();
        assert!(error.contains("non-finite X value"));

        let lossy = manifest(
            FamilyValueKind::Integer,
            FamilyValue::Integer(9_007_199_254_740_993),
        );
        let error = SourceSampleSelection::new(DatasetId::new(), 1, vec![0])
            .unwrap()
            .with_family_presentation(&lossy, &policy(ValueType::Integer))
            .unwrap_err();
        assert!(error.contains("cannot be represented losslessly"));
    }
}
