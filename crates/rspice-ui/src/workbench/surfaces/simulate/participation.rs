//! Per-instance run-set participation, as the Simulation Studio's surfaces
//! read it.
//!
//! Four surfaces have to agree about which points an analysis visits: the Run
//! Set page's forecast, its resolved point table, the specification registry's
//! coverage advisory, and the Analyses page's own control. Each used to answer
//! the question by multiplying the whole plan by the whole matrix, which is
//! exactly the arithmetic participation exists to stop.
//!
//! [`PlanParticipation`] resolves it once per frame from the working plan and
//! the working run set, through the same
//! [`crate::simulation::run_set::participating_point_keys`] the prepared
//! expansion uses. Nothing here decides participation; it only reports what
//! that resolver returned, so a surface cannot disagree with the queue.

use std::collections::HashSet;

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisInstance, AnalysisKind};
use crate::simulation::run_set::{self, AnalysisRunAt, RunSetPoint};
use crate::workbench::RSpiceApp;

/// One enabled analysis instance and the points it visits.
pub(super) struct InstanceParticipation {
    pub(super) id: AnalysisInstanceId,
    pub(super) kind: AnalysisKind,
    pub(super) run_at: AnalysisRunAt,
    /// The resolved point identities, in declared order. Empty only when the
    /// participation refused, in which case `refusal` says why.
    pub(super) keys: Vec<String>,
    /// Why this participation does not resolve against the declared space.
    pub(super) refusal: Option<String>,
}

impl InstanceParticipation {
    /// The compact stat a row or a card carries: `nominal`, `15 pts`, `4/15`.
    pub(super) fn stat(&self, matrix: usize) -> String {
        if self.refusal.is_some() {
            return "unresolved".to_owned();
        }
        match self.run_at {
            AnalysisRunAt::AllPoints => format!("{} pts", self.keys.len()),
            AnalysisRunAt::NominalPoint => "nominal".to_owned(),
            AnalysisRunAt::SelectedPoints(_) => format!("{}/{matrix}", self.keys.len()),
        }
    }
}

/// The whole plan's participation against the working run set.
pub(super) struct PlanParticipation {
    /// Every point the run set resolves, in declared order. Empty when the
    /// declaration does not expand exactly, which is a refusal the Run Set page
    /// already reports on its own terms.
    pub(super) point_keys: Vec<String>,
    /// The reference point's identity, when the declared space contains one.
    pub(super) nominal_key: Option<String>,
    pub(super) instances: Vec<InstanceParticipation>,
    /// Whether the run set declares any axis at all. With none, every analysis
    /// runs once and participation is not a question the page can ask.
    pub(super) has_axes: bool,
}

impl PlanParticipation {
    /// Resolve the working plan against the working run set.
    pub(super) fn resolve(app: &RSpiceApp) -> Self {
        let state = &app.state.sim_setup.run_set;
        let reference = app.state.sim_setup.reference_pvt;
        let has_axes = state.enabled_dimensions().next().is_some();
        let points: Vec<RunSetPoint<'_>> = run_set::resolve(state).unwrap_or_default();
        let point_keys: Vec<String> = points.iter().map(RunSetPoint::point_key).collect();
        let nominal_key = run_set::nominal_point_key(&points, reference);
        let instances = app
            .state
            .sim_setup
            .enabled_analysis_instances()
            .map(|instance| {
                let run_at = instance.run_at().clone();
                match run_set::participating_point_keys(&run_at, &points, reference) {
                    Ok(keys) => InstanceParticipation {
                        id: instance.id(),
                        kind: instance.kind(),
                        run_at,
                        keys,
                        refusal: None,
                    },
                    Err(refusal) => InstanceParticipation {
                        id: instance.id(),
                        kind: instance.kind(),
                        run_at,
                        keys: Vec::new(),
                        refusal: Some(refusal.message),
                    },
                }
            })
            .collect();
        Self {
            point_keys,
            nominal_key,
            instances,
            has_axes,
        }
    }

    /// How many points one analysis contributes a task at.
    ///
    /// A run set with no axes runs everything once, and a participation that
    /// refuses counts as the whole space — the same rule
    /// [`crate::simulation::run_set::participating_point_count`] states, because
    /// pricing a refused plan at zero is how a budget silently shrinks.
    pub(super) fn point_count_for(&self, id: AnalysisInstanceId) -> usize {
        if !self.has_axes {
            return 1;
        }
        self.instances
            .iter()
            .find(|entry| entry.id == id)
            .map_or(self.point_keys.len(), |entry| {
                if entry.refusal.is_some() {
                    self.point_keys.len()
                } else {
                    entry.keys.len()
                }
            })
    }

    pub(super) fn for_instance(&self, id: AnalysisInstanceId) -> Option<&InstanceParticipation> {
        self.instances.iter().find(|entry| entry.id == id)
    }

    /// The enabled analyses that run at one point, by kind code.
    ///
    /// Used by the point table, where the count is the cell and the names are
    /// the hover: a row that says "4/6" without naming the two that are absent
    /// tells the reader there is a problem and not where.
    pub(super) fn analyses_at(&self, key: &str) -> Vec<AnalysisKind> {
        self.instances
            .iter()
            .filter(|entry| entry.refusal.is_none() && entry.keys.iter().any(|held| held == key))
            .map(|entry| entry.kind)
            .collect()
    }

    /// The enabled analyses that do *not* run at one point.
    pub(super) fn analyses_absent_at(&self, key: &str) -> Vec<AnalysisKind> {
        self.instances
            .iter()
            .filter(|entry| entry.refusal.is_none() && !entry.keys.iter().any(|held| held == key))
            .map(|entry| entry.kind)
            .collect()
    }

    /// How many of `demanded` points at least one enabled analysis visits.
    ///
    /// This is what a specification's coverage is bounded by: a bound scoped to
    /// a point no analysis runs at can never be judged there, however the run
    /// goes.
    pub(super) fn covered_of(&self, demanded: &[String]) -> usize {
        let visited: HashSet<&str> = self
            .instances
            .iter()
            .filter(|entry| entry.refusal.is_none())
            .flat_map(|entry| entry.keys.iter().map(String::as_str))
            .collect();
        demanded
            .iter()
            .filter(|key| visited.contains(key.as_str()))
            .count()
    }
}

/// The instance a participation edit applies to, resolved from the plan.
pub(super) fn instance_run_at(app: &RSpiceApp, id: AnalysisInstanceId) -> AnalysisRunAt {
    app.state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| plan.instance(id))
        .map_or_else(AnalysisRunAt::default, |instance| {
            AnalysisInstance::run_at(instance).clone()
        })
}
