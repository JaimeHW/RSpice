//! Design intent: the specs a result is judged against and the variables that
//! parameterise it.
//!
//! A spec pins a `.MEAS` result as a tracked row whether or not it carries
//! bounds, so removing a limit never silently drops the measurement.  A design
//! variable declares its quantity, scope, and override policy explicitly,
//! because the same name can be legitimately overridden at a narrower scope
//! and the policy is what decides whether that is allowed.

use super::*;

/// Which of a run's PVT points a specification is a claim about.
///
/// A bound is only ever evidence for the points it was judged against, so the
/// scope is part of the requirement rather than a view filter over it. A
/// narrowed scope admits nothing that the executor could not attribute to a
/// point: an unattributed measurement is not proof about a corner, and
/// answering with it would make the limit pass on a fiction.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SpecPointScope {
    /// Every point the run retained, attributed or not. The verdict is the
    /// worst of them, which is what a specification with no stated scope has
    /// always meant.
    #[default]
    AllPoints,
    /// The run's own reference point only.
    Nominal,
    /// Named process corners only, matched case-insensitively against the
    /// corner name the run set and the PDK section share.
    SelectedCorners { corners: Vec<String> },
}

impl SpecPointScope {
    /// Whether evidence attributed to `point` answers this scope.
    ///
    /// One owner for the rule, so the evaluator, the coverage text and the
    /// registry's standing check cannot disagree about what is in scope.
    #[must_use]
    pub fn admits(&self, point: Option<&AnalysisResultPvtPoint>) -> bool {
        match self {
            Self::AllPoints => true,
            Self::Nominal => point.is_some_and(AnalysisResultPvtPoint::is_nominal),
            Self::SelectedCorners { corners } => point.is_some_and(|point| {
                corners
                    .iter()
                    .any(|corner| corner.eq_ignore_ascii_case(point.process()))
            }),
        }
    }

    /// The corners this scope names, empty for every other case.
    #[must_use]
    pub fn named_corners(&self) -> &[String] {
        match self {
            Self::AllPoints | Self::Nominal => &[],
            Self::SelectedCorners { corners } => corners,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let Self::SelectedCorners { corners } = self else {
            return Ok(());
        };
        if corners.is_empty() {
            return Err(
                "a specification scoped to selected corners must name at least one".to_owned(),
            );
        }
        let mut seen = HashSet::with_capacity(corners.len());
        for corner in corners {
            validate_bounded_text("corner", corner, 64, false)?;
            if !seen.insert(corner.to_ascii_uppercase()) {
                return Err(format!("specification scope repeats corner {corner}"));
            }
        }
        Ok(())
    }
}

/// Release significance of a specification result.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecificationRole {
    #[default]
    Blocking,
    Review,
    Informational,
}

/// Exact comparison represented by one governed specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SpecificationComparison {
    Tracked,
    Minimum { limit: f64 },
    Maximum { limit: f64 },
    Range { minimum: f64, maximum: f64 },
    EqualWithin { target: f64, tolerance: f64 },
}

impl SpecificationComparison {
    fn from_legacy(entry: &SpecEntry) -> Self {
        match (entry.min, entry.max) {
            (None, None) => Self::Tracked,
            (Some(limit), None) => Self::Minimum { limit },
            (None, Some(limit)) => Self::Maximum { limit },
            (Some(minimum), Some(maximum)) => Self::Range { minimum, maximum },
        }
    }

    fn bounds(&self) -> (Option<f64>, Option<f64>) {
        match *self {
            Self::Tracked => (None, None),
            Self::Minimum { limit } => (Some(limit), None),
            Self::Maximum { limit } => (None, Some(limit)),
            Self::Range { minimum, maximum } => (Some(minimum), Some(maximum)),
            Self::EqualWithin { target, tolerance } => {
                (Some(target - tolerance), Some(target + tolerance))
            }
        }
    }

    fn validate(&self) -> Result<(), String> {
        let finite = |label: &str, value: f64| {
            value
                .is_finite()
                .then_some(())
                .ok_or_else(|| format!("specification {label} must be finite"))
        };
        match *self {
            Self::Tracked => Ok(()),
            Self::Minimum { limit } | Self::Maximum { limit } => finite("limit", limit),
            Self::Range { minimum, maximum } => {
                finite("minimum", minimum)?;
                finite("maximum", maximum)?;
                (minimum <= maximum)
                    .then_some(())
                    .ok_or_else(|| "specification minimum must not exceed maximum".to_owned())
            }
            Self::EqualWithin { target, tolerance } => {
                finite("equality target", target)?;
                finite("equality tolerance", tolerance)?;
                (tolerance >= 0.0).then_some(()).ok_or_else(|| {
                    "specification equality tolerance must not be negative".to_owned()
                })
            }
        }
    }

    pub(crate) fn bitwise_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tracked, Self::Tracked) => true,
            (Self::Minimum { limit: left }, Self::Minimum { limit: right })
            | (Self::Maximum { limit: left }, Self::Maximum { limit: right }) => {
                left.to_bits() == right.to_bits()
            }
            (
                Self::Range {
                    minimum: left_minimum,
                    maximum: left_maximum,
                },
                Self::Range {
                    minimum: right_minimum,
                    maximum: right_maximum,
                },
            ) => {
                left_minimum.to_bits() == right_minimum.to_bits()
                    && left_maximum.to_bits() == right_maximum.to_bits()
            }
            (
                Self::EqualWithin {
                    target: left_target,
                    tolerance: left_tolerance,
                },
                Self::EqualWithin {
                    target: right_target,
                    tolerance: right_tolerance,
                },
            ) => {
                left_target.to_bits() == right_target.to_bits()
                    && left_tolerance.to_bits() == right_tolerance.to_bits()
            }
            _ => false,
        }
    }
}

/// Durable provenance of an imported requirement row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecificationSource {
    pub logical_path: String,
    pub row: u64,
    pub imported_revision: String,
    pub source_digest: ContentDigest,
}

impl SpecificationSource {
    fn validate(&self) -> Result<(), String> {
        validate_bounded_text(
            "specification source path",
            &self.logical_path,
            1_024,
            false,
        )?;
        if self.row == 0 {
            return Err("specification source row must be one-based".to_owned());
        }
        validate_bounded_text(
            "specification imported revision",
            &self.imported_revision,
            128,
            false,
        )
    }
}

/// Explicit disposition attached to a governed requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecificationWaiver {
    pub reference: String,
    pub owner: String,
    pub rationale: String,
}

impl SpecificationWaiver {
    fn validate(&self) -> Result<(), String> {
        validate_bounded_text("waiver reference", &self.reference, 128, false)?;
        validate_bounded_text("waiver owner", &self.owner, 256, false)?;
        validate_bounded_text("waiver rationale", &self.rationale, 4_096, false)
    }
}

/// Canonical governed definition layered over the legacy scalar-spec
/// projection. Existing projects migrate deterministically on first access;
/// new authoring writes both projections until the legacy field is retired.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecificationDefinition {
    pub id: SpecificationId,
    pub requirement_key: String,
    pub requirement_name: String,
    pub measurement: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producing_analysis: Option<AnalysisInstanceId>,
    pub comparison: SpecificationComparison,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guard_band: Option<f64>,
    #[serde(default)]
    pub role: SpecificationRole,
    #[serde(default, skip_serializing_if = "SpecPointScope::is_all_points")]
    pub scope: SpecPointScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SpecificationSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver: Option<SpecificationWaiver>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub unit: String,
}

impl SpecificationDefinition {
    fn migrated_id(plan_id: SimulationPlanId, index: usize, measurement: &str) -> SpecificationId {
        let name = format!(
            "rspice.legacy-specification/v1/{index}/{}",
            measurement.to_ascii_lowercase()
        );
        SpecificationId::from_namespace(plan_id.as_uuid(), name.as_bytes())
    }

    pub(crate) fn from_legacy(plan_id: SimulationPlanId, index: usize, entry: &SpecEntry) -> Self {
        Self {
            id: Self::migrated_id(plan_id, index, &entry.measurement),
            requirement_key: format!("LEGACY-{:04}", index + 1),
            requirement_name: entry.measurement.clone(),
            measurement: entry.measurement.clone(),
            expression: entry.expression.clone(),
            producing_analysis: None,
            comparison: SpecificationComparison::from_legacy(entry),
            guard_band: None,
            role: SpecificationRole::Blocking,
            scope: entry.scope.clone(),
            source: None,
            waiver: None,
            unit: entry.unit.clone(),
        }
    }

    /// Create a genuinely new requirement row without reusing a deterministic
    /// migration identity. Deterministic IDs are reserved for upgrading
    /// historical payloads whose rows never had identities of their own.
    pub(crate) fn new_from_projection(entry: &SpecEntry) -> Self {
        let id = SpecificationId::new();
        let compact_id = id.to_string();
        Self {
            id,
            requirement_key: format!("SPEC-{}", &compact_id[..8]),
            requirement_name: entry.measurement.clone(),
            measurement: entry.measurement.clone(),
            expression: entry.expression.clone(),
            producing_analysis: None,
            comparison: SpecificationComparison::from_legacy(entry),
            guard_band: None,
            role: SpecificationRole::Blocking,
            scope: entry.scope.clone(),
            source: None,
            waiver: None,
            unit: entry.unit.clone(),
        }
    }

    pub(crate) fn apply_legacy_projection(&mut self, entry: &SpecEntry) {
        // The legacy projection cannot distinguish an authored
        // `EqualWithin` comparison from an equivalent min/max range. Preserve
        // the richer comparison whenever its projected rails did not change;
        // scope, name, expression, and unit edits must never erase that
        // semantic intent as collateral damage.
        let existing_bounds = self.comparison.bounds();
        let replacement_bounds = (entry.min, entry.max);
        let bounds_changed = existing_bounds.0.map(f64::to_bits)
            != replacement_bounds.0.map(f64::to_bits)
            || existing_bounds.1.map(f64::to_bits) != replacement_bounds.1.map(f64::to_bits);
        self.measurement.clone_from(&entry.measurement);
        self.expression.clone_from(&entry.expression);
        if bounds_changed {
            self.comparison = SpecificationComparison::from_legacy(entry);
        }
        self.scope.clone_from(&entry.scope);
        self.unit.clone_from(&entry.unit);
    }

    pub fn projected_entry(&self) -> SpecEntry {
        let (min, max) = self.comparison.bounds();
        SpecEntry {
            measurement: self.measurement.clone(),
            expression: self.expression.clone(),
            min,
            max,
            unit: self.unit.clone(),
            scope: self.scope.clone(),
        }
    }

    pub(crate) fn cloned_for_new_plan(
        &self,
        plan_id: SimulationPlanId,
        index: usize,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.id = Self::migrated_id(plan_id, index, &self.measurement);
        if let Some(analysis_id) = self.producing_analysis {
            cloned.producing_analysis = Some(
                analysis_identity_map
                    .get(&analysis_id)
                    .copied()
                    .ok_or(analysis_id)?,
            );
        }
        Ok(cloned)
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_bounded_text("requirement key", &self.requirement_key, 128, false)?;
        validate_bounded_text("requirement name", &self.requirement_name, 512, false)?;
        validate_parameter_name(&self.measurement)
            .map_err(|error| format!("measurement name is invalid: {error}"))?;
        if !self.expression.trim().is_empty() {
            validate_single_line_expression("measurement expression", &self.expression)?;
        }
        self.comparison.validate()?;
        if self
            .guard_band
            .is_some_and(|value| !value.is_finite() || value < 0.0)
        {
            return Err("specification guard band must be finite and nonnegative".to_owned());
        }
        if let Some(guard_band) = self.guard_band {
            match self.comparison {
                SpecificationComparison::Tracked => {
                    return Err(
                        "a tracked specification cannot carry an acceptance guard band".to_owned(),
                    );
                }
                SpecificationComparison::Range { minimum, maximum }
                    if guard_band > (maximum - minimum) / 2.0 =>
                {
                    return Err(
                        "specification guard band leaves no valid range between its limits"
                            .to_owned(),
                    );
                }
                SpecificationComparison::EqualWithin { tolerance, .. }
                    if guard_band > tolerance =>
                {
                    return Err(
                        "specification guard band exceeds its equality tolerance".to_owned()
                    );
                }
                SpecificationComparison::Minimum { .. }
                | SpecificationComparison::Maximum { .. }
                | SpecificationComparison::Range { .. }
                | SpecificationComparison::EqualWithin { .. } => {}
            }
        }
        self.scope.validate()?;
        if let Some(source) = &self.source {
            source.validate()?;
        }
        if let Some(waiver) = &self.waiver {
            waiver.validate()?;
        }
        validate_bounded_text("specification unit", &self.unit, 64, true)
    }

    pub(crate) fn bitwise_eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.requirement_key == other.requirement_key
            && self.requirement_name == other.requirement_name
            && self.measurement == other.measurement
            && self.expression == other.expression
            && self.producing_analysis == other.producing_analysis
            && self.comparison.bitwise_eq(&other.comparison)
            && self.guard_band.map(f64::to_bits) == other.guard_band.map(f64::to_bits)
            && self.role == other.role
            && self.scope == other.scope
            && self.source == other.source
            && self.waiver == other.waiver
            && self.unit == other.unit
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NominalFailurePolicy {
    #[default]
    Block,
    RecordDisposition,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RegressionSpecificationPolicy {
    #[default]
    LimitAndWaveform,
    LimitOnly,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MissingMeasurementPolicy {
    #[default]
    FailClosed,
    ReportUnmapped,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum MonteCarloSpecificationGate {
    #[default]
    NotGated,
    YieldAtLeast {
        percent: f64,
    },
}

/// Plan-wide policy for evaluating specification evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SpecificationPolicy {
    #[serde(default)]
    pub nominal_failure: NominalFailurePolicy,
    #[serde(default)]
    pub monte_carlo: MonteCarloSpecificationGate,
    #[serde(default)]
    pub regression: RegressionSpecificationPolicy,
    #[serde(default)]
    pub missing_measurement: MissingMeasurementPolicy,
}

impl MonteCarloSpecificationGate {
    /// Whether a population of `total` judged trials, `passing` of which held
    /// the guard-banded bound, satisfies this gate.
    ///
    /// The one predicate. The acceptance gate asks it to decide whether a run
    /// signs off, and the specification registry asks it to decide what a row
    /// says — so a population the sign-off accepts cannot be a row that reads
    /// `Fail`. An ungated policy clears everything: the worst-trial verdict is
    /// then the whole answer, which is what "not gated" means.
    ///
    /// An empty population never clears a gate. Dividing by nothing is not a
    /// yield, and a specification with no trials is missing evidence rather
    /// than a distribution that held.
    #[must_use]
    pub fn clears(&self, passing: u64, total: u64) -> bool {
        match self {
            Self::NotGated => true,
            Self::YieldAtLeast { percent } => {
                total > 0 && 100.0 * passing as f64 / total as f64 >= *percent
            }
        }
    }
}

impl SpecificationPolicy {
    pub fn validate(&self) -> Result<(), String> {
        if let MonteCarloSpecificationGate::YieldAtLeast { percent } = self.monte_carlo
            && (!percent.is_finite() || !(0.0..=100.0).contains(&percent))
        {
            return Err(
                "Monte Carlo specification yield gate must be from 0 through 100 percent"
                    .to_owned(),
            );
        }
        Ok(())
    }

    pub(crate) fn bitwise_eq(&self, other: &Self) -> bool {
        self.nominal_failure == other.nominal_failure
            && match (&self.monte_carlo, &other.monte_carlo) {
                (MonteCarloSpecificationGate::NotGated, MonteCarloSpecificationGate::NotGated) => {
                    true
                }
                (
                    MonteCarloSpecificationGate::YieldAtLeast { percent: left },
                    MonteCarloSpecificationGate::YieldAtLeast { percent: right },
                ) => left.to_bits() == right.to_bits(),
                _ => false,
            }
            && self.regression == other.regression
            && self.missing_measurement == other.missing_measurement
    }
}

/// One specification bound for a `.MEAS` result — a row of the specs
/// matrix. At least one of `min`/`max` is normally set; a spec with
/// neither still pins the measurement as a tracked row (value-only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecEntry {
    /// `.MEAS` result name this spec bounds (case-insensitive match).
    pub measurement: String,
    /// Authored `.MEAS` expression or statement body that defines the
    /// measurement. Older projects did not retain this text, so migration
    /// leaves it empty rather than reconstructing source from a result value.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expression: String,
    /// Lower bound (pass when value ≥ min).
    pub min: Option<f64>,
    /// Upper bound (pass when value ≤ max).
    pub max: Option<f64>,
    /// Display unit, purely cosmetic (e.g. "V", "s", "dB").
    pub unit: String,
    /// Which PVT points this bound is a claim about. Plans authored before
    /// the scope existed judged every retained point, so the default has to
    /// stay [`SpecPointScope::AllPoints`] or reloading one would narrow a
    /// requirement nobody narrowed.
    #[serde(default, skip_serializing_if = "SpecPointScope::is_all_points")]
    pub scope: SpecPointScope,
}

impl SpecPointScope {
    /// Whether serialization may leave the field out entirely.
    #[must_use]
    pub const fn is_all_points(&self) -> bool {
        matches!(self, Self::AllPoints)
    }
}

impl SpecEntry {
    pub fn validate(&self) -> Result<(), String> {
        validate_parameter_name(&self.measurement)
            .map_err(|error| format!("measurement name is invalid: {error}"))?;
        if !self.expression.trim().is_empty() {
            validate_single_line_expression("measurement expression", &self.expression)?;
        }
        self.scope.validate()?;
        if self.min.is_some_and(|value| !value.is_finite())
            || self.max.is_some_and(|value| !value.is_finite())
        {
            return Err("specification bounds must be finite".to_owned());
        }
        if self
            .min
            .zip(self.max)
            .is_some_and(|(minimum, maximum)| minimum > maximum)
        {
            return Err("specification minimum exceeds its maximum".to_owned());
        }
        validate_bounded_text("unit", &self.unit, 64, true)
    }

    /// Spec verdict for one measured value.
    pub fn passes(&self, value: f64) -> bool {
        self.min.is_none_or(|min| value >= min) && self.max.is_none_or(|max| value <= max)
    }

    /// Violation magnitude (how far outside the bounds), 0 when passing.
    pub fn violation(&self, value: f64) -> f64 {
        let below = self.min.map_or(0.0, |min| (min - value).max(0.0));
        let above = self.max.map_or(0.0, |max| (value - max).max(0.0));
        below.max(above)
    }
}

/// Physical quantity carried by a design variable. The quantity is retained
/// independently from the expression so editors can validate units without
/// coercing the user's exact engineering input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableQuantity {
    Resistance,
    Capacitance,
    Voltage,
    Current,
    Temperature,
    Dimensionless,
}

impl DesignVariableQuantity {
    pub const ALL: [Self; 6] = [
        Self::Resistance,
        Self::Capacitance,
        Self::Voltage,
        Self::Current,
        Self::Temperature,
        Self::Dimensionless,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Resistance => "Resistance",
            Self::Capacitance => "Capacitance",
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Temperature => "Temperature",
            Self::Dimensionless => "Dimensionless",
        }
    }
}

/// Exact ownership boundary for a design variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DesignVariableScope {
    Testbench,
    Project,
    SelectedCell { cell: CellViewRef },
    SelectedAnalysis { analysis_id: AnalysisInstanceId },
}

impl DesignVariableScope {
    /// The ownership boundary, named as a boundary.
    ///
    /// `Testbench` read "Lab characterization · testbench", which baked the
    /// *default plan name* into a scope label: rename the plan and the label
    /// went on naming the old one, in a column whose other three entries name
    /// nothing but boundaries. The spec-sheet importer accepts these spellings
    /// as cell values, so the rename travels there too.
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Testbench => "Testbench",
            Self::Project => "Project",
            Self::SelectedCell { .. } => "Selected cell",
            Self::SelectedAnalysis { .. } => "Selected analysis only",
        }
    }
}

/// Inclusive engineering bounds for a variable. Bounds remain expressions so
/// suffixes and owner variables survive a lossless project round trip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignVariableRange {
    pub minimum: String,
    pub maximum: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableSweepEligibility {
    NestedSweepAndOptimization,
    OptimizationOnly,
    FixedParameter,
}

impl DesignVariableSweepEligibility {
    pub const ALL: [Self; 3] = [
        Self::NestedSweepAndOptimization,
        Self::OptimizationOnly,
        Self::FixedParameter,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::NestedSweepAndOptimization => "Nested sweep + optimization",
            Self::OptimizationOnly => "Optimization only",
            Self::FixedParameter => "Fixed parameter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableOverridePolicy {
    ExplicitTestLocalOverride,
    InheritOwnerOnly,
}

impl DesignVariableOverridePolicy {
    pub const ALL: [Self; 2] = [Self::ExplicitTestLocalOverride, Self::InheritOwnerOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ExplicitTestLocalOverride => "Explicit test-local override",
            Self::InheritOwnerOnly => "Inherit owner only",
        }
    }
}

/// Persisted, typed simulation-plan parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DesignVariable {
    pub id: DesignVariableId,
    pub revision: ObjectRevision,
    pub name: String,
    pub expression: String,
    pub quantity: DesignVariableQuantity,
    pub scope: DesignVariableScope,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub allowed_range: Option<DesignVariableRange>,
    pub sweep_eligibility: DesignVariableSweepEligibility,
    pub override_policy: DesignVariableOverridePolicy,
}

/// Which contract a design variable failed.
///
/// Deliberately coarse. These are the four kinds of thing an author can get
/// wrong about a variable, not one variant per message: a finer split would
/// have to be kept in step with the wording, which is the coupling this exists
/// to remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignVariableDefect {
    /// The name is not a usable parameter identifier.
    Identifier,
    /// A value does not read as the quantity it is declared in — the
    /// expression itself, or one end of the allowed range.
    Dimension,
    /// The allowed range is inconsistent, or the resolved value falls outside
    /// it.
    Bounds,
    /// The rest of the record contract: description length, ownership scope.
    Record,
}

impl DesignVariable {
    pub fn new(
        name: impl Into<String>,
        expression: impl Into<String>,
        quantity: DesignVariableQuantity,
        scope: DesignVariableScope,
        description: impl Into<String>,
        allowed_range: Option<DesignVariableRange>,
        sweep_eligibility: DesignVariableSweepEligibility,
        override_policy: DesignVariableOverridePolicy,
    ) -> Result<Self, String> {
        Self::new_defect(
            name,
            expression,
            quantity,
            scope,
            description,
            allowed_range,
            sweep_eligibility,
            override_policy,
        )
        .map_err(|(_, message)| message)
    }

    /// [`Self::new`], with the failed contract named beside the sentence. See
    /// [`Self::validate_defect`] for why anything needs that.
    pub fn new_defect(
        name: impl Into<String>,
        expression: impl Into<String>,
        quantity: DesignVariableQuantity,
        scope: DesignVariableScope,
        description: impl Into<String>,
        allowed_range: Option<DesignVariableRange>,
        sweep_eligibility: DesignVariableSweepEligibility,
        override_policy: DesignVariableOverridePolicy,
    ) -> Result<Self, (DesignVariableDefect, String)> {
        let variable = Self {
            id: DesignVariableId::new(),
            revision: ObjectRevision::INITIAL,
            name: name.into(),
            expression: expression.into(),
            quantity,
            scope,
            description: description.into(),
            allowed_range,
            sweep_eligibility,
            override_policy,
        };
        variable.validate_defect()?;
        Ok(variable)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_defect().map_err(|(_, message)| message)
    }

    /// [`Self::validate`], with the failed contract named beside the sentence.
    ///
    /// The sentence says *how* a variable is wrong, which is what a reader
    /// needs. A caller that has to route, group or identify refusals needs to
    /// know *which* contract failed, and reading that back out of English is
    /// how a message reword becomes a silent behaviour change. The spec-sheet
    /// import is the caller that needs it: its refusals carry a stable
    /// identity, and this is where the rule that failed already is.
    ///
    /// Every arm of [`Self::validate`] is tagged here rather than a second set
    /// of checks being written beside them, so there remains exactly one
    /// definition of a valid design variable.
    pub fn validate_defect(&self) -> Result<(), (DesignVariableDefect, String)> {
        use DesignVariableDefect as Defect;
        let tag = |defect: Defect| move |message: String| (defect, message);

        validate_parameter_name(&self.name).map_err(tag(Defect::Identifier))?;
        validate_single_line_expression("expression", &self.expression)
            .map_err(tag(Defect::Dimension))?;
        let value = self.resolved_value_si().map_err(tag(Defect::Dimension))?;
        if let Some(range) = &self.allowed_range {
            validate_single_line_expression("allowed-range minimum", &range.minimum)
                .map_err(tag(Defect::Bounds))?;
            validate_single_line_expression("allowed-range maximum", &range.maximum)
                .map_err(tag(Defect::Bounds))?;
            let minimum = parse_design_quantity(&range.minimum, self.quantity)
                .map_err(|error| {
                    format!(
                        "allowed-range minimum is invalid for {}: {error}",
                        self.quantity.label()
                    )
                })
                .map_err(tag(Defect::Dimension))?;
            let maximum = parse_design_quantity(&range.maximum, self.quantity)
                .map_err(|error| {
                    format!(
                        "allowed-range maximum is invalid for {}: {error}",
                        self.quantity.label()
                    )
                })
                .map_err(tag(Defect::Dimension))?;
            if minimum > maximum {
                return Err((
                    Defect::Bounds,
                    "allowed-range minimum exceeds its maximum".to_owned(),
                ));
            }
            if value < minimum || value > maximum {
                return Err((
                    Defect::Bounds,
                    format!(
                        "resolved value {value} is outside the inclusive allowed range {minimum}..={maximum}"
                    ),
                ));
            }
        }
        validate_bounded_text("description", &self.description, 1_024, true)
            .map_err(tag(Defect::Record))?;
        if let DesignVariableScope::SelectedCell { cell } = &self.scope {
            cell.validate_name_segments()
                .map_err(|error| format!("selected cell is invalid: {error}"))
                .map_err(tag(Defect::Record))?;
        }
        Ok(())
    }

    pub fn resolved_value_si(&self) -> Result<f64, String> {
        parse_design_quantity(&self.expression, self.quantity).map_err(|error| {
            format!(
                "expression is invalid for {}: {error}",
                self.quantity.label()
            )
        })
    }

    /// Canonical top-level SPICE statement. Validation is intentionally kept
    /// separate so callers can aggregate every project diagnostic at once.
    pub fn netlist_statement(&self) -> String {
        let value = self
            .resolved_value_si()
            .expect("validated design variables always resolve to finite SI values");
        format!(".param {}={value:.17e}", self.name)
    }

    pub(super) fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.id = DesignVariableId::new();
        cloned.revision = ObjectRevision::INITIAL;
        if let DesignVariableScope::SelectedAnalysis { analysis_id } = &mut cloned.scope {
            *analysis_id = analysis_identity_map
                .get(analysis_id)
                .copied()
                .ok_or(*analysis_id)?;
        }
        Ok(cloned)
    }
}

pub(super) const LEGACY_DESIGN_VARIABLE_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0x3c56_6c65_03dc_5e65_b66c_a4d4_86dc_8d53);

impl<'de> Deserialize<'de> for DesignVariable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            #[serde(default = "missing_identity_sentinel")]
            id: serde_json::Value,
            #[serde(default)]
            revision: ObjectRevision,
            name: String,
            expression: String,
            quantity: DesignVariableQuantity,
            scope: DesignVariableScope,
            #[serde(default)]
            description: String,
            #[serde(default)]
            allowed_range: Option<DesignVariableRange>,
            sweep_eligibility: DesignVariableSweepEligibility,
            override_policy: DesignVariableOverridePolicy,
        }

        let wire = Wire::deserialize(deserializer)?;
        let identity = serde_json::to_vec(&(
            &wire.name,
            &wire.expression,
            wire.quantity,
            &wire.scope,
            &wire.description,
            &wire.allowed_range,
            wire.sweep_eligibility,
            wire.override_policy,
        ))
        .map_err(D::Error::custom)?;
        let id = deserialize_or_migrate_identity::<DesignVariableId, D::Error>(
            wire.id,
            LEGACY_DESIGN_VARIABLE_ID_NAMESPACE,
            &identity,
            DesignVariableId::from_namespace,
        )?;
        Ok(Self {
            id,
            revision: wire.revision,
            name: wire.name,
            expression: wire.expression,
            quantity: wire.quantity,
            scope: wire.scope,
            description: wire.description,
            allowed_range: wire.allowed_range,
            sweep_eligibility: wire.sweep_eligibility,
            override_policy: wire.override_policy,
        })
    }
}

#[cfg(test)]
mod monte_carlo_gate_tests {
    use super::MonteCarloSpecificationGate;

    /// The gate is one predicate, and both readers of it are decisions.
    ///
    /// The acceptance gate asks it whether a run signs off; the specification
    /// registry asks it what a row says. They were separate arithmetic, so a
    /// population the sign-off accepted could be drawn as `Fail` — the worst
    /// trial's verdict, which is the one number a yield gate exists to stop
    /// deciding on its own.
    #[test]
    fn a_yield_gate_is_cleared_at_its_boundary_and_missed_below_it() {
        let gate = MonteCarloSpecificationGate::YieldAtLeast { percent: 95.0 };

        assert!(gate.clears(95, 100), "exactly the gate is clearing it");
        assert!(gate.clears(96, 100));
        assert!(!gate.clears(94, 100));
        assert!(
            gate.clears(0, 0) == false,
            "dividing by no trials is not a yield"
        );
    }

    #[test]
    fn an_ungated_policy_clears_every_population_including_an_empty_one() {
        // "Not gated" means the worst-of verdict is the whole answer, so this
        // predicate must never be what turns such a specification down.
        let gate = MonteCarloSpecificationGate::NotGated;

        assert!(gate.clears(0, 500));
        assert!(gate.clears(0, 0));
    }
}
