//! Resolve exact project PDK pins against the retained validated runtime packages.

use super::ValidatedPdkTechnologyPackage;
use rspice_model_library::{
    ProjectPdkPackagePin, ProjectTechnologyBinding, TechnologyBindingError,
};

pub fn project_pdk_package<'a>(
    packages: &'a [ValidatedPdkTechnologyPackage],
    binding: Option<&ProjectTechnologyBinding>,
) -> Result<Option<&'a ValidatedPdkTechnologyPackage>, String> {
    let Some(binding) = binding else {
        return Ok(None);
    };
    let Some(pin) = binding.pdk_package() else {
        return Err("Project technology binding has no PDK package pin.".to_owned());
    };
    validate_project_binding(packages, binding)
        .map_err(|error| format!("PDK project binding is unavailable: {error}"))?;
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
            "The project's exact PDK package is not present in the current trusted runtime catalog."
                .to_owned()
        })
}

pub fn validate_project_binding(
    packages: &[ValidatedPdkTechnologyPackage],
    binding: &ProjectTechnologyBinding,
) -> Result<(), TechnologyBindingError> {
    let pin = binding
        .pdk_package()
        .ok_or(TechnologyBindingError::MissingPdkPackage)?;
    validate_project_pin(packages, pin)
}

pub fn validate_project_pin(
    packages: &[ValidatedPdkTechnologyPackage],
    pin: &ProjectPdkPackagePin,
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
        .ok_or_else(|| TechnologyBindingError::PdkPackageUnavailable {
            package_id: pin.package_id().to_owned(),
            revision: pin.revision().to_owned(),
        })?;
    let observed = ProjectPdkPackagePin::from_package_metadata(package.metadata())?;
    if &observed != pin {
        return Err(TechnologyBindingError::PdkPackageMetadataDrift {
            package_id: pin.package_id().to_owned(),
            revision: pin.revision().to_owned(),
        });
    }
    package
        .runtime_compatibility()
        .map_err(TechnologyBindingError::PdkPackageRuntime)
}

/// Check the live model catalog, runtime PDK package and physical layouts for a project binding.
pub fn validate_project_technology_inputs(
    binding: &ProjectTechnologyBinding,
    models: &rspice_model_library::ModelCatalog,
    packages: &[ValidatedPdkTechnologyPackage],
    physical_layouts: &std::collections::BTreeMap<
        String,
        rspice_design::physical_layout::PhysicalLayoutDocument,
    >,
) -> Result<(), String> {
    models.validate_attached_technology(Some(binding))?;
    validate_project_binding(packages, binding)
        .map_err(|error| format!("PDK project binding is unavailable: {error}"))?;
    let pdk_pin = binding
        .pdk_package()
        .expect("validated project technology has an exact PDK package");
    let package = packages
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
        .expect("validated project PDK pin resolves to an exact package");
    let manifest = package.manifest();
    let database_unit =
        rspice_vocabulary::quantity::LayoutDatabaseUnit::from_metres(manifest.database_unit_meters)
            .map_err(|error| format!("Project PDK database unit is invalid: {error}"))?;
    let expected_layout_technology =
        rspice_design::physical_layout::LayoutTechnologyBinding::try_new(
            manifest.package_id.clone(),
            manifest.revision.clone(),
            package.manifest_digest(),
            package.archive_digest(),
            manifest.technology_name.clone(),
            manifest.stack_name.clone(),
            database_unit,
        )
        .map_err(|error| format!("Project PDK layout authority is invalid: {error}"))?;
    let allowed_layer_purposes = manifest
        .layers
        .iter()
        .flat_map(|layer| {
            layer.purposes.iter().map(move |purpose| {
                (
                    layer.name.to_ascii_lowercase(),
                    purpose.to_ascii_lowercase(),
                )
            })
        })
        .collect::<std::collections::BTreeSet<_>>();
    for (key, document) in physical_layouts {
        document
            .validate()
            .map_err(|error| format!("Physical layout '{key}' is invalid: {error}"))?;
        if document.technology() != &expected_layout_technology {
            return Err(format!(
                "Physical layout '{key}' is bound to a different PDK technology than the project"
            ));
        }
        for (kind, id, layer, purpose) in document
            .shapes()
            .iter()
            .map(|(id, shape)| {
                (
                    "shape",
                    id.to_string(),
                    shape.layer_purpose.layer.as_str(),
                    shape.layer_purpose.purpose.as_str(),
                )
            })
            .chain(document.texts().iter().map(|(id, text)| {
                (
                    "text",
                    id.to_string(),
                    text.layer_purpose.layer.as_str(),
                    text.layer_purpose.purpose.as_str(),
                )
            }))
        {
            if !allowed_layer_purposes
                .contains(&(layer.to_ascii_lowercase(), purpose.to_ascii_lowercase()))
            {
                return Err(format!(
                    "Physical layout '{key}' {kind} {id} uses layer/purpose '{layer}/{purpose}' outside the exact project PDK"
                ));
            }
        }
    }
    Ok(())
}

impl super::PdkTechnologyRegistry {
    pub fn project_pdk_package(
        &self,
        binding: Option<&ProjectTechnologyBinding>,
    ) -> Result<Option<&ValidatedPdkTechnologyPackage>, String> {
        super::project_pdk_package(self.validated_packages(), binding)
    }

    pub fn validate_project_binding(
        &self,
        binding: &ProjectTechnologyBinding,
    ) -> Result<(), TechnologyBindingError> {
        super::validate_project_binding(self.validated_packages(), binding)
    }
}
