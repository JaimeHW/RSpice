use std::{fmt, time::Duration};

#[cfg(not(target_arch = "wasm32"))]
use futures_util::StreamExt as _;
use http::{
    HeaderMap, Method, StatusCode,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, LOCATION, RETRY_AFTER},
};
use reqwest::RequestBuilder;
use rspice_cloud_contract::{
    API_VERSION, Artifact, ArtifactDownload, ArtifactState, ArtifactUpload, AuditEvent, BuildInfo,
    Circuit, CircuitRevision, CircuitShare, CollaborationTicketProtocol,
    CreateArtifactUploadRequest, CreateCircuitRequest, CreateCircuitRevisionRequest,
    CreateCollaborationTicketRequest, CreatePublicationRequest, CreateSimulationRunRequest,
    CreateWorkspaceRequest, CreatedCircuitShare, CreatedWorkspaceInvitation, CurrentPrincipal,
    Entitlement, IssueLicenseLeaseRequest, IssuedLicenseLease, LicenseJwkSet, LicenseLeaseList,
    Page, ProblemDetails, PublicPublication, Publication, SharedCircuit, SimulationRun,
    UpdateCircuitRequest, UpdateWorkspaceMemberRequest, UpdateWorkspaceRequest, Uuid, Workspace,
    WorkspaceInvitation, WorkspaceMember,
};
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

#[cfg(not(target_arch = "wasm32"))]
use crate::error::map_transport;
use crate::{
    BearerToken, ClientConfig, CloudError, CreateCircuitShare, CreateWorkspaceInvitation,
    CreatedCircuitRevision, IdempotencyKey, InvitationToken, PageRequest, ProtocolFailure,
    ResponseMetadata, ShareToken, TransportFailure,
    artifacts::{
        valid_artifact, valid_artifact_download, valid_artifact_list, valid_artifact_upload,
    },
    circuits::{
        circuit_matches_create_request, circuit_matches_revision_request,
        circuit_matches_update_request, revision_id_from_location, valid_circuit,
        valid_circuit_list, valid_circuit_revision, valid_circuit_revision_list,
    },
    collaboration::{CollaborationTicketFailure, validate_collaboration_ticket},
    governance::{
        created_invitation_handoff_is_safe, created_invitation_matches_request, valid_audit_events,
        valid_workspace_invitations, valid_workspace_member, valid_workspace_members,
        workspace_invitation_api_path,
    },
    identity::{valid_build_info, valid_current_principal},
    licensing::{
        issued_license_lease_matches_request, valid_entitlement_list, valid_license_jwks,
        valid_license_lease_list,
    },
    pagination::{valid_page_parts, valid_page_shape},
    publications::{
        PublicationSlug, publication_api_path, publication_matches_request,
        valid_public_publication, valid_publication_list,
    },
    shares::{
        created_share_handoff_is_safe, created_share_matches_request, share_id_from_location,
        valid_circuit_share, valid_circuit_share_list, valid_shared_circuit,
    },
    simulations::{
        simulation_run_api_path, simulation_run_matches_request, valid_simulation_run,
        valid_simulation_run_list,
    },
    workspaces::{
        valid_workspace, valid_workspace_list, workspace_matches_create_request,
        workspace_matches_update_request,
    },
};

const ACCEPT_JSON: &str = "application/json, application/problem+json";
const IDEMPOTENCY_KEY: &str = "idempotency-key";
pub(super) const X_REQUEST_ID: &str = "x-request-id";
pub(super) const IDEMPOTENCY_REPLAYED: &str = "idempotency-replayed";
const MAX_REQUEST_ID_BYTES: usize = 128;
const MAX_RETRY_AFTER_SECONDS: u64 = 24 * 60 * 60;
const MAX_LOCATION_BYTES: usize = 512;

/// Native and browser-WebAssembly HTTP client for the RSpice Cloud control plane.
#[derive(Clone)]
pub struct CloudClient {
    pub(crate) config: ClientConfig,
    pub(crate) http: reqwest::Client,
}

impl CloudClient {
    /// Builds a client from already validated settings.
    pub fn new(config: ClientConfig) -> Result<Self, CloudError> {
        let http = build_http_client()?;
        Ok(Self { config, http })
    }

    /// Returns deployed service and protocol build identity.
    pub async fn get_build_info(&self) -> Result<CloudResponse<BuildInfo>, CloudError> {
        let url = self.url(["api", API_VERSION, "meta"])?;
        let response: CloudResponse<BuildInfo> =
            self.get_json(url, None, &[StatusCode::OK]).await?;
        if !valid_build_info(&response.body) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Returns public PS256 keys used to verify native license leases.
    pub async fn get_license_jwks(&self) -> Result<CloudResponse<LicenseJwkSet>, CloudError> {
        let url = self.url(["api", API_VERSION, "licensing", "jwks"])?;
        let response: CloudResponse<LicenseJwkSet> =
            self.get_json(url, None, &[StatusCode::OK]).await?;
        if !valid_license_jwks(&response.body) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Bootstraps the current authenticated principal.
    pub async fn get_current_principal(
        &self,
        token: &BearerToken<'_>,
    ) -> Result<CloudResponse<CurrentPrincipal>, CloudError> {
        let url = self.url(["api", API_VERSION, "me"])?;
        let response: CloudResponse<CurrentPrincipal> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_current_principal(&response.body) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Lists commercial entitlements available to the current principal.
    pub async fn list_entitlements(
        &self,
        token: &BearerToken<'_>,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<Entitlement>>, CloudError> {
        let mut url = self.url(["api", API_VERSION, "entitlements"])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<Entitlement>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_parts(
            response.body.items.len(),
            response.body.next_cursor.as_deref(),
            page.limit(),
        ) || !valid_entitlement_list(&response.body.items)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Lists workspaces available to the current principal.
    pub async fn list_workspaces(
        &self,
        token: &BearerToken<'_>,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<Workspace>>, CloudError> {
        let mut url = self.url(["api", API_VERSION, "workspaces"])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<Workspace>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_parts(
            response.body.items.len(),
            response.body.next_cursor.as_deref(),
            page.limit(),
        ) || !valid_workspace_list(&response.body.items)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Gets one workspace visible to the current principal.
    pub async fn get_workspace(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
    ) -> Result<CloudResponse<Workspace>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let url = self.url(["api", API_VERSION, "workspaces", workspace_id_path.as_str()])?;
        let response: CloudResponse<Workspace> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_workspace(&response.body, Some(workspace_id)) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Creates or exactly replays a workspace mutation under a stable key.
    ///
    /// The caller owns persistence and retry scheduling for the supplied key;
    /// this method performs one network attempt. A successful response must
    /// carry authenticated replay metadata from an idempotency-capable API.
    pub async fn create_workspace_idempotent(
        &self,
        token: &BearerToken<'_>,
        key: &IdempotencyKey<'_>,
        request: &CreateWorkspaceRequest,
    ) -> Result<CloudResponse<Workspace>, CloudError> {
        let url = self.url(["api", API_VERSION, "workspaces"])?;
        let http_request = self
            .request(Method::POST, url.clone(), Some(token))
            .header(IDEMPOTENCY_KEY, key.value())
            .json(request);
        let response = self
            .execute_json(http_request, url, &[StatusCode::CREATED])
            .await?;
        let Some(replayed) = response.metadata.idempotency_replayed() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        if !workspace_matches_create_request(&response.body, request, replayed) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Renames a workspace with optimistic concurrency and no automatic retry.
    pub async fn update_workspace(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        request: &UpdateWorkspaceRequest,
    ) -> Result<CloudResponse<Workspace>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let url = self.url(["api", API_VERSION, "workspaces", workspace_id_path.as_str()])?;
        let response = self
            .mutation_json(Method::PATCH, url, token, request, &[StatusCode::OK])
            .await?;
        if !workspace_matches_update_request(&response.body, workspace_id, request) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Lists immutable administrator-visible workspace audit evidence.
    pub async fn list_workspace_audit_events(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<AuditEvent>>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "audit-events",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<AuditEvent>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_audit_events(&response.body.items, workspace_id)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Lists the current workspace roster in stable newest-first order.
    pub async fn list_workspace_members(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<WorkspaceMember>>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "members",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<WorkspaceMember>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_workspace_members(&response.body.items, workspace_id)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Adds an existing principal or changes one member's role.
    ///
    /// This method makes one network attempt. The returned member must bind to
    /// the exact workspace, target principal, and requested role.
    pub async fn update_workspace_member(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        principal_id: Uuid,
        request: &UpdateWorkspaceMemberRequest,
    ) -> Result<CloudResponse<WorkspaceMember>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let principal_id_path = principal_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "members",
            principal_id_path.as_str(),
        ])?;
        let response = self
            .mutation_json(Method::PUT, url, token, request, &[StatusCode::OK])
            .await?;
        if !valid_workspace_member(
            &response.body,
            Some(workspace_id),
            Some(principal_id),
            Some(request.role),
        ) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Removes one workspace member without automatic retry.
    pub async fn remove_workspace_member(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        principal_id: Uuid,
    ) -> Result<CloudResponse<()>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let principal_id_path = principal_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "members",
            principal_id_path.as_str(),
        ])?;
        let request = self.request(Method::DELETE, url.clone(), Some(token));
        self.execute_empty(request, url, &[StatusCode::NO_CONTENT])
            .await
    }

    /// Lists non-secret workspace invitation lifecycle records.
    pub async fn list_workspace_invitations(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<WorkspaceInvitation>>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "invitations",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<WorkspaceInvitation>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_workspace_invitations(&response.body.items, workspace_id)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Revokes one unaccepted workspace invitation without automatic retry.
    pub async fn revoke_workspace_invitation(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        invitation_id: Uuid,
    ) -> Result<CloudResponse<()>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let invitation_id_path = invitation_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "invitations",
            invitation_id_path.as_str(),
        ])?;
        let request = self.request(Method::DELETE, url.clone(), Some(token));
        self.execute_empty(request, url, &[StatusCode::NO_CONTENT])
            .await
    }

    /// Lists non-deleted artifact metadata in one visible workspace.
    pub async fn list_workspace_artifacts(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<Artifact>>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "artifacts",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<Artifact>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_artifact_list(&response.body.items, workspace_id)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Creates or exactly replays one checksum-bound direct-upload session.
    ///
    /// The caller durably owns the complete command and idempotency key and
    /// performs the object PUT separately. This method makes one network
    /// attempt, requires explicit replay metadata, validates the returned
    /// artifact identity and integrity fields, and treats the upload URL and
    /// required headers as one atomic capability bundle.
    pub async fn create_artifact_upload_idempotent(
        &self,
        token: &BearerToken<'_>,
        key: &IdempotencyKey<'_>,
        workspace_id: Uuid,
        request: &CreateArtifactUploadRequest,
    ) -> Result<CloudResponse<ArtifactUpload>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "artifacts",
        ])?;
        let http_request = self
            .request(Method::POST, url.clone(), Some(token))
            .header(IDEMPOTENCY_KEY, key.value())
            .json(request);
        let response: CloudResponse<ArtifactUpload> = self
            .execute_json(http_request, url, &[StatusCode::CREATED])
            .await?;
        let Some(replayed) = response.metadata.idempotency_replayed() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        if !valid_artifact_upload(
            &response.body,
            workspace_id,
            request,
            replayed,
            self.config.mode,
            &self.config.object_storage_origin,
        ) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Gets one tenant-scoped artifact's integrity and lifecycle metadata.
    pub async fn get_artifact(
        &self,
        token: &BearerToken<'_>,
        artifact_id: Uuid,
    ) -> Result<CloudResponse<Artifact>, CloudError> {
        let artifact_id_path = artifact_id.to_string();
        let url = self.url(["api", API_VERSION, "artifacts", artifact_id_path.as_str()])?;
        let response: CloudResponse<Artifact> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        require_artifact(response, artifact_id, None, StatusCode::OK)
    }

    /// Finalizes an uploaded object after server-side size and checksum checks.
    ///
    /// This call is server-idempotent but deliberately makes one network
    /// attempt. An ambiguous result is reconciled with [`Self::get_artifact`].
    pub async fn complete_artifact_upload(
        &self,
        token: &BearerToken<'_>,
        artifact_id: Uuid,
    ) -> Result<CloudResponse<Artifact>, CloudError> {
        let artifact_id_path = artifact_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "artifacts",
            artifact_id_path.as_str(),
            "complete",
        ])?;
        let request = self.request(Method::POST, url.clone(), Some(token));
        let response: CloudResponse<Artifact> =
            self.execute_json(request, url, &[StatusCode::OK]).await?;
        require_artifact(
            response,
            artifact_id,
            Some(ArtifactState::Available),
            StatusCode::OK,
        )
    }

    /// Issues a short-lived direct download for one available artifact.
    pub async fn download_artifact(
        &self,
        token: &BearerToken<'_>,
        artifact_id: Uuid,
    ) -> Result<CloudResponse<ArtifactDownload>, CloudError> {
        let artifact_id_path = artifact_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "artifacts",
            artifact_id_path.as_str(),
            "download",
        ])?;
        let response: CloudResponse<ArtifactDownload> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        require_artifact_download(
            response,
            artifact_id,
            self.config.mode,
            &self.config.object_storage_origin,
            StatusCode::OK,
        )
    }

    /// Queues deletion of one unreferenced artifact without automatic retry.
    pub async fn delete_artifact(
        &self,
        token: &BearerToken<'_>,
        artifact_id: Uuid,
    ) -> Result<CloudResponse<()>, CloudError> {
        let artifact_id_path = artifact_id.to_string();
        let url = self.url(["api", API_VERSION, "artifacts", artifact_id_path.as_str()])?;
        let request = self.request(Method::DELETE, url.clone(), Some(token));
        self.execute_empty(request, url, &[StatusCode::NO_CONTENT])
            .await
    }

    /// Lists visible circuits in one workspace.
    pub async fn list_circuits(
        &self,
        token: &BearerToken<'_>,
        workspace_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<Circuit>>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "circuits",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<Circuit>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_circuit_list(&response.body.items, workspace_id)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Gets one visible circuit by its public identifier.
    pub async fn get_circuit(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
    ) -> Result<CloudResponse<Circuit>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let url = self.url(["api", API_VERSION, "circuits", circuit_id_path.as_str()])?;
        let response: CloudResponse<Circuit> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        require_valid_circuit(response, Some(circuit_id), None, StatusCode::OK)
    }

    /// Creates or exactly replays a circuit and its initial revision.
    ///
    /// The caller durably owns the key and complete command and controls all
    /// retry scheduling. This method makes one network attempt and requires
    /// explicit replay metadata from an idempotency-capable API.
    pub async fn create_circuit_idempotent(
        &self,
        token: &BearerToken<'_>,
        key: &IdempotencyKey<'_>,
        workspace_id: Uuid,
        request: &CreateCircuitRequest,
    ) -> Result<CloudResponse<Circuit>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "circuits",
        ])?;
        let http_request = self
            .request(Method::POST, url.clone(), Some(token))
            .header(IDEMPOTENCY_KEY, key.value())
            .json(request);
        let response: CloudResponse<Circuit> = self
            .execute_json(http_request, url, &[StatusCode::CREATED])
            .await?;
        let Some(replayed) = response.metadata.idempotency_replayed() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        if !circuit_matches_create_request(&response.body, workspace_id, request, replayed) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Applies one optimistic-concurrency circuit metadata update.
    ///
    /// This method never retries. Callers must refetch after an ambiguous
    /// result or precondition failure before deciding on another command.
    pub async fn update_circuit(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        request: &UpdateCircuitRequest,
    ) -> Result<CloudResponse<Circuit>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let url = self.url(["api", API_VERSION, "circuits", circuit_id_path.as_str()])?;
        let response: CloudResponse<Circuit> = self
            .mutation_json(Method::PATCH, url, token, request, &[StatusCode::OK])
            .await?;
        if !circuit_matches_update_request(&response.body, circuit_id, request) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Lists sealed immutable revisions for one visible circuit.
    pub async fn list_circuit_revisions(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<CircuitRevision>>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "revisions",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<CircuitRevision>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_circuit_revision_list(&response.body.items)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Gets one sealed immutable circuit revision.
    pub async fn get_circuit_revision(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        revision_id: Uuid,
    ) -> Result<CloudResponse<CircuitRevision>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let revision_id_path = revision_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "revisions",
            revision_id_path.as_str(),
        ])?;
        let response: CloudResponse<CircuitRevision> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_circuit_revision(&response.body, Some(revision_id)) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Creates or exactly replays one immutable successor revision.
    ///
    /// The caller durably owns the key and complete command. A successful
    /// response must identify the original immutable revision through a
    /// validated, circuit-bound `Location`, even when a replay returns a
    /// newer current circuit projection. This method makes one network attempt.
    pub async fn create_circuit_revision_idempotent(
        &self,
        token: &BearerToken<'_>,
        key: &IdempotencyKey<'_>,
        circuit_id: Uuid,
        request: &CreateCircuitRevisionRequest,
    ) -> Result<CloudResponse<CreatedCircuitRevision>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "revisions",
        ])?;
        let http_request = self
            .request(Method::POST, url.clone(), Some(token))
            .header(IDEMPOTENCY_KEY, key.value())
            .json(request);
        let response: CloudResponse<Circuit> = self
            .execute_json(http_request, url, &[StatusCode::CREATED])
            .await?;
        let Some(replayed) = response.metadata.idempotency_replayed() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        let Some(location) = response.metadata.location() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        let Some(revision_id) = revision_id_from_location(location, circuit_id) else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        if !circuit_matches_revision_request(
            &response.body,
            circuit_id,
            revision_id,
            request,
            replayed,
        ) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(CloudResponse {
            body: CreatedCircuitRevision::new(response.body, revision_id),
            status: response.status,
            metadata: response.metadata,
        })
    }

    /// Issues one short-lived request-target-safe live-collaboration ticket.
    ///
    /// This method sends a stable caller-owned client-instance identifier,
    /// makes one network attempt, validates the returned circuit-bound
    /// WebSocket endpoint and canonical credential protocol. The response body contains a
    /// bearer capability and is therefore redacted from diagnostic formatting.
    pub async fn issue_collaboration_ticket(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        client_instance_id: Uuid,
    ) -> Result<CloudResponse<CollaborationTicketProtocol>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "collaboration",
            "ticket-protocols",
        ])?;
        let request = CreateCollaborationTicketRequest {
            client_instance_id: Some(client_instance_id),
        };
        let response: CloudResponse<CollaborationTicketProtocol> = self
            .mutation_json(Method::POST, url, token, &request, &[StatusCode::CREATED])
            .await?;
        if let Err(failure) =
            validate_collaboration_ticket(&response.body, circuit_id, client_instance_id)
        {
            return Err(CloudError::Protocol {
                failure: match failure {
                    CollaborationTicketFailure::Invalid => ProtocolFailure::InvalidSuccessResponse,
                    CollaborationTicketFailure::CapabilityExpired => {
                        ProtocolFailure::CapabilityExpired
                    }
                },
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Lists durable remote-simulation runs for one visible circuit.
    pub async fn list_circuit_simulation_runs(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<SimulationRun>>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "simulation-runs",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<SimulationRun>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_simulation_run_list(&response.body.items, circuit_id)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Queues or exactly replays one remote simulation under a durable key.
    ///
    /// The caller owns the complete request, idempotency key, and retry
    /// schedule. This method makes one attempt, requires replay metadata plus
    /// an exact resource Location, and binds the returned run to the requested
    /// circuit, optional revision, and analysis payload. A fresh response must
    /// be queued; an exact replay may return any later lifecycle state.
    pub async fn queue_simulation_run_idempotent(
        &self,
        token: &BearerToken<'_>,
        key: &IdempotencyKey<'_>,
        circuit_id: Uuid,
        request: &CreateSimulationRunRequest,
    ) -> Result<CloudResponse<SimulationRun>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "simulation-runs",
        ])?;
        let http_request = self
            .request(Method::POST, url.clone(), Some(token))
            .header(IDEMPOTENCY_KEY, key.value())
            .json(request);
        let response: CloudResponse<SimulationRun> = self
            .execute_json(http_request, url, &[StatusCode::CREATED])
            .await?;
        let Some(replayed) = response.metadata.idempotency_replayed() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        if response.metadata.location() != Some(simulation_run_api_path(response.body.id).as_str())
            || !simulation_run_matches_request(&response.body, circuit_id, request, replayed)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Gets one durable remote-simulation run by its public identifier.
    pub async fn get_simulation_run(
        &self,
        token: &BearerToken<'_>,
        run_id: Uuid,
    ) -> Result<CloudResponse<SimulationRun>, CloudError> {
        let run_id_path = run_id.to_string();
        let url = self.url(["api", API_VERSION, "simulation-runs", run_id_path.as_str()])?;
        let response: CloudResponse<SimulationRun> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if response.body.id != run_id || !valid_simulation_run(&response.body) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Cancels a queued run or requests cancellation of a running run.
    ///
    /// The server operation is idempotent, but this method still makes one
    /// network attempt. Reconcile an ambiguous result with
    /// [`Self::get_simulation_run`] before scheduling another call.
    pub async fn cancel_simulation_run(
        &self,
        token: &BearerToken<'_>,
        run_id: Uuid,
    ) -> Result<CloudResponse<()>, CloudError> {
        let run_id_path = run_id.to_string();
        let url = self.url(["api", API_VERSION, "simulation-runs", run_id_path.as_str()])?;
        let request = self.request(Method::DELETE, url.clone(), Some(token));
        self.execute_empty(request, url, &[StatusCode::NO_CONTENT])
            .await
    }

    /// Lists immutable publication management records for one circuit.
    pub async fn list_circuit_publications(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<Publication>>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "publications",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<Publication>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_publication_list(&response.body.items, circuit_id)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Creates or exactly replays one immutable public publication.
    ///
    /// The caller durably owns the complete disclosure command and
    /// idempotency key. This method makes one attempt, requires replay
    /// evidence, and validates the returned non-secret `/c/<slug>` identity
    /// against the circuit, revision, simulation, and normalized metadata.
    pub async fn create_circuit_publication_idempotent(
        &self,
        token: &BearerToken<'_>,
        key: &IdempotencyKey<'_>,
        circuit_id: Uuid,
        request: &CreatePublicationRequest,
    ) -> Result<CloudResponse<Publication>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "publications",
        ])?;
        let http_request = self
            .request(Method::POST, url.clone(), Some(token))
            .header(IDEMPOTENCY_KEY, key.value())
            .json(request);
        let response: CloudResponse<Publication> = self
            .execute_json(http_request, url, &[StatusCode::CREATED])
            .await?;
        let Some(replayed) = response.metadata.idempotency_replayed() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        if response.metadata.location() != Some(publication_api_path(&response.body.slug).as_str())
            || !publication_matches_request(&response.body, circuit_id, request, replayed)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Soft-unpublishes one immutable publication without recycling its slug.
    ///
    /// The server operation is idempotent, but the client makes one network
    /// attempt and leaves ambiguous-result reconciliation to the caller.
    pub async fn unpublish_circuit_publication(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        publication_id: Uuid,
    ) -> Result<CloudResponse<()>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let publication_id_path = publication_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "publications",
            publication_id_path.as_str(),
        ])?;
        let request = self.request(Method::DELETE, url.clone(), Some(token));
        self.execute_empty(request, url, &[StatusCode::NO_CONTENT])
            .await
    }

    /// Resolves one active immutable publication without authentication.
    pub async fn resolve_publication(
        &self,
        slug: PublicationSlug<'_>,
    ) -> Result<CloudResponse<PublicPublication>, CloudError> {
        let url = self.url(["api", API_VERSION, "publications", slug.value()])?;
        let response: CloudResponse<PublicPublication> =
            self.get_json(url, None, &[StatusCode::OK]).await?;
        if !valid_public_publication(&response.body, slug) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Issues a short-lived download for one explicitly published artifact.
    pub async fn download_publication_artifact(
        &self,
        slug: PublicationSlug<'_>,
        artifact_id: Uuid,
    ) -> Result<CloudResponse<ArtifactDownload>, CloudError> {
        let artifact_id_path = artifact_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "publications",
            slug.value(),
            "artifacts",
            artifact_id_path.as_str(),
            "download",
        ])?;
        let response: CloudResponse<ArtifactDownload> =
            self.get_json(url, None, &[StatusCode::OK]).await?;
        require_artifact_download(
            response,
            artifact_id,
            self.config.mode,
            &self.config.object_storage_origin,
            StatusCode::OK,
        )
    }

    /// Gets the current head revision of a circuit with public visibility.
    ///
    /// This unauthenticated discovery projection can change whenever the
    /// circuit head or metadata changes. Use [`Self::resolve_publication`] for
    /// a stable engineering reference suitable for an RSpice `/c/<slug>` URL.
    pub async fn get_public_circuit(
        &self,
        circuit_id: Uuid,
    ) -> Result<CloudResponse<SharedCircuit>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "public",
            "circuits",
            circuit_id_path.as_str(),
        ])?;
        let response: CloudResponse<SharedCircuit> =
            self.get_json(url, None, &[StatusCode::OK]).await?;
        require_shared_circuit(response, Some(circuit_id), StatusCode::OK)
    }

    /// Lists non-secret bearer-share metadata for one managed circuit.
    pub async fn list_circuit_shares(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<Page<CircuitShare>>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let mut url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "shares",
        ])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<Page<CircuitShare>> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_shape(&response.body, page.limit())
            || !valid_circuit_share_list(&response.body.items, circuit_id)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Gets one circuit share's non-secret management metadata.
    pub async fn get_circuit_share(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        share_id: Uuid,
    ) -> Result<CloudResponse<CircuitShare>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let share_id_path = share_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "shares",
            share_id_path.as_str(),
        ])?;
        let response: CloudResponse<CircuitShare> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_circuit_share(&response.body, circuit_id, Some(share_id)) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Creates or exactly replays a client-committed bearer share.
    ///
    /// The caller durably owns the raw token, complete command, and key. This
    /// method makes one network attempt, serializes only the token commitment,
    /// requires replay and circuit-bound `Location` metadata, and rejects any
    /// keyed success that returns a raw bearer token.
    pub async fn create_circuit_share_idempotent(
        &self,
        token: &BearerToken<'_>,
        key: &IdempotencyKey<'_>,
        circuit_id: Uuid,
        request: &CreateCircuitShare<'_>,
    ) -> Result<CloudResponse<CreatedCircuitShare>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "shares",
        ])?;
        let http_request = self
            .request(Method::POST, url.clone(), Some(token))
            .header(IDEMPOTENCY_KEY, key.value())
            .json(request);
        let response: CloudResponse<CreatedCircuitShare> = self
            .execute_json(http_request, url, &[StatusCode::CREATED])
            .await?;
        let Some(replayed) = response.metadata.idempotency_replayed() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        let Some(location) = response.metadata.location() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        if share_id_from_location(location, circuit_id) != Some(response.body.id)
            || !created_share_matches_request(&response.body, circuit_id, request, replayed)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        if !created_share_handoff_is_safe(&response.body) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::CapabilityExpired,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Revokes one managed bearer share without automatic retry.
    pub async fn revoke_circuit_share(
        &self,
        token: &BearerToken<'_>,
        circuit_id: Uuid,
        share_id: Uuid,
    ) -> Result<CloudResponse<()>, CloudError> {
        let circuit_id_path = circuit_id.to_string();
        let share_id_path = share_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "circuits",
            circuit_id_path.as_str(),
            "shares",
            share_id_path.as_str(),
        ])?;
        let request = self.request(Method::DELETE, url.clone(), Some(token));
        self.execute_empty(request, url, &[StatusCode::NO_CONTENT])
            .await
    }

    /// Resolves an active bearer share through the stable no-secret target.
    ///
    /// The share capability is sent only in the sensitive `Authorization`
    /// field; the server does not expose a token-in-path alternative.
    pub async fn resolve_circuit_share(
        &self,
        share_token: ShareToken<'_>,
    ) -> Result<CloudResponse<SharedCircuit>, CloudError> {
        let url = self.url(["api", API_VERSION, "shares", "resolve"])?;
        let request = self.request_with_share_token(Method::GET, url.clone(), share_token);
        let response = self.execute_json(request, url, &[StatusCode::OK]).await?;
        require_shared_circuit(response, None, StatusCode::OK)
    }

    /// Issues a short-lived download for one artifact snapshotted by a share.
    ///
    /// Both the share credential and returned presigned URL are bearer
    /// capabilities. The request uses only the stable no-secret target and the
    /// response body remains redacted from diagnostic formatting.
    pub async fn download_shared_artifact(
        &self,
        share_token: ShareToken<'_>,
        artifact_id: Uuid,
    ) -> Result<CloudResponse<ArtifactDownload>, CloudError> {
        let artifact_id_path = artifact_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "shares",
            "artifacts",
            artifact_id_path.as_str(),
            "download",
        ])?;
        let request = self.request_with_share_token(Method::GET, url.clone(), share_token);
        let response = self.execute_json(request, url, &[StatusCode::OK]).await?;
        require_artifact_download(
            response,
            artifact_id,
            self.config.mode,
            &self.config.object_storage_origin,
            StatusCode::OK,
        )
    }

    /// Creates or exactly replays a client-committed workspace invitation.
    ///
    /// The caller owns durable storage of the raw invitation token, command,
    /// and idempotency key as well as all retry scheduling. This method makes
    /// one network attempt, serializes only the token commitment, requires
    /// explicit replay metadata plus an exact resource Location, and rejects a
    /// keyed success that returns a raw token contrary to the preferred
    /// protocol.
    pub async fn create_workspace_invitation_idempotent(
        &self,
        token: &BearerToken<'_>,
        key: &IdempotencyKey<'_>,
        workspace_id: Uuid,
        request: &CreateWorkspaceInvitation<'_>,
    ) -> Result<CloudResponse<CreatedWorkspaceInvitation>, CloudError> {
        let workspace_id_path = workspace_id.to_string();
        let url = self.url([
            "api",
            API_VERSION,
            "workspaces",
            workspace_id_path.as_str(),
            "invitations",
        ])?;
        let http_request = self
            .request(Method::POST, url.clone(), Some(token))
            .header(IDEMPOTENCY_KEY, key.value())
            .json(request);
        let response: CloudResponse<CreatedWorkspaceInvitation> = self
            .execute_json(http_request, url, &[StatusCode::CREATED])
            .await?;
        let Some(replayed) = response.metadata.idempotency_replayed() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        let Some(location) = response.metadata.location() else {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidMetadata,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        };
        if location != workspace_invitation_api_path(workspace_id, response.body.id)
            || !created_invitation_matches_request(&response.body, workspace_id, request, replayed)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        if !created_invitation_handoff_is_safe(&response.body) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::CapabilityExpired,
                status: Some(StatusCode::CREATED.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Accepts a workspace invitation through the stable no-secret request target.
    ///
    /// The OIDC credential is sent only in `Authorization`; the distinct
    /// invitation capability is sent only in the bounded JSON body. This
    /// method makes one network attempt and leaves any retry to the caller.
    pub async fn accept_workspace_invitation(
        &self,
        oidc_token: &BearerToken<'_>,
        invitation_token: InvitationToken<'_>,
    ) -> Result<CloudResponse<WorkspaceMember>, CloudError> {
        #[derive(Serialize)]
        struct AcceptInvitationRequest<'a> {
            token: &'a str,
        }

        let url = self.url(["api", API_VERSION, "invitations", "accept"])?;
        let request = AcceptInvitationRequest {
            token: invitation_token.as_str(),
        };
        let response = self
            .mutation_json(Method::POST, url, oidc_token, &request, &[StatusCode::OK])
            .await?;
        if !valid_workspace_member(&response.body, None, None, None) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Lists renewable native license lease records for the current principal.
    pub async fn list_license_leases(
        &self,
        token: &BearerToken<'_>,
        page: PageRequest<'_>,
    ) -> Result<CloudResponse<LicenseLeaseList>, CloudError> {
        let mut url = self.url(["api", API_VERSION, "license-leases"])?;
        add_page_query(&mut url, page);
        let response: CloudResponse<LicenseLeaseList> =
            self.get_json(url, Some(token), &[StatusCode::OK]).await?;
        if !valid_page_parts(
            response.body.items.len(),
            response.body.next_cursor.as_deref(),
            page.limit(),
        ) || !valid_license_lease_list(&response.body.items)
        {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(StatusCode::OK.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Issues or explicitly retries a device-bound native license lease.
    ///
    /// The supplied request UUID makes a caller-controlled retry safe. This
    /// method sends the request exactly once and never schedules that retry.
    pub async fn issue_license_lease(
        &self,
        token: &BearerToken<'_>,
        request: &IssueLicenseLeaseRequest,
    ) -> Result<CloudResponse<IssuedLicenseLease>, CloudError> {
        let url = self.url(["api", API_VERSION, "license-leases"])?;
        let response: CloudResponse<IssuedLicenseLease> = self
            .mutation_json(
                Method::POST,
                url,
                token,
                request,
                &[StatusCode::OK, StatusCode::CREATED],
            )
            .await?;
        if !issued_license_lease_matches_request(&response.body, request) {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::InvalidSuccessResponse,
                status: Some(response.status.as_u16()),
                metadata: response.metadata,
            });
        }
        Ok(response)
    }

    /// Revokes a renewable native license lease without automatic retry.
    pub async fn revoke_license_lease(
        &self,
        token: &BearerToken<'_>,
        lease_id: Uuid,
    ) -> Result<CloudResponse<()>, CloudError> {
        let lease_id = lease_id.to_string();
        let url = self.url(["api", API_VERSION, "license-leases", lease_id.as_str()])?;
        let request = self.request(Method::DELETE, url.clone(), Some(token));
        self.execute_empty(request, url, &[StatusCode::NO_CONTENT])
            .await
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        url: Url,
        token: Option<&BearerToken<'_>>,
        expected: &[StatusCode],
    ) -> Result<CloudResponse<T>, CloudError> {
        let request = self.request(Method::GET, url.clone(), token);
        self.execute_json(request, url, expected).await
    }

    async fn mutation_json<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        url: Url,
        token: &BearerToken<'_>,
        body: &B,
        expected: &[StatusCode],
    ) -> Result<CloudResponse<T>, CloudError> {
        let request = self.request(method, url.clone(), Some(token)).json(body);
        self.execute_json(request, url, expected).await
    }

    async fn execute_json<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
        requested_url: Url,
        expected: &[StatusCode],
    ) -> Result<CloudResponse<T>, CloudError> {
        let received = self.send(request, &requested_url).await?;
        if !expected.contains(&received.status) {
            return Err(error_response(received));
        }
        if !has_media_type(&received.headers, "application/json") {
            return Err(received.protocol(ProtocolFailure::UnexpectedContentType));
        }
        let body = serde_json::from_slice(&received.body)
            .map_err(|_| received.protocol(ProtocolFailure::InvalidJson))?;
        Ok(CloudResponse {
            body,
            status: received.status,
            metadata: received.metadata,
        })
    }

    async fn execute_empty(
        &self,
        request: RequestBuilder,
        requested_url: Url,
        expected: &[StatusCode],
    ) -> Result<CloudResponse<()>, CloudError> {
        let received = self.send(request, &requested_url).await?;
        if !expected.contains(&received.status) {
            return Err(error_response(received));
        }
        if !received.body.is_empty() {
            return Err(received.protocol(ProtocolFailure::UnexpectedBody));
        }
        Ok(CloudResponse {
            body: (),
            status: received.status,
            metadata: received.metadata,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn send(
        &self,
        request: RequestBuilder,
        requested_url: &Url,
    ) -> Result<Received, CloudError> {
        let response = request
            .send()
            .await
            .map_err(|error| CloudError::Transport {
                failure: map_transport(&error),
                status: None,
                metadata: None,
            })?;
        let status = response.status();
        if response.url() != requested_url {
            return Err(CloudError::Protocol {
                failure: ProtocolFailure::RedirectedResponse,
                status: Some(status.as_u16()),
                metadata: ResponseMetadata::default(),
            });
        }
        let headers = response.headers().clone();
        let metadata = parse_metadata(&headers, status).map_err(|()| CloudError::Protocol {
            failure: ProtocolFailure::InvalidMetadata,
            status: Some(status.as_u16()),
            metadata: ResponseMetadata::default(),
        })?;

        if response.content_length().is_some_and(|length| {
            length > u64::try_from(self.config.max_response_bytes).expect("usize fits in u64")
        }) {
            return Err(CloudError::ResponseTooLarge {
                limit: self.config.max_response_bytes,
                status: status.as_u16(),
                metadata,
            });
        }

        let mut body = Vec::new();
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|error| CloudError::Transport {
                failure: map_transport(&error),
                status: Some(status.as_u16()),
                metadata: Some(metadata.clone()),
            })?;
            let next_length =
                body.len()
                    .checked_add(chunk.len())
                    .ok_or(CloudError::ResponseTooLarge {
                        limit: self.config.max_response_bytes,
                        status: status.as_u16(),
                        metadata: metadata.clone(),
                    })?;
            if next_length > self.config.max_response_bytes {
                return Err(CloudError::ResponseTooLarge {
                    limit: self.config.max_response_bytes,
                    status: status.as_u16(),
                    metadata,
                });
            }
            body.extend_from_slice(&chunk);
        }

        Ok(Received {
            status,
            headers,
            metadata,
            body,
        })
    }

    #[cfg(target_arch = "wasm32")]
    async fn send(
        &self,
        request: RequestBuilder,
        requested_url: &Url,
    ) -> Result<Received, CloudError> {
        crate::wasm_transport::send(&self.config, request, requested_url).await
    }
    fn request(&self, method: Method, url: Url, token: Option<&BearerToken<'_>>) -> RequestBuilder {
        let mut request = self.base_request(method, url);
        if let Some(token) = token {
            request = request.bearer_auth(token.secret());
        }
        request
    }

    fn request_with_share_token(
        &self,
        method: Method,
        url: Url,
        token: ShareToken<'_>,
    ) -> RequestBuilder {
        self.base_request(method, url)
            .header(AUTHORIZATION, token.authorization_value())
    }

    fn base_request(&self, method: Method, url: Url) -> RequestBuilder {
        let request = self
            .http
            .request(method, url)
            .header(ACCEPT, ACCEPT_JSON)
            .timeout(self.config.request_timeout);
        harden_platform_request(request)
    }

    fn url<const N: usize>(&self, segments: [&str; N]) -> Result<Url, CloudError> {
        let mut url = self.config.endpoint.clone();
        url.path_segments_mut()
            .map_err(|()| CloudError::Transport {
                failure: TransportFailure::Request,
                status: None,
                metadata: None,
            })?
            .clear()
            .extend(segments);
        Ok(url)
    }
}

fn require_valid_circuit(
    response: CloudResponse<Circuit>,
    expected_id: Option<Uuid>,
    expected_workspace_id: Option<Uuid>,
    status: StatusCode,
) -> Result<CloudResponse<Circuit>, CloudError> {
    if !valid_circuit(&response.body, expected_id, expected_workspace_id) {
        return Err(CloudError::Protocol {
            failure: ProtocolFailure::InvalidSuccessResponse,
            status: Some(status.as_u16()),
            metadata: response.metadata,
        });
    }
    Ok(response)
}

fn require_artifact(
    response: CloudResponse<Artifact>,
    expected_id: Uuid,
    expected_state: Option<ArtifactState>,
    status: StatusCode,
) -> Result<CloudResponse<Artifact>, CloudError> {
    if !valid_artifact(&response.body, Some(expected_id), None)
        || expected_state.is_some_and(|state| response.body.state != state)
    {
        return Err(CloudError::Protocol {
            failure: ProtocolFailure::InvalidSuccessResponse,
            status: Some(status.as_u16()),
            metadata: response.metadata,
        });
    }
    Ok(response)
}

fn require_shared_circuit(
    response: CloudResponse<SharedCircuit>,
    expected_circuit_id: Option<Uuid>,
    status: StatusCode,
) -> Result<CloudResponse<SharedCircuit>, CloudError> {
    if !valid_shared_circuit(&response.body, expected_circuit_id) {
        return Err(CloudError::Protocol {
            failure: ProtocolFailure::InvalidSuccessResponse,
            status: Some(status.as_u16()),
            metadata: response.metadata,
        });
    }
    Ok(response)
}

fn require_artifact_download(
    response: CloudResponse<ArtifactDownload>,
    expected_artifact_id: Uuid,
    mode: crate::EndpointMode,
    object_storage_origin: &Url,
    status: StatusCode,
) -> Result<CloudResponse<ArtifactDownload>, CloudError> {
    if !valid_artifact_download(
        &response.body,
        expected_artifact_id,
        mode,
        object_storage_origin,
    ) {
        return Err(CloudError::Protocol {
            failure: ProtocolFailure::InvalidSuccessResponse,
            status: Some(status.as_u16()),
            metadata: response.metadata,
        });
    }
    Ok(response)
}

impl fmt::Debug for CloudClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloudClient")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

/// Successful response body and its non-secret operational metadata.
pub struct CloudResponse<T> {
    body: T,
    status: StatusCode,
    metadata: ResponseMetadata,
}

impl<T> CloudResponse<T> {
    /// Borrows the typed response body.
    pub fn body(&self) -> &T {
        &self.body
    }

    /// Consumes the wrapper and returns the typed body.
    pub fn into_body(self) -> T {
        self.body
    }

    /// Returns the successful HTTP status accepted for this operation.
    pub fn status(&self) -> u16 {
        self.status.as_u16()
    }

    /// Returns support and caller-controlled retry metadata.
    pub fn metadata(&self) -> &ResponseMetadata {
        &self.metadata
    }
}

impl<T> fmt::Debug for CloudResponse<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CloudResponse")
            .field("body", &"[REDACTED]")
            .field("status", &self.status)
            .field("metadata", &self.metadata)
            .finish()
    }
}

pub(super) struct Received {
    pub(super) status: StatusCode,
    pub(super) headers: HeaderMap,
    pub(super) metadata: ResponseMetadata,
    pub(super) body: Vec<u8>,
}

impl Received {
    fn protocol(&self, failure: ProtocolFailure) -> CloudError {
        CloudError::Protocol {
            failure,
            status: Some(self.status.as_u16()),
            metadata: self.metadata.clone(),
        }
    }
}

fn error_response(received: Received) -> CloudError {
    if received.status.is_redirection() {
        return received.protocol(ProtocolFailure::RedirectedResponse);
    }
    if !has_media_type(&received.headers, "application/problem+json") {
        return received.protocol(ProtocolFailure::UnexpectedStatus);
    }
    let Ok(problem) = serde_json::from_slice::<ProblemDetails>(&received.body) else {
        return received.protocol(ProtocolFailure::InvalidProblem);
    };
    if problem.status != received.status.as_u16()
        || problem.kind.is_empty()
        || problem.title.is_empty()
    {
        return received.protocol(ProtocolFailure::InvalidProblem);
    }
    CloudError::Problem {
        details: Box::new(problem),
        metadata: received.metadata,
    }
}

fn has_media_type(headers: &HeaderMap, expected: &str) -> bool {
    headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<mime::Mime>().ok())
        .is_some_and(|value| value.essence_str().eq_ignore_ascii_case(expected))
}

pub(super) fn parse_metadata(
    headers: &HeaderMap,
    status: StatusCode,
) -> Result<ResponseMetadata, ()> {
    let request_id = single_header(headers, X_REQUEST_ID)?
        .map(|value| value.to_str().map_err(|_| ()))
        .transpose()?
        .map(|value| {
            if value.is_empty()
                || value.len() > MAX_REQUEST_ID_BYTES
                || !value.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':')
                })
            {
                Err(())
            } else {
                Ok(value.to_owned())
            }
        })
        .transpose()?;

    let retry_after = single_header(headers, RETRY_AFTER)?
        .map(|value| {
            let seconds = value
                .to_str()
                .map_err(|_| ())?
                .parse::<u64>()
                .map_err(|_| ())?;
            if seconds > MAX_RETRY_AFTER_SECONDS {
                return Err(());
            }
            Ok(Duration::from_secs(seconds))
        })
        .transpose()?;

    let idempotency_replayed = single_header(headers, IDEMPOTENCY_REPLAYED)?
        .map(|value| match value.as_bytes() {
            b"true" => Ok(true),
            b"false" => Ok(false),
            _ => Err(()),
        })
        .transpose()?;

    let location = if status.is_success() {
        single_header(headers, LOCATION)?
            .map(|value| value.to_str().map_err(|_| ()))
            .transpose()?
            .map(|value| {
                if value.len() > MAX_LOCATION_BYTES
                    || !value.starts_with("/api/")
                    || value.ends_with('/')
                    || value.contains("//")
                    || value
                        .split('/')
                        .skip(1)
                        .any(|segment| matches!(segment, "." | ".."))
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.')
                    })
                {
                    Err(())
                } else {
                    Ok(value.to_owned())
                }
            })
            .transpose()?
    } else {
        None
    };

    Ok(ResponseMetadata {
        request_id,
        retry_after,
        idempotency_replayed,
        location,
    })
}

fn single_header(
    headers: &HeaderMap,
    name: impl http::header::AsHeaderName,
) -> Result<Option<&http::HeaderValue>, ()> {
    let mut values = headers.get_all(name).iter();
    let value = values.next();
    if values.next().is_some() {
        return Err(());
    }
    Ok(value)
}

fn add_page_query(url: &mut Url, page: PageRequest<'_>) {
    let limit = page.limit().to_string();
    let mut pairs = url.query_pairs_mut();
    pairs.append_pair("limit", &limit);
    if let Some(cursor) = page.cursor() {
        pairs.append_pair("cursor", cursor);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn build_http_client() -> Result<reqwest::Client, CloudError> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .no_proxy()
        .user_agent(concat!("rspice-cloud-client/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|_| CloudError::Transport {
            failure: TransportFailure::ClientBuild,
            status: None,
            metadata: None,
        })
}

#[cfg(target_arch = "wasm32")]
fn build_http_client() -> Result<reqwest::Client, CloudError> {
    reqwest::Client::builder()
        .build()
        .map_err(|_| CloudError::Transport {
            failure: TransportFailure::ClientBuild,
            status: None,
            metadata: None,
        })
}

#[cfg(not(target_arch = "wasm32"))]
fn harden_platform_request(request: RequestBuilder) -> RequestBuilder {
    request
}

#[cfg(target_arch = "wasm32")]
fn harden_platform_request(request: RequestBuilder) -> RequestBuilder {
    request.fetch_credentials_omit().fetch_cache_no_store()
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use http::{HeaderValue, header::AUTHORIZATION};

    use super::*;

    #[test]
    fn native_authorization_headers_are_marked_sensitive() {
        let cloud = CloudClient::new(
            ClientConfig::loopback_development("http://127.0.0.1:8080", "http://127.0.0.1:9000")
                .expect("loopback client configuration"),
        )
        .expect("native HTTP client");
        let raw = "header.payload.signature";
        let token = BearerToken::new(raw).expect("valid bearer token");
        let request = cloud
            .request(
                Method::GET,
                Url::parse("http://127.0.0.1:8080/api/v1/me").expect("valid request URL"),
                Some(&token),
            )
            .build()
            .expect("build request");
        let authorization = request
            .headers()
            .get(AUTHORIZATION)
            .expect("authorization header");

        assert_eq!(authorization, "Bearer header.payload.signature");
        assert!(authorization.is_sensitive());
        assert!(!format!("{request:?}").contains(raw));

        let raw_share_token = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let share_token = ShareToken::new(raw_share_token).expect("valid share token");
        let share_request = cloud
            .request_with_share_token(
                Method::GET,
                Url::parse("http://127.0.0.1:8080/api/v1/shares/resolve").expect("valid share URL"),
                share_token,
            )
            .build()
            .expect("build share request");
        let share_authorization = share_request
            .headers()
            .get(AUTHORIZATION)
            .expect("share authorization header");
        assert_eq!(
            share_authorization.to_str().expect("ASCII authorization"),
            format!("Bearer {raw_share_token}")
        );
        assert!(share_authorization.is_sensitive());
        assert!(!format!("{share_request:?}").contains(raw_share_token));
    }

    #[test]
    fn request_paths_fail_closed_if_endpoint_authority_drifts() {
        let mut config =
            ClientConfig::loopback_development("http://127.0.0.1:8080", "http://127.0.0.1:9000")
                .expect("loopback client configuration");
        config.endpoint = Url::parse("mailto:invalid@example.test").expect("opaque URL fixture");
        let cloud = CloudClient::new(config).expect("native HTTP client");

        let error = cloud
            .url(["api", API_VERSION, "me"])
            .expect_err("opaque endpoint cannot accept hierarchical API paths");

        assert!(matches!(
            error,
            CloudError::Transport {
                failure: TransportFailure::Request,
                status: None,
                metadata: None,
            }
        ));
    }

    #[test]
    fn response_locations_are_strict_single_values_and_debug_redacted() {
        let location = "/api/v1/circuits/00000000-0000-0000-0000-000000000001/revisions/00000000-0000-0000-0000-000000000002";
        let mut headers = HeaderMap::new();
        headers.insert(LOCATION, HeaderValue::from_static(location));
        let metadata =
            parse_metadata(&headers, StatusCode::CREATED).expect("safe resource location");
        assert_eq!(metadata.location(), Some(location));
        assert!(!format!("{metadata:?}").contains(location));

        headers.append(LOCATION, HeaderValue::from_static("/api/v1/duplicate"));
        assert!(parse_metadata(&headers, StatusCode::CREATED).is_err());

        let mut unsafe_headers = HeaderMap::new();
        unsafe_headers.insert(
            LOCATION,
            HeaderValue::from_static("/api/v1/resource?token=secret"),
        );
        assert!(parse_metadata(&unsafe_headers, StatusCode::CREATED).is_err());

        unsafe_headers.insert(
            LOCATION,
            HeaderValue::from_static("/api/v1/circuits/../secrets"),
        );
        assert!(parse_metadata(&unsafe_headers, StatusCode::CREATED).is_err());

        let redirect = parse_metadata(&unsafe_headers, StatusCode::TEMPORARY_REDIRECT)
            .expect("redirect targets are not success metadata");
        assert!(redirect.location().is_none());
    }
}
