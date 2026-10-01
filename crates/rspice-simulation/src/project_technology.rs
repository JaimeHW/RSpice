//! Project technology validation and source sealing over borrowed authorities.

pub struct ProjectTechnologyInputs<'a> {
    pub project: &'a rspice_project_contract::ProjectDescriptor,
    pub sim_setup: &'a rspice_simulation_contract::setup_state::SimulationSetup,
    pub models: &'a rspice_model_library::ModelCatalog,
    pub resolutions: &'a rspice_model_library::ModelResolutionRecords,
    pub registry: &'a crate::pdk::PdkTechnologyRegistry,
    pub layouts: &'a std::collections::BTreeMap<
        String,
        rspice_design::physical_layout::PhysicalLayoutDocument,
    >,
}

impl ProjectTechnologyInputs<'_> {
    pub fn validate_project_technology_contract(&self) -> Result<(), String> {
        let binding = self.project.validated_technology_binding()?;
        crate::pdk::validate_project_technology_inputs(
            binding,
            self.models,
            self.registry.validated_packages(),
            self.layouts,
        )
    }

    pub fn project_technology_in_effect(&self) -> bool {
        self.project.has_audited_technology_binding()
    }

    pub fn technology_demand(&self) -> crate::preparation::TechnologyDemand {
        crate::preparation::technology_demand(self.sim_setup, self.layouts)
    }

    pub fn technology_gate_block_reason(&self) -> Result<(), String> {
        if self.project.technology_binding().is_some() {
            return self.validate_project_technology_contract();
        }
        self.technology_demand().block_reason().map_or(Ok(()), Err)
    }

    pub fn seal_project_execution_model_sources(
        &self,
    ) -> Result<crate::model_sources::SealedModelExecutionSources, String> {
        self.validate_project_technology_contract()?;
        let project_binding = self
            .project
            .technology_binding()
            .expect("validated project technology has an exact binding");
        let signed_pin = project_binding
            .signed_package()
            .expect("validated project technology has an exact signed package");
        let package_binding = rspice_model_library::pdk::contracts::PdkTechnologyBinding {
            package_id: signed_pin.package_id().to_owned(),
            revision: signed_pin.revision().to_owned(),
            manifest_digest: signed_pin.manifest_digest(),
        };
        let sealed_pdk = self
            .registry
            .seal_model_sources_for_binding(&package_binding, signed_pin.archive_digest())
            .map_err(|error| {
                format!("Signed PDK model sources cannot be sealed for project execution: {error}")
            })?;
        crate::model_sources::seal_plan_execution_sources(
            self.models,
            self.resolutions,
            &self.sim_setup.model_bindings,
        )?
        .with_pdk_model_sources(sealed_pdk)
    }
}
