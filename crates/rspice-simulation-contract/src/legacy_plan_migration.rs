//! Deterministic schema-3 singleton-plan migration.
//!
//! The application supplies legacy draft values; plan identity, ordering and
//! prerequisite binding are portable contracts.

use std::collections::HashSet;

use rspice_app_types::product::{AnalysisInstanceId, ObjectRevision, ProjectId, SimulationPlanId};
use uuid::Uuid;

use crate::analysis_draft::AnalysisDraft;
use crate::analysis_kind::AnalysisKind;
use crate::plan_model::{AnalysisDependency, AnalysisInstance, SimulationPlan};
use crate::run_set::RunSetState;

/// The schema-3 singleton noise draft, retained only for project migration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoiseSetup {
    /// Output node.
    pub output: String,
    /// Reference node (0 = ground).
    pub reference: String,
    /// Input source.
    pub input: String,
    /// Start frequency.
    pub fstart: String,
    /// Stop frequency.
    pub fstop: String,
}

impl Default for NoiseSetup {
    fn default() -> Self {
        Self {
            output: "out".to_owned(),
            reference: "0".to_owned(),
            input: "V1".to_owned(),
            fstart: "1".to_owned(),
            fstop: "100Meg".to_owned(),
        }
    }
}

/// A missing global run set predates the Studio declaration and means one
/// reference run, without inventing PVT tasks or technology requirements.
pub fn default_global_run_set() -> RunSetState {
    RunSetState::reference_only()
}

/// Decode a schema-3 analysis-index set without hiding duplicate entries.
pub fn deserialize_analysis_set<'de, D>(deserializer: D) -> Result<HashSet<usize>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let indices = <Vec<usize> as serde::Deserialize>::deserialize(deserializer)?;
    let mut unique = HashSet::with_capacity(indices.len());
    for index in indices {
        if !unique.insert(index) {
            return Err(serde::de::Error::custom(format!(
                "duplicate analysis index {index}"
            )));
        }
    }
    Ok(unique)
}

const LEGACY_ANALYSIS_PLAN_NAMESPACE: Uuid =
    Uuid::from_u128(0x4fca_8534_7bd6_52fb_a3f9_5ca4_d8e3_b127);

/// Migrate a legacy singleton declaration into the one authoritative plan.
/// The caller supplies the exact legacy draft for each canonical kind.
pub fn migrate_schema3_analysis_plan(
    project_id: ProjectId,
    enabled: &HashSet<usize>,
    listed: &HashSet<usize>,
    analysis_order: &[usize],
    mut draft_for: impl FnMut(AnalysisKind) -> AnalysisDraft,
) -> Result<SimulationPlan, String> {
    let analysis_count = AnalysisKind::ALL.len();
    if let Some(index) = enabled
        .iter()
        .chain(listed.iter())
        .copied()
        .find(|index| *index >= analysis_count)
    {
        return Err(format!(
            "legacy simulation plan contains unsupported analysis index {index}"
        ));
    }

    let mut seen = HashSet::with_capacity(analysis_count);
    let mut ordered = Vec::with_capacity(analysis_count);
    for index in analysis_order.iter().copied() {
        if index >= analysis_count {
            return Err(format!(
                "legacy simulation plan order contains unsupported analysis index {index}"
            ));
        }
        if !seen.insert(index) {
            return Err(format!(
                "legacy simulation plan order contains duplicate analysis index {index}"
            ));
        }
        if !enabled.contains(&index) {
            return Err(format!(
                "legacy simulation plan order contains disabled analysis index {index}"
            ));
        }
        ordered.push(index);
    }
    let mut missing_enabled = enabled
        .iter()
        .copied()
        .filter(|index| !seen.contains(index))
        .collect::<Vec<_>>();
    missing_enabled.sort_unstable();
    for index in missing_enabled {
        seen.insert(index);
        ordered.push(index);
    }
    for index in 0..analysis_count {
        if seen.insert(index) {
            ordered.push(index);
        }
    }

    let project_scope = project_id.to_string();
    let plan_id = SimulationPlanId::from_namespace(
        LEGACY_ANALYSIS_PLAN_NAMESPACE,
        format!("{project_scope}/simulation-plan/schema-3").as_bytes(),
    );
    let revision = ObjectRevision::INITIAL;
    let staged = ordered
        .into_iter()
        .map(|index| {
            let kind = AnalysisKind::from_legacy_index(index)
                .expect("validated legacy analysis indices map exactly");
            let id = AnalysisInstanceId::from_namespace(
                LEGACY_ANALYSIS_PLAN_NAMESPACE,
                format!(
                    "{project_scope}/simulation-plan/schema-3/{}/slot-0",
                    kind.stable_id()
                )
                .as_bytes(),
            );
            (id, kind, draft_for(kind), enabled.contains(&index))
        })
        .collect::<Vec<_>>();

    let mut instances = Vec::with_capacity(staged.len());
    for (position, (id, kind, draft, enabled)) in staged.iter().cloned().enumerate() {
        let dependencies = if enabled {
            draft
                .prerequisite_roles()
                .iter()
                .filter_map(|prerequisite| {
                    staged[..position]
                        .iter()
                        .rev()
                        .find(|(_, candidate_kind, _, candidate_enabled)| {
                            *candidate_enabled && candidate_kind == prerequisite
                        })
                        .map(|(target, _, _, _)| AnalysisDependency::new(*prerequisite, *target))
                })
                .collect()
        } else {
            Vec::new()
        };
        instances.push(
            AnalysisInstance::supplied(id, kind, draft, enabled, dependencies, revision, revision)
                .map_err(|error| format!("legacy analysis migration failed: {error}"))?,
        );
    }

    let mut plan = SimulationPlan::from_ordered_instances(plan_id, revision, instances)
        .map_err(|error| format!("legacy analysis migration failed: {error}"))?;
    plan.prepare_after_restore();
    Ok(plan)
}
