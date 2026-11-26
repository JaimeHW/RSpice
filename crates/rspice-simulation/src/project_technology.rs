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
        let (package_binding, archive_digest) = self
            .pinned_pdk()
            .expect("validated project technology has an exact PDK package");
        let sealed_pdk = self
            .registry
            .seal_model_sources_for_binding(&package_binding, archive_digest)
            .map_err(|error| {
                format!("The PDK's model files cannot be kept with the run: {error}")
            })?;
        crate::model_sources::seal_plan_execution_sources(
            self.models,
            self.resolutions,
            &self.sim_setup.model_bindings,
        )?
        .with_pdk_model_sources(sealed_pdk)
    }

    /// What the reference process decides in this project's run, read from
    /// the sources the run binds: the plan's model libraries, each with the
    /// corner its binding selects, and, under a project technology, the
    /// processes the pinned PDK contracts model sources for.
    ///
    /// A binding to a library the catalog no longer holds, or a pin no
    /// trusted package answers, is left out: the run refuses either before it
    /// binds a process.
    #[must_use]
    pub fn reference_process_choice(&self) -> crate::model_sources::ReferenceProcessChoice {
        let libraries = self
            .sim_setup
            .model_bindings
            .iter()
            .filter_map(|binding| {
                let library = self.models.get_library(&binding.library_name)?;
                Some((library, binding.selected_corner.as_deref()))
            })
            .collect::<Vec<_>>();
        let pdk = self
            .project_technology_in_effect()
            .then(|| self.pinned_pdk())
            .flatten()
            .and_then(|(binding, archive_digest)| {
                self.registry.validated_packages().iter().find(|package| {
                    package.archive_digest() == archive_digest && package.binding() == binding
                })
            })
            .map(|package| {
                package
                    .manifest()
                    .model_sources
                    .iter()
                    .filter(|contract| !contract.sources.is_empty())
                    .map(|contract| contract.process)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        crate::model_sources::ReferenceProcessChoice::of(&libraries, &pdk)
    }

    /// The PDK package the project pins, and the archive it pins it to.
    fn pinned_pdk(
        &self,
    ) -> Option<(
        rspice_model_library::pdk::contracts::PdkTechnologyBinding,
        rspice_vocabulary::product::ContentDigest,
    )> {
        let pin = self.project.technology_binding()?.pdk_package()?;
        Some((
            rspice_model_library::pdk::contracts::PdkTechnologyBinding {
                package_id: pin.package_id().to_owned(),
                revision: pin.revision().to_owned(),
                manifest_digest: pin.manifest_digest(),
            },
            pin.archive_digest(),
        ))
    }
}
