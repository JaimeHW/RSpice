//! Canonical document emission shared by the generator binary and the
//! qualification harness: realize every authored case against the live
//! engine and serialize the policy and corpus as the exact bytes release
//! admission validates.

use serde_json::json;
use sha2::{Digest, Sha256};

use crate::canonical::canonical_json;
use crate::capture::realize;
use crate::{contract, families};

/// The policy and corpus as their canonical byte serializations, already
/// self-validated with the exact admission checks.
pub fn canonical_documents() -> Result<(Vec<u8>, Vec<u8>), String> {
    let policy = contract::build_policy();
    let policy_bytes = canonical_json(&policy)
        .ok_or("policy must serialize canonically")?
        .into_bytes();
    let policy_digest: [u8; 32] = Sha256::digest(&policy_bytes).into();
    let policy_sha256 = rspice_engine_adapter::wire::digest_hex(&policy_digest);

    let mut drafts = families::all();
    drafts.sort_by(|left, right| left.id.cmp(&right.id));
    let mut cases = Vec::with_capacity(drafts.len());
    for draft in drafts {
        cases.push(realize(draft)?);
    }

    let corpus = json!({
        "format_version": 1,
        "suite": contract::SUITE,
        "suite_version": 1,
        "profile": contract::PROFILE,
        "profile_version": 1,
        "policy_sha256": policy_sha256,
        "cases": cases,
    });
    let corpus_bytes = canonical_json(&corpus)
        .ok_or("corpus must serialize canonically")?
        .into_bytes();
    contract::validate_policy_and_corpus(&policy_bytes, &corpus_bytes)?;
    Ok((policy_bytes, corpus_bytes))
}
