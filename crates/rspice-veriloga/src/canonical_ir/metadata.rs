//! Provenance metadata and the reproducible digest used to stamp it.
//!
//! [`CanonicalMetadata`] records which source package a model was compiled
//! from; [`StableDigest`] provides the compact reproducible digest historically
//! exposed by compiled model descriptors. Security-sensitive source identity
//! is recorded independently as a full BLAKE3 digest.

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Canonical HIR/MIR artifact version emitted and accepted by this build.
///
/// This is a hard compatibility boundary: caches and external backends must
/// never deserialize a structurally different artifact merely because its HIR
/// and metadata happen to repeat the same stale version number.
///
/// Version 54 recompiles eager four-state conditionals with branch-controlled evaluation.
/// Version 53 preserves control flow in independently observed digital expressions.
/// Version 52 retains numeric real/integer conversions in digital CFGs.
/// Version 51 retains indirect-equation absolute tolerances.
/// Version 50 validates and retains nature inheritance and physical declaration closures.
/// Version 49 exposes indirect source activation to runtime topology guards.
/// Version 48 validates indirect constraint controls, left sides, and uniqueness.
/// Version 47 retains normalized repeat counts and repeated event subscriptions.
/// Version 46 executes computed event expressions and runtime bit indices.
/// Version 45 rejects direct/indirect source conflicts across parallel branches.
/// Version 44 retains ordered switch-branch source kinds and accepted mode state.
/// Version 43 retains event-controlled nonblocking capture subscriptions.
/// Version 42 captures delayed nonblocking writes independently of their process.
/// Version 41 evaluates procedural delay expressions in their activation.
/// Version 40 adds digital clock queries and exact wide digital expressions.
/// Version 39 links each contribution to its shared physical branch unknown.
/// Version 38 resolves declaration-owned time queries before hierarchy flattening.
/// Version 37 retains module time scales and resolved digital delay ticks.
/// Version 36 resolves ground qualifiers and disciplines before allocating solver nodes.
/// Version 35 retains potential branch identities and a shared contribution direction.
/// Version 34 preserves module-local branches, nested port flows, and coincident-probe derivatives.
/// Version 33 retains the active connection source closure for every runtime transport.
/// Version 32 combines authenticated mixed parameter source and discrete analog state inputs
/// with the simultaneous flow-source probe equations introduced independently in version 31.
/// Version 31 gives flow-source probes simultaneous solver equations.
/// Version 30 distinguishes transient discontinuities from Newton convergence hints.
/// Version 29 retains control tasks in structured HIR and resets them before runtime loops.
/// Version 28 gives implicit integrators feedback-determined solver unknowns.
/// Version 27 protects scalar and packed derivative quotients against intermediate range loss.
/// Version 26 adds numerical value selection to the serialized CFG vocabulary.
/// Version 25 retains the primal validation dependency of symbolic derivatives.
/// Version 24 retains signed integer arithmetic before real conversion.
/// Earlier artifacts erased these operator types and must be recompiled.
/// Version 23 separates global-event analysis filters from phase-sensitive
/// analysis queries. Old filters cannot be recovered from lowered expressions.
/// Version 22 rejects analog $fatal, $stop, and initialization $error before
/// folding; earlier artifacts may have irrecoverably discarded those calls.
/// Version 21 separates declaration and analog initialization from Newton evaluation.
/// Version 20 retains ordered analog task calls and their source/phase metadata.
/// Version 19 rejects unrepresentable digital select and delay constants instead
/// of clamping them. Earlier artifacts must be rebuilt from source. Version 18
/// fixed constant integer comparisons; version 17 fixed digital range arithmetic.
pub const CANONICAL_IR_SCHEMA_VERSION: u32 = 54;

/// Collision-resistant identity of one exact preprocessed source closure.
pub fn source_identity(source_text: &str) -> String {
    blake3::hash(source_text.as_bytes()).to_hex().to_string()
}

pub(crate) fn is_source_identity(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Stable deterministic digest for canonical IR metadata.
///
/// This uses a fixed 64-bit FNV-1a-style hash to make metadata reproducible
/// across runs and platforms. It is not a cryptographic digest and must not be
/// used for trust, authentication, or collision-resistant identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StableDigest(u64);

impl StableDigest {
    pub fn from_text(text: &str) -> Self {
        let mut hash = 0xcbf29ce484222325u64;
        for byte in text.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        Self(hash)
    }

    pub fn as_hex(self) -> String {
        format!("{:016x}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMetadata {
    pub schema_version: u32,
    /// Stable logical identity of the root source document.
    ///
    /// File-backed compilation stores a portable path relative to the first
    /// configured include root containing the file, or just the file name for
    /// a standalone source. Physical paths remain file-API dependency and
    /// diagnostic data and are intentionally excluded from canonical identity.
    pub source_package: SmolStr,
    pub source_digest: SmolStr,
    /// Full BLAKE3 identity of the exact preprocessed source closure.
    pub source_identity: SmolStr,
    pub compiler_version: SmolStr,
    pub feature_flags: Vec<SmolStr>,
}

impl CanonicalMetadata {
    pub fn for_source(source_package: impl Into<SmolStr>, source_text: &str) -> Self {
        Self {
            schema_version: CANONICAL_IR_SCHEMA_VERSION,
            source_package: source_package.into(),
            source_digest: StableDigest::from_text(source_text).as_hex().into(),
            source_identity: source_identity(source_text).into(),
            compiler_version: env!("CARGO_PKG_VERSION").into(),
            feature_flags: Vec::new(),
        }
    }
}
