//! Content-addressed export for the active model-source dependency closure.
//!
//! This artifact contains identities, digests, resolution edges, definitions,
//! and explicit provider choices. It never exports proprietary source bytes.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Serialize;
use sha2::{Digest as _, Sha256};

use crate::diagnostics::ConsoleMessage;
use crate::product::ContentDigest;
use crate::state::model_library::{
    ModelDefinitionProvider, ModelLibraryManager, ModelSourceAuthority,
};
use crate::workbench::RSpiceApp;
use crate::workbench::workflows::export_workflow::SaveDialogConfig;

const MODEL_DEPENDENCY_MANIFEST_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestProject {
    id: String,
    name: String,
    revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestProduct {
    version: String,
    build: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestSource {
    path: PathBuf,
    digest: String,
    byte_length: usize,
    root: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestEdge {
    owner: PathBuf,
    requested_path: String,
    target: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestDefinition {
    name: String,
    kind: String,
    spice_type: Option<String>,
    section: Option<String>,
    source: Option<PathBuf>,
    source_line: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestLibrary {
    name: String,
    pdk_name: String,
    technology_node: String,
    version: String,
    authority: ModelSourceAuthority,
    root: PathBuf,
    selected_corner: Option<String>,
    sources: Vec<ManifestSource>,
    edges: Vec<ManifestEdge>,
    definitions: Vec<ManifestDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestResolution {
    normalized_name: String,
    provider_library: String,
    provider_model: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestConflict {
    normalized_name: String,
    providers: Vec<ManifestProvider>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct ManifestProvider {
    library: String,
    model: String,
    source: Option<PathBuf>,
    source_line: Option<usize>,
}

impl From<&ModelDefinitionProvider> for ManifestProvider {
    fn from(provider: &ModelDefinitionProvider) -> Self {
        Self {
            library: provider.library.clone(),
            model: provider.model.clone(),
            source: provider.source.clone(),
            source_line: provider.source_line,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ModelDependencyManifest {
    schema_version: u16,
    artifact_kind: &'static str,
    project: ManifestProject,
    product: ManifestProduct,
    closure_digest: String,
    execution_ready: bool,
    libraries: Vec<ManifestLibrary>,
    provider_resolutions: Vec<ManifestResolution>,
    unresolved_conflicts: Vec<ManifestConflict>,
}

fn build_model_dependency_manifest(
    manager: &ModelLibraryManager,
    project: ManifestProject,
) -> Result<(ModelDependencyManifest, String), String> {
    let executable_libraries = manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| library.source_authority.has_execution_source())
        .collect::<Vec<_>>();
    if executable_libraries.is_empty() {
        return Err("No executable model-source libraries are loaded".to_owned());
    }

    // This reauthenticates native external files, validates every retained
    // byte/digest pair, closes the graph, and validates provider precedence.
    // A manifest must never claim execution readiness from stale metadata.
    manager.seal_execution_sources()?;

    let mut active_subcircuit_providers = BTreeMap::<String, Vec<String>>::new();
    for library in &executable_libraries {
        for subcircuit in library.subcircuits.values().filter(|subcircuit| {
            subcircuit.section.is_none()
                || subcircuit.section.as_deref() == library.selected_corner.as_deref()
        }) {
            active_subcircuit_providers
                .entry(subcircuit.name.to_ascii_lowercase())
                .or_default()
                .push(format!(
                    "{}/{}",
                    library.name,
                    subcircuit.section.as_deref().unwrap_or("top-level")
                ));
        }
    }
    if let Some((name, providers)) = active_subcircuit_providers
        .into_iter()
        .find(|(_, providers)| providers.len() > 1)
    {
        return Err(format!(
            "Active subcircuit definition '{name}' has multiple providers ({}); execution readiness cannot be certified",
            providers.join(", ")
        ));
    }

    let mut libraries = Vec::with_capacity(executable_libraries.len());
    for library in executable_libraries {
        let root = library.root_path.clone().ok_or_else(|| {
            format!(
                "Executable model library '{}' has no root source identity",
                library.name
            )
        })?;
        let byte_lengths = library
            .source_contents
            .iter()
            .map(|content| (content.path.clone(), content.bytes.len()))
            .collect::<BTreeMap<_, _>>();
        let mut sources = library
            .source_closure
            .iter()
            .map(|source| ManifestSource {
                path: source.path.clone(),
                digest: source.digest.to_string(),
                byte_length: byte_lengths.get(&source.path).copied().unwrap_or(0),
                root: source.path == root,
            })
            .collect::<Vec<_>>();
        sources.sort_by(|left, right| left.path.cmp(&right.path));

        let mut edges = library
            .source_edges
            .iter()
            .map(|edge| ManifestEdge {
                owner: edge.owner.clone(),
                requested_path: edge.requested_path.clone(),
                target: edge.target.clone(),
            })
            .collect::<Vec<_>>();
        edges.sort_by(|left, right| {
            left.owner
                .cmp(&right.owner)
                .then_with(|| left.requested_path.cmp(&right.requested_path))
                .then_with(|| left.target.cmp(&right.target))
        });

        let mut definitions = library
            .models
            .values()
            .map(|model| ManifestDefinition {
                name: model.name.clone(),
                kind: "model".to_owned(),
                spice_type: model.spice_type.clone(),
                section: model.section.clone(),
                source: model.file_path.clone(),
                source_line: model.source_line,
            })
            .chain(
                library
                    .subcircuits
                    .values()
                    .map(|subcircuit| ManifestDefinition {
                        name: subcircuit.name.clone(),
                        kind: "subcircuit".to_owned(),
                        spice_type: None,
                        section: subcircuit.section.clone(),
                        source: subcircuit.file_path.clone(),
                        source_line: subcircuit.source_line,
                    }),
            )
            .collect::<Vec<_>>();
        definitions.sort_by(|left, right| {
            left.name
                .to_ascii_lowercase()
                .cmp(&right.name.to_ascii_lowercase())
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.section.cmp(&right.section))
                .then_with(|| left.source.cmp(&right.source))
                .then_with(|| left.source_line.cmp(&right.source_line))
        });

        libraries.push(ManifestLibrary {
            name: library.name.clone(),
            pdk_name: library.pdk_name.clone(),
            technology_node: library.technology_node.clone(),
            version: library.version.clone(),
            authority: library.source_authority,
            root,
            selected_corner: library.selected_corner.clone(),
            sources,
            edges,
            definitions,
        });
    }

    let provider_resolutions = manager
        .definition_resolutions()
        .into_iter()
        .map(|resolution| ManifestResolution {
            normalized_name: resolution.normalized_name.clone(),
            provider_library: resolution.provider_library.clone(),
            provider_model: resolution.provider_model.clone(),
        })
        .collect::<Vec<_>>();
    let unresolved_conflicts = manager
        .definition_conflicts()
        .into_iter()
        .filter(|conflict| {
            manager
                .definition_resolution(&conflict.normalized_name)
                .is_none()
        })
        .map(|conflict| ManifestConflict {
            normalized_name: conflict.normalized_name,
            providers: conflict
                .providers
                .iter()
                .map(ManifestProvider::from)
                .collect(),
        })
        .collect::<Vec<_>>();
    if !unresolved_conflicts.is_empty() {
        return Err(
            "The model-source closure still contains unresolved definition conflicts".to_owned(),
        );
    }

    let material = serde_json::to_vec(&(
        MODEL_DEPENDENCY_MANIFEST_SCHEMA_VERSION,
        &libraries,
        &provider_resolutions,
        &unresolved_conflicts,
    ))
    .map_err(|error| format!("Could not encode model dependency manifest material: {error}"))?;
    let closure_digest = ContentDigest::from_bytes(Sha256::digest(material).into()).to_string();
    let manifest = ModelDependencyManifest {
        schema_version: MODEL_DEPENDENCY_MANIFEST_SCHEMA_VERSION,
        artifact_kind: "rspice.model-source-dependency-manifest",
        project,
        product: ManifestProduct {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            build: env!("RSPICE_BUILD_HASH").to_owned(),
        },
        closure_digest: closure_digest.clone(),
        execution_ready: true,
        libraries,
        provider_resolutions,
        unresolved_conflicts,
    };
    Ok((manifest, closure_digest))
}

pub(super) fn export_model_dependency_manifest(
    app: &mut RSpiceApp,
) -> Result<Option<String>, String> {
    let project = ManifestProject {
        id: app.state.workspace.project.id().to_string(),
        name: app.state.workspace.project.name().to_owned(),
        revision: app.state.workspace.project.revision().get(),
    };
    let (manifest, digest) =
        build_model_dependency_manifest(&app.state.model_library_manager, project)?;
    let mut json = serde_json::to_string_pretty(&manifest)
        .map_err(|error| format!("Could not encode model dependency manifest: {error}"))?;
    json.push('\n');
    let default_name = format!("rspice-model-dependencies-{}.json", &digest[..12]);
    let Some(mut path) = app.export_workflow_io.show_save_dialog(SaveDialogConfig {
        title: "Export model dependency manifest",
        default_name: &default_name,
        filter_name: "RSpice dependency manifest",
        filter_extensions: &["json"],
    })?
    else {
        return Ok(None);
    };
    if path.extension().is_none() {
        path.set_extension("json");
    }
    let destination = app.export_workflow_io.observe_destination(&path)?;
    app.export_workflow_io
        .write_text_file_observed(&destination, &json)?;
    let receipt = if app.export_workflow_io.saved_paths_are_reopenable() {
        format!(
            "Exported authenticated model dependency manifest {digest} to {}",
            path.display()
        )
    } else {
        format!(
            "Handed authenticated model dependency manifest {digest} to the browser download manager"
        )
    };
    app.state
        .push_user_message(ConsoleMessage::info(receipt.clone()));
    Ok(Some(receipt))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_manager() -> ModelLibraryManager {
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_bytes(
                "precision.lib",
                b".lib TT\n.model nch NMOS (LEVEL=54 VTH0=0.45)\n.subckt amp in out PARAMS: GAIN=10\nE1 out 0 in 0 {GAIN}\n.ends amp\n.endl TT\n"
                    .to_vec(),
                Some("TT"),
            )
            .expect("fixture library");
        manager
    }

    fn fixture_project() -> ManifestProject {
        ManifestProject {
            id: "018f0000-0000-7000-8000-000000000001".to_owned(),
            name: "Manifest fixture".to_owned(),
            revision: 7,
        }
    }

    #[test]
    fn dependency_manifest_is_deterministic_complete_and_source_free() {
        let manager = fixture_manager();
        let (first, first_digest) =
            build_model_dependency_manifest(&manager, fixture_project()).expect("first manifest");
        let (second, second_digest) =
            build_model_dependency_manifest(&manager, fixture_project()).expect("second manifest");
        assert_eq!(first, second);
        assert_eq!(first_digest, second_digest);
        let mut other_project = fixture_project();
        other_project.name = "Another project".to_owned();
        other_project.revision = 99;
        let (other, other_digest) =
            build_model_dependency_manifest(&manager, other_project).expect("other manifest");
        assert_ne!(first.project, other.project);
        assert_eq!(
            first_digest, other_digest,
            "closure identity must not depend on the project that references it"
        );
        assert_eq!(first_digest.len(), 64);
        assert!(first.execution_ready);
        assert_eq!(first.libraries.len(), 1);
        assert_eq!(first.libraries[0].sources.len(), 1);
        assert!(
            first.libraries[0]
                .definitions
                .iter()
                .any(|definition| definition.name == "nch"
                    && definition.kind == "model"
                    && definition.section.as_deref() == Some("TT"))
        );
        assert!(
            first.libraries[0]
                .definitions
                .iter()
                .any(|definition| definition.name == "amp"
                    && definition.kind == "subcircuit"
                    && definition.section.as_deref() == Some("TT"))
        );

        let json = serde_json::to_string(&first).expect("manifest JSON");
        assert!(!json.contains(".model nch"));
        assert!(!json.contains("E1 out"));
    }

    #[test]
    fn dependency_manifest_refuses_tampered_retained_bytes() {
        let mut manager = fixture_manager();
        manager
            .get_library_mut("precision")
            .expect("fixture library")
            .source_contents[0]
            .bytes
            .extend_from_slice(b"* tampered\n");
        let error = build_model_dependency_manifest(&manager, fixture_project())
            .expect_err("tampered bytes must fail");
        assert!(
            error.contains("do not match the accepted digest"),
            "{error}"
        );
    }
}
