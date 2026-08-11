//! The typed run-set state: dimensions, their values, and how they compose.
//!
//! A run set is the plan's declaration of *what space is executed*. It is one
//! ordered list of enabled dimensions, a composition rule that turns them into
//! points, and the budgets a preview is checked against. Every value carries a
//! stable identity and the revision it came from, so a point in a manifest can
//! be traced back to the exact declaration that produced it.
//!
//! Only kinds the engine can bind are representable. A dimension the solver
//! could not execute would be a control that validates and persists and then
//! silently does nothing, which is the one failure this module exists to make
//! impossible.

use serde::{Deserialize, Serialize};

/// What a dimension varies, and therefore how it reaches the engine.
///
/// Each variant names an execution binding that exists. `ProcessSection`
/// selects the model-library section every device resolves through;
/// `Supply` scales the netlist's DC supply; `Temperature` sets the solve
/// temperature. Together they are the three axes the corner executor runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunSetDimensionKind {
    /// PDK/library section: `tt`, `ss`, `ff`, `sf`, `fs`.
    ProcessSection,
    /// Supply voltage applied to the design's DC sources.
    Supply,
    /// Ambient and model-evaluation temperature, in °C.
    Temperature,
}

impl RunSetDimensionKind {
    /// Every kind, in the order a fresh run set declares them.
    pub const ALL: [Self; 3] = [Self::ProcessSection, Self::Supply, Self::Temperature];

    /// Stable lexical name, matching the run-set contract vocabulary.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProcessSection => "process-section",
            Self::Supply => "supply",
            Self::Temperature => "temperature",
        }
    }

    /// The value type a dimension of this kind carries. The kind decides it:
    /// a process section is a reference into a library, a supply and a
    /// temperature are physical quantities.
    #[must_use]
    pub fn value_type(self) -> RunSetValueType {
        match self {
            Self::ProcessSection => RunSetValueType::Reference,
            Self::Supply | Self::Temperature => RunSetValueType::Quantity,
        }
    }

    /// Canonical storage unit, or `None` when the values are references.
    #[must_use]
    pub fn unit(self) -> Option<&'static str> {
        match self {
            Self::ProcessSection => None,
            Self::Supply => Some("V"),
            Self::Temperature => Some("°C"),
        }
    }

    /// Where the values are authored, shown as the dimension's authority.
    #[must_use]
    pub fn source(self) -> &'static str {
        match self {
            Self::ProcessSection => "model-library:process-sections",
            Self::Supply => "testbench:supply-sources",
            Self::Temperature => "run-set:temperature-axis",
        }
    }

    /// A dimension of this kind may appear at most once: the engine binds one
    /// process section, one supply scaling and one temperature per point, so a
    /// second dimension of the same kind would have no distinct binding.
    #[must_use]
    pub fn default_name(self) -> &'static str {
        match self {
            Self::ProcessSection => "Process section",
            Self::Supply => "Supply voltage",
            Self::Temperature => "Temperature",
        }
    }
}

/// The type every value of a dimension is parsed as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunSetValueType {
    /// A physical quantity in the dimension's canonical unit.
    Quantity,
    /// A name resolved against an external authority.
    Reference,
}

impl RunSetValueType {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Quantity => "quantity",
            Self::Reference => "reference",
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
}

impl RunSetCompositionMode {
    pub const ALL: [Self; 2] = [Self::Cartesian, Self::Zipped];

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cartesian => "cartesian",
            Self::Zipped => "zipped",
        }
    }

    /// Glyph drawn between adjacent axis cards.
    #[must_use]
    pub fn operator(self) -> &'static str {
        match self {
            Self::Cartesian => "×",
            Self::Zipped => "⇅",
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
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Cartesian => "Cartesian · every combination",
            Self::Zipped => "Zipped · index-aligned",
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

/// The composition rule and its parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RunSetComposition {
    pub mode: RunSetCompositionMode,
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
            sequence: 4,
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
            composition: self.composition,
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
