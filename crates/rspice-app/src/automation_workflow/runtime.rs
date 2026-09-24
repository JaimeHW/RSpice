//! Adapter from persisted project source identity to the worker protocol.

use std::str::FromStr as _;

use rspice_automation_protocol::{
    CapabilityGrant, Digest, DocumentRole, SourceDocument, SourceSnapshot,
};
use sha2::{Digest as _, Sha256};

use crate::automation_workflow::AutomationWorkspaceManifest;
use crate::product::{ContentDigest, ProjectId};
use crate::state::{ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceRole};

pub(crate) fn build_automation_runtime_snapshot(
    project_id: ProjectId,
    bundle: &ProjectSourceBundle,
    manifest: &AutomationWorkspaceManifest,
    capabilities: Vec<CapabilityGrant>,
) -> Result<SourceSnapshot, AutomationRuntimeSnapshotError> {
    if bundle.language() != ProjectSourceLanguage::RSpiceAutomation {
        return Err(AutomationRuntimeSnapshotError::WrongLanguage);
    }
    bundle
        .validate()
        .map_err(|error| AutomationRuntimeSnapshotError::InvalidBundle(error.to_string()))?;

    require_role(
        bundle,
        &manifest.entry_path,
        ProjectSourceRole::AutomationEntry,
    )?;
    require_role(
        bundle,
        &manifest.run_plan_path,
        ProjectSourceRole::AutomationRunPlan,
    )?;
    require_role(
        bundle,
        &manifest.environment_lock_path,
        ProjectSourceRole::AutomationEnvironmentLock,
    )?;
    require_role(
        bundle,
        &manifest.permissions_path,
        ProjectSourceRole::AutomationPermissionManifest,
    )?;

    let mut documents = Vec::with_capacity(bundle.files().len() + 1);
    documents.push(runtime_document(
        bundle,
        bundle.root().logical_path(),
        bundle.root().content(),
    )?);
    for file in bundle.files() {
        documents.push(runtime_document(
            bundle,
            file.logical_path(),
            file.content(),
        )?);
    }

    let entry_document_id = bundle
        .document_id(&manifest.entry_path)
        .ok_or_else(|| AutomationRuntimeSnapshotError::MissingDocument {
            path: manifest.entry_path.clone(),
        })?
        .as_uuid();
    let selected_run_plan_document_id = bundle
        .document_id(&manifest.run_plan_path)
        .ok_or_else(|| AutomationRuntimeSnapshotError::MissingDocument {
            path: manifest.run_plan_path.clone(),
        })?
        .as_uuid();
    let permission_source = bundle
        .file_content(&manifest.permissions_path)
        .ok_or_else(|| AutomationRuntimeSnapshotError::MissingDocument {
            path: manifest.permissions_path.clone(),
        })?;
    let permission_digest = Digest(Sha256::digest(permission_source.as_bytes()).into());
    let environment_digest = manifest
        .environment_digest
        .strip_prefix("sha256:")
        .ok_or(AutomationRuntimeSnapshotError::InvalidEnvironmentDigest)
        .and_then(|value| {
            ContentDigest::from_str(value)
                .map(|digest| Digest(*digest.as_bytes()))
                .map_err(|_| AutomationRuntimeSnapshotError::InvalidEnvironmentDigest)
        })?;

    let snapshot = SourceSnapshot {
        project_id: project_id.as_uuid(),
        workspace_id: bundle.id().as_uuid(),
        workspace_revision: bundle.revision().get(),
        closure_digest: Digest(*bundle.closure_digest().as_bytes()),
        environment_digest,
        permission_digest,
        entry_document_id,
        selected_run_plan_document_id: Some(selected_run_plan_document_id),
        python_requirement: manifest.python_version.clone(),
        api_requirement: manifest.api_version.clone(),
        browser_runtime_requirement: manifest.browser_runtime_requirement.clone(),
        documents,
        capabilities,
    };
    snapshot
        .validate()
        .map_err(AutomationRuntimeSnapshotError::InvalidProtocolSnapshot)?;
    Ok(snapshot)
}

fn runtime_document(
    bundle: &ProjectSourceBundle,
    path: &str,
    source: &str,
) -> Result<SourceDocument, AutomationRuntimeSnapshotError> {
    let role = match bundle.role_for_path(path) {
        Some(ProjectSourceRole::VerilogABuildProfile) => {
            return Err(AutomationRuntimeSnapshotError::InvalidBundle(format!(
                "Verilog-A build-profile role is not valid in Automation bundle '{}'",
                bundle.id()
            )));
        }
        Some(ProjectSourceRole::AutomationEntry) => DocumentRole::PythonEntry,
        Some(ProjectSourceRole::AutomationRunPlan) => DocumentRole::RunPlan,
        Some(ProjectSourceRole::AutomationEnvironmentLock) => DocumentRole::EnvironmentLock,
        Some(ProjectSourceRole::AutomationPermissionManifest) => DocumentRole::PermissionManifest,
        None if path.to_ascii_lowercase().ends_with(".py") => DocumentRole::PythonModule,
        None => DocumentRole::Resource,
    };
    let document_id = bundle.document_id(path).ok_or_else(|| {
        AutomationRuntimeSnapshotError::MissingDocument {
            path: path.to_owned(),
        }
    })?;
    let revision = bundle.document_revision(path).ok_or_else(|| {
        AutomationRuntimeSnapshotError::MissingDocument {
            path: path.to_owned(),
        }
    })?;
    Ok(SourceDocument {
        document_id: document_id.as_uuid(),
        logical_path: path.to_owned(),
        revision: revision.get(),
        role,
        read_only: role == DocumentRole::EnvironmentLock,
        source: source.to_owned(),
    })
}

fn require_role(
    bundle: &ProjectSourceBundle,
    path: &str,
    expected: ProjectSourceRole,
) -> Result<(), AutomationRuntimeSnapshotError> {
    match bundle.role_for_path(path) {
        Some(actual) if actual == expected => Ok(()),
        Some(actual) => Err(AutomationRuntimeSnapshotError::RoleMismatch {
            path: path.to_owned(),
            expected,
            actual: Some(actual),
        }),
        None => Err(AutomationRuntimeSnapshotError::RoleMismatch {
            path: path.to_owned(),
            expected,
            actual: None,
        }),
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AutomationRuntimeSnapshotError {
    #[error("runtime snapshots require an RSpice Automation source bundle")]
    WrongLanguage,
    #[error("Automation source bundle is invalid: {0}")]
    InvalidBundle(String),
    #[error("Automation document '{path}' is missing")]
    MissingDocument { path: String },
    #[error(
        "Automation document '{path}' must be bound as {expected:?}; current role is {actual:?}"
    )]
    RoleMismatch {
        path: String,
        expected: ProjectSourceRole,
        actual: Option<ProjectSourceRole>,
    },
    #[error("environment lock contains an invalid SHA-256 environment identity")]
    InvalidEnvironmentDigest,
    #[error("runtime snapshot violates the worker protocol: {0}")]
    InvalidProtocolSnapshot(#[from] rspice_automation_protocol::ProtocolError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automation_workflow::compile_automation_documents;
    use crate::state::{
        DEFAULT_AUTOMATION_PERMISSIONS, DEFAULT_AUTOMATION_PYTHON, DEFAULT_AUTOMATION_RUN_PLAN,
        DEFAULT_ENVIRONMENT_LOCK, ProjectSourceDependency, ProjectSourceFile, ProjectSourceOwner,
        ProjectSourceRoleBinding,
    };

    fn role(path: &str, role: ProjectSourceRole) -> ProjectSourceRoleBinding {
        ProjectSourceRoleBinding::try_new(path, role).unwrap()
    }

    fn bundle() -> ProjectSourceBundle {
        ProjectSourceBundle::try_new_with_roles(
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation),
            ProjectSourceLanguage::RSpiceAutomation,
            "flows/nightly.py",
            DEFAULT_AUTOMATION_PYTHON.replace("runplan.rspice.yaml", "plans/release.data"),
            [
                ProjectSourceFile::try_new("plans/release.data", DEFAULT_AUTOMATION_RUN_PLAN)
                    .unwrap(),
                ProjectSourceFile::try_new("environment/runtime.toml", DEFAULT_ENVIRONMENT_LOCK)
                    .unwrap(),
                ProjectSourceFile::try_new("security/policy.toml", DEFAULT_AUTOMATION_PERMISSIONS)
                    .unwrap(),
                ProjectSourceFile::try_new(
                    "flows/helpers.py",
                    "def normalize(value):\n    return value.casefold()\n",
                )
                .unwrap(),
            ],
            [
                ProjectSourceDependency::try_new("flows/nightly.py", "plans/release.data").unwrap(),
                ProjectSourceDependency::try_new("flows/nightly.py", "environment/runtime.toml")
                    .unwrap(),
                ProjectSourceDependency::try_new("flows/nightly.py", "security/policy.toml")
                    .unwrap(),
                ProjectSourceDependency::try_new("flows/nightly.py", "flows/helpers.py").unwrap(),
            ],
            [
                role("flows/nightly.py", ProjectSourceRole::AutomationEntry),
                role("plans/release.data", ProjectSourceRole::AutomationRunPlan),
                role(
                    "environment/runtime.toml",
                    ProjectSourceRole::AutomationEnvironmentLock,
                ),
                role(
                    "security/policy.toml",
                    ProjectSourceRole::AutomationPermissionManifest,
                ),
            ],
        )
        .unwrap()
    }

    #[test]
    fn exact_bundle_builds_a_valid_worker_snapshot_with_stable_document_ids() {
        let bundle = bundle();
        let documents = std::iter::once(bundle.root())
            .map(
                |root| crate::automation_workflow::AutomationSourceDocument {
                    path: root.logical_path(),
                    source: root.content(),
                },
            )
            .chain(bundle.files().iter().map(|file| {
                crate::automation_workflow::AutomationSourceDocument {
                    path: file.logical_path(),
                    source: file.content(),
                }
            }))
            .collect::<Vec<_>>();
        let roles = [
            crate::automation_workflow::AutomationRoleBinding {
                path: "plans/release.data",
                role: crate::automation_workflow::AutomationSourceRole::RunPlan,
            },
            crate::automation_workflow::AutomationRoleBinding {
                path: "environment/runtime.toml",
                role: crate::automation_workflow::AutomationSourceRole::EnvironmentLock,
            },
            crate::automation_workflow::AutomationRoleBinding {
                path: "security/policy.toml",
                role: crate::automation_workflow::AutomationSourceRole::PermissionManifest,
            },
        ];
        let (_, manifest) =
            compile_automation_documents(bundle.root().logical_path(), &documents, &roles).unwrap();
        let snapshot =
            build_automation_runtime_snapshot(ProjectId::new(), &bundle, &manifest, Vec::new())
                .unwrap();
        snapshot.validate().unwrap();
        assert_eq!(snapshot.documents.len(), 5);
        assert_eq!(snapshot.entry_document_id, bundle.root().id().as_uuid());
        assert_eq!(
            snapshot
                .documents
                .iter()
                .find(|document| document.logical_path == "flows/helpers.py")
                .unwrap()
                .role,
            DocumentRole::PythonModule
        );
    }
}
