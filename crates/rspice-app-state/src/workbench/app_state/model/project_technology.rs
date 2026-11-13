//! What the project's technology binding resolves to.

use crate::state::pdk_config::ValidatedPdkTechnologyPackage;

use super::Model;

impl Model {
    /// The reference-process cards a run of the active plan reads: the
    /// libraries the plan binds, joined by the PDK package while the
    /// project's technology is in effect. A technology failing its own
    /// contract has its own finding, so that failure answers nothing here.
    pub fn run_reference_model_cards(&self) -> Result<Vec<String>, String> {
        let technology = self.technology_inputs();
        let sealed = if !technology.project_technology_in_effect() {
            self.model_library_manager
                .seal_execution_sources_for_plan(&self.sim_setup.model_bindings)?
        } else if technology.validate_project_technology_contract().is_ok() {
            technology.seal_project_execution_model_sources()?
        } else {
            return Ok(Vec::new());
        };
        sealed.reference_process_model_cards(self.sim_setup.reference_pvt.process)
    }

    /// Resolve the project-owned technology contract against both mutable
    /// execution catalogs. No simulation or governed save may infer authority
    /// from a display label, an active registry package, or model paths alone.
    pub fn validate_project_technology_contract(&self) -> Result<(), String> {
        self.technology_inputs()
            .validate_project_technology_contract()
    }

    /// Resolve the exact project PDK pin into the physical-layout unit and
    /// stack authority a new or imported Layout document must retain.
    pub fn exact_project_layout_technology_binding(
        &self,
    ) -> Result<crate::state::LayoutTechnologyBinding, String> {
        let package = self.exact_project_pdk_package()?;
        let manifest = package.manifest();
        let database_unit =
            crate::quantity::LayoutDatabaseUnit::from_metres(manifest.database_unit_meters)
                .map_err(|error| format!("Project PDK database unit is invalid: {error}"))?;
        crate::state::LayoutTechnologyBinding::try_new(
            manifest.package_id.clone(),
            manifest.revision.clone(),
            package.manifest_digest(),
            package.archive_digest(),
            manifest.technology_name.clone(),
            manifest.stack_name.clone(),
            database_unit,
        )
        .map_err(|error| format!("Project PDK layout authority is invalid: {error}"))
    }

    /// Resolve only the currently validated package selected by the exact
    /// project PDK pin. Administrative activation is deliberately ignored.
    pub fn exact_project_pdk_package(&self) -> Result<&ValidatedPdkTechnologyPackage, String> {
        self.validate_project_technology_contract()?;
        let pdk_pin = self
            .workspace
            .content
            .project
            .technology_binding()
            .and_then(crate::state::ProjectTechnologyBinding::pdk_package)
            .expect("validated project technology has an exact PDK package");
        self.pdk_config
            .technology_registry
            .validated_packages()
            .iter()
            .find(|package| {
                let manifest = package.manifest();
                manifest
                    .package_id
                    .eq_ignore_ascii_case(pdk_pin.package_id())
                    && manifest.revision == pdk_pin.revision()
                    && package.manifest_digest() == pdk_pin.manifest_digest()
                    && package.archive_digest() == pdk_pin.archive_digest()
            })
            .ok_or_else(|| {
                "The PDK package the project is pinned to is no longer available".to_owned()
            })
    }

    /// Resolve the exact currently validated PDK package pinned by this
    /// project for read-only workspace overlays.
    pub fn project_pdk_package(&self) -> Result<Option<&ValidatedPdkTechnologyPackage>, String> {
        self.pdk_config
            .technology_registry
            .project_pdk_package(self.workspace.content.project.technology_binding())
    }
}
