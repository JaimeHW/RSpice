#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Security-conscious, platform-neutral HTTP client for RSpice Cloud.
//!
//! This crate sits above [`rspice_cloud_contract`]. It supports native Rust
//! applications and browser WebAssembly without owning credentials, retrying
//! mutations, accepting redirects, or buffering unbounded response bodies.

mod artifacts;
mod auth;
mod circuits;
mod client;
mod clock;
mod collaboration;
mod config;
mod error;
mod governance;
mod idempotency;
mod identity;
mod invitations;
mod licensing;
mod live_sessions;
#[cfg(not(target_arch = "wasm32"))]
mod native_licensing;
#[cfg(not(target_arch = "wasm32"))]
mod native_transfer;
mod pagination;
mod publications;
mod shares;
mod simulations;
mod transfer;
mod validation;
#[cfg(target_arch = "wasm32")]
mod wasm_transfer;
#[cfg(target_arch = "wasm32")]
mod wasm_transport;
mod workspaces;

pub use auth::{BearerToken, CredentialError};
pub use circuits::CreatedCircuitRevision;
pub use client::{CloudClient, CloudResponse};
pub use config::{ClientConfig, ConfigurationError, EndpointMode};
pub use error::{CloudError, ProtocolFailure, ResponseMetadata, TransportFailure};
pub use idempotency::{IdempotencyKey, IdempotencyKeyError};
pub use invitations::CreateWorkspaceInvitation;
#[cfg(not(target_arch = "wasm32"))]
pub use native_licensing::{
    NativeLicenseVerificationError, NativeLicenseVerificationFailure, NativeLicenseVerifier,
    VerifiedNativeLicense,
};
pub use pagination::{PageRequest, PaginationError};
pub use publications::{PublicationSlug, PublicationSlugError};
pub use shares::{
    CapabilityTokenError, CreateCircuitShare, InvitationToken, InvitationTokenError, ShareToken,
    ShareTokenError,
};
pub use transfer::{
    ArtifactTransferError, ArtifactTransferFailure, ArtifactTransferReceipt,
    MAX_ARTIFACT_SINK_CHUNK_BYTES,
};

pub use rspice_cloud_contract as contract;
