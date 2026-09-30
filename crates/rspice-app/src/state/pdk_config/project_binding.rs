//! Bind project pin validation to the registry's current runtime package cache.
use super::{PdkTechnologyRegistry, ValidatedPdkTechnologyPackage};
use rspice_project::{ProjectTechnologyBinding, TechnologyBindingError};

impl PdkTechnologyRegistry {
    pub(crate) fn project_signed_technology_package(
        &self,
        binding: Option<&ProjectTechnologyBinding>,
    ) -> Result<Option<&ValidatedPdkTechnologyPackage>, String> {
        rspice_simulation::pdk::project_signed_technology_package(
            self.validated_packages(),
            binding,
        )
    }

    pub(crate) fn validate_project_binding(
        &self,
        binding: &ProjectTechnologyBinding,
    ) -> Result<(), TechnologyBindingError> {
        rspice_simulation::pdk::validate_project_binding(self.validated_packages(), binding)
    }
}
