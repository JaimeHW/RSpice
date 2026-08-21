//! The run set: the space a simulation plan executes.
//!
//! A plan says *what* is analysed; the run set says *where* — which process
//! section, which supply, which temperature, and how those axes compose into
//! points. It is a transactional working state: edits move a revision and
//! leave receipts, a preview freezes a forecast that any later edit clears, and
//! nothing is dispatched against a space that has not validated.
//!
//! The model deliberately admits only dimension kinds the executor binds. A
//! dimension that rendered, validated and persisted without reaching the solver
//! would be indistinguishable from one that worked, which is the failure this
//! module is shaped to prevent.

mod model;
mod participation;
mod points;
mod transaction;
mod validate;

#[cfg(test)]
mod tests;

pub use model::{
    InvalidValuePolicy, RunSetAdaptivePolicy, RunSetBudgets, RunSetCompositionMode,
    RunSetDimension, RunSetDimensionKind, RunSetState, parse_parameter_source_authority,
    parse_source_value_authority, parse_supply_source_authority,
};
#[cfg(test)]
pub use model::{NETLIST_SUPPLY_SOURCE_PREFIX, RunSetComposition};
pub use participation::{AnalysisRunAt, nominal_point_key, participating_point_keys};
pub use points::{RunSetPoint, compose, resolve};
#[cfg(test)]
pub use transaction::dispatch;
pub use transaction::{RunSetAction, RunSetReceipt, RunSetReceiptStatus, dispatch_for_plan};
#[cfg(test)]
pub use validate::validate_with_task_count;
pub use validate::{RunSetForecast, RunSetStatus, RunSetValidation, validate, validate_for_plan};

use crate::product::ProcessCorner;
use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, CornerPointSpec};

/// Storage sizes in the binary units the budgets are authored in.
#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    const KIB: u64 = 1024;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// Parse a storage budget written in engineering units.
///
/// The field round-trips [`format_bytes`], so its own output must parse back to
/// the same number; decimal suffixes are accepted because vendor quotas are
/// written that way.
pub fn parse_bytes(text: &str) -> Result<u64, String> {
    let trimmed = text.trim();
    let (magnitude, scale) = [
        ("TiB", 1024u64.pow(4)),
        ("GiB", 1024u64.pow(3)),
        ("MiB", 1024u64.pow(2)),
        ("KiB", 1024),
        ("TB", 1_000_000_000_000),
        ("GB", 1_000_000_000),
        ("MB", 1_000_000),
        ("kB", 1_000),
        ("B", 1),
    ]
    .into_iter()
    .find_map(|(suffix, scale)| {
        trimmed
            .strip_suffix(suffix)
            .or_else(|| {
                trimmed
                    .len()
                    .checked_sub(suffix.len())
                    .filter(|split| trimmed.is_char_boundary(*split))
                    .filter(|split| trimmed[*split..].eq_ignore_ascii_case(suffix))
                    .map(|split| &trimmed[..split])
            })
            .map(|magnitude| (magnitude, scale))
    })
    .unwrap_or((trimmed, 1));
    let magnitude: f64 = magnitude
        .trim()
        .replace([',', '_'], "")
        .parse()
        .map_err(|_| format!("{text:?} is not a storage size"))?;
    if !magnitude.is_finite() || magnitude < 0.0 {
        return Err(format!("{text:?} is not a storage size"));
    }
    Ok((magnitude * scale as f64).round() as u64)
}

/// The modelled wall-clock cost of a queue of `task_count` tasks.
///
/// The one owner of the tasks-to-duration arithmetic. Duration is not an
/// independent estimate: it is the task count priced at the run set's own
/// per-task budget, so a surface that states a duration is restating the queue
/// it already promised. Everything that shows one — the Run Set forecast tile,
/// the resolved point table, the preview receipt, the preflight Execution cell
/// and the per-analysis task-rate table — multiplies here rather than locally,
/// which is what makes a duration that disagrees with its own task count
/// inexpressible.
///
/// Saturating rather than checked: a task count that overflows the budget has
/// already been refused by [`RunSetBudgets::maximum_tasks`], and a `None`
/// duration on a surface that did state a task count reads as "unknown" when
/// the truth is "longer than anyone will wait".
#[must_use]
pub const fn modelled_cost_ms(task_count: usize, cost_per_task_ms: u64) -> u64 {
    (task_count as u64).saturating_mul(cost_per_task_ms)
}

/// A duration in the form the forecast tile shows.
#[must_use]
pub fn format_duration_ms(milliseconds: u64) -> String {
    let seconds = milliseconds as f64 / 1000.0;
    if seconds < 60.0 {
        format!("{seconds:.2} s")
    } else if seconds < 3600.0 {
        format!("{:.0} m {:02.0} s", seconds / 60.0, seconds % 60.0)
    } else {
        format!(
            "{:.0} h {:02.0} m",
            seconds / 3600.0,
            (seconds % 3600.0) / 60.0
        )
    }
}

impl RunSetState {
    /// Derive the executable corner configuration this run set declares.
    ///
    /// Every enabled dimension binds to one axis of the corner executor, so the
    /// space shown on the page and the space the engine expands are the same
    /// declaration read twice rather than two configurations kept in step. A
    /// dimension that is absent contributes exactly one point: the run still
    /// happens, at the deck's own value for that quantity.
    ///
    /// A filtered space cannot be stated as axes at all — that is the point of
    /// it — so it is carried as the resolved point list instead. The axes are
    /// still emitted, holding the distinct values those points use, because the
    /// process axis is what decides which model sections are materialized.
    pub fn to_corner_config(
        &self,
        base_analysis: CornerBaseAnalysis,
        reference: ReferencePoint,
    ) -> Result<CornerConfig, String> {
        let validation = validate(self, 1);
        if let Some(error) = validation.errors.first() {
            return Err(error.message.clone());
        }

        let process_corners = match self.enabled_dimension_of(RunSetDimensionKind::ProcessSection) {
            Some(dimension) => dimension
                .values
                .iter()
                .map(|value| {
                    model::process_section_index(&value.lexical)
                        .map(|index| PROCESS_CORNERS[index])
                        .ok_or_else(|| format!("{} is not a process section", value.lexical))
                })
                .collect::<Result<Vec<_>, _>>()?,
            // No process axis: every point resolves through the plan's
            // reference section, which is exactly one process corner.
            None => vec![reference.process],
        };

        // With no supply axis the ratio the executor applies is 1.0, so the
        // deck's own supply is used untouched. The value itself is arbitrary
        // and only has to match the nominal the executor divides by.
        let (voltages, supply_source_names) =
            match self.enabled_dimension_of(RunSetDimensionKind::Supply) {
                Some(dimension) => (
                    dimension.canonical_values(),
                    model::parse_supply_source_authority(&dimension.source)?,
                ),
                None => (vec![UNSWEPT_SUPPLY], Vec::new()),
            };

        let temperatures = match self.enabled_dimension_of(RunSetDimensionKind::Temperature) {
            Some(dimension) => dimension.canonical_values(),
            None => vec![reference.temperature_celsius],
        };

        let points = if self.composition.filters() {
            self.explicit_points(reference)?
        } else {
            Vec::new()
        };

        // An axis value every point excluded is a value the run never reaches,
        // and leaving it on the process axis would demand a PDK section for a
        // corner that is not executed. So a filtered space narrows its axes to
        // the values its points actually use.
        let config = if points.is_empty() {
            CornerConfig {
                process_corners,
                voltages,
                supply_source_names,
                temperatures,
                full_matrix: self.composition.mode != RunSetCompositionMode::Zipped,
                points,
                base_analysis,
            }
        } else {
            CornerConfig {
                process_corners: retain_used(&process_corners, &points, |point| point.process),
                voltages: retain_used(&voltages, &points, |point| point.voltage),
                supply_source_names,
                temperatures: retain_used(&temperatures, &points, |point| {
                    point.temperature_celsius
                }),
                full_matrix: true,
                points,
                base_analysis,
            }
        };
        config.validate()?;
        Ok(config)
    }

    /// The resolved points of a filtered space, each stated in full.
    ///
    /// An axis the run set does not declare has no coordinate on the point, so
    /// it is filled from the plan's reference here — the executor takes points,
    /// not partial ones, and leaving a hole would make the filtered path
    /// disagree with the axis path about what an undeclared axis means.
    fn explicit_points(&self, reference: ReferencePoint) -> Result<Vec<CornerPointSpec>, String> {
        let resolved = resolve(self).ok_or_else(|| {
            "The declared space does not expand exactly, so its points cannot be executed"
                .to_owned()
        })?;
        resolved
            .into_iter()
            .map(|point| {
                let mut spec = CornerPointSpec {
                    process: reference.process,
                    voltage: UNSWEPT_SUPPLY,
                    temperature_celsius: reference.temperature_celsius,
                };
                for (dimension, value) in &point.coordinates {
                    let canonical = value
                        .canonical
                        .ok_or_else(|| format!("{} is not a usable value", value.lexical))?;
                    match dimension.kind {
                        RunSetDimensionKind::ProcessSection => {
                            spec.process =
                                *PROCESS_CORNERS.get(canonical as usize).ok_or_else(|| {
                                    format!("{} is not a process section", value.lexical)
                                })?;
                        }
                        RunSetDimensionKind::Supply => spec.voltage = canonical,
                        RunSetDimensionKind::Temperature => spec.temperature_celsius = canonical,
                        // Non-PVT coordinates are materialized directly into
                        // the prepared task/deck. They do not alter the corner
                        // projection used by legacy corner services.
                        RunSetDimensionKind::Parameter
                        | RunSetDimensionKind::Source
                        | RunSetDimensionKind::Model
                        | RunSetDimensionKind::Frequency
                        | RunSetDimensionKind::Time
                        | RunSetDimensionKind::Seed
                        | RunSetDimensionKind::Sample
                        | RunSetDimensionKind::AnalysisSelection
                        | RunSetDimensionKind::DigitalConfiguration
                        | RunSetDimensionKind::ExternalDataset => {}
                    }
                }
                Ok(spec)
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
    /// Derived from the same validation the page reports, so a caller that only
    /// needs the size cannot arrive at a different one.
    #[must_use]
    pub fn point_count(&self) -> usize {
        validate(self, 1).forecast.point_count
    }

    /// Build a run set from an executable corner configuration.
    ///
    /// Test-only since the Corner draft stopped declaring a space of its own:
    /// nothing in the product now turns a `CornerConfig` back into a run set,
    /// because the run set is what produced it. It survives as the way a test
    /// states a space the way the engine sees it.
    ///
    /// It reads the axes only: an explicit point list is what a space looks
    /// like once it has stopped being an axis composition, and there is no axis
    /// form to recover it into.
    #[cfg(test)]
    #[must_use]
    pub fn from_corner_config(config: &CornerConfig) -> Self {
        let mut state = Self {
            revision: 1,
            sequence: 4,
            dimensions: Vec::new(),
            composition: RunSetComposition {
                mode: if config.full_matrix {
                    RunSetCompositionMode::Cartesian
                } else {
                    RunSetCompositionMode::Zipped
                },
                excluded_points: std::collections::BTreeSet::new(),
                ..RunSetComposition::default()
            },
            budgets: RunSetBudgets::default(),
            preview: None,
            receipts: Vec::new(),
            history: Vec::new(),
            future: Vec::new(),
        };

        let sections: Vec<String> = config
            .process_corners
            .iter()
            .map(|corner| corner.short_name().to_owned())
            .collect();
        state.dimensions.push(RunSetDimension::new(
            "dimension-process",
            RunSetDimensionKind::ProcessSection,
            &sections.iter().map(String::as_str).collect::<Vec<_>>(),
            1,
        ));

        let supplies: Vec<String> = config.voltages.iter().map(f64::to_string).collect();
        let mut supply = RunSetDimension::new(
            "dimension-supply",
            RunSetDimensionKind::Supply,
            &supplies.iter().map(String::as_str).collect::<Vec<_>>(),
            1,
        );
        if !config.supply_source_names.is_empty() {
            supply.source = format!(
                "{}{}",
                model::NETLIST_SUPPLY_SOURCE_PREFIX,
                config.supply_source_names.join(",")
            );
        }
        // A single supply value is not a sweep: it is the deck's own value, and
        // enabling an axis for it would report a dimension the run does not
        // actually vary.
        supply.enabled = config.voltages.len() > 1;
        state.dimensions.push(supply);

        let temperatures: Vec<String> = config.temperatures.iter().map(f64::to_string).collect();
        state.dimensions.push(RunSetDimension::new(
            "dimension-temperature",
            RunSetDimensionKind::Temperature,
            &temperatures.iter().map(String::as_str).collect::<Vec<_>>(),
            1,
        ));

        state
    }
}

/// The declared axis values at least one point uses, in declaration order.
fn retain_used<T: PartialEq + Copy>(
    declared: &[T],
    points: &[CornerPointSpec],
    coordinate: impl Fn(&CornerPointSpec) -> T,
) -> Vec<T> {
    declared
        .iter()
        .copied()
        .filter(|value| points.iter().any(|point| coordinate(point) == *value))
        .collect()
}

/// Process corners in the order [`PROCESS_SECTIONS`] names them.
const PROCESS_CORNERS: [ProcessCorner; 5] = [
    ProcessCorner::TT,
    ProcessCorner::SS,
    ProcessCorner::FF,
    ProcessCorner::SF,
    ProcessCorner::FS,
];

/// Placeholder supply used when no supply axis is declared. It is both the
/// swept value and the nominal it is divided by, so the executor's ratio is
/// exactly one.
const UNSWEPT_SUPPLY: f64 = 1.0;

/// The plan's nominal point.
///
/// A dimension the run set does not declare still has to resolve to something
/// the executor can run; it resolves to this, so an undeclared axis means "the
/// plan's reference value" rather than a constant this module invented. It is
/// the same point the workbench chrome selects and the solver's `TEMP` option
/// carries — `ReferencePvtPoint` is this type, not a copy of it.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferencePoint {
    pub process: ProcessCorner,
    pub temperature_celsius: f64,
}

impl Default for ReferencePoint {
    fn default() -> Self {
        Self {
            process: ProcessCorner::TT,
            temperature_celsius: 27.0,
        }
    }
}
