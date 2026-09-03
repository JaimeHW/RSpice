//! Immutable, lazy materialization of canonical deck-plan coordinates.
//!
//! [`DeckPlan`] owns target-neutral axis and analysis identity. This module is
//! the single bridge from those identities to the existing checked STEP
//! reparser and the engine's elaborated topology. Preparing a materializer
//! allocates only bounded coordinate metadata; netlists and topologies are
//! produced one requested coordinate at a time.

use super::plan::analysis_kind;
use super::{
    AnalysisInstanceId, AnalysisKind, DeckPlan, DeckPlanError, PlannedAnalysis, RunAxisValue,
    RunCoordinate, RunCoordinateId,
};
use crate::Netlist;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::engine::{Engine, SimulationError, StepPlan, StepPlanLimits};
use crate::netlist::{AnalysisCommand, StepCommand, StepSweep, StepTarget};

/// Stable, path-neutral namespace identity for one coordinate/analysis pair.
///
/// Frontends choose their own directory separators and file extensions. They
/// must preserve these two components so output and checkpoint identities do
/// not collide across repeated analyses or Cartesian coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactNamespace {
    coordinate: RunCoordinateId,
    analysis: AnalysisInstanceId,
}

impl ArtifactNamespace {
    const fn new(coordinate: RunCoordinateId, analysis: AnalysisInstanceId) -> Self {
        Self {
            coordinate,
            analysis,
        }
    }

    pub const fn coordinate_id(self) -> RunCoordinateId {
        self.coordinate
    }

    pub const fn analysis_id(self) -> AnalysisInstanceId {
        self.analysis
    }

    pub fn coordinate_component(self) -> String {
        format!("run-{}", self.coordinate)
    }

    pub fn analysis_component(self) -> String {
        self.analysis.tag()
    }

    pub fn components(self) -> [String; 2] {
        [self.coordinate_component(), self.analysis_component()]
    }
}

/// One configured physical analysis in a materialized coordinate.
///
/// Attached `.FOUR` and `.FFT` requests deliberately do not receive an
/// independent entry here: `.FOUR` remains in [`Netlist::analyses`] and
/// `.FFT` remains in [`Netlist::fft_analyses`] so an executor can bind each
/// post-process to its exact parent transient.
#[derive(Debug, Clone)]
pub struct MaterializedAnalysis {
    planned: PlannedAnalysis,
    /// `None` exists only for the explicit implicit-OP plan request.
    command: Option<AnalysisCommand>,
    output_namespace: ArtifactNamespace,
    checkpoint_namespace: ArtifactNamespace,
}

impl MaterializedAnalysis {
    pub const fn planned(&self) -> &PlannedAnalysis {
        &self.planned
    }

    pub const fn id(&self) -> AnalysisInstanceId {
        self.planned.id()
    }

    /// Exact coordinate-local authored configuration, after parameter and
    /// conditional materialization. Implicit OP deliberately has no authored
    /// command.
    pub const fn command(&self) -> Option<&AnalysisCommand> {
        self.command.as_ref()
    }

    pub const fn output_namespace(&self) -> ArtifactNamespace {
        self.output_namespace
    }

    pub const fn checkpoint_namespace(&self) -> ArtifactNamespace {
        self.checkpoint_namespace
    }
}

/// One immutable, fully elaborated member of a [`DeckPlan`].
///
/// [`Self::analyses`] contains planned physical/implicit analyses only. The
/// retained netlist is also the authoritative source for attached `.FOUR`
/// and `.FFT` post-processing requests.
#[derive(Debug, Clone)]
pub struct MaterializedRun {
    coordinate: RunCoordinate,
    netlist: Netlist,
    topology: super::TopologyFingerprint,
    analyses: Vec<MaterializedAnalysis>,
}

impl MaterializedRun {
    pub const fn coordinate(&self) -> &RunCoordinate {
        &self.coordinate
    }

    pub const fn netlist(&self) -> &Netlist {
        &self.netlist
    }

    pub const fn topology_fingerprint(&self) -> super::TopologyFingerprint {
        self.topology
    }

    pub fn analyses(&self) -> &[MaterializedAnalysis] {
        &self.analyses
    }

    /// Consume this run without cloning its coordinate-local netlist or
    /// analysis configurations.
    ///
    /// Frontend adapters that need an owned netlist should prefer this over
    /// cloning [`Self::netlist`], because a materialized hierarchy can be
    /// large even though only one coordinate is retained at a time.
    pub fn into_parts(
        self,
    ) -> (
        RunCoordinate,
        Netlist,
        super::TopologyFingerprint,
        Vec<MaterializedAnalysis>,
    ) {
        (self.coordinate, self.netlist, self.topology, self.analyses)
    }
}

/// Prepared lazy bridge from one canonical planning structure to its borrowed
/// netlist and materialized runs.
///
/// Preparation proves that the plan's axes and ordered analysis identities
/// equal a fresh canonical plan of the borrowed netlist. The bridge then
/// retains that exact engine/netlist borrow, preventing later materialization
/// with a different pair. It does not claim that [`DeckPlan`] is a digest of
/// every circuit statement. The complete bounded coordinate-metadata product
/// is retained under the engine resource policy; full netlists and topologies
/// are still produced only one requested coordinate at a time.
pub struct DeckPlanMaterializer<'a> {
    engine: &'a Engine,
    base_netlist: &'a Netlist,
    analyses: Vec<PlannedAnalysis>,
    coordinates: Vec<RunCoordinate>,
    step_plan: Option<StepPlan<'a>>,
}

impl DeckPlanMaterializer<'_> {
    pub fn coordinates(&self) -> &[RunCoordinate] {
        &self.coordinates
    }

    pub fn analyses(&self) -> &[PlannedAnalysis] {
        &self.analyses
    }

    pub const fn len(&self) -> usize {
        self.coordinates.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.coordinates.is_empty()
    }

    /// Materialize one coordinate without an external cancellation source.
    pub fn materialize_run(
        &self,
        run_index: usize,
    ) -> Result<MaterializedRun, MaterializedRunError> {
        self.materialize_run_with_abort(run_index, &NoAbort)
    }

    /// Materialize one coordinate while observing cooperative cancellation.
    pub fn materialize_run_with_abort(
        &self,
        run_index: usize,
        abort: &dyn AbortSignal,
    ) -> Result<MaterializedRun, MaterializedRunError> {
        if abort.is_aborted() {
            return Err(MaterializedRunError::Aborted);
        }
        let coordinate = self.coordinates.get(run_index).cloned().ok_or(
            MaterializedRunError::CoordinateIndex {
                index: run_index,
                coordinate_count: self.coordinates.len(),
            },
        )?;

        let mut netlist = match &self.step_plan {
            Some(step_plan) => {
                let indices_match = step_plan
                    .coordinate_indices_match(run_index, &coordinate, abort)
                    .map_err(materialization_simulation_error)?
                    .ok_or(MaterializedRunError::CoordinateIndex {
                        index: run_index,
                        coordinate_count: self.coordinates.len(),
                    })?;
                if !indices_match {
                    return Err(MaterializedRunError::CoordinateIdentityMismatch {
                        coordinate: coordinate.stable_id(),
                    });
                }
                self.engine
                    .materialize_step_run_with_abort(step_plan, run_index, abort)
                    .map_err(materialization_simulation_error)?
                    .into_parts()
                    .1
            }
            None => self.base_netlist.clone(),
        };
        if abort.is_aborted() {
            return Err(MaterializedRunError::Aborted);
        }

        // A Spectre `statistics` block draws from a coordinate. Monte Carlo
        // stamps its own per trial; every other coordinate — plain, `.STEP`,
        // `.TEMP`, or DATA — gets the one this plan defines. Without it the
        // builder finds no coordinate, skips process and mismatch sampling
        // entirely, and a swept statistical deck runs nominal at every point.
        if !netlist.spectre_statistics.variations.is_empty()
            && netlist.spectre_statistical_coordinate.is_none()
        {
            netlist.spectre_statistical_coordinate =
                Some(self.spectre_statistical_coordinate(&coordinate, &netlist)?);
        }

        // A concrete run must never feed its meta-analysis cards back into an
        // executor. Post-processing cards such as FOUR remain on the netlist.
        netlist.analyses.retain(|analysis| {
            !matches!(
                analysis,
                AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. }
            )
        });
        let mut configured_count = 0usize;
        for analysis in &netlist.analyses {
            ensure_not_aborted(abort)?;
            if !matches!(analysis, AnalysisCommand::Four { .. }) {
                configured_count =
                    configured_count
                        .checked_add(1)
                        .ok_or(MaterializedRunError::Allocation {
                            object: "materialized analysis configurations",
                        })?;
            }
        }
        let mut configured =
            try_vec_with_capacity(configured_count, "materialized analysis configurations")?;
        for analysis in &netlist.analyses {
            ensure_not_aborted(abort)?;
            if !matches!(analysis, AnalysisCommand::Four { .. }) {
                configured.push(analysis.clone());
            }
        }
        validate_analysis_identity(&self.analyses, &configured, coordinate.stable_id(), abort)?;

        let mut analyses = try_vec_with_capacity(
            self.analyses.len(),
            "materialized analysis identities and namespaces",
        )?;
        if is_implicit_op(&self.analyses) {
            ensure_not_aborted(abort)?;
            let planned = self.analyses[0].clone();
            let namespace = ArtifactNamespace::new(coordinate.stable_id(), planned.id());
            analyses.push(MaterializedAnalysis {
                planned,
                command: None,
                output_namespace: namespace,
                checkpoint_namespace: namespace,
            });
        } else {
            for (planned, command) in self.analyses.iter().cloned().zip(configured) {
                ensure_not_aborted(abort)?;
                let namespace = ArtifactNamespace::new(coordinate.stable_id(), planned.id());
                analyses.push(MaterializedAnalysis {
                    planned,
                    command: Some(command),
                    output_namespace: namespace,
                    checkpoint_namespace: namespace,
                });
            }
        }

        let topology = self
            .engine
            .topology_fingerprint_with_abort(&netlist, abort)
            .map_err(materialization_simulation_error)?;
        if abort.is_aborted() {
            return Err(MaterializedRunError::Aborted);
        }

        Ok(MaterializedRun {
            coordinate,
            netlist,
            topology,
            analyses,
        })
    }

    /// Statistical coordinate for one planned run coordinate.
    ///
    /// The axes are the coordinate's own assignments: a numeric `.STEP` or
    /// `.TEMP` value directly, and a DATA row as one axis per bound parameter
    /// so two rows that bind identical values are one statistical point.
    /// `spectre_statistics` canonicalizes and sorts these names before
    /// hashing, so the order they are collected in is immaterial. The
    /// temperature is the one the coordinate materialized, or the engine's
    /// configured temperature when no axis moved it.
    fn spectre_statistical_coordinate(
        &self,
        coordinate: &RunCoordinate,
        netlist: &Netlist,
    ) -> Result<crate::netlist::SpectreStatisticalCoordinate, MaterializedRunError> {
        let mut axes = Vec::new();
        for assignment in coordinate.assignments() {
            match assignment.value() {
                RunAxisValue::Numeric(value) => axes.push((assignment.name().to_owned(), *value)),
                RunAxisValue::DataRow(bindings) => axes.extend(
                    bindings
                        .iter()
                        .map(|binding| (binding.name().to_owned(), binding.value())),
                ),
                // Preparation already refuses textual ALTER; a variant axis
                // has no numeric coordinate to seed a draw from.
                RunAxisValue::AlterVariant { .. } => {
                    return Err(MaterializedRunError::AlterUnsupported);
                }
            }
        }
        let temperature_celsius = netlist.options.temp.unwrap_or_else(|| {
            crate::constants::kelvin_to_celsius(self.engine.config().temperature)
        });
        Ok(crate::netlist::SpectreStatisticalCoordinate {
            seed: 0,
            monte_carlo_run: 0,
            temperature_celsius,
            axes,
        })
    }
}

impl Engine {
    /// Prepare a lazy materializer under this engine's resource policy.
    pub fn prepare_deck_plan_materializer<'a>(
        &'a self,
        netlist: &'a Netlist,
        plan: &DeckPlan,
    ) -> Result<DeckPlanMaterializer<'a>, MaterializedRunError> {
        self.prepare_deck_plan_materializer_with_abort(netlist, plan, &NoAbort)
    }

    /// Prepare a lazy materializer while observing cooperative cancellation.
    pub fn prepare_deck_plan_materializer_with_abort<'a>(
        &'a self,
        netlist: &'a Netlist,
        plan: &DeckPlan,
        abort: &dyn AbortSignal,
    ) -> Result<DeckPlanMaterializer<'a>, MaterializedRunError> {
        if abort.is_aborted() {
            return Err(MaterializedRunError::Aborted);
        }
        if contains_textual_alter(netlist, abort)? {
            return Err(MaterializedRunError::AlterUnsupported);
        }
        for axis in plan.axes() {
            ensure_not_aborted(abort)?;
            if axis.kind() == super::AxisKind::Alter {
                return Err(MaterializedRunError::AlterUnsupported);
            }
        }

        let limits = self.config().resource_limits;
        let canonical = DeckPlan::from_netlist_with_abort(netlist, &limits, abort)
            .map_err(materialization_plan_error)?;
        if &canonical != plan {
            return Err(MaterializedRunError::PlanNetlistMismatch);
        }
        let coordinates = plan
            .coordinates_with_abort(&limits, abort)
            .map_err(materialization_plan_error)?;
        let step_plan = if plan.axes().is_empty() {
            None
        } else {
            let steps = canonical_step_commands(netlist, abort)?;
            if steps.len() != plan.axes().len() {
                return Err(MaterializedRunError::AxisMaterializerMismatch {
                    planned: plan.axes().len(),
                    materialized: steps.len(),
                });
            }
            let step_plan = self
                .plan_step_commands_with_abort(
                    netlist,
                    &steps,
                    StepPlanLimits::from_resource_limits(limits),
                    abort,
                )
                .map_err(materialization_simulation_error)?;
            if step_plan.total_runs() != coordinates.len() {
                return Err(MaterializedRunError::CoordinateCardinalityMismatch {
                    planned: coordinates.len(),
                    materialized: step_plan.total_runs(),
                });
            }
            Some(step_plan)
        };

        let mut analyses =
            try_vec_with_capacity(plan.analyses().len(), "prepared analysis identities")?;
        for analysis in plan.analyses() {
            ensure_not_aborted(abort)?;
            analyses.push(analysis.clone());
        }
        Ok(DeckPlanMaterializer {
            engine: self,
            base_netlist: netlist,
            analyses,
            coordinates,
            step_plan,
        })
    }
}

fn contains_textual_alter(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<bool, MaterializedRunError> {
    let Some(source) = netlist.source_text.as_deref() else {
        return Ok(false);
    };
    for line in source.lines() {
        ensure_not_aborted(abort)?;
        if line
            .split_whitespace()
            .next()
            .is_some_and(|token| token.eq_ignore_ascii_case(".alter"))
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn canonical_step_commands(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<Vec<StepCommand>, MaterializedRunError> {
    let mut data_count = 0usize;
    let mut step_count = 0usize;
    let mut temperature_count = 0usize;
    for command in &netlist.analyses {
        ensure_not_aborted(abort)?;
        let count = match command {
            AnalysisCommand::Step(step) if matches!(&step.sweep, StepSweep::Data { .. }) => {
                &mut data_count
            }
            AnalysisCommand::Step(step) if step.target == StepTarget::Temp => {
                &mut temperature_count
            }
            AnalysisCommand::Step(_) => &mut step_count,
            AnalysisCommand::Temp { .. } => &mut temperature_count,
            _ => continue,
        };
        *count = count
            .checked_add(1)
            .ok_or(MaterializedRunError::Allocation {
                object: "materializer command counts",
            })?;
    }
    let mut data = try_vec_with_capacity(data_count, "DATA materializer commands")?;
    let mut step = try_vec_with_capacity(step_count, "STEP materializer commands")?;
    let mut temperature =
        try_vec_with_capacity(temperature_count, "temperature materializer commands")?;
    for command in &netlist.analyses {
        ensure_not_aborted(abort)?;
        match command {
            AnalysisCommand::Step(command) if matches!(&command.sweep, StepSweep::Data { .. }) => {
                data.push(command.clone());
            }
            AnalysisCommand::Step(command) if command.target == StepTarget::Temp => {
                temperature.push(command.clone());
            }
            AnalysisCommand::Step(command) => step.push(command.clone()),
            AnalysisCommand::Temp { temperatures } => temperature.push(StepCommand {
                target: StepTarget::Temp,
                name: "TEMP".to_string(),
                param_name: None,
                sweep: StepSweep::List(temperatures.clone()),
            }),
            _ => {}
        }
    }
    let total = data_count
        .checked_add(step_count)
        .and_then(|count| count.checked_add(temperature_count))
        .ok_or(MaterializedRunError::Allocation {
            object: "ordered materializer commands",
        })?;
    let mut ordered = try_vec_with_capacity(total, "ordered materializer commands")?;
    ordered.extend(data);
    ordered.extend(step);
    ordered.extend(temperature);
    Ok(ordered)
}

fn is_implicit_op(analyses: &[PlannedAnalysis]) -> bool {
    analyses.len() == 1 && analyses[0].id().kind() == AnalysisKind::ImplicitOp
}

fn validate_analysis_identity(
    planned: &[PlannedAnalysis],
    configured: &[AnalysisCommand],
    coordinate: RunCoordinateId,
    abort: &dyn AbortSignal,
) -> Result<(), MaterializedRunError> {
    let valid = if is_implicit_op(planned) {
        configured.is_empty()
    } else {
        let mut valid = planned.len() == configured.len();
        if valid {
            for (planned, configured) in planned.iter().zip(configured) {
                ensure_not_aborted(abort)?;
                if planned.id().kind() != analysis_kind(configured) {
                    valid = false;
                    break;
                }
            }
        }
        valid
    };
    if valid {
        Ok(())
    } else {
        let mut expected =
            try_vec_with_capacity(planned.len(), "expected analysis mismatch details")?;
        for analysis in planned {
            ensure_not_aborted(abort)?;
            expected.push(analysis.id().kind());
        }
        let mut actual =
            try_vec_with_capacity(configured.len(), "actual analysis mismatch details")?;
        for analysis in configured {
            ensure_not_aborted(abort)?;
            actual.push(analysis_kind(analysis));
        }
        Err(MaterializedRunError::AnalysisIdentityMismatch {
            coordinate,
            expected,
            actual,
        })
    }
}

fn try_vec_with_capacity<T>(
    capacity: usize,
    object: &'static str,
) -> Result<Vec<T>, MaterializedRunError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| MaterializedRunError::Allocation { object })?;
    Ok(values)
}

fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), MaterializedRunError> {
    if abort.is_aborted() {
        Err(MaterializedRunError::Aborted)
    } else {
        Ok(())
    }
}

fn materialization_plan_error(error: DeckPlanError) -> MaterializedRunError {
    match error {
        DeckPlanError::Aborted => MaterializedRunError::Aborted,
        other => MaterializedRunError::DeckPlan(other),
    }
}

fn materialization_simulation_error(error: SimulationError) -> MaterializedRunError {
    match error {
        SimulationError::Aborted => MaterializedRunError::Aborted,
        other => MaterializedRunError::Simulation(other),
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MaterializedRunError {
    #[error("deck materialization aborted")]
    Aborted,
    #[error("unable to allocate bounded {object}")]
    Allocation { object: &'static str },
    #[error(transparent)]
    DeckPlan(DeckPlanError),
    #[error(transparent)]
    Simulation(SimulationError),
    #[error("textual ALTER variants must be expanded before DeckPlan materialization")]
    AlterUnsupported,
    #[error(
        "DeckPlan axes or ordered analysis identities do not match the supplied netlist's canonical plan"
    )]
    PlanNetlistMismatch,
    #[error(
        "DeckPlan has {planned} run axes, but its netlist exposes {materialized} materializer dimensions"
    )]
    AxisMaterializerMismatch { planned: usize, materialized: usize },
    #[error("DeckPlan has {planned} coordinates, but its checked materializer has {materialized}")]
    CoordinateCardinalityMismatch { planned: usize, materialized: usize },
    #[error("run coordinate index {index} is outside {coordinate_count} planned coordinates")]
    CoordinateIndex {
        index: usize,
        coordinate_count: usize,
    },
    #[error("checked materializer selected different values for coordinate {coordinate}")]
    CoordinateIdentityMismatch { coordinate: RunCoordinateId },
    #[error(
        "materialized analysis identity changed at coordinate {coordinate}: expected {expected:?}, got {actual:?}"
    )]
    AnalysisIdentityMismatch {
        coordinate: RunCoordinateId,
        expected: Vec<AnalysisKind>,
        actual: Vec<AnalysisKind>,
    },
}

impl MaterializedRunError {
    pub const fn is_aborted(&self) -> bool {
        matches!(self, Self::Aborted)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::abort_signal::ImmediateAbort;
    use crate::execution::{
        AnalysisRequest, AxisKind, RunAxis, RunAxisValue, StepAxisTarget, TopologyFingerprint,
    };
    use crate::resource::{ResourceKind, ResourceLimitError};
    use crate::{ResourceLimits, SimulationConfig};

    fn plan(netlist: &Netlist) -> DeckPlan {
        DeckPlan::from_netlist(netlist, &ResourceLimits::default()).expect("canonical deck plan")
    }

    struct AbortOnPoll {
        polls: AtomicUsize,
        abort_on: usize,
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn no_axis_deck_materializes_one_explicit_implicit_op_namespace() {
        let netlist = Netlist::parse("implicit op\nV1 in 0 1\nR1 in 0 1k\n.end\n")
            .expect("implicit-OP deck parses");
        let plan = plan(&netlist);
        let engine = Engine::default();
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("scalar materializer");

        assert_eq!(materializer.len(), 1);
        assert!(!materializer.is_empty());
        assert_eq!(materializer.analyses(), plan.analyses());
        let run = materializer.materialize_run(0).expect("scalar run");
        assert_eq!(run.coordinate(), &materializer.coordinates()[0]);
        assert!(run.coordinate().assignments().is_empty());
        assert_eq!(run.analyses().len(), 1);
        assert_eq!(run.analyses()[0].id().kind(), AnalysisKind::ImplicitOp);
        assert!(run.analyses()[0].command().is_none());
        let expected = [run.coordinate().stable_tag(), "implicit-op-001".to_string()];
        assert_eq!(run.analyses()[0].output_namespace().components(), expected);
        assert_eq!(
            run.analyses()[0].checkpoint_namespace().components(),
            expected
        );
        assert!(run.netlist().analyses.is_empty());
        let expected_topology = run.topology_fingerprint();
        let (coordinate, netlist, topology, analyses) = run.into_parts();
        assert_eq!(coordinate, materializer.coordinates()[0]);
        assert!(netlist.analyses.is_empty());
        assert_eq!(topology, expected_topology);
        assert_eq!(analyses.len(), 1);
    }

    #[test]
    fn data_step_temp_coordinates_bit_match_the_checked_reparser() {
        let netlist = Netlist::parse(
            "DATA STEP TEMP materialization\n\
             .param bias=1 gain=1\n\
             V1 out 0 {bias+gain}\n\
             R1 out 0 1k\n\
             .data samples bias\n\
             10\n\
             20\n\
             .enddata\n\
             .step data=samples\n\
             .step param gain list 1 2\n\
             .temp 25 100\n\
             .ac lin 1 {gain} {gain}\n\
             .end\n",
        )
        .expect("mixed-axis deck parses");
        let plan = plan(&netlist);
        let expected = plan
            .coordinates_with_abort(&ResourceLimits::default(), &NoAbort)
            .expect("canonical coordinates");
        let engine = Engine::default();
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("mixed-axis materializer");

        assert_eq!(materializer.coordinates(), expected);
        assert_eq!(materializer.len(), 8);
        for (index, coordinate) in expected.iter().enumerate() {
            let run = materializer
                .materialize_run(index)
                .unwrap_or_else(|error| panic!("materialize coordinate {index}: {error}"));
            assert_eq!(run.coordinate(), coordinate);
            assert!(run.netlist().analyses.iter().all(|analysis| !matches!(
                analysis,
                AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. }
            )));
            let [data, gain, temperature] = coordinate.assignments() else {
                panic!("coordinate must retain DATA, STEP, and TEMP assignments")
            };
            assert_eq!(data.kind(), AxisKind::Data);
            assert_eq!(gain.kind(), AxisKind::Step);
            assert_eq!(temperature.kind(), AxisKind::Temperature);
            let RunAxisValue::DataRow(bindings) = data.value() else {
                panic!("first assignment must be one coupled DATA row")
            };
            assert_eq!(bindings.len(), 1);
            assert_eq!(bindings[0].name(), "bias");
            assert_eq!(bindings[0].value(), [10.0, 20.0][data.value_index()]);
            let RunAxisValue::Numeric(gain_value) = gain.value() else {
                panic!("gain assignment must be numeric")
            };
            let Some(AnalysisCommand::Ac {
                start_freq,
                stop_freq,
                ..
            }) = run.analyses()[0].command()
            else {
                panic!("coordinate must retain its configured AC command")
            };
            assert_eq!(start_freq.to_bits(), gain_value.to_bits());
            assert_eq!(stop_freq.to_bits(), gain_value.to_bits());
        }
    }

    #[test]
    fn repeated_analysis_namespaces_compose_coordinate_and_analysis_ids() {
        let netlist = Netlist::parse(
            "repeated AC\n\
             V1 in 0 AC 1\n\
             R1 in 0 1k\n\
             .ac lin 1 1k 1k\n\
             .ac lin 1 10k 10k\n\
             .end\n",
        )
        .expect("repeated-analysis deck parses");
        let plan = plan(&netlist);
        let engine = Engine::default();
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("repeated-analysis materializer");
        let run = materializer.materialize_run(0).expect("scalar run");

        assert_eq!(
            run.analyses()
                .iter()
                .map(MaterializedAnalysis::id)
                .map(AnalysisInstanceId::tag)
                .collect::<Vec<_>>(),
            ["ac-001", "ac-002"]
        );
        let first = run.analyses()[0].output_namespace();
        let second = run.analyses()[1].output_namespace();
        assert_eq!(first.coordinate_id(), run.coordinate().stable_id());
        assert_eq!(second.coordinate_id(), run.coordinate().stable_id());
        assert_ne!(first, second);
        assert_eq!(first.components()[1], "ac-001");
        assert_eq!(second.components()[1], "ac-002");
        assert_eq!(
            run.analyses()[0].checkpoint_namespace(),
            run.analyses()[0].output_namespace()
        );
    }

    #[test]
    fn attached_fourier_and_fft_remain_on_their_parent_transient_netlist() {
        let netlist = Netlist::parse(
            "attached transient post-processing\n\
             V1 out 0 SIN(0 1 1k)\n\
             R1 out 0 1k\n\
             .four 1k V(out)\n\
             .tran 10u 1m\n\
             .fft v(out) np=8 window=hann\n\
             .end\n",
        )
        .expect("transient post-process deck parses");
        let plan = plan(&netlist);
        assert_eq!(plan.analyses().len(), 1);
        assert_eq!(plan.analyses()[0].id().kind(), AnalysisKind::Tran);
        let engine = Engine::default();
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("post-process materializer");
        let run = materializer.materialize_run(0).expect("transient run");

        assert_eq!(run.analyses().len(), 1);
        assert_eq!(run.analyses()[0].id().tag(), "tran-001");
        assert!(matches!(
            run.analyses()[0].command(),
            Some(AnalysisCommand::Tran { .. })
        ));
        assert_eq!(
            run.netlist()
                .analyses
                .iter()
                .filter(|analysis| matches!(analysis, AnalysisCommand::Four { .. }))
                .count(),
            1
        );
        assert_eq!(run.netlist().fft_analyses.len(), 1);
    }

    fn conditional_topologies(
        values: &str,
    ) -> BTreeMap<i32, (RunCoordinateId, TopologyFingerprint)> {
        let netlist = Netlist::parse(&format!(
            "conditional topology\n\
             .param mode=0\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             .if (mode==1)\n\
             R2 out extra 1k\n\
             R3 extra 0 1k\n\
             .else\n\
             R4 out 0 1k\n\
             .endif\n\
             .step param mode list {values}\n\
             .op\n\
             .end\n"
        ))
        .expect("conditional-topology deck parses");
        let plan = plan(&netlist);
        let engine = Engine::default();
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("conditional-topology materializer");

        materializer
            .coordinates()
            .iter()
            .enumerate()
            .map(|(index, coordinate)| {
                let RunAxisValue::Numeric(mode) = coordinate.assignments()[0].value() else {
                    panic!("mode must be numeric")
                };
                let run = materializer
                    .materialize_run(index)
                    .expect("conditional coordinate materializes");
                (
                    *mode as i32,
                    (run.coordinate().stable_id(), run.topology_fingerprint()),
                )
            })
            .collect()
    }

    #[test]
    fn conditional_topology_is_coordinate_local_and_order_independent() {
        let forward = conditional_topologies("0 1");
        let reverse = conditional_topologies("1 0");
        assert_ne!(forward[&0].1, forward[&1].1);
        assert_eq!(forward, reverse);
    }

    #[test]
    fn conditional_analysis_mismatch_is_lazy_and_fails_closed() {
        let netlist = Netlist::parse(
            "conditional analysis\n\
             .param mode=0\n\
             V1 out 0 1\n\
             R1 out 0 1k\n\
             .step param mode list 0 1\n\
             .if (mode==1)\n\
             .tran 1n 2n\n\
             .endif\n\
             .end\n",
        )
        .expect("conditional-analysis deck parses");
        let plan = plan(&netlist);
        assert_eq!(plan.analyses()[0].id().kind(), AnalysisKind::ImplicitOp);
        let engine = Engine::default();

        // Preparation is intentionally lazy: it does not build either
        // coordinate's netlist or topology.
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("lazy materializer");
        materializer
            .materialize_run(0)
            .expect("unchanged first coordinate");
        assert!(matches!(
            materializer.materialize_run(1),
            Err(MaterializedRunError::AnalysisIdentityMismatch {
                expected,
                actual,
                ..
            }) if expected == vec![AnalysisKind::ImplicitOp]
                && actual == vec![AnalysisKind::Tran]
        ));
    }

    #[test]
    fn alter_is_explicitly_unsupported_until_source_variants_join_deck_plan() {
        let netlist =
            Netlist::parse("ALTER boundary\nV1 out 0 1\n.op\n.end\n").expect("base deck parses");
        let alter = RunAxis::new(
            AxisKind::Alter,
            "alter",
            vec![RunAxisValue::AlterVariant {
                label: "variant".to_string(),
                materialization_digest: [1; 32],
            }],
        )
        .expect("valid explicit ALTER axis");
        let alter_plan = DeckPlan::new(vec![alter], vec![AnalysisRequest::new(AnalysisKind::Op)])
            .expect("manual ALTER plan");
        assert!(matches!(
            Engine::default().prepare_deck_plan_materializer(&netlist, &alter_plan),
            Err(MaterializedRunError::AlterUnsupported)
        ));

        let authored = Netlist::parse(
            "authored ALTER boundary\n\
             V1 out 0 1\n\
             .op\n\
             .alter second\n\
             V1 out 0 2\n\
             .end\n",
        )
        .expect("parser retains the base ALTER variant");
        let authored_plan = plan(&authored);
        assert!(matches!(
            Engine::default().prepare_deck_plan_materializer(&authored, &authored_plan),
            Err(MaterializedRunError::AlterUnsupported)
        ));
    }

    #[test]
    fn preparation_and_materialization_observe_limits_abort_and_indices() {
        let netlist = Netlist::parse(
            "bounded plan\n\
             .param p=0\n\
             V1 out 0 {p}\n\
             .step param p list 1 2\n\
             .op\n\
             .end\n",
        )
        .expect("bounded deck parses");
        let plan = plan(&netlist);
        let mut config = SimulationConfig::default();
        config.resource_limits.max_batch_runs = 1;
        let limited = Engine::new(config);
        assert!(matches!(
            limited.prepare_deck_plan_materializer(&netlist, &plan),
            Err(MaterializedRunError::DeckPlan(
                DeckPlanError::ResourceLimit(ResourceLimitError {
                    resource: ResourceKind::BatchRuns,
                    requested: 2,
                    limit: 1,
                })
            ))
        ));

        let engine = Engine::default();
        assert!(matches!(
            engine.prepare_deck_plan_materializer_with_abort(&netlist, &plan, &ImmediateAbort),
            Err(MaterializedRunError::Aborted)
        ));
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("normal materializer");
        assert!(matches!(
            materializer.materialize_run_with_abort(0, &ImmediateAbort),
            Err(MaterializedRunError::Aborted)
        ));
        assert!(matches!(
            materializer.materialize_run(2),
            Err(MaterializedRunError::CoordinateIndex {
                index: 2,
                coordinate_count: 2,
            })
        ));
    }

    #[test]
    fn materialization_observes_abort_during_analysis_projection() {
        let netlist = Netlist::parse(
            "projection cancellation\n\
             V1 out 0 1\n\
             R1 out 0 1k\n\
             .op\n\
             .ac lin 1 1k 1k\n\
             .end\n",
        )
        .expect("multi-analysis deck parses");
        let plan = plan(&netlist);
        let engine = Engine::default();
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("scalar materializer");
        let abort = AbortOnPoll {
            polls: AtomicUsize::new(0),
            abort_on: 3,
        };

        assert!(matches!(
            materializer.materialize_run_with_abort(0, &abort),
            Err(MaterializedRunError::Aborted)
        ));
        assert_eq!(abort.polls.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn plan_and_netlist_axis_or_analysis_identity_mismatch_is_rejected() {
        let first = Netlist::parse(
            "first plan\n\
             .param p=0\n\
             V1 out 0 {p}\n\
             .step param p list 1 2\n\
             .op\n\
             .end\n",
        )
        .expect("first deck parses");
        let different_axis = Netlist::parse(
            "different axis\n\
             .param p=0\n\
             V1 out 0 {p}\n\
             .step param p list 1 3\n\
             .op\n\
             .end\n",
        )
        .expect("different-axis deck parses");
        let different_analysis = Netlist::parse(
            "different analysis\n\
             .param p=0\n\
             V1 out 0 {p}\n\
             .step param p list 1 2\n\
             .tran 1n 2n\n\
             .end\n",
        )
        .expect("different-analysis deck parses");
        let first_plan = plan(&first);
        let engine = Engine::default();

        assert!(matches!(
            engine.prepare_deck_plan_materializer(&different_axis, &first_plan),
            Err(MaterializedRunError::PlanNetlistMismatch)
        ));
        assert!(matches!(
            engine.prepare_deck_plan_materializer(&different_analysis, &first_plan),
            Err(MaterializedRunError::PlanNetlistMismatch)
        ));
    }

    #[test]
    fn step_target_configuration_survives_materialization() {
        let netlist = Netlist::parse(
            "typed target\n\
             .param p=1\n\
             V1 out 0 {p}\n\
             .step param p list 1 2\n\
             .op\n\
             .end\n",
        )
        .expect("typed-target deck parses");
        let plan = plan(&netlist);
        assert!(matches!(
            plan.axes()[0].step_target(),
            Some(StepAxisTarget::Parameter { name }) if name == "p"
        ));
        let engine = Engine::default();
        let materializer = engine
            .prepare_deck_plan_materializer(&netlist, &plan)
            .expect("typed-target materializer");
        for index in 0..2 {
            let run = materializer.materialize_run(index).expect("coordinate");
            assert!(matches!(
                run.analyses()[0].command(),
                Some(AnalysisCommand::Op)
            ));
            assert_eq!(
                run.coordinate().assignments()[0].step_target(),
                plan.axes()[0].step_target()
            );
        }
    }
}
