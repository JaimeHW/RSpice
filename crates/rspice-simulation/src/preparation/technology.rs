//! Determine whether the authored plan and physical layouts require a signed technology.

use rspice_app_types::product::ProcessCorner;
use rspice_simulation_contract::run_set::RunSetDimensionKind;
use rspice_simulation_contract::setup_state::SimulationSetup;

/// One authored entity that requires an attached project technology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TechnologyDemandReason {
    /// The plan's reference point selects a non-typical process section.
    NonTtReference(ProcessCorner),
    /// The global Simulation Studio Run Set resolves non-typical sections.
    GlobalRunSetSections { sections: Vec<ProcessCorner> },
    /// Physical layout is authored against the signed PDK's layer stack, so it
    /// needs the pin whatever the plan's process sections are.
    PhysicalLayout { documents: Vec<String> },
}

impl TechnologyDemandReason {
    /// Preflight's `Observed` cell: the entity that demands a technology.
    pub fn observed(&self) -> String {
        let clause = self.clause();
        let mut characters = clause.chars();
        characters.next().map_or_else(String::new, |first| {
            first.to_uppercase().collect::<String>() + characters.as_str()
        })
    }

    /// Preflight's `Required` cell: what an attached technology must define.
    pub fn required(&self) -> String {
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
pub struct TechnologyDemand {
    reasons: Vec<TechnologyDemandReason>,
}

impl TechnologyDemand {
    /// True when the plan runs without a project technology.
    pub fn is_empty(&self) -> bool {
        self.reasons.is_empty()
    }

    /// Every demanding entity, in authored order.
    pub fn reasons(&self) -> &[TechnologyDemandReason] {
        &self.reasons
    }

    /// One line naming every demanding entity, for the run gate. `None` when
    /// nothing in the plan demands a technology.
    pub fn block_reason(&self) -> Option<String> {
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
pub fn technology_demand(
    sim_setup: &SimulationSetup,
    physical_layouts: &std::collections::BTreeMap<
        String,
        rspice_design::physical_layout::PhysicalLayoutDocument,
    >,
) -> TechnologyDemand {
    let mut reasons = Vec::new();
    let reference = sim_setup.reference_pvt.process;
    if reference != ProcessCorner::TT {
        reasons.push(TechnologyDemandReason::NonTtReference(reference));
    }
    if let Some(reason) = global_run_set_section_reason(sim_setup) {
        reasons.push(reason);
    }
    let documents: Vec<String> = physical_layouts.keys().cloned().collect();
    if !documents.is_empty() {
        reasons.push(TechnologyDemandReason::PhysicalLayout { documents });
    }
    TechnologyDemand { reasons }
}

/// The non-typical process sections the global Run Set applies to every plan
/// analysis. Invalid declarations are reported by Run Set validation and do
/// not manufacture a second technology blocker here.
fn global_run_set_section_reason(sim_setup: &SimulationSetup) -> Option<TechnologyDemandReason> {
    sim_setup
        .run_set
        .enabled_dimension_of(RunSetDimensionKind::ProcessSection)?;
    let config = sim_setup
        .run_set
        .to_corner_config(
            rspice_simulation_contract::corner_config::CornerBaseAnalysis::Op,
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

// A corner analysis used to contribute its own section demand, read from the
// space it carried. It no longer carries one: the plan declares the space, and
// `global_run_set_section_reason` above already reports exactly the sections
// every analysis — corner instances included — will materialize. A second
// reason derived from the same declaration would demand the same technology
// twice and name it once per corner instance.

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
