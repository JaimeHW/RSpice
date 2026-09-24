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

use super::{
    ReferencePoint, RunSetBudgets, RunSetComposition, RunSetDimension, RunSetDimensionKind,
    RunSetForecast, RunSetReceipt,
};

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
    pub preview: Option<RunSetForecast>,
    /// Transaction receipts, oldest first. Session evidence: a receipt records
    /// what a user did in this sitting, not a property of the saved plan.
    #[serde(skip)]
    pub receipts: Vec<RunSetReceipt>,
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
    pub preview: Option<RunSetForecast>,
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

    /// Kinds that may still be added.
    ///
    /// Parameter and source axes are repeatable because their authority names
    /// distinguish independent bindings. Singleton environment kinds remain
    /// unavailable once declared, even while their existing card is disabled.
    #[must_use]
    pub fn addable_kinds(&self) -> Vec<RunSetDimensionKind> {
        RunSetDimensionKind::ALL
            .into_iter()
            .filter(|kind| {
                kind.allows_multiple_authorities()
                    || !self
                        .dimensions
                        .iter()
                        .any(|dimension| dimension.kind == *kind)
            })
            .collect()
    }

    /// The temperatures this plan's axis declares, whether or not the axis is
    /// enabled, or the reference temperature when it declares none.
    ///
    /// Deliberately not [`Self::enabled_dimension_of`]. Enabling an axis says
    /// "cross the whole plan by this"; it does not decide which temperatures
    /// the plan considers meaningful. A qualification programme names its
    /// temperatures once, and an analysis that inherits them wants that list
    /// even when the operator has chosen not to run every analysis across it.
    ///
    /// An axis whose values do not all parse yields `None` rather than a
    /// shortened list: silently dropping a value would run a narrower sweep
    /// than the one declared, and validation already names the bad value.
    #[must_use]
    pub fn declared_temperatures_celsius(&self, reference: ReferencePoint) -> Option<Vec<f64>> {
        let Some(dimension) = self
            .dimensions
            .iter()
            .find(|dimension| dimension.kind == RunSetDimensionKind::Temperature)
            .filter(|dimension| !dimension.values.is_empty())
        else {
            return Some(vec![reference.temperature_celsius]);
        };
        dimension
            .values
            .iter()
            .map(|value| value.canonical)
            .collect::<Option<Vec<f64>>>()
    }

    /// How many points the declared space expands to.
    ///
    /// Derived from the same point-count rule the forecast uses, without
    /// allocating validation findings for callers that only need the size.
    #[must_use]
    pub fn point_count(&self) -> usize {
        super::forecast_point_count(self)
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
