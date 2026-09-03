//! Parent-run identity of a compressed transient result.
//!
//! The analysis card, the shared-deck coordinate and the topology fingerprint
//! are what tie a restored result back to the run that produced it, so the
//! rebuild rejects a digest of the wrong width rather than dropping it.

use super::*;

/// Analysis identity, coordinate identity, and topology fingerprint.
pub(crate) type CompressedIdentityPersistenceState = (
    Option<(String, u32)>,
    Option<(Vec<u8>, u32, usize, String)>,
    Option<Vec<u8>>,
);

pub(crate) fn identity_persistence_state(
    identity: &rspice_core::engine::TransientResultIdentity,
) -> CompressedIdentityPersistenceState {
    (
        identity
            .analysis
            .as_ref()
            .map(|analysis| (analysis.kind_tag.clone(), analysis.ordinal)),
        identity.coordinate.as_ref().map(|coordinate| {
            (
                coordinate.semantic.to_vec(),
                coordinate.occurrence,
                coordinate.ordinal,
                coordinate.label.clone(),
            )
        }),
        identity
            .topology_fingerprint
            .map(|fingerprint| fingerprint.to_vec()),
    )
}

pub(crate) fn rebuild_identity(
    state: CompressedIdentityPersistenceState,
) -> PyResult<rspice_core::engine::TransientResultIdentity> {
    let (analysis, coordinate, topology) = state;
    let analysis = analysis
        .map(|(kind_tag, ordinal)| {
            rspice_core::engine::TransientAnalysisIdentity::new(kind_tag, ordinal)
                .map_err(crate::errors::value_error)
        })
        .transpose()?;
    let coordinate = coordinate
        .map(|(semantic, occurrence, ordinal, label)| {
            let semantic: [u8; 16] = semantic.try_into().map_err(|_| {
                crate::errors::value_error(
                    "compressed-transient coordinate identity requires a 16-byte semantic digest",
                )
            })?;
            rspice_core::engine::TransientCoordinateIdentity::new(
                semantic, occurrence, ordinal, label,
            )
            .map_err(crate::errors::value_error)
        })
        .transpose()?;
    let topology_fingerprint = topology
        .map(|bytes| {
            let bytes: [u8; 32] = bytes.try_into().map_err(|_| {
                crate::errors::value_error(
                    "compressed-transient topology fingerprint requires 32 bytes",
                )
            })?;
            Ok::<_, PyErr>(bytes)
        })
        .transpose()?;
    Ok(rspice_core::engine::TransientResultIdentity {
        analysis,
        coordinate,
        topology_fingerprint,
    })
}
