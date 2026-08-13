//! Why this plan would need an attached project technology.
//!
//! A project need not have a technology. This module makes that conditional
//! exact: it names the authored entities that cannot resolve without a signed
//! PDK — a non-typical reference process, an enabled corner analysis asking
//! for non-typical process sections, a physical layout — so the gate can
//! demand a technology only when the plan actually consumes one.
//!
//! The demand is computed from the plan alone, before any netlist exists.
//! Nothing here reads a generated deck, a sealed model source or a run
//! result; a project that fails this gate must fail it before preparation
//! starts, and preflight must be able to report it next to the plan's other
//! blockers.
//!
//! Demand is deliberately narrow. Monte Carlo, reliability, temperature
//! sweeps, technology-defined global nets, signed-PDK Verilog-A and model
//! references named in a netlist all resolve against the attached model
//! library, and none of them requires the project itself to own a signed
//! technology.

use super::SimSetupState;
use crate::product::AnalysisInstanceId;
use crate::simulation::dialog::corner::ProcessCorner;
use crate::simulation::plan::AnalysisKind;
use crate::simulation::run_set::RunSetDimensionKind;

/// One authored entity that requires an attached project technology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TechnologyDemandReason {
    /// The plan's reference point selects a non-typical process section.
    NonTtReference(ProcessCorner),
    /// The global Simulation Studio Run Set resolves non-typical sections.
    GlobalRunSetSections { sections: Vec<ProcessCorner> },
    /// An enabled corner analysis resolves to non-typical process sections.
    CornerSections {
        instance: AnalysisInstanceId,
        sections: Vec<ProcessCorner>,
    },
    /// Physical layout is authored against the signed PDK's layer stack, so it
    /// needs the pin whatever the plan's process sections are.
    PhysicalLayout { documents: Vec<String> },
}

impl TechnologyDemandReason {
    /// Preflight's `Observed` cell: the entity that demands a technology.
    pub(crate) fn observed(&self) -> String {
        let clause = self.clause();
        let mut characters = clause.chars();
        characters.next().map_or_else(String::new, |first| {
            first.to_uppercase().collect::<String>() + characters.as_str()
        })
    }

    /// Preflight's `Required` cell: what an attached technology must define.
    pub(crate) fn required(&self) -> String {
        match self {
            Self::NonTtReference(process) => format!(
                "An attached project technology defining the {} process section",
                process.short_name()
            ),
            Self::GlobalRunSetSections { sections } => format!(
                "An attached project technology defining the {} process {}",
                section_list(sections),
                section_noun(sections.len())
            ),
            Self::CornerSections { sections, .. } => format!(
                "An attached project technology defining the {} process {}",
                section_list(sections),
                section_noun(sections.len())
            ),
            Self::PhysicalLayout { .. } => {
                "An attached project technology whose signed PDK owns the layout layer stack"
                    .to_owned()
            }
        }
    }

    /// The same fact as one clause of a joined one-line block reason.
    fn clause(&self) -> String {
        match self {
            Self::NonTtReference(process) => {
                format!("reference process is {}", process.short_name())
            }
            Self::GlobalRunSetSections { sections } => format!(
                "global Run Set requests {} process {}",
                section_list(sections),
                section_noun(sections.len())
            ),
            Self::CornerSections { instance, sections } => format!(
                "Corner analysis '{instance}' requests {} process {}",
                section_list(sections),
                section_noun(sections.len())
            ),
            Self::PhysicalLayout { documents } => match documents.as_slice() {
                [single] => format!("physical layout '{single}' requires a signed technology"),
                _ => format!(
                    "physical layouts {} require a signed technology",
                    quoted_list(documents)
                ),
            },
        }
    }
}

/// Everything in the authored plan that requires an attached technology.
#[derive(Debug, Default)]
pub(crate) struct TechnologyDemand {
    reasons: Vec<TechnologyDemandReason>,
}

impl TechnologyDemand {
    /// True when the plan runs without a project technology.
    pub(crate) fn is_empty(&self) -> bool {
        self.reasons.is_empty()
    }

    /// Every demanding entity, in authored order.
    pub(crate) fn reasons(&self) -> &[TechnologyDemandReason] {
        &self.reasons
    }

    /// One line naming every demanding entity, for the run gate. `None` when
    /// nothing in the plan demands a technology.
    pub(crate) fn block_reason(&self) -> Option<String> {
        (!self.reasons.is_empty()).then(|| {
            format!(
                "This plan requires an attached project technology: {}.",
                self.reasons
                    .iter()
                    .map(TechnologyDemandReason::clause)
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        })
    }
}

/// Resolve what the authored plan needs a project technology for.
pub(crate) fn technology_demand(
    sim_setup: &SimSetupState,
    workspace: &crate::state::ProjectWorkspace,
) -> TechnologyDemand {
    let mut reasons = Vec::new();
    let reference = sim_setup.reference_pvt.process;
    if reference != ProcessCorner::TT {
        reasons.push(TechnologyDemandReason::NonTtReference(reference));
    }
    if let Some(reason) = global_run_set_section_reason(sim_setup) {
        reasons.push(reason);
    }
    reasons.extend(corner_section_reasons(sim_setup));
    let documents: Vec<String> = workspace
        .physical_layout_documents()
        .keys()
        .cloned()
        .collect();
    if !documents.is_empty() {
        reasons.push(TechnologyDemandReason::PhysicalLayout { documents });
    }
    TechnologyDemand { reasons }
}

/// The non-typical process sections the global Run Set applies to every plan
/// analysis. Invalid declarations are reported by Run Set validation and do
/// not manufacture a second technology blocker here.
fn global_run_set_section_reason(sim_setup: &SimSetupState) -> Option<TechnologyDemandReason> {
    if sim_setup
        .run_set
        .enabled_dimension_of(RunSetDimensionKind::ProcessSection)
        .is_none()
    {
        return None;
    }
    let config = sim_setup
        .run_set
        .to_corner_config(
            crate::simulation::dialog::corner::CornerBaseAnalysis::Op,
            sim_setup.reference_pvt,
        )
        .ok()?;
    let sections = config
        .process_corners
        .into_iter()
        .filter(|section| *section != ProcessCorner::TT)
        .collect::<Vec<_>>();
    (!sections.is_empty()).then_some(TechnologyDemandReason::GlobalRunSetSections { sections })
}

/// The non-typical process sections each enabled corner analysis resolves to.
///
/// A plan that cannot freeze and a draft that cannot resolve are preflight
/// blockers in their own right; neither may also manufacture technology
/// demand, so every resolution failure here contributes nothing.
fn corner_section_reasons(sim_setup: &SimSetupState) -> Vec<TechnologyDemandReason> {
    let Ok(plan) = sim_setup.stable_analysis_plan() else {
        return Vec::new();
    };
    // Freezing keeps this on the enabled set and the instance projection the
    // executor runs, rather than a second reading of the authored plan.
    let Ok(frozen) = plan.freeze() else {
        return Vec::new();
    };
    let mut reasons = Vec::new();
    for instance in frozen
        .instances()
        .iter()
        .filter(|instance| instance.kind() == AnalysisKind::Corner)
    {
        let Ok(projection) = sim_setup.frozen_instance_projection(&frozen, instance) else {
            continue;
        };
        // Without a declared process axis every point resolves through the
        // reference section, which `NonTtReference` already owns.
        if projection
            .corner
            .run_set
            .enabled_dimension_of(RunSetDimensionKind::ProcessSection)
            .is_none()
        {
            continue;
        }
        let Ok(config) = projection.corner.to_config(sim_setup.reference_pvt) else {
            continue;
        };
        let sections: Vec<ProcessCorner> = config
            .process_corners
            .iter()
            .copied()
            .filter(|section| *section != ProcessCorner::TT)
            .collect();
        if !sections.is_empty() {
            reasons.push(TechnologyDemandReason::CornerSections {
                instance: instance.id(),
                sections,
            });
        }
    }
    reasons
}

/// `SS, FF`
fn section_list(sections: &[ProcessCorner]) -> String {
    sections
        .iter()
        .map(ProcessCorner::short_name)
        .collect::<Vec<_>>()
        .join(", ")
}

/// `'top', 'pads'`
fn quoted_list(documents: &[String]) -> String {
    documents
        .iter()
        .map(|document| format!("'{document}'"))
        .collect::<Vec<_>>()
        .join(", ")
}

const fn section_noun(count: usize) -> &'static str {
    if count == 1 { "section" } else { "sections" }
}

#[cfg(test)]
mod tests {
    use super::{TechnologyDemand, TechnologyDemandReason};
    use crate::product::AnalysisInstanceId;
    use crate::simulation::dialog::corner::{CornerDialogState, ProcessCorner};
    use crate::simulation::plan::AnalysisDraft;
    use crate::simulation::run_set::RunSetDimensionKind;
    use crate::workbench::app_state::AppState;

    fn insert_corner(state: &mut AppState, corner: CornerDialogState) -> AnalysisInstanceId {
        let plan = state
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("a default state owns a stable plan");
        let position = plan.instances().len();
        plan.insert_draft_with_id(
            AnalysisInstanceId::new(),
            AnalysisDraft::Corner(corner),
            true,
            position,
        )
        .expect("a corner analysis has no prerequisites")
        .0
    }

    /// The default space declares SS/TT/FF; this drops the axis entirely, so
    /// every point resolves through the plan's reference section.
    fn corner_without_a_process_axis() -> CornerDialogState {
        let mut corner = CornerDialogState::default();
        for dimension in &mut corner.run_set.dimensions {
            if dimension.kind == RunSetDimensionKind::ProcessSection {
                dimension.enabled = false;
            }
        }
        corner
    }

    fn sole_reason(demand: &TechnologyDemand) -> &TechnologyDemandReason {
        assert_eq!(
            demand.reasons().len(),
            1,
            "expected exactly one reason: {:?}",
            demand.reasons()
        );
        &demand.reasons()[0]
    }

    fn disable_global_process_axis(state: &mut AppState) {
        for dimension in &mut state.sim_setup.run_set.dimensions {
            if dimension.kind == RunSetDimensionKind::ProcessSection {
                dimension.enabled = false;
            }
        }
    }

    #[test]
    fn a_nominal_global_run_set_demands_no_technology() {
        let mut state = AppState::default();
        disable_global_process_axis(&mut state);
        let demand = state.technology_demand();
        assert!(demand.is_empty(), "{:?}", demand.reasons());
        assert_eq!(demand.block_reason(), None);
        assert_eq!(state.technology_gate_block_reason(), Ok(()));
    }

    #[test]
    fn the_default_global_pvt_run_set_demands_its_non_typical_sections() {
        let mut state = AppState::default();
        for dimension in &mut state.sim_setup.run_set.dimensions {
            dimension.enabled = true;
        }
        let demand = state.technology_demand();
        assert_eq!(
            sole_reason(&demand),
            &TechnologyDemandReason::GlobalRunSetSections {
                sections: vec![ProcessCorner::SS, ProcessCorner::FF],
            }
        );
        assert_eq!(
            sole_reason(&demand).observed(),
            "Global Run Set requests SS, FF process sections"
        );
    }

    #[test]
    fn a_non_typical_reference_process_demands_its_section() {
        let mut state = AppState::default();
        disable_global_process_axis(&mut state);
        state
            .sim_setup
            .set_reference_pvt(ProcessCorner::SS, 27.0)
            .expect("the reference point is valid");
        let demand = state.technology_demand();
        assert_eq!(
            sole_reason(&demand),
            &TechnologyDemandReason::NonTtReference(ProcessCorner::SS)
        );
        assert_eq!(sole_reason(&demand).observed(), "Reference process is SS");
        let required = sole_reason(&demand).required();
        assert!(required.contains("SS process section"), "{required}");
        let blocked = state
            .technology_gate_block_reason()
            .expect_err("no technology is attached");
        assert_eq!(
            blocked,
            "This plan requires an attached project technology: reference process is SS."
        );
    }

    #[test]
    fn an_enabled_corner_analysis_demands_its_non_typical_sections() {
        let mut state = AppState::default();
        disable_global_process_axis(&mut state);
        let id = insert_corner(&mut state, CornerDialogState::default());
        let demand = state.technology_demand();
        let TechnologyDemandReason::CornerSections { instance, sections } = sole_reason(&demand)
        else {
            panic!("expected corner sections: {:?}", demand.reasons());
        };
        assert_eq!(*instance, id);
        assert_eq!(sections, &[ProcessCorner::SS, ProcessCorner::FF]);
        let observed = sole_reason(&demand).observed();
        assert_eq!(
            observed,
            format!("Corner analysis '{id}' requests SS, FF process sections")
        );
        let blocked = state
            .technology_gate_block_reason()
            .expect_err("no technology is attached");
        assert_eq!(
            blocked,
            format!(
                "This plan requires an attached project technology: Corner analysis '{id}' requests SS, FF process sections."
            )
        );
    }

    #[test]
    fn a_corner_without_a_process_axis_demands_nothing_at_a_typical_reference() {
        let mut state = AppState::default();
        disable_global_process_axis(&mut state);
        insert_corner(&mut state, corner_without_a_process_axis());
        let demand = state.technology_demand();
        assert!(demand.is_empty(), "{:?}", demand.reasons());
        assert_eq!(state.technology_gate_block_reason(), Ok(()));
    }

    #[test]
    fn a_corner_without_a_process_axis_does_not_restate_the_reference_section() {
        let mut state = AppState::default();
        disable_global_process_axis(&mut state);
        state
            .sim_setup
            .set_reference_pvt(ProcessCorner::SS, 27.0)
            .expect("the reference point is valid");
        insert_corner(&mut state, corner_without_a_process_axis());
        let demand = state.technology_demand();
        assert_eq!(
            sole_reason(&demand),
            &TechnologyDemandReason::NonTtReference(ProcessCorner::SS)
        );
    }

    /// Attaching a layout document needs an exact signed pin, which is what
    /// this reason reports the absence of; the cells it renders are stated
    /// directly instead.
    #[test]
    fn a_physical_layout_names_itself_and_the_signed_stack() {
        let reason = TechnologyDemandReason::PhysicalLayout {
            documents: vec!["top".to_owned()],
        };
        assert_eq!(
            reason.observed(),
            "Physical layout 'top' requires a signed technology"
        );
        let required = reason.required();
        assert!(required.contains("signed PDK"), "{required}");
        let several = TechnologyDemandReason::PhysicalLayout {
            documents: vec!["top".to_owned(), "pads".to_owned()],
        };
        assert_eq!(
            several.observed(),
            "Physical layouts 'top', 'pads' require a signed technology"
        );
    }
}
