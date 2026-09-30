//! Project a declared run space into executable corner configuration.

use super::{
    ReferencePoint, RunSetCompositionMode, RunSetDimensionKind, RunSetState,
    parse_supply_source_authority, resolve, validate,
};
use crate::corner_config::{CornerBaseAnalysis, CornerConfig, CornerPointSpec};
use rspice_app_types::product::ProcessCorner;

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
                    crate::run_set::process_section_index(&value.lexical)
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
                    parse_supply_source_authority(&dimension.source)?,
                ),
                None => (vec![UNSWEPT_SUPPLY], Vec::new()),
            };

        let temperatures = match self.enabled_dimension_of(RunSetDimensionKind::Temperature) {
            Some(dimension) => dimension.canonical_values(),
            None => vec![reference.temperature_celsius],
        };

        let points = if self.composition.filters() {
            explicit_points(self, reference)?
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
}

/// The resolved points of a filtered space, each stated in full.
///
/// An axis the run set does not declare has no coordinate on the point, so
/// it is filled from the plan's reference here — the executor takes points,
/// not partial ones, and leaving a hole would make the filtered path
/// disagree with the axis path about what an undeclared axis means.
fn explicit_points(
    state: &RunSetState,
    reference: ReferencePoint,
) -> Result<Vec<CornerPointSpec>, String> {
    let resolved = resolve(state).ok_or_else(|| {
        "The declared space does not expand exactly, so its points cannot be executed".to_owned()
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
                        spec.process = *PROCESS_CORNERS
                            .get(canonical as usize)
                            .ok_or_else(|| format!("{} is not a process section", value.lexical))?;
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

/// Process corners in the order the process-section declaration names them.
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
