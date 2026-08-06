//! The qualification-corpus generator binary.
//!
//! Realizes every authored case against the live engine, assembles the
//! policy and corpus documents, self-validates them exactly as release
//! admission will, and writes both files as their canonical byte
//! serializations (the corpus binds to the policy by the digest of those
//! literal bytes). The authoritative run happens in the publishing lane on
//! linux/amd64 at the same source revision as the attested adapter image;
//! runs elsewhere prove structure and oracle math, not final hashes.

use std::process::ExitCode;

use rspice_engine_adapter::wire::digest_hex;
use rspice_qualification_corpus::{contract, emit};
use sha2::{Digest, Sha256};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("corpus generation failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut arguments = std::env::args().skip(1);
    let output = arguments
        .next()
        .ok_or("usage: rspice-qualification-corpus <output-directory>")?;
    if arguments.next().is_some() {
        return Err("exactly one argument, the output directory, is accepted".to_owned());
    }

    // The engine parallelizes device evaluation through rayon, and the
    // qualification harness compares series hashes bit for bit; the shipped
    // adapter pins its pool to one thread, so generation must match.
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .map_err(|error| format!("failed to pin the compute pool: {error}"))?;

    let (policy_bytes, corpus_bytes) = emit::canonical_documents()?;
    let policy_digest: [u8; 32] = Sha256::digest(&policy_bytes).into();
    let policy_sha256 = digest_hex(&policy_digest);
    let corpus_digest: [u8; 32] = Sha256::digest(&corpus_bytes).into();
    let summary = contract::validate_policy_and_corpus(&policy_bytes, &corpus_bytes)?;

    let directory = std::path::Path::new(&output);
    std::fs::create_dir_all(directory)
        .map_err(|error| format!("cannot create {}: {error}", directory.display()))?;
    let policy_path = directory.join("simulator-qualification-policy.json");
    let corpus_path = directory.join("simulator-qualification-corpus.json");
    std::fs::write(&policy_path, &policy_bytes)
        .map_err(|error| format!("cannot write {}: {error}", policy_path.display()))?;
    std::fs::write(&corpus_path, &corpus_bytes)
        .map_err(|error| format!("cannot write {}: {error}", corpus_path.display()))?;

    println!(
        "policy   {} bytes  sha256 {policy_sha256}",
        policy_bytes.len()
    );
    println!(
        "corpus   {} bytes  sha256 {}",
        corpus_bytes.len(),
        digest_hex(&corpus_digest)
    );
    println!(
        "cases    {} across {} executions, case set sha256 {}",
        summary.case_count, summary.execution_count, summary.case_set_sha256
    );
    for (category, count) in &summary.category_counts {
        println!("  {category:>32} {count}");
    }
    Ok(())
}
