//! The typed run-set state: dimensions, their values, and how they compose.
//!
//! A run set is the plan's declaration of *what space is executed*. It is one
//! ordered list of enabled dimensions, a composition rule that turns them into
//! points, and the budgets a preview is checked against. Every value carries a
//! stable identity and the revision it came from, so a point in a manifest can
//! be traced back to the exact declaration that produced it.
//!
//! The persisted vocabulary matches the Simulation Studio contract. Validation
//! additionally proves that every enabled kind has a concrete binding for the
//! current execution path, so a visible axis can never silently do nothing.

use serde::{Deserialize, Serialize};

/// What a dimension varies, and therefore how it reaches the engine.
///
/// A kind becomes executable through its source authority. For example,
/// `Parameter` uses `design-variable:<name>`, while `Supply` uses an explicit
/// list of independent voltage-source instance names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunSetDimensionKind {
    /// A named SPICE/design variable.
    Parameter,
    /// An independent source value or waveform choice.
    Source,
    /// PDK/library section: `tt`, `ss`, `ff`, `sf`, `fs`.
    ProcessSection,
    /// Supply voltage applied to the design's DC sources.
    Supply,
    /// Ambient and model-evaluation temperature, in °C.
    Temperature,
    /// A section/reference belonging to one selected model-library binding.
    Model,
    /// Analysis frequency or frequency-range control.
    Frequency,
    /// Analysis time or time-range control.
    Time,
    /// Reproducible statistical seed.
    Seed,
    /// Statistical sample/trial count.
    Sample,
    /// Selects which analysis instances execute at a point.
    AnalysisSelection,
    /// Selects a frozen mixed-signal/digital configuration.
    DigitalConfiguration,
    /// Selects a sealed external dataset.
    ExternalDataset,
}

impl RunSetDimensionKind {
    /// Every kind, in the order a fresh run set declares them.
    pub const ALL: [Self; 13] = [
        Self::Parameter,
        Self::Source,
        Self::Temperature,
        Self::ProcessSection,
        Self::Model,
        Self::Supply,
        Self::Frequency,
        Self::Time,
        Self::Seed,
        Self::Sample,
        Self::AnalysisSelection,
        Self::DigitalConfiguration,
        Self::ExternalDataset,
    ];

    /// Stable lexical name, matching the run-set contract vocabulary.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parameter => "parameter",
            Self::Source => "source",
            Self::ProcessSection => "process-section",
            Self::Supply => "supply",
            Self::Temperature => "temperature",
            Self::Model => "model",
            Self::Frequency => "frequency",
            Self::Time => "time",
            Self::Seed => "seed",
            Self::Sample => "sample",
            Self::AnalysisSelection => "analysis-selection",
            Self::DigitalConfiguration => "digital-configuration",
            Self::ExternalDataset => "external-dataset",
        }
    }

    /// The value type a dimension of this kind carries. The kind decides it:
    /// a process section is a reference into a library, a supply and a
    /// temperature are physical quantities.
    #[must_use]
    pub fn value_type(self) -> RunSetValueType {
        match self {
            Self::ProcessSection
            | Self::Model
            | Self::AnalysisSelection
            | Self::DigitalConfiguration
            | Self::ExternalDataset => RunSetValueType::Reference,
            Self::Seed | Self::Sample => RunSetValueType::Integer,
            Self::Parameter
            | Self::Source
            | Self::Supply
            | Self::Temperature
            | Self::Frequency
            | Self::Time => RunSetValueType::Quantity,
        }
    }

    /// Canonical storage unit, or `None` when the values are references.
    #[must_use]
    pub fn unit(self) -> Option<&'static str> {
        match self {
            Self::ProcessSection
            | Self::Parameter
            | Self::Source
            | Self::Model
            | Self::Seed
            | Self::Sample
            | Self::AnalysisSelection
            | Self::DigitalConfiguration
            | Self::ExternalDataset => None,
            Self::Supply => Some("V"),
            Self::Temperature => Some("°C"),
            Self::Frequency => Some("Hz"),
            Self::Time => Some("s"),
        }
    }

    /// Where the values are authored, shown as the dimension's authority.
    #[must_use]
    pub fn source(self) -> &'static str {
        match self {
            Self::Parameter => "design-variable:",
            Self::Source => "netlist-source:",
            Self::ProcessSection => "model-library:process-sections",
            Self::Supply => "unbound:supply-sources",
            Self::Temperature => "run-set:temperature-axis",
            Self::Model => "model-binding:",
            Self::Frequency => "analysis-frequency:all",
            Self::Time => "analysis-time:all",
            Self::Seed => "monte-carlo:seed",
            Self::Sample => "monte-carlo:samples",
            Self::AnalysisSelection => "analysis:",
            Self::DigitalConfiguration => "digital-configuration:",
            Self::ExternalDataset => "external-dataset:",
        }
    }

    /// Current execution limitation for a persisted dimension vocabulary.
    ///
    /// Unsupported kinds remain deserializable so an older or newer project
    /// never loses authored intent. Studio creation and execution surfaces use
    /// this same reason to keep those declarations visible and fail closed.
    #[must_use]
    pub const fn execution_blocker(self) -> Option<&'static str> {
        match self {
            Self::Model => Some(
                "per-point model selection is unavailable; ordered model-library bindings currently apply to the whole simulation plan",
            ),
            Self::Frequency => Some(
                "per-point frequency-control binding is unavailable; configure the frequency sweep on each analysis instance",
            ),
            Self::Time => Some(
                "per-point time-control binding is unavailable; configure time controls on each transient analysis instance",
            ),
            Self::Seed => Some(
                "per-point statistical seed binding is unavailable; configure the reproducible seed on the Monte Carlo analysis",
            ),
            Self::Sample => Some(
                "per-point statistical sample-count binding is unavailable; configure samples on the Monte Carlo analysis",
            ),
            Self::AnalysisSelection => Some(
                "per-point analysis selection requires a task-subset scheduler that is unavailable in this engine build",
            ),
            Self::DigitalConfiguration => Some(
                "frozen mixed-signal digital configurations are not available in this engine build",
            ),
            Self::ExternalDataset => {
                Some("sealed external-dataset resolution is not available in this engine build")
            }
            Self::Parameter
            | Self::Source
            | Self::ProcessSection
            | Self::Supply
            | Self::Temperature => None,
        }
    }

    /// A dimension of this kind may appear at most once: the engine binds one
    /// process section, one supply scaling and one temperature per point, so a
    /// second dimension of the same kind would have no distinct binding.
    #[must_use]
    pub fn default_name(self) -> &'static str {
        match self {
            Self::Parameter => "Design parameter",
            Self::Source => "Source value",
            Self::ProcessSection => "Process section",
            Self::Supply => "Supply voltage",
            Self::Temperature => "Temperature",
            Self::Model => "Model selection",
            Self::Frequency => "Frequency",
            Self::Time => "Time",
            Self::Seed => "Random seed",
            Self::Sample => "Sample count",
            Self::AnalysisSelection => "Analysis selection",
            Self::DigitalConfiguration => "Digital configuration",
            Self::ExternalDataset => "External dataset",
        }
    }
}

/// The type every value of a dimension is parsed as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunSetValueType {
    /// A physical quantity in the dimension's canonical unit.
    Quantity,
    /// A signed whole number represented exactly.
    Integer,
    /// A true/false switch.
    Boolean,
    /// One member of a lexical vocabulary.
    Enum,
    /// A name resolved against an external authority.
    Reference,
    /// A SPICE expression retained exactly for a compatible binding.
    Expression,
    /// A structured JSON value used by configuration bindings.
    Object,
}

impl RunSetValueType {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Quantity => "quantity",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::Enum => "enum",
            Self::Reference => "reference",
            Self::Expression => "expression",
            Self::Object => "object",
        }
    }
}

/// How the enabled dimensions become points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunSetCompositionMode {
    /// Full cross product; the point count is the product of the value counts.
    #[default]
    Cartesian,
    /// Index-aligned pairing. Non-scalar dimensions must share one length —
    /// implicit cycling is prohibited, because a run set that quietly repeated
    /// a value would report a point count the manifest could not justify.
    Zipped,
    /// The cross product with named points removed. The removals are held as
    /// point identities in [`RunSetComposition::excluded_points`], so a
    /// combination the design cannot reach is dropped from the run without
    /// distorting the axes that produced it.
    Filtered,
    /// Cartesian candidates retained only when a typed predicate evaluates to
    /// true for their upstream coordinates.
    Conditional,
    /// Initial declared points followed by proposals from a frozen policy.
    Adaptive,
    /// Ordered hierarchical traversal of the same exact Cartesian leaves.
    Nested,
}

impl RunSetCompositionMode {
    pub const ALL: [Self; 6] = [
        Self::Cartesian,
        Self::Zipped,
        Self::Filtered,
        Self::Conditional,
        Self::Adaptive,
        Self::Nested,
    ];

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cartesian => "cartesian",
            Self::Zipped => "zipped",
            Self::Filtered => "filtered",
            Self::Conditional => "conditional",
            Self::Adaptive => "adaptive",
            Self::Nested => "nested",
        }
    }

    /// Glyph drawn between adjacent axis cards.
    ///
    /// A filtered space is a cross product, so it carries the cross product's
    /// glyph: the exclusions remove points from the result, they do not change
    /// how the axes combine.
    #[must_use]
    pub fn operator(self) -> &'static str {
        match self {
            Self::Cartesian | Self::Filtered | Self::Nested => "×",
            Self::Zipped => "⇅",
            Self::Conditional => "?",
            Self::Adaptive => "↗",
        }
    }

    /// The mode's contract, stated where it is chosen.
    #[must_use]
    pub fn contract(self) -> &'static str {
        match self {
            Self::Cartesian => {
                "Full cross product of every enabled dimension; the point count is the product of \
                 their value counts."
            }
            Self::Zipped => {
                "Index-aligned pairing: non-scalar dimensions must share one length, and implicit \
                 cycling is prohibited."
            }
            Self::Filtered => {
                "Full cross product less an explicit set of excluded points, each named by the \
                 identities of the values it is made of. An exclusion the space no longer contains \
                 is reported, never silently discarded."
            }
            Self::Conditional => {
                "Cartesian candidates accepted only when the versioned predicate over declared upstream dimensions evaluates to true."
            }
            Self::Adaptive => {
                "The declared matrix is the initial design; a frozen deterministic policy may add proposals up to its stated maximum."
            }
            Self::Nested => {
                "Ordered hierarchical traversal of Cartesian leaves, bounded by the declared maximum nesting depth."
            }
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Cartesian => "Cartesian · every combination",
            Self::Zipped => "Zipped · index-aligned",
            Self::Filtered => "Filtered · Cartesian less excluded points",
            Self::Conditional => "Conditional · predicate-selected points",
            Self::Adaptive => "Adaptive · frozen proposal policy",
            Self::Nested => "Nested · hierarchical traversal",
        }
    }

    /// Current execution limitation for a persisted composition vocabulary.
    #[must_use]
    pub const fn execution_blocker(self) -> Option<&'static str> {
        match self {
            Self::Adaptive => Some(
                "adaptive composition requires a result-feedback proposal scheduler that is unavailable in this engine build",
            ),
            Self::Cartesian | Self::Zipped | Self::Filtered | Self::Conditional | Self::Nested => {
                None
            }
        }
    }
}

/// What happens to the rest of the run when one value does not parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InvalidValuePolicy {
    /// Retain the invalid value with its identity and block only the points it
    /// participates in.
    #[default]
    PreserveAndBlockAffectedPoints,
    /// Refuse the whole run set until the value is corrected.
    BlockEntireRunSet,
}

impl InvalidValuePolicy {
    pub const ALL: [Self; 2] = [
        Self::PreserveAndBlockAffectedPoints,
        Self::BlockEntireRunSet,
    ];
}

/// One typed value of one dimension.
///
/// `lexical` is exactly what was authored and is never rewritten: a value that
/// does not parse stays visible with its identity intact rather than being
/// dropped, so the point it blocks can be named. `canonical` is the parsed
/// number the engine receives, absent when the lexical form is not valid.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSetValue {
    /// Stable identity, unique across the whole run set.
    pub id: String,
    /// The authored text.
    pub lexical: String,
    /// The parsed value in the dimension's canonical unit.
    pub canonical: Option<f64>,
    /// The plan revision this value was authored at.
    pub source_revision: u32,
}

impl RunSetValue {
    /// Parse `lexical` for `kind` and build the value with a stable identity.
    #[must_use]
    pub fn new(id: String, lexical: &str, kind: RunSetDimensionKind, source_revision: u32) -> Self {
        Self {
            id,
            lexical: lexical.to_owned(),
            canonical: canonical_value(lexical, kind),
            source_revision,
        }
    }
}

/// Parse one authored value for a dimension kind.
///
/// A process section is a reference and carries no number, so it canonicalizes
/// to its own index in the section vocabulary; that keeps "every value has a
/// canonical form" true without inventing a physical quantity for a name.
fn canonical_value(lexical: &str, kind: RunSetDimensionKind) -> Option<f64> {
    let text = lexical.trim();
    if text.is_empty() {
        return None;
    }
    match kind {
        RunSetDimensionKind::ProcessSection => {
            process_section_index(text).map(|index| index as f64)
        }
        RunSetDimensionKind::Parameter | RunSetDimensionKind::Source => {
            let value = crate::simulation::dialog::options::parse_si_value(text).ok()?;
            value.is_finite().then_some(value)
        }
        RunSetDimensionKind::Supply => {
            let value = crate::simulation::dialog::options::parse_si_value(text).ok()?;
            (value.is_finite() && value > 0.0).then_some(value)
        }
        RunSetDimensionKind::Temperature => {
            // A temperature may be written with its unit ("125 °C") because
            // that is how the axis reads in the point table; the canonical
            // form is always Celsius, which is what the solver's TEMP takes.
            let stripped = text
                .trim_end_matches(['C', 'c'])
                .trim_end_matches('°')
                .trim();
            let value = crate::simulation::dialog::options::parse_si_value(stripped).ok()?;
            (value.is_finite() && value > -273.15).then_some(value)
        }
        RunSetDimensionKind::Frequency | RunSetDimensionKind::Time => {
            let value = crate::simulation::dialog::options::parse_si_value(text).ok()?;
            (value.is_finite() && value > 0.0).then_some(value)
        }
        RunSetDimensionKind::Seed => text
            .parse::<u64>()
            .ok()
            .filter(|value| *value <= (1_u64 << 53))
            .map(|value| value as f64),
        RunSetDimensionKind::Sample => text
            .parse::<u64>()
            .ok()
            .filter(|value| *value > 0 && *value <= (1_u64 << 53))
            .map(|value| value as f64),
        // These kinds carry references. Their exact lexical identity is the
        // canonical payload; the numeric sentinel records parse validity for
        // the legacy point expander without hashing or otherwise weakening the
        // reference text.
        RunSetDimensionKind::Model
        | RunSetDimensionKind::AnalysisSelection
        | RunSetDimensionKind::DigitalConfiguration
        | RunSetDimensionKind::ExternalDataset => Some(0.0),
    }
}

/// The five process sections, in the order the corner executor declares them.
pub const PROCESS_SECTIONS: [&str; 5] = ["TT", "SS", "FF", "SF", "FS"];

/// Index of a process section name, case-insensitively.
#[must_use]
pub fn process_section_index(name: &str) -> Option<usize> {
    PROCESS_SECTIONS
        .iter()
        .position(|section| section.eq_ignore_ascii_case(name.trim()))
}

/// One axis of the run space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSetDimension {
    /// Stable identity, unique within the run set.
    pub id: String,
    /// Display name, editable.
    pub name: String,
    /// What this dimension varies, and how it binds to the engine.
    pub kind: RunSetDimensionKind,
    /// Where its values are authored.
    pub source: String,
    /// Ordered typed values.
    pub values: Vec<RunSetValue>,
    /// What an unparseable value does to the run.
    pub invalid_value_policy: InvalidValuePolicy,
    /// Whether this dimension contributes to the space.
    pub enabled: bool,
}

/// Prefix used by a supply dimension's source authority. The suffix is a
/// comma-separated list of exact independent voltage-source instance names.
pub const NETLIST_SUPPLY_SOURCE_PREFIX: &str = "netlist-source:";
/// Prefix used by a parameter dimension's source authority. The suffix is the
/// exact SPICE parameter/design-variable identifier to override.
pub const DESIGN_VARIABLE_SOURCE_PREFIX: &str = "design-variable:";

/// Resolve a parameter axis to the exact identifier written into its
/// point-specific `.param` card.
pub fn parse_parameter_source_authority(source: &str) -> Result<String, String> {
    parse_single_identifier_authority(source, DESIGN_VARIABLE_SOURCE_PREFIX, "parameter")
}

/// Resolve an absolute source-value axis to one exact independent source.
pub fn parse_source_value_authority(source: &str) -> Result<String, String> {
    parse_single_identifier_authority(source, NETLIST_SUPPLY_SOURCE_PREFIX, "source")
}

fn parse_single_identifier_authority(
    source: &str,
    prefix: &str,
    noun: &str,
) -> Result<String, String> {
    let Some(name) = source.strip_prefix(prefix) else {
        return Err(format!(
            "{noun} source authority must use {prefix} followed by one exact identifier"
        ));
    };
    if name.is_empty()
        || name != name.trim()
        || !name
            .chars()
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        || !name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(format!(
            "{noun} source authority {source:?} does not end in one valid SPICE identifier"
        ));
    }
    Ok(name.to_owned())
}

/// Resolve the exact independent voltage sources a supply axis is authorized
/// to modify. Ambiguous authority labels from older projects intentionally do
/// not infer a rail from topology.
pub fn parse_supply_source_authority(source: &str) -> Result<Vec<String>, String> {
    let Some(names) = source.strip_prefix(NETLIST_SUPPLY_SOURCE_PREFIX) else {
        return Err(format!(
            "Supply source authority must use {NETLIST_SUPPLY_SOURCE_PREFIX} followed by one or more exact source names"
        ));
    };
    let mut seen = std::collections::BTreeSet::new();
    let mut resolved = Vec::new();
    for name in names.split(',') {
        let name = name.trim();
        if name.is_empty() || name.chars().any(char::is_control) {
            return Err(
                "Supply source authority contains an empty or malformed source name".to_owned(),
            );
        }
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(format!("Supply source authority repeats {name:?}"));
        }
        resolved.push(name.to_owned());
    }
    if resolved.is_empty() {
        return Err("Supply source authority requires at least one source name".to_owned());
    }
    Ok(resolved)
}

impl RunSetDimension {
    /// A dimension of `kind` carrying `values`, authored at `revision`.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        kind: RunSetDimensionKind,
        values: &[&str],
        revision: u32,
    ) -> Self {
        let id = id.into();
        let values = values
            .iter()
            .enumerate()
            .map(|(index, lexical)| RunSetValue::new(value_id(&id, index), lexical, kind, revision))
            .collect();
        Self {
            id,
            name: kind.default_name().to_owned(),
            kind,
            source: kind.source().to_owned(),
            values,
            invalid_value_policy: InvalidValuePolicy::default(),
            enabled: true,
        }
    }

    /// The dimension's canonical unit.
    #[must_use]
    pub fn unit(&self) -> Option<&'static str> {
        self.kind.unit()
    }

    /// Replace the value list from one authored line per value, preserving the
    /// identity of every value that keeps its position.
    pub fn set_values_from_lines(&mut self, text: &str, revision: u32) {
        let lines: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();
        let mut values = Vec::with_capacity(lines.len());
        for (index, lexical) in lines.into_iter().enumerate() {
            let id = self
                .values
                .get(index)
                .map_or_else(|| value_id(&self.id, index), |value| value.id.clone());
            values.push(RunSetValue::new(id, lexical, self.kind, revision));
        }
        self.values = values;
    }

    /// One authored value per line, the form the editor round-trips.
    #[must_use]
    pub fn values_text(&self) -> String {
        self.values
            .iter()
            .map(|value| value.lexical.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Values that parsed, in order.
    #[must_use]
    pub fn canonical_values(&self) -> Vec<f64> {
        self.values
            .iter()
            .filter_map(|value| value.canonical)
            .collect()
    }
}

/// Deterministic value identity: `<dimension>-value-001`.
#[must_use]
pub fn value_id(dimension_id: &str, index: usize) -> String {
    format!("{dimension_id}-value-{:03}", index + 1)
}

/// Limits a preview is checked against before anything is dispatched.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RunSetBudgets {
    /// Maximum tasks the composed space may expand to.
    pub maximum_tasks: usize,
    /// Maximum bytes the run may store.
    pub maximum_storage_bytes: u64,
    /// Modelled solve cost of one task, in milliseconds.
    pub cost_per_point_ms: u64,
    /// Modelled stored bytes of one task.
    pub bytes_per_point: u64,
}

impl Default for RunSetBudgets {
    fn default() -> Self {
        Self {
            maximum_tasks: 1_000_000,
            maximum_storage_bytes: 10 * 1024 * 1024 * 1024,
            cost_per_point_ms: 250,
            bytes_per_point: 4 * 1024 * 1024,
        }
    }
}

/// Deterministic adaptive proposal contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunSetAdaptivePolicy {
    pub id: String,
    pub objective: String,
    pub seed: u64,
    /// Typed bounds in canonical JSON. They remain text here so equality and
    /// digests preserve the exact frozen declaration.
    pub bounds: String,
    pub stop_rule: String,
    pub maximum_proposals: usize,
}

/// The composition rule and its parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunSetComposition {
    pub mode: RunSetCompositionMode,
    /// Point identities removed from the cross product, read only by
    /// [`RunSetCompositionMode::Filtered`].
    ///
    /// A set rather than a list: excluding a point twice is the same space, and
    /// the ordered form makes the declaration byte-identical however the
    /// exclusions were authored. Each entry is a `RunSetPoint::point_key`,
    /// which is built from value identities rather than authored text so that
    /// retyping a value keeps the exclusion attached to it.
    #[serde(default)]
    pub excluded_points: std::collections::BTreeSet<String>,
    /// Typed conditional predicate. The executable grammar is intentionally
    /// compact: `<dimension-id> == <value>` clauses joined by `&&`.
    #[serde(default)]
    pub predicate: String,
    /// Dimensions the conditional predicate is authorized to read.
    #[serde(default)]
    pub upstream_dimension_ids: Vec<String>,
    /// Frozen proposal policy for adaptive composition.
    #[serde(default)]
    pub adaptive_policy: Option<RunSetAdaptivePolicy>,
    /// Maximum hierarchy depth for nested composition.
    #[serde(default = "default_nested_depth")]
    pub maximum_depth: u8,
}

const fn default_nested_depth() -> u8 {
    3
}

impl Default for RunSetComposition {
    fn default() -> Self {
        Self {
            mode: RunSetCompositionMode::default(),
            excluded_points: std::collections::BTreeSet::new(),
            predicate: String::new(),
            upstream_dimension_ids: Vec::new(),
            adaptive_policy: None,
            maximum_depth: default_nested_depth(),
        }
    }
}

impl RunSetComposition {
    /// The composition with a different rule and the same exclusions.
    #[must_use]
    pub fn with_mode(&self, mode: RunSetCompositionMode) -> Self {
        Self {
            mode,
            excluded_points: self.excluded_points.clone(),
            predicate: self.predicate.clone(),
            upstream_dimension_ids: self.upstream_dimension_ids.clone(),
            adaptive_policy: self.adaptive_policy.clone(),
            maximum_depth: self.maximum_depth,
        }
    }

    /// Whether exclusions are being applied, as opposed to merely stored.
    ///
    /// Exclusions survive a switch to another mode so that returning to
    /// `Filtered` restores the space that was authored; only this predicate
    /// decides whether they are subtracted from the run.
    #[must_use]
    pub fn filters(&self) -> bool {
        self.mode == RunSetCompositionMode::Filtered && !self.excluded_points.is_empty()
    }
}

/// The complete run-set working state.
///
/// `revision` moves on every accepted mutation and is what a receipt reports;
/// `preview` is only ever set by an explicit validate-and-preview, so a stale
/// forecast can never be mistaken for a current one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSetState {
    /// Monotonic working revision.
    pub revision: u32,
    /// Next identity number handed to a new dimension.
    pub sequence: u32,
    /// Ordered dimensions.
    pub dimensions: Vec<RunSetDimension>,
    /// How they compose.
    pub composition: RunSetComposition,
    /// Execution limits.
    pub budgets: RunSetBudgets,
    /// The forecast frozen by the last successful preview.
    #[serde(default)]
    pub preview: Option<super::RunSetForecast>,
    /// Transaction receipts, oldest first. Session evidence: a receipt records
    /// what a user did in this sitting, not a property of the saved plan.
    #[serde(skip)]
    pub receipts: Vec<super::RunSetReceipt>,
    /// Undo stack of editable snapshots.
    #[serde(skip)]
    pub history: Vec<RunSetSnapshot>,
    /// Redo stack of editable snapshots.
    #[serde(skip)]
    pub future: Vec<RunSetSnapshot>,
}

/// The part of the state an undo restores. Receipts are evidence of what
/// happened and are deliberately outside it: undoing an edit does not unmake
/// the record that the edit was made.
#[derive(Debug, Clone, PartialEq)]
pub struct RunSetSnapshot {
    pub revision: u32,
    pub sequence: u32,
    pub dimensions: Vec<RunSetDimension>,
    pub composition: RunSetComposition,
    pub budgets: RunSetBudgets,
    pub preview: Option<super::RunSetForecast>,
}

impl Default for RunSetState {
    /// The commercial PVT space: the three speed corners across a ±10 % supply
    /// and the mil-spec temperature range. This is the same default the corner
    /// analysis has always started from, stated as declared axes.
    fn default() -> Self {
        Self {
            revision: 1,
            sequence: 5,
            dimensions: vec![
                RunSetDimension::new(
                    "dimension-process",
                    RunSetDimensionKind::ProcessSection,
                    &["SS", "TT", "FF"],
                    1,
                ),
                RunSetDimension::new(
                    "dimension-supply",
                    RunSetDimensionKind::Supply,
                    &["0.9", "1.0", "1.1"],
                    1,
                ),
                RunSetDimension::new(
                    "dimension-temperature",
                    RunSetDimensionKind::Temperature,
                    &["-40", "25", "125"],
                    1,
                ),
                {
                    let mut dimension = RunSetDimension::new(
                        "dimension-cload",
                        RunSetDimensionKind::Parameter,
                        &["1p", "10p", "100p"],
                        1,
                    );
                    dimension.name = "Load capacitance".to_owned();
                    dimension.source = "design-variable:CLOAD".to_owned();
                    dimension.enabled = false;
                    dimension
                },
            ],
            composition: RunSetComposition::default(),
            budgets: RunSetBudgets::default(),
            preview: None,
            receipts: Vec::new(),
            history: Vec::new(),
            future: Vec::new(),
        }
    }
}

impl RunSetState {
    /// A fresh plan's runnable reference point. The conventional PVT values
    /// remain authored as ready-to-enable templates, but no process, supply,
    /// or temperature sweep is activated implicitly.
    #[must_use]
    pub fn reference_only() -> Self {
        let mut state = Self::default();
        for dimension in &mut state.dimensions {
            dimension.enabled = false;
        }
        state
    }

    /// Dimensions that contribute to the space.
    pub fn enabled_dimensions(&self) -> impl Iterator<Item = &RunSetDimension> {
        self.dimensions.iter().filter(|dimension| dimension.enabled)
    }

    /// The enabled dimension of a kind, if the run set declares one.
    #[must_use]
    pub fn enabled_dimension_of(&self, kind: RunSetDimensionKind) -> Option<&RunSetDimension> {
        self.enabled_dimensions()
            .find(|dimension| dimension.kind == kind)
    }

    /// Index of a dimension by identity.
    #[must_use]
    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.dimensions
            .iter()
            .position(|dimension| dimension.id == id)
    }

    /// A dimension by identity.
    #[must_use]
    pub fn dimension(&self, id: &str) -> Option<&RunSetDimension> {
        self.dimensions.iter().find(|dimension| dimension.id == id)
    }

    /// Kinds that are not yet declared, and so may still be added.
    #[must_use]
    pub fn addable_kinds(&self) -> Vec<RunSetDimensionKind> {
        RunSetDimensionKind::ALL
            .into_iter()
            .filter(|kind| {
                !self
                    .dimensions
                    .iter()
                    .any(|dimension| dimension.kind == *kind)
            })
            .collect()
    }

    /// Capture the editable state for the undo stack.
    #[must_use]
    pub(super) fn snapshot(&self) -> RunSetSnapshot {
        RunSetSnapshot {
            revision: self.revision,
            sequence: self.sequence,
            dimensions: self.dimensions.clone(),
            composition: self.composition.clone(),
            budgets: self.budgets,
            preview: self.preview,
        }
    }

    /// Restore an editable snapshot, leaving the receipt log untouched.
    pub(super) fn restore(&mut self, snapshot: RunSetSnapshot) {
        self.revision = snapshot.revision;
        self.sequence = snapshot.sequence;
        self.dimensions = snapshot.dimensions;
        self.composition = snapshot.composition;
        self.budgets = snapshot.budgets;
        self.preview = snapshot.preview;
    }
}
