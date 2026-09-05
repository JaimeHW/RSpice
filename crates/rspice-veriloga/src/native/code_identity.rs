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
/// # The observable set leaving the CFG plan's roots
///
/// `86d6920e…` moved to `556e82e4…` at W-F14c, when the CFG plan's assignment
/// pass stopped being rooted on every externally observable variable. W-F14
/// had already taken the postfix entries' reads off those roots; the observable
/// set stayed because operating-point readback had nowhere else to come from,
/// and it kept almost the whole pass alive on almost every module. A readback
/// now compiles a pass of its own on demand
/// ([`crate::device::VerilogADevice::observe_variables`]), so what an evaluation
/// publishes is what the plan reads: the static conditions, the event-state
/// leaves, and `$bound_step`/`$discontinuity`, which the stepper reads back
/// between evaluations and no entry ever reads.
///
/// **Forty-one modules moved, two are byte identical, and not one image grew.**
/// The two are exactly the W-D refusals — `mvsg_cmc` (2,861,548) and the
/// `BSIM_SOI_100.1.1` `bsimsoi` (9,498,652) — which keep the postfix plan,
/// whose entries do read the variable array and whose roots therefore did not
/// change. There is no third case, and no module can be in the wrong one: a
/// module on the CFG route that did not move would be one whose whole pass was
/// already reachable from what its plan reads.
///
/// The corpus went from 354,171,040 bytes to **105,811,376, −70.12 per cent**.
/// The largest factors are the two large `bsimsoi` variants the route accepts —
/// the 4.6.1 `bsimsoi_va` −94.20 (14,476,364 to 839,964) and the
/// `BSIM-SOI_4.7.0` `bsimsoi` −92.44 (14,987,868 to 1,133,260) — and
/// `PSPNQS104VA` −80.98 (30,795,492 to 5,858,268). The smallest are `ekv_va`
/// −16.71, `asmesd_dio` −16.92 and `EPFL_HEMT_10a` −16.96, three small modules
/// with little procedural body to drop. The three `hisimhv` variants fall from 57 MiB to 12.3
/// (`hisimhv_n5_va` 59,846,716 to 12,888,476, −78.46 per cent), which is what
/// takes [`crate::native::SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES`] from 60
/// MiB to 16, and `l_utsoi` −32.22 (7,414,136 to 5,025,336) — the module whose
/// CFG plan was the estate's one size *and* time regression, and is now neither:
/// [`crate::native::cfg_cost_census`] reads it at 0.644 of its postfix plan
/// where it read 1.734 with the pass in place.
///
/// The time reading is the sharper half of this change and belongs here beside
/// the bytes, because the pass the images no longer hold is a pass the CFG
/// route was still *running* on every evaluation. On `hisimhv_n5_va` that is
/// 4,880,238 ns per evaluation before and 41,212 after.
///
/// Measured 2026-09-05, peers idle, 300 s; the before column is `86d6920e…`'s
/// own, from the run recorded in the section below.
///
/// # The block programs' back edges getting a counter
///
/// `001961cf…` moved to `86d6920e…` at W-F14b, when every back edge of every
/// block program started counting its trips against the hundred-thousand
/// iteration limit only an assignment-step loop was held to before.
/// The reading is the sharpest of the four here, because the change has an
/// exact predicate: a module's image moves **if and only if one of its block
/// programs has a natural loop**.
///
/// **Seventeen moved and twenty-six are byte identical, and the seventeen are
/// exactly the seventeen a plan-level count of `loop_ranges` names.** Not one
/// module without a loop moved a byte, and not one with a loop stayed put.
/// The corpus went from 354,110,368 bytes to **354,171,040, plus 0.017 per
/// cent**, and the three `hisimhv` variants stay under
/// [`crate::native::SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES`], which was
/// then 60 MiB, with `hisimhv_n5_va` at 57.07 MiB.
///
/// The per-module growth divides into two rates, and both are accounted for.
/// Fourteen modules pay only the guard, at **74 to 80 bytes per back edge**:
/// `ekv3_rf`, `hicumL2va` and `bsimbulk` +80 for one loop each; `bsimsoi_va`
/// and the 4.6.1 `bsimsoi` +144 for two; `hisimsotb_va` +1,600 for
/// twenty-one; the three `hisimsoi_va` variants +2,800/+2,816 for
/// thirty-seven; the three `hisimhv` variants +12,080 for a hundred and
/// fifty-nine; and the two `l_utsoi` +1,800/+1,784 for twenty-four. Twenty of
/// those bytes are the counter's load, increment, store and compare; the other
/// fifty-five are the call to `rspice_native_loop_limit_error` on the path that
/// never runs.
///
/// The three PSP variants pay more — +3,204, +3,236 and +3,864 for ten, ten and
/// twelve loops — and the excess is not the guard. Their loop-carrying entries
/// also call helpers, so they now keep the evaluation context in R12 and R13
/// rather than in the volatile registers it arrives in, and a memory operand
/// based on R12 needs a SIB byte while one based on R13 needs a displacement
/// byte even at zero. One byte per context or variable access, across entries of
/// nineteen thousand instructions, is the whole of the difference. `l_utsoi`'s
/// cone entries take the same rule and are inside the first rate because they
/// read the context far less often.
///
/// Measured 2026-09-05, peers idle, 805 s; the before column is this box's own,
/// from a run on the pre-change sources that reproduced `001961cf…` exactly in
/// 1,123 s. Both runs paid a cold front end — the census cache keys on the
/// test binary, so a source change invalidates it — which is 502 s of the 805.
///
/// # The assignment pass leaving the CFG plan
///
/// `8c19a2f5…` moved to `001961cf…` at W-F14, when the CFG plan's assignment
/// liveness stopped rooting on what the *postfix* plan's entries read. Under a
/// CFG plan those entries are prelude-slot loads that read no variable, so the
/// assignments they used to keep alive were a pass held open for programs the
/// image no longer holds. The roots became what this plan reads — the static
/// conditions the route does not replace, the event-state leaves the prelude
/// lowers to a `LoadVariable`, and the externally observable set, which stayed
/// until operating-point readback had somewhere else to come from. The section
/// above is where it got one.
///
/// The reading is the same shape as the two below and it comes out cleaner than
/// either: **forty-one modules moved, two are byte-identical, and not one image
/// grew**. The two identical ones are exactly the W-D refusals — `mvsg_cmc`
/// (2,861,548) and the `BSIM_SOI_100.1.1` `bsimsoi` (9,498,652) — which keep the
/// postfix plan and are therefore untouched by a change to the CFG plan's
/// liveness. There is no third case, and no module can be in the wrong one: a
/// module that took the CFG route and did *not* move would be one whose
/// assignment pass had nothing in it that only a postfix entry read.
///
/// The corpus went from 487,148,176 bytes to **354,110,368, −27.31 per cent**.
/// The largest factors are `asmhemt` −67.64 (17,290,656 to 5,595,628) and the
/// three `hisimsoi_va` variants −60.3 each; the smallest are `r2_cmc` −2.71 and
/// `r2_et_cmc` −3.14, two resistor models with almost no procedural body to
/// re-root. The three `hisimhv` variants are back **under**
/// [`crate::native::SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES`] on this change
/// alone — 59,834,636 / 59,830,484 / 59,329,776 against 62,914,560 — and
/// `hisimhv_n5_va` at 57.06 MiB is what now sets that budget. `l_utsoi` carries
/// a second change on top: its eleven deferred contribution-current cones are
/// called from the fused stamp kernel instead of inlined into it, which is 4.46
/// MB of its 40 per cent (12,362,140 to 7,412,336).
///
/// Both halves were measured on this box rather than carried over. The census
/// run on the pre-change sources reproduces `8c19a2f5…` exactly (634 s), which
/// is also what says the per-module before column below is this box's and not a
/// previous lane's.
///
/// Measured 2026-09-04, peers idle, 634 s.
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
/// The images grew, and this file used to call that "the CFG route's known cost
/// and not news". W-F13b attributed it and it was neither. On `hisimhv_n5_va`
/// 81.9 per cent of the flipped image was the *postfix* assignment pass, which
/// the CFG plan kept verbatim and ran before its own prelude on every
/// evaluation, because the assignment liveness still rooted on what the postfix
/// entries read — entries that, under a CFG plan, are prelude-slot loads that
/// read no variable at all. W-F14 re-rooted it, and eleven megabytes of that
/// module went with the roots. `l_utsoi`'s 4.2x was the other half and also not
/// intrinsic: eleven deferred contribution-current cones, held once as value
/// entries and a second time inlined into the fused stamp kernel, which now
/// calls them. The growth figures below are the flip's, kept because they are
/// what the digest before this one measured:
/// `bsimcmg_va` 4,798,396 to 5,623,596, `asmhemt` 16,661,736 to 17,287,712 and
/// `hicumL2va` 939,040 to 1,266,812 reproduce
/// [`crate::native::cfg_size_census`]'s `image route=cfg` figures exactly, which
/// is the cross-check that these are the images that census measures. Two
/// readings are worth naming beyond it: `l_utsoi` grew 4.2x (2,926,408 to
/// 12,356,892, and its `_nqs` sibling likewise), the largest factor in the
/// corpus; and the three `hisimhv` variants — 65,119,620, 71,142,388 and
/// 71,138,396 bytes — were then **past**
/// [`crate::native::SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES`], which stood
/// at 60 MiB. `native_shipped_models` asserts that budget and cannot see it, because
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
    "556e82e45b434464a31de8abf01cd71f295d800684e3364228458a5cee6dee65";

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
