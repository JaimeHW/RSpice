//! What the project's technology binding resolves to, and the one
//! transaction that runs a callback of the package it pins.

use crate::state::pdk_config::{
    PdkAdministrativeAuthority, ProjectPdkCallbackReceipt, ValidatedPdkTechnologyPackage,
};

use super::Model;

impl Model {
    /// Resolve the project-owned technology contract against both mutable
    /// execution catalogs. No simulation or governed save may infer authority
    /// from a display label, an active registry package, or model paths alone.
    pub fn validate_project_technology_contract(&self) -> Result<(), String> {
        self.technology_inputs()
            .validate_project_technology_contract()
    }

    /// Resolve the exact signed project pin into the physical-layout unit and
    /// stack authority a new or imported Layout document must retain.
    pub fn exact_project_layout_technology_binding(
        &self,
    ) -> Result<crate::state::LayoutTechnologyBinding, String> {
        let package = self.exact_project_pdk_package()?;
        let manifest = package.manifest();
        let database_unit =
            crate::quantity::LayoutDatabaseUnit::from_metres(manifest.database_unit_meters)
                .map_err(|error| format!("Signed project PDK database unit is invalid: {error}"))?;
        crate::state::LayoutTechnologyBinding::try_new(
            manifest.package_id.clone(),
            manifest.revision.clone(),
            package.manifest_digest(),
            package.archive_digest(),
            manifest.technology_name.clone(),
            manifest.stack_name.clone(),
            database_unit,
        )
        .map_err(|error| format!("Signed project PDK layout authority is invalid: {error}"))
    }

    /// Resolve only the currently validated package selected by the exact
    /// signed project pin. Administrative activation is deliberately ignored.
    pub fn exact_project_pdk_package(&self) -> Result<&ValidatedPdkTechnologyPackage, String> {
        self.validate_project_technology_contract()?;
        let signed_pin = self
            .workspace
            .content
            .project
            .technology_binding()
            .and_then(crate::state::ProjectTechnologyBinding::signed_package)
            .expect("validated project technology has an exact signed package");
        self.pdk_config
            .technology_registry
            .validated_packages()
            .iter()
            .find(|package| {
                let manifest = package.manifest();
                manifest
                    .package_id
                    .eq_ignore_ascii_case(signed_pin.package_id())
                    && manifest.revision == signed_pin.revision()
                    && package.manifest_digest() == signed_pin.manifest_digest()
                    && package.archive_digest() == signed_pin.archive_digest()
            })
            .ok_or_else(|| {
                "The signed PDK package the project is pinned to is no longer available".to_owned()
            })
    }

    /// Resolve the exact currently trusted signed package pinned by this
    /// project for read-only workspace overlays.
    pub fn project_signed_technology_package(
        &self,
    ) -> Result<Option<&ValidatedPdkTechnologyPackage>, String> {
        self.pdk_config
            .technology_registry
            .project_signed_technology_package(self.workspace.content.project.technology_binding())
    }

    /// Execute one signed PDK callback for the exact package pinned by the
    /// project and commit its evidence as one project revision transaction.
    ///
    /// Inputs are the validated active simulation-plan variables normalized to
    /// finite SI values. Administrative activation is never consulted. Guest
    /// output remains derived metadata inside the immutable receipt; it does
    /// not silently mutate models, layout, or simulation configuration.
    pub fn execute_project_pdk_callback(
        &mut self,
        callback_id: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<ProjectPdkCallbackReceipt, String> {
        for (field, value, maximum) in [
            ("actor ID", authority.actor_id.as_str(), 256_usize),
            ("authority ID", authority.authority_id.as_str(), 256_usize),
            ("reason", reason, 2_048_usize),
        ] {
            if value.is_empty()
                || value != value.trim()
                || value.len() > maximum
                || value.chars().any(char::is_control)
            {
                return Err(format!(
                    "Project callback {field} must be nonempty, trimmed, control-free, and at most {maximum} bytes"
                ));
            }
        }
        self.technology_inputs()
            .validate_project_technology_contract()?;

        let project_binding = self
            .workspace
            .content
            .project
            .technology_binding()
            .expect("validated project technology has an exact binding");
        let signed_pin = project_binding
            .signed_package()
            .expect("validated project technology has an exact signed package");
        let package_binding = crate::state::pdk_config::PdkTechnologyBinding {
            package_id: signed_pin.package_id().to_owned(),
            revision: signed_pin.revision().to_owned(),
            manifest_digest: signed_pin.manifest_digest(),
        };
        let plan = self.sim_setup.stable_analysis_plan().map_err(|error| {
            format!("Project callback requires an authoritative active simulation plan: {error}")
        })?;
        let plan_id = plan.id();
        let plan_revision = plan.revision();
        let mut project_parameters = std::collections::BTreeMap::new();
        if let Some(payload) = self.workspace.content.plan_data(plan_id) {
            for (index, variable) in payload.design_variables.iter().enumerate() {
                variable.validate().map_err(|error| {
                    format!("Active-plan design variable[{index}] is invalid: {error}")
                })?;
                let value = variable.resolved_value_si().map_err(|error| {
                    format!(
                        "Active-plan design variable '{}' is unresolved: {error}",
                        variable.name
                    )
                })?;
                if project_parameters
                    .insert(variable.name.clone(), format!("{value:.17e}"))
                    .is_some()
                {
                    return Err(format!(
                        "Active plan repeats project callback parameter '{}'",
                        variable.name
                    ));
                }
            }
        }
        let input = crate::state::pdk_config::PdkCallbackExecutionInput { project_parameters };
        let execution = self
            .pdk_config
            .technology_registry
            .execute_callback_for_binding(
                &package_binding,
                signed_pin.archive_digest(),
                callback_id,
                &input,
            )
            .map_err(|error| format!("Signed PDK callback execution was rejected: {error}"))?;
        let receipt = self
            .workspace
            .content
            .commit_pdk_callback_execution(
                plan_id,
                plan_revision,
                authority,
                reason,
                input,
                execution,
            )
            .map_err(|error| format!("Signed PDK callback result was not recorded: {error}"))?;

        self.workspace.content.project_metadata_dirty = true;
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.netlist_artifacts.current_generation_input_digest = None;
        self.clear_project_design_history();
        Ok(receipt)
    }
}
