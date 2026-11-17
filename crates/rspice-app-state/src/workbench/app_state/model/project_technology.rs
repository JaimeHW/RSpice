//! What the project's technology binding resolves to.
//!
//! # Proving the contract is done on change, not on a frame
//!
//! The proof hashes every retained byte of the attached model library and
//! walks every layout shape, so a surface that shows the verdict reads
//! [`Model::project_technology_verdict`], which proves it again only when the
//! project's binding, the model catalogue, the validated PDK packages or the
//! layouts have changed since the last proof. Saving and running prove it
//! afresh through [`Model::validate_project_technology_contract`].

use std::cell::RefCell;
use std::sync::Arc;

use crate::product::{ContentDigest, ObjectRevision};
use crate::state::ProjectTechnologyBinding;
use crate::state::pdk_config::ValidatedPdkTechnologyPackage;

use super::Model;

/// The project's technology contract as last proven, kept beside what it was
/// proven against. Runtime only, and a clone shares the proof: a clone is the
/// same content, so the same verdict.
#[derive(Debug, Clone, Default)]
pub struct ProjectTechnologyVerdict(RefCell<Option<Arc<ProvenContract>>>);

#[derive(Debug)]
struct ProvenContract {
    project_id: crate::product::ProjectId,
    project_revision: ObjectRevision,
    binding: Option<ProjectTechnologyBinding>,
    authority_receipts: usize,
    catalog_stamp: u64,
    packages: Vec<(ContentDigest, ContentDigest)>,
    layout_stamp: u64,
    verdict: Result<(), String>,
}

impl ProvenContract {
    fn of(model: &Model, verdict: Result<(), String>) -> Self {
        let project = &model.workspace.content.project;
        Self {
            project_id: project.id(),
            project_revision: project.revision(),
            binding: project.technology_binding().cloned(),
            authority_receipts: project.technology_change_audit().len(),
            catalog_stamp: model.model_library_manager.catalog_stamp(),
            packages: package_identities(model).collect(),
            layout_stamp: model.workspace.content.physical_layout_stamp(),
            verdict,
        }
    }

    /// Whether every input the proof read still stands as it was proven.
    fn stands(&self, model: &Model) -> bool {
        let project = &model.workspace.content.project;
        self.project_id == project.id()
            && self.project_revision == project.revision()
            && self.authority_receipts == project.technology_change_audit().len()
            && self.catalog_stamp == model.model_library_manager.catalog_stamp()
            && self.layout_stamp == model.workspace.content.physical_layout_stamp()
            && self.binding.as_ref() == project.technology_binding()
            && package_identities(model).eq(self.packages.iter().copied())
    }
}

/// Each validated PDK package by the digests that name its exact content.
fn package_identities(model: &Model) -> impl Iterator<Item = (ContentDigest, ContentDigest)> + '_ {
    model
        .pdk_config
        .technology_registry
        .validated_packages()
        .iter()
        .map(|package| (package.manifest_digest(), package.archive_digest()))
}

impl Model {
    /// [`Self::validate_project_technology_contract`] as last proven, proven
    /// again only when one of its inputs has changed since. What a surface
    /// paints reads this; what saves or runs proves the contract afresh.
    pub fn project_technology_verdict(&self) -> Result<(), String> {
        if let Some(proven) = self.technology_verdict.0.borrow().as_ref()
            && proven.stands(self)
        {
            return proven.verdict.clone();
        }
        let verdict = self.validate_project_technology_contract();
        *self.technology_verdict.0.borrow_mut() =
            Some(Arc::new(ProvenContract::of(self, verdict.clone())));
        verdict
    }

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
