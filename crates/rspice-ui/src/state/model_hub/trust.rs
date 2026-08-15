//! The single trust anchor every Model Hub verdict is reached under.
//!
//! One key and one set of format limits, in one place. The snapshot decode,
//! the archive verification, and every re-proof of an installed pack all read
//! this value, so there is no second, weaker authority anywhere in the client
//! and no code path that can be persuaded to skip verification by supplying a
//! different key.

use rspice_pack::{Limits, VerifyingKey, verifying_key_from_hex};

/// Development placeholder for the Model Hub catalog signing key.
///
/// **This value is replaced at the release-key ceremony.** It is the public
/// half of a well-known development seed, published here so a developer build
/// can exercise the whole pipeline against locally signed packs. It grants
/// nothing: a pack signed by the production key will not verify against it,
/// and a pack signed by this key will not verify against production once the
/// ceremony replaces the constant.
///
/// Replacing it is the entire ceremony as far as this client is concerned —
/// there is no configuration file, environment variable, or command-line
/// switch that can substitute a key, because a client that can be pointed at
/// another key is a client an attacker can point at their own.
pub(crate) const CATALOG_VERIFYING_KEY_HEX: &str =
    "d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737";

/// The key and format limits every verification runs under.
///
/// Tests construct one from their own key through
/// [`TrustAnchor::from_verifying_key`]; that is the only seam, and it takes a
/// key rather than a flag, so no test can turn verification *off*.
#[derive(Debug, Clone)]
pub struct TrustAnchor {
    key: VerifyingKey,
    limits: Limits,
}

impl TrustAnchor {
    /// The anchor a shipped build verifies under.
    ///
    /// Fails only if [`CATALOG_VERIFYING_KEY_HEX`] is not a canonical ed25519
    /// point, which a unit test in this module proves it is.
    pub fn release() -> Result<Self, super::ModelHubError> {
        verifying_key_from_hex(CATALOG_VERIFYING_KEY_HEX)
            .map(Self::from_verifying_key)
            .map_err(|_| super::ModelHubError::TrustAnchorUnusable)
    }

    /// Builds an anchor around an explicit key, under the format limits.
    pub fn from_verifying_key(key: VerifyingKey) -> Self {
        Self {
            key,
            limits: Limits::default(),
        }
    }

    pub const fn key(&self) -> &VerifyingKey {
        &self.key
    }

    pub const fn limits(&self) -> &Limits {
        &self.limits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shipped_anchor_is_a_canonical_ed25519_key() {
        let anchor = TrustAnchor::release().expect("the compiled-in anchor must be usable");
        assert_eq!(anchor.limits().max_snapshot_bytes, 32 * 1024 * 1024);
        assert_eq!(anchor.limits().max_archive_bytes, 64 * 1024 * 1024);
        assert_eq!(
            rspice_pack::encode_hex(anchor.key().as_bytes()),
            CATALOG_VERIFYING_KEY_HEX
        );
    }
}
