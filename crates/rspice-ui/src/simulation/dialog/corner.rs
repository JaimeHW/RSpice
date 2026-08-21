//! Corner Analysis Configuration
//!
//! Configuration for PVT (Process, Voltage, Temperature) corner analysis.
//! Corner analysis runs simulations across multiple operating conditions
//! to verify design robustness.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Standard process corners (TT, SS, FF, SF, FS)
//! - Voltage corner sweep
//! - Temperature corner sweep
//! - Full matrix or diagonal sweep modes
//! - Summary statistics and worst-case identification
//!
//! # Example Usage
//!
//! Run transient analysis at SS corner with reduced voltage and hot temperature.

use crate::product::ProcessCorner;
use crate::simulation::run_set::RunSetState;

// =============================================================================
// Base Analysis Type
// =============================================================================

/// Base analysis to run at each corner
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CornerBaseAnalysis {
    /// Transient analysis
    #[default]
    Transient,
    /// AC analysis
    Ac,
    /// DC analysis
    Dc,
    /// Operating point
    Op,
}

// =============================================================================
// Corner Configuration
// =============================================================================

/// One explicitly declared point of a filtered space.
///
/// A filtered run set is not an axis product, so it cannot be carried as one.
/// Every point states all three quantities: an axis the run set left undeclared
/// has already been resolved against the plan's reference by the time a point
/// reaches here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerPointSpec {
    pub process: ProcessCorner,
    pub voltage: f64,
    pub temperature_celsius: f64,
}

/// Corner analysis configuration
#[derive(Debug, Clone)]
pub struct CornerConfig {
    /// Process corners to simulate
    pub process_corners: Vec<ProcessCorner>,
    /// Voltage values to sweep (V)
    pub voltages: Vec<f64>,
    /// Exact independent voltage-source instances that form the swept supply
    /// domain. Empty is valid only while no supply axis is enabled.
    pub supply_source_names: Vec<String>,
    /// Temperature values to sweep (°C)
    ///
    /// Always Celsius. The run configuration, the engine's `TEMP` option and
    /// every result label agree on that one unit, so there is nothing here to
    /// select between.
    pub temperatures: Vec<f64>,
    /// Full matrix (all combinations) or diagonal sweep
    pub full_matrix: bool,
    /// The exact points to run, when the space is not an axis composition.
    ///
    /// Empty means the axes above compose as `full_matrix` says. Non-empty
    /// replaces that composition entirely: it is the list the run set resolved,
    /// and re-deriving it from the axes would re-admit the points a filtered
    /// space removed. The axes stay populated with the distinct values the
    /// points use, because the process axis still selects which model sections
    /// are materialized.
    pub points: Vec<CornerPointSpec>,
    /// Base analysis type
    pub base_analysis: CornerBaseAnalysis,
}

impl Default for CornerConfig {
    fn default() -> Self {
        Self {
            process_corners: vec![ProcessCorner::TT],
            voltages: vec![1.0],
            supply_source_names: Vec::new(),
            temperatures: vec![25.0],
            full_matrix: true,
            points: Vec::new(),
            base_analysis: CornerBaseAnalysis::Transient,
        }
    }
}

impl CornerConfig {
    /// Create typical commercial PVT setup
    pub fn commercial_pvt() -> Self {
        Self {
            process_corners: ProcessCorner::speed_corners(),
            voltages: vec![0.9, 1.0, 1.1],          // ±10%
            temperatures: vec![-40.0, 25.0, 125.0], // Mil-spec range
            full_matrix: true,
            ..Default::default()
        }
    }

    /// Total number of corners
    pub fn num_corners(&self) -> usize {
        if !self.points.is_empty() {
            return self.points.len();
        }
        if self.full_matrix {
            self.process_corners.len() * self.voltages.len() * self.temperatures.len()
        } else {
            self.process_corners
                .len()
                .max(self.voltages.len())
                .max(self.temperatures.len())
        }
    }

    /// Generate corner names
    pub fn corner_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        if !self.points.is_empty() {
            return self
                .points
                .iter()
                .map(|point| {
                    format!(
                        "{}_{:.2}V_{:.0}C",
                        point.process.short_name(),
                        point.voltage,
                        point.temperature_celsius
                    )
                })
                .collect();
        }

        if self.full_matrix {
            for p in &self.process_corners {
                for v in &self.voltages {
                    for t in &self.temperatures {
                        names.push(format!("{}_{:.2}V_{:.0}C", p.short_name(), v, t));
                    }
                }
            }
        } else {
            let n = self.num_corners();
            for i in 0..n {
                let p = self
                    .process_corners
                    .get(i % self.process_corners.len())
                    .copied()
                    .unwrap_or(ProcessCorner::TT);
                let v = self
                    .voltages
                    .get(i % self.voltages.len())
                    .copied()
                    .unwrap_or(1.0);
                let t = self
                    .temperatures
                    .get(i % self.temperatures.len())
                    .copied()
                    .unwrap_or(25.0);
                names.push(format!("{}_{:.2}V_{:.0}C", p.short_name(), v, t));
            }
        }

        names
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.process_corners.is_empty() {
            return Err("At least one process corner required".to_string());
        }

        if self.voltages.is_empty() {
            return Err("At least one voltage value required".to_string());
        }

        for v in &self.voltages {
            if *v <= 0.0 {
                return Err("Voltage values must be positive".to_string());
            }
        }

        let mut distinct_supply_values: Vec<u64> = if self.points.is_empty() {
            self.voltages.iter().map(|value| value.to_bits()).collect()
        } else {
            self.points
                .iter()
                .map(|point| point.voltage.to_bits())
                .collect()
        };
        distinct_supply_values.sort_unstable();
        distinct_supply_values.dedup();
        if distinct_supply_values.len() > 1 && self.supply_source_names.is_empty() {
            return Err(
                "Voltage sweeping requires an explicit netlist supply-source binding".to_owned(),
            );
        }
        let mut seen_supply_sources = std::collections::BTreeSet::new();
        for source in &self.supply_source_names {
            if source.trim().is_empty()
                || source != source.trim()
                || source.chars().any(char::is_control)
                || !seen_supply_sources.insert(source.to_ascii_lowercase())
            {
                return Err(format!(
                    "Supply source binding {source:?} is empty, malformed, or duplicated"
                ));
            }
        }

        if self.temperatures.is_empty() {
            return Err("At least one temperature value required".to_string());
        }

        for t in &self.temperatures {
            if *t < -273.15 {
                return Err("Temperature cannot be below absolute zero".to_string());
            }
        }

        for point in &self.points {
            if !point.voltage.is_finite() || point.voltage <= 0.0 {
                return Err("Explicit corner points require positive voltages".to_string());
            }
            if !point.temperature_celsius.is_finite() || point.temperature_celsius < -273.15 {
                return Err(
                    "Explicit corner point temperature cannot be below absolute zero".to_string(),
                );
            }
            // The process axis is what selects the model section a point is
            // materialized against, so a point naming a process the axis omits
            // would run against whichever section happened to be loaded.
            if !self.process_corners.contains(&point.process) {
                return Err(format!(
                    "Explicit corner point uses process {} which the process axis does not declare",
                    point.process.short_name()
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Dialog State
// =============================================================================

/// The corner analysis's authored state.
///
/// The run space is *not* here. It is declared once, by the plan — PVT, sweeps
/// & variation owns it — and this analysis reads that declaration. A corner
/// form that also stored a space would be a second owner of one fact, and two
/// owners of a run space eventually disagree about how many points run; the
/// form used to render the embedded copy while claiming to state the plan's,
/// which is that disagreement already happening silently.
///
/// What remains here is the only thing the corner analysis actually owns: which
/// base analysis is repeated at every declared point.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(from = "CornerDialogStateRepr")]
pub struct CornerDialogState {
    /// Base analysis type index.
    pub base_analysis_idx: usize,
    /// The space a project authored back when this draft owned one.
    ///
    /// A migration alias, never an owner: it is only ever read out of a
    /// persisted project and is consumed exactly once by
    /// [`Self::adopt_legacy_run_set`]. It is deliberately not serialized, so
    /// saving a migrated project retires the second declaration for good.
    #[serde(skip)]
    pub legacy_run_set: Option<RunSetState>,
    /// Initialized flag.
    #[serde(skip)]
    pub initialized: bool,
}

/// Wire form of [`CornerDialogState`].
///
/// Projects authored before the run set stored the space as three fixed
/// buffers. Those keys are still read here and migrated exactly once, so an
/// existing plan opens on the space it was saved with instead of silently
/// reverting to the default PVT matrix. They are never written back.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CornerDialogStateRepr {
    #[serde(default)]
    run_set: Option<RunSetState>,
    #[serde(default)]
    base_analysis_idx: usize,
    #[serde(default)]
    process_tt: bool,
    #[serde(default)]
    process_ss: bool,
    #[serde(default)]
    process_ff: bool,
    #[serde(default)]
    process_sf: bool,
    #[serde(default)]
    process_fs: bool,
    #[serde(default)]
    voltage_min: String,
    #[serde(default)]
    voltage_nom: String,
    #[serde(default)]
    voltage_max: String,
    #[serde(default)]
    enable_voltage_sweep: bool,
    #[serde(default)]
    temp_cold: String,
    #[serde(default)]
    temp_room: String,
    #[serde(default)]
    temp_hot: String,
    #[serde(default)]
    enable_temp_sweep: bool,
    #[serde(default)]
    full_matrix: bool,
}

impl From<CornerDialogStateRepr> for CornerDialogState {
    fn from(mut repr: CornerDialogStateRepr) -> Self {
        let legacy_run_set = repr.run_set.take().or_else(|| migrate_legacy_space(&repr));
        Self {
            base_analysis_idx: repr.base_analysis_idx,
            legacy_run_set,
            initialized: true,
        }
    }
}

/// Rebuild the declared space from the pre-run-set buffers.
///
/// A value that no longer parses is carried across as written rather than
/// dropped: validation will refuse it by name, which is what a user needs to
/// see, and losing it would quietly shrink the space the plan was saved with.
///
/// `None` when the buffers hold nothing authored. That is a fresh draft rather
/// than a saved space, and handing back a default here would manufacture a
/// declaration to migrate — which, against an empty plan-global run set, would
/// enable three axes nobody enabled.
fn migrate_legacy_space(repr: &CornerDialogStateRepr) -> Option<RunSetState> {
    use crate::simulation::run_set::{RunSetDimension, RunSetDimensionKind};

    let sections: Vec<&str> = [
        (repr.process_tt, "TT"),
        (repr.process_ss, "SS"),
        (repr.process_ff, "FF"),
        (repr.process_sf, "SF"),
        (repr.process_fs, "FS"),
    ]
    .into_iter()
    .filter_map(|(selected, name)| selected.then_some(name))
    .collect();
    if sections.is_empty() && repr.voltage_nom.is_empty() && repr.temp_room.is_empty() {
        // Nothing was authored: this is a fresh state, not a saved space.
        return None;
    }

    let supplies: Vec<&str> = if repr.enable_voltage_sweep {
        vec![
            repr.voltage_min.as_str(),
            repr.voltage_nom.as_str(),
            repr.voltage_max.as_str(),
        ]
    } else {
        vec![repr.voltage_nom.as_str()]
    };
    let temperatures: Vec<&str> = if repr.enable_temp_sweep {
        vec![
            repr.temp_cold.as_str(),
            repr.temp_room.as_str(),
            repr.temp_hot.as_str(),
        ]
    } else {
        vec![repr.temp_room.as_str()]
    };

    let mut supply = RunSetDimension::new(
        "dimension-supply",
        RunSetDimensionKind::Supply,
        &supplies,
        1,
    );
    supply.enabled = repr.enable_voltage_sweep;

    let mut state = RunSetState::default();
    state.composition.mode = if repr.full_matrix {
        crate::simulation::run_set::RunSetCompositionMode::Cartesian
    } else {
        crate::simulation::run_set::RunSetCompositionMode::Zipped
    };
    state.dimensions = vec![
        RunSetDimension::new(
            "dimension-process",
            RunSetDimensionKind::ProcessSection,
            &sections,
            1,
        ),
        supply,
        RunSetDimension::new(
            "dimension-temperature",
            RunSetDimensionKind::Temperature,
            &temperatures,
            1,
        ),
    ];
    Some(state)
}

impl CornerDialogState {
    /// Initialize from config.
    ///
    /// Only the base analysis is read: the space the configuration carries
    /// belongs to the plan, and adopting it here would re-create the second
    /// owner this type just stopped being.
    pub fn from_config(config: &CornerConfig) -> Self {
        Self {
            base_analysis_idx: match config.base_analysis {
                CornerBaseAnalysis::Transient => 0,
                CornerBaseAnalysis::Ac => 1,
                CornerBaseAnalysis::Dc => 2,
                CornerBaseAnalysis::Op => 3,
            },
            legacy_run_set: None,
            initialized: true,
        }
    }

    /// The base analysis executed at every point.
    pub fn base_analysis(&self) -> CornerBaseAnalysis {
        match self.base_analysis_idx {
            0 => CornerBaseAnalysis::Transient,
            1 => CornerBaseAnalysis::Ac,
            2 => CornerBaseAnalysis::Dc,
            _ => CornerBaseAnalysis::Op,
        }
    }

    /// Convert to config against the plan's declared space and nominal point.
    ///
    /// Both are passed in rather than stored. The space is the plan's one
    /// declaration, and the reference answers any axis that declaration leaves
    /// undeclared — the plan owns each of them, so neither is copied here.
    pub fn to_config(
        &self,
        run_set: &RunSetState,
        reference: crate::simulation::run_set::ReferencePoint,
    ) -> Result<CornerConfig, String> {
        run_set.to_corner_config(self.base_analysis(), reference)
    }

    /// Fold a project's pre-unification corner space into the plan-global one.
    ///
    /// Consumes the alias whatever the outcome, so this runs exactly once per
    /// load and a saved project carries a single declaration afterwards.
    pub fn adopt_legacy_run_set(&mut self, global: &mut RunSetState) -> CornerRunSetMigration {
        let Some(legacy) = self.legacy_run_set.take() else {
            return CornerRunSetMigration::Nothing;
        };
        if declared_axes(&legacy).is_empty() {
            // The draft declared nothing executable. There is no space to carry
            // over, and adopting it would replace the plan's declaration with
            // an empty one.
            return CornerRunSetMigration::Nothing;
        }
        if declared_axes(global).is_empty() {
            // The plan never declared a space of its own, so the one the
            // project actually ran on is the honest thing to keep.
            let adopted = legacy.point_count();
            *global = legacy;
            return CornerRunSetMigration::Adopted {
                point_count: adopted,
            };
        }
        if declared_axes(global) == declared_axes(&legacy) {
            return CornerRunSetMigration::Agreed;
        }
        // Two declarations that disagree. The plan-global one wins because it
        // is the one the Run Set page edits and every other analysis already
        // executes across; silently preferring the draft would move a plan onto
        // a space its own page never showed. What the draft declared is
        // reported rather than discarded, so the loss is visible and can be
        // re-authored.
        CornerRunSetMigration::Dropped {
            dropped_axes: declared_axes(&legacy),
            dropped_point_count: legacy.point_count(),
        }
    }

    /// Ensure initialized.
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            let base = Self::from_config(&CornerConfig::commercial_pvt());
            self.base_analysis_idx = base.base_analysis_idx;
            self.initialized = true;
        }
    }
}

/// One enabled axis of a declared space, as `kind=value,value`.
///
/// Comparison is over what the space *executes* — kind, order and authored
/// values — rather than over the whole state, because revision counters, undo
/// stacks and a frozen preview differ between two declarations that run
/// identically, and a migration that called those a disagreement would report a
/// loss that did not happen.
fn declared_axes(state: &RunSetState) -> Vec<String> {
    state
        .enabled_dimensions()
        .map(|dimension| {
            format!(
                "{}={}",
                dimension.kind.as_str(),
                dimension
                    .values
                    .iter()
                    .map(|value| value.lexical.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect()
}

/// What folding a legacy corner space into the plan-global one did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CornerRunSetMigration {
    /// No second declaration was carried, so nothing moved.
    Nothing,
    /// The plan declared no space, so the draft's became the plan's.
    Adopted { point_count: usize },
    /// Both declared the same space; the duplicate was simply retired.
    Agreed,
    /// Both declared a space and they differed. The plan-global declaration
    /// stands, and this is the one that no longer contributes.
    Dropped {
        dropped_axes: Vec<String>,
        dropped_point_count: usize,
    },
}

impl CornerRunSetMigration {
    /// What to tell the reader, or `None` when nothing needs saying.
    ///
    /// Only a dropped declaration is worth a sentence: adopting a space and
    /// retiring an identical duplicate both leave the plan running exactly what
    /// it ran before.
    #[must_use]
    pub fn dropped_declaration_note(&self) -> Option<String> {
        let Self::Dropped {
            dropped_axes,
            dropped_point_count,
        } = self
        else {
            return None;
        };
        Some(format!(
            "The Corner analysis carried its own run space ({}, {dropped_point_count} points). \
             PVT, sweeps & variation already declared a different one, which is what this plan \
             runs; the Corner copy has been retired. Re-author it there if it was the space you \
             wanted.",
            dropped_axes.join(" · "),
        ))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::run_set::{RunSetDimension, RunSetDimensionKind};

    /// A plan-global run set declaring one temperature axis.
    fn global_with(temperatures: &[&str]) -> RunSetState {
        let mut state = RunSetState::reference_only();
        state.dimensions = vec![RunSetDimension::new(
            "dimension-temperature",
            RunSetDimensionKind::Temperature,
            temperatures,
            1,
        )];
        state
    }

    /// A corner draft as an older project persisted it: its own space inline.
    fn draft_carrying(temperatures: &[&str]) -> CornerDialogState {
        let persisted = serde_json::json!({
            "run_set": global_with(temperatures),
            "base_analysis_idx": 0,
        });
        serde_json::from_value(persisted).expect("a legacy corner draft decodes")
    }

    #[test]
    fn an_empty_plan_adopts_the_space_the_project_actually_ran_on() {
        let mut draft = draft_carrying(&["-40", "25", "125"]);
        // A fresh plan declares no axes at all, which is what every project
        // saved before the Run Set page existed loads with.
        let mut global = RunSetState::reference_only();

        let migration = draft.adopt_legacy_run_set(&mut global);

        assert_eq!(migration, CornerRunSetMigration::Adopted { point_count: 3 });
        assert_eq!(global.point_count(), 3, "the saved space survives the load");
        assert!(
            draft.legacy_run_set.is_none(),
            "the alias is consumed, so a second load cannot re-adopt it"
        );
        assert!(migration.dropped_declaration_note().is_none());
    }

    #[test]
    fn two_declarations_that_agree_retire_the_duplicate_silently() {
        let mut draft = draft_carrying(&["-40", "25", "125"]);
        let mut global = global_with(&["-40", "25", "125"]);

        let migration = draft.adopt_legacy_run_set(&mut global);

        assert_eq!(migration, CornerRunSetMigration::Agreed);
        assert_eq!(global.point_count(), 3, "the plan runs what it always ran");
        assert!(
            migration.dropped_declaration_note().is_none(),
            "nothing was lost, so there is nothing to report"
        );
    }

    #[test]
    fn when_the_two_declarations_disagree_the_plan_global_one_wins_and_says_what_it_dropped() {
        let mut draft = draft_carrying(&["0", "85"]);
        let mut global = global_with(&["-40", "25", "125"]);

        let migration = draft.adopt_legacy_run_set(&mut global);

        assert_eq!(
            migration,
            CornerRunSetMigration::Dropped {
                dropped_axes: vec!["temperature=0,85".to_owned()],
                dropped_point_count: 2,
            },
            "the corner declaration is reported, not silently discarded"
        );
        assert_eq!(
            global.point_count(),
            3,
            "the plan-global declaration is the one that stands"
        );
        let note = migration
            .dropped_declaration_note()
            .expect("a dropped declaration is told to the reader");
        assert!(note.contains("temperature=0,85"), "{note}");
        assert!(note.contains("2 points"), "{note}");
    }

    #[test]
    fn a_draft_that_never_declared_a_space_migrates_nothing() {
        let mut draft = CornerDialogState::default();
        let mut global = global_with(&["-40", "25"]);

        assert_eq!(
            draft.adopt_legacy_run_set(&mut global),
            CornerRunSetMigration::Nothing
        );
        assert_eq!(global.point_count(), 2, "the plan is left alone");
    }

    #[test]
    fn the_retired_space_is_never_written_back() {
        let draft = draft_carrying(&["-40", "25", "125"]);

        let encoded = serde_json::to_value(&draft).expect("a corner draft encodes");

        assert!(
            encoded.get("run_set").is_none(),
            "saving a migrated project must retire the second declaration: {encoded}"
        );
    }

    #[test]
    fn the_pre_run_set_buffers_still_migrate_into_the_alias() {
        // The oldest shape of all: three fixed buffers, no run set at all.
        let persisted = r#"{
            "base_analysis_idx": 0,
            "process_tt": true,
            "process_ss": true,
            "voltage_nom": "1.0",
            "temp_room": "25",
            "enable_temp_sweep": true,
            "temp_cold": "-40",
            "temp_hot": "125",
            "full_matrix": true
        }"#;

        let mut draft: CornerDialogState =
            serde_json::from_str(persisted).expect("the oldest corner draft decodes");
        let mut global = RunSetState::reference_only();

        let migration = draft.adopt_legacy_run_set(&mut global);

        assert!(
            matches!(migration, CornerRunSetMigration::Adopted { .. }),
            "{migration:?}"
        );
        assert_eq!(
            global
                .enabled_dimension_of(RunSetDimensionKind::Temperature)
                .expect("the temperature axis carried across")
                .canonical_values(),
            vec![-40.0, 25.0, 125.0]
        );
    }
}
