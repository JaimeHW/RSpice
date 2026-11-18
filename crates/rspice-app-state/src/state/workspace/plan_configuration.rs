//! The design a simulation plan runs: the project's configurations as the
//! plan names them.
//!
//! A plan follows the configuration the project marks "Used for runs" unless
//! it names one of its own. Every run-side reader asks here, so the root a
//! run checks, the hierarchy it elaborates, the deck it netlists and the
//! configuration it records are the plan's, and the project's mark is left
//! where the Configuration page put it.

use std::cell::RefCell;
use std::ops::Deref;

use rspice_design::configuration_set::ConfigurationSetCatalog;
use rspice_design::projection::ProjectionSource;
use rspice_design::stamped::Stamped;
use rspice_simulation_contract::design_configuration::DesignConfiguration;

use super::*;

/// The configuration catalog a plan's runs elaborate under.
#[derive(Debug)]
pub enum PlanConfigurationSets<'a> {
    /// The project's own, as it stands.
    Project(&'a Stamped<ConfigurationSetCatalog>),
    /// A copy with the plan's configuration marked for runs.
    Plan(Stamped<ConfigurationSetCatalog>),
}

impl Deref for PlanConfigurationSets<'_> {
    type Target = Stamped<ConfigurationSetCatalog>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Project(catalog) => catalog,
            Self::Plan(catalog) => catalog,
        }
    }
}

/// The plan's view of the catalog, kept under one stamp for as long as the
/// project's catalog and the plan's choice stand, so the projections and
/// checks keyed on it are reused rather than redone every frame.
#[derive(Debug, Clone, Default)]
pub(super) struct PlanConfigurationMemo(
    RefCell<Option<(u64, DesignConfiguration, Stamped<ConfigurationSetCatalog>)>>,
);

impl ProjectWorkspace {
    /// The catalog a plan naming `design` runs under, or why it cannot run.
    pub fn plan_configuration_sets(
        &self,
        design: DesignConfiguration,
    ) -> Result<PlanConfigurationSets<'_>, PlanConfigurationError> {
        let stamp = self.content.configuration_sets.stamp();
        if let Some((memo_stamp, memo_design, catalog)) =
            &*self.session.plan_configuration.0.borrow()
            && *memo_stamp == stamp
            && *memo_design == design
        {
            return Ok(PlanConfigurationSets::Plan(catalog.clone()));
        }
        let Some(candidate) = self.content.configuration_sets_for(design)? else {
            return Ok(PlanConfigurationSets::Project(
                &self.content.configuration_sets,
            ));
        };
        let candidate = Stamped::from(candidate);
        *self.session.plan_configuration.0.borrow_mut() = Some((stamp, design, candidate.clone()));
        Ok(PlanConfigurationSets::Plan(candidate))
    }

    /// The root a plan naming `design` runs: its configuration's root, the
    /// project's top cell under the built-in default. A plan whose
    /// configuration cannot be run is refused where it would run; until then
    /// it is described by the project's own root.
    pub fn plan_simulation_root(&self, design: DesignConfiguration) -> CellViewRef {
        self.plan_configuration_sets(design).map_or_else(
            |_| self.content.simulation_root_reference(),
            |catalog| self.content.simulation_root_under(&catalog),
        )
    }

    /// The projection of the design a plan naming `design` runs, resolved
    /// or not, for a reader that states what does not resolve.
    pub fn plan_inspect_design_projection(
        &self,
        design: DesignConfiguration,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Result<std::sync::Arc<DesignProjection>, ConfigurationExecutionPlanError> {
        match self.plan_configuration_sets(design).map_err(|error| {
            ConfigurationExecutionPlanError::PlanConfiguration(error.to_string())
        })? {
            PlanConfigurationSets::Project(_) => {
                self.inspect_design_projection(libraries, active_reference, active_schematic)
            }
            PlanConfigurationSets::Plan(catalog) => self.inspect_candidate_design_projection(
                &catalog,
                libraries,
                active_reference,
                active_schematic,
                &self.session.plan_projection_cache,
            ),
        }
    }

    /// The executable projection of the design a plan naming `design` runs.
    pub fn plan_execution_projection(
        &self,
        design: DesignConfiguration,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Result<ConfigurationExecutionProjection, ConfigurationExecutionPlanError> {
        self.plan_inspect_design_projection(design, libraries, active_reference, active_schematic)?
            .into_execution()
    }
}

/// How the configuration a run recorded reads now: its name when it ran, and
/// whether the project has changed or deleted it since, so a result from a
/// configuration that moved never passes for one from the current design.
pub fn recorded_configuration_words(
    catalog: &ConfigurationSetCatalog,
    recorded: &rspice_results::run_receipt::RunConfiguration,
) -> String {
    let Some(id) = recorded.id else {
        return recorded.name.clone();
    };
    match catalog.find(id) {
        None => format!("{} (deleted since)", recorded.name),
        Some(configuration) if configuration.revision() != recorded.revision => {
            format!("{} (changed since)", recorded.name)
        }
        Some(_) => recorded.name.clone(),
    }
}
