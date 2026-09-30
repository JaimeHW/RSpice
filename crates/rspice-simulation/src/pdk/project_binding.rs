//! Resolve exact signed project pins against the retained validated runtime packages.

use super::ValidatedPdkTechnologyPackage;
use rspice_model_library::{
    ProjectSignedTechnologyPin, ProjectTechnologyBinding, TechnologyBindingError,
};

pub fn project_signed_technology_package<'a>(
    packages: &'a [ValidatedPdkTechnologyPackage],
    binding: Option<&ProjectTechnologyBinding>,
) -> Result<Option<&'a ValidatedPdkTechnologyPackage>, String> {
    let Some(binding) = binding else {
        return Ok(None);
    };
    let Some(pin) = binding.signed_package() else {
        return Err("Project technology binding has no signed package pin.".to_owned());
    };
    validate_project_binding(packages, binding)
        .map_err(|error| format!("Signed PDK project binding is unavailable: {error}"))?;
    packages
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
        .map(Some)
        .ok_or_else(|| {
            "The project's exact signed PDK package is not present in the current trusted runtime catalog."
                .to_owned()
        })
}

pub fn validate_project_binding(
    packages: &[ValidatedPdkTechnologyPackage],
    binding: &ProjectTechnologyBinding,
) -> Result<(), TechnologyBindingError> {
    let pin = binding
        .signed_package()
        .ok_or(TechnologyBindingError::MissingSignedPackage)?;
    validate_project_pin(packages, pin)
}

pub fn validate_project_pin(
    packages: &[ValidatedPdkTechnologyPackage],
    pin: &ProjectSignedTechnologyPin,
) -> Result<(), TechnologyBindingError> {
    pin.validate()?;
    let package = packages
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
