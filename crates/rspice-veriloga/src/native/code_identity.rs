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

use std::path::PathBuf;

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
/// # The noise magnitudes joining the prelude
///
/// `0899a73a…` moved to `8c19a2f5…` at W-F13a, when the noise magnitudes
/// stopped being per-entry cones and became prelude slots like every other
/// value entry. The reading is the same shape as the flip's below, with one
/// more case in it: a module's image moves **exactly when it has a noise
/// source the CFG route lowers**, so the modules that stay identical are the
/// two the route refuses plus the two that carry no noise at all.
///
/// Four are identical and thirty-nine moved. `mvsg_cmc` (2,861,548 bytes) and
/// the `BSIM_SOI_100.1.1` `bsimsoi` (9,498,652) are the W-D refusals named
/// below — byte for byte what they were before this change, which is what says
/// a refused module's plan is untouched by it. `EPFL_HEMT_10a` (241,272) and
/// `vbic_4T_et_cf` (344,160) are the other case: they are the only two shipped
/// modules with no noise source, so there was no magnitude of theirs to move.
/// Every one of the thirty-nine that moved has at least one.
///
/// The images grew by 442,240 bytes over the whole corpus — 486,705,936 to
/// 487,148,176, 0.09 per cent — and not one shrank. The largest factor is
/// `angelov` at 4.77 per cent (193,556 to 202,788); it and `angelov_gan` are
/// the two modules the frame census reads as publishing sixteen of twenty-two
/// magnitudes, the largest hoisted remainder in the corpus. The five the cost
/// census reads are `asmesd` 0.74, `vbic13_4t` 0.42, `hicumL2va` 0.09,
/// `bsimcmg_va` 0.76 and `asmhemt` 0.02 per cent. The two readings the flip
/// singled out barely move: `l_utsoi` 0.042 per cent and the three `hisimhv`
/// variants 0.005 — they are past the 60 MiB budget for the flip's reasons and
/// not for this change's.
///
/// Measured 2026-09-04, peers idle, 558 s, and reproduced per module at 178 s.
///
/// # The CFG plan flip, and how the previous value was re-baselined for it
///
/// `79e2c753…` moved to `0899a73a…` when production stopped compiling the
/// postfix plan. The census compiles through
/// [`crate::native::x64::compile_model_with_canonical_ir`], which is exactly the
/// path that changed, so the digest moving is the expected result and the
/// evidence is the per-module list read a particular way: **every** module the
/// fallback names must be digest identical, because nothing about its plan
/// changed, and **every other** module must move, because one that did not would
/// be one that never took the CFG route.
///
/// Both halves hold, and there is no third case. Two modules are identical —
/// `mvsg_cmc` (`cmc/mvsg_cmc_v4.0.0_official/vacode/mvsg_cmc_4.0.0.va`) at
/// 2,861,548 bytes and `bsimsoi`
/// (`cmc/BSIM_SOI_100.1.1_09152025/code/bsimsoi.va`) at 9,498,652 — and those
/// are precisely the two the CFG route refuses, both on the W-D
/// `Ddt`-under-condition class that [`crate::native::cfg_size_census`] reports as
/// `lowering=refused … canonical analog operator Ddt runs under a condition and
/// its operand cone contains BlockParameter`. The other forty-one all moved.
///
/// The images grew, which is the CFG route's known cost and not news:
/// `bsimcmg_va` 4,798,396 to 5,623,596, `asmhemt` 16,661,736 to 17,287,712 and
/// `hicumL2va` 939,040 to 1,266,812 reproduce
/// [`crate::native::cfg_size_census`]'s `image route=cfg` figures exactly, which
/// is the cross-check that these are the images that census measures. Two
/// readings are worth naming beyond it: `l_utsoi` grew 4.2x (2,926,408 to
/// 12,356,892, and its `_nqs` sibling likewise), the largest factor in the
/// corpus; and the three `hisimhv` variants — 65,119,620, 71,142,388 and
/// 71,138,396 bytes — are now **past**
/// [`crate::native::SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES`], which is 60
/// MiB. `native_shipped_models` asserts that budget and cannot see it, because
/// it fails earlier on `vbic13_4t` for an unrelated reason that reproduces on
/// the pre-flip revision.
///
/// Measured 2026-09-03 on the flip, peers idle, 744 s.
///
/// # Earlier history
///
/// `fe9a2072…` moved to `e00c6b88…` at W-F3a (`84ba2c2bb`), which ruled
/// `limexp`'s threshold once for the whole estate — exactly five modules
/// changed (`angelov`, `angelov_gan`, `hicumL0va`, `hicumL2va`, `vbic_4T_et_cf`,
/// the five that call `limexp`) and the other thirty-eight were digest
/// identical. `e00c6b88…` moved to `79e2c753…` at W-F8/W-F8b (`5a8e55c7b`,
/// `1ba20f27c`), the reaching-definition snapshot: an equation now reads the
/// definition that reaches it, not the slot's final value, so exactly eight
/// modules changed — the five the compile-time survey named (`ekv3_rf` `tmp`;
/// `asmhemt` `t0`/`t1`; `bsimsoi_va` `qgate`; the two `bsimsoi` variants,
/// `qgate` and `sqig`/`mig`/`sqid`) plus three whose only capture is an operator
/// operand the survey's first pass did not walk (`bsimbulk` `FNPowerAt1Hz`,
/// `hicumL2va` `flicker_Pwr`, `r3_cmc` `gc`) — and the other thirty-five were
/// digest identical (measured 2026-09-03 on `1ba20f27c`, peers idle, 893 s).
const SHIPPED_CENSUS_DIGEST: &str =
    "8c19a2f5c9d55b4ab125ff355a2029f4ab010e6963b1fbf5c8e853b7f6feab6b";

/// One shipped module's compiled machine-code digest.
struct ModelImageDigest {
    name: String,
    /// The source the module was declared in.
    ///
    /// Three shipped modules are named `hisimsoi_va`, two `l_utsoi` and two
    /// `bsimsoi`, each from a different source, so a re-baseline that reads the
    /// per-module lines below cannot key a module on its name alone.
    path: PathBuf,
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
            path: shipped.path.clone(),
            digest: blake3::hash(&normalized).to_hex().to_string(),
            bytes: image.len(),
            helper_calls,
            seconds: shipped.compile_seconds + census_seconds,
        });
    }
    eprintln!(
        "code-identity total_compile_seconds={total_compile_seconds:.1} total_census_seconds={total_census_seconds:.1}"
    );
    digests.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.path.cmp(&right.path))
    });
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
            "code-identity model={} path={} bytes={} helper_calls={} seconds={:.1} digest={}",
            entry.name,
            entry.path.display(),
            entry.bytes,
            entry.helper_calls,
            entry.seconds,
            entry.digest
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
