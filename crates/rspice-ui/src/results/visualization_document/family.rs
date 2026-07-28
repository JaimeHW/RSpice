//! Trace families: how one swept dimension becomes many curves.
//!
//! A family binds curves to the dimensions that generated them, and the
//! binding is persisted with its value *type* — so a source-schema change
//! surfaces as a validation error rather than silently reinterpreting a saved
//! plot.  The presentation policy decides encoding, faceting, and overflow,
//! and overflow is always declared rather than curves being dropped unseen.

use super::*;

/// Stable, explicitly typed reference to an immutable result dimension.
///
/// The value type is persisted with the policy so a source-schema change is a
/// validation error rather than an implicit reinterpretation of a saved plot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FamilyDimension {
    #[serde(deserialize_with = "deserialize_key_string")]
    pub key: String,
    pub value_type: ValueType,
}

impl FamilyDimension {
    pub fn new(key: impl Into<String>, value_type: ValueType) -> Result<Self, VisualizationError> {
        let dimension = Self {
            key: key.into(),
            value_type,
        };
        dimension.validate()?;
        Ok(dimension)
    }

    fn validate(&self) -> Result<(), VisualizationError> {
        validate_key("family.dimension.key", &self.key)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FamilyXOrdering {
    /// Preserve the exact immutable source ordering.
    #[default]
    Source,
    Ascending,
    Descending,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyXDimension {
    pub dimension: FamilyDimension,
    #[serde(default)]
    pub ordering: FamilyXOrdering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum FamilyAggregationMethod {
    None,
    Mean,
    Median,
    Minimum,
    Maximum,
    RootMeanSquare,
    Envelope,
    Percentile { percentile_basis_points: u16 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyAggregationPolicy {
    pub method: FamilyAggregationMethod,
    /// Family dimensions eliminated by the aggregation.
    #[serde(deserialize_with = "deserialize_family_dimensions")]
    pub over_dimensions: Vec<FamilyDimension>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FamilyComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
}

/// Typed, serializable filter predicate. The AST is authoritative; `source`
/// on [`FamilyFilterExpression`] is retained for exact UI round-tripping and
/// review receipts.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum FamilyPredicate {
    Constant {
        value: bool,
    },
    Compare {
        dimension: FamilyDimension,
        operator: FamilyComparisonOperator,
        value: TypedValue,
    },
    In {
        dimension: FamilyDimension,
        values: Vec<TypedValue>,
    },
    Between {
        dimension: FamilyDimension,
        lower: TypedValue,
        upper: TypedValue,
        include_lower: bool,
        include_upper: bool,
    },
    All {
        predicates: Vec<FamilyPredicate>,
    },
    Any {
        predicates: Vec<FamilyPredicate>,
    },
    Not {
        predicate: Box<FamilyPredicate>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub(super) enum FamilyPredicateWire {
    Constant {
        value: bool,
    },
    Compare {
        dimension: FamilyDimension,
        operator: FamilyComparisonOperator,
        value: TypedValue,
    },
    In {
        dimension: FamilyDimension,
        #[serde(deserialize_with = "deserialize_family_predicate_values")]
        values: Vec<TypedValue>,
    },
    Between {
        dimension: FamilyDimension,
        lower: TypedValue,
        upper: TypedValue,
        include_lower: bool,
        include_upper: bool,
    },
    All {
        #[serde(deserialize_with = "deserialize_family_predicate_children")]
        predicates: Vec<FamilyPredicate>,
    },
    Any {
        #[serde(deserialize_with = "deserialize_family_predicate_children")]
        predicates: Vec<FamilyPredicate>,
    },
    Not {
        predicate: Box<FamilyPredicate>,
    },
}

#[derive(Default)]
pub(super) struct PredicateDecodeBudget {
    depth: usize,
    nodes: usize,
}

std::thread_local! {
    static PREDICATE_DECODE_BUDGET: RefCell<Option<PredicateDecodeBudget>> = const { RefCell::new(None) };
}

pub(super) struct PredicateDecodeGuard {
    root: bool,
}

impl PredicateDecodeGuard {
    fn enter() -> Result<Self, String> {
        PREDICATE_DECODE_BUDGET.with(|slot| {
            let mut slot = slot.borrow_mut();
            let root = slot.is_none();
            let budget = slot.get_or_insert_with(PredicateDecodeBudget::default);
            if budget.depth >= MAX_FAMILY_PREDICATE_DEPTH {
                return Err(format!(
                    "family predicate exceeds the {MAX_FAMILY_PREDICATE_DEPTH}-level decode depth limit"
                ));
            }
            if budget.nodes >= MAX_FAMILY_PREDICATE_NODES {
                return Err(format!(
                    "family predicate exceeds the {MAX_FAMILY_PREDICATE_NODES}-node decode limit"
                ));
            }
            budget.depth += 1;
            budget.nodes += 1;
            Ok(Self { root })
        })
    }
}

impl Drop for PredicateDecodeGuard {
    fn drop(&mut self) {
        PREDICATE_DECODE_BUDGET.with(|slot| {
            let mut slot = slot.borrow_mut();
            if self.root {
                *slot = None;
            } else if let Some(budget) = slot.as_mut() {
                budget.depth = budget.depth.saturating_sub(1);
            }
        });
    }
}

impl<'de> Deserialize<'de> for FamilyPredicate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _guard = PredicateDecodeGuard::enter().map_err(serde::de::Error::custom)?;
        Ok(match FamilyPredicateWire::deserialize(deserializer)? {
            FamilyPredicateWire::Constant { value } => Self::Constant { value },
            FamilyPredicateWire::Compare {
                dimension,
                operator,
                value,
            } => Self::Compare {
                dimension,
                operator,
                value,
            },
            FamilyPredicateWire::In { dimension, values } => Self::In { dimension, values },
            FamilyPredicateWire::Between {
                dimension,
                lower,
                upper,
                include_lower,
                include_upper,
            } => Self::Between {
                dimension,
                lower,
                upper,
                include_lower,
                include_upper,
            },
            FamilyPredicateWire::All { predicates } => Self::All { predicates },
            FamilyPredicateWire::Any { predicates } => Self::Any { predicates },
            FamilyPredicateWire::Not { predicate } => Self::Not { predicate },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FamilyFilterExpression {
    /// User-authored expression preserved exactly for display and audit.
    #[serde(deserialize_with = "deserialize_filter_source_string")]
    pub source: String,
    /// Parsed, typed predicate used for evaluation.
    pub predicate: FamilyPredicate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MissingPointPolicy {
    PreserveAsNotRun,
    ExcludeWithOmissionRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccessibleColorPalette {
    OkabeItoCategorical,
    TolBrightCategorical,
    CividisSequential,
    ViridisSequential,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FacetDirection {
    Rows,
    Columns,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FacetAxisSharing {
    Shared,
    IndependentVertical,
    Independent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FacetOverflowPolicy {
    Reject,
    Paginate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyFacetLayout {
    pub axis_sharing: FacetAxisSharing,
    pub overflow: FacetOverflowPolicy,
    pub maximum_panels: u16,
}

/// One deterministic mapping from a declared family dimension to a visual
/// channel. Color palettes are restricted to accessibility-reviewed choices,
/// and policy validation requires a redundant non-color cue for every color
/// mapping.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "channel")]
pub enum FamilyEncodingMap {
    Color {
        dimension: FamilyDimension,
        palette: AccessibleColorPalette,
    },
    Dash {
        dimension: FamilyDimension,
    },
    Marker {
        dimension: FamilyDimension,
    },
    Thickness {
        dimension: FamilyDimension,
        minimum_points: f32,
        maximum_points: f32,
    },
    Facet {
        dimension: FamilyDimension,
        direction: FacetDirection,
    },
    Label {
        dimension: FamilyDimension,
        #[serde(deserialize_with = "deserialize_label_prefix")]
        prefix: Option<String>,
    },
}

impl FamilyEncodingMap {
    #[must_use]
    pub const fn dimension(&self) -> &FamilyDimension {
        match self {
            Self::Color { dimension, .. }
            | Self::Dash { dimension }
            | Self::Marker { dimension }
            | Self::Thickness { dimension, .. }
            | Self::Facet { dimension, .. }
            | Self::Label { dimension, .. } => dimension,
        }
    }
}

/// Complete, immutable-data-aware presentation policy for one pane's result
/// family. Policies are optional on panes; once present, every referenced
/// dimension must resolve with the declared type in every bound source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FamilyPresentationPolicy {
    pub x_dimension: FamilyXDimension,
    #[serde(deserialize_with = "deserialize_family_dimensions")]
    pub family_dimensions: Vec<FamilyDimension>,
    pub facet_layout: Option<FamilyFacetLayout>,
    pub aggregation: FamilyAggregationPolicy,
    pub filter: Option<FamilyFilterExpression>,
    pub missing_points: MissingPointPolicy,
    #[serde(deserialize_with = "deserialize_family_encodings")]
    pub encodings: Vec<FamilyEncodingMap>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum FamilyEncodingSlot {
    Color,
    Dash,
    Marker,
    Thickness,
    Facet(FacetDirection),
    Label,
}

impl FamilyPresentationPolicy {
    /// Validates policy structure independently of a document. Document
    /// transactions additionally resolve every dimension against immutable
    /// pane sources before committing.
    pub fn validate(&self) -> Result<(), VisualizationError> {
        self.x_dimension.dimension.validate()?;
        if !matches!(
            self.x_dimension.dimension.value_type,
            ValueType::Real | ValueType::Integer
        ) {
            return Err(VisualizationError::InvalidValue {
                field: "family.x-dimension",
                message: "the X dimension must be real or integer valued".to_owned(),
            });
        }
        if self.family_dimensions.is_empty() || self.family_dimensions.len() > MAX_FAMILY_DIMENSIONS
        {
            return Err(VisualizationError::InvalidValue {
                field: "family.dimensions",
                message: format!(
                    "a family policy requires between 1 and {MAX_FAMILY_DIMENSIONS} dimensions"
                ),
            });
        }
        let mut dimension_keys = HashSet::with_capacity(self.family_dimensions.len());
        for dimension in &self.family_dimensions {
            dimension.validate()?;
            if dimension.key == self.x_dimension.dimension.key {
                return Err(VisualizationError::InvalidValue {
                    field: "family.dimensions",
                    message: "the X dimension cannot also be a family dimension".to_owned(),
                });
            }
            if !dimension_keys.insert(dimension.key.as_str()) {
                return Err(VisualizationError::InvalidValue {
                    field: "family.dimensions",
                    message: "family dimension keys must be unique".to_owned(),
                });
            }
        }

        let aggregated = self.validate_aggregation()?;
        if let Some(filter) = &self.filter {
            self.validate_filter(filter)?;
        }
        self.validate_encodings(&aggregated)
    }

    fn validate_aggregation(&self) -> Result<HashSet<&str>, VisualizationError> {
        ensure_maximum_len(
            "family.aggregation.dimensions",
            self.aggregation.over_dimensions.len(),
            MAX_FAMILY_DIMENSIONS,
        )?;
        let is_none = self.aggregation.method == FamilyAggregationMethod::None;
        if is_none != self.aggregation.over_dimensions.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "family.aggregation.dimensions",
                message: "None aggregation must have no dimensions and all other methods require at least one"
                    .to_owned(),
            });
        }
        if let FamilyAggregationMethod::Percentile {
            percentile_basis_points,
        } = self.aggregation.method
            && !(1..10_000).contains(&percentile_basis_points)
        {
            return Err(VisualizationError::InvalidValue {
                field: "family.aggregation.percentile",
                message: "percentile must be strictly between 0 and 100 percent".to_owned(),
            });
        }
        let mut aggregated = HashSet::with_capacity(self.aggregation.over_dimensions.len());
        for dimension in &self.aggregation.over_dimensions {
            self.require_declared_family_dimension(dimension, "family.aggregation.dimensions")?;
            if !aggregated.insert(dimension.key.as_str()) {
                return Err(VisualizationError::InvalidValue {
                    field: "family.aggregation.dimensions",
                    message: "aggregation dimensions must be unique".to_owned(),
                });
            }
        }
        Ok(aggregated)
    }

    fn validate_filter(&self, filter: &FamilyFilterExpression) -> Result<(), VisualizationError> {
        if filter.source.trim().is_empty()
            || filter.source.chars().any(char::is_control)
            || filter.source.len() > 4096
        {
            return Err(VisualizationError::InvalidValue {
                field: "family.filter.source",
                message: "filter source must be non-blank, contain no control characters, and be at most 4096 bytes"
                    .to_owned(),
            });
        }
        let mut nodes = 0_u16;
        self.validate_predicate(&filter.predicate, 0, &mut nodes)
    }

    fn validate_predicate(
        &self,
        predicate: &FamilyPredicate,
        depth: u8,
        nodes: &mut u16,
    ) -> Result<(), VisualizationError> {
        if usize::from(depth) >= MAX_FAMILY_PREDICATE_DEPTH {
            return Err(VisualizationError::InvalidValue {
                field: "family.filter.predicate",
                message: format!(
                    "filter predicate nesting cannot exceed {MAX_FAMILY_PREDICATE_DEPTH} levels"
                ),
            });
        }
        *nodes = nodes
            .checked_add(1)
            .ok_or_else(|| VisualizationError::InvalidValue {
                field: "family.filter.predicate",
                message: "filter predicate is too large".to_owned(),
            })?;
        if usize::from(*nodes) > MAX_FAMILY_PREDICATE_NODES {
            return Err(VisualizationError::InvalidValue {
                field: "family.filter.predicate",
                message: format!(
                    "filter predicate cannot exceed {MAX_FAMILY_PREDICATE_NODES} nodes"
                ),
            });
        }
        match predicate {
            FamilyPredicate::Constant { .. } => Ok(()),
            FamilyPredicate::Compare {
                dimension,
                operator,
                value,
            } => {
                self.require_declared_dimension(dimension, "family.filter.dimension")?;
                validate_filter_value(dimension, value)?;
                match operator {
                    FamilyComparisonOperator::Equal | FamilyComparisonOperator::NotEqual => Ok(()),
                    FamilyComparisonOperator::LessThan
                    | FamilyComparisonOperator::LessThanOrEqual
                    | FamilyComparisonOperator::GreaterThan
                    | FamilyComparisonOperator::GreaterThanOrEqual
                        if matches!(dimension.value_type, ValueType::Real | ValueType::Integer) =>
                    {
                        Ok(())
                    }
                    FamilyComparisonOperator::Contains
                    | FamilyComparisonOperator::StartsWith
                    | FamilyComparisonOperator::EndsWith
                        if dimension.value_type == ValueType::Text =>
                    {
                        Ok(())
                    }
                    _ => Err(VisualizationError::InvalidValue {
                        field: "family.filter.operator",
                        message: "comparison operator is incompatible with the dimension type"
                            .to_owned(),
                    }),
                }
            }
            FamilyPredicate::In { dimension, values } => {
                self.require_declared_dimension(dimension, "family.filter.dimension")?;
                if values.is_empty() || values.len() > MAX_FAMILY_PREDICATE_VALUES {
                    return Err(VisualizationError::InvalidValue {
                        field: "family.filter.values",
                        message: format!(
                            "set membership requires between 1 and {MAX_FAMILY_PREDICATE_VALUES} values"
                        ),
                    });
                }
                for (index, value) in values.iter().enumerate() {
                    validate_filter_value(dimension, value)?;
                    if values[..index].iter().any(|prior| prior.exact_eq(value)) {
                        return Err(VisualizationError::InvalidValue {
                            field: "family.filter.values",
                            message: "set membership values must be exact and unique".to_owned(),
                        });
                    }
                }
                Ok(())
            }
            FamilyPredicate::Between {
                dimension,
                lower,
                upper,
                include_lower,
                include_upper,
            } => {
                self.require_declared_dimension(dimension, "family.filter.dimension")?;
                validate_filter_value(dimension, lower)?;
                validate_filter_value(dimension, upper)?;
                let ordering = match (lower, upper) {
                    (TypedValue::Real(lower), TypedValue::Real(upper)) => lower.partial_cmp(upper),
                    (TypedValue::Integer(lower), TypedValue::Integer(upper)) => {
                        Some(lower.cmp(upper))
                    }
                    _ => None,
                };
                match ordering {
                    Some(std::cmp::Ordering::Less) => Ok(()),
                    Some(std::cmp::Ordering::Equal) if *include_lower && *include_upper => Ok(()),
                    _ => Err(VisualizationError::InvalidValue {
                        field: "family.filter.range",
                        message: "range bounds must be numeric, ordered, and non-empty".to_owned(),
                    }),
                }
            }
            FamilyPredicate::All { predicates } | FamilyPredicate::Any { predicates } => {
                if predicates.is_empty() || predicates.len() > MAX_FAMILY_PREDICATE_CHILDREN {
                    return Err(VisualizationError::InvalidValue {
                        field: "family.filter.predicate",
                        message: format!(
                            "logical groups require between 1 and {MAX_FAMILY_PREDICATE_CHILDREN} predicates"
                        ),
                    });
                }
                for child in predicates {
                    self.validate_predicate(child, depth + 1, nodes)?;
                }
                Ok(())
            }
            FamilyPredicate::Not { predicate } => {
                self.validate_predicate(predicate, depth + 1, nodes)
            }
        }
    }

    fn validate_encodings(&self, aggregated: &HashSet<&str>) -> Result<(), VisualizationError> {
        if self.encodings.len() > MAX_FAMILY_ENCODINGS {
            return Err(VisualizationError::InvalidValue {
                field: "family.encodings",
                message: format!(
                    "a family policy supports at most {MAX_FAMILY_ENCODINGS} encoding maps"
                ),
            });
        }
        let mut slots = HashSet::with_capacity(self.encodings.len());
        for encoding in &self.encodings {
            let dimension = encoding.dimension();
            self.require_declared_family_dimension(dimension, "family.encoding.dimension")?;
            if aggregated.contains(dimension.key.as_str()) {
                return Err(VisualizationError::InvalidValue {
                    field: "family.encoding.dimension",
                    message: "an aggregated dimension cannot also drive a visual encoding"
                        .to_owned(),
                });
            }
            let slot = match encoding {
                FamilyEncodingMap::Color { .. } => FamilyEncodingSlot::Color,
                FamilyEncodingMap::Dash { .. } => FamilyEncodingSlot::Dash,
                FamilyEncodingMap::Marker { .. } => FamilyEncodingSlot::Marker,
                FamilyEncodingMap::Thickness {
                    minimum_points,
                    maximum_points,
                    ..
                } => {
                    if !matches!(dimension.value_type, ValueType::Real | ValueType::Integer)
                        || !minimum_points.is_finite()
                        || !maximum_points.is_finite()
                        || !(0.5..=12.0).contains(minimum_points)
                        || !(0.5..=12.0).contains(maximum_points)
                        || minimum_points >= maximum_points
                    {
                        return Err(VisualizationError::InvalidValue {
                            field: "family.encoding.thickness",
                            message: "thickness requires a numeric dimension and finite increasing widths from 0.5 to 12 points"
                                .to_owned(),
                        });
                    }
                    FamilyEncodingSlot::Thickness
                }
                FamilyEncodingMap::Facet { direction, .. } => FamilyEncodingSlot::Facet(*direction),
                FamilyEncodingMap::Label { prefix, .. } => {
                    if prefix.as_ref().is_some_and(|prefix| {
                        prefix.trim().is_empty()
                            || prefix.chars().any(char::is_control)
                            || prefix.len() > 64
                    }) {
                        return Err(VisualizationError::InvalidValue {
                            field: "family.encoding.label-prefix",
                            message:
                                "label prefix must be absent or non-blank and at most 64 bytes"
                                    .to_owned(),
                        });
                    }
                    FamilyEncodingSlot::Label
                }
            };
            if !slots.insert(slot) {
                return Err(VisualizationError::InvalidValue {
                    field: "family.encodings",
                    message: "each visual channel may be mapped at most once".to_owned(),
                });
            }
        }

        let facet_count = self
            .encodings
            .iter()
            .filter(|encoding| matches!(encoding, FamilyEncodingMap::Facet { .. }))
            .count();
        match (facet_count, self.facet_layout) {
            (0, None) => {}
            (0, Some(_)) => {
                return Err(VisualizationError::InvalidValue {
                    field: "family.facet-layout",
                    message: "facet layout requires a facet encoding".to_owned(),
                });
            }
            (_, None) => {
                return Err(VisualizationError::InvalidValue {
                    field: "family.facet-layout",
                    message: "facet encodings require an explicit facet layout".to_owned(),
                });
            }
            (_, Some(layout)) if !(1..=256).contains(&layout.maximum_panels) => {
                return Err(VisualizationError::InvalidValue {
                    field: "family.facet-layout.maximum-panels",
                    message: "maximum facet panels must be between 1 and 256".to_owned(),
                });
            }
            _ => {}
        }

        for dimension in &self.family_dimensions {
            if aggregated.contains(dimension.key.as_str()) {
                continue;
            }
            if !self
                .encodings
                .iter()
                .any(|encoding| encoding.dimension() == dimension)
            {
                return Err(VisualizationError::InvalidValue {
                    field: "family.encodings",
                    message: format!(
                        "retained family dimension '{}' requires a visual encoding",
                        dimension.key
                    ),
                });
            }
        }

        for color_dimension in self.encodings.iter().filter_map(|encoding| match encoding {
            FamilyEncodingMap::Color { dimension, .. } => Some(dimension),
            _ => None,
        }) {
            let has_redundant_cue = self.encodings.iter().any(|encoding| {
                encoding.dimension() == color_dimension
                    && matches!(
                        encoding,
                        FamilyEncodingMap::Dash { .. }
                            | FamilyEncodingMap::Marker { .. }
                            | FamilyEncodingMap::Facet { .. }
                            | FamilyEncodingMap::Label { .. }
                    )
            });
            if !has_redundant_cue {
                return Err(VisualizationError::InvalidValue {
                    field: "family.encodings.accessibility",
                    message: format!(
                        "color mapping for '{}' requires a redundant dash, marker, facet, or label cue",
                        color_dimension.key
                    ),
                });
            }
        }
        Ok(())
    }

    fn require_declared_dimension(
        &self,
        dimension: &FamilyDimension,
        field: &'static str,
    ) -> Result<(), VisualizationError> {
        if dimension == &self.x_dimension.dimension || self.family_dimensions.contains(dimension) {
            Ok(())
        } else {
            Err(VisualizationError::InvalidValue {
                field,
                message: format!(
                    "dimension '{}' is not declared with the same value type",
                    dimension.key
                ),
            })
        }
    }

    fn require_declared_family_dimension(
        &self,
        dimension: &FamilyDimension,
        field: &'static str,
    ) -> Result<(), VisualizationError> {
        if self.family_dimensions.contains(dimension) {
            Ok(())
        } else {
            Err(VisualizationError::InvalidValue {
                field,
                message: format!(
                    "dimension '{}' is not a declared family dimension with the same value type",
                    dimension.key
                ),
            })
        }
    }
}

pub(super) fn validate_filter_value(
    dimension: &FamilyDimension,
    value: &TypedValue,
) -> Result<(), VisualizationError> {
    value.validate("family.filter.value")?;
    if value.value_type() != dimension.value_type {
        return Err(VisualizationError::InvalidValue {
            field: "family.filter.value",
            message: format!(
                "filter value type {:?} does not match dimension '{}' type {:?}",
                value.value_type(),
                dimension.key,
                dimension.value_type
            ),
        });
    }
    Ok(())
}
