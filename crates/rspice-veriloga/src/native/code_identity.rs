//! Machine-code identity census over the shipped Verilog-A model set.
//!
//! The block model is meant to change nothing about what a single-block
//! program compiles to. This census makes that measurable rather than
//! argued: it compiles every shipped module through the production x64 path
//! and digests the published image, so the same run on any two revisions
//! either produces the same census digest or names the model that moved.
//!
//! It is `#[ignore]`d because compiling the whole compact-model census is
//! release-qualification work, not a per-commit gate. Run it with
//! `--release --features native -- --ignored --nocapture`.

use super::census_models::shipped_census_models;

/// How many modules the shipped model tree declares.
///
/// Pinned beside the digest because the digest alone cannot tell "no module
/// moved" from "the corpus shrank": a census that discovered forty-two modules
/// would produce a different digest and say nothing about which one it lost.
const SHIPPED_CENSUS_MODELS: usize = 43;

/// The combined machine-code digest of the shipped corpus.
///
/// # This is a drift detector, not a proof
///
/// Nothing about this number says the emitted code is *correct*. What it says
/// is that the code emitted for the shipped corpus is the same code as when it
/// was last measured, so a change that was supposed to be a refactor and moved
/// a byte says so here instead of in a customer's transient.
///
/// # How it is re-baselined
///
/// Only with evidence that names the modules that moved and why. The census
/// prints a per-module digest before the combined one, so a run that changes
/// this value already knows which modules are responsible; a re-baseline
/// commit has to say which those were and what emitted-code change accounts
/// for each. "The census turned red so the constant was updated" is the one
/// move this pin exists to prevent — it converts a detector into a rubber
/// stamp.
///
/// # This value is stale by construction, and was not re-measured
///
/// W-F3c flipped [`crate::jit::cfg_plan_builder::build_default_model_plan`] over
/// to the CFG route for every module's `stamp_values`, `jacobians` and
/// `reactive_jacobians`, and this census compiles through
/// [`crate::native::x64::compile_model_with_canonical_ir`], which is that path.
/// So every module the CFG route builds emits different machine code now, and
/// the value below cannot be the answer.
///
/// It was not re-measured in that lane: a full census peaks near 24 GB of
/// working set, and the box had 19.2 GB free with twenty-two peer `rustc`
/// processes resident. Rather than stamp in an unmeasured number, the old value
/// stands and the assertion fails loudly, which is the honest state.
///
/// What the next run has to do is more than copy the digest it prints. The
/// per-module list is now evidence about the flip:
///
/// * a module the flip's fallback logs by name (`[JIT] Model '…' takes the
///   postfix plan`) must be **digest identical**, because nothing about its
///   plan changed;
/// * every other module is expected to move, and a module that does *not* move
///   means its residual and Jacobian entries did not take the CFG route after
///   all;
/// * a module that fails to compile at all is the flip's defect, not a
///   re-baseline.
///
/// # How it is re-baselined
///
/// Only with evidence that names the modules that moved and why. The census
/// prints a per-module digest before the combined one, so a run that changes
/// this value already knows which modules are responsible; a re-baseline
/// commit has to say which those were and what emitted-code change accounts
/// for each. "The census turned red so the constant was updated" is the one
/// move this pin exists to prevent — it converts a detector into a rubber
/// stamp.
///
/// The value below was measured on the W-F3b tree, before the flip. The
/// previous value, `fe9a2072…`, moved to `e00c6b88…` at W-F3a (`84ba2c2bb`),
/// which ruled `limexp`'s threshold once for the whole estate: exactly five
/// modules changed — `angelov`, `angelov_gan`, `hicumL0va`, `hicumL2va` and
/// `vbic_4T_et_cf`, the five that call `limexp` — and the other thirty-eight
/// were digest identical across that change.
const SHIPPED_CENSUS_DIGEST: &str =
    "e00c6b8818e88bba07840df35b68f69687bba439910b56b2b76c9ffe5ae601a3";

/// One shipped module's compiled machine-code digest.
struct ModelImageDigest {
    name: String,
    digest: String,
    bytes: usize,
    helper_calls: usize,
    /// Front end plus native compilation, so a run that slows down says which
    /// module it slowed down on rather than only that the census took longer.
    seconds: f64,
}

/// Blank the host addresses an image embeds, so its digest survives ASLR.
///
/// A runtime helper is called as `movabs <reg>, <address>` immediately
/// followed by `call <reg>`, and that address is where the loader happened to
/// place the process this run. Requiring the call to name the same register
/// the move loaded is what keeps a mask constant — the encoder emits those
/// through the same `movabs` — out of the normalization. Everything else in
/// the image is position independent: literals are RIP relative and entry
/// calls are image relative.
fn normalize_host_addresses(image: &[u8]) -> (Vec<u8>, usize) {
    let mut normalized = image.to_vec();
    let mut helper_calls = 0;
    let mut offset = 0;
    while offset + 10 <= normalized.len() {
        let rex = normalized[offset];
        let opcode = normalized[offset + 1];
        if (rex == 0x48 || rex == 0x49) && (0xB8..=0xBF).contains(&opcode) {
            let register = (opcode - 0xB8) | ((rex & 1) << 3);
            let after = offset + 10;
            let call = match (rex & 1, normalized.get(after..after + 3)) {
                // call r0-r7: FF /2
                (0, Some([0xFF, modrm, _])) if *modrm == 0xD0 | register => Some(2),
                // call r8-r15: REX.B FF /2
                (1, Some([0x41, 0xFF, modrm])) if *modrm == 0xD0 | (register & 7) => Some(3),
                _ => None,
            };
            if call.is_some() {
                normalized[offset + 2..offset + 10].fill(0);
                helper_calls += 1;
                offset = after;
                continue;
            }
        }
        offset += 1;
    }
    (normalized, helper_calls)
}

fn census() -> Vec<ModelImageDigest> {
    let mut digests = Vec::new();
    let mut total_compile_seconds = 0.0_f64;
    let mut total_census_seconds = 0.0_f64;
    for shipped in shipped_census_models() {
        let module = &shipped.name;
        let started = std::time::Instant::now();
        let native = crate::native::x64::compile_model_with_canonical_ir(
            &shipped.model,
            &shipped.canonical_ir,
        )
        .unwrap_or_else(|error| {
            panic!(
                "native compile {} :: {module}: {error}",
                shipped.path.display()
            )
        });
        let image = native.image_bytes();
        let (normalized, helper_calls) = normalize_host_addresses(image);
        let census_seconds = started.elapsed().as_secs_f64();
        // The front end is shared with the other two censuses and the native
        // compile is not, so a run that slows down says which half slowed.
        eprintln!(
            "code-identity model={module} compile_seconds={:.1} census_seconds={census_seconds:.1} cached={}",
            shipped.compile_seconds, shipped.from_cache
        );
        total_compile_seconds += shipped.compile_seconds;
        total_census_seconds += census_seconds;
        digests.push(ModelImageDigest {
            name: module.to_string(),
            digest: blake3::hash(&normalized).to_hex().to_string(),
            bytes: image.len(),
            helper_calls,
            seconds: shipped.compile_seconds + census_seconds,
        });
    }
    eprintln!(
        "code-identity total_compile_seconds={total_compile_seconds:.1} total_census_seconds={total_census_seconds:.1}"
    );
    digests.sort_by(|left, right| left.name.cmp(&right.name));
    digests
}

#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn shipped_model_machine_code_census_digest() {
    let census = census();
    assert!(
        !census.is_empty(),
        "the shipped model tree produced no compilable module"
    );
    let mut combined = blake3::Hasher::new();
    for entry in &census {
        eprintln!(
            "code-identity model={} bytes={} helper_calls={} seconds={:.1} digest={}",
            entry.name, entry.bytes, entry.helper_calls, entry.seconds, entry.digest
        );
        combined.update(entry.name.as_bytes());
        combined.update(entry.digest.as_bytes());
    }
    let digest = combined.finalize().to_hex().to_string();
    eprintln!(
        "code-identity models={} census_digest={digest}",
        census.len(),
    );
    assert_eq!(
        census.len(),
        SHIPPED_CENSUS_MODELS,
        "the shipped model tree no longer declares {SHIPPED_CENSUS_MODELS} compilable modules"
    );
    assert_eq!(
        digest, SHIPPED_CENSUS_DIGEST,
        "the shipped corpus's emitted machine code moved; the per-module digests above name \
         which modules, and re-baselining SHIPPED_CENSUS_DIGEST means naming them and the \
         emitted-code change that accounts for each"
    );
}
