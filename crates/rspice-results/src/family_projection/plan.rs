//! Source-bound sample selection, family grouping, styling and overlay correspondence.

use super::{
    FamilyDimension, FamilyManifest, FamilyPointStatus, FamilyValue, FamilyValueKind,
    require_manifest_dimension,
};
use crate::analysis_type::AnalysisType;
use crate::family_metadata::AnalysisResultFamilyMetadata;
use crate::visualization_document::{
    AccessibleColorPalette, FamilyAggregationMethod, FamilyDimension as DocumentFamilyDimension,
    FamilyEncodingMap, FamilyPresentationPolicy, FamilyXOrdering,
};
use rspice_app_types::product::DatasetId;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};

/// Presentation-only selection of exact source sample rows for one immutable
/// analysis. The dataset and analysis identities prevent a selection from
/// being applied to a later run merely because its arrays have the same size.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceSampleSelection {
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
        analysis_type: AnalysisType,
        metadata: Option<&AnalysisResultFamilyMetadata>,
    ) -> Result<Option<FamilyRenderPlan>, String> {
        let Some(active_plan) = &self.family_render_plan else {
            return Ok(None);
        };
        let manifest = FamilyManifest::from_metadata(analysis_type, metadata)?
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
pub struct FamilyRenderPlan {
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
pub struct FamilyXAxis {
    pub dimension_key: String,
    pub label: String,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FamilyRenderGroup {
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
pub struct FamilyTraceStyle {
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
pub struct FamilyColorStyle {
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
