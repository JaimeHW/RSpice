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
/// Version 143 resolves inherited digital-net disciplines before selecting converters.
/// Version 142 validates port discipline compatibility and retains local branch natures.
/// Version 141 resolves structural concatenations before real-net port lowering.
/// Version 140 resolves connected wire/tri net types per concrete HDL occurrence.
/// Version 139 resolves conservative array coordinates and ranged grounds before lowering.
/// Version 138 binds real bus shapes and formal views through original resolved drivers.
/// Version 137 retains transitive real-net aliases through linking and execution.
/// Version 136 retains inline discrete disciplines and discipline-first typed ports.
/// Version 135 binds real-net array elements and scalar ports to shared signal identities.
/// Version 132 retains computed mixed-input operands and physical converter bit ownership.
/// Version 131 preserves local packed bounds and signedness across whole-net ports.
/// Version 125 preserves physical vector lanes and their elaboration dependencies.
/// Version 124 retains source assignments and hierarchy for configuration replay.
/// Version 123 retains explicit connection libraries and selected blocks for replay.
/// Version 122 materializes selected connect bodies inside typed HDL hierarchy.
/// Version 121 emits numeric parameter arrays through the generated Rust backend.
/// Version 120 initializes read-only numeric parameter arrays for shared execution.
/// Version 119 specializes array, packed, and structural hierarchy overrides together.
/// Version 118 expands bounded, parameter-specialized variable array patterns.
/// Version 117 retains atomic aggregate analog assignment occurrence groups.
/// Version 116 lowers whole-array values with complete RHS capture and declared direction.
/// Version 115 adds ordered analog counter effects to executable control flow.
/// Version 114 retains continuous array shapes, coordinates, and shaped analog probe groups.
/// Version 113 retains multidimensional digital array shapes and checked coordinate operations.
/// Version 107 retains event-assigned analog reads and continuous-driver dependencies.
/// Version 106 binds analog event occurrence probes to private digital signals.
/// Version 105 binds relocated mixed-hierarchy numeric inputs to explicit digital signals.
/// Version 104 preserves RHS signedness when initializing integer parameters.
/// Version 103 guards scalar dependencies of compiled digital behavior and numeric array shapes.
/// Version 102 guards supplied-state dependencies of elaborated structure.
/// Version 101 retains supplied-state identity for exact elaboration parameters.
/// Version 100 specializes analog-child scalar parameters before hierarchy flattening.
/// Version 99 preserves packed parameter declarations and elaboration-bound inputs.
/// Version 98 infers native scalar parameter types from final assignment values.
/// Version 97 preserves interleaved parameter/localparam declaration dependencies.
/// Version 96 retains exact private localparams through analog and digital hierarchy.
/// Version 95 closes dependent defaults over immutable exact parameter values.
/// Version 94 retains exact packed elaboration parameters outside numeric runtime slots.
/// Version 93 retains packed operand evaluation before real conversion in dependent defaults.
/// Version 92 evaluates replication operands once, including zero-count operands,
/// and uses iterative concatenation and selection lowering.
/// Version 91 represents real unary negation as exact sign inversion.
/// Version 90 short-circuits logical expressions in the discrete domain.
/// Version 89 sizes comparison operands before evaluating their arithmetic.
/// Version 88 shares typed closed scalar parameter/default evaluation across domains.
/// Version 87 specializes digital child parameters and uses typed generate constants.
/// Version 86 gives process-local arrays scoped, scheduler-visible element storage.
/// Version 85 adds exact digital integer power and corrects power associativity.
/// Version 84 binds static local initializers in complete lexical scopes and source order.
/// Version 83 gives observed and deferred process locals shared signal storage.
/// Version 82 adds pure four-state packed updates for process-local writes.
/// Version 81 validates analog packed shapes and fixed write targets/types.
/// Version 80 captures dynamic bit writes and authored packed write bounds.
/// Version 79 admits constant clipped selections of digital vectors and integers.
/// Version 78 validates packed selection widths, directions and integral types before lowering.
/// Version 77 preserves analog integer packed-read ranges and indexed probes.
/// Version 76 folds digital select bounds with typed exact expression semantics.
/// Version 75 retains dynamic packed selectors and encoded known-bit chunks.
/// Version 74 binds packed discrete selections before analog numeric conversion.
/// Version 73 adds packed array-element selects and captures partial write targets.
/// Version 72 adds checked indexed reads of immutable projected input arrays.
/// Version 71 shares one selector for each discrete value/validity array read.
/// Version 70 checks discrete-input validity at executed analog reads.
/// Version 69 adds discrete array cells and runtime-indexed read/write operations.
/// Version 68 recompiles array shapes/constant indices without float narrowing.
/// Version 67 retains runtime-indexed analog array reads in digital plans.
/// Version 66 carries typed numeric declaration initialization in digital storage.
/// Version 65 preserves table derivative payloads until final division.
/// Version 64 defines exact mixed input/timing absdelay derivative actions.
/// Version 63 bounds recursive derivatives with guarded parameter-range expansion.
/// Version 62 keeps inlined analog-function storage in the private namespace.
/// Version 61 resolves typed digital parameter values before process lowering.
/// Version 60 adds checked runtime-index selection to canonical SSA.
/// Version 59 retains procedural entry inputs and zero-seeded derivative lifetimes.
/// Version 58 retains per-site DDT candidate derivatives and checked companion arithmetic.
/// Version 57 preserves integer ownership and cross-domain signedness.
/// Version 56 retains typed analog-owned-variable reads in digital plans.
/// Version 55 retains typed node and named-branch probes in digital plans.
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
pub const CANONICAL_IR_SCHEMA_VERSION: u32 = 143;

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
