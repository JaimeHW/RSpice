//! Object identity.
//!
//! UUID-backed identifiers, and the parse errors that keep an invalid one
//! from being guessed into a valid object.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::ProductObjectKind;

macro_rules! define_uuid_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn try_from_uuid(value: Uuid) -> Result<Self, UuidIdError> {
                (!value.is_nil())
                    .then_some(Self(value))
                    .ok_or(UuidIdError::Nil)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = Uuid::deserialize(deserializer)?;
                Self::try_from_uuid(value).map_err(serde::de::Error::custom)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = UuidIdParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let value = Uuid::parse_str(value)?;
                Ok(Self::try_from_uuid(value)?)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum UuidIdError {
    #[error("product object identity must not be the nil UUID")]
    Nil,
}

#[derive(Debug, thiserror::Error)]
pub enum UuidIdParseError {
    #[error(transparent)]
    Parse(#[from] uuid::Error),
    #[error(transparent)]
    Invalid(#[from] UuidIdError),
}

/// The same identity plus its raw value, for the identities that are digested
/// or ordered by it.
macro_rules! define_raw_uuid_id {
    ($name:ident) => {
        define_uuid_id!($name);

        impl $name {
            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }
    };
}

/// The same again plus deterministic derivation within an explicitly
/// versioned namespace. Derivation is reserved for reproducible migrations
/// and immutable source projections; ordinary object creation uses
/// `Self::new`.
macro_rules! define_derivable_uuid_id {
    ($name:ident) => {
        define_raw_uuid_id!($name);

        impl $name {
            #[must_use]
            pub fn from_namespace(namespace: Uuid, name: &[u8]) -> Self {
                Self(Uuid::new_v5(&namespace, name))
            }
        }
    };
}

// The identities the product model actually mints. `DesignId`, `TestbenchId`,
// `RunSetId`, `VerificationPlanId`, `AutomationPipelineId`, `ModelBindingId`,
// and `ReleaseCandidateId` were declared here too and never constructed
// anywhere — vocabulary for object kinds that do not exist yet. Reintroducing
// one is a single line whenever the object arrives.
define_derivable_uuid_id!(ProjectId);
define_derivable_uuid_id!(SimulationPlanId);
define_derivable_uuid_id!(AnalysisInstanceId);
define_derivable_uuid_id!(DesignVariableId);
define_derivable_uuid_id!(SavedOutputId);
define_uuid_id!(JobId);
// A lifecycle transaction is a token, not a product object: it exists only to
// let a completion that arrives on a later frame prove it belongs to the
// operation that started it. It lives here because every layer that carries
// one — a dialog, a document, an import workflow — must be able to name it
// without reaching up to the lifecycle module that mints it.
define_uuid_id!(TransactionId);
define_derivable_uuid_id!(RunId);
define_derivable_uuid_id!(DatasetId);
define_raw_uuid_id!(ResultDocumentId);
define_derivable_uuid_id!(VerificationEvidenceId);
define_derivable_uuid_id!(ModelSourceId);

/// A type-erased reference used by receipts and cross-domain links. Domain
/// records retain their strongly typed IDs; erasure occurs only at boundaries
/// that must carry heterogeneous objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "ObjectRefData")]
pub struct ObjectRef {
    kind: ProductObjectKind,
    id: Uuid,
}

impl ObjectRef {
    pub fn new(kind: ProductObjectKind, id: Uuid) -> Result<Self, UuidIdError> {
        (!id.is_nil())
            .then_some(Self { kind, id })
            .ok_or(UuidIdError::Nil)
    }

    #[must_use]
    pub const fn kind(self) -> ProductObjectKind {
        self.kind
    }

    #[must_use]
    pub const fn id(self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
struct ObjectRefData {
    kind: ProductObjectKind,
    id: Uuid,
}

impl TryFrom<ObjectRefData> for ObjectRef {
    type Error = UuidIdError;

    fn try_from(value: ObjectRefData) -> Result<Self, Self::Error> {
        Self::new(value.kind, value.id)
    }
}

/// Monotonic revision of a versioned object. Zero is never a valid persisted
/// revision, which prevents an uninitialized value from masquerading as data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct ObjectRevision(u64);

impl ObjectRevision {
    pub const INITIAL: Self = Self(1);

    pub fn new(value: u64) -> Result<Self, RevisionError> {
        (value > 0)
            .then_some(Self(value))
            .ok_or(RevisionError::Zero)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn next(self) -> Result<Self, RevisionError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(RevisionError::Exhausted)
    }
}

impl Default for ObjectRevision {
    fn default() -> Self {
        Self::INITIAL
    }
}

impl<'de> Deserialize<'de> for ObjectRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum RevisionError {
    #[error("object revision must be greater than zero")]
    Zero,
    #[error("object revision space is exhausted")]
    Exhausted,
}

// Semantic digests are stamped into persisted design-management records, so
// the type is defined in `rspice-design-model` and named here through the
// module that has always owned product identity.
pub use rspice_design_model::ContentDigest;

/// A dataset named together with the exact bytes it held when it was bound.
/// Presentation that carries a binding rather than a bare `DatasetId` cannot
/// silently follow a dataset that was rewritten underneath it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DatasetBinding {
    pub dataset_id: DatasetId,
    pub content_digest: ContentDigest,
}

impl DatasetBinding {
    #[must_use]
    pub const fn new(dataset_id: DatasetId, content_digest: ContentDigest) -> Self {
        Self {
            dataset_id,
            content_digest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_ids_round_trip_without_losing_their_domain_type() {
        let id = ProjectId::new();
        let json = serde_json::to_string(&id).expect("serialize project id");
        let restored: ProjectId = serde_json::from_str(&json).expect("deserialize project id");

        assert_eq!(restored, id);
        assert_eq!(id.to_string().len(), 36);
        assert!(
            serde_json::from_str::<ProjectId>("\"00000000-0000-0000-0000-000000000000\"").is_err()
        );
        assert!(
            "00000000-0000-0000-0000-000000000000"
                .parse::<ProjectId>()
                .is_err()
        );
    }

    #[test]
    fn namespaced_ids_are_reproducible_and_domain_typed() {
        let namespace = Uuid::from_u128(0x3d4a_52cc_27ac_5fef_9c54_087d_e8a4_f57d);
        let first = AnalysisInstanceId::from_namespace(namespace, b"project-a/ac/legacy-0");
        let second = AnalysisInstanceId::from_namespace(namespace, b"project-a/ac/legacy-0");
        let other = AnalysisInstanceId::from_namespace(namespace, b"project-a/ac/legacy-1");

        assert_eq!(first, second);
        assert_ne!(first, other);
        assert!(!first.as_uuid().is_nil());
    }

    #[test]
    fn erased_object_references_reject_nil_identity() {
        let json = r#"{"kind":"project","id":"00000000-0000-0000-0000-000000000000"}"#;

        assert!(serde_json::from_str::<ObjectRef>(json).is_err());
    }

    #[test]
    fn revision_rejects_zero_and_overflow() {
        assert_eq!(ObjectRevision::new(0), Err(RevisionError::Zero));
        assert_eq!(
            ObjectRevision::new(u64::MAX)
                .expect("maximum revision is representable")
                .next(),
            Err(RevisionError::Exhausted)
        );
        assert!(serde_json::from_str::<ObjectRevision>("0").is_err());
    }

    #[test]
    fn digest_is_canonical_and_rejects_malformed_data() {
        let uppercase = "AB".repeat(32);
        let digest: ContentDigest = uppercase.parse().expect("parse uppercase digest");
        assert_eq!(digest.to_string(), "ab".repeat(32));
        assert_eq!(
            serde_json::from_str::<ContentDigest>(&format!("\"{}\"", "ab".repeat(32)))
                .expect("deserialize digest"),
            digest
        );
        assert!("ab".repeat(31).parse::<ContentDigest>().is_err());
        assert!(
            format!("{}gg", "ab".repeat(31))
                .parse::<ContentDigest>()
                .is_err()
        );
    }
}
