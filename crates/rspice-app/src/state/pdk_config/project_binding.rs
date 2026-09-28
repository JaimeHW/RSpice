//! Resolve persisted project pins against the currently trusted runtime registry.
use super::PdkTechnologyRegistry;
use rspice_project::{
    ProjectSignedTechnologyPin, ProjectTechnologyBinding, TechnologyBindingError,
};

impl PdkTechnologyRegistry {
    pub(crate) fn validate_project_binding(
        &self,
        binding: &ProjectTechnologyBinding,
    ) -> Result<(), TechnologyBindingError> {
        let pin = binding
            .signed_package()
            .ok_or(TechnologyBindingError::MissingSignedPackage)?;
        self.validate_project_pin(pin)
    }

    pub(crate) fn validate_project_pin(
        &self,
        pin: &ProjectSignedTechnologyPin,
    ) -> Result<(), TechnologyBindingError> {
        pin.validate()?;
        let package = self
            .validated_packages()
            .iter()
            .find(|package| {
                package
                    .manifest()
                    .package_id
                    .eq_ignore_ascii_case(pin.package_id())
                    && package.manifest().revision == pin.revision()
                    && package.manifest_digest() == pin.manifest_digest()
                    && package.archive_digest() == pin.archive_digest()
            })
            .ok_or_else(|| TechnologyBindingError::SignedPackageUnavailable {
                package_id: pin.package_id().to_owned(),
                revision: pin.revision().to_owned(),
            })?;
        let observed = ProjectSignedTechnologyPin::from_package_metadata(package.metadata())?;
        if &observed != pin {
            return Err(TechnologyBindingError::SignedPackageMetadataDrift {
                package_id: pin.package_id().to_owned(),
                revision: pin.revision().to_owned(),
            });
        }
        package
            .runtime_compatibility()
            .map_err(TechnologyBindingError::SignedPackageRuntime)
    }
}
