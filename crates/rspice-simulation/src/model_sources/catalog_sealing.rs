//! Validate catalog selection and seal exact execution-source bytes.

use super::{SealedModelExecutionSources, seal_model_sources};
use rspice_model_library::{ModelCatalog, ModelResolutionRecords, SimulationPlanModelBinding};

pub fn seal_catalog_execution_sources(
    catalog: &ModelCatalog,
    resolution_records: &ModelResolutionRecords,
) -> Result<SealedModelExecutionSources, String> {
    resolution_records.validate_against_catalog(catalog)?;
    let mut libraries: Vec<_> = catalog
        .libraries()
        .filter(|library| library.source_authority.has_execution_source())
        .map(|library| (library, library.selected_corner.clone()))
        .collect();
    libraries.sort_by(|(left, _), (right, _)| left.name.cmp(&right.name));
    #[cfg(not(target_arch = "wasm32"))]
    {
        seal_model_sources(libraries, resolution_records.as_map(), |path| {
            std::fs::read(path).map_err(|error| error.to_string())
        })
    }
    #[cfg(target_arch = "wasm32")]
    {
        seal_model_sources(libraries, resolution_records.as_map(), |path| {
            Err(format!(
                "browser execution cannot authenticate external model path '{}'",
                path.display()
            ))
        })
    }
}

pub fn seal_plan_execution_sources(
    catalog: &ModelCatalog,
    resolution_records: &ModelResolutionRecords,
    bindings: &[SimulationPlanModelBinding],
) -> Result<SealedModelExecutionSources, String> {
    resolution_records.validate_against_catalog(catalog)?;
    let libraries = catalog.resolve_simulation_plan_bindings(bindings)?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        seal_model_sources(libraries, resolution_records.as_map(), |path| {
            std::fs::read(path).map_err(|error| error.to_string())
        })
    }
    #[cfg(target_arch = "wasm32")]
    {
        seal_model_sources(libraries, resolution_records.as_map(), |path| {
            Err(format!(
                "browser execution cannot authenticate external model path '{}'",
                path.display()
            ))
        })
    }
}
