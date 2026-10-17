//! Validate catalog selection and seal exact execution-source bytes.

use super::{SealedModelExecutionSources, seal_model_sources};
use rspice_app_types::product::ContentDigest;
use rspice_model_library::{ModelCatalog, ModelResolutionRecords, SimulationPlanModelBinding};
use sha2::{Digest as _, Sha256};
use std::path::Path;

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
    seal_model_sources(
        libraries,
        resolution_records.as_map(),
        read_external_model_source,
    )
}

pub fn seal_plan_execution_sources(
    catalog: &ModelCatalog,
    resolution_records: &ModelResolutionRecords,
    bindings: &[SimulationPlanModelBinding],
) -> Result<SealedModelExecutionSources, String> {
    resolution_records.validate_against_catalog(catalog)?;
    let libraries = catalog.resolve_simulation_plan_bindings(bindings)?;
    seal_model_sources(
        libraries,
        resolution_records.as_map(),
        read_external_model_source,
    )
}

/// Refuse unless every host file sealing `bindings` reads still has the
/// digest it is pinned to.
///
/// [`seal_plan_execution_sources`] reads an external library's pinned closure
/// from the host on every seal and refuses a file whose SHA-256 has moved; a
/// project-owned library or a retained import is sealed from bytes the catalog
/// carries, and reads nothing. This is that read and that comparison alone,
/// for a caller holding a run sealed earlier from the same catalog and
/// bindings, who needs to know the host would still seal it.
pub fn verify_plan_external_model_sources(
    catalog: &ModelCatalog,
    bindings: &[SimulationPlanModelBinding],
) -> Result<(), String> {
    let mut checked = std::collections::HashSet::new();
    for (library, _) in catalog.resolve_simulation_plan_bindings(bindings)? {
        if library.source_authority.uses_retained_bytes() {
            continue;
        }
        for source in &library.source_closure {
            if !checked.insert(source.path.as_path()) {
                continue;
            }
            let bytes = read_external_model_source(&source.path).map_err(|error| {
                format!(
                    "Model library dependency is unavailable at '{}' (used by {}): {error}",
                    source.path.display(),
                    library.name
                )
            })?;
            if ContentDigest::from_bytes(Sha256::digest(&bytes).into()) != source.digest {
                return Err(format!(
                    "Model library dependency changed at '{}'; refresh or re-import the library to explicitly accept the new source closure before simulation",
                    source.path.display()
                ));
            }
        }
    }
    Ok(())
}

/// The bytes of an external model source, as the host has them now.
#[cfg(not(target_arch = "wasm32"))]
fn read_external_model_source(path: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|error| error.to_string())
}

#[cfg(target_arch = "wasm32")]
fn read_external_model_source(path: &Path) -> Result<Vec<u8>, String> {
    Err(format!(
        "browser execution cannot authenticate external model path '{}'",
        path.display()
    ))
}
